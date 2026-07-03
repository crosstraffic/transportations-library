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
