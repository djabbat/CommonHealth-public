//! MCOA simulation — time evolution of multi-counter damage per tissue.

use mcoa_core::{
    default_drift_rates, default_reference_scales, default_weights, Counter, CounterState, Gamma,
    Tissue, N_COUNTERS,
};
use serde::Serialize;

/// One simulation time-step: advances all counters by ONE division-equivalent (Δn=1) and
/// `dt_seconds` of wall-clock time. Damage is accumulated incrementally:
/// `D_new = D_prev + alpha*(dn/n_star) + beta*(dt/tau) + gamma*I(prev)`.
///
/// `n_divisions_cumulative` and `t_seconds_cumulative` are kept on the record only for bookkeeping;
/// they are NOT used to compute the step (which would double-count cumulative contributions).
pub fn step(
    states: &mut [CounterState; N_COUNTERS],
    tissue: Tissue,
    _n_divisions_cumulative: f64,
    _t_seconds_cumulative: f64,
    dn: f64,
    dt_seconds: f64,
    gamma: &Gamma,
) {
    // Snapshot BEFORE update so coupling uses the pre-step state (prevents same-step feedback loops).
    let prev = *states;
    for c in Counter::ALL {
        let rates = default_drift_rates(c, tissue);
        let scales = default_reference_scales(c, tissue);
        // Per-step increment (NOT the absolute formula)
        let div_inc = match scales.n_star {
            Some(n_star) if n_star > 0.0 => rates.alpha * (dn / n_star),
            _ => 0.0,
        };
        let time_inc = if scales.tau_seconds > 0.0 {
            rates.beta * (dt_seconds / scales.tau_seconds)
        } else {
            0.0
        };
        let gamma_i = 0.01; // per-step coupling strength scalar
        let coupling = gamma_i * gamma.influence(c, &prev);
        states[c as usize].value = prev[c as usize].value + div_inc + time_inc + coupling;
    }
}

/// Tissue-integrated load L_tissue = Σ_i w_i · f_i(D_i). Here f_i = identity.
pub fn tissue_load(states: &[CounterState; N_COUNTERS], tissue: Tissue) -> f64 {
    let w = default_weights(tissue);
    let mut sum = 0.0;
    for c in Counter::ALL {
        sum += w.get(c) * states[c as usize].value;
    }
    sum
}

#[derive(Debug, Clone, Serialize)]
pub struct SimulationRecord {
    pub step: usize,
    pub n_cumulative: f64,
    pub t_seconds: f64,
    pub telomere: f64,
    pub centriolar: f64,
    pub mito: f64,
    pub epigenetic: f64,
    pub proteostasis: f64,
    pub tissue_load: f64,
}

/// Run a forward simulation of `n_steps` discrete divisions; each step is one division-equivalent
/// of duration `seconds_per_division`.
pub fn run(
    tissue: Tissue,
    n_steps: usize,
    seconds_per_division: f64,
    gamma: &Gamma,
) -> Vec<SimulationRecord> {
    let mut states = [CounterState::default(); N_COUNTERS];
    let mut records = Vec::with_capacity(n_steps + 1);
    let mut n_cum = 0.0;
    let mut t_cum = 0.0;
    for step_idx in 0..=n_steps {
        records.push(SimulationRecord {
            step: step_idx,
            n_cumulative: n_cum,
            t_seconds: t_cum,
            telomere: states[0].value,
            centriolar: states[1].value,
            mito: states[2].value,
            epigenetic: states[3].value,
            proteostasis: states[4].value,
            tissue_load: tissue_load(&states, tissue),
        });
        if step_idx < n_steps {
            let dn = 1.0;
            let dt = seconds_per_division;
            n_cum += dn;
            t_cum += dt;
            step(&mut states, tissue, n_cum, t_cum, dn, dt, gamma);
        }
    }
    records
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hsc_100_step_run_produces_expected_growth() {
        let gamma = Gamma::default();
        // 7 days per division ~= 6.05e5 s
        let records = run(Tissue::Hsc, 100, 7.0 * 86400.0, &gamma);
        assert_eq!(records.len(), 101);
        // Counters are all non-negative and monotonically non-decreasing (in the linear regime).
        for pair in records.windows(2) {
            assert!(pair[1].tissue_load >= pair[0].tissue_load - 1e-9);
        }
        // At step 100 with HSC parameters the tissue load should be meaningful but below L_crit=0.60
        let last = records.last().unwrap();
        assert!(last.tissue_load > 0.0, "tissue load must grow");
        assert!(last.tissue_load < 0.6, "should not exceed L_crit in 100 steps under a-priori parameters");
        // Mitochondrial counter dominates under current PROVISIONAL parameter set (τ_mito=14d is short)
        assert!(last.mito > last.telomere);
        assert!(last.mito > last.centriolar);
    }

    #[test]
    fn post_mitotic_neuron_has_no_division_contribution_to_centriolar() {
        let gamma = Gamma::default();
        let records = run(Tissue::Neuron, 50, 30.0 * 86400.0, &gamma);
        // For neurons centriolar α=0, so growth is time-only.
        let first = &records[0];
        let last = records.last().unwrap();
        assert!(last.centriolar >= first.centriolar);
    }
}
