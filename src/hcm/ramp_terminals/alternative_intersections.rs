//! HCM Chapter 23, Part C: Alternative Intersection Evaluation
//! (RCUT, MUT, and DLT operational analysis).
//!
//! The computational steps follow HCM 7th Edition Exhibit 23-47 (the same
//! 10-step framework used for interchanges in Part B). Only the Part C–
//! specific steps are modeled here; the junction-level control delays are
//! produced by the Chapter 19 (signalized) and Chapter 20 (two-way STOP)
//! engines and enter this module as junction steps:
//!
//! * Step 1 — O-D demands redistributed to component-junction movements
//!   (Equation 23-57 flow-rate conversion; the redistribution table is the
//!   analyst's input, matching Exhibits 34-124/34-127/34-131/34-135).
//! * Steps 2–3 — lane / movement groups and lane utilization are performed
//!   by the Chapter 19 engine for signalized junctions (not repeated here).
//! * Step 4 — signal progression (arrival types of Exhibit 23-51 or the
//!   Chapter 18 flow-profile procedure) feeds the Chapter 19 junction
//!   delay; carried as an input to the signalized junction step.
//! * Step 5 — additional control-based adjustments: the U-turn crossover
//!   saturation adjustment (Exhibit 23-52) and the default STOP critical
//!   headway (4.4 s) / follow-up time (2.6 s) for a U-turn crossover.
//! * Step 6 — junction-specific performance: STOP-controlled junctions are
//!   evaluated here with the Chapter 20 gap-acceptance capacity
//!   (Equation 20-18) and delay (Equation 20-61); signalized junctions
//!   supply the Chapter 19 incremental-queue-accumulation control delay.
//! * Step 7 — extra distance travel time EDTT (Equations 23-58 / 23-59).
//! * Step 8 — additional weaving delay (RCUT with merges only; analyst
//!   supplied).
//! * Step 9 — experienced travel time ETT (Equation 23-60), with approach
//!   and intersection aggregation (Equations 23-61 / 23-62).
//! * Step 10 — LOS from Exhibit 23-13 (`los_alternative_intersection_od`).
//!
//! DLT intersections (Exhibit 23-53 layout) are analyzed as extensions of
//! the urban-street / signalized-intersection procedures: the Step 5 offset
//! computation (Equations 23-63 through 23-68) and the volume-weighted
//! control-delay aggregation (Equation 23-69) are provided here; the
//! per-junction control delays come from the Chapter 18 / Chapter 19
//! procedures.
//!
//! Sources (HCM 7th Edition EPUB): 178_Ch23_pt3_01.xhtml (introduction),
//! 179_Ch23_pt3_02.xhtml (concepts), 180_Ch23_pt3_03.xhtml (core
//! methodology, Equations 23-57 through 23-69 and Exhibits 23-47 through
//! 23-55), 181_Ch23_pt3_04.xhtml (extensions), 182_Ch23_pt3_05.xhtml
//! (applications). Numeric conventions cross-checked against Chapter 34
//! Example Problems 12–17 (269_Ch34_02b.xhtml and 269_Ch34_02c.xhtml,
//! Exhibits 34-123 through 34-150).

use serde::{Deserialize, Serialize};

use super::exhibits::los_alternative_intersection_od;
use crate::hcm::twsc::twsc::Twsc;
use crate::hcm::common::delay::control_delay_unsignalized;
use crate::hcm::common::gap_acceptance::potential_capacity;
use crate::hcm::common::LevelOfService;

// ═══════════════════════════════════════════════════════════════════════════════
// Constants (Part C, Section 3)
// ═══════════════════════════════════════════════════════════════════════════════

/// Conversion factor from mi/h to ft/s used throughout the Part C EDTT and
/// offset equations (HCM Equations 23-58, 23-59, 23-63).
pub const MPH_TO_FT_S: f64 = 1.47;

/// Deceleration/acceleration delay term `a` for a minor-street left-turn
/// movement in the RCUT-with-merges EDTT (HCM Equation 23-58 discussion:
/// "For minor-street left-turn movements, a is assumed to be 10 s"), s.
pub const EDTT_MERGE_ACCEL_DECEL_MINOR_LEFT_S: f64 = 10.0;

/// Deceleration/acceleration delay term `a` for a minor-street through
/// movement in the RCUT-with-merges EDTT (HCM Equation 23-58 discussion:
/// "for a minor-street through movement, it is assumed to be 15 s"), s.
pub const EDTT_MERGE_ACCEL_DECEL_MINOR_THROUGH_S: f64 = 15.0;

/// Default base critical headway t_c for a STOP-controlled U-turn crossover
/// at an RCUT or MUT (HCM Chapter 23 Step 5: "a critical headway of 4.4 s
/// and a follow-up time of 2.6 s were observed" at a three-through-lane,
/// 55-mi/h site), s.
pub const UTURN_CROSSOVER_DEFAULT_CRITICAL_HEADWAY_S: f64 = 4.4;

/// Default follow-up time t_f for a STOP-controlled U-turn crossover at an
/// RCUT or MUT (HCM Chapter 23 Step 5), s.
pub const UTURN_CROSSOVER_DEFAULT_FOLLOWUP_HEADWAY_S: f64 = 2.6;

// ═══════════════════════════════════════════════════════════════════════════════
// Exhibit 23-52: U-turn crossover saturation flow adjustment factor
// ═══════════════════════════════════════════════════════════════════════════════

/// HCM Exhibit 23-52: default saturation flow rate adjustment factor for a
/// signalized MUT/RCUT U-turn crossover, by median width.
///
/// * Narrow (< 35 ft): 0.80
/// * Typical (35–80 ft): 0.85
/// * Very wide (> 80 ft): 0.95
///
/// Applied as an extra saturation flow adjustment factor (Step 5) at the
/// U-turn crossover lane group of the Chapter 19 analysis.
pub fn uturn_saturation_adjustment(median_width_ft: f64) -> f64 {
    if median_width_ft < 35.0 {
        0.80
    } else if median_width_ft <= 80.0 {
        0.85
    } else {
        0.95
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Step 7: extra distance travel time (Equations 23-58 / 23-59)
// ═══════════════════════════════════════════════════════════════════════════════

/// HCM Equation 23-58: extra distance travel time for a rerouted movement
/// at an RCUT with merges
///
/// `EDTT = (D_t + D_f) / (1.47 × S_f) + a`
///
/// * `dist_to_crossover_ft` — distance from the main junction to the U-turn
///   crossover D_t, ft
/// * `dist_from_crossover_ft` — distance from the U-turn crossover back to
///   the main junction D_f, ft (equal to D_t when the crossovers are
///   symmetric)
/// * `free_flow_speed_mph` — major-street free-flow speed S_f, mi/h
/// * `accel_decel_s` — deceleration/acceleration delay term `a`, s
///   ([`EDTT_MERGE_ACCEL_DECEL_MINOR_LEFT_S`] = 10 s for a minor-street left
///   turn, [`EDTT_MERGE_ACCEL_DECEL_MINOR_THROUGH_S`] = 15 s for a minor
///   through)
///
/// EDTT is experienced as a round trip from the main junction to the U-turn
/// crossover and back.
pub fn edtt_merge(
    dist_to_crossover_ft: f64,
    dist_from_crossover_ft: f64,
    free_flow_speed_mph: f64,
    accel_decel_s: f64,
) -> f64 {
    if free_flow_speed_mph <= 0.0 {
        return 0.0;
    }
    (dist_to_crossover_ft + dist_from_crossover_ft) / (MPH_TO_FT_S * free_flow_speed_mph)
        + accel_decel_s
}

/// HCM Equation 23-59: extra distance travel time for a rerouted movement
/// at an RCUT or MUT with STOP signs or signals
///
/// `EDTT = (D_t + D_f) / (1.47 × S_f)`
///
/// There is no acceleration/deceleration term because it is already
/// captured by the STOP or signal control-delay computation.
pub fn edtt_stop_or_signal(
    dist_to_crossover_ft: f64,
    dist_from_crossover_ft: f64,
    free_flow_speed_mph: f64,
) -> f64 {
    if free_flow_speed_mph <= 0.0 {
        return 0.0;
    }
    (dist_to_crossover_ft + dist_from_crossover_ft) / (MPH_TO_FT_S * free_flow_speed_mph)
}

// ═══════════════════════════════════════════════════════════════════════════════
// Step 6: STOP-controlled junction delay (Chapter 20 primitives)
// ═══════════════════════════════════════════════════════════════════════════════

/// Result of evaluating a STOP-controlled junction movement (an RCUT/MUT
/// main-junction minor movement, or a U-turn crossover) with the Chapter 20
/// gap-acceptance procedure.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct StopJunctionResult {
    /// Movement (or potential) capacity c, veh/h (Equation 20-18).
    pub capacity_veh_h: f64,
    /// Volume-to-capacity ratio v/c.
    pub vc_ratio: f64,
    /// Control delay d, s/veh (Equation 20-61).
    pub control_delay_s: f64,
    /// 95th-percentile queue Q_95, veh (Equation 20-66).
    pub queue_95_veh: f64,
}

/// Evaluate a STOP-controlled junction movement with the Chapter 20
/// gap-acceptance capacity (Equation 20-18) and control-delay (Equation
/// 20-61) equations.
///
/// * `flow_veh_h` — movement demand flow rate v, veh/h
/// * `conflicting_flow_veh_h` — conflicting flow rate v_c, veh/h
/// * `critical_headway_s` — adjusted critical headway t_c,x, s
/// * `followup_headway_s` — adjusted follow-up headway t_f,x, s
/// * `analysis_period_h` — analysis period T, h (typically 0.25)
///
/// This models a rank-2 movement with no capacity impedance from higher-
/// rank conflicting streams (the movement capacity equals the potential
/// capacity), which is the situation at RCUT/MUT crossovers and the
/// redistributed main-junction minor movements (Chapter 34 Example
/// Problem 13, Exhibit 34-128). Higher-rank impedance, when present, is
/// applied through the Chapter 20 engine directly.
pub fn stop_junction_delay(
    flow_veh_h: f64,
    conflicting_flow_veh_h: f64,
    critical_headway_s: f64,
    followup_headway_s: f64,
    analysis_period_h: f64,
) -> StopJunctionResult {
    let capacity =
        potential_capacity(conflicting_flow_veh_h, critical_headway_s, followup_headway_s);
    if capacity <= 0.0 {
        return StopJunctionResult {
            capacity_veh_h: 0.0,
            vc_ratio: f64::INFINITY,
            control_delay_s: f64::INFINITY,
            queue_95_veh: 0.0,
        };
    }
    StopJunctionResult {
        capacity_veh_h: capacity,
        vc_ratio: flow_veh_h / capacity,
        control_delay_s: control_delay_unsignalized(flow_veh_h, capacity, analysis_period_h),
        queue_95_veh: Twsc::queue_95(flow_veh_h, capacity, analysis_period_h),
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Movement journeys, junction steps, and the facility (Steps 1, 6, 7, 9, 10)
// ═══════════════════════════════════════════════════════════════════════════════

/// Alternative-intersection forms covered by the Part C methodology
/// (Exhibits 23-41 through 23-45, 23-53, 23-54).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AltIntersectionForm {
    /// Four-legged restricted crossing U-turn (Exhibit 23-41 / 23-42).
    RcutFourLeg,
    /// Three-legged restricted crossing U-turn (Exhibit 23-43).
    RcutThreeLeg,
    /// Four-legged median U-turn (Exhibit 23-44).
    MutFourLeg,
    /// Three-legged median U-turn.
    MutThreeLeg,
    /// Partial displaced left-turn intersection (Exhibit 23-53).
    DltPartial,
    /// Full displaced left-turn intersection (Exhibit 23-54).
    DltFull,
}

/// Junction control type encountered along a movement's journey.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum JunctionControl {
    /// Signalized junction — control delay from the Chapter 19 IQA
    /// procedure (supplied to this module as a `Provided` junction step).
    Signal,
    /// STOP- or YIELD-controlled junction — control delay from the
    /// Chapter 20 gap-acceptance procedure.
    Stop,
    /// Free-flow merge onto the major street (RCUT with merges); control
    /// delay is assumed to be zero (Step 5 / Step 6).
    Merge,
}

/// A single junction encountered by a movement on its journey through the
/// alternative intersection (an element of the Exhibit 23-48 / 23-49 /
/// 23-50 traversal tables).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum JunctionStep {
    /// Control delay supplied from an upstream analysis (a Chapter 19
    /// signalized junction, or a merge with a nonzero analyst-supplied
    /// weaving delay). `vc_gt_1` / `rq_gt_1` flag whether any lane group at
    /// the junction is over capacity or over its storage (forcing LOS F).
    Provided {
        /// Control delay d_i experienced at the junction, s/veh.
        control_delay_s: f64,
        /// True when v/c > 1 for a lane group containing this movement.
        #[serde(default)]
        vc_gt_1: bool,
        /// True when the queue-storage ratio R_Q > 1 for a lane group
        /// containing this movement.
        #[serde(default)]
        rq_gt_1: bool,
    },
    /// A STOP- or YIELD-controlled junction evaluated with the Chapter 20
    /// gap-acceptance procedure (Equations 20-18 and 20-61).
    Stop {
        /// Movement demand flow rate v, veh/h.
        flow_veh_h: f64,
        /// Conflicting flow rate v_c, veh/h.
        conflicting_flow_veh_h: f64,
        /// Adjusted critical headway t_c,x, s.
        critical_headway_s: f64,
        /// Adjusted follow-up headway t_f,x, s.
        followup_headway_s: f64,
        /// Available storage L_a, ft (`None` = not checked). When the
        /// 95th-percentile queue exceeds this, `rq_gt_1` is set.
        #[serde(default)]
        storage_ft: Option<f64>,
        /// Average stationary queue spacing L_h, ft/veh (default 25).
        #[serde(default = "default_queue_spacing_ft")]
        queue_spacing_ft: f64,
    },
    /// A free-flow merge junction: zero control delay (Step 6, RCUT with
    /// merges passing the weaving-area test).
    Merge,
}

fn default_queue_spacing_ft() -> f64 {
    25.0
}

/// Approach (compass direction) a movement belongs to, for the Equation
/// 23-61 approach aggregation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Approach {
    Eb,
    Wb,
    Nb,
    Sb,
}

/// One O-D movement's journey through an RCUT or MUT (Step 9). Holds the
/// ordered junction steps it traverses (Exhibit 23-48 / 23-49 / 23-50) and
/// the Step 7 extra distance travel time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AltMovement {
    /// Human-readable movement label (e.g. "EB L", "NB T").
    pub label: String,
    /// Approach the movement belongs to (for approach aggregation).
    pub approach: Approach,
    /// O-D demand flow rate v, veh/h (used as the aggregation weight).
    pub demand_veh_h: f64,
    /// Ordered junctions encountered (Exhibit 23-48 / 23-49 / 23-50).
    pub junctions: Vec<JunctionStep>,
    /// Extra distance travel time EDTT, s/veh (Step 7). Zero for movements
    /// that are not rerouted.
    #[serde(default)]
    pub edtt_s: f64,
    /// Analysis period T, h (for STOP junction delay). Default 0.25.
    #[serde(default = "default_analysis_period_h")]
    pub analysis_period_h: f64,
}

fn default_analysis_period_h() -> f64 {
    0.25
}

/// Computed per-movement results (Steps 9–10).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AltMovementResult {
    /// Movement label.
    pub label: String,
    /// Control delay experienced at each junction, in journey order, s/veh.
    pub junction_delays_s: Vec<f64>,
    /// Sum of junction control delays, s/veh.
    pub total_control_delay_s: f64,
    /// Extra distance travel time EDTT, s/veh.
    pub edtt_s: f64,
    /// Experienced travel time ETT (Equation 23-60), s/veh.
    pub ett_s: f64,
    /// True when any junction on the journey is over capacity (LOS F).
    pub vc_gt_1: bool,
    /// True when any junction on the journey is over its storage (LOS F).
    pub rq_gt_1: bool,
    /// LOS from Exhibit 23-13.
    pub los: LevelOfService,
}

impl AltMovement {
    /// Evaluate the movement (Steps 6, 9, 10): compute each junction's
    /// control delay, the experienced travel time (Equation 23-60), and the
    /// LOS (Exhibit 23-13).
    pub fn evaluate(&self) -> AltMovementResult {
        let mut junction_delays = Vec::with_capacity(self.junctions.len());
        let mut vc_gt_1 = false;
        let mut rq_gt_1 = false;
        for step in &self.junctions {
            match step {
                JunctionStep::Provided {
                    control_delay_s,
                    vc_gt_1: v,
                    rq_gt_1: r,
                } => {
                    junction_delays.push(*control_delay_s);
                    vc_gt_1 |= *v;
                    rq_gt_1 |= *r;
                }
                JunctionStep::Merge => junction_delays.push(0.0),
                JunctionStep::Stop {
                    flow_veh_h,
                    conflicting_flow_veh_h,
                    critical_headway_s,
                    followup_headway_s,
                    storage_ft,
                    queue_spacing_ft,
                } => {
                    let res = stop_junction_delay(
                        *flow_veh_h,
                        *conflicting_flow_veh_h,
                        *critical_headway_s,
                        *followup_headway_s,
                        self.analysis_period_h,
                    );
                    junction_delays.push(res.control_delay_s);
                    if res.vc_ratio > 1.0 {
                        vc_gt_1 = true;
                    }
                    if let Some(la) = storage_ft {
                        if res.queue_95_veh * queue_spacing_ft > *la {
                            rq_gt_1 = true;
                        }
                    }
                }
            }
        }
        let total_control_delay_s: f64 = junction_delays.iter().sum();
        let ett_s = total_control_delay_s + self.edtt_s; // Equation 23-60
        let los = los_alternative_intersection_od(ett_s, vc_gt_1, rq_gt_1);
        AltMovementResult {
            label: self.label.clone(),
            junction_delays_s: junction_delays,
            total_control_delay_s,
            edtt_s: self.edtt_s,
            ett_s,
            vc_gt_1,
            rq_gt_1,
            los,
        }
    }
}

/// An RCUT or MUT alternative intersection (Steps 1, 9, 10 assembly). DLT
/// intersections use [`dlt_offset`] and [`dlt_weighted_average_delay`]
/// instead.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlternativeIntersection {
    /// Intersection form.
    pub form: AltIntersectionForm,
    /// The O-D movements traversing the intersection.
    pub movements: Vec<AltMovement>,
}

impl AlternativeIntersection {
    /// Construct a new RCUT/MUT facility.
    pub fn new(form: AltIntersectionForm, movements: Vec<AltMovement>) -> Self {
        AlternativeIntersection { form, movements }
    }

    /// Deserialize a facility from JSON.
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Evaluate every movement (Steps 6, 9, 10).
    pub fn evaluate(&self) -> Vec<AltMovementResult> {
        self.movements.iter().map(AltMovement::evaluate).collect()
    }

    /// HCM Equation 23-61: demand-weighted approach experienced travel time
    /// `ETT_A = Σ(ETT_j v_j) / Σ v_j`, for all movements on `approach`.
    /// Returns `None` when the approach carries no demand.
    pub fn approach_ett(&self, approach: Approach) -> Option<f64> {
        let mut num = 0.0;
        let mut den = 0.0;
        for m in &self.movements {
            if m.approach == approach {
                let ett = m.evaluate().ett_s;
                num += ett * m.demand_veh_h;
                den += m.demand_veh_h;
            }
        }
        if den > 0.0 {
            Some(num / den)
        } else {
            None
        }
    }

    /// HCM Equation 23-62: demand-weighted intersection experienced travel
    /// time `ETT_I = Σ(ETT_k v_k) / Σ v_k`, over all movements.
    /// Returns `None` when the intersection carries no demand.
    pub fn intersection_ett(&self) -> Option<f64> {
        let mut num = 0.0;
        let mut den = 0.0;
        for m in &self.movements {
            let ett = m.evaluate().ett_s;
            num += ett * m.demand_veh_h;
            den += m.demand_veh_h;
        }
        if den > 0.0 {
            Some(num / den)
        } else {
            None
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// DLT: Step 5 offset computation (Equations 23-63 through 23-68)
// ═══════════════════════════════════════════════════════════════════════════════

/// Result of the DLT supplemental-intersection offset computation
/// (Equations 23-63 through 23-68).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct DltOffsetResult {
    /// Displaced left-turn roadway travel time TT_DLT, s (Equation 23-63).
    pub tt_dlt_s: f64,
    /// System start time of the displaced left-turn phase ST_DLT, s
    /// (Equation 23-64).
    pub st_dlt_s: f64,
    /// System start time of the major-street through phase ST_TH, s
    /// (Equation 23-65).
    pub st_th_s: f64,
    /// Adjusted offset at the upstream supplemental intersection O_SUPP, s
    /// (Equations 23-66 through 23-68, wrapped into `[0, C)`).
    pub offset_supp_s: f64,
}

/// HCM Equations 23-63 through 23-68: compute the supplemental-intersection
/// offset so that displaced left-turn vehicles arrive during the guaranteed
/// green window at the main intersection.
///
/// * `td_dlt_ft` — displaced left-turn roadway travel distance TD_DLT, ft
/// * `sf_dlt_mph` — free-flow speed of the displaced left-turn roadway
///   S_f,DLT, mi/h
/// * `lag_dlt_s` — duration between the reference point and the start of the
///   displaced left-turn phase at the supplemental intersection LAG_DLT, s
/// * `lag_th_s` — duration between the reference point and the start of the
///   major-street through phase at the main intersection LAG_TH, s
/// * `offset_supp_s` — initial supplemental-intersection offset O_SUPP, s
/// * `offset_main_s` — main-intersection offset O_MAIN, s
/// * `cycle_s` — background cycle length C, s
///
/// Reproduces Chapter 34 Example Problem 16 (TT_DLT = 6.8 s, LAG_TH = 52 s,
/// O_SUPP = 45.2 s; the example rounds TT_DLT to 7 s and reports 45 s).
pub fn dlt_offset(
    td_dlt_ft: f64,
    sf_dlt_mph: f64,
    lag_dlt_s: f64,
    lag_th_s: f64,
    offset_supp_s: f64,
    offset_main_s: f64,
    cycle_s: f64,
) -> DltOffsetResult {
    // Equation 23-63.
    let tt_dlt_s = if sf_dlt_mph > 0.0 {
        td_dlt_ft / (sf_dlt_mph * MPH_TO_FT_S)
    } else {
        0.0
    };
    // Equations 23-64 and 23-65.
    let st_dlt_s = lag_dlt_s + offset_supp_s;
    let st_th_s = lag_th_s + offset_main_s;
    // Equation 23-66: O_SUPP(s) = O_SUPP - ST_DLT + ST_TH - TT_DLT.
    let mut o_supp = offset_supp_s - st_dlt_s + st_th_s - tt_dlt_s;
    // Equations 23-67 / 23-68: wrap the offset into the valid range [0, C).
    if cycle_s > 0.0 {
        while o_supp >= cycle_s {
            o_supp -= cycle_s;
        }
        while o_supp < 0.0 {
            o_supp += cycle_s;
        }
    }
    DltOffsetResult {
        tt_dlt_s,
        st_dlt_s,
        st_th_s,
        offset_supp_s: o_supp,
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// DLT: Step 9/10 volume-weighted control delay (Equation 23-69)
// ═══════════════════════════════════════════════════════════════════════════════

/// One (flow, control-delay) cell of the DLT weighted-average control-delay
/// table (Exhibit 34-145 / 34-150): the flow through a component junction
/// and the control delay that flow experiences there.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct DltDelayCell {
    /// Flow rate v_j at the junction, veh/h.
    pub flow_veh_h: f64,
    /// Control delay d_j at the junction, s/veh.
    pub control_delay_s: f64,
}

/// HCM Equation 23-69: weighted-average experienced travel time (equal to
/// control delay for a DLT) for the whole DLT intersection
///
/// `ETT_DLT = Σ(d_j v_j) / Σ v_OD`
///
/// * `cells` — the per-junction (flow, delay) products of Exhibit 34-145
/// * `total_od_demand_veh_h` — the O-D demand total Σ v_OD, which for a DLT
///   must equal the conventional-intersection movement-demand total to
///   avoid double-counting trips within the spatial boundaries
///
/// For DLT intersections, ETT is assumed equal to control delay (Step 7
/// EDTT is negligible). Reproduces Chapter 34 Example Problem 16 (Σ products
/// = 159,675; Σ v_OD = 5,594; ETT_DLT = 28.5 s/veh).
pub fn dlt_weighted_average_delay(cells: &[DltDelayCell], total_od_demand_veh_h: f64) -> f64 {
    if total_od_demand_veh_h <= 0.0 {
        return 0.0;
    }
    let numerator: f64 = cells
        .iter()
        .map(|c| c.control_delay_s * c.flow_veh_h)
        .sum();
    numerator / total_od_demand_veh_h
}

/// A DLT intersection analysis (partial or full). Holds the weighted-average
/// control-delay table (Exhibit 34-145 / 34-150) and the O-D demand total.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplacedLeftTurn {
    /// `DltPartial` or `DltFull`.
    pub form: AltIntersectionForm,
    /// The per-junction (flow, delay) cells.
    pub cells: Vec<DltDelayCell>,
    /// O-D demand total Σ v_OD, veh/h.
    pub total_od_demand_veh_h: f64,
}

impl DisplacedLeftTurn {
    /// Deserialize a DLT analysis from JSON.
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Weighted-average intersection ETT (= control delay), Equation 23-69.
    pub fn intersection_ett(&self) -> f64 {
        dlt_weighted_average_delay(&self.cells, self.total_od_demand_veh_h)
    }

    /// Intersection LOS. Per Chapter 34 Example Problems 16 and 17, DLT LOS
    /// is read from the Chapter 19 signalized-intersection thresholds (ETT
    /// assumed equal to control delay), not Exhibit 23-13.
    ///
    /// // VERIFY-HCM: Part C Step 10 for RCUT/MUT uses Exhibit 23-13, but
    /// // the DLT worked examples (34-145 discussion) read LOS from the
    /// // Chapter 19 control-delay thresholds. This method follows the
    /// // worked examples; Exhibit 23-13 is available via
    /// // `los_alternative_intersection_od` if the analyst prefers it.
    pub fn los(&self) -> LevelOfService {
        crate::hcm::common::los_tables::los_signalized_intersection(self.intersection_ett(), false)
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn edtt_equations_reproduce_example_12() {
        // Chapter 34 Example Problem 12: D_t = D_f = 2,000 ft, S_f = 60 mi/h.
        let left = edtt_merge(2000.0, 2000.0, 60.0, EDTT_MERGE_ACCEL_DECEL_MINOR_LEFT_S);
        let through = edtt_merge(2000.0, 2000.0, 60.0, EDTT_MERGE_ACCEL_DECEL_MINOR_THROUGH_S);
        assert!((left - 55.4).abs() < 0.1, "minor-left EDTT {left}");
        assert!((through - 60.4).abs() < 0.1, "minor-through EDTT {through}");
    }

    #[test]
    fn edtt_stop_reproduces_example_13() {
        // Example Problem 13: D_t = D_f = 700 ft, S_f = 60 mi/h.
        let e = edtt_stop_or_signal(700.0, 700.0, 60.0);
        assert!((e - 15.9).abs() < 0.1, "EDTT {e}");
    }

    #[test]
    fn stop_junction_reproduces_example_13_uturn_crossover() {
        // Exhibit 34-128 SB U-turn crossover: v = 167, v_c = 1,189,
        // default headways 4.4 / 2.6 s, T = 0.25 h.
        let r = stop_junction_delay(
            167.0,
            1189.0,
            UTURN_CROSSOVER_DEFAULT_CRITICAL_HEADWAY_S,
            UTURN_CROSSOVER_DEFAULT_FOLLOWUP_HEADWAY_S,
            0.25,
        );
        assert!((r.capacity_veh_h - 483.0).abs() < 1.0, "c_p {}", r.capacity_veh_h);
        assert!((r.vc_ratio - 0.35).abs() < 0.01, "v/c {}", r.vc_ratio);
        assert!((r.control_delay_s - 16.3).abs() < 0.1, "delay {}", r.control_delay_s);
        assert!((r.queue_95_veh - 1.5).abs() < 0.1, "Q95 {}", r.queue_95_veh);
    }

    #[test]
    fn stop_junction_reproduces_example_13_main_junction() {
        // Exhibit 34-128 main-junction EB R: v = 344, v_c = 444,
        // t_c = 7.22, t_f = 3.36.
        let ebr = stop_junction_delay(344.0, 444.0, 7.22, 3.36, 0.25);
        assert!((ebr.capacity_veh_h - 537.0).abs() < 1.0, "c_p {}", ebr.capacity_veh_h);
        assert!((ebr.control_delay_s - 22.9).abs() < 0.1, "delay {}", ebr.control_delay_s);
        // NB L: v = 189, v_c = 1,044, t_c = 4.22, t_f = 2.26.
        let nbl = stop_junction_delay(189.0, 1044.0, 4.22, 2.26, 0.25);
        assert!((nbl.capacity_veh_h - 638.0).abs() < 1.0, "c_p {}", nbl.capacity_veh_h);
        assert!((nbl.control_delay_s - 13.0).abs() < 0.1, "delay {}", nbl.control_delay_s);
    }

    #[test]
    fn uturn_saturation_adjustment_exhibit_23_52() {
        assert_eq!(uturn_saturation_adjustment(30.0), 0.80);
        assert_eq!(uturn_saturation_adjustment(40.0), 0.85);
        assert_eq!(uturn_saturation_adjustment(90.0), 0.95);
    }

    #[test]
    fn dlt_offset_reproduces_example_16() {
        // Example Problem 16: TD_DLT = 350 ft, S_f,DLT = 35 mi/h,
        // LAG_DLT = 0, LAG_TH = 52, offsets 0, C = 65 s.
        let r = dlt_offset(350.0, 35.0, 0.0, 52.0, 0.0, 0.0, 65.0);
        assert!((r.tt_dlt_s - 6.8).abs() < 0.05, "TT_DLT {}", r.tt_dlt_s);
        assert_eq!(r.st_th_s, 52.0);
        // Published O_SUPP = 45 s (uses TT_DLT rounded to 7); computed 45.2.
        assert!((r.offset_supp_s - 45.2).abs() < 0.1, "O_SUPP {}", r.offset_supp_s);
    }

    #[test]
    fn dlt_weighted_average_reproduces_example_16() {
        // Exhibit 34-145 (flow, delay) cells.
        let cells = [
            DltDelayCell { flow_veh_h: 761.0, control_delay_s: 22.5 },
            DltDelayCell { flow_veh_h: 859.0, control_delay_s: 0.4 },
            DltDelayCell { flow_veh_h: 437.0, control_delay_s: 41.9 },
            DltDelayCell { flow_veh_h: 1352.0, control_delay_s: 2.5 },
            DltDelayCell { flow_veh_h: 422.0, control_delay_s: 42.5 },
            DltDelayCell { flow_veh_h: 486.0, control_delay_s: 25.7 },
            DltDelayCell { flow_veh_h: 1397.0, control_delay_s: 4.0 },
            DltDelayCell { flow_veh_h: 340.0, control_delay_s: 29.3 },
            DltDelayCell { flow_veh_h: 667.0, control_delay_s: 0.4 },
            DltDelayCell { flow_veh_h: 328.0, control_delay_s: 29.7 },
            DltDelayCell { flow_veh_h: 739.0, control_delay_s: 23.7 },
            DltDelayCell { flow_veh_h: 439.0, control_delay_s: 19.8 },
            DltDelayCell { flow_veh_h: 425.0, control_delay_s: 19.8 },
            DltDelayCell { flow_veh_h: 500.0, control_delay_s: 26.2 },
            DltDelayCell { flow_veh_h: 364.0, control_delay_s: 23.4 },
            DltDelayCell { flow_veh_h: 353.0, control_delay_s: 23.5 },
        ];
        let ett = dlt_weighted_average_delay(&cells, 5594.0);
        assert!((ett - 28.5).abs() < 0.1, "ETT_DLT {ett}");
    }
}
