//! HCM Chapter 12: Basic Freeway and Multilane Highway Segments,
//! including the managed lanes methodology.
//!
//! Two supplemental chapters extend the basic segment methodology and live here rather than
//! in modules of their own: `mixed_flow` is the Chapter 26 single-grade mixed-flow model, and
//! `composite_grade` is the Chapter 25 chaining of it across consecutive grades. Both analyse
//! basic segments, and Chapter 26 is itself titled "Basic Freeway and Highway Segments:
//! Supplemental".

pub mod basicfreeways;
pub mod composite_grade;
pub mod managed_lanes;
pub mod mixed_flow;

pub use basicfreeways::*;
pub use composite_grade::*;
pub use managed_lanes::*;
pub use mixed_flow::*;

pub const CHAPTER: u8 = 12;
pub const TITLE: &str = "Basic Freeway and Multilane Highway Segments";
