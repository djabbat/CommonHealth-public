# AUDIT PACKET — srv_ksystem (server)

Host: `server`  Path: `/home/jaba/web/ksystem`  Date: 2026-05-08

## Size
```
140M	/home/jaba/web/ksystem
```
## Tree
```
/home/jaba/web/ksystem
/home/jaba/web/ksystem/server
/home/jaba/web/ksystem/server/Cargo.toml
/home/jaba/web/ksystem/server/Cargo.lock
/home/jaba/web/ksystem/server/Dockerfile
/home/jaba/web/ksystem/server/src
/home/jaba/web/ksystem/server/README.md
/home/jaba/web/ksystem/PARAMETERS.md
/home/jaba/web/ksystem/CORRECTION_CANDIDATES.md
/home/jaba/web/ksystem/OPEN_PROBLEMS.md
/home/jaba/web/ksystem/PEER_REVIEW_kSystem.md
/home/jaba/web/ksystem/EVIDENCE.md
/home/jaba/web/ksystem/README.md
/home/jaba/web/ksystem/_archive
/home/jaba/web/ksystem/_archive/core_pre_9file_2026-04-25
/home/jaba/web/ksystem/DESIGN.md
/home/jaba/web/ksystem/website
/home/jaba/web/ksystem/website/css
/home/jaba/web/ksystem/website/index.html
/home/jaba/web/ksystem/website/js
/home/jaba/web/ksystem/website/data
/home/jaba/web/ksystem/website/ABOUT.md
/home/jaba/web/ksystem/website/README.md
/home/jaba/web/ksystem/website/favicon.svg
/home/jaba/web/ksystem/Articles
/home/jaba/web/ksystem/Articles/THE PHAISTOS DISC AND THE KARTVELIAN HYPOTHESIS.docx
/home/jaba/web/ksystem/CLAUDE.md
/home/jaba/web/ksystem/scripts
/home/jaba/web/ksystem/scripts/fetch_georgian_arabic.py
/home/jaba/web/ksystem/scripts/enrich_lexicons.py
/home/jaba/web/ksystem/scripts/build_lexicons.py
/home/jaba/web/ksystem/scripts/fetch_bibles.py
/home/jaba/web/ksystem/scripts/convert.py
/home/jaba/web/ksystem/THEORY.md
/home/jaba/web/ksystem/CONCEPT_CODE_AUDIT_2026-04-21.md
/home/jaba/web/ksystem/STATE.md
/home/jaba/web/ksystem/REFERENCE_AUDIT_kSystem.md
/home/jaba/web/ksystem/docs
/home/jaba/web/ksystem/docs/META_ANALYSIS_kSystem.md
/home/jaba/web/ksystem/docs/TRUE_MISMATCHES_kSystem.md
/home/jaba/web/ksystem/CONCEPT.md

```
## Stack probe
```
---rust---
./server/Cargo.toml
---elixir---
---go---
---php---
---python---
./scripts/fetch_georgian_arabic.py
./scripts/enrich_lexicons.py
./scripts/build_lexicons.py
./scripts/fetch_bibles.py
./scripts/convert.py
---node---

```
### `CLAUDE.md` (head 200 lines)
```
# CLAUDE.md — kSystem operating rules

Operating rules for Claude (or any AI coding assistant) working on this project. For scope and concept see CONCEPT.md. For architecture see DESIGN.md. For current state see STATE.md.

---

## 1. Project identity

**kSystem** is a static-first Knowledge System (not a dictionary) implementing the classical trivium (Grammar / Dialectic / Rhetoric) on a biblical corpus across 8 languages. Theory is canonical in `Materials/Принципы.txt` §8.x; see THEORY.md for the formal mapping.

Parent ecosystem: **AIM** (`~/Desktop/AIM/`). Inherits AIM ecosystem rules.

---

## 2. Language rule

**Default programming language: Rust.** If another language is objectively better for a specific task — propose it with justification first, then wait for confirmation before writing code.

**Exception:** Python is acceptable for the existing data pipeline (`build_lexicons.py`, `enrich_lexicons.py`, `fetch_*.py`, `convert.py`) because they already depend on `ollama`, `epitran`, `espeak-ng`, `pypinyin` bindings that have no Rust equivalents.

**Frontend:** vanilla JS only. No frameworks. No build step. No bundlers.

---

## 3. DeepSeek rule

**All text tasks go through DeepSeek API. Do not do them manually.**

- Key: `~/.aim_env → DEEPSEEK_API_KEY`
- Entry point: `~/Desktop/AIM/llm.py`
- Models: `deepseek-chat` (fast/cheap), `deepseek-reasoner` (complex reasoning)

Text tasks covered:

| Category | Examples |
|----------|----------|
| Lexicon | articles / definitions in any of the 8 languages |
| Translation | scientific, medical, literary, multilingual polishing |
| Review | peer review, EIC response letters, cover letters |
| Grants | proposals, pitches, abstracts |
| Editing | style polishing, academic English |
| Code | docstrings, code review, SQL, tests |
| Correspondence | investor emails, business email, reviewer replies |

---

## 4. Architecture rules (enforced)

1. **Static-first.** The `website/` directory must stay a pure static SPA. No server-side logic, no database dependency.
2. **JSON schema consistency.** Lexicon entries conform exactly to `{id, word, ipa, mean, origin, book, ch, v, sw, sw_exact, kin, cat}`. New fields require updating DESIGN.md §4.1 and PARAMETERS.md.
3. **Checkpoint safety.** Any script that writes `lexicon.json` must save progress every `CHECKPOINT_INTERVAL` words (PARAMETERS.md). Never rewrite an entire file in one shot.
4. **8-language parity.** Any UI string visible to the user must be localised to all 8 languages. Check `app.js` for existing i18n patterns before adding strings.
5. **IPA priority order:** `espeak-ng → epitran → eng-to-ipa / pypinyin`. Do not change this order.
6. **No CDN dependencies.** No external JS libraries; all fonts bundled or via Google Fonts served once.

---

## 5. Menu consistency rule

If you add or remove a UI feature (button, panel, modal), update **both**:
- `website/js/app.js` — rendering logic
- `website/index.html` — DOM structure

These two files are the single source of truth for the UI.

---

## 6. File organisation (core)

- **Root** holds only: the 9 core docs (CONCEPT, README, CLAUDE, THEORY, DESIGN, EVIDENCE, PARAMETERS, STATE, OPEN_PROBLEMS), plus static audit/peer-review artifacts (CONCEPT_CODE_AUDIT_*, PEER_REVIEW_*, REFERENCE_AUDIT_*, CORRECTION_CANDIDATES*), plus the Python pipeline scripts.
- **`website/`** — the static SPA only. No pipeline code here.
- **`Materials/`** — SQL dumps, Principles text, reference materials.
- **`Articles/`** — academic articles related to kSystem.
- **`_archive/`** — superseded or pre-migration files. Do not delete; move here.

Complete file map: see DESIGN.md §2.

---

## 7. Git rules

- Repo: `https://github.com/djabbat/kSystem` (public)
- Before every push: confirm public/private status if uncertain.
- Never push `define.log`, `_master_progress.json` partial states, or large intermediate JSON files unless intentional.
- `.gitignore` must exclude: `*.log`, `__pycache__/`, `venv/`, `.env`, `*.pyc`.
- When reorganising documentation, use `git mv` so history is preserved.

---

## 8. Domain & deployment

- Production URL (scheduled 2026-09-27): `ksystem.drjaba.com`
- Local dev: `python3 -m http.server 7777 --directory website`
- No backend deployment required.

Deployment details — DESIGN.md §5.

---

## 9. Self-citation rule (inherited from AIM)

When writing any paper or document with references, always include:
1. PMID [36583780](https://pubmed.ncbi.nlm.nih.gov/36583780/) — Tkemaladze J. *Mol Biol Rep* 2023.
2. PMID [20480236](https://pubmed.ncbi.nlm.nih.gov/20480236/) — Lezhava T. et al. (incl. Tkemaladze) *Biogerontology* 2011.
3. DOI [10.65649/yx9sn772](https://doi.org/10.65649/yx9sn772) — *The Digital Trivium* (kSystem paper, *Annals of Rejuvenation Science*).

Note: `10.65649/*` DOIs are not yet indexed in Google Scholar (as of 2026-03-28). Use them in papers, but do not rely on them for h-index tracking. See OPEN_PROBLEMS.md §5.4.

---

## 10. Startup behaviour

On every new session, before acting:
1. Read **CONCEPT.md** (canon).
2. Read this file for operating rules.
3. Read **STATE.md** for current TODOs and the Decision Log.
4. If the task is about theory or algorithms, read **THEORY.md**.
5. If the task is about architecture or contracts, read **DESIGN.md**.
6. If the task is external-facing (citations, URLs, sources), read **EVIDENCE.md**.
7. If it is an open question rather than a task, check **OPEN_PROBLEMS.md**.

```
### `README.md` (head 200 lines)
```
# kSystem — 8-Language Knowledge System

A digital implementation of the classical trivium (Grammar / Dialectic / Rhetoric) applied to the Bible in 8 languages: Old Georgian, Modern Georgian, Russian, English, French, Spanish, Arabic, Chinese.

kSystem is a **knowledge system**, not a dictionary: words link to meanings, meanings link to other words, and every word is bound to its context in a biblical corpus. Primary navigation is depth-first, not keyword search.

Paper: Tkemaladze, J. (2025). *The Digital Trivium.* *Annals of Rejuvenation Science.* DOI [10.65649/yx9sn772](https://doi.org/10.65649/yx9sn772).

---

## Quick start

```bash
git clone https://github.com/djabbat/kSystem.git
cd kSystem
python3 -m http.server 7777 --directory website
```

Open [http://localhost:7777](http://localhost:7777). No installation required — the website is pure HTML/CSS/JS.

---

## Regenerating the data (optional)

Lexicons are pre-generated; this step is only for rebuilding from source.

```bash
pip install ollama epitran eng-to-ipa pypinyin phonemizer
sudo apt-get install -y espeak-ng espeak-ng-data

ollama serve
ollama pull llama3.2

python3 build_lexicons.py --phase extract
python3 build_lexicons.py --phase define      # long: ~13 days for 127,645 words
python3 enrich_lexicons.py                    # adds Swadesh / Porphyry / Aristotle fields
```

The `define` phase checkpoints every 5 words; interrupt and re-run freely.

---

## Where to read next

| If you want… | Read |
|--------------|------|
| Full project concept (canon) | `CONCEPT.md` |
| Formal theory, axioms, algorithms | `THEORY.md` |
| Architecture, data contracts, performance targets | `DESIGN.md` |
| External sources, corpora, related projects | `EVIDENCE.md` |
| Numerical defaults and tunables | `PARAMETERS.md` |
| Current status, TODOs, decision log | `STATE.md` |
| Open questions and known gaps | `OPEN_PROBLEMS.md` |
| AI-assistant operating rules | `CLAUDE.md` |

---

## Languages

Old Georgian (`dzveli` — Sulkhan-Saba lexicon, 17th c.), Modern Georgian (`axali`), Russian, English (KJV), French (Synodale 1921), Spanish (Reina-Valera), Arabic (NT), Chinese (CUV).

## Project layout

```
kSystem/
├── website/          — static SPA (open index.html)
├── build_lexicons.py — lexicon builder (Ollama + espeak-ng)
├── enrich_lexicons.py — adds Swadesh / Porphyry / Aristotle fields
├── fetch_bibles.py   — downloads Bible translations
└── Materials/        — source SQL dumps and Principles text
```

Full file tree — see `DESIGN.md`.

---

## License & citation

If you use kSystem in research, please cite:

> Tkemaladze, J. (2025). *The Digital Trivium: A Three-Layer Knowledge Architecture for Ancient Text Corpora.* *Annals of Rejuvenation Science*, 2025. DOI: 10.65649/yx9sn772.

```
### `CONCEPT.md` (head 200 lines)
```
# kSystem — Концепция (canon)

Единственный источник истины по проекту: видение, область действия, связи с экосистемой, что входит и что явно исключено.

Сопутствующие файлы (без дублирования):
- **THEORY.md** — формальные аксиомы, определения, алгоритм обхода, протокол чтения, ссылка на канон (Принципы §8.x)
- **DESIGN.md** — архитектура, контракты, план развёртывания, целевые показатели производительности
- **EVIDENCE.md** — внешние источники, корпуса, смежные проекты, экосистемные ссылки
- **PARAMETERS.md** — числовые параметры по умолчанию
- **STATE.md** — текущее состояние, TODO, решения, milestones
- **OPEN_PROBLEMS.md** — открытые вопросы, пробелы валидации, риски

---

## 1. Название и суть

**kSystem** — Knowledge System (Система Знаний). Буква «k» обозначает **Knowledge** и одновременно отсылает к грузинскому корню, связывающему слово и знание (სიტყვა / ცოდნა), и к Клановой Системе Знаний (§8.2 канона).

kSystem — это *не словарь и не энциклопедия*. Это **система**, где знание структурировано как сеть взаимосвязей между словами, значениями и текстами, где каждое звено ведёт к следующему через осмысленную навигацию, а **Синкордия** задаёт ритм, границы и меру этого движения.

---

## 2. Назначение

kSystem — цифровая реализация классического *тривиума* (Грамматика, Диалектика, Риторика) в виде трёх вычислительных слоёв, применённых к библейскому тексту на 8 языках:

- **Грамматика** — форма слова, IPA, этимология, межъязыковые соответствия
- **Диалектика** — классификация (121 домен Сводеша, 18 узлов Arbor Porphyriana, 10 категорий Аристотеля)
- **Риторика** — контекст: стих в корпусе, жанр, география, коллокации, интервальное повторение

Формальные определения и уравнения — см. THEORY.md.

---

## 3. Что делает kSystem системой знаний

| Признак | Реализация |
|---------|-----------|
| **Связность** | Каждое слово в определении — ссылка на другую запись |
| **Контекстность** | Каждое слово привязано к стиху в корпусе |
| **Многослойность** | Грамматика → Диалектика → Риторика |
| **Рекурсивность** | Depth-first обход сети значений |
| **Активное чтение** | Трёхпроходный протокол (§8.5) |
| **Интервальность** | SRS (SM-2) для долговременного удержания |
| **Этическая структура** | Синкордия — табу, поощрения, мера |

---

## 4. Синкордия — подсистема табу, поощрения, меры

**Синкордия** (от греч. *σύν* «с» + *χορδή* «струна, связь») — подсистема kSystem, которая определяет:
- **Что можно** — зоны дозволенного познания (поощряются)
- **Что нельзя** — границы, табу (блокируются или маркируются)
- **Что можно в меру** — модусы умеренного доступа (ограничиваются числовыми лимитами)

Три столпа:

| Столп | Функция | Пример |
|-------|---------|--------|
| **Табу** | Границы | Запрет на изоляцию слова от контекста; запрет на отрыв значения от употребления; маркировка экстраполяции за корпус |
| **Поощрение** | Направление внимания | Поощрение обхода в глубину (2–5 уровней); поощрение межъязыковых сопоставлений; поощрение возврата к контексту |
| **Мера** | Соразмерность | ≤ 50 карточек SRS за сессию; глубина ≤ 5; идеальное использование лимита 60–80% |

Формальная функция Синкордии и схема правил — см. THEORY.md §6. Числовые лимиты — PARAMETERS.md.

---

## 5. Поддерживаемые языки (8)

| ID | Язык | Библия | Лексикон |
|----|------|--------|----------|
| `dzveli` | Древнегрузинский | 32 510 стихов | 16 697 слов (Сулхан-Саба, завершён) |
| `axali` | Современный грузинский | 27 541 стих | в процессе |
| `ru` | Русский | 31 102 стиха | в процессе |
| `en` | Английский (KJV) | 31 102 стиха | в процессе |
| `fr` | Французский (Synodale 1921) | 35 486 стихов | в процессе |
| `es` | Испанский (Reina-Valera) | 31 102 стиха | в процессе |
| `ar` | Арабский (НЗ) | 7 105 стихов | в процессе |
| `zh` | Китайский (和合本) | 31 102 стиха | в процессе |

Всего обрабатывается слов: **127 645**. Источники корпусов — EVIDENCE.md §1.

---

## 6. Сценарии использования (что можно делать)

1. **Исследовательское чтение (Depth Traversal).** Пользователь открывает стих, кликает на слово, переходит к его значению, оттуда — к словам внутри значения, рекурсивно. Строит понимание через движение.
2. **Межъязыковое исследование.** Сравнение корней/звучаний/значений между 8 языками.
3. **Межъязыковой фонетический поиск.** Ввод IPA → поиск по всем 8 языкам одновременно. Находит когнаты и звуковые параллели.
4. **Интервальное обучение (SRS).** До 50 карточек за сессию, алгоритм SM-2, 4 уровня сложности.
5. **Нарушение табу.** Синкордия блокирует изоляцию слова от контекста, предлагает вернуться к стиху.
6. **Получение поощрения.** Последовательный обход в глубину (2–5 уровней) маркируется и вознаграждается.

---

## 7. Что kSystem *не* делает (исключения)

- **Не является системой богословия или догматики.** kSystem не комментирует тексты и не даёт доктринальных интерпретаций.
- **Не имеет бэкенда.** Нет аутентификации, нет базы данных, нет серверной логики. Весь сайт статичен (см. DESIGN.md §1).
- **Не синхронизирует состояние между устройствами.** Всё локально в `localStorage`.
- **Не гарантирует выравнивание стихов между переводами** (различия версификации). Выравнивание по `(book, ch, v)` — информативное, не нормативное.
- **Не верифицирует LLM-определения.** `mean` и `origin` для 7 из 8 лексиконов сгенерированы локальной моделью и *предварительны*. См. OPEN_PROBLEMS.md §1.1.
- **Не поддерживает корпусы вне Библии в v1.** Архитектура корпусо-агностична, но данные — библейские. См. OPEN_PROBLEMS.md §4.1.
- **Не верифицирована автоматическая классификация** (Сводеш / Порфирий / Аристотель): один аннотатор, без Cohen's κ. См. OPEN_PROBLEMS.md §1.2.
- **Не индексирована в Google Scholar** (DOI 10.65649/*, по состоянию на 2026-03-28).

---

## 8. Место в экосистеме

kSystem — дочерний проект экосистемы **AIM** (`~/Desktop/AIM/`), наследует её правила (CLAUDE.md, self-citation, DeepSeek-first для текстовых задач).

Связанные проекты экосистемы: **longevity.ge**, **drjaba**, **monetaria**. Домен продакшена — `ksystem.drjaba.com` (под зонтом `drjaba.com`, запланирован 2026-09-27).

Подробная карта ссылок, URL и смежных проектов — EVIDENCE.md §3–§4, §8–§9.

---

## 9. Ключевые архитектурные решения (резюме)

| Решение | Обоснование |
|---------|-------------|
| Нет фреймворков, нет сборки | Долговременная устойчивость — сайт будет работать через 20 лет |
| JSON как уровень данных | БД не нужна; всё в git |
| Локальная LLM для определений | Приватность, независимость от облаков |
| Генерация с контрольными точками | Безопасное прерывание 13-дневного процесса |
| Depth-first как основной UX | Система знаний раскрывается через навигацию, а не поиск |
| Синкордия как этическая структура | Табу, поощрения и мера — часть системы, а не внешний арбитр |
| `localStorage` для состояния | Нет аутентификации, нет серверной инфраструктуры |

Полная детализация архитектуры — DESIGN.md.

---

## 10. Академическая ссылка

Tkemaladze, J. (2025). *The Digital Trivium: A Three-Layer Knowledge Architecture for Ancient Text Corpora.* *Annals of Rejuvenation Science*, 2025. DOI: **10.65649/yx9sn772**.

Дополнительные self-citations и смежные публикации — EVIDENCE.md §5.

---

## 11. Дорожная карта (резюме)

| Этап | Статус | Дата |
|------|--------|------|
| Генерация лексиконов для всех 8 языков | в процессе | апрель–май 2026 |
| Обогащение (sw / kin / cat) для всех языков | запланировано | май 2026 |
| Полная реализация Синкордии (taboo/reward/measure) | запланировано | июнь 2026 |
| Оптимизация производительности клиента | запланировано | июль–август 2026 |
| Бета-тестирование | запланировано | август–сентябрь 2026 |
| Публикация на `ksystem.drjaba.com` | запланировано | **27 сентября 2026** |

Оперативный список задач с приоритетами — STATE.md §"Active TODOs".

---

## 12. Заключение

kSystem — это не приложение для чтения Библии и не словарь. Это реализация философского принципа: **знание существует не в изолированных фактах, а в связях между ними**.

- **Грамматика** даёт слово в формальной определённости.
- **Диалектика** помещает слово в структуру бытия и мышления.
- **Риторика** показывает слово в действии — в тексте, жанре, пространстве.

**Синкордия** добавляет этическое измерение:
- **Табу** защищают знание от изоляции и отрыва от контекста.
- **Поощрения** направляют внимание к путям глубинного понимания.
- **Мера** сохраняет соразмерность между усилием и результатом.

Технологический выбор (чистый HTML/CSS/JS, статические JSON, локальная генерация) обеспечивает долговременную устойчивость: kSystem не зависит от облачных сервисов, баз данных или аутентификации, и будет работать десятилетиями.

*«Знание — это не то, что ты имеешь, а то, через что ты движешься. Синкордия — это ритм этого движения, его границы и его мера.»*

```
### `THEORY.md` (head 200 lines)
```
# THEORY.md — kSystem

Formal theoretical foundation of kSystem. For scope and ecosystem, see CONCEPT.md. For numerical values, see PARAMETERS.md. For external sources, see EVIDENCE.md.

---

## 1. Canon

The theoretical canon is **Materials/Принципы.txt**, sections §8.1–8.6 (the "Knowledge System" chapters). All definitions, protocols and algorithms in this document reference that source.

| Canon section | Topic | Implemented in |
|---------------|-------|----------------|
| §8.1 | Word as node of knowledge | lexicon schema (`word`, `ipa`, `mean`, `origin`) |
| §8.2 | Clan-based knowledge system (philosophical root of "k" in kSystem) | project naming; Porphyrian tree (hierarchy → clan) |
| §8.3 | Grammar / Dialectic / Rhetoric trivium | 3-layer architecture (see §3 below) |
| §8.4 | Knowledge System vs. dictionary vs. glossary | corpus linking: word ↔ meaning ↔ contextual text |
| §8.5 | Three-pass reading protocol | reading protocol (§4 below) |
| §8.6 | Depth-first navigation | depth-traversal algorithm (§5 below) |

---

## 2. Axioms

**A1 (Connectivity).** A knowledge system must link every word to a meaning, every meaning to other words, and every word to at least one contextual occurrence in a stable long-lived corpus. Without the word↔word links (kinship tree) it is a dictionary; without word↔context links it is a glossary.

**A2 (Contextuality).** A word has no meaning in isolation; meaning is a function of usage in a corpus. Formally: `meaning(w) = f(w, C)` where `C = {(book, ch, v, text)}` is the contextual corpus.

**A3 (Recursion).** Every word appearing inside the definition `mean(w)` is itself an entry and can be entered recursively. This is what makes knowledge a network rather than a list.

**A4 (Measure).** Depth, breadth and frequency of recursion must be bounded. Unbounded recursion destroys coherence; zero recursion destroys depth. Bounds are set by Syncordia (§6).

**A5 (Three-layer decomposition).** Any entry in a knowledge system can be described by three orthogonal layers: formal (grammar), classificatory (dialectic), contextual (rhetoric). Each layer is computable independently; together they constitute "full" knowledge of the word.

---

## 3. Three-layer architecture (formal definitions)

Let `W` be the set of all word forms in the corpus. Let `L = {dzveli, axali, ru, en, fr, es, ar, zh}` be the language set.

### Layer I — Grammar (word layer)

```
G(w) = { ipa(w), origin(w), cognates(w, L) }
```

- `ipa(w)`: IPA transcription, produced by pipeline `espeak-ng → epitran → eng_to_ipa → pypinyin` (see PARAMETERS.md).
- `origin(w)`: etymology field (free-text, LLM-generated).
- `cognates(w, L)`: cross-language equivalents — discovered via phonetic search on IPA strings.

### Layer II — Dialectic (classification layer)

```
D(w) = { sw(w), sw_exact(w), kin(w), cat(w) }
```

- `sw(w) ∈ SW_META`: one of 121 Swadesh-extended thematic domains.
- `sw_exact(w) ∈ {true, false}`: true iff `w` is in the canonical Swadesh-207 list.
- `kin(w) ∈ KIN_META`: one of 18 Porphyrian tree nodes (`por.being`, `por.substance`, ..., `por.abstract`, `⊘`).
- `cat(w) ∈ ARISTOTLE_CATS`: one of 10 Aristotelian categories (`cat.substance` … `cat.passion`).

Assignment is rule-based (regex patterns in `enrich_lexicons.py`), ordered specific → general, with fallback `kin = por.being`.

### Layer III — Rhetoric (context layer)

```
R(w) = { (book, ch, v, text) ∈ C : w ∈ tokens(text) } × genre × geo × collocations(w)
```

- Genre ∈ {Torah, History, Wisdom, Poetry, Prophecy, Gospel, Acts, Epistle, Apocalypse}.
- Geo: WGS84 coordinates if `book:ch:v` references one of 71 mapped places (`data/geo.json`).
- `collocations(w) = top-8 w' ≠ w` by co-occurrence count within same verse across `C`.

### Full entry

```
Entry(w) = ⟨ G(w), D(w), R(w) ⟩
```

This is the formal semantics of `{id, word, ipa, mean, origin, book, ch, v, sw, sw_exact, kin, cat}`.

---

## 4. Reading Protocol (§8.5 of Principles)

A three-pass discipline applied to a verse `v ∈ C`:

1. **Pass 1 — comprehension.** Read `v` to extract its surface sense.
2. **Pass 2 — interrogation.** Enumerate every question `v` raises.
3. **Pass 3 — filtration.** Retain only those questions that have an unambiguous affirmative answer *inside the text*. Discard the rest as speculation.

Output of Pass 3 is the set of text-supported propositions — the core knowledge extractable from `v`.

---

## 5. Navigation algorithm — Depth-First Traversal (§8.6)

```
traverse(w, depth):
    if depth > DEPTH_LIMIT: return                  # measure (A4)
    entry = Entry(w)
    emit(entry)
    for w' in tokens(mean(w)):
        if w' ∈ lexicon and w' ∉ visited:
            visited.add(w')
            traverse(w', depth + 1)
    for (book, ch, v, text) in R(w):
        emit_context(book, ch, v, text)             # return to corpus
```

- `DEPTH_LIMIT` ∈ [3, 5] (soft), 5 (hard). See PARAMETERS.md.
- Cycle detection via `visited` set.
- The algorithm alternates between *network* (word→word) and *corpus* (word→verse) until exhaustion or boundary.

---

## 6. Syncordia — measure, taboo, reward

**Syncordia** (σύν + χορδή) is the sub-system that regulates §8.5 and §8.6 in practice. It is an ethical/epistemological layer, not a logical layer.

### Three pillars

| Pillar | Formal role | Examples |
|--------|-------------|----------|
| **Taboo** | Rejects ill-formed queries | `block` isolated-word queries (no context); `warn` definition without usage; `mark` extra-corpus extrapolation |
| **Reward** | Reinforces well-formed navigation | +10 for depth-2..5 traversal; +5 for viewing ≥3 languages; +N for returning to source verse |
| **Measure** | Enforces bounds | SRS ≤ 50 cards/session; depth ≤ 5; ideal utilisation 0.6–0.8 of every limit |

### Rules data model

See CONCEPT.md §"Схема синкордия-файла" for the JSON shape (`taboo`, `reward`, `measure` arrays with `id`, `action`, `condition`, `message`).

### Formal statement

Syncordia is a function `S: Event → {allow, warn, mark, block} × ℤ` that inspects a user action and returns a verdict plus a point delta. A well-formed session `σ` maximises cumulative reward under taboo constraints and inside measure bounds.

---

## 7. Corpus structure

```
C = ⋃_{l ∈ L} C_l
C_l = {(id, book, ch, v, text_l) : verse in language l}
```

| Language | `|C_l|` | Canon |
|----------|---------|-------|
| dzveli | 32,510 | Full OT+NT, incl. apocrypha (Tobit, Judith, Maccabees) |
| axali | 27,541 | Synodal-equivalent, modern Georgian |
| ru | 31,102 | Synodal |
| en | 31,102 | KJV 1611 |
| fr | 35,486 | Synodale 1921 (+ deuterocanonical) |
| es | 31,102 | Reina-Valera |
| ar | 7,105 | NT only |
| zh | 31,102 | 和合本 (CUV) |

Alignment across `C_l` is by `(book, ch, v)` identifier triple; not all verses align (deuterocanonical differences, versification).

---

## 8. Chapter ↔ module map

Traceability from theory to code:

| Principle | Theory §§ | Implementation |
|-----------|-----------|----------------|
| Word as node | §8.1 | `{lang}/lexicon.json` entry |
| Clan-based knowledge | §8.2 | `kin` field (Porphyrian tree) |
| Trivium | §8.3 | Layer I = `ipa`/`origin`; Layer II = `sw`/`kin`/`cat`; Layer III = `book`/`ch`/`v` + `geo.json` |
| Word ↔ corpus binding | §8.4 | `book`, `ch`, `v` fields; Bible search panel |
| Three-pass reading | §8.5 | UI reading mode (planned); currently informal |
| Depth-first | §8.6 | `openEntry()` click-through in `website/js/app.js`; cycle-detect in visited set |
| Syncordia | (ethical extension of §§8.4–8.6) | `website/data/syncordia.json` (proposed v2.0.0); `website/js/syncordia.js` (planned) |

```
### `PARAMETERS.md` (head 200 lines)
```
# PARAMETERS.md — kSystem

Numerical defaults for the kSystem pipeline and website. Change here first, then update code and re-derive downstream data. For architectural context see DESIGN.md. For theoretical meaning see THEORY.md.

---

## 1. Lexicon generation pipeline (`build_lexicons.py`)

| Parameter | Value | Notes |
|-----------|-------|-------|
| `CHECKPOINT_INTERVAL` | `5` words | Save `lexicon.json` every N words |
| `OLLAMA_MODEL` | `llama3.2` | LLM used for definition generation |
| `OLLAMA_TIMEOUT` | `60` s | Per-word LLM timeout |
| `ESPEAK_TIMEOUT` | `5` s | Per-word IPA timeout |
| `BATCH_SIZE` | `1` | Words per LLM call (currently no batching) |
| `MAX_WORDS_PER_RUN` | unlimited | Set to `N` to stop after N words (for testing) |

---

## 2. IPA fallback chain

Ordered list. First success wins.

| Order | Tool | Languages |
|-------|------|-----------|
| 1 | `espeak-ng` | all 8: `en`, `ru`, `fr`, `es`, `zh`, `ar`, `ka` (for `axali` and `dzveli`) |
| 2 | `epitran` | `ru` (rus-Cyrl), `fr` (fra-Latn), `es` (spa-Latn), `ar` (ara-Arab), `ka` (kat-Geor) |
| 3 | `eng-to-ipa` | `en` only |
| 4 | `pypinyin` | `zh` only |

**Do not change this order without updating CLAUDE.md §4.5.**

---

## 3. Enrichment (`enrich_lexicons.py`)

| Parameter | Value | Notes |
|-----------|-------|-------|
| `SW_META` size | `121` thematic domains | Swadesh-207 core + thematic extensions |
| `KIN_META` size | `18` Porphyrian nodes | `por.being` … `por.abstract` + `⊘` (not classifiable) |
| `ARISTOTLE_CATS` size | `10` | `cat.substance`, `cat.quantity`, `cat.quality`, `cat.relation`, `cat.place`, `cat.time`, `cat.position`, `cat.state`, `cat.action`, `cat.passion` |
| Pattern order | specific → general | Regex patterns checked in order; first match wins |
| Fallback for `kin` | `por.being` | When no Porphyrian pattern matches |

---

## 4. Website / UI (`website/js/app.js`)

| Parameter | Value | Notes |
|-----------|-------|-------|
| Max lexicon results | `200` | Linear-scan cap |
| Max Bible results | `500` | Linear-scan cap |
| SRS cards per session | `50` | SM-2 session limit |
| SRS difficulty levels | `4` | ❌ Again · 😐 Hard · ✅ Know · ⭐ Easy |
| Collocation top-N | `8` | Co-occurring words shown per entry |
| IPA keyboard variants | `8` | One per language |
| `localStorage` key prefix | `ksystem_` | All SRS + UI state keys |
| GEO_IPA_MAP coverage | `33` characters | Full Mkhedruli alphabet → IPA |
| Depth-traversal soft cap | `5` | Syncordia "measure" limit |
| Depth-traversal hard cap | `5` | Hard stop (same as soft; adjust separately if needed) |

---

## 5. Syncordia defaults (`syncordia.json` v2.0.0)

| Parameter | Value | Notes |
|-----------|-------|-------|
| Reward for depth-traversal | `+10` points | Condition: `2 ≤ depth ≤ 5` |
| Reward for cross-language | `+5` points | Condition: `viewed_languages_count ≥ 3` |
| Ideal utilisation range | `0.6`–`0.8` | Fraction of any limit considered "in measure" |
| Hard limit: SRS cards/session | `50` | Matches UI parameter above |
| Soft limit: recursion depth | `5` | Matches UI parameter above |

See THEORY.md §6 for semantics.

---

## 6. Corpus sizes (as of 2026-03-22)

| Language | `bible.json` | `_words.json` | `lexicon.json` |
|----------|-------------|---------------|----------------|
| `dzveli` | 32,510 verses | — | 16,697 words (complete) |
| `axali` | 27,541 verses | 26,398 words | generating |
| `ru` | 31,102 verses | 26,125 words | generating |
| `en` | 31,102 verses | 8,549 words | generating |
| `fr` | 35,486 verses | 16,580 words | generating |
| `es` | 31,102 verses | 15,180 words | generating |
| `ar` | 7,105 verses (NT) | 8,353 words | generating |
| `zh` | 31,102 verses | 26,460 words | generating |
| **Total words to process** | | **127,645** | |

Definition phase throughput: ~9 s/word → ≈ 13 days wall-clock on a single CPU.

---

## 7. Indexing thresholds

| Parameter | Value | Location |
|-----------|-------|----------|
| All lexicons loaded into browser memory | up to `~64 MB` uncompressed | `app.js` initial fetch |
| Gzipped transfer (expected) | `~15 MB` | Subject to web-server compression |
| Cross-language search corpus (`allLexMini`) | sum of all 8 lexicons ≈ `128 k` entries | Built in-browser on page load |

---

## 8. Environment variables

| Variable | Source | Role |
|----------|--------|------|
| `DEEPSEEK_API_KEY` | `~/.aim_env` | DeepSeek API (inherited from AIM) |

No other environment variables are required for the website or the pipeline.

---

## 9. Systemd service (template)

```ini
# ~/.config/systemd/user/ksystem-define.service
[Unit]
Description=kSystem lexicon definition generator

[Service]
WorkingDirectory=/home/oem/Desktop/kSystem
ExecStart=/usr/bin/python3 build_lexicons.py --phase define
Restart=no
StandardOutput=append:/home/oem/Desktop/kSystem/define.log
StandardError=append:/home/oem/Desktop/kSystem/define.log

[Install]
WantedBy=default.target
```

---

## 10. Deployment

| Parameter | Value |
|-----------|-------|
| Local dev port | `7777` |
| Production domain | `ksystem.drjaba.com` |
| Production launch | `2026-09-27` |
| Hosting type | Static (any web server or CDN) |
| GitHub repo | `https://github.com/djabbat/kSystem` (public) |

---

## 11. What is NOT configured here (by design)

- **Authentication** — kSystem has no user accounts.
- **Database** — no DBMS; JSON only.
- **Backend URL** — there is no backend service.
- **Cross-device sync** — all state is `localStorage`-local.
- **Rate limiting** — there is no API to rate-limit.
- **Payment / billing** — not applicable.

These absences are intentional architectural choices, not missing configuration. See CLAUDE.md §4 and DESIGN.md §1.

```
### `STATE.md` (head 200 lines)
```
# STATE.md — kSystem

Volatile state. Replaces the legacy TODO.md + MEMORY.md + UPGRADE.md triad. Update freely. Newest entries on top in each dated section.

---

## Current status (2026-04-25)

- Active development phase; public launch scheduled **2026-09-27** on `ksystem.drjaba.com`.
- Background systemd service `ksystem-define.service` is processing 127,645 words across 8 languages (~9 s/word → ~13 days total). Started 2026-03-22.
- Only `dzveli` (16,697 words) is a fully complete and enriched lexicon. The other seven lexicons are mid-generation.
- GitHub repo public: https://github.com/djabbat/kSystem.
- Doc system migrated today to the 9-file core schema.

---

## Startup checklist (read on every new session)

1. Read **CONCEPT.md** for the canonical scope and definitions.
2. Read **CLAUDE.md** for assistant operating rules (Rust default; DeepSeek for text tasks).
3. Read this **STATE.md** for current priorities and the Decision Log.
4. Check if `ksystem-define.service` is running: `systemctl --user status ksystem-define.service`.
5. Tail the log if investigating data issues: `tail -f /home/oem/Desktop/kSystem/define.log`.
6. Default programming language is **Rust**. Python is permitted only for the existing pipeline scripts.
7. Text-generation tasks (lexicon articles, translations, polishing, reviews, grants) must go through DeepSeek API via `~/Desktop/AIM/llm.py` — not handcrafted.

---

## Active TODOs

Priorities: P0 = blocker, P1 = next up, P2 = important soon, P3 = useful, P4 = from published paper's Future Work, P5 = aspirational.

### P0 — Blockers

(None currently. `define` service runs in background autonomously.)

### P1 — Immediate

- **Add IPA to `dzveli` lexicon.** The Saba lexicon (16,697 words) is complete but lacks the `ipa` field. One-shot script:
  ```bash
  python3 -c "
  import json
  from build_lexicons import get_ipa
  lex = json.load(open('website/data/dzveli/lexicon.json'))
  for e in lex:
      if 'ipa' not in e:
          e['ipa'] = get_ipa(e['word'], 'dzveli')
  json.dump(lex, open('website/data/dzveli/lexicon.json','w'), ensure_ascii=False, separators=(',',':'))
  print('Done:', len(lex))
  "
  ```
- **Enable `axali` in UI.** In `website/js/app.js` find `'axali'` entry and set `hasLex: true` *after* define completes for axali.
- **Post-define verification.** When `ksystem-define.service` finishes, run:
  ```bash
  for lang in en ru fr es zh ar axali; do
    python3 -c "import json; d=json.load(open('website/data/$lang/lexicon.json')); print('$lang:', len(d), 'words')"
  done
  ```
  Then run `enrich_lexicons.py` for each.

### P2 — Lexical completeness

- **Full concordance in word view.** Show all verses that contain the current word (currently partial via bible search).
- **Morphological parser.** Especially needed for Georgian, Arabic, Russian — rich inflection currently breaks exact-match search. See OPEN_PROBLEMS.md §"Morphology".
- **Root family display** — cluster lexicon entries that share a root.
- **Parallel word-by-word alignment** — dzveli ↔ en ↔ ru.
- **Polysemy.** Multiple distinct senses for one word from different contexts.
- **Parallel viewer.** Two-language split bible panel.
- **Verse export.** Copy quote with book:ch:v citation.
- **Mobile layout.** Panel swipe on small viewports.

### P3 — Paper Future Work (Digital Trivium, Table 4)

- **Morphological analysis [P1 in paper].** Stemmer/lemmatizer for ka/dzveli.
- **Extend SW to ~300 domains [P2 in paper].** Current 121; target full Swadesh-207 + thematic overlay. Measure by share of lexicon with `sw_exact=true`.
- **Inter-annotator verification [P3 in paper].** Dzveli currently single-annotator. Need second independent annotator with Cohen's κ ≥ 0.70, especially for `kin` (por.*) and `cat`.
- **Seven-language lexicon coverage [P4 in paper].** After define completes, run enrichment and flip `hasLex: true` for all.
- **Parallel alignment [P5 in paper].** GIZA++ or BibleAlign between dzveli/en/ru.
- **Corpus expansion [P6 in paper].** Add apocrypha to non-dzveli corpora; extend ar to full OT; add patristic commentary as secondary corpus.

### P4 — From UPGRADE.md proposals (2026-03-29)

- **Graph visualisation of word connections** (D3.js / Cytoscape.js force-directed).
- **Public REST/GraphQL API** for external developers and researchers; rate-limited, tiered (free/academic/commercial).
- **Mobile app** (React Native / Expo) with offline top-10k cache.
- **Semantic search with multilingual embeddings** (LaBSE or mE5-large + FAISS/ChromaDB).

### P5 — Aspirational

- Patristic commentary corpus.
- Full Arabic OT.
- User-collaborative annotation with verification workflow.

---

## Milestones (done)

### 2026-04-25 — 9-file core migration
- Migrated documentation from legacy 10-file core to new 9-file schema.
- Moved TODO.md, MEMORY.md, UPGRADE.md, LINKS.md, KNOWLEDGE.md, MAP.md to `_archive/core_pre_9file_2026-04-25/`.
- Created THEORY.md, DESIGN.md, EVIDENCE.md, STATE.md, OPEN_PROBLEMS.md.
- Revised CONCEPT.md, README.md, CLAUDE.md, PARAMETERS.md for DRY (no duplicated content).

### 2026-03-28 — Project core created
- Full 10-file project core assembled: CONCEPT, CLAUDE, PARAMETERS, MAP, MEMORY, LINKS, KNOWLEDGE, UPGRADE, README, TODO.
- Default language rule established: Rust first, Python only for legacy pipeline.

### 2026-03-22 — IPA system complete
- `espeak-ng` installed and integrated (all 8 languages).
- `GEO_IPA_MAP` added to `app.js` for in-browser real-time Mkhedruli → IPA.
- Web Speech API integrated (🔊 button) in detail view and IPA search results.
- IPA cross-language search panel added to UI (centre panel).
- All three panels (lex / bible / IPA) synchronised on click; `localStorage` persistence.

### 2026-03-22 — Peer-review round features
- Genre classification badges (8 genres × 8 languages).
- Word frequency counter in entry card.
- Collocation panel — top-8 co-occurring words.
- SRS flashcards with SM-2 algorithm, 50 cards/session, 4-level difficulty.

### 2026-03-22 — Digital Trivium paper + Porphyry
- `KIN_META` switched from kinship terms to 18-node *Arbor Porphyriana*.
- `KIN_RULES` regex patterns for Porphyrian classification in `enrich_lexicons.py`.
- `SW_META` corrected to 121 domains (not ~300 as in early paper drafts).
- `ABOUT.md` rewritten across all 8 languages matching published paper.
- Entry-click synchronisation extended to update `lex-search` field.

### 2026-03-22 — Publication + GitHub
- DOI 10.65649/yx9sn772 — *The Digital Trivium*, *Annals of Rejuvenation Science* 2025.
- GitHub repo made public at https://github.com/djabbat/kSystem.

### 2026-03-18 — Launch date decision
- Public launch deferred to 2026-09-27. Development continues locally.

---

## Decision Log (newest first)

| Date | Decision | Rationale |
|------|----------|-----------|
| 2026-04-25 | Migrated to 9-file core schema | Consolidate TODO+MEMORY+UPGRADE into STATE; add THEORY/DESIGN/EVIDENCE/OPEN_PROBLEMS; eliminate documentation drift |
| 2026-03-28 | Default language: Rust for new code; Python stays for existing pipeline | Rust gives long-term binaries; Python pipeline already depends on `ollama`/`epitran`/`espeak-ng` Python bindings with no Rust equivalents |
| 2026-03-22 | SW_META = 121 domains | Corrected earlier "~300" claim in draft paper — code was always 121 |
| 2026-03-22 | `KIN_META` ← Porphyrian tree (18 nodes) | Required by *Digital Trivium* paper for ontological classification; replaces less-principled kinship terms |
| 2026-03-22 | GitHub repo made public | Research transparency; no secrets in repo |
| 2026-03-18 | Public launch deferred to 2026-09-27 | Content not ready; define pipeline just starting |

---

## What NOT to do

- **Do not push `define.log`, `_master_progress.json` partial states, or intermediate JSON files** to GitHub unless intentional.
- **Do not add a build step / framework / bundler to `website/`.** Static-first is a hard architecture rule (see DESIGN.md §1).
- **Do not add CDN-hosted JS libraries.** Everything bundled or absent.
- **Do not rewrite an entire `lexicon.json` in one shot.** All pipeline scripts must checkpoint every N words (default 5).
- **Do not add fields to the lexicon schema without updating PARAMETERS.md and DESIGN.md §4.1.**
- **Do not generate lexicon articles or translations by hand.** Route them through DeepSeek (`~/Desktop/AIM/llm.py`).
- **Do not touch `Articles/`, `docs/`, `scripts/`, `website/`, or `.git*` during documentation migrations** — core docs only.
- **Do not break 8-language UI parity.** Any user-visible string must exist in all 8 language packs.
- **Do not rely on DOI `10.65649/*` for Google Scholar tracking** — not yet indexed as of 2026-03-28.

---

## Open questions (pointers)

Substantive unresolved questions live in **OPEN_PROBLEMS.md**. This section only indexes them.

- Morphological analyser — Rust or Python? When?
- Expand Arabic to full OT — which source?
- Inter-annotator agreement study — who and when?
- Parallel word alignment — GIZA++ vs. modern attention-based?
- Axali UI enablement timing.

```
### `DESIGN.md` (head 200 lines)
```
# DESIGN.md — kSystem

System architecture, file layout, workflow and contracts. For theory see THEORY.md. For parameters see PARAMETERS.md.

---

## 1. Architectural principles

1. **Static-first.** The `website/` directory is a pure static SPA (HTML5 + CSS3 + vanilla JS). No server-side logic, no database, no build step.
2. **JSON as data layer.** All runtime data lives in versioned JSON files; no DBMS.
3. **Checkpointable pipelines.** Any long-running Python script writes progress every N records and resumes on restart.
4. **No CDN dependencies.** Fonts and libraries bundled or absent; kSystem must survive offline.
5. **Depth traversal over search.** The primary UX is click-through navigation through definitions, not keyword lookup.
6. **Zero-framework front-end.** No React, Vue, Svelte. One `app.js`, one `style.css`, one `index.html`.

---

## 2. File tree with role per folder

```
kSystem/
│
├── CONCEPT.md                  — single source of truth (canon)
├── README.md                   — public-safe quickstart (goes to public git)
├── CLAUDE.md                   — AI assistant rules
├── THEORY.md                   — formal equations, axioms, canon map
├── DESIGN.md                   — this file
├── EVIDENCE.md                 — external sources, corpora, related work
├── PARAMETERS.md               — numerical defaults
├── STATE.md                    — volatile: TODOs, milestones, decisions
├── OPEN_PROBLEMS.md            — validation gaps, open questions, risks
│
├── CONCEPT_CODE_AUDIT_2026-04-21.md   — audit artifact (static)
├── PEER_REVIEW_kSystem.md             — peer review artifact (static)
├── REFERENCE_AUDIT_kSystem.md         — reference audit artifact
├── CORRECTION_CANDIDATES.md           — correction candidates artifact
│
├── _archive/core_pre_9file_2026-04-25/   — superseded pre-migration core docs
│
├── scripts/                    — Python data pipeline
│   ├── build_lexicons.py       — phase=extract + phase=define (Ollama + espeak-ng)
│   ├── enrich_lexicons.py      — adds sw / kin / cat fields
│   ├── fetch_bibles.py         — scrollmapper downloader
│   ├── fetch_georgian_arabic.py— orthodoxy.ge + azbyka.ru scraper
│   ├── convert.py              — SQLite → JSON for dzveli
│   └── build_syncordia.py      — generates syncordia.json (planned)
│
├── docs/                       — long-form documentation assets
│
├── Articles/                   — related academic articles
│
└── website/                    — static SPA (the deliverable)
    ├── index.html              — 3-panel layout: Lexicon | Bible | IPA/Sound
    ├── css/style.css           — styling incl. RTL (Arabic) and SRS cards
    ├── js/
    │   ├── app.js              — all application logic (~2 500 LOC)
    │   ├── syncordia.js        — taboo/reward/measure (planned)
    │   └── srs.js              — SM-2 spaced repetition (planned split)
    └── data/
        ├── geo.json            — 71 biblical places, WGS84
        ├── syncordia.json      — taboo/reward/measure rule set (planned)
        ├── _master_progress.json — define-phase checkpoint (127 645 words)
        └── {lang}/             — per-language data (dzveli, axali, ru, en, fr, es, ar, zh)
            ├── bible.json      — [{id, book, ch, v, text}]
            ├── _words.json     — unique word list (input for define)
            ├── _progress.json  — per-language progress
            └── lexicon.json    — [{id, word, ipa, mean, origin, book, ch, v, sw, sw_exact, kin, cat}]
```

Note: pipeline scripts currently live at the repo root alongside `Materials/`; migration to a `scripts/` directory is a planned reorganisation (tracked in STATE.md).

---

## 3. Workflow — corpus → knowledge graph

```
            ┌─────────────────┐
            │  Raw sources    │  scrollmapper / orthodoxy.ge / azbyka.ru / saba.sql
            └────────┬────────┘
                     │  fetch_bibles.py, fetch_georgian_arabic.py, convert.py
                     ▼
            ┌─────────────────┐
            │  bible.json     │  per-language corpus C_l
            └────────┬────────┘
                     │  build_lexicons.py --phase extract
                     ▼
            ┌─────────────────┐
            │  _words.json    │  unique word list per language
            └────────┬────────┘
                     │  build_lexicons.py --phase define
                     │  (Ollama llama3.2  +  espeak-ng)
                     │  checkpoint every 5 words
                     ▼
            ┌─────────────────┐
            │  lexicon.json   │  {id, word, ipa, mean, origin, book, ch, v}
            └────────┬────────┘
                     │  enrich_lexicons.py
                     │  (regex patterns → sw, sw_exact, kin, cat)
                     ▼
            ┌─────────────────┐
            │  lexicon.json   │  (enriched with dialectic layer)
            └────────┬────────┘
                     │  loaded by website/js/app.js on page open
                     ▼
            ┌─────────────────┐
            │  Knowledge graph│  in-browser traversal (depth-first §8.6)
            └─────────────────┘
```

---

## 4. API / data contracts

### 4.1 Lexicon entry schema

```json
{
  "id": 12345,
  "word": "სიყვარული",
  "ipa": "sikʼvaruli",
  "mean": "უპირობო განწყობა სხვისადმი",
  "origin": "Kartvelian root *qʷar-",
  "book": "1Cor",
  "ch": 13,
  "v": 1,
  "sw": "emotion.positive",
  "sw_exact": true,
  "kin": "por.abstract",
  "cat": "cat.quality"
}
```

All fields required except `sw_exact` defaults `false`. Adding a field requires updating PARAMETERS.md and this section.

### 4.2 Bible verse schema

```json
{ "id": 23456, "book": "Matt", "ch": 5, "v": 3, "text": "..." }
```

### 4.3 Syncordia rule schema (planned v2.0.0)

```json
{
  "_syncordia_manifest": { "version": "2.0.0", "created_at": "ISO-8601", "philosophy": "..." },
  "taboo":   { "rules": [ { "id", "name", "action": "block|warn|mark", "message" } ] },
  "reward":  { "rules": [ { "id", "name", "condition", "points", "message" } ] },
  "measure": {
    "limits": [ { "id", "name", "max", "action": "soft_limit|hard_limit", "message" } ],
    "ideal_range": { "min": 0.6, "max": 0.8, "description": "..." }
  }
}
```

### 4.4 Internal JS API (stable handles in `app.js`)

| Function | Contract |
|----------|----------|
| `openEntry(word, lang)` | Renders lexicon entry; updates IPA and Bible panels; syncs `lex-search`. |
| `ipaSearch(query)` | Cross-language search over `allLexMini`; returns ranked list. |
| `playWord(text, lang)` | Uses Web Speech API to speak `text` with best-available voice for `lang`. |
| `get_ipa(word, lang)` (Python) | IPA priority: espeak-ng → epitran → eng_to_ipa → pypinyin. |

---

## 5. Deployment plan

| Stage | Mode | Command |
|-------|------|---------|
| Local dev | `python3 -m http.server 7777 --directory website` | Open `http://localhost:7777` |
| Static hosting (target) | Any static host: nginx, GitHub Pages, Cloudflare Pages, Netlify | Serve `website/` as document root |
| Production domain | `ksystem.drjaba.com` (scheduled 2026-09-27) | CNAME or static-site deploy |
| Background pipeline | `systemctl --user start ksystem-define.service` | Processes 127 645 words over ~13 days |

No backend, no runtime environment beyond a static file server.

---

## 6. Performance targets

| Metric | Target | Current / method |
|--------|--------|------------------|
| Lexicon lookup latency (single entry, client) | < 50 ms | O(n) linear scan over pre-loaded JSON; cached after first search |
| Bible verse search latency | < 200 ms | O(n) over `bible.json`; capped at 500 results |
| Cross-language IPA search | < 300 ms | Scan over `allLexMini`; 8 × |lex| ≈ 128 k entries worst case |
| Initial page load (all 8 languages) | < 5 s on local, < 15 s on 3G | ~64 MB total data uncompressed; gzip brings to ~15 MB |
| SRS card turn-around | < 100 ms | Pure localStorage; no network |
| Corpus indexing (full define phase) | ≤ 13 days on single CPU | ~9 s/word × 127 645 words, checkpointed every 5 |
| Enrichment pass (all 8 langs) | < 30 min | Regex-based, single file scan |

---

## 7. Concurrency and checkpointing

- `build_lexicons.py` writes `lexicon.json` every `CHECKPOINT_INTERVAL` words (default 5).
- Master list position recorded in `website/data/_master_progress.json`.
- SIGINT (Ctrl+C) is safe: the next run resumes from the last saved index.
- Never rewrite an entire JSON file in one shot; always read-modify-rename (`foo.json.tmp` → `foo.json`).

---

```
### `EVIDENCE.md` (head 200 lines)
```
# EVIDENCE.md — kSystem

External sources, corpora, tools and related projects. Everything in this file is *outside* the kSystem repo — upstream data, dependencies, prior art. For internal content see CONCEPT.md, THEORY.md, DESIGN.md.

---

## 1. Bible corpora (8 languages)

| ID | Language | Edition | Source | Verses | Notes |
|----|----------|---------|--------|--------|-------|
| `dzveli` | Old Georgian | Manuscript tradition + Sulkhan-Saba lexicon | `Materials/saba.sql` (SQLite dump, 17th c.) | 32,510 | Includes apocrypha (Tobit, Judith, Maccabees) |
| `axali` | Modern Georgian | Synodal equivalent | https://www.orthodoxy.ge (scraped) | 27,541 | Via `fetch_georgian_arabic.py` |
| `ru` | Russian | Synodal | https://scrollmapper.github.io | 31,102 | |
| `en` | English | King James Version (1611) | https://scrollmapper.github.io | 31,102 | |
| `fr` | French | Synodale 1921 | https://scrollmapper.github.io | 35,486 | Includes deuterocanonical |
| `es` | Spanish | Reina-Valera | https://scrollmapper.github.io | 31,102 | |
| `ar` | Arabic | New Testament only | https://azbyka.ru (scraped) | 7,105 | OT not yet scraped |
| `zh` | Chinese | 和合本 (CUV — Chinese Union Version) | https://scrollmapper.github.io | 31,102 | |

Total verses across all languages: ≈ 227,050.

---

## 2. Lexicographic sources

| Lexicon | Source | License / status |
|---------|--------|------------------|
| **Sulkhan-Saba Orbeliani, სიტყვის კონა** ("Bouquet of Words") | `Materials/saba.sql` — SQLite dump, 17th-century original, 16,697 entries | Public domain; only fully completed lexicon in kSystem |
| All other languages | Generated in-project via Ollama `llama3.2` + `espeak-ng` against biblical context | Created by kSystem pipeline; LLM-generated definitions are preliminary, unverified by human experts |

---

## 3. Related projects / prior art

| Project | URL | Overlap with kSystem | Differentiator |
|---------|-----|----------------------|----------------|
| **Logos Bible Software** | https://www.logos.com | Multi-language Bible search, morphological search | Proprietary, subscription, Protestant-oriented; kSystem is open, static, philosophy-first |
| **Accordance** | https://accordancebible.com | Original-language research | Proprietary; kSystem free and corpus-bound |
| **STEP Bible** (Tyndale House) | https://www.stepbible.org | Open, multilingual | STEP is lookup-oriented; kSystem is depth-traversal and ontology-annotated |
| **OpenBible.info** | https://www.openbible.info | Geographic Bible data | OpenBible gives geodata only; kSystem integrates geo with lexicon + trivium |
| **Perseus Digital Library** | https://www.perseus.tufts.edu | Greek/Latin morphology | Perseus is classical Greek/Latin; kSystem is biblical 8-language |
| **Logeion** | https://logeion.uchicago.edu | Merged classical lexicons | Logeion = multi-source dictionary; kSystem = ontology + corpus binding |
| **BibleHub** | https://biblehub.com | Parallel translations, interlinear | BibleHub linear search; kSystem depth-first |
| **Bible Gateway** | https://www.biblegateway.com | Multiple translations via API | Proprietary API, consumer UX; kSystem research-grade |

---

## 4. Related ecosystem projects (authored by same principal)

| Project | Role |
|---------|------|
| **AIM** (`~/Desktop/AIM/`) | Parent ecosystem; shared DeepSeek key at `~/.aim_env`; shared CLAUDE.md rules |
| **longevity.ge** | Scientific journal platform (OJS); ecosystem sibling |
| **drjaba** | Umbrella site for Dr Jaba Tkemaladze; kSystem production domain `ksystem.drjaba.com` |
| **monetaria** | Ecosystem-level project (economics/governance); shares CLAUDE/state schema conventions |

---

## 5. Academic publications

| Publication | DOI / PMID | Role in kSystem |
|-------------|------------|-----------------|
| Tkemaladze, J. (2025). *The Digital Trivium: A Three-Layer Knowledge Architecture for Ancient Text Corpora.* *Annals of Rejuvenation Science*, 2025. | DOI: 10.65649/yx9sn772 | Primary kSystem paper; describes Grammar/Dialectic/Rhetoric layers on Bible corpus |
| Tkemaladze J. *Mol Biol Rep* 2023 — "Reduction, proliferation, and differentiation defects of stem cells over time: a consequence of selective accumulation of old centrioles in the stem cells?" | PMID: 36583780 | Self-citation (inherited from AIM) |
| Lezhava T. et al. (incl. Tkemaladze), *Biogerontology* 2011 — "Gerontology research in Georgia." | PMID: 20480236 | Self-citation (inherited from AIM) |

Note (from `REFERENCE_AUDIT_kSystem.md`): PMIDs 36583780 and 20480236 are flagged low-context-match against their enclosing self-citation rule; the citations themselves are valid but reviewers may find the context/keyword overlap weak. DOI 10.65649/* is not yet indexed in Google Scholar as of 2026-03-28.

---

## 6. Linguistic / philosophical references

| Framework | URL | Usage |
|-----------|-----|-------|
| Swadesh list (207-word core) | https://en.wikipedia.org/wiki/Swadesh_list | `sw_exact` field; extended to 121 thematic domains |
| Arbor Porphyriana | https://en.wikipedia.org/wiki/Porphyrian_tree | `kin` field — 18 ontological nodes |
| Aristotle, *Categories* | https://en.wikipedia.org/wiki/Categories_(Aristotle) | `cat` field — 10 predicamental categories |
| SuperMemo SM-2 algorithm | https://www.supermemo.com/en/articles/two | SRS flashcards, 4-level difficulty |
| WGS84 coordinate system | https://en.wikipedia.org/wiki/World_Geodetic_System | `geo.json` biblical places |
| Σύν + χορδή etymology | standard Greek lexica (LSJ) | "Syncordia" naming justification |

---

## 7. Tools and dependencies

| Tool | URL | Role |
|------|-----|------|
| Ollama | https://ollama.ai | Local LLM runtime (`llama3.2`) for definition generation |
| espeak-ng | https://github.com/espeak-ng/espeak-ng | Primary IPA source for all 8 languages |
| epitran | https://github.com/dmort27/epitran | IPA fallback (ru, fr, es, ar, ka) |
| eng-to-ipa | https://pypi.org/project/eng-to-ipa/ | English IPA fallback |
| pypinyin | https://github.com/mozillazg/python-pinyin | Chinese IPA fallback |
| phonemizer | https://github.com/bootphon/phonemizer | Alt phonemisation backend |
| Hunspell ka_GE | https://github.com/gamag/ka_GE.spell | Georgian spell-checker dictionary — candidate source for future stemmer/lemmatizer (not yet integrated) |
| Web Speech API | https://developer.mozilla.org/en-US/docs/Web/API/Web_Speech_API | Browser-side audio playback |

---

## 8. Repository & deployment URLs

| Resource | URL |
|----------|-----|
| GitHub (public) | https://github.com/djabbat/kSystem |
| Production domain (scheduled 2026-09-27) | https://ksystem.drjaba.com |
| Local dev | http://localhost:7777 |

---

## 9. AIM ecosystem linkages

| Resource | Path / key |
|----------|-----------|
| AIM root | `~/Desktop/AIM/` |
| DeepSeek API key | `~/.aim_env → DEEPSEEK_API_KEY` |
| DeepSeek entry point | `~/Desktop/AIM/llm.py` |
| Ecosystem CLAUDE.md | `~/Desktop/AIM/CLAUDE.md` |
| DeepSeek models | `deepseek-chat` (fast), `deepseek-reasoner` (complex) |

```
### `OPEN_PROBLEMS.md` (head 200 lines)
```
# OPEN_PROBLEMS.md — kSystem

Unresolved theoretical, methodological, empirical and strategic questions. This file is *not* a TODO — it collects open problems that may or may not be solvable, and tracks them across releases. Operational tasks live in STATE.md.

---

## 1. Validation gaps

### 1.1 LLM-generated definitions are unverified

`mean` and `origin` fields for all non-`dzveli` lexicons are generated by local `llama3.2` against biblical context. No human expert has verified any of these. The text presented to users is *preliminary*. We have no ground-truth corpus, no inter-rater agreement metric, no hallucination rate estimate.

**Needed:** (a) sample-based manual audit (e.g. 1% random sample per language); (b) known-good gold standard for at least one language (candidate: Strong's-style concordance for English); (c) machine-measurable hallucination proxy.

### 1.2 Classification (`sw`, `kin`, `cat`) is rule-based, single-annotator

Porphyrian tree and Aristotelian category assignments are performed by regex patterns in `enrich_lexicons.py`. For `dzveli` (the only complete lexicon) these have been reviewed by a single annotator. Scientific weight requires a second independent annotator and Cohen's κ ≥ 0.70. This is P3 in the published paper's Future Work.

### 1.3 Swadesh coverage metric is not reported

The claim "121 thematic domains extending Swadesh-207" has no accompanying coverage number: of the 207 canonical Swadesh items, how many receive `sw_exact=true`? Of the 16,697 dzveli entries, what fraction land in each domain? Need a coverage table and a saturation curve.

### 1.4 Cross-translation verse alignment

We align verses across languages by `(book, ch, v)` identifier triple. This silently fails whenever versification differs (e.g. Psalms numbering, deuterocanonical insertions, Masoretic vs. Septuagint chapter splits). We do not know how many verses are mis-aligned and we do not surface that uncertainty in the UI.

### 1.5 IPA quality

IPA is produced via a fallback chain `espeak-ng → epitran → eng_to_ipa → pypinyin`. Some languages (notably Old Georgian) have no authoritative IPA dictionary; espeak-ng output is not guaranteed correct for dead/archaic forms. No accuracy study has been run. For dzveli specifically, espeak-ng uses modern Georgian (`ka`) phonology, which is anachronistic.

### 1.6 Syncordia rule-set is empirically uncalibrated

The taboo/reward/measure rules (v2.0.0) are theoretical — they have not been validated against user behaviour. The claim that "ideal utilisation = 0.6–0.8 of every limit" has no empirical support yet. Needs user study.

---

## 2. Theoretical open questions

### 2.1 What stops a depth-first traversal from drifting off-topic?

The algorithm guarantees termination (depth cap + visited set) but not *relevance*. After 5 hops from "love" we may be looking at "microchip" via a chain of partial semantic overlaps. A coherence metric is absent — we have no way to flag when a traversal path has left the original query's semantic neighbourhood.

### 2.2 Is the three-layer trivium exhaustive?

The Grammar/Dialectic/Rhetoric decomposition is motivated by classical pedagogy, not by a formal completeness theorem. Could a "Pragmatic" (speech-act) or "Hermeneutic" (historical reception) fourth layer yield knowledge that our three layers cannot express?

### 2.3 Taboo is a type of ethics, not a type of information structure

Syncordia's taboo rules (`block` isolated-word queries, `warn` definitions without usage, `mark` extra-corpus extrapolation) operate at the UX layer. But is *taboo* a property of the system's state machine, of the user's intent, or of the request itself? The current implementation conflates these. A formal separation — intent vs. request vs. system-state — is unexplored.

### 2.4 How does measure interact with exploration?

A hard limit at depth=5 is a reasonable default but may cut off legitimate deep research. Is there a better-than-fixed-cap measure? E.g. a diminishing-returns termination that looks at novelty-per-hop and stops when novelty drops below a threshold.

### 2.5 Ontology choice bias

Porphyry and Aristotle are Greek/Western frameworks. Applying them to a Sulkhan-Saba Georgian lexicon is a methodological choice, not a neutral measurement. We should document the bias and possibly provide an alternative (e.g. Bhartṛhari's *sphoṭa* theory, or Abhidharma categories) for comparative study.

---

## 3. Implementation gaps

### 3.1 Morphology (P1 in the paper)

Current matching is full-word only. Georgian has polypersonal verbs and seven cases; Russian has six cases × three genders × two numbers; Arabic has root-and-pattern templatic morphology. Exact-match loses many legitimate occurrences. A stemmer/lemmatizer is needed per language. Candidate stacks: Hunspell + custom rules; Stanza; spaCy (limited Georgian support); custom rule-based (Rust) for ka.

### 3.2 Arabic corpus: NT only

Only New Testament scraped from azbyka.ru (7,105 verses). Full Bible requires finding and scraping/licensing an Arabic OT. Candidate sources: Van Dyke Arabic Bible (public domain), SVD. Requires a new scraper.

### 3.3 `syncordia.json` and `syncordia.js` are designed but not shipped

The schema is defined in CONCEPT.md and THEORY.md. `build_syncordia.py` is planned but not written. `website/js/syncordia.js` is planned but not wired in. Current UI enforces the `measure` rule for SRS (50 cards/session) but not the taboo/reward rules.

### 3.4 SRS implementation is inside `app.js`

The SM-2 algorithm currently lives in `app.js` rather than a dedicated `srs.js`. DESIGN.md's ideal split is planned, not realised. Extraction requires care because `localStorage` keys are shared.

### 3.5 Pipeline scripts not under `scripts/`

`build_lexicons.py`, `enrich_lexicons.py`, `fetch_*.py`, `convert.py` currently live at repo root. DESIGN.md shows them under `scripts/`. Reorganisation is planned but not done.

### 3.6 No automated tests

No test suite. No CI. `build_lexicons.py` and `enrich_lexicons.py` are long-running stateful processes — regressions would be invisible until a user notices. At minimum: schema-validation tests on `lexicon.json` and `bible.json`; round-trip tests on the IPA fallback chain; determinism tests on enrichment.

### 3.7 No per-language font audit

CSS loads default system fonts. For `dzveli`/`axali` (Mkhedruli) and `ar` (RTL with shaping), rendering on minority-platform browsers is unverified. No screenshot test matrix.

---

## 4. Scope-level limitations

### 4.1 Single-corpus focus

kSystem is designed around *the Bible*. The theoretical machinery — trivium, Syncordia, depth-traversal — is corpus-agnostic, but the data pipeline, genre taxonomy and geo-mapping are biblical-specific. Adapting kSystem to Quran, Avesta, Pāli Canon or classical Greek corpus requires re-engineering `fetch_*.py`, re-drawing `geo.json`, and re-thinking genre tags.

### 4.2 No authentication, no multi-user

All state is local-only (`localStorage`). Teaching scenarios where an instructor wants to see student progress are impossible without a backend. Deliberately excluded from v1.

### 4.3 No offline-first mobile story

The SPA runs offline after first load *on desktop*. Mobile installation (PWA manifest, service worker) is absent. Mobile users pay a ~64 MB first-load cost every visit.

### 4.4 No versioned content releases

`lexicon.json` is overwritten in place. There is no "kSystem 2026-Q2" vs. "kSystem 2026-Q3" snapshot. Reproducing a research claim against a past lexicon state requires consulting git history manually.

---

## 5. Strategic risks

### 5.1 "Clever combination" risk (raised in peer review)

Peer review (PEER_REVIEW_kSystem.md, 2026-03-29) rated novelty 5/10, warning that kSystem may be "an elegant combination of existing ideas (semantic networks, trivium, spaced repetition, ethics in design) without a breakthrough theoretical or technological core." Mitigation: a stronger original contribution is needed — candidates are (a) a formal metric on depth-traversal coherence, (b) an empirical study of Syncordia's effect on retention, (c) a novel alignment technique for ancient polysynthetic languages.

### 5.2 LLM-dependency for 7/8 lexicons

If Ollama or `llama3.2` changes output format, regeneration may be needed. Reproducibility of definitions requires pinning model versions. Currently not pinned beyond "llama3.2".

### 5.3 Single-maintainer project

Bus factor = 1. All knowledge of the pipeline, service units, and data topology lives with one person. Mitigation: this 9-file schema is partly an attempt to externalise that knowledge.

### 5.4 DOI 10.65649/* not indexed

Assets published under this DOI namespace are not yet indexed in Google Scholar (as of 2026-03-28). Discoverability risk for the Digital Trivium paper. Mitigation: also push to ORCID, ResearchGate, arXiv (preprint), Zenodo.

### 5.5 Public launch date (2026-09-27) is a fixed commitment

Background define pipeline has ~13 days of nominal runtime but any interruption resets the clock. A failure in June/July could slip launch. Mitigation: run a parallel safety generation on a second machine.

### 5.6 Reference-audit flags (REFERENCE_AUDIT_kSystem.md)

Three PMID citations flagged as low-context-match. The citations themselves are valid; the *context* in which they are cited has weak keyword overlap with the papers. Not an error, but reviewers may flag it. Mitigation: tighten citation context strings.

---

## 6. Parked ideas (deferred, not dead)

- Graph visualisation of word connections (UPGRADE.md 2026-03-29). Deferred until after launch.
- Public REST/GraphQL API. Deferred — requires rate limiting and abuse protection.
- React Native / Expo mobile app. Deferred until desktop product is stable.
- Multilingual semantic embeddings (LaBSE, mE5-large, FAISS). Deferred — high compute cost vs. current static-first principle; possibly a separate service rather than inside `website/`.

```
## systemd snapshot
```
  UNIT                                                                                                      LOAD   ACTIVE SUB       DESCRIPTION
  sys-devices-pci0000:00-0000:00:02.0-0000:01:00.0-virtio1-net-eth0.device                                  loaded active plugged   Virtio 1.0 network device
  sys-devices-pci0000:00-0000:00:02.2-0000:03:00.0-virtio2-virtio\x2dports-vport2p1.device                  loaded active plugged   /sys/devices/pci0000:00/0000:00:02.2/0000:03:00.0/virtio2/virtio-ports/vport2p1
  sys-devices-pci0000:00-0000:00:02.5-0000:06:00.0-virtio5-host0-target0:0:0-0:0:0:0-block-sr0.device       loaded active plugged   QEMU_CD-ROM
  sys-devices-pci0000:00-0000:00:02.5-0000:06:00.0-virtio5-host0-target0:0:0-0:0:0:1-block-sda-sda1.device  loaded active plugged   QEMU_HARDDISK 1
  sys-devices-pci0000:00-0000:00:02.5-0000:06:00.0-virtio5-host0-target0:0:0-0:0:0:1-block-sda-sda14.device loaded active plugged   QEMU_HARDDISK 14
  sys-devices-pci0000:00-0000:00:02.5-0000:06:00.0-virtio5-host0-target0:0:0-0:0:0:1-block-sda-sda15.device loaded active plugged   QEMU_HARDDISK 15
  sys-devices-pci0000:00-0000:00:02.5-0000:06:00.0-virtio5-host0-target0:0:0-0:0:0:1-block-sda.device       loaded active plugged   QEMU_HARDDISK
  sys-devices-pci0000:00-0000:00:02.6-0000:07:00.0-virtio6-net-enp7s0.device                                loaded active plugged   Virtio 1.0 network device
  sys-devices-pci0000:00-0000:00:04.0-0000:00:04.0:0-0000:00:04.0:0.0-tty-ttyS0.device                      loaded active plugged   QEMU PCI 16550A Adapter (QEMU Virtual Machine)
  sys-devices-platform-ARMH0011:00-ARMH0011:00:0-ARMH0011:00:0.0-tty-ttyAMA0.device                         loaded active plugged   /sys/devices/platform/ARMH0011:00/ARMH0011:00:0/ARMH0011:00:0.0/tty/ttyAMA0
  sys-devices-platform-serial8250-serial8250:0-serial8250:0.1-tty-ttyS1.device                              loaded active plugged   /sys/devices/platform/serial8250/serial8250:0/serial8250:0.1/tty/ttyS1
  
```
## Code histogram
```
rs 4
ex 0
exs 0
heex 0
go 0
py 5
php 0
ts 0
tsx 0
js 1

```