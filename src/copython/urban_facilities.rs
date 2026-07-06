//! Python bindings for HCM Chapter 16 (Urban Street Facilities).

use crate::hcm::urban_facilities::urban_facilities::UrbanFacility as LibUrbanFacility;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

/// HCM Chapter 16 urban street facility analysis (motorized vehicle
/// methodology, one direction of travel).
///
/// Construct from a JSON configuration matching the
/// `tests/ExampleCases/hcm/UrbanFacilities/*.json` fixture format: a
/// `segments` array of Chapter 18 `UrbanSegment` objects (ordered
/// upstream to downstream) plus optional `prop_left_turn_lanes` and
/// `spillback_inputs`. Call `analyze()` (runs the Chapter 18 engine per
/// segment, then aggregates) or `aggregate()` (aggregates already-supplied
/// per-segment measures), then read the results.
#[pyclass]
#[derive(Debug, Clone)]
pub struct UrbanFacility {
    pub inner: LibUrbanFacility,
}

#[pymethods]
impl UrbanFacility {
    /// Create an urban street facility analysis from a JSON configuration
    /// string.
    #[new]
    pub fn new(config_json: &str) -> PyResult<Self> {
        let inner = LibUrbanFacility::from_json(config_json)
            .map_err(|e| PyValueError::new_err(format!("invalid UrbanFacility JSON: {e}")))?;
        Ok(UrbanFacility { inner })
    }

    /// Run the full Chapter 16 pipeline: evaluate every segment with the
    /// Chapter 18 engine, then aggregate (Equations 16-2 through 16-4,
    /// Exhibit 16-3 LOS).
    pub fn analyze(&mut self) -> PyResult<()> {
        self.inner.analyze().map(|_| ()).map_err(PyValueError::new_err)
    }

    /// Aggregate the per-segment measures already present on the segments
    /// (e.g., published Chapter 18 outputs) without re-running the
    /// Chapter 18 engine.
    pub fn aggregate(&mut self) -> PyResult<()> {
        self.inner.aggregate().map(|_| ()).map_err(PyValueError::new_err)
    }

    /// Number of segments on the facility.
    #[getter]
    pub fn num_segments(&self) -> usize {
        self.inner.num_segments()
    }

    /// Facility length Σ L_i, ft.
    #[getter]
    pub fn length_ft(&self) -> f64 {
        self.inner.length_ft()
    }

    /// Facility base free-flow speed S_fo,F, mi/h (Equation 16-2).
    #[getter]
    pub fn base_free_flow_speed_mph(&self) -> Option<f64> {
        self.inner.get_base_ffs_mph()
    }

    /// Facility travel speed S_T,F, mi/h (Equation 16-3).
    #[getter]
    pub fn travel_speed_mph(&self) -> Option<f64> {
        self.inner.get_travel_speed_mph()
    }

    /// Facility spatial stop rate H_F, stops/mi (Equation 16-4).
    #[getter]
    pub fn spatial_stop_rate(&self) -> Option<f64> {
        self.inner.get_spatial_stop_rate()
    }

    /// Critical through-movement volume-to-capacity ratio (largest among
    /// the boundary intersections; Exhibit 16-3 footnote).
    #[getter]
    pub fn critical_vc_ratio(&self) -> Option<f64> {
        self.inner.get_critical_vc_ratio()
    }

    /// Facility motorized vehicle LOS letter (Exhibit 16-3).
    #[getter]
    pub fn los(&self) -> Option<String> {
        self.inner.get_los().map(|l| format!("{l:?}"))
    }

    /// LOS of the poorest-performing segment (Chapter 16 Step 4 context
    /// report).
    #[getter]
    pub fn poorest_segment_los(&self) -> Option<String> {
        self.inner.get_poorest_segment_los().map(|l| format!("{l:?}"))
    }

    /// Facility automobile traveler perception score I_a (Chapter 16 Step
    /// 3 with H_F and the facility P_LTL).
    #[getter]
    pub fn perception_score(&self) -> Option<f64> {
        self.inner.get_perception_score()
    }

    /// Per-segment spillback flags from the queue-storage check hook
    /// (None when no spillback inputs were provided).
    #[getter]
    pub fn spillback_flags(&self) -> Option<Vec<bool>> {
        self.inner.spillback_flags.clone()
    }

    /// Per-segment travel speeds, mi/h (after analyze()).
    pub fn segment_travel_speeds(&self) -> Vec<Option<f64>> {
        self.inner.segments.iter().map(|s| s.travel_speed_mph).collect()
    }

    /// Serialize the full analysis (inputs and results) to JSON.
    pub fn to_json(&self) -> PyResult<String> {
        self.inner
            .to_json()
            .map_err(|e| PyValueError::new_err(format!("serialization failed: {e}")))
    }

    pub fn __repr__(&self) -> String {
        format!(
            "UrbanFacility(segments={}, length={:.0} ft, los={:?})",
            self.inner.num_segments(),
            self.inner.length_ft(),
            self.inner.get_los()
        )
    }

    pub fn __str__(&self) -> String {
        format!(
            "Urban street facility with {} segments",
            self.inner.num_segments()
        )
    }
}

pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<UrbanFacility>()?;
    Ok(())
}
