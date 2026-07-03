//! Unit tests for the HCM Chapter 19 motorized vehicle methodology.
//!
//! Published reference values come from HCM 7th Edition, Chapter 31
//! (Signalized Intersections: Supplemental):
//! * Example Problem 1 (Exhibits 31-69 through 31-82) — intersection of
//!   5th Avenue and 12th Street; the converged phase durations of
//!   Exhibit 31-79 are supplied as fixed timing.
//! * The pretimed phase duration example of Section 2 (Exhibit 31-7).
//!
//! Tolerances: published exhibit values are rounded ("six or more
//! significant digits" are carried internally per the exhibit notes), and
//! the shared-lane lane-flow procedure reproduces the published lane flows
//! within about 2 veh/h, so intermediate values are asserted with
//! commensurate tolerances (documented per test).

use super::*;
use crate::hcm::common::intersection::Direction;
use crate::hcm::common::LevelOfService;

fn example_problem_1() -> SignalizedIntersection {
    let json = include_str!("../../../tests/ExampleCases/hcm/Signalized/case1.json");
    let mut ix: SignalizedIntersection = serde_json::from_str(json).expect("case1.json parses");
    ix.analyze();
    ix
}

fn group<'a>(
    ix: &'a SignalizedIntersection,
    dir: Direction,
    kind: LaneGroupKind,
) -> &'a LaneGroup {
    ix.lane_groups
        .iter()
        .find(|lg| lg.direction == dir && lg.kind == kind)
        .expect("lane group present")
}

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

// ═══════════════════════════════════════════════════════════════════════════
// Steps 1–3: movement groups and lane group flow rates
// ═══════════════════════════════════════════════════════════════════════════

/// HCM Exhibit 31-75: movement group flow rates (RTOR subtracted).
#[test]
fn test_step_2_movement_group_flow_rates() {
    let ix = example_problem_1();
    // The through+right movement group flow equals the sum of the through
    // and shared lane group flows.
    let mg = |d: Direction| {
        group(&ix, d, LaneGroupKind::ExclusiveThrough).flow_rate
            + group(&ix, d, LaneGroupKind::SharedRightThrough).flow_rate
    };
    assert_near!(mg(Direction::EB), 424.0, 0.5, "EB T+R movement group");
    assert_near!(mg(Direction::WB), 624.0, 0.5, "WB T+R movement group");
    assert_near!(mg(Direction::NB), 1_733.0, 0.5, "NB T+R movement group");
    assert_near!(mg(Direction::SB), 1_011.0, 0.5, "SB T+R movement group");
    // Left-turn movement groups pass through unchanged.
    assert_near!(
        group(&ix, Direction::NB, LaneGroupKind::ExclusiveLeft).flow_rate,
        133.0,
        1e-9,
        "NB L movement group"
    );
}

/// HCM Exhibit 31-76: lane group flow rates from the Chapter 31 Section 2
/// multiple-lane-approach procedure. Tolerance ±3 veh/h (the published
/// engine output differs from the transcribed procedure by <2 veh/h).
#[test]
fn test_step_3_lane_group_flow_rates() {
    let ix = example_problem_1();
    let cases = [
        (Direction::EB, 71.0, 239.0, 185.0),
        (Direction::WB, 118.0, 337.0, 287.0),
        (Direction::NB, 133.0, 870.0, 863.0),
        (Direction::SB, 194.0, 513.0, 497.0),
    ];
    for (dir, v_l, v_t, v_tr) in cases {
        assert_near!(
            group(&ix, dir, LaneGroupKind::ExclusiveLeft).flow_rate,
            v_l,
            0.5,
            format!("{dir:?} L lane group flow")
        );
        assert_near!(
            group(&ix, dir, LaneGroupKind::ExclusiveThrough).flow_rate,
            v_t,
            3.0,
            format!("{dir:?} T lane group flow")
        );
        assert_near!(
            group(&ix, dir, LaneGroupKind::SharedRightThrough).flow_rate,
            v_tr,
            3.0,
            format!("{dir:?} T+R lane group flow")
        );
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Step 4: adjusted saturation flow rates
// ═══════════════════════════════════════════════════════════════════════════

/// HCM Equation 31-100 (Example Problem 1, Step 4 text): the eastbound
/// permitted left-turn saturation flow rate is 813 veh/h/ln for an
/// opposing flow of 624 veh/h.
#[test]
fn test_permitted_left_saturation_flow_eq_31_100() {
    assert_near!(permitted_left_saturation_flow(624.0), 813.0, 1.0, "s_p EB");
    // EL1 = s_o / s_p (Eq. 31-101).
    let el1 = el1_permitted_left(1_900.0, permitted_left_saturation_flow(624.0));
    assert_near!(el1, 1_900.0 / 813.0, 0.01, "EL1 EB");
}

/// HCM Exhibit 31-77: adjusted saturation flow rate per lane group.
/// Tolerance ±10 veh/h/ln.
#[test]
fn test_step_4_adjusted_saturation_flow() {
    let ix = example_problem_1();
    let cases = [
        (Direction::EB, 702.0, 1_643.0, 1_201.0),
        (Direction::WB, 825.0, 1_643.0, 1_398.0),
        (Direction::NB, 1_603.0, 1_683.0, 1_648.0),
        (Direction::SB, 1_603.0, 1_683.0, 1_630.0),
    ];
    for (dir, s_l, s_t, s_tr) in cases {
        assert_near!(
            group(&ix, dir, LaneGroupKind::ExclusiveLeft)
                .sat_flow
                .unwrap(),
            s_l,
            10.0,
            format!("{dir:?} L sat flow")
        );
        assert_near!(
            group(&ix, dir, LaneGroupKind::ExclusiveThrough)
                .sat_flow
                .unwrap(),
            s_t,
            10.0,
            format!("{dir:?} T sat flow")
        );
        assert_near!(
            group(&ix, dir, LaneGroupKind::SharedRightThrough)
                .sat_flow
                .unwrap(),
            s_tr,
            10.0,
            format!("{dir:?} T+R sat flow")
        );
    }
}

/// HCM Exhibit 31-79 (last row): duration of the permitted left-turn green
/// not blocked by the opposing queue g_u. Published: 11.4 (EB), 17.0 (WB),
/// 32.5 (NB), 0.0 (SB) s.
#[test]
fn test_unblocked_permitted_green_exhibit_31_79() {
    let ix = example_problem_1();
    let gu = |d| group(&ix, d, LaneGroupKind::ExclusiveLeft).g_u.unwrap();
    assert_near!(gu(Direction::EB), 11.4, 0.3, "g_u EB");
    assert_near!(gu(Direction::WB), 17.0, 0.3, "g_u WB");
    assert_near!(gu(Direction::NB), 32.5, 0.3, "g_u NB");
    assert_near!(gu(Direction::SB), 0.0, 1e-9, "g_u SB");
}

/// Pedestrian–bicycle adjustment factors of Exhibit 31-77: f_Rpb = 0.88
/// (east–west, 120 p/h) and 0.98 (north–south, 40 p/h); f_Lpb = 1.00 (EB)
/// and 0.98 (WB).
#[test]
fn test_ped_bike_factors_exhibit_31_77() {
    // f_Rpb for the EB approach: g = 30 s, g_ped = min(30, 5+14) = 19 s,
    // 2 receiving lanes > 1 turn lane.
    let f_rpb_eb = ped_bike_factor_right(120.0, 0.0, 30.0, 19.0, 101.8, true);
    assert_near!(f_rpb_eb, 0.88, 0.005, "f_Rpb EB");
    // f_Rpb for the NB approach: g = 50 s, g_ped = min(50, 5+16) = 21 s.
    let f_rpb_nb = ped_bike_factor_right(40.0, 0.0, 50.0, 21.0, 101.8, true);
    assert_near!(f_rpb_nb, 0.98, 0.005, "f_Rpb NB");
    // f_Lpb for the EB left (the opposing queue consumes almost the whole
    // pedestrian service time): ~1.00.
    let f_lpb_eb = ped_factor_left_two_way(120.0, 624.0, 30.0, 11.44, 19.0, 101.8, true);
    assert_near!(f_lpb_eb, 1.00, 0.005, "f_Lpb EB");
    // f_Lpb for the WB left: ~0.98.
    let f_lpb_wb = ped_factor_left_two_way(120.0, 424.0, 30.0, 17.06, 19.0, 101.8, true);
    assert_near!(f_lpb_wb, 0.98, 0.01, "f_Lpb WB");
    // No pedestrians -> both factors are 1.0.
    assert_eq!(ped_bike_factor_right(0.0, 0.0, 30.0, 19.0, 100.0, true), 1.0);
    assert_eq!(
        ped_factor_left_two_way(0.0, 500.0, 30.0, 10.0, 19.0, 100.0, true),
        1.0
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// Step 5: proportion arriving during green (Exhibit 31-78)
// ═══════════════════════════════════════════════════════════════════════════

/// HCM Exhibit 31-78: effective green and P = R_p g/C per lane group.
#[test]
fn test_step_5_proportion_arriving_on_green() {
    let ix = example_problem_1();
    let cases = [
        (Direction::EB, LaneGroupKind::ExclusiveLeft, 30.0, 0.29),
        (Direction::EB, LaneGroupKind::ExclusiveThrough, 30.0, 0.29),
        (Direction::NB, LaneGroupKind::ExclusiveLeft, 6.2, 0.06),
        (Direction::NB, LaneGroupKind::ExclusiveThrough, 50.0, 0.49),
        (Direction::SB, LaneGroupKind::ExclusiveLeft, 9.8, 0.10),
        (Direction::SB, LaneGroupKind::SharedRightThrough, 53.6, 0.53),
    ];
    for (dir, kind, g, p) in cases {
        let lg = group(&ix, dir, kind);
        assert_near!(
            lg.effective_green_s.unwrap(),
            g,
            0.05,
            format!("{dir:?} {kind:?} g")
        );
        assert_near!(
            lg.proportion_on_green.unwrap(),
            p,
            0.005,
            format!("{dir:?} {kind:?} P")
        );
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Step 7: capacity and volume-to-capacity ratio (Exhibit 31-80)
// ═══════════════════════════════════════════════════════════════════════════

/// HCM Exhibit 31-80: capacity (±6 veh/h) and v/c ratio (±0.02).
#[test]
fn test_step_7_capacity_and_vc_ratio() {
    let ix = example_problem_1();
    let cases = [
        (Direction::EB, LaneGroupKind::ExclusiveLeft, 149.0, 0.47),
        (Direction::EB, LaneGroupKind::ExclusiveThrough, 484.0, 0.49),
        (Direction::EB, LaneGroupKind::SharedRightThrough, 354.0, 0.52),
        (Direction::WB, LaneGroupKind::ExclusiveLeft, 208.0, 0.57),
        (Direction::WB, LaneGroupKind::ExclusiveThrough, 484.0, 0.70),
        (Direction::WB, LaneGroupKind::SharedRightThrough, 412.0, 0.70),
        (Direction::NB, LaneGroupKind::ExclusiveLeft, 328.0, 0.41),
        (Direction::NB, LaneGroupKind::ExclusiveThrough, 827.0, 1.05),
        (Direction::NB, LaneGroupKind::SharedRightThrough, 809.0, 1.07),
        (Direction::SB, LaneGroupKind::ExclusiveLeft, 225.0, 0.86),
        (Direction::SB, LaneGroupKind::ExclusiveThrough, 887.0, 0.58),
        (Direction::SB, LaneGroupKind::SharedRightThrough, 859.0, 0.58),
    ];
    for (dir, kind, c, x) in cases {
        let lg = group(&ix, dir, kind);
        assert_near!(lg.capacity.unwrap(), c, 6.0, format!("{dir:?} {kind:?} c"));
        assert_near!(lg.vc_ratio.unwrap(), x, 0.02, format!("{dir:?} {kind:?} X"));
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Step 8: delay (Exhibit 31-81)
// ═══════════════════════════════════════════════════════════════════════════

/// HCM Exhibit 31-81: uniform delay d1. Through and shared lane groups use
/// Equation 19-19 (±0.5 s/veh); permitted / protected-permitted left-turn
/// lane groups use the incremental queue accumulation procedure, which
/// reproduces the computational engine within ±1.5 s/veh (the engine's
/// polygon carries interval detail beyond what the text publishes).
#[test]
fn test_step_8_uniform_delay() {
    let ix = example_problem_1();
    let cases = [
        (Direction::EB, LaneGroupKind::ExclusiveLeft, 44.6, 1.5),
        (Direction::EB, LaneGroupKind::ExclusiveThrough, 29.6, 0.5),
        (Direction::EB, LaneGroupKind::SharedRightThrough, 29.9, 0.5),
        (Direction::WB, LaneGroupKind::ExclusiveLeft, 41.3, 1.5),
        (Direction::WB, LaneGroupKind::ExclusiveThrough, 31.9, 0.5),
        (Direction::WB, LaneGroupKind::SharedRightThrough, 31.9, 0.5),
        (Direction::NB, LaneGroupKind::ExclusiveLeft, 13.2, 1.5),
        (Direction::NB, LaneGroupKind::ExclusiveThrough, 25.9, 0.5),
        (Direction::NB, LaneGroupKind::SharedRightThrough, 25.9, 0.5),
        (Direction::SB, LaneGroupKind::ExclusiveLeft, 28.9, 1.5),
        (Direction::SB, LaneGroupKind::ExclusiveThrough, 16.4, 0.5),
        (Direction::SB, LaneGroupKind::SharedRightThrough, 16.4, 0.5),
    ];
    for (dir, kind, d1, tol) in cases {
        assert_near!(
            group(&ix, dir, kind).uniform_delay_s.unwrap(),
            d1,
            tol,
            format!("{dir:?} {kind:?} d1")
        );
    }
}

/// HCM Exhibit 31-81: incremental delay d2 with the actuated incremental
/// delay factor k (Equations 19-22 through 19-25). Tolerance ±1.0 s/veh
/// (d2 of the oversaturated NB lane groups is sensitive to the ±2 veh/h
/// lane-flow differences).
#[test]
fn test_step_8_incremental_delay() {
    let ix = example_problem_1();
    let cases = [
        (Direction::EB, LaneGroupKind::ExclusiveLeft, 0.9),
        (Direction::EB, LaneGroupKind::ExclusiveThrough, 0.3),
        (Direction::EB, LaneGroupKind::SharedRightThrough, 0.7),
        (Direction::WB, LaneGroupKind::ExclusiveLeft, 2.3),
        (Direction::WB, LaneGroupKind::ExclusiveThrough, 3.6),
        (Direction::WB, LaneGroupKind::SharedRightThrough, 4.3),
        (Direction::NB, LaneGroupKind::ExclusiveLeft, 0.3),
        (Direction::NB, LaneGroupKind::ExclusiveThrough, 46.0),
        (Direction::NB, LaneGroupKind::SharedRightThrough, 50.8),
        (Direction::SB, LaneGroupKind::ExclusiveLeft, 3.8),
        (Direction::SB, LaneGroupKind::ExclusiveThrough, 0.6),
        (Direction::SB, LaneGroupKind::SharedRightThrough, 0.7),
    ];
    for (dir, kind, d2) in cases {
        assert_near!(
            group(&ix, dir, kind).incremental_delay_s.unwrap(),
            d2,
            1.0,
            format!("{dir:?} {kind:?} d2")
        );
    }
    // No initial queue: d3 = 0 for every lane group (HCM Ch. 19 Step 8B).
    for lg in &ix.lane_groups {
        assert_eq!(lg.initial_queue_delay_s, Some(0.0));
    }
}

/// HCM Exhibit 31-81: lane group control delay and LOS (LOS exact).
#[test]
fn test_step_8_control_delay_and_lane_group_los() {
    let ix = example_problem_1();
    use LevelOfService::*;
    let cases = [
        (Direction::EB, LaneGroupKind::ExclusiveLeft, 45.5, 1.5, D),
        (Direction::EB, LaneGroupKind::ExclusiveThrough, 29.9, 0.5, C),
        (Direction::EB, LaneGroupKind::SharedRightThrough, 30.6, 0.5, C),
        (Direction::WB, LaneGroupKind::ExclusiveLeft, 43.5, 1.5, D),
        (Direction::WB, LaneGroupKind::ExclusiveThrough, 35.5, 0.5, D),
        (Direction::WB, LaneGroupKind::SharedRightThrough, 36.2, 0.5, D),
        (Direction::NB, LaneGroupKind::ExclusiveLeft, 13.5, 1.5, B),
        (Direction::NB, LaneGroupKind::ExclusiveThrough, 72.0, 1.5, F),
        (Direction::NB, LaneGroupKind::SharedRightThrough, 76.7, 1.5, F),
        (Direction::SB, LaneGroupKind::ExclusiveLeft, 32.6, 1.5, C),
        (Direction::SB, LaneGroupKind::ExclusiveThrough, 17.0, 0.5, B),
        (Direction::SB, LaneGroupKind::SharedRightThrough, 17.1, 0.5, B),
    ];
    for (dir, kind, d, tol, los) in cases {
        let lg = group(&ix, dir, kind);
        assert_near!(
            lg.control_delay_s.unwrap(),
            d,
            tol,
            format!("{dir:?} {kind:?} d")
        );
        assert_eq!(lg.los, Some(los), "{dir:?} {kind:?} LOS");
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Step 9: aggregated delay and LOS (Exhibit 31-81)
// ═══════════════════════════════════════════════════════════════════════════

/// HCM Exhibit 31-81: approach delay (±0.5 s/veh), intersection delay
/// 45.9 s/veh (±0.5), and LOS (exact).
#[test]
fn test_step_9_aggregated_delay_and_los() {
    let ix = example_problem_1();
    use LevelOfService::*;
    let cases = [
        (Direction::EB, 32.4, C),
        (Direction::WB, 37.0, D),
        (Direction::NB, 70.0, E),
        (Direction::SB, 19.6, B),
    ];
    for (dir, d, los) in cases {
        let ar = ix
            .approach_results
            .iter()
            .find(|a| a.direction == dir)
            .unwrap();
        assert_near!(ar.control_delay_s, d, 0.5, format!("{dir:?} approach delay"));
        assert_eq!(ar.los, los, "{dir:?} approach LOS");
    }
    assert_near!(
        ix.intersection_delay_s.unwrap(),
        45.9,
        0.5,
        "intersection delay"
    );
    assert_eq!(ix.intersection_los, Some(D), "intersection LOS");
}

// ═══════════════════════════════════════════════════════════════════════════
// Step 10: back of queue and queue storage ratio (Exhibit 31-82)
// ═══════════════════════════════════════════════════════════════════════════

/// HCM Exhibit 31-82: 50th percentile back of queue for the through and
/// shared lane groups (basic arrival–departure polygon, ±0.5 veh/ln except
/// the oversaturated NB groups at ±0.8) and the left-turn lane groups
/// (left-turn arrival–departure polygon of Exhibits 31-26 through 31-31,
/// Equation 31-141, ±0.5 veh/ln). The southbound left is the milestone-2
/// acceptance case: with g_u = 0 it is served only during its protected
/// phase and by sneakers, so its full-stop count (published 4.9 veh/ln) far
/// exceeds the instantaneous peak queue; the ADP first-term procedure
/// reproduces it (milestone 1 reported 3.2 from the QAP maximum-queue
/// approximation).
#[test]
fn test_step_10_back_of_queue() {
    let ix = example_problem_1();
    let cases = [
        (Direction::EB, LaneGroupKind::ExclusiveLeft, 1.8, 0.4),
        (Direction::EB, LaneGroupKind::ExclusiveThrough, 4.8, 0.5),
        (Direction::EB, LaneGroupKind::SharedRightThrough, 3.8, 0.5),
        (Direction::WB, LaneGroupKind::ExclusiveThrough, 7.6, 0.5),
        (Direction::WB, LaneGroupKind::SharedRightThrough, 6.6, 0.5),
        (Direction::NB, LaneGroupKind::ExclusiveLeft, 1.4, 0.5),
        (Direction::NB, LaneGroupKind::ExclusiveThrough, 28.9, 0.8),
        (Direction::NB, LaneGroupKind::SharedRightThrough, 29.4, 0.8),
        (Direction::SB, LaneGroupKind::ExclusiveLeft, 4.9, 0.5),
        (Direction::SB, LaneGroupKind::ExclusiveThrough, 7.7, 0.5),
        (Direction::SB, LaneGroupKind::SharedRightThrough, 7.5, 0.5),
    ];
    for (dir, kind, q, tol) in cases {
        assert_near!(
            group(&ix, dir, kind).back_of_queue_veh.unwrap(),
            q,
            tol,
            format!("{dir:?} {kind:?} Q50")
        );
    }
    // Queue storage ratios (Exhibit 31-82), ±0.02 (through-lane storage of
    // 1,000 ft assumed; see the fixture note).
    let rq_cases = [
        (Direction::EB, LaneGroupKind::ExclusiveLeft, 0.23),
        (Direction::EB, LaneGroupKind::ExclusiveThrough, 0.12),
        (Direction::EB, LaneGroupKind::SharedRightThrough, 0.10),
        (Direction::NB, LaneGroupKind::ExclusiveLeft, 0.18),
        (Direction::NB, LaneGroupKind::ExclusiveThrough, 0.74),
        (Direction::SB, LaneGroupKind::ExclusiveThrough, 0.20),
    ];
    for (dir, kind, rq) in rq_cases {
        assert_near!(
            group(&ix, dir, kind).queue_storage_ratio.unwrap(),
            rq,
            0.02,
            format!("{dir:?} {kind:?} RQ")
        );
    }
    // The 95th percentile queue always exceeds the 50th percentile queue.
    for lg in &ix.lane_groups {
        assert!(lg.back_of_queue_95_veh.unwrap() >= lg.back_of_queue_veh.unwrap());
    }
}

/// HCM Exhibits 31-26 through 31-31 with Equation 31-141: the left-turn ADP
/// first-term back of queue counts full stops (N_f), which for a lane group
/// served in two batches per cycle exceeds the instantaneous peak queue. A
/// single protected batch (all vehicles arrive on red, one discharge)
/// reduces to the peak; a permitted lane group whose queue is held over most
/// of the cycle and released by sneakers counts nearly every arrival.
#[test]
fn test_adp_first_term_left_full_stops() {
    let c = 101.8_f64;
    let d_a = accel_decel_delay(35.0);
    let q = 194.0 / 3_600.0; // SB-left lane arrival rate, veh/s/ln
                             // Protected-permitted leading polygon with g_u = 0: a short protected
                             // green (9.8 s at s = 1,603) and a 2-vehicle sneaker release, with the
                             // queue otherwise accumulating over the whole cycle.
    let intervals = [
        QapInterval {
            duration_s: 9.8,
            discharge_veh_h: 1_603.0,
            arrival_veh_s: q,
            sneakers_veh: 0.0,
        },
        QapInterval {
            duration_s: 4.0,
            discharge_veh_h: 0.0,
            arrival_veh_s: q,
            sneakers_veh: 0.0,
        },
        QapInterval {
            duration_s: 53.6,
            discharge_veh_h: 0.0,
            arrival_veh_s: q,
            sneakers_veh: 2.0,
        },
        QapInterval {
            duration_s: 34.4,
            discharge_veh_h: 0.0,
            arrival_veh_s: q,
            sneakers_veh: 0.0,
        },
    ];
    let nf = adp_first_term_left(&intervals, c, d_a);
    // Full stops far exceed the instantaneous peak queue (~3 veh/ln); the
    // published SB-left first term is about 4.7 veh/ln (Q = 4.9 with Q2).
    let peak = qap_evaluate(&intervals, c, q).max_queue_veh;
    assert!(nf > peak + 1.0, "N_f {nf} should exceed peak {peak}");
    assert_near!(nf, 4.7, 0.6, "SB-left ADP first term");
    // A purely protected movement (single discharge) reduces to ~the peak.
    let prot = [
        QapInterval {
            duration_s: c - 20.0,
            discharge_veh_h: 0.0,
            arrival_veh_s: q,
            sneakers_veh: 0.0,
        },
        QapInterval {
            duration_s: 20.0,
            discharge_veh_h: 1_603.0,
            arrival_veh_s: q,
            sneakers_veh: 0.0,
        },
    ];
    let nf_prot = adp_first_term_left(&prot, c, d_a);
    let peak_prot = qap_evaluate(&prot, c, q).max_queue_veh;
    assert!((nf_prot - peak_prot).abs() < 0.6, "protected N_f ~ peak");
}

/// HCM Equations 31-131 / 31-132: acceleration–deceleration delay is in
/// the typical 8–14 s range noted in the Chapter 31 Section 4 text.
#[test]
fn test_accel_decel_delay_range() {
    let d35 = accel_decel_delay(35.0);
    assert!(d35 > 8.0 && d35 < 14.0, "d_a(35 mi/h) = {d35}");
    assert!(accel_decel_delay(45.0) > d35);
}

// ═══════════════════════════════════════════════════════════════════════════
// Pretimed phase duration design (HCM Ch. 31 §2, Exhibit 31-7 example)
// ═══════════════════════════════════════════════════════════════════════════

/// HCM Chapter 31, Section 2, pretimed phase duration example
/// (Exhibit 31-7): critical flow ratios 0.45 (Phase 2) and 0.35 (Phase 8),
/// cycle lost time 8 s. Published results: minimum cycle 40 s, target
/// cycle 61 s at X_t = 0.92, X_c = 0.923 at C = 60 s, g2 = 29.3 s,
/// g8 = 22.7 s.
#[test]
fn test_pretimed_phase_duration_exhibit_31_7() {
    let sum_yc = 0.45 + 0.35;
    // Eq. 31-68 with X_c = 1.0: minimum cycle length = 40 s.
    assert_near!(
        cycle_length_for_target_xc(8.0, 1.0, sum_yc).unwrap(),
        40.0,
        0.5,
        "minimum cycle"
    );
    // X_t = 0.80 is infeasible (Eq. 31-68 denominator -> None).
    assert!(cycle_length_for_target_xc(8.0, 0.80, sum_yc).is_none());
    // X_t = 0.92: C = 61 s.
    assert_near!(
        cycle_length_for_target_xc(8.0, 0.92, sum_yc).unwrap(),
        61.0,
        0.5,
        "target cycle"
    );
    // Eq. 19-30 / 31-67 at the selected 60-s cycle: X_c = 0.923.
    let xc = critical_vc_ratio_eq(60.0, 8.0, sum_yc);
    assert_near!(xc, 0.923, 0.001, "X_c");
    // Eq. 31-69 green allocation: g2 = 29.3 s, g8 = 22.7 s (published
    // values rounded to one decimal; the exact results are 29.25 / 22.75).
    assert_near!(pretimed_effective_green(0.45, 60.0, xc), 29.3, 0.06, "g2");
    assert_near!(pretimed_effective_green(0.35, 60.0, xc), 22.7, 0.06, "g8");
    // Timing check: g2 + g8 + L = C.
    assert_near!(
        pretimed_effective_green(0.45, 60.0, xc) + pretimed_effective_green(0.35, 60.0, xc) + 8.0,
        60.0,
        0.05,
        "cycle closure"
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// Building blocks
// ═══════════════════════════════════════════════════════════════════════════

/// HCM Equations 19-1 and 19-3: effective green, phase lost time, and
/// available effective green (Equation 19-25).
#[test]
fn test_phase_timing_effective_green() {
    let p = PhaseTiming {
        phase_no: 2,
        duration_s: 34.0,
        yellow_s: 4.0,
        red_clearance_s: 0.0,
        max_green_s: Some(30.0),
        passage_time_s: Some(2.0),
        walk_s: None,
        ped_clear_s: None,
        min_green_s: None,
        detector_length_ft: None,
        recall_max: false,
    };
    // g = D_p - l1 - l2 = 34 - 2 - (4 - 2) = 30 s.
    assert_near!(p.effective_green_s(), 30.0, 1e-9, "effective green");
    // l_t = l1 + Y + Rc - e = 4 s.
    assert_near!(p.lost_time_s(), 4.0, 1e-9, "lost time");
    let left = PhaseTiming {
        phase_no: 3,
        duration_s: 10.2,
        yellow_s: 4.0,
        red_clearance_s: 0.0,
        max_green_s: Some(25.0),
        passage_time_s: Some(2.0),
        walk_s: None,
        ped_clear_s: None,
        min_green_s: None,
        detector_length_ft: None,
        recall_max: false,
    };
    // g_a = G_max + Y + Rc - l1 - l2 = 25 + 4 - 2 - 2 = 25 s.
    assert_near!(left.available_effective_green_s(), 25.0, 1e-9, "g_a");
    assert_near!(left.effective_green_s(), 6.2, 1e-9, "left effective green");
}

/// HCM Exhibit 31-12, Perm–Perm row, checked against the Example Problem 1
/// eastbound left: Dp = 34 s both streets, CP = 4 s, Gq = 20.55 s
/// -> G_U = 9.45, g_u = 11.45, g_p = 30.
#[test]
fn test_permitted_green_times_perm_perm() {
    let pg = permitted_green_times(
        LeftTurnSequence::PermPerm,
        0.0,
        0.0,
        34.0,
        34.0,
        0.0,
        4.0,
        4.0,
        20.55,
    );
    assert_near!(pg.g_p, 30.0, 1e-9, "g_p");
    assert_near!(pg.g_u, 11.45, 0.01, "g_u");
    assert_near!(pg.l1p, 2.0, 1e-9, "l1p");
}

/// HCM Exhibit 31-12, Lead–Lead row, checked against the Example Problem 1
/// northbound left: Dp3 = 10.2, Dp4 = 57.6, Dp7 = 13.8, Dp8 = 54.0,
/// CP = 4 s, Gq = 23.14 -> G_U = min(50.0, 30.46) = 30.46, l1* clamps to
/// 2.0, g_u = 32.46, g_p = 50.0.
#[test]
fn test_permitted_green_times_lead_lead() {
    let pg = permitted_green_times(
        LeftTurnSequence::LeadLead,
        10.2, // own left (phase 3)
        13.8, // opposing left (phase 7)
        54.0, // own through (phase 8)
        57.6, // opposing through (phase 4)
        4.0,
        4.0,
        4.0,
        23.14,
    );
    assert_near!(pg.g_p, 50.0, 1e-6, "g_p NB");
    assert_near!(pg.g_u, 32.46, 0.01, "g_u NB");
    assert_near!(pg.l1p, 2.0, 1e-9, "l1p NB");
}

/// Queue accumulation polygon (Eqs. 19-34..19-36) reproduces the closed
/// form of Equation 19-19 for a protected movement with uniform arrivals.
#[test]
fn test_qap_matches_closed_form_uniform_delay() {
    let (c, g, v, s) = (100.0_f64, 40.0_f64, 400.0_f64, 1_800.0_f64);
    let q = v / 3_600.0;
    let intervals = [
        QapInterval {
            duration_s: c - g,
            discharge_veh_h: 0.0,
            arrival_veh_s: q,
            sneakers_veh: 0.0,
        },
        QapInterval {
            duration_s: g,
            discharge_veh_h: s,
            arrival_veh_s: q,
            sneakers_veh: 0.0,
        },
    ];
    let res = qap_evaluate(&intervals, c, q);
    let x = v / (s * g / c);
    let d1_closed = crate::hcm::common::delay::uniform_delay(c, g, x, 1.0);
    assert_near!(res.uniform_delay_s, d1_closed, 0.01, "QAP vs Eq. 19-19");
    // Max queue is at least the red-period accumulation.
    assert!(res.max_queue_veh >= q * (c - g) - 1e-9);
}

/// The QAP steady-state iteration converges for near-capacity demand.
#[test]
fn test_qap_steady_state_iteration() {
    let (c, g, s) = (100.0_f64, 30.0_f64, 1_800.0_f64);
    let v = 0.99 * s * g / c;
    let q = v / 3_600.0;
    let intervals = [
        QapInterval {
            duration_s: c - g,
            discharge_veh_h: 0.0,
            arrival_veh_s: q,
            sneakers_veh: 0.0,
        },
        QapInterval {
            duration_s: g,
            discharge_veh_h: s,
            arrival_veh_s: q,
            sneakers_veh: 0.0,
        },
    ];
    let res = qap_evaluate(&intervals, c, q);
    let x = v / (s * g / c);
    let d1_closed = crate::hcm::common::delay::uniform_delay(c, g, x, 1.0);
    assert_near!(res.uniform_delay_s, d1_closed, 0.5, "QAP near capacity");
}

/// HCM Equation 31-155: average vehicle spacing (25 ft passenger car,
/// 45 ft heavy vehicle).
#[test]
fn test_average_vehicle_spacing() {
    assert_near!(average_vehicle_spacing(0.0), 25.0, 1e-9, "0% HV");
    assert_near!(average_vehicle_spacing(5.0), 26.0, 1e-9, "5% HV");
    assert_near!(average_vehicle_spacing(2.0), 25.4, 1e-9, "2% HV");
    assert_near!(average_vehicle_spacing(100.0), 45.0, 1e-9, "100% HV");
}

/// HCM Equation 31-142 spot check against Example Problem 1 NB through:
/// Q2 = 827/3,600 * 46.0 = 10.6 veh/ln.
#[test]
fn test_second_term_back_of_queue() {
    assert_near!(second_term_back_of_queue(827.0, 1, 46.0), 10.57, 0.01, "Q2");
}

/// Serde round trip of the facility with computed results.
#[test]
fn test_serde_roundtrip() {
    let ix = example_problem_1();
    let json = serde_json::to_string(&ix).unwrap();
    let back: SignalizedIntersection = serde_json::from_str(&json).unwrap();
    assert_eq!(back.lane_groups.len(), ix.lane_groups.len());
    assert_near!(
        back.intersection_delay_s.unwrap(),
        ix.intersection_delay_s.unwrap(),
        1e-9,
        "roundtrip delay"
    );
    assert_eq!(back.intersection_los, ix.intersection_los);
}

// ═══════════════════════════════════════════════════════════════════════════
// RTOR estimation (HCM Ch. 19 Step 2 / Ch. 31 §8) and actuated timing wiring
// ═══════════════════════════════════════════════════════════════════════════

/// HCM Chapter 31, Section 8: the exclusive right-turn-lane RTOR estimate
/// equals the complementary cross-street protected left-turn demand, capped
/// at the right-turn demand. Example Problem 1 modified so the eastbound
/// approach has an exclusive right-turn lane shadowed by the northbound
/// protected-permitted left.
#[test]
fn test_rtor_estimate_exclusive_right_lane() {
    let json = include_str!("../../../tests/ExampleCases/hcm/Signalized/case1.json");
    let mut ix: SignalizedIntersection = serde_json::from_str(json).unwrap();
    // Give the eastbound approach an exclusive right-turn lane.
    {
        let eb = ix
            .approaches
            .iter_mut()
            .find(|a| a.direction == Direction::EB)
            .unwrap();
        eb.exclusive_right_lanes = 1;
        eb.shared_right_through_lane = false;
        eb.volume_rtor = 0.0;
    }
    // Northbound left is protected-permitted (has a left phase), demand 133;
    // eastbound right demand is 106, so the estimate is capped at 106.
    let est = ix.estimate_rtor_volume(Direction::EB);
    assert_near!(est, 106.0, 1e-9, "EB RTOR estimate (capped at v_r)");

    // Westbound is shadowed by the southbound protected-permitted left
    // (demand 194); westbound right demand 24 caps the estimate at 24.
    {
        let wb = ix
            .approaches
            .iter_mut()
            .find(|a| a.direction == Direction::WB)
            .unwrap();
        wb.exclusive_right_lanes = 1;
        wb.shared_right_through_lane = false;
        wb.volume_rtor = 0.0;
    }
    assert_near!(ix.estimate_rtor_volume(Direction::WB), 24.0, 1e-9, "WB RTOR");

    // A shared right-turn lane yields no estimate (HCM offers none).
    assert_eq!(ix.estimate_rtor_volume(Direction::NB), 0.0);

    // apply_rtor_estimates populates volume_rtor only where unset.
    ix.apply_rtor_estimates();
    let eb = ix
        .approaches
        .iter()
        .find(|a| a.direction == Direction::EB)
        .unwrap();
    assert_near!(eb.volume_rtor, 106.0, 1e-9, "EB volume_rtor applied");
}

/// The complementary-left shadow map (approach 90° counterclockwise) is a
/// consistent rotation with no fixed points.
#[test]
fn test_rtor_no_left_phase_no_estimate() {
    let json = include_str!("../../../tests/ExampleCases/hcm/Signalized/case1.json");
    let mut ix: SignalizedIntersection = serde_json::from_str(json).unwrap();
    // Eastbound exclusive right, but make the northbound left permitted-only
    // (no protected phase): no shadow, no estimate.
    {
        let eb = ix
            .approaches
            .iter_mut()
            .find(|a| a.direction == Direction::EB)
            .unwrap();
        eb.exclusive_right_lanes = 1;
        eb.shared_right_through_lane = false;
    }
    {
        let nb = ix
            .approaches
            .iter_mut()
            .find(|a| a.direction == Direction::NB)
            .unwrap();
        nb.left_turn_mode = LeftTurnMode::Permitted;
    }
    assert_eq!(ix.estimate_rtor_volume(Direction::EB), 0.0);
}

/// The actuated phase-duration estimator is reachable from the analyzed
/// facility and reproduces the Example Problem 1 minor-street through phases
/// (HCM Exhibit 31-79) within the documented tolerance.
#[test]
fn test_estimate_actuated_timings_from_facility() {
    let ix = example_problem_1();
    let res = ix.estimate_actuated_timings(true);
    assert_eq!(res.len(), 6, "six phases");
    let dur = |no: u8| res.iter().find(|r| r.phase_no == no).unwrap().duration_s;
    // Barrier balance is exact; minor-street through phases are within ~4 s
    // of the published 54.0 / 57.6 s.
    assert_near!(dur(3) + dur(4), dur(7) + dur(8), 1e-6, "minor barrier balance");
    assert_near!(dur(8), 54.0, 4.0, "Ph8 NB through");
    assert_near!(dur(4), 57.6, 5.0, "Ph4 SB through");
}
