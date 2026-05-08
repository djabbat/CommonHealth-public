//! `Parameter` — a single named numerical parameter as used in PARAMETERS.md
//! tables across LC subprojects.  Lets each subproject machine-read every
//! other subproject's parameters with the same struct.
//!
//! YAML example (matches existing PARAMETERS.md tables once they migrate):
//!
//! ```yaml
//! id: alpha_HSC
//! symbol: "α_HSC"
//! description: Damage gain per HSC division.
//! unit: damage_per_division
//! value: 0.0082
//! ci_low: 0.006
//! ci_high: 0.011
//! source: "Round-7 MCMC posterior; calibration.rs"
//! status: fitted
//! sensitivity_index_s1: 0.224
//! subproject: CDATA
//! ```

use crate::units::Unit;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ParameterStatus {
    /// Value zafiks. apriori, не двигается при фиттинге.
    FixedCanonical,
    /// Установлен независимым экспериментом.
    FixedMeasured,
    /// Оценено из литературы (composite).
    LiteratureEstimate,
    /// Получено фиттингом (MCMC, MLE, ...).
    Fitted,
    /// Требует калибровки на данных.
    ToBeCalibrated,
    /// Временное значение для симуляций.
    Placeholder,
    /// Канонический null hypothesis (например γ_i = 0).
    DefaultNull,
    /// Объявлен deprecated (есть в коде, но не используется).
    Deprecated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    /// Stable identifier (snake_case).
    pub id: String,
    /// Mathematical symbol (UTF-8).
    pub symbol: Option<String>,
    pub description: String,
    pub unit: Unit,
    /// Free-form unit suffix when `unit == Other`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_tag: Option<String>,
    pub value: f64,
    /// 95 % CI lower bound (inclusive) if known.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ci_low: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ci_high: Option<f64>,
    /// Reference: PMID, DOI, internal file path, calibration run, etc.
    pub source: String,
    pub status: ParameterStatus,
    /// Sobol first-order sensitivity index (if computed).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sensitivity_index_s1: Option<f64>,
    /// Subproject owning this parameter (CDATA, BioSense, ...).
    pub subproject: String,
    /// Module within the subproject (counter name, EEG processing, ...).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ParameterRegistry {
    /// Indexed by composite key `<subproject>::<id>`.
    pub params: HashMap<String, Parameter>,
}

impl ParameterRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn key_for(subproject: &str, id: &str) -> String {
        format!("{subproject}::{id}")
    }

    pub fn insert(&mut self, p: Parameter) -> Option<Parameter> {
        let k = Self::key_for(&p.subproject, &p.id);
        self.params.insert(k, p)
    }

    pub fn get(&self, subproject: &str, id: &str) -> Option<&Parameter> {
        self.params.get(&Self::key_for(subproject, id))
    }

    /// Return parameters that share the same `id` across multiple subprojects
    /// — useful for spotting cross-project drift (audit P0 #6).
    pub fn cross_project_drift(&self, id: &str) -> Vec<&Parameter> {
        self.params
            .values()
            .filter(|p| p.id == id)
            .collect()
    }

    /// Validate a registry: returns the list of `id`s that have inconsistent
    /// values across subprojects (different `value` for same `id` and `unit`).
    /// Different `unit_tag`s are considered intentional (different tissues, etc.).
    pub fn report_inconsistencies(&self) -> Vec<InconsistencyReport> {
        use std::collections::BTreeMap;
        let mut by_id: BTreeMap<&str, Vec<&Parameter>> = BTreeMap::new();
        for p in self.params.values() {
            by_id.entry(p.id.as_str()).or_default().push(p);
        }
        let mut out = Vec::new();
        for (id, group) in by_id {
            if group.len() < 2 {
                continue;
            }
            // Group by unit
            let mut by_unit: BTreeMap<&str, Vec<&Parameter>> = BTreeMap::new();
            for p in &group {
                by_unit.entry(p.unit.as_str()).or_default().push(p);
            }
            for (unit, gs) in by_unit {
                if gs.len() < 2 {
                    continue;
                }
                let v0 = gs[0].value;
                let mismatch = gs.iter().any(|p| (p.value - v0).abs() > 1e-9);
                if mismatch {
                    out.push(InconsistencyReport {
                        param_id: id.to_string(),
                        unit: unit.to_string(),
                        across: gs
                            .iter()
                            .map(|p| (p.subproject.clone(), p.value))
                            .collect(),
                    });
                }
            }
        }
        out
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InconsistencyReport {
    pub param_id: String,
    pub unit: String,
    /// (subproject, value) pairs that disagree.
    pub across: Vec<(String, f64)>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(sub: &str, id: &str, val: f64, unit: Unit) -> Parameter {
        Parameter {
            id: id.into(),
            symbol: None,
            description: "test".into(),
            unit,
            unit_tag: None,
            value: val,
            ci_low: None,
            ci_high: None,
            source: "test".into(),
            status: ParameterStatus::Placeholder,
            sensitivity_index_s1: None,
            subproject: sub.into(),
            module: None,
        }
    }

    #[test]
    fn registry_inserts_and_finds() {
        let mut r = ParameterRegistry::new();
        r.insert(p("CDATA", "alpha_HSC", 0.0082, Unit::DamagePerDivision));
        r.insert(p("Telomere", "alpha2", 100.0, Unit::Bp));
        assert!(r.get("CDATA", "alpha_HSC").is_some());
        assert!(r.get("Telomere", "alpha2").is_some());
        assert!(r.get("CDATA", "nope").is_none());
    }

    #[test]
    fn detects_value_drift_for_same_id_and_unit() {
        let mut r = ParameterRegistry::new();
        r.insert(p("CDATA", "alpha", 0.05, Unit::DamagePerDivision));
        r.insert(p("Telomere", "alpha", 0.10, Unit::DamagePerDivision));
        let drift = r.report_inconsistencies();
        assert_eq!(drift.len(), 1);
        assert_eq!(drift[0].param_id, "alpha");
    }

    #[test]
    fn ignores_drift_when_units_differ() {
        let mut r = ParameterRegistry::new();
        r.insert(p("CDATA", "alpha", 0.05, Unit::DamagePerDivision));
        r.insert(p("Telomere", "alpha", 100.0, Unit::Bp));
        let drift = r.report_inconsistencies();
        assert!(drift.is_empty());
    }

    #[test]
    fn json_roundtrip() {
        let mut r = ParameterRegistry::new();
        r.insert(p("CDATA", "alpha_HSC", 0.0082, Unit::DamagePerDivision));
        let s = serde_json::to_string_pretty(&r).unwrap();
        let r2: ParameterRegistry = serde_json::from_str(&s).unwrap();
        assert_eq!(
            r.get("CDATA", "alpha_HSC").unwrap().value,
            r2.get("CDATA", "alpha_HSC").unwrap().value
        );
    }
}
