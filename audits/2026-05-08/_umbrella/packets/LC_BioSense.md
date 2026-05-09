# AUDIT PACKET — LC_BioSense

Path: `/home/oem/Desktop/LongevityCommon/BioSense`  Date: 2026-05-08

## Size & file counts
```
393M	/home/oem/Desktop/LongevityCommon/BioSense
```
**Extensions:** .mat=211, .json=99, .md=11, .png=11, .py=8, .gz=7, .docx=3, .m=3, .sh=2, .zip=2, (noext)=1, .ex=1, .heex=1, .exs=1, .txt=1
## Tree (depth=2, max 200 entries)
```
.
./ze_Middle_50.json
./PARAMETERS.md
./ze_Young_25.json
./biosense-web
./biosense-web/lib
./biosense-web/config
./src
./src/requirements.txt
./src/ze_batch_pipeline.py
./src/ze_bandwise.py
./src/ze_dortmund_pipeline.py
./src/eeg_ze_processor.py
./src/ze_alpha_peak.py
./src/ze_ec_eo_analysis.py
./src/ze_lemon_analysis.py
./src/ze_cuban_analysis.py
./biosense.sh
./Materials
./Materials/Ze.docx
./Materials/Twin Paradox Without Paradox.docx
./Materials/Toward Integral Field Tomography of Living Systems.docx
./README.md
./results
./results/ze_Elder_70.json
./results/ze_Young_25.json
./results/ze_Middle_45.json
./results/ze_age_comparison.png
./UPGRADE.md
./backend
./backend/deploy
./backend/Cargo.toml
./backend/src
./backend/Cargo.lock
./TODO.md
./MEMORY.md
./data
./data/lemon
./data/zenodo
./data/dortmund
./data/cuban
./ze_eeg_validation
./CLAUDE.md
./LINKS.md
./ze_Elder_72.json
./CONCEPT.md
./MAP.md
./KNOWLEDGE.md
./ze_age_comparison.png
```
## Detected stack: **Rust, Python**
## Core files

### `CLAUDE.md` (1832 chars)
```md
# BioSense — Носимый браслет: EEG · HRV · Запах

## Backend port — `:4502` (native systemd, production since 2026-05-08)

**Production:** `biosense-backend.service` (native Rust, this crate
`BioSense/backend/`) on `127.0.0.1:4502`. nginx `biosense.longevity.ge`
proxies `/api/` → `:4502/api/*`, `/live/` and `/` → `:4501`
(`biosense-web` Phoenix LiveView dashboard).

**Wire format:** ChiZeRequest accepts BOTH conventions via `serde(alias)`:
- `{"v_eeg": x, "v_hrv": y, "v_resp": z, "v_sleep": w}` — idiomatic Rust shape.
- `{"eeg": x, "hrv": y, "resp": z, "sleep": w}` — legacy shape used by
  Phoenix `biosense-web` client and the retired Docker container.

**Routes** (all mounted at both `/<name>` and `/api/<name>`):
- `GET  /healthz` — liveness
- `POST /chi_ze` — composed χ_Ze biomarker computation
- `POST /bridge` — CDATA D → χ_Ze stub
- `POST /exacerbation` — risk score
- `GET  /v_star` — canonical v* (Article + Python forms)

**History:** Until 2026-05-08, `:4502` was held by the Docker container
`deploy-biosense-backend-1` (image `deploy-biosense-backend`, running
since Apr 30). After Phase 4.4 field-name reconciliation
(`#[serde(alias)]`), the container was stopped and native systemd
took over. Per memory `feedback_no_docker` rule.

---


## 📌 Правило: DeepSeek для нетехнических задач

**Код (Python/Rust) — Claude. Всё остальное — DeepSeek API.**
Примеры: статьи о χ_Ze, peer review, введение/обсуждение, переводы.
**Ключ:** `~/.aim_env → DEEPSEEK_API_KEY` · **Вход:** `~/Desktop/AIM/llm.py`
**Модели:** `deepseek-chat` (быстро) · `deepseek-reasoner` (научные рассуждения)

---

## Проект

Верификация Ze-теории на EEG-данных. Гипотеза: χ_Ze(молодые) > χ_Ze(пожилые).
Статья `ze_eeg_paper.docx` v8 (410KB) — почти готова к отправке.

## Связь с AIM

`ze_ecg.py` → AIM HRV анализ пациентов (χ_Ze, v*, RMSSD).

```
### `README.md` (3298 chars)
```md
# BioSense

**Multisensor wearable platform for Ze-based biomarker analysis: EEG · HRV · Olfaction**

BioSense applies Ze Theory (Tkemaladze) to three biosignal channels for aging biomarker detection
and clinical diagnostics. The EEG module is validated across 4 public datasets (N up to 196,
lifespan ages 5–97). HRV and olfaction modules are in development.

---

## Modules

| Module | Status | Key metric |
|--------|--------|------------|
| EEG | Validated | χ_Ze aging index; Cuban d=1.694; Dortmund p=0.006 |
| HRV | Planned | χ_Ze of RR intervals, autonomic profiling |
| Olfaction | Planned | Turin tunneling theory, VOC diagnostics |

---

## Quick Start (EEG)

```bash
# Install dependencies
pip install -r src/requirements.txt

# Demo (no data needed)
python3 src/eeg_ze_processor.py --demo

# Single EEG file
python3 src/eeg_ze_processor.py --file recording.edf --age 35 --label "Subj01" --resample 128

# Cuban lifespan dataset
export ZE_CUBAN_DIR=/path/to/cuban/EyesClose
python3 src/ze_cuban_analysis.py

# Dortmund young vs old
export ZE_DORTMUND_DIR=/path/to/dortmund
python3 src/ze_dortmund_pipeline.py
```

Or use the launcher:

```bash
./biosense.sh
```

---

## Ze Theory (Core)

```
Binary sequence:  x_k = 1  if  sample > median, else 0
Ze velocity:      v = N_S / (N − 1)        [N_S = switches]
Fixed point:      v* = 0.45631
Cheating index:   χ_Ze = 1 − |v − v*| / max(v*, 1−v*)    ∈ [0, 1]
```

**Aging hypothesis:** signal slows with age → v moves away from v* → χ_Ze decreases.

Ze-optimal frequency: **f_opt = v* × fs/2** (≈ 29.2 Hz at 128 Hz sampling rate)

---

## Structure

```
BioSense/
├── CONCEPT.md          # Full project concept
├── README.md           # This file
├── CLAUDE.md           # AI assistant rules
├── TODO.md             # Task list
├── PARAMETERS.md       # Key parameters and constants
├── MAP.md              # Component and dependency map
├── MEMORY.md           # Decisions and lessons learned
├── LINKS.md            # Ecosystem connections
├── KNOWLEDGE.md        # Domain knowledge corpus
├── biosense.sh         # Main launcher
├── src/                # All source code
│   ├── eeg_ze_processor.py
│   ├── ze_cuban_analysis.py
│   ├── ze_dortmund_pipeline.py
│   ├── ze_ec_eo_analysis.py
│   ├── ze_lemon_analysis.py
│   ├── ze_bandwise.py
│   ├── ze_alpha_peak.py
│   ├── ze_batch_pipeline.py
│   └── requirements.txt
├── data/               # Datasets (not committed to git)
│   ├── cuban/
│   ├── lemon/
│   └── zenodo/
├── results/            # Analysis outputs (JSON, PNG)
└── Materials/          # Reference documents (Ze.docx, etc.)
```

---

## Validated Results

| Dataset | N | Age range | Result |
|---------|---|-----------|--------|
| Zenodo 3875159 EC vs EO | 1 subj | — | Δχ_Ze = +0.064 |
| MPI-LEMON | 30 | 22–72 yr | d=0.110, p=0.765 (underpowered) |
| Dortmund ds005385 | 60 | 20–70 yr | p=0.006, d=0.732; AUC=0.715 |
| Cuban Zenodo 4244765 | 196 | 5–97 yr | Inverted-U, peak 36.5 yr, d=1.694 |

---

## Citation

Tkemaladze, J. (2026). *Ze cheating index (χ_Ze) as a group-level index of neurodynamic aging:
Experimental EEG validation across the human lifespan.* [Manuscript under review]

Also cite:
- PMID 36583780 — Tkemaladze J. *Mol Biol Reports* 2023
- PMID 20480236 — Lezhava T. et al. *Biogerontology* 2011

```
### `CONCEPT.md` (17941 chars)
```md
# BioSense — Мультисенсорная носимая платформа: ЭЭГ · ВСР · Ольфакция

**Статус:** Активная разработка — модуль ЭЭГ валидирован, модули ВСР и ольфакции на стадии проектирования.
**Версия:** 3.0 (2026-03-29) — обновление по результатам экспертной рецензии.

---

## Миссия

BioSense — это носимая платформа биомониторинга, объединяющая три комплементарных канала анализа биологических сигналов для детекции биомаркеров старения и клинической диагностики в реальном времени:

1. **ЭЭГ (электроэнцефалография)** — Ze-анализ мозговых ритмов (\( \chi_{Ze} \), биомаркер когнитивного старения)
2. **ВСР / RR-интервалы** — Ze-поток кардиосигнала, оценка вегетативного статуса, детекция предболезненных состояний
3. **Ольфакция / ЛОС** — молекулярная спектроскопия (теория Турина: тунеллирование электронов), диагностика по летучим органическим соединениям пота и выдыхаемого воздуха

---

## Единая теоретическая основа: Ze Theory

Ze Theory (Ткемаладзе) определяет:

- **Бинарная скорость переключения:** \( v = N_S / (N - 1) \), где \( N_S \) — количество переключений между состояниями, \( v \in [0, 1] \)
- **Теоретическая фиксированная точка:** \( v^* = 0.45631 \) (Python form; **Article form** — канонический cross-subproject — \( v^* = -0.08738 \), root `PARAMETERS.md § 1`)
- **Индекс старения (cheating index):** \( \chi_{Ze} = 1 - |v - v^*| / \max(v^*, 1 - v^*) \), \( \chi_{Ze} \in [0, 1] \)

**Происхождение \( v^* \):**  
Значение \( v^* = 0.45631 \) получено из теоретической модели динамических систем, описывающей переход между двумя состояниями как процесс максимальной материализации (эквивалент золотого сечения для бинарных систем). Оно не зависит от эмпирических данных и служит эталоном, относительно которого оценивается отклонение биологических сигналов. Эмпирически у здоровых молодых испытуемых среднее значение \( v \) близко к \( v^* \), что подтверждает теоретический статус этой точки как оптимума.

**Основная гипотеза старения:** С возрастом биологический сигнал замедляется → доминирующая частота снижается → \( v \) удаляется от \( v^* \) → \( \chi_{Ze} \) уменьшается.

**Абсолютный физиологический Ze-оптимальный диапазон:**  
В ходе валидации на 4 датасетах ЭЭГ выявлено, что наиболее чувствительной к возрастным изменениям является зона **25–35 Гц** (бета/гамма-граница).  
Эта полоса является **аппаратно независимым биологическим инвариантом**. Сигнал всегда ресемплируется до 128 Гц перед анализом, что обеспечивает стабильность расчетов.

---

## Модуль 1: ЭЭГ

**Статус:** Валидирован на 4 датасетах (2025–2026)  
**Гипотеза:** \( \chi_{Ze}(\text{молодые}) > \chi_{Ze}(\text{пожилые}) \)

### Результаты валидации

| Датасет | N | Ключевой результат |
|---------|---|-------------------|
| Zenodo 3875159 — EO vs EC | 1 исп. | \( \Delta\chi_{Ze}(EO-EC) = +0.064 \) (внутрисубъектный эффект) |
| MPI-LEMON alpha peak | 30 | d = 0.110, p = 0.765 (недостаточная мощность) |
| Dortmund Vital Study ds005385 | 60 | p = 0.006, d = 0.732; ANCOVA p = 0.037; AUC = 0.715 |
| Cuban Normative EEG Zenodo 4244765 | 196 | Инвертированная U-образная зависимость, пик в 36.5 лет; \( \chi_{Ze} \) (20–30 лет): 0.87 ± 0.04; \( \chi_{Ze} \) (60+ лет): 0.71 ± 0.06; d = 1.694, p < 0.0001 |

### Фундаментальное наблюдение
Альфа-ритм (~10 Гц) находится далеко от Ze-оптимальной зоны, поэтому индивидуальные различия в нем малы.  
Ze-анализ дает максимальную чувствительность:
- в переходных процессах (открытые/закрытые глаза)
- в когнитивных задачах, активирующих бета/гамма-ритм
- при анализе узкой полосы 25–35 Гц

---

## Модуль 2: ВСР (HRV) — детализированный алгоритм

**Статус:** Проектирование, прототип `ze_ecg.py`

### Подход
Для RR-ряда — нестационарной временной последовательности — предлагается двухэтапный алгоритм:

1. **Спектральная декомпозиция:**  
   Вычисление спектра мощности ВСР с выделением компонент:
   - LF (0.04–0.15 Гц) — симпатическая активность
   - HF (0.15–0.4 Гц) — парасимпатическая активность

2. **Ze-анализ с гистерезисом:**  
   Состояние определяется с учетом гистерезиса для устойчивости:
   - Если LF > HF × (1 + δ) → симпатическое состояние (1)
   - Если HF > LF × (1 + δ) → парасимпатическое состояние (0)
   - В зоне неопределенности \( |LF - HF| / \max(LF, HF) \leq \delta \) состояние сохраняется предыдущим
   - δ = 0.1 (10% зона неопределенности, калибруется эмпирически)
   
   \( N_S \) — число переходов между состояниями за временное окно (длительностью 300 секунд, 5 минут, с перекрытием 50%)
   \( N \) — общее число RR-интервалов в окне
   \( v = N_S / (N - 1) \), затем \( \chi_{Ze}^{HRV} \) вычисляется по стандартной формуле.

### Клиническая гипотеза
При предболезненных состояниях (гипертония, начальные формы диабета, хронический стресс) вегетативная регуляция становится более ригидной → переключения редки → \( v \) стремится к 0 или 1 → \( \chi_{Ze}^{HRV} \) снижается.

### Планируемая валидация
- **Датасеты:** PhysioNet CinC Challenge 2017 (норма vs фибрилляция предсердий), MIMIC-III (пациенты с ССЗ), а также собственные сборы.
- **Метрики:** AUC, чувствительность/специфичность для разделения нормы и предболезненных состояний.

---

## Модуль 3: Ольфакция / ЛОС — детализированный подход

**Теоретическая основа:** Теория Турина — ольфакторные рецепторы работают как молекулярные спектрометры (неупругое тунеллирование электронов), что позволяет детектировать молекулы по их вибрационным спектрам независимо от формы.

### Стратегия поэтапной реализации

#### Этап 1 (MVP, Q4 2026 — Q2 2027): Электронный нос (e-nose)
- Используется матрица из 6–8 коммерческих MEMS-сенсоров на основе оксидов металлов (например, Bosch BME688, Sensirion SGP40)
- Сенсоры имеют перекрестную чувствительность, но формируют уникальные паттерны отклика
- Классификация: обучение модели (SVM или нейросеть) на паттернах «молодые vs пожилые» по VOC профилю пота
- Ze-анализ применяется к временной динамике отклика сенсорной матрицы

#### Этап 2 (Q3–Q4 2027): Селективная панель маркеров
После валидации e-nose определяются наиболее информативные VOC-маркеры. Панель предварительно включает:
- **Аммиак** — коррелирует с почечной функцией и мышечным катаболизмом
- **Изопрен** — маркер холестеринового обмена
- **Ацетон** — метаболический статус
- **2-ноненаль** — характерный «запах старения»
- **Альдегиды** (нонаналь, гексаналь) — продукты перекисного окисления липидов

#### Этап 3 (2028): Прототип Турин-сенсора
Разработка экспериментального сенсора на основе эффекта неупругого электронного туннелирования — отдельный R&D-трек с ожидаемым прототипом в 2028 г.

### Ze-интерпретация для ЛОС
- Измерения проводятся сериями по 5 замеров с интервалом 1 минута
- Для каждого сенсора (или маркера) фиксируется бинарное событие: отклик выше/ниже порога (пороги определяются на этапе калибровки)
- \( v \) рассчитывается как доля изменений состояния между последовательными замерами
- \( \chi_{Ze}^{VOC} \) отражает динамическую стабильность молекулярного профиля

---

## Аппаратный прототип — уточненные требования

**Форм-фактор:** Браслет с тремя сенсорными блоками

### 1. ЭЭГ (сухие электроды)
- **Зона размещения:** лобная область (Fp1, Fp2) с референсом заушным (A1/A2)
- **Частотный диапазон интереса:** 25–35 Гц
- **AFE:** ADS1299 (8 каналов), конфигурация: 2 канала ЭЭГ, 2 канала ЭКГ, остальные зарезервированы
- **Активное экранирование:** для снижения импеданса сухих электродов
- **Гальваническая развязка:** обязательная, защита от ESD
- **Артефакты:** встроенный 3-осевой акселерометр (например, LIS3DH) для детекции движений; ICA-фильтрация на устройстве или адаптивная фильтрация с опорным каналом

### 2. ЭКГ (для ВСР)
- Два электрода на внутренней стороне браслета (используются 2 канала ADS1299)
- Частота дискретизации: 500 Гц для точного детектирования R-пиков
- Алгоритм детекции R-пиков: Pan-Tompkins, реализованный в Rust

### 3. VOC-сенсор (этап 1: e-nose)
- **Модуль:** Bosch BME688 или аналогичный (4–8 газовых сенсоров в одном корпусе)
- Размещение: внешняя сторона браслета с доступом воздуха
- **Энергопотребление:** сенсоры требуют нагрева (~30–50 мВт), используются периодическими измерениями

### Энергопотребление и обработка
- **MCU:** nRF52840 или аналогичный с поддержкой BLE и DSP-инструкциями
- **Rust-core:** обработка на устройстве — расчет спектра (FFT), детекция R-пиков, вычисление \( v \) и \( \chi_{Ze} \)
- **Передача:** только агрегированные индексы \( \chi_{Ze} \) по BLE, сырые данные — по запросу
- **Целевая автономность:** 24 часа с периодической работой VOC-сенсора

---

## Программная экосистема

1. **Open-source Rust/Python библиотека — `biosense-core`**  
   - Реализация расчета \( v \) и \( \chi_{Ze} \) для ЭЭГ, ВСР и VOC  
   - Скрипты валидации на открытых датасетах  
   - Примеры интеграции с AIM и Regenesis

2. **Интеграция с AIM**  
   Модуль `ze_analysis` анализирует ВСР пациентов и передает \( \chi_{Ze}^{HRV} \) в клинический протокол.  
   Пороговые значения для предболезненных состояний устанавливаются на основе валидационных когорт.

3. **Интеграция с Regenesis**  
   Снижение \( \chi_{Ze} \) по любому из каналов служит триггером для назначения anti-aging протоколов.

---

## Целевые результаты

| Результат | Статус |
|-----------|--------|
| Open-source библиотека `biosense-core` | В разработке |
| **Статья 1:** Ze index (χ_Ze) как групповой маркер нейродинамического старения | Готова, на рецензии |
| **Статья 2:** Ze-поток ВСР как биомаркер предболезненных состояний | Планируется, валидация на PhysioNet/MIMIC-III |
| **Статья 3:** Ольфакция по теории Турина + VOC диагностика | Планируется, 2027–2028 |
| Аппаратный прототип браслета | MVP (ЭЭГ+ЭКГ): Q4 2026; с e-nose: Q2 2027; Турин-сенсор: 2028 |

---

## Ключевые параметры (уточненные)

| Параметр | Значение |
|----------|---------|
| \( v^* \) | 0.45631 (теоретический инвариант, получен из модели динамических систем) |
| Ze-чувствительный диапазон (ЭЭГ) | **25–35 Гц** (абсолютный биологический инвариант) |
| Частота ресемплинга | 128 Гц |
| Период анализа для ВСР | 300 с (5 мин), перекрытие 50%, гистерезис δ = 0.1 |
| VOC-анализ | 5 замеров × 1 мин, бинаризация по калиброванным порогам |

---

## Этапы разработки (детализированные)

### Фаза 1 (Q3–Q4 2026): MVP с ЭЭГ + ЭКГ
- Сборка и тестирование прототипа браслета с ЭЭГ (лобные отведения) и ЭКГ
- Реализация Rust-core для обработки на nRF52840
- Валидация \( \chi_{Ze}^{HRV} \) на датасетах PhysioNet
- **Критерий готовности:** стабильная регистрация сигналов, расчет \( \chi_{Ze} \) в реальном времени

…<truncated 170 more lines>…
```
### `MAP.md` (4124 chars)
```md
# BioSense — Component Map

## Architecture Overview

```
BioSense Platform
│
├── Module 1: EEG [ACTIVE]
│   ├── Core library:         src/eeg_ze_processor.py
│   │   ├── ze_cheating_index()     — compute χ_Ze from binary sequence
│   │   ├── alpha_peak_ze()         — proxy method (PSD peak → v → χ_Ze)
│   │   ├── narrowband_ze()         — narrowband binarization method
│   │   ├── load_cuban_mcr()        — Cuban .mat cross-spectral loader
│   │   └── group_statistics()      — t-test, Cohen's d, CI, AUC, ANCOVA
│   │
│   ├── Dataset analyses:
│   │   ├── ze_ec_eo_analysis.py    — Zenodo 3875159: EC vs EO (1 subj)
│   │   ├── ze_lemon_analysis.py    — MPI-LEMON: broadband Ze
│   │   ├── ze_bandwise.py          — MPI-LEMON: per-band Ze
│   │   ├── ze_alpha_peak.py        — MPI-LEMON: alpha peak → χ_Ze (N=30)
│   │   ├── ze_batch_pipeline.py    — MPI-LEMON: batch download + analysis
│   │   ├── ze_dortmund_pipeline.py — Dortmund: young vs old (N=60)
│   │   └── ze_cuban_analysis.py    — Cuban: lifespan curve (N=196)
│   │
│   └── Data:
│       ├── data/cuban/             — Cuban Normative EEG (.mat)
│       ├── data/lemon/             — MPI-LEMON (.set, EC condition)
│       └── data/zenodo/            — Zenodo 3875159 (BrainVision)
│
├── Module 2: HRV [PLANNED]
│   └── ze_ecg.py (→ AIM integration point)
│       ├── RR-interval Ze velocity
│       ├── χ_Ze cardiac signal
│       └── RMSSD + autonomic profiling
│
└── Module 3: Olfaction [PLANNED]
    ├── Turin theory: tunneling electron spectroscopy
    ├── VOC sensor interface
    └── Disease fingerprint classifier
```

---

## Data Flow: EEG Module

```
Raw EEG (EDF/BrainVision/EEGLAB)
        │
        ▼
[eeg_ze_processor.py: load + resample to 128 Hz]
        │
        ├──── Proxy method ─────────────────────────────────────────────────
        │     PSD computation (Welch) → find alpha peak f_peak
        │     v_peak = 2 × f_peak / fs
        │     χ_Ze = 1 − |v_peak − v*| / max(v*, 1−v*)
        │
        └──── Narrowband Ze method ─────────────────────────────────────────
              Bandpass filter (8–12 Hz)
              Binarize: x_k = 1 if sample > median else 0
              v = N_switches / (N − 1)
              χ_Ze = 1 − |v − v*| / max(v*, 1−v*)
                        │
                        ▼
              Group statistics:
              t-test + Cohen's d + 95% CI + AUC + ANCOVA (sex-adjusted)
                        │
                        ▼
              Results: JSON + PNG → results/
```

---

## Feedback Loops

```
EEG results → KNOWLEDGE.md (validated findings accumulate)
      ↓
Paper writing (Ze.docx → peer review → publication)
      ↓
New hypotheses → new datasets → validate → loop
      ↓
AIM integration: χ_Ze patient biomarker → clinical use
```

---

## Central Nodes (highest connectivity)

1. **`eeg_ze_processor.py`** — imported by all analysis scripts; core Ze math
2. **Ze Theory (v*, χ_Ze formula)** — shared across EEG, HRV, Olfaction modules
3. **AIM `ze_ecg.py`** — bridge between BioSense and patient care system
4. **KNOWLEDGE.md** — accumulates validated results for paper writing

---

## Cross-Module Dependencies

```
BioSense EEG ←──────── Ze Theory ──────────→ BioSense HRV
                             │
                             ▼
                      ZeAnastasis (theoretical)
                             │
                             ▼
                    AIM patient HRV analysis
                             │
                             ▼
                    Regenesis protocols
```

---

## External Repository

`ze_eeg_validation/` — git submodule / separate repo (djabbat/ze-eeg-validation)
Contains the full EEG validation codebase with its own README and git history.

---

## File Placement Rules

| Category | Location |
|----------|----------|
| Python source | `src/` |
| Dataset analysis scripts | `src/` |
| Raw data (not committed) | `data/` |
| Analysis outputs (JSON/PNG) | `results/` |
| Reference papers / .docx | `Materials/` |
| Core project docs (9 files) | root |
| Launcher script | root (`biosense.sh`) |

---

_Last updated: 2026-03-28_

```
### `PARAMETERS.md` (3765 chars)
```md
# BioSense — Parameters

## Ze Theory Constants

> **Canonical convention (2026-05-07):** root `~/Desktop/LongevityCommon/PARAMETERS.md § 1`
> requires Article form (`v*_active = −0.08738`) for cross-subproject
> APIs. BioSense computes χ_Ze internally in Python form `0.45631`
> (matches Ze code); converts to Article at API boundary
> (`Article = 2 · Python − 1`).

| Parameter | Value (Python, internal) | Article equivalent | Description |
|-----------|--------------------------|-------------------:|-------------|
| v*_active | `0.45631` | `−0.08738` | Ze active-agent fixed point (canonical, root §1) |
| f_opt @ 128 Hz | `29.2 Hz` | n/a | Ze-optimal frequency at standard resample rate |
| f_opt @ 100 Hz | `22.8 Hz` | n/a | Ze-optimal frequency (Cuban dataset native rate) |
| f_opt @ 250 Hz | `57.0 Hz` | n/a | Ze-optimal frequency (MPI-LEMON native rate) |

Formula: **f_opt = v*_active × fs / 2** (using Python form, then convert
emit to Article when crossing API boundary).

---

## EEG Processing Parameters

| Parameter | Value | Rationale |
|-----------|-------|-----------|
| Default resample | 128 Hz | Standard for Ze analysis; balances resolution and cost |
| Alpha band | 8–12 Hz | Standard neurophysiology |
| Ze-sensitive band | 25–35 Hz | Around f_opt; maximum χ_Ze sensitivity |
| Beta band | 13–30 Hz | Partially overlaps Ze-sensitive range |
| Gamma band | 30–45 Hz | Highest χ_Ze values in band-wise analysis |

---

## Dataset Parameters

### Cuban Normative EEG (Zenodo 4244765)
- Channels: 19 (10–20 system)
- Native sampling rate: 100 Hz
- Format: MATLAB .mat (cross-spectral matrix, averaged reference)
- N: 198 subjects, ages 5–97

### MPI-LEMON (Babayan et al. 2019)
- Channels: 62
- Native sampling rate: 250 Hz → resample to 128 Hz
- Format: EEGLAB .set
- N: 228 subjects (30 analyzed: 15 young 20–30, 15 old 65–75)

### Dortmund Vital Study (ds005385)
- Channels: standard EEG cap
- Native sampling rate: resampled to 128 Hz
- Format: EDF/BIDS
- N: 608 total (60 analyzed: 30 young 20–30, 30 old 63–70)

### Zenodo 3875159 (Jabès et al. 2021)
- Channels: 128
- Native sampling rate: 512 Hz → resample to 128 Hz
- Format: BrainVision (.vhdr + .vmrk + .eeg)
- N: 1 subject (within-subject EC vs EO validation)

---

## Statistical Thresholds

| Metric | Threshold | Interpretation |
|--------|-----------|----------------|
| Cohen's d | < 0.2 | Negligible |
| Cohen's d | 0.2–0.5 | Small |
| Cohen's d | 0.5–0.8 | Medium |
| Cohen's d | > 0.8 | Large |
| AUC | > 0.7 | Acceptable biomarker |
| AUC | > 0.8 | Good biomarker |
| p-value | < 0.05 | Significant |

---

## Age Group Definitions (BioSense standard)

| Group | Age range | Rationale |
|-------|-----------|-----------|
| Children | 5–12 yr | Pre-adolescent brain development |
| Teens | 12–18 yr | Adolescent |
| Young adults | 18–35 yr | Peak χ_Ze range |
| Middle-aged | 35–60 yr | Post-peak decline |
| Older adults | 60–80 yr | Age-related EEG slowing |
| Oldest | 80+ yr | Extreme aging |

Ze-peak predicted age: **36.5 years** (confirmed quadratic model, Cuban dataset)

---

## HRV Parameters (Planned)

| Parameter | Standard value | Description |
|-----------|---------------|-------------|
| RMSSD | ms | Root mean square of successive RR differences |
| v (HRV Ze) | — | Ze velocity of RR binarized sequence |
| χ_Ze (HRV) | — | Cheating index of cardiac signal |
| Recording length | ≥ 5 min | Minimum for stable HRV metrics |

---

## Environment Variables

| Variable | Description |
|----------|-------------|
| ZE_CUBAN_DIR | Path to Cuban EyesClose/ folder |
| ZE_DORTMUND_DIR | Path to Dortmund BIDS root |
| ZE_LEMON_DIR | Path to MPI-LEMON preprocessed data |
| ZE_ZENODO_VHDR | Path to Zenodo 3875159 .vhdr file |

---

_Last updated: 2026-03-28_

```
### `UPGRADE.md` (393 chars)
```md
# UPGRADE.md — BioSense

Suggestions for project development from external analysis, literature, and cross-project review.

**Format:**
```
## [YYYY-MM-DD] Title
**Source:** [what triggered this]
**Status:** [ ] proposed | [✓ approved YYYY-MM-DD] | [✓✓ implemented YYYY-MM-DD]
```

---

## Pending proposals

*(No pending proposals yet — add from literature review or cross-project analysis)*

```
### `TODO.md` (8369 chars)
```md
# BioSense — Носимый браслет: EEG · HRV · Запах

_Переименовано из EEG: 2026-03-26. Проект расширен до мультисенсорной платформы._

**Модули браслета:**
- **EEG** — Ze-анализ мозговых ритмов (χ_Ze биомаркер старения/когниции)
- **HRV / RR-интервалы** — Ze-поток кардиосигнала, вегетативный профиль, предболезненные состояния
- **Запах** — молекулярная спектроскопия (теория Турина: туннельный эффект), диагностика по летучим органическим соединениям пота/выдоха

---

## Статьи к написанию (из NEEDTOWRITE.md 🔥)

- [ ] Теория Турина: рецепторы запаха как молекулярные спектрометры (туннельный эффект)
- [ ] HRV как диагностика предболезненных состояний для браслета
- [ ] ЭЭГ с Ze-анализом — мозговые ритмы как Ze-поток

---

## 📌 Правило: язык программирования по умолчанию

**Если нет явного указания на конкретный язык — писать код на Rust.**
Если другой язык объективно лучше для задачи — сначала предложить и обосновать, и только после подтверждения писать код.

---

## 📌 Правило: DeepSeek для текстовых задач

**Если задача подходит DeepSeek — использовать DeepSeek API, не делать вручную.**

| Категория | Примеры |
|-----------|---------|
| **Текст / статьи** | написать статью, раздел, введение, обсуждение |
| **Перевод** | научный, медицинский, художественный текст |
| **Рецензирование** | peer review, ответ рецензентам, cover letter |
| **Гранты / документы** | грант, питч, меморандум, резюме, абстракт |
| **Редактура** | полировка текста, стиль, академический английский |
| **Пациенты (AIM)** | объяснить диагноз, назначение, анализы — понятным языком |
| **Код** | объяснить код, docstrings, code review, тесты, SQL |
| **kSystem** | статьи лексикона на 8 языках |
| **Kartvely** | главы книги, анализ исторических источников |
| **Space** | описания упражнений на 4 языках |
| **Regenesis** | протоколы, клинические обоснования |
| **ŠamnuAzuzi** | либретто на др. языках, программные заметки |
| **Переписка** | письма инвесторам, деловые email, ответы на замечания EIC |

**Ключ:** `~/.aim_env → DEEPSEEK_API_KEY` · **Модели:** `deepseek-chat` (быстро) · `deepseek-reasoner` (сложно)


---

_Обновлено: 2026-03-21_

---

## Гипотеза Ze
χ_Ze(молодые) > χ_Ze(пожилые) — старение = EEG замедляется → v отдаляется от v* → χ_Ze падает.

**v* = 0.45631** при 128Hz → f_opt = v* × 128/2 = **29.2 Hz** (граница бета/гамма)

---

## Результаты по датасетам

### 0. Cuban Human Normative EEG (Zenodo 4244765) — ЛАЙФСПЭН ✅✅
**Датасет**: Valdés-Sosa et al. | 19ch, 100Hz | Mcr кросс-спектр | N=198 EC, возраст 5–97 лет
**Файл**: `/tmp/ze_cuban_analysis.py` | α-пик из диагонали Mcr → v → χ_Ze

| Группа | N | f_peak Hz | χ_Ze |
|--------|---|-----------|------|
| children (5-12) | 53 | 9.05±0.81 | 0.4936±0.030 |
| teens (12-18) | 41 | 9.75±0.87 | 0.5192±0.032 |
| **young (18-35)** | 31 | **10.00±0.98** | **0.5287±0.036** ← ПИКОВОЕ |
| middle (35-60) | 37 | 9.64±0.90 | 0.5153±0.033 |
| old (60-80) | 34 | 8.94±0.98 | 0.4895±0.036 |
| oldest (80+) | 2 | 8.98±0.78 | 0.4912±0.029 |

- Квадратичный пик: **36.5 лет** ✅ (предсказание: 15–40 лет)
- R² квадратичный = **0.153**
- Young vs Children: t=4.758, **p<0.0001**, **d=1.049** ✅ крупный эффект
- Young vs Old: t=5.847, **p<0.0001**, **d=1.694** ✅ очень крупный эффект
- Линейная корреляция: r=−0.116, p=0.10 (ожидаемо незначима — кривая ∩-образная)

✅ **Перевёрнутая U-кривая подтверждена**: χ_Ze растёт в детстве, пик ~35 лет, снижается при старении.

---

### 1. Zenodo 3875159 — Subject 360 (внутрисубъектная валидация)
**Файл**: `ze_ec_eo_analysis.py` | EC vs EO, BrainVision 128ch, 512→128Hz

| Условие | χ_Ze | v_mean |
|---------|------|--------|
| Eyes-Closed (EC) ×3 | 0.271 ± 0.004 | 0.058 |
| Eyes-Open  (EO) ×3  | **0.335 ± 0.022** | 0.090 |
| Δ EO−EC             | **+0.064** | — |

✅ **Ze гипотеза работает внутрисубъектно**: EO (альфа-десинхронизация) → выше χ_Ze.
Эффект стабилен во всех 3 парах EC/EO сегментов.

---

### 2. MPI-LEMON — Рesting State (межсубъектный анализ)
**Датасет**: Babayan et al. 2019, Sci Data 6:308 | 62ch, 250→128Hz | EC condition
**Файлы**: `ze_lemon_analysis.py`, `ze_bandwise.py`, `ze_alpha_peak.py`
**Субъекты**: N=30 (15 young 20-30, 15 old 65-75)

#### A. Широкополосный (broadband) анализ — N=6
- Young: χ_Ze = 0.504; Old: χ_Ze = 0.583 → **Old > Young (обратный!)**
- Объяснение: оба далеко от v*, пожилые случайно ближе по артефактам ICA

#### B. Полосовой (band-wise) анализ — N=24
| Диапазон | Young | Old | Δ (Y−O) |
|----------|-------|-----|---------|
| delta 1-4Hz | 0.231 | 0.229 | +0.002 |
| theta 4-8Hz | 0.335 | 0.339 | −0.005 |
| alpha 8-12Hz | 0.447 | 0.445 | +0.002 |
| beta 13-30Hz | 0.740 | 0.730 | +0.010 |
| **gamma 30-45Hz** | **0.828** | **0.824** | **+0.004** |

Все различия ≈ 0. Гамма — правильное направление, но без значимости.

#### C. Alpha Peak Ze (точный PSD-пик) — N=30
| | Young (N=15) | Old (N=15) |
|--|-------------|-----------|
| f_peak | 9.525 ± 1.530 Hz | 9.367 ± 1.224 Hz |
| χ_Ze | 0.4345 ± 0.044 | 0.4299 ± 0.035 |
| t-test | t=0.302, **p=0.765** | — |
| Cohen's d | 0.110 | — |
| r(age, χ_Ze) | −0.046 | p=0.810 |

**Направление верное** (Y > O), **но незначимо**.
Для 80% мощности (d=0.11) нужно N ≈ **1289/группу** — недостижимо без мегадатасета.

---

## Фундаментальный вывод

**Почему покоящийся EEG не годится для Ze-теста:**
- Альфа-пик (~10Hz) при 128Hz → v ≈ 0.156 → далеко от v* = 0.456
- Δf_peak с возрастом ~0.2Hz → Δv = 0.003 → Δχ_Ze = **0.005** — слишком мало
- ICA-преобработка сглаживает индивидуальные различия

**Где Ze теория РАБОТАЕТ (подтверждено):**
1. ✅ **EC vs EO** (Zenodo): Δχ_Ze = +0.064 — крупный детектируемый сдвиг
2. ✅ **Синтетические данные** (demo): r(age, χ_Ze) = −0.994 при правильных частотах
3. ✅ **Dortmund Vital Study** (ds005385): Young(20-30) χ_Ze=0.4492 vs Old(63-70) χ_Ze=0.4285, p=0.006, **d=0.732**, N=60
4. ✅ **Cuban Normative EEG** (Zenodo 4244765): Перевёрнутая U-кривая, пик 36.5 лет, Young vs Old **d=1.694**, N=198

**Где Ze теория должна лучше работать (не проверено):**
- **Когнитивные задачи** (рабочая память, внимание): бета/гамма активация → v ближе к v*
- **Суженный диапазон 25-35Hz** (вокруг f_opt = 29.2Hz): наибольшая чувствительность χ_Ze

---

## Файлы кода

| Файл | Назначение |
|------|-----------|
| `eeg_ze_processor.py` | Core: Ze метрики, EDF/BrainVision loader, CLI |
| `ze_ec_eo_analysis.py` | EC vs EO по маркерам (Zenodo 360) |
| `ze_lemon_analysis.py` | Broadband Ze + scatter age vs χ_Ze (LEMON) |
| `ze_bandwise.py` | Per-band Ze (delta/theta/alpha/beta/gamma) |
| `ze_alpha_peak.py` | PSD alpha peak → v → χ_Ze (точный метод) |

---

## Данные

### Zenodo 3875159 (Jabès et al. 2021)
- `data/zenodo/360.vhdr` + `.vmrk` + `360_GKstudy-Deci_Segmentation_outputBA.eeg`
- 128 каналов, 512Hz, ~8.5 мин, BrainVision

### MPI-LEMON (Babayan et al. 2019)
- `data/lemon/participants.csv` — метаданные 228 субъектов
- `data/lemon/sub-032301..032525/` — 30 субъектов распакованы (EC.set + EC.fdt)
- `data/lemon/results/` — JSON + PNG результаты
- S3: `s3://fcp-indi/.../EEG_Preprocessed_BIDS_ID/EEG_Preprocessed/sub-XXXXXX.tar.gz`

---

## Следующие шаги (приоритет)

**Высокий приоритет:**
- [x] ~~Кубинский датасет: лайфспэн кривая~~ ✅ Cuban N=198, пик 36.5 лет, d=1.694
- [x] ~~Dortmund: young vs old~~ ✅ N=60, p=0.006, d=0.732
- [ ] Написать статью: Ze.docx + EEG экспериментальная секция (есть 4 подтверждения!)
- [ ] Найти задачный EEG (task-based) датасет: рабочая память / n-back
- [ ] Проверить Ze в **узкой полосе 25-35Hz** вокруг f_opt во время когнитивной нагрузки

**Средний приоритет:**
- [ ] Топографические карты χ_Ze (mne.viz.plot_topomap) по каналам в гамма-диапазоне
- [ ] Написать статью: Ze.docx + EEG экспериментальная секция

**Открытые вопросы для статьи:**
1. Почему v* = 0.45631 (теоретический вывод Ze vs эмпирическая калибровка)?
2. Как интерпретировать χ_Ze в единицах нейрофизиологии?
3. Почему EC→EO переход (d≈0.6) намного больше young→old (d≈0.1)?

---

…<truncated 8 more lines>…
```
### `KNOWLEDGE.md` (5376 chars)
```md
# BioSense — Knowledge System

## Ze Theory: Core Definitions

### Fundamental formulas
```
Binary sequence:  x_k = 1  if  sample > median, else 0
Ze velocity:      v = N_S / (N − 1)           [N_S = number of binary switches]
Fixed point:      v* = 0.45631                 [theoretical maximum materialization]
Cheating index:   χ_Ze = 1 − |v − v*| / max(v*, 1−v*)    ∈ [0, 1]
Ze-optimal freq:  f_opt = v* × fs / 2
```

### Properties
- χ_Ze = 1 when v = v* (optimal)
- χ_Ze → 0 as v → 0 (no switching) or v → 1 (constant switching)
- At 128 Hz: f_opt = 29.2 Hz (beta/gamma boundary)
- At 100 Hz: f_opt = 22.8 Hz
- α-band proxy: v_proxy = 2 × f_peak / fs (monotonic transformation of f_peak)

---

## Validated Empirical Facts (EEG)

### Fact 1: Inverted-U lifespan curve (Cuban, N=196)
- χ_Ze peaks at ~36.5 years of age
- χ_Ze(young 18–35) = 0.5287 ± 0.036 — highest group
- χ_Ze(old 60–80) = 0.4895 ± 0.036
- Young vs Old: t=5.847, p<0.0001, d=1.694 [1.147, 2.487], AUC=0.715
- Quadratic model R² = 0.153
- f_opt prediction (Ze): ~22.8 Hz at 100 Hz sampling
- Observation: alpha peak in this dataset ~9–10 Hz (far from f_opt)

### Fact 2: Within-subject EC vs EO effect (Zenodo 3875159)
- Δχ_Ze(EO − EC) = +0.064 (large effect)
- Stable across 3 repeated EC/EO pairs
- Mechanism: EO causes alpha desynchronization → f shifts upward → v closer to v*
- This is the largest reproducible Ze effect observed in EEG

### Fact 3: Cross-sectional young vs old resting EEG (Dortmund, N=60)
- Proxy method: Young χ_Ze=0.449 vs Old χ_Ze=0.429; p=0.006, d=0.732; AUC=0.715
- Narrowband Ze: Young=0.450 vs Old=0.444; p=0.028, d=0.584
- ANCOVA (sex-adjusted): F(1,57)=4.56, p=0.037
- Sex × group interaction: p=0.442 (effect is not sex-specific)

### Fact 4: MPI-LEMON null result (N=30)
- Young (20–30): χ_Ze = 0.4345 ± 0.044; Old (65–75): χ_Ze = 0.4299 ± 0.035
- t=0.302, p=0.765, d=0.110 — NOT significant
- Required N for 80% power: ~1289/group (not achievable)
- Reason: ICA preprocessing + narrow age range + small N

---

## Theoretical Interpretation

### Why resting-state alpha is a weak Ze context
- Alpha peak at ~10 Hz → v_proxy ≈ 0.156 (at 128 Hz) — far from v*=0.456
- Δf_peak ≈ 0.2 Hz per decade → Δv ≈ 0.003 → Δχ_Ze ≈ 0.005 (tiny)
- Individual variability ≈ 1–2 Hz >> age effect

### Why EC→EO transition is a strong Ze context
- Alpha desynchronization shifts f from ~10 Hz toward ~12–15 Hz
- This moves v closer to v* (toward 29.2 Hz at 128 Hz)
- Large Δf → measurable Δχ_Ze

### Where Ze theory should work best (unvalidated)
1. Cognitive tasks (n-back, working memory): beta/gamma activation → v closer to v*
2. Narrow band 25–35 Hz (around f_opt): maximum χ_Ze sensitivity per Hz
3. Sleep stage transitions: major frequency shifts

---

## Turin Olfaction Theory (Knowledge Base)

### Core claim
Olfactory receptors function as molecular spectrometers via inelastic electron tunneling,
not via lock-and-key shape matching (classical theory).

### Mechanism
- Electron tunnels from donor to acceptor site in receptor
- Inelastic tunneling: electron loses energy = phonon emitted = vibrational mode activated
- Molecule is detected by its vibrational spectrum (like IR spectroscopy)
- Explains: why molecules with same shape but different deuterium content smell different

### Evidence for Turin theory
- Drosophila behavioral studies: deuterated compounds smell different despite same shape
- Human psychophysics: some evidence for spectral discrimination
- Counter-evidence: some exceptions not explained by vibration theory

### Relevance to BioSense
- VOC sensors could exploit vibrational fingerprints
- Disease states alter VOC composition → measurable spectral signatures
- Aging changes VOC profile (2-nonenal, dimethyl sulfide, etc.)

---

## HRV Ze Theory (Planned Knowledge)

### Hypothesis
- Heart rate variability = RR interval sequence
- Apply Ze binarization: v_HRV = N_switches / (N_RR − 1)
- χ_Ze_HRV as autonomic nervous system health index
- Pre-disease states → reduced HRV complexity → v moves away from v*

### Known HRV facts (standard)
- RMSSD: root mean square successive differences — parasympathetic marker
- SDNN: standard deviation of NN intervals — overall HRV
- HF power (0.15–0.4 Hz): vagal tone
- LF power (0.04–0.15 Hz): sympathetic + vagal
- LF/HF ratio: sympathovagal balance
- HRV decreases with age (consistent with Ze aging hypothesis)

---

## Key Datasets — Summary

| Dataset | Zenodo ID / Source | N | Age | Modality | Ze result |
|---------|---------------------|---|-----|----------|-----------|
| Cuban Normative EEG | 4244765 | 198 | 5–97 | EEG 19ch MAT | d=1.694 *** |
| Zenodo Jabès 2021 | 3875159 | 1 | — | EEG 128ch BrainVision | Δ=+0.064 |
| Dortmund Vital | ds005385/OpenNeuro | 60 | 20–70 | EEG EDF/BIDS | d=0.732 ** |
| MPI-LEMON | Babayan 2019 | 30 | 22–72 | EEG 62ch EEGLAB | d=0.110 ns |
| PhysioNet EEG-MMI | physionet.org | 109 | 20–89 | EEG EDF | Not analyzed |

---

## Key References

- **Ze Theory:** Tkemaladze J. Mol Biol Reports 2023. PMID 36583780
- **Aging biology:** Lezhava T. et al. Biogerontology 2011. PMID 20480236
- **MPI-LEMON:** Babayan A. et al. Sci Data 2019; 6:308
- **Cuban EEG:** Valdés-Sosa P.A. et al. NeuroImage 2021
- **Turin olfaction:** Turin L. Chem Senses 1996; 21(6):773–91
- **Ze EEG paper:** Tkemaladze J. [Manuscript under review, 2026]

---

_Last updated: 2026-03-28_

```
### `MEMORY.md` (4274 chars)
```md
# BioSense — Project Memory

## Decisions Made

### 2026-03-26: Project renamed EEG → BioSense
- **Decision:** Expand scope from EEG-only to multisensor platform (EEG + HRV + Olfaction)
- **Rationale:** Ze theory applies to any biosignal; wearable bracelet concept requires multiple sensors
- **Impact:** TODO.md, CLAUDE.md updated; ze_eeg_validation subfolder retained as-is

### 2026-03-24: Cuban dataset chosen as primary lifespan validation
- **Decision:** Cuban Normative EEG (Zenodo 4244765, N=198, ages 5–97) selected as strongest validation
- **Rationale:** Largest N, full lifespan, confirmed inverted-U curve, d=1.694
- **Result:** χ_Ze peak at 36.5 years confirmed; validates Ze aging hypothesis

### 2026-03: Proxy method adopted as primary Ze metric for resting EEG
- **Decision:** Use alpha peak frequency → v_proxy rather than broadband binarization
- **Rationale:** Resting EEG alpha (~10 Hz) is far from v* (0.45631); broadband Ze gives
  noisy results. Proxy method is equivalent to standard alpha peak analysis but expresses
  results in Ze units.
- **Limitation:** χ_Ze is a monotonic transformation of f_peak at fixed fs; no independent
  information beyond alpha peak frequency.

### 2026-03: Resting-state EEG identified as low-sensitivity context for Ze
- **Decision:** Shift focus to task-based EEG (cognitive load) and narrow 25–35 Hz band
- **Rationale:** At alpha peak (~10 Hz), Δχ_Ze per Hz is tiny (~0.005); for 80% power
  (d=0.11, LEMON result) need N≈1289/group — not achievable
- **Next target:** n-back / working memory datasets where beta/gamma dominate

### 2025: ze_eeg_validation/ created as separate git repo
- **Decision:** Keep EEG validation code in separate repo (djabbat/ze-eeg-validation)
- **Rationale:** Public scientific code should be independently citable and reproducible
- **Structure:** ze_eeg_validation/ is a subfolder within BioSense but has own git history

---

## Lessons Learned

### EEG analysis
1. **ICA preprocessing smooths individual differences** — preprocessed LEMON data showed
   smaller effects than Dortmund (less preprocessing). Less filtering = more Ze signal.
2. **Cross-spectral matrix (Cuban) gives cleaner f_peak** than PSD on raw EEG —
   averaging reference + Laplacian improves signal-to-noise.
3. **EC vs EO transition is the strongest Ze effect** (d≈0.6–1.0) — much larger than
   young vs old resting state (d≈0.1). Within-subject designs are more powerful.
4. **Band-specific Ze in gamma (30–45 Hz) shows correct direction** but needs larger N.
   Beta/gamma bands are where Ze theory has theoretical advantage.

### Statistical
5. **ANCOVA (sex-adjusted) is necessary** for cross-sectional age comparisons —
   sex effects on alpha frequency confound Ze group differences.
6. **AUC > 0.7 = acceptable biomarker** — Dortmund AUC=0.715 meets this threshold.
7. **Quadratic model for lifespan** — inverted-U better than linear (χ_Ze peaks ~36.5 yr).

### Project management
8. **Keep raw data OUT of git** — data/ folder should be in .gitignore
9. **Results (JSON/PNG) can be committed** — small size, useful for quick review
10. **ze_eeg_validation/ has its own git** — do not double-commit files in subdirectory

---

## History / Milestones

| Date | Milestone |
|------|-----------|
| ~2025 | EEG project started; Ze theory applied to EEG |
| 2025 | Zenodo 3875159 EC vs EO analysis — within-subject validation |
| 2025 | MPI-LEMON analysis — null result, underpowered (d=0.11) |
| 2026-03 | Dortmund ds005385 — significant result (p=0.006, d=0.732, N=60) |
| 2026-03 | Cuban Normative EEG — lifespan curve confirmed (d=1.694, N=196) |
| 2026-03 | ze_eeg_validation/ repo structured; README written; ready for submission |
| 2026-03-26 | Project renamed EEG → BioSense; scope expanded |
| 2026-03-28 | Full project initialization: 9 core files created; git push to djabbat/BioSense |

---

## Open Questions

1. Why exactly is v* = 0.45631? (theoretical derivation vs empirical calibration)
2. How to interpret χ_Ze in units of neurophysiology (Hz equivalents)?
3. Why EC→EO transition (d≈0.6–1.0) >> young→old resting (d≈0.1)?
4. Will task-based EEG (n-back, working memory) show larger Ze aging effect?
5. Does Ze theory work for HRV with the same v*?

---

_Last updated: 2026-03-28_

```
### `LINKS.md` (3437 chars)
```md
# BioSense — Ecosystem Links

## Direct Integrations

### AIM (~/Desktop/AIM/)
- **Link:** `ze_ecg.py` in AIM provides HRV analysis for patients using χ_Ze metric
- **Data flow:** BioSense Ze algorithms → AIM patient assessments
- **Shared:** Ze theory constants (v* = 0.45631), cheating index formula
- **AIM modules using Ze:** `ze_ecg.py` → patient HRV profile, RMSSD, v*-based autonomic score
- **Clinical use:** BioSense biomarkers (χ_Ze) inform integrative medicine protocols in AIM

### ZeAnastasis (~/Desktop/ZeAnastasis/)
- **Link:** Ze Theory is the mathematical foundation of both projects
- **BioSense role:** Experimental validation of Ze Theory on biological signals (EEG, HRV)
- **ZeAnastasis role:** Theoretical development of Ze framework
- **Shared:** v* constant, χ_Ze formula, Ze velocity definition
- **Publications:** Ze.docx (in BioSense/Materials/) is the primary Ze theory paper

---

## Indirect Connections

### CDATA (~/Desktop/CDATA/)
- **Link:** Statistical methods (t-test, Cohen's d, ANCOVA) overlap
- **Data:** No shared data but similar biostatistics approach
- **Publications:** May co-cite Ze theory paper from BioSense

### Regenesis (~/Desktop/Regenesis/)
- **Link:** BioSense EEG and HRV biomarkers (χ_Ze) inform anti-aging protocols
- **Future:** BioSense wearable could monitor Regenesis protocol effectiveness
- **Shared:** Aging biomarker framework

### ClinicA (~/Desktop/ClinicA/)
- **Link:** Clinical implementation of BioSense biomarkers in Dr. Tkemaladze's practice
- **Future:** BioSense wearable data integrated into patient clinical records

---

## External Resources

### Datasets
| Dataset | URL | Status |
|---------|-----|--------|
| Cuban Normative EEG | https://zenodo.org/records/4244765 | Downloaded to data/cuban/ |
| Zenodo 3875159 | https://zenodo.org/records/3875159 | In data/zenodo/ |
| MPI-LEMON | https://fcon_1000.projects.nitrc.org/indi/retro/MPI_LEMON.html | Partial (30 subj in data/lemon/) |
| Dortmund ds005385 | https://openneuro.org/datasets/ds005385 | Used in analysis |
| PhysioNet EEG-MMI | https://physionet.org/content/eegmmidb/ | Not downloaded |

### GitHub Repositories
| Repo | URL | Content |
|------|-----|---------|
| ze-eeg-validation (public) | https://github.com/djabbat/ze-eeg-validation | EEG validation codebase |
| BioSense (private) | https://github.com/djabbat/BioSense | Full project (this repo) |

### Key Literature
| Reference | Relevance |
|-----------|-----------|
| PMID 36583780 — Tkemaladze J. Mol Biol Reports 2023 | Ze Theory primary paper |
| PMID 20480236 — Lezhava T. et al. Biogerontology 2011 | Aging biology foundation |
| Babayan et al. 2019, Sci Data 6:308 | MPI-LEMON dataset paper |
| Valdés-Sosa et al. | Cuban Normative EEG dataset paper |
| Turin G. — olfactory receptor tunneling theory | Olfaction module theory |

---

## Publications Using BioSense Results

### Under review
- "Ze cheating index (χ_Ze) as a group-level index of neurodynamic aging:
  Experimental EEG validation across the human lifespan" — Tkemaladze J. (2026)

### Materials/Ze.docx
- Main Ze theory paper (located in Materials/)

---

## Notes on Ecosystem Consistency

- BioSense is listed under **AIM known projects** (CLAUDE.md Known projects table: BioSense)
- Not in AIM/Deferred/ — this is an **active** project
- AIM/NEEDTOWRITE.md contains 3 BioSense-related articles (Turin theory, HRV, EEG Ze-flow)

---

_Last updated: 2026-03-28_

```
### `backend/Cargo.toml` (757 chars)
```toml
[package]
name = "biosense-backend"
version = "0.1.0"
edition = "2021"
authors = ["Jaba Tkemaladze <jaba@longevity.ge>"]
license = "MIT"
description = "BioSense Rust backend — χ_Ze biomarker computation, MCOA bridge, exacerbation risk. Listens on 127.0.0.1:4502 (decided 2026-05-07)."

[[bin]]
name = "biosense-backend"
path = "src/main.rs"

[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
chrono = { version = "0.4", features = ["serde"] }

[dev-dependencies]
tower = { version = "0.5", features = ["util"] }
http-body-util = "0.1"
hyper = { version = "1", features = ["full"] }

```
### `src/requirements.txt` (56 chars)
```txt
mne>=1.0.0
numpy>=1.21.0
scipy>=1.7.0
matplotlib>=3.4.0

```
### code `backend/src/main.rs`
```
//! biosense-backend — Rust HTTP server для χ_Ze биомаркера.
//!
//! Endpoints (called by `biosense-web` Phoenix LiveView via BackendClient):
//!     GET  /healthz                     — liveness probe
//!     POST /chi_ze                      — body {v_eeg,v_hrv,v_resp,v_sleep}
//!                                          → {chi_ze, components}
//!     POST /bridge                      — body {d}
//!                                          → {chi_ze} (CDATA bridge stub)
//!     POST /exacerbation                — body {age,sex,chi_now,chi_7d}
//!                                          → {risk, level}
//!     GET  /api/v_star                  — return canonical v* (Article form)
//!
//! Port: 127.0.0.1:4502 (decided 2026-05-07; nginx biosense.longevity.ge
//! already maps /api/ → :4502 and /,/live/ → :4501 biosense-web).
//!
//! Reference: ~/Desktop/LongevityCommon/BioSense/CONCEPT.md.

use axum::{
    extract::Json,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

/// Canonical v*_active in **Python form** for internal computation;
/// Article form is `2·python − 1 = -0.08738` (root PARAMETERS.md § 1).
const V_STAR_ACTIVE_PY: f64 = 0.45631;
const V_STAR_ACTIVE_ARTICLE: f64 = -0.08738;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".parse().unwrap()),
        )
        .init();

    // Routes are mounted at BOTH `/<name>` (legacy short form) and
    // `/api/<name>` (matches nginx `biosense.longevity.ge.conf` /api/ →
    // :4502/api/). Phoenix biosense-web client uses /api/ prefix, the
    // dropped Docker container `deploy-biosense-backend-1` did the same.
    let app = Router::new()
        .route("/healthz",         get(healthz))
        .route("/api/healthz",     get(healthz))
        .route("/chi_ze",          post(chi_ze))
        .route("/api/chi_ze",      post(chi_ze))
        .route("/bridge",          post(bridge))
        .route("/api/bridge",      post(bridge))
        .route("/exacerbation",    post(exacerbation))
        .route("/api/exacerbation", post(exacerbation))
        .route("/api/v_star",      get(get_v_star))
        .route("/v_star",          get(get_v_star));

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(4502);
    let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let addr: SocketAddr = format!("{host}:{port}").parse().expect("HOST:PORT parse");

    let listener = tokio::net::TcpListener::bind(addr).await.expect("bind");
    tracing::info!(%addr, "biosense-backend listening");
    axum::serve(listener, app).await.expect("serve");
}

// ── handlers ─────────────────────────────────────────────────────

async fn healthz() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "ok",
        "version": env!("CARGO_PKG_VERSION"),
        "ts": chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
    }))
}

async fn get_v_star() -> impl IntoResponse {
…<truncated 255 more lines>…
```
## Code volume
| ext | files | bytes |
|---|---|---|
| .py | 8 | 96455 |
| .ex | 1 | 11932 |
| .rs | 1 | 11295 |
| .heex | 1 | 5194 |
| .exs | 1 | 744 |