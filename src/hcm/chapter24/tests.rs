//! Unit tests for the HCM Chapter 24 off-street pedestrian and bicycle facility
//! methodologies. Expected intermediate and final values are taken from HCM
//! 7th Edition, Chapter 35 (Pedestrians and Bicycles: Supplemental), Example
//! Problems 1 and 2.

use assert_approx_eq::assert_approx_eq;

use super::offstreet_pedbike::*;
use crate::hcm::common::LevelOfService;

// ─────────────────────────────────────────────────────────────────────────────
// Exclusive pedestrian facilities (Chapter 35, Example Problem 1, Step 4)
// ─────────────────────────────────────────────────────────────────────────────

/// Example Problem 1 exclusive path: 5-ft path, no obstacles, peak 15-min
/// volume of 100 p, pedestrian speed 4.0 ft/s (240 ft/min), random flow.
fn example_problem_1_exclusive_path() -> ExclusivePedestrianFacility {
    ExclusivePedestrianFacility::new(
        5.0,
        0.0,
        None,
        Some(100.0),
        Some(0.83),
        Some(240.0),
        PedestrianFacilityType::Walkway,
        PedestrianFlowType::Random,
    )
}

#[test]
fn test_step1_determine_effective_walkway_width() {
    // Equation 24-1: W_E = W_T - W_O = 5 - 0 = 5 ft.
    let mut facility = example_problem_1_exclusive_path();
    assert_approx_eq!(facility.determine_effective_walkway_width(), 5.0, 1e-9);

    // With a tree (effective width 4 ft, Exhibit 24-9) on an 8-ft walkway.
    let mut obstructed = ExclusivePedestrianFacility {
        total_walkway_width: 8.0,
        fixed_object_width: 4.0,
        ..Default::default()
    };
    assert_approx_eq!(obstructed.determine_effective_walkway_width(), 4.0, 1e-9);
}

#[test]
fn test_step2_calculate_pedestrian_flow_rate() {
    // Chapter 35, Example Problem 1, Step 4.2: v_p = 100 / (15 × 5) = 1.33 p/ft/min.
    let mut facility = example_problem_1_exclusive_path();
    let vp = facility.calculate_pedestrian_flow_rate();
    assert_approx_eq!(vp, 1.333, 0.001);
    // The measured peak 15-min volume is used directly (Equation 24-2 skipped).
    assert_approx_eq!(facility.flow_rate_15min.unwrap(), 100.0, 1e-9);
}

#[test]
fn test_step2_hourly_demand_conversion() {
    // Equation 24-2: v_15 = v_h / (4 × PHF) = 1000 / (4 × 0.8) = 312.5 p.
    let mut facility = ExclusivePedestrianFacility {
        total_walkway_width: 10.0,
        pedestrian_demand: Some(1000.0),
        phf: 0.8,
        ..Default::default()
    };
    facility.calculate_pedestrian_flow_rate();
    assert_approx_eq!(facility.flow_rate_15min.unwrap(), 312.5, 1e-9);
    // Equation 24-3: v_p = 312.5 / (15 × 10) = 2.083 p/ft/min.
    assert_approx_eq!(facility.unit_flow_rate.unwrap(), 2.0833, 0.001);
}

#[test]
fn test_step3_calculate_average_pedestrian_space() {
    // Chapter 35, Example Problem 1, Step 4.3:
    // A_p = (4.0 ft/s × 60 s/min) / 1.33 p/ft/min = 180 ft²/p.
    let mut facility = example_problem_1_exclusive_path();
    let ap = facility.calculate_average_pedestrian_space();
    assert_approx_eq!(ap, 180.0, 0.5);
}

#[test]
fn test_step4_determine_los_example_problem_1() {
    // Chapter 35, Example Problem 1, Step 4.4: 180 ft²/p → LOS A (Exhibit 24-1).
    let mut facility = example_problem_1_exclusive_path();
    assert_eq!(facility.analyze(), LevelOfService::A);
}

#[test]
fn test_exhibit_24_1_walkway_random_flow_los_thresholds() {
    assert_eq!(walkway_random_flow_los(61.0), LevelOfService::A);
    assert_eq!(walkway_random_flow_los(60.0), LevelOfService::B);
    assert_eq!(walkway_random_flow_los(40.0), LevelOfService::C);
    assert_eq!(walkway_random_flow_los(24.0), LevelOfService::D);
    assert_eq!(walkway_random_flow_los(15.0), LevelOfService::E);
    assert_eq!(walkway_random_flow_los(8.0), LevelOfService::F);
    assert_eq!(walkway_random_flow_los(3.0), LevelOfService::F);
}

#[test]
fn test_exhibit_24_2_walkway_platoon_flow_los_thresholds() {
    assert_eq!(walkway_platoon_flow_los(531.0), LevelOfService::A);
    assert_eq!(walkway_platoon_flow_los(530.0), LevelOfService::B);
    assert_eq!(walkway_platoon_flow_los(90.0), LevelOfService::C);
    assert_eq!(walkway_platoon_flow_los(40.0), LevelOfService::D);
    assert_eq!(walkway_platoon_flow_los(23.0), LevelOfService::E);
    assert_eq!(walkway_platoon_flow_los(11.0), LevelOfService::F);
}

#[test]
fn test_exhibit_24_3_stairway_los_thresholds() {
    assert_eq!(stairway_los(21.0), LevelOfService::A);
    assert_eq!(stairway_los(20.0), LevelOfService::B);
    assert_eq!(stairway_los(17.0), LevelOfService::C);
    assert_eq!(stairway_los(12.0), LevelOfService::D);
    assert_eq!(stairway_los(8.0), LevelOfService::E);
    assert_eq!(stairway_los(5.0), LevelOfService::F);
}

#[test]
fn test_cross_flow_los_ef_threshold() {
    // Exhibit 24-1 note c: in cross-flow situations, the LOS E-F threshold is
    // 13 ft²/p. Space of 12 ft²/p is LOS E on a walkway but LOS F in cross-flow.
    let mut walkway = ExclusivePedestrianFacility {
        total_walkway_width: 10.0,
        peak_15min_volume: Some(2500.0),
        pedestrian_speed: 300.0,
        ..Default::default()
    };
    // v_p = 2500/(15×10) = 16.67 p/ft/min; A_p = 300/16.67 = 18 ft²/p → within E-D range
    walkway.analyze();
    assert_approx_eq!(walkway.pedestrian_space.unwrap(), 18.0, 0.01);
    assert_eq!(walkway.los.unwrap(), LevelOfService::D);

    let mut cross_flow = ExclusivePedestrianFacility {
        total_walkway_width: 10.0,
        peak_15min_volume: Some(3750.0),
        pedestrian_speed: 300.0,
        facility_type: PedestrianFacilityType::CrossFlow,
        ..Default::default()
    };
    // v_p = 3750/(15×10) = 25 p/ft/min; A_p = 300/25 = 12 ft²/p → F in cross-flow
    cross_flow.analyze();
    assert_approx_eq!(cross_flow.pedestrian_space.unwrap(), 12.0, 0.01);
    assert_eq!(cross_flow.los.unwrap(), LevelOfService::F);
}

#[test]
fn test_step5_volume_to_capacity_ratio() {
    // Random-flow walkway capacity is 23 p/min/ft (Chapter 24, Step 5).
    let mut facility = example_problem_1_exclusive_path();
    facility.analyze();
    // v/c = 1.333/23 = 0.058
    assert_approx_eq!(facility.vc_ratio.unwrap(), 1.3333 / 23.0, 0.001);

    // Stairway capacity is 15 p/min/ft.
    let mut stairway = ExclusivePedestrianFacility {
        total_walkway_width: 5.0,
        peak_15min_volume: Some(100.0),
        facility_type: PedestrianFacilityType::Stairway,
        ..Default::default()
    };
    stairway.analyze();
    assert_approx_eq!(stairway.vc_ratio.unwrap(), 1.3333 / 15.0, 0.001);
}

// ─────────────────────────────────────────────────────────────────────────────
// Pedestrians on shared-use paths (Chapter 35, Example Problem 1)
// ─────────────────────────────────────────────────────────────────────────────

/// Example Problem 1 shared-use path: Q_sb = Q_ob = 100 bicycles/h, PHF = 0.83,
/// S_p = 4.0 ft/s, S_b = 16.0 ft/s.
fn example_problem_1_shared_path() -> SharedUsePathPedestrian {
    SharedUsePathPedestrian::new(
        Some(100.0),
        Some(100.0),
        Some(0.83),
        Some(4.0),
        Some(16.0),
    )
}

#[test]
fn test_step2_passing_and_meeting_events() {
    let mut path = example_problem_1_shared_path();
    let (fp, fm, f) = path.calculate_bicycle_passing_and_meeting_events();
    // Equation 24-5: F_p = (100/0.83)(1 - 4/16) = 90 events/h.
    assert_approx_eq!(fp, 90.36, 0.5);
    // Equation 24-6: F_m = (100/0.83)(1 + 4/16) = 151 events/h.
    assert_approx_eq!(fm, 150.60, 0.5);
    // Equation 24-7: F = 90 + 0.5 × 151 = 166 events/h.
    assert_approx_eq!(f, 165.66, 0.5);
}

#[test]
fn test_step3_shared_path_pedestrian_los() {
    // Chapter 35, Example Problem 1, Step 3: 166 events/h → LOS E.
    let mut path = example_problem_1_shared_path();
    assert_eq!(path.analyze(), LevelOfService::E);
}

#[test]
fn test_one_way_path_has_no_meeting_events() {
    let mut path = example_problem_1_shared_path();
    path.is_one_way = true;
    let (fp, fm, f) = path.calculate_bicycle_passing_and_meeting_events();
    assert!(fp > 0.0);
    assert_approx_eq!(fm, 0.0, 1e-9);
    assert_approx_eq!(f, fp, 1e-9);
}

#[test]
fn test_peak_15min_flow_rates_bypass_phf() {
    // Field-measured peak 15-min flow rates substitute for the Q/PHF terms.
    let mut path = SharedUsePathPedestrian {
        bicycle_flow_rate_same_direction: Some(120.0),
        bicycle_flow_rate_opposing: Some(120.0),
        pedestrian_speed: 4.0,
        bicycle_speed: 16.0,
        ..Default::default()
    };
    let (fp, fm, _) = path.calculate_bicycle_passing_and_meeting_events();
    assert_approx_eq!(fp, 120.0 * 0.75, 1e-9);
    assert_approx_eq!(fm, 120.0 * 1.25, 1e-9);
}

#[test]
fn test_exhibit_24_4_shared_path_pedestrian_los_thresholds() {
    assert_eq!(shared_use_path_pedestrian_los(38.0), LevelOfService::A);
    assert_eq!(shared_use_path_pedestrian_los(38.1), LevelOfService::B);
    assert_eq!(shared_use_path_pedestrian_los(60.1), LevelOfService::C);
    assert_eq!(shared_use_path_pedestrian_los(103.1), LevelOfService::D);
    assert_eq!(shared_use_path_pedestrian_los(144.1), LevelOfService::E);
    assert_eq!(shared_use_path_pedestrian_los(180.1), LevelOfService::F);
}

// ─────────────────────────────────────────────────────────────────────────────
// Off-street bicycle facilities (Chapter 35, Example Problem 2)
// ─────────────────────────────────────────────────────────────────────────────

/// Example Problem 2: 10-ft path, no centerline, 3-mi segment, 340 users/h
/// two-way, PHF = 0.90, 50/50 directional split, Exhibit 24-6 default mode
/// splits and speeds.
fn example_problem_2_facility() -> OffStreetBicycleFacility {
    OffStreetBicycleFacility::new(10.0, 3.0, false, Some(340.0), Some(0.5), Some(0.90))
}

#[test]
fn test_step1_directional_flow_rates() {
    // Chapter 35, Example Problem 2, Step 1 (Equation 24-8):
    // bicycles 104/h, pedestrians 38/h, runners 19/h, skaters 19/h, children 9/h.
    let mut facility = example_problem_2_facility();
    let (qs, qo) = facility.calculate_directional_flow_rates();
    assert_approx_eq!(qs[PathUserMode::Bicycle as usize], 103.9, 0.5);
    assert_approx_eq!(qs[PathUserMode::Pedestrian as usize], 37.8, 0.5);
    assert_approx_eq!(qs[PathUserMode::Runner as usize], 18.9, 0.5);
    assert_approx_eq!(qs[PathUserMode::InlineSkater as usize], 18.9, 0.5);
    assert_approx_eq!(qs[PathUserMode::ChildBicyclist as usize], 9.4, 0.5);
    for i in 0..NUM_PATH_MODES {
        assert_approx_eq!(qs[i], qo[i], 1e-9);
    }
}

#[test]
fn test_step2_active_passings_per_minute() {
    // Chapter 35, Example Problem 2, Step 2 (Equations 24-9 to 24-12):
    // bicycles 0.18, pedestrians 1.74, runners 0.31, skaters 0.09,
    // child bicyclists 0.10; total A_T = 2.42 passings/min.
    let mut facility = example_problem_2_facility();
    let at = facility.calculate_active_passings_per_minute();
    let by_mode = facility.active_passings_by_mode.unwrap();
    assert_approx_eq!(by_mode[PathUserMode::Bicycle as usize], 0.18, 0.01);
    assert_approx_eq!(by_mode[PathUserMode::Pedestrian as usize], 1.74, 0.01);
    assert_approx_eq!(by_mode[PathUserMode::Runner as usize], 0.31, 0.01);
    assert_approx_eq!(by_mode[PathUserMode::InlineSkater as usize], 0.09, 0.01);
    assert_approx_eq!(by_mode[PathUserMode::ChildBicyclist as usize], 0.10, 0.01);
    assert_approx_eq!(at, 2.42, 0.01);
}

#[test]
fn test_step3_meetings_per_minute() {
    // Chapter 35, Example Problem 2, Step 3 (Equations 24-13 to 24-16):
    // M_1 = 5.36 (the published value uses a runner speed of 6.6 mi/h, a typo
    // for the 6.5 mi/h Exhibit 24-6 default; the exact value is 5.38),
    // M_2 by mode: bicycles 1.55, pedestrians 0.63, runners 0.32, skaters 0.31,
    // child bicyclists 0.16; total M_T = 8.33 meetings/min.
    let mut facility = example_problem_2_facility();
    let mt = facility.calculate_meetings_per_minute();
    assert_approx_eq!(facility.meetings_on_segment.unwrap(), 5.38, 0.03);
    let m2 = facility.meetings_beyond_segment_by_mode.unwrap();
    assert_approx_eq!(m2[PathUserMode::Bicycle as usize], 1.55, 0.01);
    assert_approx_eq!(m2[PathUserMode::Pedestrian as usize], 0.63, 0.01);
    assert_approx_eq!(m2[PathUserMode::Runner as usize], 0.32, 0.01);
    assert_approx_eq!(m2[PathUserMode::InlineSkater as usize], 0.31, 0.01);
    assert_approx_eq!(m2[PathUserMode::ChildBicyclist as usize], 0.16, 0.01);
    assert_approx_eq!(mt, 8.33, 0.03);
}

#[test]
fn test_step4_effective_lanes_exhibit_24_14() {
    // Exhibit 24-14: 8.0-10.5 ft → 2 lanes; 11.0-14.5 ft → 3; 15.0-20.0 ft → 4.
    let widths_lanes = [
        (8.0, 2),
        (10.0, 2),
        (10.5, 2),
        (11.0, 3),
        (14.5, 3),
        (15.0, 4),
        (20.0, 4),
    ];
    for (width, lanes) in widths_lanes {
        let mut facility = OffStreetBicycleFacility {
            path_width: width,
            ..Default::default()
        };
        assert_eq!(
            facility.determine_number_of_effective_lanes(),
            lanes,
            "width {width} ft"
        );
    }
}

#[test]
fn test_step5_blocked_lane_and_pair_probabilities() {
    // Chapter 35, Example Problem 2, Step 5 (Equations 24-17 and 24-20), using
    // the example's rounded directional flow rates (104 bicycles/h, 38 p/h):
    // P_n,bike = 0.1426, P_n,ped = 0.1908, P_ds(bike-ped) = 0.1707.
    let p_n_bike = probability_blocked(100.0, 104.0 / 12.8);
    let p_n_ped = probability_blocked(100.0, 38.0 / 3.4);
    assert_approx_eq!(p_n_bike, 0.1426, 0.0005);
    assert_approx_eq!(p_n_ped, 0.1908, 0.0005);
    let p_ds = delayed_passing_probability_two_lane(p_n_bike, p_n_ped);
    assert_approx_eq!(p_ds, 0.1707, 0.0005);
}

#[test]
fn test_step5_total_probability_of_delayed_passing() {
    // Chapter 35, Example Problem 2, Step 6 (Equation 24-33): P_Tds = 0.8334.
    let mut facility = example_problem_2_facility();
    let p_tds = facility.calculate_probability_of_delayed_passing();
    assert_approx_eq!(p_tds, 0.8334, 0.002);
}

#[test]
fn test_step6_delayed_passings_per_minute() {
    // Chapter 35, Example Problem 2, Step 6 (Equation 24-34):
    // DP_m = 2.42 × 0.8334 × 0.90 = 1.82.
    let mut facility = example_problem_2_facility();
    let dpm = facility.calculate_delayed_passings_per_minute();
    assert_approx_eq!(dpm, 1.82, 0.01);
}

#[test]
fn test_step7_blos_score_and_los() {
    // Chapter 35, Example Problem 2, Step 7 (Equation 24-35):
    // BLOS = 5.446 - 0.00809×[8.33 + 10×2.42] - 15.86×0.1 - 0.287×0 - 0.91 = 2.69
    // → LOS D (Exhibit 24-5).
    let mut facility = example_problem_2_facility();
    let los = facility.analyze();
    assert_approx_eq!(facility.blos_score.unwrap(), 2.69, 0.01);
    assert_eq!(los, LevelOfService::D);
}

#[test]
fn test_step8_low_volume_adjustment() {
    // A narrow 8-ft path with very low volume cannot reach LOS A through
    // Equation 24-35, but Step 8 assigns LOS A when weighted events/min ≤ 5.
    let mut facility = OffStreetBicycleFacility::new(8.0, 1.0, false, Some(20.0), None, None);
    let los = facility.analyze();
    assert!(facility.weighted_events_per_minute.unwrap() <= 5.0);
    assert_eq!(los, LevelOfService::A);
}

#[test]
fn test_exhibit_24_5_bicycle_los_thresholds() {
    assert_eq!(bicycle_los_from_score(4.1), LevelOfService::A);
    assert_eq!(bicycle_los_from_score(4.0), LevelOfService::B);
    assert_eq!(bicycle_los_from_score(3.5), LevelOfService::C);
    assert_eq!(bicycle_los_from_score(3.0), LevelOfService::D);
    assert_eq!(bicycle_los_from_score(2.5), LevelOfService::E);
    assert_eq!(bicycle_los_from_score(2.0), LevelOfService::F);
}

#[test]
fn test_exclusive_bicycle_facility_zero_nonbike_modes() {
    // In the special case of an exclusive off-street bicycle facility, the
    // volume for all nonbicycle modes is zero and events are determined solely
    // by the bicycle volume.
    let mut facility = example_problem_2_facility();
    for i in 1..NUM_PATH_MODES {
        facility.user_groups[i].mode_split = 0.0;
    }
    facility.user_groups[PathUserMode::Bicycle as usize].mode_split = 1.0;
    facility.analyze();
    let by_mode = facility.active_passings_by_mode.unwrap();
    for i in 1..NUM_PATH_MODES {
        assert_approx_eq!(by_mode[i], 0.0, 1e-12);
    }
    assert!(by_mode[PathUserMode::Bicycle as usize] > 0.0);
}

#[test]
fn test_one_way_bicycle_path_has_no_meetings() {
    let mut facility = example_problem_2_facility();
    facility.is_one_way = true;
    facility.analyze();
    assert_approx_eq!(facility.meetings_per_minute.unwrap(), 0.0, 1e-12);
    assert!(facility.active_passings_per_minute.unwrap() > 0.0);
}

#[test]
fn test_normal_cdf_reference_values() {
    assert_approx_eq!(normal_cdf(0.0, 0.0, 1.0), 0.5, 1e-6);
    assert_approx_eq!(normal_cdf(1.96, 0.0, 1.0), 0.975, 0.0005);
    // Chapter 35, Example Problem 2, Step 2: P[v_bike < 12.76] = 0.4950 with
    // mean 12.8 mi/h and standard deviation 3.4 mi/h.
    assert_approx_eq!(normal_cdf(12.76 - 0.0, 12.8, 3.4), 0.4953, 0.001);
}
