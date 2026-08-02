//! HCM Chapter 19, Section 5: Pedestrian methodology for signalized
//! intersections (LOS determination, Steps 1-3).
//!
//! Computes pedestrian delay (Equations 19-51/19-54) and the pedestrian LOS
//! score for the intersection (Equations 19-55 through 19-60), with LOS from
//! the Exhibit 19-9 thresholds. This score is the `I_p,int` input consumed by
//! the Chapter 18 pedestrian segment methodology.
//!
//! The optional street-corner and crosswalk circulation-area measures
//! (Equations 19-61 onward, Steps 4-5) are performance measures that do not
//! affect LOS and are not implemented here. The effective walk time uses
//! Equation 19-51 (pedestrian signal head present, rest-in-walk not enabled, or
//! pretimed); the rest-in-walk / no-signal variants (Equations 19-52/19-53) are
//! not covered.

use serde::{Deserialize, Serialize};
use crate::hcm::common::LevelOfService;
use super::bicycle::ped_bike_intersection_los;

/// Inputs for the HCM Chapter 19 pedestrian LOS at a signalized intersection
/// (one crosswalk).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct PedestrianIntersection {
    /// Cycle length C, s.
    pub cycle_length_s: f64,
    /// Walk setting for the phase serving the pedestrians (the street parallel
    /// to the crosswalk) Walk, s. Effective walk time is Walk + 4.0
    /// (Equation 19-51).
    pub walk_setting_s: f64,
    /// Number of traffic lanes crossed by the subject crosswalk N_d, ln
    /// (Equation 19-56).
    pub lanes_crossed: f64,
    /// Right-turn-on-red demand flow rate that conflicts with the crossing
    /// v_rtor, veh/h.
    pub v_rtor: f64,
    /// Permitted left-turn demand flow rate that conflicts with the crossing
    /// v_lt,perm, veh/h.
    pub v_lt_perm: f64,
    /// Number of right-turn channelizing islands along the crosswalk
    /// N_rtci,d (Equation 19-57).
    pub num_rtci: f64,
    /// Sum of the movement demand flow rates on the street being crossed
    /// Σ v_i, veh/h (Equation 19-60).
    pub crossed_street_volume_sum: f64,
    /// Number of through lanes on the street being crossed N_c, ln
    /// (Equation 19-60).
    pub crossed_street_lanes: f64,
    /// 85th percentile speed of vehicles on the street being crossed at a
    /// midsegment location S_85, mi/h (Equation 19-58).
    pub speed_85_mph: f64,
}

impl Default for PedestrianIntersection {
    fn default() -> Self {
        Self {
            cycle_length_s: 0.0,
            walk_setting_s: 7.0,
            lanes_crossed: 2.0,
            v_rtor: 0.0,
            v_lt_perm: 0.0,
            num_rtci: 0.0,
            crossed_street_volume_sum: 0.0,
            crossed_street_lanes: 2.0,
            speed_85_mph: 0.0,
        }
    }
}

/// Result of a pedestrian intersection evaluation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PedestrianIntersectionAnalysis {
    /// Effective walk time g_Walk, s (Equation 19-51).
    pub effective_walk_s: f64,
    /// Pedestrian delay d_p, s/p (Equation 19-54).
    pub delay: f64,
    /// Vehicles per lane crossing in 15 min n_15 (Equation 19-60).
    pub n15_per_lane: f64,
    /// Cross-section adjustment factor F_w (Equation 19-56).
    pub f_w: f64,
    /// Motorized vehicle adjustment factor F_v (Equation 19-57).
    pub f_v: f64,
    /// Motorized vehicle speed adjustment factor F_s (Equation 19-58).
    pub f_s: f64,
    /// Pedestrian delay adjustment factor F_delay (Equation 19-59).
    pub f_delay: f64,
    /// Pedestrian LOS score for the intersection I_p,int (Equation 19-55).
    pub los_score: f64,
    /// Pedestrian LOS (Exhibit 19-9).
    pub los: LevelOfService,
}

impl PedestrianIntersection {
    pub fn new() -> Self {
        Self::default()
    }

    /// Full pedestrian intersection evaluation (Steps 1-3).
    pub fn analyze(&self) -> PedestrianIntersectionAnalysis {
        let c = self.cycle_length_s;
        // Equation 19-51: effective walk time.
        let effective_walk_s = self.walk_setting_s + 4.0;
        // Equation 19-54: pedestrian delay.
        let delay = if c > 0.0 {
            (c - effective_walk_s).powi(2) / (2.0 * c)
        } else {
            0.0
        };
        // Equation 19-60: 15-min vehicle count per lane on the crossed street.
        let n15 = if self.crossed_street_lanes > 0.0 {
            0.25 * self.crossed_street_volume_sum / self.crossed_street_lanes
        } else {
            0.0
        };
        // Equation 19-56: cross-section adjustment factor.
        let f_w = 0.681 * self.lanes_crossed.powf(0.514);
        // Equation 19-57: motorized vehicle adjustment factor.
        let f_v = 0.00569 * (self.v_rtor + self.v_lt_perm) / 4.0
            - self.num_rtci * (0.0027 * n15 - 0.1946);
        // Equation 19-58: motorized vehicle speed adjustment factor.
        let f_s = 0.00013 * n15 * self.speed_85_mph;
        // Equation 19-59: pedestrian delay adjustment factor.
        let f_delay = if delay > 0.0 { 0.0401 * delay.ln() } else { 0.0 };
        // Equation 19-55: pedestrian LOS score.
        let los_score = 0.5997 + f_w + f_v + f_s + f_delay;

        PedestrianIntersectionAnalysis {
            effective_walk_s,
            delay,
            n15_per_lane: n15,
            f_w,
            f_v,
            f_s,
            f_delay,
            los_score,
            los: ped_bike_intersection_los(los_score),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Two-stage pedestrian crossing delay (Equations 19-78 through 19-88)
// ═══════════════════════════════════════════════════════════════════════════════

/// Inputs for the delay of a pedestrian crossing one intersection leg in two
/// stages (with a median refuge) - HCM Chapter 19 "Crossing One Intersection
/// Leg in Two Stages" procedure.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct TwoStageCrossing {
    /// Cycle length C, s.
    pub cycle_length_s: f64,
    /// Walk setting of the phase serving the first stage (Phase X) Walk_X, s.
    pub walk_setting_x_s: f64,
    /// Walk setting of the phase serving the second stage (Phase Y) Walk_Y, s.
    pub walk_setting_y_s: f64,
    /// First-stage crossing distance from the corner to the far side of the
    /// median L_X, ft (Equation 19-88).
    pub first_stage_distance_ft: f64,
    /// Average pedestrian crossing speed S_p, ft/s.
    pub walk_speed_fps: f64,
    /// Relative start time of the Phase X Walk interval T_Walk,X, s (from the
    /// phase sequence).
    pub walk_start_x_s: f64,
    /// Relative start time of the Phase Y Walk interval T_Walk,Y, s.
    pub walk_start_y_s: f64,
}

impl Default for TwoStageCrossing {
    fn default() -> Self {
        Self {
            cycle_length_s: 0.0,
            walk_setting_x_s: 5.0,
            walk_setting_y_s: 5.0,
            first_stage_distance_ft: 0.0,
            walk_speed_fps: 3.5,
            walk_start_x_s: 0.0,
            walk_start_y_s: 0.0,
        }
    }
}

impl TwoStageCrossing {
    /// Pedestrian delay for a two-stage crossing of one leg d_p, s/p
    /// (Equation 19-86), following Equations 19-78 through 19-88.
    pub fn delay(&self) -> f64 {
        let c = self.cycle_length_s;
        // Equation 19-51: effective walk times.
        let gx = self.walk_setting_x_s + 4.0;
        let gy = self.walk_setting_y_s + 4.0;
        // Equation 19-88: first-stage crossing time.
        let t_x = if self.walk_speed_fps > 0.0 {
            self.first_stage_distance_ft / self.walk_speed_fps
        } else {
            0.0
        };
        // Equation 19-79 / 19-81: time between Walk intervals; median wait time.
        let t_yx = (self.walk_start_y_s - self.walk_start_x_s).rem_euclid(c);
        let t = (t_yx - t_x).rem_euclid(c);
        // Equation 19-78: first-stage delay.
        let d_p1 = (c - gx).powi(2) / (2.0 * c);
        // Equation 19-80: median delay given arrival during DON'T WALK.
        let d2_dw = if t < c - gy { t } else { 0.0 };
        // Equations 19-82/19-83 (t < g_Walk,X) or 19-84/19-85 (t >= g_Walk,X):
        // median delay given arrival during WALK.
        let d2_w = if t < gx {
            let a = gx - gy - t;
            if t + gy < gx {
                (0.5 * (a + t).powi(2) + a * (c - gx)) / gx
            } else if t + gy <= c {
                0.5 * t * t / gx
            } else {
                0.5 * (c - gy).powi(2) / gx
            }
        } else {
            let b = gx - gy - t + c;
            if t + gy < c {
                t - 0.5 * gx
            } else if t + gy <= c + gx {
                (0.5 * b * b + b * (t - gx)) / gx
            } else {
                0.0
            }
        };
        // Equation 19-87: proportion arriving during DON'T WALK.
        let p_dw1 = (c - gx) / c;
        // Equation 19-86: two-stage crossing delay.
        d_p1 + d2_dw * p_dw1 + d2_w * (1.0 - p_dw1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// HCM Chapter 31, Example Problem 2: pedestrian LOS of a crosswalk across
    /// the minor-street (north) leg of a signalized intersection.
    #[test]
    fn example_problem_2_pedestrian_los() {
        let ix = PedestrianIntersection {
            cycle_length_s: 80.0,
            walk_setting_s: 7.0, // major-street phase serves the crossing
            lanes_crossed: 2.0,
            v_rtor: 30.0,
            v_lt_perm: 42.0,
            num_rtci: 0.0,
            // Minor-street movements: 72 + 336 + 60 + 42 + 400 + 76 = 986 veh/h
            crossed_street_volume_sum: 986.0,
            crossed_street_lanes: 2.0,
            speed_85_mph: 35.0,
        };
        let a = ix.analyze();
        assert!((a.effective_walk_s - 11.0).abs() < 1e-9, "g_Walk {}", a.effective_walk_s);
        assert!((a.delay - 29.8).abs() < 0.1, "d_p {}", a.delay);
        assert!((a.n15_per_lane - 123.3).abs() < 0.1, "n_15 {}", a.n15_per_lane);
        assert!((a.f_w - 0.972).abs() < 0.01, "F_w {}", a.f_w);
        assert!((a.f_v - 0.102).abs() < 0.01, "F_v {}", a.f_v);
        assert!((a.f_s - 0.561).abs() < 0.01, "F_s {}", a.f_s);
        assert!((a.f_delay - 0.136).abs() < 0.01, "F_delay {}", a.f_delay);
        assert!((a.los_score - 2.37).abs() < 0.02, "I_p,int {}", a.los_score);
        assert_eq!(a.los, LevelOfService::B);
    }

    /// HCM Chapter 31, Example Problem 4: pedestrian delay with a two-stage
    /// crossing of one intersection leg (northbound, Corner B to Corner A).
    #[test]
    fn example_problem_4_two_stage_one_leg() {
        let x = TwoStageCrossing {
            cycle_length_s: 140.0,
            walk_setting_x_s: 5.0,
            walk_setting_y_s: 5.0,
            first_stage_distance_ft: 56.0, // 40 ft crosswalk + 16 ft median
            walk_speed_fps: 3.3,
            walk_start_x_s: 78.0,  // phase 8: 21 + 57
            walk_start_y_s: 112.0, // phase 7: 21 + 57 + 34
        };
        assert!((x.delay() - 78.0).abs() < 0.5, "d_p {}", x.delay());
    }
}
