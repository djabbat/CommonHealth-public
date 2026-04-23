//! ze_sim — CLI for Ze Theory simulators.
//!
//! Usage:
//!     ze_sim impedance --scenario <routine|novelty|meditation|cheating> [--horizon N]
//!     ze_sim chsh      [--h 0.5] [--alpha 0.03] [--delta 0.05]
//!     ze_sim autowaves [--steps 2000] [--n 200]

use std::env;
use std::fs::File;
use std::io::{self, Write};

use ze_simulator::{autowaves, chsh, impedance};

fn arg_str(args: &[String], key: &str) -> Option<String> {
    args.iter()
        .position(|a| a == key)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn arg_f64(args: &[String], key: &str) -> Option<f64> {
    arg_str(args, key).and_then(|s| s.parse().ok())
}

fn arg_usize(args: &[String], key: &str) -> Option<usize> {
    arg_str(args, key).and_then(|s| s.parse().ok())
}

fn emit<T: serde::Serialize>(args: &[String], value: &T) -> io::Result<()> {
    let json = serde_json::to_string_pretty(value).expect("serde");
    match arg_str(args, "--output") {
        Some(path) => {
            File::create(&path)?.write_all(json.as_bytes())?;
            eprintln!("wrote {}", path);
        }
        None => println!("{}", json),
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mode = args.get(1).cloned().unwrap_or_else(|| "impedance".into());

    match mode.as_str() {
        "impedance" => {
            let mut cfg = impedance::RunConfig::default();
            if let Some(s) = arg_str(&args, "--scenario") {
                cfg.params.scenario = s;
            }
            if let Some(v) = arg_f64(&args, "--horizon") {
                cfg.t_end = v;
            }
            if let Some(v) = arg_f64(&args, "--lambda") {
                cfg.params.lambda = v;
            }
            if cfg.params.scenario == "cheating" {
                cfg.cheating_spike = Some((10.0, 0.2));
            }
            let traj = impedance::simulate(&cfg);
            emit(&args, &traj)?;
        }
        "chsh" => {
            let mut p = chsh::Params::default();
            if let Some(v) = arg_f64(&args, "--h") {
                p.h = v;
            }
            if let Some(v) = arg_f64(&args, "--alpha") {
                p.alpha = v;
            }
            if let Some(v) = arg_f64(&args, "--delta") {
                p.delta0 = v;
            }
            let rep = chsh::run(p);
            emit(&args, &rep)?;
        }
        "autowaves" => {
            let mut p = autowaves::Params::default();
            if let Some(v) = arg_usize(&args, "--n") {
                p.n = v;
            }
            if let Some(v) = arg_f64(&args, "--dt") {
                p.dt = v;
            }
            let steps = arg_usize(&args, "--steps").unwrap_or(2000);
            let snap = arg_usize(&args, "--snapshot-every").unwrap_or(200);
            let run = autowaves::simulate(p, steps, snap);
            emit(&args, &run)?;
        }
        other => {
            eprintln!("unknown mode: {} (use impedance | chsh | autowaves)", other);
            std::process::exit(2);
        }
    }
    Ok(())
}
