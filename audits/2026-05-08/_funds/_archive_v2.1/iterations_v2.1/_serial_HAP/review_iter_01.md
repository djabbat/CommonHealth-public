# Review of HAP

## Verdict
**REVISE_MAJOR**

## Scores (1-5)
- **Premise:** 4  
- **Method:** 3  
- **Evidence:** 3  
- **Falsif:** 3  
- **Deliv:** 2  
- **Novelty:** 4  
- **Risk:** 3  

## Checklist (✓/✗ each + explanation)

1. **Operationalised falsifiability (numeric thresholds)** – ⚠️ Частично ✓  
   Числовые пороги приведены (α=0.01, power 0.80/0.90, Cohen's d, N для каждой группы).  
   **Проблема:** в разделе `Sample size calculation` используется α=0.05 (Z<sub>α/2</sub>=1.96) и не тот же α, что в Falsifiability. В тексте claimed «унифицировано α=0.01», но расчётная формула этого не отражает. Требуется пересчёт с α=0.01 (Z≈2.576) и исправление N.

2. **Pre-registration plan (OSF placeholder + date)** – ✓  
   OSF ID `osf.io/TBD`, дата 2026-06-01. Прописаны embargo, deviations, exclusion criteria. Допустимо как placeholder.

3. **Sample size calc (power analysis)** – ✓ (с оговорками)  
   Формула приведена, эффект, α, power указаны. N=15 (консервативно).  
   **Недостатки:** расчёт использует α=0.05 вместо заявленного α=0.01; размер эффекта взят из головы (d=2.5) без пилотных данных. Формально условие выполнено, но требует исправления.

4. **Risk matrix ≥5 rows** – ✓  
   7 строк рисков (включая отдельную строку про недоступность линии). Probability, Impact, Mitigation указаны. Строк ≥5.

5. **Limitations section** – ✓  
   7 пунктов, включая фундаментальную нефальсифицируемость, неопределённость механизмов, зависимость от дорогих экспериментов. Приемлемо.

6. **Consortium / collaboration plan** – ✓ (placeholder)  
   Список партнёров с ролями и статусом «To be confirmed». Для ERC AdG/Wellcome требуется минимум 2 подтверждённых письма поддержки. Здесь все TBD. Пока засчитываем как план, но директивно: до подачи необходимо **реальное подтверждение** хотя бы 2-3 партнёров.

7. **References PubMed/Crossref-verified or explicitly marked as pre-print** – ✗  
   В разделе `References` CONCEPT.md перечислены только 2 источника (Anderson 2014 — verified, Tkemaladze 2026 — pre-print). Остальная литература, цитируемая в тексте (Mertens 2017, Yanguas-Casas 2017, Huang 2016, Shang 2022 и др.) **не вынесена в список references** и не помечена как pre-print. В KNOWLEDGE.md ссылки есть, но не указан их статус верификации.  
   Требование: **каждая ссылка должна либо иметь DOI/PMID с подтверждением, либо быть явно помечена [PREPRINT]**. Нарушение критическое.

8. **No fabrication markers** – ✓  
   [REF_NEEDED] и [PMID_REMOVED] отсутствуют. Есть [PREPRINT] — допустимо.

## Top 5 text-level fixes (если НЕ FUND_AS_IS — что добавить/изменить)

1. **`CONCEPT.md` — раздел References**  
   Добавить полный список всех цитируемых источников с DOI/PMID или пометкой [PREPRINT]. Каждая ссылка, упомянутая в тексте (включая KNOWLEDGE.md), должна быть включена. Пример:  
   `Mertens, J., et al. (2017). Bile acids in CSF. Hepatology. DOI:... [PubMed:...]`  
   Если не удаётся подтвердить – `[PREPRINT]`.

2. **`CONCEPT.md` — раздел Sample size calculation**  
   Пересчитать N с использованием α=0.01 (Z<sub>α/2</sub>=2.576) вместо α=0.05. Унифицировать расчёт с порогами из раздела Falsifiability. Указать, что это placeholder до пилотных данных.

3. **`CONCEPT.md` — раздел Falsifiability**  
   Согласовать все числовые пороги: в предсказаниях #2-#5 сейчас указан α=0.01, но в расчёте sample size использован α=0.05. Исправить расчёт или привести α=0.05 везде (но тогда пересмотреть power и N). Желательно везде α=0.01.

4. **`CONCEPT.md` — раздел Consortium/partners**  
   Заменить хотя бы 2-3 «To be confirmed» на подтверждённых партнёров с указанием института, роли и даты письма поддержки. Если ещё нет – указать крайний срок получения писем (например, 2026-09-01) и план действий.

5. **`CONCEPT.md` — раздел Limitations**  
   Убрать пункт «Несоответствие α между разделами Falsifiability и Sample size calculation — исправлено», если это не исправлено. Вместо этого честно признать текущее несоответствие и указать, что оно будет устранено в финальной версии.

## PACKET
# HAP