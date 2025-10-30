use std::collections::vec_deque;

use serde::{Deserialize, Serialize};

/// Level of Service enumeration used throughout HCM
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LevelOfService {
    A, B, C, D, E, F
}

/// City type enumeration for urban vs rural contexts
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CityType {
    Urban,
    Rural,
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

/// Common facility calculation parameters
pub struct FacilityCalculation {
    pub segments: Vec<Segment>,
    pub lane_widths: Vec<f64>,
    pub city_types: CityType,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Segment {
    /// Passing Type. TODO: Defined with enum?
    /// 0 -> Passing Constrained
    /// 1 -> Passing Zone
    /// 2 -> Passing Lane
    pub passing_type: usize,
    /// Length of segment, mi.
    pub length: f64,
    /// Number of lanes in the segment.
    pub lanes: i32,
    /// Segment percent grade.
    pub grade: f64,
    /// Posted speed limit, mi/hr.
    pub spl: f64,
    /// Whether the segment has horizontal class or not.
    pub is_hc: Option<bool>,
    /// Demand volume for direction i, veh/hr.
    pub volume: Option<f64>,
    /// Demand volume for opposite direction o, veh/hr. Required for PZ segments.
    /// 1500 veh/hr for PC segments and 0 for PL segments.
    pub volume_op: Option<f64>,
    /// Demand flow rate for analysis direction i, veh/hr
    pub flow_rate: Option<f64>,
    /// Demand flow rate for opposite direction i, veh/hr
    pub flow_rate_o: Option<f64>,
    /// Capacity, veh/hr
    pub capacity: Option<i32>,
    /// Free flow speed, mi/hr
    pub ffs: Option<f64>,
    /// Average speed, mi/hr
    pub avg_speed: Option<f64>,
    /// Vertical class of the segment.
    pub vertical_class: Option<i32>,
    /// Subsegments of the segment.
    // pub subsegments: Option<Vec<SubSegment>>,
    /// Peak hour factor, unitless.
    pub phf: Option<f64>,
    /// Percentage of heavy vehicles, unitless
    pub phv: Option<f64>,
    /// Percent Followers
    pub pf: Option<f64>,
    /// Followers Density
    pub fd: Option<f64>,
    /// Followers Density in the middle of PL section
    pub fd_mid: Option<f64>,
    /// Horizontal class of the segment.
    pub hor_class: Option<i32>,
}


impl FacilityCalculation {

    pub fn new(
        segments: Vec<Segment>,
        lane_widths: Vec<f64>,
    ) -> Self {
        FacilityCalculation {
            segments: segments,
            lane_widths: lane_widths,
            city_types: CityType::Urban,
        }
    }

    /// Calculate facility density given segment lengths and densities
    pub fn determine_density(&self) -> f64 {
        let mut facility_density: f64 = 0.0;
        for segment in self.segments.iter() {
            facility_density += segment.length * (segment.lanes as f64) * segment.fd.unwrap_or(0.0) / ((segment.lanes as f64) * segment.length);
        }
        facility_density
   }

    /// Common method to determine Level of Service based on density
    pub fn los_from_density(&self, density: f64, vc_ratio: Option<f64>) -> LevelOfService {
        if self.city_types == CityType::Urban {
            return self.urban_los_from_density(density, vc_ratio);
        } else {
            return self.rural_los_from_density(density, vc_ratio);
        }
    }

    /// Caluclate LOS in urban scenario
    pub fn urban_los_from_density(&self, density: f64, vc_ratio: Option<f64>) -> LevelOfService {
        if vc_ratio > Some(1.0) {
            return LevelOfService::F;
        }
        match density {
            d if d <= 11.0 => LevelOfService::A,
            d if d <= 18.0 => LevelOfService::B,
            d if d <= 26.0 => LevelOfService::C,
            d if d <= 35.0 => LevelOfService::D,
            d if d <= 45.0 => LevelOfService::E,
            _ => LevelOfService::F,
        }
    }

    /// Caluclate LOS in rural scenario
    pub fn rural_los_from_density(&self, density: f64, vc_ratio: Option<f64>) -> LevelOfService {
        if vc_ratio > Some(1.0) {
            return LevelOfService::F;
        }
        match density {
            d if d <= 6.0 => LevelOfService::A,
            d if d <= 14.0 => LevelOfService::B,
            d if d <= 22.0 => LevelOfService::C,
            d if d <= 29.0 => LevelOfService::D,
            d if d <= 39.0 => LevelOfService::E,
            _ => LevelOfService::F,
        }
    }
}