// Ontogenesis v4.1 — src/data/normalization.rs
// Normalize records to unified age grid: 0–300 months, step = 1 month

use crate::data::ingestion::DataRecord;
use crate::params::OntogenesisParams;
use std::collections::HashMap;

/// Age grid: months 0..=300 with step 1
pub struct AgeGrid {
    pub step: u32,
    pub min: u32,
    pub max: u32,
}

impl AgeGrid {
    pub fn from_params(p: &OntogenesisParams) -> Self {
        Self {
            step: p.age_step,
            min: p.age_min_months,
            max: p.age_max_months,
        }
    }

    /// Assign each record to a grid bucket (floor to nearest step)
    pub fn assign_bucket(&self, age_months: f64) -> Option<u32> {
        let bucket = (age_months as u32 / self.step) * self.step;
        if bucket >= self.min && bucket <= self.max {
            Some(bucket)
        } else {
            None
        }
    }
}

/// Records grouped by (parameter, age_bucket_months)
pub type GridData = HashMap<String, HashMap<u32, Vec<f64>>>;

/// Group records onto the age grid per parameter
pub fn bin_records(records: &[DataRecord], grid: &AgeGrid) -> GridData {
    let mut out: GridData = HashMap::new();
    for r in records {
        if let Some(bucket) = grid.assign_bucket(r.age_months) {
            out.entry(r.parameter.clone())
                .or_default()
                .entry(bucket)
                .or_default()
                .push(r.value);
        }
    }
    out
}

/// Compute mean and SD for each (parameter, age_bucket)
pub fn compute_stats(grid_data: &GridData) -> HashMap<String, HashMap<u32, (f64, f64, usize)>> {
    let mut out = HashMap::new();
    for (param, age_map) in grid_data {
        let mut age_stats: HashMap<u32, (f64, f64, usize)> = HashMap::new();
        for (age, values) in age_map {
            if values.is_empty() {
                continue;
            }
            let n = values.len();
            let mean = values.iter().sum::<f64>() / n as f64;
            let variance = values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / n as f64;
            let sd = variance.sqrt();
            age_stats.insert(*age, (mean, sd, n));
        }
        out.insert(param.clone(), age_stats);
    }
    out
}

/// Coefficient of variation at each age bucket
pub fn compute_cv(stats: &HashMap<u32, (f64, f64, usize)>) -> HashMap<u32, f64> {
    stats
        .iter()
        .filter_map(|(age, (mean, sd, _n))| {
            if *mean == 0.0 {
                None
            } else {
                Some((*age, sd / mean))
            }
        })
        .collect()
}

/// 90th–10th percentile range per age bucket
pub fn compute_range_90_10(grid_data: &HashMap<u32, Vec<f64>>) -> HashMap<u32, f64> {
    grid_data
        .iter()
        .filter_map(|(age, values)| {
            let mut sorted = values.clone();
            sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let n = sorted.len();
            if n < 10 {
                return None;
            }
            let p10_idx = (n as f64 * 0.10).round() as usize;
            let p90_idx = (n as f64 * 0.90).round() as usize;
            let p10 = sorted[p10_idx.min(n - 1)];
            let p90 = sorted[p90_idx.min(n - 1)];
            Some((*age, p90 - p10))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bucket_assignment() {
        let params = OntogenesisParams::default();
        let grid = AgeGrid::from_params(&params);
        assert_eq!(grid.assign_bucket(0.0), Some(0));
        assert_eq!(grid.assign_bucket(12.5), Some(12));
        assert_eq!(grid.assign_bucket(300.0), Some(300));
        assert_eq!(grid.assign_bucket(301.0), None);
    }

    #[test]
    fn test_cv_calculation() {
        let mut stats = HashMap::new();
        stats.insert(12u32, (100.0_f64, 10.0_f64, 30usize));
        let cv = compute_cv(&stats);
        assert!((cv[&12] - 0.10).abs() < 1e-9);
    }
}
