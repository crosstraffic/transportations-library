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
    // No conflicting-flow overrides. The published Example Problem 3 values
    // reproduce natively under the December 2022 corrections (Equations
    // 20-14/20-15 with Exhibit 20-16, and the Exhibit 20-14 movement 8
    // Stage II / movement 11 Stage I swap); see the comment in twsc.rs.
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

/// Conflicting-flow factors with no overrides applied.
///
/// All four two-stage movements now match the published Example Problem 3 values without help.
/// The December 2022 corrections to Exhibit 20-16 replaced the major-street right-turn term in
/// the left turns' Stage II with the opposing minor-street through movement (before that, this
/// test pinned v_c,II,7 = 332 and v_c,II,10 = 216, neither of which the book ever printed), and
/// the corrections' Exhibit 20-14 swap of the movement 8 Stage II / movement 11 Stage I entries
/// for conflicting movement 6 resolved the crossing movements the same way (before it, the
/// defaults gave v_c,II,8 = 482 and v_c,I,11 = 532, the published values transposed).
#[test]
fn test_ep3_step3_default_exhibit_factors() {
    let mut t = example_problem_3();
    t.conflicting_flow_overrides.clear();
    t.step1_2_demand_flow_rates();
    t.step3_conflicting_flows();
    let m = |mv: Mv| &t.movements[mv.idx()];
    // Corrected Exhibit 20-14: movement 8 Stage II factor on v6 is 1 (not channelized),
    // reproducing the published 532 veh/h.
    assert!(approx(m(Mv::M8).conflicting_flow_stage2.unwrap(), 532.0, 0.01));
    // Corrected Exhibit 20-14: movement 11 Stage I factor on v6 is 0.5 (shared lane),
    // reproducing the published 482 veh/h.
    assert!(approx(m(Mv::M11).conflicting_flow_stage1.unwrap(), 482.0, 0.01));
    // Corrected Exhibit 20-16: v_c,II,7 uses 0.5 v_11, reproducing the published 337 veh/h.
    assert!(approx(m(Mv::M7).conflicting_flow_stage2.unwrap(), 337.0, 0.01));
    // Corrected Exhibit 20-16: v_c,II,10 uses 0.5 v_8, reproducing the published 257 veh/h.
    assert!(approx(m(Mv::M10).conflicting_flow_stage2.unwrap(), 257.0, 0.01));
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

/// Step 7d, Equations 20-33/20-34: the shared-lane (n_L = 0) queue-free
/// probability reproduces the published HCM Chapter 32 Example Problem 4
/// value p*_0 = 0.856 from its inputs (p_0 = 0.900, x_2+3 = 0.304).
#[test]
fn test_p0_star_shared_major_ep4_value() {
    // x_2+3 = f_LL (v2/s2 + v3/s3) = 0.5 (982/1800 + 94/1500) (Equation 20-30)
    let x_23 = 0.5 * (982.0 / 1800.0 + 94.0 / 1500.0);
    assert!(approx(x_23, 0.304, 5e-4), "x_2+3 = {x_23}");
    let p_star = Twsc::prob_queue_free_shared_major(0.900, x_23, 0);
    assert!(approx(p_star, 0.856, 1e-3), "p*_0 = {p_star} (published 0.856)");
}

/// Step 7d, Equations 20-29/20-31: the short-pocket (n_L > 0) form is the
/// (n_L + 1)-root of Equation 20-29, lies between the shared-lane value and
/// the exclusive-lane p_0, and increases monotonically toward p_0 as the
/// pocket lengthens.
#[test]
fn test_p0_star_short_pocket_variant() {
    let (p0, x): (f64, f64) = (0.90, 0.304);
    let n_l = 2u32;
    // Closed-form Equation 20-29 for n_L = 2
    let n = (n_l + 1) as f64;
    let bracket = (1.0 + x.powf(n) / (1.0 - x)).powf(1.0 / n);
    let expected = 1.0 - (1.0 - p0) * bracket;
    let p_pocket = Twsc::prob_queue_free_shared_major(p0, x, n_l);
    assert!(approx(p_pocket, expected, 1e-12), "p*(n_L=2) = {p_pocket}");
    // Ordering: shared (n_L = 0) < short pocket < exclusive p_0
    let p_shared = Twsc::prob_queue_free_shared_major(p0, x, 0);
    assert!(p_shared < p_pocket, "{p_shared} !< {p_pocket}");
    assert!(p_pocket < p0, "{p_pocket} !< {p0}");
    // Monotone increasing toward p_0 as the pocket grows
    let p_long = Twsc::prob_queue_free_shared_major(p0, x, 10);
    assert!(p_long > p_pocket && p_long <= p0 + 1e-9);
}

/// Equations 20-62/20-63 with the Example Problem 4 inputs reproduce the
/// published Rank 1 shared-lane delay d_2+3 = 1.3 s.
#[test]
fn test_rank1_delay_ep4_value() {
    // (1 - 0.856) f_LL (v2+v3) / (v_1+1U + f_LL (v2+v3)) d_1+1U, N = 2
    let d = Twsc::rank1_delay(0.856, 75.0, 982.0, 94.0, 10.3, 0.5, 2);
    assert!(approx(d, 1.3, 0.05), "d_2+3 = {d} (published 1.3)");
}

/// End-to-end Step 7d wiring: declaring a shared major-street left lane
/// substitutes p*_0 for p_0, lowering the Rank 4 minor-left capacity, and
/// exposes the Step 11b Rank 1 delay; the default `Exclusive` config is
/// bit-identical to omitting the field (backward compatibility).
#[test]
fn test_major_left_shared_wiring() {
    let demand = TwscDemand {
        v1: 75.0,
        v2: 982.0,
        v3: 94.0,
        v4: 76.0,
        v5: 992.0,
        v6: 94.0,
        v7: 80.0,
        v9: 100.0,
        v10: 80.0,
        v12: 100.0,
        ..Default::default()
    };
    let base_geom = TwscGeometry {
        is_three_leg: false,
        major_lanes_per_direction: 2,
        minor_lanes_nb: MinorLaneConfig::Separate,
        minor_lanes_sb: MinorLaneConfig::Separate,
        ..Default::default()
    };

    // Exclusive (default): no p* substitution, no Rank 1 delay.
    let mut excl = Twsc::new(demand.clone(), base_geom.clone());
    excl.heavy_vehicle_pct = 1.0;
    excl.analyze();
    let cm7_excl = excl.get_movement_capacity(Mv::M7).unwrap();
    assert!(excl.rank1_major_delay.is_none(), "exclusive => no Rank 1 delay");

    // Shared major left on both approaches: p*_0 < p_0 lowers c_m,7 and the
    // Rank 1 delay becomes nonzero.
    let mut shared = Twsc::new(
        demand,
        TwscGeometry {
            major_left_eb: MajorLeftLaneConfig::Shared,
            major_left_wb: MajorLeftLaneConfig::Shared,
            ..base_geom
        },
    );
    shared.heavy_vehicle_pct = 1.0;
    shared.analyze();
    let cm7_shared = shared.get_movement_capacity(Mv::M7).unwrap();
    assert!(
        cm7_shared < cm7_excl,
        "shared c_m,7 = {cm7_shared} !< exclusive {cm7_excl}"
    );
    let [d23, d56] = shared.rank1_major_delay.expect("shared => Rank 1 delay");
    assert!(d23 > 0.0 && d56 > 0.0, "Rank 1 delay = [{d23}, {d56}]");
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

/// Exhibit 20-19: proportion-blocked mapping per movement and stage.
#[test]
fn test_platoon_blockage_exhibit_20_19_mapping() {
    let pb = PlatoonBlockage {
        pb1: 0.11,
        pb4: 0.44,
        pb7: 0.77,
        pb8: 0.88,
        pb9: 0.99,
        pb10: 0.10,
        pb11: 0.11,
        pb12: 0.12,
    };
    // One-stage totals; U-turns mirror their companion left turn.
    assert_eq!(pb.total(Mv::M1), Some(0.11));
    assert_eq!(pb.total(Mv::M1U), Some(0.11));
    assert_eq!(pb.total(Mv::M4), Some(0.44));
    assert_eq!(pb.total(Mv::M4U), Some(0.44));
    assert_eq!(pb.total(Mv::M7), Some(0.77));
    assert_eq!(pb.total(Mv::M8), Some(0.88));
    assert_eq!(pb.total(Mv::M9), Some(0.99));
    assert_eq!(pb.total(Mv::M10), Some(0.10));
    assert_eq!(pb.total(Mv::M11), Some(0.11));
    assert_eq!(pb.total(Mv::M12), Some(0.12));
    // Rank 1 movements have no proportion blocked.
    for mv in [Mv::M2, Mv::M3, Mv::M5, Mv::M6] {
        assert_eq!(pb.total(mv), None);
    }
    // Two-stage per-stage mapping: movements 7/8 -> (p_b,4, p_b,1);
    // movements 10/11 -> (p_b,1, p_b,4).
    assert_eq!(pb.stages(Mv::M7), Some((0.44, 0.11)));
    assert_eq!(pb.stages(Mv::M8), Some((0.44, 0.11)));
    assert_eq!(pb.stages(Mv::M10), Some((0.11, 0.44)));
    assert_eq!(pb.stages(Mv::M11), Some((0.11, 0.44)));
    for mv in [Mv::M1, Mv::M4, Mv::M9, Mv::M12] {
        assert_eq!(pb.stages(mv), None);
    }
}

/// Equation 20-19: the unblocked-period conflicting flow selects the subtract
/// branch when v_c > 1.5 v_c,min p_b and the zero branch otherwise. Verified
/// through the published Example Problem 4 case v_c,1 = 1,086, p_b = 0.170,
/// N = 2 -> v_c,u = 694, and a low-flow case that triggers the zero branch.
#[test]
fn test_eq_20_19_unblocked_flow_branches() {
    // Subtract branch: 1.5 * 2000 * 0.170 = 510 < 1086, and the resulting
    // platooned c_p,1 matches the published 750 veh/h.
    let cp = Twsc::potential_capacity_upstream_signal(1086.0, 4.12, 2.21, 0.170, 2);
    assert!(approx(cp, 750.0, 2.0), "c_p,1 = {cp}");
    // Zero branch: v_c = 300 < 1.5 * 2000 * 0.5 = 1500, so v_c,u = 0 and
    // Equation 20-21 falls to 3600/t_f, giving c_p = (1 - p_b) * 3600/t_f.
    let cp0 = Twsc::potential_capacity_upstream_signal(300.0, 6.5, 3.5, 0.5, 2);
    assert!(approx(cp0, 0.5 * 3600.0 / 3.5, 1e-9), "zero-branch c_p = {cp0}");
}

/// Equation 20-21: when v_c,u,x = 0 the random-flow capacity is 3600/t_f and
/// the potential capacity is (1 - p_b) * 3600/t_f (the fully-random ceiling).
#[test]
fn test_eq_20_21_zero_flow_branch() {
    // v_c = 0 always yields v_c,u = 0 for any p_b in [0, 1).
    let tf = 3.3;
    for pb in [0.1, 0.3, 0.6] {
        let cp = Twsc::potential_capacity_upstream_signal(0.0, 6.9, tf, pb, 2);
        assert!(approx(cp, (1.0 - pb) * 3600.0 / tf, 1e-9), "p_b = {pb}: {cp}");
    }
}

/// Platoon-off equivalence: with no p_b inputs (or an all-zero
/// [`PlatoonBlockage`]) Step 5 reduces to Equation 20-18 exactly, and the
/// full pipeline is bit-identical to a run without platooning.
#[test]
fn test_platoon_off_equivalence() {
    // Standalone Step 5b with p_b = 0 equals Equation 20-18.
    let base = Twsc::potential_capacity_upstream_signal(500.0, 6.5, 3.5, 0.0, 2);
    let plain = crate::hcm::common::gap_acceptance::potential_capacity(500.0, 6.5, 3.5);
    assert!(approx(base, plain, 1e-12));

    // Full pipeline: None vs Some(all-zero) must produce identical results.
    let mut a = example_problem_3();
    let mut b = example_problem_3();
    b.platoon_blockage = Some(PlatoonBlockage::default());
    a.analyze();
    b.analyze();
    for mv in ALL_MOVEMENTS {
        assert_eq!(
            a.movements[mv.idx()].potential_capacity,
            b.movements[mv.idx()].potential_capacity,
            "potential capacity differs for {mv:?}"
        );
        assert_eq!(
            a.movements[mv.idx()].movement_capacity,
            b.movements[mv.idx()].movement_capacity,
            "movement capacity differs for {mv:?}"
        );
    }
    assert_eq!(a.intersection_delay, b.intersection_delay);
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

// ═══════════════════════════════════════════════════════════════════════════════
// Computed proportion of time blocked (HCM Chapter 30, Section 3)
// ═══════════════════════════════════════════════════════════════════════════════

use crate::hcm::urban_segments::platoon_dispersion::MovementDischarge;
use super::computed_pb::{
    blocked_period_steps, proportion_time_blocked_from_profile, UpstreamSignal, UpstreamSignals,
};

/// A single upstream signal feeding one major-street through-lane group with a
/// compact platoon: a `green_duration_s`-long green whose queue clears over
/// the whole green at the saturation rate, so the discharge profile is a
/// rectangular platoon of `green_duration_s` steps.
fn platoon_signal(distance_ft: f64, green_start_s: f64, green_duration_s: f64) -> UpstreamSignal {
    let sat = 1_800.0; // veh/h -> 0.5 veh/step at d_t = 1 s
    // The queue service time spans the whole green, so every green step
    // discharges at the saturation rate (a flat rectangular platoon) and the
    // discharge volume only fixes the — here empty — post-queue tail.
    UpstreamSignal {
        distance_ft,
        progression_speed_mph: 30.0,
        uniform_volume_veh_h: 0.0,
        discharges: vec![MovementDischarge {
            discharge_volume_veh_h: 400.0,
            saturation_flow_veh_h: sat,
            green_start_s,
            green_duration_s,
            queue_service_time_s: green_duration_s,
        }],
    }
}

/// A two-lane major TWSC intersection used only for its critical-headway
/// values (Equation 20-16); demand is irrelevant to p_b.
fn two_lane_twsc() -> Twsc {
    let geometry = TwscGeometry {
        major_lanes_per_direction: 2,
        ..Default::default()
    };
    Twsc::new(TwscDemand::default(), geometry)
}

/// HCM Equation 30-13 on a hand-constructed square-wave arrival profile: a
/// 20-step platoon at 0.5 veh/step inside a 100-step cycle, with a threshold
/// (q_c = 1,080 veh/h -> 0.30 veh/step at d_t = 1 s) that exactly 20 steps
/// exceed, so p_b = 20 (1) / 100 = 0.20.
#[test]
fn test_proportion_time_blocked_square_wave() {
    let mut profile = vec![0.10; 100];
    for step in profile.iter_mut().take(35).skip(15) {
        *step = 0.50;
    }
    let q_c = 1_080.0; // threshold 0.30 veh/step at d_t = 1 s
    assert!(approx(blocked_period_steps(&profile, q_c, 1.0), 20.0, 1e-12));
    let pb = proportion_time_blocked_from_profile(&profile, q_c, 1.0, 100.0);
    assert!(approx(pb, 0.20, 1e-12), "p_b = {pb}, expected 0.20");
    // A threshold above the platoon peak blocks nothing.
    assert!(approx(
        proportion_time_blocked_from_profile(&profile, 2_000.0, 1.0, 100.0),
        0.0,
        1e-12
    ));
}

/// Directional mapping (HCM Chapter 30, Section 3): with only the eastbound
/// upstream signal present, the eastbound-fed movements (4 = WB left, 9 = NB
/// right) can be blocked while the westbound-fed movements (1 = EB left,
/// 12 = SB right) are not.
#[test]
fn test_computed_pb_direction_mapping() {
    let twsc = two_lane_twsc();
    let signals = UpstreamSignals {
        cycle_s: 100.0,
        eastbound: Some(platoon_signal(150.0, 10.0, 20.0)),
        westbound: None,
        time_step_s: 1.0,
    };
    let pb = signals.compute_platoon_blockage(&twsc);
    assert!(pb.pb4 > 0.0, "EB platoon should block movement 4, got {}", pb.pb4);
    assert!(pb.pb9 > 0.0, "EB platoon should block movement 9, got {}", pb.pb9);
    assert_eq!(pb.pb1, 0.0, "no WB signal -> movement 1 unblocked");
    assert_eq!(pb.pb12, 0.0, "no WB signal -> movement 12 unblocked");
}

/// Dispersion-flattening monotonicity (HCM Equations 30-9 through 30-12): as
/// the upstream signal moves farther away, the platoon disperses, its peak
/// arrival rate drops, and the proportion of time blocked is non-increasing.
#[test]
fn test_computed_pb_decreases_with_distance() {
    let twsc = two_lane_twsc();
    let pb_at = |distance_ft: f64| {
        UpstreamSignals {
            cycle_s: 100.0,
            eastbound: Some(platoon_signal(distance_ft, 10.0, 20.0)),
            westbound: None,
            time_step_s: 1.0,
        }
        .compute_platoon_blockage(&twsc)
        .pb4
    };
    let near = pb_at(100.0);
    let mid = pb_at(1_000.0);
    let far = pb_at(4_000.0);
    assert!(near > 0.0, "a nearby platoon should block movement 4");
    assert!(mid <= near + 1e-12, "p_b should not grow with distance: {mid} > {near}");
    assert!(far <= mid + 1e-12, "p_b should not grow with distance: {far} > {mid}");
    assert!(far < near, "dispersion should reduce p_b over distance: {far} !< {near}");
}

/// The minor-street left/through movements are blocked when a platoon is
/// present from either direction (HCM Chapter 30, Section 3), so their p_b
/// with both upstream signals is at least the one-direction value.
#[test]
fn test_computed_pb_union_of_both_directions() {
    let twsc = two_lane_twsc();
    // Offset the two platoons so their blocked steps do not fully overlap.
    let eb_only = UpstreamSignals {
        cycle_s: 100.0,
        eastbound: Some(platoon_signal(150.0, 10.0, 20.0)),
        westbound: None,
        time_step_s: 1.0,
    };
    let both = UpstreamSignals {
        cycle_s: 100.0,
        eastbound: Some(platoon_signal(150.0, 10.0, 20.0)),
        westbound: Some(platoon_signal(150.0, 55.0, 20.0)),
        time_step_s: 1.0,
    };
    let pb7_eb = eb_only.compute_platoon_blockage(&twsc).pb7;
    let pb7_both = both.compute_platoon_blockage(&twsc).pb7;
    assert!(pb7_eb > 0.0);
    assert!(
        pb7_both > pb7_eb,
        "union over both directions ({pb7_both}) should exceed one direction ({pb7_eb})"
    );
}

/// Analyst-supplied `platoon_blockage` takes precedence over `upstream_signals`
/// (the analyst-input path is unchanged).
#[test]
fn test_analyst_pb_takes_precedence_over_upstream() {
    let analyst = PlatoonBlockage {
        pb1: 0.17,
        pb4: 0.17,
        ..Default::default()
    };
    let mut t = example_problem_3();
    t.platoon_blockage = Some(analyst.clone());
    t.upstream_signals = Some(UpstreamSignals {
        cycle_s: 100.0,
        eastbound: Some(platoon_signal(150.0, 10.0, 20.0)),
        westbound: Some(platoon_signal(150.0, 10.0, 20.0)),
        time_step_s: 1.0,
    });
    t.analyze();
    assert_eq!(
        t.platoon_blockage.as_ref().unwrap(),
        &analyst,
        "explicit platoon_blockage must not be overwritten by upstream_signals"
    );
}

/// Wiring: running `analyze` with `upstream_signals` produces the same
/// end-to-end results as manually setting the computed `PlatoonBlockage` and
/// running the standard Step 5b path (no separate code path).
#[test]
fn test_computed_pb_matches_manual_platoon_blockage() {
    let signals = UpstreamSignals {
        cycle_s: 100.0,
        eastbound: Some(platoon_signal(150.0, 10.0, 20.0)),
        westbound: Some(platoon_signal(150.0, 40.0, 20.0)),
        time_step_s: 1.0,
    };

    let mut computed = example_problem_3();
    computed.upstream_signals = Some(signals.clone());
    let derived = signals.compute_platoon_blockage(&computed);
    computed.analyze();

    let mut manual = example_problem_3();
    manual.platoon_blockage = Some(derived);
    manual.analyze();

    for mv in ALL_MOVEMENTS {
        assert_eq!(
            computed.movements[mv.idx()].potential_capacity,
            manual.movements[mv.idx()].potential_capacity,
            "potential capacity differs for {mv:?}"
        );
        assert_eq!(
            computed.movements[mv.idx()].movement_capacity,
            manual.movements[mv.idx()].movement_capacity,
            "movement capacity differs for {mv:?}"
        );
    }
    assert_eq!(computed.intersection_delay, manual.intersection_delay);
}

/// A zero-platoon set of upstream signals (empty discharges) leaves the
/// pipeline bit-identical to a no-platooning run: the computed p_b is all
/// zeros, so Step 5 reduces to Equation 20-18.
#[test]
fn test_computed_pb_empty_signals_equivalent_to_off() {
    let mut a = example_problem_3();
    let mut b = example_problem_3();
    b.upstream_signals = Some(UpstreamSignals {
        cycle_s: 100.0,
        eastbound: Some(UpstreamSignal {
            distance_ft: 200.0,
            progression_speed_mph: 30.0,
            uniform_volume_veh_h: 0.0,
            discharges: Vec::new(),
        }),
        westbound: None,
        time_step_s: 1.0,
    });
    a.analyze();
    b.analyze();
    for mv in ALL_MOVEMENTS {
        assert_eq!(
            a.movements[mv.idx()].potential_capacity,
            b.movements[mv.idx()].potential_capacity,
            "potential capacity differs for {mv:?}"
        );
    }
    assert_eq!(a.intersection_delay, b.intersection_delay);
}

/// Serde round-trip through the full `Twsc` JSON with `upstream_signals`.
#[test]
fn test_upstream_signals_serde_roundtrip() {
    let mut t = two_lane_twsc();
    t.upstream_signals = Some(UpstreamSignals {
        cycle_s: 100.0,
        eastbound: Some(platoon_signal(150.0, 10.0, 20.0)),
        westbound: None,
        time_step_s: 1.0,
    });
    let json = t.to_json().unwrap();
    let back = Twsc::from_json(&json).unwrap();
    let s = back.upstream_signals.expect("upstream_signals survives round-trip");
    assert_eq!(s.cycle_s, 100.0);
    assert!(s.eastbound.is_some());
    assert!(s.westbound.is_none());
}

// ═══════════════════════════════════════════════════════════════════════════════
// Section 5 pedestrian mode (Equations 20-76 through 20-99)
// ═══════════════════════════════════════════════════════════════════════════════

use super::pedestrian::*;
use crate::hcm::common::LevelOfService;

/// A single-stage crossing with the given lane count and yield rate, using the
/// Example Problem 2 geometry so that n >= 2 potential yielding events exist.
fn ped_crossing(through_lanes: u32, motorist_yield_rate: f64) -> PedestrianCrossing {
    PedestrianCrossing {
        stages: vec![PedestrianCrossingStage {
            crossing_length_ft: 20.0,
            conflicting_flow_veh_h: 850.0,
            through_lanes,
        }],
        walk_speed_fps: 4.0,
        startup_clearance_s: 1.0,
        motorist_yield_rate,
        peak_hour_volume_veh_h: 1700.0,
        k_factor: 0.08,
        ..Default::default()
    }
}

/// `prob_all_blocking_yield` collapses Equations 20-86, 20-88, 20-90, and
/// 20-92 into one binomial sum. Because P(Y_1) = P_d * (per-event / P_d) =
/// per-event, the first yield probability is exactly the published closed form
/// for each lane count, so this pins the unification against literal
/// transcriptions of the four intact equations.
#[test]
fn test_pedestrian_yield_matches_published_closed_forms() {
    let m_y = 0.5;

    // One lane, Equation 20-86: P(Y_1) = P_d M_y, with P_d = P_b.
    let s = &ped_crossing(1, m_y).analyze().stages[0];
    let p_b = s.prob_blocked_lane;
    assert!(
        (s.prob_delayed_crossing - p_b).abs() < 1e-12,
        "one lane: Equation 20-81 must reduce to P_d = P_b"
    );
    assert!((s.prob_yield[1] - s.prob_delayed_crossing * m_y).abs() < 1e-12);
    // Equation 20-87 for i = 2: P(Y_2) = P_d M_y (1 - M_y).
    assert!((s.prob_yield[2] - s.prob_delayed_crossing * m_y * (1.0 - m_y)).abs() < 1e-12);

    // Two lanes, Equation 20-88: 2 P_b (1 - P_b) M_y + P_b^2 M_y^2.
    let s = &ped_crossing(2, m_y).analyze().stages[0];
    let p_b = s.prob_blocked_lane;
    let eq_20_88 = 2.0 * p_b * (1.0 - p_b) * m_y + p_b.powi(2) * m_y.powi(2);
    assert!((s.prob_yield[1] - eq_20_88).abs() < 1e-12);

    // Three lanes, Equation 20-90.
    let s = &ped_crossing(3, m_y).analyze().stages[0];
    let p_b = s.prob_blocked_lane;
    let eq_20_90 = p_b.powi(3) * m_y.powi(3)
        + 3.0 * p_b.powi(2) * (1.0 - p_b) * m_y.powi(2)
        + 3.0 * p_b * (1.0 - p_b).powi(2) * m_y;
    assert!((s.prob_yield[1] - eq_20_90).abs() < 1e-12);

    // Four lanes, Equation 20-92 (read from the intact numerator of 20-93).
    let s = &ped_crossing(4, m_y).analyze().stages[0];
    let p_b = s.prob_blocked_lane;
    let eq_20_92 = p_b.powi(4) * m_y.powi(4)
        + 4.0 * p_b.powi(3) * (1.0 - p_b) * m_y.powi(3)
        + 6.0 * p_b.powi(2) * (1.0 - p_b).powi(2) * m_y.powi(2)
        + 4.0 * p_b * (1.0 - p_b).powi(3) * m_y;
    assert!((s.prob_yield[1] - eq_20_92).abs() < 1e-12);
}

/// A 100% yield rate must be clamped to 99.99% per the HCM note on Equation
/// 20-87, which otherwise evaluates 0^0 at i = 1.
#[test]
fn test_pedestrian_yield_rate_clamped_at_one() {
    let s = &ped_crossing(2, 1.0).analyze().stages[0];
    assert!(
        s.prob_yield.iter().all(|p| p.is_finite()),
        "M_y = 1.0 must not produce a NaN in Equation 20-87"
    );
    assert!(s.prob_yield[1] > 0.0 && s.prob_yield[1] <= s.prob_delayed_crossing);
}

/// A zero conflicting flow must not divide by zero in Equations 20-82 and
/// 20-85; the HCM floors v at 0.0001 veh/s.
#[test]
fn test_pedestrian_zero_conflicting_flow_is_finite() {
    let mut c = ped_crossing(2, 0.5);
    c.stages[0].conflicting_flow_veh_h = 0.0;
    let r = c.analyze();
    assert!(
        r.delay.is_finite(),
        "zero conflicting flow must not divide by zero"
    );
    // With effectively no traffic, nobody is blocked and nobody is delayed.
    assert!(r.stages[0].prob_delayed_crossing < 1e-3);
    // The HCM floors v at 0.0001 veh/s rather than allowing zero, which leaves
    // a residual d_g of about v * t_c,G^2 / 2 = 0.002 s instead of exactly zero.
    assert!(
        r.delay < 0.01,
        "no conflicting traffic means negligible gap delay"
    );
    // LOS still is not A, because Exhibit 20-3 keys on satisfaction rather than
    // delay: Equation 20-95 penalizes the 21,250 veh/day AADT and credits no
    // countermeasures, so P(D, no delay) alone lands the crossing in LOS E even
    // though pedestrians never wait. This is the pedestrian mode behaving as
    // specified, not a degenerate-input artifact.
    assert!(
        (r.proportion_dissatisfied - r.prob_dissatisfied_no_delay).abs() < 1e-3,
        "with P_nd = 1 Equation 20-99 must collapse to P(D, no delay)"
    );
    assert_eq!(r.los, LevelOfService::E);
}

/// Exhibit 20-3 band edges. The bands are closed on the lower bound and open
/// on the upper, so each threshold value belongs to the worse LOS.
#[test]
fn test_pedestrian_los_bands_exhibit_20_3() {
    assert_eq!(pedestrian_los(0.0), LevelOfService::A);
    assert_eq!(pedestrian_los(0.049), LevelOfService::A);
    assert_eq!(pedestrian_los(0.05), LevelOfService::B);
    assert_eq!(pedestrian_los(0.149), LevelOfService::B);
    assert_eq!(pedestrian_los(0.15), LevelOfService::C);
    assert_eq!(pedestrian_los(0.249), LevelOfService::C);
    assert_eq!(pedestrian_los(0.25), LevelOfService::D);
    assert_eq!(pedestrian_los(0.329), LevelOfService::D);
    assert_eq!(pedestrian_los(0.33), LevelOfService::E);
    assert_eq!(pedestrian_los(0.499), LevelOfService::E);
    assert_eq!(pedestrian_los(0.50), LevelOfService::F);
    assert_eq!(pedestrian_los(1.0), LevelOfService::F);
}

/// Pedestrian platooning raises the group critical headway through Equations
/// 20-77 through 20-79, which must in turn raise delay. Without platooning
/// N_p = 1 and t_c,G collapses to t_c.
#[test]
fn test_pedestrian_platooning_raises_group_critical_headway() {
    let base = ped_crossing(2, 0.5);
    let no_platoon = base.analyze();
    assert!(
        (no_platoon.stages[0].group_critical_headway - no_platoon.stages[0].critical_headway).abs()
            < 1e-12,
        "without platooning t_c,G = t_c (Equation 20-79 with N_p = 1)"
    );

    let mut platooned = base.clone();
    platooned.pedestrian_platooning = true;
    platooned.crosswalk_width_ft = 8.0;
    platooned.pedestrian_flow_p_h = 1800.0;
    let r = platooned.analyze();
    assert!(
        r.stages[0].spatial_distribution > 1.0,
        "a narrow crosswalk with heavy pedestrian flow must give N_p > 1 row"
    );
    assert!(r.stages[0].group_critical_headway > r.stages[0].critical_headway);
    assert!(r.delay > no_platoon.delay);
}

/// Equation 20-94 sums the stages, and the AADT override bypasses the
/// peak-hour / K-factor estimate used in Example Problem 2 Step 7.
#[test]
fn test_pedestrian_stage_sum_and_aadt_override() {
    let mut c = ped_crossing(2, 0.5);
    c.stages.push(c.stages[0].clone());
    let r = c.analyze();
    assert_eq!(r.stages.len(), 2);
    assert!((r.delay - (r.stages[0].delay + r.stages[1].delay)).abs() < 1e-12);

    // 1,700 / 0.08 = 21,250, the value Example Problem 2 derives.
    let mut explicit = c.clone();
    explicit.aadt_veh = Some(21_250.0);
    explicit.peak_hour_volume_veh_h = 0.0;
    explicit.k_factor = 0.0;
    assert!(
        (explicit.analyze().proportion_dissatisfied - r.proportion_dissatisfied).abs() < 1e-12,
        "an explicit AADT must reproduce the peak-hour / K-factor estimate"
    );
}
