//! Integration tests for the HCM Chapter 23 (Ramp Terminals and
//! Alternative Intersections, Part B) interchange methodology: full
//! pipeline runs against the published answers of HCM 7th Edition,
//! Chapter 34 (Interchange Ramp Terminals: Supplemental).
//!
//! Fixtures:
//! * `case1.json` — Chapter 34, Example Problem 1: conventional diamond
//!   interchange (published O-D results in Exhibit 34-16).
//! * `case2.json` — Chapter 34, Example Problem 5: diverging diamond
//!   interchange with signal control (published O-D results in
//!   Exhibit 34-65).
//! * `case3.json` — Chapter 34, Example Problem 3: diamond interchange
//!   with queue spillback (published O-D results in Exhibit 34-43).
//! * `case4.json` — Chapter 34, Example Problem 4: diamond interchange
//!   with demand starvation (published O-D results in Exhibit 34-57).
//! * `case5.json` — Chapter 34, Example Problem 6: the Example Problem 5
//!   DDI with the four off-ramp turns YIELD-controlled instead of
//!   signalized (published results in Exhibits 34-67 through 34-71).
//! * `case6.json` — Chapter 34, Example Problem 2: Parclo A-2Q
//!   interchange (published O-D results in Exhibit 34-29).
//!
//! * `case7.json` — Chapter 34, Example Problem 7: single-point urban
//!   interchange (published O-D results in Exhibit 34-82).
//!
//! The SPUI is a single signalized point. Every one of its O-Ds resolves onto
//! exactly one lane group, so `od_path` returns a one-element path and the
//! O-D delay is the movement delay unchanged, which is what Exhibit 34-82
//! shows (its ETT column equals the Exhibit 34-80 / 34-81 movement delays,
//! there being one intersection and no extra distance traveled). The two
//! arterial left turns operate with a protected phase and a permitted phase
//! from the same exclusive lane; that is one lane group with two phase
//! components, not two lane groups, because Exhibit 34-78 carries a single
//! saturation flow, effective green, capacity and v/c for each of them and
//! Steps 6, 7 and 9 read only those. See `ProtectedPermittedLeft`.
//!
//! Example Problem 8 (diamond with an adjacent closely spaced intersection,
//! Exhibits 34-83 through 34-98) has no fixture. Its Step 4 intermediates are
//! covered by `test_ep8_adjacent_intersection_queues_and_lost_time`, which
//! drives the free functions against Exhibits 34-89 and 34-90. The full
//! example is not assembled because it is two facilities, not one: the
//! adjacent signalized intersection is a Chapter 19 analysis in its own right
//! (Exhibits 34-88, 34-93, 34-96, and the Exhibit 34-98 LOS table), and
//! `Interchange` has no slot for a third intersection —
//! `adjacent_intersection_lost_time`
//! (src/hcm/ramp_terminals/ramp_terminals.rs:2171) exposes only the coupling
//! term, and the Chapter 23 text leaves the rest of the interaction to the
//! analyst. Published targets: Exhibit 34-97 interchange O-D ETTs (A 47.2,
//! B 41.1, C 51.9, D 74.5, E 100.0, F 37.2, G 34.1, H 88.9, I 56.2, J 38.2
//! s/veh; interchange ETT 53.8 s/veh, LOS C) and the Exhibit 34-98 adjacent
//! intersection delays (EB 48.5 / 67.6, WB 41.4 / 60.0, NB 85.4 / 109.1,
//! SB 68.4 / 138.6 / 226.6 s/veh).
//!
//! Example Problem 10 (operational analysis for interchange type selection,
//! Exhibits 34-103 through 34-114) has no fixture and no test. It exercises
//! the Chapter 34 Section 3 type-selection methodology — Equations 34-1
//! through 34-14 over the Exhibit 34-151 default saturation flows and the
//! Exhibit 34-152 O-D-to-NEMA mapping, closed by the Exhibit 34-159 delay
//! regression — and none of that is implemented anywhere in `src/hcm`
//! (grep for `34-159`, `critical flow ratio`, and `type_selection` returns
//! nothing outside the Chapter 19 and 31 critical-flow-ratio machinery,
//! which is a different quantity). It is a planning-level screening model
//! that shares no code with the Part B operational pipeline. Published
//! targets: the Exhibit 34-114 interchange delays for the eight types with
//! signalized right turns (SPUI 62.9, TUDI 217.7, CUDI 35.9, CDI 26.6,
//! Parclo A-4Q 26.2, Parclo A-2Q 47.4, Parclo B-4Q 11.9, Parclo B-2Q 30.7
//! s/veh) and with free or YIELD-controlled right turns (22.0, 33.3, 27.4,
//! 21.7, 21.6, 29.0, 11.3, 29.0 s/veh), with Parclo B-4Q selected.
//!
//! Example Problem 11 (alternative analysis tool, Exhibits 34-115 through
//! 34-122) is skipped. It is a simulation study of self-aggravating queue
//! interactions — ramp metering, left-bay spillover blocking through
//! traffic, and a TWSC intersection blocked by a stationary queue — and it
//! publishes no HCM-computed values, only demand-versus-discharge curves
//! read off simulation output. The example exists precisely to show what the
//! Chapter 23 methodology does not capture, so there is nothing here for the
//! engine to reproduce.
//!
//! Documented tolerances:
//! * Example Problem 1 — O-D control delay and ETT ±1.0 s/veh, asserted at
//!   equation-based values with the published Exhibit 34-16 values inline for
//!   the six O-Ds that use an external through movement (see
//!   `test_case1_diamond_od_results`); O-D LOS exact against the published
//!   letters; interchange ETT ±0.5 s/veh; interchange LOS exact.
//! * Example Problem 5 — the published Exhibit 34-64 movement delays are
//!   not reproducible from the printed Chapter 19 / 23 equations (the
//!   published uniform delays are inconsistent with Equation 19-19 for
//!   M1 / M2 / M4 / M5 under any tabulated arrival type). The test
//!   asserts the equation-based results (±0.5 s/veh) with the published
//!   values and deltas recorded inline. O-D E, the case that most directly
//!   isolates the Equation 19-26 capacity term because it runs on the 3-lane
//!   external crossover at X = 0.84, reproduces the published 24.7 s/veh and
//!   LOS B exactly. The westbound O-Ds run short and carry the demand-weighted
//!   interchange ETT to 29.8 s/veh against the published 34.9, which is 0.2
//!   s/veh below the Exhibit 23-10 B/C boundary and so grades B against the
//!   published C.
//! * Example Problem 3 — saturation flows ±6 veh/h, effective greens
//!   ±0.01 s, the Exhibit 34-37 additional lost time ±0.05 s, capacities
//!   ±2 veh/h,
//!   v/c ±0.005, uniform delays ±0.15 s/veh. Control delays and O-D ETTs
//!   are asserted at the equation-based values with the published ones
//!   inline, and now agree to 0.2 s/veh on every O-D. O-D LOS letters are
//!   exact for the eight O-Ds that do not use an external right-turn lane,
//!   and the interchange LOS matches the published E.
//! * Example Problem 4 — saturation flows ±5 veh/h, lane utilization
//!   ±0.001, effective greens and demand-starvation lost times ±0.05 s,
//!   and control delays ±0.4 s/veh for the eight lane groups that are not
//!   an external approach. The two external approach capacities published
//!   in Exhibits 34-53 and 34-55 are not reproducible (see
//!   `test_ep4_diamond_demand_starvation_external_capacity_defect`).
//!
//! * Example Problem 6 — the Step 6 YIELD capacity chain is exact: all four
//!   v/c ratios reproduce the Exhibit 34-70 printed values at two decimals.
//!   The Exhibit 34-70 control delays do not reproduce (see
//!   `test_ep6_ddi_yield_control_delay_defect`), so the O-D table is
//!   asserted at the equation-based values with the published ones inline.
//! * Example Problem 2 — effective greens exact, saturation flows ±5 veh/h,
//!   capacities ±2 veh/h, v/c ±0.005, upstream filtering ±0.005, and control
//!   delays ±0.15 s/veh, for the eight lane groups that are not an internal
//!   shared through-and-right. Those two are asserted at engine values with
//!   the published ones inline, because Exhibit 34-22 gives them a lane
//!   utilization factor of 1.000 where Chapter 19's default for a three-lane
//!   through group is 0.908 (see
//!   `test_ep2_parclo_a2q_internal_lane_utilization_defect`). O-D ETTs are
//!   asserted at ±0.8 s/veh against the published Exhibit 34-29 values, with
//!   LOS letters and the v/c and R_Q flags exact, and the interchange ETT
//!   reads 61.5 s/veh against the published 61.3, LOS D either way.
//! * Example Problem 7 — the Exhibit 34-75 and 34-76 saturation flow
//!   exhibits are not reproducible from the HCM 7 equations, so O-D ETTs are
//!   asserted at the equation-based values with the published Exhibit 34-82
//!   ones inline. Eight of the ten O-D LOS letters and the interchange LOS
//!   land exactly, and the interchange reads 45.4 s/veh against the published
//!   48.3, LOS C either way. Three defects account for the gap and each has
//!   its own test: a lane width factor of 0.967 that HCM 7 Exhibit 19-20
//!   cannot produce for any width in its flat 10.0-to-12.9-ft tier, a
//!   heavy-vehicle factor of 1.000 on the ramp approaches that the example's
//!   own 5% heavy vehicles contradicts, and four left-turn traffic pressure
//!   values that Equation 23-15 does not give (it reproduces the other ten
//!   columns exactly). The protected-plus-permitted uniform delay itself is
//!   validated directly against Exhibit 34-77, reproducing 22.1 and 22.8
//!   s/veh from the published component saturation flows.
//! * Example Problem 8 — Exhibit 34-89 common greens exact, Exhibit 34-90
//!   queue lengths ±0.15 ft and additional lost times ±0.05 s.
//! * Example Problem 9 — Chapter 22 entry capacities ±1 pc/h of the
//!   Exhibit 34-101 values under the equation the example actually used, and
//!   ±0.15 s/veh on the delays under that same reading; the HCM 7 equations
//!   give different numbers and are asserted separately (see
//!   `test_ep9_roundabout_terminals_use_superseded_chapter_22_equations`).
//!
//! The one book defect these examples surface that is not a delay-equation
//! deviation is Exhibit 34-22's internal lane utilization, documented at its
//! assertion site.

use transportations_library::hcm::ramp_terminals::{
    adjacent_intersection_lost_time, common_green_time, demand_starvation_initial_queue,
    demand_starvation_lost_time, downstream_queue_length_ft, downstream_queue_lost_time,
    extra_distance_travel_time, los_roundabout_interchange_od, movements,
    traffic_pressure_factor, GreenInterval, Interchange, InterchangeMovement, OdMovement,
};
use transportations_library::hcm::signalized::exhibits::{
    heavy_vehicle_grade_factor, lane_width_factor,
};
use transportations_library::hcm::signalized::signalized::{qap_evaluate, QapInterval};
use transportations_library::hcm::common::delay::control_delay_roundabout;
use transportations_library::hcm::common::LevelOfService;
use transportations_library::hcm::roundabouts::{capacity_exponential, capacity_single_lane};

fn load_case(name: &str) -> Interchange {
    let path = format!(
        "{}/tests/ExampleCases/hcm/RampTerminals/{}",
        env!("CARGO_MANIFEST_DIR"),
        name
    );
    let json = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {path}: {e}"));
    serde_json::from_str(&json).unwrap_or_else(|e| panic!("parse {path}: {e}"))
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

fn od<'a>(ix: &'a Interchange, m: OdMovement) -> &'a transportations_library::hcm::ramp_terminals::OdResult {
    ix.od_results
        .iter()
        .find(|r| r.movement == m)
        .unwrap_or_else(|| panic!("O-D {m:?} missing"))
}

fn group<'a>(
    ix: &'a Interchange,
    m: InterchangeMovement,
) -> &'a transportations_library::hcm::ramp_terminals::LaneGroupResult {
    ix.results
        .iter()
        .find(|r| r.movement == m)
        .unwrap_or_else(|| panic!("lane group {m:?} missing"))
}

/// Chapter 34, Example Problem 1 (diamond): lane-group intermediates
/// against Exhibits 34-7 / 34-8 / 34-14 / 34-15.
#[test]
fn test_case1_diamond_lane_groups() {
    use movements::*;
    let mut ix = load_case("case1.json");
    ix.analyze();

    // Adjusted saturation flows (lane group totals), Exhibits 34-7/34-8.
    // Tolerance ±20 veh/h (±0.6%): the engine uses the Chapter 19
    // Equation 19-10 heavy-vehicle/grade form where the example used the
    // split fHV x fg convention on the ramps.
    for (mv, s_pub) in [
        (EB_EXT_THROUGH, 3_700.0),
        (EB_INT_THROUGH, 3_568.0),
        (EB_INT_LEFT, 1_703.0),
        (WB_EXT_THROUGH, 3_637.0),
        (WB_INT_THROUGH, 3_535.0),
        (WB_INT_LEFT, 1_767.0),
        (NB_RAMP_LEFT, 1_749.0),
        (NB_RAMP_RIGHT, 1_656.0),
        (SB_RAMP_LEFT, 1_734.0),
        (SB_RAMP_RIGHT, 1_638.0),
    ] {
        assert_near!(group(&ix, mv).sat_flow.unwrap(), s_pub, 20.0, format!("s {mv:?}"));
    }

    // No lost time due to downstream queues or demand starvation
    // (Exhibits 34-10 / 34-11): effective greens equal the displayed
    // greens.
    for (mv, g_pub) in [
        (EB_EXT_THROUGH, 63.0),
        (EB_INT_THROUGH, 97.0),
        (EB_INT_LEFT, 29.0),
        (WB_EXT_THROUGH, 63.0),
        (WB_INT_THROUGH, 111.0),
        (WB_INT_LEFT, 43.0),
        (NB_RAMP_LEFT, 53.0),
        (SB_RAMP_LEFT, 39.0),
    ] {
        assert_near!(
            group(&ix, mv).effective_green_s.unwrap(),
            g_pub,
            1e-9,
            format!("g {mv:?}")
        );
        assert_near!(
            group(&ix, mv).downstream_queue_lost_time_s.unwrap_or(0.0)
                + group(&ix, mv).demand_starvation_lost_time_s.unwrap_or(0.0),
            0.0,
            1e-9,
            format!("lost time {mv:?}")
        );
    }

    // Movement control delays, Exhibits 34-14 / 34-15 (±1.0 s/veh). Column 1
    // is asserted; column 2 in the comment is the published value where the
    // two differ. Only the two 2-lane external throughs differ, and only by
    // the d2 term: Example Problem 1 is the one worked example whose
    // published incremental delay reproduces on a per-lane basis (EB d2 = 4.65
    // per-lane against the published 4.6, 2.33 with the lane group capacity).
    // Equation 19-26 defines c_A as the Step 7 lane group capacity and both
    // Example Problem 3 and Example Problem 5 agree with that reading, so the
    // engine follows the equation and the EP1 worksheet is treated as a book
    // defect. See the note in src/hcm/ramp_terminals/ramp_terminals.rs.
    for (mv, d_engine) in [
        (EB_EXT_THROUGH, 41.99), // published 44.1
        (EB_INT_LEFT, 55.0),
        (EB_INT_THROUGH, 7.8),
        (WB_EXT_THROUGH, 34.61), // published 37.5
        (WB_INT_LEFT, 45.2),
        (WB_INT_THROUGH, 2.3),
        (NB_RAMP_LEFT, 43.4),
        (NB_RAMP_RIGHT, 43.4),
        (SB_RAMP_LEFT, 55.9),
        (SB_RAMP_RIGHT, 54.6),
    ] {
        assert_near!(
            group(&ix, mv).control_delay_s.unwrap(),
            d_engine,
            1.0,
            format!("d {mv:?}")
        );
    }

    // v/c ratios below 1 and queue storage ratios below 1 throughout
    // (Exhibits 34-12 / 34-13).
    for r in &ix.results {
        assert!(r.vc_ratio.unwrap() < 1.0, "{:?} v/c", r.movement);
        assert!(
            r.queue_storage_ratio.unwrap() < 1.0,
            "{:?} R_Q",
            r.movement
        );
    }
}

/// Chapter 34, Example Problem 1 (diamond): O-D results against
/// Exhibit 34-16 (delay/ETT ±1.0 s/veh; LOS exact).
#[test]
fn test_case1_diamond_od_results() {
    use LevelOfService as L;
    use OdMovement::*;
    let mut ix = load_case("case1.json");
    ix.analyze();

    // (O-D, demand, control delay, EDTT, ETT, LOS). Column 1 is the asserted
    // engine value; the comment carries the published Exhibit 34-16 pair
    // (delay, ETT) wherever the two differ by more than the ±1.0 s/veh
    // tolerance. Every difference is the Equation 19-26 d2 correction reaching
    // an external through movement, and nothing else: the six O-Ds whose path
    // includes EbExtThrough or WbExtThrough drop by exactly the 2.11 or 2.89
    // s/veh those two lane groups lost, while the four that avoid both (A, B,
    // C, D) still reproduce the published values inside tolerance. Every LOS
    // letter still matches the published one.
    let expected = [
        (A, 233.0, 45.7, 1.9, 47.7, L::C),
        (B, 227.0, 43.8, -1.9, 41.8, L::C),
        (C, 173.0, 54.6, -1.9, 52.7, L::C),
        (D, 206.0, 63.7, 1.9, 65.7, L::D),
        (E, 107.0, 97.0, 1.9, 98.9, L::E), // published 99.2 / 101.1
        (F, 89.0, 42.0, -1.9, 40.0, L::C), // published 44.2 /  42.3
        (G, 150.0, 34.6, -1.9, 32.7, L::C), // published 37.5 /  35.6
        (H, 236.0, 79.8, 1.9, 81.8, L::D), // published 82.7 /  84.6
        (I, 761.0, 49.8, 0.0, 49.8, L::C), // published 52.0 /  52.0
        (J, 650.0, 36.9, 0.0, 36.9, L::C), // published 39.8 /  39.8
    ];
    for (m, demand, delay, edtt, ett, los) in expected {
        let r = od(&ix, m);
        assert_near!(r.demand, demand, 1.0, format!("demand {m:?}"));
        assert_near!(r.control_delay_s, delay, 1.0, format!("delay {m:?}"));
        assert_near!(r.edtt_s, edtt, 0.1, format!("EDTT {m:?}"));
        assert_near!(r.ett_s, ett, 1.0, format!("ETT {m:?}"));
        assert_eq!(r.los, los, "LOS {m:?} (ETT {})", r.ett_s);
        assert!(!r.vc_exceeds_one && !r.rq_exceeds_one, "{m:?} flags");
    }

    // Interchange ETT 50.7 s/veh against the published 52.4 (Exhibit 34-16
    // totals row), same LOS C. The 1.7 s/veh is the demand-weighted share of
    // the two external-through d2 corrections above.
    assert_near!(ix.interchange_ett_s.unwrap(), 50.7, 0.5, "interchange ETT");
    assert_eq!(ix.interchange_los.unwrap(), L::C);
}

/// Chapter 34, Example Problem 5 (DDI with signal control): saturation
/// flows and effective greens against Exhibits 34-62 / 34-63, and O-D
/// results against Exhibit 34-65 (see the module notes for the
/// documented deltas).
#[test]
fn test_case2_ddi_results() {
    use movements::*;
    use LevelOfService as L;
    use OdMovement::*;
    let mut ix = load_case("case2.json");
    ix.analyze();

    // Adjusted saturation flows (Exhibit 34-62 lane-group totals):
    // M6 = 3,563, M2 = 2,045, M1 = 3,229, M5 = 3,156, M3 = 1,682,
    // M4 = 1,601, M7 = 1,674, M8 = 1,601 veh/h. The external crossovers
    // deviate up to 1.2% from the published values through the f_LU
    // rounding (published models round %V_Lmax to two decimals) and the
    // Equation 23-15 left-turn form on the ramp lefts (the example used
    // the through-movement form).
    for (mv, s_pub, tol) in [
        (EB_EXT_THROUGH, 3_563.0, 55.0),
        (WB_EXT_THROUGH, 2_045.0, 5.0),
        (EB_INT_THROUGH, 3_229.0, 5.0),
        (WB_INT_THROUGH, 3_156.0, 5.0),
        (NB_RAMP_LEFT, 1_682.0, 25.0),
        (NB_RAMP_RIGHT, 1_601.0, 5.0),
        (SB_RAMP_LEFT, 1_674.0, 20.0),
        (SB_RAMP_RIGHT, 1_601.0, 5.0),
    ] {
        assert_near!(group(&ix, mv).sat_flow.unwrap(), s_pub, tol, format!("s {mv:?}"));
    }

    // Effective green times (Exhibit 34-63 publishes rounded-down
    // values: 31, 20, 35, 25, 24, 20, 14, 30 s).
    for (mv, g_pub, tol) in [
        (EB_EXT_THROUGH, 31.0, 0.1), // M6
        (WB_EXT_THROUGH, 21.0, 0.1), // M2: published 20 (VERIFY-HCM: 25+5-9)
        (EB_INT_THROUGH, 35.0, 0.1), // M1
        (WB_INT_THROUGH, 25.0, 0.1), // M5
        (NB_RAMP_LEFT, 24.5, 0.1),   // M3: published 24
        (NB_RAMP_RIGHT, 20.1, 0.1),  // M4: published 20
        (SB_RAMP_LEFT, 14.5, 0.1),   // M7: published 14
        (SB_RAMP_RIGHT, 30.1, 0.1),  // M8: published 30
    ] {
        assert_near!(
            group(&ix, mv).effective_green_s.unwrap(),
            g_pub,
            tol,
            format!("g {mv:?}")
        );
    }

    // DDIs have no demand starvation lost time (Chapter 23 Step 4).
    assert_near!(
        group(&ix, EB_INT_THROUGH)
            .demand_starvation_lost_time_s
            .unwrap(),
        0.0,
        1e-12,
        "M1 L_DS"
    );

    // O-D results. Column 1: equation-based expectation (asserted,
    // ±0.5 s/veh); column 2 in the comment: published Exhibit 34-65
    // value. Example Problem 5 is the sharpest case for evaluating the
    // Equation 19-26 incremental delay with the lane group capacity: O-D E
    // runs entirely on the 3-lane eastbound external crossover at X = 0.84,
    // and it lands on the published 24.7 s/veh where the per-lane form gave
    // 33.9 s/veh and the wrong LOS letter. It also moves the O-Ds that use the
    // 2-lane westbound crossover further from their published values, but the
    // Exhibit 34-64 movement delays those come from are already documented as
    // not reproducible from the printed equations (the published uniform
    // delays are inconsistent with Equation 19-19 for M1 / M2 / M4 / M5), so
    // the d2 term is not what is being measured there.
    let expected = [
        (A, 42.7, L::C), // published 40.1 C
        (B, 21.4, L::B), // published 21.0 B
        (C, 12.1, L::A), // published 11.4 A
        (D, 64.8, L::D), // published 76.3 D
        (E, 24.7, L::B), // published 24.7 B
        (F, 0.0, L::A),  // free-flow bypass
        (G, 0.0, L::A),  // free-flow bypass
        (H, 31.5, L::C), // published 50.3 C
        (I, 37.0, L::C), // published 45.5 C
        (J, 48.3, L::C), // published 66.4 D
    ];
    for (m, ett, los) in expected {
        let r = od(&ix, m);
        assert_near!(r.ett_s, ett, 0.5, format!("ETT {m:?}"));
        assert_eq!(r.los, los, "LOS {m:?} (ETT {})", r.ett_s);
    }

    // Demand-weighted interchange ETT 29.8 s/veh against the published 34.9
    // (Exhibit 34-65 totals row), LOS B against the published C. The band
    // boundary is 30 s/veh, so the aggregate sits 0.2 s/veh on the wrong side
    // of it and is carried by the westbound O-Ds above rather than by the
    // Step 9 aggregation.
    assert_eq!(ix.interchange_los.unwrap(), L::B);
    assert_near!(ix.interchange_ett_s.unwrap(), 29.8, 0.5, "interchange ETT");
}

/// Chapter 34, Example Problem 2 (Parclo A-2Q): the Step 4 common green
/// and downstream-queue free functions against Exhibits 34-23 and 34-24,
/// driven directly rather than through `case6.json`, so that the free
/// functions stay pinned to the published intermediates independently of
/// how the fixture composes them. I-75 at Newberry Avenue, C = 140 s,
/// D = 800 ft, both offsets zero, PHF = 0.95.
#[test]
fn test_ep2_parclo_a2q_common_green_and_downstream_queues() {
    const C: f64 = 140.0;
    const D_FT: f64 = 800.0;
    const L_H: f64 = 25.0;

    // Exhibit 34-23 movement green windows. Intersection I runs phases
    // 0-25 / 30-90 / 95-135 and Intersection II runs 0-25 / 30-65 /
    // 70-135; the WB external through holds NEMA 6 in phases 1 and 3 of
    // Intersection II and therefore receives green twice per cycle.
    let eb_ext = [GreenInterval { begin_s: 0.0, duration_s: 90.0 }];
    let eb_int = [GreenInterval { begin_s: 70.0, duration_s: 65.0 }];
    let wb_ext = [
        GreenInterval { begin_s: 0.0, duration_s: 25.0 },
        GreenInterval { begin_s: 70.0, duration_s: 65.0 },
    ];
    let wb_int = [GreenInterval { begin_s: 30.0, duration_s: 60.0 }];
    let sb_ramp = [GreenInterval { begin_s: 95.0, duration_s: 40.0 }];
    let nb_ramp = [GreenInterval { begin_s: 30.0, duration_s: 35.0 }];

    // Exhibit 34-23 common green column: 20, 20, 40, 35 s.
    assert_near!(common_green_time(&eb_ext, &eb_int, C), 20.0, 1e-9, "CG EB EXT/INT");
    assert_near!(common_green_time(&wb_ext, &wb_int, C), 20.0, 1e-9, "CG WB EXT/INT");
    assert_near!(common_green_time(&sb_ramp, &eb_int, C), 40.0, 1e-9, "CG SB ramp/EB INT");
    assert_near!(common_green_time(&nb_ramp, &wb_int, C), 35.0, 1e-9, "CG NB ramp/WB INT");

    // Exhibit 34-24: (subject, feeding flow, feeding lanes, feeding
    // green, downstream green, common green, published queue length ft,
    // published DQ ft). Feeding flows are the PHF-adjusted values of
    // Exhibit 34-19 composed by the Exhibit 34-163 worksheet: the ramp
    // left feeds the arterial phase and the external arterial through
    // feeds the ramp phase.
    for (what, v_feed, n_feed, g_feed, g_down, cg, q_pub, dq_pub) in [
        ("EB EXT-TH", 289.0, 1u32, 40.0, 65.0, 20.0, 0.9, 799.0),
        ("SB-L", 1_066.0, 3, 90.0, 65.0, 40.0, 48.6, 751.0),
        ("WB EXT-TH", 229.0, 1, 35.0, 60.0, 20.0, 0.0, 800.0),
        ("NB-L", 1_249.0, 3, 95.0, 60.0, 35.0, 89.4, 711.0),
    ] {
        // ±0.3 ft: the published values round the Equation 23-33 / 23-34
        // inputs to whole veh/h before multiplying by L_h.
        let q = downstream_queue_length_ft(v_feed, n_feed, g_feed, g_down, cg, C, L_H);
        assert_near!(q, q_pub, 0.3, format!("Q {what}"));
        // The DQ row is printed to whole feet.
        assert_near!(D_FT - q, dq_pub, 0.5, format!("DQ {what}"));
        // Every DQ clears the 200 ft threshold, so Exhibit 34-24 reports
        // zero additional lost time on all four approaches.
        assert_near!(
            downstream_queue_lost_time(g_feed, D_FT - q, cg, C),
            0.0,
            1e-12,
            format!("L_D {what}")
        );
    }
}

/// Chapter 34, Example Problem 2 (Parclo A-2Q): lane-group intermediates
/// against Exhibits 34-21 / 34-22 / 34-24 / 34-25 / 34-26 / 34-27 / 34-28.
///
/// This is the first fixture whose lane groups are not the diamond
/// skeleton. Each arterial direction has an external through, an external
/// left onto the loop ramp, and an internal shared through-and-right, and
/// the two internal approaches have no left turn at all.
#[test]
fn test_ep2_parclo_a2q_lane_groups() {
    use movements::*;
    let mut ix = load_case("case6.json");
    ix.analyze();

    // Adjusted saturation flows, Exhibits 34-21 and 34-22 (lane group
    // totals; the exhibits print the northbound and southbound values
    // per lane and those groups have one lane each). ±5 veh/h: the
    // published traffic pressure and turn radius factors are rounded to
    // three decimals. The two internal through-and-right groups are
    // excluded and asserted separately.
    for (mv, s_pub) in [
        (EB_EXT_THROUGH, 3_786.0),
        (EB_EXT_LEFT, 1_798.0),
        (WB_EXT_THROUGH, 3_310.0),
        (WB_EXT_LEFT, 1_733.0),
        (NB_RAMP_LEFT, 1_674.0),
        (NB_RAMP_RIGHT, 1_658.0),
        (SB_RAMP_LEFT, 1_701.0),
        (SB_RAMP_RIGHT, 1_617.0),
    ] {
        assert_near!(group(&ix, mv).sat_flow.unwrap(), s_pub, 5.0, format!("s {mv:?}"));
    }

    // Effective greens (Exhibit 34-24 for the four movements it covers,
    // Exhibits 34-25 and 34-26 for the rest). Every approach carries the
    // same 6 s total lost time, so g' is the displayed green minus 1 s,
    // and every value is exact. The westbound external through is the
    // case that needs the wrapping green window: 95 s of displayed green
    // across the cycle boundary, not the 90 s the two windows of
    // Exhibit 34-23 add up to.
    for (mv, g_pub) in [
        (EB_EXT_THROUGH, 89.0),
        (EB_EXT_LEFT, 24.0),
        (EB_INT_THROUGH_RIGHT, 64.0),
        (WB_EXT_THROUGH, 94.0),
        (WB_EXT_LEFT, 24.0),
        (WB_INT_THROUGH_RIGHT, 59.0),
        (NB_RAMP_LEFT, 34.0),
        (NB_RAMP_RIGHT, 34.0),
        (SB_RAMP_LEFT, 39.0),
        (SB_RAMP_RIGHT, 39.0),
    ] {
        assert_near!(
            group(&ix, mv).effective_green_s.unwrap(),
            g_pub,
            0.01,
            format!("g {mv:?}")
        );
    }

    // Exhibit 34-24: every DQ clears the 200 ft threshold on an 800 ft
    // internal link, so no approach carries additional lost time due to a
    // downstream queue, and neither internal approach is starved. The
    // starvation term is zero here for a structural reason rather than an
    // arithmetic one: a parclo A internal approach has no left turn, so
    // every Intersection I phase feeds the eastbound link.
    for mv in [EB_EXT_THROUGH, WB_EXT_THROUGH, NB_RAMP_LEFT, SB_RAMP_LEFT] {
        assert_near!(
            group(&ix, mv).downstream_queue_lost_time_s.unwrap(),
            0.0,
            1e-12,
            format!("L_D {mv:?}")
        );
    }
    for mv in [EB_INT_THROUGH_RIGHT, WB_INT_THROUGH_RIGHT] {
        assert_near!(
            group(&ix, mv).demand_starvation_lost_time_s.unwrap(),
            0.0,
            1e-12,
            format!("L_DS {mv:?}")
        );
    }

    // Capacities, v/c ratios, upstream filtering factors, and control
    // delays (Exhibits 34-25 through 34-28). The Equation 19-6 filtering
    // factors on the internal groups are the first check of the composed
    // reading on a shared through-and-right: 0.90 eastbound off X_u = 0.44
    // and 0.81 westbound off X_u = 0.56, both published.
    // The v/c tolerance is ±0.01 rather than the ±0.005 Example Problem 3
    // gets, because the four ramp groups carry a capacity 1 to 2 veh/h
    // below the published one (the exhibits round f_HVg and f_v to three
    // decimals on a single-lane group, where that rounding is worth 0.2%
    // of the saturation flow) and that is enough to move the second
    // decimal of X. The capacities those ratios come from are asserted
    // against the published values on the same row.
    for (mv, c_pub, x_pub, i_pub, d_pub) in [
        (EB_EXT_THROUGH, 2_407.0, 0.44, 1.00, 13.5),
        (EB_EXT_LEFT, 308.0, 1.02, 1.00, 115.7),
        (WB_EXT_THROUGH, 2_222.0, 0.56, 1.00, 13.2),
        (WB_EXT_LEFT, 297.0, 0.58, 1.00, 61.6),
        (NB_RAMP_LEFT, 407.0, 0.56, 1.00, 52.1),
        (NB_RAMP_RIGHT, 403.0, 0.65, 1.00, 55.7),
        (SB_RAMP_LEFT, 474.0, 0.61, 1.00, 49.7),
        (SB_RAMP_RIGHT, 450.0, 0.28, 1.00, 41.1),
    ] {
        let r = group(&ix, mv);
        assert_near!(r.capacity.unwrap(), c_pub, 2.0, format!("c {mv:?}"));
        assert_near!(r.vc_ratio.unwrap(), x_pub, 0.01, format!("X {mv:?}"));
        assert_near!(r.upstream_filtering.unwrap(), i_pub, 0.005, format!("I {mv:?}"));
        assert_near!(
            r.control_delay_s.unwrap(),
            d_pub,
            0.15,
            format!("d {mv:?}")
        );
    }
    for mv in [EB_INT_THROUGH_RIGHT, WB_INT_THROUGH_RIGHT] {
        assert_near!(
            group(&ix, mv).upstream_filtering.unwrap(),
            if mv == EB_INT_THROUGH_RIGHT { 0.90 } else { 0.81 },
            0.005,
            format!("I {mv:?}")
        );
    }

    // Exhibit 34-25: the eastbound external left is the movement that
    // fails, at v/c 1.02 and a queue 1.96 times its 200 ft bay.
    assert!(group(&ix, EB_EXT_LEFT).vc_ratio.unwrap() > 1.0, "EB EXT-L v/c");
    assert_near!(
        group(&ix, EB_EXT_LEFT).queue_storage_ratio.unwrap(),
        1.96,
        0.01,
        "EB EXT-L R_Q"
    );
    // The westbound external left is the same 200 ft bay under the lighter
    // demand, and Exhibit 34-25 publishes R_Q = 0.78 for it. Pinned at the
    // published value rather than merely bounded below 1.0, so a regression
    // that inflates the queue has to clear the exhibit and not just the
    // overflow threshold.
    assert_near!(
        group(&ix, WB_EXT_LEFT).queue_storage_ratio.unwrap(),
        0.78,
        0.01,
        "WB EXT-L R_Q"
    );
}

/// Chapter 34, Example Problem 2: Exhibit 34-22 gives the two internal
/// shared through-and-right groups a lane utilization factor of 1.000,
/// where the engine applies the Chapter 19 Exhibit 19-15 default of 0.908
/// for a three-lane through group. This test pins both readings and the
/// evidence for choosing the default.
///
/// Chapter 23 Step 3 sends every approach that is not an external arterial
/// approach to "the procedures of Chapter 19", and Example Problems 1, 3,
/// and 4 all print the Chapter 19 default in their own f_LU column
/// (Exhibit 34-34 prints 0.908 for exactly this three-lane internal
/// through group). Example Problem 2 is the only one that prints 1.000, so
/// the text and three worked examples outvote one column, the same grounds
/// on which the Equation 19-26 d2 convention was settled.
///
/// The measurement, held here so a later reader does not have to redo it:
/// overriding f_LU to the published 1.000 reproduces the published
/// saturation flows to within 4 veh/h, which shows the factor is the whole
/// of that gap, but it does not improve the published O-D table. Mean
/// absolute error against the ten Exhibit 34-29 ETTs is 0.26 s/veh at
/// 0.908 and 0.63 s/veh at 1.000, because O-Ds A, H, and J each move about
/// 1.7 s/veh away from their published values. Exhibit 34-22 is therefore
/// inconsistent with Exhibits 34-27 through 34-29 of its own example.
#[test]
fn test_ep2_parclo_a2q_internal_lane_utilization_defect() {
    use movements::*;
    let mut ix = load_case("case6.json");
    ix.analyze();

    for (mv, f_lu_engine, s_engine, s_published) in [
        (EB_INT_THROUGH_RIGHT, 0.908, 4_766.5, 5_253.0),
        (WB_INT_THROUGH_RIGHT, 0.908, 4_784.1, 5_271.0),
    ] {
        let r = group(&ix, mv);
        assert_near!(
            r.lane_utilization.unwrap(),
            f_lu_engine,
            1e-9,
            format!("f_LU {mv:?}")
        );
        assert_near!(r.sat_flow.unwrap(), s_engine, 0.5, format!("s {mv:?}"));
        // The published value is the engine value divided by the Chapter 19
        // default, to within the exhibit's own three-decimal rounding.
        assert_near!(
            r.sat_flow.unwrap() / f_lu_engine,
            s_published,
            4.0,
            format!("s {mv:?} at the published f_LU")
        );
    }

    // Control delays, engine against published (Exhibit 34-27).
    assert_near!(
        group(&ix, EB_INT_THROUGH_RIGHT).control_delay_s.unwrap(),
        21.03, // published 20.3
        0.05,
        "EB INT-TH&R d"
    );
    assert_near!(
        group(&ix, WB_INT_THROUGH_RIGHT).control_delay_s.unwrap(),
        26.85, // published 26.8
        0.05,
        "WB INT-TH&R d"
    );
}

/// Chapter 34, Example Problem 2 (Parclo A-2Q): O-D results against
/// Exhibit 34-29.
///
/// This is the routing check for the family. Every O-D of a parclo A-2Q
/// takes a different turn from its diamond counterpart at one terminal or
/// the other, and the published delay column decomposes cleanly into the
/// Exhibits 34-27 and 34-28 movement delays, which is what fixes the
/// routing: O-D E is the external through plus the internal
/// through-and-right (13.5 + 20.3 = 33.8), where a diamond would send it
/// through an internal left; O-D F is the external left alone (115.7),
/// where a diamond would share it with the external through; and O-D A is
/// the ramp left plus the opposite internal through-and-right
/// (52.1 + 26.8 = 78.9).
#[test]
fn test_ep2_parclo_a2q_od_results() {
    use LevelOfService as L;
    use OdMovement::*;
    let mut ix = load_case("case6.json");
    ix.analyze();

    // (O-D, PHF-adjusted demand, published ETT, published LOS, v/c flag).
    // Column 3 is asserted at ±0.8 s/veh. What is left is the internal
    // lane utilization defect above, which reaches the six O-Ds that use an
    // internal through-and-right, plus a uniform 0.05 s/veh on every
    // diverted O-D from Equation 23-50's printed 1.47 conversion.
    let expected = [
        (A, 229.0, 99.5, L::E, false),
        (B, 263.0, 40.1, L::C, false),
        (C, 126.0, 25.5, L::B, false),
        (D, 289.0, 90.6, L::E, false),
        (E, 198.0, 71.5, L::D, false),
        (F, 316.0, 136.3, L::F, true),
        (G, 174.0, 82.2, L::D, false),
        (H, 368.0, 77.7, L::D, false),
        (I, 868.0, 33.8, L::C, false),
        (J, 881.0, 40.0, L::C, false),
    ];
    for (m, demand, ett, los, flagged) in expected {
        let r = od(&ix, m);
        assert_near!(r.demand, demand, 1.0, format!("demand {m:?}"));
        assert_near!(r.ett_s, ett, 0.8, format!("ETT {m:?}"));
        assert_eq!(r.los, los, "LOS {m:?} (ETT {})", r.ett_s);
        assert_eq!(r.vc_exceeds_one, flagged, "{m:?} v/c flag");
        assert_eq!(r.rq_exceeds_one, flagged, "{m:?} R_Q flag");
    }

    // Interchange ETT 61.5 s/veh against the published 61.3 (Exhibit 34-29
    // totals row), LOS D as published.
    assert_near!(ix.interchange_ett_s.unwrap(), 61.3, 0.5, "interchange ETT");
    assert_eq!(ix.interchange_los.unwrap(), L::D, "interchange LOS");
}

/// Chapter 34, Example Problem 3 (diamond with queue spillback):
/// lane-group intermediates against Exhibits 34-34 / 34-35 / 34-37 /
/// 34-39 / 34-41 / 34-42.
#[test]
fn test_ep3_diamond_spillback_lane_groups() {
    use movements::*;
    let mut ix = load_case("case3.json");
    ix.analyze();

    // Adjusted saturation flows, Exhibits 34-34 and 34-35. Tolerance
    // ±6 veh/h (±0.4%): the ramp groups carry the published f_HVg of
    // 0.990 rounded to three decimals.
    for (mv, s_pub) in [
        (EB_EXT_THROUGH, 3_400.0),
        (EB_INT_THROUGH, 4_807.0),
        (EB_INT_LEFT, 1_676.0),
        (WB_EXT_THROUGH, 4_021.0),
        (WB_INT_THROUGH, 4_822.0),
        (WB_INT_LEFT, 1_764.0),
        (NB_RAMP_LEFT, 1_628.0),
        (NB_RAMP_RIGHT, 1_703.0),
        (SB_RAMP_LEFT, 1_600.0),
        (SB_RAMP_RIGHT, 1_606.0),
    ] {
        assert_near!(group(&ix, mv).sat_flow.unwrap(), s_pub, 6.0, format!("s {mv:?}"));
    }

    // Exhibit 34-37: the SB off-ramp left is the only approach with
    // additional lost time due to the downstream queue. The 108.6 ft
    // queue and the 5.5 s lost time are the headline numbers of this
    // example problem.
    assert_near!(
        group(&ix, SB_RAMP_LEFT).downstream_queue_lost_time_s.unwrap(),
        5.5,
        0.05,
        "SB-L L_D"
    );
    for mv in [EB_EXT_THROUGH, WB_EXT_THROUGH, NB_RAMP_LEFT] {
        assert_near!(
            group(&ix, mv).downstream_queue_lost_time_s.unwrap(),
            0.0,
            1e-12,
            format!("L_D {mv:?}")
        );
    }
    // Exhibit 34-38: neither internal through movement is starved, so
    // both keep their full displayed green plus the change interval.
    for mv in [EB_INT_THROUGH, WB_INT_THROUGH] {
        assert_near!(
            group(&ix, mv).demand_starvation_lost_time_s.unwrap(),
            0.0,
            1e-12,
            format!("L_DS {mv:?}")
        );
    }

    // Effective greens (Exhibits 34-37 / 34-38 / 34-39) and capacities
    // (Exhibits 34-39 / 34-40), with the Exhibit 34-39 v/c ratios and the
    // Equation 19-6 upstream filtering factors.
    for (mv, g_pub, c_pub, x_pub, i_pub) in [
        (EB_EXT_THROUGH, 59.0, 1_672.0, 1.23, 1.00),
        (EB_INT_THROUGH, 71.0, 2_844.0, 0.29, 0.09),
        (EB_INT_LEFT, 27.0, 377.0, 0.18, 0.09),
        (WB_EXT_THROUGH, 39.0, 1_307.0, 0.80, 1.00),
        (WB_INT_THROUGH, 83.0, 3_336.0, 0.27, 0.49),
        (WB_INT_LEFT, 19.0, 279.0, 1.09, 0.49),
        (NB_RAMP_LEFT, 39.0, 529.0, 0.26, 1.00),
        (NB_RAMP_RIGHT, 39.0, 553.0, 0.86, 1.00),
        (SB_RAMP_LEFT, 21.5, 287.0, 0.20, 1.00),
        (SB_RAMP_RIGHT, 27.0, 362.0, 0.30, 1.00),
    ] {
        let r = group(&ix, mv);
        // ±0.01 s: every green but the SB off-ramp left is exact, and
        // that one carries the 5.4987 s Equation 23-29 lost time the
        // exhibit prints as 5.5.
        assert_near!(r.effective_green_s.unwrap(), g_pub, 0.01, format!("g {mv:?}"));
        assert_near!(r.capacity.unwrap(), c_pub, 2.0, format!("c {mv:?}"));
        assert_near!(r.vc_ratio.unwrap(), x_pub, 0.005, format!("X {mv:?}"));
        assert_near!(r.upstream_filtering.unwrap(), i_pub, 0.005, format!("I {mv:?}"));
    }

    // Uniform delays, Exhibits 34-41 and 34-42 (±0.15 s/veh).
    for (mv, d1_pub) in [
        (EB_EXT_THROUGH, 30.5),
        (EB_INT_THROUGH, 5.8),
        (EB_INT_LEFT, 37.5),
        (WB_EXT_THROUGH, 37.0),
        (WB_INT_THROUGH, 1.5),
        (WB_INT_LEFT, 50.5),
        (NB_RAMP_LEFT, 29.9),
        (NB_RAMP_RIGHT, 37.9),
        (SB_RAMP_LEFT, 41.9),
        (SB_RAMP_RIGHT, 38.6),
    ] {
        assert_near!(
            group(&ix, mv).uniform_delay_s.unwrap(),
            d1_pub,
            0.15,
            format!("d1 {mv:?}")
        );
    }

    // Control delays reproduce Exhibits 34-41 / 34-42 to ±0.4 s/veh
    // everywhere except the two external through movements (asserted in
    // `test_ep3_diamond_spillback_od_results` with the d2 convention
    // note).
    for (mv, d_pub) in [
        (EB_INT_THROUGH, 5.8),
        (EB_INT_LEFT, 37.6),
        (WB_INT_THROUGH, 1.6),
        (WB_INT_LEFT, 114.6),
        (NB_RAMP_LEFT, 31.1),
        (NB_RAMP_RIGHT, 53.6),
        (SB_RAMP_LEFT, 43.5),
        (SB_RAMP_RIGHT, 40.7),
    ] {
        assert_near!(
            group(&ix, mv).control_delay_s.unwrap(),
            d_pub,
            0.4,
            format!("d {mv:?}")
        );
    }

    // Queue storage ratio, Exhibit 34-39: the WB internal left spills
    // back out of its 200 ft bay (R_Q = 1.65).
    assert_near!(
        group(&ix, WB_INT_LEFT).queue_storage_ratio.unwrap(),
        1.65,
        0.01,
        "WB INT-L R_Q"
    );
    assert!(group(&ix, EB_EXT_THROUGH).queue_storage_ratio.unwrap() > 1.0, "EB EXT R_Q > 1");
}

/// Chapter 34, Example Problem 3 (diamond with queue spillback): O-D
/// results against Exhibit 34-43, plus the two engine gaps the example
/// exposes.
#[test]
fn test_ep3_diamond_spillback_od_results() {
    use LevelOfService as L;
    use OdMovement::*;
    let mut ix = load_case("case3.json");
    ix.analyze();

    // The exclusive external right-turn lane groups of Exhibits 34-34 /
    // 34-39 / 34-41, which the diamond skeleton had no composition for and
    // which used to leave O-Ds F and G scored as free-flowing at -1.2
    // s/veh. Published against engine: s 1,675 / 1,674.998 and 1,614 /
    // 1,613.69, c 824 / 823.54 and 524 / 524.45, X 0.38 / 0.3755 and
    // 0.13 / 0.1337, d 20.3 / 20.32 and 29.1 / 29.11 s/veh, R_Q 0.23 /
    // 0.2274 and 0.06 / 0.0617.
    for (mv, s_pub, c_pub, x_pub, d_pub, rq_pub) in [
        (movements::EB_EXT_RIGHT, 1_675.0, 824.0, 0.38, 20.3, 0.23),
        (movements::WB_EXT_RIGHT, 1_614.0, 524.0, 0.13, 29.1, 0.06),
    ] {
        let r = group(&ix, mv);
        assert_near!(r.sat_flow.unwrap(), s_pub, 1.0, format!("s {mv:?}"));
        assert_near!(r.capacity.unwrap(), c_pub, 1.0, format!("c {mv:?}"));
        assert_near!(r.vc_ratio.unwrap(), x_pub, 0.005, format!("X {mv:?}"));
        assert_near!(r.control_delay_s.unwrap(), d_pub, 0.05, format!("d {mv:?}"));
        assert_near!(
            r.queue_storage_ratio.unwrap(),
            rq_pub,
            0.005,
            format!("R_Q {mv:?}")
        );
    }

    // Exhibit 34-43 O-Ds F and G, now scored through those lane groups
    // rather than as free-flowing: published 19.1 and 27.9 s/veh, both
    // LOS B.
    assert_near!(od(&ix, F).ett_s, 19.1, 0.1, "O-D F ETT");
    assert_eq!(od(&ix, F).los, L::B, "O-D F LOS");
    assert_near!(od(&ix, G).ett_s, 27.9, 0.1, "O-D G ETT");
    assert_eq!(od(&ix, G).los, L::B, "O-D G LOS");

    // The remaining eight O-Ds. Column 1: equation-based expectation
    // (asserted); column 2 in the comment: published Exhibit 34-43 value.
    // The four O-Ds whose path includes an external through movement used to
    // run 9.2 to 9.5 s/veh long, because the engine evaluated the Equation
    // 19-26 incremental delay with the per-lane capacity. With the lane group
    // capacity that the equation's variable list calls for, the eastbound
    // external through reproduces the published d2 of 110.5 s/veh at 110.36
    // (c = 1,672 veh/h) instead of 119.9 (c/N = 557 veh/h/ln), and all four
    // O-Ds land within 0.2 s/veh of the published values.
    let expected = [
        (A, 139.0, 34.0, L::C),  // published 33.9 C
        (B, 474.0, 52.6, L::C),  // published 52.4 C
        (C, 107.0, 39.6, L::C),  // published 39.5 C
        (D, 58.0, 50.5, L::C),   // published 50.5 C
        (E, 1_294.0, 179.7, L::F), // published 179.8 F
        (H, 304.0, 158.1, L::F), // published 158.2 F
        (I, 768.0, 146.7, L::F), // published 146.8 F
        (J, 747.0, 44.0, L::C),  // published 44.0 C
    ];
    for (m, demand, ett, los) in expected {
        let r = od(&ix, m);
        assert_near!(r.demand, demand, 1.0, format!("demand {m:?}"));
        assert_near!(r.ett_s, ett, 0.5, format!("ETT {m:?}"));
        assert_eq!(r.los, los, "LOS {m:?} (ETT {})", r.ett_s);
    }
    // Exhibit 34-43 flags v/c > 1 and R_Q > 1 for O-Ds E, H, and I only.
    for m in [E, H, I] {
        assert!(od(&ix, m).vc_exceeds_one, "{m:?} v/c flag");
        assert!(od(&ix, m).rq_exceeds_one, "{m:?} R_Q flag");
    }
    for m in [A, B, C, D, J] {
        assert!(!od(&ix, m).vc_exceeds_one, "{m:?} v/c flag");
        assert!(!od(&ix, m).rq_exceeds_one, "{m:?} R_Q flag");
    }

    // Interchange ETT 110.24 s/veh against the published 110.3 (Exhibit
    // 34-43 totals row). It was 108.3 while O-Ds F and G were scored as
    // free-flowing; the 1.9 s/veh those two were costing the aggregate is
    // the demand-weighted share of their published delays.
    assert_near!(ix.interchange_ett_s.unwrap(), 110.3, 0.1, "interchange ETT");

    // Interchange LOS E, matching the published letter. Exhibit 23-10 applies
    // the v/c and R_Q flags to an individual O-D, so O-Ds E, H, and I are F
    // above, but Step 9 grades the interchange from the demand-weighted ETT
    // alone and explicitly anticipates a failing O-D being masked at the
    // interchange level. Exhibit 34-43 does the same, publishing LOS E at
    // ETT 110.3 while carrying three flagged O-Ds.
    assert_eq!(ix.interchange_los.unwrap(), L::E, "interchange LOS");
}

/// Chapter 34, Example Problem 4 (diamond with demand starvation):
/// lane-group intermediates against Exhibits 34-47 / 34-48 / 34-49 /
/// 34-51 / 34-53 / 34-55 / 34-56.
#[test]
fn test_ep4_diamond_demand_starvation_lane_groups() {
    use movements::*;
    let mut ix = load_case("case4.json");
    ix.analyze();

    // Lane utilization for the external approaches, Exhibit 34-47.
    // Equation 23-17 with the Exhibit 23-24 three-lane diamond
    // coefficients reproduces the published %V_L1 (0.3879 EB, 0.4032 WB)
    // and hence f_LU exactly, so this example needs no override.
    assert_near!(group(&ix, EB_EXT_THROUGH).lane_utilization.unwrap(), 0.8593, 0.001, "f_LU EB");
    assert_near!(group(&ix, WB_EXT_THROUGH).lane_utilization.unwrap(), 0.8266, 0.001, "f_LU WB");

    // Adjusted saturation flows, Exhibits 34-48 and 34-49 (±5 veh/h; the
    // ramp groups carry the published f_HVg of 0.990 rounded to three
    // decimals).
    for (mv, s_pub) in [
        (EB_EXT_THROUGH, 4_597.0),
        (EB_INT_THROUGH, 4_834.0),
        (EB_INT_LEFT, 1_714.0),
        (WB_EXT_THROUGH, 4_428.0),
        (WB_INT_THROUGH, 4_799.0),
        (WB_INT_LEFT, 1_741.0),
        (NB_RAMP_LEFT, 1_617.0),
        (NB_RAMP_RIGHT, 1_625.0),
        (SB_RAMP_LEFT, 1_635.0),
        (SB_RAMP_RIGHT, 1_606.0),
    ] {
        assert_near!(group(&ix, mv).sat_flow.unwrap(), s_pub, 5.0, format!("s {mv:?}"));
    }

    // Exhibit 34-51: no approach loses time to a downstream queue (both
    // DQ values clear the 200 ft threshold at 369 and 360 ft).
    for mv in [EB_EXT_THROUGH, WB_EXT_THROUGH, NB_RAMP_LEFT, SB_RAMP_LEFT] {
        assert_near!(
            group(&ix, mv).downstream_queue_lost_time_s.unwrap(),
            0.0,
            1e-12,
            format!("L_D {mv:?}")
        );
    }

    // Exhibit 34-52: both internal through movements are starved. These
    // are the headline numbers of this example problem.
    assert_near!(
        group(&ix, EB_INT_THROUGH).demand_starvation_lost_time_s.unwrap(),
        14.7,
        0.05,
        "EB INT-TH L_DS"
    );
    assert_near!(
        group(&ix, WB_INT_THROUGH).demand_starvation_lost_time_s.unwrap(),
        18.6,
        0.05,
        "WB INT-TH L_DS"
    );

    // Effective greens, Exhibits 34-51 / 34-52 / 34-53.
    for (mv, g_pub) in [
        (EB_EXT_THROUGH, 25.0),
        (EB_INT_THROUGH, 45.3),
        (EB_INT_LEFT, 25.0),
        (WB_EXT_THROUGH, 30.0),
        (WB_INT_THROUGH, 41.4),
        (WB_INT_LEFT, 30.0),
        (NB_RAMP_LEFT, 30.0),
        (NB_RAMP_RIGHT, 30.0),
        (SB_RAMP_LEFT, 30.0),
        (SB_RAMP_RIGHT, 30.0),
    ] {
        assert_near!(
            group(&ix, mv).effective_green_s.unwrap(),
            g_pub,
            0.05,
            format!("g {mv:?}")
        );
    }

    // Control delays, Exhibits 34-55 and 34-56, for the eight lane groups
    // that are not an external approach (±0.4 s/veh). The two external
    // approaches are handled in
    // `test_ep4_diamond_demand_starvation_external_capacity_defect`.
    for (mv, d_pub) in [
        (EB_INT_THROUGH, 13.5),
        (EB_INT_LEFT, 32.3),
        (WB_INT_THROUGH, 16.0),
        (WB_INT_LEFT, 30.1),
        (NB_RAMP_LEFT, 28.0),
        (NB_RAMP_RIGHT, 31.2),
        (SB_RAMP_LEFT, 30.1),
        (SB_RAMP_RIGHT, 27.8),
    ] {
        assert_near!(
            group(&ix, mv).control_delay_s.unwrap(),
            d_pub,
            0.4,
            format!("d {mv:?}")
        );
    }
}

/// Chapter 34, Example Problem 4: the Equation 23-38 / 23-39 demand
/// starvation chain of Exhibit 34-52, driven directly through the free
/// functions so the published intermediates are pinned independently of
/// the pipeline.
#[test]
fn test_ep4_demand_starvation_intermediates() {
    const C: f64 = 100.0;
    const T_L: f64 = 5.0; // l_1 + Y - e = 2 + 5 - 2

    // (label, v_ramp-L, v_arterial, CG_RD, CG_UD, CG_DS, internal s,
    //  published H_I, published Q_initial, published L_DS).
    // CG_RD is zero in Exhibit 34-50 for both directions and enters the
    // Equation 23-39 brackets as t_L, which is the 5 s Exhibit 34-52
    // prints.
    for (what, v_ramp, v_art, cg_rd, cg_ud, cg_ds, s_int, h_pub, q_pub, lds_pub) in [
        ("EB-INT-TH", 191.0, 1_134.0, 0.0, 25.0, 30.0, 4_834.0, 2.23_f64, 6.8, 14.7),
        ("WB-INT-TH", 129.0, 1_119.0, 0.0, 30.0, 25.0, 4_799.0, 2.25, 2.8, 18.6),
    ] {
        let h_i = 3_600.0 / (s_int / 3.0);
        assert_near!(h_i, h_pub, 0.005, format!("H_I {what}"));
        // ±0.06 veh: Exhibit 34-52 divides by the H_I it prints, rounded
        // to two decimals, which moves Q_initial by up to 0.02 veh before
        // the exhibit rounds it again to one decimal.
        let q = demand_starvation_initial_queue(v_ramp, 1, v_art, 3, C, cg_rd, cg_ud, T_L, h_i);
        assert_near!(q, q_pub, 0.06, format!("Q_initial {what}"));
        assert_near!(
            demand_starvation_lost_time(cg_ds, q, h_i),
            lds_pub,
            0.05,
            format!("L_DS {what}")
        );
    }
}

/// Chapter 34, Example Problem 4: the published external-approach
/// capacities of Exhibits 34-53 and 34-55 are not reproducible from the
/// published saturation flows and effective greens.
///
/// Equation 23-48 is c = s g/C. Exhibit 34-48 publishes s = 4,597 veh/h
/// (EB) and 4,428 veh/h (WB), Exhibit 34-51 publishes g' = 25 s and 30 s,
/// and the cycle is 100 s, which gives 1,149 and 1,328 veh/h. Exhibits
/// 34-53 and 34-55 print 1,198 and 1,383 veh/h. Both published values are
/// recovered exactly by dividing by 96 s instead of the 100 s cycle
/// (4,597 x 25/96 = 1,197 and 4,428 x 30/96 = 1,384), and every other
/// lane group in the same exhibits uses 100 s, so the defect is confined
/// to these two cells. The engine follows the equation.
///
/// The knock-on effect is a higher v/c on both external approaches (1.085
/// and 0.982 against the published 1.04 and 0.94), which inflates the
/// Equation 19-26 incremental delay and every O-D that traverses an
/// external approach.
#[test]
fn test_ep4_diamond_demand_starvation_external_capacity_defect() {
    use movements::*;
    let mut ix = load_case("case4.json");
    ix.analyze();

    for (mv, s_pub, g_pub, c_equation, c_published, x_equation) in [
        (EB_EXT_THROUGH, 4_597.0, 25.0, 1_149.3, 1_198.0, 1.085),
        (WB_EXT_THROUGH, 4_428.0, 30.0, 1_328.4, 1_383.0, 0.982),
    ] {
        let r = group(&ix, mv);
        assert_near!(r.capacity.unwrap(), c_equation, 1.0, format!("c {mv:?}"));
        assert_near!(r.vc_ratio.unwrap(), x_equation, 0.005, format!("X {mv:?}"));
        // The equation value is what s g/C gives, and the published value
        // is what s g/96 gives.
        assert_near!(s_pub * g_pub / 100.0, c_equation, 1.0, format!("s g/C {mv:?}"));
        assert_near!(s_pub * g_pub / 96.0, c_published, 1.5, format!("s g/96 {mv:?}"));
    }
}

/// Chapter 34, Example Problem 4 (diamond with demand starvation): O-D
/// results against Exhibit 34-57.
#[test]
fn test_ep4_diamond_demand_starvation_od_results() {
    use LevelOfService as L;
    use OdMovement::*;
    let mut ix = load_case("case4.json");
    ix.analyze();

    // O-Ds that stay inside the interchange without using an external
    // approach reproduce the published Exhibit 34-57 ETT to ±0.4 s/veh.
    for (m, demand, ett, los) in [
        (A, 129.0, 45.5, L::C),
        (B, 216.0, 29.6, L::B),
        (C, 124.0, 26.2, L::B),
        (D, 191.0, 45.2, L::C),
    ] {
        let r = od(&ix, m);
        assert_near!(r.demand, demand, 1.0, format!("demand {m:?}"));
        assert_near!(r.ett_s, ett, 0.4, format!("ETT {m:?}"));
        assert_eq!(r.los, los, "LOS {m:?} (ETT {})", r.ett_s);
    }

    // The six O-Ds that traverse an external approach inherit the
    // capacity defect documented in
    // `test_ep4_diamond_demand_starvation_external_capacity_defect`.
    // Column 1: equation-based expectation (asserted); column 2 in the
    // comment: published Exhibit 34-57 value. All six moved 18 to 20 s/veh
    // closer to the published values under the Equation 19-26 lane group
    // capacity, which is most of what the "external capacity defect" was
    // costing them. Every LOS letter now matches the published one except
    // O-D G, which crosses the Exhibit 23-10 C/D boundary at 55 s/veh.
    let expected = [
        (E, 206.0, 124.3, L::F), // published 121.5 F
        (F, 113.0, 88.8, L::F),  // published  86.0 F
        (G, 186.0, 53.9, L::C),  // published  56.3 D
        (H, 294.0, 87.0, L::E),  // published  89.6 E
        (I, 928.0, 104.0, L::F), // published 101.1 F
        (J, 825.0, 71.4, L::D),  // published  73.9 D
    ];
    for (m, demand, ett, los) in expected {
        let r = od(&ix, m);
        assert_near!(r.demand, demand, 1.0, format!("demand {m:?}"));
        assert_near!(r.ett_s, ett, 0.5, format!("ETT {m:?}"));
        assert_eq!(r.los, los, "LOS {m:?} (ETT {})", r.ett_s);
    }

    // Exhibit 34-57 flags v/c > 1 for O-Ds E, F, and I (all of which use
    // the EB external approach) and no R_Q anywhere.
    for m in [E, F, I] {
        assert!(od(&ix, m).vc_exceeds_one, "{m:?} v/c flag");
    }
    for m in [A, B, C, D, G, H, J] {
        assert!(!od(&ix, m).vc_exceeds_one, "{m:?} v/c flag");
    }
    for m in [A, B, C, D, E, F, G, H, I, J] {
        assert!(!od(&ix, m).rq_exceeds_one, "{m:?} R_Q flag");
    }

    // Interchange ETT 78.1 s/veh against the published 78.0 (Exhibit 34-57
    // totals row), LOS D matching the published letter. Both were F and
    // 92.4 s/veh before the Equation 19-26 and Step 9 corrections.
    assert_near!(ix.interchange_ett_s.unwrap(), 78.1, 0.5, "interchange ETT");
    assert_eq!(ix.interchange_los.unwrap(), L::D, "interchange LOS");
}

/// Chapter 34, Example Problem 6 (DDI with YIELD control): the Step 6
/// three-regime capacity chain against Exhibits 34-67, 34-68, 34-69, and
/// the v/c row of Exhibit 34-70.
///
/// This is the part of the example the engine reproduces. The fixture only
/// swaps the four off-ramp lane groups of Example Problem 5 from
/// `Signalized` to `YieldControlled`, so a passing v/c row also confirms
/// that the pipeline routes YIELD groups around the signalized capacity of
/// Equation 23-48.
#[test]
fn test_ep6_ddi_yield_control_capacity() {
    use movements::*;
    let mut ix = load_case("case5.json");
    ix.analyze();

    // The four signalized crossover movements are untouched by the YIELD
    // conversion (the example states Steps 1 through 5 are unchanged), so
    // they still carry the Example Problem 5 saturation flows.
    for (mv, s_pub, tol) in [
        (EB_EXT_THROUGH, 3_563.0, 55.0),
        (WB_EXT_THROUGH, 2_045.0, 5.0),
        (EB_INT_THROUGH, 3_229.0, 5.0),
        (WB_INT_THROUGH, 3_156.0, 5.0),
    ] {
        assert_near!(group(&ix, mv).sat_flow.unwrap(), s_pub, tol, format!("s {mv:?}"));
    }

    // Exhibit 34-70 v/c row: 0.38 (M7), 0.16 (M8), 0.35 (M3), 0.19 (M4).
    // The exhibit prints two decimals, so ±0.005 is agreement to the last
    // printed digit. Reproducing all four pins the whole Step 6 chain at
    // once, because each v/c is the movement demand over the Equation 23-47
    // combined capacity, which in turn consumes the Exhibit 34-67 blocked
    // regime (Equations 23-53 / 23-54 / 23-56), the Exhibit 34-68 gap
    // acceptance regime (Equations 23-42 / 23-43), and the Exhibit 34-69
    // no-opposing-flow regime (Equations 23-44 / 23-45).
    for (mv, v_pub, x_pub) in [
        (SB_RAMP_LEFT, 300.0, 0.38),   // M7
        (SB_RAMP_RIGHT, 200.0, 0.16),  // M8
        (NB_RAMP_LEFT, 350.0, 0.35),   // M3
        (NB_RAMP_RIGHT, 200.0, 0.19),  // M4
    ] {
        let r = group(&ix, mv);
        assert_near!(r.flow_rate, v_pub, 0.5, format!("v {mv:?}"));
        assert_near!(r.vc_ratio.unwrap(), x_pub, 0.005, format!("X {mv:?}"));
        // A YIELD group is not evaluated with a signalized capacity, so
        // the Step 6 capacity must differ from s g'/C.
        let signalized = r.sat_flow.unwrap() * r.effective_green_s.unwrap() / 70.0;
        assert!(
            (r.capacity.unwrap() - signalized).abs() > 1.0,
            "{mv:?} capacity {} coincides with the signalized s g/C {signalized}",
            r.capacity.unwrap()
        );
    }

    // M3's gap acceptance window closes entirely: Exhibit 34-68 notes that
    // p_GA for M3 came out negative and was set to zero, which in Equation
    // 23-47 means the 20-s conflicting green contributes nothing and the
    // capacity is carried by the no-opposing-flow regime alone. Exhibit
    // 34-69 publishes c_NOF = 1,385 veh/h over C − g = 50 s of the 70-s
    // cycle, so 1,385 x 50/70 = 989 veh/h.
    assert_near!(group(&ix, NB_RAMP_LEFT).capacity.unwrap(), 989.0, 7.0, "M3 c_YCT");
}

/// Chapter 34, Example Problem 6: the Exhibit 34-70 control delays are not
/// reproducible from Equation 22-17 at the capacities the same exhibit's
/// v/c row confirms.
///
/// Chapter 34 states that the control delay of a YIELD-controlled turn is
/// estimated "by using the control delay procedure for roundabouts given in
/// Equation 22-17", and `Interchange::step_8_control_delay`
/// (src/hcm/ramp_terminals/ramp_terminals.rs:1919) does exactly that. The
/// published delays are 3.1 to 3.8 times larger. Solving Equation 22-17
/// backwards for the capacity that would produce each published delay gives
/// 0.4 to 0.5 of the Equation 23-47 capacity — and that same Equation 23-47
/// capacity is what the exhibit's own v/c row reports, so the two rows of
/// Exhibit 34-70 disagree with each other. The engine follows the equation.
#[test]
fn test_ep6_ddi_yield_control_delay_defect() {
    use movements::*;
    let mut ix = load_case("case5.json");
    ix.analyze();

    // Equation 22-17 is monotone decreasing in capacity, so bisection
    // recovers the capacity implied by a published delay.
    let implied_capacity = |v: f64, d_target: f64| -> f64 {
        let (mut lo, mut hi) = (v * 1.001, 10_000.0);
        for _ in 0..200 {
            let mid = 0.5 * (lo + hi);
            if control_delay_roundabout(v, mid, 0.25) > d_target {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        0.5 * (lo + hi)
    };

    for (mv, v, d_published, d_engine) in [
        (SB_RAMP_LEFT, 300.0, 34.7, 9.11),   // M7
        (SB_RAMP_RIGHT, 200.0, 13.4, 4.28),  // M8
        (NB_RAMP_LEFT, 350.0, 31.0, 7.32),   // M3
        (NB_RAMP_RIGHT, 200.0, 16.3, 5.24),  // M4
    ] {
        let r = group(&ix, mv);
        let cap = r.capacity.unwrap();
        assert_near!(r.control_delay_s.unwrap(), d_engine, 0.05, format!("d {mv:?}"));
        // The engine value is Equation 22-17 evaluated at the Step 6
        // capacity, with nothing else added.
        assert_near!(
            r.control_delay_s.unwrap(),
            control_delay_roundabout(v, cap, 0.25),
            1e-9,
            format!("d {mv:?} is Equation 22-17 at c_YCT")
        );
        // The published delay needs roughly half that capacity.
        let ratio = implied_capacity(v, d_published) / cap;
        assert!(
            (0.35..0.55).contains(&ratio),
            "{mv:?}: published {d_published} s/veh implies c = {:.0} veh/h against the \
             Exhibit 34-70 v/c capacity {cap:.0} veh/h (ratio {ratio:.2})",
            implied_capacity(v, d_published)
        );
    }
}

/// Chapter 34, Example Problem 6 (DDI with YIELD control): O-D results
/// against Exhibit 34-71.
///
/// Exhibit 34-71 has to be read against itself. Its Control Delay, ETT, and
/// LOS columns for O-Ds A through D are reprinted unchanged from the
/// Example Problem 5 table (Exhibit 34-65) and still carry the signalized
/// delays, but its Demand x ETT column and its totals row were recomputed
/// with the YIELD delays of Exhibit 34-70. Two independent checks say the
/// products column is the live one: dividing each product by its demand
/// recovers exactly the Exhibit 34-70 YIELD delays plus the stated EDTT
/// (O-D A = M3 + M5 = 31.0 + 17.2, +1.9 EDTT = 50.1, and 368 x 50.1 =
/// 18,436), and the published interchange total 117,917 / 3,476 = 33.9
/// s/veh matches the printed totals row while the stale ETT column would
/// give the Example Problem 5 value of 34.9. The recomputed values are used
/// below.
#[test]
fn test_ep6_ddi_yield_control_od_results() {
    use LevelOfService as L;
    use OdMovement::*;
    let mut ix = load_case("case5.json");
    ix.analyze();

    // (O-D, demand, engine ETT, engine LOS); the comment carries the
    // journey and the recomputed published ETT. The engine runs short on
    // every O-D, and on two separate counts: O-Ds A through D carry the
    // YIELD delay gap of `test_ep6_ddi_yield_control_delay_defect`, while
    // every O-D that uses a signalized crossover also carries the Exhibit
    // 34-64 movement delay gap already documented for Example Problem 5.
    // O-D E is the exception that isolates the split, running only on the
    // eastbound external crossover and reproducing its published 24.7 s/veh
    // exactly, as it does in the Example Problem 5 fixture.
    let expected = [
        (A, 350.0, 27.22, L::B), // M3 + M5, published 50.1
        (B, 200.0, 3.30, L::A),  // M4,      published 14.4
        (C, 200.0, 2.34, L::A),  // M8,      published 11.5
        (D, 300.0, 24.52, L::B), // M7 + M1, published 58.5
        (E, 600.0, 24.67, L::B), // M6,      published 24.7
        (F, 200.0, 0.0, L::A),   // free-flow bypass
        (G, 300.0, 0.0, L::A),   // free-flow bypass
        (H, 300.0, 31.51, L::C), // M2,      published 50.3
        (I, 700.0, 36.96, L::C), // M6 + M1, published 45.5
        (J, 150.0, 48.30, L::C), // M2 + M5, published 66.4
    ];
    for (m, demand, ett, los) in expected {
        let r = od(&ix, m);
        assert_near!(r.demand, demand, 0.5, format!("demand {m:?}"));
        assert_near!(r.ett_s, ett, 0.5, format!("ETT {m:?}"));
        assert_eq!(r.los, los, "LOS {m:?} (ETT {})", r.ett_s);
    }

    // The recomputed published table is internally consistent: the O-D
    // demands are those of Example Problem 5 (the YIELD conversion changes
    // no demand), and weighting the recomputed ETTs by them reproduces the
    // published totals row of Exhibit 34-71 (117,917 veh-s/h over 3,476
    // veh/h, 33.9 s/veh, LOS C).
    let published_demands = [368.0, 211.0, 211.0, 316.0, 632.0, 211.0, 316.0, 316.0, 737.0, 158.0];
    let published_etts = [50.1, 14.4, 11.5, 58.5, 24.7, 0.0, 0.0, 50.3, 45.5, 66.4];
    let products: f64 = published_demands
        .iter()
        .zip(published_etts)
        .map(|(v, ett)| v * ett)
        .sum();
    let total_demand: f64 = published_demands.iter().sum();
    assert_near!(products, 117_917.0, 5.0, "Exhibit 34-71 demand x ETT total");
    assert_near!(total_demand, 3_476.0, 0.5, "Exhibit 34-71 demand total");
    assert_near!(products / total_demand, 33.9, 0.05, "Exhibit 34-71 interchange ETT");

    // The engine's own interchange result, carrying both delay gaps.
    assert_near!(ix.interchange_ett_s.unwrap(), 22.8, 0.5, "interchange ETT");
    assert_eq!(ix.interchange_los.unwrap(), L::B);
}

/// Chapter 34, Example Problem 8 (diamond with an adjacent closely spaced
/// intersection): the Exhibit 34-89 common greens and the Exhibit 34-90
/// queue lengths and additional lost times.
///
/// The full example is not assembled as an `Interchange` (see the module
/// notes on the missing third intersection), but its Step 4 coupling terms
/// depend only on green windows, feeding flows, and lane counts, so they
/// exercise the same free functions the diamond fixtures use. I-99 at
/// University Drive with Spring Street 300 ft to the west, C = 160 s,
/// L_h = 25.006 ft.
#[test]
fn test_ep8_adjacent_intersection_queues_and_lost_time() {
    const C: f64 = 160.0;
    const D_INTERCHANGE_FT: f64 = 500.0;
    const D_ADJACENT_FT: f64 = 300.0;
    const L_H: f64 = 25.006;

    // Green windows as Exhibit 34-89 prints them in its movement rows, not
    // as derived from its phase table: the phase table runs Intersection I
    // at 0-63 / 68-111 / 116-155, Intersection II at 150-53 (wrapping the
    // cycle) / 58-111 / 116-145, and the adjacent intersection at 0-33 /
    // 38-62 / 67-96 / 96-155, but the movement rows fold change intervals
    // and consecutive phases into the windows below (the two adjacent
    // through movements are printed as one 38-97 window spanning that
    // intersection's Phases 2 and 3). The movement rows are what the
    // exhibit's own common green column is computed from. The eastbound
    // internal through is the only movement green twice per cycle.
    let eb_ext = [GreenInterval { begin_s: 0.0, duration_s: 63.0 }];
    let eb_int = [
        GreenInterval { begin_s: 150.0, duration_s: 63.0 },
        GreenInterval { begin_s: 116.0, duration_s: 34.0 },
    ];
    let wb_ext = [GreenInterval { begin_s: 150.0, duration_s: 63.0 }];
    let wb_int = [GreenInterval { begin_s: 0.0, duration_s: 111.0 }];
    let wb_int_left = [GreenInterval { begin_s: 68.0, duration_s: 43.0 }];
    let eb_int_left = [GreenInterval { begin_s: 116.0, duration_s: 29.0 }];
    let sb_ramp = [GreenInterval { begin_s: 116.0, duration_s: 39.0 }];
    let nb_ramp = [GreenInterval { begin_s: 58.0, duration_s: 53.0 }];
    let adj_eb = [GreenInterval { begin_s: 38.0, duration_s: 59.0 }];
    let adj_wb = [GreenInterval { begin_s: 38.0, duration_s: 59.0 }];
    let adj_sb_left = [GreenInterval { begin_s: 102.0, duration_s: 24.0 }];
    let adj_nb_right = [GreenInterval { begin_s: 131.0, duration_s: 24.0 }];

    // Exhibit 34-89 common green column, in the order the exhibit prints it.
    // The SB RAMP row is handled separately below.
    for (what, a, b, cg_pub) in [
        ("EB EXT-TH / EB INT-TH", &eb_ext[..], &eb_int[..], 53.0),
        ("WB EXT-TH / WB INT-TH", &wb_ext[..], &wb_int[..], 53.0),
        ("NB RAMP / WB INT-TH", &nb_ramp[..], &wb_int[..], 53.0),
        ("WB INT-L / EB INT-TH", &wb_int_left[..], &eb_int[..], 0.0),
        ("EB INT-L / WB INT-TH", &eb_int_left[..], &wb_int[..], 0.0),
        ("EB EXT-TH / ADJ EB-TH", &eb_ext[..], &adj_eb[..], 25.0),
        ("EB EXT-TH / ADJ SB-L", &eb_ext[..], &adj_sb_left[..], 0.0),
        ("EB EXT-TH / ADJ NB-R", &eb_ext[..], &adj_nb_right[..], 0.0),
        ("ADJ WB-TH / WB INT-TH", &adj_wb[..], &wb_int[..], 59.0),
        ("ADJ WB-TH / SB RAMP", &adj_wb[..], &sb_ramp[..], 0.0),
    ] {
        assert_near!(common_green_time(a, b, C), cg_pub, 1e-9, format!("CG {what}"));
    }

    // The SB RAMP / EB INT-TH row is the one that pins the phase-pair
    // scope of `common_green_time`, because the eastbound internal through
    // is the only movement in these examples that receives green twice per
    // cycle. Its two windows, 150-53 and 116-150, are contiguous, so
    // unioning them first would make the movement green continuously from
    // 116 to 53 and the overlap with the 116-155 ramp phase would be the
    // whole 39-s ramp green. Both Exhibit 34-89 here and Exhibit 34-9 in
    // Example Problem 1 print 34 s, the overlap with the 116-150 window
    // alone, and Example Problem 1 corroborates that downstream: Exhibit
    // 34-10 publishes a 4.1-ft SB-L queue, which Equation 23-34 returns at
    // CG = 34 s and not at 39 s.
    assert_near!(
        common_green_time(&sb_ramp, &eb_int, C),
        34.0,
        1e-9,
        "CG SB RAMP / EB INT-TH"
    );

    // Exhibit 34-90, upper block: (label, feeding flow, feeding lanes,
    // feeding green, downstream green, common green, published queue). The
    // first four rows are the interchange's own approaches, which all clear
    // their 500-ft internal links; the last five are the approaches coupled
    // to the 300-ft link toward the adjacent intersection.
    for (what, v_feed, n_feed, g_feed, g_down, cg, q_pub) in [
        ("EB EXT-TH", 191.0, 1u32, 39.0, 97.0, 53.0, 0.0),
        ("SB-L", 805.0, 2, 63.0, 97.0, 34.0, 0.0),
        ("WB EXT-TH", 216.0, 1, 53.0, 111.0, 53.0, 0.0),
        ("NB-L", 822.0, 2, 63.0, 111.0, 53.0, 0.0),
        ("ADJ EB-TH", 474.0, 1, 48.0, 63.0, 25.0, 56.9),
        ("ADJ SB-L", 804.0, 2, 59.0, 63.0, 0.0, 102.6),
        ("ADJ NB-R", 804.0, 2, 59.0, 63.0, 0.0, 102.6),
        ("WB INT-TH", 156.0, 1, 39.0, 59.0, 15.0, 0.0),
        ("SB-R", 795.0, 2, 111.0, 59.0, 39.0, 91.1),
    ] {
        // ±0.15 ft: the exhibit prints the queue to one decimal.
        let q = downstream_queue_length_ft(v_feed, n_feed, g_feed, g_down, cg, C, L_H);
        assert_near!(q, q_pub, 0.15, format!("Q {what}"));
    }

    // Exhibit 34-90, lower block: the additional lost time uses the subject
    // approach's own green and the distance to the back of the downstream
    // queue. Only the two adjacent-intersection movements whose downstream
    // storage falls below the 200-ft threshold lose time, and they are the
    // headline numbers of this example (2.10 and 3.07 s).
    for (what, g_subject, d_ft, q_ft, cg, ld_pub) in [
        ("EB EXT-TH", 63.0, D_INTERCHANGE_FT, 0.0, 53.0, 0.0),
        ("SB-L", 39.0, D_INTERCHANGE_FT, 0.0, 34.0, 0.0),
        ("WB EXT-TH", 63.0, D_INTERCHANGE_FT, 0.0, 53.0, 0.0),
        ("NB-L", 53.0, D_INTERCHANGE_FT, 0.0, 53.0, 0.0),
        ("ADJ EB-TH", 59.0, D_ADJACENT_FT, 56.9, 25.0, 0.0),
        ("ADJ SB-L", 24.0, D_ADJACENT_FT, 102.6, 29.0, 2.10),
        ("ADJ NB-R", 24.0, D_ADJACENT_FT, 102.6, 0.0, 3.07),
        ("WB INT-TH", 119.0, D_ADJACENT_FT, 0.0, 15.0, 0.0),
        ("SB-R", 39.0, D_ADJACENT_FT, 91.1, 39.0, 0.0),
    ] {
        // ±0.05 s: the exhibit prints the DQ column to whole feet, which
        // moves Equation 23-40 by up to 0.05 s through the 0.106 DQ term.
        assert_near!(
            adjacent_intersection_lost_time(g_subject, d_ft - q_ft, cg, C),
            ld_pub,
            0.05,
            format!("L_D {what}")
        );
    }

    // Equation 23-40 is the Equation 23-29 form applied at the adjacent
    // intersection, and `adjacent_intersection_lost_time` is a thin alias,
    // so the two must not diverge.
    assert_near!(
        adjacent_intersection_lost_time(24.0, D_ADJACENT_FT - 102.6, 29.0, C),
        downstream_queue_lost_time(24.0, D_ADJACENT_FT - 102.6, 29.0, C),
        1e-12,
        "Equation 23-40 alias"
    );
}

/// Chapter 34, Example Problem 9 (diamond interchange with roundabouts):
/// the published Exhibit 34-101 capacities and delays come from superseded
/// Chapter 22 equations.
///
/// Chapter 23 evaluates a roundabout ramp terminal by handing each approach
/// to Chapter 22. Two things then have to hold, and neither does. First,
/// every published capacity is recovered by `1,130 e^(-1.0e-3 v_c)`, the
/// HCM 2010 single-lane entry model, not by the HCM 7 Equation 22-1
/// `1,380 e^(-1.02e-3 v_c)` that `capacity_single_lane`
/// (src/hcm/roundabouts/roundabouts.rs:51) implements. Second, every
/// published delay is recovered by Equation 22-17 evaluated at those
/// capacities *minus* its `5 min(x, 1)` term, which is the term Chapter 22
/// carries for the geometric delay of yielding. Both readings hold across
/// all six approaches, so neither is a rounding artifact.
#[test]
fn test_ep9_roundabout_terminals_use_superseded_chapter_22_equations() {
    // (approach, entering flow pc/h, conflicting flow pc/h, published
    //  capacity pc/h, published control delay s/veh) — Exhibit 34-101.
    for (what, v_e, v_c, c_pub, d_pub) in EP9_APPROACHES {
        // ±1 pc/h: the exhibit prints whole pc/h.
        assert_near!(
            capacity_exponential(1_130.0, 1.0e-3, v_c),
            c_pub,
            1.0,
            format!("HCM 2010 capacity {what}")
        );
        // The HCM 7 equation is 17 to 22% higher on every approach.
        let c7 = capacity_single_lane(v_c);
        assert!(
            c7 > c_pub * 1.15,
            "{what}: HCM 7 capacity {c7:.0} should exceed the published {c_pub}"
        );

        // ±0.15 s/veh: the exhibit prints one decimal and rounds its
        // capacities to whole pc/h first.
        let x: f64 = v_e / c_pub;
        let d_no_geometric = control_delay_roundabout(v_e, c_pub, 0.25) - 5.0 * x.min(1.0);
        assert_near!(d_no_geometric, d_pub, 0.15, format!("d without 5 min(x,1) {what}"));
        // With the term, the published delay is missed by 5 x on the nose.
        assert_near!(
            control_delay_roundabout(v_e, c_pub, 0.25) - d_pub,
            5.0 * x.min(1.0),
            0.15,
            format!("geometric term {what}")
        );
    }
}

/// Chapter 34, Example Problem 9: O-D assembly and interchange LOS.
///
/// The Chapter 23 roundabout ramp terminal wraps Chapter 22 rather than
/// extending the signalized pipeline, and the piece that would let
/// `Interchange` drive it — the Exhibit 34-160 / 34-161 worksheet that maps
/// the O-D demands onto each roundabout's entering and conflicting flows —
/// is not implemented (`InterchangeForm`,
/// src/hcm/ramp_terminals/ramp_terminals.rs:89-106, has no roundabout
/// variant, and `Roundabouts`, src/hcm/roundabouts/roundabouts.rs:340,
/// takes per-leg turning volumes rather than interchange O-Ds). The
/// published entering and conflicting flows are therefore taken as given
/// here and everything downstream of them is computed: Chapter 22 capacity,
/// Chapter 22 delay, the Step 8 O-D journeys, Equation 23-50 EDTT, and the
/// Exhibit 23-14 LOS table.
#[test]
fn test_ep9_diamond_with_roundabouts_od_results() {
    use LevelOfService as L;

    let delay = |approach: &str| -> f64 {
        let (_, v_e, v_c, _, _) = EP9_APPROACHES
            .iter()
            .find(|(a, ..)| *a == approach)
            .unwrap_or_else(|| panic!("approach {approach}"));
        control_delay_roundabout(*v_e, capacity_single_lane(*v_c), 0.25)
    };

    // (O-D, heavy-vehicle-adjusted demand, approaches traversed, extra
    //  distance ft, engine ETT, engine LOS, published ETT, published LOS).
    // The demands are the Exhibit 34-100 heavy-vehicle-adjusted column. The
    // Exhibit 34-102 demand column prints the Example Problem 7 O-D demands
    // instead (174 / 168 / 126 / 547 / 177 / 84 / 221 / 194 / 911 / 881
    // veh/h), but its own Demand x ETT products divide out to the Exhibit
    // 34-100 values and its totals row prints 2,252 veh/h, which is the
    // Exhibit 34-100 total and not the 3,483 veh/h of Example Problem 7.
    struct Ep9Od {
        movement: &'static str,
        demand: f64,
        path: &'static [&'static str],
        distance_ft: f64,
        ett_engine: f64,
        los_engine: LevelOfService,
        ett_published: f64,
        los_published: LevelOfService,
    }
    let ep9 = |movement, demand, path, distance_ft, ett_engine, los_engine, ett_published,
                los_published| Ep9Od {
        movement,
        demand,
        path,
        distance_ft,
        ett_engine,
        los_engine,
        ett_published,
        los_published,
    };
    let ods = [
        ep9("A", 191.0, &["NB RAMP", "WB INT"], 100.0, 33.44, L::C, 46.1, L::D),
        ep9("B", 179.0, &["NB RAMP"], -100.0, 19.31, L::B, 29.0, L::C),
        ep9("C", 130.0, &["SB RAMP"], -100.0, 19.33, L::B, 29.2, L::C),
        ep9("D", 242.0, &["SB RAMP", "EB INT"], 100.0, 33.52, L::C, 46.4, L::D),
        ep9("E", 99.0, &["EB EXT", "EB INT"], 100.0, 30.90, L::C, 49.8, L::D),
        ep9("F", 82.0, &["EB EXT"], -100.0, 16.72, L::B, 32.6, L::C),
        ep9("G", 100.0, &["WB EXT"], -100.0, 15.87, L::B, 31.9, L::C),
        ep9("H", 127.0, &["WB EXT", "WB INT"], 100.0, 30.00, L::C, 49.0, L::D),
        ep9("I", 541.0, &["EB EXT", "EB INT"], 0.0, 28.96, L::C, 47.9, L::D),
        ep9("J", 561.0, &["WB EXT", "WB INT"], 0.0, 28.06, L::C, 47.1, L::D),
    ];

    let (mut num, mut den, mut num_published) = (0.0, 0.0, 0.0);
    for Ep9Od {
        movement: m,
        demand,
        path,
        distance_ft,
        ett_engine,
        los_engine,
        ett_published,
        los_published,
    } in ods
    {
        // Chapter 23 Step 8 sums the approach delays along the journey.
        let d: f64 = path.iter().map(|a| delay(a)).sum();
        // Equation 23-50 with 100 ft of ramp travel at 35 mi/h: ±1.9 s.
        let edtt = extra_distance_travel_time(distance_ft, 35.0, 0.0);
        let ett = d + edtt;
        assert_near!(ett, ett_engine, 0.05, format!("ETT {m}"));
        assert_eq!(
            los_roundabout_interchange_od(ett, false, false),
            los_engine,
            "LOS {m} (ETT {ett})"
        );
        // The published ETT is the published approach delays summed with
        // the same EDTT, and it grades one letter worse on every O-D.
        assert_eq!(
            los_roundabout_interchange_od(ett_published, false, false),
            los_published,
            "published LOS {m}"
        );
        num += ett * demand;
        num_published += ett_published * demand;
        den += demand;
    }

    // Exhibit 34-102 totals row: 2,252 veh/h, 98,374.3 veh-s/h, 43.7 s/veh,
    // LOS D. Reproducing the products total from the Exhibit 34-100 demands
    // is what identifies the printed demand column as the stale one.
    assert_near!(den, 2_252.0, 0.5, "Exhibit 34-102 demand total");
    assert_near!(num_published, 98_374.3, 1.0, "Exhibit 34-102 demand x ETT total");
    assert_near!(num_published / den, 43.7, 0.05, "published interchange ETT");
    assert_eq!(
        los_roundabout_interchange_od(num_published / den, false, false),
        L::D,
        "published interchange LOS"
    );

    // The HCM 7 equations give 27.4 s/veh and LOS C for the same
    // interchange, the difference being the two Chapter 22 readings pinned
    // in `test_ep9_roundabout_terminals_use_superseded_chapter_22_equations`.
    assert_near!(num / den, 27.4, 0.05, "engine interchange ETT");
    assert_eq!(
        los_roundabout_interchange_od(num / den, false, false),
        L::C,
        "engine interchange LOS"
    );
}

/// Chapter 34 Exhibit 34-101: entering flow, conflicting flow, published
/// capacity, and published control delay for the six roundabout approaches
/// of Example Problem 9, all in pc/h except the delay in s/veh.
const EP9_APPROACHES: [(&str, f64, f64, f64, f64); 6] = [
    ("EB EXT", 722.0, 369.0, 782.0, 34.5),
    ("EB INT", 882.0, 0.0, 1_130.0, 13.4),
    ("WB EXT", 788.0, 289.0, 846.0, 33.8),
    ("WB INT", 879.0, 0.0, 1_130.0, 13.3),
    ("NB RAMP", 370.0, 882.0, 468.0, 30.9),
    ("SB RAMP", 372.0, 879.0, 469.0, 31.1),
];

/// Serialization round trip: the analyzed facility serializes and
/// deserializes with results intact.
#[test]
fn test_serde_round_trip() {
    let mut ix = load_case("case1.json");
    ix.analyze();
    let json = serde_json::to_string(&ix).unwrap();
    let back: Interchange = serde_json::from_str(&json).unwrap();
    assert_eq!(back.od_results.len(), ix.od_results.len());
    assert_near!(
        back.interchange_ett_s.unwrap(),
        ix.interchange_ett_s.unwrap(),
        1e-12,
        "ETT round trip"
    );
}

// ═══════════════════════════════════════════════════════════════════════════════
// Example Problem 7: single-point urban interchange (Exhibits 34-72 to 34-82)
// ═══════════════════════════════════════════════════════════════════════════════

/// Chapter 34, Example Problem 7 (`case7.json`): the SPUI routes as a single
/// signalized point, and the two arterial left turns operate with both a
/// protected and a permitted phase.
///
/// The single-point convention: `turning_movements_from_od` sends every SPUI
/// O-D through exactly one lane group, an `Ext` group for the arterial
/// approaches and a `Ramp` group for the off-ramps, so `od_path` returns a
/// one-element path for all ten O-Ds and the O-D delay is that movement's
/// delay unchanged. Exhibit 34-82 is what confirms it: its ETT column equals
/// the Exhibit 34-80 / 34-81 movement delays exactly, because a SPUI has one
/// intersection and no extra distance traveled.
#[test]
fn test_case7_spui_is_a_single_signalized_point() {
    let mut ix = load_case("case7.json");
    ix.analyze();
    for (m, want) in [
        (OdMovement::A, "NbRampLeft"),
        (OdMovement::B, "NbRampRight"),
        (OdMovement::C, "SbRampRight"),
        (OdMovement::D, "SbRampLeft"),
        (OdMovement::E, "EbExtLeft"),
        (OdMovement::F, "EbExtRight"),
        (OdMovement::G, "WbExtRight"),
        (OdMovement::H, "WbExtLeft"),
        (OdMovement::I, "EbExtThrough"),
        (OdMovement::J, "WbExtThrough"),
    ] {
        let path = ix.od_path(m);
        assert_eq!(path.len(), 1, "O-D {m:?} should traverse one lane group");
        assert_eq!(path[0].name(), want, "O-D {m:?} lane group");
        // Exhibit 34-82: ETT is the movement delay, with no EDTT.
        let o = od(&ix, m);
        assert_near!(o.edtt_s, 0.0, 1e-12, "EDTT for O-D {m:?}");
        assert_near!(o.ett_s, o.control_delay_s, 1e-12, "ETT = d for O-D {m:?}");
    }
}

/// Exhibit 34-75: the eastbound and westbound left turns are the only lane
/// groups with two phase components, and their recombination is capacity
/// addition.
///
/// The published pair is a protected saturation flow over the protected green
/// and a permitted saturation flow over the permitted green, and Exhibit
/// 34-78 collapses them into one saturation flow, one effective green, one
/// capacity and one v/c before Steps 6, 7 and 9 ever run. This test pins that
/// the engine does the same: the collapsed capacity equals the sum of the two
/// component capacities, and the collapsed effective green is the sum of the
/// two displayed greens (48 s, the published value).
#[test]
fn test_case7_protected_permitted_recombination_is_capacity_addition() {
    let mut ix = load_case("case7.json");
    ix.analyze();
    let c = ix.get_cycle_length();
    for m in [movements::EB_EXT_LEFT, movements::WB_EXT_LEFT] {
        let r = group(&ix, m);
        let g_prot = 16.0;
        let g_perm = 32.0;
        let g_u = if m == movements::EB_EXT_LEFT { 13.01 } else { 11.78 };
        let s_prot = r.protected_sat_flow.expect("protected component");
        let s_perm = r.permitted_sat_flow.expect("permitted component");
        // Exhibit 34-80 publishes g' = 48.0 s for both left turns.
        assert_near!(r.effective_green_s.unwrap(), 48.0, 1e-9, "{m:?} g'");
        // Capacity addition (Equation 31-124 without its sneaker term, which
        // Chapter 34 omits; see `test_ep7_capacity_omits_the_sneaker_term`).
        let want_c = (s_prot * g_prot + s_perm * g_u) / c;
        assert_near!(r.capacity.unwrap(), want_c, 1e-9, "{m:?} capacity");
        // ... re-expressed over the summed green, which is the single
        // saturation flow Exhibit 34-78 reports.
        let want_s = (s_prot * g_prot + s_perm * g_u) / (g_prot + g_perm);
        assert_near!(r.sat_flow.unwrap(), want_s, 1e-9, "{m:?} recombined s");
    }
    // Every other lane group stays single-component.
    for r in ix.get_results() {
        if r.movement == movements::EB_EXT_LEFT || r.movement == movements::WB_EXT_LEFT {
            continue;
        }
        assert!(
            r.protected_sat_flow.is_none() && r.permitted_sat_flow.is_none(),
            "{:?} should carry no phase components",
            r.movement
        );
    }
}

/// Exhibit 34-77 against the Chapter 31 Exhibit 31-15 queue accumulation
/// polygon: the uniform delay of a leading protected-plus-permitted left turn
/// in an exclusive lane.
///
/// This drives `qap_evaluate` with the *published* Exhibit 34-75 component
/// saturation flows rather than with the engine's own, so it validates the
/// polygon shape chosen in Step 8 independently of the Chapter 23 saturation
/// flow chain. The permitted discharge rate is the Equation 31-100
/// gap-acceptance rate `s_l`, recovered from the published permitted
/// saturation flow by undoing the `g_u / G_perm` proration that Exhibit
/// 34-75's permitted f_LT embeds (Chapter 19: for a permitted movement the
/// tabulated saturation flow "is an average for the permitted green period").
///
/// Both published uniform delays reproduce, and doing so resolves a
/// contradiction inside the example: Exhibit 34-77 prints 22.7 s/veh for the
/// westbound left while Exhibit 34-80 prints 22.8 for the same movement, and
/// 22.8 is the live one (it is what Exhibit 34-80's own d = d1 + d2 + d3 =
/// 22.8 + 11.8 = 34.6 adds up to).
#[test]
fn test_ep7_uniform_delay_polygon_reproduces_exhibit_34_77() {
    let cycle = 110.0;
    let g_prot = 16.0;
    let g_perm = 32.0;
    for (name, v, s_prot, s_perm_avg, g_u, published_d1) in [
        // v = Exhibit 34-78 demand; s_prot / s_perm_avg = Exhibit 34-75.
        ("eastbound left", 177.0, 1_560.0, 228.0, 13.01, 22.1),
        ("westbound left", 194.0, 1_561.0, 211.0, 11.78, 22.8),
    ] {
        let q = v / 3_600.0;
        let s_l = s_perm_avg * g_perm / g_u; // undo the g_u / G_perm proration
        let intervals = [
            QapInterval {
                duration_s: cycle - g_prot - g_perm,
                discharge_veh_h: 0.0,
                arrival_veh_s: q,
                sneakers_veh: 0.0,
            },
            QapInterval {
                duration_s: g_prot,
                discharge_veh_h: s_prot,
                arrival_veh_s: q,
                sneakers_veh: 0.0,
            },
            QapInterval {
                duration_s: g_perm - g_u,
                discharge_veh_h: 0.0,
                arrival_veh_s: q,
                sneakers_veh: 0.0,
            },
            QapInterval {
                duration_s: g_u,
                discharge_veh_h: s_l,
                arrival_veh_s: q,
                sneakers_veh: 0.0,
            },
        ];
        let got = qap_evaluate(&intervals, cycle, q);
        assert_near!(got.uniform_delay_s, published_d1, 0.05, "{name} d1");
        // Exhibit 34-77's breakpoint queues, which it mislabels "ft": they
        // are vehicles (Chapter 31 defines every polygon breakpoint queue in
        // veh). Q_A = q_a r is the queue at the end of effective red.
        let q_a = q * (cycle - g_prot - g_perm);
        assert!(
            (3.0..=3.4).contains(&q_a),
            "{name} Q_A {q_a} should match the published 3.0 / 3.3 veh"
        );
    }
}

/// Example Problem 7 O-D results against Exhibit 34-82.
///
/// Asserted at the equation-based values with the published ones inline,
/// because Exhibits 34-75 and 34-76 are not reproducible from the HCM 7
/// equations (see `test_ep7_saturation_flow_factor_defects`). Eight of the
/// ten O-D LOS letters and the interchange LOS still land exactly.
#[test]
fn test_ep7_spui_od_results() {
    let mut ix = load_case("case7.json");
    ix.analyze();
    // (O-D, demand, engine ETT, published Exhibit 34-82 ETT, published LOS)
    let published = [
        (OdMovement::A, 174.0, 27.68, 27.9, LevelOfService::B),
        (OdMovement::B, 168.0, 64.01, 63.6, LevelOfService::D),
        (OdMovement::C, 126.0, 53.17, 53.0, LevelOfService::C),
        (OdMovement::D, 547.0, 50.77, 56.0, LevelOfService::D),
        (OdMovement::E, 177.0, 29.18, 31.0, LevelOfService::C),
        (OdMovement::F, 84.0, 25.54, 25.4, LevelOfService::B),
        (OdMovement::G, 221.0, 29.60, 29.1, LevelOfService::B),
        (OdMovement::H, 194.0, 32.33, 34.6, LevelOfService::C),
        (OdMovement::I, 911.0, 50.27, 54.6, LevelOfService::C),
        (OdMovement::J, 881.0, 47.64, 51.0, LevelOfService::C),
    ];
    // The two O-Ds whose LOS letter differs from the published one are the
    // two that sit closest to an Exhibit 23-10 boundary: O-D D runs 5.2 s/veh
    // short and crosses the C/D line at 55, and O-D E runs 1.8 s/veh short and
    // crosses the B/C line at 30. Both differences are inside the saturation
    // flow deviation documented below.
    let los_differs = [OdMovement::D, OdMovement::E];
    for (m, demand, engine, book, book_los) in published {
        let r = od(&ix, m);
        assert_near!(r.demand, demand, 0.6, "O-D {m:?} demand");
        assert_near!(r.ett_s, engine, 0.05, format!("O-D {m:?} ETT (published {book})"));
        assert!(!r.vc_exceeds_one, "O-D {m:?} v/c should not exceed 1");
        assert!(!r.rq_exceeds_one, "O-D {m:?} R_Q should not exceed 1");
        if !los_differs.contains(&m) {
            assert_eq!(r.los, book_los, "O-D {m:?} LOS");
        }
    }
    // Interchange ETT 45.35 s/veh against the published 48.3; LOS C either way.
    assert_near!(ix.get_interchange_ett().unwrap(), 45.35, 0.05, "interchange ETT");
    assert_eq!(ix.get_interchange_los().unwrap(), LevelOfService::C);
}

/// Book defects in the Example Problem 7 saturation flow exhibits.
///
/// Every one of the ten lane group columns of Exhibits 34-75 and 34-76 lists
/// a lane width adjustment of f_w = 0.967 for the stated 10.3 ft lanes. HCM 7
/// Exhibit 19-20 is a three-tier lookup that returns 1.000 for every width
/// from 10.0 to 12.9 ft, so 0.967 cannot be produced for any width the
/// example could have meant; it is the HCM 2000 continuous form
/// `f_w = 1 + (W - 12)/30` evaluated at 11 ft. The exhibits were carried over
/// from a superseded edition.
///
/// Exhibit 34-76 then lists f_HVg = 1.000 for the northbound and southbound
/// approaches, which the example's own text gives 5% heavy vehicles. Equation
/// 19-10 gives 0.961 there, and 0.961 is exactly what the exhibit prints for
/// the eastbound and westbound approaches (3.4% heavy vehicles on a 2% grade,
/// which also evaluates to 0.961). The two deviations nearly cancel on the
/// ramp approaches, which is why their delays still reproduce within
/// 0.5 s/veh while the arterial ones do not.
#[test]
fn test_ep7_saturation_flow_factor_defects() {
    // f_w: no width in the flat tier can produce the published 0.967.
    for w in [10.0, 10.3, 11.0, 12.0, 12.9] {
        assert_near!(lane_width_factor(w), 1.000, 1e-12, "f_w at {w} ft");
    }
    // f_HVg: the northbound / southbound value the exhibit omits.
    assert_near!(
        heavy_vehicle_grade_factor(5.0, 0.0),
        0.961,
        0.0005,
        "f_HVg at 5% heavy vehicles, level (Exhibit 34-76 prints 1.000)"
    );
    // ... and the eastbound / westbound value it does print, which coincides.
    assert_near!(
        heavy_vehicle_grade_factor(3.4, 2.0),
        0.961,
        0.0005,
        "f_HVg at 3.4% heavy vehicles on a 2% grade (Exhibit 34-75)"
    );

    // The traffic pressure row reproduces exactly for all ten columns that
    // are not a left-turn phase component, and for none of the four that are.
    let cycle = 110.0;
    for (name, v, lanes, left, published) in [
        ("EB through", 911.0, 2.0, false, 0.998),
        ("EB right", 84.0, 1.0, false, 0.946),
        ("WB through", 881.0, 2.0, false, 0.995),
        ("WB right", 221.0, 1.0, false, 0.964),
        ("NB left", 174.0, 1.0, true, 0.967),
        ("NB right", 168.0, 1.0, false, 0.957),
        ("SB left", 547.0, 1.0, true, 1.044),
        ("SB right", 126.0, 1.0, false, 0.951),
    ] {
        let f_v = traffic_pressure_factor(v * cycle / 3_600.0 / lanes, left);
        assert_near!(f_v, published, 0.0006, format!("f_v for {name}"));
    }
    // The four left-turn phase components do not: Equation 23-15 gives 0.967
    // eastbound and 0.971 westbound, against the published 0.950 / 0.951 and
    // 0.950 / 0.954. The published protected and permitted values also differ
    // from each other for a single movement at a single demand, which the
    // equation cannot do.
    assert_near!(
        traffic_pressure_factor(177.0 * cycle / 3_600.0, true),
        0.967,
        0.0006,
        "f_v for the eastbound left (Exhibit 34-75 prints 0.950 / 0.951)"
    );
    assert_near!(
        traffic_pressure_factor(194.0 * cycle / 3_600.0, true),
        0.971,
        0.0006,
        "f_v for the westbound left (Exhibit 34-75 prints 0.950 / 0.954)"
    );
}

/// Exhibit 34-77 contradicts itself on the westbound left turn.
///
/// The exhibit's own relation between the opposing queue service time and the
/// unblocked permitted green is `g_u = G_perm - g_q - l_1`, which closes
/// eastbound (32 - 17 - 2 = 13.0 against the published 13.01) and does not
/// close westbound (32 - 20 - 2 = 10.0 against the published 11.78; the
/// published g_u instead implies g_q = 18.22). `g_u` is the live value of the
/// pair, because it is what enters the polygon, and both published g_u values
/// reproduce their published uniform delays. `case7.json` therefore carries
/// g_u and not g_q.
#[test]
fn test_ep7_exhibit_34_77_westbound_queue_service_time_inconsistent() {
    let g_perm: f64 = 32.0;
    let l1: f64 = 2.0;
    assert_near!(g_perm - 17.0 - l1, 13.01, 0.02, "eastbound g_u closes");
    let wb_from_gq: f64 = g_perm - 20.0 - l1;
    assert!(
        (wb_from_gq - 11.78).abs() > 1.5,
        "westbound g_q = 20 implies g_u = {wb_from_gq}, not the published 11.78"
    );
}

/// Chapter 34 drops the sneaker term that Chapter 31 requires.
///
/// Equation 31-124 gives the capacity of a protected-plus-permitted left turn
/// in an exclusive lane as
/// `c = [g_l s_lt / C + (g_u s_l + 3,600 n_s) / C] N`, with n_s = 2.0 veh of
/// left turns that clear on the change interval each cycle. Exhibit 34-78's
/// published 293 veh/h for the eastbound left omits that term; including it
/// would give 359 veh/h. The interchange path follows Chapter 34, so the
/// engine omits it too, and this test pins the size of the choice rather than
/// leaving it implicit.
#[test]
fn test_ep7_capacity_omits_the_sneaker_term() {
    let mut ix = load_case("case7.json");
    ix.analyze();
    let r = group(&ix, movements::EB_EXT_LEFT);
    let c = ix.get_cycle_length();
    let with_sneakers = r.capacity.unwrap() + 3_600.0 * 2.0 / c;
    assert!(
        with_sneakers - r.capacity.unwrap() > 60.0,
        "the omitted sneaker term is worth {} veh/h",
        with_sneakers - r.capacity.unwrap()
    );
}
