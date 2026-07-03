use serde::{Deserialize, Serialize};
use crate::hcm::common::LevelOfService;

/// Chapter 13: Freeway Weaving Segments
///
/// This module provides analysis methodology for freeway weaving segments,
/// including capacity, speed, density, and LOS determination.

// =============================================================================
// Constants
// =============================================================================

/// Minimum speed expected in a weaving segment (mi/h)
pub const MIN_WEAVING_SPEED: f64 = 15.0;

/// Minimum weaving segment length for optional lane changes (ft)
pub const MIN_WEAVING_LENGTH: f64 = 300.0;

/// Maximum weaving flow rate for NWL = 2 lanes (pc/h)
pub const MAX_WEAVING_FLOW_NWL2: f64 = 2400.0;

/// Maximum weaving flow rate for NWL = 3 lanes (pc/h)
pub const MAX_WEAVING_FLOW_NWL3: f64 = 3500.0;

/// Density threshold for capacity (breakdown) in weaving segments (pc/mi/ln)
pub const WEAVING_BREAKDOWN_DENSITY: f64 = 43.0;

/// Density threshold for capacity on multilane highways/C-D roads (pc/mi/ln)
pub const MULTILANE_BREAKDOWN_DENSITY: f64 = 40.0;

// =============================================================================
// Enums and Types
// =============================================================================

/// Type of weaving segment
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WeavingType {
    /// One-sided weaving (ramps on same side)
    OneSided,
    /// Two-sided weaving (ramps on opposite sides)
    TwoSided,
}

/// Configuration type of weaving segment
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WeavingConfiguration {
    /// Ramp weave: one-lane on-ramp followed by one-lane off-ramp with auxiliary lane
    RampWeave,
    /// Major weave: three or more entry/exit legs have multiple lanes
    MajorWeave,
}

/// Facility type for LOS criteria
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FacilityType {
    /// Freeway weaving segment
    Freeway,
    /// Multilane highway or C-D road weaving segment
    MultilaneOrCD,
}

/// LOS criteria for weaving segments - Exhibit 13-6
pub fn determine_weaving_los(density: f64, demand_exceeds_capacity: bool, facility: FacilityType) -> LevelOfService {
    if demand_exceeds_capacity {
        return LevelOfService::F;
    }

    match facility {
        FacilityType::Freeway => {
            match density {
                d if d <= 10.0 => LevelOfService::A,
                d if d <= 20.0 => LevelOfService::B,
                d if d <= 28.0 => LevelOfService::C,
                d if d <= 35.0 => LevelOfService::D,
                d if d <= 43.0 => LevelOfService::E,
                _ => LevelOfService::F,
            }
        },
        FacilityType::MultilaneOrCD => {
            match density {
                d if d <= 12.0 => LevelOfService::A,
                d if d <= 24.0 => LevelOfService::B,
                d if d <= 32.0 => LevelOfService::C,
                d if d <= 36.0 => LevelOfService::D,
                d if d <= 40.0 => LevelOfService::E,
                _ => LevelOfService::F,
            }
        },
    }
}

// =============================================================================
// Input Parameters
// =============================================================================

/// Input parameters for weaving segment analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeavingInput {
    /// Type of weaving segment (one-sided or two-sided)
    pub weaving_type: WeavingType,
    /// Facility type (freeway or multilane/C-D)
    pub facility_type: FacilityType,
    /// Short length of weaving segment (ft)
    pub length_short: f64,
    /// Number of lanes within weaving segment
    pub num_lanes: u32,
    /// Number of weaving lanes (NWL): lanes from which weaving can be done with <= 1 lane change
    /// For one-sided: typically 2 or 3
    /// For two-sided: always 0
    pub num_weaving_lanes: u32,
    /// Free-flow speed of weaving segment (mi/h)
    pub ffs: f64,
    /// Freeway-to-freeway demand volume (veh/h)
    pub v_ff: f64,
    /// Freeway-to-ramp demand volume (veh/h)
    pub v_fr: f64,
    /// Ramp-to-freeway demand volume (veh/h)
    pub v_rf: f64,
    /// Ramp-to-ramp demand volume (veh/h)
    pub v_rr: f64,
    /// Peak hour factor
    pub phf: f64,
    /// Heavy vehicle percentage (decimal, e.g., 0.05 for 5%)
    pub heavy_vehicle_pct: f64,
    /// Terrain type for PCE calculation
    pub terrain: TerrainType,
    /// Minimum lane changes for ramp-to-freeway movement (one-sided only)
    pub lc_rf: u32,
    /// Minimum lane changes for freeway-to-ramp movement (one-sided only)
    pub lc_fr: u32,
    /// Minimum lane changes for ramp-to-ramp movement (two-sided only)
    pub lc_rr: u32,
    /// Interchange density (interchanges/mi)
    pub interchange_density: f64,
    /// Capacity of basic freeway segment with same FFS (pc/h/ln)
    pub basic_freeway_capacity: f64,
    /// Capacity adjustment factor (default 1.0)
    pub caf: f64,
    /// Speed adjustment factor (default 1.0)
    pub saf: f64,
}

/// Terrain type for PCE calculation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TerrainType {
    Level,
    Rolling,
    Mountainous,
}

impl Default for WeavingInput {
    fn default() -> Self {
        Self {
            weaving_type: WeavingType::OneSided,
            facility_type: FacilityType::Freeway,
            length_short: 1500.0,
            num_lanes: 4,
            num_weaving_lanes: 2,
            ffs: 70.0,
            v_ff: 3000.0,
            v_fr: 500.0,
            v_rf: 500.0,
            v_rr: 100.0,
            phf: 0.94,
            heavy_vehicle_pct: 0.05,
            terrain: TerrainType::Level,
            lc_rf: 1,
            lc_fr: 1,
            lc_rr: 0,
            interchange_density: 0.8,
            basic_freeway_capacity: 2400.0,
            caf: 1.0,
            saf: 1.0,
        }
    }
}

// =============================================================================
// Output Results
// =============================================================================

/// Results of weaving segment analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeavingResult {
    /// Total flow rate (pc/h)
    pub v: f64,
    /// Weaving flow rate (pc/h)
    pub v_w: f64,
    /// Nonweaving flow rate (pc/h)
    pub v_nw: f64,
    /// Volume ratio (v_w / v)
    pub volume_ratio: f64,
    /// Minimum lane-changing rate (lc/h)
    pub lc_min: f64,
    /// Maximum weaving length (ft)
    pub l_max: f64,
    /// Whether segment operates as weaving (length < l_max)
    pub is_weaving_segment: bool,
    /// Total lane-changing rate - weaving vehicles (lc/h)
    pub lc_w: f64,
    /// Total lane-changing rate - nonweaving vehicles (lc/h)
    pub lc_nw: f64,
    /// Total lane-changing rate - all vehicles (lc/h)
    pub lc_all: f64,
    /// Lane-changing intensity (lc/ft)
    pub lc_intensity: f64,
    /// Weaving intensity factor
    pub weaving_intensity: f64,
    /// Capacity based on density criterion (veh/h)
    pub capacity_density: f64,
    /// Capacity based on weaving flow criterion (veh/h)
    pub capacity_weaving: Option<f64>,
    /// Final capacity (veh/h)
    pub capacity: f64,
    /// Volume-to-capacity ratio
    pub vc_ratio: f64,
    /// Average speed of weaving vehicles (mi/h)
    pub speed_weaving: f64,
    /// Average speed of nonweaving vehicles (mi/h)
    pub speed_nonweaving: f64,
    /// Average speed of all vehicles (mi/h)
    pub speed_avg: f64,
    /// Density (pc/mi/ln)
    pub density: f64,
    /// Level of Service
    pub los: LevelOfService,
    /// Whether demand exceeds capacity
    pub demand_exceeds_capacity: bool,
}

// =============================================================================
// Heavy Vehicle Adjustment
// =============================================================================

/// Calculate heavy vehicle adjustment factor f_HV
/// Uses PCE values from Exhibit 12-25
pub fn calculate_fhv(heavy_vehicle_pct: f64, terrain: TerrainType) -> f64 {
    let e_t = match terrain {
        TerrainType::Level => 2.0,
        TerrainType::Rolling => 3.0,
        TerrainType::Mountainous => 5.0,
    };

    1.0 / (1.0 + heavy_vehicle_pct * (e_t - 1.0))
}

/// Convert demand volume to flow rate - Equation 13-1
pub fn convert_to_flow_rate(volume: f64, phf: f64, f_hv: f64) -> f64 {
    volume / (phf * f_hv)
}

// =============================================================================
// Configuration Calculations - Step 3
// =============================================================================

/// Calculate minimum lane-changing rate for one-sided weaving - Equation 13-2
/// LCMIN = (LCRF × vRF) + (LCFR × vFR)
pub fn calculate_lc_min_one_sided(lc_rf: u32, v_rf: f64, lc_fr: u32, v_fr: f64) -> f64 {
    (lc_rf as f64) * v_rf + (lc_fr as f64) * v_fr
}

/// Calculate minimum lane-changing rate for two-sided weaving - Equation 13-3
/// LCMIN = LCRR × vRR
pub fn calculate_lc_min_two_sided(lc_rr: u32, v_rr: f64) -> f64 {
    (lc_rr as f64) * v_rr
}

// =============================================================================
// Maximum Weaving Length - Step 4
// =============================================================================

/// Calculate maximum weaving length - Equation 13-4
/// LMAX = 5728 × (1 + VR)^1.6 - 1566 × NWL
pub fn calculate_max_weaving_length(volume_ratio: f64, nwl: u32) -> f64 {
    5728.0 * (1.0 + volume_ratio).powf(1.6) - 1566.0 * (nwl as f64)
}

// =============================================================================
// Capacity Calculations - Step 5
// =============================================================================

/// Calculate capacity per lane based on density criterion - Equation 13-5
/// cIWL = cIFL - 438.2 × (1 + VR)^1.6 + 0.0765 × LS + 119.8 × NWL
pub fn calculate_capacity_per_lane_density(
    basic_capacity: f64,
    volume_ratio: f64,
    length_short: f64,
    nwl: u32,
) -> f64 {
    basic_capacity - 438.2 * (1.0 + volume_ratio).powf(1.6) + 0.0765 * length_short + 119.8 * (nwl as f64)
}

/// Convert ideal capacity to prevailing conditions - Equation 13-6
/// cW = cIWL × N × fHV
pub fn capacity_to_prevailing(c_iwl: f64, num_lanes: u32, f_hv: f64) -> f64 {
    c_iwl * (num_lanes as f64) * f_hv
}

/// Calculate capacity based on weaving flow constraint - Equation 13-7
/// For NWL = 2: cIW = 2400 / VR
/// For NWL = 3: cIW = 3500 / VR
/// For NWL = 0 (two-sided): no limit
pub fn calculate_capacity_weaving_constraint(volume_ratio: f64, nwl: u32) -> Option<f64> {
    if volume_ratio <= 0.0 {
        return None;
    }

    match nwl {
        0 => None, // Two-sided weaving has no weaving flow limit
        2 => Some(MAX_WEAVING_FLOW_NWL2 / volume_ratio),
        3 => Some(MAX_WEAVING_FLOW_NWL3 / volume_ratio),
        _ => Some(MAX_WEAVING_FLOW_NWL2 / volume_ratio), // Default to NWL=2 constraint
    }
}

/// Apply capacity adjustment factor - Equation 13-9
pub fn adjust_capacity(capacity: f64, caf: f64) -> f64 {
    capacity * caf
}

// =============================================================================
// Lane-Changing Rate Calculations - Step 6
// =============================================================================

/// Calculate lane-changing rate for weaving vehicles - Equation 13-11
/// LCW = LCMIN + 0.39 × (LS - 300)^0.5 × N^2 × (1 + ID)^0.8
pub fn calculate_lc_weaving(
    lc_min: f64,
    length_short: f64,
    num_lanes: u32,
    interchange_density: f64,
) -> f64 {
    let ls_adj = (length_short - MIN_WEAVING_LENGTH).max(0.0);
    let n = num_lanes as f64;

    lc_min + 0.39 * ls_adj.sqrt() * n.powi(2) * (1.0 + interchange_density).powf(0.8)
}

/// Calculate nonweaving vehicle index - Equation 13-12
/// INW = LS × ID × vNW / 10000
pub fn calculate_nonweaving_index(length_short: f64, interchange_density: f64, v_nw: f64) -> f64 {
    length_short * interchange_density * v_nw / 10000.0
}

/// Calculate lane-changing rate for nonweaving vehicles (INW <= 1300) - Equation 13-13
/// LCNW1 = (0.206 × vNW) + (0.542 × LS) - (192.6 × N)
pub fn calculate_lc_nonweaving_low(v_nw: f64, length_short: f64, num_lanes: u32) -> f64 {
    let result = 0.206 * v_nw + 0.542 * length_short - 192.6 * (num_lanes as f64);
    result.max(0.0) // Minimum value is 0
}

/// Calculate lane-changing rate for nonweaving vehicles (INW >= 1950) - Equation 13-14
/// LCNW2 = 2135 + 0.223 × (vNW - 2000)
pub fn calculate_lc_nonweaving_high(v_nw: f64) -> f64 {
    2135.0 + 0.223 * (v_nw - 2000.0)
}

/// Calculate lane-changing rate for nonweaving vehicles - Equation 13-16
pub fn calculate_lc_nonweaving(
    v_nw: f64,
    length_short: f64,
    num_lanes: u32,
    interchange_density: f64,
) -> f64 {
    let i_nw = calculate_nonweaving_index(length_short, interchange_density, v_nw);
    let lc_nw1 = calculate_lc_nonweaving_low(v_nw, length_short, num_lanes);
    let lc_nw2 = calculate_lc_nonweaving_high(v_nw);

    if i_nw <= 1300.0 {
        lc_nw1
    } else if i_nw >= 1950.0 {
        lc_nw2
    } else {
        // Interpolation - Equation 13-15
        if lc_nw1 < lc_nw2 {
            lc_nw1 + (lc_nw2 - lc_nw1) * (i_nw - 1300.0) / 650.0
        } else {
            lc_nw2
        }
    }
}

// =============================================================================
// Speed Calculations - Step 7
// =============================================================================

/// Calculate weaving intensity factor
/// W = 0.226 × (LCALL/LS)^0.789
pub fn calculate_weaving_intensity(lc_all: f64, length_short: f64) -> f64 {
    let lc_intensity = lc_all / length_short;
    0.226 * lc_intensity.powf(0.789)
}

/// Calculate average speed of weaving vehicles - Equation 13-19
/// SW = 15 + (FFS × SAF - 15) / (1 + W)
pub fn calculate_weaving_speed(ffs: f64, saf: f64, weaving_intensity: f64) -> f64 {
    let ffs_adj = ffs * saf;
    MIN_WEAVING_SPEED + (ffs_adj - MIN_WEAVING_SPEED) / (1.0 + weaving_intensity)
}

/// Calculate average speed of nonweaving vehicles - Equation 13-21
/// SNW = FFS × SAF - 0.0072 × LCMIN - 0.0048 × (v/N)
pub fn calculate_nonweaving_speed(ffs: f64, saf: f64, lc_min: f64, v: f64, num_lanes: u32) -> f64 {
    let ffs_adj = ffs * saf;
    let v_per_lane = v / (num_lanes as f64);
    (ffs_adj - 0.0072 * lc_min - 0.0048 * v_per_lane).max(MIN_WEAVING_SPEED)
}

/// Calculate average speed of all vehicles - Equation 13-22
/// S = (vW × SW + vNW × SNW) / v
pub fn calculate_average_speed(v_w: f64, s_w: f64, v_nw: f64, s_nw: f64) -> f64 {
    let v = v_w + v_nw;
    if v <= 0.0 {
        return s_w; // Default to weaving speed if no flow
    }
    (v_w * s_w + v_nw * s_nw) / v
}

// =============================================================================
// Density Calculation - Step 8
// =============================================================================

/// Calculate density - Equation 13-23
/// D = (v/N) / S
pub fn calculate_density(v: f64, num_lanes: u32, speed: f64) -> f64 {
    if speed <= 0.0 {
        return f64::INFINITY;
    }
    (v / (num_lanes as f64)) / speed
}

// =============================================================================
// Main Analysis Function
// =============================================================================

/// Perform weaving segment analysis
pub fn analyze(input: &WeavingInput) -> WeavingResult {
    // Step 2: Adjust volume - convert to flow rates
    let f_hv = calculate_fhv(input.heavy_vehicle_pct, input.terrain);
    let v_ff = convert_to_flow_rate(input.v_ff, input.phf, f_hv);
    let v_fr = convert_to_flow_rate(input.v_fr, input.phf, f_hv);
    let v_rf = convert_to_flow_rate(input.v_rf, input.phf, f_hv);
    let v_rr = convert_to_flow_rate(input.v_rr, input.phf, f_hv);

    // Calculate weaving and nonweaving flows based on segment type
    let (v_w, v_nw) = match input.weaving_type {
        WeavingType::OneSided => {
            // Weaving = ramp-to-freeway + freeway-to-ramp
            // Nonweaving = freeway-to-freeway + ramp-to-ramp
            (v_rf + v_fr, v_ff + v_rr)
        },
        WeavingType::TwoSided => {
            // Weaving = ramp-to-ramp only
            // Nonweaving = everything else
            (v_rr, v_ff + v_fr + v_rf)
        },
    };

    let v = v_w + v_nw;
    let volume_ratio = if v > 0.0 { v_w / v } else { 0.0 };

    // Step 3: Determine configuration characteristics
    let nwl = match input.weaving_type {
        WeavingType::OneSided => input.num_weaving_lanes,
        WeavingType::TwoSided => 0, // Always 0 for two-sided
    };

    let lc_min = match input.weaving_type {
        WeavingType::OneSided => calculate_lc_min_one_sided(input.lc_rf, v_rf, input.lc_fr, v_fr),
        WeavingType::TwoSided => calculate_lc_min_two_sided(input.lc_rr, v_rr),
    };

    // Step 4: Determine maximum weaving length
    let l_max = calculate_max_weaving_length(volume_ratio, nwl);
    let is_weaving_segment = input.length_short < l_max;

    // Step 5: Determine capacity
    let c_iwl = calculate_capacity_per_lane_density(
        input.basic_freeway_capacity,
        volume_ratio,
        input.length_short,
        nwl,
    );
    let capacity_density = capacity_to_prevailing(c_iwl, input.num_lanes, f_hv);

    let capacity_weaving = calculate_capacity_weaving_constraint(volume_ratio, nwl)
        .map(|c_iw| capacity_to_prevailing(c_iw / (input.num_lanes as f64), input.num_lanes, f_hv));

    // Final capacity is the minimum of density-based and weaving-based
    let capacity_unadj = match capacity_weaving {
        Some(c_w) => capacity_density.min(c_w),
        None => capacity_density,
    };

    let capacity = adjust_capacity(capacity_unadj, input.caf);

    // Check if demand exceeds capacity
    let total_demand = (input.v_ff + input.v_fr + input.v_rf + input.v_rr) / input.phf;
    let demand_exceeds_capacity = total_demand > capacity;
    let vc_ratio = total_demand / capacity;

    // Step 6: Determine lane-changing rates
    let lc_w = calculate_lc_weaving(lc_min, input.length_short, input.num_lanes, input.interchange_density);
    let lc_nw = calculate_lc_nonweaving(v_nw, input.length_short, input.num_lanes, input.interchange_density);
    let lc_all = lc_w + lc_nw;
    let lc_intensity = lc_all / input.length_short;

    // Step 7: Determine speeds
    let weaving_intensity = calculate_weaving_intensity(lc_all, input.length_short);
    let speed_weaving = calculate_weaving_speed(input.ffs, input.saf, weaving_intensity);
    let speed_nonweaving = calculate_nonweaving_speed(input.ffs, input.saf, lc_min, v, input.num_lanes);
    let speed_avg = calculate_average_speed(v_w, speed_weaving, v_nw, speed_nonweaving);

    // Step 8: Determine LOS
    let density = calculate_density(v, input.num_lanes, speed_avg);
    let los = determine_weaving_los(density, demand_exceeds_capacity, input.facility_type);

    WeavingResult {
        v,
        v_w,
        v_nw,
        volume_ratio,
        lc_min,
        l_max,
        is_weaving_segment,
        lc_w,
        lc_nw,
        lc_all,
        lc_intensity,
        weaving_intensity,
        capacity_density,
        capacity_weaving,
        capacity,
        vc_ratio,
        speed_weaving,
        speed_nonweaving,
        speed_avg,
        density,
        los,
        demand_exceeds_capacity,
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_los_criteria_freeway() {
        assert_eq!(determine_weaving_los(8.0, false, FacilityType::Freeway), LevelOfService::A);
        assert_eq!(determine_weaving_los(15.0, false, FacilityType::Freeway), LevelOfService::B);
        assert_eq!(determine_weaving_los(25.0, false, FacilityType::Freeway), LevelOfService::C);
        assert_eq!(determine_weaving_los(32.0, false, FacilityType::Freeway), LevelOfService::D);
        assert_eq!(determine_weaving_los(40.0, false, FacilityType::Freeway), LevelOfService::E);
        assert_eq!(determine_weaving_los(45.0, false, FacilityType::Freeway), LevelOfService::F);
        assert_eq!(determine_weaving_los(25.0, true, FacilityType::Freeway), LevelOfService::F);
    }

    #[test]
    fn test_los_criteria_multilane() {
        assert_eq!(determine_weaving_los(10.0, false, FacilityType::MultilaneOrCD), LevelOfService::A);
        assert_eq!(determine_weaving_los(20.0, false, FacilityType::MultilaneOrCD), LevelOfService::B);
        assert_eq!(determine_weaving_los(30.0, false, FacilityType::MultilaneOrCD), LevelOfService::C);
        assert_eq!(determine_weaving_los(35.0, false, FacilityType::MultilaneOrCD), LevelOfService::D);
        assert_eq!(determine_weaving_los(38.0, false, FacilityType::MultilaneOrCD), LevelOfService::E);
        assert_eq!(determine_weaving_los(42.0, false, FacilityType::MultilaneOrCD), LevelOfService::F);
    }

    #[test]
    fn test_heavy_vehicle_adjustment() {
        // Level terrain (E_T = 2.0) with 5% trucks
        let f_hv = calculate_fhv(0.05, TerrainType::Level);
        assert!((f_hv - 0.9524).abs() < 0.01);

        // Rolling terrain (E_T = 3.0) with 10% trucks
        let f_hv = calculate_fhv(0.10, TerrainType::Rolling);
        assert!((f_hv - 0.8333).abs() < 0.01);
    }

    #[test]
    fn test_lc_min_one_sided() {
        // LCRF = 1, vRF = 500, LCFR = 1, vFR = 400
        let lc_min = calculate_lc_min_one_sided(1, 500.0, 1, 400.0);
        assert_eq!(lc_min, 900.0);
    }

    #[test]
    fn test_lc_min_two_sided() {
        // LCRR = 2, vRR = 200
        let lc_min = calculate_lc_min_two_sided(2, 200.0);
        assert_eq!(lc_min, 400.0);
    }

    #[test]
    fn test_max_weaving_length() {
        // VR = 0.3, NWL = 2
        let l_max = calculate_max_weaving_length(0.3, 2);
        assert!(l_max > 3000.0 && l_max < 6000.0);

        // VR = 0.3, NWL = 3 should be longer
        let l_max_3 = calculate_max_weaving_length(0.3, 3);
        assert!(l_max_3 < l_max); // More weaving lanes = shorter max length
    }

    #[test]
    fn test_capacity_weaving_constraint() {
        // NWL = 2, VR = 0.4
        let c = calculate_capacity_weaving_constraint(0.4, 2);
        assert!(c.is_some());
        assert!((c.unwrap() - 6000.0).abs() < 1.0); // 2400 / 0.4 = 6000

        // NWL = 0 (two-sided) has no limit
        let c_two_sided = calculate_capacity_weaving_constraint(0.4, 0);
        assert!(c_two_sided.is_none());
    }

    #[test]
    fn test_weaving_analysis() {
        let input = WeavingInput {
            weaving_type: WeavingType::OneSided,
            facility_type: FacilityType::Freeway,
            length_short: 1500.0,
            num_lanes: 4,
            num_weaving_lanes: 2,
            ffs: 70.0,
            v_ff: 3000.0,
            v_fr: 500.0,
            v_rf: 600.0,
            v_rr: 100.0,
            phf: 0.94,
            heavy_vehicle_pct: 0.05,
            terrain: TerrainType::Level,
            lc_rf: 1,
            lc_fr: 1,
            lc_rr: 0,
            interchange_density: 0.8,
            basic_freeway_capacity: 2400.0,
            caf: 1.0,
            saf: 1.0,
        };

        let result = analyze(&input);

        // Check that results are reasonable
        assert!(result.v > 0.0);
        assert!(result.v_w > 0.0);
        assert!(result.v_nw > 0.0);
        assert!(result.volume_ratio > 0.0 && result.volume_ratio < 1.0);
        assert!(result.speed_avg > 0.0 && result.speed_avg <= input.ffs);
        assert!(result.density > 0.0);
        assert!(result.capacity > 0.0);
    }

    #[test]
    fn test_two_sided_weaving() {
        let input = WeavingInput {
            weaving_type: WeavingType::TwoSided,
            facility_type: FacilityType::Freeway,
            length_short: 1200.0,
            num_lanes: 4,
            num_weaving_lanes: 0, // Always 0 for two-sided
            ffs: 65.0,
            v_ff: 3500.0,
            v_fr: 400.0,
            v_rf: 500.0,
            v_rr: 300.0, // This is the weaving flow
            phf: 0.92,
            heavy_vehicle_pct: 0.08,
            terrain: TerrainType::Level,
            lc_rf: 0,
            lc_fr: 0,
            lc_rr: 2, // Ramp-to-ramp requires 2 lane changes
            interchange_density: 0.6,
            basic_freeway_capacity: 2350.0,
            caf: 1.0,
            saf: 1.0,
        };

        let result = analyze(&input);

        // For two-sided, weaving flow should be ramp-to-ramp only
        assert!(result.v_w < result.v_nw);
        assert!(result.density > 0.0);
    }
}
