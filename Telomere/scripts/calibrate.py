#!/usr/bin/env python3
"""Calibrate Telomere counter parameters from literature anchor points.

Anchors (t_years, n_divisions, expected D_i) from:
  Harley 1990 PMID 2342578; Parrinello 2003 PMID 12855956
"""
from __future__ import annotations
import json, sys
from pathlib import Path

try:
    import numpy as np
except ImportError:
    print("numpy required", file=sys.stderr); sys.exit(1)

ANCHORS = [(0, 0, 0.0), (30, 30, 0.33), (50, 80, 0.55)]

N_STAR_DIV = 50.0    # anchor reference division count
TAU_YEARS  = 90.0 # anchor reference time
D_CRITICAL = 0.55    # tissue-average critical threshold

def fit(anchors):
    A = np.array([[a[0] / N_STAR_DIV, a[1] / TAU_YEARS] for a in anchors])
    y = np.array([a[2] for a in anchors])
    sol, *_ = np.linalg.lstsq(A, y, rcond=None)
    return float(sol[0]), float(sol[1])

def main():
    alpha, beta = fit(ANCHORS)
    # Residuals as sanity check
    A = np.array([[a[0] / N_STAR_DIV, a[1] / TAU_YEARS] for a in ANCHORS])
    y_true = np.array([a[2] for a in ANCHORS])
    y_pred = A @ np.array([alpha, beta])
    rms = float(np.sqrt(np.mean((y_true - y_pred)**2)))
    calibrated = dict(
        counter_name = 'Telomere',
        alpha = alpha,
        beta = beta,
        gamma = 0.0,
        n_star_divisions = N_STAR_DIV,
        tau_days = TAU_YEARS * 365.0,
        d_critical = D_CRITICAL,
        fit_rms = rms,
        anchor_count = len(ANCHORS),
        source_pmids = 'Harley 1990 PMID 2342578; Parrinello 2003 PMID 12855956',
        method = "linear-lstsq-on-anchors",
    )
    out_path = Path(__file__).parent.parent / "PARAMETERS_calibrated.json"
    out_path.write_text(json.dumps(calibrated, indent=2))
    print(f'α = {alpha:.4f}, β = {beta:.4f}, fit RMS = {rms:.4e}')
    print(f'→ {out_path}')

if __name__ == "__main__":
    main()
