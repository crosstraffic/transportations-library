//! HCM Chapter 19: Signalized Intersections.
//!
//! Motorized vehicle methodology (HCM 7th Edition, Chapter 19, Section 3,
//! with the Section 4 extensions and the supplemental procedures of
//! Chapter 31, Signalized Intersections: Supplemental).
//!
//! Milestone 1 covers pretimed and coordinated operation (fixed, known
//! signal timing). The actuated phase-duration estimation loop (Chapter 31,
//! Section 2), the full left-turn arrival–departure polygon family for
//! percentile back-of-queue, and the pedestrian/bicycle LOS methodologies
//! are deferred to a later milestone; the data structures already carry the
//! fields (e.g., `max_green_s`, `passage_time_s`) needed to add them without
//! breaking changes.

pub mod exhibits;
pub mod signalized;

#[cfg(test)]
mod tests;

pub use exhibits::*;
pub use signalized::*;

pub const CHAPTER: u8 = 19;
pub const TITLE: &str = "Signalized Intersections";
