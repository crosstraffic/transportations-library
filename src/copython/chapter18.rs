//! Python bindings for HCM Chapter 18 (Urban Street Segments).

use crate::hcm::chapter18::urban_segments::UrbanSegment as LibUrbanSegment;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

/// HCM Chapter 18 urban street segment analysis (motorized vehicle
/// methodology, one direction of travel).
///
/// Construct from a JSON configuration matching the
/// `tests/ExampleCases/hcm/UrbanSegments/*.json` fixture format (segment
/// geometry, demand flow rates, and downstream boundary intersection
/// performance inputs), call `analyze()`, then read the results.
#[pyclass]
#[derive(Debug, Clone)]
pub struct UrbanSegment {
    pub inner: LibUrbanSegment,
}

#[pymethods]
impl UrbanSegment {
    /// Create an urban street segment analysis from a JSON configuration
    /// string.
    ///
    /// Args:
    ///     config_json: JSON with `segment_length_ft`, `n_through_lanes`,
    ///         `speed_limit_mph`, `through_demand_veh_h`, `control`
    ///         ("Signalized", "AllWayStop", "YieldControlled",
    ///         "Roundabout", or "Uncontrolled"), and the optional
    ///         geometry/performance inputs of the fixture format.
    #[new]
    pub fn new(config_json: &str) -> PyResult<Self> {
        let inner = LibUrbanSegment::from_json(config_json)
            .map_err(|e| PyValueError::new_err(format!("invalid UrbanSegment JSON: {e}")))?;
        Ok(UrbanSegment { inner })
    }

    /// Run the Chapter 18 motorized vehicle pipeline (Steps 1-3 and 5-10).
    pub fn analyze(&mut self) {
        self.inner.analyze();
    }

    /// Segment length L, ft.
    #[getter]
    pub fn get_segment_length_ft(&self) -> f64 {
        self.inner.segment_length_ft
    }

    #[setter]
    pub fn set_segment_length_ft(&mut self, l: f64) {
        self.inner.segment_length_ft = l;
    }

    /// Posted speed limit S_pl, mi/h.
    #[getter]
    pub fn get_speed_limit_mph(&self) -> f64 {
        self.inner.speed_limit_mph
    }

    #[setter]
    pub fn set_speed_limit_mph(&mut self, s: f64) {
        self.inner.speed_limit_mph = s;
    }

    /// Through-demand flow rate v_th at the downstream boundary
    /// intersection, veh/h.
    #[getter]
    pub fn get_through_demand_veh_h(&self) -> f64 {
        self.inner.through_demand_veh_h
    }

    #[setter]
    pub fn set_through_demand_veh_h(&mut self, v: f64) {
        self.inner.through_demand_veh_h = v;
    }

    /// Base free-flow speed S_fo, mi/h (Equation 18-3).
    #[getter]
    pub fn get_base_free_flow_speed_mph(&self) -> Option<f64> {
        self.inner.base_ffs_mph
    }

    /// Free-flow speed S_f, mi/h (Equation 18-5).
    #[getter]
    pub fn get_free_flow_speed_mph(&self) -> Option<f64> {
        self.inner.free_flow_speed_mph
    }

    /// Segment running time t_R, s (Equation 18-7).
    #[getter]
    pub fn get_running_time_s(&self) -> Option<f64> {
        self.inner.running_time_s
    }

    /// Segment running speed, mi/h.
    #[getter]
    pub fn get_running_speed_mph(&self) -> Option<f64> {
        self.inner.running_speed_mph
    }

    /// Proportion of vehicles arriving during green P (Step 3).
    #[getter]
    pub fn get_proportion_arriving_green(&self) -> Option<f64> {
        self.inner.proportion_arriving_green
    }

    /// Through delay d_t at the downstream boundary intersection, s/veh
    /// (Step 5).
    #[getter]
    pub fn get_through_delay_s(&self) -> Option<f64> {
        self.inner.through_delay_s
    }

    /// Full stop rate h, stops/veh (Step 6).
    #[getter]
    pub fn get_full_stop_rate(&self) -> Option<f64> {
        self.inner.full_stop_rate
    }

    /// Travel speed S_T,seg, mi/h (Equation 18-15).
    #[getter]
    pub fn get_travel_speed_mph(&self) -> Option<f64> {
        self.inner.travel_speed_mph
    }

    /// Spatial stop rate H_seg, stops/mi (Equation 18-16).
    #[getter]
    pub fn get_spatial_stop_rate(&self) -> Option<f64> {
        self.inner.spatial_stop_rate_stops_mi
    }

    /// Volume-to-capacity ratio of the through movement at the downstream
    /// boundary intersection.
    #[getter]
    pub fn get_vc_ratio(&self) -> Option<f64> {
        self.inner.vc_ratio
    }

    /// Segment LOS letter (Exhibit 18-1).
    #[getter]
    pub fn get_los(&self) -> Option<String> {
        self.inner.los.map(|l| format!("{l:?}"))
    }

    /// Automobile traveler perception score I_a,seg (Equation 18-17).
    #[getter]
    pub fn get_perception_score(&self) -> Option<f64> {
        self.inner.perception_score
    }

    /// Full analysis (inputs + results) as JSON.
    pub fn to_json(&self) -> PyResult<String> {
        self.inner
            .to_json()
            .map_err(|e| PyValueError::new_err(format!("serialize UrbanSegment: {e}")))
    }

    pub fn __repr__(&self) -> String {
        format!(
            "UrbanSegment(length_ft={}, control={:?}, travel_speed_mph={:?}, los={:?})",
            self.inner.segment_length_ft,
            self.inner.control,
            self.inner.travel_speed_mph,
            self.inner.los
        )
    }
}

pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<UrbanSegment>()?;
    Ok(())
}
