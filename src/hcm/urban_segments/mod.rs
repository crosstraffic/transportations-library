//! HCM Chapter 18: Urban Street Segments.
//!
//! Motorized vehicle methodology (HCM 7th Edition, Chapter 18, Section 3),
//! with the supplemental material of Chapter 30, Urban Street Segments:
//! Supplemental.
//!
//! The per-direction segment evaluation uses analyst-supplied
//! boundary-intersection performance inputs (through control delay,
//! capacity, and stop-rate components are "HCM method output" inputs per
//! Exhibit 18-5 — obtain them from the Chapter 19/20/21/22 engines in this
//! crate). The Chapter 30, Section 4 access-point delay procedure
//! ([`access_point_delay`], Equations 30-31 through 30-68) and the
//! Section 3 platoon-dispersion primitives ([`platoon_dispersion`],
//! Equations 30-9 through 30-13) are implemented and wired into Steps 2 and
//! 3: when access-point geometry/volumes are supplied the computed
//! `d_ap = d_ap,l + d_ap,r` replaces the Exhibit 18-13 planning estimate,
//! and when upstream discharge-flow profiles are supplied the computed
//! proportion arriving during green replaces the uniform / platoon-ratio
//! assumption. Deferred: the Chapter 30, Section 2 demand adjustment
//! (origin–destination, volume balance, spillback checks) and the
//! coordinated-system convergence loop that would drive the discharge
//! profiles directly from the Chapter 19 timing (so reproducing Example
//! Problem 1's computed `P = 0.493` from the raw signal is deferred — see
//! `docs/hcm/VERIFICATION.md`). The pedestrian ([`pedestrian`], Section 4),
//! bicycle ([`bicycle`], Section 5), and transit ([`transit`], Section 6)
//! methodologies are implemented as self-contained segment LOS models and
//! reproduce Chapter 30 Example Problems 2, 3, and 4 respectively.

pub mod access_point_delay;
pub mod bicycle;
pub mod exhibits;
pub mod pedestrian;
pub mod platoon_dispersion;
pub mod transit;
pub mod urban_segments;

#[cfg(test)]
mod tests;

pub use access_point_delay::*;
pub use bicycle::*;
pub use exhibits::*;
pub use pedestrian::*;
pub use platoon_dispersion::*;
pub use transit::*;
pub use urban_segments::*;

pub const CHAPTER: u8 = 18;
pub const TITLE: &str = "Urban Street Segments";
