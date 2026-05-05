//! L_PRIVACY scrubber. Recursively walks the payload; if any string
//! matches a PII pattern, the whole call aborts.
//!
//! Patterns mirror `AI/ai/hive_telemetry.py::_PII_PATTERNS` exactly.

use once_cell::sync::Lazy;
use regex::Regex;
use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ScrubError {
    #[error("PII pattern '{pattern}' matched in {sample}")]
    PiiMatch { pattern: &'static str, sample: String },
}

struct PiiPattern {
    name: &'static str,
    re: Regex,
}

static PATTERNS: Lazy<Vec<PiiPattern>> = Lazy::new(|| {
    vec![
        PiiPattern {
            name: "email",
            re: Regex::new(r"\b[\w._%+-]+@[\w.-]+\.[A-Za-z]{2,}\b").unwrap(),
        },
        PiiPattern {
            name: "phone",
            re: Regex::new(r"\+\d{6,}").unwrap(),
        },
        PiiPattern {
            name: "user_path",
            re: Regex::new(r"/home/\w+|/Users/\w+|C:\\Users\\\w+").unwrap(),
        },
        PiiPattern {
            // first + last name ASCII Latin only — best-effort.
            name: "name_pair",
            re: Regex::new(r"\b[A-Z][a-z]+ [A-Z][a-z]+\b").unwrap(),
        },
        PiiPattern {
            name: "publication_id",
            re: Regex::new(r"\bPMID[: ]?\d+|10\.\d{4,}/\S+").unwrap(),
        },
    ]
});

/// Walk a JSON value; raise [`ScrubError`] on any PII match. Returns
/// the same value untouched on success.
pub fn scrub_value(v: Value) -> Result<Value, ScrubError> {
    match &v {
        Value::String(s) => {
            for p in PATTERNS.iter() {
                if let Some(m) = p.re.find(s) {
                    return Err(ScrubError::PiiMatch {
                        pattern: p.name,
                        sample: format!("{}…", m.as_str().chars().take(60).collect::<String>()),
                    });
                }
            }
        }
        Value::Array(arr) => {
            for item in arr {
                check_value(item)?;
            }
        }
        Value::Object(map) => {
            for (_k, val) in map {
                check_value(val)?;
            }
        }
        _ => {}
    }
    Ok(v)
}

fn check_value(v: &Value) -> Result<(), ScrubError> {
    match v {
        Value::String(s) => {
            for p in PATTERNS.iter() {
                if let Some(m) = p.re.find(s) {
                    return Err(ScrubError::PiiMatch {
                        pattern: p.name,
                        sample: format!("{}…", m.as_str().chars().take(60).collect::<String>()),
                    });
                }
            }
            Ok(())
        }
        Value::Array(arr) => {
            for item in arr {
                check_value(item)?;
            }
            Ok(())
        }
        Value::Object(map) => {
            for (_k, val) in map {
                check_value(val)?;
            }
            Ok(())
        }
        _ => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn pass_clean_payload() {
        let v = json!({"v":1,"counts":[1,2,3],"theme":"diagnosis"});
        scrub_value(v).unwrap();
    }

    #[test]
    fn block_email() {
        let v = json!({"contact":"djabbat@gmail.com"});
        let err = scrub_value(v).unwrap_err();
        assert!(matches!(err, ScrubError::PiiMatch { pattern: "email", .. }));
    }

    #[test]
    fn block_phone() {
        let v = json!({"contact":"+995555185161"});
        assert!(matches!(scrub_value(v).unwrap_err(),
            ScrubError::PiiMatch { pattern: "phone", .. }));
    }

    #[test]
    fn block_user_path() {
        let v = json!({"path":"/home/jaba/web"});
        assert!(matches!(scrub_value(v).unwrap_err(),
            ScrubError::PiiMatch { pattern: "user_path", .. }));
    }

    #[test]
    fn block_name() {
        let v = json!({"author":"Jaba Tkemaladze"});
        assert!(matches!(scrub_value(v).unwrap_err(),
            ScrubError::PiiMatch { pattern: "name_pair", .. }));
    }

    #[test]
    fn block_pmid() {
        let v = json!({"ref":"PMID 36583780"});
        assert!(matches!(scrub_value(v).unwrap_err(),
            ScrubError::PiiMatch { pattern: "publication_id", .. }));
    }

    #[test]
    fn block_doi() {
        let v = json!({"doi":"10.65649/xf5vp867"});
        assert!(matches!(scrub_value(v).unwrap_err(),
            ScrubError::PiiMatch { pattern: "publication_id", .. }));
    }

    #[test]
    fn block_nested_array() {
        let v = json!({"items":[{"x":1},{"contact":"djabbat@gmail.com"}]});
        assert!(scrub_value(v).is_err());
    }
}
