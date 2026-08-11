//! # Pedestrian mode at TWSC intersections (HCM Chapter 20, Section 5)
//!
//! Implements the seven-step pedestrian methodology of HCM Exhibit 20-27 for a
//! pedestrian crossing the uncontrolled (major-street) traffic stream at a
//! two-way STOP-controlled intersection. The same procedure applies to midblock
//! pedestrian crossings.
//!
//! This is a different procedure from the pedestrian-*impedance* extension in
//! [`super::twsc`] (Section 4, Equations 20-67 through 20-75), where pedestrian
//! volumes v13-v16 reduce vehicular movement capacity. Here the pedestrian is
//! the subject and the service measure is the proportion of pedestrians who
//! would rate the crossing "dissatisfied" or worse.
//!
//! ## Computational steps (HCM Exhibit 20-27)
//!
//! 1. Identify two-stage crossings (a median refuge splits the crossing into
//!    stages, each with its own length and conflicting flow)
//! 2. Determine critical headway (Equations 20-76 through 20-79)
//! 3. Estimate probability of a delayed crossing (Equations 20-80 and 20-81)
//! 4. Calculate average delay to wait for an adequate gap (Equations 20-82 and
//!    20-83)
//! 5. Calculate average pedestrian delay for the crossing stage, including the
//!    reduction from yielding motorists (Equations 20-84 through 20-93)
//! 6. Calculate average pedestrian delay over all stages (Equation 20-94)
//! 7. Calculate pedestrian satisfaction probabilities and determine LOS
//!    (Equations 20-95 through 20-99, Exhibit 20-3)
//!
//! Steps 2 through 5 run per crossing stage; Steps 6 and 7 aggregate.

use serde::{Deserialize, Serialize};

use crate::hcm::common::LevelOfService;

/// Minimum conflicting flow rate, veh/s. The HCM text following Equation 20-78
/// recommends this floor so that the 1/v terms in Equations 20-82 and 20-85
/// cannot divide by zero.
const MIN_CONFLICTING_FLOW_VEH_S: f64 = 0.0001;

/// Maximum motorist yield rate (decimal). The HCM text following Equation 20-87
/// requires a 100% yield rate to be entered as 99.99%, because the
/// `(1 - M_y)^(i-1)` term is undefined as 0^0 at i = 1.
const MAX_YIELD_RATE: f64 = 0.9999;

/// Default clear effective width used by a single pedestrian to avoid
/// interference when passing others, ft (the 8.0 constant of Equation 20-77).
const CLEAR_EFFECTIVE_WIDTH_FT: f64 = 8.0;

// ═══════════════════════════════════════════════════════════════════════════════
// Inputs
// ═══════════════════════════════════════════════════════════════════════════════

/// One stage of a pedestrian crossing (HCM Chapter 20, Section 5, Step 1).
///
/// A crossing without a median refuge has a single stage spanning the whole
/// street. A crossing with a median refuge has one stage per side, each with the
/// through lanes and conflicting flow of that side only.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct PedestrianCrossingStage {
    /// Crosswalk length for this stage L, ft (Equation 20-76).
    pub crossing_length_ft: f64,
    /// Conflicting vehicular flow rate for this stage v, veh/h. Combined both
    /// directions for a one-stage crossing; the flow of the crossed side only
    /// for a stage of a two-stage crossing (Equations 20-78, 20-80, 20-82,
    /// 20-85).
    pub conflicting_flow_veh_h: f64,
    /// Number of through lanes crossed in this stage N_L (Equations 20-80 and
    /// 20-81).
    pub through_lanes: u32,
}

impl Default for PedestrianCrossingStage {
    fn default() -> Self {
        Self {
            crossing_length_ft: 0.0,
            conflicting_flow_veh_h: 0.0,
            through_lanes: 2,
        }
    }
}

/// Inputs for the HCM Chapter 20, Section 5 pedestrian mode at one TWSC
/// crossing (or midblock crossing).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct PedestrianCrossing {
    /// Crossing stages in travel order (one entry for a single-stage crossing,
    /// two for a crossing with a median refuge).
    pub stages: Vec<PedestrianCrossingStage>,
    /// Average pedestrian walking speed S_p, ft/s (Equation 20-76). Defaults to
    /// the 3.5 ft/s 15th-percentile design value discussed under Exhibit 20-26;
    /// the measured average for uncontrolled crossings is higher (4.7 ft/s).
    pub walk_speed_fps: f64,
    /// Pedestrian start-up time and end clearance time t_s, s
    /// (Equation 20-76). Defaults to the 3.0 s conservative design value under
    /// Exhibit 20-26; the observed average is 0 s.
    pub startup_clearance_s: f64,
    /// Motorist yield rate M_y (decimal, not percent). Exhibit 20-28 gives
    /// average observed rates by crossing treatment. Values above
    /// [`MAX_YIELD_RATE`] are clamped per the HCM note on Equation 20-87.
    pub motorist_yield_rate: f64,
    /// Whether pedestrians are observed crossing in platoons. When false the
    /// spatial distribution N_p is taken as one row and Equations 20-77 and
    /// 20-78 are skipped, per the Step 2 text.
    pub pedestrian_platooning: bool,
    /// Crosswalk width W_c, ft (Equation 20-77). Used only when
    /// `pedestrian_platooning` is true.
    pub crosswalk_width_ft: f64,
    /// Pedestrian flow rate v_p, p/h (Equation 20-78). Used only when
    /// `pedestrian_platooning` is true. Converted to p/s internally.
    pub pedestrian_flow_p_h: f64,
    /// Peak hour volume of the street being crossed, both directions, veh/h.
    /// Used with `k_factor` to estimate AADT for Equation 20-95 when
    /// `aadt_veh` is not given.
    pub peak_hour_volume_veh_h: f64,
    /// K-factor (proportion of AADT occurring in the peak hour). Used with
    /// `peak_hour_volume_veh_h` to estimate AADT for Equation 20-95.
    pub k_factor: f64,
    /// AADT of the street being crossed, veh/day. Overrides the
    /// `peak_hour_volume_veh_h` / `k_factor` estimate when given.
    pub aadt_veh: Option<f64>,
    /// Indicator I_RRFB: a rectangular rapid-flashing beacon is present at the
    /// crossing (Equation 20-95).
    pub has_rrfb: bool,
    /// Indicator I_MC: the crosswalk is marked (Equation 20-95).
    pub has_marked_crosswalk: bool,
    /// Indicator I_MR: a median refuge is present (Equation 20-95).
    pub has_median_refuge: bool,
}

impl Default for PedestrianCrossing {
    fn default() -> Self {
        Self {
            stages: Vec::new(),
            walk_speed_fps: 3.5,
            startup_clearance_s: 3.0,
            motorist_yield_rate: 0.0,
            pedestrian_platooning: false,
            crosswalk_width_ft: 0.0,
            pedestrian_flow_p_h: 0.0,
            peak_hour_volume_veh_h: 0.0,
            k_factor: 0.0,
            aadt_veh: None,
            has_rrfb: false,
            has_marked_crosswalk: false,
            has_median_refuge: false,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Results
// ═══════════════════════════════════════════════════════════════════════════════

/// Per-stage results of the Step 2 through Step 5 calculations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PedestrianStageResult {
    /// Critical headway for a single pedestrian t_c, s (Equation 20-76).
    pub critical_headway: f64,
    /// Total pedestrians in the crossing platoon N_c, p (Equation 20-78).
    /// 1.0 when no platooning is observed.
    pub platoon_size: f64,
    /// Spatial distribution of pedestrians N_p, rows (Equation 20-77).
    pub spatial_distribution: f64,
    /// Group critical headway t_c,G, s (Equation 20-79).
    pub group_critical_headway: f64,
    /// Probability of a blocked lane P_b (Equation 20-80).
    pub prob_blocked_lane: f64,
    /// Probability of a delayed crossing P_d (Equation 20-81).
    pub prob_delayed_crossing: f64,
    /// Average pedestrian gap delay d_g, s (Equation 20-82).
    pub gap_delay: f64,
    /// Average gap delay for pedestrians who incur nonzero delay d_gd, s
    /// (Equation 20-83).
    pub gap_delay_when_delayed: f64,
    /// Average headway of those headways less than the group critical headway
    /// h, s (Equation 20-85).
    pub average_short_headway: f64,
    /// Average number of potential yielding events before an adequate gap is
    /// available n = int(d_gd / h) (Equation 20-84).
    pub yield_events: u32,
    /// Probability that motorists yield on potential yielding event i, P(Y_i),
    /// indexed from i = 0 through i = n (Equations 20-86 through 20-93).
    /// P(Y_0) is always 0.0.
    pub prob_yield: Vec<f64>,
    /// Average pedestrian delay for this crossing stage d_p,s, s
    /// (Equation 20-84).
    pub delay: f64,
}

/// Result of a full HCM Chapter 20, Section 5 pedestrian crossing evaluation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PedestrianCrossingAnalysis {
    /// Per-stage results in travel order (Steps 2 through 5).
    pub stages: Vec<PedestrianStageResult>,
    /// Average pedestrian control delay for the whole crossing d_p, s
    /// (Equation 20-94).
    pub delay: f64,
    /// Interpretation of the control delay (Exhibit 20-29).
    pub delay_interpretation: String,
    /// Odds of being satisfied when the crossing is not delayed, O(S/D) with
    /// I_NY = 0 (Equation 20-95).
    pub odds_satisfied_no_delay: f64,
    /// Probability of a "satisfied" rating when no delay occurs P(S, no delay)
    /// (Equation 20-96).
    pub prob_satisfied_no_delay: f64,
    /// Probability of a "dissatisfied" rating when no delay occurs
    /// P(D, no delay) (Equation 20-97).
    pub prob_dissatisfied_no_delay: f64,
    /// Odds of being satisfied when the crossing is delayed, O(S/D) with
    /// I_NY = 1 (Equation 20-95).
    pub odds_satisfied_delay: f64,
    /// Probability of a "satisfied" rating when a delayed crossing occurs
    /// P(S, delay) (Equation 20-96).
    pub prob_satisfied_delay: f64,
    /// Probability of a "dissatisfied" rating when a delayed crossing occurs
    /// P(D, delay) (Equation 20-97).
    pub prob_dissatisfied_delay: f64,
    /// Probability of a potentially delayed crossing P_d used in Step 7
    /// (Equation 20-81, governing stage).
    pub prob_delayed_crossing: f64,
    /// Probability that all blocking vehicles yield on the first potential
    /// yielding event P(Y_1) used in Step 7 (governing stage).
    pub prob_yield_first_event: f64,
    /// Probability of a non-delayed crossing P_nd (Equation 20-98).
    pub prob_non_delayed: f64,
    /// Average proportion of "dissatisfied" ratings P_D (Equation 20-99).
    pub proportion_dissatisfied: f64,
    /// Pedestrian LOS (Exhibit 20-3).
    pub los: LevelOfService,
}

// ═══════════════════════════════════════════════════════════════════════════════
// Step 7 satisfaction model coefficients (Equation 20-95)
// ═══════════════════════════════════════════════════════════════════════════════

/// Intercept of Equation 20-95.
const SAT_INTERCEPT: f64 = 0.9951;
/// Coefficient on V_KAADT (AADT in 1000s of veh) in Equation 20-95.
const SAT_COEF_AADT: f64 = -0.0438;
/// Coefficient on I_RRFB in Equation 20-95.
const SAT_COEF_RRFB: f64 = 1.9572;
/// Coefficient on I_MC in Equation 20-95.
const SAT_COEF_MARKED_CROSSWALK: f64 = 0.9843;
/// Coefficient on I_MR in Equation 20-95.
///
/// VERIFY-HCM: Equation 20-95 is typeset in the HCM 7th Edition PDF inside a
/// horizontally scrollable box that is clipped after the `0.9843 I_MC` term, so
/// the I_MR and I_NY coefficients do not appear anywhere in the Chapter 20 text
/// (the equation is an image with no text layer behind the clip). Both were
/// recovered by solving Equation 20-95 against the six published O(S/D) values
/// of Chapter 32, TWSC Example Problem 2 (Scenario A in the Step 7 prose,
/// Scenarios B and C in Exhibit 32-7), which over-determine the two unknowns.
/// The least-squares fit reproduces all six published odds to within their
/// four-significant-figure rounding. Replace with the published coefficients if
/// an uncut copy of Equation 20-95 becomes available.
const SAT_COEF_MEDIAN_REFUGE: f64 = 1.5490;
/// Coefficient on I_NY in Equation 20-95. Recovered together with
/// [`SAT_COEF_MEDIAN_REFUGE`]; see that constant's VERIFY-HCM note.
const SAT_COEF_NOT_YIELDING: f64 = -1.9043;

// ═══════════════════════════════════════════════════════════════════════════════
// Exhibits
// ═══════════════════════════════════════════════════════════════════════════════

/// Pedestrian LOS from the average proportion of "dissatisfied" ratings P_D
/// (HCM Exhibit 20-3).
pub fn pedestrian_los(proportion_dissatisfied: f64) -> LevelOfService {
    match proportion_dissatisfied {
        p if p < 0.05 => LevelOfService::A,
        p if p < 0.15 => LevelOfService::B,
        p if p < 0.25 => LevelOfService::C,
        p if p < 0.33 => LevelOfService::D,
        p if p < 0.50 => LevelOfService::E,
        _ => LevelOfService::F,
    }
}

/// Interpretation of a pedestrian control delay (HCM Exhibit 20-29). This is
/// commentary on the delay, not a LOS determination; LOS comes from
/// [`pedestrian_los`].
pub fn delay_interpretation(delay_s: f64) -> &'static str {
    match delay_s {
        d if d <= 5.0 => "Usually no conflicting traffic",
        d if d <= 10.0 => "Occasionally some delay due to conflicting traffic",
        d if d <= 20.0 => "Delay noticeable to pedestrians, but not inconveniencing",
        d if d <= 30.0 => "Delay noticeable and irritating, increased likelihood of risk taking",
        d if d <= 45.0 => "Delay approaches tolerance level, risk-taking behavior likely",
        _ => "Delay exceeds tolerance level, high likelihood of pedestrian risk taking",
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Step 5: probability that motorists yield (Equations 20-86 through 20-93)
// ═══════════════════════════════════════════════════════════════════════════════

/// Probability that every blocking vehicle yields on a single potential
/// yielding event, for a crossing of `n_lanes` through lanes.
///
/// This is the bracketed numerator shared by Equations 20-88 through 20-93. The
/// HCM writes it out once per lane count: `P_b M_y` for one lane (implicit in
/// Equation 20-86, where P_d = P_b when N_L = 1), Equation 20-88 for two lanes,
/// Equation 20-90 for three, and Equation 20-92 for four. All four are the same
/// binomial sum over the number of blocked lanes k,
///
/// ```text
///   sum_{k=1}^{N_L} C(N_L, k) P_b^k (1 - P_b)^(N_L - k) M_y^k
/// ```
///
/// which this function evaluates directly rather than transcribing four
/// special cases.
///
/// VERIFY-HCM: Equations 20-91 and 20-92 are clipped by the same scrollable-box
/// typesetting that truncates Equation 20-95, but unlike Equation 20-95 both are
/// fully recoverable from the chapter itself. The Equation 20-91 numerator is
/// the complete Equation 20-90, and the truncated Equation 20-92 appears in
/// full as the numerator of Equation 20-93. The binomial form above was checked
/// term by term against the intact Equations 20-88, 20-90, and 20-93.
///
/// VERIFY-HCM: The HCM enumerates one- through four-lane crossings only. The
/// binomial form extends to any lane count and is used here without a cap, but
/// five or more through lanes in a single crossing stage is outside the range
/// the book states.
fn prob_all_blocking_yield(n_lanes: u32, p_b: f64, m_y: f64) -> f64 {
    let n = n_lanes as i64;
    let mut total = 0.0;
    let mut binom = 1.0_f64;
    for k in 1..=n {
        // C(n, k) built up iteratively from C(n, k-1).
        binom = binom * ((n - k + 1) as f64) / (k as f64);
        total += binom * p_b.powi(k as i32) * (1.0 - p_b).powi((n - k) as i32) * m_y.powi(k as i32);
    }
    total
}

impl PedestrianCrossing {
    pub fn new() -> Self {
        Self::default()
    }

    /// AADT of the street being crossed in 1000s of vehicles, V_KAADT
    /// (Equation 20-95). Uses `aadt_veh` when given, otherwise the peak hour
    /// volume divided by the K-factor, as in Chapter 32, TWSC Example
    /// Problem 2 Step 7.
    fn v_kaadt(&self) -> f64 {
        let aadt = match self.aadt_veh {
            Some(a) => a,
            None if self.k_factor > 0.0 => self.peak_hour_volume_veh_h / self.k_factor,
            None => 0.0,
        };
        aadt / 1000.0
    }

    /// Odds that a pedestrian is satisfied relative to being dissatisfied,
    /// O(S/D) (Equation 20-95). `not_yielding` is the I_NY indicator.
    fn satisfaction_odds(&self, not_yielding: bool) -> f64 {
        let exponent = SAT_INTERCEPT
            + SAT_COEF_AADT * self.v_kaadt()
            + SAT_COEF_RRFB * f64::from(self.has_rrfb)
            + SAT_COEF_MARKED_CROSSWALK * f64::from(self.has_marked_crosswalk)
            + SAT_COEF_MEDIAN_REFUGE * f64::from(self.has_median_refuge)
            + SAT_COEF_NOT_YIELDING * f64::from(not_yielding);
        exponent.exp()
    }

    /// Steps 2 through 5 for one crossing stage.
    fn analyze_stage(&self, stage: &PedestrianCrossingStage) -> PedestrianStageResult {
        // Conflicting flow in veh/s, floored per the HCM note on Equation 20-78.
        let v = (stage.conflicting_flow_veh_h / 3600.0).max(MIN_CONFLICTING_FLOW_VEH_S);
        let m_y = self.motorist_yield_rate.clamp(0.0, MAX_YIELD_RATE);
        let n_l = stage.through_lanes.max(1);

        // Equation 20-76: critical headway for a single pedestrian.
        let t_c = if self.walk_speed_fps > 0.0 {
            stage.crossing_length_ft / self.walk_speed_fps + self.startup_clearance_s
        } else {
            self.startup_clearance_s
        };

        // Equations 20-77 and 20-78: platoon size and spatial distribution.
        // The Step 2 text takes N_p as one row when no pedestrian grouping is
        // observed, which skips both equations.
        let (n_c, n_p) = if self.pedestrian_platooning && self.crosswalk_width_ft > 0.0 {
            let v_p = self.pedestrian_flow_p_h / 3600.0;
            // Equation 20-78: total pedestrians in the crossing platoon.
            let numerator = v_p * (v_p * t_c).exp() + v * (-v * t_c).exp();
            let denominator = (v_p + v) * ((v_p - v) * t_c).exp();
            let n_c = if denominator > 0.0 {
                numerator / denominator
            } else {
                1.0
            };
            // Equation 20-77: spatial distribution, in pedestrian rows.
            //
            // VERIFY-HCM: N_p is described as a count of pedestrian rows but
            // Equation 20-77 is a ratio and the HCM neither rounds nor
            // truncates it. It is carried as a real number here, which is what
            // Equation 20-79 consumes.
            let n_p = (CLEAR_EFFECTIVE_WIDTH_FT * n_c / self.crosswalk_width_ft).max(1.0);
            (n_c, n_p)
        } else {
            (1.0, 1.0)
        };

        // Equation 20-79: group critical headway.
        let t_c_g = t_c + 2.0 * (n_p - 1.0);

        // Equation 20-80: probability of a blocked lane.
        let p_b = 1.0 - (-t_c_g * v / f64::from(n_l)).exp();
        // Equation 20-81: probability of a delayed crossing.
        let p_d = 1.0 - (1.0 - p_b).powi(n_l as i32);

        // Equation 20-82: average pedestrian gap delay.
        let d_g = ((v * t_c_g).exp() - v * t_c_g - 1.0) / v;
        // Equation 20-83: average gap delay given nonzero delay.
        let d_gd = if p_d > 0.0 { d_g / p_d } else { 0.0 };

        // Equation 20-85: average of those headways shorter than t_c,G.
        let exp_neg = (-v * t_c_g).exp();
        let h = if (1.0 - exp_neg).abs() > f64::EPSILON {
            (1.0 / v - (t_c_g + 1.0 / v) * exp_neg) / (1.0 - exp_neg)
        } else {
            0.0
        };

        // n = int(d_gd / h): average number of potential yielding events before
        // an adequate gap is available (Equation 20-84).
        let n_events = if h > 0.0 {
            (d_gd / h).floor().max(0.0) as u32
        } else {
            0
        };

        // Equations 20-86 through 20-93: P(Y_i) for i = 0..n. P(Y_0) = 0.0 per
        // the Step 5 text ("The probability of yielding P(Y_0) when there are
        // no potential yielding events equals 0.0 regardless of how many lanes
        // are crossed"). Every lane count uses the same recursion: the yield
        // probability for event i is the residual probability mass not already
        // consumed by events 0..i-1, scaled by the single-event yield
        // probability normalized on P_d.
        let per_event = prob_all_blocking_yield(n_l, p_b, m_y);
        let scale = if p_d > 0.0 { per_event / p_d } else { 0.0 };
        let mut prob_yield = Vec::with_capacity(n_events as usize + 1);
        prob_yield.push(0.0);
        let mut cumulative = 0.0;
        for _ in 1..=n_events {
            let p_yi = (p_d - cumulative) * scale;
            cumulative += p_yi;
            prob_yield.push(p_yi);
        }

        // Equation 20-84: average pedestrian delay for the crossing stage. The
        // first term is the expected delay from crossings on a yielding event,
        // the second the expected delay from waiting out an adequate gap.
        let yielding_delay: f64 = prob_yield
            .iter()
            .enumerate()
            .map(|(i, p)| h * (i as f64 - 0.5) * p)
            .sum();
        let gap_wait_delay = (p_d - cumulative) * d_gd;
        let delay = yielding_delay + gap_wait_delay;

        PedestrianStageResult {
            critical_headway: t_c,
            platoon_size: n_c,
            spatial_distribution: n_p,
            group_critical_headway: t_c_g,
            prob_blocked_lane: p_b,
            prob_delayed_crossing: p_d,
            gap_delay: d_g,
            gap_delay_when_delayed: d_gd,
            average_short_headway: h,
            yield_events: n_events,
            prob_yield,
            delay,
        }
    }

    /// Full pedestrian crossing evaluation (Steps 1 through 7).
    pub fn analyze(&self) -> PedestrianCrossingAnalysis {
        // Steps 2-5, once per crossing stage identified in Step 1.
        let stages: Vec<PedestrianStageResult> =
            self.stages.iter().map(|s| self.analyze_stage(s)).collect();

        // Equation 20-94: average pedestrian delay summed over the stages. For
        // a one-stage crossing this is d_p,1, as the Step 6 text states.
        let delay: f64 = stages.iter().map(|s| s.delay).sum();

        // Step 7. Equations 20-98 and 20-99 take a single P_d and P(Y_1), but a
        // two-stage crossing produces one of each per stage and the HCM does
        // not say how to combine them.
        //
        // VERIFY-HCM: the first stage's values are used here. Chapter 32, TWSC
        // Example Problem 2 rules out the obvious alternative of treating a
        // non-delayed crossing as one that is undelayed at every stage: its two
        // stages are identical with P_nd = 0.481 per stage, and Exhibit 32-7
        // reports P_nd = 0.481, not the across-stage product 0.481^2 = 0.231.
        // The example cannot discriminate between the first stage, the last,
        // and the worst, because its two stages are identical by construction.
        let (p_d, p_y1) = match stages.first() {
            Some(s) => (
                s.prob_delayed_crossing,
                s.prob_yield.get(1).copied().unwrap_or(0.0),
            ),
            None => (0.0, 0.0),
        };

        // Equation 20-95 with I_NY = 0 and 1, then Equations 20-96 and 20-97.
        let odds_no_delay = self.satisfaction_odds(false);
        let p_s_no_delay = odds_no_delay / (odds_no_delay + 1.0);
        let odds_delay = self.satisfaction_odds(true);
        let p_s_delay = odds_delay / (odds_delay + 1.0);

        // Equation 20-98: probability of a non-delayed crossing.
        let p_nd = (1.0 - p_d) + p_d * p_y1;

        let p_d_no_delay = 1.0 - p_s_no_delay;
        let p_d_delay = 1.0 - p_s_delay;
        // Equation 20-99: volume-weighted average proportion dissatisfied.
        let proportion_dissatisfied = p_nd * p_d_no_delay + (1.0 - p_nd) * p_d_delay;

        PedestrianCrossingAnalysis {
            stages,
            delay,
            delay_interpretation: delay_interpretation(delay).to_string(),
            odds_satisfied_no_delay: odds_no_delay,
            prob_satisfied_no_delay: p_s_no_delay,
            prob_dissatisfied_no_delay: p_d_no_delay,
            odds_satisfied_delay: odds_delay,
            prob_satisfied_delay: p_s_delay,
            prob_dissatisfied_delay: p_d_delay,
            prob_delayed_crossing: p_d,
            prob_yield_first_event: p_y1,
            prob_non_delayed: p_nd,
            proportion_dissatisfied,
            los: pedestrian_los(proportion_dissatisfied),
        }
    }

    /// Deserialize a crossing configuration from JSON.
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Serialize the crossing configuration to JSON.
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}
