//! HCM Chapter 18, Section 4: Pedestrian methodology for urban street segments.
//!
//! Implements the ten-step segment-based pedestrian evaluation: pedestrian
//! space (Equations 18-23 through 18-30), travel speed (Equation 18-31), the
//! link pedestrian LOS score (Equations 18-32 through 18-35 with the Exhibit
//! 18-19 conditions), the roadway-crossing difficulty factor (Equations 18-36
//! through 18-38 with the Exhibit 18-21 delay-to-score thresholds), and the
//! segment pedestrian LOS score (Equation 18-39). Segment LOS combines the
//! score with average pedestrian space per Exhibit 18-2. Boundary-intersection
//! delays and the intersection LOS score are Chapter 19/20 outputs.

use serde::{Deserialize, Serialize};
use crate::hcm::common::LevelOfService;

/// Default free-flow walking speed when field data are unavailable, ft/s.
pub const DEFAULT_FREE_FLOW_WALK_SPEED: f64 = 4.4;
/// Default proportion of pedestrians desiring a midblock crossing (Step 9).
pub const DEFAULT_PROP_MIDBLOCK_CROSSING: f64 = 0.35;

/// Rank a LOS letter A=0 .. F=5 for "worse-of" combination.
fn los_rank(l: LevelOfService) -> u8 {
    let c: char = l.into();
    c as u8 - b'A'
}

/// The worse (higher-lettered) of two LOS values.
fn worse_los(a: LevelOfService, b: LevelOfService) -> LevelOfService {
    if los_rank(a) >= los_rank(b) {
        a
    } else {
        b
    }
}

/// Score-based segment pedestrian LOS - Exhibit 18-2 (left).
/// A <=2.00, B <=2.75, C <=3.50, D <=4.25, E <=5.00, F >5.00.
pub fn pedestrian_score_los(score: f64) -> LevelOfService {
    match score {
        s if s <= 2.00 => LevelOfService::A,
        s if s <= 2.75 => LevelOfService::B,
        s if s <= 3.50 => LevelOfService::C,
        s if s <= 4.25 => LevelOfService::D,
        s if s <= 5.00 => LevelOfService::E,
        _ => LevelOfService::F,
    }
}

/// Space-based pedestrian LOS - Exhibit 18-2 (average pedestrian space,
/// ft^2/p): A >60, B >40, C >24, D >15, E >8, F <=8.
pub fn pedestrian_space_los(space: f64) -> LevelOfService {
    match space {
        s if s > 60.0 => LevelOfService::A,
        s if s > 40.0 => LevelOfService::B,
        s if s > 24.0 => LevelOfService::C,
        s if s > 15.0 => LevelOfService::D,
        s if s > 8.0 => LevelOfService::E,
        _ => LevelOfService::F,
    }
}

/// Link-based pedestrian LOS score - Exhibit 18-2 (right).
/// A <=1.50, B <=2.50, C <=3.50, D <=4.50, E <=5.50, F >5.50.
pub fn pedestrian_link_los(score: f64) -> LevelOfService {
    match score {
        s if s <= 1.50 => LevelOfService::A,
        s if s <= 2.50 => LevelOfService::B,
        s if s <= 3.50 => LevelOfService::C,
        s if s <= 4.50 => LevelOfService::D,
        s if s <= 5.50 => LevelOfService::E,
        _ => LevelOfService::F,
    }
}

/// LOS score for a midblock pedestrian crossing delay (s) - Exhibit 18-21,
/// piecewise-linear with interpolation; delays over 70 s map to 6.0.
pub fn crossing_delay_los_score(delay_s: f64) -> f64 {
    // (delay breakpoint, score) knots.
    const KNOTS: [(f64, f64); 7] = [
        (0.0, 0.0),
        (10.0, 1.5),
        (20.0, 2.5),
        (30.0, 3.5),
        (40.0, 4.5),
        (60.0, 5.5),
        (70.0, 6.0),
    ];
    if delay_s >= 70.0 {
        return 6.0;
    }
    for w in KNOTS.windows(2) {
        let (d0, s0) = w[0];
        let (d1, s1) = w[1];
        if delay_s <= d1 {
            return s0 + (s1 - s0) * (delay_s - d0) / (d1 - d0);
        }
    }
    6.0
}

/// Inputs for the HCM Chapter 18 pedestrian segment evaluation (one sidewalk,
/// one direction). Widths in feet, delays in s/p, speeds in ft/s or mi/h as
/// noted.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct PedestrianSegment {
    /// Segment length L, ft.
    pub length_ft: f64,
    /// Number of through lanes in the subject direction N_th, ln.
    pub num_through_lanes: f64,
    /// Midsegment motorized vehicle flow rate v_m, veh/h.
    pub midseg_flow_rate: f64,
    /// Pedestrian flow rate on the sidewalk (both directions) v_ped, p/h.
    pub ped_flow_rate: f64,
    /// Proportion of on-street parking occupied p_pk, decimal.
    pub prop_parking_occupied: f64,
    /// Total walkway (sidewalk) width W_T, ft.
    pub width_sidewalk_ft: f64,
    /// Buffer width between sidewalk and curb W_buf, ft.
    pub width_buffer_ft: f64,
    /// Effective width of fixed objects, inside of sidewalk W_O,i, ft.
    pub fixed_object_width_inside_ft: f64,
    /// Effective width of fixed objects, outside of sidewalk W_O,o, ft.
    pub fixed_object_width_outside_ft: f64,
    /// Proportion of sidewalk adjacent to a window display p_window.
    pub prop_window: f64,
    /// Proportion of sidewalk adjacent to a building face p_building.
    pub prop_building: f64,
    /// Proportion of sidewalk adjacent to a fence or low wall p_fence.
    pub prop_fence: f64,
    /// Continuous barrier >= 3 ft high in the buffer (sets f_b = 5.37).
    pub continuous_barrier: bool,
    /// Width of the outside through lane W_ol, ft.
    pub width_outside_lane_ft: f64,
    /// Width of the bicycle lane W_bl (0 if none), ft.
    pub width_bike_lane_ft: f64,
    /// Width of the paved outside shoulder W_os, ft.
    pub width_outside_shoulder_ft: f64,
    /// Width of the striped parking lane W_pk, ft.
    pub width_parking_lane_ft: f64,
    /// Curb present along the outside edge.
    pub curb_present: bool,
    /// Motorized vehicle running speed S_R, mi/h.
    pub motor_running_speed: f64,
    /// Free-flow walking speed S_pf, ft/s.
    pub free_flow_walk_speed: f64,
    /// Pedestrian delay walking parallel to the segment d_pp, s/p.
    pub ped_delay_parallel: f64,
    /// Pedestrian delay crossing at the nearest signal-controlled crossing d_pc, s/p.
    pub ped_delay_crossing_signal: f64,
    /// Pedestrian delay waiting for a gap at an uncontrolled midsegment crossing d_pw, s/p.
    pub ped_delay_crossing_uncontrolled: f64,
    /// Pedestrian LOS score at the boundary intersection I_p,int.
    pub ped_los_score_intersection: f64,
    /// Distance to the nearest signal-controlled crossing D_c, ft. When 0 or
    /// negative, the uniform-crossing default L/3 is used.
    pub dist_nearest_crossing_ft: f64,
    /// Proportion of pedestrians desiring a midblock crossing p_mx.
    pub prop_midblock_crossing: f64,
}

impl Default for PedestrianSegment {
    fn default() -> Self {
        Self {
            length_ft: 1320.0,
            num_through_lanes: 2.0,
            midseg_flow_rate: 0.0,
            ped_flow_rate: 0.0,
            prop_parking_occupied: 0.0,
            width_sidewalk_ft: 5.0,
            width_buffer_ft: 0.0,
            fixed_object_width_inside_ft: 0.0,
            fixed_object_width_outside_ft: 0.0,
            prop_window: 0.0,
            prop_building: 0.0,
            prop_fence: 0.0,
            continuous_barrier: false,
            width_outside_lane_ft: 12.0,
            width_bike_lane_ft: 0.0,
            width_outside_shoulder_ft: 0.0,
            width_parking_lane_ft: 0.0,
            curb_present: false,
            motor_running_speed: 30.0,
            free_flow_walk_speed: DEFAULT_FREE_FLOW_WALK_SPEED,
            ped_delay_parallel: 0.0,
            ped_delay_crossing_signal: 0.0,
            ped_delay_crossing_uncontrolled: 0.0,
            ped_los_score_intersection: 0.0,
            dist_nearest_crossing_ft: 0.0,
            prop_midblock_crossing: DEFAULT_PROP_MIDBLOCK_CROSSING,
        }
    }
}

/// Result of a pedestrian segment evaluation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PedestrianAnalysis {
    /// Effective sidewalk width W_E, ft (Equation 18-23).
    pub effective_sidewalk_width: f64,
    /// Pedestrian flow per unit width v_p, p/ft/min (Equation 18-28).
    pub flow_per_width: f64,
    /// Average walking speed S_p, ft/s (Equation 18-29).
    pub walking_speed: f64,
    /// Average pedestrian space A_p, ft^2/p (Equation 18-30).
    pub pedestrian_space: f64,
    /// Pedestrian travel speed S_Tp,seg, ft/s (Equation 18-31).
    pub travel_speed: f64,
    /// Cross-section adjustment factor F_w (Equation 18-33).
    pub f_w: f64,
    /// Motorized vehicle volume adjustment factor F_v (Equation 18-34).
    pub f_v: f64,
    /// Motorized vehicle speed adjustment factor F_s (Equation 18-35).
    pub f_s: f64,
    /// Link pedestrian LOS score I_p,link (Equation 18-32).
    pub link_score: f64,
    /// Link pedestrian LOS (Exhibit 18-2).
    pub link_los: LevelOfService,
    /// Midsegment crossing LOS score I_p,mx (Equation 18-38).
    pub crossing_score: f64,
    /// Segment pedestrian LOS score I_p,seg (Equation 18-39).
    pub segment_score: f64,
    /// Segment pedestrian LOS (Exhibit 18-2: worse of score and space LOS).
    pub segment_los: LevelOfService,
}

impl PedestrianSegment {
    pub fn new() -> Self {
        Self::default()
    }

    /// Adjusted paved shoulder width W_os* (curb subtracts 1.5 ft), ft.
    fn adjusted_shoulder(&self) -> f64 {
        if self.curb_present {
            (self.width_outside_shoulder_ft - 1.5).max(0.0)
        } else {
            self.width_outside_shoulder_ft
        }
    }

    /// Full pedestrian segment evaluation (Steps 2-10).
    pub fn analyze(&self) -> PedestrianAnalysis {
        let s_pf = self.free_flow_walk_speed;

        // ── Step 2: pedestrian space ────────────────────────────────────────
        // Equation 18-24 / 18-25: inside and outside shy distances.
        let w_si = self.width_buffer_ft.max(1.5);
        let w_so = 3.0 * self.prop_window + 2.0 * self.prop_building + 1.5 * self.prop_fence;
        // Equation 18-23: effective sidewalk width.
        let w_e = (self.width_sidewalk_ft
            - self.fixed_object_width_inside_ft
            - self.fixed_object_width_outside_ft
            - w_si
            - w_so)
            .max(0.0);
        // Equation 18-28: flow per unit width (p/ft/min).
        let v_p = if w_e > 0.0 {
            self.ped_flow_rate / (60.0 * w_e)
        } else {
            f64::INFINITY
        };
        // Equation 18-29: average walking speed, floored at 0.5 S_pf.
        let walking_speed = ((1.0 - 0.00078 * v_p * v_p) * s_pf).max(0.5 * s_pf);
        // Equation 18-30: average pedestrian space.
        let pedestrian_space = if v_p > 0.0 {
            60.0 * walking_speed / v_p
        } else {
            f64::INFINITY
        };

        // ── Step 4: pedestrian travel speed (Equation 18-31) ────────────────
        let travel_speed =
            self.length_ft / (self.length_ft / walking_speed + self.ped_delay_parallel);

        // ── Step 6: link pedestrian LOS score ───────────────────────────────
        let w_os_star = self.adjusted_shoulder();
        let adjacent_width = self.width_bike_lane_ft + w_os_star + self.width_parking_lane_ft;
        // Exhibit 18-19, row 1: effective outside-lane-plus-shoulder width W_v.
        let w_sum = self.width_outside_lane_ft + self.width_bike_lane_ft + w_os_star + self.width_parking_lane_ft;
        let w_v = if self.midseg_flow_rate > 160.0 || adjacent_width > 0.0 {
            w_sum
        } else {
            w_sum * (2.0 - 0.005 * self.midseg_flow_rate)
        };
        // Exhibit 18-19, row 2: combined bicycle-lane and parking-lane width W_l.
        let w_l = if self.prop_parking_occupied > 0.25 || adjacent_width < 10.0 {
            adjacent_width
        } else {
            10.0
        };
        // Adjusted available sidewalk width and the sidewalk-width coefficient.
        let w_aa = (self.width_sidewalk_ft - self.width_buffer_ft).min(10.0);
        let f_sw = 6.0 - 0.3 * w_aa;
        // Buffer area coefficient (Exhibit 18-19 note).
        let f_b = if self.continuous_barrier { 5.37 } else { 1.0 };

        // Equation 18-33.
        let f_w = -1.2276
            * (w_v
                + 0.5 * w_l
                + 50.0 * self.prop_parking_occupied
                + self.width_buffer_ft * f_b
                + w_aa * f_sw)
                .ln();
        // Equation 18-34.
        let f_v = 0.0091 * self.midseg_flow_rate / (4.0 * self.num_through_lanes);
        // Equation 18-35.
        let f_s = 4.0 * (self.motor_running_speed / 100.0).powi(2);
        // Equation 18-32.
        let link_score = 6.0468 + f_w + f_v + f_s;

        // ── Step 8: roadway crossing difficulty factor ──────────────────────
        let d_c = if self.dist_nearest_crossing_ft > 0.0 {
            self.dist_nearest_crossing_ft
        } else {
            self.length_ft / 3.0
        };
        let d_d = 2.0 * d_c; // Equation 18-36.
        // Equation 18-37: perceived diversion delay.
        let d_pd_los = 0.084 * d_d / walking_speed + self.ped_delay_crossing_signal;
        let i_pd = crossing_delay_los_score(d_pd_los);
        let i_pw = crossing_delay_los_score(self.ped_delay_crossing_uncontrolled);
        // Equation 18-38.
        let crossing_score = i_pw.min(i_pd).min(6.0);

        // ── Step 9: segment pedestrian LOS score (Equation 18-39) ───────────
        let p_mx = self.prop_midblock_crossing;
        let t_walk = self.length_ft / walking_speed;
        let link_term = link_score * (1.0 - p_mx) + crossing_score * p_mx;
        let num = link_term.powi(3) * t_walk
            + self.ped_los_score_intersection.powi(3) * self.ped_delay_parallel;
        let segment_score = (num / (t_walk + self.ped_delay_parallel)).cbrt();

        // ── Step 10: segment LOS (worse of score-based and space-based) ─────
        let segment_los = worse_los(
            pedestrian_score_los(segment_score),
            pedestrian_space_los(pedestrian_space),
        );

        PedestrianAnalysis {
            effective_sidewalk_width: w_e,
            flow_per_width: v_p,
            walking_speed,
            pedestrian_space,
            travel_speed,
            f_w,
            f_v,
            f_s,
            link_score,
            link_los: pedestrian_link_los(link_score),
            crossing_score,
            segment_score,
            segment_los,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// HCM Chapter 30, Example Problem 2: pedestrian LOS on the south sidewalk
    /// of a 1,320-ft collector segment (Exhibit 30-37).
    #[test]
    fn example_problem_2_pedestrian_los() {
        let seg = PedestrianSegment {
            length_ft: 1320.0,
            num_through_lanes: 2.0,
            midseg_flow_rate: 940.0,
            ped_flow_rate: 2000.0,
            prop_parking_occupied: 0.20,
            width_sidewalk_ft: 10.0,
            width_buffer_ft: 5.0,
            fixed_object_width_inside_ft: 0.0,
            fixed_object_width_outside_ft: 0.0,
            prop_window: 0.0,
            prop_building: 0.0,
            prop_fence: 0.50,
            continuous_barrier: false,
            width_outside_lane_ft: 12.0,
            width_bike_lane_ft: 5.0,
            width_outside_shoulder_ft: 0.0,
            width_parking_lane_ft: 9.5,
            curb_present: true,
            motor_running_speed: 33.0,
            free_flow_walk_speed: 4.4,
            ped_delay_parallel: 40.0,
            ped_delay_crossing_signal: 80.0,
            ped_delay_crossing_uncontrolled: 740.0,
            ped_los_score_intersection: 3.60,
            dist_nearest_crossing_ft: 0.0, // uniform crossing -> L/3
            prop_midblock_crossing: 0.35,
        };
        let a = seg.analyze();

        assert!((a.effective_sidewalk_width - 4.25).abs() < 0.01, "W_E {}", a.effective_sidewalk_width);
        assert!((a.flow_per_width - 7.84).abs() < 0.02, "v_p {}", a.flow_per_width);
        assert!((a.walking_speed - 4.19).abs() < 0.02, "S_p {}", a.walking_speed);
        assert!((a.pedestrian_space - 32.0).abs() < 0.3, "A_p {}", a.pedestrian_space);
        assert!((a.travel_speed - 3.72).abs() < 0.02, "S_Tp,seg {}", a.travel_speed);
        assert!((a.f_w - -5.20).abs() < 0.02, "F_w {}", a.f_w);
        assert!((a.f_v - 1.07).abs() < 0.01, "F_v {}", a.f_v);
        assert!((a.f_s - 0.44).abs() < 0.01, "F_s {}", a.f_s);
        assert!((a.link_score - 2.35).abs() < 0.02, "I_p,link {}", a.link_score);
        assert_eq!(a.link_los, LevelOfService::B);
        assert!((a.crossing_score - 6.0).abs() < 1e-9, "I_p,mx {}", a.crossing_score);
        assert!((a.segment_score - 3.62).abs() < 0.03, "I_p,seg {}", a.segment_score);
        assert_eq!(a.segment_los, LevelOfService::D);
    }

    #[test]
    fn crossing_delay_score_curve() {
        assert!((crossing_delay_los_score(0.0) - 0.0).abs() < 1e-9);
        assert!((crossing_delay_los_score(10.0) - 1.5).abs() < 1e-9);
        assert!((crossing_delay_los_score(50.0) - 5.0).abs() < 1e-9);
        assert!((crossing_delay_los_score(70.0) - 6.0).abs() < 1e-9);
        assert!((crossing_delay_los_score(740.0) - 6.0).abs() < 1e-9);
    }
}
