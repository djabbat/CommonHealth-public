//! BioSense CLI — read RR intervals from CSV, output HRV JSON.
//!
//! Usage:
//!     biosense hrv --input rr.csv [--output out.json]
//!
//! CSV format: one float per line — RR interval in milliseconds.

use biosense_core::RrSeries;
use biosense_hrv::{autonomic_switching, freq_domain_simple, time_domain};
use clap::{Parser, Subcommand};
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "biosense", version, about = "BioSense HRV CLI (Rust port of ze_hrv.py)")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Compute HRV metrics from RR-intervals CSV.
    Hrv {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        output: Option<PathBuf>,
    },
}

fn read_rr_csv(path: &PathBuf) -> anyhow::Result<RrSeries> {
    let content = fs::read_to_string(path)?;
    let rr_ms: Vec<f64> = content
        .lines()
        .filter_map(|line| line.trim().parse().ok())
        .collect();
    Ok(RrSeries {
        rr_ms,
        source: path.display().to_string(),
    })
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Hrv { input, output } => {
            let rr = read_rr_csv(&input)?;
            let td = time_domain(&rr)?;
            let fd = freq_domain_simple(&rr).ok();
            let auto = autonomic_switching(&rr, 1.0).ok();

            let payload = serde_json::json!({
                "input": input.display().to_string(),
                "time_domain": td,
                "freq_domain": fd,
                "autonomic_switching": auto,
            });

            let json = serde_json::to_string_pretty(&payload)?;
            match output {
                Some(p) => {
                    fs::write(&p, &json)?;
                    eprintln!("wrote {}", p.display());
                }
                None => println!("{}", json),
            }
        }
    }
    Ok(())
}
