//! Exhibit lookups for HCM Chapter 18 (Urban Street Segments), motorized
//! vehicle methodology.
//!
//! All values transcribed from the HCM 7th Edition EPUB:
//! * Exhibit 18-1 — `127_Ch18_02.xhtml`
//! * Exhibits 18-5, 18-7, 18-11, 18-13, Equations 18-1 through 18-8 —
//!   `128_Ch18_03.xhtml`

use crate::hcm::common::LevelOfService;

// ═══════════════════════════════════════════════════════════════════════════════
// Exhibit 18-1: LOS Criteria: Motorized Vehicle Mode
// ═══════════════════════════════════════════════════════════════════════════════

/// Base free-flow speed column headings of Exhibit 18-1 (mi/h).
const EXHIBIT_18_1_BFFS: [f64; 7] = [55.0, 50.0, 45.0, 40.0, 35.0, 30.0, 25.0];

/// Travel speed thresholds of Exhibit 18-1 (mi/h). Rows are LOS A–E (a
/// travel speed strictly greater than the row value is required for that
/// LOS); columns follow [`EXHIBIT_18_1_BFFS`]. Speeds at or below the LOS E
/// row are LOS F.
///
/// | LOS | 55  | 50  | 45  | 40  | 35  | 30  | 25  |
/// |-----|-----|-----|-----|-----|-----|-----|-----|
/// | A   | >44 | >40 | >36 | >32 | >28 | >24 | >20 |
/// | B   | >37 | >34 | >30 | >27 | >23 | >20 | >17 |
/// | C   | >28 | >25 | >23 | >20 | >18 | >15 | >13 |
/// | D   | >22 | >20 | >18 | >16 | >14 | >12 | >10 |
/// | E   | >17 | >15 | >14 | >12 | >11 | >9  | >8  |
/// | F   | ≤17 | ≤15 | ≤14 | ≤12 | ≤11 | ≤9  | ≤8  |
///
/// A through-movement volume-to-capacity ratio greater than 1.0 is LOS F
/// regardless of travel speed.
const EXHIBIT_18_1_THRESHOLDS: [[f64; 7]; 5] = [
    [44.0, 40.0, 36.0, 32.0, 28.0, 24.0, 20.0], // LOS A
    [37.0, 34.0, 30.0, 27.0, 23.0, 20.0, 17.0], // LOS B
    [28.0, 25.0, 23.0, 20.0, 18.0, 15.0, 13.0], // LOS C
    [22.0, 20.0, 18.0, 16.0, 14.0, 12.0, 10.0], // LOS D
    [17.0, 15.0, 14.0, 12.0, 11.0, 9.0, 8.0],   // LOS E
];

/// Exhibit 18-1 travel speed thresholds `[A, B, C, D, E]` (mi/h) for a
/// given base free-flow speed, linearly interpolated between the column
/// headings as directed by the Chapter 18 text ("The threshold value is
/// interpolated when the base free-flow speed is between the values shown
/// in the column headings"). A base free-flow speed outside the tabulated
/// 25–55 mi/h range is clamped to the nearest column (the exhibit does not
/// define thresholds beyond its headings).
///
/// * `base_ffs_mph` — base free-flow speed S_fo, mi/h
pub fn exhibit_18_1_speed_thresholds(base_ffs_mph: f64) -> [f64; 5] {
    let s = base_ffs_mph.clamp(25.0, 55.0);
    // Columns are in descending BFFS order.
    let mut hi = 0; // index of the column with the larger BFFS
    while hi < 5 && EXHIBIT_18_1_BFFS[hi + 1] > s {
        hi += 1;
    }
    let (b_hi, b_lo) = (EXHIBIT_18_1_BFFS[hi], EXHIBIT_18_1_BFFS[hi + 1]);
    let frac = if (b_hi - b_lo).abs() < f64::EPSILON {
        0.0
    } else {
        (s - b_lo) / (b_hi - b_lo)
    };
    let mut out = [0.0; 5];
    for (i, row) in EXHIBIT_18_1_THRESHOLDS.iter().enumerate() {
        out[i] = row[hi + 1] + frac * (row[hi] - row[hi + 1]);
    }
    out
}

/// HCM Exhibit 18-1: LOS Criteria: Motorized Vehicle Mode.
///
/// * `travel_speed_mph` — through-vehicle travel speed S_T,seg, mi/h
///   (Equation 18-15)
/// * `base_ffs_mph` — base free-flow speed S_fo, mi/h (Equation 18-3)
/// * `vc_gt_1` — true if the through-movement volume-to-capacity ratio at
///   the downstream boundary intersection exceeds 1.0 (forces LOS F)
pub fn exhibit_18_1_los(
    travel_speed_mph: f64,
    base_ffs_mph: f64,
    vc_gt_1: bool,
) -> LevelOfService {
    if vc_gt_1 {
        return LevelOfService::F;
    }
    let t = exhibit_18_1_speed_thresholds(base_ffs_mph);
    match travel_speed_mph {
        s if s > t[0] => LevelOfService::A,
        s if s > t[1] => LevelOfService::B,
        s if s > t[2] => LevelOfService::C,
        s if s > t[3] => LevelOfService::D,
        s if s > t[4] => LevelOfService::E,
        _ => LevelOfService::F,
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Exhibit 18-11: Base Free-Flow Speed Adjustment Factors
// ═══════════════════════════════════════════════════════════════════════════════

/// Speed constant S_0 (mi/h) — Exhibit 18-11, note a:
/// `S_0 = 25.6 + 0.47 S_pl`, where S_pl is the posted speed limit (mi/h).
///
/// The tabulated Exhibit 18-11 values (37.4, 39.7, 42.1, 44.4, 46.8, 49.1,
/// 51.5 for 25–55 mi/h) are this equation rounded to 0.1 mi/h; the
/// methodology (and Chapter 30, Example Problem 1) uses the unrounded
/// equation.
pub fn speed_constant_s0(speed_limit_mph: f64) -> f64 {
    25.6 + 0.47 * speed_limit_mph
}

/// Adjustment for cross section f_CS (mi/h) — Exhibit 18-11, note b:
/// `f_CS = 1.5 p_rm − 0.47 p_curb − 3.7 p_curb p_rm`.
///
/// * `p_rm` — proportion of link length with restrictive median (decimal)
/// * `p_curb` — proportion of segment with curb on the right-hand side,
///   within 4 ft of the traveled way (decimal)
pub fn cross_section_adjustment(p_rm: f64, p_curb: f64) -> f64 {
    1.5 * p_rm - 0.47 * p_curb - 3.7 * p_curb * p_rm
}

/// Access point density D_a (points/mi) — Exhibit 18-11, note c:
/// `D_a = 5,280 (N_ap,s + N_ap,o) / (L − W_i)`.
///
/// * `n_ap_s` — access point approaches on the right side in the subject
///   direction of travel (points)
/// * `n_ap_o` — access point approaches on the right side in the opposing
///   direction of travel (points)
/// * `segment_length_ft` — segment length L, ft
/// * `intersection_width_ft` — width of the upstream signalized
///   intersection W_i, ft
pub fn access_point_density(
    n_ap_s: f64,
    n_ap_o: f64,
    segment_length_ft: f64,
    intersection_width_ft: f64,
) -> f64 {
    let link = segment_length_ft - intersection_width_ft;
    if link <= 0.0 {
        return 0.0;
    }
    5_280.0 * (n_ap_s + n_ap_o) / link
}

/// Adjustment for access points f_A (mi/h) — Exhibit 18-11, note c:
/// `f_A = −0.078 D_a / N_th`.
///
/// * `d_a` — access point density on the segment, points/mi
/// * `n_th` — number of through lanes on the segment in the subject
///   direction of travel (ln)
pub fn access_point_adjustment(d_a: f64, n_th: u32) -> f64 {
    -0.078 * d_a / (n_th.max(1) as f64)
}

/// Adjustment for on-street parking f_pk (mi/h) — Exhibit 18-11, note d:
/// `f_pk = −3.0 ×` proportion of link length with on-street parking
/// available on the right-hand side (decimal).
pub fn parking_adjustment(p_pk: f64) -> f64 {
    -3.0 * p_pk
}

// ═══════════════════════════════════════════════════════════════════════════════
// Exhibit 18-7: Default Access Point Density Values
// ═══════════════════════════════════════════════════════════════════════════════

/// Area type for the Exhibit 18-7 default access point density lookup.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AreaType {
    Urban,
    SuburbanOrRural,
}

/// HCM Exhibit 18-7: Default Access Point Density Values (points/mi) by
/// area type, median type, and speed limit.
///
/// | Area           | Median      | 25 | 30 | 35 | 40 | 45 | 50 | 55 |
/// |----------------|-------------|----|----|----|----|----|----|----|
/// | Urban          | Restrictive | 62 | 50 | 41 | 35 | 30 | 26 | 22 |
/// | Urban          | Other       | 73 | 61 | 52 | 46 | 41 | 37 | 33 |
/// | Suburban/rural | Restrictive | 40 | 27 | 19 | 12 | 7  | 3  | 0  |
/// | Suburban/rural | Other       | 51 | 38 | 30 | 23 | 18 | 14 | 11 |
///
/// * `area` — area type
/// * `restrictive_median` — true for a restrictive median
/// * `speed_limit_mph` — posted speed limit (values between the tabulated
///   5-mi/h headings interpolate linearly; outside 25–55 mi/h returns None)
pub fn exhibit_18_7_default_access_density(
    area: AreaType,
    restrictive_median: bool,
    speed_limit_mph: f64,
) -> Option<f64> {
    if !(25.0..=55.0).contains(&speed_limit_mph) {
        return None;
    }
    let row: [f64; 7] = match (area, restrictive_median) {
        (AreaType::Urban, true) => [62.0, 50.0, 41.0, 35.0, 30.0, 26.0, 22.0],
        (AreaType::Urban, false) => [73.0, 61.0, 52.0, 46.0, 41.0, 37.0, 33.0],
        (AreaType::SuburbanOrRural, true) => [40.0, 27.0, 19.0, 12.0, 7.0, 3.0, 0.0],
        (AreaType::SuburbanOrRural, false) => [51.0, 38.0, 30.0, 23.0, 18.0, 14.0, 11.0],
    };
    let pos = (speed_limit_mph - 25.0) / 5.0;
    let i = (pos.floor() as usize).min(5);
    let frac = pos - i as f64;
    Some(row[i] + frac * (row[i + 1] - row[i]))
}

// ═══════════════════════════════════════════════════════════════════════════════
// Exhibit 18-13: Delay due to Turning Vehicles
// ═══════════════════════════════════════════════════════════════════════════════

/// Through-vehicle delay (s/veh/pt) by number of through lanes — Exhibit
/// 18-13. Rows: midsegment volume 200–700 veh/h/ln in 100-veh/h/ln steps;
/// columns: 1 lane, 2 lanes, 3 lanes.
const EXHIBIT_18_13_DELAY: [[f64; 3]; 6] = [
    [0.04, 0.04, 0.05], // 200 veh/h/ln
    [0.08, 0.08, 0.09], // 300 veh/h/ln
    [0.12, 0.15, 0.15], // 400 veh/h/ln
    [0.18, 0.25, 0.15], // 500 veh/h/ln
    [0.27, 0.41, 0.15], // 600 veh/h/ln
    [0.39, 0.72, 0.15], // 700 veh/h/ln
];

/// HCM Exhibit 18-13: delay to through vehicles due to left and right turns
/// at one access point intersection (s/veh/pt), for planning and
/// preliminary engineering analyses.
///
/// The tabulated values assume 10% left turns and 10% right turns from the
/// segment at the access point (see [`exhibit_18_13_turn_delay_adjusted`]
/// for the turn-percentage and turn-bay adjustments described in the
/// Chapter 18 text).
///
/// * `midsegment_volume_veh_h_ln` — midsegment volume, veh/h/ln (values
///   between the tabulated rows interpolate linearly; the exhibit does not
///   define values outside 200–700 veh/h/ln, so inputs are clamped to the
///   edge rows)
/// * `n_th` — number of through lanes (1, 2, or 3; more than 3 lanes uses
///   the 3-lane column)
pub fn exhibit_18_13_turn_delay(midsegment_volume_veh_h_ln: f64, n_th: u32) -> f64 {
    let col = (n_th.clamp(1, 3) - 1) as usize;
    let v = midsegment_volume_veh_h_ln.clamp(200.0, 700.0);
    let pos = (v - 200.0) / 100.0;
    let i = (pos.floor() as usize).min(4);
    let frac = pos - i as f64;
    EXHIBIT_18_13_DELAY[i][col] + frac * (EXHIBIT_18_13_DELAY[i + 1][col] - EXHIBIT_18_13_DELAY[i][col])
}

/// Exhibit 18-13 delay per access point with the adjustments described in
/// the Chapter 18 text:
///
/// * "The values listed ... represent 10% left turns and 10% right turns
///   ... If the actual turn percentages are less than 10%, the delays can
///   be reduced proportionally" — the value is scaled by
///   `(pct_left + pct_right) / 20`.
/// * "if a turn bay of adequate length is provided for one turn movement
///   but not the other, the values ... should be multiplied by 0.5. If both
///   turn movements are provided a bay of adequate length, the delay ...
///   can be assumed to equal 0.0" s/veh/pt.
///
/// * `midsegment_volume_veh_h_ln` — midsegment volume, veh/h/ln
/// * `n_th` — number of through lanes (ln)
/// * `pct_left`, `pct_right` — percentage of the midsegment volume turning
///   left/right at the access point (%)
/// * `left_bay_adequate`, `right_bay_adequate` — true if a turn bay of
///   adequate length serves the movement
pub fn exhibit_18_13_turn_delay_adjusted(
    midsegment_volume_veh_h_ln: f64,
    n_th: u32,
    pct_left: f64,
    pct_right: f64,
    left_bay_adequate: bool,
    right_bay_adequate: bool,
) -> f64 {
    let bay_factor = match (left_bay_adequate, right_bay_adequate) {
        (true, true) => 0.0,
        (true, false) | (false, true) => 0.5,
        (false, false) => 1.0,
    };
    let turn_factor = ((pct_left + pct_right) / 20.0).max(0.0);
    exhibit_18_13_turn_delay(midsegment_volume_veh_h_ln, n_th) * turn_factor * bay_factor
}

// ═══════════════════════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use LevelOfService as L;

    #[test]
    fn test_speed_constant_matches_exhibit_18_11_column() {
        // Exhibit 18-11 tabulates S_0 rounded to 0.1 mi/h.
        let expected = [
            (25.0, 37.4),
            (30.0, 39.7),
            (35.0, 42.1),
            (40.0, 44.4),
            (45.0, 46.8),
            (50.0, 49.1),
            (55.0, 51.5),
        ];
        for (spl, s0) in expected {
            assert!(
                (speed_constant_s0(spl) - s0).abs() < 0.06,
                "S_0({spl}) = {} vs Exhibit 18-11 {s0}",
                speed_constant_s0(spl)
            );
        }
    }

    #[test]
    fn test_cross_section_adjustment_matches_exhibit_18_11() {
        // Restrictive median rows (no curb / curb) of Exhibit 18-11.
        let rows = [
            (0.2, 0.3, -0.9),
            (0.4, 0.6, -1.4),
            (0.6, 0.9, -1.8),
            (0.8, 1.2, -2.2),
            (1.0, 1.5, -2.7),
        ];
        for (p_rm, no_curb, curb) in rows {
            assert!((cross_section_adjustment(p_rm, 0.0) - no_curb).abs() < 0.05);
            assert!((cross_section_adjustment(p_rm, 1.0) - curb).abs() < 0.05);
        }
        // Nonrestrictive / no median rows: 0.0 without curb, -0.5 with curb.
        assert!((cross_section_adjustment(0.0, 0.0) - 0.0).abs() < 1e-9);
        assert!((cross_section_adjustment(0.0, 1.0) - (-0.47)).abs() < 1e-9); // tabulated -0.5
    }

    #[test]
    fn test_access_point_adjustment_matches_exhibit_18_11() {
        let rows = [
            (2.0, [-0.2, -0.1, -0.1]),
            (4.0, [-0.3, -0.2, -0.1]),
            (10.0, [-0.8, -0.4, -0.3]),
            (20.0, [-1.6, -0.8, -0.5]),
            (40.0, [-3.1, -1.6, -1.0]),
            (60.0, [-4.7, -2.3, -1.6]),
        ];
        for (d_a, by_lanes) in rows {
            for (i, expected) in by_lanes.iter().enumerate() {
                let f_a = access_point_adjustment(d_a, (i + 1) as u32);
                assert!(
                    (f_a - expected).abs() < 0.06,
                    "f_A(D_a={d_a}, N_th={}) = {f_a} vs {expected}",
                    i + 1
                );
            }
        }
    }

    #[test]
    fn test_parking_adjustment_matches_exhibit_18_11() {
        for (pct, f_pk) in [(0.0, 0.0), (0.2, -0.6), (0.4, -1.2), (0.6, -1.8), (0.8, -2.4), (1.0, -3.0)] {
            assert!((parking_adjustment(pct) - f_pk).abs() < 1e-9);
        }
    }

    #[test]
    fn test_exhibit_18_7_defaults() {
        assert_eq!(
            exhibit_18_7_default_access_density(AreaType::Urban, true, 35.0),
            Some(41.0)
        );
        assert_eq!(
            exhibit_18_7_default_access_density(AreaType::Urban, false, 25.0),
            Some(73.0)
        );
        assert_eq!(
            exhibit_18_7_default_access_density(AreaType::SuburbanOrRural, true, 55.0),
            Some(0.0)
        );
        assert_eq!(
            exhibit_18_7_default_access_density(AreaType::SuburbanOrRural, false, 40.0),
            Some(23.0)
        );
        assert_eq!(exhibit_18_7_default_access_density(AreaType::Urban, true, 20.0), None);
    }

    #[test]
    fn test_exhibit_18_13_table_values() {
        let rows = [
            (200.0, [0.04, 0.04, 0.05]),
            (300.0, [0.08, 0.08, 0.09]),
            (400.0, [0.12, 0.15, 0.15]),
            (500.0, [0.18, 0.25, 0.15]),
            (600.0, [0.27, 0.41, 0.15]),
            (700.0, [0.39, 0.72, 0.15]),
        ];
        for (v, by_lanes) in rows {
            for (i, expected) in by_lanes.iter().enumerate() {
                assert!((exhibit_18_13_turn_delay(v, (i + 1) as u32) - expected).abs() < 1e-9);
            }
        }
        // Interpolation halfway between 500 and 600, 2 lanes: 0.33.
        assert!((exhibit_18_13_turn_delay(550.0, 2) - 0.33).abs() < 1e-9);
        // Clamped below/above the table.
        assert!((exhibit_18_13_turn_delay(100.0, 1) - 0.04).abs() < 1e-9);
        assert!((exhibit_18_13_turn_delay(900.0, 1) - 0.39).abs() < 1e-9);
    }

    #[test]
    fn test_exhibit_18_13_adjustments() {
        // 5% + 5% turns halves the tabulated delay (Chapter 18 text example).
        let base = exhibit_18_13_turn_delay(500.0, 2);
        assert!(
            (exhibit_18_13_turn_delay_adjusted(500.0, 2, 5.0, 5.0, false, false) - 0.5 * base)
                .abs()
                < 1e-9
        );
        // One adequate bay halves the delay; two zero it.
        assert!(
            (exhibit_18_13_turn_delay_adjusted(500.0, 2, 10.0, 10.0, true, false) - 0.5 * base)
                .abs()
                < 1e-9
        );
        assert_eq!(
            exhibit_18_13_turn_delay_adjusted(500.0, 2, 10.0, 10.0, true, true),
            0.0
        );
    }

    #[test]
    fn test_exhibit_18_1_thresholds_at_column_headings() {
        let t55 = exhibit_18_1_speed_thresholds(55.0);
        assert_eq!(t55, [44.0, 37.0, 28.0, 22.0, 17.0]);
        let t25 = exhibit_18_1_speed_thresholds(25.0);
        assert_eq!(t25, [20.0, 17.0, 13.0, 10.0, 8.0]);
    }

    /// Chapter 18 text example: "the LOS A threshold for a segment with a
    /// base free-flow speed of 42 mi/h is [33.6] mi/h
    /// [= (42 – 40)/(45 – 40) × (36 – 32) + 32]".
    #[test]
    fn test_exhibit_18_1_interpolation_chapter_text_example() {
        let t = exhibit_18_1_speed_thresholds(42.0);
        assert!((t[0] - 33.6).abs() < 1e-9, "LOS A threshold {}", t[0]);
    }

    /// Chapter 30, Example Problem 1: base FFS 40.78 mi/h interpolates to
    /// thresholds >32.6, >27.5, >20.5, >16.3, and >12.3 mi/h; a travel
    /// speed of 23.67 mi/h is LOS C.
    #[test]
    fn test_exhibit_18_1_example_problem_1() {
        let t = exhibit_18_1_speed_thresholds(40.78);
        let expected = [32.6, 27.5, 20.5, 16.3, 12.3];
        for (i, e) in expected.iter().enumerate() {
            assert!((t[i] - e).abs() < 0.05, "threshold {i}: {} vs {e}", t[i]);
        }
        assert_eq!(exhibit_18_1_los(23.67, 40.78, false), L::C);
    }

    #[test]
    fn test_exhibit_18_1_los_boundaries() {
        // At BFFS 55: A >44, B >37, C >28, D >22, E >17, F <=17.
        assert_eq!(exhibit_18_1_los(44.01, 55.0, false), L::A);
        assert_eq!(exhibit_18_1_los(44.0, 55.0, false), L::B);
        assert_eq!(exhibit_18_1_los(37.0, 55.0, false), L::C);
        assert_eq!(exhibit_18_1_los(28.0, 55.0, false), L::D);
        assert_eq!(exhibit_18_1_los(22.0, 55.0, false), L::E);
        assert_eq!(exhibit_18_1_los(17.0, 55.0, false), L::F);
        // v/c > 1.0 forces LOS F regardless of speed.
        assert_eq!(exhibit_18_1_los(50.0, 55.0, true), L::F);
    }
}
