//! mcoa-compare-cdata — MANDATORY MCOA vs CDATA comparison harness.
//!
//! Rust port of `scripts/compare_mcoa_cdata.py`.
//!
//! Per project rule (`feedback_mcoa_cdata_comparison`): every MCOA simulation run
//! MUST be paired with an analogous CDATA run and a Δ report filed to
//! `docs/comparisons/YYYY-MM-DD_label.md`.

use clap::Parser;
use mcoa_compare::{compare_mcoa_cdata, CompareArgs};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "mcoa-compare-cdata", about = "MCOA vs CDATA comparison harness")]
struct Cli {
    #[arg(long)]
    mcoa_csv: PathBuf,
    #[arg(long)]
    cdata_csv: PathBuf,
    #[arg(long)]
    tissue: String,
    #[arg(long)]
    label: String,
    #[arg(long, default_value = "../docs/comparisons/")]
    out_dir: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let c = Cli::parse();
    let args = CompareArgs {
        mcoa_csv: &c.mcoa_csv,
        cdata_csv: &c.cdata_csv,
        tissue: &c.tissue,
        label: &c.label,
        out_dir: &c.out_dir,
    };
    compare_mcoa_cdata(args)?;
    Ok(())
}
