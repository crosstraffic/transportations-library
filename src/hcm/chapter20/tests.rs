//! Per-step unit tests for the HCM Chapter 20 TWSC methodology, using the
//! intermediate values published in HCM Chapter 32, TWSC Example Problems 1
//! and 3.

use super::twsc::*;

/// HCM Chapter 32, TWSC Example Problem 1: three-leg TWSC intersection,
/// one lane per direction on the major street, 10% heavy vehicles, flow
/// rates given directly (peak 15-min volumes x 4).
fn example_problem_1() -> Twsc {
    let demand = TwscDemand {
        v2: 240.0,
        v3: 40.0,
        v4: 160.0,
        v5: 300.0,
        v7: 40.0,
        v9: 120.0,
        ..Default::default()
    };
    let geometry = TwscGeometry {
        is_three_leg: true,
        major_lanes_per_direction: 1,
        ..Default::default()
    };
    let mut twsc = Twsc::new(demand, geometry);
    twsc.heavy_vehicle_pct = 10.0;
    twsc
}

/// HCM Chapter 32, TWSC Example Problem 3: four-leg intersection, two
/// major-street lanes per direction, two-stage gap acceptance (n_m = 2),
/// flared single-lane minor approaches (n_R = 1), 10% heavy vehicles.
/// Demand values are the published flow rates (hourly volumes already
/// divided by PHF = 0.92 in the example).
fn example_problem_3() -> Twsc {
    let demand = TwscDemand {
        v1: 33.0,
        v2: 250.0,
        v3: 50.0,
        v4: 66.0,
        v5: 300.0,
        v6: 100.0,
        v7: 44.0,
        v8: 132.0,
        v9: 55.0,
        v10: 11.0,
        v11: 110.0,
        v12: 28.0,
        ..Default::default()
    };
    let geometry = TwscGeometry {
        is_three_leg: false,
        major_lanes_per_direction: 2,
        median_storage_nb: Some(2),
        median_storage_sb: Some(2),
        flare_storage_nb: Some(1),
        flare_storage_sb: Some(1),
        ..Default::default()
    };
    let mut twsc = Twsc::new(demand, geometry);
    twsc.heavy_vehicle_pct = 10.0;
    // The published example's conflicting flows for the Stage II crossing
    // and left-turn movements follow the HCM 6th Edition equation forms
    // (see the VERIFY-HCM note in twsc.rs); override to match Chapter 32.
    twsc.conflicting_flow_overrides = vec![
        ConflictingFlowOverride {
            movement: "8".into(),
            stage: "stage2".into(),
            value: 532.0,
        },
        ConflictingFlowOverride {
            movement: "11".into(),
            stage: "stage1".into(),
            value: 482.0,
        },
        ConflictingFlowOverride {
            movement: "7".into(),
            stage: "stage2".into(),
            value: 337.0,
        },
        ConflictingFlowOverride {
            movement: "10".into(),
            stage: "stage2".into(),
            value: 257.0,
        },
    ];
    twsc
}

fn approx(a: f64, b: f64, tol: f64) -> bool {
    (a - b).abs() <= tol
}

// ═══════════════════════════════════════════════════════════════════════════════
// Example Problem 1 step tests
// ═══════════════════════════════════════════════════════════════════════════════

/// Step 3 (Equations 20-3, 20-4, 20-12/20-14): conflicting flows of
/// Example Problem 1: v_c,4 = 280, v_c,9 = 260, v_c,7 = 880 veh/h.
#[test]
fn test_ep1_step3_conflicting_flows() {
    let mut t = example_problem_1();
    t.step1_2_demand_flow_rates();
    t.step3_conflicting_flows();
    assert!(approx(t.movements[Mv::M4.idx()].conflicting_flow.unwrap(), 280.0, 0.01));
    assert!(approx(t.movements[Mv::M9.idx()].conflicting_flow.unwrap(), 260.0, 0.01));
    assert!(approx(t.movements[Mv::M7.idx()].conflicting_flow.unwrap(), 880.0, 0.01));
}

/// Step 4 (Equations 20-16 and 20-17): t_c,4 = 4.2, t_c,9 = 6.3,
/// t_c,7 = 6.5 s; t_f,4 = 2.29, t_f,9 = 3.39, t_f,7 = 3.59 s.
#[test]
fn test_ep1_step4_headways() {
    let mut t = example_problem_1();
    t.step1_2_demand_flow_rates();
    t.step3_conflicting_flows();
    t.step4_headways();
    assert!(approx(t.movements[Mv::M4.idx()].critical_headway.unwrap(), 4.2, 1e-9));
    assert!(approx(t.movements[Mv::M9.idx()].critical_headway.unwrap(), 6.3, 1e-9));
    // t_c,7 = 7.1 + 1.0(0.1) + 0.2(0) - 0.7 = 6.5 (t_3,LT applies at a T)
    assert!(approx(t.movements[Mv::M7.idx()].critical_headway.unwrap(), 6.5, 1e-9));
    assert!(approx(t.movements[Mv::M4.idx()].followup_headway.unwrap(), 2.29, 1e-9));
    assert!(approx(t.movements[Mv::M9.idx()].followup_headway.unwrap(), 3.39, 1e-9));
    assert!(approx(t.movements[Mv::M7.idx()].followup_headway.unwrap(), 3.59, 1e-9));
}

/// Step 5 (Equation 20-18): c_p,4 = 1,238; c_p,9 = 760; c_p,7 = 308 veh/h.
#[test]
fn test_ep1_step5_potential_capacity() {
    let mut t = example_problem_1();
    t.step1_2_demand_flow_rates();
    t.step3_conflicting_flows();
    t.step4_headways();
    t.step5_potential_capacities();
    assert!(approx(t.movements[Mv::M4.idx()].potential_capacity.unwrap(), 1238.0, 1.0));
    assert!(approx(t.movements[Mv::M9.idx()].potential_capacity.unwrap(), 760.0, 1.0));
    assert!(approx(t.movements[Mv::M7.idx()].potential_capacity.unwrap(), 308.0, 1.0));
}

/// Steps 7–8 (Equations 20-22, 20-23, 20-28, 20-35, 20-36):
/// c_m,4 = 1,238; c_m,9 = 760; p_0,4 = 0.871; c_m,7 = 268 veh/h.
#[test]
fn test_ep1_step7_8_movement_capacity() {
    let mut t = example_problem_1();
    t.step1_2_demand_flow_rates();
    t.step3_conflicting_flows();
    t.step4_headways();
    t.step5_potential_capacities();
    t.step6_9_movement_capacities();
    assert!(approx(t.movements[Mv::M4.idx()].movement_capacity.unwrap(), 1238.0, 1.0));
    assert!(approx(t.movements[Mv::M9.idx()].movement_capacity.unwrap(), 760.0, 1.0));
    // f_7 = p_0,4 = 1 - 160/1,238 = 0.871; c_m,7 = 308 x 0.871 = 268
    assert!(approx(t.movements[Mv::M7.idx()].movement_capacity.unwrap(), 268.0, 1.5));
}

/// Step 10 (Equation 20-49): shared-lane capacity of the northbound
/// approach c_SH,NB = 521 veh/h.
#[test]
fn test_ep1_step10_shared_lane() {
    let mut t = example_problem_1();
    t.step1_2_demand_flow_rates();
    t.step3_conflicting_flows();
    t.step4_headways();
    t.step5_potential_capacities();
    t.step6_9_movement_capacities();
    t.step10_lane_capacities();
    assert_eq!(t.lanes_nb.len(), 1);
    assert!(approx(t.lanes_nb[0].capacity, 521.0, 1.5));
    assert!(approx(t.lanes_nb[0].flow_rate, 160.0, 1e-9));
    assert!(t.lanes_sb.is_empty());
}

/// Steps 11–13 (Equations 20-61, 20-64 through 20-66, Exhibit 20-2):
/// d_4 = 8.3 s (LOS A); d_SH,NB = 14.9 s (LOS B); d_A,WB = 2.9 s;
/// d_I = 4.1 s; Q_95,4 = 0.4 veh; Q_95,NB = 1.3 veh.
#[test]
fn test_ep1_steps11_13_delay_los_queue() {
    let mut t = example_problem_1();
    t.analyze();
    let m4 = &t.movements[Mv::M4.idx()];
    assert!(approx(m4.control_delay.unwrap(), 8.3, 0.1));
    assert_eq!(m4.los.unwrap(), 'A');
    assert!(approx(m4.queue_95.unwrap(), 0.4, 0.1));

    let nb = &t.lanes_nb[0];
    assert!(approx(nb.control_delay, 14.9, 0.2));
    assert_eq!(nb.los, 'B');
    assert!(approx(nb.queue_95, 1.3, 0.1));

    let [d_eb, d_wb, d_nb, _d_sb] = t.approach_delays.unwrap();
    assert!(approx(d_eb, 0.0, 1e-9));
    assert!(approx(d_wb, 2.9, 0.1));
    assert!(approx(d_nb, 14.9, 0.2));
    assert!(approx(t.intersection_delay.unwrap(), 4.1, 0.1));
}

// ═══════════════════════════════════════════════════════════════════════════════
// Example Problem 3 step tests (two-stage gap acceptance + flared lanes)
// ═══════════════════════════════════════════════════════════════════════════════

/// Step 3 for Example Problem 3 with the published conflicting-flow values
/// (Stage II values overridden per the Chapter 32 worked example).
#[test]
fn test_ep3_step3_conflicting_flows() {
    let mut t = example_problem_3();
    t.step1_2_demand_flow_rates();
    t.step3_conflicting_flows();
    let m = |mv: Mv| &t.movements[mv.idx()];
    assert!(approx(m(Mv::M1).conflicting_flow.unwrap(), 400.0, 0.01));
    assert!(approx(m(Mv::M4).conflicting_flow.unwrap(), 300.0, 0.01));
    assert!(approx(m(Mv::M9).conflicting_flow.unwrap(), 150.0, 0.01));
    assert!(approx(m(Mv::M12).conflicting_flow.unwrap(), 200.0, 0.01));
    // Stage values (with overrides matching the published example)
    assert!(approx(m(Mv::M8).conflicting_flow_stage1.unwrap(), 341.0, 0.01));
    assert!(approx(m(Mv::M8).conflicting_flow_stage2.unwrap(), 532.0, 0.01));
    assert!(approx(m(Mv::M8).conflicting_flow.unwrap(), 873.0, 0.01));
    assert!(approx(m(Mv::M11).conflicting_flow.unwrap(), 848.0, 0.01));
    assert!(approx(m(Mv::M7).conflicting_flow_stage1.unwrap(), 341.0, 0.01));
    assert!(approx(m(Mv::M7).conflicting_flow.unwrap(), 678.0, 0.01));
    assert!(approx(m(Mv::M10).conflicting_flow.unwrap(), 739.0, 0.01));
}

/// Default (7th Edition Exhibit 20-14/20-16) conflicting-flow factors
/// without overrides: v_c,II,8 = 482 (0.5 x v_6), v_c,I,11 = 532
/// (1.0 x v_6), v_c,II,7 = 332, v_c,II,10 = 216 veh/h.
#[test]
fn test_ep3_step3_default_exhibit_factors() {
    let mut t = example_problem_3();
    t.conflicting_flow_overrides.clear();
    t.step1_2_demand_flow_rates();
    t.step3_conflicting_flows();
    let m = |mv: Mv| &t.movements[mv.idx()];
    // Exhibit 20-14: movement 8 Stage II factor on v6 is 0.5 (shared lane)
    assert!(approx(m(Mv::M8).conflicting_flow_stage2.unwrap(), 482.0, 0.01));
    // Exhibit 20-14: movement 11 Stage I factor on v6 is 1 (not channelized)
    assert!(approx(m(Mv::M11).conflicting_flow_stage1.unwrap(), 532.0, 0.01));
    // Exhibit 20-16: movement 7 Stage II = 2(66) + 0.5(300) + 0.5(100) = 332
    assert!(approx(m(Mv::M7).conflicting_flow_stage2.unwrap(), 332.0, 0.01));
    // Exhibit 20-16: movement 10 Stage II = 2(33) + 0.5(250) + 0.5(50) = 216
    assert!(approx(m(Mv::M10).conflicting_flow_stage2.unwrap(), 216.0, 0.01));
}

/// Step 4 for Example Problem 3: two-lane major street adjustments
/// (t_c,HV = 2.0, t_f,HV = 1.0).
#[test]
fn test_ep3_step4_headways() {
    let mut t = example_problem_3();
    t.step1_2_demand_flow_rates();
    t.step3_conflicting_flows();
    t.step4_headways();
    let m = |mv: Mv| &t.movements[mv.idx()];
    assert!(approx(m(Mv::M1).critical_headway.unwrap(), 4.3, 1e-9));
    assert!(approx(m(Mv::M9).critical_headway.unwrap(), 7.1, 1e-9));
    assert!(approx(m(Mv::M8).critical_headway.unwrap(), 6.7, 1e-9));
    assert!(approx(m(Mv::M8).critical_headway_stage1.unwrap(), 5.7, 1e-9));
    assert!(approx(m(Mv::M7).critical_headway.unwrap(), 7.7, 1e-9));
    assert!(approx(m(Mv::M7).critical_headway_stage1.unwrap(), 6.7, 1e-9));
    assert!(approx(m(Mv::M1).followup_headway.unwrap(), 2.3, 1e-9));
    assert!(approx(m(Mv::M9).followup_headway.unwrap(), 3.4, 1e-9));
    assert!(approx(m(Mv::M8).followup_headway.unwrap(), 4.1, 1e-9));
    assert!(approx(m(Mv::M7).followup_headway.unwrap(), 3.6, 1e-9));
}

/// Step 5 for Example Problem 3: published potential capacities.
#[test]
fn test_ep3_step5_potential_capacities() {
    let mut t = example_problem_3();
    t.step1_2_demand_flow_rates();
    t.step3_conflicting_flows();
    t.step4_headways();
    t.step5_potential_capacities();
    let m = |mv: Mv| &t.movements[mv.idx()];
    assert!(approx(m(Mv::M1).potential_capacity.unwrap(), 1100.0, 1.0));
    assert!(approx(m(Mv::M4).potential_capacity.unwrap(), 1202.0, 1.0));
    assert!(approx(m(Mv::M9).potential_capacity.unwrap(), 845.0, 1.0));
    assert!(approx(m(Mv::M12).potential_capacity.unwrap(), 783.0, 1.0));
    assert!(approx(m(Mv::M8).potential_capacity_stage1.unwrap(), 618.0, 1.0));
    assert!(approx(m(Mv::M8).potential_capacity_stage2.unwrap(), 504.0, 1.0));
    assert!(approx(m(Mv::M8).potential_capacity.unwrap(), 273.0, 1.0));
    assert!(approx(m(Mv::M11).potential_capacity_stage1.unwrap(), 532.0, 1.0));
    assert!(approx(m(Mv::M11).potential_capacity_stage2.unwrap(), 601.0, 1.0));
    assert!(approx(m(Mv::M11).potential_capacity.unwrap(), 283.0, 1.0));
    assert!(approx(m(Mv::M7).potential_capacity_stage1.unwrap(), 626.0, 1.0));
    assert!(approx(m(Mv::M7).potential_capacity_stage2.unwrap(), 629.0, 1.0));
    assert!(approx(m(Mv::M7).potential_capacity.unwrap(), 323.0, 1.0));
    assert!(approx(m(Mv::M10).potential_capacity_stage1.unwrap(), 514.0, 1.0));
    assert!(approx(m(Mv::M10).potential_capacity_stage2.unwrap(), 703.0, 1.0));
    assert!(approx(m(Mv::M10).potential_capacity.unwrap(), 291.0, 1.0));
}

/// Steps 8–9 for Example Problem 3: two-stage total capacities
/// c_T,8 = 390, c_T,11 = 405, c_T,7 = 365, c_T,10 = 342 veh/h.
#[test]
fn test_ep3_two_stage_capacities() {
    let mut t = example_problem_3();
    t.step1_2_demand_flow_rates();
    t.step3_conflicting_flows();
    t.step4_headways();
    t.step5_potential_capacities();
    t.step6_9_movement_capacities();
    let m = |mv: Mv| &t.movements[mv.idx()];
    // Stage capacities from the worked example
    assert!(approx(m(Mv::M8).movement_capacity_stage1.unwrap(), 599.0, 1.5));
    assert!(approx(m(Mv::M8).movement_capacity_stage2.unwrap(), 476.0, 1.5));
    assert!(approx(m(Mv::M8).movement_capacity.unwrap(), 390.0, 2.0));
    assert!(approx(m(Mv::M11).movement_capacity.unwrap(), 405.0, 2.0));
    assert!(approx(m(Mv::M7).movement_capacity.unwrap(), 365.0, 2.0));
    assert!(approx(m(Mv::M10).movement_capacity.unwrap(), 342.0, 2.0));
}

/// Equations 20-37/20-38/20-39 in isolation with the Example Problem 3
/// movement 8 values: a = 0.949, y = 1.808, c_T = 390 veh/h.
#[test]
fn test_two_stage_total_capacity_equation() {
    assert!(approx(Twsc::two_stage_adjustment_a(2), 0.949, 0.001));
    let c_t = Twsc::two_stage_total_capacity(599.0, 476.0, 250.0, 33.0, 2);
    assert!(approx(c_t, 390.0, 1.0));
    // y = 1 branch (Equation 20-40): requires c_I = c_II - v_L, e.g.,
    // c_I = 400, c_II = 450, v_L = 50 gives y = 1 exactly.
    let c_t1 = Twsc::two_stage_total_capacity(400.0, 450.0, 200.0, 50.0, 2);
    let a = Twsc::two_stage_adjustment_a(2);
    let expected = a / 3.0 * (2.0 * (450.0 - 50.0) + 200.0);
    assert!(approx(c_t1, expected, 1e-9));
}

/// Step 10 for Example Problem 3 (Equations 20-49 and 20-50):
/// c_m,7+8 = 383, c_F,NB = 498; c_m,10+11 = 398, c_F,SB = 487 veh/h.
#[test]
fn test_ep3_step10_flared_lanes() {
    let mut t = example_problem_3();
    t.step1_2_demand_flow_rates();
    t.step3_conflicting_flows();
    t.step4_headways();
    t.step5_potential_capacities();
    t.step6_9_movement_capacities();
    t.step10_lane_capacities();
    assert_eq!(t.lanes_nb.len(), 1);
    assert_eq!(t.lanes_sb.len(), 1);
    assert!(approx(t.lanes_nb[0].capacity, 498.0, 3.0));
    assert!(approx(t.lanes_sb[0].capacity, 487.0, 3.0));
}

/// Equation 20-50 in isolation with the Example Problem 3 numbers.
#[test]
fn test_flared_lane_capacity_equation() {
    let c_f = Twsc::flared_lane_capacity(55.0, 845.0, 176.0, 383.0, 1);
    assert!(approx(c_f, 498.0, 1.0));
    // n_R = 0 reduces to the shared-lane form (Equation 20-49)
    let c0 = Twsc::flared_lane_capacity(55.0, 845.0, 176.0, 383.0, 0);
    let c_sh = Twsc::shared_lane_capacity(&[(55.0, 845.0), (176.0, 383.0)]);
    assert!(approx(c0, c_sh, 1e-9));
}

/// Steps 11–13 for Example Problem 3: d_1 = 8.4, d_4 = 8.2, d_NB = 18.3,
/// d_SB = 15.6 s/veh; LOS A/A/C/C; d_I = 6.3 s; queues 0.1/0.2/2.4/1.3 veh.
#[test]
fn test_ep3_steps11_13() {
    let mut t = example_problem_3();
    t.analyze();
    let m = |mv: Mv| &t.movements[mv.idx()];
    assert!(approx(m(Mv::M1).control_delay.unwrap(), 8.4, 0.1));
    assert!(approx(m(Mv::M4).control_delay.unwrap(), 8.2, 0.1));
    assert_eq!(m(Mv::M1).los.unwrap(), 'A');
    assert_eq!(m(Mv::M4).los.unwrap(), 'A');
    assert!(approx(t.lanes_nb[0].control_delay, 18.3, 0.3));
    assert!(approx(t.lanes_sb[0].control_delay, 15.6, 0.3));
    assert_eq!(t.lanes_nb[0].los, 'C');
    assert_eq!(t.lanes_sb[0].los, 'C');
    let [d_eb, d_wb, _, _] = t.approach_delays.unwrap();
    assert!(approx(d_eb, 0.8, 0.1));
    assert!(approx(d_wb, 1.2, 0.1));
    assert!(approx(t.intersection_delay.unwrap(), 6.3, 0.15));
    assert!(approx(m(Mv::M1).queue_95.unwrap(), 0.1, 0.1));
    assert!(approx(m(Mv::M4).queue_95.unwrap(), 0.2, 0.1));
    assert!(approx(t.lanes_nb[0].queue_95, 2.4, 0.2));
    assert!(approx(t.lanes_sb[0].queue_95, 1.3, 0.2));
}

// ═══════════════════════════════════════════════════════════════════════════════
// Auxiliary procedures
// ═══════════════════════════════════════════════════════════════════════════════

/// Equations 20-29/20-33: shared-major-lane queue-free probability reduces
/// to the closed form 1 - (1 - p0)/(1 - x) for n_L = 0.
#[test]
fn test_prob_queue_free_shared_major() {
    let p0 = 0.9;
    let x = 0.4;
    let p_star = Twsc::prob_queue_free_shared_major(p0, x, 0);
    assert!(approx(p_star, 1.0 - (1.0 - p0) / (1.0 - x), 1e-12));
    // Larger pockets approach the exclusive-lane probability
    let p_big = Twsc::prob_queue_free_shared_major(p0, x, 20);
    assert!(p_big > p_star);
    assert!(p_big <= p0 + 1e-9);
}

/// Equation 20-19 through 20-21: upstream-signal potential capacity limits.
#[test]
fn test_potential_capacity_upstream_signal() {
    // p_b = 0 reduces to Equation 20-18
    let base = Twsc::potential_capacity_upstream_signal(500.0, 6.5, 3.5, 0.0, 1);
    let plain = crate::hcm::common::gap_acceptance::potential_capacity(500.0, 6.5, 3.5);
    assert!(approx(base, plain, 1e-9));
    // Fully blocked period contributes zero capacity
    let blocked = Twsc::potential_capacity_upstream_signal(500.0, 6.5, 3.5, 1.0, 1);
    assert!(approx(blocked, 0.0, 1e-9));
    // Low conflicting flow during the unblocked period: v_c,u = 0 branch
    let low = Twsc::potential_capacity_upstream_signal(100.0, 6.5, 3.5, 0.5, 1);
    assert!(approx(low, 0.5 * 3600.0 / 3.5, 1e-9));
}

/// Equations 20-51 through 20-60: shared major-street lane capacity is
/// bounded by the through saturation flow rate and decreases with left-turn
/// demand.
#[test]
fn test_shared_major_lane_capacity() {
    // No left turns: capacity equals s_2+3 (Equation 20-55 bound)
    let c = Twsc::shared_major_lane_capacity(0.0, 1000.0, 800.0, 100.0, 1800.0, 1500.0, 1.0, 1, 0);
    let s23 = (800.0 + 100.0) / (800.0 / 1800.0 + 100.0 / 1500.0);
    assert!(approx(c, s23, 1e-9));
    // Heavy left-turn demand reduces the shared-lane capacity below the
    // saturation bound
    let c_lt =
        Twsc::shared_major_lane_capacity(300.0, 400.0, 800.0, 100.0, 1800.0, 1500.0, 1.0, 1, 0);
    assert!(c_lt < c, "c_lt = {c_lt}, c = {c}");
}

/// Equations 20-62/20-63: Rank 1 delay is zero when no vehicles are blocked
/// and scales with the blocked proportion.
#[test]
fn test_rank1_delay() {
    assert!(approx(Twsc::rank1_delay(1.0, 100.0, 500.0, 0.0, 9.0, 1.0, 1), 0.0, 1e-12));
    // Single-lane major: d = (1 - p*) d_left
    let d = Twsc::rank1_delay(0.8, 100.0, 500.0, 0.0, 9.0, 1.0, 1);
    assert!(approx(d, 0.2 * 9.0, 1e-12));
    // Multilane major weights by the left-lane through share
    let d2 = Twsc::rank1_delay(0.8, 100.0, 500.0, 0.0, 9.0, 0.5, 2);
    assert!(approx(d2, 0.2 * 250.0 / 350.0 * 9.0, 1e-12));
}

/// Movement labels round-trip.
#[test]
fn test_movement_labels() {
    for (label, mv) in [
        ("1", Mv::M1),
        ("1U", Mv::M1U),
        ("4U", Mv::M4U),
        ("12", Mv::M12),
    ] {
        assert_eq!(Mv::from_label(label), Some(mv));
    }
    assert_eq!(Mv::from_label("13"), None);
}

/// Serde round-trip of the full analysis type.
#[test]
fn test_serde_roundtrip() {
    let mut t = example_problem_1();
    t.analyze();
    let json = t.to_json().unwrap();
    let back = Twsc::from_json(&json).unwrap();
    assert!(approx(
        back.movements[Mv::M4.idx()].movement_capacity.unwrap(),
        t.movements[Mv::M4.idx()].movement_capacity.unwrap(),
        1e-9
    ));
}
