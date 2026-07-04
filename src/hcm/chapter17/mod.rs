//! HCM Chapter 17: Urban Street Reliability and ATDM.
//!
//! Evaluates the travel time reliability of an urban street facility over
//! a reliability reporting period (typically weekdays for one year) by
//! wrapping the Chapter 16 facility methodology in a scenario loop
//! (Chapter 17, Section 3, with the computational details of Chapter 29,
//! Section 2): seeded Monte Carlo weather event generation (Equations
//! 29-1 through 29-12), systematic demand variation (Exhibits 17-5
//! through 17-8), seeded Monte Carlo incident generation (Equations 29-13
//! through 29-24), and per-analysis-period scenario datasets (Equations
//! 29-25 through 29-36). Each scenario is evaluated with the Chapter
//! 16/18 engine; facility travel times feed the shared
//! [`crate::hcm::common::reliability`] TTI distribution, from which the
//! Chapter 17 performance measures (mean/50th/80th/95th percentile TTI,
//! PTI, and the urban reliability rating at TTI < 2.5) are computed.
//!
//! ATDM strategies, work zones, and special events are supported at the
//! alternative-dataset input-hook level ([`AtdmStrategy`]: scheduled
//! demand / saturation flow / green time / free-flow speed / crash
//! frequency adjustments); the Chapter 37 strategy-specific models are
//! deferred. Other documented deferrals are listed in
//! [`urban_reliability`].

pub mod exhibits;
pub mod urban_reliability;

#[cfg(test)]
mod tests;

pub use exhibits::{
    additional_delay_s, adjusted_base_ffs, crash_proportion, default_incident_duration_min,
    exhibit_17_10_clearance_time_min, exhibit_17_5_hour_of_day_ratio,
    exhibit_17_6_day_of_week_ratio, exhibit_17_7_month_of_year_ratio, exhibit_17_9_cfaf,
    exhibit_17_9_response_time_min, exhibit_29_5_extra_lt_headway_s, incident_joint_proportions,
    incident_sat_flow_factor, incident_severity_coefficient, weather_ffs_factor,
    weather_sat_flow_factor, FunctionalClass, IncidentSeverity, IncidentType, LaneLocation,
    StreetLocation, WeatherCondition, CHAPTER, DEFAULT_INCIDENT_DETECTION_MIN,
    URBAN_RELIABILITY_RATING_TTI_THRESHOLD,
};
pub use urban_reliability::{
    equivalent_crash_frequency_dry, gamma_inverse, generate_weather_events, normal_inverse,
    weather_at, weather_condition_hours, AtdmStrategy, BoundarySignal, IncidentConfig,
    MonthlyWeather, UrbanIncident, UrbanReliability, UrbanReliabilityConfig,
    UrbanReliabilityResults, UrbanScenario, UrbanScenarioResult, WeatherEvent,
};

pub const TITLE: &str = "Urban Street Reliability and ATDM";
