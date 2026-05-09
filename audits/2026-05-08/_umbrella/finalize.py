#!/usr/bin/env python3
"""
Build the final executive header for AUDIT_REPORT_2026-05-08.md:
  - Project-by-project ACCEPT table with rounds needed
  - Systemic synthesis (already added) stays
  - Stack compliance map (declared stack vs actual code histogram)
  - Recommendations summary
Then prepend the executive header to the existing report (keeping per-project sections below).
"""
import json, re, pathlib, os
from collections import OrderedDict

ROOT = pathlib.Path("/home/oem/Desktop/AUDIT_2026-05-08")
INV  = json.loads((ROOT / "inventory.json").read_text())
PACKETS = ROOT / "packets"
ITERS = ROOT / "iterations"
REVIEWS = ROOT / "reviews"
PLANS = ROOT / "plans"
SYN = ROOT / "synthesis.md"
REPORT = pathlib.Path("/home/oem/Desktop/AUDIT_REPORT_2026-05-08.md")

VERDICT_RE = re.compile(r"^##\s*VERDICT\s*\n+\s*(\*\*)?(ACCEPT|NEEDS_REVISION|REJECT|MAJOR_REVISION|MINOR_REVISION)",
                        re.MULTILINE | re.IGNORECASE)
SCORES_RE = re.compile(r"^##\s*SCORES.*?(?=^##|\Z)", re.MULTILINE | re.DOTALL)

def parse_verdict(text: str) -> str:
    m = VERDICT_RE.search(text)
    return m.group(2).upper() if m else "?"

def rounds_needed(slug: str) -> tuple[int, str]:
    """Find lowest version where check returns ACCEPT."""
    files = sorted(ITERS.glob(f"{slug}.check.v*.md"),
                   key=lambda p: int(re.search(r"v(\d+)", p.name).group(1)))
    for f in files:
        v = parse_verdict(f.read_text())
        if v == "ACCEPT":
            ver = int(re.search(r"v(\d+)", f.name).group(1))
            return ver, "ACCEPT"
    if files:
        return len(files), parse_verdict(files[-1].read_text())
    return 0, "NO_CHECK"

def review_verdict(slug: str) -> str:
    f = REVIEWS / f"{slug}.review.md"
    if not f.exists():
        return "?"
    text = f.read_text()
    # Match line VERDICT ... or "## VERDICT" then word
    m = re.search(r"^##\s*VERDICT\s*\n+\s*(?:\*\*)?(ACCEPT|MAJOR_REVISION|MINOR_REVISION|REJECT)",
                  text, re.MULTILINE | re.IGNORECASE)
    return m.group(1).upper() if m else "?"

def code_histogram(slug: str) -> str:
    f = PACKETS / f"{slug}.md"
    if not f.exists():
        return ""
    text = f.read_text()
    m = re.search(r"## Code volume\n.*?\n((?:\|.*\n)+)", text, re.DOTALL)
    if m:
        lines = m.group(1).strip().splitlines()
        # Skip header rows
        data = [l for l in lines if l.startswith("|") and not l.startswith("|---") and "ext" not in l.lower()]
        parts = []
        for d in data[:5]:
            cols = [c.strip() for c in d.strip("|").split("|")]
            if len(cols) >= 2:
                parts.append(f"{cols[0]}={cols[1]}")
        return ",".join(parts)
    # Server packets — different format
    m = re.search(r"## Code histogram\n```\n(.*?)```", text, re.DOTALL)
    if m:
        parts = []
        for line in m.group(1).strip().splitlines()[:5]:
            tokens = line.split()
            if len(tokens) == 2 and tokens[1] != "0":
                parts.append(f".{tokens[0]}={tokens[1]}")
        return ",".join(parts)
    return ""

def detected_stack(slug: str) -> str:
    f = PACKETS / f"{slug}.md"
    if not f.exists():
        return ""
    text = f.read_text()
    m = re.search(r"##\s*Detected stack:\s*\*\*([^*]+)\*\*", text)
    if m:
        return m.group(1).strip()
    # Server: parse stack probe section ('---rust---' etc, then optional findings)
    parts = []
    for marker, label in [
        ("---rust---", "Rust"), ("---elixir---", "Phoenix/Elixir"),
        ("---go---", "Go"), ("---php---", "PHP"),
        ("---python---", "Python"), ("---node---", "Node/JS"),
    ]:
        idx = text.find(marker)
        if idx == -1:
            continue
        end = text.find("---", idx + len(marker))
        chunk = text[idx + len(marker):end if end != -1 else None]
        # Strip the leading newline + check non-empty file paths
        chunk_lines = [l for l in chunk.strip().splitlines() if l.strip().startswith("./")]
        if chunk_lines:
            parts.append(label)
    # Fallback: parse code histogram section for non-zero counts
    if not parts:
        m = re.search(r"## Code histogram\n```\n(.*?)```", text, re.DOTALL)
        if m:
            mapping = {"rs": "Rust", "ex": "Phoenix/Elixir", "exs": "Phoenix/Elixir",
                       "go": "Go", "php": "PHP", "py": "Python", "ts": "Node/JS",
                       "tsx": "Node/JS", "js": "Node/JS"}
            seen = set()
            for line in m.group(1).strip().splitlines():
                tk = line.split()
                if len(tk) == 2 and tk[1].isdigit() and int(tk[1]) > 0:
                    label = mapping.get(tk[0])
                    if label and label not in seen:
                        seen.add(label); parts.append(label)
    return ", ".join(parts) or "doc-only"

def stack_compliance(slug: str) -> str:
    """Verdict on Rust+Phoenix-only rule:
       OK if only Rust+Elixir+Phoenix (or doc-only allowed for legitimate non-code projects).
       VIOLATES if PHP/Python/Go/Node beyond legacy AIM/OCR exception."""
    s = detected_stack(slug).lower()
    if not s or s in {"?", "unknown"}:
        return "doc-only"
    has_rust = "rust" in s
    has_elx = "phoenix" in s or "elixir" in s
    has_python = "python" in s
    has_php = "php" in s
    has_go = "go" in s
    has_node = "node" in s

    # AIM/OCR exception
    is_aim = "aim" in slug.lower()

    violations = []
    if has_python and not is_aim:
        violations.append("Python")
    if has_php:
        violations.append("PHP")
    if has_go:
        violations.append("Go")
    if has_node:
        violations.append("Node/JS")

    if not violations and (has_rust or has_elx):
        return "OK"
    if not violations:
        return "doc-only"
    return f"VIOLATES ({','.join(violations)})"

def stack_compliance_for_subproject(slug: str) -> str:
    """Documentation-only / outreach / NGO projects don't need code."""
    DOC_ONLY_OK = {
        "GLA_umbrella", "GLA_Annals", "Iqalto_Aqtivirebuli",
        "Marketing_umbrella", "Marketing_Books", "Marketing_JabaEkimi",
        "PhD_umbrella", "PhD_dissertation", "PhD_E0", "PhD_microscope",
        "Regenesis", "SamnuAzuzi", "Sulkalmakhi", "WLRAbastumani",
        "Claude_service", "LC_CytogeneticTree", "LC_FCLC", "LC_HAP",
        "LC_Ontogenesis", "LC_AutomatedMicroscopy", "LC_deploy",
    }
    base = stack_compliance(slug)
    if slug in DOC_ONLY_OK and base == "doc-only":
        return "doc-only (OK)"
    if base == "VIOLATES" and slug.startswith("srv_") and slug not in {"srv_aim"}:
        return base + " — ожидаемо для legacy"
    return base

def build_summary_table() -> str:
    rows = ["| # | Project | Initial verdict | ACCEPT @ | Detected stack | Compliance | Code volume |",
            "|---|---|---|---|---|---|---|"]
    i = 0
    all_projs = INV["local"] + INV["server"]
    for proj in all_projs:
        slug = proj["slug"]
        i += 1
        rv = review_verdict(slug)
        rounds, final_v = rounds_needed(slug)
        rounds_str = f"v{rounds}" if rounds else "—"
        stack = detected_stack(slug)[:40] or "?"
        compl = stack_compliance_for_subproject(slug)
        codev = code_histogram(slug)
        rows.append(f"| {i} | `{slug}` | {rv} | {rounds_str} ({final_v}) | {stack} | {compl} | {codev} |")
    return "\n".join(rows)

def build_executive_header() -> str:
    n_total = len(INV["local"]) + len(INV["server"])
    accepted = 0
    needed_iter = 0
    rejected = 0
    for proj in INV["local"] + INV["server"]:
        rounds, final_v = rounds_needed(proj["slug"])
        if final_v == "ACCEPT":
            accepted += 1
            if rounds > 1:
                needed_iter += 1
        else:
            rejected += 1

    return f"""# AUDIT REPORT — 2026-05-08

Глубокий аудит всех проектов и подпроектов: локальная машина (`~/Desktop/`) + сервер (`ssh server`).

**Метод:**
1. Inventory: 49 проектов (38 локальных + 11 на сервере)
2. Per-project audit packet: размер, дерево (depth=2), детект стека, дамп core-файлов (CONCEPT/THEORY/CLAUDE/README/MAP/PARAMETERS/UPGRADE/STATE/TODO + Cargo.toml/mix.exs/package.json), entry-point код, гистограмма кода по расширениям
3. Peer review через **DeepSeek-reasoner** (~/Desktop/LongevityCommon/AIM/llm.py не использовался напрямую — сделан минимальный клиент `ds_review.py` для надёжности; ключ читается из `~/.aim_env`)
4. Improvement plan через DeepSeek-reasoner с инструкцией про P0/P1/P2 + правило Rust+Phoenix
5. Check цикл: peer review проверяет план → если NEEDS_REVISION, план переписывается с учётом REMAINING_GAPS → повтор до ACCEPT (max 3 раунда)
6. Cross-project synthesis: одно крупное ревью по всему bundle отзывов → системные паттерны

**Результат:**
- ACCEPT после итераций: **{accepted}/{n_total}**
- Потребовали ≥2 раундов уточнения плана: **{needed_iter}**
- Не достигли ACCEPT: **{rejected}**
- Системный вердикт по экосистеме: **REJECT** (см. синтез ниже — отдельные проекты улучшаются, но как программная система ансамбль несвязан)

---

## Топ-10 системных проблем (из cross-project синтеза)

1. **Систематическое нарушение правила «Rust + Phoenix only»** — Python/Arduino/PHP/Node присутствуют в большинстве проектов (LC_AutomatedMicroscopy, LC_HAP, LC_MCOA, srv_drjaba, srv_books, srv_longevity и др.), без чёткого scaffolding-плана миграции.
2. **Отсутствие исполняемого кода** — >50% «проектов» это концептуальные packs из markdown без целевого стека.
3. **Дублирование/противоречия в core-файлах** — README ≠ CONCEPT ≠ DESIGN ≠ PARAMETERS внутри одного проекта (LC_BioSense v*; LC_CDATA два damage-уравнения; GLA_Annals JCAL vs ARS).
4. **Бинарные артефакты в git** — десятки .docx, .pdf, старых снапшотов в LC_Ze, LC_MCOA, GLA, PhD/sources_pdfs.
5. **Полное отсутствие CI/CD, тестов, lock-файлов** — кроме `Iqalto/iqalto-core` (8 unit-тестов, и те с ошибкой) ни один проект не имеет работающей test-suite.
6. **Несогласованность параметров между подпроектами LongevityCommon** — `v*`, `α`, `β`, `τ` в PARAMETERS.md разных модулей не унифицированы; нет shared-крейта типов.
7. **Документация-без-кода как доминирующий паттерн** — усилия уходят в CONCEPT/THEORY/KNOWLEDGE, но MVP не доводится до запуска.
8. **Нарушение правила "no Docker"** — Dockerfile найден в LC_AIM, что противоречит `feedback_no_docker`.
9. **Server-side legacy** — drjaba/longevity/books на чистом PHP, что противоречит правилу стека (исключение для legacy не задокументировано).
10. **Отсутствие межпроектного API** — все LC-подпроекты позиционируются как часть единой системы старения, но shared-API/protobuf/единого workspace нет.

---

## Per-project executive table

{build_summary_table()}

Колонки:
- **Initial verdict** — вердикт DeepSeek-reasoner на исходный packet (ACCEPT / MINOR_REVISION / MAJOR_REVISION / REJECT)
- **ACCEPT @** — на каком раунде проверки плана достигнут вердикт ACCEPT (v1 = с первой итерации)
- **Detected stack** — стек по факту наличия Cargo.toml/mix.exs/go.mod/package.json/requirements
- **Compliance** — соответствие правилу Rust+Phoenix (OK / VIOLATES / doc-only OK)

---

## Cross-project systemic synthesis

{(SYN.read_text() if SYN.exists() else "(synthesis missing)")}

---

# Per-project full reviews + improvement plans

(Полные тексты review/plan/check для каждого из {n_total} проектов следуют ниже.)

"""

def main():
    header = build_executive_header()
    # Existing report — keep only per-project sections (after the first "---\n\n## " split)
    body = REPORT.read_text()
    # The current report has: "# AUDIT REPORT...\n\n## Cross-project ...\n\n## Сводка\n...\n---\n\n## <slug>\n..."
    # We want to keep per-project sections only.
    if "---\n\n## " in body:
        per_project = "---" + body.split("---", 1)[1]
        # Drop the first chunk that's actually the synthesis (which is now in header)
        # Find first "---\n\n## " that introduces a project (slug match)
        lines = per_project.splitlines()
        for i, l in enumerate(lines):
            if l.startswith("## ") and not any(x in l for x in
                ("Cross-project", "Сводка", "VERDICT", "SCORES", "CRITICAL",
                 "MINOR", "STRENGTHS", "ROOT", "REMAINING", "NOTES")):
                per_project = "\n".join(lines[i-1:]) if i > 0 else "\n".join(lines[i:])
                break
    else:
        per_project = body

    REPORT.write_text(header + "\n\n" + per_project)
    print(f"OK report rewritten: {REPORT} -> {REPORT.stat().st_size} bytes")

if __name__ == "__main__":
    main()
