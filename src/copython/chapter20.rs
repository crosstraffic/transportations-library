//! Python bindings for HCM Chapter 20 (Two-Way STOP-Controlled
//! Intersections).

use crate::hcm::chapter20::twsc::{Mv, PlatoonBlockage, Twsc as LibTwsc};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

/// HCM Chapter 20 TWSC intersection analysis.
///
/// Construct from a JSON configuration matching the
/// `tests/ExampleCases/hcm/Twsc/*.json` fixture format (demand by HCM
/// Exhibit 20-1 movement number, geometry, PHF, heavy-vehicle percentage),
/// call `analyze()`, then read per-movement and per-lane results.
#[pyclass]
#[derive(Debug, Clone)]
pub struct Twsc {
    pub inner: LibTwsc,
}

fn parse_movement(label: &str) -> PyResult<Mv> {
    Mv::from_label(label)
        .ok_or_else(|| PyValueError::new_err(format!("unknown TWSC movement label: {label}")))
}

#[pymethods]
impl Twsc {
    /// Create a TWSC analysis from a JSON configuration string.
    ///
    /// Args:
    ///     config_json: JSON with `demand`, `geometry`, optional `phf`,
    ///         `analysis_period_h` (default 0.25), `heavy_vehicle_pct`,
    ///         `conflicting_flow_overrides`, and `platoon_blockage` (the
    ///         Step 5b proportion-of-time-blocked inputs p_b,x for coordinated
    ///         upstream signals, HCM Equations 20-19 through 20-21).
    #[new]
    pub fn new(config_json: &str) -> PyResult<Self> {
        let inner = LibTwsc::from_json(config_json)
            .map_err(|e| PyValueError::new_err(format!("invalid TWSC JSON: {e}")))?;
        Ok(Twsc { inner })
    }

    /// Run the complete HCM Chapter 20 procedure (Steps 1-13).
    pub fn analyze(&mut self) {
        self.inner.analyze();
    }

    /// Peak hour factor (None if demand values are flow rates).
    #[getter]
    pub fn get_phf(&self) -> Option<f64> {
        self.inner.phf
    }

    #[setter]
    pub fn set_phf(&mut self, phf: Option<f64>) {
        self.inner.phf = phf;
    }

    /// Heavy-vehicle percentage applied to all movements (%).
    #[getter]
    pub fn get_heavy_vehicle_pct(&self) -> f64 {
        self.inner.heavy_vehicle_pct
    }

    #[setter]
    pub fn set_heavy_vehicle_pct(&mut self, pct: f64) {
        self.inner.heavy_vehicle_pct = pct;
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

    /// Set the upstream-signal proportion-of-time-blocked inputs p_b,x for
    /// Step 5b (HCM Equations 20-19 through 20-21, Exhibit 20-19). Movements
    /// 1U and 4U reuse `pb1`/`pb4`; the two-stage movements 7, 8, 10, and 11
    /// draw their Stage I/II values from `pb1`/`pb4` per Exhibit 20-19. Pass
    /// all zeros (or never call this) for a no-platooning analysis.
    #[allow(clippy::too_many_arguments)]
    pub fn set_platoon_blockage(
        &mut self,
        pb1: f64,
        pb4: f64,
        pb7: f64,
        pb8: f64,
        pb9: f64,
        pb10: f64,
        pb11: f64,
        pb12: f64,
    ) {
        self.inner.platoon_blockage = Some(PlatoonBlockage {
            pb1,
            pb4,
            pb7,
            pb8,
            pb9,
            pb10,
            pb11,
            pb12,
        });
    }

    /// The upstream-signal proportion-of-time-blocked inputs as
    /// `(pb1, pb4, pb7, pb8, pb9, pb10, pb11, pb12)`, or `None` if unset.
    #[getter]
    pub fn get_platoon_blockage(&self) -> Option<(f64, f64, f64, f64, f64, f64, f64, f64)> {
        self.inner
            .platoon_blockage
            .as_ref()
            .map(|p| (p.pb1, p.pb4, p.pb7, p.pb8, p.pb9, p.pb10, p.pb11, p.pb12))
    }

    /// Demand flow rate of a movement ("1", "1U", ..., "12"), veh/h.
    pub fn get_flow_rate(&self, movement: &str) -> PyResult<f64> {
        Ok(self.inner.get_flow_rate(parse_movement(movement)?))
    }

    /// Conflicting flow rate v_c,x of a movement, veh/h (Step 3).
    pub fn get_conflicting_flow(&self, movement: &str) -> PyResult<Option<f64>> {
        let mv = parse_movement(movement)?;
        Ok(self.inner.movements[mv.idx()].conflicting_flow)
    }

    /// Potential capacity c_p,x of a movement, veh/h (Equation 20-18).
    pub fn get_potential_capacity(&self, movement: &str) -> PyResult<Option<f64>> {
        let mv = parse_movement(movement)?;
        Ok(self.inner.movements[mv.idx()].potential_capacity)
    }

    /// Movement capacity c_m,x, veh/h (Steps 6-9).
    pub fn get_movement_capacity(&self, movement: &str) -> PyResult<Option<f64>> {
        let mv = parse_movement(movement)?;
        Ok(self.inner.movements[mv.idx()].movement_capacity)
    }

    /// Control delay of an exclusive-lane movement, s/veh (Equation 20-61).
    pub fn get_movement_delay(&self, movement: &str) -> PyResult<Option<f64>> {
        let mv = parse_movement(movement)?;
        Ok(self.inner.movements[mv.idx()].control_delay)
    }

    /// LOS letter of an exclusive-lane movement (Exhibit 20-2).
    pub fn get_movement_los(&self, movement: &str) -> PyResult<Option<String>> {
        let mv = parse_movement(movement)?;
        Ok(self.inner.movements[mv.idx()].los.map(|c| c.to_string()))
    }

    /// 95th percentile queue of a movement, veh (Equation 20-66).
    pub fn get_movement_queue_95(&self, movement: &str) -> PyResult<Option<f64>> {
        let mv = parse_movement(movement)?;
        Ok(self.inner.movements[mv.idx()].queue_95)
    }

    /// Number of minor-street approach lanes ("NB" or "SB").
    pub fn get_lane_count(&self, approach: &str) -> PyResult<usize> {
        Ok(self.lanes(approach)?.len())
    }

    /// (capacity veh/h, delay s/veh, LOS, Q95 veh) of a minor approach lane.
    pub fn get_lane_result(
        &self,
        approach: &str,
        lane: usize,
    ) -> PyResult<(f64, f64, String, f64)> {
        let lanes = self.lanes(approach)?;
        let l = lanes
            .get(lane)
            .ok_or_else(|| PyValueError::new_err(format!("no lane {lane} on {approach}")))?;
        Ok((l.capacity, l.control_delay, l.los.to_string(), l.queue_95))
    }

    /// Approach control delays [EB, WB, NB, SB], s/veh (Equation 20-64).
    #[getter]
    pub fn get_approach_delays(&self) -> Option<[f64; 4]> {
        self.inner.approach_delays
    }

    /// Intersection control delay, s/veh (Equation 20-65). Note LOS is not
    /// defined for a TWSC intersection as a whole.
    #[getter]
    pub fn get_intersection_delay(&self) -> Option<f64> {
        self.inner.intersection_delay
    }

    /// Full analysis (inputs + results) as JSON.
    pub fn to_json(&self) -> PyResult<String> {
        self.inner
            .to_json()
            .map_err(|e| PyValueError::new_err(format!("serialize TWSC: {e}")))
    }

    pub fn __repr__(&self) -> String {
        format!(
            "Twsc(three_leg={}, major_lanes={}, intersection_delay={:?})",
            self.inner.geometry.is_three_leg,
            self.inner.geometry.major_lanes_per_direction,
            self.inner.intersection_delay
        )
    }
}

impl Twsc {
    fn lanes(&self, approach: &str) -> PyResult<&Vec<crate::hcm::chapter20::twsc::TwscLaneResult>> {
        match approach.to_ascii_uppercase().as_str() {
            "NB" => Ok(&self.inner.lanes_nb),
            "SB" => Ok(&self.inner.lanes_sb),
            other => Err(PyValueError::new_err(format!(
                "TWSC minor approach must be NB or SB, got {other}"
            ))),
        }
    }
}

pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Twsc>()?;
    Ok(())
}
