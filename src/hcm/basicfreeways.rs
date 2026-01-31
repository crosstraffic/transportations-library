use crate::utils::math;
use super::common::{CommonSegment, LevelOfService, LaneCapacity, FacilityCalculation, CityType, BaseLaneCapacity};
use serde::{Deserialize, Serialize};
use super::utils::pce_table::{ET_TABLE_30SUT, ET_TABLE_50SUT, ET_TABLE_70SUT};

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
    /// Capacity, vehicle/hr
    pub capacity: f64,
    /// Number of lanes in analysis direction. Usually it is duplicated in the opposite direction, too.
    pub lane_count: u32,
    /// Road density, pc/mi/ln
    pub density: f64,
    /// Lane length, mi.
    pub length: f64,
    /// Lane width, ft.
    pub lw: Option<f64>,
    /// Right-side lateral clearance, mph.
    pub lc_r: u32,
    /// Left-side lateral clearance, mph.
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
    /// Adjustment for heavy vehicles
    pub phv: f64,
    /// Level of Service
    pub los: Option<LevelOfService>,
}

impl BasicFreeways {
    /// Method to create a new BasicFreeways instance
    pub fn new() -> Self {
        Self {
            // segments: segments.unwrap_or_default(),
            ffs: 65.0,
            ffs_adj: 65.0,
            capacity: 2000.0,
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
        }
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

    fn set_ffs(&mut self, ffs: f64) {
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

    /// Adjustment for total lateral clearance
    fn adjustment_total_lateral_clearance(&mut self) -> Result<f64, String> {
        let tlc = self.lc_r + self.lc_l;

        if self.lc_r > 6 || self.lc_r < 0 {
            return Err(format!(
                "Right-side lateral clearance is within 0 to 6 ft",
            ));
        }

        if self.lc_l > 6 || self.lc_l < 0 {
            return Err(format!(
                "Left-side lateral clearance is within 0 to 6 ft",
            ));
        }

        match (self.lane_count, tlc) {
            // lane_count = 2
            (2, 12) => Ok(0.0),
            (2, 10) => Ok(0.4),
            (2, 8) => Ok(0.9),
            (2, 6) => Ok(1.3),
            (2, 4) => Ok(1.8),
            (2, 2) => Ok(2.2),
            (2, 0) => Ok(2.7),

            // lane_count = 3
            (3, 12..) => Ok(0.0),
            (3, 10) => Ok(0.4),
            (3, 8) => Ok(0.9),
            (3, 6) => Ok(1.3),
            (3, 4) => Ok(1.7),
            (3, 2) => Ok(2.8),
            (3, 0) => Ok(3.9),

            _ => Err(format!(
                "No adjustment defined for lane_count={} tlc={}",
                self.lane_count, tlc
            )),
        }
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

    /// Adjustment for access point density
    fn adjustment_access_point_density(&mut self) -> f64 {
        match self.apd {
            0 => 0.0,
            10 => 2.5,
            20 => 5.0,
            30 => 7.5,
            40.. => 10.0,
            _ => 0.0,
        }
    }

    /// Estimate free-flow speed
    pub fn determine_free_flow_speed(&mut self) -> f64 {

        // Speed adjustment factor
        let saf = 1.0;

        if self.highway_type == "basic" {
            self.estimate_basic_lane_ffs();
        } else {
            self.estimate_multi_lane_ffs();
        }

        // Adjusted free-flow speed
        self.ffs_adj = self.ffs * saf;

        self.ffs_adj
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

    pub fn estimate_capacity(&mut self) -> Result<u32, String> {

        let caf = 1.0;

        self.capacity = match self.highway_type.as_str() {
            "basic" => 2200.0 + 10.0 * (self.ffs_adj - 50.0),
            "multilane" => 1900.0 + 20.0 * (self.ffs_adj - 45.0),
            _ => 2000.0,
        };

        let base_lane_capacity = BaseLaneCapacity {
            highway_type: self.highway_type.clone(),
            speed_limit: self.speed_limit,
        };

        let base_capacity = base_lane_capacity.calculate_capacity() .unwrap_or(0);

        if self.capacity > base_capacity as f64 {
            Err("Estimated capacity exceeds base capacity".to_string())
        } else {
            // Adjust capacity of segment (pc/hr), c_adj
            Ok((self.capacity * caf) as u32)
        }
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

    // Estimate the number of lanes for design analysis
    pub fn estimate_number_of_lanes(&mut self) -> (u32, f64){
        self.adjustment_heavy_vehicle_factor();
        let msf: f64;
        if self.highway_type == "basic" {
            msf = self.determine_basic_max_service_flow_rate();
        } else {
            msf = self.determine_multilane_max_service_flow_rate();
        }

        let demand_flow_rate = self.v_p / (self.phf * self.phv);
        let required_lanes = demand_flow_rate / msf;

        // Needs to be next-higher integer
        let required_lanes_res = required_lanes.ceil() as u32;

        self.set_lane_count(required_lanes_res);

        // In case more investigation is needed, it returns intermediate results too
        (required_lanes_res, required_lanes)
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
    pub fn estimate_density(&mut self) -> f64 {
        // Calculate breakpoint value
        let bp = 3000;
        if self.v_p <= bp as f64 {
            self.density = self.v_p / self.ffs_adj;
        }

        self.density

    }

    /// Determine Level of Service for the segment
    pub fn determine_segment_los(&self) -> LevelOfService {

        let facility = FacilityCalculation { segments: self.set_segments(), city_types: self.city_type };
        let los = facility.los_from_density(self.density, None);

        los
    }
}