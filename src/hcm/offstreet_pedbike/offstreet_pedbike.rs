//! # Off-Street Pedestrian and Bicycle Facilities (HCM Chapter 24)
//!
//! This module implements the Highway Capacity Manual (HCM) 7th Edition methodologies
//! for analyzing off-street pedestrian and bicycle facilities. Off-street facilities
//! serve only nonmotorized traffic and are separated from motor vehicle traffic to the
//! extent that such traffic does not affect their quality of service.
//!
//! ## Methodologies
//!
//! Three methodologies are implemented, matching the three categories of the chapter:
//!
//! 1. **Exclusive off-street pedestrian facilities** ([`ExclusivePedestrianFacility`]):
//!    walkways, cross-flow areas, and stairways. The service measure is average
//!    pedestrian space (ft²/p), with LOS from Exhibit 24-1 (random flow),
//!    Exhibit 24-2 (platoon flow), or Exhibit 24-3 (stairways).
//!
//! 2. **Pedestrians on shared-use paths** ([`SharedUsePathPedestrian`]): the service
//!    measure is the weighted number of bicycle passing and meeting events per hour
//!    (Equations 24-5 to 24-7), with LOS from Exhibit 24-4.
//!
//! 3. **Bicyclists on shared-use and exclusive off-street bicycle facilities**
//!    ([`OffStreetBicycleFacility`]): the service measure is a bicycle LOS (BLOS)
//!    score (Equation 24-35) incorporating meetings per minute, active passings per
//!    minute, path width, centerline presence, and delayed passings, with LOS from
//!    Exhibit 24-5. In the special case of an exclusive bicycle facility, the volume
//!    for all nonbicycle modes is set to zero.
//!
//! ## Analysis Scope
//!
//! Analysis occurs at the segment level; the analysis period is 15 min. Typical
//! segment lengths range from about 0.25 mi to 2-3 mi. The shared-use path
//! methodology is not applicable to paths wider than 20 ft, nor to soft (unpaved)
//! surfaces.
//!
//! ## References
//!
//! - HCM 7th Edition, Chapter 24: Off-Street Pedestrian and Bicycle Facilities
//! - HCM 7th Edition, Chapter 35: Pedestrians and Bicycles: Supplemental (example problems)
//! - Hummer et al., FHWA shared-use path LOS research (HCM Ch. 24 Ref. 5)

use serde::{Deserialize, Serialize};

use crate::hcm::common::LevelOfService;

// ═══════════════════════════════════════════════════════════════════════════════
// Capacities and defaults
// ═══════════════════════════════════════════════════════════════════════════════

/// Capacity of walkways with random flow (p/min/ft). HCM Chapter 24, Step 5
/// (exclusive pedestrian facilities).
pub const CAPACITY_WALKWAY_RANDOM: f64 = 23.0;

/// Capacity of walkways with platoon flow, averaged over 5 min (p/min/ft).
/// HCM Chapter 24, Step 5 (exclusive pedestrian facilities).
pub const CAPACITY_WALKWAY_PLATOON: f64 = 18.0;

/// Capacity of cross-flow areas (p/min/ft, sum of both crossing flows).
/// HCM Chapter 24, Step 5 (exclusive pedestrian facilities).
pub const CAPACITY_CROSS_FLOW: f64 = 17.0;

/// Capacity of stairways in the ascending direction (p/min/ft).
/// HCM Chapter 24, Step 5 (exclusive pedestrian facilities).
pub const CAPACITY_STAIRWAY: f64 = 15.0;

/// LOS E-F (capacity) threshold for cross-flow situations (ft²/p).
/// HCM Exhibit 24-1 and Exhibit 24-2, note c.
pub const CROSS_FLOW_LOS_F_SPACE_THRESHOLD: f64 = 13.0;

/// Default peak hour factor. HCM Exhibit 24-6.
pub const DEFAULT_PHF: f64 = 0.85;

/// Default average pedestrian speed on exclusive pedestrian facilities (ft/min).
/// HCM Exhibit 24-6.
pub const DEFAULT_PEDESTRIAN_SPEED_FT_MIN: f64 = 300.0;

/// Discretization step for the shared-use path numerical integrations (mi).
/// HCM Chapter 24 (research finding, Ref. 5): dx = 0.01 mi is appropriate for
/// Equation 24-11 and subsequent equations.
pub const PATH_INTEGRATION_STEP_MI: f64 = 0.01;

// ═══════════════════════════════════════════════════════════════════════════════
// Shared-use path user modes
// ═══════════════════════════════════════════════════════════════════════════════

/// Number of path user mode groups in the shared-use path BLOS methodology.
pub const NUM_PATH_MODES: usize = 5;

/// Path user mode groups addressed by the shared-use path BLOS methodology.
///
/// The discriminant is the index into the per-mode arrays used throughout
/// [`OffStreetBicycleFacility`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PathUserMode {
    /// Adult bicyclists.
    Bicycle = 0,
    /// Pedestrians (walking).
    Pedestrian = 1,
    /// Runners.
    Runner = 2,
    /// Inline skaters.
    InlineSkater = 3,
    /// Child bicyclists.
    ChildBicyclist = 4,
}

/// All path user modes, in per-mode array index order.
pub const PATH_MODES: [PathUserMode; NUM_PATH_MODES] = [
    PathUserMode::Bicycle,
    PathUserMode::Pedestrian,
    PathUserMode::Runner,
    PathUserMode::InlineSkater,
    PathUserMode::ChildBicyclist,
];

/// HCM Exhibit 24-15: Required Bicycle Passing Distance (ft), indexed by the
/// mode being passed ([`PathUserMode`] order: bicycle, pedestrian, runner,
/// inline skater, child bicyclist).
pub const REQUIRED_PASSING_DISTANCE_FT: [f64; NUM_PATH_MODES] = [100.0, 60.0, 70.0, 100.0, 70.0];

/// HCM Exhibit 24-16: Frequency of Blocking of Two Lanes (decimal), indexed by
/// [`PathUserMode`] (bicycle, pedestrian, runner, inline skater, child bicyclist).
pub const TWO_LANE_BLOCKING_FREQUENCY: [f64; NUM_PATH_MODES] = [0.05, 0.36, 0.12, 0.08, 0.01];

/// Demand and speed characteristics for one path user mode group.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PathUserGroup {
    /// Path mode split for this user group, p_i (decimal).
    pub mode_split: f64,
    /// Average mode group speed, μ_i (mi/h).
    pub average_speed: f64,
    /// Mode group speed standard deviation, σ_i (mi/h).
    pub speed_standard_deviation: f64,
}

/// HCM Exhibit 24-6: default mode splits, average speeds (mi/h), and speed
/// standard deviations (mi/h) for the five path user mode groups, in
/// [`PathUserMode`] order (bicycle, pedestrian, runner, inline skater,
/// child bicyclist).
pub const DEFAULT_PATH_USER_GROUPS: [PathUserGroup; NUM_PATH_MODES] = [
    PathUserGroup { mode_split: 0.55, average_speed: 12.8, speed_standard_deviation: 3.4 },
    PathUserGroup { mode_split: 0.20, average_speed: 3.4, speed_standard_deviation: 0.6 },
    PathUserGroup { mode_split: 0.10, average_speed: 6.5, speed_standard_deviation: 1.2 },
    PathUserGroup { mode_split: 0.10, average_speed: 10.1, speed_standard_deviation: 2.7 },
    PathUserGroup { mode_split: 0.05, average_speed: 7.9, speed_standard_deviation: 1.9 },
];

// ═══════════════════════════════════════════════════════════════════════════════
// LOS tables (Exhibits 24-1 through 24-5)
// ═══════════════════════════════════════════════════════════════════════════════

/// HCM Exhibit 24-1: Random-Flow LOS Criteria for Walkways.
///
/// LOS is based on average pedestrian space (ft²/p):
///
/// | LOS | Average Space (ft²/p) | Flow Rate (p/min/ft) | v/c Ratio     |
/// |-----|-----------------------|----------------------|---------------|
/// | A   | >60                   | ≤5                   | ≤0.21         |
/// | B   | >40-60                | >5-7                 | >0.21-0.31    |
/// | C   | >24-40                | >7-10                | >0.31-0.44    |
/// | D   | >15-24                | >10-15               | >0.44-0.65    |
/// | E   | >8-15                 | >15-23               | >0.65-1.00    |
/// | F   | ≤8                    | Variable             | Variable      |
///
/// Does not apply to walkways with grades over 5%. In cross-flow situations the
/// LOS E-F threshold is 13 ft²/p (see [`CROSS_FLOW_LOS_F_SPACE_THRESHOLD`]).
pub fn walkway_random_flow_los(average_space: f64) -> LevelOfService {
    match average_space {
        s if s > 60.0 => LevelOfService::A,
        s if s > 40.0 => LevelOfService::B,
        s if s > 24.0 => LevelOfService::C,
        s if s > 15.0 => LevelOfService::D,
        s if s > 8.0 => LevelOfService::E,
        _ => LevelOfService::F,
    }
}

/// HCM Exhibit 24-2: Platoon-Adjusted LOS Criteria for Walkways.
///
/// LOS is based on average pedestrian space (ft²/p):
///
/// | LOS | Average Space (ft²/p) | Flow Rate over 5 min (p/min/ft) |
/// |-----|-----------------------|---------------------------------|
/// | A   | >530                  | ≤0.5                            |
/// | B   | >90-530               | >0.5-3                          |
/// | C   | >40-90                | >3-6                            |
/// | D   | >23-40                | >6-11                           |
/// | E   | >11-23                | >11-18                          |
/// | F   | ≤11                   | >18                             |
///
/// In cross-flow situations the LOS E-F threshold is 13 ft²/p.
pub fn walkway_platoon_flow_los(average_space: f64) -> LevelOfService {
    match average_space {
        s if s > 530.0 => LevelOfService::A,
        s if s > 90.0 => LevelOfService::B,
        s if s > 40.0 => LevelOfService::C,
        s if s > 23.0 => LevelOfService::D,
        s if s > 11.0 => LevelOfService::E,
        _ => LevelOfService::F,
    }
}

/// HCM Exhibit 24-3: LOS Criteria for Stairways.
///
/// LOS is based on average pedestrian space (ft²/p):
///
/// | LOS | Average Space (ft²/p) | Flow Rate (p/min/ft) | v/c Ratio  |
/// |-----|-----------------------|----------------------|------------|
/// | A   | >20                   | ≤5                   | ≤0.33      |
/// | B   | >17-20                | >5-6                 | >0.33-0.41 |
/// | C   | >12-17                | >6-8                 | >0.41-0.53 |
/// | D   | >8-12                 | >8-11                | >0.53-0.73 |
/// | E   | >5-8                  | >11-15               | >0.73-1.00 |
/// | F   | ≤5                    | Variable             | Variable   |
pub fn stairway_los(average_space: f64) -> LevelOfService {
    match average_space {
        s if s > 20.0 => LevelOfService::A,
        s if s > 17.0 => LevelOfService::B,
        s if s > 12.0 => LevelOfService::C,
        s if s > 8.0 => LevelOfService::D,
        s if s > 5.0 => LevelOfService::E,
        _ => LevelOfService::F,
    }
}

/// HCM Exhibit 24-4: Pedestrian LOS Criteria for Shared-Use Paths.
///
/// LOS is based on the weighted number of bicycle passing/meeting events per
/// hour experienced by the average pedestrian:
///
/// | LOS | Event Rate (events/h) |
/// |-----|-----------------------|
/// | A   | ≤38                   |
/// | B   | >38-60                |
/// | C   | >60-103               |
/// | D   | >103-144              |
/// | E   | >144-180              |
/// | F   | >180                  |
pub fn shared_use_path_pedestrian_los(events_per_hour: f64) -> LevelOfService {
    match events_per_hour {
        e if e <= 38.0 => LevelOfService::A,
        e if e <= 60.0 => LevelOfService::B,
        e if e <= 103.0 => LevelOfService::C,
        e if e <= 144.0 => LevelOfService::D,
        e if e <= 180.0 => LevelOfService::E,
        _ => LevelOfService::F,
    }
}

/// HCM Exhibit 24-5: LOS Criteria for Bicycles on Shared-Use and Exclusive Paths.
///
/// | LOS | BLOS Score |
/// |-----|------------|
/// | A   | >4.0       |
/// | B   | >3.5-4.0   |
/// | C   | >3.0-3.5   |
/// | D   | >2.5-3.0   |
/// | E   | >2.0-2.5   |
/// | F   | ≤2.0       |
pub fn bicycle_los_from_score(blos_score: f64) -> LevelOfService {
    match blos_score {
        s if s > 4.0 => LevelOfService::A,
        s if s > 3.5 => LevelOfService::B,
        s if s > 3.0 => LevelOfService::C,
        s if s > 2.5 => LevelOfService::D,
        s if s > 2.0 => LevelOfService::E,
        _ => LevelOfService::F,
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Normal distribution helpers
// ═══════════════════════════════════════════════════════════════════════════════

/// Error function approximation (Abramowitz & Stegun 7.1.26, |error| < 1.5e-7).
fn erf(x: f64) -> f64 {
    let sign = if x < 0.0 { -1.0 } else { 1.0 };
    let x = x.abs();
    let t = 1.0 / (1.0 + 0.327_591_1 * x);
    let poly = t
        * (0.254_829_592
            + t * (-0.284_496_736 + t * (1.421_413_741 + t * (-1.453_152_027 + t * 1.061_405_429))));
    sign * (1.0 - poly * (-x * x).exp())
}

/// Cumulative distribution function of a normal distribution with the given
/// mean and standard deviation, evaluated at `x`.
pub fn normal_cdf(x: f64, mean: f64, std_dev: f64) -> f64 {
    if std_dev <= 0.0 {
        return if x >= mean { 1.0 } else { 0.0 };
    }
    0.5 * (1.0 + erf((x - mean) / (std_dev * std::f64::consts::SQRT_2)))
}

// ═══════════════════════════════════════════════════════════════════════════════
// Exclusive off-street pedestrian facilities
// ═══════════════════════════════════════════════════════════════════════════════

/// Type of exclusive pedestrian facility being analyzed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum PedestrianFacilityType {
    /// A walkway, pedestrian zone, plaza circulation route, or ramp with grade ≤5%.
    #[default]
    Walkway,
    /// A location where two approximately perpendicular pedestrian streams cross
    /// (e.g., the intersection of two walkways or a building entrance). LOS E-F
    /// threshold is 13 ft²/p and capacity is 17 p/min/ft (sum of both flows).
    CrossFlow,
    /// A stairway. The upward (ascending) flow rate should be used for analysis.
    /// A small reverse flow should be assumed to occupy one 30-in. pedestrian
    /// lane, included in the fixed-object width term of Equation 24-1.
    Stairway,
}

/// Nature of pedestrian flow along the facility.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum PedestrianFlowType {
    /// Pedestrian arrivals are not influenced by platooning (Exhibit 24-1).
    #[default]
    Random,
    /// Pedestrians arrive in platoons, e.g., released by an upstream traffic
    /// signal or arriving on transit vehicles (Exhibit 24-2).
    Platooned,
}

/// Analysis of an exclusive off-street pedestrian facility (walkway, cross-flow
/// area, or stairway) per HCM Chapter 24.
///
/// The methodology follows Exhibit 24-7:
/// 1. Determine effective walkway width (Equation 24-1).
/// 2. Calculate pedestrian flow rate (Equations 24-2, 24-3).
/// 3. Calculate average pedestrian space (Equation 24-4).
/// 4. Determine LOS (Exhibits 24-1, 24-2, or 24-3).
/// 5. Calculate volume-to-capacity ratio.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ExclusivePedestrianFacility {
    // ── Inputs ──────────────────────────────────────────────────────────────
    /// Total walkway width at a given point along the walkway, W_T (ft).
    pub total_walkway_width: f64,
    /// Sum of fixed-object effective widths and linear-feature shy distances at
    /// the same point, W_O (ft). Typical fixed-object effective widths are given
    /// in HCM Exhibit 24-9 (e.g., light pole 2.5-3.5 ft, tree 3.0-4.0 ft,
    /// bench 5.0 ft). For stairways, a small reverse flow is represented by
    /// adding one 30-in. (2.5-ft) pedestrian lane to this term.
    pub fixed_object_width: f64,
    /// Pedestrian demand during the analysis hour, v_h (p/h). Not required if
    /// `peak_15min_volume` is provided.
    pub pedestrian_demand: Option<f64>,
    /// Field-measured pedestrian volume during the peak 15 min (p). When
    /// provided, it is used directly without applying a PHF.
    pub peak_15min_volume: Option<f64>,
    /// Peak hour factor (decimal). Default 0.85 (Exhibit 24-6).
    pub phf: f64,
    /// Average pedestrian speed, S_p (ft/min). Default 300 ft/min (Exhibit 24-6).
    /// Pedestrian speeds reduce when grades exceed 5%; the service measure is
    /// highly sensitive to this input.
    pub pedestrian_speed: f64,
    /// Type of facility (walkway, cross-flow area, or stairway).
    pub facility_type: PedestrianFacilityType,
    /// Pedestrian flow type (random or platooned). Ignored for stairways.
    pub flow_type: PedestrianFlowType,

    // ── Computed ────────────────────────────────────────────────────────────
    /// Effective walkway width, W_E (ft). HCM Equation 24-1.
    pub effective_width: Option<f64>,
    /// Pedestrian volume during the peak 15 min, v_15 (p). HCM Equation 24-2.
    pub flow_rate_15min: Option<f64>,
    /// Pedestrian flow per unit width, v_p (p/ft/min). HCM Equation 24-3.
    pub unit_flow_rate: Option<f64>,
    /// Average pedestrian space, A_p (ft²/p). HCM Equation 24-4.
    pub pedestrian_space: Option<f64>,
    /// Volume-to-capacity ratio (decimal).
    pub vc_ratio: Option<f64>,
    /// Level of service (Exhibit 24-1, 24-2, or 24-3).
    pub los: Option<LevelOfService>,
}

impl Default for ExclusivePedestrianFacility {
    fn default() -> Self {
        ExclusivePedestrianFacility {
            total_walkway_width: 0.0,
            fixed_object_width: 0.0,
            pedestrian_demand: None,
            peak_15min_volume: None,
            phf: DEFAULT_PHF,
            pedestrian_speed: DEFAULT_PEDESTRIAN_SPEED_FT_MIN,
            facility_type: PedestrianFacilityType::Walkway,
            flow_type: PedestrianFlowType::Random,
            effective_width: None,
            flow_rate_15min: None,
            unit_flow_rate: None,
            pedestrian_space: None,
            vc_ratio: None,
            los: None,
        }
    }
}

impl ExclusivePedestrianFacility {
    /// Create a new exclusive pedestrian facility analysis.
    ///
    /// # Arguments
    /// * `total_walkway_width` - Total walkway width W_T (ft)
    /// * `fixed_object_width` - Sum of fixed-object effective widths and shy distances W_O (ft)
    /// * `pedestrian_demand` - Hourly pedestrian demand v_h (p/h), if known
    /// * `peak_15min_volume` - Peak 15-min pedestrian volume (p), if field-measured
    /// * `phf` - Peak hour factor (default 0.85 when `None`)
    /// * `pedestrian_speed` - Average pedestrian speed S_p (ft/min; default 300 when `None`)
    /// * `facility_type` - Walkway, cross-flow area, or stairway
    /// * `flow_type` - Random or platooned pedestrian flow
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        total_walkway_width: f64,
        fixed_object_width: f64,
        pedestrian_demand: Option<f64>,
        peak_15min_volume: Option<f64>,
        phf: Option<f64>,
        pedestrian_speed: Option<f64>,
        facility_type: PedestrianFacilityType,
        flow_type: PedestrianFlowType,
    ) -> Self {
        ExclusivePedestrianFacility {
            total_walkway_width,
            fixed_object_width,
            pedestrian_demand,
            peak_15min_volume,
            phf: phf.unwrap_or(DEFAULT_PHF),
            pedestrian_speed: pedestrian_speed.unwrap_or(DEFAULT_PEDESTRIAN_SPEED_FT_MIN),
            facility_type,
            flow_type,
            ..Default::default()
        }
    }

    /// Step 1: Determine Effective Walkway Width.
    ///
    /// HCM Equation 24-1: `W_E = W_T - W_O`
    ///
    /// where W_E is the effective walkway width (ft), W_T is the total walkway
    /// width (ft), and W_O is the sum of fixed-object effective widths and
    /// linear-feature shy distances (ft). The result is floored at zero.
    pub fn determine_effective_walkway_width(&mut self) -> f64 {
        let we = (self.total_walkway_width - self.fixed_object_width).max(0.0);
        self.effective_width = Some(we);
        we
    }

    /// Step 2: Calculate Pedestrian Flow Rate.
    ///
    /// If a field-measured peak 15-min volume is available it is used directly.
    /// Otherwise, HCM Equation 24-2 converts hourly demand into the peak 15-min
    /// volume: `v_15 = v_h / (4 × PHF)`.
    ///
    /// HCM Equation 24-3 then converts the peak 15-min volume into a unit flow
    /// rate: `v_p = v_15 / (15 × W_E)` (p/ft/min).
    ///
    /// For stairways, the upward (ascending) flow rate should be supplied,
    /// because lower flow rates typically occur in the ascending direction.
    ///
    /// Returns the unit flow rate v_p (p/ft/min).
    pub fn calculate_pedestrian_flow_rate(&mut self) -> f64 {
        let we = match self.effective_width {
            Some(we) => we,
            None => self.determine_effective_walkway_width(),
        };
        // HCM Equation 24-2
        let v15 = match self.peak_15min_volume {
            Some(v) => v,
            None => {
                let vh = self.pedestrian_demand.unwrap_or(0.0);
                if self.phf > 0.0 { vh / (4.0 * self.phf) } else { 0.0 }
            }
        };
        self.flow_rate_15min = Some(v15);
        // HCM Equation 24-3
        let vp = if we > 0.0 { v15 / (15.0 * we) } else { 0.0 };
        self.unit_flow_rate = Some(vp);
        vp
    }

    /// Step 3: Calculate Average Pedestrian Space.
    ///
    /// HCM Equation 24-4: `A_p = S_p / v_p`
    ///
    /// where A_p is pedestrian space (ft²/p), S_p is pedestrian speed (ft/min),
    /// and v_p is pedestrian flow per unit width (p/ft/min).
    ///
    /// Returns `f64::INFINITY` when the unit flow rate is zero (empty facility);
    /// the stored computed field is left as `None` in that case.
    pub fn calculate_average_pedestrian_space(&mut self) -> f64 {
        let vp = match self.unit_flow_rate {
            Some(vp) => vp,
            None => self.calculate_pedestrian_flow_rate(),
        };
        if vp > 0.0 {
            let ap = self.pedestrian_speed / vp;
            self.pedestrian_space = Some(ap);
            ap
        } else {
            self.pedestrian_space = None;
            f64::INFINITY
        }
    }

    /// Step 4: Determine LOS.
    ///
    /// Uses HCM Exhibit 24-1 (walkways with random flow), Exhibit 24-2
    /// (walkways with platoon flow), or Exhibit 24-3 (stairways), based on the
    /// average pedestrian space. In cross-flow situations the LOS E-F threshold
    /// is 13 ft²/p (Exhibit 24-1/24-2, note c).
    pub fn determine_los(&mut self) -> LevelOfService {
        let space = match self.pedestrian_space {
            Some(ap) => ap,
            None => self.calculate_average_pedestrian_space(),
        };
        let los = match self.facility_type {
            PedestrianFacilityType::Stairway => stairway_los(space),
            PedestrianFacilityType::Walkway => match self.flow_type {
                PedestrianFlowType::Random => walkway_random_flow_los(space),
                PedestrianFlowType::Platooned => walkway_platoon_flow_los(space),
            },
            PedestrianFacilityType::CrossFlow => {
                // Cross-flow areas use the walkway exhibits, but the LOS E-F
                // (capacity) threshold occurs at 13 ft²/p.
                if space <= CROSS_FLOW_LOS_F_SPACE_THRESHOLD {
                    LevelOfService::F
                } else {
                    match self.flow_type {
                        PedestrianFlowType::Random => walkway_random_flow_los(space),
                        PedestrianFlowType::Platooned => walkway_platoon_flow_los(space),
                    }
                }
            }
        };
        self.los = Some(los);
        los
    }

    /// Step 5: Calculate Volume-to-Capacity Ratio.
    ///
    /// Capacities of exclusive pedestrian facilities (HCM Chapter 24, Step 5):
    /// - Walkways with random flow: 23 p/min/ft
    /// - Walkways with platoon flow (average over 5 min): 18 p/min/ft
    /// - Cross-flow areas: 17 p/min/ft (sum of both flows)
    /// - Stairways: 15 p/min/ft in the ascending direction
    ///
    /// For cross-flow areas the supplied demand should be the sum of both
    /// crossing flows.
    pub fn calculate_volume_to_capacity_ratio(&mut self) -> f64 {
        let vp = match self.unit_flow_rate {
            Some(vp) => vp,
            None => self.calculate_pedestrian_flow_rate(),
        };
        let capacity = match self.facility_type {
            PedestrianFacilityType::Walkway => match self.flow_type {
                PedestrianFlowType::Random => CAPACITY_WALKWAY_RANDOM,
                PedestrianFlowType::Platooned => CAPACITY_WALKWAY_PLATOON,
            },
            PedestrianFacilityType::CrossFlow => CAPACITY_CROSS_FLOW,
            PedestrianFacilityType::Stairway => CAPACITY_STAIRWAY,
        };
        let vc = vp / capacity;
        self.vc_ratio = Some(vc);
        vc
    }

    /// Run the complete exclusive pedestrian facility methodology (Steps 1-5).
    pub fn analyze(&mut self) -> LevelOfService {
        self.determine_effective_walkway_width();
        self.calculate_pedestrian_flow_rate();
        self.calculate_average_pedestrian_space();
        let los = self.determine_los();
        self.calculate_volume_to_capacity_ratio();
        los
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Pedestrians on shared-use paths
// ═══════════════════════════════════════════════════════════════════════════════

/// Analysis of pedestrian LOS on a shared-use path per HCM Chapter 24.
///
/// LOS is based on hindrance: the weighted number of events per hour in which a
/// pedestrian meets an oncoming bicyclist or is passed by a bicyclist.
///
/// The methodology follows Exhibit 24-10:
/// 1. Gather input data.
/// 2. Calculate the number of bicycle passing and meeting events
///    (Equations 24-5 to 24-7).
/// 3. Determine LOS (Exhibit 24-4).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct SharedUsePathPedestrian {
    // ── Inputs ──────────────────────────────────────────────────────────────
    /// Hourly bicycle demand in the same direction as the pedestrian,
    /// Q_sb (bicycles/h). Not required if `bicycle_flow_rate_same_direction`
    /// is provided.
    pub bicycle_demand_same_direction: Option<f64>,
    /// Hourly bicycle demand in the opposing direction, Q_ob (bicycles/h).
    /// Not required if `bicycle_flow_rate_opposing` is provided.
    pub bicycle_demand_opposing: Option<f64>,
    /// Field-measured peak 15-min bicycle flow rate in the same direction
    /// (bicycles/h). When provided, it substitutes for the Q_sb/PHF term of
    /// Equation 24-5.
    pub bicycle_flow_rate_same_direction: Option<f64>,
    /// Field-measured peak 15-min bicycle flow rate in the opposing direction
    /// (bicycles/h). When provided, it substitutes for the Q_ob/PHF term of
    /// Equation 24-6.
    pub bicycle_flow_rate_opposing: Option<f64>,
    /// Peak hour factor (decimal). Default 0.85 (Exhibit 24-6).
    pub phf: f64,
    /// Mean pedestrian speed on the path, S_p. Default 3.4 mi/h (Exhibit 24-6).
    /// Only the ratio S_p/S_b is used, so any speed unit may be used as long as
    /// it is consistent with `bicycle_speed`.
    pub pedestrian_speed: f64,
    /// Mean bicycle speed on the path, S_b. Default 12.8 mi/h (Exhibit 24-6).
    pub bicycle_speed: f64,
    /// One-way path flag. On one-way paths there are no meeting events, so only
    /// F_p is calculated. Paths 15 ft or more in width may effectively operate
    /// as two adjacent one-way facilities, in which case F_m may be set to zero
    /// by setting this flag.
    pub is_one_way: bool,

    // ── Computed ────────────────────────────────────────────────────────────
    /// Number of passing events, F_p (events/h). HCM Equation 24-5.
    pub passing_events: Option<f64>,
    /// Number of meeting events, F_m (events/h). HCM Equation 24-6.
    pub meeting_events: Option<f64>,
    /// Total (weighted) number of events, F (events/h). HCM Equation 24-7.
    pub total_events: Option<f64>,
    /// Pedestrian level of service (Exhibit 24-4).
    pub los: Option<LevelOfService>,
}

impl Default for SharedUsePathPedestrian {
    fn default() -> Self {
        SharedUsePathPedestrian {
            bicycle_demand_same_direction: None,
            bicycle_demand_opposing: None,
            bicycle_flow_rate_same_direction: None,
            bicycle_flow_rate_opposing: None,
            phf: DEFAULT_PHF,
            pedestrian_speed: 3.4,
            bicycle_speed: 12.8,
            is_one_way: false,
            passing_events: None,
            meeting_events: None,
            total_events: None,
            los: None,
        }
    }
}

impl SharedUsePathPedestrian {
    /// Create a new shared-use path pedestrian analysis.
    ///
    /// # Arguments
    /// * `bicycle_demand_same_direction` - Q_sb (bicycles/h)
    /// * `bicycle_demand_opposing` - Q_ob (bicycles/h)
    /// * `phf` - Peak hour factor (default 0.85 when `None`)
    /// * `pedestrian_speed` - S_p (default 3.4 mi/h when `None`)
    /// * `bicycle_speed` - S_b (default 12.8 mi/h when `None`)
    pub fn new(
        bicycle_demand_same_direction: Option<f64>,
        bicycle_demand_opposing: Option<f64>,
        phf: Option<f64>,
        pedestrian_speed: Option<f64>,
        bicycle_speed: Option<f64>,
    ) -> Self {
        SharedUsePathPedestrian {
            bicycle_demand_same_direction,
            bicycle_demand_opposing,
            phf: phf.unwrap_or(DEFAULT_PHF),
            pedestrian_speed: pedestrian_speed.unwrap_or(3.4),
            bicycle_speed: bicycle_speed.unwrap_or(12.8),
            ..Default::default()
        }
    }

    /// Peak 15-min same-direction bicycle flow rate (bicycles/h): the measured
    /// value if provided, otherwise Q_sb/PHF.
    fn same_direction_flow_rate(&self) -> f64 {
        match self.bicycle_flow_rate_same_direction {
            Some(q) => q,
            None => {
                let q = self.bicycle_demand_same_direction.unwrap_or(0.0);
                if self.phf > 0.0 { q / self.phf } else { 0.0 }
            }
        }
    }

    /// Peak 15-min opposing bicycle flow rate (bicycles/h): the measured value
    /// if provided, otherwise Q_ob/PHF. Zero for one-way paths.
    fn opposing_flow_rate(&self) -> f64 {
        if self.is_one_way {
            return 0.0;
        }
        match self.bicycle_flow_rate_opposing {
            Some(q) => q,
            None => {
                let q = self.bicycle_demand_opposing.unwrap_or(0.0);
                if self.phf > 0.0 { q / self.phf } else { 0.0 }
            }
        }
    }

    /// Step 2: Calculate Number of Bicycle Passing and Meeting Events.
    ///
    /// HCM Equation 24-5: `F_p = (Q_sb / PHF) × (1 - S_p/S_b)`
    ///
    /// HCM Equation 24-6: `F_m = (Q_ob / PHF) × (1 + S_p/S_b)`
    ///
    /// HCM Equation 24-7: `F = F_p + 0.5 × F_m`
    ///
    /// Meeting events allow direct visual contact and cause less hindrance, so a
    /// factor of 0.5 is applied to meeting events. If peak 15-min directional
    /// volumes are known they substitute for the Q/PHF terms. For one-way paths
    /// there are no meeting events (F_m = 0).
    ///
    /// Returns `(F_p, F_m, F)` in events/h.
    pub fn calculate_bicycle_passing_and_meeting_events(&mut self) -> (f64, f64, f64) {
        let speed_ratio = if self.bicycle_speed > 0.0 {
            self.pedestrian_speed / self.bicycle_speed
        } else {
            0.0
        };
        // HCM Equation 24-5
        let fp = self.same_direction_flow_rate() * (1.0 - speed_ratio);
        // HCM Equation 24-6
        let fm = self.opposing_flow_rate() * (1.0 + speed_ratio);
        // HCM Equation 24-7
        let f = fp + 0.5 * fm;
        self.passing_events = Some(fp);
        self.meeting_events = Some(fm);
        self.total_events = Some(f);
        (fp, fm, f)
    }

    /// Step 3: Determine LOS.
    ///
    /// Uses HCM Exhibit 24-4 based on the total events per hour calculated in
    /// Step 2. The LOS E-F threshold does not reflect the capacity of a
    /// shared-use path but rather a point of severely diminished experience.
    pub fn determine_los(&mut self) -> LevelOfService {
        let f = match self.total_events {
            Some(f) => f,
            None => self.calculate_bicycle_passing_and_meeting_events().2,
        };
        let los = shared_use_path_pedestrian_los(f);
        self.los = Some(los);
        los
    }

    /// Run the complete shared-use path pedestrian methodology (Steps 1-3).
    pub fn analyze(&mut self) -> LevelOfService {
        self.calculate_bicycle_passing_and_meeting_events();
        self.determine_los()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Off-street bicycle facilities (BLOS)
// ═══════════════════════════════════════════════════════════════════════════════

/// Probability that a passing section of the given length is blocked by users of
/// one mode, based on a Poisson distribution of users along the path.
///
/// HCM Equation 24-17: `P_n,i = 1 - e^(-p_i × k_i)`
///
/// # Arguments
/// * `passing_distance_ft` - Distance required to pass mode i, p_i (ft; Exhibit 24-15)
/// * `density_per_mi` - Density of users of mode i, k_i = q_i/μ_i (users/mi)
pub fn probability_blocked(passing_distance_ft: f64, density_per_mi: f64) -> f64 {
    1.0 - (-(passing_distance_ft / 5280.0) * density_per_mi).exp()
}

/// Probability of delayed passing in the subject direction on a two-lane path.
///
/// HCM Equation 24-20 (closed-form solution of Equations 24-18 and 24-19):
///
/// `P_ds = [P_no×P_ns + P_no×(1-P_ns)²] / [1 - P_no×P_ns×(1-P_no)×(1-P_ns)]`
///
/// # Arguments
/// * `p_ns` - Probability of a blocked lane in the subject direction
/// * `p_no` - Probability of a blocked lane in the opposing direction
pub fn delayed_passing_probability_two_lane(p_ns: f64, p_no: f64) -> f64 {
    let numerator = p_no * p_ns + p_no * (1.0 - p_ns).powi(2);
    let denominator = 1.0 - p_no * p_ns * (1.0 - p_no) * (1.0 - p_ns);
    if denominator > 0.0 { numerator / denominator } else { 1.0 }
}

/// Analysis of bicycle LOS (BLOS) on a shared-use or exclusive off-street
/// bicycle facility per HCM Chapter 24.
///
/// The methodology follows Exhibit 24-11:
/// 1. Gather input data (Equation 24-8).
/// 2. Calculate active passings per minute (Equations 24-9 to 24-12).
/// 3. Calculate meetings per minute (Equations 24-13 to 24-16).
/// 4. Determine the number of effective lanes (Exhibit 24-14).
/// 5. Calculate the probability of delayed passing (Equations 24-17 to 24-32).
/// 6. Calculate delayed passings per minute (Equations 24-33, 24-34).
/// 7. Determine BLOS (Equation 24-35, Exhibit 24-5).
/// 8. Adjust LOS for low-volume paths.
///
/// For an exclusive off-street bicycle facility, set the bicycle mode split to
/// 1.0 and all other mode splits to zero.
///
/// The methodology is applicable to paved paths up to 20 ft wide. It was
/// developed from data on two-way paths but may be applied to one-way paths by
/// setting opposing volumes equal to zero (`is_one_way`).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct OffStreetBicycleFacility {
    // ── Inputs ──────────────────────────────────────────────────────────────
    /// Path width (ft). The methodology is not applicable to paths wider than 20 ft.
    pub path_width: f64,
    /// Whether the path has a centerline stripe.
    pub has_centerline: bool,
    /// Length of the analysis path segment, L (mi).
    pub segment_length: f64,
    /// One-way path flag. On one-way paths there are no opposing users, so
    /// meetings per minute M_T is zero.
    pub is_one_way: bool,
    /// Total two-directional path demand (path users/h). Split into directional
    /// demands with `directional_split` when directional demands are not given.
    pub two_way_demand: Option<f64>,
    /// Proportion of the two-way demand traveling in the subject direction
    /// (decimal). Default 0.50 (Exhibit 24-6). LOS results are highly sensitive
    /// to this factor; field measurement is recommended.
    pub directional_split: f64,
    /// Total hourly path demand in the subject direction, Q_T (path users/h).
    /// Overrides `two_way_demand` when provided.
    pub subject_demand: Option<f64>,
    /// Total hourly path demand in the opposing direction (path users/h).
    /// Overrides `two_way_demand` when provided.
    pub opposing_demand: Option<f64>,
    /// Peak hour factor (decimal). Default 0.85 (Exhibit 24-6).
    pub phf: f64,
    /// Mode split, average speed (mi/h), and speed standard deviation (mi/h)
    /// for each of the five path user mode groups, in [`PathUserMode`] order.
    /// Defaults from HCM Exhibit 24-6 (see [`DEFAULT_PATH_USER_GROUPS`]).
    pub user_groups: [PathUserGroup; NUM_PATH_MODES],

    // ── Computed ────────────────────────────────────────────────────────────
    /// Hourly directional path flow rate by mode in the subject direction,
    /// q_i (modal users/h). HCM Equation 24-8.
    pub subject_flow_rates: Option<[f64; NUM_PATH_MODES]>,
    /// Hourly directional path flow rate by mode in the opposing direction
    /// (modal users/h). HCM Equation 24-8.
    pub opposing_flow_rates: Option<[f64; NUM_PATH_MODES]>,
    /// Expected active passings per minute by mode, A_i. HCM Equation 24-11.
    pub active_passings_by_mode: Option<[f64; NUM_PATH_MODES]>,
    /// Total expected active passings per minute, A_T. HCM Equation 24-12.
    pub active_passings_per_minute: Option<f64>,
    /// Meetings per minute of users already on the path segment, M_1.
    /// HCM Equation 24-13.
    pub meetings_on_segment: Option<f64>,
    /// Expected meetings per minute by mode of users beyond the segment when the
    /// average bicyclist enters, M_2,i. HCM Equation 24-15.
    pub meetings_beyond_segment_by_mode: Option<[f64; NUM_PATH_MODES]>,
    /// Total meetings per minute, M_T. HCM Equation 24-16.
    pub meetings_per_minute: Option<f64>,
    /// Number of effective lanes (Exhibit 24-14).
    pub effective_lanes: Option<u8>,
    /// Total probability of delayed passing, P_Tds. HCM Equation 24-33.
    pub total_probability_delayed_passing: Option<f64>,
    /// Delayed passings per minute, DP_m. HCM Equation 24-34.
    pub delayed_passings_per_minute: Option<f64>,
    /// Weighted events per minute, E = M_T + 10 × A_T (Equation 24-35 term).
    pub weighted_events_per_minute: Option<f64>,
    /// BLOS score. HCM Equation 24-35.
    pub blos_score: Option<f64>,
    /// Bicycle level of service (Exhibit 24-5, adjusted per Step 8).
    pub los: Option<LevelOfService>,
}

impl Default for OffStreetBicycleFacility {
    fn default() -> Self {
        OffStreetBicycleFacility {
            path_width: 10.0,
            has_centerline: false,
            segment_length: 0.0,
            is_one_way: false,
            two_way_demand: None,
            directional_split: 0.5,
            subject_demand: None,
            opposing_demand: None,
            phf: DEFAULT_PHF,
            user_groups: DEFAULT_PATH_USER_GROUPS,
            subject_flow_rates: None,
            opposing_flow_rates: None,
            active_passings_by_mode: None,
            active_passings_per_minute: None,
            meetings_on_segment: None,
            meetings_beyond_segment_by_mode: None,
            meetings_per_minute: None,
            effective_lanes: None,
            total_probability_delayed_passing: None,
            delayed_passings_per_minute: None,
            weighted_events_per_minute: None,
            blos_score: None,
            los: None,
        }
    }
}

impl OffStreetBicycleFacility {
    /// Create a new off-street bicycle facility analysis using the Exhibit 24-6
    /// default mode splits and speeds.
    ///
    /// # Arguments
    /// * `path_width` - Path width (ft)
    /// * `segment_length` - Path segment length L (mi)
    /// * `has_centerline` - Whether the path has a centerline stripe
    /// * `two_way_demand` - Total two-directional path demand (users/h)
    /// * `directional_split` - Subject-direction share of demand (default 0.50 when `None`)
    /// * `phf` - Peak hour factor (default 0.85 when `None`)
    pub fn new(
        path_width: f64,
        segment_length: f64,
        has_centerline: bool,
        two_way_demand: Option<f64>,
        directional_split: Option<f64>,
        phf: Option<f64>,
    ) -> Self {
        OffStreetBicycleFacility {
            path_width,
            segment_length,
            has_centerline,
            two_way_demand,
            directional_split: directional_split.unwrap_or(0.5),
            phf: phf.unwrap_or(DEFAULT_PHF),
            ..Default::default()
        }
    }

    /// Speed of the average bicyclist, U (mi/h): the average speed of the
    /// bicycle mode group.
    pub fn average_bicyclist_speed(&self) -> f64 {
        self.user_groups[PathUserMode::Bicycle as usize].average_speed
    }

    /// Path segment travel time for the average bicyclist, t (min).
    pub fn segment_travel_time(&self) -> f64 {
        let u = self.average_bicyclist_speed();
        if u > 0.0 { self.segment_length / u * 60.0 } else { 0.0 }
    }

    /// Step 1: Gather Input Data — calculate hourly directional flow rates.
    ///
    /// HCM Equation 24-8: `q_i = (Q_T × p_i) / PHF`
    ///
    /// where q_i is the hourly directional path flow rate for user group i
    /// (modal users/h), Q_T is the total hourly directional path demand
    /// (users/h), p_i is the path mode split for user group i, and PHF is the
    /// peak hour factor. Directional demands are derived from the two-way
    /// demand and directional split when not given directly. For one-way paths
    /// the opposing flow rates are zero.
    ///
    /// Returns `(subject_flow_rates, opposing_flow_rates)` by mode.
    pub fn calculate_directional_flow_rates(
        &mut self,
    ) -> ([f64; NUM_PATH_MODES], [f64; NUM_PATH_MODES]) {
        let two_way = self.two_way_demand.unwrap_or(0.0);
        let qt_subject = self
            .subject_demand
            .unwrap_or(two_way * self.directional_split);
        let qt_opposing = if self.is_one_way {
            0.0
        } else {
            self.opposing_demand
                .unwrap_or(two_way * (1.0 - self.directional_split))
        };
        let mut qs = [0.0; NUM_PATH_MODES];
        let mut qo = [0.0; NUM_PATH_MODES];
        for i in 0..NUM_PATH_MODES {
            let split = self.user_groups[i].mode_split;
            if self.phf > 0.0 {
                qs[i] = qt_subject * split / self.phf;
                qo[i] = qt_opposing * split / self.phf;
            }
        }
        self.subject_flow_rates = Some(qs);
        self.opposing_flow_rates = Some(qo);
        (qs, qo)
    }

    /// Directional user densities k_i = q_i/μ_i (users/mi) for one direction.
    fn densities(&self, flow_rates: &[f64; NUM_PATH_MODES]) -> [f64; NUM_PATH_MODES] {
        let mut k = [0.0; NUM_PATH_MODES];
        for i in 0..NUM_PATH_MODES {
            let mu = self.user_groups[i].average_speed;
            if mu > 0.0 {
                k[i] = flow_rates[i] / mu;
            }
        }
        k
    }

    /// Step 2: Calculate Active Passings per Minute.
    ///
    /// Active passings are same-direction path users passed by the average
    /// bicyclist (traveling at constant speed U). Mode group speeds are assumed
    /// normally distributed with mean μ_i and standard deviation σ_i.
    ///
    /// HCM Equation 24-9: `P(v_i) = P[v_i < U(1 - x/L)]`
    ///
    /// HCM Equation 24-10: `P(v_i) = 0.5[F(x - dx) + F(x)]`
    ///
    /// HCM Equation 24-11: `A_i = Σ_j P(v_i) × (q_i/μ_i) × (1/t) × dx_j`
    ///
    /// HCM Equation 24-12: `A_T = Σ_i A_i`
    ///
    /// The path of length L is divided into discrete pieces of length
    /// dx = 0.01 mi (research finding, HCM Ch. 24 Ref. 5).
    ///
    /// Returns A_T, the expected active passings per minute by the average
    /// bicyclist during the peak 15 min.
    pub fn calculate_active_passings_per_minute(&mut self) -> f64 {
        let qs = match self.subject_flow_rates {
            Some(q) => q,
            None => self.calculate_directional_flow_rates().0,
        };
        let u = self.average_bicyclist_speed();
        let l = self.segment_length;
        let t = self.segment_travel_time();
        let mut by_mode = [0.0; NUM_PATH_MODES];
        if u > 0.0 && l > 0.0 && t > 0.0 {
            let n = ((l / PATH_INTEGRATION_STEP_MI).round() as usize).max(1);
            let dx = l / n as f64;
            for i in 0..NUM_PATH_MODES {
                let group = &self.user_groups[i];
                if qs[i] <= 0.0 || group.average_speed <= 0.0 {
                    continue;
                }
                let density = qs[i] / group.average_speed;
                // HCM Equation 24-9: F(x) = P[v_i < U(1 - x/L)]
                let f = |x: f64| {
                    normal_cdf(
                        u * (1.0 - x / l),
                        group.average_speed,
                        group.speed_standard_deviation,
                    )
                };
                let mut sum = 0.0;
                let mut f_prev = f(0.0);
                for j in 1..=n {
                    let f_cur = f(j as f64 * dx);
                    // HCM Equations 24-10 and 24-11
                    sum += 0.5 * (f_prev + f_cur) * density * dx / t;
                    f_prev = f_cur;
                }
                by_mode[i] = sum;
            }
        }
        // HCM Equation 24-12
        let at: f64 = by_mode.iter().sum();
        self.active_passings_by_mode = Some(by_mode);
        self.active_passings_per_minute = Some(at);
        at
    }

    /// Step 3: Calculate Meetings per Minute.
    ///
    /// Meetings are opposing-direction path users passed by the average
    /// bicyclist within the segment.
    ///
    /// HCM Equation 24-13 (users already on the segment):
    /// `M_1 = (U/60) × Σ_i (q_i/μ_i)`
    ///
    /// HCM Equation 24-14 (users beyond the segment):
    /// `P(v_O,i) = P(v_i > X×U/L)`
    ///
    /// HCM Equation 24-15: `M_2,i = Σ_j P(v_O,i) × (q_i/μ_i) × (1/t) × dx_j`
    ///
    /// HCM Equation 24-16: `M_T = M_1 + Σ_i M_2,i`
    ///
    /// The supply length beyond the segment is x* = L, which captures at least
    /// 99% of meetings (HCM Ch. 24 Ref. 5). For one-way paths M_T = 0.
    ///
    /// Returns M_T, the total expected meetings per minute during the peak 15 min.
    pub fn calculate_meetings_per_minute(&mut self) -> f64 {
        if self.subject_flow_rates.is_none() {
            self.calculate_directional_flow_rates();
        }
        let qo = self.opposing_flow_rates.unwrap_or([0.0; NUM_PATH_MODES]);
        if self.is_one_way {
            self.meetings_on_segment = Some(0.0);
            self.meetings_beyond_segment_by_mode = Some([0.0; NUM_PATH_MODES]);
            self.meetings_per_minute = Some(0.0);
            return 0.0;
        }
        let u = self.average_bicyclist_speed();
        let l = self.segment_length;
        let t = self.segment_travel_time();

        // HCM Equation 24-13
        let mut m1 = 0.0;
        for i in 0..NUM_PATH_MODES {
            let mu = self.user_groups[i].average_speed;
            if mu > 0.0 {
                m1 += qo[i] / mu;
            }
        }
        m1 *= u / 60.0;

        // HCM Equations 24-14 and 24-15, with x* = L
        let mut m2 = [0.0; NUM_PATH_MODES];
        if u > 0.0 && l > 0.0 && t > 0.0 {
            let n = ((l / PATH_INTEGRATION_STEP_MI).round() as usize).max(1);
            let dx = l / n as f64;
            for i in 0..NUM_PATH_MODES {
                let group = &self.user_groups[i];
                if qo[i] <= 0.0 || group.average_speed <= 0.0 {
                    continue;
                }
                let density = qo[i] / group.average_speed;
                // HCM Equation 24-14: G(X) = P(v_i > X×U/L)
                let g = |x: f64| {
                    1.0 - normal_cdf(
                        x * u / l,
                        group.average_speed,
                        group.speed_standard_deviation,
                    )
                };
                let mut sum = 0.0;
                let mut g_prev = g(0.0);
                for j in 1..=n {
                    let g_cur = g(j as f64 * dx);
                    // HCM Equation 24-10 (with X substituted for x) and 24-15
                    sum += 0.5 * (g_prev + g_cur) * density * dx / t;
                    g_prev = g_cur;
                }
                m2[i] = sum;
            }
        }

        // HCM Equation 24-16
        let mt = m1 + m2.iter().sum::<f64>();
        self.meetings_on_segment = Some(m1);
        self.meetings_beyond_segment_by_mode = Some(m2);
        self.meetings_per_minute = Some(mt);
        mt
    }

    /// Step 4: Determine Number of Effective Lanes.
    ///
    /// HCM Exhibit 24-14: Effective Lanes by Path Width
    ///
    /// | Path Width (ft) | Effective Lanes |
    /// |-----------------|-----------------|
    /// | 8.0-10.5        | 2               |
    /// | 11.0-14.5       | 3               |
    /// | 15.0-20.0       | 4               |
    ///
    /// // VERIFY-HCM: Exhibit 24-14 leaves gaps (widths below 8.0 ft, between
    /// // 10.5 and 11.0 ft, between 14.5 and 15.0 ft, and above 20.0 ft). This
    /// // implementation assigns 2 lanes below 11.0 ft, 3 lanes below 15.0 ft,
    /// // and 4 lanes otherwise; the methodology is stated as not applicable to
    /// // paths wider than 20 ft.
    pub fn determine_number_of_effective_lanes(&mut self) -> u8 {
        let lanes = if self.path_width < 11.0 {
            2
        } else if self.path_width < 15.0 {
            3
        } else {
            4
        };
        self.effective_lanes = Some(lanes);
        lanes
    }

    /// Step 5: Calculate Probability of Delayed Passing.
    ///
    /// A delayed passing occurs when a same-direction user ahead of the average
    /// bicyclist and other path users blocking the available lanes prevent an
    /// immediate passing maneuver. The blocked-lane probability for a passing
    /// section follows a Poisson distribution (HCM Equation 24-17), using the
    /// required passing distances of Exhibit 24-15.
    ///
    /// - **Two-lane paths** (HCM Equations 24-18 to 24-20): computed for each
    ///   of the 25 modal pairs (mode passed in the subject direction × opposing
    ///   mode), then combined with HCM Equation 24-33:
    ///   `P_Tds = 1 - Π_m (1 - P_m,ds)`.
    ///   // VERIFY-HCM: for each pair, the required passing distance of the
    ///   // subject (passed) mode is applied to both the subject and opposing
    ///   // blocked-lane probabilities. This reproduces HCM Chapter 35 Example
    ///   // Problem 2 exactly (P_n,ped = 0.1908 computed with 100 ft, and
    ///   // P_Tds = 0.8334), but the text of Equation 24-17 is ambiguous on
    ///   // this point.
    /// - **Three-lane paths** (HCM Equations 24-21 to 24-32): mode-aggregated
    ///   single- and double-lane blockage probabilities are substituted into
    ///   Equations 24-23 and 24-24.
    ///   // VERIFY-HCM: Equations 24-29 and 24-30 as printed read
    ///   // `1 - e^(p_i k) - P_b`; the exponent sign is a typographical error
    ///   // and is implemented as `1 - e^(-p_i k) - P_b` (at-least-one-lane
    ///   // blockage minus two-lane blockage). No worked example is published
    ///   // for three-lane paths.
    /// - **Four-lane paths**: the path operates like a divided four-lane
    ///   highway; P_ds equals the probability that both subject lanes are
    ///   blocked, P_bs (Equations 24-25 and 24-27).
    ///
    /// Returns the total probability of delayed passing, P_Tds.
    pub fn calculate_probability_of_delayed_passing(&mut self) -> f64 {
        if self.subject_flow_rates.is_none() {
            self.calculate_directional_flow_rates();
        }
        let qs = self.subject_flow_rates.unwrap_or([0.0; NUM_PATH_MODES]);
        let qo = self.opposing_flow_rates.unwrap_or([0.0; NUM_PATH_MODES]);
        let ks = self.densities(&qs);
        let ko = self.densities(&qo);
        let lanes = match self.effective_lanes {
            Some(lanes) => lanes,
            None => self.determine_number_of_effective_lanes(),
        };

        let p_tds = match lanes {
            2 => {
                // HCM Equations 24-17, 24-20, and 24-33: 25 modal pairs.
                let mut product = 1.0;
                for i in 0..NUM_PATH_MODES {
                    let p_i = REQUIRED_PASSING_DISTANCE_FT[i];
                    let p_ns = probability_blocked(p_i, ks[i]);
                    for j in 0..NUM_PATH_MODES {
                        let p_no = probability_blocked(p_i, ko[j]);
                        let p_ds = delayed_passing_probability_two_lane(p_ns, p_no);
                        product *= 1.0 - p_ds;
                    }
                }
                1.0 - product
            }
            3 => {
                // HCM Equations 24-25 to 24-32: mode-aggregated probabilities.
                let (mut p_ns, mut p_no, mut p_bs, mut p_bo) = (0.0, 0.0, 0.0, 0.0);
                for i in 0..NUM_PATH_MODES {
                    let p_i = REQUIRED_PASSING_DISTANCE_FT[i];
                    let any_s = probability_blocked(p_i, ks[i]);
                    let any_o = probability_blocked(p_i, ko[i]);
                    // HCM Equations 24-25 and 24-26
                    let p_bs_i = TWO_LANE_BLOCKING_FREQUENCY[i] * any_s;
                    let p_bo_i = TWO_LANE_BLOCKING_FREQUENCY[i] * any_o;
                    // HCM Equations 24-27 and 24-28
                    p_bs += p_bs_i;
                    p_bo += p_bo_i;
                    // HCM Equations 24-29 to 24-32 (single-lane-only blockage)
                    p_ns += any_s - p_bs_i;
                    p_no += any_o - p_bo_i;
                }
                // HCM Equation 24-23
                let d = if (1.0 - p_ns * p_no).abs() > f64::EPSILON {
                    ((p_bs - p_bo) + (p_ns * p_bo - p_no * p_bs)) / (1.0 - p_ns * p_no)
                } else {
                    0.0
                };
                // HCM Equation 24-24
                ((p_ns * (p_bo + p_no * (1.0 + d)) + p_bs) / (1.0 + p_ns * p_no)).clamp(0.0, 1.0)
            }
            _ => {
                // Four-lane paths: HCM Equations 24-25 and 24-27; P_ds = P_bs.
                let mut p_bs = 0.0;
                for i in 0..NUM_PATH_MODES {
                    let p_i = REQUIRED_PASSING_DISTANCE_FT[i];
                    p_bs += TWO_LANE_BLOCKING_FREQUENCY[i] * probability_blocked(p_i, ks[i]);
                }
                p_bs.clamp(0.0, 1.0)
            }
        };
        self.total_probability_delayed_passing = Some(p_tds);
        p_tds
    }

    /// Step 6: Calculate Delayed Passings per Minute.
    ///
    /// HCM Equation 24-34: `DP_m = A_T × P_Tds × PHF`
    ///
    /// The delayed passing factor was calibrated with peak hour volumes rather
    /// than peak 15-min volumes, so a PHF is applied to convert A_T from peak
    /// 15-min flow rate conditions to hourly conditions.
    pub fn calculate_delayed_passings_per_minute(&mut self) -> f64 {
        let at = match self.active_passings_per_minute {
            Some(at) => at,
            None => self.calculate_active_passings_per_minute(),
        };
        let p_tds = match self.total_probability_delayed_passing {
            Some(p) => p,
            None => self.calculate_probability_of_delayed_passing(),
        };
        let dpm = at * p_tds * self.phf;
        self.delayed_passings_per_minute = Some(dpm);
        dpm
    }

    /// Step 7: Determine BLOS.
    ///
    /// HCM Equation 24-35:
    ///
    /// `BLOS = 5.446 - 0.00809×E - 15.86×RW - 0.287×CL - DP`
    ///
    /// where
    /// - E = weighted events per minute = meetings per minute + 10 × active
    ///   passings per minute,
    /// - RW = reciprocal of path width = 1/path width (ft),
    /// - CL = 1 if the trail has a centerline, 0 otherwise, and
    /// - DP = min(DP_m × 0.5, 1.5).
    ///
    /// The LOS letter is determined from Exhibit 24-5 (see
    /// [`bicycle_los_from_score`]); the Step 8 low-volume adjustment is applied
    /// separately.
    ///
    /// Returns the BLOS score.
    pub fn determine_blos(&mut self) -> f64 {
        let at = match self.active_passings_per_minute {
            Some(at) => at,
            None => self.calculate_active_passings_per_minute(),
        };
        let mt = match self.meetings_per_minute {
            Some(mt) => mt,
            None => self.calculate_meetings_per_minute(),
        };
        let dpm = match self.delayed_passings_per_minute {
            Some(dpm) => dpm,
            None => self.calculate_delayed_passings_per_minute(),
        };
        let e = mt + 10.0 * at;
        self.weighted_events_per_minute = Some(e);
        let rw = if self.path_width > 0.0 { 1.0 / self.path_width } else { 0.0 };
        let cl = if self.has_centerline { 1.0 } else { 0.0 };
        let dp = (dpm * 0.5).min(1.5);
        // HCM Equation 24-35
        let blos = 5.446 - 0.00809 * e - 15.86 * rw - 0.287 * cl - dp;
        self.blos_score = Some(blos);
        self.los = Some(bicycle_los_from_score(blos));
        blos
    }

    /// Step 8: Adjust LOS for Low-Volume Paths.
    ///
    /// Equation 24-35 cannot produce LOS A or B for narrow (e.g., 8-ft) paths,
    /// so paths with very low volumes receive the following adjustments:
    /// - Paths with five or fewer weighted events per minute are assigned LOS A.
    /// - Paths with more than five and up to 10 weighted events per minute are
    ///   assigned LOS B, unless Equation 24-35 would result in LOS A.
    pub fn adjust_los_for_low_volume_paths(&mut self) -> LevelOfService {
        if self.blos_score.is_none() {
            self.determine_blos();
        }
        let e = self.weighted_events_per_minute.unwrap_or(0.0);
        let base = self
            .los
            .unwrap_or(LevelOfService::F);
        let adjusted = if e <= 5.0 {
            LevelOfService::A
        } else if e <= 10.0 && base != LevelOfService::A {
            LevelOfService::B
        } else {
            base
        };
        self.los = Some(adjusted);
        adjusted
    }

    /// Run the complete off-street bicycle facility BLOS methodology (Steps 1-8).
    pub fn analyze(&mut self) -> LevelOfService {
        self.calculate_directional_flow_rates();
        self.calculate_active_passings_per_minute();
        self.calculate_meetings_per_minute();
        self.determine_number_of_effective_lanes();
        self.calculate_probability_of_delayed_passing();
        self.calculate_delayed_passings_per_minute();
        self.determine_blos();
        self.adjust_los_for_low_volume_paths()
    }
}
