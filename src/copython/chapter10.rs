//! Python bindings for HCM Chapter 10 (Freeway Facilities Core Methodology),
//! including the managed-lane facility extension and the Chapter 25 planning
//! method.

use crate::hcm::chapter10::freeway_facilities::FreewayFacility as LibFreewayFacility;
use crate::hcm::chapter10::managed_lanes::ManagedLaneFacility as LibManagedLaneFacility;
use crate::hcm::chapter10::planning::PlanningFacility as LibPlanningFacility;
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

/// HCM Chapter 10 managed-lane freeway facility (Steps A-9/A-13/A-14/A-17):
/// a general-purpose lane group with a parallel managed-lane lane group.
#[pyclass]
#[derive(Debug, Clone)]
pub struct ManagedLaneFacility {
    pub inner: LibManagedLaneFacility,
}

#[pymethods]
impl ManagedLaneFacility {
    /// Create a managed-lane freeway facility.
    ///
    /// Args:
    ///     json: Optional JSON document (same schema as the Rust
    ///         `ManagedLaneFacility` serde model and the
    ///         tests/ExampleCases/hcm/FreewayFacilities/ml_case1.json
    ///         fixture): `gp` (a FreewayFacility), `ml` (a list parallel to
    ///         the GP segments, each entry `null` or an ML segment with
    ///         lane_type/lanes/ffs/caf/saf and ML ramp demands),
    ///         `ml_entry_demand` per period (veh/h), `ml_ffs`, and optional
    ///         per-GP-segment `cross_weave` entries (Step A-9).
    #[new]
    #[pyo3(signature = (json=None))]
    pub fn new(json: Option<String>) -> PyResult<Self> {
        let inner = match json {
            Some(text) => serde_json::from_str(&text)
                .map_err(|e| PyValueError::new_err(format!("invalid ML facility JSON: {e}")))?,
            None => LibManagedLaneFacility::new(),
        };
        Ok(ManagedLaneFacility { inner })
    }

    /// Run the full managed-lane facility analysis.
    pub fn run_analysis(&mut self) -> PyResult<()> {
        self.inner.run_analysis().map_err(PyValueError::new_err)
    }

    #[getter]
    pub fn num_segments(&self) -> usize {
        self.inner.num_segments()
    }

    #[getter]
    pub fn num_periods(&self) -> usize {
        self.inner.num_periods()
    }

    /// GP segment capacity matrix [segment][period], veh/h.
    pub fn gp_capacity(&self) -> Vec<Vec<f64>> {
        self.inner.gp.capacity.clone()
    }

    /// GP segment speed matrix [segment][period], mi/h.
    pub fn gp_speed(&self) -> Vec<Vec<f64>> {
        self.inner.gp.speed.clone()
    }

    /// GP segment density matrix [segment][period], veh/mi/ln.
    pub fn gp_density_veh(&self) -> Vec<Vec<f64>> {
        self.inner.gp.density_veh.clone()
    }

    /// ML segment capacity matrix [segment][period], veh/h.
    pub fn ml_capacity(&self) -> Vec<Vec<f64>> {
        self.inner.ml_capacity.clone()
    }

    /// ML demand-to-capacity ratio matrix [segment][period].
    pub fn ml_dc_ratio(&self) -> Vec<Vec<f64>> {
        self.inner.ml_dc_ratio.clone()
    }

    /// ML segment speed matrix [segment][period], mi/h.
    pub fn ml_speed(&self) -> Vec<Vec<f64>> {
        self.inner.ml_speed.clone()
    }

    /// ML segment density matrix [segment][period], veh/mi/ln.
    pub fn ml_density_veh(&self) -> Vec<Vec<f64>> {
        self.inner.ml_density_veh.clone()
    }

    /// Whether the Step A-13 adjacent friction was active [segment][period].
    pub fn ml_friction_active(&self) -> Vec<Vec<bool>> {
        self.inner.ml_friction_active.clone()
    }

    /// Combined facility space mean speed for one period, mi/h.
    pub fn facility_speed(&self, period: usize) -> f64 {
        self.inner.get_facility_speed(period)
    }

    /// Combined facility average density for one period, veh/mi/ln.
    pub fn facility_density_veh(&self, period: usize) -> f64 {
        self.inner.get_facility_density_veh(period)
    }

    /// Combined facility LOS letter for one period (Exhibit 10-6).
    pub fn facility_los(&self, period: usize) -> String {
        self.inner.get_facility_los(period).to_string()
    }

    /// Serialize the facility (inputs and computed results) to JSON.
    pub fn to_json(&self) -> PyResult<String> {
        serde_json::to_string(&self.inner)
            .map_err(|e| PyValueError::new_err(format!("serialization failed: {e}")))
    }

    pub fn __repr__(&self) -> String {
        format!(
            "ManagedLaneFacility(segments={}, periods={})",
            self.inner.num_segments(),
            self.inner.num_periods()
        )
    }
}

/// HCM Chapter 25, Section 6 planning-level freeway facility method.
#[pyclass]
#[derive(Debug, Clone)]
pub struct PlanningFacility {
    pub inner: LibPlanningFacility,
}

#[pymethods]
impl PlanningFacility {
    /// Create a planning-level freeway facility.
    ///
    /// Args:
    ///     json: Optional JSON document (same schema as the Rust
    ///         `PlanningFacility` serde model and the
    ///         tests/ExampleCases/hcm/FreewayFacilities/planning_case1.json
    ///         fixture): `sections` (each with sec_type basic/weave/ramp,
    ///         length_mi, lanes, inflow_aadt/outflow_aadt, weave_vr),
    ///         `ffs`, `k_factor`, `growth_factor`, `phf`, `pct_sut`,
    ///         `pct_tt`, `terrain`, and `city_type`.
    #[new]
    #[pyo3(signature = (json=None))]
    pub fn new(json: Option<String>) -> PyResult<Self> {
        let inner = match json {
            Some(text) => serde_json::from_str(&text).map_err(|e| {
                PyValueError::new_err(format!("invalid planning facility JSON: {e}"))
            })?,
            None => LibPlanningFacility::new(),
        };
        Ok(PlanningFacility { inner })
    }

    /// Run the planning-level analysis (Steps 1-5).
    pub fn run_analysis(&mut self) -> PyResult<()> {
        self.inner.run_analysis().map_err(PyValueError::new_err)
    }

    #[getter]
    pub fn num_sections(&self) -> usize {
        self.inner.num_sections()
    }

    #[getter]
    pub fn total_length_mi(&self) -> f64 {
        self.inner.total_length_mi()
    }

    /// Demand-to-capacity ratio for a section and period.
    pub fn dc_ratio(&self, section: usize, period: usize) -> f64 {
        self.inner.dc_ratio(section, period)
    }

    /// Section space mean speed for a section and period, mi/h.
    pub fn section_speed(&self, section: usize, period: usize) -> f64 {
        self.inner.section_speed(section, period)
    }

    /// Section density for a section and period, pc/mi/ln.
    pub fn section_density(&self, section: usize, period: usize) -> f64 {
        self.inner.section_density(section, period)
    }

    /// Facility space mean speed for one period, mi/h.
    pub fn facility_speed(&self, period: usize) -> f64 {
        self.inner.facility_speed(period)
    }

    /// Facility average density for one period, pc/mi/ln.
    pub fn facility_density(&self, period: usize) -> f64 {
        self.inner.facility_density(period)
    }

    /// Facility LOS letter for one period (Exhibit 25-17).
    pub fn facility_los(&self, period: usize) -> String {
        self.inner.facility_los(period).to_string()
    }

    /// Serialize the facility (inputs and computed results) to JSON.
    pub fn to_json(&self) -> PyResult<String> {
        serde_json::to_string(&self.inner)
            .map_err(|e| PyValueError::new_err(format!("serialization failed: {e}")))
    }

    pub fn __repr__(&self) -> String {
        format!(
            "PlanningFacility(sections={}, length={:.2} mi)",
            self.inner.num_sections(),
            self.inner.total_length_mi()
        )
    }
}

pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<FreewayFacility>()?;
    m.add_class::<ManagedLaneFacility>()?;
    m.add_class::<PlanningFacility>()?;
    Ok(())
}
