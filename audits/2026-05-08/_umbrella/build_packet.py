#!/usr/bin/env python3
"""
Build audit packet for a project directory.

Usage: build_packet.py <project_root> <slug> <out.md> [--server] [--ssh-host server]

Packet contents:
  1. Path, size, file counts
  2. Stack detection (Rust / Phoenix / Python / Go / PHP / etc)
  3. Top-level tree (depth=2)
  4. Core .md files (CONCEPT/THEORY/CLAUDE/README/MAP/PARAMETERS/UPGRADE/STATE etc), trimmed
  5. Key code samples (Cargo.toml, mix.exs, package.json, main entry points), trimmed
  6. Cross-file consistency hints
"""
import os, sys, json, subprocess, pathlib, re, shlex
from typing import List, Dict, Optional

CORE_FILES = [
    "CLAUDE.md", "README.md", "CONCEPT.md", "THEORY.md", "MAP.md",
    "PARAMETERS.md", "UPGRADE.md", "STATE.md", "TODO.md", "DESIGN.md",
    "EVIDENCE.md", "OPEN_PROBLEMS.md", "KNOWLEDGE.md", "MEMORY.md",
    "LINKS.md", "NEWS.md", "AGENT.md", "AGENTS.md",
    "Cargo.toml", "mix.exs", "package.json", "go.mod", "pyproject.toml",
    "requirements.txt", "Dockerfile", "compose.yaml", "docker-compose.yaml",
    "Makefile",
]

CODE_GLOBS = ["*.rs", "*.ex", "*.exs", "*.go", "*.py", "*.php", "*.ts", "*.tsx", "*.js"]

def run(cmd: List[str], capture=True, timeout=30) -> str:
    try:
        r = subprocess.run(cmd, capture_output=capture, text=True, timeout=timeout)
        return r.stdout or ""
    except Exception as e:
        return f"<<ERR {e}>>"

def is_remote(args) -> bool:
    return "--server" in args

def ssh_host(args) -> str:
    if "--ssh-host" in args:
        return args[args.index("--ssh-host") + 1]
    return "server"

def remote_run(host: str, shell_cmd: str, timeout=60) -> str:
    return run(["ssh", host, shell_cmd], timeout=timeout)

def head(text: str, max_lines: int) -> str:
    lines = text.splitlines()
    if len(lines) <= max_lines:
        return text
    return "\n".join(lines[:max_lines]) + f"\n…<truncated {len(lines)-max_lines} more lines>…"

def build_local(root: str, slug: str) -> str:
    p = pathlib.Path(root)
    if not p.exists():
        return f"# packet {slug}\n\n**ERROR**: path not found: {root}\n"

    out = [f"# AUDIT PACKET — {slug}\n", f"Path: `{root}`  Date: 2026-05-08\n"]

    # 1. Stats
    du = run(["du", "-sh", "--exclude=.git", "--exclude=node_modules",
              "--exclude=target", "--exclude=_build", "--exclude=__pycache__", root])
    out.append(f"## Size & file counts\n```\n{du.strip()}\n```")

    # File-extension distribution
    counts = {}
    for r, dirs, files in os.walk(root):
        dirs[:] = [d for d in dirs if d not in (
            ".git", "node_modules", "target", "_build", "__pycache__",
            ".venv", "venv", "deps", "dist", "build", "_archive", "Archive")]
        for f in files:
            ext = pathlib.Path(f).suffix.lower() or "(noext)"
            counts[ext] = counts.get(ext, 0) + 1
    top_ext = sorted(counts.items(), key=lambda kv: -kv[1])[:15]
    out.append("**Extensions:** " + ", ".join(f"{e}={c}" for e, c in top_ext))

    # 2. Tree depth=2
    tree = run(["find", root, "-maxdepth", "2", "-not", "-path", "*/.*",
                "-not", "-path", "*/node_modules*", "-not", "-path", "*/target*",
                "-not", "-path", "*/_build*", "-not", "-path", "*/__pycache__*"])
    tree_lines = [l.replace(root, ".") for l in tree.splitlines()][:200]
    out.append(f"## Tree (depth=2, max 200 entries)\n```\n" + "\n".join(tree_lines) + "\n```")

    # 3. Stack detection
    stack = []
    if (p / "Cargo.toml").exists() or any(p.rglob("Cargo.toml")):
        stack.append("Rust")
    if (p / "mix.exs").exists() or any(p.rglob("mix.exs")):
        stack.append("Phoenix/Elixir")
    if (p / "go.mod").exists() or any(p.rglob("go.mod")):
        stack.append("Go")
    if (p / "package.json").exists() or any(p.rglob("package.json")):
        stack.append("Node/JS")
    if any(p.rglob("requirements.txt")) or any(p.rglob("pyproject.toml")) or any(p.glob("*.py")):
        stack.append("Python")
    if any(p.rglob("*.php")):
        stack.append("PHP")
    out.append(f"## Detected stack: **{', '.join(stack) or 'unknown'}**")

    # 4. Core files
    out.append("## Core files\n")
    for fname in CORE_FILES:
        candidates = list(p.glob(fname)) + list(p.glob(f"*/{fname}"))[:3]
        for f in candidates[:4]:
            try:
                content = f.read_text(errors="replace")
            except Exception:
                continue
            # Trim limits per file type
            limit = 200 if f.suffix == ".md" else 150
            out.append(f"### `{f.relative_to(p)}` ({len(content)} chars)\n```{f.suffix.lstrip('.')}\n"
                       + head(content, limit) + "\n```")

    # 5. Code samples (entry points)
    code_samples = []
    for pat in ["src/main.rs", "src/lib.rs", "lib/*/application.ex", "main.go",
                "main.py", "app.py", "router.ex", "endpoint.ex"]:
        code_samples.extend(p.rglob(pat))
    for f in code_samples[:6]:
        try:
            t = f.read_text(errors="replace")
            out.append(f"### code `{f.relative_to(p)}`\n```\n" + head(t, 80) + "\n```")
        except Exception:
            pass

    # 6. Code histogram by language
    by_lang = {}
    for r, dirs, files in os.walk(root):
        dirs[:] = [d for d in dirs if d not in (
            ".git", "node_modules", "target", "_build", "__pycache__",
            ".venv", "venv", "deps", "dist", "build", "_archive", "Archive")]
        for f in files:
            ext = pathlib.Path(f).suffix.lower()
            if ext in {".rs", ".ex", ".exs", ".eex", ".heex", ".go", ".py", ".php", ".ts", ".tsx", ".js"}:
                full = os.path.join(r, f)
                try:
                    sz = os.path.getsize(full)
                    by_lang[ext] = by_lang.get(ext, [0, 0])
                    by_lang[ext][0] += 1
                    by_lang[ext][1] += sz
                except Exception:
                    pass
    if by_lang:
        out.append("## Code volume\n| ext | files | bytes |\n|---|---|---|")
        for ext, (n, sz) in sorted(by_lang.items(), key=lambda kv: -kv[1][1]):
            out.append(f"| {ext} | {n} | {sz} |")

    return "\n".join(out)

def build_remote(host: str, root: str, slug: str) -> str:
    out = [f"# AUDIT PACKET — {slug} (server)\n", f"Host: `{host}`  Path: `{root}`  Date: 2026-05-08\n"]
    du = remote_run(host, f"du -sh --exclude=node_modules --exclude=target --exclude=_build --exclude=__pycache__ {shlex.quote(root)} 2>/dev/null")
    out.append(f"## Size\n```\n{du.strip()}\n```")
    tree = remote_run(host, f"find {shlex.quote(root)} -maxdepth 2 -not -path '*/.*' -not -path '*/node_modules*' -not -path '*/target*' -not -path '*/_build*' 2>/dev/null | head -200")
    out.append(f"## Tree\n```\n{tree}\n```")
    # Stack detection
    stack_probe = remote_run(host, f"""
cd {shlex.quote(root)} 2>/dev/null && {{
echo '---rust---'; find . -maxdepth 4 -name 'Cargo.toml' 2>/dev/null | head -5
echo '---elixir---'; find . -maxdepth 4 -name 'mix.exs' 2>/dev/null | head -5
echo '---go---'; find . -maxdepth 4 -name 'go.mod' 2>/dev/null | head -5
echo '---php---'; find . -maxdepth 4 -name '*.php' 2>/dev/null | head -10
echo '---python---'; find . -maxdepth 4 -name '*.py' 2>/dev/null | head -10
echo '---node---'; find . -maxdepth 4 -name 'package.json' -not -path '*/node_modules*' 2>/dev/null | head -5
}}
""")
    out.append(f"## Stack probe\n```\n{stack_probe}\n```")
    # Core files
    for fname in CORE_FILES:
        content = remote_run(host, f"cd {shlex.quote(root)} 2>/dev/null && [ -f {fname} ] && head -200 {fname}")
        if content.strip():
            out.append(f"### `{fname}` (head 200 lines)\n```\n{content}\n```")
    # Service files
    services = remote_run(host, f"systemctl --user list-units --no-pager 2>/dev/null; sudo systemctl list-units --no-pager 2>/dev/null | head -40")
    out.append(f"## systemd snapshot\n```\n{services[:2000]}\n```")
    # Code histogram
    histogram = remote_run(host, f"""
cd {shlex.quote(root)} 2>/dev/null && for ext in rs ex exs heex go py php ts tsx js; do
  c=$(find . -name "*.$ext" -not -path '*/node_modules*' -not -path '*/target*' -not -path '*/_build*' 2>/dev/null | wc -l)
  echo "$ext $c"
done
""")
    out.append(f"## Code histogram\n```\n{histogram}\n```")
    return "\n".join(out)

def main():
    args = sys.argv[1:]
    root = args[0]
    slug = args[1]
    out_path = args[2]
    if is_remote(args):
        text = build_remote(ssh_host(args), root, slug)
    else:
        text = build_local(root, slug)
    pathlib.Path(out_path).write_text(text)
    print(f"OK packet {slug} -> {out_path} ({len(text)} chars)")

if __name__ == "__main__":
    main()
