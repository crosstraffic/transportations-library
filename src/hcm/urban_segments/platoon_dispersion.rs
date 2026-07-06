//! # Platoon Dispersion and the Arrival Flow Profile (HCM Chapter 30, §3)
//!
//! Implements the Chapter 30, Section 3 platoon-dispersion machinery used to
//! project an upstream signalized intersection's discharge flow profile onto
//! a downstream junction and to compute the two platoon descriptors — the
//! proportion of vehicles arriving during green `P` and the proportion of
//! time a movement is blocked `p_b` (EPUB source `235_Ch30_03.xhtml`,
//! Equations 30-9 through 30-13).
//!
//! The cycle is represented as `C'` one-second time steps (Chapter 30 uses
//! the TRANSYT-7F flow-profile convention). A discharge flow profile is a
//! per-step vector of departure flow rates (veh/step); the Robertson-style
//! dispersion model (Equation 30-9) smooths and lags it into an arrival flow
//! profile at the downstream junction.
//!
//! ## Scope
//!
//! These are the exact Chapter 30 §3 dispersion primitives plus a
//! discharge-profile constructor and an arrival-profile builder. Driving
//! them for a full coordinated system requires the upstream signal's
//! phase durations, saturation flows, and queue service times (from the
//! Chapter 19 coordinated-actuated engine) together with the Section 2
//! origin–destination distribution. Reproducing Example Problem 1's
//! computed `P = 0.493` (only +0.007 above the uniform `g/C = 0.486`)
//! requires that full upstream engine and the O-D matrix — see
//! `docs/hcm/VERIFICATION.md`. The primitives below are unit-tested against
//! the equations directly, and [`UrbanSegment::step_3`](crate::hcm::
//! urban_segments::UrbanSegment) uses them when discharge-profile inputs are
//! supplied, falling back to the uniform / platoon-ratio assumption
//! otherwise.

use serde::{Deserialize, Serialize};

/// HCM Equation 30-11: platoon-dispersion smoothing factor,
/// `F = 1 / (1 + 0.138 t'_R + 0.315 / d_t)`, where `t'_R = t_R / d_t` is the
/// segment running time in time steps.
///
/// * `running_time_s` — segment running time t_R, s
/// * `time_step_s` — time step duration d_t, s/step (Chapter 30 recommends
///   1.0)
pub fn smoothing_factor(running_time_s: f64, time_step_s: f64) -> f64 {
    let dt = time_step_s.max(1e-9);
    let t_r_prime = running_time_s / dt;
    1.0 / (1.0 + 0.138 * t_r_prime + 0.315 / dt)
}

/// HCM Equation 30-12: platoon arrival time to the downstream intersection,
/// `t' = t'_R − 1/F + 1.25` (steps), where `t'_R = t_R / d_t` and `F` is the
/// Equation 30-11 smoothing factor.
///
/// * `running_time_s` — segment running time t_R, s
/// * `time_step_s` — time step duration d_t, s/step
pub fn platoon_arrival_time_steps(running_time_s: f64, time_step_s: f64) -> f64 {
    let dt = time_step_s.max(1e-9);
    let t_r_prime = running_time_s / dt;
    let f = smoothing_factor(running_time_s, time_step_s);
    t_r_prime - 1.0 / f + 1.25
}

/// HCM Equation 30-9/30-10: disperse a discharge flow profile into an
/// arrival flow profile at the downstream intersection,
/// `q'_a|j = F q'_u,i + (1 − F) q'_a|j−1` with `j = i + t'`.
///
/// The profile is cyclic (one average signal cycle of `C' = profile.len()`
/// time steps); the recursion is iterated to its steady-state periodic
/// solution. Total flow is conserved (dispersion redistributes flow across
/// time steps without creating or destroying vehicles).
///
/// * `discharge_profile` — departure flow rate per time step q'_u,i,
///   veh/step, indexed `0..C'`
/// * `running_time_s` — segment running time t_R, s
/// * `time_step_s` — time step duration d_t, s/step
pub fn disperse_profile(
    discharge_profile: &[f64],
    running_time_s: f64,
    time_step_s: f64,
) -> Vec<f64> {
    let n = discharge_profile.len();
    if n == 0 {
        return Vec::new();
    }
    let f = smoothing_factor(running_time_s, time_step_s);
    // Round the platoon arrival time to an integer number of steps for the
    // index lag j = i + t' (Equation 30-10); the fractional lead is carried
    // by the smoothing recursion.
    let shift = platoon_arrival_time_steps(running_time_s, time_step_s)
        .round()
        .rem_euclid(n as f64) as usize;
    let mut arrival = vec![0.0; n];
    // Iterate the periodic recursion to convergence (the geometric memory of
    // the (1 − F) term decays quickly; 100 cycles is ample for F ≥ 1e-3).
    for _ in 0..100 {
        let mut prev = arrival[n - 1];
        let mut next = vec![0.0; n];
        for j in 0..n {
            let i = (j + n - shift) % n;
            next[j] = f * discharge_profile[i] + (1.0 - f) * prev;
            prev = next[j];
        }
        // Convergence check on the wrap-around seam.
        let delta: f64 = next
            .iter()
            .zip(arrival.iter())
            .map(|(a, b)| (a - b).abs())
            .sum();
        arrival = next;
        if delta < 1e-12 {
            break;
        }
    }
    arrival
}

/// Proportion of vehicles arriving during the downstream green,
/// `P = (Σ over green steps q'_a|j) / (Σ over all steps q'_a|j)`.
///
/// The green window may wrap past the end of the cycle; steps are counted
/// modulo `C'`.
///
/// * `arrival_profile` — arrival flow rate per time step, veh/step
/// * `green_start_step` — first green time step (0-based, inclusive)
/// * `green_steps` — number of green time steps (effective green g / d_t)
pub fn proportion_arriving_green(
    arrival_profile: &[f64],
    green_start_step: usize,
    green_steps: usize,
) -> f64 {
    let n = arrival_profile.len();
    if n == 0 {
        return 0.0;
    }
    let total: f64 = arrival_profile.iter().sum();
    if total <= 0.0 {
        return 0.0;
    }
    let green: f64 = (0..green_steps.min(n))
        .map(|k| arrival_profile[(green_start_step + k) % n])
        .sum();
    (green / total).clamp(0.0, 1.0)
}

/// HCM Equation 30-13: proportion of time a movement is blocked by a
/// platoon, `p_b = t'_p d_t / C`.
///
/// * `blocked_period_steps` — blocked period duration t'_p, steps
/// * `time_step_s` — time step duration d_t, s/step
/// * `cycle_s` — cycle length C, s
pub fn proportion_time_blocked(blocked_period_steps: f64, time_step_s: f64, cycle_s: f64) -> f64 {
    if cycle_s <= 0.0 {
        return 0.0;
    }
    (blocked_period_steps * time_step_s / cycle_s).clamp(0.0, 1.0)
}

/// Critical platoon flow rate `q_c = 3,600 / t_c` (veh/h), the arrival flow
/// rate above which platoon headways are too short for a minor movement to
/// enter or cross (Chapter 30, Section 3, "Proportion of Time Blocked"). The
/// blocked period is the span of the arrival flow profile whose rate exceeds
/// this threshold.
///
/// * `critical_headway_s` — critical headway t_c of the minor movement
///   (Chapter 20), s
pub fn critical_platoon_flow_rate(critical_headway_s: f64) -> f64 {
    if critical_headway_s <= 0.0 {
        return f64::INFINITY;
    }
    3_600.0 / critical_headway_s
}

/// A single upstream signalized movement's discharge flow profile over one
/// average cycle, used to build the projected arrival flow profile
/// (Chapter 30, Section 3, "Discharge Flow Profile").
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct MovementDischarge {
    /// Discharge (adjusted) volume entering the segment from this movement,
    /// veh/h. Set equal to the movement capacity when demand exceeds it
    /// (Chapter 30, Section 2).
    pub discharge_volume_veh_h: f64,
    /// Saturation flow rate of the movement's lane group, veh/h (total across
    /// its lanes). The queue discharges at this rate during the queue
    /// service time.
    pub saturation_flow_veh_h: f64,
    /// Effective green start time relative to system time 0.0, s.
    pub green_start_s: f64,
    /// Effective green duration g, s.
    pub green_duration_s: f64,
    /// Queue service time g_s, s: the time from the start of green until the
    /// queue that formed during red clears. During this interval the
    /// discharge rate is the saturation flow rate; afterward it is the
    /// arrival (adjusted discharge) rate (Chapter 30, Section 3).
    pub queue_service_time_s: f64,
}

impl MovementDischarge {
    /// Construct the per-step discharge flow profile q'_u,i (veh/step) over a
    /// cycle of `cycle_steps` time steps of `time_step_s` each. During the
    /// queue service time the rate is the saturation flow rate; for the rest
    /// of green it is the adjusted discharge rate so that the profile
    /// integrates to the discharge volume; outside green it is zero.
    pub fn to_profile(&self, cycle_steps: usize, time_step_s: f64) -> Vec<f64> {
        let n = cycle_steps;
        let mut profile = vec![0.0; n];
        if n == 0 || self.green_duration_s <= 0.0 {
            return profile;
        }
        let veh_per_cycle = self.discharge_volume_veh_h * (n as f64 * time_step_s) / 3_600.0;
        let sat_per_step = self.saturation_flow_veh_h * time_step_s / 3_600.0;
        let g_s = self.queue_service_time_s.clamp(0.0, self.green_duration_s);
        let start = (self.green_start_s / time_step_s).round() as isize;
        // Discretize the green and queue-service intervals to whole time
        // steps first, then set the post-queue-service rate from the integer
        // step counts so the profile integrates to the discharge volume
        // exactly (no continuous-vs-discrete rounding drift).
        let g_steps = ((self.green_duration_s / time_step_s).round() as isize).max(0);
        let gs_steps = ((g_s / time_step_s).round() as isize).clamp(0, g_steps);
        let queue_veh = sat_per_step * gs_steps as f64;
        let post_steps = g_steps - gs_steps;
        let post_rate = if post_steps > 0 {
            (veh_per_cycle - queue_veh).max(0.0) / post_steps as f64
        } else {
            0.0
        };
        for k in 0..g_steps {
            let idx = (start + k).rem_euclid(n as isize) as usize;
            profile[idx] += if k < gs_steps { sat_per_step } else { post_rate };
        }
        profile
    }
}

/// Build the combined arrival flow profile at a downstream junction by
/// dispersing each upstream movement's discharge profile (Equation 30-9)
/// and summing, then optionally adding a uniform (midblock access-point)
/// arrival component (Chapter 30, Section 3, "Arrival Flow Profile" —
/// midsegment arrivals "are assumed to have a uniform arrival flow profile").
///
/// * `movements` — upstream discharge profiles contributing to the subject
///   arrival flow
/// * `uniform_volume_veh_h` — midblock (access-point) volume entering the
///   arrival flow uniformly across the cycle, veh/h
/// * `cycle_steps` — number of time steps in the cycle C'
/// * `time_step_s` — time step duration d_t, s/step
/// * `running_time_s` — segment running time t_R, s
pub fn combined_arrival_profile(
    movements: &[MovementDischarge],
    uniform_volume_veh_h: f64,
    cycle_steps: usize,
    time_step_s: f64,
    running_time_s: f64,
) -> Vec<f64> {
    let n = cycle_steps;
    let mut arrival = vec![0.0; n];
    if n == 0 {
        return arrival;
    }
    for m in movements {
        let disch = m.to_profile(n, time_step_s);
        let dispersed = disperse_profile(&disch, running_time_s, time_step_s);
        for (a, d) in arrival.iter_mut().zip(dispersed.iter()) {
            *a += d;
        }
    }
    if uniform_volume_veh_h > 0.0 {
        let per_step = uniform_volume_veh_h * time_step_s / 3_600.0;
        for a in arrival.iter_mut() {
            *a += per_step;
        }
    }
    arrival
}
