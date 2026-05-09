# AUDIT PACKET — Iqalto_iqalto-core

Path: `/home/oem/Desktop/Iqalto/iqalto-core`  Date: 2026-05-08

## Size & file counts
```
176K	/home/oem/Desktop/Iqalto/iqalto-core
```
**Extensions:** .rs=14, .toml=5, .lock=1
## Tree (depth=2, max 200 entries)
```
.
./Cargo.toml
./crates
./crates/curriculum
./crates/assessment
./crates/simulation
./crates/ffi
./Cargo.lock
```
## Detected stack: **Rust**
## Core files

### `Cargo.toml` (332 chars)
```toml
[workspace]
members = [
    "crates/simulation",
    "crates/assessment",
    "crates/curriculum",
    "crates/ffi",
]
resolver = "2"

[workspace.dependencies]
serde       = { version = "1", features = ["derive"] }
serde_json  = "1"
uuid        = { version = "1", features = ["v4", "serde"] }
thiserror   = "1"
rustler     = "0.32"

```
### code `crates/curriculum/src/lib.rs`
```
pub mod student_record;
pub use student_record::{StudentRecord, CurriculumPath};

use iqalto_simulation::{CraftKind, SimLevel};

/// The four disciplines and their required completion order.
pub const ALL_CRAFTS: [CraftKind; 4] = [
    CraftKind::Winery,
    CraftKind::Bakery,
    CraftKind::Pottery,
    CraftKind::Forge,
];

pub const ALL_LEVELS: [SimLevel; 3] = [
    SimLevel::Cognitive,
    SimLevel::Procedural,
    SimLevel::Diagnostic,
];

/// Compute the next curriculum milestone for a student.
///
/// Returns `None` if the student has completed all paths.
pub fn unlock_next(record: &StudentRecord) -> Option<CurriculumPath> {
    match &record.path {
        CurriculumPath::Trivium => {
            // Must complete Cognitive on all 4 crafts to advance to Quadrivium
            let all_cognitive = ALL_CRAFTS.iter().all(|c| {
                record.has_passed(*c, SimLevel::Cognitive)
            });
            if all_cognitive { Some(CurriculumPath::Quadrivium) } else { None }
        }
        CurriculumPath::Quadrivium => {
            // Must pass Procedural + Diagnostic on all crafts
            let all_diagnostic = ALL_CRAFTS.iter().all(|c| {
                record.has_passed(*c, SimLevel::Diagnostic)
            });
            if all_diagnostic {
                Some(CurriculumPath::Faculty(record.chosen_craft
                    .expect("craft must be chosen before faculty")))
            } else {
                None
            }
        }
        CurriculumPath::Faculty(_) => None, // End of simulator path
    }
}

```
### code `crates/assessment/src/lib.rs`
```
pub mod report;
pub mod thresholds;

pub use report::AssessmentReport;
pub use thresholds::{pass_threshold, DIAGNOSTIC_PASS, PROCEDURAL_PASS, COGNITIVE_PASS};

use iqalto_simulation::{CraftKind, SimLevel, StepResult};

/// Assess a completed simulation run.
///
/// Takes the ordered log of step results and returns a scored report.
pub fn assess(craft: CraftKind, level: SimLevel, run_log: &[StepResult]) -> AssessmentReport {
    if run_log.is_empty() {
        return AssessmentReport::zero(craft, level);
    }

    let total_steps = run_log.len() as f32;
    let correct_steps = run_log.iter().filter(|r| r.correct).count() as f32;
    let accuracy = correct_steps / total_steps;

    // Use the final quality_score from simulation as a second signal
    let final_quality = run_log.last().map(|r| r.state.quality_score).unwrap_or(0.0);

    // Weighted score: 60% accuracy, 40% quality
    let score = accuracy * 0.60 + final_quality * 0.40;

    let threshold  = pass_threshold(level);
    let passed     = score >= threshold;
    let cycle_done = run_log.last().map(|r| r.cycle_done).unwrap_or(false);

    // Identify failed checkpoints (incorrect steps)
    let checkpoints: Vec<report::CheckpointResult> = run_log.iter().map(|r| {
        report::CheckpointResult {
            step:      r.step,
            action:    r.action.clone(),
            correct:   r.correct,
            message:   r.message.clone(),
            delta:     r.delta_quality,
        }
    }).collect();

    let critical_errors = run_log.iter().filter(|r| r.state.critical_error).count() as u32;

    AssessmentReport {
        craft,
        level,
        score,
        accuracy,
        final_quality,
        passed,
        cycle_complete: cycle_done,
        critical_errors,
        checkpoint_details:     checkpoints,
        unlock_real_practice:   passed && level == SimLevel::Diagnostic,
        unlock_next_level:      passed && level != SimLevel::Diagnostic,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use iqalto_simulation::{SimState, StepResult};

    fn make_run(correct_count: usize, wrong_count: usize, final_quality: f32) -> Vec<StepResult> {
        let state = SimState::new(CraftKind::Winery, SimLevel::Diagnostic);
        let mut log = Vec::new();
        for i in 0..(correct_count + wrong_count) {
            let correct = i < correct_count;
            let mut r = if correct {
                StepResult::ok("action", 0.0, "ok", state.clone())
            } else {
                StepResult::err("action", -0.1, "err", state.clone())
            };
            r.state.quality_score = final_quality;
            if i == correct_count + wrong_count - 1 { r.cycle_done = true; }
            log.push(r);
        }
        log
    }

…<truncated 24 more lines>…
```
### code `crates/simulation/src/lib.rs`
```
pub mod arteli;
pub mod bakery;
pub mod craft;
pub mod forge;
pub mod pottery;
pub mod state;
pub mod winery;

pub use craft::{CraftKind, SimLevel};
pub use state::{SimError, SimState, StepResult};
pub use arteli::{ArteliAction, ArteliRole, ArteliState, ArteliStepResult, merge_arteli_actions};

use serde::{Deserialize, Serialize};

/// Universal action envelope — wraps craft-specific actions.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "craft")]
pub enum Action {
    Winery  { action: winery::WineryAction },
    Bakery  { action: bakery::BakeryAction },
    Pottery { action: pottery::PotteryAction },
    Forge   { action: forge::ForgeAction },
}

/// Dispatch an action to the appropriate craft simulator.
pub fn run_step(state: SimState, action: Action) -> Result<StepResult, SimError> {
    match action {
        Action::Winery  { action } => winery::step_winery(state, action),
        Action::Bakery  { action } => bakery::step_bakery(state, action),
        Action::Pottery { action } => pottery::step_pottery(state, action),
        Action::Forge   { action } => forge::step_forge(state, action),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn winery_good_path() {
        use winery::WineryAction;
        let mut state = SimState::new(CraftKind::Winery, SimLevel::Procedural);
        state.grape_sugar_brix = Some(23.0);
        state.fermentation_temp_c = Some(18.0);

        let r = step_via_action(state, Action::Winery { action: WineryAction::HarvestGrapes });
        assert!(r.correct, "Good harvest should be correct");

        let r2 = step_via_action(r.state, Action::Winery { action: WineryAction::CrushAndTransfer });
        assert_eq!(r2.state.maceration_days, Some(0));
    }

    #[test]
    fn winery_overheated_fermentation() {
        use winery::WineryAction;
        let state = SimState::new(CraftKind::Winery, SimLevel::Procedural);
        let r = step_via_action(state, Action::Winery {
            action: WineryAction::AdjustTemp { delta_c: 15.0 }
        });
        assert!(!r.correct);
        assert!(r.state.fermentation_temp_c.unwrap() > 25.0);
    }

    #[test]
    fn bakery_cold_tone_fails() {
        use bakery::BakeryAction;
        let mut state = SimState::new(CraftKind::Bakery, SimLevel::Procedural);
        state.tone_temp_c = Some(100.0); // too cold
        state.dough_hydration_pct = Some(68.0);
        let r = step_via_action(state, Action::Bakery { action: BakeryAction::SlapOntoTone });
        assert!(!r.correct);
    }

    #[test]
    fn forge_overheated_metal_critical() {
        use forge::{ForgeAction};
        let state = SimState::new(CraftKind::Forge, SimLevel::Procedural);
        let r = step_via_action(state, Action::Forge {
            action: ForgeAction::HeatMetal { minutes: 20.0 }
        });
…<truncated 17 more lines>…
```
### code `crates/ffi/src/lib.rs`
```
/// Rustler NIF — Exposes iqalto-core to Elixir/Phoenix.
///
/// All functions communicate via JSON strings for simplicity.
/// Elixir side: see lms/lib/iqalto/core_nif.ex
use rustler::{Encoder, Env, NifResult, Term};
use iqalto_simulation::{Action, CraftKind, SimLevel, SimState, run_step};
use iqalto_assessment::assess;
use iqalto_simulation::arteli::{ArteliState, merge_arteli_actions};

rustler::init!("Elixir.Iqalto.CoreNif", [
    nif_step,
    nif_assess,
    nif_arteli_step,
    nif_new_state,
]);

/// Create a new SimState for the given craft and level.
/// Input:  craft (string), level (string)
/// Output: JSON-encoded SimState
#[rustler::nif]
fn nif_new_state(env: Env, craft_str: String, level_str: String) -> NifResult<Term> {
    let craft = parse_craft(&craft_str)?;
    let level = parse_level(&level_str)?;
    let state = SimState::new(craft, level);
    let json = serde_json::to_string(&state)
        .map_err(|e| rustler::Error::Term(Box::new(e.to_string())))?;
    Ok(json.encode(env))
}

/// Execute one simulation step.
/// Input:  state_json (SimState), action_json (Action)
/// Output: JSON-encoded StepResult or {:error, reason}
#[rustler::nif]
fn nif_step(env: Env, state_json: String, action_json: String) -> NifResult<Term> {
    let state: SimState = serde_json::from_str(&state_json)
        .map_err(|e| rustler::Error::Term(Box::new(format!("bad state: {}", e))))?;
    let action: Action = serde_json::from_str(&action_json)
        .map_err(|e| rustler::Error::Term(Box::new(format!("bad action: {}", e))))?;

    match run_step(state, action) {
        Ok(result) => {
            let json = serde_json::to_string(&result)
                .map_err(|e| rustler::Error::Term(Box::new(e.to_string())))?;
            Ok(json.encode(env))
        }
        Err(e) => Err(rustler::Error::Term(Box::new(e.to_string()))),
    }
}

/// Assess a completed simulation run.
/// Input:  craft (string), level (string), run_log_json (Vec<StepResult>)
/// Output: JSON-encoded AssessmentReport
#[rustler::nif]
fn nif_assess(env: Env, craft_str: String, level_str: String, run_log_json: String) -> NifResult<Term> {
    use iqalto_simulation::StepResult;

    let craft = parse_craft(&craft_str)?;
    let level = parse_level(&level_str)?;
    let run_log: Vec<StepResult> = serde_json::from_str(&run_log_json)
        .map_err(|e| rustler::Error::Term(Box::new(format!("bad run_log: {}", e))))?;

    let report = assess(craft, level, &run_log);
    let json = serde_json::to_string(&report)
        .map_err(|e| rustler::Error::Term(Box::new(e.to_string())))?;
    Ok(json.encode(env))
}

/// Resolve one Arteli (cooperative) round.
/// Input:  arteli_state_json (ArteliState)
/// Output: JSON-encoded ArteliStepResult
#[rustler::nif]
fn nif_arteli_step(env: Env, arteli_json: String) -> NifResult<Term> {
    let arteli: ArteliState = serde_json::from_str(&arteli_json)
        .map_err(|e| rustler::Error::Term(Box::new(format!("bad arteli state: {}", e))))?;
    let result = merge_arteli_actions(arteli);
    let json = serde_json::to_string(&result)
        .map_err(|e| rustler::Error::Term(Box::new(e.to_string())))?;
    Ok(json.encode(env))
}

…<truncated 18 more lines>…
```
## Code volume
| ext | files | bytes |
|---|---|---|
| .rs | 14 | 69331 |