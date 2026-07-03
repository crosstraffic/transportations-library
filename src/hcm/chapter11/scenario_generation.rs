//! HCM Chapter 25, Section 9: Freeway Scenario Generation (the
//! computational core behind Chapter 11 Steps B-4 through B-9).
//!
//! Implements the 34-step hybrid scenario generation process of Exhibit
//! 25-39 (`200_Ch25_09.xhtml`):
//! - demand combinations and scenario probabilities (Steps 2–5; Equations
//!   25-71 through 25-73);
//! - deterministic calendar-based work zone assignment (Steps 6–9;
//!   Equation 25-74);
//! - weather event frequency generation (Steps 10–13; Equations
//!   25-75/25-76) and random assignment to scenarios (Steps 14–18);
//! - incident frequency estimation (Steps 19–24; Equations 25-77 through
//!   25-81), severity generation (Steps 25–26; Equations 25-82 through
//!   25-85), duration generation (Steps 27–28; Equations 25-86/25-87 with
//!   the Exhibit 25-41 lognormal parameters), and start time/location
//!   assignment (Steps 29–34; Equations 25-88 through 25-93).
//!
//! Enumeration vs. sampling: the HCM procedure itself is a hybrid. Event
//! *counts* (how many weather events per month, how many scenarios carry
//! k incidents, severity/duration/location/start-time marginal counts) are
//! generated **deterministically** by the delta-adjusted rounding
//! equations (25-76, 25-80 through 25-93) so that the generated marginal
//! distributions match the inputs exactly — this module reproduces those
//! counts deterministically and they are asserted against the published
//! intermediate tables. Only the *pairing* of events with scenarios,
//! start times, and segments is stochastic (Monte Carlo in FREEVAL); this
//! module uses a small in-module xorshift64* PRNG with an explicit user
//! seed, so results are fully reproducible for a given seed. Published
//! example-problem reliability results (Chapter 25 Example Problem 7)
//! come from FREEVAL's own Monte Carlo stream (seed 1) and are therefore
//! only reproducible at the distribution level, not per scenario.

use serde::{Deserialize, Serialize};

use super::exhibits::{
    weather_caf, weather_saf, IncidentDurationParams, IncidentSeverity, WeatherType,
    DEFAULT_INCIDENT_DURATION_PARAMS, DEFAULT_INCIDENT_SEVERITY_DISTRIBUTION,
    DEFAULT_INCIDENT_TO_CRASH_RATIO, INCIDENT_SEVERITIES, SEVERE_WEATHER_TYPES,
    URBAN_DEMAND_RATIOS,
};

/// Duration of one analysis period, min.
const ANALYSIS_PERIOD_MIN: f64 = 15.0;

// ═════════════════════════════════════════════════════════════════════════
// Seeded PRNG (no external `rand` dependency)
// ═════════════════════════════════════════════════════════════════════════

/// Minimal xorshift64* pseudorandom number generator with an explicit
/// seed, used for the stochastic assignment steps (weather/incident
/// pairing). Deterministic and portable for a given seed.
#[derive(Debug, Clone)]
pub struct Prng {
    state: u64,
}

impl Prng {
    pub fn new(seed: u64) -> Self {
        // Avoid the all-zeros fixed point.
        Self {
            state: if seed == 0 { 0x9E37_79B9_7F4A_7C15 } else { seed },
        }
    }

    pub fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.state = x;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    /// Uniform f64 in [0, 1).
    pub fn next_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }

    /// Uniform integer in [0, n) (n > 0).
    pub fn gen_range(&mut self, n: usize) -> usize {
        (self.next_f64() * n as f64) as usize % n.max(1)
    }

    /// Draw an index from a discrete weight vector (weights >= 0).
    pub fn pick_weighted(&mut self, weights: &[f64]) -> usize {
        let total: f64 = weights.iter().sum();
        if total <= 0.0 || weights.is_empty() {
            return 0;
        }
        let mut r = self.next_f64() * total;
        for (i, w) in weights.iter().enumerate() {
            r -= w;
            if r <= 0.0 {
                return i;
            }
        }
        weights.len() - 1
    }

    /// Fisher–Yates shuffle.
    pub fn shuffle<T>(&mut self, items: &mut [T]) {
        for i in (1..items.len()).rev() {
            let j = self.gen_range(i + 1);
            items.swap(i, j);
        }
    }
}

// ═════════════════════════════════════════════════════════════════════════
// Calendar primitives
// ═════════════════════════════════════════════════════════════════════════

/// Day of week (columns of Exhibits 11-18/11-19).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl Weekday {
    /// Column index Monday = 0 … Sunday = 6.
    pub fn index(self) -> usize {
        match self {
            Weekday::Monday => 0,
            Weekday::Tuesday => 1,
            Weekday::Wednesday => 2,
            Weekday::Thursday => 3,
            Weekday::Friday => 4,
            Weekday::Saturday => 5,
            Weekday::Sunday => 6,
        }
    }
}

/// The five weekdays (the default reliability reporting period includes
/// weekdays only; Chapter 11, Step B-1).
pub const WEEKDAYS: [Weekday; 5] = [
    Weekday::Monday,
    Weekday::Tuesday,
    Weekday::Wednesday,
    Weekday::Thursday,
    Weekday::Friday,
];

// ═════════════════════════════════════════════════════════════════════════
// Inputs
// ═════════════════════════════════════════════════════════════════════════

/// Weather inputs: timewise event probabilities by month and mean event
/// durations (Chapter 11 Step B-6; Chapter 25 Steps 10–18).
///
/// National default probabilities per metropolitan area live in the HCM
/// Volume 4 Technical Reference Library (not transcribed here); they are
/// user inputs. Default CAF/SAF values come from Exhibits 11-20/11-21 at
/// the facility FFS unless overridden.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct WeatherInputs {
    /// Timewise probability P_t{w, j} of each severe weather type
    /// (columns, `SEVERE_WEATHER_TYPES` order) by month (rows, Jan–Dec),
    /// decimal (Equation 25-75).
    pub probabilities_by_month: Vec<Vec<f64>>,
    /// Mean event duration by severe weather type, min (rounded to the
    /// nearest 15-min analysis period during generation; minimum 15 min).
    pub durations_min: Vec<f64>,
    /// Optional CAF override per severe weather type (default: Exhibit
    /// 11-20 at the facility FFS).
    pub caf_override: Option<Vec<f64>>,
    /// Optional SAF override per severe weather type (default: Exhibit
    /// 11-21 at the facility FFS).
    pub saf_override: Option<Vec<f64>>,
    /// Demand adjustment factor during weather events (no national
    /// default; Chapter 11 Step B-6 notes DAFs are user-supplied).
    pub daf: f64,
}

impl Default for WeatherInputs {
    fn default() -> Self {
        Self {
            probabilities_by_month: vec![vec![0.0; SEVERE_WEATHER_TYPES.len()]; 12],
            durations_min: vec![0.0; SEVERE_WEATHER_TYPES.len()],
            caf_override: None,
            saf_override: None,
            daf: 1.0,
        }
    }
}

/// Incident inputs (Chapter 11 Step B-7; Chapter 25 Steps 19–34).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct IncidentInputs {
    /// Directly specified expected incident frequency per study period by
    /// month (n_j of Equation 25-77). Takes precedence over the crash-rate
    /// estimation when provided.
    pub monthly_frequencies: Option<Vec<f64>>,
    /// Local facility-wide crash rate per 100 million VMT (CR_j of
    /// Equation 25-78, constant across months here; use
    /// `monthly_frequencies` for month-varying local data).
    pub crash_rate_per_100mvmt: Option<f64>,
    /// Local incident-to-crash ratio ICR (Equation 25-78; national default
    /// 4.9).
    pub incident_to_crash_ratio: f64,
    /// Incident severity distribution G(i) (Equation 25-82; default
    /// Equation 25-85 national values).
    pub severity_distribution: Vec<f64>,
    /// Lognormal duration parameters by severity (default Exhibit
    /// 25-41 / Exhibit 11-22).
    pub duration_params: Vec<IncidentDurationParams>,
    /// Demand adjustment factor during incidents (user-defined; default 1).
    pub daf: f64,
}

impl Default for IncidentInputs {
    fn default() -> Self {
        Self {
            monthly_frequencies: None,
            crash_rate_per_100mvmt: None,
            incident_to_crash_ratio: DEFAULT_INCIDENT_TO_CRASH_RATIO,
            severity_distribution: DEFAULT_INCIDENT_SEVERITY_DISTRIBUTION.to_vec(),
            duration_params: DEFAULT_INCIDENT_DURATION_PARAMS.to_vec(),
            daf: 1.0,
        }
    }
}

/// A scheduled short-term work zone (Chapter 11 Step B-8; Chapter 25
/// Steps 6–9). Long-term work zones covering the whole RRP belong in the
/// base scenario instead.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct WorkZoneEvent {
    /// Months (1–12) in which the work zone is active.
    pub months: Vec<u32>,
    /// Weekdays on which the work zone is active.
    pub weekdays: Vec<Weekday>,
    /// Ratio r_DC of active days of each weekday type per month to the
    /// total number of that weekday type in the month (Step 7); the work
    /// zone is assigned to `round(r_DC × N_replications)` replications
    /// (Equation 25-74).
    pub active_day_ratio: f64,
    /// Affected segment indices (0-based).
    pub segments: Vec<usize>,
    /// Affected analysis periods (0-based; `None` = whole study period).
    pub periods: Option<Vec<usize>>,
    /// Capacity adjustment factor (e.g., from the Chapter 10 work zone
    /// model, Equations 10-7 through 10-12).
    pub caf: f64,
    /// Speed adjustment factor.
    pub saf: f64,
    /// Demand adjustment factor (user-defined).
    pub daf: f64,
    /// Number of lanes closed (informational; the capacity effect must be
    /// captured in `caf`).
    pub lanes_closed: u32,
}

impl Default for WorkZoneEvent {
    fn default() -> Self {
        Self {
            months: Vec::new(),
            weekdays: WEEKDAYS.to_vec(),
            active_day_ratio: 1.0,
            segments: Vec::new(),
            periods: None,
            caf: 1.0,
            saf: 1.0,
            daf: 1.0,
            lanes_closed: 0,
        }
    }
}

/// A special event (input hook; Chapter 11 Step B-8 guidance). Applied to
/// one specific scenario replication.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct SpecialEvent {
    /// Month (1–12).
    pub month: u32,
    /// Day of week.
    pub weekday: Weekday,
    /// Replication index (0-based) within the demand combination.
    pub replication: u32,
    /// Affected segment indices (0-based; `None` = all).
    pub segments: Option<Vec<usize>>,
    /// Affected analysis periods (0-based; `None` = all).
    pub periods: Option<Vec<usize>>,
    /// Demand adjustment factor (event-generated demand).
    pub daf: f64,
    /// Capacity adjustment factor (e.g., traffic control effects).
    pub caf: f64,
    /// Speed adjustment factor.
    pub saf: f64,
}

impl Default for SpecialEvent {
    fn default() -> Self {
        Self {
            month: 1,
            weekday: Weekday::Monday,
            replication: 0,
            segments: None,
            periods: None,
            daf: 1.0,
            caf: 1.0,
            saf: 1.0,
        }
    }
}

/// Full scenario generation configuration (Chapter 11 Steps B-1 through
/// B-8 inputs).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ScenarioGenerationConfig {
    /// Months (1–12) included in the reliability reporting period.
    pub months: Vec<u32>,
    /// Days of week included (default Monday–Friday).
    pub weekdays: Vec<Weekday>,
    /// Number of replications per demand combination (default 4; Exhibit
    /// 11-9 recommends more for shorter RRPs).
    pub replications: u32,
    /// Number of days n_Day,k in the RRP for each demand combination, in
    /// `months × weekdays` iteration order (month-major). `None` uses the
    /// replication count for every combination, which yields equal
    /// scenario probabilities 1/N_scen (Equation 25-73).
    pub day_counts: Option<Vec<f64>>,
    /// Month (1–12) of the seed (base) dataset date.
    pub seed_month: u32,
    /// Day of week of the seed dataset date.
    pub seed_weekday: Weekday,
    /// Demand multipliers by month (rows, Jan–Dec) and day of week
    /// (columns, Mon–Sun), expressed relative to any common base (only
    /// ratios are used; Equation 25-72). Defaults to the Exhibit 11-18
    /// urban ratios.
    pub demand_multipliers: Vec<Vec<f64>>,
    /// Weather inputs (`None` = no weather events modeled).
    pub weather: Option<WeatherInputs>,
    /// Incident inputs (`None` = no incidents modeled).
    pub incidents: Option<IncidentInputs>,
    /// Scheduled short-term work zones.
    pub work_zones: Vec<WorkZoneEvent>,
    /// Special events.
    pub special_events: Vec<SpecialEvent>,
    /// Seed for the stochastic assignment steps.
    pub rng_seed: u64,
}

impl Default for ScenarioGenerationConfig {
    fn default() -> Self {
        Self {
            months: (1..=12).collect(),
            weekdays: WEEKDAYS.to_vec(),
            replications: 4,
            day_counts: None,
            seed_month: 1,
            seed_weekday: Weekday::Monday,
            demand_multipliers: URBAN_DEMAND_RATIOS.iter().map(|r| r.to_vec()).collect(),
            weather: None,
            incidents: None,
            work_zones: Vec::new(),
            special_events: Vec::new(),
            rng_seed: 1,
        }
    }
}

impl ScenarioGenerationConfig {
    /// Demand multiplier DM for a month (1–12) and weekday.
    pub fn demand_multiplier(&self, month: u32, weekday: Weekday) -> f64 {
        self.demand_multipliers
            .get((month as usize).saturating_sub(1))
            .and_then(|row| row.get(weekday.index()))
            .copied()
            .unwrap_or(1.0)
    }

    /// Demand multiplier of the seed dataset date DM(Seed).
    pub fn seed_demand_multiplier(&self) -> f64 {
        self.demand_multiplier(self.seed_month, self.seed_weekday)
    }
}

// ═════════════════════════════════════════════════════════════════════════
// Seed statistics (from the base dataset)
// ═════════════════════════════════════════════════════════════════════════

/// Aggregates of the seed (base) dataset needed by the scenario generator.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeedStatistics {
    /// Demand VMT by segment and analysis period, veh-mi
    /// (`demand × 0.25 h × segment length`).
    pub vmt: Vec<Vec<f64>>,
    /// Number of analysis periods in the study period.
    pub num_periods: usize,
    /// Number of directional lanes per segment.
    pub lanes: Vec<u32>,
    /// Facility free-flow speed, mi/h (for the Exhibit 11-20/11-21 lookup).
    pub ffs: f64,
}

impl SeedStatistics {
    /// Total seed-file VMT over the study period, veh-mi.
    pub fn total_vmt(&self) -> f64 {
        self.vmt.iter().flatten().sum()
    }

    /// Study period duration D_SP, h.
    pub fn study_period_h(&self) -> f64 {
        self.num_periods as f64 * ANALYSIS_PERIOD_MIN / 60.0
    }

    /// Equation 25-88: probability that an incident occurs on each segment
    /// (proportional to segment VMT across the study period).
    pub fn location_distribution(&self) -> Vec<f64> {
        let total = self.total_vmt();
        self.vmt
            .iter()
            .map(|row| {
                let s: f64 = row.iter().sum();
                if total > 0.0 {
                    s / total
                } else {
                    0.0
                }
            })
            .collect()
    }

    /// Equation 25-89: probability that an incident starts in each analysis
    /// period (proportional to facility VMT by period).
    pub fn start_time_distribution(&self) -> Vec<f64> {
        let total = self.total_vmt();
        (0..self.num_periods)
            .map(|p| {
                let s: f64 = self.vmt.iter().map(|row| row[p]).sum();
                if total > 0.0 {
                    s / total
                } else {
                    0.0
                }
            })
            .collect()
    }
}

// ═════════════════════════════════════════════════════════════════════════
// Generated scenarios
// ═════════════════════════════════════════════════════════════════════════

/// A weather event assigned to a scenario (Steps 13–14).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherEventAssignment {
    /// Weather type.
    pub weather: WeatherType,
    /// First affected analysis period (0-based).
    pub start_period: usize,
    /// Duration in whole analysis periods (>= 1).
    pub duration_periods: usize,
    /// Capacity adjustment factor applied facility-wide while active.
    pub caf: f64,
    /// Speed adjustment factor applied facility-wide while active.
    pub saf: f64,
}

/// An incident assigned to a scenario (Steps 22–34).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncidentAssignment {
    /// Severity (possibly downgraded to keep at least one lane open at the
    /// assigned segment).
    pub severity: IncidentSeverity,
    /// Affected segment (0-based).
    pub segment: usize,
    /// First affected analysis period (0-based).
    pub start_period: usize,
    /// Duration in whole analysis periods (>= 1).
    pub duration_periods: usize,
}

/// One fully characterized reliability scenario (a single study period).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FreewayScenario {
    /// Sequential scenario id (0-based).
    pub id: usize,
    /// Month of year (1–12).
    pub month: u32,
    /// Day of week.
    pub weekday: Weekday,
    /// Replication index within the demand combination (0-based).
    pub replication: u32,
    /// Scenario probability (Equation 25-73). Probabilities over the whole
    /// scenario set sum to 1.
    pub probability: f64,
    /// Demand multiplier DM(s) of the scenario's demand combination.
    pub demand_multiplier: f64,
    /// Facility-wide demand adjustment factor DAF_s = DM(s)/DM(Seed)
    /// (Equation 25-72).
    pub daf: f64,
    /// Weather events active in this scenario.
    pub weather_events: Vec<WeatherEventAssignment>,
    /// Incidents active in this scenario.
    pub incidents: Vec<IncidentAssignment>,
    /// Indices into `ScenarioGenerationConfig::work_zones`.
    pub work_zones: Vec<usize>,
    /// Indices into `ScenarioGenerationConfig::special_events`.
    pub special_events: Vec<usize>,
}

/// Scenario generation output: the scenario list plus deterministic
/// intermediate results useful for verification and reporting.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioSet {
    pub scenarios: Vec<FreewayScenario>,
    /// Expected weather event counts E[n_w,j] by month (1–12 rows) and
    /// severe weather type (Equation 25-76).
    pub expected_weather_events: Vec<Vec<u32>>,
    /// Expected incident frequency n_j per study period by month
    /// (Equation 25-77), zero for months outside the RRP.
    pub monthly_incident_frequency: Vec<f64>,
    /// Total number of generated incidents across all scenarios.
    pub total_incidents: usize,
    /// Total number of generated weather events across all scenarios.
    pub total_weather_events: usize,
}

// ═════════════════════════════════════════════════════════════════════════
// Deterministic count generation (Equations 25-76, 25-80 through 25-93)
// ═════════════════════════════════════════════════════════════════════════

/// Distribute `n` integer counts over categories with probabilities
/// `probs` using the HCM delta-adjustment rounding (Equations 25-80/25-81,
/// 25-83/25-84, 25-86/25-87, 25-90 through 25-93): find delta such that
/// `Σ_i round(delta × n × p_i) = n`.
///
/// `Σ round(delta·n·p_i)` is non-decreasing in delta, so a bisection search
/// is used; when the rounding steps skip the target exactly (no delta
/// achieves equality), the residual difference is resolved by largest-remainder
/// apportionment at the best delta found, which preserves the marginal
/// distribution as closely as possible.
pub fn counts_matching_distribution(probs: &[f64], n: usize) -> Vec<usize> {
    if n == 0 || probs.is_empty() {
        return vec![0; probs.len()];
    }
    let nf = n as f64;
    let counts_at = |delta: f64| -> Vec<usize> {
        probs
            .iter()
            .map(|p| (delta * nf * p).round().max(0.0) as usize)
            .collect()
    };
    let total_at = |delta: f64| -> usize { counts_at(delta).iter().sum() };

    // Bracket the target.
    let (mut lo, mut hi) = (0.0_f64, 1.0_f64);
    while total_at(hi) < n && hi < 1e6 {
        hi *= 2.0;
    }
    for _ in 0..200 {
        let mid = 0.5 * (lo + hi);
        if total_at(mid) < n {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    let mut counts = counts_at(hi);
    let mut total: usize = counts.iter().sum();
    if total != n {
        // Largest-remainder fallback for the residual (at most a few
        // counts when rounding jumps skip the target).
        let raw: Vec<f64> = probs.iter().map(|p| hi * nf * p).collect();
        while total > n {
            // Remove from the category with the largest rounding gain.
            let i = (0..counts.len())
                .filter(|&i| counts[i] > 0)
                .max_by(|&a, &b| {
                    let ga = counts[a] as f64 - raw[a];
                    let gb = counts[b] as f64 - raw[b];
                    ga.partial_cmp(&gb).unwrap_or(std::cmp::Ordering::Equal)
                })
                .unwrap();
            counts[i] -= 1;
            total -= 1;
        }
        while total < n {
            // Add to the category with the largest rounding loss.
            let i = (0..counts.len())
                .max_by(|&a, &b| {
                    let la = raw[a] - counts[a] as f64;
                    let lb = raw[b] - counts[b] as f64;
                    la.partial_cmp(&lb).unwrap_or(std::cmp::Ordering::Equal)
                })
                .unwrap();
            counts[i] += 1;
            total += 1;
        }
    }
    counts
}

/// Poisson probability mass function values for k = 0..=k_max.
pub fn poisson_pmf(mean: f64, k_max: usize) -> Vec<f64> {
    let mut pmf = Vec::with_capacity(k_max + 1);
    let mut term = (-mean).exp();
    pmf.push(term);
    for k in 1..=k_max {
        term *= mean / k as f64;
        pmf.push(term);
    }
    pmf
}

/// Error function approximation (Abramowitz & Stegun 7.1.26; max abs error
/// 1.5e-7), used for the lognormal incident duration CDF.
fn erf(x: f64) -> f64 {
    let sign = if x < 0.0 { -1.0 } else { 1.0 };
    let x = x.abs();
    let t = 1.0 / (1.0 + 0.327_591_1 * x);
    let y = 1.0
        - (((((1.061_405_429 * t - 1.453_152_027) * t) + 1.421_413_741) * t - 0.284_496_736)
            * t
            + 0.254_829_592)
            * t
            * (-x * x).exp();
    sign * y
}

/// Standard normal CDF.
fn normal_cdf(z: f64) -> f64 {
    0.5 * (1.0 + erf(z / std::f64::consts::SQRT_2))
}

/// Lognormal CDF with the given arithmetic mean and standard deviation
/// (moment-matched: sigma² = ln(1 + s²/m²), mu = ln m − sigma²/2).
fn lognormal_cdf(x: f64, mean: f64, std_dev: f64) -> f64 {
    if x <= 0.0 || mean <= 0.0 {
        return 0.0;
    }
    let cv2 = (std_dev / mean).powi(2);
    let sigma2 = (1.0 + cv2).ln();
    let mu = mean.ln() - 0.5 * sigma2;
    normal_cdf((x.ln() - mu) / sigma2.sqrt())
}

/// Discretize a severity type's lognormal duration distribution (Exhibit
/// 25-41 parameters) into 15-min analysis-period bins between the
/// distribution's min and max, returning `(durations_in_periods, probs)`
/// normalized to sum to 1 (Step 27; incident durations must be integer
/// numbers of analysis periods per the Chapter 11 limitations).
pub fn incident_duration_bins(params: &IncidentDurationParams) -> (Vec<usize>, Vec<f64>) {
    let max_periods = ((params.max / ANALYSIS_PERIOD_MIN).ceil() as usize).max(1);
    let min_period = ((params.min / ANALYSIS_PERIOD_MIN).floor() as usize).max(1);
    let mut durations = Vec::new();
    let mut probs = Vec::new();
    for k in min_period..=max_periods {
        let t = k as f64 * ANALYSIS_PERIOD_MIN;
        let lo = t - ANALYSIS_PERIOD_MIN / 2.0;
        let hi = t + ANALYSIS_PERIOD_MIN / 2.0;
        let p = lognormal_cdf(hi, params.mean, params.std_dev)
            - lognormal_cdf(lo, params.mean, params.std_dev);
        durations.push(k);
        probs.push(p.max(0.0));
    }
    let total: f64 = probs.iter().sum();
    if total > 0.0 {
        for p in &mut probs {
            *p /= total;
        }
    } else if !probs.is_empty() {
        probs[0] = 1.0;
    }
    (durations, probs)
}

/// Equation 25-76: expected (rounded) frequency of weather event `w` in a
/// month: `E[n_w,j] = round(P_t{w,j} × D_SP × N_scen,j / E15[D_w])` with
/// `E15[D_w]` the mean event duration rounded to the nearest 15-min
/// increment (minimum 0.25 h), in hours.
pub fn expected_weather_frequency(
    probability: f64,
    study_period_h: f64,
    num_scenarios_in_month: usize,
    duration_min: f64,
) -> u32 {
    if probability <= 0.0 || duration_min <= 0.0 {
        return 0;
    }
    let dur_h = ((duration_min / ANALYSIS_PERIOD_MIN).round().max(1.0)) * ANALYSIS_PERIOD_MIN
        / 60.0;
    (probability * study_period_h * num_scenarios_in_month as f64 / dur_h).round() as u32
}

// ═════════════════════════════════════════════════════════════════════════
// Scenario generation (the 34-step procedure)
// ═════════════════════════════════════════════════════════════════════════

/// Generate the reliability scenario set (Chapter 25, Section 9).
pub fn generate_scenarios(
    config: &ScenarioGenerationConfig,
    seed_stats: &SeedStatistics,
) -> Result<ScenarioSet, String> {
    validate_config(config, seed_stats)?;
    let mut rng = Prng::new(config.rng_seed);

    // ── Steps 2–5: demand combinations, scenario sets, probabilities ────
    let ndc = config.months.len() * config.weekdays.len();
    let nr = config.replications as usize;
    let day_counts: Vec<f64> = match &config.day_counts {
        Some(v) => v.clone(),
        None => vec![config.replications as f64; ndc],
    };
    if day_counts.len() != ndc {
        return Err(format!(
            "day_counts has {} entries but there are {} demand combinations",
            day_counts.len(),
            ndc
        ));
    }
    let total_days: f64 = day_counts.iter().sum();
    let dm_seed = config.seed_demand_multiplier();
    if dm_seed <= 0.0 {
        return Err("seed demand multiplier must be positive".into());
    }

    let mut scenarios = Vec::with_capacity(ndc * nr);
    for (dc, (month, weekday)) in config
        .months
        .iter()
        .flat_map(|m| config.weekdays.iter().map(move |d| (*m, *d)))
        .enumerate()
    {
        let dm = config.demand_multiplier(month, weekday);
        // Equation 25-73.
        let probability = day_counts[dc] / (nr as f64 * total_days);
        for r in 0..nr {
            scenarios.push(FreewayScenario {
                id: scenarios.len(),
                month,
                weekday,
                replication: r as u32,
                probability,
                demand_multiplier: dm,
                daf: dm / dm_seed, // Equation 25-72
                weather_events: Vec::new(),
                incidents: Vec::new(),
                work_zones: Vec::new(),
                special_events: Vec::new(),
            });
        }
    }

    // ── Steps 6–9: deterministic work zone assignment ────────────────────
    for (wz_idx, wz) in config.work_zones.iter().enumerate() {
        // Equation 25-74: adjusted number of replications.
        let n_wz = ((wz.active_day_ratio * nr as f64).round() as usize).min(nr);
        for sc in scenarios.iter_mut() {
            if wz.months.contains(&sc.month)
                && wz.weekdays.contains(&sc.weekday)
                && (sc.replication as usize) < n_wz
            {
                sc.work_zones.push(wz_idx);
            }
        }
    }

    // Special events (input hook).
    for (se_idx, se) in config.special_events.iter().enumerate() {
        for sc in scenarios.iter_mut() {
            if sc.month == se.month
                && sc.weekday == se.weekday
                && sc.replication == se.replication
            {
                sc.special_events.push(se_idx);
            }
        }
    }

    // ── Steps 10–18: weather events ──────────────────────────────────────
    let d_sp = seed_stats.study_period_h();
    let num_periods = seed_stats.num_periods;
    let mut expected_weather_events = vec![vec![0u32; SEVERE_WEATHER_TYPES.len()]; 12];
    let mut total_weather_events = 0usize;

    if let Some(weather) = &config.weather {
        for &month in &config.months {
            let month_ids: Vec<usize> = scenarios
                .iter()
                .filter(|s| s.month == month)
                .map(|s| s.id)
                .collect();
            let nscen_j = month_ids.len();
            let month_probs: Vec<f64> = month_ids
                .iter()
                .map(|&id| scenarios[id].probability)
                .collect();

            for (w, weather_type) in SEVERE_WEATHER_TYPES.iter().enumerate() {
                let p = weather
                    .probabilities_by_month
                    .get(month as usize - 1)
                    .and_then(|row| row.get(w))
                    .copied()
                    .unwrap_or(0.0);
                let dur_min = weather.durations_min.get(w).copied().unwrap_or(0.0);
                // Equation 25-76.
                let n_events = expected_weather_frequency(p, d_sp, nscen_j, dur_min);
                expected_weather_events[month as usize - 1][w] = n_events;
                if n_events == 0 {
                    continue;
                }
                let duration_periods =
                    ((dur_min / ANALYSIS_PERIOD_MIN).round() as usize).max(1);
                let caf = weather
                    .caf_override
                    .as_ref()
                    .and_then(|v| v.get(w).copied())
                    .unwrap_or_else(|| weather_caf(*weather_type, seed_stats.ffs));
                let saf = weather
                    .saf_override
                    .as_ref()
                    .and_then(|v| v.get(w).copied())
                    .unwrap_or_else(|| weather_saf(*weather_type, seed_stats.ffs));

                for _ in 0..n_events {
                    // Steps 14–16: random scenario (probability-weighted)
                    // and uniformly random start time; redraw on temporal
                    // overlap with an already-assigned weather event.
                    let mut placed = false;
                    for _attempt in 0..1000 {
                        let pick = rng.pick_weighted(&month_probs);
                        let sid = month_ids[pick];
                        let start = rng.gen_range(num_periods);
                        let overlaps = scenarios[sid].weather_events.iter().any(|e| {
                            periods_overlap(
                                e.start_period,
                                e.duration_periods,
                                start,
                                duration_periods,
                                num_periods,
                            )
                        });
                        if !overlaps {
                            scenarios[sid].weather_events.push(WeatherEventAssignment {
                                weather: *weather_type,
                                start_period: start,
                                duration_periods,
                                caf,
                                saf,
                            });
                            total_weather_events += 1;
                            placed = true;
                            break;
                        }
                    }
                    if !placed {
                        // Greedy fallback: first scenario/period slot
                        // without overlap (should essentially never occur
                        // for realistic inputs).
                        'outer: for &sid in &month_ids {
                            for start in 0..num_periods {
                                let overlaps =
                                    scenarios[sid].weather_events.iter().any(|e| {
                                        periods_overlap(
                                            e.start_period,
                                            e.duration_periods,
                                            start,
                                            duration_periods,
                                            num_periods,
                                        )
                                    });
                                if !overlaps {
                                    scenarios[sid].weather_events.push(
                                        WeatherEventAssignment {
                                            weather: *weather_type,
                                            start_period: start,
                                            duration_periods,
                                            caf,
                                            saf,
                                        },
                                    );
                                    total_weather_events += 1;
                                    break 'outer;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // ── Steps 19–34: incidents ───────────────────────────────────────────
    let mut monthly_incident_frequency = vec![0.0; 12];
    let mut total_incidents = 0usize;

    if let Some(incidents) = &config.incidents {
        let g = &incidents.severity_distribution;
        if (g.iter().sum::<f64>() - 1.0).abs() > 1e-6 {
            return Err("incident severity distribution must sum to 1.0".into());
        }

        // Step 20: expected incident frequency by month (Equations
        // 25-77/25-78): n_j = IR_j × VMT_j, with VMT_j the average scenario
        // VMT in month j (seed VMT scaled by the month's mean DAF).
        let seed_vmt = seed_stats.total_vmt();
        let mut incident_list: Vec<(usize, usize)> = Vec::new(); // (scenario id, count index)

        for &month in &config.months {
            let month_ids: Vec<usize> = scenarios
                .iter()
                .filter(|s| s.month == month)
                .map(|s| s.id)
                .collect();
            let nscen_j = month_ids.len();
            if nscen_j == 0 {
                continue;
            }
            let n_j = if let Some(freqs) = &incidents.monthly_frequencies {
                freqs.get(month as usize - 1).copied().unwrap_or(0.0)
            } else if let Some(cr) = incidents.crash_rate_per_100mvmt {
                let ir = cr * incidents.incident_to_crash_ratio; // Eq. 25-78
                let mean_daf: f64 = month_ids
                    .iter()
                    .map(|&id| scenarios[id].daf)
                    .sum::<f64>()
                    / nscen_j as f64;
                ir * (seed_vmt * mean_daf) / 1e8 // Eq. 25-77
            } else {
                0.0
            };
            monthly_incident_frequency[month as usize - 1] = n_j;
            if n_j <= 0.0 {
                continue;
            }

            // Step 21 (Equations 25-80/25-81): per-scenario incident counts
            // matching a Poisson distribution with mean n_j.
            let k_max = ((n_j + 6.0 * n_j.sqrt()).ceil() as usize).max(6);
            let pmf = poisson_pmf(n_j, k_max);
            let count_of_k = counts_matching_distribution(&pmf, nscen_j);

            // Step 22: random assignment of counts to scenarios.
            let mut counts: Vec<usize> = count_of_k
                .iter()
                .enumerate()
                .flat_map(|(k, &c)| std::iter::repeat(k).take(c))
                .collect();
            rng.shuffle(&mut counts);
            for (&sid, &k) in month_ids.iter().zip(&counts) {
                for _ in 0..k {
                    incident_list.push((sid, incident_list.len()));
                }
            }
        }

        let n_inc = incident_list.len();
        total_incidents = n_inc;

        if n_inc > 0 {
            // Steps 25–26 (Equations 25-83/25-84): severities.
            let severity_counts = counts_matching_distribution(g, n_inc);
            let mut severities: Vec<IncidentSeverity> = severity_counts
                .iter()
                .enumerate()
                .flat_map(|(i, &c)| std::iter::repeat(INCIDENT_SEVERITIES[i]).take(c))
                .collect();
            rng.shuffle(&mut severities);

            // Steps 27–28 (Equations 25-86/25-87): durations by severity.
            let mut durations: Vec<usize> = vec![1; n_inc];
            for (sev_idx, severity) in INCIDENT_SEVERITIES.iter().enumerate() {
                let members: Vec<usize> = (0..n_inc)
                    .filter(|&i| severities[i] == *severity)
                    .collect();
                if members.is_empty() {
                    continue;
                }
                let params = incidents
                    .duration_params
                    .get(sev_idx)
                    .copied()
                    .unwrap_or(DEFAULT_INCIDENT_DURATION_PARAMS[sev_idx]);
                let (bins, probs) = incident_duration_bins(&params);
                let bin_counts = counts_matching_distribution(&probs, members.len());
                let mut pool: Vec<usize> = bin_counts
                    .iter()
                    .enumerate()
                    .flat_map(|(b, &c)| std::iter::repeat(bins[b]).take(c))
                    .collect();
                rng.shuffle(&mut pool);
                for (&i, &d) in members.iter().zip(&pool) {
                    durations[i] = d;
                }
            }

            // Steps 29–30 (Equations 25-88 through 25-93): location and
            // start time pools.
            let loc_dist = seed_stats.location_distribution();
            let start_dist = seed_stats.start_time_distribution();
            let loc_counts = counts_matching_distribution(&loc_dist, n_inc);
            let start_counts = counts_matching_distribution(&start_dist, n_inc);
            let mut loc_pool: Vec<usize> = loc_counts
                .iter()
                .enumerate()
                .flat_map(|(s, &c)| std::iter::repeat(s).take(c))
                .collect();
            let mut start_pool: Vec<usize> = start_counts
                .iter()
                .enumerate()
                .flat_map(|(p, &c)| std::iter::repeat(p).take(c))
                .collect();
            rng.shuffle(&mut loc_pool);
            rng.shuffle(&mut start_pool);

            // Steps 31–34: pair (scenario, severity, duration) with
            // (location, start), avoiding same-scenario same-segment
            // temporal overlap.
            for (idx, &(sid, _)) in incident_list.iter().enumerate() {
                let severity = severities[idx];
                let duration = durations[idx];
                let mut chosen: Option<(usize, usize)> = None;
                'search: for li in 0..loc_pool.len() {
                    for pi in 0..start_pool.len() {
                        let seg = loc_pool[li];
                        let start = start_pool[pi];
                        let overlaps = scenarios[sid].incidents.iter().any(|inc| {
                            inc.segment == seg
                                && periods_overlap(
                                    inc.start_period,
                                    inc.duration_periods,
                                    start,
                                    duration,
                                    num_periods,
                                )
                        });
                        if !overlaps {
                            chosen = Some((li, pi));
                            break 'search;
                        }
                    }
                }
                let (li, pi) = chosen.unwrap_or((0, 0));
                let seg = loc_pool.remove(li.min(loc_pool.len().saturating_sub(1)));
                let start = start_pool.remove(pi.min(start_pool.len().saturating_sub(1)));
                // Keep at least one lane open at the assigned segment
                // (Chapter 11 limitation: full closures are reassigned to
                // less severe types).
                let severity =
                    super::exhibits::feasible_severity(seed_stats.lanes[seg], severity);
                scenarios[sid].incidents.push(IncidentAssignment {
                    severity,
                    segment: seg,
                    start_period: start,
                    duration_periods: duration,
                });
            }
        }
    }

    Ok(ScenarioSet {
        scenarios,
        expected_weather_events,
        monthly_incident_frequency,
        total_incidents,
        total_weather_events,
    })
}

/// Whether two events overlap in time (both truncated at the study period
/// end).
fn periods_overlap(
    start_a: usize,
    dur_a: usize,
    start_b: usize,
    dur_b: usize,
    num_periods: usize,
) -> bool {
    let end_a = (start_a + dur_a).min(num_periods);
    let end_b = (start_b + dur_b).min(num_periods);
    start_a < end_b && start_b < end_a
}

fn validate_config(
    config: &ScenarioGenerationConfig,
    seed_stats: &SeedStatistics,
) -> Result<(), String> {
    if config.months.is_empty() {
        return Err("RRP must include at least one month".into());
    }
    if config.weekdays.is_empty() {
        return Err("RRP must include at least one day of week".into());
    }
    if config.replications == 0 {
        return Err("number of replications must be positive".into());
    }
    if seed_stats.num_periods == 0 {
        return Err("seed dataset has no analysis periods".into());
    }
    for m in &config.months {
        if !(1..=12).contains(m) {
            return Err(format!("invalid month {m}"));
        }
    }
    if config.demand_multipliers.len() != 12 {
        return Err("demand_multipliers must have 12 rows (Jan-Dec)".into());
    }
    if let Some(w) = &config.weather {
        if w.probabilities_by_month.len() != 12 {
            return Err("weather probabilities_by_month must have 12 rows".into());
        }
        if w.durations_min.len() != SEVERE_WEATHER_TYPES.len() {
            return Err(format!(
                "weather durations_min must have {} entries",
                SEVERE_WEATHER_TYPES.len()
            ));
        }
    }
    Ok(())
}
