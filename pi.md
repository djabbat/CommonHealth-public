# pi.md — LongevityCommon

Памятка агента pi. Создан 2026-05-11.

## Суть

Hypothesis-stage framework: 5 подпроектов (MCOA, CDATA, Ze, BioSense, FCLC) + социальный слой (server/web/realtime). Все AUC — exploratory, не confirmatory. Pre-registered тесты χ_Ze — NULL (deprecated). Публикации НЕ peer-reviewed.

## EIC Pathfinder Challenges 2026

- Deadline: **28 Oct 2026**, бюджет до €4M
- Variant C: WP1-5 (FCLC + Ze + CDATA + BioSense + Aqtivirebuli), €3.0M / 36 mo
- Партнёры: Geiger (Ulm), Janke Curie, COSIC/Preneel KU Leuven

## Структура

18 поддиректорий. См. CONCEPT.md для authority order.

## Связь с сервером

- `AIM/` → симлинк → `~/hive_queen/AI` (на сервере)
- Локальная копия (`~/Desktop/LongevityCommon/`) — сервер (`~/LongevityCommon/`) — git-оригинал

## Команды

```bash
# Пуш на сервер
cd ~/Desktop/LongevityCommon && git push server main
```
