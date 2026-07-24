//! HCM Chapter 19, Section 6: Bicycle methodology for signalized intersections.
//!
//! Computes bicycle delay (Equations 19-106/19-107) and the bicycle LOS score
//! for the intersection (Equations 19-108 through 19-110), with LOS from the
//! Exhibit 19-9 thresholds. This score is the `I_b,int` input consumed by the
//! Chapter 18 bicycle segment methodology.

use serde::{Deserialize, Serialize};
use crate::hcm::common::LevelOfService;

/// Default bicycle saturation flow rate s_b (bicycles/h) - Chapter 19 guidance.
pub const DEFAULT_BICYCLE_SATURATION_FLOW: f64 = 2000.0;

/// Pedestrian/bicycle intersection LOS from a LOS score - Exhibit 19-9.
/// A <=1.50, B <=2.50, C <=3.50, D <=4.50, E <=5.50, F >5.50.
pub fn ped_bike_intersection_los(score: f64) -> LevelOfService {
    match score {
        s if s <= 1.50 => LevelOfService::A,
        s if s <= 2.50 => LevelOfService::B,
        s if s <= 3.50 => LevelOfService::C,
        s if s <= 4.50 => LevelOfService::D,
        s if s <= 5.50 => LevelOfService::E,
        _ => LevelOfService::F,
    }
}

/// Inputs for the HCM Chapter 19 bicycle LOS at a signalized intersection
/// (one bicycle lane / approach).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct BicycleIntersection {
    /// Bicycle saturation flow rate s_b, bicycles/h.
    pub saturation_flow: f64,
    /// Effective green time for the bicycle movement g_b, s.
    pub effective_green_s: f64,
    /// Cycle length C, s.
    pub cycle_length_s: f64,
    /// Bicycle flow rate v_bic, bicycles/h.
    pub bicycle_flow: f64,
    /// Curb-to-curb cross street width W_cd, ft.
    pub cross_street_width_ft: f64,
    /// Total width of the outside through lane, bicycle lane, and paved
    /// shoulder W_t, ft.
    pub total_width_ft: f64,
    /// Left-turn demand flow rate v_lt at the intersection, veh/h.
    pub v_left: f64,
    /// Through demand flow rate v_th, veh/h.
    pub v_through: f64,
    /// Right-turn demand flow rate v_rt, veh/h.
    pub v_right: f64,
    /// Number of through lanes N_th, ln.
    pub num_through_lanes: f64,
}

impl Default for BicycleIntersection {
    fn default() -> Self {
        Self {
            saturation_flow: DEFAULT_BICYCLE_SATURATION_FLOW,
            effective_green_s: 0.0,
            cycle_length_s: 0.0,
            bicycle_flow: 0.0,
            cross_street_width_ft: 0.0,
            total_width_ft: 0.0,
            v_left: 0.0,
            v_through: 0.0,
            v_right: 0.0,
            num_through_lanes: 1.0,
        }
    }
}

/// Result of a bicycle intersection evaluation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BicycleIntersectionAnalysis {
    /// Bicycle lane capacity c_b, bicycles/h (Equation 19-106).
    pub capacity: f64,
    /// Bicycle delay d_b, s/bicycle (Equation 19-107).
    pub delay: f64,
    /// Cross-section adjustment factor F_w (Equation 19-109).
    pub f_w: f64,
    /// Motor-vehicle volume adjustment factor F_v (Equation 19-110).
    pub f_v: f64,
    /// Bicycle LOS score for the intersection I_b,int (Equation 19-108).
    pub los_score: f64,
    /// Bicycle LOS (Exhibit 19-9).
    pub los: LevelOfService,
}

impl BicycleIntersection {
    pub fn new() -> Self {
        Self::default()
    }

    /// Full bicycle intersection evaluation (Chapter 19, Section 6 steps).
    pub fn analyze(&self) -> BicycleIntersectionAnalysis {
        let gc = if self.cycle_length_s > 0.0 {
            self.effective_green_s / self.cycle_length_s
        } else {
            0.0
        };
        // Equation 19-106: bicycle lane capacity.
        let capacity = self.saturation_flow * gc;
        // Equation 19-107: bicycle delay.
        let vc = if capacity > 0.0 {
            (self.bicycle_flow / capacity).min(1.0)
        } else {
            1.0
        };
        let denom = 1.0 - vc * gc;
        let delay = if denom > 0.0 {
            0.5 * self.cycle_length_s * (1.0 - gc).powi(2) / denom
        } else {
            f64::INFINITY
        };
        // Equation 19-109: cross-section adjustment factor.
        let f_w = 0.0153 * self.cross_street_width_ft - 0.2144 * self.total_width_ft;
        // Equation 19-110: motor-vehicle volume adjustment factor.
        let f_v = 0.0066 * (self.v_left + self.v_through + self.v_right)
            / (4.0 * self.num_through_lanes);
        // Equation 19-108: bicycle LOS score.
        let los_score = 4.1324 + f_w + f_v;

        BicycleIntersectionAnalysis {
            capacity,
            delay,
            f_w,
            f_v,
            los_score,
            los: ped_bike_intersection_los(los_score),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// HCM Chapter 31, Example Problem 3: bicycle LOS of a 5-ft bicycle lane at
    /// a signalized intersection.
    #[test]
    fn example_problem_3_bicycle_los() {
        let ix = BicycleIntersection {
            saturation_flow: 2000.0,
            effective_green_s: 48.0,
            cycle_length_s: 120.0,
            bicycle_flow: 120.0,
            cross_street_width_ft: 70.0,
            total_width_ft: 17.0,
            v_left: 85.0,
            v_through: 924.0,
            v_right: 77.0,
            num_through_lanes: 2.0,
        };
        let a = ix.analyze();
        assert!((a.capacity - 800.0).abs() < 1e-6, "c_b {}", a.capacity);
        assert!((a.delay - 23.0).abs() < 0.1, "d_b {}", a.delay);
        assert!((a.f_w - -2.57).abs() < 0.01, "F_w {}", a.f_w);
        assert!((a.f_v - 0.90).abs() < 0.01, "F_v {}", a.f_v);
        assert!((a.los_score - 2.45).abs() < 0.01, "I_b,int {}", a.los_score);
        assert_eq!(a.los, LevelOfService::B);
    }

    #[test]
    fn los_thresholds() {
        assert_eq!(ped_bike_intersection_los(1.50), LevelOfService::A);
        assert_eq!(ped_bike_intersection_los(2.45), LevelOfService::B);
        assert_eq!(ped_bike_intersection_los(5.51), LevelOfService::F);
    }
}
