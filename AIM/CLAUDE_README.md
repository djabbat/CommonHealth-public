Claude, пиши всё в лог-файл:
- каждый вызов API (модель, effort, время, ошибки)
- каждый полученный score
- каждый фикс (сколько символов, что изменил)
- каждый вердикт
- свои решения и почему

Лог: `~/Desktop/LongevityCommon/AIM/tbpr_claude_output/debug.log`

Формат:
[time] MODEL=deepseek-v4-flash EFFORT=high ACTION=review STATUS=ok SCORE=34/55 DURATION=41s
[time] MODEL=flash-high ACTION=fix CHARS=13948 STATUS=ok
[time] DECISION: agent 3 (flash-high) gave best score 33/55, keeping as champion
