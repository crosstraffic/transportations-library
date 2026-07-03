//! HCM Chapter 11: Freeway Reliability Analysis (Steps B-1 through B-13).
//!
//! Wraps the Chapter 10 freeway facilities core methodology in a scenario
//! loop: the scenario generator (Chapter 25, Section 9; see
//! [`super::scenario_generation`]) produces a set of study-period
//! scenarios with demand/capacity/speed adjustments; each scenario is
//! evaluated with the core methodology (Step B-10); facility travel times
//! per analysis period are assembled into a probability- and VMT-weighted
//! travel time index distribution (Step B-11); and the Chapter 11
//! reliability performance measures are computed from that distribution
//! via [`crate::hcm::common::reliability`] (Steps B-11/B-13).
//!
//! Sources (HCM 7th Edition EPUB): `76_Ch11_03.xhtml` (methodology and
//! computational steps), `200_Ch25_09.xhtml` (scenario generation),
//! `202_Ch25_11a.xhtml` (Example Problem 7).
//!
//! Adjustment-factor semantics (Step B-9): whenever a scenario contains
//! multiple effects (weather, incident, work zone), CAFs/SAFs/DAFs are
//! multiplicative; incident lane closures enter the segment capacity via
//! the Exhibit 11-23 per-open-lane CAF times the open-lane ratio (see
//! [`super::exhibits::incident_caf_total`]).
//!
//! VERIFY-HCM: the FREEVAL computational engine additionally reduces the
//! *number of lanes* (NLAF) on incident/work-zone segments, which alters
//! per-lane density and speed on those segments; the Chapter 10 engine in
//! this crate models the closure entirely through the total-capacity CAF
//! (the segment keeps its lane count for density purposes). Facility
//! travel times, the quantity feeding the reliability distribution, are
//! driven by the capacity restriction (queueing) and are only marginally
//! affected by this simplification.

use serde::{Deserialize, Serialize};

use crate::hcm::chapter10::freeway_facilities::FreewayFacility;
use crate::hcm::common::reliability::{
    ReliabilityMetrics, Scenario as ScenarioSummary, TravelTimeDistribution,
};

use super::exhibits::incident_caf_total;
use super::scenario_generation::{
    generate_scenarios, FreewayScenario, ScenarioGenerationConfig, ScenarioSet, SeedStatistics,
};

/// HCM chapter implemented by this module.
pub use super::exhibits::CHAPTER;

/// Duration of one analysis period, h.
const PERIOD_H: f64 = 0.25;

/// Facility-level results of one evaluated scenario.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioResult {
    /// Scenario id (index into the scenario set).
    pub id: usize,
    /// Scenario probability.
    pub probability: f64,
    /// Facility travel time by analysis period, min.
    pub travel_time_min: Vec<f64>,
    /// Facility TTI by analysis period.
    pub tti: Vec<f64>,
    /// Served VMT by analysis period, veh-mi.
    pub vmt: Vec<f64>,
    /// Vehicle hours of delay across the study period, veh-h.
    pub vhd: f64,
    /// Whether the scenario had any oversaturated cell (vd/c > 1).
    pub oversaturated: bool,
}

/// A freeway facility reliability analysis (Chapter 11, Part B).
///
/// Deserializable from JSON with the same schema as the
/// `tests/ExampleCases/hcm/FreewayReliability` fixtures: a `facility` key
/// (Chapter 10 `FreewayFacility` schema) and a `scenario_generation` key
/// ([`ScenarioGenerationConfig`]).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ReliabilityAnalysis {
    /// Base (seed) facility dataset — one study period evaluated with the
    /// Chapter 10 core methodology (Step B-1 prerequisite).
    pub facility: FreewayFacility,
    /// Scenario generation configuration (Steps B-1 through B-8).
    pub scenario_generation: ScenarioGenerationConfig,
    /// Weight travel-time observations by VMT in addition to scenario
    /// probability (default true, matching the VMT-weighted TTI
    /// distributions reported by the computational engine, Exhibit
    /// 25-105). Set false for a purely time-based (analysis-period)
    /// distribution.
    pub vmt_weighted: bool,

    // ── Computed ─────────────────────────────────────────────────────────
    /// Generated scenario set (populated by [`Self::run`]).
    #[serde(skip)]
    pub scenario_set: Option<ScenarioSet>,
    /// Per-scenario facility results (populated by [`Self::run`]).
    #[serde(skip)]
    pub scenario_results: Vec<ScenarioResult>,
    /// The weighted TTI distribution (Step B-11).
    #[serde(skip)]
    pub distribution: TravelTimeDistribution,
    /// Reliability performance measures (Step B-13).
    #[serde(skip)]
    pub metrics: Option<ReliabilityMetrics>,
    /// Free-flow facility travel time, min (TTI denominator).
    #[serde(skip)]
    pub free_flow_travel_time_min: f64,
    /// Probability-weighted expected vehicle hours of delay per study
    /// period, veh-h.
    #[serde(skip)]
    pub expected_vhd: f64,
}

impl Default for ReliabilityAnalysis {
    fn default() -> Self {
        Self {
            facility: FreewayFacility::new(),
            scenario_generation: ScenarioGenerationConfig::default(),
            vmt_weighted: true,
            scenario_set: None,
            scenario_results: Vec::new(),
            distribution: TravelTimeDistribution::new(),
            metrics: None,
            free_flow_travel_time_min: 0.0,
            expected_vhd: 0.0,
        }
    }
}

impl ReliabilityAnalysis {
    pub fn new(facility: FreewayFacility, config: ScenarioGenerationConfig) -> Self {
        Self {
            facility,
            scenario_generation: config,
            ..Default::default()
        }
    }

    /// Free-flow facility travel time, min: `Σ L_i / FFS_i` over segments,
    /// using base (unadjusted) segment free-flow speeds.
    pub fn free_flow_travel_time(&self) -> f64 {
        self.facility
            .segments
            .iter()
            .enumerate()
            .map(|(i, s)| {
                let ffs = self.facility.segments[i].ffs.unwrap_or(self.facility.ffs);
                s.length_mi() / ffs * 60.0
            })
            .sum()
    }

    /// Seed statistics for the scenario generator (segment/period VMT from
    /// the base demand matrix; Equations 25-77, 25-88, 25-89).
    pub fn seed_statistics(&self) -> SeedStatistics {
        let mut fac = self.facility.clone();
        fac.compute_demands();
        let vmt: Vec<Vec<f64>> = fac
            .demand
            .iter()
            .enumerate()
            .map(|(i, row)| {
                let l_mi = fac.segments[i].length_mi();
                row.iter().map(|d| d * PERIOD_H * l_mi).collect()
            })
            .collect();
        SeedStatistics {
            vmt,
            num_periods: fac.num_periods(),
            lanes: fac.segments.iter().map(|s| s.lanes).collect(),
            ffs: fac.ffs,
        }
    }

    /// Build the Chapter 10 facility for one scenario: scale demands by
    /// the scenario DAF and fold the scenario's weather/incident/work
    /// zone/special event CAF and SAF matrices into the per-segment
    /// schedules (Steps B-9/B-10).
    pub fn build_scenario_facility(&self, sc: &FreewayScenario) -> FreewayFacility {
        let mut fac = self.facility.clone();
        let n = fac.num_segments();
        let p_count = fac.num_periods();

        // Per-period facility-wide DAF: scenario demand-combination DAF ×
        // event DAFs.
        let mut daf = vec![sc.daf; p_count];
        // Per-segment/per-period CAF/SAF event multipliers.
        let mut caf = vec![vec![1.0; p_count]; n];
        let mut saf = vec![vec![1.0; p_count]; n];

        let weather_daf = self
            .scenario_generation
            .weather
            .as_ref()
            .map(|w| w.daf)
            .unwrap_or(1.0);
        for ev in &sc.weather_events {
            let end = (ev.start_period + ev.duration_periods).min(p_count);
            for p in ev.start_period..end {
                for i in 0..n {
                    caf[i][p] *= ev.caf;
                    saf[i][p] *= ev.saf;
                }
                daf[p] *= weather_daf;
            }
        }

        let incident_daf = self
            .scenario_generation
            .incidents
            .as_ref()
            .map(|i| i.daf)
            .unwrap_or(1.0);
        for inc in &sc.incidents {
            let end = (inc.start_period + inc.duration_periods).min(p_count);
            let seg = inc.segment.min(n - 1);
            let lanes = fac.segments[seg].lanes;
            // Total-capacity multiplier (Exhibit 11-23 per-open-lane CAF ×
            // open-lane ratio); severity was already made feasible during
            // generation, so the lookup cannot fail for lanes >= 2.
            let caf_inc = incident_caf_total(lanes, inc.severity).unwrap_or(1.0);
            for p in inc.start_period..end {
                caf[seg][p] *= caf_inc;
                daf[p] *= incident_daf;
            }
        }

        for &wz_idx in &sc.work_zones {
            let wz = &self.scenario_generation.work_zones[wz_idx];
            let periods: Vec<usize> = wz
                .periods
                .clone()
                .unwrap_or_else(|| (0..p_count).collect());
            for &p in periods.iter().filter(|&&p| p < p_count) {
                for &s in wz.segments.iter().filter(|&&s| s < n) {
                    caf[s][p] *= wz.caf;
                    saf[s][p] *= wz.saf;
                }
                daf[p] *= wz.daf;
            }
        }

        for &se_idx in &sc.special_events {
            let se = &self.scenario_generation.special_events[se_idx];
            let periods: Vec<usize> = se
                .periods
                .clone()
                .unwrap_or_else(|| (0..p_count).collect());
            let segments: Vec<usize> =
                se.segments.clone().unwrap_or_else(|| (0..n).collect());
            for &p in periods.iter().filter(|&&p| p < p_count) {
                for &s in segments.iter().filter(|&&s| s < n) {
                    caf[s][p] *= se.caf;
                    saf[s][p] *= se.saf;
                }
                daf[p] *= se.daf;
            }
        }

        // Apply demand scaling (facility-wide proportional adjustment per
        // the Chapter 11 methodology assumptions).
        for p in 0..p_count {
            fac.mainline_demand[p] *= daf[p];
        }
        for seg in fac.segments.iter_mut() {
            for (p, v) in seg.on_ramp_demand.iter_mut().enumerate() {
                *v *= daf.get(p).copied().unwrap_or(1.0);
            }
            for (p, v) in seg.off_ramp_demand.iter_mut().enumerate() {
                *v *= daf.get(p).copied().unwrap_or(1.0);
            }
            for (p, v) in seg.ramp_to_ramp_demand.iter_mut().enumerate() {
                *v *= daf.get(p).copied().unwrap_or(1.0);
            }
        }

        // Fold event CAF/SAF into the per-segment schedules on top of any
        // base calibration schedule/scalar (multiplicative; Step B-9).
        for (i, seg) in fac.segments.iter_mut().enumerate() {
            let base_caf: Vec<f64> = (0..p_count)
                .map(|p| {
                    seg.caf_schedule
                        .as_ref()
                        .and_then(|v| v.get(p).copied())
                        .unwrap_or(seg.caf)
                })
                .collect();
            let base_saf: Vec<f64> = (0..p_count)
                .map(|p| {
                    seg.saf_schedule
                        .as_ref()
                        .and_then(|v| v.get(p).copied())
                        .unwrap_or(seg.saf)
                })
                .collect();
            seg.caf_schedule =
                Some((0..p_count).map(|p| base_caf[p] * caf[i][p]).collect());
            seg.saf_schedule =
                Some((0..p_count).map(|p| base_saf[p] * saf[i][p]).collect());
        }

        fac
    }

    /// Run the full reliability methodology (Steps B-1 through B-13):
    /// generate scenarios, evaluate each with the Chapter 10 core
    /// methodology, assemble the TTI distribution, and compute the
    /// reliability performance measures.
    pub fn run(&mut self) -> Result<(), String> {
        self.facility.validate()?;
        self.free_flow_travel_time_min = self.free_flow_travel_time();
        if self.free_flow_travel_time_min <= 0.0 {
            return Err("free-flow travel time must be positive".into());
        }

        let seed_stats = self.seed_statistics();
        let set = generate_scenarios(&self.scenario_generation, &seed_stats)?;

        self.distribution = TravelTimeDistribution::new();
        self.scenario_results = Vec::with_capacity(set.scenarios.len());
        self.expected_vhd = 0.0;

        for sc in &set.scenarios {
            let mut fac = self.build_scenario_facility(sc);
            fac.run_analysis()?;

            let p_count = fac.num_periods();
            let mut travel_time_min = Vec::with_capacity(p_count);
            let mut tti = Vec::with_capacity(p_count);
            let mut vmt = Vec::with_capacity(p_count);
            let mut vhd = 0.0;
            for p in 0..p_count {
                let tt_min = facility_travel_time_min(&fac, p);
                let tti_p = (tt_min / self.free_flow_travel_time_min).max(1.0);
                let vmt_p = fac.facility_performance[p].vmt_served;
                let weight = if self.vmt_weighted {
                    sc.probability * vmt_p
                } else {
                    sc.probability
                };
                self.distribution.add(tti_p, weight);
                travel_time_min.push(tt_min);
                tti.push(tti_p);
                vmt.push(vmt_p);
                vhd += fac.facility_performance[p].vhd;
            }
            self.expected_vhd += sc.probability * vhd;
            self.scenario_results.push(ScenarioResult {
                id: sc.id,
                probability: sc.probability,
                travel_time_min,
                tti,
                vmt,
                vhd,
                oversaturated: fac.oversaturated,
            });
        }

        self.metrics = Some(self.distribution.metrics());
        self.scenario_set = Some(set);
        Ok(())
    }

    /// Failure measure (Step B-11): percentage of the weighted
    /// distribution with facility space mean speed below
    /// `target_speed_mi_h` (targets of 35/45/50 mi/h are typical).
    pub fn failure_pct_below_speed(&self, target_speed_mi_h: f64) -> f64 {
        let length_mi = self.facility.total_length_mi();
        if self.free_flow_travel_time_min <= 0.0 || target_speed_mi_h <= 0.0 {
            return 0.0;
        }
        // Observation speed = L / TT; TTI threshold = FFS_equiv / target.
        let ffs_equiv = length_mi / (self.free_flow_travel_time_min / 60.0);
        self.distribution.failure_pct(ffs_equiv / target_speed_mi_h)
    }

    /// On-time percentage at a target speed (complement of
    /// [`Self::failure_pct_below_speed`]).
    pub fn on_time_pct_at_speed(&self, target_speed_mi_h: f64) -> f64 {
        100.0 - self.failure_pct_below_speed(target_speed_mi_h)
    }

    /// Scenario summaries in the shared [`ScenarioSummary`] form.
    pub fn scenario_summaries(&self) -> Vec<ScenarioSummary> {
        let Some(set) = &self.scenario_set else {
            return Vec::new();
        };
        set.scenarios
            .iter()
            .map(|sc| {
                let weather = if sc.weather_events.is_empty() {
                    None
                } else {
                    Some(
                        sc.weather_events
                            .iter()
                            .map(|e| {
                                format!(
                                    "{} (AP {}-{})",
                                    e.weather.name(),
                                    e.start_period + 1,
                                    e.start_period + e.duration_periods
                                )
                            })
                            .collect::<Vec<_>>()
                            .join("; "),
                    )
                };
                let incident = if sc.incidents.is_empty() {
                    None
                } else {
                    Some(
                        sc.incidents
                            .iter()
                            .map(|i| {
                                format!(
                                    "{} seg {} (AP {}-{})",
                                    i.severity.name(),
                                    i.segment + 1,
                                    i.start_period + 1,
                                    i.start_period + i.duration_periods
                                )
                            })
                            .collect::<Vec<_>>()
                            .join("; "),
                    )
                };
                let caf = sc
                    .weather_events
                    .iter()
                    .map(|e| e.caf)
                    .fold(1.0, f64::min);
                let saf = sc
                    .weather_events
                    .iter()
                    .map(|e| e.saf)
                    .fold(1.0, f64::min);
                ScenarioSummary {
                    probability: sc.probability,
                    demand_multiplier: sc.demand_multiplier,
                    caf,
                    saf,
                    daf: sc.daf,
                    weather,
                    incident,
                }
            })
            .collect()
    }
}

/// Facility travel time for one analysis period, min: `Σ L_i / U_i` over
/// segments at the period's segment space mean speeds.
fn facility_travel_time_min(fac: &FreewayFacility, p: usize) -> f64 {
    fac.segments
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let u = fac.speed[i][p];
            if u > 0.0 {
                s.length_mi() / u * 60.0
            } else {
                // Fully stopped segment: bound by a nominal crawl speed to
                // keep travel times finite.
                s.length_mi() / 1.0 * 60.0
            }
        })
        .sum()
}
