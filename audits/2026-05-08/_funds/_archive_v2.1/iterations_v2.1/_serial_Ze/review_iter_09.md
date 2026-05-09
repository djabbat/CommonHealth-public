# Review of Ze

## Verdict
**REVISE_MAJOR**

Обоснование: Не выполнены **4 из 8** обязательных text-level условий (операционализированная фальсифицируемость с числовыми порогами, обоснование размера выборки с power analysis, верификация ссылок, отсутствие fabrication markers). Наличие меток `[Reference removed]` автоматически disqualifизирует заявку для FUND_AS_IS или REVISE_MINOR. Проект имеет оригинальную концепцию, но эмпирическая база крайне слаба, а методологические дефекты (I²=90.3%, статистически недействительные пороги) требуют фундаментальной переработки до повторной подачи.

---

## Scores (1–5)

| Критерий | Оценка | Комментарий |
|----------|--------|-------------|
| **Premise** | 2 | Ze Vectors — творческая рамка, но претензия на теорию всего без формальной аксиоматики и связных предсказаний. |
| **Method** | 2 | Математический формализм (POVM, Hilbert space) есть, но EEG-аппроксимация эквивалентна классической вероятности (диагональная ρ). Power analysis содержит только placeholder'ы. |
| **Evidence** | 1 | Единственный эмпирический «результат» v*≈0.456 получен при I²=90.3% (сливать данные нельзя). Пилотные N≤196 не дают никакой статистической уверенности. |
| **Falsif** | 2 | Риск-матрица и раздел falsifiability есть, но все effect sizes, σ, N — TBD. Фактическая фальсифицируемость отсутствует. |
| **Deliv** | 2 | Consortium описан на уровне «TBD institutions», нет писем поддержки, нет детального плана работ по WP с вехами. |
| **Novelty** | 4 | Идея оригинальна: соединение квантовой информации, байесовского вывода и старения через единый счетчик. |
| **Risk** | 1 | Исключительно высокий: ни одно ключевое утверждение не подтверждено, есть явные fabrication markers, нет репликаций, отсутствуют пре-регистрации. |

---

## Checklist (✓/✗ + explanation)

| № | Требование | Статус | Объяснение |
|---|------------|--------|------------|
| 1 | **Operationalised falsifiability (numeric thresholds)** | ✗ | Все пороги (effect size, σ, N) указаны как `TBD`. Для v*_active нет 95% CI, нет bootstrap-оценки. Risk matrix есть, но не заменяет числовых гипотез. |
| 2 | **Pre-registration plan (OSF placeholder + date)** | ✓ | OSF ID placeholder `osf.io/TBD` и даты 2026-09-01, 2026-12-01 указаны. Формально выполнено (хотя без конкретного ID). |
| 3 | **Sample size calc (power analysis)** | ✗ | Формула есть, но все параметры (δ, σ, N) TBD. Без числовых значений power analysis не операционален. |
| 4 | **Risk matrix ≥5 rows** | ✓ | 6 rows, probability × impact × mitigation указаны. |
| 5 | **Limitations section** | ✓ | Раздел 2.0.1: 7 пунктов, включая малые выборки, отсутствие пре-регистрации, confounding. |
| 6 | **Consortium / collaboration plan** | ✓ | Список потенциальных партнёров (пусть TBD institutions) и planned consortium structure. |
| 7 | **All references PubMed/Crossref-verified or marked as pre-print** | ✗ | В KNOWLEDGE.md есть ссылки на figshare, s4me.info, themanual.com — не PubMed/Crossref. Есть пометки `Reference removed — citation pending verification`. Не все ссылки верифицированы. |
| 8 | **No fabrication markers ([REF_NEEDED] / [PMID_REMOVED])** | ✗ | В KNOWLEDGE.md явно присутствуют строки `[Reference removed — citation pending verification. Will be replaced with valid PMID or removed entirely.]` и `[reference removed — to be replaced with valid PMID or removed entirely]`. Это fabrication markers. |

**Итог:** 4 из 8 выполнены → обязательный REVISE_MAJOR.

---

## Top 5 text-level fixes (что добавить/изменить)

### 1. `CONCEPT.md` — Удалить все fabrication markers
- **Найти:** строки `[Reference removed — citation pending verification...]` и `[reference removed — to be replaced with valid PMID or removed entirely]`
- **Заменить на:** либо реальный PMID/DOI после верификации, либо удалить предложение целиком. Никаких «pending» маркеров. Если ссылка не верифицирована — убрать.
- **Пример:** `[Reference removed — citation pending verification. Will be replaced with valid PMID or removed entirely.]` → (удалить всю строку или вставить `[Preprint: doi.org/TBD]` с явным статусом «preprint»).

### 2. `CONCEPT.md` — Заполнить numerical thresholds в разделе «Operational falsifiability»
- **Текущее:** все эффекты `d = TBD`, `σ = TBD`, `N = TBD`.
- **Требование:** Для каждой гипотезы указать конкретный ожидаемый effect size с обоснованием (например, из литературы или пилота), α (0.05), power (0.80), расчёт N. Даже если оценка грубая — должна быть не TBD.
- **Пример для v*_active:** «Based on Cohen's convention, a medium effect size d=0.5 is assumed. With α=0.05, power=0.80, required N=64 per group. After pilot data (N=12), the observed d=1.69 (Cuban cohort) suggests N=12 would be sufficient for Cuban-specific comparison, but cross-cohort generalisability is unknown due to I²=90%».

### 3. `CONCEPT.md` — Исправить v*_active: перестать объединять гетерогенные датасеты
- **Текущее:** v*_active = 0.456 (медиана pooled N=196).
- **Проблема:** I²=90.3%, pooling статистически некорректен.
- **Требование:** Заменить на per-dataset оценки с 95% CI, указать Cochran Q и I². Во всех текстах использовать формулировку: «v*≈0.456 is an estimate from the Cuban EEG cohort (N=88, 95% CI pending); cross-cohort generalisability is uncertain (I²=90.3%). Determining universality is a primary objective of WP1».

### 4. `CONCEPT.md` — Убрать клинические пороги χ_Ze до валидации
- **Текущее:** были пороги 0.80/0.60/0.40, затем заменены на TBD. Но в некоторых местах (например, в таблице «clinical thresholds» в PARAMETERS.md) остались числа 0.35–0.45 и т.д.
- **Требование:** Все пороги, основанные на N<50, должны быть либо заменены на TBD, либо явно помечены как «preliminary, not clinically validated». НЕ использовать MCID=0.05 (N=12 — статистически недействительно).

### 5. `CONCEPT.md` — Явно указать статус теоремы 5.1 для EEG
- **Текущее:** теорема 5.1 (Born rule минимизирует T-события) используется без оговорки.
- **Проблема:** Условие θ_Q < log₂(d) для d=2 и θ_Q=1.5 не выполняется → теорема неприменима к EEG (H=ℂ²).
- **Требование:** Вставить дисклеймер: «In the EEG application (d=2, θ_Q=1.5), Theorem 5.1 is not applicable because θ_Q ≥ log₂(2)=1.0. Therefore, χ_Ze is an empirically motivated biomarker, not a theorem-derived quantity. A theoretical derivation for binary EEG states is in preparation.»

---

**Резюме:** Проект имеет потенциал, но текущая версия не удовлетворяет ни одному жёсткому требованию ERC AdG / NIH R01. Без устранения fabrication markers и предоставления конкретных числовых гипотез подача в указанные фонды невозможна. Рекомендуется сначала опубликовать пилотные данные в рецензируемом журнале, а затем подавать грант на основе валидированных эффектов.