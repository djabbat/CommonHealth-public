//! Integration tests for MCOA Counter #4.

use epigenetic_counter::{compute_damage, CounterParams, divisions_to_critical};
use epigenetic_counter::tissue::Tissue;
use epigenetic_counter::trajectory::{run_trajectory, TrajectoryRequest};

#[test]
fn six_tissue_panel_all_run() {
    for tissue in Tissue::all() {
        let req = TrajectoryRequest {
            tissue: *tissue,
            division_rate_per_day: 0.01,
            coupling_source: None,
            horizon_days: 365.0 * 5.0,
            params_override: None,
        };
        let traj = run_trajectory(&req);
        assert!(!traj.is_empty());
        let final_d = traj.last().unwrap().d;
        assert!(final_d.is_finite(), "NaN/inf d for tissue {:?}", tissue);
    }
}

#[test]
fn tissue_params_validate() {
    for tissue in Tissue::all() {
        tissue.params().validate().unwrap();
    }
}

#[test]
fn divisions_to_critical_monotone_in_coupling() {
    let p = CounterParams {
        d0: 0.0, alpha: 1.0, beta: 0.0, gamma: 0.1,
        n_star: 50.0, tau_days: 365.0, d_critical: 1.0,
    };
    let n_low_coupling  = divisions_to_critical(&p, 100.0, 0.0).unwrap();
    let n_high_coupling = divisions_to_critical(&p, 100.0, 0.5).unwrap();
    // Higher coupling → less room before critical → fewer divisions needed.
    assert!(n_high_coupling < n_low_coupling);
}
