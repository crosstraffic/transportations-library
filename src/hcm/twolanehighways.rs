//! # Two-Lane Highways Analysis (HCM Chapter 15)
//!
//! This module implements the Highway Capacity Manual (HCM) 7th Edition methodology
//! for analyzing two-lane highways, covering both motorized vehicle and bicycle operations.
//!
//! ## Overview
//!
//! Two-lane highways have one lane for traffic in each direction. The principal characteristic
//! that distinguishes two-lane highways from other uninterrupted-flow facilities is that
//! passing maneuvers take place in the opposing lane of traffic. As demand flows and geometric
//! restrictions increase, opportunities to pass decrease, creating platoons and degrading
//! service quality at relatively low volume-to-capacity ratios.
//!
//! ## Segment Types
//!
//! The methodology classifies segments into three types:
//!
//! - **Passing Constrained (PC)**: Segments where passing in the oncoming lane is prohibited
//!   or effectively negligible due to geometric or sight distance limitations.
//!
//! - **Passing Zone (PZ)**: Segments where passing in the oncoming lane is permitted and
//!   sufficient sight distance exists for safe passing maneuvers.
//!
//! - **Passing Lane (PL)**: Segments with an additional lane in the same travel direction,
//!   allowing faster vehicles to pass slower vehicles without using the opposing lane.
//!
//! ## Performance Measures
//!
//! ### Motorized Vehicle Methodology (Section 3)
//!
//! The methodology produces these performance measures:
//! - **Free-Flow Speed (FFS)**: Speed under low-demand conditions (≤200 veh/h two-way)
//! - **Average Speed**: Mean speed of vehicles at the segment endpoint
//! - **Percent Followers**: Percentage of vehicles with headway ≤2.5 seconds
//! - **Follower Density**: Followers per mile per lane (used for LOS determination)
//!
//! ### Bicycle Methodology (Section 4)
//!
//! The bicycle LOS methodology produces:
//! - **BLOS Score**: Numeric score reflecting bicyclist perception of conditions
//! - **Bicycle LOS**: Letter grade A-F based on BLOS score
//!
//! ## Capacity
//!
//! - Passing Constrained and Passing Zone segments: 1,700 veh/h/ln
//! - Passing Lane segments: 1,100-1,500 veh/h depending on vertical class and heavy vehicle %
//!
//! ## Level of Service Criteria (Exhibit 15-6)
//!
//! | LOS | Higher-Speed (≥50 mi/h) | Lower-Speed (<50 mi/h) |
//! |-----|-------------------------|------------------------|
//! | A   | FD ≤ 2.0               | FD ≤ 2.5              |
//! | B   | FD ≤ 4.0               | FD ≤ 5.0              |
//! | C   | FD ≤ 8.0               | FD ≤ 10.0             |
//! | D   | FD ≤ 12.0              | FD ≤ 15.0             |
//! | E   | FD > 12.0              | FD > 15.0             |
//! | F   | Demand > Capacity      | Demand > Capacity     |
//!
//! ## Key Equations
//!
//! ### Free-Flow Speed (Equations 15-2 to 15-6)
//! ```text
//! BFFS = 1.14 × Spl                           (Eq 15-2)
//! FFS = BFFS - a × HV% - f_LS - f_A           (Eq 15-3)
//! f_LS = 0.6 × (12 - LW) + 0.7 × (6 - SW)     (Eq 15-5)
//! f_A = min(APD/4, 10)                        (Eq 15-6)
//! ```
//!
//! ### Average Speed (Equation 15-7)
//! ```text
//! S = FFS - m × (v_d/1000 - 0.1)^p
//! ```
//! where m (slope) and p (power) coefficients depend on vertical class and segment type.
//!
//! ### Percent Followers (Equation 15-17)
//! ```text
//! PF = 100 × (1 - e^(m × (v_d/1000)^p))
//! ```
//!
//! ### Follower Density (Equation 15-35)
//! ```text
//! FD = (PF × v_d) / (100 × S)
//! ```
//!
//! ## References
//!
//! - HCM 7th Edition, Chapter 15: Two-Lane Highways
//! - NCHRP Project 17-65: Improved Analysis of Two-Lane Highway Capacity and Operational Performance

use crate::utils::math;
use serde::{Deserialize, Serialize};

/// Represents a horizontal curve subsegment within a two-lane highway segment.
///
/// Subsegments are used to model varying horizontal curvature within a single analysis segment.
/// Each horizontal curve affects vehicle speeds and is classified according to Exhibit 15-22.
///
/// # Horizontal Classes (Exhibit 15-22)
///
/// | Class | Description | Typical Radius Range |
/// |-------|-------------|---------------------|
/// | 0     | Tangent (no curve) | ≥2550 ft or no restriction |
/// | 1     | Mild curve | 1350-2549 ft |
/// | 2     | Moderate curve | 750-1349 ft |
/// | 3     | Sharp curve | 450-749 ft |
/// | 4     | Very sharp curve | 300-449 ft |
/// | 5     | Severe curve | <300 ft |
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubSegment {
    /// Length of subsegment, ft.
    pub length: Option<f64>,
    /// Average speed, mi/hr.
    pub avg_speed: Option<f64>,
    /// Design radius of subsegment, ft.
    pub design_rad: Option<f64>,
    /// Central Angel (Not used in HCM. Option for the visualization), deg.
    pub central_angle: Option<f64>,
    /// Horizontal Class
    pub hor_class: Option<i32>,
    /// Superelevation of subsegment, %.
    pub sup_ele: Option<f64>,
}

/// Represents a homogeneous segment of a two-lane highway for analysis.
///
/// Each segment should have consistent properties including traffic demand, grade,
/// lane/shoulder widths, posted speed limit, and passing type. The methodology
/// analyzes each segment in upstream-to-downstream order.
///
/// # Segment Types
///
/// - **Passing Constrained (0)**: No passing opportunities due to sight distance or markings
/// - **Passing Zone (1)**: Passing in opposing lane is permitted with adequate sight distance
/// - **Passing Lane (2)**: Added lane allows passing without using opposing lane
///
/// # Vertical Alignment Classes (Exhibit 15-11)
///
/// Vertical class is determined by segment length and grade percentage:
///
/// | Class | Description | Effect on Operations |
/// |-------|-------------|---------------------|
/// | 1     | Level/gentle | Minimal speed reduction |
/// | 2     | Mild grade | Slight truck speed reduction |
/// | 3     | Moderate grade | Noticeable truck slowdown |
/// | 4     | Steep grade | Significant truck speed reduction |
/// | 5     | Severe grade | Trucks at crawl speed |
///
/// # Minimum/Maximum Segment Lengths (Exhibit 15-10)
///
/// | Vertical Class | PC Min/Max | PZ Min/Max | PL Min/Max |
/// |---------------|------------|------------|------------|
/// | 1-2           | 0.25/3.0 mi | 0.25/2.0 mi | 0.5/3.0 mi |
/// | 3             | 0.25/1.1 mi | 0.25/1.1 mi | 0.5/1.1 mi |
/// | 4-5           | 0.5/3.0 mi | 0.5/2.0 mi | 0.5/3.0 mi |
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Segment {
    /// Passing type classification:
    /// - 0: Passing Constrained (PC) - no passing opportunities
    /// - 1: Passing Zone (PZ) - passing in opposing lane permitted
    /// - 2: Passing Lane (PL) - added lane for passing
    pub passing_type: usize,
    /// Length of segment, mi.
    pub length: f64,
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
    pub subsegments: Option<Vec<SubSegment>>,
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

/// Main analysis structure for Two-Lane Highways (HCM Chapter 15).
///
/// This struct represents a two-lane highway facility composed of one or more
/// contiguous segments. It provides methods for computing all performance measures
/// defined in the HCM methodology.
///
/// # Analysis Workflow
///
/// The recommended analysis sequence for each segment:
///
/// 1. **Step 1**: Identify vertical class with `identify_vertical_class()`
/// 2. **Step 2**: Determine demand flow rates with `determine_demand_flow()`
/// 3. **Step 3**: Determine vertical alignment with `determine_vertical_alignment()`
/// 4. **Step 4**: Determine free-flow speed with `determine_free_flow_speed()`
/// 5. **Step 5**: Estimate average speed with `estimate_average_speed()`
/// 6. **Step 6**: Estimate percent followers with `estimate_percent_followers()`
/// 7. **Step 7**: For passing lanes, calculate lane-specific measures
/// 8. **Step 8**: Calculate follower density with `determine_follower_density_*()` methods
/// 9. **Step 9**: Adjust for upstream passing lanes with `determine_adjustment_to_follower_density()`
/// 10. **Step 10**: Determine segment LOS with `determine_segment_los()`
/// 11. **Step 11**: Combine segments for facility analysis with `determine_facility_los()`
///
/// # Base Conditions (Exhibit 15-8)
///
/// Default values when site-specific data is unavailable:
/// - Lane width: 12 ft
/// - Shoulder width: 6 ft
/// - Access point density: 0 points/mi
/// - Peak hour factor: 0.94
/// - Heavy vehicle percentage: 6%
///
/// # Example
///
/// ```ignore
/// let mut highway = TwoLaneHighways::new(segments, Some(12.0), Some(6.0), Some(5.0), None, None);
///
/// for seg_num in 0..highway.segments.len() {
///     highway.determine_demand_flow(seg_num);
///     highway.determine_vertical_alignment(seg_num);
///     highway.determine_free_flow_speed(seg_num);
///     let (speed, _) = highway.estimate_average_speed(seg_num);
///     highway.estimate_percent_followers(seg_num);
///
///     if highway.segments[seg_num].passing_type == 2 {
///         highway.determine_follower_density_pl(seg_num);
///     } else {
///         highway.determine_follower_density_pc_pz(seg_num);
///     }
///
///     let los = highway.determine_segment_los(seg_num, speed, capacity);
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TwoLaneHighways {
    /// Collection of contiguous highway segments for analysis
    pub segments: Vec<Segment>,
    /// Lane width in feet (default: 12 ft).
    /// Base condition is ≥12 ft; narrower lanes reduce FFS.
    pub lane_width: Option<f64>,
    /// Paved shoulder width in feet (default: 6 ft).
    /// Base condition is ≥6 ft; narrower shoulders reduce FFS.
    pub shoulder_width: Option<f64>,
    /// Access point density in access points per mile (default: 0).
    /// Includes major driveways and side roads with ADT ≥20 vehicles/day.
    pub apd: Option<f64>,
    /// Proportion of heavy vehicles using the faster/passing lane (for PL segments).
    /// Used in Equation 15-28 for passing lane analysis.
    pub pmhvfl: Option<f64>,
    /// Effective downstream distance from passing lane start (mi).
    /// Distance where passing lane benefits persist.
    pub l_de: Option<f64>,
}

/// Implement methods for SubSegment
impl SubSegment {
    /// Method to create a new SubSegment instance
    pub fn new(
        length: Option<f64>,
        avg_speed: Option<f64>,
        hor_class: Option<i32>,
        design_rad: Option<f64>,
        central_angle: Option<f64>,
        sup_ele: Option<f64>,
    ) -> SubSegment {
        SubSegment {
            length,
            avg_speed,
            hor_class,
            design_rad,
            central_angle,
            sup_ele,
        }
    }

    /// Method to get the length of the SubSegment
    pub fn get_length(&self) -> f64 {
        self.length.unwrap_or(0.0)
    }

    pub fn get_avg_speed(&self) -> f64 {
        self.avg_speed.unwrap_or(0.0)
    }

    pub fn set_avg_speed(&mut self, avg_speed: f64) {
        self.avg_speed = Some(avg_speed);
    }

    pub fn get_hor_class(&self) -> i32 {
        self.hor_class.unwrap_or(0)
    }

    pub fn set_hor_class(&mut self, hor_class: i32) {
        self.hor_class = Some(hor_class);
    }

    pub fn get_design_rad(&self) -> f64 {
        self.design_rad.unwrap_or(0.0)
    }

    pub fn set_central_angle(&mut self, central_angle: f64) {
        self.central_angle = Some(central_angle);
    }

    pub fn get_central_angle(&self) -> f64 {
        self.central_angle.unwrap_or(0.0)
    }

    pub fn get_sup_ele(&self) -> f64 {
        self.sup_ele.unwrap_or(0.0)
    }
}

/// Implement methods for Segment
// impl SegmentOperations for Segment {
impl Segment {
    /// Method to create a new Segment instance
    pub fn new(
        passing_type: usize,
        length: f64,
        grade: f64,
        spl: f64,
        is_hc: Option<bool>,
        volume: Option<f64>,
        volume_op: Option<f64>,
        flow_rate: Option<f64>,
        flow_rate_o: Option<f64>,
        capacity: Option<i32>,
        ffs: Option<f64>,
        avg_speed: Option<f64>,
        vertical_class: Option<i32>,
        subsegments: Option<Vec<SubSegment>>,
        phf: Option<f64>,
        phv: Option<f64>,
        pf: Option<f64>,
        fd: Option<f64>,
        fd_mid: Option<f64>,
        hor_class: Option<i32>,
    ) -> Segment {
        Segment {
            passing_type,
            length,
            grade,
            spl,
            is_hc,
            volume,
            volume_op,
            flow_rate,
            flow_rate_o,
            capacity,
            ffs,
            avg_speed,
            vertical_class,
            subsegments,
            phf,
            phv,
            pf,
            fd,
            fd_mid,
            hor_class,
        }
    }

    /// Get passing type
    // fn get_passing_type<'a>(&'a self) -> &'a str {
    //     return &self.passing_type
    // }
    pub fn get_passing_type(&self) -> usize {
        return self.passing_type;
    }

    /// Get total length
    /// Need to check segment length is equal to the total length of subsegments
    pub fn get_length(&self) -> f64 {
        return self.length;
        // TODO
    }

    pub fn get_grade(&self) -> f64 {
        return self.grade;
    }

    pub fn get_spl(&self) -> f64 {
        return self.spl;
    }

    pub fn get_is_hc(&self) -> bool {
        return self.is_hc.unwrap_or(false);
    }

    pub fn get_volume(&self) -> f64 {
        return self.volume.unwrap_or(1000.0);
    }

    pub fn get_volume_op(&self) -> f64 {
        return self.volume_op.unwrap_or(1500.0);
    }

    pub fn get_flow_rate(&self) -> f64 {
        return self.flow_rate.unwrap_or(0.0);
    }

    fn set_flow_rate(&mut self, flow_rate: f64) {
        self.flow_rate = Some(flow_rate);
    }

    pub fn get_flow_rate_o(&self) -> f64 {
        return self.flow_rate_o.unwrap_or(0.0);
    }

    fn set_flow_rate_o(&mut self, flow_rate_o: f64) {
        self.flow_rate_o = Some(flow_rate_o);
    }

    pub fn get_capacity(&self) -> i32 {
        self.capacity.unwrap_or(1700)
    }

    fn set_capacity(&mut self, capacity: i32) {
        self.capacity = Some(capacity)
    }

    pub fn get_ffs(&self) -> f64 {
        return self.ffs.unwrap_or(0.0);
    }

    fn set_ffs(&mut self, ffs: f64) {
        self.ffs = Some(ffs);
    }

    pub fn get_avg_speed(&self) -> f64 {
        return self.avg_speed.unwrap_or(0.0);
    }

    fn set_avg_speed(&mut self, avg_speed: f64) {
        self.avg_speed = Some(avg_speed);
    }

    pub fn get_vertical_class(&self) -> i32 {
        return self.vertical_class.unwrap_or(1);
    }

    fn set_vertical_class(&mut self, vertical_class: i32) {
        self.vertical_class = Some(vertical_class);
    }

    pub fn get_subsegments(&self) -> &Vec<SubSegment> {
        match &self.subsegments {
            Some(subsegments) => subsegments,
            None => {
                // Return empty vec reference - you might want to handle this differently
                static EMPTY_VEC: Vec<SubSegment> = Vec::new();
                &EMPTY_VEC
            }
        }
    }

    fn set_subsegments_avg_speed(&mut self, index: usize, avg_speed: f64) {
        if let Some(ref mut subsegments) = self.subsegments {
            if let Some(subsegment) = subsegments.get_mut(index) {
                subsegment.set_avg_speed(avg_speed);
            }
        }
    }

    fn set_subsegments_hor_class(&mut self, index: usize, hor_class: i32) {
        if let Some(ref mut subsegments) = self.subsegments {
            if let Some(subsegment) = subsegments.get_mut(index) {
                subsegment.set_hor_class(hor_class);
            }
        }
    }

    pub fn get_phf(&self) -> f64 {
        return self.phf.unwrap_or(0.95)
    }

    pub fn get_phv(&self) -> f64 {
        return self.phv.unwrap_or(5.0)
    }

    pub fn get_percent_followers(&self) -> f64 {
        self.pf.unwrap_or(0.0)
    }

    fn set_percent_followers(&mut self, pf: f64) {
        self.pf = Some(pf);
    }

    pub fn get_followers_density(&self) -> f64 {
        self.fd.unwrap_or(0.0)
    }

    fn set_followers_density(&mut self, fd: f64) {
        self.fd = Some(fd);
    }

    pub fn get_followers_density_mid(&self) -> f64 {
        self.fd_mid.unwrap_or(0.0)
    }

    fn set_followers_density_mid(&mut self, fd_mid: f64) {
        self.fd_mid = Some(fd_mid);
    }

    pub fn get_hor_class(&self) -> i32 {
        return self.hor_class.unwrap_or(0);
    }
}

impl TwoLaneHighways {
    /// Creates a new TwoLaneHighways facility for analysis.
    ///
    /// # Arguments
    ///
    /// * `segments` - Vector of highway segments (must be contiguous and in order)
    /// * `lane_width` - Lane width in feet (default: 12 ft if None)
    /// * `shoulder_width` - Paved shoulder width in feet (default: 6 ft if None)
    /// * `apd` - Access point density in points/mile (default: 0 if None)
    /// * `pmhvfl` - Proportion of heavy vehicles in faster lane for PL segments
    /// * `l_de` - Effective distance from passing lane (computed if None)
    ///
    /// # Example
    ///
    /// ```ignore
    /// let highway = TwoLaneHighways::new(
    ///     segments,
    ///     Some(12.0),  // 12-ft lanes
    ///     Some(6.0),   // 6-ft shoulders
    ///     Some(5.0),   // 5 access points/mi
    ///     None,
    ///     None,
    /// );
    /// ```
    pub fn new(
        segments: Vec<Segment>,
        lane_width: Option<f64>,
        shoulder_width: Option<f64>,
        apd: Option<f64>,
        pmhvfl: Option<f64>,
        l_de: Option<f64>,
    ) -> TwoLaneHighways {
        TwoLaneHighways {
            segments,
            lane_width,
            shoulder_width,
            apd,
            pmhvfl,
            l_de,
        }
    }

    // pub fn get_segments(&self) -> &Vec<T> {
    pub fn get_segments(&self) -> &Vec<Segment> {
        return &self.segments;
    }

    /// Step 1: Identify minimum and maximum segment lengths based on vertical class.
    ///
    /// Returns the allowable segment length range based on the vertical alignment class
    /// and segment type, per Exhibit 15-10. Segments outside this range should use
    /// the min/max values for Steps 2-9, with actual lengths used in Step 10.
    ///
    /// # Arguments
    ///
    /// * `seg_num` - Index of the segment to analyze
    ///
    /// # Returns
    ///
    /// Tuple of (minimum_length, maximum_length) in miles
    ///
    /// # Notes
    ///
    /// - Passing lanes exceeding 3 mi may be better analyzed as multilane highways
    /// - Class 3 segments have shorter max length (1.1 mi) due to transitional nature
    pub fn identify_vertical_class(&mut self, seg_num: usize) -> (f64, f64) {
        let mut _min = 0.0;
        let mut _max = 0.0;
        let vc = self.segments[seg_num].get_vertical_class();
        let pt = self.segments[seg_num].get_passing_type();
        if (vc == 1) || (vc == 2) {
            if pt == 0 {
                _min = 0.25;
                _max = 3.0;
            } else if pt == 1 {
                _min = 0.25;
                _max = 2.0;
            } else if pt == 2 {
                _min = 0.5;
                _max = 3.0;
            }
        } else if vc == 3 {
            if pt == 0 {
                _min = 0.25;
                _max = 1.1;
            } else if pt == 1 {
                _min = 0.25;
                _max = 1.1;
            } else if pt == 2 {
                _min = 0.5;
                _max = 1.1;
            }
        } else if (vc == 4) || (vc == 5) {
            if pt == 0 {
                _min = 0.5;
                _max = 3.0;
            } else if pt == 1 {
                _min = 0.5;
                _max = 2.0;
            } else if pt == 2 {
                _min = 0.5;
                _max = 3.0;
            }
        };
        (_min, _max)
    }

    /// Step 2: Determine demand flow rates and segment capacity.
    ///
    /// Calculates demand flow rates using Equation 15-1: v_i = V_i / PHF
    ///
    /// Capacity values per Exhibit 15-5:
    /// - PC and PZ segments: 1,700 veh/h
    /// - PL segments: 1,100-1,500 veh/h depending on HV% and vertical class
    ///
    /// # Arguments
    ///
    /// * `seg_num` - Index of the segment to analyze
    ///
    /// # Returns
    ///
    /// Tuple of (analysis_direction_flow, opposing_direction_flow, capacity)
    ///
    /// # Opposing Flow Assumptions
    ///
    /// - PC segments: v_o = 1,500 veh/h (assumed high opposing flow)
    /// - PZ segments: v_o from input or 0 if not provided
    /// - PL segments: v_o = 0 (passing doesn't use opposing lane)
    pub fn determine_demand_flow(&mut self, seg_num: usize) -> (f64, f64, i32) {
        let v_i = self.segments[seg_num].get_volume();
        let v_o = self.segments[seg_num].get_volume_op();
        let phf = self.segments[seg_num].get_phf();
        let phv = self.segments[seg_num].get_phv();
        let pt = self.segments[seg_num].get_passing_type();
        let vc = self.segments[seg_num].get_vertical_class();

        let demand_flow_i = v_i / phf;
        let mut demand_flow_o = 0.0;
        let mut capacity = 0;

        if (pt == 1) && (v_o == 0.0) {
            capacity = 1700;
        } else if pt == 1 {
            demand_flow_o = v_o / phf;
            capacity = 1700;
        } else if pt == 0 {
            demand_flow_o = 1500.0;
            capacity = 1700;
        } else if pt == 2 {
            demand_flow_o = 0.0;
            if phv < 5.0 {
                capacity = 1500;
            } else if phv >= 5.0 && phv < 10.0 {
                if vc == 1 || vc == 2 || vc == 3 {
                    capacity = 1500;
                } else {
                    capacity = 1500;
                }
            } else if phv >= 10.0 && phv < 15.0 {
                if vc == 1 || vc == 2 || vc == 3 {
                    capacity = 1400;
                } else {
                    capacity = 1300;
                }
            } else if phv >= 15.0 && phv < 20.0 {
                if vc == 1 || vc == 2 || vc == 3 || vc == 4 {
                    capacity = 1300;
                } else {
                    capacity = 1200;
                }
            } else if phv >= 20.0 && phv < 25.0 {
                if vc == 1 || vc == 2 || vc == 3 {
                    capacity = 1300;
                } else if vc == 4 {
                    capacity = 1200;
                } else {
                    capacity = 1100;
                }
            } else if phv >= 25.0 {
                capacity = 1100;
            }
        }
        self.segments[seg_num].set_flow_rate(demand_flow_i);
        self.segments[seg_num].set_capacity(capacity);
        self.segments[seg_num].set_flow_rate_o(demand_flow_o);

        (demand_flow_i, demand_flow_o, capacity)
    }

    /// Step 3: Determine vertical alignment classification (Exhibit 15-11).
    ///
    /// Classifies the segment's vertical alignment into one of 5 classes based on
    /// segment length and grade percentage. The vertical class affects coefficient
    /// values used in speed and percent follower calculations.
    ///
    /// # Vertical Classes
    ///
    /// | Class | Effect | Typical Conditions |
    /// |-------|--------|-------------------|
    /// | 1 | Minimal | Level terrain, short mild grades |
    /// | 2 | Slight | Moderate grades, trucks slightly slower |
    /// | 3 | Moderate | Trucks noticeably slower |
    /// | 4 | Significant | Substantial truck speed reduction |
    /// | 5 | Severe | Trucks at or near crawl speed |
    ///
    /// # Arguments
    ///
    /// * `seg_num` - Index of the segment to analyze
    ///
    /// # Returns
    ///
    /// Vertical alignment class (1-5)
    ///
    /// # Notes
    ///
    /// - Upgrades and downgrades have different classification thresholds
    /// - If the computed class differs from stored value, it updates and re-runs Step 1
    pub fn determine_vertical_alignment(&mut self, seg_num: usize) -> i32 {
        let mut seg_length = self.segments[seg_num].get_length();
        let seg_grade = self.segments[seg_num].get_grade();

        let ver_align: i32;

        if seg_grade >= 0.0 {
            if seg_length <= 0.1 {
                if seg_grade <= 7.0 {
                    ver_align = 1
                } else {
                    ver_align = 2
                };
            } else if seg_length > 0.1 && seg_length <= 0.2 {
                if seg_grade <= 4.0 {
                    ver_align = 1
                } else if seg_grade <= 7.0 {
                    ver_align = 2
                } else {
                    ver_align = 3
                };
            } else if seg_length > 0.2 && seg_length <= 0.3 {
                if seg_grade <= 3.0 {
                    ver_align = 1
                } else if seg_grade <= 5.0 {
                    ver_align = 2
                } else if seg_grade <= 7.0 {
                    ver_align = 3
                } else if seg_grade <= 9.0 {
                    ver_align = 4
                } else {
                    ver_align = 5
                };
            } else if seg_length > 0.3 && seg_length <= 0.4 {
                if seg_grade <= 2.0 {
                    ver_align = 1
                } else if seg_grade <= 4.0 {
                    ver_align = 2
                } else if seg_grade <= 6.0 {
                    ver_align = 3
                } else if seg_grade <= 7.0 {
                    ver_align = 4
                } else {
                    ver_align = 5
                };
            } else if seg_length > 0.4 && seg_length <= 0.5 {
                if seg_grade <= 2.0 {
                    ver_align = 1
                } else if seg_grade <= 4.0 {
                    ver_align = 2
                } else if seg_grade <= 3.0 {
                    ver_align = 2
                } else if seg_grade <= 5.0 {
                    ver_align = 3
                } else if seg_grade <= 6.0 {
                    ver_align = 4
                } else {
                    ver_align = 5
                };
            } else if seg_length > 0.6 && seg_length <= 0.7 {
                if seg_grade <= 2.0 {
                    ver_align = 1
                } else if seg_grade <= 3.0 {
                    ver_align = 2
                } else if seg_grade <= 4.0 {
                    ver_align = 3
                } else if seg_grade <= 6.0 {
                    ver_align = 4
                } else {
                    ver_align = 5
                };
            } else if seg_length > 0.8 && seg_length <= 1.1 {
                if seg_grade <= 2.0 {
                    ver_align = 1
                } else if seg_grade <= 3.0 {
                    ver_align = 2
                } else if seg_grade <= 4.0 {
                    ver_align = 3
                } else if seg_grade <= 5.0 {
                    ver_align = 4
                } else {
                    ver_align = 5
                };
            } else {
                if seg_grade <= 2.0 {
                    ver_align = 1
                } else if seg_grade <= 3.0 {
                    ver_align = 2
                } else if seg_grade <= 5.0 {
                    ver_align = 4
                } else {
                    ver_align = 5
                };
            }
        } else {
            seg_length = -1.0 * seg_length;
            if seg_length <= 0.1 {
                if seg_grade <= 8.0 {
                    ver_align = 1
                } else {
                    ver_align = 2
                };
            } else if seg_length > 0.1 && seg_length <= 0.2 {
                if seg_grade <= 5.0 {
                    ver_align = 1
                } else if seg_grade <= 8.0 {
                    ver_align = 2
                } else {
                    ver_align = 3
                };
            } else if seg_length > 0.2 && seg_length <= 0.3 {
                if seg_grade <= 4.0 {
                    ver_align = 1
                } else if seg_grade <= 6.0 {
                    ver_align = 2
                } else if seg_grade <= 8.0 {
                    ver_align = 3
                } else if seg_grade <= 9.0 {
                    ver_align = 4
                } else {
                    ver_align = 5
                };
            } else if seg_length > 0.3 && seg_length <= 0.4 {
                if seg_grade <= 2.0 {
                    ver_align = 1
                } else if seg_grade <= 5.0 {
                    ver_align = 2
                } else if seg_grade <= 6.0 {
                    ver_align = 3
                } else if seg_grade <= 8.0 {
                    ver_align = 4
                } else {
                    ver_align = 5
                };
            } else if seg_length > 0.4 && seg_length <= 0.5 {
                if seg_grade <= 3.0 {
                    ver_align = 1
                } else if seg_grade <= 4.0 {
                    ver_align = 2
                } else if seg_grade <= 6.0 {
                    ver_align = 3
                } else if seg_grade <= 7.0 {
                    ver_align = 4
                } else {
                    ver_align = 5
                };
            } else if seg_length > 0.5 && seg_length <= 0.7 {
                if seg_grade <= 3.0 {
                    ver_align = 1
                } else if seg_grade <= 4.0 {
                    ver_align = 2
                } else if seg_grade <= 5.0 {
                    ver_align = 3
                } else if seg_grade <= 6.0 {
                    ver_align = 4
                } else {
                    ver_align = 5
                };
            } else if seg_length > 0.7 && seg_length <= 0.8 {
                if seg_grade <= 3.0 {
                    ver_align = 1
                } else if seg_grade <= 4.0 {
                    ver_align = 3
                } else if seg_grade <= 6.0 {
                    ver_align = 4
                } else {
                    ver_align = 5
                };
            } else if seg_length > 0.8 && seg_length <= 0.9 {
                if seg_grade <= 3.0 {
                    ver_align = 1
                } else if seg_grade <= 4.0 {
                    ver_align = 3
                } else if seg_grade <= 5.0 {
                    ver_align = 4
                } else {
                    ver_align = 5
                };
            } else if seg_length > 0.9 && seg_length <= 1.1 {
                if seg_grade <= 2.0 {
                    ver_align = 1
                } else if seg_grade <= 3.0 {
                    ver_align = 2
                } else if seg_grade <= 4.0 {
                    ver_align = 3
                } else if seg_grade <= 5.0 {
                    ver_align = 4
                } else {
                    ver_align = 5
                };
            } else {
                if seg_grade <= 2.0 {
                    ver_align = 1
                } else if seg_grade <= 3.0 {
                    ver_align = 2
                } else if seg_grade <= 5.0 {
                    ver_align = 4
                } else {
                    ver_align = 5
                };
            }
        }
        if ver_align != self.segments[seg_num].get_vertical_class() {
            self.segments[seg_num].set_vertical_class(ver_align);
            // Run step 1 again.
            self.identify_vertical_class(seg_num);
        }

        ver_align
    }

    /// Step 4: Determine free-flow speed (Equations 15-2 to 15-6).
    ///
    /// Estimates the free-flow speed based on posted speed limit, heavy vehicle percentage,
    /// lane/shoulder width, and access point density.
    ///
    /// # Equations
    ///
    /// ```text
    /// BFFS = 1.14 × Spl                                              (Eq 15-2)
    /// FFS = BFFS - a × HV% - f_LS - f_A                              (Eq 15-3)
    /// a = max(0.0333, a0 + a1×BFFS + a2×L + ...)                     (Eq 15-4)
    /// f_LS = 0.6 × (12 - LW) + 0.7 × (6 - SW)                        (Eq 15-5)
    /// f_A = min(APD/4, 10)                                           (Eq 15-6)
    /// ```
    ///
    /// # Arguments
    ///
    /// * `seg_num` - Index of the segment to analyze
    ///
    /// # Returns
    ///
    /// Free-flow speed in mi/h
    ///
    /// # Coefficient Table (Exhibit 15-12)
    ///
    /// Coefficients a0-a5 vary by vertical class:
    /// - Class 1: All zeros (no HV effect on level terrain)
    /// - Classes 2-5: Increasing HV speed reduction with steeper grades
    pub fn determine_free_flow_speed(&mut self, seg_num: usize) -> f64 {
        let spl = self.segments[seg_num].get_spl();
        let vc = self.segments[seg_num].get_vertical_class();
        let vo = self.segments[seg_num].get_flow_rate_o();
        let lw = self.lane_width.unwrap_or(12.0);
        let sw = self.shoulder_width.unwrap_or(6.0);
        let apd = self.apd.unwrap_or(5.0);
        let phv = self.segments[seg_num].get_phv();
        let seg_length = self.segments[seg_num].get_length();

        let ffs: f64;

        let bffs = 1.14 * spl;
        let mut a0 = 0.0;
        let mut a1 = 0.0;
        let mut a2 = 0.0;
        let mut a3 = 0.0;
        let mut a4 = 0.0;
        let mut a5 = 0.0;

        if vc == 1 {
            a0 = 0.0;
            a1 = 0.0;
            a2 = 0.0;
            a3 = 0.0;
            a4 = 0.0;
            a5 = 0.0;
        } else if vc == 2 {
            a0 = -0.45036;
            a1 = 0.00814;
            a2 = 0.01543;
            a3 = 0.01358;
            a4 = 0.0;
            a5 = 0.0;
        } else if vc == 3 {
            a0 = -0.29591;
            a1 = 0.00743;
            a2 = 0.0;
            a3 = 0.01246;
            a4 = 0.0;
            a5 = 0.0;
        } else if vc == 4 {
            a0 = -0.40902;
            a1 = 0.00975;
            a2 = 0.00767;
            a3 = -0.18363;
            a4 = 0.00423;
            a5 = 0.0;
        } else if vc == 5 {
            a0 = -0.3836;
            a1 = 0.01074;
            a2 = 0.01945;
            a3 = -0.69848;
            a4 = 0.01069;
            a5 = 0.127;
        }

        let a = f64::max(
            0.0333,
            a0 + a1 * bffs
                + a2 * seg_length
                + (f64::max(0.0, a3 + a4 * bffs + a5 * seg_length) * vo) / 1000.0,
        );

        // adjustment for lane and shoulder width, mi/hr
        let f_ls = 0.6 * (12.0 - lw) + 0.7 * (6.0 - sw);
        // adjustment for access point density, mi/hr
        let f_a = f64::min(apd / 4.0, 10.0);

        ffs = bffs - a * phv - f_ls - f_a;
        self.segments[seg_num].set_ffs(ffs);

        ffs
    }

    /// Step 5: Estimate average speed (Equations 15-7 to 15-16).
    ///
    /// Calculates the average speed at the segment endpoint, accounting for
    /// traffic flow effects and horizontal curvature adjustments.
    ///
    /// # Core Equation (Eq 15-7)
    ///
    /// ```text
    /// S = FFS                           if v_d ≤ 100 veh/h
    /// S = FFS - m × (v_d/1000 - 0.1)^p  if v_d > 100 veh/h
    /// ```
    ///
    /// where:
    /// - m = slope coefficient (from Eq 15-8, Exhibits 15-13/15-14)
    /// - p = power coefficient (from Eq 15-11, Exhibits 15-19/15-20)
    ///
    /// # Horizontal Curvature Adjustment (Step 5d)
    ///
    /// If the segment contains horizontal curves:
    /// 1. Each curve is classified (Exhibit 15-22) based on radius and superelevation
    /// 2. Speeds on curves are reduced using Equations 15-12 to 15-15
    /// 3. Segment speed is length-weighted average (Equation 15-16)
    ///
    /// # Arguments
    ///
    /// * `seg_num` - Index of the segment to analyze
    ///
    /// # Returns
    ///
    /// Tuple of (average_speed, horizontal_class)
    /// - average_speed in mi/h
    /// - horizontal_class (0-5, or 0 if no curves)
    pub fn estimate_average_speed(&mut self, seg_num: usize) -> (f64, i32) {
        let spl = self.segments[seg_num].get_spl();
        let bffs = math::round_to_significant_digits(1.14 * spl, 3);

        // Get variables from segments
        let mut s: f64; // average speed
        let mut tot_s: f64 = 0.0; // total speed
        let res_s: f64; // Results speed
        let mut hor_class: i32;
        let seg_s: f64;
        let seg_hor_class: i32;
        let ffs = self.segments[seg_num].get_ffs();
        let pt = self.segments[seg_num].get_passing_type();
        let phf = self.segments[seg_num].get_phf();
        let phv = self.segments[seg_num].get_phv();
        let vc = self.segments[seg_num].get_vertical_class();
        let vd = self.segments[seg_num].get_flow_rate();
        let vo = self.segments[seg_num].get_flow_rate_o();
        let is_hc = self.segments[seg_num].get_is_hc();

        // Determine Segment Avg Speed
        let seg_length = self.segments[seg_num].get_length();
        // Only affected when it contains subsegments
        let rad = 0.0;
        let sup_ele = 0.0;
        (seg_s, seg_hor_class) = self.calc_speed(
            seg_length, bffs, ffs, pt, vc, vd, vo, phv, phf, false, rad, sup_ele,
        );

        if is_hc {
            // Get variables from subsegments
            let subseg_num = self.segments[seg_num].get_subsegments().len();
            // let mut subseg_length: Vec<f64>; // = (0../collect();
            // let mut sup_ele: Vec<f64>; // = (0..seg_num).collect();
            let mut i = 0;
            while i < subseg_num {
                let subseg_length =
                    self.segments[seg_num].get_subsegments()[i].get_length() / 5280.0;
                let rad = self.segments[seg_num].get_subsegments()[i].get_design_rad();
                let sup_ele = self.segments[seg_num].get_subsegments()[i].get_sup_ele();
                if rad > 0.0 {
                    (s, hor_class) = self.calc_speed(
                        seg_length, bffs, ffs, pt, vc, vd, vo, phv, phf, is_hc, rad, sup_ele,
                    );
                    tot_s += s * subseg_length;

                    // self.segments[seg_num].get_subsegments()[i].set_avg_speed(s);
                    // self.segments[seg_num].get_subsegments()[i].set_hor_class(hor_class);
                    self.segments[seg_num].set_subsegments_avg_speed(i, s);
                    self.segments[seg_num].set_subsegments_hor_class(i, hor_class);
                } else {
                    // Tangent Section
                    // self.segments[seg_num].get_subsegments()[i].set_avg_speed(seg_s);
                    // self.segments[seg_num].get_subsegments()[i].set_hor_class(seg_hor_class);
                    self.segments[seg_num].set_subsegments_avg_speed(i, seg_s);
                    self.segments[seg_num].set_subsegments_hor_class(i, seg_hor_class);
                    tot_s += math::round_up_to_n_decimal(seg_s, 1) * subseg_length;

                    // println!("Sub Segments: {i}, Speed: {seg_s}: Length: {subseg_length}");
                }
                i += 1;
            }
            res_s = tot_s / (seg_length) as f64;
        } else {
            res_s = seg_s;
        }

        self.segments[seg_num].set_avg_speed(res_s);
        // self.segments[seg_num].seg_hor_class(hor_class);

        (res_s, seg_hor_class)
    }

    fn calc_speed(
        &self,
        seg_length: f64,
        bffs: f64,
        mut ffs: f64,
        pt: usize,
        vc: i32,
        vd: f64,
        vo: f64,
        phv: f64,
        phf: f64,
        is_hc: bool,
        rad: f64,
        sup_ele: f64,
    ) -> (f64, i32) {
        // Parameter initialization
        let (mut b0, mut b1, mut b2, mut b3, mut b4, mut b5) =
            (0.0000, 0.0000, 0.0000, 0.0000, 0.0000, 0.0000);
        let (mut c0, mut c1, mut c2, mut c3) = (0.0000, 0.0000, 0.0000, 0.0000);
        let (mut d0, mut d1, mut d2, mut d3) = (0.0000, 0.0000, 0.0000, 0.0000);
        let (mut f0, mut f1, mut f2, mut f3, mut f4, mut f5, mut f6, mut f7, mut f8) = (
            0.0000, 0.0000, 0.0000, 0.0000, 0.0000, 0.0000, 0.0000, 0.0000, 0.0000,
        );

        ffs = math::round_up_to_n_decimal(ffs, 1);

        let mut s: f64;
        let mut hor_class: i32 = 0;

        if pt == 0 || pt == 1 {
            if vc == 1 {
                b0 = 0.0558;
                b1 = 0.0542;
                b2 = 0.3278;
                b3 = 0.1029;
                f0 = 0.67576;
                f3 = 0.12060;
                f4 = -0.35919;
            } else if vc == 2 {
                b0 = 5.7280;
                b1 = -0.0809;
                b2 = 0.7404;
                b5 = 3.1155;
                c0 = -13.8036;
                c2 = 0.2446;
                d0 = -1.7765;
                d2 = 0.0392;
                b3 = c0 + c1 * f64::sqrt(seg_length) + c2 * ffs + c3 * ffs * f64::sqrt(seg_length);
                b4 = d0 + d1 * f64::sqrt(phv) + d2 * ffs + d3 * ffs * f64::sqrt(phf);
                f0 = 0.34524;
                f1 = 0.00591;
                f2 = 0.02031;
                f3 = 0.14911;
                f4 = -0.43784;
                f5 = -0.00296;
                f6 = 0.02956;
                f8 = 0.41622;
            } else if vc == 3 {
                b0 = 9.3079;
                b1 = -0.1706;
                b2 = 1.1292;
                b5 = 3.1155;
                c0 = -11.9703;
                c2 = 0.2542;
                d0 = -3.5550;
                d2 = 0.0826;
                b3 = c0 + c1 * f64::sqrt(seg_length) + c2 * ffs + c3 * ffs * f64::sqrt(seg_length);
                b4 = d0 + d1 * f64::sqrt(phv) + d2 * ffs + d3 * ffs * f64::sqrt(phv);
                f0 = 0.17291;
                f1 = 0.00917;
                f2 = 0.05698;
                f3 = 0.27734;
                f4 = -0.61893;
                f5 = -0.00918;
                f6 = 0.09184;
                f8 = 0.41622;
            } else if vc == 4 {
                b0 = 9.0115;
                b1 = -0.1994;
                b2 = 1.8252;
                b5 = 3.2685;
                c0 = -12.5113;
                c2 = 0.2656;
                d0 = -5.7775;
                d2 = 0.1373;
                b3 = math::round_up_to_n_decimal(
                    c0 + c1 * f64::sqrt(seg_length) + c2 * ffs + c3 * ffs * f64::sqrt(seg_length),
                    4,
                );
                b4 = math::round_up_to_n_decimal(
                    d0 + d1 * f64::sqrt(phv) + d2 * ffs + d3 * ffs * f64::sqrt(phv),
                    4,
                );
                f0 = 0.67689;
                f1 = 0.00534;
                f2 = -0.13037;
                f3 = 0.25699;
                f4 = -0.68465;
                f5 = -0.00709;
                f6 = 0.07087;
                f8 = 0.33950;
            } else if vc == 5 {
                b0 = 23.9144;
                b1 = -0.6925;
                b2 = 1.9473;
                b5 = 3.5115;
                c0 = -14.8961;
                c2 = 0.4370;
                d0 = -18.2910;
                d1 = 2.3875;
                d2 = 0.4494;
                d3 = -0.0520;
                b3 = c0 + c1 * f64::sqrt(seg_length) + c2 * ffs + c3 * ffs * f64::sqrt(seg_length);
                b4 = d0 + d1 * f64::sqrt(phv) + d2 * ffs + d3 * ffs * f64::sqrt(phv);
                f0 = 1.13262;
                f2 = -0.26367;
                f3 = 0.18811;
                f4 = -0.64304;
                f5 = -0.00867;
                f6 = 0.08675;
                f8 = 0.30590;
            }
        } else if pt == 2 {
            if vc == 1 {
                b0 = -1.1379;
                b1 = 0.0941;
                c1 = 0.2667;
                d1 = 0.1252;
                b3 = c0 + c1 * f64::sqrt(seg_length) + c2 * ffs + c3 * ffs * f64::sqrt(seg_length);
                b4 = d0 + d1 * f64::sqrt(phv) + d2 * ffs + d3 * ffs * f64::sqrt(phv);
                f0 = 0.91793;
                f1 = -0.00557;
                f2 = 0.36862;
                f5 = 0.00611;
                f7 = -0.00419;
            } else if vc == 2 {
                b0 = -2.0668;
                b1 = 0.1053;
                c1 = 0.4479;
                d1 = 0.1631;
                b3 = c0 + c1 * f64::sqrt(seg_length) + c2 * ffs + c3 * ffs * f64::sqrt(seg_length);
                b4 = d0 + d1 * f64::sqrt(phv) + d2 * ffs + d3 * ffs * f64::sqrt(phv);
                f0 = 0.65105;
                f2 = 0.34931;
                f5 = 0.00722;
                f7 = -0.00391;
            } else if vc == 3 {
                b0 = -0.5074;
                b1 = 0.0935;
                d1 = -0.2201;
                d3 = 0.0072;
                b4 = d0 + d1 * f64::sqrt(phv) + d2 * ffs + d3 * ffs * f64::sqrt(phv);
                f0 = 0.40117;
                f2 = 0.68633;
                f5 = 0.02350;
                f7 = -0.02088;
            } else if vc == 4 {
                b0 = 8.0354;
                b1 = -0.0860;
                b5 = 4.1900;
                c0 = -27.1244;
                c1 = 11.5196;
                c2 = 0.4681;
                c3 = -0.1873;
                d1 = -0.7506;
                d3 = 0.0193;
                b3 = c0 + c1 * f64::sqrt(seg_length) + c2 * ffs + c3 * ffs * f64::sqrt(seg_length);
                b4 = d0 + d1 * f64::sqrt(phv) + d2 * ffs + d3 * ffs * f64::sqrt(phv);
                f0 = 1.13282;
                f1 = -0.00798;
                f2 = 0.35425;
                f5 = 0.01521;
                f7 = -0.00987;
            } else if vc == 5 {
                b0 = 7.2991;
                b1 = -0.3535;
                b5 = 4.8700;
                c0 = -45.3391;
                c1 = 17.3749;
                c2 = 1.0587;
                c3 = -0.3729;
                d0 = 3.8457;
                d1 = -0.9112;
                d3 = 0.0170;
                b3 = c0 + c1 * f64::sqrt(seg_length) + c2 * ffs + c3 * ffs * f64::sqrt(seg_length);
                b4 = d0 + d1 * f64::sqrt(phv) + d2 * ffs + d3 * ffs * f64::sqrt(phv);
                f0 = 1.12077;
                f1 = -0.00550;
                f2 = 0.25431;
                f5 = 0.01269;
                f7 = -0.01053;
            }
        }
        b3 = math::round_up_to_n_decimal(b3, 3);
        b4 = math::round_up_to_n_decimal(b4, 3);
        // slope coefficient for average speed calculation
        let mut ms = f64::max(
            b5,
            b0 + b1 * ffs
                + b2 * f64::sqrt(vo / 1000.0)
                + f64::max(0.0, b3) * f64::sqrt(seg_length)
                + f64::max(0.0, b4) * f64::sqrt(phv),
        );

        // power coefficient for average speed calculation
        let mut ps = f64::max(
            f8,
            f0 + f1 * ffs
                + f2 * seg_length
                + (f3 * vo) / 1000.0
                + f4 * f64::sqrt(vo / 1000.0)
                + f5 * phv
                + f6 * f64::sqrt(phv)
                + f7 * seg_length * phv,
        );

        ms = math::round_up_to_n_decimal(ms, 3);
        ps = math::round_up_to_n_decimal(ps, 3);

        // Length of horizontal curves = radius x central angle x pi/180
        // determine horizontal class
        if rad == 0.0 {
            hor_class = 0;
        } else if rad > 0.0 && rad < 300.0 {
            hor_class = 5;
        } else if rad >= 300.0 && rad < 450.0 {
            hor_class = 4;
        } else if rad >= 450.0 && rad < 600.0 {
            if sup_ele < 1.0 {
                hor_class = 4
            } else {
                hor_class = 3
            };
        } else if rad >= 600.0 && rad < 750.0 {
            if sup_ele < 6.0 {
                hor_class = 3
            } else {
                hor_class = 2
            };
        } else if rad >= 750.0 && rad < 900.0 {
            hor_class = 2;
        } else if rad >= 900.0 && rad < 1050.0 {
            if sup_ele < 8.0 {
                hor_class = 2
            } else {
                hor_class = 1
            };
        } else if rad >= 1050.0 && rad < 1200.0 {
            if sup_ele < 4.0 {
                hor_class = 2
            } else {
                hor_class = 1
            };
        } else if rad >= 1200.0 && rad < 1350.0 {
            if sup_ele < 2.0 {
                hor_class = 2
            } else {
                hor_class = 1
            };
        } else if rad >= 1350.0 && rad < 1500.0 {
            hor_class = 1;
        } else if rad >= 1500.0 && rad < 1750.0 {
            if sup_ele < 8.0 {
                hor_class = 1
            } else {
                hor_class = 0
            };
        } else if rad >= 1750.0 && rad < 1800.0 {
            if sup_ele < 6.0 {
                hor_class = 1
            } else {
                hor_class = 0
            };
        } else if rad >= 1800.0 && rad < 1950.0 {
            if sup_ele < 5.0 {
                hor_class = 1
            } else {
                hor_class = 0
            };
        } else if rad >= 1950.0 && rad < 2100.0 {
            if sup_ele < 4.0 {
                hor_class = 1
            } else {
                hor_class = 0
            };
        } else if rad >= 2100.0 && rad < 2250.0 {
            if sup_ele < 3.0 {
                hor_class = 1
            } else {
                hor_class = 0
            };
        } else if rad >= 2250.0 && rad < 2400.0 {
            if sup_ele < 2.0 {
                hor_class = 1
            } else {
                hor_class = 0
            };
        } else if rad >= 2400.0 && rad < 2550.0 {
            if sup_ele < 1.0 {
                hor_class = 1
            } else {
                hor_class = 0
            };
        } else if rad >= 2550.0 {
            hor_class = 0;
        }

        if vd <= 100.0 {
            let st = ffs;
            s = st;
        } else {
            let st = ffs - ms * f64::powf(vd / 1000.0 - 0.1, ps);
            s = st;
        }

        if is_hc {
            // calculate horizontal class
            let bffshc = f64::min(bffs, 44.32 + 0.3728 * bffs - 6.868 * hor_class as f64);
            let ffshc = bffshc - 0.0255 * phv;
            let mhc = math::round_to_significant_digits(
                f64::max(
                    0.277,
                    -25.8993 - 0.7756 * ffshc
                        + 10.6294 * f64::sqrt(ffshc)
                        + 2.4766 * hor_class as f64
                        - 9.8238 * f64::sqrt(hor_class as f64),
                ),
                5,
            );
            // println!("s: {s}");
            let shc = math::round_to_significant_digits(
                f64::min(s, ffshc - mhc * f64::sqrt(vd / 1000.0 - 0.1)),
                3,
            ); // Should be ST instead of S?
               // println!("BFFS: {bffshc}, FFSHC: {ffshc}, MHC: {mhc}, SHC: {shc}");
            s = shc;
        }
        (s, hor_class)
    }

    fn calc_percent_followers(
        &self,
        seg_length: f64,
        mut ffs: f64,
        cap: i32,
        pt: usize,
        vc: i32,
        vd: f64,
        vo: f64,
        phv: f64,
    ) -> f64 {
        let (mut b0, mut b1, mut b2, mut b3, mut b4, mut b5, mut b6, mut b7) = (
            0.000000, 0.000000, 0.000000, 0.000000, 0.000000, 0.000000, 0.000000, 0.000000,
        );
        let (mut c0, mut c1, mut c2, mut c3, mut c4, mut c5, mut c6, mut c7) = (
            0.000000, 0.000000, 0.000000, 0.000000, 0.000000, 0.000000, 0.000000, 0.000000,
        );
        let (mut d1, mut d2) = (0.000000, 0.000000);
        let (mut e0, mut e1, mut e2, mut e3, mut e4) =
            (0.000000, 0.000000, 0.000000, 0.000000, 0.000000);

        ffs = math::round_up_to_n_decimal(ffs, 2);

        // Percent followers at capacity
        let mut pf_cap = 0.0;
        let mut pf_25_cap = 0.0;

        if pt == 0 || pt == 1 {
            if vc == 1 {
                b0 = 37.68080;
                b1 = 3.05089;
                b2 = -7.90866;
                b3 = -0.94321;
                b4 = 13.64266;
                b5 = -0.00050;
                b6 = -0.05500;
                b7 = 7.13760;
                c0 = 18.01780;
                c1 = 10.00000;
                c2 = -21.60000;
                c3 = -0.97853;
                c4 = 12.05214;
                c5 = -0.00750;
                c6 = -0.06700;
                c7 = 11.60410;
            } else if vc == 2 {
                b0 = 58.21104;
                b1 = 5.73387;
                b2 = -13.66293;
                b3 = -0.66126;
                b4 = 9.08575;
                b5 = -0.00950;
                b6 = -0.03602;
                b7 = 7.14619;
                c0 = 47.83887;
                c1 = 12.80000;
                c2 = -28.20000;
                c3 = -0.61758;
                c4 = 5.80000;
                c5 = -0.04550;
                c6 = -0.03344;
                c7 = 11.35573;
            } else if vc == 3 {
                b0 = 113.20439;
                b1 = 10.01778;
                b2 = -18.90000;
                b3 = 0.46542;
                b4 = -6.75338;
                b5 = -0.03000;
                b6 = -0.05800;
                b7 = 10.03239;
                c0 = 125.40000;
                c1 = 19.50000;
                c2 = -34.90000;
                c3 = 0.90672;
                c4 = -16.10000;
                c5 = -0.11000;
                c6 = -0.06200;
                c7 = 14.71136;
            } else if vc == 4 {
                b0 = 58.29978;
                b1 = -0.53611;
                b2 = 7.35076;
                b3 = -0.27046;
                b4 = 4.49850;
                b5 = -0.01100;
                b6 = -0.02968;
                b7 = 8.89680;
                c0 = 103.13534;
                c1 = 14.68459;
                c2 = -23.72704;
                c3 = 0.66444;
                c4 = -11.95763;
                c5 = -0.10000;
                c6 = 0.00172;
                c7 = 14.70067;
            } else if vc == 5 {
                b0 = 3.32968;
                b1 = -0.84377;
                b2 = 7.08952;
                b3 = -1.32089;
                b4 = 19.98477;
                b5 = -0.01250;
                b6 = -0.02960;
                b7 = 9.99453;
                c0 = 89.00000;
                c1 = 19.02642;
                c2 = -34.54240;
                c3 = 0.29792;
                c4 = -6.62528;
                c5 = -0.16000;
                c6 = 0.00480;
                c7 = 17.56611;
            }
            d1 = -0.29764;
            d2 = -0.71917;
            e0 = 0.81165;
            e1 = 0.37920;
            e2 = -0.49524;
            e3 = -2.11289;
            e4 = 2.41146;

            pf_cap = b0
                + b1 * seg_length
                + b2 * f64::sqrt(seg_length)
                + b3 * ffs
                + b4 * f64::sqrt(ffs)
                + b5 * phv
                + b6 * ffs * vo / 1000.0
                + b7 * f64::sqrt(vo / 1000.0);
            pf_25_cap = c0
                + c1 * seg_length
                + c2 * f64::sqrt(seg_length)
                + c3 * ffs
                + c4 * f64::sqrt(ffs)
                + c5 * phv
                + c6 * ffs * vo / 1000.0
                + c7 * f64::sqrt(vo / 1000.0);
        } else if pt == 2 {
            if vc == 1 {
                b0 = 61.73075;
                b1 = 6.73922;
                b2 = -23.68853;
                b3 = -0.84126;
                b4 = 11.44533;
                b5 = -1.05124;
                b6 = 1.50390;
                b7 = 0.00491;
                c0 = 80.37105;
                c1 = 14.44997;
                c2 = -46.41831;
                c3 = -0.23367;
                c4 = 0.84914;
                c5 = -0.56747;
                c6 = 0.89427;
                c7 = 0.00119;
            } else if vc == 2 {
                b0 = 12.30096;
                b1 = 9.57465;
                b2 = -30.79427;
                b3 = -1.79448;
                b4 = 25.76436;
                b5 = -0.66350;
                b6 = 1.26039;
                b7 = -0.00323;
                c0 = 18.37886;
                c1 = 14.71856;
                c2 = -47.78892;
                c3 = -1.43373;
                c4 = 18.32040;
                c5 = -0.13226;
                c6 = 0.77127;
                c7 = -0.00778;
            } else if vc == 3 {
                b0 = 206.07369;
                b1 = -4.29885;
                b2 = 0.00000;
                b3 = 1.96483;
                b4 = -30.32556;
                b5 = -0.75812;
                b6 = 1.06453;
                b7 = -0.00839;
                c0 = 239.98930;
                c1 = 15.90683;
                c2 = -46.87525;
                c3 = 2.73582;
                c4 = -42.88130;
                c5 = -0.53746;
                c6 = -0.76271;
                c7 = -0.00428;
            } else if vc == 4 {
                b0 = 263.13428;
                b1 = 5.38749;
                b2 = -19.04859;
                b3 = 2.73018;
                b4 = -42.76919;
                b5 = -1.31277;
                b6 = -0.32242;
                b7 = 0.01412;
                c0 = 223.68435;
                c1 = 10.26908;
                c2 = -35.60830;
                c3 = 2.31877;
                c4 = -38.30034;
                c5 = -0.60275;
                c6 = -0.67758;
                c7 = 0.00117;
            } else if vc == 5 {
                b0 = 126.95629;
                b1 = 5.95754;
                b2 = -19.22229;
                b3 = 0.43238;
                b4 = -7.35636;
                b5 = -1.03017;
                b6 = -2.66026;
                b7 = 0.01389;
                c0 = 137.37633;
                c1 = 11.00106;
                c2 = -38.89043;
                c3 = 0.78501;
                c4 = -14.88672;
                c5 = -0.72576;
                c6 = -2.49546;
                c7 = 0.00872;
            }
            d1 = -0.15808;
            d2 = -0.83732;
            e0 = -1.63246;
            e1 = 1.64960;
            e2 = -4.45823;
            e3 = -4.89119;
            e4 = 10.33057;

            pf_cap = b0
                + b1 * seg_length
                + b2 * f64::sqrt(seg_length)
                + b3 * ffs
                + b4 * f64::sqrt(ffs)
                + b5 * phv
                + b6 * f64::sqrt(phv)
                + b7 * ffs * phv;
            pf_25_cap = c0
                + c1 * seg_length
                + c2 * f64::sqrt(seg_length)
                + c3 * ffs
                + c4 * f64::sqrt(ffs)
                + c5 * phv
                + c6 * f64::sqrt(phv)
                + c7 * ffs * phv;
        }

        let z_cap = (0.0 - f64::ln(1.0 - pf_cap / 100.0)) / (cap as f64 / 1000.0);
        let z_25_cap = (0.0 - f64::ln(1.0 - pf_25_cap / 100.0)) / ((0.25 * cap as f64) / 1000.0);

        // Slope Coefficient
        let m_pf = d1 * z_25_cap + d2 * z_cap;
        // Power Coefficient
        let p_pf =
            e0 + e1 * z_25_cap + e2 * z_cap + e3 * f64::sqrt(z_25_cap) + e4 * f64::sqrt(z_cap);

        let pf = 100.0 * (1.0 - f64::exp(m_pf * f64::powf(vd / 1000.0, p_pf)));

        pf
    }

    /// Step 6: Estimate percent followers (Equations 15-17 to 15-23).
    ///
    /// Calculates the percentage of vehicles in a "follower" state (headway ≤2.5 seconds).
    /// Percent followers represents freedom to maneuver and serves as a proxy for
    /// the need to provide passing opportunities.
    ///
    /// # Core Equation (Eq 15-17)
    ///
    /// ```text
    /// PF = 100 × (1 - e^(m × (v_d/1000)^p))
    /// ```
    ///
    /// The exponential curve is fitted through two points:
    /// - PF at 25% of capacity (Eq 15-20/15-21)
    /// - PF at capacity (Eq 15-18/15-19)
    ///
    /// # Arguments
    ///
    /// * `seg_num` - Index of the segment to analyze
    ///
    /// # Returns
    ///
    /// Percent followers (0-100%)
    ///
    /// # Notes
    ///
    /// - Even at capacity, PF won't reach 100% due to gaps between platoons
    /// - PF is not adjusted for horizontal curvature (minimal impact compared to speed)
    /// - Different coefficient tables for PC/PZ vs PL segments (Exhibits 15-24 to 15-29)
    pub fn estimate_percent_followers(&mut self, seg_num: usize) -> f64 {
        let seg_length = self.segments[seg_num].get_length();
        let ffs = self.segments[seg_num].get_ffs();
        let cap = self.segments[seg_num].get_capacity();
        let pt = self.segments[seg_num].get_passing_type();
        let vc = self.segments[seg_num].get_vertical_class();
        let vd = self.segments[seg_num].get_flow_rate();
        let vo = self.segments[seg_num].get_flow_rate_o();
        let phv = self.segments[seg_num].get_phv();

        let pf = self.calc_percent_followers(seg_length, ffs, cap, pt, vc, vd, vo, phv);

        self.segments[seg_num].set_percent_followers(pf);

        pf
    }

    pub fn estimate_average_speed_sf(
        &mut self,
        seg_num: usize,
        length: f64,
        vd: f64,
        phv: f64,
        rad: f64,
        sup_ele: f64,
    ) -> (f64, i32) {
        let spl = self.segments[seg_num].get_spl();
        let bffs = 1.14 * spl;

        // Get variables from segments
        let s: f64; // average speed
        let hor_class: i32;
        let ffs = self.segments[seg_num].get_ffs();
        let _pt = self.segments[seg_num].get_passing_type();
        let phf = self.segments[seg_num].get_phf();
        let vc = self.segments[seg_num].get_vertical_class();
        let vo = self.segments[seg_num].get_flow_rate_o();
        let is_hc = self.segments[seg_num].get_is_hc();

        (s, hor_class) = self.calc_speed(
            length, bffs, ffs, 2, vc, vd, vo, phv, phf, is_hc, rad, sup_ele,
        );

        (s, hor_class)
    }

    pub fn estimate_percent_followers_sf(&self, seg_num: usize, vd: f64, phv: f64) -> f64 {
        let seg_length = self.segments[seg_num].get_length();
        let ffs = self.segments[seg_num].get_ffs();
        let cap = self.segments[seg_num].get_capacity();
        let pt = self.segments[seg_num].get_passing_type();
        let vc = self.segments[seg_num].get_vertical_class();
        let vo = self.segments[seg_num].get_flow_rate_o();

        let pf = self.calc_percent_followers(seg_length, ffs, cap, pt, vc, vd, vo, phv);

        pf
    }

    /// Steps 7-8: Calculate passing lane performance and follower density (Equations 15-24 to 15-35).
    ///
    /// For Passing Lane segments, this method:
    /// 1. Distributes flow between faster lane (FL) and slower lane (SL)
    /// 2. Calculates separate speeds and percent followers for each lane
    /// 3. Computes follower density at the passing lane midpoint (for LOS)
    /// 4. Also computes follower density at segment endpoint
    ///
    /// # Flow Distribution (Equations 15-24 to 15-27)
    ///
    /// ```text
    /// NumHV = v_d × HV%                                              (Eq 15-24)
    /// PropFlowFL = 0.92183 - 0.05022×ln(v_d) - 0.00030×NumHV        (Eq 15-25)
    /// FlowFL = v_d × PropFlowFL                                      (Eq 15-26)
    /// FlowSL = v_d - FlowFL                                          (Eq 15-27)
    /// ```
    ///
    /// # Midpoint Follower Density (Equation 15-34)
    ///
    /// ```text
    /// FD_PLmid = (PF_FL×v_FL/S_FL + PF_SL×v_SL/S_SL) / 200
    /// ```
    ///
    /// # Arguments
    ///
    /// * `seg_num` - Index of the Passing Lane segment to analyze
    ///
    /// # Returns
    ///
    /// Tuple of (endpoint_follower_density, midpoint_follower_density)
    ///
    /// # Notes
    ///
    /// - FD_PLmid is used for LOS determination on passing lane segments
    /// - Speed differential adjustment accounts for faster vehicles passing slower ones
    pub fn determine_follower_density_pl(&mut self, seg_num: usize) -> (f64, f64) {
        let mut s_init_fl: f64;
        let mut s_init_sl: f64;
        let pf_fl: f64;
        let pf_sl: f64;

        let seg_length = self.segments[seg_num].get_length();
        let subseg_num = self.segments[seg_num].get_subsegments().len();
        let vd = self.segments[seg_num].get_flow_rate();
        let phv = self.segments[seg_num].get_phv();
        let pm_hv_fl = self.pmhvfl.unwrap_or(0.0);

        // Calculate passing lane parameters
        let nhv = f64::round(vd * phv / 100.0);
        let p_v_fl = 0.92183 - 0.05022 * f64::ln(vd) - 0.00030 * nhv;
        let vd_fl = f64::round(vd * p_v_fl);
        let vd_sl = f64::round(vd * (1.0 - p_v_fl));
        let phv_fl = phv * pm_hv_fl;
        let nhv_sl = f64::ceil(nhv - (vd_fl * phv_fl / 100.0));
        let phv_sl = math::round_up_to_n_decimal(nhv_sl / vd_sl * 100.0, 1);
        let mut fl_tot: f64 = 0.0;
        let mut sl_tot: f64 = 0.0;

        // Subsection
        let mut j = 0;
        // One subseg list is set to be initialized with 0 inputs
        if subseg_num > 1 {
            while j < subseg_num {
                let sub_seg_len = self.segments[seg_num].get_subsegments()[j].get_length() / 5280.0;
                let rad = self.segments[seg_num].get_subsegments()[j].get_design_rad();
                let sup_ele = self.segments[seg_num].get_subsegments()[j].get_sup_ele();
                (s_init_fl, _) = self.estimate_average_speed_sf(
                    seg_num,
                    sub_seg_len,
                    vd_fl,
                    phv_fl,
                    rad,
                    sup_ele,
                );
                (s_init_sl, _) = self.estimate_average_speed_sf(
                    seg_num,
                    sub_seg_len,
                    vd_sl,
                    phv_sl,
                    rad,
                    sup_ele,
                );

                fl_tot += s_init_fl * sub_seg_len;
                sl_tot += s_init_sl * sub_seg_len;
                j += 1;
            }
            s_init_fl = fl_tot / seg_length;
            s_init_sl = sl_tot / seg_length;
        } else {
            let rad = 0.0;
            let sup_ele = 0.0;
            (s_init_fl, _) =
                self.estimate_average_speed_sf(seg_num, seg_length, vd_fl, phv_fl, rad, sup_ele);
            (s_init_sl, _) =
                self.estimate_average_speed_sf(seg_num, seg_length, vd_sl, phv_sl, rad, sup_ele);
        }

        pf_fl = self.estimate_percent_followers_sf(seg_num, vd_fl, phv_fl);
        pf_sl = self.estimate_percent_followers_sf(seg_num, vd_sl, phv_sl);

        let sda = 2.750 + 0.00056 * vd + 3.8521 * phv / 100.0;
        let s_mid_fl = s_init_fl + sda / 2.0;
        let s_mid_sl = s_init_sl - sda / 2.0;
        // println!("{}, {}, {}, {}", s_init_fl, s_init_sl, s_mid_fl, s_mid_sl);

        // it's acutually fd at the midpoint of the PL segment but used for LOS calculation
        let fd_mid = (pf_fl * vd_fl / s_mid_fl + pf_sl * vd_sl / s_mid_sl) / 200.0;

        self.segments[seg_num].set_followers_density_mid(fd_mid);

        let fd = self.determine_follower_density_pc_pz(seg_num);
        self.segments[seg_num].set_followers_density(fd);

        (fd, fd_mid)
    }

    /// Calculate follower density for Passing Constrained and Passing Zone segments (Equation 15-35).
    ///
    /// Follower density is the service measure used to determine LOS on two-lane highways.
    /// It combines the effects of both platooning (percent followers) and traffic density.
    ///
    /// # Equation 15-35
    ///
    /// ```text
    /// FD = (PF × v_d) / (100 × S)
    /// ```
    ///
    /// where:
    /// - FD = follower density (followers/mi/ln)
    /// - PF = percent followers (%)
    /// - v_d = demand flow rate (veh/h)
    /// - S = average speed (mi/h)
    ///
    /// # Arguments
    ///
    /// * `seg_num` - Index of the segment to analyze
    ///
    /// # Returns
    ///
    /// Follower density in followers/mi/ln
    pub fn determine_follower_density_pc_pz(&mut self, seg_num: usize) -> f64 {
        let s = self.segments[seg_num].get_avg_speed();
        let pf = self.segments[seg_num].get_percent_followers();
        let vd = self.segments[seg_num].get_flow_rate();
        let fd = (pf * vd) / (100.0 * s);

        self.segments[seg_num].set_followers_density(fd);
        fd
    }

    pub fn determine_adjustment_to_follower_density(&mut self, seg_num: usize) -> f64 {
        let seg_len = self.segments.len();
        let mut is_pl_list: Vec<bool> = Vec::new();
        let s = self.segments[seg_num].get_avg_speed();
        let mut pl_loc = 100;
        let pass_type = self.segments[seg_num].get_passing_type();

        for s_num in 0..seg_len {
            let p_type = self.segments[s_num].get_passing_type();
            if p_type == 2 {
                is_pl_list.push(true);
                pl_loc = s_num; // TODO: if there are more than three PL section
            } else {
                is_pl_list.push(false);
            }
        }

        // Accumulate segments length from PL on upstream
        let mut l_d: f64 = 0.0;
        if pl_loc <= seg_num {
            for s_num in pl_loc..seg_num + 1 {
                l_d += self.segments[s_num].get_length();
            }
        }

        // Calculate downstream distance from start of passing lane
        let mut fd_adj: f64 = 0.0;
        let pf = self.segments[seg_num].get_percent_followers();

        if seg_num > 0 && is_pl_list.iter().filter(|&&x| x).count() > 0 {
            // let pf_u = self.segments[seg_num-1].get_percent_followers();
            let pf_u = self.segments[pl_loc - 1].get_percent_followers();
            let vd = self.segments[seg_num].get_flow_rate();
            let vd_u = self.segments[seg_num - 1].get_flow_rate();
            let _fd_u = self.segments[seg_num - 1].get_followers_density();
            let l_de: f64; // effective distance

            let x_2 = 0.1 * f64::max(0.0, pf_u - 30.0);
            let x_3a = 3.5 * f64::ln(f64::max(0.3, self.segments[pl_loc].get_length()));
            let x_3b = 0.75 * self.segments[pl_loc].get_length();

            // Determine effective distance of PL
            if pass_type == 2 {
                let x_4a = 0.01 * vd_u;
                let x_4b = 0.005 * vd_u;
                let y_1a = 27.0 + x_2 + x_3a - x_4a;
                let y_2a = 3.0 + x_2 + x_3b - x_4b;
                let _y_3 =
                    (95.0 * self.segments[seg_num - 1].get_followers_density() * s) / (pf_u * vd_u);

                // Solve for downstream effective length of passing lane from start of PL (LDE)
                // The percentage improvement to the percent followers becomes zero
                let l_de_1 = f64::exp(y_1a / 8.75);

                // Follower density is at least 95% of the level entering the passing lane
                let l_de_2 = f64::max(
                    0.1,
                    f64::exp(-1.0 * (f64::max(0.0, -1.0 * y_1a + 32.0) - 27.0) / 8.75),
                );

                l_de = math::round_up_to_n_decimal(f64::min(l_de_1, l_de_2), 1);
                self.l_de = Some(l_de);

                let pf_improve = f64::max(0.0, y_1a - 8.75 * f64::ln(f64::max(0.1, l_de)));
                let s_improve = f64::max(0.0, y_2a - 0.8 * l_de);
                let _y_3 = (100.0 - pf_improve) / (100.0 + s_improve);

                fd_adj = (pf_u / 100.0) * (1.0 - pf_improve / 100.0) * vd_u
                    / (s * (1.0 + s_improve / 100.0));
                // fd_adj = (pf_u / 100.0) * (1.0 - pf_improve / 100.0) * vd_u / (58.8 * (1.0 + s_improve / 100.0));
            } else {
                // Determine adjustment to follower density
                // if segment is within effective distance of neaest upstream passing lane
                // Passing Lane itself can also be placed within the effective length

                if l_d < self.l_de.unwrap_or(0.0) {
                    let x_1a = 8.75 * f64::ln(f64::max(0.1, l_d));
                    let x_1b = 0.8 * l_d;
                    let x_4c = 0.01 * self.segments[seg_num].get_flow_rate();
                    let x_4d = 0.005 * self.segments[seg_num].get_flow_rate();
                    let y_1b = 27.0 - x_1a + x_2 + x_3a - x_4c;
                    let y_2b = 3.0 - x_1b + x_2 + x_3b - x_4d;
                    let pf_improve = math::round_up_to_n_decimal(f64::max(0.0, y_1b), 1);
                    let s_improve = math::round_up_to_n_decimal(f64::max(0.0, y_2b), 1);

                    fd_adj = math::round_up_to_n_decimal(pf, 1) / 100.0
                        * (1.0 - pf_improve / 100.0)
                        * math::round_to_significant_digits(vd, 3)
                        / (math::round_up_to_n_decimal(s, 1) * (1.0 + s_improve / 100.0));
                }
            }
        }
        fd_adj
    }

    /// Step 10: Determine segment Level of Service (Exhibit 15-6).
    ///
    /// Determines LOS based on follower density and posted speed limit category.
    /// Two sets of thresholds account for different driver expectations on
    /// higher-speed vs lower-speed highways.
    ///
    /// # LOS Criteria (Exhibit 15-6)
    ///
    /// | LOS | Higher-Speed (≥50 mi/h) | Lower-Speed (<50 mi/h) |
    /// |-----|-------------------------|------------------------|
    /// | A   | FD ≤ 2.0               | FD ≤ 2.5              |
    /// | B   | FD ≤ 4.0               | FD ≤ 5.0              |
    /// | C   | FD ≤ 8.0               | FD ≤ 10.0             |
    /// | D   | FD ≤ 12.0              | FD ≤ 15.0             |
    /// | E   | FD > 12.0              | FD > 15.0             |
    /// | F   | Demand > Capacity      | Demand > Capacity     |
    ///
    /// # Arguments
    ///
    /// * `seg_num` - Index of the segment
    /// * `s_pl` - Posted speed limit (mi/h) - determines threshold set
    /// * `cap` - Segment capacity (veh/h)
    ///
    /// # Returns
    ///
    /// LOS letter ('A' through 'F')
    ///
    /// # Notes
    ///
    /// - For PL segments, uses midpoint follower density (FD_PLmid)
    /// - For PC/PZ segments, uses endpoint follower density
    /// - LOS F occurs when demand exceeds capacity, regardless of FD
    pub fn determine_segment_los(&self, seg_num: usize, s_pl: f64, cap: i32) -> char {
        let mut los: char = 'F';

        let vd = self.segments[seg_num].get_flow_rate();
        let pt = self.segments[seg_num].get_passing_type();
        let fd: f64;
        if pt == 2 {
            fd = self.segments[seg_num].get_followers_density_mid();
        } else {
            fd = self.segments[seg_num].get_followers_density();
        }

        if s_pl >= 50.0 {
            if fd <= 2.0 {
                los = 'A'
            } else if fd > 2.0 && fd <= 4.0 {
                los = 'B'
            } else if fd > 4.0 && fd <= 8.0 {
                los = 'C'
            } else if fd > 8.0 && fd <= 12.0 {
                los = 'D'
            } else if fd > 12.0 {
                los = 'E'
            };
            if vd > cap as f64 {
                los = 'F'
            };
        } else {
            if fd <= 2.5 {
                los = 'A'
            } else if fd > 2.5 && fd <= 5.0 {
                los = 'B'
            } else if fd > 5.0 && fd <= 10.0 {
                los = 'C'
            } else if fd > 10.0 && fd <= 15.0 {
                los = 'D'
            } else if fd > 15.0 {
                los = 'E'
            }
            if vd > cap as f64 {
                los = 'F'
            };
        }

        los
    }

    /// Step 11: Determine facility-level Level of Service (Equation 15-39).
    ///
    /// For multi-segment facility analysis, computes the length-weighted average
    /// follower density and determines overall facility LOS.
    ///
    /// # Facility Follower Density (Equation 15-39)
    ///
    /// ```text
    /// FD_F = Σ(FD_i × L_i) / Σ(L_i)
    /// ```
    ///
    /// where actual segment lengths are used (not min/max constrained values).
    /// For passing lane segments, FD_PLmid is used as the segment FD value.
    ///
    /// # Arguments
    ///
    /// * `fd` - Facility average follower density (followers/mi/ln)
    /// * `s_pl` - Posted speed limit (mi/h) - determines threshold set
    ///
    /// # Returns
    ///
    /// Facility LOS letter ('A' through 'F')
    ///
    /// # Example
    ///
    /// ```ignore
    /// // After analyzing all segments, compute facility LOS:
    /// let facility_fd = total_fd_times_length / total_length;
    /// let avg_speed = total_speed_times_length / total_length;
    /// let facility_los = highway.determine_facility_los(facility_fd, avg_speed);
    /// ```
    pub fn determine_facility_los(&self, fd: f64, s_pl: f64) -> char {
        let mut los: char = 'F';

        if s_pl >= 50.0 {
            if fd <= 2.0 {
                los = 'A'
            } else if fd > 2.0 && fd <= 4.0 {
                los = 'B'
            } else if fd > 4.0 && fd <= 8.0 {
                los = 'C'
            } else if fd > 8.0 && fd <= 12.0 {
                los = 'D'
            } else if fd > 12.0 {
                los = 'E'
            };
        } else {
            if fd <= 2.5 {
                los = 'A'
            } else if fd > 2.5 && fd <= 5.0 {
                los = 'B'
            } else if fd > 5.0 && fd <= 10.0 {
                los = 'C'
            } else if fd > 10.0 && fd <= 15.0 {
                los = 'D'
            } else if fd > 15.0 {
                los = 'E'
            }
        }

        los
    }
}

/// Bicycle Level of Service analysis for Two-Lane and Multilane Highways (Section 4, Chapter 15).
///
/// This methodology evaluates bicyclist perception of operating conditions using a
/// traveler perception model. The same model applies to both two-lane and multilane
/// highways since bicyclists operate similarly on both facility types - traveling
/// slowly, staying far right, and using shoulders when available.
///
/// # Performance Measures
///
/// - **BLOS Score**: Numeric score (typically 0.5 to 6.5) reflecting bicyclist perception
/// - **Bicycle LOS**: Letter grade A-F based on BLOS score thresholds
///
/// # Key Factors (in order of importance)
///
/// 1. Average effective width of outside through lane
/// 2. Motorized vehicle volumes
/// 3. Motorized vehicle speeds
/// 4. Heavy vehicle (truck) volumes
/// 5. Pavement condition
///
/// # LOS Criteria (Exhibit 15-7)
///
/// | LOS | BLOS Score |
/// |-----|------------|
/// | A   | ≤ 1.5     |
/// | B   | > 1.5 - 2.5 |
/// | C   | > 2.5 - 3.5 |
/// | D   | > 3.5 - 4.5 |
/// | E   | > 4.5 - 5.5 |
/// | F   | > 5.5     |
///
/// # Example
///
/// ```ignore
/// let bike_analysis = BicycleLOS::new(
///     12.0,   // lane width
///     6.0,    // shoulder width
///     50.0,   // speed limit
///     1,      // number of lanes
///     4.0,    // pavement condition (good)
///     500.0,  // hourly volume
///     0.88,   // PHF
///     0.06,   // heavy vehicle %
///     0.0,    // on-highway parking %
/// );
///
/// let result = bike_analysis.analyze();
/// println!("BLOS Score: {:.2}, LOS: {}", result.blos_score, result.los);
/// ```
///
/// # Limitations
///
/// - Model developed from urban/suburban data; rural conditions may differ
/// - Does not account for regional driver behavior variations
/// - Input ranges used in model development:
///   - Lane width: 10-16 ft
///   - Shoulder width: 0-6 ft
///   - AADT: up to 36,000
///   - Speed limit: 25-50 mi/h
///   - Heavy vehicles: 0-2%
///   - Pavement condition: 2-5 (FHWA scale)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BicycleLOS {
    /// Outside lane width in feet (default: 12 ft).
    /// Model calibrated for 10-16 ft range.
    pub lane_width: f64,
    /// Paved shoulder width in feet (default: 6 ft).
    /// Wider shoulders (≥4 ft) provide dedicated bicycle space.
    pub shoulder_width: f64,
    /// Posted speed limit in mi/h.
    /// Higher speeds increase speed differential with bicyclists.
    pub speed_limit: f64,
    /// Number of directional through lanes (1 for two-lane highways, 2+ for multilane).
    pub num_lanes: i32,
    /// Pavement condition using FHWA 5-point scale:
    /// 1=very poor, 2=poor, 3=fair, 4=good, 5=very good.
    /// Very poor pavement (1) typically results in LOS F.
    pub pavement_condition: f64,
    /// Hourly directional volume, veh/hr
    pub hourly_volume: f64,
    /// Peak hour factor (default: 0.88)
    pub phf: f64,
    /// Heavy vehicle percentage (decimal, e.g., 0.06 for 6%)
    pub heavy_vehicle_pct: f64,
    /// Percent of segment with occupied on-highway parking (decimal)
    pub pct_on_highway_parking: f64,
}

impl Default for BicycleLOS {
    fn default() -> Self {
        BicycleLOS {
            lane_width: 12.0,
            shoulder_width: 6.0,
            speed_limit: 50.0,
            num_lanes: 1,
            pavement_condition: 4.0,
            hourly_volume: 500.0,
            phf: 0.88,
            heavy_vehicle_pct: 0.06,
            pct_on_highway_parking: 0.0,
        }
    }
}

impl BicycleLOS {
    /// Create a new BicycleLOS instance
    pub fn new(
        lane_width: f64,
        shoulder_width: f64,
        speed_limit: f64,
        num_lanes: i32,
        pavement_condition: f64,
        hourly_volume: f64,
        phf: f64,
        heavy_vehicle_pct: f64,
        pct_on_highway_parking: f64,
    ) -> Self {
        BicycleLOS {
            lane_width,
            shoulder_width,
            speed_limit,
            num_lanes,
            pavement_condition,
            hourly_volume,
            phf,
            heavy_vehicle_pct,
            pct_on_highway_parking,
        }
    }

    /// Step 2: Calculate the directional flow rate in the outside lane (Equation 15-40)
    /// v_OL = V / (PHF * N)
    pub fn calculate_flow_rate_outside_lane(&self) -> f64 {
        self.hourly_volume / (self.phf * self.num_lanes as f64)
    }

    /// Calculate effective width as a function of traffic volume (Equations 15-44, 15-45)
    fn calculate_wv(&self) -> f64 {
        if self.hourly_volume > 160.0 {
            // Equation 15-44
            self.lane_width
        } else {
            // Equation 15-45
            self.lane_width + self.shoulder_width
        }
    }

    /// Step 3: Calculate the effective width (Equations 15-41 to 15-45)
    pub fn calculate_effective_width(&self) -> f64 {
        let ws = self.shoulder_width;
        let wol = self.lane_width;
        let wv = self.calculate_wv();
        let pct_ohp = self.pct_on_highway_parking;

        let we = if ws >= 8.0 {
            // Equation 15-41: Ws >= 8 ft
            wol + ws - 20.0 * pct_ohp
        } else if ws >= 4.0 {
            // Equation 15-42: 4 ft <= Ws < 8 ft
            wol + ws - 20.0 * pct_ohp
        } else {
            // Equation 15-43: Ws < 4 ft
            wv + ws * (1.0 - 2.0 * pct_ohp)
        };

        // Ensure effective width is positive
        we.max(1.0)
    }

    /// Step 4: Calculate the effective speed factor (Equation 15-46)
    /// St = 1.1199 * ln(Spl - 20) + 0.8103
    pub fn calculate_effective_speed_factor(&self) -> f64 {
        1.1199 * f64::ln(self.speed_limit - 20.0) + 0.8103
    }

    /// Step 5: Calculate the BLOS score (Equation 15-47)
    /// BLOS = 0.507 * ln(v_OL) + 0.199 * St * (1 + 10.38 * HV)^2 + 7.066 * (1/P)^2 - 0.005 * We^2 + 0.760
    pub fn calculate_blos_score(&self) -> f64 {
        let v_ol = self.calculate_flow_rate_outside_lane();
        let st = self.calculate_effective_speed_factor();
        let we = self.calculate_effective_width();
        let p = self.pavement_condition;

        // For low volumes (V < 200 veh/h), HV should be limited to max 0.5
        let hv = if self.hourly_volume < 200.0 {
            self.heavy_vehicle_pct.min(0.5)
        } else {
            self.heavy_vehicle_pct
        };

        // Handle edge case where v_OL is very small or zero
        let ln_v_ol = if v_ol > 0.0 { f64::ln(v_ol) } else { 0.0 };

        // Equation 15-47
        0.507 * ln_v_ol
            + 0.199 * st * (1.0 + 10.38 * hv).powi(2)
            + 7.066 * (1.0 / p).powi(2)
            - 0.005 * we.powi(2)
            + 0.760
    }

    /// Determine Bicycle LOS based on BLOS score (Exhibit 15-7)
    pub fn determine_bicycle_los(&self) -> char {
        let blos = self.calculate_blos_score();

        if blos <= 1.5 {
            'A'
        } else if blos <= 2.5 {
            'B'
        } else if blos <= 3.5 {
            'C'
        } else if blos <= 4.5 {
            'D'
        } else if blos <= 5.5 {
            'E'
        } else {
            'F'
        }
    }

    /// Get all bicycle performance measures
    pub fn analyze(&self) -> BicycleLOSResult {
        BicycleLOSResult {
            flow_rate_outside_lane: self.calculate_flow_rate_outside_lane(),
            effective_width: self.calculate_effective_width(),
            effective_speed_factor: self.calculate_effective_speed_factor(),
            blos_score: self.calculate_blos_score(),
            los: self.determine_bicycle_los(),
        }
    }
}

/// Results from bicycle LOS analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BicycleLOSResult {
    /// Directional demand flow rate in outside lane (veh/h)
    pub flow_rate_outside_lane: f64,
    /// Effective width of the outside through lane (ft)
    pub effective_width: f64,
    /// Effective speed factor
    pub effective_speed_factor: f64,
    /// Bicycle LOS score
    pub blos_score: f64,
    /// Bicycle Level of Service (A-F)
    pub los: char,
}
