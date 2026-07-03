//! HCM Chapter 10: Freeway Facilities Core Methodology.
//!
//! Analyzes a directional freeway facility — an ordered set of basic,
//! weaving, merge, diverge, and overlapping-ramp segments — over multiple
//! consecutive 15-min analysis periods, including the Chapter 25
//! oversaturated (queue-tracking) time-step engine, work zone CAF/SAF
//! models, and facility-level LOS per Exhibit 10-6.
//!
//! This module also covers the managed-lane facility extension (Steps
//! A-9/A-13/A-14/A-17; [`managed_lanes`]) and the Chapter 25 Section 6
//! planning-level method ([`planning`]).
//!
//! Out of scope in this pass (documented deferrals): the Chapter 25 Section 4
//! oversaturated managed-lane **vertical-queue** delay accounting (Equations
//! 25-35/25-36; see [`managed_lanes`]) and the Chapter 25 Section 5 special
//! work zone configuration tables (Exhibits 25-8 through 25-14).

pub mod exhibits;
pub mod freeway_facilities;
pub mod managed_lanes;
pub mod oversaturated;
pub mod planning;

#[cfg(test)]
mod tests;

pub use exhibits::{los_freeway_facility, WorkZone};
pub use freeway_facilities::{
    segment_ramp_section, FacilitySegment, FreewayFacility, PeriodPerformance, SegmentType,
    Terrain, CHAPTER,
};
pub use managed_lanes::{
    cross_weave_caf, cross_weave_crf, CrossWeave, LaneGroupPerformance, ManagedLaneFacility,
    MlSegmentInput,
};
pub use oversaturated::{OversatPeriodInput, OversatPeriodResult, OversaturatedEngine};
pub use planning::{
    PlanningFacility, PlanningFacilityResult, PlanningSection, PlanningSectionResult,
    PlanningSectionType,
};
