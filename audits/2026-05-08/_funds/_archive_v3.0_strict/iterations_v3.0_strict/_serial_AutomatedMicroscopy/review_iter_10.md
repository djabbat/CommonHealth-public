# Review of AutomatedMicroscopy

## Verdict
**REVISE_MAJOR**

## Scores (1-5)
- Premise: 4
- Method: 2
- Evidence: 2
- Falsif: 3
- Deliv: 2
- Novelty: 4
- Risk: 3
- RefIntegrity: 5
- EvidenceDepth: 2
- MethodDepth: 2
- Reproducibility: 1

## Checklist (✓/✗ + объяснение по каждому из 12 условий)

1. **Operationalised falsifiability (numeric thresholds)** – ✗  
   Числовые пороги заданы для M1 (concordance ≥80%, α=0.05, power 0.80, N=286), но для M2–M4 и для contamination rate пороги есть, но без формальной статистической мощности и с TBD (N for contamination = TBD). Неполное выполнение.

2. **Pre-registration plan (OSF placeholder + date)** – ✗  
   Указаны OSF placeholder (osf.io/automicroscopy_cdata и osf.io/TBD — неконсистентно) и дата 2026-06-01. Конкретный ID отсутствует, два разных placeholder’а. Требуется единый конкретный ID.

3. **Sample size calc (power analysis)** – ✗  
   Для CDATA эксперимента расчёт выполнен (n=30/группа, Cohen’s d=0.75, α=0.05, β=0.20). Для contamination rate — N=TBD. Для uptime — N=180 фиксировано, но без расчёта мощности. Частично выполнено.

4. **Risk matrix ≥5 rows** – ✓  
   В CONCEPT.md 6 строк, в EVIDENCE.md ещё 7 строк. Все строки содержат Probability, Impact, Mitigation. Выполнено.

5. **Limitations section** – ✓  
   В CONCEPT.md и EVIDENCE.md явные разделы limitations, перечислены 8–12 пунктов. Выполнено.

6. **Consortium / collaboration plan** – ✗  
   Список партнёров есть (James Smith, Lena Zhang, OpenFlexure, Micro-Manager, University of Bristol и др.), но многие статусы «TBD» или «Contact initiated». Роли указаны, но конкретные соглашения и сроки отсутствуют. Требуется завершённость.

7. **Reference reality + match** – ✓  
   Все проверенные DOI/PMID (OpenFlexure, CellPose, ImageJ, GT335, Ninein, Burger, Boiko, Bran, Hayflick) реальны и соответствуют утверждениям. Идентификаторы производителей (Zeiss, FLIR) валидны. Замечаний нет.

8. **No fabrication markers** – ✗  
   В тексте множество TBD, placeholder’ов и [Reference needed] (например, OSF ID TBD, sample size TBD, cost verification TBD, partner status TBD, protocol links TBD). Это превышает допустимый предел для REVISE_MINOR (≤1 минорный пробел). Требуется замена на конкретные данные.

9. **Internal consistency core docs** – ✗  
   DESIGN.md, PARAMETERS.md, OPEN_PROBLEMS.md — пустые stubs. Методы, описанные в CONCEPT.md и EVIDENCE.md, не подкреплены детальной архитектурой и параметрами. Отсутствует AUTOMATED_MICROSCOPY_SETUP.md (не предоставлен). Согласованность не обеспечена.

10. **Evidence base depth (≥3 indep refs/claim, sys-review or meta-analysis, contradicting results addressed)** – ✗  
    Ни одно ключевое утверждение не подкреплено тремя независимыми источниками. Например, «AI-operated microscopy feasibility» — 3 ссылки (Burger, Boiko, Bran), но они по химии, не по микроскопии. «Low-cost DIY stage» — 1 ссылка (OpenFlexure). Систематический обзор или мета-анализ отсутствует. Противоречия не рассмотрены (указано «не найдены», но поиск несистематический). State-of-the-art упомянут без конкретных ссылок на коммерческие системы. Требуется полная переработка.

11. **Methodology depth (replication-ready protocol, SAP, controls, replication strategy, blinding)** – ✗  
    Step-by-step protocol описан поверхностно, без конкретных числовых настроек, команд и алгоритмов. Ссылка на AUTOMATED_MICROSCOPY_SETUP.md не раскрыта. SAP неполон: primary endpoint (concordance) есть, secondary endpoints перечислены, missing-data strategy только LOCF, sensitivity analyses не указаны. Replication strategy — split-sample и независимый датасет (TBD). Positive/negative controls указаны. Blinding описано. Но общая детализация недостаточна для независимой репликации. Требуется доработка.

12. **Reproducibility & open science (code, data, pre-reg, materials)** – ✗  
    Code availability: обещание GitHub «upon acceptance», без ссылки. Data deposit: Zenodo/OSF, DOI TBD. Pre-registration: OSF TBD. Materials transparency: protocols.io TBD, requirements.txt обещан. Все — placeholder’ы. Open science compliance отсутствует.

## Reference audit (обязательная таблица — все ссылки компонента)

| # | Цитата (короткая) | DOI/