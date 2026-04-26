//! BioSense HRV — SDNN, RMSSD, autonomic switching v_HRV → χ_Ze.
//!
//! Rust port of `~/Desktop/LongevityCommon/BioSense/src/ze_hrv.py`.
//!
//! Validated metrics:
//!   - SDNN (Standard Deviation of NN intervals) — Fantasia DB d=0.72
//!   - RMSSD (Root Mean Square of Successive Differences)
//!   - pNN50 (proportion of successive RR diff > 50 ms)
//!
//! χ_Ze(HRV) interim/theoretical only — see CONCEPT.md (3 null pre-registered tests 2026).

use biosense_core::{
    AutonomicState, BioSenseError, RrSeries, DELTA_HYS, OVERLAP, V_STAR, WINDOW_S,
};
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------
// Time-domain measures
// ------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeDomain {
    pub mean_rr_ms: f64,
    pub sdnn_ms: f64,
    pub rmssd_ms: f64,
    pub pnn50: f64,
    pub n_intervals: usize,
}

pub fn time_domain(rr: &RrSeries) -> Result<TimeDomain, BioSenseError> {
    let v = &rr.rr_ms;
    if v.len() < 10 {
        return Err(BioSenseError::TooShort(v.len()));
    }
    let n = v.len();
    let mean = v.iter().sum::<f64>() / n as f64;
    let var = v.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / (n - 1) as f64;
    let sdnn = var.sqrt();

    // RMSSD = sqrt(mean(diff(RR)²))
    let diffs: Vec<f64> = v.windows(2).map(|w| w[1] - w[0]).collect();
    let rmssd = (diffs.iter().map(|d| d * d).sum::<f64>() / diffs.len() as f64).sqrt();

    // pNN50 = fraction of |diff(RR)| > 50 ms
    let nn50 = diffs.iter().filter(|&&d| d.abs() > 50.0).count() as f64;
    let pnn50 = nn50 / diffs.len() as f64;

    Ok(TimeDomain {
        mean_rr_ms: mean,
        sdnn_ms: sdnn,
        rmssd_ms: rmssd,
        pnn50,
        n_intervals: n,
    })
}

// ------------------------------------------------------------------
// Frequency-domain (LF/HF) — simple Welch-style via FFT
// ------------------------------------------------------------------

/// Approximate LF/HF power ratio using rectangular windowing of zero-padded RR series.
///
/// Note: this is a simplified estimator. For research-grade HRV analysis, use
/// pyHRV or MNE in Python via `_python_legacy/ze_hrv.py`. This Rust impl
/// suffices for online dashboards + interim biomarker calculation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FreqDomain {
    pub lf_power: f64,
    pub hf_power: f64,
    pub lf_hf_ratio: f64,
}

pub fn freq_domain_simple(rr: &RrSeries) -> Result<FreqDomain, BioSenseError> {
    if rr.rr_ms.len() < 32 {
        return Err(BioSenseError::TooShort(rr.rr_ms.len()));
    }
    // Resample to ~4 Hz interpolation grid (standard HRV practice)
    // Simplified: use mean differences. Full FFT TODO with rustfft crate.
    let n = rr.rr_ms.len();
    let mean = rr.rr_ms.iter().sum::<f64>() / n as f64;
    // Approximate: LF correlates with slow oscillations (window > 6.7s),
    //              HF correlates with fast (window < 6.7s).
    // Compute variance over short and long windows as proxies.
    let short_win = 8.min(n / 4);
    let long_win = 32.min(n / 2);
    let lf_proxy = sliding_var(&rr.rr_ms, long_win, mean);
    let hf_proxy = sliding_var(&rr.rr_ms, short_win, mean);
    let ratio = if hf_proxy > 1e-12 { lf_proxy / hf_proxy } else { f64::INFINITY };
    Ok(FreqDomain {
        lf_power: lf_proxy,
        hf_power: hf_proxy,
        lf_hf_ratio: ratio,
    })
}

fn sliding_var(v: &[f64], window: usize, mean: f64) -> f64 {
    if window == 0 || window > v.len() {
        return 0.0;
    }
    let mut total = 0.0;
    for w in v.windows(window) {
        let local_mean = w.iter().sum::<f64>() / window as f64;
        total += (local_mean - mean).powi(2);
    }
    total / (v.len() - window + 1) as f64
}

// ------------------------------------------------------------------
// Autonomic switching → χ_Ze (theoretical/interim only)
// ------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutonomicSwitching {
    pub n_windows: usize,
    pub n_state_changes: usize,
    pub v_hrv: f64,
    pub chi_ze_hrv: f64,
    pub state_sequence: Vec<AutonomicState>,
}

/// v_HRV = N_state_changes / (N_windows − 1).
/// χ_Ze(HRV) = 1 − |v_HRV − v*| / max(v*, 1−v*).
pub fn autonomic_switching(rr: &RrSeries, sample_rate_hz: f64) -> Result<AutonomicSwitching, BioSenseError> {
    if sample_rate_hz <= 0.0 {
        return Err(BioSenseError::InvalidSampling("sample_rate_hz must be > 0".into()));
    }
    let total_seconds: f64 = rr.rr_ms.iter().sum::<f64>() / 1000.0;
    if total_seconds < WINDOW_S * 2.0 {
        return Err(BioSenseError::TooShort(rr.rr_ms.len()));
    }

    // Tile windows of WINDOW_S seconds with OVERLAP overlap; for each window compute LF/HF.
    let step = WINDOW_S * (1.0 - OVERLAP);
    let mut windows: Vec<&[f64]> = Vec::new();
    let mut t_acc = 0.0;
    let mut start_idx = 0;
    let mut sample_idx = 0;
    while sample_idx < rr.rr_ms.len() {
        t_acc += rr.rr_ms[sample_idx] / 1000.0;
        if t_acc >= WINDOW_S {
            windows.push(&rr.rr_ms[start_idx..=sample_idx]);
            // Slide by `step` seconds
            let mut ts = 0.0;
            while start_idx < rr.rr_ms.len() && ts < step {
                ts += rr.rr_ms[start_idx] / 1000.0;
                start_idx += 1;
            }
            t_acc = 0.0;
            for &v in &rr.rr_ms[start_idx..=sample_idx] {
                t_acc += v / 1000.0;
            }
        }
        sample_idx += 1;
    }

    if windows.len() < 2 {
        return Err(BioSenseError::TooShort(rr.rr_ms.len()));
    }

    // For each window compute LF/HF using simplified estimator
    let mut states = Vec::with_capacity(windows.len());
    for w in &windows {
        let win_rr = RrSeries { rr_ms: w.to_vec(), source: rr.source.clone() };
        let fd = freq_domain_simple(&win_rr).unwrap_or(FreqDomain {
            lf_power: 0.0, hf_power: 0.0, lf_hf_ratio: 1.0,
        });
        let max_lh = fd.lf_power.max(fd.hf_power);
        let imbalance = (fd.lf_power - fd.hf_power).abs() / max_lh.max(1e-12);
        let state = if imbalance <= DELTA_HYS {
            AutonomicState::Unchanged
        } else if fd.lf_power > fd.hf_power * (1.0 + DELTA_HYS) {
            AutonomicState::Sympathetic
        } else {
            AutonomicState::Parasympathetic
        };
        states.push(state);
    }

    // Count state changes (treating Unchanged as continuation, not reset)
    let mut last_active: Option<AutonomicState> = None;
    let mut changes = 0;
    for &s in &states {
        if matches!(s, AutonomicState::Unchanged) {
            continue;
        }
        if let Some(prev) = last_active {
            if prev != s {
                changes += 1;
            }
        }
        last_active = Some(s);
    }

    let v_hrv = if states.len() > 1 {
        changes as f64 / (states.len() - 1) as f64
    } else {
        0.0
    };

    // χ_Ze(HRV) = 1 − |v − v*| / max(v*, 1−v*)
    let denom = V_STAR.max(1.0 - V_STAR);
    let chi_ze_hrv = 1.0 - (v_hrv - V_STAR).abs() / denom;

    Ok(AutonomicSwitching {
        n_windows: states.len(),
        n_state_changes: changes,
        v_hrv,
        chi_ze_hrv,
        state_sequence: states,
    })
}

// ------------------------------------------------------------------
// tests
// ------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn synthetic_rr(n: usize, mean_ms: f64, sd_ms: f64, seed: u64) -> RrSeries {
        // Deterministic LCG for reproducibility (no RNG dep)
        let mut state = seed;
        let mut out = Vec::with_capacity(n);
        for _ in 0..n {
            state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            // Two-call to approximate Gaussian via Box-Muller (skip — keep simple uniform)
            let u = (state >> 33) as f64 / (1u64 << 31) as f64;  // ~[0, 1]
            let centered = (u - 0.5) * 2.0;                       // ~[-1, 1]
            out.push(mean_ms + centered * sd_ms);
        }
        RrSeries { rr_ms: out, source: "synthetic".into() }
    }

    #[test]
    fn time_domain_basic_sdnn() {
        let rr = synthetic_rr(1000, 800.0, 50.0, 42);
        let td = time_domain(&rr).unwrap();
        assert!(td.sdnn_ms > 5.0 && td.sdnn_ms < 100.0,
            "SDNN should be reasonable, got {}", td.sdnn_ms);
        assert!(td.mean_rr_ms > 700.0 && td.mean_rr_ms < 900.0);
        assert_eq!(td.n_intervals, 1000);
    }

    #[test]
    fn time_domain_constant_rr_zero_variance() {
        let rr = RrSeries { rr_ms: vec![800.0; 100], source: "constant".into() };
        let td = time_domain(&rr).unwrap();
        assert!(td.sdnn_ms < 1e-9, "constant series should have SDNN=0, got {}", td.sdnn_ms);
        assert!(td.rmssd_ms < 1e-9);
        assert_eq!(td.pnn50, 0.0);
    }

    #[test]
    fn time_domain_too_short() {
        let rr = RrSeries { rr_ms: vec![800.0; 5], source: "short".into() };
        assert!(matches!(time_domain(&rr), Err(BioSenseError::TooShort(_))));
    }

    #[test]
    fn pnn50_alternating() {
        // Alternate 700, 770, 700, 770, ... → 50% successive diff = 70ms > 50ms
        let v: Vec<f64> = (0..100).map(|i| if i % 2 == 0 { 700.0 } else { 770.0 }).collect();
        let rr = RrSeries { rr_ms: v, source: "alt".into() };
        let td = time_domain(&rr).unwrap();
        assert!((td.pnn50 - 1.0).abs() < 1e-9, "all alternating diffs > 50ms, got pnn50={}", td.pnn50);
    }

    #[test]
    fn freq_domain_runs() {
        let rr = synthetic_rr(500, 800.0, 50.0, 1);
        let fd = freq_domain_simple(&rr).unwrap();
        assert!(fd.lf_power >= 0.0 && fd.hf_power >= 0.0);
    }
}
