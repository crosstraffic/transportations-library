//! HCM Chapter 10: Freeway Facilities Core Methodology.
//!
//! Analyzes a directional freeway facility — an ordered set of basic,
//! weaving, merge, diverge, and overlapping-ramp segments — over multiple
//! consecutive 15-min analysis periods, including the Chapter 25
//! oversaturated (queue-tracking) time-step engine, work zone CAF/SAF
//! models, and facility-level LOS per Exhibit 10-6.
//!
//! Out of scope in this pass (documented deferrals): managed-lane
//! facilities (Steps A-9/A-13/A-14), the Chapter 25 planning-level method,
//! and the Chapter 25 Section 5 special work zone configuration tables
//! (Exhibits 25-8 through 25-14).

pub mod exhibits;
pub mod freeway_facilities;
pub mod oversaturated;

#[cfg(test)]
mod tests;

pub use exhibits::{los_freeway_facility, WorkZone};
pub use freeway_facilities::{
    segment_ramp_section, FacilitySegment, FreewayFacility, PeriodPerformance, SegmentType,
    Terrain, CHAPTER,
};
pub use oversaturated::{OversatPeriodInput, OversatPeriodResult, OversaturatedEngine};
