# Review of AutomatedMicroscopy

## Verdict
**REVISE_MAJOR**

## Scores (1-5)
- Premise: 4
- Method: 3
- Evidence: 2
- Falsif: 2
- Deliv: 2
- Novelty: 4
- Risk: 3
- RefIntegrity: 5
- EvidenceDepth: 1
- MethodDepth: 3
- Reproducibility: 1

## Checklist (✓/✗ + объяснение)

1. **✗ Operationalised falsifiability (numeric thresholds)**  
   Частично. Для concordance (M1) есть N=286, α=0.05, power=0.80, δ=0.05. Для uptime и contamination — только пороги без power analysis. Для interpretability и reproducibility — критерии ≥99% / ≥90% без расчёта выборки. Требуются полные power-расчёты для всех falsification conditions.

2. **✓ Pre-registration plan (OSF placeholder + date)**  
   Указан OSF (osf.io/TBD) и planned date 2026-06-01. Placeholder допустим, но желательно конкретное имя. Засчитываю.

3. **✗ Sample size calc (power analysis)**  
   Частично. Есть расчёт для CDATA-эксперимента (n=30/group, Cohen's d=0.75, формула). Но для contamination, uptime, AI concordance (уже отдельно) — не хватает. Для вторичных целей sample size не указан.

4. **✓ Risk matrix ≥5 rows**  
   В CONCEPT.md — 6 строк, в EVIDENCE.md — 7 строк. Выполнено.

5. **✓ Limitations section**  
   Есть отдельный раздел в CONCEPT.md (8 пунктов) и в EVIDENCE.md. Выполнено.

6. **✓ Consortium / collaboration plan**  
   Таблица партнёров с ролями и статусами (LongevityCommon, Bristol, Zeiss, FLIR, ThorLabs, OpenTrons). Status частично "placeholder" или "TBD", но это допустимо на этой стадии.

7. **✓ Reference reality + match**  
   Все DOI/PMID проверены: реальны и соответствуют тексту. Ни одной фальшивой ссылки. Счёт RefIntegrity 5.

8. **✗ No fabrication markers**  
   Множество TBD, placeholder, [Placeholder: e.g., ...] в ключевых разделах: sample size (σ² TBD, δ TBD), falsification (contamination N=TBD), pre-registration (osf.io/TBD), consortium (статусы TBD), Evidence base (placeholders), Open science (TBD). Это нарушение.

9. **✓ Internal consistency core docs**  
   Мелкие расхождения (разные списки партнёров в CONCEPT.md и DESIGN.md, разные risk matrices), но не противоречат друг другу. Основные утверждения согласованы.

10. **✗ Evidence base depth (≥3 indep refs/claim, sys-review, contradictions)**  
    - Ключевое утверждение "AI-operated microscopy replicates industrial-grade" не имеет ≥3 независимых источников: есть аналоги в химии (Burger, Boiko, Bran), но нет прямых работ по LLM-микроскопии.  
    - Систематический обзор / мета-анализ отсутствует (упомянут "planned scoping review").  
    - Противоречия не обсуждены (сказано "no contradictory results identified", но это не анализ).  
    - State-of-the-art описан без цитирования.  
    Провал.

11. **✓ Methodology depth (replication-ready protocol, SAP, controls, replication strategy)**  
    - Step-by-step протокол есть (хотя краткий).  
    - SAP: primary endpoint (concordance), secondary, Bonferroni, LOCF для missing data.  
    - Controls: positive (human), negative (random).  
    - Replication: split-sample + independent dataset.  
    - Blinding: evaluators blinded.  
    Выполнено, хотя SAP мог бы быть детальнее.

12. **✗ Reproducibility & open science (code, data, pre-reg, materials)**  
    - Code: "GitHub upon acceptance, TBD".  
    - Data: "Zenodo/OSF, TBD".  
    - Pre-registration: "osf.io/TBD".  
    - Materials: "protocols.io, TBD".  
    Всё TBD, ни одного конкретного идентификатора или обещания с платформой. Провал.

## Reference audit

| # | Цитата | DOI/PMID | Реальна? | Соответствует тексту? | Решение |
|---|--------|----------|----------|------------------------|---------|
| 1 | OpenFlexure microscope | 10.1063/1.4941068 | ✅ | ✅ | OK |
| 2 | FLIR Blackfly S specs | flir.com (URL) | ✅ | ✅