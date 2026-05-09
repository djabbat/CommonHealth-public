# Review of AIM

## Verdict
**REVISE_MAJOR**

## Scores (1-5)
- Premise: 4  
- Method: 3  
- Evidence: 2  
- Falsifiability: 4  
- Deliv: 3  
- Novelty: 4  
- Risk: 3  
- RefIntegrity: 1  

## Checklist (✓/✗ + объяснение по каждому из 9 условий)

### 1. Operationalised falsifiability (numeric thresholds)
**✓**  
Пороги заданы: PAM-13 Δ ≥ 5.4 points (MCID), α = 0.05 (two-sided), power = 0.80, N ≥ 55 per group, stopping rule (AE >5%), interim analysis с O'Brien-Fleming. В THEORY.md есть согласованные значения.  
*Замечание:* в CONCEPT.md дважды повторяется блок с разными PMID для Hibbard 2004 – это создаёт путаницу, но сами числа корректны.

### 2. Pre-registration plan (OSF placeholder + date)
**✓**  
Указан OSF ID `osf.io/TBD` (placeholder), planned date 2026-09-01, описан дизайн, primary outcome, analysis plan. Единственный допустимый placeholder в core-документах.

### 3. Sample size calc (power analysis)
**✓**  
Формула: n = (Z_α/2 + Z_β)²·2σ²/δ². Подстановка: Z=1.96, Z=0.84, δ=5.4, σ=10 → n≈55. Sensitivity analysis для σ=8,10,12. Justification через Hibbard 2004.  
*Проблема:* justification ссылается на PMID 15527447 (Hibbard 2005) в одном месте и на 15333167 (Hibbard 2004) в другом – несогласованность.

### 4. Risk matrix ≥5 rows
**✓**  
В CONCEPT.md приведена таблица из 7 строк (Recruitment, LLM hallucination, Data privacy, PAM-13 license, Reference integrity, Dropout, σ estimate). Probability/Impact/mitigation указаны.

### 5. Limitations section
**✓**  
Явный раздел в CONCEPT.md (8 пунктов: single-centre, short follow-up, self-report, digital literacy, Hawthorne, placebo, σ assumption, reference integrity). Дублируется в THEORY.md.

### 6. Consortium / collaboration plan
**✗**  
Список партнёров есть (Lead PI TBD, Co-I Clinical TBD, Co-I Technical TBD, Insignia Health, Fraunhofer IGD, TSU, Univ. Copenhagen), но **большинство имён – placeholders**. По условию placeholders допустимы только в pre-reg плане и risk matrix. Здесь они в consortium – нарушение.  
*Дополнительно:* letters of support pending – это нормально, но имена должны быть указаны.

### 7. Reference reality + match
**✗**  
Несколько критических нарушений:
- Hibbard 2009 (PAM scoring, proprietary manual) – нет DOI/PMID/ISBN. Неверифицируема.  
- Tao et al. 2026 (Nat Med) – указан "DOI TBD". Нет идентификатора.  
- Blumenthal-Lee 2024 (JAMA) – "DOI TBD".  
- Tkemaladze 2026 (Longevity Horizon) – "DOI TBD".  
- Hibbard 2004 в одном из блоков указан с PMID 15527447 (ошибка, хотя рядом есть правильный).  
Все эти ссылки либо не имеют идентификатора, либо не соответствуют (2005 PMID отнесён к 2004 работе).  
*Положительно:* Hibbard 2004 (PMID 15333167) и Hibbard 2005 (PMID 15527447) – реальны. Mayo Clinic, MedlinePlus, WHO – URL работают.

### 8. No fabrication markers
**✗**  
Помимо consortium placeholders, в тексте встречаются "osf.io/XXXXX" (допустимый), "DOI TBD" (недопустимый в ссылках), "Name TBD" в consortium.  
В разделе Limitations упоминание "one reference (Hibbard 2004) had incorrect PMID" – это признание ошибки, но не маркер фабрикации.  
Тем не менее, наличие TBD вне pre-reg/risk matrix – нарушение.

### 9. Internal consistency core docs
**✗**  
- **Аналиты:** PARAMETERS.md = 59, KNOWLEDGE.md = 71. Прямое противоречие.  
- **Sample size calculation:** в CONCEPT.md дважды приведён расчёт с разными PMID для одной работы.  
- **α-уровень:** THEORY.md упоминает p<0.001 (удалено в CONCEPT.md), что говорит о несинхронизации версий.  
- **Фреймворк L3:** THEORY.md утверждает, что CONCEPT.md является источником; в CONCEPT.md есть §0 с отсылкой на Tkemaladze – согласовано, но другие несоответствия подрывают доверие.

## Reference audit (обязательная таблица — все ссылки компонента)

| # | Цитата (короткая) | DOI/PMID/arXiv | Реальна? | Соответствует тексту? | Решение |
|---|---|---|---|---|---|
| 1 | Hibbard et al. (2004) Health Serv Res 39(4 Pt 1):1005–26 | PMID 15333167 (DOI 10.1111/j.147