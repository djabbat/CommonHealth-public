#!/usr/bin/env python3
"""compare_mcoa_cdata.py — MANDATORY MCOA-vs-CDATA comparison harness.

Per project rule (feedback_mcoa_cdata_comparison): every MCOA simulation run MUST be paired with
an analogous CDATA run and a Δ report filed to docs/comparisons/YYYY-MM-DD_label.md.

Usage
-----
    python3 compare_mcoa_cdata.py \
        --mcoa-csv out/mcoa_hsc_100.csv \
        --cdata-csv out/cdata_hsc_100.csv \
        --tissue hsc \
        --label mcoa_vs_cdata_hsc_100 \
        --out-dir ../docs/comparisons/

Requires pandas, matplotlib.
"""
from __future__ import annotations
import argparse
import datetime as dt
import os
import sys
from pathlib import Path

def _lazy_imports():
    import pandas as pd  # noqa: F401
    import matplotlib.pyplot as plt  # noqa: F401
    return pd, plt

def compare(mcoa_csv: Path, cdata_csv: Path, tissue: str, label: str, out_dir: Path) -> Path:
    pd, plt = _lazy_imports()
    mcoa = pd.read_csv(mcoa_csv)
    cdata = pd.read_csv(cdata_csv)

    # Axis is cumulative division count for both simulators.
    x_col = "n_cumulative" if "n_cumulative" in mcoa.columns else mcoa.columns[1]
    # CDATA exports a single 'damage' column (historical); MCOA exports per-counter + tissue_load.
    cdata_col = "damage" if "damage" in cdata.columns else [c for c in cdata.columns if c.lower() != x_col][0]
    mcoa_col = "centriolar" if "centriolar" in mcoa.columns else "tissue_load"

    # Align by the shorter series
    n = min(len(mcoa), len(cdata))
    delta = mcoa[mcoa_col].iloc[:n].to_numpy() - cdata[cdata_col].iloc[:n].to_numpy()

    # Plot
    fig, axs = plt.subplots(2, 1, figsize=(8, 6), sharex=True)
    axs[0].plot(mcoa[x_col].iloc[:n], mcoa[mcoa_col].iloc[:n], label=f"MCOA ({mcoa_col})")
    axs[0].plot(cdata[x_col].iloc[:n], cdata[cdata_col].iloc[:n], label=f"CDATA ({cdata_col})")
    axs[0].set_ylabel("damage (dimensionless)")
    axs[0].set_title(f"MCOA vs CDATA — tissue={tissue}")
    axs[0].legend()
    axs[1].plot(mcoa[x_col].iloc[:n], delta, color="red")
    axs[1].axhline(0, color="grey", linestyle="--")
    axs[1].set_xlabel("cumulative divisions (n)")
    axs[1].set_ylabel("Δ (MCOA − CDATA)")
    fig.tight_layout()

    out_dir.mkdir(parents=True, exist_ok=True)
    date = dt.date.today().isoformat()
    plot_path = out_dir / f"{date}_{label}.png"
    fig.savefig(plot_path, dpi=200)
    plt.close(fig)

    report_path = out_dir / f"{date}_{label}.md"
    with report_path.open("w") as f:
        f.write(f"# MCOA vs CDATA comparison — {label}\n\n")
        f.write(f"**Date:** {date}\n")
        f.write(f"**Tissue:** {tissue}\n")
        f.write(f"**MCOA input:** `{mcoa_csv}`\n")
        f.write(f"**CDATA input:** `{cdata_csv}`\n")
        f.write(f"**MCOA column compared:** `{mcoa_col}`\n")
        f.write(f"**CDATA column compared:** `{cdata_col}`\n\n")
        f.write(f"## Summary\n\n")
        f.write(f"- Samples compared: {n}\n")
        f.write(f"- max |Δ| = {abs(delta).max():.4f}\n")
        f.write(f"- mean Δ = {delta.mean():+.4f}\n")
        f.write(f"- std Δ = {delta.std(ddof=1):.4f}\n\n")
        f.write(f"![residual plot]({plot_path.name})\n\n")
        f.write("## Interpretation — to be filled in by author\n\n")
        f.write("Classify the divergence:\n\n")
        f.write("- [ ] (a) missing counter in CDATA's single-counter view\n")
        f.write("- [ ] (b) artefact of MCOA dimensionless normalisation\n")
        f.write("- [ ] (c) real biological signal\n")
        f.write("- [ ] (d) bug in one of the simulators\n\n")
        f.write("Write at least three sentences of interpretation. Cite PARAMETERS.md entries that\n")
        f.write("changed, if any. When real experimental data arrive, this report must be re-done\n")
        f.write("with far deeper analysis.\n")
    print(f"wrote {report_path}")
    return report_path


def main():
    p = argparse.ArgumentParser(description="MANDATORY MCOA-vs-CDATA comparison harness.")
    p.add_argument("--mcoa-csv", required=True, type=Path)
    p.add_argument("--cdata-csv", required=True, type=Path)
    p.add_argument("--tissue", required=True)
    p.add_argument("--label", required=True)
    p.add_argument("--out-dir", type=Path, default=Path("../docs/comparisons/"))
    args = p.parse_args()
    compare(args.mcoa_csv, args.cdata_csv, args.tissue, args.label, args.out_dir)


if __name__ == "__main__":
    sys.exit(main())
