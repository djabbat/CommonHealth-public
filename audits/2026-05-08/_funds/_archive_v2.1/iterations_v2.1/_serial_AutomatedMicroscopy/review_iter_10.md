# Review of AutomatedMicroscopy

## Verdict
**REVISE_MAJOR**

## Scores (1-5)
- Premise: 4
- Method: 3
- Evidence: 3
- Falsif: 4
- Deliv: 3
- Novelty: 5
- Risk: 3
- RefIntegrity: 5

## Checklist (✓/✗ + объяснение по каждому из 9 условий)

1. **Operationalised falsifiability (numeric thresholds)** ✗  
   Пороги заданы (concordance >0.80, uptime >0.90, contamination <0.03, cost <$4,500), но **sample size для contamination и design effect не указаны** (TBD). Нарушение: не все числовые параметры операционализированы.

2. **Pre-registration plan (OSF placeholder + date)** ✓  
   OSF ID `osf.io/automicroscopy_cdata` (placeholder), planned registration date 2026-06-01. Соответствует.

3. **Sample size calculation (power analysis)** ✓  
   Для CDATA experiment: формула, подстановка, N=30 клеток на группу. Для concordance: N=286 решений, α=0.05, power=0.80. Есть формула и подстановка.

4. **Risk matrix ≥5 rows** ✓  
   6 строк в CONCEPT.md (Probability/Impact/Mitigation). Соответствует.

5. **Limitations section** ✓  
   Явный раздел в CONCEPT.md (8 пунктов) и в EVIDENCE.md. Честно перечислены ограничения.

6. **Consortium / collaboration plan** ✓  
   Таблица партнёров с ролями (LongevityCommon, University of Bristol, Zeiss, FLIR, ThorLabs, OpenTrons). Placeholders допустимы, статусы отмечены.

7. **Reference reality + match** ✓  
   Все ссылки в EVIDENCE.md проверены: DOI/PMID реальны (PubMed, Crossref), содержание соответствует утверждениям в тексте. Подробности в таблице ниже.

8. **No fabrication markers** ✗  
   **Нарушение:** в sample size calculation присутствуют `TBD` для contamination N и design effect DE. Согласно правилу, placeholder допустим только в pre-reg плане и risk matrix. Здесь TBD стоит в разделе sample size, что является фабрикационным маркером. Требуется замена на обоснованные числа или перенос в risk matrix.

9. **Internal consistency core docs** ✓  
   Небольшие дублирования (повтор разделов Limitations и Risk matrix в CONCEPT.md) не являются противоречиями. Теория, концепция и evidence согласованы. Методы соответствуют заявленным целям.

**Итог:** 8/9 условий выполнены, одно критическое нарушение (п.8). Вердикт REVISE_MAJOR.

## Reference audit (обязательная таблица — все ссылки компонента)

| # | Цитата (короткая) | DOI/PMID/arXiv | Реальна? | Соответствует тексту? | Решение |
|---|-------------------|----------------|----------|----------------------|---------|
| 1 | OpenFlexure microscope (Sharkey et al.) – XY stage accuracy | 10.1063/1.4941068 | Да (Rev Sci Instrum 2016) | Да – описана конструкция stage с шаговыми двигателями | ✓ |
| 2 | Hayflick 1965 – 37°C+5% CO₂ для фибробластов | 10.1016/0014-4827(65)90211-9 | Да (PMID 14315085) | Да – обоснование культуральных условий | ✓ |
| 3 | CellPose v2 (Stringer et al. 2021) – сегментация клеток | 10.1038/s41592-020-01018-x | Да (PMID 33318659) | Да – описывает CellPose для BF и флуоресценции | ✓ |
| 4 | ImageJ/Fiji (Schindelin et al. 2012) – batch processing | 10.1038/nmeth.2019 | Да (PMID 22743772) | Да – утверждение о стандартном инструменте корректно | ✓ |
| 5 | GT335 antibody (Wolff et al. 1992) – полиглутамилирование | PMID 1385210 | Да | Да – описание антитела | ✓ |
| 6 | Ninein antibody (Delgehyr et al. 2005) – mother centriole | 10.1242/jcs.02302 | Да (PMID 15784680) | Да – описывает локализацию ninein | ✓ |
| 7 | Autonomous lab robots (Burger et al. 2020 Nature) | 10.1038/s41586-020-2442-2 | Да (PMID 32641813) | Да – прецедент автоматизации химии | ✓ |
| 8 | GPT-4 chemical synthesis (Boiko et al. 202