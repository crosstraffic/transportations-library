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
//! Lane groups are composed rather than enumerated: an
//! `InterchangeMovement` is an approach, a position in the interchange
//! skeleton, and a turn, and the routing and demand composition for every
//! form are derived from that form's own Chapter 34 worksheet
//! (`od_journey`). All nine forms of Exhibit 23-17 therefore run the
//! pipeline, but only three are validated against published numbers: the
//! diamond (Chapter 34 Example Problems 1, 3, and 4), the DDI (Example
//! Problems 5 and 6), and the Parclo A-2Q (Example Problem 2). The Parclo
//! A-4Q, AB-2Q, AB-4Q, B-2Q, B-4Q, and the SPUI are
//! **structurally supported and unvalidated** — Chapter 34 publishes no
//! Part B worked example that would pin them, and the only tests they
//! carry are the synthetic end-to-end smoke tests in `tests.rs`.
//!
//! Milestone 1 covers signalized interchange ramp terminals and DDIs
//! (`ramp_terminals`). Interchanges with roundabout ramp terminals are
//! supported through the Exhibit 34-161 O-D composition table and the
//! Exhibit 23-14 LOS criteria (`los_roundabout_interchange_od`), with the
//! roundabout approaches themselves evaluated by the Chapter 22 engine.
//!
//! Milestone 2 covers the Part C alternative intersections — RCUT, MUT, and
//! DLT (`alternative_intersections`) — with the Exhibit 23-47 10-step
//! framework: O-D → junction traversal (Exhibits 23-48/23-49/23-50),
//! extra distance travel time (Equations 23-58/23-59), experienced travel
//! time and its approach/intersection aggregation (Equations 23-60 through
//! 23-62), the Exhibit 23-13 LOS table, and the DLT offset (Equations 23-63
//! through 23-68) and volume-weighted control delay (Equation 23-69).
//! STOP-controlled junction delays are computed from the Chapter 20
//! gap-acceptance primitives; signalized junction delays enter from the
//! Chapter 19 engine.

pub mod alternative_intersections;
pub mod exhibits;
pub mod ramp_terminals;

#[cfg(test)]
mod tests;

pub use alternative_intersections::*;
pub use exhibits::*;
pub use ramp_terminals::*;

pub const CHAPTER: u8 = 23;
pub const TITLE: &str = "Ramp Terminals and Alternative Intersections";
