# MAP.md — BioSense

```
BioSense/
├── CONCEPT.md          ← Авторитетный документ (v3.2)
├── README.md
├── CLAUDE.md           ← ⚠️ Критические ограничения χ_Ze
├── TODO.md
├── PARAMETERS.md       ← SDNN/RMSSD, v*, χ_Ze, Health Score веса
├── MAP.md
├── MEMORY.md
├── LINKS.md
├── KNOWLEDGE.md
├── UPGRADE.md
├── src/                ← DSP код (Python/Rust)
├── scripts/            ← Анализ датасетов
├── Materials/          ← Данные, JSON, PNG
└── biosense.sh         ← Запуск
```

## Три канала

1. **ЭЭГ** → χ_Ze (exploratory), бинарная скорость переключения
2. **ВСР** → SDNN + RMSSD (validated interim), χ_Ze_hrv (exploratory)
3. **Ольфакция** → Turin electron tunneling (R&D)
