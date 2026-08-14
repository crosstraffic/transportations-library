//! Python bindings for the HCM Chapter 25/26 mixed-flow model.
//!
//! Both entry points are JSON in, JSON out rather than classes, because the inputs are small
//! plain records and the outputs are wide result structs that serde already describes. That
//! keeps the binding a single line each and means new fields on the Rust side reach Python
//! without any change here.

use crate::hcm::basicfreeways::composite_grade::CompositeGrade;
use crate::hcm::basicfreeways::mixed_flow::MixedFlowSegment;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

/// Analyze a single grade with the HCM Chapter 26 mixed-flow model.
///
/// Args:
///     config_json: JSON object with ffs (mi/h), length (mi), grade (percent),
///         v_mix (veh/h/ln), p_sut and p_tt (decimals), and optionally caf_ao.
///
/// Returns:
///     JSON object with the full Chapter 26 chain: the capacity adjustment factors,
///     mixed-flow capacity, kinematic truck rates, mixed-flow free-flow speed,
///     breakpoint, calibration speeds, phi, and the mixed-flow speed and density.
///     s_mix and d_mix are null when demand exceeds mixed-flow capacity.
///
/// Raises:
///     ValueError: if the config is malformed, the inputs are out of range, or the
///         grade and speed combination is outside the digitised truck curves.
#[pyfunction]
pub fn analyze_mixed_flow(config_json: &str) -> PyResult<String> {
    let seg = MixedFlowSegment::from_json(config_json)
        .map_err(|e| PyValueError::new_err(format!("invalid mixed-flow config: {e}")))?;
    let result = seg
        .analyze()
        .map_err(|e| PyValueError::new_err(format!("mixed-flow analysis failed: {e}")))?;
    serde_json::to_string(&result)
        .map_err(|e| PyValueError::new_err(format!("serialize error: {e}")))
}

/// Analyze a composite grade with the HCM Chapter 25 mixed-flow model.
///
/// Args:
///     config_json: JSON object with ffs (mi/h), v_mix (veh/h/ln), p_sut and p_tt
///         (decimals), optionally caf_ao, and segments, a list of objects with
///         length (mi) and grade (percent) in the order a vehicle meets them.
///
/// Returns:
///     JSON object with per-segment results, the governing capacity and which segment
///     sets it, the segment travel times, the overall mixed-flow speed, and the spot
///     and space mean speeds for automobiles, SUTs and TTs.
///
/// Raises:
///     ValueError: if the config is malformed, the inputs are out of range, or the
///         chain reaches a grade or entry speed outside the digitised truck curves.
#[pyfunction]
pub fn analyze_composite_grade(config_json: &str) -> PyResult<String> {
    let facility = CompositeGrade::from_json(config_json)
        .map_err(|e| PyValueError::new_err(format!("invalid composite-grade config: {e}")))?;
    let result = facility
        .analyze()
        .map_err(|e| PyValueError::new_err(format!("composite-grade analysis failed: {e}")))?;
    serde_json::to_string(&result)
        .map_err(|e| PyValueError::new_err(format!("serialize error: {e}")))
}

pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(analyze_mixed_flow, m)?)?;
    m.add_function(wrap_pyfunction!(analyze_composite_grade, m)?)?;
    Ok(())
}
