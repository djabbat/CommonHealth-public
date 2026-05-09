# AUDIT PACKET — Claude_service

Path: `/home/oem/Desktop/Claude`  Date: 2026-05-08

## Size & file counts
```
8,7M	/home/oem/Desktop/Claude
```
**Extensions:** .md=85, .docx=47, .sh=11, .txt=4, .py=3, (noext)=2, .json=2, .ps1=2
## Tree (depth=2, max 200 entries)
```
.
./CONCEPTS.md
./SESSION_STATE.md
./PUBLICATIONS.md
./writing
./writing/Stambler_personal_short_2026-05-06.txt
./writing/ORCID_Biography_2026-05-06.txt
./writing/CV_Dr_Jaba_Tkemaladze.docx
./writing/Stambler_outreach_bilingual_2026-05-06.txt
./writing/NEEDTOWRITE.md
./writing/researchgate_longevity_horizon_request.md
./protocols
./protocols/MCOA_PROPAGATION_PLAN.md
./protocols/START.md
./protocols/PROMPT
./SYNC.md
./REVIEWERS.md
./workflows
./workflows/THURSDAY.md
./workflows/DEPLOY.md
./audits
./audits/AUDIT_Aqtivirebuli.md
./audits/AUDIT_HAP.docx
./audits/AUDIT_DrJaba.md
./audits/AUDIT_FCLC.md
./audits/pubmed_verify_2026-04-24.json
./audits/AUDIT_Iqalto.docx
./audits/AUDIT_Ze.md
./audits/AUDIT_FCLC.docx
./audits/AUDIT_Regenesis.md
./audits/AUDIT_Iqalto.md
./audits/AUDIT_kSystem.md
./audits/AUDIT_Ze.docx
./audits/AUDIT_AIM.docx
./audits/AUDIT_AIM.md
./audits/AUDIT_HAP.md
./audits/AUDIT_kSystem.docx
./audits/AUDIT_CommonHealth.docx
./audits/peer_reviews
./audits/overnight_2026-04-24
./audits/AUDIT_CDATA.docx
./audits/AUDIT_WLRAbastumani.md
./audits/AUDIT_Ontogenesis.docx
./audits/SUMMARY_PEER_REVIEW_2026-04-17.md
./audits/pubmed_verify_2026-04-24.md
./audits/AUDIT_BioSense.md
./audits/AUDIT_Ontogenesis.md
./audits/AUDIT_CommonHealth.md
./audits/AUDIT_SpellCheckerKa.md
./audits/literature
./audits/SUMMARY_PEER_REVIEW_2026-04-17.docx
./audits/AUDIT_BioSense.docx
./audits/AUDIT_SpellCheckerKa.docx
./audits/revisions
./audits/AUDIT_WLRAbastumani.docx
./audits/AUDIT_CDATA.md
./audits/AUDIT_DrJaba.docx
./audits/AUDIT_Aqtivirebuli.docx
./audits/AUDIT_Regenesis.docx
./Trips
./remote-laptop
./remote-laptop/setup-laptop-windows.ps1
./remote-laptop/connect-laptop.sh
./remote-laptop/setup-laptop-linux.sh
./remote-laptop/README.md
./remote-laptop/install-laptop-rustdesk.sh
./remote-laptop/laptop-secrets.txt
./remote-laptop/install-laptop-rustdesk.ps1
./TODO.md
./scripts
./scripts/pre_write_check.py
./scripts/build_web_deploys.sh
./scripts/md_to_docx.py
./scripts/weekly.sh
./scripts/article_digest.py
./scripts/push_archive.sh
./scripts/claude-session-end.sh
./scripts/claude-session-start.sh
./scripts/laptop-initial-setup.sh
./scripts/deploy.sh
./scripts/friday-reminder.sh
./data
./data/author_publications.json
./UPGRADE_TKEMALADZE.md
./CONTACTS.md
./WEB_TODO.md
./Archive
./Archive/PhD_root_admin_REMOVED_2026-05-05
./Archive/historical_2026-04
./Archive/PhD_bsu_REMOVED_2026-05-04
./Archive/PhD_tsu_REMOVED_2026-05-04
./Archive/PhD_program_folders_REMOVED_2026-05-05
./Archive/2026-04-24_Tbilisi_Lezhava_REMOVED_2026-05-04
./LAPTOP_SETUP.md
```
## Detected stack: **unknown**
## Core files

### `remote-laptop/README.md` (2257 chars)
```md
# Remote control desktop (jaba) ⇄ laptop

Wi-Fi: **immorta** · Subnet: **192.168.100.0/24**

## Endpoints

| host    | role           | RustDesk ID  | Pass      | SSH  | mDNS         |
|---------|----------------|--------------|-----------|------|--------------|
| jaba    | desktop/client | **219110943**| 3.14dar100STOP  | yes  | jaba.local   |
| laptop  | server         | *(TBD)*      | 3.14dar100STOP  | yes  | *(TBD).local |

## Что уже сделано

- ✅ desktop: установлены `remmina`, `tigervnc-viewer`, `freerdp2-x11`, `avahi-daemon`, `nmap`, `rustdesk` 1.3.6
- ✅ desktop: RustDesk service запущен, ID 219110943, permanent password `3.14dar100STOP`
- ⏳ laptop: ничего ещё не настроено (он офлайн / запущен другой сетевой контекст)

## Действия на laptop (один раз)

### Linux Ubuntu/Debian/Mint

Скопируй `install-laptop-rustdesk.sh` (USB, sshfs, scp с другой машины) и:
```bash
bash install-laptop-rustdesk.sh
```
Скрипт ставит RustDesk, прописывает permanent password `3.14dar100STOP`, выводит ID. Сообщи мне этот ID.

Дополнительно (для нативного SSH/RDP/VNC):
```bash
bash setup-laptop-linux.sh    # ставит openssh-server + xrdp + x11vnc + avahi
```

### Windows

В **админ-PowerShell**:
```powershell
Set-ExecutionPolicy -Scope Process Bypass -Force
.\install-laptop-rustdesk.ps1
```
Дополнительно для SSH/RDP: `setup-laptop-windows.ps1`.

## Подключение с desktop

```bash
# RustDesk (работает через NAT/firewall)
~/Desktop/Claude/remote-laptop/connect-laptop.sh <LAPTOP_ID>

# или SSH (когда настроен openssh-server на laptop)
ssh user@laptop.local

# или VNC (mirror живого экрана) через SSH-туннель
ssh -L 5900:localhost:5900 user@laptop.local
remmina -c vnc://localhost
```

После того как узнаем laptop-ID, сохранить его:
```bash
echo "<LAPTOP_ID>" > ~/Desktop/Claude/remote-laptop/laptop-id.txt
```
тогда `./connect-laptop.sh` без аргументов сразу подключается.

## Быстрая диагностика

```bash
nmcli dev wifi list | head           # Wi-Fi список
sudo nmap -sn 192.168.100.0/24       # кто живой в LAN
sudo nmap -p 22,3389,5900 <host>     # какие сервисы открыты
avahi-browse -art | grep _ssh        # mDNS-зарегистрированные ssh
systemctl status rustdesk            # сервис на этой машине
rustdesk --get-id                    # мой ID
```

```
### `TODO.md` (5071 chars)
```md
# TODO — service index of active projects

**Цель:** удовлетворить правило `feedback_projects_todo_tracking` ("All projects must be recorded in the service folder TODO; at every startup remind about them"). Это **индекс**, не оперативный список — детальные TODO живут в каждом проекте.

**Last updated:** 2026-05-07 (audit cycle).

---

## Топ-5 приоритетов (refresh каждую сессию через `feedback_daily_strategic_check`)

1. **GLA EIC Pathfinder** — outline + budget + 5 WPs (deadline 2026-10-28, 5 месяцев)
2. **PhD programme selection** — UNED preinscripción 2026-06-04, UJ Kraków 2026-06-15
3. **Books Ze Theory 4-step launch** — D2D upload 2026-05-31 → KDP 2026-06-15 → Stripe 2026-06-30 → Gumroad/Payhip 2026-07-15
4. **Annals first issue** + DOAJ submission Q1 2027 (требует policies, ещё editorial board members, 12-mo wait)
5. **JabaEkimi YPP application** — submit 2026-05-20

---

## Активные проекты (локально, `~/Desktop/`)

| Проект | Статус core | Главный TODO | Внутренний приоритет |
|---|---|---|---|
| **GLA** | 11/11 ✅ | `~/Desktop/GLA/TODO.md` (157+ items) | EIC + Annals + Board |
| **Iqalto** | 9/11 (нет README+THEORY) | `~/Desktop/Iqalto/TODO.md` | peer review applied; Aqtivirebuli papers commit ✅ |
| **Marketing** | 11/11 ✅ (восстановлено 2026-05-07) | `~/Desktop/Marketing/TODO.md` | Books launch + JabaEkimi YPP |
| **PhD** | core нейтральный ✅ (восстановлен 2026-05-07) | `~/Desktop/PhD/TODO.md` | multi-programme outreach |
| **Regenesis** | 10/11 (.docx удалены 2026-05-07) | `~/Desktop/Regenesis/TODO.md` | clinical material standalone .md (P5) |
| **ŠamnuAzuzi** | минимальный + STRATEGY/PRODUCTION_PLAN ✅ 2026-05-07 | `~/Desktop/ŠamnuAzuzi/TODO.md` (старый) | D1–D7 решения нужны от user |
| **Sulkalmakhi** | 13/11 ✅ | `~/Desktop/Sulkalmakhi/TODO.md` | Week 1–2 verify registration extract |
| **WLRAbastumani** | 10/11 (нет THEORY) + pricing aligned 2026-05-07 | `~/Desktop/WLRAbastumani/TODO.md` | ждёт инвестора |
| **LongevityCommon** | (umbrella, audit отдельно — другой терминал) | — | — |

## Подпроекты (subproject git rule — без своих repo, кроме E0)

| Подпроект | Parent | Особое |
|---|---|---|
| Aqtivirebuli | Iqalto | PNAS anchor встроен; papers committed 2026-05-07 |
| Pinekan | Regenesis | CLAUDE.md создан 2026-05-07 |
| JabaEkimi | Marketing | YouTube канал 2,270 subs, YPP-eligible |
| Books | Marketing | full 11-file core (subproject); Ze Theory v10 |
| E0 | PhD (submodule, отдельный repo `djabbat/E0`) | full 11-file core |
| English | PhD | English Mastery routine |

## Серверные проекты (`/home/jaba/web/` — `ssh server`)

См. `~/Desktop/Claude/WEB_TODO.md` (актуальный 2026-05-06).

| Сервис | Статус | Repo |
|---|---|---|
| longevity.ge OJS (Annals + Longevity Horizon) | live ✅ | `Longevity-OJS-private` |
| longevity.ge/ NGO landing | live ✅ | `ngo-landing-private` (added 2026-05-07) |
| drjaba.com (Phoenix) | live ✅ | `DrJaba` |
| books.drjaba.com | live ✅ | `books-landing-private` (added 2026-05-07) |
| drjaba-shared (layout assets) | live ✅ | `drjaba-shared-private` (added 2026-05-07) |
| ksystem.drjaba.com | live ✅ | `kSystem` |
| space.drjaba.com (Phoenix) | live ✅ | `Space` |
| spellcheckerka.drjaba.com | live ✅ | `SpellCheckerKa-private` |
| metabase.drjaba.com | 301 → drjaba.com (deferred) | — |
| **monetaria** | orphan на сервере (no vhost, no service) | `monetaria-private` (added 2026-05-07) |
| aim/biosense/cdata/fclc/hive/mcoa/ze.longevity.ge | LongevityCommon (out of scope) | `LongevityCommon-public` |

## Решения 2026-05-07 (по итогам аудита)

- **THEORY.md правило** (`feedback_project_core`) — ✅ обновлено: 10 MUST + 1 optional (THEORY.md → SHOULD WHEN APPLICABLE).
- **monetaria orphan** — ✅ удалена с сервера + GitHub. Backup `/home/jaba/backups/monetaria_deleted_2026-05-07/monetaria.tar.gz`.
- **Repo naming convention** (`feedback_git_naming`) — ✅ обновлено: bare name = private (default); `-public` суффикс ТОЛЬКО для публичных. Bare-name репозитории (PhD, Iqalto, Regenesis, WLRAbastumani, SamnuAzuzi, E0, DrJaba, FCLC, kSystem, Space, Claude) — НЕ переименовываем, они корректно приватны.
- **`app.longevity.ge` LongevityCommon umbrella deploy** — ✅ решено: переписать на native systemd (3 unit-а по образцу ze-web/biosense-web/fclc-web). Полный rewrite — в LongevityCommon-терминале, см. `WEB_TODO.md §3`.
- **ŠamnuAzuzi творческий процесс не завершён** — ✅ убран archive trigger 2026-11-07; проект остаётся открытым неопределённо долго; D1–D7 решения принимаются по готовности, не по дедлайну.

## Открытые системные вопросы

- Visibility audit GitHub: убедиться, что все bare-name репозитории действительно приватны на GitHub (не случайно public).
- LongevityCommon umbrella: native systemd rewrite (`app-realtime.service` + `app-social.service` + nginx static для PWA).

## Ссылки

- Подробный аудит: см. transcript сессии 2026-05-07 (главный аудит ecosystem)
- Web deployments: `WEB_TODO.md`
- Сессия state: `SESSION_STATE.md`
- Контакты: `CONTACTS.md`
- Reviewers (для подачи статей): `REVIEWERS.md`
- Personal academic upgrade KPIs: `UPGRADE_TKEMALADZE.md`

```
## Code volume
| ext | files | bytes |
|---|---|---|
| .py | 3 | 27738 |