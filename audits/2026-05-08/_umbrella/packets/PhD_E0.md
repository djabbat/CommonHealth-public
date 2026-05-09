# AUDIT PACKET — PhD_E0

Path: `/home/oem/Desktop/PhD/E0`  Date: 2026-05-08

## Size & file counts
```
14M	/home/oem/Desktop/PhD/E0
```
**Extensions:** .md=39, .jpg=35, .pdf=4, (noext)=1, .mp4=1
## Tree (depth=2, max 200 entries)
```
.
./PARAMETERS.md
./photos
./photos/photo_2026-04-23_14-41-55.jpg
./photos/photo_2026-04-23_13-51-11.jpg
./photos/video.mp4
./photos/photo_2026-04-23_21-48-00.jpg
./photos/photo_2026-04-23_14-18-01.jpg
./photos/photo_2026-04-23_21-19-46.jpg
./photos/photo_2026-04-23_18-05-25.jpg
./photos/photo_2026-04-23_13-51-15.jpg
./photos/photo_2026-04-23_21-47-49.jpg
./photos/photo_2026-04-23_21-47-34.jpg
./photos/photo_2026-04-23_17-42-20.jpg
./photos/photo_2026-04-23_13-51-21.jpg
./photos/photo_2026-04-23_21-48-07.jpg
./photos/photo_2026-04-23_18-05-20.jpg
./photos/photo_2026-04-23_21-47-43.jpg
./photos/photo_2026-04-23_13-51-42.jpg
./photos/photo_2026-04-23_21-48-16.jpg
./photos/photo_2026-04-23_21-47-55.jpg
./photos/photo_2026-04-23_14-18-10.jpg
./photos/photo_2026-04-23_13-51-38.jpg
./photos/photo_2026-04-24_01-42-16.jpg
./photos/photo_2026-04-23_13-51-34.jpg
./photos/photo_2026-04-23_21-19-52.jpg
./photos/photo_2026-04-23_13-50-56.jpg
./photos/photo_2026-04-23_14-18-15.jpg
./photos/photo_2026-04-23_13-51-30.jpg
./photos/photo_2026-04-23_13-50-59.jpg
./photos/photo_2026-04-23_21-48-22.jpg
./photos/photo_2026-04-23_17-42-16.jpg
./photos/photo_2026-04-23_13-51-26.jpg
./photos/photo_2026-04-23_13-48-37.jpg
./photos/photo_2026-04-23_13-50-52.jpg
./photos/photo_2026-04-23_17-42-24.jpg
./photos/INDEX.md
./photos/photo_2026-04-23_13-51-05.jpg
./photos/photo_2026-04-23_21-48-11.jpg
./photos/photo_2026-04-23_21-47-39.jpg
./README.md
./UPGRADE.md
./TODO.md
./MEMORY.md
./CLAUDE.md
./LINKS.md
./docs
./docs/ENCLOSURE.md
./docs/Покупки_Китай.md
./docs/Чистка_микроскопа_IM35.md
./docs/PEER_REVIEW_v4_Tsomaia_2026-04-27.md
./docs/Adapter_для_токаря.md
./docs/COMMIT_LOG_2026-04-26.md
./docs/PEER_REVIEW_v2_Funds_2026-04-26.md
./docs/Полное_Описание.md
./docs/PEER_REVIEW_v3_Tsomaia_KA_2026-04-27.md
./docs/PEER_REVIEW_v3_Tsomaia_2026-04-27.md
./docs/01_CONTROL_ARCHITECTURE.md
./docs/CLAUDE_AGENT.md
./docs/For_brother_in_China.md
./docs/Техническая_реализация.md
./docs/Для_брата_в_Китае.pdf
./docs/Чистка_микроскопа_IM35.pdf
./docs/SCHEMAS.md
./docs/UNIVERSAL_RIG_2026-04-26.md
./docs/inverted-im35-icm405.pdf
./docs/REFERENCE_AUDIT_2026-04-26.md
./docs/PEER_REVIEW_v2_2026-04-26.md
./docs/02_HARDWARE_INVENTORY.md
./docs/BOM.md
./docs/Phase_0_Prototype.md
./docs/TSOMAIA_DESIGN_2026-04-27.md
./docs/For_brother_in_China.pdf
./docs/INDEX.md
./docs/Для_брата_в_Китае.md
./docs/PHASE1_ORDERING_LIST_DRAFT_2026-04-27.md
./docs/Ilie_Telegram_followup_2026-04-26.md
./docs/VIDEOS.md
./docs/00_EXECUTIVE_SUMMARY.md
./CONCEPT.md
./_archive
./_archive/PEER_REVIEW_DRAFT.md
./MAP.md
./KNOWLEDGE.md
./LICENSE
```
## Detected stack: **unknown**
## Core files

### `CLAUDE.md` (2574 chars)
```md
# CLAUDE.md — Experiment 0

## Контекст

Проект — HW+SW commissioning AI-directed laser ablation rig на Zeiss IM 35 + LGY40-C motorized stage + Claude Code /overnight agent. НЕ биологический пилот.

## Правила (специфичные для проекта)

1. **Всё через DeepSeek** где можно (per `~/CLAUDE.md`). Исключение: Rust/Python для локальной интеграции.
2. **Source of truth** — `CONCEPT.md` + `BOM.md` + `PARAMETERS.md`. При противоречии с legacy-файлами (Полное_Описание, Phase_0_Prototype и т.д.) — приоритет core-файлов.
3. **НЕ модифицировать Zeiss IM 35** физически. Все upgrade решения должны быть обратимы (stacking, не screwing).
4. **Safety first** — hardware interlock always. Software kill — вторично. Laser gate через SPDT relay + reed switch.
5. **Before commissioning** — фото port, измерить base, визуальная инвентаризация объективов.
6. **6-мес непрерывная работа** — все компоненты должны поддерживать 24/7 (Cree XHP50 underrun, UPS, self-restart).

## Startup Protocol

При каждом старте сессии в этом проекте:
1. Проверить что все компоненты закуплены (см. TODO P1 checklist)
2. Напомнить о pending peer-review issues (см. PEER_REVIEW_DRAFT.md + TODO Pending)
3. Спросить: работаем сегодня над hardware (сборка) или software (Arduino firmware / Python driver / Claude agent)?
4. Если laser integration — проверить safety checklist перед включением

## Связь с auto-memory

- `project_cdata_copi_candidates` — Impetus Phase A co-PI candidates
- `feedback_mcoa_cdata_comparison` — MCOA vs CDATA (связь с PhD)
- `pubmed_authoritative` — 10 PMIDs authoritative
- (PhD-supervisor memories удалены 2026-05-04 правилом feedback_no_supervisor_names)

## Что НЕ делать

- ❌ Не писать биологических claim'ов про Elodea как surrogate мammalian центриолей (peer review уже reject'ил)
- ❌ Не модифицировать Zeiss оригинальными screws / knobs / couplers
- ❌ Не пропускать dose matrix calibration (prerequisite для любой ablation)
- ❌ Не запускать 6-мес session без pre-registration протокола (OSF)
- ❌ Не обещать translational validity для Impetus без Experiment A

## Связь с другими проектами

- `PhD` — диссертация, не Experiment 0 scope
- `LongevityCommon/CDATA` — theoretical basis
- `LongevityCommon/AutomatedMicroscopy` — параллельный AI-microscopy проект
- `LongevityCommon/MCOA` — meta-framework, Counter integration
- Impetus LOI / EIC Pathfinder — grants, не Experiment 0 scope

## Git / sync

- `~/Desktop/E0/` — единственный источник правды (`djabbat/E0-private`, выделено 2026-04-26)
- Старые копии в `~/Desktop/PhD/microscope/Experiment_0/` удалены при выделении

```
### `README.md` (7250 chars)
```md
# Experiment 0 — HW+SW Commissioning

**Цель:** валидировать hardware + software stack для AI-directed laser ablation rig на Zeiss IM 35 / ICM 405. **НЕ биологический пилот.**

**Что валидируется:**
1. **Claude Code `/overnight` agent** — автономное ведение time-lapse 24/7
2. **Laser TTL control** — Arduino PWM от Python команды → ablation target
3. **Motorized stage** — AI командует перемещение к target, центровка в FOV
4. **Feedback loop** — детекция → решение → действие → логирование
5. **Error handling** — interlock, temp overrun, UPS event, camera crash, network loss
6. **Data pipeline** — 6-мес TIFF stream без data loss
7. **Safety infrastructure** — light-tight box, двухконтурный interlock, UPS

**Что НЕ валидируется (явно):**
- Центриолярная биология (признано — см. `PEER_REVIEW_DRAFT.md`)
- Translational claims к mammalian CDATA
- Impetus pilot positioning

**Модель:** *Elodea canadensis* leaves, chloroplasts. Выбор обоснован:
- Бесплатно / non-precious / без ethics issues
- Chloroplasts видимы в brightfield без окраски
- Discrete targets для AI detection
- Ablation viz: выжигание видно на месте
- Позволяет отладить pipeline на реальных биологических данных без биологических claims

**Deliverable:**
- Open-source GitHub repo: Arduino sketch + Python tool functions + Claude agent PROMPT.md
- Technical paper на arXiv (HW/SW engineering): "AI-driven autonomous single-organelle laser ablation platform for sub-$1000"
- Proven workflow для переноса на Experiment A (правильная биология, Impetus Phase A)

---

## Архитектура системы

```
┌─────────────────────────────────────────────────────────────┐
│  EXTERNAL                                                   │
│  ┌─────────┐  ┌────────────┐  ┌──────────────────┐         │
│  │ Monitor │  │ UPS APC    │  │ External HDD 4TB │         │
│  │  24"    │  │ SMT1500    │  │ (weekly rsync)    │         │
│  └────▲────┘  └──────▲─────┘  └────▲──────────────┘        │
│       │ HDMI         │ 220V         │ USB 3.0                │
│  ┌────┴──────────────┴──────────────┴────────────────────┐ │
│  │ PC (Linux Ubuntu 22.04)                                │ │
│  │ - Micro-Manager 2.0 + PyMMCore-Plus                    │ │
│  │ - Claude Code CLI with /overnight agent                │ │
│  │ - Python tool functions (ablate_target, move_stage...)│ │
│  │ - Arduino serial over USB                              │ │
│  └───────┬────────────────────────────────────────────────┘ │
└──────────┼──────────────────────────────────────────────────┘
           │ USB2/3 + GPIO
           │
┌──────────┼──────────────────────────────────────────────────┐
│  LIGHT-TIGHT ENCLOSURE (ACP 3mm black, 600×500×700 мм)      │
│  ┌───────┴──────────┐                                        │
│  │ Zeiss IM 35      │                                        │
│  │  ┌────────────┐  │  ┌───────────────────────────────┐    │
│  │  │ LED xform  │◄─┼──┤ Arduino Nano (PWM LED, laser)  │    │
│  │  │ Cree XHP50 │  │  │ + ESP8266 (WiFi alerts, MQTT)  │    │
│  │  └────────────┘  │  │ + DS18B20 × 3 (temp)           │    │
│  │  ┌────────────┐  │  │ + BPW34 (brightness)           │    │
│  │  │ Motorized  │◄─┼──┤ + Interlock D2 (reed switch)   │    │
│  │  │ XY stage   │  │  │ + Laser GATE (SPDT relay HW)   │    │
│  │  │ NEMA-17×2  │  │  └───────────────────────────────┘    │
│  │  └────────────┘  │                                        │
│  │  ┌────────────┐  │  ┌───────────────────────────────┐    │
│  │  │ ToupCam    │──┼──┤ USB 3.0 → PC                   │    │
│  │  │ E3CMOS     │  │  └───────────────────────────────┘    │
│  │  └────────────┘  │                                        │
│  │  ┌────────────┐  │  ┌───────────────────────────────┐    │
│  │  │ Laser 450  │◄─┼──┤ 12V PSU + TTL from Arduino    │    │
│  │  │ nm 500mW   │  │  │ (via SPDT relay hardware kill) │    │
│  │  └────────────┘  │  └───────────────────────────────┘    │
│  └──────────────────┘                                        │
│                                                             │
│  OVERVIEW CAMERAS:                                          │
│  ┌─────────────────┐  ┌──────────────────┐                  │
│  │ RPi Cam 3 Wide  │  │ USB endoscope    │                  │
│  │ NoIR + IR LEDs  │  │ 1080p 8mm macro  │                  │
│  │ (Pi Zero 2W)    │  │ (close-up stage) │                  │
│  └─────────────────┘  └──────────────────┘                  │
│                                                             │
│  VENT: 2× Z-baffle light-trap + Noctua 120мм                │
│  INTERLOCK: magnetic door switch → reed → relay → Arduino   │
└─────────────────────────────────────────────────────────────┘
```

---

## Файлы проекта

| Файл | Что |
|---|---|
| `README.md` | Этот файл — обзор + архитектура |
| `Полное_Описание.md` | ⭐ **МАСТЕР-ДОКУМЕНТ** — все детали (inventory, BOM, сборка, протокол, параметры) |
| `BOM.md` | Детальный Bill of Materials с ссылками |
| `ENCLOSURE.md` | CAD-схема светонепроницаемого бокса |
| `Покупки_Китай.md` | Приоритетный чеклист по неделям |
| `CLAUDE_AGENT.md` | ⭐ Tool functions + PROMPT.md template для Claude Code agent |
| `PEER_REVIEW_DRAFT.md` | Критический peer review от DeepSeek reviewer |

---

## Фазы (6 месяцев)

| Месяц | Задача |
|---|---|
| **1** | Закупка (3-4 нед shipping) + локально Тбилиси |
| **2** | Сборка бокса + LED retrofit + motorized stage DIY + Arduino sketch |
| **3** | Калибровка Köhler, stage precision, laser focus, interlock test |
| **4** | Claude Code agent integration test (dry-run на синтетических данных) |
| **5** | 6-нед continuous run с Claude /overnight контролем |
| **6** | Анализ, GitHub release, arXiv preprint, transition plan в Experiment A |

---

## Бюджет

**Sync'd с `PARAMETERS.md` (authoritative source) 2026-04-24:**

- **Минимум:** $881 (с LGY40-C + NEMA-8 мotorization + ToupCam + UPS Back-UPS 1500)
- **Оптимум:** $1687 (с premium LGY40-C linear stepper actuator + Smart-UPS SMT1500 + reserve камеры)

Полная смета — в `docs/BOM.md` (§1b updated для LGY40-C 2026-04-23, §1.5/1.6 halogen = OSRAM 64607 8V 50W).

> **Примечание:** предыдущие оценки $530-770-1400 считались на старый план (NEMA-17 + knob coupling) с частично отсутствующими позициями; не использовать.

---

## Транзит к Experiment A (Impetus Phase A, 2026 Q3+)

**Что переиспользуется из Experiment 0:**
- ✅ Бокс + interlock + вентиляция
- ✅ Overview cameras (RPi + endoscope)
- ✅ Arduino sketch (расширить для fluorescence PWM каналов)
- ✅ Claude Code agent + tool functions framework
- ✅ Motorized stage (precision ±20-50 μm достаточно для cell-level targeting)
- ✅ Zeiss microscope + objectives
- ✅ UPS + storage
- ✅ LED transmitted light (возможен upgrade на high-CRI)

**Что меняется:**
- ❌ Laser: 450 nm CW → pulsed ns UV 355 nm (Cobolt Tor или Rapp UGA-42, $15-25K)
- ❌ Biological model: Elodea → iPSC-organoids с Centriolin-RITE или Drosophila GSC
- ❌ HBO replacement: LED epi-source для fluorescence
- ❌ Objective: upgrade to Plan-Apo 100×/1.4 oil
- ❌ Environmental: 37°C + CO₂ chamber для mammalian
- ❌ Compliance: BSL-2 collaboration с Georgian biomedical institute

**Realistic reuse: ~50-60%** hardware, ~80% software stack.

---

*README создан 2026-04-23. Reframe после peer review.*

```
### `CONCEPT.md` (2721 chars)
```md
# CONCEPT.md — Experiment 0

**Версия:** 1.0
**Дата:** 2026-04-23
**Статус:** HW+SW commissioning, в процессе закупки компонентов

## Цель

Отладить **Claude Code /overnight agent** управление прецизионным инвертированным микроскопом (Zeiss IM 35/ICM 405) — автономную 24/7 laser-ablation + imaging + tracking систему — на Elodea canadensis chloroplasts как testbed.

**НЕ биологический пилот.** Commissioning only.

## Что валидируется

1. AI-agent layer (Claude Code + DeepSeek router) для научных decision-making в real-time
2. Python tool function API (move_stage, fire_laser, capture_image, detect_targets, ...)
3. Arduino Nano firmware как realtime layer (PWM, stepper, interlock, sensors)
4. LGY40-C motorized XY stage (stacking поверх винтажного Zeiss, никакая модификация оригинала)
5. 6-месячная stability rig + agent + data pipeline
6. Safety infrastructure (interlock, light-tight enclosure, OD 4+ goggles, UPS)

## Что НЕ валидируется

- Центриолярная биология
- Translational claims
- Impetus pilot positioning (биология)
- Elodea как "mammalian surrogate" (см. `PEER_REVIEW_DRAFT.md` — Elodea chloroplasts стохастические, центриоли детерминированные)

## Связь с экосистемой

- **PhD** (`~/Desktop/PhD/`) — диссертация CDATA. Experiment 0 — не часть диссертации (dissertation = critical commentary по 10 PMID, не экспериментальная).
- **LongevityCommon/CDATA** (`~/Desktop/LongevityCommon/CDATA/`) — научная основа. Experiment 0 — отладка rig перед Experiment A (настоящий биологический пилот).
- **LongevityCommon/AutomatedMicroscopy** — параллельный проект про AI-microscopy. Experiment 0 — конкретный commissioning этого направления.
- **LongevityCommon/MCOA** — meta-framework. Experiment 0 — отладка агента для будущих MCOA-экспериментов.

## Переход в Experiment A

После успешного 6-мес commissioning:
- Rig готов к iPSC-organoids + Centriolin-RITE (Royall 2023 eLife PMID 37882444)
- ИЛИ Drosophila GSC Centrobin-GFP (Yamashita 2003 Science)
- Budget Experiment A: $80k Phase A (Impetus LOI 2026-04-25) + $120k Phase B (Geiger Ulm)

## Ключевые риски (см. `PEER_REVIEW_DRAFT.md`)

1. **Biological surrogate gap** — Elodea ≠ mammalian centrioles. Для Impetus не подходит как pilot data.
2. **Laser type** — 450 nm CW, не Q-switched ns UV (нужен для single-organelle ablation без phototoxicity)
3. **Optics UV coating** — Zeiss IM 35 объективы пропускают <30% at 450 nm
4. **Statistics** — нет power calc, pre-registration, blinding
5. **Vibration** — квартирный стол, нет оптического стола

## Source of truth

- `CONCEPT.md` (этот файл) — общая концепция
- `BOM.md` — авторитетный Bill of Materials
- `Полное_Описание.md` — расширенное reference (1000 строк)
- `PEER_REVIEW_DRAFT.md` — самокритика

```
### `MAP.md` (2909 chars)
```md
# MAP.md — Experiment 0

**Версия:** 1.0
**Дата:** 2026-04-23
**Назначение:** Файловая структура проекта + связь с экосистемой. Источник истины — `CONCEPT.md`.

## Файловая структура

```
~/Desktop/E0/
├── INDEX.md                         — навигация для внешнего рецензента
├── CONCEPT.md                       — концепция, цели, границы
├── README.md                        — project overview (legacy)
├── CLAUDE.md                        — правила работы Claude Code в проекте
├── MAP.md                           — этот файл
├── MEMORY.md                        — ссылки на auto-memory записи
├── KNOWLEDGE.md                     — подтверждённые факты, verified references
├── PARAMETERS.md                    — численные параметры (optics, laser, camera)
├── TODO.md                          — задачи
├── UPGRADE.md                       — roadmap улучшений Phase 0 → Phase A
├── LINKS.md                         — внешние ссылки (AliExpress, eBay, PMIDs)
│
├── 00_EXECUTIVE_SUMMARY.md          — 1-page pitch для specialist
├── 01_CONTROL_ARCHITECTURE.md       — Claude + Arduino + Python stack
├── 02_HARDWARE_INVENTORY.md         — inventory + фото-ссылки
│
├── BOM.md                           — Bill of Materials (AUTHORITATIVE)
├── ENCLOSURE.md                     — light-tight box CAD
├── CLAUDE_AGENT.md                  — Claude agent implementation detail
├── Phase_0_Prototype.md             — cheaper prototype alternative
├── Полное_Описание.md               — full 1000-line reference
├── Техническая_реализация.md        — deep engineering
├── Покупки_Китай.md                 — Taobao shopping через брата
├── Adapter_для_токаря.md            — LED mount adapter чертёж
├── PEER_REVIEW_DRAFT.md             — самокритика / known limitations
│
└── photos/                          — фото-инвентарь + video
    ├── INDEX.md                     — описание каждого фото
    ├── photo_2026-04-23_*.jpg       — 35 фотографий
    └── video.mp4                    — обзорное видео
```

## Связь с экосистемой (другие репо/директории)

| Внешний ресурс | Связь |
|---|---|
| `~/Desktop/LongevityCommon/CDATA/` | Научная основа (CDATA theory) |
| `~/Desktop/LongevityCommon/AutomatedMicroscopy/` | Параллельный project про AI microscopy |
| `~/Desktop/LongevityCommon/MCOA/` | Meta-framework, Counter frameworks |
| `~/Desktop/AIM/llm.py` | LLM router (DeepSeek primary) для Claude agent |
| `~/.aim_env` | DEEPSEEK_API_KEY |
| `~/Desktop/PhD/docs/` | Gitignored переписка, не для этого проекта |

## Что НЕ в E0 (external dependencies)

- **Impetus LOI** → `~/Documents/Submissions/2026-04-25_Impetus_CDATA/` (NOT here, отдельный grant submission)
- **MCOA submission** → `~/Documents/Submissions/2026-04-25_NatureAging_MCOA/`
- **PhD dissertation** → `~/Desktop/PhD/dissertation/`

## Git tracking

`~/Desktop/E0/` → `djabbat/E0-private` (выделено из PhD/microscope/Experiment_0 — 2026-04-26).

```
### `PARAMETERS.md` (7841 chars)
```md
# PARAMETERS.md — Experiment 0

**Версия:** 1.0
**Дата:** 2026-04-23
**Назначение:** Численные параметры — optics, laser, camera, control, timing. Авторитетный источник для BOM + control software.

## Микроскоп

| Параметр | Значение |
|---|---|
| Model | Zeiss IM 35 / ICM 405 (inverted, West Germany) |
| Год выпуска | ~1978-1989 (Opton era) |
| Объектив primary (Phase 0) | 40× / 0.65 (Plan) |
| Photo port projection | 3.2× (встроенный, OPTON 47 17 73-9901) |
| Effective magnification на sensor | 128× (объектив × 3.2) |
| Filter cube slider | OPTON 46 52 24-01 (2 позиции: DAPI + FITC/GFP) |

## Sampling / resolution (при 40× objective)

| Параметр | Значение |
|---|---|
| Sensor pixel size (ToupCam IMX264) | 3.45 µm |
| Image pixel в sample plane | 27 nm (oversampled 15×) |
| FOV at 40× on 5MP sensor | ~42 × 27 µm |
| Diffraction limit (Abbe, 550 nm, NA=0.65) | 423 nm |
| Рекомендация: 0.5× reducer | → 54 nm/pixel, 8× oversample, FOV удваивается |

## Освещение — transmitted light

| Параметр | Значение |
|---|---|
| **Zeiss original spec (per IM 35 PDF)** | 12V 60W halogen, Zeiss p/n **3800 18-2520**, lamp socket 60/I p/n 468015, housing p/n 467257 |
| **Spare box у PI** | OSRAM 64607 EFM HLX BELLAPHOT, 8V 50W, GZ6.35 MR16 (другое напряжение — нужно сверить с трансформатором) |
| Base (оба типа) | **GZ6.35** (bi-pin, 6.35 мм spacing, MR16-класс) |
| ⚠️ TODO | замерить мультиметром **напряжение трансформатора** до вставления лампы. 12V → нужна Zeiss 12V 60W. 8V → OSRAM 64607 подходит. |
| Ресурс halogen | 50-200 ч (недостаточно для 6 мес = 4,320 ч непрерывно) |
| **Upgrade: LED retrofit** | Cree XHP50.2 J4 6500K underrun @ 5W — **с pulse-on-capture режимом** (см. ниже) |
| LED driver | Meanwell LDD-700H (CC 700mA, PWM gate input) |
| LED mount | Custom 3D-printed под GZ6.35 socket, height = высота оригинала (сохранить Köhler plane) |

### LED operation mode — **pulse-on-capture** (НЕ continuous 6 мес!)

| Параметр | Значение |
|---|---|
| **Mode** | Pulse/cycle — включается ТОЛЬКО на время capture |
| Warmup before trigger | 50 ms (settle LED driver + stable output) |
| Exposure window | 100 ms (brightfield Elodea) |
| Per-capture ON time | **150 ms** (50 warmup + 100 exposure) |
| Between captures | LED **OFF** (Arduino PWM pin = LOW) |
| Time-lapse interval | 30 min |
| Captures per timepoint | 2 (pre + post ablation) |
| **Duty cycle** | 300 ms / 30 min = **0.017%** (~1 part in 6,000) |
| **Total ON за 6 мес** | ~30 минут (vs 4,320 часов если 24/7) |
| Photodose на sample | **минимален** — сохраняет хлоропласты, избегает phototoxicity |
| Thermal drift | <0.5°C (vs +3-5°C при 24/7) |
| LED degradation за 6 мес | **<0.01%** (vs 3% при 24/7) |
| UPS runtime в standby (5W→2W idle) | **20+ часов** (vs 2-3 часа при 24/7) |

**Why pulse mode:**
1. 6 месяцев непрерывного 5W света @ 6500K = **phototoxicity disaster** для Elodea chloroplasts (выгорят за 1-2 дня, клетки умрут за неделю)
2. Background fluorescence поднимается 10-100× при continuous → SNR рушится
3. Thermal stability образца
4. Default-safe state (pin LOW = OFF) при Claude crash

**Arduino firmware pattern:**
```cpp
void capture_with_led(uint16_t exposure_ms) {
    analogWrite(LED_PWM_PIN, led_target_pwm);  // ON, start warmup
    delay(50);                                  // LED settles
    camera.trigger_start();                     // begin exposure
    delay(exposure_ms);                         // exposure window
    camera.trigger_stop();
    analogWrite(LED_PWM_PIN, 0);                // OFF immediately
}
```

## Освещение — epi-fluorescence (Phase 0 optional, Phase A required)

| Параметр | Значение |
|---|---|
| Housing | OPTON HBO 50W mercury (физически установлен) |
| Current state | Лампа скорее всего дохла (30+ лет) |
| Filter cube 1 (DAPI) | BP 436 / FT 460 / LP 470 |
| Filter cube 2 (FITC/GFP) | 450-490 / FT 510 / LP 520 |
| **Upgrade Phase A** | HBO 50W bulb $50-150 ИЛИ LED epi-source $400-500 |

## Лазер (Phase 0)

| Параметр | Значение |
|---|---|
| Wavelength | 450 nm (blue diode) |
| Type | CW (continuous wave) |
| Max optical power | 500 mW |
| Operating range | 10-100 mW (underrun via PWM) |
| Focal spot через 40× | 2-5 µm |
| Phototoxicity radius | >10 µm (CW → limited to plant cells) |
| TTL input | 0-5V PWM, rise time ~100µs |
| Safety class | 3B / 4 on peak |
| **Upgrade Phase A target** | Cobolt Tor 355 nm Q-switched ns ($15-25K) |

## Камера (scientific)

| Параметр | Значение |
|---|---|
| Model | ToupCam E3CMOS05000KMA MONO |
| Sensor | Sony IMX264 CMOS |
| Resolution | 5 MP (2448 × 2048) |
| Pixel size | 3.45 µm |
| Shutter | Global |
| QE peak | 73% @ 550 nm |
| Read noise | 2.4 e⁻ RMS |
| Dark current @ 25°C | 0.3 e⁻/s (no TEC) |
| Bit depth | 12 bit |
| Frame rate | 23 fps at full res |
| Interface | USB 3.0 |

## XY stage (motorized)

| Параметр | Значение |
|---|---|
| Base mechanical | XIMU LGY40-C manual cross-roller XY (куплен) |
| Table size | 40 × 40 мм |
| **Travel** | **±6.5 мм = 13 мм total per axis** (verified per vendor spec 2026-04-24) |
| Load capacity | 19.6 N (2 kg) |
| Parallelism | 0.06 мм |
| Micrometer heads | 2× класса 0-13мм (шаг 0.5 мм/rev, верньер 10 µm/div) |
| **Thimble Ø** | **14.5 мм** (замерено штангенциркулем 2026-04-24) |
| Weight | ~0.26 kg |
| Stepper (Variant A) | NEMA-11 0.9°/step, 400 full steps/оборот, 5mm shaft |
| Driver | A4988 или DRV8825, microstepping ×16 |
| Coupler | **Flex 5mm → 14mm + heatshrink 0.5мм** (эффективно 14.5мм) |
| **Precision open-loop** | 0.5mm / (400×16) = **78 nm/microstep** theoretical |
| **Practical repeatability** | **1-5 µm** (после homing + backlash comp) |
| **Abs accuracy open-loop** | **±10 µm** over 13mm travel |
| Homing | 4× endstop microswitches (X+, X-, Y+, Y-) |

## Control electronics

| Компонент | Model | Role |
|---|---|---|
| MCU main | Arduino Nano R3 | Realtime (PWM, stepper, interlock, sensors) |
| WiFi | NodeMCU ESP8266 | MQTT, alerts |
| Temp sensors | DS18B20 × 3 | LED heatsink, box air, stage |
| Photodiode | BPW34 | Laser dose feedback |
| Door switch | Magnetic reed NC | Interlock (hardware-first) |
| Laser kill | SPDT 12V relay | Hardware kill (физически разрывает +12V) |
| LED/Laser PWM | IRLZ44N MOSFET × 2 | Power switching |

## Power consumption

| Устройство | Мощность |
|---|---|
| LED (underrun 5W) | 5 W |
| Laser (50% PWM avg) | 5-10 W |
| Fan Noctua 120mm | 2 W |
| Arduino + sensors | 0.5 W |
| IR LED strip (overview cam) | 1 W |
| PC idle | 50 W |
| ToupCam USB | 2 W |
| **Total inside box** | ~15 W |
| **Total system** | ~80-100 W |
| UPS SMT1500 capacity | 900 W (derated VA) |
| Runtime @ 100W | 2-3 hours |

## Бокс (enclosure)

| Параметр | Значение |
|---|---|
| Material | ACP 3mm black/black |
| External dims | 600 × 500 × 700 мм |
| Internal volume | ~210 л |
| Frame | Al profile 20×20×2 мм, 12 рёбер |
| Vents | 2× Z-baffle light-trap 80×80×40 мм + Noctua 120 |
| Light tightness target | <0.01 lux leakage снаружи при 100% LED |
| Interlock | Reed switch + SPDT relay + Arduino interrupt |

## Timeline / Data

| Параметр | Значение |
|---|---|
| Assembly | 2-3 недели (Weeks 1-3 покупки, Weeks 4-6 сборка) |
| Commissioning session | 6 месяцев непрерывно |
| Time-lapse interval | 30 мин |
| N positions | до 10 fields |
| Per-image size | 10 MB (5MP mono 16-bit TIFF) |
| Per-day data | ~5 GB |
| Total 6мес | ~900 GB |
| Storage primary | Internal PC SSD 1TB |
| Storage backup | External HDD 4TB (weekly rsync) |

## Бюджет

| Раздел | Минимум | Оптимум |
|---|---|---|
| Освещение (LED retrofit) | $47 | $87 |
| Motorized stage (LGY40-C + stepper) | $50 | $340 (premium) |
| Laser + safety | $40 | $70 |
| Camera + adapter | $394 | $610 |
| Arduino + sensors | $40 | $50 |
| Enclosure + ventilation | $100 | $150 |
…<truncated 3 more lines>…
```
### `UPGRADE.md` (3732 chars)
```md
# UPGRADE.md — Experiment 0

**Версия:** 1.0
**Дата:** 2026-04-23
**Назначение:** Roadmap улучшений. Что реализовано, что pending, что отменено.

## Реализовано

- [x✓] Инвентарь оригинальной оптики Zeiss IM 35 (фото 2026-04-23, 35 файлов в `photos/`)
- [x✓] LGY40-C manual XY stage куплен (XIMU 喜睦)
- [x✓] Halogen spec подтверждена — OSRAM 64607 8V 50W (НЕ 12V 60W GZ6.35 как в ранних docs)
- [x✓] OPTON filter cube 46 52 24-01 (DAPI + FITC/GFP) — подтверждён на месте
- [x✓] OPTON photo port adapter 47 17 73-9901 — подтверждён
- [x✓] Spare halogen lamp OSRAM 64607 в коробке — один резервный
- [x✓] Orange acrylic UV/blue-block shield (~A4) — подтверждён
- [x✓] Core project structure (CONCEPT, MAP, PARAMETERS, KNOWLEDGE, TODO, UPGRADE, LINKS, CLAUDE, MEMORY)
- [x✓] BOM §1b переписан под LGY40-C (старый NEMA-17 + belt + knob coupler отменён)
- [x✓] Control architecture (01_CONTROL_ARCHITECTURE.md) documented для specialist review

## Запланировано (P1 — до Weeks 4)

- [ ] AliExpress закупка LED retrofit + laser + Arduino + sensors
- [ ] AliExpress закупка ToupCam + adapter
- [ ] AliExpress закупка LGY40-C motorization (NEMA-8 + A4988 + couplers)
- [ ] Локальная закупка ACP + Al profile + виброподкладки + Elodea
- [ ] eBay/Amazon UPS
- [ ] External HDD 4TB

## Запланировано (P2 — Weeks 4-6)

- [ ] Enclosure сборка (ACP каркас, интерlock, vents)
- [ ] LED retrofit (Cree XHP50 + underrun 5W)
- [ ] Монтаж микроскопа в бокс
- [ ] LGY40-C motorization и тестирование
- [ ] Arduino firmware + serial protocol
- [ ] Python driver layer (tool functions API)
- [ ] Claude Code /overnight integration

## Запланировано (P3 — после Weeks 7-8)

- [ ] Dose matrix calibration (PWM × duration, 7 сессий × N=10)
- [ ] Sham controls (4 arms)
- [ ] Light-tight verification (<0.01 lux leakage)
- [ ] Safety interlock full test

## Запланировано (P4 — Months 2-6)

- [ ] 6-мес time-lapse session (30-min interval, 10 positions, OME-TIFF)
- [ ] Weekly rsync на external HDD
- [ ] Analysis pipeline (cellpose segmentation + CenFind chloroplast detect)
- [ ] Preprint bioRxiv (если data valid)

## Отменённые планы (DEPRECATED)

- [x❌] **NEMA-17 + GT2 belt + knob couplers на Zeiss stage** (старый §2.1b) — заменён на LGY40-C + linear steppers (см. BOM §1b revised). Причины: низкая точность (±20-50µm vs ±1-5µm), риск для винтажного Zeiss stage, частичная обратимость.
- [x❌] **12V 60W GZ6.35 halogen** — неправильный spec, actual = OSRAM 64607 8V 50W. BOM §1.5/1.6 обновлён.
- [x❌] **Elodea как pilot для Impetus Aim A** — reframe 2026-04-23 после DeepSeek peer review. Теперь это HW+SW commissioning, НЕ биологический пилот.

## Известные пробелы (нужно дорабатывать)

- ⚠️ Legacy files `Phase_0_Prototype.md`, `Техническая_реализация.md`, `Покупки_Китай.md`, `Полное_Описание.md` всё ещё содержат ссылки на старый план (NEMA-17 + knob coupler) в отдельных параграфах. Нужно пройти и актуализировать. Приоритет: не блокирует specialist review т.к. primary docs (INDEX, 00, 01, 02, BOM) уже правильные.
- ⚠️ Halogen spec в legacy files (Phase_0_Prototype, Техническая_реализация, Полное_Описание §1.3/§5.3) всё ещё говорит 12V 60W GZ6.35. Обновить.
- ⚠️ ENCLOSURE.md чертёж LED mount adapter написан под GZ6.35 base — переделать под правильный prefocus base после измерения.

## Long-term (Experiment A, Phase A после Impetus funded)

- iPSC-organoids + Centriolin-RITE system (Royall 2023 eLife PMID 37882444) ИЛИ Drosophila GSC Centrobin-GFP (Yamashita 2003 Science PMID 12970569)
- 355 nm Q-switched ns laser (Cobolt Tor) — замена 450 nm CW
- Phase contrast / DIC для centriole visualization
- Z-focus motorization (NEMA-8 на Zeiss fine focus knob, или piezo Z-stage)
- HBO 50W bulb replacement или LED epi-source для fluorescence

```
### `TODO.md` (3870 chars)
```md
# TODO.md — Experiment 0

## P0 — Pre-commissioning (до покупки compoments)

- [ ] **Измерить prefocus base** halogen socket (штангенциркуль — диаметр, pin spacing)
- [ ] **Сфотографировать photo port открытый** (после снятия M35W film camera) — для подбора C-mount adapter
- [ ] **Визуальная инвентаризация объективов** на турели (гравировка на каждом)
- [ ] **Замерить габариты** микроскопа (для ENCLOSURE.md — если >600×500×700, увеличить бокс)

## P1 — Закупки (Weeks 1-3)

- [ ] AliExpress Week 1 ($145): LED retrofit (Cree XHP50 + Meanwell LDD-700H), laser 450nm, safety goggles OD4+, Arduino starter kit, sensors (DS18B20, BPW34, IRLZ44N, reed switch, relay)
- [ ] AliExpress Week 2 ($130): RPi Camera Module 3 Wide NoIR + Pi Zero 2W, USB endoscope, Noctua 120mm fan, cable glands, EPDM seal
- [ ] AliExpress Week 3 ($395-610): ToupCam E3CMOS05000KMA MONO + Zeiss → C-mount adapter (+0.5× reducer опционально)
- [ ] AliExpress LGY40-C motorization ($50): 2× NEMA-8/11 steppers, A4988 drivers, flex couplers, endstops, mounting bracket
- [ ] Локально Тбилиси ($150): ACP 3mm, Al profile, матовая чёрная краска, Elodea canadensis, предметные/покровные стёкла, External HDD 4TB, виброподкладки
- [ ] eBay / Amazon: APC Smart-UPS SMT1500 (Renewed ~$299 ИЛИ Tbilisi tapio.ge $150)

## P2 — Сборка (Weeks 4-6)

- [ ] **Enclosure**: раскрой ACP + каркас Al profile + монтаж ACP + силикон + покраска внутри матовой чёрной (2 слоя)
- [ ] **Дверь + interlock**: петли, reed switch, магнитный замок, EPDM уплотнитель
- [ ] **Вентиляция**: Z-baffle light-trap × 2 + Noctua fan в потолке + cable glands
- [ ] **LED retrofit**: сборка Cree XHP50 + heatsink + driver + collimator + custom mount adapter
- [ ] **Монтаж микроскопа** в бокс + виброподкладки под ножки
- [ ] **LGY40-C motorization**: 3D-print bracket + установка steppers + flex couplers + endstops
- [ ] **Köhler alignment** после LED retrofit
- [ ] **Arduino sketch**: firmware (stepper control, PWM, interlock, sensors, serial protocol JSON-lines)

## P3 — Камера + software

- [ ] Снять M35W film camera → установить Zeiss→C-mount adapter → ToupCam
- [ ] Установить Micro-Manager 2.0 + ToupCam adapter (github.com/toupcam/toupcam-mm-plugin)
- [ ] Первое сфокусированное изображение Elodea chloroplast через 40× (test TIFF 16-bit)

## P4 — Claude agent integration

- [ ] Python driver layer: tool functions `move_stage`, `fire_laser`, `capture_image`, `detect_targets`, `check_interlock`, `get_sensor_reading`
- [ ] Arduino serial protocol JSON-lines через USB 115200 baud
- [ ] Claude Code /overnight agent скрипт: цикл detect → decide → act → log, с error recovery
- [ ] Systemd unit для auto-restart Claude Code при crash
- [ ] Test безопасности: interlock trip → physical laser kill → recovery

## P5 — Commissioning 6-мес сессии

- [ ] Dose matrix calibration (PWM 10-70%, duration 100-500ms, 7 сессий × N=10 chloroplasts)
- [ ] Sham controls (untreated / empty-location / mechanical / laser test — 4 arms)
- [ ] Single ablation run testing (1 target / session)
- [ ] 6-мес time-lapse в MDA config (30-min interval, 10 positions)
- [ ] Weekly rsync backup на external HDD

## P6 — Documentation / transition to Experiment A

- [ ] Preprint на bioRxiv о commissioning methodology (если rig + agent работают стабильно)
- [ ] После успешного commissioning — спланировать Experiment A с iPSC-organoids + Centriolin-RITE (Royall 2023) ИЛИ Drosophila GSC (Yamashita 2003)
- [ ] LGY40-C upgrade до Варианта B (linear stepper actuator) если Вариант A precision недостаточна

## Pending peer-review issues (см. PEER_REVIEW_DRAFT.md)

- [ ] Явно переформулировать что это commissioning, НЕ биологический пилот (для Impetus)
- [ ] Рассмотреть переход на 355 nm Q-switched ns laser для Experiment A
- [ ] Pre-registration protocol (OSF) перед запуском 6-мес сессии
- [ ] Pre-calculate required N для statistical power

```
### `KNOWLEDGE.md` (6642 chars)
```md
# KNOWLEDGE.md — Experiment 0

**Версия:** 1.0
**Дата:** 2026-04-23
**Назначение:** Подтверждённые факты, validated references, known constraints. НЕ hypotheses — только то, что проверено.

## Validated hardware facts

1. Микроскоп **Zeiss IM 35 / ICM 405** — инвертированный, West Germany, ~1978-89. Документация: `~/Desktop/PhD/microscope/inverted-im35-icm405.pdf`.
2. Lamp housing принимает **OSRAM Halogen-BELLAPHOT 64607, 8V 50W** (verified by physical inspection 2026-04-23, запасная лампа в оригинальной OSRAM-коробке на месте).
3. Filter cube slider p/n **OPTON 46 52 24-01** (2 позиции: DAPI + FITC/GFP) — установлен.
4. Photo port adapter p/n **OPTON 47 17 73-9901** — есть, внутри 3.2× projection lens.
5. **XIMU LGY40-C** manual XY stage (40×40 мм, dovetail, 2 micrometer heads 0.5 мм pitch) — куплен, в квартире.
6. Объективы доступны из manual: Plan 6.3/0.16, Plan 16/0.35, Plan-Neofluar 25/0.8 W Oil, LD-Plan 40/0.6, Plan 40/0.65, Planapo 63/1.4 Oil, Plan 100/1.25 Oil. **Точный набор на турели — надо проверить визуально** (гравировка на объективах).

## Validated control stack capabilities

1. **Claude Code /overnight mode** — подтверждено работает через `~/Desktop/AIM/llm.py` → DeepSeek API (`~/.aim_env → DEEPSEEK_API_KEY`).
2. **DeepSeek models:** `deepseek-chat` (fast) и `deepseek-reasoner` (для сложных задач) — верифицировано 2026-04-23.
3. **LLM router** `llm.py` уже маршрутизирует между DeepSeek / Kimi / Qwen / Groq; fallback logic работает.
4. **Arduino Nano + A4988 + NEMA-8/11 → stepper** — стандартный well-tested паттерн; accelerated через AccelStepper library.
5. **Micro-Manager 2.0 + ToupCam adapter** — community adapter существует (github.com/toupcam/toupcam-mm-plugin).

## Validated references (10 authoritative PubMed publications)

| # | PMID | Year | Journal | Title (partial) |
|---|---|---|---|---|
| 1 | 15886028 | 2005 | Cell Biol Int | Potential role of centrioles in determining the morphogenetic status of animal somatic cells |
| 2 | 16336191 | 2005 | Biochemistry (Moscow) | Centriolar mechanisms of differentiation and replicative aging of higher animal cells |
| 3 | 19432168 | 2008 | Adv Gerontol | Centrosomal hypothesis of cellular aging and differentiation |
| 4 | 20426623 | 2010 | Rejuvenation Res | Centriole, differentiation, and senescence |
| 5 | 20480236 | 2011 | Biogerontology | Gerontology research in Georgia |
| 6 | 22356233 | 2012 | Nucleosides Nucleotides Nucleic Acids | Discovery of centrosomal RNA and centrosomal hypothesis |
| 7 | 22708440 | 2012 | Adv Gerontol | New class of RNA and centrosomal hypothesis of cell aging |
| 8 | 22684578 | 2013 | Protoplasma | RNA in centrosomes: structure and possible functions |
| 9 | 36583780 | 2023 | Mol Biol Rep | Reduction, proliferation, and differentiation defects of stem cells over time |
| 10 | 38510429 | 2024 | Front Pharmacol | Editorial: Molecular mechanism of ageing and therapeutic advances |

Source: `~/.claude/projects/-home-oem/memory/pubmed_authoritative.md`. Verification via PubMed E-utilities (см. `~/Desktop/Claude/audits/pubmed_verify_2026-04-24.md`).

## External references (non-author, используемые в обосновании)

Все PMID проверены 2026-04-23 (см. `Полное_Описание.md` §7):

1. **Royall L, Machado D, Jessberger S, Denoth-Lippuner A** (2023) eLife 12:e83157. **PMID 37882444** — Centriolin-RITE tool. Для Experiment A.
2. **Mangione F, D'Antuono R, Tapon N** (2022) Front Physiol 13:1093303. **PMID 36685184, PMC9845895** — 405 nm ns ablation benchmark. <!-- corrected 2026-04-26: prior wording cited fabricated «Strunov 2022 PMID 36685234»; real authors/PMID per PubMed verification, see docs/REFERENCE_AUDIT_2026-04-26.md row 7 -->
3. **Bürgy L et al** (2023) BMC Bioinformatics 24:120. **PMID 36977999, PMC10045196** — CenFind centriole detection CNN.
4. **Zeigler MB, Chiu DT** (2009) Photochemistry and Photobiology 85:1218-1224. **PMID 19558419, PMC5600466** — laser selection / cell viability.
5. **Botvinick EL, Venugopalan V, Shah JV, Liaw LH, Berns MW** (2004) Biophys J 87:4203-12. **PMID 15454403**, DOI 10.1529/biophysj.104.049528 — picosecond laser microtubule ablation, gold standard for sub-μm cellular surgery. <!-- corrected 2026-04-26: prior wording cited fabricated «Maiato/Khodjakov 2004 PMC1304929»; real authors/PMID per docs/REFERENCE_AUDIT_2026-04-26.md row 4 -->
6. **Yamashita YM, Jones DL, Fuller MT** (2003) Science 301:1547-1550. **PMID 12970569** — Drosophila GSC centrosome asymmetric division.
7. **Verzijlbergen KF et al** (2010) PNAS 107:64-68. **PMID 20018668** — Original RITE.
8. **Icha J et al** (2017) BioEssays 39:e201700003. **PMID 28749075** — phototoxicity guide. <!-- corrected 2026-04-26: PMID was 28749007 (off-by-68, that PMID = Aitken sperm ROS paper); correct PMID per docs/REFERENCE_AUDIT_2026-04-26.md row 21 -->
9. **Vogel A et al** (2005) Applied Physics B 81:1015-1047 — fs laser nanosurgery mechanisms.
10. **Laissue PP et al** (2017) Nat Methods 14:657-661. **PMID 28661494** — phototoxicity assessment. <!-- corrected 2026-04-26: PMID was 28661495 (off-by-1, that PMID = Zoppè size perception); correct PMID per docs/REFERENCE_AUDIT_2026-04-26.md row 22 -->

## Known physical constraints

- **Vibration:** квартирный стол, нет оптического стола. Нужны резиновые виброподкладки 10мм. Мitigation частичная — для 100×/1.4 + laser ablation может быть критично, для 40×/0.65 ok.
- **Thermal drift:** отсутствует активное охлаждение. В жаркую погоду +2°C = stage drift ~5 µm за 1 час. Для 30-min intervals приемлемо.
- **Phototoxicity:** 450 nm CW фототоксичен. Dose matrix calibration обязательна (см. `Полное_Описание.md` §4.4).
- **Elodea стохастичность:** chloroplast inheritance stochastic, НЕ моделирует детерминированную центриолярную асимметрию (см. PEER_REVIEW_DRAFT).

## Known software constraints

- Claude Code session timeout (длинные сессии могут прерываться) — mitigation через /overnight + systemd restart.
- DeepSeek API rate limits — не должно быть проблемой на этом masштабе.
- Arduino Nano 2KB RAM — ограничивает sophisticated motion planning; достаточно для Step/Dir + interrupt handlers.
- Micro-Manager MDA (Multi-Dimensional Acquisition) надёжность на 6-месячных сессиях не верифицирована — risk.

## Validated budget estimates

См. `PARAMETERS.md` раздел "Бюджет" + `BOM.md` — итоговый диапазон $881–$1687 для minimum и optimum комплектов.

## References not yet verified

- ~~Точные параметры OSRAM 64607 base type (prefocus)~~ — требует измерения штангенциркулем
- ~~Photo port thread type для C-mount adapter~~ — требует фото open port изнутри
- ~~Объективы на турели~~ — требует визуальной проверки гравировки

```
### `MEMORY.md` (1768 chars)
```md
# MEMORY.md — Experiment 0

**Назначение:** Индекс auto-memory записей, релевантных для этого проекта. Живут в `~/.claude/projects/-home-oem/memory/`.

## Релевантные memories

### Project-level

- `project_cdata_copi_candidates.md` — Impetus Phase A co-PI candidates (rig будет использован в Phase A)
- (PhD-supervisor memories удалены 2026-05-04 правилом feedback_no_supervisor_names)
- `project_aubrey_collaboration.md` — Aubrey de Grey engagement with CDATA (context)
- `project_academic_upgrade.md` — академический upgrade pathway
- `project_eic_umbrella.md` — EIC Pathfinder consortium (Experiment 0 → WP3)

### Feedback (правила работы в проекте)

- `feedback_bradford_hill_rule.md` — Bradford Hill criteria для causality claims
- `feedback_mcoa_cdata_comparison.md` — MCOA vs CDATA comparison methodology
- `feedback_deepseek_primary.md` — DeepSeek как primary LLM
- `feedback_verify_references.md` — правило проверки reference перед claim

### Reference

- `pubmed_authoritative.md` — 10 authoritative PubMed publications
- `publications.md` — полный список публикаций автора
- `feedback_article_workflow.md` — workflow для submission

## Что сохранять в auto-memory из Experiment 0 работы

1. **Validated hardware specs** после закупки и тестирования (actual values, not estimated)
2. **Calibration constants** после dose matrix session (PWM → mW curve)
3. **Known issues** specific to этому конкретному rig (вибрация источников, thermal drift при t°C в квартире)
4. **Lessons learned** после Phase 0 → применимы к Phase A

## Как использовать

Before starting any substantive work on этом проекте — прочитать релевантные memories выше. После важных discoveries (validation, calibration, unexpected behavior) — сохранять новую memory в правильной категории.

```
### `LINKS.md` (6239 chars)
```md
# LINKS.md — Experiment 0

**Версия:** 1.0
**Дата:** 2026-04-23
**Назначение:** Внешние ссылки (AliExpress, eBay, datasheets, PMIDs) собранные в одном месте.

## Vendor / shopping

### AliExpress (Week 1 — освещение + laser + electronics)

- Cree XHP50.2 J4 6500K 20mm copper star: https://www.aliexpress.com/item/32819991436.html
- Meanwell LDD-700H constant-current driver: https://www.aliexpress.com/item/32615558869.html
- Aspheric collimator 10° 10mm LED: https://www.aliexpress.com/wholesale?SearchText=aspheric+collimator+10+degree+LED+lens
- OSRAM Halogen-BELLAPHOT 64607 8V 50W (spare): https://www.aliexpress.com/wholesale?SearchText=OSRAM+64607+8V+50W+halogen
- 450 nm 500mW laser module TTL focusable: https://www.aliexpress.com/wholesale?SearchText=450nm+laser+module+TTL+focusable+500mW
  - Powell lens variant: https://www.aliexpress.us/item/3256808918024344.html
- Laser safety goggles OD4+ 400-500 nm: https://www.aliexpress.com/wholesale?SearchText=laser+safety+goggles+450nm+blue+OD+4
- Arduino Nano R3 + ESP8266 combo: https://www.aliexpress.com/wholesale?SearchText=Arduino+Nano+ESP8266+combo
- DS18B20 waterproof temperature: https://www.aliexpress.com/wholesale?SearchText=DS18B20+waterproof+temperature
- BPW34 photodiode: https://www.aliexpress.com/wholesale?SearchText=BPW34+photodiode
- IRLZ44N MOSFET: https://www.aliexpress.com/wholesale?SearchText=IRLZ44N+MOSFET
- Magnetic reed switch: https://www.aliexpress.com/wholesale?SearchText=magnetic+door+reed+switch
- SPDT 12V relay module: https://www.aliexpress.com/wholesale?SearchText=SPDT+12V+relay+module

### AliExpress (Week 2 — overview cameras + enclosure)

- Raspberry Pi Camera Module 3 Wide NoIR 12MP: https://www.aliexpress.com/item/1005005121309398.html
- Raspberry Pi Zero 2W: https://www.aliexpress.com/wholesale?SearchText=Raspberry+Pi+Zero+2W
- USB endoscope 1080p 8mm focusable: https://www.aliexpress.us/item/3256806334941454.html
- Noctua NF-A12x25 120mm: https://www.aliexpress.com/wholesale?SearchText=Noctua+NF-A12x25+120mm
- Cable glands PG7: https://www.aliexpress.com/wholesale?SearchText=cable+gland+PG7

### AliExpress (Week 3 — scientific camera + adapter)

- ToupCam E3CMOS05000KMA MONO (Sony IMX264): https://www.aliexpress.com/wholesale?SearchText=ToupCam+E3CMOS05000+mono
- Zeiss photo port → C-mount adapter 1×: https://www.aliexpress.com/item/32823540728.html
- Zeiss 0.5× reducer adapter (если нужен wider FOV): https://www.aliexpress.com/item/4000837404339.html

### AliExpress (LGY40-C motorization)

- NEMA-8 / NEMA-11 stepper 0.9°/step: https://www.aliexpress.com/wholesale?SearchText=NEMA-11+stepper+0.9
- A4988 / DRV8825 stepper driver: https://www.aliexpress.com/wholesale?SearchText=DRV8825+stepper+driver
- Flex shaft coupler 5mm→6mm: https://www.aliexpress.com/wholesale?SearchText=flex+shaft+coupler+5mm+6mm
- Endstop microswitch: https://www.aliexpress.com/wholesale?SearchText=endstop+microswitch

### eBay / Amazon

- APC Smart-UPS SMT1500 pure sine Renewed: https://www.amazon.com/APC-Smart-UPS-Battery-SMT1500-Renewed/dp/B07N5L2DW2
- APC Smart-UPS SMT1500 б/у: https://www.ebay.com/itm/155463846957
- Used Zeiss IM 35 photo adapter C-mount: https://www.ebay.com/sch/i.html?_nkw=Zeiss+IM+35+photo+adapter+C-mount

### Локально Тбилиси

- ACP 3mm black/black 1500×1000 (алюкобонд): строит. рынок Лило
- Al profile 20×20×2 mm 8м: Castorama / рынок
- Чёрная матовая краска: хоз. магазин
- Elodea canadensis: аквариумный магазин
- Предметные/покровные стёкла: медмагазин
- External HDD 4TB WD/Seagate: tapio.ge
- EPDM виброподкладки 10мм: строит. рынок
- Силикон чёрный Permatex: авторынок

### Taobao (через брата — китайский текст в `Покупки_Китай.md`)

Ключевые запросы:
- 图帝 E3CMOS 500万像素 单色 显微镜相机 (ToupCam mono)
- 450nm 500mW 激光模块 TTL 可调焦 (450nm laser)
- CREE XHP50 6500K 20mm 星板 (Cree LED)
- 明纬 LDD-700H LED 驱动器 (Meanwell driver)
- 蔡司 显微镜 C接口 转接环 30mm (Zeiss C-mount adapter)
- 树莓派 Camera Module 3 广角 NoIR (RPi Cam)

## Software / Documentation

- Micro-Manager 2.0 Linux: https://download.micro-manager.org/nightly/2.0/Linux/MMSetup_Linux.sh
- ToupCam MM adapter: https://github.com/toupcam/toupcam-mm-plugin
- Cellpose (cell segmentation): https://github.com/MouseLand/cellpose
- CenFind (centriole detection CNN): https://github.com/UPOTS/centriole-detection (нужно verify URL)
- Claude Code: Anthropic CLI
- DeepSeek API: https://platform.deepseek.com

## Datasheets

- Cree XHP50.2 J4: https://www.cree.com/led-components/media/documents/XLampXHP50_2.pdf
- Meanwell LDD-700H: https://www.meanwell.com/Upload/PDF/LDD-H/LDD-H-SPEC.PDF
- Arduino Nano R3 pinout: https://content.arduino.cc/assets/Pinout-NANO_latest.png
- A4988 stepper driver: https://www.pololu.com/product/1182
- DS18B20 datasheet: https://datasheets.maximintegrated.com/en/ds/DS18B20.pdf
- BPW34 photodiode: https://www.vishay.com/docs/81521/bpw34.pdf
- Sony IMX264 sensor: datasheet via https://www.sony-semicon.com/en/products/is/industry/global-shutter.html

## PMIDs cited в проекте

### Authoritative 10 publications (author's own)

- 15886028, 16336191, 19432168, 20426623, 20480236, 22356233, 22708440, 22684578, 36583780, 38510429

Источник: `~/.claude/projects/-home-oem/memory/pubmed_authoritative.md`.

### External references

- 37882444 — Royall 2023 eLife (Centriolin-RITE)
- 36685184 — Mangione F, D'Antuono R, Tapon N 2022 Front Physiol 13:1093303 (405nm ns ablation) <!-- corrected 2026-04-26: prior «Strunov 36685234» was fabricated -->
- 36977999 — Bürgy 2023 BMC Bioinformatics (CenFind CNN)
- 19558419 — Zeigler & Chiu 2009 Photochem Photobiol (laser cell viability)
- 12970569 — Yamashita 2003 Science (Drosophila GSC centrosome)
- 20018668 — Verzijlbergen 2010 PNAS (RITE original)
- 28749075 — Icha 2017 BioEssays (phototoxicity) <!-- corrected 2026-04-26: was 28749007 (off-by-68) -->
- 28661494 — Laissue 2017 Nat Methods (phototoxicity assessment) <!-- corrected 2026-04-26: was 28661495 (off-by-1) -->

## Zeiss / Opton references

- Zeiss IM 35 manual: `~/Desktop/PhD/microscope/inverted-im35-icm405.pdf`
- OSRAM 64607 lamp datasheet: https://www.osram.com/ecat/Classic/64607 (verify URL)
- OPTON 46 52 24-01 filter block — Zeiss catalog legacy
- OPTON 47 17 73-9901 photo port adapter — Zeiss catalog legacy

```