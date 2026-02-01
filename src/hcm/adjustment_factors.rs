use serde::{Deserialize, Serialize};

/// Chapter 11: Freeway Reliability Analysis - Adjustment Factors
///
/// This module provides Capacity Adjustment Factors (CAF) and Speed Adjustment
/// Factors (SAF) for weather, incidents, and work zones as defined in HCM Chapter 11.

// =============================================================================
// Weather Conditions (Exhibit 11-20 and 11-21)
// =============================================================================

/// Weather event types as defined in HCM Chapter 11
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WeatherCondition {
    /// >0.10-0.25 in./h
    MediumRain,
    /// >0.25 in./h
    HeavyRain,
    /// >0.00-0.05 in./h
    LightSnow,
    /// >0.05-0.10 in./h
    LightMediumSnow,
    /// >0.10-0.50 in./h
    MediumHeavySnow,
    /// >0.50 in./h
    HeavySnow,
    /// <-4°F
    SevereCold,
    /// 0.50-0.99 mi visibility
    LowVisibility,
    /// 0.25-0.49 mi visibility
    VeryLowVisibility,
    /// <0.25 mi visibility
    MinimalVisibility,
    /// All conditions not listed above
    NonSevereWeather,
}

impl WeatherCondition {
    /// Get the Capacity Adjustment Factor (CAF) for this weather condition
    /// based on facility free-flow speed.
    /// From Exhibit 11-20: Default CAFs by Weather Condition
    pub fn get_caf(&self, ffs: f64) -> f64 {
        // FFS bins: 55, 60, 65, 70, 75 mi/h
        let caf_values = match self {
            WeatherCondition::MediumRain => [0.94, 0.93, 0.92, 0.91, 0.90],
            WeatherCondition::HeavyRain => [0.89, 0.88, 0.86, 0.84, 0.82],
            WeatherCondition::LightSnow => [0.97, 0.96, 0.96, 0.95, 0.95],
            WeatherCondition::LightMediumSnow => [0.95, 0.94, 0.92, 0.90, 0.88],
            WeatherCondition::MediumHeavySnow => [0.93, 0.91, 0.90, 0.88, 0.87],
            WeatherCondition::HeavySnow => [0.80, 0.78, 0.76, 0.74, 0.72],
            WeatherCondition::SevereCold => [0.93, 0.92, 0.92, 0.91, 0.90],
            WeatherCondition::LowVisibility => [0.90, 0.90, 0.90, 0.90, 0.90],
            WeatherCondition::VeryLowVisibility => [0.88, 0.88, 0.88, 0.88, 0.88],
            WeatherCondition::MinimalVisibility => [0.90, 0.90, 0.90, 0.90, 0.90],
            WeatherCondition::NonSevereWeather => [1.00, 1.00, 1.00, 1.00, 1.00],
        };

        Self::interpolate_by_ffs(ffs, &caf_values)
    }

    /// Get the Speed Adjustment Factor (SAF) for this weather condition
    /// based on facility free-flow speed.
    /// From Exhibit 11-21: Default SAFs by Weather Condition
    pub fn get_saf(&self, ffs: f64) -> f64 {
        // FFS bins: 55, 60, 65, 70, 75 mi/h
        let saf_values = match self {
            WeatherCondition::MediumRain => [0.96, 0.95, 0.94, 0.93, 0.93],
            WeatherCondition::HeavyRain => [0.94, 0.93, 0.93, 0.92, 0.91],
            WeatherCondition::LightSnow => [0.94, 0.92, 0.89, 0.87, 0.84],
            WeatherCondition::LightMediumSnow => [0.92, 0.90, 0.88, 0.86, 0.83],
            WeatherCondition::MediumHeavySnow => [0.90, 0.88, 0.86, 0.84, 0.82],
            WeatherCondition::HeavySnow => [0.88, 0.86, 0.85, 0.83, 0.81],
            WeatherCondition::SevereCold => [0.95, 0.95, 0.94, 0.93, 0.92],
            WeatherCondition::LowVisibility => [0.96, 0.95, 0.94, 0.94, 0.93],
            WeatherCondition::VeryLowVisibility => [0.95, 0.94, 0.93, 0.92, 0.91],
            WeatherCondition::MinimalVisibility => [0.95, 0.94, 0.93, 0.92, 0.91],
            WeatherCondition::NonSevereWeather => [1.00, 1.00, 1.00, 1.00, 1.00],
        };

        Self::interpolate_by_ffs(ffs, &saf_values)
    }

    /// Interpolate adjustment factor based on FFS
    ///
    /// # Note
    /// TODO: VERIFY - Using linear interpolation between FFS bins.
    /// Check if HCM specifies a different interpolation method or requires exact bin values only.
    /// See docs/verification_notes.md for details.
    fn interpolate_by_ffs(ffs: f64, values: &[f64; 5]) -> f64 {
        let ffs_bins = [55.0, 60.0, 65.0, 70.0, 75.0];

        if ffs <= ffs_bins[0] {
            return values[0];
        }
        if ffs >= ffs_bins[4] {
            return values[4];
        }

        for i in 0..4 {
            if ffs >= ffs_bins[i] && ffs <= ffs_bins[i + 1] {
                let ratio = (ffs - ffs_bins[i]) / (ffs_bins[i + 1] - ffs_bins[i]);
                return values[i] + ratio * (values[i + 1] - values[i]);
            }
        }

        values[2] // Default to 65 mi/h value
    }
}

// =============================================================================
// Incident Types (Exhibit 11-22 and 11-23)
// =============================================================================

/// Incident severity types as defined in HCM Chapter 11
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IncidentSeverity {
    /// Shoulder closure only
    ShoulderClosed,
    /// One lane blocked
    OneLaneClosed,
    /// Two lanes blocked
    TwoLanesClosed,
    /// Three lanes blocked
    ThreeLanesClosed,
    /// Four or more lanes blocked
    FourPlusLanesClosed,
}

/// Incident duration parameters from Exhibit 11-22
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncidentDuration {
    /// Mean duration in minutes
    pub mean: f64,
    /// Standard deviation in minutes
    pub std_dev: f64,
    /// Minimum duration in minutes
    pub min: f64,
    /// Maximum duration in minutes
    pub max: f64,
}

impl IncidentSeverity {
    /// Get the default incident severity distribution percentage
    /// From Exhibit 11-22
    pub fn get_distribution_pct(&self) -> f64 {
        match self {
            IncidentSeverity::ShoulderClosed => 75.4,
            IncidentSeverity::OneLaneClosed => 19.6,
            IncidentSeverity::TwoLanesClosed => 3.1,
            IncidentSeverity::ThreeLanesClosed => 1.9,
            IncidentSeverity::FourPlusLanesClosed => 0.0,
        }
    }

    /// Get the default incident duration parameters
    /// From Exhibit 11-22
    pub fn get_duration_params(&self) -> IncidentDuration {
        match self {
            IncidentSeverity::ShoulderClosed => IncidentDuration {
                mean: 34.0,
                std_dev: 15.1,
                min: 8.7,
                max: 58.0,
            },
            IncidentSeverity::OneLaneClosed => IncidentDuration {
                mean: 34.6,
                std_dev: 13.8,
                min: 16.0,
                max: 58.2,
            },
            IncidentSeverity::TwoLanesClosed => IncidentDuration {
                mean: 53.6,
                std_dev: 13.9,
                min: 30.5,
                max: 66.9,
            },
            IncidentSeverity::ThreeLanesClosed => IncidentDuration {
                mean: 67.9,
                std_dev: 21.9,
                min: 36.0,
                max: 93.3,
            },
            IncidentSeverity::FourPlusLanesClosed => IncidentDuration {
                mean: 67.9,
                std_dev: 21.9,
                min: 36.0,
                max: 93.3,
            },
        }
    }

    /// Get the Capacity Adjustment Factor (CAF) for this incident type
    /// based on number of directional lanes.
    /// From Exhibit 11-23: CAFs by Incident Type and Number of Directional Lanes
    ///
    /// Returns None if the incident is not applicable (lanes closed >= directional lanes)
    pub fn get_caf(&self, directional_lanes: u32) -> Option<f64> {
        if directional_lanes < 2 || directional_lanes > 8 {
            return None;
        }

        // CAF table indexed by [lanes - 2][incident_type]
        // Columns: No Incident, Shoulder, 1 Lane, 2 Lanes, 3 Lanes, 4 Lanes
        let caf_table: [[Option<f64>; 6]; 7] = [
            // 2 lanes
            [Some(1.00), Some(0.81), Some(0.70), None, None, None],
            // 3 lanes
            [Some(1.00), Some(0.83), Some(0.74), Some(0.51), None, None],
            // 4 lanes
            [Some(1.00), Some(0.85), Some(0.77), Some(0.50), Some(0.52), None],
            // 5 lanes
            [Some(1.00), Some(0.87), Some(0.81), Some(0.67), Some(0.50), Some(0.50)],
            // 6 lanes
            [Some(1.00), Some(0.89), Some(0.85), Some(0.75), Some(0.52), Some(0.52)],
            // 7 lanes
            [Some(1.00), Some(0.91), Some(0.88), Some(0.80), Some(0.63), Some(0.63)],
            // 8 lanes
            [Some(1.00), Some(0.93), Some(0.89), Some(0.84), Some(0.66), Some(0.66)],
        ];

        let row = (directional_lanes - 2) as usize;
        let col = match self {
            IncidentSeverity::ShoulderClosed => 1,
            IncidentSeverity::OneLaneClosed => 2,
            IncidentSeverity::TwoLanesClosed => 3,
            IncidentSeverity::ThreeLanesClosed => 4,
            IncidentSeverity::FourPlusLanesClosed => 5,
        };

        caf_table[row][col]
    }

    /// Get the number of lanes blocked by this incident
    pub fn lanes_blocked(&self) -> u32 {
        match self {
            IncidentSeverity::ShoulderClosed => 0,
            IncidentSeverity::OneLaneClosed => 1,
            IncidentSeverity::TwoLanesClosed => 2,
            IncidentSeverity::ThreeLanesClosed => 3,
            IncidentSeverity::FourPlusLanesClosed => 4,
        }
    }
}

// =============================================================================
// Work Zone Types
// =============================================================================

/// Work zone types for capacity adjustment
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkZoneType {
    /// Shoulder work only
    ShoulderWork,
    /// One lane closed
    OneLaneClosure,
    /// Two lanes closed
    TwoLaneClosure,
    /// Three or more lanes closed
    ThreePlusLaneClosure,
}

/// Work zone barrier types affecting capacity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkZoneBarrier {
    /// Cones, drums, or other soft barrier
    SoftBarrier,
    /// Concrete or other hard barrier
    HardBarrier,
}

impl WorkZoneType {
    /// Get the default CAF range for this work zone type
    /// Returns (min_caf, max_caf)
    ///
    /// # Note
    /// TODO: VERIFY - These values need verification from HCM Chapter 10, Section 4.
    /// The ThreePlusLaneClosure variant may not exist in the manual.
    /// See docs/verification_notes.md for details.
    pub fn get_caf_range(&self) -> (f64, f64) {
        match self {
            WorkZoneType::ShoulderWork => (0.90, 0.95),
            WorkZoneType::OneLaneClosure => (0.70, 0.85),
            WorkZoneType::TwoLaneClosure => (0.50, 0.65),
            WorkZoneType::ThreePlusLaneClosure => (0.30, 0.50), // TODO: VERIFY - may not exist
        }
    }

    /// Get a typical CAF value for this work zone type
    pub fn get_typical_caf(&self) -> f64 {
        let (min, max) = self.get_caf_range();
        (min + max) / 2.0
    }

    /// Get the number of lanes closed
    pub fn lanes_closed(&self) -> u32 {
        match self {
            WorkZoneType::ShoulderWork => 0,
            WorkZoneType::OneLaneClosure => 1,
            WorkZoneType::TwoLaneClosure => 2,
            WorkZoneType::ThreePlusLaneClosure => 3,
        }
    }
}

// =============================================================================
// Driver Population Adjustment
// =============================================================================

/// Driver population types affecting capacity and speed
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DriverPopulation {
    /// Commuter (familiar with roadway)
    Commuter,
    /// Recreational drivers
    Recreational,
    /// Heavy tourist traffic
    TouristHeavy,
}

impl DriverPopulation {
    /// Get the CAF for this driver population type
    /// Returns (min_caf, max_caf)
    ///
    /// # Note
    /// TODO: VERIFY - These values need verification from HCM Chapter 26 or relevant section.
    /// See docs/verification_notes.md for details.
    pub fn get_caf_range(&self) -> (f64, f64) {
        match self {
            DriverPopulation::Commuter => (1.00, 1.00),
            DriverPopulation::Recreational => (0.85, 0.95),  // TODO: VERIFY
            DriverPopulation::TouristHeavy => (0.75, 0.90),  // TODO: VERIFY
        }
    }

    /// Get the SAF for this driver population type
    ///
    /// # Note
    /// TODO: VERIFY - These values need verification from HCM Chapter 26 or relevant section.
    /// See docs/verification_notes.md for details.
    pub fn get_saf(&self) -> f64 {
        match self {
            DriverPopulation::Commuter => 1.00,
            DriverPopulation::Recreational => 0.95,  // TODO: VERIFY
            DriverPopulation::TouristHeavy => 0.90,  // TODO: VERIFY
        }
    }

    /// Get typical CAF value
    pub fn get_typical_caf(&self) -> f64 {
        let (min, max) = self.get_caf_range();
        (min + max) / 2.0
    }
}

// =============================================================================
// Demand Ratios (Exhibit 11-18 and 11-19)
// =============================================================================

/// Day of week for demand ratio lookup
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DayOfWeek {
    Monday = 0,
    Tuesday = 1,
    Wednesday = 2,
    Thursday = 3,
    Friday = 4,
    Saturday = 5,
    Sunday = 6,
}

/// Month of year for demand ratio lookup
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Month {
    January = 0,
    February = 1,
    March = 2,
    April = 3,
    May = 4,
    June = 5,
    July = 6,
    August = 7,
    September = 8,
    October = 9,
    November = 10,
    December = 11,
}

/// Urban freeway demand ratios from Exhibit 11-18
/// Ratios relative to Monday in January
pub fn get_urban_demand_ratio(month: Month, day: DayOfWeek) -> f64 {
    // [month][day_of_week]
    // From Exhibit 11-18: Default Urban Freeway Demand Ratios (ADT/Mondays in January)
    let ratios: [[f64; 7]; 12] = [
        // Jan: Mon, Tue, Wed, Thu, Fri, Sat, Sun
        [1.00, 1.00, 1.02, 1.05, 1.17, 1.01, 0.89],
        // Feb
        [1.03, 1.03, 1.05, 1.08, 1.21, 1.04, 0.92],
        // Mar
        [1.12, 1.12, 1.14, 1.18, 1.31, 1.13, 0.99],
        // Apr
        [1.19, 1.19, 1.21, 1.25, 1.39, 1.20, 1.05],
        // May
        [1.18, 1.18, 1.21, 1.24, 1.39, 1.20, 1.05],
        // Jun
        [1.24, 1.24, 1.27, 1.31, 1.46, 1.26, 1.10],
        // Jul
        [1.38, 1.38, 1.41, 1.45, 1.62, 1.39, 1.22],
        // Aug
        [1.26, 1.26, 1.28, 1.32, 1.47, 1.27, 1.12],
        // Sep
        [1.29, 1.29, 1.32, 1.36, 1.52, 1.31, 1.15],
        // Oct
        [1.21, 1.21, 1.24, 1.27, 1.42, 1.22, 1.07],
        // Nov
        [1.21, 1.21, 1.24, 1.27, 1.42, 1.22, 1.07],
        // Dec
        [1.19, 1.19, 1.21, 1.25, 1.40, 1.20, 1.06],
    ];

    ratios[month as usize][day as usize]
}

/// Rural freeway demand ratios from Exhibit 11-19
/// Ratios relative to Monday in January
pub fn get_rural_demand_ratio(month: Month, day: DayOfWeek) -> f64 {
    // [month][day_of_week]
    // From Exhibit 11-19: Default Rural Freeway Demand Ratios (ADT/Mondays in January)
    let ratios: [[f64; 7]; 12] = [
        // Jan: Mon, Tue, Wed, Thu, Fri, Sat, Sun
        [1.00, 0.96, 0.98, 1.03, 1.22, 1.11, 1.06],
        // Feb
        [1.11, 1.06, 1.09, 1.14, 1.35, 1.23, 1.18],
        // Mar
        [1.24, 1.19, 1.21, 1.28, 1.51, 1.37, 1.32],
        // Apr
        [1.33, 1.27, 1.30, 1.37, 1.62, 1.47, 1.41],
        // May
        [1.46, 1.39, 1.42, 1.50, 1.78, 1.61, 1.55],
        // Jun
        [1.48, 1.42, 1.45, 1.53, 1.81, 1.63, 1.57],
        // Jul
        [1.66, 1.59, 1.63, 1.72, 2.03, 1.84, 1.77],
        // Aug
        [1.52, 1.46, 1.49, 1.57, 1.86, 1.68, 1.62],
        // Sep
        [1.46, 1.39, 1.42, 1.50, 1.78, 1.61, 1.55],
        // Oct
        [1.33, 1.28, 1.31, 1.38, 1.63, 1.47, 1.42],
        // Nov
        [1.30, 1.25, 1.28, 1.35, 1.59, 1.44, 1.39],
        // Dec
        [1.17, 1.12, 1.14, 1.20, 1.43, 1.29, 1.24],
    ];

    ratios[month as usize][day as usize]
}

// =============================================================================
// Combined Adjustment Factors
// =============================================================================

/// Combined capacity adjustment factors for reliability analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapacityAdjustmentFactors {
    /// Weather-related CAF
    pub weather_caf: f64,
    /// Incident-related CAF
    pub incident_caf: Option<f64>,
    /// Work zone CAF
    pub work_zone_caf: f64,
    /// Driver population CAF
    pub driver_population_caf: f64,
}

impl Default for CapacityAdjustmentFactors {
    fn default() -> Self {
        Self {
            weather_caf: 1.0,
            incident_caf: None,
            work_zone_caf: 1.0,
            driver_population_caf: 1.0,
        }
    }
}

impl CapacityAdjustmentFactors {
    /// Create new adjustment factors
    pub fn new() -> Self {
        Self::default()
    }

    /// Set weather CAF from weather condition
    pub fn with_weather(mut self, condition: WeatherCondition, ffs: f64) -> Self {
        self.weather_caf = condition.get_caf(ffs);
        self
    }

    /// Set incident CAF from incident severity
    pub fn with_incident(mut self, severity: IncidentSeverity, directional_lanes: u32) -> Self {
        self.incident_caf = severity.get_caf(directional_lanes);
        self
    }

    /// Set work zone CAF
    pub fn with_work_zone(mut self, work_zone: WorkZoneType) -> Self {
        self.work_zone_caf = work_zone.get_typical_caf();
        self
    }

    /// Set driver population CAF
    pub fn with_driver_population(mut self, population: DriverPopulation) -> Self {
        self.driver_population_caf = population.get_typical_caf();
        self
    }

    /// Calculate combined CAF (multiplicative)
    /// When multiple factors affect capacity, their combined effect is multiplicative
    pub fn combined_caf(&self) -> f64 {
        let mut caf = self.weather_caf * self.work_zone_caf * self.driver_population_caf;
        if let Some(incident_caf) = self.incident_caf {
            caf *= incident_caf;
        }
        caf
    }
}

/// Combined speed adjustment factors for reliability analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeedAdjustmentFactors {
    /// Weather-related SAF
    pub weather_saf: f64,
    /// Driver population SAF
    pub driver_population_saf: f64,
}

impl Default for SpeedAdjustmentFactors {
    fn default() -> Self {
        Self {
            weather_saf: 1.0,
            driver_population_saf: 1.0,
        }
    }
}

impl SpeedAdjustmentFactors {
    /// Create new adjustment factors
    pub fn new() -> Self {
        Self::default()
    }

    /// Set weather SAF from weather condition
    pub fn with_weather(mut self, condition: WeatherCondition, ffs: f64) -> Self {
        self.weather_saf = condition.get_saf(ffs);
        self
    }

    /// Set driver population SAF
    pub fn with_driver_population(mut self, population: DriverPopulation) -> Self {
        self.driver_population_saf = population.get_saf();
        self
    }

    /// Calculate combined SAF (multiplicative)
    pub fn combined_saf(&self) -> f64 {
        self.weather_saf * self.driver_population_saf
    }
}

/// Demand Adjustment Factor for reliability analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DemandAdjustmentFactor {
    /// Base demand multiplier from day/month combination
    pub demand_ratio: f64,
    /// Additional multiplier for special events, weather effects, etc.
    pub additional_multiplier: f64,
}

impl Default for DemandAdjustmentFactor {
    fn default() -> Self {
        Self {
            demand_ratio: 1.0,
            additional_multiplier: 1.0,
        }
    }
}

impl DemandAdjustmentFactor {
    /// Create new DAF
    pub fn new() -> Self {
        Self::default()
    }

    /// Set demand ratio for urban freeway
    pub fn with_urban_demand(mut self, month: Month, day: DayOfWeek) -> Self {
        self.demand_ratio = get_urban_demand_ratio(month, day);
        self
    }

    /// Set demand ratio for rural freeway
    pub fn with_rural_demand(mut self, month: Month, day: DayOfWeek) -> Self {
        self.demand_ratio = get_rural_demand_ratio(month, day);
        self
    }

    /// Set additional multiplier
    pub fn with_multiplier(mut self, multiplier: f64) -> Self {
        self.additional_multiplier = multiplier;
        self
    }

    /// Calculate combined DAF
    pub fn combined_daf(&self) -> f64 {
        self.demand_ratio * self.additional_multiplier
    }
}

// =============================================================================
// Reliability Analysis Parameters
// =============================================================================

/// Scenario for reliability analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReliabilityScenario {
    /// Day of week
    pub day: DayOfWeek,
    /// Month of year
    pub month: Month,
    /// Weather condition
    pub weather: WeatherCondition,
    /// Incident severity (if any)
    pub incident: Option<IncidentSeverity>,
    /// Work zone type (if any)
    pub work_zone: Option<WorkZoneType>,
    /// Driver population
    pub driver_population: DriverPopulation,
}

impl ReliabilityScenario {
    /// Create a base scenario with no incidents or work zones
    pub fn new(day: DayOfWeek, month: Month) -> Self {
        Self {
            day,
            month,
            weather: WeatherCondition::NonSevereWeather,
            incident: None,
            work_zone: None,
            driver_population: DriverPopulation::Commuter,
        }
    }

    /// Set weather condition
    pub fn with_weather(mut self, weather: WeatherCondition) -> Self {
        self.weather = weather;
        self
    }

    /// Set incident
    pub fn with_incident(mut self, incident: IncidentSeverity) -> Self {
        self.incident = Some(incident);
        self
    }

    /// Set work zone
    pub fn with_work_zone(mut self, work_zone: WorkZoneType) -> Self {
        self.work_zone = Some(work_zone);
        self
    }

    /// Set driver population
    pub fn with_driver_population(mut self, population: DriverPopulation) -> Self {
        self.driver_population = population;
        self
    }

    /// Calculate CAF for this scenario
    pub fn calculate_caf(&self, ffs: f64, directional_lanes: u32, is_urban: bool) -> f64 {
        let mut caf_builder = CapacityAdjustmentFactors::new()
            .with_weather(self.weather, ffs)
            .with_driver_population(self.driver_population);

        if let Some(incident) = self.incident {
            caf_builder = caf_builder.with_incident(incident, directional_lanes);
        }

        if let Some(work_zone) = self.work_zone {
            caf_builder = caf_builder.with_work_zone(work_zone);
        }

        // Note: is_urban is available for future extensions
        let _ = is_urban;

        caf_builder.combined_caf()
    }

    /// Calculate SAF for this scenario
    pub fn calculate_saf(&self, ffs: f64) -> f64 {
        SpeedAdjustmentFactors::new()
            .with_weather(self.weather, ffs)
            .with_driver_population(self.driver_population)
            .combined_saf()
    }

    /// Calculate DAF for this scenario
    pub fn calculate_daf(&self, is_urban: bool) -> f64 {
        if is_urban {
            get_urban_demand_ratio(self.month, self.day)
        } else {
            get_rural_demand_ratio(self.month, self.day)
        }
    }
}

// =============================================================================
// Travel Time Index Calculations (Planning Level - Equations 11-1 to 11-5)
// =============================================================================

/// Calculate recurring delay rate (Equation 11-2)
///
/// # Arguments
/// * `ffs` - Free-flow speed (mi/h)
/// * `peak_hour_speed` - Peak hour speed (mi/h)
///
/// # Returns
/// Recurring delay rate in hours per mile
pub fn calculate_recurring_delay_rate(ffs: f64, peak_hour_speed: f64) -> f64 {
    if peak_hour_speed <= 0.0 {
        return 0.0;
    }
    (1.0 / peak_hour_speed) - (1.0 / ffs)
}

/// Calculate incident delay rate (Equation 11-3)
///
/// # Arguments
/// * `num_lanes` - Number of lanes in one direction (2 to 4)
/// * `vc_ratio` - Peak hour volume-to-capacity ratio (capped at 1.0)
///
/// # Returns
/// Incident delay rate in hours per mile
///
/// # Note
/// TODO: VERIFY COEFFICIENTS - The coefficients below are assumed values.
/// Need to verify from HCM Chapter 11 or Chapter 25 for exact Equation 11-3 formula.
/// See docs/verification_notes.md for details.
pub fn calculate_incident_delay_rate(num_lanes: u32, vc_ratio: f64) -> f64 {
    // Cap values as specified in the methodology
    let n = num_lanes.min(4).max(2);
    let x = vc_ratio.min(1.0);

    // Equation 11-3 coefficients depend on number of lanes
    // IDR = a * X^b where a and b depend on N
    // TODO: VERIFY - These coefficients are assumed, not from manual
    let (a, b) = match n {
        2 => (0.000584, 6.32),
        3 => (0.000382, 5.42),
        4 => (0.000260, 4.56),
        _ => (0.000382, 5.42), // Default to 3 lanes
    };

    a * x.powf(b)
}

/// Calculate mean travel time index (Equation 11-1)
///
/// # Arguments
/// * `ffs` - Free-flow speed (mi/h)
/// * `recurring_delay_rate` - From Equation 11-2
/// * `incident_delay_rate` - From Equation 11-3
///
/// # Returns
/// Mean annual travel time index (unitless, >= 1.0)
pub fn calculate_mean_tti(ffs: f64, recurring_delay_rate: f64, incident_delay_rate: f64) -> f64 {
    1.0 + ffs * (recurring_delay_rate + incident_delay_rate)
}

/// Calculate 95th percentile travel time index (Equation 11-4)
///
/// # Arguments
/// * `tti_mean` - Mean travel time index from Equation 11-1
///
/// # Returns
/// 95th percentile TTI (unitless)
pub fn calculate_tti_95(tti_mean: f64) -> f64 {
    1.0 + 3.67 * (tti_mean - 1.0)
}

/// Calculate percent of trips below 45 mi/h (Equation 11-5)
///
/// # Arguments
/// * `tti_mean` - Mean travel time index
///
/// # Returns
/// Percentage of trips at speeds < 45 mi/h (0.0 to 1.0)
pub fn calculate_pt_45(tti_mean: f64) -> f64 {
    let pt = (tti_mean - 1.0) / 0.0135;
    pt.max(0.0).min(1.0)
}

/// Planning-level reliability analysis results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanningReliabilityResult {
    /// Mean annual travel time index
    pub tti_mean: f64,
    /// 95th percentile travel time index (Planning Time Index)
    pub tti_95: f64,
    /// Percent of trips below 45 mi/h
    pub pt_45: f64,
    /// Recurring delay rate (h/mi)
    pub recurring_delay_rate: f64,
    /// Incident delay rate (h/mi)
    pub incident_delay_rate: f64,
}

/// Perform planning-level reliability analysis
///
/// # Arguments
/// * `ffs` - Free-flow speed (mi/h)
/// * `peak_hour_speed` - Peak hour speed (mi/h)
/// * `num_lanes` - Number of directional lanes
/// * `vc_ratio` - Peak hour volume-to-capacity ratio
///
/// # Returns
/// Planning-level reliability results
pub fn planning_reliability_analysis(
    ffs: f64,
    peak_hour_speed: f64,
    num_lanes: u32,
    vc_ratio: f64,
) -> PlanningReliabilityResult {
    let rdr = calculate_recurring_delay_rate(ffs, peak_hour_speed);
    let idr = calculate_incident_delay_rate(num_lanes, vc_ratio);
    let tti_mean = calculate_mean_tti(ffs, rdr, idr);
    let tti_95 = calculate_tti_95(tti_mean);
    let pt_45 = calculate_pt_45(tti_mean);

    PlanningReliabilityResult {
        tti_mean,
        tti_95,
        pt_45,
        recurring_delay_rate: rdr,
        incident_delay_rate: idr,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weather_caf() {
        // Test at exact FFS values
        assert!((WeatherCondition::MediumRain.get_caf(55.0) - 0.94).abs() < 0.001);
        assert!((WeatherCondition::MediumRain.get_caf(75.0) - 0.90).abs() < 0.001);
        assert!((WeatherCondition::HeavySnow.get_caf(65.0) - 0.76).abs() < 0.001);
        assert!((WeatherCondition::NonSevereWeather.get_caf(70.0) - 1.00).abs() < 0.001);
    }

    #[test]
    fn test_weather_saf() {
        assert!((WeatherCondition::MediumRain.get_saf(55.0) - 0.96).abs() < 0.001);
        assert!((WeatherCondition::LightSnow.get_saf(75.0) - 0.84).abs() < 0.001);
        assert!((WeatherCondition::NonSevereWeather.get_saf(65.0) - 1.00).abs() < 0.001);
    }

    #[test]
    fn test_incident_caf() {
        // From Exhibit 11-23
        assert_eq!(IncidentSeverity::ShoulderClosed.get_caf(2), Some(0.81));
        assert_eq!(IncidentSeverity::OneLaneClosed.get_caf(3), Some(0.74));
        assert_eq!(IncidentSeverity::TwoLanesClosed.get_caf(6), Some(0.75));
        assert_eq!(IncidentSeverity::TwoLanesClosed.get_caf(2), None); // N/A
    }

    #[test]
    fn test_incident_distribution() {
        assert!((IncidentSeverity::ShoulderClosed.get_distribution_pct() - 75.4).abs() < 0.1);
        assert!((IncidentSeverity::OneLaneClosed.get_distribution_pct() - 19.6).abs() < 0.1);
    }

    #[test]
    fn test_urban_demand_ratio() {
        // From Exhibit 11-18: Default Urban Freeway Demand Ratios
        // Monday in January should be 1.00
        assert!((get_urban_demand_ratio(Month::January, DayOfWeek::Monday) - 1.00).abs() < 0.01);
        // Tuesday in January should be 1.00
        assert!((get_urban_demand_ratio(Month::January, DayOfWeek::Tuesday) - 1.00).abs() < 0.01);
        // Sunday in January should be 0.89
        assert!((get_urban_demand_ratio(Month::January, DayOfWeek::Sunday) - 0.89).abs() < 0.01);
        // Friday in July should be 1.62
        assert!((get_urban_demand_ratio(Month::July, DayOfWeek::Friday) - 1.62).abs() < 0.01);
    }

    #[test]
    fn test_rural_demand_ratio() {
        // From Exhibit 11-19: Default Rural Freeway Demand Ratios
        // Monday in January should be 1.00
        assert!((get_rural_demand_ratio(Month::January, DayOfWeek::Monday) - 1.00).abs() < 0.01);
        // Tuesday in January should be 0.96
        assert!((get_rural_demand_ratio(Month::January, DayOfWeek::Tuesday) - 0.96).abs() < 0.01);
        // Friday in July should be 2.03 (highest)
        assert!((get_rural_demand_ratio(Month::July, DayOfWeek::Friday) - 2.03).abs() < 0.01);
    }

    #[test]
    fn test_combined_caf() {
        let caf = CapacityAdjustmentFactors::new()
            .with_weather(WeatherCondition::MediumRain, 65.0)
            .with_incident(IncidentSeverity::ShoulderClosed, 4);

        // 0.92 * 0.85 = 0.782
        assert!((caf.combined_caf() - 0.782).abs() < 0.01);
    }

    #[test]
    fn test_planning_reliability() {
        let result = planning_reliability_analysis(70.0, 55.0, 3, 0.9);

        // TTI mean should be > 1.0
        assert!(result.tti_mean > 1.0);
        // TTI 95 should be > TTI mean
        assert!(result.tti_95 > result.tti_mean);
        // PT 45 should be between 0 and 1
        assert!(result.pt_45 >= 0.0 && result.pt_45 <= 1.0);
    }

    #[test]
    fn test_scenario_calculations() {
        let scenario = ReliabilityScenario::new(DayOfWeek::Friday, Month::July)
            .with_weather(WeatherCondition::MediumRain)
            .with_incident(IncidentSeverity::OneLaneClosed);

        let caf = scenario.calculate_caf(70.0, 4, true);
        let saf = scenario.calculate_saf(70.0);
        let daf = scenario.calculate_daf(true);

        // All factors should be reasonable
        assert!(caf > 0.0 && caf <= 1.0);
        assert!(saf > 0.0 && saf <= 1.0);
        assert!(daf > 0.0);
    }
}
