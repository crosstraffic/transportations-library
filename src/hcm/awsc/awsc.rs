//! # All-Way STOP-Controlled Intersections (HCM Chapter 21)
//!
//! Implements the HCM 7th Edition Chapter 21 motorized vehicle core
//! methodology (Section 3) including the three-lane-approach extension
//! (Section 4). The procedure iterates departure headways and degrees of
//! utilization over the degree-of-conflict probability states until
//! convergence (HCM Exhibit 21-10, Steps 5–11).
//!
//! ## Computational steps
//!
//! 1. Convert movement demand volumes to flow rates (Equation 21-12)
//! 2. Determine lane flow rates (movement volumes are assigned to lanes by
//!    the caller)
//! 3. Determine geometry group for each approach (Exhibit 21-11)
//! 4. Determine saturation headway adjustments (Equation 21-13,
//!    Exhibit 21-12)
//! 5.–11. Iterate departure headways: initial h_d = 3.2 s, degree of
//!    utilization x = v h_d / 3,600 (Equation 21-14), probability states
//!    (Equations 21-15/21-34, Exhibits 21-13/21-14/21-16), probability
//!    adjustment factors (Equations 21-16 through 21-26 for two-lane
//!    frameworks; 21-34 through 21-45 for three-lane frameworks),
//!    saturation headways (Equation 21-27, Exhibit 21-15), departure
//!    headway h_d = Σ P'(i) h_si (Equation 21-28), convergence at 0.1 s
//! 12. Compute capacity (increase subject-lane flow until x = 1)
//! 13. Compute service time t_s = h_d − m (Equation 21-29)
//! 14. Compute control delay and LOS per lane (Equation 21-30,
//!    Exhibit 21-8)
//! 15. Aggregate delay/LOS per approach and intersection (Equations 21-31
//!    and 21-32)
//! 16. Compute 95th percentile queue (Equation 21-33)

use serde::{Deserialize, Serialize};

use crate::hcm::common::delay::{aggregate_control_delay, control_delay_awsc};
use crate::hcm::common::los_tables::los_unsignalized;

/// Initial departure headway assumed for the first iteration (HCM
/// Chapter 21, Step 5).
pub const INITIAL_DEPARTURE_HEADWAY_S: f64 = 3.2;

/// Convergence tolerance on departure headway (HCM Chapter 21, Step 11:
/// "If the values change by more than 0.1 s ... repeat").
pub const CONVERGENCE_TOLERANCE_S: f64 = 0.1;

/// Serial-correlation factor α of Equations 21-21 through 21-25 and 21-40
/// through 21-44 (0.01, or 0.00 to ignore correlation).
pub const ALPHA: f64 = 0.01;

// ═══════════════════════════════════════════════════════════════════════════════
// Geometry groups (Exhibit 21-11)
// ═══════════════════════════════════════════════════════════════════════════════

/// AWSC geometry group per HCM Exhibit 21-11.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GeometryGroup {
    G1,
    G2,
    G3a,
    G3b,
    G4a,
    G4b,
    G5,
    G6,
}

impl GeometryGroup {
    /// Move-up time m, s (HCM Equation 21-29 definitions: 2.0 s for
    /// Geometry Groups 1–4, 2.3 s for Groups 5 and 6).
    pub fn move_up_time(self) -> f64 {
        match self {
            GeometryGroup::G5 | GeometryGroup::G6 => 2.3,
            _ => 2.0,
        }
    }

    /// Column index into the Exhibit 21-12 / 21-15 tables.
    fn col(self) -> usize {
        match self {
            GeometryGroup::G1 => 0,
            GeometryGroup::G2 => 1,
            GeometryGroup::G3a => 2,
            GeometryGroup::G3b => 3,
            GeometryGroup::G4a => 4,
            GeometryGroup::G4b => 5,
            GeometryGroup::G5 => 6,
            GeometryGroup::G6 => 7,
        }
    }
}

/// HCM Exhibit 21-11: determine the geometry group for an approach.
///
/// * `four_leg` — true for a four-leg intersection, false for a T
/// * `subject_lanes` — lanes on the subject approach (1–3)
/// * `opposing_lanes` — lanes on the opposing approach (0–3)
/// * `conflicting_lanes` — lanes on the conflicting approaches (1–3); if
///   the two conflicting approaches differ, the higher value (Exhibit 21-11
///   note a)
pub fn geometry_group(
    four_leg: bool,
    subject_lanes: u32,
    opposing_lanes: u32,
    conflicting_lanes: u32,
) -> GeometryGroup {
    let (s, o, c) = (subject_lanes, opposing_lanes, conflicting_lanes);
    match s {
        1 => match (o, c) {
            (0..=1, 1) => GeometryGroup::G1,
            (0..=1, 2) => GeometryGroup::G2,
            (0..=1, _) => GeometryGroup::G5, // (1, 0 or 1, 3)
            (2, 1) => {
                if four_leg {
                    GeometryGroup::G4a
                } else {
                    GeometryGroup::G3a
                }
            }
            (2, 2) => {
                if four_leg {
                    GeometryGroup::G4b
                } else {
                    GeometryGroup::G3b
                }
            }
            (2, _) => GeometryGroup::G6, // (1, 2, 3)
            (_, 1) => GeometryGroup::G5, // (1, 3, 1)
            _ => GeometryGroup::G6,      // (1, 3, 2 or 3)
        },
        2 => match (o, c) {
            (0..=2, 1..=2) => GeometryGroup::G5,
            _ => GeometryGroup::G6, // (2, 3, any) or (2, any, 3)
        },
        _ => match (o, c) {
            (0..=1, _) => GeometryGroup::G5, // (3, 0 or 1, 1..3)
            (_, 1) => GeometryGroup::G5,     // (3, 2 or 3, 1)
            _ => GeometryGroup::G6,          // (3, 2 or 3, 2 or 3)
        },
    }
}

/// HCM Exhibit 21-12: saturation headway adjustments by geometry group, s.
/// Rows: left turns, right turns, heavy vehicles; columns: Groups 1, 2, 3a,
/// 3b, 4a, 4b, 5, 6.
const H_LT_ADJ: [f64; 8] = [0.2, 0.2, 0.2, 0.2, 0.2, 0.2, 0.5, 0.5];
const H_RT_ADJ: [f64; 8] = [-0.6, -0.6, -0.6, -0.6, -0.6, -0.6, -0.7, -0.7];
const H_HV_ADJ: [f64; 8] = [1.7, 1.7, 1.7, 1.7, 1.7, 1.7, 1.7, 1.7];

/// HCM Equation 21-13: headway adjustment
///
/// `h_adj = h_LT,adj P_LT + h_RT,adj P_RT + h_HV,adj P_HV`
///
/// * `group` — geometry group (Exhibit 21-12 column)
/// * `p_lt`, `p_rt` — proportions of left- and right-turning vehicles in
///   the lane (decimal)
/// * `p_hv` — proportion of heavy vehicles in the lane (decimal)
pub fn headway_adjustment(group: GeometryGroup, p_lt: f64, p_rt: f64, p_hv: f64) -> f64 {
    let c = group.col();
    H_LT_ADJ[c] * p_lt + H_RT_ADJ[c] * p_rt + H_HV_ADJ[c] * p_hv
}

/// HCM Exhibit 21-15: base saturation headway h_base, s, by
/// degree-of-conflict case (1–5), number of opposing+conflicting vehicles,
/// and geometry group.
///
/// Groups 1–4b use a single value per case; Groups 5 and 6 vary with the
/// vehicle count.
pub fn base_saturation_headway(case: u8, vehicles: u32, group: GeometryGroup) -> f64 {
    let col = group.col();
    // Groups 1, 2, 3a, 3b, 4a, 4b (single value per case)
    const BY_CASE: [[f64; 6]; 5] = [
        [3.9, 3.9, 4.0, 4.3, 4.0, 4.5], // Case 1
        [4.7, 4.7, 4.8, 5.1, 4.8, 5.3], // Case 2
        [5.8, 5.8, 5.9, 6.2, 5.9, 6.4], // Case 3
        [7.0, 7.0, 7.1, 7.4, 7.1, 7.6], // Case 4
        [9.6, 9.6, 9.7, 10.0, 9.7, 10.2], // Case 5
    ];
    if col < 6 {
        return BY_CASE[(case - 1) as usize][col];
    }
    let group5 = col == 6;
    match (case, vehicles) {
        (1, _) => 4.5,
        (2, 1) => {
            if group5 {
                5.0
            } else {
                6.0
            }
        }
        (2, 2) => {
            if group5 {
                6.2
            } else {
                6.8
            }
        }
        // Group 5 has at most two vehicles in Case 2 (two-lane opposing);
        // the >= 3 column exists only for Group 6.
        (2, _) => {
            if group5 {
                6.2
            } else {
                7.4
            }
        }
        (3, 1) => {
            if group5 {
                6.4
            } else {
                6.6
            }
        }
        (3, 2) => {
            if group5 {
                7.2
            } else {
                7.3
            }
        }
        (3, _) => {
            if group5 {
                7.2
            } else {
                7.8
            }
        }
        (4, 0..=2) => {
            if group5 {
                7.6
            } else {
                8.1
            }
        }
        (4, 3) => {
            if group5 {
                7.8
            } else {
                8.7
            }
        }
        (4, 4) => {
            if group5 {
                9.0
            } else {
                9.6
            }
        }
        (4, _) => {
            if group5 {
                9.0
            } else {
                12.3
            }
        }
        (_, 0..=3) => {
            if group5 {
                9.7
            } else {
                10.0
            }
        }
        (_, 4) => {
            if group5 {
                9.7
            } else {
                11.1
            }
        }
        (_, 5) => {
            if group5 {
                10.0
            } else {
                11.4
            }
        }
        (_, _) => {
            if group5 {
                11.5
            } else {
                13.3
            }
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Input model
// ═══════════════════════════════════════════════════════════════════════════════

/// Approach direction (the direction of travel of the approach).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApproachDir {
    EB,
    WB,
    NB,
    SB,
}

impl ApproachDir {
    /// Opposing approach.
    pub fn opposing(self) -> ApproachDir {
        match self {
            ApproachDir::EB => ApproachDir::WB,
            ApproachDir::WB => ApproachDir::EB,
            ApproachDir::NB => ApproachDir::SB,
            ApproachDir::SB => ApproachDir::NB,
        }
    }

    /// Approach conflicting from the subject driver's left.
    pub fn conflicting_left(self) -> ApproachDir {
        match self {
            ApproachDir::EB => ApproachDir::SB,
            ApproachDir::WB => ApproachDir::NB,
            ApproachDir::NB => ApproachDir::EB,
            ApproachDir::SB => ApproachDir::WB,
        }
    }

    /// Approach conflicting from the subject driver's right.
    pub fn conflicting_right(self) -> ApproachDir {
        match self {
            ApproachDir::EB => ApproachDir::NB,
            ApproachDir::WB => ApproachDir::SB,
            ApproachDir::NB => ApproachDir::WB,
            ApproachDir::SB => ApproachDir::EB,
        }
    }

    fn idx(self) -> usize {
        match self {
            ApproachDir::EB => 0,
            ApproachDir::WB => 1,
            ApproachDir::NB => 2,
            ApproachDir::SB => 3,
        }
    }
}

const DIRS: [ApproachDir; 4] = [
    ApproachDir::EB,
    ApproachDir::WB,
    ApproachDir::NB,
    ApproachDir::SB,
];

/// One approach lane with its assigned movement demand (HCM Chapter 21,
/// Step 2: the caller assigns turning-movement volumes to lanes; assume an
/// equal distribution among lanes if unknown).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AwscLane {
    /// Left-turn demand assigned to this lane, veh/h.
    #[serde(default)]
    pub volume_left: f64,
    /// Through demand assigned to this lane, veh/h.
    #[serde(default)]
    pub volume_through: f64,
    /// Right-turn demand assigned to this lane, veh/h.
    #[serde(default)]
    pub volume_right: f64,

    // ── Computed ────────────────────────────────────────────────────────────
    /// Lane flow rate, veh/h (Equation 21-12).
    #[serde(default)]
    pub flow_rate: Option<f64>,
    /// Headway adjustment h_adj, s (Equation 21-13).
    #[serde(default)]
    pub headway_adjustment: Option<f64>,
    /// Converged departure headway h_d, s (Equation 21-28).
    #[serde(default)]
    pub departure_headway: Option<f64>,
    /// Degree of utilization x = v h_d / 3,600 (Equation 21-14).
    #[serde(default)]
    pub degree_of_utilization: Option<f64>,
    /// Service time t_s = h_d − m, s (Equation 21-29).
    #[serde(default)]
    pub service_time: Option<f64>,
    /// Lane capacity, veh/h (Step 12).
    #[serde(default)]
    pub capacity: Option<f64>,
    /// Control delay, s/veh (Equation 21-30).
    #[serde(default)]
    pub control_delay: Option<f64>,
    /// Level of service (Exhibit 21-8; v/c > 1.0 assigned F).
    #[serde(default)]
    pub los: Option<char>,
    /// 95th percentile queue, veh (Equation 21-33).
    #[serde(default)]
    pub queue_95: Option<f64>,
}

impl AwscLane {
    /// Create a lane from assigned movement volumes (L, T, R), veh/h.
    pub fn new(volume_left: f64, volume_through: f64, volume_right: f64) -> Self {
        AwscLane {
            volume_left,
            volume_through,
            volume_right,
            ..Default::default()
        }
    }

    /// Total assigned demand volume, veh/h.
    pub fn total_volume(&self) -> f64 {
        self.volume_left + self.volume_through + self.volume_right
    }
}

/// One AWSC approach.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AwscApproach {
    /// Lanes on the approach in left-to-right order (0–3 lanes; an empty
    /// vector marks a leg with no approach, e.g., the fourth leg of a T).
    pub lanes: Vec<AwscLane>,
    /// Percentage of heavy vehicles on the approach (%).
    #[serde(default)]
    pub heavy_vehicle_pct: f64,

    // ── Computed ────────────────────────────────────────────────────────────
    /// Geometry group (Exhibit 21-11).
    #[serde(default)]
    pub geometry_group: Option<GeometryGroup>,
    /// Approach control delay, s/veh (Equation 21-31).
    #[serde(default)]
    pub control_delay: Option<f64>,
    /// Approach LOS (Exhibit 21-8, delay only).
    #[serde(default)]
    pub los: Option<char>,
}

/// HCM Chapter 21 AWSC motorized vehicle analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Awsc {
    /// Approaches indexed EB, WB, NB, SB.
    pub eb: AwscApproach,
    /// Westbound approach.
    pub wb: AwscApproach,
    /// Northbound approach.
    pub nb: AwscApproach,
    /// Southbound approach.
    pub sb: AwscApproach,
    /// Intersection peak hour factor (Equation 21-12). `None` if lane
    /// volumes are already flow rates.
    #[serde(default)]
    pub phf: Option<f64>,
    /// Analysis period T, h (0.25 for 15 min).
    #[serde(default = "default_t")]
    pub analysis_period_h: f64,

    // ── Computed ────────────────────────────────────────────────────────────
    /// Intersection control delay, s/veh (Equation 21-32).
    #[serde(default)]
    pub intersection_delay: Option<f64>,
    /// Intersection LOS (Exhibit 21-8).
    #[serde(default)]
    pub intersection_los: Option<char>,
    /// Iterations needed for departure-headway convergence (Step 11).
    #[serde(default)]
    pub iterations: Option<u32>,
}

fn default_t() -> f64 {
    0.25
}

/// Internal per-lane iteration state.
#[derive(Clone, Copy)]
struct LaneRef {
    dir: ApproachDir,
    lane: usize,
}

impl Awsc {
    /// Create an analysis from four approaches.
    pub fn new(eb: AwscApproach, wb: AwscApproach, nb: AwscApproach, sb: AwscApproach) -> Self {
        Awsc {
            eb,
            wb,
            nb,
            sb,
            phf: None,
            analysis_period_h: 0.25,
            intersection_delay: None,
            intersection_los: None,
            iterations: None,
        }
    }

    /// Deserialize from JSON (fixture format).
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Serialize inputs + results to JSON.
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    /// Approach accessor by direction.
    pub fn approach(&self, dir: ApproachDir) -> &AwscApproach {
        match dir {
            ApproachDir::EB => &self.eb,
            ApproachDir::WB => &self.wb,
            ApproachDir::NB => &self.nb,
            ApproachDir::SB => &self.sb,
        }
    }

    fn approach_mut(&mut self, dir: ApproachDir) -> &mut AwscApproach {
        match dir {
            ApproachDir::EB => &mut self.eb,
            ApproachDir::WB => &mut self.wb,
            ApproachDir::NB => &mut self.nb,
            ApproachDir::SB => &mut self.sb,
        }
    }

    /// True if the intersection has four legs (all approaches present).
    pub fn is_four_leg(&self) -> bool {
        DIRS.iter().all(|&d| !self.approach(d).lanes.is_empty())
    }

    /// Maximum number of lanes on any approach (framework selector: 64
    /// probability states for one/two-lane approaches, 512 for three-lane
    /// approaches per Chapter 21 Section 4).
    pub fn max_lanes(&self) -> u32 {
        DIRS.iter()
            .map(|&d| self.approach(d).lanes.len() as u32)
            .max()
            .unwrap_or(0)
    }

    // ═══════════════════════════════════════════════════════════════════════
    // Steps 1–2: flow rates
    // ═══════════════════════════════════════════════════════════════════════

    /// HCM Equation 21-12: convert lane movement volumes to flow rates
    /// `v_i = V_i / PHF` and store the per-lane totals.
    pub fn step1_2_flow_rates(&mut self) {
        let phf = match self.phf {
            Some(p) if p > 0.0 => p,
            _ => 1.0,
        };
        for dir in DIRS {
            for lane in self.approach_mut(dir).lanes.iter_mut() {
                lane.flow_rate = Some(lane.total_volume() / phf);
            }
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    // Step 3: geometry groups
    // ═══════════════════════════════════════════════════════════════════════

    /// HCM Exhibit 21-11: assign a geometry group to every approach.
    pub fn step3_geometry_groups(&mut self) {
        let four_leg = self.is_four_leg();
        for dir in DIRS {
            if self.approach(dir).lanes.is_empty() {
                continue;
            }
            let subject = self.approach(dir).lanes.len() as u32;
            let opposing = self.approach(dir.opposing()).lanes.len() as u32;
            let conflicting = self
                .approach(dir.conflicting_left())
                .lanes
                .len()
                .max(self.approach(dir.conflicting_right()).lanes.len())
                as u32;
            let g = geometry_group(four_leg, subject, opposing, conflicting.max(1));
            self.approach_mut(dir).geometry_group = Some(g);
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    // Step 4: saturation headway adjustments
    // ═══════════════════════════════════════════════════════════════════════

    /// HCM Equation 21-13 and Exhibit 21-12: headway adjustment for every
    /// lane based on its turning proportions and heavy-vehicle percentage.
    pub fn step4_headway_adjustments(&mut self) {
        for dir in DIRS {
            let group = match self.approach(dir).geometry_group {
                Some(g) => g,
                None => continue,
            };
            let p_hv = self.approach(dir).heavy_vehicle_pct / 100.0;
            for lane in self.approach_mut(dir).lanes.iter_mut() {
                let total = lane.total_volume();
                let (p_lt, p_rt) = if total > 0.0 {
                    (lane.volume_left / total, lane.volume_right / total)
                } else {
                    (0.0, 0.0)
                };
                lane.headway_adjustment = Some(headway_adjustment(group, p_lt, p_rt, p_hv));
            }
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    // Steps 5–11: departure headway iteration
    // ═══════════════════════════════════════════════════════════════════════

    fn lane_refs(&self) -> Vec<LaneRef> {
        let mut v = Vec::new();
        for dir in DIRS {
            for lane in 0..self.approach(dir).lanes.len() {
                v.push(LaneRef { dir, lane });
            }
        }
        v
    }

    /// Departure headway of one subject lane given the current degrees of
    /// utilization `x` of every lane (HCM Equations 21-15 through 21-28).
    ///
    /// `frame_lanes` is the framework lane count per approach (2 for the
    /// 64-case Exhibit 21-14 table, 3 for the 512-case Exhibit 32-15 table).
    fn departure_headway_for(
        &self,
        subject: LaneRef,
        x: &[Vec<f64>; 4],
        frame_lanes: usize,
        group: GeometryGroup,
        h_adj: f64,
    ) -> f64 {
        let o = subject.dir.opposing();
        let cl = subject.dir.conflicting_left();
        let cr = subject.dir.conflicting_right();

        // Occupied-probability of each framework lane (Exhibits 21-13 and
        // 21-16: P(a_j = 1) = x_j for V_j > 0, else 0).
        let probs = |dir: ApproachDir| -> Vec<f64> {
            let mut p = vec![0.0; frame_lanes];
            let xs = &x[dir.idx()];
            for (i, &xi) in xs.iter().enumerate().take(frame_lanes) {
                p[i] = xi.clamp(0.0, 1.0);
            }
            p
        };
        let p_o = probs(o);
        let p_cl = probs(cl);
        let p_cr = probs(cr);

        // Class sizes for the probability adjustment divisors:
        // two-lane framework (Equations 21-21..21-25): 1, 3, 6, 27, 27
        // three-lane framework (Equations 21-40..21-44): 1, 7, 14, 147, 343
        let m = (1u32 << frame_lanes) - 1; // occupancy patterns per approach
        let n_c2 = m as f64;
        let n_c3 = 2.0 * m as f64;
        let n_c4 = 3.0 * (m as f64) * (m as f64);
        let n_c5 = (m as f64).powi(3);

        // Enumerate all 2^(3·frame_lanes) occupancy combinations, tracking
        // P(i), the degree-of-conflict case, and the vehicle count.
        let mut p_case = [0.0f64; 5]; // P(C1)..P(C5)
        let mut combos: Vec<(f64, u8, u32)> = Vec::with_capacity(1 << (3 * frame_lanes));
        let n_states = 1u32 << (3 * frame_lanes);
        for state in 0..n_states {
            let mut prob = 1.0;
            let mut occupied = [0u32; 3]; // per approach group: O, CL, CR
            for (g, pg) in [&p_o, &p_cl, &p_cr].iter().enumerate() {
                for (lane, &p_occ) in pg.iter().enumerate() {
                    let bit = (state >> (g * frame_lanes + lane)) & 1;
                    if bit == 1 {
                        prob *= p_occ;
                        occupied[g] += 1;
                    } else {
                        prob *= 1.0 - p_occ;
                    }
                }
            }
            let vehicles: u32 = occupied.iter().sum();
            // Degree-of-conflict case (Exhibits 21-5/21-6/21-7):
            let case = match (occupied[0] > 0, occupied[1] > 0, occupied[2] > 0) {
                (false, false, false) => 1,
                (true, false, false) => 2,
                (false, true, false) | (false, false, true) => 3,
                (true, true, false) | (true, false, true) | (false, true, true) => 4,
                (true, true, true) => 5,
            };
            p_case[(case - 1) as usize] += prob;
            combos.push((prob, case, vehicles));
        }

        // Probability adjustment factors (Equations 21-21 through 21-25 /
        // 21-40 through 21-44).
        let adj = [
            ALPHA * (p_case[1] + 2.0 * p_case[2] + 3.0 * p_case[3] + 4.0 * p_case[4]) / 1.0,
            ALPHA * (p_case[2] + 2.0 * p_case[3] + 3.0 * p_case[4] - p_case[1]) / n_c2,
            ALPHA * (p_case[3] + 2.0 * p_case[4] - 3.0 * p_case[2]) / n_c3,
            ALPHA * (p_case[4] - 6.0 * p_case[3]) / n_c4,
            -ALPHA * (10.0 * p_case[4]) / n_c5,
        ];

        // Departure headway h_d = Σ P'(i) h_si (Equations 21-26 through
        // 21-28) with h_si = h_base + h_adj (Equation 21-27, Exhibit 21-15).
        // The adjustment AdjP(i) is applied only to attainable combinations
        // (P(i) > 0), matching the Chapter 32 AWSC Example Problem 1
        // computation where only the applicable Exhibit 21-14 rows carry
        // adjusted probabilities.
        let mut h_d = 0.0;
        for (prob, case, vehicles) in combos {
            if prob <= 0.0 {
                continue;
            }
            let p_adj = prob + adj[(case - 1) as usize];
            let h_si = base_saturation_headway(case, vehicles, group) + h_adj;
            h_d += p_adj * h_si;
        }
        h_d
    }

    /// Run the Steps 5–11 departure-headway iteration for the current lane
    /// flow rates, filling `departure_headway` and `degree_of_utilization`
    /// on every lane. Returns the number of iterations used.
    ///
    /// The iteration starts each lane at 3.2 s (Step 5) and repeats until no
    /// lane's departure headway changes by more than `tolerance_s`
    /// (Step 11; HCM value 0.1 s), with an iteration cap of 100.
    pub fn iterate_departure_headways(&mut self, tolerance_s: f64) -> u32 {
        let frame_lanes = if self.max_lanes() >= 3 { 3 } else { 2 };
        let refs = self.lane_refs();
        // Current h_d estimates per approach lane
        let mut hd: [Vec<f64>; 4] = [
            vec![INITIAL_DEPARTURE_HEADWAY_S; self.eb.lanes.len()],
            vec![INITIAL_DEPARTURE_HEADWAY_S; self.wb.lanes.len()],
            vec![INITIAL_DEPARTURE_HEADWAY_S; self.nb.lanes.len()],
            vec![INITIAL_DEPARTURE_HEADWAY_S; self.sb.lanes.len()],
        ];
        let flows: [Vec<f64>; 4] = [
            self.eb.lanes.iter().map(|l| l.flow_rate.unwrap_or(0.0)).collect(),
            self.wb.lanes.iter().map(|l| l.flow_rate.unwrap_or(0.0)).collect(),
            self.nb.lanes.iter().map(|l| l.flow_rate.unwrap_or(0.0)).collect(),
            self.sb.lanes.iter().map(|l| l.flow_rate.unwrap_or(0.0)).collect(),
        ];
        let mut iterations = 0;
        for _ in 0..100 {
            iterations += 1;
            // Step 6: degree of utilization x = v h_d / 3,600 (Equation
            // 21-14), reset to 1 if it exceeds 1 during iteration.
            let mut x: [Vec<f64>; 4] = Default::default();
            for d in 0..4 {
                x[d] = flows[d]
                    .iter()
                    .zip(hd[d].iter())
                    .map(|(v, h)| (v * h / 3_600.0).min(1.0))
                    .collect();
            }
            // Steps 7–10 for every lane
            let mut new_hd = hd.clone();
            let mut max_delta = 0.0f64;
            for r in &refs {
                let group = self.approach(r.dir).geometry_group.unwrap_or(GeometryGroup::G1);
                let h_adj = self.approach(r.dir).lanes[r.lane]
                    .headway_adjustment
                    .unwrap_or(0.0);
                let h = self.departure_headway_for(*r, &x, frame_lanes, group, h_adj);
                max_delta = max_delta.max((h - hd[r.dir.idx()][r.lane]).abs());
                new_hd[r.dir.idx()][r.lane] = h;
            }
            hd = new_hd;
            // Step 11: convergence check
            if max_delta <= tolerance_s {
                break;
            }
        }
        // Store the converged values
        for r in &refs {
            let h = hd[r.dir.idx()][r.lane];
            let v = flows[r.dir.idx()][r.lane];
            let lane = &mut self.approach_mut(r.dir).lanes[r.lane];
            lane.departure_headway = Some(h);
            lane.degree_of_utilization = Some(v * h / 3_600.0);
        }
        self.iterations = Some(iterations);
        iterations
    }

    // ═══════════════════════════════════════════════════════════════════════
    // Step 12: capacity
    // ═══════════════════════════════════════════════════════════════════════

    /// HCM Chapter 21, Step 12: capacity of one lane, found by increasing
    /// the subject-lane flow rate (holding all other flows constant) until
    /// its degree of utilization reaches 1.0 (bisection search).
    pub fn capacity_of_lane(&self, dir: ApproachDir, lane: usize) -> f64 {
        let x_at = |v: f64| -> f64 {
            let mut trial = self.clone();
            trial.approach_mut(dir).lanes[lane].flow_rate = Some(v);
            trial.iterate_departure_headways(0.01);
            trial.approach(dir).lanes[lane]
                .degree_of_utilization
                .unwrap_or(0.0)
        };
        let (mut lo, mut hi) = (0.0, 400.0);
        // Expand until oversaturated (cap at 3,600 veh/h)
        while x_at(hi) < 1.0 && hi < 3_600.0 {
            lo = hi;
            hi += 200.0;
        }
        for _ in 0..30 {
            let mid = 0.5 * (lo + hi);
            if x_at(mid) < 1.0 {
                lo = mid;
            } else {
                hi = mid;
            }
            if hi - lo < 1.0 {
                break;
            }
        }
        0.5 * (lo + hi)
    }

    /// Step 12 for every lane (slow: runs a full iteration per bisection
    /// step). Stores `capacity` on each lane.
    pub fn step12_capacities(&mut self) {
        let refs = self.lane_refs();
        for r in refs {
            let c = self.capacity_of_lane(r.dir, r.lane);
            self.approach_mut(r.dir).lanes[r.lane].capacity = Some(c);
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    // Steps 13–16: service time, delay, LOS, queues
    // ═══════════════════════════════════════════════════════════════════════

    /// HCM Equation 21-33: 95th percentile queue, veh
    ///
    /// `Q_95 ≈ (900 T / h_d) [ (x - 1) + sqrt((x - 1)^2 + h_d x / (150 T)) ]`
    pub fn queue_95(x: f64, h_d: f64, t_h: f64) -> f64 {
        if h_d <= 0.0 {
            return 0.0;
        }
        900.0 * t_h / h_d
            * ((x - 1.0) + ((x - 1.0).powi(2) + h_d * x / (150.0 * t_h)).sqrt())
    }

    /// HCM Chapter 21, Steps 13, 14, and 16: service time
    /// (Equation 21-29), lane control delay and LOS (Equation 21-30,
    /// Exhibit 21-8), and 95th percentile queue (Equation 21-33).
    pub fn step13_14_16_lane_delay(&mut self) {
        let t = self.analysis_period_h;
        for dir in DIRS {
            let m = self
                .approach(dir)
                .geometry_group
                .map(|g| g.move_up_time())
                .unwrap_or(2.0);
            for lane in self.approach_mut(dir).lanes.iter_mut() {
                let (h_d, x) = match (lane.departure_headway, lane.degree_of_utilization) {
                    (Some(h), Some(x)) => (h, x),
                    _ => continue,
                };
                let ts = h_d - m;
                let d = control_delay_awsc(ts, h_d, x, t);
                lane.service_time = Some(ts);
                lane.control_delay = Some(d);
                lane.los = Some(los_char(los_unsignalized(d, x > 1.0)));
                lane.queue_95 = Some(Self::queue_95(x, h_d, t));
            }
        }
    }

    /// HCM Chapter 21, Step 15 (Equations 21-31 and 21-32): approach and
    /// intersection control delay and LOS. For approaches and the
    /// intersection, LOS is defined solely by control delay (Exhibit 21-8
    /// note a).
    pub fn step15_aggregate_delay(&mut self) -> f64 {
        let mut approach_pairs = Vec::new();
        for dir in DIRS {
            let pairs: Vec<(f64, f64)> = self
                .approach(dir)
                .lanes
                .iter()
                .filter_map(|l| Some((l.control_delay?, l.flow_rate?)))
                .collect();
            if pairs.is_empty() {
                continue;
            }
            let d_a = aggregate_control_delay(&pairs);
            let v_a: f64 = pairs.iter().map(|(_, v)| v).sum();
            let a = self.approach_mut(dir);
            a.control_delay = Some(d_a);
            a.los = Some(los_char(los_unsignalized(d_a, false)));
            approach_pairs.push((d_a, v_a));
        }
        let d_i = aggregate_control_delay(&approach_pairs);
        self.intersection_delay = Some(d_i);
        self.intersection_los = Some(los_char(los_unsignalized(d_i, false)));
        d_i
    }

    /// Run the complete Chapter 21 procedure (Steps 1–16, excluding the
    /// expensive Step 12 capacity search; call [`Awsc::step12_capacities`]
    /// separately if lane capacities are needed).
    pub fn analyze(&mut self) {
        self.step1_2_flow_rates();
        self.step3_geometry_groups();
        self.step4_headway_adjustments();
        self.iterate_departure_headways(CONVERGENCE_TOLERANCE_S);
        self.step13_14_16_lane_delay();
        self.step15_aggregate_delay();
    }
}

/// Convert a [`crate::hcm::common::LevelOfService`] to its letter.
fn los_char(los: crate::hcm::common::LevelOfService) -> char {
    los.into()
}
