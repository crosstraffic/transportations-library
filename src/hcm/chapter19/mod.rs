//! HCM Chapter 19: Signalized Intersections.
//!
//! Motorized vehicle methodology (HCM 7th Edition, Chapter 19, Section 3,
//! with the Section 4 extensions and the supplemental procedures of
//! Chapter 31, Signalized Intersections: Supplemental).
//!
//! Milestone 1 covers pretimed and coordinated operation (fixed, known
//! signal timing). Milestone 2 adds the actuated phase-duration estimation
//! procedure (Chapter 31, Section 2, Equations 31-1 through 31-45; see
//! [`actuated`]), the left-turn arrival–departure polygon first-term back of
//! queue (Chapter 31, Section 4, Exhibits 31-26 through 31-31), and the
//! right-turn-on-red volume estimate (Chapter 31, Section 8). Fixed timing
//! remains the default analysis path; the actuated estimator is a separate
//! entry point ([`signalized::SignalizedIntersection::estimate_actuated_timings`]).
//! The pedestrian/bicycle LOS methodologies and multi-period analysis are
//! deferred to a later milestone.

pub mod actuated;
pub mod exhibits;
pub mod signalized;

#[cfg(test)]
mod tests;

pub use actuated::*;
pub use exhibits::*;
pub use signalized::*;

pub const CHAPTER: u8 = 19;
pub const TITLE: &str = "Signalized Intersections";
