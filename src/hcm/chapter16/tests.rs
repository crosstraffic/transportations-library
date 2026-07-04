//! Unit tests for HCM Chapter 16 (Urban Street Facilities), motorized
//! vehicle methodology. Published values from the HCM 7th Edition EPUB:
//! Chapter 29, Example Problem 1 (`230_Ch29_05.xhtml`, Exhibits 29-47
//! through 29-49) and the Chapter 16 text (`111_Ch16_02.xhtml`,
//! `112_Ch16_03.xhtml`).

use super::urban_facilities::*;
use crate::hcm::chapter18::urban_segments::{BoundaryControlType, UrbanSegment};
use crate::hcm::common::LevelOfService as L;

// ═══════════════════════════════════════════════════════════════════════════
// Equations 16-2 through 16-4
// ═══════════════════════════════════════════════════════════════════════════

/// Chapter 29, Example Problem 1: three 1,320-ft segments with a base FFS
/// of 40.9 mi/h and two 660-ft segments at 37.9 mi/h aggregate to the
/// published facility base free-flow speed of 40.1 mi/h (Exhibits 29-47,
/// 29-48, and 29-49).
#[test]
fn test_eq_16_2_example_problem_1_base_ffs() {
    let segs = [
        (1_320.0, 40.9),
        (1_320.0, 40.9),
        (1_320.0, 40.9),
        (660.0, 37.9),
        (660.0, 37.9),
    ];
    let s_fo_f = facility_base_ffs(&segs);
    assert!(
        (s_fo_f - 40.1).abs() < 0.05,
        "facility base FFS {s_fo_f} vs published 40.1"
    );
}

/// Equation 16-2/16-3 are harmonic (travel-time-weighted): a facility of
/// equal-length segments at 30 and 60 mi/h travels at 40 mi/h, not 45.
#[test]
fn test_harmonic_aggregation() {
    let segs = [(1_000.0, 30.0), (1_000.0, 60.0)];
    assert!((facility_travel_speed(&segs) - 40.0).abs() < 1e-9);
    assert!((facility_base_ffs(&segs) - 40.0).abs() < 1e-9);
}

/// Equation 16-4 is a length-weighted arithmetic mean.
#[test]
fn test_eq_16_4_spatial_stop_rate() {
    let segs = [(1_000.0, 1.0), (3_000.0, 3.0)];
    assert!((facility_spatial_stop_rate(&segs) - 2.5).abs() < 1e-9);
    // Example Problem 1 eastbound: segments at 1.72 (1,320 ft ×3 assumed
    // equal) and 2.63 (660 ft ×2) give a facility rate near the published
    // 1.83 stops/mi (segments 2-4 are not individually published; this
    // asserts the aggregation form, not the published value).
    let ep1 = [
        (1_320.0, 1.72),
        (1_320.0, 1.72),
        (1_320.0, 1.72),
        (660.0, 2.63),
        (660.0, 2.63),
    ];
    let h_f = facility_spatial_stop_rate(&ep1);
    assert!((h_f - 1.9475).abs() < 1e-4, "H_F = {h_f}");
}

// ═══════════════════════════════════════════════════════════════════════════
// Exhibit 16-3 LOS
// ═══════════════════════════════════════════════════════════════════════════

/// Exhibit 16-3 thresholds are identical to Exhibit 18-1; the Chapter 16
/// text interpolation example ("the LOS A threshold for a segment with a
/// base free-flow speed of 42 mi/h is 34 mi/h [= (42-40)/(45-40) ×
/// (36-32) + 32]", `111_Ch16_02.xhtml`) must hold, as must the v/c > 1.0
/// footnote rule.
#[test]
fn test_exhibit_16_3_los() {
    // Interpolated LOS A threshold at BFFS 42 is 33.6 (the chapter text
    // rounds to 34): 33.7 mi/h is LOS A, 33.5 mi/h is LOS B.
    assert_eq!(exhibit_16_3_los(33.7, 42.0, false), L::A);
    assert_eq!(exhibit_16_3_los(33.5, 42.0, false), L::B);
    // Column-heading checks at BFFS 50: A >40, B >34, C >25, D >20, E >15.
    assert_eq!(exhibit_16_3_los(40.1, 50.0, false), L::A);
    assert_eq!(exhibit_16_3_los(34.1, 50.0, false), L::B);
    assert_eq!(exhibit_16_3_los(25.1, 50.0, false), L::C);
    assert_eq!(exhibit_16_3_los(20.1, 50.0, false), L::D);
    assert_eq!(exhibit_16_3_los(15.1, 50.0, false), L::E);
    assert_eq!(exhibit_16_3_los(15.0, 50.0, false), L::F);
    // Footnote: v/c > 1.0 at any boundary intersection forces LOS F.
    assert_eq!(exhibit_16_3_los(45.0, 50.0, true), L::F);
}

// ═══════════════════════════════════════════════════════════════════════════
// Aggregation over summaries
// ═══════════════════════════════════════════════════════════════════════════

fn ep1_eastbound_summaries() -> Vec<SegmentSummary> {
    // Published segment 1 (Exhibit 29-47) and segment 5 (Exhibit 29-48)
    // eastbound values; segments 2-3 assume segment 1's values and
    // segment 4 assumes segment 5's (per-segment values not published).
    let seg1 = SegmentSummary {
        length_ft: 1_320.0,
        base_ffs_mph: 40.9,
        travel_speed_mph: 24.2,
        spatial_stop_rate_stops_mi: Some(1.72),
        vc_ratio: Some(0.85),
        los: Some(L::C),
    };
    let seg5 = SegmentSummary {
        length_ft: 660.0,
        base_ffs_mph: 37.9,
        travel_speed_mph: 17.6,
        spatial_stop_rate_stops_mi: Some(2.63),
        vc_ratio: Some(0.9),
        los: Some(L::D),
    };
    vec![seg1.clone(), seg1.clone(), seg1, seg5.clone(), seg5]
}

#[test]
fn test_aggregate_example_problem_1_eastbound() {
    let r = aggregate_segment_summaries(&ep1_eastbound_summaries(), Some(1.0)).unwrap();
    assert!((r.length_ft - 5_280.0).abs() < 1e-9);
    // Published facility base FFS: 40.1 mi/h (exact; all segment BFFS
    // values are published).
    assert!((r.base_ffs_mph - 40.1).abs() < 0.05, "BFFS {}", r.base_ffs_mph);
    // Published facility travel speed: 22.6 mi/h. Segments 2-4 are not
    // individually published, so the reproduction from segments 1/5
    // values is approximate (22.1 mi/h); assert the band and the exact
    // published LOS C, which holds across the band.
    assert!(
        (r.travel_speed_mph - 22.6).abs() < 0.6,
        "travel speed {} vs published 22.6",
        r.travel_speed_mph
    );
    assert_eq!(r.los, L::C, "published facility LOS C");
    // Published poorest-performing segment LOS: D (Exhibit 29-49).
    assert_eq!(r.poorest_segment_los, Some(L::D));
    // Critical v/c is the max of the boundary ratios.
    assert!((r.critical_vc_ratio.unwrap() - 0.9).abs() < 1e-9);
    assert!(r.perception_score.is_some());
    // Consistency: travel time = 3,600 L / (5,280 S).
    assert!(
        (r.travel_time_s - 3_600.0 * 5_280.0 / (5_280.0 * r.travel_speed_mph)).abs() < 1e-9
    );
}

#[test]
fn test_critical_vc_rule_forces_f() {
    let mut segs = ep1_eastbound_summaries();
    segs[2].vc_ratio = Some(1.05);
    let r = aggregate_segment_summaries(&segs, None).unwrap();
    assert_eq!(r.los, L::F, "v/c > 1.0 at one boundary intersection is LOS F");
    assert!((r.critical_vc_ratio.unwrap() - 1.05).abs() < 1e-9);
}

#[test]
fn test_aggregate_rejects_empty_and_invalid() {
    assert!(aggregate_segment_summaries(&[], None).is_err());
    let bad = vec![SegmentSummary {
        length_ft: 0.0,
        base_ffs_mph: 40.0,
        travel_speed_mph: 20.0,
        spatial_stop_rate_stops_mi: None,
        vc_ratio: None,
        los: None,
    }];
    assert!(aggregate_segment_summaries(&bad, None).is_err());
}

// ═══════════════════════════════════════════════════════════════════════════
// Facility built from Chapter 18 segments (cross-check)
// ═══════════════════════════════════════════════════════════════════════════

fn ch18_segment(length_ft: f64, delay_s: f64) -> UrbanSegment {
    let mut s = UrbanSegment::new(length_ft, 2, 35.0, 900.0, BoundaryControlType::Signalized);
    s.through_capacity_veh_h = Some(1_700.0);
    s.through_control_delay_s = Some(delay_s);
    s.cycle_length_s = Some(100.0);
    s.effective_green_s = Some(45.0);
    s.full_stop_rate_override = Some(0.5);
    s
}

/// The facility travel speed (Equation 16-3) must equal the
/// length-weighted travel-time computation performed directly on the
/// Chapter 18 outputs: `S_T,F = Σ L_i / Σ (L_i / S_T,seg,i)`, which is
/// identical to total length over total (running + control) travel time.
#[test]
fn test_facility_speed_equals_length_weighted_ch18_computation() {
    let mut facility = UrbanFacility::new(vec![
        ch18_segment(1_800.0, 18.0),
        ch18_segment(1_320.0, 25.0),
        ch18_segment(900.0, 12.0),
    ]);
    let results = facility.analyze().unwrap().clone();

    // Direct Chapter 18 recomputation.
    let mut total_length = 0.0;
    let mut total_time_h = 0.0;
    let mut total_time_s = 0.0;
    for seg in &facility.segments {
        let l = seg.segment_length_ft;
        let s_t = seg.travel_speed_mph.expect("segment travel speed");
        total_length += l;
        total_time_h += (l / 5_280.0) / s_t;
        // Equivalent: running time + through delay in seconds.
        total_time_s += seg.running_time_s.unwrap() + seg.through_delay_s.unwrap();
    }
    let expected = (total_length / 5_280.0) / total_time_h;
    assert!(
        (results.travel_speed_mph - expected).abs() < 1e-9,
        "facility speed {} vs length-weighted Ch18 {}",
        results.travel_speed_mph,
        expected
    );
    // And the harmonic identity: same speed from summed seconds.
    let expected_from_seconds = 3_600.0 * total_length / (5_280.0 * total_time_s);
    assert!(
        (results.travel_speed_mph - expected_from_seconds).abs() < 1e-9,
        "facility speed {} vs Σt_R+d_t form {}",
        results.travel_speed_mph,
        expected_from_seconds
    );
    // v/c of every boundary is 900/1700; critical matches.
    assert!((results.critical_vc_ratio.unwrap() - 900.0 / 1_700.0).abs() < 1e-9);
}

#[test]
fn test_facility_json_round_trip() {
    let mut facility = UrbanFacility::new(vec![ch18_segment(1_800.0, 18.0)]);
    facility.prop_left_turn_lanes = Some(0.5);
    facility.analyze().unwrap();
    let json = facility.to_json().unwrap();
    let restored = UrbanFacility::from_json(&json).unwrap();
    assert_eq!(restored.segments.len(), 1);
    assert!(
        (restored.results.unwrap().travel_speed_mph
            - facility.results.as_ref().unwrap().travel_speed_mph)
            .abs()
            < 1e-12
    );
}

#[test]
fn test_spillback_check_hook() {
    let mut facility = UrbanFacility::new(vec![
        ch18_segment(1_800.0, 18.0),
        ch18_segment(660.0, 30.0),
    ]);
    facility.spillback_inputs = Some(vec![
        SpillbackCheckInput {
            available_storage_ft: 1_750.0,
            back_of_queue_veh_ln: 12.0,
            avg_vehicle_spacing_ft: 25.0,
        },
        SpillbackCheckInput {
            available_storage_ft: 610.0,
            back_of_queue_veh_ln: 30.0,
            avg_vehicle_spacing_ft: 25.0,
        },
    ]);
    facility.analyze().unwrap();
    // 25×12/1750 = 0.17 (no spillback); 25×30/610 = 1.23 (spillback).
    assert_eq!(facility.spillback_flags, Some(vec![false, true]));
}

/// Equation 16-1 re-export sanity: c_th = 1,800 (N_th − 1 + p*_0,j).
#[test]
fn test_eq_16_1_through_capacity() {
    assert!((through_capacity_uncontrolled(2, 1.0) - 3_600.0).abs() < 1e-9);
    assert!((through_capacity_uncontrolled(1, 0.75) - 1_350.0).abs() < 1e-9);
}
