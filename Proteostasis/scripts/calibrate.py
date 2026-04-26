#!/usr/bin/env python3
"""Calibrate Proteostasis counter parameters from literature anchor points.

Anchors (t_years, n_divisions, expected D_i) from:
  Klaips 2018 PMID 29127110; Hipp 2019 PMID 30733602; Kaushik 2021 PMID 34563704
"""
from __future__ import annotations
import json, sys
from pathlib import Path

try:
    import numpy as np
except ImportError:
    print("numpy required", file=sys.stderr); sys.exit(1)

ANCHORS = [(0, 0, 0.0), (30, 30, 0.15), (50, 50, 0.3), (70, 70, 0.55)]

N_STAR_DIV = 80.0    # anchor reference division count
TAU_YEARS  = 80.0 # anchor reference time
D_CRITICAL = 0.6    # tissue-average critical threshold

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
        counter_name = 'Proteostasis',
        alpha = alpha,
        beta = beta,
        gamma = 0.0,
        n_star_divisions = N_STAR_DIV,
        tau_days = TAU_YEARS * 365.0,
        d_critical = D_CRITICAL,
        fit_rms = rms,
        anchor_count = len(ANCHORS),
        source_pmids = 'Klaips 2018 PMID 29127110; Hipp 2019 PMID 30733602; Kaushik 2021 PMID 34563704',
        method = "linear-lstsq-on-anchors",
    )
    out_path = Path(__file__).parent.parent / "PARAMETERS_calibrated.json"
    out_path.write_text(json.dumps(calibrated, indent=2))
    print(f'α = {alpha:.4f}, β = {beta:.4f}, fit RMS = {rms:.4e}')
    print(f'→ {out_path}')

if __name__ == "__main__":
    main()
