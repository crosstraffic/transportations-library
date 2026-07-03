//! Python bindings for HCM Chapter 10 (Freeway Facilities Core Methodology).

use crate::hcm::chapter10::freeway_facilities::FreewayFacility as LibFreewayFacility;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

#[pyclass]
#[derive(Debug, Clone)]
pub struct FreewayFacility {
    pub inner: LibFreewayFacility,
}

#[pymethods]
impl FreewayFacility {
    /// Create an HCM Chapter 10 freeway facility.
    ///
    /// Args:
    ///     json: Optional JSON document describing the facility (same schema
    ///         as the Rust `FreewayFacility` serde model and the
    ///         tests/ExampleCases/hcm/FreewayFacilities fixtures): ordered
    ///         `segments` (seg_type Basic/Merge/Diverge/Weaving/
    ///         OverlappingRamp, length_ft, lanes, ramp demands per period,
    ///         weaving attributes, CAF/SAF hooks, optional work_zone),
    ///         `mainline_demand` per 15-min analysis period (veh/h), `ffs`,
    ///         `heavy_vehicle_pct`, `terrain`, `city_type`, and the global
    ///         parameters `jam_density_pc` and `queue_discharge_drop`.
    ///
    /// Returns:
    ///     FreewayFacility: a new facility instance.
    #[new]
    #[pyo3(signature = (json=None))]
    pub fn new(json: Option<String>) -> PyResult<Self> {
        let inner = match json {
            Some(text) => serde_json::from_str(&text)
                .map_err(|e| PyValueError::new_err(format!("invalid facility JSON: {e}")))?,
            None => LibFreewayFacility::new(),
        };
        Ok(FreewayFacility { inner })
    }

    /// Run the full core methodology (Steps A-1 through A-17).
    pub fn run_analysis(&mut self) -> PyResult<()> {
        self.inner
            .run_analysis()
            .map_err(PyValueError::new_err)
    }

    /// Number of segments on the facility.
    #[getter]
    pub fn num_segments(&self) -> usize {
        self.inner.num_segments()
    }

    /// Number of 15-min analysis periods.
    #[getter]
    pub fn num_periods(&self) -> usize {
        self.inner.num_periods()
    }

    /// Total facility length, mi.
    #[getter]
    pub fn total_length_mi(&self) -> f64 {
        self.inner.total_length_mi()
    }

    /// Whether any cell of the time-space domain has vd/c > 1.0.
    #[getter]
    pub fn oversaturated(&self) -> bool {
        self.inner.oversaturated
    }

    /// Segment demand matrix [segment][period], veh/h.
    pub fn demand(&self) -> Vec<Vec<f64>> {
        self.inner.demand.clone()
    }

    /// Segment capacity matrix [segment][period], veh/h.
    pub fn capacity(&self) -> Vec<Vec<f64>> {
        self.inner.capacity.clone()
    }

    /// Demand-to-capacity ratio matrix [segment][period].
    pub fn dc_ratio(&self) -> Vec<Vec<f64>> {
        self.inner.dc_ratio.clone()
    }

    /// Volume served matrix [segment][period], veh/h.
    pub fn volume_served(&self) -> Vec<Vec<f64>> {
        self.inner.volume_served.clone()
    }

    /// Segment space mean speed matrix [segment][period], mi/h.
    pub fn speed(&self) -> Vec<Vec<f64>> {
        self.inner.speed.clone()
    }

    /// Segment density matrix [segment][period], veh/mi/ln.
    pub fn density_veh(&self) -> Vec<Vec<f64>> {
        self.inner.density_veh.clone()
    }

    /// Segment density matrix [segment][period], pc/mi/ln.
    pub fn density_pc(&self) -> Vec<Vec<f64>> {
        self.inner.density_pc.clone()
    }

    /// Density-based segment LOS matrix [segment][period], letters A-F.
    pub fn los(&self) -> Vec<Vec<String>> {
        self.inner
            .los
            .iter()
            .map(|row| row.iter().map(|l| l.to_string()).collect())
            .collect()
    }

    /// Mainline queue length at the end of each period [segment][period], ft.
    pub fn queue_length_ft(&self) -> Vec<Vec<f64>> {
        self.inner.queue_length_ft.clone()
    }

    /// Facility space mean speed for one period, mi/h (Equation 25-2).
    pub fn facility_speed(&self, period: usize) -> f64 {
        self.inner.get_facility_speed(period)
    }

    /// Facility average density for one period, veh/mi/ln (Equation 10-1).
    pub fn facility_density_veh(&self, period: usize) -> f64 {
        self.inner.get_facility_density_veh(period)
    }

    /// Facility LOS letter for one period (Exhibit 10-6).
    pub fn facility_los(&self, period: usize) -> String {
        self.inner.get_facility_los(period).to_string()
    }

    /// Overall space mean speed across all periods, mi/h (Equation 25-4).
    pub fn overall_speed(&self) -> f64 {
        self.inner.overall_space_mean_speed()
    }

    /// Overall average density across all periods, veh/mi/ln (Equation 25-5).
    pub fn overall_density_veh(&self) -> f64 {
        self.inner.overall_density_veh()
    }

    /// Serialize the facility (inputs and computed results) to JSON.
    pub fn to_json(&self) -> PyResult<String> {
        serde_json::to_string(&self.inner)
            .map_err(|e| PyValueError::new_err(format!("serialization failed: {e}")))
    }

    pub fn __repr__(&self) -> String {
        format!(
            "FreewayFacility(segments={}, periods={}, length={:.2} mi, oversaturated={})",
            self.inner.num_segments(),
            self.inner.num_periods(),
            self.inner.total_length_mi(),
            self.inner.oversaturated
        )
    }

    pub fn __str__(&self) -> String {
        format!(
            "Freeway facility with {} segments over {} analysis periods ({:.2} mi)",
            self.inner.num_segments(),
            self.inner.num_periods(),
            self.inner.total_length_mi()
        )
    }
}

pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<FreewayFacility>()?;
    Ok(())
}
