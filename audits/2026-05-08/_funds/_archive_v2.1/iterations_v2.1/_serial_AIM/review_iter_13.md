# Review of AIM

## Verdict
**REJECT**

## Scores (1-5)
- Premise: 3
- Method: 3
- Evidence: 2
- Falsif: 3
- Deliv: 2
- Novelty: 4
- Risk: 3
- RefIntegrity: 1

## Checklist (✓/✗ + объяснение по каждому из 9 условий)

1. **Operationalised falsifiability (numeric thresholds)** — ✓  
   Числовые пороги представлены: PAM-13 Δ ≥ 5.4, α = 0.05, power = 0.80, N ≥ 55 per group, stopping rule, interim analysis. Однако внутри документа путаница с α (0.05 / 0.025 / 0.001), что снижает доверие, но формально условие выполнено.

2. **Pre-registration plan (OSF placeholder + date)** — ✓  
   OSF ID указан как placeholder (`osf.io/TBD` / `osf.io/XXXXX`), дата регистрации — 2026-09-01. Этого достаточно для pre-registration plan.

3. **Sample size calc (power analysis)** — ✓  
   Формула, подстановка (Z_α/2 = 1.96, Z_β = 0.84, δ = 5.4, σ = 10), результат n = 55 per group, учтён dropout 20% → 132 total. Sensitivity analysis для разных σ присутствует.

4. **Risk matrix ≥5 rows** — ✓  
   В документе есть несколько таблиц рисков, минимум 5 строк. Условие выполнено.

5. **Limitations section** — ✓  
   Отдельный раздел с 8 пунктами в CONCEPT.md, также в THEORY.md. Присутствует.

6. **Consortium / collaboration plan** — ✓  
   Указаны Lead PI, Co-I (holder names TBD), потенциальные партнёры с ролями (Insignia Health, Fraunhofer IGD, TSU, University of Copenhagen). Формально выполнено.

7. **Reference reality + match** — ✗  
   **Автоматический REJECT компонента.**  
   - Ссылка Tao et al. (2026) *Nature Medicine* — DOI TBD. Невалидный идентификатор.  
   - Ссылка Blumenthal-Lee (2024) *JAMA* — DOI TBD.  
   - Ссылка Tkemaladze (2026) *Longevity Horizon* — DOI TBD.  
   - В одном месте Hibbard 2004 указан с PMID 15527447 (это статья 2005 года), что является несоответствием содержания.  
   Reference Integrity Score = 1.

8. **No fabrication markers** — ✗  
   DOI TBD — это TBD на месте, где должен стоять конкретный идентификатор. Нарушение: placeholder для обязательных данных. Также в тексте встречаются дублирующиеся разделы с разными PMID, что указывает на невычищенные черновики.

9. **Internal consistency core docs** — ✗  
   - CONCEPT.md содержит дублированные разделы (Falsifiability, Sample size calculation — по два варианта).  
   - Противоречие в α: в одном месте α = 0.05 + Bonferroni 0.025, в другом p < 0.001, в THEORY.md α = 0.05 без коррекции.  
   - PMID Hibbard 2004 в разных местах разный (15333167 vs 15527447).  
   - Несогласованность между CONCEPT.md и THEORY.md по stopping rule и interim analysis.  
   - README.md ссылается на нереализованные провайдеры KIMI/Qwen как активные, что противоречит CONCEPT.md (vapor cleanup).  

**Вывод:** Ни одно из условий 7, 8, 9 не выполнено. Требуется REJECT.

## Reference audit (обязательная таблица)

| # | Цитата (короткая) | DOI/PMID/arXiv | Реальна? | Соответствует тексту? | Решение |
|---|---|---|---|---|---|
| 1 | Hibbard et al. (2004) PAM development | PMID 15333167 | Да (Health Serv Res 2004) | Да, если указан 2004 год. Но в одном месте ошибочно приписан PMID 15527447 (2005) — несоответствие | [REF_VERIFY: PMID 15527447 in Sample size calculation mismatched year] |
| 2 | Hibbard et al. (2005) short-form PAM | PMID 15527447 | Да (Health Serv Res 2005) | Да | OK |
| 3 | Hibbard et al. (2009) PAM scoring & MCID | Нет DOI/PMID (proprietary manual) | Невозможно проверить | Утверждение о MCID 5.4 — общепринято, но ссылка неверифицируема | Принять условно |
| 4 | Tao et al. (2026) Co-design, Nature Medicine | DOI TBD | Нет — несуществующий идентификатор | Утверждение "co-design > fine-tuning" не может быть проверено | **REJECT** (невалидный DOI) |
| 5 | Blumenthal D., Lee J. (2024) 4-zone HCI, JAMA | DOI TBD | Нет | Невозможно проверить существование фреймворка | **REJECT** |
| 6 | Tkemaladze J. (2026) Patient as a Project, Longevity Horizon | DOI TBD | Нет | Невозможно проверить трёхуровневую модель | **REJECT** |
| 7 | Mayo Clinic Laboratories Reference Values | URL (mayoclinic.org) | Да | Соответствует | OK |
| 8 | NIH MedlinePlus | URL (medlineplus.gov) | Да | Соответствует | OK |

## Top 5 text-level fixes (для перехода к REVISE_MAJOR)

1. **CONCEPT.md**: удалить дублированные разделы Falsifiability и Sample size calculation. Оставить единый согласованный блок с α = 0.05 (primary) + Bonferroni 0.025 (≤2 secondary). Убрать упоминание p < 0.001.

2. **CONCEPT.md** и **THEORY.md**: заменить все DOI TBD на реальные идентификаторы. Для Tao et al. (2026) и Blumenthal-Lee (2024) — как минимум arXiv ID или preprint DOI. Если статьи не опубликованы — убрать ссылки или указать "submitted". Для Tkemaladze (2026) — указать DOI журнала.

3. **CONCEPT.md** (Sample size calculation): исправить PMID 15527447 для Hibbard 2004 на 15333167. Указать, что Hibbard 2004 — это PMID 15333167, а 15527447 — Hibbard 2005. Проверить все вхождения.

4. **CONCEPT.md** (Pre‑registration plan): заменить placeholder OSF ID на резервированный ID (например, `osf.io/abcde`). Указать точную дату регистрации.

5. **README.md**: удалить упоминания нереализованных провайдеров KIMI и Qwen как активных. Привести в соответствие с CONCEPT.md (vapor cleanup). Добавить актуальные провайдеры (Groq, Gemini) и их статус.

## PACKET

Пакет составляет один core-документ AIM с четырьмя вложенными файлами: CONCEPT.md (основной), THEORY.md, PARAMETERS.md, KNOWLEDGE.md, README.md. Все файлы проанализированы совместно. Вердикт вынесен на основе критических нарушений ссылочной целостности и внутренней согласованности.