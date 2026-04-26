//! MCOA falsifiability test harnesses.
//!
//! Implements simulated versions of §6.1–6.5 of the Nature Aging Perspective. Each harness
//! generates synthetic data under the MCOA prior and under a competing null (e.g. single-counter),
//! so that a statistical pipeline can be calibrated against expected effect sizes before real
//! data are available.

/// Test 4 (Aubrey's test) — α vs β decomposition via a 2×2 organoid design.
pub mod test4_aubrey;

/// Test 1 — tissue-specific counter dominance.
pub mod test1_dominance;
