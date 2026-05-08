//! Lightweight SI/biological units enum used in `Parameter`.  This is not a
//! compile-time dimensional type system (we don't pull `uom` for MVP); it is a
//! runtime-checked tag so PARAMETERS.md serialization stays consistent across
//! subprojects.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum Unit {
    /// Hertz (frequency).
    Hz,
    /// Years (time).
    Year,
    /// Days (time).
    Day,
    /// Seconds (time).
    Second,
    /// Population doublings (cell divisions, dimensionless).
    Pd,
    /// Damage / unitless.
    Unitless,
    /// Base pairs (telomere length).
    Bp,
    /// Cell-divisions per year (rate).
    DivisionPerYear,
    /// Damage units per division.
    DamagePerDivision,
    /// Damage units per year.
    DamagePerYear,
    /// Other / project-specific (free-form string in `tag`).
    Other,
}

impl Unit {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Hz => "Hz",
            Self::Year => "year",
            Self::Day => "day",
            Self::Second => "s",
            Self::Pd => "PD",
            Self::Unitless => "1",
            Self::Bp => "bp",
            Self::DivisionPerYear => "1/year",
            Self::DamagePerDivision => "damage/division",
            Self::DamagePerYear => "damage/year",
            Self::Other => "other",
        }
    }
}
