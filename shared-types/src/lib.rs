//! LongevityCommon shared types.
//!
//! Closes audit P0 #6 (cross-project parameter inconsistency).  Single source
//! of truth for canonical Ze constants and a parameter registry that every
//! LC subproject can reference instead of redefining its own.
//!
//! Authoritative document: `~/Desktop/LongevityCommon/PARAMETERS.md §1`.
//!
//! # Quick start
//!
//! ```
//! use lc_shared_types::ze;
//! assert!((ze::V_STAR_ACTIVE_ARTICLE - (-0.08738)).abs() < 1e-6);
//! assert!((ze::python_to_article(0.45631) + 0.08738).abs() < 1e-6);
//! ```

pub mod parameter;
pub mod units;
pub mod ze;

pub use parameter::{Parameter, ParameterRegistry, ParameterStatus};
pub use units::Unit;
