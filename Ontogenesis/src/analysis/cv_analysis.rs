// Ontogenesis v4.1 — src/analysis/cv_analysis.rs
// CVAnalysis + RangeAnalysis: detect high-variability age windows
// These are the cross-sectional transition markers

use std::collections::HashMap;
use crate::data::normalization::{compute_cv, compute_range_90_10};
use crate::params::OntogenesisParams;

/// Ages where CV exceeds threshold (anatomical cross-sectional transitions)
pub fn detect_cv_peaks(
    stats: &HashMap<u32, (f64, f64, usize)>,  // age → (mean, sd, n)
    params: &OntogenesisParams,
) -> Vec<u32> {
    let cv_map = compute_cv(stats);
    if cv_map.len() < 3 {
        return vec![];
    }

    let cv_vals: Vec<f64> = cv_map.values().copied().collect();
    let cv_mean = cv_vals.iter().sum::<f64>() / cv_vals.len() as f64;
    let cv_var = cv_vals.iter().map(|v| (v - cv_mean).powi(2)).sum::<f64>() / cv_vals.len() as f64;
    let cv_sd = cv_var.sqrt();

    let threshold = cv_mean + params.anat_cross_sd_mult * cv_sd;

    let mut peaks: Vec<u32> = cv_map
        .iter()
        .filter(|(_, cv)| **cv > threshold)
        .map(|(age, _)| *age)
        .collect();
    peaks.sort();
    peaks
}

/// Ages where Range_90_10 exceeds threshold (endocrine cross-sectional)
pub fn detect_range_peaks(
    grid_data: &HashMap<u32, Vec<f64>>,
    params: &OntogenesisParams,
) -> Vec<u32> {
    let range_map = compute_range_90_10(grid_data);
    if range_map.len() < 3 {
        return vec![];
    }

    let vals: Vec<f64> = range_map.values().copied().collect();
    let mean = vals.iter().sum::<f64>() / vals.len() as f64;
    let var = vals.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / vals.len() as f64;
    let sd = var.sqrt();

    let threshold = mean + params.endo_cross_sd_mult * sd;

    let mut peaks: Vec<u32> = range_map
        .iter()
        .filter(|(_, r)| **r > threshold)
        .map(|(age, _)| *age)
        .collect();
    peaks.sort();
    peaks
}

/// Cluster nearby age marks within `radius` months
pub fn cluster_marks(marks: &[u32], radius: u32) -> Vec<u32> {
    if marks.is_empty() {
        return vec![];
    }
    let mut sorted = marks.to_vec();
    sorted.sort();
    let mut clusters: Vec<Vec<u32>> = vec![vec![sorted[0]]];

    for &age in &sorted[1..] {
        let last_cluster = clusters.last_mut().unwrap();
        if age - *last_cluster.last().unwrap() <= radius {
            last_cluster.push(age);
        } else {
            clusters.push(vec![age]);
        }
    }

    // Representative = median of cluster
    clusters
        .iter()
        .map(|c| {
            let mid = c.len() / 2;
            c[mid]
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cluster_merges_nearby() {
        let marks = vec![12, 14, 15, 36, 38, 100];
        let params = OntogenesisParams::default();
        let clusters = cluster_marks(&marks, params.cluster_radius);
        // 12,14,15 → cluster (within 6m), 36,38 → cluster, 100 → alone
        assert_eq!(clusters.len(), 3);
    }

    #[test]
    fn test_cluster_empty() {
        assert!(cluster_marks(&[], 6).is_empty());
    }
}
