//! Tissue-specific parameter panel for MCOA Counter #2.
//!
//! These are the **a priori** tissue weights (MCOA Axiom M3).  They must
//! be fixed before any fitting procedure.  Any post-hoc adjustment is a
//! model correction, not a model prediction.

use crate::CounterParams;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Tissue {
    HSC,
    Fibroblast,
    Neuron,
    Cardiomyocyte,
    Hepatocyte,
    IntestinalCrypt,
}

impl Tissue {
    pub fn all() -> &'static [Tissue] {
        &[
            Tissue::HSC,
            Tissue::Fibroblast,
            Tissue::Neuron,
            Tissue::Cardiomyocyte,
            Tissue::Hepatocyte,
            Tissue::IntestinalCrypt,
        ]
    }

    /// a-priori weight w_2(tissue) for this counter.
    /// Sum across the 5 MCOA counters for any one tissue should be ~1.0.
    pub fn weight(self) -> f64 {
        match self {
            Tissue::HSC             => 0.35,
            Tissue::Fibroblast      => 0.5,
            Tissue::Neuron          => 0.05,
            Tissue::Cardiomyocyte   => 0.05,
            Tissue::Hepatocyte      => 0.1,
            Tissue::IntestinalCrypt => 0.4,
        }
    }

    /// Tissue-specific default parameter set (starting points for calibration).
    pub fn params(self) -> CounterParams {
        let mut p = CounterParams::default();
        // Simple heuristic: post-mitotic tissues get α → 0, raise β.
        match self {
            Tissue::Neuron | Tissue::Cardiomyocyte => {
                p.alpha *= 0.05;
                p.beta  *= 1.5;
            },
            Tissue::IntestinalCrypt => {
                p.alpha *= 1.5;
                p.beta  *= 0.8;
            },
            Tissue::HSC => {
                // Slight α boost vs default
                p.alpha *= 1.2;
            },
            _ => {}
        }
        p
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_tissues_produce_valid_params() {
        for t in Tissue::all() {
            t.params().validate().expect("tissue param must validate");
        }
    }

    #[test]
    fn weight_is_in_zero_one() {
        for t in Tissue::all() {
            let w = t.weight();
            assert!((0.0..=1.0).contains(&w), "weight out of [0,1]: {}", w);
        }
    }
}
