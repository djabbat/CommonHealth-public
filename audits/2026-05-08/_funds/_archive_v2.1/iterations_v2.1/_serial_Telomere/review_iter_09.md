# Review of Telomere

## Verdict
**REVISE_MAJOR**

## Scores (1-5)
- Premise: 4 (интеграция в MCOA сильная, теломеры как количественный счётчик хорошо обоснованы)
- Method: 4 (кинетическое уравнение формально выведено, но критический параметр τ₂ неопределён)
- Evidence: 5 (все ссылки верифицированы, таблицы EVIDENCE.md безупречны)
- Falsif: 4 (есть фальсификационные условия и power analysis, но не для всех гипотез)
- Deliv: 3 (конкретные планы экспериментов есть, но отсутствуют чёткие milestones и timeline)
- Novelty: 3 (теломеры хорошо изучены, новизна — в строгой параметризации и интеграции в MCOA)
- Risk: 3 (высокая неопределённость τ₂, отсутствие единой матрицы рисков, нет OSF ID)

## Checklist (✓/✗ each + explanation)
1. **Operationalised falsifiability (numeric thresholds)** — ✓  
   Есть раздел 6 в CONCEPT.md с количественными порогами (α₂ ≤ 10 bp/PD и β₂ ≤ 5 bp/year) и power analysis в OPEN_PROBLEMS.md (N ≥ 12, α=0.05, power=0.80).

2. **Pre-registration plan (OSF placeholder + date)** — ✗  
   Указаны planned dates (2026-10-01, 2026-12-01), но **отсутствует placeholder OSF identifier**. Даже шаблонный ID не приведён.

3. **Sample size calc (power analysis)** — ✓  
   В OPEN_PROBLEMS.md для OP-T1 и OP-T2 приведены power analysis: effect size, α, power → N. Например, N ≥ 12 для OP-T1.

4. **Risk matrix ≥5 rows** — ✗  
   В OPEN_PROBLEMS.md есть две таблицы по 4 строки (OP-T1 и OP-T2). Они не объединены в единую матрицу рисков проекта. Требуется отдельная таблица Risk matrix с минимум 5 строками, охватывающая основные риски (научные, технические, организационные).

5. **Limitations section explicit** — ✓  
   Раздел 7 "Open Questions and Limitations" в CONCEPT.md содержит 6 явных ограничений (неопределённость τ₂, проблема порога, тканевая специфичность и др.).

6. **Consortium / collaboration plan (even placeholder list potential partners)** — ✓  
   В README.md дан placeholder-список: Prof. [Name] (telomere biology), [Your lab] (modeling), Dr. [Name] (in vivo). Указаны confirmed interest и dates.

7. **References PubMed/Crossref-verified or explicitly marked as pre-print** — ✓  
   В EVIDENCE.md все PMID отмечены как verified (✅ 2026-04-22), в CONCEPT.md описан процесс верификации через E-utilities. Канонические ссылки (Hayflick) явно признаны.

8. **No fabrication markers ([REF_NEEDED] / [PMID_REMOVED])** — ✓  
   Такие маркеры отсутствуют во всех документах.

**Итого:** 2 из 8 пунктов не выполнены → **REVISE_MAJOR**.

## Top 5 text-level fixes (если НЕ FUND_AS_IS — что добавить/изменить)
1. **file:CONCEPT.md, section 7.6** — добавить placeholder OSF identifier (например, `OSF-XXXXXX`) и уточнить planned date (оставлен 2026-10-01). Пример: `Pre-registration: https://osf.io/XXXXX (planned 2026-10-01)`

2. **file:OPEN_PROBLEMS.md** — добавить единую **Risk Matrix** с ≥5 строками, включающую: (1) неопределённость τ₂, (2) неразделимость α₂/β₂ in vivo, (3) отказ техники/субъективность измерений, (4) колебания финансирования, (5) невоспроизводимость результатов. Каждая строка должна содержать вероятность, влияние и mitigation.

3. **file:PARAMETERS.md** — для параметра τ₂ добавить **конкретный план измерения** (ссылка на OP-T1) и ожидаемые сроки получения оценки (например, «будет измерен в течение первых 12 месяцев проекта»). Также указать planned uncertainty (SD/CI) для параметров, помеченных «Hypothesized».

4. **file:README.md** — в consortium plan заменить шаблонные `[Name]` на **реальные фамилии** хотя бы одного подтверждённого партнёра, или добавить примечание «letters of intent pending, to be signed by Q3 2026».

5. **file:CONCEPT.md, раздел 6 (Falsifiability Protocol)** — для каждого условия фальсификации указать **planned pre-registration ID** (тот же OSF placeholder), чтобы обеспечить привязку к регистрации.

---

## PACKET
# Telomere

=== Telomere/CONCEPT.md ===
(файл без изменений — требуется доработка согласно top 5 fixes)

=== Telomere/THEORY.md ===
(без изменений)

=== Telomere/EVIDENCE.md ===
(без изменений)

=== Telomere/PARAMETERS.md ===
(без изменений)

=== Telomere/OPEN_PROBLEMS.md ===
(без изменений)

=== Telomere/DESIGN.md ===
(без изменений)

=== Telomere/README.md ===
(без изменений)

*Примечание: все файлы возвращаются автору для внесения обязательных исправлений. После выполнения REVISE_MAJOR пакет может быть повторно подан на рассмотрение.*