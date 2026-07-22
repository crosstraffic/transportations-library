//! HCM Chapter 25, Section 4: Oversaturated Segment Evaluation — the
//! node–segment time-step engine invoked by the Chapter 10 core freeway
//! facility methodology (Step A-12) when any segment has vd/c > 1.0.
//!
//! Source (HCM 7th Edition EPUB): `195_Ch25_04.xhtml` — Equations 25-6
//! through 25-34, Exhibits 25-1 through 25-5.
//!
//! Representation (Exhibit 25-1): a facility of `n` segments has `n + 1`
//! nodes. Node `i` (0-based here) is the upstream end of segment `i`;
//! node `n` is the facility exit. On-ramp flows enter at a segment's
//! upstream node (`onrd[i]` feeds segment `i`); off-ramp flows leave at a
//! segment's downstream node (`offrd[i]` exits from segment `i − 1` at
//! node `i`), matching Exhibit 25-4.
//!
//! All flow state is carried in vehicles per time step; per-period inputs
//! are veh/h. The default time step is 15 s (Chapter 25, Procedure
//! Parameters), giving `S = 60` steps per 15-min analysis period and
//! `T = 240` steps per hour.

use serde::{Deserialize, Serialize};

use super::exhibits::DENSITY_AT_CAPACITY_PC;

/// A large stand-in for "no constraint" (avoids `f64::INFINITY` arithmetic
/// in weighted averages of historical mainline outputs).
const BIG: f64 = 1.0e12;

/// Per-period inputs to the oversaturated engine, all in veh/h except
/// where noted. Vectors indexed by segment (`0..n`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OversatPeriodInput {
    /// Segment capacity SC(i, p), veh/h (prebreakdown; Equation 25-29's
    /// queue discharge drop is applied dynamically inside the engine).
    pub capacity: Vec<f64>,
    /// Segment demand SD(i, p), veh/h.
    pub demand: Vec<f64>,
    /// Mainline demand entering the facility, veh/h.
    pub mainline_demand: f64,
    /// On-ramp demand entering at node i (upstream end of segment i), veh/h.
    pub onrd: Vec<f64>,
    /// Off-ramp demand exiting at node i (from segment i − 1), veh/h.
    /// `offrd[0]` must be 0; an off-ramp at the exit gore of segment n − 1
    /// is not modeled (Chapter 10 recommends ending with a basic segment).
    pub offrd: Vec<f64>,
    /// On-ramp roadway capacity at node i (Exhibit 14-12 adjusted to veh/h);
    /// 0 where no ramp exists.
    pub ramp_capacity: Vec<f64>,
    /// Optional ramp-metering rate at node i, veh/h (Chapter 10, Step A-7:
    /// "the capacity of each entrance ramp ... is changed to reflect the
    /// specified ramp-metering rate").
    pub ramp_metering: Vec<Option<f64>>,
    /// Background density KB(i, p), veh/mi/ln — from the Chapter 12/13/14
    /// procedures evaluated at the expected demand ED (Equation 25-6).
    pub background_density: Vec<f64>,
    /// Off-ramp diverge percentage for this period,
    /// `OFRD(i, p) / SD(i − 1, p)` (Equations 25-23 through 25-25), by node.
    pub diverge_pct: Vec<f64>,
    /// Off-ramp diverge percentage of the preceding period, by node.
    pub diverge_pct_prev: Vec<f64>,
    /// Front-clearing-queue flag per segment (Equation 25-12, evaluated at
    /// the analysis-period level by the caller).
    pub front_clearing: Vec<bool>,
}

/// Per-period outputs, indexed by segment unless noted.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OversatPeriodResult {
    /// Average segment flow SF(i, p), veh/h (Equation 25-30).
    pub segment_flow: Vec<f64>,
    /// Average number of vehicles NV(i, p), veh (Equation 25-31).
    pub avg_vehicles: Vec<f64>,
    /// Average density K(i, p), veh/mi/ln (Equation 25-32).
    pub density: Vec<f64>,
    /// Whether the segment carried unserved vehicles (UV > 0.001) during
    /// any time step of the period (selects the queued performance path).
    pub had_queue: Vec<bool>,
    /// Unserved vehicles at the end of the period UV(i, S, p), veh.
    pub unserved_end: Vec<f64>,
    /// Queue length at the end of the period, ft (Equation 25-34).
    pub queue_length_ft: Vec<f64>,
    /// On-ramp queue at the end of the period, veh, by node
    /// (Equation 25-21).
    pub onr_queue_end: Vec<f64>,
    /// Average on-ramp flow served, veh/h, by node.
    pub onr_flow: Vec<f64>,
    /// Average off-ramp flow served, veh/h, by node.
    pub ofr_flow: Vec<f64>,
    /// Unserved vehicles held upstream of the facility entrance at the end
    /// of the period, veh (queue outside the spatial domain; Chapter 10
    /// reports these as unserved vehicles).
    pub entry_queue_end: Vec<f64>,
}

/// Time-step engine state carried across analysis periods.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OversaturatedEngine {
    /// Number of segments n.
    pub num_segments: usize,
    /// Time steps per analysis period S (default 60 = 15-s steps).
    pub steps_per_period: usize,
    /// Time steps per hour T (default 240).
    pub steps_per_hour: f64,
    /// Segment lengths, mi.
    pub length_mi: Vec<f64>,
    /// Segment lane counts.
    pub lanes: Vec<f64>,
    /// Heavy-vehicle adjustment factor f_HV (facility-wide), decimal.
    pub f_hv: f64,
    /// Jam density KJ, pc/mi/ln (Chapter 10 Step A-6 global parameter).
    pub jam_density_pc: f64,
    /// Queue discharge capacity drop alpha, decimal (Equation 25-29).
    pub capacity_drop: f64,

    // ── State (previous time step / carried across periods) ─────────────
    /// Number of vehicles on each segment NV, veh.
    nv: Vec<f64>,
    /// Unserved vehicles on each segment UV, veh.
    uv: Vec<f64>,
    /// Segment flow in the previous step, veh/step.
    sf_prev: Vec<f64>,
    /// Mainline flow across each node in the previous step, veh/step.
    mf_prev: Vec<f64>,
    /// On-ramp flow at each node in the previous step, veh/step.
    onrf_prev: Vec<f64>,
    /// MO2 of each node in the previous step, veh/step.
    mo2_prev: Vec<f64>,
    /// MO3 of each node in the previous step, veh/step.
    mo3_prev: Vec<f64>,
    /// On-ramp queue at each node, veh.
    onrq: Vec<f64>,
    /// Unserved vehicles upstream of the facility entrance, veh.
    entry_queue: f64,
    /// Cumulative demand destined into each segment since oversaturation
    /// began, veh (first term of Equation 25-22; updated at period end).
    cum_demand: Vec<f64>,
    /// Cumulative arrivals into each segment since oversaturation began,
    /// veh (second/third terms of Equation 25-22; updated every step).
    cum_arrivals: Vec<f64>,
    /// History ring buffers for the MO3 wave-travel-time lookback
    /// (Equation 25-15), by node then step.
    hist_mo1: Vec<Vec<f64>>,
    hist_mo2: Vec<Vec<f64>>,
    hist_mo3: Vec<Vec<f64>>,
    hist_ofrf: Vec<Vec<f64>>,
    hist_sc: Vec<Vec<f64>>,
    /// Whether `init_period` has run at least once.
    started: bool,
}

impl OversaturatedEngine {
    /// Create an engine for `num_segments` segments.
    ///
    /// * `length_mi`, `lanes` — static segment geometry
    /// * `f_hv` — heavy-vehicle adjustment factor (converts pc to veh)
    /// * `jam_density_pc` — KJ, pc/mi/ln
    /// * `capacity_drop` — queue discharge drop alpha (Equation 25-29)
    /// * `time_step_s` — step duration, s (15 s default per Chapter 25)
    pub fn new(
        length_mi: Vec<f64>,
        lanes: Vec<f64>,
        f_hv: f64,
        jam_density_pc: f64,
        capacity_drop: f64,
        time_step_s: f64,
    ) -> Self {
        let n = length_mi.len();
        let steps_per_period = (900.0 / time_step_s).round() as usize;
        let steps_per_hour = 3600.0 / time_step_s;
        Self {
            num_segments: n,
            steps_per_period,
            steps_per_hour,
            length_mi,
            lanes,
            f_hv,
            jam_density_pc,
            capacity_drop,
            nv: vec![0.0; n],
            uv: vec![0.0; n],
            sf_prev: vec![0.0; n],
            mf_prev: vec![0.0; n + 1],
            onrf_prev: vec![0.0; n + 1],
            mo2_prev: vec![BIG; n + 1],
            mo3_prev: vec![BIG; n + 1],
            onrq: vec![0.0; n + 1],
            entry_queue: 0.0,
            cum_demand: vec![0.0; n],
            cum_arrivals: vec![0.0; n],
            hist_mo1: vec![Vec::new(); n + 1],
            hist_mo2: vec![Vec::new(); n + 1],
            hist_mo3: vec![Vec::new(); n + 1],
            hist_ofrf: vec![Vec::new(); n + 1],
            hist_sc: vec![Vec::new(); n],
            started: false,
        }
    }

    /// Expected demand ED (Equation 25-6):
    /// `ED(i, p) = min[SC(i, p), ED(i − 1, p) + ONRD(i, p) − OFRD(i, p)]`
    /// evaluated recursively from the facility entrance. Static helper so
    /// the caller can derive background densities before running a period.
    ///
    /// Note: the EPUB prints the off-ramp term as `OFRD(i − 1, p)` under its
    /// segment-based ramp indexing; with this module's node-based indexing
    /// (`offrd[i]` leaves segment i − 1 at node i) the same ramp is
    /// `offrd[i]`.
    pub fn expected_demand(
        capacity: &[f64],
        mainline_demand: f64,
        onrd: &[f64],
        offrd: &[f64],
    ) -> Vec<f64> {
        let n = capacity.len();
        let mut ed = vec![0.0; n];
        let mut upstream = mainline_demand;
        for i in 0..n {
            let arriving = upstream + onrd[i] - offrd[i];
            ed[i] = arriving.min(capacity[i]);
            upstream = ed[i];
        }
        ed
    }

    /// Queue density (Equation 25-10), veh/mi/ln:
    /// `KQ = KJ×f_HV − (KJ − KC)×f_HV × SF(i, t−1) / SC(i, t)`
    fn queue_density(&self, sf_prev_step: f64, sc_step: f64) -> f64 {
        let kj = self.jam_density_pc;
        let kc = DENSITY_AT_CAPACITY_PC;
        if sc_step <= 0.0 {
            return kj * self.f_hv;
        }
        kj * self.f_hv - (kj - kc) * self.f_hv * (sf_prev_step / sc_step)
    }

    /// Segment initialization (Equation 25-7): number of vehicles at the
    /// start of the period from the background density plus carried-over
    /// unserved vehicles: `NV(i, 0, p) = KB(i, p)×N(i)×L(i) + UV(i, S, p−1)`.
    fn init_period(&mut self, input: &OversatPeriodInput) {
        for i in 0..self.num_segments {
            let background = input.background_density[i] * self.lanes[i] * self.length_mi[i];
            self.nv[i] = background + self.uv[i];
        }
        if !self.started {
            // Seed "previous step" flows for the first oversaturated step
            // from the expected demands (queues have not yet formed).
            let ed = Self::expected_demand(
                &input.capacity,
                input.mainline_demand,
                &input.onrd,
                &input.offrd,
            );
            let t = self.steps_per_hour;
            for i in 0..self.num_segments {
                self.sf_prev[i] = ed[i] / t;
            }
            self.mf_prev[0] = input.mainline_demand.min(input.capacity[0]) / t;
            for i in 1..=self.num_segments {
                let up = ed[i - 1];
                let ofr = up * input.diverge_pct[i.min(input.diverge_pct.len() - 1)];
                self.mf_prev[i] = (up - ofr) / t;
            }
            for i in 0..self.num_segments {
                self.onrf_prev[i] = input.onrd[i] / t;
            }
            self.started = true;
        }
    }

    /// Weighted-average lookback into a node history buffer at a possibly
    /// non-integer number of steps `back` before the current step
    /// (Equation 25-15: "If the wave travel time is not an integer number
    /// of time steps, then the weighted average performance ... is taken
    /// for the time steps nearest the wave travel time").
    fn lookback(hist: &[f64], back: f64) -> Option<f64> {
        if hist.is_empty() {
            return None;
        }
        let len = hist.len() as f64;
        let idx = len - back; // fractional index into history (1-based end)
        if idx < 1.0 {
            return None; // wave has not yet reached the upstream end
        }
        let lo = idx.floor() as usize - 1;
        let hi = (idx.ceil() as usize - 1).min(hist.len() - 1);
        let w = idx - idx.floor();
        Some(hist[lo] * (1.0 - w) + hist[hi] * w)
    }

    /// Run one 15-min analysis period of the oversaturated procedure
    /// (Exhibit 25-3 flowchart; Equations 25-8 through 25-34).
    pub fn run_period(&mut self, input: &OversatPeriodInput) -> OversatPeriodResult {
        let n = self.num_segments;
        let s_steps = self.steps_per_period;
        let t = self.steps_per_hour;

        self.init_period(input);

        // Per-step capacities, veh/step (queue discharge drop applied
        // dynamically per Equation 25-29).
        let base_sc_step: Vec<f64> = input.capacity.iter().map(|c| c / t).collect();

        // Front-clearing wave travel time per segment, steps
        // (Equations 25-13/25-14): WS = SC/[N×(KJ−KC)×f_HV]; WTT = T×L/WS.
        let wtt: Vec<Option<f64>> = (0..n)
            .map(|i| {
                if input.front_clearing[i] {
                    let ws = input.capacity[i]
                        / (self.lanes[i]
                            * (self.jam_density_pc - DENSITY_AT_CAPACITY_PC)
                            * self.f_hv);
                    Some(t * self.length_mi[i] / ws)
                } else {
                    None
                }
            })
            .collect();

        // Accumulators
        let mut sum_sf = vec![0.0; n];
        let mut sum_nv = vec![0.0; n];
        let mut had_queue = vec![false; n];
        let mut sum_onrf = vec![0.0; n + 1];
        let mut sum_ofrf = vec![0.0; n + 1];
        let mut kq_last = vec![0.0; n];

        for _step in 0..s_steps {
            // Effective capacities this step: Equation 25-29 — any active
            // bottleneck (UV on the upstream segment > 0.001) discharges at
            // (1 − alpha) × SC.
            let mut sc_step = base_sc_step.clone();
            for i in 0..n {
                let upstream_queued = if i == 0 {
                    self.entry_queue > 0.001
                } else {
                    self.uv[i - 1] > 0.001
                };
                if upstream_queued {
                    sc_step[i] *= 1.0 - self.capacity_drop;
                }
            }

            let mut mf = vec![0.0; n + 1];
            let mut onrf = vec![0.0; n + 1];
            let mut ofrf = vec![0.0; n + 1];
            let mut mo1 = vec![BIG; n + 1];
            let mut mo2 = vec![BIG; n + 1];
            let mut mo3 = vec![BIG; n + 1];

            for node in 0..=n {
                // ── Off-ramp flow (Exhibit 25-3, Steps 5–8) ──────────────
                if node > 0 && input.offrd[node.min(input.offrd.len() - 1)] > 0.0 {
                    let inflow_now = mf[node - 1] + onrf[node - 1];
                    // Equation 25-22: deficit of vehicles destined into
                    // segment node − 1 that were metered upstream.
                    let deficit =
                        (self.cum_demand[node - 1] - self.cum_arrivals[node - 1]).max(0.0);
                    let pct_now = input.diverge_pct[node];
                    let pct_prev = input.diverge_pct_prev[node];
                    ofrf[node] = if deficit > 0.001 {
                        if inflow_now <= deficit {
                            // Equation 25-23
                            inflow_now * pct_prev
                        } else {
                            // Equation 25-24
                            deficit * pct_prev + (inflow_now - deficit) * pct_now
                        }
                    } else {
                        // Equation 25-25
                        inflow_now * pct_now
                    };
                }

                // ── Mainline input (Equation 25-8) ───────────────────────
                let mi = if node == 0 {
                    input.mainline_demand / t + self.entry_queue
                } else {
                    mf[node - 1] + onrf[node - 1] - ofrf[node] + self.uv[node - 1]
                };

                if node == n {
                    // Facility exit: unconstrained downstream.
                    mf[node] = mi.min(sc_step[n - 1]);
                    continue;
                }

                // ── On-ramp flow (Equations 25-17 through 25-21) ─────────
                if input.onrd[node] > 0.0 || self.onrq[node] > 0.0 {
                    // Equation 25-17
                    let onri = input.onrd[node] / t + self.onrq[node];
                    // Equation 25-18: total throughput available at the
                    // merge point, estimated from the previous step.
                    let lambda = sc_step[node]
                        .min(self.mf_prev[node + 1] + self.onrf_prev[node])
                        .min(self.mo3_prev[node] + self.onrf_prev[node]);
                    let mut onro = (lambda - mi).max(lambda / (2.0 * self.lanes[node]));
                    if input.ramp_capacity[node] > 0.0 {
                        onro = onro.min(input.ramp_capacity[node] / t);
                    }
                    if let Some(rm) = input.ramp_metering[node] {
                        onro = onro.min(rm / t);
                    }
                    // Equations 25-19/25-20/25-21
                    if onri <= onro {
                        onrf[node] = onri;
                        self.onrq[node] = 0.0;
                    } else {
                        onrf[node] = onro.max(0.0);
                        self.onrq[node] = onri - onrf[node];
                    }
                }

                // ── MO1 (Equation 25-9): on-ramp flow constraint ─────────
                if onrf[node] > 0.0 {
                    mo1[node] = (sc_step[node] - onrf[node])
                        .min(self.mo2_prev[node])
                        .min(self.mo3_prev[node]);
                }

                // ── MO3 (Equations 25-13 through 25-15): front-clearing ──
                if let Some(wtt_steps) = wtt[node] {
                    let m1 = Self::lookback(&self.hist_mo1[node + 1], wtt_steps);
                    let m2 = Self::lookback(&self.hist_mo2[node + 1], wtt_steps);
                    let m3 = Self::lookback(&self.hist_mo3[node + 1], wtt_steps);
                    let of = Self::lookback(&self.hist_ofrf[node + 1], wtt_steps);
                    let sc_hist = Self::lookback(&self.hist_sc[node], wtt_steps);
                    let sc_down = if node + 1 < n {
                        Self::lookback(&self.hist_sc[node + 1], wtt_steps)
                    } else {
                        Some(BIG)
                    };
                    if let (Some(m1), Some(m2), Some(m3), Some(of), Some(sc_h), Some(sc_d)) =
                        (m1, m2, m3, of, sc_hist, sc_down)
                    {
                        // Equation 25-15
                        mo3[node] = m1
                            .min(m2 + of)
                            .min(m3 + of)
                            .min(sc_h)
                            .min(sc_d + of)
                            - ofrf[node];
                    }
                }

                // ── MO2 (Equations 25-10/25-11): downstream storage ──────
                // VERIFY-HCM: Equation 25-29 redefines SC in place, which
                // would also lower the Equation 25-10 queue density and
                // spread queues much farther upstream than the published
                // Chapter 25 Example Problem 2 results. The prebreakdown
                // capacity is therefore used in the KQ ratio (queue storage
                // density), while the reduced (queue discharge) capacity
                // governs node throughput (MO1/MF; Equation 25-16).
                let kq = self.queue_density(self.sf_prev[node], base_sc_step[node]);
                kq_last[node] = kq;
                let max_veh = kq * self.lanes[node] * self.length_mi[node];
                mo2[node] = self.sf_prev[node] - onrf[node] + max_veh - self.nv[node];

                // ── Mainline flow (Equation 25-16) ───────────────────────
                let upstream_cap = if node == 0 { BIG } else { sc_step[node - 1] };
                mf[node] = mi
                    .min(mo1[node])
                    .min(mo2[node])
                    .min(mo3[node])
                    .min(sc_step[node])
                    .min(upstream_cap)
                    .max(0.0);
            }

            // Entry queue update (vehicles unable to enter segment 0).
            let entry_input = input.mainline_demand / t + self.entry_queue;
            self.entry_queue = (entry_input - mf[0]).max(0.0);

            // ── Segment flows and vehicle conservation (Eqs. 25-26/27/28) ─
            let mut sf = vec![0.0; n];
            for i in 0..n {
                sf[i] = mf[i + 1] + ofrf[i + 1]; // Equation 25-26
                let inflow = mf[i] + onrf[i];
                self.nv[i] += inflow - sf[i]; // Equation 25-27
                // Cumulative arrivals for the off-ramp deficit method.
                self.cum_arrivals[i] += inflow;
                // Background is the minimum number of vehicles that can be
                // on the segment (Chapter 25, Segment Initialization).
                let background =
                    input.background_density[i] * self.lanes[i] * self.length_mi[i];
                if self.nv[i] < background {
                    self.nv[i] = background;
                }
                // Equation 25-28
                self.uv[i] = (self.nv[i] - background).max(0.0);
                if self.uv[i] > 0.001 {
                    had_queue[i] = true;
                }
                sum_sf[i] += sf[i];
                sum_nv[i] += self.nv[i];
            }
            for node in 0..=n {
                sum_onrf[node] += onrf[node];
                sum_ofrf[node] += ofrf[node];
            }

            // Roll step state and histories.
            self.sf_prev = sf;
            self.mf_prev = mf;
            self.onrf_prev = onrf.clone();
            self.mo2_prev = mo2.clone();
            self.mo3_prev = mo3.clone();
            for node in 0..=n {
                Self::push_hist(&mut self.hist_mo1[node], mo1[node]);
                Self::push_hist(&mut self.hist_mo2[node], mo2[node]);
                Self::push_hist(&mut self.hist_mo3[node], mo3[node]);
                Self::push_hist(&mut self.hist_ofrf[node], ofrf[node]);
            }
            for i in 0..n {
                Self::push_hist(&mut self.hist_sc[i], sc_step[i]);
            }
        }

        // Update cumulative demand destined into each segment (Eq. 25-22).
        let period_h = s_steps as f64 / t;
        let mut upstream_demand = input.mainline_demand;
        for i in 0..n {
            let arriving = upstream_demand + input.onrd[i] - input.offrd[i];
            self.cum_demand[i] += arriving * period_h;
            upstream_demand = input.demand[i];
        }

        // ── Period aggregation (Equations 25-30 through 25-34) ───────────
        let s = s_steps as f64;
        let mut segment_flow = vec![0.0; n];
        let mut avg_vehicles = vec![0.0; n];
        let mut density = vec![0.0; n];
        let mut queue_length_ft = vec![0.0; n];
        for i in 0..n {
            segment_flow[i] = (t / s) * sum_sf[i]; // Equation 25-30, veh/h
            avg_vehicles[i] = sum_nv[i] / s; // Equation 25-31
            density[i] = avg_vehicles[i] / (self.length_mi[i] * self.lanes[i]); // Eq. 25-32
            // Equation 25-34: queue length from unserved vehicles and the
            // queue-vs-background density difference.
            // VERIFY-HCM: Equation 25-34 as printed omits the lane count in
            // the density difference; the per-lane densities are multiplied
            // by N here so that UV (veh) over veh/mi yields miles.
            if self.uv[i] > 0.001 {
                let dk = (kq_last[i] - input.background_density[i]).max(1.0) * self.lanes[i];
                queue_length_ft[i] = (self.uv[i] / dk * 5280.0).min(self.length_mi[i] * 5280.0);
            }
        }

        OversatPeriodResult {
            segment_flow,
            avg_vehicles,
            density,
            had_queue,
            unserved_end: self.uv.clone(),
            queue_length_ft,
            onr_queue_end: self.onrq.clone(),
            onr_flow: sum_onrf.iter().map(|v| v * t / s).collect(),
            ofr_flow: sum_ofrf.iter().map(|v| v * t / s).collect(),
            entry_queue_end: vec![self.entry_queue],
        }
    }

    fn push_hist(hist: &mut Vec<f64>, value: f64) {
        // Keep a bounded history: two analysis periods is ample for any
        // realistic recovery-wave travel time (Equation 25-14).
        const MAX: usize = 240;
        if hist.len() >= MAX {
            hist.remove(0);
        }
        hist.push(value);
    }

    /// Front-clearing-queue test (Equation 25-12): a queue clears from the
    /// front when this period's capacity net of on-ramp demand exceeds both
    /// the preceding period's net capacity and this period's segment demand.
    pub fn front_clearing_queue(
        sc_now: f64,
        onrd_now: f64,
        sc_prev: f64,
        onrd_prev: f64,
        sd_now: f64,
    ) -> bool {
        (sc_now - onrd_now) > (sc_prev - onrd_prev) && (sc_now - onrd_now) > sd_now
    }
}

// ═════════════════════════════════════════════════════════════════════════
// Tests
// ═════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    fn simple_engine(n: usize) -> OversaturatedEngine {
        OversaturatedEngine::new(vec![1.0; n], vec![2.0; n], 1.0, 190.0, 0.07, 15.0)
    }

    fn simple_input(n: usize, capacity: f64, mainline: f64) -> OversatPeriodInput {
        OversatPeriodInput {
            capacity: vec![capacity; n],
            demand: vec![mainline; n],
            mainline_demand: mainline,
            onrd: vec![0.0; n],
            offrd: vec![0.0; n + 1],
            ramp_capacity: vec![0.0; n + 1],
            ramp_metering: vec![None; n + 1],
            background_density: vec![20.0; n],
            diverge_pct: vec![0.0; n + 1],
            diverge_pct_prev: vec![0.0; n + 1],
            front_clearing: vec![false; n],
        }
    }

    #[test]
    fn test_equation_25_6_expected_demand() {
        // Bottleneck (capacity 4,000) meters downstream expected demands.
        let ed = OversaturatedEngine::expected_demand(
            &[6000.0, 4000.0, 6000.0],
            5000.0,
            &[0.0, 0.0, 0.0],
            &[0.0, 0.0, 0.0, 0.0],
        );
        assert_eq!(ed, vec![5000.0, 4000.0, 4000.0]);
    }

    #[test]
    fn test_equation_25_10_queue_density() {
        let eng = simple_engine(1);
        // SF = SC (flow at capacity): KQ = KC = 45 (f_HV = 1)
        let kq = eng.queue_density(10.0, 10.0);
        assert!((kq - 45.0).abs() < 1e-9);
        // SF = 0: KQ = KJ
        let kq = eng.queue_density(0.0, 10.0);
        assert!((kq - 190.0).abs() < 1e-9);
    }

    #[test]
    fn test_undersaturated_passthrough() {
        // Demand below capacity everywhere: no queues, served = demand.
        let mut eng = simple_engine(3);
        let input = simple_input(3, 4000.0, 3000.0);
        let res = eng.run_period(&input);
        for i in 0..3 {
            assert!((res.segment_flow[i] - 3000.0).abs() < 1.0, "seg {i}");
            assert!(!res.had_queue[i]);
            assert!(res.unserved_end[i] < 0.001);
        }
        assert!(res.entry_queue_end[0] < 1e-9);
    }

    #[test]
    fn test_bottleneck_meters_and_queues() {
        // Segment 2 of 3 is a 3,000 veh/h bottleneck with 4,000 veh/h demand.
        let mut eng = simple_engine(3);
        let mut input = simple_input(3, 5000.0, 4000.0);
        input.capacity[1] = 3000.0;
        let res = eng.run_period(&input);

        // Queue accumulates on segment 0 (upstream of the bottleneck).
        assert!(res.had_queue[0], "expected upstream queue");
        assert!(res.unserved_end[0] > 1.0);
        // The bottleneck discharges at most its (dropped) capacity.
        assert!(res.segment_flow[1] <= 3000.0 + 1.0);
        // Downstream is metered below demand.
        assert!(res.segment_flow[2] < 3200.0);
        // Vehicle conservation: unserved = 15-min demand - served
        // (entry queue included).
        let served_in = res.segment_flow[0].min(4000.0);
        assert!(served_in <= 4000.0);
    }

    #[test]
    fn test_queue_discharge_drop_applied() {
        // Once a queue exists upstream, the bottleneck discharges at
        // (1 - alpha) x capacity (Equation 25-29).
        let mut eng = simple_engine(2);
        let mut input = simple_input(2, 5000.0, 4000.0);
        input.capacity[1] = 3000.0;
        // First period: breakdown.
        eng.run_period(&input);
        // Second period: same demand; discharge locked at 0.93 x 3,000.
        let res = eng.run_period(&input);
        assert!(
            (res.segment_flow[1] - 3000.0 * 0.93).abs() < 15.0,
            "expected queue discharge ~2,790, got {}",
            res.segment_flow[1]
        );
    }

    #[test]
    fn test_queue_recovery() {
        // Queue forms in period 1, demand drops, queue clears.
        let mut eng = simple_engine(2);
        let mut congested = simple_input(2, 5000.0, 4000.0);
        congested.capacity[1] = 3000.0;
        eng.run_period(&congested);
        let mut light = simple_input(2, 5000.0, 1000.0);
        light.capacity[1] = 3000.0;
        let res = eng.run_period(&light);
        assert!(res.unserved_end[0] < 0.5, "queue should clear");
        // Volume served in the recovery period exceeds demand (stored
        // vehicles discharged).
        assert!(res.segment_flow[1] > 1000.0);
    }

    #[test]
    fn test_on_ramp_forced_merge_shares_lane1() {
        // Very high mainline + ramp demand at a merge: the ramp gets at
        // least Lambda / 2N (one-to-one merging in Lane 1, Equation 25-18).
        let mut eng = simple_engine(2);
        let mut input = simple_input(2, 4000.0, 3900.0);
        input.onrd[1] = 1000.0;
        input.ramp_capacity[1] = 2000.0;
        input.demand = vec![3900.0, 4900.0];
        let res = eng.run_period(&input);
        // Lambda ~= 4,000 (dropped to 3,720 once queued); ramp share >=
        // Lambda / (2 x 2 lanes) ~= 930 veh/h.
        assert!(
            res.onr_flow[1] > 900.0,
            "ramp should keep ~half of Lane 1, got {}",
            res.onr_flow[1]
        );
        assert!(res.had_queue[0], "mainline should queue upstream");
    }

    #[test]
    fn test_ramp_metering_limits_ramp_flow() {
        let mut eng = simple_engine(2);
        let mut input = simple_input(2, 5000.0, 2000.0);
        input.onrd[1] = 800.0;
        input.ramp_capacity[1] = 2000.0;
        input.ramp_metering[1] = Some(400.0);
        input.demand = vec![2000.0, 2800.0];
        let res = eng.run_period(&input);
        assert!((res.onr_flow[1] - 400.0).abs() < 1.0);
        // Metered vehicles accumulate in the ramp queue:
        // (800 - 400) veh/h x 0.25 h = 100 veh.
        assert!((res.onr_queue_end[1] - 100.0).abs() < 1.0);
    }

    #[test]
    fn test_off_ramp_diverge_percentage() {
        let mut eng = simple_engine(2);
        let mut input = simple_input(2, 5000.0, 3000.0);
        input.offrd[1] = 600.0; // exits at node 1 from segment 0
        input.diverge_pct[1] = 600.0 / 3000.0;
        input.diverge_pct_prev[1] = 600.0 / 3000.0;
        input.demand = vec![3000.0, 2400.0];
        let res = eng.run_period(&input);
        assert!((res.ofr_flow[1] - 600.0).abs() < 1.0);
        assert!((res.segment_flow[1] - 2400.0).abs() < 1.0);
    }

    #[test]
    fn test_equation_25_12_front_clearing() {
        // Capacity recovers and exceeds demand: front-clearing conditions.
        assert!(OversaturatedEngine::front_clearing_queue(
            6000.0, 0.0, 4000.0, 0.0, 5000.0
        ));
        // Capacity did not increase: no front-clearing.
        assert!(!OversaturatedEngine::front_clearing_queue(
            4000.0, 0.0, 4000.0, 0.0, 3000.0
        ));
        // Capacity increased but still below demand: no front-clearing.
        assert!(!OversaturatedEngine::front_clearing_queue(
            5000.0, 0.0, 4000.0, 0.0, 5500.0
        ));
    }
}
