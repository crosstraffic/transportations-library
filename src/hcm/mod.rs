//! Highway Capacity Manual (HCM 7th Edition) methodologies, organized one
//! module per HCM chapter, plus shared types (`common`) and cross-cutting
//! geometry/topology support (`support`).

pub mod common;
pub mod support;

pub mod chapter12;
pub mod chapter13;
pub mod chapter14;
pub mod chapter15;

pub const HCM_VERSION: &str = "7th Edition";

// Backward-compatible module paths from the pre-chapter layout.
// Deprecated: import via `hcm::chapterNN::*` or `hcm::support::*` instead.
pub use chapter12::basicfreeways;
pub use chapter12::managed_lanes;
pub use chapter13::weaving;
pub use chapter14::merge_diverge;
pub use chapter15::twolanehighways;
pub use support::{constraints, geometric, topology, traffic_flow};

pub mod adjustment_factors {
    pub use crate::hcm::common::adjustment_factors::*;
}
