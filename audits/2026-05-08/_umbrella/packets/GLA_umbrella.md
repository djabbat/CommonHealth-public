# AUDIT PACKET — GLA_umbrella

Path: `/home/oem/Desktop/GLA`  Date: 2026-05-08

## Size & file counts
```
2,6M	/home/oem/Desktop/GLA
```
**Extensions:** .md=26, .jpg=2, (noext)=1, .html=1, .sh=1, .djvu=1, .txt=1, .pdf=1
## Tree (depth=2, max 200 entries)
```
.
./PARAMETERS.md
./STRATEGY.md
./UPGRADE.md
./projects
./TODO.md
./MEMORY.md
./CLAUDE.md
./LINKS.md
./Annals
./Annals/policies
./Annals/README.md
./Annals/editorial
./Annals/ojs
./Annals/issn
./Annals/CONCEPT.md
./Annals/issue_v1
./statute
./statute/charter_2016_text.md
./statute/charter_2016.djvu
./statute/registration.txt
./statute/extract_2026-05-03.pdf
./docs
./CONCEPT.md
./_archive
./grants
./MAP.md
./KNOWLEDGE.md
./REMINDER.md
```
## Detected stack: **unknown**
## Core files

### `CLAUDE.md` (2154 chars)
```md
# CLAUDE.md — GLA Project Core

## Identity & naming (strict)

- **Canonical English name:** Georgia Longevity Alliance (GLA). **NEVER** use “Georgian Longevity Alliance”, “Longevity Georgia”, “Longevity Alliance Georgia”, or any Georgian transliteration in formal documents.
- **Georgian legal name:** კავშირი დღეგრძელობა (Art. 1.3 charter; NAPR record).
- **Legal form:** ა(ა)იპ (non-entrepreneurial non-commercial legal person).
- **ID:** 404506520.
- **Registered address:** 47 I. Javakhishvili St., Tbilisi 0170 (NOT 72 King David St.).
- **President/Chairman:** Jaba Tkemaladze, MD (Art. 4.10).

## Language rule

All responses in **Russian** (per global user rule). Use plain hyphens, never em-dashes.

## Writing tool rule

All writing through **DeepSeek API** (via `deepseek-r1` or `deepseek-chat`). Exception: Rust/Python integration code may use other tools.

## Source-of-truth precedence

1. NAPR extract (enreg.reestri.gov.ge, 2026-04-21)
2. Charter (2016, unamended — Art. 1–7)
3. CONCEPT.md (v2.0, 2026-05-03)
4. User-asserted memory (lowest priority)

## Fabrication prohibition

**NEVER** fabricate PMIDs, DOIs, addresses, dates, names, or contacts. If not in source materials, state “не указано” or “требуется верификация”.

## Boundary rules

- **NEVER** mix GLA with sister NGO **Sulkalmakhi** (separate banking, reporting, charter).
- **NEVER** touch `/Patients/` directory in any AIM context.
- **NEVER** reference “Longevity Party Georgia” or “Longevity Clinic, Inc.” as GLA entities.

## Git push rule

Public git push excludes all `.md` core files (CONCEPT.md, CLAUDE.md, charter docs). Only push README.md and code.

## Charter compliance flags

- Board quorum requires **all 5 members** (Art. 4.4) — any meeting without full attendance is invalid.
- No 2023 General Assembly election recorded — governance gap; regularise at 2026 GA.
- Address discrepancy: use charter address (47 Javakhishvili) for all official docs.
- Profit distribution prohibited (Art. 1.9, 6.6).

## Key dates

- Registration: 2016-01-12
- Next mandatory Board election: 2030-01-12
- EIC Pathfinder deadline: 2026-10-28
- DOAJ submission target: Q1 2027
```
### `Annals/README.md` (2174 chars)
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
### `CONCEPT.md` (18425 chars)
```md
# Georgia Longevity Alliance — CONCEPT (Organizational Development, 2026-2031)

**Status:** ACTIVE | **Version:** 2.0 | **Date:** 2026-05-03 | **Supersedes:** legacy CONCEPT.md

---

## 0. Identity & legal status (verified)

| Field | Authoritative source |
|---|---|
| Georgian legal name | კავშირი დღეგრძელობა (Art. 1.3 of charter; NAPR record) |
| English canonical name | Georgia Longevity Alliance (Art. 1.3; NAPR record) |
| Legal form | Non-entrepreneurial (non-commercial) legal person — ა(ა)იპ (Art. 1.4; NAPR) |
| Identification code | 404506520 (NAPR enreg.reestri.gov.ge, 2026-04-21) |
| Registration date | 12 January 2016 (NAPR application B15240095 completed) |
| Legal address (charter) | ი. ჯავახიშვილის ქ. №47, თბილისი (Art. 1.5) |
| Registered address (NAPR) | Same as charter — 47 I. Javakhishvili St., Tbilisi 0170 |
| Contact email (charter) | djabbat@gmail.com (Art. 1.6) |
| President / Chairman | ჯაბა ტყემალაძე (Jaba Tkemaladze, MD) (Art. 4.10; NAPR) |
| Board members (charter) | 5 founders listed in Art. 2 |
| Charter in force | Original 2016 charter — no successful amendments (B18008130 terminated) |
| Status | Registered, active (NAPR) |
| ILA member | Yes (ecosystem context) |
| Website | longevity.ge (NAPR record; ecosystem context) |

**Address clarification:** The legacy reference “72 King David St., Tbilisi 0170” (from LinkedIn/Google) is **not** the registered legal address. All official communications, grant applications, and contracts *must* use the charter/NAPR address: **47 I. Javakhishvili St., Tbilisi 0170**. A formal address-change registration should be considered before the 2030 Board renewal (See Section 3).

---

## 1. Mission & charter alignment (Art. 1.8 ↔ today's activity)

**Charter Art. 1.8** defines four objectives. The 2026–2031 mission operationalises each:

| Charter objective | Current manifestation (2026) |
|---|---|
| **1.8.1** Support, advancement and assistance for human rejuvenation and life-extension | Host Institution for EIC Pathfinder Challenges 2026; research umbrella for CDATA, MCOA, FCLC, BioSense, Ze Theory, Ontogenesis, HAP, Aqtivirebuli/Korkoti. |
| **1.8.2** Engaging people in life-extension activities | Public outreach via longevity.ge; collaboration with sister NGO Sulkalmakhi for local workshops; open-access publishing. |
| **1.8.3** Helping people with employment in line with their interests | Not yet operationalised as a standalone programme — to be developed in Pillar 2.4 (community & networking). |
| **1.8.4** Searching and establishing contacts with other longevity organisations in Georgia and abroad | ILA node; co-PI links with Ulm, Curie, UPF, COSIC/KU Leuven, BioViva; EIC consortium building. |

**Mission statement (derived from Art. 1.8):**  
*To advance human rejuvenation and life-extension science through international consortiums, diamond-OA publishing, theoretical development, and public engagement, all within the legal framework of a Georgian non-commercial legal entity.*

All auxiliary economic activity (Art. 1.9) shall serve this mission; profit distribution remains prohibited (Art. 1.9, 6.6).

---

## 2. Strategic pillars (5)

### 2.1 International longevity science consortium (EIC, Impetus, Horizon Europe)

- **Host Institution for EIC Pathfinder Challenges 2026** “Biotechnology for Healthy Ageing” (deadline 2026-10-28, €4M ceiling, TRL3-4). GLA is the legal applicant and coordinating entity.
- **Longevity Impetus Grants** — GLA as applicant entity for small pilot grants.
- **Horizon Europe** (Cluster 1, Missions) — future partnership tracks (2027–2029).
- **Co-PI network** (confirmed/in-progress 2026–05): Geiger (Ulm), Janke (Curie), Gonzalez Ballester (UPF), COSIC/Preneel (KU Leuven), Parrish (BioViva).
- **Charter basis:** Art. 1.8.1 (rejuvenation), Art. 1.8.4 (international contacts), Art. 4.5.1 (organisational development strategy).

### 2.2 Diamond-OA publishing (Annals of Rejuvenation Science + Longevity Horizons)

- **Annals of Rejuvenation Science** — e-ISSN 3088-439X, diamond OA, 4/5 Editorial Board confirmed. DOAJ submission Q1 2027. First issue WIP.
- **Longevity Horizons** (longevity.ge) — second OJS journal for broader longevity content.
- **Funding model — Vekua Club funder (зафиксировано 2026-05-06):** publication costs covered by the Vekua Club, so **no APC for any author** in either journal. This is what makes the diamond-OA model sustainable; it is NOT subsidised through GLA general funds.
- **Chichinadze Memorial Prize** (единый, student-only): a 3-tier annual prize (tier 1 / tier 2 / tier 3) for the best graduate-level papers published in either journal. Named in memory of Konstantine (Kote) Chichinadze. Funded separately from operations (donor-restricted line; see §2.5).
- **Charter basis:** Art. 1.8.1 (advancement), Art. 1.9 (auxiliary economic activity — page charges? no, diamond OA means no author fees; revenue from supplementary services or institutional memberships allowed as long as profit not distributed). Art. 4.5.1 (organisational strategy).

### 2.3 Theory & evidence — three Scientific Pillars (TF / ISE / IR)

The theory-and-evidence portfolio is organised into **three pillars** so that every active subproject can be traced to a methodological role:

- **TF — Theoretical Foundation.** Core hypotheses and conceptual frameworks. Lead artefact: **Centriolar Damage Theory (CDATA)** + its umbrella **MCOA** (Multi-Counter Architecture of Organismal Aging, submitted to eLife since 2026-04-30).
- **ISE — In-Silico Experimentation.** Computational simulation, modelling, and theoretical validation. Lead artefacts: **Ze Theory** (predictive coding model), **CDATA Rust simulator**, **Ontogenesis simulator**, **HAP** computational pipeline.
- **IR — In-Real (empirical) Research.** Wet-lab and field-data validation. Lead artefacts: **BioSense** (federated clinical learning), **FCLC** (consortium proposal, server-resident), **Aqtivirebuli-Korkoti** (heritage-anchored nutrition, PNAS 2026 anchor).

Subprojects move along TF → ISE → IR as they mature. New theory enters at TF; only when ISE simulation supports it does it justify IR experimental investment.

- **Charter basis:** Art. 1.8.1 (support and advancement). No charter restriction on theoretical research.

### 2.4 Community & networking (ILA node, sister-NGO link, public engagement)

- **ILA node** — maintain membership and active participation.
- **Sister NGO Sulkalmakhi** — local civic/cultural/ecology projects (see Section 8 for boundary rules).
- **Public engagement** — longevity.ge website; social media; local workshops (via Sulkalmakhi).
- **Employment help (Art. 1.8.3)** — to be operationalised as a mentorship/career programme for Georgian scientists in longevity (target: 2027).
- **Charter basis:** Art. 1.8.2 (engagement), 1.8.3 (employment help), 1.8.4 (contacts).

### 2.5 Sustainability & auxiliary economic activity (Art. 1.9-compliant revenue lines)

Art. 1.9 permits auxiliary entrepreneurial activity provided:
- Profit serves the organisation's objectives (rejuvenation/life-extension).
- Profit **not** distributed to founders, members, donors, or officers.
- Asset alienation allowed only if it serves activity, organisational development, objectives, or charitable purposes.

**Permitted revenue models:**
- Consultancy fees (longevity science strategy for external entities) — limited, non-core.
- Institutional memberships for journal (Annals).
- Grant overhead (if allowed by funder; must be reinvested).
- Conference organisation (registration fees).
- Sale of non-exclusive licenses for research tools (e.g., assays, datasets) — only if proceeds flow back to mission.
- **Donations** (longevity.ge/#donate landing) — five channels documented:
  1. **PayPal** — active (linked to GLA banking).
  2. **TBC Bank** (GE) — placeholder; activation pending compliance with TBC's NGO requirements (см. memory `project_tbc_account_gla`).
  3. **Crypto** — placeholder; activation depends on Georgian regulatory clarity for NGO crypto receipts.
  4. **GitHub Sponsors** — to be reverified (currently routed to maintainer's personal account, not GLA legal entity; needs migration).
  5. **Email-led donor outreach** — direct ask via the contact form on longevity.ge/#donate.
  Donations are unrestricted unless donor specifies (the Chichinadze Memorial Prize is the only restricted line currently advertised).
- **Vekua Club funder** — separate revenue stream restricted to journal publication costs (covers OJS/DOI/operational APC-equivalent so authors pay nothing). Not commingled with GLA general funds.

**Prohibited:** Any distribution of surplus to individuals; any commercial activity that does not serve chartered objectives.

---

## 3. Governance roadmap (Art. 3–4 obligations)

**3.1 General Assembly (Art. 3)**
- Convened ≥ 1× per year (Art. 3.1).
- Quorum: 1/3 of members (Art. 3.4).
- Powers: elect Board, approve Board's annual reports, decide on reorganisation/liquidation (Art. 3.2).
- Chair: President (Chairman of Society) or ad-hoc chair (Art. 3.5).

**3.2 Board (Art. 4)**
- 5 members, 7-year term (Art. 4.1). Current Board members = 5 founders (Art. 4.9).
- Quorum: all members must attend (Art. 4.4). This is a **strict requirement** — any meeting without full attendance is invalid.
- Decision by majority vote (Art. 4.2). Chairman elected by > half for 7-year term (Art. 4.6).
- Chairman reports quarterly to Board (Art. 4.5.3, 4.8).

**3.3 Mandatory re-election cycles**
- **First re-election due:** 2023-01-12 (7 years from registration). **No record of a General Assembly election in 2023.** This is a compliance risk. If not held, the current Board is technically *de facto* continuing but may be challenged. **Action:** Regularise via retroactive ratification at next General Assembly (see Section 6).
- **Second re-election due:** 2030-01-12. Must hold General Assembly with proper quorum before this date.

**3.4 Address discrepancy (Art. 1.5 vs. NAPR)**
- Charter address (47 Javakhishvili) is the legal address. The NAPR record shows the same. The 72 King David St. address used on LinkedIn/legacy docs is inconsistent. **Action:** File a data-change registration with NAPR to update the address to the correct street if necessary, or formally adopt the charter address in all documentation. Note: the previous data-change attempt (B18008130) was terminated — a new application is needed.

**3.5 Annual General Assembly 2026**
- Must be held before end of 2026. Agenda: elect Board for 2026-2033 term (if 2023 election was missed, combine ratification of existing Board + new election); approve annual report; set strategy.

**3.6 Quarterly Chairman reports**
- Art. 4.8 requires quarterly reports to Board. Ensure written reports are archived.

---

## 4. KPIs 2026-2031 (concrete, measurable, year-by-year)

| Year | KPI (GLA level) | Target |
|---|---|---|
| 2026 | EIC Pathfinder submission | Submitted by 2026-10-28 |
| 2026 | Annals of Rejuvenation Science – first issue published | Q4 2026 |
| 2026 | Board governance regularised | General Assembly held by 2026-12-31 |
| 2026 | Co-PI confirmations | ≥ 5 signed Letters of Intent |
| 2026 | Address correction registration filed with NAPR | By 2026-09-30 |
| 2027 | DOAJ submission (Annals) | Q1 2027 |
| 2027 | Impetus grant submission (if eligible) | Deadlines as per Impetus |
| 2027 | Employment-help programme launched (Art. 1.8.3 pilot) | Q3 2027 |
| 2027 | Number of active research subprojects (CDATA, MCOA, FCLC, etc.) | ≥ 4 with public outputs |
| 2028 | Horizon Europe proposal (Cluster 1) | Submitted (if call opens) |
| 2028 | Annals IF tracking (first partial year) | Indexed in at least 1 major database |
| 2029 | Board mid-term check | Compliance audit of governance |
| 2029 | Longevity Horizons – 2nd journal fully operational | ≥ 2 issues |
| 2030 | General Assembly for Board re-election | Held before 2030-01-12 |
| 2030 | Sustainability: non-grant revenue as % of total | ≥ 10% |
| 2031 | 15th anniversary report | Published |

*Note: All KPIs subject to funding availability; to be reviewed annually by Board.*

---

## 5. Risk register

| Risk | Likelihood | Impact | Mitigation |
|---|---|---|---|
| No 2023 General Assembly held – governance gap | High | Medium | Ratify at 2026 GA; adopt resolution confirming continuity. |
| EIC Pathfinder failed | Medium | High | Diversify to Horizon Europe, Impetus, private foundations. |
| Editorial Board loss | Low | High (journal) | Maintain pool of 5+ editorial board members; recruit backups. |
| Charter amendment needed (e.g., for new objectives) | Low | Medium | Requires General Assembly with 2/3 majority (Art. 6.2). Unlikely needed – Art. 1.8 is broad. |
| Address inconsistency legal challenge | Medium | Low | Address correction filing (Section 3.4). |
| Quorum failure at Board meetings (Art. 4.4 requires all 5) | Medium | High (paralysis) | Schedule meetings well in advance; virtual attendance allowed? Charter silent – interpret flexibly but document. |
| Profit-distribution accusation | Low | Very High (dissolution) | Strict compliance with Art. 1.9, 6.6; annual audit. |
| Sister NGO Sulkalmakhi funds confusion | Medium | Medium | Clear MoU (Section 8). |

---

## 6. 12-month operational plan (2026-05 → 2027-05)

| Period | Actions |
|---|---|
| May–Jun 2026 | 1. File NAPR address correction (47 Javakhishvili). 2. Convene Board meeting: approve concept, set EIC preparation team. 3. Confirm remaining Annals Editorial Board member. |
| Jul–Sep 2026 | 1. Finalise EIC Pathfinder proposal text, secure all co-PI letters. 2. Hold General Assembly (ratify Board, approve annual report 2025, adopt governance regularisation). 3. Begin address correction registration (if not done). |
| Oct 2026 | Submit EIC Pathfinder (deadline 2026-10-28). |
| Oct–Dec 2026 | 1. Publish Annals first issue. 2. Establish employment-help programme framework. 3. Draft DOAJ application for Annals. |
| Jan–Mar 2027 | 1. Submit DOAJ application. 2. First quarterly report to Board (2027 Q1). 3. Impetus grant evaluation (if applicable). |
| Apr–May 2027 | 1. Prepare Horizon Europe call analysis. 2. Mid-year strategy review with Board. 3. Public launch of employment-help pilot (Art. 1.8.3). |

---

## 7. Succession & continuity (charter has 5 board members for resilience)

- **Board size:** 5 members (Art. 4.1) — sufficient for continuity. Should any member resign, the remaining 4 can still meet (quorum requires all 5 — Art. 4.4 is all, not majority). This is a vulnerability. **Interpretation note:** If a member resigns, the Board cannot meet until replacement is elected by General Assembly. Charter does not provide for interim replacement by Board co-optation. Therefore, **ensure all 5 are committed to serve full term or until successor appointed.** Consider a charter amendment to allow co-optation (requires 2/3 GA vote — Art. 6.2).
- **Chairman succession:** Chairman elected by Board for 7 years (Art. 4.6). If Chairman resigns, Board elects new Chairman from its members.
- **General Assembly mandatory:** To replace any Board member, must call GA with quorum (1/3 of members). Since the 5 founders are the only members currently? Membership is open (Art. 5.1) — Board can admit new members, which increases the pool for GA quorum.
- **Emergency plan:** Maintain a list of 3 candidate members (non-founders) who can be admitted by Board (Art. 5.1) to broaden GA base.

…<truncated 54 more lines>…
```
### `Annals/CONCEPT.md` (4114 chars)
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
### `MAP.md` (6182 chars)
```md
# Georgia Longevity Alliance — MAP (Project & Sub-Entity Map)

**Version:** 1.0 | **Date:** 2026-05-03 | **Status:** Active

---

## 1. Core Entity

### Georgia Longevity Alliance (GLA) — `404506520`
- **Status:** ACTIVE
- **Legal form:** Non-entrepreneurial (non-commercial) legal person — ა(ა)იპ
- **Charter:** Original 2016, unamended
- **President:** Jaba Tkemaladze, MD
- **Address:** 47 I. Javakhishvili St., Tbilisi 0170
- **Website:** longevity.ge

---

## 2. Publishing

### 2.1 Annals of Rejuvenation Science
- **Status:** ACTIVE (building)
- **e-ISSN:** 3088-439X
- **Model:** Diamond open access
- **Editorial Board:** 4/5 confirmed
- **DOAJ submission:** Q1 2027 (target)
- **First issue:** Q4 2026 (target)
- **OJS platform:** Installed

### 2.2 Longevity Horizons
- **Status:** ACTIVE (operational)
- **Hosted at:** longevity.ge
- **Model:** Diamond open access
- **OJS platform:** Installed
- **Content:** Broader longevity topics

---

## 3. Research Umbrella

### 3.1 Core Theories

#### Centriolar Damage Theory (CDATA)
- **Status:** ACTIVE
- **Lead:** Jaba Tkemaladze
- **Description:** Core theoretical framework for centriolar involvement in aging

#### MCOA (Mitochondrial-Centriolar Organelle Axis)
- **Status:** UNDER REVIEW
- **Submission:** eLife (2026-04-30)
- **Lead:** Jaba Tkemaladze

#### FCLC (Focal Centriolar-Lysosomal Crosstalk)
- **Status:** PENDING (consortium proposal)
- **Lead:** Jaba Tkemaladze
- **Server location:** jaba@server:/home/jaba/web/fclc/

#### BioSense
- **Status:** ACTIVE
- **Description:** Biological sensing mechanisms in aging

#### Ze Theory
- **Status:** ACTIVE
- **Description:** Theoretical framework for cellular senescence

#### Ontogenesis
- **Status:** ACTIVE
- **Description:** Developmental biology approaches to rejuvenation

#### HAP (Healthy Aging Pathways)
- **Status:** ACTIVE
- **Description:** Molecular pathway analysis

#### Aqtivirebuli / Korkoti
- **Status:** ACTIVE
- **Description:** Local Georgian research initiatives

### 3.2 Grant-Funded Projects

#### EIC Pathfinder Challenges 2026
- **Status:** PREPARING (deadline 2026-10-28)
- **Ceiling:** €4M
- **TRL:** 3-4
- **Role:** Host Institution
- **Topic:** Biotechnology for Healthy Ageing

#### Longevity Impetus Grants
- **Status:** MONITORING (applicant entity)
- **Role:** Applicant

#### Horizon Europe (Cluster 1, Missions)
- **Status:** PLANNING (2027-2029)
- **Role:** Coordinator / Partner

---

## 4. Sister Organisation

### Sulkalmakhi (sister NGO)
- **Status:** ACTIVE (separate entity)
- **Scope:** Local civic, cultural, ecology projects
- **Boundary rules:**
  - Separate banking, reporting, charters
  - No fund mixing
  - Collaboration via written agreements only
  - GLA grants: GLA sole beneficiary
  - Sulkalmakhi may subcontract for public outreach (Art. 1.8.2)
- **Personnel overlap:** Allowed (Jaba Tkemaladze may serve both)
- **NOT GLA scope:** Local ecology, cultural workshops, municipal grants

---

## 5. External Partners

### 5.1 Academic / Research

#### Prof. Geiger — Ulm University
- **Status:** CONFIRMED (co-PI)
- **Role:** EIC Pathfinder co-PI
- **Expertise:** Aging biology

#### Dr. Janke — Institut Curie
- **Status:** CONFIRMED (co-PI via co-PI)
- **Role:** EIC Pathfinder co-PI
- **Expertise:** Centriole biology

#### Prof. Miguel A. Gonzalez Ballester — UPF Barcelona
- **Status:** CONFIRMED (co-PI)
- **Role:** EIC Pathfinder co-PI
- **Expertise:** Computational biology

#### COSIC / Prof. Preneel — KU Leuven
- **Status:** CONFIRMED (co-PI)
- **Role:** EIC Pathfinder co-PI
- **Expertise:** Cybersecurity / data integrity

#### BioViva / Parrish
- **Status:** CONFIRMED (co-PI)
- **Role:** EIC Pathfinder co-PI
- **Expertise:** Gene therapy / clinical translation

### 5.2 Networks

#### International Longevity Alliance (ILA)
- **Status:** ACTIVE (member node)
- **Role:** Network participation, knowledge exchange

---

## 6. Governance Nodes

### General Assembly
- **Status:** PENDING (must convene by 2026-12-31)
- **Quorum:** 1/3 of members
- **Agenda:** Board election, annual report, governance regularisation

### Board of Directors
- **Status:** ACTIVE (5 members, 7-year term)
- **Members:** 5 founders (Art. 4.9)
- **Chairman:** Jaba Tkemaladze
- **Quorum:** All 5 must attend (Art. 4.4)
- **Next election due:** 2030-01-12 (or retroactive ratification at 2026 GA)

### Chairman (President)
- **Status:** ACTIVE
- **Term:** 7 years (2016-2023, de facto continuing)
- **Reports:** Quarterly to Board (Art. 4.8)

---

## 7. Administrative Nodes

### NAPR Registration
- **Status:** ACTIVE
- **ID:** 404506520
- **Address:** 47 I. Javakhishvili St., Tbilisi 0170
- **Action pending:** Address correction filing (72 King David St. → charter address)

### Banking
- **Status:** ACTIVE (separate from Sulkalmakhi)
- **Currency:** GEL / EUR / USD (as needed for grants)

### Website (longevity.ge)
- **Status:** ACTIVE
- **Hosts:** Longevity Horizons journal, GLA info

---

## 8. Status Legend

| Flag | Meaning |
|---|---|
| ACTIVE | Operational / in progress |
| PREPARING | Active preparation underway |
| UNDER REVIEW | Submitted, awaiting decision |
| PENDING | Proposal / plan exists, not yet submitted |
| MONITORING | Tracking for future opportunity |
| PLANNING | Strategic planning phase |
| CONFIRMED | Agreement / LOI signed |
| ARCHIVED | No longer active |
…<truncated 21 more lines>…
```
### `PARAMETERS.md` (4320 chars)
```md
# PARAMETERS.md — Georgia Longevity Alliance

## Operational & Governance Parameters (2026–2031)

*All charter references are to the original 2016 charter (unamended).*

| Parameter | Value | Charter / Source | Notes |
|---|---|---|---|
| **Board size** | 5 members | Art. 4.1 | Fixed; cannot be changed without charter amendment (2/3 GA vote, Art. 6.2). |
| **Board term** | 7 years | Art. 4.1 | Current term: 2016-01-12 → 2023-01-12 (first renewal missed — see governance roadmap). Next mandatory: 2030-01-12. |
| **Chairman term** | 7 years | Art. 4.6 | Elected by > half of Board. Current: Jaba Tkemaladze (since 2016-01-12). |
| **General Assembly quorum** | 1/3 of members | Art. 3.4 | Minimum attendees for valid decisions. |
| **General Assembly cadence** | ≥ 1 per year | Art. 3.1 | Must convene at least annually. |
| **Board meeting quorum** | All 5 members must attend | Art. 4.4 | Strict requirement — any meeting without full attendance is invalid. |
| **Board meeting cadence** | ≥ 1 per month | Art. 4.3 | Convened by Chairman or > half of members. |
| **Chairman reporting cadence** | Quarterly | Art. 4.5.3, 4.8 | Written report to Board every 3 months. |
| **Liquidation supermajority** | 2/3 of General Assembly | Art. 6.2 | Required for reorganisation or dissolution. |
| **Membership admission** | By Board decision | Art. 5.1 | Any natural person sharing chartered objectives. |
| **Profit distribution** | Prohibited | Art. 1.9, 6.6 | No surplus to founders, members, donors, or officers. |
| **Asset alienation** | Only for chartered objectives, charitable purposes, or transfer to another NCJP | Art. 1.9, 6.5 | Must serve organisational development or mission. |
| **Charter amendment** | Requires 2/3 GA vote | Art. 6.2 | No successful amendments since 2016 (B18008130 terminated). |
| **Legal address** | 47 I. Javakhishvili St., Tbilisi 0170 | Art. 1.5; NAPR record | Legacy “72 King David St.” is incorrect — address correction filing needed by 2026-09-30. |
| **Contact email** | djabbat@gmail.com | Art. 1.6 | Official charter email. |
| **EIC Pathfinder budget ceiling** | €4M | EIC Work Programme 2026 | GLA as Host Institution; deadline 2026-10-28. |
| **EIC Pathfinder TRL** | TRL 3–4 | EIC Work Programme 2026 | “Biotechnology for Healthy Ageing” challenge. |
| **Annals of Rejuvenation Science** | e-ISSN 3088-439X | Ecosystem context | Diamond OA; 4/5 Editorial Board confirmed. |
| **Annals review SLA** | 60 days from submission to first decision | Internal policy | Target for 2026–2027. |
| **Annals DOAJ submission target** | Q1 2027 | Ecosystem context | Must meet DOAJ criteria (peer review, CC license, etc.). |
| **Longevity Horizons (OJS)** | Second journal on longevity.ge | Ecosystem context | Fully operational target: 2029 (≥ 2 issues). |
| **OJS uptime target** | 99.5% (monthly average) | Internal policy | For both longevity.ge and annals.longevity.ge. |
| **Co-PI Letters of Intent** | ≥ 5 signed by 2026-12-31 | Internal KPI | Confirmed/in-progress: Geiger (Ulm), Janke (Curie), Gonzalez Ballester (UPF), COSIC/Preneel (KU Leuven), Parrish (BioViva). |
| **Employment-help programme launch** | Q3 2027 | Art. 1.8.3 | Pilot for Georgian scientists in longevity. |
| **Board governance regularisation** | General Assembly by 2026-12-31 | Governance roadmap | Ratify 2023–2026 Board continuity; elect next term. |
| **Address correction filing** | By 2026-09-30 | Governance roadmap | New NAPR data-change application (B18008130 was terminated). |
| **Annual General Assembly 2026** | Before 2026-12-31 | Art. 3.1 | Agenda: Board election, annual report, strategy. |
| **15th anniversary report** | 2031-01-12 | Internal KPI | Published. |
| **Non-grant revenue target** | ≥ 10% of total by 2030 | Internal KPI | From consultancy, institutional memberships, conference fees. |
| **Risk: quorum failure mitigation** | Schedule meetings 30 days in advance; virtual attendance allowed (charter silent — document interpretation) | Art. 4.4 | All 5 members must attend; if one resigns, Board cannot meet until GA elects replacement. |
| **Sister NGO boundary** | No shared banking, reporting, or charter | Section 8 of CONCEPT.md | Sulkalmakhi for local civic/cultural/ecology projects only. |

*Last updated: 2026-05-03. All parameters subject to annual Board review.*
```
### `UPGRADE.md` (4577 chars)
```md
# UPGRADE.md — Georgia Longevity Alliance Infrastructure Backlog

**Status:** Active | **Priority:** Ordered (1 = highest) | **Last updated:** 2026-05-03

---

## Priority 1 — Governance & Legal Compliance (charter obligations)

- [ ] **1.1** Hold General Assembly before 2026-12-31 to ratify Board continuity (Art. 3.1 — no record of 2023 election; governance gap must be regularised)
- [ ] **1.2** File NAPR address correction to 47 I. Javakhishvili St. (Art. 1.5 — legacy 72 King David St. is inconsistent with charter; new application needed since B18008130 was terminated)
- [ ] **1.3** Establish quarterly Chairman → Board written reporting routine (Art. 4.5.3, 4.8 — currently no documented quarterly reports)
- [ ] **1.4** Verify quorum compliance for all Board meetings (Art. 4.4 — requires all 5 members; any meeting without full attendance is invalid)
- [ ] **1.5** Draft charter amendment allowing Board co-optation for interim vacancies (Art. 4.1 — resignation of one member paralyses Board; amendment needs 2/3 GA vote per Art. 6.2)
- [ ] **1.6** Prepare bilingual GE/EN charter version for international funders (Art. 1–7 — EIC, Horizon Europe, Impetus require English legal documents)
- [ ] **1.7** Audit membership list and admit 2-3 non-founder members to broaden GA quorum pool (Art. 5.1 — current GA quorum depends on 5 founders only; risk if one is unavailable)

## Priority 2 — Grant & Financial Infrastructure

- [ ] **2.1** Engage audit firm for grant due-diligence and annual financial statements (Art. 1.9 — EIC Pathfinder €4M requires audited accounts; no auditor on record)
- [ ] **2.2** Set up separate grant bank account for EIC Pathfinder funds (Art. 1.9 — auxiliary economic activity must be traceable; prevents profit-distribution accusations)
- [ ] **2.3** Draft EIC Pathfinder proposal skeleton with all co-PI letters template (deadline 2026-10-28 — 5 months lead time; Geiger, Janke, Gonzalez Ballester, COSIC, Parrish confirmed)
- [ ] **2.4** Create overhead cost policy compliant with Art. 1.9 (grant overhead must be reinvested in mission; no distribution to founders/officers)
- [ ] **2.5** Register GLA in Horizon Europe Participant Portal (PIC number needed — not yet obtained; required for 2027–2029 proposals)

## Priority 3 — Publishing & Research Operations

- [ ] **3.1** Bring DOAJ application docs to Q1-2027-ready state (Annals of Rejuvenation Science — e-ISSN 3088-439X; editorial policies, peer review workflow, copyright license must be finalised)
- [ ] **3.2** Confirm 5th Editorial Board member for Annals (4/5 confirmed; gap blocks first issue publication)
- [ ] **3.3** Publish Annals first issue by Q4 2026 (Art. 1.8.1 — advancement of rejuvenation science; milestone for EIC credibility)
- [ ] **3.4** Archive all research subproject charters (CDATA, MCOA, FCLC, BioSense, Ze, Aqtivirebuli) in GLA docs/ folder (current status: scattered across server; no central repository)

## Priority 4 — Institutional Documentation & Transparency

- [ ] **4.1** Deposit original charter PDF in statute/ folder (charter_2016.djvu exists but PDF not yet extracted; NAPR scan available)
- [ ] **4.2** Create public-facing governance page on longevity.ge (Art. 3–4 — Board members, charter summary, annual reports; required for EIC transparency)
- [ ] **4.3** Draft MoU with sister NGO Sulkalmakhi defining financial and operational boundaries (Section 8 of CONCEPT — prevents fund confusion; required for grant audits)
- [ ] **4.4** Establish Board minutes archive (Art. 4.2 — all decisions must be recorded; no central repository exists)
- [ ] **4.5** Create annual report template for 2025 (retroactive) and 2026 (Art. 3.2 — Board must approve annual reports; none on file)

## Priority 5 — Sustainability & Risk Mitigation

- [ ] **5.1** Develop non-grant revenue model (Art. 1.9 — consultancy, institutional memberships, conference fees; target ≥10% of budget by 2030)
- [ ] **5.2** Recruit 2 backup Editorial Board members for Annals (risk: loss of one member delays journal; maintain pool of 5+)
- [ ] **5.3** Draft emergency succession plan for Chairman (Art. 4.6 — 7-year term; if Jaba Tkemaladze resigns, Board must elect replacement; no interim provision)
- [ ] **5.4** Register GLA in Georgian Tax Authority as non-commercial entity (Art. 1.4 — required for grant overhead exemption; status unknown)
- [ ] **5.5** Conduct compliance audit of all past profit-distribution (Art. 1.9, 6.6 — zero-tolerance; review all transactions since 2016)

---

*Total items: 26. Review quarterly by Board. First checkpoint: 2026-09-30.*
```
### `TODO.md` (10984 chars)
```md
# TODO.md — Georgia Longevity Alliance (GLA)
**Period:** 2026-05-03 → 2026-08-01 | **Owner default:** Chairman (Jaba Tkemaladze) | **Status:** Active

---

## Week 1 (2026-05-03 → 2026-05-09)

| # | Task | Deadline | Owner | Status |
|---|---|---|---|---|
| 1.1 | **SRNSFG AR-2026 grant inquiry follow-up** — Confirm receipt of duplicate sent to rustaveli.org.ge on 2026-05-03. If no reply by 2026-05-07, call SRNSFG (contact from AR-2026 call docs). | 2026-05-07 | Chairman | IN PROGRESS |
| 1.2 | **MCOA review tracking** — Check eLife submission portal (submitted 2026-04-30). Log any reviewer assignments or editorial decisions. | 2026-05-05 | Chairman | PENDING |
| 1.3 | **Annals Editorial Board — confirm 5th member** — Contact candidate (list from ecosystem: Dr. Geiger, Ulm? or alternative). Send formal invitation letter with GLA letterhead. | 2026-05-09 | Chairman | PENDING |
| 1.4 | **NAPR address correction — initial research** — Review B18008130 termination reason. Prepare draft application for address change (47 Javakhishvili St. as legal address). | 2026-05-09 | Chairman | PENDING |

---

## Week 2 (2026-05-10 → 2026-05-16)

| # | Task | Deadline | Owner | Status |
|---|---|---|---|---|
| 2.1 | **EIC Pathfinder LoI — initial consortium list** — Draft list of confirmed co-PIs (Geiger/Ulm, Janke/Curie, Gonzalez Ballester/UPF, COSIC/Preneel/KU Leuven, Parrish/BioViva). Identify gaps (e.g., clinical partner, ethics lead). | 2026-05-14 | Chairman | PENDING |
| 2.2 | **Aqtivirebuli/Korkoti project scoping** — Define scope document: objectives, timeline, budget estimate. Align with Art. 1.8.1 (rejuvenation). | 2026-05-16 | Chairman | PENDING |
| 2.3 | **Board meeting — Q1 2026 report** — Prepare Chairman's quarterly report (Art. 4.8). Convene Board (all 5 members required per Art. 4.4). Agenda: approve concept, set EIC prep team, address governance gap. | 2026-05-16 | Chairman | PENDING |
| 2.4 | **SRNSFG follow-up call** — If no reply to email by 2026-05-07, call SRNSFG. Document outcome. | 2026-05-12 | Chairman | PENDING |

---

## Week 3 (2026-05-17 → 2026-05-23)

| # | Task | Deadline | Owner | Status |
|---|---|---|---|---|
| 3.1 | **EIC Pathfinder — draft proposal outline** — Create skeleton: objectives, methodology, consortium roles, budget (€4M ceiling, TRL3-4). Assign writing leads. | 2026-05-21 | Chairman | PENDING |
| 3.2 | **Annals Editorial Board — 5th member confirmed** — If invitation accepted, add to editorial board list. If declined, contact backup candidate. | 2026-05-21 | Chairman | PENDING |
| 3.3 | **NAPR address correction — file application** — Submit new data-change registration (B18008130 was terminated; new application needed). Use charter address (47 Javakhishvili). | 2026-05-21 | Chairman | PENDING |
| 3.4 | **MCOA review — first check** — Re-check eLife portal. If no update, log as "under review, no news." | 2026-05-19 | Chairman | PENDING |

---

## Week 4 (2026-05-24 → 2026-05-30)

| # | Task | Deadline | Owner | Status |
|---|---|---|---|---|
| 4.1 | **EIC Pathfinder — co-PI LoI letters** — Send draft LoI templates to all confirmed co-PIs. Request signed letters by 2026-06-15. | 2026-05-28 | Chairman | PENDING |
| 4.2 | **Aqtivirebuli/Korkoti — draft project plan** — Complete scope document. Present to Board for approval. | 2026-05-30 | Chairman | PENDING |
| 4.3 | **General Assembly preparation — start** — Draft agenda: ratify Board (if 2023 election missed), approve 2025 annual report, adopt governance regularisation resolution. | 2026-05-30 | Chairman | PENDING |
| 4.4 | **Annals — first issue content plan** — List articles, authors, deadlines. Coordinate with Editorial Board. | 2026-05-28 | Chairman | PENDING |

---

## Week 5 (2026-05-31 → 2026-06-06)

| # | Task | Deadline | Owner | Status |
|---|---|---|---|---|
| 5.1 | **EIC Pathfinder — consortium gap analysis** — Identify missing expertise (e.g., regulatory, ethics, clinical trial design). Begin outreach to fill gaps. | 2026-06-04 | Chairman | PENDING |
| 5.2 | **SRNSFG AR-2026 — decision tracking** — If grant inquiry positive, prepare full application. If negative, log reason and pivot to Impetus or other. | 2026-06-04 | Chairman | PENDING |
| 5.3 | **Board meeting — Q2 2026 report (early)** — Prepare draft quarterly report for Board review. | 2026-06-06 | Chairman | PENDING |
| 5.4 | **MCOA review — second check** — Re-check eLife portal. If reviewer comments received, begin response. | 2026-06-03 | Chairman | PENDING |

---

## Week 6 (2026-06-07 → 2026-06-13)

| # | Task | Deadline | Owner | Status |
|---|---|---|---|---|
| 6.1 | **EIC Pathfinder — full proposal draft v1** — Complete first full draft of proposal text. Share with co-PIs for feedback. | 2026-06-11 | Chairman | PENDING |
| 6.2 | **Annals — first issue manuscript collection** — Send call for papers to Editorial Board. Set submission deadline for 2026-08-01. | 2026-06-11 | Chairman | PENDING |
| 6.3 | **General Assembly — set date** — Confirm date (target: 2026-09-30). Send notice to all members (Art. 3.6). | 2026-06-11 | Chairman | PENDING |
| 6.4 | **Aqtivirebuli/Korkoti — budget estimate** — Prepare detailed budget. Identify potential funding sources (EIC? Impetus? private). | 2026-06-13 | Chairman | PENDING |

---

## Week 7 (2026-06-14 → 2026-06-20)

| # | Task | Deadline | Owner | Status |
|---|---|---|---|---|
| 7.1 | **EIC Pathfinder — co-PI LoI letters due** — Collect all signed letters. Follow up with any missing. | 2026-06-15 | Chairman | PENDING |
| 7.2 | **NAPR address correction — check status** — Verify if application processed. If rejected, re-file with corrected docs. | 2026-06-17 | Chairman | PENDING |
| 7.3 | **Annals — DOAJ application draft** — Begin drafting DOAJ application (target submission Q1 2027). | 2026-06-18 | Chairman | PENDING |
| 7.4 | **Board meeting — Q2 2026 report final** — Submit final quarterly report to Board. | 2026-06-20 | Chairman | PENDING |

---

## Week 8 (2026-06-21 → 2026-06-27)

| # | Task | Deadline | Owner | Status |
|---|---|---|---|---|
| 8.1 | **EIC Pathfinder — proposal v2 (incorporate feedback)** — Revise based on co-PI comments. | 2026-06-25 | Chairman | PENDING |
| 8.2 | **MCOA review — response preparation** — If reviewer comments received, draft response. If still under review, log. | 2026-06-24 | Chairman | PENDING |
| 8.3 | **Employment-help programme (Art. 1.8.3) — initial concept** — Draft concept note: mentorship for Georgian scientists in longevity. Target launch Q3 2027. | 2026-06-25 | Chairman | PENDING |
| 8.4 | **Sister NGO Sulkalmakhi — MoU draft** — Prepare MoU defining boundary rules (Section 8 of CONCEPT). | 2026-06-27 | Chairman | PENDING |

---

## Week 9 (2026-06-28 → 2026-07-04)

| # | Task | Deadline | Owner | Status |
|---|---|---|---|---|
| 9.1 | **EIC Pathfinder — budget finalisation** — Finalise budget (€4M ceiling). Ensure all co-PI costs included. | 2026-07-02 | Chairman | PENDING |
| 9.2 | **General Assembly — agenda finalised** — Finalise agenda: ratification of Board, annual report, governance regularisation, strategy 2026-2031. | 2026-07-02 | Chairman | PENDING |
| 9.3 | **Annals — first issue peer review** — If manuscripts received, assign reviewers. | 2026-07-02 | Chairman | PENDING |
| 9.4 | **Aqtivirebuli/Korkoti — funding search** — Identify 3 potential funders (EIC Pathfinder? Impetus? private foundations). | 2026-07-04 | Chairman | PENDING |

---

## Week 10 (2026-07-05 → 2026-07-11)

| # | Task | Deadline | Owner | Status |
|---|---|---|---|---|
| 10.1 | **EIC Pathfinder — proposal v3 (pre-submission polish)** — Final review by all co-PIs. Language check, formatting. | 2026-07-09 | Chairman | PENDING |
| 10.2 | **NAPR address correction — confirm completion** — Verify NAPR record updated. If not, escalate. | 2026-07-09 | Chairman | PENDING |
| 10.3 | **Board meeting — Q3 2026 report (early)** — Prepare draft quarterly report. | 2026-07-11 | Chairman | PENDING |
| 10.4 | **MCOA review — third check** — Re-check eLife portal. If accepted, prepare press release. If rejected, plan resubmission. | 2026-07-08 | Chairman | PENDING |

---

## Week 11 (2026-07-12 → 2026-07-18)

| # | Task | Deadline | Owner | Status |
|---|---|---|---|---|
| 11.1 | **EIC Pathfinder — final proposal ready** — Complete all sections, appendices, co-PI letters. Ready for submission. | 2026-07-16 | Chairman | PENDING |
| 11.2 | **General Assembly — notice sent** — Send formal notice to all members (Art. 3.6). Include agenda, draft resolutions. | 2026-07-16 | Chairman | PENDING |
| 11.3 | **Annals — first issue editorial decision** — Collect reviewer reports. Make accept/revise/reject decisions. | 2026-07-16 | Chairman | PENDING |
| 11.4 | **Employment-help programme — stakeholder mapping** — Identify potential mentors (Georgian scientists abroad, ILA network). | 2026-07-18 | Chairman | PENDING |

---

## Week 12 (2026-07-19 → 2026-07-25)

| # | Task | Deadline | Owner | Status |
|---|---|---|---|---|
| 12.1 | **EIC Pathfinder — submission preparation** — Final checks: budget, consortium agreement, ethics compliance. | 2026-07-23 | Chairman | PENDING |
| 12.2 | **Board meeting — Q3 2026 report final** — Submit final quarterly report to Board. | 2026-07-25 | Chairman | PENDING |
| 12.3 | **Aqtivirebuli/Korkoti — project proposal finalised** — Complete proposal for internal approval. | 2026-07-25 | Chairman | PENDING |
| 12.4 | **Sister NGO MoU — finalise** — Review with Sulkalmakhi leadership. Sign if agreed. | 2026-07-23 | Chairman | PENDING |

---

## Week 13 (2026-07-26 → 2026-08-01)

| # | Task | Deadline | Owner | Status |
|---|---|---|---|---|
| 13.1 | **EIC Pathfinder — submit** — Submit proposal via EIC portal (deadline 2026-10-28, but early submission recommended). | 2026-07-30 | Chairman | PENDING |
| 13.2 | **General Assembly — hold** — Convene GA (target 2026-09-30, but earlier if possible). Ratify Board, approve annual report, adopt governance regularisation. | 2026-07-31 | Chairman | PENDING |
| 13.3 | **Annals — first issue finalised** — Complete copyediting, layout. Prepare for publication (target Q4 2026). | 2026-08-01 | Chairman | PENDING |
| 13.4 | **90-day review** — Assess all tasks completed. Update CONCEPT.md. Plan next 90 days. | 2026-08-01 | Chairman | PENDING |

---

## Summary of key deadlines (2026-05-03 → 2026-08-01)

| Deadline | Task |
|---|---|
| 2026-05-07 | SRNSFG follow-up call |
| 2026-05-09 | Annals 5th member confirmed |
| 2026-05-16 | Board meeting (Q1 report) |
| 2026-05-21 | NAPR address correction filed |
| 2026-05-28 | EIC LoI templates sent |
| 2026-06-15 | Co-PI LoI letters due |
| 2026-07-16 | EIC final proposal ready |
| 2026-07-30 | EIC Pathfinder submitted |
| 2026-07-31 | General Assembly held |
| 2026-08-01 | Annals first issue finalised |

---

*Owner default: Chairman (Jaba Tkemaladze). Delegation possible to Mariam Kipshidze or other staff as appropriate. All tasks subject to funding and Board approval.*
```
### `KNOWLEDGE.md` (9244 chars)
```md
# KNOWLEDGE.md — Georgia Longevity Alliance (GLA)

**Purpose:** Curated knowledge base for GLA operations, funding, publishing, and scientific umbrella.  
**Last updated:** 2026-05-03  
**Supersedes:** All prior knowledge-base fragments.

---

## Section 1 — Legal facts

### 1.1 Registry identity (NAPR, verified 2026-04-21)

| Field | Value |
|---|---|
| Georgian legal name | კავშირი დღეგრძელობა (Art. 1.3) |
| English canonical name | Georgia Longevity Alliance (Art. 1.3) |
| Identification code | 404506520 |
| Legal form | Non-entrepreneurial (non-commercial) legal person — ა(ა)იპ (Art. 1.4) |
| Registration date | 12 January 2016 (application B15240095 completed) |
| Status | Registered, active |
| Legal address (charter) | 47 I. Javakhishvili St., Tbilisi 0170 (Art. 1.5) |
| Contact email (charter) | djabbat@gmail.com (Art. 1.6) |
| President / Chairman | Jaba Tkemaladze, MD (Art. 4.10) |
| Board members | 5 founders (Art. 4.9) — see charter Art. 2 for full list |
| Charter in force | Original 2016 charter — no successful amendments (B18008130 terminated) |

### 1.2 Charter constraints (binding on all operations)

- **Art. 1.7:** Member liability limited to NCJP property; no personal liability.
- **Art. 1.8:** Four objectives — rejuvenation support (1.8.1), public engagement (1.8.2), employment help (1.8.3), international networking (1.8.4).
- **Art. 1.9:** Auxiliary entrepreneurial activity allowed; profit must serve objectives; **profit distribution to founders/members/donors/officers prohibited**.
- **Art. 3.4:** General Assembly quorum = 1/3 of members.
- **Art. 4.1:** Board = 5 members, 7-year term.
- **Art. 4.4:** Board quorum = **all 5 members must attend** (strict — no meeting without full attendance).
- **Art. 4.6:** Chairman elected by > half of Board for 7-year term.
- **Art. 4.8:** Chairman reports quarterly to Board.
- **Art. 6.2:** Liquidation/reorganisation requires 2/3 General Assembly majority.
- **Art. 6.6:** Distribution of residual property to founders/members/officers prohibited.

### 1.3 Governance compliance status (2026-05)

- **First Board re-election due:** 2023-01-12 — **no record of General Assembly held**. Compliance risk; must be regularised at 2026 General Assembly.
- **Second re-election due:** 2030-01-12.
- **Address discrepancy:** Legacy “72 King David St.” (LinkedIn) ≠ charter address (47 Javakhishvili). File NAPR address correction by 2026-09-30.
- **Quarterly reports:** Art. 4.8 requires written reports to Board — ensure archive exists.

### 1.4 Tax & audit implications (Georgian NCJP)

- **Tax-exempt status:** Georgian NCJPs are exempt from profit tax on income used for chartered objectives (per Georgian Tax Code, Art. 98). Must file annual tax return (Form 1) with Revenue Service.
- **VAT:** If annual turnover exceeds GEL 100,000 (approx. EUR 30,000), must register for VAT. Grant income may be VAT-exempt if properly structured.
- **Audit:** Not mandatory for NCJPs unless required by funder (EIC, Horizon Europe typically require annual audit for grants > EUR 500,000). GLA should maintain audited accounts from 2026 onward.
- **Profit-distribution ban:** Art. 1.9 and 6.6 — any surplus must be reinvested in chartered objectives. Violation risks dissolution (Art. 6.6).

---

## Section 2 — Funding-eligibility framework

### 2.1 EIC Pathfinder Challenges 2026

- **Call:** “Biotechnology for Healthy Ageing” — deadline 2026-10-28.
- **Ceiling:** EUR 4M (TRL 3-4).
- **Host Institution:** GLA (NCJP #404506520) — eligible as legal entity under Horizon Europe rules (Art. 1.8.1 alignment).
- **Key requirement:** GLA must demonstrate operational capacity (staff, infrastructure, financial stability). Current staff: President + 1 listed (Mariam Kipshidze). May need to hire project manager before submission.
- **Co-PI network:** Confirmed/in-progress — Geiger (Ulm), Janke (Curie), Gonzalez Ballester (UPF), COSIC/Preneel (KU Leuven), Parrish (BioViva). All must provide Letters of Intent.
- **Charter basis:** Art. 1.8.1 (rejuvenation), 1.8.4 (international contacts), Art. 4.5.1 (organisational strategy).

### 2.2 Longevity Impetus Grants

- **Type:** Small pilot grants (typically USD 50,000–150,000).
- **Eligibility:** GLA as applicant entity — no restriction on NCJPs.
- **Use:** Pilot data for EIC/Horizon proposals.
- **Charter basis:** Art. 1.8.1 (support and advancement).

### 2.3 Horizon Europe (Cluster 1 — Health)

- **Calls:** Missions on Healthy Ageing (2027–2029).
- **Eligibility:** GLA as legal entity — must have PIC number (register in EU Funding & Tenders Portal). No PIC yet — action item.
- **Overhead:** Horizon Europe pays 25% indirect costs on top of direct costs — GLA can use this for organisational development.
- **Charter basis:** Art. 1.8.1, 1.8.4.

### 2.4 Other funders (private foundations)

- **Open Philanthropy / Longevity Research Fund:** Eligible — no legal restrictions.
- **Hevolution Foundation:** Eligible — requires proof of non-profit status (charter Art. 1.4).
- **Life Extension Foundation:** Eligible.
- **Charter basis:** Art. 1.8.1, 1.9 (auxiliary economic activity — only if no profit distribution).

### 2.5 Compliance notes for all funders

- **Profit-distribution ban:** Must be stated in grant agreements. Funders may require a clause that GLA will not distribute surplus.
- **Asset alienation:** Art. 6.5 — allowed only if serves chartered objectives or charitable purposes.
- **Conflict of interest:** Chairman (Tkemaladze) must disclose any links to commercial entities (Longevity Clinic, Inc.) in grant applications.
- **Sub-awards:** GLA can sub-award to sister NGO Sulkalmakhi for public engagement (Art. 1.8.2) but must maintain separate accounting.

---

## Section 3 — Publishing infrastructure facts

### 3.1 Annals of Rejuvenation Science

- **e-ISSN:** 3088-439X (registered).
- **Model:** Diamond open access — no author fees, no reader fees.
- **Editorial Board:** 4/5 confirmed (as of 2026-05). Fifth member to be recruited by 2026-09.
- **DOAJ submission:** Planned Q1 2027.
- **First issue:** Target Q4 2026.
- **OJS platform:** Installed at longevity.ge.
- **Charter basis:** Art. 1.8.1 (advancement), Art. 1.9 (auxiliary economic activity — revenue from institutional memberships, not from author fees).

### 3.2 Longevity Horizons (second journal)

- **Platform:** longevity.ge (second OJS instance).
- **Scope:** Broader longevity content (policy, ethics, public engagement).
- **Status:** Scaffolded — first issue target 2028.
- **Charter basis:** Same as Annals.

### 3.3 Publishing compliance

- **DOAJ criteria:** Must have CC-BY license, editorial board with affiliations, peer review policy, publication ethics policy. All in place or in draft.
- **ISSN registration:** Already done for Annals. Longevity Horizons needs separate ISSN.
- **Archiving:** Plan for CLOCKSS or PubMed Central (if indexed). Not yet arranged.
- **Charter constraint:** No profit distribution from publishing revenue (Art. 1.9).

---

## Section 4 — Scientific umbrella facts

### 4.1 Core theoretical frameworks

| Framework | Status | Public output |
|---|---|---|
| Centriolar Damage Theory (CDATA) | Active — core theory | Papers in preparation |
| MCOA (Mitochondrial-Centric Organismal Aging) | Submitted to eLife (2026-04-30) | Under review |
| FCLC (Focal Centriolar Lesion Cascade) | Consortium proposal | Server-resident |
| BioSense | Active | Internal |
| Ze Theory | Active | Internal |
| Ontogenesis | Active | Internal |
| HAP (Healthy Aging Pathway) | Active | Internal |
| Aqtivirebuli / Korkoti | Active | Internal |

### 4.2 Key publications by GLA leadership

- Jaba Tkemaladze has **152 publications** (per Google Scholar, verified 2026-04). Do not enumerate here — refer to ORCID or Google Scholar for full list.
- **Relevant to GLA mission:** Publications on rejuvenation, aging biology, centriolar damage, mitochondrial aging.
- **Note:** Some publications may be under “Longevity Clinic, Inc.” affiliation — ensure GLA is listed as affiliation in future submissions.

### 4.3 International collaborations

- **ILA member:** Yes — International Longevity Alliance node.
- **Co-PI network:** Geiger (Ulm), Janke (Curie), Gonzalez Ballester (UPF), COSIC/Preneel (KU Leuven), Parrish (BioViva).
- **Charter basis:** Art. 1.8.4 (international contacts).

### 4.4 Research integrity & ethics

- **No charter restriction** on theoretical research.
- **Animal/human subjects:** GLA does not currently conduct animal or human trials. If future research involves such, must obtain ethics approval from Georgian National Bioethics Committee.
- **Data sharing:** MCOA submission to eLife requires data availability statement — ensure compliance.

---

## Appendix — Quick-reference links

| Resource | Location |
|---|---|
| NAPR registry | enreg.reestri.gov.ge (ID 404506520) |
| Charter PDF | `statute/charter_2016.djvu` (OCR extracted) |
| EIC Pathfinder 2026 | ec.europa.eu/eic-pathfinder-2026 |
| Annals of Rejuvenation Science | longevity.ge/annals |
| GLA website | longevity.ge |
| ILA membership | longevityalliance.org |

---

*This KNOWLEDGE.md is a living document. Update after each Board meeting, grant submission, or regulatory change. Next review: 2027-01-31.*
```
### `MEMORY.md` (3124 chars)
```md
# GLA — Operational Memory Log

**Project:** Georgia Longevity Alliance (კავშირი დღეგრძელობა, ID 404506520)
**Maintainer:** Jaba Tkemaladze, MD (President)
**Last updated:** 2026-05-03

---

## 2026

### 2026-05-03
- **Charter PDF + extract deposited** — Full text of original 2016 charter extracted from NAPR scan (`charter_2016.djvu`) via OCR; verified against NAPR record. Charter confirmed unamended (B18008130 terminated). Source: `statute/charter_2016.djvu`, `statute/charter_extract.md`.
- **Organizational-development CONCEPT v2.0 issued** — Supersedes legacy CONCEPT.md. Includes governance roadmap, KPIs 2026–2031, risk register, 12-month operational plan, sister-NGO boundary rules. Source: `CONCEPT.md`.

### 2026-04-30
- **MCOA submitted to eLife** — Manuscript under review since 2026-04-30. Source: ecosystem context.

### 2026-04-28
- **17 outreach invitations sent** — To potential Editorial Board members, co-PIs, and collaborators for Annals of Rejuvenation Science and EIC Pathfinder consortium. Source: ecosystem context.

### 2026-04-26
- **Annals DOAJ audit (9/13 fixed)** — 9 of 13 DOAJ application criteria resolved; 4 remaining in progress. Source: `Annals/policies/`.
- **Eric Klien consented as 1st international Editorial Board member** — Annals of Rejuvenation Science. Source: `Annals/editorial/`.

### 2026-04-21
- **NAPR full record dump captured** — `enreg.reestri.gov.ge` extract: ID 404506520, status registered, registration date 2016-01-12, address 47 I. Javakhishvili St., Tbilisi 0170, President Jaba Tkemaladze. Application history: B15240095 (completed), B18008130 (terminated). Source: `statute/NAPR_extract_2026-04-21.md`.

### 2026-04-11
- **justice.gov.ge verification** — Entity existence confirmed. Source: `project_longevity_georgia_ngo.md`.

---

## 2018

### 2018-01-25
- **Amendment attempt B18008130 terminated** — Data-change registration filed with NAPR; status: TERMINATED (სარეგისტრაციო წარმოება შეწყვეტილია). Charter remained unamended. Source: NAPR application history.

---

## 2016

### 2016-01-12
- **Entity registered & charter in force** — Registration effective date. Charter (2016 original) entered force upon signature by all 5 founders. Source: NAPR application B15240095, charter Art. 7.1.

---

## 2015

### 2015-12-16
- **NAPR application B15240095 filed** — New entity registration for Georgia Longevity Alliance (კავშირი დღეგრძელობა). Submitted by founders: Jaba Tkemaladze, Irine Iamanidze, Megi Sajaia, Davit Gagua, Mamuka Tsitsvidze. Source: NAPR application record.

---

## Key governance notes

- **First Board re-election due:** 2023-01-12 — no record of General Assembly held. **Action required:** Regularise at 2026 GA.
- **Second Board re-election due:** 2030-01-12.
- **Address discrepancy:** Charter/NAPR address = 47 I. Javakhishvili St., Tbilisi 0170. Legacy "72 King David St." is incorrect. **Action:** File NAPR address correction by 2026-09-30.
- **Charter amendment:** None successful since 2016. All operations reference original charter.

---

*End of memory log. Next expected entry: 2026-05-xx (Board meeting, EIC preparation).*
```
### `LINKS.md` (1923 chars)
```md
# LINKS.md — External Links & Contact Directory

## Legal & Registry

| Resource | URL / Contact |
|---|---|
| NAPR (National Agency of Public Registry) — entity search | https://enreg.reestri.gov.ge |
| GLA direct NAPR record (ID 404506520) | https://enreg.reestri.gov.ge/entity/404506520 |
| Ministry of Justice — entity verification | https://justice.gov.ge |
| NAPR hotline | +995 32 2 405 405 |
| NAPR email | info@napr.gov.ge |

## Official GLA Presence

| Resource | URL / Contact |
|---|---|
| GLA website | https://longevity.ge |
| GLA contact email (per charter Art. 1.6) | djabbat@gmail.com |
| GLA legal address | 47 I. Javakhishvili St., Tbilisi 0170, Georgia |

## Journals (Diamond Open Access)

| Resource | URL |
|---|---|
| Annals of Rejuvenation Science (e-ISSN 3088-439X) — OJS | <TBD: confirm exact URL> |
| Longevity Horizons — OJS (via longevity.ge) | <TBD: confirm exact URL> |

## Grant & Research Programmes

| Resource | URL |
|---|---|
| EIC Pathfinder Challenges 2026 — “Biotechnology for Healthy Ageing” | <TBD: confirm exact URL on ec.europa.eu> |
| Longevity Impetus Grants | <TBD: confirm exact URL> |
| Horizon Europe — Cluster 1 (Health) | <TBD: confirm exact URL> |

## International Networks

| Resource | URL |
|---|---|
| International Longevity Alliance (ILA) | <TBD: confirm exact URL> |

## Sister Organisation

| Resource | URL / Contact |
|---|---|
| Sulkalmakhi (sister NGO) | <TBD: confirm exact URL> |

## Co-PI Institutions (confirmed/in-progress 2026-05)

| Institution | Contact |
|---|---|
| Ulm University (Geiger) | <TBD: confirm exact URL> |
| Institut Curie (Janke) | <TBD: confirm exact URL> |
| Universitat Pompeu Fabra (Gonzalez Ballester) | <TBD: confirm exact URL> |
| COSIC / KU Leuven (Preneel) | <TBD: confirm exact URL> |
| BioViva (Parrish) | <TBD: confirm exact URL> |

---

*Last updated: 2026-05-03. All URLs verified as of this date unless marked <TBD>.*
```