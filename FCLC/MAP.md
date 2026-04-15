# MAP.md — FCLC

## Структура проекта

```
FCLC/
├── CONCEPT.md          ← Авторитетный документ (v6.2)
├── README.md           ← Краткое описание
├── CLAUDE.md           ← Инструкции для Claude
├── TODO.md             ← Текущие задачи
├── PARAMETERS.md       ← DP, SecAgg+, FL параметры
├── MAP.md              ← Этот файл
├── MEMORY.md           ← Индекс памяти
├── LINKS.md            ← Ссылки
├── KNOWLEDGE.md        ← База знаний (FL, DP, SecAgg)
├── UPGRADE.md          ← План развития
└── src/                ← Rust/Python код (если есть)
```

## Компоненты архитектуры

```
Клиенты (локальные ноды)
    ↓ градиенты (DP-зашумлённые)
SecAgg+ агрегатор
    ↓ агрегированная модель
Координатор (Python/FastAPI)
    ↓ LEVF-оценка вклада
Реестр участников
```

## Статус трека (2026-04-16)

- EIC Pathfinder: ⏸ отменён
- Следующий: Longevity Impetus Grants / Rustaveli Foundation
