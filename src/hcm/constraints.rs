//! # Parameter Constraints Module
//!
//! This module defines the valid ranges and constraints for all parameters
//! used in the HCM methodologies. These constraints serve as the single
//! source of truth for input validation.
//!
//! ## Purpose
//!
//! 1. Define valid parameter ranges based on HCM and AASHTO standards
//! 2. Export constraints as JSON for use by validators
//! 3. Provide runtime validation within the library
//!
//! ## Usage
//!
//! ```rust
//! use transportations_library::hcm::constraints::{get_constraints_json, CONSTRAINTS};
//!
//! // Get all constraints as JSON string
//! let json = get_constraints_json();
//!
//! // Access specific constraint
//! let lane_width = &CONSTRAINTS.two_lane_highways.lane_width;
//! assert!(lane_width.is_valid(11.0));
//! ```

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

/// Constraint for a numeric range parameter.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RangeConstraint {
    /// Parameter name
    pub name: String,
    /// Minimum valid value (inclusive)
    pub min: f64,
    /// Maximum valid value (inclusive)
    pub max: f64,
    /// Unit of measurement
    pub unit: String,
    /// HCM/AASHTO source reference
    pub source: String,
    /// Human-readable description
    pub description: String,
}

impl RangeConstraint {
    /// Check if a value is within the valid range.
    pub fn is_valid(&self, value: f64) -> bool {
        value >= self.min && value <= self.max
    }

    /// Get an error message if the value is invalid.
    pub fn validate(&self, value: f64) -> Option<String> {
        if self.is_valid(value) {
            None
        } else {
            Some(format!(
                "{} = {} {} is outside valid range [{}, {}]. Source: {}",
                self.name, value, self.unit, self.min, self.max, self.source
            ))
        }
    }
}

/// Constraint for an enumerated parameter.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnumConstraint {
    /// Parameter name
    pub name: String,
    /// List of valid values
    pub values: Vec<i32>,
    /// Labels for each value
    pub labels: Vec<String>,
    /// HCM/AASHTO source reference
    pub source: String,
    /// Human-readable description
    pub description: String,
}

impl EnumConstraint {
    /// Check if a value is valid.
    pub fn is_valid(&self, value: i32) -> bool {
        self.values.contains(&value)
    }

    /// Get an error message if the value is invalid.
    pub fn validate(&self, value: i32) -> Option<String> {
        if self.is_valid(value) {
            None
        } else {
            Some(format!(
                "{} = {} is not a valid value. Must be one of {:?}. Source: {}",
                self.name, value, self.values, self.source
            ))
        }
    }
}

/// Constraint based on a lookup table (e.g., speed-radius relationship).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableConstraint {
    /// Parameter name
    pub name: String,
    /// Dependent parameter name
    pub depends_on: String,
    /// Lookup table: key -> minimum required value
    pub table: Vec<(i32, f64)>,
    /// Unit of measurement
    pub unit: String,
    /// HCM/AASHTO source reference
    pub source: String,
    /// Human-readable description
    pub description: String,
}

impl TableConstraint {
    /// Get the minimum required value for a given key.
    pub fn get_min_for(&self, key: i32) -> Option<f64> {
        // Exact match
        for (k, v) in &self.table {
            if *k == key {
                return Some(*v);
            }
        }

        // Interpolate between values
        let mut sorted: Vec<_> = self.table.clone();
        sorted.sort_by_key(|(k, _)| *k);

        for i in 0..sorted.len() - 1 {
            let (k1, v1) = sorted[i];
            let (k2, v2) = sorted[i + 1];
            if k1 < key && key < k2 {
                let ratio = (key - k1) as f64 / (k2 - k1) as f64;
                return Some(v1 + ratio * (v2 - v1));
            }
        }

        None
    }

    /// Check if a value meets the minimum for a given key.
    pub fn is_valid(&self, key: i32, value: f64) -> bool {
        match self.get_min_for(key) {
            Some(min) => value >= min,
            None => true, // Out of table range, can't validate
        }
    }
}

/// Constraints for Two-Lane Highways (HCM Chapter 15).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TwoLaneHighwaysConstraints {
    /// Lane width: 9-12 ft (HCM Exhibit 15-8)
    pub lane_width: RangeConstraint,
    /// Shoulder width: 0-8 ft (HCM Exhibit 15-8)
    pub shoulder_width: RangeConstraint,
    /// Passing type: 0, 1, 2 (HCM Chapter 15.3)
    pub passing_type: EnumConstraint,
    /// Horizontal class: 0-5 (HCM Exhibit 15-22)
    pub horizontal_class: EnumConstraint,
    /// Vertical class: 1-5 (HCM Exhibit 15-11)
    pub vertical_class: EnumConstraint,
    /// Grade: -10% to +10% (AASHTO)
    pub grade: RangeConstraint,
    /// Peak hour factor: 0.5-1.0 (HCM)
    pub phf: RangeConstraint,
    /// Percent heavy vehicles: 0-100% (HCM)
    pub phv: RangeConstraint,
    /// Speed limit: 15-80 mph (AASHTO)
    pub speed_limit: RangeConstraint,
    /// Design radius vs speed (AASHTO Green Book Table 3-7)
    pub speed_radius: TableConstraint,
    /// Access point density: 0-30 points/mi (HCM)
    pub access_point_density: RangeConstraint,
    /// Segment length: 0.25-3.0 mi (HCM Exhibit 15-10)
    pub segment_length: RangeConstraint,
}

/// All constraints for the library.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Constraints {
    /// Library version
    pub version: String,
    /// Two-Lane Highways constraints (Chapter 15)
    pub two_lane_highways: TwoLaneHighwaysConstraints,
}

/// Global constraints instance - single source of truth.
pub static CONSTRAINTS: Lazy<Constraints> = Lazy::new(|| {
    Constraints {
        version: env!("CARGO_PKG_VERSION").to_string(),
        two_lane_highways: TwoLaneHighwaysConstraints {
            lane_width: RangeConstraint {
                name: "lane_width".to_string(),
                min: 9.0,
                max: 12.0,
                unit: "ft".to_string(),
                source: "HCM 7th Edition, Exhibit 15-8".to_string(),
                description: "Lane width for two-lane highways".to_string(),
            },
            shoulder_width: RangeConstraint {
                name: "shoulder_width".to_string(),
                min: 0.0,
                max: 8.0,
                unit: "ft".to_string(),
                source: "HCM 7th Edition, Exhibit 15-8".to_string(),
                description: "Paved shoulder width".to_string(),
            },
            passing_type: EnumConstraint {
                name: "passing_type".to_string(),
                values: vec![0, 1, 2],
                labels: vec![
                    "Passing Constrained (PC)".to_string(),
                    "Passing Zone (PZ)".to_string(),
                    "Passing Lane (PL)".to_string(),
                ],
                source: "HCM 7th Edition, Chapter 15.3".to_string(),
                description: "Segment passing type classification".to_string(),
            },
            horizontal_class: EnumConstraint {
                name: "hor_class".to_string(),
                values: vec![0, 1, 2, 3, 4, 5],
                labels: vec![
                    "Tangent (no curve)".to_string(),
                    "Mild curve".to_string(),
                    "Moderate curve".to_string(),
                    "Sharp curve".to_string(),
                    "Very sharp curve".to_string(),
                    "Severe curve".to_string(),
                ],
                source: "HCM 7th Edition, Exhibit 15-22".to_string(),
                description: "Horizontal alignment class based on curve radius".to_string(),
            },
            vertical_class: EnumConstraint {
                name: "vertical_class".to_string(),
                values: vec![1, 2, 3, 4, 5],
                labels: vec![
                    "Level/gentle".to_string(),
                    "Mild grade".to_string(),
                    "Moderate grade".to_string(),
                    "Steep grade".to_string(),
                    "Severe grade".to_string(),
                ],
                source: "HCM 7th Edition, Exhibit 15-11".to_string(),
                description: "Vertical alignment class based on grade and length".to_string(),
            },
            grade: RangeConstraint {
                name: "grade".to_string(),
                min: -10.0,
                max: 10.0,
                unit: "%".to_string(),
                source: "AASHTO Green Book, Chapter 3".to_string(),
                description: "Segment grade percentage".to_string(),
            },
            phf: RangeConstraint {
                name: "phf".to_string(),
                min: 0.5,
                max: 1.0,
                unit: "".to_string(),
                source: "HCM 7th Edition, Chapter 15".to_string(),
                description: "Peak hour factor".to_string(),
            },
            phv: RangeConstraint {
                name: "phv".to_string(),
                min: 0.0,
                max: 100.0,
                unit: "%".to_string(),
                source: "HCM 7th Edition, Chapter 15".to_string(),
                description: "Percentage of heavy vehicles in traffic stream".to_string(),
            },
            speed_limit: RangeConstraint {
                name: "spl".to_string(),
                min: 15.0,
                max: 80.0,
                unit: "mph".to_string(),
                source: "AASHTO Green Book, Chapter 2".to_string(),
                description: "Posted speed limit".to_string(),
            },
            speed_radius: TableConstraint {
                name: "design_rad".to_string(),
                depends_on: "spl".to_string(),
                table: vec![
                    (15, 50.0),
                    (20, 90.0),
                    (25, 170.0),
                    (30, 230.0),
                    (35, 340.0),
                    (40, 430.0),
                    (45, 560.0),
                    (50, 710.0),
                    (55, 835.0),
                    (60, 1000.0),
                    (65, 1150.0),
                    (70, 1310.0),
                    (75, 1560.0),
                    (80, 1810.0),
                ],
                unit: "ft".to_string(),
                source: "AASHTO Green Book, Table 3-7".to_string(),
                description: "Minimum curve radius for design speed".to_string(),
            },
            access_point_density: RangeConstraint {
                name: "apd".to_string(),
                min: 0.0,
                max: 30.0,
                unit: "points/mi".to_string(),
                source: "HCM 7th Edition, Exhibit 15-6".to_string(),
                description: "Access point density (driveways, intersections)".to_string(),
            },
            segment_length: RangeConstraint {
                name: "length".to_string(),
                min: 0.25,
                max: 3.0,
                unit: "mi".to_string(),
                source: "HCM 7th Edition, Exhibit 15-10".to_string(),
                description: "Analysis segment length".to_string(),
            },
        },
    }
});

/// Get all constraints as a JSON string.
///
/// This function is the primary interface for external validators
/// to access the library's parameter constraints.
pub fn get_constraints_json() -> String {
    serde_json::to_string_pretty(&*CONSTRAINTS).unwrap_or_else(|_| "{}".to_string())
}

/// Validate a Two-Lane Highway input against constraints.
///
/// Returns a list of validation errors, or empty if valid.
pub fn validate_two_lane_highway(
    lane_width: Option<f64>,
    shoulder_width: Option<f64>,
    passing_type: Option<i32>,
    hor_class: Option<i32>,
    grade: Option<f64>,
    phf: Option<f64>,
    phv: Option<f64>,
    spl: Option<f64>,
) -> Vec<String> {
    let mut errors = Vec::new();
    let c = &CONSTRAINTS.two_lane_highways;

    if let Some(v) = lane_width {
        if let Some(err) = c.lane_width.validate(v) {
            errors.push(err);
        }
    }

    if let Some(v) = shoulder_width {
        if let Some(err) = c.shoulder_width.validate(v) {
            errors.push(err);
        }
    }

    if let Some(v) = passing_type {
        if let Some(err) = c.passing_type.validate(v) {
            errors.push(err);
        }
    }

    if let Some(v) = hor_class {
        if let Some(err) = c.horizontal_class.validate(v) {
            errors.push(err);
        }
    }

    if let Some(v) = grade {
        if let Some(err) = c.grade.validate(v) {
            errors.push(err);
        }
    }

    if let Some(v) = phf {
        if let Some(err) = c.phf.validate(v) {
            errors.push(err);
        }
    }

    if let Some(v) = phv {
        if let Some(err) = c.phv.validate(v) {
            errors.push(err);
        }
    }

    if let Some(v) = spl {
        if let Some(err) = c.speed_limit.validate(v) {
            errors.push(err);
        }
    }

    errors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lane_width_constraint() {
        let c = &CONSTRAINTS.two_lane_highways.lane_width;
        assert!(c.is_valid(9.0));
        assert!(c.is_valid(12.0));
        assert!(c.is_valid(10.5));
        assert!(!c.is_valid(8.0));
        assert!(!c.is_valid(13.0));
    }

    #[test]
    fn test_passing_type_constraint() {
        let c = &CONSTRAINTS.two_lane_highways.passing_type;
        assert!(c.is_valid(0));
        assert!(c.is_valid(1));
        assert!(c.is_valid(2));
        assert!(!c.is_valid(3));
        assert!(!c.is_valid(-1));
    }

    #[test]
    fn test_speed_radius_constraint() {
        let c = &CONSTRAINTS.two_lane_highways.speed_radius;

        // Exact match
        assert_eq!(c.get_min_for(50), Some(710.0));
        assert_eq!(c.get_min_for(60), Some(1000.0));

        // Interpolation
        let min_55 = c.get_min_for(55).unwrap();
        assert!(min_55 > 710.0 && min_55 < 1000.0);

        // Validation
        assert!(c.is_valid(60, 1000.0));
        assert!(c.is_valid(60, 1500.0));
        assert!(!c.is_valid(60, 500.0));
    }

    #[test]
    fn test_constraints_json() {
        let json = get_constraints_json();
        assert!(json.contains("lane_width"));
        assert!(json.contains("HCM 7th Edition"));
        assert!(json.contains("two_lane_highways"));
    }

    #[test]
    fn test_validate_function() {
        // Valid input
        let errors = validate_two_lane_highway(
            Some(11.0),
            Some(6.0),
            Some(0),
            Some(3),
            Some(2.0),
            Some(0.95),
            Some(5.0),
            Some(50.0),
        );
        assert!(errors.is_empty());

        // Invalid lane width
        let errors = validate_two_lane_highway(Some(8.0), None, None, None, None, None, None, None);
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("lane_width"));
    }
}
