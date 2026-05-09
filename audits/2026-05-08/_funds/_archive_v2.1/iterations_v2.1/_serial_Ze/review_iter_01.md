## Review of Ze

**Verdict:** **REVISE_MAJOR**  
*(Не выполнены 6 из 8 обязательных условий. Проект содержит интересную теоретическую рамку, но критически не готов к рассмотрению фондами ERC/Wellcome/NIH.)*

---

## Scores (1–5)

| Критерий | Оценка | Обоснование |
|----------|--------|-------------|
| **Premise** | 3 | Оригинальная интерпретативная рамка, но не выведена из первых принципов. Связь с биологией и старением — гипотетична. |
| **Method** | 1 | Нет операционализированной фальсифицируемости, нет пре-регистрации, выборка не обоснована. |
| **Evidence** | 1 | Пилот N=12–196 с высокой гетерогенностью (I²=90%). Все эффекты TBD. |
| **Falsif** | 1 | Пороги не заданы численно (все TBD), power analysis не completed. |
| **Deliv** | 1 | Нет консорциума, риск-матрицы, плана верификации. |
| **Novelty** | 4 | Идея дискретного счётчика собственного времени как универсального принципа — свежая. |
| **Risk** | 5 | Максимальный — отсутствие эмпирической базы, непроверенные гипотезы, fabrication markers. |

---

## Checklist (✓/✗)

| # | Условие | Статус | Объяснение |
|---|---------|--------|------------|
| 1 | **Operationalised falsifiability** (numeric thresholds) | ✗ | В таблице все effect size и N — «TBD». Нет числовых границ для отвержения гипотез. |
| 2 | **Pre-registration plan** (OSF placeholder + date) | ✗ | Указано «no studies pre-registered». Нет идентификатора OSF, нет даты. |
| 3 | **Sample size calculation** (power analysis) | ✗ | Формула есть, но δ, σ, N — TBD. Расчёт не завершён. |
| 4 | **Risk matrix** (≥5 rows) | ✗ | Отсутствует. |
| 5 | **Limitations section** | ✓ | Раздел 2.0.1 — явный, 6 пунктов (но нет упоминания fabrication markers). |
| 6 | **Consortium / collaboration plan** | ✓ (частично) | Список планируемых партнёров приведён, хотя все «TBD». Подходит как placeholder. |
| 7 | **References PubMed/Crossref-verified** или явно pre-print | ✗ | Ссылка на s4me.info — не PubMed, не pre-print. Часть референсов помечена как «Reference pending — fabrication». |
| 8 | **No fabrication markers** ([REF_NEEDED] / [PMID_REMOVED]) | ✗ | В KNOWLEDGE.md есть комментарий: «[Reference pending — will be replaced with valid PMID or removed entirely before submission]» — это fabrication marker. |

**Итог:** 2 ✓ из 8 (5 и 6 частично). **Необходимо исправить минимум 4 пункта (1,2,3,4,8).**

---

## Top 5 text-level fixes

1. **CONCEPT.md : Tables in §§ 2.3.0** — Заменить все «TBD» на реальные численные пороги (effect size, α, power, N) для каждой гипотезы. Например: `d=0.5, α=0.05, power=0.80 → N≥64 per group`. Без этого фальсифицируемость не операционализирована.

2. **CONCEPT.md : добавить Pre-registration block** — Вставить: `Pre-registration: OSF registration https://osf.io/XXXXXXXX planned for 2026-09-01. Hypothesis and analysis plan frozen prior to data collection.` (заменить X на валидный идентификатор).

3. **CONCEPT.md : вставить Risk matrix** — Добавить таблицу ≥5 строк:  
   | Риск | Вероятность | Влияние | Митигация |
   |------|------------|---------|-----------|
   | Неподтверждение v*_active | высокая | высокое | A/B тест на независимой когорте |
   | … | … | … | … |

4. **KNOWLEDGE.md : удалить все fabrication markers** — Заменить `[Reference pending — will be replaced with valid PMID or removed entirely before submission]` на реальную ссылку PubMed/DOI или полностью удалить утверждение. Привести все ссылки к формату PubMed (PMID) или явно пометить как pre-print.

5. **CONCEPT.md : раздел «Operational definitions»** — Указать, как измеряется каждая гипотеза: какой прибор, какой протокол, какой минимальный эффект считается значимым. Без этого «v*_active», «χ_Ze» — пустые ярлыки.

---

**Заключение:** Проект Ze демонстрирует амбициозную теоретическую конструкцию, но на текущем этапе **не соответствует минимальным требованиям фондов** по воспроизводимости, формальной фальсифицируемости и прозрачности. После исправления перечисленных 5 пунктов (особенно 1–4, 8) может быть повторно подан как REVISE_MINOR. До этого — **REVISE_MAJOR**.