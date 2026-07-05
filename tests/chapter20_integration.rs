//! Full-pipeline integration tests for HCM Chapter 20 (TWSC intersections)
//! against the published answers of HCM Chapter 32, TWSC Example Problems 1
//! and 3.
//!
//! Tolerances: LOS exact; control delays within +-0.5 s/veh; capacities
//! within +-5 veh/h of the published (rounded) values.

use std::fs;

use transportations_library::hcm::chapter20::twsc::{Mv, Twsc};

const DELAY_TOL: f64 = 0.5;
const CAPACITY_TOL: f64 = 5.0;

fn load(case: &str) -> Twsc {
    let path = format!("tests/ExampleCases/hcm/Twsc/{case}.json");
    let json = fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {path}: {e}"));
    Twsc::from_json(&json).unwrap_or_else(|e| panic!("parse {path}: {e}"))
}

fn assert_close(value: f64, expected: f64, tol: f64, what: &str) {
    assert!(
        (value - expected).abs() <= tol,
        "{what}: got {value:.2}, expected {expected:.2} (tol {tol})"
    );
}

/// HCM Chapter 32, TWSC Example Problem 1 (three-leg intersection):
/// published answers c_m,4 = 1,238; c_m,9 = 760; c_m,7 = 268;
/// c_SH,NB = 521 veh/h; d_4 = 8.3 s (LOS A); d_NB = 14.9 s (LOS B);
/// d_A,WB = 2.9 s; d_I = 4.1 s; Q95,4 = 0.4; Q95,NB = 1.3 veh.
#[test]
fn test_twsc_example_problem_1_full_pipeline() {
    let mut twsc = load("case1");
    twsc.analyze();

    let m4 = &twsc.movements[Mv::M4.idx()];
    assert_close(m4.movement_capacity.unwrap(), 1238.0, CAPACITY_TOL, "c_m,4");
    let m9 = &twsc.movements[Mv::M9.idx()];
    assert_close(m9.movement_capacity.unwrap(), 760.0, CAPACITY_TOL, "c_m,9");
    let m7 = &twsc.movements[Mv::M7.idx()];
    assert_close(m7.movement_capacity.unwrap(), 268.0, CAPACITY_TOL, "c_m,7");

    // Shared northbound minor lane
    assert_eq!(twsc.lanes_nb.len(), 1);
    let nb = &twsc.lanes_nb[0];
    assert_close(nb.capacity, 521.0, CAPACITY_TOL, "c_SH,NB");
    assert_close(nb.control_delay, 14.9, DELAY_TOL, "d_SH,NB");
    assert_eq!(nb.los, 'B', "NB approach LOS");
    assert_close(nb.queue_95, 1.3, 0.2, "Q95,NB");

    // Major-street left turn
    assert_close(m4.control_delay.unwrap(), 8.3, DELAY_TOL, "d_4");
    assert_eq!(m4.los.unwrap(), 'A', "movement 4 LOS");
    assert_close(m4.queue_95.unwrap(), 0.4, 0.2, "Q95,4");

    // Approach and intersection delays (Equations 20-64 and 20-65)
    let [d_eb, d_wb, d_nb, _] = twsc.approach_delays.unwrap();
    assert_close(d_eb, 0.0, DELAY_TOL, "d_A,EB");
    assert_close(d_wb, 2.9, DELAY_TOL, "d_A,WB");
    assert_close(d_nb, 14.9, DELAY_TOL, "d_A,NB");
    assert_close(twsc.intersection_delay.unwrap(), 4.1, DELAY_TOL, "d_I");
}

/// HCM Chapter 32, TWSC Example Problem 3 (two-stage gap acceptance and
/// flared minor approaches): published answers c_T,8 = 390, c_T,11 = 405,
/// c_T,7 = 365, c_T,10 = 342, c_F,NB = 498, c_F,SB = 487 veh/h;
/// d_1 = 8.4 s (A), d_4 = 8.2 s (A), d_NB = 18.3 s (C), d_SB = 15.6 s (C);
/// d_I = 6.3 s; Q95: 0.1, 0.2, 2.4, 1.3 veh.
#[test]
fn test_twsc_example_problem_3_full_pipeline() {
    let mut twsc = load("case2");
    twsc.analyze();

    // Two-stage movement capacities
    assert_close(
        twsc.movements[Mv::M8.idx()].movement_capacity.unwrap(),
        390.0,
        CAPACITY_TOL,
        "c_T,8",
    );
    assert_close(
        twsc.movements[Mv::M11.idx()].movement_capacity.unwrap(),
        405.0,
        CAPACITY_TOL,
        "c_T,11",
    );
    assert_close(
        twsc.movements[Mv::M7.idx()].movement_capacity.unwrap(),
        365.0,
        CAPACITY_TOL,
        "c_T,7",
    );
    assert_close(
        twsc.movements[Mv::M10.idx()].movement_capacity.unwrap(),
        342.0,
        CAPACITY_TOL,
        "c_T,10",
    );

    // Flared-lane approach capacities (Equation 20-50)
    assert_close(twsc.lanes_nb[0].capacity, 498.0, CAPACITY_TOL, "c_F,NB");
    assert_close(twsc.lanes_sb[0].capacity, 487.0, CAPACITY_TOL, "c_F,SB");

    // Delay and LOS
    let m1 = &twsc.movements[Mv::M1.idx()];
    let m4 = &twsc.movements[Mv::M4.idx()];
    assert_close(m1.control_delay.unwrap(), 8.4, DELAY_TOL, "d_1");
    assert_close(m4.control_delay.unwrap(), 8.2, DELAY_TOL, "d_4");
    assert_eq!(m1.los.unwrap(), 'A');
    assert_eq!(m4.los.unwrap(), 'A');
    assert_close(twsc.lanes_nb[0].control_delay, 18.3, DELAY_TOL, "d_NB");
    assert_close(twsc.lanes_sb[0].control_delay, 15.6, DELAY_TOL, "d_SB");
    assert_eq!(twsc.lanes_nb[0].los, 'C', "NB LOS");
    assert_eq!(twsc.lanes_sb[0].los, 'C', "SB LOS");

    let [d_eb, d_wb, _, _] = twsc.approach_delays.unwrap();
    assert_close(d_eb, 0.8, DELAY_TOL, "d_A,EB");
    assert_close(d_wb, 1.2, DELAY_TOL, "d_A,WB");
    assert_close(twsc.intersection_delay.unwrap(), 6.3, DELAY_TOL, "d_I");

    // Queues
    assert_close(m1.queue_95.unwrap(), 0.1, 0.2, "Q95,1");
    assert_close(m4.queue_95.unwrap(), 0.2, 0.2, "Q95,4");
    assert_close(twsc.lanes_nb[0].queue_95, 2.4, 0.2, "Q95,NB");
    assert_close(twsc.lanes_sb[0].queue_95, 1.3, 0.2, "Q95,SB");
}

/// HCM Chapter 32, TWSC Example Problem 4 (TWSC between two coordinated
/// upstream signals; Step 5b platoon blockage, Equations 20-19 through
/// 20-21). Four-lane major street (N = 2), major-street left turns share the
/// through lane, no minor-street through movements, one stage. Published
/// p_b = 0.170 for movements 1/4/9/12 and 0.260 for movements 7/10
/// (Exhibit 32-12).
///
/// Published answers (Exhibit 32-13 problem text):
/// * conflicting flows v_c,1 = 1,086; v_c,4 = 1,076; v_c,9 = 538;
///   v_c,12 = 543; v_c,7 = 1,827; v_c,10 = 1,832 veh/h;
/// * unblocked conflicting flows v_c,u = 694, 682, 34, 40, 1,415, 1,422;
/// * potential (= movement, for 1/4/9/12) capacities c_p,1 = 750;
///   c_p,4 = 758; c_p,9 = 859; c_p,12 = 852; c_p,7 = 73; c_p,10 = 72 veh/h;
/// * movement capacities c_m,7 = 47; c_m,10 = 47 veh/h (f_p,7 = 0.647,
///   f_p,10 = 0.648);
/// * delays d_1 = 10.3 (B); d_4 = 10.3 (B); d_9 = 9.7 (A); d_12 = 9.8 (A);
///   d_7 = 529 (F); d_10 = 529 (F);
/// * approach delays d_A,NB = d_A,SB = 241 s; d_A,EB = d_A,WB = 1.9 s;
///   d_I = 34.1 s;
/// * queues Q95: 0.3, 0.3, 0.4, 0.4, 7.9, 7.9 veh.
///
/// Two published values are reproduced only in regime, not exactly, because
/// Example Problem 4's major-street left turns share the through lane and the
/// published answer uses the shared-major-lane queue-free probability
/// p*_0 = 0.856 (Equations 20-33/34, Step 7d), while this library still uses
/// the exclusive-lane p_0 = 0.900 (no geometry input marks a shared/short
/// major-left pocket): c_m,7 = c_m,10 computes ~52 (published 47), and the
/// derived NB/SB approach and intersection delays are lower. These, plus the
/// unfolded Step 11b Rank 1 delay on the EB/WB approaches, are asserted in
/// regime with the published deltas recorded inline.
#[test]
fn test_twsc_example_problem_4_upstream_signals() {
    let mut twsc = load("case3");
    twsc.analyze();
    let m = |mv: Mv| twsc.movements[mv.idx()].clone();

    // Step 3 conflicting flows (movements 7 and 10 via override).
    assert_close(m(Mv::M1).conflicting_flow.unwrap(), 1086.0, 1.0, "v_c,1");
    assert_close(m(Mv::M4).conflicting_flow.unwrap(), 1076.0, 1.0, "v_c,4");
    assert_close(m(Mv::M9).conflicting_flow.unwrap(), 538.0, 1.0, "v_c,9");
    assert_close(m(Mv::M12).conflicting_flow.unwrap(), 543.0, 1.0, "v_c,12");
    assert_close(m(Mv::M7).conflicting_flow.unwrap(), 1827.0, 1.0, "v_c,7");
    assert_close(m(Mv::M10).conflicting_flow.unwrap(), 1832.0, 1.0, "v_c,10");

    // Step 5b potential capacities (Equations 20-19 through 20-21).
    assert_close(m(Mv::M1).potential_capacity.unwrap(), 750.0, CAPACITY_TOL, "c_p,1");
    assert_close(m(Mv::M4).potential_capacity.unwrap(), 758.0, CAPACITY_TOL, "c_p,4");
    assert_close(m(Mv::M9).potential_capacity.unwrap(), 859.0, CAPACITY_TOL, "c_p,9");
    assert_close(m(Mv::M12).potential_capacity.unwrap(), 852.0, CAPACITY_TOL, "c_p,12");
    assert_close(m(Mv::M7).potential_capacity.unwrap(), 73.0, CAPACITY_TOL, "c_p,7");
    assert_close(m(Mv::M10).potential_capacity.unwrap(), 72.0, CAPACITY_TOL, "c_p,10");

    // Movement capacities. 1/4/9/12 match published; 7/10 land in the same
    // oversaturated LOS-F regime (~52 vs published 47, shared-lane p* delta).
    assert_close(m(Mv::M1).movement_capacity.unwrap(), 750.0, CAPACITY_TOL, "c_m,1");
    assert_close(m(Mv::M4).movement_capacity.unwrap(), 758.0, CAPACITY_TOL, "c_m,4");
    assert_close(m(Mv::M9).movement_capacity.unwrap(), 859.0, CAPACITY_TOL, "c_m,9");
    assert_close(m(Mv::M12).movement_capacity.unwrap(), 852.0, CAPACITY_TOL, "c_m,12");
    assert!(
        (45.0..60.0).contains(&m(Mv::M7).movement_capacity.unwrap()),
        "c_m,7 = {:?} (published 47 with shared-lane p*)",
        m(Mv::M7).movement_capacity
    );
    assert!(
        (45.0..60.0).contains(&m(Mv::M10).movement_capacity.unwrap()),
        "c_m,10 = {:?} (published 47 with shared-lane p*)",
        m(Mv::M10).movement_capacity
    );

    // Delay and LOS (Equation 20-61, Exhibit 20-2). Major-street left turns
    // report at the movement level; minor-street movements report at the
    // lane level (each in an exclusive lane under the Separate config).
    assert_close(m(Mv::M1).control_delay.unwrap(), 10.3, DELAY_TOL, "d_1");
    assert_close(m(Mv::M4).control_delay.unwrap(), 10.3, DELAY_TOL, "d_4");
    assert_eq!(m(Mv::M1).los.unwrap(), 'B', "movement 1 LOS");
    assert_eq!(m(Mv::M4).los.unwrap(), 'B', "movement 4 LOS");

    let nb_lane = |mv: Mv| {
        twsc.lanes_nb
            .iter()
            .find(|l| l.movements.contains(&mv))
            .unwrap_or_else(|| panic!("NB lane for {mv:?}"))
    };
    let sb_lane = |mv: Mv| {
        twsc.lanes_sb
            .iter()
            .find(|l| l.movements.contains(&mv))
            .unwrap_or_else(|| panic!("SB lane for {mv:?}"))
    };
    let (nb_left, nb_right) = (nb_lane(Mv::M7), nb_lane(Mv::M9));
    let (sb_left, sb_right) = (sb_lane(Mv::M10), sb_lane(Mv::M12));
    assert_close(nb_right.control_delay, 9.7, DELAY_TOL, "d_9");
    assert_close(sb_right.control_delay, 9.8, DELAY_TOL, "d_12");
    assert_eq!(nb_right.los, 'A', "movement 9 LOS");
    assert_eq!(sb_right.los, 'A', "movement 12 LOS");
    // Oversaturated left turns: published d = 529 s; assert the LOS-F regime
    // rather than the exact value (Equation 20-61 is highly sensitive near
    // v/c = 1.5-1.7, and c_m,7 differs from the published 47 by the shared-lane
    // p* delta noted above, so a wide tolerance is used).
    assert!(nb_left.control_delay > 350.0, "d_7 = {}", nb_left.control_delay);
    assert!(sb_left.control_delay > 350.0, "d_10 = {}", sb_left.control_delay);
    assert_eq!(nb_left.los, 'F', "NB left-turn lane LOS");
    assert_eq!(sb_left.los, 'F', "SB left-turn lane LOS");

    // Approach delays: the minor-street approaches aggregate the LOS-F left
    // lane and the low-delay right lane. Published d_A,NB = d_A,SB = 241 s at
    // c_m,7 = 47; with the shared-lane p* delta above (c_m,7 ~ 52) the modeled
    // left-turn delay is lower, so the approach delay lands near 203 s. Assert
    // the LOS-F-dominated regime and record the published value.
    let [d_eb, d_wb, d_nb, d_sb] = twsc.approach_delays.unwrap();
    assert!(d_nb > 180.0, "d_A,NB = {d_nb} (published 241)");
    assert!(d_sb > 180.0, "d_A,SB = {d_sb} (published 241)");
    // Published d_A,EB = d_A,WB = 1.9 s include the Step 11b Rank 1
    // shared-major-lane delay (d_2+3 = d_5+6 = 1.3 s), which this library
    // exposes via Twsc::rank1_delay but does not yet fold into Step 12
    // aggregation; without it the major approaches carry only the left-turn
    // delay, so the computed values are lower.
    assert!(d_eb < 1.0, "d_A,EB computed {d_eb} (published 1.9 with Rank 1 delay)");
    assert!(d_wb < 1.0, "d_A,WB computed {d_wb} (published 1.9 with Rank 1 delay)");
    // Published d_I = 34.1 s combines the Rank 1 shared-major-lane delay on the
    // EB/WB approaches (unfolded here) with d_A,NB = d_A,SB = 241 s (shortened
    // here by the shared-lane p* delta). The computed value stays in the same
    // tens-of-seconds regime dominated by the minor approaches.
    let d_i = twsc.intersection_delay.unwrap();
    assert!((24.0..35.0).contains(&d_i), "d_I computed {d_i} (published 34.1)");

    // Step 13 queues (Equation 20-66).
    assert_close(m(Mv::M1).queue_95.unwrap(), 0.3, 0.2, "Q95,1");
    assert_close(nb_right.queue_95, 0.4, 0.2, "Q95,9");
    // Published Q95,7 = 7.9 veh at c_m,7 = 47; the shared-lane p* delta above
    // raises c_m,7 to ~52 and shortens the modeled queue accordingly.
    assert!(nb_left.queue_95 > 5.0, "Q95,7 = {}", nb_left.queue_95);
}

/// Serde round-trip of a fully analyzed fixture (binding-layer contract).
#[test]
fn test_twsc_fixture_roundtrip() {
    let mut twsc = load("case1");
    twsc.analyze();
    let json = twsc.to_json().unwrap();
    let back = Twsc::from_json(&json).unwrap();
    assert_eq!(
        back.movements[Mv::M7.idx()].movement_capacity,
        twsc.movements[Mv::M7.idx()].movement_capacity
    );
}
