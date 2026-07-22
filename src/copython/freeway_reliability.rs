//! Python bindings for HCM Chapter 11 (Freeway Reliability Analysis).

use crate::hcm::freeway_reliability::reliability::ReliabilityAnalysis as LibReliabilityAnalysis;
use crate::hcm::freeway_reliability::exhibits;
use crate::hcm::common::atdm;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

#[pyclass]
#[derive(Debug, Clone)]
pub struct FreewayReliability {
    pub inner: LibReliabilityAnalysis,
}

#[pymethods]
impl FreewayReliability {
    /// Create an HCM Chapter 11 freeway reliability analysis.
    ///
    /// Args:
    ///     json: JSON document with the same schema as the Rust
    ///         `ReliabilityAnalysis` serde model and the
    ///         tests/ExampleCases/hcm/FreewayReliability fixtures: a
    ///         `facility` key (Chapter 10 `FreewayFacility` schema) and a
    ///         `scenario_generation` key (months, weekdays, replications,
    ///         seed date, demand multipliers, weather/incident inputs,
    ///         work zones, special events, rng_seed), plus an optional
    ///         `vmt_weighted` flag (default true).
    ///
    /// Returns:
    ///     FreewayReliability: a new analysis instance.
    #[new]
    #[pyo3(signature = (json=None))]
    pub fn new(json: Option<String>) -> PyResult<Self> {
        let inner = match json {
            Some(text) => serde_json::from_str(&text)
                .map_err(|e| PyValueError::new_err(format!("invalid reliability JSON: {e}")))?,
            None => LibReliabilityAnalysis::default(),
        };
        Ok(FreewayReliability { inner })
    }

    /// Run the full reliability methodology (Steps B-1 through B-13).
    pub fn run(&mut self) -> PyResult<()> {
        self.inner.run().map_err(PyValueError::new_err)
    }

    /// Number of generated scenarios.
    #[getter]
    pub fn num_scenarios(&self) -> usize {
        self.inner.scenario_results.len()
    }

    /// Number of observations in the travel time distribution.
    #[getter]
    pub fn num_observations(&self) -> usize {
        self.inner.distribution.len()
    }

    /// Free-flow facility travel time, min.
    #[getter]
    pub fn free_flow_travel_time_min(&self) -> f64 {
        self.inner.free_flow_travel_time_min
    }

    /// Probability-weighted expected vehicle hours of delay per study
    /// period, veh-h.
    #[getter]
    pub fn expected_vhd(&self) -> f64 {
        self.inner.expected_vhd
    }

    /// Reliability performance measures (Step B-11) as a JSON object:
    /// tti_mean, tti_50, tti_80, tti_95 (PTI), tti_max, misery_index,
    /// reliability_rating, semi_std_dev, std_dev, pct_tti_above_2.
    pub fn metrics(&self) -> PyResult<String> {
        match &self.inner.metrics {
            Some(m) => serde_json::to_string(m)
                .map_err(|e| PyValueError::new_err(format!("serialization failed: {e}"))),
            None => Err(PyValueError::new_err("run() must be called first")),
        }
    }

    /// Mean travel time index.
    pub fn tti_mean(&self) -> f64 {
        self.inner.distribution.mean()
    }

    /// Weighted percentile TTI (p in 0-100), e.g. 95 for the PTI.
    pub fn tti_percentile(&self, p: f64) -> f64 {
        self.inner.distribution.percentile(p)
    }

    /// Misery index (mean of the worst 5% of TTIs).
    pub fn misery_index(&self) -> f64 {
        self.inner.distribution.misery_index()
    }

    /// Reliability rating, % (weighted share with TTI < 1.33).
    pub fn reliability_rating(&self) -> f64 {
        self.inner.distribution.reliability_rating()
    }

    /// Semi-standard deviation (one-sided about TTI = 1).
    pub fn semi_std_dev(&self) -> f64 {
        self.inner.distribution.semi_std_dev()
    }

    /// Percentage of the weighted distribution below the target space mean
    /// speed (failure measure), %.
    pub fn failure_pct_below_speed(&self, target_speed_mi_h: f64) -> f64 {
        self.inner.failure_pct_below_speed(target_speed_mi_h)
    }

    /// Per-scenario TTI matrix [scenario][period].
    pub fn scenario_tti(&self) -> Vec<Vec<f64>> {
        self.inner
            .scenario_results
            .iter()
            .map(|r| r.tti.clone())
            .collect()
    }

    /// Scenario probabilities.
    pub fn scenario_probabilities(&self) -> Vec<f64> {
        self.inner
            .scenario_results
            .iter()
            .map(|r| r.probability)
            .collect()
    }

    /// Serialize the generated scenario set (scenarios, expected weather
    /// event counts, monthly incident frequencies) to JSON.
    pub fn scenario_set_json(&self) -> PyResult<String> {
        match &self.inner.scenario_set {
            Some(s) => serde_json::to_string(s)
                .map_err(|e| PyValueError::new_err(format!("serialization failed: {e}"))),
            None => Err(PyValueError::new_err("run() must be called first")),
        }
    }

    pub fn __repr__(&self) -> String {
        format!(
            "FreewayReliability(scenarios={}, observations={}, fftt={:.2} min)",
            self.inner.scenario_results.len(),
            self.inner.distribution.len(),
            self.inner.free_flow_travel_time_min
        )
    }

    pub fn __str__(&self) -> String {
        format!(
            "Freeway reliability analysis with {} scenarios",
            self.inner.scenario_results.len()
        )
    }
}

/// Chapter 11 planning-level reliability method (Equations 11-1 through
/// 11-5): returns (TTI_mean, TTI_95, PT_45) for a facility with the given
/// free-flow speed (mi/h), peak-hour speed (mi/h), directional lanes, and
/// peak-hour volume-to-capacity ratio.
#[pyfunction]
pub fn planning_reliability(
    ffs: f64,
    peak_speed: f64,
    lanes: u32,
    vc_ratio: f64,
) -> (f64, f64, f64) {
    let tti_mean = exhibits::planning_tti_mean(ffs, peak_speed, lanes, vc_ratio);
    (
        tti_mean,
        exhibits::planning_tti_95(tti_mean),
        exhibits::planning_pt45(tti_mean),
    )
}

/// HCM Chapter 37, Section 3 shoulder/median lane strategy: total-segment
/// capacity adjustment factor (CAF) for opening a shoulder/median lane
/// (Equation 37-1), for use in a `WorkZoneEvent`'s `caf` field (or a
/// `scenario_generation.work_zones[].caf` entry in the JSON config).
///
/// Args:
///     shoulder_capacity_veh_h_ln: capacity of the shoulder/median lane
///         itself, veh/h/ln (e.g., half a normal lane's capacity for a
///         general-traffic auxiliary lane; see
///         `shoulder_lane_capacity_veh_h_ln`).
///     mixed_flow_capacity_veh_h_ln: capacity of a normal mixed-flow
///         lane, veh/h/ln.
///     mixed_flow_lanes: number of mixed-flow lanes in the section.
///
/// Returns:
///     float: capacity adjustment factor (>= 1.0).
#[pyfunction]
pub fn shoulder_lane_caf(
    shoulder_capacity_veh_h_ln: f64,
    mixed_flow_capacity_veh_h_ln: f64,
    mixed_flow_lanes: u32,
) -> f64 {
    atdm::shoulder_lane_caf(shoulder_capacity_veh_h_ln, mixed_flow_capacity_veh_h_ln, mixed_flow_lanes)
}

/// Default capacity of an auxiliary shoulder/median lane open to all
/// traffic, veh/h/ln (half a normal mixed-flow lane's capacity; HCM
/// Chapter 37, Section 3).
///
/// Args:
///     mixed_flow_capacity_veh_h_ln: capacity of a normal mixed-flow
///         lane, veh/h/ln.
///
/// Returns:
///     float: default shoulder/median lane capacity, veh/h/ln.
#[pyfunction]
pub fn shoulder_lane_default_capacity_veh_h_ln(mixed_flow_capacity_veh_h_ln: f64) -> f64 {
    atdm::AUX_SHOULDER_CAPACITY_RATIO * mixed_flow_capacity_veh_h_ln
}

/// HCM Chapter 37, Section 4 ramp-metering strategy: capacity adjustment
/// factor recommended for freeway merge segments while ramp metering is
/// in operation (1.03).
#[pyfunction]
pub fn ramp_metered_merge_caf() -> f64 {
    atdm::RAMP_METERED_MERGE_CAF
}

/// HCM Equation 37-2 (ALINEA-derived locally dynamic ramp-metering rate).
///
/// Args:
///     downstream_capacity_veh_h: capacity of the downstream section,
///         veh/h (CM).
///     upstream_volume_veh_h: volume on the upstream section for the
///         analysis period, veh/h (VM(t)).
///     ramp_volume_veh_h: volume on the ramp during the analysis period,
///         veh/h (VR(t)).
///     ramp_queue_prev_veh: queue on the ramp at the end of the previous
///         analysis period, veh (QR(t-1)).
///     ramp_queue_storage_veh: queue storage capacity of the ramp, veh
///         (QRS).
///     metered_lanes: number of metered lanes on the ramp (NR).
///     min_rate_veh_h_ln: minimum metering rate, veh/h/ln (default 240 if
///         omitted).
///     max_rate_veh_h_ln: maximum metering rate, veh/h/ln (default 900 if
///         omitted).
///
/// Returns:
///     float: metering rate R(t), veh/h/ln.
#[pyfunction]
#[pyo3(signature = (
    downstream_capacity_veh_h,
    upstream_volume_veh_h,
    ramp_volume_veh_h,
    ramp_queue_prev_veh,
    ramp_queue_storage_veh,
    metered_lanes,
    min_rate_veh_h_ln=None,
    max_rate_veh_h_ln=None,
))]
#[allow(clippy::too_many_arguments)]
pub fn alinea_metering_rate(
    downstream_capacity_veh_h: f64,
    upstream_volume_veh_h: f64,
    ramp_volume_veh_h: f64,
    ramp_queue_prev_veh: f64,
    ramp_queue_storage_veh: f64,
    metered_lanes: u32,
    min_rate_veh_h_ln: Option<f64>,
    max_rate_veh_h_ln: Option<f64>,
) -> f64 {
    atdm::alinea_metering_rate(
        downstream_capacity_veh_h,
        upstream_volume_veh_h,
        ramp_volume_veh_h,
        ramp_queue_prev_veh,
        ramp_queue_storage_veh,
        metered_lanes,
        min_rate_veh_h_ln.unwrap_or(atdm::ALINEA_DEFAULT_MIN_RATE_VEH_H_LN),
        max_rate_veh_h_ln.unwrap_or(atdm::ALINEA_DEFAULT_MAX_RATE_VEH_H_LN),
    )
}

pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<FreewayReliability>()?;
    m.add_function(wrap_pyfunction!(planning_reliability, m)?)?;
    m.add_function(wrap_pyfunction!(shoulder_lane_caf, m)?)?;
    m.add_function(wrap_pyfunction!(shoulder_lane_default_capacity_veh_h_ln, m)?)?;
    m.add_function(wrap_pyfunction!(ramp_metered_merge_caf, m)?)?;
    m.add_function(wrap_pyfunction!(alinea_metering_rate, m)?)?;
    Ok(())
}
