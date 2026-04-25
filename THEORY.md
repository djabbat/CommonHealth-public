# THEORY — CommonHealth (Ecosystem)

**Назначение:** теоретическая основа экосистемы CommonHealth. Формальные определения для интеграции подпроектов.

**CommonHealth** — это thin social layer + ML-инфраструктура поверх научных подпроектов. Сама по себе CommonHealth не имеет формальной теории — она использует теории подпроектов:

| Подпроект | THEORY.md | Что формализовано |
|---|---|---|
| MCOA | `MCOA/THEORY.md` | Multi-Counter Architecture: `Aging_rate ≈ Σ α_i · Counter_i / threshold_i` |
| CDATA | `CDATA/THEORY.md` | Counter #1 (Centriolar): polyGlu PTM accumulation в материнской центриоли |
| HAP | `HAP/THEORY.md` | Hepato-Affective Primacy: печёночная регуляция эмоций |
| Ze | `Ze/CONCEPT.md` + `Ze Theory.pdf` | Entropic-Geometric TOE: `I(Z) = S(Z_real ‖ Z_model)` |
| BioSense | `BioSense/THEORY.md` | EEG/HRV/olfactory inputs to MCOA counters |
| FCLC | `FCLC/THEORY.md` | SecAgg+ + DP + Federated Shapley |
| Ontogenesis | `Ontogenesis/THEORY.md` | Онтогенез 0-25 лет |

## Интеграция

### 1. Aging score pipeline
```
Sensors (BioSense)  →  Counter inputs (HRV → input_A; etc.)
                              ↓
        MCOA: Aging_rate(tissue) = Σ_i w_i(tissue) · Counter_i
                              ↓
                    L_tissue per organ
                              ↓
              Personal aging dashboard (CommonHealth UX)
```

### 2. FCLC role
Веса `w_i(tissue)` калибруются через FCLC на федеративных клинических данных без передачи PII.

### 3. Ze role
Универсальный фреймворк энтропического времени (`t = ∫I dτ`). MCOA aging счётчики могут быть проинтерпретированы как локальные `I(τ)` в Ze-формализме.

## Health = ОРГАНИЗМ + ПСИХИКА + СОЗНАНИЕ + СОЦИУМ

4 фактора здоровья в UI и API. Health Score формула с априорными весами УДАЛЕНА (CORRECTIONS_2026-04-22). Используется напрямую `L_tissue` с tissue-specific `w_i`.

## Каноны

- CORRECTIONS_2026-04-22 (`_archive/audits/CORRECTIONS_2026-04-22.md`) — все утверждения после этой даты обновлены
- При расхождении: умbrella CONCEPT.md > подпроект CONCEPT.md
