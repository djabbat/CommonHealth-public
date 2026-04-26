//! Test 4 — α/β decomposition (Aubrey's test) stub.
//!
//! A 2×2 design (high/low division × normal/accelerated time) on iPSC-derived organoids. MCOA
//! predicts α_i ≠ 0 AND β_i ≠ 0 for division-linked counters; falsified if β_i dominates for a
//! counter claimed to be division-linked.

pub struct Condition {
    pub divisions_per_week: f64,
    pub time_stress_multiplier: f64,
}

pub const HIGH_DIV_NORMAL_TIME: Condition = Condition { divisions_per_week: 1.0, time_stress_multiplier: 1.0 };
pub const LOW_DIV_NORMAL_TIME:  Condition = Condition { divisions_per_week: 0.05, time_stress_multiplier: 1.0 };
pub const HIGH_DIV_ACC_TIME:    Condition = Condition { divisions_per_week: 1.0, time_stress_multiplier: 3.0 };
pub const LOW_DIV_ACC_TIME:     Condition = Condition { divisions_per_week: 0.05, time_stress_multiplier: 3.0 };

// TODO v0.2: synthetic data generator, OLS / mixed-effects fit, effect-size estimation.
