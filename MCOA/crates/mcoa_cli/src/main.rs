//! mcoa-sim — run an MCOA simulation and write per-step records to CSV.
//!
//! Per the mandatory comparison rule (see ~/Desktop/LongevityCommon/MCOA/CLAUDE.md), every simulation
//! output MUST be paired with an analogous CDATA run via `scripts/compare_mcoa_cdata.py`.

use clap::Parser;
use mcoa_core::{Gamma, Tissue};
use mcoa_simulation::run;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "mcoa-sim",
    version,
    about = "Run a Multi-Counter Architecture simulation.",
    long_about = "Runs a discrete-time MCOA simulation for a given tissue, writing per-step counter \
                   states and tissue load to CSV. Pair each output with a matching CDATA run (see \
                   scripts/compare_mcoa_cdata.py)."
)]
struct Cli {
    /// Tissue name: fibroblast | hsc | neuron | hepatocyte | beta_cell | cd8_t_memory
    #[arg(long, default_value = "hsc")]
    tissue: String,
    /// Number of division-equivalent steps
    #[arg(long, default_value_t = 100)]
    divisions: usize,
    /// Seconds per division-equivalent (default: 7 days)
    #[arg(long, default_value_t = 604800.0)]
    seconds_per_division: f64,
    /// Output CSV path
    #[arg(long, default_value = "mcoa_run.csv")]
    output: PathBuf,
}

fn parse_tissue(s: &str) -> Result<Tissue, String> {
    match s {
        "fibroblast" => Ok(Tissue::Fibroblast),
        "hsc" => Ok(Tissue::Hsc),
        "neuron" => Ok(Tissue::Neuron),
        "hepatocyte" => Ok(Tissue::Hepatocyte),
        "beta_cell" => Ok(Tissue::BetaCell),
        "cd8_t_memory" => Ok(Tissue::CD8TMemory),
        other => Err(format!("unknown tissue '{other}'")),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let tissue = parse_tissue(&cli.tissue)?;
    let gamma = Gamma::default();
    let records = run(tissue, cli.divisions, cli.seconds_per_division, &gamma);
    let mut writer = csv::Writer::from_path(&cli.output)?;
    for r in &records {
        writer.serialize(r)?;
    }
    writer.flush()?;
    eprintln!(
        "mcoa-sim: wrote {} records for tissue={} to {}",
        records.len(),
        cli.tissue,
        cli.output.display()
    );
    eprintln!("REMINDER: pair this output with CDATA via scripts/compare_mcoa_cdata.py");
    Ok(())
}
