//! Python bindings for HCM Chapter 21 (All-Way STOP-Controlled
//! Intersections).

use crate::hcm::chapter21::awsc::{ApproachDir, Awsc as LibAwsc, AwscLane};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

fn parse_dir(dir: &str) -> PyResult<ApproachDir> {
    match dir.to_ascii_uppercase().as_str() {
        "EB" => Ok(ApproachDir::EB),
        "WB" => Ok(ApproachDir::WB),
        "NB" => Ok(ApproachDir::NB),
        "SB" => Ok(ApproachDir::SB),
        other => Err(PyValueError::new_err(format!(
            "approach must be EB/WB/NB/SB, got {other}"
        ))),
    }
}

/// HCM Chapter 21 AWSC intersection analysis.
///
/// Construct from a JSON configuration matching the
/// `tests/ExampleCases/hcm/Awsc/*.json` fixture format (per-approach lanes
/// with assigned left/through/right volumes), call `analyze()`, then read
/// per-lane and aggregate results.
#[pyclass]
#[derive(Debug, Clone)]
pub struct Awsc {
    pub inner: LibAwsc,
}

#[pymethods]
impl Awsc {
    /// Create an AWSC analysis from a JSON configuration string.
    #[new]
    pub fn new(config_json: &str) -> PyResult<Self> {
        let inner = LibAwsc::from_json(config_json)
            .map_err(|e| PyValueError::new_err(format!("invalid AWSC JSON: {e}")))?;
        Ok(Awsc { inner })
    }

    /// Run the HCM Chapter 21 procedure (Steps 1-16 except the Step 12
    /// capacity search; see `compute_lane_capacity`).
    pub fn analyze(&mut self) {
        self.inner.analyze();
    }

    /// Peak hour factor (None if lane volumes are flow rates).
    #[getter]
    pub fn get_phf(&self) -> Option<f64> {
        self.inner.phf
    }

    #[setter]
    pub fn set_phf(&mut self, phf: Option<f64>) {
        self.inner.phf = phf;
    }

    /// Analysis period T, h.
    #[getter]
    pub fn get_analysis_period_h(&self) -> f64 {
        self.inner.analysis_period_h
    }

    #[setter]
    pub fn set_analysis_period_h(&mut self, t: f64) {
        self.inner.analysis_period_h = t;
    }

    /// Iterations used by the departure-headway convergence loop.
    #[getter]
    pub fn get_iterations(&self) -> Option<u32> {
        self.inner.iterations
    }

    /// Number of lanes on an approach ("EB"/"WB"/"NB"/"SB").
    pub fn get_lane_count(&self, approach: &str) -> PyResult<usize> {
        Ok(self.inner.approach(parse_dir(approach)?).lanes.len())
    }

    /// Converged departure headway h_d of a lane, s (Equation 21-28).
    pub fn get_departure_headway(&self, approach: &str, lane: usize) -> PyResult<Option<f64>> {
        Ok(self.lane(approach, lane)?.departure_headway)
    }

    /// Degree of utilization x = v h_d / 3,600 (Equation 21-14).
    pub fn get_degree_of_utilization(&self, approach: &str, lane: usize) -> PyResult<Option<f64>> {
        Ok(self.lane(approach, lane)?.degree_of_utilization)
    }

    /// Service time t_s = h_d - m, s (Equation 21-29).
    pub fn get_service_time(&self, approach: &str, lane: usize) -> PyResult<Option<f64>> {
        Ok(self.lane(approach, lane)?.service_time)
    }

    /// Lane control delay, s/veh (Equation 21-30).
    pub fn get_lane_delay(&self, approach: &str, lane: usize) -> PyResult<Option<f64>> {
        Ok(self.lane(approach, lane)?.control_delay)
    }

    /// Lane LOS letter (Exhibit 21-8).
    pub fn get_lane_los(&self, approach: &str, lane: usize) -> PyResult<Option<String>> {
        Ok(self.lane(approach, lane)?.los.map(|c| c.to_string()))
    }

    /// Lane 95th percentile queue, veh (Equation 21-33).
    pub fn get_lane_queue_95(&self, approach: &str, lane: usize) -> PyResult<Option<f64>> {
        Ok(self.lane(approach, lane)?.queue_95)
    }

    /// Step 12 capacity of a lane, veh/h (iterative search; expensive).
    pub fn compute_lane_capacity(&mut self, approach: &str, lane: usize) -> PyResult<f64> {
        let dir = parse_dir(approach)?;
        if lane >= self.inner.approach(dir).lanes.len() {
            return Err(PyValueError::new_err(format!(
                "no lane {lane} on {approach}"
            )));
        }
        // Ensure flows/groups/adjustments are populated
        self.inner.step1_2_flow_rates();
        self.inner.step3_geometry_groups();
        self.inner.step4_headway_adjustments();
        Ok(self.inner.capacity_of_lane(dir, lane))
    }

    /// Approach control delay, s/veh (Equation 21-31).
    pub fn get_approach_delay(&self, approach: &str) -> PyResult<Option<f64>> {
        Ok(self.inner.approach(parse_dir(approach)?).control_delay)
    }

    /// Approach LOS letter (Exhibit 21-8).
    pub fn get_approach_los(&self, approach: &str) -> PyResult<Option<String>> {
        Ok(self
            .inner
            .approach(parse_dir(approach)?)
            .los
            .map(|c| c.to_string()))
    }

    /// Intersection control delay, s/veh (Equation 21-32).
    #[getter]
    pub fn get_intersection_delay(&self) -> Option<f64> {
        self.inner.intersection_delay
    }

    /// Intersection LOS letter (Exhibit 21-8).
    #[getter]
    pub fn get_intersection_los(&self) -> Option<String> {
        self.inner.intersection_los.map(|c| c.to_string())
    }

    /// Full analysis (inputs + results) as JSON.
    pub fn to_json(&self) -> PyResult<String> {
        self.inner
            .to_json()
            .map_err(|e| PyValueError::new_err(format!("serialize AWSC: {e}")))
    }

    pub fn __repr__(&self) -> String {
        format!(
            "Awsc(four_leg={}, intersection_delay={:?}, los={:?})",
            self.inner.is_four_leg(),
            self.inner.intersection_delay,
            self.inner.intersection_los
        )
    }
}

impl Awsc {
    fn lane(&self, approach: &str, lane: usize) -> PyResult<&AwscLane> {
        let dir = parse_dir(approach)?;
        self.inner
            .approach(dir)
            .lanes
            .get(lane)
            .ok_or_else(|| PyValueError::new_err(format!("no lane {lane} on {approach}")))
    }
}

pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Awsc>()?;
    Ok(())
}
