// hcm/core/mod.rs
pub mod HcmCore;

#[cfg(test)]
mod tests;

pub use HcmCore::*;

pub const HCM_VERSION: &str = "7th Edition";
pub const TITLE: &str = "HCM Core Data Structures";