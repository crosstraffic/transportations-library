//! Integration tests for the HCM Chapter 16 (Urban Street Facilities)
//! motorized vehicle methodology, against the published answers of HCM
//! 7th Edition, Chapter 29, Section 5, Example Problem 1 (Exhibits 29-39
//! through 29-49) and Chapter 30, Section 8, Example Problem 1.
//!
//! Fixtures:
//! * `case1.json` — Chapter 29 EP1, eastbound: published per-segment
//!   Chapter 18 measures aggregated with Equations 16-2 through 16-4.
//!   Exact: facility base FFS 40.1 mi/h, LOS C, poorest-segment LOS D.
//!   Approximate (Segments 2-4 are not individually published and copy
//!   Segments 1/5): travel speed 22.6 published vs. 22.1 computed
//!   (±0.6 tolerance), stop rate 1.83 published vs. 1.95 computed
//!   (±0.15).
//! * `case2.json` — Chapter 29 EP1, westbound. Exact: base FFS 40.1,
//!   LOS C, poorest-segment LOS D. Approximate: travel speed 22.2 vs.
//!   21.5 computed (±0.8), stop rate 1.93 vs. 2.14 computed (±0.25).
//! * `case3.json` — full Chapter 18-driven facility (three copies of the
//!   Chapter 30 EP1 eastbound segment): the facility must reproduce the
//!   published segment values exactly (harmonic/arithmetic means of
//!   identical values), verifying the analyze() pipeline end to end.

use transportations_library::hcm::urban_facilities::{FacilityResults, UrbanFacility};
use transportations_library::hcm::common::LevelOfService;

fn load_case(name: &str) -> UrbanFacility {
    let path = format!(
        "{}/tests/ExampleCases/hcm/UrbanFacilities/{}",
        env!("CARGO_MANIFEST_DIR"),
        name
    );
    let json = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {path}: {e}"));
    UrbanFacility::from_json(&json).unwrap_or_else(|e| panic!("parse {path}: {e}"))
}

macro_rules! assert_near {
    ($actual:expr, $expected:expr, $tol:expr, $what:expr) => {
        let (a, e) = ($actual, $expected);
        assert!(
            (a - e).abs() <= $tol,
            "{}: got {a}, expected {e} (tolerance {})",
            $what,
            $tol
        );
    };
}

/// Chapter 29, Example Problem 1, eastbound (Exhibit 29-49).
#[test]
fn test_case1_example_problem_1_eastbound() {
    let mut facility = load_case("case1.json");
    let r: FacilityResults = facility.aggregate().unwrap().clone();

    assert_near!(r.length_ft, 5_280.0, 1e-9, "facility length");
    // Exact — every segment base FFS is published (Exhibits 29-47/29-48).
    assert_near!(r.base_ffs_mph, 40.1, 0.05, "facility base FFS (Equation 16-2)");
    // Approximate — Segments 2-4 not individually published.
    assert_near!(r.travel_speed_mph, 22.6, 0.6, "facility travel speed (Equation 16-3)");
    assert_near!(
        r.spatial_stop_rate_stops_mi.unwrap(),
        1.83,
        0.15,
        "facility stop rate (Equation 16-4)"
    );
    // Exact.
    assert_eq!(r.los, LevelOfService::C, "facility LOS (Exhibit 16-3)");
    assert_eq!(r.poorest_segment_los, Some(LevelOfService::D), "poorest segment LOS");
    assert!(r.critical_vc_ratio.unwrap() <= 1.0, "undersaturated boundary intersections");
    assert!(r.perception_score.is_some());
}

/// Chapter 29, Example Problem 1, westbound (Exhibit 29-49).
#[test]
fn test_case2_example_problem_1_westbound() {
    let mut facility = load_case("case2.json");
    let r = facility.aggregate().unwrap().clone();

    assert_near!(r.base_ffs_mph, 40.1, 0.05, "facility base FFS");
    assert_near!(r.travel_speed_mph, 22.2, 0.8, "facility travel speed");
    assert_near!(r.spatial_stop_rate_stops_mi.unwrap(), 1.93, 0.25, "facility stop rate");
    assert_eq!(r.los, LevelOfService::C, "facility LOS");
    assert_eq!(r.poorest_segment_los, Some(LevelOfService::D), "poorest segment LOS");
}

/// Chapter 18-driven pipeline: three copies of the Chapter 30 EP1
/// eastbound segment must aggregate to the published segment values.
#[test]
fn test_case3_chapter18_driven_facility() {
    let mut facility = load_case("case3.json");
    let r = facility.analyze().unwrap().clone();

    // Published Chapter 30 EP1 values (Exhibit 30-36), reproduced at
    // facility level for identical segments.
    assert_near!(r.base_ffs_mph, 40.78, 0.02, "facility base FFS");
    assert_near!(r.travel_speed_mph, 23.67, 0.02, "facility travel speed");
    assert_near!(r.spatial_stop_rate_stops_mi.unwrap(), 1.61, 0.02, "facility stop rate");
    assert_near!(r.critical_vc_ratio.unwrap(), 0.52, 0.005, "critical v/c (968/1848)");
    assert_eq!(r.los, LevelOfService::C, "facility LOS");
    assert_eq!(r.poorest_segment_los, Some(LevelOfService::C), "poorest segment LOS");

    // Cross-check: facility speed must equal the length-weighted travel
    // time computation on the Chapter 18 outputs (Equation 16-3).
    let total_len: f64 = facility.segments.iter().map(|s| s.segment_length_ft).sum();
    let total_time: f64 = facility
        .segments
        .iter()
        .map(|s| s.segment_length_ft / s.travel_speed_mph.unwrap())
        .sum();
    assert_near!(
        r.travel_speed_mph,
        total_len / total_time,
        1e-12,
        "harmonic-mean identity with Chapter 18 outputs"
    );
}

/// The Exhibit 16-3 footnote: a v/c ratio above 1.0 at any boundary
/// intersection forces facility LOS F regardless of travel speed.
#[test]
fn test_case1_vc_footnote_forces_los_f() {
    let mut facility = load_case("case1.json");
    facility.segments[1].vc_ratio = Some(1.02);
    let r = facility.aggregate().unwrap().clone();
    assert_eq!(r.los, LevelOfService::F, "v/c > 1.0 rule");
    assert_near!(r.critical_vc_ratio.unwrap(), 1.02, 1e-9, "critical v/c");
}

/// JSON round trip of a fully analyzed facility.
#[test]
fn test_round_trip() {
    let mut facility = load_case("case3.json");
    facility.analyze().unwrap();
    let json = facility.to_json().unwrap();
    let restored = UrbanFacility::from_json(&json).unwrap();
    assert_eq!(restored.segments.len(), 3);
    assert_near!(
        restored.results.unwrap().travel_speed_mph,
        facility.results.as_ref().unwrap().travel_speed_mph,
        1e-12,
        "round-tripped travel speed"
    );
}
