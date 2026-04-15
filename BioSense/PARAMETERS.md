# PARAMETERS.md — BioSense v3.2

_Источник: CONCEPT.md v3.2 | Дата: 2026-04-16_
_⚠️ ORGANISM BIOMARKER REVISION — χ_Ze exploratory, SDNN/RMSSD validated_

---

## § Ze-анализ (exploratory)

| Параметр | Значение | Статус | Описание |
|----------|----------|--------|----------|
| `v*` | 0.45631 | Эмпирический | Медиан N=196 (Cuban N=88 + Dortmund N=108) |
| `v*_passive` | 1−ln2 ≈ 0.3069 | Аналитически доказан | Пассивное состояние |
| Формула v | N_S/(N−1) | — | N_S — переключений, N — точек |
| χ_Ze | 1−|v−v*|/max(v*,1−v*) | ∈ [0,1] | Exploratory aging index |
| Диапазон ЭЭГ | 25–35 Гц | Post-hoc | Требует pre-registration |

## § Interim Organism Score (validated)

| Биомаркер | Эффект (d) | n | Датасет | Статус |
|-----------|-----------|---|---------|--------|
| SDNN | 0.724 | 40 | PhysioNet Fantasia | ✅ Validated |
| RMSSD | аналогично | 40 | PhysioNet Fantasia | ✅ Validated |
| LF/HF ratio | d=−0.112 | 40 | PhysioNet Fantasia | Null result |

## § Health Score (интеграция в CommonHealth)

| Домен | Вес | Источник |
|-------|-----|---------|
| Организм | 0.40 | SDNN + RMSSD |
| Психика | 0.25 | Психологические шкалы |
| Сознание | 0.20 | ЭЭГ (когда χ_Ze validated) |
| Социум | 0.15 | GSS/ESS показатели |

## § ЭЭГ датасеты

| Датасет | n | Ссылка |
|---------|---|--------|
| Cuban EEG | 88 | Zenodo 4244765 |
| Dortmund Vital | — | ds005385 |
| MPI-LEMON | — | Pre-reg osf.io/9m3yx |
| Zenodo | — | 3875159 |
