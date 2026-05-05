#!/usr/bin/env bash
# AIM server deployment — runs on the remote server (jaba@server pattern).
#
# Used to deploy the public-facing AIM instance behind nginx, like Ze /
# BioSense / FCLC. No Docker. Native systemd. Per CLAUDE.md HARD CONSTRAINT.
#
# Expects to be run AS root on the server (or with sudo) after `git pull`
# under /home/jaba/web/aim/.
#
# Usage on the server:
#   sudo /home/jaba/web/aim/AIM/install/deploy-server.sh

set -euo pipefail

REPO_ROOT="${REPO_ROOT:-/home/jaba/web/aim}"
PREFIX="${PREFIX:-/opt/aim}"
SERVICE_USER="${SERVICE_USER:-jaba}"
NGINX_HOST="${NGINX_HOST:-aim.longevity.ge}"
PHX_PORT="${PHX_PORT:-4000}"

log() { printf '\033[1;36m[aim-deploy]\033[0m %s\n' "$*"; }

if [[ "$EUID" -ne 0 ]]; then
  log "re-exec with sudo"
  exec sudo -E "$0" "$@"
fi

[[ -d "$REPO_ROOT/AIM/rust-core" ]] || { echo "no checkout at $REPO_ROOT" >&2; exit 1; }

# ── build as service user (so target/ owned correctly) ──────────────────
log "building Rust + Phoenix as $SERVICE_USER"
sudo -u "$SERVICE_USER" bash -c "
  cd '$REPO_ROOT/AIM/rust-core' && cargo build --release --workspace &&
  cd '$REPO_ROOT/AIM/phoenix-umbrella' &&
    MIX_ENV=prod mix deps.get --only prod &&
    MIX_ENV=prod mix compile &&
    MIX_ENV=prod mix release --overwrite
"

# ── stage ────────────────────────────────────────────────────────────────
log "staging into $PREFIX"
mkdir -p "$PREFIX"/{bin,phoenix,etc,logs}
chown -R "$SERVICE_USER:$SERVICE_USER" "$PREFIX"

cp -f "$REPO_ROOT/AIM/rust-core/target/release/aim-llm" "$PREFIX/bin/" 2>/dev/null || true
PHX_REL="$(find "$REPO_ROOT/AIM/phoenix-umbrella" -maxdepth 6 -type d -name 'rel' | head -1)"
[[ -d "$PHX_REL" ]] && cp -r "$PHX_REL/." "$PREFIX/phoenix/"
chown -R "$SERVICE_USER:$SERVICE_USER" "$PREFIX"

# ── system-wide systemd units ────────────────────────────────────────────
log "writing /etc/systemd/system/aim-{orchestrator,phoenix}.service"

cat > /etc/systemd/system/aim-orchestrator.service <<EOF
[Unit]
Description=AIM Rust orchestrator
After=network-online.target
Wants=network-online.target

[Service]
Type=simple
User=$SERVICE_USER
Group=$SERVICE_USER
WorkingDirectory=$PREFIX
EnvironmentFile=-/home/$SERVICE_USER/.aim_env
ExecStart=$PREFIX/bin/aim-llm serve
Restart=on-failure
RestartSec=5
StandardOutput=append:$PREFIX/logs/orchestrator.log
StandardError=append:$PREFIX/logs/orchestrator.err.log
LimitNOFILE=65536

[Install]
WantedBy=multi-user.target
EOF

cat > /etc/systemd/system/aim-phoenix.service <<EOF
[Unit]
Description=AIM Phoenix LiveView
After=network-online.target aim-orchestrator.service
Wants=aim-orchestrator.service

[Service]
Type=simple
User=$SERVICE_USER
Group=$SERVICE_USER
WorkingDirectory=$PREFIX/phoenix
EnvironmentFile=-/home/$SERVICE_USER/.aim_env
Environment=MIX_ENV=prod
Environment=PHX_SERVER=true
Environment=PORT=$PHX_PORT
ExecStart=$PREFIX/phoenix/bin/aim_web start
Restart=on-failure
RestartSec=5
StandardOutput=append:$PREFIX/logs/phoenix.log
StandardError=append:$PREFIX/logs/phoenix.err.log
LimitNOFILE=65536

[Install]
WantedBy=multi-user.target
EOF

systemctl daemon-reload

# ── nginx vhost ──────────────────────────────────────────────────────────
NGINX_CONF="/etc/nginx/sites-available/$NGINX_HOST"
if [[ ! -f "$NGINX_CONF" ]]; then
  log "writing nginx vhost: $NGINX_CONF"
  cat > "$NGINX_CONF" <<EOF
server {
    listen 80;
    server_name $NGINX_HOST;

    # Let's Encrypt webroot
    location /.well-known/acme-challenge/ {
        root /var/www/certbot;
    }

    location / {
        proxy_pass http://127.0.0.1:$PHX_PORT;
        proxy_http_version 1.1;
        proxy_set_header Upgrade           \$http_upgrade;
        proxy_set_header Connection        "upgrade";
        proxy_set_header Host              \$host;
        proxy_set_header X-Real-IP         \$remote_addr;
        proxy_set_header X-Forwarded-For   \$proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto \$scheme;
        proxy_read_timeout 600s;
    }

    client_max_body_size 50M;
}
EOF
  ln -sf "$NGINX_CONF" "/etc/nginx/sites-enabled/$NGINX_HOST"
  nginx -t && systemctl reload nginx
fi

# ── start ────────────────────────────────────────────────────────────────
log "enabling + starting services"
systemctl enable --now aim-orchestrator aim-phoenix

log "done"
cat <<HINT

✅ AIM deployed
   prefix: $PREFIX
   user:   $SERVICE_USER
   url:    http://$NGINX_HOST/  (then certbot --nginx -d $NGINX_HOST)
   logs:   journalctl -u aim-orchestrator -u aim-phoenix -f

HINT
