//! Integration tests for the HCM Chapter 18 (Urban Street Segments)
//! motorized vehicle methodology: full pipeline runs against the published
//! answers of HCM 7th Edition, Chapter 30, Section 8, Example Problem 1
//! (Exhibits 30-26 through 30-36).
//!
//! Fixtures (three modes for the access-point delay term):
//! * `case1.json` — Example Problem 1, eastbound direction, with the
//!   published Chapter 30, Section 4 per-access-point turning delays
//!   (Exhibit 30-35) supplied directly as the `access_point_delays_s` input
//!   hook. Reproduces every published segment performance measure of
//!   Exhibit 30-36.
//! * `case2.json` — Example Problem 1, westbound direction (identical
//!   published results by symmetry), exercising the Exhibit 18-13
//!   planning-level turning-delay estimate instead. The estimate is
//!   0.540 s total vs. the Section 4 procedure's 0.387 s, so running time
//!   computes to 33.70 s vs. the published 33.54 s and travel speed to
//!   23.60 mi/h vs. the published 23.67 mi/h (asserted within the fixture
//!   tolerances below); all other measures reproduce exactly.
//! * `case3.json` — Example Problem 1, eastbound, COMPUTED MODE: the
//!   Chapter 30, Section 4 access-point delay procedure (Equations 30-31
//!   through 30-68) computes the per-access-point delay from the access
//!   point geometry and turn volumes, reproducing the published 0.193/0.194
//!   s/veh (Exhibit 30-35) and the 0.115 inside-lane blockage probability,
//!   and every downstream measure identically to case1.
//!
//! Documented tolerances:
//! * LOS — exact;
//! * base free-flow speed — ±0.01 mi/h (case1) / ±0.01 (case2);
//! * running time — ±0.01 s (case1); ±0.5 s (case2, Exhibit 18-13
//!   estimate);
//! * travel speed — ±0.01 mi/h (case1); ±0.5 mi/h (case2);
//! * through delay — input pass-through, ±0.001 s/veh;
//! * spatial stop rate — ±0.01 stops/mi; v/c — ±0.005; perception score —
//!   ±0.01.
//!
//! The through control delay, through capacity, and full stop rate are
//! "HCM method output" inputs per Exhibit 18-5 (they come from the
//! boundary-intersection engines, here the published values of Exhibits
//! 30-32, 30-33, and 30-36); the tests therefore assert the Chapter 18
//! segment equations built on them, not the Chapter 19 engine.

use transportations_library::hcm::urban_segments::UrbanSegment;
use transportations_library::hcm::common::LevelOfService;

fn load_case(name: &str) -> UrbanSegment {
    let path = format!(
        "{}/tests/ExampleCases/hcm/UrbanSegments/{}",
        env!("CARGO_MANIFEST_DIR"),
        name
    );
    let json = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {path}: {e}"));
    serde_json::from_str(&json).unwrap_or_else(|e| panic!("parse {path}: {e}"))
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

/// Chapter 30, Example Problem 1, eastbound: published Exhibit 30-36
/// performance measures.
#[test]
fn test_case1_example_problem_1_eastbound() {
    let mut seg = load_case("case1.json");
    seg.analyze();

    assert_near!(seg.base_ffs_mph.unwrap(), 40.78, 0.01, "base FFS [30-36]");
    assert_near!(seg.running_time_s.unwrap(), 33.54, 0.01, "running time [30-36]");
    assert_near!(seg.running_speed_mph.unwrap(), 36.59, 0.01, "running speed [30-36]");
    assert_near!(seg.through_delay_s.unwrap(), 18.310, 0.001, "through delay [30-36]");
    assert_near!(seg.travel_speed_mph.unwrap(), 23.67, 0.01, "travel speed [30-36]");
    assert_near!(seg.full_stop_rate.unwrap(), 0.547, 0.001, "stop rate [30-36]");
    assert_near!(
        seg.spatial_stop_rate_stops_mi.unwrap(),
        1.61,
        0.01,
        "spatial stop rate [30-36]"
    );
    assert_near!(seg.vc_ratio.unwrap(), 0.52, 0.005, "through v/c [30-36]");
    assert_eq!(seg.los, Some(LevelOfService::C), "LOS [30-36]");
    assert_near!(
        seg.perception_score.unwrap(),
        2.53,
        0.01,
        "traveler perception score [30-36]"
    );

    // Intermediates of the Step 2 chain (hand-verified from Equations
    // 18-3 through 18-7): S_0 = 42.05, f_CS = -0.329, f_A = -0.941,
    // f_L = 0.9644, S_f = 39.33, f_v = 1.034.
    assert_near!(seg.speed_constant_mph.unwrap(), 42.05, 0.001, "S_0");
    assert_near!(seg.f_cs_mph.unwrap(), -0.329, 0.001, "f_CS");
    assert_near!(seg.f_a_mph.unwrap(), -0.941, 0.001, "f_A");
    assert_near!(seg.f_l.unwrap(), 0.9644, 0.0005, "f_L");
    assert_near!(seg.free_flow_speed_mph.unwrap(), 39.33, 0.01, "S_f");
    assert_near!(seg.f_v.unwrap(), 1.034, 0.0005, "f_v");

    // Step 3 under the uniform-arrival assumption (no upstream discharge-
    // flow profiles supplied): P = g/C = 48.63/100 = 0.486. The published
    // engine value is 0.493 (Exhibit 30-32, WB through — an internal
    // movement — at Intersection 1) via the Chapter 30 platoon-dispersion
    // procedure. The dispersion primitives (Equations 30-9 through 30-13)
    // are implemented and unit-tested; reproducing the 0.493 arrival profile
    // from the raw coordinated-actuated signal requires the full Chapter 19
    // discharge-profile + O-D engine (see docs/hcm/VERIFICATION.md).
    assert_near!(
        seg.proportion_arriving_green.unwrap(),
        0.486,
        0.001,
        "P (uniform-arrival assumption; published dispersion value 0.493)"
    );
}

/// Chapter 30, Example Problem 1, westbound, with the Exhibit 18-13
/// planning-level access point delay estimate (documented deviation:
/// running time +0.16 s, travel speed −0.07 mi/h vs. the published
/// Section 4 procedure values).
#[test]
fn test_case2_example_problem_1_westbound_planning_estimate() {
    let mut seg = load_case("case2.json");
    seg.analyze();

    assert_near!(seg.base_ffs_mph.unwrap(), 40.78, 0.01, "base FFS [30-36]");
    // Exhibit 18-13 estimate: 0.37 s/veh/pt at 575 veh/h/ln (2 lanes)
    // × (6.5 + 8.1)/20 × 2 points = 0.540 s.
    assert_near!(
        seg.access_point_delay_total_s.unwrap(),
        0.540,
        0.005,
        "sum d_ap (Exhibit 18-13 estimate; Section 4 procedure: 0.387)"
    );
    // Computed 33.70 s vs. published 33.54 s.
    assert_near!(seg.running_time_s.unwrap(), 33.54, 0.5, "running time [30-36]");
    assert_near!(seg.running_time_s.unwrap(), 33.70, 0.01, "running time (computed)");
    // Computed 23.60 mi/h vs. published 23.67 mi/h (±0.5 mi/h tolerance).
    assert_near!(seg.travel_speed_mph.unwrap(), 23.67, 0.5, "travel speed [30-36]");
    assert_near!(seg.travel_speed_mph.unwrap(), 23.60, 0.01, "travel speed (computed)");
    assert_near!(seg.through_delay_s.unwrap(), 18.310, 0.001, "through delay [30-36]");
    assert_near!(
        seg.spatial_stop_rate_stops_mi.unwrap(),
        1.61,
        0.01,
        "spatial stop rate [30-36]"
    );
    assert_near!(seg.vc_ratio.unwrap(), 0.52, 0.005, "through v/c [30-36]");
    assert_eq!(seg.los, Some(LevelOfService::C), "LOS [30-36]");
    assert_near!(
        seg.perception_score.unwrap(),
        2.53,
        0.01,
        "traveler perception score [30-36]"
    );
}

/// Chapter 30, Example Problem 1, eastbound — COMPUTED MODE. The Chapter 30,
/// Section 4 access-point delay procedure (Equations 30-31 through 30-68)
/// computes the per-access-point through delay from the access-point
/// geometry and turn volumes, in place of the case1 published-input hook.
/// Asserts the computed intermediates against the published Exhibit 30-35
/// values and confirms every downstream performance measure reproduces the
/// published Exhibit 30-36 values identically to case1.
#[test]
fn test_case3_example_problem_1_computed_access_point_delay() {
    let mut seg = load_case("case3.json");
    seg.analyze();

    // Computed per-access-point delay (Exhibit 30-35): AP1 = 0.193 s/veh,
    // AP2 = 0.194 s/veh; inside-lane blockage probability 0.115 at both.
    let computed = seg
        .access_point_delays_computed
        .as_ref()
        .expect("computed access-point delays");
    assert_eq!(computed.len(), 2, "two active access points");
    assert_near!(computed[0].delay_total_s, 0.193, 0.001, "d_ap AP1 [30-35]");
    assert_near!(computed[1].delay_total_s, 0.194, 0.001, "d_ap AP2 [30-35]");
    assert_near!(
        computed[0].prob_inside_lane_blocked,
        0.115,
        0.001,
        "p_ov AP1 [30-35]"
    );
    assert_near!(
        computed[1].prob_inside_lane_blocked,
        0.115,
        0.001,
        "p_ov AP2 [30-35]"
    );
    // Σ d_ap,i matches the case1 published-input total (0.193 + 0.194).
    // Σ = 0.1934 + 0.1947 = 0.3881 vs published 0.193 + 0.194 = 0.387
    // (the two per-point roundings accumulate).
    assert_near!(
        seg.access_point_delay_total_s.unwrap(),
        0.387,
        0.002,
        "Σ d_ap,i (computed vs Exhibit 30-35)"
    );

    // Every published Exhibit 30-36 measure reproduces identically to case1.
    assert_near!(seg.base_ffs_mph.unwrap(), 40.78, 0.01, "base FFS [30-36]");
    assert_near!(seg.running_time_s.unwrap(), 33.54, 0.01, "running time [30-36]");
    assert_near!(seg.running_speed_mph.unwrap(), 36.59, 0.01, "running speed [30-36]");
    assert_near!(seg.travel_speed_mph.unwrap(), 23.67, 0.01, "travel speed [30-36]");
    assert_near!(
        seg.spatial_stop_rate_stops_mi.unwrap(),
        1.61,
        0.01,
        "spatial stop rate [30-36]"
    );
    assert_near!(seg.vc_ratio.unwrap(), 0.52, 0.005, "through v/c [30-36]");
    assert_eq!(seg.los, Some(LevelOfService::C), "LOS [30-36]");
    assert_near!(
        seg.perception_score.unwrap(),
        2.53,
        0.01,
        "traveler perception score [30-36]"
    );

    // Step 3 proportion arriving during green: without upstream discharge-
    // flow profiles supplied, the uniform assumption gives P = g/C = 0.486.
    // The published dispersion value (0.493) requires the full Chapter 19
    // coordinated-actuated discharge-profile + O-D engine (see
    // docs/hcm/VERIFICATION.md); the dispersion primitives themselves are
    // unit-tested in urban_segments/tests.rs.
    assert_near!(
        seg.proportion_arriving_green.unwrap(),
        0.486,
        0.001,
        "P (uniform; computed-dispersion value 0.493 deferred)"
    );
}

/// The fixture format round-trips through serde with results attached.
#[test]
fn test_fixture_round_trip() {
    let mut seg = load_case("case1.json");
    seg.analyze();
    let json = seg.to_json().expect("serialize");
    let back = UrbanSegment::from_json(&json).expect("deserialize");
    assert_eq!(back.los, seg.los);
    assert_eq!(back.travel_speed_mph, seg.travel_speed_mph);
}
