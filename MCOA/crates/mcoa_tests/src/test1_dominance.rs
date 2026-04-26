//! Test 1 — tissue-specific counter dominance.
//!
//! Simulates six tissues × four counters × four time-points across N=85 synthetic animals per
//! time-point, then runs a multiple linear regression per tissue to identify the dominant counter
//! (Benjamini–Hochberg FDR q < 0.05 across 96 tests).
//!
//! This is the stub; full implementation in v0.3.

pub struct Test1Config {
    pub n_per_timepoint: usize,   // 85
    pub timepoints_months: Vec<u32>, // [3, 12, 18, 24]
}

impl Default for Test1Config {
    fn default() -> Self {
        Self { n_per_timepoint: 85, timepoints_months: vec![3, 12, 18, 24] }
    }
}
