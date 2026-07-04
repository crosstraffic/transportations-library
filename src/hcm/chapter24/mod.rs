//! HCM Chapter 24: Off-Street Pedestrian and Bicycle Facilities.

pub mod offstreet_pedbike;

pub use offstreet_pedbike::*;

pub const CHAPTER: u8 = 24;
pub const TITLE: &str = "Off-Street Pedestrian and Bicycle Facilities";

#[cfg(test)]
mod tests;
