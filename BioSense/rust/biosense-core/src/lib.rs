//! BioSense core — types and Ze constants.
//!
//! Constants must match `~/Desktop/LongevityCommon/server/src/services/ze_compute.rs`.

use serde::{Deserialize, Serialize};
use thiserror::Error;

// ── Ze constants (server/src/services/ze_compute.rs — must match) ──
pub const V_STAR: f64 = 0.45631;
pub const K_DUAL: f64 = 0.45;
pub const K_EEG_ONLY: f64 = 0.42;
pub const K_HRV_ONLY: f64 = 0.38;
pub const D_NORM_ALPHA: f64 = 1.2;
pub const DEFAULT_SE_CHI: f64 = 0.05;

// ── HRV / autonomic ──
pub const DELTA_HYS: f64 = 0.10;        // 10% hysteresis
pub const WINDOW_S: f64 = 300.0;        // 5 min analysis window
pub const OVERLAP: f64 = 0.50;          // 50% overlap
pub const LF_LOW: f64 = 0.04;
pub const LF_HIGH: f64 = 0.15;
pub const HF_LOW: f64 = 0.15;
pub const HF_HIGH: f64 = 0.40;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RrSeries {
    /// RR intervals in milliseconds.
    pub rr_ms: Vec<f64>,
    /// Sampling source — for traceability.
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EegSignal {
    /// Sample frequency in Hz.
    pub fs: f64,
    /// Channel labels (e.g., ["Fp1", "Fp2", "F3", ...]).
    pub channels: Vec<String>,
    /// Samples — flat row-major: ch0_t0, ch1_t0, ..., chN_t0, ch0_t1, ...
    pub samples: Vec<f32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AutonomicState {
    Sympathetic,
    Parasympathetic,
    Unchanged,
}

#[derive(Debug, Error)]
pub enum BioSenseError {
    #[error("RR series too short: {0} intervals (need ≥ 10)")]
    TooShort(usize),
    #[error("invalid sampling: {0}")]
    InvalidSampling(String),
    #[error("computation error: {0}")]
    Compute(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constants_match_server() {
        // Sanity: v_star must equal server constant; if changed, update both sides.
        assert!((V_STAR - 0.45631).abs() < 1e-12);
        assert!((K_DUAL - 0.45).abs() < 1e-12);
    }

    #[test]
    fn delta_hys_in_range() {
        assert!(DELTA_HYS > 0.0 && DELTA_HYS < 1.0);
    }
}
