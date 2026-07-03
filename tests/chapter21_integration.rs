//! Full-pipeline integration tests for HCM Chapter 21 (AWSC intersections)
//! against the published answers of HCM Chapter 32, AWSC Example Problems 1
//! and 2.
//!
//! Tolerances: LOS exact; control delays within +-0.5 s/veh; departure
//! headways within +-0.1 s.

use std::fs;

use transportations_library::hcm::chapter21::awsc::{ApproachDir, Awsc, GeometryGroup};

const DELAY_TOL: f64 = 0.5;

fn load(case: &str) -> Awsc {
    let path = format!("tests/ExampleCases/hcm/Awsc/{case}.json");
    let json = fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {path}: {e}"));
    Awsc::from_json(&json).unwrap_or_else(|e| panic!("parse {path}: {e}"))
}

fn assert_close(value: f64, expected: f64, tol: f64, what: &str) {
    assert!(
        (value - expected).abs() <= tol,
        "{what}: got {value:.3}, expected {expected:.3} (tol {tol})"
    );
}

/// HCM Chapter 32, AWSC Example Problem 1 (single-lane, three-leg):
/// published answers h_d,EB = 4.97 s, h_d,WB = 4.74 s, h_d,SB = 5.70 s
/// (Exhibit 32-21); t_s,EB = 2.97 s; d_EB = 13.0 s (LOS B), d_WB = 13.5 s,
/// d_minor = 10.6 s; intersection 12.8 s LOS B; Q95,EB = 2.9 veh.
#[test]
fn test_awsc_example_problem_1_full_pipeline() {
    let mut awsc = load("case1");
    awsc.analyze();

    assert_eq!(awsc.eb.geometry_group, Some(GeometryGroup::G1));

    let eb = &awsc.eb.lanes[0];
    assert_close(eb.departure_headway.unwrap(), 4.97, 0.1, "h_d,EB");
    assert_close(eb.degree_of_utilization.unwrap(), 0.508, 0.01, "x_EB");
    assert_close(eb.service_time.unwrap(), 2.97, 0.1, "t_s,EB");
    assert_close(eb.control_delay.unwrap(), 13.0, DELAY_TOL, "d_EB");
    assert_eq!(eb.los.unwrap(), 'B', "EB LOS");
    assert_close(eb.queue_95.unwrap(), 2.9, 0.2, "Q95,EB");

    let wb = &awsc.wb.lanes[0];
    assert_close(wb.departure_headway.unwrap(), 4.74, 0.1, "h_d,WB");
    assert_close(wb.control_delay.unwrap(), 13.5, DELAY_TOL, "d_WB");
    assert_eq!(wb.los.unwrap(), 'B');

    let sb = &awsc.sb.lanes[0];
    assert_close(sb.departure_headway.unwrap(), 5.70, 0.1, "h_d,SB");
    assert_close(sb.control_delay.unwrap(), 10.6, DELAY_TOL, "d_SB");

    assert_close(awsc.intersection_delay.unwrap(), 12.8, DELAY_TOL, "d_I");
    assert_eq!(awsc.intersection_los.unwrap(), 'B', "intersection LOS");
}

/// HCM Chapter 32, AWSC Example Problem 1, Step 12: eastbound lane capacity
/// approximately 720 veh/h (below the naive 748 veh/h estimate because of
/// approach interactions). See chapter21/tests.rs for the tolerance note.
#[test]
fn test_awsc_example_problem_1_capacity() {
    let mut awsc = load("case1");
    awsc.step1_2_flow_rates();
    awsc.step3_geometry_groups();
    awsc.step4_headway_adjustments();
    let c = awsc.capacity_of_lane(ApproachDir::EB, 0);
    assert_close(c, 720.0, 20.0, "c_EB");
    assert!(c < 748.0, "capacity must reflect approach interactions");
}

/// HCM Chapter 32, AWSC Example Problem 2 (multilane, four-leg, 512-state
/// framework): published answers h_d,EB,1 = 8.19 s, x_EB,1 = 0.1274,
/// t_s,EB,1 = 5.89 s, d_EB,1 = 12.1 s (LOS B), d_EB,2 = 16.1 s;
/// d_EB = 15.3 s (LOS C), d_WB = 14.3 s, d_NB = 13.1 s, d_SB = 12.6 s;
/// intersection 14.0 s LOS B; Q95,EB,1 = 0.4 veh.
#[test]
fn test_awsc_example_problem_2_full_pipeline() {
    let mut awsc = load("case2");
    awsc.analyze();

    assert_eq!(awsc.eb.geometry_group, Some(GeometryGroup::G6));

    let eb1 = &awsc.eb.lanes[0];
    assert_close(eb1.departure_headway.unwrap(), 8.19, 0.15, "h_d,EB,1");
    assert_close(eb1.degree_of_utilization.unwrap(), 0.1274, 0.005, "x_EB,1");
    assert_close(eb1.service_time.unwrap(), 5.89, 0.15, "t_s,EB,1");
    assert_close(eb1.control_delay.unwrap(), 12.1, DELAY_TOL, "d_EB,1");
    assert_eq!(eb1.los.unwrap(), 'B', "EB lane 1 LOS");
    assert_close(eb1.queue_95.unwrap(), 0.4, 0.2, "Q95,EB,1");

    let eb2 = &awsc.eb.lanes[1];
    assert_close(eb2.control_delay.unwrap(), 16.1, DELAY_TOL, "d_EB,2");
    assert_eq!(eb2.los.unwrap(), 'C', "EB lane 2 LOS");

    assert_close(awsc.eb.control_delay.unwrap(), 15.3, DELAY_TOL, "d_EB");
    assert_eq!(awsc.eb.los.unwrap(), 'C', "EB approach LOS");
    assert_close(awsc.wb.control_delay.unwrap(), 14.3, DELAY_TOL, "d_WB");
    assert_close(awsc.nb.control_delay.unwrap(), 13.1, DELAY_TOL, "d_NB");
    assert_close(awsc.sb.control_delay.unwrap(), 12.6, DELAY_TOL, "d_SB");

    assert_close(awsc.intersection_delay.unwrap(), 14.0, DELAY_TOL, "d_I");
    assert_eq!(awsc.intersection_los.unwrap(), 'B', "intersection LOS");
}

/// Serde round-trip of a fully analyzed fixture.
#[test]
fn test_awsc_fixture_roundtrip() {
    let mut awsc = load("case1");
    awsc.analyze();
    let json = awsc.to_json().unwrap();
    let back = Awsc::from_json(&json).unwrap();
    assert_eq!(back.intersection_delay, awsc.intersection_delay);
}
