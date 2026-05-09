# AUDIT PACKET — SamnuAzuzi

Path: `/home/oem/Desktop/ŠamnuAzuzi`  Date: 2026-05-08

## Size & file counts
```
334M	/home/oem/Desktop/ŠamnuAzuzi
```
**Extensions:** .md=24, .musicxml=7, .mscz=7, .midi=7, .ly=6, .pdf=4, (noext)=1
## Tree (depth=2, max 200 entries)
```
.
./SAMNU_AZUZI_v5.musicxml
./PRODUCTION_PLAN.md
./STRATEGY.md
./TODO.md
./Šamnu_Azuzi.mscz
./CLAUDE.md
./PEER_REVIEW_APPLIED_2026-04-22
./PEER_REVIEW_APPLIED_2026-04-22/00_CONSOLIDATED_DIRECTIVES.md
./PEER_REVIEW_APPLIED_2026-04-22/Act_I_revised.md
./PEER_REVIEW_APPLIED_2026-04-22/Act_III_revised.md
./PEER_REVIEW_APPLIED_2026-04-22/Act_V_revised.md
./PEER_REVIEW_APPLIED_2026-04-22/Act_II_revised.md
./PEER_REVIEW_APPLIED_2026-04-22/Act_IV_revised.md
./SAMNU_AZUZI_v5.pdf
./MUSESCORE3_SCORE_2026-04-22
./MUSESCORE3_SCORE_2026-04-22/04_Act_IV_VocalPiano.ly
./MUSESCORE3_SCORE_2026-04-22/02_Act_II_VocalPiano.ly
./MUSESCORE3_SCORE_2026-04-22/05_Act_V_VocalPiano.pdf
./MUSESCORE3_SCORE_2026-04-22/02_Act_II_VocalPiano-3.midi
./MUSESCORE3_SCORE_2026-04-22/02_Act_II_VocalPiano.pdf
./MUSESCORE3_SCORE_2026-04-22/03_Act_III_VocalPiano.ly
./MUSESCORE3_SCORE_2026-04-22/README.md
./MUSESCORE3_SCORE_2026-04-22/mscz
./MUSESCORE3_SCORE_2026-04-22/02_Act_II_VocalPiano-2.midi
./MUSESCORE3_SCORE_2026-04-22/02_Act_II_VocalPiano-1.midi
./MUSESCORE3_SCORE_2026-04-22/05_Act_V_VocalPiano.ly
./MUSESCORE3_SCORE_2026-04-22/02_Act_II_VocalPiano.midi
./MUSESCORE3_SCORE_2026-04-22/01_Act_I_VocalPiano.ly
./MUSESCORE3_SCORE_2026-04-22/INSTRUMENTS_MAPPING.md
./MUSESCORE3_SCORE_2026-04-22/musicxml
./MUSESCORE3_SCORE_2026-04-22/02_Act_II_VocalPiano-4.midi
./MUSESCORE3_SCORE_2026-04-22/02_Act_II_VocalPiano-6.midi
./MUSESCORE3_SCORE_2026-04-22/03_Act_III_VocalPiano.pdf
./MUSESCORE3_SCORE_2026-04-22/02_Act_II_VocalPiano-5.midi
./MUSESCORE3_SCORE_2026-04-22/00_Overture.ly
./Archive
./Archive/SAMNU_AZUZI_v5 (copy).musicxml
./Archive/Materials
./Archive/Libretto
./Archive/SAMNU_AZUZI_v5_score.pdf
./Archive/SAMNU_AZUZI_production_v5.pdf
./FINAL_SCORE_2026-04-22
./FINAL_SCORE_2026-04-22/01_ACT_I_SCORE_SPEC.md
./FINAL_SCORE_2026-04-22/05_ACT_V_SCORE_SPEC.md
./FINAL_SCORE_2026-04-22/03_ACT_III_SCORE_SPEC.md
./FINAL_SCORE_2026-04-22/06_DEEP_PANEL_ANALYSIS.md
./FINAL_SCORE_2026-04-22/00_OVERTURE.md
./FINAL_SCORE_2026-04-22/02_ACT_II_SCORE_SPEC.md
./FINAL_SCORE_2026-04-22/04_ACT_IV_SCORE_SPEC.md
./PEER_REVIEWS_2026-04-22
./PEER_REVIEWS_2026-04-22/repetition_analyst_review.md
./PEER_REVIEWS_2026-04-22/librettist_review.md
./PEER_REVIEWS_2026-04-22/historian_review.md
./PEER_REVIEWS_2026-04-22/music_director_review.md
./PEER_REVIEWS_2026-04-22/dramaturg_review.md
```
## Detected stack: **unknown**
## Core files

### `CLAUDE.md` (509 chars)
```md
# Šamnu Azuzi — Опера по Гильгамешу

## 📌 Правило: DeepSeek для нетехнических задач

**Код — Claude. Всё остальное — DeepSeek API.**
Примеры: либретто на других языках, программные заметки, переводы текстов оперы.
**Ключ:** `~/.aim_env → DEEPSEEK_API_KEY` · **Вход:** `~/Desktop/AIM/llm.py`
**Модели:** `deepseek-chat` (быстро) · `deepseek-reasoner` (художественный перевод)

---

## Проект

Опера в 5 актах по эпосу о Гильгамеше. ~3ч 15мин · 17 сцен · грузинская полифония.
Статус: ✅ Завершена (2026-03-18).

```
### `MUSESCORE3_SCORE_2026-04-22/README.md` (3965 chars)
```md
# ŠAMNU AZUZI — Пакет для MuseScore 3 (2026-04-22)

Компилятивный эскиз партитуры, сгенерированный автоматически по 26 директивам peer review.

## 📁 Структура

```
MUSESCORE3_SCORE_2026-04-22/
├── INSTRUMENTS_MAPPING.md          ← Грузинские ↔ MuseScore 3 эквиваленты
├── README.md                        ← этот файл
├── 00_Overture.ly                   ← LilyPond-источник (для справки/компиляции)
├── 01_Act_I_VocalPiano.ly
├── 02_Act_II_VocalPiano.ly
├── 03_Act_III_VocalPiano.ly
├── 04_Act_IV_VocalPiano.ly
├── 05_Act_V_VocalPiano.ly
├── musicxml/
│   ├── 00_Overture_SKELETON.musicxml
│   ├── 01_Act_I_VocalPiano_SKELETON.musicxml
│   └── ... (5 актов)
└── mscz/                            ← ← ← ОТКРЫВАЙ ЭТИ
    ├── 00_Overture_SKELETON.mscz    ← двойной клик → MuseScore 3
    ├── 01_Act_I_VocalPiano_SKELETON.mscz
    └── ... (5 актов)
```

## 🎼 Что содержат `.mscz` скелеты

**Overture** (`00_Overture_SKELETON.mscz`):
- 31 инструмент: пикколо, 2 флейты, 2 гобоя, англ. рожок, 2 кларнета B♭, бас-кларнет, 2 фагота, контрафагот, 4 валторны, 3 трубы, 3 тромбона, туба, литавры, арфа, челеста, струнные 5 групп
- 120 тактов, разделены на 8 разделов:
  - mm.1-8 Largo misterioso (4/4, G minor) — Ночь
  - mm.9-24 Andante agitato — Сгущение
  - mm.25-40 Allegro selvaggio (7/8, A Phrygian) — Появление Дикого
  - mm.41-56 Andante dolce (6/8, A♭ major) — Вход Любви (Шамхат)
  - mm.57-72 Poco più mosso (5/4) — Трио-канон
  - mm.73-88 Adagio ombroso — Тень Смерти
  - mm.89-104 Poco a poco animando (3/4) — Восхождение к Цветку
  - mm.105-120 Attacca a Scena 1 — Переход в Акт I

**Акты** (`0N_Act_N_VocalPiano_SKELETON.mscz`):
- 5 вокальных голосов: Гильгамеш (баритон), Энкиду (тенор), Шамхат (сопрано), Нинсун (меццо), хор SATB
- Фортепиано grand staff (правая + левая рука) — оркестровая редукция
- По 8-10 нумерованных пьес в акте (Rehearsal marks №1, №2, ...), каждая по 16 тактов
- Темпы/размеры/тональности по умолчанию — композитор перенастраивает

## 🚀 Открыть

Двойной клик на любой `.mscz` в проводнике — откроется в MuseScore 3.

Либо из терминала:
```
mscore3 mscz/00_Overture_SKELETON.mscz
```

## ⚠️ Это СКЕЛЕТ, не партитура

Такты пустые (whole rest). Композитор вводит ноты поверх готовой разметки:
- Инструменты правильно сконфигурированы (транспозиции, ключи)
- Темпы, размеры, тональности, rehearsal marks выставлены по спецификации
- Текстуры/лейтмотивы описаны в `FINAL_SCORE_2026-04-22/00_OVERTURE.md` и `01-05_ACT_N_SCORE_SPEC.md`

## 🎵 Следующий шаг — полная партитура

Для получения партитуры с реальными нотами нужно:

**Опция А** — ввод в MuseScore 3 вручную по спецификации (24 KB текста описания увертюры в `FINAL_SCORE_2026-04-22/00_OVERTURE.md`, аналогично для каждого акта).

**Опция Б** — установить LilyPond и скомпилировать `.ly` в PDF:
```
sudo apt install lilypond
lilypond 00_Overture.ly   # → 00_Overture.pdf + .midi
```
Затем открыть `.midi` в MuseScore 3 для получения нот в редакторе.

**Опция В** — найти профессионального оркестратора и передать ему:
- Спецификацию партитуры (`FINAL_SCORE_2026-04-22/00_OVERTURE.md` и 5 актов)
- Скелет MuseScore 3 (`mscz/`)
- Ревизованное либретто (`PEER_REVIEW_APPLIED_2026-04-22/Act_*_revised.md`)
- Директивы (`PEER_REVIEW_APPLIED_2026-04-22/00_CONSOLIDATED_DIRECTIVES.md`)

## 🔤 Грузинские тексты

MuseScore 3 требует грузинский шрифт для отображения либретто:
- **BPG Nino Mtavruli** (предпочтительно, академический стандарт)
- **Sylfaen** (Windows default)
- **Noto Serif Georgian** (Google, кроссплатформенный)

Установить: Format → Style → Text Styles → Lyrics → Font → выбрать грузинский шрифт. Сохранить стиль как `ŠamnuAzuzi_Style.mss` и переиспользовать.

## 📝 Замечания

- Гармонический/контрапунктический материал в скелетах минимален (только структурная разметка)
- Лейтмотивы прописаны на уровне мотивного каталога в спецификации, а не в нотах
- Редукция фортепиано — заглушка; финальная оркестровка делается композитором по готовым вокальным линиям и спецификации

```
### `TODO.md` (6605 chars)
```md
# Šamnu Azuzi — TODO

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

_Обновлено: 2026-03-18_

> **«Šamnu Azuzi»** — опера в 5 актах по эпосу о Гильгамеше.
> Музыка и либретто: Джаба Ткемаладзе.
> ~3ч 15мин · 17 сцен · грузинская полифония.

---

## СТАТУС

| Компонент | Статус | Файл |
|-----------|--------|------|
| Структура оперы | ✅ Готово | `SCORE.md` |
| Либретто (все 5 актов) | ✅ Готово | `Libretto/Act_I–V.md` |
| Либретто (English) | ✅ Готово | `Libretto/SAMNU_AZUZI_English_Libretto.md` |
| Тематический каталог (T1–T20) | ✅ Готово | `HARMONY.md` |
| MIDI-наброски тем | ✅ Готово | `Score/T01–T20_*.mid` |
| Партитура (полная) | ✅ Готово | `Score/SAMNU_AZUZI_v3.musicxml` (34 MB) |
| Увертюра | ✅ Готово | `Score/SAMNU_AZUZI_Overture.musicxml` (1.75 MB) |
| Трио Гильгамеша (6 песен) | ✅ Готово | `Score/SAMNU_AZUZI_KrimanchuliTrio.musicxml` (1.15 MB) |
| **Полная партитура (Увертюра+Опера)** | ✅ Готово | `Score/SAMNU_AZUZI_Complete.musicxml` (37 MB) |
| Материалы — народные мелодии | ✅ Готово | `Materials/songs/` |
| **Партитура v5 (доработанная драматургия)** | ✅ Готово 2026-03-25 | `Score/SAMNU_AZUZI_v5.musicxml` (31 MB) |
| **Производственный гид v5 (режиссура + костюмы + ТА-трио)** | ✅ Готово 2026-03-25 | `Opera/SAMNU_AZUZI_production_v5.pdf` (157 KB) |
| **Финальный PDF v5 (партитура + гид, 791 стр.)** | ✅ Готово 2026-03-25 | `Opera/SAMNU_AZUZI_v5.pdf` (5.4 MB) |

---

## ОТКРЫТЬ В MUSESCORE

```bash
musescore3 'Score/SAMNU_AZUZI_Complete.musicxml'   # Полная партитура
musescore3 'Score/SAMNU_AZUZI_Overture.musicxml'   # Только увертюра
musescore3 'Score/SAMNU_AZUZI_KrimanchuliTrio.musicxml'  # Трио Гильгамеша
```

---

## ЧТО ОСТАЛОСЬ СДЕЛАТЬ

### P0 — Внешнее сотрудничество (критично)

- [ ] **Связаться с ансамблем «Рустави»** или «Мгзавреби» — консультация по кримачули
  - Вопросы: орнаментика, диапазон мкримани, запись a cappella в операх
  - Контакт: Тбилисская государственная капелла «Рустави»
- [ ] **Музыковед ТГУ** — кафедра этномузыкологии Тбилисской консерватории
  - Проверить аутентичность применения гурийской, сванской, мегрельской, рачинской техники
- [ ] **Либреттист** — грузинский поэт для полировки текста сцен (шумерско-грузинская метрика)

### P1 — Редакция партитуры

- [ ] **Проверить в MuseScore** каждый акт:
  - Правильность ключей (скрипичный / альтовый / басовый)
  - Диапазоны голосов (не выходить за рамки)
  - Динамические обозначения (ppp → fff)
  - Темповые переходы между сценами
- [ ] **Кримачули** — проверить вокальный диапазон мкримани:
  - Нормальный: `a¹–e³`; крайний фальцет до `g³`
  - Пометить `"falsetto"` / `"krimanchuli ornament"` в партитуре
- [ ] **Текст песен** трио — проверить слоговое соответствие нотам (syllabification)

### P2 — Запись и постановка

- [ ] **Демо-запись** — MIDI экспорт из MuseScore + базовый микс
  ```bash
  musescore3 --export-to demo.mp3 Score/SAMNU_AZUZI_Complete.musicxml
  ```
- [ ] **Связаться с Тбилисской оперой** (`opera.ge`) — интерес к постановке
- [ ] **Грант** — Culture Fund of Georgia / Европейские программы поддержки оперы

### P3 — Публикация

- [ ] Опубликовать либретто в **Longevity Horizon** (OJS) как литературное приложение к статье
- [ ] GitHub-репозиторий: `djabbat/SamnuAzuzi` — партитура + либретто
- [ ] Статья: «Синтез грузинской полифонии и шумерского эпоса в современной опере»

---

## СТРУКТУРА ПАРТИТУРЫ

### Увертюра (180 тактов, ~9 мин)
| Раздел | Такты | Тема | Тональность |
|--------|-------|------|-------------|
| Урук | 1–20 | Контрабасы + там-там | g-moll, Largo |
| Гильгамеш | 21–50 | T1 Цинцкаро | D-dur, Allegro moderato |
| Энкиду/Природа | 51–80 | T4 Зар (флейта соло) | a-moll, Andante |
| Конфликт | 81–120 | T1 vs T4 + T19 Хумбаба | хроматика, Agitato |
| Реприза | 121–150 | T1 (торжество с тревогой) | D-dur, Maestoso |
| Нинсун | 151–180 | T7 Нана (чунири соло) | d-moll→A, Lento |

### Опера (1573 такта, ~210 мин)
| Акт | Такты | Сцены | Ключевые темы |
|-----|-------|-------|---------------|
| I — Урук | 238 | 1–3 | T16 Бери-Берикаоба, T1 Трио, T7 Нана |
| II — Встреча | 335 | 4–7 | T12 Мтиулури, T13 Гандагана, T4/T5/T6 Зар |
| III — Лес | 245 | 8–10 | T10 Самкураo, T15 Хасанбегура, T19 Хумбаба |
| IV — Суд | 274 | 11–13 | T20 Чакрулo, T17 Мравалжамиер, T2 (распад Трио) |
| V — Поиск | 481 | 14–17 | T2 (атональный распад), T3 Алило (финал) |

### Партии трио Гильгамеша (208 тактов)
| Песня | Такты | Метр | Тональность | Характер |
|-------|-------|------|-------------|----------|
| Mravalzhamier | 1–32 | 4/4 | D Dorian | Торжественный тост |
| Khasanbegura | 33–64 | 6/8 | g-moll | Боевой |
| Tsintskaro | 65–96 | 5/8 | D-dur | Сновидение (бани молчит) |
| Beri Berikaoba | 97–128 | 3/4 | g-moll | Ритуальная насмешка |
| Mze Shina | 129–168 | 4/4 | d-moll→Bb | Разлад (смерть Энкиду) |
| Alilo | 169–208 | 4/4 | D→Mixolydian | Финальная гармония |

---

## МАТЕРИАЛЫ

| Папка | Содержание |
|-------|-----------|
| `Materials/songs/gurian_krimanchuli.md` | 6 гурийских песен для Трио |
| `Materials/songs/megrelian.md` | 5 мегрельских для Нинсун |
| `Materials/songs/svan.md` | 6 сванских для Энкиду |
| `Materials/songs/rachian.md` | 6 рачинских для Шамхат |
| `Materials/analysis_article.md` | Философский анализ, 5 актов |
| `HARMONY.md` | Ноты тем T1–T20, характеристики |

```