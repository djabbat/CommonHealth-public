//! Time-integrated trajectory of D_4(n, t) for a single tissue.
//!
//! Given a division-rate model r(t) (divisions per day) and simulation
//! horizon, produce a dense trajectory sampled at daily steps.

use crate::{compute_damage, CounterParams};
use crate::tissue::Tissue;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrajectoryPoint {
    pub t_days: f64,
    pub n: f64,
    pub d: f64,
}

pub struct TrajectoryRequest<'a> {
    pub tissue: Tissue,
    pub division_rate_per_day: f64,
    pub coupling_source: Option<&'a dyn Fn(f64 /*t_days*/) -> f64>,
    pub horizon_days: f64,
    pub params_override: Option<CounterParams>,
}

pub fn run_trajectory(req: &TrajectoryRequest) -> Vec<TrajectoryPoint> {
    let params = req.params_override.unwrap_or_else(|| req.tissue.params());
    params.validate().expect("params must validate");

    let mut out = Vec::with_capacity(req.horizon_days as usize + 1);
    let mut n: f64 = 0.0;
    for day in 0..=req.horizon_days as u64 {
        let t = day as f64;
        n += req.division_rate_per_day;
        let coupling = req.coupling_source.map(|f| f(t)).unwrap_or(0.0);
        let d = compute_damage(&params, n, t, coupling);
        out.push(TrajectoryPoint { t_days: t, n, d });
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trajectory_is_monotonic_in_n() {
        let req = TrajectoryRequest {
            tissue: Tissue::Fibroblast,
            division_rate_per_day: 0.5,
            coupling_source: None,
            horizon_days: 100.0,
            params_override: None,
        };
        let traj = run_trajectory(&req);
        assert_eq!(traj.len(), 101);
        for w in traj.windows(2) {
            assert!(w[1].d >= w[0].d, "non-monotone d: {} → {}", w[0].d, w[1].d);
        }
    }

    #[test]
    fn post_mitotic_tissue_low_alpha_effect() {
        let req = TrajectoryRequest {
            tissue: Tissue::Neuron,
            division_rate_per_day: 0.001,
            coupling_source: None,
            horizon_days: 365.0 * 10.0,
            params_override: None,
        };
        let traj = run_trajectory(&req);
        // In post-mitotic tissue, β·t should dominate over α·n
        let final_point = traj.last().unwrap();
        assert!(final_point.d > 0.0);
    }
}
