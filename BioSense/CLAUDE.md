# CLAUDE.md — BioSense

## Контекст

**BioSense** — носимая мультисенсорная платформа: ЭЭГ · ВСР · Ольфакция.
Статус: v3.2 · ORGANISM BIOMARKER REVISION (2026-04-12).
⚠️ χ_Ze_eeg и χ_Ze_hrv не прошли эмпирические тесты → **Interim biomarker: SDNN + RMSSD**.
Расположение: `~/Desktop/CommonHealth/BioSense/`

## Авторитетный документ

`CONCEPT.md` v3.2 — единственный источник истины.

## КРИТИЧЕСКИЕ правила

1. **χ_Ze**: всегда "exploratory", НИКОГДА "validated" без peer-reviewed подтверждения
2. **SDNN/RMSSD**: interim organism score — validated (d=0.72, Fantasia N=40)
3. **v* = 0.45631**: эмпирический медиан, НЕ теоретически выведенный
4. **25–35 Гц**: выбран post-hoc → все эффекты exploratory до pre-registration
5. **IRB**: для будущего проспективного N≥200 — IRB approval ОБЯЗАТЕЛЕН до сбора данных
6. **Диапазон дат**: pre-registrations на OSF (osf.io/9m3yx и osf.io/qhtz7) — цитировать в §Methods

## Pre-registrations (OSF)

- BioSense χ_Ze alpha: https://osf.io/9m3yx
- EEG delta BTR Dortmund: https://osf.io/qhtz7 (NULL result: d=−0.002, p=0.995)

## Язык разработки

- Алгоритмы Ze-анализа → Python / Rust
- DSP: scipy, numpy (Python); реальное время → Rust
- Тексты → DeepSeek API
