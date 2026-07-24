//! HCM Chapter 18, Section 6: Transit methodology for urban street segments.
//!
//! Implements the segment-based transit evaluation: transit vehicle running
//! time (Equations 18-48 through 18-53), travel speed (Equation 18-55), the
//! transit wait-ride score (Equations 18-56 through 18-62), and the segment
//! transit LOS score (Equation 18-63). LOS comes from the Exhibit 18-3 transit
//! thresholds. Reentry delay, the through control delay, and the link
//! pedestrian LOS score are outputs of supporting methodologies.

use serde::{Deserialize, Serialize};
use crate::hcm::common::LevelOfService;
use super::exhibits::segment_los_from_score;

/// Default bus acceleration rate r_at, ft/s^2 (TCQSM).
pub const DEFAULT_ACCEL_RATE: f64 = 3.3;
/// Default bus deceleration rate r_dt, ft/s^2 (TCQSM).
pub const DEFAULT_DECEL_RATE: f64 = 4.0;
/// Default average passenger trip length L_pt, mi.
pub const DEFAULT_PASSENGER_TRIP_LENGTH: f64 = 3.7;
/// Default base travel time rate T_btt outside a large-metropolitan CBD, min/mi.
pub const DEFAULT_BASE_TRAVEL_TIME_RATE: f64 = 4.0;
/// On-time standard: minutes late still considered "on time" (Equation 18-61).
pub const DEFAULT_LATE_THRESHOLD_MIN: f64 = 5.0;
/// Calibration coefficient `e` in the perceived travel time factor
/// (Equation 18-57). This is a fitted constant, not Euler's number.
pub const FTT_COEFFICIENT: f64 = -0.40;

/// Inputs for the HCM Chapter 18 transit segment evaluation (one route, one
/// direction).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct TransitSegment {
    /// Segment length L, ft.
    pub length_ft: f64,
    /// Number of transit stops on the segment for the route N_ts.
    pub num_transit_stops: f64,
    /// Motorized vehicle running speed S_R, mi/h.
    pub motor_running_speed: f64,
    /// Average dwell time t_d, s.
    pub dwell_time_s: f64,
    /// Transit frequency (route service frequency) v_s, veh/h.
    pub transit_frequency: f64,
    /// Effective green-to-cycle ratio g/C at the downstream boundary
    /// intersection (used for f_ad and f_dt at a near-side signalized stop).
    pub g_c_ratio: f64,
    /// The stop is near-side at a signalized boundary intersection (so
    /// f_ad = f_dt = g/C); otherwise f_ad = f_dt = 1.0.
    pub near_side_signalized_stop: bool,
    /// Bus acceleration rate r_at, ft/s^2.
    pub accel_rate: f64,
    /// Bus deceleration rate r_dt, ft/s^2.
    pub decel_rate: f64,
    /// Reentry delay d_re, s (0 for an on-line stop).
    pub reentry_delay_s: f64,
    /// Through vehicle control delay at the boundary intersection d_t, s/veh.
    pub through_delay_s: f64,
    /// Passenger load factor F_l, passengers/seat.
    pub passenger_load_factor: f64,
    /// Proportion of stops with a shelter p_sh.
    pub prop_stops_shelter: f64,
    /// Proportion of stops with a bench (no shelter) p_be.
    pub prop_stops_bench: f64,
    /// Average passenger trip length L_pt, mi.
    pub passenger_trip_length: f64,
    /// On-time performance p_ot (decimal); used to estimate excess wait time.
    pub on_time_performance: f64,
    /// On-time standard t_late, min (minutes late still counted "on time").
    pub late_threshold_min: f64,
    /// Base travel time rate T_btt, min/mi (4.0 outside a large-metro CBD).
    pub base_travel_time_rate: f64,
    /// Pedestrian LOS score for the link I_p,link (Chapter 18 pedestrian output).
    pub ped_los_score_link: f64,
}

impl Default for TransitSegment {
    fn default() -> Self {
        Self {
            length_ft: 1320.0,
            num_transit_stops: 1.0,
            motor_running_speed: 30.0,
            dwell_time_s: 20.0,
            transit_frequency: 4.0,
            g_c_ratio: 0.45,
            near_side_signalized_stop: true,
            accel_rate: DEFAULT_ACCEL_RATE,
            decel_rate: DEFAULT_DECEL_RATE,
            reentry_delay_s: 0.0,
            through_delay_s: 0.0,
            passenger_load_factor: 0.0,
            prop_stops_shelter: 0.0,
            prop_stops_bench: 0.0,
            passenger_trip_length: DEFAULT_PASSENGER_TRIP_LENGTH,
            on_time_performance: 1.0,
            late_threshold_min: DEFAULT_LATE_THRESHOLD_MIN,
            base_travel_time_rate: DEFAULT_BASE_TRAVEL_TIME_RATE,
            ped_los_score_link: 0.0,
        }
    }
}

/// Result of a transit segment evaluation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitAnalysis {
    /// Transit vehicle segment running speed S_Rt, mi/h (Equation 18-48).
    pub running_speed: f64,
    /// Bus stop delay due to acceleration/deceleration d_ad, s (Equation 18-49).
    pub delay_accel_decel: f64,
    /// Bus stop delay due to serving passengers d_ps, s (Equation 18-51).
    pub delay_passenger_service: f64,
    /// Total delay due to the transit stop d_ts, s (Equation 18-52).
    pub delay_stop: f64,
    /// Transit vehicle running time t_Rt, s (Equation 18-53).
    pub running_time: f64,
    /// Transit travel speed S_Tt,seg, mi/h (Equation 18-55).
    pub travel_speed: f64,
    /// Headway factor F_h (Equation 18-56).
    pub headway_factor: f64,
    /// Perceived travel time rate T_ptt, min/mi (Equation 18-58).
    pub perceived_travel_time_rate: f64,
    /// Perceived travel time factor F_tt (Equation 18-57).
    pub travel_time_factor: f64,
    /// Transit wait-ride score s_w-r (Equation 18-62).
    pub wait_ride_score: f64,
    /// Segment transit LOS score I_t,seg (Equation 18-63).
    pub segment_score: f64,
    /// Segment transit LOS (Exhibit 18-3).
    pub segment_los: LevelOfService,
}

impl TransitSegment {
    pub fn new() -> Self {
        Self::default()
    }

    /// Full transit segment evaluation (Steps 1-7).
    pub fn analyze(&self) -> TransitAnalysis {
        let l = self.length_ft;

        // ── Step 1: transit vehicle running time ────────────────────────────
        // Equation 18-48: running speed.
        let running_speed = self.motor_running_speed.min(
            61.0 / (1.0 + (-1.00 + 1185.0 * self.num_transit_stops / l).exp()),
        );
        // f_ad and f_dt: g/C at a near-side signalized stop, else 1.0.
        let f_ad = if self.near_side_signalized_stop {
            self.g_c_ratio
        } else {
            1.0
        };
        let f_dt = f_ad;
        // Equation 18-49: acceleration-deceleration delay.
        let delay_accel_decel = (5280.0 / 3600.0)
            * (running_speed / 2.0)
            * (1.0 / self.accel_rate + 1.0 / self.decel_rate)
            * f_ad;
        // Equation 18-51: passenger service delay.
        let delay_passenger_service = self.dwell_time_s * f_dt;
        // Equation 18-52: total stop delay (single stop shown; N_ts identical stops).
        let delay_stop = delay_accel_decel + delay_passenger_service + self.reentry_delay_s;
        // Equation 18-53: running time (sum of per-stop delays).
        let running_time =
            3600.0 * l / (5280.0 * running_speed) + self.num_transit_stops * delay_stop;

        // ── Step 3: travel speed (Equation 18-55) ───────────────────────────
        let travel_speed = 3600.0 * l / (5280.0 * (running_time + self.through_delay_s));

        // ── Step 4: transit wait-ride score ─────────────────────────────────
        // Equation 18-56: headway factor.
        let headway_factor = 4.00 * (-1.434 / (self.transit_frequency + 0.001)).exp();
        // Equation 18-60: amenity time rate.
        let t_at = (1.3 * self.prop_stops_shelter + 0.2 * self.prop_stops_bench)
            / self.passenger_trip_length;
        // Equation 18-61: excess wait time -> rate.
        let t_ex_min = (self.late_threshold_min * (1.0 - self.on_time_performance)).powi(2);
        let t_ex = t_ex_min / self.passenger_trip_length;
        // Equation 18-59: passenger load waiting factor.
        let a1 = 1.0 + 4.0 * (self.passenger_load_factor - 0.80) / 4.2;
        // Equation 18-58: perceived travel time rate.
        let perceived_travel_time_rate = a1 * (60.0 / travel_speed) + 2.0 * t_ex - t_at;
        // Equation 18-57: perceived travel time factor.
        let e = FTT_COEFFICIENT;
        let t_btt = self.base_travel_time_rate;
        let t_ptt = perceived_travel_time_rate;
        let travel_time_factor =
            ((e - 1.0) * t_btt - (e + 1.0) * t_ptt) / ((e - 1.0) * t_ptt - (e + 1.0) * t_btt);
        // Equation 18-62: wait-ride score.
        let wait_ride_score = headway_factor * travel_time_factor;

        // ── Step 6: segment transit LOS score (Equation 18-63) ──────────────
        let segment_score = 6.0 - 1.50 * wait_ride_score + 0.15 * self.ped_los_score_link;

        TransitAnalysis {
            running_speed,
            delay_accel_decel,
            delay_passenger_service,
            delay_stop,
            running_time,
            travel_speed,
            headway_factor,
            perceived_travel_time_rate,
            travel_time_factor,
            wait_ride_score,
            segment_score,
            segment_los: segment_los_from_score(segment_score),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// HCM Chapter 30, Example Problem 4: transit LOS for an eastbound bus route
    /// on a 1,320-ft collector segment with one near-side, off-line stop
    /// (Exhibit 30-39).
    #[test]
    fn example_problem_4_transit_los() {
        let seg = TransitSegment {
            length_ft: 1320.0,
            num_transit_stops: 1.0,
            motor_running_speed: 33.0,
            dwell_time_s: 20.0,
            transit_frequency: 4.0,
            g_c_ratio: 0.4729,
            near_side_signalized_stop: true,
            accel_rate: 3.3,
            decel_rate: 4.0,
            reentry_delay_s: 16.17,
            through_delay_s: 19.4,
            passenger_load_factor: 0.83,
            prop_stops_shelter: 0.0,
            prop_stops_bench: 1.0,
            passenger_trip_length: 3.7,
            on_time_performance: 0.92,
            late_threshold_min: 5.0,
            base_travel_time_rate: 4.0,
            ped_los_score_link: 3.53,
        };
        let a = seg.analyze();

        assert!((a.running_speed - 32.1).abs() < 0.1, "S_Rt {}", a.running_speed);
        assert!((a.delay_accel_decel - 6.15).abs() < 0.05, "d_ad {}", a.delay_accel_decel);
        assert!((a.delay_passenger_service - 9.46).abs() < 0.02, "d_ps {}", a.delay_passenger_service);
        assert!((a.delay_stop - 31.78).abs() < 0.1, "d_ts {}", a.delay_stop);
        assert!((a.running_time - 59.9).abs() < 0.2, "t_Rt {}", a.running_time);
        assert!((a.travel_speed - 11.3).abs() < 0.1, "S_Tt,seg {}", a.travel_speed);
        assert!((a.headway_factor - 2.80).abs() < 0.02, "F_h {}", a.headway_factor);
        assert!((a.perceived_travel_time_rate - 5.50).abs() < 0.05, "T_ptt {}", a.perceived_travel_time_rate);
        assert!((a.travel_time_factor - 0.881).abs() < 0.005, "F_tt {}", a.travel_time_factor);
        assert!((a.wait_ride_score - 2.47).abs() < 0.02, "s_w-r {}", a.wait_ride_score);
        assert!((a.segment_score - 2.83).abs() < 0.03, "I_t,seg {}", a.segment_score);
        assert_eq!(a.segment_los, LevelOfService::C);
    }

    #[test]
    fn los_thresholds() {
        assert_eq!(segment_los_from_score(2.00), LevelOfService::A);
        assert_eq!(segment_los_from_score(2.83), LevelOfService::C);
        assert_eq!(segment_los_from_score(5.01), LevelOfService::F);
    }
}
