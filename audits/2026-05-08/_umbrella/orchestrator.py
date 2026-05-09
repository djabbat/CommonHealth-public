#!/usr/bin/env python3
"""
Overnight audit orchestrator.

Phases:
  1. Build packets for every project in inventory.json
  2. Peer-review each packet via deepseek-reasoner (parallel)
  3. Generate improvement plan for each
  4. Check plan vs review; iterate (max 3 rounds) until ACCEPT
  5. Aggregate AUDIT_REPORT_2026-05-08.md on Desktop

Resumable: skips steps whose output file already exists.
"""
import os, sys, json, subprocess, pathlib, time, traceback
from concurrent.futures import ThreadPoolExecutor, as_completed

ROOT       = pathlib.Path("/home/oem/Desktop/AUDIT_2026-05-08")
PACKETS    = ROOT / "packets"
REVIEWS    = ROOT / "reviews"
PLANS      = ROOT / "plans"
ITERS      = ROOT / "iterations"
LOGS       = ROOT / "logs"
STATUS     = ROOT / "status.json"
INVENTORY  = ROOT / "inventory.json"
DS_REVIEW  = ROOT / "ds_review.py"
BUILD_PACK = ROOT / "build_packet.py"
REPORT     = pathlib.Path("/home/oem/Desktop/AUDIT_REPORT_2026-05-08.md")

CONCURRENCY = int(os.environ.get("AUDIT_CONCURRENCY", "5"))
MAX_ITERS   = int(os.environ.get("AUDIT_MAX_ITERS", "3"))

def log(msg: str):
    ts = time.strftime("%H:%M:%S")
    line = f"[{ts}] {msg}"
    print(line, flush=True)
    with (LOGS / "orchestrator.log").open("a") as f:
        f.write(line + "\n")

def load_status() -> dict:
    if STATUS.exists():
        try:
            return json.loads(STATUS.read_text())
        except Exception:
            return {}
    return {}

def save_status(s: dict):
    STATUS.write_text(json.dumps(s, indent=2, ensure_ascii=False))

def run(cmd, timeout=900):
    return subprocess.run(cmd, capture_output=True, text=True, timeout=timeout)

def phase_packets(inv: dict, status: dict):
    log(f"PHASE 1: build packets ({len(inv['local']) + len(inv['server'])} projects)")
    for proj in inv["local"]:
        slug, root = proj["slug"], proj["root"]
        out = PACKETS / f"{slug}.md"
        if out.exists() and out.stat().st_size > 200:
            continue
        log(f"  packet local {slug}")
        r = run([sys.executable, str(BUILD_PACK), root, slug, str(out)], timeout=120)
        if r.returncode != 0:
            log(f"  ERR packet {slug}: {r.stderr[:200]}")
    for proj in inv["server"]:
        slug, root = proj["slug"], proj["root"]
        out = PACKETS / f"{slug}.md"
        if out.exists() and out.stat().st_size > 200:
            continue
        log(f"  packet server {slug}")
        r = run([sys.executable, str(BUILD_PACK), root, slug, str(out), "--server", "--ssh-host", "server"], timeout=180)
        if r.returncode != 0:
            log(f"  ERR packet srv {slug}: {r.stderr[:200]}")
    save_status(status)

def call_ds(mode: str, args: list[str]) -> bool:
    r = run([sys.executable, str(DS_REVIEW), mode, *args], timeout=900)
    if r.returncode != 0:
        log(f"  DS_FAIL {mode} {args[-1]}: {r.stderr[:300]}")
        return False
    return True

def project_pipeline(slug: str) -> dict:
    """Run review → plan → check → iterate for one project."""
    packet = PACKETS / f"{slug}.md"
    if not packet.exists() or packet.stat().st_size < 200:
        return {"slug": slug, "stage": "no_packet", "verdict": "SKIP"}

    review = REVIEWS / f"{slug}.review.md"
    if not review.exists():
        log(f"  review {slug}")
        if not call_ds("review", [str(packet), str(review)]):
            return {"slug": slug, "stage": "review_failed"}

    plan = PLANS / f"{slug}.plan.v1.md"
    if not plan.exists():
        log(f"  plan {slug}")
        if not call_ds("plan", [str(packet), str(review), str(plan)]):
            return {"slug": slug, "stage": "plan_failed"}

    final_plan = plan
    final_check = None
    accepted = False

    for it in range(1, MAX_ITERS + 1):
        check = ITERS / f"{slug}.check.v{it}.md"
        if not check.exists():
            log(f"  check {slug} v{it}")
            if not call_ds("check", [str(packet), str(final_plan), str(check)]):
                break
        body = check.read_text()
        final_check = check
        if "ACCEPT" in body.split("\n")[0:20].__str__().upper() and "NEEDS_REVISION" not in body[:500].upper():
            # Coarse parse: first VERDICT line
            verdict_line = ""
            for line in body.splitlines():
                if "VERDICT" in line.upper() and ":" in line:
                    verdict_line = line
                    break
                if line.strip().upper() in ("ACCEPT", "NEEDS_REVISION"):
                    verdict_line = line
                    break
            if "NEEDS_REVISION" not in verdict_line.upper() and "ACCEPT" in body.upper()[:1000]:
                accepted = True
                break
        # Refine plan: send packet + review + last check + last plan to planner
        next_plan = PLANS / f"{slug}.plan.v{it+1}.md"
        if not next_plan.exists():
            log(f"  refine plan {slug} -> v{it+1}")
            # Use 'plan' mode but pass merged review = review + check feedback
            merged_review = ITERS / f"{slug}.merged_review.v{it+1}.md"
            merged_review.write_text(
                "# Original review\n\n" + review.read_text()
                + "\n\n# Last plan\n\n" + final_plan.read_text()
                + "\n\n# Last check feedback\n\n" + body
                + "\n\n## Instruction\nПереработай план так, чтобы закрыть REMAINING_GAPS из последней проверки."
            )
            if not call_ds("plan", [str(packet), str(merged_review), str(next_plan)]):
                break
        final_plan = next_plan

    return {
        "slug": slug,
        "review": str(review),
        "plan_final": str(final_plan),
        "check_final": str(final_check) if final_check else None,
        "accepted": accepted,
    }

def phase_pipeline(inv: dict, status: dict):
    log("PHASE 2-4: review + plan + iterate (parallel)")
    all_slugs = [p["slug"] for p in inv["local"]] + [p["slug"] for p in inv["server"]]
    results = {}
    with ThreadPoolExecutor(max_workers=CONCURRENCY) as ex:
        futs = {ex.submit(project_pipeline, s): s for s in all_slugs}
        for fut in as_completed(futs):
            s = futs[fut]
            try:
                results[s] = fut.result()
                log(f"  DONE {s}: accepted={results[s].get('accepted')}")
            except Exception as e:
                log(f"  EXC {s}: {e}")
                results[s] = {"slug": s, "error": str(e)}
            status["pipeline"] = results
            save_status(status)
    return results

def aggregate(inv: dict, results: dict):
    log("PHASE 5: aggregate AUDIT_REPORT_2026-05-08.md")
    out = ["# AUDIT REPORT — 2026-05-08\n",
           "Глубокий аудит всех проектов и подпроектов: локальная машина + сервер.\n",
           "Метод: per-project audit packet → DeepSeek-reasoner peer review → improvement plan → iterate до ACCEPT (max 3 rounds).\n",
           f"Проектов в обзоре: {len(results)}.\n\n"]
    accepted = sum(1 for r in results.values() if r.get("accepted"))
    out.append(f"## Сводка\n- ACCEPT после итераций: **{accepted}/{len(results)}**\n\n")

    # Per-project sections
    for slug in sorted(results.keys()):
        r = results[slug]
        out.append(f"---\n\n## {slug}\n")
        if r.get("review") and pathlib.Path(r["review"]).exists():
            out.append("### Peer review (v1)\n")
            out.append(pathlib.Path(r["review"]).read_text())
            out.append("\n")
        if r.get("plan_final") and pathlib.Path(r["plan_final"]).exists():
            out.append(f"### Improvement plan ({pathlib.Path(r['plan_final']).name})\n")
            out.append(pathlib.Path(r["plan_final"]).read_text())
            out.append("\n")
        if r.get("check_final") and pathlib.Path(r["check_final"]).exists():
            out.append(f"### Final check ({pathlib.Path(r['check_final']).name}) — accepted={r.get('accepted')}\n")
            out.append(pathlib.Path(r["check_final"]).read_text())
            out.append("\n")
        if r.get("error"):
            out.append(f"### Error\n{r['error']}\n")

    REPORT.write_text("\n".join(out))
    log(f"WROTE {REPORT} ({REPORT.stat().st_size} bytes)")

def cross_project_synthesis(results: dict):
    """Final pass: ask DeepSeek to summarize systemic issues across all reviews."""
    log("PHASE 6: cross-project systemic synthesis")
    fragments = []
    for slug in sorted(results.keys()):
        r = results[slug]
        if r.get("review") and pathlib.Path(r["review"]).exists():
            txt = pathlib.Path(r["review"]).read_text()
            # Extract verdict + critical issues only
            fragments.append(f"## {slug}\n{txt[:3500]}")
    bundle = "\n\n".join(fragments)
    bundle_path = ROOT / "bundle_for_synthesis.md"
    bundle_path.write_text(bundle)
    syn_path = ROOT / "synthesis.md"
    if not syn_path.exists():
        # Use review mode but with bundle
        # Instead, construct a one-off call via ds_review.py review
        # We'll reuse 'review' mode — works since system prompt is generic
        # But better: ask via plan mode with custom instruction
        pkt = ROOT / "_synthesis_packet.md"
        pkt.write_text(
            "# Cross-project audit synthesis\n\n"
            "Ниже приведены peer reviews для ВСЕЙ экосистемы проектов "
            "(локальная машина + сервер). Найди СИСТЕМНЫЕ паттерны: общие "
            "архитектурные ошибки, повторяющиеся нарушения правила Rust+Phoenix, "
            "проблемы согласованности между проектами, корневые причины. "
            "Дай вердикт по экосистеме в целом.\n\n" + bundle[:80_000]
        )
        run([sys.executable, str(DS_REVIEW), "review", str(pkt), str(syn_path)], timeout=900)
    return syn_path

def main():
    inv = json.loads(INVENTORY.read_text())
    status = load_status()
    LOGS.mkdir(exist_ok=True)
    PACKETS.mkdir(exist_ok=True)
    REVIEWS.mkdir(exist_ok=True)
    PLANS.mkdir(exist_ok=True)
    ITERS.mkdir(exist_ok=True)
    log("=== START orchestrator ===")
    phase_packets(inv, status)
    results = phase_pipeline(inv, status)
    syn_path = cross_project_synthesis(results)
    aggregate(inv, results)
    # Append synthesis at the top of the report
    if syn_path.exists() and REPORT.exists():
        body = REPORT.read_text()
        head = body.split("## Сводка")[0]
        rest = "## Сводка" + body.split("## Сводка", 1)[1]
        synth = "## Cross-project systemic synthesis\n\n" + syn_path.read_text() + "\n\n"
        REPORT.write_text(head + synth + rest)
        log("appended synthesis to top of report")
    log("=== DONE ===")

if __name__ == "__main__":
    try:
        main()
    except Exception:
        log("FATAL\n" + traceback.format_exc())
        sys.exit(2)
