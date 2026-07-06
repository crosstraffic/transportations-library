//! Per-step unit tests for the HCM Chapter 22 roundabout methodology, using
//! the intermediate values published in HCM Chapter 33, Example Problems 1
//! and 2.

use super::roundabouts::*;

fn approx(a: f64, b: f64, tol: f64) -> bool {
    (a - b).abs() <= tol
}

/// HCM Chapter 33, Example Problem 1: four-leg single-lane roundabout with
/// a yielding WB bypass and a nonyielding SB bypass; 2% heavy vehicles;
/// PHF = 0.94; 50 p/h across the south leg (NB entry).
fn example_problem_1() -> Roundabouts {
    let nb = RoundaboutApproach {
        v_u: 30.0,
        v_l: 105.0,
        v_t: 210.0,
        v_r: 50.0,
        heavy_vehicle_pct: 2.0,
        n_ped: 50.0,
        ..Default::default()
    };
    let sb = RoundaboutApproach {
        v_u: 20.0,
        v_l: 175.0,
        v_t: 95.0,
        v_r: 580.0,
        heavy_vehicle_pct: 2.0,
        bypass: BypassType::NonYielding,
        ..Default::default()
    };
    let eb = RoundaboutApproach {
        v_u: 50.0,
        v_l: 190.0,
        v_t: 280.0,
        v_r: 85.0,
        heavy_vehicle_pct: 2.0,
        ..Default::default()
    };
    let wb = RoundaboutApproach {
        v_u: 20.0,
        v_l: 110.0,
        v_t: 395.0,
        v_r: 610.0,
        heavy_vehicle_pct: 2.0,
        bypass: BypassType::Yielding,
        ..Default::default()
    };
    let mut r = Roundabouts::new(nb, sb, eb, wb);
    r.phf = Some(0.94);
    r
}

/// HCM Chapter 33, Example Problem 2: multilane roundabout. NB single-lane
/// entry against two circulating lanes; SB two lanes (L+T | R) against two
/// circulating lanes; EB and WB two lanes (LT | TR) against one circulating
/// lane. 5% heavy vehicles EB/WB, 2% NB/SB; PHF = 0.95.
fn example_problem_2() -> Roundabouts {
    let nb = RoundaboutApproach {
        v_l: 50.0,
        v_t: 60.0,
        v_r: 120.0,
        heavy_vehicle_pct: 2.0,
        entry_lanes: 1,
        circulating_lanes: 2,
        ..Default::default()
    };
    let sb = RoundaboutApproach {
        v_l: 240.0,
        v_t: 60.0,
        v_r: 400.0,
        heavy_vehicle_pct: 2.0,
        entry_lanes: 2,
        circulating_lanes: 2,
        lane_assignment: LaneAssignment::LeftThroughAndRight,
        ..Default::default()
    };
    let eb = RoundaboutApproach {
        v_l: 230.0,
        v_t: 420.0,
        v_r: 80.0,
        heavy_vehicle_pct: 5.0,
        entry_lanes: 2,
        circulating_lanes: 1,
        lane_assignment: LaneAssignment::LeftThroughAndThroughRight,
        ..Default::default()
    };
    let wb = RoundaboutApproach {
        v_l: 400.0,
        v_t: 250.0,
        v_r: 90.0,
        heavy_vehicle_pct: 5.0,
        entry_lanes: 2,
        circulating_lanes: 1,
        lane_assignment: LaneAssignment::LeftThroughAndThroughRight,
        ..Default::default()
    };
    let mut r = Roundabouts::new(nb, sb, eb, wb);
    r.phf = Some(0.95);
    r
}

// ═══════════════════════════════════════════════════════════════════════════════
// Capacity equations (Equations 22-1 through 22-7)
// ═══════════════════════════════════════════════════════════════════════════════

/// Equation 22-1 at the Example Problem 1 conflicting flows:
/// c(796) = 613, c(769) = 630, c(487) = 840, c(655) = 708 pc/h.
#[test]
fn test_capacity_single_lane_equation_22_1() {
    assert!(approx(capacity_single_lane(796.0), 613.0, 1.5));
    assert!(approx(capacity_single_lane(769.0), 630.0, 1.5));
    assert!(approx(capacity_single_lane(487.0), 840.0, 1.5));
    assert!(approx(capacity_single_lane(655.0), 708.0, 1.5));
    // Zero conflicting flow: capacity equals the intercept
    assert!(approx(capacity_single_lane(0.0), 1380.0, 1e-9));
}

/// Equations 22-2 through 22-5 at the Example Problem 2 conflicting flows.
#[test]
fn test_multilane_capacity_equations() {
    // Equation 22-3: c(976) = 619 pc/h (NB single entry, two circulating)
    assert!(approx(capacity_one_lane_entry_two_circ(976.0), 619.0, 1.5));
    // Equation 22-4: c(772) = 737 pc/h (SB right lane)
    assert!(approx(capacity_two_lane_entry_two_circ_right(772.0), 737.0, 1.5));
    // Equation 22-5: c(772) = 664 pc/h (SB left lane)
    assert!(approx(capacity_two_lane_entry_two_circ_left(772.0), 664.0, 1.5));
    // Equation 22-2: c(764) = 709 pc/h (EB lanes), c(372) = 1,012 pc/h (WB)
    assert!(approx(capacity_two_lane_entry_one_circ(764.0), 709.0, 1.5));
    assert!(approx(capacity_two_lane_entry_one_circ(372.0), 1012.0, 1.5));
}

/// Equation 22-6: bypass capacity c(454) = 868 pc/h (Example Problem 1).
#[test]
fn test_bypass_capacity_equations() {
    assert!(approx(capacity_bypass_one_exit_lane(454.0), 868.0, 1.5));
    // Equation 22-7 uses the two-circulating-lane coefficients
    assert!(approx(
        capacity_bypass_two_exit_lanes(500.0),
        capacity_two_lane_entry_two_circ_right(500.0),
        1e-9
    ));
}

/// Equations 22-21 through 22-23: calibration reproduces Equation 22-1
/// with t_f = 2.609 s and t_c = 4.976 s (A = 1,380, B = 1.02e-3).
#[test]
fn test_calibration_equations() {
    let t_f = 3_600.0 / 1_380.0;
    let a = calibrated_intercept_a(t_f);
    assert!(approx(a, 1_380.0, 1e-9));
    // Solve Equation 22-23 for t_c giving B = 1.02e-3
    let t_c = 1.02e-3 * 3_600.0 + t_f / 2.0;
    let b = calibrated_slope_b(t_c, t_f);
    assert!(approx(b, 1.02e-3, 1e-12));
    assert!(approx(
        capacity_exponential(a, b, 796.0),
        capacity_single_lane(796.0),
        1e-9
    ));
}

// ═══════════════════════════════════════════════════════════════════════════════
// Pedestrian impedance (Exhibits 22-18 and 22-20)
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_ped_factor_one_lane_exhibit_22_18() {
    // Example Problem 1: n_ped = 50 -> f_ped = 1 - 0.000137(50) = 0.993
    assert!(approx(ped_factor_one_lane(50.0, 796.0), 0.993, 0.001));
    // High conflicting flow: no pedestrian effect
    assert!(approx(ped_factor_one_lane(200.0, 900.0), 1.0, 1e-12));
    // No pedestrians: no effect
    assert!(approx(ped_factor_one_lane(0.0, 400.0), 1.0, 1e-12));
    // Heavy pedestrians branch is bounded to [0, 1] and decreasing
    let f1 = ped_factor_one_lane(200.0, 400.0);
    let f2 = ped_factor_one_lane(400.0, 400.0);
    assert!(f1 <= 1.0 && f2 <= f1 && f2 >= 0.0);
}

#[test]
fn test_ped_factor_two_lane_exhibit_22_20() {
    assert!(approx(ped_factor_two_lane(0.0, 400.0), 1.0, 1e-12));
    // Continuous across n_ped = 100
    let below = ped_factor_two_lane(99.999, 500.0);
    let at = ped_factor_two_lane(100.0, 500.0);
    assert!(approx(below, at, 1e-3));
    // Decreasing with pedestrian volume
    assert!(ped_factor_two_lane(300.0, 500.0) < ped_factor_two_lane(150.0, 500.0));
}

// ═══════════════════════════════════════════════════════════════════════════════
// Example Problem 1 steps
// ═══════════════════════════════════════════════════════════════════════════════

/// Steps 1–3: adjusted flows and circulating/exiting flows.
/// v_c,NB = 796; v_c,SB = 769; v_c,EB = 487; v_c,WB = 655;
/// v_ex,NB = 454 pc/h (published values, computed here without
/// intermediate rounding: tolerance 3 pc/h).
#[test]
fn test_ep1_steps1_3_conflicting_flows() {
    let mut r = example_problem_1();
    r.step1_2_flow_rates_pce();
    r.step3_conflicting_flows();
    assert!(approx(r.nb.circulating_flow_pce.unwrap(), 796.0, 3.0));
    assert!(approx(r.sb.circulating_flow_pce.unwrap(), 769.0, 3.0));
    assert!(approx(r.eb.circulating_flow_pce.unwrap(), 487.0, 3.0));
    assert!(approx(r.wb.circulating_flow_pce.unwrap(), 655.0, 3.0));
    assert!(approx(r.wb.bypass_conflicting_flow_pce.unwrap(), 454.0, 3.0));
}

/// Step 4: entry flows v_e,NB = 428, v_e,SB = 314, v_e,EB = 656,
/// v_e,WB = 568 pc/h (right turns on SB/WB use the bypass lanes).
#[test]
fn test_ep1_step4_entry_flows() {
    let mut r = example_problem_1();
    r.step1_2_flow_rates_pce();
    let (_, nb) = r.entry_lane_flows_pce(Leg::NB);
    let (_, sb) = r.entry_lane_flows_pce(Leg::SB);
    let (_, eb) = r.entry_lane_flows_pce(Leg::EB);
    let (_, wb) = r.entry_lane_flows_pce(Leg::WB);
    assert!(approx(nb, 428.0, 2.0));
    assert!(approx(sb, 314.0, 2.0));
    assert!(approx(eb, 656.0, 2.0));
    assert!(approx(wb, 568.0, 2.0));
}

/// Steps 5–9: capacities (veh/h), v/c ratios, and delays.
/// c_NB = 597, c_SB = 618, c_EB = 824, c_WB = 694, c_bypass,WB = 851;
/// x_NB = 0.70, x_WB = 0.80; d_NB = 22.6, d_WB = 26.8, d_bypass = 20.2.
#[test]
fn test_ep1_steps5_9_capacity_delay() {
    let mut r = example_problem_1();
    r.analyze();
    let nb = &r.nb.lanes[0];
    assert!(approx(nb.capacity_veh, 597.0, 5.0));
    assert!(approx(nb.v_c_ratio, 0.70, 0.01));
    assert!(approx(nb.control_delay, 22.6, 0.5));
    assert_eq!(nb.los, 'C');
    let sb = &r.sb.lanes[0];
    assert!(approx(sb.capacity_veh, 618.0, 5.0));
    assert!(approx(sb.control_delay, 14.0, 0.5));
    assert_eq!(sb.los, 'B');
    let eb = &r.eb.lanes[0];
    assert!(approx(eb.capacity_veh, 824.0, 5.0));
    assert!(approx(eb.control_delay, 22.0, 0.5));
    let wb = &r.wb.lanes[0];
    assert!(approx(wb.capacity_veh, 694.0, 5.0));
    assert!(approx(wb.control_delay, 26.8, 0.5));
    assert_eq!(wb.los, 'D');
    let bp = r.wb.bypass_lane.as_ref().unwrap();
    assert!(approx(bp.capacity_veh, 851.0, 5.0));
    assert!(approx(bp.control_delay, 20.2, 0.5));
    assert_eq!(bp.los, 'C');
    // Nonyielding SB bypass: delay assumed 0, LOS A
    let sb_bp = r.sb.bypass_lane.as_ref().unwrap();
    assert!(approx(sb_bp.control_delay, 0.0, 1e-12));
    assert_eq!(sb_bp.los, 'A');
}

/// Steps 11–12: approach delays d_WB = 23.3 s (LOS C), d_SB = 4.7 s
/// (LOS A); intersection delay 17.5 s (LOS C); Q95,NB = 5.7 veh.
#[test]
fn test_ep1_steps11_12_aggregation() {
    let mut r = example_problem_1();
    r.analyze();
    assert!(approx(r.wb.control_delay.unwrap(), 23.3, 0.5));
    assert_eq!(r.wb.los.unwrap(), 'C');
    assert!(approx(r.sb.control_delay.unwrap(), 4.7, 0.5));
    assert_eq!(r.sb.los.unwrap(), 'A');
    assert!(approx(r.intersection_delay.unwrap(), 17.5, 0.5));
    assert_eq!(r.intersection_los.unwrap(), 'C');
    assert!(approx(r.nb.lanes[0].queue_95, 5.7, 0.3));
}

// ═══════════════════════════════════════════════════════════════════════════════
// Example Problem 2 steps
// ═══════════════════════════════════════════════════════════════════════════════

/// Steps 1–3: v_c,EB = 764, v_c,WB = 372, v_c,NB = 976, v_c,SB = 772 pc/h.
#[test]
fn test_ep2_steps1_3_conflicting_flows() {
    let mut r = example_problem_2();
    r.step1_2_flow_rates_pce();
    r.step3_conflicting_flows();
    assert!(approx(r.eb.circulating_flow_pce.unwrap(), 764.0, 3.0));
    assert!(approx(r.wb.circulating_flow_pce.unwrap(), 372.0, 3.0));
    assert!(approx(r.nb.circulating_flow_pce.unwrap(), 976.0, 3.0));
    assert!(approx(r.sb.circulating_flow_pce.unwrap(), 772.0, 3.0));
}

/// Step 4: lane flows. EB (no de facto lanes): right = 427, left = 379;
/// WB (de facto left-turn lane): left = 442, right = 376; SB: left = 322,
/// right = 429 pc/h.
#[test]
fn test_ep2_step4_lane_flows() {
    let mut r = example_problem_2();
    r.step1_2_flow_rates_pce();
    let (eb_l, eb_r) = r.entry_lane_flows_pce(Leg::EB);
    assert!(approx(eb_r, 427.0, 2.0));
    assert!(approx(eb_l, 379.0, 2.0));
    let (wb_l, wb_r) = r.entry_lane_flows_pce(Leg::WB);
    assert!(approx(wb_l, 442.0, 2.0));
    assert!(approx(wb_r, 376.0, 2.0));
    let (sb_l, sb_r) = r.entry_lane_flows_pce(Leg::SB);
    assert!(approx(sb_l, 322.0, 2.0));
    assert!(approx(sb_r, 429.0, 2.0));
    let (_, nb_r) = r.entry_lane_flows_pce(Leg::NB);
    assert!(approx(nb_r, 247.0, 2.0));
}

/// Steps 5–9: capacities and delays per lane. c_NB = 607, c_SB,L = 651,
/// c_SB,R = 723, c_EB = 675, c_WB = 964 veh/h; d_NB = 11.8 s;
/// d_EB,R = 16.1 s (LOS C); d_WB,R = 7.8 s (LOS A).
#[test]
fn test_ep2_steps5_9_capacity_delay() {
    let mut r = example_problem_2();
    r.analyze();
    assert!(approx(r.nb.lanes[0].capacity_veh, 607.0, 5.0));
    assert!(approx(r.nb.lanes[0].control_delay, 11.8, 0.5));
    assert_eq!(r.nb.lanes[0].los, 'B');
    assert!(approx(r.sb.lanes[0].capacity_veh, 651.0, 5.0)); // left
    assert!(approx(r.sb.lanes[1].capacity_veh, 723.0, 5.0)); // right
    assert!(approx(r.sb.lanes[0].control_delay, 13.0, 0.5));
    assert!(approx(r.sb.lanes[1].control_delay, 14.6, 0.5));
    assert!(approx(r.eb.lanes[0].capacity_veh, 675.0, 5.0));
    assert!(approx(r.eb.lanes[1].capacity_veh, 675.0, 5.0));
    assert!(approx(r.eb.lanes[0].control_delay, 14.0, 0.5));
    assert!(approx(r.eb.lanes[1].control_delay, 16.1, 0.5));
    assert_eq!(r.eb.lanes[1].los, 'C');
    assert!(approx(r.wb.lanes[0].capacity_veh, 964.0, 5.0));
    assert!(approx(r.wb.lanes[0].control_delay, 8.8, 0.5));
    assert!(approx(r.wb.lanes[1].control_delay, 7.8, 0.5));
    assert_eq!(r.wb.lanes[1].los, 'A');
}

/// Steps 11–12: approach delays d_SB = 13.9, d_EB = 15.1, d_WB = 8.3 s;
/// intersection 12.3 s (LOS B); Q95,NB = 1.9 veh.
#[test]
fn test_ep2_steps11_12_aggregation() {
    let mut r = example_problem_2();
    r.analyze();
    assert!(approx(r.sb.control_delay.unwrap(), 13.9, 0.5));
    assert!(approx(r.eb.control_delay.unwrap(), 15.1, 0.5));
    assert!(approx(r.wb.control_delay.unwrap(), 8.3, 0.5));
    assert!(approx(r.intersection_delay.unwrap(), 12.3, 0.5));
    assert_eq!(r.intersection_los.unwrap(), 'B');
    assert!(approx(r.nb.lanes[0].queue_95, 1.9, 0.2));
}

/// De facto lane checks of Exhibit 22-14: heavy right turns create a
/// de facto right-turn lane on an LT|TR entry.
#[test]
fn test_de_facto_right_turn_lane() {
    let mut r = example_problem_2();
    // Give EB overwhelming right-turn demand
    r.eb.v_l = 10.0;
    r.eb.v_t = 20.0;
    r.eb.v_r = 500.0;
    r.step1_2_flow_rates_pce();
    let (left, right) = r.entry_lane_flows_pce(Leg::EB);
    let [_, l, t, rr] = r.eb.flows_pce.unwrap();
    assert!(approx(left, l + t, 1e-9));
    assert!(approx(right, rr, 1e-9));
}

/// Serde round-trip.
#[test]
fn test_serde_roundtrip() {
    let mut r = example_problem_1();
    r.analyze();
    let json = r.to_json().unwrap();
    let back = Roundabouts::from_json(&json).unwrap();
    assert!(approx(
        back.intersection_delay.unwrap(),
        r.intersection_delay.unwrap(),
        1e-9
    ));
}
