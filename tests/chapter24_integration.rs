//! Integration tests for HCM Chapter 24 (Off-Street Pedestrian and Bicycle
//! Facilities) against the published results of HCM 7th Edition, Chapter 35
//! (Pedestrians and Bicycles: Supplemental), Example Problems 1 and 2.
//!
//! Tolerances: LOS letters are asserted exactly. Published numeric results are
//! rounded to two or three significant figures and were computed with rounded
//! intermediate values (e.g., directional flow rates rounded to whole users per
//! hour), so numeric assertions use tolerances of roughly half a unit in the
//! last published digit, widened where the HCM's own rounding drift is larger
//! (documented per assertion).

use assert_approx_eq::assert_approx_eq;
use serde::Deserialize;

use transportations_library::hcm::chapter24::offstreet_pedbike::{
    ExclusivePedestrianFacility, OffStreetBicycleFacility, PathUserMode, SharedUsePathPedestrian,
};
use transportations_library::hcm::common::LevelOfService;

fn load_case(name: &str) -> serde_json::Value {
    let path = format!(
        "{}/tests/ExampleCases/hcm/OffStreetPedBike/{}",
        env!("CARGO_MANIFEST_DIR"),
        name
    );
    let data = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("failed to read fixture {path}: {e}"));
    serde_json::from_str(&data).unwrap_or_else(|e| panic!("failed to parse fixture {path}: {e}"))
}

fn los_from_str(s: &str) -> LevelOfService {
    LevelOfService::from(s.chars().next().expect("empty LOS string"))
}

#[derive(Deserialize)]
struct Case1Expected {
    passing_events_per_hour: f64,
    meeting_events_per_hour: f64,
    total_events_per_hour: f64,
    shared_use_path_pedestrian_los: String,
    effective_width_ft: f64,
    unit_flow_rate_p_ft_min: f64,
    pedestrian_space_ft2_p: f64,
    exclusive_path_pedestrian_los: String,
}

/// HCM Chapter 35, Example Problem 1: pedestrian LOS on shared-use and
/// exclusive paths.
#[test]
fn test_example_problem_1_pedestrian_los() {
    let case = load_case("case1.json");
    let expected: Case1Expected = serde_json::from_value(case["expected"].clone()).unwrap();

    // Part 1: pedestrian LOS on the existing shared-use path.
    let mut shared: SharedUsePathPedestrian =
        serde_json::from_value(case["shared_use_path"].clone()).unwrap();
    let los = shared.analyze();

    // Published: F_p = 90 events/h, F_m = 151 events/h, F = 166 events/h
    // (values rounded to whole events; exact values are 90.36, 150.60, 165.66).
    assert_approx_eq!(
        shared.passing_events.unwrap(),
        expected.passing_events_per_hour,
        0.5
    );
    assert_approx_eq!(
        shared.meeting_events.unwrap(),
        expected.meeting_events_per_hour,
        0.5
    );
    assert_approx_eq!(
        shared.total_events.unwrap(),
        expected.total_events_per_hour,
        0.5
    );
    // Published: LOS E (Exhibit 24-4).
    assert_eq!(los, los_from_str(&expected.shared_use_path_pedestrian_los));

    // Part 2: pedestrian LOS on a parallel 5-ft exclusive path.
    let mut exclusive: ExclusivePedestrianFacility =
        serde_json::from_value(case["exclusive_path"].clone()).unwrap();
    let los = exclusive.analyze();

    // Published: W_E = 5 ft, v_p = 1.33 p/ft/min, A_p = 180 ft²/p, LOS A.
    assert_approx_eq!(
        exclusive.effective_width.unwrap(),
        expected.effective_width_ft,
        1e-9
    );
    assert_approx_eq!(
        exclusive.unit_flow_rate.unwrap(),
        expected.unit_flow_rate_p_ft_min,
        0.005
    );
    assert_approx_eq!(
        exclusive.pedestrian_space.unwrap(),
        expected.pedestrian_space_ft2_p,
        0.5
    );
    assert_eq!(los, los_from_str(&expected.exclusive_path_pedestrian_los));
}

#[derive(Deserialize)]
struct Case2Expected {
    directional_bicycle_flow_rate: f64,
    directional_pedestrian_flow_rate: f64,
    directional_runner_flow_rate: f64,
    directional_inline_skater_flow_rate: f64,
    directional_child_bicyclist_flow_rate: f64,
    active_passings_per_minute: f64,
    meetings_per_minute: f64,
    effective_lanes: u8,
    total_probability_delayed_passing: f64,
    delayed_passings_per_minute: f64,
    blos_score: f64,
    bicycle_los: String,
}

/// HCM Chapter 35, Example Problem 2: bicycle LOS on a shared-use path.
#[test]
fn test_example_problem_2_bicycle_los() {
    let case = load_case("case2.json");
    let expected: Case2Expected = serde_json::from_value(case["expected"].clone()).unwrap();

    let mut facility: OffStreetBicycleFacility =
        serde_json::from_value(case["bicycle_facility"].clone()).unwrap();
    let los = facility.analyze();

    // Step 1 (Equation 24-8), published values rounded to whole users/h:
    // 104 bicycles/h, 38 p/h, 19 runners/h, 19 skaters/h, 9 child bicyclists/h.
    // Tolerance 0.5 users/h against the exact (unrounded) computation.
    let qs = facility.subject_flow_rates.unwrap();
    assert_approx_eq!(
        qs[PathUserMode::Bicycle as usize],
        expected.directional_bicycle_flow_rate,
        0.5
    );
    assert_approx_eq!(
        qs[PathUserMode::Pedestrian as usize],
        expected.directional_pedestrian_flow_rate,
        0.5
    );
    assert_approx_eq!(
        qs[PathUserMode::Runner as usize],
        expected.directional_runner_flow_rate,
        0.5
    );
    assert_approx_eq!(
        qs[PathUserMode::InlineSkater as usize],
        expected.directional_inline_skater_flow_rate,
        0.5
    );
    // The published 9 child bicyclists/h is truncated from 9.44.
    assert_approx_eq!(
        qs[PathUserMode::ChildBicyclist as usize],
        expected.directional_child_bicyclist_flow_rate,
        0.5
    );

    // Step 2 (Equations 24-9 to 24-12), published: A_T = 2.42 passings/min.
    assert_approx_eq!(
        facility.active_passings_per_minute.unwrap(),
        expected.active_passings_per_minute,
        0.01
    );

    // Step 3 (Equations 24-13 to 24-16), published: M_T = 8.33 meetings/min.
    // Tolerance 0.03: the published M_1 = 5.36 was computed with a 6.6 mi/h
    // runner speed (a typo for the 6.5 mi/h Exhibit 24-6 default).
    assert_approx_eq!(
        facility.meetings_per_minute.unwrap(),
        expected.meetings_per_minute,
        0.03
    );

    // Step 4 (Exhibit 24-14), published: 2 effective lanes for a 10-ft path.
    assert_eq!(facility.effective_lanes.unwrap(), expected.effective_lanes);

    // Steps 5-6 (Equations 24-17 to 24-34), published: P_Tds = 0.8334,
    // DP_m = 1.82. Tolerance reflects the example's rounded flow rates.
    assert_approx_eq!(
        facility.total_probability_delayed_passing.unwrap(),
        expected.total_probability_delayed_passing,
        0.002
    );
    assert_approx_eq!(
        facility.delayed_passings_per_minute.unwrap(),
        expected.delayed_passings_per_minute,
        0.01
    );

    // Step 7 (Equation 24-35), published: BLOS = 2.69 → LOS D (Exhibit 24-5).
    assert_approx_eq!(facility.blos_score.unwrap(), expected.blos_score, 0.01);
    assert_eq!(los, los_from_str(&expected.bicycle_los));
}
