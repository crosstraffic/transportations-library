//! # Delay due to Turns at Access Points (HCM Chapter 30, Section 4)
//!
//! Implements the Chapter 30 procedure for the delay imposed on major-street
//! through vehicles by vehicles turning left or right from the major street
//! into an unsignalized access point intersection (EPUB source
//! `236_Ch30_04.xhtml`, Equations 30-31 through 30-68). The per-access-point
//! result `d_ap = d_ap,l + d_ap,r` is the "delay to through vehicles"
//! reported in Exhibit 30-35; summed over the influential access points it
//! is the `Σ d_ap,i` term of the Chapter 18 running-time equation
//! (Equation 18-7), replacing the Exhibit 18-13 planning estimate when the
//! access-point geometry and turn volumes are supplied.
//!
//! ## Scope and assumptions
//!
//! * The procedure assumes random (unplatooned) segment flow — "conservative
//!   in that it will yield slightly larger estimates of delay" (Chapter 30,
//!   Section 4).
//! * `d_ap,l` (Equations 30-31 through 30-54) is the delay to inside-lane
//!   through vehicles that stop behind a left-turn queue that has overflowed
//!   the bay (an undivided cross section has no left-turn storage, so the
//!   inside lane is blocked with probability `p_ov = v_lt / c_l`).
//! * `d_ap,r` (Equations 30-55 through 30-68) is the delay to outside-lane
//!   through vehicles that slow behind a right-turning vehicle.
//!
//! ## VERIFY-HCM (turn-delay approach speed)
//!
//! Equations 30-56 and 30-58 define the approaching-vehicle speed as
//! `S_f = free-flow speed`. In Example Problem 1 the published per-access-
//! point delay (0.193/0.194 s/veh, Exhibit 30-35) and the published
//! probability of inside-lane blockage (0.115) reproduce exactly when the
//! right-turn branch uses the **posted speed limit** (35 mi/h) as the
//! approach speed; using the segment free-flow speed (39.33 mi/h) yields
//! 0.217 s/veh. The reference computational engine therefore appears to
//! evaluate the right-turn maneuver at the posted speed. `p_ov` and
//! `d_ap,l` are independent of the approach speed and reproduce the
//! published values regardless. See `docs/hcm/VERIFICATION.md`.

use serde::{Deserialize, Serialize};

// ─────────────────────────── Model constants ───────────────────────────

/// Critical merge headway t_lc (Equations 30-32 and 30-47), s.
const T_LC: f64 = 3.7;
/// Follow-up headway for a permitted left turn t_fh (Equation 30-35), s.
const T_FH: f64 = 2.2;
/// Critical headway for a permitted left turn t_cg (Equation 30-35), s.
const T_CG: f64 = 4.1;
/// Right-turn speed u_rt (Equations 30-56 and 30-58), ft/s.
const U_RT: f64 = 20.0;
/// Deceleration rate r_d (Equations 30-56, 30-58, and 30-60), ft/s².
const R_D: f64 = 6.7;
/// Acceleration rate r_a (Equation 30-60), ft/s².
const R_A: f64 = 3.5;
/// Clearance time of the right-turn vehicle t_cl (Equation 30-58), s.
const T_CL: f64 = 0.6;
/// Headway of the bunched vehicle stream Δ (Equations 30-56 through 30-66),
/// s/veh.
const DELTA: f64 = 1.5;
/// Stored passenger-car lane length L_pc (Equation 30-15), ft.
const L_PC: f64 = 25.0;
/// Stored heavy-vehicle lane length L_HV (Equation 30-15), ft.
const L_HV: f64 = 45.0;
/// ft/s per mi/h (1 mi/h = 1.47 ft/s, the HCM rounding used in
/// Equations 30-56 through 30-60).
const FT_S_PER_MPH: f64 = 1.47;

// ───────────────────────── Equation primitives ─────────────────────────

/// HCM Equation 30-15: average vehicle spacing in a stationary queue,
/// `L_h = L_pc (1 − 0.01 P_HV) + 0.01 L_HV P_HV`, ft/veh.
///
/// * `pct_heavy_veh` — percent heavy vehicles P_HV in the movement group (%)
pub fn stationary_queue_spacing(pct_heavy_veh: f64) -> f64 {
    L_PC * (1.0 - 0.01 * pct_heavy_veh) + 0.01 * L_HV * pct_heavy_veh
}

/// HCM Equation 30-35: capacity of a permitted left-turn movement,
/// `c_l = v_o e^(−v_o t_cg/3,600) / (1 − e^(−v_o t_fh/3,600))`, veh/h.
///
/// * `opposing_flow_veh_h` — opposing demand flow rate v_o (through plus
///   right turn), veh/h
pub fn permitted_left_capacity(opposing_flow_veh_h: f64) -> f64 {
    let v_o = opposing_flow_veh_h;
    if v_o <= 0.0 {
        return 1_800.0;
    }
    let num = v_o * (-v_o * T_CG / 3_600.0).exp();
    let den = 1.0 - (-v_o * T_FH / 3_600.0).exp();
    if den <= 0.0 {
        return 1_800.0;
    }
    num / den
}

/// HCM Equation 30-32: probability of a lane change among the approach
/// through lanes, `P_lc = 1 − [(2 v_app / s_lc) − 1]² ≥ 0.0`, with
/// `s_lc = 3,600 / t_lc` (Equation 30-33 supplies `v_app`). The ratio
/// `v_app / s_lc` is capped at 1.0 per the Chapter 30 text.
///
/// * `v_app` — average demand flow rate per through lane, veh/h/ln
pub fn probability_lane_change(v_app: f64) -> f64 {
    let s_lc = 3_600.0 / T_LC;
    let ratio = (v_app / s_lc).min(1.0);
    (1.0 - (2.0 * ratio - 1.0).powi(2)).max(0.0)
}

/// HCM Equation 30-53/30-54: probability of left-turn bay overflow,
/// `p_ov = (v_lt / c_l)^(N_qx,lt + 1)` with
/// `N_qx,lt = N_lt L_a,lt / L_h` (0.0 for an undivided cross section, so
/// `p_ov = v_lt / c_l`).
///
/// * `v_lt` — left-turn demand flow rate, veh/h
/// * `c_l` — permitted left-turn capacity, veh/h (Equation 30-35)
/// * `n_qx_lt` — maximum left-turn queue storage N_qx,lt, veh
pub fn probability_left_bay_overflow(v_lt: f64, c_l: f64, n_qx_lt: f64) -> f64 {
    if c_l <= 0.0 {
        return 1.0;
    }
    (v_lt / c_l).powf(n_qx_lt + 1.0).min(1.0)
}

/// Randomized (incremental) queue-delay term shared by Equations 30-48 and
/// 30-51, `3,600 (1/c − 1/1,800) + 900 T [ v/c − 1 + √((v/c − 1)² + 8 v /
/// (c² T)) ]`, s/veh.
fn incremental_delay(v: f64, c: f64, t_h: f64) -> f64 {
    if c <= 0.0 {
        return 0.0;
    }
    let x = v / c;
    3_600.0 * (1.0 / c - 1.0 / 1_800.0)
        + 900.0 * t_h * ((x - 1.0) + ((x - 1.0).powi(2) + 8.0 * v / (c * c * t_h)).sqrt())
}

// ─────────────────────────── Inputs ───────────────────────────

/// One major-street approach to an access point intersection: the
/// through-and-turn volumes turning in from the segment, the approach lane
/// configuration, and the opposing flow. Populate one per direction of
/// travel evaluated.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct AccessPointApproach {
    /// Left-turn demand flow rate v_lt from the major street into the access
    /// point, veh/h.
    pub v_lt: f64,
    /// Through demand flow rate v_th on the major-street approach, veh/h.
    pub v_th: f64,
    /// Right-turn demand flow rate v_rt from the major street into the access
    /// point, veh/h.
    pub v_rt: f64,
    /// Number of lanes in the shared left-turn/through lane group N_sl (ln).
    pub n_sl: u32,
    /// Number of lanes in the exclusive-through lane group N_t (ln).
    pub n_t: u32,
    /// Number of lanes in the shared right-turn/through lane group N_sr (ln).
    pub n_sr: u32,
    /// Opposing demand flow rate v_o for the permitted left-turn capacity
    /// (Equation 30-35): opposing through plus opposing right turn, veh/h.
    pub opposing_flow_veh_h: f64,
    /// True if a left-turn bay is provided on the major street at the access
    /// point (I_lt = 0; E_L1 = 1.0).
    pub left_turn_bay: bool,
    /// True if a right-turn bay is provided on the major street at the access
    /// point (I_rt = 0; E_R,ap = 1.0).
    pub right_turn_bay: bool,
    /// Number of lanes in the left-turn bay N_lt (ln); used with the bay
    /// storage length for N_qx,lt (Equation 30-54).
    pub n_lt_lanes: u32,
    /// Available left-turn bay storage distance L_a,lt, ft/ln (Equation
    /// 30-54). Ignored for an undivided cross section (`left_turn_bay`
    /// false), where N_qx,lt = 0.
    pub left_bay_storage_ft: f64,
    /// Percent heavy vehicles P_HV in the movement group (Equation 30-15), %.
    pub pct_heavy_veh: f64,
}

impl AccessPointApproach {
    /// Total number of through lanes on the approach, `N_sl + N_t + N_sr`.
    fn n_through(&self) -> u32 {
        (self.n_sl + self.n_t + self.n_sr).max(1)
    }
    /// Proportion of left-turning vehicles on the approach P_lt.
    fn p_lt(&self) -> f64 {
        let tot = self.v_lt + self.v_th + self.v_rt;
        if tot <= 0.0 {
            0.0
        } else {
            self.v_lt / tot
        }
    }
    /// Proportion of right-turning vehicles on the approach P_rt.
    fn p_rt(&self) -> f64 {
        let tot = self.v_lt + self.v_th + self.v_rt;
        if tot <= 0.0 {
            0.0
        } else {
            self.v_rt / tot
        }
    }
}

/// The three lane-proportion / lane-flow intermediates of Equations 30-38
/// through 30-46 (P_L, P_R, and the inside/outside lane flow rates).
#[derive(Debug, Clone, Copy)]
struct LaneSplit {
    /// Proportion of left turns in the inside through lane P_L (Equation
    /// 30-38).
    p_l: f64,
    /// Proportion of right turns in the outside through lane P_R (Equation
    /// 30-42).
    p_r: f64,
    /// Inside-lane flow rate v_1, veh/h/ln (Equation 30-44).
    v_1: f64,
    /// Outside-lane flow rate v_n, veh/h/ln (Equation 30-45).
    v_n: f64,
    /// Adjacent-lane flow rate v_2 used by the merge capacity (Equation
    /// 30-47): the intermediate-lane flow (Equation 30-46) for approaches
    /// with more than two lanes, else the outside-lane flow.
    v_2: f64,
}

/// Solve Equations 30-36 through 30-46 for the lane split. `i_t` is the
/// indicator variable I_t (1.0 for left-turn-delay evaluation, 0.00001 for
/// right-turn-delay evaluation); `p_lc` is the lane-change probability
/// (Equation 30-32 for the left-turn branch, forced to 1.0 for the
/// right-turn branch per the Chapter 30 text).
fn lane_split(ap: &AccessPointApproach, e_l1: f64, i_t: f64, p_lc: f64) -> LaneSplit {
    let n_through = ap.n_through() as f64;
    let p_lt = ap.p_lt();
    let p_rt = ap.p_rt();
    let i_lt = if ap.left_turn_bay { 0.0 } else { 1.0 };
    let i_rt = if ap.right_turn_bay { 0.0 } else { 1.0 };
    let e_r_ap = if ap.right_turn_bay { 1.0 } else { 2.20 };

    // Equations 30-36 and 30-37 (modified through-car equivalents).
    let e_l1_m = (e_l1 - 1.0) * p_lc + 1.0;
    let e_r_m = (e_r_ap - 1.0) * p_lc + 1.0;

    // Equation 30-41.
    let r = 1.0 + i_rt * p_rt * (e_r_m - 1.0);

    // Equation 30-38 (with Equations 30-39 and 30-40), P_L.
    let p_l = if ap.n_through() == 1 {
        p_lt
    } else {
        // Equation 30-39.
        let b = r - i_lt * p_lt * (i_t + (n_through - 1.0) * ((1.0 + i_t) * e_l1_m - 1.0));
        // Equation 30-40.
        let c = -i_lt * p_lt * n_through;
        let disc = (b * b - 4.0 * i_t * r * c).max(0.0);
        ((-b + disc.sqrt()) / (2.0 * i_t * r)).min(1.0)
    };

    // Equation 30-43 (saturation flow rate for the inside lane).
    let s_1 = 1_800.0 * (1.0 + p_l * i_t)
        / (1.0 + p_l * (e_l1_m - 1.0) + p_l * e_l1_m * i_t);

    // Equation 30-42, P_R.
    let p_r = if ap.n_through() == 1 {
        p_rt
    } else {
        let num = s_1 / 1_800.0 + n_through - 1.0;
        let den = 1.0 - i_rt * p_rt * (s_1 / 1_800.0 + n_through - 2.0) * (e_r_m - 1.0);
        (i_rt * p_rt * num / den).min(1.0)
    };

    // Equations 30-44 and 30-45.
    let v_1 = if p_l > 0.0 { ap.v_lt / p_l } else { 0.0 };
    let v_n = if p_r > 0.0 {
        ap.v_rt / p_r
    } else {
        (ap.v_lt + ap.v_th + ap.v_rt - v_1) / (n_through - 1.0).max(1.0)
    };
    // Equation 30-46 (intermediate lanes) for more than two lanes.
    let v_2 = if ap.n_through() > 2 {
        (ap.v_lt + ap.v_th + ap.v_rt - v_1 - v_n) / (n_through - 2.0)
    } else {
        v_n
    };

    LaneSplit { p_l, p_r, v_1, v_n, v_2 }
}

/// Result of the access-point through-delay procedure for one approach.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct AccessPointDelay {
    /// Through-vehicle delay due to left turns d_ap,l (Equation 30-31),
    /// s/veh.
    pub delay_left_s: f64,
    /// Through-vehicle delay due to right turns d_ap,r (Equation 30-55),
    /// s/veh.
    pub delay_right_s: f64,
    /// Total through-vehicle delay `d_ap = d_ap,l + d_ap,r`, s/veh (the
    /// Exhibit 30-35 "delay to through vehicles" value).
    pub delay_total_s: f64,
    /// Probability of left-turn bay overflow / inside-lane blockage p_ov
    /// (Equation 30-53; the Exhibit 30-35 "probability of inside through
    /// lane being blocked").
    pub prob_inside_lane_blocked: f64,
}

/// HCM Chapter 30, Section 4: delay to through vehicles due to left and
/// right turns from the major street at one access point approach.
///
/// * `ap` — the major-street approach turn-in volumes and lane geometry
/// * `speed_mph` — approaching through-vehicle speed used by the right-turn
///   branch (Equations 30-56 and 30-58; see the VERIFY-HCM note in the
///   module docs — the posted speed reproduces the Example Problem 1
///   published value)
/// * `analysis_period_h` — analysis period duration T (Equations 30-48 and
///   30-51), h
pub fn access_point_through_delay(
    ap: &AccessPointApproach,
    speed_mph: f64,
    analysis_period_h: f64,
) -> AccessPointDelay {
    let p_lt = ap.p_lt();
    let p_rt = ap.p_rt();
    let n_through = ap.n_through() as f64;
    let t_h = analysis_period_h.max(1e-6);

    // Through-vehicle equivalent for a permitted left turn (Equations 30-34
    // and 30-35); 1.0 when a left-turn bay is provided.
    let c_l = permitted_left_capacity(ap.opposing_flow_veh_h);
    let e_l1 = if ap.left_turn_bay { 1.0 } else { 1_800.0 / c_l };

    // ───────────── Delay due to left turns (I_t = 1.0) ─────────────
    // Step 1: lane-change probability (Equations 30-32 and 30-33).
    let v_app = (ap.v_lt + ap.v_th + ap.v_rt) / n_through;
    let p_lc = probability_lane_change(v_app);
    let split_l = lane_split(ap, e_l1, 1.0, p_lc);

    // Steps 8–9: merge capacity and merge delay (Equations 30-47 through
    // 30-49).
    let d_t1 = if ap.n_through() == 1 {
        // Single-lane approach: through vehicles cannot merge; the inside
        // lane is the only lane, so use the non-merge delay only.
        let c_nm = 1_800.0 * (1.0 + split_l.p_l)
            / (1.0 + split_l.p_l * (e_l1 - 1.0) + split_l.p_l * e_l1);
        incremental_delay(split_l.v_1, c_nm, t_h)
    } else {
        let v_2 = split_l.v_2;
        // Equation 30-47.
        let c_mg = if v_2 > 0.0 {
            let e = (-v_2 * T_LC / 3_600.0).exp();
            v_2 * e / (1.0 - e)
        } else {
            0.0
        };
        // Equations 30-49 and 30-48.
        let v_mg = (split_l.v_1 - ap.v_lt).max(0.0);
        let d_mg = incremental_delay(v_mg, c_mg, t_h);
        // Steps 10–11: non-merge capacity and delay (Equations 30-50 and
        // 30-51).
        let c_nm = 1_800.0 * (1.0 + split_l.p_l)
            / (1.0 + split_l.p_l * (e_l1 - 1.0) + split_l.p_l * e_l1);
        let d_nm = incremental_delay(split_l.v_1, c_nm, t_h);
        // Step 12 (Equation 30-52).
        d_nm.min(d_mg)
    };

    // Step 13: probability of left-turn bay overflow (Equations 30-53 and
    // 30-54). Undivided (no bay): N_qx,lt = 0.
    let n_qx_lt = if ap.left_turn_bay {
        let l_h = stationary_queue_spacing(ap.pct_heavy_veh);
        (ap.n_lt_lanes as f64) * ap.left_bay_storage_ft / l_h
    } else {
        0.0
    };
    let p_ov = probability_left_bay_overflow(ap.v_lt, c_l, n_qx_lt);

    // Step 14: through-vehicle delay due to left turns (Equation 30-31).
    let d_ap_l = if split_l.p_l > 0.0 && (1.0 - p_lt - p_rt) > 0.0 {
        p_ov * d_t1 * (1.0 / split_l.p_l - 1.0) * p_lt / (1.0 - p_lt - p_rt)
    } else {
        0.0
    };

    // ───────────── Delay due to right turns (I_t = 0.00001) ─────────────
    // A right-turn bay removes the through-lane deceleration entirely (the
    // right-turner decelerates in the bay), consistent with the Chapter 18
    // Exhibit 18-13 treatment ("if both turn movements are provided a bay of
    // adequate length, the delay ... can be assumed to equal 0.0").
    let d_ap_r = if ap.right_turn_bay || (1.0 - p_lt - p_rt) <= 0.0 {
        0.0
    } else {
        // Recompute the lane split with P_lc = 1.0 and I_t = 0.00001 to
        // obtain the outside-lane flow rate v_n and P_R (Chapter 30,
        // Section 4 text).
        let split_r = lane_split(ap, e_l1, 0.000_01, 1.0);
        let d_t_r =
            through_delay_per_right_turn(speed_mph, split_r.v_n, split_r.p_r, ap.pct_heavy_veh);
        0.67 * d_t_r * p_rt / (1.0 - p_lt - p_rt)
    };

    AccessPointDelay {
        delay_left_s: d_ap_l,
        delay_right_s: d_ap_r,
        delay_total_s: d_ap_l + d_ap_r,
        prob_inside_lane_blocked: p_ov,
    }
}

/// Through-vehicle delay per right-turn maneuver d_t|r (Equations 30-56
/// through 30-68).
///
/// * `speed_mph` — approaching through-vehicle speed S_f (see the VERIFY-HCM
///   note), mi/h
/// * `v_n` — outside-lane flow rate, veh/h/ln (Equation 30-45)
/// * `p_r` — proportion of right turns in the outside lane P_R (Equation
///   30-42)
/// * `pct_heavy_veh` — percent heavy vehicles for the queue spacing
///   (Equation 30-15), %
fn through_delay_per_right_turn(speed_mph: f64, v_n: f64, p_r: f64, pct_heavy_veh: f64) -> f64 {
    let v_ft_s = FT_S_PER_MPH * speed_mph;
    if v_n <= 0.0 || v_ft_s <= U_RT {
        return 0.0;
    }
    let l_h = stationary_queue_spacing(pct_heavy_veh);
    // Equation 30-59: flow-rate parameter λ (veh/s).
    let q_n = v_n / 3_600.0;
    let inv = 1.0 / q_n - DELTA;
    if inv <= 0.0 {
        // Flow at or above one vehicle per Δ: the platoon never clears.
        return 0.0;
    }
    let lambda = 1.0 / inv;

    // Conditional mean headway between Δ and H (Equations 30-57, 30-62,
    // 30-65): 1/λ + (Δ − H e^(−λ(H−Δ))) / (1 − e^(−λ(H−Δ))).
    let hbar = |h: f64| {
        let e = (-lambda * (h - DELTA)).exp();
        1.0 / lambda + (DELTA - h * e) / (1.0 - e)
    };
    // Equation 30-58: maximum first-vehicle delay headway H_1.
    let h1 = ((v_ft_s - U_RT) / R_D + T_CL + l_h / v_ft_s).max(DELTA);
    // Equation 30-56: minimum speed of the delayed first through vehicle.
    let u_m = (v_ft_s - R_D * (h1 - hbar(h1))).max(U_RT);
    // Equation 30-60: conditional delay to the first through vehicle.
    let d1 = ((v_ft_s - u_m).powi(2) / (2.0 * v_ft_s)) * (1.0 / R_D + 1.0 / R_A);

    // Equations 30-61 through 30-66: delay to subsequent through vehicles,
    // repeated until the delay falls below 0.1 s.
    let mut headways = vec![h1];
    let mut delays = vec![d1];
    let mut d_prev = d1;
    for _ in 0..50 {
        let h_i = d_prev + DELTA; // Equations 30-63 and 30-66.
        let d_i = d_prev - (hbar(h_i) - DELTA); // Equations 30-61 and 30-64.
        if d_i < 0.1 {
            break;
        }
        headways.push(h_i);
        delays.push(d_i);
        d_prev = d_i;
    }

    // Equation 30-68 (general form of Equation 30-67): probability-weighted
    // sum of the per-vehicle delays.
    let mut d_t_r = 0.0;
    let mut prob_prod = 1.0;
    for (i, (&d_i, &h_i)) in delays.iter().zip(headways.iter()).enumerate() {
        prob_prod *= 1.0 - (-lambda * (h_i - DELTA)).exp();
        d_t_r += d_i * prob_prod * (1.0 - p_r).powi(i as i32 + 1);
    }
    d_t_r
}

