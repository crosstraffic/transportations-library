//! Coefficient tables and saturation-flow adjustment factors transcribed
//! from the HCM 7th Edition, Chapter 19 (Signalized Intersections) and the
//! supplemental factor procedures of Chapter 31.
//!
//! Every constant and equation carries a citation of the HCM exhibit or
//! equation number it was transcribed from.

// ═══════════════════════════════════════════════════════════════════════════════
// Calibration constants (HCM Chapter 19 / Chapter 31)
// ═══════════════════════════════════════════════════════════════════════════════

/// Default base saturation flow rate for metropolitan areas with population
/// >= 250,000, pc/h/ln (HCM Exhibit 19-11).
pub const BASE_SATURATION_FLOW_METRO: f64 = 1_900.0;

/// Default base saturation flow rate for areas with population < 250,000,
/// pc/h/ln (HCM Exhibit 19-11).
pub const BASE_SATURATION_FLOW_NON_METRO: f64 = 1_750.0;

/// Start-up lost time l1 = 2.0 s (HCM Equation 19-1 variable list).
pub const START_UP_LOST_TIME: f64 = 2.0;

/// Extension of effective green e = 2.0 s (HCM Equation 19-1 variable list).
pub const EXTENSION_OF_EFFECTIVE_GREEN: f64 = 2.0;

/// Equivalent number of through cars for a protected left-turning vehicle,
/// E_L = 1.05 (HCM Equation 19-14).
pub const E_L_PROTECTED_LEFT: f64 = 1.05;

/// Equivalent number of through cars for a protected right-turning vehicle,
/// E_R = 1.18 (HCM Equation 19-13).
pub const E_R_PROTECTED_RIGHT: f64 = 1.18;

/// Critical headway for a permitted left turn, t_cg = 4.5 s
/// (HCM Equation 31-100 variable list).
pub const CRITICAL_HEADWAY_PERMITTED_LEFT: f64 = 4.5;

/// Follow-up headway for a permitted left turn, t_fh = 2.5 s
/// (HCM Equation 31-100 variable list).
pub const FOLLOW_UP_HEADWAY_PERMITTED_LEFT: f64 = 2.5;

/// Number of left-turn sneakers per cycle, n_s = 2.0 veh
/// (HCM Chapter 31, Section 3, discussion of Exhibit 31-13).
pub const SNEAKERS_PER_CYCLE: f64 = 2.0;

/// Critical merge headway for a lane change, t_lc = 3.7 s
/// (HCM Equation 31-50 variable list). The maximum flow rate at which a
/// lane change can occur is s_lc = 3,600 / t_lc veh/h/ln.
pub const CRITICAL_MERGE_HEADWAY: f64 = 3.7;

/// Threshold speed defining a stopped vehicle, S_s = 5.0 mi/h
/// (HCM Equation 31-131 variable list).
pub const STOP_THRESHOLD_SPEED_MPH: f64 = 5.0;

/// Acceleration rate r_a = 3.5 ft/s^2 (HCM Equation 31-131 variable list).
pub const QUEUE_ACCELERATION_RATE: f64 = 3.5;

/// Deceleration rate r_d = 4.0 ft/s^2 (HCM Equation 31-131 variable list).
pub const QUEUE_DECELERATION_RATE: f64 = 4.0;

/// Stored passenger-car lane length, L_pc = 25 ft
/// (HCM Equation 31-155 variable list).
pub const STORED_PASSENGER_CAR_LENGTH_FT: f64 = 25.0;

/// Stored heavy-vehicle lane length, L_HV = 45 ft
/// (HCM Equation 31-155 variable list).
pub const STORED_HEAVY_VEHICLE_LENGTH_FT: f64 = 45.0;

// ═══════════════════════════════════════════════════════════════════════════════
// Saturation flow adjustment factors (HCM Chapter 19, Step 4)
// ═══════════════════════════════════════════════════════════════════════════════

/// HCM Exhibit 19-20: Lane Width Adjustment Factor f_w.
///
/// | Average lane width (ft) | f_w  |
/// |-------------------------|------|
/// | < 10.0 (>= 8.0)         | 0.96 |
/// | 10.0–12.9               | 1.00 |
/// | > 12.9                  | 1.04 |
///
/// The factor must not be used for average lane widths below 8.0 ft
/// (Exhibit 19-20 note a); widths below 8.0 ft are clamped to the
/// 0.96 tier here.
pub fn lane_width_factor(avg_lane_width_ft: f64) -> f64 {
    if avg_lane_width_ft < 10.0 {
        0.96
    } else if avg_lane_width_ft <= 12.9 {
        1.00
    } else {
        1.04
    }
}

/// HCM Equations 19-9 and 19-10: heavy-vehicle and grade adjustment factor
/// f_HVg (7th Edition combined factor).
///
/// Downhill (P_g < 0):  `f_HVg = (100 - 0.79 P_HV - 2.07 P_g) / 100`   (Eq. 19-9)
/// Level/uphill:        `f_HVg = (100 - 0.78 P_HV - 0.31 P_g^2) / 100` (Eq. 19-10)
///
/// * `pct_heavy_vehicles` — percentage heavy vehicles in the movement
///   group P_HV (%), valid up to 50%
/// * `grade_pct` — approach grade P_g (%), positive upgrade, valid
///   -4.0% to +10.0%
pub fn heavy_vehicle_grade_factor(pct_heavy_vehicles: f64, grade_pct: f64) -> f64 {
    if grade_pct < 0.0 {
        (100.0 - 0.79 * pct_heavy_vehicles - 2.07 * grade_pct) / 100.0
    } else {
        (100.0 - 0.78 * pct_heavy_vehicles - 0.31 * grade_pct * grade_pct) / 100.0
    }
}

/// HCM Equation 19-11: parking adjustment factor
///
/// `f_p = (N - 0.1 - 18 N_m / 3,600) / N >= 0.050`
///
/// * `n_lanes` — number of lanes in the lane group N (ln)
/// * `maneuvers_per_h` — parking maneuver rate adjacent to the lane group
///   N_m (maneuvers/h); a practical upper limit of 180 maneuvers/h applies
///
/// Returns 1.0 if no parking is present (call only when parking exists).
pub fn parking_factor(n_lanes: u32, maneuvers_per_h: f64) -> f64 {
    let n = n_lanes.max(1) as f64;
    let nm = maneuvers_per_h.min(180.0);
    ((n - 0.1 - 18.0 * nm / 3_600.0) / n).max(0.050)
}

/// HCM Equation 19-12: bus blockage adjustment factor
///
/// `f_bb = (N - 14.4 N_b / 3,600) / N >= 0.050`
///
/// * `n_lanes` — number of lanes in the lane group N (ln)
/// * `buses_per_h` — bus stopping rate on the approach N_b (buses/h);
///   a practical upper limit of 250 buses/h applies
pub fn bus_blockage_factor(n_lanes: u32, buses_per_h: f64) -> f64 {
    let n = n_lanes.max(1) as f64;
    let nb = buses_per_h.min(250.0);
    ((n - 14.4 * nb / 3_600.0) / n).max(0.050)
}

/// HCM Chapter 19, Step 4, Adjustment for Area Type: f_a = 0.90 in areas
/// with CBD-like characteristics, 1.00 otherwise.
pub fn area_type_factor(cbd: bool) -> f64 {
    if cbd {
        0.90
    } else {
        1.00
    }
}

/// HCM Equation 19-7: lane utilization adjustment factor
///
/// `f_LU = v_g / (N_e * vg1)`
///
/// * `v_g` — demand flow rate for the movement group (veh/h)
/// * `n_exclusive_lanes` — number of exclusive lanes in the movement group N_e (ln)
/// * `vg1` — demand flow rate in the single exclusive lane with the highest
///   flow rate of all exclusive lanes in the movement group (veh/h/ln)
pub fn lane_utilization_factor(v_g: f64, n_exclusive_lanes: u32, vg1: f64) -> f64 {
    if vg1 <= 0.0 || n_exclusive_lanes == 0 {
        return 1.0;
    }
    v_g / (n_exclusive_lanes as f64 * vg1)
}

/// Movement-group type for the Exhibit 19-15 default lane utilization
/// factor lookup.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LaneUtilizationGroup {
    ExclusiveThrough,
    ExclusiveLeft,
    ExclusiveRight,
}

/// HCM Exhibit 19-15: Default Lane Utilization Adjustment Factors.
///
/// | Movement group     | Lanes | % in heaviest lane | f_LU  |
/// |--------------------|-------|--------------------|-------|
/// | Exclusive through  | 1     | 100.0              | 1.000 |
/// |                    | 2     | 52.5               | 0.952 |
/// |                    | 3+    | 36.7               | 0.908 |
/// | Exclusive left     | 1     | 100.0              | 1.000 |
/// |                    | 2+    | 51.5               | 0.971 |
/// | Exclusive right    | 1     | 100.0              | 1.000 |
/// |                    | 2+    | 56.5               | 0.885 |
///
/// Per Exhibit 19-15 note a, groups with more lanes than tabulated use the
/// smallest tabulated factor for the group type.
pub fn default_lane_utilization_factor(group: LaneUtilizationGroup, lanes: u32) -> f64 {
    match (group, lanes) {
        (_, 0) | (_, 1) => 1.000,
        (LaneUtilizationGroup::ExclusiveThrough, 2) => 0.952,
        (LaneUtilizationGroup::ExclusiveThrough, _) => 0.908,
        (LaneUtilizationGroup::ExclusiveLeft, _) => 0.971,
        (LaneUtilizationGroup::ExclusiveRight, _) => 0.885,
    }
}

/// HCM Equation 19-13: right-turn adjustment factor `f_RT = 1 / E_R`
/// with E_R = 1.18 (protected right turn in an exclusive lane).
pub fn protected_right_turn_factor() -> f64 {
    1.0 / E_R_PROTECTED_RIGHT
}

/// HCM Equation 19-14: left-turn adjustment factor `f_LT = 1 / E_L`
/// with E_L = 1.05 (protected left turn in an exclusive lane).
pub fn protected_left_turn_factor() -> f64 {
    1.0 / E_L_PROTECTED_LEFT
}

/// HCM Equations 31-89, 31-90, and 31-91: work zone presence adjustment
/// factor
///
/// `f_wz = 0.858 * f_wid * f_reduce <= 1.0`
/// `f_wid = 1 / (1 - 0.0057 (a_w - 12))`
/// `f_reduce = 1 / (1 + 0.0402 (n_o - n_wz))`
///
/// * `approach_lane_width_ft` — total width of all open left-turn, through,
///   and right-turn lanes during the work zone a_w (ft)
/// * `lanes_open_normal` — number of left-turn and through lanes open during
///   normal operation n_o (ln)
/// * `lanes_open_work_zone` — number of left-turn and through lanes open
///   during work zone presence n_wz (ln)
pub fn work_zone_factor(
    approach_lane_width_ft: f64,
    lanes_open_normal: u32,
    lanes_open_work_zone: u32,
) -> f64 {
    let f_wid = 1.0 / (1.0 - 0.0057 * (approach_lane_width_ft - 12.0));
    let f_reduce =
        1.0 / (1.0 + 0.0402 * (lanes_open_normal as f64 - lanes_open_work_zone as f64));
    (0.858 * f_wid * f_reduce).min(1.0)
}

// ═══════════════════════════════════════════════════════════════════════════════
// Arrival type and platoon ratio (HCM Exhibits 19-13 and 19-14)
// ═══════════════════════════════════════════════════════════════════════════════

/// HCM Exhibit 19-13: Relationship Between Arrival Type and Progression
/// Quality.
///
/// | Platoon ratio R_p | Arrival type | Progression quality      |
/// |-------------------|--------------|--------------------------|
/// | 0.33              | 1            | Very poor                |
/// | 0.67              | 2            | Unfavorable              |
/// | 1.00              | 3            | Random arrivals          |
/// | 1.33              | 4            | Favorable                |
/// | 1.67              | 5            | Highly favorable         |
/// | 2.00              | 6            | Exceptionally favorable  |
///
/// Returns `None` for arrival types outside 1–6.
pub fn platoon_ratio_for_arrival_type(arrival_type: u8) -> Option<f64> {
    match arrival_type {
        1 => Some(0.33),
        2 => Some(0.67),
        3 => Some(1.00),
        4 => Some(1.33),
        5 => Some(1.67),
        6 => Some(2.00),
        _ => None,
    }
}

/// HCM Exhibit 19-13: progression quality description for an arrival type.
pub fn progression_quality_for_arrival_type(arrival_type: u8) -> Option<&'static str> {
    match arrival_type {
        1 => Some("Very poor"),
        2 => Some("Unfavorable"),
        3 => Some("Random arrivals"),
        4 => Some("Favorable"),
        5 => Some("Highly favorable"),
        6 => Some("Exceptionally favorable"),
        _ => None,
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Default rate tables (HCM Exhibits 19-16 and 19-17)
// ═══════════════════════════════════════════════════════════════════════════════

/// HCM Exhibit 19-16: Default Parking Maneuver Rate (maneuvers/h) for an
/// approach with on-street parking, based on parking within 250 ft of the
/// stop line (25 ft per space, 80% occupancy).
///
/// | Street type | Spaces in 250 ft | Time limit (h) | Turnover (veh/h) | Maneuvers/h |
/// |-------------|------------------|----------------|------------------|-------------|
/// | Two-way     | 10               | 1              | 1.0              | 16          |
/// | Two-way     | 10               | 2              | 0.5              | 8           |
/// | One-way     | 20               | 1              | 1.0              | 32          |
/// | One-way     | 20               | 2              | 0.5              | 16          |
///
/// * `one_way` — true for a one-way street (parking on both sides)
/// * `parking_time_limit_h` — posted parking time limit (1 or 2 h)
pub fn default_parking_maneuver_rate(one_way: bool, parking_time_limit_h: f64) -> f64 {
    match (one_way, parking_time_limit_h >= 2.0) {
        (false, false) => 16.0,
        (false, true) => 8.0,
        (true, false) => 32.0,
        (true, true) => 16.0,
    }
}

/// Street class for the Exhibit 19-17 default system cycle length lookup.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StreetClass {
    MajorArterial,
    MinorArterialOrGrid,
}

/// Left-turn phasing extent for the Exhibit 19-17 default system cycle
/// length lookup.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LeftTurnPhasingExtent {
    NoLeftTurnPhases,
    LeftTurnPhasesOnOneStreet,
    LeftTurnPhasesOnBothStreets,
}

/// HCM Exhibit 19-17: Default System Cycle Length (s) for coordinated
/// signal systems.
///
/// | Avg. segment length (ft) | Major arterial (none/one/both) | Minor arterial or grid (none/one/both) |
/// |--------------------------|--------------------------------|----------------------------------------|
/// | 1,300                    | 90 / 120 / 150                 | 60 / 80 / 120                          |
/// | 2,600                    | 90 / 120 / 150                 | 100 / 100 / 120                        |
/// | 3,900                    | 110 / 120 / 150                | n/a (returns `None`)                   |
///
/// Segment lengths are matched to the nearest tabulated row.
pub fn default_system_cycle_length(
    street_class: StreetClass,
    left_turn_phasing: LeftTurnPhasingExtent,
    avg_segment_length_ft: f64,
) -> Option<f64> {
    // Nearest tabulated segment-length row (1,300 / 2,600 / 3,900 ft).
    let row = if avg_segment_length_ft < 1_950.0 {
        0
    } else if avg_segment_length_ft < 3_250.0 {
        1
    } else {
        2
    };
    use LeftTurnPhasingExtent::*;
    use StreetClass::*;
    match (street_class, left_turn_phasing, row) {
        (MajorArterial, NoLeftTurnPhases, 0 | 1) => Some(90.0),
        (MajorArterial, NoLeftTurnPhases, _) => Some(110.0),
        (MajorArterial, LeftTurnPhasesOnOneStreet, _) => Some(120.0),
        (MajorArterial, LeftTurnPhasesOnBothStreets, _) => Some(150.0),
        (MinorArterialOrGrid, NoLeftTurnPhases, 0) => Some(60.0),
        (MinorArterialOrGrid, NoLeftTurnPhases, 1) => Some(100.0),
        (MinorArterialOrGrid, LeftTurnPhasesOnOneStreet, 0) => Some(80.0),
        (MinorArterialOrGrid, LeftTurnPhasesOnOneStreet, 1) => Some(100.0),
        (MinorArterialOrGrid, LeftTurnPhasesOnBothStreets, 0 | 1) => Some(120.0),
        (MinorArterialOrGrid, _, _) => None,
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    /// HCM Exhibit 19-20 tiers.
    #[test]
    fn test_lane_width_factor_exhibit_19_20() {
        assert_eq!(lane_width_factor(9.5), 0.96);
        assert_eq!(lane_width_factor(10.0), 1.00);
        assert_eq!(lane_width_factor(12.0), 1.00);
        assert_eq!(lane_width_factor(12.9), 1.00);
        assert_eq!(lane_width_factor(13.0), 1.04);
    }

    /// HCM Equations 19-9 and 19-10 (Example Problem 1 values: 5% HV level
    /// grade -> 0.961; 2% HV level grade -> 0.9844; Exhibit 31-77).
    #[test]
    fn test_heavy_vehicle_grade_factor() {
        assert!((heavy_vehicle_grade_factor(5.0, 0.0) - 0.961).abs() < 1e-9);
        assert!((heavy_vehicle_grade_factor(2.0, 0.0) - 0.9844).abs() < 1e-9);
        // Eq 19-9 downhill
        assert!((heavy_vehicle_grade_factor(10.0, -2.0) - (100.0 - 7.9 + 4.14) / 100.0).abs() < 1e-9);
        // Eq 19-10 uphill
        assert!(
            (heavy_vehicle_grade_factor(10.0, 4.0) - (100.0 - 7.8 - 0.31 * 16.0) / 100.0).abs()
                < 1e-9
        );
    }

    /// HCM Equation 19-11 (Example Problem 1: N=1, 5 maneuvers/h -> 0.875,
    /// shown as 0.88 in Exhibit 31-77).
    #[test]
    fn test_parking_factor() {
        assert!((parking_factor(1, 5.0) - 0.875).abs() < 1e-9);
        // Lower bound 0.050
        assert!((parking_factor(1, 180.0) - 0.05).abs() < 1e-12);
        // Rate capped at 180 maneuvers/h
        assert_eq!(parking_factor(1, 500.0), parking_factor(1, 180.0));
    }

    /// HCM Equation 19-12.
    #[test]
    fn test_bus_blockage_factor() {
        assert!((bus_blockage_factor(1, 0.0) - 1.0).abs() < 1e-12);
        assert!((bus_blockage_factor(2, 10.0) - (2.0 - 0.04) / 2.0).abs() < 1e-9);
        // Rate capped at 250 buses/h and factor bounded at 0.050
        assert_eq!(bus_blockage_factor(1, 400.0), bus_blockage_factor(1, 250.0));
        assert!(bus_blockage_factor(1, 250.0) >= 0.05);
    }

    #[test]
    fn test_area_type_factor() {
        assert_eq!(area_type_factor(true), 0.90);
        assert_eq!(area_type_factor(false), 1.00);
    }

    /// HCM Exhibit 19-15 default lane utilization values.
    #[test]
    fn test_default_lane_utilization_exhibit_19_15() {
        use LaneUtilizationGroup::*;
        assert_eq!(default_lane_utilization_factor(ExclusiveThrough, 1), 1.000);
        assert_eq!(default_lane_utilization_factor(ExclusiveThrough, 2), 0.952);
        assert_eq!(default_lane_utilization_factor(ExclusiveThrough, 3), 0.908);
        assert_eq!(default_lane_utilization_factor(ExclusiveThrough, 4), 0.908);
        assert_eq!(default_lane_utilization_factor(ExclusiveLeft, 2), 0.971);
        assert_eq!(default_lane_utilization_factor(ExclusiveRight, 2), 0.885);
    }

    /// HCM Equation 19-7: Exhibit 19-15 percentages reproduce the tabulated
    /// factors (e.g., 2 through lanes, 52.5% in the heaviest lane).
    #[test]
    fn test_lane_utilization_equation_19_7() {
        let f = lane_utilization_factor(1_000.0, 2, 525.0);
        assert!((f - 0.952).abs() < 0.001);
    }

    /// HCM Equations 19-13 and 19-14.
    #[test]
    fn test_turn_factors() {
        assert!((protected_right_turn_factor() - 1.0 / 1.18).abs() < 1e-12);
        assert!((protected_left_turn_factor() - 1.0 / 1.05).abs() < 1e-12);
    }

    /// HCM Exhibit 19-13.
    #[test]
    fn test_platoon_ratio_exhibit_19_13() {
        assert_eq!(platoon_ratio_for_arrival_type(1), Some(0.33));
        assert_eq!(platoon_ratio_for_arrival_type(3), Some(1.00));
        assert_eq!(platoon_ratio_for_arrival_type(6), Some(2.00));
        assert_eq!(platoon_ratio_for_arrival_type(0), None);
        assert_eq!(platoon_ratio_for_arrival_type(7), None);
        assert_eq!(
            progression_quality_for_arrival_type(3),
            Some("Random arrivals")
        );
    }

    /// HCM Equations 31-89 through 31-91: no work-zone narrowing or lane
    /// reduction gives f_wz = 0.858; the factor is capped at 1.0.
    #[test]
    fn test_work_zone_factor() {
        assert!((work_zone_factor(12.0, 2, 2) - 0.858).abs() < 1e-9);
        // Narrower lanes and a lane closure both reduce the factor.
        assert!(work_zone_factor(10.0, 2, 2) < 0.858);
        assert!(work_zone_factor(12.0, 2, 1) < 0.858);
        // Very wide approach cannot push the factor above 1.0.
        assert!(work_zone_factor(40.0, 2, 2) <= 1.0);
    }

    /// HCM Exhibit 19-16.
    #[test]
    fn test_default_parking_maneuver_rate_exhibit_19_16() {
        assert_eq!(default_parking_maneuver_rate(false, 1.0), 16.0);
        assert_eq!(default_parking_maneuver_rate(false, 2.0), 8.0);
        assert_eq!(default_parking_maneuver_rate(true, 1.0), 32.0);
        assert_eq!(default_parking_maneuver_rate(true, 2.0), 16.0);
    }

    /// HCM Exhibit 19-17.
    #[test]
    fn test_default_system_cycle_length_exhibit_19_17() {
        use LeftTurnPhasingExtent::*;
        use StreetClass::*;
        assert_eq!(
            default_system_cycle_length(MajorArterial, NoLeftTurnPhases, 1_300.0),
            Some(90.0)
        );
        assert_eq!(
            default_system_cycle_length(MajorArterial, NoLeftTurnPhases, 3_900.0),
            Some(110.0)
        );
        assert_eq!(
            default_system_cycle_length(MajorArterial, LeftTurnPhasesOnBothStreets, 2_600.0),
            Some(150.0)
        );
        assert_eq!(
            default_system_cycle_length(MinorArterialOrGrid, NoLeftTurnPhases, 1_300.0),
            Some(60.0)
        );
        assert_eq!(
            default_system_cycle_length(MinorArterialOrGrid, LeftTurnPhasesOnOneStreet, 2_600.0),
            Some(100.0)
        );
        assert_eq!(
            default_system_cycle_length(MinorArterialOrGrid, NoLeftTurnPhases, 3_900.0),
            None
        );
    }
}
