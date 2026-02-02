use serde::{Deserialize, Serialize};

// ═══════════════════════════════════════════════════════════════════════════════
// Lane Marking Types and Colors (MUTCD Chapter 3)
// ═══════════════════════════════════════════════════════════════════════════════

/// Lane marking line types per MUTCD Section 3A.04
/// Used for pavement marking classification and OpenDRIVE mapping
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LaneMarkingType {
    /// Solid line - discourages or prohibits crossing
    Solid,
    /// Broken line - permissive condition (10ft segments, 30ft gaps standard)
    Broken,
    /// Dotted line - warning of downstream lane change (3ft segments, 9ft gaps)
    Dotted,
    /// Double solid - maximum restriction, no crossing either direction
    DoubleSolid,
    /// Solid + Broken - no crossing from solid side, permitted from broken side
    SolidBroken,
    /// Broken + Solid - permitted from broken side, no crossing from solid side
    BrokenSolid,
    /// No marking present
    None,
}

/// Lane marking colors per MUTCD Section 3A.03
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MarkingColor {
    /// White - same direction separation, right edge
    White,
    /// Yellow - opposite direction separation, left edge of divided highway
    Yellow,
    /// Blue - handicapped parking supplement
    Blue,
    /// Red - wrong way / do not enter (raised markers only)
    Red,
    /// Purple - toll plaza electronic lanes
    Purple,
}

impl LaneMarkingType {
    /// Convert to OpenDRIVE road_mark_type value
    pub fn to_opendrive(&self) -> &'static str {
        match self {
            LaneMarkingType::Solid => "solid",
            LaneMarkingType::Broken => "broken",
            LaneMarkingType::Dotted => "broken", // OpenDRIVE uses broken for both
            LaneMarkingType::DoubleSolid => "solid solid",
            LaneMarkingType::SolidBroken => "solid broken",
            LaneMarkingType::BrokenSolid => "broken solid",
            LaneMarkingType::None => "none",
        }
    }
}

impl MarkingColor {
    /// Convert to OpenDRIVE road_mark_color value
    pub fn to_opendrive(&self) -> &'static str {
        match self {
            MarkingColor::White => "white",
            MarkingColor::Yellow => "yellow",
            MarkingColor::Blue => "blue",
            MarkingColor::Red => "red",
            MarkingColor::Purple => "standard", // OpenDRIVE doesn't have purple
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// MUTCD Marking Standards (Section 3A.04)
// ═══════════════════════════════════════════════════════════════════════════════

/// Normal line width range: 4-6 inches (MUTCD 3A.04)
pub const MARKING_WIDTH_NORMAL_MIN_IN: f64 = 4.0;
pub const MARKING_WIDTH_NORMAL_MAX_IN: f64 = 6.0;

/// Wide line width: at least 2x normal (MUTCD 3A.04)
pub const MARKING_WIDTH_WIDE_MIN_IN: f64 = 8.0;

/// Broken line segment length (ft) - MUTCD 3A.04
pub const BROKEN_LINE_SEGMENT_FT: f64 = 10.0;
/// Broken line gap length (ft) - MUTCD 3A.04
pub const BROKEN_LINE_GAP_FT: f64 = 30.0;

/// Dotted line segment length (ft) - MUTCD 3A.04
pub const DOTTED_LINE_SEGMENT_FT: f64 = 3.0;
/// Dotted line gap length (ft) - MUTCD 3A.04
pub const DOTTED_LINE_GAP_FT: f64 = 9.0;

/// Minimum retroreflectivity for speed >= 35 mph (mcd/m²/lx) - MUTCD 3A.05
pub const MIN_RETROREFLECTIVITY_35MPH: f64 = 50.0;
/// Minimum retroreflectivity for speed >= 70 mph (mcd/m²/lx) - MUTCD 3A.05
pub const MIN_RETROREFLECTIVITY_70MPH: f64 = 100.0;

// ═══════════════════════════════════════════════════════════════════════════════
// Lane Width Standards (Green Book Section 4.3)
// ═══════════════════════════════════════════════════════════════════════════════

/// Lane width by facility type (Green Book Table 4-2)
#[derive(Debug, Clone, Copy)]
pub struct LaneWidthStandards {
    pub min: f64,
    pub standard: f64,
    pub max: f64,
}

/// Freeway lane width: 12 ft standard (Green Book 4.3)
pub const LANE_WIDTH_FREEWAY: LaneWidthStandards = LaneWidthStandards {
    min: 11.0,
    standard: 12.0,
    max: 12.0,
};

/// Arterial lane width: 10-12 ft (Green Book 4.3)
pub const LANE_WIDTH_ARTERIAL: LaneWidthStandards = LaneWidthStandards {
    min: 10.0,
    standard: 12.0,
    max: 12.0,
};

/// Collector lane width: 10-12 ft (Green Book 4.3)
pub const LANE_WIDTH_COLLECTOR: LaneWidthStandards = LaneWidthStandards {
    min: 10.0,
    standard: 11.0,
    max: 12.0,
};

/// Local road lane width: 9-12 ft (Green Book 4.3)
pub const LANE_WIDTH_LOCAL: LaneWidthStandards = LaneWidthStandards {
    min: 9.0,
    standard: 10.0,
    max: 12.0,
};

/// Multilane highway lane width: 10-12 ft (Green Book 4.3)
pub const LANE_WIDTH_MULTILANE: LaneWidthStandards = LaneWidthStandards {
    min: 10.0,
    standard: 12.0,
    max: 12.0,
};

// ═══════════════════════════════════════════════════════════════════════════════
// Shoulder Width Standards (Green Book Section 4.4.2)
// ═══════════════════════════════════════════════════════════════════════════════

/// Shoulder width by facility type (Green Book Table 4-3)
#[derive(Debug, Clone, Copy)]
pub struct ShoulderWidthStandards {
    pub min: f64,
    pub standard: f64,
    pub max: f64,
}

/// Freeway shoulder width: 10-12 ft (Green Book 4.4.2)
pub const SHOULDER_WIDTH_FREEWAY: ShoulderWidthStandards = ShoulderWidthStandards {
    min: 10.0,
    standard: 10.0,
    max: 12.0,
};

/// Arterial shoulder width: 4-10 ft (Green Book 4.4.2)
pub const SHOULDER_WIDTH_ARTERIAL: ShoulderWidthStandards = ShoulderWidthStandards {
    min: 4.0,
    standard: 8.0,
    max: 10.0,
};

/// Collector shoulder width: 2-8 ft (Green Book 4.4.2)
pub const SHOULDER_WIDTH_COLLECTOR: ShoulderWidthStandards = ShoulderWidthStandards {
    min: 2.0,
    standard: 6.0,
    max: 8.0,
};

/// Local road shoulder width: 2-6 ft (Green Book 4.4.2)
pub const SHOULDER_WIDTH_LOCAL: ShoulderWidthStandards = ShoulderWidthStandards {
    min: 2.0,
    standard: 4.0,
    max: 6.0,
};

/// Multilane highway shoulder width: 4-10 ft (Green Book 4.4.2)
pub const SHOULDER_WIDTH_MULTILANE: ShoulderWidthStandards = ShoulderWidthStandards {
    min: 4.0,
    standard: 8.0,
    max: 10.0,
};

// ═══════════════════════════════════════════════════════════════════════════════
// Median Standards (Green Book Section 4.5)
// ═══════════════════════════════════════════════════════════════════════════════

/// Median type classification (Green Book 4.5)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MedianType {
    /// Undivided - no physical separation (painted only)
    Undivided,
    /// Two-Way Left Turn Lane (TWLTL) - flush median with turn lane
    TWLTL,
    /// Raised median - curbed separation
    Raised,
    /// Depressed median - graded separation below roadway
    Depressed,
    /// Barrier median - concrete or cable barrier
    Barrier,
}

impl MedianType {
    /// Convert to OpenDRIVE lane type for median representation
    pub fn to_opendrive(&self) -> &'static str {
        match self {
            MedianType::Undivided => "none",
            MedianType::TWLTL => "bidirectional", // OpenDRIVE bidirectional lane
            MedianType::Raised => "median",
            MedianType::Depressed => "median",
            MedianType::Barrier => "median",
        }
    }

    /// Whether the median provides physical separation
    pub fn is_divided(&self) -> bool {
        !matches!(self, MedianType::Undivided)
    }

    /// Whether the median has a barrier
    pub fn has_barrier(&self) -> bool {
        matches!(self, MedianType::Barrier)
    }
}

/// Minimum median width for TWLTL (Green Book 4.5.2)
pub const MEDIAN_WIDTH_TWLTL_MIN: f64 = 10.0;
/// Desirable median width for TWLTL
pub const MEDIAN_WIDTH_TWLTL_DESIRABLE: f64 = 14.0;

/// Minimum raised median width (Green Book 4.5.3)
pub const MEDIAN_WIDTH_RAISED_MIN: f64 = 4.0;
/// Desirable raised median width for left turn storage
pub const MEDIAN_WIDTH_RAISED_DESIRABLE: f64 = 16.0;

/// Minimum depressed median width (Green Book 4.5.4)
pub const MEDIAN_WIDTH_DEPRESSED_MIN: f64 = 30.0;
/// Desirable depressed median width for recovery area
pub const MEDIAN_WIDTH_DEPRESSED_DESIRABLE: f64 = 60.0;

// ═══════════════════════════════════════════════════════════════════════════════
// Superelevation Standards (Green Book Section 3.3.4)
// ═══════════════════════════════════════════════════════════════════════════════

/// Maximum superelevation rates by area type (Green Book 3.3.4)
/// High-speed roads with no snow/ice: 12%
pub const SUPERELEVATION_MAX_RURAL: f64 = 12.0;
/// Roads with snow/ice considerations: 8%
pub const SUPERELEVATION_MAX_SNOW: f64 = 8.0;
/// Urban low-speed roads: 4-6%
pub const SUPERELEVATION_MAX_URBAN: f64 = 6.0;
/// Minimum rate for drainage: 2%
pub const SUPERELEVATION_MIN: f64 = 2.0;

// ═══════════════════════════════════════════════════════════════════════════════
// Horizontal Alignment Class (HCM Exhibit 15-22, derived from Green Book 3.3)
// ═══════════════════════════════════════════════════════════════════════════════

/// Horizontal alignment classification (HCM Exhibit 15-22)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HorizontalClass {
    /// Class 0: Tangent or very gentle curve (R >= 2550 ft)
    Tangent = 0,
    /// Class 1: Gentle curve (1350 <= R < 2550 ft)
    Gentle = 1,
    /// Class 2: Moderate curve (750 <= R < 1350 ft)
    Moderate = 2,
    /// Class 3: Sharp curve (450 <= R < 750 ft)
    Sharp = 3,
    /// Class 4: Very sharp curve (300 <= R < 450 ft)
    VerySharp = 4,
    /// Class 5: Severe curve (R < 300 ft)
    Severe = 5,
}

impl HorizontalClass {
    /// Get class from design radius (ft)
    pub fn from_radius(radius_ft: f64) -> Self {
        if radius_ft == 0.0 || radius_ft >= 2550.0 {
            HorizontalClass::Tangent
        } else if radius_ft >= 1350.0 {
            HorizontalClass::Gentle
        } else if radius_ft >= 750.0 {
            HorizontalClass::Moderate
        } else if radius_ft >= 450.0 {
            HorizontalClass::Sharp
        } else if radius_ft >= 300.0 {
            HorizontalClass::VerySharp
        } else {
            HorizontalClass::Severe
        }
    }

    /// Get class from curvature (1/ft)
    pub fn from_curvature(curvature_per_ft: f64) -> Self {
        if curvature_per_ft == 0.0 {
            return HorizontalClass::Tangent;
        }
        let radius_ft = 1.0 / curvature_per_ft.abs();
        Self::from_radius(radius_ft)
    }

    /// Get class from curvature in metric units (1/m) - OpenDRIVE convention
    pub fn from_curvature_metric(curvature_per_m: f64) -> Self {
        if curvature_per_m == 0.0 {
            return HorizontalClass::Tangent;
        }
        let radius_m = 1.0 / curvature_per_m.abs();
        let radius_ft = radius_m * 3.28084;
        Self::from_radius(radius_ft)
    }

    /// Get minimum radius threshold for this class (ft)
    pub fn min_radius(&self) -> f64 {
        match self {
            HorizontalClass::Tangent => 2550.0,
            HorizontalClass::Gentle => 1350.0,
            HorizontalClass::Moderate => 750.0,
            HorizontalClass::Sharp => 450.0,
            HorizontalClass::VerySharp => 300.0,
            HorizontalClass::Severe => 0.0,
        }
    }

    /// Get description
    pub fn description(&self) -> &'static str {
        match self {
            HorizontalClass::Tangent => "Tangent (straight)",
            HorizontalClass::Gentle => "Gentle curve",
            HorizontalClass::Moderate => "Moderate curve",
            HorizontalClass::Sharp => "Sharp curve",
            HorizontalClass::VerySharp => "Very sharp curve",
            HorizontalClass::Severe => "Severe curve",
        }
    }

    /// Convert to integer value (for compatibility with HCM tables)
    pub fn as_i32(&self) -> i32 {
        *self as i32
    }
}

impl From<i32> for HorizontalClass {
    fn from(value: i32) -> Self {
        match value {
            0 => HorizontalClass::Tangent,
            1 => HorizontalClass::Gentle,
            2 => HorizontalClass::Moderate,
            3 => HorizontalClass::Sharp,
            4 => HorizontalClass::VerySharp,
            5 => HorizontalClass::Severe,
            _ => HorizontalClass::Severe, // Default to worst case
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Horizontal Class Thresholds (HCM Exhibit 15-22, derived from Green Book 3.3)
// ═══════════════════════════════════════════════════════════════════════════════

/// Horizontal class thresholds based on design radius (ft)
/// These are simplified thresholds; full classification also considers superelevation
/// Source: HCM 7th Edition, Exhibit 15-22
pub const HORIZONTAL_CLASS_THRESHOLDS: [(i32, f64, &str); 6] = [
    (0, 2550.0, "Tangent"),      // Class 0: R >= 2550 ft (straight)
    (1, 1350.0, "Gentle"),       // Class 1: 1350 <= R < 2550 ft
    (2, 750.0, "Moderate"),      // Class 2: 750 <= R < 1350 ft
    (3, 450.0, "Sharp"),         // Class 3: 450 <= R < 750 ft
    (4, 300.0, "Very Sharp"),    // Class 4: 300 <= R < 450 ft
    (5, 0.0, "Severe"),          // Class 5: R < 300 ft
];

/// Convert design radius (ft) to horizontal class (0-5)
/// Simplified version without superelevation consideration
pub fn radius_to_horizontal_class(radius_ft: f64) -> i32 {
    if radius_ft == 0.0 || radius_ft >= 2550.0 {
        0 // Tangent
    } else if radius_ft >= 1350.0 {
        1 // Gentle
    } else if radius_ft >= 750.0 {
        2 // Moderate
    } else if radius_ft >= 450.0 {
        3 // Sharp
    } else if radius_ft >= 300.0 {
        4 // Very Sharp
    } else {
        5 // Severe
    }
}

/// Convert curvature (1/m) to horizontal class
/// curvature = 1/radius, uses OpenDRIVE convention (1/m)
pub fn curvature_to_horizontal_class(curvature_per_m: f64) -> i32 {
    if curvature_per_m == 0.0 {
        return 0; // Tangent (straight)
    }
    let radius_m = 1.0 / curvature_per_m.abs();
    let radius_ft = radius_m * 3.28084; // Convert m to ft
    radius_to_horizontal_class(radius_ft)
}

/// Get horizontal class description
pub fn horizontal_class_description(class: i32) -> &'static str {
    match class {
        0 => "Tangent (straight)",
        1 => "Gentle curve",
        2 => "Moderate curve",
        3 => "Sharp curve",
        4 => "Very sharp curve",
        5 => "Severe curve",
        _ => "Unknown",
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Cross Slope Standards (Green Book Section 4.2.2)
// ═══════════════════════════════════════════════════════════════════════════════

/// Normal paved surface cross slope range (%) - Green Book 4.2.2
pub const CROSS_SLOPE_PAVED_MIN: f64 = 1.5;
pub const CROSS_SLOPE_PAVED_MAX: f64 = 2.0;

/// Unpaved surface cross slope range (%) - Green Book Table 4-1
pub const CROSS_SLOPE_UNPAVED_MIN: f64 = 2.0;
pub const CROSS_SLOPE_UNPAVED_MAX: f64 = 6.0;

/// Maximum algebraic difference at superelevation edge (%) - Green Book 4.4.3
pub const MAX_CROSS_SLOPE_BREAK: f64 = 8.0;

// ═══════════════════════════════════════════════════════════════════════════════
// Level of Service
// ═══════════════════════════════════════════════════════════════════════════════

/// Level of Service enumeration used throughout HCM
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LevelOfService {
    A, B, C, D, E, F
}

/// Bicycle Level of Service criteria from Exhibit 12-31
/// Used for Two-Lane and Multilane Highways
/// The bicycle LOS score is based on traveler perception model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BicycleLOS {
    /// Bicycle LOS score (typically 0.5 to 6.5)
    pub score: f64,
    /// Resulting LOS letter
    pub los: LevelOfService,
}

impl BicycleLOS {
    /// Determine bicycle LOS from score
    /// Exhibit 12-31: Bicycle LOS for Two-Lane and Multilane Highways
    pub fn from_score(score: f64) -> Self {
        let los = match score {
            s if s <= 1.5 => LevelOfService::A,
            s if s <= 2.5 => LevelOfService::B,
            s if s <= 3.5 => LevelOfService::C,
            s if s <= 4.5 => LevelOfService::D,
            s if s <= 5.5 => LevelOfService::E,
            _ => LevelOfService::F,
        };
        Self { score, los }
    }

    /// Get the LOS letter
    pub fn get_los(&self) -> LevelOfService {
        self.los
    }
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
    pub segments: Vec<CommonSegment>,
    pub city_types: CityType,
}

/// Common traffic flow parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrafficFlow {
    pub volume: f64,           // veh/hr
    pub peak_hour_factor: f64, // unitless
    pub heavy_vehicles: f64,   // percentage
}

pub struct BaseLaneCapacity {
    pub highway_type: String,
    pub speed_limit: u32,
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

/// Common geometric parameters (Green Book Chapter 4)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeometricParams {
    /// Lane width in ft (Green Book 4.3: 9-12 ft, 12 ft preferred)
    pub lane_width: Option<f64>,
    /// Shoulder width in ft (Green Book 4.4.2: 2-12 ft)
    pub shoulder_width: Option<f64>,
    /// Median width in ft
    pub median_width: Option<f64>,
    /// Lateral clearance in ft (HCM adjustment factor input)
    pub lateral_clearance: Option<f64>,
    /// Cross slope in % (Green Book 4.2.2: 1.5-2% paved)
    pub cross_slope: Option<f64>,
    /// Superelevation in % (Green Book 3.3: max 8-12%)
    pub superelevation: Option<f64>,
}

impl Default for GeometricParams {
    fn default() -> Self {
        Self {
            lane_width: Some(12.0),        // Green Book preferred
            shoulder_width: Some(6.0),     // Green Book typical
            median_width: None,
            lateral_clearance: Some(6.0),  // HCM base condition
            cross_slope: Some(2.0),        // Green Book typical paved
            superelevation: None,          // Only on curves
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommonSegment {
    /// Length of segment, mi.
    pub length: f64,
    /// Number of lanes in the segment.
    pub lane_count: i32,
    /// Lane width, ft.
    pub lane_width: Option<f64>,
    /// Segment percent grade.
    pub grade: f64,
    /// Posted speed limit, mi/hr.
    pub spl: f64,
    /// Demand flow rate for analysis direction i, veh/hr
    pub flow_rate: Option<f64>,
    /// Capacity, veh/hr
    pub capacity: Option<i32>,
    /// Free flow speed, mi/hr
    pub ffs: Option<f64>,
    /// Subsegments of the segment.
    // pub subsegments: Option<Vec<SubSegment>>,
    /// Peak hour factor, unitless.
    pub phf: Option<f64>,
    /// Percentage of heavy vehicles, unitless
    pub phv: f64,
    /// Percent Followers
    pub pf: Option<f64>,
    /// Followers Density
    pub fd: Option<f64>,
    /// Level of Service
    pub los: Option<LevelOfService>,
}


impl FacilityCalculation {

    pub fn new(
        segments: Vec<CommonSegment>,
        city_types: CityType,
    ) -> Self {
        FacilityCalculation {
            segments: segments,
            city_types: city_types,
        }
    }

    /// Calculate facility density given segment lengths and densities
    pub fn determine_density(&self) -> f64 {
        let mut facility_density: f64 = 0.0;
        for segment in self.segments.iter() {
            facility_density += segment.length * (segment.lane_count as f64) * segment.fd.unwrap_or(0.0) / ((segment.lane_count as f64) * segment.length);
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

pub trait LaneCapacity {
    fn calculate_capacity(&self) -> Option<u32>;
    fn basic_lane_capacity(&self) -> Option<u32>;
    fn multi_lanes_capacity(&self) -> Option<u32>;
}

impl LaneCapacity for BaseLaneCapacity {

    fn calculate_capacity(&self) -> Option<u32> {
        match self.highway_type.as_str() {
            "basic" => self.basic_lane_capacity(),
            "multilane" => self.multi_lanes_capacity(),
            _ => None,
        }
    }

    /// Basic lane capacity pc/h/ln
    fn basic_lane_capacity(&self) -> Option<u32> {
        match self.speed_limit {
            75 => Some(2400),
            70 => Some(2400),
            65 => Some(2350),
            60 => Some(2300),
            55 => Some(2250),
            50 => None,
            45 => None,
            _ => None
        }
    }

    /// Multi-lanes capacity, pc/h/ln
    fn multi_lanes_capacity(&self) -> Option<u32> {
        match self.speed_limit {
            75 => None,
            70 => Some(2300),
            65 => Some(2300),
            60 => Some(2200),
            55 => Some(2100),
            50 => Some(2000),
            45 => Some(1900),
            _ => None
        }
    }
}
