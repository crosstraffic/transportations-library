//! HCM Chapter 10 (Freeway Facilities Core Methodology) exhibit lookups and
//! equation transcriptions, plus the Chapter 25 (Freeway Facilities:
//! Supplemental) constants shared by the undersaturated and oversaturated
//! engines.
//!
//! Sources (HCM 7th Edition EPUB):
//! - Exhibit 10-6 (LOS criteria, urban/rural facilities): `68_Ch10_02.xhtml`
//! - Equations 10-1 through 10-6: `68_Ch10_02.xhtml` / `69_Ch10_03.xhtml`
//! - Equations 10-7 through 10-12, Exhibit 10-15 (work zones): `70_Ch10_04.xhtml`
//! - Equation 25-1 (maximum achievable speed): `194_Ch25_03.xhtml`

use serde::{Deserialize, Serialize};

use crate::hcm::common::{CityType, LevelOfService};

// ═════════════════════════════════════════════════════════════════════════
// Global parameter defaults (Chapter 10, Step A-6; Exhibit 10-7)
// ═════════════════════════════════════════════════════════════════════════

/// Default jam density, pc/mi/ln — Chapter 10, Step A-6 / Exhibit 10-7
/// (range 150–270 pc/mi/ln).
pub const DEFAULT_JAM_DENSITY_PC: f64 = 190.0;

/// Default queue discharge capacity drop, decimal — Chapter 10, Step A-6 /
/// Exhibit 10-7 (7%, range 0%–20%).
pub const DEFAULT_QUEUE_DISCHARGE_DROP: f64 = 0.07;

/// Density at capacity, pc/mi/ln — Chapter 12, Exhibit 12-6 (capacity is
/// defined as the flow at a density of 45 pc/mi/ln; also used to anchor the
/// congested linear flow–density branch of Exhibit 25-2).
pub const DENSITY_AT_CAPACITY_PC: f64 = 45.0;

/// Default average queue discharge capacity drop inside freeway work zones,
/// decimal — Chapter 10, Work Zone Analysis (13.4%, from NCHRP 03-107).
pub const DEFAULT_WORK_ZONE_DISCHARGE_DROP: f64 = 0.134;

/// Oversaturated-engine time step, s — Chapter 25, Procedure Parameters
/// ("The computational engine assumes a time step of 15 s ... adequate for
/// most facilities with a minimum segment length greater than 300 ft").
pub const DEFAULT_TIME_STEP_S: f64 = 15.0;

/// Minimum segment length for which the 15-s time step is adequate, ft —
/// Chapter 25, Procedure Parameters.
pub const MIN_SEGMENT_LENGTH_FT: f64 = 300.0;

// ═════════════════════════════════════════════════════════════════════════
// Exhibit 10-6: LOS criteria for urban and rural freeway facilities
// ═════════════════════════════════════════════════════════════════════════

/// HCM Exhibit 10-6: LOS Criteria for Urban and Rural Freeway Facilities.
///
/// | LOS | Urban density (pc/mi/ln) | Rural density (pc/mi/ln) |
/// |-----|--------------------------|--------------------------|
/// | A   | <=11                     | <=6                      |
/// | B   | >11–18                   | >6–14                    |
/// | C   | >18–26                   | >14–22                   |
/// | D   | >26–35                   | >22–29                   |
/// | E   | >35–45                   | >29–39                   |
/// | F   | >45                      | >39                      |
///
/// LOS F also applies when **any** component segment has a
/// demand-to-capacity ratio vd/c > 1.00 (both area types).
///
/// * `density_pc_mi_ln` — facility average density (Equation 10-1), pc/mi/ln
/// * `any_segment_demand_exceeds_capacity` — true if any component segment
///   has vd/c > 1.00 in the analysis period
/// * `city_type` — urban or rural facility classification
pub fn los_freeway_facility(
    density_pc_mi_ln: f64,
    any_segment_demand_exceeds_capacity: bool,
    city_type: CityType,
) -> LevelOfService {
    if any_segment_demand_exceeds_capacity {
        return LevelOfService::F;
    }
    let (a, b, c, d, e) = match city_type {
        CityType::Urban => (11.0, 18.0, 26.0, 35.0, 45.0),
        CityType::Rural => (6.0, 14.0, 22.0, 29.0, 39.0),
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

// ═════════════════════════════════════════════════════════════════════════
// Equations 10-1 through 10-6
// ═════════════════════════════════════════════════════════════════════════

/// Equation 10-1: average facility density for one analysis period,
/// weighted by segment length and number of lanes:
///
/// `D_F = Σ(D_i × L_i × N_i) / Σ(L_i × N_i)`
///
/// * `density` — segment densities D_i (any consistent length unit for L)
/// * `length` — segment lengths L_i
/// * `lanes` — segment lane counts N_i
pub fn facility_density(density: &[f64], length: &[f64], lanes: &[f64]) -> f64 {
    let num: f64 = density
        .iter()
        .zip(length)
        .zip(lanes)
        .map(|((d, l), n)| d * l * n)
        .sum();
    let den: f64 = length.iter().zip(lanes).map(|(l, n)| l * n).sum();
    if den > 0.0 {
        num / den
    } else {
        0.0
    }
}

/// Equation 25-2 (referenced by Chapter 10 Step A-15): facility space mean
/// speed in one analysis period:
///
/// `SMS = Σ(SF_i × L_i) / Σ(SF_i × L_i / U_i)`
///
/// * `flow` — segment flows SF_i, veh/h
/// * `length` — segment lengths L_i (any consistent unit)
/// * `speed` — segment space mean speeds U_i, mi/h
pub fn facility_space_mean_speed(flow: &[f64], length: &[f64], speed: &[f64]) -> f64 {
    let num: f64 = flow.iter().zip(length).map(|(f, l)| f * l).sum();
    let den: f64 = flow
        .iter()
        .zip(length)
        .zip(speed)
        .filter(|(_, s)| **s > 0.0)
        .map(|((f, l), s)| f * l / s)
        .sum();
    if den > 0.0 {
        num / den
    } else {
        0.0
    }
}

/// Equation 10-2: time interval scale factor for analysis period i —
/// the ratio of total facility entering counts to total exiting counts:
///
/// `f_TIS,i = Σ_j VON15_ij / Σ_j VOFF15_ij`
pub fn time_interval_scale_factor(entering_counts: &[f64], exiting_counts: &[f64]) -> f64 {
    let on: f64 = entering_counts.iter().sum();
    let off: f64 = exiting_counts.iter().sum();
    if off > 0.0 {
        on / off
    } else {
        1.0
    }
}

/// Equation 10-3: adjusted 15-min exit demands
/// `VdOFF15_ij = VOFF15_ij × f_TIS,i` (demand balancing, Step A-4).
pub fn balance_exit_demands(entering_counts: &[f64], exiting_counts: &[f64]) -> Vec<f64> {
    let f_tis = time_interval_scale_factor(entering_counts, exiting_counts);
    exiting_counts.iter().map(|v| v * f_tis).collect()
}

/// Equation 10-4: adjusted free-flow speed `FFS_adj = FFS × SAF_cal`.
pub fn adjusted_ffs(ffs: f64, saf: f64) -> f64 {
    ffs * saf
}

/// Equation 10-5: adjusted capacity `c_adj = c × CAF_cal`.
pub fn adjusted_capacity(capacity: f64, caf: f64) -> f64 {
    capacity * caf
}

/// Equation 10-6: adjusted demand `v_adj = v × DAF_cal`.
pub fn adjusted_demand(demand: f64, daf: f64) -> f64 {
    demand * daf
}

// ═════════════════════════════════════════════════════════════════════════
// Equation 25-1: maximum achievable speed constraint
// ═════════════════════════════════════════════════════════════════════════

/// Equation 25-1: maximum achievable segment speed given a low speed on the
/// immediately upstream segment:
///
/// `V_max = FFS − (FFS − V_prev) × e^(−0.00162 × L)`
///
/// * `ffs` — subject segment free-flow speed, mi/h
/// * `v_prev` — average speed on the immediate upstream segment, mi/h
/// * `distance_ft` — distance between the midpoints of the upstream segment
///   and the subject segment, ft
pub fn max_achievable_speed(ffs: f64, v_prev: f64, distance_ft: f64) -> f64 {
    ffs - (ffs - v_prev) * (-0.00162 * distance_ft).exp()
}

// ═════════════════════════════════════════════════════════════════════════
// Work zones: Equations 10-7 through 10-12, Exhibit 10-15
// ═════════════════════════════════════════════════════════════════════════

/// Work zone description used to derive CAF/SAF per HCM Chapter 10,
/// Section 4 (Work Zone Analysis; NCHRP 03-107 models).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct WorkZone {
    /// Total (normal) number of lanes upstream of the work zone, ln.
    pub total_lanes: u32,
    /// Number of open lanes through the work zone, ln.
    pub open_lanes: u32,
    /// Barrier type: `true` = cone/plastic drum or other soft barrier
    /// (f_Br = 1), `false` = concrete/hard barrier (f_Br = 0).
    pub soft_barrier: bool,
    /// Area type: `true` = rural (f_AT = 1), `false` = urban (f_AT = 0).
    pub rural: bool,
    /// Lateral distance from the edge of the travel lane adjacent to the
    /// work zone to the barrier, barricades, or cones, ft (0–12).
    pub lateral_distance_ft: f64,
    /// Time of day: `true` = night (f_DN = 1), `false` = daylight (f_DN = 0).
    pub night: bool,
    /// Non–work zone speed limit divided by work zone speed limit, decimal
    /// (capped to 1.0–1.2 per Equation 10-10).
    pub speed_ratio: f64,
    /// Work zone regulatory speed limit, mi/h.
    pub speed_limit_mi_h: f64,
    /// Total ramp density along the facility, ramps/mi.
    pub total_ramp_density: f64,
    /// Percentage drop in prebreakdown capacity at the work zone due to
    /// queuing, decimal (alpha_wz; default 0.134 per NCHRP 03-107).
    pub queue_discharge_drop: f64,
}

impl Default for WorkZone {
    fn default() -> Self {
        Self {
            total_lanes: 3,
            open_lanes: 2,
            soft_barrier: false,
            rural: false,
            lateral_distance_ft: 6.0,
            night: false,
            speed_ratio: 1.0,
            speed_limit_mi_h: 55.0,
            total_ramp_density: 1.0,
            queue_discharge_drop: DEFAULT_WORK_ZONE_DISCHARGE_DROP,
        }
    }
}

/// Equation 10-7: lane closure severity index
/// `LCSI = 1 / (OR × N_o)` where `OR = N_o / N_total` (open ratio) and
/// `N_o` = number of open lanes. Capped at 2.0 for severe closures (e.g.,
/// 3-to-1, 4-to-1) per the Chapter 10 text.
///
/// Exhibit 10-15 values reproduced by this equation:
/// 3-to-3: 0.33; 2-to-2: 0.50; 4-to-3: 0.44; 3-to-2: 0.75; 4-to-2: 1.00;
/// 2-to-1: 2.00.
pub fn lane_closure_severity_index(total_lanes: u32, open_lanes: u32) -> f64 {
    if total_lanes == 0 || open_lanes == 0 {
        return 2.0;
    }
    let open_ratio = open_lanes as f64 / total_lanes as f64;
    (1.0 / (open_ratio * open_lanes as f64)).min(2.0)
}

impl WorkZone {
    /// Equation 10-7 / Exhibit 10-15: LCSI for this work zone.
    pub fn lcsi(&self) -> f64 {
        lane_closure_severity_index(self.total_lanes, self.open_lanes)
    }

    /// Equation 10-8: average 15-min work zone queue discharge rate,
    /// pc/h/ln:
    ///
    /// `QDR_wz = 2,093 − 154×LCSI − 194×f_Br − 179×f_AT + 9×f_LAT − 59×f_DN`
    pub fn queue_discharge_rate(&self) -> f64 {
        2093.0 - 154.0 * self.lcsi()
            - 194.0 * f64::from(u8::from(self.soft_barrier))
            - 179.0 * f64::from(u8::from(self.rural))
            + 9.0 * self.lateral_distance_ft
            - 59.0 * f64::from(u8::from(self.night))
    }

    /// Equation 10-9: prebreakdown work zone capacity, pc/h/ln:
    ///
    /// `c_wz = QDR_wz / (100 − alpha_wz) × 100`
    ///
    /// The result is capped at the non–work zone capacity
    /// `non_wz_capacity_pc` (Chapter 10 text: "the calculated work zone
    /// capacity should not be greater than the non–work zone capacity").
    pub fn capacity_pc(&self, non_wz_capacity_pc: f64) -> f64 {
        let c_wz = self.queue_discharge_rate() / (1.0 - self.queue_discharge_drop);
        c_wz.min(non_wz_capacity_pc)
    }

    /// Equation 10-10: work zone free-flow speed, mi/h:
    ///
    /// `FFS_wz = 9.95 + 33.49×f_Sr + 0.53×SL_wz − 5.60×LCSI − 3.84×f_Br
    ///           − 1.71×f_DN − 8.7×TRD` with `1 <= f_Sr <= 1.2`
    ///
    /// Capped at the non–work zone FFS `non_wz_ffs` per the Chapter 10 text.
    pub fn ffs(&self, non_wz_ffs: f64) -> f64 {
        let f_sr = self.speed_ratio.clamp(1.0, 1.2);
        let ffs_wz = 9.95 + 33.49 * f_sr + 0.53 * self.speed_limit_mi_h
            - 5.60 * self.lcsi()
            - 3.84 * f64::from(u8::from(self.soft_barrier))
            - 1.71 * f64::from(u8::from(self.night))
            - 8.7 * self.total_ramp_density;
        ffs_wz.min(non_wz_ffs)
    }

    /// Equation 10-11: work zone capacity adjustment factor
    /// `CAF_wz = c_wz / c`, capped at 1.0.
    ///
    /// `non_wz_capacity_pc` is the non–work zone per-lane capacity, pc/h/ln.
    pub fn caf(&self, non_wz_capacity_pc: f64) -> f64 {
        if non_wz_capacity_pc <= 0.0 {
            return 1.0;
        }
        (self.capacity_pc(non_wz_capacity_pc) / non_wz_capacity_pc).min(1.0)
    }

    /// Equation 10-12: work zone speed adjustment factor
    /// `SAF_wz = FFS_wz / FFS`, capped at 1.0.
    pub fn saf(&self, non_wz_ffs: f64) -> f64 {
        if non_wz_ffs <= 0.0 {
            return 1.0;
        }
        (self.ffs(non_wz_ffs) / non_wz_ffs).min(1.0)
    }
}

// ═════════════════════════════════════════════════════════════════════════
// Tests
// ═════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use LevelOfService as L;

    #[test]
    fn test_exhibit_10_6_urban_boundaries() {
        assert_eq!(los_freeway_facility(11.0, false, CityType::Urban), L::A);
        assert_eq!(los_freeway_facility(11.01, false, CityType::Urban), L::B);
        assert_eq!(los_freeway_facility(18.0, false, CityType::Urban), L::B);
        assert_eq!(los_freeway_facility(26.0, false, CityType::Urban), L::C);
        assert_eq!(los_freeway_facility(35.0, false, CityType::Urban), L::D);
        assert_eq!(los_freeway_facility(45.0, false, CityType::Urban), L::E);
        assert_eq!(los_freeway_facility(45.01, false, CityType::Urban), L::F);
        // Any segment vd/c > 1.00 forces facility LOS F
        assert_eq!(los_freeway_facility(20.0, true, CityType::Urban), L::F);
    }

    #[test]
    fn test_exhibit_10_6_rural_boundaries() {
        assert_eq!(los_freeway_facility(6.0, false, CityType::Rural), L::A);
        assert_eq!(los_freeway_facility(6.01, false, CityType::Rural), L::B);
        assert_eq!(los_freeway_facility(14.0, false, CityType::Rural), L::B);
        assert_eq!(los_freeway_facility(22.0, false, CityType::Rural), L::C);
        assert_eq!(los_freeway_facility(29.0, false, CityType::Rural), L::D);
        assert_eq!(los_freeway_facility(39.0, false, CityType::Rural), L::E);
        assert_eq!(los_freeway_facility(39.01, false, CityType::Rural), L::F);
        assert_eq!(los_freeway_facility(10.0, true, CityType::Rural), L::F);
    }

    #[test]
    fn test_equation_10_1_facility_density() {
        // Uniform density: weighted average equals the density
        let d = facility_density(&[30.0, 30.0], &[1.0, 2.0], &[3.0, 3.0]);
        assert!((d - 30.0).abs() < 1e-12);
        // Weighting by L×N
        let d = facility_density(&[20.0, 40.0], &[1.0, 1.0], &[2.0, 2.0]);
        assert!((d - 30.0).abs() < 1e-12);
        let d = facility_density(&[20.0, 40.0], &[3.0, 1.0], &[2.0, 2.0]);
        assert!((d - 25.0).abs() < 1e-12);
    }

    #[test]
    fn test_equations_10_2_10_3_demand_balancing() {
        // Entering 1,100 vs exiting 1,000 => f_TIS = 1.10
        let f = time_interval_scale_factor(&[600.0, 500.0], &[700.0, 300.0]);
        assert!((f - 1.1).abs() < 1e-12);
        let adj = balance_exit_demands(&[600.0, 500.0], &[700.0, 300.0]);
        assert!((adj[0] - 770.0).abs() < 1e-9);
        assert!((adj[1] - 330.0).abs() < 1e-9);
    }

    #[test]
    fn test_equation_25_1_max_achievable_speed() {
        // No upstream slowdown: V_max = FFS
        assert!((max_achievable_speed(60.0, 60.0, 1890.0) - 60.0).abs() < 1e-12);
        // HCM Chapter 25 Example Problem 1, Segment 3, Analysis Period 1:
        // FFS = 60, V_prev = 53.9, L = 1,890 ft => V_max ≈ 59.7 mi/h
        let v = max_achievable_speed(60.0, 53.9, 1890.0);
        assert!((v - 59.71).abs() < 0.05, "got {v}");
    }

    #[test]
    fn test_exhibit_10_15_lcsi_values() {
        assert!((lane_closure_severity_index(3, 3) - 1.0 / 3.0).abs() < 0.005); // 0.33
        assert!((lane_closure_severity_index(2, 2) - 0.50).abs() < 1e-12);
        assert!((lane_closure_severity_index(4, 3) - 0.4444).abs() < 0.001); // 0.44
        assert!((lane_closure_severity_index(3, 2) - 0.75).abs() < 1e-12);
        assert!((lane_closure_severity_index(4, 2) - 1.00).abs() < 1e-12);
        assert!((lane_closure_severity_index(2, 1) - 2.00).abs() < 1e-12);
        // 3-to-1 computes to 3.0 but is capped at 2.0 (Chapter 10 text)
        assert!((lane_closure_severity_index(3, 1) - 2.00).abs() < 1e-12);
    }

    #[test]
    fn test_equation_10_8_queue_discharge_rate() {
        // Base case: 2-to-2 (LCSI = 0.5), hard barrier, urban, 6 ft lateral,
        // daytime: QDR = 2,093 − 77 − 0 − 0 + 54 − 0 = 2,070 pc/h/ln
        let wz = WorkZone::default_2to2();
        assert!((wz.queue_discharge_rate() - 2070.0).abs() < 1e-9);
        // Severe: 2-to-1 (LCSI = 2), soft barrier, rural, 0 ft, night:
        // QDR = 2,093 − 308 − 194 − 179 + 0 − 59 = 1,353 pc/h/ln
        let wz = WorkZone {
            total_lanes: 2,
            open_lanes: 1,
            soft_barrier: true,
            rural: true,
            lateral_distance_ft: 0.0,
            night: true,
            ..Default::default()
        };
        assert!((wz.queue_discharge_rate() - 1353.0).abs() < 1e-9);
    }

    impl WorkZone {
        fn default_2to2() -> Self {
            WorkZone {
                total_lanes: 2,
                open_lanes: 2,
                soft_barrier: false,
                rural: false,
                lateral_distance_ft: 6.0,
                night: false,
                ..Default::default()
            }
        }
    }

    #[test]
    fn test_equations_10_9_through_10_12() {
        let wz = WorkZone::default_2to2();
        // Equation 10-9: c_wz = 2,070 / (1 − 0.134) = 2,390.3 pc/h/ln,
        // capped at the non-WZ capacity
        let c_wz = wz.capacity_pc(2400.0);
        assert!((c_wz - 2070.0 / 0.866).abs() < 0.1);
        assert!((wz.capacity_pc(2300.0) - 2300.0).abs() < 1e-9); // cap

        // Equation 10-10: f_Sr = 1.0 (65/65 clamped), SL_wz = 55, LCSI = 0.5,
        // TRD = 1.0: FFS_wz = 9.95 + 33.49 + 29.15 − 2.8 − 0 − 0 − 8.7 = 61.09,
        // capped at FFS = 60
        let ffs_wz = wz.ffs(70.0);
        assert!((ffs_wz - 61.09).abs() < 0.01, "got {ffs_wz}");
        assert!((wz.ffs(60.0) - 60.0).abs() < 1e-9); // cap

        // Equations 10-11/10-12: CAF/SAF never exceed 1.0
        assert!(wz.caf(2400.0) <= 1.0);
        assert!(wz.saf(60.0) <= 1.0);
        assert!((wz.caf(2400.0) - (2070.0 / 0.866) / 2400.0).abs() < 1e-6);
        assert!((wz.saf(70.0) - 61.09 / 70.0).abs() < 0.001);
    }
}
