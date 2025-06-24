// src/hcm/common.rs
//! Common types and utilities shared across HCM chapters

use serde::{Deserialize, Serialize};

/// Level of Service enumeration used throughout HCM
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LevelOfService {
    A, B, C, D, E, F
}

impl From<char> for LevelOfService {
    fn from(c: char) -> Self {
        match c.to_ascii_uppercase() {
            'A' => LevelOfService::A,
            'B' => LevelOfService::B,
            'C' => LevelOfService::C,
            'D' => LevelOfService::D,
            'E' => LevelOfService::E,
            'F' => LevelOfService::F,
            _ => LevelOfService::F, // Default to worst case
        }
    }
}

impl Into<char> for LevelOfService {
    fn into(self) -> char {
        match self {
            LevelOfService::A => 'A',
            LevelOfService::B => 'B',
            LevelOfService::C => 'C',
            LevelOfService::D => 'D',
            LevelOfService::E => 'E',
            LevelOfService::F => 'F',
        }
    }
}

impl std::fmt::Display for LevelOfService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c: char = (*self).into();
        write!(f, "{}", c)
    }
}

/// Common HCM facility types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FacilityType {
    TwoLaneHighway,
    BasicFreeway,
    MultilaneHighway,
    UrbanStreet,
    Intersection,
    Interchange,
}

/// Common traffic flow parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrafficFlow {
    pub volume: f64,           // veh/hr
    pub peak_hour_factor: f64, // unitless
    pub heavy_vehicles: f64,   // percentage
}

impl TrafficFlow {
    pub fn new(volume: f64, phf: f64, hv_percent: f64) -> Self {
        Self {
            volume,
            peak_hour_factor: phf,
            heavy_vehicles: hv_percent,
        }
    }
    
    pub fn demand_flow_rate(&self) -> f64 {
        self.volume / self.peak_hour_factor
    }
}

/// Common geometric parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeometricParams {
    pub lane_width: Option<f64>,      // ft
    pub shoulder_width: Option<f64>,  // ft
    pub median_width: Option<f64>,    // ft
    pub lateral_clearance: Option<f64>, // ft
}

impl Default for GeometricParams {
    fn default() -> Self {
        Self {
            lane_width: Some(12.0),
            shoulder_width: Some(6.0),
            median_width: None,
            lateral_clearance: Some(6.0),
        }
    }
}
