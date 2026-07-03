//! HCM Chapter 18: Urban Street Segments.
//!
//! Motorized vehicle methodology (HCM 7th Edition, Chapter 18, Section 3),
//! with the supplemental material of Chapter 30, Urban Street Segments:
//! Supplemental.
//!
//! Milestone 1 covers the per-direction segment evaluation with
//! analyst-supplied boundary-intersection performance inputs (through
//! control delay, capacity, and stop-rate components are "HCM method
//! output" inputs per Exhibit 18-5 — obtain them from the Chapter 19/20/21/
//! 22 engines in this crate). Deferred to milestone 2: the Chapter 30,
//! Section 2 demand adjustment (origin–destination, volume balance,
//! spillback checks), the Section 3 platoon-dispersion arrival-profile and
//! coordinated-system convergence loop (Steps 3–4), and the Section 4
//! access-point delay procedure (the Exhibit 18-13 planning estimate and a
//! per-access-point input hook are provided instead). The pedestrian,
//! bicycle, and transit methodologies of Chapter 18 are out of scope.

pub mod exhibits;
pub mod urban_segments;

#[cfg(test)]
mod tests;

pub use exhibits::*;
pub use urban_segments::*;

pub const CHAPTER: u8 = 18;
pub const TITLE: &str = "Urban Street Segments";
