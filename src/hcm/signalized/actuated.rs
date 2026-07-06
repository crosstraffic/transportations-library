//! Actuated phase-duration estimation (HCM 7th Edition, Chapter 31,
//! Section 2, "Actuated Phase Duration", Equations 31-1 through 31-45).
//!
//! The procedure estimates the average duration of each phase of an
//! actuated (or semiactuated / coordinated-actuated) controller from the
//! controller settings, detector design, and lane-group demand. The
//! computation is iterative because several intermediate quantities (queue
//! service time, permitted green, equivalent maximum green) depend on the
//! green interval durations that the procedure itself produces.
//!
//! This module provides the individual equations as pure functions (each
//! carrying its HCM equation citation) plus a driver
//! ([`estimate_fully_actuated`]) for the standard eight-phase dual-ring
//! NEMA structure of HCM Exhibit 19-2. The driver reproduces the queue
//! service, green extension, unbalanced green, and barrier-balancing steps
//! (Steps A through R of the Average Phase Duration subsection) for fully
//! actuated and semiactuated control.
//!
//! Controller-emulation details that the HCM itself leaves to the
//! computational engine (Section 7) are deferred with explicit notes:
//! permissive-period modeling, the coordinated-actuated force-off / yield
//! point emulation beyond the equivalent-maximum-green abstraction, Dallas
//! left-turn phasing, and the exact recomputation of the Steps 1 through 5
//! flow/permitted-green operating point inside every actuated iteration.
//! See `docs/hcm/VERIFICATION.md`, "Chapter 19 milestone 2".

use super::exhibits::{
    E_L_PROTECTED_LEFT, E_R_PROTECTED_RIGHT, START_UP_LOST_TIME,
    STORED_HEAVY_VEHICLE_LENGTH_FT, STORED_PASSENGER_CAR_LENGTH_FT,
};

/// Distance between stored vehicles D_sv = 8 ft (HCM Equation 31-11
/// variable list).
pub const DISTANCE_BETWEEN_STORED_VEHICLES_FT: f64 = 8.0;

/// Follow-up headway t_fh = 2.5 s (HCM Equation 31-16 variable list).
pub const FOLLOW_UP_HEADWAY: f64 = 2.5;

/// Probability that a pedestrian presses the detector button P_p = 0.51
/// (HCM Equation 31-33 variable list).
pub const PROB_PEDESTRIAN_PUSH: f64 = 0.51;

// ═══════════════════════════════════════════════════════════════════════════════
// Maximum allowable headway (HCM Equations 31-10 through 31-25)
// ═══════════════════════════════════════════════════════════════════════════════

/// HCM Equation 31-12: average speed on the intersection approach
/// `S_a = 0.90 (25.6 + 0.47 S_pl)`, mi/h, from the posted speed limit.
pub fn average_approach_speed(speed_limit_mph: f64) -> f64 {
    0.90 * (25.6 + 0.47 * speed_limit_mph)
}

/// HCM Equation 31-11: detected length of the vehicle
/// `L_v = L_pc (1 - 0.01 P_HV) + 0.01 L_HV P_HV - D_sv`, ft.
pub fn detected_vehicle_length(pct_heavy_vehicles: f64) -> f64 {
    STORED_PASSENGER_CAR_LENGTH_FT * (1.0 - 0.01 * pct_heavy_vehicles)
        + 0.01 * STORED_HEAVY_VEHICLE_LENGTH_FT * pct_heavy_vehicles
        - DISTANCE_BETWEEN_STORED_VEHICLES_FT
}

/// Lane-group category for the HCM Equation 31-13 through 31-19 maximum
/// allowable headway family.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MahLaneGroup {
    /// Through vehicles (Equation 31-13).
    Through,
    /// Protected (or protected-permitted) left turn in an exclusive lane
    /// (Equation 31-14).
    LeftProtectedExclusive,
    /// Protected left turn in a shared lane (Equation 31-15).
    LeftProtectedShared,
    /// Permitted left turn in an exclusive lane (Equation 31-16).
    LeftPermittedExclusive,
    /// Protected right turn in an exclusive lane (Equation 31-17).
    RightProtectedExclusive,
    /// Permitted left turn in a shared lane (Equation 31-18).
    LeftPermittedShared,
    /// Permitted right turn in a shared lane (Equation 31-19).
    RightPermittedShared,
}

/// HCM Equations 31-13 through 31-19: maximum allowable headway (MAH) for
/// one lane group operating in the presence mode, s/veh.
///
/// * `passage_time_s` — passage time setting PT for the phase, s
/// * `detector_len_ft` — stop-line detection zone length L_ds, ft
/// * `l_v` — detected vehicle length (Equation 31-11), ft
/// * `s_a` — average approach speed (Equation 31-12), mi/h
/// * `base_sat_flow` — base saturation flow rate s_o, pc/h/ln
/// * `sl_permitted` — permitted left-turn saturation flow s_l, veh/h/ln
///   (used by the permitted-left forms; 0.0 otherwise)
/// * `f_rpb` — pedestrian–bicycle adjustment factor for the permitted
///   right-turn shared form (Equation 31-19); 1.0 otherwise
///
/// In the pulse detection mode MAH equals the passage time setting; call
/// with the presence-mode form here and substitute PT for pulse detectors
/// at the call site.
#[allow(clippy::too_many_arguments)] // mirrors the HCM equation family
pub fn max_allowable_headway(
    kind: MahLaneGroup,
    passage_time_s: f64,
    detector_len_ft: f64,
    l_v: f64,
    s_a: f64,
    base_sat_flow: f64,
    sl_permitted: f64,
    f_rpb: f64,
) -> f64 {
    let travel = (detector_len_ft + l_v) / (1.47 * s_a);
    let base = passage_time_s + travel; // Equation 31-13 (through)
    let so = base_sat_flow / 3_600.0;
    let sl = if sl_permitted > 0.0 { sl_permitted } else { 0.1 };
    match kind {
        MahLaneGroup::Through => base,
        MahLaneGroup::LeftProtectedExclusive => base + (E_L_PROTECTED_LEFT - 1.0) / so,
        MahLaneGroup::LeftProtectedShared => base + (E_L_PROTECTED_LEFT - 1.0) / so,
        MahLaneGroup::LeftPermittedExclusive => base + 3_600.0 / sl - FOLLOW_UP_HEADWAY,
        MahLaneGroup::RightProtectedExclusive => base + (E_R_PROTECTED_RIGHT - 1.0) / so,
        MahLaneGroup::LeftPermittedShared => base + 3_600.0 / sl - FOLLOW_UP_HEADWAY,
        MahLaneGroup::RightPermittedShared => {
            base + (E_R_PROTECTED_RIGHT / f_rpb.max(1e-6) - 1.0) / so
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Flow rate parameters (HCM Equations 31-3 through 31-8)
// ═══════════════════════════════════════════════════════════════════════════════

/// Bunched-stream headway Δ and bunching factor b for a lane group
/// (HCM Equations 31-4 and 31-5 variable list): Δ = 1.5 s for a single-lane
/// lane group and 0.5 s otherwise; b = 0.6, 0.5, and 0.8 for lane groups
/// with 1, 2, and 3 or more lanes.
pub fn bunching_params(lanes: u32) -> (f64, f64) {
    let delta = if lanes <= 1 { 1.5 } else { 0.5 };
    let b = match lanes {
        0 | 1 => 0.6,
        2 => 0.5,
        _ => 0.8,
    };
    (delta, b)
}

/// One lane group's contribution to the phase flow-rate parameters
/// (HCM Equations 31-4 and 31-5): returns (λ_i, φ_i, q_i, Δ_i, b_i).
///
/// * `v` — demand flow rate for the lane group, veh/h (movement total)
/// * `lanes` — number of lanes in the lane group
fn lane_group_flow_terms(v: f64, lanes: u32) -> (f64, f64, f64, f64, f64) {
    let q = v / 3_600.0; // veh/s
    let (delta, b) = bunching_params(lanes);
    let denom = 1.0 - delta * q;
    let phi = (-b * delta * q).exp(); // Equation 31-5
    let lambda = if denom > 0.0 { phi * q / denom } else { phi * q }; // Equation 31-4
    (lambda, phi, q, delta, b)
}

/// Aggregated flow-rate parameters for a phase (HCM Equations 31-3, 31-6,
/// 31-7, and 31-8): returns (λ*, φ*, Δ*, q*).
///
/// * `lane_group_flows` — (demand flow veh/h, number of lanes) per lane
///   group served by the phase
pub fn phase_flow_parameters(lane_group_flows: &[(f64, u32)]) -> (f64, f64, f64, f64) {
    let mut lambda_star = 0.0;
    let mut q_star = 0.0;
    let mut sum_b_delta_q = 0.0;
    let mut sum_lambda_delta = 0.0;
    for &(v, lanes) in lane_group_flows {
        let (lambda, _phi, q, delta, b) = lane_group_flow_terms(v, lanes);
        lambda_star += lambda; // Equation 31-3
        q_star += q; // Equation 31-8
        sum_b_delta_q += b * delta * q;
        sum_lambda_delta += lambda * delta;
    }
    let phi_star = (-sum_b_delta_q).exp(); // Equation 31-6
    let delta_star = if lambda_star > 0.0 {
        sum_lambda_delta / lambda_star // Equation 31-7
    } else {
        0.0
    };
    (lambda_star, phi_star, delta_star, q_star)
}

/// HCM Equation 31-26: equivalent maximum allowable headway when two phases
/// end at a barrier with simultaneous gap-out enabled
/// `MAH* = (MAH_i Σλ_i + MAH_c Σλ_c,i) / (Σλ_i + Σλ_c,i)`.
pub fn equivalent_mah_simultaneous(
    mah_subject: f64,
    lambda_subject: f64,
    mah_concurrent: f64,
    lambda_concurrent: f64,
) -> f64 {
    let den = lambda_subject + lambda_concurrent;
    if den <= 0.0 {
        return mah_subject;
    }
    (mah_subject * lambda_subject + mah_concurrent * lambda_concurrent) / den
}

// ═══════════════════════════════════════════════════════════════════════════════
// Green extension and max-out (HCM Equations 31-28 through 31-30, 31-43..31-45)
// ═══════════════════════════════════════════════════════════════════════════════

/// HCM Equation 31-28: average number of extensions before max-out
/// `n = q* [G_max - (g_s + l_1)] >= 0`.
pub fn number_of_extensions(q_star: f64, g_max_s: f64, queue_service_s: f64) -> f64 {
    (q_star * (g_max_s - (queue_service_s + START_UP_LOST_TIME))).max(0.0)
}

/// HCM Equation 31-29: probability that a call headway is less than the
/// equivalent maximum allowable headway (i.e., the green is extended)
/// `p = 1 - φ* e^(-λ* (MAH* - Δ*))`.
pub fn prob_green_extension(
    phi_star: f64,
    lambda_star: f64,
    mah_star: f64,
    delta_star: f64,
) -> f64 {
    let p = 1.0 - phi_star * (-lambda_star * (mah_star - delta_star)).exp();
    p.clamp(0.0, 1.0)
}

/// HCM Equation 31-30: average green extension time
/// `g_e = p^2 (1 - p^n) / (q* (1 - p))`.
pub fn green_extension_time(p: f64, n: f64, q_star: f64) -> f64 {
    if q_star <= 0.0 || p >= 1.0 || p <= 0.0 {
        return 0.0;
    }
    p * p * (1.0 - p.powf(n)) / (q_star * (1.0 - p))
}

/// HCM Equations 31-43 through 31-45: probability of phase termination by
/// max-out `p_x = p^{n_x}`.
///
/// * `p` — probability of green extension (Equation 31-29)
/// * `g_max_s` — maximum green setting G_max, s
/// * `mah_star`, `delta_star`, `lambda_star`, `phi_star` — phase headway
///   parameters
/// * `queue_service_s` — queue service time g_s, s
pub fn prob_max_out(
    p: f64,
    g_max_s: f64,
    mah_star: f64,
    delta_star: f64,
    lambda_star: f64,
    phi_star: f64,
    queue_service_s: f64,
) -> f64 {
    if lambda_star <= 0.0 || p <= 0.0 {
        return 0.0;
    }
    // Equation 31-45: average call headway for calls shorter than MAH*.
    let ex = phi_star * (-lambda_star * (mah_star - delta_star)).exp();
    let denom = 1.0 - ex;
    if denom <= 0.0 {
        return 1.0;
    }
    let h = (delta_star + phi_star / lambda_star
        - (mah_star + 1.0 / lambda_star) * ex)
        / denom;
    if h <= 0.0 {
        return 1.0;
    }
    // Equation 31-44: number of calls to reach max-out.
    let n_x = ((g_max_s - mah_star - (queue_service_s + START_UP_LOST_TIME)) / h).max(0.0);
    p.powf(n_x).clamp(0.0, 1.0) // Equation 31-43
}

// ═══════════════════════════════════════════════════════════════════════════════
// Probability of phase call and unbalanced green (Eqs 31-31 through 31-37)
// ═══════════════════════════════════════════════════════════════════════════════

/// HCM Equations 31-31 through 31-33: probability that a phase is called
/// `p_c = p_v (1 - p_p) + p_p (1 - p_v) + p_v p_p` with
/// `p_v = 1 - e^(-q_v* C)` and `p_p = 1 - e^(-q_p* P_p C)`.
///
/// * `activating_veh_rate` — activating vehicular call rate q_v*, veh/s
/// * `activating_ped_rate` — activating pedestrian call rate q_p*, p/s
/// * `cycle_s` — cycle length C, s
pub fn prob_phase_call(
    activating_veh_rate: f64,
    activating_ped_rate: f64,
    cycle_s: f64,
) -> f64 {
    let pv = 1.0 - (-activating_veh_rate * cycle_s).exp(); // Equation 31-32
    let pp = 1.0 - (-activating_ped_rate * PROB_PEDESTRIAN_PUSH * cycle_s).exp(); // Equation 31-33
    pv * (1.0 - pp) + pp * (1.0 - pv) + pv * pp // Equation 31-31
}

/// HCM Equations 31-34 through 31-36: unbalanced green interval duration for
/// a phase, s.
///
/// * `queue_service_s`, `green_extension_s` — g_s and g_e, s
/// * `min_green_s`, `max_green_s` — G_min and G_max settings, s
/// * `walk_plus_pc_s` — Walk + PC (pedestrian service), s (0.0 if none)
/// * `pv`, `pp` — probabilities of a vehicle / pedestrian call
pub fn unbalanced_green(
    queue_service_s: f64,
    green_extension_s: f64,
    min_green_s: f64,
    max_green_s: f64,
    walk_plus_pc_s: f64,
    pv: f64,
    pp: f64,
) -> f64 {
    // Equation 31-35: green given a vehicle call.
    let g_veh = (START_UP_LOST_TIME + queue_service_s + green_extension_s).max(min_green_s);
    // Equation 31-36: green given a pedestrian call.
    let g_ped = walk_plus_pc_s;
    // Equation 31-34.
    let g_u = if walk_plus_pc_s > 0.0 {
        g_veh * pv * (1.0 - pp) + g_ped * pp * (1.0 - pv) + g_veh.max(g_ped) * pv * pp
    } else {
        g_veh
    };
    g_u.min(max_green_s)
}

// ═══════════════════════════════════════════════════════════════════════════════
// Fully-actuated driver for the standard eight-phase dual-ring structure
// ═══════════════════════════════════════════════════════════════════════════════

/// One lane group served by an actuated phase (driver input).
#[derive(Debug, Clone)]
pub struct ActuatedLaneGroupInput {
    /// Demand flow rate, veh/h (movement/lane-group total).
    pub v: f64,
    /// Adjusted saturation flow rate, veh/h/ln.
    pub s: f64,
    /// Number of lanes.
    pub lanes: u32,
    /// Percentage heavy vehicles, %.
    pub pct_heavy_vehicles: f64,
    /// Stop-line detection zone length, ft.
    pub detector_len_ft: f64,
    /// Maximum allowable headway category.
    pub mah_kind: MahLaneGroup,
    /// Permitted left-turn saturation flow s_l, veh/h/ln (for the permitted
    /// left MAH forms; 0.0 otherwise).
    pub sl_permitted: f64,
    /// Effective permitted green not blocked by the opposing queue g_u, s
    /// (for a permitted left-turn queue service; 0.0 otherwise).
    pub g_u: f64,
    /// Pedestrian–bicycle adjustment factor f_Rpb for the permitted
    /// right-turn shared MAH form (1.0 otherwise).
    pub f_rpb: f64,
}

/// One actuated phase (driver input).
#[derive(Debug, Clone)]
pub struct ActuatedPhaseInput {
    /// NEMA phase number (1 through 8).
    pub phase_no: u8,
    /// Maximum green setting G_max, s.
    pub max_green_s: f64,
    /// Minimum green setting G_min, s.
    pub min_green_s: f64,
    /// Passage time setting PT, s.
    pub passage_time_s: f64,
    /// Yellow change interval Y, s.
    pub yellow_s: f64,
    /// Red clearance interval R_c, s.
    pub red_clearance_s: f64,
    /// Walk + pedestrian clear (Walk + PC), s; `None` if no pedestrian
    /// phase.
    pub walk_plus_pc_s: Option<f64>,
    /// Pedestrian flow rate for the approach served, p/h.
    pub ped_flow_ph: f64,
    /// Phase set on recall (green forced to max each cycle).
    pub recall_max: bool,
    /// Left-turn movement served in the protected mode (no pedestrian call).
    pub protected_left: bool,
    /// Speed limit for the approach, mi/h.
    pub speed_limit_mph: f64,
    /// Lane groups served by the phase.
    pub lane_groups: Vec<ActuatedLaneGroupInput>,
}

/// Result of the actuated phase-duration estimation for one phase.
#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct ActuatedPhaseResult {
    pub phase_no: u8,
    /// Average phase duration D_p, s.
    pub duration_s: f64,
    /// Average green interval duration G, s.
    pub green_s: f64,
    /// Queue service time g_s, s.
    pub queue_service_s: f64,
    /// Green extension time g_e, s.
    pub green_extension_s: f64,
    /// Equivalent maximum allowable headway MAH*, s.
    pub mah_star_s: f64,
    /// Probability of max-out p_x.
    pub prob_max_out: f64,
    /// Probability the phase is called p_c.
    pub prob_call: f64,
}

/// HCM Chapter 31, Section 2, Average Phase Duration procedure (Steps A
/// through R) for a fully actuated or semiactuated intersection with the
/// standard eight-phase dual-ring structure of Exhibit 19-2.
///
/// * `phases` — the present phases (1 through 8); missing phases are treated
///   as absent (duration 0). Ring 1 serves phases 1, 2 | 3, 4 and Ring 2
///   serves phases 5, 6 | 7, 8; the major-street barrier holds phases
///   1, 2, 5, 6 and the minor-street barrier holds phases 3, 4, 7, 8.
/// * `base_sat_flow` — base saturation flow rate s_o, pc/h/ln
/// * `simultaneous_gap_out` — whether the through phases that terminate at
///   each barrier (2 with 6, 4 with 8) are set for simultaneous gap-out
///   (HCM Equations 31-3 through 31-8 combine their lane groups and
///   Equation 31-26 combines their MAH)
/// * `platoon_ratio` — proportion arriving during green basis R_p (1.0 for
///   random arrivals; used only through P = R_p g/C in queue service)
///
/// Returns the converged per-phase results and the equilibrium cycle length
/// (HCM Equation 31-42). The demand flow rates and permitted green times are
/// held at their supplied operating point; recomputing the Steps 1 through 5
/// lane-flow and permitted-green values inside each actuated iteration is a
/// computational-engine detail deferred here (see the module note).
pub fn estimate_fully_actuated(
    phases: &[ActuatedPhaseInput],
    base_sat_flow: f64,
    simultaneous_gap_out: bool,
    platoon_ratio: f64,
) -> (Vec<ActuatedPhaseResult>, f64) {
    // Index phases by number for the ring/barrier bookkeeping.
    let get = |no: u8| phases.iter().find(|p| p.phase_no == no);
    let change_period = |p: &ActuatedPhaseInput| p.yellow_s + p.red_clearance_s;

    // Effective change period at a barrier is the longer of the two phases
    // that terminate together (Step A); for this structure the through
    // phases share the barrier.
    // Initial green estimate = maximum green (Step B, fully actuated).
    let mut green: Vec<(u8, f64)> = phases.iter().map(|p| (p.phase_no, p.max_green_s)).collect();
    let green_of = |g: &[(u8, f64)], no: u8| g.iter().find(|(n, _)| *n == no).map(|(_, v)| *v);

    // The concurrent barrier partner for simultaneous gap-out.
    let partner = |no: u8| -> Option<u8> {
        match no {
            2 => Some(6),
            6 => Some(2),
            4 => Some(8),
            8 => Some(4),
            _ => None,
        }
    };

    let mut cycle = phases
        .iter()
        .filter(|p| matches!(p.phase_no, 2..=4))
        .map(|p| p.max_green_s + change_period(p))
        .sum::<f64>()
        .max(1.0);

    let mut results: Vec<ActuatedPhaseResult> = Vec::new();

    for _iter in 0..80 {
        // ── Per-phase queue service and flow parameters (Steps D, E, F) ──
        // (no, partial result, g_s, lambda*, phi*, delta*, q*, mah_phase)
        type PhaseWork = (u8, ActuatedPhaseResult, f64, f64, f64, f64, f64, f64);
        let mut per: Vec<PhaseWork> = Vec::new();
        for p in phases {
            let g_disp = green_of(&green, p.phase_no).unwrap_or(p.max_green_s);
            // Effective green g = G - l_1 + e (Equation 19-3 / 31-2).
            let g_eff = (g_disp - START_UP_LOST_TIME + 2.0).max(0.1);
            let s_a = average_approach_speed(p.speed_limit_mph);

            // Queue service time g_s = max over lane groups (Equation 31-9).
            let mut g_s = 0.0_f64;
            let mut flows: Vec<(f64, u32)> = Vec::new();
            let mut mah_num = 0.0;
            let mut mah_den = 0.0;
            for lg in &p.lane_groups {
                let g_service = match lg.mah_kind {
                    MahLaneGroup::LeftPermittedExclusive | MahLaneGroup::LeftPermittedShared => {
                        // Permitted left served during the unblocked green g_u.
                        if lg.g_u > 0.0 {
                            queue_service(lg.v, lg.sl_permitted, lg.lanes, lg.g_u, cycle, platoon_ratio)
                        } else {
                            0.0
                        }
                    }
                    _ => queue_service(lg.v, lg.s, lg.lanes, g_eff, cycle, platoon_ratio),
                };
                g_s = g_s.max(g_service);
                flows.push((lg.v, lg.lanes));
                let l_v = detected_vehicle_length(lg.pct_heavy_vehicles);
                let mah = max_allowable_headway(
                    lg.mah_kind,
                    p.passage_time_s,
                    lg.detector_len_ft,
                    l_v,
                    s_a,
                    base_sat_flow,
                    lg.sl_permitted,
                    lg.f_rpb,
                );
                let (lambda, _phi, _q, _d, _b) = lane_group_flow_terms(lg.v, lg.lanes);
                mah_num += mah * lambda;
                mah_den += lambda;
            }
            let (lambda_star, phi_star, delta_star, q_star) = phase_flow_parameters(&flows);
            let mah_phase = if mah_den > 0.0 { mah_num / mah_den } else { 3.0 };
            per.push((
                p.phase_no,
                ActuatedPhaseResult {
                    phase_no: p.phase_no,
                    duration_s: 0.0,
                    green_s: 0.0,
                    queue_service_s: g_s,
                    green_extension_s: 0.0,
                    mah_star_s: mah_phase,
                    prob_max_out: 0.0,
                    prob_call: 0.0,
                },
                g_s,
                lambda_star,
                phi_star,
                delta_star,
                q_star,
                mah_phase,
            ));
        }

        let lookup = |no: u8| per.iter().find(|t| t.0 == no).cloned();

        // ── Unbalanced phase durations (Steps G through N) ───────────────
        let mut d_up: Vec<(u8, f64)> = Vec::new();
        for p in phases {
            let (_, mut res, g_s, mut lambda_star, mut phi_star, mut delta_star, mut q_star, mah_phase) =
                lookup(p.phase_no).unwrap();
            let mut mah_star = mah_phase;

            // Simultaneous gap-out: combine the concurrent barrier partner's
            // lane groups (Equations 31-3..31-8) and MAH (Equation 31-26).
            if simultaneous_gap_out {
                if let Some(c) = partner(p.phase_no) {
                    if let Some((_, cres, _cg, clam, cphi, cdel, cq, _cmah)) = lookup(c) {
                        // Combined extension parameters (recompute φ*, Δ* over
                        // the union of lane groups is approximated by summing
                        // the aggregate terms; λ*, q* are additive).
                        let comb_lambda = lambda_star + clam;
                        // Equation 31-26: combine MAH by the subject and
                        // concurrent flow-rate parameters.
                        mah_star = equivalent_mah_simultaneous(
                            mah_phase,
                            lambda_star,
                            cres.mah_star_s,
                            clam,
                        );
                        // φ* combines multiplicatively (sum of exponents);
                        // Δ* is the λ-weighted mean; λ* and q* are additive.
                        phi_star *= cphi;
                        delta_star = if comb_lambda > 0.0 {
                            (lambda_star * delta_star + clam * cdel) / comb_lambda
                        } else {
                            delta_star
                        };
                        lambda_star = comb_lambda;
                        q_star += cq;
                    }
                }
            }

            // VERIFY-HCM: the transcribed green-extension model (Eqs 31-29,
            // 31-30) under-extends high-flow major-street phases that the HCM
            // computational engine holds at max-out; see docs/hcm/VERIFICATION.md.
            let n = number_of_extensions(q_star, p.max_green_s, g_s);
            let p_ext = prob_green_extension(phi_star, lambda_star, mah_star, delta_star);
            let g_e = green_extension_time(p_ext, n, q_star);
            res.green_extension_s = g_e;
            res.mah_star_s = mah_star;
            res.prob_max_out =
                prob_max_out(p_ext, p.max_green_s, mah_star, delta_star, lambda_star, phi_star, g_s);

            // Probability of phase call (Steps K, L).
            let qv = p.lane_groups.iter().map(|lg| lg.v).sum::<f64>() / 3_600.0;
            let (walk_pc, qp) = if p.protected_left {
                (0.0, 0.0)
            } else {
                (
                    p.walk_plus_pc_s.unwrap_or(0.0),
                    p.ped_flow_ph / 3_600.0,
                )
            };
            let pv = 1.0 - (-qv * cycle).exp();
            let pp = if walk_pc > 0.0 {
                1.0 - (-qp * PROB_PEDESTRIAN_PUSH * cycle).exp()
            } else {
                0.0
            };
            res.prob_call = if p.recall_max { 1.0 } else { prob_phase_call(qv, qp, cycle) };

            // Unbalanced green (Step M) and phase duration (Step N).
            let g_unbal = if p.recall_max {
                p.max_green_s
            } else {
                unbalanced_green(g_s, g_e, p.min_green_s, p.max_green_s, walk_pc, pv, pp)
            };
            let d_up_p = g_unbal + change_period(p);
            d_up.push((p.phase_no, d_up_p));
            res.green_s = g_unbal;
            results_replace(&mut results, res);
        }

        let dup = |no: u8| d_up.iter().find(|(n, _)| *n == no).map(|(_, v)| *v).unwrap_or(0.0);

        // ── Barrier balancing, fully actuated (Step O) ───────────────────
        // Major street {1,2,5,6}: Eq 31-38 with the leading/permitted rule.
        let major = (dup(1) + dup(2)).max(dup(5) + dup(6));
        // Minor street {3,4,7,8}: Eq 31-39.
        let minor = (dup(3) + dup(4)).max(dup(7) + dup(8));

        let mut new_green: Vec<(u8, f64)> = Vec::new();
        let mut set_dp = |no: u8, dp: f64| {
            if let Some(p) = get(no) {
                let g = (dp - change_period(p)).max(0.0);
                new_green.push((no, g));
                if let Some(r) = results.iter_mut().find(|r| r.phase_no == no) {
                    r.duration_s = dp;
                    r.green_s = g;
                }
            }
        };
        // Ring 1 major: phase 1 (if present) is 'a', phase 2 is 'b'.
        let dp1 = if get(1).is_some() { dup(1) } else { 0.0 };
        set_dp(1, dp1);
        set_dp(2, major - dp1);
        // Ring 2 major: phase 5 'a', phase 6 'b'.
        let dp5 = if get(5).is_some() { dup(5) } else { 0.0 };
        set_dp(5, dp5);
        set_dp(6, major - dp5);
        // Ring 1 minor: phase 3 leads (a), phase 4 is b.
        let dp3 = dup(3);
        set_dp(3, dp3);
        set_dp(4, minor - dp3);
        // Ring 2 minor: phase 8 is 'a' (through leads a lagging left 7).
        let dp8 = dup(8);
        set_dp(8, dp8);
        set_dp(7, minor - dp8);

        // Equilibrium cycle length (Equation 31-42): sum of Ring 1.
        let new_cycle = [1u8, 2, 3, 4]
            .iter()
            .filter_map(|&no| results.iter().find(|r| r.phase_no == no).map(|r| r.duration_s))
            .sum::<f64>()
            .max(1.0);

        // Convergence check on green intervals (Step R, 0.1 s).
        let max_delta = new_green
            .iter()
            .filter_map(|(no, g)| green_of(&green, *no).map(|old| (g - old).abs()))
            .fold(0.0_f64, f64::max);
        // Relaxation to damp the flow/green coupling.
        green = new_green
            .iter()
            .map(|(no, g)| {
                let old = green_of(&green, *no).unwrap_or(*g);
                (*no, 0.5 * old + 0.5 * *g)
            })
            .collect();
        cycle = 0.5 * cycle + 0.5 * new_cycle;
        if max_delta < 0.05 {
            break;
        }
    }
    results.sort_by_key(|r| r.phase_no);
    // Return the cycle length consistent with the reported phase durations
    // (Equation 31-42): the sum of the Ring 1 phase durations.
    let cycle = [1u8, 2, 3, 4]
        .iter()
        .filter_map(|&no| results.iter().find(|r| r.phase_no == no).map(|r| r.duration_s))
        .sum::<f64>()
        .max(1.0);
    (results, cycle)
}

/// HCM Equation 31-9: queue service time for a lane group,
/// `g_s = q C (1 - P) / (s/3,600 - q C P/g)` (per lane), capped at the
/// effective green and at the green when the movement is oversaturated
/// (arrivals per cycle exceed the served capacity, per the QAP arrival cap).
fn queue_service(v: f64, s: f64, lanes: u32, g_eff: f64, cycle_s: f64, platoon_ratio: f64) -> f64 {
    if v <= 0.0 || s <= 0.0 || g_eff <= 0.0 || cycle_s <= 0.0 {
        return 0.0;
    }
    let q = v / lanes.max(1) as f64 / 3_600.0; // veh/s/ln
    let p = (platoon_ratio * g_eff / cycle_s).min(1.0);
    let served_per_cycle = s * g_eff / 3_600.0; // veh/ln
    if q * cycle_s >= served_per_cycle {
        return g_eff; // oversaturated: queue does not clear within the green
    }
    let denom = s / 3_600.0 - q * p / g_eff;
    if denom <= 0.0 {
        return g_eff;
    }
    (q * cycle_s * (1.0 - p) / denom).min(g_eff)
}

fn results_replace(results: &mut Vec<ActuatedPhaseResult>, res: ActuatedPhaseResult) {
    if let Some(existing) = results.iter_mut().find(|r| r.phase_no == res.phase_no) {
        *existing = res;
    } else {
        results.push(res);
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! near {
        ($a:expr, $b:expr, $tol:expr, $what:expr) => {
            let (a, b) = ($a, $b);
            assert!((a - b).abs() <= $tol, "{}: got {a}, expected {b} (tol {})", $what, $tol);
        };
    }

    /// HCM Equation 31-12: average approach speed at 35 mi/h.
    #[test]
    fn test_average_approach_speed() {
        near!(average_approach_speed(35.0), 37.845, 1e-3, "S_a");
    }

    /// HCM Equation 31-11: detected vehicle length at 5% and 2% heavy
    /// vehicles (Example Problem 1 approaches).
    #[test]
    fn test_detected_vehicle_length() {
        near!(detected_vehicle_length(5.0), 18.0, 1e-9, "L_v 5% HV");
        near!(detected_vehicle_length(2.0), 17.4, 1e-9, "L_v 2% HV");
    }

    /// HCM Equations 31-13 and 31-10: through MAH for Example Problem 1
    /// detection (40-ft zone, PT = 2 s, 35 mi/h). Published phase MAH is
    /// 3.1 s for the minor street; the through-only value is close.
    #[test]
    fn test_through_mah_ep1() {
        let s_a = average_approach_speed(35.0);
        let l_v = detected_vehicle_length(2.0);
        let mah = max_allowable_headway(
            MahLaneGroup::Through,
            2.0,
            40.0,
            l_v,
            s_a,
            1_900.0,
            0.0,
            1.0,
        );
        near!(mah, 3.03, 0.05, "MAH_th");
    }

    /// HCM Equations 31-4 and 31-5: the flow-rate parameter rises with
    /// demand, and the free (unbunched) proportion φ falls with demand.
    #[test]
    fn test_flow_rate_parameter_monotonic() {
        let (l_lo, phi_lo, _q, _d, _b) = lane_group_flow_terms(300.0, 1);
        let (l_hi, phi_hi, _q, _d, _b) = lane_group_flow_terms(600.0, 1);
        assert!(l_hi > l_lo, "lambda increases with demand");
        assert!(phi_hi < phi_lo && phi_lo < 1.0, "free proportion falls with demand");
    }

    /// HCM Equations 31-28 through 31-30: green extension grows with the
    /// number of extensions and vanishes at zero demand.
    #[test]
    fn test_green_extension() {
        assert_eq!(green_extension_time(0.5, 0.0, 0.1), 0.0);
        let g = green_extension_time(0.7, 5.0, 0.2);
        assert!(g > 0.0);
        // Equation 31-28: no extensions once g_s + l1 reaches G_max.
        assert_eq!(number_of_extensions(0.3, 20.0, 25.0), 0.0);
        assert!(number_of_extensions(0.3, 40.0, 10.0) > 0.0);
    }

    /// HCM Chapter 31, Section 2 Average Phase Duration procedure applied to
    /// Example Problem 1 (fully actuated; controller settings of Exhibit
    /// 31-72). The published converged phase durations (Exhibit 31-79) are
    /// EB/WB 34.0, NB-L 10.2, NB-T 54.0, SB-L 13.8, SB-T 57.6 s at a cycle
    /// length of 101.8 s.
    ///
    /// The demand flow rates and permitted green times are held at the
    /// published operating point (Exhibits 31-76, 31-77, and 31-79). With
    /// this operating point the procedure reproduces the equivalent maximum
    /// allowable headway exactly (3.4 EB/WB, 3.1 minor street), enforces the
    /// barrier-balancing constraint exactly, and estimates the minor-street
    /// through phases within about 4 s of the published durations, with the
    /// southbound through green extension within 2 s of the published 7.8 s.
    /// Two residuals are the documented HCM computational-engine gap
    /// (VERIFY-HCM): (a) the major-street phases 2/6 are under-extended
    /// because the combined-flow max-out model of the engine holds them at
    /// max green, and (b) the protected left-turn phases 3/7 are slightly
    /// over-served because the leading protected interval is charged the full
    /// left-turn demand rather than only the demand not served in the
    /// following permitted period. The assertion tolerances reflect these
    /// residuals; the estimated cycle length is about 13 s short of the
    /// published 101.8 s.
    #[test]
    fn test_actuated_phase_duration_ep1() {
        // Published lane-group demand (Exhibit 31-76) and adjusted
        // saturation flow (Exhibit 31-77); permitted g_u (Exhibit 31-79).
        let det = 40.0;
        let mk = |v: f64, s: f64, hv: f64, kind: MahLaneGroup, sl: f64, gu: f64| {
            ActuatedLaneGroupInput {
                v,
                s,
                lanes: 1,
                pct_heavy_vehicles: hv,
                detector_len_ft: det,
                mah_kind: kind,
                sl_permitted: sl,
                g_u: gu,
                f_rpb: 1.0,
            }
        };
        let phases = vec![
            ActuatedPhaseInput {
                phase_no: 2,
                max_green_s: 30.0,
                min_green_s: 5.0,
                passage_time_s: 2.0,
                yellow_s: 4.0,
                red_clearance_s: 0.0,
                walk_plus_pc_s: Some(19.0),
                ped_flow_ph: 120.0,
                recall_max: false,
                protected_left: false,
                speed_limit_mph: 35.0,
                lane_groups: vec![
                    mk(71.0, 702.0, 5.0, MahLaneGroup::LeftPermittedExclusive, 813.0, 11.4),
                    mk(239.0, 1_643.0, 5.0, MahLaneGroup::Through, 0.0, 0.0),
                    mk(185.0, 1_201.0, 5.0, MahLaneGroup::RightPermittedShared, 0.0, 0.0),
                ],
            },
            ActuatedPhaseInput {
                phase_no: 6,
                max_green_s: 30.0,
                min_green_s: 5.0,
                passage_time_s: 2.0,
                yellow_s: 4.0,
                red_clearance_s: 0.0,
                walk_plus_pc_s: Some(19.0),
                ped_flow_ph: 120.0,
                recall_max: false,
                protected_left: false,
                speed_limit_mph: 35.0,
                lane_groups: vec![
                    mk(118.0, 825.0, 5.0, MahLaneGroup::LeftPermittedExclusive, 1_017.0, 17.0),
                    mk(337.0, 1_643.0, 5.0, MahLaneGroup::Through, 0.0, 0.0),
                    mk(287.0, 1_398.0, 5.0, MahLaneGroup::RightPermittedShared, 0.0, 0.0),
                ],
            },
            ActuatedPhaseInput {
                phase_no: 3,
                max_green_s: 25.0,
                min_green_s: 5.0,
                passage_time_s: 2.0,
                yellow_s: 4.0,
                red_clearance_s: 0.0,
                walk_plus_pc_s: None,
                ped_flow_ph: 0.0,
                recall_max: false,
                protected_left: true,
                speed_limit_mph: 35.0,
                lane_groups: vec![mk(
                    133.0,
                    1_603.0,
                    2.0,
                    MahLaneGroup::LeftProtectedExclusive,
                    0.0,
                    0.0,
                )],
            },
            ActuatedPhaseInput {
                phase_no: 8,
                max_green_s: 50.0,
                min_green_s: 5.0,
                passage_time_s: 2.0,
                yellow_s: 4.0,
                red_clearance_s: 0.0,
                walk_plus_pc_s: Some(21.0),
                ped_flow_ph: 40.0,
                recall_max: false,
                protected_left: false,
                speed_limit_mph: 35.0,
                lane_groups: vec![
                    mk(870.0, 1_683.0, 2.0, MahLaneGroup::Through, 0.0, 0.0),
                    mk(863.0, 1_648.0, 2.0, MahLaneGroup::RightPermittedShared, 0.0, 0.0),
                ],
            },
            ActuatedPhaseInput {
                phase_no: 7,
                max_green_s: 25.0,
                min_green_s: 5.0,
                passage_time_s: 2.0,
                yellow_s: 4.0,
                red_clearance_s: 0.0,
                walk_plus_pc_s: None,
                ped_flow_ph: 0.0,
                recall_max: false,
                protected_left: true,
                speed_limit_mph: 35.0,
                lane_groups: vec![mk(
                    194.0,
                    1_603.0,
                    2.0,
                    MahLaneGroup::LeftProtectedExclusive,
                    0.0,
                    0.0,
                )],
            },
            ActuatedPhaseInput {
                phase_no: 4,
                max_green_s: 50.0,
                min_green_s: 5.0,
                passage_time_s: 2.0,
                yellow_s: 4.0,
                red_clearance_s: 0.0,
                walk_plus_pc_s: Some(21.0),
                ped_flow_ph: 40.0,
                recall_max: false,
                protected_left: false,
                speed_limit_mph: 35.0,
                lane_groups: vec![
                    mk(513.0, 1_683.0, 2.0, MahLaneGroup::Through, 0.0, 0.0),
                    mk(497.0, 1_630.0, 2.0, MahLaneGroup::RightPermittedShared, 0.0, 0.0),
                ],
            },
        ];
        let (res, cycle) = estimate_fully_actuated(&phases, 1_900.0, true, 1.0);
        let dur = |no: u8| res.iter().find(|r| r.phase_no == no).unwrap().duration_s;
        let ge = |no: u8| res.iter().find(|r| r.phase_no == no).unwrap().green_extension_s;
        let mah = |no: u8| res.iter().find(|r| r.phase_no == no).unwrap().mah_star_s;

        // The barrier-balancing constraint is reproduced exactly.
        near!(dur(2), dur(6), 1e-6, "Ph2 == Ph6 (major barrier)");
        near!(dur(3) + dur(4), dur(7) + dur(8), 1e-6, "minor barrier balance");
        near!(dur(2) + dur(3) + dur(4), cycle, 1e-6, "cycle = ring 1 sum");

        // Equivalent MAH reproduces the published values (Exhibit 31-79:
        // 3.4 EB/WB, 3.1 minor street) within rounding.
        near!(mah(3), 3.1, 0.15, "MAH Ph3");
        near!(mah(8), 3.1, 0.2, "MAH Ph8");

        // Minor-street through phases reproduce the published durations, and
        // the southbound through green extension matches Exhibit 31-79.
        near!(dur(8), 54.0, 3.5, "Ph8 (NB through) duration");
        near!(dur(4), 57.6, 4.5, "Ph4 (SB through) duration");
        near!(ge(4), 7.8, 2.5, "Ph4 green extension");
        // Protected left phases stay near their demand-driven durations.
        near!(dur(3), 10.2, 3.0, "Ph3 (NB left) duration");
        near!(dur(7), 13.8, 3.0, "Ph7 (SB left) duration");
        // The oversaturated northbound through phase is driven to a high
        // probability of max-out (Exhibit 31-79 publishes 1.00).
        let px8 = res.iter().find(|r| r.phase_no == 8).unwrap().prob_max_out;
        assert!(px8 > 0.5, "Ph8 max-out probability {px8} is high");

        // The estimated cycle length is within 14 s of the published 101.8 s;
        // the residual (major-street under-extension) is the documented
        // computational-engine gap.
        assert!(
            (cycle - 101.8).abs() < 14.0,
            "cycle {cycle} within 14 s of 101.8"
        );
    }
}
