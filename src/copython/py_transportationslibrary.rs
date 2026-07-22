//! Python extension-module entry point. Each HCM chapter's wrapper classes
//! live in their own `copython::<topic>` module (e.g. `copython::twolanehighways`
//! for Chapter 15) and are registered here.

use pyo3::prelude::*;

pub use super::freeway_facilities::*; // HCM Chapter 10
pub use super::freeway_reliability::*; // HCM Chapter 11
pub use super::basicfreeways::*; // HCM Chapter 12
pub use super::weaving::*; // HCM Chapter 13
pub use super::merge_diverge::*; // HCM Chapter 14
pub use super::twolanehighways::*; // HCM Chapter 15
pub use super::urban_facilities::*; // HCM Chapter 16
pub use super::urban_reliability::*; // HCM Chapter 17
pub use super::urban_segments::*; // HCM Chapter 18
pub use super::signalized::*; // HCM Chapter 19
pub use super::twsc::*; // HCM Chapter 20
pub use super::awsc::*; // HCM Chapter 21
pub use super::roundabouts::*; // HCM Chapter 22
pub use super::ramp_terminals::*; // HCM Chapter 23
pub use super::offstreet_pedbike::*; // HCM Chapter 24

#[pymodule]
fn transportations_library(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    super::freeway_facilities::register(m)?;
    super::freeway_reliability::register(m)?;
    super::basicfreeways::register(m)?;
    super::weaving::register(m)?;
    super::merge_diverge::register(m)?;
    super::twolanehighways::register(m)?;
    super::urban_facilities::register(m)?;
    super::urban_reliability::register(m)?;
    super::urban_segments::register(m)?;
    super::signalized::register(m)?;
    super::twsc::register(m)?;
    super::awsc::register(m)?;
    super::roundabouts::register(m)?;
    super::ramp_terminals::register(m)?;
    super::offstreet_pedbike::register(m)?;
    super::support::register(m)?;

    m.add(
        "__doc__",
        "Transportation analysis library implementing HCM 7th Edition methodologies.\n\n\
        Provides capacity and level-of-service analysis per Highway Capacity Manual\n\
        procedures, one set of classes per HCM chapter.\n\n\
        Main Classes:\n\
        - SubSegment, Segment, TwoLaneHighways: HCM Chapter 15 (two-lane highways)\n\
        - BasicFreeways, ManagedLanes: HCM Chapter 12 (basic freeway, multilane\n\
          highway, and basic managed lane segments)\n\
        - WeavingSegment: HCM Chapter 13 (freeway weaving segments)\n\
        - RampSegment: HCM Chapter 14 (freeway merge and diverge segments)\n\
        - FreewayFacility: HCM Chapter 10 (freeway facilities core methodology)\n\
        - FreewayReliability: HCM Chapter 11 (freeway reliability analysis)\n\
        - UrbanFacility: HCM Chapter 16 (urban street facilities)\n\
        - UrbanReliability: HCM Chapter 17 (urban street reliability and ATDM)\n\
        - UrbanSegment: HCM Chapter 18 (urban street segments)\n\
        - SignalizedIntersection: HCM Chapter 19 (signalized intersections)\n\
        - Twsc: HCM Chapter 20 (two-way STOP-controlled intersections)\n\
        - Awsc: HCM Chapter 21 (all-way STOP-controlled intersections)\n\
        - Roundabouts: HCM Chapter 22 (roundabouts)\n\
        - Interchange: HCM Chapter 23 (interchange ramp terminals)\n\
        - ExclusivePedestrianFacility, SharedUsePathPedestrian, OffStreetBicycleFacility:\n\
          HCM Chapter 24 (off-street pedestrian and bicycle facilities)\n\n\
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
