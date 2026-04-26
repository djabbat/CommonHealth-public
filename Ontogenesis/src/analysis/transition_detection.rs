// Ontogenesis v4.1 — src/analysis/transition_detection.rs
// TransitionDetection: CONCEPT §2.2–2.3 algorithm
//
// Longitudinal: change > N×SD from individual trajectory
// Cross-sectional: CV or Range_90_10 > mean + 2×SD
// Clustering: radius = 6 months, min stable period = 3 months

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::data::ingestion::{DataRecord, DataType};
use crate::data::normalization::{AgeGrid, GridData, bin_records, compute_stats};
use crate::analysis::cv_analysis::{detect_cv_peaks, detect_range_peaks, cluster_marks};
use crate::params::OntogenesisParams;

/// Type of developmental transition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TransitionType {
    /// Anatomical (structural change)
    Anatomical,
    /// Endocrine (hormonal shift)
    Endocrine,
    /// Combined (both signals coincide)
    Combined,
}

/// A detected developmental transition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transition {
    /// Age at transition (months)
    pub age_months: u32,
    /// Type of transition
    pub transition_type: TransitionType,
    /// Parameters that triggered this transition
    pub parameters: Vec<String>,
    /// Approximate age in years/months (human-readable)
    pub age_label: String,
}

impl Transition {
    fn age_label(months: u32) -> String {
        let years = months / 12;
        let rem = months % 12;
        if rem == 0 {
            format!("{years}y")
        } else {
            format!("{years}y {rem}m")
        }
    }
}

/// Main transition detection engine
pub struct TransitionDetector {
    params: OntogenesisParams,
}

impl TransitionDetector {
    pub fn new(params: OntogenesisParams) -> Self {
        Self { params }
    }

    pub fn with_defaults() -> Self {
        Self::new(OntogenesisParams::default())
    }

    /// Run full detection on a set of records.
    /// Returns clustered transitions sorted by age.
    pub fn detect(&self, records: &[DataRecord]) -> Vec<Transition> {
        let grid = AgeGrid::from_params(&self.params);

        let anat_records: Vec<&DataRecord> = records
            .iter()
            .filter(|r| r.data_type == DataType::Anatomical)
            .collect();
        let endo_records: Vec<&DataRecord> = records
            .iter()
            .filter(|r| r.data_type == DataType::Endocrine)
            .collect();

        // Cross-sectional: CV peaks (anatomical)
        let anat_marks = self.detect_cross_sectional_anat(&anat_records, &grid);

        // Cross-sectional: Range peaks (endocrine)
        let endo_marks = self.detect_cross_sectional_endo(&endo_records, &grid);

        // Longitudinal: SD threshold (all data with individual_id)
        let long_marks = self.detect_longitudinal(records);

        // Combine and classify
        self.combine_and_classify(anat_marks, endo_marks, long_marks)
    }

    fn detect_cross_sectional_anat(
        &self,
        records: &[&DataRecord],
        grid: &AgeGrid,
    ) -> HashMap<String, Vec<u32>> {
        let owned: Vec<DataRecord> = records.iter().map(|r| (*r).clone()).collect();
        let grid_data: GridData = bin_records(&owned, grid);

        let mut param_marks: HashMap<String, Vec<u32>> = HashMap::new();
        for (param, age_values) in &grid_data {
            // Only use buckets with enough samples
            let filtered: HashMap<u32, Vec<f64>> = age_values
                .iter()
                .filter(|(_, v)| v.len() >= self.params.min_sample_size)
                .map(|(k, v)| (*k, v.clone()))
                .collect();

            if filtered.len() < 3 {
                continue;
            }

            let stats = compute_stats(&HashMap::from([(param.clone(), filtered.clone())]));
            let param_stats = match stats.get(param) {
                Some(s) => s,
                None => continue,
            };

            let peaks = detect_cv_peaks(param_stats, &self.params);
            if !peaks.is_empty() {
                param_marks.insert(param.clone(), peaks);
            }
        }
        param_marks
    }

    fn detect_cross_sectional_endo(
        &self,
        records: &[&DataRecord],
        grid: &AgeGrid,
    ) -> HashMap<String, Vec<u32>> {
        let owned: Vec<DataRecord> = records.iter().map(|r| (*r).clone()).collect();
        let grid_data: GridData = bin_records(&owned, grid);

        let mut param_marks: HashMap<String, Vec<u32>> = HashMap::new();
        for (param, age_values) in &grid_data {
            let filtered: HashMap<u32, Vec<f64>> = age_values
                .iter()
                .filter(|(_, v)| v.len() >= self.params.min_sample_size)
                .map(|(k, v)| (*k, v.clone()))
                .collect();

            if filtered.len() < 3 {
                continue;
            }

            let peaks = detect_range_peaks(&filtered, &self.params);
            if !peaks.is_empty() {
                param_marks.insert(param.clone(), peaks);
            }
        }
        param_marks
    }

    fn detect_longitudinal(&self, records: &[DataRecord]) -> Vec<u32> {
        // Group by (individual_id, parameter) → time series
        let mut series: HashMap<(String, String), Vec<(f64, f64)>> = HashMap::new();
        for r in records {
            if let Some(id) = &r.individual_id {
                series
                    .entry((id.clone(), r.parameter.clone()))
                    .or_default()
                    .push((r.age_months, r.value));
            }
        }

        let mut marks: Vec<u32> = Vec::new();

        for ((_, _param), mut points) in series {
            if points.len() < 4 {
                continue;
            }
            points.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

            // Compute per-individual SD of differences
            let diffs: Vec<f64> = points.windows(2).map(|w| w[1].1 - w[0].1).collect();
            let mean_diff = diffs.iter().sum::<f64>() / diffs.len() as f64;
            let sd_diff = {
                let var = diffs.iter().map(|d| (d - mean_diff).powi(2)).sum::<f64>()
                    / diffs.len() as f64;
                var.sqrt()
            };

            let threshold_sd = match () {
                _ => self.params.anat_threshold_sd, // use anatomical by default
            };

            for (i, &diff) in diffs.iter().enumerate() {
                if (diff - mean_diff).abs() > threshold_sd * sd_diff {
                    // Transition at midpoint between points[i] and points[i+1]
                    let mid_age = (points[i].0 + points[i + 1].0) / 2.0;
                    if mid_age >= self.params.age_min_months as f64
                        && mid_age <= self.params.age_max_months as f64
                    {
                        marks.push(mid_age as u32);
                    }
                }
            }
        }

        marks.sort();
        marks.dedup();
        marks
    }

    fn combine_and_classify(
        &self,
        anat_marks: HashMap<String, Vec<u32>>,
        endo_marks: HashMap<String, Vec<u32>>,
        long_marks: Vec<u32>,
    ) -> Vec<Transition> {
        // Flatten all marks with their type
        let mut all_anat: Vec<u32> = anat_marks.values().flatten().copied().collect();
        let all_endo: Vec<u32> = endo_marks.values().flatten().copied().collect();
        all_anat.extend_from_slice(&long_marks);

        // Cluster each type
        let clustered_anat = cluster_marks(&all_anat, self.params.cluster_radius);
        let clustered_endo = cluster_marks(&all_endo, self.params.cluster_radius);

        let mut transitions: Vec<Transition> = Vec::new();

        // Find combined transitions (within cluster_radius of each other)
        let mut endo_used = vec![false; clustered_endo.len()];
        for &a_age in &clustered_anat {
            let combined_with = clustered_endo
                .iter()
                .enumerate()
                .find(|(i, &e_age)| {
                    !endo_used[*i] && a_age.abs_diff(e_age) <= self.params.cluster_radius
                });

            if let Some((i, _)) = combined_with {
                endo_used[i] = true;
                let params_involved: Vec<String> = anat_marks
                    .iter()
                    .filter(|(_, v)| v.iter().any(|&m| a_age.abs_diff(m) <= self.params.cluster_radius))
                    .map(|(k, _)| k.clone())
                    .collect();
                transitions.push(Transition {
                    age_months: a_age,
                    transition_type: TransitionType::Combined,
                    parameters: params_involved,
                    age_label: Transition::age_label(a_age),
                });
            } else {
                let params_involved: Vec<String> = anat_marks
                    .iter()
                    .filter(|(_, v)| v.iter().any(|&m| a_age.abs_diff(m) <= self.params.cluster_radius))
                    .map(|(k, _)| k.clone())
                    .collect();
                transitions.push(Transition {
                    age_months: a_age,
                    transition_type: TransitionType::Anatomical,
                    parameters: params_involved,
                    age_label: Transition::age_label(a_age),
                });
            }
        }

        // Remaining endocrine-only
        for (i, &e_age) in clustered_endo.iter().enumerate() {
            if !endo_used[i] {
                let params_involved: Vec<String> = endo_marks
                    .iter()
                    .filter(|(_, v)| v.iter().any(|&m| e_age.abs_diff(m) <= self.params.cluster_radius))
                    .map(|(k, _)| k.clone())
                    .collect();
                transitions.push(Transition {
                    age_months: e_age,
                    transition_type: TransitionType::Endocrine,
                    parameters: params_involved,
                    age_label: Transition::age_label(e_age),
                });
            }
        }

        transitions.sort_by_key(|t| t.age_months);
        transitions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_age_label() {
        assert_eq!(Transition::age_label(24), "2y");
        assert_eq!(Transition::age_label(14), "1y 2m");
    }

    #[test]
    fn test_empty_records() {
        let detector = TransitionDetector::with_defaults();
        let transitions = detector.detect(&[]);
        assert!(transitions.is_empty());
    }
}
