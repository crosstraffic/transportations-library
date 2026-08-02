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

use transportations_library::hcm::weaving::weaving::{
    cross_weave_gp_capacity, service_flow_rate_ideal, service_volumes, DemandSplit, WeavingSegment,
};
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

/// HCM Chapter 27, Example Problem 4 (Trial 1): design of a major weaving
/// segment. A direct five-lane connection forces the freeway-to-ramp movement
/// to make two lane changes (N_WL = 2), so the weaving-flow capacity of 2,400 /
/// VR = 5,654 pc/h falls below the 6,950 pc/h demand and the segment fails.
#[test]
fn example_problem_4_trial1_design_los_f() {
    let mut seg = load_case("case4a.json");
    let los = seg.run_analysis();

    assert_approx(seg.get_flow_weaving(), 2950.0, 1.0, "v_W (pc/h)");
    assert_approx(seg.get_flow_nonweaving(), 4000.0, 1.0, "v_NW (pc/h)");
    assert_approx(seg.get_flow_total(), 6950.0, 1.0, "v (pc/h)");
    assert_approx(seg.get_volume_ratio(), 0.424, 0.001, "VR");

    assert_approx(seg.get_lc_min(), 2900.0, 1.0, "LC_MIN (lc/h)");
    assert_approx(seg.c_iwl.unwrap(), 1944.0, 5.0, "c_IWL (pc/h/ln)");
    // Weaving-flow criterion (Eq. 13-7/13-8) governs and is below demand.
    assert_approx(seg.capacity_weaving.unwrap(), 5654.0, 15.0, "c_W weaving (pc/h)");
    assert_approx(seg.get_capacity(), 5654.0, 15.0, "c_W (pc/h)");
    assert!(seg.get_vc_ratio() > 1.0, "demand should exceed capacity");
    assert_eq!(los, LevelOfService::F);
}

/// HCM Chapter 27, Example Problem 4 (Trial 2): adding a lane to the exit leg
/// drops the freeway-to-ramp movement to a single lane change and raises N_WL
/// to 3. The weaving-flow capacity climbs to 3,500 / VR = 8,255 pc/h and the
/// segment delivers the target LOS C.
#[test]
fn example_problem_4_trial2_design_los_c() {
    let mut seg = load_case("case4b.json");
    let los = seg.run_analysis();

    assert_approx(seg.get_lc_min(), 1450.0, 1.0, "LC_MIN (lc/h)");
    assert_approx(seg.get_l_max(), 5391.0, 5.0, "L_MAX (ft)");
    assert!(seg.is_weaving_segment());

    assert_approx(seg.c_iwl.unwrap(), 2064.0, 5.0, "c_IWL (pc/h/ln)");
    assert_approx(seg.get_capacity(), 8255.0, 15.0, "c_W (pc/h)");
    assert!(seg.get_vc_ratio() < 1.0, "demand should be below capacity");

    assert_approx(seg.lc_w.unwrap(), 1899.0, 5.0, "LC_W (lc/h)");
    assert_approx(seg.lc_nw.unwrap(), 403.0, 5.0, "LC_NW (lc/h)");
    assert_approx(seg.get_lc_all(), 2302.0, 8.0, "LC_ALL (lc/h)");

    assert_approx(seg.get_speed_weaving(), 56.8, 0.5, "S_W (mi/h)");
    assert_approx(seg.get_speed_nonweaving(), 57.9, 0.5, "S_NW (mi/h)");
    assert_approx(seg.get_speed_avg(), 57.4, 0.5, "S (mi/h)");

    assert_approx(seg.get_density(), 24.2, 0.5, "D (pc/mi/ln)");
    assert_eq!(los, LevelOfService::C);
}

/// HCM Chapter 27, Example Problem 5: constructing a service volume table.
/// The chapter method does not yield service flow rates directly; they are
/// found by raising demand until each LOS threshold density is reached. The
/// published SFI values (Exhibit 27-15) are rounded down to the nearest 100
/// pc/h, so the reconstructed values are checked to within one rounding bucket.
#[test]
fn example_problem_5_service_volume_table() {
    // Case geometry (one-sided major weave, FFS = 65, ID = 1, level terrain).
    // The ramp-to-freeway movement needs no lane change (LC_RF = 0); the
    // freeway-to-ramp movement needs two for N_WL = 2.
    let template = WeavingSegment {
        weaving_type:
            transportations_library::hcm::weaving::weaving::WeavingType::OneSided,
        facility_type:
            transportations_library::hcm::weaving::weaving::FacilityType::Freeway,
        num_lanes: 3,
        num_weaving_lanes: 2,
        ffs: 65.0,
        interchange_density: 1.0,
        lc_rf: 0,
        lc_fr: 2,
        lc_rr: 0,
        basic_freeway_capacity: 2350.0,
        ..Default::default()
    };
    let split = DemandSplit { ff: 0.65, rf: 0.15, fr: 0.12, rr: 0.08 };

    // Exhibit 27-15, N = 3, N_WL = 2, L_S = 1,500 ft.
    let mut seg = WeavingSegment { length_short: 1500.0, ..template.clone() };
    // LOS A (D = 10) -> 1,700 pc/h; B (D = 20) -> 3,200; C (D = 28) -> 4,300.
    let sfi_a = service_flow_rate_ideal(&seg, &split, 10.0);
    let sfi_b = service_flow_rate_ideal(&seg, &split, 20.0);
    let sfi_c = service_flow_rate_ideal(&seg, &split, 28.0);
    assert_approx(round_down_100(sfi_a), 1700.0, 100.0, "SFI LOS A");
    assert_approx(round_down_100(sfi_b), 3200.0, 100.0, "SFI LOS B");
    assert_approx(round_down_100(sfi_c), 4300.0, 100.0, "SFI LOS C");

    // The SFI at LOS E is the segment capacity (ideal). Confirm the segment
    // capacity sits in the published 6,100 pc/h neighbourhood for this cell.
    seg.phf = 1.0;
    seg.heavy_vehicle_pct = 0.0;
    seg.v_ff = split.ff * 6100.0;
    seg.v_rf = split.rf * 6100.0;
    seg.v_fr = split.fr * 6100.0;
    seg.v_rr = split.rr * 6100.0;
    seg.run_analysis();
    assert_approx(round_down_100(seg.get_capacity()), 6100.0, 100.0, "SFI LOS E (capacity)");

    // Prevailing-condition chain (Exhibits 27-16 through 27-18): 5% trucks on
    // level terrain (E_T = 2.0 -> f_HV = 0.952), PHF = 0.93, K = 0.08, D = 0.55.
    let f_hv = 1.0 / (1.0 + 0.05 * (2.0 - 1.0));
    let sv = service_volumes(4300.0, f_hv, 0.93, 0.08, 0.55);
    assert_approx(sv.sf, 4300.0 * f_hv, 0.01, "SF = SFI x f_HV");
    assert_approx(sv.sv, 4300.0 * f_hv * 0.93, 0.01, "SV = SF x PHF");
    assert_approx(sv.dsv, sv.sv / (0.08 * 0.55), 0.01, "DSV = SV / (K x D)");
}

/// HCM Chapter 27, Example Problem 6: an ML access segment with cross-weaving.
/// The access segment itself is analyzed as a one-sided ramp weave (LOS C),
/// while the adjacent GP merge segment loses capacity to the cross-weave
/// movement (Equation 13-24).
#[test]
fn example_problem_6_ml_access_cross_weave() {
    let mut seg = load_case("case6.json");
    let los = seg.run_analysis();

    assert_approx(seg.get_flow_weaving(), 900.0, 1.0, "v_W (pc/h)");
    assert_approx(seg.get_flow_nonweaving(), 3700.0, 1.0, "v_NW (pc/h)");
    assert_approx(seg.get_flow_total(), 4600.0, 1.0, "v (pc/h)");
    assert_approx(seg.get_volume_ratio(), 0.196, 0.001, "VR");

    assert_approx(seg.get_lc_min(), 900.0, 1.0, "LC_MIN (lc/h)");
    assert_approx(seg.get_l_max(), 4495.0, 5.0, "L_MAX (ft)");
    assert_approx(seg.c_iwl.unwrap(), 2121.0, 5.0, "c_IWL (pc/h/ln)");
    assert_approx(seg.get_capacity(), 8483.0, 10.0, "c_W (pc/h)");

    assert_approx(seg.lc_w.unwrap(), 1276.0, 5.0, "LC_W (lc/h)");
    assert_approx(seg.lc_nw.unwrap(), 805.0, 5.0, "LC_NW (lc/h)");
    assert_approx(seg.get_lc_all(), 2081.0, 8.0, "LC_ALL (lc/h)");

    assert_approx(seg.get_speed_weaving(), 53.7, 0.5, "S_W (mi/h)");
    assert_approx(seg.get_speed_nonweaving(), 53.0, 0.5, "S_NW (mi/h)");
    assert_approx(seg.get_speed_avg(), 53.1, 0.5, "S (mi/h)");

    assert_approx(seg.get_density(), 21.7, 0.5, "D (pc/mi/ln)");
    assert_eq!(los, LevelOfService::C);

    // Cross-weave capacity reduction on the GP merge segment (Lane Group Pair 1):
    // CW = 360/0.9 = 400 pc/h, L_cw-min = 1,000 ft, N_GP = 3.
    let cw = cross_weave_gp_capacity(400.0, 1000.0, 3, 7050.0).unwrap();
    assert_approx(cw.crf, 0.056, 0.001, "CRF");
}

/// HCM Chapter 27, Example Problem 7: an ML access segment with a downstream
/// off-ramp. Part 1 analyzes the access segment as a three-lane ramp weave
/// (LOS B); Part 2 applies the cross-weave capacity adjustment to the two GP
/// lanes feeding the off-ramp.
#[test]
fn example_problem_7_ml_access_downstream_offramp() {
    let mut seg = load_case("case7.json");
    let los = seg.run_analysis();

    assert_approx(seg.get_flow_weaving(), 300.0, 1.0, "v_W (pc/h)");
    assert_approx(seg.get_flow_total(), 4300.0, 1.0, "v (pc/h)");
    assert_approx(seg.get_volume_ratio(), 0.070, 0.001, "VR");

    assert_approx(seg.get_lc_min(), 300.0, 1.0, "LC_MIN (lc/h)");
    assert_approx(seg.get_l_max(), 3251.0, 5.0, "L_MAX (ft)");
    assert_approx(seg.c_iwl.unwrap(), 2228.0, 5.0, "c_IWL (pc/h/ln)");
    assert_approx(seg.get_capacity(), 6684.0, 10.0, "c_W (pc/h)");

    assert_approx(seg.lc_w.unwrap(), 462.0, 5.0, "LC_W (lc/h)");
    assert_approx(seg.lc_nw.unwrap(), 788.0, 5.0, "LC_NW (lc/h)");
    assert_approx(seg.get_lc_all(), 1250.0, 8.0, "LC_ALL (lc/h)");

    assert_approx(seg.get_speed_weaving(), 58.3, 0.5, "S_W (mi/h)");
    assert_approx(seg.get_speed_nonweaving(), 61.0, 0.5, "S_NW (mi/h)");
    assert_approx(seg.get_speed_avg(), 60.8, 0.5, "S (mi/h)");

    assert_approx(seg.get_density(), 23.6, 0.5, "D (pc/mi/ln)");
    // The published solution reports LOS B, citing a B/C boundary of 24
    // pc/mi/ln (Exhibit 13-6, multilane/C-D row: A<=12, B<=24, C<=32). Note the
    // manual applies the multilane/C-D thresholds to this ML access segment,
    // unlike Example Problem 6, which used the freeway thresholds (B<=20).
    // case7.json therefore sets facility_type = "multilane" to reproduce the
    // published result; see VERIFY-HCM note in the PR description.
    assert_eq!(los, LevelOfService::B);

    // Part 2: cross-weave on the GP lanes. CW = 100 pc/h, L_cw-min = 1,500 ft,
    // N_GP = 2, c_GP = 2,400 x 2 = 4,800 pc/h.
    let cw = cross_weave_gp_capacity(100.0, 1500.0, 2, 4800.0).unwrap();
    assert_approx(cw.crf, 0.0105, 0.0005, "CRF");
    assert_approx(cw.caf, 0.9895, 0.0005, "CAF");
    assert_approx(cw.c_gpa, 4750.0, 5.0, "c_GPA (pc/h)");
}

/// Round a flow rate down to the nearest 100, matching the HCM's service-volume
/// presentation convention (Chapter 27, Example Problem 5).
fn round_down_100(x: f64) -> f64 {
    (x / 100.0).floor() * 100.0
}
