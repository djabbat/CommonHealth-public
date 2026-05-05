//! aim-patient-memory — markdown-canonical patient state.
//!
//! Port of `agents/patient_memory.py`. Format: `Patients/<ID>/MEMORY.md`
//! is the human-editable canonical store. SQLite index lives behind a
//! pluggable [`PatientIndex`] trait so the markdown round-trip is testable
//! without sqlite.

use std::path::{Path, PathBuf};

use chrono::{DateTime, Utc};
use regex::Regex;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PatientError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("not found: {0}")]
    NotFound(String),
}

pub type Result<T> = std::result::Result<T, PatientError>;

// ── data ────────────────────────────────────────────────────────────────────

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Demographics {
    pub age: Option<i32>,
    pub sex: Option<String>,
    pub country: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Medication {
    pub name: String,
    pub dose: Option<String>,
    pub freq: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Condition {
    pub dx: String,
    pub since: Option<String>,
    pub notes: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct PatientMemory {
    pub id: String,
    pub demographics: Demographics,
    pub allergies: Vec<String>,
    pub medications: Vec<Medication>,
    pub conditions: Vec<Condition>,
    pub history: Vec<String>,
    pub known_unknowns: Vec<String>,
    pub red_flags: Vec<String>,
    pub missing_labs_count: i32,
    pub history_contradictions: i32,
    pub unexplained_symptoms_count: i32,
    pub last_visit_years_ago: f64,
    pub dx_without_evidence: bool,
    pub primary_complaint_undiagnosed: bool,
    pub has_confirmed_dx: bool,
}

impl PatientMemory {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            primary_complaint_undiagnosed: true,
            ..Default::default()
        }
    }

    /// Flat dict shape for kernel scoring.
    pub fn to_kernel_json(&self) -> serde_json::Value {
        serde_json::json!({
            "id": self.id,
            "age": self.demographics.age,
            "sex": self.demographics.sex,
            "allergies": self.allergies,
            "medications": self.medications,
            "red_flags": self.red_flags,
            "missing_labs_count": self.missing_labs_count,
            "history_contradictions": self.history_contradictions,
            "unexplained_symptoms_count": self.unexplained_symptoms_count,
            "last_visit_years_ago": self.last_visit_years_ago,
            "dx_without_evidence": self.dx_without_evidence,
            "primary_complaint_undiagnosed": self.primary_complaint_undiagnosed,
            "has_confirmed_dx": self.has_confirmed_dx,
        })
    }
}

// ── markdown rendering ─────────────────────────────────────────────────────

pub const MEMORY_FILE: &str = "MEMORY.md";

fn bullets<I: IntoIterator<Item = S>, S: AsRef<str>>(items: I, empty: &str) -> String {
    let lines: Vec<String> = items
        .into_iter()
        .map(|s| format!("- {}", s.as_ref()))
        .collect();
    if lines.is_empty() {
        empty.to_string()
    } else {
        lines.join("\n")
    }
}

fn med_bullet(m: &Medication) -> String {
    format!(
        "- {} · {} · {}",
        if m.name.is_empty() { "?" } else { &m.name },
        m.dose.as_deref().unwrap_or("?"),
        m.freq.as_deref().unwrap_or("?")
    )
}

fn cond_bullet(c: &Condition) -> String {
    format!(
        "- {} ({}): {}",
        if c.dx.is_empty() { "?" } else { &c.dx },
        c.since.as_deref().unwrap_or("?"),
        c.notes.as_deref().unwrap_or("")
    )
}

pub fn render(mem: &PatientMemory, ts: DateTime<Utc>) -> String {
    let medications = if mem.medications.is_empty() {
        "_(none)_".to_string()
    } else {
        mem.medications
            .iter()
            .map(med_bullet)
            .collect::<Vec<_>>()
            .join("\n")
    };
    let conditions = if mem.conditions.is_empty() {
        "_(none)_".to_string()
    } else {
        mem.conditions
            .iter()
            .map(cond_bullet)
            .collect::<Vec<_>>()
            .join("\n")
    };
    let allergies = bullets(&mem.allergies, "_(none)_");
    let history = bullets(&mem.history, "_(none)_");
    let unknowns = bullets(&mem.known_unknowns, "_(none)_");
    let age = mem
        .demographics
        .age
        .map(|n| n.to_string())
        .unwrap_or_else(|| "?".into());
    let sex = mem.demographics.sex.as_deref().unwrap_or("?");
    let country = mem.demographics.country.as_deref().unwrap_or("?");
    format!(
        "# Memory — {id}\n\n\
## Demographics\n\
- Age: {age}\n\
- Sex: {sex}\n\
- Country: {country}\n\n\
## Allergies\n{allergies}\n\n\
## Medications\n{medications}\n\n\
## Conditions\n{conditions}\n\n\
## History (reverse-chron)\n{history}\n\n\
## Known unknowns\n{unknowns}\n\n\
## Derived (для kernel scoring)\n\
- primary_complaint_undiagnosed: {pcu}\n\
- has_confirmed_dx: {hcd}\n\
- missing_labs_count: {mlc}\n\
- history_contradictions: {hc}\n\
- unexplained_symptoms_count: {usc}\n\
- last_visit_years_ago: {lvya}\n\
- dx_without_evidence: {dwe}\n\n\
---\n_Last updated: {ts}. Edit freely; AIM will parse on next read._\n",
        id = mem.id,
        age = age,
        sex = sex,
        country = country,
        allergies = allergies,
        medications = medications,
        conditions = conditions,
        history = history,
        unknowns = unknowns,
        pcu = mem.primary_complaint_undiagnosed,
        hcd = mem.has_confirmed_dx,
        mlc = mem.missing_labs_count,
        hc = mem.history_contradictions,
        usc = mem.unexplained_symptoms_count,
        lvya = mem.last_visit_years_ago,
        dwe = mem.dx_without_evidence,
        ts = ts.format("%Y-%m-%d %H:%M:%S")
    )
}

// ── markdown parsing ────────────────────────────────────────────────────────

fn split_sections(text: &str) -> std::collections::BTreeMap<String, Vec<String>> {
    let mut map: std::collections::BTreeMap<String, Vec<String>> = std::collections::BTreeMap::new();
    let mut current: Option<String> = None;
    for line in text.lines() {
        if let Some(rest) = line.strip_prefix("## ") {
            let name = rest.trim().to_string();
            current = Some(name.clone());
            map.entry(name).or_default();
        } else if let Some(name) = &current {
            map.get_mut(name).unwrap().push(line.to_string());
        }
    }
    map
}

fn parse_bullet(line: &str) -> Option<&str> {
    let trimmed = line.trim();
    if !trimmed.starts_with("- ") {
        return None;
    }
    let body = &trimmed[2..];
    if body.starts_with('_') {
        return None;
    }
    Some(body.trim())
}

pub fn parse(id: &str, text: &str) -> PatientMemory {
    let sections = split_sections(text);
    let mut mem = PatientMemory::new(id);

    if let Some(lines) = sections.get("Demographics") {
        let kv = Regex::new(r"^- (\w+):\s*(.+)$").unwrap();
        for line in lines {
            if let Some(c) = kv.captures(line.trim()) {
                let key = c.get(1).map(|m| m.as_str().to_lowercase()).unwrap_or_default();
                let val = c.get(2).map(|m| m.as_str().trim().to_string()).unwrap_or_default();
                match key.as_str() {
                    "age" => mem.demographics.age = val.parse().ok(),
                    "sex" => {
                        mem.demographics.sex = if val == "?" { None } else { Some(val) };
                    }
                    "country" => {
                        mem.demographics.country = if val == "?" { None } else { Some(val) };
                    }
                    _ => {}
                }
            }
        }
    }

    if let Some(lines) = sections.get("Allergies") {
        for line in lines {
            if let Some(body) = parse_bullet(line) {
                mem.allergies.push(body.to_string());
            }
        }
    }

    if let Some(lines) = sections.get("Medications") {
        for line in lines {
            if let Some(body) = parse_bullet(line) {
                let parts: Vec<String> = body.split('·').map(|s| s.trim().to_string()).collect();
                let mut med = Medication {
                    name: parts.first().cloned().unwrap_or_else(|| "?".into()),
                    dose: parts.get(1).cloned(),
                    freq: parts.get(2).cloned(),
                };
                // treat literal "?" placeholders as None
                if med.dose.as_deref() == Some("?") {
                    med.dose = None;
                }
                if med.freq.as_deref() == Some("?") {
                    med.freq = None;
                }
                mem.medications.push(med);
            }
        }
    }

    if let Some(lines) = sections.get("Conditions") {
        let cond_re = Regex::new(r"^- (.+?) \((.+?)\):?\s*(.*)$").unwrap();
        for line in lines {
            let trimmed = line.trim();
            if !trimmed.starts_with("- ") || trimmed.starts_with("- _") {
                continue;
            }
            if let Some(c) = cond_re.captures(trimmed) {
                let dx = c.get(1).map(|m| m.as_str().to_string()).unwrap_or_default();
                let since = c.get(2).map(|m| m.as_str().to_string());
                let notes = c.get(3).map(|m| m.as_str().to_string()).filter(|s| !s.is_empty());
                mem.conditions.push(Condition {
                    dx,
                    since,
                    notes,
                });
            }
        }
    }

    if let Some(lines) = sections.get("History (reverse-chron)") {
        for line in lines {
            if let Some(body) = parse_bullet(line) {
                mem.history.push(body.to_string());
            }
        }
    }

    if let Some(lines) = sections.get("Known unknowns") {
        for line in lines {
            if let Some(body) = parse_bullet(line) {
                mem.known_unknowns.push(body.to_string());
            }
        }
    }

    if let Some(lines) = sections.get("Derived (для kernel scoring)") {
        let kv = Regex::new(r"^- (\w+):\s*(.+)$").unwrap();
        for line in lines {
            if let Some(c) = kv.captures(line.trim()) {
                let key = c.get(1).map(|m| m.as_str()).unwrap_or("");
                let val = c.get(2).map(|m| m.as_str().trim()).unwrap_or("");
                match key {
                    "primary_complaint_undiagnosed" => {
                        mem.primary_complaint_undiagnosed = val.eq_ignore_ascii_case("true");
                    }
                    "has_confirmed_dx" => {
                        mem.has_confirmed_dx = val.eq_ignore_ascii_case("true");
                    }
                    "dx_without_evidence" => {
                        mem.dx_without_evidence = val.eq_ignore_ascii_case("true");
                    }
                    "missing_labs_count" => {
                        if let Ok(n) = val.parse() {
                            mem.missing_labs_count = n;
                        }
                    }
                    "history_contradictions" => {
                        if let Ok(n) = val.parse() {
                            mem.history_contradictions = n;
                        }
                    }
                    "unexplained_symptoms_count" => {
                        if let Ok(n) = val.parse() {
                            mem.unexplained_symptoms_count = n;
                        }
                    }
                    "last_visit_years_ago" => {
                        if let Ok(n) = val.parse() {
                            mem.last_visit_years_ago = n;
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    // Derive: conditions present implies confirmed dx + complaint diagnosed.
    if !mem.conditions.is_empty() && !mem.has_confirmed_dx {
        mem.has_confirmed_dx = true;
        mem.primary_complaint_undiagnosed = false;
    }

    mem
}

// ── filesystem I/O + index ─────────────────────────────────────────────────

pub fn memory_path(patients_root: &Path, patient_id: &str) -> PathBuf {
    patients_root.join(patient_id).join(MEMORY_FILE)
}

pub trait PatientIndex: Send + Sync {
    fn upsert(&self, mem: &PatientMemory) -> Result<()>;
    fn list(&self) -> Result<Vec<PatientMemory>>;
}

pub struct NoopIndex;
impl PatientIndex for NoopIndex {
    fn upsert(&self, _: &PatientMemory) -> Result<()> {
        Ok(())
    }
    fn list(&self) -> Result<Vec<PatientMemory>> {
        Ok(Vec::new())
    }
}

pub trait Clock: Send + Sync {
    fn now(&self) -> DateTime<Utc>;
}

pub struct SystemClock;
impl Clock for SystemClock {
    fn now(&self) -> DateTime<Utc> {
        Utc::now()
    }
}

pub struct FixedClock(pub DateTime<Utc>);
impl Clock for FixedClock {
    fn now(&self) -> DateTime<Utc> {
        self.0
    }
}

pub fn write_memory(
    patients_root: &Path,
    mem: &PatientMemory,
    clock: &dyn Clock,
    index: &dyn PatientIndex,
) -> Result<PathBuf> {
    let path = memory_path(patients_root, &mem.id);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(&path, render(mem, clock.now()))?;
    index.upsert(mem)?;
    Ok(path)
}

pub fn read_memory(
    patients_root: &Path,
    patient_id: &str,
    index: &dyn PatientIndex,
) -> Result<Option<PatientMemory>> {
    let path = memory_path(patients_root, patient_id);
    if !path.exists() {
        return Ok(None);
    }
    let text = std::fs::read_to_string(&path)?;
    let mem = parse(patient_id, &text);
    index.upsert(&mem)?;
    Ok(Some(mem))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use tempfile::TempDir;

    fn ts() -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2026, 5, 5, 0, 0, 0).unwrap()
    }

    // ── render ─────────────────────────────────────────────────────────────

    #[test]
    fn render_includes_all_sections() {
        let mem = PatientMemory {
            id: "Smith_John_1980_05_15".into(),
            demographics: Demographics {
                age: Some(45),
                sex: Some("M".into()),
                country: Some("GE".into()),
            },
            allergies: vec!["penicillin".into()],
            medications: vec![Medication {
                name: "metformin".into(),
                dose: Some("500 mg".into()),
                freq: Some("BID".into()),
            }],
            conditions: vec![Condition {
                dx: "T2DM".into(),
                since: Some("2020".into()),
                notes: Some("controlled".into()),
            }],
            history: vec!["MI 2018".into()],
            known_unknowns: vec!["family hx".into()],
            ..Default::default()
        };
        let s = render(&mem, ts());
        assert!(s.starts_with("# Memory — Smith_John_1980_05_15"));
        assert!(s.contains("- Age: 45"));
        assert!(s.contains("- penicillin"));
        assert!(s.contains("metformin · 500 mg · BID"));
        assert!(s.contains("T2DM (2020): controlled"));
        assert!(s.contains("- MI 2018"));
        assert!(s.contains("- family hx"));
        assert!(s.contains("Last updated: 2026-05-05 00:00:00"));
    }

    #[test]
    fn render_uses_none_placeholders_for_empty_lists() {
        let mem = PatientMemory::new("X");
        let s = render(&mem, ts());
        assert!(s.contains("## Allergies\n_(none)_"));
        assert!(s.contains("## Medications\n_(none)_"));
        assert!(s.contains("## Conditions\n_(none)_"));
    }

    #[test]
    fn render_demographic_question_marks_when_empty() {
        let mem = PatientMemory::new("X");
        let s = render(&mem, ts());
        assert!(s.contains("- Age: ?"));
        assert!(s.contains("- Sex: ?"));
    }

    // ── parse ──────────────────────────────────────────────────────────────

    #[test]
    fn parse_round_trips_demographics_and_allergies() {
        let original = PatientMemory {
            id: "X_Y_1990_01_01".into(),
            demographics: Demographics {
                age: Some(35),
                sex: Some("F".into()),
                country: Some("GE".into()),
            },
            allergies: vec!["aspirin".into(), "ibuprofen".into()],
            ..Default::default()
        };
        let text = render(&original, ts());
        let parsed = parse(&original.id, &text);
        assert_eq!(parsed.demographics, original.demographics);
        assert_eq!(parsed.allergies, original.allergies);
    }

    #[test]
    fn parse_handles_medications_with_dose_and_freq() {
        let mem = PatientMemory {
            id: "X".into(),
            medications: vec![Medication {
                name: "warfarin".into(),
                dose: Some("5 mg".into()),
                freq: Some("daily".into()),
            }],
            ..Default::default()
        };
        let text = render(&mem, ts());
        let p = parse(&mem.id, &text);
        assert_eq!(p.medications.len(), 1);
        assert_eq!(p.medications[0].name, "warfarin");
        assert_eq!(p.medications[0].dose.as_deref(), Some("5 mg"));
        assert_eq!(p.medications[0].freq.as_deref(), Some("daily"));
    }

    #[test]
    fn parse_skips_none_placeholders() {
        let mem = PatientMemory::new("X");
        let text = render(&mem, ts());
        let p = parse(&mem.id, &text);
        assert!(p.allergies.is_empty());
        assert!(p.medications.is_empty());
        assert!(p.conditions.is_empty());
    }

    #[test]
    fn parse_conditions_round_trip() {
        let mem = PatientMemory {
            id: "X".into(),
            conditions: vec![Condition {
                dx: "HTN".into(),
                since: Some("2015".into()),
                notes: Some("on lisinopril".into()),
            }],
            ..Default::default()
        };
        let text = render(&mem, ts());
        let p = parse(&mem.id, &text);
        assert_eq!(p.conditions.len(), 1);
        assert_eq!(p.conditions[0].dx, "HTN");
        assert_eq!(p.conditions[0].since.as_deref(), Some("2015"));
        assert_eq!(p.conditions[0].notes.as_deref(), Some("on lisinopril"));
    }

    #[test]
    fn parse_derives_confirmed_dx_when_conditions_present() {
        let mem = PatientMemory {
            id: "X".into(),
            conditions: vec![Condition {
                dx: "HTN".into(),
                since: Some("2015".into()),
                notes: None,
            }],
            primary_complaint_undiagnosed: true,
            has_confirmed_dx: false,
            ..Default::default()
        };
        let text = render(&mem, ts());
        let p = parse(&mem.id, &text);
        assert!(p.has_confirmed_dx);
        assert!(!p.primary_complaint_undiagnosed);
    }

    #[test]
    fn parse_derived_section_round_trips() {
        let mem = PatientMemory {
            id: "X".into(),
            missing_labs_count: 3,
            history_contradictions: 1,
            unexplained_symptoms_count: 2,
            last_visit_years_ago: 1.5,
            dx_without_evidence: true,
            primary_complaint_undiagnosed: true,
            has_confirmed_dx: false,
            ..Default::default()
        };
        let text = render(&mem, ts());
        let p = parse(&mem.id, &text);
        assert_eq!(p.missing_labs_count, 3);
        assert_eq!(p.history_contradictions, 1);
        assert_eq!(p.unexplained_symptoms_count, 2);
        assert!((p.last_visit_years_ago - 1.5).abs() < 1e-9);
        assert!(p.dx_without_evidence);
    }

    // ── filesystem I/O ─────────────────────────────────────────────────────

    #[test]
    fn write_then_read_round_trips() {
        let tmp = TempDir::new().unwrap();
        let mem = PatientMemory {
            id: "Smith_John_1980_05_15".into(),
            demographics: Demographics {
                age: Some(45),
                sex: Some("M".into()),
                country: None,
            },
            allergies: vec!["aspirin".into()],
            ..Default::default()
        };
        let clk = FixedClock(ts());
        let idx = NoopIndex;
        let path = write_memory(tmp.path(), &mem, &clk, &idx).unwrap();
        assert!(path.exists());
        assert!(path.ends_with("Smith_John_1980_05_15/MEMORY.md"));
        let reloaded = read_memory(tmp.path(), &mem.id, &idx).unwrap().unwrap();
        assert_eq!(reloaded.demographics, mem.demographics);
        assert_eq!(reloaded.allergies, mem.allergies);
    }

    #[test]
    fn read_memory_missing_returns_none() {
        let tmp = TempDir::new().unwrap();
        let r = read_memory(tmp.path(), "ghost", &NoopIndex).unwrap();
        assert!(r.is_none());
    }

    // ── PatientIndex side-effect ───────────────────────────────────────────

    #[derive(Default)]
    struct CountingIndex(parking_lot::Mutex<usize>);
    impl CountingIndex {
        fn count(&self) -> usize {
            *self.0.lock()
        }
    }
    impl PatientIndex for CountingIndex {
        fn upsert(&self, _: &PatientMemory) -> Result<()> {
            *self.0.lock() += 1;
            Ok(())
        }
        fn list(&self) -> Result<Vec<PatientMemory>> {
            Ok(Vec::new())
        }
    }

    #[test]
    fn write_memory_calls_index_upsert() {
        let tmp = TempDir::new().unwrap();
        let mem = PatientMemory::new("X");
        let clk = FixedClock(ts());
        let idx = CountingIndex::default();
        write_memory(tmp.path(), &mem, &clk, &idx).unwrap();
        assert_eq!(idx.count(), 1);
    }

    #[test]
    fn read_memory_calls_index_upsert() {
        let tmp = TempDir::new().unwrap();
        let mem = PatientMemory::new("X");
        let clk = FixedClock(ts());
        let noop = NoopIndex;
        write_memory(tmp.path(), &mem, &clk, &noop).unwrap();
        let idx = CountingIndex::default();
        read_memory(tmp.path(), "X", &idx).unwrap();
        assert_eq!(idx.count(), 1);
    }

    // ── to_kernel_json ─────────────────────────────────────────────────────

    #[test]
    fn to_kernel_json_emits_flat_fields() {
        let mem = PatientMemory {
            id: "X".into(),
            demographics: Demographics {
                age: Some(30),
                sex: Some("F".into()),
                country: None,
            },
            allergies: vec!["x".into()],
            primary_complaint_undiagnosed: true,
            has_confirmed_dx: false,
            ..Default::default()
        };
        let v = mem.to_kernel_json();
        assert_eq!(v["age"], serde_json::json!(30));
        assert_eq!(v["sex"], serde_json::json!("F"));
        assert_eq!(v["allergies"], serde_json::json!(["x"]));
        assert_eq!(v["primary_complaint_undiagnosed"], serde_json::json!(true));
    }
}
