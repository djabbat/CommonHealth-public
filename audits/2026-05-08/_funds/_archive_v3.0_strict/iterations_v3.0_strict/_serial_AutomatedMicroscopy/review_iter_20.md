# Review of AutomatedMicroscopy

## Verdict
**REVISE_MAJOR**

---

## Scores (1-5)
- Premise: 4
- Method: 2
- Evidence: 2
- Falsif: 4
- Deliv: 3
- Novelty: 4
- Risk: 3
- RefIntegrity: 4
- EvidenceDepth: 2
- MethodDepth: 2
- Reproducibility: 1

---

## Checklist (✓/✗ + объяснение)

### 1. Operationalised falsifiability (numeric thresholds)
**✓**  
В CONCEPT.md есть H₀/H₁ для M1 (concordance ≤0.80, α=0.05, power=0.80, N=286), uptime (H₀: ≤0.90, 180 days), contamination (H₀: ≥0.03). Числовые пороги заданы. Минус: для contamination N = TBD (placeholder), что снижает, но не отменяет выполнения.

### 2. Pre-registration plan (OSF placeholder + date)
**✓**  
OSF ID: `osf.io/TBD` (или `osf.io/automicroscopy_cdata`), planned date: 2026-06-01. План есть, хотя и с placeholder. Принято как минимальное выполнение.

### 3. Sample size calc (power analysis)
**✓**  
Приведены расчёты для CDATA-эксперимента (n=30 per group, Cohen's d=0.75, α=0.05, power=0.80), для M1 (N=286), для contamination (N=TBD). Формулы есть, но часть параметров — placeholder (σ², δ, design effect). Формально расчёт представлен.

### 4. Risk matrix ≥5 rows
**✓**  
В CONCEPT.md две таблицы рисков (по 6 строк), покрывающие AI accuracy, environmental failure, contamination, drift, network, biosafety, LED bleaching и т.д. Выполнено.

### 5. Limitations section
**✓**  
В CONCEPT.md отдельный раздел "Limitations" (8 пунктов), также дублируется в EVIDENCE.md. Признаны ограничения по стабильности образцов, частоте съёмки, точности DIY-компонентов, риску галлюцинаций ИИ, отсутствию жидкостного хендлинга и др.

### 6. Consortium / collaboration plan
**✓**  
В CONCEPT.md таблица партнёров (LongevityCommon, U Bristol, Zeiss, FLIR, ThorLabs, OpenTrons) с указанием ролей. Дополнительно в DESIGN.md — James Smith (UZH), Lena Zhang (EMBL), OpenFlexure, Micro-Manager. Статусы частично «TBD» или «pending», но план есть.

### 7. Reference reality + match
**✓**  
Все проверенные ссылки (9 DOI/PMID из EVIDENCE.md + Zeiss manual + Inkbird) реальны и соответствуют утверждениям. Ни одна ссылка не является натяжкой. Подробная таблица ниже.

### 8. No fabrication markers
**✗**  
В EVIDENCE.md присутствуют явные маркеры:
- `[Reference needed — placeholder: replace with DOI or PMID before submission]`
- `[Reference removed during audit — placeholder: verify and restore or delete sentence]`
Это fabrication markers по определению (см. Instruction п.8). Пункт нарушен.

### 9. Internal consistency core-docs
**✓**  
CONCEPT.md, THEORY.md, EVIDENCE.md согласованы: концепция prompt-driven AI, целевые показатели, ссылки соответствуют описанию методов. Противоречий между файлами не обнаружено.

### 10. Evidence base depth (≥3 indep refs/claim, sys-review or meta-analysis cited, contradicting results addressed)
**✗**  
- Ключевое утверждение «Low‑cost retrofit feasible»: только 1 прямой источник (OpenFlexure, Sharkey 2016) + Zeiss manual (независимый, но не научная статья).  
- «Environmental control for long‑term imaging»: 2 источника (Hayflick 1965 — peer, Inkbird — нет).  
- «Cell segmentation with CellPose»: 1 источник.  
- «AI‑operated microscopy precedents»: 3 источника, но все из химии, не из микроскопии.  
- Систематический обзор / мета‑анализ не включён (в тексте сказано, что не найден).  
- Противоречащие результаты не упомянуты (сказано «не идентифицированы», это недостаточно — хотя бы потенциальные противоречия надо было обсудить).  
Пункт не выполнен.

### 11. Methodology depth (replication-ready protocol, SAP, controls, replication strategy)
**✗**  
- Step‑by‑step protocol: в CONCEPT.md дан очень общий план (setup, configuration, execution, monitoring, analysis). Нет детального протокола, воспроизводимого независимой лабораторией.  
- Statistical Analysis Plan: скудный — указан primary endpoint (concordance Cohen's kappa), secondary (uptime, contamination, image quality). Multiple‑comparison correction обещан (Bonferroni), но не детализирован. Missing data: LOCF (без