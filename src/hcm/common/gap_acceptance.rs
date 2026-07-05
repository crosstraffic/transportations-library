//! Gap-acceptance capacity building blocks (HCM Chapter 20 TWSC core),
//! reused by Chapter 22 (roundabouts) and Chapter 23 (ramp terminals).
//!
//! Units: flow rates and capacities in veh/h, headways in s.

/// Pedestrian walking speed assumed by the Chapter 20 pedestrian-impedance
/// procedure (HCM Chapter 20, Equation 20-67 variable definitions:
/// "pedestrian walking speed, assumed to be 3.5 ft/s").
pub const PEDESTRIAN_WALKING_SPEED_FT_S: f64 = 3.5;

/// HCM Equation 20-18: potential capacity of minor movement x
///
/// `c_p,x = v_c,x * exp(-v_c,x t_c,x / 3,600) / (1 - exp(-v_c,x t_f,x / 3,600))`
///
/// * `v_c` — conflicting flow rate for the movement, veh/h
/// * `t_c` — critical headway for the minor movement, s
/// * `t_f` — follow-up headway for the minor movement, s
///
/// Returns potential capacity, veh/h. In the limit `v_c -> 0` the
/// expression tends to `3,600 / t_f` (one entry per follow-up headway),
/// which is returned explicitly for `v_c <= 0`.
pub fn potential_capacity(v_c: f64, t_c: f64, t_f: f64) -> f64 {
    if v_c <= 0.0 {
        return 3_600.0 / t_f;
    }
    v_c * (-v_c * t_c / 3_600.0).exp() / (1.0 - (-v_c * t_f / 3_600.0).exp())
}

/// HCM Equation 20-28: probability that a movement operates in a
/// queue-free state
///
/// `p_0,j = 1 - v_j / c_m,j`
///
/// * `v` — demand flow rate of movement j, veh/h
/// * `c_m` — movement capacity of movement j, veh/h
///
/// The result is clamped to `[0, 1]` (an oversaturated impeding movement is
/// never queue-free). The same form gives the Rank 2 U-turn adjustment
/// factors f_1U and f_4U (HCM Equations 20-24 and 20-25).
pub fn prob_queue_free(v: f64, c_m: f64) -> f64 {
    if c_m <= 0.0 {
        return 0.0;
    }
    (1.0 - v / c_m).clamp(0.0, 1.0)
}

/// HCM Equation 20-35: vehicular capacity adjustment (impedance) factor for
/// Rank 3 (and, combined with additional p_0 terms, Rank 4) movements
///
/// `f_k = Π_j p_0,j`
///
/// * `p0_impeding` — queue-free-state probabilities p_0,j (Equation 20-28)
///   of each impeding higher-rank movement
///
/// Returns the product of the probabilities (1.0 for an empty slice).
pub fn vehicular_impedance_factor(p0_impeding: &[f64]) -> f64 {
    p0_impeding.iter().product()
}

/// HCM Equation 20-67: pedestrian blockage factor
///
/// `f_pb = (v_x * w / S_p) / 3,600`
///
/// * `v_ped` — pedestrian flow rate of the conflicting pedestrian
///   movement, p/h
/// * `lane_width_ft` — width w of the lane the minor movement is
///   negotiating into, ft
/// * `walking_speed_ft_s` — pedestrian walking speed S_p, ft/s
///   ([`PEDESTRIAN_WALKING_SPEED_FT_S`] = 3.5 ft/s assumed by the HCM)
///
/// Returns the proportion of time the lane is blocked by pedestrians.
pub fn pedestrian_blockage_factor(v_ped: f64, lane_width_ft: f64, walking_speed_ft_s: f64) -> f64 {
    (v_ped * lane_width_ft / walking_speed_ft_s) / 3_600.0
}

/// HCM Equation 20-68: pedestrian impedance factor
///
/// `p_p,x = 1 - f_pb`
///
/// * `f_pb` — pedestrian blockage factor (Equation 20-67)
///
/// Clamped to `[0, 1]`.
pub fn pedestrian_impedance_factor(f_pb: f64) -> f64 {
    (1.0 - f_pb).clamp(0.0, 1.0)
}

/// HCM Equations 20-22, 20-26, and 20-36: movement capacity
///
/// `c_m = c_p * f`
///
/// * `c_p` — potential capacity (Equation 20-18), veh/h
/// * `impedance` — combined capacity adjustment factor: the product of the
///   vehicular impedance (Equation 20-35) and any pedestrian impedance
///   factors (Equation 20-68); 1.0 for unimpeded Rank 2 major-street left
///   turns (Equation 20-22)
///
/// Returns movement capacity, veh/h.
pub fn movement_capacity(c_p: f64, impedance: f64) -> f64 {
    c_p * impedance
}

// ═══════════════════════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_potential_capacity_zero_conflicting_limit() {
        // v_c -> 0: c_p -> 3,600 / t_f
        let t_f = 3.3;
        assert!((potential_capacity(0.0, 6.4, t_f) - 3_600.0 / t_f).abs() < 1e-9);
        // Approaching the limit smoothly from v_c > 0
        let near = potential_capacity(1e-6, 6.4, t_f);
        assert!((near - 3_600.0 / t_f).abs() < 0.01);
    }

    #[test]
    fn test_potential_capacity_monotonic_decrease_in_vc() {
        let (t_c, t_f) = (6.4, 3.5);
        let mut prev = f64::INFINITY;
        for i in 0..=20 {
            let v_c = i as f64 * 100.0;
            let c_p = potential_capacity(v_c, t_c, t_f);
            assert!(c_p < prev || i == 0, "c_p not decreasing at v_c={v_c}");
            assert!(c_p > 0.0);
            prev = c_p;
        }
    }

    #[test]
    fn test_potential_capacity_typical_value() {
        // Spot check of Eq 20-18 arithmetic: v_c = 800 veh/h, t_c = 6.4 s,
        // t_f = 3.5 s => c_p = 800 e^(-800*6.4/3600) / (1 - e^(-800*3.5/3600))
        let c_p = potential_capacity(800.0, 6.4, 3.5);
        let expected = 800.0 * (-800.0 * 6.4 / 3_600.0f64).exp()
            / (1.0 - (-800.0 * 3.5 / 3_600.0f64).exp());
        assert!((c_p - expected).abs() < 1e-9);
        // Longer critical headway reduces capacity
        assert!(potential_capacity(800.0, 7.5, 3.5) < c_p);
    }

    #[test]
    fn test_prob_queue_free() {
        // Eq 20-28: p_0 = 1 - v/c_m
        assert!((prob_queue_free(200.0, 800.0) - 0.75).abs() < 1e-12);
        assert_eq!(prob_queue_free(0.0, 800.0), 1.0);
        // Oversaturated impeding movement clamps to 0
        assert_eq!(prob_queue_free(900.0, 800.0), 0.0);
        assert_eq!(prob_queue_free(100.0, 0.0), 0.0);
    }

    #[test]
    fn test_vehicular_impedance_factor() {
        // Eq 20-35: product of queue-free probabilities
        let f = vehicular_impedance_factor(&[0.9, 0.8]);
        assert!((f - 0.72).abs() < 1e-12);
        assert_eq!(vehicular_impedance_factor(&[]), 1.0);
    }

    #[test]
    fn test_pedestrian_impedance() {
        // Eq 20-67: f_pb = (v_x * w/S_p)/3,600
        let f_pb = pedestrian_blockage_factor(100.0, 12.0, PEDESTRIAN_WALKING_SPEED_FT_S);
        let expected = (100.0 * 12.0 / 3.5) / 3_600.0;
        assert!((f_pb - expected).abs() < 1e-12);
        // Eq 20-68: p_p = 1 - f_pb
        assert!((pedestrian_impedance_factor(f_pb) - (1.0 - expected)).abs() < 1e-12);
        // No pedestrians: no impedance
        assert_eq!(
            pedestrian_impedance_factor(pedestrian_blockage_factor(0.0, 12.0, 3.5)),
            1.0
        );
    }

    #[test]
    fn test_movement_capacity_composition() {
        // Eq 20-36 with Eq 20-35 and Eq 20-68 combined
        let c_p = potential_capacity(600.0, 6.5, 4.0);
        let f_veh = vehicular_impedance_factor(&[prob_queue_free(150.0, 900.0)]);
        let p_ped = pedestrian_impedance_factor(pedestrian_blockage_factor(50.0, 12.0, 3.5));
        let c_m = movement_capacity(c_p, f_veh * p_ped);
        assert!(c_m < c_p);
        assert!(c_m > 0.0);
        // Unimpeded Rank 2 (Eq 20-22): c_m = c_p
        assert!((movement_capacity(c_p, 1.0) - c_p).abs() < 1e-12);
    }
}
