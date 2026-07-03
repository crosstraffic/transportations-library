//! # Urban Street Segments (HCM Chapter 18), motorized vehicle methodology
//!
//! Implements the HCM 7th Edition Chapter 18, Section 3 computational steps
//! for one direction of travel on an urban street segment (EPUB source
//! `128_Ch18_03.xhtml`; concepts and Exhibit 18-1 from `127_Ch18_02.xhtml`):
//!
//! 1. Determine traffic demand adjustments — simplified: flow rates are
//!    supplied by the analyst; the capacity-constraint check flags (but does
//!    not meter) entry demand above capacity. The Chapter 30, Section 2
//!    origin–destination/volume-balance and spillback-check procedures are
//!    deferred (milestone 2).
//! 2. Determine running time — base free-flow speed (Equation 18-3 with
//!    Exhibit 18-11), signal-spacing adjustment (Equation 18-4), free-flow
//!    speed (Equation 18-5), vehicle proximity (Equation 18-6), delay due to
//!    turning vehicles (analyst-supplied per access point, or the Exhibit
//!    18-13 planning estimate), and segment running time (Equations 18-7 and
//!    18-8).
//! 3. Determine the proportion arriving during green — from an
//!    analyst-supplied platoon ratio or arrival type (HCM Equation 19-15,
//!    `P = R_p g/C`, with the Exhibit 19-13 arrival-type mapping). The
//!    Chapter 30, Section 3 platoon-dispersion arrival-profile procedure for
//!    coordinated systems is deferred (milestone 2); without an arrival
//!    input, arrivals are assumed uniform (`P = g/C`), per the Chapter 18
//!    text for noncoordinated upstream intersections.
//! 4. Determine signal phase duration — not implemented: phase times are
//!    inputs (`cycle_length_s`, `effective_green_s`). Use the Chapter 19
//!    engine (pretimed/coordinated timing) to obtain them; the actuated
//!    average-phase-duration loop is deferred with it.
//! 5. Determine through delay — the through control delay at the downstream
//!    boundary intersection is an input computed with the Chapter 19/21/22
//!    engines (Exhibit 18-5 lists it as "HCM method output"). Equation 18-10
//!    (shared-lane weighted through delay) is provided as
//!    [`shared_lane_through_delay`]. An uncontrolled through movement (e.g.,
//!    the major-street through movement at a TWSC boundary intersection) has
//!    0.0 s/veh through control delay per the Chapter 18 text; delay imposed
//!    on the major street by turning traffic at a TWSC boundary is not
//!    modeled here (it appears midsegment via the access-point delay terms).
//! 6. Determine through stop rate — Equation 18-11 (deterministic first term
//!    plus the overflow second term) with the Equations 18-12/18-13/18-14
//!    shared-lane weighting helpers; unsignalized boundary defaults per the
//!    Chapter 18 text (STOP 1.0 stops/veh, uncontrolled 0.0, YIELD = v/c).
//! 7. Determine travel speed — Equation 18-15.
//! 8. Determine spatial stop rate — Equation 18-16.
//! 9. Determine LOS — Exhibit 18-1 (travel speed thresholds by base
//!    free-flow speed, plus the v/c > 1.0 rule).
//! 10. Determine automobile traveler perception score — Equations 18-17
//!    through 18-22.
//!
//! ## Scope notes
//!
//! * Each `UrbanSegment` evaluates one direction of travel ("Each travel
//!   direction along the segment is separately evaluated", Chapter 18).
//! * Applicable to segments up to 2 mi long and not "short" (roughly
//!   > 700 ft when bounded by signals; see the Chapter 18 Spatial and
//!   Temporal Limits discussion).
//! * The Chapter 30, Section 4 access-point delay procedure (probability of
//!   inside-lane blockage, per-movement platoon interaction) is deferred;
//!   the analyst can supply its per-access-point results through
//!   `access_point_delays_s`, or use the Exhibit 18-13 planning estimate.

use serde::{Deserialize, Serialize};

use super::exhibits::{
    access_point_adjustment, access_point_density, cross_section_adjustment,
    exhibit_18_13_turn_delay_adjusted, exhibit_18_1_los, parking_adjustment, speed_constant_s0,
};
use crate::hcm::chapter19::exhibits::platoon_ratio_for_arrival_type;
use crate::hcm::common::LevelOfService;

// ═══════════════════════════════════════════════════════════════════════════════
// Boundary intersection control
// ═══════════════════════════════════════════════════════════════════════════════

/// Type of control regulating the segment through movement at the
/// downstream boundary intersection (HCM Chapter 18, Equation 18-8 and the
/// Step 6 stop-rate defaults).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BoundaryControlType {
    /// Traffic signal (Chapter 19 or 23 outputs supply delay/stop inputs).
    Signalized,
    /// All-way STOP control (Chapter 21 output supplies the through delay).
    AllWayStop,
    /// YIELD control other than a roundabout.
    YieldControlled,
    /// Roundabout (YIELD control; Chapter 22 output supplies the through
    /// delay; the Chapter 30, Section 9 geometric delay is deferred and can
    /// be folded into `through_control_delay_s` by the analyst).
    Roundabout,
    /// Through movement is uncontrolled at the boundary intersection —
    /// e.g., the major-street through movement at a two-way STOP-controlled
    /// intersection. Through control delay is 0.0 s/veh and the stop rate
    /// is 0.0 stops/veh per the Chapter 18 text. The through capacity can
    /// be estimated with Equation 18-2 ([`through_capacity_uncontrolled`]).
    Uncontrolled,
}

// ═══════════════════════════════════════════════════════════════════════════════
// Free helper functions (equations usable outside the step pipeline)
// ═══════════════════════════════════════════════════════════════════════════════

/// HCM Equation 18-1: default number of access point approaches on the
/// right side in the subject direction of travel,
/// `N_ap,s = 0.5 D_a L / 5,280`.
///
/// * `d_a` — access point density on the segment, points/mi (e.g., the
///   Exhibit 18-7 default)
/// * `segment_length_ft` — segment length L, ft
pub fn default_access_point_count(d_a: f64, segment_length_ft: f64) -> f64 {
    0.5 * d_a * segment_length_ft / 5_280.0
}

/// HCM Equation 18-2: capacity of an uncontrolled through movement at a
/// two-way STOP-controlled boundary intersection,
/// `c_th = 1,800 (N_th − 1 + p*_0,j)`.
///
/// * `n_th` — number of through lanes, shared or exclusive (ln)
/// * `p0_star` — probability that there will be no queue in the inside
///   through lane. Equal to 1.0 if a left-turn bay is provided for left
///   turns from the major street; otherwise computed with
///   `Twsc::prob_queue_free_shared_major` (HCM Equations 20-29 through
///   20-34 — the Chapter 18 text cites "Equation 20-43", which is the
///   HCM 6th Edition number for this equation; in the 7th Edition EPUB,
///   Equation 20-43 is the Rank 4 movement capacity).
pub fn through_capacity_uncontrolled(n_th: u32, p0_star: f64) -> f64 {
    1_800.0 * ((n_th as f64) - 1.0 + p0_star)
}

/// HCM Equation 18-4: signal spacing adjustment factor,
/// `f_L = 1.02 − 4.7 (S_fo − 19.5) / max(L_s, 400) ≤ 1.0`.
///
/// * `base_ffs_mph` — base free-flow speed S_fo, mi/h
/// * `signal_spacing_ft` — distance L_s between the two intersections that
///   bracket the subject segment and can legally require the through
///   movement to stop or yield, ft
pub fn signal_spacing_adjustment(base_ffs_mph: f64, signal_spacing_ft: f64) -> f64 {
    (1.02 - 4.7 * (base_ffs_mph - 19.5) / signal_spacing_ft.max(400.0)).min(1.0)
}

/// HCM Equation 18-6: proximity adjustment factor,
/// `f_v = 2 / (1 + (1 − v_m / (52.8 N_th S_f))^0.21)`.
///
/// * `v_m` — midsegment demand flow rate, veh/h
/// * `n_th` — number of through lanes in the subject direction (ln)
/// * `s_f` — free-flow speed, mi/h
pub fn proximity_adjustment(v_m: f64, n_th: u32, s_f: f64) -> f64 {
    let denom = 52.8 * (n_th as f64) * s_f;
    if denom <= 0.0 {
        return 1.0;
    }
    // The speed–flow relationship (Exhibit 18-12) is defined for demand
    // below one-lane saturation; clamp the ratio below 1.0 to keep the
    // exponentiation real for out-of-range inputs.
    let ratio = (v_m / denom).clamp(0.0, 0.999_999);
    2.0 / (1.0 + (1.0 - ratio).powf(0.21))
}

/// HCM Equation 18-10: weighted through delay when the through movement
/// shares one or more lanes at a signalized boundary intersection,
/// `d_t = (d_th v_t N_t + d_sl v_sl (1 − P_L) + d_sr v_sr (1 − P_R)) / v_th`.
///
/// * `v_th` — through-demand flow rate, veh/h
/// * `exclusive` — `(d_th, v_t, N_t)`: delay (s/veh), demand flow rate per
///   lane (veh/h/ln), and lane count of the exclusive-through lane group
/// * `shared_left` — `(d_sl, v_sl, P_L)`: delay (s/veh), demand flow rate
///   (veh/h), and proportion of left-turning vehicles in the shared
///   left-turn/through lane group
/// * `shared_right` — `(d_sr, v_sr, P_R)`: same for the shared
///   right-turn/through lane group
pub fn shared_lane_through_delay(
    v_th: f64,
    exclusive: Option<(f64, f64, u32)>,
    shared_left: Option<(f64, f64, f64)>,
    shared_right: Option<(f64, f64, f64)>,
) -> f64 {
    if v_th <= 0.0 {
        return 0.0;
    }
    let mut num = 0.0;
    if let Some((d_th, v_t, n_t)) = exclusive {
        num += d_th * v_t * n_t as f64;
    }
    if let Some((d_sl, v_sl, p_l)) = shared_left {
        num += d_sl * v_sl * (1.0 - p_l);
    }
    if let Some((d_sr, v_sr, p_r)) = shared_right {
        num += d_sr * v_sr * (1.0 - p_r);
    }
    num / v_th
}

/// HCM Equations 18-12, 18-13, and 18-14: weighted per-lane quantity across
/// the lane groups serving the through movement,
/// `x = (x_t N_t + x_sl (1 − P_L) + x_sr (1 − P_R)) / N_th`.
///
/// Applies to the number of fully stopped vehicles N_f (Equation 18-12),
/// the adjusted saturation flow rate s (Equation 18-13), and the
/// back-of-queue size Q_2+3 (Equation 18-14).
///
/// * `n_th` — number of through lanes, shared or exclusive (ln)
/// * `exclusive` — `(x_t, N_t)`: per-lane value and lane count of the
///   exclusive-through lane group
/// * `shared_left` — `(x_sl, P_L)`: per-lane value of the shared
///   left/through lane group and proportion of left-turning vehicles in
///   the shared lane
/// * `shared_right` — `(x_sr, P_R)`: same for the shared right/through
///   lane group
pub fn weighted_through_lane_value(
    n_th: u32,
    exclusive: Option<(f64, u32)>,
    shared_left: Option<(f64, f64)>,
    shared_right: Option<(f64, f64)>,
) -> f64 {
    let n = (n_th.max(1)) as f64;
    let mut num = 0.0;
    if let Some((x_t, n_t)) = exclusive {
        num += x_t * n_t as f64;
    }
    if let Some((x_sl, p_l)) = shared_left {
        num += x_sl * (1.0 - p_l);
    }
    if let Some((x_sr, p_r)) = shared_right {
        num += x_sr * (1.0 - p_r);
    }
    num / n
}

/// HCM Equation 18-11: full stop rate at a signalized boundary
/// intersection,
/// `h = 3,600 [ N_f / (min(1, v_th C / (N_th s g)) g s) + N_th Q_2+3 / (v_th C) ]`.
///
/// * `n_f` — number of fully stopped vehicles N_f, veh/ln (Chapter 31,
///   Section 4 output; Equation 18-12 weighting for multiple lane groups)
/// * `q_2_plus_3` — second- plus third-term back-of-queue size Q_2+3,
///   veh/ln (Chapter 31 output; Equation 18-14 weighting)
/// * `g_s` — effective green time g, s
/// * `sat_flow_veh_h_ln` — adjusted saturation flow rate s, veh/h/ln
///   (Equation 18-13 weighting)
/// * `v_th` — through-demand flow rate, veh/h
/// * `cycle_s` — cycle length C, s
/// * `n_th` — number of through lanes, shared or exclusive (ln)
pub fn full_stop_rate_signalized(
    n_f: f64,
    q_2_plus_3: f64,
    g_s: f64,
    sat_flow_veh_h_ln: f64,
    v_th: f64,
    cycle_s: f64,
    n_th: u32,
) -> f64 {
    let s = sat_flow_veh_h_ln;
    let n = n_th.max(1) as f64;
    if g_s <= 0.0 || s <= 0.0 || v_th <= 0.0 || cycle_s <= 0.0 {
        return 0.0;
    }
    let flow_ratio = (v_th * cycle_s / (n * s * g_s)).min(1.0);
    if flow_ratio <= 0.0 {
        return 0.0;
    }
    3_600.0 * (n_f / (flow_ratio * g_s * s) + n * q_2_plus_3 / (v_th * cycle_s))
}

/// HCM Equations 18-17 through 18-22: automobile traveler perception score,
/// `I_a,seg = 1 + P_BCDEF + P_CDEF + P_DEF + P_EF + P_F` with
/// `P_x = (1 + e^(a_x − 0.253 H_seg + 0.3434 P_LTL,seg))^−1` and intercepts
/// a = −1.1614, 0.6234, 1.7389, 2.7047, and 3.8044.
///
/// * `spatial_stop_rate` — H_seg, stops/mi (Equation 18-16)
/// * `p_ltl_seg` — proportion of intersections with a left-turn lane (or
///   bay) on the segment (decimal)
pub fn traveler_perception_score(spatial_stop_rate: f64, p_ltl_seg: f64) -> f64 {
    let shift = -0.253 * spatial_stop_rate + 0.3434 * p_ltl_seg;
    let p = |intercept: f64| 1.0 / (1.0 + (intercept + shift).exp());
    1.0 + p(-1.1614) + p(0.6234) + p(1.7389) + p(2.7047) + p(3.8044)
}

// ═══════════════════════════════════════════════════════════════════════════════
// Urban street segment (one direction of travel)
// ═══════════════════════════════════════════════════════════════════════════════

fn default_one() -> f64 {
    1.0
}
fn default_ten() -> f64 {
    10.0
}

/// One direction of travel on an urban street segment (HCM Chapter 18,
/// motorized vehicle methodology).
///
/// Populate the input fields (directly or via [`UrbanSegment::from_json`]),
/// call [`UrbanSegment::analyze`], then read the computed `Option` fields.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UrbanSegment {
    // ───────────────────────── Geometry ─────────────────────────
    /// Segment length L, ft (stop line to stop line).
    pub segment_length_ft: f64,
    /// Width of the upstream boundary intersection W_i, ft. The link
    /// length used by the methodology is `L − W_i`.
    #[serde(default)]
    pub upstream_intersection_width_ft: f64,
    /// Number of through lanes N_th on the segment in the subject
    /// direction of travel (ln).
    pub n_through_lanes: u32,
    /// Posted speed limit S_pl, mi/h.
    pub speed_limit_mph: f64,
    /// Length of the segment with a restrictive median (e.g., raised
    /// curb), ft, measured median nose to median nose. Divided by the link
    /// length to obtain the proportion p_rm of Exhibit 18-11, note b.
    #[serde(default)]
    pub restrictive_median_length_ft: f64,
    /// Proportion of the link length with curb on the right-hand side
    /// within 4 ft of the traveled way, p_curb (decimal). Exhibit 18-5
    /// default: 1.0 (curb on both sides).
    #[serde(default = "default_one")]
    pub proportion_with_curb: f64,
    /// Proportion of the link length with on-street parking on the
    /// right-hand side (decimal).
    #[serde(default)]
    pub proportion_on_street_parking: f64,
    /// Number of access point approaches on the right side in the subject
    /// direction of travel, N_ap,s (points). All unsignalized driveway and
    /// public-street approaches count, active or not.
    #[serde(default)]
    pub n_access_points_subject: f64,
    /// Number of access point approaches on the right side in the opposing
    /// direction of travel, N_ap,o (points).
    #[serde(default)]
    pub n_access_points_opposing: f64,
    /// Proportion p_ap,lt of N_ap,o that can be accessed by a left turn
    /// from the subject direction of travel (decimal). Default 1.0
    /// (undivided cross section); use 0.0 for a full restrictive median
    /// with no openings.
    #[serde(default = "default_one")]
    pub prop_opposing_left_accessible: f64,
    /// Distance L_s between the two intersections that bracket the subject
    /// segment and can legally require the through movement to stop or
    /// yield, ft (Equation 18-4). Defaults to the segment length.
    #[serde(default)]
    pub signal_spacing_ft: Option<f64>,

    // ───────────────────────── Speed inputs ─────────────────────────
    /// Base free-flow speed calibration factor S_calib, mi/h (Equation
    /// 18-3; default 0.0).
    #[serde(default)]
    pub s_calib_mph: f64,
    /// Field-measured free-flow speed S_f, mi/h. When present it replaces
    /// the Equation 18-3/18-5 prediction ("Alternatively, it can be
    /// entered directly by the analyst").
    #[serde(default)]
    pub free_flow_speed_override_mph: Option<f64>,

    // ───────────────────────── Demand ─────────────────────────
    /// Through-demand flow rate v_th at the downstream boundary
    /// intersection, veh/h.
    pub through_demand_veh_h: f64,
    /// Midsegment demand flow rate v_m, veh/h (all movements traveling
    /// along the segment). Exhibit 18-5 default: the demand flow rate at
    /// the downstream boundary intersection approach — when absent, the
    /// total entering/approach volume should be supplied via
    /// `through_demand_veh_h`; here the default is `through_demand_veh_h`.
    #[serde(default)]
    pub midsegment_flow_veh_h: Option<f64>,
    /// Through-movement capacity c_th at the downstream boundary
    /// intersection, veh/h ("HCM method output" per Exhibit 18-5: Chapter
    /// 19/21/22 engines, or Equation 18-2 via
    /// [`through_capacity_uncontrolled`] for a TWSC major-street through
    /// movement).
    #[serde(default)]
    pub through_capacity_veh_h: Option<f64>,

    // ─────────────── Downstream boundary intersection ───────────────
    /// Control type regulating the through movement at the downstream
    /// boundary intersection.
    pub control: BoundaryControlType,
    /// Through control delay d_t at the downstream boundary intersection,
    /// s/veh ("HCM method output" per Exhibit 18-5; use
    /// [`shared_lane_through_delay`] for shared-lane weighting). Ignored
    /// (taken as 0.0) for an uncontrolled through movement.
    #[serde(default)]
    pub through_control_delay_s: Option<f64>,
    /// Cycle length C at the downstream signal, s (Steps 3 and 6).
    #[serde(default)]
    pub cycle_length_s: Option<f64>,
    /// Effective green time g for the phase serving the through movement
    /// at the downstream signal, s (Steps 3 and 6).
    #[serde(default)]
    pub effective_green_s: Option<f64>,
    /// Arrival type (1–6) describing arrivals at the downstream signal
    /// (mapped to a platoon ratio with Exhibit 19-13). Ignored when
    /// `platoon_ratio` is set.
    #[serde(default)]
    pub arrival_type: Option<u8>,
    /// Platoon ratio R_p describing arrivals at the downstream signal.
    /// When neither this nor `arrival_type` is given, arrivals are assumed
    /// uniform (`P = g/C`), per the Chapter 18 Step 3 text for
    /// noncoordinated upstream intersections.
    #[serde(default)]
    pub platoon_ratio: Option<f64>,

    // ─────────────── Stop-rate inputs (signalized boundary) ───────────────
    /// Number of fully stopped vehicles N_f, veh/ln (Chapter 31, Section 4
    /// output; Equation 18-12 weighting when the through movement is
    /// served in several lane groups).
    #[serde(default)]
    pub stopped_vehicles_veh_ln: Option<f64>,
    /// Second-term back-of-queue size Q_2, veh/ln (Chapter 31 output).
    #[serde(default)]
    pub queue2_veh_ln: Option<f64>,
    /// Third-term back-of-queue size Q_3, veh/ln (Chapter 31 output).
    #[serde(default)]
    pub queue3_veh_ln: Option<f64>,
    /// Adjusted saturation flow rate s of the through lane group,
    /// veh/h/ln (Equation 18-13 weighting for multiple lane groups).
    #[serde(default)]
    pub sat_flow_veh_h_ln: Option<f64>,
    /// Full stop rate h, stops/veh, supplied directly (e.g., from an HCM
    /// computational engine) in lieu of the Equation 18-11 inputs.
    #[serde(default)]
    pub full_stop_rate_override: Option<f64>,
    /// Full stop rate due to other (midsegment) sources h_other,
    /// stops/veh (Equation 18-16).
    #[serde(default)]
    pub stop_rate_other: f64,

    // ───────────────────── Access point delay ─────────────────────
    /// Delay to through vehicles due to left and right turns at each
    /// influential access point intersection, d_ap,i, s/veh (Chapter 30,
    /// Section 4 procedure output or field data). When absent, the
    /// Exhibit 18-13 planning estimate is used.
    #[serde(default)]
    pub access_point_delays_s: Option<Vec<f64>>,
    /// Number of influential access point approaches N_ap used with the
    /// Exhibit 18-13 estimate. Default:
    /// `N_ap = N_ap,s + p_ap,lt N_ap,o` (Equation 18-7 definition). Set it
    /// to the number of *active* access points when the inactive ones are
    /// known to contribute negligible turning delay.
    #[serde(default)]
    pub n_influential_access_points: Option<f64>,
    /// Percentage of the midsegment volume turning left at a
    /// representative access point (%; Exhibit 18-13 assumes 10%).
    #[serde(default = "default_ten")]
    pub pct_left_turns_access: f64,
    /// Percentage of the midsegment volume turning right at a
    /// representative access point (%; Exhibit 18-13 assumes 10%).
    #[serde(default = "default_ten")]
    pub pct_right_turns_access: f64,
    /// True if left turns at the access points are served by a turn bay of
    /// adequate length (Exhibit 18-13 adjustment).
    #[serde(default)]
    pub access_left_bay_adequate: bool,
    /// True if right turns at the access points are served by a turn bay
    /// of adequate length (Exhibit 18-13 adjustment).
    #[serde(default)]
    pub access_right_bay_adequate: bool,
    /// Delay due to other midsegment sources d_other, s/veh (e.g., curb
    /// parking maneuvers or midsegment crosswalks; Equation 18-7).
    #[serde(default)]
    pub midsegment_other_delay_s: f64,

    // ───────────────────── Perception score ─────────────────────
    /// Proportion of intersections with a left-turn lane (or bay) on the
    /// segment, P_LTL,seg (decimal; Equations 18-18 through 18-22).
    #[serde(default)]
    pub prop_left_turn_lanes: Option<f64>,

    // ───────────────────── Computed results ─────────────────────
    /// Speed constant S_0, mi/h (Exhibit 18-11, note a).
    #[serde(default)]
    pub speed_constant_mph: Option<f64>,
    /// Cross-section adjustment f_CS, mi/h (Exhibit 18-11, note b).
    #[serde(default)]
    pub f_cs_mph: Option<f64>,
    /// Access point adjustment f_A, mi/h (Exhibit 18-11, note c).
    #[serde(default)]
    pub f_a_mph: Option<f64>,
    /// On-street parking adjustment f_pk, mi/h (Exhibit 18-11, note d).
    #[serde(default)]
    pub f_pk_mph: Option<f64>,
    /// Base free-flow speed S_fo, mi/h (Equation 18-3).
    #[serde(default)]
    pub base_ffs_mph: Option<f64>,
    /// Signal spacing adjustment factor f_L (Equation 18-4).
    #[serde(default)]
    pub f_l: Option<f64>,
    /// Free-flow speed S_f, mi/h (Equation 18-5).
    #[serde(default)]
    pub free_flow_speed_mph: Option<f64>,
    /// Proximity adjustment factor f_v (Equation 18-6).
    #[serde(default)]
    pub f_v: Option<f64>,
    /// Total delay due to turning vehicles at access points Σ d_ap,i,
    /// s/veh (Equation 18-7 term).
    #[serde(default)]
    pub access_point_delay_total_s: Option<f64>,
    /// Segment running time t_R, s (Equation 18-7).
    #[serde(default)]
    pub running_time_s: Option<f64>,
    /// Segment running speed `= 3,600 L / (5,280 t_R)`, mi/h (Chapter 18,
    /// Step 2 discussion of Exhibit 18-12).
    #[serde(default)]
    pub running_speed_mph: Option<f64>,
    /// Proportion of vehicles arriving during green P (Step 3; Equation
    /// 19-15 with the supplied platoon ratio or arrival type).
    #[serde(default)]
    pub proportion_arriving_green: Option<f64>,
    /// Through delay d_t, s/veh (Step 5).
    #[serde(default)]
    pub through_delay_s: Option<f64>,
    /// Full stop rate h, stops/veh (Step 6).
    #[serde(default)]
    pub full_stop_rate: Option<f64>,
    /// Travel speed S_T,seg, mi/h (Equation 18-15).
    #[serde(default)]
    pub travel_speed_mph: Option<f64>,
    /// Spatial stop rate H_seg, stops/mi (Equation 18-16).
    #[serde(default)]
    pub spatial_stop_rate_stops_mi: Option<f64>,
    /// Volume-to-capacity ratio of the through movement at the downstream
    /// boundary intersection (Step 9).
    #[serde(default)]
    pub vc_ratio: Option<f64>,
    /// True when the entry demand exceeds the through-movement capacity
    /// (Step 1 capacity-constraint check; metering itself is deferred).
    #[serde(default)]
    pub demand_exceeds_capacity: Option<bool>,
    /// Segment LOS (Exhibit 18-1).
    #[serde(default)]
    pub los: Option<LevelOfService>,
    /// Automobile traveler perception score I_a,seg (Equation 18-17).
    #[serde(default)]
    pub perception_score: Option<f64>,
}

impl UrbanSegment {
    /// Create a segment with the required inputs; optional inputs take the
    /// Exhibit 18-5 defaults encoded in the field docs.
    pub fn new(
        segment_length_ft: f64,
        n_through_lanes: u32,
        speed_limit_mph: f64,
        through_demand_veh_h: f64,
        control: BoundaryControlType,
    ) -> Self {
        UrbanSegment {
            segment_length_ft,
            upstream_intersection_width_ft: 0.0,
            n_through_lanes,
            speed_limit_mph,
            restrictive_median_length_ft: 0.0,
            proportion_with_curb: 1.0,
            proportion_on_street_parking: 0.0,
            n_access_points_subject: 0.0,
            n_access_points_opposing: 0.0,
            prop_opposing_left_accessible: 1.0,
            signal_spacing_ft: None,
            s_calib_mph: 0.0,
            free_flow_speed_override_mph: None,
            through_demand_veh_h,
            midsegment_flow_veh_h: None,
            through_capacity_veh_h: None,
            control,
            through_control_delay_s: None,
            cycle_length_s: None,
            effective_green_s: None,
            arrival_type: None,
            platoon_ratio: None,
            stopped_vehicles_veh_ln: None,
            queue2_veh_ln: None,
            queue3_veh_ln: None,
            sat_flow_veh_h_ln: None,
            full_stop_rate_override: None,
            stop_rate_other: 0.0,
            access_point_delays_s: None,
            n_influential_access_points: None,
            pct_left_turns_access: 10.0,
            pct_right_turns_access: 10.0,
            access_left_bay_adequate: false,
            access_right_bay_adequate: false,
            midsegment_other_delay_s: 0.0,
            prop_left_turn_lanes: None,
            speed_constant_mph: None,
            f_cs_mph: None,
            f_a_mph: None,
            f_pk_mph: None,
            base_ffs_mph: None,
            f_l: None,
            free_flow_speed_mph: None,
            f_v: None,
            access_point_delay_total_s: None,
            running_time_s: None,
            running_speed_mph: None,
            proportion_arriving_green: None,
            through_delay_s: None,
            full_stop_rate: None,
            travel_speed_mph: None,
            spatial_stop_rate_stops_mi: None,
            vc_ratio: None,
            demand_exceeds_capacity: None,
            los: None,
            perception_score: None,
        }
    }

    /// Deserialize a segment from the `tests/ExampleCases/hcm/UrbanSegments`
    /// fixture JSON format (field names match the struct fields).
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Serialize the full analysis (inputs and results) to JSON.
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    // ─────────────────────── Derived geometry ───────────────────────

    /// Link length `L − W_i`, ft (Chapter 18, Segment Length subsection).
    pub fn link_length_ft(&self) -> f64 {
        (self.segment_length_ft - self.upstream_intersection_width_ft).max(0.0)
    }

    /// Proportion of the link length with a restrictive median, p_rm
    /// (decimal), used by Exhibit 18-11, note b.
    pub fn proportion_restrictive_median(&self) -> f64 {
        let link = self.link_length_ft();
        if link <= 0.0 {
            return 0.0;
        }
        (self.restrictive_median_length_ft / link).clamp(0.0, 1.0)
    }

    /// Number of influential access point approaches,
    /// `N_ap = N_ap,s + p_ap,lt N_ap,o` (Equation 18-7 definition).
    pub fn n_influential_access_points_computed(&self) -> f64 {
        self.n_access_points_subject
            + self.prop_opposing_left_accessible * self.n_access_points_opposing
    }

    /// Midsegment demand flow rate v_m, veh/h — the input value, or the
    /// Exhibit 18-5 default (the downstream boundary approach demand,
    /// proxied by the through demand).
    pub fn midsegment_flow_rate(&self) -> f64 {
        self.midsegment_flow_veh_h.unwrap_or(self.through_demand_veh_h)
    }

    // ─────────────────────── Accessors ───────────────────────

    pub fn get_segment_length_ft(&self) -> f64 {
        self.segment_length_ft
    }
    pub fn set_segment_length_ft(&mut self, l: f64) {
        self.segment_length_ft = l;
    }
    pub fn get_speed_limit_mph(&self) -> f64 {
        self.speed_limit_mph
    }
    pub fn set_speed_limit_mph(&mut self, s: f64) {
        self.speed_limit_mph = s;
    }
    pub fn get_through_demand_veh_h(&self) -> f64 {
        self.through_demand_veh_h
    }
    pub fn set_through_demand_veh_h(&mut self, v: f64) {
        self.through_demand_veh_h = v;
    }
    pub fn get_base_ffs_mph(&self) -> Option<f64> {
        self.base_ffs_mph
    }
    pub fn get_free_flow_speed_mph(&self) -> Option<f64> {
        self.free_flow_speed_mph
    }
    pub fn get_running_time_s(&self) -> Option<f64> {
        self.running_time_s
    }
    pub fn get_travel_speed_mph(&self) -> Option<f64> {
        self.travel_speed_mph
    }
    pub fn get_spatial_stop_rate(&self) -> Option<f64> {
        self.spatial_stop_rate_stops_mi
    }
    pub fn get_vc_ratio(&self) -> Option<f64> {
        self.vc_ratio
    }
    pub fn get_los(&self) -> Option<LevelOfService> {
        self.los
    }
    pub fn get_perception_score(&self) -> Option<f64> {
        self.perception_score
    }

    // ─────────────────────── Computational steps ───────────────────────

    /// Step 1: Determine traffic demand adjustments (simplified).
    ///
    /// Records the capacity-constraint check (`demand_exceeds_capacity`)
    /// and returns the midsegment flow rate v_m (input value or Exhibit
    /// 18-5 default). The Chapter 30, Section 2 volume-balance,
    /// origin–destination, and spillback-check procedures are deferred;
    /// analysts should supply already-balanced demand flow rates.
    pub fn step_1_demand_adjustment(&mut self) -> f64 {
        self.demand_exceeds_capacity = self
            .through_capacity_veh_h
            .map(|c| c > 0.0 && self.through_demand_veh_h > c);
        self.midsegment_flow_rate()
    }

    /// Step 2: Determine running time (Equations 18-3 through 18-8 and
    /// Exhibits 18-11 and 18-13). Computes and stores the base free-flow
    /// speed, free-flow speed, proximity factor, access point delay, and
    /// segment running time; returns t_R, s.
    pub fn step_2_running_time(&mut self) -> f64 {
        let n_th = self.n_through_lanes.max(1);
        // A. Free-flow speed.
        let s_0 = speed_constant_s0(self.speed_limit_mph);
        let f_cs = cross_section_adjustment(
            self.proportion_restrictive_median(),
            self.proportion_with_curb,
        );
        let d_a = access_point_density(
            self.n_access_points_subject,
            self.n_access_points_opposing,
            self.segment_length_ft,
            self.upstream_intersection_width_ft,
        );
        let f_a = access_point_adjustment(d_a, n_th);
        let f_pk = parking_adjustment(self.proportion_on_street_parking);
        // Equation 18-3.
        let s_fo = self.s_calib_mph + s_0 + f_cs + f_a + f_pk;
        // Equation 18-4.
        let l_s = self.signal_spacing_ft.unwrap_or(self.segment_length_ft);
        let f_l = signal_spacing_adjustment(s_fo, l_s);
        // Equation 18-5 (never below the speed limit); the analyst can
        // supply a field-measured value instead.
        let s_f = match self.free_flow_speed_override_mph {
            Some(v) => v,
            None => (s_fo * f_l).max(self.speed_limit_mph),
        };
        self.speed_constant_mph = Some(s_0);
        self.f_cs_mph = Some(f_cs);
        self.f_a_mph = Some(f_a);
        self.f_pk_mph = Some(f_pk);
        self.base_ffs_mph = Some(s_fo);
        self.f_l = Some(f_l);
        self.free_flow_speed_mph = Some(s_f);

        // B. Vehicle proximity (Equation 18-6).
        let v_m = self.midsegment_flow_rate();
        let f_v = proximity_adjustment(v_m, n_th, s_f);
        self.f_v = Some(f_v);

        // C. Delay due to turning vehicles at access points.
        let d_ap_total = match &self.access_point_delays_s {
            Some(delays) => delays.iter().sum(),
            None => {
                let n_ap = self
                    .n_influential_access_points
                    .unwrap_or_else(|| self.n_influential_access_points_computed());
                let per_point = exhibit_18_13_turn_delay_adjusted(
                    v_m / n_th as f64,
                    n_th,
                    self.pct_left_turns_access,
                    self.pct_right_turns_access,
                    self.access_left_bay_adequate,
                    self.access_right_bay_adequate,
                );
                per_point * n_ap
            }
        };
        self.access_point_delay_total_s = Some(d_ap_total);

        // E. Segment running time (Equations 18-7 and 18-8).
        // Start-up lost time l_1: 2.0 s if signalized, 2.5 s if STOP or
        // YIELD controlled (Equation 18-7 definitions).
        let (l_1, f_x) = match self.control {
            BoundaryControlType::Signalized => (2.0, 1.0),
            BoundaryControlType::AllWayStop => (2.5, 1.0),
            BoundaryControlType::YieldControlled | BoundaryControlType::Roundabout => {
                let vc = match self.through_capacity_veh_h {
                    Some(c) if c > 0.0 => (self.through_demand_veh_h / c).min(1.0),
                    _ => 1.0, // conservative when capacity is unknown
                };
                (2.5, vc)
            }
            BoundaryControlType::Uncontrolled => (2.5, 0.0),
        };
        let l = self.segment_length_ft;
        let t_r = (6.0 - l_1) / (0.0025 * l) * f_x
            + (3_600.0 * l) / (5_280.0 * s_f) * f_v
            + d_ap_total
            + self.midsegment_other_delay_s;
        self.running_time_s = Some(t_r);
        self.running_speed_mph = Some(3_600.0 * l / (5_280.0 * t_r));
        t_r
    }

    /// Step 3: Determine the proportion arriving during green (signalized
    /// downstream boundary only). Uses the analyst-supplied platoon ratio
    /// (or Exhibit 19-13 arrival type) with `P = R_p g/C` (Equation 19-15);
    /// with no arrival input, arrivals are assumed uniform (`P = g/C`).
    /// The Chapter 30 platoon-dispersion computation for coordinated
    /// systems is deferred. Returns None if the boundary is not signalized
    /// or the signal timing inputs are absent.
    pub fn step_3_proportion_arriving_green(&mut self) -> Option<f64> {
        if self.control != BoundaryControlType::Signalized {
            self.proportion_arriving_green = None;
            return None;
        }
        let (g, c) = (self.effective_green_s?, self.cycle_length_s?);
        if c <= 0.0 {
            return None;
        }
        let r_p = self
            .platoon_ratio
            .or_else(|| self.arrival_type.and_then(platoon_ratio_for_arrival_type))
            .unwrap_or(1.0);
        let p = (r_p * g / c).min(1.0);
        self.proportion_arriving_green = Some(p);
        Some(p)
    }

    /// Step 5: Determine through delay, s/veh. The control delay at a
    /// controlled boundary intersection is an input from the appropriate
    /// chapter's methodology; an uncontrolled through movement has
    /// 0.0 s/veh (Chapter 18 text). Geometric delay is negligible for
    /// noncircular intersections; for a roundabout, include the Chapter
    /// 30, Section 9 geometric delay in the input value.
    pub fn step_5_through_delay(&mut self) -> f64 {
        let d_t = match self.control {
            BoundaryControlType::Uncontrolled => 0.0,
            _ => self.through_control_delay_s.unwrap_or(0.0),
        };
        self.through_delay_s = Some(d_t);
        d_t
    }

    /// Step 6: Determine through stop rate h, stops/veh.
    ///
    /// * Signalized — Equation 18-11 from the Chapter 31 inputs (N_f,
    ///   Q_2, Q_3, g, s, C), or `full_stop_rate_override` when supplied.
    /// * STOP-controlled approach — 1.0 stops/veh.
    /// * Uncontrolled approach — 0.0 stops/veh.
    /// * YIELD-controlled (incl. roundabout) — the through-movement
    ///   volume-to-capacity ratio.
    ///
    /// Returns None for a signalized boundary without stop-rate inputs.
    pub fn step_6_stop_rate(&mut self) -> Option<f64> {
        if let Some(h) = self.full_stop_rate_override {
            self.full_stop_rate = Some(h);
            return Some(h);
        }
        let h = match self.control {
            BoundaryControlType::Signalized => {
                let n_f = self.stopped_vehicles_veh_ln?;
                let q23 = self.queue2_veh_ln.unwrap_or(0.0) + self.queue3_veh_ln.unwrap_or(0.0);
                Some(full_stop_rate_signalized(
                    n_f,
                    q23,
                    self.effective_green_s?,
                    self.sat_flow_veh_h_ln?,
                    self.through_demand_veh_h,
                    self.cycle_length_s?,
                    self.n_through_lanes,
                ))
            }
            BoundaryControlType::AllWayStop => Some(1.0),
            BoundaryControlType::Uncontrolled => Some(0.0),
            BoundaryControlType::YieldControlled | BoundaryControlType::Roundabout => self
                .through_capacity_veh_h
                .filter(|c| *c > 0.0)
                .map(|c| self.through_demand_veh_h / c),
        };
        self.full_stop_rate = h;
        h
    }

    /// Step 7: Determine travel speed (Equation 18-15),
    /// `S_T,seg = 3,600 L / (5,280 (t_R + d_t))`, mi/h. Requires Steps 2
    /// and 5.
    pub fn step_7_travel_speed(&mut self) -> Option<f64> {
        let t_r = self.running_time_s?;
        let d_t = self.through_delay_s?;
        let s = 3_600.0 * self.segment_length_ft / (5_280.0 * (t_r + d_t));
        self.travel_speed_mph = Some(s);
        Some(s)
    }

    /// Step 8: Determine spatial stop rate (Equation 18-16),
    /// `H_seg = 5,280 (h + h_other) / L`, stops/mi. Requires Step 6.
    pub fn step_8_spatial_stop_rate(&mut self) -> Option<f64> {
        let h = self.full_stop_rate?;
        let h_seg = 5_280.0 * (h + self.stop_rate_other) / self.segment_length_ft;
        self.spatial_stop_rate_stops_mi = Some(h_seg);
        Some(h_seg)
    }

    /// Step 9: Determine LOS (Exhibit 18-1) from the travel speed, the
    /// base free-flow speed, and the through-movement volume-to-capacity
    /// ratio at the downstream boundary intersection. Requires Steps 2
    /// and 7; without a capacity input the v/c > 1.0 rule cannot be
    /// evaluated and v/c is treated as ≤ 1.0.
    pub fn step_9_los(&mut self) -> Option<LevelOfService> {
        let speed = self.travel_speed_mph?;
        let base = self.base_ffs_mph?;
        let vc = self.through_capacity_veh_h.and_then(|c| {
            (c > 0.0).then(|| self.through_demand_veh_h / c)
        });
        self.vc_ratio = vc;
        let los = exhibit_18_1_los(speed, base, vc.is_some_and(|x| x > 1.0));
        self.los = Some(los);
        Some(los)
    }

    /// Step 10: Determine the automobile traveler perception score
    /// (Equations 18-17 through 18-22). Requires Step 8 and the
    /// `prop_left_turn_lanes` input.
    pub fn step_10_perception_score(&mut self) -> Option<f64> {
        let h_seg = self.spatial_stop_rate_stops_mi?;
        let p_ltl = self.prop_left_turn_lanes?;
        let score = traveler_perception_score(h_seg, p_ltl);
        self.perception_score = Some(score);
        Some(score)
    }

    /// Run the full Chapter 18 motorized vehicle pipeline (Steps 1–3 and
    /// 5–10; Step 4 signal phase duration is an input, see the module
    /// docs).
    pub fn analyze(&mut self) {
        self.step_1_demand_adjustment();
        self.step_2_running_time();
        self.step_3_proportion_arriving_green();
        self.step_5_through_delay();
        self.step_6_stop_rate();
        self.step_7_travel_speed();
        self.step_8_spatial_stop_rate();
        self.step_9_los();
        self.step_10_perception_score();
    }
}
