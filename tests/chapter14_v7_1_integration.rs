//! HCM Edition 7.1, Chapter 14 (Freeway Merge and Diverge Segments), validated against the worked
//! example problems of the Edition 7.1 replacement Chapter 28.
//!
//! These exercise the public versioned API: a `RampSegment` carrying `HcmVersion::V7_1` run
//! through `run_analysis`. Tolerances follow the manual's own rounding of flow rates to whole pc/h
//! and f_HV to three decimals.

use transportations_library::hcm::common::{HcmVersion, LevelOfService};
use transportations_library::hcm::merge_diverge::merge_diverge::{
    RampLanes, RampSegment, RampSide, RampType, TerrainType,
};

fn approx(actual: f64, expected: f64, tol: f64, label: &str) {
    assert!(
        (actual - expected).abs() <= tol,
        "{label}: got {actual}, expected {expected} (+-{tol})"
    );
}

/// Example Problem 1: an isolated one-lane, right-hand on-ramp to a four-lane freeway. The
/// per-lane demand sits above the breakpoint, so the equivalent basic segment speed comes off the
/// curved part of Equation 12-1.
#[test]
fn example_problem_1_isolated_on_ramp() {
    let mut seg = RampSegment {
        version: HcmVersion::V7_1,
        ramp_type: RampType::OnRamp,
        ramp_side: RampSide::Right,
        ramp_lanes: RampLanes::OneLane,
        freeway_lanes: 2,
        freeway_ffs: 60.0,
        ramp_ffs: 45.0,
        accel_lane_length: Some(740.0),
        decel_lane_length: None,
        freeway_demand: 2500.0,
        ramp_demand: 535.0,
        phf: 0.90,
        heavy_vehicle_pct: 0.05,
        terrain: TerrainType::Level,
        ..Default::default()
    };

    let los = seg.run_analysis();
    let a = seg.analysis_v7_1.as_ref().expect("7.1 analysis stored");

    approx(a.flow_freeway, 2918.0, 2.0, "v_F");
    approx(a.flow_ramp, 624.0, 1.0, "v_R");
    approx(a.flow_influence, 3542.0, 3.0, "v");
    approx(a.flow_per_lane, 1771.0, 2.0, "v/N");
    approx(a.breakpoint_adj, 1600.0, 1e-9, "BP_adj");
    approx(a.capacity_basic_adj, 2300.0, 1e-9, "C_b,adj");
    approx(a.speed_basic, 59.47, 0.02, "S_b");
    approx(a.speed_impedance, 4.37, 0.02, "SIM");
    approx(a.speed_avg.unwrap(), 55.10, 0.03, "S_M");
    approx(a.capacity_per_lane.unwrap(), 1882.0, 3.0, "C_M");
    approx(a.dc_ratio.unwrap(), 0.94, 0.005, "d/c");

    // The other two capacity checks: Exhibit 14-8 for the downstream freeway, Exhibit 14-10 for
    // the ramp roadway. Both are satisfied.
    approx(a.capacity_neighboring_freeway, 4600.0, 1e-9, "downstream freeway capacity");
    approx(a.capacity_ramp_roadway, 2100.0, 1e-9, "ramp roadway capacity");
    assert!(!a.demand_exceeds_capacity);

    approx(a.density, 32.1, 0.1, "D_M");
    assert_eq!(los, Some(LevelOfService::E));
}

/// Example Problem 2, first off-ramp: one of two adjacent one-lane, right-hand off-ramps on a
/// six-lane freeway. Edition 7.1 analyzes each ramp independently and applies the worse LOS to the
/// overlapping influence area.
#[test]
fn example_problem_2_first_off_ramp() {
    let mut seg = RampSegment {
        version: HcmVersion::V7_1,
        ramp_type: RampType::OffRamp,
        ramp_side: RampSide::Right,
        ramp_lanes: RampLanes::OneLane,
        freeway_lanes: 3,
        freeway_ffs: 60.0,
        ramp_ffs: 40.0,
        accel_lane_length: None,
        decel_lane_length: Some(500.0),
        freeway_demand: 4500.0,
        ramp_demand: 300.0,
        phf: 0.95,
        heavy_vehicle_pct: 0.075,
        terrain: TerrainType::Level,
        ..Default::default()
    };

    let los = seg.run_analysis();
    let a = seg.analysis_v7_1.as_ref().expect("7.1 analysis stored");

    approx(a.flow_freeway, 5093.0, 3.0, "v_F");
    approx(a.flow_ramp, 340.0, 1.0, "v_R1");
    // A diverge's influence-area flow is the mainline flow, which already contains the exiting
    // vehicles, so v_R is not added.
    approx(a.flow_influence, 5093.0, 3.0, "v_1");
    approx(a.flow_per_lane, 1698.0, 2.0, "v_1/N");
    approx(a.speed_basic, 59.83, 0.02, "S_b");
    approx(a.speed_impedance, 2.04, 0.02, "SID");
    approx(a.speed_avg.unwrap(), 57.79, 0.03, "S_D");
    approx(a.capacity_per_lane.unwrap(), 1940.0, 3.0, "C_D");
    approx(a.capacity_neighboring_freeway, 6900.0, 1e-9, "upstream freeway capacity");
    approx(a.capacity_ramp_roadway, 2000.0, 1e-9, "ramp roadway capacity");
    assert!(!a.demand_exceeds_capacity);
    approx(a.density, 29.4, 0.1, "D_D");
    assert_eq!(los, Some(LevelOfService::D));
}

/// Example Problem 2, second off-ramp. Its per-lane demand of 1,584 pc/h/ln falls just below the
/// 1,600 pc/h/ln breakpoint, so the equivalent basic segment runs at the adjusted FFS.
#[test]
fn example_problem_2_second_off_ramp() {
    let mut seg = RampSegment {
        version: HcmVersion::V7_1,
        ramp_type: RampType::OffRamp,
        ramp_side: RampSide::Right,
        ramp_lanes: RampLanes::OneLane,
        freeway_lanes: 3,
        freeway_ffs: 60.0,
        ramp_ffs: 25.0,
        accel_lane_length: None,
        decel_lane_length: Some(300.0),
        // The mainline flow reaching the second ramp is the original demand less what left at the
        // first: 4,500 - 300 = 4,200 veh/h.
        freeway_demand: 4200.0,
        ramp_demand: 500.0,
        phf: 0.95,
        heavy_vehicle_pct: 0.075,
        terrain: TerrainType::Level,
        ..Default::default()
    };

    let los = seg.run_analysis();
    let a = seg.analysis_v7_1.as_ref().expect("7.1 analysis stored");

    approx(a.flow_freeway, 4753.0, 3.0, "v_2");
    approx(a.flow_ramp, 566.0, 1.0, "v_R2");
    approx(a.flow_per_lane, 1584.0, 2.0, "v_2/N");
    // Below the breakpoint, so the basic segment runs at the adjusted FFS.
    approx(a.speed_basic, 60.0, 1e-9, "S_b");
    approx(a.speed_impedance, 4.04, 0.02, "SID");
    approx(a.speed_avg.unwrap(), 55.96, 0.03, "S_D");
    approx(a.capacity_per_lane.unwrap(), 1874.0, 3.0, "C_D");
    approx(a.capacity_ramp_roadway, 1900.0, 1e-9, "ramp roadway capacity");
    assert!(!a.demand_exceeds_capacity);
    approx(a.density, 28.3, 0.1, "D_D");
    assert_eq!(los, Some(LevelOfService::D));
}

/// The two editions are genuinely different models. Under the 7th Edition this junction's density
/// comes from the Lanes 1-2 flow; under Edition 7.1 it comes from the whole cross-section and a
/// speed impedance, and the answers differ.
#[test]
fn the_two_editions_disagree_on_the_same_junction() {
    let base = RampSegment {
        ramp_type: RampType::OnRamp,
        ramp_side: RampSide::Right,
        ramp_lanes: RampLanes::OneLane,
        freeway_lanes: 3,
        freeway_ffs: 65.0,
        ramp_ffs: 40.0,
        accel_lane_length: Some(700.0),
        decel_lane_length: None,
        freeway_demand: 4000.0,
        ramp_demand: 600.0,
        phf: 0.92,
        heavy_vehicle_pct: 0.05,
        terrain: TerrainType::Level,
        ..Default::default()
    };

    let mut v7 = base.clone();
    v7.version = HcmVersion::V7;
    v7.run_analysis();

    let mut v71 = base.clone();
    v71.version = HcmVersion::V7_1;
    v71.run_analysis();

    assert!(
        (v7.get_density() - v71.get_density()).abs() > 0.5,
        "editions should not agree by accident: {} vs {}",
        v7.get_density(),
        v71.get_density()
    );
    assert!(v7.analysis_v7_1.is_none());
    assert!(v71.analysis_v7_1.is_some());
}

/// The default edition is the 7th, so code written before Edition 7.1 existed keeps its numbers.
#[test]
fn default_edition_is_the_seventh() {
    assert_eq!(RampSegment::default().version, HcmVersion::V7);
}

/// A major merge under capacity has no 7th Edition level of service: Chapter 14 checks its
/// capacity and stops. `run_analysis` returns None there rather than inventing a letter, and the
/// stored field agrees with the returned value.
#[test]
fn major_merge_under_capacity_has_no_seventh_edition_los() {
    let mut seg = RampSegment {
        version: HcmVersion::V7,
        ramp_type: RampType::MajorMerge,
        ramp_side: RampSide::Right,
        ramp_lanes: RampLanes::TwoLane,
        freeway_lanes: 3,
        freeway_ffs: 65.0,
        ramp_ffs: 50.0,
        accel_lane_length: Some(1000.0),
        freeway_demand: 3000.0,
        ramp_demand: 1200.0,
        phf: 0.95,
        heavy_vehicle_pct: 0.05,
        terrain: TerrainType::Level,
        ..Default::default()
    };

    assert_eq!(seg.run_analysis(), None);
    assert_eq!(seg.get_los(), None);

    // Edition 7.1 closes the hole: Exhibit 14-2 states its criteria apply to all ramp-freeway
    // junctions "and may also be applied to major merges and diverges".
    seg.version = HcmVersion::V7_1;
    let los = seg.run_analysis();
    assert!(los.is_some(), "Edition 7.1 defines LOS for a major merge");
    assert_eq!(seg.get_los(), los);
}
