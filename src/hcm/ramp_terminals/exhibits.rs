//! HCM Chapter 23 (Ramp Terminals and Alternative Intersections) exhibit
//! tables, constants, and single-equation adjustment factors.
//!
//! Sources (HCM 7th Edition):
//! * Exhibit 23-10 — LOS criteria for each O-D within signalized
//!   interchanges (172_Ch23_pt1_02.xhtml)
//! * Exhibit 23-13 — LOS criteria for each O-D within alternative
//!   intersections (172_Ch23_pt1_02.xhtml; carried for the milestone-2
//!   RCUT/MUT/DLT extension)
//! * Exhibit 23-14 — LOS criteria for each O-D of an interchange with
//!   roundabouts (172_Ch23_pt1_02.xhtml)
//! * Equation 23-15 / Exhibit 23-23 — traffic pressure adjustment f_v
//!   (175_Ch23_pt2_03.xhtml)
//! * Equations 23-16 / 23-17 / Exhibit 23-24 — lane utilization for the
//!   external arterial approaches of diamond and parclo interchanges
//!   (175_Ch23_pt2_03.xhtml)
//! * Equation 23-18 / Exhibits 23-25 / 23-26 — DDI lane utilization
//!   (175_Ch23_pt2_03.xhtml)
//! * Equations 23-19 through 23-23 / Exhibit 23-27 — turn radius
//!   adjustment (175_Ch23_pt2_03.xhtml)
//! * Exhibit 23-36 — default DDI turn calibration parameters
//!   (175_Ch23_pt2_03.xhtml)
//! * Exhibit 34-161 — notation of O-D demands at interchanges with
//!   roundabouts (271_Ch34_04.xhtml)

use crate::hcm::common::LevelOfService;
use serde::{Deserialize, Serialize};

// ═══════════════════════════════════════════════════════════════════════════════
// Constants
// ═══════════════════════════════════════════════════════════════════════════════

/// Saturation flow adjustment for the DDI crossover f_DDI
/// (HCM Chapter 23, Interchange Saturation Flow Adjustment No. 3:
/// "= 0.913 according to research (5)"; average field-measured saturation
/// flows at 11 DDIs were lower by this factor).
pub const F_DDI: f64 = 0.913;

/// Upper limit of the demand flow rate per cycle per lane used in the
/// traffic pressure adjustment (HCM Equation 23-15 discussion: "For values
/// of v_i' higher than 30 veh/cycle/ln, 30 veh/cycle/ln should be used").
pub const TRAFFIC_PRESSURE_DEMAND_CAP: f64 = 30.0;

/// Distance-to-downstream-queue threshold above which no additional lost
/// time is experienced (HCM Chapter 23, Lost Time Adjustment No. 1: "if
/// DQA or DQR exceeds 200 ft, the lost time will be zero"), ft.
pub const DOWNSTREAM_QUEUE_LOST_TIME_MAX_DISTANCE_FT: f64 = 200.0;

/// Default average queue spacing in a stationary queue L_h, ft/veh
/// (HCM Exhibit 23-29: "default of 25 ft/veh").
pub const DEFAULT_QUEUE_SPACING_FT: f64 = 25.0;

/// Upper limit of Equation 23-17 validity for the intersection spacing D
/// (HCM Chapter 23: "Equation 23-17 is valid for values of D below
/// 800 ft"), ft.
pub const LANE_UTILIZATION_MAX_SPACING_FT: f64 = 800.0;

/// Default deceleration/acceleration delay for a loop ramp movement in the
/// EDTT computation (HCM Equation 23-50: "assumed to be 5 s for a loop
/// ramp movement"), s.
pub const EDTT_LOOP_RAMP_ACCEL_DECEL_S: f64 = 5.0;

/// Default critical headway t_c for YIELD-controlled DDI left turns, s
/// (HCM Exhibit 23-36).
pub const DDI_LEFT_CRITICAL_HEADWAY_S: f64 = 3.9;
/// Default follow-up headway t_f for YIELD-controlled DDI left turns, s
/// (HCM Exhibit 23-36).
pub const DDI_LEFT_FOLLOW_UP_HEADWAY_S: f64 = 2.6;
/// Default critical headway t_c for YIELD-controlled DDI right turns, s
/// (HCM Exhibit 23-36).
pub const DDI_RIGHT_CRITICAL_HEADWAY_S: f64 = 1.8;
/// Default follow-up headway t_f for YIELD-controlled DDI right turns, s
/// (HCM Exhibit 23-36).
pub const DDI_RIGHT_FOLLOW_UP_HEADWAY_S: f64 = 2.4;

// ═══════════════════════════════════════════════════════════════════════════════
// LOS tables (Exhibits 23-10, 23-13, 23-14)
// ═══════════════════════════════════════════════════════════════════════════════

/// HCM Exhibit 23-10: LOS criteria for each O-D within signalized
/// interchanges.
///
/// * `ett_s` — experienced travel time ETT, s/veh
/// * `vc_gt_1` — true when v/c > 1 for any lane group containing this O-D
/// * `rq_gt_1` — true when the queue storage ratio R_Q > 1 for any lane
///   group containing this O-D
///
/// LOS is F when either condition flag is set, regardless of ETT. The ETT
/// thresholds reflect a control delay component greater by a factor of 1.5
/// than those for signalized intersections (Exhibit 23-10 discussion).
pub fn los_signalized_interchange_od(ett_s: f64, vc_gt_1: bool, rq_gt_1: bool) -> LevelOfService {
    if vc_gt_1 || rq_gt_1 {
        return LevelOfService::F;
    }
    match ett_s {
        e if e <= 15.0 => LevelOfService::A,
        e if e <= 30.0 => LevelOfService::B,
        e if e <= 55.0 => LevelOfService::C,
        e if e <= 85.0 => LevelOfService::D,
        e if e <= 120.0 => LevelOfService::E,
        _ => LevelOfService::F,
    }
}

/// HCM Exhibit 23-13: LOS criteria for each O-D within alternative
/// intersections (RCUT, MUT, DLT — milestone 2). Thresholds are identical
/// to conventional signalized intersections and 33% lower than the
/// interchange thresholds of Exhibit 23-10.
pub fn los_alternative_intersection_od(ett_s: f64, vc_gt_1: bool, rq_gt_1: bool) -> LevelOfService {
    if vc_gt_1 || rq_gt_1 {
        return LevelOfService::F;
    }
    match ett_s {
        e if e <= 10.0 => LevelOfService::A,
        e if e <= 20.0 => LevelOfService::B,
        e if e <= 35.0 => LevelOfService::C,
        e if e <= 55.0 => LevelOfService::D,
        e if e <= 80.0 => LevelOfService::E,
        _ => LevelOfService::F,
    }
}

/// HCM Exhibit 23-14: LOS criteria for each O-D of an interchange with
/// roundabouts. The v/c and R_Q flags are evaluated per roundabout
/// approach rather than per signalized lane group.
pub fn los_roundabout_interchange_od(ett_s: f64, vc_gt_1: bool, rq_gt_1: bool) -> LevelOfService {
    if vc_gt_1 || rq_gt_1 {
        return LevelOfService::F;
    }
    match ett_s {
        e if e <= 15.0 => LevelOfService::A,
        e if e <= 25.0 => LevelOfService::B,
        e if e <= 35.0 => LevelOfService::C,
        e if e <= 50.0 => LevelOfService::D,
        e if e <= 75.0 => LevelOfService::E,
        _ => LevelOfService::F,
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Interchange saturation flow adjustment No. 1: traffic pressure f_v
// (Equation 23-15, Exhibit 23-23)
// ═══════════════════════════════════════════════════════════════════════════════

/// HCM Equation 23-15: adjustment factor for traffic pressure
///
/// `f_v = 1 / (1.07 - 0.00672 min(v_i', 30))`  (left turn)
/// `f_v = 1 / (1.07 - 0.00486 min(v_i', 30))`  (through or right turn)
///
/// * `demand_per_cycle_per_lane` — demand flow rate per cycle per lane
///   v_i' (veh/cycle/ln)
/// * `left_turn` — true for a left-turn movement
///
/// Tabulated in Exhibit 23-23 (e.g., v' = 30: 1.152 left / 1.082 through).
/// When a lane group is shared by several movements, the factor is the
/// flow-weighted average of the respective movements (Chapter 23 text).
pub fn traffic_pressure_factor(demand_per_cycle_per_lane: f64, left_turn: bool) -> f64 {
    let v = demand_per_cycle_per_lane.min(TRAFFIC_PRESSURE_DEMAND_CAP).max(0.0);
    if left_turn {
        1.0 / (1.07 - 0.00672 * v)
    } else {
        1.0 / (1.07 - 0.00486 * v)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Interchange saturation flow adjustment No. 2: lane utilization f_LU
// (Equations 23-16 / 23-17 / 23-18, Exhibits 23-24 / 23-26)
// ═══════════════════════════════════════════════════════════════════════════════

/// HCM Equation 23-16: lane utilization adjustment factor
/// `f_LU = 1 / (%V_Lmax × N)`.
///
/// * `pct_v_lmax` — percent of the total approach flow in the lane with
///   the highest volume, expressed as a decimal
/// * `n_lanes` — number of lanes in the lane group N
pub fn lane_utilization_factor_from_max(pct_v_lmax: f64, n_lanes: u32) -> f64 {
    if pct_v_lmax <= 0.0 || n_lanes == 0 {
        return 1.0;
    }
    (1.0 / (pct_v_lmax * n_lanes as f64)).min(1.0)
}

/// Coefficient rows of HCM Exhibit 23-24 (external arterial approaches of
/// diamond and parclo interchanges). Each interchange type / approach
/// grouping provides coefficients a1..a3 of Equation 23-17 for the
/// leftmost lane (L1) and, for 3- and 4-lane groups, the rightmost lane
/// (Ln); the middle-lane share is estimated by subtraction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LaneUtilizationModel {
    /// Diamond interchanges (also applied to compressed / tight diamonds).
    Diamond,
    /// Parclo A-2Q (both directions).
    ParcloA2Q,
    /// Parclo B-2Q, B-4Q, and AB-4Q westbound approaches.
    ParcloB2QB4QAb4QWestbound,
    /// Parclo A-4Q, AB-2Q eastbound, and AB-4Q eastbound approaches.
    ParcloA4QAb2QEbAb4QEastbound,
    /// Parclo AB-2Q westbound approach.
    ParcloAB2QWestbound,
}

/// Exhibit 23-24 coefficients (a1, a2, a3) for the requested model, lane
/// group size (2, 3, or 4 lanes), and lane position. Returns `None` when
/// the exhibit provides no model (rightmost lane of a 2-lane group, or an
/// unsupported lane count).
///
/// * `leftmost` — true for lane L1; false for the rightmost lane Ln
pub fn lane_utilization_coefficients(
    model: LaneUtilizationModel,
    n_lanes: u32,
    leftmost: bool,
) -> Option<(f64, f64, f64)> {
    use LaneUtilizationModel::*;
    let row = match (model, n_lanes) {
        (Diamond, 2) => [Some((-0.154, 0.187, -0.181)), None],
        (Diamond, 3) => [Some((-0.245, 0.465, 0.0)), Some((0.609, -0.326, 0.0))],
        (Diamond, 4) => [Some((-0.328, 0.684, 0.0)), Some((0.640, -0.233, 0.0))],
        (ParcloA2Q, 2) => [Some((0.0, -0.527, 0.0)), None],
        (ParcloA2Q, 3) => [Some((0.0, -0.363, 0.0)), Some((0.0, 0.605, 0.0))],
        (ParcloA2Q, 4) => [Some((0.0, -0.257, 0.0)), Some((0.0, 0.747, 0.0))],
        (ParcloB2QB4QAb4QWestbound, 2) => [Some((0.387, -0.344, 0.0)), None],
        (ParcloB2QB4QAb4QWestbound, 3) => {
            [Some((0.559, -0.218, 0.0)), Some((-0.429, 0.695, 0.0))]
        }
        (ParcloB2QB4QAb4QWestbound, 4) => {
            [Some((0.643, -0.103, 0.0)), Some((-0.359, 0.794, 0.0))]
        }
        (ParcloA4QAb2QEbAb4QEastbound, 2) => [Some((-0.306, -0.484, 0.0)), None],
        (ParcloA4QAb2QEbAb4QEastbound, 3) => {
            [Some((-0.333, -0.289, 0.0)), Some((0.579, 0.428, 0.0))]
        }
        (ParcloA4QAb2QEbAb4QEastbound, 4) => {
            [Some((-0.233, -0.237, 0.0)), Some((0.703, 0.641, 0.0))]
        }
        (ParcloAB2QWestbound, 2) => [Some((0.468, 0.0, 0.0)), None],
        (ParcloAB2QWestbound, 3) => [Some((0.735, 0.0, 0.0)), Some((-0.308, 0.0, 0.0))],
        (ParcloAB2QWestbound, 4) => [Some((0.768, 0.0, 0.0)), Some((-0.202, 0.0, 0.0))],
        _ => [None, None],
    };
    if leftmost {
        row[0]
    } else {
        row[1]
    }
}

/// HCM Equation 23-17: percent of the total external-approach flow in lane
/// L_i of a diamond / parclo external arterial approach
///
/// `%V_Li = 1/n + a1 (v_R / (v_L+v_R+v_T)) + a2 (v_L / (v_L+v_R+v_T))
///          + a3 (D v_L / 10^6)`
///
/// * `coeffs` — (a1, a2, a3) from Exhibit 23-24
/// * `v_l` — O-D demand traveling through the first intersection and
///   turning left at the second, veh/h
/// * `v_r` — right-turning O-D demand of the approach (v_F or v_G from
///   Exhibit 23-20; 0 when an exclusive right-turn lane exists)
/// * `v_t` — O-D demand traveling through both intersections, veh/h
/// * `spacing_ft` — distance D between the two intersections (valid below
///   800 ft; beyond that the Chapter 19 defaults are recommended)
pub fn pct_volume_in_lane(
    coeffs: (f64, f64, f64),
    n_lanes: u32,
    v_l: f64,
    v_r: f64,
    v_t: f64,
    spacing_ft: f64,
) -> f64 {
    let total = v_l + v_r + v_t;
    let (a1, a2, a3) = coeffs;
    let base = 1.0 / n_lanes.max(1) as f64;
    if total <= 0.0 {
        return base;
    }
    base + a1 * (v_r / total) + a2 * (v_l / total) + a3 * (spacing_ft * v_l / 1.0e6)
}

/// Highest lane volume share %V_Lmax for a diamond / parclo external
/// arterial approach (Equation 23-17 with Exhibit 23-24), taking the
/// maximum over the modeled lanes (leftmost, rightmost, and the
/// by-subtraction middle lanes).
///
/// // VERIFY-HCM: Chapter 34 Example Problems 1 and 3 publish %V_Lmax
/// // values (Exhibits 34-6 and 34-33) that are not reproduced by
/// // Equation 23-17 with the Exhibit 23-24 coefficients as printed
/// // (e.g., Example 1 EB external: published 0.5056 vs. 0.4969 computed).
/// // Field-measured values may be supplied through the lane utilization
/// // override input instead.
pub fn pct_v_lmax_arterial(
    model: LaneUtilizationModel,
    n_lanes: u32,
    v_l: f64,
    v_r: f64,
    v_t: f64,
    spacing_ft: f64,
) -> f64 {
    let n = n_lanes.max(1);
    let base = 1.0 / n as f64;
    let left = lane_utilization_coefficients(model, n, true)
        .map(|c| pct_volume_in_lane(c, n, v_l, v_r, v_t, spacing_ft));
    let right = lane_utilization_coefficients(model, n, false)
        .map(|c| pct_volume_in_lane(c, n, v_l, v_r, v_t, spacing_ft));
    match (left, right) {
        (Some(l), Some(r)) => {
            // Middle lane(s) by subtraction (Exhibit 23-24 note), split
            // evenly when the group has 4 lanes.
            let middle = ((1.0 - l - r) / (n as f64 - 2.0).max(1.0)).max(0.0);
            l.max(r).max(middle)
        }
        (Some(l), None) => l.max(1.0 - l), // 2-lane group: other lane by subtraction
        _ => base,
    }
}

/// DDI external-crossover lane configurations of HCM Exhibit 23-25.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DdiLaneConfiguration {
    /// 2 lanes, left turn from a shared lane.
    TwoLaneShared,
    /// 3 lanes, left turn from a shared lane.
    ThreeLaneShared,
    /// 3 lanes, exclusive left-turn lane.
    ThreeLaneExclusive,
    /// 3 lanes, exclusive left with a middle shared lane.
    ThreeLaneExclusiveMiddleShared,
    /// 4 lanes, exclusive left-turn lane.
    FourLaneExclusive,
}

impl DdiLaneConfiguration {
    /// Number of lanes at the external crossover.
    pub fn lanes(self) -> u32 {
        match self {
            DdiLaneConfiguration::TwoLaneShared => 2,
            DdiLaneConfiguration::FourLaneExclusive => 4,
            _ => 3,
        }
    }
}

/// HCM Equation 23-18 with Exhibit 23-26: highest lane volume share
/// %V_Lmax for a DDI external crossover
///
/// `%V_Li,DDI = a1 × LTDR + a2`
///
/// * `ltdr` — left-turn demand ratio: left-turn demand at the external
///   crossover divided by the total approach volume (decimal)
///
/// The regime (and therefore the modeled lane) is selected by the LTDR
/// breakpoint of Exhibit 23-26; only the highest-volume lane is needed.
pub fn ddi_pct_v_lmax(config: DdiLaneConfiguration, ltdr: f64) -> f64 {
    use DdiLaneConfiguration::*;
    let (a1, a2) = match config {
        TwoLaneShared => {
            if ltdr <= 0.35 {
                (0.2129, 0.5250)
            } else {
                (0.5386, 0.4110)
            }
        }
        ThreeLaneShared => {
            if ltdr <= 0.13 {
                (-0.1831, 0.3863)
            } else if ltdr <= 0.43 {
                (0.2245, 0.3336)
            } else {
                (0.6460, 0.1523)
            }
        }
        ThreeLaneExclusive => {
            if ltdr <= 0.33 {
                (-0.5983, 0.5237)
            } else {
                (0.9695, 0.0096)
            }
        }
        ThreeLaneExclusiveMiddleShared => {
            if ltdr <= 0.50 {
                (-0.2884, 0.5626)
            } else {
                (0.4903, 0.1761)
            }
        }
        FourLaneExclusive => {
            if ltdr <= 0.35 {
                (-0.5432, 0.5095)
            } else {
                (0.9286, -0.0071)
            }
        }
    };
    (a1 * ltdr + a2).max(1.0 / config.lanes() as f64)
}

// ═══════════════════════════════════════════════════════════════════════════════
// Interchange saturation flow adjustment No. 4: turn radius f_R
// (Equations 23-19 through 23-23, Exhibit 23-27)
// ═══════════════════════════════════════════════════════════════════════════════

/// HCM Equation 23-19: adjustment factor for travel path radius
/// `f_R = 1 / (1 + 5.61/R)` with R the radius of curvature of the turning
/// path (at the center of the path), ft. Tabulated in Exhibit 23-27
/// (e.g., R = 50 ft: 0.899; R = 75 ft: 0.930).
pub fn turn_radius_factor(radius_ft: f64) -> f64 {
    if radius_ft <= 0.0 {
        return 1.0;
    }
    1.0 / (1.0 + 5.61 / radius_ft)
}

/// HCM Equations 23-20 / 23-21: left-turn saturation flow adjustment for
/// interchanges. For a protected exclusive left-turn lane f_LT = f_R
/// (Equation 23-20); for a protected shared lane
/// `f_LT = 1 / (1 + P_LT (1/f_R − 1))` (Equation 23-21).
///
/// * `p_lt` — proportion of left turns in the lane group (1.0 for an
///   exclusive lane)
pub fn left_turn_radius_adjustment(p_lt: f64, f_r: f64) -> f64 {
    if f_r <= 0.0 {
        return 1.0;
    }
    if p_lt >= 1.0 {
        return f_r; // Eq. 23-20
    }
    1.0 / (1.0 + p_lt.max(0.0) * (1.0 / f_r - 1.0)) // Eq. 23-21
}

/// HCM Equations 23-22 / 23-23: right-turn saturation flow adjustment for
/// interchanges (f_RT = f_R exclusive; shared form mirrors Equation 23-21
/// with P_RT).
pub fn right_turn_radius_adjustment(p_rt: f64, f_r: f64) -> f64 {
    left_turn_radius_adjustment(p_rt, f_r)
}

// ═══════════════════════════════════════════════════════════════════════════════
// Exhibit 34-161: O-D demands contributing to each movement of an
// interchange with roundabouts
// ═══════════════════════════════════════════════════════════════════════════════

/// O-D demand letters of Exhibit 23-20 / Exhibit 34-162.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OdMovement {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
}

impl OdMovement {
    /// All fourteen O-D letters in order.
    pub const ALL: [OdMovement; 14] = [
        OdMovement::A,
        OdMovement::B,
        OdMovement::C,
        OdMovement::D,
        OdMovement::E,
        OdMovement::F,
        OdMovement::G,
        OdMovement::H,
        OdMovement::I,
        OdMovement::J,
        OdMovement::K,
        OdMovement::L,
        OdMovement::M,
        OdMovement::N,
    ];
}

/// Interchange forms whose roundabout movement composition is listed in
/// HCM Exhibit 34-161.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RoundaboutInterchangeForm {
    Diamond,
    ParcloA2Q,
    ParcloB2Q,
    ParcloB4Q,
    Spui,
    ParcloAB4Q,
    ParcloA4Q,
    ParcloAB2Q,
}

/// HCM Exhibit 34-161: the O-D demands (Exhibit 34-160 letters) that
/// constitute roundabout movement `movement_no` (1–16) for the given
/// interchange form. Returns `None` for movements that do not exist for
/// the form. The movement numbering follows the Exhibit 34-160
/// illustration of an interchange with two roundabouts.
pub fn roundabout_movement_ods(
    form: RoundaboutInterchangeForm,
    movement_no: u8,
) -> Option<&'static [OdMovement]> {
    use OdMovement::*;
    use RoundaboutInterchangeForm as F;
    let table: &'static [OdMovement] = match (form, movement_no) {
        (F::Diamond, 1) => &[C, D, L, N],
        (F::Diamond, 2) => &[D, H, L, M, N],
        (F::Diamond, 3) => &[E, F, I],
        (F::Diamond, 4) => &[D, E, F, H, I, L, M, N],
        (F::Diamond, 7) => &[A, H, J, M],
        (F::Diamond, 8) => &[J, M],
        (F::Diamond, 11) => &[D, E, I, N],
        (F::Diamond, 12) => &[D, E, I, N],
        (F::Diamond, 13) => &[A, B, K, M],
        (F::Diamond, 14) => &[A, E, K, M, N],
        (F::Diamond, 15) => &[G, H, J],
        (F::Diamond, 16) => &[A, E, G, H, J, K, M, N],
        (F::ParcloA2Q, 1) => &[C, D, N],
        (F::ParcloA2Q, 2) => &[D, N],
        (F::ParcloA2Q, 3) => &[E, F],
        (F::ParcloA2Q, 4) => &[D, E, F, I, N],
        (F::ParcloA2Q, 6) => &[F],
        (F::ParcloA2Q, 7) => &[A, H, J, M],
        (F::ParcloA2Q, 8) => &[A, F, H, J, M],
        (F::ParcloA2Q, 10) => &[G],
        (F::ParcloA2Q, 11) => &[D, E, I, N],
        (F::ParcloA2Q, 12) => &[D, E, G, I, N],
        (F::ParcloA2Q, 13) => &[A, B, M],
        (F::ParcloA2Q, 14) => &[A, M],
        (F::ParcloA2Q, 15) => &[G, H, J],
        (F::ParcloA2Q, 16) => &[A, G, H, J, M],
        (F::ParcloB2Q, 2) => &[H, M, N],
        (F::ParcloB2Q, 3) => &[E, F, I],
        (F::ParcloB2Q, 4) => &[E, F, H, I, M],
        (F::ParcloB2Q, 5) => &[C],
        (F::ParcloB2Q, 6) => &[C],
        (F::ParcloB2Q, 7) => &[A, H, J, M],
        (F::ParcloB2Q, 8) => &[A, C, H, J, M],
        (F::ParcloB2Q, 9) => &[A, B, M],
        (F::ParcloB2Q, 10) => &[B],
        (F::ParcloB2Q, 11) => &[D, E, I, N],
        (F::ParcloB2Q, 12) => &[B, D, E],
        (F::ParcloB2Q, 14) => &[E, N],
        (F::ParcloB2Q, 15) => &[G, H, J],
        (F::ParcloB2Q, 16) => &[E, G, H, J, N],
        (F::ParcloB4Q, 1) => &[C],
        (F::ParcloB4Q, 2) => &[H, M],
        (F::ParcloB4Q, 3) => &[E, F, I],
        (F::ParcloB4Q, 4) => &[E, F, H, I, M],
        (F::ParcloB4Q, 5) => &[D, N],
        (F::ParcloB4Q, 7) => &[A, H, J, M],
        (F::ParcloB4Q, 8) => &[A, H, J, M],
        (F::ParcloB4Q, 9) => &[A, M],
        (F::ParcloB4Q, 11) => &[D, E, I, N],
        (F::ParcloB4Q, 12) => &[D, E, I, N],
        (F::ParcloB4Q, 13) => &[B],
        (F::ParcloB4Q, 14) => &[E, N],
        (F::ParcloB4Q, 15) => &[G, H, J],
        (F::ParcloB4Q, 16) => &[E, G, H, J, N],
        (F::Spui, 1) => &[C, D, L, N],
        (F::Spui, 2) => &[D, H, L, M, N],
        (F::Spui, 3) => &[E, F, I],
        (F::Spui, 4) => &[D, E, I, N],
        (F::Spui, 5) => &[A, B, K, M],
        (F::Spui, 6) => &[A, E, K, M, N],
        (F::Spui, 7) => &[G, H, J],
        (F::Spui, 8) => &[A, H, J, M],
        (F::ParcloAB4Q, 1) => &[C],
        (F::ParcloAB4Q, 2) => &[H, M],
        (F::ParcloAB4Q, 3) => &[E, F, I],
        (F::ParcloAB4Q, 4) => &[E, F, H, I, M],
        (F::ParcloAB4Q, 5) => &[D, N],
        (F::ParcloAB4Q, 7) => &[A, H, J, M],
        (F::ParcloAB4Q, 8) => &[A, H, J, M],
        (F::ParcloAB4Q, 11) => &[D, E, I, N],
        (F::ParcloAB4Q, 12) => &[D, E, I, N],
        (F::ParcloAB4Q, 13) => &[A, B, M],
        (F::ParcloAB4Q, 14) => &[A, M],
        (F::ParcloAB4Q, 15) => &[G, H, J],
        (F::ParcloAB4Q, 16) => &[A, G, H, J, M],
        (F::ParcloA4Q, 1) => &[C, D, N],
        (F::ParcloA4Q, 2) => &[D, N],
        (F::ParcloA4Q, 3) => &[E, F, I],
        (F::ParcloA4Q, 4) => &[D, E, F, I, N],
        (F::ParcloA4Q, 7) => &[A, H, J, M],
        (F::ParcloA4Q, 8) => &[A, H, J, M],
        (F::ParcloA4Q, 11) => &[D, E, I, N],
        (F::ParcloA4Q, 12) => &[D, E, I, N],
        (F::ParcloA4Q, 13) => &[A, B, M],
        (F::ParcloA4Q, 14) => &[A, M],
        (F::ParcloA4Q, 15) => &[G, H, J],
        (F::ParcloA4Q, 16) => &[A, G, H, J, M],
        (F::ParcloAB2Q, 2) => &[H, M],
        (F::ParcloAB2Q, 3) => &[E, F, I],
        (F::ParcloAB2Q, 4) => &[E, F, H, I, M],
        (F::ParcloAB2Q, 5) => &[C, D, N],
        (F::ParcloAB2Q, 6) => &[C],
        (F::ParcloAB2Q, 7) => &[A, H, J, M],
        (F::ParcloAB2Q, 8) => &[A, C, H, J, M],
        (F::ParcloAB2Q, 10) => &[G],
        (F::ParcloAB2Q, 11) => &[D, E, I, N],
        (F::ParcloAB2Q, 12) => &[D, E, G, I, N],
        (F::ParcloAB2Q, 13) => &[A, B, M],
        (F::ParcloAB2Q, 14) => &[A, M],
        (F::ParcloAB2Q, 15) => &[G, H, J],
        (F::ParcloAB2Q, 16) => &[A, G, H, J, M],
        _ => return None,
    };
    Some(table)
}
