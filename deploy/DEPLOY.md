# LongevityCommon — server deployment guide (Option C)

**Target:** Full umbrella deployment on `app.longevity.ge` server (alongside existing `fclc.longevity.ge`).
**Subdomains:** `app.longevity.ge`, `ze.longevity.ge`, `biosense.longevity.ge`.
**Status (2026-04-28):** Phase A artifacts ready; Phase B (DNS+secrets) pending; Phase C (deploy) pending Phase B.

---

## Pre-flight: server state

| Resource | Capacity | Note |
|----------|----------|------|
| Disk | 38 GB total, ~15 GB free (59% used) | Tight; clean Docker build cache after each build |
| Postgres | port 5432 already used by `mariadb`, `space-db`, `drjaba-monetaria_db` | New postgres instance for umbrella will run inside docker network only (NOT exposed to host) |
| Existing services on conflict ports | 4000 (space-app), 4001 (docker-app), 8080 (drjaba-app) | New compose maps to 4400-4700 range to avoid conflict |

## Port mapping (server-side)

| Service | Host port (loopback) | Container port |
|---------|---------------------:|---------------:|
| ze-web (Phoenix) | 4400 | 4000 |
| ze-backend (Rust) | 4401 | 4001 |
| biosense-web (Phoenix) | 4501 | 4100 |
| biosense-backend (Rust) | 4502 | 4101 |
| realtime (Phoenix Channels) | 4500 | 4500 |
| server (Rust axum) | 4600 | 8080 |
| web (nginx static PWA) | 4700 | 80 |
| postgres | (internal only) | 5432 |

External access: only via nginx vhosts on `*.longevity.ge` (HTTPS via Let's Encrypt).

---

## Phase B — User actions required (gating)

### B.1 — Add DNS A-records in Cloudflare

Add 3 A-records pointing to the same server IP as `fclc.longevity.ge`:
```
ze.longevity.ge → <server IP>
biosense.longevity.ge → <server IP>
app.longevity.ge → <server IP>
```

Server IP can be checked: `dig +short fclc.longevity.ge`.

After DNS propagation (typically 1-5 min via Cloudflare), verify:
```
dig +short ze.longevity.ge
dig +short biosense.longevity.ge
dig +short app.longevity.ge
```

### B.2 — Generate secrets and place in `.env`

On the server:
```bash
ssh server
cd ~/web/longevitycommon # after rsync from desktop, see Phase A.2
cp deploy/.env.template .env
chmod 600 .env

# Generate three Phoenix secret_key_base values:
docker run --rm hexpm/elixir:1.17.3-erlang-27.2-debian-bookworm-20241202-slim \
 sh -c 'mix local.hex --force && mix archive.install --force hex phx_new && mix phx.gen.secret' >> .env # repeat 3x

# Or, if you have local Elixir:
for i in 1 2 3; do mix phx.gen.secret; done

# Generate postgres password:
echo "POSTGRES_PASSWORD=$(openssl rand -hex 24)" >> .env
```

Edit `.env` to assign the 4 values to the right variable names per `deploy/.env.template`.

---

## Phase A — Done (committed)

- ✅ docker-compose-all.yml with port re-mapping (4400-4700 range)
- ✅ Dockerfiles for: Ze backend + ze-web; BioSense backend + biosense-web; server, realtime, web
- ✅ nginx vhost templates for 3 subdomains: ze.longevity.ge, biosense.longevity.ge, app.longevity.ge
- ✅ .env.template with all required secrets
- ✅ This DEPLOY.md

## Phase C — Deploy (after Phase B complete)

### C.1 — rsync repo to server

```bash
# From local desktop:
cd ~/Desktop
rsync -avz --delete \
 --exclude='_archive/' \
 --exclude='target/' \
 --exclude='_build/' \
 --exclude='deps/' \
 --exclude='node_modules/' \
 --exclude='priv/static/' \
 --exclude='.git/' \
 LongevityCommon/ server:/home/jaba/web/longevitycommon/
```

### C.2 — Build images on server

```bash
ssh server
cd ~/web/longevitycommon
docker compose -f deploy/docker-compose-all.yml build 2>&1 | tee /tmp/lcommon-build.log
```

Expected: 5-10 min for first build. Builds run sequentially (parallel limited by docker buildkit free RAM).

### C.3 — Bring up postgres + migrate

```bash
docker compose -f deploy/docker-compose-all.yml up -d postgres
docker compose -f deploy/docker-compose-all.yml run --rm server /usr/local/bin/longevitycommon-server migrate
# (server binary should support a `migrate` subcommand; if not, run sqlx-cli manually)
```

### C.4 — Bring up rest

```bash
docker compose -f deploy/docker-compose-all.yml up -d
docker compose -f deploy/docker-compose-all.yml ps
```

All services should be `Up (healthy)` within 30 sec.

### C.5 — Enable nginx vhosts

```bash
sudo cp deploy/nginx/ze.longevity.ge.conf /etc/nginx/sites-available/
sudo cp deploy/nginx/biosense.longevity.ge.conf /etc/nginx/sites-available/
sudo cp deploy/nginx/app.longevity.ge.conf /etc/nginx/sites-available/

sudo ln -s /etc/nginx/sites-available/ze.longevity.ge.conf /etc/nginx/sites-enabled/
sudo ln -s /etc/nginx/sites-available/biosense.longevity.ge.conf /etc/nginx/sites-enabled/
sudo ln -s /etc/nginx/sites-available/app.longevity.ge.conf /etc/nginx/sites-enabled/

sudo nginx -t && sudo systemctl reload nginx
```

### C.6 — Issue SSL certificates

```bash
sudo certbot --nginx -d ze.longevity.ge -d biosense.longevity.ge -d app.longevity.ge
```

### C.7 — Smoke tests

```bash
curl -fsS https://ze.longevity.ge/ | grep -i "ze theory"
curl -fsS https://biosense.longevity.ge/ | grep -i "biosense"
curl -fsS https://app.longevity.ge/api/health
curl -fsS https://app.longevity.ge/api/disclosures/v5_changes | jq '.longevitycommon_version'
```

### C.8 — Cloudflare cache purge (if any)

If responses look stale, purge in Cloudflare dashboard for the 3 subdomains.

---

## Phase D — Unified linking (after Phase C verified)

Add cross-project footer to each Phoenix:

- `Ze/ze-web/lib/ze_web_web/components/layouts/app.html.heex` — append footer with links
- `BioSense/biosense-web/lib/biosense_web_web/components/layouts/app.html.heex` — same
- `realtime/lib/longevitycommon_realtime_web/...` — same

Footer template (Phoenix HEEX):
```heex
<footer class="footer">
 <div>Part of the LongevityCommon ecosystem ·
 <a href="https://app.longevity.ge">Home</a> ·
 <a href="https://ze.longevity.ge">Ze Simulator</a> ·
 <a href="https://biosense.longevity.ge">BioSense</a> ·
 <a href="https://fclc.longevity.ge">FCLC</a> ·
 <a href="https://longevity.ge/rescience/">Annals of Rejuvenation Science</a> ·
 <a href="https://github.com/djabbat/LongevityCommon" rel="noopener">Source</a>
 </div>
</footer>
```

After footer additions, rebuild + redeploy the three webs (re-run C.2-C.4 for the modified containers).

---

## Rollback

```bash
ssh server
cd ~/web/longevitycommon
docker compose -f deploy/docker-compose-all.yml down
sudo rm /etc/nginx/sites-enabled/{ze,biosense,app}.longevity.ge.conf
sudo systemctl reload nginx
```

DNS A-records can stay; nginx vhosts gone = subdomains return DNS-only response.

---

## Maintenance commands

```bash
# Tail logs
docker compose -f deploy/docker-compose-all.yml logs -f --tail=100

# Restart single service
docker compose -f deploy/docker-compose-all.yml restart ze-web

# Rebuild after code change (incremental)
docker compose -f deploy/docker-compose-all.yml up -d --build <service>

# Database backup
docker compose -f deploy/docker-compose-all.yml exec postgres \
 pg_dump -U lcommon longevitycommon > /backup/lcommon-$(date +%F).sql
```
