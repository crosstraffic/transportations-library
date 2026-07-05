//! # Two-Way STOP-Controlled Intersections (HCM Chapter 20)
//!
//! Implements the HCM 7th Edition Chapter 20 motorized vehicle core
//! methodology (Section 3) plus the pedestrian-impedance extension
//! (Section 4) for two-way STOP-controlled (TWSC) intersections.
//!
//! ## Movement numbering (HCM Exhibit 20-1)
//!
//! | Approach              | Left | Through | Right | U-turn |
//! |-----------------------|------|---------|-------|--------|
//! | Major EB (west leg)   | 1    | 2       | 3     | 1U     |
//! | Major WB (east leg)   | 4    | 5       | 6     | 4U     |
//! | Minor NB (south leg)  | 7    | 8       | 9     | —      |
//! | Minor SB (north leg)  | 10   | 11      | 12    | —      |
//!
//! Pedestrian movements: 13 crosses the west (major) leg, 14 crosses the
//! east (major) leg, 15 crosses the south (minor) leg, 16 crosses the north
//! (minor) leg.
//!
//! For a three-leg (T) intersection the minor street is assumed to be the
//! northbound stem (movements 7 and 9 only), matching the T-intersection
//! panel of HCM Exhibit 20-1.
//!
//! ## Computational steps (HCM Exhibit 20-6)
//!
//! 1. Determine and label movement priorities (implicit in the data model)
//! 2. Convert movement demand volumes to flow rates (Equation 20-1)
//! 3. Determine conflicting flow rates (Equations 20-2 through 20-15,
//!    Exhibits 20-8, 20-10, 20-12, 20-14, and 20-16)
//! 4. Determine critical headways and follow-up headways (Equations 20-16
//!    and 20-17, Exhibits 20-17 and 20-18)
//! 5. Compute potential capacities (Equation 20-18; Equations 20-19 through
//!    20-21 with upstream signals)
//! 6.–9. Compute movement capacities with vehicular and pedestrian
//!    impedance (Equations 20-22 through 20-48 and 20-67 through 20-75)
//! 10. Final capacity adjustments: shared and flared lanes (Equations 20-49
//!    and 20-50) and shared major-street lanes (Equations 20-51 to 20-60)
//! 11. Compute movement control delay (Equation 20-61; Rank 1 delay via
//!    Equations 20-62 and 20-63)
//! 12. Compute approach and intersection control delay (Equations 20-64 and
//!    20-65)
//! 13. Compute 95th percentile queue lengths (Equation 20-66)

use serde::{Deserialize, Serialize};

use crate::hcm::common::delay::{aggregate_control_delay, control_delay_unsignalized};
use crate::hcm::common::gap_acceptance::{
    movement_capacity, pedestrian_blockage_factor, pedestrian_impedance_factor,
    potential_capacity, prob_queue_free, PEDESTRIAN_WALKING_SPEED_FT_S,
};
use crate::hcm::common::los_tables::los_unsignalized;

// ═══════════════════════════════════════════════════════════════════════════════
// Movement identifiers
// ═══════════════════════════════════════════════════════════════════════════════

/// Vehicular movement identifiers per HCM Exhibit 20-1.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Mv {
    /// Movement 1: major-street left turn (EB)
    M1,
    /// Movement 1U: major-street U-turn (EB)
    M1U,
    /// Movement 2: major-street through (EB)
    M2,
    /// Movement 3: major-street right turn (EB)
    M3,
    /// Movement 4: major-street left turn (WB)
    M4,
    /// Movement 4U: major-street U-turn (WB)
    M4U,
    /// Movement 5: major-street through (WB)
    M5,
    /// Movement 6: major-street right turn (WB)
    M6,
    /// Movement 7: minor-street left turn (NB)
    M7,
    /// Movement 8: minor-street through (NB)
    M8,
    /// Movement 9: minor-street right turn (NB)
    M9,
    /// Movement 10: minor-street left turn (SB)
    M10,
    /// Movement 11: minor-street through (SB)
    M11,
    /// Movement 12: minor-street right turn (SB)
    M12,
}

/// All 14 vehicular movements in computation order.
pub const ALL_MOVEMENTS: [Mv; 14] = [
    Mv::M1,
    Mv::M1U,
    Mv::M2,
    Mv::M3,
    Mv::M4,
    Mv::M4U,
    Mv::M5,
    Mv::M6,
    Mv::M7,
    Mv::M8,
    Mv::M9,
    Mv::M10,
    Mv::M11,
    Mv::M12,
];

impl Mv {
    /// Index into the internal movement array.
    pub fn idx(self) -> usize {
        match self {
            Mv::M1 => 0,
            Mv::M1U => 1,
            Mv::M2 => 2,
            Mv::M3 => 3,
            Mv::M4 => 4,
            Mv::M4U => 5,
            Mv::M5 => 6,
            Mv::M6 => 7,
            Mv::M7 => 8,
            Mv::M8 => 9,
            Mv::M9 => 10,
            Mv::M10 => 11,
            Mv::M11 => 12,
            Mv::M12 => 13,
        }
    }

    /// Parse an HCM movement label ("1", "1U", ..., "12").
    pub fn from_label(label: &str) -> Option<Mv> {
        match label {
            "1" => Some(Mv::M1),
            "1U" | "1u" => Some(Mv::M1U),
            "2" => Some(Mv::M2),
            "3" => Some(Mv::M3),
            "4" => Some(Mv::M4),
            "4U" | "4u" => Some(Mv::M4U),
            "5" => Some(Mv::M5),
            "6" => Some(Mv::M6),
            "7" => Some(Mv::M7),
            "8" => Some(Mv::M8),
            "9" => Some(Mv::M9),
            "10" => Some(Mv::M10),
            "11" => Some(Mv::M11),
            "12" => Some(Mv::M12),
        _ => None,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Input structures
// ═══════════════════════════════════════════════════════════════════════════════

/// Turning-movement demand for a TWSC intersection.
///
/// Values are hourly demand volumes (veh/h) if a peak hour factor is
/// supplied on [`Twsc`], or peak 15-min demand flow rates (veh/h) otherwise
/// (HCM Chapter 20, Step 2). Pedestrian movements `v13`–`v16` are pedestrian
/// flow rates (p/h) per HCM Exhibit 20-1.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TwscDemand {
    /// Movement 1: major-street left turn (EB), veh/h
    #[serde(default)]
    pub v1: f64,
    /// Movement 1U: major-street U-turn (EB), veh/h
    #[serde(default)]
    pub v1u: f64,
    /// Movement 2: major-street through (EB), veh/h
    #[serde(default)]
    pub v2: f64,
    /// Movement 3: major-street right turn (EB), veh/h
    #[serde(default)]
    pub v3: f64,
    /// Movement 4: major-street left turn (WB), veh/h
    #[serde(default)]
    pub v4: f64,
    /// Movement 4U: major-street U-turn (WB), veh/h
    #[serde(default)]
    pub v4u: f64,
    /// Movement 5: major-street through (WB), veh/h
    #[serde(default)]
    pub v5: f64,
    /// Movement 6: major-street right turn (WB), veh/h
    #[serde(default)]
    pub v6: f64,
    /// Movement 7: minor-street left turn (NB), veh/h
    #[serde(default)]
    pub v7: f64,
    /// Movement 8: minor-street through (NB), veh/h
    #[serde(default)]
    pub v8: f64,
    /// Movement 9: minor-street right turn (NB), veh/h
    #[serde(default)]
    pub v9: f64,
    /// Movement 10: minor-street left turn (SB), veh/h
    #[serde(default)]
    pub v10: f64,
    /// Movement 11: minor-street through (SB), veh/h
    #[serde(default)]
    pub v11: f64,
    /// Movement 12: minor-street right turn (SB), veh/h
    #[serde(default)]
    pub v12: f64,
    /// Pedestrian movement 13 crossing the west major-street leg, p/h
    #[serde(default)]
    pub v13: f64,
    /// Pedestrian movement 14 crossing the east major-street leg, p/h
    #[serde(default)]
    pub v14: f64,
    /// Pedestrian movement 15 crossing the south minor-street leg, p/h
    #[serde(default)]
    pub v15: f64,
    /// Pedestrian movement 16 crossing the north minor-street leg, p/h
    #[serde(default)]
    pub v16: f64,
}

impl TwscDemand {
    fn volume(&self, m: Mv) -> f64 {
        match m {
            Mv::M1 => self.v1,
            Mv::M1U => self.v1u,
            Mv::M2 => self.v2,
            Mv::M3 => self.v3,
            Mv::M4 => self.v4,
            Mv::M4U => self.v4u,
            Mv::M5 => self.v5,
            Mv::M6 => self.v6,
            Mv::M7 => self.v7,
            Mv::M8 => self.v8,
            Mv::M9 => self.v9,
            Mv::M10 => self.v10,
            Mv::M11 => self.v11,
            Mv::M12 => self.v12,
        }
    }
}

/// Configuration of a major-street right-turn movement, which controls the
/// conflicting-flow factors in HCM Exhibits 20-8, 20-10, 20-14, and 20-16.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum MajorRightTurnConfig {
    /// Right turn shares a lane with the through movement
    #[default]
    Shared,
    /// Right turn has a separate (exclusive) right-turn lane
    Exclusive,
    /// Right turn in a STOP- or YIELD-controlled channelized lane
    Channelized,
}

/// Lane allocation on a minor-street approach.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum MinorLaneConfig {
    /// One lane shared by all movements on the approach
    #[default]
    SingleShared,
    /// Shared left–through lane plus an exclusive right-turn lane
    SharedLeftThroughExclusiveRight,
    /// Exclusive left-turn lane plus a shared through–right lane
    ExclusiveLeftSharedThroughRight,
    /// An exclusive lane for each movement
    Separate,
}

/// Lane configuration of a major-street left-turn movement, controlling the
/// queue-free probability used in the Rank 3/4 impedance chain (HCM Step 7d,
/// Equations 20-29 through 20-34).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum MajorLeftLaneConfig {
    /// Exclusive left-turn lane long enough to store the left-turn queue. The
    /// queue-free probability p_0,j of Equation 20-28 is used unchanged.
    #[default]
    Exclusive,
    /// Left turn shares the adjacent through lane (n_L = 0). The shared-lane
    /// queue-free probability p*_0,j (Equations 20-33/20-34) is substituted
    /// for p_0,j everywhere it enters the impedance products.
    Shared,
    /// Left turn has a short storage pocket holding `storage_veh` vehicles
    /// (n_L > 0). The short-pocket queue-free probability p*_0,j (Equations
    /// 20-29/20-31, the (n_L + 1)-root form) is substituted for p_0,j.
    SharedShortPocket {
        /// Number of vehicles that can be stored in the left-turn pocket (n_L,
        /// HCM Exhibit 20-20).
        storage_veh: u32,
    },
}

/// Median nose width category for major-street U-turns (note a of HCM
/// Exhibit 20-17: narrow < 21 ft, wide >= 21 ft).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum UTurnMedianWidth {
    /// Median nose width >= 21 ft
    #[default]
    Wide,
    /// Median nose width < 21 ft
    Narrow,
}

/// Geometric description of the TWSC intersection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TwscGeometry {
    /// Three-leg (T) intersection with the minor stem assumed northbound.
    #[serde(default)]
    pub is_three_leg: bool,
    /// Number of through lanes per direction on the major street (1–3).
    pub major_lanes_per_direction: u32,
    /// Configuration of major-street right-turn movement 3 (EB approach).
    #[serde(default)]
    pub major_right_turn_eb: MajorRightTurnConfig,
    /// Configuration of major-street right-turn movement 6 (WB approach).
    #[serde(default)]
    pub major_right_turn_wb: MajorRightTurnConfig,
    /// Left-turn lane configuration on the EB major approach (movements 1+1U),
    /// HCM Step 7d. Default `Exclusive` reproduces the exclusive-lane p_0,j of
    /// Equation 20-28; `Shared` / `SharedShortPocket` trigger the p*_0,j
    /// substitution of Equations 20-29 through 20-34.
    #[serde(default)]
    pub major_left_eb: MajorLeftLaneConfig,
    /// Left-turn lane configuration on the WB major approach (movements 4+4U).
    #[serde(default)]
    pub major_left_wb: MajorLeftLaneConfig,
    /// Median nose width category for U-turn base headways (Exhibit 20-17).
    #[serde(default)]
    pub uturn_median_width: UTurnMedianWidth,
    /// Grade of the northbound minor approach (%, integer sign convention of
    /// Equation 20-16: negative for downhill).
    #[serde(default)]
    pub grade_minor_nb_pct: f64,
    /// Grade of the southbound minor approach (%).
    #[serde(default)]
    pub grade_minor_sb_pct: f64,
    /// Lane allocation on the northbound minor approach.
    #[serde(default)]
    pub minor_lanes_nb: MinorLaneConfig,
    /// Lane allocation on the southbound minor approach.
    #[serde(default)]
    pub minor_lanes_sb: MinorLaneConfig,
    /// Number of vehicles that can store in the median for two-stage gap
    /// acceptance by the NB minor-street movements (n_m, Equations 20-37 and
    /// 20-45). `None` (or 0) selects one-stage gap acceptance.
    #[serde(default)]
    pub median_storage_nb: Option<u32>,
    /// Median storage for the SB minor-street movements (n_m).
    #[serde(default)]
    pub median_storage_sb: Option<u32>,
    /// Storage spaces in the flared portion of the NB minor approach (n_R,
    /// Equation 20-50). `None` (or 0) means no flare.
    #[serde(default)]
    pub flare_storage_nb: Option<u32>,
    /// Flare storage on the SB minor approach (n_R).
    #[serde(default)]
    pub flare_storage_sb: Option<u32>,
    /// Width of the lane a minor movement negotiates into, used by the
    /// pedestrian blockage factor (w in Equation 20-67), ft.
    #[serde(default = "default_lane_width")]
    pub lane_width_ft: f64,
}

fn default_lane_width() -> f64 {
    12.0
}

impl Default for TwscGeometry {
    fn default() -> Self {
        TwscGeometry {
            is_three_leg: false,
            major_lanes_per_direction: 1,
            major_right_turn_eb: MajorRightTurnConfig::Shared,
            major_right_turn_wb: MajorRightTurnConfig::Shared,
            major_left_eb: MajorLeftLaneConfig::Exclusive,
            major_left_wb: MajorLeftLaneConfig::Exclusive,
            uturn_median_width: UTurnMedianWidth::Wide,
            grade_minor_nb_pct: 0.0,
            grade_minor_sb_pct: 0.0,
            minor_lanes_nb: MinorLaneConfig::SingleShared,
            minor_lanes_sb: MinorLaneConfig::SingleShared,
            median_storage_nb: None,
            median_storage_sb: None,
            flare_storage_nb: None,
            flare_storage_sb: None,
            lane_width_ft: 12.0,
        }
    }
}

/// Optional per-stage conflicting-flow override, applied after Step 3.
///
/// The HCM text below Exhibits 20-8 through 20-16 states "Values may be
/// modified based on field data"; overrides also allow reproducing the
/// Chapter 32 example problems, whose worked conflicting flows for
/// movements 7, 8, 10, and 11 follow the HCM 6th Edition equation forms
/// rather than the 7th Edition exhibit factors (see `// VERIFY-HCM` note in
/// [`Twsc::step3_conflicting_flows`]).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictingFlowOverride {
    /// HCM movement label ("1", "1U", ..., "12").
    pub movement: String,
    /// "total", "stage1", or "stage2".
    pub stage: String,
    /// Conflicting flow rate, veh/h.
    pub value: f64,
}

/// Analyst-supplied proportion-of-time-blocked inputs (p_b,x) for the
/// upstream-signal potential-capacity calculation (HCM Chapter 20, Step 5b,
/// Equations 20-19 through 20-21; per-movement/per-stage mapping in
/// Exhibit 20-19).
///
/// Each value is the fraction of the analysis period that movement x is
/// blocked by a platoon released from a coordinated upstream signal. When
/// [`Twsc::platoon_blockage`] is `None` (the default) or a movement's value
/// is `0`, Step 5 reduces to Equation 20-18 exactly and results are
/// bit-identical to a run without platooning.
///
/// Per HCM Exhibit 20-19 the U-turn movements reuse their companion
/// left-turn value (movement 1U uses p_b,1; movement 4U uses p_b,4) and the
/// two-stage minor movements take a one-stage total plus per-stage values
/// drawn from the opposing major-street left-turn direction:
///
/// | Movement x | One-stage total | Stage I | Stage II |
/// |------------|-----------------|---------|----------|
/// | 1, 1U      | p_b,1           | NA      | NA       |
/// | 4, 4U      | p_b,4           | NA      | NA       |
/// | 7          | p_b,7           | p_b,4   | p_b,1    |
/// | 8          | p_b,8           | p_b,4   | p_b,1    |
/// | 9          | p_b,9           | NA      | NA       |
/// | 10         | p_b,10          | p_b,1   | p_b,4    |
/// | 11         | p_b,11          | p_b,1   | p_b,4    |
/// | 12         | p_b,12          | NA      | NA       |
///
/// The HCM Chapter 30, Section 3 computation that derives these proportions
/// from upstream signal timing and platoon dispersion (the platoon-dispersion
/// primitives exist in [`crate::hcm::chapter18`] for a future pass) is
/// deferred; supply the values directly, e.g. from the Chapter 30
/// movement-based access-point output (HCM Exhibit 32-12).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PlatoonBlockage {
    /// p_b,1 — major-street EB left turn (also applied to movement 1U).
    #[serde(default)]
    pub pb1: f64,
    /// p_b,4 — major-street WB left turn (also applied to movement 4U).
    #[serde(default)]
    pub pb4: f64,
    /// p_b,7 — minor-street NB left turn (one-stage total).
    #[serde(default)]
    pub pb7: f64,
    /// p_b,8 — minor-street NB through (one-stage total).
    #[serde(default)]
    pub pb8: f64,
    /// p_b,9 — minor-street NB right turn.
    #[serde(default)]
    pub pb9: f64,
    /// p_b,10 — minor-street SB left turn (one-stage total).
    #[serde(default)]
    pub pb10: f64,
    /// p_b,11 — minor-street SB through (one-stage total).
    #[serde(default)]
    pub pb11: f64,
    /// p_b,12 — minor-street SB right turn.
    #[serde(default)]
    pub pb12: f64,
}

impl PlatoonBlockage {
    /// One-stage (total) proportion blocked p_b,x for movement `mv`
    /// (HCM Exhibit 20-19). Returns `None` for Rank 1 movements (2, 3, 5, 6),
    /// which have no capacity of their own. Movements 1U and 4U mirror the
    /// companion left-turn values p_b,1 and p_b,4.
    pub fn total(&self, mv: Mv) -> Option<f64> {
        match mv {
            Mv::M1 | Mv::M1U => Some(self.pb1),
            Mv::M4 | Mv::M4U => Some(self.pb4),
            Mv::M7 => Some(self.pb7),
            Mv::M8 => Some(self.pb8),
            Mv::M9 => Some(self.pb9),
            Mv::M10 => Some(self.pb10),
            Mv::M11 => Some(self.pb11),
            Mv::M12 => Some(self.pb12),
            Mv::M2 | Mv::M3 | Mv::M5 | Mv::M6 => None,
        }
    }

    /// Per-stage (Stage I, Stage II) proportion blocked for the two-stage
    /// minor movements 7, 8, 10, and 11 (HCM Exhibit 20-19). Stage I and
    /// Stage II take the p_b of the opposing major-street left-turn
    /// direction: movements 7 and 8 use (p_b,4, p_b,1); movements 10 and 11
    /// use (p_b,1, p_b,4). Returns `None` for every other movement.
    pub fn stages(&self, mv: Mv) -> Option<(f64, f64)> {
        match mv {
            Mv::M7 | Mv::M8 => Some((self.pb4, self.pb1)),
            Mv::M10 | Mv::M11 => Some((self.pb1, self.pb4)),
            _ => None,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Computed results
// ═══════════════════════════════════════════════════════════════════════════════

/// Per-movement computed values (populated by the step methods).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TwscMovementResult {
    /// Demand flow rate v_x, veh/h (Equation 20-1)
    pub flow_rate: f64,
    /// Conflicting flow rate v_c,x, veh/h (Step 3); for two-stage movements
    /// this is the one-stage total (Stage I + Stage II)
    pub conflicting_flow: Option<f64>,
    /// Stage I conflicting flow, veh/h (two-stage movements only)
    pub conflicting_flow_stage1: Option<f64>,
    /// Stage II conflicting flow, veh/h (two-stage movements only)
    pub conflicting_flow_stage2: Option<f64>,
    /// Critical headway t_c,x, s (Equation 20-16); one-stage value
    pub critical_headway: Option<f64>,
    /// Stage I critical headway, s
    pub critical_headway_stage1: Option<f64>,
    /// Stage II critical headway, s
    pub critical_headway_stage2: Option<f64>,
    /// Follow-up headway t_f,x, s (Equation 20-17)
    pub followup_headway: Option<f64>,
    /// Potential capacity c_p,x, veh/h (Equation 20-18); one-stage value
    pub potential_capacity: Option<f64>,
    /// Stage I potential capacity, veh/h
    pub potential_capacity_stage1: Option<f64>,
    /// Stage II potential capacity, veh/h
    pub potential_capacity_stage2: Option<f64>,
    /// Movement capacity c_m,x, veh/h (after impedance and, for two-stage
    /// movements, the total capacity c_T of Equations 20-39/20-40 and
    /// 20-47/20-48)
    pub movement_capacity: Option<f64>,
    /// Stage I movement capacity, veh/h
    pub movement_capacity_stage1: Option<f64>,
    /// Stage II movement capacity, veh/h
    pub movement_capacity_stage2: Option<f64>,
    /// Control delay, s/veh (Equation 20-61); exclusive-lane movements only
    pub control_delay: Option<f64>,
    /// Level of service (Exhibit 20-2); exclusive-lane movements only
    pub los: Option<char>,
    /// 95th percentile queue, veh (Equation 20-66)
    pub queue_95: Option<f64>,
}

/// A minor-street approach lane (or flared approach) result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TwscLaneResult {
    /// Movements served by the lane.
    pub movements: Vec<Mv>,
    /// Lane demand flow rate, veh/h.
    pub flow_rate: f64,
    /// Lane capacity, veh/h (shared-lane Equation 20-49 or flared-lane
    /// Equation 20-50; movement capacity for an exclusive lane).
    pub capacity: f64,
    /// Control delay, s/veh (Equation 20-61).
    pub control_delay: f64,
    /// Level of service (Exhibit 20-2, with v/c > 1.0 assigned LOS F).
    pub los: char,
    /// 95th percentile queue, veh (Equation 20-66).
    pub queue_95: f64,
}

// ═══════════════════════════════════════════════════════════════════════════════
// Main analysis type
// ═══════════════════════════════════════════════════════════════════════════════

/// HCM Chapter 20 TWSC motorized vehicle analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Twsc {
    /// Turning-movement demand.
    pub demand: TwscDemand,
    /// Intersection geometry.
    pub geometry: TwscGeometry,
    /// Intersection peak hour factor (Equation 20-1). `None` if demand
    /// values are already peak 15-min flow rates.
    #[serde(default)]
    pub phf: Option<f64>,
    /// Analysis period T, h (0.25 h for a 15-min period).
    #[serde(default = "default_analysis_period")]
    pub analysis_period_h: f64,
    /// Percentage of heavy vehicles on all movements (%).
    #[serde(default)]
    pub heavy_vehicle_pct: f64,
    /// Optional conflicting-flow overrides (see [`ConflictingFlowOverride`]).
    #[serde(default)]
    pub conflicting_flow_overrides: Vec<ConflictingFlowOverride>,
    /// Optional proportion-of-time-blocked inputs for coordinated upstream
    /// signals (HCM Step 5b, Equations 20-19 through 20-21, Exhibit 20-19).
    /// `None` (the default) selects the no-platooning Equation 20-18.
    #[serde(default)]
    pub platoon_blockage: Option<PlatoonBlockage>,

    // ── Computed ────────────────────────────────────────────────────────────
    /// Per-movement results, indexed by [`Mv::idx`].
    #[serde(default)]
    pub movements: Vec<TwscMovementResult>,
    /// Northbound minor approach lane results (Steps 10–13).
    #[serde(default)]
    pub lanes_nb: Vec<TwscLaneResult>,
    /// Southbound minor approach lane results (Steps 10–13).
    #[serde(default)]
    pub lanes_sb: Vec<TwscLaneResult>,
    /// Approach control delays [EB, WB, NB, SB], s/veh (Equation 20-64).
    #[serde(default)]
    pub approach_delays: Option<[f64; 4]>,
    /// Intersection control delay, s/veh (Equation 20-65). Note LOS is not
    /// defined for a TWSC intersection as a whole (Exhibit 20-2 note).
    #[serde(default)]
    pub intersection_delay: Option<f64>,
    /// HCM Step 11b Rank 1 delay to major-street through-and-right vehicles
    /// that share a lane with a blocked left turn, `[d_2+3 (EB), d_5+6 (WB)]`,
    /// s/veh (Equations 20-62 and 20-63). `None` when both major-street left
    /// turns have exclusive lanes, in which case Rank 1 delay is 0 s/veh.
    #[serde(default)]
    pub rank1_major_delay: Option<[f64; 2]>,
}

fn default_analysis_period() -> f64 {
    0.25
}

/// Default major-street through saturation flow rate s_2 = s_5 (HCM
/// Equations 20-30/20-32), veh/h.
const MAJOR_THROUGH_SAT_FLOW: f64 = 1_800.0;
/// Default major-street right-turn saturation flow rate s_3 = s_6 (HCM
/// Equations 20-30/20-32), veh/h.
const MAJOR_RIGHT_SAT_FLOW: f64 = 1_500.0;

impl Twsc {
    /// Create a new analysis from demand and geometry.
    pub fn new(demand: TwscDemand, geometry: TwscGeometry) -> Self {
        Twsc {
            demand,
            geometry,
            phf: None,
            analysis_period_h: 0.25,
            heavy_vehicle_pct: 0.0,
            conflicting_flow_overrides: Vec::new(),
            platoon_blockage: None,
            movements: (0..14).map(|_| TwscMovementResult::default()).collect(),
            lanes_nb: Vec::new(),
            lanes_sb: Vec::new(),
            approach_delays: None,
            intersection_delay: None,
            rank1_major_delay: None,
        }
    }

    /// Deserialize from JSON (fixture format used in `tests/ExampleCases`).
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        let mut twsc: Twsc = serde_json::from_str(json)?;
        if twsc.movements.len() != 14 {
            twsc.movements = (0..14).map(|_| TwscMovementResult::default()).collect();
        }
        Ok(twsc)
    }

    /// Serialize the full analysis (inputs + results) to JSON.
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    fn m(&self, mv: Mv) -> &TwscMovementResult {
        &self.movements[mv.idx()]
    }

    fn m_mut(&mut self, mv: Mv) -> &mut TwscMovementResult {
        &mut self.movements[mv.idx()]
    }

    /// Flow rate of movement `mv`, veh/h (0.0 before Step 2).
    pub fn get_flow_rate(&self, mv: Mv) -> f64 {
        self.m(mv).flow_rate
    }

    /// Movement capacity of `mv`, veh/h (`None` before Steps 6–9).
    pub fn get_movement_capacity(&self, mv: Mv) -> Option<f64> {
        self.m(mv).movement_capacity
    }

    /// Proportion of heavy vehicles (decimal) for use in Equations 20-16
    /// and 20-17.
    fn phv(&self) -> f64 {
        self.heavy_vehicle_pct / 100.0
    }

    /// Number of major-street through lanes per direction N.
    fn n_major(&self) -> u32 {
        self.geometry.major_lanes_per_direction
    }

    /// Factor f_LL estimating the portion of major-street through and
    /// right-turn traffic using the left lane (HCM Equations 20-30 and 20-32):
    /// 1.0 for one through lane, and the default 0.5 for two / 0.33 for three.
    fn f_ll_major(&self) -> f64 {
        match self.n_major() {
            1 => 1.0,
            2 => 0.5,
            _ => 0.33,
        }
    }

    /// Vehicles storable in the major-street left-turn pocket (n_L) for the
    /// p*_0,j substitution, or `None` when the left turn has an exclusive lane
    /// (in which case the exclusive-lane p_0,j of Equation 20-28 is kept).
    fn major_left_storage(cfg: MajorLeftLaneConfig) -> Option<u32> {
        match cfg {
            MajorLeftLaneConfig::Exclusive => None,
            MajorLeftLaneConfig::Shared => Some(0),
            MajorLeftLaneConfig::SharedShortPocket { storage_veh } => Some(storage_veh),
        }
    }

    /// Combined degree of saturation x_2+3 (EB) or x_5+6 (WB) of the
    /// major-street through and right-turn movements (HCM Equations 20-30 and
    /// 20-32), using default saturation flow rates s = 1,800 veh/h (through)
    /// and 1,500 veh/h (right turn).
    fn major_through_saturation(&self, eb: bool) -> f64 {
        let (through, right) = if eb {
            (Mv::M2, Mv::M3)
        } else {
            (Mv::M5, Mv::M6)
        };
        let v_t = self.m(through).flow_rate;
        let v_r = self.m(right).flow_rate;
        self.f_ll_major() * (v_t / MAJOR_THROUGH_SAT_FLOW + v_r / MAJOR_RIGHT_SAT_FLOW)
    }

    /// HCM Step 7d: shared/short-pocket queue-free probability p*_0,j for a
    /// major-street left turn (Equations 20-29/20-31, reducing to 20-33/20-34
    /// for a shared lane with n_L = 0), to be substituted for the
    /// exclusive-lane p_0,j (Equation 20-28) in every Rank 3/4 impedance
    /// product and in the Step 11b Rank 1 delay. Returns `None` when the
    /// approach has an exclusive left-turn lane.
    ///
    /// * `eb` — true for the EB approach (movements 1+1U), false for WB (4+4U)
    /// * `p0_exclusive` — exclusive-lane p_0,j from [`Twsc::p0_major_left`]
    fn p0_star_major_left(&self, eb: bool, p0_exclusive: f64) -> Option<f64> {
        let cfg = if eb {
            self.geometry.major_left_eb
        } else {
            self.geometry.major_left_wb
        };
        let n_l = Self::major_left_storage(cfg)?;
        let x_23 = self.major_through_saturation(eb);
        Some(Self::prob_queue_free_shared_major(p0_exclusive, x_23, n_l))
    }

    /// Whether a movement is two-stage per the geometry configuration.
    fn is_two_stage(&self, mv: Mv) -> bool {
        let nm = match mv {
            Mv::M7 | Mv::M8 => self.geometry.median_storage_nb.unwrap_or(0),
            Mv::M10 | Mv::M11 => self.geometry.median_storage_sb.unwrap_or(0),
            _ => 0,
        };
        nm > 0 && !self.geometry.is_three_leg
    }

    // ═══════════════════════════════════════════════════════════════════════
    // Steps 1 and 2: movement priorities and demand flow rates
    // ═══════════════════════════════════════════════════════════════════════

    /// HCM Chapter 20, Steps 1 and 2 (Equation 20-1):
    /// `v_i = V_i / PHF`.
    ///
    /// Populates `flow_rate` on every movement result and returns the total
    /// intersection flow rate, veh/h. If `phf` is `None`, volumes are
    /// treated as flow rates.
    pub fn step1_2_demand_flow_rates(&mut self) -> f64 {
        let phf = match self.phf {
            Some(p) if p > 0.0 => p,
            _ => 1.0,
        };
        let mut total = 0.0;
        for mv in ALL_MOVEMENTS {
            let v = self.demand.volume(mv) / phf;
            self.movements[mv.idx()].flow_rate = v;
            total += v;
        }
        total
    }

    // ═══════════════════════════════════════════════════════════════════════
    // Step 3: conflicting flow rates
    // ═══════════════════════════════════════════════════════════════════════

    /// Conflicting flow factor for the major-street through movement seen by
    /// minor-street right turns (HCM Exhibit 20-10): 1 for one lane, 0.5 for
    /// two or three lanes.
    fn f_minor_rt_vs_major_through(&self) -> f64 {
        if self.n_major() == 1 {
            1.0
        } else {
            0.5
        }
    }

    /// Conflicting flow factor for a major-street right turn in
    /// shared/exclusive configuration (HCM Exhibits 20-10, 20-14, and 20-16
    /// "Shared lane 0.5 / Separate right-turn lane 0").
    fn f_shared_half(cfg: MajorRightTurnConfig) -> f64 {
        match cfg {
            MajorRightTurnConfig::Shared => 0.5,
            MajorRightTurnConfig::Exclusive | MajorRightTurnConfig::Channelized => 0.0,
        }
    }

    /// Conflicting flow factor for a major-street right turn per HCM
    /// Exhibits 20-8, 20-14, and 20-16 rows "STOP- or YIELD-controlled
    /// channelized lane 0 / All others 1".
    fn f_channelized_zero(cfg: MajorRightTurnConfig) -> f64 {
        match cfg {
            MajorRightTurnConfig::Channelized => 0.0,
            _ => 1.0,
        }
    }

    /// Conflicting flow factor for the major-street through movement seen by
    /// Stage II of the minor-street left turn (HCM Exhibit 20-16): 1 for one
    /// lane, 0.5 for two lanes, 0.4 for three lanes.
    fn f_minor_lt_vs_major_through(&self) -> f64 {
        match self.n_major() {
            1 => 1.0,
            2 => 0.5,
            _ => 0.4,
        }
    }

    /// Conflicting flow factor for the opposing major through movement seen
    /// by major-street U-turns (HCM Exhibit 20-12): 1 for two through lanes,
    /// 0.73 for three through lanes.
    fn f_uturn_vs_major_through(&self) -> f64 {
        if self.n_major() >= 3 {
            0.73
        } else {
            1.0
        }
    }

    /// Conflicting flow factor for the opposing major right turn seen by
    /// major-street U-turns (HCM Exhibit 20-12): shared right turn counts at
    /// the through factor, separate right-turn lane counts 0.
    fn f_uturn_vs_major_right(&self, cfg: MajorRightTurnConfig) -> f64 {
        match cfg {
            MajorRightTurnConfig::Shared => self.f_uturn_vs_major_through(),
            _ => 0.0,
        }
    }

    /// HCM Chapter 20, Step 3: determine conflicting flow rates
    /// (Equations 20-2 through 20-15 with the default conflicting flow
    /// factors of Exhibits 20-8, 20-10, 20-12, 20-14, and 20-16).
    ///
    /// Pedestrian movements 13–16 enter the sums with a factor of 1 per the
    /// exhibits. Two-stage movements additionally receive Stage I and
    /// Stage II conflicting flows; the one-stage value is their sum.
    ///
    /// Any [`ConflictingFlowOverride`]s are applied afterwards.
    // VERIFY-HCM: the Chapter 32 TWSC Example Problem 3 worked values apply
    // the HCM 6th Edition equation forms for movements 8/11 (v6 or v3 with
    // factor 1.0 in Stage II/Stage I of the crossing movement instead of the
    // Exhibit 20-14 shared-lane factor 0.5) and movements 7/10 (one-half of
    // the *opposing minor-street through* flow in Stage II instead of the
    // Exhibit 20-16 factor 0.5 on the major-street right turn). This
    // implementation follows the 7th Edition exhibits; use
    // `conflicting_flow_overrides` to reproduce the published example.
    // The same class of discrepancy appears in Example Problem 4: its
    // published v_c,7 = 1,827 and v_c,10 = 1,832 drop the major-street
    // right-turn term (0.5 v_6 for movement 7, 0.5 v_3 for movement 10) in
    // Stage II, where the Exhibit 20-16 factors used here give 1,874 and
    // 1,879; case3.json overrides the totals to match.
    pub fn step3_conflicting_flows(&mut self) {
        let d = &self.demand;
        let (v13, v14, v15, v16) = (d.v13, d.v14, d.v15, d.v16);
        let f = |mv: Mv| self.m(mv).flow_rate;
        let (v1, v1u, v2, v3) = (f(Mv::M1), f(Mv::M1U), f(Mv::M2), f(Mv::M3));
        let (v4, v4u, v5, v6) = (f(Mv::M4), f(Mv::M4U), f(Mv::M5), f(Mv::M6));
        let v9 = f(Mv::M9);
        let v12 = f(Mv::M12);
        let cfg_eb = self.geometry.major_right_turn_eb;
        let cfg_wb = self.geometry.major_right_turn_wb;

        // HCM Equation 20-2 (Exhibit 20-8): v_c,1 = v5 + f(1,6) v6 + v16
        let vc1 = v5 + Self::f_channelized_zero(cfg_wb) * v6 + v16;
        // HCM Equation 20-3 (Exhibit 20-8): v_c,4 = v2 + f(4,3) v3 + v15
        let vc4 = v2 + Self::f_channelized_zero(cfg_eb) * v3 + v15;

        // HCM Equation 20-4 (Exhibit 20-10):
        // v_c,9 = f(9,2) v2 + f(9,3) v3 + 0 v4U + v14 + v15
        let vc9 = self.f_minor_rt_vs_major_through() * v2
            + Self::f_shared_half(cfg_eb) * v3
            + v14
            + v15;
        // HCM Equation 20-5 (Exhibit 20-10):
        // v_c,12 = 0 v1U + f(12,5) v5 + f(12,6) v6 + v13 + v16
        let vc12 = self.f_minor_rt_vs_major_through() * v5
            + Self::f_shared_half(cfg_wb) * v6
            + v13
            + v16;

        // HCM Equations 20-6 and 20-7 (Exhibit 20-12):
        // v_c,1U = f(1U,5) v5 + f(1U,6) v6 + 0 v12
        let vc1u = self.f_uturn_vs_major_through() * v5 + self.f_uturn_vs_major_right(cfg_wb) * v6;
        // v_c,4U = f(4U,2) v2 + f(4U,3) v3 + 0 v9
        let vc4u = self.f_uturn_vs_major_through() * v2 + self.f_uturn_vs_major_right(cfg_eb) * v3;

        // HCM Equations 20-8 and 20-10 (Exhibit 20-14), minor through 8:
        // Stage I:  v_c,I,8  = 2 v1 + 2 v1U + v2 + f(8,3) v3 + v15
        // Stage II: v_c,II,8 = 2 v4 + 2 v4U + v5 + f(8,6) v6 + v16
        let vc8_s1 = 2.0 * v1 + 2.0 * v1u + v2 + Self::f_shared_half(cfg_eb) * v3 + v15;
        let vc8_s2 = 2.0 * v4 + 2.0 * v4u + v5 + Self::f_shared_half(cfg_wb) * v6 + v16;

        // HCM Equations 20-9 and 20-11 (Exhibit 20-14), minor through 11:
        // Stage I:  v_c,I,11  = 2 v4 + 2 v4U + v5 + f(11,6) v6 + v16
        // Stage II: v_c,II,11 = 2 v1 + 2 v1U + v2 + f(11,3) v3 + v15
        let vc11_s1 = 2.0 * v4 + 2.0 * v4u + v5 + Self::f_channelized_zero(cfg_wb) * v6 + v16;
        let vc11_s2 = 2.0 * v1 + 2.0 * v1u + v2 + Self::f_channelized_zero(cfg_eb) * v3 + v15;

        // HCM Equations 20-12 and 20-14 (Exhibit 20-16), minor left 7:
        // Stage I:  v_c,I,7  = 2 v1 + 2 v1U + v2 + f(7,3) v3 + v15
        // Stage II: v_c,II,7 = 2 v4 + 2 v4U + f(7,5) v5 + f(7,6) v6 + v13
        let vc7_s1 = 2.0 * v1 + 2.0 * v1u + v2 + Self::f_shared_half(cfg_eb) * v3 + v15;
        let vc7_s2 = 2.0 * v4
            + 2.0 * v4u
            + self.f_minor_lt_vs_major_through() * v5
            + (if cfg_wb == MajorRightTurnConfig::Channelized {
                0.0
            } else {
                0.5
            }) * v6
            + v13;

        // HCM Equations 20-13 and 20-15 (Exhibit 20-16), minor left 10:
        // Stage I:  v_c,I,10  = 2 v4 + 2 v4U + v5 + f(10,6) v6 + v16
        // Stage II: v_c,II,10 = 2 v1 + 2 v1U + f(10,2) v2 + f(10,3) v3 + v14
        let vc10_s1 = 2.0 * v4 + 2.0 * v4u + v5 + Self::f_shared_half(cfg_wb) * v6 + v16;
        let vc10_s2 = 2.0 * v1
            + 2.0 * v1u
            + self.f_minor_lt_vs_major_through() * v2
            + (if cfg_eb == MajorRightTurnConfig::Channelized {
                0.0
            } else {
                0.5
            }) * v3
            + v14;

        self.m_mut(Mv::M1).conflicting_flow = Some(vc1);
        self.m_mut(Mv::M4).conflicting_flow = Some(vc4);
        self.m_mut(Mv::M9).conflicting_flow = Some(vc9);
        self.m_mut(Mv::M12).conflicting_flow = Some(vc12);
        self.m_mut(Mv::M1U).conflicting_flow = Some(vc1u);
        self.m_mut(Mv::M4U).conflicting_flow = Some(vc4u);

        for (mv, s1, s2) in [
            (Mv::M8, vc8_s1, vc8_s2),
            (Mv::M11, vc11_s1, vc11_s2),
            (Mv::M7, vc7_s1, vc7_s2),
            (Mv::M10, vc10_s1, vc10_s2),
        ] {
            let r = self.m_mut(mv);
            r.conflicting_flow_stage1 = Some(s1);
            r.conflicting_flow_stage2 = Some(s2);
            r.conflicting_flow = Some(s1 + s2);
        }
        let _ = (v9, v12); // conflicting factor 0 per Exhibits 20-10/20-12

        // Apply overrides (stage overrides refresh the one-stage total).
        let overrides = self.conflicting_flow_overrides.clone();
        for ov in overrides {
            if let Some(mv) = Mv::from_label(&ov.movement) {
                {
                    let r = self.m_mut(mv);
                    match ov.stage.as_str() {
                        "stage1" => r.conflicting_flow_stage1 = Some(ov.value),
                        "stage2" => r.conflicting_flow_stage2 = Some(ov.value),
                        _ => r.conflicting_flow = Some(ov.value),
                    }
                }
                let r = self.m_mut(mv);
                if let (Some(s1), Some(s2)) =
                    (r.conflicting_flow_stage1, r.conflicting_flow_stage2)
                {
                    if ov.stage != "total" {
                        r.conflicting_flow = Some(s1 + s2);
                    }
                }
            }
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    // Step 4: critical headways and follow-up headways
    // ═══════════════════════════════════════════════════════════════════════

    /// Base critical headway t_c,base, s (HCM Exhibit 20-17), by movement
    /// and major-street cross section (2N total lanes). `stage` is `None`
    /// for one-stage values, `Some(1)`/`Some(2)` for two-stage values.
    fn base_critical_headway(&self, mv: Mv, stage: Option<u8>) -> f64 {
        let lanes = self.n_major(); // per direction: 1 => "Two Lanes", etc.
        match mv {
            // Left turn from major street: 4.1 / 4.1 / 5.3
            Mv::M1 | Mv::M4 => match lanes {
                1 | 2 => 4.1,
                _ => 5.3,
            },
            // U-turn from major street: NA / 6.4 (wide) 6.9 (narrow) / 5.6
            Mv::M1U | Mv::M4U => match lanes {
                2 => match self.geometry.uturn_median_width {
                    UTurnMedianWidth::Wide => 6.4,
                    UTurnMedianWidth::Narrow => 6.9,
                },
                3 => 5.6,
                // VERIFY-HCM: Exhibit 20-17 lists "NA" for U-turns on
                // two-lane major streets; the four-lane value is used as a
                // fallback if a U-turn is coded there.
                _ => 6.4,
            },
            // Right turn from minor street: 6.2 / 6.9 / 7.1
            Mv::M9 | Mv::M12 => match lanes {
                1 => 6.2,
                2 => 6.9,
                _ => 7.1,
            },
            // Through traffic on minor street: 1-stage 6.5; Stage I/II 5.5
            // (all cross sections; six-lane values flagged as estimated)
            Mv::M8 | Mv::M11 => match stage {
                None => 6.5,
                Some(_) => 5.5,
            },
            // Left turn from minor street:
            //   two lanes: 7.1 (1-stage), 6.1/6.1 (Stage I/II)
            //   four lanes: 7.5, 6.5/6.5
            //   six lanes: 6.4, 7.3 (Stage I), 6.7 (Stage II)
            Mv::M7 | Mv::M10 => match (lanes, stage) {
                (1, None) => 7.1,
                (1, Some(_)) => 6.1,
                (2, None) => 7.5,
                (2, Some(_)) => 6.5,
                (_, None) => 6.4,
                (_, Some(1)) => 7.3,
                (_, Some(_)) => 6.7,
            },
            // Rank 1 movements have no critical headway
            Mv::M2 | Mv::M3 | Mv::M5 | Mv::M6 => 0.0,
        }
    }

    /// Base follow-up headway t_f,base, s (HCM Exhibit 20-18).
    fn base_followup_headway(&self, mv: Mv) -> f64 {
        let lanes = self.n_major();
        match mv {
            Mv::M1 | Mv::M4 => match lanes {
                1 | 2 => 2.2,
                _ => 3.1,
            },
            Mv::M1U | Mv::M4U => match lanes {
                2 => match self.geometry.uturn_median_width {
                    UTurnMedianWidth::Wide => 2.5,
                    UTurnMedianWidth::Narrow => 3.1,
                },
                3 => 2.3,
                _ => 2.5, // VERIFY-HCM: NA in Exhibit 20-18 for two lanes
            },
            Mv::M9 | Mv::M12 => match lanes {
                1 | 2 => 3.3,
                _ => 3.9,
            },
            Mv::M8 | Mv::M11 => 4.0,
            Mv::M7 | Mv::M10 => match lanes {
                1 | 2 => 3.5,
                _ => 3.8,
            },
            Mv::M2 | Mv::M3 | Mv::M5 | Mv::M6 => 0.0,
        }
    }

    /// Grade adjustment factor t_c,G, s (Equation 20-16 definitions: 0.1 for
    /// movements 9 and 12; 0.2 for movements 7, 8, 10, and 11).
    fn tc_grade_factor(mv: Mv) -> f64 {
        match mv {
            Mv::M9 | Mv::M12 => 0.1,
            Mv::M7 | Mv::M8 | Mv::M10 | Mv::M11 => 0.2,
            _ => 0.0,
        }
    }

    /// Approach grade (%) applicable to a movement.
    fn grade_for(&self, mv: Mv) -> f64 {
        match mv {
            Mv::M7 | Mv::M8 | Mv::M9 => self.geometry.grade_minor_nb_pct,
            Mv::M10 | Mv::M11 | Mv::M12 => self.geometry.grade_minor_sb_pct,
            _ => 0.0,
        }
    }

    /// HCM Equation 20-16: critical headway for movement x
    ///
    /// `t_c,x = t_c,base + t_c,HV P_HV + t_c,G G - t_3,LT`
    pub fn critical_headway(&self, mv: Mv, stage: Option<u8>) -> f64 {
        let tc_base = self.base_critical_headway(mv, stage);
        // t_c,HV: 1.0 for one-lane majors, 2.0 for two/three-lane majors
        let tc_hv = if self.n_major() == 1 { 1.0 } else { 2.0 };
        let tc_g = Self::tc_grade_factor(mv);
        // t_3,LT: 0.7 for minor-street left turn at three-leg intersections
        let t3_lt = if self.geometry.is_three_leg && (mv == Mv::M7 || mv == Mv::M10) {
            0.7
        } else {
            0.0
        };
        tc_base + tc_hv * self.phv() + tc_g * self.grade_for(mv) - t3_lt
    }

    /// HCM Equation 20-17: follow-up headway for movement x
    ///
    /// `t_f,x = t_f,base + t_f,HV P_HV`
    pub fn followup_headway(&self, mv: Mv) -> f64 {
        let tf_base = self.base_followup_headway(mv);
        // t_f,HV: 0.9 for one-lane majors, 1.0 for two/three-lane majors
        let tf_hv = if self.n_major() == 1 { 0.9 } else { 1.0 };
        tf_base + tf_hv * self.phv()
    }

    /// HCM Chapter 20, Step 4: populate critical and follow-up headways for
    /// all Rank 2–4 movements (Equations 20-16 and 20-17, Exhibits 20-17 and
    /// 20-18).
    pub fn step4_headways(&mut self) {
        for mv in ALL_MOVEMENTS {
            if matches!(mv, Mv::M2 | Mv::M3 | Mv::M5 | Mv::M6) {
                continue; // Rank 1
            }
            let tc = self.critical_headway(mv, None);
            let tf = self.followup_headway(mv);
            let two_stage = self.is_two_stage(mv);
            let tc1 = self.critical_headway(mv, Some(1));
            let tc2 = self.critical_headway(mv, Some(2));
            let r = self.m_mut(mv);
            r.critical_headway = Some(tc);
            r.followup_headway = Some(tf);
            if two_stage {
                r.critical_headway_stage1 = Some(tc1);
                r.critical_headway_stage2 = Some(tc2);
            }
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    // Step 5: potential capacities
    // ═══════════════════════════════════════════════════════════════════════

    /// HCM Chapter 20, Step 5 (Equation 20-18, or Equations 20-19 through
    /// 20-21 when [`Twsc::platoon_blockage`] is supplied): potential capacity
    /// of every Rank 2–4 movement, plus Stage I/II potential capacities for
    /// two-stage movements.
    ///
    /// With no platoon-blockage inputs (Step 5a) each capacity is the plain
    /// Equation 20-18 gap-acceptance value. With inputs (Step 5b) each
    /// movement's p_b,x (and, for two-stage movements, its Stage I/II p_b per
    /// Exhibit 20-19) selects the unblocked-period conflicting flow of
    /// Equation 20-19 and the platooned capacity of Equations 20-20/20-21. A
    /// movement whose p_b is `0` falls back to Equation 20-18, so a
    /// zero-filled [`PlatoonBlockage`] is equivalent to `None`.
    pub fn step5_potential_capacities(&mut self) {
        let n = self.n_major();
        let pb = self.platoon_blockage.clone();
        // Potential capacity of one conflict period: Equation 20-18 when
        // p_b is absent or zero, otherwise the platooned Step 5b value.
        let cap = |vc: f64, tc: f64, tf: f64, pbx: Option<f64>| -> f64 {
            match pbx {
                Some(pbx) if pbx > 0.0 => {
                    Self::potential_capacity_upstream_signal(vc, tc, tf, pbx, n)
                }
                _ => potential_capacity(vc, tc, tf),
            }
        };
        for mv in ALL_MOVEMENTS {
            if matches!(mv, Mv::M2 | Mv::M3 | Mv::M5 | Mv::M6) {
                continue;
            }
            let r = self.m(mv);
            let (vc, tf) = (r.conflicting_flow, r.followup_headway);
            let (vc, tf) = match (vc, tf) {
                (Some(vc), Some(tf)) => (vc, tf),
                _ => continue,
            };
            let tc = r.critical_headway.unwrap_or(0.0);
            let pbx = pb.as_ref().and_then(|p| p.total(mv));
            let cp = cap(vc, tc, tf, pbx);
            let two_stage = self.is_two_stage(mv);
            let s = if two_stage {
                let s1 = self.m(mv).conflicting_flow_stage1.unwrap_or(0.0);
                let s2 = self.m(mv).conflicting_flow_stage2.unwrap_or(0.0);
                let tc1 = self.m(mv).critical_headway_stage1.unwrap_or(tc);
                let tc2 = self.m(mv).critical_headway_stage2.unwrap_or(tc);
                let (pb1, pb2) = match pb.as_ref().and_then(|p| p.stages(mv)) {
                    Some((p1, p2)) => (Some(p1), Some(p2)),
                    None => (None, None),
                };
                Some((cap(s1, tc1, tf, pb1), cap(s2, tc2, tf, pb2)))
            } else {
                None
            };
            let r = self.m_mut(mv);
            r.potential_capacity = Some(cp);
            if let Some((cp1, cp2)) = s {
                r.potential_capacity_stage1 = Some(cp1);
                r.potential_capacity_stage2 = Some(cp2);
            }
        }
    }

    /// HCM Chapter 20, Step 5b (Equations 20-19 through 20-21): potential
    /// capacity accounting for platooning from coordinated upstream signals.
    ///
    /// * `vc` — total conflicting flow from Step 3, veh/h
    /// * `tc`, `tf` — critical and follow-up headway, s
    /// * `pb` — proportion of the analysis period the movement is blocked by
    ///   a platoon (p_b,x, Exhibit 20-19; from the Chapter 30 procedure)
    /// * `n_major` — number of through lanes per direction on the major
    ///   street (v_c,min = 1,000 N)
    ///
    /// Returns the potential capacity c_p,x, veh/h.
    pub fn potential_capacity_upstream_signal(
        vc: f64,
        tc: f64,
        tf: f64,
        pb: f64,
        n_major: u32,
    ) -> f64 {
        let vc_min = 1_000.0 * n_major as f64;
        // Equation 20-19
        let vc_u = if vc > 1.5 * vc_min * pb {
            (vc - 1.5 * vc_min * pb) / (1.0 - pb)
        } else {
            0.0
        };
        // Equation 20-21
        let cr = if vc_u > 0.0 {
            potential_capacity(vc_u, tc, tf)
        } else {
            3_600.0 / tf
        };
        // Equation 20-20
        (1.0 - pb) * cr
    }

    // ═══════════════════════════════════════════════════════════════════════
    // Steps 6–9: movement capacities (with pedestrian impedance)
    // ═══════════════════════════════════════════════════════════════════════

    /// HCM Equations 20-67 and 20-68: pedestrian impedance factor p_p,x for
    /// pedestrian movement x (13–16).
    fn ped_impedance(&self, v_ped: f64) -> f64 {
        pedestrian_impedance_factor(pedestrian_blockage_factor(
            v_ped,
            self.geometry.lane_width_ft,
            PEDESTRIAN_WALKING_SPEED_FT_S,
        ))
    }

    /// HCM Equation 20-27 / 20-49 shared-lane capacity
    /// `c_SH = Σ v_y / Σ (v_y / c_m,y)` for the given (flow, capacity) pairs.
    pub fn shared_lane_capacity(flows_and_capacities: &[(f64, f64)]) -> f64 {
        let total_v: f64 = flows_and_capacities.iter().map(|(v, _)| v).sum();
        if total_v <= 0.0 {
            // Empty shared lane: report the minimum movement capacity
            return flows_and_capacities
                .iter()
                .map(|(_, c)| *c)
                .fold(f64::INFINITY, f64::min);
        }
        let denom: f64 = flows_and_capacities
            .iter()
            .filter(|(v, _)| *v > 0.0)
            .map(|(v, c)| v / c)
            .sum();
        total_v / denom
    }

    /// Probability of queue-free state for the combined major-street left
    /// turn + U-turn (Equation 20-28 with j = 1+1U or 4+4U, using
    /// shared-lane volume and capacity per Equation 20-27).
    fn p0_major_left(&self, left: Mv, uturn: Mv) -> f64 {
        let v_l = self.m(left).flow_rate;
        let v_u = self.m(uturn).flow_rate;
        let c_l = self.m(left).movement_capacity;
        let c_u = self.m(uturn).movement_capacity;
        match (c_l, c_u) {
            (Some(cl), Some(cu)) if v_u > 0.0 => {
                let c_sh = Self::shared_lane_capacity(&[(v_l, cl), (v_u, cu)]);
                prob_queue_free(v_l + v_u, c_sh)
            }
            (Some(cl), _) => prob_queue_free(v_l, cl),
            _ => 1.0,
        }
    }

    /// HCM Equation 20-37 / 20-45: two-stage adjustment factor
    /// `a = 1 - 0.32 e^(-1.3 sqrt(n_m))` for n_m > 0.
    ///
    /// (The exponent carries sqrt(n_m) per the EPUB MathML; the Chapter 32
    /// Example Problem 3 value a = 1 - 0.32 e^(-1.3 sqrt(2)) = 0.949
    /// confirms it.)
    pub fn two_stage_adjustment_a(n_m: u32) -> f64 {
        1.0 - 0.32 * (-1.3 * (n_m as f64).sqrt()).exp()
    }

    /// HCM Equations 20-38/20-39/20-40 (Rank 3) and 20-46/20-47/20-48
    /// (Rank 4): total capacity c_T for a two-stage movement.
    ///
    /// * `c_stage1` — Stage I movement capacity c_I, veh/h
    /// * `c_stage2` — Stage II movement capacity c_II, veh/h
    /// * `c_one_stage` — one-stage movement capacity c_m,x, veh/h
    /// * `v_major_left` — conflicting major left + U-turn flow v_L, veh/h
    /// * `n_m` — median storage, veh
    pub fn two_stage_total_capacity(
        c_stage1: f64,
        c_stage2: f64,
        c_one_stage: f64,
        v_major_left: f64,
        n_m: u32,
    ) -> f64 {
        let a = Self::two_stage_adjustment_a(n_m);
        let nm = n_m as f64;
        // Equation 20-38 / 20-46
        let y = (c_stage1 - c_one_stage) / (c_stage2 - v_major_left - c_one_stage);
        if (y - 1.0).abs() < 1e-9 {
            // Equation 20-40 / 20-48
            a / (nm + 1.0) * (nm * (c_stage2 - v_major_left) + c_one_stage)
        } else {
            // Equation 20-39 / 20-47
            a / (y.powf(nm + 1.0) - 1.0)
                * (y * (y.powf(nm) - 1.0) * (c_stage2 - v_major_left) + (y - 1.0) * c_one_stage)
        }
    }

    /// HCM Chapter 20, Steps 6 through 9: movement capacities for Rank 2,
    /// Rank 3, and Rank 4 movements, including vehicular impedance
    /// (Equations 20-22 through 20-48) and pedestrian impedance
    /// (Equations 20-67 through 20-75).
    pub fn step6_9_movement_capacities(&mut self) {
        // Pedestrian impedance factors (Equations 20-67 and 20-68)
        let pp13 = self.ped_impedance(self.demand.v13);
        let pp14 = self.ped_impedance(self.demand.v14);
        let pp15 = self.ped_impedance(self.demand.v15);
        let pp16 = self.ped_impedance(self.demand.v16);

        // ── Step 7a/7b: Rank 2 major-street left turns (Equations 20-22 and
        //    20-69: c_m = c_p × p_p,i) and minor-street right turns
        //    (Equations 20-23 and 20-70 through 20-72).
        let assign = |t: &mut Twsc, mv: Mv, imp: f64| {
            if let Some(cp) = t.m(mv).potential_capacity {
                t.m_mut(mv).movement_capacity = Some(movement_capacity(cp, imp));
            }
        };
        assign(self, Mv::M1, pp16); // Exhibit 20-22: v1 yields to v16
        assign(self, Mv::M4, pp15); // Exhibit 20-22: v4 yields to v15
        assign(self, Mv::M9, pp15 * pp14); // Equation 20-70
        assign(self, Mv::M12, pp16 * pp13); // Equation 20-71

        // ── Step 7c: major-street U-turns (Equations 20-24 through 20-26).
        // f_1U = p_0,12 and f_4U = p_0,9 (queue-free minor right turns).
        let p0_12 = self
            .m(Mv::M12)
            .movement_capacity
            .map(|c| prob_queue_free(self.m(Mv::M12).flow_rate, c))
            .unwrap_or(1.0);
        let p0_9 = self
            .m(Mv::M9)
            .movement_capacity
            .map(|c| prob_queue_free(self.m(Mv::M9).flow_rate, c))
            .unwrap_or(1.0);
        assign(self, Mv::M1U, p0_12); // Equation 20-24 + 20-26
        assign(self, Mv::M4U, p0_9); // Equation 20-25 + 20-26

        // ── Step 7d: queue-free probabilities of the major-street left-turn
        //    movements. Equation 20-28 gives the exclusive-lane p_0,j (using a
        //    combined L+U shared-lane capacity per Equation 20-27); if the left
        //    turn instead shares the through lane or uses a short pocket, the
        //    p*_0,j of Equations 20-29 through 20-34 is substituted here so it
        //    propagates to every Rank 3/4 impedance product below and to the
        //    Step 11b Rank 1 delay (HCM Chapter 32 Example Problem 4).
        let p0_1_excl = self.p0_major_left(Mv::M1, Mv::M1U);
        let p0_4_excl = self.p0_major_left(Mv::M4, Mv::M4U);
        let p0_1 = self
            .p0_star_major_left(true, p0_1_excl)
            .unwrap_or(p0_1_excl);
        let p0_4 = self
            .p0_star_major_left(false, p0_4_excl)
            .unwrap_or(p0_4_excl);

        // ── Step 8: Rank 3 movement capacities.
        if self.geometry.is_three_leg {
            // At a T intersection the minor-street left turn (movement 7) is
            // the Rank 3 movement; it is impeded by the major-street left
            // turn 4+4U (Equation 20-35) and pedestrians (Exhibit 20-24
            // mapping: p_p,15 × p_p,13).
            let f7 = p0_4 * pp15 * pp13;
            assign(self, Mv::M7, f7);
        } else {
            // Movements 8 and 11 impeded by both major-street left turns
            // (Equation 20-35) and pedestrians 15 and 16 (Equation 20-73,
            // Exhibit 20-23).
            let f_r3 = p0_1 * p0_4 * pp15 * pp16;
            for mv in [Mv::M8, Mv::M11] {
                let cp = match self.m(mv).potential_capacity {
                    Some(cp) => cp,
                    None => continue,
                };
                let cm_one = movement_capacity(cp, f_r3); // Equation 20-36
                if self.is_two_stage(mv) {
                    // Step 8b (Equations 20-37 through 20-40). Stage I is
                    // impeded by the near-side major left (1+1U for movement
                    // 8, 4+4U for movement 11); Stage II by the far side.
                    let (p_i, p_ii, v_l, n_m) = if mv == Mv::M8 {
                        (
                            p0_1,
                            p0_4,
                            self.m(Mv::M1).flow_rate + self.m(Mv::M1U).flow_rate,
                            self.geometry.median_storage_nb.unwrap_or(0),
                        )
                    } else {
                        (
                            p0_4,
                            p0_1,
                            self.m(Mv::M4).flow_rate + self.m(Mv::M4U).flow_rate,
                            self.geometry.median_storage_sb.unwrap_or(0),
                        )
                    };
                    let cp1 = self.m(mv).potential_capacity_stage1.unwrap_or(cp);
                    let cp2 = self.m(mv).potential_capacity_stage2.unwrap_or(cp);
                    let cm1 = movement_capacity(cp1, p_i);
                    let cm2 = movement_capacity(cp2, p_ii);
                    let c_t = Self::two_stage_total_capacity(cm1, cm2, cm_one, v_l, n_m);
                    let r = self.m_mut(mv);
                    r.movement_capacity_stage1 = Some(cm1);
                    r.movement_capacity_stage2 = Some(cm2);
                    r.movement_capacity = Some(c_t);
                } else {
                    self.m_mut(mv).movement_capacity = Some(cm_one);
                }
            }
        }

        // ── Step 9: Rank 4 movement capacities (four-leg only).
        if !self.geometry.is_three_leg {
            // Queue-free probabilities of the crossing movements use the
            // final (possibly two-stage) capacities from Step 8.
            let p0_8 = self
                .m(Mv::M8)
                .movement_capacity
                .map(|c| prob_queue_free(self.m(Mv::M8).flow_rate, c))
                .unwrap_or(1.0);
            let p0_11 = self
                .m(Mv::M11)
                .movement_capacity
                .map(|c| prob_queue_free(self.m(Mv::M11).flow_rate, c))
                .unwrap_or(1.0);

            // Equations 20-41/20-74 and 20-42/20-75 (with pedestrian terms):
            // f_p,7  = [1 / (1/(p_0,1+1U p_0,4+4U) + 1/p_0,11 - 1)] p_0,12 p_p,x
            // f_p,10 = [1 / (1/(p_0,1+1U p_0,4+4U) + 1/p_0,8  - 1)] p_0,9  p_p,x
            let dependent = |p_majors: f64, p_cross: f64| -> f64 {
                if p_majors <= 0.0 || p_cross <= 0.0 {
                    return 0.0;
                }
                1.0 / (1.0 / p_majors + 1.0 / p_cross - 1.0)
            };
            let fp7 = dependent(p0_1 * p0_4, p0_11) * p0_12 * pp15 * pp13;
            let fp10 = dependent(p0_1 * p0_4, p0_8) * p0_9 * pp16 * pp14;

            for (mv, fp) in [(Mv::M7, fp7), (Mv::M10, fp10)] {
                let cp = match self.m(mv).potential_capacity {
                    Some(cp) => cp,
                    None => continue,
                };
                let cm_one = movement_capacity(cp, fp); // Eq. 20-43 / 20-44
                if self.is_two_stage(mv) {
                    // Step 9b: Stage I impeded by the near-side major left;
                    // Stage II by the far-side major left, the conflicting
                    // minor right turn, and Stage I of the opposing crossing
                    // movement (p_0,I per the Chapter 32 Example Problem 3
                    // procedure).
                    let (p_i, v_l, n_m) = if mv == Mv::M7 {
                        (
                            p0_1,
                            self.m(Mv::M1).flow_rate + self.m(Mv::M1U).flow_rate,
                            self.geometry.median_storage_nb.unwrap_or(0),
                        )
                    } else {
                        (
                            p0_4,
                            self.m(Mv::M4).flow_rate + self.m(Mv::M4U).flow_rate,
                            self.geometry.median_storage_sb.unwrap_or(0),
                        )
                    };
                    // p_0 of Stage I of the opposing crossing movement
                    let p0_i_cross = |t: &Twsc, cross: Mv| -> f64 {
                        t.m(cross)
                            .movement_capacity_stage1
                            .map(|c| prob_queue_free(t.m(cross).flow_rate, c))
                            .unwrap_or(1.0)
                    };
                    let p_ii = if mv == Mv::M7 {
                        p0_4 * p0_12 * p0_i_cross(self, Mv::M11)
                    } else {
                        p0_1 * p0_9 * p0_i_cross(self, Mv::M8)
                    };
                    let cp1 = self.m(mv).potential_capacity_stage1.unwrap_or(cp);
                    let cp2 = self.m(mv).potential_capacity_stage2.unwrap_or(cp);
                    let cm1 = movement_capacity(cp1, p_i);
                    let cm2 = movement_capacity(cp2, p_ii);
                    let c_t = Self::two_stage_total_capacity(cm1, cm2, cm_one, v_l, n_m);
                    let r = self.m_mut(mv);
                    r.movement_capacity_stage1 = Some(cm1);
                    r.movement_capacity_stage2 = Some(cm2);
                    r.movement_capacity = Some(c_t);
                } else {
                    self.m_mut(mv).movement_capacity = Some(cm_one);
                }
            }
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    // Step 10: final capacity adjustments (shared and flared minor lanes)
    // ═══════════════════════════════════════════════════════════════════════

    /// HCM Equation 20-50: capacity of a flared minor-street lane
    ///
    /// `c_F = (v_R + v_L+TH) / [ (v_R/c_R)^(n_R+1) + (v_L+TH/c_L+TH)^(n_R+1) ]^(1/(n_R+1))`
    ///
    /// For `n_R = 0` this reduces to the shared-lane capacity
    /// (Equation 20-49).
    pub fn flared_lane_capacity(
        v_right: f64,
        c_right: f64,
        v_left_through: f64,
        c_left_through: f64,
        n_r: u32,
    ) -> f64 {
        let n = (n_r + 1) as f64;
        let sum = (v_right / c_right).powf(n) + (v_left_through / c_left_through).powf(n);
        if sum <= 0.0 {
            return c_right.max(c_left_through);
        }
        (v_right + v_left_through) / sum.powf(1.0 / n)
    }

    fn minor_lanes(&self, approach_nb: bool) -> Vec<TwscLaneResult> {
        let (l, t, r, cfg, flare) = if approach_nb {
            (
                Mv::M7,
                Mv::M8,
                Mv::M9,
                self.geometry.minor_lanes_nb,
                self.geometry.flare_storage_nb.unwrap_or(0),
            )
        } else {
            (
                Mv::M10,
                Mv::M11,
                Mv::M12,
                self.geometry.minor_lanes_sb,
                self.geometry.flare_storage_sb.unwrap_or(0),
            )
        };
        let fv = |mv: Mv| self.m(mv).flow_rate;
        let fc = |mv: Mv| self.m(mv).movement_capacity.unwrap_or(0.0);
        let present: Vec<Mv> = [l, t, r]
            .into_iter()
            .filter(|&mv| fv(mv) > 0.0 || self.m(mv).movement_capacity.is_some())
            .collect();
        if present.is_empty() {
            return Vec::new();
        }

        let lane = |movs: Vec<Mv>, capacity: f64| -> TwscLaneResult {
            let flow: f64 = movs.iter().map(|&mv| fv(mv)).sum();
            TwscLaneResult {
                movements: movs,
                flow_rate: flow,
                capacity,
                control_delay: 0.0,
                los: 'A',
                queue_95: 0.0,
            }
        };
        let shared = |movs: &[Mv]| -> f64 {
            let pairs: Vec<(f64, f64)> = movs
                .iter()
                .filter(|&&mv| fv(mv) > 0.0 || self.m(mv).movement_capacity.is_some())
                .map(|&mv| (fv(mv), fc(mv)))
                .collect();
            Self::shared_lane_capacity(&pairs)
        };

        match cfg {
            MinorLaneConfig::SingleShared => {
                if flare > 0 {
                    // Step 10b (Equation 20-50): right-turn storage in the
                    // flare, left+through as the shared remainder.
                    let lt: Vec<Mv> = present.iter().copied().filter(|&mv| mv != r).collect();
                    let c_lt = shared(&lt);
                    let v_lt: f64 = lt.iter().map(|&mv| fv(mv)).sum();
                    let c_f = Self::flared_lane_capacity(fv(r), fc(r), v_lt, c_lt, flare);
                    vec![lane(present, c_f)]
                } else {
                    // Step 10a (Equation 20-49)
                    let c_sh = shared(&present);
                    vec![lane(present, c_sh)]
                }
            }
            MinorLaneConfig::SharedLeftThroughExclusiveRight => {
                let lt: Vec<Mv> = present.iter().copied().filter(|&mv| mv != r).collect();
                let mut lanes = vec![lane(lt.clone(), shared(&lt))];
                if present.contains(&r) {
                    lanes.push(lane(vec![r], fc(r)));
                }
                lanes
            }
            MinorLaneConfig::ExclusiveLeftSharedThroughRight => {
                let tr: Vec<Mv> = present.iter().copied().filter(|&mv| mv != l).collect();
                let mut lanes = Vec::new();
                if present.contains(&l) {
                    lanes.push(lane(vec![l], fc(l)));
                }
                lanes.push(lane(tr.clone(), shared(&tr)));
                lanes
            }
            MinorLaneConfig::Separate => present
                .into_iter()
                .map(|mv| lane(vec![mv], fc(mv)))
                .collect(),
        }
    }

    /// HCM Chapter 20, Step 10: shared-lane (Equation 20-49) and flared-lane
    /// (Equation 20-50) capacities of the minor-street approaches.
    pub fn step10_lane_capacities(&mut self) {
        self.lanes_nb = self.minor_lanes(true);
        self.lanes_sb = if self.geometry.is_three_leg {
            Vec::new()
        } else {
            self.minor_lanes(false)
        };
    }

    /// HCM Equations 20-51 through 20-60: capacity of a shared major-street
    /// lane where the left-turn pocket is too short (or absent).
    ///
    /// * `v_left` — major-street left-turn + U-turn flow rate v_1+1U, veh/h
    /// * `c_m_left` — movement capacity of the left+U-turn movement, veh/h
    /// * `v_through`, `v_right` — major through / right flow rates, veh/h
    ///   (`v_right` = 0 with an exclusive right-turn lane)
    /// * `s_through`, `s_right` — saturation flow rates (defaults 1,800 and
    ///   1,500 veh/h)
    /// * `f_ll` — portion of through/right traffic in the left lane (1.0 for
    ///   one lane; defaults 0.5 / 0.33 for two / three lanes)
    /// * `n_major` — through lanes per direction N
    /// * `n_l` — vehicles storable in the left-turn pocket
    ///
    /// Returns c_SS, the shared short-lane capacity, veh/h.
    #[allow(clippy::too_many_arguments)]
    pub fn shared_major_lane_capacity(
        v_left: f64,
        c_m_left: f64,
        v_through: f64,
        v_right: f64,
        s_through: f64,
        s_right: f64,
        f_ll: f64,
        n_major: u32,
        n_l: u32,
    ) -> f64 {
        // Equation 20-53 / 20-58
        let x_l = if c_m_left > 0.0 { v_left / c_m_left } else { 0.0 };
        // Equation 20-54 / 20-59
        let x_23 = f_ll * (v_through / s_through + v_right / s_right);
        // Equation 20-52 / 20-57 (nL+1 root form, cf. Equation 20-29)
        let n = (n_l + 1) as f64;
        let bracket = (1.0 + x_23.powf(n) / (1.0 - x_23)).powf(1.0 / n);
        let x_shared = x_l * bracket;
        // Equation 20-55 / 20-60
        let s_23 = (v_through + v_right)
            / (v_through / (n_major as f64 * s_through) + v_right / s_right);
        // Equation 20-51 / 20-56
        if x_shared <= 0.0 {
            return s_23;
        }
        ((v_left + v_through + v_right) / x_shared).min(s_23)
    }

    /// HCM Equations 20-29 through 20-34: probability of a queue-free state
    /// p*_0 for a major-street approach with a shared or short left-turn
    /// lane.
    ///
    /// * `p0` — queue-free probability assuming an exclusive left-turn lane
    ///   (Equation 20-28)
    /// * `x_23` — combined degree of saturation of the major through and
    ///   right movements (Equation 20-30)
    /// * `n_l` — vehicles storable in the left-turn pocket (0 = shared lane)
    pub fn prob_queue_free_shared_major(p0: f64, x_23: f64, n_l: u32) -> f64 {
        let n = (n_l + 1) as f64;
        // Equations 20-29 / 20-31 (root form; for n_l = 0 this reduces to
        // Equations 20-33 / 20-34: p* = 1 - (1 - p0)/(1 - x_23))
        let bracket = (1.0 + x_23.powf(n) / (1.0 - x_23)).powf(1.0 / n);
        (1.0 - (1.0 - p0) * bracket).clamp(0.0, 1.0)
    }

    // ═══════════════════════════════════════════════════════════════════════
    // Steps 11–13: delay, LOS, queues
    // ═══════════════════════════════════════════════════════════════════════

    /// HCM Equation 20-66: 95th percentile queue, veh.
    pub fn queue_95(v: f64, c: f64, t_h: f64) -> f64 {
        if c <= 0.0 {
            return 0.0;
        }
        let x = v / c;
        900.0 * t_h
            * (x - 1.0 + ((x - 1.0).powi(2) + (3_600.0 / c) * x / (150.0 * t_h)).sqrt())
            * (c / 3_600.0)
    }

    /// HCM Chapter 20, Step 11a: control delay to Rank 2 through Rank 4
    /// movements (Equation 20-61) and per-lane delay for the minor-street
    /// approach lanes, plus LOS (Exhibit 20-2) and Step 13 queues.
    pub fn step11_movement_delay(&mut self) {
        let t = self.analysis_period_h;
        // Major-street left turns and U-turns (exclusive lanes assumed; a
        // shared L+U lane uses the combined flow and Equation 20-27).
        for mv in [Mv::M1, Mv::M1U, Mv::M4, Mv::M4U] {
            let (v, c) = (self.m(mv).flow_rate, self.m(mv).movement_capacity);
            if let Some(c) = c {
                if c > 0.0 {
                    let d = control_delay_unsignalized(v, c, t);
                    let r = self.m_mut(mv);
                    r.control_delay = Some(d);
                    r.los = Some(los_char(los_unsignalized(d, v > c)));
                    r.queue_95 = Some(Self::queue_95(v, c, t));
                }
            }
        }
        // Minor-street lanes (movement delay is reported at the lane level
        // for shared lanes).
        let mut all_lanes = std::mem::take(&mut self.lanes_nb);
        let n_nb = all_lanes.len();
        all_lanes.extend(std::mem::take(&mut self.lanes_sb));
        for lane in all_lanes.iter_mut() {
            if lane.capacity > 0.0 {
                lane.control_delay = control_delay_unsignalized(lane.flow_rate, lane.capacity, t);
                lane.los = los_char(los_unsignalized(
                    lane.control_delay,
                    lane.flow_rate > lane.capacity,
                ));
                lane.queue_95 = Self::queue_95(lane.flow_rate, lane.capacity, t);
            }
        }
        self.lanes_sb = all_lanes.split_off(n_nb);
        self.lanes_nb = all_lanes;

        // ── Step 11b: Rank 1 delay from a shared or short major-street
        //    left-turn lane (Equations 20-62/20-63); `None` (0 s/veh) when both
        //    major lefts have exclusive lanes.
        self.rank1_major_delay = self.compute_rank1_major_delay();
    }

    /// HCM Step 11b: Rank 1 delay `[d_2+3 (EB), d_5+6 (WB)]` to major-street
    /// through-and-right vehicles sharing a lane with a blocked left turn
    /// (Equations 20-62/20-63), or `None` when both major-street left turns
    /// have exclusive lanes. Requires the Step 11a major-left delays to have
    /// been computed. `d_1+1U` / `d_4+4U` are taken as the left-turn movement
    /// delay (exact when no U-turn shares the lane, as in Example Problem 4).
    fn compute_rank1_major_delay(&self) -> Option<[f64; 2]> {
        let f_ll = self.f_ll_major();
        let n = self.n_major();
        let per_approach = |eb: bool, left: Mv, uturn: Mv, through: Mv, right: Mv| -> Option<f64> {
            let p0_excl = self.p0_major_left(left, uturn);
            let p0_star = self.p0_star_major_left(eb, p0_excl)?;
            let v_left = self.m(left).flow_rate + self.m(uturn).flow_rate;
            let v_through = self.m(through).flow_rate;
            let v_right = self.m(right).flow_rate;
            let d_left = self.m(left).control_delay.unwrap_or(0.0);
            Some(Self::rank1_delay(
                p0_star, v_left, v_through, v_right, d_left, f_ll, n,
            ))
        };
        let d23 = per_approach(true, Mv::M1, Mv::M1U, Mv::M2, Mv::M3);
        let d56 = per_approach(false, Mv::M4, Mv::M4U, Mv::M5, Mv::M6);
        match (d23, d56) {
            (None, None) => None,
            _ => Some([d23.unwrap_or(0.0), d56.unwrap_or(0.0)]),
        }
    }

    /// HCM Chapter 20, Step 11b (Equations 20-62 and 20-63): delay to
    /// Rank 1 major-street vehicles blocked by a shared or short left-turn
    /// lane.
    ///
    /// * `p0_star` — proportion of Rank 1 vehicles not blocked
    ///   (Equations 20-29/20-31)
    /// * `v_left` — major left + U-turn flow in the shared lane, veh/h
    /// * `v_through`, `v_right` — major through / right flows, veh/h
    /// * `d_left` — delay to the major left-turning vehicles, s/veh
    ///   (Equation 20-61)
    /// * `f_ll` — portion of through/right traffic in the left lane
    /// * `n_major` — through lanes per direction
    pub fn rank1_delay(
        p0_star: f64,
        v_left: f64,
        v_through: f64,
        v_right: f64,
        d_left: f64,
        f_ll: f64,
        n_major: u32,
    ) -> f64 {
        if n_major > 1 {
            let v_tr_left_lane = f_ll * (v_through + v_right);
            if v_left + v_tr_left_lane <= 0.0 {
                return 0.0;
            }
            (1.0 - p0_star) * v_tr_left_lane / (v_left + v_tr_left_lane) * d_left
        } else {
            (1.0 - p0_star) * d_left
        }
    }

    /// HCM Chapter 20, Step 12 (Equations 20-64 and 20-65): approach and
    /// intersection control delay. Rank 1 major-street through/right movements
    /// carry 0 s/veh with an exclusive major-left lane; with a shared or short
    /// major-left lane they carry the Step 11b Rank 1 delay d_2+3 / d_5+6
    /// (Equations 20-62/20-63), as in HCM Chapter 32 Example Problem 4.
    ///
    /// Returns `[d_EB, d_WB, d_NB, d_SB]` and stores the intersection delay.
    pub fn step12_approach_intersection_delay(&mut self) -> [f64; 4] {
        let f = |mv: Mv| self.m(mv).flow_rate;
        let d = |mv: Mv| self.m(mv).control_delay.unwrap_or(0.0);
        // Step 11b Rank 1 delay to the shared-lane major through/right
        // movements; 0 s/veh when the major left has an exclusive lane.
        let [d23, d56] = self.rank1_major_delay.unwrap_or([0.0, 0.0]);
        // Equation 20-64 per approach.
        let eb: Vec<(f64, f64)> = vec![
            (d(Mv::M1), f(Mv::M1)),
            (d(Mv::M1U), f(Mv::M1U)),
            (d23, f(Mv::M2)),
            (d23, f(Mv::M3)),
        ];
        let wb: Vec<(f64, f64)> = vec![
            (d(Mv::M4), f(Mv::M4)),
            (d(Mv::M4U), f(Mv::M4U)),
            (d56, f(Mv::M5)),
            (d56, f(Mv::M6)),
        ];
        let nb: Vec<(f64, f64)> = self
            .lanes_nb
            .iter()
            .map(|l| (l.control_delay, l.flow_rate))
            .collect();
        let sb: Vec<(f64, f64)> = self
            .lanes_sb
            .iter()
            .map(|l| (l.control_delay, l.flow_rate))
            .collect();
        let d_eb = aggregate_control_delay(&eb);
        let d_wb = aggregate_control_delay(&wb);
        let d_nb = aggregate_control_delay(&nb);
        let d_sb = aggregate_control_delay(&sb);
        // Equation 20-65
        let va: [f64; 4] = [
            eb.iter().map(|(_, v)| v).sum(),
            wb.iter().map(|(_, v)| v).sum(),
            nb.iter().map(|(_, v)| v).sum(),
            sb.iter().map(|(_, v)| v).sum(),
        ];
        let pairs: Vec<(f64, f64)> = [d_eb, d_wb, d_nb, d_sb]
            .into_iter()
            .zip(va)
            .collect();
        self.intersection_delay = Some(aggregate_control_delay(&pairs));
        self.approach_delays = Some([d_eb, d_wb, d_nb, d_sb]);
        [d_eb, d_wb, d_nb, d_sb]
    }

    /// Run the complete Chapter 20 motorized vehicle procedure
    /// (Steps 1–13).
    pub fn analyze(&mut self) {
        if self.movements.len() != 14 {
            self.movements = (0..14).map(|_| TwscMovementResult::default()).collect();
        }
        self.step1_2_demand_flow_rates();
        self.step3_conflicting_flows();
        self.step4_headways();
        self.step5_potential_capacities();
        self.step6_9_movement_capacities();
        self.step10_lane_capacities();
        self.step11_movement_delay();
        self.step12_approach_intersection_delay();
    }
}

/// Convert a [`crate::hcm::common::LevelOfService`] to its letter.
pub(crate) fn los_char(los: crate::hcm::common::LevelOfService) -> char {
    los.into()
}
