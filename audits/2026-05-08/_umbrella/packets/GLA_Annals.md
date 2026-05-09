# AUDIT PACKET — GLA_Annals

Path: `/home/oem/Desktop/GLA/Annals`  Date: 2026-05-08

## Size & file counts
```
148K	/home/oem/Desktop/GLA/Annals
```
**Extensions:** .md=14, .jpg=2, .html=1, .sh=1
## Tree (depth=2, max 200 entries)
```
.
./policies
./README.md
./editorial
./editorial/03_email_to_liz_with_draft.md
./editorial/07_email_to_aubrey_with_draft.md
./editorial/08_aubrey_de_grey_static_page.html
./editorial/06_aubrey_de_grey_bio_draft_for_approval.md
./editorial/00_BOARD_MASTER.md
./editorial/photos
./editorial/02_liz_parrish_bio_draft_for_approval.md
./editorial/01_invitation_Klien_EN.md
./editorial/04_liz_parrish_INSTALLED_pre_approval.md
./editorial/05_outreach_wave2_editorial_board_2026-04-29.md
./ojs
./ojs/orcid_author_guide.md
./ojs/orcid_setup_BOTH_JOURNALS.md
./ojs/activate_orcid.sh
./issn
./issn/01_inquiry_to_NPLG_KA.md
./issn/02_activate_in_portal_KA.md
./CONCEPT.md
./issue_v1
```
## Detected stack: **unknown**
## Core files

### `README.md` (2174 chars)
```md
# GLA / JCAL — Journal of Centriolar Aging and Longevity

**Стартовано:** 2026-04-26
**Издатель:** Georgia Longevity Alliance (рег. №404506520)
**President:** Jaba Tkemaladze, MD
**Co-publisher / Editorial Board contact:** Lifeboat Foundation (Eric Klien, admin@lifeboat.com)

## Рабочее название журнала

**Journal of Centriolar Aging and Longevity (JCAL)** — рекомендация согласно `~/.claude/projects/-home-oem/memory/project_doaj_for_GLA.md`.

Альтернативы:
- Georgian Longevity Research (GLR)
- CDATA Journal of Cellular Aging

⚠️ **Финальное название утверждается до подачи на ISSN.**

## Scope

"Molecular and cellular mechanisms of centriolar damage in aging, centriologenesis, centrosome dysfunction, and therapeutic approaches to extend healthspan."

## Action plan (Tier 1 — apr-jul 2026)

| Шаг | Срок | Статус |
|---|---|---|
| 1. Выяснить процедуру ISSN у NPLG (Nino Simonishvili) | apr 2026 | 📨 драфт письма готов |
| 2. Финализировать имя журнала | apr 2026 | ⏳ ждёт user |
| 3. Подать заявку на ISSN | apr-may 2026 | — |
| 4. Editorial board invitations: Eric Klien + 2-3 грузинских биолога | apr-may 2026 | 📨 драфт Klien готов |
| 5. Установить OJS на VPS ($5-10/мес Hetzner/DigitalOcean) | may 2026 | — |
| 6. Шаблон сайта (логотип GLA, рубрикатор CDATA, политики) | may 2026 | — |
| 7. Сбор 5-7 статей для Volume 1 Issue 1 | may-jun 2026 | — |
| 8. Архивирование через PKP PN | jul 2026 | — |
| 9. Подача в DOAJ | oct-dec 2026 | — |

## Files in workspace

- `issn/` — ISSN application drafts and procedure
- `editorial/` — Editorial board correspondence (Klien + Georgian invitees)
- `ojs/` — OJS setup notes, VPS choice, configuration
- `policies/` — Author guidelines, peer-review policy, ethics, archiving
- `issue_v1/` — First issue articles, structure, metadata

## Contacts

- **NPLG ISSN office:** Nino Simonishvili `nsimonishvili@nplg.gov.ge` / `ninosim@hotmail.com` / +(995 32) 297 16 32 / 5 Gudiashvili St, Tbilisi
- **Lifeboat:** Eric Klien `admin@lifeboat.com`
- **DOAJ apply:** https://doaj.org/apply (free)
- **OJS / PKP:** https://pkp.sfu.ca/ojs/

## Memory references

- `project_doaj_for_GLA.md` — full strategy + Tier 1/2/3 action plan

```
### `CONCEPT.md` (4114 chars)
```md
# Annals of Rejuvenation Science — CONCEPT

**Canonical journal name (2026-04-26+):** Annals of Rejuvenation Science
**e-ISSN:** **3088-439X**
**URL:** <https://longevity.ge/rescience/>
**Platform:** OJS 3.5.0.3
**Crossref member:** 55174
**Publisher:** Georgia Longevity Alliance (Reg. №404506520)
**Editor-in-Chief:** Jaba Tkemaladze, MD
**Started:** 2026-04-26 (publication setup)
**Target:** DOAJ submission Q1 2027

## Identity history

Renames during setup phase (do NOT use historical variants):
- ❌ "Journal of Centriolar Aging and Longevity (JCAL)" — early working name
- ❌ "Georgian Longevity Research (GLR)" — alternative considered
- ❌ "CDATA Journal of Cellular Aging" — alternative considered
- ✅ **Annals of Rejuvenation Science** — final, ISSN-registered

## Scope

Molecular and cellular mechanisms of aging-related dysfunction, centriologenesis,
centrosome biology, regenerative medicine, healthspan-extending therapies. Strong
overlap with CDATA (Centriolar Damage And) theory umbrella + MCOA (Multi-Counter
Architecture of Organismal Aging).

## Twin-journal split (canonical strategy 2026-04-26)

GLA hosts **two journals**, separated by selectivity vs inclusivity:

| | Annals of Rejuvenation Science | Longevity Horizon |
|---|---|---|
| Selectivity | high (DOAJ-bound, peer review-strict) | broader, inclusive |
| Reject rate target | ≥40% | ≥10% |
| Use case | flagship CDATA / MCOA papers | early-career, niche, conference |

**Don't trim Longevity Horizon to be like Annals** — they exist in tension on purpose.

## Editorial Board status (2026-04-30)

Target: 3-5 international editors before DOAJ submission. **Currently 4/5 confirmed.**

| # | Name | Affiliation | Consented | Bio-approved |
|---|---|---|---|---|
| 1 | Eric Klien | Lifeboat Foundation | ✅ 2026-04-26 | ✅ |
| 2 | Liz Parrish | BioViva CEO | ✅ 2026-04-28 (provisional) | ⚠ pending |
| 3 | Alexey Moskalev | Petrovsky RRCS, Moscow | ✅ 2026-04-29 | bio sent |
| 4 | Aubrey de Grey | LEV Foundation | ✅ 2026-04-30 00:45 | bio not yet sent |
| 5 | TBD | TBD | — | — |

Need 1 more confirmed before Jan 2027 DOAJ submission for safety margin.

## Outreach Waves

- **Wave 1 (2026-04-28):** 17 author/editor invitations sent. Status check 2026-05-03 (today) via cloud routine `trig_01WSqXC8HXMP9qnJ83sq8x1Q`. Soft-pings sent to silent invitees by user 2026-05-03.
- **Wave 2 (2026-04-29):** additional editor invitations.

## DOAJ readiness — 2026-04-26 audit findings

3 critical blockers + 10 serious issues identified. **9/13 FIXED.** Prognosis 5% → 80% on DOAJ acceptance after step-fixes. Optimal submission window: **January 2027** (12 months past start meets DOAJ tenure rule).

## Sub-folder map

```
Annals/
├── README.md              ← entry (note: contained outdated "JCAL" name; superseded by this CONCEPT)
├── CONCEPT.md             ← this file (authoritative)
├── editorial/             ← editorial board management, peer review pool
├── issn/                  ← ISSN docs (3088-439X assigned)
├── issue_v1/              ← first issue WIP
├── ojs/                   ← Open Journal Systems install artifacts (deployed on longevity.ge server)
│                            ORCID auto-update wired 2026-04-28; 86 DOI redeposit done
└── policies/              ← editorial / ethics / authorship policies
```

## Cross-refs

- DOAJ audit detail: `~/.claude/projects/-home-oem/memory/project_annals_doaj_audit_2026-04-26.md`
- Wave 1 outreach status: `project_annals_outreach_wave1.md`
- Twin-journal strategy: `feedback_journal_split_strategy.md`
- ORCID auto-update routine: `project_orcid_ojs_setup.md` + `project_orcid_monitor_routine.md`

## Pre-DOAJ submission TODO (timeline)

| Action | Target |
|---|---|
| Confirm 5th international editor | by 2026-08 |
| Publish ≥4 articles to demonstrate active publishing | by 2026-12 |
| Bio approvals from Parrish, Moskalev, de Grey | by 2026-06 |
| Editorial board page polished + headshots | by 2026-09 |
| Author guidelines + ethics policy reviewed | by 2026-09 |
| **DOAJ submission window** | **Jan 2027** |
| Decision (DOAJ typically 3-6 months) | Apr-Jul 2027 |

```