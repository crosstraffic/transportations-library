//! Python bindings for HCM Chapter 17 (Urban Street Reliability and ATDM).

use crate::hcm::chapter17::exhibits::URBAN_RELIABILITY_RATING_TTI_THRESHOLD;
use crate::hcm::chapter17::urban_reliability::UrbanReliability as LibUrbanReliability;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

/// HCM Chapter 17 urban street reliability analysis.
///
/// Construct from a JSON configuration matching the
/// `tests/ExampleCases/hcm/UrbanReliability/*.json` fixture format: a
/// `facility` key (Chapter 16 `UrbanFacility` schema, subject direction),
/// a `config` key (reliability reporting period, weather statistics,
/// incident inputs, boundary signal data, Monte Carlo seeds), and an
/// optional `atdm_strategies` array of alternative-dataset hooks. Call
/// `run()`, then read the results.
#[pyclass]
#[derive(Debug, Clone)]
pub struct UrbanReliability {
    pub inner: LibUrbanReliability,
}

#[pymethods]
impl UrbanReliability {
    /// Create an urban street reliability analysis from a JSON
    /// configuration string.
    #[new]
    pub fn new(config_json: &str) -> PyResult<Self> {
        let inner = LibUrbanReliability::from_json(config_json)
            .map_err(|e| PyValueError::new_err(format!("invalid UrbanReliability JSON: {e}")))?;
        Ok(UrbanReliability { inner })
    }

    /// Run the full Chapter 17 methodology: weather/demand/incident
    /// generation, per-analysis-period scenario datasets, Chapter 16/18
    /// facility evaluation of every scenario, and the performance
    /// summary.
    pub fn run(&mut self) -> PyResult<()> {
        self.inner.run().map(|_| ()).map_err(PyValueError::new_err)
    }

    /// Number of scenarios (analysis periods) evaluated.
    #[getter]
    pub fn num_scenarios(&self) -> usize {
        self.inner.results.as_ref().map_or(0, |r| r.num_scenarios)
    }

    /// Number of generated weather events (2-year record).
    #[getter]
    pub fn num_weather_events(&self) -> usize {
        self.inner.weather_events.len()
    }

    /// Number of generated incidents (reliability reporting period).
    #[getter]
    pub fn num_incidents(&self) -> usize {
        self.inner.incidents.len()
    }

    /// Facility base free-flow travel time, s (the TTI baseline).
    #[getter]
    pub fn base_free_flow_travel_time_s(&self) -> Option<f64> {
        self.inner.results.as_ref().map(|r| r.base_free_flow_travel_time_s)
    }

    /// Mean facility travel time across scenarios, s.
    #[getter]
    pub fn mean_travel_time_s(&self) -> Option<f64> {
        self.inner.results.as_ref().map(|r| r.mean_travel_time_s)
    }

    /// Mean travel time index.
    pub fn tti_mean(&self) -> f64 {
        self.inner.distribution.mean()
    }

    /// Weighted percentile TTI (p in 0-100), e.g. 95 for the PTI.
    pub fn tti_percentile(&self, p: f64) -> f64 {
        self.inner.distribution.percentile(p)
    }

    /// Urban street reliability rating, %: weighted share (VMT when
    /// VMT-weighted) with TTI below 2.5 (Chapter 17, Section 3).
    pub fn reliability_rating(&self) -> f64 {
        self.inner
            .distribution
            .pct_at_or_below(URBAN_RELIABILITY_RATING_TTI_THRESHOLD)
    }

    /// Total through-movement vehicle hours of delay over the reliability
    /// reporting period, veh-h.
    #[getter]
    pub fn total_vhd(&self) -> Option<f64> {
        self.inner.results.as_ref().map(|r| r.total_vhd)
    }

    /// Full performance-measure bundle as a JSON object
    /// (`UrbanReliabilityResults` schema: num_scenarios, base/mean travel
    /// times, TTI metrics, urban reliability rating, VHD, event counts).
    pub fn results(&self) -> PyResult<String> {
        match &self.inner.results {
            Some(r) => serde_json::to_string(r)
                .map_err(|e| PyValueError::new_err(format!("serialization failed: {e}"))),
            None => Err(PyValueError::new_err("run() must be called first")),
        }
    }

    /// Per-scenario travel times, s.
    pub fn scenario_travel_times(&self) -> Vec<f64> {
        self.inner
            .scenario_results
            .iter()
            .map(|r| r.travel_time_s)
            .collect()
    }

    /// Per-scenario travel time indices.
    pub fn scenario_tti(&self) -> Vec<f64> {
        self.inner.scenario_results.iter().map(|r| r.tti).collect()
    }

    pub fn __repr__(&self) -> String {
        format!(
            "UrbanReliability(segments={}, scenarios={}, incidents={})",
            self.inner.facility.num_segments(),
            self.inner.scenario_results.len(),
            self.inner.incidents.len()
        )
    }

    pub fn __str__(&self) -> String {
        format!(
            "Urban street reliability analysis with {} scenarios",
            self.inner.scenario_results.len()
        )
    }
}

pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<UrbanReliability>()?;
    Ok(())
}
