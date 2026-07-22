//! Highway Capacity Manual (HCM 7th Edition) methodologies, organized one
//! module per HCM chapter, plus shared types (`common`) and cross-cutting
//! geometry/topology helpers (`utils`).

pub mod common;
pub mod utils;

pub mod freeway_facilities; // HCM Chapter 10
pub mod freeway_reliability; // HCM Chapter 11
pub mod basicfreeways; // HCM Chapter 12
pub mod weaving; // HCM Chapter 13
pub mod merge_diverge; // HCM Chapter 14
pub mod twolanehighways; // HCM Chapter 15
pub mod urban_facilities; // HCM Chapter 16
pub mod urban_reliability; // HCM Chapter 17
pub mod urban_segments; // HCM Chapter 18
pub mod signalized; // HCM Chapter 19
pub mod twsc; // HCM Chapter 20
pub mod awsc; // HCM Chapter 21
pub mod roundabouts; // HCM Chapter 22
pub mod ramp_terminals; // HCM Chapter 23
pub mod offstreet_pedbike; // HCM Chapter 24

pub const HCM_VERSION: &str = "7th Edition";

// Chapter-number module aliases, kept for external path stability: code
// written against `hcm::chapterNN::...` keeps compiling unchanged. New code
// should import via the topic module names declared above instead.
pub use freeway_facilities as chapter10;
pub use freeway_reliability as chapter11;
pub use basicfreeways as chapter12;
pub use weaving as chapter13;
pub use merge_diverge as chapter14;
pub use twolanehighways as chapter15;
pub use urban_facilities as chapter16;
pub use urban_reliability as chapter17;
pub use urban_segments as chapter18;
pub use signalized as chapter19;
pub use twsc as chapter20;
pub use awsc as chapter21;
pub use roundabouts as chapter22;
pub use ramp_terminals as chapter23;
pub use offstreet_pedbike as chapter24;

// Backward-compatible module paths from the pre-chapter layout.
// Deprecated: import via the topic modules above or `hcm::utils::*` instead.
pub use basicfreeways::managed_lanes;
pub use utils::{constraints, geometric, topology, traffic_flow};

pub mod adjustment_factors {
    pub use crate::hcm::common::adjustment_factors::*;
}
