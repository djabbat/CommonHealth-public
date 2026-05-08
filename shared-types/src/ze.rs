//! Canonical Ze theory constants.  Authoritative source: `~/Desktop/LongevityCommon/PARAMETERS.md §1`.
//!
//! Two coexistent conventions exist:
//!
//! - **Article form** (`v*_active = −0.08738`) — authoritative for cross-subproject APIs,
//!   manuscripts, grants, external interfaces.
//! - **Python form** (`v*_active = 0.45631`) — internal helper used in Ze theorem proofs
//!   and Ze/Python code.
//!
//! Conversion: `Article = 2 · Python − 1` (linear bijection on `[0, 1] ↔ [−1, 1]`).

/// `v*_active` in Article form (canonical for inter-subproject exchange).
pub const V_STAR_ACTIVE_ARTICLE: f64 = -0.08738;

/// `v*_active` in Python form (internal Ze code).
pub const V_STAR_ACTIVE_PYTHON: f64 = 0.45631;

/// `v*_passive = 1 − ln 2` (Shannon-optimal point of the passive counter).
pub const V_STAR_PASSIVE_PYTHON: f64 = 0.30685281944005463; // 1.0 - LN_2

/// `v*_passive` in Article form: `−0.3862…`
pub const V_STAR_PASSIVE_ARTICLE: f64 = 2.0 * V_STAR_PASSIVE_PYTHON - 1.0;

/// CHSH deformation constant from `Ze/THEORY §3.4` Lemma C.
pub const CHSH_DEFORMATION: f64 = 1.7478;

/// Tsirelson bound `2√2 ≈ 2.828427…` — QM upper bound for CHSH.
pub const TSIRELSON_BOUND: f64 = 2.8284271247461903;

/// Empirical 95 % CI for `v*_active` from All-of-Us N=500 swept search (article §4.4).
pub const V_STAR_ACTIVE_EMPIRICAL_LOW_ARTICLE: f64 = -0.114;
pub const V_STAR_ACTIVE_EMPIRICAL_MID_ARTICLE: f64 = -0.098;
pub const V_STAR_ACTIVE_EMPIRICAL_HIGH_ARTICLE: f64 = -0.082;

/// Convert a Python-form value to Article form.
#[inline]
pub fn python_to_article(p: f64) -> f64 {
    2.0 * p - 1.0
}

/// Convert an Article-form value to Python form.
#[inline]
pub fn article_to_python(a: f64) -> f64 {
    (a + 1.0) / 2.0
}

/// `f_opt = v*_active · fs / 2` — Ze-optimal frequency at sample rate `fs` (Hz).
/// Uses Python form internally per BioSense convention; output in Hz.
#[inline]
pub fn f_opt_hz(fs_hz: f64) -> f64 {
    V_STAR_ACTIVE_PYTHON * fs_hz / 2.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conversion_roundtrip() {
        for p in [0.0, 0.30685281944, 0.45631, 0.5, 1.0] {
            let a = python_to_article(p);
            let p2 = article_to_python(a);
            assert!((p - p2).abs() < 1e-12, "roundtrip failed for {p}");
        }
    }

    #[test]
    fn canonical_v_star_pair_matches() {
        let derived_article = python_to_article(V_STAR_ACTIVE_PYTHON);
        assert!(
            (derived_article - V_STAR_ACTIVE_ARTICLE).abs() < 1e-5,
            "Python {V_STAR_ACTIVE_PYTHON} → article {derived_article}; expected {V_STAR_ACTIVE_ARTICLE}"
        );
    }

    #[test]
    fn passive_ze_matches_one_minus_ln2() {
        assert!((V_STAR_PASSIVE_PYTHON - (1.0 - 2.0_f64.ln())).abs() < 1e-12);
    }

    #[test]
    fn f_opt_reasonable_at_128hz() {
        let f = f_opt_hz(128.0);
        // BioSense PARAMETERS.md says f_opt @ 128 Hz ≈ 29.2 Hz.
        assert!((f - 29.2).abs() < 0.1, "got {f}");
    }
}
