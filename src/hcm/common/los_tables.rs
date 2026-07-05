//! Level-of-service threshold lookups transcribed from HCM 7th Edition
//! exhibits (Chapters 12, 13, 14, 19, 20, 21, and 22).
//!
//! All delay thresholds are in s/veh; all density thresholds are in
//! pc/mi/ln.

use super::LevelOfService;

/// HCM Exhibit 19-8: LOS Criteria: Motorized Vehicle Mode (signalized
/// intersections).
///
/// | Control delay (s/veh) | v/c <= 1.0 | v/c > 1.0 |
/// |-----------------------|------------|-----------|
/// | <=10                  | A          | F         |
/// | >10–20                | B          | F         |
/// | >20–35                | C          | F         |
/// | >35–55                | D          | F         |
/// | >55–80                | E          | F         |
/// | >80                   | F          | F         |
///
/// * `control_delay_s` — control delay, s/veh
/// * `vc_gt_1` — true if the lane group volume-to-capacity ratio exceeds 1.0
///
/// Note (Exhibit 19-8): for approach-based and intersectionwide
/// assessments, LOS is defined solely by control delay (pass
/// `vc_gt_1 = false`).
pub fn los_signalized_intersection(control_delay_s: f64, vc_gt_1: bool) -> LevelOfService {
    if vc_gt_1 {
        return LevelOfService::F;
    }
    match control_delay_s {
        d if d <= 10.0 => LevelOfService::A,
        d if d <= 20.0 => LevelOfService::B,
        d if d <= 35.0 => LevelOfService::C,
        d if d <= 55.0 => LevelOfService::D,
        d if d <= 80.0 => LevelOfService::E,
        _ => LevelOfService::F,
    }
}

/// HCM Exhibits 20-2 (TWSC), 21-8 (AWSC), and 22-8 (roundabouts):
/// LOS Criteria: Motorized Vehicle Mode. All three exhibits share the same
/// thresholds:
///
/// | Control delay (s/veh) | v/c <= 1.0 | v/c > 1.0 |
/// |-----------------------|------------|-----------|
/// | 0–10                  | A          | F         |
/// | >10–15                | B          | F         |
/// | >15–25                | C          | F         |
/// | >25–35                | D          | F         |
/// | >35–50                | E          | F         |
/// | >50                   | F          | F         |
///
/// * `control_delay_s` — control delay, s/veh
/// * `vc_gt_1` — true if the movement/lane volume-to-capacity ratio
///   exceeds 1.0 (LOS F is assigned regardless of delay)
///
/// Notes: Exhibit 20-2 applies per lane/movement on the minor street and to
/// major-street left turns (LOS is not defined for a TWSC intersection as a
/// whole); Exhibits 21-8 and 22-8 define approach/intersection LOS solely by
/// control delay (pass `vc_gt_1 = false`).
pub fn los_unsignalized(control_delay_s: f64, vc_gt_1: bool) -> LevelOfService {
    if vc_gt_1 {
        return LevelOfService::F;
    }
    match control_delay_s {
        d if d <= 10.0 => LevelOfService::A,
        d if d <= 15.0 => LevelOfService::B,
        d if d <= 25.0 => LevelOfService::C,
        d if d <= 35.0 => LevelOfService::D,
        d if d <= 50.0 => LevelOfService::E,
        _ => LevelOfService::F,
    }
}

/// HCM Exhibit 12-15: LOS Criteria for Basic Freeway and Multilane Highway
/// Segments.
///
/// | LOS | Density (pc/mi/ln)                        |
/// |-----|-------------------------------------------|
/// | A   | <=11                                      |
/// | B   | >11–18                                    |
/// | C   | >18–26                                    |
/// | D   | >26–35                                    |
/// | E   | >35–45                                    |
/// | F   | demand exceeds capacity OR density > 45   |
///
/// * `density_pc_mi_ln` — segment density, pc/mi/ln
/// * `demand_exceeds_capacity` — true if the demand flow rate exceeds
///   segment capacity (forces LOS F)
///
/// Per the Chapter 12 text, the density boundaries for multilane highways
/// are the same as for basic freeway segments; use this function for both
/// (see also [`los_multilane`]).
pub fn los_basic_freeway(density_pc_mi_ln: f64, demand_exceeds_capacity: bool) -> LevelOfService {
    if demand_exceeds_capacity {
        return LevelOfService::F;
    }
    match density_pc_mi_ln {
        d if d <= 11.0 => LevelOfService::A,
        d if d <= 18.0 => LevelOfService::B,
        d if d <= 26.0 => LevelOfService::C,
        d if d <= 35.0 => LevelOfService::D,
        d if d <= 45.0 => LevelOfService::E,
        _ => LevelOfService::F,
    }
}

/// HCM Exhibit 12-15: multilane highway segment LOS. Identical thresholds
/// to basic freeway segments ("For all levels of service, the density
/// boundaries on basic freeway segments are the same as those for multilane
/// highways", HCM Chapter 12).
pub fn los_multilane(density_pc_mi_ln: f64, demand_exceeds_capacity: bool) -> LevelOfService {
    los_basic_freeway(density_pc_mi_ln, demand_exceeds_capacity)
}

/// Facility family for the weaving-segment LOS lookup (HCM Exhibit 13-6
/// distinguishes freeway weaving segments from weaving segments on
/// multilane highways or collector–distributor roads).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeavingFacilityType {
    /// Freeway weaving segment
    Freeway,
    /// Weaving segment on a multilane highway or C-D road
    MultilaneOrCDRoad,
}

/// HCM Exhibit 13-6: LOS for Weaving Segments.
///
/// | LOS | Freeway (pc/mi/ln)          | Multilane/C-D road (pc/mi/ln)  |
/// |-----|-----------------------------|--------------------------------|
/// | A   | 0–10                        | 0–12                           |
/// | B   | >10–20                      | >12–24                         |
/// | C   | >20–28                      | >24–32                         |
/// | D   | >28–35                      | >32–36                         |
/// | E   | >35–43                      | >36–40                         |
/// | F   | >43, or demand > capacity   | >40, or demand > capacity      |
///
/// * `density_pc_mi_ln` — weaving segment density, pc/mi/ln
/// * `demand_exceeds_capacity` — true if demand flow rate exceeds capacity
/// * `facility` — freeway vs. multilane/C-D thresholds
pub fn los_weaving(
    density_pc_mi_ln: f64,
    demand_exceeds_capacity: bool,
    facility: WeavingFacilityType,
) -> LevelOfService {
    if demand_exceeds_capacity {
        return LevelOfService::F;
    }
    let (a, b, c, d, e) = match facility {
        WeavingFacilityType::Freeway => (10.0, 20.0, 28.0, 35.0, 43.0),
        WeavingFacilityType::MultilaneOrCDRoad => (12.0, 24.0, 32.0, 36.0, 40.0),
    };
    match density_pc_mi_ln {
        x if x <= a => LevelOfService::A,
        x if x <= b => LevelOfService::B,
        x if x <= c => LevelOfService::C,
        x if x <= d => LevelOfService::D,
        x if x <= e => LevelOfService::E,
        _ => LevelOfService::F,
    }
}

/// HCM Exhibit 14-3: LOS Criteria for Freeway Merge and Diverge Segments.
///
/// | LOS | Density (pc/mi/ln)        |
/// |-----|---------------------------|
/// | A   | <=10                      |
/// | B   | >10–20                    |
/// | C   | >20–28                    |
/// | D   | >28–35                    |
/// | E   | >35                       |
/// | F   | demand exceeds capacity   |
///
/// * `density_pc_mi_ln` — ramp influence area density, pc/mi/ln
/// * `demand_exceeds_capacity` — true if demand flow rate exceeds capacity
///
/// Note: unlike Exhibits 12-15 and 13-6, Exhibit 14-3 assigns LOS F only
/// when demand exceeds capacity; density above 35 pc/mi/ln alone is LOS E.
pub fn los_merge_diverge(
    density_pc_mi_ln: f64,
    demand_exceeds_capacity: bool,
) -> LevelOfService {
    if demand_exceeds_capacity {
        return LevelOfService::F;
    }
    match density_pc_mi_ln {
        d if d <= 10.0 => LevelOfService::A,
        d if d <= 20.0 => LevelOfService::B,
        d if d <= 28.0 => LevelOfService::C,
        d if d <= 35.0 => LevelOfService::D,
        _ => LevelOfService::E,
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use LevelOfService as L;

    #[test]
    fn test_los_signalized_boundaries_exhibit_19_8() {
        assert_eq!(los_signalized_intersection(10.0, false), L::A);
        assert_eq!(los_signalized_intersection(10.01, false), L::B);
        assert_eq!(los_signalized_intersection(20.0, false), L::B);
        assert_eq!(los_signalized_intersection(20.01, false), L::C);
        assert_eq!(los_signalized_intersection(35.0, false), L::C);
        assert_eq!(los_signalized_intersection(35.01, false), L::D);
        assert_eq!(los_signalized_intersection(55.0, false), L::D);
        assert_eq!(los_signalized_intersection(55.01, false), L::E);
        assert_eq!(los_signalized_intersection(80.0, false), L::E);
        assert_eq!(los_signalized_intersection(80.01, false), L::F);
        // v/c > 1.0 forces F regardless of delay
        assert_eq!(los_signalized_intersection(5.0, true), L::F);
    }

    #[test]
    fn test_los_unsignalized_boundaries_exhibits_20_2_21_8_22_8() {
        assert_eq!(los_unsignalized(10.0, false), L::A);
        assert_eq!(los_unsignalized(10.01, false), L::B);
        assert_eq!(los_unsignalized(15.0, false), L::B);
        assert_eq!(los_unsignalized(15.01, false), L::C);
        assert_eq!(los_unsignalized(25.0, false), L::C);
        assert_eq!(los_unsignalized(25.01, false), L::D);
        assert_eq!(los_unsignalized(35.0, false), L::D);
        assert_eq!(los_unsignalized(35.01, false), L::E);
        assert_eq!(los_unsignalized(50.0, false), L::E);
        assert_eq!(los_unsignalized(50.01, false), L::F);
        assert_eq!(los_unsignalized(2.0, true), L::F);
    }

    #[test]
    fn test_los_basic_freeway_boundaries_exhibit_12_15() {
        assert_eq!(los_basic_freeway(11.0, false), L::A);
        assert_eq!(los_basic_freeway(11.01, false), L::B);
        assert_eq!(los_basic_freeway(18.0, false), L::B);
        assert_eq!(los_basic_freeway(26.0, false), L::C);
        assert_eq!(los_basic_freeway(35.0, false), L::D);
        assert_eq!(los_basic_freeway(45.0, false), L::E);
        assert_eq!(los_basic_freeway(45.01, false), L::F);
        assert_eq!(los_basic_freeway(20.0, true), L::F);
        // Multilane uses identical thresholds (Exhibit 12-15)
        for d in [5.0, 15.0, 22.0, 30.0, 40.0, 50.0] {
            assert_eq!(los_multilane(d, false), los_basic_freeway(d, false));
        }
    }

    #[test]
    fn test_los_weaving_boundaries_exhibit_13_6() {
        use WeavingFacilityType::*;
        assert_eq!(los_weaving(10.0, false, Freeway), L::A);
        assert_eq!(los_weaving(10.01, false, Freeway), L::B);
        assert_eq!(los_weaving(20.0, false, Freeway), L::B);
        assert_eq!(los_weaving(28.0, false, Freeway), L::C);
        assert_eq!(los_weaving(35.0, false, Freeway), L::D);
        assert_eq!(los_weaving(43.0, false, Freeway), L::E);
        assert_eq!(los_weaving(43.01, false, Freeway), L::F);
        assert_eq!(los_weaving(5.0, true, Freeway), L::F);

        assert_eq!(los_weaving(12.0, false, MultilaneOrCDRoad), L::A);
        assert_eq!(los_weaving(12.01, false, MultilaneOrCDRoad), L::B);
        assert_eq!(los_weaving(24.0, false, MultilaneOrCDRoad), L::B);
        assert_eq!(los_weaving(32.0, false, MultilaneOrCDRoad), L::C);
        assert_eq!(los_weaving(36.0, false, MultilaneOrCDRoad), L::D);
        assert_eq!(los_weaving(40.0, false, MultilaneOrCDRoad), L::E);
        assert_eq!(los_weaving(40.01, false, MultilaneOrCDRoad), L::F);
    }

    #[test]
    fn test_los_merge_diverge_boundaries_exhibit_14_3() {
        assert_eq!(los_merge_diverge(10.0, false), L::A);
        assert_eq!(los_merge_diverge(10.01, false), L::B);
        assert_eq!(los_merge_diverge(20.0, false), L::B);
        assert_eq!(los_merge_diverge(28.0, false), L::C);
        assert_eq!(los_merge_diverge(35.0, false), L::D);
        // Exhibit 14-3: density > 35 alone is LOS E; F only when demand
        // exceeds capacity
        assert_eq!(los_merge_diverge(50.0, false), L::E);
        assert_eq!(los_merge_diverge(15.0, true), L::F);
    }
}
