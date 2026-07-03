//! HCM Chapter 20: Two-Way STOP-Controlled Intersections.

pub mod twsc;

pub use twsc::*;

pub const CHAPTER: u8 = 20;
pub const TITLE: &str = "Two-Way STOP-Controlled Intersections";

#[cfg(test)]
mod tests;
