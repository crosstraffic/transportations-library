//! HCM Chapter 13: Freeway Weaving Segments.
//!
//! Two editions of this chapter are implemented. [`weaving`] holds the 7th Edition methodology and
//! [`v7_1`] the Edition 7.1 replacement chapter (November 2025, NCHRP Research Report 1038). A
//! [`WeavingSegment`](weaving::WeavingSegment) carries the edition it should be analyzed under; see
//! [`crate::hcm::common::HcmVersion`].

pub mod v7_1;
pub mod weaving;

pub use weaving::*;

pub const CHAPTER: u8 = 13;
pub const TITLE: &str = "Freeway Weaving Segments";
