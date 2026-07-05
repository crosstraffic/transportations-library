//! Traffic Flow Fundamentals for Agent Reasoning
//!
//! This module implements the fundamental traffic flow equations from HCM Chapter 2,
//! providing the core relationships between volume, speed, and density that enable
//! AI agent reasoning about traffic conditions.
//!
//! # Key Equations
//! - **Fundamental Equation**: D = V / S (Density = Volume / Speed)
//! - **V/C Ratio**: v/c = V / C (Volume-to-Capacity Ratio)
//!
//! # Level of Service (LOS)
//! LOS is determined from density and v/c ratio using HCM thresholds.

use serde::{Deserialize, Serialize};
use crate::hcm::common::LevelOfService;

// ═══════════════════════════════════════════════════════════════════════════════
// Traffic Flow State - Fundamental Equation Implementation
// ═══════════════════════════════════════════════════════════════════════════════

/// Traffic flow state representing the fundamental traffic stream parameters
/// Implements the fundamental equation: D = V / S
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrafficFlowState {
    /// Volume or flow rate (veh/hr)
    pub volume: f64,
    /// Speed (mph)
    pub speed: f64,
    /// Density (veh/mi/ln)
    pub density: f64,
    /// Optional: 15-minute flow rate (veh/hr, adjusted by PHF)
    pub flow_rate: Option<f64>,
}

impl TrafficFlowState {
    /// Create a new TrafficFlowState from volume and speed
    /// Automatically calculates density using fundamental equation: D = V / S
    pub fn from_volume_speed(volume: f64, speed: f64) -> Result<Self, String> {
        if speed <= 0.0 {
            return Err("Speed must be positive".to_string());
        }
        if volume < 0.0 {
            return Err("Volume cannot be negative".to_string());
        }

        let density = Self::calculate_density(volume, speed);

        Ok(Self {
            volume,
            speed,
            density,
            flow_rate: None,
        })
    }

    /// Create a new TrafficFlowState from density and speed
    /// Automatically calculates volume using: V = D * S
    pub fn from_density_speed(density: f64, speed: f64) -> Result<Self, String> {
        if speed <= 0.0 {
            return Err("Speed must be positive".to_string());
        }
        if density < 0.0 {
            return Err("Density cannot be negative".to_string());
        }

        let volume = Self::calculate_volume(density, speed);

        Ok(Self {
            volume,
            speed,
            density,
            flow_rate: None,
        })
    }

    /// Create a new TrafficFlowState with all parameters and validate consistency
    pub fn new(volume: f64, speed: f64, density: f64, flow_rate: Option<f64>) -> Result<Self, String> {
        if speed <= 0.0 {
            return Err("Speed must be positive".to_string());
        }
        if volume < 0.0 {
            return Err("Volume cannot be negative".to_string());
        }
        if density < 0.0 {
            return Err("Density cannot be negative".to_string());
        }

        // Validate fundamental equation: D = V / S
        let expected_density = Self::calculate_density(volume, speed);
        if (density - expected_density).abs() > 0.1 {
            return Err(format!(
                "Inconsistent parameters: density {} doesn't match V/S = {:.2}",
                density, expected_density
            ));
        }

        Ok(Self {
            volume,
            speed,
            density,
            flow_rate,
        })
    }

    /// Calculate density from volume and speed
    /// Fundamental equation: D = V / S
    pub fn calculate_density(volume: f64, speed: f64) -> f64 {
        if speed <= 0.0 {
            return 0.0;
        }
        volume / speed
    }

    /// Calculate volume from density and speed
    /// Derived from fundamental equation: V = D * S
    pub fn calculate_volume(density: f64, speed: f64) -> f64 {
        density * speed
    }

    /// Calculate speed from volume and density
    /// Derived from fundamental equation: S = V / D
    pub fn calculate_speed(volume: f64, density: f64) -> f64 {
        if density <= 0.0 {
            return 0.0;
        }
        volume / density
    }

    /// Set the 15-minute flow rate (adjusted by PHF)
    pub fn with_flow_rate(mut self, flow_rate: f64) -> Self {
        self.flow_rate = Some(flow_rate);
        self
    }

    /// Calculate flow rate from volume and peak hour factor
    /// flow_rate = volume / PHF
    pub fn calculate_flow_rate(&self, phf: f64) -> f64 {
        if phf <= 0.0 || phf > 1.0 {
            return self.volume;
        }
        self.volume / phf
    }

    /// Check if the traffic state is at free-flow conditions (low density)
    pub fn is_free_flow(&self, threshold_density: f64) -> bool {
        self.density <= threshold_density
    }

    /// Check if the traffic state is congested (high density)
    pub fn is_congested(&self, threshold_density: f64) -> bool {
        self.density > threshold_density
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Volume-to-Capacity Ratio
// ═══════════════════════════════════════════════════════════════════════════════

/// Volume-to-Capacity ratio for demand analysis
/// v/c > 1.0 indicates oversaturated conditions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeCapacityRatio {
    /// Demand volume (veh/hr)
    pub volume: f64,
    /// Capacity (veh/hr or pc/hr/ln)
    pub capacity: f64,
    /// Calculated v/c ratio
    pub vc_ratio: f64,
}

impl VolumeCapacityRatio {
    /// Create a new VolumeCapacityRatio
    pub fn new(volume: f64, capacity: f64) -> Result<Self, String> {
        if capacity <= 0.0 {
            return Err("Capacity must be positive".to_string());
        }
        if volume < 0.0 {
            return Err("Volume cannot be negative".to_string());
        }

        let vc_ratio = volume / capacity;

        Ok(Self {
            volume,
            capacity,
            vc_ratio,
        })
    }

    /// Create with explicit v/c ratio and validate consistency
    pub fn with_ratio(volume: f64, capacity: f64, vc_ratio: f64) -> Result<Self, String> {
        if capacity <= 0.0 {
            return Err("Capacity must be positive".to_string());
        }
        if volume < 0.0 {
            return Err("Volume cannot be negative".to_string());
        }

        // Validate v/c calculation
        let expected_vc = volume / capacity;
        if (vc_ratio - expected_vc).abs() > 0.01 {
            return Err(format!(
                "Inconsistent v/c ratio: {} doesn't match V/C = {:.3}",
                vc_ratio, expected_vc
            ));
        }

        Ok(Self {
            volume,
            capacity,
            vc_ratio,
        })
    }

    /// Check if conditions are oversaturated (v/c > 1.0)
    pub fn is_oversaturated(&self) -> bool {
        self.vc_ratio > 1.0
    }

    /// Check if conditions are near capacity (v/c > 0.85)
    pub fn is_near_capacity(&self) -> bool {
        self.vc_ratio > 0.85
    }

    /// Get the degree of saturation as percentage
    pub fn saturation_percent(&self) -> f64 {
        self.vc_ratio * 100.0
    }

    /// Calculate the reserve capacity (remaining capacity)
    pub fn reserve_capacity(&self) -> f64 {
        if self.vc_ratio >= 1.0 {
            0.0
        } else {
            self.capacity - self.volume
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Level of Service Thresholds (HCM Tables)
// ═══════════════════════════════════════════════════════════════════════════════

/// LOS density thresholds for different facility types
/// Based on HCM Exhibit 12-15 (Basic Freeway Segments) and Exhibit 15-6 (Two-Lane Highways)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LOSThresholds {
    /// Maximum density for LOS A (veh/mi/ln)
    pub los_a: f64,
    /// Maximum density for LOS B (veh/mi/ln)
    pub los_b: f64,
    /// Maximum density for LOS C (veh/mi/ln)
    pub los_c: f64,
    /// Maximum density for LOS D (veh/mi/ln)
    pub los_d: f64,
    /// Maximum density for LOS E (veh/mi/ln)
    pub los_e: f64,
    // Note: LOS F is above los_e or when v/c > 1.0
}

impl LOSThresholds {
    /// Basic Freeway Segments LOS thresholds (HCM Exhibit 12-15)
    /// Thresholds are for density in pc/mi/ln
    pub fn basic_freeway() -> Self {
        Self {
            los_a: 11.0,
            los_b: 18.0,
            los_c: 26.0,
            los_d: 35.0,
            los_e: 45.0,
        }
    }

    /// Two-Lane Highway LOS thresholds for high-speed facilities (HCM Exhibit 15-6)
    /// Based on follower density (followers/mi/ln)
    pub fn two_lane_high_speed() -> Self {
        Self {
            los_a: 2.0,
            los_b: 4.0,
            los_c: 8.0,
            los_d: 12.0,
            los_e: 18.0,
        }
    }

    /// Multilane Highway LOS thresholds (HCM Exhibit 14-5)
    pub fn multilane_highway() -> Self {
        Self {
            los_a: 11.0,
            los_b: 18.0,
            los_c: 26.0,
            los_d: 35.0,
            los_e: 40.0,
        }
    }

    /// Urban Street LOS thresholds (speed-based, converted to density equivalent)
    pub fn urban_street() -> Self {
        // Note: Urban streets typically use speed-based LOS
        // These are approximate density equivalents
        Self {
            los_a: 6.0,
            los_b: 14.0,
            los_c: 22.0,
            los_d: 29.0,
            los_e: 39.0,
        }
    }

    /// Determine LOS from density and optional v/c ratio
    /// Returns LOS F if v/c > 1.0 regardless of density
    pub fn get_los(&self, density: f64, vc_ratio: Option<f64>) -> LevelOfService {
        // Check for oversaturated conditions first
        if let Some(vc) = vc_ratio {
            if vc > 1.0 {
                return LevelOfService::F;
            }
        }

        // Determine LOS from density
        if density <= self.los_a {
            LevelOfService::A
        } else if density <= self.los_b {
            LevelOfService::B
        } else if density <= self.los_c {
            LevelOfService::C
        } else if density <= self.los_d {
            LevelOfService::D
        } else if density <= self.los_e {
            LevelOfService::E
        } else {
            LevelOfService::F
        }
    }

    /// Get the density threshold for a specific LOS
    pub fn threshold_for_los(&self, los: LevelOfService) -> f64 {
        match los {
            LevelOfService::A => self.los_a,
            LevelOfService::B => self.los_b,
            LevelOfService::C => self.los_c,
            LevelOfService::D => self.los_d,
            LevelOfService::E => self.los_e,
            LevelOfService::F => f64::INFINITY,
        }
    }

    /// Calculate maximum volume for a given LOS at a specific speed
    /// Uses V = D * S
    pub fn max_volume_for_los(&self, los: LevelOfService, speed: f64) -> f64 {
        let max_density = self.threshold_for_los(los);
        if max_density.is_infinite() {
            f64::INFINITY
        } else {
            max_density * speed
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Capacity Values (HCM Tables)
// ═══════════════════════════════════════════════════════════════════════════════

/// Capacity values by facility type and conditions
/// Based on HCM exhibits for different facility types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapacityValues {
    /// Base capacity (pc/hr/ln)
    pub base_capacity: f64,
    /// Free-flow speed used for this capacity (mph)
    pub ffs: f64,
    /// Capacity at LOS E (operational capacity)
    pub operational_capacity: f64,
}

impl CapacityValues {
    /// Get freeway capacity by free-flow speed (HCM Exhibit 12-6)
    /// Returns capacity in pc/hr/ln
    pub fn freeway_by_ffs(ffs: f64) -> Self {
        let (base_capacity, operational_capacity) = if ffs >= 75.0 {
            (2400.0, 2400.0)
        } else if ffs >= 70.0 {
            (2400.0, 2400.0)
        } else if ffs >= 65.0 {
            (2350.0, 2350.0)
        } else if ffs >= 60.0 {
            (2300.0, 2300.0)
        } else if ffs >= 55.0 {
            (2250.0, 2250.0)
        } else {
            // Below 55 mph, capacity decreases further
            let capacity = 2250.0 - (55.0 - ffs) * 10.0;
            (capacity.max(1800.0), capacity.max(1800.0))
        };

        Self {
            base_capacity,
            ffs,
            operational_capacity,
        }
    }

    /// Get two-lane highway capacity (HCM Chapter 15)
    /// Returns capacity in veh/hr for both directions combined
    ///
    /// # Arguments
    /// * `passing_type` - 0: Passing Constrained, 1: Passing Zone, 2: Passing Lane
    /// * `vertical_class` - 1-5 (terrain difficulty)
    /// * `hv_percent` - Heavy vehicle percentage (0-100)
    pub fn two_lane_highway(passing_type: i32, vertical_class: i32, hv_percent: f64) -> f64 {
        // Base two-way capacity: 3200 pc/hr (HCM)
        let base_capacity = 3200.0;

        // Adjustment for passing type
        let passing_factor = match passing_type {
            0 => 0.70, // Constrained
            1 => 0.85, // Zone
            2 => 1.00, // Lane
            _ => 0.85, // Default to zone
        };

        // Adjustment for vertical class (terrain)
        let terrain_factor = match vertical_class {
            1 => 1.00, // Level
            2 => 0.93, // Rolling
            3 => 0.85, // Mountainous low
            4 => 0.75, // Mountainous medium
            5 => 0.65, // Mountainous severe
            _ => 0.85, // Default to rolling
        };

        // Heavy vehicle adjustment factor (simplified)
        // E_T = 1 + 0.02 * HV% for level terrain
        let e_t = 1.0 + 0.02 * (vertical_class as f64);
        let hv_factor = 1.0 / (1.0 + (hv_percent / 100.0) * (e_t - 1.0));

        base_capacity * passing_factor * terrain_factor * hv_factor
    }

    /// Get multilane highway capacity by FFS (HCM Exhibit 14-4)
    pub fn multilane_highway_by_ffs(ffs: f64) -> Self {
        let base_capacity = if ffs >= 60.0 {
            2200.0
        } else if ffs >= 55.0 {
            2100.0
        } else if ffs >= 50.0 {
            2000.0
        } else if ffs >= 45.0 {
            1900.0
        } else {
            1800.0
        };

        Self {
            base_capacity,
            ffs,
            operational_capacity: base_capacity,
        }
    }

    /// Apply heavy vehicle adjustment to capacity
    /// Returns adjusted capacity in veh/hr/ln
    pub fn adjust_for_heavy_vehicles(&self, hv_percent: f64, terrain_factor: f64) -> f64 {
        let e_t = 1.0 + terrain_factor;
        let f_hv = 1.0 / (1.0 + (hv_percent / 100.0) * (e_t - 1.0));
        self.base_capacity * f_hv
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Peak Hour Factor
// ═══════════════════════════════════════════════════════════════════════════════

/// Peak hour factor calculations and validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeakHourFactor {
    /// Peak hour factor value (0.70-1.00)
    pub phf: f64,
}

impl PeakHourFactor {
    /// Create a new PHF with validation
    pub fn new(phf: f64) -> Result<Self, String> {
        if phf < 0.70 || phf > 1.0 {
            return Err(format!("PHF {} outside valid range (0.70-1.00)", phf));
        }
        Ok(Self { phf })
    }

    /// Calculate PHF from hourly volume and peak 15-minute volume
    /// PHF = V / (4 * V15)
    pub fn from_volumes(hourly_volume: f64, peak_15min_volume: f64) -> Result<Self, String> {
        if peak_15min_volume <= 0.0 {
            return Err("Peak 15-minute volume must be positive".to_string());
        }
        let phf = hourly_volume / (4.0 * peak_15min_volume);
        Self::new(phf)
    }

    /// Convert hourly volume to peak 15-minute flow rate
    pub fn to_flow_rate(&self, hourly_volume: f64) -> f64 {
        hourly_volume / self.phf
    }

    /// Check if PHF indicates peaky traffic (PHF < 0.85)
    pub fn is_peaky(&self) -> bool {
        self.phf < 0.85
    }

    /// Check if PHF indicates uniform traffic (PHF > 0.95)
    pub fn is_uniform(&self) -> bool {
        self.phf > 0.95
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Speed-Flow Relationship
// ═══════════════════════════════════════════════════════════════════════════════

/// Speed-flow relationship for capacity analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeedFlowRelationship {
    /// Free-flow speed (mph)
    pub ffs: f64,
    /// Breakpoint density (veh/mi/ln) where speed starts to decline
    pub breakpoint_density: f64,
    /// Capacity (pc/hr/ln)
    pub capacity: f64,
    /// Jam density (veh/mi/ln)
    pub jam_density: f64,
}

impl SpeedFlowRelationship {
    /// Create a basic freeway speed-flow relationship
    pub fn basic_freeway(ffs: f64) -> Self {
        Self {
            ffs,
            breakpoint_density: 25.0, // Typical breakpoint
            capacity: CapacityValues::freeway_by_ffs(ffs).base_capacity,
            jam_density: 190.0, // Typical jam density
        }
    }

    /// Calculate speed at a given density
    /// Uses linear speed-density relationship below breakpoint
    /// and curved relationship above breakpoint
    pub fn speed_at_density(&self, density: f64) -> f64 {
        if density <= 0.0 {
            return self.ffs;
        }
        if density >= self.jam_density {
            return 0.0;
        }

        if density <= self.breakpoint_density {
            // Free-flow regime
            self.ffs
        } else {
            // Congested regime - linear decrease
            let slope = -self.ffs / (self.jam_density - self.breakpoint_density);
            self.ffs + slope * (density - self.breakpoint_density)
        }
    }

    /// Calculate flow at a given density
    pub fn flow_at_density(&self, density: f64) -> f64 {
        let speed = self.speed_at_density(density);
        density * speed
    }

    /// Find density that maximizes flow (capacity density)
    pub fn capacity_density(&self) -> f64 {
        // For typical BPR-style curves, capacity occurs around 45 veh/mi/ln
        // This is a simplified estimate
        self.jam_density * 0.24
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fundamental_equation() {
        // D = V / S
        let volume = 1800.0; // veh/hr
        let speed = 60.0; // mph
        let density = TrafficFlowState::calculate_density(volume, speed);
        assert!((density - 30.0).abs() < 0.001);
    }

    #[test]
    fn test_traffic_flow_state_creation() {
        let state = TrafficFlowState::from_volume_speed(1800.0, 60.0);
        assert!(state.is_ok());
        let state = state.unwrap();
        assert!((state.density - 30.0).abs() < 0.001);
    }

    #[test]
    fn test_traffic_flow_state_validation() {
        // Valid: D = V/S = 1800/60 = 30
        let state = TrafficFlowState::new(1800.0, 60.0, 30.0, None);
        assert!(state.is_ok());

        // Invalid: D should be 30, not 25
        let state = TrafficFlowState::new(1800.0, 60.0, 25.0, None);
        assert!(state.is_err());
    }

    #[test]
    fn test_volume_capacity_ratio() {
        let vcr = VolumeCapacityRatio::new(1800.0, 2400.0);
        assert!(vcr.is_ok());
        let vcr = vcr.unwrap();
        assert!((vcr.vc_ratio - 0.75).abs() < 0.001);
        assert!(!vcr.is_oversaturated());
    }

    #[test]
    fn test_oversaturated() {
        let vcr = VolumeCapacityRatio::new(2600.0, 2400.0).unwrap();
        assert!(vcr.is_oversaturated());
    }

    #[test]
    fn test_los_basic_freeway() {
        let thresholds = LOSThresholds::basic_freeway();

        // LOS A: density <= 11
        assert_eq!(thresholds.get_los(10.0, None), LevelOfService::A);

        // LOS B: 11 < density <= 18
        assert_eq!(thresholds.get_los(15.0, None), LevelOfService::B);

        // LOS C: 18 < density <= 26
        assert_eq!(thresholds.get_los(22.0, None), LevelOfService::C);

        // LOS D: 26 < density <= 35
        assert_eq!(thresholds.get_los(30.0, None), LevelOfService::D);

        // LOS E: 35 < density <= 45
        assert_eq!(thresholds.get_los(40.0, None), LevelOfService::E);

        // LOS F: density > 45
        assert_eq!(thresholds.get_los(50.0, None), LevelOfService::F);
    }

    #[test]
    fn test_los_oversaturated() {
        let thresholds = LOSThresholds::basic_freeway();
        // Even with low density, v/c > 1.0 means LOS F
        assert_eq!(thresholds.get_los(10.0, Some(1.1)), LevelOfService::F);
    }

    #[test]
    fn test_freeway_capacity_by_ffs() {
        let cap = CapacityValues::freeway_by_ffs(75.0);
        assert_eq!(cap.base_capacity, 2400.0);

        let cap = CapacityValues::freeway_by_ffs(60.0);
        assert_eq!(cap.base_capacity, 2300.0);

        let cap = CapacityValues::freeway_by_ffs(55.0);
        assert_eq!(cap.base_capacity, 2250.0);
    }

    #[test]
    fn test_two_lane_capacity() {
        // Base capacity with passing lane, level terrain, no HV
        let cap = CapacityValues::two_lane_highway(2, 1, 0.0);
        assert!((cap - 3200.0).abs() < 0.01);

        // Constrained passing, rolling terrain
        let cap = CapacityValues::two_lane_highway(0, 2, 10.0);
        assert!(cap < 3200.0);
    }

    #[test]
    fn test_phf_validation() {
        let phf = PeakHourFactor::new(0.92);
        assert!(phf.is_ok());

        let phf = PeakHourFactor::new(0.50);
        assert!(phf.is_err());

        let phf = PeakHourFactor::new(1.1);
        assert!(phf.is_err());
    }

    #[test]
    fn test_phf_flow_rate() {
        let phf = PeakHourFactor::new(0.90).unwrap();
        let flow_rate = phf.to_flow_rate(1800.0);
        assert!((flow_rate - 2000.0).abs() < 0.001);
    }

    #[test]
    fn test_speed_flow_relationship() {
        let sfr = SpeedFlowRelationship::basic_freeway(70.0);

        // At low density, should be at FFS
        let speed = sfr.speed_at_density(10.0);
        assert!((speed - 70.0).abs() < 0.001);

        // At jam density, speed should be near zero
        let speed = sfr.speed_at_density(190.0);
        assert!(speed < 1.0);
    }

    #[test]
    fn test_max_volume_for_los() {
        let thresholds = LOSThresholds::basic_freeway();
        let max_vol_a = thresholds.max_volume_for_los(LevelOfService::A, 70.0);
        // V = D * S = 11 * 70 = 770
        assert!((max_vol_a - 770.0).abs() < 0.001);
    }
}
