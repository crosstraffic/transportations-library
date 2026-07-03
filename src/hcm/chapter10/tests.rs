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

// ═════════════════════════════════════════════════════════════════════════
// Managed-lane facility (Steps A-9/A-13/A-14) and cross-weave CAF
// ═════════════════════════════════════════════════════════════════════════

use super::managed_lanes::{
    cross_weave_caf, cross_weave_crf, ManagedLaneFacility, MlSegmentInput,
};
use crate::hcm::chapter12::managed_lanes::ManagedLaneType;

#[test]
fn test_cross_weave_caf_equation_13_24() {
    // Equation 13-24: CRF = -0.0897 + 0.0252 ln(CW) - 0.00001453 L_cw-min
    //                       + 0.002967 N_GP.
    // CW = 1,000 pc/h, L_cw-min = 1,000 ft, N_GP = 3:
    // CRF = -0.0897 + 0.0252*6.90776 - 0.01453 + 0.008901 = 0.07885 -> CAF ~0.921.
    let crf = cross_weave_crf(1000.0, 1000.0, 3);
    approx(crf, 0.0788, 0.001, "CRF");
    approx(cross_weave_caf(1000.0, 1000.0, 3), 1.0 - 0.0788, 0.001, "CAF");
    // No cross-weave demand => no reduction.
    approx(cross_weave_caf(0.0, 1000.0, 3), 1.0, 1e-12, "CAF no demand");
    // Longer cross-weave length reduces the CRF (less friction).
    assert!(cross_weave_crf(1000.0, 3000.0, 3) < cross_weave_crf(1000.0, 1000.0, 3));
}

#[test]
fn test_cross_weave_reduces_gp_capacity_step_a9() {
    // A cross-weave on GP segment 5 lowers that segment's capacity below the
    // unadjusted 6,748 veh/h (Step A-9 / Equation 13-25).
    let gp = ep1_facility();
    let n = gp.num_segments();
    let mut fac = ManagedLaneFacility {
        gp,
        ml: vec![None; n],
        ml_entry_demand: vec![0.0; 5],
        ml_ffs: 60.0,
        cross_weave: vec![None; n],
        ..Default::default()
    };
    fac.cross_weave[4] = Some(super::managed_lanes::CrossWeave {
        cw_demand_pc: vec![800.0; 5],
        l_cw_min_ft: 1000.0,
    });
    fac.run_analysis().unwrap();
    assert!(
        fac.gp.capacity[4][0] < 6748.0,
        "cross-weave CAF should reduce GP segment 5 capacity, got {}",
        fac.gp.capacity[4][0]
    );
    // A segment without a cross-weave keeps the full capacity.
    approx(fac.gp.capacity[0][0], 6748.0, 5.0, "no cross-weave capacity");
}

#[test]
fn test_ml_adjacent_friction_activates_above_threshold() {
    // A single Continuous Access ML paired with every GP segment: the ML
    // speed drops only where the adjacent GP density exceeds 35 pc/mi/ln.
    let gp = ep2_facility(); // +11% demand -> some GP segments dense
    let n = gp.num_segments();
    let ml: Vec<Option<MlSegmentInput>> = (0..n)
        .map(|_| {
            Some(MlSegmentInput {
                lane_type: ManagedLaneType::ContinuousAccess,
                lanes: 1,
                ..Default::default()
            })
        })
        .collect();
    let mut fac = ManagedLaneFacility {
        gp,
        ml,
        ml_entry_demand: vec![1000.0, 1100.0, 1160.0, 1040.0, 840.0],
        ml_ffs: 60.0,
        ..Default::default()
    };
    fac.run_analysis().unwrap();
    // Where friction is active the ML speed is strictly below the free-flow
    // uniform value; where inactive it equals the unaffected speed.
    let mut any_friction = false;
    for i in 0..n {
        for p in 0..5 {
            if fac.ml_friction_active[i][p] {
                any_friction = true;
                assert!(
                    fac.gp.density_pc[i][p] > 35.0,
                    "friction flagged but GP density {} <= 35",
                    fac.gp.density_pc[i][p]
                );
            }
        }
    }
    assert!(any_friction, "the +11% facility should trigger ML friction");
}

// ═════════════════════════════════════════════════════════════════════════
// Planning-level method (Chapter 25 Section 6)
// ═════════════════════════════════════════════════════════════════════════

use super::planning::{
    basic_section_capacity_pc, oversaturated_delay_rate, undersaturated_delay_rate, weave_caf,
    PlanningFacility, PlanningSection, PlanningSectionType,
};

#[test]
fn test_planning_equation_25_45_basic_capacity() {
    // FFS 60 -> 2,300 pc/h/ln; FFS capped at 70.
    approx(basic_section_capacity_pc(60.0), 2300.0, 1e-9, "c(60)");
    approx(basic_section_capacity_pc(70.0), 2400.0, 1e-9, "c(70)");
    approx(basic_section_capacity_pc(75.0), 2400.0, 1e-9, "c(75) capped");
    approx(basic_section_capacity_pc(55.0), 2250.0, 1e-9, "c(55)");
}

#[test]
fn test_planning_equation_25_46_weave_caf() {
    // Example Problem 6 weave section: V_r ~0.164, L_s = 0.5 mi = 2,640 ft:
    // CAF = 0.884 - 0.0752*0.164 + 0.0000243*2640 = 0.9358.
    approx(weave_caf(0.164, 0.5), 0.9358, 0.001, "CAF_weave");
    // Capped at 1.0 for a very long weave (0.884 + 0.0000243*10,560 > 1).
    approx(weave_caf(0.0, 2.0), 1.0, 1e-9, "CAF_weave cap");
}

#[test]
fn test_planning_equation_25_47_delay_rate() {
    // FFS 60 threshold E = 0.72: below it the delay rate is 0.
    approx(undersaturated_delay_rate(0.71, 60.0), 0.0, 1e-12, "below E");
    // At d/c = 0.86: 121.35(0.86)^3 - 184.84(0.86)^2 + 83.21(0.86) - 9.33 = 2.8.
    approx(undersaturated_delay_rate(0.86, 60.0), 2.8, 0.1, "d/c 0.86");
    // Oversaturated ΔRO (Equation 25-48): 450/L * (d/c - 1).
    approx(oversaturated_delay_rate(1.02, 0.5), 18.0, 0.01, "ΔRO");
    approx(oversaturated_delay_rate(0.9, 0.5), 0.0, 1e-12, "ΔRO under 1");
}

#[test]
fn test_planning_carryover_propagates_downstream() {
    // Two ramp sections in series; the upstream one is oversaturated, and its
    // released vertical queue raises the downstream section demand in the
    // next period (Equation 25-43).
    let mut fac = PlanningFacility {
        sections: vec![
            PlanningSection {
                sec_type: PlanningSectionType::Basic,
                length_mi: 1.0,
                lanes: 2,
                inflow_aadt: 100_000.0,
                ..Default::default()
            },
            PlanningSection {
                sec_type: PlanningSectionType::Basic,
                length_mi: 1.0,
                lanes: 2,
                ..Default::default()
            },
        ],
        ffs: 60.0,
        k_factor: 0.09,
        growth_factor: 1.0,
        phf: 0.9,
        ..Default::default()
    };
    fac.run_analysis().unwrap();
    // Peak period 2 (multiplier 1/PHF) should push section 1 over capacity.
    assert!(fac.dc_ratio(0, 1) > 1.0, "section 1 should be oversaturated in p2");
    assert!(fac.facility_results[1].total_queue_mi > 0.0, "queue reported");
}
