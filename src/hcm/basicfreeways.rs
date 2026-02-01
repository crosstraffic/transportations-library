use crate::utils::math;
use super::common::{CommonSegment, LevelOfService, FacilityCalculation, CityType};
use serde::{Deserialize, Serialize};
use super::utils::pce_table::{ET_TABLE_30SUT, ET_TABLE_50SUT, ET_TABLE_70SUT};

/// Constants from HCM Chapter 12
/// Density at capacity (pc/mi/ln) - Exhibit 12-6
pub const DENSITY_AT_CAPACITY: f64 = 45.0;

/// Exponent calibration parameter for basic freeway segments - Exhibit 12-6
pub const EXPONENT_BASIC_FREEWAY: f64 = 2.0;

/// Exponent calibration parameter for multilane highway segments - Exhibit 12-6
pub const EXPONENT_MULTILANE: f64 = 1.31;

/// Breakpoint for multilane highway segments (constant) - Exhibit 12-6
pub const BREAKPOINT_MULTILANE: f64 = 1400.0;

/// Default base free-flow speed for basic freeway segments - Step 2
pub const DEFAULT_BFFS_FREEWAY: f64 = 75.4;

/// Default values from Exhibit 12-18
/// Required Input Data, Potential Data Sources, and Default Values
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultValues {
    /// Lane width (ft) - default 12 ft, range 10-12 ft
    pub lane_width: f64,
    /// Right-side lateral clearance (ft) - default 10 ft for freeway, 6 ft for multilane
    pub right_lateral_clearance: f64,
    /// Left-side (median) lateral clearance for multilane (ft) - default 6 ft
    pub left_lateral_clearance: f64,
    /// Heavy vehicle percentage (decimal)
    pub heavy_vehicle_pct: f64,
    /// Peak hour factor
    pub phf: f64,
    /// Access point density for multilane highways (points/mi)
    pub access_point_density: u32,
    /// K-factor for planning analysis
    pub k_factor: f64,
    /// D-factor (directional distribution)
    pub d_factor: f64,
}

impl DefaultValues {
    /// Get default values for urban basic freeway segments
    pub fn urban_freeway() -> Self {
        Self {
            lane_width: 12.0,
            right_lateral_clearance: 10.0,
            left_lateral_clearance: 6.0,
            heavy_vehicle_pct: 0.05,  // 5%
            phf: 0.94,
            access_point_density: 0,  // N/A for freeways
            k_factor: 0.09,  // Range 0.08-0.10 for urban
            d_factor: 0.55,
        }
    }

    /// Get default values for rural basic freeway segments
    pub fn rural_freeway() -> Self {
        Self {
            lane_width: 12.0,
            right_lateral_clearance: 10.0,
            left_lateral_clearance: 6.0,
            heavy_vehicle_pct: 0.12,  // 12%
            phf: 0.94,
            access_point_density: 0,
            k_factor: 0.11,  // Range 0.09-0.13 for rural
            d_factor: 0.55,
        }
    }

    /// Get default values for urban multilane highways
    pub fn urban_multilane() -> Self {
        Self {
            lane_width: 12.0,
            right_lateral_clearance: 6.0,
            left_lateral_clearance: 6.0,
            heavy_vehicle_pct: 0.05,  // 5%
            phf: 0.95,
            access_point_density: 16,  // Low density suburban
            k_factor: 0.09,
            d_factor: 0.55,
        }
    }

    /// Get default values for rural multilane highways
    pub fn rural_multilane() -> Self {
        Self {
            lane_width: 12.0,
            right_lateral_clearance: 6.0,
            left_lateral_clearance: 6.0,
            heavy_vehicle_pct: 0.12,  // 12%
            phf: 0.88,
            access_point_density: 8,  // Rural default
            k_factor: 0.11,
            d_factor: 0.55,
        }
    }

    /// Get default values for high-density suburban multilane highways
    pub fn suburban_multilane_high_density() -> Self {
        Self {
            lane_width: 12.0,
            right_lateral_clearance: 6.0,
            left_lateral_clearance: 6.0,
            heavy_vehicle_pct: 0.05,
            phf: 0.95,
            access_point_density: 25,  // High density suburban
            k_factor: 0.09,
            d_factor: 0.55,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicFreeways {
    // /// Segments making up the freeway
    // pub segments: Vec<CommonSegment>,
    /// Access pont density, number of access points per mile
    pub apd: u32,
    /// Total ramp density, number of ramps per mile
    pub trd: u32,
    /// Base free flow speed, mph.
    pub bffs: f64,
    /// Free flow speed, mph.
    pub ffs: f64,
    /// Adjusted free flow speed, mph.
    pub ffs_adj: f64,
    /// Capacity, vehicle/hr (base capacity)
    pub capacity: f64,
    /// Adjusted capacity, vehicle/hr - Equation 12-8
    pub capacity_adj: f64,
    /// Number of lanes in analysis direction. Usually it is duplicated in the opposite direction, too.
    pub lane_count: u32,
    /// Road density, pc/mi/ln
    pub density: f64,
    /// Lane length, mi.
    pub length: f64,
    /// Lane width, ft.
    pub lw: Option<f64>,
    /// Right-side lateral clearance, ft.
    pub lc_r: u32,
    /// Left-side lateral clearance, ft.
    pub lc_l: u32,
    /// Proportation of Single Unit Trucks (SUTs) and Tractor-Trailers (TTs) in traffic stream, percentage float
    pub p_t: Option<f64>,
    /// Passenger equivalent of one heavy vehicle in the traffic stream (PCEs)
    pub e_t: Option<f64>,
    /// Demand flow rate for analysis direction i, veh/hr
    pub demand_flow_i: f64,
    /// Demand volume for direction i, veh/hr.
    pub v_p: f64,
    /// Peak hour factor
    pub phf: f64,
    /// Grade, percentage
    pub grade: f64,
    /// Level, Rolling, Mountainous Terrain
    pub terrain_type: Option<String>,
    /// Percentage of SUTs and TTs in traffic stream, decimal (%)
    pub sut_percentage: u32,
    /// City Type: Urban, Suburban, Rural
    pub city_type: CityType,
    /// Highway type: basic, multilane
    pub highway_type: String,
    /// Median type: undivided, twltl, divided
    pub median_type: String,
    /// Speed limit, mi/hr
    pub speed_limit: u32,
    /// Adjustment for heavy vehicles (f_HV) - Equation 12-10
    pub phv: f64,
    /// Level of Service
    pub los: Option<LevelOfService>,
    /// Speed Adjustment Factor (SAF) - Equation 12-5, Exhibit 12-6
    /// Used for weather, work zones, driver population effects
    pub saf: f64,
    /// Capacity Adjustment Factor (CAF) - Equation 12-8, Exhibit 12-6
    /// Used for weather, incidents, work zones, driver population effects
    pub caf: f64,
    /// Breakpoint in the speed-flow curve (pc/h/ln) - Exhibit 12-6
    pub breakpoint: f64,
    /// Space mean speed of traffic stream (mi/h) - Equation 12-1
    pub speed: f64,
    /// Volume-to-capacity ratio
    pub vc_ratio: f64,
    /// Annual Average Daily Traffic (veh/day) - for planning analysis
    pub aadt: Option<f64>,
    /// K-factor: proportion of AADT in peak hour - Equation 12-20
    pub k_factor: f64,
    /// D-factor: directional distribution - Equation 12-20
    pub d_factor: f64,
}

impl BasicFreeways {
    /// Method to create a new BasicFreeways instance
    pub fn new() -> Self {
        Self {
            ffs: 65.0,
            ffs_adj: 65.0,
            capacity: 2000.0,
            capacity_adj: 2000.0,
            lane_count: 2,
            density: 0.0,
            lc_r: 6,
            lc_l: 6,
            p_t: None,
            e_t: None,
            apd: 0,
            grade: 0.0,
            terrain_type: None,
            sut_percentage: 50,
            city_type: CityType::Urban,
            highway_type: "basic".to_string(),
            median_type: "divided".to_string(),
            trd: 0,
            bffs: 65.0,
            demand_flow_i: 0.0,
            phf: 1.0,
            length: 0.0,
            lw: Some(12.0),
            speed_limit: 65,
            phv: 1.0,
            v_p: 1000.0,
            los: Some('F'.into()),
            // New fields initialized with defaults
            saf: 1.0,  // Speed Adjustment Factor (1.0 = base conditions)
            caf: 1.0,  // Capacity Adjustment Factor (1.0 = base conditions)
            breakpoint: 1000.0,  // Will be calculated based on FFS
            speed: 65.0,  // Space mean speed
            vc_ratio: 0.0,  // Volume-to-capacity ratio
            aadt: None,  // Annual Average Daily Traffic
            k_factor: 0.09,  // Default K-factor for urban freeways
            d_factor: 0.55,  // Default directional distribution
        }
    }

    /// Create a new BasicFreeways with default values for urban freeways
    /// Uses defaults from Exhibit 12-18
    pub fn with_urban_freeway_defaults() -> Self {
        let defaults = DefaultValues::urban_freeway();
        let mut instance = Self::new();
        instance.lw = Some(defaults.lane_width);
        instance.lc_r = defaults.right_lateral_clearance as u32;
        instance.p_t = Some(defaults.heavy_vehicle_pct);
        instance.phf = defaults.phf;
        instance.k_factor = defaults.k_factor;
        instance.d_factor = defaults.d_factor;
        instance.highway_type = "basic".to_string();
        instance.city_type = CityType::Urban;
        instance
    }

    /// Create a new BasicFreeways with default values for rural freeways
    pub fn with_rural_freeway_defaults() -> Self {
        let defaults = DefaultValues::rural_freeway();
        let mut instance = Self::new();
        instance.lw = Some(defaults.lane_width);
        instance.lc_r = defaults.right_lateral_clearance as u32;
        instance.p_t = Some(defaults.heavy_vehicle_pct);
        instance.phf = defaults.phf;
        instance.k_factor = defaults.k_factor;
        instance.d_factor = defaults.d_factor;
        instance.highway_type = "basic".to_string();
        instance.city_type = CityType::Rural;
        instance
    }

    /// Create a new BasicFreeways with default values for urban multilane highways
    pub fn with_urban_multilane_defaults() -> Self {
        let defaults = DefaultValues::urban_multilane();
        let mut instance = Self::new();
        instance.lw = Some(defaults.lane_width);
        instance.lc_r = defaults.right_lateral_clearance as u32;
        instance.lc_l = defaults.left_lateral_clearance as u32;
        instance.p_t = Some(defaults.heavy_vehicle_pct);
        instance.phf = defaults.phf;
        instance.apd = defaults.access_point_density;
        instance.k_factor = defaults.k_factor;
        instance.d_factor = defaults.d_factor;
        instance.highway_type = "multilane".to_string();
        instance.city_type = CityType::Urban;
        instance
    }

    /// Create a new BasicFreeways with default values for rural multilane highways
    pub fn with_rural_multilane_defaults() -> Self {
        let defaults = DefaultValues::rural_multilane();
        let mut instance = Self::new();
        instance.lw = Some(defaults.lane_width);
        instance.lc_r = defaults.right_lateral_clearance as u32;
        instance.lc_l = defaults.left_lateral_clearance as u32;
        instance.p_t = Some(defaults.heavy_vehicle_pct);
        instance.phf = defaults.phf;
        instance.apd = defaults.access_point_density;
        instance.k_factor = defaults.k_factor;
        instance.d_factor = defaults.d_factor;
        instance.highway_type = "multilane".to_string();
        instance.city_type = CityType::Rural;
        instance
    }

    /// Apply defaults from a DefaultValues struct
    pub fn apply_defaults(&mut self, defaults: &DefaultValues) {
        self.lw = Some(defaults.lane_width);
        self.lc_r = defaults.right_lateral_clearance as u32;
        self.lc_l = defaults.left_lateral_clearance as u32;
        self.p_t = Some(defaults.heavy_vehicle_pct);
        self.phf = defaults.phf;
        self.apd = defaults.access_point_density;
        self.k_factor = defaults.k_factor;
        self.d_factor = defaults.d_factor;
    }

    /// Set segments for the BasicFreeways instance with inputs
    pub fn set_segments(&self) -> Vec<CommonSegment> {
        let basic_segment = CommonSegment {
            length: self.length,
            lane_count: self.lane_count as i32,
            lane_width: self.lw,
            grade: self.grade,
            spl: self.speed_limit as f64,
            flow_rate: Some(self.demand_flow_i),
            capacity: Some(self.capacity as i32),
            ffs: Some(self.ffs),
            phf: Some(self.phf),
            phv: self.phv,
            pf: Some(1.0),
            fd: Some(self.density),
            los: self.los.clone(),
        };
        vec![basic_segment]
    }

    pub fn get_ffs(&self) -> f64 {
        self.ffs
    }

    pub fn get_capacity(&self) -> f64 {
        self.capacity
    }

    /// Set free-flow speed directly (e.g., from field measurement)
    pub fn set_ffs(&mut self, ffs: f64) {
        self.ffs = ffs;
    }

    pub fn get_lane_count(&self) -> u32 {
        self.lane_count
    }

    pub fn set_lane_count(&mut self, lane_count: u32) {
        self.lane_count = lane_count;
    }

    pub fn get_density(&self) -> f64 {
        self.density
    }

    /// Set Speed Adjustment Factor (SAF)
    /// Used for weather, work zones, driver population effects
    /// Default: 1.0 (base conditions)
    pub fn set_saf(&mut self, saf: f64) {
        self.saf = saf;
    }

    /// Set Capacity Adjustment Factor (CAF)
    /// Used for weather, incidents, work zones, driver population effects
    /// Default: 1.0 (base conditions)
    pub fn set_caf(&mut self, caf: f64) {
        self.caf = caf;
    }

    /// Set Annual Average Daily Traffic (AADT) for planning analysis
    pub fn set_aadt(&mut self, aadt: f64) {
        self.aadt = Some(aadt);
    }

    /// Set K-factor (proportion of AADT in peak hour)
    /// Typical values: 0.08-0.10 (urban), 0.09-0.13 (rural)
    pub fn set_k_factor(&mut self, k: f64) {
        self.k_factor = k;
    }

    /// Set D-factor (directional distribution)
    /// Typical value: 0.55
    pub fn set_d_factor(&mut self, d: f64) {
        self.d_factor = d;
    }

    /// Get the current speed mean speed
    pub fn get_speed(&self) -> f64 {
        self.speed
    }

    /// Get the volume-to-capacity ratio
    pub fn get_vc_ratio(&self) -> f64 {
        self.vc_ratio
    }

    /// Get the breakpoint value
    pub fn get_breakpoint(&self) -> f64 {
        self.breakpoint
    }

    /// Get adjusted capacity
    pub fn get_adjusted_capacity(&self) -> f64 {
        self.capacity_adj
    }

    /// Adjustment for average lane width
    fn adjustment_average_lane_width(&mut self) -> Result<f64, String> {
        // let avg_lw = self.segments.iter().map(|s| s.lane_width.unwrap_or(12.0)).sum::<f64>() / self.segments.len() as f64;

        match self.lw {
            Some(lw) if lw >= 12.0 => Ok(0.0),
            Some(lw) if lw >= 11.0 && lw < 12.0 => Ok(1.9),
            Some(lw) if lw >= 10.0 && lw < 11.0 => Ok(6.6),
            _ => Err(format!("Average lane width is infeasible {:?}", self.lw)),
        }
    }

    /// Adjustment for right-side lateral clearance
    fn adjustment_right_side_lateral_clearance(&mut self) -> Result<f64, String> {

        match (self.lane_count, self.lc_r) {
            // lane_count = 2
            (2, 6..) => Ok(0.0),
            (2, 5) => Ok(0.6),
            (2, 4) => Ok(1.2),
            (2, 3) => Ok(1.8),
            (2, 2) => Ok(2.4),
            (2, 1) => Ok(3.0),
            (2, 0) => Ok(3.6),

            // lane_count = 3
            (3, 6..) => Ok(0.0),
            (3, 5) => Ok(0.4),
            (3, 4) => Ok(0.8),
            (3, 3) => Ok(1.2),
            (3, 2) => Ok(1.6),
            (3, 1) => Ok(2.0),
            (3, 0) => Ok(2.4),

            // lane_count = 4
            (4, 6..) => Ok(0.0),
            (4, 5) => Ok(0.2),
            (4, 4) => Ok(0.4),
            (4, 3) => Ok(0.6),
            (4, 2) => Ok(0.8),
            (4, 1) => Ok(1.0),
            (4, 0) => Ok(1.2),

            // lane_count >= 5
            (5.., 6..) => Ok(0.0),
            (5.., 5) => Ok(0.1),
            (5.., 4) => Ok(0.2),
            (5.., 3) => Ok(0.3),
            (5.., 2) => Ok(0.4),
            (5.., 1) => Ok(0.5),
            (5.., 0) => Ok(0.6),

            _ => Err(format!(
                "No adjustment defined for lane_count={} lc_r={}",
                self.lane_count, self.lc_r
            )),
        }
    }

    /// Adjustment for total lateral clearance for multilane highways
    /// Exhibit 12-22: Adjustment to FFS for Lateral Clearances for Multilane Highways
    /// Supports interpolation for non-integer TLC values
    fn adjustment_total_lateral_clearance(&mut self) -> Result<f64, String> {
        // Cap lateral clearances at 6 ft max each
        let lc_r = (self.lc_r as f64).min(6.0);
        let lc_l = (self.lc_l as f64).min(6.0);
        let tlc = lc_r + lc_l;

        // Four-Lane Highways (2 lanes in one direction) - Exhibit 12-22
        // TLC: 12→0.0, 10→0.4, 8→0.9, 6→1.3, 4→1.8, 2→3.6, 0→5.4
        let four_lane_values: [(f64, f64); 7] = [
            (12.0, 0.0), (10.0, 0.4), (8.0, 0.9), (6.0, 1.3),
            (4.0, 1.8), (2.0, 3.6), (0.0, 5.4)
        ];

        // Six-Lane Highways (3 lanes in one direction) - Exhibit 12-22
        // TLC: 12→0.0, 10→0.4, 8→0.9, 6→1.3, 4→1.7, 2→2.8, 0→3.9
        let six_lane_values: [(f64, f64); 7] = [
            (12.0, 0.0), (10.0, 0.4), (8.0, 0.9), (6.0, 1.3),
            (4.0, 1.7), (2.0, 2.8), (0.0, 3.9)
        ];

        let values = if self.lane_count <= 2 {
            &four_lane_values
        } else {
            &six_lane_values
        };

        // Interpolate for the given TLC value
        Ok(Self::interpolate_adjustment(tlc, values))
    }

    /// Linear interpolation helper for adjustment factors
    fn interpolate_adjustment(value: f64, table: &[(f64, f64)]) -> f64 {
        // Table is sorted in descending order by first element
        for i in 0..table.len() - 1 {
            let (x1, y1) = table[i];
            let (x2, y2) = table[i + 1];
            if value <= x1 && value >= x2 {
                // Linear interpolation: y = y1 + (y2 - y1) * (x - x1) / (x2 - x1)
                if (x1 - x2).abs() < 0.001 {
                    return y1;
                }
                return y1 + (y2 - y1) * (value - x1) / (x2 - x1);
            }
        }
        // If value is outside range, return boundary value
        if value >= table[0].0 {
            table[0].1
        } else {
            table[table.len() - 1].1
        }
    }

    /// Adjustment for right-side lateral clearance with interpolation support
    /// Exhibit 12-21: Adjustment to FFS for Right-Side Lateral Clearance
    fn adjustment_right_side_lateral_clearance_interpolated(&self, lc_r: f64) -> f64 {
        // Values for different lane counts (descending TLC order)
        let two_lane: [(f64, f64); 7] = [
            (6.0, 0.0), (5.0, 0.6), (4.0, 1.2), (3.0, 1.8),
            (2.0, 2.4), (1.0, 3.0), (0.0, 3.6)
        ];
        let three_lane: [(f64, f64); 7] = [
            (6.0, 0.0), (5.0, 0.4), (4.0, 0.8), (3.0, 1.2),
            (2.0, 1.6), (1.0, 2.0), (0.0, 2.4)
        ];
        let four_lane: [(f64, f64); 7] = [
            (6.0, 0.0), (5.0, 0.2), (4.0, 0.4), (3.0, 0.6),
            (2.0, 0.8), (1.0, 1.0), (0.0, 1.2)
        ];
        let five_plus_lane: [(f64, f64); 7] = [
            (6.0, 0.0), (5.0, 0.1), (4.0, 0.2), (3.0, 0.3),
            (2.0, 0.4), (1.0, 0.5), (0.0, 0.6)
        ];

        let table = match self.lane_count {
            2 => &two_lane,
            3 => &three_lane,
            4 => &four_lane,
            _ => &five_plus_lane,
        };

        Self::interpolate_adjustment(lc_r.min(6.0), table)
    }

    /// Adjustment for median type
    fn adjustment_median_type(&mut self) -> f64 {
        if self.median_type == "undivided" {
            1.6
        } else if self.median_type == "twltl" {
            0.0
        } else if self.median_type == "divided" {
            0.0
        } else {
            0.0
        }
    }

    /// Adjustment for access point density for multilane highways
    /// Exhibit 12-24: Adjustment to FFS for Access Point Density
    /// Each access point per mile decreases FFS by approximately 0.25 mi/h
    /// Supports interpolation as recommended in the exhibit note
    fn adjustment_access_point_density(&mut self) -> f64 {
        // Formula: f_A = 0.25 × APD, capped at 10.0 mi/h for APD ≥ 40
        let apd = self.apd as f64;
        (0.25 * apd).min(10.0)
    }

    /// Alternative access point density adjustment using table lookup with interpolation
    /// Exhibit 12-24 values: 0→0.0, 10→2.5, 20→5.0, 30→7.5, ≥40→10.0
    #[allow(dead_code)]
    fn adjustment_access_point_density_table(&self, apd: f64) -> f64 {
        let table: [(f64, f64); 5] = [
            (0.0, 0.0), (10.0, 2.5), (20.0, 5.0), (30.0, 7.5), (40.0, 10.0)
        ];

        if apd >= 40.0 {
            return 10.0;
        }

        // Find interval and interpolate
        for i in 0..table.len() - 1 {
            let (x1, y1) = table[i];
            let (x2, y2) = table[i + 1];
            if apd >= x1 && apd <= x2 {
                return y1 + (y2 - y1) * (apd - x1) / (x2 - x1);
            }
        }
        0.0
    }

    /// Estimate free-flow speed
    /// Equation 12-2 for basic freeway segments
    /// Equation 12-3 for multilane highway segments
    pub fn determine_free_flow_speed(&mut self) -> f64 {

        if self.highway_type == "basic" {
            let _ = self.estimate_basic_lane_ffs();
        } else {
            let _ = self.estimate_multi_lane_ffs();
        }

        // Adjusted free-flow speed - Equation 12-5
        // FFS_adj = FFS × SAF
        self.ffs_adj = self.ffs * self.saf;

        // Calculate breakpoint after FFS is determined
        self.calculate_breakpoint();

        self.ffs_adj
    }

    /// Calculate breakpoint (BP) for speed-flow curve
    /// Exhibit 12-6:
    /// - Basic freeway: BP = [1,000 + 40 × (75 - FFS_adj)] × CAF^2
    /// - Multilane highway: BP = 1,400 (constant)
    pub fn calculate_breakpoint(&mut self) -> f64 {
        if self.highway_type == "basic" {
            self.breakpoint = (1000.0 + 40.0 * (75.0 - self.ffs_adj)) * self.caf.powi(2);
        } else {
            self.breakpoint = BREAKPOINT_MULTILANE;
        }
        self.breakpoint
    }

    /// Calculate space mean speed using speed-flow relationship
    /// Equation 12-1:
    /// S = FFS_adj                                                    if v_p ≤ BP
    /// S = FFS_adj - (FFS_adj - c_adj/D_c) × ((v_p - BP)/(c_adj - BP))^a   if v_p > BP
    pub fn calculate_speed(&mut self) -> f64 {
        // Ensure breakpoint and capacity are calculated
        if self.breakpoint <= 0.0 {
            self.calculate_breakpoint();
        }
        if self.capacity_adj <= 0.0 {
            let _ = self.estimate_adjusted_capacity();
        }

        // Determine exponent based on highway type
        let exponent = if self.highway_type == "basic" {
            EXPONENT_BASIC_FREEWAY
        } else {
            EXPONENT_MULTILANE
        };

        // Speed at capacity = c_adj / D_c
        let speed_at_capacity = self.capacity_adj / DENSITY_AT_CAPACITY;

        if self.v_p <= self.breakpoint {
            // Constant speed range
            self.speed = self.ffs_adj;
        } else if self.v_p <= self.capacity_adj {
            // Decreasing speed range (parabolic relationship)
            let numerator = self.v_p - self.breakpoint;
            let denominator = self.capacity_adj - self.breakpoint;
            if denominator > 0.0 {
                self.speed = self.ffs_adj - (self.ffs_adj - speed_at_capacity)
                    * (numerator / denominator).powf(exponent);
            } else {
                self.speed = speed_at_capacity;
            }
        } else {
            // Demand exceeds capacity - LOS F
            self.speed = 0.0;  // Indicates breakdown
        }

        self.speed
    }

    /// Calculate Total Lateral Clearance for multilane highways
    /// Equation 12-4: TLC = LC_R + LC_L
    /// Maximum values: LC_R = 6 ft, LC_L = 6 ft, TLC = 12 ft
    pub fn calculate_total_lateral_clearance(&self) -> f64 {
        let lc_r = (self.lc_r as f64).min(6.0);
        let lc_l = (self.lc_l as f64).min(6.0);
        (lc_r + lc_l).min(12.0)
    }

    fn estimate_basic_lane_ffs(&mut self) -> Result<(), String>{
        let f_lw = self.adjustment_average_lane_width().unwrap_or(0.0);
        let f_rlc = self.adjustment_right_side_lateral_clearance().unwrap_or(0.0);

        println!("f_lw: {}, f_rlc: {}", f_lw, f_rlc);
        self.ffs = self.bffs - f_lw - f_rlc - 3.22 * f64::powf(self.trd as f64, 0.84);

        Ok(())
    }

    fn estimate_multi_lane_ffs(&mut self) -> Result<(), String> {
        let f_lw = self.adjustment_average_lane_width().unwrap_or(0.0);
        let f_tlc = self.adjustment_total_lateral_clearance().unwrap_or(0.0);
        let f_m = self.adjustment_median_type();
        let f_a = self.adjustment_access_point_density();

        self.ffs = self.bffs - f_lw - f_tlc - f_m - f_a;

        Ok(())
    }

    /// Estimate base capacity (pc/h/ln)
    /// Equation 12-6 for basic freeway segments: c = 2,200 + 10 × (FFS - 50)
    /// Equation 12-7 for multilane highway segments: c = 1,900 + 20 × (FFS - 45)
    pub fn estimate_capacity(&mut self) -> Result<u32, String> {

        // Equation 12-6 for basic freeway: c = 2200 + 10 × (FFS - 50)
        // Equation 12-7 for multilane highway: c = 1900 + 20 × (FFS - 45)
        self.capacity = match self.highway_type.as_str() {
            "basic" => 2200.0 + 10.0 * (self.ffs_adj - 50.0),
            "multilane" => 1900.0 + 20.0 * (self.ffs_adj - 45.0),
            _ => 2000.0,
        };

        // Capacity cannot exceed maximum from Exhibit 12-4
        // Basic freeway: max 2,400 pc/h/ln (at FFS >= 70 mi/h)
        // Multilane highway: max 2,300 pc/h/ln (at FFS = 60 mi/h)
        let max_capacity = if self.highway_type == "basic" { 2400.0 } else { 2300.0 };
        if self.capacity > max_capacity {
            self.capacity = max_capacity;
        }

        // Calculate adjusted capacity
        let _ = self.estimate_adjusted_capacity();

        Ok(self.capacity as u32)
    }

    /// Estimate adjusted capacity
    /// Equation 12-8: c_adj = c × CAF
    pub fn estimate_adjusted_capacity(&mut self) -> Result<u32, String> {
        self.capacity_adj = self.capacity * self.caf;
        Ok(self.capacity_adj as u32)
    }

    fn adjustment_heavy_vehicle_factor(&mut self) -> f64 {
        match self.terrain_type.as_deref() {
            Some("level") => {
                self.e_t = Some(2.0);
            }
            Some("rolling") => {
                self.e_t = Some(3.0);
            }
            // Chapter 25 and 26 for mixed-flow model
            Some("mountainous") => {
                self.e_t = Some(2.5);
            }
            _ => {
                self.e_t = Some(2.0);
            }
        }
 
        // HashMap lookup
        if self.sut_percentage == 30 {
            if self.p_t >= Some(0.25) {
                self.e_t = match (self.grade, self.length) {
                    (-2.0, 0.125) => Some(1.97),
                    (-2.0, 0.375) => Some(1.97),
                    (-2.0, 0.625) => Some(1.97),
                    (-2.0, 0.875) => Some(1.97),
                    (-2.0, 1.25) => Some(1.97),
                    (-2.0, 1.5) => Some(1.97),

                    (0.0, 0.125) => Some(1.97),
                    (0.0, 0.375) => Some(1.97),
                    (0.0, 0.625) => Some(1.97),
                    (0.0, 0.875) => Some(1.97),
                    (0.0, 1.25) => Some(1.97),
                    (0.0, 1.5) => Some(1.97),

                    (2.0, 0.125) => Some(1.97),
                    (2.0, 0.375) => Some(2.09),
                    (2.0, 0.625) => Some(2.17),
                    (2.0, 0.875) => Some(2.21),
                    (2.0, 1.25) => Some(2.23),
                    (2.0, 1.5) => Some(2.23),

                    (2.5, 0.125) => Some(1.97),
                    (2.5, 0.375) => Some(2.13),
                    (2.5, 0.625) => Some(2.23),
                    (2.5, 0.875) => Some(2.28),
                    (2.5, 1.25) => Some(2.31),
                    (2.5, 1.5) => Some(2.32),

                    (_, _) => todo!("Unhandled grade/length combination"),
                }
            } else {
                let key = (
                    (self.p_t.unwrap() * 100.0) as i32,
                    (self.length * 1000.0) as i32,
                    (self.grade * 100.0) as i32,
                );
                self.e_t = ET_TABLE_30SUT.get(&key).copied();
                // Some(self.e_t) = ET_TABLE30.get(&(self.p_t, self.length, self.grade));
            }
        }

        if self.sut_percentage == 50 {
            if self.p_t >= Some(0.25) {
                self.e_t = match (self.grade, self.length) {
                    (-2.0, 0.125) => Some(1.93),
                    (-2.0, 0.375) => Some(1.93),
                    (-2.0, 0.625) => Some(1.93),
                    (-2.0, 0.875) => Some(1.93),
                    (-2.0, 1.25) => Some(1.93),
                    (-2.0, 1.5) => Some(1.93),

                    (0.0, 0.125) => Some(1.93),
                    (0.0, 0.375) => Some(1.93),
                    (0.0, 0.625) => Some(1.93),
                    (0.0, 0.875) => Some(1.93),
                    (0.0, 1.25) => Some(1.93),
                    (0.0, 1.5) => Some(1.93),

                    (2.0, 0.125) => Some(1.97),
                    (2.0, 0.375) => Some(2.13),
                    (2.0, 0.625) => Some(2.17),
                    (2.0, 0.875) => Some(2.21),
                    (2.0, 1.25) => Some(2.23),
                    (2.0, 1.5) => Some(2.23),

                    (2.5, 0.125) => Some(1.97),
                    (2.5, 0.375) => Some(2.13),
                    (2.5, 0.625) => Some(2.23),
                    (2.5, 0.875) => Some(2.28),
                    (2.5, 1.25) => Some(2.31),
                    (2.5, 1.5) => Some(2.32),

                    (_, _) => todo!("Unhandled grade/length combination"),
                }
            } else {
                let key = (
                    (self.p_t.unwrap() * 100.0) as i32,
                    (self.length * 1000.0) as i32,
                    (self.grade * 100.0) as i32,
                );
                self.e_t = ET_TABLE_50SUT.get(&key).copied();
            }
        }

        if self.sut_percentage == 70 {
            if self.p_t >= Some(0.25) {
                self.e_t = match (self.grade, self.length) {
                    (-2.0, 0.125) => Some(1.83),
                    (-2.0, 0.375) => Some(1.83),
                    (-2.0, 0.625) => Some(1.83),
                    (-2.0, 0.875) => Some(1.83),
                    (-2.0, 1.25) => Some(1.83),
                    (-2.0, 1.5) => Some(1.83),

                    (0.0, 0.125) => Some(1.83),
                    (0.0, 0.375) => Some(1.83),
                    (0.0, 0.625) => Some(1.83),
                    (0.0, 0.875) => Some(1.83),
                    (0.0, 1.25) => Some(1.83),
                    (0.0, 1.5) => Some(1.83),

                    (2.0, 0.125) => Some(1.93),
                    (2.0, 0.375) => Some(2.06),
                    (2.0, 0.625) => Some(2.12),
                    (2.0, 0.875) => Some(2.15),
                    (2.0, 1.25) => Some(2.17),
                    (2.0, 1.5) => Some(2.17),

                    (2.5, 0.125) => Some(1.87),
                    (2.5, 0.375) => Some(2.01),
                    (2.5, 0.625) => Some(2.08),
                    (2.5, 0.875) => Some(2.12),
                    (2.5, 1.25) => Some(2.14),
                    (2.5, 1.5) => Some(2.15),

                    (_, _) => todo!("Unhandled grade/length combination"),
                }
            } else {
                let key = (
                    (self.p_t.unwrap() * 100.0) as i32,
                    (self.length * 1000.0) as i32,
                    (self.grade * 100.0) as i32,
                );
                self.e_t = ET_TABLE_70SUT.get(&key).copied();
            }
        }


        // Default fallback rules
        if self.length == 0.125 {
            self.e_t = match self.p_t {
                Some(0.02) => Some(2.62),
                Some(0.04) => Some(2.37),
                Some(0.05) => Some(2.30),
                Some(0.06) => Some(2.24),
                Some(0.08) => Some(2.17),
                Some(0.10) => Some(2.12),
                Some(0.15) => Some(2.04),
                Some(0.20) => Some(1.99),
                Some(x) if x >= 0.25 => Some(1.97),
                _ => None,
            }
        }

        self.phv = 1.0 / (1.0 + self.p_t.unwrap_or(0.0) * (self.e_t.unwrap_or(0.0) - 1.0));

        math::round_up_to_n_decimal(self.phv, 3)
    }

    // Adjusted demand volume
    pub fn estimate_demand_volume(&mut self) -> f64 {
        self.adjustment_heavy_vehicle_factor();
        println!("demand_flow_i: {} phf: {}, lane_count {}, self.phv {}", self.demand_flow_i, self.phf, self.lane_count, self.phv);
        let _lane_count = self.get_lane_count();
        self.v_p = self.demand_flow_i / (self.phf * self.lane_count as f64 * math::round_up_to_n_decimal(self.phv, 3));

        self.v_p
    }

    /// Estimate the number of lanes for design analysis
    /// Equation 12-22: N = v / MSF_i (rounded up)
    /// Equation 12-23: N = V / (PHF × f_HV × MSF_i)
    /// where N = number of lanes, v = demand flow rate (pc/h),
    /// V = demand volume (veh/h), MSF = max service flow rate (pc/h/ln)
    pub fn estimate_number_of_lanes(&mut self) -> (u32, f64) {
        self.adjustment_heavy_vehicle_factor();

        // Determine maximum service flow rate based on highway type and target LOS
        let msf: f64 = if self.highway_type == "basic" {
            self.determine_basic_max_service_flow_rate()
        } else {
            self.determine_multilane_max_service_flow_rate()
        };

        // Equation 12-21: v = V / (PHF × f_HV)
        // This gives demand flow rate in pc/h (not per lane)
        let demand_flow_rate = self.demand_flow_i / (self.phf * self.phv);

        // Equation 12-22: N = v / MSF_i
        let required_lanes = demand_flow_rate / msf;

        // Always round up to next-higher integer
        let required_lanes_res = required_lanes.ceil() as u32;

        self.set_lane_count(required_lanes_res);

        // In case more investigation is needed, it returns intermediate results too
        (required_lanes_res, required_lanes)
    }

    /// Planning analysis: Estimate number of lanes from AADT
    /// Combines Equation 12-20 (DDHV) with Equation 12-23
    pub fn estimate_lanes_from_aadt(&mut self) -> Option<(u32, f64)> {
        // First calculate DDHV from AADT
        let ddhv = self.estimate_ddhv()?;

        // Set the demand volume
        self.demand_flow_i = ddhv;

        // Use existing method to calculate lanes
        Some(self.estimate_number_of_lanes())
    }

    pub fn determine_basic_max_service_flow_rate(&mut self) -> f64 {
        let _ffs = math::round_up_to_nearest_5(self.ffs_adj);
        let msf = match (_ffs, self.los) {
            (55, Some(LevelOfService::A)) => 600.0,
            (55, Some(LevelOfService::B)) => 990.0,
            (55, Some(LevelOfService::C)) => 1430.0,
            (55, Some(LevelOfService::D)) => 1910.0,
            (55, Some(LevelOfService::E)) => 2250.0,

            (60, Some(LevelOfService::A)) => 600.0,
            (60, Some(LevelOfService::B)) => 1080.0,
            (60, Some(LevelOfService::C)) => 1560.0,
            (60, Some(LevelOfService::D)) => 2000.0,
            (60, Some(LevelOfService::E)) => 2300.0,

            (65, Some(LevelOfService::A)) => 710.0,
            (65, Some(LevelOfService::B)) => 1170.0,
            (65, Some(LevelOfService::C)) => 1660.0,
            (65, Some(LevelOfService::D)) => 2060.0,
            (65, Some(LevelOfService::E)) => 2350.0,

            (70, Some(LevelOfService::A)) => 770.0,
            (70, Some(LevelOfService::B)) => 1260.0,
            (70, Some(LevelOfService::C)) => 1730.0,
            (70, Some(LevelOfService::D)) => 2110.0,
            (70, Some(LevelOfService::E)) => 2400.0,

            (75, Some(LevelOfService::A)) => 820.0,
            (75, Some(LevelOfService::B)) => 1330.0,
            (75, Some(LevelOfService::C)) => 1780.0,
            (75, Some(LevelOfService::D)) => 2130.0,
            (75, Some(LevelOfService::E)) => 2400.0,

            _ =>  2000.0,
        };

        msf
    }

    pub fn determine_multilane_max_service_flow_rate(&mut self) -> f64 {
        let _ffs = math::round_up_to_nearest_5(self.ffs_adj);
        let msf = match (_ffs, self.los) {
            (45, Some(LevelOfService::A)) => 490.0,
            (45, Some(LevelOfService::B)) => 810.0,
            (45, Some(LevelOfService::C)) => 1170.0,
            (45, Some(LevelOfService::D)) => 1550.0,
            (45, Some(LevelOfService::E)) => 1900.0,

            (50, Some(LevelOfService::A)) => 550.0,
            (50, Some(LevelOfService::B)) => 900.0,
            (50, Some(LevelOfService::C)) => 1300.0,
            (50, Some(LevelOfService::D)) => 1680.0,
            (50, Some(LevelOfService::E)) => 2000.0,

            (55, Some(LevelOfService::A)) => 600.0,
            (55, Some(LevelOfService::B)) => 990.0,
            (55, Some(LevelOfService::C)) => 1430.0,
            (55, Some(LevelOfService::D)) => 1790.0,
            (55, Some(LevelOfService::E)) => 2100.0,

            (60, Some(LevelOfService::A)) => 660.0,
            (60, Some(LevelOfService::B)) => 1080.0,
            (60, Some(LevelOfService::C)) => 1530.0,
            (60, Some(LevelOfService::D)) => 1890.0,
            (60, Some(LevelOfService::E)) => 2200.0,

            _ =>  2000.0,
        };

        msf
    }

    /// Estimate density in the segment per lane, pc/mi/ln
    /// Equation 12-11: D = v_p / S
    /// where D = density (pc/mi/ln), v_p = demand flow rate (pc/h/ln), S = mean speed (mi/h)
    pub fn estimate_density(&mut self) -> f64 {
        // First calculate speed if not already done
        if self.speed <= 0.0 {
            self.calculate_speed();
        }

        // Check for LOS F (demand exceeds capacity)
        self.vc_ratio = self.v_p / self.capacity_adj;
        if self.vc_ratio > 1.0 {
            // Demand exceeds capacity - LOS F
            // Density cannot be determined with this method
            // Chapter 10 methodology must be used
            self.density = DENSITY_AT_CAPACITY + 1.0; // Indicates oversaturation
            return self.density;
        }

        // Equation 12-11: D = v_p / S
        if self.speed > 0.0 {
            self.density = self.v_p / self.speed;
        } else {
            self.density = DENSITY_AT_CAPACITY + 1.0; // Indicates breakdown
        }

        self.density
    }

    /// Calculate volume-to-capacity ratio
    pub fn calculate_vc_ratio(&mut self) -> f64 {
        if self.capacity_adj > 0.0 {
            self.vc_ratio = self.v_p / self.capacity_adj;
        } else {
            self.vc_ratio = 0.0;
        }
        self.vc_ratio
    }

    /// Planning analysis: Estimate Directional Design Hour Volume (DDHV)
    /// Equation 12-20: DDHV = AADT × K × D
    /// where K = proportion of AADT in peak hour, D = directional distribution
    pub fn estimate_ddhv(&self) -> Option<f64> {
        self.aadt.map(|aadt| aadt * self.k_factor * self.d_factor)
    }

    /// Calculate service flow rate for a given LOS
    /// Equation 12-24: SF_i = MSF_i × N × f_HV
    /// where SF = service flow rate (veh/h), MSF = max service flow rate (pc/h/ln),
    /// N = number of lanes, f_HV = heavy vehicle adjustment factor
    pub fn calculate_service_flow_rate(&self, msf: f64) -> f64 {
        msf * self.lane_count as f64 * self.phv
    }

    /// Calculate service volume for a given LOS
    /// Equation 12-25: SV_i = SF_i × PHF
    pub fn calculate_service_volume(&self, msf: f64) -> f64 {
        self.calculate_service_flow_rate(msf) * self.phf
    }

    /// Calculate daily service volume for a given LOS
    /// Equation 12-26: DSV_i = SV_i / (K × D)
    pub fn calculate_daily_service_volume(&self, msf: f64) -> f64 {
        let sv = self.calculate_service_volume(msf);
        if self.k_factor > 0.0 && self.d_factor > 0.0 {
            sv / (self.k_factor * self.d_factor)
        } else {
            0.0
        }
    }

    /// Determine Level of Service for the segment
    /// Exhibit 12-15: LOS Criteria for Basic Freeway and Multilane Highway Segments
    /// LOS A: D ≤ 11 pc/mi/ln
    /// LOS B: D > 11-18 pc/mi/ln
    /// LOS C: D > 18-26 pc/mi/ln
    /// LOS D: D > 26-35 pc/mi/ln
    /// LOS E: D > 35-45 pc/mi/ln
    /// LOS F: Demand exceeds capacity OR D > 45 pc/mi/ln
    pub fn determine_segment_los(&mut self) -> LevelOfService {
        // Check for LOS F based on v/c ratio first
        self.calculate_vc_ratio();
        if self.vc_ratio > 1.0 {
            self.los = Some(LevelOfService::F);
            return LevelOfService::F;
        }

        let facility = FacilityCalculation { segments: self.set_segments(), city_types: self.city_type };
        let los = facility.los_from_density(self.density, Some(self.vc_ratio));

        self.los = Some(los);
        los
    }

    /// Run complete operational analysis following HCM methodology
    /// Steps from Exhibit 12-19:
    /// Step 1: Input Data (already set in struct)
    /// Step 2: Estimate and Adjust FFS
    /// Step 3: Estimate and Adjust Capacity
    /// Step 4: Adjust Demand Volume
    /// Step 5: Estimate Speed and Density
    /// Step 6: Determine LOS
    pub fn run_operational_analysis(&mut self) -> LevelOfService {
        // Step 2: Estimate and Adjust FFS
        self.determine_free_flow_speed();

        // Step 3: Estimate and Adjust Capacity
        let _ = self.estimate_capacity();

        // Step 4: Adjust Demand Volume (convert to pc/h/ln)
        self.estimate_demand_volume();

        // Check for LOS F (demand > capacity) before continuing
        self.calculate_vc_ratio();
        if self.vc_ratio > 1.0 {
            self.los = Some(LevelOfService::F);
            return LevelOfService::F;
        }

        // Step 5: Estimate Speed and Density
        self.calculate_speed();
        self.estimate_density();

        // Step 6: Determine LOS
        self.determine_segment_los()
    }

    /// Get summary of analysis results
    pub fn get_analysis_summary(&self) -> AnalysisSummary {
        AnalysisSummary {
            ffs: self.ffs,
            ffs_adj: self.ffs_adj,
            capacity: self.capacity,
            capacity_adj: self.capacity_adj,
            demand_flow_rate: self.v_p,
            speed: self.speed,
            density: self.density,
            vc_ratio: self.vc_ratio,
            los: self.los,
            breakpoint: self.breakpoint,
        }
    }
}

/// Summary of analysis results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisSummary {
    /// Free-flow speed (mi/h)
    pub ffs: f64,
    /// Adjusted free-flow speed (mi/h)
    pub ffs_adj: f64,
    /// Base capacity (pc/h/ln)
    pub capacity: f64,
    /// Adjusted capacity (pc/h/ln)
    pub capacity_adj: f64,
    /// Demand flow rate (pc/h/ln)
    pub demand_flow_rate: f64,
    /// Space mean speed (mi/h)
    pub speed: f64,
    /// Density (pc/mi/ln)
    pub density: f64,
    /// Volume-to-capacity ratio
    pub vc_ratio: f64,
    /// Level of Service
    pub los: Option<LevelOfService>,
    /// Breakpoint (pc/h/ln)
    pub breakpoint: f64,
}