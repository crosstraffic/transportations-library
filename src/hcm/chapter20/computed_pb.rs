//! # Computed proportion of time blocked from upstream signals (HCM Chapter 30, §3)
//!
//! Derives the Chapter 20 [`PlatoonBlockage`] proportions p_b,x from a
//! description of the coordinated signalized intersections upstream of a TWSC
//! intersection, instead of taking them as analyst inputs. This closes the
//! deferred item noted in `docs/hcm/procedures/chapter20.md` and
//! `chapter18-computed.md`: Chapter 20, Step 5b consumes p_b,x, and the
//! Chapter 30, Section 3 "Proportion of Time Blocked" procedure (EPUB
//! `235_Ch30_03.xhtml`, Equation 30-13) produces them.
//!
//! ## Procedure (HCM Chapter 30, Section 3)
//!
//! Each major-street through-lane group carries a *combined arrival flow
//! profile* at the TWSC intersection, built by dispersing the upstream
//! signal's discharge flow profiles over the segment (the Robertson-style
//! machinery of [`crate::hcm::chapter18::platoon_dispersion`], Equations 30-9
//! through 30-12). A minor movement is *blocked* during the steps of the
//! cycle whose arrival flow rate exceeds the critical platoon flow rate
//! q_c = 3,600 / t_c, where t_c is the movement's Chapter 20 critical headway
//! (the platoon headways are then too short to enter or cross). The blocked
//! period duration t'_p is the count of such steps, and
//! p_b = t'_p d_t / C (Equation 30-13).
//!
//! The through-lane group evaluated depends on the movement (HCM Chapter 30,
//! Section 3, "Proportion of Time Blocked"):
//!
//! | TWSC movement            | Blocking through-lane group        |
//! |--------------------------|------------------------------------|
//! | 1  (EB major left)       | opposing = westbound (movement 5)  |
//! | 4  (WB major left)       | opposing = eastbound (movement 2)  |
//! | 9  (NB minor right)      | approaching from left = eastbound  |
//! | 12 (SB minor right)      | approaching from left = westbound  |
//! | 7, 8  (NB minor LT/TH)   | both directions (platoon present from either) |
//! | 10, 11 (SB minor LT/TH)  | both directions                    |
//!
//! The direction assignments match the Chapter 20 conflicting-flow equations
//! (v_c,1 and v_c,12 involve v_5; v_c,4 and v_c,9 involve v_2). For the
//! minor-street left and through movements the blocked period is the union of
//! the two directions' blocked steps ("the time when a platoon from either
//! direction is present"). The Stage I / Stage II proportions of the
//! two-stage movements are *not* produced here; per HCM Exhibit 20-19 they are
//! the opposing major-street-left proportions p_b,4 / p_b,1, which
//! [`PlatoonBlockage::stages`] already derives from `pb1`/`pb4`.
//!
//! ## v_c,min interplay
//!
//! Section 3 does not use the v_c,min = 1,000 N threshold; that threshold
//! enters only in Chapter 20, Step 5b (Equation 20-19), which consumes p_b,x
//! downstream. The blocked period here is governed solely by the critical
//! platoon flow rate q_c = 3,600 / t_c.
//!
//! ## Validation
//!
//! The HCM does not publish a hand-computable derivation chain from upstream
//! signal timing to the p_b values (Chapter 30 Example Problem 1 reports them
//! as engine output — the narrative p_b = 0.15 for the eastbound major left
//! and 0.25 for the northbound minor left at Access Point Intersection 1; the
//! 0.170 / 0.260 set consumed by Chapter 32 TWSC Example Problem 4 is
//! Exhibit 32-12, sourced "from Chapter 30, Example Problem 1"). Reproducing
//! those exactly requires the full Chapter 19 coordinated-actuated engine and
//! the Section 2 origin–destination distribution, which are deferred (see the
//! module doc comment on `platoon_dispersion.rs`). The mechanism is therefore
//! unit-tested against a hand-computable square-wave platoon and against the
//! dispersion-flattening monotonicity property (p_b non-increasing with
//! distance), not against a manufactured published target.

use serde::{Deserialize, Serialize};

use crate::hcm::chapter18::platoon_dispersion::{combined_arrival_profile, MovementDischarge};
use crate::hcm::chapter20::twsc::{Mv, PlatoonBlockage, Twsc};

/// Feet per second per mile per hour (5,280 ft / 3,600 s).
const FT_PER_S_PER_MPH: f64 = 5_280.0 / 3_600.0;

fn default_time_step() -> f64 {
    1.0
}

/// One coordinated upstream signalized intersection feeding one major-street
/// through-lane group at the TWSC intersection (HCM Chapter 30, Section 3).
///
/// The [`discharges`](UpstreamSignal::discharges) are the upstream movements
/// that combine into this direction's arrival flow profile — typically the
/// upstream through movement plus the upstream left and right turns that feed
/// the segment. Each is a [`MovementDischarge`] carrying the departure timing
/// and queue-service behavior used to build the discharge flow profile
/// (Equation 30-9 machinery in [`crate::hcm::chapter18::platoon_dispersion`]).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpstreamSignal {
    /// Distance from the upstream signal to the TWSC intersection along the
    /// segment (segment length L), ft. Used with `progression_speed_mph` to
    /// obtain the running time t_R that drives platoon dispersion.
    pub distance_ft: f64,
    /// Platoon progression speed on the segment, mph. Running time
    /// t_R = distance_ft / (progression_speed_mph × 5,280 / 3,600), s.
    pub progression_speed_mph: f64,
    /// Upstream movements discharging into this through-lane group over one
    /// cycle (Chapter 30, Section 3, "Discharge Flow Profile").
    pub discharges: Vec<MovementDischarge>,
    /// Midblock (access-point) volume entering this through-lane group with a
    /// uniform arrival flow profile, veh/h (Chapter 30, Section 3, "Arrival
    /// Flow Profile"). Defaults to 0.
    #[serde(default)]
    pub uniform_volume_veh_h: f64,
}

impl UpstreamSignal {
    /// Segment running time t_R, s (distance ÷ progression speed).
    fn running_time_s(&self) -> f64 {
        let speed_ft_s = (self.progression_speed_mph * FT_PER_S_PER_MPH).max(1e-9);
        self.distance_ft / speed_ft_s
    }

    /// Combined arrival flow profile at the TWSC intersection for this
    /// through-lane group, veh/step over `cycle_steps` steps (Equation 30-9,
    /// [`combined_arrival_profile`]).
    fn arrival_profile(&self, cycle_steps: usize, time_step_s: f64) -> Vec<f64> {
        combined_arrival_profile(
            &self.discharges,
            self.uniform_volume_veh_h,
            cycle_steps,
            time_step_s,
            self.running_time_s(),
        )
    }
}

/// Coordinated upstream signals bracketing a TWSC intersection on the major
/// street, from which the Chapter 20 [`PlatoonBlockage`] proportions p_b,x are
/// computed (HCM Chapter 30, Section 3).
///
/// `eastbound` describes the signal producing the eastbound through-lane
/// group's arrival profile (movement 2) at the TWSC intersection; `westbound`
/// describes the westbound through-lane group (movement 5). Either may be
/// `None` (an uncoordinated or one-sided approach contributes no platoon and
/// hence no blocking on the movements it would feed).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpstreamSignals {
    /// System cycle length C, s, shared by the coordinated signals
    /// (Equation 30-13 denominator).
    pub cycle_s: f64,
    /// Upstream signal feeding the eastbound through-lane group (movement 2).
    #[serde(default)]
    pub eastbound: Option<UpstreamSignal>,
    /// Upstream signal feeding the westbound through-lane group (movement 5).
    #[serde(default)]
    pub westbound: Option<UpstreamSignal>,
    /// Flow-profile time step d_t, s/step (Chapter 30 recommends 1.0).
    #[serde(default = "default_time_step")]
    pub time_step_s: f64,
}

impl UpstreamSignals {
    /// Number of time steps in one cycle, C' = round(C / d_t).
    fn cycle_steps(&self) -> usize {
        (self.cycle_s / self.time_step_s.max(1e-9)).round().max(0.0) as usize
    }

    /// Eastbound / westbound arrival flow profiles, veh/step (`None` when that
    /// direction has no upstream signal).
    fn direction_profiles(&self) -> (Option<Vec<f64>>, Option<Vec<f64>>) {
        let n = self.cycle_steps();
        let dt = self.time_step_s;
        let eb = self.eastbound.as_ref().map(|s| s.arrival_profile(n, dt));
        let wb = self.westbound.as_ref().map(|s| s.arrival_profile(n, dt));
        (eb, wb)
    }

    /// Compute the Chapter 20 [`PlatoonBlockage`] p_b,x proportions from the
    /// upstream signals, using each movement's Chapter 20 critical headway
    /// t_c (from `twsc`) for the critical platoon flow rate q_c = 3,600 / t_c
    /// (HCM Chapter 30, Section 3; Equation 30-13).
    ///
    /// Only the one-stage totals (`pb1`, `pb4`, `pb7`, `pb8`, `pb9`, `pb10`,
    /// `pb11`, `pb12`) are populated; the Stage I/II proportions of the
    /// two-stage movements are derived from `pb1`/`pb4` by
    /// [`PlatoonBlockage::stages`] per HCM Exhibit 20-19.
    pub fn compute_platoon_blockage(&self, twsc: &Twsc) -> PlatoonBlockage {
        let (eb, wb) = self.direction_profiles();
        let dt = self.time_step_s;
        let cycle = self.cycle_s;

        // p_b for a movement blocked by a single through-lane group.
        let single = |profile: &Option<Vec<f64>>, mv: Mv| -> f64 {
            let tc = twsc.critical_headway(mv, None);
            match profile {
                Some(p) => proportion_time_blocked_from_profile(p, q_c(tc), dt, cycle),
                None => 0.0,
            }
        };
        // p_b for a movement blocked when a platoon is present from either
        // direction (union of blocked steps), HCM Chapter 30, Section 3.
        let both = |mv: Mv| -> f64 {
            let tc = twsc.critical_headway(mv, None);
            union_proportion_time_blocked(eb.as_deref(), wb.as_deref(), q_c(tc), dt, cycle)
        };

        PlatoonBlockage {
            // Movement 1 (EB left) opposes the westbound through-lane group;
            // movement 4 (WB left) opposes the eastbound one.
            pb1: single(&wb, Mv::M1),
            pb4: single(&eb, Mv::M4),
            // Movement 9 (NB right) merges with the eastbound through-lane
            // group approaching from the left; movement 12 (SB right) with the
            // westbound one.
            pb9: single(&eb, Mv::M9),
            pb12: single(&wb, Mv::M12),
            // Minor-street left and through movements: platoon from either
            // direction blocks them.
            pb7: both(Mv::M7),
            pb8: both(Mv::M8),
            pb10: both(Mv::M10),
            pb11: both(Mv::M11),
        }
    }
}

/// Critical platoon flow rate q_c = 3,600 / t_c, veh/h (HCM Chapter 30,
/// Section 3). Above this arrival flow rate the platoon headways are too short
/// for a minor movement to enter or cross.
fn q_c(critical_headway_s: f64) -> f64 {
    if critical_headway_s <= 0.0 {
        f64::INFINITY
    } else {
        3_600.0 / critical_headway_s
    }
}

/// Number of cycle steps whose arrival flow rate exceeds the critical platoon
/// flow rate (the blocked period duration t'_p in steps, HCM Chapter 30,
/// Section 3). The per-step profile is veh/step, so the threshold in the same
/// units is q_c d_t / 3,600.
///
/// * `profile` — arrival flow profile, veh/step
/// * `q_c_veh_h` — critical platoon flow rate q_c, veh/h
/// * `time_step_s` — time step duration d_t, s/step
pub fn blocked_period_steps(profile: &[f64], q_c_veh_h: f64, time_step_s: f64) -> f64 {
    if q_c_veh_h.is_infinite() {
        return 0.0;
    }
    let threshold = q_c_veh_h * time_step_s / 3_600.0;
    profile.iter().filter(|&&r| r > threshold).count() as f64
}

/// Proportion of time blocked for a single through-lane group arrival profile
/// (HCM Equation 30-13, p_b = t'_p d_t / C, where t'_p is the number of steps
/// whose arrival flow rate exceeds q_c).
///
/// * `profile` — arrival flow profile, veh/step
/// * `q_c_veh_h` — critical platoon flow rate q_c = 3,600 / t_c, veh/h
/// * `time_step_s` — time step duration d_t, s/step
/// * `cycle_s` — cycle length C, s
pub fn proportion_time_blocked_from_profile(
    profile: &[f64],
    q_c_veh_h: f64,
    time_step_s: f64,
    cycle_s: f64,
) -> f64 {
    let tp = blocked_period_steps(profile, q_c_veh_h, time_step_s);
    proportion(tp, time_step_s, cycle_s)
}

/// Proportion of time blocked when a platoon from *either* direction blocks
/// the movement: the blocked period is the union of the two directions'
/// blocked steps (HCM Chapter 30, Section 3, minor-street left/through).
fn union_proportion_time_blocked(
    eb: Option<&[f64]>,
    wb: Option<&[f64]>,
    q_c_veh_h: f64,
    time_step_s: f64,
    cycle_s: f64,
) -> f64 {
    if q_c_veh_h.is_infinite() {
        return 0.0;
    }
    let threshold = q_c_veh_h * time_step_s / 3_600.0;
    let n = eb.map(|p| p.len()).max(wb.map(|p| p.len())).unwrap_or(0);
    let blocked = |p: Option<&[f64]>, i: usize| -> bool {
        p.and_then(|p| p.get(i)).map(|&r| r > threshold).unwrap_or(false)
    };
    let count = (0..n).filter(|&i| blocked(eb, i) || blocked(wb, i)).count() as f64;
    proportion(count, time_step_s, cycle_s)
}

/// Equation 30-13: p_b = t'_p d_t / C, clamped to [0, 1].
fn proportion(blocked_period_steps: f64, time_step_s: f64, cycle_s: f64) -> f64 {
    if cycle_s <= 0.0 {
        return 0.0;
    }
    (blocked_period_steps * time_step_s / cycle_s).clamp(0.0, 1.0)
}
