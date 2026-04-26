//! mcoa-compare-all — pairwise comparison across all 5 counters × 6 tissues.
//!
//! Skeleton port of `scripts/compare_all.py`. Full orchestration of 5 simulators
//! still depends on `mcoa_simulation` API; this binary computes pairwise Δ
//! statistics for any pair of CSVs and emits a flat report.
//!
//! Per `feedback_mcoa_cdata_comparison v2`: 5 counter simulators (CDATA,
//! Telomere, MitoROS, EpigeneticDrift, Proteostasis) + MCOA coordinator.
//! Every full run produces all 15 pairwise counter-vs-counter residuals
//! across 6 tissues = 90 cells.

use clap::Parser;
use mcoa_compare::{delta_stats, read_csv};
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "mcoa-compare-all", about = "Pairwise comparison across counter CSVs")]
struct Cli {
    /// Counter CSV files. Provide ≥2; pairwise Δ is computed for every pair.
    #[arg(long, num_args = 2..)]
    csvs: Vec<PathBuf>,
    /// Column name to compare across CSVs.
    #[arg(long, default_value = "tissue_load")]
    column: String,
    /// Output markdown report path.
    #[arg(long, default_value = "../docs/comparisons/all_pairwise.md")]
    out: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let c = Cli::parse();
    let mut series = Vec::new();
    for path in &c.csvs {
        let s = read_csv(path)?;
        let col = s.columns.get(&c.column)
            .ok_or_else(|| anyhow::anyhow!("CSV {} missing column {}", path.display(), c.column))?
            .clone();
        series.push((path.display().to_string(), col));
    }

    let mut report = String::new();
    report.push_str("# Pairwise Δ matrix\n\n");
    report.push_str(&format!("Column compared: `{}`\n\n", c.column));
    report.push_str("| A | B | n | max\\|Δ\\| | mean Δ | std Δ |\n");
    report.push_str("|---|---|---|---|---|---|\n");

    for i in 0..series.len() {
        for j in (i + 1)..series.len() {
            let s = delta_stats(&series[i].1, &series[j].1);
            report.push_str(&format!(
                "| `{}` | `{}` | {} | {:.4} | {:+.4} | {:.4} |\n",
                series[i].0, series[j].0, s.n, s.max_abs, s.mean, s.std
            ));
        }
    }

    if let Some(parent) = c.out.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&c.out, &report)?;
    println!("wrote {}", c.out.display());
    Ok(())
}
