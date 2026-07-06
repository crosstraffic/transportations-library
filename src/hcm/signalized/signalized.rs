//! HCM Chapter 19 motorized vehicle methodology for signalized
//! intersections (pretimed / coordinated operation, milestone 1).
//!
//! The computational steps follow HCM 7th Edition Exhibit 19-18:
//!
//! 1. Determine movement groups and lane groups (Exhibit 19-19 rules)
//! 2. Determine movement group flow rate
//! 3. Determine lane group flow rate (Chapter 31, Section 2,
//!    "Lane Group Flow Rate on Multiple-Lane Approaches",
//!    Equations 31-46 through 31-66)
//! 4. Determine adjusted saturation flow rate (Equation 19-8 with the
//!    Chapter 31 permitted-turn and pedestrian–bicycle supplements)
//! 5. Determine proportion arriving during green (Equation 19-15)
//! 6. Determine signal phase duration (input for pretimed/coordinated
//!    control; the Chapter 31 pretimed design procedure is provided as
//!    free functions; the actuated estimation loop is milestone 2)
//! 7. Determine capacity and volume-to-capacity ratio (Equations 19-16,
//!    19-17, 19-30, and 31-117 through 31-127)
//! 8. Determine delay (Equations 19-18 through 19-27 via
//!    `hcm::common::delay`, plus the Chapter 19 Section 4 incremental
//!    queue accumulation procedure for permitted left turns)
//! 9. Determine LOS (Exhibit 19-8 via `hcm::common::los_tables`)
//! 10. Determine queue storage ratio (Chapter 31, Section 4,
//!     Equations 31-130 through 31-156)

use serde::{Deserialize, Serialize};

use super::actuated::{
    estimate_fully_actuated, ActuatedLaneGroupInput, ActuatedPhaseInput, ActuatedPhaseResult,
    MahLaneGroup,
};
use super::exhibits::*;
use crate::hcm::common::delay::{
    aggregate_control_delay, control_delay_signalized, incremental_delay_factor_actuated,
    incremental_delay_factor_min, incremental_delay_signalized, initial_queue_delay,
    progression_factor, uniform_delay, K_PRETIMED,
};
use crate::hcm::common::intersection::{ControlType, Direction};
use crate::hcm::common::los_tables::los_signalized_intersection;
use crate::hcm::common::LevelOfService;

// ═══════════════════════════════════════════════════════════════════════════════
// Input data model
// ═══════════════════════════════════════════════════════════════════════════════

/// Left-turn operational mode (HCM Exhibit 19-11, "Left-turn operational
/// mode" input).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LeftTurnMode {
    /// No left-turn movement on the approach.
    NotPresent,
    /// Left turns are served only by a protected phase.
    Protected,
    /// Left turns filter through the opposing flow during the through phase.
    Permitted,
    /// Protected phase plus a permitted period.
    ProtectedPermitted,
}

/// Phase sequence of the subject left-turn phase relative to the opposing
/// left-turn phase (rows of HCM Exhibit 31-12, Unblocked Permitted Green
/// Time). The first term describes the subject left turn, the second the
/// opposing left turn (`Perm` = no protected phase).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LeftTurnSequence {
    LeadLead,
    LeadLag,
    LagLead,
    LagLag,
    PermLead,
    PermLag,
    PermPerm,
}

/// Timing of one signal phase (input for pretimed and coordinated
/// operation; converged average durations may be supplied for actuated
/// control).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhaseTiming {
    /// Signal phase number (NEMA numbering, HCM Exhibit 19-1).
    pub phase_no: u8,
    /// Phase duration D_p = G + Y + R_c, s (HCM Equation 19-2).
    pub duration_s: f64,
    /// Yellow change interval Y, s.
    pub yellow_s: f64,
    /// Red clearance interval R_c, s.
    pub red_clearance_s: f64,
    /// Maximum green setting G_max, s (actuated control; used for the
    /// available capacity c_a of HCM Equations 19-24/19-25 and the
    /// incremental delay factor k of Equation 19-22). `None` for
    /// pretimed phases.
    pub max_green_s: Option<f64>,
    /// Passage time setting PT, s (actuated control; HCM Equation 19-23).
    pub passage_time_s: Option<f64>,
    /// Walk interval, s (used for the pedestrian service time g_ped of the
    /// Chapter 31 pedestrian–bicycle adjustment procedure).
    pub walk_s: Option<f64>,
    /// Pedestrian clear interval, s.
    pub ped_clear_s: Option<f64>,
    /// Minimum green setting G_min, s (actuated control; HCM Equation 31-35).
    /// Defaults to 5.0 s when absent.
    #[serde(default)]
    pub min_green_s: Option<f64>,
    /// Stop-line detection zone length L_ds, ft (actuated control;
    /// HCM Equation 31-10). Defaults to 40.0 ft when absent.
    #[serde(default)]
    pub detector_length_ft: Option<f64>,
    /// Phase set on recall to maximum (actuated control; forces the green to
    /// its maximum every cycle, HCM Equation 31-34 discussion). Defaults to
    /// false.
    #[serde(default)]
    pub recall_max: bool,
}

impl PhaseTiming {
    /// Effective green time `g = D_p - l_1 - l_2` (HCM Equation 19-3), with
    /// `l_1` = 2.0 s and `l_2 = Y + R_c - e` (HCM Equation 19-1).
    pub fn effective_green_s(&self) -> f64 {
        let l2 = self.yellow_s + self.red_clearance_s - EXTENSION_OF_EFFECTIVE_GREEN;
        (self.duration_s - START_UP_LOST_TIME - l2).max(0.0)
    }

    /// Phase lost time `l_t = l_1 + Y + R_c - e` (HCM Equation 19-1).
    pub fn lost_time_s(&self) -> f64 {
        START_UP_LOST_TIME + self.yellow_s + self.red_clearance_s - EXTENSION_OF_EFFECTIVE_GREEN
    }

    /// Change period `CP = Y + R_c` (HCM Exhibit 19-6).
    pub fn change_period_s(&self) -> f64 {
        self.yellow_s + self.red_clearance_s
    }

    /// Available effective green `g_a = G_max + Y + R_c - l_1 - l_2`
    /// (HCM Equation 19-25). Falls back to the effective green when no
    /// maximum green is set (pretimed phases).
    pub fn available_effective_green_s(&self) -> f64 {
        match self.max_green_s {
            Some(gmax) => {
                (gmax + self.change_period_s()
                    - START_UP_LOST_TIME
                    - (self.change_period_s() - EXTENSION_OF_EFFECTIVE_GREEN))
                    .max(0.0)
            }
            None => self.effective_green_s(),
        }
    }
}

/// One signalized intersection approach: demand, geometry, and signal
/// control inputs of HCM Exhibits 19-11 and 19-12 (movement/approach basis).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalApproach {
    /// Direction of travel of the approach.
    pub direction: Direction,

    // ── Traffic characteristics ────────────────────────────────────────────
    /// Left-turn demand volume, veh/h.
    pub volume_left: f64,
    /// Through demand volume, veh/h.
    pub volume_through: f64,
    /// Right-turn demand volume, veh/h.
    pub volume_right: f64,
    /// Right-turn-on-red flow rate, veh/h (subtracted from the right-turn
    /// demand per HCM Chapter 19, Step 2; default 0.0).
    pub volume_rtor: f64,
    /// Peak hour factor applied to the three demand volumes. `None` if the
    /// volumes are already analysis-period flow rates.
    pub peak_hour_factor: Option<f64>,
    /// Percentage heavy vehicles for the left-turn movement group, %.
    pub pct_heavy_vehicles_left: f64,
    /// Percentage heavy vehicles for the through/right movement group, %.
    pub pct_heavy_vehicles_through: f64,
    /// Platoon ratio R_p for the left-turn movement group
    /// (HCM Exhibit 19-13; 1.00 = random arrivals).
    pub platoon_ratio_left: f64,
    /// Platoon ratio R_p for the through movement group.
    pub platoon_ratio_through: f64,
    /// Upstream filtering adjustment factor I (HCM Equation 19-6;
    /// 1.0 for isolated intersections).
    pub upstream_filtering_i: f64,
    /// Initial queue at the start of the analysis period for the
    /// through/right movement group Q_b, veh.
    pub initial_queue_through_veh: f64,
    /// Initial queue for the left-turn movement group Q_b, veh.
    pub initial_queue_left_veh: f64,
    /// Pedestrian flow rate in the conflicting crosswalk, p/h (two-way).
    pub ped_flow_ph: f64,
    /// Bicycle flow rate conflicting with right turns, bicycles/h.
    pub bike_flow_ph: f64,

    // ── Geometric design ───────────────────────────────────────────────────
    /// Number of exclusive left-turn lanes (0 if the left turn is shared or
    /// not present).
    pub exclusive_left_lanes: u32,
    /// Number of exclusive through lanes.
    pub through_lanes: u32,
    /// Number of exclusive right-turn lanes.
    pub exclusive_right_lanes: u32,
    /// Whether a shared left-turn/through lane is present (one additional
    /// lane).
    pub shared_left_through_lane: bool,
    /// Whether a shared right-turn/through lane is present (one additional
    /// lane).
    pub shared_right_through_lane: bool,
    /// Average lane width, ft (HCM Exhibit 19-20).
    pub lane_width_ft: f64,
    /// Approach grade, % (positive upgrade).
    pub grade_pct: f64,
    /// Number of receiving lanes for turning vehicles (HCM Equations 31-81
    /// and 31-82 compare receiving lanes with turn lanes).
    pub receiving_lanes: u32,
    /// On-street parking adjacent to the curb lane group.
    pub parking_present: bool,
    /// Parking maneuver rate N_m, maneuvers/h (HCM Equation 19-11).
    pub parking_maneuvers_h: f64,
    /// Local bus stopping rate N_b, buses/h (HCM Equation 19-12).
    pub bus_stops_h: f64,
    /// Available queue storage for the left-turn lane group L_a, ft/ln
    /// (turn bay length; HCM Equation 31-154). `None` if unbounded.
    pub storage_left_ft: Option<f64>,
    /// Available queue storage for through lane groups, ft/ln (typically
    /// the segment length). `None` if unbounded.
    pub storage_through_ft: Option<f64>,
    /// Posted speed limit, mi/h (HCM Equation 31-132).
    pub speed_limit_mph: f64,
    /// Whether an exclusive right-turn lane on the opposing approach
    /// influences the subject left-turn drivers' gap acceptance
    /// (HCM Chapter 31, Section 3, Step 3: Case 1 when `true` — the
    /// default — and Case 2 when `false`).
    pub opposing_right_turn_influences_gaps: bool,

    // ── Signal control ─────────────────────────────────────────────────────
    /// Left-turn operational mode.
    pub left_turn_mode: LeftTurnMode,
    /// Sequence of the subject vs. opposing left-turn phase
    /// (HCM Exhibit 31-12 rows).
    pub left_turn_sequence: LeftTurnSequence,
    /// Protected left-turn phase timing (`None` if the left turn is
    /// permitted-only or not present).
    pub left_phase: Option<PhaseTiming>,
    /// Phase serving the through (and right-turn / permitted left-turn)
    /// movements.
    pub through_phase: PhaseTiming,
}

impl SignalApproach {
    /// Peak-hour-factor-adjusted demand flow rates (v = V / PHF), veh/h.
    fn flow_rates(&self) -> (f64, f64, f64) {
        let phf = match self.peak_hour_factor {
            Some(p) if p > 0.0 => p,
            _ => 1.0,
        };
        (
            self.volume_left / phf,
            self.volume_through / phf,
            self.volume_right / phf,
        )
    }

    /// Total number of lanes serving through vehicles (shared or
    /// exclusive), N_th of HCM Equation 31-46.
    fn through_serving_lanes(&self) -> u32 {
        self.through_lanes
            + u32::from(self.shared_left_through_lane)
            + u32::from(self.shared_right_through_lane)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Lane group results
// ═══════════════════════════════════════════════════════════════════════════════

/// Lane group designation per HCM Exhibit 19-19.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LaneGroupKind {
    ExclusiveLeft,
    SharedLeftThrough,
    ExclusiveThrough,
    SharedRightThrough,
    ExclusiveRight,
}

/// Computed performance of one lane group. `Option` fields are filled by
/// the corresponding methodology step.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaneGroup {
    /// Approach the lane group belongs to.
    pub direction: Direction,
    /// Lane group designation (HCM Exhibit 19-19).
    pub kind: LaneGroupKind,
    /// Controlling signal phase number.
    pub phase_no: u8,
    /// Number of lanes in the lane group N, ln.
    pub lanes: u32,
    /// Demand flow rate v, veh/h (Step 3).
    pub flow_rate: f64,
    /// Adjusted saturation flow rate s, veh/h/ln (Step 4; protected-period
    /// rate for protected-permitted lane groups).
    pub sat_flow: Option<f64>,
    /// Saturation flow rate during the permitted period s_l
    /// (HCM Equation 31-110) or shared-lane average s_sl (Equation 31-122),
    /// veh/h/ln.
    pub sat_flow_permitted: Option<f64>,
    /// Effective green time g, s (protected-phase green for
    /// protected-permitted left-turn lane groups).
    pub effective_green_s: Option<f64>,
    /// Effective green time for permitted left-turn operation g_p, s
    /// (HCM Equation 31-94).
    pub g_p: Option<f64>,
    /// Permitted green not blocked by the opposing queue g_u, s
    /// (HCM Equation 31-95).
    pub g_u: Option<f64>,
    /// Time before the first left-turning vehicle blocks a shared lane
    /// g_f, s (HCM Equations 31-97/31-98).
    pub g_f: Option<f64>,
    /// Proportion of left-turning vehicles in the shared lane P_L.
    pub p_left_shared: Option<f64>,
    /// Proportion of right-turning vehicles in the shared lane P_R.
    pub p_right_shared: Option<f64>,
    /// Proportion arriving during green P (HCM Equation 19-15; Step 5).
    pub proportion_on_green: Option<f64>,
    /// Lane group capacity c, veh/h (Step 7).
    pub capacity: Option<f64>,
    /// Available capacity c_a, veh/h (HCM Equation 19-24 family).
    pub available_capacity: Option<f64>,
    /// Volume-to-capacity ratio X (HCM Equation 19-17).
    pub vc_ratio: Option<f64>,
    /// Incremental delay factor k (HCM Equations 19-22/19-23; 0.50 for
    /// pretimed and coordinated phases).
    pub k_factor: Option<f64>,
    /// Uniform delay d1, s/veh (Step 8A).
    pub uniform_delay_s: Option<f64>,
    /// Incremental delay d2, s/veh (Step 8D).
    pub incremental_delay_s: Option<f64>,
    /// Initial queue delay d3, s/veh (Step 8B).
    pub initial_queue_delay_s: Option<f64>,
    /// Control delay d = d1 + d2 + d3, s/veh (HCM Equation 19-18).
    pub control_delay_s: Option<f64>,
    /// Level of service (HCM Exhibit 19-8).
    pub los: Option<LevelOfService>,
    /// First-term back of queue Q1, veh/ln (HCM Equation 31-141).
    pub q1_veh: Option<f64>,
    /// Second-term back of queue Q2, veh/ln (HCM Equation 31-142).
    pub q2_veh: Option<f64>,
    /// Third-term back of queue Q3, veh/ln (HCM Equation 31-143).
    pub q3_veh: Option<f64>,
    /// 50th percentile back of queue Q = Q1 + Q2 + Q3, veh/ln
    /// (HCM Equation 31-149).
    pub back_of_queue_veh: Option<f64>,
    /// 95th percentile back of queue Q_95%, veh/ln (HCM Equation 31-150
    /// with z = 1.64).
    pub back_of_queue_95_veh: Option<f64>,
    /// 50th percentile queue storage ratio R_Q (HCM Equation 31-154).
    pub queue_storage_ratio: Option<f64>,
    /// 95th percentile queue storage ratio RQ_95% (HCM Equation 31-156).
    pub queue_storage_ratio_95: Option<f64>,
}

impl LaneGroup {
    fn new(direction: Direction, kind: LaneGroupKind, phase_no: u8, lanes: u32, v: f64) -> Self {
        LaneGroup {
            direction,
            kind,
            phase_no,
            lanes,
            flow_rate: v,
            sat_flow: None,
            sat_flow_permitted: None,
            effective_green_s: None,
            g_p: None,
            g_u: None,
            g_f: None,
            p_left_shared: None,
            p_right_shared: None,
            proportion_on_green: None,
            capacity: None,
            available_capacity: None,
            vc_ratio: None,
            k_factor: None,
            uniform_delay_s: None,
            incremental_delay_s: None,
            initial_queue_delay_s: None,
            control_delay_s: None,
            los: None,
            q1_veh: None,
            q2_veh: None,
            q3_veh: None,
            back_of_queue_veh: None,
            back_of_queue_95_veh: None,
            queue_storage_ratio: None,
            queue_storage_ratio_95: None,
        }
    }
}

/// Aggregated approach results (HCM Equation 19-28 and Exhibit 19-8).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApproachResult {
    pub direction: Direction,
    /// Approach flow rate, veh/h.
    pub flow_rate: f64,
    /// Approach control delay d_A, s/veh (HCM Equation 19-28).
    pub control_delay_s: f64,
    /// Approach LOS (HCM Exhibit 19-8; delay-only basis per the exhibit
    /// note).
    pub los: LevelOfService,
}

// ═══════════════════════════════════════════════════════════════════════════════
// Facility struct
// ═══════════════════════════════════════════════════════════════════════════════

/// A signalized intersection analyzed with the HCM Chapter 19 motorized
/// vehicle methodology (pretimed / coordinated operation).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalizedIntersection {
    /// Cycle length C, s.
    pub cycle_length_s: f64,
    /// Analysis period duration T, h (0.25 h for a 15-min period,
    /// HCM Exhibit 19-11).
    pub analysis_period_h: f64,
    /// Base saturation flow rate s_o, pc/h/ln (HCM Exhibit 19-11 default:
    /// 1,900 pc/h/ln for metro areas >= 250,000 population).
    pub base_saturation_flow: f64,
    /// Area type: true if the intersection is in a CBD-like environment
    /// (HCM Chapter 19, Adjustment for Area Type; f_a = 0.90).
    pub area_type_cbd: bool,
    /// Traffic control type. Pretimed and coordinated (fixed timing) are
    /// fully supported; for `ActuatedSignal` / `SemiActuatedSignal` the
    /// supplied phase durations are treated as converged average durations
    /// and the incremental delay factor k is computed from
    /// `max_green_s` / `passage_time_s` (HCM Equations 19-22 through
    /// 19-25). The actuated phase-duration estimation loop itself is a
    /// milestone-2 item.
    pub control: ControlType,
    /// Number of left-turn sneakers per cycle n_s (HCM Chapter 31,
    /// Section 3; default 2.0).
    pub sneakers_per_cycle: f64,
    /// The intersection approaches (typically 4; 3 for a T intersection).
    pub approaches: Vec<SignalApproach>,

    // ── Computed results ───────────────────────────────────────────────────
    /// Lane groups with per-step results (filled by `analyze`).
    #[serde(default)]
    pub lane_groups: Vec<LaneGroup>,
    /// Approach aggregates (filled by Step 9).
    #[serde(default)]
    pub approach_results: Vec<ApproachResult>,
    /// Intersection control delay d_I, s/veh (HCM Equation 19-29).
    #[serde(default)]
    pub intersection_delay_s: Option<f64>,
    /// Intersection LOS (HCM Exhibit 19-8).
    #[serde(default)]
    pub intersection_los: Option<LevelOfService>,
    /// Critical intersection volume-to-capacity ratio X_c
    /// (HCM Equation 19-30).
    #[serde(default)]
    pub critical_vc_ratio: Option<f64>,
}

// ═══════════════════════════════════════════════════════════════════════════════
// Permitted-movement building blocks (HCM Chapter 31, Sections 2 and 3)
// ═══════════════════════════════════════════════════════════════════════════════

/// HCM Equation 31-100: saturation flow rate of a permitted left-turn
/// movement
///
/// `s_p = v_o e^(-v_o t_cg / 3,600) / (1 - e^(-v_o t_fh / 3,600))`
///
/// * `v_o` — opposing demand flow rate, veh/h (0.1 veh/h substituted when
///   zero, per the Chapter 31 Section 3 text)
///
/// Returns s_p in veh/h/ln, with t_cg = 4.5 s and t_fh = 2.5 s.
pub fn permitted_left_saturation_flow(v_o: f64) -> f64 {
    let v_o = if v_o <= 0.0 { 0.1 } else { v_o };
    v_o * (-v_o * CRITICAL_HEADWAY_PERMITTED_LEFT / 3_600.0).exp()
        / (1.0 - (-v_o * FOLLOW_UP_HEADWAY_PERMITTED_LEFT / 3_600.0).exp())
}

/// HCM Equation 31-101: through-car equivalent of a permitted left turn
/// `EL1 = s_o / s_p`.
pub fn el1_permitted_left(base_sat_flow: f64, s_p: f64) -> f64 {
    if s_p <= 0.0 {
        return E_L_PROTECTED_LEFT;
    }
    base_sat_flow / s_p
}

/// HCM Equations 31-102 and 31-103: through-car equivalent for a permitted
/// left turn opposed by a queue on a single-lane approach
///
/// `EL2 = (1 - (1 - P_lto)^n_q) / P_lto >= E_L` with
/// `n_q = 0.278 (g_p - g_u - g_f) >= 0.0`
///
/// * `p_lto` — proportion of left-turning vehicles in the opposing traffic
///   stream (decimal)
pub fn el2_permitted_left(p_lto: f64, g_p: f64, g_u: f64, g_f: f64) -> f64 {
    let n_q = (0.278 * (g_p - g_u - g_f)).max(0.0);
    if p_lto <= 0.0 {
        return E_L_PROTECTED_LEFT;
    }
    ((1.0 - (1.0 - p_lto).powf(n_q)) / p_lto).max(E_L_PROTECTED_LEFT)
}

/// HCM Equations 31-96 through 31-99: time before the first left-turning
/// vehicle arrives and blocks a shared lane, g_f.
///
/// `g_f,max = (1 - P_L)/(0.5 P_L) (1 - (1 - P_L)^(0.5 g_p)) - l_1,p >= 0`  (Eq. 31-96)
/// one-lane approach: `g_f = max(G_p e^(-0.860 LTC^0.629) - l_1,p, 0) <= g_f,max` (Eq. 31-97)
/// multilane approach: `g_f = max(G_p e^(-0.882 LTC^0.717) - l_1,p, 0) <= g_f,max` (Eq. 31-98)
/// `LTC = v_lt C / 3,600` (Eq. 31-99)
pub fn time_before_first_left_blocks(
    g_p_displayed: f64,
    g_p: f64,
    l1p: f64,
    p_l: f64,
    v_lt: f64,
    cycle_s: f64,
    single_lane_approach: bool,
) -> f64 {
    let ltc = (v_lt * cycle_s / 3_600.0).max(0.0);
    let gf_max = if p_l > 0.0 && p_l < 1.0 {
        ((1.0 - p_l) / (0.5 * p_l) * (1.0 - (1.0 - p_l).powf(0.5 * g_p)) - l1p).max(0.0)
    } else if p_l <= 0.0 {
        g_p
    } else {
        0.0
    };
    let g_f = if single_lane_approach {
        (g_p_displayed * (-0.860 * ltc.powf(0.629)).exp() - l1p).max(0.0)
    } else {
        (g_p_displayed * (-0.882 * ltc.powf(0.717)).exp() - l1p).max(0.0)
    };
    g_f.min(gf_max)
}

/// Result of the HCM Exhibit 31-12 unblocked permitted green time lookup.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PermittedGreen {
    /// Effective green time for permitted left-turn operation g_p, s
    /// (HCM Equation 31-94).
    pub g_p: f64,
    /// Displayed green interval corresponding to g_p, s.
    pub g_p_displayed: f64,
    /// Permitted green not blocked by the opposing queue g_u, s
    /// (HCM Equation 31-95).
    pub g_u: f64,
    /// Permitted start-up lost time l_1,p, s (Exhibit 31-12).
    pub l1p: f64,
    /// Permitted extension of effective green e_p, s (Exhibit 31-12).
    pub e_p: f64,
}

/// HCM Exhibit 31-12 with Equations 31-94 and 31-95: displayed and
/// effective permitted green times for a left-turn movement (non-Dallas
/// phasing).
///
/// * `seq` — subject/opposing left-turn phase sequence (Exhibit 31-12 rows)
/// * `dp_own_left` / `dp_opp_left` — left-turn phase durations D_p, s
///   (0 when the phase does not exist)
/// * `dp_own_through` / `dp_opp_through` — through-phase durations, s
/// * `cp_own_left`, `cp_own_through`, `cp_opp_through` — change periods
///   (Y + R_c), s
/// * `g_q` — opposing-queue clearance Gq (= worst opposing-lane g_s + l_1,
///   Exhibit 31-12 note a), s
#[allow(clippy::too_many_arguments)] // mirrors the HCM equation parameter list
pub fn permitted_green_times(
    seq: LeftTurnSequence,
    dp_own_left: f64,
    dp_opp_left: f64,
    dp_own_through: f64,
    dp_opp_through: f64,
    cp_own_left: f64,
    cp_own_through: f64,
    cp_opp_through: f64,
    g_q: f64,
) -> PermittedGreen {
    let l1 = START_UP_LOST_TIME;
    let e = EXTENSION_OF_EFFECTIVE_GREEN;

    // Displayed unblocked green G_U for a given opposing-queue time
    // (Exhibit 31-12, second column; movement-1 equations mapped to the
    // subject movement).
    let displayed = |gq: f64| -> f64 {
        let gu = match seq {
            // Lead–Lead: GU = min[Dp_L + Dp_OT − Dp_OL − CP_T, Dp_OT − CP_T − Gq]
            LeftTurnSequence::LeadLead => (dp_own_left + dp_opp_through
                - dp_opp_left
                - cp_own_through)
                .min(dp_opp_through - cp_own_through - gq),
            // Lead–Lag / Lead–Perm: GU = Dp_T − CP_T − Dp_L − Gq
            LeftTurnSequence::LeadLag => dp_own_through - cp_own_through - dp_own_left - gq,
            // Lag–Lead / Lag–Perm: GU = Dp_OT − CP_OT − max[Dp_OL, Gq]
            LeftTurnSequence::LagLead => dp_opp_through - cp_opp_through - dp_opp_left.max(gq),
            // Lag–Lag: GU = min[Dp_OT − CP_OT, Dp_T − CP_T] − Gq
            LeftTurnSequence::LagLag => {
                (dp_opp_through - cp_opp_through).min(dp_own_through - cp_own_through) - gq
            }
            // Perm–Lead: GU = Dp_OT − CP_OT − max[Dp_OL, Gq]
            LeftTurnSequence::PermLead => dp_opp_through - cp_opp_through - dp_opp_left.max(gq),
            // Perm–Lag: GU = min[Dp_OT − CP_OT, Dp_T − CP_T] − Gq
            LeftTurnSequence::PermLag => {
                (dp_opp_through - cp_opp_through).min(dp_own_through - cp_own_through) - gq
            }
            // Perm–Perm: GU = Dp_OT − CP_T − Gq
            LeftTurnSequence::PermPerm => dp_opp_through - cp_own_through - gq,
        };
        gu.max(0.0)
    };

    // Permitted start-up lost time l_1,p and extension e_p
    // (Exhibit 31-12, two right-hand columns).
    let (l1p, e_p) = match seq {
        LeftTurnSequence::LeadLead => {
            // Note b: l1* = Dp_OL − (Dp_L − CP_L) + l1 − e when
            // Dp_OL > Dp_L − CP_L, clamped to [0, l1].
            let l1s = if dp_opp_left > dp_own_left - cp_own_left {
                (dp_opp_left - (dp_own_left - cp_own_left) + l1 - e).clamp(0.0, l1)
            } else {
                0.0
            };
            (l1s, e)
        }
        LeftTurnSequence::LeadLag => (0.0, e),
        LeftTurnSequence::LagLead => (l1, 0.0),
        LeftTurnSequence::LagLag => {
            // Note c: e* = Dp_OT − (Dp_T − CP_T), clamped to [0, e].
            let es = (dp_opp_through - (dp_own_through - cp_own_through)).clamp(0.0, e);
            (l1, es)
        }
        LeftTurnSequence::PermLead | LeftTurnSequence::PermLag | LeftTurnSequence::PermPerm => {
            (l1, e)
        }
    };

    let g_u_displayed = displayed(g_q);
    let g_p_displayed = displayed(0.0);
    // Eq. 31-94: g_p = G_p − l_1,p + e_p >= 0
    let g_p = (g_p_displayed - l1p + e_p).max(0.0);
    // Eq. 31-95: g_u = G_U + e_p <= g_p
    let g_u = if g_u_displayed > 0.0 {
        (g_u_displayed + e_p).min(g_p)
    } else {
        0.0
    };
    PermittedGreen {
        g_p,
        g_p_displayed,
        g_u,
        l1p,
        e_p,
    }
}

/// Queue service time `g_s = Q_r / (s/3,600 - q_g)` for one lane
/// (HCM Exhibit 31-11 variable list), capped at the effective green time.
///
/// * `v_ln` — lane demand flow rate, veh/h/ln
/// * `s_ln` — lane saturation flow rate, veh/h/ln
/// * `p` — proportion arriving during green
/// * `g`, `cycle_s` — effective green and cycle length, s
pub fn queue_service_time(v_ln: f64, s_ln: f64, p: f64, g: f64, cycle_s: f64) -> f64 {
    let r = cycle_s - g;
    if r <= 0.0 || v_ln <= 0.0 || g <= 0.0 {
        return 0.0;
    }
    let q = v_ln / 3_600.0;
    let q_r = (1.0 - p) * q * cycle_s / r; // arrival rate during red, veh/s
    let q_g = p * q * cycle_s / g; // arrival rate during green, veh/s
    let q_at_red_end = q_r * r;
    let w = s_ln / 3_600.0 - q_g;
    if w <= 0.0 {
        return g; // saturated: the queue does not clear within the green
    }
    (q_at_red_end / w).min(g)
}

/// HCM Equation 31-122: average saturation flow rate of a shared left-turn
/// and through lane over the permitted period
///
/// `s_sl = s_th / g_p * ( g_f + g_diff/(1 + P_L[EL2/f_Lpb - 1])
///        + min(g_p - g_f, g_u)/(1 + P_L[EL1/f_Lpb - 1]) )`
///
/// with `g_diff = g_p - g_u - g_f >= 0` (HCM Equation 31-107).
#[allow(clippy::too_many_arguments)] // mirrors the HCM equation parameter list
pub fn shared_left_lane_saturation_flow(
    s_th: f64,
    g_p: f64,
    g_f: f64,
    g_u: f64,
    p_l: f64,
    el1: f64,
    el2: f64,
    f_lpb: f64,
) -> f64 {
    if g_p <= 0.0 {
        return s_th;
    }
    let g_diff = (g_p - g_u - g_f).max(0.0); // Eq. 31-107
    let term2 = g_diff / (1.0 + p_l * (el2 / f_lpb - 1.0));
    let term3 = (g_p - g_f).min(g_u) / (1.0 + p_l * (el1 / f_lpb - 1.0));
    s_th / g_p * (g_f + term2 + term3)
}

/// HCM Equation 31-59 with Equation 31-60: shared left-turn lane
/// saturation flow used inside the lane-flow-distribution procedure
/// (modified through-car equivalents EL1,m / EL2,m and the sneaker term
/// `3,600 n_s* f_ms f_sp / s_th`).
#[allow(clippy::too_many_arguments)] // mirrors the HCM equation parameter list
pub fn shared_left_lane_saturation_flow_modified(
    s_th: f64,
    g_p: f64,
    g_f: f64,
    g_u: f64,
    p_l: f64,
    el1_m: f64,
    el2_m: f64,
    n_sneakers: f64,
) -> f64 {
    if g_p <= 0.0 {
        return s_th;
    }
    let g_diff = (g_p - g_u - g_f).max(0.0);
    // Eq. 31-60: expected sneakers per cycle in a shared left-turn lane.
    let ns_star = if p_l < 0.999 {
        p_l / (1.0 - p_l) * (1.0 - p_l.powf(n_sneakers))
    } else {
        n_sneakers * p_l
    };
    let term2 = g_diff / (1.0 + p_l * (el2_m - 1.0));
    let term3 = (g_p - g_f).min(g_u) / (1.0 + p_l * (el1_m - 1.0));
    s_th / g_p * (g_f + term2 + term3 + 3_600.0 * ns_star / s_th)
}

// ═══════════════════════════════════════════════════════════════════════════════
// Pedestrian–bicycle saturation flow adjustment (HCM Ch. 31, Section 2)
// ═══════════════════════════════════════════════════════════════════════════════

/// HCM Equations 31-74 through 31-83: pedestrian–bicycle adjustment factor
/// for permitted right-turn movements f_Rpb (also f_Lpb for left turns
/// from a one-way street, Equation 31-84).
///
/// * `v_ped` — two-way pedestrian flow rate in the conflicting crosswalk, p/h
/// * `v_bic` — conflicting bicycle flow rate, bicycles/h
/// * `g` — effective green time of the phase, s
/// * `g_ped` — pedestrian service time, s (`min(g, Walk + PC)` for actuated
///   phases with pedestrian heads and no rest-in-walk; otherwise `g`)
/// * `receiving_gt_turn_lanes` — true when the number of cross-street
///   receiving lanes exceeds the number of turn lanes (Equation 31-82;
///   Equation 31-81 otherwise)
pub fn ped_bike_factor_right(
    v_ped: f64,
    v_bic: f64,
    g: f64,
    g_ped: f64,
    cycle_s: f64,
    receiving_gt_turn_lanes: bool,
) -> f64 {
    if (v_ped <= 0.0 && v_bic <= 0.0) || g <= 0.0 {
        return 1.0;
    }
    let g_ped = g_ped.min(g).max(0.0);
    // Eq. 31-74: pedestrian flow during service, upper limit 5,000 p/h.
    let v_pedg = (v_ped * cycle_s / g_ped.max(1e-9)).min(5_000.0);
    // Eqs. 31-75 / 31-76: average pedestrian occupancy.
    let occ_pedg = if v_pedg <= 1_000.0 {
        v_pedg / 2_000.0
    } else {
        (0.4 + v_pedg / 10_000.0).min(0.90)
    };
    // Eq. 31-77: bicycle flow during green, upper limit 1,900 bicycles/h.
    let v_bicg = (v_bic * cycle_s / g).min(1_900.0);
    // Eq. 31-78: average bicycle occupancy.
    let occ_bicg = if v_bic > 0.0 { 0.02 + v_bicg / 2_700.0 } else { 0.0 };
    // Eq. 31-79 (no bicycles) / Eq. 31-80 (with bicycles).
    let ped_term = g_ped / g * occ_pedg;
    let occ_r = if v_bic > 0.0 {
        ped_term + occ_bicg - ped_term * occ_bicg
    } else {
        ped_term
    };
    // Eqs. 31-81 / 31-82 with Eq. 31-83: f_Rpb = A_pbT.
    if receiving_gt_turn_lanes {
        1.0 - 0.6 * occ_r
    } else {
        1.0 - occ_r
    }
}

/// HCM Equations 31-85 through 31-88: pedestrian adjustment factor for
/// permitted / protected-permitted left-turn movements on a two-way street
/// f_Lpb.
///
/// * `v_ped` — two-way pedestrian flow rate in the conflicting crosswalk, p/h
/// * `v_o` — opposing demand flow rate, veh/h
/// * `g_p` — effective permitted green time, s
/// * `g_u` — unblocked permitted green, s (the opposing-queue service time
///   is g_q = g_p − g_u)
/// * `g_ped` — pedestrian service time, s
pub fn ped_factor_left_two_way(
    v_ped: f64,
    v_o: f64,
    g_p: f64,
    g_u: f64,
    g_ped: f64,
    cycle_s: f64,
    receiving_gt_turn_lanes: bool,
) -> f64 {
    if v_ped <= 0.0 || g_p <= 0.0 {
        return 1.0;
    }
    let g_ped = g_ped.min(g_p).max(0.0);
    let g_q = (g_p - g_u).max(0.0);
    // Steps A–B of the right-turn procedure with g_p substituted for g.
    let v_pedg = (v_ped * cycle_s / g_ped.max(1e-9)).min(5_000.0);
    let occ_pedg = if v_pedg <= 1_000.0 {
        v_pedg / 2_000.0
    } else {
        (0.4 + v_pedg / 10_000.0).min(0.90)
    };
    // Eqs. 31-85 / 31-86: pedestrian occupancy after the opposing queue
    // clears.
    let occ_pedu = if g_q < g_ped {
        occ_pedg * (1.0 - 0.5 * g_q / g_ped.max(1e-9))
    } else {
        0.0
    };
    // Eq. 31-87: relevant conflict zone occupancy.
    let denom = (g_p - g_q).max(1e-9);
    let occ_r = (g_ped - g_q).max(0.0) / denom * occ_pedu * (-5.00 * v_o / 3_600.0).exp();
    // Eqs. 31-81 / 31-82 with Eq. 31-88.
    if receiving_gt_turn_lanes {
        1.0 - 0.6 * occ_r
    } else {
        1.0 - occ_r
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Pretimed phase duration design (HCM Ch. 31, Section 2) and X_c
// ═══════════════════════════════════════════════════════════════════════════════

/// HCM Equation 19-30 (identical to Equation 31-67): critical intersection
/// volume-to-capacity ratio `X_c = (C / (C - L)) Σ y_c,i`.
///
/// * `cycle_s` — cycle length C, s
/// * `lost_time_s` — cycle lost time L (HCM Equation 19-31), s
/// * `sum_critical_flow_ratios` — Σ y_c,i over the critical phases
pub fn critical_vc_ratio_eq(cycle_s: f64, lost_time_s: f64, sum_critical_flow_ratios: f64) -> f64 {
    cycle_s / (cycle_s - lost_time_s) * sum_critical_flow_ratios
}

/// HCM Equation 31-68: cycle length for a target critical
/// volume-to-capacity ratio, `C = L X_c / (X_c - Σ y_c,i)`.
///
/// Returns `None` (an infinite cycle) when the target ratio does not
/// exceed the sum of critical flow ratios.
pub fn cycle_length_for_target_xc(
    lost_time_s: f64,
    target_xc: f64,
    sum_critical_flow_ratios: f64,
) -> Option<f64> {
    if target_xc <= sum_critical_flow_ratios {
        return None;
    }
    Some(lost_time_s * target_xc / (target_xc - sum_critical_flow_ratios))
}

/// HCM Equation 31-69: effective green allocation for a critical phase,
/// `g_i = y_c,i (C / X_i)`.
pub fn pretimed_effective_green(critical_flow_ratio: f64, cycle_s: f64, target_x: f64) -> f64 {
    critical_flow_ratio * cycle_s / target_x
}

// ═══════════════════════════════════════════════════════════════════════════════
// Incremental queue accumulation (HCM Ch. 19 §4, Eqs. 19-32..19-36)
// ═══════════════════════════════════════════════════════════════════════════════

/// One interval of a queue accumulation polygon: a period of the cycle
/// with constant arrival and discharge rates (HCM Equation 19-34).
#[derive(Debug, Clone, Copy)]
pub struct QapInterval {
    /// Interval duration t_d,i, s.
    pub duration_s: f64,
    /// Discharge (saturation) flow rate during the interval, veh/h/ln
    /// (0.0 while blocked or red).
    pub discharge_veh_h: f64,
    /// Arrival flow rate during the interval, veh/s/ln.
    pub arrival_veh_s: f64,
    /// Vehicles removed from the front of the queue at the end of the
    /// interval (left-turn sneakers, HCM Chapter 31 Section 3).
    pub sneakers_veh: f64,
}

/// Result of a queue accumulation polygon evaluation.
#[derive(Debug, Clone, Copy)]
pub struct QapResult {
    /// Uniform delay d1, s/veh (HCM Equation 19-35).
    pub uniform_delay_s: f64,
    /// Maximum queue size reached during the cycle, veh/ln.
    pub max_queue_veh: f64,
}

/// HCM Equations 19-34 through 19-36 (equivalently 31-92, 31-115, and
/// 31-116): evaluate a queue accumulation polygon.
///
/// The polygon is iterated until the starting queue equals the ending
/// queue (HCM Chapter 31, Section 3, General QAP Construction Procedure,
/// Step 9). Arrival rates must already be capped at capacity (Step 5 of
/// the general procedure).
pub fn qap_evaluate(intervals: &[QapInterval], cycle_s: f64, q_avg_veh_s: f64) -> QapResult {
    let mut q_start = 0.0_f64;
    let mut area = 0.0;
    let mut max_q = 0.0_f64;
    for _pass in 0..60 {
        area = 0.0;
        max_q = 0.0;
        let mut q = q_start;
        for iv in intervals {
            let td = iv.duration_s;
            if td > 0.0 {
                let w = iv.arrival_veh_s - iv.discharge_veh_h / 3_600.0; // veh/s
                if w >= 0.0 {
                    let q_end = q + w * td;
                    area += 0.5 * (q + q_end) * td;
                    q = q_end;
                } else {
                    // Eq. 19-36: the trapezoid lasts min(t_d, Q/w_q).
                    let t_zero = if q > 0.0 { q / (-w) } else { 0.0 };
                    if t_zero >= td {
                        let q_end = (q + w * td).max(0.0);
                        area += 0.5 * (q + q_end) * td;
                        q = q_end;
                    } else {
                        area += 0.5 * q * t_zero;
                        q = 0.0; // arrivals served on arrival thereafter
                    }
                }
                max_q = max_q.max(q);
            }
            if iv.sneakers_veh > 0.0 {
                q = (q - iv.sneakers_veh).max(0.0);
            }
        }
        if (q - q_start).abs() < 1e-9 || q > 1e6 {
            break;
        }
        q_start = q;
    }
    let d1 = if q_avg_veh_s > 0.0 {
        area / (q_avg_veh_s * cycle_s) // Eq. 19-35
    } else {
        0.0
    };
    QapResult {
        uniform_delay_s: d1,
        max_queue_veh: max_q,
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Back of queue (HCM Ch. 31, Section 4)
// ═══════════════════════════════════════════════════════════════════════════════

/// HCM Equations 31-131 and 31-132: acceleration–deceleration delay
///
/// `d_a = [1.47 (S_a - S_s)]^2 / (2 * 1.47 S_a) (1/r_a + 1/r_d)` with
/// `S_a = 0.90 (25.6 + 0.47 S_pl)`
pub fn accel_decel_delay(speed_limit_mph: f64) -> f64 {
    let s_a = 0.90 * (25.6 + 0.47 * speed_limit_mph); // Eq. 31-132
    let s_s = STOP_THRESHOLD_SPEED_MPH;
    (1.47 * (s_a - s_s)).powi(2) / (2.0 * 1.47 * s_a)
        * (1.0 / QUEUE_ACCELERATION_RATE + 1.0 / QUEUE_DECELERATION_RATE)
}

/// HCM Equations 31-133 through 31-141: first-term back of queue Q1 for
/// the basic arrival–departure polygon (through movements, protected
/// turns, and shared through+right lane groups; Exhibit 31-25).
///
/// * `v_ln` — lane demand flow rate, veh/h/ln (capped at lane capacity
///   per the Chapter 31 Section 4 text)
/// * `c_ln` — lane capacity, veh/h/ln
/// * `s_ln` — adjusted saturation flow rate, veh/h/ln
/// * `p` — proportion arriving during green
/// * `g`, `cycle_s` — effective green and cycle, s
/// * `d_a` — acceleration–deceleration delay (Equation 31-131), s
pub fn first_term_back_of_queue(
    v_ln: f64,
    c_ln: f64,
    s_ln: f64,
    p: f64,
    g: f64,
    cycle_s: f64,
    d_a: f64,
) -> f64 {
    if v_ln <= 0.0 || g <= 0.0 || s_ln <= 0.0 {
        return 0.0;
    }
    let x = if c_ln > 0.0 { v_ln / c_ln } else { 1.0 };
    let q = v_ln.min(c_ln) / 3_600.0;
    let s = s_ln / 3_600.0;
    let r = cycle_s - g;
    let x1 = x.min(1.0);
    let q_r = (1.0 - p) * q * cycle_s / r.max(1e-9);
    let q_g = p * q * cycle_s / g;
    if d_a <= (1.0 - p) * g * x {
        // Eq. 31-137 with Eq. 31-139.
        let t_f = q * cycle_s * (1.0 - p - p * d_a / g) / (s * (1.0 - x1 * p));
        (q_r * r + q_g * (t_f - d_a)).max(0.0)
    } else {
        // Eq. 31-138 with Eq. 31-140.
        let t_f = q * cycle_s * (1.0 - p) * (r - d_a) / (s * (r - x1 * (1.0 - p) * g));
        (q_r * (r - d_a + t_f)).max(0.0)
    }
}

/// HCM Exhibits 31-26 through 31-31 with Equations 31-133 through 31-141
/// (Section 4, Steps 2 through 6): first-term back of queue (number of full
/// stops N_f) for a permitted or protected-permitted left-turn lane group,
/// evaluated from its queue accumulation polygon.
///
/// The polygon `intervals` (the same arrival–departure construction used by
/// the Section 3 delay procedure) is iterated to its steady state, then the
/// first-term back of queue is taken as the largest per-busy-period arrival
/// count between queue-dissipation points. This follows the Chapter 31,
/// Section 4, Step 6 rule for the more complex left-turn polygons: "For some
/// of the more complex ADPs that include left-turn movements operating with
/// the permitted mode, the queue may dissipate at two or more points during
/// the cycle. If this occurs, then N_f,i is computed for each of the i
/// periods between queue dissipation points. The first-term back-of-queue
/// estimate is then equal to the largest of the N_f,i values."
///
/// A busy period runs from the moment the queue begins to form until the
/// solid queue clears. The fully-stopped queue (the dashed departure line of
/// Step 3) clears `d_a/2` seconds earlier than the solid queue, so the
/// interval over which arriving vehicles complete a full stop ends `d_a/2`
/// before the solid clearance and the count is reduced by `q d_a/2`
/// (Equations 31-139 and 31-141).
///
/// * `intervals` — left-turn queue accumulation polygon (one cycle); the
///   arrival rate `q` is read from the first interval (constant across the
///   cycle) and is already capped at capacity per the general procedure.
/// * `cycle_s` — cycle length C, s (the polygon spans one cycle)
/// * `d_a` — acceleration–deceleration delay (Equation 31-131), s
pub fn adp_first_term_left(intervals: &[QapInterval], cycle_s: f64, d_a: f64) -> f64 {
    if intervals.is_empty() || cycle_s <= 0.0 {
        return 0.0;
    }
    let q = intervals[0].arrival_veh_s; // veh/s/ln, constant across the cycle
    if q <= 0.0 {
        return 0.0;
    }
    let eps = 1e-4;
    // Steady-state starting queue: the fixed point of the polygon iteration
    // (HCM Chapter 31, Section 3, General QAP Construction, Step 9).
    let mut q_start = 0.0_f64;
    for _ in 0..80 {
        let mut queue = q_start;
        for iv in intervals {
            if iv.duration_s > 0.0 {
                let w = iv.arrival_veh_s - iv.discharge_veh_h / 3_600.0;
                queue = (queue + w * iv.duration_s).max(0.0);
            }
            if iv.sneakers_veh > 0.0 {
                queue = (queue - iv.sneakers_veh).max(0.0);
            }
        }
        if (queue - q_start).abs() < 1e-7 || queue > 1e6 {
            q_start = queue;
            break;
        }
        q_start = queue;
    }
    // Final pass over two concatenated cycles (steady state) so a busy
    // period that spans the cycle boundary is captured in full. The maximum
    // contiguous duration for which the queue exceeds zero bounds the
    // largest per-busy-period arrival count.
    let dt = 0.02_f64;
    let mut queue = q_start;
    let mut busy = 0.0_f64;
    let mut max_busy = 0.0_f64;
    for _cycle in 0..2 {
        for iv in intervals {
            let s = iv.discharge_veh_h / 3_600.0;
            let w = iv.arrival_veh_s - s;
            let steps = (iv.duration_s / dt).ceil().max(0.0) as usize;
            let step = if steps > 0 {
                iv.duration_s / steps as f64
            } else {
                0.0
            };
            for _ in 0..steps {
                queue = (queue + w * step).max(0.0);
                if queue > eps {
                    busy += step;
                    max_busy = max_busy.max(busy);
                } else {
                    busy = 0.0;
                }
            }
            if iv.sneakers_veh > 0.0 {
                queue = (queue - iv.sneakers_veh).max(0.0);
                if queue <= eps {
                    busy = 0.0;
                }
            }
        }
    }
    // Q1 = N_f = arrivals over the longest busy period, less the partial-stop
    // window q d_a/2 (Equations 31-139 and 31-141).
    // VERIFY-HCM: the exact engine accounts for full stops per dissipation
    // interval (Eqs 31-137..31-140); this busy-period form reproduces the
    // published EP1 left-turn queues within tolerance. See docs/hcm/VERIFICATION.md.
    (q * max_busy - q * d_a / 2.0).max(0.0)
}

/// HCM Equation 31-142: second-term back of queue
/// `Q2 = c_A / (3,600 N) d2`.
pub fn second_term_back_of_queue(c_a: f64, n_lanes: u32, d2: f64) -> f64 {
    c_a / (3_600.0 * n_lanes.max(1) as f64) * d2
}

/// HCM Equations 31-143 through 31-148: third-term back of queue Q3 for an
/// initial queue Q_b (0.0 when there is no initial queue).
pub fn third_term_back_of_queue(q_b: f64, v: f64, c_a: f64, n_lanes: u32, t_h: f64) -> f64 {
    if q_b <= 0.0 || t_h <= 0.0 {
        return 0.0;
    }
    let (q_eo, t_a) = if v >= c_a {
        (t_h * (v - c_a), t_h) // Eqs. 31-145 / 31-146
    } else {
        (0.0, (q_b / (c_a - v)).min(t_h)) // Eqs. 31-147 / 31-148
    };
    let q_e = q_b + t_a * (v - c_a); // Eq. 31-144
    1.0 / (n_lanes.max(1) as f64 * t_h) * (t_a * (q_b + q_e - q_eo) / 2.0) // Eq. 31-143
}

/// HCM Equations 31-150 through 31-153: percentile back of queue
///
/// `Q_% = (Q1 + Q2) fB% + Q3` with
/// `fB% = min(1.8, 1.0 + z sqrt(I/(Q1+Q2)) + 0.60 z^0.24 (g/C)^0.33 (1 - e^(2-2X_A)))`
/// when v >= c_A, and `fB% = min(1.8, 1.0 + z sqrt(I/(Q1+Q2)))` otherwise.
///
/// * `z` — percentile parameter (1.04 = 85th, 1.28 = 90th, 1.64 = 95th)
/// * `i_factor` — upstream filtering adjustment factor I
#[allow(clippy::too_many_arguments)] // mirrors the HCM equation parameter list
pub fn percentile_back_of_queue(
    q1: f64,
    q2: f64,
    q3: f64,
    z: f64,
    i_factor: f64,
    g_over_c: f64,
    x_a: f64,
    v_ge_ca: bool,
) -> f64 {
    let q12 = q1 + q2;
    if q12 <= 0.0 {
        return q3;
    }
    let base = 1.0 + z * (i_factor / q12).sqrt();
    let f_b = if v_ge_ca {
        (base + 0.60 * z.powf(0.24) * g_over_c.max(0.0).powf(0.33) * (1.0 - (2.0 - 2.0 * x_a).exp()))
            .min(1.8)
    } else {
        base.min(1.8)
    };
    q12 * f_b + q3 // Eq. 31-150
}

/// HCM Equation 31-155: average vehicle spacing in a stationary queue
/// `L_h = L_pc (1 - 0.01 P_HV) + 0.01 L_HV P_HV`, ft/veh.
pub fn average_vehicle_spacing(pct_heavy_vehicles: f64) -> f64 {
    STORED_PASSENGER_CAR_LENGTH_FT * (1.0 - 0.01 * pct_heavy_vehicles)
        + 0.01 * STORED_HEAVY_VEHICLE_LENGTH_FT * pct_heavy_vehicles
}

/// HCM Equation 31-154 / 31-156: queue storage ratio `R_Q = L_h Q / L_a`.
pub fn queue_storage_ratio_eq(spacing_ft: f64, queue_veh: f64, storage_ft: f64) -> f64 {
    if storage_ft <= 0.0 {
        return 0.0;
    }
    spacing_ft * queue_veh / storage_ft
}

// ═══════════════════════════════════════════════════════════════════════════════
// Internal per-approach working state
// ═══════════════════════════════════════════════════════════════════════════════

/// Working values shared between the analysis passes for one approach.
#[derive(Debug, Clone, Default)]
struct ApproachState {
    /// PHF-adjusted movement flow rates; v_rt is net of RTOR, veh/h.
    v_lt: f64,
    v_th: f64,
    v_rt: f64,
    /// Lane-group flow assignment (group totals), veh/h.
    v_left_group: f64,
    v_thru_group: f64,
    v_sl_group: f64,
    v_sr_group: f64,
    v_right_group: f64,
    /// Turn flows carried in shared lanes, veh/h.
    v_sl_lt: f64,
    v_sr_rt: f64,
    /// Saturation flow of one exclusive through lane, veh/h/ln.
    s_th_excl: f64,
    /// Through-lane saturation flow applicable to the curb (shared) lane,
    /// veh/h/ln (includes the parking factor).
    s_th_curb: f64,
    /// Adjusted saturation flows per lane group, veh/h/ln.
    s_left_prot: f64,
    s_left_perm: f64,
    s_thru: f64,
    s_shared_lt: f64,
    s_shared_rt: f64,
    s_right: f64,
    /// Permitted left-turn parameters (HCM Ch. 31 §3).
    g_p: f64,
    g_p_displayed: f64,
    g_u: f64,
    g_f: f64,
    l1p: f64,
    s_p: f64,
    el1: f64,
    el2: f64,
    f_lpb: f64,
    f_rpb: f64,
    /// Opposing left-turn phase duration D_p, s (0.0 when none).
    dp_opp_left: f64,
    /// Effective green times, s.
    g_through: f64,
    g_left: f64,
}

// ═══════════════════════════════════════════════════════════════════════════════
// Facility implementation
// ═══════════════════════════════════════════════════════════════════════════════

impl SignalizedIntersection {
    /// Create a new signalized intersection. Defaults: T = 0.25 h,
    /// s_o = 1,900 pc/h/ln, non-CBD, pretimed control, 2.0 sneakers/cycle.
    pub fn new(cycle_length_s: f64, approaches: Vec<SignalApproach>) -> Self {
        Self {
            cycle_length_s,
            analysis_period_h: 0.25,
            base_saturation_flow: BASE_SATURATION_FLOW_METRO,
            area_type_cbd: false,
            control: ControlType::PretimedSignal,
            sneakers_per_cycle: SNEAKERS_PER_CYCLE,
            approaches,
            lane_groups: Vec::new(),
            approach_results: Vec::new(),
            intersection_delay_s: None,
            intersection_los: None,
            critical_vc_ratio: None,
        }
    }

    // ── Accessors ──────────────────────────────────────────────────────────

    pub fn get_cycle_length(&self) -> f64 {
        self.cycle_length_s
    }
    pub fn set_cycle_length(&mut self, c: f64) {
        self.cycle_length_s = c;
    }
    pub fn get_analysis_period(&self) -> f64 {
        self.analysis_period_h
    }
    pub fn set_analysis_period(&mut self, t: f64) {
        self.analysis_period_h = t;
    }
    pub fn get_base_saturation_flow(&self) -> f64 {
        self.base_saturation_flow
    }
    pub fn set_base_saturation_flow(&mut self, s: f64) {
        self.base_saturation_flow = s;
    }
    pub fn get_intersection_delay(&self) -> Option<f64> {
        self.intersection_delay_s
    }
    pub fn get_intersection_los(&self) -> Option<LevelOfService> {
        self.intersection_los
    }
    pub fn get_critical_vc_ratio(&self) -> Option<f64> {
        self.critical_vc_ratio
    }
    pub fn get_lane_groups(&self) -> &[LaneGroup] {
        &self.lane_groups
    }
    pub fn get_approach_results(&self) -> &[ApproachResult] {
        &self.approach_results
    }

    /// Index of the opposing approach (NB<->SB, EB<->WB); `None` for a
    /// T-intersection leg with no opposing approach.
    fn opposing_index(&self, i: usize) -> Option<usize> {
        let opp = match self.approaches[i].direction {
            Direction::NB => Direction::SB,
            Direction::SB => Direction::NB,
            Direction::EB => Direction::WB,
            Direction::WB => Direction::EB,
        };
        self.approaches.iter().position(|a| a.direction == opp)
    }

    /// The intersecting approach 90° counterclockwise of `dir` (i.e., the
    /// cross-street approach to the left of the subject driver). Its
    /// left-turn movement is the complementary movement that "shadows" the
    /// subject approach's right turn during its protected phase.
    fn cross_street_left_shadow(dir: Direction) -> Direction {
        // VERIFY-HCM: HCM Ch 31 §8 says "complementary cross street left-turn
        // movement" without a formal movement map; the 90°-CCW approach is
        // the receiving-lane match. See docs/hcm/VERIFICATION.md.
        match dir {
            Direction::EB => Direction::NB,
            Direction::NB => Direction::WB,
            Direction::WB => Direction::SB,
            Direction::SB => Direction::EB,
        }
    }

    /// Estimate the right-turn-on-red flow rate for one approach from the
    /// published Chapter 19 / Chapter 31 rule.
    ///
    /// The HCM motorized vehicle methodology treats RTOR only by removing
    /// RTOR vehicles from the right-turn demand (HCM Chapter 19, Step 2). It
    /// suggests one estimate when the right turn is served by an exclusive
    /// lane (HCM Chapter 31, Section 8, "Effect of Right-Turn-on-Red
    /// Operation"): "If the right-turn movement is served by an exclusive
    /// lane, the methodology suggests RTOR volume can be estimated as equal
    /// to the left-turn demand of the complementary cross street left-turn
    /// movement, whenever this movement is provided a left-turn phase." The
    /// complementary cross-street left turn is the one whose protected phase
    /// stops the through movement that would otherwise conflict with the
    /// subject right turn; it discharges into the same receiving lanes (see
    /// the right-turn-overlap discussion in HCM Chapter 31, Section 2). This
    /// is the approach 90° counterclockwise of the subject.
    ///
    /// Returns 0.0 when the subject approach has no exclusive right-turn lane,
    /// when the complementary approach is absent, or when that approach's
    /// left turn is not provided a protected (or protected-permitted) phase.
    /// The estimate is capped at the right-turn demand. Shared-lane RTOR is
    /// left at 0.0 (the HCM offers no estimate and recommends field data or
    /// an alternative tool). The exact identification of the complementary
    /// movement is a documented VERIFY-HCM item (the published text says
    /// "cross street" without a formal movement map).
    pub fn estimate_rtor_volume(&self, direction: Direction) -> f64 {
        let ap = match self.approaches.iter().find(|a| a.direction == direction) {
            Some(a) => a,
            None => return 0.0,
        };
        if ap.exclusive_right_lanes == 0 {
            return 0.0;
        }
        let shadow_dir = Self::cross_street_left_shadow(direction);
        let shadow = match self.approaches.iter().find(|a| a.direction == shadow_dir) {
            Some(a) => a,
            None => return 0.0,
        };
        let has_left_phase = matches!(
            shadow.left_turn_mode,
            LeftTurnMode::Protected | LeftTurnMode::ProtectedPermitted
        );
        if !has_left_phase {
            return 0.0;
        }
        let (_, _, v_r) = ap.flow_rates();
        let shadow_left = shadow.flow_rates().0;
        // RTOR estimate = complementary cross-street left-turn demand, capped
        // at the subject right-turn demand.
        shadow_left.min(v_r).max(0.0)
    }

    /// Populate each approach's `volume_rtor` with the Chapter 31 exclusive
    /// right-turn-lane estimate ([`estimate_rtor_volume`]) where no RTOR flow
    /// has been supplied. Call before [`analyze`]; approaches that already
    /// carry a nonzero `volume_rtor` (e.g., a field measurement) are left
    /// unchanged.
    pub fn apply_rtor_estimates(&mut self) {
        let estimates: Vec<(Direction, f64)> = self
            .approaches
            .iter()
            .filter(|a| a.volume_rtor <= 0.0)
            .map(|a| (a.direction, self.estimate_rtor_volume(a.direction)))
            .collect();
        for (dir, est) in estimates {
            if let Some(a) = self.approaches.iter_mut().find(|a| a.direction == dir) {
                a.volume_rtor = est;
            }
        }
    }

    /// Estimate the average actuated phase durations from the controller
    /// settings (HCM Chapter 31, Section 2, Actuated Phase Duration,
    /// Equations 31-1 through 31-45), using the analyzed lane-group demand
    /// and saturation flows as the operating point.
    ///
    /// Requires [`analyze`] to have been run first (the estimate consumes the
    /// Step 3/4 lane-group flow rates and adjusted saturation flows and the
    /// Step 6 permitted unblocked green g_u). The demand and permitted green
    /// are held fixed at that operating point; recomputing them inside the
    /// actuated iteration is the deferred computational-engine coupling
    /// documented in [`crate::hcm::signalized::actuated`].
    ///
    /// * `simultaneous_gap_out` — whether the through phases that terminate
    ///   at each barrier are set for simultaneous gap-out (Equation 31-26)
    ///
    /// Returns the per-phase results (duration, queue service, green
    /// extension, MAH, and max-out / call probabilities). Does not mutate
    /// `self`; the fixed-timing analysis pipeline is unchanged (the supplied
    /// phase durations remain the default path).
    pub fn estimate_actuated_timings(
        &self,
        simultaneous_gap_out: bool,
    ) -> Vec<ActuatedPhaseResult> {
        // Group analyzed lane groups by controlling phase number, attaching
        // the owning approach for detection, pedestrian, and speed inputs.
        let mut phase_nos: Vec<u8> = self.lane_groups.iter().map(|lg| lg.phase_no).collect();
        phase_nos.sort_unstable();
        phase_nos.dedup();

        let mut phases: Vec<ActuatedPhaseInput> = Vec::new();
        for &no in &phase_nos {
            // The PhaseTiming and approach that own this phase number.
            let owner = self.approaches.iter().find_map(|ap| {
                if ap.through_phase.phase_no == no {
                    Some((&ap.through_phase, ap, false))
                } else if ap.left_phase.as_ref().map(|p| p.phase_no) == Some(no) {
                    Some((ap.left_phase.as_ref().unwrap(), ap, true))
                } else {
                    None
                }
            });
            let (pt, ap, is_left_phase) = match owner {
                Some(v) => v,
                None => continue,
            };
            let lane_groups: Vec<ActuatedLaneGroupInput> = self
                .lane_groups
                .iter()
                .filter(|lg| lg.phase_no == no)
                .map(|lg| {
                    let owner_ap = self
                        .approaches
                        .iter()
                        .find(|a| a.direction == lg.direction)
                        .unwrap_or(ap);
                    let (mah_kind, sl_perm, g_u) = match lg.kind {
                        LaneGroupKind::ExclusiveLeft => match owner_ap.left_turn_mode {
                            LeftTurnMode::Permitted => (
                                MahLaneGroup::LeftPermittedExclusive,
                                lg.sat_flow_permitted.unwrap_or(0.0),
                                lg.g_u.unwrap_or(0.0),
                            ),
                            _ => (MahLaneGroup::LeftProtectedExclusive, 0.0, 0.0),
                        },
                        LaneGroupKind::SharedLeftThrough => (
                            MahLaneGroup::LeftPermittedShared,
                            lg.sat_flow_permitted.unwrap_or(0.0),
                            lg.g_u.unwrap_or(0.0),
                        ),
                        LaneGroupKind::ExclusiveThrough => (MahLaneGroup::Through, 0.0, 0.0),
                        LaneGroupKind::SharedRightThrough => {
                            (MahLaneGroup::RightPermittedShared, 0.0, 0.0)
                        }
                        LaneGroupKind::ExclusiveRight => {
                            (MahLaneGroup::RightProtectedExclusive, 0.0, 0.0)
                        }
                    };
                    let pct_hv = match lg.kind {
                        LaneGroupKind::ExclusiveLeft | LaneGroupKind::SharedLeftThrough => {
                            owner_ap.pct_heavy_vehicles_left
                        }
                        _ => owner_ap.pct_heavy_vehicles_through,
                    };
                    ActuatedLaneGroupInput {
                        v: lg.flow_rate,
                        s: lg.sat_flow.unwrap_or(0.0),
                        lanes: lg.lanes,
                        pct_heavy_vehicles: pct_hv,
                        detector_len_ft: pt.detector_length_ft.unwrap_or(40.0),
                        mah_kind,
                        sl_permitted: sl_perm,
                        g_u,
                        f_rpb: 1.0,
                    }
                })
                .collect();
            if lane_groups.is_empty() {
                continue;
            }
            let walk_plus_pc = match (pt.walk_s, pt.ped_clear_s) {
                (Some(w), Some(pc)) => Some(w + pc),
                _ => None,
            };
            phases.push(ActuatedPhaseInput {
                phase_no: no,
                max_green_s: pt
                    .max_green_s
                    .unwrap_or_else(|| pt.duration_s - pt.change_period_s()),
                min_green_s: pt.min_green_s.unwrap_or(5.0),
                passage_time_s: pt.passage_time_s.unwrap_or(2.0),
                yellow_s: pt.yellow_s,
                red_clearance_s: pt.red_clearance_s,
                walk_plus_pc_s: walk_plus_pc,
                ped_flow_ph: ap.ped_flow_ph,
                recall_max: pt.recall_max,
                protected_left: is_left_phase,
                speed_limit_mph: ap.speed_limit_mph,
                lane_groups,
            });
        }
        let (results, _cycle) =
            estimate_fully_actuated(&phases, self.base_saturation_flow, simultaneous_gap_out, 1.0);
        results
    }

    // ═══════════════════════════════════════════════════════════════════════
    // Full pipeline
    // ═══════════════════════════════════════════════════════════════════════

    /// Run the full motorized vehicle methodology (Steps 1–10 of HCM
    /// Exhibit 19-18) and store the results in `self`.
    pub fn analyze(&mut self) {
        let states = self.compute_states(); // Steps 1–4 (iterated)
        self.build_lane_groups(&states);
        self.step_5_proportion_arriving_on_green();
        // Step 6 (signal phase duration) is an input for pretimed and
        // coordinated control (HCM Ch. 19 Step 6); see
        // `cycle_length_for_target_xc` / `pretimed_effective_green` for
        // the Chapter 31 pretimed design procedure.
        self.step_7_capacity_and_vc(&states);
        self.step_8_delay(&states);
        self.step_9_los();
        self.step_10_queue_storage();
        self.critical_vc_ratio = Some(self.compute_critical_vc(&states));
    }

    // ═══════════════════════════════════════════════════════════════════════
    // Steps 1–4: movement groups, lane groups, flow distribution, and
    // adjusted saturation flow. These are iterated jointly because the
    // Chapter 31 Section 2 lane-flow procedure needs the Step 4 saturation
    // flows, which depend on the opposing lane flows through g_u.
    // ═══════════════════════════════════════════════════════════════════════

    fn compute_states(&self) -> Vec<ApproachState> {
        let n = self.approaches.len();
        let mut st: Vec<ApproachState> = vec![ApproachState::default(); n];

        // Step 2: movement-group flow rates (PHF applied; RTOR subtracted
        // from the right-turn demand per HCM Ch. 19 Step 2).
        for (i, ap) in self.approaches.iter().enumerate() {
            let (v_l, v_t, v_r) = ap.flow_rates();
            st[i].v_lt = v_l;
            st[i].v_th = v_t;
            st[i].v_rt = (v_r - ap.volume_rtor).max(0.0);
            st[i].g_through = ap.through_phase.effective_green_s();
            st[i].g_left = ap
                .left_phase
                .as_ref()
                .map(|p| p.effective_green_s())
                .unwrap_or(0.0);
            st[i].f_lpb = 1.0;
            st[i].f_rpb = 1.0;
            st[i].el1 = E_L_PROTECTED_LEFT;
            st[i].el2 = E_L_PROTECTED_LEFT;
            // Initial (even) lane-flow assignment; refined below.
            self.assign_flows_simple(i, &mut st);
        }

        // Iterate Steps 3–4 to convergence (flows, unblocked permitted
        // green, and shared-lane saturation flows must all settle).
        for _iter in 0..25 {
            let prev: Vec<[f64; 6]> = st
                .iter()
                .map(|s| {
                    [
                        s.v_thru_group,
                        s.v_sl_group,
                        s.v_sr_group,
                        s.g_u,
                        s.s_shared_rt,
                        s.s_shared_lt,
                    ]
                })
                .collect();

            // Step 4 static factors (Equation 19-8 components).
            for (i, ap) in self.approaches.iter().enumerate() {
                let f_w = lane_width_factor(ap.lane_width_ft);
                let f_hvg =
                    heavy_vehicle_grade_factor(ap.pct_heavy_vehicles_through, ap.grade_pct);
                let f_a = area_type_factor(self.area_type_cbd);
                let f_bb = bus_blockage_factor(1, ap.bus_stops_h);
                let f_p = if ap.parking_present {
                    parking_factor(1, ap.parking_maneuvers_h)
                } else {
                    1.0
                };
                let f_lu = default_lane_utilization_factor(
                    LaneUtilizationGroup::ExclusiveThrough,
                    ap.through_lanes,
                );
                // Exclusive through lane (Eq. 19-8 with unit turn factors).
                st[i].s_th_excl = self.base_saturation_flow * f_w * f_hvg * f_a * f_bb * f_lu;
                // Curb (shared) lane: the parking factor applies to the
                // lane group adjacent to the parking lane (HCM Ch. 19,
                // Adjustment for Parking).
                st[i].s_th_curb = self.base_saturation_flow * f_w * f_hvg * f_a * f_bb * f_p;
            }

            // Pedestrian–bicycle factors and permitted-left parameters.
            for i in 0..n {
                self.update_permitted_state(i, &mut st);
            }

            // Lane-group saturation flows (Step 4 / Ch. 31 §3 Step 6).
            for i in 0..n {
                self.update_lane_group_sat_flows(i, &mut st);
            }

            // Step 3: lane group flow rates (Ch. 31 §2).
            for i in 0..n {
                self.distribute_lane_flows(i, &mut st);
            }

            let max_delta = st
                .iter()
                .zip(prev.iter())
                .map(|(s, p)| {
                    let cur = [
                        s.v_thru_group,
                        s.v_sl_group,
                        s.v_sr_group,
                        s.g_u,
                        s.s_shared_rt,
                        s.s_shared_lt,
                    ];
                    cur.iter()
                        .zip(p.iter())
                        .map(|(a, b)| (a - b).abs())
                        .fold(0.0_f64, f64::max)
                })
                .fold(0.0_f64, f64::max);
            if max_delta < 0.05 {
                break;
            }
        }
        st
    }

    /// One-to-one movement-group to lane-group assignment (HCM Ch. 19
    /// Step 3, no-shared-lane or single-lane case), also used as the
    /// starting point for the iterative procedure.
    fn assign_flows_simple(&self, i: usize, st: &mut [ApproachState]) {
        let ap = &self.approaches[i];
        let (v_lt, v_th, v_rt) = (st[i].v_lt, st[i].v_th, st[i].v_rt);
        st[i].v_left_group = if ap.exclusive_left_lanes > 0 { v_lt } else { 0.0 };
        st[i].v_right_group = if ap.exclusive_right_lanes > 0 { v_rt } else { 0.0 };
        st[i].v_sl_lt = if ap.shared_left_through_lane && ap.exclusive_left_lanes == 0 {
            v_lt
        } else {
            0.0
        };
        st[i].v_sr_rt = if ap.shared_right_through_lane && ap.exclusive_right_lanes == 0 {
            v_rt
        } else {
            0.0
        };
        // Distribute the through flow evenly across through-serving lanes.
        let n_th = ap.through_serving_lanes().max(1) as f64;
        let v_th_per_lane = v_th / n_th;
        st[i].v_sl_group = if ap.shared_left_through_lane {
            st[i].v_sl_lt + v_th_per_lane
        } else {
            0.0
        };
        st[i].v_sr_group = if ap.shared_right_through_lane {
            st[i].v_sr_rt + v_th_per_lane
        } else {
            0.0
        };
        st[i].v_thru_group = (v_th
            - (st[i].v_sl_group - st[i].v_sl_lt)
            - (st[i].v_sr_group - st[i].v_sr_rt))
            .max(0.0);
    }

    /// Permitted-left parameters for approach `i`: g_p / g_u
    /// (Exhibit 31-12), s_p (Eq. 31-100), EL1/EL2 (Eqs. 31-101..31-103),
    /// g_f (Eqs. 31-96..31-99), and the pedestrian–bicycle factors
    /// (Ch. 31 §2).
    fn update_permitted_state(&self, i: usize, st: &mut [ApproachState]) {
        let ap = &self.approaches[i];
        let c = self.cycle_length_s;
        let g = st[i].g_through;
        let g_ped = match (ap.through_phase.walk_s, ap.through_phase.ped_clear_s) {
            (Some(w), Some(pc)) => (w + pc).min(g),
            _ => g,
        };
        let right_turn_lanes =
            (ap.exclusive_right_lanes + u32::from(ap.shared_right_through_lane)).max(1);
        st[i].f_rpb = ped_bike_factor_right(
            ap.ped_flow_ph,
            ap.bike_flow_ph,
            g,
            g_ped,
            c,
            ap.receiving_lanes > right_turn_lanes,
        );

        let permitted = matches!(
            ap.left_turn_mode,
            LeftTurnMode::Permitted | LeftTurnMode::ProtectedPermitted
        );
        if !permitted {
            st[i].f_lpb = 1.0; // protected mode or split phasing
            st[i].g_p = 0.0;
            st[i].g_u = 0.0;
            st[i].g_f = 0.0;
            st[i].s_p = 0.0;
            st[i].el1 = E_L_PROTECTED_LEFT;
            st[i].el2 = E_L_PROTECTED_LEFT;
            return;
        }

        let opp = self.opposing_index(i);
        let (g_q_worst, v_o, p_lto, opp_single_lane) = match opp {
            Some(j) => self.opposing_queue_data(i, j, st),
            None => (0.0, 0.1, 0.0, false),
        };
        let (dp_ol, cp_ot, dp_ot) = match opp {
            Some(j) => {
                let oa = &self.approaches[j];
                (
                    oa.left_phase.as_ref().map(|p| p.duration_s).unwrap_or(0.0),
                    oa.through_phase.change_period_s(),
                    oa.through_phase.duration_s,
                )
            }
            None => (
                0.0,
                ap.through_phase.change_period_s(),
                ap.through_phase.duration_s,
            ),
        };
        let pg = permitted_green_times(
            ap.left_turn_sequence,
            ap.left_phase.as_ref().map(|p| p.duration_s).unwrap_or(0.0),
            dp_ol,
            ap.through_phase.duration_s,
            dp_ot,
            ap.left_phase
                .as_ref()
                .map(|p| p.change_period_s())
                .unwrap_or(0.0),
            ap.through_phase.change_period_s(),
            cp_ot,
            g_q_worst,
        );
        st[i].g_p = pg.g_p;
        st[i].g_p_displayed = pg.g_p_displayed;
        st[i].g_u = pg.g_u;
        st[i].l1p = pg.l1p;
        st[i].dp_opp_left = dp_ol;
        st[i].s_p = permitted_left_saturation_flow(v_o);
        st[i].el1 = el1_permitted_left(self.base_saturation_flow, st[i].s_p);
        if ap.shared_left_through_lane {
            let single = ap.through_serving_lanes() == 1;
            let p_l = if st[i].v_sl_group > 0.0 {
                (st[i].v_sl_lt / st[i].v_sl_group).min(1.0)
            } else {
                0.0
            };
            st[i].g_f = time_before_first_left_blocks(
                st[i].g_p_displayed,
                st[i].g_p,
                st[i].l1p,
                p_l,
                st[i].v_lt,
                c,
                single,
            );
        } else {
            st[i].g_f = 0.0;
        }
        st[i].el2 = if opp_single_lane {
            el2_permitted_left(p_lto, st[i].g_p, st[i].g_u, st[i].g_f)
        } else {
            E_L_PROTECTED_LEFT
        };
        let left_turn_lanes =
            (ap.exclusive_left_lanes + u32::from(ap.shared_left_through_lane)).max(1);
        st[i].f_lpb = ped_factor_left_two_way(
            ap.ped_flow_ph,
            v_o,
            st[i].g_p,
            st[i].g_u,
            g_ped.min(st[i].g_p),
            c,
            ap.receiving_lanes > left_turn_lanes,
        );
    }

    /// Opposing-approach data for the permitted-left computations:
    /// * Gq — worst opposing-lane queue clearance (g_s + l_1;
    ///   Exhibit 31-12 note a, excluding any opposing shared left-turn
    ///   lane),
    /// * v_o — opposing demand flow rate (Ch. 31 §3 Step 3, Case 1/2),
    /// * P_lto — proportion of lefts in the opposing stream,
    /// * whether the opposing approach is effectively single-lane
    ///   (Ch. 31 §3 discussion following Equation 31-103).
    fn opposing_queue_data(
        &self,
        subject: usize,
        j: usize,
        st: &[ApproachState],
    ) -> (f64, f64, f64, bool) {
        let oa = &self.approaches[j];
        let so = &st[j];
        let c = self.cycle_length_s;
        let g = so.g_through;
        let p = (oa.platoon_ratio_through * g / c).min(1.0);
        let mut g_s: f64 = 0.0;
        if oa.through_lanes > 0 && so.v_thru_group > 0.0 {
            let v_ln = so.v_thru_group / oa.through_lanes as f64;
            g_s = g_s.max(queue_service_time(v_ln, so.s_thru, p, g, c));
        }
        if oa.shared_right_through_lane && so.v_sr_group > 0.0 {
            g_s = g_s.max(queue_service_time(so.v_sr_group, so.s_shared_rt, p, g, c));
        }
        let g_q = if g_s > 0.0 { g_s + START_UP_LOST_TIME } else { 0.0 };
        // Case 2 applies when the opposing right turn does not influence
        // gap acceptance (analyst option on an exclusive lane) or there is
        // no opposing right-turn movement; Case 1 otherwise.
        let case2 = (oa.exclusive_right_lanes > 0
            && !self.approaches[subject].opposing_right_turn_influences_gaps)
            || so.v_rt <= 0.0;
        let v_o = if case2 { so.v_th } else { so.v_th + so.v_rt };
        let total = so.v_lt + so.v_th + so.v_rt;
        let p_lto = if total > 0.0 { so.v_lt / total } else { 0.0 };
        let single = oa.through_serving_lanes() == 1
            && oa.shared_left_through_lane
            && oa.exclusive_right_lanes == 0;
        (g_q, v_o, p_lto, single)
    }

    /// Lane-group adjusted saturation flows for approach `i`
    /// (HCM Equation 19-8 and Chapter 31 Equations 31-104..31-114).
    fn update_lane_group_sat_flows(&self, i: usize, st: &mut [ApproachState]) {
        let ap = &self.approaches[i];
        let f_w = lane_width_factor(ap.lane_width_ft);
        let f_a = area_type_factor(self.area_type_cbd);
        let f_hvg_l = heavy_vehicle_grade_factor(ap.pct_heavy_vehicles_left, ap.grade_pct);
        let f_lu_l = default_lane_utilization_factor(
            LaneUtilizationGroup::ExclusiveLeft,
            ap.exclusive_left_lanes,
        );
        // Eq. 31-112: exclusive left, protected period.
        st[i].s_left_prot = self.base_saturation_flow
            * f_w
            * f_hvg_l
            * f_a
            * f_lu_l
            * protected_left_turn_factor();
        // Eq. 31-110: exclusive left, permitted period (s_p replaces s_o;
        // no f_LT; f_Lpb applies).
        st[i].s_left_perm = st[i].s_p * f_w * f_hvg_l * f_a * f_lu_l * st[i].f_lpb;
        // Exclusive through lanes; Eq. 31-62: 0.91 on through lanes when a
        // shared left-turn lane with permitted operation is present.
        st[i].s_thru = st[i].s_th_excl;
        if ap.shared_left_through_lane
            && matches!(
                ap.left_turn_mode,
                LeftTurnMode::Permitted | LeftTurnMode::ProtectedPermitted
            )
        {
            st[i].s_thru *= 0.91;
        }
        // Shared right+through lane (Eq. 31-105).
        if ap.shared_right_through_lane {
            let p_r = if st[i].v_sr_group > 0.0 {
                (st[i].v_sr_rt / st[i].v_sr_group).min(1.0)
            } else {
                0.0
            };
            st[i].s_shared_rt =
                st[i].s_th_curb / (1.0 + p_r * (E_R_PROTECTED_RIGHT / st[i].f_rpb - 1.0));
        } else {
            st[i].s_shared_rt = 0.0;
        }
        // Shared left+through lane (Eq. 31-122).
        if ap.shared_left_through_lane {
            let p_l = if st[i].v_sl_group > 0.0 {
                (st[i].v_sl_lt / st[i].v_sl_group).min(1.0)
            } else {
                0.0
            };
            st[i].s_shared_lt = shared_left_lane_saturation_flow(
                st[i].s_th_curb,
                st[i].g_p,
                st[i].g_f,
                st[i].g_u,
                p_l,
                st[i].el1,
                st[i].el2,
                st[i].f_lpb,
            );
        } else {
            st[i].s_shared_lt = 0.0;
        }
        // Exclusive right lane with permitted operation (Eq. 31-104).
        let f_lu_r = default_lane_utilization_factor(
            LaneUtilizationGroup::ExclusiveRight,
            ap.exclusive_right_lanes,
        );
        st[i].s_right = self.base_saturation_flow
            * f_w
            * heavy_vehicle_grade_factor(ap.pct_heavy_vehicles_through, ap.grade_pct)
            * f_a
            * f_lu_r
            * protected_right_turn_factor()
            * st[i].f_rpb;
    }

    /// Step 3 (HCM Ch. 31 §2, Lane Group Flow Rate on Multiple-Lane
    /// Approaches, Equations 31-46 through 31-66) for approach `i`.
    ///
    /// Exclusive turn lane groups exchange volume with the through lanes
    /// only when a shared lane serves the same turn movement; otherwise
    /// their flow is fixed and the equalization runs over the remaining
    /// lane groups.
    fn distribute_lane_flows(&self, i: usize, st: &mut [ApproachState]) {
        let ap = &self.approaches[i];
        let (v_lt, v_th, v_rt) = (st[i].v_lt, st[i].v_th, st[i].v_rt);
        let n_th = ap.through_serving_lanes();
        let has_shared = ap.shared_left_through_lane || ap.shared_right_through_lane;

        if !has_shared || n_th <= 1 {
            self.assign_flows_simple(i, st);
            return;
        }

        // Eq. 31-51: average flow per through-serving lane (upstream of
        // any turn bays, so turning traffic is included).
        let v_app = (v_lt + v_th + v_rt) / n_th as f64;
        // Eq. 31-50: probability of a lane change, with
        // s_lc = 3,600 / t_lc (t_lc = 3.7 s).
        let s_lc = 3_600.0 / CRITICAL_MERGE_HEADWAY;
        let p_lc = (1.0 - (2.0 * v_app / s_lc - 1.0).powi(2)).max(0.0);
        // Eqs. 31-48, 31-49, and 31-53: modified through-car equivalents.
        let el1_m = (st[i].el1 / st[i].f_lpb - 1.0) * p_lc + 1.0;
        let el2_m = (st[i].el2 / st[i].f_lpb - 1.0) * p_lc + 1.0;
        let er_m = (E_R_PROTECTED_RIGHT / st[i].f_rpb - 1.0) * p_lc + 1.0;

        let n_t = ap.through_lanes;
        let n_sl = u32::from(ap.shared_left_through_lane);
        let n_sr = u32::from(ap.shared_right_through_lane);
        // Turn volumes fixed in shared lanes when no exclusive lane exists.
        let left_exchanges = n_sl > 0 && ap.exclusive_left_lanes > 0;
        let right_exchanges = n_sr > 0 && ap.exclusive_right_lanes > 0;

        // Step B: initial estimates.
        let mut v_sl = if n_sl > 0 { v_app } else { 0.0 };
        let mut v_sr = if n_sr > 0 { v_app } else { 0.0 };
        let mut v_sl_lt = if n_sl > 0 && !left_exchanges { v_lt } else { 0.0 };
        let mut v_sr_rt = if n_sr > 0 && !right_exchanges { v_rt } else { 0.0 };

        for _ in 0..60 {
            // Step D: proportions of turns in shared lanes (Eq. 31-57).
            let p_l = if v_sl > 0.0 { (v_sl_lt / v_sl).min(1.0) } else { 0.0 };
            let p_r = if v_sr > 0.0 { (v_sr_rt / v_sr).min(1.0) } else { 0.0 };
            // Step E: lane-group saturation flows with modified
            // equivalents (Eqs. 31-59, 31-61, 31-62).
            let s_sl = if n_sl > 0 {
                shared_left_lane_saturation_flow_modified(
                    st[i].s_th_curb,
                    st[i].g_p,
                    st[i].g_f,
                    st[i].g_u,
                    p_l,
                    el1_m,
                    el2_m,
                    self.sneakers_per_cycle,
                )
            } else {
                0.0
            };
            let s_sr = if n_sr > 0 {
                st[i].s_th_curb / (1.0 + p_r * (er_m - 1.0))
            } else {
                0.0
            };
            let s_t = st[i].s_thru;

            // Step C: per-lane flows of exchanging exclusive groups.
            let v_l_ln = if left_exchanges {
                ((v_lt - v_sl_lt) / ap.exclusive_left_lanes as f64).max(0.0)
            } else {
                0.0
            };
            let v_r_ln = if right_exchanges {
                ((v_rt - v_sr_rt) / ap.exclusive_right_lanes as f64).max(0.0)
            } else {
                0.0
            };
            let v_t_ln = if n_t > 0 {
                ((v_th - (v_sl - v_sl_lt) - (v_sr - v_sr_rt)) / n_t as f64).max(0.0)
            } else {
                0.0
            };

            // Exclusive-left saturation for the distribution (Eq. 31-59
            // with P_L = 1, g_f = g_diff = 0, s_th -> s_lt).
            let s_l_dist = if left_exchanges {
                match ap.left_turn_mode {
                    LeftTurnMode::Permitted | LeftTurnMode::ProtectedPermitted
                        if st[i].g_p > 0.0 =>
                    {
                        let slt = st[i].s_left_prot;
                        slt / st[i].g_p
                            * (st[i].g_u / el1_m
                                + 3_600.0 * self.sneakers_per_cycle / slt
                                + if ap.left_turn_mode == LeftTurnMode::ProtectedPermitted {
                                    st[i].g_left
                                } else {
                                    0.0
                                })
                    }
                    _ => st[i].s_left_prot,
                }
            } else {
                0.0
            };
            let s_r_dist = if right_exchanges { st[i].s_right } else { 0.0 };

            // Step F: approach flow ratio y* (Eq. 31-64) over the
            // exchanging lane groups.
            let mut num = v_sl + v_sr + v_t_ln * n_t as f64;
            let mut den = s_sl * n_sl as f64 + s_sr * n_sr as f64 + s_t * n_t as f64;
            if left_exchanges {
                num += v_l_ln * ap.exclusive_left_lanes as f64;
                den += s_l_dist * ap.exclusive_left_lanes as f64;
            }
            if right_exchanges {
                num += v_r_ln * ap.exclusive_right_lanes as f64;
                den += s_r_dist * ap.exclusive_right_lanes as f64;
            }
            if den <= 0.0 {
                break;
            }
            let y_star = num / den;

            // Step G: revised lane-group flow rates (Eq. 31-65).
            let v_sl_new = if n_sl > 0 { s_sl * y_star } else { 0.0 };
            let v_sr_new = if n_sr > 0 { s_sr * y_star } else { 0.0 };
            // Step H: turn flows in shared lanes (Eq. 31-66).
            let v_sl_lt_new = if left_exchanges {
                (v_lt - (s_l_dist * y_star * ap.exclusive_left_lanes as f64)).max(0.0)
            } else {
                v_sl_lt
            };
            let v_sr_rt_new = if right_exchanges {
                (v_rt - (s_r_dist * y_star * ap.exclusive_right_lanes as f64)).max(0.0)
            } else {
                v_sr_rt
            };

            let delta = (v_sl_new - v_sl).abs().max((v_sr_new - v_sr).abs());
            v_sl = v_sl_new;
            v_sr = v_sr_new;
            v_sl_lt = v_sl_lt_new.min(v_sl);
            v_sr_rt = v_sr_rt_new.min(v_sr);
            if delta < 0.1 {
                break;
            }
        }

        // Conservation: exclusive groups carry the remainder.
        let v_left_group = if ap.exclusive_left_lanes > 0 {
            (v_lt - v_sl_lt).max(0.0)
        } else {
            0.0
        };
        let v_right_group = if ap.exclusive_right_lanes > 0 {
            (v_rt - v_sr_rt).max(0.0)
        } else {
            0.0
        };
        let v_thru_group =
            (v_lt + v_th + v_rt - v_left_group - v_right_group - v_sl - v_sr).max(0.0);
        st[i].v_left_group = v_left_group;
        st[i].v_right_group = v_right_group;
        st[i].v_sl_group = v_sl;
        st[i].v_sr_group = v_sr;
        st[i].v_thru_group = v_thru_group;
        st[i].v_sl_lt = v_sl_lt;
        st[i].v_sr_rt = v_sr_rt;
    }

    /// Materialize the lane-group list (Step 1 rules of HCM Exhibit 19-19)
    /// with the Step 2–4 results.
    fn build_lane_groups(&mut self, st: &[ApproachState]) {
        let mut groups = Vec::new();
        for (i, ap) in self.approaches.iter().enumerate() {
            let s = &st[i];
            // Exclusive left-turn lane group.
            if ap.exclusive_left_lanes > 0 && ap.left_turn_mode != LeftTurnMode::NotPresent {
                let left_phase_no = ap
                    .left_phase
                    .as_ref()
                    .map(|p| p.phase_no)
                    .unwrap_or(ap.through_phase.phase_no);
                let (sat, sat_perm, g, phase_no) = match ap.left_turn_mode {
                    LeftTurnMode::Protected => {
                        (s.s_left_prot, None, s.g_left, left_phase_no)
                    }
                    LeftTurnMode::Permitted => (
                        s.s_left_perm,
                        Some(s.s_left_perm),
                        s.g_p,
                        ap.through_phase.phase_no,
                    ),
                    LeftTurnMode::ProtectedPermitted => {
                        (s.s_left_prot, Some(s.s_left_perm), s.g_left, left_phase_no)
                    }
                    LeftTurnMode::NotPresent => unreachable!(),
                };
                let mut lg = LaneGroup::new(
                    ap.direction,
                    LaneGroupKind::ExclusiveLeft,
                    phase_no,
                    ap.exclusive_left_lanes,
                    s.v_left_group,
                );
                lg.sat_flow = Some(sat);
                lg.sat_flow_permitted = sat_perm;
                lg.effective_green_s = Some(g);
                lg.g_p = Some(s.g_p);
                lg.g_u = Some(s.g_u);
                groups.push(lg);
            }
            // Shared left+through lane group.
            if ap.shared_left_through_lane {
                let p_l = if s.v_sl_group > 0.0 {
                    (s.v_sl_lt / s.v_sl_group).min(1.0)
                } else {
                    0.0
                };
                let mut lg = LaneGroup::new(
                    ap.direction,
                    LaneGroupKind::SharedLeftThrough,
                    ap.through_phase.phase_no,
                    1,
                    s.v_sl_group,
                );
                lg.sat_flow = Some(s.s_shared_lt);
                lg.sat_flow_permitted = Some(s.s_shared_lt);
                lg.effective_green_s = Some(if s.g_p > 0.0 { s.g_p } else { s.g_through });
                lg.g_p = Some(s.g_p);
                lg.g_u = Some(s.g_u);
                lg.g_f = Some(s.g_f);
                lg.p_left_shared = Some(p_l);
                groups.push(lg);
            }
            // Exclusive through lane group.
            if ap.through_lanes > 0 {
                let mut lg = LaneGroup::new(
                    ap.direction,
                    LaneGroupKind::ExclusiveThrough,
                    ap.through_phase.phase_no,
                    ap.through_lanes,
                    s.v_thru_group,
                );
                lg.sat_flow = Some(s.s_thru);
                lg.effective_green_s = Some(s.g_through);
                groups.push(lg);
            }
            // Shared right+through lane group.
            if ap.shared_right_through_lane {
                let p_r = if s.v_sr_group > 0.0 {
                    (s.v_sr_rt / s.v_sr_group).min(1.0)
                } else {
                    0.0
                };
                let mut lg = LaneGroup::new(
                    ap.direction,
                    LaneGroupKind::SharedRightThrough,
                    ap.through_phase.phase_no,
                    1,
                    s.v_sr_group,
                );
                lg.sat_flow = Some(s.s_shared_rt);
                lg.effective_green_s = Some(s.g_through);
                lg.p_right_shared = Some(p_r);
                groups.push(lg);
            }
            // Exclusive right-turn lane group.
            if ap.exclusive_right_lanes > 0 {
                let mut lg = LaneGroup::new(
                    ap.direction,
                    LaneGroupKind::ExclusiveRight,
                    ap.through_phase.phase_no,
                    ap.exclusive_right_lanes,
                    s.v_right_group,
                );
                lg.sat_flow = Some(s.s_right);
                lg.effective_green_s = Some(s.g_through);
                groups.push(lg);
            }
        }
        self.lane_groups = groups;
    }

    /// Step 5 (HCM Equation 19-15): `P = R_p (g / C)` per lane group.
    pub fn step_5_proportion_arriving_on_green(&mut self) -> Vec<f64> {
        let c = self.cycle_length_s;
        let approaches = self.approaches.clone();
        let mut out = Vec::with_capacity(self.lane_groups.len());
        for lg in &mut self.lane_groups {
            let ap = approaches
                .iter()
                .find(|a| a.direction == lg.direction)
                .expect("lane group approach");
            let rp = match lg.kind {
                LaneGroupKind::ExclusiveLeft | LaneGroupKind::SharedLeftThrough => {
                    ap.platoon_ratio_left
                }
                _ => ap.platoon_ratio_through,
            };
            let g = lg.effective_green_s.unwrap_or(0.0);
            let p = (rp * g / c).min(1.0);
            lg.proportion_on_green = Some(p);
            out.push(p);
        }
        out
    }

    /// Step 7: capacity (HCM Equations 19-16 and 31-117..31-127) and
    /// volume-to-capacity ratio (HCM Equation 19-17) per lane group.
    fn step_7_capacity_and_vc(&mut self, st: &[ApproachState]) {
        let c_len = self.cycle_length_s;
        let ns = self.sneakers_per_cycle;
        let approaches = self.approaches.clone();
        for lg in &mut self.lane_groups {
            let idx = approaches
                .iter()
                .position(|a| a.direction == lg.direction)
                .expect("approach");
            let ap = &approaches[idx];
            let s = &st[idx];
            let n = lg.lanes as f64;
            let (cap, avail) = match (lg.kind, ap.left_turn_mode) {
                (LaneGroupKind::ExclusiveLeft, LeftTurnMode::Permitted) => {
                    // Eq. 31-119 / Eq. 31-120 (f_ms = f_sp = 1.0).
                    let c_le = (s.g_u * s.s_left_perm + 3_600.0 * ns) / c_len * n;
                    let gmax = ap
                        .through_phase
                        .max_green_s
                        .unwrap_or(ap.through_phase.duration_s - ap.through_phase.change_period_s());
                    let ca = c_le + (gmax - s.g_p).max(0.0) * s.s_left_perm / c_len * n;
                    (c_le, ca)
                }
                (LaneGroupKind::ExclusiveLeft, LeftTurnMode::ProtectedPermitted) => {
                    // Eq. 31-124 / Eq. 31-125.
                    let cap = (s.g_left * s.s_left_prot + s.g_u * s.s_left_perm + 3_600.0 * ns)
                        / c_len
                        * n;
                    let gmax_l = ap
                        .left_phase
                        .as_ref()
                        .and_then(|p| p.max_green_s)
                        .unwrap_or(s.g_left);
                    let ca = (gmax_l * s.s_left_prot + s.g_u * s.s_left_perm + 3_600.0 * ns)
                        / c_len
                        * n;
                    (cap, ca.max(cap))
                }
                (LaneGroupKind::SharedLeftThrough, m)
                    if matches!(m, LeftTurnMode::Permitted | LeftTurnMode::ProtectedPermitted) =>
                {
                    // Eq. 31-121 (permitted); Eq. 31-126 adds the protected
                    // period at ssl4 (Eq. 31-113) for protected-permitted.
                    let p_l = lg.p_left_shared.unwrap_or(0.0);
                    let mut cap = (s.g_p * s.s_shared_lt + 3_600.0 * (1.0 + p_l)) / c_len;
                    if m == LeftTurnMode::ProtectedPermitted {
                        let ssl4 = s.s_th_curb / (1.0 + p_l * (E_L_PROTECTED_LEFT - 1.0));
                        cap += s.g_left * ssl4 / c_len;
                    }
                    // Available capacity, Eq. 31-123 (permitted) / Eq. 31-127
                    // (protected-permitted): add the extension the movement
                    // could serve if the through phase ran to its maximum
                    // green, at the end-of-green shared-lane saturation flow
                    // s_sl3 (Eq. 31-109). Mirrors the exclusive-lane arms above;
                    // available capacity feeds the actuated k in step_8 and is
                    // inert under pretimed control (k = K_PRETIMED there).
                    let ssl3 = s.s_th_curb / (1.0 + p_l * (s.el1 / s.f_lpb - 1.0));
                    let gmax = ap.through_phase.max_green_s.unwrap_or(
                        ap.through_phase.duration_s - ap.through_phase.change_period_s(),
                    );
                    let ca = cap + (gmax - s.g_p).max(0.0) * ssl3 / c_len;
                    (cap, ca.max(cap))
                }
                _ => {
                    // Eq. 19-16: c = N s g / C, with the available capacity
                    // from Eqs. 19-24 / 19-25.
                    let g = lg.effective_green_s.unwrap_or(0.0);
                    let sat = lg.sat_flow.unwrap_or(0.0);
                    let cap = n * sat * g / c_len;
                    let ga = ap.through_phase.available_effective_green_s();
                    let ca = n * sat * ga / c_len;
                    (cap, ca.max(cap))
                }
            };
            lg.capacity = Some(cap);
            lg.available_capacity = Some(avail);
            lg.vc_ratio = Some(if cap > 0.0 {
                lg.flow_rate / cap
            } else {
                f64::INFINITY
            });
        }
    }

    /// Step 8 (HCM Equations 19-18 through 19-27 and 19-44 through 19-49):
    /// uniform, incremental, and initial-queue delay per lane group.
    fn step_8_delay(&mut self, st: &[ApproachState]) {
        let c_len = self.cycle_length_s;
        let t_h = self.analysis_period_h;
        let pretimed = matches!(self.control, ControlType::PretimedSignal);
        let approaches = self.approaches.clone();
        let sneakers = self.sneakers_per_cycle;
        for lg in &mut self.lane_groups {
            let idx = approaches
                .iter()
                .position(|a| a.direction == lg.direction)
                .expect("approach");
            let ap = &approaches[idx];
            let s = &st[idx];
            let x = lg.vc_ratio.unwrap_or(0.0);
            let cap = lg.capacity.unwrap_or(0.0);
            let p = lg.proportion_on_green.unwrap_or(0.0);
            let n = lg.lanes.max(1) as f64;

            // ── d1: uniform delay ────────────────────────────────────────
            let uses_qap = matches!(
                (lg.kind, ap.left_turn_mode),
                (LaneGroupKind::ExclusiveLeft, LeftTurnMode::Permitted)
                    | (LaneGroupKind::ExclusiveLeft, LeftTurnMode::ProtectedPermitted)
                    | (LaneGroupKind::SharedLeftThrough, LeftTurnMode::Permitted)
                    | (LaneGroupKind::SharedLeftThrough, LeftTurnMode::ProtectedPermitted)
            );
            let d1 = if uses_qap {
                // Incremental queue accumulation (HCM Ch. 19 §4,
                // Eqs. 19-32..19-36; polygon shapes of Exhibits
                // 31-13..31-16).
                let intervals = build_left_turn_qap(lg, ap, s, x, c_len, sneakers);
                // Arrival rate per lane, capped at capacity like the
                // polygon arrivals (general QAP procedure Step 5).
                let mut q_avg = lg.flow_rate / n / 3_600.0;
                if x > 1.0 {
                    q_avg /= x;
                }
                let res = qap_evaluate(&intervals, c_len, q_avg);
                // First-term back of queue for Step 10 from the left-turn
                // arrival–departure polygon family (HCM Exhibits 31-26
                // through 31-31, Equation 31-141): the number of full stops
                // N_f, not the instantaneous maximum queue. For lane groups
                // served in two batches per cycle (e.g., a protected phase
                // plus permitted/sneaker service) N_f exceeds the peak queue
                // because it counts every vehicle that stops.
                let d_a = accel_decel_delay(ap.speed_limit_mph);
                lg.q1_veh = Some(adp_first_term_left(&intervals, c_len, d_a));
                res.uniform_delay_s
            } else {
                // Eq. 19-19 with the Eq. 19-20 progression factor.
                let g = lg.effective_green_s.unwrap_or(0.0);
                let g_over_c = (g / c_len).min(0.999_999);
                let pf = progression_factor(p, g_over_c, x);
                uniform_delay(c_len, g, x, pf)
            };

            // ── d2: incremental delay ────────────────────────────────────
            let k = if pretimed {
                K_PRETIMED
            } else {
                // Eqs. 19-22 through 19-25 when actuated settings are
                // available; K_PRETIMED for coordinated phases otherwise.
                let phase = match (lg.kind, ap.left_turn_mode) {
                    (
                        LaneGroupKind::ExclusiveLeft,
                        LeftTurnMode::Protected | LeftTurnMode::ProtectedPermitted,
                    ) => ap.left_phase.as_ref().unwrap_or(&ap.through_phase),
                    _ => &ap.through_phase,
                };
                match (phase.passage_time_s, phase.max_green_s) {
                    (Some(pt), Some(_)) => {
                        let k_min = incremental_delay_factor_min(pt);
                        let ca = lg.available_capacity.unwrap_or(cap).max(1e-9);
                        incremental_delay_factor_actuated(lg.flow_rate / ca, k_min)
                    }
                    _ => K_PRETIMED,
                }
            };
            lg.k_factor = Some(k);
            let d2 = if cap > 0.0 {
                incremental_delay_signalized(t_h, x, cap, k, ap.upstream_filtering_i)
            } else {
                0.0
            };

            // ── d3: initial queue delay (Eqs. 19-44..19-49) ─────────────
            // Milestone 1 uses c_A = c (the saturated-capacity re-timing of
            // HCM Ch. 19 §4 Steps A–C requires the actuated engine).
            let q_b = initial_queue_for_group(lg, ap);
            let d3 = initial_queue_delay(q_b, lg.flow_rate, cap.max(1e-9), t_h);

            lg.uniform_delay_s = Some(d1);
            lg.incremental_delay_s = Some(d2);
            lg.initial_queue_delay_s = Some(d3);
            lg.control_delay_s = Some(control_delay_signalized(d1, d2, d3));
        }
    }

    /// Step 9 (HCM Exhibit 19-8 with Equations 19-28 and 19-29): LOS per
    /// lane group, approach, and intersection.
    fn step_9_los(&mut self) {
        for lg in &mut self.lane_groups {
            let d = lg.control_delay_s.unwrap_or(0.0);
            let x = lg.vc_ratio.unwrap_or(0.0);
            lg.los = Some(los_signalized_intersection(d, x > 1.0));
        }
        let mut approach_results = Vec::new();
        for ap in &self.approaches {
            let pairs: Vec<(f64, f64)> = self
                .lane_groups
                .iter()
                .filter(|lg| lg.direction == ap.direction)
                .map(|lg| (lg.control_delay_s.unwrap_or(0.0), lg.flow_rate))
                .collect();
            let d_a = aggregate_control_delay(&pairs); // Eq. 19-28
            approach_results.push(ApproachResult {
                direction: ap.direction,
                flow_rate: pairs.iter().map(|(_, v)| v).sum(),
                control_delay_s: d_a,
                los: los_signalized_intersection(d_a, false),
            });
        }
        self.approach_results = approach_results;
        let pairs: Vec<(f64, f64)> = self
            .lane_groups
            .iter()
            .map(|lg| (lg.control_delay_s.unwrap_or(0.0), lg.flow_rate))
            .collect();
        let d_i = aggregate_control_delay(&pairs); // Eq. 19-29
        self.intersection_delay_s = Some(d_i);
        self.intersection_los = Some(los_signalized_intersection(d_i, false));
    }

    /// Step 10 (HCM Ch. 31, Section 4): back of queue and queue storage
    /// ratio. The basic arrival–departure polygon (Exhibit 31-25) covers
    /// through, shared through+right, and protected-turn lane groups; for
    /// permitted / protected-permitted left-turn lane groups the maximum
    /// queue of the Step 8 queue accumulation polygon is used as the Q1
    /// estimate (the left-turn ADP family of Exhibits 31-26 through 31-31
    /// is a milestone-2 item).
    fn step_10_queue_storage(&mut self) {
        let c_len = self.cycle_length_s;
        let t_h = self.analysis_period_h;
        let approaches = self.approaches.clone();
        for lg in &mut self.lane_groups {
            let ap = approaches
                .iter()
                .find(|a| a.direction == lg.direction)
                .expect("approach");
            let n = lg.lanes.max(1);
            let cap = lg.capacity.unwrap_or(0.0);
            let cap_ln = cap / n as f64;
            let v_ln = lg.flow_rate / n as f64;
            let d_a = accel_decel_delay(ap.speed_limit_mph);
            let is_perm_left = matches!(
                lg.kind,
                LaneGroupKind::ExclusiveLeft | LaneGroupKind::SharedLeftThrough
            ) && matches!(
                ap.left_turn_mode,
                LeftTurnMode::Permitted | LeftTurnMode::ProtectedPermitted
            );
            let q1 = if is_perm_left {
                lg.q1_veh.unwrap_or(0.0)
            } else {
                first_term_back_of_queue(
                    v_ln,
                    cap_ln,
                    lg.sat_flow.unwrap_or(0.0),
                    lg.proportion_on_green.unwrap_or(0.0),
                    lg.effective_green_s.unwrap_or(0.0),
                    c_len,
                    d_a,
                )
            };
            let q2 = second_term_back_of_queue(cap, n, lg.incremental_delay_s.unwrap_or(0.0));
            let q_b = initial_queue_for_group(lg, ap);
            let q3 = third_term_back_of_queue(q_b, lg.flow_rate, cap, n, t_h);
            let q50 = q1 + q2 + q3; // Eq. 31-149
            let q95 = percentile_back_of_queue(
                q1,
                q2,
                q3,
                1.64,
                ap.upstream_filtering_i,
                lg.effective_green_s.unwrap_or(0.0) / c_len,
                lg.vc_ratio.unwrap_or(0.0),
                lg.flow_rate >= cap && cap > 0.0,
            );
            lg.q1_veh = Some(q1);
            lg.q2_veh = Some(q2);
            lg.q3_veh = Some(q3);
            lg.back_of_queue_veh = Some(q50);
            lg.back_of_queue_95_veh = Some(q95);
            let pct_hv = match lg.kind {
                LaneGroupKind::ExclusiveLeft | LaneGroupKind::SharedLeftThrough => {
                    ap.pct_heavy_vehicles_left
                }
                _ => ap.pct_heavy_vehicles_through,
            };
            let l_h = average_vehicle_spacing(pct_hv); // Eq. 31-155
            let storage = match lg.kind {
                LaneGroupKind::ExclusiveLeft => ap.storage_left_ft,
                _ => ap.storage_through_ft,
            };
            if let Some(la) = storage {
                lg.queue_storage_ratio = Some(queue_storage_ratio_eq(l_h, q50, la));
                lg.queue_storage_ratio_95 = Some(queue_storage_ratio_eq(l_h, q95, la));
            }
        }
    }

    /// Critical intersection volume-to-capacity ratio X_c (HCM Equations
    /// 19-30 and 19-31 with the critical-path rules of Chapter 19,
    /// Section 4).
    fn compute_critical_vc(&self, st: &[ApproachState]) -> f64 {
        let c = self.cycle_length_s;
        let mut sum_yc = 0.0;
        let mut lost = 0.0;
        for pair in [
            [Direction::NB, Direction::SB],
            [Direction::EB, Direction::WB],
        ] {
            let idx: Vec<usize> = self
                .approaches
                .iter()
                .enumerate()
                .filter(|(_, a)| pair.contains(&a.direction))
                .map(|(i, _)| i)
                .collect();
            if idx.is_empty() {
                continue;
            }
            // Per-approach phase flow ratios: y_through is the max over the
            // through-phase lane groups; the protected-left flow ratio
            // apportions volume to the protected phase first (Ch. 19 §4).
            struct Ratios {
                y_through: f64,
                y_left_prot: f64,
                y_left_perm: f64,
                lt_through: f64,
                lt_left: f64,
            }
            let ratios: Vec<Ratios> = idx
                .iter()
                .map(|&i| {
                    let ap = &self.approaches[i];
                    let s = &st[i];
                    let y_through = self
                        .lane_groups
                        .iter()
                        .filter(|lg| {
                            lg.direction == ap.direction
                                && lg.kind != LaneGroupKind::ExclusiveLeft
                        })
                        .map(|lg| {
                            let sat = lg.sat_flow.unwrap_or(0.0) * lg.lanes as f64;
                            if sat > 0.0 {
                                lg.flow_rate / sat
                            } else {
                                0.0
                            }
                        })
                        .fold(0.0_f64, f64::max);
                    let (y_left_prot, y_left_perm, lt_left) = match ap.left_turn_mode {
                        LeftTurnMode::Protected | LeftTurnMode::ProtectedPermitted => {
                            let n_l = ap.exclusive_left_lanes.max(1) as f64;
                            let cap_prot = n_l * s.s_left_prot * s.g_left / c;
                            let v_prot = s.v_left_group.min(cap_prot);
                            let v_perm = (s.v_left_group - v_prot).max(0.0);
                            let y_p = if s.s_left_prot > 0.0 {
                                v_prot / (n_l * s.s_left_prot)
                            } else {
                                0.0
                            };
                            let y_perm = if s.s_left_perm > 0.0 {
                                v_perm / (n_l * s.s_left_perm)
                            } else {
                                0.0
                            };
                            (
                                y_p,
                                y_perm,
                                ap.left_phase
                                    .as_ref()
                                    .map(|p| p.lost_time_s())
                                    .unwrap_or(0.0),
                            )
                        }
                        LeftTurnMode::Permitted => {
                            let n_l = ap.exclusive_left_lanes.max(1) as f64;
                            let y_perm = if s.s_left_perm > 0.0 {
                                s.v_left_group / (n_l * s.s_left_perm)
                            } else {
                                0.0
                            };
                            (0.0, y_perm, 0.0)
                        }
                        _ => (0.0, 0.0, 0.0),
                    };
                    Ratios {
                        y_through,
                        y_left_prot,
                        y_left_perm,
                        lt_through: ap.through_phase.lost_time_s(),
                        lt_left,
                    }
                })
                .collect();

            let (mut best_sum, mut best_lost);
            if ratios.len() == 2 {
                // Rules 1 and 2: subject protected left + opposing through
                // in each ring.
                let ring1 = ratios[0].y_left_prot + ratios[1].y_through;
                let ring2 = ratios[1].y_left_prot + ratios[0].y_through;
                if ring1 >= ring2 {
                    best_sum = ring1;
                    best_lost = ratios[1].lt_through
                        + if ratios[0].y_left_prot > 0.0 {
                            ratios[0].lt_left
                        } else {
                            0.0
                        };
                } else {
                    best_sum = ring2;
                    best_lost = ratios[0].lt_through
                        + if ratios[1].y_left_prot > 0.0 {
                            ratios[1].lt_left
                        } else {
                            0.0
                        };
                }
                // Rule 3 (lead–lead / lag–lag protected-permitted): the
                // protected plus permitted flow ratios of the same
                // left-turn lane group; only one phase lost time applies
                // (l1 of the first phase + l2 of the second = l_t once).
                for r in &ratios {
                    let path = r.y_left_prot + r.y_left_perm;
                    if path > best_sum {
                        best_sum = path;
                        best_lost = r.lt_left.max(r.lt_through);
                    }
                }
            } else {
                best_sum = ratios[0].y_through + ratios[0].y_left_prot;
                best_lost = ratios[0].lt_through
                    + if ratios[0].y_left_prot > 0.0 {
                        ratios[0].lt_left
                    } else {
                        0.0
                    };
            }
            sum_yc += best_sum;
            lost += best_lost;
        }
        if c <= lost {
            return f64::INFINITY;
        }
        critical_vc_ratio_eq(c, lost, sum_yc) // Eq. 19-30
    }
}

/// Initial queue Q_b for a lane group, distributed from the movement-group
/// input per HCM Chapter 19, Section 4, Step A (lane-count proration for
/// shared lanes).
fn initial_queue_for_group(lg: &LaneGroup, ap: &SignalApproach) -> f64 {
    match lg.kind {
        LaneGroupKind::ExclusiveLeft | LaneGroupKind::SharedLeftThrough => {
            ap.initial_queue_left_veh
        }
        _ => {
            let n_th = ap.through_serving_lanes().max(1) as f64;
            ap.initial_queue_through_veh * lg.lanes as f64 / n_th
        }
    }
}

/// Build the queue accumulation polygon intervals for a permitted or
/// protected-permitted left-turn lane group (HCM Exhibits 31-13 through
/// 31-16 shapes). Times are in effective green/red terms and the interval
/// list covers exactly one cycle.
fn build_left_turn_qap(
    lg: &LaneGroup,
    ap: &SignalApproach,
    s: &ApproachState,
    x: f64,
    cycle_s: f64,
    sneakers_per_cycle: f64,
) -> Vec<QapInterval> {
    let n = lg.lanes.max(1) as f64;
    // Arrival rate per lane, capped at capacity (general QAP procedure
    // Step 5: q' = q / X when X > 1).
    let mut q = lg.flow_rate / n / 3_600.0;
    if x > 1.0 {
        q /= x;
    }
    let g_p = s.g_p;
    let g_u = s.g_u.min(g_p);
    let blocked = (g_p - g_u).max(0.0);

    if lg.kind == LaneGroupKind::SharedLeftThrough {
        // Exhibit 31-14 (permitted shared lane): g_f at s_th, g_diff at
        // ssl2, remainder at ssl3, sneakers (1 + P_L) at the end. For the
        // protected-permitted shared lane the protected period (ssl4,
        // Eq. 31-113) is appended per Exhibit 31-17 (leading case).
        let g_f = s.g_f.min(g_p);
        let g_diff = (g_p - g_u - g_f).max(0.0);
        let p_l = lg.p_left_shared.unwrap_or(0.0);
        let ssl2 = s.s_th_curb / (1.0 + p_l * (s.el2 / s.f_lpb - 1.0));
        let ssl3 = s.s_th_curb / (1.0 + p_l * (s.el1 / s.f_lpb - 1.0));
        let sneak = 1.0 + p_l;
        let mut iv = Vec::new();
        if ap.left_turn_mode == LeftTurnMode::ProtectedPermitted && s.g_left > 0.0 {
            let ssl4 = s.s_th_curb / (1.0 + p_l * (E_L_PROTECTED_LEFT - 1.0));
            iv.push(QapInterval {
                duration_s: s.g_left,
                discharge_veh_h: ssl4,
                arrival_veh_s: q,
                sneakers_veh: 0.0,
            });
        }
        let g_prot = if ap.left_turn_mode == LeftTurnMode::ProtectedPermitted {
            s.g_left
        } else {
            0.0
        };
        iv.extend([
            QapInterval {
                duration_s: g_f,
                discharge_veh_h: s.s_th_curb,
                arrival_veh_s: q,
                sneakers_veh: 0.0,
            },
            QapInterval {
                duration_s: g_diff,
                discharge_veh_h: ssl2,
                arrival_veh_s: q,
                sneakers_veh: 0.0,
            },
            QapInterval {
                duration_s: (g_p - g_f - g_diff).max(0.0),
                discharge_veh_h: ssl3,
                arrival_veh_s: q,
                sneakers_veh: sneak,
            },
            QapInterval {
                duration_s: (cycle_s - g_p - g_prot).max(0.0),
                discharge_veh_h: 0.0,
                arrival_veh_s: q,
                sneakers_veh: 0.0,
            },
        ]);
        return iv;
    }

    match ap.left_turn_mode {
        LeftTurnMode::Permitted => {
            // Exhibit 31-13: red, blocked permitted, unblocked permitted
            // at s_l; sneakers at the end of the permitted period.
            vec![
                QapInterval {
                    duration_s: (cycle_s - g_p).max(0.0),
                    discharge_veh_h: 0.0,
                    arrival_veh_s: q,
                    sneakers_veh: 0.0,
                },
                QapInterval {
                    duration_s: blocked,
                    discharge_veh_h: 0.0,
                    arrival_veh_s: q,
                    sneakers_veh: 0.0,
                },
                QapInterval {
                    duration_s: g_u,
                    discharge_veh_h: s.s_left_perm,
                    arrival_veh_s: q,
                    sneakers_veh: sneakers_per_cycle,
                },
            ]
        }
        _ => {
            // Protected-permitted, exclusive lane (Exhibits 31-15 / 31-16).
            let g_l = s.g_left;
            let dp_l = ap.left_phase.as_ref().map(|p| p.duration_s).unwrap_or(0.0);
            let leading = matches!(
                ap.left_turn_sequence,
                LeftTurnSequence::LeadLead | LeftTurnSequence::LeadLag
            );
            if leading {
                // Gap between the end of the protected effective green and
                // the start of the permitted effective green: the permitted
                // period begins when both through phases are green
                // (i.e., after the longer of the two leading left phases
                // for Lead–Lead, or after the subject left phase for
                // Lead–Lag).
                let prot_end = START_UP_LOST_TIME + g_l;
                let perm_start = match ap.left_turn_sequence {
                    LeftTurnSequence::LeadLead => dp_l.max(s.dp_opp_left) + s.l1p,
                    _ => dp_l + s.l1p,
                };
                let gap_s = (perm_start - prot_end).max(0.0);
                let red_tail = (cycle_s - (g_l + gap_s + g_p)).max(0.0);
                vec![
                    QapInterval {
                        duration_s: g_l,
                        discharge_veh_h: s.s_left_prot,
                        arrival_veh_s: q,
                        sneakers_veh: 0.0,
                    },
                    QapInterval {
                        duration_s: gap_s,
                        discharge_veh_h: 0.0,
                        arrival_veh_s: q,
                        sneakers_veh: 0.0,
                    },
                    QapInterval {
                        duration_s: blocked,
                        discharge_veh_h: 0.0,
                        arrival_veh_s: q,
                        sneakers_veh: 0.0,
                    },
                    QapInterval {
                        duration_s: g_u,
                        discharge_veh_h: s.s_left_perm,
                        arrival_veh_s: q,
                        sneakers_veh: sneakers_per_cycle,
                    },
                    QapInterval {
                        duration_s: red_tail,
                        discharge_veh_h: 0.0,
                        arrival_veh_s: q,
                        sneakers_veh: 0.0,
                    },
                ]
            } else {
                // Lagging left (Exhibit 31-16): permitted (blocked then
                // unblocked), protected phase, red.
                let red_tail = (cycle_s - (g_p + g_l)).max(0.0);
                vec![
                    QapInterval {
                        duration_s: blocked,
                        discharge_veh_h: 0.0,
                        arrival_veh_s: q,
                        sneakers_veh: 0.0,
                    },
                    QapInterval {
                        duration_s: g_u,
                        discharge_veh_h: s.s_left_perm,
                        arrival_veh_s: q,
                        sneakers_veh: sneakers_per_cycle,
                    },
                    QapInterval {
                        duration_s: g_l,
                        discharge_veh_h: s.s_left_prot,
                        arrival_veh_s: q,
                        sneakers_veh: 0.0,
                    },
                    QapInterval {
                        duration_s: red_tail,
                        discharge_veh_h: 0.0,
                        arrival_veh_s: q,
                        sneakers_veh: 0.0,
                    },
                ]
            }
        }
    }
}
