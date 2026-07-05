//! Common intersection data model shared by HCM Chapters 19–23
//! (signalized intersections, TWSC, AWSC, roundabouts, and ramp terminals).
//!
//! Movement numbering follows the NEMA scheme illustrated in
//! HCM 7th Edition Exhibit 19-1 (Intersection Traffic Movements and
//! Numbering Scheme).

use serde::{Deserialize, Serialize};

// ═══════════════════════════════════════════════════════════════════════════════
// Enumerations
// ═══════════════════════════════════════════════════════════════════════════════

/// Turning movement type at an intersection approach.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TurnType {
    Left,
    Through,
    Right,
    UTurn,
}

/// Cardinal direction of travel of an approach (the direction vehicles are
/// heading, e.g., `NB` = northbound traffic entering from the south leg).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Direction {
    NB,
    SB,
    EB,
    WB,
}

/// Intersection control type covered by HCM Chapters 19–23.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ControlType {
    /// Pretimed traffic signal (HCM Chapter 19)
    PretimedSignal,
    /// Fully actuated traffic signal (HCM Chapter 19)
    ActuatedSignal,
    /// Semiactuated traffic signal (HCM Chapter 19)
    SemiActuatedSignal,
    /// Two-way STOP control (HCM Chapter 20)
    TwoWayStop,
    /// All-way STOP control (HCM Chapter 21)
    AllWayStop,
    /// Roundabout / YIELD-on-entry circular intersection (HCM Chapter 22)
    Roundabout,
    /// YIELD control on the minor approaches
    YieldControl,
}

// ═══════════════════════════════════════════════════════════════════════════════
// NEMA movement numbering (HCM Exhibit 19-1)
// ═══════════════════════════════════════════════════════════════════════════════

/// HCM Exhibit 19-1: Intersection Traffic Movements and Numbering Scheme.
///
/// Left-turn and through movements carry NEMA phase numbers 1–8:
///
/// | Approach | Left | Through | Right |
/// |----------|------|---------|-------|
/// | EB       |  5   |    2    |  12   |
/// | WB       |  1   |    6    |  16   |
/// | NB       |  3   |    8    |  18   |
/// | SB       |  7   |    4    |  14   |
///
/// Right turns are numbered as the approach through-movement number plus 10
/// (12, 14, 16, 18). Pedestrian crossings are denoted 2P, 4P, 6P, and 8P,
/// matching the concurrent vehicular through phase.
///
/// U-turns have no NEMA phase number: Exhibit 19-1 depicts only left,
/// through, and right movements per approach, so there is no phase slot to
/// assign a U-turn to, and `None` is returned for every `TurnType::UTurn`
/// regardless of `direction`. This is unrelated to (and not fixed by) HCM
/// Chapter 20's Exhibit 20-1, which uses its own, non-NEMA movement
/// numbering scheme for two-way STOP-controlled (TWSC) intersections and
/// does assign the major-street U-turns adjunct labels 1U and 4U there; see
/// [`ch20_uturn_label`] for that separate, chapter-specific convention (and
/// `chapter20::twsc::Mv::M1U`/`M4U` for the TWSC-native representation).
/// Downstream Chapter 19 code relies on `None` here for `UTurn`, so this
/// function's return value for `TurnType::UTurn` must not change.
pub fn nema_movement_number(direction: Direction, turn_type: TurnType) -> Option<u8> {
    match (direction, turn_type) {
        (Direction::EB, TurnType::Left) => Some(5),
        (Direction::EB, TurnType::Through) => Some(2),
        (Direction::EB, TurnType::Right) => Some(12),
        (Direction::WB, TurnType::Left) => Some(1),
        (Direction::WB, TurnType::Through) => Some(6),
        (Direction::WB, TurnType::Right) => Some(16),
        (Direction::NB, TurnType::Left) => Some(3),
        (Direction::NB, TurnType::Through) => Some(8),
        (Direction::NB, TurnType::Right) => Some(18),
        (Direction::SB, TurnType::Left) => Some(7),
        (Direction::SB, TurnType::Through) => Some(4),
        (Direction::SB, TurnType::Right) => Some(14),
        (_, TurnType::UTurn) => None,
    }
}

/// HCM Chapter 20 Exhibit 20-1: adjunct U-turn movement label ("1U"/"4U")
/// for two-way STOP-controlled (TWSC) intersections.
///
/// Unlike Exhibit 19-1's NEMA phase numbers (see [`nema_movement_number`],
/// which always returns `None` for `TurnType::UTurn`), Chapter 20 defines
/// its own consecutive 1-12 movement numbering for TWSC intersections and
/// gives the two major-street U-turns adjunct labels: 1U (paired with the
/// major-street EB movements 1/2/3) and 4U (paired with the major-street WB
/// movements 4/5/6). Minor-street U-turns (NB, SB) are not assigned a label
/// in Exhibit 20-1 (shown as "—"), so this returns `None` for those
/// directions.
///
/// This is a separate, chapter-specific convention from
/// [`nema_movement_number`]: the two exhibits assign different numeric
/// slots to the same cardinal directions (e.g., movement 1 is a WB left
/// turn under Exhibit 19-1's NEMA scheme but an EB left turn under Exhibit
/// 20-1) and are not interchangeable. `chapter20::twsc::Mv::M1U`/`M4U` is
/// the TWSC module's own native representation of the same two movements;
/// this function exists so callers working with the common `Direction`
/// type can look up the label without depending on `chapter20::twsc`.
pub fn ch20_uturn_label(direction: Direction) -> Option<&'static str> {
    match direction {
        Direction::EB => Some("1U"),
        Direction::WB => Some("4U"),
        Direction::NB | Direction::SB => None,
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Data model
// ═══════════════════════════════════════════════════════════════════════════════

/// A single vehicular movement at an intersection approach.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Movement {
    /// NEMA movement number per HCM Exhibit 19-1 (1–8 for lefts/throughs,
    /// 12/14/16/18 for right turns). `None` if not assigned (e.g., U-turns).
    pub movement_no: Option<u8>,
    /// Turning movement type.
    pub turn_type: TurnType,
    /// Hourly demand volume, veh/h.
    pub volume: f64,
    /// Peak hour factor (unitless, 0–1). `None` if volumes are already
    /// flow rates for the analysis period.
    pub phf: Option<f64>,
    /// Heavy vehicle percentage (%). `None` if unknown.
    pub heavy_vehicle_pct: Option<f64>,
    /// Number of lanes serving this movement.
    pub lanes: u32,
    /// Whether the movement operates in a lane shared with another movement.
    pub shared_lane: Option<bool>,
}

impl Movement {
    /// Convenience constructor with the NEMA number auto-assigned from
    /// HCM Exhibit 19-1 for the given approach direction.
    pub fn new(direction: Direction, turn_type: TurnType, volume: f64, lanes: u32) -> Self {
        Self {
            movement_no: nema_movement_number(direction, turn_type),
            turn_type,
            volume,
            phf: None,
            heavy_vehicle_pct: None,
            lanes,
            shared_lane: None,
        }
    }

    /// Demand flow rate v = V / PHF, veh/h (peak 15-min flow rate expressed
    /// as an hourly rate; standard HCM PHF conversion, e.g., HCM Chapter 19
    /// Step 2 and Chapter 20 Step 3).
    ///
    /// If `phf` is `None` (or not positive), the volume is assumed to
    /// already be a flow rate and is returned unchanged.
    pub fn demand_flow_rate(&self) -> f64 {
        match self.phf {
            Some(phf) if phf > 0.0 => self.volume / phf,
            _ => self.volume,
        }
    }
}

/// One intersection approach (all movements traveling in one direction).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Approach {
    /// Direction of travel of the approach.
    pub direction: Direction,
    /// Movements served on this approach.
    pub movements: Vec<Movement>,
    /// Approach grade (%), positive upgrade. `None` if unknown/level.
    pub grade_pct: Option<f64>,
}

impl Approach {
    /// Total approach demand volume, veh/h.
    pub fn total_volume(&self) -> f64 {
        self.movements.iter().map(|m| m.volume).sum()
    }

    /// Total approach demand flow rate Σ(V/PHF), veh/h.
    pub fn total_flow_rate(&self) -> f64 {
        self.movements.iter().map(|m| m.demand_flow_rate()).sum()
    }
}

/// Intersection description shared by the Chapter 19–23 methodologies.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Intersection {
    /// Intersection approaches (typically 3 or 4).
    pub approaches: Vec<Approach>,
    /// Traffic control type.
    pub control: ControlType,
}

impl Intersection {
    /// Total intersection demand volume, veh/h.
    pub fn total_volume(&self) -> f64 {
        self.approaches.iter().map(|a| a.total_volume()).sum()
    }

    /// Total intersection demand flow rate Σ(V/PHF), veh/h.
    pub fn total_flow_rate(&self) -> f64 {
        self.approaches.iter().map(|a| a.total_flow_rate()).sum()
    }

    /// Find an approach by direction.
    pub fn approach(&self, direction: Direction) -> Option<&Approach> {
        self.approaches.iter().find(|a| a.direction == direction)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    /// HCM Exhibit 19-1 numbering.
    #[test]
    fn test_nema_numbering_exhibit_19_1() {
        assert_eq!(nema_movement_number(Direction::EB, TurnType::Left), Some(5));
        assert_eq!(nema_movement_number(Direction::EB, TurnType::Through), Some(2));
        assert_eq!(nema_movement_number(Direction::EB, TurnType::Right), Some(12));
        assert_eq!(nema_movement_number(Direction::WB, TurnType::Left), Some(1));
        assert_eq!(nema_movement_number(Direction::WB, TurnType::Through), Some(6));
        assert_eq!(nema_movement_number(Direction::WB, TurnType::Right), Some(16));
        assert_eq!(nema_movement_number(Direction::NB, TurnType::Left), Some(3));
        assert_eq!(nema_movement_number(Direction::NB, TurnType::Through), Some(8));
        assert_eq!(nema_movement_number(Direction::NB, TurnType::Right), Some(18));
        assert_eq!(nema_movement_number(Direction::SB, TurnType::Left), Some(7));
        assert_eq!(nema_movement_number(Direction::SB, TurnType::Through), Some(4));
        assert_eq!(nema_movement_number(Direction::SB, TurnType::Right), Some(14));
        assert_eq!(nema_movement_number(Direction::EB, TurnType::UTurn), None);
    }

    /// `nema_movement_number` returns `None` for `TurnType::UTurn`
    /// regardless of direction, since Exhibit 19-1 has no NEMA phase slot
    /// for U-turns at all (not just for one approach).
    #[test]
    fn test_nema_movement_number_uturn_always_none() {
        for dir in [Direction::NB, Direction::SB, Direction::EB, Direction::WB] {
            assert_eq!(nema_movement_number(dir, TurnType::UTurn), None);
        }
    }

    /// HCM Chapter 20 Exhibit 20-1: major-street U-turns get adjunct labels
    /// 1U (EB) / 4U (WB); minor-street U-turns (NB/SB) are unlabeled.
    #[test]
    fn test_ch20_uturn_label() {
        assert_eq!(ch20_uturn_label(Direction::EB), Some("1U"));
        assert_eq!(ch20_uturn_label(Direction::WB), Some("4U"));
        assert_eq!(ch20_uturn_label(Direction::NB), None);
        assert_eq!(ch20_uturn_label(Direction::SB), None);
    }

    #[test]
    fn test_right_turn_is_through_plus_ten() {
        for dir in [Direction::NB, Direction::SB, Direction::EB, Direction::WB] {
            let through = nema_movement_number(dir, TurnType::Through).unwrap();
            let right = nema_movement_number(dir, TurnType::Right).unwrap();
            assert_eq!(right, through + 10);
        }
    }

    #[test]
    fn test_demand_flow_rate() {
        let mut m = Movement::new(Direction::EB, TurnType::Through, 900.0, 2);
        m.phf = Some(0.9);
        assert!((m.demand_flow_rate() - 1000.0).abs() < 1e-9);
        // No PHF: volume treated as flow rate
        m.phf = None;
        assert!((m.demand_flow_rate() - 900.0).abs() < 1e-9);
    }

    #[test]
    fn test_totals() {
        let eb = Approach {
            direction: Direction::EB,
            movements: vec![
                Movement {
                    movement_no: Some(5),
                    turn_type: TurnType::Left,
                    volume: 100.0,
                    phf: Some(0.5),
                    heavy_vehicle_pct: None,
                    lanes: 1,
                    shared_lane: Some(false),
                },
                Movement {
                    movement_no: Some(2),
                    turn_type: TurnType::Through,
                    volume: 400.0,
                    phf: Some(0.8),
                    heavy_vehicle_pct: Some(2.0),
                    lanes: 2,
                    shared_lane: Some(false),
                },
            ],
            grade_pct: Some(0.0),
        };
        let wb = Approach {
            direction: Direction::WB,
            movements: vec![Movement::new(Direction::WB, TurnType::Through, 500.0, 2)],
            grade_pct: None,
        };
        let ix = Intersection {
            approaches: vec![eb, wb],
            control: ControlType::PretimedSignal,
        };
        assert!((ix.total_volume() - 1000.0).abs() < 1e-9);
        // 100/0.5 + 400/0.8 + 500 = 200 + 500 + 500
        assert!((ix.total_flow_rate() - 1200.0).abs() < 1e-9);
        assert!(ix.approach(Direction::WB).is_some());
        assert!(ix.approach(Direction::NB).is_none());
    }

    #[test]
    fn test_serde_roundtrip() {
        let ix = Intersection {
            approaches: vec![Approach {
                direction: Direction::NB,
                movements: vec![Movement::new(Direction::NB, TurnType::Left, 50.0, 1)],
                grade_pct: Some(2.0),
            }],
            control: ControlType::TwoWayStop,
        };
        let json = serde_json::to_string(&ix).unwrap();
        let back: Intersection = serde_json::from_str(&json).unwrap();
        assert_eq!(back.approaches[0].movements[0].movement_no, Some(3));
        assert_eq!(back.control, ControlType::TwoWayStop);
    }
}
