//! HCM Chapter 18, Section 5: Bicycle methodology for urban street segments.
//!
//! Implements the eight-step segment-based bicycle evaluation: travel speed
//! (Equation 18-40), the link bicycle LOS score (Equations 18-41 through 18-45
//! with the Exhibit 18-25 effective-width conditions), and the segment bicycle
//! LOS score (Equations 18-46/18-47). LOS letters come from the Exhibit 18-3
//! bicycle thresholds. The boundary-intersection inputs (bicycle control delay
//! and intersection LOS score) are "HCM method output" values obtained from the
//! Chapter 19 bicycle procedure.

use serde::{Deserialize, Serialize};
use crate::hcm::common::LevelOfService;

/// Default average bicycle running speed when field data are unavailable
/// (Chapter 18, Step 1 guidance), mi/h.
pub const DEFAULT_BICYCLE_RUNNING_SPEED: f64 = 15.0;

/// Bicycle LOS from a segment-based bicycle LOS score - Exhibit 18-3.
/// A <=2.00, B <=2.75, C <=3.50, D <=4.25, E <=5.00, F >5.00.
pub fn bicycle_segment_los(score: f64) -> LevelOfService {
    match score {
        s if s <= 2.00 => LevelOfService::A,
        s if s <= 2.75 => LevelOfService::B,
        s if s <= 3.50 => LevelOfService::C,
        s if s <= 4.25 => LevelOfService::D,
        s if s <= 5.00 => LevelOfService::E,
        _ => LevelOfService::F,
    }
}

/// Bicycle LOS from a link-based bicycle LOS score - Exhibit 18-3.
/// A <=1.50, B <=2.50, C <=3.50, D <=4.50, E <=5.50, F >5.50.
pub fn bicycle_link_los(score: f64) -> LevelOfService {
    match score {
        s if s <= 1.50 => LevelOfService::A,
        s if s <= 2.50 => LevelOfService::B,
        s if s <= 3.50 => LevelOfService::C,
        s if s <= 4.50 => LevelOfService::D,
        s if s <= 5.50 => LevelOfService::E,
        _ => LevelOfService::F,
    }
}

/// Inputs for the HCM Chapter 18 bicycle segment evaluation (one direction of
/// travel). Widths are in feet; the boundary-intersection performance measures
/// come from the Chapter 19 bicycle methodology.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct BicycleSegment {
    /// Segment length L, ft.
    pub length_ft: f64,
    /// Number of through lanes in the subject direction N_th, ln.
    pub num_through_lanes: f64,
    /// Midsegment motorized vehicle flow rate v_m, veh/h.
    pub midseg_flow_rate: f64,
    /// Percent heavy vehicles in the midsegment flow P_HV, %.
    pub pct_heavy_vehicles: f64,
    /// Proportion of on-street parking occupied p_pk, decimal.
    pub prop_parking_occupied: f64,
    /// Width of the outside through lane W_ol, ft.
    pub width_outside_lane_ft: f64,
    /// Width of the bicycle lane W_bl (0 if none), ft.
    pub width_bike_lane_ft: f64,
    /// Width of the paved outside shoulder W_os, ft.
    pub width_outside_shoulder_ft: f64,
    /// Width of the striped parking lane W_pk, ft.
    pub width_parking_lane_ft: f64,
    /// Divided median (nonrestrictive or restrictive) present.
    pub median_divided: bool,
    /// Curb present along the outside edge.
    pub curb_present: bool,
    /// Access point approaches on the right side in the subject direction N_ap,s.
    pub num_access_points_right: f64,
    /// Pavement condition rating P_c (Exhibit 18-23, 0-5 scale).
    pub pavement_condition: f64,
    /// Motorized vehicle running speed S_R, mi/h (Chapter 18 Section 3 output).
    pub motor_running_speed: f64,
    /// Average bicycle running speed S_b, mi/h.
    pub bicycle_running_speed: f64,
    /// Bicycle control delay at the boundary intersection d_b, s/bicycle.
    pub bicycle_control_delay: f64,
    /// Bicycle LOS score at the boundary intersection I_b,int (0 if two-way STOP).
    pub bicycle_los_score_intersection: f64,
}

impl Default for BicycleSegment {
    fn default() -> Self {
        Self {
            length_ft: 1320.0,
            num_through_lanes: 2.0,
            midseg_flow_rate: 0.0,
            pct_heavy_vehicles: 0.0,
            prop_parking_occupied: 0.0,
            width_outside_lane_ft: 12.0,
            width_bike_lane_ft: 0.0,
            width_outside_shoulder_ft: 0.0,
            width_parking_lane_ft: 0.0,
            median_divided: false,
            curb_present: false,
            num_access_points_right: 0.0,
            pavement_condition: 3.5,
            motor_running_speed: 30.0,
            bicycle_running_speed: DEFAULT_BICYCLE_RUNNING_SPEED,
            bicycle_control_delay: 0.0,
            bicycle_los_score_intersection: 0.0,
        }
    }
}

/// Result of a bicycle segment evaluation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BicycleAnalysis {
    /// Segment running time of through bicycles t_Rb, s.
    pub running_time_s: f64,
    /// Bicycle travel speed S_Tb,seg, mi/h (Equation 18-40).
    pub travel_speed: f64,
    /// Effective width of the outside through lane W_e, ft (Exhibit 18-25).
    pub effective_width: f64,
    /// Cross-section adjustment factor F_w (Equation 18-42).
    pub f_w: f64,
    /// Motorized vehicle volume adjustment factor F_v (Equation 18-43).
    pub f_v: f64,
    /// Motorized vehicle speed adjustment factor F_s (Equation 18-44).
    pub f_s: f64,
    /// Pavement condition adjustment factor F_p (Equation 18-45).
    pub f_p: f64,
    /// Link bicycle LOS score I_b,link (Equation 18-41).
    pub link_score: f64,
    /// Link bicycle LOS (Exhibit 18-3).
    pub link_los: LevelOfService,
    /// Unsignalized conflicts factor F_c (Equation 18-47).
    pub f_c: f64,
    /// Segment bicycle LOS score I_b,seg (Equation 18-46).
    pub segment_score: f64,
    /// Segment bicycle LOS (Exhibit 18-3).
    pub segment_los: LevelOfService,
}

impl BicycleSegment {
    pub fn new() -> Self {
        Self::default()
    }

    /// Segment running time of through bicycles t_Rb = 3,600 L / (5,280 S_b), s.
    pub fn running_time(&self) -> f64 {
        3600.0 * self.length_ft / (5280.0 * self.bicycle_running_speed)
    }

    /// Effective width of the outside through lane W_e, ft - Exhibit 18-25
    /// (the sequential cross-section conditions).
    pub fn effective_width(&self) -> f64 {
        // Adjusted paved shoulder width: subtract 1.5 ft when a curb is present.
        let w_os_star = if self.curb_present {
            (self.width_outside_shoulder_ft - 1.5).max(0.0)
        } else {
            self.width_outside_shoulder_ft
        };
        let w_l = self.width_bike_lane_ft + w_os_star + self.width_parking_lane_ft;
        // Row 1: the parking-lane width is included only when no parking is occupied.
        let w_t = if self.prop_parking_occupied <= 0.0 {
            self.width_outside_lane_ft + self.width_bike_lane_ft + w_os_star + self.width_parking_lane_ft
        } else {
            self.width_outside_lane_ft + self.width_bike_lane_ft + w_os_star
        };
        // Row 2: volume-based effective width.
        let w_v = if self.midseg_flow_rate > 160.0 || self.median_divided {
            w_t
        } else {
            w_t * (2.0 - 0.005 * self.midseg_flow_rate)
        };
        // Row 3: effective width, floored at 0.
        if w_l < 4.0 {
            (w_v - 10.0 * self.prop_parking_occupied).max(0.0)
        } else {
            (w_v + w_l - 20.0 * self.prop_parking_occupied).max(0.0)
        }
    }

    /// Full bicycle segment evaluation (Steps 3-8).
    pub fn analyze(&self) -> BicycleAnalysis {
        let t_rb = self.running_time();
        let d_b = self.bicycle_control_delay;
        // Equation 18-40: travel speed.
        let travel_speed = 3600.0 * self.length_ft / (5280.0 * (t_rb + d_b));

        // Exhibit 18-25 adjusted variables.
        let w_e = self.effective_width();
        let p_hva = if self.midseg_flow_rate * (1.0 - 0.01 * self.pct_heavy_vehicles) < 200.0
            && self.pct_heavy_vehicles > 50.0
        {
            50.0
        } else {
            self.pct_heavy_vehicles
        };
        let s_ra = self.motor_running_speed.max(21.0);
        let v_ma = self.midseg_flow_rate.max(4.0 * self.num_through_lanes);

        // Equations 18-42 through 18-45.
        let f_w = -0.005 * w_e * w_e;
        let f_v = 0.507 * (v_ma / (4.0 * self.num_through_lanes)).ln();
        let f_s = 0.199
            * (1.1199 * (s_ra - 20.0).ln() + 0.8103)
            * (1.0 + 0.1038 * p_hva).powi(2);
        let f_p = 7.066 / (self.pavement_condition * self.pavement_condition);
        // Equation 18-41.
        let link_score = 0.760 + f_w + f_v + f_s + f_p;

        // Equation 18-47.
        let f_c = 0.035 * (5280.0 * self.num_access_points_right / self.length_ft - 20.0);
        let i_int = self.bicycle_los_score_intersection;
        // Equation 18-46.
        let num = (f_c + link_score + 1.0).powi(3) * t_rb + (i_int + 1.0).powi(3) * d_b;
        let segment_score = 0.75 * (num / (t_rb + d_b)).cbrt() + 0.125;

        BicycleAnalysis {
            running_time_s: t_rb,
            travel_speed,
            effective_width: w_e,
            f_w,
            f_v,
            f_s,
            f_p,
            link_score,
            link_los: bicycle_link_los(link_score),
            f_c,
            segment_score,
            segment_los: bicycle_segment_los(segment_score),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// HCM Chapter 30, Example Problem 3: bicycle LOS on a 1,320-ft collector
    /// segment with an eastbound bicycle lane (Exhibit 30-38).
    #[test]
    fn example_problem_3_bicycle_los() {
        let seg = BicycleSegment {
            length_ft: 1320.0,
            num_through_lanes: 2.0,
            midseg_flow_rate: 940.0,
            pct_heavy_vehicles: 8.0,
            prop_parking_occupied: 0.20,
            width_outside_lane_ft: 12.0,
            width_bike_lane_ft: 5.0,
            width_outside_shoulder_ft: 0.0,
            width_parking_lane_ft: 9.5,
            median_divided: false,
            curb_present: true,
            num_access_points_right: 3.0,
            pavement_condition: 2.0,
            motor_running_speed: 33.0,
            bicycle_running_speed: 15.0,
            bicycle_control_delay: 40.0,
            bicycle_los_score_intersection: 0.08,
        };
        let a = seg.analyze();

        assert!((a.running_time_s - 60.0).abs() < 0.1, "t_Rb {}", a.running_time_s);
        assert!((a.travel_speed - 9.0).abs() < 0.1, "S_Tb,seg {}", a.travel_speed);
        assert!((a.effective_width - 27.5).abs() < 0.1, "W_e {}", a.effective_width);
        assert!((a.f_w - -3.78).abs() < 0.01, "F_w {}", a.f_w);
        assert!((a.f_v - 2.42).abs() < 0.01, "F_v {}", a.f_v);
        assert!((a.f_s - 2.46).abs() < 0.01, "F_s {}", a.f_s);
        assert!((a.f_p - 1.77).abs() < 0.01, "F_p {}", a.f_p);
        assert!((a.link_score - 3.62).abs() < 0.02, "I_b,link {}", a.link_score);
        assert_eq!(a.link_los, LevelOfService::D);
        assert!((a.f_c - -0.28).abs() < 0.01, "F_c {}", a.f_c);
        assert!((a.segment_score - 2.88).abs() < 0.02, "I_b,seg {}", a.segment_score);
        assert_eq!(a.segment_los, LevelOfService::C);
    }

    #[test]
    fn los_thresholds() {
        assert_eq!(bicycle_segment_los(2.00), LevelOfService::A);
        assert_eq!(bicycle_segment_los(2.88), LevelOfService::C);
        assert_eq!(bicycle_segment_los(5.01), LevelOfService::F);
        assert_eq!(bicycle_link_los(1.50), LevelOfService::A);
        assert_eq!(bicycle_link_los(3.62), LevelOfService::D);
    }
}
