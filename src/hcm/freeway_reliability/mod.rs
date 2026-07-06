//! HCM Chapter 11: Freeway Reliability Analysis.
//!
//! Evaluates a freeway facility's travel time reliability over a
//! multiday/multimonth reliability reporting period (RRP) by wrapping the
//! Chapter 10 core methodology in a scenario loop (Steps B-1 through
//! B-13): the Chapter 25 Section 9 scenario generator combines
//! day-of-week × month-of-year demand variability (deterministic),
//! scheduled work zones (deterministic), and weather and incident events
//! (deterministic event counts via the delta-rounding equations, seeded
//! stochastic assignment); each scenario is evaluated with the core
//! methodology, and the resulting facility travel times form a weighted
//! TTI distribution from which the Chapter 11 reliability performance
//! measures (TTI percentiles/PTI, misery index, reliability rating,
//! semi-standard deviation, failure/on-time measures) are computed via
//! the shared [`crate::hcm::common::reliability`] module.
//!
//! Also includes the Chapter 11 planning-level reliability method
//! (Equations 11-1 through 11-5) in [`exhibits`].
//!
//! Out of scope in this pass (documented deferrals): managed lane
//! reliability, the Section 4 ATDM strategy assessment (Steps C-1 through
//! C-9), and the Chapter 25 reliability calibration methodology.

pub mod exhibits;
pub mod reliability;
pub mod scenario_generation;

#[cfg(test)]
mod tests;

pub use exhibits::{
    hers_crash_rate, incident_caf_per_open_lane, incident_caf_total, planning_pt45,
    planning_tti_95, planning_tti_mean, weather_caf, weather_saf, IncidentDurationParams,
    IncidentSeverity, WeatherType, CHAPTER, DEFAULT_INCIDENT_SEVERITY_DISTRIBUTION,
    DEFAULT_INCIDENT_TO_CRASH_RATIO, RURAL_DEMAND_RATIOS, URBAN_DEMAND_RATIOS,
};
pub use reliability::{ReliabilityAnalysis, ScenarioResult};
pub use scenario_generation::{
    generate_scenarios, FreewayScenario, IncidentAssignment, IncidentInputs, Prng,
    ScenarioGenerationConfig, ScenarioSet, SeedStatistics, SpecialEvent, WeatherEventAssignment,
    WeatherInputs, Weekday, WorkZoneEvent,
};
