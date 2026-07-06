//! Shared control-delay building blocks for HCM Chapters 19–23.
//!
//! Implements the uniform / incremental / initial-queue delay terms of the
//! signalized-intersection method (HCM Chapter 19) and the unsignalized
//! control-delay family shared by TWSC (Chapter 20), AWSC (Chapter 21),
//! and roundabouts (Chapter 22).
//!
//! All delays are in s/veh, capacities in veh/h, analysis period `t_h` in
//! hours (0.25 h for a 15-min period).

// ═══════════════════════════════════════════════════════════════════════════════
// Constants (HCM Chapter 19, Step 8, and Equation 19-6 discussion)
// ═══════════════════════════════════════════════════════════════════════════════

/// Incremental delay factor k recommended for pretimed phases, coordinated
/// phases, and phases set to recall-to-maximum (HCM Chapter 19, Step 8,
/// Part C: "A factor value of 0.50 is recommended for pretimed phases,
/// coordinated phases, and phases set to 'recall-to-maximum.'").
pub const K_PRETIMED: f64 = 0.50;

/// Minimum permitted incremental delay factor k for actuated phases
/// (HCM Equation 19-23 lower bound: k_min >= 0.04).
pub const K_MIN_LOWER_BOUND: f64 = 0.04;

/// Upstream filtering adjustment factor I for an isolated intersection,
/// i.e., one 0.6 mi or more from the nearest upstream signalized
/// intersection (HCM Chapter 19, Equation 19-6 discussion).
pub const I_ISOLATED: f64 = 1.0;

/// Lower bound of the upstream filtering adjustment factor
/// (HCM Equation 19-6: I >= 0.090).
pub const I_MIN: f64 = 0.090;

// ═══════════════════════════════════════════════════════════════════════════════
// Signalized intersection delay terms (HCM Chapter 19)
// ═══════════════════════════════════════════════════════════════════════════════

/// HCM Equation 19-21: flow ratio `y = min(1, X) * g/C`.
///
/// * `x` — volume-to-capacity ratio X (unitless)
/// * `g_over_c` — effective green ratio g/C (unitless)
pub fn flow_ratio(x: f64, g_over_c: f64) -> f64 {
    x.min(1.0) * g_over_c
}

/// HCM Equation 19-20: progression adjustment factor
///
/// `PF = (1 - P)/(1 - g/C) * (1 - y)/(1 - min(1, X) P) * [1 + y (1 - P C/g)/(1 - g/C)]`
///
/// * `p` — proportion of vehicles arriving during the green indication (decimal)
/// * `g_over_c` — effective green ratio g/C (unitless)
/// * `x` — volume-to-capacity ratio X (unitless)
///
/// For random arrivals (P = g/C) the factor equals 1.0 at low degrees of
/// saturation.
///
/// At `g/C = 1.0` (no red interval) `term1` and `term3` divide by
/// `1 - g/C = 0`. Physically this limit is moot: [`uniform_delay`] (the
/// only place PF is used) tends to 0 as `g/C -> 1` regardless of PF, so PF's
/// value cannot affect the resulting delay. This function returns `1.0`
/// (no adjustment) for `g_over_c >= 1.0` rather than propagate `NaN`/`inf`,
/// matching the guard already used at the chapter23 ramp-terminal call site
/// (`if g_over_c < 1.0 { progression_factor(..) } else { 1.0 }`), now made
/// intrinsic to the function so every caller gets the same behavior.
pub fn progression_factor(p: f64, g_over_c: f64, x: f64) -> f64 {
    if g_over_c >= 1.0 {
        return 1.0;
    }
    let y = flow_ratio(x, g_over_c);
    let term1 = (1.0 - p) / (1.0 - g_over_c);
    let term2 = (1.0 - y) / (1.0 - x.min(1.0) * p);
    let term3 = 1.0 + y * (1.0 - p / g_over_c) / (1.0 - g_over_c);
    term1 * term2 * term3
}

/// HCM Equation 19-19: uniform delay (progression-adjusted form)
///
/// `d1 = PF * [0.5 C (1 - g/C)^2] / [1 - min(1, X) g/C]`
///
/// * `cycle_s` — cycle length C, s
/// * `green_s` — effective green time g, s
/// * `x` — volume-to-capacity ratio X (unitless)
/// * `pf` — progression adjustment factor PF (Equation 19-20); 1.0 for
///   random arrivals
///
/// Returns uniform delay d1, s/veh. Valid for a lane group serving one
/// traffic movement with no permitted movements (see HCM Chapter 19, Step 8,
/// Part A).
///
/// At `g/C = 1.0` there is no red interval. For `X >= 1`, `min(1, X) = 1`
/// so the denominator `1 - min(1, X) g/C` also goes to 0 as `g/C -> 1`,
/// together with the numerator `0.5 C (1 - g/C)^2`, producing `0/0` (`NaN`)
/// rather than the true limit. Taking `g/C -> 1^-` with `X >= 1` fixed:
/// `0.5 C (1 - g/C)^2 / (1 - g/C) = 0.5 C (1 - g/C) -> 0`. So `d1 -> 0` as
/// `g/C -> 1`, independent of X and PF (no red time means no cyclical
/// queuing, so there is nothing for arrival pattern to modulate). For
/// `X < 1` the denominator instead tends to `1 - X > 0` and the unguarded
/// formula already evaluates the same correct `0/(1 - X) = 0` limit, so
/// this guard changes no observable behavior there — it only replaces the
/// `X >= 1` singularity with its limit.
pub fn uniform_delay(cycle_s: f64, green_s: f64, x: f64, pf: f64) -> f64 {
    let g_over_c = green_s / cycle_s;
    if g_over_c >= 1.0 {
        return 0.0;
    }
    pf * (0.5 * cycle_s * (1.0 - g_over_c).powi(2)) / (1.0 - x.min(1.0) * g_over_c)
}

/// HCM Equation 19-6: upstream filtering adjustment factor for nonisolated
/// intersections
///
/// `I = 1.0 - 0.91 Xu^2.68 >= 0.090`
///
/// * `x_u` — weighted volume-to-capacity ratio of all upstream movements
///   contributing to the subject movement group (capped at 1.0 per the
///   Chapter 19 text)
///
/// Use [`I_ISOLATED`] (1.0) for isolated intersections.
pub fn upstream_filtering_factor(x_u: f64) -> f64 {
    let x_u = x_u.min(1.0).max(0.0);
    (1.0 - 0.91 * x_u.powf(2.68)).max(I_MIN)
}

/// HCM Equation 19-23: minimum incremental delay factor for an actuated phase
///
/// `k_min = -0.375 + 0.354 PT - 0.0910 PT^2 + 0.00889 PT^3 >= 0.04`
///
/// * `passage_time_s` — passage time setting PT, s
pub fn incremental_delay_factor_min(passage_time_s: f64) -> f64 {
    let pt = passage_time_s;
    (-0.375 + 0.354 * pt - 0.0910 * pt * pt + 0.00889 * pt.powi(3)).max(K_MIN_LOWER_BOUND)
}

/// HCM Equation 19-22: incremental delay factor for an actuated phase
///
/// `k = (1 - 2 k_min)(v/c_a - 0.5) + k_min <= 0.50`
///
/// * `v_over_ca` — ratio of demand flow rate to available capacity c_a
///   (Equation 19-24)
/// * `k_min` — minimum incremental delay factor (Equation 19-23)
///
/// The result is clamped to `[k_min, 0.50]`. Use [`K_PRETIMED`] (0.50) for
/// pretimed and coordinated phases.
pub fn incremental_delay_factor_actuated(v_over_ca: f64, k_min: f64) -> f64 {
    ((1.0 - 2.0 * k_min) * (v_over_ca - 0.5) + k_min)
        .max(k_min)
        .min(K_PRETIMED)
}

/// HCM Equation 19-26: incremental delay for a signalized lane group
///
/// `d2 = 900 T [ (X_A - 1) + sqrt((X_A - 1)^2 + 8 k I X_A / (c_A T)) ]`
///
/// * `t_h` — analysis period duration T, h (0.25 h for 15 min)
/// * `x` — average volume-to-capacity ratio X_A = v/c_A (Equation 19-27)
/// * `capacity` — average lane group capacity c_A, veh/h
/// * `k` — incremental delay factor (0.04–0.50; [`K_PRETIMED`] for pretimed)
/// * `i_factor` — upstream filtering adjustment factor I (Equation 19-6;
///   [`I_ISOLATED`] for isolated intersections)
///
/// Returns incremental delay d2, s/veh. Valid for all values of X_A,
/// including highly oversaturated lane groups (HCM Chapter 19, Step 8,
/// Part D).
pub fn incremental_delay_signalized(
    t_h: f64,
    x: f64,
    capacity: f64,
    k: f64,
    i_factor: f64,
) -> f64 {
    900.0 * t_h
        * ((x - 1.0) + ((x - 1.0).powi(2) + 8.0 * k * i_factor * x / (capacity * t_h)).sqrt())
}

/// Generic incremental delay term in the HCM Chapter 19 form
/// (HCM Equation 19-26). Alias of [`incremental_delay_signalized`] for use
/// by chapters that reuse the same algebraic form.
///
/// Note the unsignalized control-delay family (HCM Equations 20-61, 21-30,
/// and 22-17) uses the same 900T[(x-1) + sqrt(...)] structure with the
/// radicand written as `(3,600/c) x / (450 T)`, which is algebraically
/// `8 x / (c T)` — i.e., this equation with `k * I` fixed at 1.0.
pub fn incremental_delay(t_h: f64, x: f64, capacity: f64, k: f64, i_factor: f64) -> f64 {
    incremental_delay_signalized(t_h, x, capacity, k, i_factor)
}

/// HCM Equations 19-44 through 19-49: initial queue delay d3.
///
/// `d3 = 3,600/(v T) * [ t_A (Q_b + Q_e - Q_eo)/2 + (Q_e^2 - Q_eo^2)/(2 c_A) - Q_b^2/(2 c_A) ]`  (Eq. 19-44)
///
/// with `Q_e = Q_b + t_A (v - c_A)` (Eq. 19-45) and
/// * if v >= c_A: `Q_eo = T (v - c_A)` (Eq. 19-46), `t_A = T` (Eq. 19-47)
/// * if v <  c_A: `Q_eo = 0.0 veh` (Eq. 19-48), `t_A = min(Q_b/(c_A - v), T)` (Eq. 19-49)
///
/// * `queue_initial_veh` — initial queue at the start of the analysis
///   period Q_b, veh
/// * `v` — demand flow rate, veh/h
/// * `capacity` — average lane group capacity c_A, veh/h
/// * `t_h` — analysis period duration T, h
///
/// Returns initial queue delay d3, s/veh. Per HCM Chapter 19, Step 8,
/// Part B, d3 = 0.0 s/veh when there is no initial queue (Q_b = 0).
pub fn initial_queue_delay(queue_initial_veh: f64, v: f64, capacity: f64, t_h: f64) -> f64 {
    let qb = queue_initial_veh;
    if qb <= 0.0 || v <= 0.0 || t_h <= 0.0 {
        return 0.0;
    }
    let (qeo, ta) = if v >= capacity {
        (t_h * (v - capacity), t_h) // Eq. 19-46, 19-47
    } else {
        (0.0, (qb / (capacity - v)).min(t_h)) // Eq. 19-48, 19-49
    };
    let qe = qb + ta * (v - capacity); // Eq. 19-45
    3_600.0 / (v * t_h)
        * (ta * (qb + qe - qeo) / 2.0 + (qe * qe - qeo * qeo) / (2.0 * capacity)
            - qb * qb / (2.0 * capacity))
}

/// HCM Equation 19-45: residual queue at the end of the analysis period
/// `Qe = Qb + t_A (v - c_A)`, with `t_A`/`Qeo` from Equations 19-46 through
/// 19-49 as in [`initial_queue_delay`].
///
/// HCM Chapter 17, Section 3, "Facility Evaluation": for a multi-period
/// analysis, "the initial queue input value for the next analysis period
/// is set equal to the residual queue output for the current analysis
/// period" — i.e., this function's return value is the `queue_initial_veh`
/// (Qb) to pass to [`initial_queue_delay`] and to this function itself for
/// the next chronological analysis period. Chapter 29, Section 3 describes
/// the same hand-off for the multiple-time-period/spillback technique
/// ("the residual queue from one subperiod becomes the initial queue for
/// the next subperiod").
///
/// * `queue_initial_veh` — initial queue at the start of the analysis
///   period Qb, veh
/// * `v` — demand flow rate, veh/h
/// * `capacity` — average lane group capacity c_A, veh/h
/// * `t_h` — analysis period duration T, h
///
/// Returns Qe, veh (>= 0). When `v < c_A` and the initial queue fully
/// dissipates within the period (`t_A < T`), Qe = 0 by construction. When
/// there is no initial queue and `v < c_A`, Qe = 0 (no queue forms). When
/// `v >= c_A`, Qe = Qb + T(v - c_A) regardless of whether Qb was 0 (a new
/// queue forms/grows during an oversaturated period even without a
/// carried-in queue).
pub fn queue_end_of_period(queue_initial_veh: f64, v: f64, capacity: f64, t_h: f64) -> f64 {
    let qb = queue_initial_veh.max(0.0);
    if v <= 0.0 || t_h <= 0.0 {
        return qb;
    }
    let t_a = if v >= capacity {
        t_h // Eq. 19-47
    } else if capacity > v {
        (qb / (capacity - v)).min(t_h) // Eq. 19-49
    } else {
        t_h
    };
    (qb + t_a * (v - capacity)).max(0.0)
}

/// HCM Equation 19-18: lane group control delay `d = d1 + d2 + d3`.
///
/// * `d1` — uniform delay, s/veh (Equation 19-19)
/// * `d2` — incremental delay, s/veh (Equation 19-26)
/// * `d3` — initial queue delay, s/veh (Equations 19-44 through 19-49)
pub fn control_delay_signalized(d1: f64, d2: f64, d3: f64) -> f64 {
    d1 + d2 + d3
}

// ═══════════════════════════════════════════════════════════════════════════════
// Unsignalized control delay family (HCM Chapters 20, 21, 22)
// ═══════════════════════════════════════════════════════════════════════════════

/// HCM Equation 20-61: TWSC movement control delay
///
/// `d = 3,600/c_m,x + 900 T [ v_x/c_m,x - 1 + sqrt((v_x/c_m,x - 1)^2
///      + (3,600/c_m,x)(v_x/c_m,x)/(450 T)) ] + 5`
///
/// * `volume` — movement demand flow rate v_x, veh/h
/// * `capacity` — movement capacity c_m,x, veh/h
/// * `t_h` — analysis period T, h (0.25 h for 15 min)
///
/// Returns control delay, s/veh. The constant 5 s/veh accounts for
/// deceleration to and acceleration from the stop. The model assumes
/// demand < capacity for the analysis period; for degrees of saturation
/// above about 0.9 the result is sensitive to `t_h` (HCM Chapter 20,
/// Step 11a). The same algebraic form is used by AWSC (Equation 21-30,
/// see [`control_delay_awsc`]) and roundabouts (Equation 22-17, see
/// [`control_delay_roundabout`]).
pub fn control_delay_unsignalized(volume: f64, capacity: f64, t_h: f64) -> f64 {
    let x = volume / capacity;
    3_600.0 / capacity
        + 900.0
            * t_h
            * (x - 1.0 + ((x - 1.0).powi(2) + (3_600.0 / capacity) * x / (450.0 * t_h)).sqrt())
        + 5.0
}

/// HCM Equation 22-17: roundabout lane control delay
///
/// `d = 3,600/c + 900 T [ x - 1 + sqrt((x - 1)^2 + (3,600/c) x / (450 T)) ]
///      + 5 * min(x, 1)`
///
/// * `volume` — lane demand flow rate, veh/h
/// * `capacity` — lane capacity c, veh/h
/// * `t_h` — analysis period T, h
///
/// Identical to Equation 20-61 except the +5 s/veh term is scaled by
/// `min(x, 1)` to reflect YIELD control (vehicles need not stop when
/// there is no conflict).
pub fn control_delay_roundabout(volume: f64, capacity: f64, t_h: f64) -> f64 {
    let x = volume / capacity;
    3_600.0 / capacity
        + 900.0
            * t_h
            * (x - 1.0 + ((x - 1.0).powi(2) + (3_600.0 / capacity) * x / (450.0 * t_h)).sqrt())
        + 5.0 * x.min(1.0)
}

/// HCM Equation 21-30: AWSC lane control delay
///
/// `d = t_s + 900 T [ (x - 1) + sqrt((x - 1)^2 + h_d x / (450 T)) ] + 5`
///
/// * `service_time_s` — service time t_s, s
/// * `departure_headway_s` — departure headway h_d, s
/// * `x` — degree of utilization `x = v h_d / 3,600` (unitless)
/// * `t_h` — analysis period T, h
pub fn control_delay_awsc(
    service_time_s: f64,
    departure_headway_s: f64,
    x: f64,
    t_h: f64,
) -> f64 {
    service_time_s
        + 900.0
            * t_h
            * ((x - 1.0) + ((x - 1.0).powi(2) + departure_headway_s * x / (450.0 * t_h)).sqrt())
        + 5.0
}

/// HCM Equation 19-28 (signalized approach delay) / Equation 20-64 (TWSC
/// approach delay): flow-rate-weighted average control delay
///
/// `d_A = Σ(d_i v_i) / Σ(v_i)`
///
/// * `delays_and_volumes` — (control delay s/veh, flow rate veh/h) pairs
///
/// Returns the aggregated delay in s/veh (0.0 if total volume is zero).
/// The same weighted-average form aggregates approaches to the
/// intersection level (Equations 19-29 and 20-65).
pub fn aggregate_control_delay(delays_and_volumes: &[(f64, f64)]) -> f64 {
    let total_v: f64 = delays_and_volumes.iter().map(|(_, v)| v).sum();
    if total_v <= 0.0 {
        return 0.0;
    }
    delays_and_volumes.iter().map(|(d, v)| d * v).sum::<f64>() / total_v
}

// ═══════════════════════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    const T: f64 = 0.25;

    #[test]
    fn test_incremental_delay_x_to_zero_limit() {
        // As x -> 0, d2 -> 900T[(-1) + sqrt(1 + 0)] = 0
        let d2 = incremental_delay_signalized(T, 0.0, 1000.0, K_PRETIMED, I_ISOLATED);
        assert!(d2.abs() < 1e-9, "d2 at x=0 should be 0, got {d2}");
        let d2_small = incremental_delay_signalized(T, 1e-6, 1000.0, K_PRETIMED, I_ISOLATED);
        assert!(d2_small < 0.01);
    }

    #[test]
    fn test_incremental_delay_continuity_at_x_1() {
        // Continuous through x = 1: d2(1) = 900T sqrt(8kI/(cT))
        let c = 800.0;
        let expected = 900.0 * T * (8.0 * K_PRETIMED * I_ISOLATED / (c * T)).sqrt();
        let at_one = incremental_delay_signalized(T, 1.0, c, K_PRETIMED, I_ISOLATED);
        assert!((at_one - expected).abs() < 1e-9);
        let below = incremental_delay_signalized(T, 1.0 - 1e-8, c, K_PRETIMED, I_ISOLATED);
        let above = incremental_delay_signalized(T, 1.0 + 1e-8, c, K_PRETIMED, I_ISOLATED);
        assert!((above - below).abs() < 1e-3);
    }

    #[test]
    fn test_incremental_delay_monotonic_in_x() {
        let mut prev = -1.0;
        for i in 0..40 {
            let x = i as f64 * 0.05; // 0.0 .. 2.0
            let d2 = incremental_delay_signalized(T, x, 900.0, K_PRETIMED, I_ISOLATED);
            assert!(d2 >= prev, "d2 not monotonic at x={x}");
            prev = d2;
        }
    }

    #[test]
    fn test_uniform_delay_limits() {
        // x -> 0 with PF = 1: d1 = 0.5 C (1 - g/C)^2
        let d1 = uniform_delay(100.0, 40.0, 0.0, 1.0);
        assert!((d1 - 0.5 * 100.0 * 0.6 * 0.6).abs() < 1e-9);
        // Monotonic increasing in x up to x = 1
        let lo = uniform_delay(100.0, 40.0, 0.3, 1.0);
        let hi = uniform_delay(100.0, 40.0, 0.9, 1.0);
        assert!(hi > lo);
        // Capped at x = 1: x > 1 gives same d1 as x = 1
        let at1 = uniform_delay(100.0, 40.0, 1.0, 1.0);
        let above1 = uniform_delay(100.0, 40.0, 1.5, 1.0);
        assert!((at1 - above1).abs() < 1e-12);
    }

    #[test]
    fn test_uniform_delay_g_over_c_one_exact() {
        // g/C = 1.0 exactly (no red interval): the unguarded formula would
        // divide 0 by 0 for X >= 1 (min(1,X)*g/C = 1 = g/C). The guard
        // returns the analytic limit, 0, instead of NaN.
        let d1_at_capacity = uniform_delay(100.0, 100.0, 1.0, 1.0);
        assert_eq!(d1_at_capacity, 0.0);
        let d1_oversaturated = uniform_delay(100.0, 100.0, 1.8, 1.0);
        assert_eq!(d1_oversaturated, 0.0);
        // X < 1 at g/C = 1.0 is not actually singular (denominator is
        // 1 - X > 0), but the guard must still return the same 0 the
        // unguarded formula would produce.
        let d1_undersaturated = uniform_delay(100.0, 100.0, 0.5, 1.0);
        assert_eq!(d1_undersaturated, 0.0);
    }

    #[test]
    fn test_uniform_delay_g_over_c_near_one_continuity() {
        // g/C = 0.9999 with X >= 1: d1 should be small and finite, and
        // approach the g/C = 1.0 guarded value (0) as g/C -> 1.
        let c = 100.0;
        let g_near = 0.9999 * c;
        let d1_near = uniform_delay(c, g_near, 1.5, 1.0);
        assert!(d1_near.is_finite(), "d1 should be finite near g/C=1, got {d1_near}");
        assert!(d1_near > 0.0);
        assert!(d1_near < 0.01, "d1 near g/C=1 should be tiny, got {d1_near}");
        // Closer to 1.0 => closer to the limit of 0 (monotone approach in
        // this regime: d1 ~ 0.5 C (1 - g/C) for X >= 1 near g/C = 1).
        let g_closer = 0.999_99 * c;
        let d1_closer = uniform_delay(c, g_closer, 1.5, 1.0);
        assert!(d1_closer < d1_near);
        let d1_at_one = uniform_delay(c, c, 1.5, 1.0);
        assert_eq!(d1_at_one, 0.0);
    }

    #[test]
    fn test_uniform_delay_x_above_one_at_g_over_c_one() {
        // X > 1 (oversaturated) with g/C = 1.0: still 0, matching the
        // guard's X-independence (no red time => no uniform delay term,
        // regardless of how oversaturated the lane group is).
        let d1 = uniform_delay(90.0, 90.0, 3.0, 1.0);
        assert_eq!(d1, 0.0);
        // PF's value must not matter either once guarded.
        let d1_pf = uniform_delay(90.0, 90.0, 3.0, 2.5);
        assert_eq!(d1_pf, 0.0);
    }

    #[test]
    fn test_progression_factor_g_over_c_one_returns_one() {
        // g/C = 1.0: term1 and term3 would divide by 0. Guard returns 1.0
        // (no adjustment), matching the pre-existing external guard in
        // chapter23 ramp_terminals.rs.
        assert_eq!(progression_factor(0.4, 1.0, 0.8), 1.0);
        assert_eq!(progression_factor(0.4, 1.0, 1.5), 1.0);
        // g/C slightly above 1.0 (invalid input, defensive)
        assert_eq!(progression_factor(0.4, 1.000_001, 0.8), 1.0);
    }

    #[test]
    fn test_progression_factor_random_arrivals() {
        // For random arrivals, P = g/C; at x = 0, y = 0 and PF = 1
        let g_c = 0.4;
        let pf = progression_factor(g_c, g_c, 0.0);
        assert!((pf - 1.0).abs() < 1e-9);
    }

    #[test]
    fn test_upstream_filtering_factor() {
        // Eq 19-6: I = 1 - 0.91 Xu^2.68 >= 0.090
        assert!((upstream_filtering_factor(0.0) - 1.0).abs() < 1e-9);
        assert!((upstream_filtering_factor(1.0) - 0.09).abs() < 1e-9);
        assert!(upstream_filtering_factor(0.5) < 1.0);
        assert!(upstream_filtering_factor(0.5) > 0.09);
        // Xu capped at 1.0
        assert!((upstream_filtering_factor(2.0) - upstream_filtering_factor(1.0)).abs() < 1e-12);
    }

    #[test]
    fn test_incremental_delay_factor() {
        // Eq 19-23 lower bound
        assert!((incremental_delay_factor_min(0.0) - 0.04).abs() < 1e-12);
        // Eq 19-22: at v/ca = 0.5, k = k_min; capped at 0.50
        let kmin = 0.1;
        assert!((incremental_delay_factor_actuated(0.5, kmin) - kmin).abs() < 1e-12);
        assert!((incremental_delay_factor_actuated(1.0, kmin) - 0.5).abs() < 1e-12);
        assert!(incremental_delay_factor_actuated(2.0, kmin) <= 0.5);
        assert!(incremental_delay_factor_actuated(0.0, kmin) >= kmin);
    }

    #[test]
    fn test_initial_queue_delay_base_case() {
        // Qb = 0 => d3 = 0 (HCM Ch. 19, Step 8, Part B)
        assert_eq!(initial_queue_delay(0.0, 500.0, 600.0, T), 0.0);
    }

    #[test]
    fn test_initial_queue_delay_undersaturated_clears() {
        // v < cA with small Qb: queue clears within T, d3 > 0
        let d3 = initial_queue_delay(10.0, 400.0, 800.0, T);
        assert!(d3 > 0.0);
        // Larger initial queue -> more delay
        let d3_big = initial_queue_delay(20.0, 400.0, 800.0, T);
        assert!(d3_big > d3);
    }

    #[test]
    fn test_initial_queue_delay_oversaturated() {
        // v >= cA: tA = T, Qe = Qb + T(v - cA), Qeo = T(v - cA)
        let (qb, v, ca) = (10.0, 900.0, 600.0);
        let qe = qb + T * (v - ca);
        let qeo = T * (v - ca);
        let expected = 3_600.0 / (v * T)
            * (T * (qb + qe - qeo) / 2.0 + (qe * qe - qeo * qeo) / (2.0 * ca)
                - qb * qb / (2.0 * ca));
        let d3 = initial_queue_delay(qb, v, ca, T);
        assert!((d3 - expected).abs() < 1e-9);
        assert!(d3 > 0.0);
    }

    #[test]
    fn test_queue_end_of_period_no_initial_queue_undersaturated() {
        // Qb = 0, v < cA => no queue forms, Qe = 0.
        assert_eq!(queue_end_of_period(0.0, 400.0, 800.0, T), 0.0);
    }

    #[test]
    fn test_queue_end_of_period_clears_within_period() {
        // Small Qb, v < cA, queue fully dissipates before T => Qe = 0.
        let qe = queue_end_of_period(10.0, 400.0, 800.0, T);
        assert_eq!(qe, 0.0);
    }

    #[test]
    fn test_queue_end_of_period_large_initial_queue_undersaturated() {
        // Large Qb, v < cA, but t_A = Qb/(cA-v) > T => queue does not fully
        // clear; Qe = Qb + T(v - cA) > 0.
        let (qb, v, ca) = (500.0, 400.0, 800.0);
        let expected = qb + T * (v - ca);
        let qe = queue_end_of_period(qb, v, ca, T);
        assert!((qe - expected).abs() < 1e-9);
        assert!(qe > 0.0);
    }

    #[test]
    fn test_queue_end_of_period_oversaturated_matches_eq_19_45() {
        // v >= cA: Qe = Qb + T(v - cA), independent of whether Qb = 0.
        let (v, ca) = (900.0, 600.0);
        let qe_no_initial = queue_end_of_period(0.0, v, ca, T);
        assert!((qe_no_initial - T * (v - ca)).abs() < 1e-9);
        let qe_with_initial = queue_end_of_period(20.0, v, ca, T);
        assert!((qe_with_initial - (20.0 + T * (v - ca))).abs() < 1e-9);
        assert!(qe_with_initial > qe_no_initial);
    }

    #[test]
    fn test_queue_end_of_period_monotonic_in_initial_queue() {
        let mut prev = 0.0;
        for i in 0..10 {
            let qb = i as f64 * 50.0;
            let qe = queue_end_of_period(qb, 900.0, 600.0, T);
            assert!(qe >= prev, "Qe not monotonic in Qb at qb={qb}");
            prev = qe;
        }
    }

    #[test]
    fn test_control_delay_unsignalized_zero_volume_limit() {
        // v -> 0: x -> 0, d -> 3600/c + 900T[-1 + 1] + 5 = 3600/c + 5
        let c = 720.0;
        let d = control_delay_unsignalized(0.0, c, T);
        assert!((d - (3_600.0 / c + 5.0)).abs() < 1e-9);
    }

    #[test]
    fn test_control_delay_unsignalized_monotonic() {
        let c = 600.0;
        let mut prev = 0.0;
        for i in 1..=24 {
            let v = i as f64 * 50.0; // up to v/c = 2
            let d = control_delay_unsignalized(v, c, T);
            assert!(d > prev, "delay not monotonic at v={v}");
            prev = d;
        }
    }

    #[test]
    fn test_control_delay_roundabout_yield_term() {
        // At x = 0 the +5 term vanishes: d = 3600/c
        let c = 1_000.0;
        let d = control_delay_roundabout(0.0, c, T);
        assert!((d - 3_600.0 / c).abs() < 1e-9);
        // For x >= 1 the term is exactly +5 vs. the same-family TWSC form
        let d_r = control_delay_roundabout(1_000.0, c, T);
        let d_t = control_delay_unsignalized(1_000.0, c, T);
        assert!((d_r - d_t).abs() < 1e-9);
    }

    #[test]
    fn test_control_delay_awsc_zero_utilization() {
        // x = 0: d = ts + 900T[-1 + 1] + 5 = ts + 5
        let d = control_delay_awsc(3.0, 5.0, 0.0, T);
        assert!((d - 8.0).abs() < 1e-9);
    }

    #[test]
    fn test_unsignalized_radicand_equals_generic_with_ki_one() {
        // (3600/c) x / (450 T) == 8 x / (c T): the unsignalized family is the
        // Ch. 19 incremental form with k*I = 1 (plus the 3600/c and +5 terms).
        let (v, c) = (500.0, 700.0);
        let x = v / c;
        let d_unsig = control_delay_unsignalized(v, c, T);
        let d_generic = 3_600.0 / c + incremental_delay(T, x, c, 1.0, 1.0) + 5.0;
        assert!((d_unsig - d_generic).abs() < 1e-9);
    }

    #[test]
    fn test_aggregate_control_delay() {
        // Eq 19-28 / 20-64 weighted average
        let d = aggregate_control_delay(&[(10.0, 100.0), (30.0, 300.0)]);
        assert!((d - 25.0).abs() < 1e-9);
        assert_eq!(aggregate_control_delay(&[]), 0.0);
    }

    #[test]
    fn test_control_delay_signalized_sum() {
        assert!((control_delay_signalized(10.0, 5.0, 2.5) - 17.5).abs() < 1e-12);
    }
}
