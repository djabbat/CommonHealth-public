## Verdict
**REVISE_MAJOR**

## Scores (1-5)
- Premise: 4  
- Method: 2  
- Evidence: 1  
- Falsif: 1  
- Deliv: 2  
- Novelty: 5  
- Risk: 5  

## Checklist (✓/✗ each + explanation)

1. **Operationalised falsifiability (numeric thresholds)** ✗  
   Таблица “Operational falsifiability for each hypothesis” содержит эффект-сайзы = `TBD`, N = `TBD`. Все параметры — placeholders, не конкретные числа. Без δ, α, power, N нет операционализируемой фальсифицируемости.  
   *Требование:* каждое утверждение должно быть сформулировано как `H₀: effect ≤ δ, α=0.05, power≥0.80, N≥X`.

2. **Pre-registration plan (OSF placeholder + date)** ✓  
   Указаны `osf.io/TBD` и даты `2026-09-01`, `2026-12-01`. Это формально выполнено, хотя даты выглядят нереалистично (сегодня 2026-05-08, до сентября 4 месяца, а исследования ещё не спланированы). Рекомендую сдвинуть на Q1 2027.

3. **Sample size calculation (power analysis)** ✗  
   Таблица “Sample size calculation” содержит `δ = TBD`, `σ = TBD`, `Calculated N = TBD`. Нет ни одного числового показателя. Формула есть, но без входных данных это не power analysis.

4. **Risk matrix ≥5 rows** ✓  
   Есть таблица с 6 строками, каждая содержит Probability, Impact, Mitigation.

5. **Limitations section** ✓  
   Приведены 7 пунктов в разделе “2.0.1. Limitations”. Достаточно подробно и честно.

6. **Consortium / collaboration plan** ✓  
   В разделе “1.1. Consortium / partners” перечислены планируемые партнёрства (Neurophysiology, Biostatistics, Clinical) с указанием TBD институтов. План есть, хотя и не детализирован.

7. **References PubMed/Crossref-verified** ✗  
   В KNOWLEDGE.md есть пометка о fabrication cleanup и удалении некоторых ссылок. Однако:
   - Ссылка на BrainYears (bioRxiv) — pre-print, помечена.
   - Ссылка на “Wearable Aging Clock — Nature Communications 2025” — не указан PMID/DOI в тексте.
   - Ссылка на “HRV-CV Digital Biomarker 2026” — Science for Me, не PubMed.
   - Ссылка на “WHOOP Age” — ненаучный источник.
   - В CONCEPT.md упоминается PMID 27330520 (Koo & Mae 2016) — вероятно, валидный, но остальные ссылки (Shannon 1948, Revicki et al. 2008) не верифицированы.  
   Требование: **все** ссылки должны быть либо снабжены валидным PMID/DOI, либо явно помечены как pre-print. Здесь этого нет. Необходимо провести аудит и заменить сомнительные ссылки.

8. **No fabrication markers** ✓  
   Явных маркеров `[REF_NEEDED]` или `[PMID_REMOVED]` в предоставленных текстах не обнаружено. Пометка о fabrication cleanup говорит о том, что эта проблема осознаётся и частично решена.

## Top 5 text-level fixes (в порядке приоритета)

1. **`CONCEPT.md:2.3.0 (Operational falsifiability)`**  
   Заменить `TBD` на конкретные числа: указать δ (напр., Cohen’s d=0.5 для первичной гипотезы), α=0.05, power=0.80, рассчитать N. Если нет пилотных данных, использовать литературные оценки или явно указать “will be set after pilot N=50 → pre-registration”.

2. **`CONCEPT.md:2.3.0 (Sample size calculation)`**  
   Аналогично: заполнить δ и σ хотя бы как консервативные оценки (напр., δ=0.5, pooled σ из литературы) и вычислить N. Убрать “TBD” во всех ячейках, кроме тех, где данные ещё не собраны (но тогда нужно дать план, когда они будут).

3. **`KNOWLEDGE.md` (верификация ссылок)**  
   Провести полный аудит: каждую ссылку проверить через PubMed/Crossref. Для неверифицируемых источников (Science for ME, The Manual) заменить на рецензируемые статьи или удалить. Pre-prints (bioRxiv) оставить с пометкой “pre-print”.

4. **`CONCEPT.md:2.3.0 (Pre-registration plan)`**  
   Изменить даты с 2026-09-01 на реально достижимые (2027-03-01 для первого исследования). Указать, что OSF ID будет зарегистрирован после утверждения гранта или в течение 3 месяцев после старта.

5. **`CONCEPT.md:2.3.0 (v*_active статистический план)`**  
   В разделе про v*_active уже указано, что I²=90.3% делает pooling некорректным. Зафиксировать это в тексте как основную проблему. Добавить: “We will report per-dataset bootstrap estimates (BCa CI) and only pool if I²<50% after meta-regression.” Убрать утверждение “v*≈0.456” без 95% ДИ.

---

### Общий комментарий
Проект амбициозен и содержит интересные теоретические построения. Однако для ERC AdG / Wellcome Discovery текущее состояние — недостаточная зрелость. Основные проблемы: (1) все числовые параметры — placeholder, (2) отсутствие верифицированных ссылок, (3) фальсифицируемость не operationalized. После исправления этих трёх пунктов можно повторно подавать на REVISE_MINOR. Рекомендую сначала собрать пилотные данные (N≥50), провести power analysis и пре-регистрацию, затем обновить документ.