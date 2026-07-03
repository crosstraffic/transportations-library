//! Python extension-module entry point. Each HCM chapter's wrapper classes
//! live in their own `copython::chapterNN` module and are registered here.

use pyo3::prelude::*;

pub use super::chapter12::*;
pub use super::chapter15::*;
pub use super::chapter20::*;
pub use super::chapter21::*;
pub use super::chapter22::*;

#[pymodule]
fn transportations_library(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    super::chapter12::register(m)?;
    super::chapter15::register(m)?;
    super::chapter20::register(m)?;
    super::chapter21::register(m)?;
    super::chapter22::register(m)?;
    super::support::register(m)?;

    m.add(
        "__doc__",
        "Transportation analysis library implementing HCM 7th Edition methodologies.\n\n\
        Provides capacity and level-of-service analysis per Highway Capacity Manual\n\
        procedures, one set of classes per HCM chapter.\n\n\
        Main Classes:\n\
        - SubSegment, Segment, TwoLaneHighways: HCM Chapter 15 (two-lane highways)\n\
        - BasicFreeways: HCM Chapter 12 (basic freeway and multilane highway segments)\n\
        - Twsc: HCM Chapter 20 (two-way STOP-controlled intersections)\n\
        - Awsc: HCM Chapter 21 (all-way STOP-controlled intersections)\n\
        - Roundabouts: HCM Chapter 22 (roundabouts)\n\n\
        Constraint Functions:\n\
        - get_constraints(): Get all parameter constraints as JSON\n\
        - validate_input(): Validate input parameters against HCM/AASHTO constraints\n\n\
        Example Usage:\n\
        >>> from transportations_library import Segment, TwoLaneHighways\n\
        >>> segment = Segment(passing_type=1, length=2.5, grade=3.0, spl=55.0)\n\
        >>> highway = TwoLaneHighways([segment])\n",
    )?;
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;

    Ok(())
}
