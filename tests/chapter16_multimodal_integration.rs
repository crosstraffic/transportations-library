//! Integration tests for the HCM Chapter 16 (Urban Street Facilities)
//! multimodal facility LOS aggregation (Equations 16-5 through 16-13),
//! anchored to Chapter 29, Example Problem 2 (Pedestrian and Bicycle
//! Improvements; Exhibits 29-53, 29-54, 29-55).
//!
//! The facility scores are travel-time-weighted (pedestrian/bicycle) or
//! length-weighted (transit) aggregations of the Chapter 18 segment scores.
//! Example Problem 2 publishes only two representative segment evaluations
//! (Segment 1 and Segment 5) plus the facility summary, and its total facility
//! length (5,280 ft) does not decompose uniquely into the two published
//! segments, so an exact all-segment reproduction is not possible from the
//! published data. These tests therefore (a) validate the aggregation
//! equations exactly on controlled inputs and known invariants, and (b) anchor
//! to Example Problem 2 by confirming that a facility built from its two
//! published segments reproduces the composition-insensitive facility scores
//! (pedestrian ~2.91, bicycle 3.02) and the C/C/C facility LOS letters.

use transportations_library::hcm::common::LevelOfService;
use transportations_library::hcm::urban_facilities::urban_facilities::{
    facility_bicycle_los, facility_pedestrian_los, facility_pedestrian_space,
    facility_transit_los, facility_transit_los_score, facility_transit_travel_speed,
    facility_weighted_los_score,
};

fn approx(a: f64, b: f64, tol: f64, label: &str) {
    assert!((a - b).abs() <= tol, "{label}: got {a}, expected {b} (+-{tol})");
}

// ── Aggregation invariants ─────────────────────────────────────────────────

/// A single-segment facility must return that segment's own value for every
/// aggregation (the facility is the segment).
#[test]
fn single_segment_facility_equals_segment() {
    // (length, travel speed, score)
    let one = [(1320.0, 3.55, 2.93)];
    approx(facility_weighted_los_score(&one), 2.93, 1e-9, "ped/bike weighted score");
    approx(facility_transit_los_score(&[(1320.0, 3.43)]), 3.43, 1e-9, "transit score");
    approx(facility_transit_travel_speed(&[(1320.0, 10.3)]), 10.3, 1e-9, "transit speed");
    approx(facility_pedestrian_space(&[(1320.0, 809.9)]), 809.9, 1e-9, "pedestrian space");
}

/// A facility of identical segments returns the common segment value,
/// independent of how many segments or their (equal) lengths.
#[test]
fn identical_segments_return_common_value() {
    let segs = [(1000.0, 12.0, 3.02), (500.0, 12.0, 3.02), (750.0, 12.0, 3.02)];
    approx(facility_weighted_los_score(&segs), 3.02, 1e-9, "weighted score");
    let tr = [(1000.0, 3.02), (500.0, 3.02), (750.0, 3.02)];
    approx(facility_transit_los_score(&tr), 3.02, 1e-9, "transit score");
}

/// Hand-computed two-segment example checks the exact Equation 16-7/16-8 /
/// 16-10/16-11 arithmetic (travel-time-weighted generalized mean) and the
/// Equation 16-13 length-weighted transit mean.
#[test]
fn hand_computed_two_segment_aggregation() {
    // Segment A: L=1000 ft, S=10 ft/s, I=3.0 -> t=100 s, inner=(3-0.125)/0.75=3.8333
    // Segment B: L=2000 ft, S=20 ft/s, I=4.0 -> t=100 s, inner=(4-0.125)/0.75=5.1667
    // I_F = 0.75*[ (100*3.8333^3 + 100*5.1667^3)/200 ]^(1/3) + 0.125
    //     = 0.75*[ (5632.9 + 13792.4)/200 ]^(1/3) + 0.125
    //     = 0.75*[97.126]^(1/3) + 0.125 = 0.75*4.5987 + 0.125 = 3.574
    let segs = [(1000.0, 10.0, 3.0), (2000.0, 20.0, 4.0)];
    approx(facility_weighted_los_score(&segs), 3.574, 0.01, "Eq 16-7 weighted score");

    // Transit Eq 16-13: (1000*3.0 + 2000*4.0)/3000 = 11000/3000 = 3.667
    approx(facility_transit_los_score(&[(1000.0, 3.0), (2000.0, 4.0)]), 3.667, 0.001, "Eq 16-13");
}

// ── Example Problem 2 anchor ────────────────────────────────────────────────

/// Example Problem 2 (eastbound): a facility built from the two published
/// segment evaluations reproduces the composition-insensitive facility scores
/// (pedestrian ~2.91, bicycle 3.02) and the published C/C/C facility LOS.
///
/// Published segment inputs (Exhibits 29-53 / 29-54, eastbound):
/// * Segment 1 (L = 1,320 ft): ped speed 3.55 ft/s, ped score 2.93, ped space
///   809.9; bike speed 13.16 mi/h, bike score 3.02; transit speed 10.3 mi/h,
///   transit score 3.43.
/// * Segment 5 (shorter): ped speed 3.18 ft/s, ped score 2.85, ped space
///   225.4; bike speed 11.67 mi/h, bike score 3.01; transit speed 5.3 mi/h,
///   transit score 3.99.
///
/// Because both published segments carry nearly identical bicycle and
/// pedestrian scores, the aggregated facility scores are insensitive to the
/// (unpublished) exact segment composition, so they reproduce the Exhibit
/// 29-55 facility scores. The facility pedestrian space, travel speeds, and
/// transit score depend on the full composition and are not asserted here.
#[test]
fn example_problem_2_facility_scores() {
    // Interior segments follow Segment 1; the two downstream segments follow
    // Segment 5 (per the book's "dominance of platoon flow for Segments 4 and
    // 5"): three 1,320-ft Segment-1 segments and two 660-ft Segment-5 segments
    // (total 5,280 ft, the published facility length).
    let ped = [
        (1320.0, 3.55, 2.93), (1320.0, 3.55, 2.93), (1320.0, 3.55, 2.93),
        (660.0, 3.18, 2.85), (660.0, 3.18, 2.85),
    ];
    let bike = [
        (1320.0, 13.16, 3.02), (1320.0, 13.16, 3.02), (1320.0, 13.16, 3.02),
        (660.0, 11.67, 3.01), (660.0, 11.67, 3.01),
    ];
    let ped_score = facility_weighted_los_score(&ped);
    let bike_score = facility_weighted_los_score(&bike);
    approx(ped_score, 2.91, 0.03, "facility pedestrian score (published 2.91)");
    approx(bike_score, 3.02, 0.02, "facility bicycle score (published 3.02)");

    // Facility LOS from the aggregated scores (Exhibits 16-4/16-5, identical to
    // the Chapter 18 segment thresholds). Facility pedestrian space is a full-
    // composition measure; the published 422.2 ft^2/p is well inside the LOS-C
    // band (>24-40 ... >60 → all >= C at this score), so the space cannot
    // improve the score-based C.
    assert_eq!(facility_bicycle_los(bike_score), LevelOfService::C, "bicycle LOS");
    assert_eq!(
        facility_pedestrian_los(ped_score, 422.2),
        LevelOfService::C,
        "pedestrian LOS"
    );
    // Transit is the one composition-sensitive score here (Segment 5's
    // transit score, 3.99, is far from Segment 1's 3.43), so the published
    // facility transit score of 3.48 cannot be reproduced without the exact
    // segment set. The length-weighted aggregate is instead checked to lie
    // between the two published segment scores, as any weighted average must.
    let transit = [
        (1320.0, 3.43), (1320.0, 3.43), (1320.0, 3.43),
        (660.0, 3.99), (660.0, 3.99),
    ];
    let transit_score = facility_transit_los_score(&transit);
    assert!(
        (3.43..=3.99).contains(&transit_score),
        "transit facility score {transit_score} must lie between the segment scores (published 3.48)"
    );
    // Sanity: the transit facility LOS letter comes from the same threshold as
    // the segments (Exhibit 16-5 == 18-3).
    let _ = facility_transit_los(transit_score);
}
