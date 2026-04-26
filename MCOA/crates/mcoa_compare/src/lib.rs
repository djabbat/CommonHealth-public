//! mcoa_compare — MCOA vs CDATA comparison and full pairwise comparison harness.
//!
//! Rust port of `scripts/compare_mcoa_cdata.py` and `scripts/compare_all.py`.
//! Plot generation is OUT OF SCOPE for the Rust port — comparison reports are
//! pure markdown + numeric statistics. Plots can be added later via plotters
//! crate or external pipeline.

use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct CsvSeries {
    pub headers: Vec<String>,
    pub columns: HashMap<String, Vec<f64>>,
    pub n_rows: usize,
}

pub fn read_csv(path: &Path) -> Result<CsvSeries> {
    let mut rdr = csv::Reader::from_path(path)?;
    let headers: Vec<String> = rdr.headers()?.iter().map(|s| s.to_string()).collect();
    let mut columns: HashMap<String, Vec<f64>> = HashMap::new();
    for h in &headers {
        columns.insert(h.clone(), Vec::new());
    }
    let mut n_rows = 0;
    for record in rdr.records() {
        let record = record?;
        for (i, field) in record.iter().enumerate() {
            let parsed: f64 = field.parse().unwrap_or(f64::NAN);
            columns.get_mut(&headers[i]).unwrap().push(parsed);
        }
        n_rows += 1;
    }
    Ok(CsvSeries { headers, columns, n_rows })
}

#[derive(Debug, Clone)]
pub struct DeltaStats {
    pub n: usize,
    pub max_abs: f64,
    pub mean: f64,
    pub std: f64,
}

pub fn delta_stats(a: &[f64], b: &[f64]) -> DeltaStats {
    let n = a.len().min(b.len());
    if n == 0 {
        return DeltaStats { n: 0, max_abs: 0.0, mean: 0.0, std: 0.0 };
    }
    let deltas: Vec<f64> = (0..n).map(|i| a[i] - b[i]).collect();
    let mean: f64 = deltas.iter().sum::<f64>() / n as f64;
    let max_abs: f64 = deltas.iter().fold(0.0_f64, |m, &d| m.max(d.abs()));
    let std = if n > 1 {
        let var = deltas.iter().map(|d| (d - mean).powi(2)).sum::<f64>() / (n - 1) as f64;
        var.sqrt()
    } else {
        0.0
    };
    DeltaStats { n, max_abs, mean, std }
}

pub struct CompareArgs<'a> {
    pub mcoa_csv: &'a Path,
    pub cdata_csv: &'a Path,
    pub tissue: &'a str,
    pub label: &'a str,
    pub out_dir: &'a Path,
}

/// MCOA-vs-CDATA comparison. Returns path to written markdown report.
pub fn compare_mcoa_cdata(args: CompareArgs) -> Result<PathBuf> {
    let mcoa = read_csv(args.mcoa_csv)?;
    let cdata = read_csv(args.cdata_csv)?;

    let x_col = if mcoa.columns.contains_key("n_cumulative") {
        "n_cumulative"
    } else if mcoa.headers.len() > 1 {
        &mcoa.headers[1]
    } else {
        return Err(anyhow!("MCOA CSV has no usable x-axis column"));
    };

    let cdata_col = if cdata.columns.contains_key("damage") {
        "damage".to_string()
    } else {
        cdata.headers.iter().find(|c| c.as_str() != x_col)
            .cloned()
            .ok_or_else(|| anyhow!("CDATA CSV has no usable damage column"))?
    };

    let mcoa_col = if mcoa.columns.contains_key("centriolar") {
        "centriolar"
    } else {
        "tissue_load"
    };

    let mcoa_data = mcoa.columns.get(mcoa_col)
        .ok_or_else(|| anyhow!("MCOA missing column {}", mcoa_col))?;
    let cdata_data = cdata.columns.get(&cdata_col)
        .ok_or_else(|| anyhow!("CDATA missing column {}", cdata_col))?;

    let stats = delta_stats(mcoa_data, cdata_data);

    fs::create_dir_all(args.out_dir)?;
    let date = chrono::Local::now().date_naive().format("%Y-%m-%d").to_string();
    let report_path = args.out_dir.join(format!("{}_{}.md", date, args.label));

    let mut s = String::new();
    s.push_str(&format!("# MCOA vs CDATA comparison — {}\n\n", args.label));
    s.push_str(&format!("**Date:** {}\n", date));
    s.push_str(&format!("**Tissue:** {}\n", args.tissue));
    s.push_str(&format!("**MCOA input:** `{}`\n", args.mcoa_csv.display()));
    s.push_str(&format!("**CDATA input:** `{}`\n", args.cdata_csv.display()));
    s.push_str(&format!("**MCOA column compared:** `{}`\n", mcoa_col));
    s.push_str(&format!("**CDATA column compared:** `{}`\n\n", cdata_col));
    s.push_str("## Summary\n\n");
    s.push_str(&format!("- Samples compared: {}\n", stats.n));
    s.push_str(&format!("- max |Δ| = {:.4}\n", stats.max_abs));
    s.push_str(&format!("- mean Δ = {:+.4}\n", stats.mean));
    s.push_str(&format!("- std Δ = {:.4}\n\n", stats.std));
    s.push_str("## Interpretation — to be filled in by author\n\n");
    s.push_str("Classify the divergence:\n\n");
    s.push_str("- [ ] (a) missing counter in CDATA's single-counter view\n");
    s.push_str("- [ ] (b) artefact of MCOA dimensionless normalisation\n");
    s.push_str("- [ ] (c) real biological signal\n");
    s.push_str("- [ ] (d) bug in one of the simulators\n\n");
    s.push_str("Write at least three sentences of interpretation. Cite PARAMETERS.md entries that ");
    s.push_str("changed, if any. When real experimental data arrive, this report must be re-done ");
    s.push_str("with far deeper analysis.\n");

    fs::write(&report_path, s)?;
    println!("wrote {}", report_path.display());
    Ok(report_path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn delta_stats_basic() {
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![1.0, 2.5, 4.0];
        let s = delta_stats(&a, &b);
        assert_eq!(s.n, 3);
        // deltas: 0.0, -0.5, -1.0; mean = -0.5; max|Δ| = 1.0
        assert!((s.mean + 0.5).abs() < 1e-12);
        assert!((s.max_abs - 1.0).abs() < 1e-12);
    }

    #[test]
    fn delta_stats_empty() {
        let s = delta_stats(&[], &[]);
        assert_eq!(s.n, 0);
        assert_eq!(s.mean, 0.0);
        assert_eq!(s.max_abs, 0.0);
    }

    #[test]
    fn delta_stats_unequal_length() {
        let a = vec![1.0, 2.0, 3.0, 4.0];
        let b = vec![1.0, 2.0];
        let s = delta_stats(&a, &b);
        assert_eq!(s.n, 2);
        assert!(s.mean.abs() < 1e-12);
    }
}
