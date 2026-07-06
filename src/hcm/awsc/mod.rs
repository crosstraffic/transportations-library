//! HCM Chapter 21: All-Way STOP-Controlled Intersections.

pub mod awsc;

pub use awsc::*;

pub const CHAPTER: u8 = 21;
pub const TITLE: &str = "All-Way STOP-Controlled Intersections";

#[cfg(test)]
mod tests;
