//! HCM Chapter 13: Freeway Weaving Segments.
//!
//! Implements the HCM 7th Edition core weaving methodology (Steps 2-8):
//! demand adjustment (Eq. 13-1), configuration characteristics (Eqs. 13-2/13-3),
//! maximum weaving length (Eq. 13-4), capacity (Eqs. 13-5 through 13-10),
//! lane-changing rates (Eqs. 13-11 through 13-17), speeds (Eqs. 13-18 through
//! 13-22), and density/LOS (Eq. 13-23, Exhibit 13-6).

use serde::{Deserialize, Serialize};
use crate::hcm::common::{HcmVersion, LevelOfService};
use super::v7_1::WeavingAnalysis;

// =============================================================================
// Constants
// =============================================================================

/// Minimum average speed of weaving vehicles expected in a weaving segment (mi/h)
/// HCM Chapter 13, Step 7 (S_MIN in Equation 13-18).
pub const MIN_WEAVING_SPEED: f64 = 15.0;

/// Minimum weaving segment length used in Equation 13-11 (ft).
/// For segments of 300 ft or shorter, LC_W = LC_MIN.
pub const MIN_WEAVING_LENGTH: f64 = 300.0;

/// Maximum weaving flow rate for N_WL = 2 lanes (pc/h) - Equation 13-7
pub const MAX_WEAVING_FLOW_NWL2: f64 = 2400.0;

/// Maximum weaving flow rate for N_WL = 3 lanes (pc/h) - Equation 13-7
pub const MAX_WEAVING_FLOW_NWL3: f64 = 3500.0;

/// Density at which breakdown is expected in a weaving segment (pc/mi/ln)
pub const WEAVING_BREAKDOWN_DENSITY: f64 = 43.0;

/// Density at which breakdown is expected on multilane highways/C-D roads (pc/mi/ln)
pub const MULTILANE_BREAKDOWN_DENSITY: f64 = 40.0;

// =============================================================================
// Enums
// =============================================================================

/// Type of weaving segment
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WeavingType {
    /// One-sided weaving (both weaving movements on the same side)
    OneSided,
    /// Two-sided weaving (ramp-to-ramp movement crosses the freeway)
    TwoSided,
}

/// Facility type for LOS criteria - Exhibit 13-6
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FacilityType {
    /// Freeway weaving segment
    Freeway,
    /// Multilane highway or C-D road weaving segment
    MultilaneOrCD,
}

/// Terrain type for PCE selection (Exhibit 12-25)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TerrainType {
    Level,
    Rolling,
    Mountainous,
}

/// LOS criteria for weaving segments - Exhibit 13-6.
///
/// Freeway weaving segments: A <=10, B <=20, C <=28, D <=35, E <=43,
/// F >43 or demand exceeds capacity.
/// Multilane/C-D weaving segments: A <=12, B <=24, C <=32, D <=36, E <=40,
/// F >40 or demand exceeds capacity.
pub fn determine_weaving_los(
    density: f64,
    demand_exceeds_capacity: bool,
    facility: FacilityType,
) -> LevelOfService {
    if demand_exceeds_capacity {
        return LevelOfService::F;
    }

    match facility {
        FacilityType::Freeway => match density {
            d if d <= 10.0 => LevelOfService::A,
            d if d <= 20.0 => LevelOfService::B,
            d if d <= 28.0 => LevelOfService::C,
            d if d <= 35.0 => LevelOfService::D,
            d if d <= WEAVING_BREAKDOWN_DENSITY => LevelOfService::E,
            _ => LevelOfService::F,
        },
        FacilityType::MultilaneOrCD => match density {
            d if d <= 12.0 => LevelOfService::A,
            d if d <= 24.0 => LevelOfService::B,
            d if d <= 32.0 => LevelOfService::C,
            d if d <= 36.0 => LevelOfService::D,
            d if d <= MULTILANE_BREAKDOWN_DENSITY => LevelOfService::E,
            _ => LevelOfService::F,
        },
    }
}

// =============================================================================
// WeavingSegment
// =============================================================================

/// A freeway weaving segment analyzed with the HCM Chapter 13 methodology.
///
/// Input fields are plain values; computed fields are `Option<T>` and are
/// populated by the step methods (in HCM step order) or by `run_analysis`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct WeavingSegment {
    // ── Inputs ──────────────────────────────────────────────────────────
    /// Which HCM edition to analyze this segment under. Edition 7.1 replaced Chapter 13 with a
    /// different methodology, so this selects between two sets of results rather than refining
    /// one. Defaults to the 7th Edition.
    pub version: HcmVersion,
    /// Type of weaving segment (one-sided or two-sided)
    pub weaving_type: WeavingType,
    /// Facility type for LOS criteria (freeway or multilane/C-D)
    pub facility_type: FacilityType,
    /// Short length of weaving segment L_S, ft
    pub length_short: f64,
    /// Number of lanes within the weaving segment N, ln
    pub num_lanes: u32,
    /// Number of weaving lanes N_WL (2 or 3 for one-sided; 0 for two-sided), ln
    pub num_weaving_lanes: u32,
    /// Free-flow speed of the weaving segment, mi/h
    pub ffs: f64,
    /// Freeway-to-freeway demand volume V_FF, veh/h
    pub v_ff: f64,
    /// Freeway-to-ramp demand volume V_FR, veh/h
    pub v_fr: f64,
    /// Ramp-to-freeway demand volume V_RF, veh/h
    pub v_rf: f64,
    /// Ramp-to-ramp demand volume V_RR, veh/h
    pub v_rr: f64,
    /// Peak hour factor, decimal
    pub phf: f64,
    /// Heavy vehicle proportion, decimal (e.g., 0.05 for 5%)
    pub heavy_vehicle_pct: f64,
    /// Terrain type for PCE selection (Exhibit 12-25)
    pub terrain: TerrainType,
    /// Minimum lane changes for one ramp-to-freeway vehicle LC_RF, lc
    pub lc_rf: u32,
    /// Minimum lane changes for one freeway-to-ramp vehicle LC_FR, lc
    pub lc_fr: u32,
    /// Minimum lane changes for one ramp-to-ramp vehicle LC_RR (two-sided), lc
    pub lc_rr: u32,
    /// Number of lanes from which a ramp-to-freeway weaving maneuver may be made with the minimum
    /// number of lane changes NW_RF, ln. Edition 7.1 only (Chapter 13, Exhibit 13-5).
    pub nw_rf: u32,
    /// Number of lanes from which a freeway-to-ramp weaving maneuver may be made with the minimum
    /// number of lane changes NW_FR, ln. Edition 7.1 only.
    pub nw_fr: u32,
    /// Number of lanes from which a ramp-to-ramp weaving maneuver may be made with the minimum
    /// number of lane changes NW_RR, ln. Edition 7.1, two-sided segments only (Exhibit 13-6).
    pub nw_rr: u32,
    /// Interchange density ID, int/mi
    pub interchange_density: f64,
    /// Capacity per lane of a basic freeway segment with the same FFS c_IFL, pc/h/ln
    pub basic_freeway_capacity: f64,
    /// Capacity adjustment factor CAF, decimal
    pub caf: f64,
    /// Speed adjustment factor SAF, decimal
    pub saf: f64,

    // ── Computed (populated by step methods) ────────────────────────────
    /// Heavy vehicle adjustment factor f_HV, decimal - Equation 12-10
    pub f_hv: Option<f64>,
    /// Weaving demand flow rate v_W, pc/h
    pub flow_weaving: Option<f64>,
    /// Nonweaving demand flow rate v_NW, pc/h
    pub flow_nonweaving: Option<f64>,
    /// Total demand flow rate v = v_W + v_NW, pc/h
    pub flow_total: Option<f64>,
    /// Volume ratio VR = v_W / v, decimal
    pub volume_ratio: Option<f64>,
    /// Minimum lane-changing rate LC_MIN, lc/h - Equations 13-2/13-3
    pub lc_min: Option<f64>,
    /// Maximum weaving length L_MAX, ft - Equation 13-4
    pub l_max: Option<f64>,
    /// Whether the segment operates as a weaving segment (L_S < L_MAX)
    pub is_weaving: Option<bool>,
    /// Capacity per lane based on density c_IWL, pc/h/ln - Equation 13-5
    pub c_iwl: Option<f64>,
    /// Capacity from the density criterion under prevailing conditions, veh/h - Equation 13-6
    pub capacity_density: Option<f64>,
    /// Capacity from the weaving-flow criterion under prevailing conditions, veh/h - Equations 13-7/13-8
    pub capacity_weaving: Option<f64>,
    /// Final adjusted capacity c_wa = min(Eq 13-6, Eq 13-8) x CAF, veh/h - Equation 13-9
    pub capacity: Option<f64>,
    /// Volume-to-capacity ratio v/c = v x f_HV / c_wa - Equation 13-10
    pub vc_ratio: Option<f64>,
    /// Lane-changing rate of weaving vehicles LC_W, lc/h - Equation 13-11
    pub lc_w: Option<f64>,
    /// Lane-changing rate of nonweaving vehicles LC_NW, lc/h - Equation 13-16
    pub lc_nw: Option<f64>,
    /// Total lane-changing rate LC_ALL, lc/h - Equation 13-17
    pub lc_all: Option<f64>,
    /// Weaving intensity factor W, unitless - Equation 13-20
    pub weaving_intensity: Option<f64>,
    /// Average speed of weaving vehicles S_W, mi/h - Equation 13-19
    pub speed_weaving: Option<f64>,
    /// Average speed of nonweaving vehicles S_NW, mi/h - Equation 13-21
    pub speed_nonweaving: Option<f64>,
    /// Space mean speed of all vehicles S, mi/h - Equation 13-22
    pub speed_avg: Option<f64>,
    /// Density D, pc/mi/ln - Equation 13-23
    pub density: Option<f64>,
    /// Whether demand exceeds capacity (v/c > 1.0)
    pub demand_exceeds_capacity: Option<bool>,
    /// Level of service - Exhibit 13-6 (7th Edition) or Exhibit 13-7 (Edition 7.1)
    pub los: Option<LevelOfService>,
    /// Full Edition 7.1 result, populated only when the segment's version is
    /// [`HcmVersion::V7_1`]. The quantities Edition 7.1 computes and the 7th Edition does not -
    /// the equivalent basic segment, the speed impedance, and the per-lane capacity C_W - live
    /// here rather than in the shared fields above, so no field carries different units depending
    /// on which edition produced it.
    pub analysis_v7_1: Option<WeavingAnalysis>,
}

impl Default for WeavingSegment {
    fn default() -> Self {
        Self {
            version: HcmVersion::V7,
            weaving_type: WeavingType::OneSided,
            facility_type: FacilityType::Freeway,
            length_short: 1500.0,
            num_lanes: 4,
            num_weaving_lanes: 2,
            ffs: 70.0,
            v_ff: 3000.0,
            v_fr: 500.0,
            v_rf: 500.0,
            v_rr: 100.0,
            phf: 0.94,
            heavy_vehicle_pct: 0.05,
            terrain: TerrainType::Level,
            lc_rf: 1,
            lc_fr: 1,
            lc_rr: 0,
            nw_rf: 1,
            nw_fr: 1,
            nw_rr: 0,
            interchange_density: 0.8,
            basic_freeway_capacity: 2400.0,
            caf: 1.0,
            saf: 1.0,
            f_hv: None,
            flow_weaving: None,
            flow_nonweaving: None,
            flow_total: None,
            volume_ratio: None,
            lc_min: None,
            l_max: None,
            is_weaving: None,
            c_iwl: None,
            capacity_density: None,
            capacity_weaving: None,
            capacity: None,
            vc_ratio: None,
            lc_w: None,
            lc_nw: None,
            lc_all: None,
            weaving_intensity: None,
            speed_weaving: None,
            speed_nonweaving: None,
            speed_avg: None,
            density: None,
            demand_exceeds_capacity: None,
            los: None,
            analysis_v7_1: None,
        }
    }
}

impl WeavingSegment {
    pub fn new() -> Self {
        Self::default()
    }

    // ── Accessors ─────────────────────────────────────────────────────────

    pub fn get_flow_weaving(&self) -> f64 {
        self.flow_weaving.unwrap_or(0.0)
    }

    pub fn get_flow_nonweaving(&self) -> f64 {
        self.flow_nonweaving.unwrap_or(0.0)
    }

    pub fn get_flow_total(&self) -> f64 {
        self.flow_total.unwrap_or(0.0)
    }

    pub fn get_volume_ratio(&self) -> f64 {
        self.volume_ratio.unwrap_or(0.0)
    }

    pub fn get_lc_min(&self) -> f64 {
        self.lc_min.unwrap_or(0.0)
    }

    pub fn get_l_max(&self) -> f64 {
        self.l_max.unwrap_or(0.0)
    }

    pub fn get_capacity(&self) -> f64 {
        self.capacity.unwrap_or(0.0)
    }

    pub fn get_vc_ratio(&self) -> f64 {
        self.vc_ratio.unwrap_or(0.0)
    }

    pub fn get_lc_all(&self) -> f64 {
        self.lc_all.unwrap_or(0.0)
    }

    pub fn get_speed_weaving(&self) -> f64 {
        self.speed_weaving.unwrap_or(0.0)
    }

    pub fn get_speed_nonweaving(&self) -> f64 {
        self.speed_nonweaving.unwrap_or(0.0)
    }

    pub fn get_speed_avg(&self) -> f64 {
        self.speed_avg.unwrap_or(0.0)
    }

    pub fn get_density(&self) -> f64 {
        self.density.unwrap_or(0.0)
    }

    pub fn get_los(&self) -> Option<LevelOfService> {
        self.los
    }

    pub fn is_weaving_segment(&self) -> bool {
        self.is_weaving.unwrap_or(false)
    }

    pub fn set_caf(&mut self, caf: f64) {
        self.caf = caf;
    }

    pub fn set_saf(&mut self, saf: f64) {
        self.saf = saf;
    }

    /// Number of weaving lanes N_WL: always 0 for two-sided segments.
    fn nwl(&self) -> u32 {
        match self.weaving_type {
            WeavingType::OneSided => self.num_weaving_lanes,
            WeavingType::TwoSided => 0,
        }
    }

    /// Heavy vehicle adjustment factor f_HV using PCEs from Exhibit 12-25.
    ///
    /// Shared by both editions: Edition 7.1 changed the weaving methodology, not the Chapter 12
    /// heavy-vehicle treatment it draws on.
    pub(crate) fn calculate_fhv(&self) -> f64 {
        let e_t = match self.terrain {
            TerrainType::Level => 2.0,    // Exhibit 12-25
            TerrainType::Rolling => 3.0,  // Exhibit 12-25
            // VERIFY-HCM: Exhibit 12-25 provides no PCE for mountainous
            // terrain (HCM directs to the Ch. 25/26 mixed-flow model);
            // 5.0 is a non-HCM approximation retained for API stability.
            TerrainType::Mountainous => 5.0,
        };
        1.0 / (1.0 + self.heavy_vehicle_pct * (e_t - 1.0))
    }

    // ── Step 2: Adjust volume ────────────────────────────────────────────

    /// Step 2: Convert component demand volumes to flow rates under
    /// equivalent ideal conditions (Equation 13-1) and aggregate weaving /
    /// nonweaving flows. Returns (v_W, v_NW, v) in pc/h.
    pub fn determine_demand_flow(&mut self) -> (f64, f64, f64) {
        let f_hv = self.calculate_fhv();
        self.f_hv = Some(f_hv);

        // Equation 13-1: v_i = V_i / (PHF x f_HV)
        let to_flow = |v: f64| v / (self.phf * f_hv);
        let v_ff = to_flow(self.v_ff);
        let v_fr = to_flow(self.v_fr);
        let v_rf = to_flow(self.v_rf);
        let v_rr = to_flow(self.v_rr);

        let (v_w, v_nw) = match self.weaving_type {
            // One-sided: weaving = ramp-to-freeway + freeway-to-ramp
            WeavingType::OneSided => (v_rf + v_fr, v_ff + v_rr),
            // Two-sided: only the ramp-to-ramp flow weaves
            WeavingType::TwoSided => (v_rr, v_ff + v_fr + v_rf),
        };
        let v = v_w + v_nw;

        self.flow_weaving = Some(v_w);
        self.flow_nonweaving = Some(v_nw);
        self.flow_total = Some(v);
        self.volume_ratio = Some(if v > 0.0 { v_w / v } else { 0.0 });

        (v_w, v_nw, v)
    }

    // ── Step 3: Configuration characteristics ───────────────────────────

    /// Step 3: Determine the minimum lane-changing rate LC_MIN (lc/h).
    /// One-sided: Equation 13-2, LC_MIN = LC_RF x v_RF + LC_FR x v_FR.
    /// Two-sided: Equation 13-3, LC_MIN = LC_RR x v_RR.
    pub fn determine_configuration_characteristics(&mut self) -> f64 {
        let f_hv = self.f_hv.unwrap_or_else(|| self.calculate_fhv());
        let to_flow = |v: f64| v / (self.phf * f_hv);

        let lc_min = match self.weaving_type {
            WeavingType::OneSided => {
                (self.lc_rf as f64) * to_flow(self.v_rf) + (self.lc_fr as f64) * to_flow(self.v_fr)
            }
            WeavingType::TwoSided => (self.lc_rr as f64) * to_flow(self.v_rr),
        };
        self.lc_min = Some(lc_min);
        lc_min
    }

    // ── Step 4: Maximum weaving length ──────────────────────────────────

    /// Step 4: Maximum weaving length (ft) - Equation 13-4:
    /// L_MAX = [5,728 x (1 + VR)^1.6] - (1,566 x N_WL)
    /// Sets `is_weaving` = (L_S < L_MAX).
    pub fn determine_max_weaving_length(&mut self) -> f64 {
        let vr = self.volume_ratio.unwrap_or(0.0);
        let l_max = 5728.0 * (1.0 + vr).powf(1.6) - 1566.0 * (self.nwl() as f64);
        self.l_max = Some(l_max);
        self.is_weaving = Some(self.length_short < l_max);
        l_max
    }

    // ── Step 5: Capacity ─────────────────────────────────────────────────

    /// Step 5: Weaving segment capacity under prevailing conditions (veh/h).
    ///
    /// Density criterion: Equation 13-5,
    ///   c_IWL = c_IFL - [438.2 x (1 + VR)^1.6] + (0.0765 x L_S) + (119.8 x N_WL),
    /// converted with Equation 13-6 (c_W = c_IWL x N x f_HV).
    /// Weaving-flow criterion: Equation 13-7 (c_IW = 2,400/VR for N_WL = 2;
    /// 3,500/VR for N_WL = 3; no limit for two-sided N_WL = 0), converted with
    /// Equation 13-8 (c_W = c_IW x f_HV).
    /// Final capacity is the smaller of the two, adjusted by CAF (Equation 13-9).
    /// Also computes v/c (Equation 13-10) and the demand_exceeds_capacity flag.
    pub fn determine_capacity(&mut self) -> f64 {
        let vr = self.volume_ratio.unwrap_or(0.0);
        let f_hv = self.f_hv.unwrap_or_else(|| self.calculate_fhv());
        let nwl = self.nwl();

        // Equation 13-5
        let c_iwl = self.basic_freeway_capacity - 438.2 * (1.0 + vr).powf(1.6)
            + 0.0765 * self.length_short
            + 119.8 * (nwl as f64);
        self.c_iwl = Some(c_iwl);

        // Equation 13-6
        let cw_density = c_iwl * (self.num_lanes as f64) * f_hv;
        self.capacity_density = Some(cw_density);

        // Equations 13-7 and 13-8
        let cw_weaving = if vr > 0.0 {
            match nwl {
                2 => Some(MAX_WEAVING_FLOW_NWL2 / vr * f_hv),
                3 => Some(MAX_WEAVING_FLOW_NWL3 / vr * f_hv),
                _ => None, // two-sided (N_WL = 0): no weaving-flow limit
            }
        } else {
            None
        };
        self.capacity_weaving = cw_weaving;

        // Final capacity = min of the two criteria, adjusted per Equation 13-9
        let c_w = match cw_weaving {
            Some(c) => cw_density.min(c),
            None => cw_density,
        };
        let c_wa = c_w * self.caf;
        self.capacity = Some(c_wa);

        // Equation 13-10: v/c = v x f_HV / c_wa
        let v = self.flow_total.unwrap_or(0.0);
        let vc = if c_wa > 0.0 { v * f_hv / c_wa } else { f64::INFINITY };
        self.vc_ratio = Some(vc);
        self.demand_exceeds_capacity = Some(vc > 1.0);

        c_wa
    }

    // ── Step 6: Lane-changing rates ──────────────────────────────────────

    /// Step 6: Total lane-changing rate LC_ALL (lc/h) - Equations 13-11..13-17.
    pub fn determine_lane_changing_rates(&mut self) -> f64 {
        let lc_min = self.lc_min.unwrap_or(0.0);
        let v_nw = self.flow_nonweaving.unwrap_or(0.0);
        let n = self.num_lanes as f64;
        let id = self.interchange_density;

        // Equation 13-11: LC_W = LC_MIN + 0.39 x [(L_S - 300)^0.5 x N^2 x (1 + ID)^0.8]
        // (300 ft is used for all lengths <= 300 ft.)
        let ls_adj = (self.length_short - MIN_WEAVING_LENGTH).max(0.0);
        let lc_w = lc_min + 0.39 * ls_adj.sqrt() * n.powi(2) * (1.0 + id).powf(0.8);
        self.lc_w = Some(lc_w);

        // Equation 13-12: I_NW = L_S x ID x v_NW / 10,000
        let i_nw = self.length_short * id * v_nw / 10_000.0;

        // Equation 13-13 (minimum externally set at 0)
        let lc_nw1 = (0.206 * v_nw + 0.542 * self.length_short - 192.6 * n).max(0.0);
        // Equation 13-14
        let lc_nw2 = 2135.0 + 0.223 * (v_nw - 2000.0);

        // Equation 13-16 (selection incl. Eq. 13-15 interpolation)
        let lc_nw = if lc_nw1 >= lc_nw2 {
            lc_nw2
        } else if i_nw <= 1300.0 {
            lc_nw1
        } else if i_nw >= 1950.0 {
            lc_nw2
        } else {
            // Equation 13-15
            lc_nw1 + (lc_nw2 - lc_nw1) * (i_nw - 1300.0) / 650.0
        };
        self.lc_nw = Some(lc_nw);

        // Equation 13-17
        let lc_all = lc_w + lc_nw;
        self.lc_all = Some(lc_all);
        lc_all
    }

    // ── Step 7: Speeds ───────────────────────────────────────────────────

    /// Step 7: Average speeds of weaving/nonweaving vehicles and the space
    /// mean speed of all vehicles (mi/h) - Equations 13-18 through 13-22.
    /// Returns (S_W, S_NW, S).
    pub fn estimate_speed(&mut self) -> (f64, f64, f64) {
        let lc_all = self.lc_all.unwrap_or(0.0);
        let lc_min = self.lc_min.unwrap_or(0.0);
        let v_w = self.flow_weaving.unwrap_or(0.0);
        let v_nw = self.flow_nonweaving.unwrap_or(0.0);
        let v = v_w + v_nw;
        let ffs_adj = self.ffs * self.saf;

        // Equation 13-20: W = 0.226 x (LC_ALL / L_S)^0.789
        let w = 0.226 * (lc_all / self.length_short).powf(0.789);
        self.weaving_intensity = Some(w);

        // Equation 13-19: S_W = 15 + (FFS x SAF - 15) / (1 + W)
        let s_w = MIN_WEAVING_SPEED + (ffs_adj - MIN_WEAVING_SPEED) / (1.0 + w);
        self.speed_weaving = Some(s_w);

        // Equation 13-21: S_NW = FFS x SAF - (0.0072 x LC_MIN) - (0.0048 x v/N)
        let s_nw = ffs_adj - 0.0072 * lc_min - 0.0048 * (v / self.num_lanes as f64);
        self.speed_nonweaving = Some(s_nw);

        // Equation 13-22 (space mean speed):
        // S = (v_W + v_NW) / [(v_W / S_W) + (v_NW / S_NW)]
        let s = if v > 0.0 && s_w > 0.0 && s_nw > 0.0 {
            v / (v_w / s_w + v_nw / s_nw)
        } else {
            s_w
        };
        self.speed_avg = Some(s);

        (s_w, s_nw, s)
    }

    // ── Step 8: Density and LOS ──────────────────────────────────────────

    /// Step 8a: Density (pc/mi/ln) - Equation 13-23: D = (v/N) / S.
    pub fn determine_density(&mut self) -> f64 {
        let v = self.flow_total.unwrap_or(0.0);
        let s = self.speed_avg.unwrap_or(0.0);
        let d = if s > 0.0 {
            (v / self.num_lanes as f64) / s
        } else {
            f64::INFINITY
        };
        self.density = Some(d);
        d
    }

    /// Step 8b: Level of service - Exhibit 13-6.
    pub fn determine_los(&mut self) -> LevelOfService {
        let d = self.density.unwrap_or(f64::INFINITY);
        let over = self.demand_exceeds_capacity.unwrap_or(false);
        let los = determine_weaving_los(d, over, self.facility_type);
        self.los = Some(los);
        los
    }

    /// Run the full Chapter 13 analysis for the segment's selected HCM edition and return the LOS.
    ///
    /// Under [`HcmVersion::V7`] this runs the 7th Edition Steps 2 through 8. Under
    /// [`HcmVersion::V7_1`] it runs the Edition 7.1 methodology
    /// ([`WeavingSegment::analyze_v7_1`]) and copies its results into the shared output fields.
    /// The two editions populate different subsets of those fields, because they compute different
    /// quantities: Edition 7.1 has no separate weaving and nonweaving speeds and no lane-changing
    /// rates, and reports capacity per lane in pc/h/ln rather than a whole-segment veh/h value.
    pub fn run_analysis(&mut self) -> LevelOfService {
        if self.version == HcmVersion::V7_1 {
            return self.run_analysis_v7_1();
        }
        self.determine_demand_flow();
        self.determine_configuration_characteristics();
        self.determine_max_weaving_length();
        self.determine_capacity();
        self.determine_lane_changing_rates();
        self.estimate_speed();
        self.determine_density();
        self.determine_los()
    }
}

// =============================================================================
// Managed-lane access segments: cross-weave capacity effect (Eqs. 13-24/13-25)
// =============================================================================

/// Effect of cross-weaving traffic on the capacity of the general purpose (GP)
/// lanes adjacent to a managed-lane (ML) access segment - HCM Equations 13-24
/// and 13-25.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct CrossWeaveEffect {
    /// Capacity reduction factor CRF (decimal) - Equation 13-24.
    pub crf: f64,
    /// Capacity adjustment factor CAF = 1 - CRF (decimal) - Equation 13-24.
    pub caf: f64,
    /// Adjusted GP-lane capacity c_GPA = c_GP x CAF (veh/h) - Equation 13-25.
    pub c_gpa: f64,
}

/// Cross-weave capacity effect on the general purpose lanes adjacent to an ML
/// access segment - HCM Chapter 13, Equations 13-24 and 13-25 (developed under
/// NCHRP Project 03-96).
///
/// ```text
/// CRF   = -0.0897 + 0.0252 ln(CW) - 0.00001453 L_cw_min + 0.002967 N_GP
/// CAF   = 1 - CRF
/// c_GPA = c_GP x CAF
/// ```
///
/// Arguments:
/// - `cw`: cross-weave demand flow rate (pc/h). Must be > 0 (the model takes
///   its natural logarithm); returns `None` otherwise.
/// - `l_cw_min`: cross-weave length L_cw-min (ft).
/// - `n_gp`: number of general purpose lanes.
/// - `c_gp`: unadjusted GP-lane capacity from Chapter 12 (veh/h).
///
/// The HCM states no numeric bounds on the regression. For a small `cw` the raw
/// CRF can fall at or below zero (implying CAF >= 1); the value is returned
/// unclamped so callers see the model's actual output, and results outside the
/// NCHRP 03-96 calibration range should be treated with caution.
pub fn cross_weave_gp_capacity(
    cw: f64,
    l_cw_min: f64,
    n_gp: u32,
    c_gp: f64,
) -> Option<CrossWeaveEffect> {
    if cw <= 0.0 {
        return None;
    }
    let crf = -0.0897 + 0.0252 * cw.ln() - 0.00001453 * l_cw_min + 0.002967 * (n_gp as f64);
    let caf = 1.0 - crf;
    Some(CrossWeaveEffect {
        crf,
        caf,
        c_gpa: c_gp * caf,
    })
}

// =============================================================================
// Service flow rates and service volumes (HCM Chapter 27, Example Problem 5)
// =============================================================================

/// Demand split of a weaving segment as fractions of the total flow rate v.
/// The four fractions are expected to sum to 1.0.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct DemandSplit {
    /// Freeway-to-freeway fraction of v.
    pub ff: f64,
    /// Ramp-to-freeway fraction of v.
    pub rf: f64,
    /// Freeway-to-ramp fraction of v.
    pub fr: f64,
    /// Ramp-to-ramp fraction of v.
    pub rr: f64,
}

/// Service flow rates and volumes for one (LOS, geometry) cell - HCM Chapter 27,
/// Example Problem 5.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ServiceVolumes {
    /// Service flow rate under ideal conditions SFI (pc/h).
    pub sfi: f64,
    /// Service flow rate under prevailing conditions SF = SFI x f_HV (veh/h).
    pub sf: f64,
    /// Service volume SV = SF x PHF (veh/h).
    pub sv: f64,
    /// Daily service volume DSV = SV / (K x D) (veh/day).
    pub dsv: f64,
}

/// Service flow rate under ideal conditions SFI (pc/h) for a target LOS density
/// - HCM Chapter 27, Example Problem 5.
///
/// Holds the segment's geometry (`template`) fixed and searches for the total
/// ideal flow rate v, apportioned by `split`, whose resulting weaving density
/// equals `target_density`. The search is done under equivalent ideal
/// conditions (PHF = 1, no heavy vehicles, CAF = SAF = 1), consistent with the
/// definition of SFI. Density rises monotonically with v below breakdown, so a
/// bisection converges. This is the SFI for LOS A through D (density thresholds
/// of 10/20/28/35 pc/mi/ln); the SFI at LOS E is the segment capacity, obtained
/// from [`WeavingSegment::determine_capacity`] instead.
pub fn service_flow_rate_ideal(
    template: &WeavingSegment,
    split: &DemandSplit,
    target_density: f64,
) -> f64 {
    // Density of an ideal-conditions probe carrying total flow v.
    let density_at = |v: f64| -> f64 {
        let mut seg = template.clone();
        seg.phf = 1.0;
        seg.heavy_vehicle_pct = 0.0;
        seg.caf = 1.0;
        seg.saf = 1.0;
        seg.v_ff = split.ff * v;
        seg.v_rf = split.rf * v;
        seg.v_fr = split.fr * v;
        seg.v_rr = split.rr * v;
        seg.run_analysis();
        seg.get_density()
    };

    // Bracket the target, then bisect. `hi` grows until its density overshoots.
    let mut lo = 0.0_f64;
    let mut hi = 1000.0_f64;
    let mut guard = 0;
    while density_at(hi) < target_density && guard < 60 {
        lo = hi;
        hi *= 2.0;
        guard += 1;
    }
    // 40 bisection steps take the bracket well below 1 pc/h wide.
    for _ in 0..40 {
        let mid = 0.5 * (lo + hi);
        if density_at(mid) < target_density {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    0.5 * (lo + hi)
}

/// Convert a service flow rate under ideal conditions (SFI, pc/h) into the
/// prevailing-condition service flow rate, service volume, and daily service
/// volume - HCM Chapter 27, Example Problem 5.
///
/// - `f_hv`: heavy-vehicle adjustment factor (SF = SFI x f_HV).
/// - `phf`: peak hour factor (SV = SF x PHF).
/// - `k`: K-factor, proportion of AADT in the analysis hour.
/// - `d`: D-factor, directional proportion (DSV = SV / (K x D)).
pub fn service_volumes(sfi: f64, f_hv: f64, phf: f64, k: f64, d: f64) -> ServiceVolumes {
    let sf = sfi * f_hv;
    let sv = sf * phf;
    let dsv = if k > 0.0 && d > 0.0 {
        sv / (k * d)
    } else {
        f64::INFINITY
    };
    ServiceVolumes { sfi, sf, sv, dsv }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_los_criteria_freeway() {
        assert_eq!(determine_weaving_los(8.0, false, FacilityType::Freeway), LevelOfService::A);
        assert_eq!(determine_weaving_los(15.0, false, FacilityType::Freeway), LevelOfService::B);
        assert_eq!(determine_weaving_los(25.0, false, FacilityType::Freeway), LevelOfService::C);
        assert_eq!(determine_weaving_los(32.0, false, FacilityType::Freeway), LevelOfService::D);
        assert_eq!(determine_weaving_los(40.0, false, FacilityType::Freeway), LevelOfService::E);
        assert_eq!(determine_weaving_los(45.0, false, FacilityType::Freeway), LevelOfService::F);
        assert_eq!(determine_weaving_los(25.0, true, FacilityType::Freeway), LevelOfService::F);
    }

    #[test]
    fn test_los_criteria_multilane() {
        assert_eq!(determine_weaving_los(10.0, false, FacilityType::MultilaneOrCD), LevelOfService::A);
        assert_eq!(determine_weaving_los(20.0, false, FacilityType::MultilaneOrCD), LevelOfService::B);
        assert_eq!(determine_weaving_los(30.0, false, FacilityType::MultilaneOrCD), LevelOfService::C);
        assert_eq!(determine_weaving_los(35.0, false, FacilityType::MultilaneOrCD), LevelOfService::D);
        assert_eq!(determine_weaving_los(38.0, false, FacilityType::MultilaneOrCD), LevelOfService::E);
        assert_eq!(determine_weaving_los(42.0, false, FacilityType::MultilaneOrCD), LevelOfService::F);
    }

    #[test]
    fn test_heavy_vehicle_adjustment() {
        // Level terrain (E_T = 2.0) with 5% trucks -> 0.952
        let mut seg = WeavingSegment { heavy_vehicle_pct: 0.05, terrain: TerrainType::Level, ..Default::default() };
        seg.determine_demand_flow();
        assert!((seg.f_hv.unwrap() - 0.9524).abs() < 0.01);

        // Rolling terrain (E_T = 3.0) with 10% trucks -> 0.833
        let mut seg = WeavingSegment { heavy_vehicle_pct: 0.10, terrain: TerrainType::Rolling, ..Default::default() };
        seg.determine_demand_flow();
        assert!((seg.f_hv.unwrap() - 0.8333).abs() < 0.01);
    }

    #[test]
    fn test_lc_min_one_sided() {
        // LC_RF = 1, v_RF = 500, LC_FR = 1, v_FR = 400 (PHF = 1, no HV)
        let mut seg = WeavingSegment {
            weaving_type: WeavingType::OneSided,
            v_rf: 500.0,
            v_fr: 400.0,
            lc_rf: 1,
            lc_fr: 1,
            phf: 1.0,
            heavy_vehicle_pct: 0.0,
            ..Default::default()
        };
        seg.determine_demand_flow();
        let lc_min = seg.determine_configuration_characteristics();
        assert!((lc_min - 900.0).abs() < 1e-9);
    }

    #[test]
    fn test_lc_min_two_sided() {
        // LC_RR = 2, v_RR = 200 (PHF = 1, no HV)
        let mut seg = WeavingSegment {
            weaving_type: WeavingType::TwoSided,
            v_rr: 200.0,
            lc_rr: 2,
            phf: 1.0,
            heavy_vehicle_pct: 0.0,
            ..Default::default()
        };
        seg.determine_demand_flow();
        let lc_min = seg.determine_configuration_characteristics();
        assert!((lc_min - 400.0).abs() < 1e-9);
    }

    #[test]
    fn test_max_weaving_length() {
        // Equation 13-4 spot check against Exhibit 13-11:
        // VR = 0.3, N_WL = 2 -> 5,584 ft; VR = 0.3, N_WL = 3 -> 4,018 ft
        let l_max_2 = 5728.0 * (1.0f64 + 0.3).powf(1.6) - 1566.0 * 2.0;
        let l_max_3 = 5728.0 * (1.0f64 + 0.3).powf(1.6) - 1566.0 * 3.0;
        assert!((l_max_2 - 5584.0).abs() < 5.0);
        assert!((l_max_3 - 4018.0).abs() < 5.0);
    }

    #[test]
    fn test_weaving_analysis_one_sided() {
        let mut seg = WeavingSegment {
            weaving_type: WeavingType::OneSided,
            facility_type: FacilityType::Freeway,
            length_short: 1500.0,
            num_lanes: 4,
            num_weaving_lanes: 2,
            ffs: 70.0,
            v_ff: 3000.0,
            v_fr: 500.0,
            v_rf: 600.0,
            v_rr: 100.0,
            phf: 0.94,
            heavy_vehicle_pct: 0.05,
            terrain: TerrainType::Level,
            lc_rf: 1,
            lc_fr: 1,
            lc_rr: 0,
            interchange_density: 0.8,
            basic_freeway_capacity: 2400.0,
            ..Default::default()
        };

        seg.run_analysis();

        assert!(seg.get_flow_total() > 0.0);
        assert!(seg.get_flow_weaving() > 0.0);
        assert!(seg.get_flow_nonweaving() > 0.0);
        assert!(seg.get_volume_ratio() > 0.0 && seg.get_volume_ratio() < 1.0);
        assert!(seg.get_speed_avg() > 0.0 && seg.get_speed_avg() <= seg.ffs);
        assert!(seg.get_density() > 0.0);
        assert!(seg.get_capacity() > 0.0);
    }

    #[test]
    fn test_two_sided_weaving() {
        let mut seg = WeavingSegment {
            weaving_type: WeavingType::TwoSided,
            facility_type: FacilityType::Freeway,
            length_short: 1200.0,
            num_lanes: 4,
            num_weaving_lanes: 0,
            ffs: 65.0,
            v_ff: 3500.0,
            v_fr: 400.0,
            v_rf: 500.0,
            v_rr: 300.0,
            phf: 0.92,
            heavy_vehicle_pct: 0.08,
            terrain: TerrainType::Level,
            lc_rf: 0,
            lc_fr: 0,
            lc_rr: 2,
            interchange_density: 0.6,
            basic_freeway_capacity: 2350.0,
            ..Default::default()
        };

        seg.run_analysis();

        // For two-sided, weaving flow is ramp-to-ramp only
        assert!(seg.get_flow_weaving() < seg.get_flow_nonweaving());
        assert!(seg.get_density() > 0.0);
        // Two-sided segments have no weaving-flow capacity limit
        assert!(seg.capacity_weaving.is_none());
    }

    #[test]
    fn test_cross_weave_gp_capacity() {
        // HCM Chapter 27, Example Problem 6: CW = 400, L_cw-min = 1,000, N_GP = 3.
        let e = cross_weave_gp_capacity(400.0, 1000.0, 3, 7050.0).unwrap();
        assert!((e.crf - 0.056).abs() < 0.001);
        assert!((e.caf - (1.0 - e.crf)).abs() < 1e-12);

        // Example Problem 7: CW = 100, L_cw-min = 1,500, N_GP = 2, c_GP = 4,800.
        let e = cross_weave_gp_capacity(100.0, 1500.0, 2, 4800.0).unwrap();
        assert!((e.crf - 0.0105).abs() < 0.0005);
        assert!((e.c_gpa - 4750.0).abs() < 5.0);

        // ln(CW) is undefined at or below zero demand.
        assert!(cross_weave_gp_capacity(0.0, 1000.0, 2, 4800.0).is_none());
    }

    #[test]
    fn test_service_flow_rate_monotone() {
        let template = WeavingSegment {
            num_lanes: 3,
            num_weaving_lanes: 2,
            ffs: 65.0,
            interchange_density: 1.0,
            lc_rf: 0,
            lc_fr: 2,
            length_short: 1500.0,
            basic_freeway_capacity: 2350.0,
            ..Default::default()
        };
        let split = DemandSplit { ff: 0.65, rf: 0.15, fr: 0.12, rr: 0.08 };
        // Higher LOS thresholds admit strictly more flow.
        let sfi_a = service_flow_rate_ideal(&template, &split, 10.0);
        let sfi_c = service_flow_rate_ideal(&template, &split, 28.0);
        assert!(sfi_c > sfi_a);
        // The chain is a straight product; verify the arithmetic.
        let sv = service_volumes(sfi_c, 0.952, 0.93, 0.08, 0.55);
        assert!((sv.sf - sfi_c * 0.952).abs() < 1e-9);
        assert!((sv.sv - sv.sf * 0.93).abs() < 1e-9);
        assert!((sv.dsv - sv.sv / (0.08 * 0.55)).abs() < 1e-6);
    }
}
