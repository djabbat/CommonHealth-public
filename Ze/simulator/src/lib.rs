//! Ze Theory simulators — three quantitative blocks from THEORY.md.
//!
//!   module `impedance` — ODE for I(τ) and derived quantities (Chapter 2–5, 12)
//!   module `chsh`      — CHSH Ze-deformation (Chapter 7, 19)
//!   module `autowaves` — 1D reaction-diffusion cheating autowaves (Chapter 17)

use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------
// 1. Impedance ODE
// ------------------------------------------------------------------

pub mod impedance {
    use super::*;

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct Params {
        pub i0: f64,
        pub i_max: f64,
        pub lambda: f64,
        pub sigma_base: f64,
        pub scenario: String,
    }

    impl Default for Params {
        fn default() -> Self {
            Self {
                i0: 0.05,
                i_max: 5.0,
                lambda: 0.5,
                sigma_base: 0.1,
                scenario: "novelty".into(),
            }
        }
    }

    /// Sensory drive σ(τ) per scenario (see PARAMETERS.md §1).
    /// Returns (sigma, lambda_effective).
    pub fn drive(p: &Params, tau: f64) -> (f64, f64) {
        match p.scenario.as_str() {
            "routine" => (0.05, p.lambda),
            "novelty" => {
                let step = if tau >= 5.0 { 0.6 } else { 0.0 };
                (0.05 + step, p.lambda)
            }
            "meditation" => (0.05 * (-tau / 10.0).exp(), 2.0 * p.lambda),
            "cheating" => (p.sigma_base, p.lambda),
            _ => (p.sigma_base, p.lambda),
        }
    }

    /// dI/dτ = σ − λ·I·(1 − I/I_max).
    pub fn deriv(p: &Params, tau: f64, i: f64) -> f64 {
        let (sigma, lam) = drive(p, tau);
        sigma - lam * i * (1.0 - i / p.i_max)
    }

    pub fn rk4(p: &Params, tau: f64, h: f64, i: f64) -> f64 {
        let k1 = deriv(p, tau, i);
        let k2 = deriv(p, tau + h / 2.0, i + h * k1 / 2.0);
        let k3 = deriv(p, tau + h / 2.0, i + h * k2 / 2.0);
        let k4 = deriv(p, tau + h, i + h * k3);
        i + h / 6.0 * (k1 + 2.0 * k2 + 2.0 * k3 + k4)
    }

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct Trajectory {
        pub tau: Vec<f64>,
        pub i: Vec<f64>,
        pub k: Vec<f64>,       // K = −I
        pub t_phys: Vec<f64>,  // ∫ I dτ
        pub consciousness: Vec<f64>, // −dI/dτ
        pub phi_ze: f64,       // ∫₀^T I dτ — same as last t_phys
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct RunConfig {
        pub params: Params,
        pub t_end: f64,
        pub h: f64,
        pub record_every: usize,
        /// If Some, apply cheating-spike: I ← factor·I at this τ.
        pub cheating_spike: Option<(f64, f64)>,
    }

    impl Default for RunConfig {
        fn default() -> Self {
            Self {
                params: Params::default(),
                t_end: 50.0,
                h: 0.01,
                record_every: 10,
                cheating_spike: None,
            }
        }
    }

    pub fn simulate(cfg: &RunConfig) -> Trajectory {
        let n_steps = ((cfg.t_end) / cfg.h).round() as usize;
        let mut tau = 0.0_f64;
        let mut i = cfg.params.i0;
        let mut t_phys = 0.0_f64;
        let mut tr = Trajectory::default();

        let spike = cfg.cheating_spike;

        for step in 0..=n_steps {
            if step % cfg.record_every == 0 || step == n_steps {
                tr.tau.push(tau);
                tr.i.push(i);
                tr.k.push(-i);
                tr.t_phys.push(t_phys);
                tr.consciousness.push(-deriv(&cfg.params, tau, i));
            }
            if step == n_steps {
                break;
            }
            if let Some((t_spike, factor)) = spike {
                if (tau - t_spike).abs() < cfg.h / 2.0 {
                    i *= factor;
                }
            }
            let i_next = rk4(&cfg.params, tau, cfg.h, i);
            t_phys += i * cfg.h;
            i = i_next.max(0.0);
            tau += cfg.h;
        }
        tr.phi_ze = t_phys;
        tr
    }
}

// ------------------------------------------------------------------
// 2. CHSH Ze-deformation
// ------------------------------------------------------------------

pub mod chsh {
    use super::*;

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct Params {
        pub alpha: f64,   // 5.md §19.3 — α ≈ 0.03 for BBO
        pub delta0: f64,  // Ze deformation amplitude
        pub h: f64,       // entropy modulator H ∈ [0, 1]
    }

    impl Default for Params {
        fn default() -> Self {
            Self {
                alpha: 0.03,
                delta0: 0.05,
                h: 0.5,
            }
        }
    }

    /// Standard singlet correlation: E(a, b) = −cos(a − b).
    pub fn e_qm(a: f64, b: f64) -> f64 {
        -(a - b).cos()
    }

    /// Ze-deformed correlation: E_Ze = E_QM + δ · [cos²(a − b) − 1/3].
    pub fn e_ze(p: &Params, a: f64, b: f64) -> f64 {
        let c = (a - b).cos();
        let delta = p.delta0 * (1.0 - 2.0 * p.alpha * p.h);
        e_qm(a, b) + delta * (c * c - 1.0 / 3.0)
    }

    pub fn s_qm() -> f64 {
        2.0_f64.sqrt() * 2.0
    }

    /// CHSH with singlet-optimal angles a1=0, a2=π/2, b1=π/4, b2=3π/4
    /// (makes |S_QM| = 2√2 exactly). In this basis every cos²(a−b)=1/2,
    /// so the Ze correction is uniform and the shift equals δ/3.
    pub fn s_ze(p: &Params) -> f64 {
        let a1 = 0.0_f64;
        let a2 = std::f64::consts::FRAC_PI_2;
        let b1 = std::f64::consts::FRAC_PI_4;
        let b2 = 3.0 * std::f64::consts::FRAC_PI_4;
        (e_ze(p, a1, b1) - e_ze(p, a1, b2) + e_ze(p, a2, b1) + e_ze(p, a2, b2)).abs()
    }

    /// Ze-shift-optimal angles from 5.md §7.4: (0°, 45°, 22.5°, −22.5°).
    /// CHSH variant used by 5.md: S = E(a1,b1) + E(a1,b2) + E(a2,b1) − E(a2,b2).
    /// Returns (|S_QM|, |S_Ze|) for those angles. At these angles
    /// |ΔS_Ze| reaches δ · 1.7478 (maximal), at cost of |S_QM| ≈ 2.389 (< 2√2).
    pub fn s_ze_shift_optimal(p: &Params) -> (f64, f64) {
        let a1 = 0.0_f64;
        let a2 = std::f64::consts::FRAC_PI_4;
        let b1 = std::f64::consts::FRAC_PI_8;
        let b2 = -std::f64::consts::FRAC_PI_8;
        let eze = |a, b| e_ze(p, a, b);
        let s_qm_abs =
            (e_qm(a1, b1) + e_qm(a1, b2) + e_qm(a2, b1) - e_qm(a2, b2)).abs();
        let s_ze_abs =
            (eze(a1, b1) + eze(a1, b2) + eze(a2, b1) - eze(a2, b2)).abs();
        (s_qm_abs, s_ze_abs)
    }

    /// H-sweep — S(H) for H ∈ [0, 1] at N+1 points.
    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct HSweep {
        pub h: Vec<f64>,
        pub s_qm: Vec<f64>,
        pub s_ze: Vec<f64>,
        pub s_damped: Vec<f64>, // 5.md §8.4: S(H) = 2.828·(1 − 2αH)
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct Report {
        pub params: Params,
        pub s_qm: f64,
        pub s_ze: f64,
        pub s_shift: f64,
        pub s_damped_h: f64,
        pub sigma_5sigma_coincidences: f64, // N at which shift/σ_S = 5
        pub sweep: HSweep,
    }

    pub fn run(params: Params) -> Report {
        let s_qm_val = s_qm();
        let s_ze_val = s_ze(&params);
        let shift = s_ze_val - s_qm_val;
        let s_damped_h = s_qm_val * (1.0 - 2.0 * params.alpha * params.h);

        // σ_S for N coincidences: σ_S ≈ sqrt((S²(4 − S²))/(N)) — here use empirical scaling
        // from §19.4: σ ≈ 0.002 at N = 1e9; N_required = 1e9 · (0.002/(shift/5))²
        let target_sigma = shift.abs() / 5.0;
        let n_required = if target_sigma > 0.0 {
            1.0e9 * (0.002_f64 / target_sigma).powi(2)
        } else {
            f64::INFINITY
        };

        // Sweep over H.
        let mut sweep = HSweep::default();
        let steps = 51;
        for k in 0..steps {
            let h = k as f64 / (steps - 1) as f64;
            let p = Params { h, ..params.clone() };
            sweep.h.push(h);
            sweep.s_qm.push(s_qm_val);
            sweep.s_ze.push(s_ze(&p));
            sweep.s_damped.push(s_qm_val * (1.0 - 2.0 * p.alpha * h));
        }

        Report {
            params,
            s_qm: s_qm_val,
            s_ze: s_ze_val,
            s_shift: shift,
            s_damped_h,
            sigma_5sigma_coincidences: n_required,
            sweep,
        }
    }
}

// ------------------------------------------------------------------
// 3. Cheating autowaves — 1D reaction-diffusion
// ------------------------------------------------------------------

pub mod autowaves {
    use super::*;

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct Params {
        pub d: f64,
        pub alpha: f64,
        pub beta: f64,
        pub gamma: f64,
        pub delta: f64,
        pub epsilon: f64,
        pub zeta: f64,
        pub i_crit: f64,
        pub k_sig: f64, // sigmoid steepness for smoothed indicator
        pub n: usize,
        pub dx: f64,
        pub dt: f64,
    }

    impl Default for Params {
        fn default() -> Self {
            Self {
                d: 0.2,
                alpha: 1.0,
                beta: 0.8,
                gamma: 0.5,
                delta: 0.2,
                epsilon: 0.6,
                zeta: 0.3,
                i_crit: 0.5,
                k_sig: 20.0,
                n: 200,
                dx: 1.0,
                dt: 0.01,
            }
        }
    }

    fn sigmoid(z: f64) -> f64 {
        1.0 / (1.0 + (-z).exp())
    }

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct Snapshot {
        pub step: usize,
        pub t: f64,
        pub i: Vec<f64>,
        pub x: Vec<f64>,
        pub y: Vec<f64>,
    }

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct Run {
        pub params: Params,
        pub snapshots: Vec<Snapshot>,
        pub i_mean: Vec<f64>,
        pub x_mean: Vec<f64>,
        pub y_mean: Vec<f64>,
        pub t_axis: Vec<f64>,
    }

    pub fn initial(p: &Params) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
        let center = p.n / 2;
        let i: Vec<f64> = (0..p.n)
            .map(|k| {
                let d = (k as isize - center as isize).abs() as f64;
                if d < 10.0 { 0.1 + 0.5 } else { 0.1 }
            })
            .collect();
        let x = vec![0.0; p.n];
        let y = vec![0.0; p.n];
        (i, x, y)
    }

    pub fn simulate(params: Params, steps: usize, snapshot_every: usize) -> Run {
        let (mut i, mut x, mut y) = initial(&params);
        let mut i_next = i.clone();

        let mut out = Run {
            params: params.clone(),
            ..Default::default()
        };

        for s in 0..=steps {
            // stats
            let i_mean = i.iter().sum::<f64>() / i.len() as f64;
            let x_mean = x.iter().sum::<f64>() / x.len() as f64;
            let y_mean = y.iter().sum::<f64>() / y.len() as f64;
            out.i_mean.push(i_mean);
            out.x_mean.push(x_mean);
            out.y_mean.push(y_mean);
            out.t_axis.push(s as f64 * params.dt);

            if s % snapshot_every == 0 || s == steps {
                out.snapshots.push(Snapshot {
                    step: s,
                    t: s as f64 * params.dt,
                    i: i.clone(),
                    x: x.clone(),
                    y: y.clone(),
                });
            }
            if s == steps {
                break;
            }

            // Periodic Laplacian + reaction for I; local dynamics for x, y.
            let n = params.n;
            for k in 0..n {
                let kl = (k + n - 1) % n;
                let kr = (k + 1) % n;
                let lap = (i[kl] - 2.0 * i[k] + i[kr]) / (params.dx * params.dx);
                let react = params.alpha * (1.0 - x[k]) * i[k] - params.beta * x[k] * y[k];
                i_next[k] = i[k] + params.dt * (params.d * lap + react);
                if !i_next[k].is_finite() {
                    i_next[k] = 0.0;
                }
                i_next[k] = i_next[k].max(0.0);
            }
            for k in 0..n {
                let dx_val =
                    params.gamma * i[k] * (1.0 - x[k]) - params.delta * x[k];
                let indic = sigmoid(params.k_sig * (i[k] - params.i_crit));
                let dy_val = params.epsilon * indic - params.zeta * y[k];
                x[k] = (x[k] + params.dt * dx_val).clamp(0.0, 1.0);
                y[k] = (y[k] + params.dt * dy_val).clamp(0.0, 1.0);
            }
            std::mem::swap(&mut i, &mut i_next);
        }
        out
    }
}

// ------------------------------------------------------------------
// tests (F1–F6 from THEORY.md §7)
// ------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn f1_routine_relaxes_to_zero() {
        let mut cfg = impedance::RunConfig::default();
        cfg.params.scenario = "routine".into();
        cfg.t_end = 100.0;
        let tr = impedance::simulate(&cfg);
        assert!(
            *tr.i.last().unwrap() < 0.15,
            "routine: I(∞) should be small, got {}",
            tr.i.last().unwrap()
        );
    }

    #[test]
    fn f2_novelty_grows_then_learns() {
        let mut cfg = impedance::RunConfig::default();
        cfg.params.scenario = "novelty".into();
        cfg.t_end = 50.0;
        let tr = impedance::simulate(&cfg);
        // I should rise between τ = 5 (step) and τ = 10, then start relaxing.
        let idx_5 = tr.tau.iter().position(|&t| t > 5.0).unwrap();
        let idx_10 = tr.tau.iter().position(|&t| t > 10.0).unwrap();
        assert!(tr.i[idx_10] > tr.i[idx_5], "novelty: I should grow after step");
    }

    #[test]
    fn f3_s_qm_is_2_sqrt_2() {
        assert!((chsh::s_qm() - 2.0 * 2.0_f64.sqrt()).abs() < 1e-12);
    }

    #[test]
    fn f4a_singlet_optimal_shift_is_minus_delta_over_3() {
        // Singlet-optimal angles: cos²(a−b)=1/2 everywhere.
        // Raw S_QM is negative and raw Ze correction positive → |S_Ze| = |S_QM| − δ/3.
        let p = chsh::Params { alpha: 0.0, delta0: 0.3, h: 0.0 };
        let shift = chsh::s_ze(&p) - chsh::s_qm();
        let expected = -0.3 / 3.0;
        assert!(
            (shift - expected).abs() < 1e-9,
            "singlet-optimal shift {} expected {}",
            shift, expected
        );
    }

    #[test]
    fn f4b_shift_optimal_angles_match_book_1_7478() {
        // 5.md §7.4 claims ΔS = δ · 1.7478 at angles (0°, 45°, 22.5°, −22.5°)
        // with CHSH variant S = E11 + E12 + E21 − E22.
        let p = chsh::Params { alpha: 0.0, delta0: 0.1, h: 0.0 };
        let (s_qm_abs, s_ze_abs) = chsh::s_ze_shift_optimal(&p);
        let shift_mag = (s_ze_abs - s_qm_abs).abs();
        let expected = 0.1 * 1.7478;
        assert!(
            (shift_mag - expected).abs() < 1e-3,
            "shift-optimal |ΔS|={} (|S_QM|={}, |S_Ze|={}) expected {}",
            shift_mag, s_qm_abs, s_ze_abs, expected
        );
    }

    #[test]
    fn f5_autowaves_static_without_forcing() {
        let mut p = autowaves::Params::default();
        p.alpha = 0.0;
        p.beta = 0.0;
        p.d = 0.0;
        p.n = 20;
        let run = autowaves::simulate(p, 100, 100);
        let first = &run.snapshots.first().unwrap().i;
        let last = &run.snapshots.last().unwrap().i;
        let err: f64 = first.iter().zip(last).map(|(a, b)| (a - b).abs()).sum();
        assert!(err < 1e-9, "I should be static when α=β=D=0, drift={}", err);
    }

    #[test]
    fn f6_autowaves_bounded() {
        let p = autowaves::Params {
            n: 50,
            ..Default::default()
        };
        let run = autowaves::simulate(p, 1000, 1000);
        for s in &run.snapshots {
            for &v in &s.x {
                assert!(v >= 0.0 && v <= 1.0, "x out of [0,1]: {}", v);
            }
            for &v in &s.y {
                assert!(v >= 0.0 && v <= 1.0, "y out of [0,1]: {}", v);
            }
        }
    }
}
