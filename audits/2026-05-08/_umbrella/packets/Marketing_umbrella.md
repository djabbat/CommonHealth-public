# AUDIT PACKET — Marketing_umbrella

Path: `/home/oem/Desktop/Marketing`  Date: 2026-05-08

## Size & file counts
```
19M	/home/oem/Desktop/Marketing
```
**Extensions:** .md=162, .docx=89, (noext)=2, .py=1, .sh=1, .pdf=1, .service=1
## Tree (depth=2, max 200 entries)
```
.
./Books
./Books/PARAMETERS.md
./Books/Diets
./Books/Ze_Theory_drjaba_patch.md
./Books/README.md
./Books/CONCEPT_CODE_AUDIT_2026-04-21.md
./Books/UPGRADE.md
./Books/Integrative
./Books/TODO.md
./Books/MEMORY.md
./Books/CLAUDE.md
./Books/LINKS.md
./Books/Kartvely
./Books/docs
./Books/CONCEPT.md
./Books/Ze_Theory_Launch.md
./Books/autoresponder
./Books/MAP.md
./Books/KNOWLEDGE.md
./Books/24
./PARAMETERS.md
./JabaEkimi
./JabaEkimi/MONETIZATION_PLAN.docx
./JabaEkimi/LEAD_MAGNET.docx
./JabaEkimi/KPI.docx
./JabaEkimi/CONTENT_STRATEGY.md
./JabaEkimi/SPONSORS.docx
./JabaEkimi/CONTENT_STRATEGY.docx
./JabaEkimi/SPONSORS.md
./JabaEkimi/CONCEPT.docx
./JabaEkimi/README.md
./JabaEkimi/LEAD_MAGNET.md
./JabaEkimi/MONETIZATION_PLAN.md
./JabaEkimi/KPI.md
./JabaEkimi/ACTIONS.md
./JabaEkimi/ACTIONS.docx
./JabaEkimi/docs
./JabaEkimi/CONCEPT.md
./README.md
./UPGRADE.md
./TODO.md
./MEMORY.md
./CLAUDE.md
./THEORY.md
./LINKS.md
./CONCEPT.md
./MAP.md
./KNOWLEDGE.md
```
## Detected stack: **unknown**
## Core files

### `CLAUDE.md` (1488 chars)
```md
# CLAUDE.md — Marketing umbrella

## Identity

**Project:** Marketing (umbrella, was `DrJaba/` until 2026-04-30)
**Location:** `~/Desktop/Marketing/`
**Sub-projects:** `JabaEkimi/` (YouTube), `Books/` (Ze Theory + future titles)

## Source of truth

`CONCEPT.md` — authoritative for the umbrella. Each sub-project has its own
CONCEPT inside its folder.

## Critical rules

1. **NOT to confuse with `djabbat/DrJaba` (drjaba.com)** — that's the
   server-side clinic website, different repo, different layer. Local
   Marketing/ is the **funnel** towards drjaba.com checkout, not the site
   itself.
2. **Commercial flow** — Tkemaladze personal / drjaba.com Stripe. NOT GLA
   or Sulkalmakhi NGO accounting.
3. **Cross-channel reuse** — Sponsors catalogue, lead magnets, KPI tracker
   live umbrella-level so JabaEkimi and Books share them.
4. **No-ISBN strategy for Books** is canonical (NPLG denied e-book ISBN
   2026-04-29). Don't try to apply for ISBN again unless user explicitly
   asks.

## Domain split (canonical)

| Need | Use |
|---|---|
| YouTube → drjaba.com funnel | JabaEkimi |
| Book sales (Ze Theory v1, future titles) | Books |
| Clinic website / drjaba.com bug fixes | server-side `djabbat/DrJaba` |
| Longevity science / grants | GLA |
| Local civic projects | Sulkalmakhi |
| PhD / dissertation / E0 | PhD |

## What lives here

- `CONCEPT.md` (umbrella)
- `JabaEkimi/` — channel ops + sponsor outreach + content strategy
- `Books/` — Ze Theory launch + future titles

```
### `Books/CLAUDE.md` (841 chars)
```md
# CLAUDE.md — Books

---

## Startup Protocol

**Полные правила:** `~/Desktop/Claude/protocols/START.md`

---

## Структура

Монорепо. Каждый подпроект — отдельная папка:

| Папка | Статус |
|-------|--------|
| `Space/` | 🟡 Активен |
| `24/` | 🟡 Активен |
| `Kartvely/` | 🟡 Активен |
| `Diets/` | 🟡 Активен |
| `Integrative/` | ✅ Closed 2026-04-19 (archived) |

---

## Правила работы

- **Все тексты через DeepSeek API** (`~/Desktop/AIM/llm.py`)
- **Переводы** на 4 языка (RU/EN/KA/DE или по запросу) — через Qwen
- **Peer review** → до ACCEPT, без промежуточных файлов
- **docx** → только через `md_to_docx.py`
- **Git:** только один репо `djabbat/Books` (private, нет public)

---

## Что писать через DeepSeek

- Главы, разделы, предисловия, послесловия
- Редактура, полировка стиля
- Перевод глав
- Аннотации, синопсисы, питч издателю

```
### `README.md` (1234 chars)
```md
# Marketing — umbrella

Lead-generation / sales / media tracks for Tkemaladze, all under one folder.

```
Marketing/
├── CONCEPT.md         — umbrella concept (source of truth)
├── README.md          — this file
├── CLAUDE.md          — instructions for Claude Code
├── TODO.md            — umbrella-level tasks (subproject TODOs are inside each)
├── PARAMETERS.md      — KPI targets, financial parameters
├── MAP.md             — file structure + cross-references
├── MEMORY.md          — between-session context
├── LINKS.md           — public URLs (channels, payment, distributors)
├── KNOWLEDGE.md       — domain knowledge: KDP/D2D rules, Stripe/Gumroad/Payhip terms, YouTube YPP
├── UPGRADE.md         — milestone tracker (4-step Ze launch + JabaEkimi monetisation)
├── THEORY.md          — funnel theory (umbrella → drjaba.com checkout)
├── JabaEkimi/         — YouTube channel @ჯაბაექიმი (own README/CONCEPT/SPONSORS/…)
└── Books/             — Ze Theory + future titles, no-ISBN distribution (own 11-file core)
```

**NOT** the same as server-side `djabbat/DrJaba` (drjaba.com clinic site). This umbrella is the **funnel**, not the site.

Subprojects each maintain their own core files. Files at this level are umbrella-only.

```
### `Books/README.md` (442 chars)
```md
# Books

Монорепозиторий книжных проектов Jaba Tkemaladze.

| Проект | Жанр | Статус |
|--------|------|--------|
| `Space/` | Нон-фикшн · нумерология · практики | 🟡 Активен |
| `24/` | Художественный роман | 🟡 Активен |
| `Kartvely/` | История Грузии | 🟡 Активен |
| `Diets/` | Научпоп · диетология | 🟡 Активен |
| `Integrative/` | Научпоп · интегративная медицина | ✅ Closed 2026-04-19 (archived) |

**Git:** `djabbat/Books` (private only)

```
### `JabaEkimi/README.md` (1165 chars)
```md
# JabaEkimi — YouTube-подпроект DrJaba

Подпроект экосистемы **DrJaba**: монетизация и развитие YouTube-канала [@ჯაბაექიმი](https://www.youtube.com/@%E1%83%AF%E1%83%90%E1%83%91%E1%83%90%E1%83%94%E1%83%A5%E1%83%98%E1%83%9B%E1%83%98).

## Структура

| Файл | Назначение |
|---|---|
| `CONCEPT.md` | Мастер-документ: позиционирование, архитектура, правила |
| `MONETIZATION_PLAN.md` | 5-этапный план монетизации, финмодель |
| `SPONSORS.md` | 20 целевых longevity-спонсоров + email-шаблон |
| `LEAD_MAGNET.md` | Источник PDF «Чек-лист биомаркеров старения» |
| `CONTENT_STRATEGY.md` | Контент-план, Shorts, расписание, коллаборации |
| `KPI.md` | Метрики и ежемесячный трекер |
| `ACTIONS.md` | Чек-листы 7 / 30 / 90 дней |
| `docs/` | Переписка, черновики, peer-review (gitignored) |

## Текущий статус

- 2 270 подписчиков · 4 100 часов просмотра · YPP: ✅ полное соответствие, заявка не подана
- Цель 12 мес.: $2 650–10 400/мес (рост-сценарий, ×5 подп.)

## Правила

- Подпроект **без своего git** (subproject git rule)
- Все `.md` кроме README = ядро, генерируется из `CONCEPT.md`
- `.docx` собираются через `~/Desktop/Claude/scripts/md_to_docx.py`
- Язык: русский

```
### `CONCEPT.md` (3212 chars)
```md
# Marketing — umbrella for Tkemaladze's lead-generation tracks

**Status:** umbrella project (renamed from DrJaba 2026-04-30)
**Location:** `~/Desktop/Marketing/`
**NOT to confuse with:** server-side `djabbat/DrJaba` (drjaba.com clinic website) — that's a separate repo, different layer.

## What this umbrella covers

Все каналы лидогенерации/продаж/медиа Tkemaladze под одной крышей.

```
Marketing/
├── CONCEPT.md         ← this file
├── JabaEkimi/         ← YouTube channel @ჯაბაექიმი
└── Books/             ← Ze Theory + future titles, no-ISBN distribution
```

(Future: PDF lead magnets / email funnels / sponsorships / courses → new sub-folders here.)

## Sub-projects

### JabaEkimi — YouTube channel

- **Location:** `~/Desktop/Marketing/JabaEkimi/`
- **Channel:** [@ჯაბაექიმი](https://www.youtube.com/@%E1%83%AF%E1%83%90%E1%83%91%E1%83%90%E1%83%94%E1%83%A5%E1%83%98%E1%83%9B%E1%83%98) ("Doctor Jaba")
- **Status (2026-04-30):** 2,270 subscribers · 4,100 watch hours · ✅ YPP eligible (1k+4k threshold met) · application not yet filed
- **Role:** medical / longevity content → funnel into drjaba.com consultations + books + products
- **Core files:** README, CONCEPT, MONETIZATION_PLAN, SPONSORS (20 longevity sponsor targets), LEAD_MAGNET (PDF "30 biomarker checklist"), CONTENT_STRATEGY

### Books — Ze Theory commercial launch

- **Location:** `~/Desktop/Marketing/Books/`
- **First title:** Ze Theory (Tkemaladze's monograph; final version v10)
- **Strategy (decided 2026-04-29):** NO-ISBN path (NPLG denied e-book ISBN). 4-step distribution covers ~90% world ebook market at €0 cost.
- **Source of truth:** `Books/Ze_Theory_Launch.md`
- **D2D account active 2026-05-01** — imprint = "Longevity Horizon Press", PayPal djabbat@gmail.com, W-8BEN submitted.

#### 4-step launch chain

| # | Channel | Royalty | Status |
|---|---|---|---|
| 1 | Amazon KDP (Kindle ASIN, no ISBN) | 70% @ $2.99-9.99 | ⏳ pending |
| 2 | drjaba.com direct (Stripe — already wired on space.drjaba.com) | 97% margin | ⏳ pending |
| 3 | Gumroad + Payhip (direct alternatives) | 91-95% | ⏳ pending |
| 4 | Draft2Digital aggregator → Apple/Kobo/B&N (free ISBN) | varies | ✅ account ready, pending upload |

## What this umbrella does NOT cover

- **Clinic website (drjaba.com)** — server-side, repo `djabbat/DrJaba` on jaba@server. Separate project.
- **Longevity scientific publishing** — uses GLA NGO entity, lives in `~/Desktop/GLA/` and `~/Desktop/LongevityCommon/`.
- **Local civic / cultural projects** — uses Sulkalmakhi NGO, lives in `~/Desktop/Sulkalmakhi/`.

## Cross-project notes

- **drjaba.com sales (Stripe)** is the consultation funnel endpoint — both JabaEkimi (CTAs in videos) and Books (royalty checkout #2) point at it.
- **Sponsors list (JabaEkimi/SPONSORS.md, 20 longevity brands)** is the catalog for cross-promotion — also reusable for Books press releases when launching.
- **Common KPI tracker** (Books velocity, JabaEkimi CTR-to-funnel) — TODO once first Books are live.

## Ownership / financials

All under Tkemaladze personal entity (drjaba.com / Longevity Clinic Inc. / Stripe). NOT under GLA or Sulkalmakhi NGO accounting — these are commercial revenue streams, separate from charity/grant flows.

```
### `Books/CONCEPT.md` (501 chars)
```md
# Books — Книжные проекты

Монорепозиторий всех книжных проектов Jaba Tkemaladze.

## Проекты

| Папка | Название | Статус |
|-------|----------|--------|
| `Space/` | «Место Силы» — нумерология + практики | 🟡 Активен |
| `24/` | «24 Жизни» — роман | 🟡 Активен |
| `Kartvely/` | История Грузии (книга) | 🟡 Активен |
| `Diets/` | Диетология | 🟡 Активен |
| `Integrative/` | Интегративная медицина | ✅ Closed 2026-04-19 (archived) |

## Репозиторий

- **Git:** `djabbat/Books` (private only, no public)

```
### `JabaEkimi/CONCEPT.md` (3852 chars)
```md
# CONCEPT — JabaEkimi (@ჯაბაექიმი)

## Идентичность

**Канал:** YouTube `@ჯაბაექიმი` (JabaEkimi = «Доктор Джаба»)
**Подпроект DrJaba:** медиа-актив + воронка лидогенерации в `drjaba.com`.

## Миссия

Превратить YouTube-канал из публикационной площадки в **многоканальную систему монетизации**, где AdSense — побочный доход (<10%), а основной поток — спонсоры longevity-индустрии, консультации через drjaba.com и собственные продукты (книга Ze, курсы, PDF-гайды).

## Позиционирование

- **Кто говорит:** Jaba Tkemaladze, MD, CEO Georgia Longevity Alliance, longevity-исследователь.
- **О чём:** доказательная longevity-медицина, центриоли & differentiation, биомаркеры старения, протоколы для пациентов клиники.
- **Тон:** экспертный, без хайпа, с цитированием первоисточников; региональная (Грузия/post-Soviet) + международная аудитория.
- **Языки:** грузинский (основной), русский (вторичный), английский (для мировой longevity-аудитории — отдельный трек).

## Текущий статус (2026-04-30)

- Подписчиков: **2 270**
- Часов просмотра (12 мес.): **4 100**
- Соответствие YPP: ✅ полное (1k подп. + 4k часов)
- Монетизация: **не активирована** — заявка не подана.

## Архитектура монетизации

```
┌──────────────────────────────────────────────────────┐
│              YouTube @ჯაბაექიმი                     │
│     (контент-движок: 2 long-form + 5 Shorts/неделя)  │
└──────────────────────────────────────────────────────┘
            │            │             │
            ▼            ▼             ▼
    ┌──────────────┐ ┌─────────┐  ┌──────────────┐
    │ YPP-модули   │ │ Внешняя │  │ Воронка      │
    │ (AdSense,    │ │ монетиз.│  │ drjaba.com   │
    │  Members,    │ │ (sponsors│ │ (консульт.,  │
    │  Super Thanks│ │  affil., │ │  клиника)    │
    │  Shopping)   │ │  Patreon)│ │              │
    └──────────────┘ └─────────┘  └──────────────┘
                          │             │
                          ▼             ▼
                    Email-воронка → Курсы / Книга Ze / PDF-гайды
```

## Стратегические оси

1. **Монетизация (этот подпроект)** — YPP + внешние источники.
2. **Контент-стратегия** — темы, формат, частота, Shorts-стратегия.
3. **Воронка** — YouTube → email → drjaba.com → консультация/клиника.
4. **Аналитика** — KPI-трекер, ежемесячный пересмотр.

## Целевая финансовая модель (12 мес.)

| Сценарий | Подпис. | Доход/мес |
|---|---|---|
| База (×2) | 4.5k | $490–3060 |
| Рост (×5) | 10k | $2650–10400 |

Бутылочное горлышко — **не подписчики, а конверсия** в платные продукты/консультации drjaba.com.

## Ядро файлов (генерируется из CONCEPT)

- `README.md` — вход в подпроект (НЕ ядро)
- `MONETIZATION_PLAN.md` — мастер-план 5 этапов + KPI
- `SPONSORS.md` — 20 longevity-спонсоров + email-шаблон
- `LEAD_MAGNET.md` — PDF «Чек-лист биомаркеров старения» (источник)
- `CONTENT_STRATEGY.md` — темы видео, Shorts, расписание, коллаборации
- `KPI.md` — метрики и ежемесячный трекер
- `ACTIONS.md` — checklist на 7/30/90 дней

## Правила работы

- Подпроект **не имеет своего git** (subproject git rule).
- Core-файлы генерируются из CONCEPT.md; при изменении CONCEPT — пересобрать ядро.
- Все `.md` (кроме README) = ядро.
- Письма к спонсорам / черновики переписки → `docs/` (gitignored).
- Конверсия в `.docx` — только через `~/Desktop/Claude/scripts/md_to_docx.py`.
- Язык документов проекта: русский.

## Связи с экосистемой

- **drjaba.com** — конечная воронка (консультации, клиника).
- **GLA / Annals of Rejuvenation Science** — источник тем для видео (свежие публикации journal).
- **Книга Ze** — продукт для продажи через канал (4-step NO-ISBN план уже зафиксирован).
- **LongevityCommon NEWS.md** — источник новостных тем для Shorts.

## Точка пересмотра

Через 30 дней после одобрения YPP: оценить реальный CPM, конверсию в подписку Memberships, отклик спонсоров на cold-питч. Перерасчёт финмодели.

```
### `THEORY.md` (3741 chars)
```md
# THEORY.md — Marketing umbrella

> **Note:** THEORY.md per project-core rule = "core hypotheses, axioms, paradigm basis".
> Для коммерческого umbrella это _funnel theory_ — не научная гипотеза, а
> операционная модель воронки. Файл компактный по дизайну.

---

## Аксиома 1 — Один endpoint на все каналы

Все 3 канала (`JabaEkimi`, `Books`, drjaba.com прямой трафик) ведут в **один Stripe checkout** через drjaba.com. Не дублировать инфраструктуру checkout per channel.

**Следствие:** Books Step 2 = добавить products в существующий Stripe (от space.drjaba.com), не отдельная интеграция.

## Аксиома 2 — Tkemaladze personal entity, не NGO

Все commercial revenue идут через персональную сущность (drjaba.com / Stripe / PayPal djabbat@gmail.com). GLA и Sulkalmakhi — отдельный non-commercial бухучёт. **Никаких commercial transactions через NGO accounts.**

## Аксиома 3 — Lead magnet = единая точка email-сбора

Один PDF ("30-biomarker checklist") размещается на drjaba.com и используется всеми тремя каналами. Не плодить tomes per channel.

## Гипотеза 1 — Funnel velocity

| Канал | Top-of-funnel volume | Conversion to email | Conversion to checkout |
|---|---|---|---|
| JabaEkimi (YouTube) | 4k watch hours/12mo | 0.3 % (≈ 12) | 5 % of email = 0.6 |
| Books (Amazon/D2D) | 500 sales/90d (target) | 5 % of buyers (= 25) | 10 % = 2.5 |
| drjaba.com прямой | TBD | 10 % | 20 % |

**Если гипотеза верна** → за 90 дней ~3 consultation bookings из Books-funnel + ~1 из YouTube. _Verify post-launch._

## Гипотеза 2 — Cross-channel multiplier

Запуск Books **усиливает** JabaEkimi sub growth (Book reader → channel sub conversion ≈ 15 %), а **JabaEkimi mention** усиливает Books velocity (sub → reader conversion ≈ 1–3 %).

**Не linear, а multiplicative.** Поэтому одновременный launch Step 1-4 предпочтительнее последовательного через 6 месяцев.

## Гипотеза 3 — Long-tail Ze Theory

Ze Theory как _foundational_ longevity text может иметь long-tail (5+ лет sales декам), а не steep peak-and-decline. Хорошие точки сравнения: «Lifespan» Sinclair (2019) — sustained 3+ года; «Outlive» Attia (2023) — год+.

**Следствие:** не оптимизировать только под launch peak. Investing в ongoing visibility (continuous content в JabaEkimi → Book mentions) > burst paid ads.

## Гипотеза 4 — Грузинский рынок ≈ 0 для Books

E-book infrastructure в Грузии не развита, NPLG отказала в ISBN (2026-04-29). **Все Books продажи — экспорт.** Ru/En audience через global retailers.

JabaEkimi — другая история: грузинская диаспора (Россия/EU) + ru-speakers — комбинированный target ~50k активных longevity-interested viewers.

---

## Paradigm — что отличает от типичного marketing

1. **Author = founder = clinic owner** (personal brand, не corporate). Это даёт authenticity, но требует self-disclosure ограничений (privacy, medical liability).
2. **Niche cross-section** — longevity science × clinical medicine × Georgian language. Уникальность даёт SEO advantage в узких queries; ограничивает scale.
3. **Non-VC-backed, bootstrap** — нет paid ads до YPP approval; всё органика + cross-promotion. Длиннее, но сохраняет 95 %+ margin.
4. **Multi-language asymmetry** — content в JabaEkimi грузинский (нативный, но узкий рынок); Books английский (широкий, но не нативный → требует pro editing).

---

## Метрики проверки теории (90 дней после Step 1 launch)

- [ ] Books total sales ≥ 500 (Гипотеза 1)
- [ ] JabaEkimi sub gain ≥ 1,500 за 90 дней после Books launch (Гипотеза 2)
- [ ] Cross-channel referral rate ≥ 10 % (Гипотеза 2)
- [ ] Long-tail check: month-3 sales ≥ 30 % month-1 sales (Гипотеза 3)
- [ ] Грузинская доля Books revenue < 5 % (Гипотеза 4)

Если ≥3 из 5 подтверждаются → теория валидна для следующих titles. Если ≤2 → пересмотр funnel/channels.

```
### `MAP.md` (3689 chars)
```md
# MAP.md — Marketing umbrella

Структура + связи с экосистемой. Источник истины — `CONCEPT.md`.

---

## 1. Файловая структура

```
Marketing/
├── CONCEPT.md             — концепт umbrella
├── README.md              — точка входа
├── CLAUDE.md              — инструкции Клоду
├── TODO.md                — umbrella tasks (Step 1-4 launch + YPP)
├── PARAMETERS.md          — pricing, KPI, бюджет, tax
├── MAP.md                 — этот файл
├── MEMORY.md              — между-сессионный контекст
├── LINKS.md               — публичные URLs
├── KNOWLEDGE.md           — правила KDP/D2D/Stripe/Payhip/YouTube YPP
├── UPGRADE.md             — milestones
├── THEORY.md              — funnel theory (umbrella → drjaba.com checkout)
│
├── JabaEkimi/             — YouTube subproject (свой README/CONCEPT)
│   ├── README.md
│   ├── CONCEPT.md
│   ├── ACTIONS.md
│   ├── CONTENT_STRATEGY.md
│   ├── KPI.md
│   ├── LEAD_MAGNET.md
│   ├── MONETIZATION_PLAN.md
│   └── SPONSORS.md (20 longevity brand targets)
│
└── Books/                 — Books subproject (полный 11-file core)
    ├── CONCEPT.md
    ├── README.md
    ├── CLAUDE.md
    ├── TODO.md
    ├── PARAMETERS.md
    ├── MAP.md
    ├── MEMORY.md
    ├── LINKS.md
    ├── KNOWLEDGE.md
    ├── UPGRADE.md
    ├── Ze_Theory_Launch.md          — source of truth для 4-step launch
    ├── Ze_Theory_drjaba_patch.md
    ├── autoresponder/
    ├── Diets/                       — отдельная книга
    ├── Integrative/                 — отдельная книга
    ├── Kartvely/                    — отдельная книга
    ├── 24/                          — отдельная книга / манускрипт
    └── docs/
```

## 2. Поток конверсии (funnel)

```
JabaEkimi видео   ──┐
                    ├──▶  drjaba.com  ──▶  Stripe checkout
Books reader     ───┤        │
                    │        ├──▶ consultation booking
Lead magnet PDF  ───┘        ├──▶ book direct (Step 2)
                             └──▶ AIM /chat (если активен)
```

Lead magnet "30-biomarker checklist" — общая точка для всех 3 каналов (живёт в `JabaEkimi/LEAD_MAGNET.md`, размещается на drjaba.com landing).

## 3. Связи с экосистемой

| Соседний проект | Связь |
|---|---|
| `~/Desktop/PhD/` | Ze Theory — это Ze (LongevityCommon), не PhD. Books не пересекается с PhD напрямую |
| `~/Desktop/LongevityCommon/Ze/` | Ze Theory v10 — текст книги растёт оттуда; обновления Ze ⇒ ревизия книги |
| `~/Desktop/GLA/` | Annals/Longevity Horizon — научное publishing, **другая воронка** (academic). Не путать с commercial Books |
| `~/Desktop/LongevityCommon/AIM/` | AIM `/chat` — может быть consult-funnel endpoint |
| Server `djabbat/DrJaba` (drjaba.com) | Site, на который воронка ведёт. Изменения в site — там, не здесь |
| Server `space.drjaba.com` | Уже-wired Stripe — Books Step 2 берёт checkout оттуда |
| Server `books.drjaba.com` | Static landing для Books — синхронизация с этим umbrella |

## 4. Что **не** входит

- Sciences/grants — `GLA/`
- Civic/eco — `Sulkalmakhi/`
- Clinic site code — server `djabbat/DrJaba`
- Wellness резорт — `WLRAbastumani/`

## 5. Git

- Repo: `git@github.com:djabbat/Marketing-private.git` (private, верификация ✅)
- `Books/` — внутри umbrella, **без** собственного git (subproject git rule, `feedback_subproject_git_rule`)
- `JabaEkimi/` — то же

## 6. Open structural questions

1. `Books/CONCEPT.md` упоминает `Space/` как 🟡 active, но папки `Books/Space/` нет. Решить: удалить упоминание (Space перенесён на сервер) или ввести stub.
2. `JabaEkimi/*.docx` дубликаты `.md` — устарели после переписывания через DeepSeek? Решить, удалить.
3. `Books/Integrative/`, `Books/Diets/`, `Books/Kartvely/`, `Books/24/` — статус каждой книги в `Books/MEMORY.md` зафиксировать.

```
### `Books/MAP.md` (1016 chars)
```md
# MAP — Books

```
Books/
├── CONCEPT.md          — обзор всех книжных проектов
├── README.md           — публичное описание
├── CLAUDE.md           — инструкции для Claude
├── TODO.md             — задачи
├── PARAMETERS.md       — параметры каждого проекта
├── MAP.md              — этот файл
├── MEMORY.md           — история решений
├── LINKS.md            — ссылки (издатели, агенты, референсы)
├── KNOWLEDGE.md        — накопленные знания о книгоиздании
├── UPGRADE.md          — план улучшений
│
├── Space/              — «Место Силы»
│   ├── CONCEPT.md
│   ├── TODO.md
│   ├── Archive/
│   └── Materials/
│
├── 24/                 — «24 Жизни» (роман)
│   └── CONCEPT.md
│       └── Book/
│
├── Kartvely/           — История Грузии 🟡
│   ├── CONCEPT.md
│   └── Archive/
│
├── Diets/              — Диетология 🟡
│   ├── CONCEPT.md
│   └── Archive/
│
└── Integrative/        — Интегративная медицина ✅ Closed 2026-04-19 (archived)
    └── (moved to ~/Documents/Archive/Books/Integrative_closed_2026-04-19/)
```

```
### `PARAMETERS.md` (2906 chars)
```md
# PARAMETERS.md — Marketing umbrella

KPI targets, ценовые параметры, financial constraints. Источник истины — `CONCEPT.md` + субпроектные параметры.

---

## 1. Books — Ze Theory pricing

| Канал | Ценовой диапазон | Royalty (net) | Источник истины |
|---|---|---|---|
| Amazon KDP (Kindle) | $2.99–9.99 | 70 % @ $2.99–9.99 ($4.99 = $3.49 net) | KDP terms |
| drjaba.com Stripe | $4.99–14.99 | ~97 % (Stripe ~3 %) | drjaba.com checkout |
| Gumroad | $4.99–14.99 | ~91 % (Gumroad 9 % flat) | Gumroad terms |
| Payhip | $4.99–14.99 | ~95 % (Payhip 5 % flat) | Payhip terms |
| D2D → Apple/Kobo/B&N | $4.99–9.99 | 60–70 % author share (varies) | D2D terms |

**Цель цены v1 (рекомендация):** $4.99 launch, $9.99 standard.
**Imprint:** Longevity Horizon Press (D2D).
**PayPal для D2D:** djabbat@gmail.com (W-8BEN submitted 2026-05-01).

## 2. Books — KPI targets (90 дней после Step 1)

| Метрика | Цель |
|---|---|
| Total downloads/sales (все каналы) | ≥ 500 |
| Stripe direct sales (Step 2) | ≥ 100 |
| KDP free promo days used | ≤ 5 (5/90) |
| Average rating (Goodreads + Amazon) | ≥ 4.0 / 5 |
| Reviews count | ≥ 25 |
| Cross-funnel: drjaba.com consult requests из Book buyers | ≥ 10 |

## 3. JabaEkimi — YouTube KPI

| Метрика | Текущее (2026-04-30) | Цель Q3 2026 |
|---|---|---|
| Subscribers | 2,270 | 5,000 |
| Watch hours (rolling 12mo) | 4,100 | 8,000 |
| YPP status | eligible, not applied | approved |
| AdSense monthly | $0 | $50–150 |
| Sponsor revenue | $0 | $200–500 (1–2 sponsors) |
| Lead-magnet email signups (drjaba.com → biomarker PDF) | 0 | 200 |
| Consult conversion (sub → drjaba.com booking) | unknown | ≥ 0.5 % |

## 4. Бюджет marketing

Жёстких лимитов нет (commercial entity), но рекомендуемые правила:

| Категория | Лимит/месяц | Комментарий |
|---|---|---|
| Платная реклама (FB/IG/Google) | $0 | до YPP approval — органика only |
| Tools/SaaS (Mailchimp/Convertkit/Canva Pro) | $50 | минимум, Canva Pro для cover-art уже оплачен через drjaba |
| D2D/KDP (one-time setup) | $20 | ✅ потрачено 2026-05-01 (D2D) |
| Cover-art / editing freelance | $200 | если v1 потребует доработки после Step 1 launch |
| **Итого/месяц** | **$50** | стартовый, повышается после первой выручки |

## 5. Финансовые сущности

- **Все commercial revenue → Tkemaladze personal entity** (drjaba.com / Longevity Clinic Inc. / Stripe).
- **НЕ под GLA или Sulkalmakhi NGO** (некоммерческие, отдельный бухучёт).
- **Stripe wired through space.drjaba.com** (уже работает) — не нужно отдельной интеграции для Books Step 2.
- **PayPal djabbat@gmail.com** — D2D + Gumroad + Payhip.

## 6. Compliance / tax

- W-8BEN submitted to D2D 2026-05-01 (Georgia tax resident, no US source income above 30 % WH).
- KDP — потребуется отдельный W-8BEN (Amazon KDP form) перед Step 1.
- Stripe — Tbilisi-based business, локальный VAT не применим к exports.
- Georgia VAT: цифровые товары для нерезидентов — 0 % (export).

```
### `Books/PARAMETERS.md` (1043 chars)
```md
# PARAMETERS — Books

## Space (Место Силы)

| Параметр | Значение |
|----------|---------|
| Жанр | Нон-фикшн, практическое руководство |
| Языки | RU (основной), EN, KA |
| Аудитория | Взрослые, интерес к нумерологии и телесным практикам |
| Объём | ~200 стр. |
| Структура | Теория + упражнения + нумерология |

## 24 (Роман)

| Параметр | Значение |
|----------|---------|
| Жанр | Художественный роман |
| Язык | RU |
| Аудитория | Общая |
| Концепция | CONCEPT v3.0 |
| Объём | TBD |

## Kartvely (История Грузии)

| Параметр | Значение |
|----------|---------|
| Жанр | Историческая книга |
| Язык | RU / KA |
| Статус | 🟡 Активен |

## Diets (Диетология)

| Параметр | Значение |
|----------|---------|
| Жанр | Научпоп |
| Язык | RU |
| Версия | v5 (последняя) |
| Статус | 🟡 Активен |

## Integrative (Интегративная медицина)

| Параметр | Значение |
|----------|---------|
| Жанр | Научпоп / медицина |
| Язык | RU |
| Статус | ✅ Closed 2026-04-19 (archived) |
| Архив | `~/Documents/Archive/Books/Integrative_closed_2026-04-19/` |

```
### `UPGRADE.md` (2752 chars)
```md
# UPGRADE.md — Marketing umbrella

Milestones. ✅ ставится сразу после выполнения.

---

## v1.0 — Umbrella formed ✅ 2026-04-30

- [x] Папка переименована `DrJaba/` → `Marketing/` ✅ 2026-04-30
- [x] CONCEPT.md написан ✅ 2026-04-30
- [x] CLAUDE.md написан ✅ 2026-04-30
- [x] JabaEkimi/ перенесён внутрь ✅ 2026-04-30
- [x] Books/ перенесён внутрь ✅ 2026-04-30
- [x] Repo `djabbat/Marketing-private` создан ✅ 2026-04-30

## v1.1 — D2D readiness ✅ 2026-05-01

- [x] D2D account opened ($20) ✅ 2026-05-01
- [x] Imprint "Longevity Horizon Press" registered ✅ 2026-05-01
- [x] PayPal djabbat@gmail.com linked ✅ 2026-05-01
- [x] W-8BEN submitted ✅ 2026-05-01

## v1.2 — Audit closure ✅ 2026-05-07

- [x] 9 missing core files created (README/TODO/PARAMETERS/MAP/MEMORY/LINKS/KNOWLEDGE/UPGRADE/THEORY) ✅ 2026-05-07
- [x] Books-landing on server initialised + pushed to GitHub `books-landing-private` ✅ 2026-05-07

## v1.3 — Books Ze Theory launch (P0, target Q3 2026)

- [ ] **Step 4:** D2D upload Ze Theory v10 (cover + epub + metadata) → **2026-05-31**
- [ ] **Step 1:** Amazon KDP upload (W-8BEN, ASIN, $4.99 launch price) → **2026-06-15**
- [ ] **Step 2:** drjaba.com Stripe products live (использовать space.drjaba.com Stripe) → **2026-06-30**
- [ ] **Step 3:** Gumroad + Payhip live (mirror Stripe pricing) → **2026-07-15**
- [ ] Cross-channel announcement: 1 JabaEkimi video + community post → **2026-07-22**

## v1.4 — JabaEkimi monetisation (P1, target Q3 2026)

- [ ] YPP application submit (AdSense linked, tax form done) → **2026-05-20**
- [ ] First AdSense payout configured → ~30 days post-approval
- [ ] Sponsor outreach wave 1 (top-5 из `JabaEkimi/SPONSORS.md`) → **2026-06-01**
- [ ] First sponsor deal закрыт → **2026-08-01**

## v1.5 — Cross-channel infrastructure (P2)

- [ ] Lead magnet PDF "30-biomarker checklist" final draft → **2026-05-25**
- [ ] Lead magnet drjaba.com landing + ConvertKit/Mailchimp wire → **2026-06-01**
- [ ] Email autoresponder sequence (`Books/autoresponder/`) активирован → **2026-06-01**
- [ ] KPI dashboard (umbrella) → **2026-06-15**

## v1.6 — Structural cleanup (P3)

- [ ] Books/CONCEPT.md — fix Space/ упоминание (перенесён на сервер) → **2026-05-15**
- [ ] JabaEkimi/*.docx — удалить устаревшие дубликаты `.md` → **2026-05-15**
- [ ] Books/Integrative/Diets/Kartvely/24/ — статус каждой книги в Books/MEMORY.md → **2026-05-20**

## v2.0 — Beyond launch (Q4 2026+)

- [ ] Ze Theory v2 (revised + new chapters) — после feedback от первой когорты readers
- [ ] Second book (TBD: kvavili / abastumani / integrative) → D2D upload
- [ ] Course platform (если cum revenue ≥ $500/mo)
- [ ] Sponsor LOIs systematised (после 1-2 закрытых сделок)
- [ ] Paid ads experiment ($100 test budget) — only после YPP approval

```
### `Books/UPGRADE.md` (695 chars)
```md
# UPGRADE — Books

## Space (Место Силы)

- [ ] Завершить описания упражнений: RU → EN → KA
- [ ] Нумерологический раздел: черновик → peer review
- [ ] Синопсис для издателя (RU + EN)
- [ ] Финальный docx

## 24 (Роман)

- [ ] CONCEPT v3.0 → утвердить структуру глав
- [ ] Главы 1–3 (черновик)
- [ ] Peer review первых глав

## Kartvely 🟡 Активен

- [ ] актуализировать источники
- [ ] Обзор текущих рукописей (EN/KA/RU)

## Diets 🟡 Активен

- [ ] обновить v5 по новым данным (2024–2026)
- [ ] Перевод EN финализировать
- [x] PMID audit 2026-04-21 (docs/PMID_audit_2026-04-21)

## Integrative ✅ Closed 2026-04-19 (archived)

- Moved to `~/Documents/Archive/Books/Integrative_closed_2026-04-19/`

```
### `TODO.md` (2503 chars)
```md
# Marketing TODO — umbrella

Только umbrella-уровень. Внутри `JabaEkimi/` и `Books/` свои TODO.

## P0 — Books: Ze Theory 4-step launch (target Q3 2026)

См. `Books/Ze_Theory_Launch.md` (source of truth). Для каждого шага нужна целевая дата.

- [x] Step 4 — D2D account + W-8BEN + imprint "Longevity Horizon Press" ✅ 2026-05-01
- [ ] **Step 4 — D2D upload** Ze Theory v10 (free ISBN per book) → target **2026-05-31**
- [ ] **Step 1 — Amazon KDP** (Kindle ASIN, no ISBN) → target **2026-06-15**
- [ ] **Step 2 — drjaba.com Stripe direct** (использовать уже-wired space.drjaba.com Stripe) → target **2026-06-30**
- [ ] **Step 3 — Gumroad + Payhip** → target **2026-07-15**
- [ ] Cross-link launch announcement в JabaEkimi (1 video + community post) → 1 неделя после Step 1

## P1 — JabaEkimi: YPP application

См. `JabaEkimi/MONETIZATION_PLAN.md`.

- [x] Threshold reached: 2,270 subs · 4,100 watch hours ✅ 2026-04-30
- [ ] **YPP application submit** (требует AdSense + tax form) → target **2026-05-20**
- [ ] First payout configured → 1 месяц после approval
- [ ] Sponsor outreach wave 1 (top-5 из `SPONSORS.md` 20-target list) → target **2026-06-01**

## P2 — Cross-channel infrastructure

- [ ] **KPI tracker** (umbrella-level): velocity Books × CTR JabaEkimi × Stripe checkout → создать как `KPI.md` или dashboard. Target 2026-06-15.
- [ ] **Lead magnet PDF "30-biomarker checklist"** (`JabaEkimi/LEAD_MAGNET.md`) — final draft + drjaba.com landing → target **2026-05-25**
- [ ] **Email autoresponder** (`Books/autoresponder/` существует — проверить и активировать) → target **2026-06-01**
- [ ] **Sponsors caталог**: расширить `JabaEkimi/SPONSORS.md` для cross-promotion в Books press releases — после Step 1

## P3 — Структурная очистка

- [ ] **Books/CONCEPT.md** — упоминает `Space/` как 🟡 active; локальной папки нет (перенесена на сервер `space.drjaba.com`). Привести CONCEPT в соответствие.
- [ ] **JabaEkimi/** — повторяющиеся `.docx` рядом с `.md` (ACTIONS, CONCEPT, CONTENT_STRATEGY, KPI, LEAD_MAGNET, MONETIZATION_PLAN, SPONSORS) — устарели? решить, удалить.
- [ ] **Books/Integrative/Diets/Kartvely/** — отдельные книги с собственными CONCEPT.md; статус каждой (rough draft / complete / on-hold) свести в `Books/MEMORY.md`.

## Backlog

- Future PDF lead magnets (по темам: longevity, kvavili, abastumani, integrative)
- Email funnels (отдельные tracks для подписчиков из YouTube vs Books vs sites)
- Course platform (когда Books приносят >$500/mo)
- Sponsorship LOIs (после YPP approval)

```
### `Books/TODO.md` (1051 chars)
```md
# Books — TODO

_Обновлено: 2026-04-16 — все книги переведены в активный статус_

---

## 🔴 Space (Место Силы)

- [ ] Завершить описания упражнений на 4 языках (RU/EN/KA/ES)
- [ ] Нумерологический раздел — структура и написание
- [ ] Интеграция с AIM: упражнения → назначения

---

## 🟡 24 (Роман «24 Жизни»)

- [x] CONCEPT v3.0 → структура глав (24 главы = 24 жизни)
- [x] 24 главы × 3 языка (RU/EN/KA) написаны — см. `Books/24/Book/`
- [ ] Peer review / редакторский проход
- [ ] План публикации (издатель / self-publishing)

---

## 🟡 Kartvely (История Грузии)

- [ ] Обзор текущего состояния рукописей (EN/KA/RU финальные .docx)
- [ ] Определить следующий раздел для дописывания
- [ ] Список источников, требующих проверки

---

## 🟡 Diets (Диетология)

- [ ] Обзор Diets_v5 (EN/KA/RU) — что осталось дописать
- [ ] Актуализировать раздел по интегративной нутрициологии
- [ ] Согласовать с протоколами Regenesis

---

## ✅ Integrative (Медицина Поколений) — Closed 2026-04-19

- Moved to `~/Documents/Archive/Books/Integrative_closed_2026-04-19/`

```
### `KNOWLEDGE.md` (4764 chars)
```md
# KNOWLEDGE.md — Marketing umbrella

Domain knowledge: правила платформ, royalty rates, gotchas. Subjects к проверке/верификации помечены ⚠.

---

## 1. Amazon KDP (Step 1)

- **Royalty:** 70 % @ price $2.99–9.99 (USD) если book ≥ $0.06/MB delivery; иначе 35 %.
- **No ISBN required** для Kindle: Amazon выдаёт ASIN автоматически.
- **70 % royalty территории:** US, UK, EU, JP, BR, IN, CA, MX, AU. Остальные = 35 %.
- **KDP Select (90-day exclusivity)** даёт +5 free promo days, Kindle Unlimited revenue, Countdown Deals — но **запрещает** sales на Stripe/Gumroad/Payhip/D2D в эти 90 дней. **Не использовать** для Step-1 launch если планируется Step 2-4.
- **Tax:** W-8BEN отдельно для KDP (не наследуется от D2D).
- **Pricing trick:** $4.99 даёт лучший royalty/margin balance чем $2.99 для большинства nichе longevity.

## 2. Draft2Digital (Step 4)

- **Royalty:** 60–70 % author share (varies per retailer); D2D берёт 10 % spread.
- **Free ISBN per book** ✅ (D2D выдаёт, имя имprint = "Longevity Horizon Press").
- **Distribution:** Apple Books, Kobo, Barnes & Noble, Tolino, Borrow Box, Hoopla, OverDrive, Bibliotheca, Vivlio.
- **W-8BEN submitted ✅ 2026-05-01** (PayPal djabbat@gmail.com).
- **Imprint name приклеивается к ISBN** — нельзя менять после publish. Проверить написание перед upload.
- ⚠ D2D **не публикует на Amazon** (Amazon = Step 1, отдельно).

## 3. Stripe direct via drjaba.com (Step 2)

- **Margin:** ~97 % (Stripe Standard 2.9 % + 30¢; для $4.99 это ~3 %).
- **Уже wired** на `space.drjaba.com` — Books Step 2 = добавить products в существующий Stripe account, не новая интеграция.
- **EU VAT:** Stripe Tax handles (отслеживает EU customer location, добавляет VAT, payouts автоматом). Включить.
- **Refund policy:** digital goods обычно non-refundable, но EU consumer law требует 14-day cooling-off. Указать в checkout.

## 4. Gumroad (Step 3a)

- **Royalty:** 91 % (9 % flat fee, без processing).
- Нет paywall между customer и seller email; идеален для **build-an-email-list** strategy.
- Поддерживает PWYW (pay-what-you-want) — полезно для launch.

## 5. Payhip (Step 3b)

- **Royalty:** 95 % (5 % flat fee на free plan); $29/mo plan = 0 % fee.
- Нативная интеграция с Mailchimp/ConvertKit.
- VAT MOSS встроен.

## 6. YouTube — YPP

- **Threshold:** 1,000 subscribers + 4,000 watch hours за 12 мес ✅ (или 10M Shorts views за 90 дней — alternative path).
- **Application:** YouTube Studio → Monetization → Sign agreement → Link AdSense → Tax form.
- **Tax form (для не-US):** W-8BEN, заявляешь tax treaty (Грузия имеет 0 % WH on royalties через US-GE treaty 1973? ⚠ verify).
- **Strikes/violations:** Community Guidelines / Copyright — могут заблокировать YPP. Контент на грузинском с медицинской тематикой — повышенный риск Health Misinfo policy. Каждое видео должно ссылаться на peer-reviewed.
- **Payment threshold:** $100 минимум для payout (через 1–6 месяцев в зависимости от региона).

## 7. Sponsorships

- **Industry standard rate (longevity nicheproject):** $20–50 CPM (cost per 1k views) для mid-tier; $5–15 для starter (<10k subs).
- **Format:** integrated 30–60s mention в начале или середине видео; не в самом конце (drop-off).
- **Disclosure:** YouTube требует Paid Promotion checkbox + on-screen disclaimer.
- **Sponsor categories** (см. `JabaEkimi/SPONSORS.md`): supplements (NMN/spermidine/rapa), DNA/biomarker testing, longevity clinics, devices (CGM/wearables), software (longevity apps).
- **Грузинский рынок узок** → primary sponsors будут англоязычные с глобальным reach.

## 8. Books — pricing psychology

- **$4.99 = sweet spot** для non-fiction debut (Amazon top-seller analysis 2024).
- **Free promo (KDP)** — повышает review velocity, но снижает perceived value. Использовать однократно для launch (1–3 days).
- **Bundle discount** (Stripe direct) — Ze Theory + future title at $7.99 vs $5+5 = $10. Plays well для loyal funnel customers.

## 9. Lead magnet → email funnel

- **30-biomarker checklist PDF** (проект `JabaEkimi/LEAD_MAGNET.md`) = primary funnel asset.
- Hosted на drjaba.com landing → ConvertKit/Mailchimp (TBD).
- **Email sequence** (рекомендуется): D0 PDF → D2 longevity science briefing → D5 introduce JabaEkimi channel → D9 introduce Books → D14 consultation offer.

## 10. Локальный рынок Грузии

- **Грузинский e-book market** ≈ 0 (нет инфраструктуры). Все Books продажи — экспорт.
- **YouTube — двойной аудитори:**: грузинская диаспора (CIS + EU) + ru-speakers общая healthtech. Не оптимизировать только под GE-locale.

---

## 11. Sources to verify ⚠

- US-Georgia tax treaty 1973 — withholding rate на royalties (1 % vs 30 %)
- Georgia VAT export rules (digital goods to non-residents)
- KDP 70 % territory list (актуальная 2026)
- Apple Books category-specific guidelines (для Ze Theory)

```
### `Books/KNOWLEDGE.md` (760 chars)
```md
# KNOWLEDGE — Books

## Книгоиздание

- Стандартный объём нон-фикшн: 50–80k слов (~200–300 стр.)
- Художественный роман: 80–120k слов
- Синопсис для агента: 1–2 стр. + первые 3 главы
- Литагент нужен для крупных издательств EN; для RU можно напрямую

## Workflow написания (проверенный)

1. CONCEPT → структура глав
2. Каждая глава → черновик через DeepSeek
3. Peer review (Claude) → правки
4. Финальная полировка → DeepSeek
5. md → docx через `md_to_docx.py`

## Space — накопленные знания

- Нумерология: система основана на авторской интерпретации
- Упражнения: нужны на RU + EN + KA минимум

## Diets — накопленные знания

- Последняя версия: v5 (RU + EN + KA)
- Основа: синтез диет Певзнера с современными данными
- Готова к публикации при размораживании

```
### `MEMORY.md` (2645 chars)
```md
# MEMORY.md — Marketing umbrella

Что Клоду нужно помнить про umbrella между сессиями. Новое сверху.

---

## 2026-05-07 — Audit + 9 missing core files restored

В eco-system audit обнаружено: Marketing/ имел только CONCEPT + CLAUDE из 11-file core. Восстановлены 9 файлов (README/TODO/PARAMETERS/MAP/MEMORY/LINKS/KNOWLEDGE/UPGRADE/THEORY) на umbrella уровне. Subprojects (`JabaEkimi/`, `Books/`) свою документацию имеют сами.

**Ещё открытые вопросы (P3 в TODO):**
- Books/CONCEPT.md упоминает Space/ как активный — папки нет (перенесена на server `space.drjaba.com`)
- JabaEkimi/*.docx дубликаты `.md` — устарели?
- Books/Integrative/Diets/Kartvely/24/ — статус каждой книги нужен

## 2026-05-01 — D2D activation

Draft2Digital account active ($20 setup fee). Imprint = "Longevity Horizon Press". PayPal djabbat@gmail.com. W-8BEN submitted. Готов к первому upload — Ze Theory v10. Это **Step 4** of 4-step launch.

См. memory `project_d2d_publishing` + `Books/Ze_Theory_Launch.md`.

## 2026-04-30 — Marketing umbrella formed

Папка переименована из `DrJaba/` → `Marketing/` чтобы избежать путаницы с server-side `djabbat/DrJaba` (drjaba.com clinic site). Создана с двумя субпроектами:
- `JabaEkimi/` — YouTube channel (был внутри DrJaba/ раньше)
- `Books/` — Ze Theory commercial launch (был внутри DrJaba/ раньше)

**Не путать:** `~/Desktop/Marketing/` (umbrella, funnel) ≠ server `djabbat/DrJaba` (site). См. memory `project_marketing`.

## 2026-04-30 — JabaEkimi YPP eligibility

Канал `@ჯაბაექიმი` достиг 2,270 subs + 4,100 watch hours → YPP-eligible (1k+4k threshold met). Application not yet filed — заблокировано подготовкой AdSense + tax form.

См. `JabaEkimi/MONETIZATION_PLAN.md`.

## 2026-04-29 — NPLG denied e-book ISBN → no-ISBN strategy

NPLG (Грузинская национальная библиотека) отказалась выдавать ISBN для e-book. Переход на NO-ISBN distribution: KDP (ASIN), drjaba.com Stripe (no ID), Gumroad/Payhip (no ID), D2D (free ISBN per book).

Это **canonical** — не пытаться снова за ISBN если пользователь явно не попросит.

См. memory `project_ze_book_launch`.

---

## Постоянные правила

- **Funnel endpoint** — drjaba.com Stripe, не отдельная интеграция per channel
- **Tkemaladze personal entity** — все commercial revenue, НЕ NGO accounting
- **No paid ads до YPP approval** — органика only
- **Lead magnet PDF** — общий для JabaEkimi/Books/site, не дублировать
- **Cross-promotion**: каждый Book launch = 1 video в JabaEkimi + community post; каждый sponsor deal в JabaEkimi → mention в Books press release (если уместно)
- `feedback_unpublished` — Ze Theory v10 = NOT YET PUBLISHED commercially (manuscript есть, launch ⏳)

```
### `Books/MEMORY.md` (327 chars)
```md
# MEMORY — Books

## 2026-04-15

- Создан монорепозиторий Books
- Space и 24 перенесены из ~/Desktop/ в Books/
- Kartvely, Diets, Integrative перенесены из Archive/Deferred/Books/
- Space имел собственный git (djabbat/Space) — .git удалён, включён в монорепо
- Решение: единый приватный репо `djabbat/Books`, без public-версии

```
### `LINKS.md` (2273 chars)
```md
# LINKS.md — Marketing umbrella

Только публичные URL. Личные emails / API-ключи / Stripe secrets — НЕ здесь.

---

## Funnel endpoints

| Resource | URL |
|---|---|
| drjaba.com (clinic site, funnel destination) | https://drjaba.com |
| books.drjaba.com (Books static landing) | https://books.drjaba.com |
| space.drjaba.com (knowledge graph + Stripe) | https://space.drjaba.com |
| ksystem.drjaba.com | https://ksystem.drjaba.com |
| spellcheckerka.drjaba.com | https://spellcheckerka.drjaba.com |

## JabaEkimi

| Resource | URL |
|---|---|
| YouTube channel | https://www.youtube.com/@ჯაბაექიმი |
| YouTube Studio | https://studio.youtube.com |
| YPP guidelines | https://support.google.com/youtube/answer/72851 |
| AdSense | https://adsense.google.com |

## Books — distribution channels

| Channel | Login URL | Public store URL |
|---|---|---|
| Amazon KDP | https://kdp.amazon.com | https://amazon.com (after Step 1) |
| Draft2Digital | https://draft2digital.com | varies (Apple/Kobo/B&N via aggregator) |
| Gumroad | https://gumroad.com | (после Step 3) |
| Payhip | https://payhip.com | (после Step 3) |
| Stripe (через drjaba.com) | https://dashboard.stripe.com | https://drjaba.com/checkout (after Step 2 wire-up) |

## Tax / compliance

| Resource | URL |
|---|---|
| W-8BEN form (IRS) | https://www.irs.gov/forms-pubs/about-form-w-8-ben |
| Georgia revenue (RS) | https://rs.ge |

## Tools

| Resource | URL |
|---|---|
| Canva (covers) | https://canva.com |
| ConvertKit/Mailchimp (autoresponder) | https://convertkit.com / https://mailchimp.com |
| Google Trends (content research) | https://trends.google.com |
| TubeBuddy / VidIQ (channel growth) | https://tubebuddy.com / https://vidiq.com |

## Cross-project (own ecosystem)

| Resource | Path / URL |
|---|---|
| GLA (academic publishing, separate funnel) | `~/Desktop/GLA/` |
| Annals of Rejuvenation Science | https://longevity.ge/ars/ |
| Longevity Horizon journal | https://longevity.ge/longhoriz/ |
| Ze project (source for Ze Theory) | `~/Desktop/LongevityCommon/Ze/` |
| AIM (potential consult endpoint) | https://aim.longevity.ge |

## Git

- Repo: `git@github.com:djabbat/Marketing-private.git` (private)
- Books-landing on server: `git@github.com:djabbat/books-landing-private.git` (private)

```
### `Books/LINKS.md` (519 chars)
```md
# LINKS — Books

## Издатели (RU)

- Альпина Паблишер — https://www.alpina.ru/authors/
- МИФ (Манн, Иванов и Фербер) — https://www.mann-ivanov-ferber.ru/
- Эксмо — https://eksmo.ru/authors/

## Издатели (EN)

- Hay House — https://www.hayhouse.com/submissions
- Penguin Random House — https://www.penguinrandomhouse.com/

## Самиздат / гибрид

- Ridero (RU) — https://ridero.ru/
- Amazon KDP — https://kdp.amazon.com/

## Референсы — Space

- (добавлять по мере работы)

## Референсы — 24

- (добавлять по мере работы)

```
## Code volume
| ext | files | bytes |
|---|---|---|
| .py | 1 | 4537 |