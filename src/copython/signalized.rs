//! PyO3 bindings for HCM Chapter 19 (Signalized Intersections).
//!
//! Exposes the `SignalizedIntersection` facility with JSON-based
//! construction (the input schema matches the serde model of
//! `hcm::signalized::signalized`; see
//! `tests/ExampleCases/hcm/Signalized/case1.json` for a complete example).

use crate::hcm::signalized::signalized::SignalizedIntersection as LibSignalizedIntersection;
use crate::hcm::common::intersection::Direction;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

/// Parse a compass direction string ("NB", "SB", "EB", "WB").
fn parse_direction(direction: &str) -> PyResult<Direction> {
    match direction.to_uppercase().as_str() {
        "NB" => Ok(Direction::NB),
        "SB" => Ok(Direction::SB),
        "EB" => Ok(Direction::EB),
        "WB" => Ok(Direction::WB),
        other => Err(PyValueError::new_err(format!("unknown direction {other}"))),
    }
}

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

    /// Estimate the right-turn-on-red flow rate for an approach ("NB", "SB",
    /// "EB", "WB") from the HCM Chapter 31, Section 8 exclusive right-turn
    /// lane rule (the complementary cross-street protected left-turn demand,
    /// capped at the right-turn demand). Returns 0.0 for shared right-turn
    /// lanes or when no complementary protected left phase exists.
    pub fn estimate_rtor_volume(&self, direction: &str) -> PyResult<f64> {
        let dir = parse_direction(direction)?;
        Ok(self.inner.estimate_rtor_volume(dir))
    }

    /// Populate each approach's RTOR flow rate with the Chapter 31 exclusive
    /// right-turn-lane estimate where none was supplied. Call before
    /// `analyze()`.
    pub fn apply_rtor_estimates(&mut self) {
        self.inner.apply_rtor_estimates();
    }

    /// Estimate the average actuated phase durations from the controller
    /// settings (HCM Chapter 31, Section 2, Equations 31-1 through 31-45) and
    /// return them as a JSON array of per-phase results (phase number,
    /// duration, green interval, queue service time, green extension time,
    /// equivalent maximum allowable headway, and max-out / call
    /// probabilities). Requires `analyze()` to have been called first.
    ///
    /// Args:
    ///     simultaneous_gap_out: whether the through phases terminating at
    ///         each barrier are set for simultaneous gap-out.
    pub fn actuated_timings_json(&self, simultaneous_gap_out: bool) -> PyResult<String> {
        let results = self.inner.estimate_actuated_timings(simultaneous_gap_out);
        serde_json::to_string(&results).map_err(|e| PyValueError::new_err(e.to_string()))
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

/// Evaluate the bicycle LOS at a signalized intersection - HCM Chapter 19,
/// Section 6. Takes a JSON `BicycleIntersection` config; returns a JSON
/// `BicycleIntersectionAnalysis` (capacity, delay, LOS score, LOS).
#[pyfunction]
pub fn analyze_signalized_bicycle(config_json: &str) -> PyResult<String> {
    let ix: crate::hcm::signalized::bicycle::BicycleIntersection =
        serde_json::from_str(config_json)
            .map_err(|e| PyValueError::new_err(format!("invalid bicycle config: {e}")))?;
    serde_json::to_string(&ix.analyze())
        .map_err(|e| PyValueError::new_err(format!("serialize error: {e}")))
}

/// Evaluate the pedestrian LOS at a signalized intersection - HCM Chapter 19,
/// Section 5 (Steps 1-3). Takes a JSON `PedestrianIntersection` config; returns
/// a JSON `PedestrianIntersectionAnalysis` (delay, LOS score, LOS).
#[pyfunction]
pub fn analyze_signalized_pedestrian(config_json: &str) -> PyResult<String> {
    let ix: crate::hcm::signalized::pedestrian::PedestrianIntersection =
        serde_json::from_str(config_json)
            .map_err(|e| PyValueError::new_err(format!("invalid pedestrian config: {e}")))?;
    serde_json::to_string(&ix.analyze())
        .map_err(|e| PyValueError::new_err(format!("serialize error: {e}")))
}

/// Pedestrian delay for a two-stage crossing of one intersection leg
/// (s/pedestrian) - HCM Chapter 19, Equations 19-78 through 19-88. Takes a JSON
/// `TwoStageCrossing` config.
#[pyfunction]
pub fn signalized_two_stage_crossing_delay(config_json: &str) -> PyResult<f64> {
    let x: crate::hcm::signalized::pedestrian::TwoStageCrossing =
        serde_json::from_str(config_json)
            .map_err(|e| PyValueError::new_err(format!("invalid two-stage config: {e}")))?;
    Ok(x.delay())
}

pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<SignalizedIntersection>()?;
    m.add_function(wrap_pyfunction!(analyze_signalized_bicycle, m)?)?;
    m.add_function(wrap_pyfunction!(analyze_signalized_pedestrian, m)?)?;
    m.add_function(wrap_pyfunction!(signalized_two_stage_crossing_delay, m)?)?;
    Ok(())
}
