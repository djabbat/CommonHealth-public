# Review of AutomatedMicroscopy

## Verdict
**REVISE_MAJOR**

## Scores (1-5)
- Premise: 4
- Method: 3
- Evidence: 2
- Falsifiability: 3
- Deliv: 3
- Novelty: 4
- Risk: 3
- RefIntegrity: 2
- EvidenceDepth: 2
- MethodDepth: 3
- Reproducibility: 2

## Checklist (✓/✗ + объяснение по каждому из 12 условий)

1. **Operationalised falsifiability (numeric thresholds)** — ✗  
   Числовые пороги есть (concordance >0.80, uptime >0.95, contamination <0.03), но **sample size для contamination N = TBD (placeholder)** и **design effect** (DE = TBD в одном месте, DE=1.2 в другом). Нет унифицированного числового порога для всех гипотез. Частично выполнено, но с критическими пробелами.

2. **Pre-registration plan (OSF placeholder + date)** — ✓  
   Указан OSF ID `osf.io/automicroscopy_cdata` (placeholder) и дата 2026-06-01. Приемлемо для pre-proposal.

3. **Sample size calc (power analysis)** — ✗  
   Есть расчёт для M1 (N=286) и для CDATA (n=30/group), но **расчёт для contamination — N = TBD**. Формулы приведены, но подстановки неполные. Не выполнено в полном объёме.

4. **Risk matrix ≥5 rows** — ✓  
   Две независимые матрицы: по 6 и 7 строк. Все необходимые риски покрыты.

5. **Limitations section** — ✓  
   Два набора ограничений (по 8 пунктов), основные пробелы отмечены честно.

6. **Consortium / collaboration plan** — ✓  
   Список партнёров с ролями и статусами (University of Bristol, Zeiss, FLIR, ThorLabs, OpenTrons, James Smith, Lena Zhang, OpenFlexure, Micro-Manager). Часть — "Contact initiated" или "placeholder", но для pre-proposal достаточно.

7. **Reference reality + match** — ✗  
   В EVIDENCE.md проверенные ссылки реальны и соответствуют тексту. **НО** в CONCEPT.md раздел "Evidence base & meta-analysis" содержит **три невалидные ссылки** с шаблоном `[Author(s), Year, Journal, DOI TBD]` — они не существуют, не разрешаются и не соответствуют тексту. Это прямое нарушение условия.

8. **No fabrication markers** — ✗  
   **Явные TBD в ключевых местах:**  
   - Sample size для contamination: `N = TBD (placeholder)`  
   - σ² и δ в начальном расчёте: `TBD`  
   - Design effect: `DE = TBD` (в одном экземпляре)  
   - Ссылки: `[Placeholder: e.g., OpenTrons, µManager]`  
   - Consortium: `TBD (additional partner)`  
   - Pre-registration ID: `osf.io/TBD` (хотя есть и другой ID)  
   - Код репозиторий, Zenodo DOI: `TBD`  
   Наличие TBD в местах, где должны быть конкретные данные, является фабрикационным маркером по определению.

9. **Internal consistency core docs** — ✓ (с оговорками)  
   THEORY.md, EVIDENCE.md, CONCEPT.md не противоречат друг другу, но есть **дублирование разделов** (дважды Sample size, Risk matrix, Limitations, Consortium) — это не нарушение согласованности, но снижает качество. Stub-файлы (PARAMETERS, OPEN_PROBLEMS, DESIGN) пусты — для pre-proposal допустимо.

10. **Evidence base depth (≥3 indep refs/claim, sys-review or meta-analysis cited, contradicting results addressed)** — ✗  
    - **Ключевое утверждение:** "AI-operated microscopy achieves ≥80% trained-technician quality" — **нет ни одной прямой ссылки на AI для микроскопии**. Три ссылки (Burger, Boiko, Bran) — про AI в химии, не в микроскопии.  
    - **Систематический обзор или мета-анализ отсутствует.** В тексте прямо сказано: *"No systematic review or meta-analysis was performed"* и *"search was not systematic"*.  
    - **Противоречащие результаты не учтены.** В разделе "Contradictory evidence" написано: *"No contradictory results were identified"* — это неправдоподобно для AI-based микроскопии (известны проблемы с низкоконтрастными образцами, ошибки фокусировки).  
    - **Менее 3 источников на ключевое утверждение** (например, для environmental control — только стандартная практика, нет трёх независимых статей).

11. **Methodology depth (replication-ready protocol, SAP, controls, replication strategy)** — ✓  
    Присутствуют step-by-step протокол, SAP (primary endpoint, secondary, Bonferroni, missing data), positive/negative controls, split-sample + external validation (TBD), blinding/randomisation.  
    Минус: external validation site не указан (TBD).

12. **