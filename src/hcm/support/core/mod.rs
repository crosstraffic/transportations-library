//! Core HCM facility data structures shared across chapter engines.

pub mod core;

#[cfg(test)]
mod tests;

pub use core::*;

pub const TITLE: &str = "HCM Core Data Structures";
