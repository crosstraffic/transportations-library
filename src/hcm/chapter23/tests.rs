//! Per-step unit tests for the HCM Chapter 23 interchange ramp terminal
//! methodology, validated against the exhibit tables of Chapter 23 and
//! the intermediate results of the Chapter 34 example problems.

use super::exhibits::*;
use super::ramp_terminals::*;
use crate::hcm::common::LevelOfService;

fn near(a: f64, b: f64, tol: f64) -> bool {
    (a - b).abs() <= tol
}

// ═══════════════════════════════════════════════════════════════════════════════
// LOS tables (Exhibits 23-10, 23-13, 23-14)
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_exhibit_23_10_signalized_interchange_los() {
    use LevelOfService::*;
    assert_eq!(los_signalized_interchange_od(15.0, false, false), A);
    assert_eq!(los_signalized_interchange_od(15.1, false, false), B);
    assert_eq!(los_signalized_interchange_od(30.0, false, false), B);
    assert_eq!(los_signalized_interchange_od(55.0, false, false), C);
    assert_eq!(los_signalized_interchange_od(85.0, false, false), D);
    assert_eq!(los_signalized_interchange_od(120.0, false, false), E);
    assert_eq!(los_signalized_interchange_od(120.1, false, false), F);
    // LOS F when v/c > 1 or R_Q > 1 for any lane group, regardless of ETT.
    assert_eq!(los_signalized_interchange_od(10.0, true, false), F);
    assert_eq!(los_signalized_interchange_od(10.0, false, true), F);
}

#[test]
fn test_exhibit_23_13_alternative_intersection_los() {
    use LevelOfService::*;
    assert_eq!(los_alternative_intersection_od(10.0, false, false), A);
    assert_eq!(los_alternative_intersection_od(20.0, false, false), B);
    assert_eq!(los_alternative_intersection_od(35.0, false, false), C);
    assert_eq!(los_alternative_intersection_od(55.0, false, false), D);
    assert_eq!(los_alternative_intersection_od(80.0, false, false), E);
    assert_eq!(los_alternative_intersection_od(80.1, false, false), F);
    assert_eq!(los_alternative_intersection_od(5.0, true, false), F);
}

#[test]
fn test_exhibit_23_14_roundabout_interchange_los() {
    use LevelOfService::*;
    assert_eq!(los_roundabout_interchange_od(15.0, false, false), A);
    assert_eq!(los_roundabout_interchange_od(25.0, false, false), B);
    assert_eq!(los_roundabout_interchange_od(35.0, false, false), C);
    assert_eq!(los_roundabout_interchange_od(50.0, false, false), D);
    assert_eq!(los_roundabout_interchange_od(75.0, false, false), E);
    assert_eq!(los_roundabout_interchange_od(75.1, false, false), F);
    assert_eq!(los_roundabout_interchange_od(5.0, false, true), F);
}

// ═══════════════════════════════════════════════════════════════════════════════
// Step 3: saturation flow adjustments
// ═══════════════════════════════════════════════════════════════════════════════

/// Equation 23-15 against the Exhibit 23-23 tabulation.
#[test]
fn test_exhibit_23_23_traffic_pressure() {
    assert!(near(traffic_pressure_factor(3.0, true), 0.953, 0.001));
    assert!(near(traffic_pressure_factor(3.0, false), 0.947, 0.001));
    assert!(near(traffic_pressure_factor(12.0, true), 1.011, 0.001));
    assert!(near(traffic_pressure_factor(12.0, false), 0.988, 0.001));
    assert!(near(traffic_pressure_factor(30.0, true), 1.152, 0.001));
    assert!(near(traffic_pressure_factor(30.0, false), 1.082, 0.001));
    // Demands above 30 veh/cycle/ln are capped at 30.
    assert!(near(
        traffic_pressure_factor(45.0, true),
        traffic_pressure_factor(30.0, true),
        1e-12
    ));
}

/// Equation 23-19 against the Exhibit 23-27 tabulation.
#[test]
fn test_exhibit_23_27_turn_radius() {
    assert!(near(turn_radius_factor(25.0), 0.817, 0.001));
    assert!(near(turn_radius_factor(50.0), 0.899, 0.001));
    assert!(near(turn_radius_factor(100.0), 0.947, 0.001));
    assert!(near(turn_radius_factor(150.0), 0.964, 0.001));
    assert!(near(turn_radius_factor(200.0), 0.973, 0.001));
    assert!(near(turn_radius_factor(300.0), 0.982, 0.001));
    // Chapter 34 example radii: 75 ft -> 0.930 (Exhibits 34-7 / 34-8).
    assert!(near(turn_radius_factor(75.0), 0.930, 0.001));
}

/// Equations 23-20 through 23-23: exclusive lanes take f_R directly; a
/// zero turning proportion leaves the group unadjusted.
#[test]
fn test_turn_radius_adjustments() {
    let f_r = turn_radius_factor(75.0);
    assert!(near(left_turn_radius_adjustment(1.0, f_r), f_r, 1e-12));
    assert!(near(left_turn_radius_adjustment(0.0, f_r), 1.0, 1e-12));
    assert!(near(right_turn_radius_adjustment(1.0, f_r), f_r, 1e-12));
    let shared = left_turn_radius_adjustment(0.5, f_r);
    assert!(shared > f_r && shared < 1.0);
}

/// Equation 23-16: f_LU = 1 / (%V_Lmax N), capped at 1.0.
#[test]
fn test_equation_23_16_lane_utilization_factor() {
    assert!(near(lane_utilization_factor_from_max(0.5056, 2), 0.9889, 0.0005));
    assert!(near(lane_utilization_factor_from_max(0.5, 2), 1.0, 1e-12));
    assert!(near(lane_utilization_factor_from_max(0.25, 4), 1.0, 1e-12));
}

/// Equation 23-17 with the Exhibit 23-24 diamond coefficients (Chapter 34
/// Example Problem 1 EB external inputs: v_L = 107, v_R = 89, v_T = 761,
/// D = 500 ft).
///
/// // VERIFY-HCM: Exhibit 34-6 publishes %V_Lmax = 0.5056 for these
/// // inputs; Equation 23-17 as printed yields 0.503 (leftmost lane
/// // 0.497). The equation is asserted as printed.
#[test]
fn test_equation_23_17_diamond_lane_utilization() {
    let coeffs = lane_utilization_coefficients(LaneUtilizationModel::Diamond, 2, true).unwrap();
    let pct_l1 = pct_volume_in_lane(coeffs, 2, 107.0, 89.0, 761.0, 500.0);
    assert!(near(pct_l1, 0.4969, 0.001), "got {pct_l1}");
    let pct_max =
        pct_v_lmax_arterial(LaneUtilizationModel::Diamond, 2, 107.0, 89.0, 761.0, 500.0);
    assert!(near(pct_max, 1.0 - pct_l1, 1e-9));
}

/// Exhibit 23-24: 3-lane models provide leftmost and rightmost lanes;
/// the middle lane follows by subtraction.
#[test]
fn test_exhibit_23_24_three_lane_model() {
    // Chapter 34 Example Problem 3 EB external: v_L = 1,294, v_R = 0
    // (exclusive right-turn lane), v_T = 768.
    let l1 = pct_volume_in_lane(
        lane_utilization_coefficients(LaneUtilizationModel::Diamond, 3, true).unwrap(),
        3,
        1_294.0,
        0.0,
        768.0,
        300.0,
    );
    // 1/3 + 0.465 × (1294/2062) = 0.625 as printed.
    assert!(near(l1, 0.6251, 0.001), "got {l1}");
    let l3 = pct_volume_in_lane(
        lane_utilization_coefficients(LaneUtilizationModel::Diamond, 3, false).unwrap(),
        3,
        1_294.0,
        0.0,
        768.0,
        300.0,
    );
    assert!(near(l3, 0.1287, 0.001), "got {l3}");
    let max = pct_v_lmax_arterial(LaneUtilizationModel::Diamond, 3, 1_294.0, 0.0, 768.0, 300.0);
    assert!(near(max, l1, 1e-9));
}

/// Equation 23-18 with Exhibit 23-26 against Chapter 34 Example Problem 5
/// (Exhibit 34-61): EB 3-lane exclusive at LTDR = 0.46 -> 0.45; WB 2-lane
/// shared at LTDR = 0.67 -> 0.77.
#[test]
fn test_exhibit_23_26_ddi_lane_utilization() {
    let eb = ddi_pct_v_lmax(DdiLaneConfiguration::ThreeLaneExclusive, 600.0 / 1_300.0);
    assert!(near(eb, 0.457, 0.001), "got {eb}");
    let wb = ddi_pct_v_lmax(DdiLaneConfiguration::TwoLaneShared, 300.0 / 450.0);
    assert!(near(wb, 0.770, 0.001), "got {wb}");
    // Regime boundaries switch coefficient rows.
    let low = ddi_pct_v_lmax(DdiLaneConfiguration::ThreeLaneExclusive, 0.2);
    assert!(near(low, -0.5983 * 0.2 + 0.5237, 1e-9));
}

// ═══════════════════════════════════════════════════════════════════════════════
// Step 4: common green times and lost-time adjustments
// ═══════════════════════════════════════════════════════════════════════════════

/// Common green times of Chapter 34 Exhibit 34-9 (Example Problem 1;
/// C = 160 s).
#[test]
fn test_exhibit_34_9_common_green() {
    let c = 160.0;
    let g = |b: f64, d: f64| GreenInterval {
        begin_s: b,
        duration_s: d,
    };
    // EB EXT THRU [0, 63] vs. EB INT THRU [150, 63] + [116, 34] -> 53 s.
    let eb_int = [g(150.0, 63.0), g(116.0, 34.0)];
    assert!(near(common_green_time(&[g(0.0, 63.0)], &eb_int, c), 53.0, 1e-9));
    // WB EXT THRU [150, 63] vs. WB INT THRU [0, 111] -> 53 s.
    let wb_int = [g(0.0, 111.0)];
    assert!(near(common_green_time(&[g(150.0, 63.0)], &wb_int, c), 53.0, 1e-9));
    // NB RAMP [58, 53] vs. WB INT THRU -> 53 s.
    assert!(near(common_green_time(&[g(58.0, 53.0)], &wb_int, c), 53.0, 1e-9));
    // WB INT LEFT [68, 43] vs. EB INT THRU -> 0 s (no starvation window).
    assert!(near(common_green_time(&[g(68.0, 43.0)], &eb_int, c), 0.0, 1e-9));
    // EB INT LEFT [116, 29] vs. WB INT THRU -> 0 s.
    assert!(near(common_green_time(&[g(116.0, 29.0)], &wb_int, c), 0.0, 1e-9));
}

/// Equation 23-34 against Chapter 34 Exhibit 34-37 (Example Problem 3
/// SB-L: Q_R = 108.6 ft) and Exhibit 34-10 (Example Problem 1 SB-L:
/// Q_R = 4.1 ft).
#[test]
fn test_equation_23_34_downstream_queue_length() {
    // Example 3: v_A = 2,062 veh/h on 3 lanes, G_A = 59, G_D = 71,
    // CG_RD = 27, C = 120, L_h = 25.
    let q3 = downstream_queue_length_ft(2_062.0, 3, 59.0, 71.0, 27.0, 120.0, 25.0);
    assert!(near(q3, 108.6, 0.2), "got {q3}");
    // Example 1: v_A = 868 veh/h on 2 lanes, G_A = 63, G_D = 97,
    // CG_RD = 34, C = 160.
    let q1 = downstream_queue_length_ft(868.0, 2, 63.0, 97.0, 34.0, 160.0, 25.0);
    assert!(near(q1, 4.1, 0.1), "got {q1}");
    // Negative results clamp to zero (Example 1 EB EXT-TH).
    let q0 = downstream_queue_length_ft(206.0, 1, 39.0, 97.0, 53.0, 160.0, 25.0);
    assert!(near(q0, 0.0, 1e-12));
}

/// Equation 23-30 against Chapter 34 Exhibit 34-37 (Example Problem 3
/// SB-L: L_D-R = 5.5 s with G_R = 27, DQ_R = 191.4 ft, CG_RD = 27,
/// C = 120).
#[test]
fn test_equation_23_30_downstream_queue_lost_time() {
    let l = downstream_queue_lost_time(27.0, 191.4, 27.0, 120.0);
    assert!(near(l, 5.5, 0.1), "got {l}");
    // DQ above 200 ft -> no lost time.
    assert!(near(downstream_queue_lost_time(27.0, 300.0, 27.0, 120.0), 0.0, 1e-12));
    // Negative results clamp to zero.
    assert!(near(downstream_queue_lost_time(10.0, 150.0, 27.0, 120.0), 0.0, 1e-12));
}

/// Equations 23-38 / 23-39 against Chapter 34 Exhibit 34-52 (Example
/// Problem 4): Q_initial = 6.8 / 2.8 veh, L_DS = 14.7 / 18.6 s.
#[test]
fn test_equations_23_38_39_demand_starvation() {
    // EB-INT-TH: v_RL = 191 (1 ln), v_A = 1,134 (3 ln), C = 100,
    // CG_RD = 5 (= t_L), CG_UD = 25, t_L = 5, h_I = 2.23.
    let q_eb = demand_starvation_initial_queue(191.0, 1, 1_134.0, 3, 100.0, 5.0, 25.0, 5.0, 2.23);
    assert!(near(q_eb, 6.84, 0.05), "got {q_eb}");
    let lds_eb = demand_starvation_lost_time(30.0, q_eb, 2.23);
    assert!(near(lds_eb, 14.7, 0.15), "got {lds_eb}");
    // WB-INT-TH: v_RL = 129, v_A = 1,119, CG_UD = 30, h_I = 2.25.
    let q_wb = demand_starvation_initial_queue(129.0, 1, 1_119.0, 3, 100.0, 5.0, 30.0, 5.0, 2.25);
    assert!(near(q_wb, 2.83, 0.05), "got {q_wb}");
    let lds_wb = demand_starvation_lost_time(25.0, q_wb, 2.25);
    assert!(near(lds_wb, 18.6, 0.15), "got {lds_wb}");
    // Zero starvation window -> zero lost time.
    assert!(near(demand_starvation_lost_time(0.0, 1.0, 2.0), 0.0, 1e-12));
}

/// Equations 23-24 / 23-27: adjusted lost time and effective green
/// (Example Problem 3 SB-L: t_L' = 10.5 s, g' = 21.5 s).
#[test]
fn test_adjusted_lost_time_and_effective_green() {
    let tl = adjusted_lost_time(2.0, 5.5, 5.0, 2.0);
    assert!(near(tl, 10.5, 1e-9));
    let g_eff = 27.0 + 5.0 - tl;
    assert!(near(g_eff, 21.5, 1e-9));
}

/// Equation 23-37 as printed (W + L − D)/(1.467 S_f).
///
/// // VERIFY-HCM: Chapter 34 Exhibit 34-63 publishes 6.5 s / 4.9 s for
/// // (W, L, D) = (200, 20, 20) and (100, 20, 60) at 25 mi/h, which
/// // correspond to (W + L + D)/(1.467 S_f); the printed equation
/// // subtracts D. Asserted as printed.
#[test]
fn test_equation_23_37_ddi_overlap_lost_time() {
    let l = ddi_overlap_lost_time(200.0, 20.0, 20.0, 25.0);
    assert!(near(l, 200.0 / (1.467 * 25.0), 1e-9));
    // Negative distances clamp to zero.
    assert!(near(ddi_overlap_lost_time(10.0, 20.0, 100.0, 25.0), 0.0, 1e-12));
}

// ═══════════════════════════════════════════════════════════════════════════════
// Step 6: YIELD-controlled DDI turns (Chapter 34 Example Problem 6)
// ═══════════════════════════════════════════════════════════════════════════════

/// Equations 23-54 / 23-56 / 23-53 against Exhibit 34-67 (M7): t_CQ =
/// 22.4 s, t_clear = 5.5 s, p_b = 0.40.
#[test]
fn test_exhibit_34_67_blocked_regime() {
    let t_cq = yield_time_to_clear_queue_random(39.0, 433.0, 1_188.0);
    assert!(near(t_cq, 22.4, 0.1), "got {t_cq}");
    let t_clear = yield_clearance_time(200.0, 25.0);
    assert!(near(t_clear, 5.5, 0.1), "got {t_clear}");
    let p_b = (t_cq + t_clear) / 70.0; // Equation 23-53
    assert!(near(p_b, 0.40, 0.005), "got {p_b}");
    // M8: t_CQ = 8.5 s, t_clear = 2.7 s, p_b = 0.16.
    let t_cq8 = yield_time_to_clear_queue_random(45.0, 250.0, 1_578.0);
    assert!(near(t_cq8, 8.5, 0.1), "got {t_cq8}");
    assert!(near(yield_clearance_time(100.0, 25.0), 2.7, 0.05));
    assert!(near(
        (t_cq8 + yield_clearance_time(100.0, 25.0)) / 70.0,
        0.16,
        0.005
    ));
}

/// Equation 23-55 reduces to Equation 23-54 when P = g/C.
#[test]
fn test_equation_23_55_coordinated_reduces_to_random() {
    let (c, g) = (70.0, 31.0);
    let coord = yield_time_to_clear_queue_coordinated(c, g / c, 433.0, 1_188.0, g);
    let random = yield_time_to_clear_queue_random(c - g, 433.0, 1_188.0);
    assert!(near(coord, random, 1e-9), "coord {coord} vs random {random}");
    // Better progression (higher P) shortens the conflicting queue.
    let better = yield_time_to_clear_queue_coordinated(c, 0.8, 433.0, 1_188.0, g);
    assert!(better < coord);
}

/// Equations 23-42 / 23-44 against Exhibit 34-68 / 34-69 (Exhibit 23-36
/// default headways): c_GA(M7) = 541, c_GA(M8) = 1,380, c_GA(M3) = 1,000,
/// c_GA(M4) = 1,228; c_NCF = 1,385 (left) / 1,500 (right).
#[test]
fn test_exhibit_34_68_gap_acceptance_capacity() {
    let m7 = yield_gap_acceptance_capacity(
        DDI_LEFT_CRITICAL_HEADWAY_S,
        DDI_LEFT_FOLLOW_UP_HEADWAY_S,
        1_300.0,
    );
    assert!(near(m7, 541.0, 2.0), "got {m7}");
    let m8 = yield_gap_acceptance_capacity(
        DDI_RIGHT_CRITICAL_HEADWAY_S,
        DDI_RIGHT_FOLLOW_UP_HEADWAY_S,
        500.0,
    );
    assert!(near(m8, 1_380.0, 3.0), "got {m8}");
    let m3 = yield_gap_acceptance_capacity(3.9, 2.6, 450.0);
    assert!(near(m3, 1_000.0, 3.0), "got {m3}");
    let m4 = yield_gap_acceptance_capacity(1.8, 2.4, 1_200.0);
    assert!(near(m4, 1_228.0, 3.0), "got {m4}");
    assert!(near(yield_no_conflict_capacity(2.6), 1_385.0, 1.0));
    assert!(near(yield_no_conflict_capacity(2.4), 1_500.0, 1.0));
}

/// Equation 23-47 against Exhibit 34-70 (M7 v/c = 0.38, M8 v/c = 0.16
/// with the Exhibit 34-67 / 34-69 regime proportions).
#[test]
fn test_equation_23_47_yield_capacity() {
    let cap_m7 = yield_turn_capacity(70.0, 31.0, 22.37, 5.44, 541.0, 1_385.0);
    assert!(near(300.0 / cap_m7, 0.38, 0.01), "v/c {}", 300.0 / cap_m7);
    let cap_m8 = yield_turn_capacity(70.0, 25.0, 8.48, 2.72, 1_380.0, 1_500.0);
    assert!(near(200.0 / cap_m8, 0.16, 0.01), "v/c {}", 200.0 / cap_m8);
    // A negative gap-acceptance interval floors at zero (Exhibit 34-68
    // note for M3), leaving only the Regime 3 capacity.
    let cap_floor = yield_turn_capacity(70.0, 20.0, 16.0, 5.5, 1_000.0, 1_385.0);
    assert!(near(cap_floor, 1_385.0 * 50.0 / 70.0, 1.0), "got {cap_floor}");
}

// ═══════════════════════════════════════════════════════════════════════════════
// Step 8: extra distance travel time (Equation 23-50)
// ═══════════════════════════════════════════════════════════════════════════════

/// Chapter 34 Example Problem 1: 100 ft at 35 mi/h -> ±1.9 s/veh.
#[test]
fn test_equation_23_50_edtt() {
    assert!(near(extra_distance_travel_time(100.0, 35.0, 0.0), 1.9, 0.05));
    assert!(near(extra_distance_travel_time(-100.0, 35.0, 0.0), -1.9, 0.05));
    // 40 ft crossover shift at a DDI -> 0.8 s (Exhibit 34-65).
    assert!(near(extra_distance_travel_time(40.0, 35.0, 0.0), 0.8, 0.05));
    // Loop ramp: the 5-s deceleration/acceleration term adds on.
    assert!(near(
        extra_distance_travel_time(1_200.0, 25.0, EDTT_LOOP_RAMP_ACCEL_DECEL_S),
        1_200.0 / (1.47 * 25.0) + 5.0,
        1e-9
    ));
}

// ═══════════════════════════════════════════════════════════════════════════════
// Step 1: O-D / turning movement conversion (Exhibits 34-163..34-177)
// ═══════════════════════════════════════════════════════════════════════════════

fn sample_od() -> OdDemands {
    OdDemands {
        a: 210.0,
        b: 204.0,
        c: 156.0,
        d: 185.0,
        e: 96.0,
        f: 80.0,
        g: 135.0,
        h: 212.0,
        i: 685.0,
        j: 585.0,
        k: 11.0,
        l: 13.0,
        m: 17.0,
        n: 19.0,
    }
}

/// Exhibit 34-176 diamond compositions as printed (EXT-TH = I + E,
/// INT-LT = E + N, INT-TH = I + D, NB LT = A + M, ...).
#[test]
fn test_exhibit_34_176_diamond_turning_movements() {
    let od = sample_od();
    let tm = turning_movements_from_od(InterchangeForm::Diamond, &od);
    assert!(near(tm.eb_ext_right, od.f, 1e-12));
    assert!(near(tm.eb_ext_through, od.i + od.e, 1e-12));
    assert!(near(tm.eb_int_left, od.e + od.n, 1e-12));
    assert!(near(tm.eb_int_through, od.i + od.d, 1e-12));
    assert!(near(tm.wb_int_left, od.h + od.m, 1e-12));
    assert!(near(tm.wb_int_through, od.j + od.a, 1e-12));
    assert!(near(tm.wb_ext_right, od.g, 1e-12));
    assert!(near(tm.wb_ext_through, od.j + od.h, 1e-12));
    assert!(near(tm.nb_left, od.a + od.m, 1e-12));
    assert!(near(tm.nb_right, od.b, 1e-12));
    assert!(near(tm.sb_left, od.d + od.n, 1e-12));
    assert!(near(tm.sb_right, od.c, 1e-12));
    assert!(near(tm.nb_uturn, od.m, 1e-12));
    assert!(near(tm.sb_uturn, od.n, 1e-12));
}

/// Every form round-trips O-D -> turning movements -> O-D (the Exhibit
/// 34-163..34-170 worksheets are the inverses of Exhibits
/// 34-171..34-177).
#[test]
fn test_od_turning_movement_round_trip() {
    let od = sample_od();
    for form in [
        InterchangeForm::Diamond,
        InterchangeForm::Ddi,
        InterchangeForm::ParcloA2Q,
        InterchangeForm::ParcloA4Q,
        InterchangeForm::ParcloAB2Q,
        InterchangeForm::ParcloAB4Q,
        InterchangeForm::ParcloB2Q,
        InterchangeForm::ParcloB4Q,
        InterchangeForm::Spui,
    ] {
        let tm = turning_movements_from_od(form, &od);
        let back = od_from_turning_movements(form, &tm);
        for m in OdMovement::ALL {
            assert!(
                near(back.get(m), od.get(m), 1e-9),
                "{form:?} O-D {m:?}: {} != {}",
                back.get(m),
                od.get(m)
            );
        }
    }
}

/// Chapter 34 Exhibit 34-5 (Example Problem 1): PHF adjustment of the
/// O-D table (PHF = 0.90).
#[test]
fn test_exhibit_34_5_phf_adjustment() {
    let od = sample_od();
    let adj = od.phf_adjusted(0.90);
    assert!(near(adj.a, 233.0, 0.5));
    assert!(near(adj.b, 227.0, 0.5));
    assert!(near(adj.i, 761.0, 0.5));
    assert!(near(adj.j, 650.0, 0.5));
}

// ═══════════════════════════════════════════════════════════════════════════════
// Exhibit 34-161: roundabout interchange O-D composition
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_exhibit_34_161_roundabout_movements() {
    use OdMovement::*;
    // "For diamond interchanges, O-D Movements G, H, and J constitute
    // Movement 15" (Chapter 34 Section 4 text).
    assert_eq!(
        roundabout_movement_ods(RoundaboutInterchangeForm::Diamond, 15),
        Some(&[G, H, J][..])
    );
    // Nonexistent movements return None (e.g., diamond Movement 5).
    assert_eq!(
        roundabout_movement_ods(RoundaboutInterchangeForm::Diamond, 5),
        None
    );
    // SPUI listing stops at Movement 8.
    assert_eq!(
        roundabout_movement_ods(RoundaboutInterchangeForm::Spui, 9),
        None
    );
    assert_eq!(
        roundabout_movement_ods(RoundaboutInterchangeForm::ParcloA2Q, 16),
        Some(&[A, G, H, J, M][..])
    );
}

// ═══════════════════════════════════════════════════════════════════════════════
// Facility pipeline tests
// ═══════════════════════════════════════════════════════════════════════════════

fn green(b: f64, d: f64) -> GreenInterval {
    GreenInterval {
        begin_s: b,
        duration_s: d,
    }
}

fn lane_group(
    mv: InterchangeMovement,
    lanes: u32,
    greens: Vec<GreenInterval>,
    y: f64,
) -> LaneGroupInput {
    let mut x = LaneGroupInput::new(mv, lanes, greens[0], y);
    x.greens = greens;
    x
}

fn minimal_diamond() -> Interchange {
    use InterchangeMovement::*;
    let od = sample_od();
    let mut ix = Interchange::new(InterchangeForm::Diamond, 160.0, od);
    ix.peak_hour_factor = 0.9;
    ix.distance_between_intersections_ft = 500.0;
    ix.lane_groups = vec![
        lane_group(EbExtThrough, 2, vec![green(0.0, 63.0)], 5.0),
        lane_group(
            EbIntThrough,
            2,
            vec![green(150.0, 63.0), green(116.0, 34.0)],
            5.0,
        ),
        lane_group(EbIntLeft, 1, vec![green(116.0, 29.0)], 5.0),
        lane_group(WbExtThrough, 2, vec![green(150.0, 63.0)], 5.0),
        lane_group(WbIntThrough, 2, vec![green(0.0, 111.0)], 5.0),
        lane_group(WbIntLeft, 1, vec![green(68.0, 43.0)], 5.0),
        lane_group(NbRampLeft, 1, vec![green(58.0, 53.0)], 5.0),
        lane_group(NbRampRight, 1, vec![green(58.0, 53.0)], 5.0),
        lane_group(SbRampLeft, 1, vec![green(116.0, 39.0)], 5.0),
        lane_group(SbRampRight, 1, vec![green(116.0, 39.0)], 5.0),
    ];
    ix
}

/// The Step 1 lane-group demand composition matches the Exhibit 34-176
/// worksheet (Example Problem 1 flows: EB EXT = 957, WB EXT = 1,036,
/// EB INT-TH = 967, WB INT-TH = 883 veh/h after PHF, ignoring the small
/// K/L/M/N demands of the sample O-D table).
#[test]
fn test_step_1_lane_group_demands() {
    let mut ix = minimal_diamond();
    ix.od.k = 0.0;
    ix.od.l = 0.0;
    ix.od.m = 0.0;
    ix.od.n = 0.0;
    ix.step_1_od_and_movement_demands();
    let flow = |ix: &Interchange, m| {
        ix.results
            .iter()
            .find(|r| r.movement == m)
            .unwrap()
            .flow_rate
    };
    use InterchangeMovement::*;
    assert!(near(flow(&ix, EbExtThrough), 957.0, 2.0));
    assert!(near(flow(&ix, WbExtThrough), 1_036.0, 2.0));
    assert!(near(flow(&ix, EbIntThrough), 967.0, 2.0));
    assert!(near(flow(&ix, WbIntThrough), 883.0, 2.0));
    assert!(near(flow(&ix, EbIntLeft), 107.0, 2.0));
    assert!(near(flow(&ix, NbRampLeft), 233.0, 2.0));
    assert!(near(flow(&ix, SbRampRight), 173.0, 1.0));
}

/// Full pipeline produces O-D results with finite ETT and a LOS for
/// every demanded O-D, and an interchange LOS.
#[test]
fn test_full_pipeline_smoke() {
    let mut ix = minimal_diamond();
    ix.analyze();
    assert!(!ix.od_results.is_empty());
    for r in &ix.od_results {
        assert!(r.ett_s.is_finite(), "O-D {:?} ETT not finite", r.movement);
    }
    assert!(ix.interchange_ett_s.unwrap() > 0.0);
    assert!(ix.interchange_los.is_some());
    // U-turn O-Ds (M, N) traverse the ramp left + opposite internal left.
    assert!(ix
        .od_results
        .iter()
        .any(|r| matches!(r.movement, OdMovement::M)));
}

/// Demand starvation appears when the internal through green overlaps
/// the opposing internal left green (Chapter 34 Example Problem 4
/// pattern): the engine computes a positive L_DS and shortens the
/// internal effective green (published: L_DS = 14.7 s, g'' = 45.3 s).
#[test]
fn test_step_4_demand_starvation_engaged() {
    use InterchangeMovement::*;
    // O-D demands matching the Exhibit 34-51/34-52 feed flows:
    // EB arterial feed I + E = 1,134, EB ramp feed D + N = 191,
    // WB arterial feed J + H = 1,119, WB ramp feed A + M = 129 veh/h.
    let od = OdDemands {
        a: 129.0,
        b: 100.0,
        c: 100.0,
        d: 191.0,
        e: 334.0,
        f: 100.0,
        g: 100.0,
        h: 369.0,
        i: 800.0,
        j: 750.0,
        ..Default::default()
    };
    let mut ix = Interchange::new(InterchangeForm::Diamond, 100.0, od);
    ix.peak_hour_factor = 1.0;
    ix.distance_between_intersections_ft = 400.0;
    // Example Problem 4 timing (Exhibit 34-50): zero offset; internal
    // throughs green 0..60 while the opposing internal lefts (0..30 /
    // 35..60) are also green, creating starvation potential.
    ix.lane_groups = vec![
        lane_group(EbExtThrough, 3, vec![green(35.0, 25.0)], 5.0),
        lane_group(EbIntThrough, 3, vec![green(0.0, 60.0)], 5.0),
        lane_group(EbIntLeft, 1, vec![green(35.0, 25.0)], 5.0),
        lane_group(WbExtThrough, 3, vec![green(0.0, 30.0)], 5.0),
        lane_group(WbIntThrough, 3, vec![green(0.0, 60.0)], 5.0),
        lane_group(WbIntLeft, 1, vec![green(0.0, 30.0)], 5.0),
        lane_group(NbRampLeft, 1, vec![green(65.0, 30.0)], 5.0),
        lane_group(NbRampRight, 1, vec![green(65.0, 30.0)], 5.0),
        lane_group(SbRampLeft, 1, vec![green(65.0, 30.0)], 5.0),
        lane_group(SbRampRight, 1, vec![green(65.0, 30.0)], 5.0),
    ];
    // Heavy vehicles on the internal throughs shift h_I toward the
    // published 2.23 s value.
    for g in ix.lane_groups.iter_mut() {
        if matches!(g.movement, EbIntThrough | WbIntThrough) {
            g.pct_heavy_vehicles = 6.1;
        }
    }
    ix.analyze();
    let int_th = ix
        .results
        .iter()
        .find(|r| r.movement == EbIntThrough)
        .unwrap();
    let lds = int_th.demand_starvation_lost_time_s.unwrap();
    assert!(lds > 5.0, "expected sizable starvation lost time, got {lds}");
    // Chapter 34 Exhibit 34-52 publishes L_DS = 14.7 s for the EB
    // internal through (saturation-flow differences shift h_I slightly).
    assert!(near(lds, 14.7, 2.0), "got {lds}");
    let g_eff = int_th.effective_green_s.unwrap();
    assert!(near(g_eff, 45.3, 2.0), "got {g_eff}");
}
