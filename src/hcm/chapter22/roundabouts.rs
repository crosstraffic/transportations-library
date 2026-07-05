//! # Roundabouts (HCM Chapter 22)
//!
//! Implements the HCM 7th Edition Chapter 22 motorized vehicle core
//! methodology (Section 3) with the capacity-model calibration extension
//! (Section 4, Equations 22-21 through 22-23).
//!
//! ## Computational steps (HCM Exhibit 22-10)
//!
//! 1. Convert movement demand volumes to flow rates (Equation 22-8)
//! 2. Adjust flow rates for heavy vehicles (Equations 22-9 and 22-10,
//!    Exhibit 22-11)
//! 3. Determine circulating and exiting flow rates (Equations 22-11 and
//!    22-12)
//! 4. Determine entry flow rates by lane (Exhibits 22-14 and 22-15)
//! 5. Determine entry-lane and bypass-lane capacity in pc/h
//!    (Equations 22-1 through 22-7, Exhibits 22-16 and 22-17)
//! 6. Determine pedestrian impedance (Exhibits 22-18 and 22-20)
//! 7. Convert flow rates and capacities to veh/h (Equations 22-13 through
//!    22-15)
//! 8. Compute the volume-to-capacity ratio per lane (Equation 22-16)
//! 9. Compute control delay per lane (Equation 22-17)
//! 10. Determine LOS per lane (Exhibit 22-8)
//! 11. Aggregate delay/LOS per approach and intersection (Equations 22-18
//!    and 22-19)
//! 12. Compute 95th percentile queues (Equation 22-20)
//!
//! The geometry convention is a standard four-leg roundabout: the NB entry
//! is on the south leg, SB on the north leg, EB on the west leg, and WB on
//! the east leg.

use serde::{Deserialize, Serialize};

use crate::hcm::common::delay::{aggregate_control_delay, control_delay_roundabout};
use crate::hcm::common::los_tables::los_unsignalized;

/// Passenger car equivalent for heavy vehicles (HCM Exhibit 22-11).
pub const E_T_HEAVY_VEHICLE: f64 = 2.0;

// ═══════════════════════════════════════════════════════════════════════════════
// Entry capacity models (Equations 22-1 through 22-7)
// ═══════════════════════════════════════════════════════════════════════════════

/// Generalized Siegloch capacity form `c_pce = A e^(-B v_c)`
/// (HCM Equation 22-21).
pub fn capacity_exponential(a: f64, b: f64, v_c_pce: f64) -> f64 {
    a * (-b * v_c_pce).exp()
}

/// HCM Equation 22-1: capacity of a one-lane entry conflicted by one
/// circulating lane: `c_e,pce = 1,380 e^(-1.02e-3 v_c,pce)`.
pub fn capacity_single_lane(v_c_pce: f64) -> f64 {
    capacity_exponential(1_380.0, 1.02e-3, v_c_pce)
}

/// HCM Equation 22-2: capacity of each lane of a two-lane entry conflicted
/// by one circulating lane: `c_e,pce = 1,420 e^(-0.91e-3 v_c,pce)`.
pub fn capacity_two_lane_entry_one_circ(v_c_pce: f64) -> f64 {
    capacity_exponential(1_420.0, 0.91e-3, v_c_pce)
}

/// HCM Equation 22-3: capacity of a one-lane entry conflicted by two
/// circulating lanes: `c_e,pce = 1,420 e^(-0.85e-3 v_c,pce)`
/// (v_c is the total of both lanes).
pub fn capacity_one_lane_entry_two_circ(v_c_pce: f64) -> f64 {
    capacity_exponential(1_420.0, 0.85e-3, v_c_pce)
}

/// HCM Equation 22-4: capacity of the right lane of a two-lane entry
/// conflicted by two circulating lanes:
/// `c_e,R,pce = 1,420 e^(-0.85e-3 v_c,pce)`.
pub fn capacity_two_lane_entry_two_circ_right(v_c_pce: f64) -> f64 {
    capacity_exponential(1_420.0, 0.85e-3, v_c_pce)
}

/// HCM Equation 22-5: capacity of the left lane of a two-lane entry
/// conflicted by two circulating lanes:
/// `c_e,L,pce = 1,350 e^(-0.92e-3 v_c,pce)`.
pub fn capacity_two_lane_entry_two_circ_left(v_c_pce: f64) -> f64 {
    capacity_exponential(1_350.0, 0.92e-3, v_c_pce)
}

/// HCM Equation 22-6: capacity of a yielding bypass lane opposed by one
/// exiting lane: `c_bypass,pce = 1,380 e^(-1.02e-3 v_ex,pce)`.
pub fn capacity_bypass_one_exit_lane(v_ex_pce: f64) -> f64 {
    capacity_exponential(1_380.0, 1.02e-3, v_ex_pce)
}

/// HCM Equation 22-7: capacity of a yielding bypass lane opposed by two
/// exiting lanes: `c_bypass,pce = 1,420 e^(-0.85e-3 v_ex,pce)`.
pub fn capacity_bypass_two_exit_lanes(v_ex_pce: f64) -> f64 {
    capacity_exponential(1_420.0, 0.85e-3, v_ex_pce)
}

/// HCM Equation 22-22: calibrated intercept `A = 3,600 / t_f`.
pub fn calibrated_intercept_a(t_f_s: f64) -> f64 {
    3_600.0 / t_f_s
}

/// HCM Equation 22-23: calibrated slope `B = (t_c - t_f/2) / 3,600`.
pub fn calibrated_slope_b(t_c_s: f64, t_f_s: f64) -> f64 {
    (t_c_s - t_f_s / 2.0) / 3_600.0
}

// ═══════════════════════════════════════════════════════════════════════════════
// Pedestrian impedance (Exhibits 22-18 and 22-20)
// ═══════════════════════════════════════════════════════════════════════════════

/// HCM Exhibit 22-18: entry capacity adjustment factor for pedestrians
/// crossing a one-lane entry (assuming pedestrian priority).
///
/// * `n_ped` — conflicting pedestrians, p/h
/// * `v_c_pce` — conflicting circulating flow, pc/h
pub fn ped_factor_one_lane(n_ped: f64, v_c_pce: f64) -> f64 {
    if v_c_pce > 881.0 {
        1.0
    } else if n_ped <= 101.0 {
        (1.0 - 0.000_137 * n_ped).min(1.0)
    } else {
        ((1_119.5 - 0.715 * v_c_pce - 0.644 * n_ped + 0.000_73 * v_c_pce * n_ped)
            / (1_068.6 - 0.654 * v_c_pce))
            .clamp(0.0, 1.0)
    }
}

/// HCM Exhibit 22-20: entry capacity adjustment factor for pedestrians
/// crossing a two-lane entry (assuming pedestrian priority).
pub fn ped_factor_two_lane(n_ped: f64, v_c_pce: f64) -> f64 {
    let f_100 = (1_260.6 - 0.329 * v_c_pce - 0.381 * 100.0) / (1_380.0 - 0.5 * v_c_pce);
    let f = if n_ped < 100.0 {
        1.0 - n_ped / 100.0 * (1.0 - f_100)
    } else {
        (1_260.6 - 0.329 * v_c_pce - 0.381 * n_ped) / (1_380.0 - 0.5 * v_c_pce)
    };
    f.clamp(0.0, 1.0)
}

// ═══════════════════════════════════════════════════════════════════════════════
// Input model
// ═══════════════════════════════════════════════════════════════════════════════

/// Entry (approach) direction of travel. NB enters from the south leg, SB
/// from the north leg, EB from the west leg, WB from the east leg.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Leg {
    NB,
    SB,
    EB,
    WB,
}

impl Leg {
    /// Opposite entry.
    pub fn opposite(self) -> Leg {
        match self {
            Leg::NB => Leg::SB,
            Leg::SB => Leg::NB,
            Leg::EB => Leg::WB,
            Leg::WB => Leg::EB,
        }
    }

    /// Entry on the subject driver's left (e.g., the west leg for a
    /// northbound driver).
    pub fn left_of(self) -> Leg {
        match self {
            Leg::NB => Leg::EB,
            Leg::SB => Leg::WB,
            Leg::EB => Leg::SB,
            Leg::WB => Leg::NB,
        }
    }

    /// Entry on the subject driver's right.
    pub fn right_of(self) -> Leg {
        match self {
            Leg::NB => Leg::WB,
            Leg::SB => Leg::EB,
            Leg::EB => Leg::NB,
            Leg::WB => Leg::SB,
        }
    }
}

const LEGS: [Leg; 4] = [Leg::NB, Leg::SB, Leg::EB, Leg::WB];

/// Right-turn bypass lane type (HCM Exhibit 22-7).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum BypassType {
    /// No bypass lane; right turns use the entry.
    #[default]
    None,
    /// Type 1: yielding bypass lane (capacity per Equations 22-6/22-7).
    Yielding,
    /// Type 2: nonyielding (merging) bypass lane; delay assumed 0.
    NonYielding,
}

/// Designated lane assignment for a two-lane entry (HCM Exhibit 22-14).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum LaneAssignment {
    /// Left lane: left only; right lane: through + right
    /// (Exhibit 22-15 case 1: "L, TR").
    LeftAndThroughRight,
    /// Left lane: left + through; right lane: right only (case 2: "LT, R").
    LeftThroughAndRight,
    /// Left lane: left + through; right lane: through + right (case 3:
    /// "LT, TR", the default).
    #[default]
    LeftThroughAndThroughRight,
    /// Left lane: left only; right lane: left–through–right (case 4:
    /// "L, LTR").
    LeftAndAllMovements,
    /// Left lane: left–through–right; right lane: right only (case 5:
    /// "LTR, R").
    AllMovementsAndRight,
}

/// One roundabout approach (entry).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoundaboutApproach {
    /// U-turn demand volume, veh/h.
    #[serde(default)]
    pub v_u: f64,
    /// Left-turn demand volume, veh/h.
    #[serde(default)]
    pub v_l: f64,
    /// Through demand volume, veh/h.
    #[serde(default)]
    pub v_t: f64,
    /// Right-turn demand volume, veh/h.
    #[serde(default)]
    pub v_r: f64,
    /// Percentage of heavy vehicles on the approach (%).
    #[serde(default)]
    pub heavy_vehicle_pct: f64,
    /// Number of entry lanes (1 or 2).
    #[serde(default = "one")]
    pub entry_lanes: u32,
    /// Number of circulating lanes conflicting with the entry (1 or 2).
    #[serde(default = "one")]
    pub circulating_lanes: u32,
    /// Number of exiting lanes on this leg (used by an upstream bypass
    /// lane's capacity, Exhibit 22-17).
    #[serde(default = "one")]
    pub exiting_lanes: u32,
    /// Right-turn bypass lane type.
    #[serde(default)]
    pub bypass: BypassType,
    /// Designated lane assignment for two-lane entries (Exhibit 22-14).
    #[serde(default)]
    pub lane_assignment: LaneAssignment,
    /// Percentage of entry traffic using the left lane for shared
    /// assignments (Exhibit 22-9 defaults: 0.47 for LT+TR and LTR+R, 0.53
    /// for L+LTR). `None` selects the default.
    #[serde(default)]
    pub pct_left_lane: Option<f64>,
    /// Conflicting pedestrians crossing this entry, p/h.
    #[serde(default)]
    pub n_ped: f64,

    // ── Computed ────────────────────────────────────────────────────────────
    /// Movement flow rates [U, L, T, R] in pc/h (Steps 1–2).
    #[serde(default)]
    pub flows_pce: Option<[f64; 4]>,
    /// Conflicting circulating flow, pc/h (Equation 22-11).
    #[serde(default)]
    pub circulating_flow_pce: Option<f64>,
    /// Conflicting exiting flow for the bypass lane, pc/h (Equation 22-12).
    #[serde(default)]
    pub bypass_conflicting_flow_pce: Option<f64>,
    /// Entry-lane results (left-to-right; one entry for one-lane entries).
    #[serde(default)]
    pub lanes: Vec<RoundaboutLaneResult>,
    /// Bypass-lane result, if present.
    #[serde(default)]
    pub bypass_lane: Option<RoundaboutLaneResult>,
    /// Approach control delay, s/veh (Equation 22-18, bypass included).
    #[serde(default)]
    pub control_delay: Option<f64>,
    /// Approach LOS (Exhibit 22-8, delay-based).
    #[serde(default)]
    pub los: Option<char>,
}

fn one() -> u32 {
    1
}

impl Default for RoundaboutApproach {
    fn default() -> Self {
        RoundaboutApproach {
            v_u: 0.0,
            v_l: 0.0,
            v_t: 0.0,
            v_r: 0.0,
            heavy_vehicle_pct: 0.0,
            entry_lanes: 1,
            circulating_lanes: 1,
            exiting_lanes: 1,
            bypass: BypassType::None,
            lane_assignment: LaneAssignment::default(),
            pct_left_lane: None,
            n_ped: 0.0,
            flows_pce: None,
            circulating_flow_pce: None,
            bypass_conflicting_flow_pce: None,
            lanes: Vec::new(),
            bypass_lane: None,
            control_delay: None,
            los: None,
        }
    }
}

/// Per-lane computed results.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RoundaboutLaneResult {
    /// Label ("left", "right", "entry", or "bypass").
    pub label: String,
    /// Lane flow rate, pc/h (Step 4).
    pub flow_pce: f64,
    /// Lane flow rate, veh/h (Equation 22-13).
    pub flow_veh: f64,
    /// Lane capacity, pc/h (Step 5).
    pub capacity_pce: f64,
    /// Lane capacity, veh/h (Equation 22-14, including f_HV and f_ped).
    pub capacity_veh: f64,
    /// Volume-to-capacity ratio (Equation 22-16).
    pub v_c_ratio: f64,
    /// Control delay, s/veh (Equation 22-17; 0 for a nonyielding bypass).
    pub control_delay: f64,
    /// Level of service (Exhibit 22-8; v/c > 1.0 assigned F).
    pub los: char,
    /// 95th percentile queue, veh (Equation 22-20).
    pub queue_95: f64,
}

/// HCM Chapter 22 roundabout motorized vehicle analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Roundabouts {
    /// Northbound entry (south leg).
    pub nb: RoundaboutApproach,
    /// Southbound entry (north leg).
    pub sb: RoundaboutApproach,
    /// Eastbound entry (west leg).
    pub eb: RoundaboutApproach,
    /// Westbound entry (east leg).
    pub wb: RoundaboutApproach,
    /// Peak hour factor (Equation 22-8). `None` if volumes are flow rates.
    #[serde(default)]
    pub phf: Option<f64>,
    /// Analysis period T, h (0.25 for 15 min).
    #[serde(default = "default_t")]
    pub analysis_period_h: f64,
    /// Optional calibrated (A, B) parameters replacing every entry-lane
    /// capacity equation (Section 4, Equations 22-21 through 22-23).
    #[serde(default)]
    pub calibration: Option<(f64, f64)>,

    // ── Computed ────────────────────────────────────────────────────────────
    /// Intersection control delay, s/veh (Equation 22-19).
    #[serde(default)]
    pub intersection_delay: Option<f64>,
    /// Intersection LOS (Exhibit 22-8).
    #[serde(default)]
    pub intersection_los: Option<char>,
}

fn default_t() -> f64 {
    0.25
}

impl Roundabouts {
    /// Create an analysis from the four approaches.
    pub fn new(
        nb: RoundaboutApproach,
        sb: RoundaboutApproach,
        eb: RoundaboutApproach,
        wb: RoundaboutApproach,
    ) -> Self {
        Roundabouts {
            nb,
            sb,
            eb,
            wb,
            phf: None,
            analysis_period_h: 0.25,
            calibration: None,
            intersection_delay: None,
            intersection_los: None,
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

    /// Approach accessor by leg.
    pub fn approach(&self, leg: Leg) -> &RoundaboutApproach {
        match leg {
            Leg::NB => &self.nb,
            Leg::SB => &self.sb,
            Leg::EB => &self.eb,
            Leg::WB => &self.wb,
        }
    }

    fn approach_mut(&mut self, leg: Leg) -> &mut RoundaboutApproach {
        match leg {
            Leg::NB => &mut self.nb,
            Leg::SB => &mut self.sb,
            Leg::EB => &mut self.eb,
            Leg::WB => &mut self.wb,
        }
    }

    /// HCM Equation 22-10: heavy-vehicle adjustment factor
    /// `f_HV = 1 / (1 + P_T (E_T - 1))`.
    pub fn heavy_vehicle_factor(pct_heavy: f64) -> f64 {
        1.0 / (1.0 + pct_heavy / 100.0 * (E_T_HEAVY_VEHICLE - 1.0))
    }

    // ═══════════════════════════════════════════════════════════════════════
    // Steps 1–2: flow rates in pc/h
    // ═══════════════════════════════════════════════════════════════════════

    /// HCM Equations 22-8 through 22-10 (Exhibit 22-11): convert demand
    /// volumes to heavy-vehicle-adjusted flow rates, pc/h.
    pub fn step1_2_flow_rates_pce(&mut self) {
        let phf = match self.phf {
            Some(p) if p > 0.0 => p,
            _ => 1.0,
        };
        for leg in LEGS {
            let a = self.approach_mut(leg);
            let f_hv = Self::heavy_vehicle_factor(a.heavy_vehicle_pct);
            let conv = |v: f64| v / phf / f_hv;
            a.flows_pce = Some([conv(a.v_u), conv(a.v_l), conv(a.v_t), conv(a.v_r)]);
        }
    }

    fn pce(&self, leg: Leg) -> [f64; 4] {
        self.approach(leg).flows_pce.unwrap_or([0.0; 4])
    }

    // ═══════════════════════════════════════════════════════════════════════
    // Step 3: circulating and exiting flows
    // ═══════════════════════════════════════════════════════════════════════

    /// HCM Equation 22-11: conflicting circulating flow for an entry, pc/h.
    ///
    /// For the northbound entry:
    /// `v_c,NB,pce = v_WB,U + v_SB,L + v_SB,U + v_EB,T + v_EB,L + v_EB,U`
    /// (the U-turn of the right leg, left + U of the opposite leg, and
    /// through + left + U of the left leg).
    pub fn circulating_flow_pce(&self, leg: Leg) -> f64 {
        let [u_r, _, _, _] = self.pce(leg.right_of());
        let [u_o, l_o, _, _] = self.pce(leg.opposite());
        let [u_l, l_l, t_l, _] = self.pce(leg.left_of());
        u_r + l_o + u_o + t_l + l_l + u_l
    }

    /// HCM Equation 22-12: exiting flow on the leg the given entry's right
    /// turn merges into (conflicting flow for a yielding bypass lane),
    /// pc/h. Right turns using an upstream bypass are excluded.
    ///
    /// For the westbound bypass (merging into the northbound exit):
    /// `v_ex,NB,pce = v_SB,U + v_EB,L + v_NB,T + v_WB,R − v_WB,R,bypass`.
    pub fn bypass_conflicting_exit_flow_pce(&self, entry: Leg) -> f64 {
        // The exit leg the bypass merges into carries the exit direction of
        // `entry.left_of()` (e.g., WB right turns exit northbound).
        let exit_dir = entry.left_of();
        let [u_o, _, _, _] = self.pce(exit_dir.opposite());
        let [_, l_l, _, _] = self.pce(exit_dir.left_of());
        let [_, _, t_e, _] = self.pce(exit_dir);
        let [_, _, _, r_r] = self.pce(exit_dir.right_of());
        // Right turns from `entry` itself (= exit_dir.right_of()) that use
        // the bypass are removed.
        let r_bypass = if self.approach(entry).bypass != BypassType::None {
            r_r
        } else {
            0.0
        };
        u_o + l_l + t_e + r_r - r_bypass
    }

    /// HCM Chapter 22, Step 3: populate circulating (and bypass exiting)
    /// flows for every entry.
    pub fn step3_conflicting_flows(&mut self) {
        for leg in LEGS {
            let vc = self.circulating_flow_pce(leg);
            let vex = if self.approach(leg).bypass != BypassType::None {
                Some(self.bypass_conflicting_exit_flow_pce(leg))
            } else {
                None
            };
            let a = self.approach_mut(leg);
            a.circulating_flow_pce = Some(vc);
            a.bypass_conflicting_flow_pce = vex;
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    // Step 4: entry flow rates by lane
    // ═══════════════════════════════════════════════════════════════════════

    /// HCM Chapter 22, Step 4 (Exhibits 22-14 and 22-15): assign entry
    /// flows to lanes. Returns `(left_lane_pce, right_lane_pce)`; a
    /// one-lane entry places everything in the right lane slot.
    ///
    /// Right-turn flow is excluded when a bypass lane is present.
    pub fn entry_lane_flows_pce(&self, leg: Leg) -> (f64, f64) {
        let a = self.approach(leg);
        let [u, l, t, r_all] = self.pce(leg);
        let r_e = if a.bypass != BypassType::None { 0.0 } else { r_all };
        let ve = u + l + t + r_e;
        if a.entry_lanes <= 1 {
            return (0.0, ve);
        }
        use LaneAssignment as La;
        // Exhibit 22-14 de facto lane checks
        let assignment = match a.lane_assignment {
            La::LeftThroughAndThroughRight => {
                if u + l > t + r_e {
                    La::LeftAndThroughRight // de facto left-turn lane
                } else if r_e > u + l + t {
                    La::LeftThroughAndRight // de facto right-turn lane
                } else {
                    La::LeftThroughAndThroughRight
                }
            }
            La::LeftAndAllMovements => {
                if t + r_e > u + l {
                    La::LeftAndThroughRight // de facto through–right lane
                } else {
                    La::LeftAndAllMovements
                }
            }
            La::AllMovementsAndRight => {
                if u + l + t > r_e {
                    La::LeftThroughAndRight // de facto left–through lane
                } else {
                    La::AllMovementsAndRight
                }
            }
            other => other,
        };
        // Exhibit 22-15 volume assignments
        match assignment {
            La::LeftAndThroughRight => (u + l, t + r_e),
            La::LeftThroughAndRight => (u + l + t, r_e),
            La::LeftThroughAndThroughRight
            | La::LeftAndAllMovements
            | La::AllMovementsAndRight => {
                // Exhibit 22-9 lane-utilization defaults
                let pct_ll = a.pct_left_lane.unwrap_or(match assignment {
                    La::LeftAndAllMovements => 0.53,
                    _ => 0.47,
                });
                (pct_ll * ve, (1.0 - pct_ll) * ve)
            }
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    // Steps 5–12 per approach
    // ═══════════════════════════════════════════════════════════════════════

    /// Entry-lane capacity in pc/h per HCM Exhibit 22-16 (Equations 22-1
    /// through 22-5), `lane_is_left` selecting Equation 22-5 for the left
    /// lane of a 2×2 configuration. A calibration `(A, B)` overrides the
    /// national models (Equation 22-21).
    fn entry_capacity_pce(&self, leg: Leg, lane_is_left: bool, v_c: f64) -> f64 {
        if let Some((a, b)) = self.calibration {
            return capacity_exponential(a, b, v_c);
        }
        let ap = self.approach(leg);
        match (ap.entry_lanes, ap.circulating_lanes) {
            (1, 1) => capacity_single_lane(v_c),
            (2, 1) => capacity_two_lane_entry_one_circ(v_c),
            (1, _) => capacity_one_lane_entry_two_circ(v_c),
            (_, _) => {
                if lane_is_left {
                    capacity_two_lane_entry_two_circ_left(v_c)
                } else {
                    capacity_two_lane_entry_two_circ_right(v_c)
                }
            }
        }
    }

    /// HCM Equation 22-20: 95th percentile queue, veh.
    pub fn queue_95(x: f64, c_veh: f64, t_h: f64) -> f64 {
        if c_veh <= 0.0 {
            return 0.0;
        }
        900.0 * t_h
            * (x - 1.0 + ((1.0 - x).powi(2) + (3_600.0 / c_veh) * x / (150.0 * t_h)).sqrt())
            * (c_veh / 3_600.0)
    }

    /// HCM Chapter 22, Steps 4 through 10 and 12 for every approach:
    /// lane flows, capacities, pedestrian impedance, veh/h conversion, v/c,
    /// control delay, LOS, and queues.
    pub fn step4_12_lane_performance(&mut self) {
        let t = self.analysis_period_h;
        for leg in LEGS {
            let (left_pce, right_pce) = self.entry_lane_flows_pce(leg);
            let v_c = self.approach(leg).circulating_flow_pce.unwrap_or(0.0);
            let n_ped = self.approach(leg).n_ped;
            let entry_lanes = self.approach(leg).entry_lanes;
            // Step 6: pedestrian impedance (Exhibits 22-18 and 22-20)
            let f_ped = if entry_lanes >= 2 {
                ped_factor_two_lane(n_ped, v_c)
            } else {
                ped_factor_one_lane(n_ped, v_c)
            };
            let f_hv = Self::heavy_vehicle_factor(self.approach(leg).heavy_vehicle_pct);

            let mut lanes = Vec::new();
            let lane_specs: Vec<(&str, f64, bool)> = if entry_lanes >= 2 {
                vec![("left", left_pce, true), ("right", right_pce, false)]
            } else {
                vec![("entry", right_pce, false)]
            };
            for (label, v_pce, is_left) in lane_specs {
                // Step 5: capacity in pc/h
                let c_pce = self.entry_capacity_pce(leg, is_left, v_c);
                // Step 7 (Equations 22-13 and 22-14)
                let v_veh = v_pce * f_hv;
                let c_veh = c_pce * f_hv * f_ped;
                // Step 8 (Equation 22-16)
                let x = if c_veh > 0.0 { v_veh / c_veh } else { 0.0 };
                // Step 9 (Equation 22-17)
                let d = control_delay_roundabout(v_veh, c_veh, t);
                lanes.push(RoundaboutLaneResult {
                    label: label.to_string(),
                    flow_pce: v_pce,
                    flow_veh: v_veh,
                    capacity_pce: c_pce,
                    capacity_veh: c_veh,
                    v_c_ratio: x,
                    control_delay: d,
                    los: los_char(los_unsignalized(d, x > 1.0)),
                    queue_95: Self::queue_95(x, c_veh, t),
                });
            }

            // Bypass lane (Steps 5–12)
            let bypass_lane = match self.approach(leg).bypass {
                BypassType::None => None,
                bypass => {
                    let [_, _, _, r_pce] = self.pce(leg);
                    let v_veh = r_pce * f_hv;
                    match bypass {
                        BypassType::Yielding => {
                            let v_ex =
                                self.approach(leg).bypass_conflicting_flow_pce.unwrap_or(0.0);
                            // Exhibit 22-17: exit lanes of the leg the
                            // bypass merges into
                            let exit_lanes =
                                self.approach(leg.left_of()).exiting_lanes.max(1);
                            let c_pce = if exit_lanes >= 2 {
                                capacity_bypass_two_exit_lanes(v_ex)
                            } else {
                                capacity_bypass_one_exit_lane(v_ex)
                            };
                            let c_veh = c_pce * f_hv; // pedestrians not modeled
                            let x = if c_veh > 0.0 { v_veh / c_veh } else { 0.0 };
                            let d = control_delay_roundabout(v_veh, c_veh, t);
                            Some(RoundaboutLaneResult {
                                label: "bypass".to_string(),
                                flow_pce: r_pce,
                                flow_veh: v_veh,
                                capacity_pce: c_pce,
                                capacity_veh: c_veh,
                                v_c_ratio: x,
                                control_delay: d,
                                los: los_char(los_unsignalized(d, x > 1.0)),
                                queue_95: Self::queue_95(x, c_veh, t),
                            })
                        }
                        _ => Some(RoundaboutLaneResult {
                            // Type 2 nonyielding bypass: delay assumed 0
                            // (HCM Chapter 33 Example Problem 1 treatment)
                            label: "bypass".to_string(),
                            flow_pce: r_pce,
                            flow_veh: v_veh,
                            capacity_pce: 0.0,
                            capacity_veh: 0.0,
                            v_c_ratio: 0.0,
                            control_delay: 0.0,
                            los: 'A',
                            queue_95: 0.0,
                        }),
                    }
                }
            };
            let a = self.approach_mut(leg);
            a.lanes = lanes;
            a.bypass_lane = bypass_lane;
        }
    }

    /// HCM Chapter 22, Step 11 (Equations 22-18 and 22-19): approach and
    /// intersection control delay and LOS.
    pub fn step11_aggregate_delay(&mut self) -> f64 {
        let mut intersection_pairs = Vec::new();
        for leg in LEGS {
            let a = self.approach(leg);
            let mut pairs: Vec<(f64, f64)> = a
                .lanes
                .iter()
                .map(|l| (l.control_delay, l.flow_veh))
                .collect();
            if let Some(b) = &a.bypass_lane {
                pairs.push((b.control_delay, b.flow_veh));
            }
            if pairs.iter().all(|(_, v)| *v <= 0.0) {
                continue;
            }
            let d_a = aggregate_control_delay(&pairs);
            let v_a: f64 = pairs.iter().map(|(_, v)| v).sum();
            let a = self.approach_mut(leg);
            a.control_delay = Some(d_a);
            a.los = Some(los_char(los_unsignalized(d_a, false)));
            intersection_pairs.push((d_a, v_a));
        }
        let d_i = aggregate_control_delay(&intersection_pairs);
        self.intersection_delay = Some(d_i);
        self.intersection_los = Some(los_char(los_unsignalized(d_i, false)));
        d_i
    }

    /// Run the complete Chapter 22 procedure (Steps 1–12).
    pub fn analyze(&mut self) {
        self.step1_2_flow_rates_pce();
        self.step3_conflicting_flows();
        self.step4_12_lane_performance();
        self.step11_aggregate_delay();
    }
}

/// Convert a [`crate::hcm::common::LevelOfService`] to its letter.
fn los_char(los: crate::hcm::common::LevelOfService) -> char {
    los.into()
}
