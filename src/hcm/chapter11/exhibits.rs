//! HCM Chapter 11 (Freeway Reliability Analysis) exhibit lookups and
//! equation transcriptions, plus the Chapter 25 Section 9 (Freeway Scenario
//! Generation) defaults shared with the scenario generator.
//!
//! Sources (HCM 7th Edition EPUB):
//! - Exhibits 11-18/11-19 (default urban/rural demand ratios),
//!   Exhibits 11-20/11-21 (default weather CAFs/SAFs),
//!   Exhibit 11-22 (incident severity distribution and durations),
//!   Exhibit 11-23 (incident CAFs by directional lanes),
//!   Equations 11-1 through 11-5 (planning-level method): `78_Ch11_05.xhtml`
//! - Equations 25-78/25-79 (incident rate from crash rate; HERS model),
//!   Equation 25-85 (default severity distribution G(i)),
//!   Exhibit 25-41 (incident duration distribution parameters):
//!   `200_Ch25_09.xhtml`

use serde::{Deserialize, Serialize};

/// HCM chapter implemented by this module.
pub const CHAPTER: u32 = 11;

// ═════════════════════════════════════════════════════════════════════════
// Weather (Exhibits 11-20 and 11-21)
// ═════════════════════════════════════════════════════════════════════════

/// HCM weather event types (Exhibit 11-20 weather event definitions).
///
/// Only weather events reducing capacity by more than about 4–5% are
/// modeled (Chapter 11 limitations; Chapter 25 Step 11). "Non-severe
/// weather" covers all other conditions with CAF = SAF = 1.0.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WeatherType {
    /// >0.10–0.25 in./h rain.
    MediumRain,
    /// >0.25 in./h rain.
    HeavyRain,
    /// >0.00–0.05 in./h snow.
    LightSnow,
    /// >0.05–0.10 in./h snow.
    LightMediumSnow,
    /// >0.10–0.50 in./h snow.
    MediumHeavySnow,
    /// >0.50 in./h snow.
    HeavySnow,
    /// < −4 °F.
    SevereCold,
    /// 0.50–0.99 mi visibility.
    LowVisibility,
    /// 0.25–0.49 mi visibility.
    VeryLowVisibility,
    /// <0.25 mi visibility.
    MinimalVisibility,
    /// All conditions not listed above (CAF = SAF = 1.0).
    NonSevere,
}

/// The ten severe weather types (excludes [`WeatherType::NonSevere`]) in
/// Exhibit 11-20 row order.
pub const SEVERE_WEATHER_TYPES: [WeatherType; 10] = [
    WeatherType::MediumRain,
    WeatherType::HeavyRain,
    WeatherType::LightSnow,
    WeatherType::LightMediumSnow,
    WeatherType::MediumHeavySnow,
    WeatherType::HeavySnow,
    WeatherType::SevereCold,
    WeatherType::LowVisibility,
    WeatherType::VeryLowVisibility,
    WeatherType::MinimalVisibility,
];

impl WeatherType {
    /// Row index in the Exhibit 11-20/11-21 tables (`None` for non-severe).
    fn row(self) -> Option<usize> {
        SEVERE_WEATHER_TYPES.iter().position(|w| *w == self)
    }

    pub fn name(self) -> &'static str {
        match self {
            WeatherType::MediumRain => "Medium rain",
            WeatherType::HeavyRain => "Heavy rain",
            WeatherType::LightSnow => "Light snow",
            WeatherType::LightMediumSnow => "Light-medium snow",
            WeatherType::MediumHeavySnow => "Medium-heavy snow",
            WeatherType::HeavySnow => "Heavy snow",
            WeatherType::SevereCold => "Severe cold",
            WeatherType::LowVisibility => "Low visibility",
            WeatherType::VeryLowVisibility => "Very low visibility",
            WeatherType::MinimalVisibility => "Minimal visibility",
            WeatherType::NonSevere => "Non-severe weather",
        }
    }
}

/// Free-flow speeds of the Exhibit 11-20/11-21 columns, mi/h.
const WEATHER_FFS_COLS: [f64; 5] = [55.0, 60.0, 65.0, 70.0, 75.0];

/// Exhibit 11-20: default CAFs by weather condition and facility FFS
/// (rows in `SEVERE_WEATHER_TYPES` order; columns 55/60/65/70/75 mi/h).
const WEATHER_CAF: [[f64; 5]; 10] = [
    [0.94, 0.93, 0.92, 0.91, 0.90], // Medium rain
    [0.89, 0.88, 0.86, 0.84, 0.82], // Heavy rain
    [0.97, 0.96, 0.96, 0.95, 0.95], // Light snow
    [0.95, 0.94, 0.92, 0.90, 0.88], // Light-medium snow
    [0.93, 0.91, 0.90, 0.88, 0.87], // Medium-heavy snow
    [0.80, 0.78, 0.76, 0.74, 0.72], // Heavy snow
    [0.93, 0.92, 0.92, 0.91, 0.90], // Severe cold
    [0.90, 0.90, 0.90, 0.90, 0.90], // Low visibility
    [0.88, 0.88, 0.88, 0.88, 0.88], // Very low visibility
    [0.90, 0.90, 0.90, 0.90, 0.90], // Minimal visibility
];

/// Exhibit 11-21: default SAFs by weather condition and facility FFS.
const WEATHER_SAF: [[f64; 5]; 10] = [
    [0.96, 0.95, 0.94, 0.93, 0.93], // Medium rain
    [0.94, 0.93, 0.93, 0.92, 0.91], // Heavy rain
    [0.94, 0.92, 0.89, 0.87, 0.84], // Light snow
    [0.92, 0.90, 0.88, 0.86, 0.83], // Light-medium snow
    [0.90, 0.88, 0.86, 0.84, 0.82], // Medium-heavy snow
    [0.88, 0.86, 0.85, 0.83, 0.81], // Heavy snow
    [0.95, 0.95, 0.94, 0.93, 0.92], // Severe cold
    [0.96, 0.95, 0.94, 0.94, 0.93], // Low visibility
    [0.95, 0.94, 0.93, 0.92, 0.91], // Very low visibility
    [0.95, 0.94, 0.93, 0.92, 0.91], // Minimal visibility
];

/// Interpolate an Exhibit 11-20/11-21 row at the facility FFS (columns are
/// 55–75 mi/h in 5-mi/h steps; FFS outside the range is clamped).
fn weather_table_lookup(table: &[[f64; 5]; 10], weather: WeatherType, ffs: f64) -> f64 {
    let Some(row) = weather.row() else {
        return 1.0; // Non-severe weather
    };
    let vals = &table[row];
    let ffs = ffs.clamp(WEATHER_FFS_COLS[0], WEATHER_FFS_COLS[4]);
    for k in 0..4 {
        let (lo, hi) = (WEATHER_FFS_COLS[k], WEATHER_FFS_COLS[k + 1]);
        if ffs <= hi {
            let f = (ffs - lo) / (hi - lo);
            return vals[k] + f * (vals[k + 1] - vals[k]);
        }
    }
    vals[4]
}

/// Exhibit 11-20: default capacity adjustment factor for a weather type at
/// facility free-flow speed `ffs` (mi/h; linear interpolation between the
/// 5-mi/h columns, clamped to 55–75 mi/h).
pub fn weather_caf(weather: WeatherType, ffs: f64) -> f64 {
    weather_table_lookup(&WEATHER_CAF, weather, ffs)
}

/// Exhibit 11-21: default speed adjustment factor for a weather type at
/// facility free-flow speed `ffs` (mi/h).
pub fn weather_saf(weather: WeatherType, ffs: f64) -> f64 {
    weather_table_lookup(&WEATHER_SAF, weather, ffs)
}

// ═════════════════════════════════════════════════════════════════════════
// Demand ratios (Exhibits 11-18 and 11-19)
// ═════════════════════════════════════════════════════════════════════════

/// Exhibit 11-18: default urban freeway demand ratios (ADT / Mondays in
/// January). Rows = months January–December; columns = Monday–Sunday.
pub const URBAN_DEMAND_RATIOS: [[f64; 7]; 12] = [
    [1.00, 1.00, 1.02, 1.05, 1.17, 1.01, 0.89], // January
    [1.03, 1.03, 1.05, 1.08, 1.21, 1.04, 0.92], // February
    [1.12, 1.12, 1.14, 1.18, 1.31, 1.13, 0.99], // March
    [1.19, 1.19, 1.21, 1.25, 1.39, 1.20, 1.05], // April
    [1.18, 1.18, 1.21, 1.24, 1.39, 1.20, 1.05], // May
    [1.24, 1.24, 1.27, 1.31, 1.46, 1.26, 1.10], // June
    [1.38, 1.38, 1.41, 1.45, 1.62, 1.39, 1.22], // July
    [1.26, 1.26, 1.28, 1.32, 1.47, 1.27, 1.12], // August
    [1.29, 1.29, 1.32, 1.36, 1.52, 1.31, 1.15], // September
    [1.21, 1.21, 1.24, 1.27, 1.42, 1.22, 1.07], // October
    [1.21, 1.21, 1.24, 1.27, 1.42, 1.22, 1.07], // November
    [1.19, 1.19, 1.21, 1.25, 1.40, 1.20, 1.06], // December
];

/// Exhibit 11-19: default rural freeway demand ratios (ADT / Mondays in
/// January). Rows = months January–December; columns = Monday–Sunday.
pub const RURAL_DEMAND_RATIOS: [[f64; 7]; 12] = [
    [1.00, 0.96, 0.98, 1.03, 1.22, 1.11, 1.06], // January
    [1.11, 1.06, 1.09, 1.14, 1.35, 1.23, 1.18], // February
    [1.24, 1.19, 1.21, 1.28, 1.51, 1.37, 1.32], // March
    [1.33, 1.27, 1.30, 1.37, 1.62, 1.47, 1.41], // April
    [1.46, 1.39, 1.42, 1.50, 1.78, 1.61, 1.55], // May
    [1.48, 1.42, 1.45, 1.53, 1.81, 1.63, 1.57], // June
    [1.66, 1.59, 1.63, 1.72, 2.03, 1.84, 1.77], // July
    [1.52, 1.46, 1.49, 1.57, 1.86, 1.68, 1.62], // August
    [1.46, 1.39, 1.42, 1.50, 1.78, 1.61, 1.55], // September
    [1.33, 1.28, 1.31, 1.38, 1.63, 1.47, 1.42], // October
    [1.30, 1.25, 1.28, 1.35, 1.59, 1.44, 1.39], // November
    [1.17, 1.12, 1.14, 1.20, 1.43, 1.29, 1.24], // December
];

// ═════════════════════════════════════════════════════════════════════════
// Incidents (Exhibits 11-22 and 11-23; Chapter 25 Equations 25-78/25-79,
// 25-85; Exhibit 25-41)
// ═════════════════════════════════════════════════════════════════════════

/// Incident severity types (Equation 25-85 index i = 1…5).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IncidentSeverity {
    /// i = 1: shoulder closed (no travel lane lost).
    Shoulder,
    /// i = 2: one lane closed.
    OneLane,
    /// i = 3: two lanes closed.
    TwoLanes,
    /// i = 4: three lanes closed.
    ThreeLanes,
    /// i = 5: four or more lanes closed (default probability 0).
    FourPlusLanes,
}

/// All severity types in Equation 25-85 order.
pub const INCIDENT_SEVERITIES: [IncidentSeverity; 5] = [
    IncidentSeverity::Shoulder,
    IncidentSeverity::OneLane,
    IncidentSeverity::TwoLanes,
    IncidentSeverity::ThreeLanes,
    IncidentSeverity::FourPlusLanes,
];

impl IncidentSeverity {
    /// Number of travel lanes closed by the incident.
    pub fn lanes_closed(self) -> u32 {
        match self {
            IncidentSeverity::Shoulder => 0,
            IncidentSeverity::OneLane => 1,
            IncidentSeverity::TwoLanes => 2,
            IncidentSeverity::ThreeLanes => 3,
            IncidentSeverity::FourPlusLanes => 4,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            IncidentSeverity::Shoulder => "Shoulder closed",
            IncidentSeverity::OneLane => "1 lane closed",
            IncidentSeverity::TwoLanes => "2 lanes closed",
            IncidentSeverity::ThreeLanes => "3 lanes closed",
            IncidentSeverity::FourPlusLanes => "4+ lanes closed",
        }
    }
}

/// Equation 25-85 / Exhibit 11-22: default national incident severity
/// distribution G(i) (shoulder, 1-lane, 2-lane, 3-lane, 4+-lane).
pub const DEFAULT_INCIDENT_SEVERITY_DISTRIBUTION: [f64; 5] =
    [0.754, 0.196, 0.031, 0.019, 0.0];

/// Lognormal incident duration parameters, minutes.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct IncidentDurationParams {
    /// Mean duration, min.
    pub mean: f64,
    /// Standard deviation of duration, min.
    pub std_dev: f64,
    /// Minimum duration, min.
    pub min: f64,
    /// Maximum duration, min.
    pub max: f64,
}

/// Exhibit 11-22 / Exhibit 25-41: default incident duration distribution
/// parameters (min) by severity (shoulder, 1, 2, 3, 4+ lanes closed).
///
/// VERIFY-HCM: Exhibit 11-22 lists a mean of 67.9 min for 3-lane closures,
/// while Exhibit 25-41 lists an average of 69.6 min and a median of 67.9
/// min for "3 or more" lanes; the Chapter 11 value (67.9) is used here for
/// both the 3-lane and 4+-lane types, matching Exhibit 11-22.
pub const DEFAULT_INCIDENT_DURATION_PARAMS: [IncidentDurationParams; 5] = [
    IncidentDurationParams { mean: 34.0, std_dev: 15.1, min: 8.7, max: 58.0 },
    IncidentDurationParams { mean: 34.6, std_dev: 13.8, min: 16.0, max: 58.2 },
    IncidentDurationParams { mean: 53.6, std_dev: 13.9, min: 30.5, max: 66.9 },
    IncidentDurationParams { mean: 67.9, std_dev: 21.9, min: 36.0, max: 93.3 },
    IncidentDurationParams { mean: 67.9, std_dev: 21.9, min: 36.0, max: 93.3 },
];

/// Exhibit 11-23: CAFs by incident type and number of directional lanes
/// (2–8). The tabulated value is the remaining relative capacity **per open
/// lane**; `None` marks infeasible combinations (closure equals or exceeds
/// the number of directional lanes — full closures are not modeled).
///
/// Columns: shoulder, 1-lane, 2-lane, 3-lane, 4-lane closure.
const INCIDENT_CAF_PER_OPEN_LANE: [[Option<f64>; 5]; 7] = [
    // 2 lanes
    [Some(0.81), Some(0.70), None, None, None],
    // 3 lanes
    [Some(0.83), Some(0.74), Some(0.51), None, None],
    // 4 lanes
    [Some(0.85), Some(0.77), Some(0.50), Some(0.52), None],
    // 5 lanes
    [Some(0.87), Some(0.81), Some(0.67), Some(0.50), Some(0.50)],
    // 6 lanes
    [Some(0.89), Some(0.85), Some(0.75), Some(0.52), Some(0.52)],
    // 7 lanes
    [Some(0.91), Some(0.88), Some(0.80), Some(0.63), Some(0.63)],
    // 8 lanes
    [Some(0.93), Some(0.89), Some(0.84), Some(0.66), Some(0.66)],
];

/// Exhibit 11-23: per-open-lane incident CAF for `directional_lanes` (2–8,
/// clamped) and `severity`. Returns `None` when the closure equals or
/// exceeds the number of directional lanes (full closures are not modeled;
/// the scenario generator reassigns such incidents to a less severe type).
pub fn incident_caf_per_open_lane(
    directional_lanes: u32,
    severity: IncidentSeverity,
) -> Option<f64> {
    let lanes = directional_lanes.clamp(2, 8) as usize;
    let col = match severity {
        IncidentSeverity::Shoulder => 0,
        IncidentSeverity::OneLane => 1,
        IncidentSeverity::TwoLanes => 2,
        IncidentSeverity::ThreeLanes => 3,
        IncidentSeverity::FourPlusLanes => 4,
    };
    if severity.lanes_closed() >= directional_lanes {
        return None;
    }
    INCIDENT_CAF_PER_OPEN_LANE[lanes - 2][col]
}

/// Effective incident CAF on the **total segment capacity** for a segment
/// with `directional_lanes`: per Exhibit 11-23 the tabulated value applies
/// per open lane, so the total-capacity multiplier is
/// `CAF_table × (N − lanes closed) / N` (e.g., a 2-lane closure on a
/// 6-lane facility keeps 0.75 × 4/6 = 50% of the original capacity, the
/// underscored example in the exhibit's note).
///
/// Returns `None` for infeasible severity/lanes combinations.
pub fn incident_caf_total(directional_lanes: u32, severity: IncidentSeverity) -> Option<f64> {
    let per_lane = incident_caf_per_open_lane(directional_lanes, severity)?;
    let n = f64::from(directional_lanes);
    let open = f64::from(directional_lanes - severity.lanes_closed());
    Some(per_lane * open / n)
}

/// Most severe feasible incident type at a segment with
/// `directional_lanes`, at most `severity` (Chapter 11 limitations: "The
/// scenario generation methodology does not assign incidents that result
/// in full segment closure; it reassigns those probabilities to other
/// (less severe) incidents").
pub fn feasible_severity(directional_lanes: u32, severity: IncidentSeverity) -> IncidentSeverity {
    let mut idx = INCIDENT_SEVERITIES.iter().position(|s| *s == severity).unwrap_or(0);
    while idx > 0 && incident_caf_per_open_lane(directional_lanes, INCIDENT_SEVERITIES[idx]).is_none()
    {
        idx -= 1;
    }
    INCIDENT_SEVERITIES[idx]
}

/// Default national incident-to-crash ratio ICR (Equation 25-78: "In the
/// absence of other data, a national default value for ICR is 4.9").
pub const DEFAULT_INCIDENT_TO_CRASH_RATIO: f64 = 4.9;

/// Equation 25-78: incident rate per 100 million VMT from the local crash
/// rate `crash_rate` (per 100 million VMT) and incident-to-crash ratio.
pub fn incident_rate_from_crash_rate(crash_rate: f64, incident_to_crash_ratio: f64) -> f64 {
    crash_rate * incident_to_crash_ratio
}

/// Equation 25-79 (HERS model): crash rate per 100 million VMT:
///
/// `CR = (154.0 − 1.203·ACR + 0.258·ACR² − 0.00000524·ACR⁵) ×
///  e^(0.0082·(12 − LW))`
///
/// * `acr` — facility AADT divided by its two-way hourly capacity
/// * `lane_width_ft` — lane width, ft
pub fn hers_crash_rate(acr: f64, lane_width_ft: f64) -> f64 {
    (154.0 - 1.203 * acr + 0.258 * acr.powi(2) - 0.000_005_24 * acr.powi(5))
        * (0.0082 * (12.0 - lane_width_ft)).exp()
}

// ═════════════════════════════════════════════════════════════════════════
// Planning-level reliability method (Equations 11-1 through 11-5)
// ═════════════════════════════════════════════════════════════════════════

/// Equation 11-2: recurring delay rate, h/mi:
/// `RDR = 1/S − 1/FFS`
///
/// * `peak_speed` — peak-hour speed S, mi/h
/// * `ffs` — free-flow speed, mi/h
pub fn planning_recurring_delay_rate(peak_speed: f64, ffs: f64) -> f64 {
    1.0 / peak_speed - 1.0 / ffs
}

/// Equation 11-3: incident delay rate, h/mi:
/// `IDR = [0.020 − (N − 2) × 0.003] × X^12`
///
/// Valid for X <= 1.00 and N = 2–4; X is capped at 1.00 and N at 4 per the
/// Chapter 11 text.
///
/// * `lanes` — number of lanes in one direction N
/// * `vc_ratio` — peak-hour volume-to-capacity ratio X
pub fn planning_incident_delay_rate(lanes: u32, vc_ratio: f64) -> f64 {
    let n = f64::from(lanes.clamp(2, 4));
    let x = vc_ratio.min(1.0);
    (0.020 - (n - 2.0) * 0.003) * x.powi(12)
}

/// Equation 11-1: average annual mean travel time index:
/// `TTI_mean = 1 + FFS × (RDR + IDR)`
pub fn planning_tti_mean(ffs: f64, peak_speed: f64, lanes: u32, vc_ratio: f64) -> f64 {
    1.0 + ffs
        * (planning_recurring_delay_rate(peak_speed, ffs)
            + planning_incident_delay_rate(lanes, vc_ratio))
}

/// Equation 11-4: 95th percentile TTI:
/// `TTI_95 = 1 + 3.67 × ln(TTI_mean)`
pub fn planning_tti_95(tti_mean: f64) -> f64 {
    1.0 + 3.67 * tti_mean.ln()
}

/// Equation 11-5: percentage of trips below 45 mi/h (decimal):
/// `PT_45 = 1 − exp[−1.5115 × (TTI_mean − 1)]`
pub fn planning_pt45(tti_mean: f64) -> f64 {
    1.0 - (-1.5115 * (tti_mean - 1.0)).exp()
}

// ═════════════════════════════════════════════════════════════════════════
// Tests
// ═════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exhibit_11_20_caf_columns() {
        // Column values at exact FFS entries (Exhibit 11-20).
        assert!((weather_caf(WeatherType::MediumRain, 60.0) - 0.93).abs() < 1e-12);
        assert!((weather_caf(WeatherType::HeavyRain, 60.0) - 0.88).abs() < 1e-12);
        assert!((weather_caf(WeatherType::HeavyRain, 75.0) - 0.82).abs() < 1e-12);
        assert!((weather_caf(WeatherType::HeavySnow, 55.0) - 0.80).abs() < 1e-12);
        assert!((weather_caf(WeatherType::LowVisibility, 65.0) - 0.90).abs() < 1e-12);
        assert!((weather_caf(WeatherType::NonSevere, 60.0) - 1.0).abs() < 1e-12);
        // Interpolation between 60 and 65 for heavy rain: 0.87.
        assert!((weather_caf(WeatherType::HeavyRain, 62.5) - 0.87).abs() < 1e-9);
        // Clamping outside the table range.
        assert!((weather_caf(WeatherType::HeavyRain, 50.0) - 0.89).abs() < 1e-12);
        assert!((weather_caf(WeatherType::HeavyRain, 80.0) - 0.82).abs() < 1e-12);
    }

    #[test]
    fn test_exhibit_11_21_saf_columns() {
        assert!((weather_saf(WeatherType::MediumRain, 60.0) - 0.95).abs() < 1e-12);
        assert!((weather_saf(WeatherType::HeavyRain, 60.0) - 0.93).abs() < 1e-12);
        assert!((weather_saf(WeatherType::LightSnow, 75.0) - 0.84).abs() < 1e-12);
        assert!((weather_saf(WeatherType::MinimalVisibility, 60.0) - 0.94).abs() < 1e-12);
        assert!((weather_saf(WeatherType::NonSevere, 70.0) - 1.0).abs() < 1e-12);
    }

    #[test]
    fn test_exhibit_11_18_11_19_demand_ratios() {
        // Exhibit 11-18 spot checks (urban).
        assert!((URBAN_DEMAND_RATIOS[0][0] - 1.00).abs() < 1e-12); // Jan Mon
        assert!((URBAN_DEMAND_RATIOS[6][4] - 1.62).abs() < 1e-12); // Jul Fri
        assert!((URBAN_DEMAND_RATIOS[10][1] - 1.21).abs() < 1e-12); // Nov Tue
        assert!((URBAN_DEMAND_RATIOS[11][6] - 1.06).abs() < 1e-12); // Dec Sun
        // Exhibit 11-19 spot checks (rural).
        assert!((RURAL_DEMAND_RATIOS[0][1] - 0.96).abs() < 1e-12); // Jan Tue
        assert!((RURAL_DEMAND_RATIOS[6][4] - 2.03).abs() < 1e-12); // Jul Fri
        assert!((RURAL_DEMAND_RATIOS[8][3] - 1.50).abs() < 1e-12); // Sep Thu
    }

    #[test]
    fn test_equation_25_85_severity_distribution() {
        let g = DEFAULT_INCIDENT_SEVERITY_DISTRIBUTION;
        assert!((g.iter().sum::<f64>() - 1.0).abs() < 1e-9);
        assert!((g[0] - 0.754).abs() < 1e-12);
        assert!((g[1] - 0.196).abs() < 1e-12);
        assert!((g[2] - 0.031).abs() < 1e-12);
        assert!((g[3] - 0.019).abs() < 1e-12);
        assert_eq!(g[4], 0.0);
    }

    #[test]
    fn test_exhibit_11_23_incident_caf() {
        use IncidentSeverity as S;
        // Table values (per open lane).
        assert_eq!(incident_caf_per_open_lane(2, S::Shoulder), Some(0.81));
        assert_eq!(incident_caf_per_open_lane(3, S::TwoLanes), Some(0.51));
        assert_eq!(incident_caf_per_open_lane(6, S::TwoLanes), Some(0.75));
        assert_eq!(incident_caf_per_open_lane(8, S::FourPlusLanes), Some(0.66));
        // N/A cells.
        assert_eq!(incident_caf_per_open_lane(2, S::TwoLanes), None);
        assert_eq!(incident_caf_per_open_lane(3, S::ThreeLanes), None);
        assert_eq!(incident_caf_per_open_lane(4, S::FourPlusLanes), None);
        // Exhibit note example: 2-lane closure on 6 directional lanes keeps
        // 0.75 x 4/6 = 50% of total capacity.
        let total = incident_caf_total(6, S::TwoLanes).unwrap();
        assert!((total - 0.50).abs() < 1e-9);
        // Shoulder closure: no lanes lost, total multiplier = table value.
        assert!((incident_caf_total(3, S::Shoulder).unwrap() - 0.83).abs() < 1e-12);
    }

    #[test]
    fn test_feasible_severity_downgrade() {
        use IncidentSeverity as S;
        assert_eq!(feasible_severity(3, S::ThreeLanes), S::TwoLanes);
        assert_eq!(feasible_severity(2, S::FourPlusLanes), S::OneLane);
        assert_eq!(feasible_severity(4, S::ThreeLanes), S::ThreeLanes);
        assert_eq!(feasible_severity(3, S::Shoulder), S::Shoulder);
    }

    #[test]
    fn test_equation_25_79_hers_crash_rate() {
        // 12-ft lanes: exponent term = 1.
        let cr = hers_crash_rate(0.0, 12.0);
        assert!((cr - 154.0).abs() < 1e-9);
        // ACR = 10: 154 - 12.03 + 25.8 - 0.524 = 167.246.
        let cr = hers_crash_rate(10.0, 12.0);
        assert!((cr - 167.246).abs() < 1e-3);
        // Narrower lanes increase the rate.
        assert!(hers_crash_rate(5.0, 11.0) > hers_crash_rate(5.0, 12.0));
    }

    /// Chapter 25, Example Problem 10 (planning-level reliability;
    /// `202_Ch25_11a.xhtml`): FFS 75 mi/h, peak speed 62 mi/h, 3 lanes,
    /// X = 0.95 => RDR 0.00280, IDR 0.00919, TTI_mean 1.899,
    /// TTI_95 3.353, PT_45 74.3%.
    #[test]
    fn test_example_problem_10_planning_method() {
        let rdr = planning_recurring_delay_rate(62.0, 75.0);
        assert!((rdr - 0.00280).abs() < 0.00001, "RDR {rdr}");
        let idr = planning_incident_delay_rate(3, 0.95);
        assert!((idr - 0.00919).abs() < 0.00001, "IDR {idr}");
        let tti_mean = planning_tti_mean(75.0, 62.0, 3, 0.95);
        assert!((tti_mean - 1.899).abs() < 0.001, "TTI_mean {tti_mean}");
        let tti_95 = planning_tti_95(tti_mean);
        assert!((tti_95 - 3.353).abs() < 0.005, "TTI_95 {tti_95}");
        let pt45 = planning_pt45(tti_mean);
        assert!((pt45 - 0.743).abs() < 0.001, "PT45 {pt45}");
    }
}
