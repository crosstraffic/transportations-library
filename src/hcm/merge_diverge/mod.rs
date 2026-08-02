//! HCM Chapter 14: Freeway Merge and Diverge Segments.
//!
//! Two editions of this chapter are implemented. [`merge_diverge`] holds the 7th Edition
//! methodology and [`v7_1`] the Edition 7.1 replacement chapter (November 2025, NCHRP Research
//! Report 1038). A [`RampSegment`](merge_diverge::RampSegment) carries the edition it should be
//! analyzed under; see [`crate::hcm::common::HcmVersion`].

pub mod merge_diverge;
pub mod v7_1;

pub use merge_diverge::*;

pub const CHAPTER: u8 = 14;
pub const TITLE: &str = "Freeway Merge and Diverge Segments";
