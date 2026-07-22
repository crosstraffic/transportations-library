//! HCM Chapter 12: Basic Freeway and Multilane Highway Segments,
//! including the managed lanes methodology.

pub mod basicfreeways;
pub mod managed_lanes;

pub use basicfreeways::*;
pub use managed_lanes::*;

pub const CHAPTER: u8 = 12;
pub const TITLE: &str = "Basic Freeway and Multilane Highway Segments";
