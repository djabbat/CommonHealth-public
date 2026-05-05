"""Shared fixtures for AI/ subproject tests.

The AI subproject is independent from `agents/` for testing purposes —
we don't share the parent tests/conftest.py's session fixtures (e.g.
PATIENTS_DIR isolation) because AI tests should never touch Patients/.
"""
import os
import sys
from pathlib import Path

# Make AIM importable when pytest is invoked from AIM/ root.
_AIM_ROOT = Path(__file__).resolve().parent.parent.parent
if str(_AIM_ROOT) not in sys.path:
    sys.path.insert(0, str(_AIM_ROOT))
