//! PyO3 bindings, compiled only with the `with-python` feature.
//! One module per HCM chapter plus `support` for constraint helpers.

#[cfg(feature = "with-python")]
pub mod chapter12;
#[cfg(feature = "with-python")]
pub mod chapter15;
#[cfg(feature = "with-python")]
pub mod chapter18;
#[cfg(feature = "with-python")]
pub mod chapter19;
#[cfg(feature = "with-python")]
pub mod chapter20;
#[cfg(feature = "with-python")]
pub mod chapter21;
#[cfg(feature = "with-python")]
pub mod chapter22;
#[cfg(feature = "with-python")]
pub mod support;
#[cfg(feature = "with-python")]
pub mod py_transportationslibrary;
