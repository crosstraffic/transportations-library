//! Analysis time period and demand profile primitives.
//!
//! Minimal shared infrastructure for multiperiod HCM analyses. The HCM
//! standard analysis period is 15 min (T = 0.25 h); freeway facilities
//! (Chapters 10 and 11) chain multiple consecutive periods. This module
//! will grow as those chapters are implemented.

use serde::{Deserialize, Serialize};

/// Duration of the standard HCM analysis period, h (15 min).
pub const DEFAULT_ANALYSIS_PERIOD_H: f64 = 0.25;

/// An analysis time frame made of one or more equal-duration periods.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisPeriod {
    /// Duration of a single analysis period, h (HCM default 0.25 h).
    pub duration_h: f64,
    /// Number of consecutive analysis periods in the study period.
    pub num_periods: u32,
}

impl Default for AnalysisPeriod {
    fn default() -> Self {
        Self {
            duration_h: DEFAULT_ANALYSIS_PERIOD_H,
            num_periods: 1,
        }
    }
}

impl AnalysisPeriod {
    /// Total study duration, h.
    pub fn total_duration_h(&self) -> f64 {
        self.duration_h * f64::from(self.num_periods)
    }

    /// Convert a vehicle count observed during one analysis period into an
    /// hourly flow rate, veh/h (e.g., a 15-min count × 4).
    pub fn count_to_flow_rate(&self, count_veh: f64) -> f64 {
        count_veh / self.duration_h
    }
}

/// Demand volumes by analysis period.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DemandProfile {
    /// Demand volume in each analysis period, veh (counts per period).
    pub period_volumes: Vec<f64>,
}

impl DemandProfile {
    /// Index of the period with the highest demand (`None` if empty).
    pub fn peak_period_index(&self) -> Option<usize> {
        self.period_volumes
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(i, _)| i)
    }

    /// Volume of the peak period, veh (`None` if empty).
    pub fn peak_period_volume(&self) -> Option<f64> {
        self.peak_period_index().map(|i| self.period_volumes[i])
    }

    /// Total demand over all periods, veh.
    pub fn total_volume(&self) -> f64 {
        self.period_volumes.iter().sum()
    }

    /// Per-period hourly flow rates, veh/h, given the period definition.
    pub fn flow_rates(&self, period: &AnalysisPeriod) -> Vec<f64> {
        self.period_volumes
            .iter()
            .map(|&v| period.count_to_flow_rate(v))
            .collect()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_analysis_period() {
        let p = AnalysisPeriod::default();
        assert!((p.duration_h - 0.25).abs() < 1e-12);
        assert_eq!(p.num_periods, 1);
        assert!((p.total_duration_h() - 0.25).abs() < 1e-12);
    }

    #[test]
    fn test_count_to_flow_rate() {
        let p = AnalysisPeriod::default();
        // 250 veh in 15 min => 1,000 veh/h
        assert!((p.count_to_flow_rate(250.0) - 1_000.0).abs() < 1e-9);
    }

    #[test]
    fn test_demand_profile_peak_and_totals() {
        let profile = DemandProfile {
            period_volumes: vec![200.0, 350.0, 300.0, 250.0],
        };
        assert_eq!(profile.peak_period_index(), Some(1));
        assert_eq!(profile.peak_period_volume(), Some(350.0));
        assert!((profile.total_volume() - 1_100.0).abs() < 1e-9);

        let p = AnalysisPeriod {
            duration_h: 0.25,
            num_periods: 4,
        };
        assert!((p.total_duration_h() - 1.0).abs() < 1e-12);
        let rates = profile.flow_rates(&p);
        assert_eq!(rates.len(), 4);
        assert!((rates[1] - 1_400.0).abs() < 1e-9);
    }

    #[test]
    fn test_empty_profile() {
        let profile = DemandProfile {
            period_volumes: vec![],
        };
        assert_eq!(profile.peak_period_index(), None);
        assert_eq!(profile.peak_period_volume(), None);
        assert_eq!(profile.total_volume(), 0.0);
    }
}
