//! Pure-logic helpers for aim-medkb, exposed as a library so they're testable
//! without spinning up the HTTP server.

use std::collections::HashMap;

pub fn canonicalise(name: &str, synonyms: &HashMap<String, String>) -> String {
    let s = name.trim().to_lowercase().replace('-', " ");
    let s: String = s.split_whitespace().collect::<Vec<_>>().join(" ");
    if let Some(v) = synonyms.get(&s) { return v.clone(); }
    s.replace(' ', "_")
}

pub fn severity_rank(sev: &str) -> u8 {
    match sev {
        "contraindicated" => 0,
        "major" => 1,
        "moderate" => 2,
        "minor" => 3,
        _ => 4,
    }
}

pub fn rank_label(r: u8) -> &'static str {
    match r {
        0 => "contraindicated",
        1 => "major",
        2 => "moderate",
        3 => "minor",
        _ => "no_known",
    }
}
