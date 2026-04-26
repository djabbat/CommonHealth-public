//! MCOA Counter #2: Telomere shortening
//!
//! Kinetic equation (MCOA-compliant, dimensionless):
//!   D_2(n, t) = D_20 + α_2·(n / n_2*) + β_2·(t / τ_2) + γ_2·I(others)
//!
//! All parameters are dimensionless; input n is integer division count,
//! input t is time in days (internally normalised to τ).

pub mod tissue;
pub mod trajectory;

use serde::{Deserialize, Serialize};

pub const COUNTER_NUMBER: u8 = 2;
pub const COUNTER_NAME: &str = "Telomere shortening";

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct CounterState {
    pub d: f64,
    pub n: f64,
    pub t_days: f64,
}

impl CounterState {
    pub fn origin() -> Self {
        Self { d: 0.0, n: 0.0, t_days: 0.0 }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CounterParams {
    pub d0: f64,
    pub alpha: f64,
    pub beta: f64,
    pub gamma: f64,
    pub n_star: f64,
    pub tau_days: f64,
    pub d_critical: f64,
}

impl Default for CounterParams {
    fn default() -> Self {
        Self {
            d0: 0.0,
            alpha: 0.5500,
            beta:  0.0000,
            gamma: 0.0,
            n_star: 50.00,
            tau_days: 32850.0,
            d_critical: 0.5500,
        }
    }
}

impl CounterParams {
    pub fn validate(&self) -> Result<(), String> {
        if self.alpha < 0.0 { return Err(format!("alpha<0: {}", self.alpha)); }
        if self.beta  < 0.0 { return Err(format!("beta<0: {}",  self.beta));  }
        if self.n_star <= 0.0 { return Err(format!("n_star<=0: {}", self.n_star)); }
        if self.tau_days <= 0.0 { return Err(format!("tau_days<=0: {}", self.tau_days)); }
        if !self.d_critical.is_finite() || self.d_critical <= 0.0 {
            return Err(format!("d_critical invalid: {}", self.d_critical));
        }
        Ok(())
    }
}

/// Compute dimensionless damage D at (n, t) with external coupling influence.
pub fn compute_damage(p: &CounterParams, n: f64, t_days: f64, coupling: f64) -> f64 {
    p.d0
        + p.alpha * (n / p.n_star)
        + p.beta  * (t_days / p.tau_days)
        + p.gamma * coupling
}

/// Has the counter crossed its tissue-specific critical threshold?
pub fn is_above_critical(p: &CounterParams, n: f64, t_days: f64, coupling: f64) -> bool {
    compute_damage(p, n, t_days, coupling) >= p.d_critical
}

/// Solve for n at which D first equals d_critical at given t_days.
/// Returns None if the counter never crosses (because α == 0 or solution is
/// beyond a tissue-reasonable division ceiling of 10 × n_star).
pub fn divisions_to_critical(p: &CounterParams, t_days: f64, coupling: f64) -> Option<f64> {
    if p.alpha <= 0.0 { return None; }
    let slack = p.d_critical - p.d0 - p.beta*(t_days/p.tau_days) - p.gamma*coupling;
    if slack <= 0.0 { return Some(0.0); }
    let n = slack * p.n_star / p.alpha;
    if n > 10.0 * p.n_star { None } else { Some(n) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_params_validate() {
        CounterParams::default().validate().unwrap();
    }

    #[test]
    fn origin_gives_d0() {
        let p = CounterParams::default();
        assert!((compute_damage(&p, 0.0, 0.0, 0.0) - p.d0).abs() < 1e-12);
    }

    #[test]
    fn linear_in_n() {
        let p = CounterParams { d0:0.0, alpha:1.0, beta: 0.0, gamma:0.0, n_star:1.0, tau_days:365.0, d_critical:1.0 };
        assert_eq!(compute_damage(&p, 3.0, 0.0, 0.0), 3.0);
    }

    #[test]
    fn linear_in_t() {
        let p = CounterParams { d0:0.0, alpha:0.0, beta: 1.0, gamma:0.0, n_star:50.0, tau_days:1.0, d_critical:1.0 };
        assert_eq!(compute_damage(&p, 0.0, 4.0, 0.0), 4.0);
    }

    #[test]
    fn crosses_critical_via_n() {
        // Use an explicit division-dominant param set so the test is
        // counter-agnostic and always crosses within 10·n_star.
        let p = CounterParams {
            d0: 0.0,
            alpha: 1.0,
            beta: 0.0,
            gamma: 0.0,
            n_star: 50.0,
            tau_days: 365.0,
            d_critical: 1.0,
        };
        let n_crit = divisions_to_critical(&p, 0.0, 0.0).expect("should cross");
        assert!(n_crit > 0.0);
        let d = compute_damage(&p, n_crit, 0.0, 0.0);
        assert!((d - p.d_critical).abs() < 1e-9);
    }

    #[test]
    fn coupling_moves_damage() {
        let mut p = CounterParams::default();
        p.gamma = 0.1;
        let base = compute_damage(&p, 10.0, 100.0, 0.0);
        let perturbed = compute_damage(&p, 10.0, 100.0, 0.5);
        assert!(perturbed > base);
    }
}
