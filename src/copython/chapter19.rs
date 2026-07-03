//! PyO3 bindings for HCM Chapter 19 (Signalized Intersections).
//!
//! Exposes the `SignalizedIntersection` facility with JSON-based
//! construction (the input schema matches the serde model of
//! `hcm::chapter19::signalized`; see
//! `tests/ExampleCases/hcm/Signalized/case1.json` for a complete example).

use crate::hcm::chapter19::signalized::SignalizedIntersection as LibSignalizedIntersection;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

#[pyclass]
#[derive(Debug, Clone)]
pub struct SignalizedIntersection {
    pub inner: LibSignalizedIntersection,
}

#[pymethods]
impl SignalizedIntersection {
    /// Create a signalized intersection from its JSON description.
    ///
    /// Args:
    ///     json: JSON object matching the Rust serde schema (cycle length,
    ///         analysis period, base saturation flow, area type, control
    ///         type, and the per-approach demand/geometry/signal inputs of
    ///         HCM Exhibits 19-11 and 19-12).
    ///
    /// Returns:
    ///     SignalizedIntersection: the facility, not yet analyzed.
    #[new]
    pub fn new(json: &str) -> PyResult<Self> {
        let inner: LibSignalizedIntersection =
            serde_json::from_str(json).map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(SignalizedIntersection { inner })
    }

    /// Alias constructor mirroring `SignalizedIntersection(json)`.
    #[staticmethod]
    pub fn from_json(json: &str) -> PyResult<Self> {
        Self::new(json)
    }

    /// Run the full HCM Chapter 19 motorized vehicle methodology
    /// (Steps 1-10 of Exhibit 19-18) for pretimed / coordinated timing.
    pub fn analyze(&mut self) {
        self.inner.analyze();
    }

    /// Serialize the facility (inputs and computed results) to JSON.
    pub fn to_json(&self) -> PyResult<String> {
        serde_json::to_string(&self.inner).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Cycle length C, s.
    #[getter]
    pub fn get_cycle_length_s(&self) -> f64 {
        self.inner.get_cycle_length()
    }

    /// Analysis period duration T, h.
    #[getter]
    pub fn get_analysis_period_h(&self) -> f64 {
        self.inner.get_analysis_period()
    }

    /// Intersection control delay d_I, s/veh (HCM Equation 19-29).
    /// None before `analyze()` is called.
    #[getter]
    pub fn get_intersection_delay_s(&self) -> Option<f64> {
        self.inner.get_intersection_delay()
    }

    /// Intersection LOS letter (HCM Exhibit 19-8), e.g. "D".
    #[getter]
    pub fn get_intersection_los(&self) -> Option<String> {
        self.inner.get_intersection_los().map(|l| format!("{l:?}"))
    }

    /// Critical intersection volume-to-capacity ratio X_c
    /// (HCM Equation 19-30).
    #[getter]
    pub fn get_critical_vc_ratio(&self) -> Option<f64> {
        self.inner.get_critical_vc_ratio()
    }

    /// Approach control delay for a direction ("NB", "SB", "EB", "WB"),
    /// s/veh (HCM Equation 19-28).
    pub fn approach_delay_s(&self, direction: &str) -> PyResult<f64> {
        self.inner
            .get_approach_results()
            .iter()
            .find(|a| format!("{:?}", a.direction) == direction.to_uppercase())
            .map(|a| a.control_delay_s)
            .ok_or_else(|| PyValueError::new_err(format!("no approach {direction}")))
    }

    /// Approach LOS letter for a direction (HCM Exhibit 19-8).
    pub fn approach_los(&self, direction: &str) -> PyResult<String> {
        self.inner
            .get_approach_results()
            .iter()
            .find(|a| format!("{:?}", a.direction) == direction.to_uppercase())
            .map(|a| format!("{:?}", a.los))
            .ok_or_else(|| PyValueError::new_err(format!("no approach {direction}")))
    }

    /// Number of lane groups established by Step 1.
    #[getter]
    pub fn get_num_lane_groups(&self) -> usize {
        self.inner.get_lane_groups().len()
    }

    /// Lane-group results as a JSON array (direction, kind, flow rate,
    /// saturation flow, capacity, v/c ratio, d1/d2/d3, control delay, LOS,
    /// back of queue, and queue storage ratio per lane group).
    pub fn lane_groups_json(&self) -> PyResult<String> {
        serde_json::to_string(self.inner.get_lane_groups())
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    pub fn __repr__(&self) -> String {
        format!(
            "SignalizedIntersection(cycle={:.1}s, approaches={}, delay={:?}, los={:?})",
            self.inner.get_cycle_length(),
            self.inner.approaches.len(),
            self.inner.get_intersection_delay(),
            self.inner.get_intersection_los(),
        )
    }

    pub fn __str__(&self) -> String {
        match (
            self.inner.get_intersection_delay(),
            self.inner.get_intersection_los(),
        ) {
            (Some(d), Some(los)) => format!(
                "Signalized intersection: control delay {d:.1} s/veh, LOS {los:?}"
            ),
            _ => "Signalized intersection (not yet analyzed)".to_string(),
        }
    }
}

pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<SignalizedIntersection>()?;
    Ok(())
}
