//! HCM Chapter 20: Two-Way STOP-Controlled Intersections.
//!
//! [`twsc`] carries the motorized vehicle methodology (Section 3) and the
//! pedestrian-impedance extension of it (Section 4). [`pedestrian`] carries the
//! separate pedestrian mode of Section 5, in which the pedestrian crossing the
//! major street is the subject and the service measure is the proportion of
//! pedestrians dissatisfied with the crossing.

pub mod computed_pb;
pub mod pedestrian;
pub mod twsc;

pub use computed_pb::*;
pub use pedestrian::*;
pub use twsc::*;

pub const CHAPTER: u8 = 20;
pub const TITLE: &str = "Two-Way STOP-Controlled Intersections";

#[cfg(test)]
mod tests;
