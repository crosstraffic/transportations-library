//! # Urban Street Reliability and ATDM (HCM Chapter 17), core methodology
//!
//! Implements the HCM 7th Edition Chapter 17, Section 3 core methodology
//! (EPUB `121_Ch17_03.xhtml`) with the computational details of Chapter 29
//! (Urban Street Facilities: Supplemental), Section 2, Scenario Generation
//! Procedure (`227_Ch29_02.xhtml`):
//!
//! 1. **Weather event generation** (Equations 29-1 through 29-12) — a
//!    day-by-day Monte Carlo prediction of rain/snow events over a 2-year
//!    record (dates, types, rates, durations, wet-pavement time).
//! 2. **Traffic demand variation generation** — hour-of-day, day-of-week,
//!    and month-of-year demand ratios (Exhibits 17-5 through 17-7) plus
//!    the weather demand change factors (Exhibit 17-8).
//! 3. **Traffic incident generation** (Equations 29-13 through 29-24) —
//!    equivalent crash frequencies by weather, hourly Poisson incident
//!    occurrence for the 12 incident types at every segment and
//!    intersection, gamma-distributed durations, and volume-weighted
//!    location assignment.
//! 4. **Scenario dataset generation** (Equations 29-25 through 29-36) —
//!    one dataset per analysis period with demand, saturation flow,
//!    free-flow speed ("other delay"), and lane adjustments.
//!
//! Each scenario (analysis period) is evaluated with the Chapter 16/18
//! facility methodology; the per-scenario facility travel times feed the
//! shared [`crate::hcm::common::reliability::TravelTimeDistribution`], from
//! which the Chapter 17 performance measures (mean/50th/80th/95th
//! percentile TTI, urban reliability rating at TTI < 2.5) are computed.
//! The travel time index baseline is the facility travel time at the base
//! free-flow speed ("For urban street facilities, the base free-flow speed
//! is used as the ideal value on which all speed- and travel-time-related
//! reliability indices are based", Chapter 17, Section 3).
//!
//! Determinism: the HCM procedure is Monte Carlo ("A random number seed is
//! used with the Monte Carlo methods ... so that the sequence of random
//! events can be reproduced", Chapter 17, Section 5). Following the
//! Chapter 11 implementation, this module uses the in-crate seeded
//! xorshift64* PRNG ([`Prng`]) with separate seeds for the weather,
//! demand, and incident generators; results are exactly reproducible for
//! given seeds but — as the HCM itself notes for STREETVAL — "evaluating
//! the same dataset and seed number in different software ... may produce
//! results different from those shown [in the printed examples]. Each
//! result, though different, will be equally valid." Published Example
//! Problem 4 outputs are therefore verified at the distribution level;
//! the deterministic sub-computations (crash frequencies, demand ratios,
//! adjustment factors, incident probabilities) are verified exactly.
//!
//! ## Residual-queue carryover between analysis periods
//! The Facility Evaluation stage (Chapter 17, Section 3) states: "The
//! analysis periods are evaluated in chronological order... the initial
//! queue input value for the next analysis period is set equal to the
//! residual queue output for the current analysis period." This is
//! implemented per boundary-intersection through movement (one lane group
//! per segment's downstream signal): each scenario's evaluation computes
//! initial queue delay d3 (Equations 19-44 through 19-49, via
//! [`crate::hcm::common::delay::initial_queue_delay`]) using the queue
//! `Qb` carried in from the prior chronological analysis period, and
//! carries the residual queue `Qe` (Equation 19-45, via
//! [`crate::hcm::common::delay::queue_end_of_period`]) forward to the
//! next. Carryover is scoped to one day's sequence of analysis periods and
//! resets to Qb = 0 at the first period of each day: the reliability
//! reporting period enumerates many nearly-independent days (e.g., 260
//! weekdays for a 7-10 a.m. study period), and a queue could not
//! physically survive the ~21-h gap between one day's last analysis
//! period and the next day's first. This mirrors the Chapter 11 freeway
//! reliability engine, where each scenario (day) is evaluated from a fresh
//! facility clone with no cross-scenario state, and the Chapter 29,
//! Section 3 multiple-time-period/spillback technique, whose queue
//! hand-off is explicitly scoped to "subperiods" of one multi-period
//! analysis rather than across separate days.
//!
//! Documented simplification vs. the full HCM/STREETVAL procedure: the
//! initial-queue extension in Chapter 19, Section 4 also blends a
//! separate "saturated capacity" `cs` (capacity while an unmet-demand
//! backlog is being served) with the ordinary capacity `c` over the
//! unmet-demand duration `t` to obtain the average capacity `cA` used in
//! d2/d3 (Equations 19-38 through 19-43), and re-derives d1 with a
//! saturated/baseline uniform-delay blend (Equations 19-40/19-41). This
//! module uses the scenario's ordinary lane-group capacity directly as
//! `cA` (no saturated/baseline capacity or uniform-delay blending), which
//! the HCM notes is exact when there is no initial queue and is a
//! reasonable approximation otherwise since d2 and d3 are additive and
//! `cA` differs from `c` only during the (typically short) unmet-demand
//! duration within a 15-min period.
//!
//! ## Documented deferrals
//! * Random 15-min demand variation (Equations 29-30 through 29-33) — the
//!   optional randomized flow-rate element; scenarios use the systematic
//!   hour/day/month/weather demand factors only.
//! * Work zones and special events — supported through the
//!   [`AtdmStrategy`] alternative-dataset hook (input-level adjustments
//!   with a schedule), not through full alternative HCM datasets.
//! * ATDM strategy assessment — input-hook level (demand/saturation
//!   flow/green-time/free-flow-speed/crash-frequency adjustments per
//!   strategy schedule) plus the Chapter 37 strategy-impact models in
//!   [`crate::hcm::common::atdm`] (shoulder use, ramp metering duration
//!   reduction, and incident management CFAF/duration adjustments feed
//!   this hook via the strategy's constructors).
//! * Critical left-turn headway adjustment (Exhibit 29-5) — exposed in
//!   [`super::exhibits::exhibit_29_5_extra_lt_headway_s`] for use with the
//!   Chapter 19/20 engines; the facility evaluation here models the
//!   through movement, which the adjustment does not affect directly.

use serde::{Deserialize, Serialize};

use crate::hcm::freeway_reliability::scenario_generation::Prng;
use crate::hcm::urban_facilities::urban_facilities::UrbanFacility;
use crate::hcm::common::delay::{
    incremental_delay, initial_queue_delay, progression_factor, queue_end_of_period, uniform_delay,
};
use crate::hcm::common::reliability::{ReliabilityMetrics, TravelTimeDistribution};

use super::exhibits::{
    additional_delay_s, adjusted_base_ffs, crash_proportion, default_incident_duration_min,
    exhibit_17_5_hour_of_day_ratio, exhibit_17_6_day_of_week_ratio,
    exhibit_17_7_month_of_year_ratio, exhibit_17_9_cfaf, incident_joint_proportions,
    incident_sat_flow_factor, weather_ffs_factor, weather_sat_flow_factor, FunctionalClass,
    IncidentSeverity, IncidentType, LaneLocation, StreetLocation, WeatherCondition,
    DEFAULT_DEMAND_CHANGE_RAIN, DEFAULT_DEMAND_CHANGE_SNOW, DEFAULT_SNOW_PAVEMENT_RUNOFF_H,
    INCIDENT_DURATION_CV, INCIDENT_TYPES, RAIN_PAVEMENT_RUNOFF_H, SNOW_TO_RAIN_DEPTH_RATIO,
    URBAN_RELIABILITY_RATING_TTI_THRESHOLD,
};

/// Analysis period duration, h (15 min; the methodology rounds event
/// durations to this increment).
const ANALYSIS_PERIOD_H: f64 = 0.25;

/// Standard deviation of the daily mean temperature about the monthly
/// normal, °F (Equation 29-3: s_T = 5.0).
const TEMPERATURE_SD_F: f64 = 5.0;

/// Days per month of the (non-leap) modeled year.
const MONTH_DAYS: [u32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

// ═══════════════════════════════════════════════════════════════════════════════
// Statistical inverse CDFs (no external dependencies)
// ═══════════════════════════════════════════════════════════════════════════════

/// Inverse standard normal CDF (Acklam's rational approximation; relative
/// error < 1.15e-9), extended to mean/sd: `normal⁻¹(p, μ, σ)`.
pub fn normal_inverse(p: f64, mean: f64, sd: f64) -> f64 {
    let p = p.clamp(1e-12, 1.0 - 1e-12);
    const A: [f64; 6] = [
        -3.969_683_028_665_376e1,
        2.209_460_984_245_205e2,
        -2.759_285_104_469_687e2,
        1.383_577_518_672_690e2,
        -3.066_479_806_614_716e1,
        2.506_628_277_459_239,
    ];
    const B: [f64; 5] = [
        -5.447_609_879_822_406e1,
        1.615_858_368_580_409e2,
        -1.556_989_798_598_866e2,
        6.680_131_188_771_972e1,
        -1.328_068_155_288_572e1,
    ];
    const C: [f64; 6] = [
        -7.784_894_002_430_293e-3,
        -3.223_964_580_411_365e-1,
        -2.400_758_277_161_838,
        -2.549_732_539_343_734,
        4.374_664_141_464_968,
        2.938_163_982_698_783,
    ];
    const D: [f64; 4] = [
        7.784_695_709_041_462e-3,
        3.224_671_290_700_398e-1,
        2.445_134_137_142_996,
        3.754_408_661_907_416,
    ];
    let p_low = 0.02425;
    let z = if p < p_low {
        let q = (-2.0 * p.ln()).sqrt();
        (((((C[0] * q + C[1]) * q + C[2]) * q + C[3]) * q + C[4]) * q + C[5])
            / ((((D[0] * q + D[1]) * q + D[2]) * q + D[3]) * q + 1.0)
    } else if p <= 1.0 - p_low {
        let q = p - 0.5;
        let r = q * q;
        (((((A[0] * r + A[1]) * r + A[2]) * r + A[3]) * r + A[4]) * r + A[5]) * q
            / (((((B[0] * r + B[1]) * r + B[2]) * r + B[3]) * r + B[4]) * r + 1.0)
    } else {
        let q = (-2.0 * (1.0 - p).ln()).sqrt();
        -(((((C[0] * q + C[1]) * q + C[2]) * q + C[3]) * q + C[4]) * q + C[5])
            / ((((D[0] * q + D[1]) * q + D[2]) * q + D[3]) * q + 1.0)
    };
    mean + sd * z
}

/// Natural log of the gamma function (Lanczos approximation).
fn ln_gamma(x: f64) -> f64 {
    const G: [f64; 6] = [
        76.180_091_729_471_46,
        -86.505_320_329_416_77,
        24.014_098_240_830_91,
        -1.231_739_572_450_155,
        0.120_865_097_386_617_9e-2,
        -0.539_523_938_495_3e-5,
    ];
    let mut y = x;
    let tmp = x + 5.5;
    let tmp = tmp - (x + 0.5) * tmp.ln();
    let mut ser = 1.000_000_000_190_015;
    for g in G {
        y += 1.0;
        ser += g / y;
    }
    -tmp + (2.506_628_274_631_000_5 * ser / x).ln()
}

/// Regularized lower incomplete gamma function P(a, x) (series for
/// x < a + 1, continued fraction otherwise; Numerical Recipes `gammp`).
fn gamma_p(a: f64, x: f64) -> f64 {
    if x <= 0.0 || a <= 0.0 {
        return 0.0;
    }
    if x < a + 1.0 {
        // Series representation.
        let mut ap = a;
        let mut sum = 1.0 / a;
        let mut del = sum;
        for _ in 0..500 {
            ap += 1.0;
            del *= x / ap;
            sum += del;
            if del.abs() < sum.abs() * 1e-14 {
                break;
            }
        }
        sum * (-x + a * x.ln() - ln_gamma(a)).exp()
    } else {
        // Continued fraction for Q(a, x).
        let mut b = x + 1.0 - a;
        let mut c = 1e300;
        let mut d = 1.0 / b;
        let mut h = d;
        for i in 1..500 {
            let an = -(i as f64) * (i as f64 - a);
            b += 2.0;
            d = an * d + b;
            if d.abs() < 1e-300 {
                d = 1e-300;
            }
            c = b + an / c;
            if c.abs() < 1e-300 {
                c = 1e-300;
            }
            d = 1.0 / d;
            let del = d * c;
            h *= del;
            if (del - 1.0).abs() < 1e-14 {
                break;
            }
        }
        1.0 - (-x + a * x.ln() - ln_gamma(a)).exp() * h
    }
}

/// Inverse CDF of a gamma distribution parameterized by its arithmetic
/// mean and standard deviation: `gamma⁻¹(p, μ, σ)` (Equations 29-5, 29-6,
/// and 29-19 use this form; Exhibit 29-72 shows the shape/scale
/// conversion `α = μ²/σ²`, `β = σ²/μ`). Solved by bisection on P(α, x/β).
pub fn gamma_inverse(p: f64, mean: f64, sd: f64) -> f64 {
    if mean <= 0.0 || sd <= 0.0 {
        return mean.max(0.0);
    }
    let p = p.clamp(1e-12, 1.0 - 1e-12);
    let alpha = (mean / sd).powi(2);
    let beta = sd * sd / mean;
    // Bracket the root.
    let mut lo = 0.0_f64;
    let mut hi = mean + 30.0 * sd;
    while gamma_p(alpha, hi / beta) < p {
        hi *= 2.0;
        if hi > 1e12 {
            break;
        }
    }
    for _ in 0..200 {
        let mid = 0.5 * (lo + hi);
        if gamma_p(alpha, mid / beta) < p {
            lo = mid;
        } else {
            hi = mid;
        }
        if hi - lo < 1e-12 * (1.0 + hi) {
            break;
        }
    }
    0.5 * (lo + hi)
}

// ═══════════════════════════════════════════════════════════════════════════════
// Configuration
// ═══════════════════════════════════════════════════════════════════════════════

/// Monthly weather statistics (Chapter 17, Weather Data: "averages by
/// month of year for a recent 10-year period", e.g., the NCDC values in
/// the Volume 4 Technical Reference Library / Exhibit 29-65).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct MonthlyWeather {
    /// Total normal precipitation, in. (rainfall plus water content of
    /// snow).
    pub total_precip_in: f64,
    /// Total normal snowfall, in. (as snow depth).
    pub total_snowfall_in: f64,
    /// Number of days with precipitation of 0.01 in. or more, days.
    pub days_with_precip: f64,
    /// Normal daily mean temperature, °F.
    pub mean_temp_f: f64,
    /// Precipitation rate (while falling), in./h.
    pub precip_rate_in_h: f64,
}

/// Signal-timing data for the through movement at a segment's downstream
/// boundary intersection, used to recompute the through control delay per
/// scenario (Chapter 19 delay equations on the adjusted demand and
/// saturation flow).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct BoundarySignal {
    /// Cycle length C, s.
    pub cycle_length_s: f64,
    /// Effective green time g for the through phase, s.
    pub effective_green_s: f64,
    /// Adjusted saturation flow rate s under base conditions, veh/h/ln.
    pub sat_flow_veh_h_ln: f64,
    /// Platoon ratio R_p for arrivals to the through movement.
    pub platoon_ratio: f64,
    /// Incremental delay factor k (0.50 for pretimed; lower for actuated
    /// phases per Exhibit 19-14).
    pub k_factor: f64,
    /// Upstream filtering factor I (1.0 for an isolated intersection).
    pub i_factor: f64,
    /// Total number of lanes on the subject through approach across all
    /// movements, Σ N_n of Equation 29-27 (defaults to the segment's
    /// through lanes when 0).
    pub approach_lanes: u32,
}

impl Default for BoundarySignal {
    fn default() -> Self {
        Self {
            cycle_length_s: 100.0,
            effective_green_s: 45.0,
            sat_flow_veh_h_ln: 1_900.0,
            platoon_ratio: 1.0,
            k_factor: 0.5,
            i_factor: 1.0,
            approach_lanes: 0,
        }
    }
}

/// Incident-generation inputs (Chapter 17, Incident Data).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct IncidentConfig {
    /// Expected (base) crash frequency per segment, crashes/year (all
    /// severities including PDO; both travel directions).
    pub segment_crash_frequencies: Vec<f64>,
    /// Expected (base) crash frequency per boundary intersection,
    /// crashes/year. Entry `j` is the intersection at the downstream end
    /// of segment `j − 1` (entry 0 is the facility's upstream entry
    /// intersection).
    pub intersection_crash_frequencies: Vec<f64>,
    /// Whether outside shoulders (able to store a disabled vehicle) are
    /// present — selects Exhibit 17-11 vs. Exhibit 17-12.
    pub shoulder_present: bool,
    /// Crash frequency adjustment factor overrides for the four weather
    /// conditions `[rainfall, wet pavement, snowfall, snow/ice]` (default
    /// Exhibit 17-9).
    pub cfaf_override: Option<[f64; 4]>,
    /// Two-way volume of each minor-street leg at the boundary
    /// intersections, veh/h (one value applied to both minor legs of
    /// every intersection), for the Equation 29-20 volume-proportional leg
    /// assignment. Default 0 (all intersection incidents land on the
    /// major-street legs).
    pub minor_leg_volume_veh_h: f64,
    /// Demand flow rate of the opposing direction on each segment, veh/h,
    /// for the Equation 29-23 direction assignment (default: equal to the
    /// subject direction).
    pub opposing_demand_veh_h: Option<Vec<f64>>,
}

impl Default for IncidentConfig {
    fn default() -> Self {
        Self {
            segment_crash_frequencies: Vec::new(),
            intersection_crash_frequencies: Vec::new(),
            shoulder_present: true,
            cfaf_override: None,
            minor_leg_volume_veh_h: 0.0,
            opposing_demand_veh_h: None,
        }
    }
}

/// An ATDM strategy / work zone / special event alternative-dataset hook:
/// input-level adjustments applied to the scenarios matching the
/// schedule. (Chapter 17, Section 4: geometric-configuration and
/// signal-control ATDM strategies "are evaluated in the HCM by using
/// scenarios ... one scenario ... for each desired lane configuration";
/// the full Chapter 37 strategy models are deferred.)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AtdmStrategy {
    /// Strategy label for reporting.
    pub name: String,
    /// Months (1-12) when active; empty = all months.
    pub months: Vec<u32>,
    /// Days of week (0 = Sunday … 6 = Saturday) when active; empty = all.
    pub days_of_week: Vec<u32>,
    /// Analysis-period indices within the study period when active;
    /// empty = all periods.
    pub periods: Vec<usize>,
    /// Multiplicative demand adjustment (e.g., diversion).
    pub demand_adjustment: f64,
    /// Multiplicative saturation flow adjustment at the boundary
    /// intersections.
    pub sat_flow_adjustment: f64,
    /// Additive change to the through phase effective green, s (e.g., the
    /// Example Problem 5 Strategy 1 split reallocation).
    pub effective_green_adjustment_s: f64,
    /// Multiplicative free-flow speed adjustment on the segments.
    pub ffs_adjustment: f64,
    /// Crash frequency adjustment factor while active (CFAF_str of
    /// Equation 29-15; captures work zone / special event / strategy
    /// safety effects, e.g., the HSM-based +11% of Example Problem 5
    /// Strategy 2).
    pub crash_frequency_adjustment: f64,
}

impl Default for AtdmStrategy {
    fn default() -> Self {
        Self {
            name: String::new(),
            months: Vec::new(),
            days_of_week: Vec::new(),
            periods: Vec::new(),
            demand_adjustment: 1.0,
            sat_flow_adjustment: 1.0,
            effective_green_adjustment_s: 0.0,
            ffs_adjustment: 1.0,
            crash_frequency_adjustment: 1.0,
        }
    }
}

impl AtdmStrategy {
    /// An HCM Chapter 37, Section 5 adaptive signal control ATDM strategy.
    /// Sets [`Self::sat_flow_adjustment`] to the
    /// [`crate::hcm::common::atdm::adaptive_signal_sat_flow_adjustment`]
    /// value for `target_delay_reduction_pct` (`None` uses the published
    /// Exhibit 37-9 range's midpoint, 13.5%). See that function's docs for
    /// the VERIFY-HCM caveat: the HCM publishes only an illustrative
    /// simulation-study range (delay reductions of 3%-24%), not a
    /// closed-form method, so this is a documented modeling
    /// simplification, not an HCM-derived equation — prefer a directly
    /// calibrated `sat_flow_adjustment` when available.
    ///
    /// * `name` — strategy label for reporting
    /// * `target_delay_reduction_pct` — desired delay reduction, percent
    /// * `months`, `days_of_week`, `periods` — the strategy's schedule
    ///   (empty = always active, matching [`AtdmStrategy::default`])
    pub fn adaptive_signal_control(
        name: impl Into<String>,
        target_delay_reduction_pct: Option<f64>,
        months: Vec<u32>,
        days_of_week: Vec<u32>,
        periods: Vec<usize>,
    ) -> Self {
        Self {
            name: name.into(),
            months,
            days_of_week,
            periods,
            sat_flow_adjustment: crate::hcm::common::atdm::adaptive_signal_sat_flow_adjustment(
                target_delay_reduction_pct,
            ),
            ..Self::default()
        }
    }
}

/// Full urban street reliability configuration (Chapter 17, Required Data
/// and Sources).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct UrbanReliabilityConfig {
    /// Roadway functional class (Exhibit 17-5/17-7 defaults).
    pub functional_class: FunctionalClass,
    /// Months (1-12) in the reliability reporting period.
    pub months: Vec<u32>,
    /// Days of week (0 = Sunday … 6 = Saturday) in the reliability
    /// reporting period (default weekdays, Mon-Fri).
    pub days_of_week: Vec<u32>,
    /// Day of week of January 1 of the modeled (non-leap) year
    /// (0 = Sunday … 6 = Saturday).
    pub jan1_day_of_week: u32,
    /// Hour of day (0-23) at which the study period starts.
    pub study_period_start_hour: u32,
    /// Number of 15-min analysis periods in the study period (e.g., 12
    /// for a 7-10 a.m. study period).
    pub analysis_periods_per_day: usize,
    /// Monthly weather statistics, Jan-Dec (12 entries). All-zero months
    /// generate no weather events.
    pub weather: Vec<MonthlyWeather>,
    /// Pavement runoff (snowpack persistence) duration after a snow event,
    /// h (Exhibit 17-8 default 0.5).
    pub snow_runoff_h: f64,
    /// Demand change factor for rain events (Exhibit 17-8 default 1.0).
    pub demand_change_rain: f64,
    /// Demand change factor for snow events (Exhibit 17-8 default 0.8).
    pub demand_change_snow: f64,
    /// Month (1-12), day of week (0-6), and hour (0-23) of the traffic
    /// count in the base dataset, defining the base demand ratio the
    /// scenario ratios are normalized to (Equation 29-29).
    pub count_month: u32,
    pub count_day_of_week: u32,
    pub count_hour: u32,
    /// Incident-generation inputs.
    pub incidents: IncidentConfig,
    /// Per-segment boundary-signal data (same order as the facility's
    /// segments) used to recompute the through control delay per scenario.
    pub boundary_signals: Vec<BoundarySignal>,
    /// Seeds for the weather, demand, and incident Monte Carlo streams.
    pub weather_seed: u64,
    pub demand_seed: u64,
    pub incident_seed: u64,
    /// Weight travel-time observations by scenario VMT (default true).
    pub vmt_weighted: bool,
}

impl Default for UrbanReliabilityConfig {
    fn default() -> Self {
        Self {
            functional_class: FunctionalClass::UrbanPrincipalArterial,
            months: (1..=12).collect(),
            days_of_week: vec![1, 2, 3, 4, 5],
            jan1_day_of_week: 0,
            study_period_start_hour: 7,
            analysis_periods_per_day: 12,
            weather: vec![MonthlyWeather::default(); 12],
            snow_runoff_h: DEFAULT_SNOW_PAVEMENT_RUNOFF_H,
            demand_change_rain: DEFAULT_DEMAND_CHANGE_RAIN,
            demand_change_snow: DEFAULT_DEMAND_CHANGE_SNOW,
            count_month: 1,
            count_day_of_week: 2,
            count_hour: 7,
            incidents: IncidentConfig::default(),
            boundary_signals: Vec::new(),
            weather_seed: 82,
            demand_seed: 11,
            incident_seed: 63,
            vmt_weighted: true,
        }
    }
}

impl UrbanReliabilityConfig {
    /// Systematic demand ratio f_hod × f_dow × f_moy for an hour of a
    /// given day (Exhibits 17-5 through 17-7 defaults).
    pub fn demand_ratio(&self, month: u32, day_of_week: u32, hour: u32) -> f64 {
        let weekend = day_of_week == 0 || day_of_week == 6;
        exhibit_17_5_hour_of_day_ratio(self.functional_class, hour, weekend)
            * exhibit_17_6_day_of_week_ratio(day_of_week)
            * exhibit_17_7_month_of_year_ratio(self.functional_class, month)
    }

    /// Base demand ratio of the traffic count in the base dataset
    /// (denominator of Equation 29-29).
    pub fn base_demand_ratio(&self) -> f64 {
        self.demand_ratio(self.count_month, self.count_day_of_week, self.count_hour)
    }

}

// ═══════════════════════════════════════════════════════════════════════════════
// Generated events
// ═══════════════════════════════════════════════════════════════════════════════

/// A generated precipitation event (Chapter 29, Section 2, weather event
/// procedure; cf. Exhibit 29-66).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherEvent {
    /// Day index within the 2-year weather record (0-729).
    pub day: usize,
    /// True for snow (daily mean temperature below 32°F, Equation 29-4).
    pub is_snow: bool,
    /// Average temperature for the day, °F (Equation 29-3).
    pub temperature_f: f64,
    /// Precipitation rate, in./h (snow events: inches of snow per hour).
    pub precip_rate_in_h: f64,
    /// Total precipitation for the event, in. (snow events: inches of
    /// snow).
    pub total_precip_in: f64,
    /// Event start, hours after midnight (rounded to the analysis period
    /// increment).
    pub start_h: f64,
    /// Precipitation duration, h (truncated at midnight, rounded to the
    /// analysis period increment).
    pub precip_duration_h: f64,
    /// Wet/snow-covered pavement duration measured from the event start,
    /// h (includes the precipitation, runoff, and drying times; truncated
    /// at midnight).
    pub pavement_duration_h: f64,
}

/// A generated traffic incident (Chapter 29, Section 2, traffic incident
/// procedure).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UrbanIncident {
    /// Day of year (0-364) within the reliability reporting period.
    pub day_of_year: usize,
    /// Street location category.
    pub street: StreetLocation,
    /// Segment index (for segment incidents) or intersection index (for
    /// intersection incidents).
    pub location_index: usize,
    /// True when the incident affects the subject direction of travel
    /// (segments: Equation 29-24; intersections: the leg serving the
    /// subject through movement, Equation 29-22).
    pub affects_subject_direction: bool,
    /// Incident type (lane location and severity).
    pub incident_type: IncidentType,
    /// Start hour of day (incidents "are assumed to occur at the start of
    /// a given hour").
    pub start_hour: u32,
    /// Duration, h, rounded to the nearest analysis period (Equation
    /// 29-19; gamma-distributed, truncated at midnight).
    pub duration_h: f64,
    /// Weather condition at the start hour (drives duration defaults).
    pub weather: WeatherCondition,
}

/// One scenario = one analysis period of one day in the reliability
/// reporting period.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UrbanScenario {
    /// Day of year (0-364).
    pub day_of_year: usize,
    /// Month (1-12) and day of week (0 = Sunday).
    pub month: u32,
    pub day_of_week: u32,
    /// Analysis period index within the study period.
    pub period: usize,
    /// Weather condition and (while falling) water-equivalent
    /// precipitation rate, in./h.
    pub weather: WeatherCondition,
    pub precip_rate_water_in_h: f64,
    /// Demand ratio relative to the base dataset (Equation 29-29 total /
    /// base, including the weather demand change factor).
    pub demand_ratio: f64,
    /// Indices into the incident list of incidents active during this
    /// analysis period.
    pub active_incidents: Vec<usize>,
}

/// Per-scenario facility evaluation result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UrbanScenarioResult {
    /// Facility travel time in the subject direction, s.
    pub travel_time_s: f64,
    /// Travel time index (vs. the base free-flow travel time).
    pub tti: f64,
    /// Facility VMT served during the analysis period, veh-mi.
    pub vmt: f64,
    /// Vehicle hours of through-movement delay vs. base free-flow travel,
    /// veh-h.
    pub vhd: f64,
    /// True if any boundary through movement had v/c > 1.
    pub oversaturated: bool,
}

/// Reliability performance measures for the facility (Chapter 17
/// performance summary).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UrbanReliabilityResults {
    /// Number of scenarios (analysis periods) evaluated.
    pub num_scenarios: usize,
    /// Facility base free-flow travel time, s (the TTI baseline).
    pub base_free_flow_travel_time_s: f64,
    /// Mean facility travel time across scenarios, s.
    pub mean_travel_time_s: f64,
    /// Standard HCM TTI-distribution measures (shared reliability module;
    /// note `reliability_rating` therein uses the freeway 1.33 threshold —
    /// use `reliability_rating_urban` for urban streets).
    pub metrics: ReliabilityMetrics,
    /// Urban street reliability rating: percentage of the weighted
    /// distribution (VMT when VMT-weighted) with TTI below 2.5 (Chapter
    /// 17, Section 3).
    pub reliability_rating_urban: f64,
    /// Total vehicle hours of through-movement delay over the reliability
    /// reporting period, veh-h.
    pub total_vhd: f64,
    /// Number of generated weather events (2-year record) and incidents
    /// (reporting period).
    pub num_weather_events: usize,
    pub num_incidents: usize,
    /// Percentage of scenarios with nondry weather.
    pub pct_nondry_scenarios: f64,
}

// ═══════════════════════════════════════════════════════════════════════════════
// Calendar helpers
// ═══════════════════════════════════════════════════════════════════════════════

/// Month (1-12) of a day-of-year index (0-364) in the modeled non-leap
/// year (2-year weather records wrap at 365).
fn month_of_day(day_of_year: usize) -> u32 {
    let mut d = (day_of_year % 365) as u32;
    for (m, len) in MONTH_DAYS.iter().enumerate() {
        if d < *len {
            return m as u32 + 1;
        }
        d -= len;
    }
    12
}

// ═══════════════════════════════════════════════════════════════════════════════
// Weather generation (Equations 29-1 through 29-12)
// ═══════════════════════════════════════════════════════════════════════════════

/// Generate the 2-year weather event record (Steps 1-8 of the weather
/// event procedure). Deterministic for a given seed.
pub fn generate_weather_events(config: &UrbanReliabilityConfig) -> Vec<WeatherEvent> {
    let mut rng = Prng::new(config.weather_seed);
    let mut events = Vec::new();
    for day in 0..(2 * 365) {
        let month = month_of_day(day) as usize - 1;
        let w = match config.weather.get(month) {
            Some(w) => w,
            None => continue,
        };
        let n_days = MONTH_DAYS[month] as f64;
        // Step 1 (Equations 29-1/29-2): does precipitation occur?
        let p_precip = if n_days > 0.0 { w.days_with_precip / n_days } else { 0.0 };
        let r_pd = rng.next_f64();
        if r_pd >= p_precip || p_precip <= 0.0 {
            continue;
        }
        // Step 2 (Equations 29-3/29-4): temperature → rain or snow.
        let temp = normal_inverse(rng.next_f64(), w.mean_temp_f, TEMPERATURE_SD_F);
        let is_snow = temp < 32.0;
        if w.days_with_precip <= 0.0 || w.precip_rate_in_h <= 0.0 || w.total_precip_in <= 0.0 {
            continue;
        }
        // Steps 3/7 (Equations 29-5 through 29-8): intensity and total.
        // Snow statistics are the precipitation statistics times the
        // snow-to-rain depth ratio (Step 7).
        let depth_ratio = if is_snow { SNOW_TO_RAIN_DEPTH_RATIO } else { 1.0 };
        let rate_mean = w.precip_rate_in_h * depth_ratio;
        let rate_sd = rate_mean; // s_rr = 1.0 × mean (Equation 29-5)
        let r_r = rng.next_f64();
        let rate = gamma_inverse(r_r, rate_mean, rate_sd);
        let total_mean = (w.total_precip_in / w.days_with_precip) * depth_ratio;
        let total_sd = (2.5 * total_mean).min(0.65 * depth_ratio); // Equation 29-8 (scaled with the event statistics)
        let total = gamma_inverse(r_r, total_mean, total_sd); // R_td = R_rd (perfectly correlated)
        if rate <= 0.0 || total <= 0.0 {
            continue;
        }
        // Step 4 (Equation 29-9): duration (no event extends past
        // midnight; capped at 24 h before start-time placement).
        let mut dur = (total / rate).min(24.0);
        // Step 5 (Equation 29-10): start time, rounded to the analysis
        // period increment.
        let start = (((24.0 - dur) * rng.next_f64() / ANALYSIS_PERIOD_H).round()
            * ANALYSIS_PERIOD_H)
            .min(24.0 - ANALYSIS_PERIOD_H);
        dur = (dur / ANALYSIS_PERIOD_H).round() * ANALYSIS_PERIOD_H;
        dur = dur.min(24.0 - start);
        // Step 6 (Equations 29-11/29-12): wet pavement duration. The
        // runoff time d_o is 0.083 h for rain and the analyst value
        // (default 0.5 h) for snow; drying time d_d = 0.888 e^(−0.0070 T)
        // + 0.19 I_night. VERIFY-HCM: Exhibit 29-66 reproduces the night
        // term for rain events only (its snow rows omit it); this
        // implementation follows the exhibit.
        let runoff = if is_snow { config.snow_runoff_h } else { RAIN_PAVEMENT_RUNOFF_H };
        let night = start < 6.0 || start >= 18.0;
        let drying = 0.888 * (-0.0070 * temp).exp()
            + if night && !is_snow { 0.19 } else { 0.0 };
        let wet_total = ((dur + runoff + drying) / ANALYSIS_PERIOD_H).round()
            * ANALYSIS_PERIOD_H;
        let wet_total = wet_total.min(24.0 - start);
        events.push(WeatherEvent {
            day,
            is_snow,
            temperature_f: temp,
            precip_rate_in_h: rate,
            total_precip_in: total,
            start_h: start,
            precip_duration_h: dur,
            pavement_duration_h: wet_total,
        });
    }
    events
}

/// Weather condition (and water-equivalent precipitation rate) at a given
/// time of a given day-of-year, from the generated event record (Step 8).
pub fn weather_at(
    events: &[WeatherEvent],
    day: usize,
    hour_start: f64,
    duration_h: f64,
) -> (WeatherCondition, f64) {
    let t0 = hour_start;
    let t1 = hour_start + duration_h;
    for e in events {
        if e.day != day {
            continue;
        }
        let precip_end = e.start_h + e.precip_duration_h;
        let wet_end = e.start_h + e.pavement_duration_h;
        // Precipitation falling during (any part of) the interval.
        if t0 < precip_end && e.start_h < t1 && e.precip_duration_h > 0.0 {
            let water_rate = if e.is_snow {
                e.precip_rate_in_h / SNOW_TO_RAIN_DEPTH_RATIO
            } else {
                e.precip_rate_in_h
            };
            return (
                if e.is_snow { WeatherCondition::Snowfall } else { WeatherCondition::Rainfall },
                water_rate,
            );
        }
        // Wet / snow-covered pavement after the precipitation: overlap
        // with [precip_end, wet_end).
        if t0 < wet_end && t1 > precip_end {
            let cond = if e.is_snow {
                WeatherCondition::SnowOrIceOnPavement
            } else {
                WeatherCondition::WetPavement
            };
            return (cond, 0.0);
        }
    }
    (WeatherCondition::Dry, 0.0)
}

/// Total hours in each weather condition over the 2-year record
/// `[dry, rainfall, wet pavement, snowfall, snow/ice]` — the N_h inputs of
/// Equation 29-13.
pub fn weather_condition_hours(events: &[WeatherEvent]) -> [f64; 5] {
    let total = 2.0 * 365.0 * 24.0;
    let mut rf = 0.0;
    let mut wp = 0.0;
    let mut sf = 0.0;
    let mut sp = 0.0;
    for e in events {
        if e.is_snow {
            sf += e.precip_duration_h;
            sp += (e.pavement_duration_h - e.precip_duration_h).max(0.0);
        } else {
            rf += e.precip_duration_h;
            wp += (e.pavement_duration_h - e.precip_duration_h).max(0.0);
        }
    }
    [total - rf - wp - sf - sp, rf, wp, sf, sp]
}

// ═══════════════════════════════════════════════════════════════════════════════
// Equivalent crash frequencies (Equations 29-13 and 29-14)
// ═══════════════════════════════════════════════════════════════════════════════

/// HCM Equation 29-13: equivalent crash frequency when every day is dry,
/// `F_c,dry = F_c × 8,760 N_y / (N_h,dry + Σ CFAF_wea N_h,wea)`.
///
/// * `expected_crash_frequency` — F_c, crashes/year (input data)
/// * `hours` — total hours in `[dry, rainfall, wet pavement, snowfall,
///   snow/ice]` over the weather record
/// * `n_years` — N_y, years covered by `hours`
/// * `cfaf` — crash frequency adjustment factors `[rainfall, wet
///   pavement, snowfall, snow/ice]`
pub fn equivalent_crash_frequency_dry(
    expected_crash_frequency: f64,
    hours: [f64; 5],
    n_years: f64,
    cfaf: [f64; 4],
) -> f64 {
    let denom = hours[0]
        + cfaf[0] * hours[1]
        + cfaf[1] * hours[2]
        + cfaf[2] * hours[3]
        + cfaf[3] * hours[4];
    if denom <= 0.0 {
        return expected_crash_frequency;
    }
    expected_crash_frequency * 8_760.0 * n_years / denom
}

// ═══════════════════════════════════════════════════════════════════════════════
// The reliability analysis
// ═══════════════════════════════════════════════════════════════════════════════

/// An urban street facility reliability analysis (Chapter 17 core
/// methodology). Deserializable from JSON with the
/// `tests/ExampleCases/hcm/UrbanReliability` fixture schema: a `facility`
/// key (Chapter 16 [`UrbanFacility`] schema, subject direction) and a
/// `config` key ([`UrbanReliabilityConfig`]), plus optional
/// `atdm_strategies`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UrbanReliability {
    /// Base dataset: the facility in the subject direction of travel.
    pub facility: UrbanFacility,
    /// Reliability configuration.
    pub config: UrbanReliabilityConfig,
    /// ATDM strategy / work zone / special event hooks (empty = none).
    #[serde(default)]
    pub atdm_strategies: Vec<AtdmStrategy>,

    // ── Computed (populated by `run`) ────────────────────────────────────
    #[serde(skip)]
    pub weather_events: Vec<WeatherEvent>,
    #[serde(skip)]
    pub incidents: Vec<UrbanIncident>,
    #[serde(skip)]
    pub scenarios: Vec<UrbanScenario>,
    #[serde(skip)]
    pub scenario_results: Vec<UrbanScenarioResult>,
    #[serde(skip)]
    pub distribution: TravelTimeDistribution,
    #[serde(skip)]
    pub results: Option<UrbanReliabilityResults>,
}

impl UrbanReliability {
    pub fn new(facility: UrbanFacility, config: UrbanReliabilityConfig) -> Self {
        Self {
            facility,
            config,
            atdm_strategies: Vec::new(),
            weather_events: Vec::new(),
            incidents: Vec::new(),
            scenarios: Vec::new(),
            scenario_results: Vec::new(),
            distribution: TravelTimeDistribution::new(),
            results: None,
        }
    }

    /// Deserialize from the fixture JSON format.
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    fn validate(&self) -> Result<(), String> {
        let n = self.facility.segments.len();
        if n == 0 {
            return Err("facility must contain at least one segment".into());
        }
        if self.config.boundary_signals.len() != n {
            return Err(format!(
                "boundary_signals has {} entries for {} segments",
                self.config.boundary_signals.len(),
                n
            ));
        }
        if self.config.incidents.segment_crash_frequencies.len() != n {
            return Err(format!(
                "segment_crash_frequencies has {} entries for {} segments",
                self.config.incidents.segment_crash_frequencies.len(),
                n
            ));
        }
        if self.config.incidents.intersection_crash_frequencies.len() != n + 1 {
            return Err(format!(
                "intersection_crash_frequencies needs {} entries (segments + 1), got {}",
                n + 1,
                self.config.incidents.intersection_crash_frequencies.len()
            ));
        }
        if self.config.weather.len() != 12 {
            return Err("weather needs 12 monthly entries (Jan-Dec)".into());
        }
        if self.config.analysis_periods_per_day == 0 {
            return Err("analysis_periods_per_day must be positive".into());
        }
        if self.config.months.is_empty() || self.config.days_of_week.is_empty() {
            return Err("reliability reporting period is empty".into());
        }
        if self.config.base_demand_ratio() <= 0.0 {
            return Err("base demand ratio is zero — check count month/day/hour".into());
        }
        Ok(())
    }

    /// Days of the modeled year within the reliability reporting period,
    /// as `(day_of_year, month, day_of_week)`.
    fn rrp_days(&self) -> Vec<(usize, u32, u32)> {
        (0..365)
            .filter_map(|d| {
                let month = month_of_day(d);
                let dow = (self.config.jan1_day_of_week + d as u32) % 7;
                (self.config.months.contains(&month)
                    && self.config.days_of_week.contains(&dow))
                .then_some((d, month, dow))
            })
            .collect()
    }

    /// Stage 1a: weather event generation.
    fn generate_weather(&mut self) {
        self.weather_events = generate_weather_events(&self.config);
    }

    /// Stage 1c: traffic incident generation (Equations 29-13 through
    /// 29-24). Requires the weather record.
    fn generate_incidents(&mut self) {
        let cfg = &self.config;
        let n_seg = self.facility.segments.len();
        let hours = weather_condition_hours(&self.weather_events);
        let cfaf = cfg.incidents.cfaf_override.unwrap_or([
            exhibit_17_9_cfaf(WeatherCondition::Rainfall),
            exhibit_17_9_cfaf(WeatherCondition::WetPavement),
            exhibit_17_9_cfaf(WeatherCondition::Snowfall),
            exhibit_17_9_cfaf(WeatherCondition::SnowOrIceOnPavement),
        ]);

        // Equations 29-13/29-14 per location.
        let fc_dry_seg: Vec<f64> = cfg
            .incidents
            .segment_crash_frequencies
            .iter()
            .map(|f| equivalent_crash_frequency_dry(*f, hours, 2.0, cfaf))
            .collect();
        let fc_dry_int: Vec<f64> = cfg
            .incidents
            .intersection_crash_frequencies
            .iter()
            .map(|f| equivalent_crash_frequency_dry(*f, hours, 2.0, cfaf))
            .collect();
        let cfaf_of = |w: WeatherCondition| -> f64 {
            match w {
                WeatherCondition::Dry => 1.0,
                WeatherCondition::Rainfall => cfaf[0],
                WeatherCondition::WetPavement => cfaf[1],
                WeatherCondition::Snowfall => cfaf[2],
                WeatherCondition::SnowOrIceOnPavement => cfaf[3],
            }
        };

        let mut rng = Prng::new(cfg.incident_seed);
        let mut incidents = Vec::new();
        for (day, month, dow) in self.rrp_days() {
            // Strategy-schedule CFAF (Equation 29-15's CFAF_str; work
            // zone / special event hook).
            let strategy_cfaf: f64 = self
                .atdm_strategies
                .iter()
                .filter(|s| {
                    (s.months.is_empty() || s.months.contains(&month))
                        && (s.days_of_week.is_empty() || s.days_of_week.contains(&dow))
                })
                .map(|s| s.crash_frequency_adjustment)
                .product();
            for hour in 0..24u32 {
                let (wea, _) = weather_at(&self.weather_events, day, hour as f64, 1.0);
                let f_dow = exhibit_17_6_day_of_week_ratio(dow);
                let f_moy = exhibit_17_7_month_of_year_ratio(cfg.functional_class, month);
                let weekend = dow == 0 || dow == 6;
                let f_hod = exhibit_17_5_hour_of_day_ratio(cfg.functional_class, hour, weekend);

                // Street locations: segments then intersections.
                for loc in 0..(n_seg + fc_dry_int.len()) {
                    let (street, fc_dry) = if loc < n_seg {
                        (StreetLocation::Segment, fc_dry_seg[loc])
                    } else {
                        (StreetLocation::Intersection, fc_dry_int[loc - n_seg])
                    };
                    // Equation 29-15: Fi = CFAF_str × Fc,wea / pc.
                    let fc_wea = fc_dry * cfaf_of(wea);
                    let fi_year = strategy_cfaf * fc_wea / crash_proportion(street);
                    // Equation 29-16: hourly frequency.
                    let fi_hour = fi_year / 8_760.0 * (24.0 * f_hod) * f_dow * f_moy;
                    let joint = incident_joint_proportions(
                        street,
                        cfg.incidents.shoulder_present,
                    );
                    for (t, itype) in INCIDENT_TYPES.iter().enumerate() {
                        let p_i = joint[t];
                        if p_i <= 0.0 {
                            continue;
                        }
                        // Equations 29-17/29-18.
                        let p0 = (-fi_hour * p_i).exp();
                        let r = rng.next_f64();
                        if r <= p0 {
                            continue;
                        }
                        // Equation 29-19: gamma duration, truncated at
                        // midnight, rounded to the analysis period.
                        let mean_h =
                            default_incident_duration_min(itype.severity, wea) / 60.0;
                        let sd_h = INCIDENT_DURATION_CV * mean_h;
                        let dur = gamma_inverse(rng.next_f64(), mean_h, sd_h)
                            .min(24.0 - hour as f64);
                        let dur = (dur / ANALYSIS_PERIOD_H).round() * ANALYSIS_PERIOD_H;
                        if dur <= 0.0 {
                            continue; // shorter than half an analysis period
                        }
                        // Equations 29-20 through 29-24: location.
                        let affects_subject = match street {
                            StreetLocation::Segment => {
                                let v_subj =
                                    self.facility.segments[loc].through_demand_veh_h;
                                let v_opp = cfg
                                    .incidents
                                    .opposing_demand_veh_h
                                    .as_ref()
                                    .and_then(|v| v.get(loc).copied())
                                    .unwrap_or(v_subj);
                                let p2 = if v_subj + v_opp > 0.0 {
                                    v_subj / (v_subj + v_opp)
                                } else {
                                    0.5
                                };
                                rng.next_f64() <= p2 // Equation 29-24
                            }
                            StreetLocation::Intersection => {
                                // Equation 29-20/29-22 with the major legs
                                // carrying the segment two-way volume and
                                // both minor legs the configured volume.
                                let int_idx = loc - n_seg;
                                let seg_ref =
                                    int_idx.min(n_seg.saturating_sub(1));
                                let v_subj = self.facility.segments[seg_ref]
                                    .through_demand_veh_h;
                                let v_opp = cfg
                                    .incidents
                                    .opposing_demand_veh_h
                                    .as_ref()
                                    .and_then(|v| v.get(seg_ref).copied())
                                    .unwrap_or(v_subj);
                                let major = v_subj + v_opp;
                                let minor = cfg.incidents.minor_leg_volume_veh_h;
                                let tv2 = 2.0 * major + 2.0 * minor;
                                let p2 = if tv2 > 0.0 { major / tv2 } else { 0.25 };
                                // The phase-2 leg serves the subject
                                // through approach at this intersection.
                                rng.next_f64() <= p2
                            }
                        };
                        incidents.push(UrbanIncident {
                            day_of_year: day,
                            street,
                            location_index: if street == StreetLocation::Segment {
                                loc
                            } else {
                                loc - n_seg
                            },
                            affects_subject_direction: affects_subject,
                            incident_type: *itype,
                            start_hour: hour,
                            duration_h: dur,
                            weather: wea,
                        });
                    }
                }
            }
        }
        self.incidents = incidents;
    }

    /// Stage 1b/1d: demand variation + scenario dataset generation — one
    /// scenario per analysis period of the reliability reporting period.
    fn generate_scenarios(&mut self) {
        let cfg = &self.config;
        let base_ratio = cfg.base_demand_ratio();
        let mut scenarios = Vec::new();
        for (day, month, dow) in self.rrp_days() {
            for period in 0..cfg.analysis_periods_per_day {
                let t0 = cfg.study_period_start_hour as f64
                    + period as f64 * ANALYSIS_PERIOD_H;
                let (wea, rate) =
                    weather_at(&self.weather_events, day, t0, ANALYSIS_PERIOD_H);
                // Demand ratio (Equation 29-29): hourly systematic factors
                // (constant across the four 15-min periods of the hour)
                // times the weather demand change factor.
                let hour = t0.floor() as u32 % 24;
                let dcf = match wea {
                    WeatherCondition::Rainfall => cfg.demand_change_rain,
                    WeatherCondition::Snowfall => cfg.demand_change_snow,
                    _ => 1.0,
                };
                let ratio = cfg.demand_ratio(month, dow, hour) * dcf / base_ratio;
                // Incidents active during this analysis period.
                let active: Vec<usize> = self
                    .incidents
                    .iter()
                    .enumerate()
                    .filter(|(_, inc)| {
                        inc.day_of_year == day
                            && (inc.start_hour as f64) < t0 + ANALYSIS_PERIOD_H
                            && t0 < inc.start_hour as f64 + inc.duration_h
                    })
                    .map(|(i, _)| i)
                    .collect();
                scenarios.push(UrbanScenario {
                    day_of_year: day,
                    month,
                    day_of_week: dow,
                    period,
                    weather: wea,
                    precip_rate_water_in_h: rate,
                    demand_ratio: ratio,
                    active_incidents: active,
                });
            }
        }
        self.scenarios = scenarios;
    }

    /// Stage 2: evaluate one scenario with the Chapter 16/18 facility
    /// methodology (Equations 29-25 through 29-36 applied to the base
    /// dataset, then the segment/facility computations).
    ///
    /// `queue_in` is the initial queue Qb, veh, at each boundary
    /// intersection's through movement (one entry per segment, same order
    /// as `self.facility.segments`), carried in from the previous
    /// chronological analysis period (0 for the first period of a day).
    /// Returns the scenario result and the residual queue Qe, veh, at each
    /// boundary intersection to carry into the next analysis period (see
    /// the module-level "Residual-queue carryover" docs).
    fn evaluate_scenario(
        &self,
        scenario: &UrbanScenario,
        queue_in: &[f64],
    ) -> (UrbanScenarioResult, Vec<f64>) {
        let cfg = &self.config;
        let n_seg = self.facility.segments.len();
        // Weather adjustment factors (Step 2).
        let f_rs = weather_sat_flow_factor(scenario.weather, scenario.precip_rate_water_in_h);
        let f_s_rs = weather_ffs_factor(scenario.weather, scenario.precip_rate_water_in_h);

        // ATDM strategies active for this scenario.
        let active_strategies: Vec<&AtdmStrategy> = self
            .atdm_strategies
            .iter()
            .filter(|s| {
                (s.months.is_empty() || s.months.contains(&scenario.month))
                    && (s.days_of_week.is_empty()
                        || s.days_of_week.contains(&scenario.day_of_week))
                    && (s.periods.is_empty() || s.periods.contains(&scenario.period))
            })
            .collect();
        let strat_demand: f64 = active_strategies.iter().map(|s| s.demand_adjustment).product();
        let strat_sat: f64 = active_strategies.iter().map(|s| s.sat_flow_adjustment).product();
        let strat_green: f64 =
            active_strategies.iter().map(|s| s.effective_green_adjustment_s).sum();
        let strat_ffs: f64 = active_strategies.iter().map(|s| s.ffs_adjustment).product();

        let mut travel_time_s = 0.0;
        let mut base_tt_s = 0.0;
        let mut vmt = 0.0;
        let mut oversaturated = false;
        let mut queue_out = vec![0.0; n_seg];

        for i in 0..n_seg {
            let base_seg = &self.facility.segments[i];
            let sig = &cfg.boundary_signals[i];
            let mut seg = base_seg.clone();
            let qb = queue_in.get(i).copied().unwrap_or(0.0).max(0.0);

            // Demand (Equation 29-29 ratio; the weather DCF is already in
            // the scenario ratio).
            let ratio = scenario.demand_ratio * strat_demand;
            seg.through_demand_veh_h = base_seg.through_demand_veh_h * ratio;
            seg.midsegment_flow_veh_h = Some(base_seg.midsegment_flow_rate() * ratio);

            // Segment incidents in the subject direction: the most severe
            // active one governs ("If more than one incident occurs at the
            // same time and location, the more serious incident is
            // considered").
            let mut seg_severity: Option<IncidentSeverity> = None;
            let mut seg_lanes_blocked = 0u32;
            // Intersection incidents on the subject through leg of the
            // downstream boundary intersection (index i + 1).
            let mut int_severity: Option<IncidentSeverity> = None;
            let mut int_lanes_blocked = 0u32;
            for &idx in &scenario.active_incidents {
                let inc = &self.incidents[idx];
                if !inc.affects_subject_direction {
                    continue;
                }
                match inc.street {
                    StreetLocation::Segment if inc.location_index == i => {
                        let blocked = lanes_blocked(
                            inc.incident_type.lanes,
                            base_seg.n_through_lanes,
                        );
                        if is_more_severe(inc.incident_type.severity, seg_severity)
                            || blocked > seg_lanes_blocked
                        {
                            seg_severity = Some(worse(
                                inc.incident_type.severity,
                                seg_severity,
                            ));
                            seg_lanes_blocked = seg_lanes_blocked.max(blocked);
                        }
                    }
                    StreetLocation::Intersection if inc.location_index == i + 1 => {
                        let blocked = lanes_blocked(
                            inc.incident_type.lanes,
                            base_seg.n_through_lanes,
                        );
                        if is_more_severe(inc.incident_type.severity, int_severity)
                            || blocked > int_lanes_blocked
                        {
                            int_severity =
                                Some(worse(inc.incident_type.severity, int_severity));
                            int_lanes_blocked = int_lanes_blocked.max(blocked);
                        }
                    }
                    _ => {}
                }
            }

            // Segment lane closure ("the variable for the number of
            // through lanes on the segment is reduced accordingly"),
            // keeping at least one lane open.
            if seg_lanes_blocked > 0 {
                seg.n_through_lanes =
                    (base_seg.n_through_lanes - seg_lanes_blocked.min(base_seg.n_through_lanes - 1))
                        .max(1);
            }

            // Additional running delay (Equations 29-34 through 29-36) on
            // the *base* free-flow speed, entered through d_other.
            let base_ffs = base_seg
                .base_ffs_mph
                .expect("base facility must be analyzed before scenario evaluation");
            let s_star = adjusted_base_ffs(
                base_ffs * strat_ffs,
                f_s_rs,
                seg_severity,
                base_seg.n_through_lanes,
            );
            seg.midsegment_other_delay_s = base_seg.midsegment_other_delay_s
                + additional_delay_s(base_seg.segment_length_ft, base_ffs * strat_ffs, s_star);

            // Boundary intersection saturation flow (Step 5): weather
            // factor times the Equation 29-27 incident factor.
            let approach_lanes = if sig.approach_lanes > 0 {
                sig.approach_lanes
            } else {
                base_seg.n_through_lanes
            };
            let f_ic = match int_severity {
                Some(sev) => incident_sat_flow_factor(
                    int_lanes_blocked,
                    base_seg.n_through_lanes,
                    approach_lanes,
                    sev,
                ),
                None => 1.0,
            };
            let s_adj = sig.sat_flow_veh_h_ln * f_rs * f_ic * strat_sat;

            // Through control delay with the Chapter 19 delay equations
            // (uniform + incremental + initial-queue; see the module-level
            // "Residual-queue carryover" docs for d3 and the Eq 19-38..43
            // average-capacity simplification).
            let g = (sig.effective_green_s + strat_green)
                .clamp(1.0, sig.cycle_length_s - 1.0);
            let c_veh_h =
                (base_seg.n_through_lanes as f64) * s_adj * g / sig.cycle_length_s;
            let x = if c_veh_h > 0.0 { seg.through_demand_veh_h / c_veh_h } else { f64::INFINITY };
            if x > 1.0 || qb > 0.0 {
                oversaturated = true;
            }
            let g_over_c = g / sig.cycle_length_s;
            let p = (sig.platoon_ratio * g_over_c).min(1.0);
            let pf = progression_factor(p, g_over_c, x.min(1.0));
            let d1 = uniform_delay(sig.cycle_length_s, g, x.min(1.0), pf);
            let d2 = incremental_delay(
                ANALYSIS_PERIOD_H,
                x,
                c_veh_h,
                sig.k_factor,
                sig.i_factor,
            );
            let d3 = initial_queue_delay(
                qb,
                seg.through_demand_veh_h,
                c_veh_h,
                ANALYSIS_PERIOD_H,
            );
            queue_out[i] = queue_end_of_period(
                qb,
                seg.through_demand_veh_h,
                c_veh_h,
                ANALYSIS_PERIOD_H,
            );
            seg.through_control_delay_s = Some(d1 + d2 + d3);
            seg.through_capacity_veh_h = Some(c_veh_h);
            seg.effective_green_s = Some(g);
            seg.sat_flow_veh_h_ln = Some(s_adj);

            seg.analyze();
            let t_r = seg.running_time_s.unwrap_or(0.0);
            let d_t = seg.through_delay_s.unwrap_or(0.0);
            travel_time_s += t_r + d_t;
            let seg_len_mi = base_seg.segment_length_ft / 5_280.0;
            base_tt_s += 3_600.0 * seg_len_mi / base_ffs;
            vmt += seg.through_demand_veh_h * ANALYSIS_PERIOD_H * seg_len_mi;
        }

        let tti = if base_tt_s > 0.0 { travel_time_s / base_tt_s } else { 0.0 };
        // Through-movement vehicle hours of delay vs. base free-flow
        // travel over this analysis period.
        let avg_flow = if n_seg > 0 {
            self.facility
                .segments
                .iter()
                .map(|s| s.through_demand_veh_h)
                .sum::<f64>()
                / n_seg as f64
                * scenario.demand_ratio
                * strat_demand
        } else {
            0.0
        };
        let vhd = avg_flow * ANALYSIS_PERIOD_H * (travel_time_s - base_tt_s).max(0.0) / 3_600.0;

        (
            UrbanScenarioResult { travel_time_s, tti: tti.max(0.0), vmt, vhd, oversaturated },
            queue_out,
        )
    }

    /// Run the full Chapter 17 reliability methodology: generate weather,
    /// incidents, and scenarios; evaluate every scenario with the Chapter
    /// 16/18 facility method; and compute the performance measures.
    pub fn run(&mut self) -> Result<&UrbanReliabilityResults, String> {
        self.validate()?;
        // Base dataset evaluation (establishes segment base FFS).
        self.facility.analyze()?;
        let base_tt = self
            .facility
            .results
            .as_ref()
            .map(|r| r.base_free_flow_travel_time_s)
            .ok_or("base facility evaluation failed")?;

        self.generate_weather();
        self.generate_incidents();
        self.generate_scenarios();

        let mut distribution = TravelTimeDistribution::new();
        let mut results = Vec::with_capacity(self.scenarios.len());
        let mut total_vhd = 0.0;
        let mut nondry = 0usize;
        let mut mean_tt_num = 0.0;
        // Residual-queue carryover state: one entry per boundary
        // intersection (segment), reset to 0 at the start of each day's
        // sequence of analysis periods (see the module-level docs).
        let n_seg = self.facility.segments.len();
        let mut queue_state = vec![0.0; n_seg];
        let mut last_day: Option<usize> = None;
        for scenario in &self.scenarios {
            if last_day != Some(scenario.day_of_year) {
                queue_state = vec![0.0; n_seg];
                last_day = Some(scenario.day_of_year);
            }
            let (r, queue_out) = self.evaluate_scenario(scenario, &queue_state);
            queue_state = queue_out;
            let weight = if self.config.vmt_weighted { r.vmt.max(1e-9) } else { 1.0 };
            distribution.add(r.tti, weight);
            total_vhd += r.vhd;
            mean_tt_num += r.travel_time_s;
            if scenario.weather != WeatherCondition::Dry {
                nondry += 1;
            }
            results.push(r);
        }
        let n = results.len();
        let metrics = distribution.metrics();
        let rating_urban =
            distribution.pct_at_or_below(URBAN_RELIABILITY_RATING_TTI_THRESHOLD);
        self.results = Some(UrbanReliabilityResults {
            num_scenarios: n,
            base_free_flow_travel_time_s: base_tt,
            mean_travel_time_s: if n > 0 { mean_tt_num / n as f64 } else { 0.0 },
            metrics,
            reliability_rating_urban: rating_urban,
            total_vhd,
            num_weather_events: self.weather_events.len(),
            num_incidents: self.incidents.len(),
            pct_nondry_scenarios: if n > 0 {
                100.0 * nondry as f64 / n as f64
            } else {
                0.0
            },
        });
        self.scenario_results = results;
        self.distribution = distribution;
        Ok(self.results.as_ref().unwrap())
    }
}

/// Number of through lanes blocked for a lane-location category on an
/// approach/segment with `n_lanes` lanes: shoulder blocks none, one-lane
/// blocks one, two-plus blocks two but always leaves one lane open
/// ("the methodology requires that at least one lane remain open").
fn lanes_blocked(lanes: LaneLocation, n_lanes: u32) -> u32 {
    let nominal = match lanes {
        LaneLocation::Shoulder => 0,
        LaneLocation::OneLane => 1,
        LaneLocation::TwoPlusLanes => 2,
    };
    nominal.min(n_lanes.saturating_sub(1))
}

fn severity_rank(s: IncidentSeverity) -> u8 {
    match s {
        IncidentSeverity::CrashFatalInjury => 3,
        IncidentSeverity::CrashPropertyDamage => 2,
        IncidentSeverity::NoncrashBreakdown => 1,
        IncidentSeverity::NoncrashOther => 0,
    }
}

fn is_more_severe(s: IncidentSeverity, current: Option<IncidentSeverity>) -> bool {
    current.map_or(true, |c| severity_rank(s) > severity_rank(c))
}

fn worse(s: IncidentSeverity, current: Option<IncidentSeverity>) -> IncidentSeverity {
    match current {
        Some(c) if severity_rank(c) >= severity_rank(s) => c,
        _ => s,
    }
}
