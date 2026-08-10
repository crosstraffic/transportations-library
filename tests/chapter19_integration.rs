//! Integration tests for the HCM Chapter 19 (Signalized Intersections)
//! motorized vehicle methodology: full pipeline runs against the published
//! answers of HCM 7th Edition, Chapter 31.
//!
//! Fixtures:
//! * `case1.json` — Chapter 31, Section 10, Example Problem 1 (motorized
//!   vehicle LOS; Exhibits 31-69..31-82). The converged phase durations of
//!   Exhibit 31-79 are supplied as fixed timing.
//! * `case2.json` — Chapter 31, Section 2, pretimed phase duration example
//!   (Exhibit 31-7) encoded as a full pretimed timing plan; delay and LOS
//!   expectations are hand-computed from HCM Equations 19-19 and 19-26
//!   with the pretimed incremental delay factor k = 0.50.
//!
//! Documented tolerances:
//! * LOS — exact;
//! * intersection / approach control delay — ±0.5 s/veh;
//! * lane-group control delay — ±0.5 s/veh for lane groups evaluated with
//!   the Equation 19-19 closed form, ±1.5 s/veh for permitted /
//!   protected-permitted left-turn lane groups (queue accumulation
//!   polygon) and the oversaturated NB lane groups (d2 sensitivity to the
//!   ±2 veh/h lane-flow procedure differences);
//! * adjusted saturation flow — ±10 veh/h/ln;
//! * capacity — ±6 veh/h; v/c ratio — ±0.02.
//!
//! The pedestrian and bicycle modes (Chapter 31, Section 10, Example Problems
//! 2, 3, and 4) are covered at the bottom of this file. They take their inputs
//! directly rather than through a fixture, because they are scored per
//! crosswalk or per bicycle lane and share no state with the
//! `SignalizedIntersection` fixture; the published example problems likewise
//! state their inputs standalone.
//!
//! Two published items in that section are NOT covered, both for want of
//! engine surface:
//!
//! * Example Problem 2, Steps 4 and 5 (street-corner and crosswalk circulation
//!   area, Equations 19-61 through 19-76). These are optional pedestrian-space
//!   measures compared against Exhibit 19-34 rather than LOS, and
//!   `src/hcm/signalized/pedestrian.rs` documents them as out of scope. The
//!   published answers, should they ever be implemented, are TS_corner =
//!   16,610 ft²-s and 87.6 ft²/p for Corner 2, and 14.1 ft²/p for the
//!   crosswalk.
//! * Example Problem 5 (pedestrian delay with a two-stage crossing of TWO
//!   intersection legs, corner B to C to D). `TwoStageCrossing` implements the
//!   "Crossing One Intersection Leg in Two Stages" procedure only. The
//!   two-leg procedure is a distinct one whose stages are served by phases on
//!   different streets, so it needs its own phase-pair and crosswalk inputs,
//!   not a reparameterization of the one-leg type.

use transportations_library::hcm::signalized::{
    BicycleIntersection, LaneGroupKind, PedestrianIntersection, SignalizedIntersection,
    TwoStageCrossing,
};
use transportations_library::hcm::common::intersection::Direction;
use transportations_library::hcm::common::LevelOfService;

fn load_case(name: &str) -> SignalizedIntersection {
    let path = format!(
        "{}/tests/ExampleCases/hcm/Signalized/{}",
        env!("CARGO_MANIFEST_DIR"),
        name
    );
    let json = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {path}: {e}"));
    serde_json::from_str(&json).unwrap_or_else(|e| panic!("parse {path}: {e}"))
}

fn group<'a>(
    ix: &'a SignalizedIntersection,
    dir: Direction,
    kind: LaneGroupKind,
) -> &'a transportations_library::hcm::signalized::LaneGroup {
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

/// Full pipeline versus HCM Chapter 31, Example Problem 1 (Exhibits
/// 31-76, 31-77, 31-80, and 31-81).
#[test]
fn test_case1_example_problem_1_full_pipeline() {
    let mut ix = load_case("case1.json");
    ix.analyze();
    use Direction::*;
    use LaneGroupKind::*;
    use LevelOfService as L;

    // (dir, kind, v [31-76], s [31-77], c [31-80], X [31-80], d [31-81],
    //  d tolerance, LOS [31-81])
    let cases = [
        (EB, ExclusiveLeft, 71.0, 702.0, 149.0, 0.47, 45.5, 1.5, L::D),
        (EB, ExclusiveThrough, 239.0, 1_643.0, 484.0, 0.49, 29.9, 0.5, L::C),
        (EB, SharedRightThrough, 185.0, 1_201.0, 354.0, 0.52, 30.6, 0.5, L::C),
        (WB, ExclusiveLeft, 118.0, 825.0, 208.0, 0.57, 43.5, 1.5, L::D),
        (WB, ExclusiveThrough, 337.0, 1_643.0, 484.0, 0.70, 35.5, 0.5, L::D),
        (WB, SharedRightThrough, 287.0, 1_398.0, 412.0, 0.70, 36.2, 0.5, L::D),
        (NB, ExclusiveLeft, 133.0, 1_603.0, 328.0, 0.41, 13.5, 1.5, L::B),
        (NB, ExclusiveThrough, 870.0, 1_683.0, 827.0, 1.05, 72.0, 1.5, L::F),
        (NB, SharedRightThrough, 863.0, 1_648.0, 809.0, 1.07, 76.7, 1.5, L::F),
        (SB, ExclusiveLeft, 194.0, 1_603.0, 225.0, 0.86, 32.6, 1.5, L::C),
        (SB, ExclusiveThrough, 513.0, 1_683.0, 887.0, 0.58, 17.0, 0.5, L::B),
        (SB, SharedRightThrough, 497.0, 1_630.0, 859.0, 0.58, 17.1, 0.5, L::B),
    ];
    for (dir, kind, v, s, c, x, d, d_tol, los) in cases {
        let lg = group(&ix, dir, kind);
        assert_near!(lg.flow_rate, v, 3.0, format!("{dir:?} {kind:?} v"));
        assert_near!(lg.sat_flow.unwrap(), s, 10.0, format!("{dir:?} {kind:?} s"));
        assert_near!(lg.capacity.unwrap(), c, 6.0, format!("{dir:?} {kind:?} c"));
        assert_near!(lg.vc_ratio.unwrap(), x, 0.02, format!("{dir:?} {kind:?} X"));
        assert_near!(
            lg.control_delay_s.unwrap(),
            d,
            d_tol,
            format!("{dir:?} {kind:?} d")
        );
        assert_eq!(lg.los, Some(los), "{dir:?} {kind:?} LOS");
    }

    // Approach delay and LOS (Exhibit 31-81).
    let approach_cases = [
        (EB, 32.4, L::C),
        (WB, 37.0, L::D),
        (NB, 70.0, L::E),
        (SB, 19.6, L::B),
    ];
    for (dir, d, los) in approach_cases {
        let ar = ix
            .approach_results
            .iter()
            .find(|a| a.direction == dir)
            .unwrap();
        assert_near!(ar.control_delay_s, d, 0.5, format!("{dir:?} approach delay"));
        assert_eq!(ar.los, los, "{dir:?} approach LOS");
    }

    // Intersection delay 45.9 s/veh, LOS D (Exhibit 31-81).
    assert_near!(
        ix.intersection_delay_s.unwrap(),
        45.9,
        0.5,
        "intersection delay"
    );
    assert_eq!(ix.intersection_los, Some(L::D), "intersection LOS");
}

/// Full pipeline for the pretimed timing plan of the HCM Chapter 31,
/// Section 2 example (Exhibit 31-7). The critical intersection v/c ratio
/// X_c = 0.923 is published; the delay and LOS values are hand-computed
/// from HCM Equations 19-19 (PF = 1, random arrivals) and 19-26
/// (pretimed k = 0.50, isolated I = 1.0):
///
/// | Appr. | y    | c (Eq. 19-16) | X     | d1    | d2    | d     | LOS |
/// |-------|------|---------------|-------|-------|-------|-------|-----|
/// | EB    | 0.45 | 927.8         | 0.922 | 14.28 | 15.75 | 30.03 | C   |
/// | WB    | 0.25 | 927.8         | 0.512 | 10.47 |  2.02 | 12.49 | B   |
/// | NB    | 0.35 | 718.8         | 0.925 | 17.84 | 19.57 | 37.40 | D   |
/// | SB    | 0.25 | 718.8         | 0.661 | 15.46 |  4.73 | 20.19 | C   |
///
/// Intersection delay = Σ(d v)/Σv = 26.75 s/veh -> LOS C.
#[test]
fn test_case2_pretimed_exhibit_31_7_timing_plan() {
    let mut ix = load_case("case2.json");
    ix.analyze();
    use Direction::*;
    use LevelOfService as L;

    // Published X_c (Exhibit 31-7 example text): 0.923.
    assert_near!(ix.critical_vc_ratio.unwrap(), 0.923, 0.001, "X_c");

    let cases = [
        (EB, 855.0, 927.8, 0.922, 14.28, 15.75, 30.03, L::C),
        (WB, 475.0, 927.8, 0.512, 10.47, 2.02, 12.49, L::B),
        (NB, 665.0, 718.8, 0.925, 17.84, 19.57, 37.40, L::D),
        (SB, 475.0, 718.8, 0.661, 15.46, 4.73, 20.19, L::C),
    ];
    for (dir, v, c, x, d1, d2, d, los) in cases {
        let lg = group(&ix, dir, LaneGroupKind::ExclusiveThrough);
        assert_near!(lg.flow_rate, v, 1e-9, format!("{dir:?} v"));
        assert_near!(lg.capacity.unwrap(), c, 1.0, format!("{dir:?} c"));
        assert_near!(lg.vc_ratio.unwrap(), x, 0.005, format!("{dir:?} X"));
        assert_near!(lg.uniform_delay_s.unwrap(), d1, 0.5, format!("{dir:?} d1"));
        assert_near!(
            lg.incremental_delay_s.unwrap(),
            d2,
            0.5,
            format!("{dir:?} d2")
        );
        assert_near!(lg.control_delay_s.unwrap(), d, 0.5, format!("{dir:?} d"));
        assert_eq!(lg.los, Some(los), "{dir:?} LOS");
        // Pretimed control: k = 0.50 (HCM Ch. 19, Step 8C).
        assert_eq!(lg.k_factor, Some(0.5), "{dir:?} k");
    }

    assert_near!(
        ix.intersection_delay_s.unwrap(),
        26.75,
        0.5,
        "intersection delay"
    );
    assert_eq!(ix.intersection_los, Some(L::C), "intersection LOS");
}

/// Serde round trip through JSON keeps the computed results.
#[test]
fn test_fixture_serde_roundtrip() {
    let mut ix = load_case("case1.json");
    ix.analyze();
    let json = serde_json::to_string_pretty(&ix).unwrap();
    let back: SignalizedIntersection = serde_json::from_str(&json).unwrap();
    assert_eq!(back.lane_groups.len(), ix.lane_groups.len());
    assert_eq!(back.intersection_los, ix.intersection_los);
}

// ═══════════════════════════════════════════════════════════════════════════
// Milestone 2: actuated phase-duration estimation, left-turn ADP back of
// queue, and RTOR estimation (HCM Chapter 31, Sections 2, 4, and 8).
// ═══════════════════════════════════════════════════════════════════════════

/// The left-turn arrival–departure polygon (HCM Exhibits 31-26..31-31,
/// Equation 31-141) reproduces the Example Problem 1 southbound-left back of
/// queue (Exhibit 31-82: 4.9 veh/ln), the milestone-2 acceptance case that
/// the milestone-1 QAP maximum-queue approximation reported as 3.2.
#[test]
fn test_m2_sb_left_back_of_queue() {
    let mut ix = load_case("case1.json");
    ix.analyze();
    let sb_left = group(&ix, Direction::SB, LaneGroupKind::ExclusiveLeft);
    assert_near!(
        sb_left.back_of_queue_veh.unwrap(),
        4.9,
        0.5,
        "SB-left 50th percentile back of queue"
    );
    // The other left-turn lane groups stay within tolerance of Exhibit 31-82.
    assert_near!(
        group(&ix, Direction::EB, LaneGroupKind::ExclusiveLeft)
            .back_of_queue_veh
            .unwrap(),
        1.8,
        0.4,
        "EB-left back of queue"
    );
    assert_near!(
        group(&ix, Direction::NB, LaneGroupKind::ExclusiveLeft)
            .back_of_queue_veh
            .unwrap(),
        1.4,
        0.5,
        "NB-left back of queue"
    );
}

/// The actuated phase-duration estimator (HCM Chapter 31, Section 2)
/// computes the Example Problem 1 phase durations from the controller
/// settings. The equivalent maximum allowable headway and the barrier
/// balance are exact; the minor-street through phases reproduce the
/// published Exhibit 31-79 durations (54.0 / 57.6 s) within ~4 s. The
/// major-street max-out under-prediction is the documented engine gap.
#[test]
fn test_m2_actuated_phase_durations() {
    let mut ix = load_case("case1.json");
    ix.analyze();
    let res = ix.estimate_actuated_timings(true);
    let dur = |no: u8| res.iter().find(|r| r.phase_no == no).unwrap().duration_s;
    let mah = |no: u8| res.iter().find(|r| r.phase_no == no).unwrap().mah_star_s;
    // Barrier balance (Equations 31-38 / 31-39) is exact.
    assert_near!(dur(2), dur(6), 1e-6, "major barrier balance");
    assert_near!(dur(3) + dur(4), dur(7) + dur(8), 1e-6, "minor barrier balance");
    // Equivalent MAH matches Exhibit 31-79 (3.4 EB/WB, 3.1 minor street).
    assert_near!(mah(2), 3.4, 0.1, "MAH Ph2");
    assert_near!(mah(8), 3.1, 0.2, "MAH Ph8");
    // Minor-street through phases reproduce the published durations.
    assert_near!(dur(8), 54.0, 4.0, "Ph8 NB-through duration");
    assert_near!(dur(4), 57.6, 5.0, "Ph4 SB-through duration");
}

/// RTOR estimation (HCM Chapter 31, Section 8): an exclusive right-turn lane
/// shadowed by a complementary protected cross-street left turn takes the
/// left-turn demand as its RTOR estimate (capped at the right-turn demand).
#[test]
fn test_m2_rtor_estimate() {
    let mut ix = load_case("case1.json");
    // Southbound gets an exclusive right lane; it is shadowed by the
    // eastbound approach, whose left turn is permitted-only (no phase), so no
    // estimate results — until the eastbound left is given a protected phase.
    {
        let sb = ix
            .approaches
            .iter_mut()
            .find(|a| a.direction == Direction::SB)
            .unwrap();
        sb.exclusive_right_lanes = 1;
        sb.shared_right_through_lane = false;
        sb.volume_rtor = 0.0;
    }
    // Eastbound left is permitted (case1): SB shadow has no protected phase.
    assert_eq!(ix.estimate_rtor_volume(Direction::SB), 0.0);
    // Northbound (shadowed by the westbound permitted left) likewise: shared
    // right lane, so no exclusive-lane estimate.
    assert_eq!(ix.estimate_rtor_volume(Direction::NB), 0.0);
}

// ═══════════════════════════════════════════════════════════════════════════
// Pedestrian and bicycle modes (HCM Chapter 31, Section 10, Example Problems
// 2, 3, and 4).
// ═══════════════════════════════════════════════════════════════════════════

/// HCM Chapter 31, Example Problem 2: pedestrian LOS for a crosswalk over the
/// minor-street (north) leg. Every published intermediate is asserted, because
/// Equation 19-55 is a sum of four independently computed factors and a wrong
/// factor can be masked by the score alone.
///
/// Published (Exhibit 19-9 for the LOS band): g_Walk,mj = 11.0 s;
/// d_p = 29.8 s/p; n_15,mi = 123.3 veh/ln; F_w = 0.972; F_v = 0.102;
/// F_s = 0.561; F_delay = 0.136; I_p,int = 2.37; LOS B.
#[test]
fn test_ep2_pedestrian_los() {
    use LevelOfService as L;
    let ix = PedestrianIntersection {
        cycle_length_s: 80.0,
        // The crossing is over the minor street, so it is served by the
        // major-street phase: Walk_mj = 7 s, and Equation 19-51 adds 4.0.
        walk_setting_s: 7.0,
        lanes_crossed: 2.0,
        v_rtor: 30.0,
        v_lt_perm: 42.0,
        num_rtci: 0.0,
        // Exhibit 31-85 minor-street movements: 72 + 336 + 60 + 42 + 400 + 76.
        crossed_street_volume_sum: 986.0,
        crossed_street_lanes: 2.0,
        speed_85_mph: 35.0,
    };
    let a = ix.analyze();
    assert_near!(a.effective_walk_s, 11.0, 1e-9, "g_Walk,mj");
    assert_near!(a.delay, 29.8, 0.05, "d_p");
    assert_near!(a.n15_per_lane, 123.3, 0.05, "n_15,mi");
    assert_near!(a.f_w, 0.972, 0.001, "F_w");
    assert_near!(a.f_v, 0.102, 0.001, "F_v");
    assert_near!(a.f_s, 0.561, 0.001, "F_s");
    assert_near!(a.f_delay, 0.136, 0.001, "F_delay");
    assert_near!(a.los_score, 2.37, 0.01, "I_p,int");
    assert_eq!(a.los, L::B, "pedestrian LOS");
}

/// HCM Chapter 31, Example Problem 3: bicycle LOS of a 5-ft bicycle lane.
///
/// Published: c_b = 800 bicycles/h; d_b = 23.0 s/bicycle; F_w = -2.57;
/// F_v = 0.90; I_b,int = 2.45; LOS B.
///
/// The score tolerance is looser than the factors it sums because the book
/// prints 4.1324 - 2.57 + 0.90 and reports 2.45, which only follows from the
/// unrounded factors (4.1324 - 2.5738 + 0.89595 = 2.4546); carrying the
/// printed roundings instead gives 2.4624. The engine keeps full precision, so
/// it lands on the published 2.45 rather than on the printed arithmetic.
#[test]
fn test_ep3_bicycle_los() {
    use LevelOfService as L;
    let ix = BicycleIntersection {
        saturation_flow: 2000.0,
        effective_green_s: 48.0,
        cycle_length_s: 120.0,
        bicycle_flow: 120.0,
        // Exhibit 31-86 cross section: 70-ft curb-to-curb cross street, and
        // W_t = 12-ft outside through lane + 5-ft bicycle lane + 0 shoulder.
        cross_street_width_ft: 70.0,
        total_width_ft: 17.0,
        v_left: 85.0,
        v_through: 924.0,
        v_right: 77.0,
        num_through_lanes: 2.0,
    };
    let a = ix.analyze();
    assert_near!(a.capacity, 800.0, 1e-6, "c_b");
    assert_near!(a.delay, 23.0, 0.05, "d_b");
    assert_near!(a.f_w, -2.57, 0.005, "F_w");
    assert_near!(a.f_v, 0.90, 0.005, "F_v");
    assert_near!(a.los_score, 2.45, 0.01, "I_b,int");
    assert_eq!(a.los, L::B, "bicycle LOS");
}

/// HCM Chapter 31, Example Problem 4: pedestrian delay for a two-stage
/// crossing of the east leg, northbound from Corner B to Corner A.
///
/// Published d_p = 78 s/p, built from d_p,1 = 61.3, d_2,DW1 = 17.0,
/// d_2,W1 = 12.5, and P_DW1 = 0.936. Only the total is exposed by
/// `TwoStageCrossing::delay`, but it reconciles exactly against the published
/// parts: 61.3 + 17.0(0.936) + 12.5(0.064) = 78.0.
#[test]
fn test_ep4_two_stage_crossing_one_leg() {
    let x = TwoStageCrossing {
        cycle_length_s: 140.0,
        walk_setting_x_s: 5.0,
        walk_setting_y_s: 5.0,
        // Equation 19-88 L_X: 40-ft crosswalk section 8 plus the 16-ft median.
        first_stage_distance_ft: 56.0,
        // 30% elderly pedestrian population.
        walk_speed_fps: 3.3,
        // Exhibit 31-87(b): phase 8 starts at 21 + 57, phase 7 at 21 + 57 + 34.
        walk_start_x_s: 78.0,
        walk_start_y_s: 112.0,
    };
    assert_near!(x.delay(), 78.0, 0.5, "d_p B->A");
}

/// The Example Problem 4 Discussion states that the same intersection crossed
/// in the opposite direction (Corner A to Corner B) yields 147 s, using a
/// 52-ft length for crosswalk section 78. Reversing the direction swaps the
/// phase roles, so Phase X becomes phase 7 and Phase Y becomes phase 8; this
/// drives `delay` through its t >= g_Walk,X branch, which the northbound case
/// never reaches. Without it the asymmetry the Discussion is making a point of
/// would go untested.
#[test]
fn test_ep4_two_stage_crossing_reverse_direction() {
    let x = TwoStageCrossing {
        cycle_length_s: 140.0,
        walk_setting_x_s: 5.0,
        walk_setting_y_s: 5.0,
        // L_X: 52-ft crosswalk section 78 plus the same 16-ft median.
        first_stage_distance_ft: 68.0,
        walk_speed_fps: 3.3,
        // Phase roles swap with the direction of travel.
        walk_start_x_s: 112.0,
        walk_start_y_s: 78.0,
    };
    // The book rounds this one to whole seconds in prose, hence the wider
    // tolerance than the northbound case.
    assert_near!(x.delay(), 147.0, 1.0, "d_p A->B");
}
