//! Integration tests for HCM Chapter 13 (Freeway Weaving Segments) against
//! the published results of HCM Chapter 27 Example Problems 1-3.
//!
//! Tolerances: flows/capacities +-5 pc/h or veh/h and lane-change rates
//! +-5 lc/h (published values are rounded to whole numbers and the book
//! carries rounded intermediates); speeds +-0.5 mi/h; densities
//! +-0.5 pc/mi/ln; LOS letters exact.

use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use transportations_library::hcm::chapter13::weaving::WeavingSegment;
use transportations_library::hcm::common::LevelOfService;

fn load_case(name: &str) -> WeavingSegment {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/ExampleCases/hcm/Weaving");
    path.push(name);
    let f = File::open(&path).unwrap_or_else(|_| panic!("Unable to open {path:?}"));
    serde_json::from_reader(BufReader::new(f)).expect("Failed to parse fixture JSON")
}

fn assert_approx(actual: f64, expected: f64, tol: f64, label: &str) {
    assert!(
        (actual - expected).abs() <= tol,
        "{label}: got {actual}, expected {expected} (+-{tol})"
    );
}

/// HCM Chapter 27, Example Problem 1: LOS of a major weaving segment.
/// One-sided major weave, L_S = 1,500 ft, N = 4, N_WL = 3, FFS = 65 mi/h.
#[test]
fn example_problem_1_major_weave() {
    let mut seg = load_case("case1.json");
    let los = seg.run_analysis();

    // Step 2 (Equation 13-1): component flows and aggregates
    assert_approx(seg.get_flow_weaving(), 1995.0, 5.0, "v_W (pc/h)");
    assert_approx(seg.get_flow_nonweaving(), 3591.0, 5.0, "v_NW (pc/h)");
    assert_approx(seg.get_flow_total(), 5586.0, 5.0, "v (pc/h)");
    assert_approx(seg.get_volume_ratio(), 0.357, 0.002, "VR");

    // Step 3 (Equation 13-2)
    assert_approx(seg.get_lc_min(), 798.0, 5.0, "LC_MIN (lc/h)");

    // Step 4 (Equation 13-4)
    assert_approx(seg.get_l_max(), 4639.0, 5.0, "L_MAX (ft)");
    assert!(seg.is_weaving_segment());

    // Step 5 (Equations 13-5 through 13-9): capacity governed by density
    assert_approx(seg.c_iwl.unwrap(), 2110.0, 5.0, "c_IWL (pc/h/ln)");
    assert_approx(seg.get_capacity(), 8038.0, 10.0, "c_W (veh/h)");

    // Step 6 (Equations 13-11 through 13-17)
    assert_approx(seg.lc_w.unwrap(), 1144.0, 5.0, "LC_W (lc/h)");
    assert_approx(seg.lc_nw.unwrap(), 782.0, 5.0, "LC_NW (lc/h)");
    assert_approx(seg.get_lc_all(), 1926.0, 8.0, "LC_ALL (lc/h)");

    // Step 7 (Equations 13-19 through 13-22)
    assert_approx(seg.get_speed_weaving(), 54.2, 0.5, "S_W (mi/h)");
    assert_approx(seg.get_speed_nonweaving(), 52.5, 0.5, "S_NW (mi/h)");
    assert_approx(seg.get_speed_avg(), 53.1, 0.5, "S (mi/h)");

    // Step 8 (Equation 13-23, Exhibit 13-6)
    assert_approx(seg.get_density(), 26.3, 0.5, "D (pc/mi/ln)");
    assert_eq!(los, LevelOfService::C);
}

/// HCM Chapter 27, Example Problem 2: LOS for a ramp weave.
/// One-sided ramp weave, L_S = 1,000 ft, N = 4, N_WL = 2, FFS = 75 mi/h,
/// demands already in pc/h (PHF = 1.00, no heavy vehicles).
#[test]
fn example_problem_2_ramp_weave() {
    let mut seg = load_case("case2.json");
    let los = seg.run_analysis();

    assert_approx(seg.get_flow_weaving(), 900.0, 1.0, "v_W (pc/h)");
    assert_approx(seg.get_flow_nonweaving(), 4100.0, 1.0, "v_NW (pc/h)");
    assert_approx(seg.get_flow_total(), 5000.0, 1.0, "v (pc/h)");
    assert_approx(seg.get_volume_ratio(), 0.180, 0.001, "VR");

    assert_approx(seg.get_lc_min(), 900.0, 1.0, "LC_MIN (lc/h)");
    assert_approx(seg.get_l_max(), 4333.0, 5.0, "L_MAX (ft)");
    assert!(seg.is_weaving_segment());

    assert_approx(seg.c_iwl.unwrap(), 2145.0, 5.0, "c_IWL (pc/h/ln)");
    assert_approx(seg.get_capacity(), 8580.0, 10.0, "c_W (pc/h)");
    // Weaving-flow criterion (Equations 13-7/13-8): 2,400/0.18 = 13,333 pc/h
    assert_approx(seg.capacity_weaving.unwrap(), 13333.0, 15.0, "c_W weaving (pc/h)");

    assert_approx(seg.lc_w.unwrap(), 1187.0, 5.0, "LC_W (lc/h)");
    assert_approx(seg.lc_nw.unwrap(), 616.0, 5.0, "LC_NW (lc/h)");
    assert_approx(seg.get_lc_all(), 1803.0, 8.0, "LC_ALL (lc/h)");

    assert_approx(seg.get_speed_weaving(), 59.1, 0.5, "S_W (mi/h)");
    assert_approx(seg.get_speed_nonweaving(), 62.5, 0.5, "S_NW (mi/h)");
    assert_approx(seg.get_speed_avg(), 61.9, 0.5, "S (mi/h)");

    assert_approx(seg.get_density(), 20.2, 0.5, "D (pc/mi/ln)");
    assert_eq!(los, LevelOfService::C);
}

/// HCM Chapter 27, Example Problem 3: LOS of a two-sided weaving segment.
/// L_S = 750 ft, N = 3, N_WL = 0, FFS = 60 mi/h, rolling terrain.
///
/// The published solution carries a slightly inconsistent nonweaving flow
/// (5,015 vs. 4,995 pc/h) into Equations 13-12/13-13, so lane-change-rate
/// tolerances are widened to +-10 lc/h for this case.
#[test]
fn example_problem_3_two_sided_weave() {
    let mut seg = load_case("case3.json");
    let los = seg.run_analysis();

    assert_approx(seg.get_flow_weaving(), 389.0, 2.0, "v_W (pc/h)");
    assert_approx(seg.get_flow_nonweaving(), 4995.0, 5.0, "v_NW (pc/h)");
    assert_approx(seg.get_flow_total(), 5384.0, 5.0, "v (pc/h)");
    assert_approx(seg.get_volume_ratio(), 0.072, 0.001, "VR");

    // Two-sided: LC_MIN = LC_RR x v_RR (Equation 13-3)
    assert_approx(seg.get_lc_min(), 778.0, 4.0, "LC_MIN (lc/h)");
    assert_approx(seg.get_l_max(), 6405.0, 5.0, "L_MAX (ft)");
    assert!(seg.is_weaving_segment());

    assert_approx(seg.c_iwl.unwrap(), 1867.0, 5.0, "c_IWL (pc/h/ln)");
    assert_approx(seg.get_capacity(), 4593.0, 10.0, "c_W (veh/h)");
    // Two-sided segments have no weaving-flow capacity limit
    assert!(seg.capacity_weaving.is_none());

    assert_approx(seg.lc_w.unwrap(), 960.0, 10.0, "LC_W (lc/h)");
    assert_approx(seg.lc_nw.unwrap(), 861.0, 10.0, "LC_NW (lc/h)");
    assert_approx(seg.get_lc_all(), 1821.0, 15.0, "LC_ALL (lc/h)");

    assert_approx(seg.get_speed_weaving(), 45.9, 0.5, "S_W (mi/h)");
    assert_approx(seg.get_speed_nonweaving(), 45.8, 0.5, "S_NW (mi/h)");
    assert_approx(seg.get_speed_avg(), 45.8, 0.5, "S (mi/h)");

    assert_approx(seg.get_density(), 39.2, 0.5, "D (pc/mi/ln)");
    assert_eq!(los, LevelOfService::E);
}
