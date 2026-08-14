//! PyO3 bindings, compiled only with the `with-python` feature.
//! One module per HCM chapter plus `support` for constraint helpers.

#[cfg(feature = "with-python")]
pub mod freeway_facilities; // HCM Chapter 10
#[cfg(feature = "with-python")]
pub mod freeway_reliability; // HCM Chapter 11
#[cfg(feature = "with-python")]
pub mod basicfreeways; // HCM Chapter 12
#[cfg(feature = "with-python")]
pub mod mixed_flow; // HCM Chapters 25 and 26 (mixed-flow model)
#[cfg(feature = "with-python")]
pub mod weaving; // HCM Chapter 13
#[cfg(feature = "with-python")]
pub mod merge_diverge; // HCM Chapter 14
#[cfg(feature = "with-python")]
pub mod twolanehighways; // HCM Chapter 15
#[cfg(feature = "with-python")]
pub mod urban_facilities; // HCM Chapter 16
#[cfg(feature = "with-python")]
pub mod urban_reliability; // HCM Chapter 17
#[cfg(feature = "with-python")]
pub mod urban_segments; // HCM Chapter 18
#[cfg(feature = "with-python")]
pub mod signalized; // HCM Chapter 19
#[cfg(feature = "with-python")]
pub mod twsc; // HCM Chapter 20
#[cfg(feature = "with-python")]
pub mod awsc; // HCM Chapter 21
#[cfg(feature = "with-python")]
pub mod roundabouts; // HCM Chapter 22
#[cfg(feature = "with-python")]
pub mod ramp_terminals; // HCM Chapter 23
#[cfg(feature = "with-python")]
pub mod offstreet_pedbike; // HCM Chapter 24
#[cfg(feature = "with-python")]
pub mod support;
#[cfg(feature = "with-python")]
pub mod py_transportationslibrary;
