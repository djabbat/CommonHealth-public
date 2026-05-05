#!/usr/bin/env python3
"""AIM/AI_run_self_diag.py — top-level launcher (DESK1+, 2026-05-03).

Top-level shim so the bash sandbox doesn't need `python -m`. Runs the
ACTUAL diagnostic (DeepSeek call), saves report, prints path.
"""
from __future__ import annotations

import sys
from pathlib import Path

_HERE = Path(__file__).resolve().parent
if str(_HERE) not in sys.path:
    sys.path.insert(0, str(_HERE))

from AI.ai.run_self_diagnostic import _main  # noqa: E402

if __name__ == "__main__":
    raise SystemExit(_main())
