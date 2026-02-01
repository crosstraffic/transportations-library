//! Basic Managed Lane Segment Analysis
//!
//! This module implements the HCM Chapter 12 Section 4 methodology for analyzing
//! basic managed lane segments on freeways, including HOV lanes, HOT lanes, and
//! express toll lanes.
//!
//! Equations implemented:
//! - Equation 12-12: Managed lane speed-flow relationship
//! - Equation 12-13: Breakpoint calculation
//! - Equation 12-14: Capacity calculation
//! - Equation 12-15: Linear speed portion (S1)
//! - Equation 12-16: Curvilinear calibration factor (A2)
//! - Equation 12-17: Speed drop in curvilinear portion (S2)
//! - Equation 12-18: Friction indicator (Ic)
//! - Equation 12-19: Additional speed drop due to GP friction (S3)

use serde::{Deserialize, Serialize};
use super::common::LevelOfService;

/// Managed lane segment types from Exhibit 12-9
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ManagedLaneType {
    /// Skip-stripe or solid single line-separated, single lane
    ContinuousAccess,
    /// Buffer-separated, single lane
    Buffer1,
    /// Buffer-separated, multiple lanes
    Buffer2,
    /// Barrier-separated, single lane
    Barrier1,
    /// Barrier-separated, multiple lanes
    Barrier2,
}

/// Parameters for managed lane segment analysis from Exhibit 12-30
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManagedLaneParams {
    /// Breakpoint for FFS of 75 mi/h (pc/h/ln)
    pub bp_75: f64,
    /// Rate of increase in breakpoint per unit decrease in FFS
    pub lambda_bp: f64,
    /// Capacity for FFS of 75 mi/h (pc/h/ln)
    pub c_75: f64,
    /// Rate of change in capacity per unit change in FFS
    pub lambda_c: f64,
    /// Speed reduction per unit of flow in curvilinear section for FFS 55 mi/h
    pub a2_55: f64,
    /// Rate of change in A2 per unit increase in FFS
    pub lambda_a2: f64,
    /// Speed reduction per unit of flow in linear section (A1)
    pub a1: f64,
    /// Density at capacity without friction effect (pc/mi/ln)
    pub k_cnf: f64,
    /// Density at capacity with friction effect (pc/mi/ln) - only for ContinuousAccess and Buffer1
    pub k_cf: Option<f64>,
}

impl ManagedLaneParams {
    /// Get parameters for a specific managed lane type
    /// Values from Exhibit 12-30
    pub fn for_type(lane_type: ManagedLaneType) -> Self {
        match lane_type {
            ManagedLaneType::ContinuousAccess => Self {
                bp_75: 500.0,
                lambda_bp: 0.0,
                c_75: 1800.0,
                lambda_c: 10.0,
                a2_55: 2.5,
                lambda_a2: 0.0,
                a1: 0.0,
                k_cnf: 30.0,
                k_cf: Some(45.0),
            },
            ManagedLaneType::Buffer1 => Self {
                bp_75: 600.0,
                lambda_bp: 0.0,
                c_75: 1700.0,
                lambda_c: 10.0,
                a2_55: 1.4,
                lambda_a2: 0.0,
                a1: 0.0033,
                k_cnf: 30.0,
                k_cf: Some(42.0),  // Average value per note in Exhibit 12-30
            },
            ManagedLaneType::Buffer2 => Self {
                bp_75: 500.0,
                lambda_bp: 10.0,
                c_75: 1850.0,
                lambda_c: 10.0,
                a2_55: 1.5,
                lambda_a2: 0.02,
                a1: 0.0,
                k_cnf: 45.0,  // Average value per note in Exhibit 12-30
                k_cf: None,  // No friction effect for Buffer2
            },
            ManagedLaneType::Barrier1 => Self {
                bp_75: 800.0,
                lambda_bp: 0.0,
                c_75: 1750.0,
                lambda_c: 10.0,
                a2_55: 1.4,
                lambda_a2: 0.0,
                a1: 0.004,
                k_cnf: 35.0,
                k_cf: None,  // No friction effect for barrier-separated
            },
            ManagedLaneType::Barrier2 => Self {
                bp_75: 700.0,
                lambda_bp: 20.0,
                c_75: 2100.0,
                lambda_c: 10.0,
                a2_55: 1.3,
                lambda_a2: 0.02,
                a1: 0.0,
                k_cnf: 45.0,
                k_cf: None,  // No friction effect for barrier-separated
            },
        }
    }
}

/// Basic Managed Lane Segment analysis structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManagedLaneSegment {
    /// Type of managed lane segment
    pub lane_type: ManagedLaneType,
    /// Parameters for this segment type
    pub params: ManagedLaneParams,
    /// Base free-flow speed (mi/h)
    pub ffs: f64,
    /// Adjusted free-flow speed (mi/h)
    pub ffs_adj: f64,
    /// Speed Adjustment Factor
    pub saf: f64,
    /// Capacity Adjustment Factor
    pub caf: f64,
    /// Demand flow rate (pc/h/ln)
    pub v_p: f64,
    /// Density of adjacent general purpose lane (pc/mi/ln)
    pub k_gp: f64,
    /// Breakpoint in speed-flow curve (pc/h/ln)
    pub breakpoint: f64,
    /// Adjusted capacity (pc/h/ln)
    pub capacity_adj: f64,
    /// Space mean speed (mi/h)
    pub speed: f64,
    /// Density (pc/mi/ln)
    pub density: f64,
    /// Level of Service
    pub los: Option<LevelOfService>,
}

impl ManagedLaneSegment {
    /// Create a new managed lane segment
    pub fn new(lane_type: ManagedLaneType, ffs: f64) -> Self {
        let params = ManagedLaneParams::for_type(lane_type);
        Self {
            lane_type,
            params,
            ffs,
            ffs_adj: ffs,
            saf: 1.0,
            caf: 1.0,
            v_p: 0.0,
            k_gp: 0.0,
            breakpoint: 0.0,
            capacity_adj: 0.0,
            speed: ffs,
            density: 0.0,
            los: None,
        }
    }

    /// Calculate adjusted free-flow speed
    pub fn calculate_ffs_adj(&mut self) -> f64 {
        self.ffs_adj = self.ffs * self.saf;
        self.ffs_adj
    }

    /// Calculate breakpoint in speed-flow curve
    /// Equation 12-13: BP = (BP_75 + λ_BP × (75 - FFS_adj)) × CAF
    pub fn calculate_breakpoint(&mut self) -> f64 {
        self.breakpoint = (self.params.bp_75 + self.params.lambda_bp * (75.0 - self.ffs_adj)) * self.caf;
        self.breakpoint
    }

    /// Calculate adjusted capacity
    /// Equation 12-14: c_adj = CAF × (c_75 - λ_c × (75 - FFS_adj))
    pub fn calculate_capacity(&mut self) -> f64 {
        self.capacity_adj = self.caf * (self.params.c_75 - self.params.lambda_c * (75.0 - self.ffs_adj));
        self.capacity_adj
    }

    /// Calculate A2 calibration factor for curvilinear portion
    /// Equation 12-16: A2 = A2_55 + λ_A2 × (FFS_adj - 55)
    fn calculate_a2(&self) -> f64 {
        self.params.a2_55 + self.params.lambda_a2 * (self.ffs_adj - 55.0)
    }

    /// Calculate speed in linear portion of curve (S1)
    /// Equation 12-15: S1 = FFS_adj - A1 × v_p
    fn calculate_s1(&self, v_p: f64) -> f64 {
        self.ffs_adj - self.params.a1 * v_p
    }

    /// Calculate speed at breakpoint
    fn calculate_s1_bp(&self) -> f64 {
        self.calculate_s1(self.breakpoint)
    }

    /// Determine friction indicator
    /// Equation 12-18: Ic = 1 if K_GP > 35 pc/mi/ln, else 0
    /// Only applicable for ContinuousAccess and Buffer1 segment types
    fn calculate_friction_indicator(&self) -> f64 {
        match self.lane_type {
            ManagedLaneType::ContinuousAccess | ManagedLaneType::Buffer1 => {
                if self.k_gp > 35.0 { 1.0 } else { 0.0 }
            }
            _ => 0.0,  // No friction effect for Buffer2, Barrier1, Barrier2
        }
    }

    /// Calculate speed drop in curvilinear portion (S2)
    /// Equation 12-17: S2 = (S1,BP - c_adj/K_cnf) × ((v_p - BP)/(c_adj - BP))^A2
    fn calculate_s2(&self) -> f64 {
        if self.v_p <= self.breakpoint {
            return 0.0;
        }

        let s1_bp = self.calculate_s1_bp();
        let speed_at_capacity = self.capacity_adj / self.params.k_cnf;
        let a2 = self.calculate_a2();

        let numerator = self.v_p - self.breakpoint;
        let denominator = self.capacity_adj - self.breakpoint;

        if denominator > 0.0 {
            (s1_bp - speed_at_capacity) * (numerator / denominator).powf(a2)
        } else {
            0.0
        }
    }

    /// Calculate additional speed drop due to GP lane friction (S3)
    /// Equation 12-19: S3 = (S1,BP - c_adj/K_cf) × ((v_p - BP)/(c_adj - BP))^A2 - S2
    fn calculate_s3(&self) -> f64 {
        if self.v_p <= self.breakpoint {
            return 0.0;
        }

        let ic = self.calculate_friction_indicator();
        if ic == 0.0 {
            return 0.0;
        }

        // Only calculate if there's a friction effect (K_cf is defined)
        if let Some(k_cf) = self.params.k_cf {
            let s1_bp = self.calculate_s1_bp();
            let speed_at_capacity_friction = self.capacity_adj / k_cf;
            let a2 = self.calculate_a2();

            let numerator = self.v_p - self.breakpoint;
            let denominator = self.capacity_adj - self.breakpoint;

            if denominator > 0.0 {
                let s3_full = (s1_bp - speed_at_capacity_friction) * (numerator / denominator).powf(a2);
                let s2 = self.calculate_s2();
                (s3_full - s2).max(0.0)  // S3 should be positive
            } else {
                0.0
            }
        } else {
            0.0
        }
    }

    /// Calculate space mean speed of managed lane segment
    /// Equation 12-12: S_ML = S1 - S2 - Ic × S3
    pub fn calculate_speed(&mut self) -> f64 {
        // Ensure breakpoint and capacity are calculated
        if self.breakpoint <= 0.0 {
            self.calculate_breakpoint();
        }
        if self.capacity_adj <= 0.0 {
            self.calculate_capacity();
        }

        let ic = self.calculate_friction_indicator();

        if self.v_p <= self.breakpoint {
            // Linear portion: S = S1
            self.speed = self.calculate_s1(self.v_p);
        } else if self.v_p <= self.capacity_adj {
            // Curvilinear portion: S = S1 - S2 - Ic × S3
            let s1 = self.calculate_s1(self.v_p);
            let s2 = self.calculate_s2();
            let s3 = self.calculate_s3();
            self.speed = s1 - s2 - ic * s3;
        } else {
            // Demand exceeds capacity
            self.speed = 0.0;
        }

        self.speed
    }

    /// Calculate density
    pub fn calculate_density(&mut self) -> f64 {
        if self.speed <= 0.0 {
            self.calculate_speed();
        }

        if self.speed > 0.0 {
            self.density = self.v_p / self.speed;
        } else {
            self.density = 50.0;  // Indicates oversaturation
        }

        self.density
    }

    /// Determine Level of Service
    /// Uses same criteria as basic freeway segments (Exhibit 12-15)
    pub fn determine_los(&mut self) -> LevelOfService {
        // Check for demand exceeding capacity
        if self.v_p > self.capacity_adj {
            self.los = Some(LevelOfService::F);
            return LevelOfService::F;
        }

        if self.density <= 0.0 {
            self.calculate_density();
        }

        let los = match self.density {
            d if d <= 11.0 => LevelOfService::A,
            d if d <= 18.0 => LevelOfService::B,
            d if d <= 26.0 => LevelOfService::C,
            d if d <= 35.0 => LevelOfService::D,
            d if d <= 45.0 => LevelOfService::E,
            _ => LevelOfService::F,
        };

        self.los = Some(los);
        los
    }

    /// Run complete operational analysis
    pub fn run_analysis(&mut self) -> LevelOfService {
        self.calculate_ffs_adj();
        self.calculate_breakpoint();
        self.calculate_capacity();
        self.calculate_speed();
        self.calculate_density();
        self.determine_los()
    }

    /// Set demand flow rate
    pub fn set_demand(&mut self, v_p: f64) {
        self.v_p = v_p;
    }

    /// Set general purpose lane density for friction analysis
    pub fn set_gp_density(&mut self, k_gp: f64) {
        self.k_gp = k_gp;
    }

    /// Set Speed Adjustment Factor
    pub fn set_saf(&mut self, saf: f64) {
        self.saf = saf;
    }

    /// Set Capacity Adjustment Factor
    pub fn set_caf(&mut self, caf: f64) {
        self.caf = caf;
    }

    /// Check if friction effect applies to this segment type
    pub fn has_friction_effect(&self) -> bool {
        matches!(self.lane_type, ManagedLaneType::ContinuousAccess | ManagedLaneType::Buffer1)
    }

    /// Check if friction is currently active (GP density > 35)
    pub fn is_friction_active(&self) -> bool {
        self.has_friction_effect() && self.k_gp > 35.0
    }
}

/// Estimated lane capacities for managed lane segments from Exhibit 12-11
/// Returns capacity in pc/h/ln based on FFS and segment type
pub fn get_estimated_capacity(lane_type: ManagedLaneType, ffs: u32) -> Option<u32> {
    match (lane_type, ffs) {
        // Continuous Access
        (ManagedLaneType::ContinuousAccess, 75) => Some(1800),
        (ManagedLaneType::ContinuousAccess, 70) => Some(1750),
        (ManagedLaneType::ContinuousAccess, 65) => Some(1700),
        (ManagedLaneType::ContinuousAccess, 60) => Some(1650),
        (ManagedLaneType::ContinuousAccess, 55) => Some(1600),

        // Buffer 1
        (ManagedLaneType::Buffer1, 75) => Some(1700),
        (ManagedLaneType::Buffer1, 70) => Some(1650),
        (ManagedLaneType::Buffer1, 65) => Some(1600),
        (ManagedLaneType::Buffer1, 60) => Some(1550),
        (ManagedLaneType::Buffer1, 55) => Some(1500),

        // Buffer 2
        (ManagedLaneType::Buffer2, 75) => Some(1850),
        (ManagedLaneType::Buffer2, 70) => Some(1800),
        (ManagedLaneType::Buffer2, 65) => Some(1750),
        (ManagedLaneType::Buffer2, 60) => Some(1700),
        (ManagedLaneType::Buffer2, 55) => Some(1650),

        // Barrier 1
        (ManagedLaneType::Barrier1, 75) => Some(1750),
        (ManagedLaneType::Barrier1, 70) => Some(1700),
        (ManagedLaneType::Barrier1, 65) => Some(1650),
        (ManagedLaneType::Barrier1, 60) => Some(1600),
        (ManagedLaneType::Barrier1, 55) => Some(1550),

        // Barrier 2
        (ManagedLaneType::Barrier2, 75) => Some(2100),
        (ManagedLaneType::Barrier2, 70) => Some(2050),
        (ManagedLaneType::Barrier2, 65) => Some(2000),
        (ManagedLaneType::Barrier2, 60) => Some(1950),
        (ManagedLaneType::Barrier2, 55) => Some(1900),

        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_continuous_access_params() {
        let params = ManagedLaneParams::for_type(ManagedLaneType::ContinuousAccess);
        assert_eq!(params.bp_75, 500.0);
        assert_eq!(params.c_75, 1800.0);
        assert!(params.k_cf.is_some());
    }

    #[test]
    fn test_barrier_no_friction() {
        let segment = ManagedLaneSegment::new(ManagedLaneType::Barrier2, 70.0);
        assert!(!segment.has_friction_effect());
    }

    #[test]
    fn test_continuous_access_friction() {
        let mut segment = ManagedLaneSegment::new(ManagedLaneType::ContinuousAccess, 70.0);
        segment.set_gp_density(40.0);
        assert!(segment.is_friction_active());
    }
}
