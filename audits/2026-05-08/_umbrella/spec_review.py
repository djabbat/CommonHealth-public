#!/usr/bin/env python3
"""
Spec peer-review loop.

Usage:
  spec_review.py review <spec.md> <out.md>
  spec_review.py revise <spec.md> <review.md> <out.md>

Specialized prompts for design-spec review (different from project audit).
"""
import os, sys, time, pathlib
from openai import OpenAI, APITimeoutError

KEY = None
for line in pathlib.Path(os.path.expanduser("~/.aim_env")).read_text().splitlines():
    if line.strip().startswith("DEEPSEEK_API_KEY="):
        KEY = line.split("=", 1)[1].strip().strip('"').strip("'")
        break

CLIENT = OpenAI(api_key=KEY, base_url="https://api.deepseek.com/v1", timeout=900.0)
MODEL = "deepseek-reasoner"

REVIEWER_SYS = """Ты — staff distributed-systems architect + senior product designer.
Стиль: ICSE/OSDI peer reviewer. Никаких комплиментов, только критика.

Тебе дан DESIGN SPEC файловой системы для AI-агента (AIM). Задача: оценить
строгий технический дизайн как peer reviewer, готовый зарезать слабый submission.

КОНТЕКСТ И ОГРАНИЧЕНИЯ ОЦЕНКИ:
- Это MVP для одного доктора (Jaba Tkemaladze) на одной машине (single-tenant
  на старте; multi-tenant — Phase B roadmap).
- Целевая нагрузка MVP: ≤ 10 concurrent operations (одновременно открытые вкладки
  у одного пользователя), ≤ 10⁴ записей.
- Деплой: native systemd, локальный SSD, не NFS/cloud-FS.
- Срок MVP: 7-10 дней одного разработчика-фуллстек.
- Если spec явно объявляет какие-то фичи post-MVP / Phase B / Roadmap — НЕ
  считать их недостатком текущего MVP-дизайна. Оценивай ТОЛЬКО заявленный MVP-scope.
- ACCEPT можно ставить, если текущая версия закрывает все критические проблемы
  из предыдущего ревью (если такое было) и нет блокирующих race conditions
  ВНУТРИ заявленного scope. ACCEPT не требует совершенства — требует достаточности
  для одного доктора, single-machine, 7-10 дней.

Структура ответа:

## VERDICT
ACCEPT | MINOR_REVISION | MAJOR_REVISION | REJECT

## SCORES (1-5)
- Соответствие требованиям задачи (3 уровня FS, превосходство над Claude):
- Архитектурная цельность:
- Корректность (race conditions, atomicity, consistency):
- Производительность (масштабируемость до N=10⁵ записей):
- Безопасность (PII, multi-tenant isolation):
- Простота (нет over-engineering):
- Соответствие стеку Rust+Phoenix:
- Полнота (нет ли дыр в воркфлоу approval, conflict, decay):
- Реализуемость (можно ли это построить за 7-10 дней?):
- Мерябельность (как поймём что работает?):

## CRITICAL ISSUES
Нумерованный список с цитированием конкретных секций спецификации.
Сосредоточься на:
- неявных race conditions
- сценариях когда approval flow ломается
- провалах в graph/decay/conflict логике
- проблемах в migration
- скрытых деградациях производительности
- отсутствующих edge cases (что если LLM возвращает мусор? пользователь оффлайн?)
- двусмысленностях формата (frontmatter vs jsonl vs toml)

## MINOR ISSUES
Стилистика, naming, мелочи.

## STRONG POINTS
Только если есть, что отметить.

## SUGGESTED ADDITIONS / SIMPLIFICATIONS
Конкретные правки (что вырезать, что добавить).

## SUFFICIENCY VS CLAUDE
Ясно ли, что AIM_FS превосходит Claude memory по каждой из 15 заявленных осей?
Если на каких-то осях преимущество не доказано — назвать.
"""

REVISER_SYS = """Ты — главный архитектор AIM. У тебя есть исходный DESIGN SPEC и
peer review с критикой. Выпусти исправленную версию SPEC, закрывающую все
CRITICAL и большую часть MINOR замечаний.

ВАЖНОЕ ПРАВИЛО:
- Если reviewer указал на over-engineering / нереалистичные сроки — БЕЗ ЖАЛОСТИ
  переноси сложные подсистемы в раздел `## Roadmap (post-MVP)` и сокращай Phase 1
  до 5-7 дней реальной работы. MVP должен быть строго минимален.
- Если reviewer требует SQLite вместо flock на распределённых ФС — соглашайся и
  фиксируй. Не оправдывай.
- Если reviewer перечислил конкретные edge cases (LLM-мусор, null confidence,
  каскадный undo, idempotency cleanup) — добавь явный раздел "Edge cases" с
  ответом на КАЖДЫЙ.
- Если reviewer не прав — в новой версии добавь пометку `<!-- review_response: ... -->`
  и аргументируй (но не больше 2 раз за весь документ).

ФОРМАТ:
- Заголовок «# AIM Filesystem Specification (AIM_FS) — vN draft» (где N = след.номер).
- Меняй дату на новую.
- В конце добавь раздел `## Changelog vN-1 → vN` с маркированным списком закрытых
  замечаний (пиши «Issue #X (CRITICAL): ... → закрыто через ...»).

Возвращай ПОЛНЫЙ обновлённый текст SPEC.
"""

def ask(system: str, user: str, retries: int = 4) -> str:
    last = None
    for i in range(retries):
        try:
            r = CLIENT.chat.completions.create(
                model=MODEL,
                messages=[{"role":"system","content":system},
                          {"role":"user","content":user}],
                max_tokens=16000,
            )
            return r.choices[0].message.content or ""
        except (APITimeoutError, Exception) as e:
            last = e
            msg = str(e).lower()
            time.sleep(60 * (i+1) if "rate" in msg or "429" in msg else 15 * (i+1))
    raise RuntimeError(f"DeepSeek failed: {last}")

def main():
    mode = sys.argv[1]
    if mode == "review":
        spec, out = sys.argv[2], sys.argv[3]
        ans = ask(REVIEWER_SYS, "# AIM_FS DESIGN SPEC\n\n" + pathlib.Path(spec).read_text() +
                  "\n\nДай строгий peer review.")
    elif mode == "revise":
        spec, review, out = sys.argv[2], sys.argv[3], sys.argv[4]
        ans = ask(REVISER_SYS,
                  "# Исходный SPEC\n\n" + pathlib.Path(spec).read_text() +
                  "\n\n# Peer review\n\n" + pathlib.Path(review).read_text() +
                  "\n\nВыпусти исправленный SPEC v_next целиком.")
    else:
        sys.exit("unknown mode")
    pathlib.Path(out).write_text(ans)
    print(f"OK {mode} -> {out} ({len(ans)} chars)")

if __name__ == "__main__":
    main()
