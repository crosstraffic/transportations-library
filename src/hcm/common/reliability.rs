//! Shared travel-time-reliability primitives (HCM 7th Edition).
//!
//! Chapter-agnostic building blocks for reliability analyses: a scenario
//! descriptor and a weighted travel-time-index (TTI) distribution
//! accumulator with the HCM reliability performance measures. Used by
//! Chapter 11 (Freeway Reliability Analysis) and intended for reuse by
//! Chapter 17 (Urban Street Reliability).
//!
//! Definitions (HCM Chapter 11, Section 2, "Travel Time Distribution and
//! Reliability Performance Measures", `75_Ch11_02.xhtml`):
//! - The travel time distribution is "the distribution of average facility
//!   travel times by analysis period across the RRP. Each 15-min analysis
//!   period within each scenario contributes one data point" — it is not a
//!   distribution of individual vehicle travel times.
//! - TTI = actual travel time / free-flow travel time (>= 1.0 by
//!   definition).
//! - TTI_95 (PTI), TTI_80, TTI_50, TTI_mean: percentiles/mean of the TTI
//!   distribution.
//! - Reliability rating: percentage of VMT experiencing TTI < 1.33.
//! - Semi–standard deviation: one-sided standard deviation referenced to
//!   free-flow travel time (TTI = 1) instead of the mean.
//! - Misery index: average of the worst 5% of travel times divided by the
//!   free-flow travel time.
//! - Failure/on-time measures: percentage of analysis periods with space
//!   mean speeds below (failure) or above (on time) a target value.

use serde::{Deserialize, Serialize};

/// A single reliability scenario summarized by facility-wide adjustment
/// factors and event metadata.
///
/// Chapter methodologies typically carry richer, per-segment/per-period
/// adjustment matrices; this shared type captures the scenario-level
/// summary used for reporting and for simple (facility-level) reliability
/// models.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Scenario {
    /// Probability of the scenario (all scenario probabilities in a
    /// reliability reporting period sum to 1.0).
    pub probability: f64,
    /// Demand multiplier relative to the seed/base dataset (the Chapter 25
    /// Equation 25-72 DAF for freeways).
    pub demand_multiplier: f64,
    /// Scenario-level capacity adjustment factor (product of weather /
    /// incident / work zone CAFs where a single summary value is
    /// meaningful; 1.0 otherwise).
    pub caf: f64,
    /// Scenario-level speed adjustment factor.
    pub saf: f64,
    /// Scenario-level demand adjustment factor applied on top of
    /// `demand_multiplier` (weather/special-event demand effects).
    pub daf: f64,
    /// Human-readable weather metadata (e.g., "Heavy rain, periods 3-5").
    pub weather: Option<String>,
    /// Human-readable incident metadata (e.g., "2-lane closure, seg 8").
    pub incident: Option<String>,
}

impl Default for Scenario {
    fn default() -> Self {
        Self {
            probability: 1.0,
            demand_multiplier: 1.0,
            caf: 1.0,
            saf: 1.0,
            daf: 1.0,
            weather: None,
            incident: None,
        }
    }
}

/// One data point of the travel time distribution: a facility average
/// travel time for a single analysis period of a single scenario.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct TravelTimeObservation {
    /// Travel time index (actual / free-flow facility travel time).
    pub tti: f64,
    /// Observation weight. The HCM freeway method weights each analysis
    /// period observation by scenario probability × VMT so that
    /// distribution measures are VMT-weighted (Chapter 25, Exhibit 25-105
    /// reports "VMT-Weighted TTI" distributions); probability-only weights
    /// yield a time-based (analysis-period) distribution.
    pub weight: f64,
}

/// Weighted TTI distribution accumulator over scenarios.
///
/// Observations are appended per scenario/analysis period and weighted by
/// scenario probability (optionally × VMT). All performance measures are
/// computed on the weighted empirical distribution.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TravelTimeDistribution {
    observations: Vec<TravelTimeObservation>,
}

impl TravelTimeDistribution {
    pub fn new() -> Self {
        Self::default()
    }

    /// Append one observation (`tti` >= 1.0 expected; `weight` > 0).
    /// Non-positive weights are ignored.
    pub fn add(&mut self, tti: f64, weight: f64) {
        if weight > 0.0 && tti.is_finite() {
            self.observations.push(TravelTimeObservation { tti, weight });
        }
    }

    pub fn len(&self) -> usize {
        self.observations.len()
    }

    pub fn is_empty(&self) -> bool {
        self.observations.is_empty()
    }

    pub fn observations(&self) -> &[TravelTimeObservation] {
        &self.observations
    }

    /// Total weight of the distribution.
    pub fn total_weight(&self) -> f64 {
        self.observations.iter().map(|o| o.weight).sum()
    }

    /// Weighted mean TTI (TTI_mean).
    pub fn mean(&self) -> f64 {
        let w = self.total_weight();
        if w <= 0.0 {
            return 0.0;
        }
        self.observations.iter().map(|o| o.tti * o.weight).sum::<f64>() / w
    }

    /// Maximum observed TTI (TTI_max).
    pub fn max(&self) -> f64 {
        self.observations
            .iter()
            .map(|o| o.tti)
            .fold(0.0, f64::max)
    }

    /// Weighted percentile TTI (`p` in 0–100), e.g., `percentile(95.0)` =
    /// TTI_95 = PTI. Uses the weighted empirical CDF: the smallest
    /// observation whose cumulative weight reaches p% of the total.
    pub fn percentile(&self, p: f64) -> f64 {
        if self.observations.is_empty() {
            return 0.0;
        }
        let mut sorted: Vec<&TravelTimeObservation> = self.observations.iter().collect();
        sorted.sort_by(|a, b| a.tti.partial_cmp(&b.tti).unwrap_or(std::cmp::Ordering::Equal));
        let target = self.total_weight() * (p / 100.0).clamp(0.0, 1.0);
        let mut cum = 0.0;
        for o in &sorted {
            cum += o.weight;
            if cum >= target - 1e-12 {
                return o.tti;
            }
        }
        sorted.last().unwrap().tti
    }

    /// Weighted standard deviation of the TTI distribution.
    pub fn std_dev(&self) -> f64 {
        let w = self.total_weight();
        if w <= 0.0 {
            return 0.0;
        }
        let mean = self.mean();
        let var = self
            .observations
            .iter()
            .map(|o| o.weight * (o.tti - mean).powi(2))
            .sum::<f64>()
            / w;
        var.max(0.0).sqrt()
    }

    /// Semi–standard deviation: one-sided standard deviation referenced to
    /// free-flow travel time (TTI = 1) instead of the mean (HCM Chapter 11,
    /// Section 2): `sqrt(Σ w·(TTI − 1)² / Σ w)` over observations with
    /// TTI > 1 contributing their full deviation (TTI < 1 is clamped to 0
    /// deviation, though TTI >= 1 by definition).
    pub fn semi_std_dev(&self) -> f64 {
        let w = self.total_weight();
        if w <= 0.0 {
            return 0.0;
        }
        let var = self
            .observations
            .iter()
            .map(|o| o.weight * (o.tti - 1.0).max(0.0).powi(2))
            .sum::<f64>()
            / w;
        var.max(0.0).sqrt()
    }

    /// Misery index: the average of the worst 5% of travel times divided by
    /// the free-flow travel time (HCM Chapter 11, Section 2 / Chapter 36) —
    /// the weighted mean TTI of the top 5% of the weighted distribution.
    pub fn misery_index(&self) -> f64 {
        self.mean_of_worst(0.05)
    }

    /// Weighted mean TTI of the worst `fraction` (0–1) of the distribution
    /// by weight (boundary observation included fractionally).
    pub fn mean_of_worst(&self, fraction: f64) -> f64 {
        if self.observations.is_empty() || fraction <= 0.0 {
            return 0.0;
        }
        let mut sorted: Vec<&TravelTimeObservation> = self.observations.iter().collect();
        sorted.sort_by(|a, b| b.tti.partial_cmp(&a.tti).unwrap_or(std::cmp::Ordering::Equal));
        let target = self.total_weight() * fraction.min(1.0);
        let mut remaining = target;
        let mut num = 0.0;
        for o in &sorted {
            let take = o.weight.min(remaining);
            num += o.tti * take;
            remaining -= take;
            if remaining <= 1e-12 {
                break;
            }
        }
        if target > 0.0 {
            num / target
        } else {
            0.0
        }
    }

    /// Percentage (0–100) of the weighted distribution with TTI at or below
    /// `threshold`. With VMT weights and `threshold = 1.33` this is the HCM
    /// reliability rating (percentage of VMT experiencing TTI < 1.33).
    pub fn pct_at_or_below(&self, threshold: f64) -> f64 {
        let w = self.total_weight();
        if w <= 0.0 {
            return 0.0;
        }
        let below: f64 = self
            .observations
            .iter()
            .filter(|o| o.tti <= threshold)
            .map(|o| o.weight)
            .sum();
        100.0 * below / w
    }

    /// Percentage (0–100) of the weighted distribution with TTI strictly
    /// above `threshold` (e.g., "percentage VMT at TTI > 2").
    pub fn pct_above(&self, threshold: f64) -> f64 {
        let w = self.total_weight();
        if w <= 0.0 {
            return 0.0;
        }
        let above: f64 = self
            .observations
            .iter()
            .filter(|o| o.tti > threshold)
            .map(|o| o.weight)
            .sum();
        100.0 * above / w
    }

    /// Reliability rating: percentage of the weighted distribution (VMT
    /// when VMT-weighted) with TTI below 1.33 (HCM Chapter 11, Section 2).
    pub fn reliability_rating(&self) -> f64 {
        self.pct_at_or_below(RELIABILITY_RATING_TTI_THRESHOLD)
    }

    /// Failure measure: percentage of the weighted distribution with TTI
    /// above `tti_threshold`. For a target space mean speed `S_target`,
    /// `tti_threshold = FFS_equivalent / S_target` where `FFS_equivalent =
    /// facility length / free-flow travel time`.
    pub fn failure_pct(&self, tti_threshold: f64) -> f64 {
        self.pct_above(tti_threshold)
    }

    /// On-time measure: percentage of the weighted distribution with TTI at
    /// or below `tti_threshold` (complement of [`Self::failure_pct`]).
    pub fn on_time_pct(&self, tti_threshold: f64) -> f64 {
        self.pct_at_or_below(tti_threshold)
    }

    /// Compute the full set of standard HCM reliability performance
    /// measures.
    pub fn metrics(&self) -> ReliabilityMetrics {
        ReliabilityMetrics {
            tti_mean: self.mean(),
            tti_50: self.percentile(50.0),
            tti_80: self.percentile(80.0),
            tti_95: self.percentile(95.0),
            tti_max: self.max(),
            misery_index: self.misery_index(),
            reliability_rating: self.reliability_rating(),
            semi_std_dev: self.semi_std_dev(),
            std_dev: self.std_dev(),
            pct_tti_above_2: self.pct_above(2.0),
            num_observations: self.len(),
            total_weight: self.total_weight(),
        }
    }
}

/// TTI threshold for the reliability rating (HCM Chapter 11, Section 2:
/// "the percentage of VMT ... that experiences a TTI less than 1.33").
pub const RELIABILITY_RATING_TTI_THRESHOLD: f64 = 1.33;

/// Standard HCM reliability performance measures computed from a weighted
/// TTI distribution (HCM Chapter 11, Section 3, Step B-11).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReliabilityMetrics {
    /// Mean TTI of the weighted distribution.
    pub tti_mean: f64,
    /// Median TTI.
    pub tti_50: f64,
    /// 80th percentile TTI.
    pub tti_80: f64,
    /// 95th percentile TTI (planning time index, PTI).
    pub tti_95: f64,
    /// Maximum observed TTI.
    pub tti_max: f64,
    /// Misery index (mean of the worst 5% of TTIs).
    pub misery_index: f64,
    /// Reliability rating, % (weighted share with TTI < 1.33).
    pub reliability_rating: f64,
    /// Semi–standard deviation (one-sided about TTI = 1).
    pub semi_std_dev: f64,
    /// Standard deviation of TTI.
    pub std_dev: f64,
    /// Percentage of the weighted distribution with TTI > 2.
    pub pct_tti_above_2: f64,
    /// Number of observations in the distribution.
    pub num_observations: usize,
    /// Total weight of the distribution.
    pub total_weight: f64,
}

// ═══════════════════════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    fn synthetic() -> TravelTimeDistribution {
        // 20 equally weighted observations: 1.0, 1.1, ..., 2.9
        let mut d = TravelTimeDistribution::new();
        for k in 0..20 {
            d.add(1.0 + 0.1 * k as f64, 1.0);
        }
        d
    }

    #[test]
    fn test_mean_and_percentiles() {
        let d = synthetic();
        assert_eq!(d.len(), 20);
        assert!((d.mean() - 1.95).abs() < 1e-9);
        // 50th percentile: cumulative weight reaches 10/20 at the 10th
        // sorted observation (tti = 1.9).
        assert!((d.percentile(50.0) - 1.9).abs() < 1e-9);
        // 95th percentile: 19th observation (tti = 2.8).
        assert!((d.percentile(95.0) - 2.8).abs() < 1e-9);
        // 80th percentile: 16th observation (tti = 2.5).
        assert!((d.percentile(80.0) - 2.5).abs() < 1e-9);
        assert!((d.max() - 2.9).abs() < 1e-9);
    }

    #[test]
    fn test_weighted_percentile() {
        let mut d = TravelTimeDistribution::new();
        d.add(1.0, 9.0);
        d.add(3.0, 1.0);
        // 90% of weight is at 1.0
        assert!((d.percentile(90.0) - 1.0).abs() < 1e-9);
        assert!((d.percentile(95.0) - 3.0).abs() < 1e-9);
        assert!((d.mean() - 1.2).abs() < 1e-9);
    }

    #[test]
    fn test_misery_index() {
        let d = synthetic();
        // Worst 5% of 20 unit weights = weight 1.0 = the single worst
        // observation (2.9).
        assert!((d.misery_index() - 2.9).abs() < 1e-9);
        // Worst 10% = two observations (2.9, 2.8) => 2.85.
        assert!((d.mean_of_worst(0.10) - 2.85).abs() < 1e-9);
    }

    #[test]
    fn test_std_and_semi_std() {
        let mut d = TravelTimeDistribution::new();
        d.add(1.0, 1.0);
        d.add(2.0, 1.0);
        // mean 1.5; var = 0.25; sd = 0.5
        assert!((d.std_dev() - 0.5).abs() < 1e-9);
        // semi-sd about TTI=1: sqrt((0 + 1)/2) = 0.7071
        assert!((d.semi_std_dev() - (0.5f64).sqrt()).abs() < 1e-9);
    }

    #[test]
    fn test_rating_failure_on_time() {
        let d = synthetic();
        // TTI <= 1.33: observations 1.0, 1.1, 1.2, 1.3 => 4/20 = 20%
        assert!((d.reliability_rating() - 20.0).abs() < 1e-9);
        // TTI > 2.0: 2.1..2.9 => 9/20 = 45%
        assert!((d.pct_tti_above_2() - 45.0).abs() < 1e-9);
        assert!((d.failure_pct(2.0) - 45.0).abs() < 1e-9);
        assert!((d.on_time_pct(2.0) - 55.0).abs() < 1e-9);
    }

    #[test]
    fn test_metrics_bundle_and_empty() {
        let d = synthetic();
        let m = d.metrics();
        assert_eq!(m.num_observations, 20);
        assert!((m.total_weight - 20.0).abs() < 1e-9);
        assert!((m.tti_mean - 1.95).abs() < 1e-9);

        let empty = TravelTimeDistribution::new();
        let m = empty.metrics();
        assert_eq!(m.num_observations, 0);
        assert_eq!(m.tti_mean, 0.0);
        assert_eq!(m.tti_95, 0.0);
    }

    #[test]
    fn test_ignores_invalid() {
        let mut d = TravelTimeDistribution::new();
        d.add(1.5, 0.0);
        d.add(1.5, -1.0);
        d.add(f64::NAN, 1.0);
        assert!(d.is_empty());
    }

    impl TravelTimeDistribution {
        fn pct_tti_above_2(&self) -> f64 {
            self.pct_above(2.0)
        }
    }

    #[test]
    fn test_scenario_default() {
        let s = Scenario::default();
        assert_eq!(s.probability, 1.0);
        assert_eq!(s.caf, 1.0);
        assert!(s.weather.is_none());
    }
}
