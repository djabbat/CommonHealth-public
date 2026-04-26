//! CLI binary: run a single-counter trajectory for a named tissue.

use std::env;
use proteostasis_counter::trajectory::{run_trajectory, TrajectoryRequest};
use proteostasis_counter::tissue::Tissue;

fn parse_args() -> (Tissue, f64, f64) {
    let mut tissue = Tissue::HSC;
    let mut days: f64 = 3650.0;
    let mut rate: f64 = 0.01;
    let args: Vec<String> = env::args().collect();
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--tissue" => {
                tissue = match args[i+1].as_str() {
                    "HSC" => Tissue::HSC,
                    "Fibroblast" => Tissue::Fibroblast,
                    "Neuron" => Tissue::Neuron,
                    "Cardiomyocyte" => Tissue::Cardiomyocyte,
                    "Hepatocyte" => Tissue::Hepatocyte,
                    "IntestinalCrypt" => Tissue::IntestinalCrypt,
                    other => { eprintln!("Unknown tissue: {}", other); std::process::exit(2); }
                };
                i += 2;
            },
            "--days" => { days = args[i+1].parse().expect("--days f64"); i += 2; },
            "--rate" => { rate = args[i+1].parse().expect("--rate f64"); i += 2; },
            flag => { eprintln!("Unknown flag: {}", flag); std::process::exit(2); }
        }
    }
    (tissue, days, rate)
}

fn main() {
    let (tissue, days, rate) = parse_args();
    let req = TrajectoryRequest {
        tissue,
        division_rate_per_day: rate,
        coupling_source: None,
        horizon_days: days,
        params_override: None,
    };
    let traj = run_trajectory(&req);
    println!("t_days,n,d,tissue,counter");
    for p in traj {
        println!("{},{},{:.8},{:?},5", p.t_days, p.n, p.d, tissue);
    }
}
