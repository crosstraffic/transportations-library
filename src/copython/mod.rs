//! PyO3 bindings, compiled only with the `with-python` feature.
//! One module per HCM chapter plus `support` for constraint helpers.

#[cfg(feature = "with-python")]
pub mod chapter12;
#[cfg(feature = "with-python")]
pub mod chapter13;
#[cfg(feature = "with-python")]
pub mod chapter14;
#[cfg(feature = "with-python")]
pub mod chapter15;
#[cfg(feature = "with-python")]
pub mod support;
#[cfg(feature = "with-python")]
pub mod py_transportationslibrary;
