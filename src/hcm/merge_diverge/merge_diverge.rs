//! HCM Chapter 14: Freeway Merge and Diverge Segments.
//!
//! Implements the HCM 7th Edition ramp-freeway junction methodology:
//! demand adjustment (Eq. 14-1), lane-distribution models P_FM/P_FD
//! (Exhibits 14-8/14-9, Eqs. 14-2 through 14-13), reasonableness checks
//! (Eqs. 14-14 through 14-19), capacity (Exhibits 14-10/14-12, Eqs. 14-20/14-21),
//! density (Eqs. 14-22/14-23/14-28), LOS (Exhibit 14-3), and speeds
//! (Exhibits 14-13/14-14/14-15, Eq. 14-24). Special cases covered: two-lane
//! ramps (Eqs. 14-25/14-26), left-hand ramps (Exhibit 14-18), 10-lane
//! freeways (Exhibit 14-19, Eq. 14-27), and major merge/diverge areas
//! (Eq. 14-28).

use serde::{Deserialize, Serialize};
use crate::hcm::common::LevelOfService;

// =============================================================================
// Constants
// =============================================================================

/// Length of a ramp influence area (ft) - Exhibit 14-1.
pub const RAMP_INFLUENCE_AREA_LENGTH: f64 = 1500.0;

/// Maximum reasonable average flow per outer lane (pc/h/ln) - Chapter 14,
/// reasonableness check on the lane-distribution prediction.
pub const MAX_OUTER_LANE_FLOW: f64 = 2700.0;

/// Maximum desirable flow rate v_R12 entering a merge influence area (pc/h)
/// - Exhibit 14-10.
pub const MAX_MERGE_INFLUENCE_FLOW: f64 = 4600.0;

/// Maximum desirable flow rate v_12 entering a diverge influence area (pc/h)
/// - Exhibit 14-10.
pub const MAX_DIVERGE_INFLUENCE_FLOW: f64 = 4400.0;

// =============================================================================
// Enums
// =============================================================================

/// Type of ramp junction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RampType {
    /// On-ramp (merge area)
    OnRamp,
    /// Off-ramp (diverge area)
    OffRamp,
    /// Major merge (two multilane facilities joining) - capacity checks only
    MajorMerge,
    /// Major diverge (facility splitting into two) - Equation 14-28
    MajorDiverge,
}

/// Side of freeway where the ramp is located
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RampSide {
    Right,
    Left,
}

/// Number of lanes on the ramp
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RampLanes {
    OneLane,
    TwoLane,
}

/// Adjacent ramp configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AdjacentRampType {
    None,
    OnRamp,
    OffRamp,
}

/// Terrain type for PCE selection (Exhibit 12-25)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TerrainType {
    Level,
    Rolling,
    Mountainous,
}

/// LOS criteria for freeway merge and diverge segments - Exhibit 14-3.
/// A <=10, B <=20, C <=28, D <=35, E >35; F only when demand exceeds capacity.
pub fn determine_ramp_los(density: f64, demand_exceeds_capacity: bool) -> LevelOfService {
    if demand_exceeds_capacity {
        return LevelOfService::F;
    }

    match density {
        d if d <= 10.0 => LevelOfService::A,
        d if d <= 20.0 => LevelOfService::B,
        d if d <= 28.0 => LevelOfService::C,
        d if d <= 35.0 => LevelOfService::D,
        _ => LevelOfService::E,
    }
}

// =============================================================================
// Capacity tables
// =============================================================================

/// Freeway capacity per lane by FFS (pc/h/ln) - Exhibit 14-10
/// (>=70: 2,400; 65: 2,350; 60: 2,300; 55: 2,250).
pub fn get_freeway_capacity_per_lane(ffs: f64) -> f64 {
    if ffs >= 70.0 {
        2400.0
    } else if ffs >= 65.0 {
        2350.0
    } else if ffs >= 60.0 {
        2300.0
    } else {
        2250.0
    }
}

/// Total directional freeway capacity by FFS and lane count - Exhibit 14-10.
pub fn get_freeway_capacity(ffs: f64, lanes: u32) -> f64 {
    get_freeway_capacity_per_lane(ffs) * (lanes as f64)
}

/// Ramp roadway capacity (pc/h) - Exhibit 14-12.
/// Single-lane: >50: 2,200; >40-50: 2,100; >30-40: 2,000; 20-30: 1,900; <20: 1,800.
/// Two-lane ramps are double the single-lane values.
pub fn get_ramp_capacity(ramp_ffs: f64, two_lane: bool) -> f64 {
    let single_lane_cap = if ramp_ffs > 50.0 {
        2200.0
    } else if ramp_ffs > 40.0 {
        2100.0
    } else if ramp_ffs > 30.0 {
        2000.0
    } else if ramp_ffs >= 20.0 {
        1900.0
    } else {
        1800.0
    };

    if two_lane {
        single_lane_cap * 2.0
    } else {
        single_lane_cap
    }
}

// =============================================================================
// Special-case helpers
// =============================================================================

/// P_FM for two-lane on-ramps (Chapter 14, Special Cases):
/// 4-lane: 1.000; 6-lane: 0.555; 8-lane: 0.209.
pub fn pfm_two_lane_onramp(freeway_lanes: u32) -> f64 {
    match freeway_lanes {
        2 => 1.000,
        3 => 0.555,
        _ => 0.209,
    }
}

/// P_FD for two-lane off-ramps (Chapter 14, Special Cases):
/// 4-lane: 1.000; 6-lane: 0.450; 8-lane: 0.260.
pub fn pfd_two_lane_offramp(freeway_lanes: u32) -> f64 {
    match freeway_lanes {
        2 => 1.000,
        3 => 0.450,
        _ => 0.260,
    }
}

/// Expected flow in Lane 5 of a 10-lane freeway (pc/h) - Exhibit 14-19.
pub fn get_lane5_flow(v_f: f64, is_on_ramp: bool) -> f64 {
    if is_on_ramp {
        if v_f >= 8500.0 {
            2500.0
        } else if v_f >= 7500.0 {
            0.285 * v_f
        } else if v_f >= 6500.0 {
            0.270 * v_f
        } else if v_f >= 5500.0 {
            0.240 * v_f
        } else {
            0.220 * v_f
        }
    } else if v_f >= 7000.0 {
        0.200 * v_f
    } else if v_f >= 5500.0 {
        0.150 * v_f
    } else if v_f >= 4000.0 {
        0.100 * v_f
    } else {
        0.0
    }
}

/// Effective acceleration lane length for two-lane on-ramps (ft) - Equation 14-25:
/// L_Aeff = 2 x L_A1 + L_A2.
/// VERIFY-HCM: HCM caps acceleration lane lengths used for calculation at
/// 1,500 ft (Exhibit 14-16 discussion); this helper caps the effective total.
pub fn effective_accel_length(l_a1: f64, l_a2: f64) -> f64 {
    (2.0 * l_a1 + l_a2).min(RAMP_INFLUENCE_AREA_LENGTH)
}

/// Effective deceleration lane length for two-lane off-ramps (ft) - Equation 14-26:
/// L_Deff = 2 x L_D1 + L_D2 (only when two deceleration lanes exist).
/// VERIFY-HCM: capped at 1,500 ft as for the acceleration case.
pub fn effective_decel_length(l_d1: f64, l_d2: f64) -> f64 {
    (2.0 * l_d1 + l_d2).min(RAMP_INFLUENCE_AREA_LENGTH)
}

/// Adjustment factors for left-hand ramp-freeway junctions - Exhibit 14-18.
/// Applied to v_12 computed as if the ramp were on the right side.
/// On-ramps: 4-lane 1.00, 6-lane 1.12, 8-lane 1.20.
/// Off-ramps: 4-lane 1.00, 6-lane 1.05, 8-lane 1.10.
pub fn left_hand_adjustment(freeway_lanes: u32, is_on_ramp: bool) -> f64 {
    match (freeway_lanes, is_on_ramp) {
        (2, _) => 1.00,
        (3, true) => 1.12,
        (3, false) => 1.05,
        (4, true) => 1.20,
        (4, false) => 1.10,
        _ => 1.0,
    }
}

// =============================================================================
// RampSegment
// =============================================================================

/// A ramp-freeway junction analyzed with the HCM Chapter 14 methodology.
///
/// Input fields are plain values; computed fields are `Option<T>` and are
/// populated by the step methods (in HCM step order) or by `run_analysis`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct RampSegment {
    // ── Inputs ──────────────────────────────────────────────────────────
    /// Type of ramp junction
    pub ramp_type: RampType,
    /// Side of the freeway (right or left)
    pub ramp_side: RampSide,
    /// Number of lanes on the ramp
    pub ramp_lanes: RampLanes,
    /// Number of freeway lanes in one direction (2-5), ln
    pub freeway_lanes: u32,
    /// Freeway free-flow speed FFS, mi/h
    pub freeway_ffs: f64,
    /// Ramp free-flow speed S_FR, mi/h
    pub ramp_ffs: f64,
    /// Length of (first) acceleration lane L_A, ft - on-ramps
    pub accel_lane_length: Option<f64>,
    /// Length of second acceleration lane L_A2, ft - two-lane on-ramps (Eq. 14-25)
    pub accel_lane_length2: Option<f64>,
    /// Length of (first) deceleration lane L_D, ft - off-ramps
    pub decel_lane_length: Option<f64>,
    /// Length of second deceleration lane L_D2, ft - two-lane off-ramps (Eq. 14-26)
    pub decel_lane_length2: Option<f64>,
    /// Freeway demand volume immediately upstream of the ramp V_F, veh/h
    pub freeway_demand: f64,
    /// Ramp demand volume V_R, veh/h
    pub ramp_demand: f64,
    /// Peak hour factor, decimal
    pub phf: f64,
    /// Heavy vehicle proportion on the freeway, decimal (e.g., 0.05 for 5%)
    pub heavy_vehicle_pct: f64,
    /// Heavy vehicle proportion on the ramp, decimal (defaults to freeway value)
    pub ramp_heavy_vehicle_pct: Option<f64>,
    /// Terrain type for PCE selection (Exhibit 12-25)
    pub terrain: TerrainType,
    /// Adjacent upstream ramp type
    pub adjacent_upstream: AdjacentRampType,
    /// Distance to adjacent upstream ramp L_UP, ft
    pub upstream_distance: Option<f64>,
    /// Demand volume on adjacent upstream ramp V_U, veh/h
    pub upstream_ramp_flow: Option<f64>,
    /// Adjacent downstream ramp type
    pub adjacent_downstream: AdjacentRampType,
    /// Distance to adjacent downstream ramp L_DOWN, ft
    pub downstream_distance: Option<f64>,
    /// Demand volume on adjacent downstream ramp V_D, veh/h
    pub downstream_ramp_flow: Option<f64>,
    /// Capacity adjustment factor CAF, decimal - Equation 14-21
    pub caf: f64,
    /// Speed adjustment factor SAF, decimal - Exhibits 14-13/14-14
    pub saf: f64,

    // ── Computed (populated by step methods) ────────────────────────────
    /// Freeway demand flow rate v_F, pc/h - Equation 14-1
    pub flow_freeway: Option<f64>,
    /// Ramp demand flow rate v_R, pc/h - Equation 14-1
    pub flow_ramp: Option<f64>,
    /// Proportion of freeway traffic in Lanes 1-2, P_FM or P_FD, decimal
    pub p_f: Option<f64>,
    /// Flow rate in Lanes 1 and 2 v_12, pc/h - Equations 14-2/14-8
    /// (for left-hand ramps this is the two leftmost lanes, v_23/v_34)
    pub v_12: Option<f64>,
    /// Total flow entering the ramp influence area, pc/h
    /// (merge: v_R12 = v_12 + v_R per Eq. 14-20; diverge: v_12)
    pub v_r12: Option<f64>,
    /// Average demand flow per outer lane v_OA, pc/h/ln - Exhibit 14-15
    pub v_oa: Option<f64>,
    /// Adjusted freeway capacity, pc/h - Exhibit 14-10 x CAF (Eq. 14-21)
    pub capacity_freeway: Option<f64>,
    /// Ramp roadway capacity, pc/h - Exhibit 14-12
    pub capacity_ramp: Option<f64>,
    /// Demand-to-capacity ratio at the critical freeway checkpoint
    pub vc_ratio: Option<f64>,
    /// Whether demand exceeds any capacity checkpoint (-> LOS F)
    pub demand_exceeds_capacity: Option<bool>,
    /// Whether flow entering the influence area exceeds the maximum
    /// desirable value of Exhibit 14-10 (does not by itself set LOS F)
    pub exceeds_max_desirable: Option<bool>,
    /// Density in the ramp influence area D_R, pc/mi/ln - Eqs. 14-22/14-23/14-28
    pub density: Option<f64>,
    /// Speed within the ramp influence area S_R, mi/h - Exhibits 14-13/14-14
    pub speed_ramp: Option<f64>,
    /// Average speed in outer lanes S_O, mi/h (None for 4-lane freeways)
    pub speed_outer: Option<f64>,
    /// Average speed of all vehicles S, mi/h - Exhibit 14-15
    pub speed_avg: Option<f64>,
    /// Aggregate density across all lanes, pc/mi/ln - Equation 14-24
    pub density_all_lanes: Option<f64>,
    /// Level of service - Exhibit 14-3
    pub los: Option<LevelOfService>,
}

impl Default for RampSegment {
    fn default() -> Self {
        Self {
            ramp_type: RampType::OnRamp,
            ramp_side: RampSide::Right,
            ramp_lanes: RampLanes::OneLane,
            freeway_lanes: 3,
            freeway_ffs: 70.0,
            ramp_ffs: 35.0,
            accel_lane_length: Some(800.0),
            accel_lane_length2: None,
            decel_lane_length: Some(400.0),
            decel_lane_length2: None,
            freeway_demand: 4000.0,
            ramp_demand: 500.0,
            phf: 0.94,
            heavy_vehicle_pct: 0.05,
            ramp_heavy_vehicle_pct: None,
            terrain: TerrainType::Level,
            adjacent_upstream: AdjacentRampType::None,
            upstream_distance: None,
            upstream_ramp_flow: None,
            adjacent_downstream: AdjacentRampType::None,
            downstream_distance: None,
            downstream_ramp_flow: None,
            caf: 1.0,
            saf: 1.0,
            flow_freeway: None,
            flow_ramp: None,
            p_f: None,
            v_12: None,
            v_r12: None,
            v_oa: None,
            capacity_freeway: None,
            capacity_ramp: None,
            vc_ratio: None,
            demand_exceeds_capacity: None,
            exceeds_max_desirable: None,
            density: None,
            speed_ramp: None,
            speed_outer: None,
            speed_avg: None,
            density_all_lanes: None,
            los: None,
        }
    }
}

impl RampSegment {
    pub fn new() -> Self {
        Self::default()
    }

    // ── Accessors ─────────────────────────────────────────────────────────

    pub fn get_flow_freeway(&self) -> f64 {
        self.flow_freeway.unwrap_or(0.0)
    }

    pub fn get_flow_ramp(&self) -> f64 {
        self.flow_ramp.unwrap_or(0.0)
    }

    pub fn get_v12(&self) -> f64 {
        self.v_12.unwrap_or(0.0)
    }

    pub fn get_vr12(&self) -> f64 {
        self.v_r12.unwrap_or(0.0)
    }

    pub fn get_capacity_freeway(&self) -> f64 {
        self.capacity_freeway.unwrap_or(0.0)
    }

    pub fn get_capacity_ramp(&self) -> f64 {
        self.capacity_ramp.unwrap_or(0.0)
    }

    pub fn get_vc_ratio(&self) -> f64 {
        self.vc_ratio.unwrap_or(0.0)
    }

    pub fn get_density(&self) -> f64 {
        self.density.unwrap_or(0.0)
    }

    pub fn get_speed_ramp(&self) -> f64 {
        self.speed_ramp.unwrap_or(0.0)
    }

    pub fn get_speed_outer(&self) -> Option<f64> {
        self.speed_outer
    }

    pub fn get_speed_avg(&self) -> f64 {
        self.speed_avg.unwrap_or(0.0)
    }

    pub fn get_los(&self) -> Option<LevelOfService> {
        self.los
    }

    pub fn set_caf(&mut self, caf: f64) {
        self.caf = caf;
    }

    pub fn set_saf(&mut self, saf: f64) {
        self.saf = saf;
    }

    fn is_on_ramp(&self) -> bool {
        matches!(self.ramp_type, RampType::OnRamp | RampType::MajorMerge)
    }

    /// Heavy vehicle adjustment factor for a given HV proportion (Eq. 12-10),
    /// using PCEs from Exhibit 12-25.
    fn fhv_for(&self, pct: f64) -> f64 {
        let e_t = match self.terrain {
            TerrainType::Level => 2.0,    // Exhibit 12-25
            TerrainType::Rolling => 3.0,  // Exhibit 12-25
            // VERIFY-HCM: Exhibit 12-25 provides no PCE for mountainous
            // terrain (HCM directs to the Ch. 25/26 mixed-flow model);
            // 5.0 is a non-HCM approximation retained for API stability.
            TerrainType::Mountainous => 5.0,
        };
        1.0 / (1.0 + pct * (e_t - 1.0))
    }

    /// Convert an adjacent-ramp demand volume (veh/h) to pc/h using the
    /// freeway heavy-vehicle percentage (Eq. 14-1).
    fn adjacent_flow_pc(&self, volume: f64) -> f64 {
        volume / (self.phf * self.fhv_for(self.heavy_vehicle_pct))
    }

    /// Acceleration lane length used in the equations (ft): the effective
    /// two-lane value (Eq. 14-25) when a second acceleration lane is given.
    fn effective_la(&self) -> f64 {
        let l_a1 = self.accel_lane_length.unwrap_or(800.0);
        match (self.ramp_lanes, self.accel_lane_length2) {
            (RampLanes::TwoLane, Some(l_a2)) => effective_accel_length(l_a1, l_a2),
            _ => l_a1,
        }
    }

    /// Deceleration lane length used in the equations (ft): the effective
    /// two-lane value (Eq. 14-26) when a second deceleration lane is given.
    fn effective_ld(&self) -> f64 {
        let l_d1 = self.decel_lane_length.unwrap_or(400.0);
        match (self.ramp_lanes, self.decel_lane_length2) {
            (RampLanes::TwoLane, Some(l_d2)) => effective_decel_length(l_d1, l_d2),
            _ => l_d1,
        }
    }

    // ── Step 1: Demand flow rates ────────────────────────────────────────

    /// Step 1: Convert freeway and ramp demand volumes to flow rates in
    /// pc/h under equivalent ideal conditions (Equation 14-1).
    /// Returns (v_F, v_R).
    pub fn determine_demand_flow(&mut self) -> (f64, f64) {
        let f_hv_freeway = self.fhv_for(self.heavy_vehicle_pct);
        let f_hv_ramp = self.fhv_for(self.ramp_heavy_vehicle_pct.unwrap_or(self.heavy_vehicle_pct));

        let mut v_f = self.freeway_demand / (self.phf * f_hv_freeway);
        let v_r = self.ramp_demand / (self.phf * f_hv_ramp);

        // 10-lane freeways: deduct the Lane 5 flow (Exhibit 14-19,
        // Equation 14-27) and analyze as an 8-lane freeway.
        if self.freeway_lanes >= 5 {
            let v_5 = get_lane5_flow(v_f, self.is_on_ramp());
            v_f -= v_5;
        }

        self.flow_freeway = Some(v_f);
        self.flow_ramp = Some(v_r);
        (v_f, v_r)
    }

    // ── Step 2: Flow in Lanes 1 and 2 ────────────────────────────────────

    /// P_FM selection for one-lane, right-side on-ramps - Exhibit 14-8.
    fn calculate_pfm(&self, v_f: f64, v_r: f64) -> f64 {
        let l_a = self.effective_la();
        // Effective lane count (10-lane freeways handled as 8-lane)
        let lanes = self.freeway_lanes.min(4);

        match lanes {
            2 => 1.0, // 4-lane freeway: all vehicles in Lanes 1 and 2
            3 => {
                // Equation 14-3 (base case): P_FM = 0.5775 + 0.000028 L_A
                let pfm_base = 0.5775 + 0.000028 * l_a;

                // Adjacent-ramp equations only apply to one-lane, right-side
                // adjacent off-ramps on six-lane freeways (Exhibit 14-8 notes).
                let mut candidates: Vec<f64> = Vec::new();

                // Upstream adjacent off-ramp: Equation 14-4 when L_UP < L_EQ
                if let (AdjacentRampType::OffRamp, Some(l_up)) =
                    (self.adjacent_upstream, self.upstream_distance)
                {
                    // Equation 14-6:
                    // L_EQ = 0.214(v_F + v_R) + 0.444 L_A + 52.32 S_FR - 2,403
                    let l_eq =
                        0.214 * (v_f + v_r) + 0.444 * l_a + 52.32 * self.ramp_ffs - 2403.0;
                    if l_up < l_eq {
                        // Equation 14-4:
                        // P_FM = 0.7289 - 0.0000135(v_F + v_R) - 0.003296 S_FR + 0.000063 L_UP
                        candidates.push(
                            0.7289 - 0.0000135 * (v_f + v_r) - 0.003296 * self.ramp_ffs
                                + 0.000063 * l_up,
                        );
                    }
                }

                // Downstream adjacent off-ramp: Equation 14-5 when L_DOWN < L_EQ
                if let (AdjacentRampType::OffRamp, Some(l_down), Some(v_d)) = (
                    self.adjacent_downstream,
                    self.downstream_distance,
                    self.downstream_ramp_flow,
                ) {
                    let v_d_pc = self.adjacent_flow_pc(v_d);
                    // Equation 14-7: L_EQ = v_D / (0.1096 + 0.000107 L_A)
                    let l_eq = v_d_pc / (0.1096 + 0.000107 * l_a);
                    if l_down < l_eq {
                        // Equation 14-5: P_FM = 0.5487 + 0.2628 (v_D / L_DOWN)
                        candidates.push(0.5487 + 0.2628 * (v_d_pc / l_down));
                    }
                }

                // When both adjacent off-ramps apply, the larger P_FM governs
                // (Chapter 14, Exhibit 14-8 discussion).
                candidates
                    .into_iter()
                    .fold(None, |acc: Option<f64>, c| {
                        Some(acc.map_or(c, |cur| cur.max(c)))
                    })
                    .unwrap_or(pfm_base)
            }
            _ => {
                // 8-lane freeway (Exhibit 14-8):
                // v_F/S_FR <= 72: P_FM = 0.2178 - 0.000125 v_R + 0.01115 (L_A / S_FR)
                // v_F/S_FR  > 72: P_FM = 0.2178 - 0.000125 v_R
                if v_f / self.ramp_ffs <= 72.0 {
                    0.2178 - 0.000125 * v_r + 0.01115 * (l_a / self.ramp_ffs)
                } else {
                    0.2178 - 0.000125 * v_r
                }
            }
        }
    }

    /// P_FD selection for one-lane, right-side off-ramps - Exhibit 14-9.
    fn calculate_pfd(&self, v_f: f64, v_r: f64) -> f64 {
        let lanes = self.freeway_lanes.min(4);

        match lanes {
            2 => 1.0,
            3 => {
                // Equation 14-9 (base case):
                // P_FD = 0.760 - 0.000025 v_F - 0.000046 v_R
                let pfd_base = 0.760 - 0.000025 * v_f - 0.000046 * v_r;

                let mut candidates: Vec<f64> = Vec::new();

                // Upstream adjacent on-ramp: Equation 14-10 when L_UP < L_EQ
                // and v_U/L_UP <= 0.2 (otherwise Equation 14-9).
                if let (AdjacentRampType::OnRamp, Some(l_up), Some(v_u)) = (
                    self.adjacent_upstream,
                    self.upstream_distance,
                    self.upstream_ramp_flow,
                ) {
                    let v_u_pc = self.adjacent_flow_pc(v_u);
                    if v_u_pc / l_up <= 0.20 {
                        // Equation 14-12:
                        // L_EQ = v_U / (0.071 + 0.000023 v_F - 0.000076 v_R)
                        let denom = 0.071 + 0.000023 * v_f - 0.000076 * v_r;
                        if denom > 0.0 && l_up < v_u_pc / denom {
                            // Equation 14-10:
                            // P_FD = 0.717 - 0.000039 v_F + 0.604 (v_U / L_UP)
                            candidates.push(0.717 - 0.000039 * v_f + 0.604 * (v_u_pc / l_up));
                        }
                    }
                }

                // Downstream adjacent off-ramp: Equation 14-11 when L_DOWN < L_EQ.
                if let (AdjacentRampType::OffRamp, Some(l_down), Some(v_d)) = (
                    self.adjacent_downstream,
                    self.downstream_distance,
                    self.downstream_ramp_flow,
                ) {
                    let v_d_pc = self.adjacent_flow_pc(v_d);
                    // Equation 14-13:
                    // L_EQ = v_D / (1.15 - 0.000032 v_F - 0.000369 v_R)
                    let denom = 1.15 - 0.000032 * v_f - 0.000369 * v_r;
                    if denom > 0.0 && l_down < v_d_pc / denom {
                        // Equation 14-11:
                        // P_FD = 0.616 - 0.000021 v_F + 0.124 (v_D / L_DOWN)
                        candidates.push(0.616 - 0.000021 * v_f + 0.124 * (v_d_pc / l_down));
                    }
                }

                // When both adjacent ramps apply, the larger P_FD governs.
                candidates
                    .into_iter()
                    .fold(None, |acc: Option<f64>, c| {
                        Some(acc.map_or(c, |cur| cur.max(c)))
                    })
                    .unwrap_or(pfd_base)
            }
            // 8-lane freeway: P_FD = 0.436 (constant) - Exhibit 14-9
            _ => 0.436,
        }
    }

    /// Reasonableness checks on v_12 - Equations 14-14 through 14-19.
    /// When both limits are violated, the larger adjusted value governs.
    fn check_v12(&self, v_f: f64, v_12: f64) -> f64 {
        // Effective lane count (10-lane freeways handled as 8-lane)
        let lanes = self.freeway_lanes.min(4);
        match lanes {
            3 => {
                // Equation 14-14: v_3 = v_F - v_12
                let v_3 = v_f - v_12;
                let mut candidates: Vec<f64> = Vec::new();
                // Equation 14-15 when v_3 > 2,700 pc/h/ln
                if v_3 > MAX_OUTER_LANE_FLOW {
                    candidates.push(v_f - MAX_OUTER_LANE_FLOW);
                }
                // Equation 14-16 when v_3 > 1.5 x (v_12 / 2): v_12a = v_F / 1.75
                if v_3 > 1.5 * (v_12 / 2.0) {
                    candidates.push(v_f / 1.75);
                }
                candidates.into_iter().fold(None, |acc: Option<f64>, c| {
                    Some(acc.map_or(c, |cur| cur.max(c)))
                })
                .unwrap_or(v_12)
            }
            4 => {
                // Equation 14-17: v_av34 = (v_F - v_12) / 2
                let v_av34 = (v_f - v_12) / 2.0;
                let mut candidates: Vec<f64> = Vec::new();
                // Equation 14-18 when v_av34 > 2,700 pc/h/ln: v_12a = v_F - 5,400
                if v_av34 > MAX_OUTER_LANE_FLOW {
                    candidates.push(v_f - 2.0 * MAX_OUTER_LANE_FLOW);
                }
                // Equation 14-19 when v_av34 > 1.5 x (v_12 / 2): v_12a = v_F / 2.50
                if v_av34 > 1.5 * (v_12 / 2.0) {
                    candidates.push(v_f / 2.50);
                }
                candidates.into_iter().fold(None, |acc: Option<f64>, c| {
                    Some(acc.map_or(c, |cur| cur.max(c)))
                })
                .unwrap_or(v_12)
            }
            _ => v_12,
        }
    }

    /// Step 2: Estimate the flow in Lanes 1 and 2 immediately upstream of
    /// the ramp influence area (pc/h). Sets `p_f`, `v_12`, `v_r12`, `v_oa`.
    ///
    /// Merge:   v_12 = v_F x P_FM (Eq. 14-2), v_R12 = v_12 + v_R (Eq. 14-20).
    /// Diverge: v_12 = v_R + (v_F - v_R) x P_FD (Eq. 14-8).
    /// Left-hand ramps: multiplied by Exhibit 14-18 factors.
    pub fn estimate_v12(&mut self) -> f64 {
        let v_f = self.flow_freeway.unwrap_or(0.0);
        let v_r = self.flow_ramp.unwrap_or(0.0);

        let mut v_12 = match self.ramp_type {
            RampType::OnRamp | RampType::MajorMerge => {
                let pfm = if self.ramp_lanes == RampLanes::TwoLane {
                    pfm_two_lane_onramp(self.freeway_lanes.min(4))
                } else {
                    self.calculate_pfm(v_f, v_r)
                };
                self.p_f = Some(pfm);
                v_f * pfm
            }
            RampType::OffRamp | RampType::MajorDiverge => {
                let pfd = if self.ramp_lanes == RampLanes::TwoLane {
                    pfd_two_lane_offramp(self.freeway_lanes.min(4))
                } else {
                    self.calculate_pfd(v_f, v_r)
                };
                self.p_f = Some(pfd);
                v_r + (v_f - v_r) * pfd
            }
        };

        // Left-hand ramps: Exhibit 14-18 adjustment applied to v_12 computed
        // as if the ramp were on the right side.
        if self.ramp_side == RampSide::Left && self.freeway_lanes > 2 {
            v_12 *= left_hand_adjustment(self.freeway_lanes.min(4), self.is_on_ramp());
        }

        // Reasonableness checks (Eqs. 14-14 through 14-19)
        v_12 = self.check_v12(v_f, v_12);
        self.v_12 = Some(v_12);

        // Total flow entering the ramp influence area
        let v_r12 = if self.is_on_ramp() { v_12 + v_r } else { v_12 };
        self.v_r12 = Some(v_r12);

        // Average flow per outer lane (Exhibit 14-15)
        let n_o = self.outer_lanes();
        self.v_oa = if n_o > 0 {
            Some((v_f - v_12) / n_o as f64)
        } else {
            Some(0.0)
        };

        v_12
    }

    /// Number of outer lanes N_O (0 for four-lane freeways).
    fn outer_lanes(&self) -> u32 {
        let lanes = self.freeway_lanes.min(4);
        lanes.saturating_sub(2)
    }

    // ── Step 3: Capacity checks ──────────────────────────────────────────

    /// Step 3: Capacity of the ramp-freeway junction (Exhibits 14-10/14-12,
    /// Eq. 14-21) compared against demand. Returns the adjusted freeway
    /// capacity (pc/h). Sets `vc_ratio`, `demand_exceeds_capacity`, and
    /// `exceeds_max_desirable`.
    pub fn determine_capacity(&mut self) -> f64 {
        let v_f = self.flow_freeway.unwrap_or(0.0);
        let v_r = self.flow_ramp.unwrap_or(0.0);

        // Equation 14-21: c_mda = c_md x CAF
        let capacity_freeway =
            get_freeway_capacity(self.freeway_ffs, self.freeway_lanes) * self.caf;
        let capacity_ramp =
            get_ramp_capacity(self.ramp_ffs, self.ramp_lanes == RampLanes::TwoLane) * self.caf;
        self.capacity_freeway = Some(capacity_freeway);
        self.capacity_ramp = Some(capacity_ramp);

        // Critical freeway checkpoint: downstream of a merge (v_FO = v_F + v_R)
        // or upstream of a diverge (v_F).
        let critical_flow = if self.is_on_ramp() { v_f + v_r } else { v_f };
        let vc = if capacity_freeway > 0.0 {
            critical_flow / capacity_freeway
        } else {
            f64::INFINITY
        };
        self.vc_ratio = Some(vc);

        // LOS F when demand exceeds freeway or ramp roadway capacity.
        let over = critical_flow > capacity_freeway || v_r > capacity_ramp;
        self.demand_exceeds_capacity = Some(over);

        // Maximum desirable flow entering the influence area (Exhibit 14-10):
        // exceeding it alone does not set LOS F.
        let v_r12 = self.v_r12.unwrap_or(0.0);
        let max_desirable = if self.is_on_ramp() {
            MAX_MERGE_INFLUENCE_FLOW
        } else {
            MAX_DIVERGE_INFLUENCE_FLOW
        };
        self.exceeds_max_desirable = Some(v_r12 > max_desirable);

        capacity_freeway
    }

    // ── Step 4: Density and LOS ──────────────────────────────────────────

    /// Step 4: Density in the ramp influence area (pc/mi/ln).
    ///
    /// Merge:   Equation 14-22, D_R = 5.475 + 0.00734 v_R + 0.0078 v_12 - 0.00627 L_A.
    /// Diverge: Equation 14-23, D_R = 4.252 + 0.0086 v_12 - 0.009 L_D.
    /// Major diverge: Equation 14-28, D_MD = 0.0175 (v_F / N).
    /// Major merge: no HCM density model (capacity checks only); returns 0
    /// and leaves `density` as None.
    pub fn determine_density(&mut self) -> f64 {
        let v_r = self.flow_ramp.unwrap_or(0.0);
        let v_12 = self.v_12.unwrap_or(0.0);

        let d = match self.ramp_type {
            RampType::OnRamp => {
                let l_a = self.effective_la();
                Some(5.475 + 0.00734 * v_r + 0.0078 * v_12 - 0.00627 * l_a)
            }
            RampType::OffRamp => {
                let l_d = self.effective_ld();
                Some(4.252 + 0.0086 * v_12 - 0.009 * l_d)
            }
            RampType::MajorDiverge => {
                let v_f = self.flow_freeway.unwrap_or(0.0);
                Some(0.0175 * (v_f / self.freeway_lanes as f64))
            }
            // No HCM performance model for major merge areas.
            RampType::MajorMerge => None,
        };

        self.density = d;
        d.unwrap_or(0.0)
    }

    /// Level of service - Exhibit 14-3. HCM defines no LOS for major merge
    /// areas (capacity checks only): `los` stays `None` unless a capacity
    /// checkpoint fails, in which case LOS F is reported.
    pub fn determine_los(&mut self) -> LevelOfService {
        let over = self.demand_exceeds_capacity.unwrap_or(false);
        if self.ramp_type == RampType::MajorMerge && !over {
            self.los = None;
            // No defined LOS; report E as the most conservative stable letter
            // is not HCM-sanctioned, so callers should consult `get_los()`.
            return LevelOfService::E;
        }
        let los = match self.density {
            Some(d) => determine_ramp_los(d, over),
            None => determine_ramp_los(f64::INFINITY, over),
        };
        self.los = Some(los);
        los
    }

    // ── Step 5: Speeds ───────────────────────────────────────────────────

    /// Step 5: Speeds in the vicinity of the junction (mi/h).
    /// Returns (S_R, S_O, S) where S_O is None for four-lane freeways.
    ///
    /// Merge (Exhibit 14-13):
    ///   M_S = 0.321 + 0.0039 e^(v_R12/1,000) - 0.002 (L_A x S_FR x SAF / 1,000)
    ///   S_R = FFS x SAF - (FFS x SAF - 42) M_S    [v_R12 capped at 4,600 for M_S]
    /// Diverge (Exhibit 14-14):
    ///   D_S = 0.883 + 0.00009 v_R - 0.013 S_FR x SAF
    ///   S_R = FFS x SAF - (FFS x SAF - 42) D_S
    /// Average of all lanes (Exhibit 14-15, space mean):
    ///   S = (v_R12 + v_OA N_O) / [(v_R12/S_R) + (v_OA N_O/S_O)], capped at FFS x SAF.
    pub fn estimate_speed(&mut self) -> (f64, Option<f64>, f64) {
        let ffs_adj = self.freeway_ffs * self.saf;
        let v_r = self.flow_ramp.unwrap_or(0.0);
        let v_r12 = self.v_r12.unwrap_or(0.0);
        let v_oa = self.v_oa.unwrap_or(0.0);
        let n_o = self.outer_lanes();

        let (s_r, s_o) = match self.ramp_type {
            RampType::OnRamp | RampType::MajorMerge => {
                let l_a = self.effective_la();
                // Exhibit 14-13 note: cap v_R12 at 4,600 pc/h for M_S
                let v_r12_capped = v_r12.min(MAX_MERGE_INFLUENCE_FLOW);
                let m_s = 0.321 + 0.0039 * (v_r12_capped / 1000.0).exp()
                    - 0.002 * (l_a * self.ramp_ffs * self.saf / 1000.0);
                // Merge-area speeds may not exceed FFS (Chapter 14 text).
                let s_r = (ffs_adj - (ffs_adj - 42.0) * m_s).min(ffs_adj);

                let s_o = if n_o > 0 {
                    Some(if v_oa < 500.0 {
                        ffs_adj
                    } else if v_oa <= 2300.0 {
                        ffs_adj - 0.0036 * (v_oa - 500.0)
                    } else {
                        ffs_adj - 6.53 - 0.006 * (v_oa - 2300.0)
                    })
                } else {
                    None
                };
                (s_r, s_o)
            }
            RampType::OffRamp | RampType::MajorDiverge => {
                let d_s = 0.883 + 0.00009 * v_r - 0.013 * self.ramp_ffs * self.saf;
                let s_r = ffs_adj - (ffs_adj - 42.0) * d_s;

                // Exhibit 14-14: outer-lane speed may marginally exceed FFS.
                let s_o = if n_o > 0 {
                    Some(if v_oa < 1000.0 {
                        1.097 * ffs_adj
                    } else {
                        1.097 * ffs_adj - 0.0039 * (v_oa - 1000.0)
                    })
                } else {
                    None
                };
                (s_r, s_o)
            }
        };

        // Exhibit 14-15: space mean speed of all vehicles, capped at FFS.
        let s_avg = match s_o {
            Some(s_o_val) if n_o > 0 => {
                let outer_flow = v_oa * n_o as f64;
                let total = v_r12 + outer_flow;
                if total > 0.0 && s_r > 0.0 && s_o_val > 0.0 {
                    (total / (v_r12 / s_r + outer_flow / s_o_val)).min(ffs_adj)
                } else {
                    s_r
                }
            }
            _ => s_r,
        };

        self.speed_ramp = Some(s_r);
        self.speed_outer = s_o;
        self.speed_avg = Some(s_avg);

        // Equation 14-24: aggregate density across all lanes, D = v / S.
        // VERIFY-HCM: Exhibit/Eq. 14-24 states v in pc/h/ln without fixing the
        // lane basis; the per-lane flow over the mainline lane count is used
        // here (merge: v_F + v_R; diverge: v_F).
        let v_f = self.flow_freeway.unwrap_or(0.0);
        let total_flow = if self.is_on_ramp() { v_f + v_r } else { v_f };
        if s_avg > 0.0 {
            self.density_all_lanes =
                Some(total_flow / (self.freeway_lanes as f64) / s_avg);
        }

        (s_r, s_o, s_avg)
    }

    /// Run the full HCM Chapter 14 analysis (Steps 1-5) and return the LOS.
    pub fn run_analysis(&mut self) -> LevelOfService {
        self.determine_demand_flow();
        self.estimate_v12();
        self.determine_capacity();
        self.determine_density();
        let los = self.determine_los();
        self.estimate_speed();
        los
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_los_criteria() {
        // Exhibit 14-3
        assert_eq!(determine_ramp_los(8.0, false), LevelOfService::A);
        assert_eq!(determine_ramp_los(15.0, false), LevelOfService::B);
        assert_eq!(determine_ramp_los(25.0, false), LevelOfService::C);
        assert_eq!(determine_ramp_los(32.0, false), LevelOfService::D);
        assert_eq!(determine_ramp_los(40.0, false), LevelOfService::E);
        assert_eq!(determine_ramp_los(25.0, true), LevelOfService::F);
    }

    #[test]
    fn test_freeway_capacity() {
        // Exhibit 14-10
        assert_eq!(get_freeway_capacity_per_lane(75.0), 2400.0);
        assert_eq!(get_freeway_capacity_per_lane(65.0), 2350.0);
        assert_eq!(get_freeway_capacity_per_lane(60.0), 2300.0);
        assert_eq!(get_freeway_capacity_per_lane(55.0), 2250.0);
        assert_eq!(get_freeway_capacity(70.0, 3), 7200.0);
    }

    #[test]
    fn test_ramp_capacity() {
        // Exhibit 14-12
        assert_eq!(get_ramp_capacity(55.0, false), 2200.0);
        assert_eq!(get_ramp_capacity(45.0, false), 2100.0);
        assert_eq!(get_ramp_capacity(35.0, false), 2000.0);
        assert_eq!(get_ramp_capacity(25.0, false), 1900.0);
        assert_eq!(get_ramp_capacity(15.0, false), 1800.0);
        assert_eq!(get_ramp_capacity(45.0, true), 4200.0);
    }

    #[test]
    fn test_two_lane_ramp_proportions() {
        assert_eq!(pfm_two_lane_onramp(2), 1.000);
        assert_eq!(pfm_two_lane_onramp(3), 0.555);
        assert_eq!(pfm_two_lane_onramp(4), 0.209);
        assert_eq!(pfd_two_lane_offramp(2), 1.000);
        assert_eq!(pfd_two_lane_offramp(3), 0.450);
        assert_eq!(pfd_two_lane_offramp(4), 0.260);
    }

    #[test]
    fn test_left_hand_adjustment() {
        // Exhibit 14-18
        assert_eq!(left_hand_adjustment(2, true), 1.00);
        assert_eq!(left_hand_adjustment(2, false), 1.00);
        assert!((left_hand_adjustment(3, true) - 1.12).abs() < 1e-9);
        assert!((left_hand_adjustment(3, false) - 1.05).abs() < 1e-9);
        assert!((left_hand_adjustment(4, true) - 1.20).abs() < 1e-9);
        assert!((left_hand_adjustment(4, false) - 1.10).abs() < 1e-9);
    }

    #[test]
    fn test_merge_analysis() {
        let mut seg = RampSegment {
            ramp_type: RampType::OnRamp,
            freeway_lanes: 3,
            freeway_ffs: 70.0,
            ramp_ffs: 40.0,
            accel_lane_length: Some(600.0),
            freeway_demand: 5000.0,
            ramp_demand: 800.0,
            phf: 0.92,
            heavy_vehicle_pct: 0.05,
            ..Default::default()
        };

        seg.run_analysis();

        assert!(seg.get_flow_freeway() > 0.0);
        assert!(seg.get_flow_ramp() > 0.0);
        assert!(seg.get_density() > 0.0);
        assert!(seg.get_speed_ramp() > 0.0 && seg.get_speed_ramp() <= seg.freeway_ffs);
        assert!(seg.get_vc_ratio() > 0.0);
    }

    #[test]
    fn test_diverge_analysis() {
        let mut seg = RampSegment {
            ramp_type: RampType::OffRamp,
            freeway_lanes: 4,
            freeway_ffs: 65.0,
            ramp_ffs: 35.0,
            decel_lane_length: Some(500.0),
            freeway_demand: 7000.0,
            ramp_demand: 1000.0,
            phf: 0.94,
            heavy_vehicle_pct: 0.08,
            ..Default::default()
        };

        seg.run_analysis();

        assert!(seg.get_flow_freeway() > 0.0);
        assert!(seg.get_flow_ramp() > 0.0);
        assert!(seg.get_density() > 0.0);
        assert!(seg.get_speed_ramp() > 0.0);
    }
}
