"""AI/ai/run_self_diagnostic.py — actually RUN the diagnostic (DESK1+, 2026-05-03).

Where `self_diagnostic.py` only emits the prompt for a human to paste,
this module actually:

  1. Builds the 9-phase prompt + ground-truth inventory.
  2. POSTs it to DeepSeek-reasoner (or DeepSeek-V4-pro fallback).
  3. Saves the model's full audit report to
     `AI/artifacts/self_diag_<date>.md`.
  4. Returns the path to the saved report.

The icon launcher should call this so that a double-click *runs* the
audit, not just opens the prompt for human pasting.

Public API:
    run(model="deepseek-reasoner", *, save=True) -> Path
"""
from __future__ import annotations

import datetime as dt
import json
import logging
import os
import sys
import time
from pathlib import Path
from typing import Optional

log = logging.getLogger("ai.run_self_diagnostic")


def project_root() -> Path:
    return Path(__file__).resolve().parent.parent.parent


def ai_root() -> Path:
    return project_root() / "AI"


def _api_key() -> Optional[str]:
    """Resolve DEEPSEEK_API_KEY from env, or by reading ~/.aim_env."""
    val = os.environ.get("DEEPSEEK_API_KEY")
    if val:
        return val
    aim_env = Path.home() / ".aim_env"
    if not aim_env.exists():
        return None
    for line in aim_env.read_text(encoding="utf-8", errors="replace").splitlines():
        line = line.strip()
        if line.startswith("DEEPSEEK_API_KEY=") or line.startswith("export DEEPSEEK_API_KEY="):
            v = line.split("=", 1)[1].strip().strip("'\"")
            if v:
                return v
    return None


def _post_deepseek(prompt: str, model: str,
                   *, timeout: int = 600) -> str:
    """Single-shot DeepSeek call. Raises RuntimeError on failure."""
    key = _api_key()
    if not key:
        raise RuntimeError("DEEPSEEK_API_KEY not found in env or ~/.aim_env")
    try:
        import httpx
    except ImportError as e:
        raise RuntimeError(f"httpx missing: {e}")
    body = {
        "model": model,
        "messages": [
            {"role": "system",
             "content": ("You are an adversarial code auditor. Find defects, "
                         "do not confirm health. Every finding must reference "
                         "path:line. Fabrications fail L_VERIFIABILITY. "
                         "Return one markdown report with all 9 phases.")},
            {"role": "user", "content": prompt},
        ],
        "temperature": 0.2,
        "max_tokens": 16000,
    }
    headers = {"Authorization": f"Bearer {key}",
               "Content-Type": "application/json"}
    url = "https://api.deepseek.com/v1/chat/completions"
    with httpx.Client(timeout=timeout) as cli:
        r = cli.post(url, json=body, headers=headers)
        r.raise_for_status()
        data = r.json()

    # Record the call to cost_monitor (if available) so daily/monthly
    # budget alerts catch self-diagnostic spending.
    try:
        usage = data.get("usage", {})
        in_tok = int(usage.get("prompt_tokens", 0))
        out_tok = int(usage.get("completion_tokens", 0))
        if in_tok or out_tok:
            from agents import cost_monitor
            cost_monitor.record(
                model=model,
                input_tokens=in_tok,
                output_tokens=out_tok,
                provider="deepseek",
                task_id="ai.self_diagnostic",
            )
    except Exception as e:
        log.debug("cost_monitor.record skipped: %s", e)

    return data["choices"][0]["message"]["content"]


def _output_path(today: Optional[dt.date] = None) -> Path:
    today = today or dt.date.today()
    out = ai_root() / "artifacts" / f"self_diag_{today.isoformat()}.md"
    out.parent.mkdir(parents=True, exist_ok=True)
    return out


_COMPLIANCE_RETRY_SUFFIX = (
    "\n\n---\n\n"
    "**CRITICAL — REPEATED INSTRUCTION:** Your previous response had "
    "{prev:.0%} line-compliance ({n_with}/{n_total} findings carried a "
    "`:line` ref). The diagnostic spec REQUIRES `path:line` (e.g. "
    "`AI/ai/distillation_tracker.py:42`) on every finding. Re-emit the "
    "ENTIRE 9-phase report with at least 80% line compliance. Findings "
    "without `:line` will be discarded post-hoc."
)


def _compliance_of(report: str) -> tuple[float, int, int]:
    from AI.ai.meta_evaluator import parse_report
    p = parse_report(report)
    n_total = len(p.findings)
    n_with = sum(1 for r in p.findings
                  if ":" in r and r.rsplit(":", 1)[-1].isdigit())
    return (p.line_compliance, n_with, n_total)


def run(model: str = "deepseek-reasoner",
        *,
        save: bool = True,
        verbose: bool = True,
        compliance_retry: bool = True,
        min_compliance: float = 0.5,
        skip_safety_gate: bool = False) -> Path:
    """Build prompt, send to DeepSeek, save the report.

    If `compliance_retry` is true and the first response has <
    `min_compliance` line-refs with `:line`, append a corrective tail
    and retry ONCE before saving. Set `compliance_retry=False` to
    disable (e.g. in tests).

    By default the call is gated by `safety_gate.can_run()` which
    blocks on cooldown / daily-budget breach. Pass
    `skip_safety_gate=True` to override (e.g. manual force-run).
    Raises `RuntimeError` if the gate refuses.
    """
    if not skip_safety_gate:
        try:
            from AI.ai.safety_gate import can_run
            v = can_run()
            if not v.allowed:
                raise RuntimeError(
                    "safety gate blocked diagnostic run: "
                    + "; ".join(v.reasons)
                )
        except RuntimeError:
            raise
        except Exception as e:
            log.debug("safety_gate skipped: %s", e)
    from AI.ai.self_diagnostic import build_prompt
    prompt = build_prompt()
    if verbose:
        print(f"[run_self_diagnostic] prompt size: {len(prompt)} chars")
        print(f"[run_self_diagnostic] model: {model}")
        print(f"[run_self_diagnostic] querying DeepSeek (this may take "
              "several minutes)...")
    t0 = time.time()
    try:
        report = _post_deepseek(prompt, model)
    except Exception as e:
        if model == "deepseek-reasoner":
            if verbose:
                print(f"[run_self_diagnostic] reasoner failed ({e}); "
                      "falling back to deepseek-chat")
            report = _post_deepseek(prompt, "deepseek-chat")
        else:
            raise
    elapsed = time.time() - t0
    if verbose:
        print(f"[run_self_diagnostic] done in {elapsed:.1f}s, "
              f"report size: {len(report)} chars")

    retry_used = False
    if compliance_retry:
        comp, n_with, n_total = _compliance_of(report)
        if n_total > 0 and comp < min_compliance:
            if verbose:
                print(f"[run_self_diagnostic] line_compliance={comp:.0%} "
                      f"(<{min_compliance:.0%}) — retrying once with "
                      "corrective suffix")
            retry_prompt = prompt + _COMPLIANCE_RETRY_SUFFIX.format(
                prev=comp, n_with=n_with, n_total=n_total)
            try:
                retry = _post_deepseek(retry_prompt, model)
                retry_comp = _compliance_of(retry)[0]
                if retry_comp > comp:
                    report = retry
                    retry_used = True
                    if verbose:
                        print(f"[run_self_diagnostic] retry compliance="
                              f"{retry_comp:.0%}; using retry")
                elif verbose:
                    print(f"[run_self_diagnostic] retry compliance="
                          f"{retry_comp:.0%} did not improve; keeping "
                          "first response")
            except Exception as e:
                if verbose:
                    print(f"[run_self_diagnostic] retry failed: {e}; "
                          "keeping first response")

    if save:
        out = _output_path()
        out.write_text(report, encoding="utf-8")
        try:
            from AI.ai.diagnostic_ledger import record_from_report
            record_from_report(report, model=model,
                                retry_used=retry_used,
                                report_path=str(out))
        except Exception as e:
            log.debug("ledger record skipped: %s", e)
        try:
            from AI.ai.prompt_versions import record_current
            record_current()
        except Exception as e:
            log.debug("prompt_versions record skipped: %s", e)
        if verbose:
            print(f"[run_self_diagnostic] saved → {out}")
            try:
                from AI.ai.meta_evaluator import parse_report
                parsed = parse_report(report)
                comp_final = parsed.line_compliance
                print(f"[run_self_diagnostic] grade={parsed.grade} "
                      f"refs={len(parsed.findings)} "
                      f"line_compliance={comp_final:.0%}")
                if comp_final < 0.5 and parsed.findings:
                    print("[run_self_diagnostic] ⚠ low line compliance — "
                          "model ignored path:line rule; rerun recommended")
            except Exception as e:
                log.debug("compliance check skipped: %s", e)
        return out
    return Path("/dev/null")


def _main() -> int:
    import argparse
    ap = argparse.ArgumentParser(description="Run AIM/AI self-diagnostic")
    ap.add_argument("--model", default="deepseek-reasoner")
    ap.add_argument("--quiet", action="store_true")
    ap.add_argument("--force", action="store_true",
                     help="bypass safety gate (cooldown + budget)")
    args = ap.parse_args()
    try:
        out = run(model=args.model, verbose=not args.quiet,
                  skip_safety_gate=args.force)
        if not args.quiet:
            print(f"\nreport: {out}")
        return 0
    except Exception as e:
        print(f"ERROR: {e}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(_main())
