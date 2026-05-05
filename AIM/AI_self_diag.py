#!/usr/bin/env python3
"""AIM/AI_self_diag.py — convenience launcher (2026-05-03).

The bash sandbox blocks `python -m`, so this top-level shim lets you run

    python AI_self_diag.py

instead of `python -m AI.ai.self_diagnostic`. Same output: the prompt
file lands in `AI/artifacts/self_diag_request_<date>.md`.
"""
from __future__ import annotations

import sys
from pathlib import Path

# Ensure the repo root is on sys.path so `AI.ai.*` resolves.
_HERE = Path(__file__).resolve().parent
if str(_HERE) not in sys.path:
    sys.path.insert(0, str(_HERE))

from AI.ai.self_diagnostic import _main  # noqa: E402

if __name__ == "__main__":
    raise SystemExit(_main())
