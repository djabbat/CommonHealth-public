#!/usr/bin/env python3
"""
Минимальный DeepSeek-reasoner клиент для overnight аудита.

Usage:
  ds_review.py review  <packet.md> <out.md>
  ds_review.py plan    <packet.md> <review.md> <out.md>
  ds_review.py check   <packet.md> <plan.md>   <out.md>
"""
import os, sys, time, pathlib, traceback
from openai import OpenAI, APITimeoutError

KEY = None
for line in pathlib.Path(os.path.expanduser("~/.aim_env")).read_text().splitlines():
    if line.strip().startswith("DEEPSEEK_API_KEY="):
        KEY = line.split("=", 1)[1].strip().strip('"').strip("'")
        break
if not KEY:
    sys.exit("DEEPSEEK_API_KEY not found in ~/.aim_env")

CLIENT = OpenAI(api_key=KEY, base_url="https://api.deepseek.com/v1", timeout=600.0)
MODEL = "deepseek-reasoner"

REVIEWER_SYS = """Ты — строгий senior software architect / научный peer reviewer.
Стиль: журналы Nature / Cell, тон конференций уровня OSDI / SOSP.
Никаких комплиментов, только сухой критический разбор.

Структура ответа (на русском):
## VERDICT
Один из: ACCEPT | MINOR_REVISION | MAJOR_REVISION | REJECT

## SCORES (1-5, где 5 = превосходно)
- Architecture:
- Optimality:
- Structure / Modularity:
- Systematicity (cross-file consistency):
- Core-files vs code alignment:
- Stack-rule compliance (Rust+Phoenix only):
- Modernity of stack:
- Quality of processes / connections:

## CRITICAL ISSUES
Нумерованный список конкретных проблем с цитированием путей/файлов.

## MINOR ISSUES
То же.

## STRENGTHS
Что сделано хорошо (если есть).

## ROOT CAUSES
Если паттерны проблем повторяются — назвать корневую причину.
"""

PLANNER_SYS = """Ты — практичный staff-engineer. На основе peer review дай
КОНКРЕТНЫЙ план улучшений: каждый пункт = 1-3 строки + затронутые файлы.
Никаких размытых "улучшить структуру" — только actionable шаги.
Группировать по приоритету: P0 (блокеры), P1 (важно), P2 (nice-to-have).
Учитывать жёсткое правило: код только Rust (backend) + Phoenix LiveView (frontend).
Python допустим ТОЛЬКО для legacy OCR/PDF и для AIM ML-роутера.
Для каждого P0 указать: оценка трудоёмкости (S/M/L) + риск.
"""

CHECKER_SYS = """Ты — тот же строгий reviewer. Тебе дан исходный packet проекта
и улучшенный план. Проверь, закрывает ли план КРИТИЧНЫЕ замечания.
Структура ответа:
## VERDICT
ACCEPT | NEEDS_REVISION

## REMAINING_GAPS
Если NEEDS_REVISION — что именно ещё не закрыто.
## NOTES
Дополнительные замечания.
"""

def ask(system: str, user: str, max_retries: int = 4) -> str:
    last_err = None
    for attempt in range(max_retries):
        try:
            resp = CLIENT.chat.completions.create(
                model=MODEL,
                messages=[
                    {"role": "system", "content": system},
                    {"role": "user", "content": user},
                ],
                max_tokens=8000,
            )
            return resp.choices[0].message.content or ""
        except (APITimeoutError, Exception) as e:
            last_err = e
            msg = str(e).lower()
            if "rate" in msg or "429" in msg or "quota" in msg:
                time.sleep(60 * (attempt + 1))
            else:
                time.sleep(15 * (attempt + 1))
    raise RuntimeError(f"DeepSeek failed after {max_retries} attempts: {last_err}")

def main():
    mode = sys.argv[1]
    if mode == "review":
        packet, out = sys.argv[2], sys.argv[3]
        text = pathlib.Path(packet).read_text()
        ans = ask(REVIEWER_SYS, f"# Аудит packet\n\n{text}\n\nДай строгий peer review.")
    elif mode == "plan":
        packet, review, out = sys.argv[2], sys.argv[3], sys.argv[4]
        p = pathlib.Path(packet).read_text()
        r = pathlib.Path(review).read_text()
        ans = ask(PLANNER_SYS,
                  f"# Project packet\n{p}\n\n# Peer review\n{r}\n\n"
                  "Сформируй план улучшений по структуре P0/P1/P2.")
    elif mode == "check":
        packet, plan, out = sys.argv[2], sys.argv[3], sys.argv[4]
        p = pathlib.Path(packet).read_text()
        pl = pathlib.Path(plan).read_text()
        ans = ask(CHECKER_SYS,
                  f"# Project packet\n{p}\n\n# Improvement plan\n{pl}\n\n"
                  "Закрывает ли план критические замечания? Дай вердикт.")
    else:
        sys.exit(f"unknown mode: {mode}")
    pathlib.Path(out).write_text(ans)
    print(f"OK {mode} -> {out} ({len(ans)} chars)")

if __name__ == "__main__":
    try:
        main()
    except Exception:
        traceback.print_exc()
        sys.exit(2)
