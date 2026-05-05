"""AI/ai/skill_standard.py — HV4 (2026-05-04).

Bidirectional adapter between AIM internal skill format and the
agentskills.io open standard. Lets AIM skills be consumable by
external agents (Hermes, OpenClaw, SwarmClaw) and vice versa.

agentskills.io schema (subset we support):
{
  "name": "<id>",
  "description": "<one-line>",
  "version": "1.0.0",
  "trigger_phrases": ["..."],
  "instructions": "<markdown body>",
  "examples": [{"input": "...", "output": "..."}],
  "metadata": {"author": "...", "tags": [...]}
}

AIM internal format (already used by S7 skill_synthesis):
{
  "skill_id": "<id>",
  "theme": ["..."],
  "rationale": "...",
  "version": int,
  "body": "..."        // optional skill text
}

Public API:
    to_agentskills(aim_skill) -> dict
    from_agentskills(external) -> dict
    export_dir(src_dir, dst_dir) -> int
    import_dir(src_dir, dst_dir) -> int
"""
from __future__ import annotations

import dataclasses
import json
import logging
from pathlib import Path
from typing import Iterable

log = logging.getLogger("ai.skill_standard")


# ── conversion ──────────────────────────────────────────────────


def to_agentskills(aim_skill: dict) -> dict:
    """Map an AIM skill dict to agentskills.io schema."""
    skill_id = aim_skill.get("skill_id")
    if not skill_id:
        raise ValueError("aim skill missing skill_id")
    description = (aim_skill.get("rationale")
                    or " ".join(aim_skill.get("theme", []))
                    or "(auto-distilled skill)")
    instructions = aim_skill.get("body", "")
    if not instructions:
        # Synthesize a stub instruction body from theme / rationale.
        theme = aim_skill.get("theme", [])
        instructions = (
            "## Trigger\n\n"
            f"Theme keywords: {', '.join(theme) if theme else '(none)'}\n\n"
            "## Approach\n\n"
            f"{description}\n"
        )
    return {
        "name": skill_id,
        "description": description,
        "version": str(aim_skill.get("version", "1.0.0")),
        "trigger_phrases": list(aim_skill.get("theme", [])),
        "instructions": instructions,
        "examples": aim_skill.get("examples", []),
        "metadata": {
            "author": "AIM Hive Queen (auto-distilled)",
            "tags": ["aim-hive", "auto-distilled"]
                     + list(aim_skill.get("tags", [])),
            "source_n": aim_skill.get("source_n"),
            "eval_delta": aim_skill.get("eval_delta"),
        },
    }


def from_agentskills(external: dict) -> dict:
    """Map an agentskills.io skill dict to AIM internal format."""
    name = external.get("name")
    if not name:
        raise ValueError("external skill missing 'name'")
    return {
        "skill_id": name,
        "theme": list(external.get("trigger_phrases", [])),
        "rationale": external.get("description", ""),
        "version": external.get("version", "1.0.0"),
        "body": external.get("instructions", ""),
        "examples": external.get("examples", []),
        "tags": (list(external.get("metadata", {}).get("tags", []))
                  + ["external-import"]),
    }


# ── round-trip ─────────────────────────────────────────────────


def round_trip_aim(aim_skill: dict) -> dict:
    """aim → agentskills → aim. Verifies idempotency on key fields."""
    return from_agentskills(to_agentskills(aim_skill))


# ── batch dir IO ───────────────────────────────────────────────


def export_dir(src_dir: Path, dst_dir: Path,
                *, overwrite: bool = False) -> int:
    """Convert every .json AIM skill in src_dir to agentskills format
    in dst_dir. Returns count written."""
    src_dir = Path(src_dir)
    dst_dir = Path(dst_dir)
    if not src_dir.exists():
        return 0
    dst_dir.mkdir(parents=True, exist_ok=True)
    n = 0
    for src in src_dir.glob("*.json"):
        try:
            aim = json.loads(src.read_text(encoding="utf-8"))
            ext = to_agentskills(aim)
        except Exception as e:
            log.warning("skip %s: %s", src.name, e)
            continue
        dst = dst_dir / src.name
        if dst.exists() and not overwrite:
            continue
        dst.write_text(json.dumps(ext, indent=2, ensure_ascii=False),
                        encoding="utf-8")
        n += 1
    return n


def import_dir(src_dir: Path, dst_dir: Path,
                *, overwrite: bool = False) -> int:
    """Convert every .json agentskills file in src_dir to AIM format
    in dst_dir. Returns count written."""
    src_dir = Path(src_dir)
    dst_dir = Path(dst_dir)
    if not src_dir.exists():
        return 0
    dst_dir.mkdir(parents=True, exist_ok=True)
    n = 0
    for src in src_dir.glob("*.json"):
        try:
            ext = json.loads(src.read_text(encoding="utf-8"))
            aim = from_agentskills(ext)
        except Exception as e:
            log.warning("skip %s: %s", src.name, e)
            continue
        dst = dst_dir / src.name
        if dst.exists() and not overwrite:
            continue
        dst.write_text(json.dumps(aim, indent=2, ensure_ascii=False),
                        encoding="utf-8")
        n += 1
    return n


def summary() -> str:
    return ("🔌 Skill standard adapter — ready.\n"
            "  to_agentskills() / from_agentskills() — single-skill conversion\n"
            "  export_dir(src, dst) / import_dir(src, dst) — batch")
