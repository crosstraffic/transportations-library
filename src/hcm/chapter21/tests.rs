//! Per-step unit tests for the HCM Chapter 21 AWSC methodology, using the
//! intermediate values published in HCM Chapter 32, AWSC Example Problems 1
//! and 2.

use super::awsc::*;

fn approx(a: f64, b: f64, tol: f64) -> bool {
    (a - b).abs() <= tol
}

/// HCM Chapter 32, AWSC Example Problem 1: three-leg (T) single-lane AWSC
/// intersection, 2% heavy vehicles, PHF = 0.95.
/// EB: L = 50, T = 300; WB: T = 300, R = 100; SB (minor stem): L = 100,
/// R = 50 veh/h.
fn example_problem_1() -> Awsc {
    let eb = AwscApproach {
        lanes: vec![AwscLane::new(50.0, 300.0, 0.0)],
        heavy_vehicle_pct: 2.0,
        ..Default::default()
    };
    let wb = AwscApproach {
        lanes: vec![AwscLane::new(0.0, 300.0, 100.0)],
        heavy_vehicle_pct: 2.0,
        ..Default::default()
    };
    let nb = AwscApproach::default(); // no south leg
    let sb = AwscApproach {
        lanes: vec![AwscLane::new(100.0, 0.0, 50.0)],
        heavy_vehicle_pct: 2.0,
        ..Default::default()
    };
    let mut awsc = Awsc::new(eb, wb, nb, sb);
    awsc.phf = Some(0.95);
    awsc
}

/// HCM Chapter 32, AWSC Example Problem 2: four-leg multilane AWSC
/// intersection; two-lane approaches (L | TR) on the east/west legs and
/// three-lane approaches (L | T | R) on the north/south legs; 2% heavy
/// vehicles; 15-min volumes (x4 = flow rates, so no PHF).
fn example_problem_2() -> Awsc {
    let eb = AwscApproach {
        lanes: vec![AwscLane::new(56.0, 0.0, 0.0), AwscLane::new(0.0, 152.0, 64.0)],
        heavy_vehicle_pct: 2.0,
        ..Default::default()
    };
    let wb = AwscApproach {
        lanes: vec![
            AwscLane::new(156.0, 0.0, 0.0),
            AwscLane::new(0.0, 92.0, 72.0),
        ],
        heavy_vehicle_pct: 2.0,
        ..Default::default()
    };
    let nb = AwscApproach {
        lanes: vec![
            AwscLane::new(76.0, 0.0, 0.0),
            AwscLane::new(0.0, 164.0, 0.0),
            AwscLane::new(0.0, 0.0, 116.0),
        ],
        heavy_vehicle_pct: 2.0,
        ..Default::default()
    };
    let sb = AwscApproach {
        lanes: vec![
            AwscLane::new(48.0, 0.0, 0.0),
            AwscLane::new(0.0, 124.0, 0.0),
            AwscLane::new(0.0, 0.0, 88.0),
        ],
        heavy_vehicle_pct: 2.0,
        ..Default::default()
    };
    Awsc::new(eb, wb, nb, sb)
}

// ═══════════════════════════════════════════════════════════════════════════════
// Exhibit 21-11 geometry groups
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_geometry_group_exhibit_21_11() {
    use GeometryGroup as G;
    // Single-lane approaches all around: Group 1 (Example Problem 1)
    assert_eq!(geometry_group(false, 1, 1, 1), G::G1);
    assert_eq!(geometry_group(true, 1, 0, 1), G::G1);
    assert_eq!(geometry_group(true, 1, 1, 2), G::G2);
    // (1, 2, 1): 3a at a T, 4a at a four-leg
    assert_eq!(geometry_group(false, 1, 2, 1), G::G3a);
    assert_eq!(geometry_group(true, 1, 2, 1), G::G4a);
    // (1, 2, 2): 3b at a T, 4b at a four-leg
    assert_eq!(geometry_group(false, 1, 2, 2), G::G3b);
    assert_eq!(geometry_group(true, 1, 2, 2), G::G4b);
    // Group 5 rows
    assert_eq!(geometry_group(true, 1, 0, 3), G::G5);
    assert_eq!(geometry_group(true, 1, 3, 1), G::G5);
    assert_eq!(geometry_group(true, 2, 1, 2), G::G5);
    assert_eq!(geometry_group(true, 3, 1, 3), G::G5);
    assert_eq!(geometry_group(true, 3, 3, 1), G::G5);
    // Group 6 rows
    assert_eq!(geometry_group(true, 1, 3, 2), G::G6);
    assert_eq!(geometry_group(true, 1, 2, 3), G::G6);
    assert_eq!(geometry_group(true, 2, 3, 2), G::G6);
    assert_eq!(geometry_group(true, 2, 2, 3), G::G6);
    assert_eq!(geometry_group(true, 3, 3, 3), G::G6); // Example Problem 2
}

#[test]
fn test_move_up_time() {
    assert_eq!(GeometryGroup::G1.move_up_time(), 2.0);
    assert_eq!(GeometryGroup::G4b.move_up_time(), 2.0);
    assert_eq!(GeometryGroup::G5.move_up_time(), 2.3);
    assert_eq!(GeometryGroup::G6.move_up_time(), 2.3);
}

// ═══════════════════════════════════════════════════════════════════════════════
// Exhibit 21-12 / Equation 21-13 headway adjustments
// ═══════════════════════════════════════════════════════════════════════════════

/// Example Problem 1 Step 4: h_adj,EB = 0.063, h_adj,WB = -0.116,
/// h_adj,SB = -0.034 s (published values use flow rates rounded to
/// integers; exact arithmetic is within 0.001 s).
#[test]
fn test_ep1_step4_headway_adjustments() {
    let mut a = example_problem_1();
    a.step1_2_flow_rates();
    a.step3_geometry_groups();
    a.step4_headway_adjustments();
    assert_eq!(a.eb.geometry_group, Some(GeometryGroup::G1));
    assert!(approx(a.eb.lanes[0].headway_adjustment.unwrap(), 0.063, 0.001));
    assert!(approx(a.wb.lanes[0].headway_adjustment.unwrap(), -0.116, 0.001));
    // Published -0.034 uses flow rates rounded to integers (105/158); the
    // exact turning proportions give -0.0327.
    assert!(approx(a.sb.lanes[0].headway_adjustment.unwrap(), -0.034, 0.002));
}

/// Example Problem 2 Step 4: h_adj,EB,1 = 0.534 (exclusive left),
/// h_adj,EB,2 = -0.173 s (shared through-right).
#[test]
fn test_ep2_step4_headway_adjustments() {
    let mut a = example_problem_2();
    a.step1_2_flow_rates();
    a.step3_geometry_groups();
    a.step4_headway_adjustments();
    assert_eq!(a.eb.geometry_group, Some(GeometryGroup::G6));
    assert_eq!(a.nb.geometry_group, Some(GeometryGroup::G6));
    assert!(approx(a.eb.lanes[0].headway_adjustment.unwrap(), 0.534, 0.001));
    assert!(approx(a.eb.lanes[1].headway_adjustment.unwrap(), -0.173, 0.001));
}

// ═══════════════════════════════════════════════════════════════════════════════
// Exhibit 21-15 base saturation headways
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_base_saturation_headway_exhibit_21_15() {
    use GeometryGroup as G;
    // Group 1 column (Example Problem 1 uses cases 1, 2, 3, 4)
    assert_eq!(base_saturation_headway(1, 0, G::G1), 3.9);
    assert_eq!(base_saturation_headway(2, 1, G::G1), 4.7);
    assert_eq!(base_saturation_headway(3, 1, G::G1), 5.8);
    assert_eq!(base_saturation_headway(4, 2, G::G1), 7.0);
    assert_eq!(base_saturation_headway(5, 3, G::G1), 9.6);
    // Group 3b / 4b columns
    assert_eq!(base_saturation_headway(1, 0, G::G3b), 4.3);
    assert_eq!(base_saturation_headway(5, 3, G::G4b), 10.2);
    // Group 5: varies with the number of vehicles
    assert_eq!(base_saturation_headway(2, 1, G::G5), 5.0);
    assert_eq!(base_saturation_headway(2, 2, G::G5), 6.2);
    assert_eq!(base_saturation_headway(4, 4, G::G5), 9.0);
    assert_eq!(base_saturation_headway(5, 6, G::G5), 11.5);
    // Group 6: varies with the number of vehicles
    assert_eq!(base_saturation_headway(2, 3, G::G6), 7.4);
    assert_eq!(base_saturation_headway(3, 2, G::G6), 7.3);
    assert_eq!(base_saturation_headway(4, 5, G::G6), 12.3);
    assert_eq!(base_saturation_headway(5, 4, G::G6), 11.1);
    assert_eq!(base_saturation_headway(5, 6, G::G6), 13.3);
}

// ═══════════════════════════════════════════════════════════════════════════════
// Departure headway iteration (Steps 5–11)
// ═══════════════════════════════════════════════════════════════════════════════

/// Example Problem 1 Steps 5–11: h_d,EB converges to about 4.97 s with
/// x_EB near 0.508 (Exhibit 32-21); SB converges to 5.70 s.
#[test]
fn test_ep1_departure_headway_iteration() {
    let mut a = example_problem_1();
    a.step1_2_flow_rates();
    a.step3_geometry_groups();
    a.step4_headway_adjustments();
    let iters = a.iterate_departure_headways(CONVERGENCE_TOLERANCE_S);
    assert!(iters >= 3 && iters <= 10, "iterations = {iters}");
    assert!(approx(a.eb.lanes[0].departure_headway.unwrap(), 4.97, 0.05));
    assert!(approx(a.wb.lanes[0].departure_headway.unwrap(), 4.74, 0.05));
    assert!(approx(a.sb.lanes[0].departure_headway.unwrap(), 5.70, 0.05));
    assert!(approx(a.eb.lanes[0].degree_of_utilization.unwrap(), 0.508, 0.01));
}

/// Example Problem 1 first iteration from h_d = 3.2 s: x_EB = 0.327,
/// x_WB = 0.374, x_SB = 0.140 and h_d,EB = 4.57 s (published Exhibit
/// 32-21).
#[test]
fn test_ep1_iteration1_values() {
    let mut a = example_problem_1();
    a.step1_2_flow_rates();
    // Initial degrees of utilization (Equation 21-14)
    let v_eb = a.eb.lanes[0].flow_rate.unwrap();
    let v_wb = a.wb.lanes[0].flow_rate.unwrap();
    let v_sb = a.sb.lanes[0].flow_rate.unwrap();
    assert!(approx(v_eb * 3.2 / 3600.0, 0.327, 0.001));
    assert!(approx(v_wb * 3.2 / 3600.0, 0.374, 0.001));
    assert!(approx(v_sb * 3.2 / 3600.0, 0.140, 0.001));
}

/// Example Problem 2 Steps 5–11 (512-state framework): h_d,EB,1 converges
/// near 8.19 s with x_EB,1 near 0.1274.
#[test]
fn test_ep2_departure_headway_iteration() {
    let mut a = example_problem_2();
    a.step1_2_flow_rates();
    a.step3_geometry_groups();
    a.step4_headway_adjustments();
    a.iterate_departure_headways(CONVERGENCE_TOLERANCE_S);
    assert!(approx(a.eb.lanes[0].departure_headway.unwrap(), 8.19, 0.1));
    assert!(approx(a.eb.lanes[0].degree_of_utilization.unwrap(), 0.1274, 0.005));
}

// ═══════════════════════════════════════════════════════════════════════════════
// Steps 13–16
// ═══════════════════════════════════════════════════════════════════════════════

/// Example Problem 1 Steps 13–16: t_s,EB = 2.97 s; d_EB = 13.0 s (LOS B);
/// d_WB = 13.5 s; d_SB = 10.6 s; intersection 12.8 s LOS B;
/// Q95,EB = 2.9 veh.
#[test]
fn test_ep1_delay_los_queue() {
    let mut a = example_problem_1();
    a.analyze();
    let eb = &a.eb.lanes[0];
    assert!(approx(eb.service_time.unwrap(), 2.97, 0.05));
    assert!(approx(eb.control_delay.unwrap(), 13.0, 0.2));
    assert_eq!(eb.los.unwrap(), 'B');
    assert!(approx(eb.queue_95.unwrap(), 2.9, 0.15));
    assert!(approx(a.wb.lanes[0].control_delay.unwrap(), 13.5, 0.2));
    assert!(approx(a.sb.lanes[0].control_delay.unwrap(), 10.6, 0.2));
    assert!(approx(a.intersection_delay.unwrap(), 12.8, 0.2));
    assert_eq!(a.intersection_los.unwrap(), 'B');
}

/// Example Problem 1 Step 12: eastbound lane capacity is "approximately
/// 720 veh/h" per HCM Chapter 32 (and below the naive 368/0.492 =
/// 748 veh/h estimate because of approach interactions). A bisection on
/// the exact (unrounded) flow rates converges to about 704 veh/h; the
/// published value reflects the HCM spreadsheet's coarser search, so a
/// +-20 veh/h tolerance is used here.
#[test]
fn test_ep1_step12_capacity() {
    let mut a = example_problem_1();
    a.step1_2_flow_rates();
    a.step3_geometry_groups();
    a.step4_headway_adjustments();
    let c = a.capacity_of_lane(ApproachDir::EB, 0);
    assert!(approx(c, 720.0, 20.0), "c_EB = {c}");
    // Interaction effect: capacity below the naive v/x estimate
    assert!(c < 748.0);
}

/// Equation 21-33 Q95 spot check with the Example Problem 1 numbers.
#[test]
fn test_queue_95_equation() {
    let q = Awsc::queue_95(0.508, 4.97, 0.25);
    assert!(approx(q, 2.9, 0.05));
    assert_eq!(Awsc::queue_95(0.5, 0.0, 0.25), 0.0);
}

/// Serde round-trip.
#[test]
fn test_serde_roundtrip() {
    let mut a = example_problem_1();
    a.analyze();
    let json = a.to_json().unwrap();
    let back = Awsc::from_json(&json).unwrap();
    assert!(approx(
        back.intersection_delay.unwrap(),
        a.intersection_delay.unwrap(),
        1e-9
    ));
}
