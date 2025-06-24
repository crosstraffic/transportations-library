// src/hcm/mod.rs (replaces hcm.rs)
//! Highway Capacity Manual (HCM) implementations

pub mod chapter15;
pub mod common;
// Future chapters:
// pub mod chapter16; 
// pub mod chapter17;
// pub mod chapter18; 

// Re-export commonly used items
pub use chapter15::*;
pub use common::*;

/// HCM version this library implements
pub const HCM_VERSION: &str = "7th Edition";
