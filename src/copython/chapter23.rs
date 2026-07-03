//! Python bindings for HCM Chapter 23 (Ramp Terminals and Alternative
//! Intersections, Part B: interchange ramp terminals).

use crate::hcm::chapter23::ramp_terminals::{
    Interchange as LibInterchange, InterchangeMovement, LaneGroupResult, OdResult,
};
use crate::hcm::chapter23::exhibits::OdMovement;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

fn parse_od(letter: &str) -> PyResult<OdMovement> {
    match letter.to_ascii_uppercase().as_str() {
        "A" => Ok(OdMovement::A),
        "B" => Ok(OdMovement::B),
        "C" => Ok(OdMovement::C),
        "D" => Ok(OdMovement::D),
        "E" => Ok(OdMovement::E),
        "F" => Ok(OdMovement::F),
        "G" => Ok(OdMovement::G),
        "H" => Ok(OdMovement::H),
        "I" => Ok(OdMovement::I),
        "J" => Ok(OdMovement::J),
        "K" => Ok(OdMovement::K),
        "L" => Ok(OdMovement::L),
        "M" => Ok(OdMovement::M),
        "N" => Ok(OdMovement::N),
        other => Err(PyValueError::new_err(format!(
            "O-D movement must be a letter A..N, got {other}"
        ))),
    }
}

fn parse_movement(name: &str) -> PyResult<InterchangeMovement> {
    use InterchangeMovement::*;
    match name {
        "EbExtThrough" => Ok(EbExtThrough),
        "EbIntThrough" => Ok(EbIntThrough),
        "EbIntLeft" => Ok(EbIntLeft),
        "WbExtThrough" => Ok(WbExtThrough),
        "WbIntThrough" => Ok(WbIntThrough),
        "WbIntLeft" => Ok(WbIntLeft),
        "NbRampLeft" => Ok(NbRampLeft),
        "NbRampRight" => Ok(NbRampRight),
        "SbRampLeft" => Ok(SbRampLeft),
        "SbRampRight" => Ok(SbRampRight),
        other => Err(PyValueError::new_err(format!(
            "unknown interchange movement {other}"
        ))),
    }
}

/// HCM Chapter 23 signalized interchange ramp terminal analysis
/// (diamond / parclo / SPUI / DDI).
///
/// Construct from a JSON configuration matching the
/// `tests/ExampleCases/hcm/RampTerminals/*.json` fixture format
/// (interchange form, cycle, O-D demands, per-lane-group geometry and
/// signal timing), call `analyze()`, then read per-O-D and interchange
/// results.
#[pyclass]
#[derive(Debug, Clone)]
pub struct Interchange {
    pub inner: LibInterchange,
}

#[pymethods]
impl Interchange {
    /// Create an interchange analysis from a JSON configuration string.
    #[new]
    pub fn new(config_json: &str) -> PyResult<Self> {
        let inner: LibInterchange = serde_json::from_str(config_json)
            .map_err(|e| PyValueError::new_err(format!("invalid interchange JSON: {e}")))?;
        Ok(Interchange { inner })
    }

    /// Run the complete HCM Chapter 23 Part B procedure (Steps 1-9 of
    /// Exhibit 23-22).
    pub fn analyze(&mut self) {
        self.inner.analyze();
    }

    /// Cycle length C, s.
    #[getter]
    pub fn get_cycle_length(&self) -> f64 {
        self.inner.get_cycle_length()
    }

    #[setter]
    pub fn set_cycle_length(&mut self, c: f64) {
        self.inner.set_cycle_length(c);
    }

    /// Peak hour factor applied to the O-D demands.
    #[getter]
    pub fn get_peak_hour_factor(&self) -> f64 {
        self.inner.get_peak_hour_factor()
    }

    #[setter]
    pub fn set_peak_hour_factor(&mut self, phf: f64) {
        self.inner.set_peak_hour_factor(phf);
    }

    /// Demand-weighted interchange experienced travel time ETT, s/veh
    /// (Equation 23-52).
    #[getter]
    pub fn get_interchange_ett(&self) -> Option<f64> {
        self.inner.get_interchange_ett()
    }

    /// Interchange LOS letter (Exhibit 23-10).
    #[getter]
    pub fn get_interchange_los(&self) -> Option<String> {
        self.inner.get_interchange_los().map(|l| format!("{l:?}"))
    }

    /// O-D letters (subset of A..N) that carry demand and have results.
    pub fn get_od_movements(&self) -> Vec<String> {
        self.inner
            .get_od_results()
            .iter()
            .map(|r| format!("{:?}", r.movement))
            .collect()
    }

    /// (demand veh/h, control delay s/veh, EDTT s/veh, ETT s/veh, LOS)
    /// for an O-D movement letter (Exhibit 23-10 basis).
    pub fn get_od_result(&self, letter: &str) -> PyResult<(f64, f64, f64, f64, String)> {
        let m = parse_od(letter)?;
        let r: &OdResult = self
            .inner
            .get_od_results()
            .iter()
            .find(|r| r.movement == m)
            .ok_or_else(|| PyValueError::new_err(format!("no result for O-D {letter}")))?;
        Ok((
            r.demand,
            r.control_delay_s,
            r.edtt_s,
            r.ett_s,
            format!("{:?}", r.los),
        ))
    }

    /// (flow veh/h, saturation flow veh/h, effective green s, capacity
    /// veh/h, v/c, control delay s/veh) for a lane group movement name
    /// (e.g. "EbExtThrough", "NbRampLeft").
    pub fn get_lane_group_result(
        &self,
        movement: &str,
    ) -> PyResult<(f64, f64, f64, f64, f64, f64)> {
        let m = parse_movement(movement)?;
        let r: &LaneGroupResult = self
            .inner
            .get_results()
            .iter()
            .find(|r| r.movement == m)
            .ok_or_else(|| PyValueError::new_err(format!("no lane group {movement}")))?;
        Ok((
            r.flow_rate,
            r.sat_flow.unwrap_or(0.0),
            r.effective_green_s.unwrap_or(0.0),
            r.capacity.unwrap_or(0.0),
            r.vc_ratio.unwrap_or(0.0),
            r.control_delay_s.unwrap_or(0.0),
        ))
    }

    /// Queue storage ratio R_Q of a lane group (None when no storage
    /// length was supplied).
    pub fn get_queue_storage_ratio(&self, movement: &str) -> PyResult<Option<f64>> {
        let m = parse_movement(movement)?;
        Ok(self
            .inner
            .get_results()
            .iter()
            .find(|r| r.movement == m)
            .and_then(|r| r.queue_storage_ratio))
    }

    /// Full analysis (inputs + results) as JSON.
    pub fn to_json(&self) -> PyResult<String> {
        serde_json::to_string(&self.inner)
            .map_err(|e| PyValueError::new_err(format!("serialize interchange: {e}")))
    }

    pub fn __repr__(&self) -> String {
        format!(
            "Interchange(form={:?}, ett={:?}, los={:?})",
            self.inner.form,
            self.inner.get_interchange_ett(),
            self.inner.get_interchange_los()
        )
    }
}

pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Interchange>()?;
    Ok(())
}
