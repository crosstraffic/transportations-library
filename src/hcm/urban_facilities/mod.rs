//! HCM Chapter 16: Urban Street Facilities.
//!
//! Motorized vehicle methodology (HCM 7th Edition, Chapter 16, Section 3,
//! EPUB `112_Ch16_03.xhtml`): a facility is an ordered sequence of Chapter
//! 18 urban street segments evaluated for one direction of travel; the
//! facility measures are aggregations of the segment measures:
//!
//! * Step 1 — facility base free-flow speed (Equation 16-2,
//!   length-weighted harmonic mean of the segment base free-flow speeds);
//! * Step 2 — facility travel speed (Equation 16-3, length-weighted
//!   harmonic mean of the segment travel speeds, i.e., total length over
//!   total travel time);
//! * Step 3 — facility spatial stop rate (Equation 16-4, length-weighted
//!   arithmetic mean of the segment spatial stop rates) and, optionally,
//!   the facility automobile traveler perception score (the Chapter 18
//!   Step 10 equations with `H_F` substituted for `H_seg`);
//! * Step 4 — motorized vehicle LOS (Exhibit 16-3 travel-speed thresholds
//!   by base free-flow speed, with LOS F when the critical through-movement
//!   volume-to-capacity ratio at any boundary intersection exceeds 1.0),
//!   plus the poorest-performing-segment LOS the chapter directs analysts
//!   to report as context.
//!
//! Equation 16-1 (capacity of the uncontrolled through movement at a TWSC
//! boundary intersection) is identical to Chapter 18's Equation 18-2 and is
//! re-exported from [`crate::hcm::urban_segments`].
//!
//! Deferred (documented): the Chapter 29, Section 3 sustained spillback
//! evaluation procedure (an iterative capacity-constraint loop over the
//! Chapter 18/19 engines). A spillback *check* hook is provided
//! ([`urban_facilities::SpillbackCheckInput`]) that flags segments whose
//! predicted back-of-queue exceeds the available queue storage. The
//! pedestrian, bicycle, and transit facility methodologies (Sections 4-6)
//! are out of scope.

pub mod urban_facilities;

#[cfg(test)]
mod tests;

pub use urban_facilities::*;

pub const CHAPTER: u8 = 16;
pub const TITLE: &str = "Urban Street Facilities";
