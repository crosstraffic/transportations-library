//! Full-pipeline integration tests for HCM Chapter 22 (roundabouts)
//! against the published answers of HCM Chapter 33, Example Problems 1
//! and 2.
//!
//! Tolerances: LOS exact; control delays within +-0.5 s/veh; capacities
//! within +-5 veh/h.

use std::fs;

use transportations_library::hcm::roundabouts::roundabouts::Roundabouts;

const DELAY_TOL: f64 = 0.5;
const CAPACITY_TOL: f64 = 5.0;

fn load(case: &str) -> Roundabouts {
    let path = format!("tests/ExampleCases/hcm/Roundabouts/{case}.json");
    let json = fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {path}: {e}"));
    Roundabouts::from_json(&json).unwrap_or_else(|e| panic!("parse {path}: {e}"))
}

fn assert_close(value: f64, expected: f64, tol: f64, what: &str) {
    assert!(
        (value - expected).abs() <= tol,
        "{what}: got {value:.2}, expected {expected:.2} (tol {tol})"
    );
}

/// HCM Chapter 33, Example Problem 1 (single-lane roundabout with bypass
/// lanes): published answers c_NB = 597, c_SB = 618, c_EB = 824,
/// c_WB = 694, c_bypass,WB = 851 veh/h; x_NB = 0.70; lane delays
/// 22.6 / 14.0 / 0 / 22.0 / 26.8 / 20.2 s with LOS C/B/A/C/D/C
/// (Exhibit 33-8); approach delays d_WB = 23.3 (C), d_SB = 4.7 (A);
/// intersection 17.5 s LOS C; Q95,NB = 5.7 veh.
#[test]
fn test_roundabout_example_problem_1_full_pipeline() {
    let mut r = load("case1");
    r.analyze();

    let nb = &r.nb.lanes[0];
    assert_close(nb.capacity_veh, 597.0, CAPACITY_TOL, "c_NB");
    assert_close(nb.v_c_ratio, 0.70, 0.01, "x_NB");
    assert_close(nb.control_delay, 22.6, DELAY_TOL, "d_NB");
    assert_eq!(nb.los, 'C', "NB entry LOS");
    assert_close(nb.queue_95, 5.7, 0.3, "Q95,NB");

    let sb = &r.sb.lanes[0];
    assert_close(sb.capacity_veh, 618.0, CAPACITY_TOL, "c_SB");
    assert_close(sb.control_delay, 14.0, DELAY_TOL, "d_SB entry");
    assert_eq!(sb.los, 'B', "SB entry LOS");
    let sb_bypass = r.sb.bypass_lane.as_ref().unwrap();
    assert_close(sb_bypass.control_delay, 0.0, 1e-9, "d_bypass,SB");
    assert_eq!(sb_bypass.los, 'A', "SB bypass LOS");

    let eb = &r.eb.lanes[0];
    assert_close(eb.capacity_veh, 824.0, CAPACITY_TOL, "c_EB");
    assert_close(eb.control_delay, 22.0, DELAY_TOL, "d_EB");
    assert_eq!(eb.los, 'C', "EB entry LOS");

    let wb = &r.wb.lanes[0];
    assert_close(wb.capacity_veh, 694.0, CAPACITY_TOL, "c_WB");
    assert_close(wb.control_delay, 26.8, DELAY_TOL, "d_WB");
    assert_eq!(wb.los, 'D', "WB entry LOS");
    let wb_bypass = r.wb.bypass_lane.as_ref().unwrap();
    assert_close(wb_bypass.capacity_veh, 851.0, CAPACITY_TOL, "c_bypass,WB");
    assert_close(wb_bypass.control_delay, 20.2, DELAY_TOL, "d_bypass,WB");
    assert_eq!(wb_bypass.los, 'C', "WB bypass LOS");

    // Approach and intersection aggregation (Equations 22-18 and 22-19)
    assert_close(r.wb.control_delay.unwrap(), 23.3, DELAY_TOL, "d_A,WB");
    assert_eq!(r.wb.los.unwrap(), 'C', "WB approach LOS");
    assert_close(r.sb.control_delay.unwrap(), 4.7, DELAY_TOL, "d_A,SB");
    assert_eq!(r.sb.los.unwrap(), 'A', "SB approach LOS");
    assert_close(r.intersection_delay.unwrap(), 17.5, DELAY_TOL, "d_I");
    assert_eq!(r.intersection_los.unwrap(), 'C', "intersection LOS");
}

/// HCM Chapter 33, Example Problem 2 (multilane roundabout): published
/// answers c_NB = 607, c_SB,L = 651, c_SB,R = 723, c_EB = 675,
/// c_WB = 964 veh/h; lane delays 11.8 / 13.0 / 14.6 / 14.0 / 16.1 / 8.8 /
/// 7.8 s with LOS B/B/B/B/C/A/A (Exhibit 33-11); approach delays 11.8 /
/// 13.9 / 15.1 / 8.3 s; intersection 12.3 s LOS B; Q95,NB = 1.9 veh.
#[test]
fn test_roundabout_example_problem_2_full_pipeline() {
    let mut r = load("case2");
    r.analyze();

    let nb = &r.nb.lanes[0];
    assert_close(nb.capacity_veh, 607.0, CAPACITY_TOL, "c_NB");
    assert_close(nb.control_delay, 11.8, DELAY_TOL, "d_NB");
    assert_eq!(nb.los, 'B', "NB LOS");
    assert_close(nb.queue_95, 1.9, 0.2, "Q95,NB");

    assert_close(r.sb.lanes[0].capacity_veh, 651.0, CAPACITY_TOL, "c_SB,L");
    assert_close(r.sb.lanes[1].capacity_veh, 723.0, CAPACITY_TOL, "c_SB,R");
    assert_close(r.sb.lanes[0].control_delay, 13.0, DELAY_TOL, "d_SB,L");
    assert_close(r.sb.lanes[1].control_delay, 14.6, DELAY_TOL, "d_SB,R");
    assert_eq!(r.sb.lanes[0].los, 'B');
    assert_eq!(r.sb.lanes[1].los, 'B');

    assert_close(r.eb.lanes[0].capacity_veh, 675.0, CAPACITY_TOL, "c_EB,L");
    assert_close(r.eb.lanes[1].capacity_veh, 675.0, CAPACITY_TOL, "c_EB,R");
    assert_close(r.eb.lanes[0].control_delay, 14.0, DELAY_TOL, "d_EB,L");
    assert_close(r.eb.lanes[1].control_delay, 16.1, DELAY_TOL, "d_EB,R");
    assert_eq!(r.eb.lanes[0].los, 'B');
    assert_eq!(r.eb.lanes[1].los, 'C');

    assert_close(r.wb.lanes[0].capacity_veh, 964.0, CAPACITY_TOL, "c_WB,L");
    assert_close(r.wb.lanes[1].capacity_veh, 964.0, CAPACITY_TOL, "c_WB,R");
    assert_close(r.wb.lanes[0].control_delay, 8.8, DELAY_TOL, "d_WB,L");
    assert_close(r.wb.lanes[1].control_delay, 7.8, DELAY_TOL, "d_WB,R");
    assert_eq!(r.wb.lanes[0].los, 'A');
    assert_eq!(r.wb.lanes[1].los, 'A');

    assert_close(r.nb.control_delay.unwrap(), 11.8, DELAY_TOL, "d_A,NB");
    assert_close(r.sb.control_delay.unwrap(), 13.9, DELAY_TOL, "d_A,SB");
    assert_close(r.eb.control_delay.unwrap(), 15.1, DELAY_TOL, "d_A,EB");
    assert_close(r.wb.control_delay.unwrap(), 8.3, DELAY_TOL, "d_A,WB");
    assert_eq!(r.eb.los.unwrap(), 'C', "EB approach LOS");
    assert_close(r.intersection_delay.unwrap(), 12.3, DELAY_TOL, "d_I");
    assert_eq!(r.intersection_los.unwrap(), 'B', "intersection LOS");
}

/// Serde round-trip of a fully analyzed fixture.
#[test]
fn test_roundabout_fixture_roundtrip() {
    let mut r = load("case1");
    r.analyze();
    let json = r.to_json().unwrap();
    let back = Roundabouts::from_json(&json).unwrap();
    assert_eq!(back.intersection_delay, r.intersection_delay);
}
