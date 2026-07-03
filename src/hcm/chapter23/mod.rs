//! HCM Chapter 23: Ramp Terminals and Alternative Intersections.
//!
//! Part B methodology (HCM 7th Edition, Chapter 23, Sections B.1–B.5):
//! final design and operational analysis of signalized interchange ramp
//! terminals — diamond, parclo, SPUI, and diverging diamond (DDI) forms —
//! including O-D / turning-movement conversion (Chapter 34 worksheets),
//! interchange-specific saturation flow adjustments, lost time due to
//! downstream internal queues and demand starvation, DDI overlap phasing
//! and YIELD-controlled turns, O-D experienced travel time, and the
//! Exhibit 23-10 LOS determination.
//!
//! Milestone 1 covers signalized interchange ramp terminals and DDIs.
//! Interchanges with roundabout ramp terminals are supported through the
//! Exhibit 34-161 O-D composition table and the Exhibit 23-14 LOS
//! criteria (`los_roundabout_interchange_od`), with the roundabout
//! approaches themselves evaluated by the Chapter 22 engine; the
//! Part C alternative intersections (RCUT / MUT / DLT) are milestone 2 —
//! the O-D framework (`OdDemands`, `OdMovement`, per-O-D path
//! aggregation, and the Exhibit 23-13 LOS table) is already shared.

pub mod exhibits;
pub mod ramp_terminals;

#[cfg(test)]
mod tests;

pub use exhibits::*;
pub use ramp_terminals::*;

pub const CHAPTER: u8 = 23;
pub const TITLE: &str = "Ramp Terminals and Alternative Intersections";
