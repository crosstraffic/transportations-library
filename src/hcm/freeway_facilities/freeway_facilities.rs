//! HCM Chapter 10: Freeway Facilities Core Methodology (motorized vehicle).
//!
//! Orchestrates the Chapter 12 (basic), Chapter 13 (weaving), and Chapter 14
//! (merge/diverge) segment engines over an ordered set of segments and
//! consecutive 15-min analysis periods (the time–space domain of Exhibit
//! 10-10), covering computational steps A-1 through A-17 of Exhibit 10-8:
//!
//! - segmentation rules (Exhibits 10-1/10-2/10-11/10-12; ramp influence
//!   areas of 1,500 ft, overlapping ramp segments, weaving boundaries);
//! - demand balancing (Equations 10-2/10-3) and demand flow accumulation;
//! - segment capacities (Step A-7) with CAF/SAF/DAF hooks (Equations
//!   10-4/10-5/10-6) and work zone adjustments (Equations 10-7 to 10-12);
//! - undersaturated evaluation (Step A-11; Chapter 25 Section 3, including
//!   the Equation 25-1 maximum-achievable-speed constraint);
//! - oversaturated evaluation (Step A-12; the Chapter 25 Section 4
//!   node–segment time-step engine in [`super::oversaturated`]);
//! - facility performance aggregation and LOS (Steps A-15/A-17; Equations
//!   10-1, 25-2 through 25-5; Exhibit 10-6).
//!
//! Managed-lane facilities (Steps A-9/A-13/A-14) and the Chapter 25
//! planning-level method are not implemented in this pass.
//!
//! Sources (HCM 7th Edition EPUB): `66_Ch10.xhtml` … `71_Ch10_05.xhtml`,
//! `194_Ch25_03.xhtml`, `195_Ch25_04.xhtml`, `202_Ch25_11.xhtml`.

use serde::{Deserialize, Serialize};

use crate::hcm::basicfreeways::basicfreeways::{
    basic_segment_breakpoint, basic_segment_capacity, basic_segment_speed, DENSITY_AT_CAPACITY,
    EXPONENT_BASIC_FREEWAY,
};
use crate::hcm::weaving::weaving::{
    FacilityType as WeaveFacility, TerrainType as WeaveTerrain, WeavingSegment,
    WeavingType,
};
use crate::hcm::merge_diverge::merge_diverge::{
    get_freeway_capacity_per_lane, get_ramp_capacity, AdjacentRampType, RampLanes,
    RampSegment, RampSide, RampType, TerrainType as RampTerrain,
};
use crate::hcm::common::los_tables::{
    los_basic_freeway, los_merge_diverge, los_weaving, WeavingFacilityType,
};
use crate::hcm::common::{CityType, LevelOfService};

use super::exhibits::{
    self, WorkZone, DEFAULT_JAM_DENSITY_PC, DEFAULT_QUEUE_DISCHARGE_DROP,
    DEFAULT_TIME_STEP_S,
};
use super::oversaturated::{OversatPeriodInput, OversaturatedEngine};

/// HCM chapter implemented by this module.
pub const CHAPTER: u32 = 10;

/// Ramp influence area length, ft (Chapter 10 segmentation; Exhibit 10-1:
/// 1,500 ft downstream of an on-ramp gore / upstream of an off-ramp gore).
pub const RAMP_INFLUENCE_AREA_FT: f64 = 1500.0;

// ═════════════════════════════════════════════════════════════════════════
// Segment data model
// ═════════════════════════════════════════════════════════════════════════

/// HCM analysis segment types (Chapter 10, Step A-2).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SegmentType {
    /// Basic freeway segment (Chapter 12).
    Basic,
    /// On-ramp (merge) segment (Chapter 14); the ramp joins at the
    /// segment's upstream gore and the influence area extends 1,500 ft
    /// downstream.
    Merge,
    /// Off-ramp (diverge) segment (Chapter 14); the ramp leaves at the
    /// segment's downstream gore and the influence area extends 1,500 ft
    /// upstream.
    Diverge,
    /// Weaving segment (Chapter 13); on-ramp at the upstream gore, off-ramp
    /// at the downstream gore, connected by an auxiliary lane.
    Weaving,
    /// Overlapping ramp segment ("R" in Exhibit 25-44): merge and diverge
    /// influence areas overlap (gore-to-gore spacing between 1,500 and
    /// 3,000 ft). Operations take the worse of the two ramp analyses
    /// (Chapter 10, Step A-2 discussion of Exhibit 10-11(c)).
    OverlappingRamp,
}

impl Default for SegmentType {
    fn default() -> Self {
        SegmentType::Basic
    }
}

/// Terrain classification for PCE selection (Exhibit 12-25).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Terrain {
    Level,
    Rolling,
    Mountainous,
}

impl Default for Terrain {
    fn default() -> Self {
        Terrain::Level
    }
}

impl Terrain {
    /// Passenger-car equivalent E_T (Exhibit 12-25: level 2.0, rolling 3.0).
    /// VERIFY-HCM: Exhibit 12-25 provides no PCE for mountainous terrain
    /// (HCM directs to the Chapter 25/26 mixed-flow model); 3.0 is used as
    /// a conservative stand-in, consistent with the other chapter modules.
    pub fn pce(self) -> f64 {
        match self {
            Terrain::Level => 2.0,
            Terrain::Rolling => 3.0,
            Terrain::Mountainous => 3.0,
        }
    }

    fn to_weave(self) -> WeaveTerrain {
        match self {
            Terrain::Level => WeaveTerrain::Level,
            Terrain::Rolling => WeaveTerrain::Rolling,
            Terrain::Mountainous => WeaveTerrain::Mountainous,
        }
    }

    fn to_ramp(self) -> RampTerrain {
        match self {
            Terrain::Level => RampTerrain::Level,
            Terrain::Rolling => RampTerrain::Rolling,
            Terrain::Mountainous => RampTerrain::Mountainous,
        }
    }
}

/// One HCM analysis segment of a freeway facility. Demand-side vectors are
/// indexed by analysis period.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct FacilitySegment {
    /// Segment type (Step A-2).
    pub seg_type: SegmentType,
    /// Segment length, ft.
    pub length_ft: f64,
    /// Number of mainline lanes.
    pub lanes: u32,
    /// Segment free-flow speed override, mi/h (facility FFS if `None`).
    pub ffs: Option<f64>,
    /// Calibration capacity adjustment factor CAF_cal (Equation 10-5).
    pub caf: f64,
    /// Calibration speed adjustment factor SAF_cal (Equation 10-4).
    pub saf: f64,
    /// Calibration demand adjustment factor DAF_cal (Equation 10-6),
    /// applied to the segment's own ramp demands.
    pub daf: f64,
    /// Optional per-period CAF schedule (overrides `caf` where present).
    pub caf_schedule: Option<Vec<f64>>,
    /// Optional per-period SAF schedule (overrides `saf` where present).
    pub saf_schedule: Option<Vec<f64>>,
    /// Work zone active on this segment (Chapter 10 Section 4); derives
    /// CAF_wz/SAF_wz via Equations 10-7 through 10-12 and multiplies the
    /// calibration factors.
    pub work_zone: Option<WorkZone>,

    // ── Ramp attributes (Merge / Diverge / Weaving) ──────────────────────
    /// On-ramp demand by period, veh/h (Merge and Weaving segments).
    pub on_ramp_demand: Vec<f64>,
    /// Off-ramp demand by period, veh/h (Diverge and Weaving segments).
    pub off_ramp_demand: Vec<f64>,
    /// Ramp-to-ramp demand by period, veh/h (Weaving segments; the
    /// component splits follow Chapter 10 Step A-3 guidance).
    pub ramp_to_ramp_demand: Vec<f64>,
    /// Ramp free-flow speed, mi/h.
    pub ramp_ffs: f64,
    /// Acceleration lane length, ft (on-ramps).
    pub accel_lane_ft: f64,
    /// Deceleration lane length, ft (off-ramps).
    pub decel_lane_ft: f64,
    /// Ramp-metering rate by period, veh/h (Step A-7: overrides the
    /// on-ramp capacity to evaluate a predetermined metering plan).
    pub ramp_metering: Option<Vec<f64>>,

    // ── Weaving attributes (Chapter 13) ──────────────────────────────────
    /// Weaving short length L_S, ft (defaults to the segment length).
    pub short_length_ft: Option<f64>,
    /// Number of weaving lanes N_WL (2 or 3 for one-sided weaves).
    pub num_weaving_lanes: u32,
    /// Minimum lane changes for ramp-to-freeway vehicles LC_RF.
    pub lc_rf: u32,
    /// Minimum lane changes for freeway-to-ramp vehicles LC_FR.
    pub lc_fr: u32,
}

impl Default for FacilitySegment {
    fn default() -> Self {
        Self {
            seg_type: SegmentType::Basic,
            length_ft: 1500.0,
            lanes: 3,
            ffs: None,
            caf: 1.0,
            saf: 1.0,
            daf: 1.0,
            caf_schedule: None,
            saf_schedule: None,
            work_zone: None,
            on_ramp_demand: Vec::new(),
            off_ramp_demand: Vec::new(),
            ramp_to_ramp_demand: Vec::new(),
            ramp_ffs: 40.0,
            accel_lane_ft: 500.0,
            decel_lane_ft: 500.0,
            ramp_metering: None,
            short_length_ft: None,
            num_weaving_lanes: 2,
            lc_rf: 1,
            lc_fr: 1,
        }
    }
}

impl FacilitySegment {
    /// Segment length, mi.
    pub fn length_mi(&self) -> f64 {
        self.length_ft / 5280.0
    }

    fn on_demand(&self, p: usize) -> f64 {
        self.on_ramp_demand.get(p).copied().unwrap_or(0.0) * self.daf
    }

    fn off_demand(&self, p: usize) -> f64 {
        self.off_ramp_demand.get(p).copied().unwrap_or(0.0) * self.daf
    }

    fn rr_demand(&self, p: usize) -> f64 {
        self.ramp_to_ramp_demand.get(p).copied().unwrap_or(0.0) * self.daf
    }
}

// ═════════════════════════════════════════════════════════════════════════
// Facility-level performance
// ═════════════════════════════════════════════════════════════════════════

/// Facility-wide performance measures for one 15-min analysis period
/// (Chapter 10, Steps A-15/A-17).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeriodPerformance {
    /// Facility space mean speed, mi/h (Equation 25-2).
    pub space_mean_speed: f64,
    /// Average facility density, veh/mi/ln (Equation 10-1 with
    /// vehicle-based segment densities, as reported in Exhibit 25-52).
    pub avg_density_veh: f64,
    /// Average facility density, pc/mi/ln (Equation 10-1; basis for the
    /// Exhibit 10-6 LOS lookup).
    pub avg_density_pc: f64,
    /// Facility LOS (Exhibit 10-6; F if any segment vd/c > 1.00).
    pub los: LevelOfService,
    /// Vehicle miles traveled by served volumes, veh-mi.
    pub vmt_served: f64,
    /// Vehicle miles traveled by demand, veh-mi.
    pub vmt_demand: f64,
    /// Vehicle hours of travel (mainline), veh-h.
    pub vht: f64,
    /// Vehicle hours of delay relative to FFS travel (mainline), veh-h.
    pub vhd: f64,
}

// ═════════════════════════════════════════════════════════════════════════
// FreewayFacility
// ═════════════════════════════════════════════════════════════════════════

/// A directional freeway facility analyzed with the HCM Chapter 10 core
/// methodology over consecutive 15-min analysis periods.
///
/// Input fields are plain values; computed fields are populated by
/// [`FreewayFacility::run_analysis`] (or the individual step methods) as
/// `[segment][period]` matrices.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct FreewayFacility {
    // ── Inputs ───────────────────────────────────────────────────────────
    /// Ordered segments, upstream to downstream (Step A-2). The first and
    /// last segments must be basic segments (Chapter 10 guidance).
    pub segments: Vec<FacilitySegment>,
    /// Mainline demand entering the facility by period, veh/h (15-min flow
    /// rates; PHF = 1.0 when true 15-min demands are supplied).
    pub mainline_demand: Vec<f64>,
    /// Facility free-flow speed, mi/h (segment overrides allowed).
    pub ffs: f64,
    /// Heavy vehicle proportion (SUT + TT), decimal.
    pub heavy_vehicle_pct: f64,
    /// Terrain classification (Exhibit 12-25 PCEs).
    pub terrain: Terrain,
    /// Urban or rural facility (Exhibit 10-6 LOS thresholds).
    pub city_type: CityType,
    /// Peak hour factor (1.0 for 15-min demand flow rates).
    pub phf: f64,
    /// Jam density, pc/mi/ln (Step A-6 global parameter; default 190).
    pub jam_density_pc: f64,
    /// Queue discharge capacity drop alpha, decimal (Step A-6; default 7%).
    pub queue_discharge_drop: f64,
    /// Total ramp density, ramps/mi (facility-wide; Chapter 10 Concepts).
    pub total_ramp_density: f64,
    /// Interchange density for the weaving engine, int/mi (defaults to
    /// `total_ramp_density`).
    pub interchange_density: Option<f64>,
    /// Base per-lane capacity override c_IFL, pc/h/ln (otherwise Equation
    /// 12-6 at the adjusted FFS).
    pub c_ifl_override: Option<f64>,
    /// Oversaturated-engine time step, s (Chapter 25 default 15 s).
    pub time_step_s: f64,

    // ── Computed ([segment][period] unless noted) ────────────────────────
    /// Segment demand SD(i, p), veh/h.
    pub demand: Vec<Vec<f64>>,
    /// Segment capacity, veh/h (Step A-7, adjusted per Step A-8).
    pub capacity: Vec<Vec<f64>>,
    /// Demand-to-capacity ratios vd/c (Step A-10).
    pub dc_ratio: Vec<Vec<f64>>,
    /// Volume served va, veh/h.
    pub volume_served: Vec<Vec<f64>>,
    /// Volume-to-capacity ratios va/c.
    pub vc_ratio: Vec<Vec<f64>>,
    /// Segment space mean speed, mi/h.
    pub speed: Vec<Vec<f64>>,
    /// Segment density, veh/mi/ln.
    pub density_veh: Vec<Vec<f64>>,
    /// Segment density, pc/mi/ln.
    pub density_pc: Vec<Vec<f64>>,
    /// Density-based segment LOS (Exhibit 25-59 upper table).
    pub los: Vec<Vec<LevelOfService>>,
    /// Demand-based segment LOS: `Some(F)` where vd/c > 1.0 (Exhibit 25-59
    /// lower table), `None` otherwise.
    pub demand_based_los: Vec<Vec<Option<LevelOfService>>>,
    /// Mainline queue length at the end of each period, ft (Equation 25-34).
    pub queue_length_ft: Vec<Vec<f64>>,
    /// Whether the segment carried a queue during the period.
    pub had_queue: Vec<Vec<bool>>,
    /// On-ramp queue at the end of each period, veh (`[segment][period]`,
    /// attributed to the segment whose upstream node hosts the ramp).
    pub on_ramp_queue: Vec<Vec<f64>>,
    /// Unserved vehicles held upstream of the facility entrance at the end
    /// of each period, veh.
    pub unserved_entry_veh: Vec<f64>,
    /// Facility-wide performance by period (Steps A-15/A-17).
    pub facility_performance: Vec<PeriodPerformance>,
    /// Whether any cell of the time–space domain had vd/c > 1.0.
    pub oversaturated: bool,
    /// First analysis period (0-based) in which oversaturation occurs.
    pub first_oversat_period: Option<usize>,
}

impl Default for FreewayFacility {
    fn default() -> Self {
        Self {
            segments: Vec::new(),
            mainline_demand: Vec::new(),
            ffs: 60.0,
            heavy_vehicle_pct: 0.05,
            terrain: Terrain::Level,
            city_type: CityType::Urban,
            phf: 1.0,
            jam_density_pc: DEFAULT_JAM_DENSITY_PC,
            queue_discharge_drop: DEFAULT_QUEUE_DISCHARGE_DROP,
            total_ramp_density: 1.0,
            interchange_density: None,
            c_ifl_override: None,
            time_step_s: DEFAULT_TIME_STEP_S,
            demand: Vec::new(),
            capacity: Vec::new(),
            dc_ratio: Vec::new(),
            volume_served: Vec::new(),
            vc_ratio: Vec::new(),
            speed: Vec::new(),
            density_veh: Vec::new(),
            density_pc: Vec::new(),
            los: Vec::new(),
            demand_based_los: Vec::new(),
            queue_length_ft: Vec::new(),
            had_queue: Vec::new(),
            on_ramp_queue: Vec::new(),
            unserved_entry_veh: Vec::new(),
            facility_performance: Vec::new(),
            oversaturated: false,
            first_oversat_period: None,
        }
    }
}

/// Result of a single-segment engine evaluation at a given served volume.
struct EngineEval {
    /// Space mean speed across all segment lanes, mi/h.
    speed: f64,
    /// Ramp influence area density D_R, pc/mi/ln (merge/diverge only).
    influence_density_pc: Option<f64>,
}

impl FreewayFacility {
    pub fn new() -> Self {
        Self::default()
    }

    // ── Accessors ────────────────────────────────────────────────────────

    pub fn num_segments(&self) -> usize {
        self.segments.len()
    }

    pub fn num_periods(&self) -> usize {
        self.mainline_demand.len()
    }

    /// Total facility length, mi.
    pub fn total_length_mi(&self) -> f64 {
        self.segments.iter().map(|s| s.length_mi()).sum()
    }

    /// Heavy vehicle adjustment factor f_HV (Equation 12-10) with Exhibit
    /// 12-25 PCEs.
    pub fn f_hv(&self) -> f64 {
        1.0 / (1.0 + self.heavy_vehicle_pct * (self.terrain.pce() - 1.0))
    }

    fn seg_ffs(&self, i: usize) -> f64 {
        self.segments[i].ffs.unwrap_or(self.ffs)
    }

    /// Effective SAF for segment `i` in period `p`: calibration SAF
    /// (schedule-aware) times the work zone SAF_wz (Equation 10-12).
    fn effective_saf(&self, i: usize, p: usize) -> f64 {
        let seg = &self.segments[i];
        let cal = seg
            .saf_schedule
            .as_ref()
            .and_then(|v| v.get(p).copied())
            .unwrap_or(seg.saf);
        match &seg.work_zone {
            Some(wz) => cal * wz.saf(self.seg_ffs(i)),
            None => cal,
        }
    }

    /// Effective CAF for segment `i` in period `p`: calibration CAF
    /// (schedule-aware) times the work zone CAF_wz (Equation 10-11).
    fn effective_caf(&self, i: usize, p: usize) -> f64 {
        let seg = &self.segments[i];
        let cal = seg
            .caf_schedule
            .as_ref()
            .and_then(|v| v.get(p).copied())
            .unwrap_or(seg.caf);
        match &seg.work_zone {
            Some(wz) => {
                let c_nonwz = self.base_capacity_pc(self.seg_ffs(i));
                cal * wz.caf(c_nonwz)
            }
            None => cal,
        }
    }

    /// Base per-lane capacity, pc/h/ln (Equation 12-6, capped at 2,400;
    /// `c_ifl_override` wins when provided).
    ///
    /// Takes the **unadjusted** segment FFS. The December 2022 corrections changed Equation 12-6
    /// from FFS_adj to FFS: SAF reaches capacity only through CAF, never through the speed the
    /// capacity is computed from. This matters here more than anywhere else in the library,
    /// because the scenario engine is where SAF actually varies.
    fn base_capacity_pc(&self, ffs: f64) -> f64 {
        match self.c_ifl_override {
            Some(c) => c,
            None => basic_segment_capacity(ffs),
        }
    }

    // ── Node-indexed ramp demand helpers ─────────────────────────────────
    // On-ramps enter at a segment's upstream node (node i for segment i);
    // off-ramps exit at a segment's downstream node (node i + 1), matching
    // Exhibit 25-4.

    fn onrd_by_node(&self, p: usize) -> Vec<f64> {
        let n = self.num_segments();
        let mut onrd = vec![0.0; n + 1];
        for (i, seg) in self.segments.iter().enumerate() {
            match seg.seg_type {
                SegmentType::Merge | SegmentType::Weaving => onrd[i] += seg.on_demand(p),
                _ => {}
            }
        }
        onrd
    }

    fn offrd_by_node(&self, p: usize) -> Vec<f64> {
        let n = self.num_segments();
        let mut offrd = vec![0.0; n + 1];
        for (i, seg) in self.segments.iter().enumerate() {
            match seg.seg_type {
                SegmentType::Diverge | SegmentType::Weaving => {
                    offrd[i + 1] += seg.off_demand(p)
                }
                _ => {}
            }
        }
        offrd
    }

    // ── Step A-3/A-4: demands ────────────────────────────────────────────

    /// Compute the segment demand matrix SD(i, p) by accumulating ramp
    /// demands along the facility (Step A-4; demands are assumed balanced —
    /// use [`exhibits::balance_exit_demands`] for Equation 10-2/10-3
    /// balancing of exit counts beforehand).
    pub fn compute_demands(&mut self) {
        let n = self.num_segments();
        let p_count = self.num_periods();
        self.demand = vec![vec![0.0; p_count]; n];
        for p in 0..p_count {
            let onrd = self.onrd_by_node(p);
            let offrd = self.offrd_by_node(p);
            let mut upstream = self.mainline_demand[p];
            for i in 0..n {
                let sd = upstream + onrd[i] - offrd[i];
                self.demand[i][p] = sd;
                upstream = sd;
            }
        }
    }

    // ── Step A-7/A-8: capacities ─────────────────────────────────────────

    /// Segment capacities in veh/h under prevailing conditions
    /// (Step A-7), including CAF/SAF adjustments (Step A-8) and work zone
    /// effects. Weaving capacities vary by period with the demand pattern
    /// (Chapter 13); other types are constant unless scheduled factors or
    /// work zones apply.
    pub fn compute_capacities(&mut self) {
        let n = self.num_segments();
        let p_count = self.num_periods();
        let f_hv = self.f_hv();
        self.capacity = vec![vec![0.0; p_count]; n];
        for i in 0..n {
            for p in 0..p_count {
                // Capacity reads the unadjusted FFS throughout (Equations 12-6/12-7 as corrected
                // December 2022); SAF reaches capacity only through CAF, and the weaving engine
                // applies its own SAF internally.
                let ffs = self.seg_ffs(i);
                let caf = self.effective_caf(i, p);
                let lanes = f64::from(self.segments[i].lanes);
                let cap = match self.segments[i].seg_type {
                    SegmentType::Basic | SegmentType::OverlappingRamp => {
                        self.base_capacity_pc(ffs) * caf * lanes * f_hv
                    }
                    SegmentType::Merge | SegmentType::Diverge => {
                        // Exhibit 14-10 freeway capacity per lane, tabulated from Equation 12-6
                        // and so read at the unadjusted FFS on the same reasoning.
                        get_freeway_capacity_per_lane(ffs) * caf * lanes * f_hv
                    }
                    SegmentType::Weaving => {
                        let mut weave = self.build_weave(i, p, self.demand[i][p]);
                        weave.determine_demand_flow();
                        weave.determine_configuration_characteristics();
                        weave.determine_max_weaving_length();
                        if weave.is_weaving_segment() {
                            weave.determine_capacity()
                        } else {
                            // L_S >= L_MAX: operates as a basic segment
                            // (Exhibit 10-12(b)).
                            self.base_capacity_pc(ffs) * caf * lanes * f_hv
                        }
                    }
                };
                self.capacity[i][p] = cap;
            }
        }
    }

    // ── Step A-10: demand-to-capacity ratios ─────────────────────────────

    /// Compute vd/c for every cell of the time–space domain and flag
    /// oversaturation (Step A-10).
    pub fn compute_dc_ratios(&mut self) {
        let n = self.num_segments();
        let p_count = self.num_periods();
        self.dc_ratio = vec![vec![0.0; p_count]; n];
        self.first_oversat_period = None;
        for p in 0..p_count {
            for i in 0..n {
                let dc = if self.capacity[i][p] > 0.0 {
                    self.demand[i][p] / self.capacity[i][p]
                } else {
                    f64::INFINITY
                };
                self.dc_ratio[i][p] = dc;
                if dc > 1.0 && self.first_oversat_period.is_none() {
                    self.first_oversat_period = Some(p);
                }
            }
        }
        self.oversaturated = self.first_oversat_period.is_some();
    }

    // ── Segment engine builders ──────────────────────────────────────────

    /// Build a Chapter 13 weaving engine for segment `i`, period `p`, with
    /// total segment volume `volume` (veh/h). Component flows follow the
    /// Chapter 10 Step A-3 split with the given ramp-to-ramp demand.
    fn build_weave(&self, i: usize, p: usize, volume: f64) -> WeavingSegment {
        let seg = &self.segments[i];
        let scale = if self.demand[i][p] > 0.0 {
            volume / self.demand[i][p]
        } else {
            1.0
        };
        let v_rr = seg.rr_demand(p) * scale;
        let v_rf = (seg.on_demand(p) * scale - v_rr).max(0.0);
        let v_fr = (seg.off_demand(p) * scale - v_rr).max(0.0);
        let v_ff = (volume - v_rf - v_rr - v_fr).max(0.0);
        let ffs_unadjusted = self.seg_ffs(i); // SAF applied inside the engine
        WeavingSegment {
            weaving_type: WeavingType::OneSided,
            facility_type: WeaveFacility::Freeway,
            length_short: seg.short_length_ft.unwrap_or(seg.length_ft),
            num_lanes: seg.lanes,
            num_weaving_lanes: seg.num_weaving_lanes,
            ffs: ffs_unadjusted,
            v_ff,
            v_fr,
            v_rf,
            v_rr,
            phf: self.phf,
            heavy_vehicle_pct: self.heavy_vehicle_pct,
            terrain: self.terrain.to_weave(),
            lc_rf: seg.lc_rf,
            lc_fr: seg.lc_fr,
            lc_rr: 0,
            interchange_density: self.interchange_density.unwrap_or(self.total_ramp_density),
            basic_freeway_capacity: self.base_capacity_pc(self.seg_ffs(i)),
            caf: self.effective_caf(i, p),
            saf: self.effective_saf(i, p),
            ..Default::default()
        }
    }

    /// Build a Chapter 14 ramp engine for segment `i`, period `p`.
    /// `mainline` approaches the junction and `ramp` uses it, both veh/h.
    fn build_ramp(&self, i: usize, p: usize, mainline: f64, ramp: f64) -> RampSegment {
        let seg = &self.segments[i];
        let is_merge = seg.seg_type == SegmentType::Merge;
        RampSegment {
            ramp_type: if is_merge {
                RampType::OnRamp
            } else {
                RampType::OffRamp
            },
            ramp_side: RampSide::Right,
            ramp_lanes: RampLanes::OneLane,
            freeway_lanes: seg.lanes,
            freeway_ffs: self.seg_ffs(i),
            ramp_ffs: seg.ramp_ffs,
            accel_lane_length: Some(seg.accel_lane_ft),
            decel_lane_length: Some(seg.decel_lane_ft),
            freeway_demand: mainline,
            ramp_demand: ramp,
            phf: self.phf,
            heavy_vehicle_pct: self.heavy_vehicle_pct,
            ramp_heavy_vehicle_pct: None,
            terrain: self.terrain.to_ramp(),
            adjacent_upstream: AdjacentRampType::None,
            adjacent_downstream: AdjacentRampType::None,
            caf: self.effective_caf(i, p),
            saf: self.effective_saf(i, p),
            ..Default::default()
        }
    }

    /// Space mean speed of a basic freeway segment at per-lane flow `v_p`
    /// (pc/h/ln) — Equation 12-1 with the Exhibit 12-6 breakpoint and
    /// capacity models.
    /// `ffs` is the unadjusted segment free-flow speed and `ffs_adj` the same speed times SAF.
    /// Capacity is computed from the former and the breakpoint and speed curve from the latter;
    /// see [`base_capacity_pc`](Self::base_capacity_pc).
    fn basic_speed(&self, v_p: f64, ffs: f64, ffs_adj: f64, caf: f64) -> f64 {
        let c_adj = self.base_capacity_pc(ffs) * caf;
        let bp = basic_segment_breakpoint(ffs_adj, caf);
        if v_p <= c_adj {
            basic_segment_speed(v_p, ffs_adj, c_adj, bp, EXPONENT_BASIC_FREEWAY)
        } else {
            // Above capacity this engine clamps to the speed at capacity; the oversaturated
            // procedure, not the Equation 12-1 curve, owns that regime.
            c_adj / DENSITY_AT_CAPACITY
        }
    }

    /// Evaluate segment `i` in period `p` with the appropriate Chapter
    /// 12/13/14 engine at served volume `volume` (veh/h) with ramp flows
    /// `onr`/`offr` (veh/h).
    fn engine_eval(&self, i: usize, p: usize, volume: f64, onr: f64, offr: f64) -> EngineEval {
        let seg = &self.segments[i];
        let f_hv = self.f_hv();
        let ffs = self.seg_ffs(i);
        let ffs_adj = ffs * self.effective_saf(i, p);
        let caf = self.effective_caf(i, p);
        let basic = |v: f64| {
            let v_p = v / (f64::from(seg.lanes) * f_hv * self.phf);
            self.basic_speed(v_p, ffs, ffs_adj, caf)
        };
        match seg.seg_type {
            SegmentType::Basic | SegmentType::OverlappingRamp => EngineEval {
                speed: basic(volume),
                influence_density_pc: None,
            },
            SegmentType::Merge => {
                let mut ramp = self.build_ramp(i, p, (volume - onr).max(0.0), onr);
                ramp.run_analysis();
                // VERIFY-HCM: in the facility context, ramp segment speeds
                // at high flows are additionally bounded by the Chapter 12
                // basic speed–flow curve at the same volume; this cap
                // reproduces the published Chapter 25 Example Problem 1/2
                // speed matrices (e.g., Exhibit 25-49, Segment 10, Analysis
                // Period 3: 51.8 mi/h = the Equation 12-1 value).
                EngineEval {
                    speed: ramp.get_speed_avg().min(basic(volume)),
                    influence_density_pc: ramp.density,
                }
            }
            SegmentType::Diverge => {
                let mut ramp = self.build_ramp(i, p, volume, offr);
                ramp.run_analysis();
                // VERIFY-HCM: same basic speed–flow cap as merge (above).
                EngineEval {
                    speed: ramp.get_speed_avg().min(basic(volume)),
                    influence_density_pc: ramp.density,
                }
            }
            SegmentType::Weaving => {
                let mut weave = self.build_weave(i, p, volume);
                weave.run_analysis();
                if weave.is_weaving_segment() {
                    EngineEval {
                        speed: weave.get_speed_avg(),
                        influence_density_pc: None,
                    }
                } else {
                    EngineEval {
                        speed: basic(volume),
                        influence_density_pc: None,
                    }
                }
            }
        }
    }

    /// Density-based LOS for segment `i` from its per-lane pc density and
    /// (for merge/diverge) the ramp influence area density.
    ///
    /// VERIFY-HCM: densities are rounded to the nearest integer before the
    /// threshold lookup; the Chapter 25 Example Problem 1/2 LOS matrices
    /// (Exhibits 25-51/25-59) are only consistent with integer-rounded
    /// densities (e.g., Segment 8, Period 4 of Example Problem 1: computed
    /// D_R = 28.2 pc/mi/ln, published LOS C at the <=28 boundary).
    fn density_los(
        &self,
        i: usize,
        pc_density: f64,
        influence_density_pc: Option<f64>,
        queued: bool,
    ) -> LevelOfService {
        let pc_density = pc_density.round();
        let influence_density_pc = influence_density_pc.map(f64::round);
        match self.segments[i].seg_type {
            SegmentType::Merge | SegmentType::Diverge => {
                if queued || influence_density_pc.is_none() {
                    // VERIFY-HCM: Exhibit 14-3 defines no density-based LOS F
                    // for ramp segments; queued ramp segments here use the
                    // Exhibit 12-15 basic thresholds on segment density,
                    // consistent with the Chapter 10 statement that queued
                    // segments are identified at densities > 45 pc/mi/ln.
                    los_basic_freeway(pc_density, false)
                } else {
                    los_merge_diverge(influence_density_pc.unwrap(), false)
                }
            }
            SegmentType::Weaving => {
                los_weaving(pc_density, false, WeavingFacilityType::Freeway)
            }
            _ => los_basic_freeway(pc_density, false),
        }
    }

    // ── Steps A-11/A-12: period evaluation ───────────────────────────────

    fn alloc_results(&mut self) {
        let n = self.num_segments();
        let p = self.num_periods();
        self.volume_served = vec![vec![0.0; p]; n];
        self.vc_ratio = vec![vec![0.0; p]; n];
        self.speed = vec![vec![0.0; p]; n];
        self.density_veh = vec![vec![0.0; p]; n];
        self.density_pc = vec![vec![0.0; p]; n];
        self.los = vec![vec![LevelOfService::A; p]; n];
        self.demand_based_los = vec![vec![None; p]; n];
        self.queue_length_ft = vec![vec![0.0; p]; n];
        self.had_queue = vec![vec![false; p]; n];
        self.on_ramp_queue = vec![vec![0.0; p]; n];
        self.unserved_entry_veh = vec![0.0; p];
    }

    /// Evaluate one period's segment chain. `served`, `onr`, `offr` are the
    /// served volumes (veh/h; node-indexed for ramps); `queued[i]` marks
    /// segments whose speed/density come from the oversaturated engine
    /// (`queued_speed`/`queued_density_veh`).
    #[allow(clippy::too_many_arguments)]
    fn evaluate_period_chain(
        &mut self,
        p: usize,
        served: &[f64],
        onr: &[f64],
        offr: &[f64],
        queued: &[bool],
        queued_speed: &[f64],
        queued_density_veh: &[f64],
    ) {
        let n = self.num_segments();
        let f_hv = self.f_hv();
        let mut prev_speed: Option<(f64, f64)> = None; // (speed, midpoint ft)
        let mut midpoint_ft = 0.0;

        for i in 0..n {
            let seg_mid = midpoint_ft + self.segments[i].length_ft / 2.0;
            let lanes = f64::from(self.segments[i].lanes);
            let (speed, influence) = if queued[i] {
                (queued_speed[i], None)
            } else {
                let eval = self.engine_eval(i, p, served[i], onr[i], offr[i
                    + 1]);
                // Equation 25-1: maximum achievable speed given the
                // upstream segment's speed.
                let capped = match prev_speed {
                    Some((v_prev, prev_mid)) => {
                        let dist = seg_mid - prev_mid;
                        let vmax = exhibits::max_achievable_speed(
                            self.seg_ffs(i) * self.effective_saf(i, p),
                            v_prev,
                            dist,
                        );
                        eval.speed.min(vmax)
                    }
                    None => eval.speed,
                };
                (capped, eval.influence_density_pc)
            };

            // Overlapping ramp segment: adopt the worse (slower) of the
            // adjacent merge (upstream, already final) and diverge
            // (downstream, raw) analyses — Chapter 10, Exhibit 10-11(c).
            let speed = if self.segments[i].seg_type == SegmentType::OverlappingRamp
                && !queued[i]
            {
                let mut worst = speed;
                if let Some((v_prev, _)) = prev_speed {
                    worst = worst.min(v_prev);
                }
                if i + 1 < n && self.segments[i + 1].seg_type == SegmentType::Diverge {
                    let next =
                        self.engine_eval(i + 1, p, served[i + 1], onr[i + 1], offr[i + 2]);
                    worst = worst.min(next.speed);
                }
                worst
            } else {
                speed
            };

            let density_veh = if queued[i] {
                queued_density_veh[i]
            } else if speed > 0.0 {
                served[i] / lanes / speed
            } else {
                0.0
            };
            let density_pc = density_veh / f_hv;

            self.volume_served[i][p] = served[i];
            self.vc_ratio[i][p] = if self.capacity[i][p] > 0.0 {
                served[i] / self.capacity[i][p]
            } else {
                0.0
            };
            self.speed[i][p] = speed;
            self.density_veh[i][p] = density_veh;
            self.density_pc[i][p] = density_pc;
            self.los[i][p] = self.density_los(i, density_pc, influence, queued[i]);
            self.demand_based_los[i][p] = if self.dc_ratio[i][p] > 1.0 {
                Some(LevelOfService::F)
            } else {
                None
            };

            prev_speed = Some((speed, seg_mid));
            midpoint_ft += self.segments[i].length_ft;
        }
    }

    /// Step A-11: undersaturated evaluation of period `p` (volume served
    /// equals demand; Chapter 25 Section 3).
    fn analyze_undersaturated_period(&mut self, p: usize) {
        let n = self.num_segments();
        let served: Vec<f64> = (0..n).map(|i| self.demand[i][p]).collect();
        let onr = self.onrd_by_node(p);
        let offr = self.offrd_by_node(p);
        let queued = vec![false; n];
        let zeros = vec![0.0; n];
        self.evaluate_period_chain(p, &served, &onr, &offr, &queued, &zeros, &zeros);
    }

    /// Step A-12: oversaturated evaluation from period `first` to the end
    /// of the study period (Chapter 25 Section 4 time-step engine).
    fn analyze_oversaturated(&mut self, first: usize) {
        let n = self.num_segments();
        let p_count = self.num_periods();
        let f_hv = self.f_hv();
        let lanes: Vec<f64> = self.segments.iter().map(|s| f64::from(s.lanes)).collect();
        let lengths: Vec<f64> = self.segments.iter().map(|s| s.length_mi()).collect();

        let mut engine = OversaturatedEngine::new(
            lengths.clone(),
            lanes.clone(),
            f_hv,
            self.jam_density_pc,
            self.queue_discharge_drop,
            self.time_step_s,
        );

        for p in first..p_count {
            let onrd = self.onrd_by_node(p);
            let offrd = self.offrd_by_node(p);
            let capacity: Vec<f64> = (0..n).map(|i| self.capacity[i][p]).collect();
            let sd: Vec<f64> = (0..n).map(|i| self.demand[i][p]).collect();

            // Diverge percentages (Equations 25-23 through 25-25).
            let pct = self.diverge_percentages(p, &offrd);
            let p_prev = p.saturating_sub(1);
            let offrd_prev = self.offrd_by_node(p_prev);
            let pct_prev = self.diverge_percentages(p_prev, &offrd_prev);

            // Expected demands and background densities (Equations 25-6/25-7).
            let ed = OversaturatedEngine::expected_demand(
                &capacity,
                self.mainline_demand[p],
                &onrd,
                &offrd,
            );
            let kb: Vec<f64> = (0..n)
                .map(|i| {
                    let onr_ed = onrd[i].min(ed[i]);
                    let eval = self.engine_eval(i, p, ed[i], onr_ed, offrd[i + 1]);
                    if eval.speed > 0.0 {
                        ed[i] / lanes[i] / eval.speed
                    } else {
                        ed[i] / lanes[i] / 1.0
                    }
                })
                .collect();

            // Front-clearing-queue detection (Equation 25-12).
            let front_clearing: Vec<bool> = (0..n)
                .map(|i| {
                    if p == 0 {
                        return false;
                    }
                    let onrd_prev_i = self.onrd_by_node(p - 1)[i];
                    OversaturatedEngine::front_clearing_queue(
                        self.capacity[i][p],
                        onrd[i],
                        self.capacity[i][p - 1],
                        onrd_prev_i,
                        self.demand[i][p],
                    )
                })
                .collect();

            let ramp_capacity: Vec<f64> = (0..=n)
                .map(|node| {
                    if node < n && onrd[node] > 0.0 {
                        get_ramp_capacity(self.segments[node].ramp_ffs, false) * f_hv
                    } else {
                        0.0
                    }
                })
                .collect();
            let ramp_metering: Vec<Option<f64>> = (0..=n)
                .map(|node| {
                    if node < n {
                        self.segments[node]
                            .ramp_metering
                            .as_ref()
                            .and_then(|v| v.get(p).copied())
                    } else {
                        None
                    }
                })
                .collect();

            let input = OversatPeriodInput {
                capacity,
                demand: sd,
                mainline_demand: self.mainline_demand[p],
                onrd: onrd.clone(),
                offrd,
                ramp_capacity,
                ramp_metering,
                background_density: kb,
                diverge_pct: pct,
                diverge_pct_prev: pct_prev,
                front_clearing,
            };
            let res = engine.run_period(&input);

            // Queued speed/density (Equations 25-32/25-33): U = SF / K with
            // per-lane density, i.e., U = (SF / N) / K.
            let queued_speed: Vec<f64> = (0..n)
                .map(|i| {
                    if res.density[i] > 0.0 {
                        (res.segment_flow[i] / lanes[i]) / res.density[i]
                    } else {
                        self.seg_ffs(i)
                    }
                })
                .collect();

            self.evaluate_period_chain(
                p,
                &res.segment_flow,
                &res.onr_flow,
                &res.ofr_flow,
                &res.had_queue,
                &queued_speed,
                &res.density,
            );
            for i in 0..n {
                self.had_queue[i][p] = res.had_queue[i];
                self.queue_length_ft[i][p] = res.queue_length_ft[i];
                self.on_ramp_queue[i][p] = res.onr_queue_end[i];
            }
            self.unserved_entry_veh[p] = res.entry_queue_end[0];
        }
    }

    /// Off-ramp diverge percentages `OFRD(i, p) / SD(i − 1, p)` by node.
    fn diverge_percentages(&self, p: usize, offrd: &[f64]) -> Vec<f64> {
        let n = self.num_segments();
        (0..=n)
            .map(|node| {
                if node == 0 || offrd[node] <= 0.0 {
                    0.0
                } else {
                    let sd_up = self.demand[node - 1][p];
                    if sd_up > 0.0 {
                        (offrd[node] / sd_up).min(1.0)
                    } else {
                        0.0
                    }
                }
            })
            .collect()
    }

    // ── Steps A-15/A-17: facility aggregation and LOS ────────────────────

    /// Facility-wide performance measures for each analysis period
    /// (Equations 25-2 and 10-1) and facility LOS (Exhibit 10-6).
    pub fn compute_facility_performance(&mut self) {
        let n = self.num_segments();
        let p_count = self.num_periods();
        let f_hv = self.f_hv();
        self.facility_performance = Vec::with_capacity(p_count);
        for p in 0..p_count {
            let flows: Vec<f64> = (0..n).map(|i| self.volume_served[i][p]).collect();
            let lengths: Vec<f64> = self.segments.iter().map(|s| s.length_ft).collect();
            let speeds: Vec<f64> = (0..n).map(|i| self.speed[i][p]).collect();
            let dens_veh: Vec<f64> = (0..n).map(|i| self.density_veh[i][p]).collect();
            let lanes: Vec<f64> = self.segments.iter().map(|s| f64::from(s.lanes)).collect();

            let sms = exhibits::facility_space_mean_speed(&flows, &lengths, &speeds);
            let k_veh = exhibits::facility_density(&dens_veh, &lengths, &lanes);
            let k_pc = k_veh / f_hv;
            let any_over = (0..n).any(|i| self.dc_ratio[i][p] > 1.0);
            let los = exhibits::los_freeway_facility(k_pc, any_over, self.city_type);

            let mut vmt_served = 0.0;
            let mut vmt_demand = 0.0;
            let mut vht = 0.0;
            let mut vhd = 0.0;
            for i in 0..n {
                let l_mi = self.segments[i].length_mi();
                vmt_served += self.volume_served[i][p] * 0.25 * l_mi;
                vmt_demand += self.demand[i][p] * 0.25 * l_mi;
                if self.speed[i][p] > 0.0 {
                    let t_hr = l_mi / self.speed[i][p];
                    let t_ffs = l_mi / self.seg_ffs(i);
                    vht += self.volume_served[i][p] * 0.25 * t_hr;
                    vhd += self.volume_served[i][p] * 0.25 * (t_hr - t_ffs).max(0.0);
                }
            }

            self.facility_performance.push(PeriodPerformance {
                space_mean_speed: sms,
                avg_density_veh: k_veh,
                avg_density_pc: k_pc,
                los,
                vmt_served,
                vmt_demand,
                vht,
                vhd,
            });
        }
    }

    /// Overall space mean speed across all analysis periods (Equation 25-4).
    pub fn overall_space_mean_speed(&self) -> f64 {
        let mut num = 0.0;
        let mut den = 0.0;
        for p in 0..self.num_periods() {
            for i in 0..self.num_segments() {
                let f = self.volume_served[i][p];
                let l = self.segments[i].length_ft;
                if self.speed[i][p] > 0.0 {
                    num += f * l;
                    den += f * l / self.speed[i][p];
                }
            }
        }
        if den > 0.0 {
            num / den
        } else {
            0.0
        }
    }

    /// Overall average density across all periods, veh/mi/ln
    /// (Equation 25-5).
    pub fn overall_density_veh(&self) -> f64 {
        let mut num = 0.0;
        let mut den = 0.0;
        for p in 0..self.num_periods() {
            for i in 0..self.num_segments() {
                let w = self.segments[i].length_ft * f64::from(self.segments[i].lanes);
                num += self.density_veh[i][p] * w;
                den += w;
            }
        }
        if den > 0.0 {
            num / den
        } else {
            0.0
        }
    }

    // ── Validation ───────────────────────────────────────────────────────

    /// Basic structural checks per the Chapter 10 guidance (first/last
    /// segment basic; demand vectors sized to the study period).
    pub fn validate(&self) -> Result<(), String> {
        if self.segments.is_empty() {
            return Err("facility has no segments".into());
        }
        if self.mainline_demand.is_empty() {
            return Err("no analysis periods (mainline_demand is empty)".into());
        }
        if self.segments.first().unwrap().seg_type != SegmentType::Basic
            || self.segments.last().unwrap().seg_type != SegmentType::Basic
        {
            return Err(
                "first and last segments must be basic freeway segments (Chapter 10)".into(),
            );
        }
        for (i, seg) in self.segments.iter().enumerate() {
            if seg.length_ft <= 0.0 {
                return Err(format!("segment {i} has non-positive length"));
            }
            if seg.lanes < 2 {
                return Err(format!("segment {i} must have at least 2 lanes"));
            }
        }
        Ok(())
    }

    /// Run the full core methodology (Steps A-1 through A-17, motorized
    /// vehicle, general purpose lanes).
    pub fn run_analysis(&mut self) -> Result<(), String> {
        self.validate()?;
        self.compute_demands(); // Steps A-3/A-4
        self.compute_capacities(); // Steps A-7/A-8
        self.compute_dc_ratios(); // Step A-10
        self.alloc_results();

        let first = self.first_oversat_period;
        let undersat_until = first.unwrap_or(self.num_periods());
        for p in 0..undersat_until {
            self.analyze_undersaturated_period(p); // Step A-11
        }
        if let Some(f) = first {
            self.analyze_oversaturated(f); // Step A-12
        }
        self.compute_facility_performance(); // Steps A-15/A-17
        Ok(())
    }

    // ── Result accessors ─────────────────────────────────────────────────

    pub fn get_speed(&self, seg: usize, period: usize) -> f64 {
        self.speed[seg][period]
    }

    pub fn get_density_veh(&self, seg: usize, period: usize) -> f64 {
        self.density_veh[seg][period]
    }

    pub fn get_los(&self, seg: usize, period: usize) -> LevelOfService {
        self.los[seg][period]
    }

    pub fn get_facility_los(&self, period: usize) -> LevelOfService {
        self.facility_performance[period].los
    }

    pub fn get_facility_speed(&self, period: usize) -> f64 {
        self.facility_performance[period].space_mean_speed
    }

    pub fn get_facility_density_veh(&self, period: usize) -> f64 {
        self.facility_performance[period].avg_density_veh
    }
}

// ═════════════════════════════════════════════════════════════════════════
// Segmentation rules (Step A-2; Exhibits 10-11 and 10-12)
// ═════════════════════════════════════════════════════════════════════════

/// Convert a section between an on-ramp gore and a downstream off-ramp
/// gore into HCM analysis segments per the Chapter 10 segmentation rules
/// (Exhibit 10-11 and the Facility Segmentation Guidance):
///
/// - with an auxiliary lane connecting the gores: one weaving segment;
/// - spacing > 3,000 ft: merge (1,500 ft) + basic (spacing − 3,000) +
///   diverge (1,500 ft);
/// - 1,500 ft < spacing <= 3,000 ft: merge (spacing − 1,500) + overlapping
///   ramp (3,000 − spacing) + diverge (spacing − 1,500);
/// - spacing <= 1,500 ft (no auxiliary lane; highly unusual): the ramp
///   influence areas are truncated at the adjacent gore and the worst case
///   applies over the whole distance — returned as a single overlapping
///   ramp segment.
///
/// Returned tuples are `(SegmentType, length_ft)`; zero-length pieces are
/// omitted.
pub fn segment_ramp_section(
    gore_to_gore_ft: f64,
    has_auxiliary_lane: bool,
) -> Vec<(SegmentType, f64)> {
    if has_auxiliary_lane {
        return vec![(SegmentType::Weaving, gore_to_gore_ft)];
    }
    let s = gore_to_gore_ft;
    if s > 2.0 * RAMP_INFLUENCE_AREA_FT {
        vec![
            (SegmentType::Merge, RAMP_INFLUENCE_AREA_FT),
            (SegmentType::Basic, s - 2.0 * RAMP_INFLUENCE_AREA_FT),
            (SegmentType::Diverge, RAMP_INFLUENCE_AREA_FT),
        ]
    } else if s > RAMP_INFLUENCE_AREA_FT {
        let outer = s - RAMP_INFLUENCE_AREA_FT;
        let overlap = 2.0 * RAMP_INFLUENCE_AREA_FT - s;
        let mut out = vec![(SegmentType::Merge, outer)];
        if overlap > 0.0 {
            out.push((SegmentType::OverlappingRamp, overlap));
        }
        out.push((SegmentType::Diverge, outer));
        out
    } else {
        vec![(SegmentType::OverlappingRamp, s)]
    }
}
