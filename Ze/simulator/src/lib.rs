//! Ze Theory simulators — quantitative blocks from `Ze Theory.pdf` (24-chapter monograph).
//!
//! Modules map to book chapters:
//!
//! | Module       | Book chapters             | Topic                                    |
//! |--------------|---------------------------|------------------------------------------|
//! | `impedance`  | §2 §3 §5 §12              | I(τ) ODE; t = ∫I dτ; K, C, Φ_Ze           |
//! | `hierarchy`  | §5.1–§5.7                 | Universe → Dark E → Energy → K → t → Being |
//! | `chsh`       | §7 §8.4 §19               | Ze-deformation of Bell + quantum damping  |
//! | `cosmology`  | §10 (Cosmology of Imp.)   | Ï + 3HÏ + m²I = 3(ä/a)/Λ_Ze (homogeneous) |
//! | `autowaves`  | §13 §17                   | 1D reaction-diffusion cheating autowaves  |
//!
//! Source of truth: `Ze Theory.pdf` / `Ze Теория.pdf` in `~/Desktop/LongevityCommon/Ze/`.

use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------
// 1. Impedance ODE — Ze Theory.pdf §2, §3, §12
// ------------------------------------------------------------------

pub mod impedance {
    use super::*;

    /// Ze Theory.pdf §2.3 properties: I ≥ 0, I=0 iff perfect model, dI/dt ≥ 0 on average.
    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct Params {
        pub i0: f64,         // initial impedance, §2.3 — small
        pub i_max: f64,      // pure-learning threshold, §18.3
        pub lambda: f64,     // learning rate
        pub sigma_base: f64, // baseline sensory drive
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

    /// Sensory drive σ(τ) per scenario. See PARAMETERS.md §1, Ze Theory.pdf §3.4 (psychological time).
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

    /// dI/dτ = σ − λ·I·(1 − I/I_max) — closed scalar agent-impedance model.
    pub fn deriv(p: &Params, tau: f64, i: f64) -> f64 {
        let (sigma, lam) = drive(p, tau);
        sigma - lam * i * (1.0 - i / p.i_max)
    }

    /// RK4 integrator step.
    pub fn rk4(p: &Params, tau: f64, h: f64, i: f64) -> f64 {
        let k1 = deriv(p, tau, i);
        let k2 = deriv(p, tau + h / 2.0, i + h * k1 / 2.0);
        let k3 = deriv(p, tau + h / 2.0, i + h * k2 / 2.0);
        let k4 = deriv(p, tau + h, i + h * k3);
        i + h / 6.0 * (k1 + 2.0 * k2 + 2.0 * k3 + k4)
    }

    /// Trajectory of all derived quantities (K, C, t_phys, Φ_Ze) per Ze Theory.pdf §5.5–§5.7, §12.2.
    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct Trajectory {
        pub tau: Vec<f64>,
        pub i: Vec<f64>,
        pub k: Vec<f64>,            // K = −I  (§5.5)
        pub t_phys: Vec<f64>,       // t = ∫ I dτ  (§3.1, §5.6)
        pub consciousness: Vec<f64>, // C = −dI/dτ  (§5.7, §12.1)
        pub phi_ze: f64,             // Φ_Ze = ∮ I dt  (§12.2)
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct RunConfig {
        pub params: Params,
        pub t_end: f64,
        pub h: f64,
        pub record_every: usize,
        /// Cheating spike per §13 — at τ apply I ← factor·I (active adaptation of reality).
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
// 2. Hierarchy of Generations — Ze Theory.pdf §5
// ------------------------------------------------------------------

/// Computes derived quantities from the hierarchy chapter:
/// `Universe → Dark Energy → Energy → Knowledge → Time → Being`.
///
/// dim(Z) ∝ t_D (§5.3); E ~ ‖dZ/dt‖ (§5.4); K = −I (§5.5); t = ∫I dτ (§5.6); C = −dI/dτ (§5.7).
pub mod hierarchy {
    use super::*;

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct Snapshot {
        pub tau: f64,
        pub dim_z: f64,         // dark energy: dim(Z) growth
        pub energy: f64,        // E ~ ‖dZ/dt‖
        pub knowledge: f64,     // K = −I
        pub time_phys: f64,     // t = ∫ I dτ
        pub consciousness: f64, // C = −dI/dτ
    }

    /// Project an `impedance::Trajectory` into the 6-level hierarchy, exposing all derived
    /// quantities of Ze Theory.pdf §5 in one struct. `dim_z(tau) = dim_z0 + dim_growth_rate · tau`
    /// is a placeholder slow growth — real cosmological growth is in `cosmology` module.
    pub fn project(
        tr: &super::impedance::Trajectory,
        dim_z0: f64,
        dim_growth_rate: f64,
    ) -> Vec<Snapshot> {
        let n = tr.tau.len();
        let mut out = Vec::with_capacity(n);
        for k in 0..n {
            let tau = tr.tau[k];
            // E ~ ‖dZ/dt‖ proxy: take |dI/dτ| as scalar measure of state-change rate
            let energy = if k > 0 {
                let dt = tau - tr.tau[k - 1];
                if dt > 0.0 {
                    (tr.i[k] - tr.i[k - 1]).abs() / dt
                } else {
                    0.0
                }
            } else {
                0.0
            };
            out.push(Snapshot {
                tau,
                dim_z: dim_z0 + dim_growth_rate * tau,
                energy,
                knowledge: tr.k[k],
                time_phys: tr.t_phys[k],
                consciousness: tr.consciousness[k],
            });
        }
        out
    }

    /// Invariant K + I = 0 (Ze Theory.pdf §5.5). Returns max abs deviation in trajectory.
    pub fn check_k_plus_i_zero(tr: &super::impedance::Trajectory) -> f64 {
        tr.i
            .iter()
            .zip(tr.k.iter())
            .map(|(&i, &k)| (i + k).abs())
            .fold(0.0_f64, f64::max)
    }
}

// ------------------------------------------------------------------
// 3. CHSH Ze-deformation — Ze Theory.pdf §7, §8.4, §19
// ------------------------------------------------------------------

pub mod chsh {
    use super::*;

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct Params {
        pub alpha: f64,  // §19.3 — α ≈ 0.03 for BBO crystal
        pub delta0: f64, // §7.2 — Ze deformation amplitude
        pub h: f64,      // §19.4 — entropy modulator H ∈ [0, 1]
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

    /// Standard singlet correlation: E(a, b) = −cos(a − b). Ze Theory.pdf §7.1.
    pub fn e_qm(a: f64, b: f64) -> f64 {
        -(a - b).cos()
    }

    /// Ze-deformed correlation: E_Ze = E_QM + δ · [cos²(a − b) − 1/3].
    /// Ze Theory.pdf §7.2; δ = δ₀ · (1 − 2αH) per §8.4.
    pub fn e_ze(p: &Params, a: f64, b: f64) -> f64 {
        let c = (a - b).cos();
        let delta = p.delta0 * (1.0 - 2.0 * p.alpha * p.h);
        e_qm(a, b) + delta * (c * c - 1.0 / 3.0)
    }

    /// Tsirelson bound 2√2.
    pub fn s_qm() -> f64 {
        2.0_f64.sqrt() * 2.0
    }

    /// CHSH at singlet-optimal angles (a₁=0, a₂=π/2, b₁=π/4, b₂=3π/4).
    /// Maximises |S_QM| but uniform Ze correction → small shift δ/3.
    pub fn s_ze(p: &Params) -> f64 {
        let a1 = 0.0_f64;
        let a2 = std::f64::consts::FRAC_PI_2;
        let b1 = std::f64::consts::FRAC_PI_4;
        let b2 = 3.0 * std::f64::consts::FRAC_PI_4;
        (e_ze(p, a1, b1) - e_ze(p, a1, b2) + e_ze(p, a2, b1) + e_ze(p, a2, b2)).abs()
    }

    /// Ze-shift-optimal angles (Ze Theory.pdf §7.4): (0°, 45°, 22.5°, −22.5°).
    /// CHSH variant: S = E11 + E12 + E21 − E22.
    /// Returns (|S_QM|, |S_Ze|). |ΔS_Ze| reaches δ · 1.7478 (max), at cost of |S_QM| ≈ 2.389.
    pub fn s_ze_shift_optimal(p: &Params) -> (f64, f64) {
        let a1 = 0.0_f64;
        let a2 = std::f64::consts::FRAC_PI_4;
        let b1 = std::f64::consts::FRAC_PI_8;
        let b2 = -std::f64::consts::FRAC_PI_8;
        let eze = |a, b| e_ze(p, a, b);
        let s_qm_abs = (e_qm(a1, b1) + e_qm(a1, b2) + e_qm(a2, b1) - e_qm(a2, b2)).abs();
        let s_ze_abs = (eze(a1, b1) + eze(a1, b2) + eze(a2, b1) - eze(a2, b2)).abs();
        (s_qm_abs, s_ze_abs)
    }

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct HSweep {
        pub h: Vec<f64>,
        pub s_qm: Vec<f64>,
        pub s_ze: Vec<f64>,
        pub s_damped: Vec<f64>, // §8.4: S(H) = 2.828·(1 − 2αH)
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

        // §19.4: σ_S ≈ 0.002 at N = 1e9 → N_required = 1e9 · (0.002/(shift/5))²
        let target_sigma = shift.abs() / 5.0;
        let n_required = if target_sigma > 0.0 {
            1.0e9 * (0.002_f64 / target_sigma).powi(2)
        } else {
            f64::INFINITY
        };

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
// 4. Cosmology of Impedance — Ze Theory.pdf §10
// ------------------------------------------------------------------

/// Homogeneous cosmological solution: Ï + 3H Ï + m_Ze² I = 3 (ä/a) / Λ_Ze (Ze Theory.pdf §10).
///
/// Inflation when I > I_crit; bounce instead of singularity at I → I_max (§10.1).
/// Dark energy current ≈ const → V(I₀) acts as cosmological constant (§10.3).
pub mod cosmology {
    use super::*;

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct Params {
        pub m_ze: f64,        // mass of Ze field
        pub lambda_ze: f64,   // Ze scale (Λ_Ze)
        pub h0: f64,          // H₀ — Hubble parameter (book: 69.2 ± 1.8 km/s/Mpc)
        pub a_ddot_over_a: f64, // ä/a — second derivative of scale factor
        pub i0: f64,
        pub i_dot0: f64,
        pub i_max: f64,       // bounce threshold, §10.1
    }

    impl Default for Params {
        fn default() -> Self {
            Self {
                m_ze: 1.0,
                lambda_ze: 1.0,
                h0: 0.69,
                a_ddot_over_a: 0.1,
                i0: 0.5,
                i_dot0: 0.0,
                i_max: 100.0,
            }
        }
    }

    /// d²I/dt² = 3(ä/a)/Λ_Ze − 3H · dI/dt − m_Ze² · I
    pub fn deriv(p: &Params, _t: f64, i: f64, i_dot: f64) -> f64 {
        3.0 * p.a_ddot_over_a / p.lambda_ze - 3.0 * p.h0 * i_dot - p.m_ze.powi(2) * i
    }

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct Trajectory {
        pub t: Vec<f64>,
        pub i: Vec<f64>,
        pub i_dot: Vec<f64>,
        pub bounced: bool,
    }

    /// Velocity-Verlet style integrator for second-order ODE.
    pub fn simulate(p: &Params, t_end: f64, dt: f64) -> Trajectory {
        let mut t = 0.0;
        let mut i = p.i0;
        let mut i_dot = p.i_dot0;
        let mut tr = Trajectory::default();
        let mut bounced = false;
        while t <= t_end {
            tr.t.push(t);
            tr.i.push(i);
            tr.i_dot.push(i_dot);

            // RK2 (midpoint) for second-order ODE
            let a1 = deriv(p, t, i, i_dot);
            let i_mid = i + i_dot * dt / 2.0;
            let i_dot_mid = i_dot + a1 * dt / 2.0;
            let a2 = deriv(p, t + dt / 2.0, i_mid, i_dot_mid);

            i += i_dot * dt + 0.5 * a2 * dt * dt;
            i_dot += a2 * dt;

            // Bounce: clamp at I_max with sign reversal of I_dot (§10.1)
            if i >= p.i_max {
                i = p.i_max;
                i_dot = -i_dot.abs();
                bounced = true;
            }
            t += dt;
        }
        tr.bounced = bounced;
        tr
    }
}

// ------------------------------------------------------------------
// 5. Cheating autowaves — Ze Theory.pdf §13, §17
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
        pub k_sig: f64, // sigmoid steepness — smoothed indicator [I > I_crit]
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

    /// 1D reaction-diffusion per Ze Theory.pdf §17:
    /// ∂I/∂t = D∇²I + α(1−x)I − βxy ;  ∂x/∂t = γI(1−x) − δx ;  ∂y/∂t = ε·σ_k(I−I_crit) − ζy
    pub fn simulate(params: Params, steps: usize, snapshot_every: usize) -> Run {
        let (mut i, mut x, mut y) = initial(&params);
        let mut i_next = i.clone();

        let mut out = Run {
            params: params.clone(),
            ..Default::default()
        };

        for s in 0..=steps {
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
// Tests F1–F8 — extended from previous F1–F6
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
        // Ze Theory.pdf §7.4: ΔS = δ · 1.7478 at angles (0°, 45°, 22.5°, −22.5°).
        let p = chsh::Params { alpha: 0.0, delta0: 0.1, h: 0.0 };
        let (s_qm_abs, s_ze_abs) = chsh::s_ze_shift_optimal(&p);
        let shift_mag = (s_ze_abs - s_qm_abs).abs();
        let expected = 0.1 * 1.7478;
        assert!(
            (shift_mag - expected).abs() < 1e-3,
            "shift-optimal |ΔS|={} expected {}",
            shift_mag, expected
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

    /// F7: hierarchy invariant K + I = 0 (Ze Theory.pdf §5.5).
    #[test]
    fn f7_hierarchy_k_plus_i_invariant() {
        let cfg = impedance::RunConfig::default();
        let tr = impedance::simulate(&cfg);
        let max_dev = hierarchy::check_k_plus_i_zero(&tr);
        assert!(max_dev < 1e-12, "K+I=0 invariant violated: {}", max_dev);
    }

    /// F8: cosmological bounce — I должен быть ограничен I_max (Ze Theory.pdf §10.1).
    #[test]
    fn f8_cosmology_bounce_at_i_max() {
        let p = cosmology::Params {
            m_ze: 0.0,            // нет восстанавливающей силы
            lambda_ze: 1.0,
            h0: 0.0,              // нет демпфирования
            a_ddot_over_a: 1.0,   // постоянное ускорение → I растёт
            i0: 0.0,
            i_dot0: 0.0,
            i_max: 5.0,
        };
        let tr = cosmology::simulate(&p, 100.0, 0.01);
        let max_i = tr.i.iter().cloned().fold(f64::MIN, f64::max);
        assert!(max_i <= p.i_max + 1e-6, "I exceeded I_max: {}", max_i);
        assert!(tr.bounced, "Expected bounce at I_max");
    }
}
