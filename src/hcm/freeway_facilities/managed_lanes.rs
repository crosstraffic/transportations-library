//! HCM Chapter 10: Managed-Lane Freeway Facilities (Steps A-9/A-13/A-14/A-17).
//!
//! Extends the general-purpose (GP) facility engine in
//! [`super::freeway_facilities`] to facilities that carry one or more managed
//! lanes (ML) alongside the GP lanes, implementing the Chapter 10 Section 4 /
//! Chapter 25 Section 2 managed-lane methodology:
//!
//! - the **lane-group concept** (Chapter 10, *Segmentation Considerations*):
//!   each GP analysis segment may be paired with a parallel ML segment of the
//!   same length; the two lane groups are analyzed with interaction effects
//!   but reported separately (Steps A-14/A-17);
//! - the **cross-weave friction effect** (Step A-9; Chapter 13 Equations
//!   13-24/13-25): where an at-grade GP on-/off-ramp lies near an ML access
//!   opening, the cross-weave demand reduces the GP segment capacity through a
//!   capacity adjustment factor;
//! - the **adjacent (ML) friction effect** (Step A-13; Chapter 12 Equations
//!   12-18/12-19): an ML segment without a physical barrier loses speed when
//!   the adjacent GP lane density exceeds 35 pc/mi/ln — evaluated by the
//!   Chapter 12 ML segment engine ([`ManagedLaneSegment`]);
//! - facility-level **lane-group and combined aggregation** (Steps A-14/A-17;
//!   Equations 10-1/25-2; Exhibit 10-6).
//!
//! Oversaturated ML operation: Chapter 25 (Section 4, *Oversaturation
//! Analysis within Managed Lanes*) runs the oversaturated engine separately
//! for each lane group and models spillback across access segments only as a
//! non-propagating **vertical queue** (Equations 25-35/25-36). That vertical
//! queue delay accounting is **deferred** here (documented below); the GP and
//! ML lane groups are each analyzed with the existing under-/oversaturated
//! engines, which is exact for facilities whose lane groups do not exchange
//! flow through access segments (the common case and the published Example
//! Problem 5, which is undersaturated).
//!
//! Sources (HCM 7th Edition EPUB): `68_Ch10_02.xhtml` … `70_Ch10_04.xhtml`
//! (Steps A-9/A-13/A-14, Managed Lanes Analysis Section 4); `195_Ch25_04.xhtml`
//! (oversaturated ML, vertical queue); `202_Ch25_11.xhtml` (Example Problem 5);
//! `91_Ch13_04.xhtml` (Equations 13-24/13-25, cross-weave CAF).

use serde::{Deserialize, Serialize};

use crate::hcm::basicfreeways::managed_lanes::{ManagedLaneSegment, ManagedLaneType};
use crate::hcm::common::LevelOfService;

use super::exhibits::{self, los_freeway_facility};
use super::freeway_facilities::{FreewayFacility, PeriodPerformance};

/// Adjacent-friction GP density threshold, pc/mi/ln — Chapter 12 Equation
/// 12-18 (the ML friction indicator I_c switches on when the adjacent GP lane
/// density exceeds this value). Also cited in Chapter 10 Step A-13.
pub const ADJACENT_FRICTION_THRESHOLD_PC: f64 = 35.0;

// ═════════════════════════════════════════════════════════════════════════
// Cross-weave capacity adjustment (Step A-9; Chapter 13 Equations 13-24/13-25)
// ═════════════════════════════════════════════════════════════════════════

/// Chapter 13 Equation 13-24: cross-weave capacity **reduction** factor CRF
/// for a general-purpose segment upstream of (or downstream of) a managed
/// lane access opening:
///
/// `CRF = −0.0897 + 0.0252·ln(CW) − 0.00001453·L_cw-min + 0.002967·N_GP`
///
/// * `cw_demand_pc` — cross-weave demand flow rate CW, pc/h (the GP ramp flow
///   destined for / originating from the managed lane)
/// * `l_cw_min_ft` — minimum cross-weave length L_cw-min, ft (gore to the
///   start of the ML access opening)
/// * `n_gp_lanes` — number of general-purpose lanes N_GP crossed
///
/// The result is clamped to `[0, 1]` (a non-positive CRF means no reduction).
pub fn cross_weave_crf(cw_demand_pc: f64, l_cw_min_ft: f64, n_gp_lanes: u32) -> f64 {
    if cw_demand_pc <= 0.0 {
        return 0.0;
    }
    let crf = -0.0897 + 0.0252 * cw_demand_pc.ln() - 0.00001453 * l_cw_min_ft
        + 0.002967 * f64::from(n_gp_lanes);
    crf.clamp(0.0, 1.0)
}

/// Chapter 13 Equations 13-24/13-25: cross-weave capacity **adjustment**
/// factor `CAF = 1 − CRF` applied to the general-purpose segment capacity
/// (`c_GPA = c_GP · CAF`, Equation 13-25). Arguments as in [`cross_weave_crf`].
pub fn cross_weave_caf(cw_demand_pc: f64, l_cw_min_ft: f64, n_gp_lanes: u32) -> f64 {
    1.0 - cross_weave_crf(cw_demand_pc, l_cw_min_ft, n_gp_lanes)
}

/// Cross-weave data for one general-purpose segment (Step A-9): the GP ramp
/// flow that must cross the GP lanes to reach the adjacent ML access opening,
/// by analysis period, and the minimum cross-weave length.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CrossWeave {
    /// Cross-weave demand CW by period, pc/h (Equation 13-24).
    pub cw_demand_pc: Vec<f64>,
    /// Minimum cross-weave length L_cw-min, ft (Equation 13-24).
    pub l_cw_min_ft: f64,
}

impl CrossWeave {
    fn caf(&self, period: usize, n_gp_lanes: u32) -> f64 {
        let cw = self.cw_demand_pc.get(period).copied().unwrap_or(0.0);
        cross_weave_caf(cw, self.l_cw_min_ft, n_gp_lanes)
    }
}

// ═════════════════════════════════════════════════════════════════════════
// Managed-lane segment input (parallel lane group)
// ═════════════════════════════════════════════════════════════════════════

/// One managed-lane analysis segment, paired with the general-purpose segment
/// at the same index (Chapter 10 lane-group concept). Demand vectors are
/// indexed by analysis period, in veh/h.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct MlSegmentInput {
    /// ML separation/type (Exhibit 12-9) governing the speed-flow parameters
    /// and whether the adjacent friction effect applies (Continuous Access
    /// and Buffer 1 only).
    pub lane_type: ManagedLaneType,
    /// Number of managed lanes in this segment (usually 1).
    pub lanes: u32,
    /// ML free-flow speed override, mi/h (facility `ml_ffs` if `None`).
    pub ffs: Option<f64>,
    /// ML calibration capacity adjustment factor.
    pub caf: f64,
    /// ML calibration speed adjustment factor.
    pub saf: f64,
    /// ML on-ramp demand by period, veh/h (ML merge/access segments).
    pub on_ramp_demand: Vec<f64>,
    /// ML off-ramp demand by period, veh/h (ML diverge/access segments).
    pub off_ramp_demand: Vec<f64>,
}

impl Default for MlSegmentInput {
    fn default() -> Self {
        Self {
            lane_type: ManagedLaneType::ContinuousAccess,
            lanes: 1,
            ffs: None,
            caf: 1.0,
            saf: 1.0,
            on_ramp_demand: Vec::new(),
            off_ramp_demand: Vec::new(),
        }
    }
}

// ═════════════════════════════════════════════════════════════════════════
// Lane-group / facility performance
// ═════════════════════════════════════════════════════════════════════════

/// Per-period performance of a single lane group (Step A-14; Exhibit 25-86).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaneGroupPerformance {
    /// Lane-group space mean speed, mi/h (Equation 25-2).
    pub space_mean_speed: f64,
    /// Lane-group average density, veh/mi/ln (Equation 10-1).
    pub avg_density_veh: f64,
    /// Lane-group average density, pc/mi/ln.
    pub avg_density_pc: f64,
    /// Lane-group LOS (Exhibit 10-6; F if any lane-group segment vd/c > 1.00).
    pub los: LevelOfService,
}

// ═════════════════════════════════════════════════════════════════════════
// ManagedLaneFacility
// ═════════════════════════════════════════════════════════════════════════

/// A directional freeway facility with a parallel managed-lane lane group,
/// analyzed with the HCM Chapter 10 managed-lane extension.
///
/// The general-purpose lane group is a full [`FreewayFacility`]; the managed
/// lane is described by `ml`, a vector parallel to `gp.segments` where each
/// entry is `Some` for GP segments that carry an adjacent managed lane. The
/// analysis:
///
/// 1. applies the Step A-9 cross-weave CAF to the GP segment capacities;
/// 2. runs the GP lane group with the existing core methodology;
/// 3. accumulates ML demands and evaluates each ML segment with the Chapter 12
///    ML engine, applying the Step A-13 adjacent friction using the adjacent
///    GP segment density;
/// 4. aggregates lane-group and combined facility performance (Steps A-14/A-17).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ManagedLaneFacility {
    // ── Inputs ───────────────────────────────────────────────────────────
    /// General-purpose lane group (Chapter 10 core methodology facility).
    pub gp: FreewayFacility,
    /// Parallel managed-lane segment inputs; `ml[i]` pairs with GP segment
    /// `i` (`None` where the segment has no adjacent managed lane).
    pub ml: Vec<Option<MlSegmentInput>>,
    /// Managed-lane demand entering the facility by period, veh/h.
    pub ml_entry_demand: Vec<f64>,
    /// Managed-lane facility free-flow speed, mi/h (segment overrides allowed).
    pub ml_ffs: f64,
    /// Cross-weave data per GP segment (Step A-9); `None` where no cross-weave.
    pub cross_weave: Vec<Option<CrossWeave>>,

    // ── Computed ([segment][period]) ─────────────────────────────────────
    /// ML segment demand, veh/h.
    pub ml_demand: Vec<Vec<f64>>,
    /// ML segment capacity, veh/h.
    pub ml_capacity: Vec<Vec<f64>>,
    /// ML demand-to-capacity ratio.
    pub ml_dc_ratio: Vec<Vec<f64>>,
    /// ML segment space mean speed, mi/h.
    pub ml_speed: Vec<Vec<f64>>,
    /// ML segment density, veh/mi/ln.
    pub ml_density_veh: Vec<Vec<f64>>,
    /// ML segment density, pc/mi/ln.
    pub ml_density_pc: Vec<Vec<f64>>,
    /// ML density-based segment LOS (Exhibit 12-15 thresholds).
    pub ml_los: Vec<Vec<LevelOfService>>,
    /// Whether the Step A-13 adjacent friction was active on the ML segment.
    pub ml_friction_active: Vec<Vec<bool>>,

    /// GP lane-group performance by period (Step A-14).
    pub gp_group_performance: Vec<LaneGroupPerformance>,
    /// ML lane-group performance by period (Step A-14).
    pub ml_group_performance: Vec<LaneGroupPerformance>,
    /// Combined facility performance by period (Step A-17; Exhibit 25-87).
    pub facility_performance: Vec<PeriodPerformance>,
}

impl Default for ManagedLaneFacility {
    fn default() -> Self {
        Self {
            gp: FreewayFacility::new(),
            ml: Vec::new(),
            ml_entry_demand: Vec::new(),
            ml_ffs: 60.0,
            cross_weave: Vec::new(),
            ml_demand: Vec::new(),
            ml_capacity: Vec::new(),
            ml_dc_ratio: Vec::new(),
            ml_speed: Vec::new(),
            ml_density_veh: Vec::new(),
            ml_density_pc: Vec::new(),
            ml_los: Vec::new(),
            ml_friction_active: Vec::new(),
            gp_group_performance: Vec::new(),
            ml_group_performance: Vec::new(),
            facility_performance: Vec::new(),
        }
    }
}

impl ManagedLaneFacility {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn num_segments(&self) -> usize {
        self.gp.num_segments()
    }

    pub fn num_periods(&self) -> usize {
        self.gp.num_periods()
    }

    fn ml_seg_ffs(&self, seg: &MlSegmentInput) -> f64 {
        seg.ffs.unwrap_or(self.ml_ffs)
    }

    /// Step A-9: fold the cross-weave CAF (Equation 13-25) into each affected
    /// GP segment's calibration CAF before the GP analysis runs. Because the
    /// cross-weave CAF varies by period, a period-varying CAF schedule is
    /// installed on the GP segment.
    fn apply_cross_weave(&mut self) {
        let p_count = self.num_periods();
        for i in 0..self.num_segments() {
            let Some(Some(cw)) = self.cross_weave.get(i) else {
                continue;
            };
            let n_gp = self.gp.segments[i].lanes;
            let base_caf = self.gp.segments[i].caf;
            let schedule: Vec<f64> = (0..p_count)
                .map(|p| base_caf * cw.caf(p, n_gp))
                .collect();
            self.gp.segments[i].caf_schedule = Some(schedule);
        }
    }

    /// Accumulate ML segment demands from the ML entry demand and per-segment
    /// ML ramp demands (mirror of the GP demand accumulation, Step A-4).
    fn compute_ml_demands(&mut self) {
        let n = self.num_segments();
        let p_count = self.num_periods();
        self.ml_demand = vec![vec![0.0; p_count]; n];
        for p in 0..p_count {
            let mut upstream = self.ml_entry_demand.get(p).copied().unwrap_or(0.0);
            for i in 0..n {
                if let Some(seg) = &self.ml[i] {
                    let on = seg.on_ramp_demand.get(p).copied().unwrap_or(0.0);
                    let off = seg.off_ramp_demand.get(p).copied().unwrap_or(0.0);
                    upstream = upstream + on - off;
                }
                self.ml_demand[i][p] = upstream;
            }
        }
    }

    /// Build a Chapter 12 ML segment engine for GP index `i`, period `p`,
    /// at served volume `volume_veh` (veh/h) with adjacent GP density
    /// `k_gp_pc` (pc/mi/ln, Step A-13). Returns `None` where there is no ML.
    fn ml_engine(&self, seg: &MlSegmentInput, volume_veh: f64, k_gp_pc: f64) -> ManagedLaneSegment {
        let f_hv = self.gp.f_hv();
        let phf = self.gp.phf;
        let lanes = f64::from(seg.lanes.max(1));
        // Convert served vehicles to per-lane passenger-car flow (Chapter 12
        // works in pc/h/ln); the ML shares the facility heavy-vehicle factor.
        let v_p = volume_veh / (lanes * f_hv * phf);
        let mut ml = ManagedLaneSegment::new(seg.lane_type, self.ml_seg_ffs(seg));
        ml.set_saf(seg.saf);
        ml.set_caf(seg.caf);
        ml.set_demand(v_p);
        ml.set_gp_density(k_gp_pc);
        ml
    }

    /// ML segment capacities (veh/h) — the Chapter 12 adjusted per-lane
    /// capacity times the lane count and heavy-vehicle factor.
    fn compute_ml_capacities(&mut self) {
        let n = self.num_segments();
        let p_count = self.num_periods();
        let f_hv = self.gp.f_hv();
        self.ml_capacity = vec![vec![0.0; p_count]; n];
        for i in 0..n {
            let Some(seg) = &self.ml[i] else { continue };
            let lanes = f64::from(seg.lanes.max(1));
            let mut ml = ManagedLaneSegment::new(seg.lane_type, self.ml_seg_ffs(seg));
            ml.set_caf(seg.caf);
            ml.set_saf(seg.saf);
            ml.calculate_ffs_adj();
            let cap_pc = ml.calculate_capacity();
            for p in 0..p_count {
                self.ml_capacity[i][p] = cap_pc * lanes * f_hv;
            }
        }
    }

    /// Run the full managed-lane facility analysis.
    pub fn run_analysis(&mut self) -> Result<(), String> {
        let n = self.num_segments();
        if self.ml.len() != n {
            return Err(format!(
                "ml has {} entries but the facility has {} segments",
                self.ml.len(),
                n
            ));
        }
        if self.cross_weave.is_empty() {
            self.cross_weave = vec![None; n];
        } else if self.cross_weave.len() != n {
            return Err(format!(
                "cross_weave has {} entries but the facility has {} segments",
                self.cross_weave.len(),
                n
            ));
        }

        // Step A-9: cross-weave friction on GP capacity, then run GP group.
        self.apply_cross_weave();
        self.gp.run_analysis()?;

        // ML demands, capacities, d/c ratios.
        self.compute_ml_demands();
        self.compute_ml_capacities();
        let p_count = self.num_periods();
        self.ml_dc_ratio = vec![vec![0.0; p_count]; n];
        for i in 0..n {
            if self.ml[i].is_none() {
                continue;
            }
            for p in 0..p_count {
                let cap = self.ml_capacity[i][p];
                self.ml_dc_ratio[i][p] = if cap > 0.0 {
                    self.ml_demand[i][p] / cap
                } else {
                    0.0
                };
            }
        }

        self.evaluate_ml_segments();
        self.aggregate_performance();
        Ok(())
    }

    /// Steps A-11/A-13: evaluate each ML segment with the Chapter 12 engine,
    /// applying the adjacent friction from the paired GP segment density.
    fn evaluate_ml_segments(&mut self) {
        let n = self.num_segments();
        let p_count = self.num_periods();
        let f_hv = self.gp.f_hv();
        self.ml_speed = vec![vec![0.0; p_count]; n];
        self.ml_density_veh = vec![vec![0.0; p_count]; n];
        self.ml_density_pc = vec![vec![0.0; p_count]; n];
        self.ml_los = vec![vec![LevelOfService::A; p_count]; n];
        self.ml_friction_active = vec![vec![false; p_count]; n];

        for i in 0..n {
            let Some(seg) = self.ml[i].clone() else {
                continue;
            };
            for p in 0..p_count {
                // Step A-13: adjacent GP lane density (pc/mi/ln), from the
                // paired GP segment. The Chapter 12 engine switches on the
                // friction speed drop when this exceeds 35 pc/mi/ln.
                //
                // VERIFY-HCM: reproduces every Example Problem 5 ML speed cell
                // (Exhibit 25-83) except Segment 10 / Period 2 (book 58.1),
                // where the adjacent GP density is 34.2 pc/mi/ln (below the
                // threshold) so no friction applies and we compute 58.9. See
                // docs/hcm/VERIFICATION.md, item 2.
                let k_gp_pc = self.gp.density_pc[i][p];
                let volume = self.ml_demand[i][p];
                let mut ml = self.ml_engine(&seg, volume, k_gp_pc);
                ml.run_analysis();
                self.ml_speed[i][p] = ml.speed;
                self.ml_density_pc[i][p] = ml.density;
                self.ml_density_veh[i][p] = ml.density * f_hv;
                self.ml_los[i][p] = ml.los.unwrap_or(LevelOfService::F);
                let friction_capable = matches!(
                    seg.lane_type,
                    ManagedLaneType::ContinuousAccess | ManagedLaneType::Buffer1
                );
                self.ml_friction_active[i][p] =
                    friction_capable && k_gp_pc > ADJACENT_FRICTION_THRESHOLD_PC;
            }
        }
    }

    /// Steps A-14/A-17: lane-group and combined facility aggregation.
    fn aggregate_performance(&mut self) {
        let n = self.num_segments();
        let p_count = self.num_periods();
        let f_hv = self.gp.f_hv();
        let city = self.gp.city_type;

        self.gp_group_performance = Vec::with_capacity(p_count);
        self.ml_group_performance = Vec::with_capacity(p_count);
        self.facility_performance = Vec::with_capacity(p_count);

        for p in 0..p_count {
            // GP lane group (Equation 10-1 / 25-2 over GP segments).
            let gp_flows: Vec<f64> = (0..n).map(|i| self.gp.volume_served[i][p]).collect();
            let gp_lengths: Vec<f64> = self.gp.segments.iter().map(|s| s.length_ft).collect();
            let gp_speeds: Vec<f64> = (0..n).map(|i| self.gp.speed[i][p]).collect();
            let gp_dens: Vec<f64> = (0..n).map(|i| self.gp.density_veh[i][p]).collect();
            let gp_lanes: Vec<f64> =
                self.gp.segments.iter().map(|s| f64::from(s.lanes)).collect();
            let gp_over = (0..n).any(|i| self.gp.dc_ratio[i][p] > 1.0);
            let gp_k = exhibits::facility_density(&gp_dens, &gp_lengths, &gp_lanes);
            self.gp_group_performance.push(LaneGroupPerformance {
                space_mean_speed: exhibits::facility_space_mean_speed(
                    &gp_flows,
                    &gp_lengths,
                    &gp_speeds,
                ),
                avg_density_veh: gp_k,
                avg_density_pc: gp_k / f_hv,
                los: los_freeway_facility(gp_k / f_hv, gp_over, city),
            });

            // ML lane group (only segments that carry a managed lane).
            let mut ml_flows = Vec::new();
            let mut ml_lengths = Vec::new();
            let mut ml_speeds = Vec::new();
            let mut ml_dens = Vec::new();
            let mut ml_lanes = Vec::new();
            let mut ml_over = false;
            for i in 0..n {
                let Some(seg) = &self.ml[i] else { continue };
                ml_flows.push(self.ml_demand[i][p]);
                ml_lengths.push(self.gp.segments[i].length_ft);
                ml_speeds.push(self.ml_speed[i][p]);
                ml_dens.push(self.ml_density_veh[i][p]);
                ml_lanes.push(f64::from(seg.lanes.max(1)));
                if self.ml_dc_ratio[i][p] > 1.0 {
                    ml_over = true;
                }
            }
            let ml_k = exhibits::facility_density(&ml_dens, &ml_lengths, &ml_lanes);
            self.ml_group_performance.push(LaneGroupPerformance {
                space_mean_speed: exhibits::facility_space_mean_speed(
                    &ml_flows,
                    &ml_lengths,
                    &ml_speeds,
                ),
                avg_density_veh: ml_k,
                avg_density_pc: ml_k / f_hv,
                los: los_freeway_facility(ml_k / f_hv, ml_over, city),
            });

            // Combined facility (both lane groups; Equation 10-1 / 25-2).
            //
            // VERIFY-HCM: this lane-mile-weighted density is the exact
            // Equation 10-1 combination of the GP and ML group densities; in
            // Example Problem 5, Period 3 it gives 28.3 veh/mi/ln whereas
            // Exhibit 25-87 reports 29.1 — a value not reproducible from the
            // book's own Exhibit 25-86 group densities. LOS is unaffected.
            let mut flows = gp_flows.clone();
            let mut lengths = gp_lengths.clone();
            let mut speeds = gp_speeds.clone();
            let mut dens = gp_dens.clone();
            let mut lanes = gp_lanes.clone();
            flows.extend(&ml_flows);
            lengths.extend(&ml_lengths);
            speeds.extend(&ml_speeds);
            dens.extend(&ml_dens);
            lanes.extend(&ml_lanes);

            let k_veh = exhibits::facility_density(&dens, &lengths, &lanes);
            let k_pc = k_veh / f_hv;
            let any_over = gp_over || ml_over;
            let los = los_freeway_facility(k_pc, any_over, city);
            let sms = exhibits::facility_space_mean_speed(&flows, &lengths, &speeds);

            // VMT/VHT/VHD across both lane groups (Steps A-15/A-17).
            let mut vmt_served = 0.0;
            let mut vmt_demand = 0.0;
            let mut vht = 0.0;
            let mut vhd = 0.0;
            for i in 0..n {
                let l_mi = self.gp.segments[i].length_ft / 5280.0;
                let ffs_gp = self.gp.segments[i].ffs.unwrap_or(self.gp.ffs);
                vmt_served += self.gp.volume_served[i][p] * 0.25 * l_mi;
                vmt_demand += self.gp.demand[i][p] * 0.25 * l_mi;
                if self.gp.speed[i][p] > 0.0 {
                    let t = l_mi / self.gp.speed[i][p];
                    vht += self.gp.volume_served[i][p] * 0.25 * t;
                    vhd += self.gp.volume_served[i][p] * 0.25 * (t - l_mi / ffs_gp).max(0.0);
                }
                if let Some(seg) = &self.ml[i] {
                    let ffs_ml = self.ml_seg_ffs(seg);
                    vmt_served += self.ml_demand[i][p] * 0.25 * l_mi;
                    vmt_demand += self.ml_demand[i][p] * 0.25 * l_mi;
                    if self.ml_speed[i][p] > 0.0 {
                        let t = l_mi / self.ml_speed[i][p];
                        vht += self.ml_demand[i][p] * 0.25 * t;
                        vhd += self.ml_demand[i][p] * 0.25 * (t - l_mi / ffs_ml).max(0.0);
                    }
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

    // ── Result accessors ─────────────────────────────────────────────────

    pub fn ml_speed(&self, seg: usize, period: usize) -> f64 {
        self.ml_speed[seg][period]
    }

    pub fn ml_density_veh(&self, seg: usize, period: usize) -> f64 {
        self.ml_density_veh[seg][period]
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
