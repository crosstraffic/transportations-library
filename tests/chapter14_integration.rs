//! Integration tests for HCM Chapter 14 (Freeway Merge and Diverge Segments)
//! against the published results of HCM Chapter 28 Example Problems 1-5.
//!
//! Tolerances: flows +-5 pc/h (published values are rounded to whole numbers
//! and the book carries rounded intermediates); speeds +-0.5 mi/h; densities
//! +-0.5 pc/mi/ln; LOS letters exact.

use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use transportations_library::hcm::merge_diverge::merge_diverge::{
    ramp_service_flow_rate_ideal, ramp_service_volumes, AdjacentRampType, RampLanes, RampSegment,
    RampSide, RampType, ServiceDemandBasis, TerrainType,
};
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

/// Build the HCM Chapter 28, Example Problem 5 geometry: an isolated single-lane
/// right-hand on-ramp on a six-lane freeway (FFS 70), ramp FFS 40, 1,000-ft
/// acceleration lane. Demands are supplied by the service-flow-rate search.
fn ep5_template() -> RampSegment {
    RampSegment {
        ramp_type: RampType::OnRamp,
        ramp_side: RampSide::Right,
        ramp_lanes: RampLanes::OneLane,
        freeway_lanes: 3,
        freeway_ffs: 70.0,
        ramp_ffs: 40.0,
        accel_lane_length: Some(1000.0),
        terrain: TerrainType::Level,
        adjacent_upstream: AdjacentRampType::None,
        adjacent_downstream: AdjacentRampType::None,
        ..Default::default()
    }
}

/// HCM Chapter 28, Example Problem 5, Case 1: ramp demand fixed at 10% of the
/// approaching freeway demand; service flow rates expressed as approaching
/// freeway flows (Exhibit 28-4). f_HV = 0.939 (6.5% trucks), f_p = 1, PHF = 0.87.
#[test]
fn example_problem_5_case1_approaching_freeway_demand() {
    let template = ep5_template();
    let basis = ServiceDemandBasis::ApproachingFreeway { ramp_fraction: 0.10 };
    let f_hv = 1.0 / (1.0 + 0.065 * (2.0 - 1.0)); // = 0.939
    let phf = 0.87;

    // LOS A-C: solve v_F at the threshold densities (Equation 14-22). The book
    // solves the linearized equation with a rounded slope (0.005454 vs. the
    // exact 0.0054569), so its published values run a few pc/h high at the
    // larger flows (e.g. LOS C: 5,280 published vs. 5,277 exact); tolerance is
    // widened to +-6 pc/h to absorb that rounded intermediate.
    let sfi_a = ramp_service_flow_rate_ideal(&template, &basis, 10.0).unwrap();
    let sfi_b = ramp_service_flow_rate_ideal(&template, &basis, 20.0).unwrap();
    let sfi_c = ramp_service_flow_rate_ideal(&template, &basis, 28.0).unwrap();
    assert_approx(sfi_a, 1979.0, 6.0, "v_F SFI LOS A (pc/h)");
    assert_approx(sfi_b, 3813.0, 6.0, "v_F SFI LOS B (pc/h)");
    assert_approx(sfi_c, 5280.0, 6.0, "v_F SFI LOS C (pc/h)");

    // Prevailing SF and SV for LOS A (Exhibit 28-4: 1,858 veh/h, 1,616 veh/h).
    let (sf_a, sv_a) = ramp_service_volumes(sfi_a, f_hv, 1.0, phf);
    assert_approx(sf_a, 1858.0, 3.0, "SF LOS A (veh/h)");
    assert_approx(sv_a, 1616.0, 3.0, "SV LOS A (veh/h)");
    let (_, sv_c) = ramp_service_volumes(sfi_c, f_hv, 1.0, phf);
    assert_approx(sv_c, 4313.0, 3.0, "SV LOS C (veh/h)");

    // LOS E is a capacity limit: downstream freeway reaches 7,200 pc/h, so with
    // v_R = 0.10 v_F, v_F = 7,200 / 1.10 = 6,545 pc/h (ramp 655 < 2,000 cap).
    let mut probe = template.clone();
    probe.phf = 1.0;
    probe.heavy_vehicle_pct = 0.0;
    probe.run_analysis();
    let cap_freeway = probe.get_capacity_freeway();
    let cap_ramp = probe.get_capacity_ramp();
    assert_approx(cap_freeway, 7200.0, 1.0, "downstream freeway capacity (pc/h)");
    assert_approx(cap_ramp, 2000.0, 1.0, "ramp capacity (pc/h)");
    let sfi_e = cap_freeway / 1.10;
    assert_approx(sfi_e, 6545.0, 2.0, "v_F SFI LOS E (pc/h)");
    assert!(0.10 * sfi_e < cap_ramp, "ramp flow at LOS E must stay under capacity");

    // LOS D is unachievable: its threshold flow (v_F ~ 6,563) exceeds the LOS E
    // capacity, so capacity is reached before density 35 pc/mi/ln (Exhibit 28-4
    // reports NA for LOS D).
    let sfi_d = ramp_service_flow_rate_ideal(&template, &basis, 35.0).unwrap();
    assert!(
        sfi_d > sfi_e,
        "LOS D threshold ({sfi_d}) should exceed the LOS E capacity ({sfi_e}), making D unachievable"
    );
}

/// HCM Chapter 28, Example Problem 5, Case 2: approaching freeway demand held at
/// 4,000 veh/h; service flow rates expressed as ramp demands (Exhibit 28-5).
#[test]
fn example_problem_5_case2_fixed_freeway_demand() {
    let template = ep5_template();
    let f_hv = 1.0 / (1.0 + 0.065 * (2.0 - 1.0)); // = 0.939
    let phf = 0.87;
    // Convert the fixed 4,000 veh/h freeway demand to pc/h under ideal conditions.
    let v_f = 4000.0 / (phf * f_hv); // = 4,896 pc/h
    assert_approx(v_f, 4896.0, 2.0, "v_F ideal (pc/h)");
    let basis = ServiceDemandBasis::FixedFreeway { v_f };

    // LOS A and B are unachievable: the minimum density (at zero ramp flow) is
    // already 22.33 pc/mi/ln, above the 10 and 20 thresholds (Exhibit 28-5 NA).
    assert!(ramp_service_flow_rate_ideal(&template, &basis, 10.0).is_none());
    assert!(ramp_service_flow_rate_ideal(&template, &basis, 20.0).is_none());

    // LOS C and D: solve for the ramp flow at the threshold densities.
    let sfi_c = ramp_service_flow_rate_ideal(&template, &basis, 28.0).unwrap();
    let sfi_d = ramp_service_flow_rate_ideal(&template, &basis, 35.0).unwrap();
    assert_approx(sfi_c, 772.0, 3.0, "v_R SFI LOS C (pc/h)");
    assert_approx(sfi_d, 1726.0, 3.0, "v_R SFI LOS D (pc/h)");

    let (sf_c, sv_c) = ramp_service_volumes(sfi_c, f_hv, 1.0, phf);
    assert_approx(sf_c, 725.0, 3.0, "SF LOS C (veh/h)");
    assert_approx(sv_c, 631.0, 3.0, "SV LOS C (veh/h)");
    let (sf_d, sv_d) = ramp_service_volumes(sfi_d, f_hv, 1.0, phf);
    assert_approx(sf_d, 1621.0, 3.0, "SF LOS D (veh/h)");
    assert_approx(sv_d, 1410.0, 3.0, "SV LOS D (veh/h)");

    // LOS E: the downstream-capacity ramp flow (7,200 - 4,896 = 2,304 pc/h)
    // exceeds the 2,000 pc/h ramp capacity, so LOS E is capped at the ramp
    // capacity (Exhibit 28-5: SFI 2,000 -> SF 1,878 -> SV 1,633 veh/h).
    let mut probe = template.clone();
    probe.phf = 1.0;
    probe.heavy_vehicle_pct = 0.0;
    probe.run_analysis();
    let sfi_e = (probe.get_capacity_freeway() - v_f).min(probe.get_capacity_ramp());
    assert_approx(sfi_e, 2000.0, 1.0, "v_R SFI LOS E (pc/h)");
    let (sf_e, sv_e) = ramp_service_volumes(sfi_e, f_hv, 1.0, phf);
    assert_approx(sf_e, 1878.0, 3.0, "SF LOS E (veh/h)");
    assert_approx(sv_e, 1633.0, 3.0, "SV LOS E (veh/h)");
}
