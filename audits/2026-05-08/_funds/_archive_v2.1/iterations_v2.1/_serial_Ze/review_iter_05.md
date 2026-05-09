# Review of Ze

## Verdict
**REVISE_MAJOR**

## Scores (1-5)
- Premise: 3 (интересная интерпретативная рамка, но без независимых подтверждений остаётся умозрительной)
- Method: 1 (placeholders вместо конкретных чисел; гетерогенность данных игнорируется; пулинг некорректен)
- Evidence: 1 (все ключевые гипотезы не проверены; MCID на N=12 статистически недействителен; единственная «валидация» — Cuban EEG с I²=90.3%)
- Falsif: 2 (попытка есть, но effect sizes, SD и N — сплошь TBD; условие θ<log₂(d) для Теоремы 5.1 не выполняется для EEG)
- Deliv: 2 (нет конкретного плана deliverables; прототип BioSense на TRL 2)
- Novelty: 4 (связь Ze↔CDATA↔старение концептуально нова, но не обоснована)
- Risk: 2 (чрезвычайно высокий: отсутствие данных, fabrication markers, некорректный пулинг)

## Checklist (✓/✗ each + explanation)

1. **Operationalised falsifiability (numeric thresholds)** ✗  
   - Все effect sizes (d, r), SD и требуемые N — placeholders (TBD).  
   - Таблица «Operational falsifiability» содержит только заголовки без чисел.  
   - Для EEG (d=2) граница применимости теоремы нарушена (θ_Q ≥ 1.0), что делает χ_Ze эмпирическим, а не теоретическим.

2. **Pre-registration plan (OSF placeholder + date)** ✓ (формально)  
   - Указаны `osf.io/TBD` и даты 2026-09-01 / 2026-12-01.  
   - Однако никаких конкретных протоколов (критерии исключения, stopping rules) не описано.

3. **Sample size calc (power analysis)** ✗  
   - Формула приведена, но effect size (δ) и SD (σ) — TBD.  
   - Рассчитанные N (64, 84) основаны на placeholder-эффектах, не на реальных данных.  
   - Нет обоснования выбора effect size (например, из пилота N=12).

4. **Risk matrix ≥5 rows** ✓  
   - 6 строк (Insufficient power, Confounding, Replication failure, EEG hardware, MCID validation, Publication bias).  
   - Качественно: некоторые риски недооценены (например, fabrication markers не учтены).

5. **Limitations section** ✓  
   - 7 пунктов в CONCEPT.md 2.0.1 (generalizability, formalism, sample size, replication, pre-registration, confounders, publication bias).  
   - Хорошо, но некоторые пункты (например, «sample size») уже сигнализируют о недостаточности.

6. **Consortium / collaboration plan** ✓ (формально)  
   - Указаны PI и 4 партнёра (neurophysiology, biostatistics, clinical, data management) с пометкой TBD institution.  
   - Нет писем поддержки, нет конкретных имён или институтов.

7. **References PubMed/Crossref-verified** ✗  
   - Часть ссылок (на bioRxiv, Nature Communications) есть, но:  
     * Утверждение «v*_passive = 1−ln2» не подкреплено ссылкой на Shannon (1948).  
     * PMID 27330520 (Koo & Mae) — если и существует, то не верифицирован.  
     * В KNOWLEDGE.md есть «[Reference pending — placeholder]» — явное нарушение.  
   - Не все ссылки явно помечены как pre-print (например, BrainYears bioRxiv помечен, но остальные не проверены).

8. **No fabrication markers** ✗  
   - В KNOWLEDGE.md присутствуют:  
     `<!-- REF_AUDIT_2026-05-08: FABRICATION CLEANUP applied -->`  
     `[Reference pending — placeholder; will be replaced...]`  
     `[reference removed — to be replaced...]`  
   - Это явные fabrication markers, указывающие на удалённые/фабрикованные ссылки.  
   - Грантовые заявки с такими маркерами немедленно отклоняются.

## Top 5 text-level fixes (добавить/изменить)

1. **CONCEPT.md: «Operational falsifiability»** — Заменить все TBD на конкретные числа, выведенные из пилотных данных (хотя бы на основе Cohen d=0.5 для среднего эффекта) или литературы. Указать 95% CI для эффектов.  
   *Что вписать:* «v*_active > v*_passive: d = 0.4 (95% CI 0.1–0.7) based on Cuban EEG pilot (N=88); α=0.05, power=0.80 → N=52 per group. Final values to be updated after independent replication.»

2. **CONCEPT.md/KNOWLEDGE.md — Удалить все fabrication markers**  
   - Убрать комментарии `<!-- REF_AUDIT ... -->`.  
   - Вместо `[Reference pending — placeholder]` вставить реальную ссылку (найти замену) или удалить утверждение.  
   - Для `[reference removed]» подобрать валидный PMID/DOI или удалить абзац.

3. **CONCEPT.md: «Sample size calculation» — Исправить**  
   - Указать конкретные δ и σ с источниками (пилотные данные, литературные мета-анализы).  
   *Что вписать:* «δ = 0.05 (based on SD of χ_Ze in pilot N=12: SD=0.03 → d=1.67; conservative estimate d=0.5 used). σ = 0.03 (from pilot). N per group = 64.»

4. **CONCEPT.md: «Pre-registration plan» — Добавить конкретные протоколы**  
   - Описать критерии включения/исключения, stopping rules, первичный и вторичный анализ.  
   *Что вписать:* «Study 1: Inclusion: age 18–65, no psychiatric medication. Exclusion: HRV artifacts >20%. Stopping rules: if interim analysis at N=50 shows effect outside 0.2–0.8, study continues; otherwise halted. OSF ID placeholder osf.io/TBD_replaced_upon_submission, pre-registration date 2026-09-01.»

5. **CONCEPT.md: «v*_active» — Прекратить пулинг гетерогенных данных**  
   - Заменить pooled estimate на per-dataset bootstrap с указанием I² и предупреждением о невалидности объединения.  
   *Что вписать:* «Cuban EEG N=88: v*=0.456 (bootstrap 95% CI pending). Dortmund HRV N=60: v*=0.42 (CI pending). Cochran's Q=20.6, I²=90.3% → pooled estimate invalid. Future work must compare per-dataset with permutation test before pooling.»

## Дополнительные замечания (не входят в Top 5, но критичны)
- **Теорема 5.1 vs EEG:** Явно указать, что для d=2 и θ_Q=1.5 теорема не применима. χ_Ze — эмпирический биомаркер.  
- **MCID:** Убрать число 0.05, пока нет валидации на N≥50.  
- **Конкурент BrainYears:** В тексте признать превосходство по точности, но подчеркнуть интерпретируемость χ_Ze.  
- **Литературные ссылки:** Добавить Shannon (1948) для v*_passive; Horvath (2013) для эпигенетических часов; Koo & Mae (2016) для ICC.

**Заключение:** Заявка имеет интересную концептуальную основу, но на текущем этапе не удовлетворяет минимальным требованиям грантовых фондов из-за отсутствия конкретных численных параметров, наличия fabrication markers и некорректного обобщения данных. Требуется серьёзная переработка с фокусом на операционализацию и устранение следов фабрикации.