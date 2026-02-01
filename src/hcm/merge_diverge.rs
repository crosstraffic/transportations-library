use serde::{Deserialize, Serialize};
use crate::hcm::common::LevelOfService;

/// Chapter 14: Freeway Merge and Diverge Segments
///
/// This module provides analysis methodology for ramp-freeway junctions
/// including on-ramps (merge), off-ramps (diverge), and major merge/diverge areas.

// =============================================================================
// Constants
// =============================================================================

/// Length of ramp influence area (ft) - from Exhibit 14-1
/// TODO: VERIFY - 1500 ft is standard value
pub const RAMP_INFLUENCE_AREA_LENGTH: f64 = 1500.0;

/// Maximum flow rate per lane in outer lanes (pc/h/ln)
/// TODO: VERIFY - from Exhibit 14-11
pub const MAX_OUTER_LANE_FLOW: f64 = 2700.0;

/// Maximum desirable flow entering merge influence area (pc/h) - Exhibit 14-10
/// TODO: VERIFY - threshold for v_R12 cap
pub const MAX_MERGE_INFLUENCE_FLOW: f64 = 4600.0;

/// Maximum desirable flow entering diverge influence area (pc/h) - Exhibit 14-10
/// TODO: VERIFY - threshold for diverge check
pub const MAX_DIVERGE_INFLUENCE_FLOW: f64 = 4400.0;

// =============================================================================
// Enums and Types
// =============================================================================

/// Type of ramp junction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RampType {
    /// On-ramp (merge area)
    OnRamp,
    /// Off-ramp (diverge area)
    OffRamp,
    /// Major merge (two facilities joining)
    MajorMerge,
    /// Major diverge (facility splitting into two)
    MajorDiverge,
}

/// Side of freeway where ramp is located
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

/// LOS criteria for merge and diverge segments - Exhibit 14-3
/// TODO: VERIFY - LOS thresholds (A<=10, B<=20, C<=28, D<=35, E>35, F=oversaturated)
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
// Input Parameters
// =============================================================================

/// Input parameters for merge/diverge analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeDivergeInput {
    /// Type of ramp junction
    pub ramp_type: RampType,
    /// Side of freeway (right or left)
    pub ramp_side: RampSide,
    /// Number of lanes on ramp
    pub ramp_lanes: RampLanes,
    /// Number of freeway lanes in one direction (2-5)
    pub freeway_lanes: u32,
    /// Free-flow speed of freeway (mi/h)
    pub freeway_ffs: f64,
    /// Free-flow speed of ramp (mi/h)
    pub ramp_ffs: f64,
    /// Length of acceleration lane (ft) - for on-ramps
    pub accel_lane_length: Option<f64>,
    /// Length of deceleration lane (ft) - for off-ramps
    pub decel_lane_length: Option<f64>,
    /// Demand volume on freeway upstream of ramp (veh/h)
    pub freeway_demand: f64,
    /// Demand volume on ramp (veh/h)
    pub ramp_demand: f64,
    /// Peak hour factor
    pub phf: f64,
    /// Heavy vehicle percentage (decimal, e.g., 0.05 for 5%)
    pub heavy_vehicle_pct: f64,
    /// Terrain type for heavy vehicle adjustment
    pub terrain: TerrainType,
    /// Adjacent upstream ramp type
    pub adjacent_upstream: AdjacentRampType,
    /// Distance to adjacent upstream ramp (ft)
    pub upstream_distance: Option<f64>,
    /// Flow rate on adjacent upstream ramp (veh/h)
    pub upstream_ramp_flow: Option<f64>,
    /// Adjacent downstream ramp type
    pub adjacent_downstream: AdjacentRampType,
    /// Distance to adjacent downstream ramp (ft)
    pub downstream_distance: Option<f64>,
    /// Flow rate on adjacent downstream ramp (veh/h)
    pub downstream_ramp_flow: Option<f64>,
    /// Capacity adjustment factor (default 1.0)
    pub caf: f64,
    /// Speed adjustment factor (default 1.0)
    pub saf: f64,
}

/// Terrain type for PCE calculation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TerrainType {
    Level,
    Rolling,
    Mountainous,
}

impl Default for MergeDivergeInput {
    fn default() -> Self {
        Self {
            ramp_type: RampType::OnRamp,
            ramp_side: RampSide::Right,
            ramp_lanes: RampLanes::OneLane,
            freeway_lanes: 3,
            freeway_ffs: 70.0,
            ramp_ffs: 35.0,
            accel_lane_length: Some(800.0),
            decel_lane_length: Some(400.0),
            freeway_demand: 4000.0,
            ramp_demand: 500.0,
            phf: 0.94,
            heavy_vehicle_pct: 0.05,
            terrain: TerrainType::Level,
            adjacent_upstream: AdjacentRampType::None,
            upstream_distance: None,
            upstream_ramp_flow: None,
            adjacent_downstream: AdjacentRampType::None,
            downstream_distance: None,
            downstream_ramp_flow: None,
            caf: 1.0,
            saf: 1.0,
        }
    }
}

// =============================================================================
// Output Results
// =============================================================================

/// Results of merge/diverge analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeDivergeResult {
    /// Flow rate on freeway (pc/h)
    pub v_f: f64,
    /// Flow rate on ramp (pc/h)
    pub v_r: f64,
    /// Flow rate in Lanes 1 and 2 (pc/h)
    pub v_12: f64,
    /// Total flow entering ramp influence area (pc/h) - for merges: v_12 + v_r
    pub v_r12: f64,
    /// Density in ramp influence area (pc/mi/ln)
    pub density_ramp: f64,
    /// Speed in ramp influence area (mi/h)
    pub speed_ramp: f64,
    /// Speed in outer lanes (mi/h)
    pub speed_outer: Option<f64>,
    /// Average speed all lanes (mi/h)
    pub speed_avg: f64,
    /// Aggregate density all lanes (pc/mi/ln)
    pub density_avg: f64,
    /// Capacity of freeway segment (pc/h)
    pub capacity_freeway: f64,
    /// Capacity of ramp roadway (pc/h)
    pub capacity_ramp: f64,
    /// Volume-to-capacity ratio
    pub vc_ratio: f64,
    /// Level of Service
    pub los: LevelOfService,
    /// Whether demand exceeds capacity
    pub demand_exceeds_capacity: bool,
}

// =============================================================================
// Capacity Functions - Exhibits 14-10, 14-11, 14-12
// =============================================================================

/// Get freeway capacity per lane based on FFS - Exhibit 14-10
/// Returns capacity in pc/h/ln
/// TODO: VERIFY - capacity values (70+: 2400, 65-70: 2350, 60-65: 2300, <60: 2250)
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

/// Get total freeway capacity based on FFS and number of lanes - Exhibit 14-10
pub fn get_freeway_capacity(ffs: f64, lanes: u32) -> f64 {
    get_freeway_capacity_per_lane(ffs) * (lanes as f64)
}

/// Get ramp roadway capacity based on ramp FFS - Exhibit 14-12
/// TODO: VERIFY - capacity values (>50: 2200, >40: 2100, >30: 2000, >=20: 1900, <20: 1800)
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
// Heavy Vehicle Adjustment - Equation 14-1
// =============================================================================

/// Calculate heavy vehicle adjustment factor f_HV
/// Uses simplified PCE values based on terrain
/// TODO: VERIFY - PCE values (Level: 1.5, Rolling: 2.5, Mountainous: 4.5) against Chapter 26
pub fn calculate_fhv(heavy_vehicle_pct: f64, terrain: TerrainType) -> f64 {
    // PCE values from Chapter 12/26 (simplified)
    let e_t = match terrain {
        TerrainType::Level => 1.5,
        TerrainType::Rolling => 2.5,
        TerrainType::Mountainous => 4.5,
    };

    1.0 / (1.0 + heavy_vehicle_pct * (e_t - 1.0))
}

/// Convert demand volume to flow rate in pc/h - Equation 14-1
pub fn convert_to_flow_rate(volume: f64, phf: f64, f_hv: f64) -> f64 {
    volume / (phf * f_hv)
}

// =============================================================================
// PFM Models for On-Ramps (Merge Areas) - Exhibit 14-8
// =============================================================================

/// Calculate PFM for 4-lane freeway (2 lanes per direction)
/// All vehicles are in Lanes 1 and 2
fn pfm_4_lane() -> f64 {
    1.0
}

/// Calculate PFM for 6-lane freeway - Equation 14-3 (base case)
/// TODO: VERIFY - coefficients (0.5775, 0.000028, 0.00001)
fn pfm_6_lane_base(v_f: f64, ramp_ffs: f64) -> f64 {
    0.5775 + 0.000028 * v_f + 0.00001 * ramp_ffs
}

/// Calculate PFM for 6-lane with upstream off-ramp - Equation 14-4
fn pfm_6_lane_upstream_off(v_f: f64, ramp_ffs: f64, v_u: f64, l_up: f64) -> f64 {
    0.7289 + 0.0000135 * v_f - 0.003296 * ramp_ffs + 0.000196 * v_u - 0.000073 * l_up
}

/// Calculate PFM for 6-lane with downstream off-ramp - Equation 14-5
fn pfm_6_lane_downstream_off(v_f: f64, ramp_ffs: f64, v_d: f64, l_down: f64) -> f64 {
    0.5487 + 0.2628 * (v_d / l_down)
}

/// Calculate equilibrium distance for upstream off-ramp - Equation 14-6
fn leq_upstream_off(v_f: f64, ramp_ffs: f64, v_u: f64) -> f64 {
    let pf_base = pfm_6_lane_base(v_f, ramp_ffs);
    (0.1514 + 0.000135 * v_f - 0.003296 * ramp_ffs + 0.000196 * v_u - pf_base) / 0.000073
}

/// Calculate equilibrium distance for downstream off-ramp - Equation 14-7
fn leq_downstream_off(v_f: f64, ramp_ffs: f64, v_d: f64) -> f64 {
    let pf_base = pfm_6_lane_base(v_f, ramp_ffs);
    0.2628 * v_d / (pf_base - 0.5487)
}

/// Calculate PFM for 8-lane freeway (4 lanes per direction)
fn pfm_8_lane(v_f: f64, ramp_ffs: f64) -> f64 {
    0.2178 + 0.000025 * v_f + 0.00001 * ramp_ffs
}

/// Calculate PFM based on freeway configuration and adjacent ramps
pub fn calculate_pfm(input: &MergeDivergeInput, v_f: f64) -> f64 {
    let lanes = input.freeway_lanes;
    let ramp_ffs = input.ramp_ffs;

    match lanes {
        2 => pfm_4_lane(),
        3 => {
            // 6-lane freeway - check for adjacent ramp effects
            let base_pfm = pfm_6_lane_base(v_f, ramp_ffs);

            // Check upstream off-ramp
            if let (AdjacentRampType::OffRamp, Some(l_up), Some(v_u)) =
                (input.adjacent_upstream, input.upstream_distance, input.upstream_ramp_flow) {
                let v_u_pc = convert_to_flow_rate(v_u, input.phf,
                    calculate_fhv(input.heavy_vehicle_pct, input.terrain));
                let l_eq = leq_upstream_off(v_f, ramp_ffs, v_u_pc);
                if l_up < l_eq {
                    return pfm_6_lane_upstream_off(v_f, ramp_ffs, v_u_pc, l_up);
                }
            }

            // Check downstream off-ramp
            if let (AdjacentRampType::OffRamp, Some(l_down), Some(v_d)) =
                (input.adjacent_downstream, input.downstream_distance, input.downstream_ramp_flow) {
                let v_d_pc = convert_to_flow_rate(v_d, input.phf,
                    calculate_fhv(input.heavy_vehicle_pct, input.terrain));
                let l_eq = leq_downstream_off(v_f, ramp_ffs, v_d_pc);
                if l_down < l_eq {
                    return pfm_6_lane_downstream_off(v_f, ramp_ffs, v_d_pc, l_down);
                }
            }

            base_pfm
        },
        4 => pfm_8_lane(v_f, ramp_ffs),
        5 => {
            // 10-lane freeway - estimate Lane 5 flow and treat as 8-lane
            // See Exhibit 14-19 for Lane 5 flow estimation
            pfm_8_lane(v_f, ramp_ffs)
        },
        _ => pfm_8_lane(v_f, ramp_ffs), // Default to 8-lane model
    }
}

// =============================================================================
// PFD Models for Off-Ramps (Diverge Areas) - Exhibit 14-9
// =============================================================================

/// Calculate PFD for 4-lane freeway
fn pfd_4_lane() -> f64 {
    1.0
}

/// Calculate PFD for 6-lane freeway - Equation 14-9 (base case)
fn pfd_6_lane_base(v_f: f64, v_r: f64, l_d: f64) -> f64 {
    0.760 + 0.000025 * v_f - 0.000046 * v_r
}

/// Calculate PFD for 6-lane with upstream on-ramp - Equation 14-10
fn pfd_6_lane_upstream_on(v_u: f64, l_up: f64) -> f64 {
    0.717 + 0.000039 * v_u + 0.604 * (v_u / l_up)
}

/// Calculate PFD for 6-lane with downstream off-ramp - Equation 14-11
fn pfd_6_lane_downstream_off(v_f: f64, v_r: f64, v_d: f64, l_down: f64) -> f64 {
    0.616 + 0.000021 * v_f - 0.000108 * v_r + 0.000076 * v_d - 0.000025 * l_down
}

/// Calculate equilibrium distance for upstream on-ramp - Equation 14-12
fn leq_upstream_on_diverge(v_f: f64, v_r: f64, v_u: f64) -> f64 {
    let pf_base = pfd_6_lane_base(v_f, v_r, 0.0);
    0.604 * v_u / (pf_base - 0.717 - 0.000039 * v_u)
}

/// Calculate equilibrium distance for downstream off-ramp (diverge) - Equation 14-13
fn leq_downstream_off_diverge(v_f: f64, v_r: f64, v_d: f64) -> f64 {
    let pf_base = pfd_6_lane_base(v_f, v_r, 0.0);
    (0.144 - 0.000021 * v_f + 0.000062 * v_r + 0.000076 * v_d - pf_base) / 0.000025
}

/// Calculate PFD for 8-lane freeway
fn pfd_8_lane(v_f: f64, v_r: f64) -> f64 {
    0.436 + 0.000025 * v_f - 0.000046 * v_r
}

/// Calculate PFD based on freeway configuration and adjacent ramps
pub fn calculate_pfd(input: &MergeDivergeInput, v_f: f64, v_r: f64) -> f64 {
    let lanes = input.freeway_lanes;
    let l_d = input.decel_lane_length.unwrap_or(400.0);

    match lanes {
        2 => pfd_4_lane(),
        3 => {
            // 6-lane freeway
            let base_pfd = pfd_6_lane_base(v_f, v_r, l_d);

            // Check upstream on-ramp
            if let (AdjacentRampType::OnRamp, Some(l_up), Some(v_u)) =
                (input.adjacent_upstream, input.upstream_distance, input.upstream_ramp_flow) {
                let v_u_pc = convert_to_flow_rate(v_u, input.phf,
                    calculate_fhv(input.heavy_vehicle_pct, input.terrain));

                // Check v_u/l_up ratio limit
                if v_u_pc / l_up <= 0.20 {
                    let l_eq = leq_upstream_on_diverge(v_f, v_r, v_u_pc);
                    if l_up < l_eq {
                        return pfd_6_lane_upstream_on(v_u_pc, l_up);
                    }
                }
            }

            // Check downstream off-ramp
            if let (AdjacentRampType::OffRamp, Some(l_down), Some(v_d)) =
                (input.adjacent_downstream, input.downstream_distance, input.downstream_ramp_flow) {
                let v_d_pc = convert_to_flow_rate(v_d, input.phf,
                    calculate_fhv(input.heavy_vehicle_pct, input.terrain));
                let l_eq = leq_downstream_off_diverge(v_f, v_r, v_d_pc);
                if l_down < l_eq {
                    return pfd_6_lane_downstream_off(v_f, v_r, v_d_pc, l_down);
                }
            }

            base_pfd
        },
        4 => pfd_8_lane(v_f, v_r),
        5 => pfd_8_lane(v_f, v_r), // 10-lane treated as 8-lane
        _ => pfd_8_lane(v_f, v_r),
    }
}

// =============================================================================
// Two-Lane Ramp Adjustments
// =============================================================================

/// Get PFM for two-lane on-ramps
pub fn pfm_two_lane_onramp(freeway_lanes: u32) -> f64 {
    match freeway_lanes {
        2 => 1.000,
        3 => 0.555,
        4 => 0.209,
        _ => 0.209,
    }
}

/// Get PFD for two-lane off-ramps
pub fn pfd_two_lane_offramp(freeway_lanes: u32) -> f64 {
    match freeway_lanes {
        2 => 1.000,
        3 => 0.450,
        4 => 0.260,
        _ => 0.260,
    }
}

/// Calculate effective acceleration lane length for two-lane ramps - Equation 14-25
/// L_A_eff = 2 * L_A1 + L_A2
pub fn effective_accel_length(l_a1: f64, l_a2: f64) -> f64 {
    (2.0 * l_a1 + l_a2).min(RAMP_INFLUENCE_AREA_LENGTH)
}

/// Calculate effective deceleration lane length for two-lane ramps - Equation 14-26
/// L_D_eff = 2 * L_D1 + L_D2
pub fn effective_decel_length(l_d1: f64, l_d2: f64) -> f64 {
    (2.0 * l_d1 + l_d2).min(RAMP_INFLUENCE_AREA_LENGTH)
}

// =============================================================================
// Left-Hand Ramp Adjustments - Exhibit 14-18
// =============================================================================

/// Get adjustment factor for left-hand ramps - Exhibit 14-18
pub fn left_hand_adjustment(freeway_lanes: u32, is_on_ramp: bool) -> f64 {
    match (freeway_lanes, is_on_ramp) {
        (3, true) => 0.865,  // 6-lane, on-ramp
        (3, false) => 0.735, // 6-lane, off-ramp
        (4, true) => 0.780,  // 8-lane, on-ramp
        (4, false) => 0.780, // 8-lane, off-ramp
        _ => 1.0,
    }
}

// =============================================================================
// Density Equations - Equations 14-22, 14-23
// =============================================================================

/// Calculate density in on-ramp (merge) influence area - Equation 14-22
/// D_R = 3.402 + 0.00456 * v_R + 0.0048 * v_12 - 0.01278 * L_A
/// TODO: VERIFY - coefficients (3.402, 0.00456, 0.0048, 0.01278)
pub fn calculate_merge_density(v_r: f64, v_12: f64, l_a: f64) -> f64 {
    3.402 + 0.00456 * v_r + 0.0048 * v_12 - 0.01278 * l_a
}

/// Calculate density in off-ramp (diverge) influence area - Equation 14-23
/// D_R = 4.252 + 0.0086 * v_12 - 0.009 * L_D
/// TODO: VERIFY - coefficients (4.252, 0.0086, 0.009)
pub fn calculate_diverge_density(v_12: f64, l_d: f64) -> f64 {
    4.252 + 0.0086 * v_12 - 0.009 * l_d
}

/// Calculate density in major diverge influence area - Equation 14-28
/// D_MD = 0.0067 * (v_F / N)
pub fn calculate_major_diverge_density(v_f: f64, n: u32) -> f64 {
    0.0067 * (v_f / n as f64)
}

// =============================================================================
// Speed Equations - Exhibits 14-13, 14-14, 14-15
// =============================================================================

/// Calculate speed in merge influence area - Exhibit 14-13
pub fn calculate_merge_speed(ffs: f64, ramp_ffs: f64, l_a: f64, v_r12: f64, saf: f64) -> f64 {
    // M_S speed index (capped at v_R12 = 4600)
    let v_r12_capped = v_r12.min(4600.0);
    let m_s = 0.321 + 0.0039 * (-(v_r12_capped / 1000.0)).exp() * (l_a - 300.0).max(0.0);

    // S_R = FFS - (FFS - 42) * M_S
    let s_r = ffs - (ffs - 42.0) * m_s;

    // Apply SAF
    (s_r * saf).min(ffs)
}

/// Calculate speed in outer lanes at merge - Exhibit 14-13
pub fn calculate_merge_outer_speed(ffs: f64, v_oa: f64, saf: f64) -> f64 {
    // S_O = FFS - 0.0036 * (v_OA - 500)
    let s_o = ffs - 0.0036 * (v_oa - 500.0).max(0.0);
    (s_o * saf).min(ffs)
}

/// Calculate speed in diverge influence area - Exhibit 14-14
pub fn calculate_diverge_speed(ffs: f64, ramp_ffs: f64, v_12: f64, v_r: f64, saf: f64) -> f64 {
    // D_S speed index
    let d_s = 0.883 + 0.00009 * v_12 - 0.013 * ramp_ffs;

    // S_R = FFS - (FFS - 42) * D_S
    let s_r = ffs - (ffs - 42.0) * d_s;

    // Apply SAF
    (s_r * saf).min(ffs)
}

/// Calculate speed in outer lanes at diverge - Exhibit 14-14
pub fn calculate_diverge_outer_speed(ffs: f64, v_f: f64, v_12: f64, lanes: u32, saf: f64) -> f64 {
    let n_o = (lanes - 2) as f64;
    if n_o <= 0.0 {
        return ffs * saf;
    }

    let v_oa = (v_f - v_12) / n_o;
    // S_O = FFS + 0.0036 * (500 - v_OA)
    let s_o = ffs + 0.0036 * (500.0 - v_oa);
    (s_o * saf).min(ffs * 1.05) // Allow slight overage for diverge
}

/// Calculate average speed across all lanes - Exhibit 14-15
pub fn calculate_average_speed(
    s_r: f64,
    s_o: f64,
    v_r12: f64,
    v_oa: f64,
    n_o: u32,
    is_merge: bool,
) -> f64 {
    if n_o == 0 {
        return s_r;
    }

    let n_o_f = n_o as f64;

    // Weighted average based on flow
    let total_flow = v_r12 + v_oa * n_o_f;
    if total_flow <= 0.0 {
        return s_r;
    }

    (v_r12 * s_r + v_oa * n_o_f * s_o) / total_flow
}

// =============================================================================
// Lane Distribution Checks - Equations 14-14 to 14-19
// =============================================================================

/// Check and adjust v_12 for 6-lane freeway - Equations 14-14 to 14-16
pub fn check_v12_6lane(v_f: f64, v_12: f64) -> f64 {
    let v_3 = v_f - v_12;

    // Check if v_3 > 2700 pc/h/ln
    if v_3 > MAX_OUTER_LANE_FLOW {
        return v_f - MAX_OUTER_LANE_FLOW;
    }

    // Check if v_3 > 1.5 * (v_12/2)
    if v_3 > 1.5 * (v_12 / 2.0) {
        return v_f / 2.5;
    }

    v_12
}

/// Check and adjust v_12 for 8-lane freeway - Equations 14-17 to 14-19
pub fn check_v12_8lane(v_f: f64, v_12: f64) -> f64 {
    let v_av34 = (v_f - v_12) / 2.0;

    // Check if v_av34 > 2700 pc/h/ln
    if v_av34 > MAX_OUTER_LANE_FLOW {
        return v_f - 2.0 * MAX_OUTER_LANE_FLOW;
    }

    // Check if v_av34 > 1.5 * (v_12/2)
    if v_av34 > 1.5 * (v_12 / 2.0) {
        return v_f / 4.0;
    }

    v_12
}

// =============================================================================
// Main Analysis Function
// =============================================================================

/// Perform merge/diverge segment analysis
pub fn analyze(input: &MergeDivergeInput) -> MergeDivergeResult {
    // Step 1: Convert volumes to flow rates
    let f_hv = calculate_fhv(input.heavy_vehicle_pct, input.terrain);
    let v_f = convert_to_flow_rate(input.freeway_demand, input.phf, f_hv);
    let v_r = convert_to_flow_rate(input.ramp_demand, input.phf, f_hv);

    // Step 2: Estimate flow in Lanes 1 and 2
    let (mut v_12, v_r12) = match input.ramp_type {
        RampType::OnRamp => {
            let pfm = if input.ramp_lanes == RampLanes::TwoLane {
                pfm_two_lane_onramp(input.freeway_lanes)
            } else {
                calculate_pfm(input, v_f)
            };
            let v12 = v_f * pfm;
            let vr12 = v12 + v_r; // Equation 14-20
            (v12, vr12)
        },
        RampType::OffRamp => {
            let pfd = if input.ramp_lanes == RampLanes::TwoLane {
                pfd_two_lane_offramp(input.freeway_lanes)
            } else {
                calculate_pfd(input, v_f, v_r)
            };
            let v12 = v_r + (v_f - v_r) * pfd; // Equation 14-8
            (v12, v12) // For diverge, v_12 includes ramp traffic
        },
        RampType::MajorMerge | RampType::MajorDiverge => {
            // Major merge/diverge - simplified analysis
            (v_f * 0.5, v_f)
        },
    };

    // Apply left-hand adjustment if needed
    if input.ramp_side == RampSide::Left && input.freeway_lanes > 2 {
        let adj = left_hand_adjustment(
            input.freeway_lanes,
            input.ramp_type == RampType::OnRamp
        );
        v_12 *= adj;
    }

    // Check and adjust v_12 for outer lane reasonableness
    v_12 = match input.freeway_lanes {
        3 => check_v12_6lane(v_f, v_12),
        4 | 5 => check_v12_8lane(v_f, v_12),
        _ => v_12,
    };

    // Step 3: Calculate capacity and check demand
    let capacity_freeway = get_freeway_capacity(input.freeway_ffs, input.freeway_lanes) * input.caf;
    let capacity_ramp = get_ramp_capacity(input.ramp_ffs, input.ramp_lanes == RampLanes::TwoLane);

    let demand_flow = match input.ramp_type {
        RampType::OnRamp => v_f + v_r,  // Downstream of merge
        RampType::OffRamp => v_f,        // Upstream of diverge
        RampType::MajorMerge => v_f + v_r,
        RampType::MajorDiverge => v_f,
    };

    let demand_exceeds_capacity = demand_flow > capacity_freeway || v_r > capacity_ramp;
    let vc_ratio = demand_flow / capacity_freeway;

    // Step 4: Calculate density
    let l_a = input.accel_lane_length.unwrap_or(800.0);
    let l_d = input.decel_lane_length.unwrap_or(400.0);

    let density_ramp = match input.ramp_type {
        RampType::OnRamp => calculate_merge_density(v_r, v_12, l_a),
        RampType::OffRamp => calculate_diverge_density(v_12, l_d),
        RampType::MajorMerge => calculate_merge_density(v_r, v_12, l_a),
        RampType::MajorDiverge => calculate_major_diverge_density(v_f, input.freeway_lanes),
    };

    // Step 5: Calculate speeds
    let n_o = if input.freeway_lanes > 2 { input.freeway_lanes - 2 } else { 0 };
    let v_oa = if n_o > 0 { (v_f - v_12) / (n_o as f64) } else { 0.0 };

    let (speed_ramp, speed_outer) = match input.ramp_type {
        RampType::OnRamp => {
            let s_r = calculate_merge_speed(input.freeway_ffs, input.ramp_ffs, l_a, v_r12, input.saf);
            let s_o = if n_o > 0 {
                Some(calculate_merge_outer_speed(input.freeway_ffs, v_oa, input.saf))
            } else {
                None
            };
            (s_r, s_o)
        },
        RampType::OffRamp => {
            let s_r = calculate_diverge_speed(input.freeway_ffs, input.ramp_ffs, v_12, v_r, input.saf);
            let s_o = if n_o > 0 {
                Some(calculate_diverge_outer_speed(input.freeway_ffs, v_f, v_12, input.freeway_lanes, input.saf))
            } else {
                None
            };
            (s_r, s_o)
        },
        RampType::MajorMerge | RampType::MajorDiverge => {
            // Simplified speed calculation
            let s = input.freeway_ffs * input.saf * (1.0 - 0.1 * (vc_ratio - 0.5).max(0.0));
            (s, Some(s))
        },
    };

    // Calculate average speed
    let speed_avg = match speed_outer {
        Some(s_o) => calculate_average_speed(speed_ramp, s_o, v_r12, v_oa, n_o,
            input.ramp_type == RampType::OnRamp),
        None => speed_ramp,
    };

    // Calculate aggregate density - Equation 14-24
    let total_lanes = input.freeway_lanes as f64 + 1.0; // Include accel/decel lane
    let density_avg = if speed_avg > 0.0 {
        demand_flow / (speed_avg * total_lanes)
    } else {
        density_ramp
    };

    // Determine LOS
    let los = determine_ramp_los(density_ramp, demand_exceeds_capacity);

    MergeDivergeResult {
        v_f,
        v_r,
        v_12,
        v_r12,
        density_ramp,
        speed_ramp,
        speed_outer,
        speed_avg,
        density_avg,
        capacity_freeway,
        capacity_ramp,
        vc_ratio,
        los,
        demand_exceeds_capacity,
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
        // From Exhibit 14-3
        assert_eq!(determine_ramp_los(8.0, false), LevelOfService::A);
        assert_eq!(determine_ramp_los(15.0, false), LevelOfService::B);
        assert_eq!(determine_ramp_los(25.0, false), LevelOfService::C);
        assert_eq!(determine_ramp_los(32.0, false), LevelOfService::D);
        assert_eq!(determine_ramp_los(40.0, false), LevelOfService::E);
        assert_eq!(determine_ramp_los(25.0, true), LevelOfService::F);
    }

    #[test]
    fn test_freeway_capacity() {
        // From Exhibit 14-10
        assert_eq!(get_freeway_capacity_per_lane(75.0), 2400.0);
        assert_eq!(get_freeway_capacity_per_lane(65.0), 2350.0);
        assert_eq!(get_freeway_capacity_per_lane(60.0), 2300.0);
        assert_eq!(get_freeway_capacity_per_lane(55.0), 2250.0);

        assert_eq!(get_freeway_capacity(70.0, 3), 7200.0);
    }

    #[test]
    fn test_ramp_capacity() {
        // From Exhibit 14-12
        assert_eq!(get_ramp_capacity(55.0, false), 2200.0);
        assert_eq!(get_ramp_capacity(45.0, false), 2100.0);
        assert_eq!(get_ramp_capacity(35.0, false), 2000.0);
        assert_eq!(get_ramp_capacity(25.0, false), 1900.0);

        assert_eq!(get_ramp_capacity(45.0, true), 4200.0);
    }

    #[test]
    fn test_heavy_vehicle_adjustment() {
        // Level terrain with 5% trucks
        let f_hv = calculate_fhv(0.05, TerrainType::Level);
        assert!((f_hv - 0.9756).abs() < 0.01);

        // Rolling terrain with 10% trucks
        let f_hv = calculate_fhv(0.10, TerrainType::Rolling);
        assert!((f_hv - 0.87).abs() < 0.02);
    }

    #[test]
    fn test_pfm_4lane() {
        assert_eq!(pfm_4_lane(), 1.0);
    }

    #[test]
    fn test_pfd_4lane() {
        assert_eq!(pfd_4_lane(), 1.0);
    }

    #[test]
    fn test_two_lane_ramp_pfm() {
        assert_eq!(pfm_two_lane_onramp(2), 1.000);
        assert_eq!(pfm_two_lane_onramp(3), 0.555);
        assert_eq!(pfm_two_lane_onramp(4), 0.209);
    }

    #[test]
    fn test_two_lane_ramp_pfd() {
        assert_eq!(pfd_two_lane_offramp(2), 1.000);
        assert_eq!(pfd_two_lane_offramp(3), 0.450);
        assert_eq!(pfd_two_lane_offramp(4), 0.260);
    }

    #[test]
    fn test_left_hand_adjustment() {
        // From Exhibit 14-18
        assert!((left_hand_adjustment(3, true) - 0.865).abs() < 0.01);
        assert!((left_hand_adjustment(3, false) - 0.735).abs() < 0.01);
        assert!((left_hand_adjustment(4, true) - 0.780).abs() < 0.01);
    }

    #[test]
    fn test_merge_analysis() {
        let input = MergeDivergeInput {
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

        let result = analyze(&input);

        // Check that results are reasonable
        assert!(result.v_f > 0.0);
        assert!(result.v_r > 0.0);
        assert!(result.density_ramp > 0.0);
        assert!(result.speed_ramp > 0.0 && result.speed_ramp <= input.freeway_ffs);
        assert!(result.vc_ratio > 0.0);
    }

    #[test]
    fn test_diverge_analysis() {
        let input = MergeDivergeInput {
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

        let result = analyze(&input);

        // Check that results are reasonable
        assert!(result.v_f > 0.0);
        assert!(result.v_r > 0.0);
        assert!(result.density_ramp > 0.0);
        assert!(result.speed_ramp > 0.0);
    }
}
