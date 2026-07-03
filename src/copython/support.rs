use crate::hcm::constraints;
use pyo3::prelude::*;

/// Get all parameter constraints as a JSON string.
///
/// This function returns the library's parameter constraints, which define
/// valid ranges for all input parameters based on HCM and AASHTO standards.
///
/// Returns:
///     str: JSON string containing all constraints
///
/// Example:
///     >>> import transportations_library as tl
///     >>> import json
///     >>> constraints = json.loads(tl.get_constraints())
///     >>> print(constraints['two_lane_highways']['lane_width'])
#[pyfunction]
fn get_constraints() -> String {
    constraints::get_constraints_json()
}

/// Validate Two-Lane Highway input parameters.
///
/// Args:
///     lane_width: Lane width in feet (optional)
///     shoulder_width: Shoulder width in feet (optional)
///     passing_type: Passing type 0, 1, or 2 (optional)
///     hor_class: Horizontal class 0-5 (optional)
///     grade: Grade percentage (optional)
///     phf: Peak hour factor (optional)
///     phv: Percent heavy vehicles (optional)
///     spl: Speed limit in mph (optional)
///
/// Returns:
///     list[str]: List of validation error messages, empty if valid
///
/// Example:
///     >>> import transportations_library as tl
///     >>> errors = tl.validate_input(lane_width=8.0)
///     >>> print(errors)
///     ['lane_width = 8 ft is outside valid range [9, 12]. Source: HCM 7th Edition, Exhibit 15-8']
#[cfg(feature = "with-python")]
#[pyfunction]
#[pyo3(signature = (lane_width=None, shoulder_width=None, passing_type=None, hor_class=None, grade=None, phf=None, phv=None, spl=None))]
fn validate_input(
    lane_width: Option<f64>,
    shoulder_width: Option<f64>,
    passing_type: Option<i32>,
    hor_class: Option<i32>,
    grade: Option<f64>,
    phf: Option<f64>,
    phv: Option<f64>,
    spl: Option<f64>,
) -> Vec<String> {
    constraints::validate_two_lane_highway(
        lane_width,
        shoulder_width,
        passing_type,
        hor_class,
        grade,
        phf,
        phv,
        spl,
    )
}

pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_constraints, m)?)?;
    m.add_function(wrap_pyfunction!(validate_input, m)?)?;
    Ok(())
}
