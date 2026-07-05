//! Exhibit lookups and adjustment-factor equations for HCM Chapter 17
//! (Urban Street Reliability and ATDM).
//!
//! All values transcribed from the HCM 7th Edition EPUB:
//! * Exhibits 17-5 through 17-12 — `121_Ch17_03.xhtml`
//! * Equations 29-25 through 29-28 and 29-34 through 29-36, and Exhibit
//!   29-5 — `227_Ch29_02.xhtml` (Chapter 29, Urban Street Facilities:
//!   Supplemental, Section 2, Scenario Generation Procedure)

use serde::{Deserialize, Serialize};

pub const CHAPTER: u8 = 17;

/// TTI threshold for the urban street reliability rating: "the percentage
/// of vehicle miles traveled on the facility associated with a TTI less
/// than 2.50. This threshold approximates the point beyond which urban
/// street facility travel times become much more variable" (Chapter 17,
/// Section 3, Measures Describing Reliability). Note the freeway threshold
/// (Chapter 11) is 1.33.
pub const URBAN_RELIABILITY_RATING_TTI_THRESHOLD: f64 = 2.5;

// ═══════════════════════════════════════════════════════════════════════════════
// Functional class / weather / severity enumerations
// ═══════════════════════════════════════════════════════════════════════════════

/// Roadway functional class for the Exhibit 17-5/17-7 default demand
/// ratios.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FunctionalClass {
    Expressway,
    UrbanPrincipalArterial,
    UrbanMinorArterial,
}

/// Weather condition categories of the urban street reliability
/// methodology (Chapter 29, Section 2): dry; rainfall; wet pavement, not
/// raining; snowfall; snow or ice on pavement, not snowing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WeatherCondition {
    Dry,
    Rainfall,
    WetPavement,
    Snowfall,
    SnowOrIceOnPavement,
}

/// Incident street-location category (Chapter 17, Incident Data: "For the
/// purposes of reliability analysis ... each crash [is categorized] in
/// accordance with the location of its occurrence").
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StreetLocation {
    Segment,
    Intersection,
}

/// Incident lane location (shoulder, one lane, two or more lanes).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LaneLocation {
    OneLane,
    TwoPlusLanes,
    Shoulder,
}

/// Incident event type and severity. Crashes are fatal/injury (FI) or
/// property damage only (PDO); noncrash incidents are breakdowns or other
/// (e.g., debris).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IncidentSeverity {
    CrashFatalInjury,
    CrashPropertyDamage,
    NoncrashBreakdown,
    NoncrashOther,
}

impl IncidentSeverity {
    pub fn is_crash(self) -> bool {
        matches!(
            self,
            IncidentSeverity::CrashFatalInjury | IncidentSeverity::CrashPropertyDamage
        )
    }
}

/// One of the 12 incident types considered each hour at each street
/// location (Chapter 29, Section 2, Step 3 of the traffic incident
/// procedure).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct IncidentType {
    pub lanes: LaneLocation,
    pub severity: IncidentSeverity,
}

/// The 12 incident types in the Exhibit 17-11 row order.
pub const INCIDENT_TYPES: [IncidentType; 12] = [
    IncidentType { lanes: LaneLocation::OneLane, severity: IncidentSeverity::CrashFatalInjury },
    IncidentType { lanes: LaneLocation::OneLane, severity: IncidentSeverity::CrashPropertyDamage },
    IncidentType { lanes: LaneLocation::TwoPlusLanes, severity: IncidentSeverity::CrashFatalInjury },
    IncidentType {
        lanes: LaneLocation::TwoPlusLanes,
        severity: IncidentSeverity::CrashPropertyDamage,
    },
    IncidentType { lanes: LaneLocation::Shoulder, severity: IncidentSeverity::CrashFatalInjury },
    IncidentType { lanes: LaneLocation::Shoulder, severity: IncidentSeverity::CrashPropertyDamage },
    IncidentType { lanes: LaneLocation::OneLane, severity: IncidentSeverity::NoncrashBreakdown },
    IncidentType { lanes: LaneLocation::OneLane, severity: IncidentSeverity::NoncrashOther },
    IncidentType { lanes: LaneLocation::TwoPlusLanes, severity: IncidentSeverity::NoncrashBreakdown },
    IncidentType { lanes: LaneLocation::TwoPlusLanes, severity: IncidentSeverity::NoncrashOther },
    IncidentType { lanes: LaneLocation::Shoulder, severity: IncidentSeverity::NoncrashBreakdown },
    IncidentType { lanes: LaneLocation::Shoulder, severity: IncidentSeverity::NoncrashOther },
];

// ═══════════════════════════════════════════════════════════════════════════════
// Exhibit 17-5: Default Hour-of-Day Demand Ratios (ADT/AADT)
// ═══════════════════════════════════════════════════════════════════════════════

/// Exhibit 17-5 hour-of-day demand ratios, weekday columns. Rows are the
/// hour starting midnight (index 0) through 11 p.m. (index 23); columns
/// are [expressway, principal arterial, minor arterial].
const EXHIBIT_17_5_WEEKDAY: [[f64; 3]; 24] = [
    [0.010, 0.010, 0.010],
    [0.006, 0.006, 0.006],
    [0.004, 0.005, 0.004],
    [0.004, 0.005, 0.002],
    [0.007, 0.009, 0.002],
    [0.025, 0.030, 0.007],
    [0.058, 0.054, 0.023],
    [0.077, 0.071, 0.067],
    [0.053, 0.058, 0.066],
    [0.037, 0.047, 0.054],
    [0.037, 0.046, 0.051],
    [0.042, 0.050, 0.056],
    [0.045, 0.053, 0.071],
    [0.045, 0.054, 0.066],
    [0.057, 0.063, 0.060],
    [0.073, 0.069, 0.062],
    [0.087, 0.072, 0.063],
    [0.090, 0.077, 0.075],
    [0.068, 0.062, 0.070],
    [0.049, 0.044, 0.053],
    [0.040, 0.035, 0.044],
    [0.037, 0.033, 0.035],
    [0.029, 0.026, 0.033],
    [0.019, 0.021, 0.019],
];

/// Exhibit 17-5 hour-of-day demand ratios, weekend columns (same layout).
const EXHIBIT_17_5_WEEKEND: [[f64; 3]; 24] = [
    [0.023, 0.023, 0.028],
    [0.015, 0.014, 0.023],
    [0.008, 0.010, 0.021],
    [0.005, 0.006, 0.008],
    [0.005, 0.006, 0.005],
    [0.009, 0.010, 0.005],
    [0.016, 0.017, 0.011],
    [0.023, 0.024, 0.018],
    [0.036, 0.035, 0.030],
    [0.045, 0.046, 0.048],
    [0.057, 0.056, 0.054],
    [0.066, 0.054, 0.057],
    [0.076, 0.071, 0.074],
    [0.073, 0.071, 0.071],
    [0.074, 0.072, 0.069],
    [0.075, 0.073, 0.067],
    [0.075, 0.073, 0.071],
    [0.071, 0.073, 0.068],
    [0.063, 0.063, 0.067],
    [0.051, 0.052, 0.056],
    [0.043, 0.044, 0.049],
    [0.037, 0.038, 0.040],
    [0.032, 0.033, 0.035],
    [0.023, 0.026, 0.024],
];

fn class_index(class: FunctionalClass) -> usize {
    match class {
        FunctionalClass::Expressway => 0,
        FunctionalClass::UrbanPrincipalArterial => 1,
        FunctionalClass::UrbanMinorArterial => 2,
    }
}

/// HCM Exhibit 17-5: default hour-of-day demand ratio (ADT/AADT).
///
/// * `class` — functional class
/// * `hour` — hour of day (0-23, hour starting)
/// * `weekend` — true for Saturday/Sunday
pub fn exhibit_17_5_hour_of_day_ratio(class: FunctionalClass, hour: u32, weekend: bool) -> f64 {
    let h = (hour % 24) as usize;
    let c = class_index(class);
    if weekend {
        EXHIBIT_17_5_WEEKEND[h][c]
    } else {
        EXHIBIT_17_5_WEEKDAY[h][c]
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Exhibit 17-6: Default Day-of-Week Demand Ratios (ADT/AADT)
// ═══════════════════════════════════════════════════════════════════════════════

/// HCM Exhibit 17-6: default day-of-week demand ratio. `day_of_week` is
/// 0 = Sunday … 6 = Saturday (the exhibit row order).
pub fn exhibit_17_6_day_of_week_ratio(day_of_week: u32) -> f64 {
    const RATIOS: [f64; 7] = [0.87, 0.98, 0.98, 1.00, 1.03, 1.15, 0.99];
    RATIOS[(day_of_week % 7) as usize]
}

// ═══════════════════════════════════════════════════════════════════════════════
// Exhibit 17-7: Default Month-of-Year Demand Ratios (ADT/AADT)
// ═══════════════════════════════════════════════════════════════════════════════

/// Exhibit 17-7 month-of-year demand ratios; rows Jan-Dec, columns
/// [expressway, principal arterial, minor arterial].
const EXHIBIT_17_7: [[f64; 3]; 12] = [
    [0.802, 0.831, 0.881],
    [0.874, 1.021, 0.944],
    [0.936, 1.030, 1.016],
    [0.958, 0.987, 0.844],
    [1.026, 1.012, 1.025],
    [1.068, 1.050, 1.060],
    [1.107, 0.991, 1.150],
    [1.142, 1.054, 1.110],
    [1.088, 1.091, 1.081],
    [1.069, 0.952, 1.036],
    [0.962, 0.992, 0.989],
    [0.933, 0.938, 0.903],
];

/// HCM Exhibit 17-7: default month-of-year demand ratio. `month` is 1-12.
pub fn exhibit_17_7_month_of_year_ratio(class: FunctionalClass, month: u32) -> f64 {
    let m = ((month.clamp(1, 12) - 1) as usize).min(11);
    EXHIBIT_17_7[m][class_index(class)]
}

// ═══════════════════════════════════════════════════════════════════════════════
// Exhibit 17-8: Default Values for Weather Events
// ═══════════════════════════════════════════════════════════════════════════════

/// Exhibit 17-8 default demand change factor for a dry analysis period.
pub const DEFAULT_DEMAND_CHANGE_DRY: f64 = 1.00;
/// Exhibit 17-8 default demand change factor for a rain event.
pub const DEFAULT_DEMAND_CHANGE_RAIN: f64 = 1.00;
/// Exhibit 17-8 default demand change factor for a snow event.
pub const DEFAULT_DEMAND_CHANGE_SNOW: f64 = 0.80;
/// Exhibit 17-8 default pavement runoff duration for a snow event, h (the
/// time after snow stops falling that snowpack/ice covers the pavement).
pub const DEFAULT_SNOW_PAVEMENT_RUNOFF_H: f64 = 0.5;
/// Duration of pavement runoff for a rain event, h (Equation 29-11 text:
/// d_o = 0.083 h).
pub const RAIN_PAVEMENT_RUNOFF_H: f64 = 0.083;
/// Ratio of snow depth to equivalent rain (water) depth used to convert
/// precipitation statistics to snowfall statistics (Chapter 29, Section 2,
/// Step 7: "estimated at 10 in./in.").
pub const SNOW_TO_RAIN_DEPTH_RATIO: f64 = 10.0;

// ═══════════════════════════════════════════════════════════════════════════════
// Exhibit 17-9: Default Values for Incidents
// ═══════════════════════════════════════════════════════════════════════════════

/// HCM Exhibit 17-9: default crash frequency adjustment factor (CFAF) for
/// a weather condition — the ratio of the hourly crash frequency during
/// the weather event to the hourly crash rate during clear, dry hours.
pub fn exhibit_17_9_cfaf(weather: WeatherCondition) -> f64 {
    match weather {
        WeatherCondition::Dry => 1.0,
        WeatherCondition::Rainfall => 2.0,
        WeatherCondition::WetPavement => 3.0,
        WeatherCondition::Snowfall => 1.5,
        WeatherCondition::SnowOrIceOnPavement => 2.75,
    }
}

/// HCM Exhibit 17-9: default incident detection time, min (all weather
/// conditions).
pub const DEFAULT_INCIDENT_DETECTION_MIN: f64 = 2.0;

/// HCM Exhibit 17-9: default incident response time, min, by weather
/// condition.
pub fn exhibit_17_9_response_time_min(weather: WeatherCondition) -> f64 {
    match weather {
        WeatherCondition::Dry
        | WeatherCondition::Rainfall
        | WeatherCondition::WetPavement => 15.0,
        WeatherCondition::Snowfall | WeatherCondition::SnowOrIceOnPavement => 20.4,
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Exhibit 17-10: Default Incident Clearance Times
// ═══════════════════════════════════════════════════════════════════════════════

/// HCM Exhibit 17-10: default incident clearance time, min. In the
/// exhibit the defaults are identical across lane locations and across
/// the two street locations (segments vs. signalized intersections);
/// they vary by severity and weather. Weather columns: dry, rainfall,
/// wet pavement, snow-or-ice (the last column "applies to snowfall and
/// to snow or ice on pavement", exhibit note b).
pub fn exhibit_17_10_clearance_time_min(
    severity: IncidentSeverity,
    weather: WeatherCondition,
) -> f64 {
    let col = match weather {
        WeatherCondition::Dry => 0,
        WeatherCondition::Rainfall => 1,
        WeatherCondition::WetPavement => 2,
        WeatherCondition::Snowfall | WeatherCondition::SnowOrIceOnPavement => 3,
    };
    let row: [f64; 4] = match severity {
        IncidentSeverity::CrashFatalInjury => [56.4, 42.1, 43.5, 76.7],
        IncidentSeverity::CrashPropertyDamage => [39.5, 28.6, 29.7, 53.7],
        IncidentSeverity::NoncrashBreakdown => [10.8, 5.6, 5.7, 14.7],
        IncidentSeverity::NoncrashOther => [6.7, 2.4, 2.8, 9.1],
    };
    row[col]
}

/// Default average incident duration, min: detection + response +
/// clearance (Chapter 29, Section 2, Step 4 text; Exhibit 29-72 example:
/// 2.0 + 15.0 + 10.8 = 27.8 min for a dry-weather noncrash one-lane
/// breakdown).
pub fn default_incident_duration_min(
    severity: IncidentSeverity,
    weather: WeatherCondition,
) -> f64 {
    DEFAULT_INCIDENT_DETECTION_MIN
        + exhibit_17_9_response_time_min(weather)
        + exhibit_17_10_clearance_time_min(severity, weather)
}

/// Standard deviation of incident duration: `s = 0.8 × mean` (Equation
/// 29-19 definitions).
pub const INCIDENT_DURATION_CV: f64 = 0.8;

// ═══════════════════════════════════════════════════════════════════════════════
// Exhibits 17-11 / 17-12: Default Incident Distribution
// ═══════════════════════════════════════════════════════════════════════════════

/// HCM Exhibit 17-11 (facilities **with** shoulders) / Exhibit 17-12
/// (facilities **without** shoulders): proportion of incidents that are
/// crashes, by street location (the "Type / Proportion" column; identical
/// in both exhibits).
pub fn crash_proportion(street: StreetLocation) -> f64 {
    match street {
        StreetLocation::Segment => 0.358,
        StreetLocation::Intersection => 0.310,
    }
}

/// Joint incident-type proportions (last column of Exhibit 17-11/17-12),
/// in [`INCIDENT_TYPES`] order. Proportions total 1.000 for a street
/// location. Shoulder rows are 0.0 when `shoulder_present` is false
/// (Exhibit 17-12).
pub fn incident_joint_proportions(
    street: StreetLocation,
    shoulder_present: bool,
) -> [f64; 12] {
    match (street, shoulder_present) {
        // Exhibit 17-11, segment rows.
        (StreetLocation::Segment, true) => [
            0.036, 0.083, 0.028, 0.030, 0.020, 0.160, // crash 1L/2L/shoulder × FI/PDO
            0.456, 0.089, 0.059, 0.017, 0.014, 0.007, // noncrash × bkd/other
        ],
        // Exhibit 17-11, signalized intersection rows.
        (StreetLocation::Intersection, true) => [
            0.037, 0.061, 0.018, 0.026, 0.018, 0.150, //
            0.486, 0.086, 0.084, 0.013, 0.018, 0.003,
        ],
        // Exhibit 17-12, segment rows (no shoulder).
        (StreetLocation::Segment, false) => [
            0.091, 0.209, 0.028, 0.030, 0.0, 0.0, //
            0.473, 0.093, 0.059, 0.017, 0.0, 0.0,
        ],
        // Exhibit 17-12, signalized intersection rows (no shoulder).
        (StreetLocation::Intersection, false) => [
            0.100, 0.165, 0.018, 0.026, 0.0, 0.0, //
            0.503, 0.089, 0.084, 0.013, 0.0, 0.0,
        ],
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Weather adjustment factors (Equations 29-25 and 29-26)
// ═══════════════════════════════════════════════════════════════════════════════

/// HCM Equation 29-25: saturation flow rate adjustment factor for rainfall
/// or snowfall, `f_rs = 1 / (1 + 0.48 R_r + 0.39 R_s)`, with the constants
/// 0.95 for wet pavement (not raining) and 0.90 for snow or ice on the
/// pavement (not snowing).
///
/// * `weather` — weather condition of the analysis period
/// * `precip_rate_in_h` — precipitation rate while precipitation is
///   falling, in equivalent inches of water per hour (0.0 otherwise)
pub fn weather_sat_flow_factor(weather: WeatherCondition, precip_rate_in_h: f64) -> f64 {
    match weather {
        WeatherCondition::Dry => 1.0,
        WeatherCondition::Rainfall => 1.0 / (1.0 + 0.48 * precip_rate_in_h.max(0.0)),
        WeatherCondition::Snowfall => 1.0 / (1.0 + 0.39 * precip_rate_in_h.max(0.0)),
        WeatherCondition::WetPavement => 0.95,
        WeatherCondition::SnowOrIceOnPavement => 0.90,
    }
}

/// HCM Equation 29-26: free-flow speed adjustment factor for rainfall or
/// snowfall, `f_s,rs = 1 / (1 + 0.48 R_r + 1.4 R_s)`, with the constants
/// 0.95 for wet pavement (not raining) and 0.90 for snow or ice on the
/// pavement (not snowing).
pub fn weather_ffs_factor(weather: WeatherCondition, precip_rate_in_h: f64) -> f64 {
    match weather {
        WeatherCondition::Dry => 1.0,
        WeatherCondition::Rainfall => 1.0 / (1.0 + 0.48 * precip_rate_in_h.max(0.0)),
        WeatherCondition::Snowfall => 1.0 / (1.0 + 1.4 * precip_rate_in_h.max(0.0)),
        WeatherCondition::WetPavement => 0.95,
        WeatherCondition::SnowOrIceOnPavement => 0.90,
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Incident adjustment factors (Equations 29-27/29-28 and 29-34 to 29-36)
// ═══════════════════════════════════════════════════════════════════════════════

/// HCM Equation 29-28 / 29-36: calibration coefficient based on incident
/// severity, `b_ic = 0.58 I_fi + 0.42 I_pdo + 0.17 I_other` (noncrash
/// incidents — breakdowns and other — use the 0.17 "other" coefficient).
pub fn incident_severity_coefficient(severity: IncidentSeverity) -> f64 {
    match severity {
        IncidentSeverity::CrashFatalInjury => 0.58,
        IncidentSeverity::CrashPropertyDamage => 0.42,
        IncidentSeverity::NoncrashBreakdown | IncidentSeverity::NoncrashOther => 0.17,
    }
}

/// HCM Equation 29-27: saturation flow rate adjustment factor for incident
/// presence on an intersection movement,
/// `f_ic = (1 − N_ic/N_n)(1 − b_ic/ΣN_n) ≥ 0.10`.
///
/// * `lanes_blocked` — number of lanes serving the movement blocked by the
///   incident, N_ic (ln)
/// * `movement_lanes` — number of lanes serving the movement under normal
///   conditions, N_n (ln)
/// * `approach_lanes` — total number of lanes on the approach across the
///   L/T/R movements, Σ N_n (ln)
/// * `severity` — incident severity (Equation 29-28 coefficient)
///
/// "If all lanes associated with a movement are closed because of the
/// incident, an adjustment factor of 0.10 is used."
pub fn incident_sat_flow_factor(
    lanes_blocked: u32,
    movement_lanes: u32,
    approach_lanes: u32,
    severity: IncidentSeverity,
) -> f64 {
    let n_n = movement_lanes.max(1) as f64;
    let n_ic = (lanes_blocked.min(movement_lanes)) as f64;
    let sum_n = approach_lanes.max(1) as f64;
    let b_ic = incident_severity_coefficient(severity);
    let f = (1.0 - n_ic / n_n) * (1.0 - b_ic / sum_n);
    f.max(0.10)
}

/// HCM Equation 29-35: adjusted base free-flow speed during an analysis
/// period, `S*_fo = S_fo × f_s,rs × (1 − b_ic / N_o)`.
///
/// * `base_ffs` — base free-flow speed S_fo (any speed unit)
/// * `weather_factor` — Equation 29-26 factor f_s,rs
/// * `incident_severity` — severity of an incident in the subject
///   direction, if any (None ⇒ b_ic = 0)
/// * `direction_lanes` — number of lanes serving the direction, N_o (ln)
pub fn adjusted_base_ffs(
    base_ffs: f64,
    weather_factor: f64,
    incident_severity: Option<IncidentSeverity>,
    direction_lanes: u32,
) -> f64 {
    let b_ic = incident_severity.map_or(0.0, incident_severity_coefficient);
    base_ffs * weather_factor * (1.0 - b_ic / direction_lanes.max(1) as f64)
}

/// HCM Equation 29-34: additional running delay from weather/incidents,
/// `d_other = L (1/S* − 1/S_fo)` (s/veh), with the segment length in feet
/// and speeds in ft/s. This delay is added to the Chapter 18 `d_other`
/// input rather than modifying the segment free-flow speed.
///
/// * `segment_length_ft` — segment length L, ft
/// * `base_ffs_mph` — base free-flow speed S_fo, mi/h
/// * `adjusted_ffs_mph` — adjusted base free-flow speed S* (Equation
///   29-35), mi/h
pub fn additional_delay_s(
    segment_length_ft: f64,
    base_ffs_mph: f64,
    adjusted_ffs_mph: f64,
) -> f64 {
    if base_ffs_mph <= 0.0 || adjusted_ffs_mph <= 0.0 {
        return 0.0;
    }
    let to_fps = 5_280.0 / 3_600.0;
    segment_length_ft * (1.0 / (adjusted_ffs_mph * to_fps) - 1.0 / (base_ffs_mph * to_fps))
}

// ═══════════════════════════════════════════════════════════════════════════════
// Exhibit 29-5: Additional Critical Left-Turn Headway due to Weather
// ═══════════════════════════════════════════════════════════════════════════════

/// HCM Exhibit 29-5: additional critical left-turn headway due to
/// weather, s (Step 8 of the scenario dataset generation procedure).
pub fn exhibit_29_5_extra_lt_headway_s(weather: WeatherCondition) -> f64 {
    match weather {
        WeatherCondition::Dry => 0.0,
        WeatherCondition::SnowOrIceOnPavement => 0.9, // clear, snow or ice on pavement
        WeatherCondition::WetPavement => 0.7,         // clear, water on pavement
        WeatherCondition::Snowfall => 1.2,            // snowing
        WeatherCondition::Rainfall => 0.7,            // raining
    }
}
