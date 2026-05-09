# AUDIT PACKET — LC_deploy

Path: `/home/oem/Desktop/LongevityCommon/deploy`  Date: 2026-05-08

## Size & file counts
```
1,4M	/home/oem/Desktop/LongevityCommon/deploy
```
**Extensions:** .conf=15, .html=13, .service=8, .jpg=7, .md=5, .template=3, .yml=2, .js=2, .sh=2, .css=1, .png=1, .timer=1
## Tree (depth=2, max 200 entries)
```
.
./web-shared
./web-shared/eco-inject.js
./web-shared/mcoa-landing.html
./web-shared/DESIGN_CONCEPT.md
./web-shared/README.md
./web-shared/cdata-landing.html
./web-shared/longevity-root
./nginx
./nginx/ze.longevity.ge.conf
./nginx/biosense.longevity.ge.conf
./nginx/app.longevity.ge.conf
./docker-compose-all.yml
./server-state
./server-state/README.md
./server-state/web-content
./server-state/nginx-snippets
./server-state/nginx-vhosts
./server-state/systemd
./DEPLOY.md
./scripts
./scripts/deploy_all.sh
./scripts/deploy-app-native.sh
./systemd
./systemd/ze-web.service
./systemd/README.md
./systemd/longevitycommon-autopull.service
./systemd/app-social.env.template
./systemd/biosense-web.service
./systemd/longevitycommon-autopull.timer
./systemd/app-social.service
./systemd/fclc-web.service
./systemd/app-realtime.service
./systemd/app-realtime.env.template
./docker-compose-all.OLD-pre-v5.6.yml
```
## Detected stack: **unknown**
## Core files

### `web-shared/README.md` (1353 chars)
```md
# deploy/web-shared/ — cross-subdomain assets

Серверные ассеты, которые инжектятся через nginx `sub_filter` или
включаются темой OJS на ВСЕХ longevity.ge поддоменах.

## Файлы

- **`eco-inject.js`** — общий sticky header (LongevityCommon brand +
  навигация по экосистеме) + dark/light переключатель + dark theme
  CSS overrides для PKP/OJS, Tailwind, кастомных стилей.

  Грузится на каждом поддомене из `https://longevity.ge/eco-inject.js`.
  Cookie `lc_theme` на `.longevity.ge` синхронизирует тему между
  поддоменами.

  Тема, навигация и dark mode — единственный механизм; не дублировать
  в темах OJS / Phoenix / Vite.

## Deploy на server

Канонический путь на сервере: `/home/jaba/web/longevity/eco-inject.js`
(симлинки/копии в `/home/jaba/web/ngo/`, и т.д. для legacy путей).

```bash
scp deploy/web-shared/eco-inject.js \
    server:/home/jaba/web/longevity/eco-inject.js
ssh server "sudo cp /home/jaba/web/longevity/eco-inject.js \
            /home/jaba/web/ngo/eco-inject.js"
```

После деплоя — bump `?v=` query string в OJS теме / nginx snippet,
иначе CF и браузеры закэшируют старую версию.

## Bump cache version

После любого изменения JS:

```bash
ssh server 'cd /home/jaba/web/longevity && \
  grep -rl "eco-inject.js?v=" plugins/themes/ | \
  xargs sed -i "s/eco-inject.js?v=[0-9]*/eco-inject.js?v=$(date +%s | tail -c 4)/g"'
```

```
### `server-state/README.md` (3854 chars)
```md
# Server State Snapshot

Live infrastructure state on `server.longevity.ge` (server-2, Ubuntu 24.04, IP 77.42.67.59) as of 2026-05-05. Captured for reproducibility — when this server burns, recreate by:

1. Provisioning a fresh Ubuntu host
2. Cloning this monorepo
3. Running `AIM/install/deploy-server.sh` for AIM
4. Copying the configs in this folder into place (paths below)
5. Running `AIM/install/server-patches/add-eco-inject-to-all-vhosts.sh`
6. Running `AIM/install/server-patches/inline-hero-style.sh`

## Layout

```
nginx-snippets/             → /etc/nginx/snippets/
  eco-inject.conf           sub_filter that injects eco-inject.js into every vhost
  hive-button.conf          sub_filter for the floating Hive button (legacy)
  hero-blue.conf            sub_filter for forcing indigo hero on home

nginx-vhosts/               → /etc/nginx/sites-enabled/
  default.conf              longevity.ge root + /team/ /grants/ /research/ etc.
  aim.longevity.ge.conf     AIM Phoenix proxy :4040 + :443 self-signed origin
  ze.longevity.ge.conf      Ze Phoenix proxy
  biosense.longevity.ge.conf
  fclc.longevity.ge.conf
  hive.longevity.ge.conf    FastAPI proxy :8090
  mcoa.longevity.ge.conf    Static landing /var/www/mcoa-landing
  cdata.longevity.ge.conf   Static landing /var/www/cdata-landing
  app.longevity.ge.conf     Placeholder umbrella

systemd/                    → /etc/systemd/system/
  aim-orchestrator.service  Runs /opt/aim/bin/aim-llm serve
  aim-phoenix.service       Runs /opt/aim/phoenix/bin/aim_web start

web-content/
  ngo/                      → /home/jaba/web/ngo/
                            longevity.ge homepage + sub-pages (Team, Research,
                            Grants, Publications, Contact). Includes eco-inject.js
                            and inline-styled <section class="hero"> for indigo.
  mcoa-landing/             → /var/www/mcoa-landing/
  cdata-landing/            → /var/www/cdata-landing/
```

## What is NOT captured (intentionally)

- `~/.aim_env` on the server holds API keys + SECRET_KEY_BASE — secrets, never push.
- `/etc/letsencrypt/` Let's Encrypt certs — server-issued; recreate via `certbot --nginx`.
- `/etc/ssl/aim-origin/` self-signed Cloudflare-origin cert — regenerate via `openssl req -x509`.
- `/opt/aim/` built Rust binaries + Phoenix release — built at deploy time from source in this repo.
- Git checkouts under `/home/jaba/web/aim`, `/home/jaba/hive_queen`, OJS — separately versioned.
- `/home/jaba/web/longevity` — OJS install (Annals/Longevity Horizon) — separately versioned, secrets inside.
- Phoenix releases (`_build/prod/rel/`) — built from source.

## Reproducing the eco-inject pipeline

`deploy/web-shared/eco-inject.js` is the canonical version. To deploy:

1. Copy to `/home/jaba/web/ngo/eco-inject.js` on the server.
2. Bump `?v=NN` in `/etc/nginx/snippets/eco-inject.conf`.
3. Bump same version in any hardcoded references inside web-content/ngo/index.html, /var/www/longevitycommon-landing/index.html.
4. `sudo systemctl reload nginx`.

For Phoenix subdomains that load it (currently AIM): bump the `<script src="...?v=NN">` inside `apps/aim_web/lib/aim_web_web/components/layouts/root.html.heex`, then rebuild.

## Cloudflare DNS state (record for reference)

| Record | Type | Target | Proxy |
|---|---|---|---|
| `aim.longevity.ge` | A | `77.42.67.59` | Proxied 🟠 |
| `ze.longevity.ge` | A | `77.42.67.59` | Proxied |
| `biosense.longevity.ge` | A | `77.42.67.59` | Proxied |
| `fclc.longevity.ge` | A | `77.42.67.59` | Proxied |
| `mcoa.longevity.ge` | A | `77.42.67.59` | Proxied |
| `cdata.longevity.ge` | A | `77.42.67.59` | Proxied |
| `hive.longevity.ge` | A | `77.42.67.59` | Proxied |
| `longevity.ge` | A | `77.42.67.59` | Proxied |

SSL/TLS encryption mode: **Flexible** (CF→origin HTTP) or **Full** (CF→origin self-signed HTTPS). NOT Full Strict.

```
### `systemd/README.md` (2380 chars)
```md
# deploy/systemd/ — native Phoenix services

These systemd unit files run the Phoenix releases (ze-web, biosense-web,
fclc-web) **natively on the host**, not inside Docker containers. The
Docker pipeline (`deploy/docker-compose-all.yml`) is kept for build-time
artefact production but no longer for runtime.

## Why native

- One process tree under `systemctl`, no docker daemon dependency
- Direct `journalctl -u ze-web` log access
- Lower memory footprint (no per-service runtime overlay)
- nginx upstream is `127.0.0.1:<port>` either way; no observable
  difference for end users

## Layout on server

```
/opt/ze-web/         ← Phoenix release (extracted from Docker image once,
/opt/biosense-web/     then rebuilt natively via mix release after edits)
/opt/fclc-web/

/etc/systemd/system/ze-web.service
/etc/systemd/system/biosense-web.service
/etc/systemd/system/fclc-web.service
```

Each release ships its own ERTS (Erlang Runtime System) — no host-level
Erlang/Elixir is needed at runtime. Build environment (asdf-managed
Erlang OTP 27 + Elixir 1.17) lives in `~/.asdf/` for source rebuilds.

## Bootstrap (one-shot, already done 2026-05-04)

```bash
# 1. Extract release from existing Docker image (one-time, before
#    asdf-based native rebuild was ready)
docker create --name extract <image>
docker cp extract:/app /tmp/release
docker rm extract
sudo mv /tmp/release /opt/<service>
sudo chown -R jaba:jaba /opt/<service>
sudo mkdir -p /opt/<service>/tmp

# 2. Install systemd unit
sudo cp deploy/systemd/<service>.service /etc/systemd/system/
sudo systemctl daemon-reload

# 3. Stop the Docker container, start the native service
docker stop <container-name>
sudo systemctl enable --now <service>
```

## Rebuild after editing source

```bash
cd /home/jaba/web/longevitycommon/<Project>/<project-web>
. ~/.asdf/asdf.sh
MIX_ENV=prod mix deps.get --only prod
MIX_ENV=prod mix compile
MIX_ENV=prod mix release --overwrite
sudo systemctl stop <service>
sudo cp -r _build/prod/rel/<service>/* /opt/<service>/
sudo systemctl start <service>
```

## Ports (loopback only — public TLS via nginx + Cloudflare)

| Service       | Port | nginx upstream            |
|---------------|------|---------------------------|
| ze-web        | 4400 | ze.longevity.ge           |
| biosense-web  | 4501 | biosense.longevity.ge     |
| fclc-web      | 4003 | fclc.longevity.ge         |

```
## Code volume
| ext | files | bytes |
|---|---|---|
| .js | 2 | 152019 |