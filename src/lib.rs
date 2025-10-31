// lib.rs - Clean and focused
//! Transportation Engineering Library
//! 
//! A comprehensive Rust library implementing transportation engineering methodologies
//! from the Highway Capacity Manual (HCM) and other sources.

pub mod hcm;
mod copython;
mod utils;

use std::fmt;
use std::io;
use serde_json;

// Re-export main types for easier access
pub use crate::hcm::*;
pub use copython::py_transportationslibrary::*;

// Library metadata
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const NAME: &str = "transportation_library";

/// Library-wide error types
#[derive(Debug)]
pub enum TransportationError {
    InvalidInput(String),
    CalculationError(String),
    ConfigurationError(String),
    Io(io::Error),
    Json(serde_json::Error),
}

impl From<io::Error> for TransportationError {
    fn from(err: io::Error) -> Self {
        TransportationError::Io(err)
    }
}

impl From<serde_json::Error> for TransportationError {
    fn from(err: serde_json::Error) -> Self {
        TransportationError::Json(err)
    }
}

impl fmt::Display for TransportationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TransportationError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            TransportationError::CalculationError(msg) => write!(f, "Calculation error: {}", msg),
            TransportationError::ConfigurationError(msg) => write!(f, "Configuration error: {}", msg),
            TransportationError::Io(err) => write!(f, "I/O error: {}", err),
            TransportationError::Json(err) => write!(f, "JSON error: {}", err),
        }
    }
}

impl std::error::Error for TransportationError {}

/// Result type used throughout the library
pub type Result<T> = std::result::Result<T, TransportationError>;

// Minimal smoke tests
#[cfg(test)]
mod lib_tests {
    use super::*;
    
    #[test]
    fn test_version_exists() {
        assert!(!VERSION.is_empty());
        assert!(!NAME.is_empty());
    }
    
    #[test]
    fn test_basic_construction() {
        let segments = vec![];
        let highway = TwoLaneHighways::new(segments, None, None, None, None, None);
        assert_eq!(highway.get_segments().len(), 0);
    }
    
    #[test]
    fn test_error_types() {
        let error = TransportationError::InvalidInput("test".to_string());
        assert!(error.to_string().contains("Invalid input"));
    }
}