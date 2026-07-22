//! Integration tests for HCM Chapter 14 (Freeway Merge and Diverge Segments)
//! against the published results of HCM Chapter 28 Example Problems 1-4.
//!
//! Tolerances: flows +-5 pc/h (published values are rounded to whole numbers
//! and the book carries rounded intermediates); speeds +-0.5 mi/h; densities
//! +-0.5 pc/mi/ln; LOS letters exact.

use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use transportations_library::hcm::merge_diverge::merge_diverge::RampSegment;
use transportations_library::hcm::common::LevelOfService;

fn load_case(name: &str) -> RampSegment {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/ExampleCases/hcm/MergeDiverge");
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

/// HCM Chapter 28, Example Problem 1: isolated one-lane, right-hand on-ramp
/// to a four-lane freeway (FFS = 60 mi/h, ramp FFS = 45 mi/h, L_A = 740 ft).
#[test]
fn example_problem_1_isolated_on_ramp_four_lane() {
    let mut seg = load_case("case1.json");
    let los = seg.run_analysis();

    // Step 1 (Equation 14-1)
    assert_approx(seg.get_flow_freeway(), 2918.0, 5.0, "v_F (pc/h)");
    assert_approx(seg.get_flow_ramp(), 625.0, 2.0, "v_R (pc/h)");

    // Step 2: four-lane freeway, P_FM = 1.000 (Exhibit 14-8)
    assert_approx(seg.p_f.unwrap(), 1.0, 1e-9, "P_FM");
    assert_approx(seg.get_v12(), 2918.0, 5.0, "v_12 (pc/h)");
    assert_approx(seg.get_vr12(), 3543.0, 6.0, "v_R12 (pc/h)");

    // Step 3: capacity checks (Exhibits 14-10/14-12)
    assert_approx(seg.get_capacity_freeway(), 4600.0, 1e-9, "freeway capacity (pc/h)");
    assert_approx(seg.get_capacity_ramp(), 2100.0, 1e-9, "ramp capacity (pc/h)");
    assert_eq!(seg.demand_exceeds_capacity, Some(false));
    assert_eq!(seg.exceeds_max_desirable, Some(false));

    // Step 4 (Equation 14-22, Exhibit 14-3)
    assert_approx(seg.get_density(), 28.2, 0.5, "D_R (pc/mi/ln)");
    assert_eq!(los, LevelOfService::D);

    // Step 5 (Exhibit 14-13): S_R = 53.0 mi/h with M_S = 0.389
    assert_approx(seg.get_speed_ramp(), 53.0, 0.5, "S_R (mi/h)");
    // No outer lanes on a four-lane freeway
    assert!(seg.get_speed_outer().is_none());
}

/// HCM Chapter 28, Example Problem 2 (first off-ramp): two adjacent
/// single-lane, right-hand off-ramps on a six-lane freeway (FFS = 60 mi/h).
/// The downstream off-ramp is beyond L_EQ = 657 ft, so Equation 14-9 governs.
#[test]
fn example_problem_2_first_off_ramp_six_lane() {
    let mut seg = load_case("case2.json");
    let los = seg.run_analysis();

    assert_approx(seg.get_flow_freeway(), 5093.0, 5.0, "v_F (pc/h)");
    assert_approx(seg.get_flow_ramp(), 340.0, 2.0, "v_R (pc/h)");

    // Step 2: isolated treatment (L_DOWN = 750 ft >= L_EQ = 657 ft),
    // Equation 14-9: P_FD = 0.617
    assert_approx(seg.p_f.unwrap(), 0.617, 0.002, "P_FD");
    assert_approx(seg.get_v12(), 3273.0, 6.0, "v_12 (pc/h)");

    // Step 3
    assert_approx(seg.get_capacity_freeway(), 6900.0, 1e-9, "freeway capacity (pc/h)");
    assert_approx(seg.get_capacity_ramp(), 2000.0, 1e-9, "ramp capacity (pc/h)");
    assert_eq!(seg.demand_exceeds_capacity, Some(false));
    assert_eq!(seg.exceeds_max_desirable, Some(false));

    // Step 4 (Equation 14-23, Exhibit 14-3)
    assert_approx(seg.get_density(), 27.9, 0.5, "D_R (pc/mi/ln)");
    assert_eq!(los, LevelOfService::C);

    // Step 5 (Exhibits 14-14/14-15): S_R = 52.9, S_O = 62.6, S = 56.0
    assert_approx(seg.get_speed_ramp(), 52.9, 0.5, "S_R (mi/h)");
    assert_approx(seg.get_speed_outer().unwrap(), 62.6, 0.5, "S_O (mi/h)");
    assert_approx(seg.get_speed_avg(), 56.0, 0.5, "S (mi/h)");
}

/// HCM Chapter 28, Example Problem 3 (first ramp): one-lane on-ramp on an
/// eight-lane freeway (FFS = 65 mi/h, ramp FFS = 30 mi/h, L_A = 260 ft).
/// The lane-distribution check fails and Equation 14-19 governs:
/// v_12a = v_F / 2.50 = 2,570 pc/h.
///
/// The published all-lane average speed (58.8 mi/h) is not reproducible from
/// the published S_R/S_O/flows via Exhibit 14-15 (which give 58.2 mi/h), so
/// this test asserts the component speeds instead.
#[test]
fn example_problem_3_on_ramp_eight_lane() {
    let mut seg = load_case("case3.json");
    let los = seg.run_analysis();

    assert_approx(seg.get_flow_freeway(), 6425.0, 5.0, "v_F (pc/h)");
    assert_approx(seg.get_flow_ramp(), 458.0, 2.0, "v_R (pc/h)");

    // Step 2: v_F/S_FR = 214 > 72 -> P_FM = 0.2178 - 0.000125 v_R = 0.16;
    // Equation 14-19 adjustment applies.
    assert_approx(seg.p_f.unwrap(), 0.160, 0.002, "P_FM");
    assert_approx(seg.get_v12(), 2570.0, 6.0, "v_12a (pc/h)");
    assert_approx(seg.get_vr12(), 3028.0, 8.0, "v_R12 (pc/h)");

    // Step 3
    assert_approx(seg.get_capacity_freeway(), 9400.0, 1e-9, "freeway capacity (pc/h)");
    assert_approx(seg.get_capacity_ramp(), 1900.0, 1e-9, "ramp capacity (pc/h)");
    assert_eq!(seg.demand_exceeds_capacity, Some(false));

    // Step 4 (Equation 14-22, Exhibit 14-3)
    assert_approx(seg.get_density(), 27.2, 0.5, "D_R (pc/mi/ln)");
    assert_eq!(los, LevelOfService::C);

    // Step 5 (Exhibit 14-13): S_R = 56.2, S_O = 59.9
    assert_approx(seg.get_speed_ramp(), 56.2, 0.5, "S_R (mi/h)");
    assert_approx(seg.get_speed_outer().unwrap(), 59.9, 0.5, "S_O (mi/h)");
}

/// HCM Chapter 28, Example Problem 4: single-lane, left-hand on-ramp on a
/// six-lane freeway (FFS = 65 mi/h, ramp FFS = 30 mi/h, L_A = 820 ft).
/// v_12 is computed as for a right-hand ramp and multiplied by the
/// Exhibit 14-18 factor (1.12), giving v_23 = 3,211 pc/h.
#[test]
fn example_problem_4_left_hand_on_ramp() {
    let mut seg = load_case("case4.json");
    let los = seg.run_analysis();

    assert_approx(seg.get_flow_freeway(), 4779.0, 5.0, "v_F (pc/h)");
    assert_approx(seg.get_flow_ramp(), 561.0, 2.0, "v_R (pc/h)");

    // Step 2: base P_FM = 0.5775 + 0.000028 x 820 = 0.600 (Equation 14-3);
    // Exhibit 14-18 left-hand factor 1.12 applied to v_12.
    assert_approx(seg.p_f.unwrap(), 0.600, 0.002, "P_FM");
    assert_approx(seg.get_v12(), 3211.0, 8.0, "v_23 (pc/h)");
    assert_approx(seg.get_vr12(), 3772.0, 10.0, "v_R23 (pc/h)");

    // Step 3
    assert_approx(seg.get_capacity_freeway(), 7050.0, 1e-9, "freeway capacity (pc/h)");
    assert_approx(seg.get_capacity_ramp(), 1900.0, 1e-9, "ramp capacity (pc/h)");
    assert_eq!(seg.demand_exceeds_capacity, Some(false));

    // Step 4 (Equation 14-22 with v_23, Exhibit 14-3)
    assert_approx(seg.get_density(), 29.5, 0.5, "D_R (pc/mi/ln)");
    assert_eq!(los, LevelOfService::D);

    // Step 5 (Exhibits 14-13/14-15): S_R = 54.8, S_O = 61.2, S = 56.5
    assert_approx(seg.get_speed_ramp(), 54.8, 0.5, "S_R (mi/h)");
    assert_approx(seg.get_speed_outer().unwrap(), 61.2, 0.5, "S_O (mi/h)");
    assert_approx(seg.get_speed_avg(), 56.5, 0.5, "S (mi/h)");
}
