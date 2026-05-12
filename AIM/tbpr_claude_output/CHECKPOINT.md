# TBPR pipeline — CHECKPOINT 2026-05-10 11:27

## Где остановились

**Проект:** E0 (CDATA/experiments/E0)
**Уровень:** concept (Level 1)
**Прогресс:** 4 из 9 агентов завершены, лучший результат 33/55 (agent 3 = flash-high)

| # | Agent           | Score | Status   |
|---|-----------------|-------|----------|
| 1 | flash-nothink   | 19/55 | done     |
| 2 | flash-low       | 28/55 | done     |
| 3 | flash-high      | 33/55 | done ★   |
| 4 | flash-max       | 33/55 | done     |
| 5 | chat-high       | —     | pending  |
| 6 | reasoner        | —     | pending  |
| 7 | reasoner-high   | —     | pending  |
| 8 | reasoner-max    | —     | pending  |
| 9 | pro-nothink     | —     | pending  |

Цель 45/55 ещё не достигнута; champion = flash-high 33/55.

## Что уже сделано в этом run-е

- ensure_core для E0 → сгенерированы THEORY.md / DESIGN.md / EVIDENCE.md из CONCEPT.md
  (записаны в `~/Desktop/LongevityCommon/CDATA/experiments/E0/` с header AUTO-GENERATED).
- 4 review + 4 fix отработаны для E0/concept.

## Что НЕ сделано

- E0/concept агенты 5-9 (5 агентов осталось)
- E0/core (Level 2) — будет запущен после concept
- E0/full (Level 3)
- Все 11 остальных проектов: AutomatedMicroscopy, CellLineageTree, EpigeneticDrift, Proteostasis, Telomere, MitoROS, BioSense, AIM, Ze, MCOA, CDATA

## Состояние файлов

- `tbpr_claude_output/orchestrator_state.json` — актуален.
- `tbpr_claude_output/orchestrator.log` — append-only.
- `tbpr_claude_output/debug.log` — append-only.
- `tbpr_claude_output/E0/concept/` — 4 review + 4 fixed_md.

## Как возобновить

### Вариант A — продолжить с E0/concept agent 5

```bash
# 1. удалить best-запись чтобы не пропустилась
python3 -c "
import json,pathlib
p = pathlib.Path('/home/oem/Desktop/LongevityCommon/AIM/tbpr_claude_output/orchestrator_state.json')
s = json.loads(p.read_text())
s['best'].pop('E0/concept', None)
p.write_text(json.dumps(s, indent=2, ensure_ascii=False))
"
# 2. запустить с E0/concept (resume пропустит то что есть, но best уже снят)
cd /home/oem/Desktop/LongevityCommon/AIM
nohup python3 scripts/tbpr_orchestrator.py --all > tbpr_claude_output/run_all.log 2>&1 &
```

⚠ Если просто запустить без шага 1 — current `--resume`-логика пропустит concept целиком (best=33 в state).

### Вариант B — принять best 33/55 и пойти дальше с E0/core

```bash
cd /home/oem/Desktop/LongevityCommon/AIM
nohup python3 scripts/tbpr_orchestrator.py --all --resume > tbpr_claude_output/run_all.log 2>&1 &
```

E0/concept пропустится (best 33/55 уже в state), сразу пойдёт ensure_core → core → full.

## Артефакты

- Скрипт: `~/Desktop/LongevityCommon/AIM/scripts/tbpr_orchestrator.py`
- README: `~/Desktop/LongevityCommon/AIM/CLAUDE_README.md`
- Output: `~/Desktop/LongevityCommon/AIM/tbpr_claude_output/`
- Архив прошлого run: `~/Desktop/LongevityCommon/AIM/tbpr_claude_output_archive_20260510_111733/`
