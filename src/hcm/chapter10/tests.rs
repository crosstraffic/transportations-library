//! Unit tests for HCM Chapter 10 (Freeway Facilities Core Methodology).
//!
//! Spot values reference HCM 7th Edition Chapter 25, Example Problem 1
//! (Exhibits 25-43 through 25-52) — the full fixture-driven integration
//! tests live in `tests/chapter10_integration.rs`.

use crate::hcm::common::{CityType, LevelOfService};

use super::freeway_facilities::{
    segment_ramp_section, FacilitySegment, FreewayFacility, SegmentType, Terrain,
};

/// Build the Example Problem 1 facility (Exhibits 25-43/25-44) with the
/// given mainline and ramp demands.
pub(crate) fn example_facility(
    mainline: Vec<f64>,
    onr1: Vec<f64>,
    onr2: Vec<f64>,
    onr3: Vec<f64>,
    ofr1: Vec<f64>,
    ofr2: Vec<f64>,
    ofr3: Vec<f64>,
    rr: Vec<f64>,
) -> FreewayFacility {
    let seg = |seg_type: SegmentType, length_ft: f64, lanes: u32| FacilitySegment {
        seg_type,
        length_ft,
        lanes,
        ramp_ffs: 40.0,
        accel_lane_ft: 500.0,
        decel_lane_ft: 500.0,
        ..Default::default()
    };
    let mut segments = vec![
        seg(SegmentType::Basic, 5280.0, 3),
        seg(SegmentType::Merge, 1500.0, 3),
        seg(SegmentType::Basic, 2280.0, 3),
        seg(SegmentType::Diverge, 1500.0, 3),
        seg(SegmentType::Basic, 5280.0, 3),
        seg(SegmentType::Weaving, 2640.0, 4),
        seg(SegmentType::Basic, 5280.0, 3),
        seg(SegmentType::Merge, 1140.0, 3),
        seg(SegmentType::OverlappingRamp, 360.0, 3),
        seg(SegmentType::Diverge, 1140.0, 3),
        seg(SegmentType::Basic, 5280.0, 3),
    ];
    segments[1].on_ramp_demand = onr1;
    segments[3].off_ramp_demand = ofr1;
    segments[5].on_ramp_demand = onr2;
    segments[5].off_ramp_demand = ofr2;
    segments[5].ramp_to_ramp_demand = rr;
    segments[5].short_length_ft = Some(1640.0);
    segments[7].on_ramp_demand = onr3;
    segments[9].off_ramp_demand = ofr3;

    FreewayFacility {
        segments,
        mainline_demand: mainline,
        ffs: 60.0,
        heavy_vehicle_pct: 0.0225, // 1.25% SUT + 1.00% TT
        terrain: Terrain::Level,
        city_type: CityType::Urban,
        phf: 1.0,
        jam_density_pc: 190.0,
        queue_discharge_drop: 0.07,
        total_ramp_density: 1.0,
        // VERIFY-HCM: the Example Problem facts give TRD = 1.0 ramp/mi but
        // no interchange density for the Chapter 13 engine; ID = 0.8 int/mi
        // reproduces the published weave speeds (result is insensitive in
        // the 0.8-1.0 range).
        interchange_density: Some(0.8),
        c_ifl_override: None, // Equation 12-6 at FFS 60 gives c_IFL = 2,300
        ..Default::default()
    }
}

pub(crate) fn ep1_facility() -> FreewayFacility {
    example_facility(
        vec![4505.0, 4955.0, 5225.0, 4685.0, 3785.0],
        vec![450.0, 540.0, 630.0, 360.0, 180.0],
        vec![540.0, 720.0, 810.0, 360.0, 270.0],
        vec![450.0, 540.0, 630.0, 450.0, 270.0],
        vec![270.0, 360.0, 270.0, 270.0, 270.0],
        vec![360.0, 360.0, 360.0, 360.0, 180.0],
        vec![270.0, 270.0, 450.0, 270.0, 180.0],
        vec![50.0, 100.0, 150.0, 80.0, 50.0],
    )
}

fn approx(a: f64, b: f64, tol: f64, label: &str) {
    assert!((a - b).abs() <= tol, "{label}: got {a}, expected {b} (+-{tol})");
}

#[test]
fn test_fhv_example_problem() {
    // 2.25% heavy vehicles, level terrain (E_T = 2): f_HV = 1/1.0225
    let fac = ep1_facility();
    approx(fac.f_hv(), 0.978, 0.0005, "f_HV");
}

#[test]
fn test_demand_accumulation_matches_exhibit_25_48() {
    // Exhibit 25-48 row 1 (undersaturated: volume served = demand).
    let mut fac = ep1_facility();
    fac.compute_demands();
    let expected = [
        4505.0, 4955.0, 4955.0, 4955.0, 4685.0, 5225.0, 4865.0, 5315.0, 5315.0, 5315.0,
        5045.0,
    ];
    for (i, e) in expected.iter().enumerate() {
        approx(fac.demand[i][0], *e, 0.001, &format!("SD(seg {}, p1)", i + 1));
    }
    // Exiting flow conservation: last segment demand minus nothing = 5,045
    // (matches the Exhibit 25-45 exiting flow rate).
}

#[test]
fn test_capacities_match_exhibit_25_46() {
    let mut fac = ep1_facility();
    fac.compute_demands();
    fac.compute_capacities();
    // Basic/ramp segments: 2,300 pc/h/ln x 3 ln x 0.978 = 6,748 veh/h.
    for i in [0usize, 1, 2, 3, 4, 6, 7, 8, 9, 10] {
        approx(fac.capacity[i][0], 6748.0, 5.0, &format!("capacity seg {}", i + 1));
    }
    // Weaving segment capacities by period (Exhibit 25-46).
    let weave_caps = [8273.0, 8281.0, 8323.0, 8403.0, 8463.0];
    for (p, c) in weave_caps.iter().enumerate() {
        approx(fac.capacity[5][p], *c, 25.0, &format!("weave capacity p{}", p + 1));
    }
}

#[test]
fn test_dc_ratios_match_exhibit_25_47() {
    let mut fac = ep1_facility();
    fac.compute_demands();
    fac.compute_capacities();
    fac.compute_dc_ratios();
    assert!(!fac.oversaturated);
    // Peak-period (p3) ratios from Exhibit 25-47.
    let expected = [0.77, 0.87, 0.87, 0.87, 0.83, 0.77, 0.89, 0.99, 0.99, 0.99, 0.92];
    for (i, e) in expected.iter().enumerate() {
        approx(fac.dc_ratio[i][2], *e, 0.005, &format!("vd/c seg {} p3", i + 1));
    }
}

#[test]
fn test_undersaturated_speeds_match_exhibit_25_49_period1() {
    let mut fac = ep1_facility();
    fac.run_analysis().unwrap();
    // Exhibit 25-49, Analysis Period 1.
    let expected = [60.0, 53.9, 59.7, 56.1, 60.0, 48.0, 59.9, 53.4, 53.4, 56.0, 59.7];
    for (i, e) in expected.iter().enumerate() {
        approx(fac.speed[i][0], *e, 0.5, &format!("speed seg {} p1", i + 1));
    }
}

#[test]
fn test_undersaturated_densities_match_exhibit_25_50_period1() {
    let mut fac = ep1_facility();
    fac.run_analysis().unwrap();
    // Exhibit 25-50, Analysis Period 1 (veh/mi/ln).
    let expected = [25.0, 30.6, 27.6, 29.4, 26.0, 27.2, 27.1, 33.2, 33.2, 31.6, 28.1];
    for (i, e) in expected.iter().enumerate() {
        approx(fac.density_veh[i][0], *e, 0.5, &format!("density seg {} p1", i + 1));
    }
}

#[test]
fn test_undersaturated_facility_performance_matches_exhibit_25_52() {
    let mut fac = ep1_facility();
    fac.run_analysis().unwrap();
    // Exhibit 25-52: speed (mi/h), density (veh/mi/ln), LOS by period.
    let expected = [
        (57.6, 27.5, LevelOfService::D),
        (56.6, 31.3, LevelOfService::D),
        (55.0, 34.8, LevelOfService::E),
        (57.9, 27.5, LevelOfService::D),
        (58.4, 21.4, LevelOfService::C),
    ];
    for (p, (s, k, l)) in expected.iter().enumerate() {
        let perf = &fac.facility_performance[p];
        approx(perf.space_mean_speed, *s, 0.5, &format!("facility SMS p{}", p + 1));
        approx(perf.avg_density_veh, *k, 0.5, &format!("facility K p{}", p + 1));
        assert_eq!(perf.los, *l, "facility LOS p{}", p + 1);
    }
}

#[test]
fn test_max_achievable_speed_constraint_applied() {
    // Segment 3 (basic) follows the slower merge segment 2: its Chapter 12
    // speed (59.9) is capped at V_max = 59.7 (Equation 25-1).
    let mut fac = ep1_facility();
    fac.run_analysis().unwrap();
    assert!(
        fac.speed[2][0] < 59.8,
        "expected Equation 25-1 to cap segment 3 speed, got {}",
        fac.speed[2][0]
    );
}

#[test]
fn test_overlapping_ramp_adopts_worse_service_measure() {
    // Segment 9 (overlapping ramp) adopts the worse (merge segment 8)
    // speed per the Exhibit 10-11(c) worst-case rule.
    let mut fac = ep1_facility();
    fac.run_analysis().unwrap();
    approx(fac.speed[8][0], fac.speed[7][0], 0.05, "overlap speed = merge speed");
}

#[test]
fn test_segment_los_matrix_period1_matches_exhibit_25_51() {
    let mut fac = ep1_facility();
    fac.run_analysis().unwrap();
    use LevelOfService as L;
    let expected = [L::C, L::C, L::D, L::C, L::D, L::C, L::D, L::D, L::D, L::D, L::D];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(fac.los[i][0], *e, "LOS seg {} p1", i + 1);
    }
}

#[test]
fn test_validation_rejects_non_basic_termini() {
    let mut fac = ep1_facility();
    fac.segments[0].seg_type = SegmentType::Merge;
    assert!(fac.validate().is_err());
}

#[test]
fn test_segmentation_rules_exhibit_10_11() {
    // (a) 4,000 ft between ramps: merge 1,500 + basic 1,000 + diverge 1,500
    let segs = segment_ramp_section(4000.0, false);
    assert_eq!(
        segs,
        vec![
            (SegmentType::Merge, 1500.0),
            (SegmentType::Basic, 1000.0),
            (SegmentType::Diverge, 1500.0),
        ]
    );
    // (b) 3,000 ft: merge and diverge influence areas define the whole length
    let segs = segment_ramp_section(3000.0, false);
    assert_eq!(
        segs,
        vec![(SegmentType::Merge, 1500.0), (SegmentType::Diverge, 1500.0)]
    );
    // (c) 2,000 ft: 1,000 ft of overlap (merge 500 + overlap 1,000 + diverge 500)
    let segs = segment_ramp_section(2000.0, false);
    assert_eq!(
        segs,
        vec![
            (SegmentType::Merge, 500.0),
            (SegmentType::OverlappingRamp, 1000.0),
            (SegmentType::Diverge, 500.0),
        ]
    );
    // Auxiliary lane: whole section is a weaving segment (Exhibit 10-12)
    let segs = segment_ramp_section(2640.0, true);
    assert_eq!(segs, vec![(SegmentType::Weaving, 2640.0)]);
    // Spacing under 1,500 ft without an auxiliary lane: worst case applies
    // over the whole distance
    let segs = segment_ramp_section(1200.0, false);
    assert_eq!(segs, vec![(SegmentType::OverlappingRamp, 1200.0)]);
}

#[test]
fn test_oversaturated_flags_and_bottleneck_metering() {
    // Example Problem 2 demands (+11%): segments 8-11 exceed capacity in
    // period 3 and the facility switches to the oversaturated engine.
    let mut fac = ep2_facility();
    fac.run_analysis().unwrap();
    assert!(fac.oversaturated);
    assert_eq!(fac.first_oversat_period, Some(2));
    // The active bottleneck (segment 8) serves at most its capacity.
    assert!(fac.vc_ratio[7][2] <= 1.005, "va/c = {}", fac.vc_ratio[7][2]);
    // Queues form upstream of the bottleneck (segments 5-7) in period 3.
    assert!(fac.had_queue[6][2], "segment 7 should be queued in p3");
    // Demand-based LOS F for segments 8-11 in period 3 (Exhibit 25-59).
    for i in 7..11 {
        assert_eq!(
            fac.demand_based_los[i][2],
            Some(LevelOfService::F),
            "demand-based LOS seg {} p3",
            i + 1
        );
    }
    // Facility LOS F in period 3 (any segment vd/c > 1.0; Exhibit 10-6).
    assert_eq!(fac.facility_performance[2].los, LevelOfService::F);
}

pub(crate) fn ep2_facility() -> FreewayFacility {
    // Example Problem 2 (Exhibit 25-53): +11% demand vs Example Problem 1.
    example_facility(
        vec![5001.0, 5500.0, 5800.0, 5200.0, 4201.0],
        vec![500.0, 599.0, 699.0, 400.0, 200.0],
        vec![599.0, 799.0, 899.0, 400.0, 300.0],
        vec![500.0, 599.0, 699.0, 500.0, 300.0],
        vec![300.0, 400.0, 300.0, 300.0, 300.0],
        vec![400.0, 400.0, 400.0, 400.0, 200.0],
        vec![300.0, 300.0, 500.0, 300.0, 200.0],
        vec![56.0, 111.0, 167.0, 89.0, 56.0],
    )
}

#[test]
fn test_work_zone_reduces_capacity_and_speed() {
    use super::exhibits::WorkZone;
    let mut fac = ep1_facility();
    // 3-to-2 nighttime closure with soft barrier on segment 5:
    // LCSI = 0.75, QDR_wz = 2,093 - 115.5 - 194 + 54 - 59 = 1,778.5 pc/h/ln,
    // c_wz = 1,778.5 / 0.866 = 2,053.7 pc/h/ln -> CAF_wz = 0.893.
    fac.segments[4].work_zone = Some(WorkZone {
        total_lanes: 3,
        open_lanes: 2,
        soft_barrier: true,
        rural: false,
        lateral_distance_ft: 6.0,
        night: true,
        speed_ratio: 1.1,
        speed_limit_mi_h: 50.0,
        total_ramp_density: 1.0,
        ..Default::default()
    });
    fac.segments[4].lanes = 2;
    let mut base = ep1_facility();
    base.run_analysis().unwrap();
    fac.run_analysis().unwrap();
    assert!(
        fac.capacity[4][0] < base.capacity[4][0] * 2.0 / 3.0,
        "work zone capacity should reflect the lane closure and CAF_wz"
    );
    assert!(fac.speed[4][0] < base.speed[4][0]);
}
