//! aim-ai-dashboard — DB1.
//!
//! One-button consolidated view of AIM/AI subproject state. Each
//! section is built best-effort — if one fails (missing data, missing
//! crate not yet ported), the dashboard still emits with the section
//! body marked unavailable.
//!
//! Rust port of `AI/ai/dashboard.py`. Sections currently wired:
//!  - score (aim-ai-health)
//!  - ledger (aim-ai-ledger)
//!  - regression (aim-ai-regression)
//!  - prompt drift (aim-ai-prompt-versions)
//!  - cases (aim-ai-cases)
//!
//! Sections still TODO (Python predecessors not yet ported):
//!  - safety_gate, suppressions, prompt_impact, compliance,
//!    distillation, gaps, reflexion. These render as
//!    `_section unavailable: not yet ported_` placeholders to keep
//!    the layout stable.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Section {
    pub name: String,
    pub title: String,
    pub body: String,
    pub ok: bool,
    pub error: Option<String>,
}

pub fn sections() -> Vec<Section> {
    let mut out: Vec<Section> = Vec::new();
    out.push(score_section());
    out.push(ledger_section());
    out.push(regression_section());
    out.push(prompt_section());
    out.push(cases_section());
    // Placeholders for not-yet-ported modules — keep layout stable.
    for (name, title) in [
        ("safety", "Safety gate (cooldown + budget)"),
        ("suppressions", "Finding suppressions"),
        ("prompt_impact", "Prompt-impact analysis"),
        ("compliance", "Compliance threshold tuner"),
        ("distillation", "Per-tier distillation matrix"),
        ("gaps", "Capability gaps"),
        ("reflexion", "Reflexion themes"),
    ] {
        out.push(Section {
            name: name.into(),
            title: title.into(),
            body: "(not yet ported to Rust — see MIGRATION_RUST_PHOENIX.md Phase 2)".into(),
            ok: false,
            error: Some("module pending port".into()),
        });
    }
    out
}

fn score_section() -> Section {
    use aim_ai_health::{compute, info_line};
    let s = (|| -> Result<String, Box<dyn std::error::Error>> {
        let ledger = aim_ai_ledger::Ledger::open_default()?;
        let score = compute(&ledger)?;
        Ok(format!(
            "{}\n  notes: {}",
            info_line(&score),
            if score.notes.is_empty() {
                "—".to_string()
            } else {
                score.notes.join("; ")
            }
        ))
    })();
    section_from("score", "Health score", s)
}

fn ledger_section() -> Section {
    let s = (|| -> Result<String, Box<dyn std::error::Error>> {
        let ledger = aim_ai_ledger::Ledger::open_default()?;
        let t = ledger.trend()?;
        if t.n_runs == 0 {
            return Ok("(no diagnostic runs recorded)".into());
        }
        Ok(format!(
            "📈 Diagnostic ledger — {} runs (first {} → last {})\n  avg compliance: {:.0}%\n  avg crit: {:.1}\n  retry share: {:.0}%",
            t.n_runs,
            t.first_ts.as_deref().unwrap_or("?").chars().take(10).collect::<String>(),
            t.last_ts.as_deref().unwrap_or("?").chars().take(10).collect::<String>(),
            t.avg_compliance * 100.0,
            t.avg_crit,
            t.retry_share * 100.0,
        ))
    })();
    section_from("ledger", "Diagnostic ledger trend", s)
}

fn regression_section() -> Section {
    let s = (|| -> Result<String, Box<dyn std::error::Error>> {
        let ledger = aim_ai_ledger::Ledger::open_default()?;
        let r = aim_ai_regression::detect(&ledger)?;
        if !r.have_baseline {
            return Ok("(no baseline — need ≥2 ledger rows)".into());
        }
        let pg = r.prev_grade.clone().unwrap_or_else(|| "?".into());
        let cg = r.curr_grade.clone().unwrap_or_else(|| "?".into());
        let pc = r.prev_crit.map(|n| n.to_string()).unwrap_or_else(|| "?".into());
        let cc = r.curr_crit.map(|n| n.to_string()).unwrap_or_else(|| "?".into());
        let verdict = if r.regressed() {
            "⚠ REGRESSED"
        } else if r.improved() {
            "✅ IMPROVED"
        } else {
            "= stable"
        };
        Ok(format!(
            "🔍 Regression check\n  grade: {pg} → {cg}\n  crit:  {pc} → {cc}\n  new findings:   {}\n  fixed findings: {}\n  {}",
            r.new_findings.len(),
            r.fixed_findings.len(),
            verdict
        ))
    })();
    section_from("regression", "Regression check", s)
}

fn prompt_section() -> Section {
    let s = (|| -> Result<String, Box<dyn std::error::Error>> {
        let store = aim_ai_prompt_versions::PromptStore::open_default()?;
        let drift = store.drift_since_last(None)?;
        if !drift.prompt_present {
            return Ok("(prompt file missing)".into());
        }
        if !drift.have_baseline {
            return Ok(format!(
                "📝 Prompt fingerprinted for the first time:\n  sha {}…",
                drift
                    .current_sha
                    .as_deref()
                    .unwrap_or("?")
                    .chars()
                    .take(12)
                    .collect::<String>()
            ));
        }
        if !drift.changed {
            return Ok("📝 Prompt unchanged since last record".into());
        }
        Ok(format!(
            "📝 Prompt drift since {}\n  sha {} → {}\n  bytes Δ {:+}  lines Δ {:+}",
            drift.last_ts.as_deref().unwrap_or("?"),
            drift.last_sha.as_deref().unwrap_or("?").chars().take(8).collect::<String>(),
            drift.current_sha.as_deref().unwrap_or("?").chars().take(8).collect::<String>(),
            drift.delta_bytes,
            drift.delta_lines,
        ))
    })();
    section_from("prompt", "Diagnostic prompt drift", s)
}

fn cases_section() -> Section {
    let s = (|| -> Result<String, Box<dyn std::error::Error>> {
        let r = aim_ai_cases::validate_dir(None);
        if r.n_cases == 0 {
            return Ok("(no eval cases found)".into());
        }
        Ok(format!(
            "📋 Case validator — {} cases ({} ok / {} failed)",
            r.n_cases, r.n_ok, r.n_failed
        ))
    })();
    section_from("cases", "Eval case validator", s)
}

fn section_from(
    name: &str,
    title: &str,
    result: Result<String, Box<dyn std::error::Error>>,
) -> Section {
    match result {
        Ok(body) => Section {
            name: name.into(),
            title: title.into(),
            body,
            ok: true,
            error: None,
        },
        Err(e) => Section {
            name: name.into(),
            title: title.into(),
            body: "(unavailable)".into(),
            ok: false,
            error: Some(format!("{}", e)),
        },
    }
}

pub fn render() -> String {
    let mut out: Vec<String> = vec!["# AIM/AI Dashboard\n".into()];
    for s in sections() {
        out.push(format!("## {}", s.title));
        out.push(String::new());
        out.push(s.body.clone());
        if let Some(e) = s.error {
            out.push(format!("_section error: {}_", e));
        }
        out.push(String::new());
    }
    let mut joined = out.join("\n");
    while joined.ends_with('\n') {
        joined.pop();
    }
    joined.push('\n');
    joined
}

pub fn render_json() -> serde_json::Value {
    let payload: Vec<_> = sections()
        .into_iter()
        .map(|s| {
            serde_json::json!({
                "name": s.name,
                "title": s.title,
                "body": s.body,
                "ok": s.ok,
                "error": s.error,
            })
        })
        .collect();
    serde_json::json!({ "sections": payload })
}

pub fn render_compact() -> String {
    let mut out: Vec<String> = vec!["📡 AIM/AI compact".into()];
    for s in sections() {
        let head = s
            .body
            .lines()
            .find(|l| !l.trim().is_empty() && !l.trim().starts_with('#'))
            .map(|l| l.trim().to_string())
            .unwrap_or_else(|| "(empty)".into());
        let head = strip_emoji_prefix(&head);
        let mark = if s.ok { "✓" } else { "✗" };
        out.push(format!("{} {}: {}", mark, s.title, head));
    }
    out.join("\n")
}

fn strip_emoji_prefix(s: &str) -> String {
    let trimmed: String = s
        .chars()
        .skip_while(|c| !c.is_alphanumeric())
        .collect();
    trimmed.chars().take(120).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_emits_all_sections() {
        let txt = render();
        assert!(txt.contains("# AIM/AI Dashboard"));
        assert!(txt.contains("## Health score"));
        assert!(txt.contains("## Diagnostic ledger trend"));
        assert!(txt.contains("## Regression check"));
        assert!(txt.contains("## Diagnostic prompt drift"));
        assert!(txt.contains("## Eval case validator"));
        assert!(txt.contains("## Reflexion themes"));
    }

    #[test]
    fn render_json_envelope() {
        let v = render_json();
        let arr = v["sections"].as_array().unwrap();
        assert!(arr.len() >= 5);
        for s in arr {
            assert!(s["name"].is_string());
            assert!(s["title"].is_string());
            assert!(s["body"].is_string());
        }
    }

    #[test]
    fn render_compact_one_line_per_section() {
        let c = render_compact();
        assert!(c.starts_with("📡 AIM/AI compact"));
        let lines: Vec<&str> = c.lines().collect();
        // Header + one per section
        assert!(lines.len() >= 6);
    }

    #[test]
    fn placeholder_sections_marked_not_ok() {
        let secs = sections();
        let placeholder = secs.iter().find(|s| s.name == "reflexion").unwrap();
        assert!(!placeholder.ok);
        assert!(placeholder.body.contains("not yet ported"));
    }
}
