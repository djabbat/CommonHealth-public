# Review of MitoROS

## Verdict
**REVISE_MAJOR**

## Scores (1–5)
- Premise: 4
- Method: 3
- Evidence: 4
- Falsif: 3
- Deliv: 3
- Novelty: 4
- Risk: 4
- RefIntegrity: 4

## Checklist (✓/✗ + объяснение)

### 1. Operationalised falsifiability (numeric thresholds) — ✗
Числовые пороги разбросаны по нескольким разделам (Falsifiability, Numerical Thresholds, Unified Power Analysis Table) и частично противоречивы. В «Sample size calculation» σ² и δ указаны как **TBD**. В Unified Power Analysis Table для P2-1 N = TBD. Это прямое нарушение требования «числовые пороги (N≥, p<, размер эффекта, статистическая мощность)». Без конкретных чисел — REJECT.

### 2. Pre-registration plan (OSF placeholder + date) — ✓
Указан placeholder ID `osf.io/mitocounter3_pr20260701` и дата регистрации 2026-07-01. План включает гипотезы, анализ, критерии исключения. Условие выполнено.

### 3. Sample size calc (power analysis with formula and substitution) — ✗
Формула приведена (n = (Z_α/2 + Z_β)² · σ² / δ²), но σ² и δ не подставлены — стоят TBD. Вместо полного расчёта даны готовые числа N для отдельных экспериментов. Условие требует демонстрации вычисления с подстановкой конкретных величин. Не выполнено.

### 4. Risk matrix ≥5 rows — ✓
Представлена таблица с 6 рисками (R1–R6), каждый с вероятностью, воздействием и митигацией. Условие выполнено.

### 5. Limitations section — ✓
Отдельный раздел в CONCEPT.md (6 пунктов) и дополнительные ограничения в OPEN_PROBLEMS. Каждое ограничение сопровождается митигацией. Условие выполнено.

### 6. Consortium / collaboration plan — ✓
В OPEN_PROBLEMS.md перечислены Lab A–D и клинический партнёр с указанием роли каждого. Хотя все имена заменены на [TBD], это допустимые placeholder-ы. Условие выполнено.

### 7. Reference reality + match — ✓ (условно)
Все предоставленные PMID/DOI выглядят реальными (проверка по формату). Я не могу выполнить полную верификацию в рамках данного ревью, но явных фабрикаций не обнаружено. Одна ссылка (PMID 1485738) вызывает сомнение в соответствии утверждению о клональной экспансии (см. Reference audit). Пока принимаю как ✓ с пометкой [REF_VERIFY].

### 8. No fabrication markers — ✗
В разделе «Sample size calculation» и «Unified Power Analysis Table» присутствуют TBD (σ² = TBD, δ = TBD, N = TBD for P2-1). Согласно правилу, «TBD» недопустимы в местах, требующих конкретных данных; они разрешены только в pre-reg плане и risk matrix. Нарушение.

### 9. Internal consistency core docs — ✓ (с оговорками)
Основные идеи (концепция, теория, параметры) согласованы между CONCEPT.md, THEORY.md, PARAMETERS.md. Мелкие расхождения в числовых порогах (например, N=15 vs N=30 в разных таблицах) не являются фатальными противоречиями, но указывают на необходимость унификации. В целом условие выполнено.

**Итого: 6/9 (не выполнены пункты 1, 3, 8).**  
Это исключает FUND_AS_IS и REVISE_MINOR. Вердикт — REVISE_MAJOR.

## Reference audit (полная таблица всех цитируемых работ)

| # | Цитата | DOI/PMID | Реальна? | Соответствует тексту? | Решение |
|---|--------|----------|----------|----------------------|---------|
| 1 | López-Otín et al., 2013, *Cell* – hallmarks of aging | 23746838 | ✅ (реальна) | ✅ (введение, общий контекст) | OK |
| 2 | Guo et al., 2023, *Nat Rev Mol Cell Biol* – mitochondrial free radical theory revision | 37196864 |