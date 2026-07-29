//! HCM Edition 7.1 (November 2025), Chapter 14: Freeway Merge and Diverge Segments.
//!
//! Edition 7.1 replaces the 7th Edition ramp chapter. The 7th Edition estimated the flow in Lanes
//! 1 and 2, then a ramp influence area density from it; Edition 7.1 drops the lane-distribution
//! model entirely and, like the new Chapter 13, subtracts a speed impedance from the speed of an
//! equivalent basic segment (Equations 14-2 and 14-3) before deriving capacity from the 35
//! pc/mi/ln breakdown density. The two editions therefore report different speeds, densities,
//! capacities, and LOS letters for the same junction. See [`crate::hcm::common::HcmVersion`].
//!
//! Implemented steps:
//!
//! * Step 1, demand adjustment (Equation 14-1)
//! * Step 2, speed in the ramp influence area (Equations 14-2 through 14-5)
//! * Step 3, capacity and the three capacity checks (Equations 14-6 through 14-14, Exhibits 14-8
//!   through 14-10)
//! * Step 4, density and LOS (Equations 14-15 and 14-16, Exhibit 14-2)

use serde::{Deserialize, Serialize};

use crate::hcm::basicfreeways::basicfreeways::{
    basic_segment_breakpoint, basic_segment_capacity, basic_segment_speed, EXPONENT_BASIC_FREEWAY,
};
use crate::hcm::common::los_tables::los_merge_diverge_v7_1;
use crate::hcm::common::LevelOfService;

use super::merge_diverge::{RampLanes, RampSegment};

// ═══════════════════════════════════════════════════════════════════════════════
// Constants
// ═══════════════════════════════════════════════════════════════════════════════

/// Density at which a merge or diverge segment is expected to break down (pc/mi/ln) - Step 3.
///
/// The same threshold Edition 7.1 adopted for weaving segments, and the reason both chapters now
/// share one set of LOS bands.
pub const RAMP_BREAKDOWN_DENSITY: f64 = 35.0;

/// Per-lane flow below which ramp turbulence does not reduce speed (pc/h/ln) - Equations 14-4/14-5.
pub const IMPEDANCE_FLOW_THRESHOLD: f64 = 500.0;

/// Merge speed model coefficient - Equation 14-4.
pub const MERGE_IMPEDANCE_COEFFICIENT: f64 = 0.00408;

/// Diverge speed model coefficient - Equation 14-5.
pub const DIVERGE_IMPEDANCE_COEFFICIENT: f64 = 0.00014;

/// Exponent applied to the deceleration lane length in the diverge speed model - Equation 14-5.
pub const DIVERGE_DECEL_EXPONENT: f64 = 0.536;

// ═══════════════════════════════════════════════════════════════════════════════
// Step 2: Speed
// ═══════════════════════════════════════════════════════════════════════════════

/// Speed impedance due to merging SIM (mi/h) - Equation 14-4.
///
/// `SIM = max[0, 0.00408 ((v_F + v_R)/N - 500) (v_R/L_A)]`. The merge model uses the flow
/// downstream of the on-ramp, so the ramp flow appears both in the per-lane term and in the
/// turbulence term.
///
/// * `flow_per_lane` - `(v_F + v_R)/N` (pc/h/ln)
/// * `flow_ramp` - ramp demand flow rate v_R (pc/h)
/// * `accel_length` - acceleration lane length L_A (ft)
pub fn merge_speed_impedance(flow_per_lane: f64, flow_ramp: f64, accel_length: f64) -> f64 {
    if accel_length <= 0.0 {
        return 0.0;
    }
    (MERGE_IMPEDANCE_COEFFICIENT
        * (flow_per_lane - IMPEDANCE_FLOW_THRESHOLD)
        * (flow_ramp / accel_length))
        .max(0.0)
}

/// Speed impedance due to diverging SID (mi/h) - Equation 14-5.
///
/// `SID = max[0, 0.00014 (v_F/N - 500) (v_R/L_D^0.536)]`. The diverge model uses the mainline flow
/// approaching the off-ramp, which already contains the exiting vehicles, so v_R does not appear
/// in the per-lane term.
///
/// * `flow_per_lane` - `v_F/N` (pc/h/ln)
/// * `flow_ramp` - ramp demand flow rate v_R (pc/h)
/// * `decel_length` - deceleration lane length L_D (ft)
pub fn diverge_speed_impedance(flow_per_lane: f64, flow_ramp: f64, decel_length: f64) -> f64 {
    if decel_length <= 0.0 {
        return 0.0;
    }
    (DIVERGE_IMPEDANCE_COEFFICIENT
        * (flow_per_lane - IMPEDANCE_FLOW_THRESHOLD)
        * (flow_ramp / decel_length.powf(DIVERGE_DECEL_EXPONENT)))
    .max(0.0)
}

// ═══════════════════════════════════════════════════════════════════════════════
// Step 3: Capacity
// ═══════════════════════════════════════════════════════════════════════════════

/// Capacity of a merge or diverge ramp influence area (pc/h/ln) - Equation 14-8.
///
/// Capacity is the per-lane flow at which the segment reaches the 35 pc/mi/ln breakdown density.
/// Substituting the speed impedance into `C/35 = S_b(C) - SI(C)` gives a quadratic in `C`, whose
/// `b` and `c` coefficients (Equations 14-10/14-11 for a merge, 14-13/14-14 for a diverge) are the
/// speed model coefficient scaled by 35 and by 35 x 500 respectively.
///
/// * `turbulence` - `v_R/L_A` for a merge, `v_R/L_D^0.536` for a diverge
/// * `coefficient` - [`MERGE_IMPEDANCE_COEFFICIENT`] or [`DIVERGE_IMPEDANCE_COEFFICIENT`]
///
/// Returns `None` when the quadratic has no real root, rather than a NaN that would travel
/// downstream looking like a capacity.
pub fn ramp_capacity_per_lane(
    turbulence: f64,
    coefficient: f64,
    ffs_adj: f64,
    capacity_basic_adj: f64,
    breakpoint_adj: f64,
) -> Option<f64> {
    let denom = (capacity_basic_adj - breakpoint_adj).powi(2);
    if denom <= 0.0 {
        return None;
    }
    // Equations 14-9 / 14-12.
    let a = RAMP_BREAKDOWN_DENSITY * (ffs_adj - capacity_basic_adj / 45.0) / denom;
    if a == 0.0 {
        return None;
    }
    // Equations 14-10 / 14-13: the printed 0.143 and 0.0049 are the speed model coefficient
    // times the breakdown density.
    let b = 1.0 + RAMP_BREAKDOWN_DENSITY * coefficient * turbulence - (2.0 * a * breakpoint_adj);
    // Equations 14-11 / 14-14: the printed 71.4 and 2.45 are that product times the 500 pc/h/ln
    // impedance threshold.
    let c = (a * breakpoint_adj.powi(2))
        - (RAMP_BREAKDOWN_DENSITY * ffs_adj)
        - (RAMP_BREAKDOWN_DENSITY * IMPEDANCE_FLOW_THRESHOLD * coefficient * turbulence);
    let discriminant = b.powi(2) - 4.0 * a * c;
    if discriminant < 0.0 {
        return None;
    }
    Some((-b + discriminant.sqrt()) / (2.0 * a))
}

/// Capacity of a neighboring freeway segment (pc/h) - Exhibit 14-8.
///
/// The capacity immediately downstream of a merge or upstream of a diverge, which is a basic
/// freeway segment capacity. Returns the whole-segment value for `lanes` lanes in one direction.
pub fn neighboring_freeway_capacity(ffs: f64, lanes: u32) -> f64 {
    let per_lane = match ffs {
        f if f >= 70.0 => 2400.0,
        f if f >= 65.0 => 2350.0,
        f if f >= 60.0 => 2300.0,
        _ => 2250.0,
    };
    per_lane * lanes as f64
}

/// Capacity of a ramp roadway (pc/h) - Exhibit 14-10.
///
/// Two-lane values are twice the one-lane values in the printed exhibit, and the manual notes they
/// rest on limited data and may need local calibration.
pub fn ramp_roadway_capacity(ramp_ffs: f64, lanes: RampLanes) -> f64 {
    let one_lane = match ramp_ffs {
        f if f > 50.0 => 2200.0,
        f if f > 40.0 => 2100.0,
        f if f > 30.0 => 2000.0,
        f if f >= 20.0 => 1900.0,
        _ => 1800.0,
    };
    match lanes {
        RampLanes::OneLane => one_lane,
        RampLanes::TwoLane => one_lane * 2.0,
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Full analysis
// ═══════════════════════════════════════════════════════════════════════════════

/// Result of an Edition 7.1 merge or diverge analysis.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RampAnalysis {
    /// Freeway demand flow rate v_F (pc/h) - Equation 14-1.
    pub flow_freeway: f64,
    /// Ramp demand flow rate v_R (pc/h) - Equation 14-1.
    pub flow_ramp: f64,
    /// Flow rate governing the influence area: `v_F + v_R` for a merge, `v_F` for a diverge (pc/h).
    pub flow_influence: f64,
    /// Per-lane flow rate in the influence area (pc/h/ln).
    pub flow_per_lane: f64,
    /// Adjusted free-flow speed FFS_adj = FFS x SAF (mi/h).
    pub ffs_adj: f64,
    /// Equivalent basic segment adjusted capacity C_b,adj (pc/h/ln) - Exhibit 12-6.
    pub capacity_basic_adj: f64,
    /// Equivalent basic segment adjusted breakpoint BP_adj (pc/h/ln) - Exhibit 12-6.
    pub breakpoint_adj: f64,
    /// Speed of the equivalent basic segment S_b (mi/h) - Equation 12-1.
    pub speed_basic: f64,
    /// Speed impedance SIM or SID (mi/h) - Equations 14-4/14-5.
    pub speed_impedance: f64,
    /// Average speed in the ramp influence area S_M or S_D (mi/h) - Equations 14-2/14-3.
    pub speed_avg: f64,
    /// Ramp influence area capacity C_M or C_D (pc/h/ln) - Equation 14-8.
    pub capacity_per_lane: Option<f64>,
    /// Demand-to-capacity ratio of the ramp influence area.
    pub dc_ratio: Option<f64>,
    /// Capacity of the neighboring freeway segment (pc/h) - Exhibit 14-8.
    pub capacity_neighboring_freeway: f64,
    /// Capacity of the ramp roadway (pc/h) - Exhibit 14-10.
    pub capacity_ramp_roadway: f64,
    /// Whether any of the three capacity checks fails.
    pub demand_exceeds_capacity: bool,
    /// Density in the ramp influence area D_M or D_D (pc/mi/ln) - Equations 14-15/14-16.
    pub density: f64,
    /// Level of service - Exhibit 14-2.
    pub los: LevelOfService,
}

impl RampSegment {
    /// Freeway and ramp demand flow rates under equivalent base conditions (pc/h) - Equation 14-1.
    ///
    /// Unlike the 7th Edition path this applies no Lane 5 deduction on 10-lane freeways. That
    /// deduction (Exhibit 14-19, 7th Edition Equation 14-27) existed to feed the lane-distribution
    /// model, and Edition 7.1 has no lane-distribution model: its speed and capacity equations read
    /// the total mainline flow per lane across all N lanes.
    fn demand_flows_v7_1(&self) -> (f64, f64) {
        let f_hv_freeway = self.fhv_for(self.heavy_vehicle_pct);
        let f_hv_ramp = self.fhv_for(self.ramp_heavy_vehicle_pct.unwrap_or(self.heavy_vehicle_pct));
        let to_flow = |v: f64, f_hv: f64| {
            if self.phf > 0.0 && f_hv > 0.0 {
                v / (self.phf * f_hv)
            } else {
                0.0
            }
        };
        (
            to_flow(self.freeway_demand, f_hv_freeway),
            to_flow(self.ramp_demand, f_hv_ramp),
        )
    }

    /// Run the Edition 7.1 Chapter 14 methodology (Steps 1 through 4).
    ///
    /// Called by [`RampSegment::run_analysis`] when the segment's version is
    /// [`crate::hcm::common::HcmVersion::V7_1`].
    pub fn analyze_v7_1(&self) -> RampAnalysis {
        let is_merge = self.is_on_ramp();

        // Step 1: Equation 14-1.
        let (flow_freeway, flow_ramp) = self.demand_flows_v7_1();

        // The merge model works on the flow downstream of the on-ramp; the diverge model on the
        // mainline flow approaching the off-ramp, which already contains the exiting vehicles.
        let flow_influence = if is_merge {
            flow_freeway + flow_ramp
        } else {
            flow_freeway
        };
        let lanes = self.freeway_lanes.max(1) as f64;
        let flow_per_lane = flow_influence / lanes;

        // Step 2: equivalent basic segment, then the speed impedance.
        let ffs_adj = self.freeway_ffs * self.saf;
        // Equation 12-6 reads the unadjusted FFS (December 2022 corrections); SAF reaches
        // capacity only through CAF. The breakpoint below does use FFS_adj.
        let capacity_basic_adj = basic_segment_capacity(self.freeway_ffs) * self.caf;
        let breakpoint_adj = basic_segment_breakpoint(ffs_adj, self.caf);
        let speed_basic = basic_segment_speed(
            flow_per_lane,
            ffs_adj,
            capacity_basic_adj,
            breakpoint_adj,
            EXPONENT_BASIC_FREEWAY,
        );

        let (speed_impedance, turbulence, coefficient) = if is_merge {
            let l_a = self.accel_lane_length.unwrap_or(0.0);
            let turbulence = if l_a > 0.0 { flow_ramp / l_a } else { 0.0 };
            (
                merge_speed_impedance(flow_per_lane, flow_ramp, l_a),
                turbulence,
                MERGE_IMPEDANCE_COEFFICIENT,
            )
        } else {
            let l_d = self.decel_lane_length.unwrap_or(0.0);
            let turbulence = if l_d > 0.0 {
                flow_ramp / l_d.powf(DIVERGE_DECEL_EXPONENT)
            } else {
                0.0
            };
            (
                diverge_speed_impedance(flow_per_lane, flow_ramp, l_d),
                turbulence,
                DIVERGE_IMPEDANCE_COEFFICIENT,
            )
        };
        // Equations 14-2 / 14-3.
        let speed_avg = speed_basic - speed_impedance;

        // Step 3: the three capacity checks.
        let capacity_per_lane = ramp_capacity_per_lane(
            turbulence,
            coefficient,
            ffs_adj,
            capacity_basic_adj,
            breakpoint_adj,
        );
        let dc_ratio = capacity_per_lane.and_then(|c| {
            if c > 0.0 {
                Some(flow_per_lane / c)
            } else {
                None
            }
        });
        // Exhibit 14-8 tabulates Equation 12-6 capacities by FFS, so it takes the unadjusted
        // FFS on the same reasoning. The corrections address Equations 12-6/12-7 explicitly and
        // this exhibit only by implication; flagged in VERIFICATION.md.
        let capacity_neighboring_freeway =
            neighboring_freeway_capacity(self.freeway_ffs, self.freeway_lanes) * self.caf;
        let capacity_ramp_roadway = ramp_roadway_capacity(self.ramp_ffs, self.ramp_lanes);

        // The neighboring-segment check applies to the flow that segment carries: downstream of a
        // merge that is v_F + v_R, upstream of a diverge it is v_F.
        let demand_exceeds_capacity = dc_ratio.map(|r| r > 1.0).unwrap_or(false)
            || flow_influence > capacity_neighboring_freeway
            || flow_ramp > capacity_ramp_roadway;

        // Step 4: Equations 14-15 / 14-16.
        let density = if speed_avg > 0.0 {
            flow_per_lane / speed_avg
        } else {
            f64::INFINITY
        };
        let los = los_merge_diverge_v7_1(density, demand_exceeds_capacity);

        RampAnalysis {
            flow_freeway,
            flow_ramp,
            flow_influence,
            flow_per_lane,
            ffs_adj,
            capacity_basic_adj,
            breakpoint_adj,
            speed_basic,
            speed_impedance,
            speed_avg,
            capacity_per_lane,
            dc_ratio,
            capacity_neighboring_freeway,
            capacity_ramp_roadway,
            demand_exceeds_capacity,
            density,
            los,
        }
    }

    /// Run the Edition 7.1 methodology and store its results, returning the LOS.
    ///
    /// Populates the fields the two editions share and the full typed result in `analysis_v7_1`.
    /// The 7th Edition lane-distribution outputs (`p_f`, `v_12`, `v_r12`, `v_oa`, `speed_outer`)
    /// are left alone, because Edition 7.1 does not model lane distribution at all and writing a
    /// value there would invent one.
    pub fn run_analysis_v7_1(&mut self) -> LevelOfService {
        let a = self.analyze_v7_1();

        self.flow_freeway = Some(a.flow_freeway);
        self.flow_ramp = Some(a.flow_ramp);
        self.speed_ramp = Some(a.speed_avg);
        self.speed_avg = Some(a.speed_avg);
        self.density = Some(a.density);
        self.capacity_ramp = Some(a.capacity_ramp_roadway);
        self.capacity_freeway = Some(a.capacity_neighboring_freeway);
        self.vc_ratio = a.dc_ratio;
        self.demand_exceeds_capacity = Some(a.demand_exceeds_capacity);
        self.los = Some(a.los);

        let los = a.los;
        self.analysis_v7_1 = Some(a);
        los
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The capacity quadratic reproduces the definition it was derived from: at C the segment sits
    /// exactly at the 35 pc/mi/ln breakdown density (Equations 14-6/14-7).
    #[test]
    fn capacity_lands_on_the_breakdown_density() {
        let (ffs, caf) = (70.0, 1.0);
        let c_b = basic_segment_capacity(ffs) * caf;
        let bp = basic_segment_breakpoint(ffs, caf);

        // Merge.
        let (v_r, l_a) = (600.0, 800.0);
        let turbulence = v_r / l_a;
        let cm =
            ramp_capacity_per_lane(turbulence, MERGE_IMPEDANCE_COEFFICIENT, ffs, c_b, bp).unwrap();
        let s_b = basic_segment_speed(cm, ffs, c_b, bp, EXPONENT_BASIC_FREEWAY);
        let s_m = s_b - merge_speed_impedance(cm, v_r, l_a);
        assert!(
            (cm / s_m - RAMP_BREAKDOWN_DENSITY).abs() < 0.05,
            "merge density at capacity {}",
            cm / s_m
        );

        // Diverge.
        let l_d = 500.0f64;
        let turbulence = v_r / l_d.powf(DIVERGE_DECEL_EXPONENT);
        let cd = ramp_capacity_per_lane(turbulence, DIVERGE_IMPEDANCE_COEFFICIENT, ffs, c_b, bp)
            .unwrap();
        let s_b = basic_segment_speed(cd, ffs, c_b, bp, EXPONENT_BASIC_FREEWAY);
        let s_d = s_b - diverge_speed_impedance(cd, v_r, l_d);
        assert!(
            (cd / s_d - RAMP_BREAKDOWN_DENSITY).abs() < 0.05,
            "diverge density at capacity {}",
            cd / s_d
        );
    }

    /// "In general, the merge model will in most cases yield a lower capacity than the diverge
    /// model, all other parameters being equal." The hedge is load-bearing. Setting the two
    /// impedance terms equal at a common ramp-lane length L gives 0.00408/L = 0.00014/L^0.536,
    /// whose solution L = (0.00408/0.00014)^(1/0.464) is about 1,435 ft and does not depend on the
    /// ramp volume at all. Below that length the merge model governs, as the manual describes;
    /// above it the two cross over and the diverge capacity is marginally the lower of the two.
    #[test]
    fn merge_capacity_is_the_lower_of_the_two_below_the_crossover() {
        let ffs = 70.0;
        let c_b = basic_segment_capacity(ffs);
        let bp = basic_segment_breakpoint(ffs, 1.0);

        let capacities = |v_r: f64, len: f64| {
            let cm = ramp_capacity_per_lane(v_r / len, MERGE_IMPEDANCE_COEFFICIENT, ffs, c_b, bp)
                .unwrap();
            let cd = ramp_capacity_per_lane(
                v_r / len.powf(DIVERGE_DECEL_EXPONENT),
                DIVERGE_IMPEDANCE_COEFFICIENT,
                ffs,
                c_b,
                bp,
            )
            .unwrap();
            (cm, cd)
        };

        // The ordinary case: ramp-lane lengths across the range the manual's own defaults cover.
        for (v_r, len) in [(400.0f64, 600.0f64), (800.0, 1000.0), (1200.0, 1400.0)] {
            let (cm, cd) = capacities(v_r, len);
            assert!(cm < cd, "v_R {v_r}, L {len}: merge {cm} >= diverge {cd}");
        }

        // Past the crossover the ordering reverses, whatever the ramp volume.
        for v_r in [400.0f64, 800.0, 1200.0] {
            let (cm, cd) = capacities(v_r, 1600.0);
            assert!(cm > cd, "v_R {v_r}, L 1600: merge {cm} <= diverge {cd}");
        }

        // The crossover length itself is independent of ramp volume.
        for v_r in [300.0f64, 900.0, 1500.0] {
            let (cm, cd) = capacities(v_r, 1435.0);
            assert!(
                (cm - cd).abs() < 0.5,
                "v_R {v_r}: models should meet at ~1,435 ft, got {cm} vs {cd}"
            );
        }
    }

    /// Below 500 pc/h/ln neither impedance term bites, so the junction runs at basic-segment speed.
    #[test]
    fn impedance_is_zero_at_low_flow() {
        assert_eq!(merge_speed_impedance(400.0, 500.0, 800.0), 0.0);
        assert_eq!(diverge_speed_impedance(400.0, 500.0, 500.0), 0.0);
        assert!(merge_speed_impedance(1500.0, 500.0, 800.0) > 0.0);
        assert!(diverge_speed_impedance(1500.0, 500.0, 500.0) > 0.0);
    }

    /// A longer acceleration or deceleration lane spreads the turbulence and raises capacity.
    #[test]
    fn longer_ramp_lanes_raise_capacity() {
        let ffs = 70.0;
        let c_b = basic_segment_capacity(ffs);
        let bp = basic_segment_breakpoint(ffs, 1.0);
        let short =
            ramp_capacity_per_lane(600.0 / 400.0, MERGE_IMPEDANCE_COEFFICIENT, ffs, c_b, bp).unwrap();
        let long =
            ramp_capacity_per_lane(600.0 / 1600.0, MERGE_IMPEDANCE_COEFFICIENT, ffs, c_b, bp)
                .unwrap();
        assert!(long > short, "long {long} should exceed short {short}");
    }

    /// Exhibit 14-10 boundaries: the bands are open below and closed above, except the >=20-30 row.
    #[test]
    fn ramp_roadway_capacity_follows_exhibit_14_10() {
        assert_eq!(ramp_roadway_capacity(55.0, RampLanes::OneLane), 2200.0);
        assert_eq!(ramp_roadway_capacity(50.0, RampLanes::OneLane), 2100.0);
        assert_eq!(ramp_roadway_capacity(40.0, RampLanes::OneLane), 2000.0);
        assert_eq!(ramp_roadway_capacity(30.0, RampLanes::OneLane), 1900.0);
        assert_eq!(ramp_roadway_capacity(19.0, RampLanes::OneLane), 1800.0);
        assert_eq!(ramp_roadway_capacity(35.0, RampLanes::TwoLane), 4000.0);
    }

    /// Exhibit 14-8, per-lane values above 4 lanes.
    #[test]
    fn neighboring_freeway_capacity_follows_exhibit_14_8() {
        assert_eq!(neighboring_freeway_capacity(70.0, 3), 7200.0);
        assert_eq!(neighboring_freeway_capacity(65.0, 3), 7050.0);
        assert_eq!(neighboring_freeway_capacity(60.0, 2), 4600.0);
        assert_eq!(neighboring_freeway_capacity(55.0, 4), 9000.0);
    }
}
