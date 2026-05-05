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
