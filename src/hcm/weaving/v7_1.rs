//! HCM Edition 7.1 (November 2025), Chapter 13: Freeway Weaving Segments.
//!
//! Edition 7.1 replaces the 7th Edition weaving chapter wholesale. The 7th Edition estimated
//! separate weaving and nonweaving speeds from lane-changing rates and averaged them; Edition 7.1
//! estimates one overall speed by subtracting a speed impedance term from the speed of an
//! equivalent basic segment (Equation 13-7), and derives capacity analytically from the density at
//! which weaving segments break down. The two editions therefore report different speeds,
//! capacities, and LOS letters for the same segment, which is why the edition is a selectable
//! input rather than a silent upgrade. See [`crate::hcm::common::HcmVersion`].
//!
//! Implemented steps:
//!
//! * Step 2, demand adjustment (Equation 13-1) and the simple weaving volume estimation method
//!   (Equations 13-2 through 13-6)
//! * Step 3, overall speed (Equations 13-7 through 13-14)
//! * Step 4, capacity and demand-to-capacity ratio (Equations 13-15 through 13-20)
//! * Step 5, density and LOS (Equation 13-21, Exhibit 13-7)

use serde::{Deserialize, Serialize};

use crate::hcm::basicfreeways::basicfreeways::{
    basic_segment_breakpoint, basic_segment_capacity, basic_segment_speed, EXPONENT_BASIC_FREEWAY,
};
use crate::hcm::common::los_tables::los_weaving_v7_1;
use crate::hcm::common::LevelOfService;

use super::weaving::{WeavingSegment, WeavingType};

// ═══════════════════════════════════════════════════════════════════════════════
// Constants
// ═══════════════════════════════════════════════════════════════════════════════

/// Density at which a weaving segment is expected to break down (pc/mi/ln) - Chapter 13, Step 4.
///
/// Edition 7.1 lowered this from the 7th Edition's 43 pc/mi/ln. A basic segment still breaks down
/// at 45 pc/mi/ln (Chapter 12, unchanged); the weaving value is lower because of the additional
/// turbulence.
pub const WEAVING_BREAKDOWN_DENSITY: f64 = 35.0;

/// Per-lane flow below which weaving turbulence does not reduce speed (pc/h/ln) - Equation 13-10.
///
/// At or below this flow the speed impedance term is zero and the weaving segment runs at the
/// speed of an equivalent basic segment.
pub const IMPEDANCE_FLOW_THRESHOLD: f64 = 500.0;

// ═══════════════════════════════════════════════════════════════════════════════
// Configuration class and speed model coefficients
// ═══════════════════════════════════════════════════════════════════════════════

/// Weaving configuration class, which selects the speed model coefficients.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WeavingClass {
    /// One-sided segment where every weaving movement takes exactly one lane change from exactly
    /// one lane: `LC_RF = LC_FR = NW_RF = NW_FR = 1` (Chapter 13, Section 2).
    Simple,
    /// Any other one-sided segment.
    Complex,
    /// Two-sided segment, where the ramp-to-ramp movement crosses the freeway.
    TwoSided,
}

/// Regression coefficients of the speed impedance model - Exhibits 13-13 and 13-14.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SpeedModelCoefficients {
    /// Multiplier alpha.
    pub alpha: f64,
    /// Exponent gamma on the configuration-weighted weaving flow.
    pub gamma: f64,
    /// Exponent delta on the reciprocal of the short length.
    pub delta: f64,
    /// Exponent epsilon on the number of lanes.
    pub epsilon: f64,
}

impl WeavingClass {
    /// Speed model coefficients for this configuration - Exhibit 13-13 (one-sided) and Exhibit
    /// 13-14 (two-sided).
    ///
    /// The two-sided row is identical to the simple one-sided row in the manual, value for value.
    /// That is what Exhibit 13-14 prints, not a transcription slip: the two models differ in the
    /// flow term they weight (Equation 13-13 uses the ramp-to-ramp flow alone), not in their
    /// coefficients.
    pub fn coefficients(&self) -> SpeedModelCoefficients {
        match self {
            WeavingClass::Simple | WeavingClass::TwoSided => SpeedModelCoefficients {
                alpha: 0.016,
                gamma: 0.021,
                delta: 0.181,
                epsilon: 3.217,
            },
            WeavingClass::Complex => SpeedModelCoefficients {
                alpha: 7.75,
                gamma: 0.200,
                delta: 1.02,
                epsilon: 3.850,
            },
        }
    }
}

/// Classify a weaving segment from its configuration parameters - Chapter 13, Section 2.
///
/// A simple weave is one where "the value of 1" holds for all four of `LC_RF`, `LC_FR`, `NW_RF`,
/// and `NW_FR`; every other one-sided configuration is complex. The class is derived rather than
/// supplied so it cannot contradict the lane-change and weaving-lane counts it is drawn from.
pub fn classify(
    weaving_type: WeavingType,
    lc_rf: u32,
    lc_fr: u32,
    nw_rf: u32,
    nw_fr: u32,
) -> WeavingClass {
    match weaving_type {
        WeavingType::TwoSided => WeavingClass::TwoSided,
        WeavingType::OneSided => {
            if lc_rf == 1 && lc_fr == 1 && nw_rf == 1 && nw_fr == 1 {
                WeavingClass::Simple
            } else {
                WeavingClass::Complex
            }
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Step 2: Volumes
// ═══════════════════════════════════════════════════════════════════════════════

/// The four weaving movement flow rates (pc/h), in the manual's order.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct MovementFlows {
    /// Ramp-to-freeway flow v_RF.
    pub v_rf: f64,
    /// Ramp-to-ramp flow v_RR.
    pub v_rr: f64,
    /// Freeway-to-ramp flow v_FR.
    pub v_fr: f64,
    /// Freeway-to-freeway flow v_FF.
    pub v_ff: f64,
}

/// Simple weaving volume estimation method - Equations 13-2 through 13-6.
///
/// Used when the four movement flows were not observed individually but the on-ramp, off-ramp, and
/// mainline flows are known. The method assumes the off-ramp attracts the same proportion `P` of
/// traffic from the mainline and from the on-ramp.
///
/// * `v_on` - on-ramp flow rate (pc/h)
/// * `v_off` - off-ramp flow rate (pc/h)
/// * `v_f` - freeway mainline flow rate entering the segment (pc/h)
/// * `v` - total flow rate in the segment (pc/h)
pub fn estimate_movement_flows(v_on: f64, v_off: f64, v_f: f64, v: f64) -> MovementFlows {
    // Equation 13-2: P = v_OFF / v.
    let p = if v > 0.0 { v_off / v } else { 0.0 };
    // Equations 13-3 through 13-6.
    let v_rf = v_on * (1.0 - p);
    let v_rr = v_on * p;
    let v_fr = v_off - v_rr;
    let v_ff = v_f - v_fr;
    MovementFlows {
        v_rf,
        v_rr,
        v_fr,
        v_ff,
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Step 3: Speed
// ═══════════════════════════════════════════════════════════════════════════════

/// Weaving intensity factor W - Equation 13-9 (one-sided) or the leading factors of Equation
/// 13-13 (two-sided).
///
/// W is the part of the speed impedance that does not depend on the per-lane flow, so Step 4 can
/// reuse it when solving for capacity.
///
/// * `configured_flow` - the configuration-weighted weaving flow: `(LC_RF+1)/(NW_RF+1) v_RF +
///   (LC_FR+1)/(NW_FR+1) v_FR` for a one-sided segment, `(LC_RR+1)/(NW_RR+1) v_RR` for a
///   two-sided one (pc/h)
/// * `num_lanes` - number of lanes in the segment N
/// * `length_short` - short length of the weaving segment L_s (ft)
pub fn weaving_intensity(
    configured_flow: f64,
    num_lanes: f64,
    length_short: f64,
    coeff: SpeedModelCoefficients,
) -> f64 {
    if num_lanes <= 0.0 || length_short <= 0.0 {
        return 0.0;
    }
    let base = configured_flow / num_lanes.powf(coeff.epsilon);
    if base <= 0.0 {
        return 0.0;
    }
    coeff.alpha * base.powf(coeff.gamma) * (1.0 / length_short).powf(coeff.delta)
}

/// The configuration-weighted weaving flow that Equation 13-8 raises to the power gamma (pc/h).
///
/// A movement's contribution is scaled by `(LC + 1)/(NW + 1)`: more required lane changes raise
/// its influence, more lanes from which the maneuver can be made lower it.
/// Takes each configuration parameter separately because that is how Chapter 13 names them; the
/// caller normally has them as individual fields on a segment rather than as a group.
#[allow(clippy::too_many_arguments)]
pub fn configured_weaving_flow(
    weaving_type: WeavingType,
    lc_rf: u32,
    lc_fr: u32,
    lc_rr: u32,
    nw_rf: u32,
    nw_fr: u32,
    nw_rr: u32,
    flows: MovementFlows,
) -> f64 {
    let weight = |lc: u32, nw: u32| (lc as f64 + 1.0) / (nw as f64 + 1.0);
    match weaving_type {
        WeavingType::OneSided => {
            weight(lc_rf, nw_rf) * flows.v_rf + weight(lc_fr, nw_fr) * flows.v_fr
        }
        WeavingType::TwoSided => weight(lc_rr, nw_rr) * flows.v_rr,
    }
}

/// Speed impedance due to weaving turbulence SIW (mi/h) - Equation 13-10.
///
/// `SIW = max[0, W (v/N - 500)]`. Below 500 pc/h/ln the segment runs at basic-segment speed.
pub fn speed_impedance(weaving_intensity: f64, flow_per_lane: f64) -> f64 {
    (weaving_intensity * (flow_per_lane - IMPEDANCE_FLOW_THRESHOLD)).max(0.0)
}

// ═══════════════════════════════════════════════════════════════════════════════
// Step 4: Capacity
// ═══════════════════════════════════════════════════════════════════════════════

/// Weaving segment capacity C_W (pc/h/ln) - Equations 13-16 through 13-19.
///
/// Capacity is the per-lane flow at which the segment reaches the 35 pc/mi/ln breakdown density.
/// Because the basic-segment speed is quadratic in flow, setting `C_W/35 = S_b - SIW` gives a
/// quadratic in `C_W` that is solved in closed form.
///
/// Returns `None` when the quadratic has no real root, which the printed procedure does not
/// contemplate; a caller gets an explicit absence rather than a NaN that would travel downstream
/// as a plausible-looking capacity.
pub fn weaving_capacity_per_lane(
    weaving_intensity: f64,
    ffs_adj: f64,
    capacity_basic_adj: f64,
    breakpoint_adj: f64,
) -> Option<f64> {
    let denom = (capacity_basic_adj - breakpoint_adj).powi(2);
    if denom <= 0.0 {
        return None;
    }
    // Equation 13-17. A non-positive `a` is off the model's fitted domain (it needs
    // FFS_adj > C_b,adj/45, which SAF below roughly 0.71 violates); the larger quadratic root
    // the method wants is `(-b + sqrt)/(2a)` only while `a` is positive, so bail out rather
    // than return the wrong root.
    let a = WEAVING_BREAKDOWN_DENSITY * (ffs_adj - capacity_basic_adj / 45.0) / denom;
    if a <= 0.0 {
        return None;
    }
    // Equation 13-18.
    let b = 1.0 + WEAVING_BREAKDOWN_DENSITY * weaving_intensity - (2.0 * a * breakpoint_adj);
    // Equation 13-19.
    let c = (a * breakpoint_adj.powi(2))
        - (IMPEDANCE_FLOW_THRESHOLD * WEAVING_BREAKDOWN_DENSITY * weaving_intensity)
        - (WEAVING_BREAKDOWN_DENSITY * ffs_adj);
    // Equation 13-16.
    let discriminant = b.powi(2) - 4.0 * a * c;
    if discriminant < 0.0 {
        return None;
    }
    Some((-b + discriminant.sqrt()) / (2.0 * a))
}

// ═══════════════════════════════════════════════════════════════════════════════
// Step 5: LOS
// ═══════════════════════════════════════════════════════════════════════════════

/// LOS for a weaving segment - Exhibit 13-7.
///
/// `A 0-11, B >11-18, C >18-25, D >25-30, E >30-35, F >35 or demand exceeds capacity.` These
/// thresholds are not the 7th Edition's (Exhibit 13-6, `A <=10 ... E <=43`); a density that read
/// LOS C under the 7th Edition can read LOS D here. The bands live in
/// [`crate::hcm::common::los_tables`], which Exhibit 14-2 shares.
pub fn determine_weaving_los(density: f64, demand_exceeds_capacity: bool) -> LevelOfService {
    los_weaving_v7_1(density, demand_exceeds_capacity)
}

// ═══════════════════════════════════════════════════════════════════════════════
// Full analysis
// ═══════════════════════════════════════════════════════════════════════════════

/// Result of an Edition 7.1 weaving segment analysis.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WeavingAnalysis {
    /// Configuration class that selected the speed model coefficients.
    pub class: WeavingClass,
    /// Heavy-vehicle adjustment factor f_HV - Equation 12-10.
    pub f_hv: f64,
    /// The four movement flow rates under equivalent base conditions (pc/h) - Equation 13-1.
    pub flows: MovementFlows,
    /// Total flow rate in the segment v (pc/h).
    pub flow_total: f64,
    /// Per-lane flow rate v/N (pc/h/ln).
    pub flow_per_lane: f64,
    /// Adjusted free-flow speed FFS_adj = FFS x SAF (mi/h).
    pub ffs_adj: f64,
    /// Equivalent basic segment adjusted capacity C_b,adj (pc/h/ln) - Exhibit 12-6.
    pub capacity_basic_adj: f64,
    /// Equivalent basic segment adjusted breakpoint BP_adj (pc/h/ln) - Exhibit 12-6.
    pub breakpoint_adj: f64,
    /// Speed of the equivalent basic segment S_b (mi/h) - Equation 12-1.
    pub speed_basic: f64,
    /// Weaving intensity factor W - Equation 13-9.
    pub weaving_intensity: f64,
    /// Speed impedance SIW (mi/h) - Equation 13-10.
    pub speed_impedance: f64,
    /// Overall mean speed of all vehicles S_o (mi/h) - Equation 13-7.
    pub speed_avg: f64,
    /// Weaving segment capacity C_W (pc/h/ln) - Equation 13-16.
    pub capacity_per_lane: Option<f64>,
    /// Weaving segment capacity across all lanes (pc/h).
    pub capacity_total: Option<f64>,
    /// Demand-to-capacity ratio - Equation 13-20.
    pub dc_ratio: Option<f64>,
    /// Whether demand exceeds capacity (d/c > 1.0).
    pub demand_exceeds_capacity: bool,
    /// Density D (pc/mi/ln) - Equation 13-21.
    pub density: f64,
    /// Level of service - Exhibit 13-7.
    pub los: LevelOfService,
}

impl WeavingSegment {
    /// Run the Edition 7.1 Chapter 13 methodology (Steps 2 through 5).
    ///
    /// Uses the segment's `nw_rf`/`nw_fr`/`nw_rr` weaving-lane counts, which the 7th Edition
    /// methodology does not read. Called by [`WeavingSegment::run_analysis`] when the segment's
    /// version is [`crate::hcm::common::HcmVersion::V7_1`].
    pub fn analyze_v7_1(&self) -> WeavingAnalysis {
        let class = classify(
            self.weaving_type,
            self.lc_rf,
            self.lc_fr,
            self.nw_rf,
            self.nw_fr,
        );

        // Step 2: Equation 13-1, demand volumes to flow rates under base conditions.
        let f_hv = self.calculate_fhv();
        let to_flow = |v: f64| {
            if self.phf > 0.0 && f_hv > 0.0 {
                v / (self.phf * f_hv)
            } else {
                0.0
            }
        };
        let flows = MovementFlows {
            v_rf: to_flow(self.v_rf),
            v_rr: to_flow(self.v_rr),
            v_fr: to_flow(self.v_fr),
            v_ff: to_flow(self.v_ff),
        };
        let flow_total = flows.v_ff + flows.v_fr + flows.v_rf + flows.v_rr;
        let num_lanes = self.num_lanes as f64;
        let flow_per_lane = if num_lanes > 0.0 {
            flow_total / num_lanes
        } else {
            0.0
        };

        // Step 3: equivalent basic segment, then the speed impedance.
        let ffs_adj = self.ffs * self.saf;
        // Equation 12-6 reads the unadjusted FFS (December 2022 corrections); SAF reaches
        // capacity only through CAF. The breakpoint below does use FFS_adj.
        let capacity_basic_adj = basic_segment_capacity(self.ffs) * self.caf;
        let breakpoint_adj = basic_segment_breakpoint(ffs_adj, self.caf);
        let speed_basic = basic_segment_speed(
            flow_per_lane,
            ffs_adj,
            capacity_basic_adj,
            breakpoint_adj,
            EXPONENT_BASIC_FREEWAY,
        );

        let configured_flow = configured_weaving_flow(
            self.weaving_type,
            self.lc_rf,
            self.lc_fr,
            self.lc_rr,
            self.nw_rf,
            self.nw_fr,
            self.nw_rr,
            flows,
        );
        let w = weaving_intensity(
            configured_flow,
            num_lanes,
            self.length_short,
            class.coefficients(),
        );
        let siw = speed_impedance(w, flow_per_lane);
        // Equation 13-7.
        let speed_avg = speed_basic - siw;

        // Step 4: capacity and d/c.
        let capacity_per_lane =
            weaving_capacity_per_lane(w, ffs_adj, capacity_basic_adj, breakpoint_adj);
        let capacity_total = capacity_per_lane.map(|c| c * num_lanes);
        let dc_ratio = capacity_per_lane.and_then(|c| {
            if c > 0.0 {
                Some(flow_per_lane / c)
            } else {
                None
            }
        });
        let demand_exceeds_capacity = dc_ratio.map(|r| r > 1.0).unwrap_or(false);

        // Step 5: density and LOS. Above capacity the speed from Step 3 is discarded and LOS F is
        // assigned, per the Level of Service F discussion in Step 4.
        let density = if speed_avg > 0.0 {
            flow_per_lane / speed_avg
        } else {
            f64::INFINITY
        };
        let los = determine_weaving_los(density, demand_exceeds_capacity);

        WeavingAnalysis {
            class,
            f_hv,
            flows,
            flow_total,
            flow_per_lane,
            ffs_adj,
            capacity_basic_adj,
            breakpoint_adj,
            speed_basic,
            weaving_intensity: w,
            speed_impedance: siw,
            speed_avg,
            capacity_per_lane,
            capacity_total,
            dc_ratio,
            demand_exceeds_capacity,
            density,
            los,
        }
    }
}

impl WeavingSegment {
    /// Run the Edition 7.1 methodology and store its results on the segment, returning the LOS.
    ///
    /// Populates the fields the two editions share (`f_hv`, `flow_total`, `speed_avg`, `density`,
    /// `demand_exceeds_capacity`, `los`, `weaving_intensity`) plus the full typed result in
    /// `analysis_v7_1`. Fields that exist only in the 7th Edition methodology - the lane-changing
    /// rates, the separate weaving and nonweaving speeds, `l_max`, `c_iwl`, and the veh/h
    /// `capacity` - are left as they were, because Edition 7.1 does not compute them and writing a
    /// value there would invent one.
    pub fn run_analysis_v7_1(&mut self) -> LevelOfService {
        let a = self.analyze_v7_1();

        self.f_hv = Some(a.f_hv);
        self.flow_total = Some(a.flow_total);
        self.weaving_intensity = Some(a.weaving_intensity);
        self.speed_avg = Some(a.speed_avg);
        self.density = Some(a.density);
        self.demand_exceeds_capacity = Some(a.demand_exceeds_capacity);
        self.los = Some(a.los);

        // The weaving and nonweaving split still describes the demand, even though Edition 7.1
        // no longer assigns the two groups different speeds.
        let (v_w, v_nw) = match self.weaving_type {
            WeavingType::OneSided => (a.flows.v_rf + a.flows.v_fr, a.flows.v_ff + a.flows.v_rr),
            WeavingType::TwoSided => (a.flows.v_rr, a.flows.v_ff + a.flows.v_fr + a.flows.v_rf),
        };
        self.flow_weaving = Some(v_w);
        self.flow_nonweaving = Some(v_nw);
        self.volume_ratio = Some(if a.flow_total > 0.0 {
            v_w / a.flow_total
        } else {
            0.0
        });

        let los = a.los;
        self.analysis_v7_1 = Some(a);
        los
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hcm::common::HcmVersion;

    /// Below FFS_adj = C_b,adj/45 the Equation 13-17 leading coefficient goes non-positive and
    /// the closed-form larger root no longer is `(-b + sqrt)/(2a)`; the solver must refuse
    /// rather than return the wrong root. FFS 75 with SAF 0.70 sits past that edge.
    #[test]
    fn capacity_solver_refuses_offdomain_negative_leading_coefficient() {
        let ffs_adj = 75.0 * 0.70;
        let c_b_adj = crate::hcm::basicfreeways::basicfreeways::basic_segment_capacity(75.0);
        let bp_adj = crate::hcm::basicfreeways::basicfreeways::basic_segment_breakpoint(ffs_adj, 1.0);
        assert!(weaving_capacity_per_lane(0.4, ffs_adj, c_b_adj, bp_adj).is_none());
    }

    /// Exhibit 27-2: the Example Problem 1 complex weaving segment.
    fn example_problem_1() -> WeavingSegment {
        WeavingSegment {
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
            // "Complex 0-1": either on-ramp lane can weave with zero or one lane changes, and the
            // right-hand mainline lane needs one lane change.
            lc_rf: 0,
            lc_fr: 1,
            nw_rf: 2,
            nw_fr: 1,
            ..Default::default()
        }
    }

    /// Chapter 27, Example Problem 1: LOS of a complex weave. Every published intermediate value
    /// is asserted, because a wrong speed and a wrong capacity can still land on the right LOS.
    #[test]
    fn example_problem_1_complex_weave() {
        let a = example_problem_1().analyze_v7_1();

        assert_eq!(a.class, WeavingClass::Complex);
        assert!((a.f_hv - 0.952).abs() < 0.001, "f_HV {}", a.f_hv);
        assert!((a.flows.v_ff - 2095.0).abs() < 1.0, "v_FF {}", a.flows.v_ff);
        assert!((a.flows.v_fr - 799.0).abs() < 1.0, "v_FR {}", a.flows.v_fr);
        assert!((a.flows.v_rf - 1197.0).abs() < 1.0, "v_RF {}", a.flows.v_rf);
        assert!((a.flows.v_rr - 1497.0).abs() < 1.0, "v_RR {}", a.flows.v_rr);
        // The manual sums the four movement flows after rounding each to a whole pc/h and after
        // rounding f_HV to 0.952, giving 5,588; carrying full precision gives 5,585.8. The 2.2
        // pc/h gap is that rounding, not a different calculation.
        assert!((a.flow_total - 5588.0).abs() < 3.0, "v {}", a.flow_total);
        assert!(
            (a.flow_per_lane - 1397.0).abs() < 1.0,
            "v/N {}",
            a.flow_per_lane
        );

        // Per-lane demand of 1,397 is below the 1,400 breakpoint, so S_b is the adjusted FFS.
        assert!(
            (a.breakpoint_adj - 1400.0).abs() < 1e-9,
            "BP_adj {}",
            a.breakpoint_adj
        );
        assert!(
            (a.capacity_basic_adj - 2350.0).abs() < 1e-9,
            "C_b,adj {}",
            a.capacity_basic_adj
        );
        assert!((a.speed_basic - 65.0).abs() < 1e-9, "S_b {}", a.speed_basic);

        assert!(
            (a.weaving_intensity - 0.006336).abs() < 5e-6,
            "W {}",
            a.weaving_intensity
        );
        assert!(
            (a.speed_impedance - 5.68).abs() < 0.02,
            "SIW {}",
            a.speed_impedance
        );
        assert!((a.speed_avg - 59.32).abs() < 0.02, "S_o {}", a.speed_avg);

        let cw = a.capacity_per_lane.expect("capacity");
        assert!((cw - 1866.0).abs() < 2.0, "C_W {cw}");
        assert!(
            (a.dc_ratio.unwrap() - 0.75).abs() < 0.005,
            "d/c {:?}",
            a.dc_ratio
        );
        assert!(!a.demand_exceeds_capacity);

        assert!((a.density - 23.6).abs() < 0.1, "D {}", a.density);
        assert_eq!(a.los, LevelOfService::C);
    }

    /// The classifier reads the four configuration parameters, and only a segment where all four
    /// equal 1 is simple.
    #[test]
    fn simple_weave_requires_all_four_parameters_to_be_one() {
        assert_eq!(
            classify(WeavingType::OneSided, 1, 1, 1, 1),
            WeavingClass::Simple
        );
        for (lc_rf, lc_fr, nw_rf, nw_fr) in
            [(0, 1, 1, 1), (1, 0, 1, 1), (1, 1, 2, 1), (1, 1, 1, 2)]
        {
            assert_eq!(
                classify(WeavingType::OneSided, lc_rf, lc_fr, nw_rf, nw_fr),
                WeavingClass::Complex,
                "{lc_rf} {lc_fr} {nw_rf} {nw_fr}"
            );
        }
        assert_eq!(
            classify(WeavingType::TwoSided, 1, 1, 1, 1),
            WeavingClass::TwoSided
        );
    }

    /// Equation 13-11: substituting the simple-weave parameters into the general Equation 13-9
    /// must reproduce the manual's simplified form.
    #[test]
    fn simple_weave_reduces_to_equation_13_11() {
        let (v_rf, v_fr, n, ls) = (900.0, 700.0, 4.0, 1200.0);
        let flows = MovementFlows {
            v_rf,
            v_rr: 0.0,
            v_fr,
            v_ff: 0.0,
        };
        let general = weaving_intensity(
            configured_weaving_flow(WeavingType::OneSided, 1, 1, 1, 1, 1, 0, flows),
            n,
            ls,
            WeavingClass::Simple.coefficients(),
        );
        let simplified =
            0.016 * ((v_rf + v_fr) / n.powf(3.217)).powf(0.021) * (1.0 / ls).powf(0.181);
        assert!(
            (general - simplified).abs() < 1e-12,
            "{general} vs {simplified}"
        );
    }

    /// Equation 13-14: the two-sided simplified form for LC_RR = 2, NW_RR = 0 weights the
    /// ramp-to-ramp flow by three.
    #[test]
    fn two_sided_weave_reduces_to_equation_13_14() {
        let (v_rr, n, ls) = (600.0, 3.0, 1000.0);
        let flows = MovementFlows {
            v_rf: 0.0,
            v_rr,
            v_fr: 0.0,
            v_ff: 0.0,
        };
        let general = weaving_intensity(
            configured_weaving_flow(WeavingType::TwoSided, 0, 0, 2, 0, 0, 0, flows),
            n,
            ls,
            WeavingClass::TwoSided.coefficients(),
        );
        let simplified =
            0.016 * ((3.0 * v_rr) / n.powf(3.217)).powf(0.021) * (1.0 / ls).powf(0.181);
        assert!(
            (general - simplified).abs() < 1e-12,
            "{general} vs {simplified}"
        );
    }

    /// Below 500 pc/h/ln the impedance term is clamped to zero, so the weaving segment runs at
    /// basic-segment speed rather than faster than one.
    #[test]
    fn impedance_is_zero_at_low_flow() {
        assert_eq!(speed_impedance(0.05, 400.0), 0.0);
        assert_eq!(speed_impedance(0.05, IMPEDANCE_FLOW_THRESHOLD), 0.0);
        assert!(speed_impedance(0.05, 600.0) > 0.0);
    }

    /// Exhibit 13-7 thresholds, including the boundary values, which differ from the 7th
    /// Edition's Exhibit 13-6.
    #[test]
    fn los_follows_exhibit_13_7() {
        assert_eq!(determine_weaving_los(11.0, false), LevelOfService::A);
        assert_eq!(determine_weaving_los(11.1, false), LevelOfService::B);
        assert_eq!(determine_weaving_los(18.0, false), LevelOfService::B);
        assert_eq!(determine_weaving_los(25.0, false), LevelOfService::C);
        assert_eq!(determine_weaving_los(30.0, false), LevelOfService::D);
        assert_eq!(determine_weaving_los(35.0, false), LevelOfService::E);
        assert_eq!(determine_weaving_los(35.1, false), LevelOfService::F);
        // Over capacity is LOS F whatever the density says.
        assert_eq!(determine_weaving_los(12.0, true), LevelOfService::F);
    }

    /// The capacity quadratic reproduces the definition it was derived from: at C_W the segment
    /// sits exactly at the 35 pc/mi/ln breakdown density (Equation 13-15).
    #[test]
    fn capacity_lands_on_the_breakdown_density() {
        let (ffs, caf) = (65.0, 1.0);
        let c_b = basic_segment_capacity(ffs) * caf;
        let bp = basic_segment_breakpoint(ffs, caf);
        let w = 0.006336;
        let cw = weaving_capacity_per_lane(w, ffs, c_b, bp).unwrap();

        let s_b = basic_segment_speed(cw, ffs, c_b, bp, EXPONENT_BASIC_FREEWAY);
        let s_o = s_b - speed_impedance(w, cw);
        assert!(
            (cw / s_o - WEAVING_BREAKDOWN_DENSITY).abs() < 0.05,
            "density at capacity {}",
            cw / s_o
        );
    }

    /// Capacity never exceeds that of a basic freeway segment with the same lanes, which the
    /// manual states the procedure guarantees.
    #[test]
    fn capacity_stays_below_the_basic_segment_capacity() {
        for ffs in [55.0, 60.0, 65.0, 70.0, 75.0] {
            let c_b = basic_segment_capacity(ffs);
            let bp = basic_segment_breakpoint(ffs, 1.0);
            for w in [0.0001, 0.001, 0.006336, 0.05] {
                let cw = weaving_capacity_per_lane(w, ffs, c_b, bp).unwrap();
                assert!(cw <= c_b, "FFS {ffs}, W {w}: C_W {cw} > C_b {c_b}");
            }
        }
    }

    /// Equations 13-2 through 13-6 are self-consistent: the estimated movements re-sum to the
    /// on-ramp, off-ramp, and mainline flows they were derived from.
    #[test]
    fn estimated_movements_reproduce_their_inputs() {
        let (v_on, v_off, v_f) = (1200.0, 900.0, 4000.0);
        let v = v_f + v_on;
        let f = estimate_movement_flows(v_on, v_off, v_f, v);
        assert!((f.v_rf + f.v_rr - v_on).abs() < 1e-9, "on-ramp");
        assert!((f.v_fr + f.v_rr - v_off).abs() < 1e-9, "off-ramp");
        assert!((f.v_ff + f.v_fr - v_f).abs() < 1e-9, "mainline");
        assert!((f.v_ff + f.v_fr + f.v_rf + f.v_rr - v).abs() < 1e-9, "total");
    }
}
