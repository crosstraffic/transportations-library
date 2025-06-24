// hcm/chapter15/mod.rs
pub mod twolanehighways;

#[cfg(test)]
mod tests; // This imports tests.rs for unit tests

pub use twolanehighways::*;

pub const CHAPTER: u8 = 15;
pub const TITLE: &str = "Two-Lane Highways";