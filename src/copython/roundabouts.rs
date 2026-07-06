//! Python bindings for HCM Chapter 22 (Roundabouts).

use crate::hcm::roundabouts::roundabouts::{
    Leg, RoundaboutApproach, RoundaboutLaneResult, Roundabouts as LibRoundabouts,
};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

fn parse_leg(leg: &str) -> PyResult<Leg> {
    match leg.to_ascii_uppercase().as_str() {
        "NB" => Ok(Leg::NB),
        "SB" => Ok(Leg::SB),
        "EB" => Ok(Leg::EB),
        "WB" => Ok(Leg::WB),
        other => Err(PyValueError::new_err(format!(
            "entry must be NB/SB/EB/WB, got {other}"
        ))),
    }
}

/// HCM Chapter 22 roundabout analysis.
///
/// Construct from a JSON configuration matching the
/// `tests/ExampleCases/hcm/Roundabouts/*.json` fixture format (per-entry
/// movement volumes, lane configuration, bypass type, pedestrians), call
/// `analyze()`, then read per-lane and aggregate results.
#[pyclass]
#[derive(Debug, Clone)]
pub struct Roundabouts {
    pub inner: LibRoundabouts,
}

#[pymethods]
impl Roundabouts {
    /// Create a roundabout analysis from a JSON configuration string.
    #[new]
    pub fn new(config_json: &str) -> PyResult<Self> {
        let inner = LibRoundabouts::from_json(config_json)
            .map_err(|e| PyValueError::new_err(format!("invalid roundabout JSON: {e}")))?;
        Ok(Roundabouts { inner })
    }

    /// Run the complete HCM Chapter 22 procedure (Steps 1-12).
    pub fn analyze(&mut self) {
        self.inner.analyze();
    }

    /// Peak hour factor (None if volumes are flow rates).
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

    /// Set a local calibration (A, B) for the entry capacity model
    /// (Equations 22-21 through 22-23; A = 3,600/t_f,
    /// B = (t_c - t_f/2)/3,600).
    pub fn set_calibration(&mut self, a: f64, b: f64) {
        self.inner.calibration = Some((a, b));
    }

    /// Conflicting circulating flow of an entry, pc/h (Equation 22-11).
    pub fn get_circulating_flow_pce(&self, entry: &str) -> PyResult<Option<f64>> {
        Ok(self.approach(entry)?.circulating_flow_pce)
    }

    /// Number of entry lanes with results for an entry.
    pub fn get_lane_count(&self, entry: &str) -> PyResult<usize> {
        Ok(self.approach(entry)?.lanes.len())
    }

    /// (flow veh/h, capacity veh/h, v/c, delay s/veh, LOS, Q95 veh) of an
    /// entry lane (0 = left/only lane).
    pub fn get_lane_result(
        &self,
        entry: &str,
        lane: usize,
    ) -> PyResult<(f64, f64, f64, f64, String, f64)> {
        let a = self.approach(entry)?;
        let l = a
            .lanes
            .get(lane)
            .ok_or_else(|| PyValueError::new_err(format!("no lane {lane} on {entry}")))?;
        Ok(lane_tuple(l))
    }

    /// Bypass-lane result tuple, if the entry has a bypass lane.
    pub fn get_bypass_result(
        &self,
        entry: &str,
    ) -> PyResult<Option<(f64, f64, f64, f64, String, f64)>> {
        Ok(self.approach(entry)?.bypass_lane.as_ref().map(lane_tuple))
    }

    /// Approach control delay, s/veh (Equation 22-18, bypass included).
    pub fn get_approach_delay(&self, entry: &str) -> PyResult<Option<f64>> {
        Ok(self.approach(entry)?.control_delay)
    }

    /// Approach LOS letter (Exhibit 22-8).
    pub fn get_approach_los(&self, entry: &str) -> PyResult<Option<String>> {
        Ok(self.approach(entry)?.los.map(|c| c.to_string()))
    }

    /// Intersection control delay, s/veh (Equation 22-19).
    #[getter]
    pub fn get_intersection_delay(&self) -> Option<f64> {
        self.inner.intersection_delay
    }

    /// Intersection LOS letter (Exhibit 22-8).
    #[getter]
    pub fn get_intersection_los(&self) -> Option<String> {
        self.inner.intersection_los.map(|c| c.to_string())
    }

    /// Full analysis (inputs + results) as JSON.
    pub fn to_json(&self) -> PyResult<String> {
        self.inner
            .to_json()
            .map_err(|e| PyValueError::new_err(format!("serialize roundabout: {e}")))
    }

    pub fn __repr__(&self) -> String {
        format!(
            "Roundabouts(intersection_delay={:?}, los={:?})",
            self.inner.intersection_delay, self.inner.intersection_los
        )
    }
}

fn lane_tuple(l: &RoundaboutLaneResult) -> (f64, f64, f64, f64, String, f64) {
    (
        l.flow_veh,
        l.capacity_veh,
        l.v_c_ratio,
        l.control_delay,
        l.los.to_string(),
        l.queue_95,
    )
}

impl Roundabouts {
    fn approach(&self, entry: &str) -> PyResult<&RoundaboutApproach> {
        Ok(self.inner.approach(parse_leg(entry)?))
    }
}

pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Roundabouts>()?;
    Ok(())
}
