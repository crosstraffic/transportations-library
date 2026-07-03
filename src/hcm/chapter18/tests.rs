//! Unit tests for the HCM Chapter 18 (Urban Street Segments) motorized
//! vehicle methodology, keyed to Chapter 30, Example Problem 1 (EPUB
//! `240_Ch30_08.xhtml`, Exhibits 30-26 through 30-36) and hand-computed
//! checks of the Chapter 18 equations.

use super::urban_segments::*;
use crate::hcm::common::LevelOfService;

macro_rules! assert_near {
    ($actual:expr, $expected:expr, $tol:expr, $what:expr) => {
        let (a, e) = ($actual, $expected);
        assert!(
            (a - e).abs() <= $tol,
            "{}: got {a}, expected {e} (tolerance {})",
            $what,
            $tol
        );
    };
}

/// Chapter 30, Example Problem 1, eastbound direction. Boundary
/// intersection performance values are the published engine outputs
/// (Exhibits 30-32, 30-33, and 30-36):
/// * through control delay 18.310 s/veh, through capacity 1,848 veh/h,
///   through demand 968 veh/h (adjusted; Exhibit 30-32 WB columns, which
///   mirror the EB movement at Signalized Intersection 2),
/// * midsegment flow 1,150 veh/h (= 1,000 through + 50 NB right + 100 SB
///   left entering the segment at Signalized Intersection 1),
/// * per-access-point turning delays 0.193 and 0.194 s/veh (Exhibit 30-35),
/// * effective green 48.63 s (Exhibit 30-33, Timer 6: 52.63 − 4.00) with a
///   100-s cycle.
fn example_problem_1_segment() -> UrbanSegment {
    let mut seg = UrbanSegment::new(
        1_800.0,
        2,
        35.0,
        968.0,
        BoundaryControlType::Signalized,
    );
    seg.upstream_intersection_width_ft = 50.0;
    seg.proportion_with_curb = 0.70;
    seg.n_access_points_subject = 4.0;
    seg.n_access_points_opposing = 4.0;
    seg.signal_spacing_ft = Some(1_800.0);
    seg.midsegment_flow_veh_h = Some(1_150.0);
    seg.through_capacity_veh_h = Some(1_848.0);
    seg.through_control_delay_s = Some(18.310);
    seg.cycle_length_s = Some(100.0);
    seg.effective_green_s = Some(48.63);
    seg.access_point_delays_s = Some(vec![0.193, 0.194]);
    seg.full_stop_rate_override = Some(0.547);
    seg.prop_left_turn_lanes = Some(0.33);
    seg
}

/// Step 2 free-flow speed chain against Example Problem 1: S_0 = 42.05,
/// f_CS = −0.329, f_A = −0.941, S_fo = 40.78 mi/h (Exhibit 30-36).
#[test]
fn test_step_2_base_free_flow_speed_example_problem_1() {
    let mut seg = example_problem_1_segment();
    seg.step_2_running_time();
    assert_near!(seg.speed_constant_mph.unwrap(), 42.05, 0.001, "S_0");
    assert_near!(seg.f_cs_mph.unwrap(), -0.329, 0.001, "f_CS");
    // D_a = 5,280 × 8 / 1,750 = 24.137 points/mi; f_A = −0.078 × 24.137/2.
    assert_near!(seg.f_a_mph.unwrap(), -0.9413, 0.001, "f_A");
    assert_near!(seg.f_pk_mph.unwrap(), 0.0, 1e-9, "f_pk");
    assert_near!(seg.base_ffs_mph.unwrap(), 40.78, 0.005, "S_fo (Exhibit 30-36)");
    // f_L = 1.02 − 4.7 (40.78 − 19.5)/1,800 = 0.9644; S_f = 39.33 mi/h.
    assert_near!(seg.f_l.unwrap(), 0.9644, 0.0005, "f_L");
    assert_near!(seg.free_flow_speed_mph.unwrap(), 39.33, 0.005, "S_f");
}

/// Step 2 running time against Example Problem 1: f_v = 1.034,
/// t_R = 33.54 s, running speed 36.59 mi/h (Exhibit 30-36).
#[test]
fn test_step_2_running_time_example_problem_1() {
    let mut seg = example_problem_1_segment();
    let t_r = seg.step_2_running_time();
    assert_near!(seg.f_v.unwrap(), 1.0340, 0.0005, "f_v");
    assert_near!(
        seg.access_point_delay_total_s.unwrap(),
        0.387,
        1e-9,
        "sum d_ap (Exhibit 30-35)"
    );
    assert_near!(t_r, 33.54, 0.01, "t_R (Exhibit 30-36)");
    assert_near!(
        seg.running_speed_mph.unwrap(),
        36.59,
        0.01,
        "running speed (Exhibit 30-36)"
    );
}

/// Equation 18-5 lower bound: the free-flow speed is never below the
/// posted speed limit.
#[test]
fn test_equation_18_5_speed_limit_floor() {
    let mut seg = UrbanSegment::new(400.0, 1, 45.0, 300.0, BoundaryControlType::Signalized);
    seg.proportion_with_curb = 1.0;
    seg.proportion_on_street_parking = 1.0;
    seg.n_access_points_subject = 10.0;
    seg.n_access_points_opposing = 10.0;
    seg.step_2_running_time();
    // Heavy adjustments plus a very short signal spacing would predict
    // S_fo f_L below the speed limit; Equation 18-5 floors it.
    assert!(seg.free_flow_speed_mph.unwrap() >= 45.0);
}

/// Equation 18-4 cap: f_L <= 1.0 (long spacing) and the max(L_s, 400)
/// divisor floor.
#[test]
fn test_equation_18_4_bounds() {
    // Long spacing: 1.02 - small positive => capped at 1.0.
    assert_near!(signal_spacing_adjustment(20.0, 100_000.0), 1.0, 1e-9, "f_L cap");
    // The divisor floors at 400 ft.
    assert_near!(
        signal_spacing_adjustment(40.0, 100.0),
        signal_spacing_adjustment(40.0, 400.0),
        1e-12,
        "f_L divisor floor"
    );
}

/// Equation 18-6 at the Example Problem 1 operating point, plus the
/// Exhibit 18-12 anchor ("At a flow rate of 1,000 veh/h/ln, each trend
/// line shows a reduction of about 2.5 mi/h relative to the free-flow
/// speed").
#[test]
fn test_equation_18_6_proximity() {
    assert_near!(proximity_adjustment(1_150.0, 2, 39.3294), 1.0340, 0.0005, "f_v EP1");
    assert_near!(proximity_adjustment(0.0, 2, 40.0), 1.0, 1e-9, "f_v at zero flow");
    // 1,000 veh/h/ln at S_f = 40 mi/h: running speed = S_f/f_v.
    let f_v = proximity_adjustment(1_000.0, 1, 40.0);
    let reduction = 40.0 - 40.0 / f_v;
    assert!(
        (2.0..3.2).contains(&reduction),
        "Exhibit 18-12 anchor: speed reduction {reduction} mi/h at 1,000 veh/h/ln"
    );
}

/// Step 3: P = R_p g/C (Equation 19-15). With the Example Problem 1
/// platoon ratio of 1.333 and g/C = 48.84/100, P = 0.651 — the published
/// "Proportion Arriving On Green" for the eastbound through movement at
/// Signalized Intersection 1 (Exhibit 30-32).
#[test]
fn test_step_3_proportion_arriving_green() {
    let mut seg = example_problem_1_segment();
    seg.effective_green_s = Some(48.84);
    seg.platoon_ratio = Some(1.333);
    assert_near!(
        seg.step_3_proportion_arriving_green().unwrap(),
        0.651,
        0.001,
        "P (Exhibit 30-32)"
    );
    // Arrival type 4 maps to R_p = 1.33 (Exhibit 19-13; Example Problem 1
    // entered 1.333 directly), so P = 1.33 × 0.4884 = 0.6496.
    seg.platoon_ratio = None;
    seg.arrival_type = Some(4);
    assert_near!(seg.step_3_proportion_arriving_green().unwrap(), 0.6496, 0.001, "P via AT4");
    // No arrival input: uniform arrivals, P = g/C.
    seg.arrival_type = None;
    assert_near!(seg.step_3_proportion_arriving_green().unwrap(), 0.4884, 0.0001, "P = g/C");
    // Not signalized: step is skipped.
    seg.control = BoundaryControlType::Uncontrolled;
    assert_eq!(seg.step_3_proportion_arriving_green(), None);
}

/// Step 5: uncontrolled through movement has 0.0 s/veh through control
/// delay (Chapter 18 text); controlled boundaries pass the input through.
#[test]
fn test_step_5_through_delay() {
    let mut seg = example_problem_1_segment();
    assert_near!(seg.step_5_through_delay(), 18.310, 1e-9, "d_t signalized");
    seg.control = BoundaryControlType::Uncontrolled;
    assert_near!(seg.step_5_through_delay(), 0.0, 1e-9, "d_t uncontrolled");
}

/// Equation 18-11 hand check with Chapter 31-style inputs consistent with
/// Example Problem 1 (v_th = 968 veh/h, C = 100 s, g = 48.63 s,
/// s = 1,900 veh/h/ln, N_th = 2):
/// flow ratio = 968×100/(2×1,900×48.63) = 0.52383;
/// h = 3,600 [7.0/(0.52383×48.63×1,900) + 2×0.35/(968×100)] = 0.5467
/// stops/veh — the same magnitude as the published segment stop rate of
/// 0.547 stops/veh (Exhibit 30-36).
#[test]
fn test_equation_18_11_stop_rate() {
    let h = full_stop_rate_signalized(7.0, 0.35, 48.63, 1_900.0, 968.0, 100.0, 2);
    assert_near!(h, 0.5467, 0.0005, "h (Equation 18-11)");
    // Degenerate inputs are guarded.
    assert_eq!(full_stop_rate_signalized(7.0, 0.35, 0.0, 1_900.0, 968.0, 100.0, 2), 0.0);
    assert_eq!(full_stop_rate_signalized(7.0, 0.35, 48.63, 1_900.0, 0.0, 100.0, 2), 0.0);
}

/// Step 6 defaults per the Chapter 18 text: STOP 1.0 stops/veh,
/// uncontrolled 0.0, YIELD = through v/c.
#[test]
fn test_step_6_stop_rate_defaults() {
    let mut seg = example_problem_1_segment();
    seg.full_stop_rate_override = None;
    seg.control = BoundaryControlType::AllWayStop;
    assert_eq!(seg.step_6_stop_rate(), Some(1.0));
    seg.control = BoundaryControlType::Uncontrolled;
    assert_eq!(seg.step_6_stop_rate(), Some(0.0));
    seg.control = BoundaryControlType::Roundabout;
    let h = seg.step_6_stop_rate().unwrap();
    assert_near!(h, 968.0 / 1_848.0, 1e-9, "h YIELD = v/c");
    // Signalized without Chapter 31 inputs: undefined.
    seg.control = BoundaryControlType::Signalized;
    assert_eq!(seg.step_6_stop_rate(), None);
    // Equation 18-11 path through the struct.
    seg.stopped_vehicles_veh_ln = Some(7.0);
    seg.queue2_veh_ln = Some(0.35);
    seg.sat_flow_veh_h_ln = Some(1_900.0);
    assert_near!(seg.step_6_stop_rate().unwrap(), 0.5467, 0.0005, "h via struct");
}

/// Steps 7–10 against Example Problem 1 (Exhibit 30-36): travel speed
/// 23.67 mi/h, spatial stop rate 1.61 stops/mi, LOS C, traveler
/// perception score 2.53.
#[test]
fn test_steps_7_to_10_example_problem_1() {
    let mut seg = example_problem_1_segment();
    seg.analyze();
    assert_near!(seg.travel_speed_mph.unwrap(), 23.67, 0.01, "S_T,seg (Exhibit 30-36)");
    assert_near!(
        seg.spatial_stop_rate_stops_mi.unwrap(),
        1.61,
        0.01,
        "H_seg (Exhibit 30-36)"
    );
    assert_near!(seg.vc_ratio.unwrap(), 0.52, 0.005, "through v/c (Exhibit 30-36)");
    assert_eq!(seg.los, Some(LevelOfService::C), "LOS (Exhibit 30-36)");
    assert_near!(
        seg.perception_score.unwrap(),
        2.53,
        0.01,
        "I_a,seg (Exhibit 30-36)"
    );
}

/// Equation 18-2: c_th = 1,800 (N_th − 1 + p*_0,j).
#[test]
fn test_equation_18_2_uncontrolled_capacity() {
    assert_near!(through_capacity_uncontrolled(2, 1.0), 3_600.0, 1e-9, "c_th, bay provided");
    assert_near!(through_capacity_uncontrolled(1, 0.8), 1_440.0, 1e-9, "c_th, 1 lane");
    assert_near!(through_capacity_uncontrolled(3, 0.5), 4_500.0, 1e-9, "c_th, 3 lanes");
}

/// Equation 18-10 shared-lane weighted through delay, hand-computed:
/// (30×400×2 + 40×300×(1−0.3) + 20×250×(1−0.2)) / 1,000 = 36.4 s/veh.
#[test]
fn test_equation_18_10_shared_lane_delay() {
    let d_t = shared_lane_through_delay(
        1_000.0,
        Some((30.0, 400.0, 2)),
        Some((40.0, 300.0, 0.3)),
        Some((20.0, 250.0, 0.2)),
    );
    assert_near!(d_t, 36.4, 1e-9, "d_t (Equation 18-10)");
    assert_eq!(shared_lane_through_delay(0.0, None, None, None), 0.0);
}

/// Equations 18-12/18-13/18-14 weighting, hand-computed:
/// (5×2 + 4×(1−0.25) + 3×(1−0.5)) / 3 = 4.8333.
#[test]
fn test_equations_18_12_to_18_14_weighting() {
    let x = weighted_through_lane_value(
        3,
        Some((5.0, 2)),
        Some((4.0, 0.25)),
        Some((3.0, 0.5)),
    );
    assert_near!(x, 14.5 / 3.0, 1e-9, "weighted per-lane value");
}

/// Equation 18-1: N_ap,s = 0.5 D_a L / 5,280.
#[test]
fn test_equation_18_1_default_access_points() {
    // D_a = 24 points/mi over 2,640 ft: 0.5 × 24 × 2,640 / 5,280 = 6.
    assert_near!(default_access_point_count(24.0, 2_640.0), 6.0, 1e-9, "N_ap,s");
}

/// Equations 18-17 through 18-22 at the Example Problem 1 operating point.
#[test]
fn test_perception_score_example_problem_1() {
    assert_near!(traveler_perception_score(1.6045, 0.33), 2.53, 0.005, "I_a,seg");
    // A zero-stop segment with left-turn bays everywhere scores near the
    // "best perceived service" end (≤ 2.0 indicates the best service).
    assert!(traveler_perception_score(0.0, 1.0) < 2.5);
}

/// Full pipeline on a segment with an uncontrolled (TWSC major-street)
/// downstream boundary: no first-term acceleration time (f_x = 0), zero
/// through delay and stop rate, and travel speed equal to the running
/// speed.
#[test]
fn test_analyze_uncontrolled_boundary() {
    let mut seg = UrbanSegment::new(2_000.0, 1, 40.0, 700.0, BoundaryControlType::Uncontrolled);
    seg.proportion_with_curb = 0.0;
    seg.n_access_points_subject = 2.0;
    seg.n_access_points_opposing = 2.0;
    // Equation 18-2 with a left-turn bay on the major street (p*_0,j = 1).
    seg.through_capacity_veh_h = Some(through_capacity_uncontrolled(1, 1.0));
    seg.analyze();
    assert_eq!(seg.through_delay_s, Some(0.0));
    assert_eq!(seg.full_stop_rate, Some(0.0));
    assert_near!(
        seg.travel_speed_mph.unwrap(),
        seg.running_speed_mph.unwrap(),
        1e-9,
        "travel speed = running speed"
    );
    assert!(seg.travel_speed_mph.unwrap() <= seg.free_flow_speed_mph.unwrap());
    assert!(seg.los.is_some());
    assert_eq!(seg.demand_exceeds_capacity, Some(false));
}

/// Step 1 capacity-constraint flag.
#[test]
fn test_step_1_capacity_constraint_flag() {
    let mut seg = example_problem_1_segment();
    seg.through_demand_veh_h = 2_000.0;
    seg.step_1_demand_adjustment();
    assert_eq!(seg.demand_exceeds_capacity, Some(true));
}

/// Exhibit 18-1 v/c rule end-to-end: demand above capacity forces LOS F
/// even at a high travel speed.
#[test]
fn test_step_9_vc_rule() {
    let mut seg = example_problem_1_segment();
    seg.through_demand_veh_h = 1_900.0;
    seg.midsegment_flow_veh_h = Some(1_900.0);
    seg.analyze();
    assert!(seg.vc_ratio.unwrap() > 1.0);
    assert_eq!(seg.los, Some(LevelOfService::F));
}

/// JSON round-trip through the fixture format.
#[test]
fn test_serde_round_trip() {
    let mut seg = example_problem_1_segment();
    seg.analyze();
    let json = seg.to_json().unwrap();
    let back = UrbanSegment::from_json(&json).unwrap();
    assert_eq!(back.los, seg.los);
    assert_eq!(back.travel_speed_mph, seg.travel_speed_mph);
}
