//! High-level navigation helpers — list projects and patients for the UI
//! without touching the SQLite layer.  Parses just enough of the on-disk
//! files to build a summary card per item.

use crate::error::Result;
use crate::AimFs;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSummary {
    pub slug: String,
    pub path: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientSummary {
    pub key: String,
    pub path: String,
    pub surname: Option<String>,
    pub name: Option<String>,
    pub dob: Option<String>,
    pub phone: Option<String>,
    /// First line of the most recent visit's intake.md, if present.
    pub last_visit_complaint: Option<String>,
}

impl AimFs {
    pub fn list_projects(&self, user_id: &str) -> Result<Vec<ProjectSummary>> {
        let dir = self.root().join("users").join(user_id).join("projects");
        if !dir.is_dir() {
            return Ok(vec![]);
        }
        let mut out = vec![];
        for entry in std::fs::read_dir(&dir)? {
            let entry = entry?;
            if !entry.file_type()?.is_dir() {
                continue;
            }
            let p = entry.path();
            let slug = p
                .file_name()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_default();
            if slug.starts_with('_') {
                continue;
            }
            out.push(read_project_summary(&p, slug));
        }
        out.sort_by(|a, b| a.slug.cmp(&b.slug));
        Ok(out)
    }

    pub fn list_patients(&self, doctor_id: &str) -> Result<Vec<PatientSummary>> {
        let dir = self.root().join("users").join(doctor_id).join("patients");
        if !dir.is_dir() {
            return Ok(vec![]);
        }
        let mut out = vec![];
        for entry in std::fs::read_dir(&dir)? {
            let entry = entry?;
            if !entry.file_type()?.is_dir() {
                continue;
            }
            let p = entry.path();
            let key = p
                .file_name()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_default();
            if key.starts_with('_') {
                continue;
            }
            out.push(read_patient_summary(&p, key));
        }
        out.sort_by(|a, b| a.key.cmp(&b.key));
        Ok(out)
    }
}

fn read_project_summary(dir: &Path, slug: String) -> ProjectSummary {
    let title = read_title(&dir.join("CONCEPT.md"))
        .or_else(|| read_title(&dir.join("README.md")));
    let description = read_first_paragraph(&dir.join("README.md"))
        .or_else(|| read_first_paragraph(&dir.join("CONCEPT.md")));
    let (status, created_at) = read_state_md(&dir.join("STATE.md"));
    ProjectSummary {
        slug,
        path: dir.display().to_string(),
        title,
        description,
        status,
        created_at,
    }
}

fn read_patient_summary(dir: &Path, key: String) -> PatientSummary {
    let mut sum = PatientSummary {
        key: key.clone(),
        path: dir.display().to_string(),
        surname: None,
        name: None,
        dob: None,
        phone: None,
        last_visit_complaint: None,
    };

    if let Ok(s) = std::fs::read_to_string(dir.join("identity.toml")) {
        for line in s.lines() {
            if let Some((k, v)) = line.split_once('=') {
                let k = k.trim();
                let v = v.trim().trim_matches('"').to_string();
                match k {
                    "surname" => sum.surname = Some(v),
                    "name" => sum.name = Some(v),
                    "dob" => sum.dob = Some(v),
                    "phone" => sum.phone = Some(v),
                    _ => {}
                }
            }
        }
    }
    // Fall back: parse <Surname>_<Name>_<YYYY_MM_DD> from folder name.
    if sum.surname.is_none() {
        let parts: Vec<&str> = key.split('_').collect();
        if parts.len() >= 5 {
            sum.surname = Some(parts[0].to_string());
            sum.name = Some(parts[1].to_string());
            sum.dob = Some(parts[2..5].join("_"));
        } else if parts.len() == 3 {
            // Surname_Name_DOB — DOB came in as one token, unusual.
            sum.surname = Some(parts[0].to_string());
            sum.name = Some(parts[1].to_string());
            sum.dob = Some(parts[2].to_string());
        }
    }

    // Latest visit — pick the highest-named directory under visits/.
    let visits_dir = dir.join("visits");
    if visits_dir.is_dir() {
        let mut visits: Vec<PathBuf> = std::fs::read_dir(&visits_dir)
            .ok()
            .into_iter()
            .flatten()
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| p.is_dir())
            .collect();
        visits.sort();
        if let Some(latest) = visits.last() {
            for cand in ["intake.md", "_first_intake.md"] {
                if let Ok(s) = std::fs::read_to_string(latest.join(cand)) {
                    if let Some(p) = first_non_heading_paragraph(&s) {
                        sum.last_visit_complaint = Some(p);
                        break;
                    }
                }
            }
        }
        // patient.yaml writes _first_intake.md DIRECTLY in visits/ (not a subdir),
        // so try that too:
        for cand in ["_first_intake.md", "intake.md"] {
            let p = visits_dir.join(cand);
            if p.is_file() {
                if let Ok(s) = std::fs::read_to_string(&p) {
                    if let Some(p) = first_non_heading_paragraph(&s) {
                        sum.last_visit_complaint
                            .get_or_insert(p);
                    }
                }
            }
        }
    }

    sum
}

fn read_title(path: &Path) -> Option<String> {
    let s = std::fs::read_to_string(path).ok()?;
    for line in s.lines() {
        let trimmed = line.trim_start_matches(|c: char| c == '#' || c.is_whitespace());
        if !trimmed.is_empty() && line.starts_with('#') {
            return Some(trimmed.to_string());
        }
    }
    None
}

fn read_first_paragraph(path: &Path) -> Option<String> {
    let s = std::fs::read_to_string(path).ok()?;
    first_non_heading_paragraph(&s)
}

fn first_non_heading_paragraph(s: &str) -> Option<String> {
    let mut buf = String::new();
    for line in s.lines() {
        if line.starts_with('#') || line.trim().is_empty() {
            if !buf.is_empty() {
                return Some(buf.trim().to_string());
            }
            continue;
        }
        if !buf.is_empty() {
            buf.push(' ');
        }
        buf.push_str(line.trim());
    }
    if buf.is_empty() {
        None
    } else {
        Some(buf.trim().to_string())
    }
}

fn read_state_md(path: &Path) -> (Option<String>, Option<String>) {
    let s = match std::fs::read_to_string(path) {
        Ok(s) => s,
        Err(_) => return (None, None),
    };
    let mut status = None;
    let mut created_at = None;
    for line in s.lines() {
        if let Some(rest) = line.strip_prefix("status:") {
            status = Some(rest.trim().to_string());
        } else if let Some(rest) = line.strip_prefix("created_at:") {
            created_at = Some(rest.trim().to_string());
        }
    }
    (status, created_at)
}
