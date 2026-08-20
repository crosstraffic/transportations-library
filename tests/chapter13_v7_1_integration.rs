//! HCM Edition 7.1, Chapter 13 (Freeway Weaving Segments), validated against the worked example
//! problems of the Edition 7.1 replacement Chapter 27.
//!
//! These exercise the public versioned API: a `WeavingSegment` carrying `HcmVersion::V7_1` run
//! through `run_analysis`. Every published intermediate value is asserted, not just the LOS
//! letter, because a wrong speed and a wrong capacity can still land on the right letter.
//!
//! Tolerances follow the manual's own rounding. Chapter 27 carries flow rates rounded to whole
//! pc/h and f_HV to two or three decimals, so a full-precision implementation lands a few pc/h
//! away from the printed totals.

use transportations_library::hcm::common::{HcmVersion, LevelOfService};
use transportations_library::hcm::weaving::v7_1::WeavingClass;
use transportations_library::hcm::weaving::weaving::{TerrainType, WeavingSegment, WeavingType};

fn approx(actual: f64, expected: f64, tol: f64, label: &str) {
    assert!(
        (actual - expected).abs() <= tol,
        "{label}: got {actual}, expected {expected} (+-{tol})"
    );
}

/// Example Problem 1 (Exhibit 27-2): LOS of a complex weave. A "Complex 0-1" configuration on a
/// four-lane urban freeway.
#[test]
fn example_problem_1_complex_weave() {
    let mut seg = WeavingSegment {
        version: HcmVersion::V7_1,
        weaving_type: WeavingType::OneSided,
        length_short: 1500.0,
        num_lanes: 4,
        ffs: 65.0,
        v_ff: 1815.0,
        v_fr: 692.0,
        v_rf: 1037.0,
        v_rr: 1297.0,
        phf: 0.91,
        heavy_vehicle_pct: 0.05,
        terrain: TerrainType::Level,
        lc_rf: 0,
        lc_fr: 1,
        nw_rf: 2,
        nw_fr: 1,
        ..Default::default()
    };

    let los = seg.run_analysis();
    let a = seg.analysis_v7_1.as_ref().expect("7.1 analysis stored");

    assert_eq!(a.class, WeavingClass::Complex);
    approx(a.f_hv, 0.952, 0.001, "f_HV");
    approx(a.flows.v_ff, 2095.0, 1.0, "v_FF");
    approx(a.flows.v_fr, 799.0, 1.0, "v_FR");
    approx(a.flows.v_rf, 1197.0, 1.0, "v_RF");
    approx(a.flows.v_rr, 1497.0, 1.0, "v_RR");
    approx(a.flow_per_lane, 1397.0, 1.0, "v/N");
    approx(a.breakpoint_adj, 1400.0, 1e-9, "BP_adj");
    approx(a.capacity_basic_adj, 2350.0, 1e-9, "C_b,adj");
    approx(a.speed_basic, 65.0, 1e-9, "S_b");
    approx(a.weaving_intensity, 0.006336, 5e-6, "W");
    approx(a.speed_impedance, 5.68, 0.02, "SIW");
    approx(a.speed_avg.unwrap(), 59.32, 0.02, "S_o");
    approx(a.capacity_per_lane.unwrap(), 1866.0, 2.0, "C_W");
    approx(a.dc_ratio.unwrap(), 0.75, 0.005, "d/c");
    approx(a.density, 23.6, 0.1, "D");
    assert_eq!(los, LevelOfService::C);

    // The shared fields carry the same numbers as the typed 7.1 result.
    approx(seg.get_speed_avg(), 59.32, 0.02, "shared speed");
    approx(seg.get_density(), 23.6, 0.1, "shared density");
    assert_eq!(seg.get_los(), Some(LevelOfService::C));
}

/// Example Problem 2 (Exhibit 27-4): LOS of a simple weave. Demands are already flow rates in
/// pc/h, so no Chapter 12 adjustment applies, and the per-lane flow sits above the breakpoint so
/// the basic-segment speed comes off the curved part of Equation 12-1.
#[test]
fn example_problem_2_simple_weave() {
    let mut seg = WeavingSegment {
        version: HcmVersion::V7_1,
        weaving_type: WeavingType::OneSided,
        length_short: 1000.0,
        num_lanes: 4,
        ffs: 75.0,
        v_ff: 4000.0,
        v_fr: 600.0,
        v_rf: 300.0,
        v_rr: 100.0,
        phf: 1.00,
        heavy_vehicle_pct: 0.0,
        terrain: TerrainType::Level,
        // Every simple weave has all four configuration parameters equal to 1.
        lc_rf: 1,
        lc_fr: 1,
        nw_rf: 1,
        nw_fr: 1,
        ..Default::default()
    };

    let los = seg.run_analysis();
    let a = seg.analysis_v7_1.as_ref().expect("7.1 analysis stored");

    assert_eq!(a.class, WeavingClass::Simple);
    approx(a.f_hv, 1.0, 1e-9, "f_HV");
    approx(a.flow_total, 5000.0, 1e-9, "v");
    approx(a.flow_per_lane, 1250.0, 1e-9, "v/N");
    approx(a.breakpoint_adj, 1000.0, 1e-9, "BP_adj");
    // 2,200 + 10(75 - 50) = 2,450, capped at the Exhibit 12-4 maximum of 2,400.
    approx(a.capacity_basic_adj, 2400.0, 1e-9, "C_b,adj");
    approx(a.speed_basic, 74.31, 0.01, "S_b");
    approx(a.weaving_intensity, 0.004814, 5e-6, "W");
    approx(a.speed_impedance, 3.61, 0.01, "SIW");
    approx(a.speed_avg.unwrap(), 70.70, 0.01, "S_o");
    approx(a.capacity_per_lane.unwrap(), 1992.0, 2.0, "C_W");
    approx(a.dc_ratio.unwrap(), 0.63, 0.005, "d/c");
    approx(a.density, 17.7, 0.05, "D");
    assert_eq!(los, LevelOfService::B);
}

/// Example Problem 3 (Exhibit 27-6): LOS of a two-sided weaving segment. Only the ramp-to-ramp
/// flow weaves, and its configuration (three lanes, single-lane ramps) is the one that reduces to
/// the simplified Equation 13-14.
#[test]
fn example_problem_3_two_sided_weave() {
    let mut seg = WeavingSegment {
        version: HcmVersion::V7_1,
        weaving_type: WeavingType::TwoSided,
        length_short: 750.0,
        num_lanes: 3,
        ffs: 60.0,
        v_ff: 3500.0,
        v_fr: 250.0,
        v_rf: 100.0,
        v_rr: 300.0,
        phf: 0.94,
        heavy_vehicle_pct: 0.11,
        terrain: TerrainType::Rolling,
        lc_rr: 2,
        nw_rr: 0,
        ..Default::default()
    };

    let los = seg.run_analysis();
    let a = seg.analysis_v7_1.as_ref().expect("7.1 analysis stored");

    assert_eq!(a.class, WeavingClass::TwoSided);
    approx(a.f_hv, 0.82, 0.001, "f_HV");
    approx(a.flows.v_ff, 4541.0, 2.0, "v_FF");
    approx(a.flows.v_fr, 324.0, 1.0, "v_FR");
    approx(a.flows.v_rf, 130.0, 1.0, "v_RF");
    approx(a.flows.v_rr, 389.0, 1.0, "v_RR");
    approx(a.flow_per_lane, 1795.0, 1.0, "v/N");
    approx(a.breakpoint_adj, 1600.0, 1e-9, "BP_adj");
    approx(a.capacity_basic_adj, 2300.0, 1e-9, "C_b,adj");
    approx(a.speed_basic, 59.31, 0.02, "S_b");
    approx(a.weaving_intensity, 0.005199, 5e-6, "W");
    approx(a.speed_impedance, 6.73, 0.02, "SIW");
    approx(a.speed_avg.unwrap(), 52.58, 0.03, "S_o");
    approx(a.capacity_per_lane.unwrap(), 1827.0, 3.0, "C_W");
    approx(a.dc_ratio.unwrap(), 0.98, 0.005, "d/c");
    approx(a.density, 34.1, 0.1, "D");
    assert_eq!(los, LevelOfService::E);
}

/// Example Problem 4, Trial 1 (Exhibit 27-9, pp. 27-17 to 27-18): design of a complex weave for a
/// desired LOS. Five lanes carried straight through, so the freeway-to-ramp movement needs two
/// lane changes and can start from no lane that weaves in one. A "Complex 0-2" configuration.
///
/// The trial skips Step 4, so the manual prints no capacity for this segment and none is asserted.
#[test]
fn example_problem_4_trial_1_complex_0_2() {
    let mut seg = WeavingSegment {
        version: HcmVersion::V7_1,
        weaving_type: WeavingType::OneSided,
        length_short: 1320.0,
        num_lanes: 5,
        ffs: 60.0,
        // All demands are already flow rates in pc/h under equivalent base conditions (p. 27-17).
        v_ff: 2000.0,
        v_fr: 1450.0,
        v_rf: 1500.0,
        v_rr: 1750.0,
        phf: 1.00,
        heavy_vehicle_pct: 0.0,
        terrain: TerrainType::Level,
        lc_rf: 0,
        lc_fr: 2,
        nw_rf: 2,
        nw_fr: 0,
        ..Default::default()
    };

    let los = seg.run_analysis();
    let a = seg.analysis_v7_1.as_ref().expect("7.1 analysis stored");

    assert_eq!(a.class, WeavingClass::Complex);
    approx(a.flow_total, 6700.0, 1e-9, "v (p. 27-17)");
    approx(a.flow_per_lane, 1340.0, 1e-9, "v/N (p. 27-17)");
    approx(a.breakpoint_adj, 1600.0, 1e-9, "BP_adj (p. 27-18)");
    approx(a.capacity_basic_adj, 2300.0, 1e-9, "C_b,adj (p. 27-18)");
    // Below the breakpoint, so the equivalent basic segment runs at the adjusted FFS of 60 mi/h.
    // The manual's prose at p. 27-18 says "the FFS of 65 mi/h" here, which contradicts both this
    // problem's stated FFS of 60 and its own S_o arithmetic on p. 27-19. Recorded in
    // docs/hcm/VERIFICATION.md.
    approx(a.speed_basic, 60.0, 1e-9, "S_b (p. 27-18)");
    approx(a.weaving_intensity, 0.008040, 5e-6, "W (p. 27-18)");
    approx(a.speed_impedance, 6.75, 0.01, "SIW (p. 27-19)");
    approx(a.speed_avg.unwrap(), 53.25, 0.01, "S_o (p. 27-19)");
    approx(a.density, 25.2, 0.05, "D (p. 27-19)");
    assert_eq!(los, LevelOfService::D);
}

/// Example Problem 4, Trial 2 (Exhibit 27-10, pp. 27-19 to 27-20): the same segment after a lane is
/// added to the exit-ramp leg, which drops LC_FR to 1 and raises NW_FR to 1. A "Complex 0-1"
/// configuration, and the design that reaches the target LOS C.
#[test]
fn example_problem_4_trial_2_complex_0_1() {
    let mut seg = WeavingSegment {
        version: HcmVersion::V7_1,
        weaving_type: WeavingType::OneSided,
        length_short: 1320.0,
        num_lanes: 5,
        ffs: 60.0,
        v_ff: 2000.0,
        v_fr: 1450.0,
        v_rf: 1500.0,
        v_rr: 1750.0,
        phf: 1.00,
        heavy_vehicle_pct: 0.0,
        terrain: TerrainType::Level,
        // Only the freeway-to-ramp configuration changes between the trials (p. 27-19).
        lc_rf: 0,
        lc_fr: 1,
        nw_rf: 2,
        nw_fr: 1,
        ..Default::default()
    };

    let los = seg.run_analysis();
    let a = seg.analysis_v7_1.as_ref().expect("7.1 analysis stored");

    assert_eq!(a.class, WeavingClass::Complex);
    approx(a.flow_per_lane, 1340.0, 1e-9, "v/N (p. 27-19)");
    approx(a.speed_basic, 60.0, 1e-9, "S_b (p. 27-19)");
    approx(a.weaving_intensity, 0.006701, 5e-6, "W (p. 27-19)");
    approx(a.speed_impedance, 5.63, 0.01, "SIW (p. 27-19)");
    approx(a.speed_avg.unwrap(), 54.37, 0.01, "S_o (p. 27-19)");
    approx(a.density, 24.6, 0.05, "D (p. 27-20)");
    // The manual's Trial 2 prose cites Equation 13-22 and Exhibit 13-6 here, and heads the step
    // "Trial 1". Both citations belong to Trial 1's Equation 13-21 and Exhibit 13-7, and the
    // letter below is the one Exhibit 13-7 gives. Recorded in docs/hcm/VERIFICATION.md.
    assert_eq!(los, LevelOfService::C);
}

/// Chapter 28's Example Problem 3 discussion (pp. 28-16 to 28-18) re-analyzes its merge/diverge
/// pair as a weaving segment, formed by connecting the two ramps with an auxiliary lane. It is a
/// Chapter 13 problem carried in the Chapter 28 text, and it is the only published Edition 7.1
/// simple-weave case that also prints a weaving capacity from Equation 13-16.
#[test]
fn chapter_28_example_problem_3_auxiliary_lane_weave() {
    let mut seg = WeavingSegment {
        version: HcmVersion::V7_1,
        weaving_type: WeavingType::OneSided,
        // Four mainline lanes plus the added auxiliary lane, striped for weaving over the full
        // 1,300 ft between the two ramps (p. 28-16).
        length_short: 1300.0,
        num_lanes: 5,
        ffs: 65.0,
        // Already flow rates in pc/h, carried over from Example Problem 3's Step 1. The
        // ramp-to-ramp movement is set to zero as the most conservative assumption (p. 28-16).
        v_ff: 5723.0,
        v_fr: 702.0,
        v_rf: 458.0,
        v_rr: 0.0,
        phf: 1.00,
        heavy_vehicle_pct: 0.0,
        terrain: TerrainType::Level,
        lc_rf: 1,
        lc_fr: 1,
        nw_rf: 1,
        nw_fr: 1,
        ..Default::default()
    };

    let los = seg.run_analysis();
    let a = seg.analysis_v7_1.as_ref().expect("7.1 analysis stored");

    assert_eq!(a.class, WeavingClass::Simple);
    approx(a.flow_total, 6883.0, 1e-9, "v (p. 28-16)");
    approx(a.flow_per_lane, 1377.0, 1.0, "v/N (p. 28-17)");
    approx(a.breakpoint_adj, 1400.0, 1e-9, "BP_adj (p. 28-13)");
    approx(a.capacity_basic_adj, 2350.0, 1e-9, "C_b,adj (p. 28-13)");
    // 1,377 pc/h/ln is below the 1,400 pc/h/ln breakpoint, so S_b is the adjusted FFS (p. 28-17).
    approx(a.speed_basic, 65.0, 1e-9, "S_b (p. 28-17)");
    approx(a.weaving_intensity, 0.004546, 5e-6, "W (p. 28-17)");
    approx(a.speed_impedance, 3.99, 0.01, "SIW (p. 28-17)");
    approx(a.speed_avg.unwrap(), 61.01, 0.01, "S_o (p. 28-17)");
    approx(a.capacity_per_lane.unwrap(), 1917.0, 2.0, "C_W (p. 28-18)");
    approx(a.dc_ratio.unwrap(), 0.72, 0.005, "d/c (p. 28-18)");
    approx(a.density, 22.6, 0.05, "D (p. 28-18)");
    assert_eq!(los, LevelOfService::C);
}

/// The two editions are genuinely different models, not a refinement: the same segment analyzed
/// under each produces different speeds, densities, and (here) different LOS letters. This is the
/// behavior that makes the edition a required input rather than a default anyone can ignore.
#[test]
fn the_two_editions_disagree_on_the_same_segment() {
    let base = WeavingSegment {
        weaving_type: WeavingType::OneSided,
        length_short: 1500.0,
        num_lanes: 4,
        ffs: 65.0,
        v_ff: 1815.0,
        v_fr: 692.0,
        v_rf: 1037.0,
        v_rr: 1297.0,
        phf: 0.91,
        heavy_vehicle_pct: 0.05,
        terrain: TerrainType::Level,
        lc_rf: 0,
        lc_fr: 1,
        nw_rf: 2,
        nw_fr: 1,
        num_weaving_lanes: 2,
        ..Default::default()
    };

    let mut v7 = base.clone();
    v7.version = HcmVersion::V7;
    v7.run_analysis();

    let mut v71 = base.clone();
    v71.version = HcmVersion::V7_1;
    v71.run_analysis();

    assert!(
        (v7.get_density() - v71.get_density()).abs() > 1.0,
        "editions should not agree by accident: {} vs {}",
        v7.get_density(),
        v71.get_density()
    );
    // Only the selected edition's own result object is populated.
    assert!(v7.analysis_v7_1.is_none());
    assert!(v71.analysis_v7_1.is_some());
}

/// The default edition is the 7th, so code written before Edition 7.1 existed keeps its numbers.
#[test]
fn default_edition_is_the_seventh() {
    assert_eq!(WeavingSegment::default().version, HcmVersion::V7);
}
