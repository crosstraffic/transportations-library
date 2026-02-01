# HCM Basic Freeways Implementation Plan

This document outlines the implementation plan for completing the Highway Capacity Manual (HCM) Chapter 12 methodology and related extensions for the transportations-library.

## Current Implementation Status

### Completed (Chapter 12)

| Component | Status | File |
|-----------|--------|------|
| Basic Freeway Segments Core | Done | `src/hcm/basicfreeways.rs` |
| Multilane Highway Segments Core | Done | `src/hcm/basicfreeways.rs` |
| Speed-Flow Relationship (Eq 12-1) | Done | `basicfreeways.rs` |
| FFS Estimation (Eq 12-2, 12-3) | Done | `basicfreeways.rs` |
| TLC Calculation (Eq 12-4) | Done | `basicfreeways.rs` |
| FFS Adjustment (Eq 12-5) | Done | `basicfreeways.rs` |
| Capacity Estimation (Eq 12-6, 12-7, 12-8) | Done | `basicfreeways.rs` |
| Demand Volume Adjustment (Eq 12-9, 12-10) | Done | `basicfreeways.rs` |
| Density Estimation (Eq 12-11) | Done | `basicfreeways.rs` |
| Planning Analysis (Eq 12-20 to 12-26) | Done | `basicfreeways.rs` |
| LOS Criteria (Exhibit 12-15) | Done | `basicfreeways.rs` |
| Default Values (Exhibit 12-18) | Done | `basicfreeways.rs` |
| Adjustment Tables with Interpolation | Done | `basicfreeways.rs` |
| Managed Lane Segments (Eq 12-12 to 12-19) | Done | `src/hcm/managed_lanes.rs` |
| Managed Lane Parameters (Exhibit 12-30) | Done | `managed_lanes.rs` |
| Bicycle LOS Criteria (Exhibit 12-31) | Done | `src/hcm/common.rs` |
| PCE Tables (Exhibits 12-25 to 12-28) | Partial | `src/hcm/utils/pce_table.rs` |

---

## Phase 1: Chapter 11 Integration (Weather, Incidents, Work Zones)

**Priority: High**
**Estimated Complexity: Medium**

### 1.1 Capacity Adjustment Factors (CAF)

Create a new module `src/hcm/adjustment_factors.rs` with:

```rust
pub struct CapacityAdjustmentFactors {
    pub weather: WeatherCAF,
    pub incident: IncidentCAF,
    pub work_zone: WorkZoneCAF,
    pub driver_population: f64,
}
```

#### Weather CAF (from Chapter 11)

| Condition | CAF Range |
|-----------|-----------|
| Clear/Dry | 1.00 |
| Rain (light) | 0.95-0.98 |
| Rain (heavy) | 0.89-0.93 |
| Snow (light) | 0.93-0.96 |
| Snow (heavy) | 0.78-0.85 |
| Ice | 0.70-0.80 |
| Fog | 0.90-0.95 |

#### Incident CAF

| Incident Type | Lanes Blocked | CAF |
|---------------|---------------|-----|
| Shoulder | 0 | 0.81-0.95 |
| 1 Lane | 1 | 0.35-0.50 |
| 2 Lanes | 2 | 0.15-0.25 |
| 3+ Lanes | 3+ | 0.00-0.10 |

#### Work Zone CAF

| Work Zone Type | CAF |
|----------------|-----|
| Lane closure (1 lane) | 0.70-0.85 |
| Lane closure (2 lanes) | 0.50-0.65 |
| Shoulder work | 0.90-0.95 |

### 1.2 Speed Adjustment Factors (SAF)

Similar structure for speed adjustments:

| Condition | SAF Range |
|-----------|-----------|
| Clear/Dry | 1.00 |
| Rain (light) | 0.96-0.99 |
| Rain (heavy) | 0.90-0.94 |
| Snow (light) | 0.92-0.95 |
| Snow (heavy) | 0.85-0.90 |

### Implementation Tasks

- [x] Create `adjustment_factors.rs` module
- [x] Implement `WeatherCondition` enum
- [x] Implement `IncidentSeverity` enum (was IncidentType)
- [x] Implement `WorkZoneType` enum
- [x] Add CAF lookup tables from Chapter 11 (Exhibit 11-20, 11-23)
- [x] Add SAF lookup tables from Chapter 11 (Exhibit 11-21)
- [x] Add demand ratio tables (Exhibit 11-18, 11-19)
- [x] Implement planning-level reliability (Equations 11-1 to 11-5)
- [x] Add DriverPopulation enum for CAF/SAF adjustments
- [x] Add ReliabilityScenario struct for scenario generation
- [ ] Integrate with `BasicFreeways` struct (CAF/SAF already in BasicFreeways)
- [x] Add unit tests (9 tests passing)

---

## Phase 2: Mixed-Flow Model (Chapters 25/26)

**Priority: Medium**
**Estimated Complexity: High**

The mixed-flow model provides more accurate speed/density estimates when:
- Truck percentages > 10%
- Grades > 3%
- Composite grades present

### 2.1 Single Grade Analysis (Chapter 26)

Create `src/hcm/mixed_flow.rs`:

```rust
pub struct MixedFlowAnalysis {
    pub segment_length: f64,
    pub grade: f64,
    pub truck_pct: f64,
    pub sut_pct: f64,  // Within truck population
    pub tt_pct: f64,   // Within truck population
}

impl MixedFlowAnalysis {
    pub fn calculate_mixed_speed(&self) -> f64;
    pub fn calculate_mixed_density(&self) -> f64;
    pub fn calculate_truck_speed(&self) -> f64;
    pub fn calculate_auto_speed(&self) -> f64;
}
```

### 2.2 Composite Grade Analysis (Chapter 25)

For grades with varying percentages over distance:

```rust
pub struct CompositeGrade {
    pub segments: Vec<GradeSegment>,
}

pub struct GradeSegment {
    pub length: f64,
    pub grade: f64,
}

impl CompositeGrade {
    pub fn equivalent_grade(&self) -> f64;
    pub fn analyze(&self, traffic: &TrafficParams) -> MixedFlowResult;
}
```

### Implementation Tasks

- [ ] Create `mixed_flow.rs` module
- [ ] Implement truck performance curves
- [ ] Implement crawl speed calculations
- [ ] Implement composite grade equivalency
- [ ] Add weight-to-horsepower ratio parameters
- [ ] Integrate with existing PCE calculations
- [ ] Add unit tests with example problems from Chapter 26

---

## Phase 3: Freeway Facilities Analysis (Chapter 10)

**Priority: High**
**Estimated Complexity: Very High**

This is a major module for analyzing complete freeway facilities with multiple segments.

### 3.1 Facility Structure

Create `src/hcm/freeway_facility.rs`:

```rust
pub struct FreewayFacility {
    pub segments: Vec<FreewaySegment>,
    pub analysis_periods: Vec<AnalysisPeriod>,
}

pub enum FreewaySegment {
    Basic(BasicFreeways),
    Merge(MergeSegment),
    Diverge(DivergeSegment),
    Weave(WeaveSegment),
    ManagedLane(ManagedLaneSegment),
}

pub struct AnalysisPeriod {
    pub start_time: u32,  // minutes
    pub duration: u32,    // typically 15 min
    pub demands: Vec<f64>,
}
```

### 3.2 Oversaturated Analysis

```rust
pub struct OversaturatedAnalysis {
    pub queue_length: f64,
    pub queue_density: f64,
    pub delay: f64,
    pub spillback_segments: Vec<usize>,
}
```

### 3.3 Key Algorithms

1. **Node-Link Model**: Track flow between segments
2. **Queue Spillback**: Propagate congestion upstream
3. **Demand Starvation**: Reduce flow when upstream queued
4. **Lane-by-Lane Analysis**: Optional detailed analysis

### Implementation Tasks

- [ ] Create `freeway_facility.rs` module
- [ ] Implement segment linking/connectivity
- [ ] Implement demand propagation
- [ ] Implement queue spillback algorithm
- [ ] Implement time-space analysis
- [ ] Add facility-level LOS calculation
- [ ] Add visualization/output structures
- [ ] Add comprehensive tests

---

## Phase 4: Merge/Diverge/Weave Segments (Chapters 13/14)

**Priority: Medium**
**Estimated Complexity: Medium**

### 4.1 Merge Segments (Chapter 14)

```rust
pub struct MergeSegment {
    pub freeway_ffs: f64,
    pub ramp_ffs: f64,
    pub acceleration_lane_length: f64,
    pub freeway_demand: f64,
    pub ramp_demand: f64,
    pub number_of_lanes: u32,
}
```

Key equations:
- Merge influence area capacity
- Density in merge influence area
- Speed estimation in merge area

### 4.2 Diverge Segments (Chapter 14)

```rust
pub struct DivergeSegment {
    pub freeway_ffs: f64,
    pub deceleration_lane_length: f64,
    pub freeway_demand: f64,
    pub off_ramp_demand: f64,
    pub number_of_lanes: u32,
}
```

### 4.3 Weaving Segments (Chapter 13)

```rust
pub struct WeaveSegment {
    pub segment_type: WeaveType,  // A, B, C
    pub length: f64,
    pub lane_configuration: LaneConfig,
    pub weaving_demands: WeavingDemands,
}

pub enum WeaveType {
    TypeA,  // One-sided
    TypeB,  // Two-sided, single weave
    TypeC,  // Two-sided, multiple weave
}
```

### Implementation Tasks

- [ ] Create `merge_diverge.rs` module
- [ ] Create `weaving.rs` module
- [ ] Implement merge capacity equations
- [ ] Implement diverge capacity equations
- [ ] Implement weave lane configuration analysis
- [ ] Implement weave speed-flow relationships
- [ ] Add LOS criteria for each segment type
- [ ] Add unit tests

---

## Phase 5: Bicycle LOS Calculation (Chapter 15)

**Priority: Low**
**Estimated Complexity: Low**

Complete the bicycle LOS methodology referenced in Chapter 12.

### 5.1 Bicycle LOS Score Calculation

```rust
pub struct BicycleLOSCalculator {
    pub outside_lane_width: f64,
    pub shoulder_width: f64,
    pub motor_vehicle_volume: f64,
    pub motor_vehicle_speed: f64,
    pub heavy_vehicle_pct: f64,
    pub pavement_condition: u8,  // 1-5 FHWA scale
    pub number_of_lanes: u32,
}

impl BicycleLOSCalculator {
    pub fn calculate_score(&self) -> f64;
    pub fn determine_los(&self) -> LevelOfService;
}
```

### 5.2 Key Formula Components

1. Effective width of outside lane
2. Volume factor
3. Speed factor
4. Heavy vehicle factor
5. Pavement condition factor

### Implementation Tasks

- [ ] Create `bicycle_los.rs` module
- [ ] Implement effective width calculation
- [ ] Implement traveler perception model
- [ ] Add input validation (ranges from Chapter 15)
- [ ] Integrate with multilane highway analysis
- [ ] Add unit tests

---

## Phase 6: State-Specific Defaults (Chapter 26)

**Priority: Low**
**Estimated Complexity: Low**

### 6.1 Heavy Vehicle Percentages by State

Create `src/hcm/state_defaults.rs`:

```rust
pub fn get_state_heavy_vehicle_pct(state: &str, facility_type: FacilityType) -> f64;
pub fn get_state_driver_population_factor(state: &str) -> f64;
```

### 6.2 Driver Population Adjustment

For recreational/tourist areas:

| Driver Population | CAF | SAF |
|-------------------|-----|-----|
| Commuter (familiar) | 1.00 | 1.00 |
| Recreational | 0.85-0.95 | 0.95 |
| Tourist heavy | 0.75-0.90 | 0.90 |

### Implementation Tasks

- [ ] Create state lookup table from HPMS data
- [ ] Implement driver population adjustment
- [ ] Add regional default system
- [ ] Add unit tests

---

## Phase 7: Reliability Analysis (Chapter 11)

**Priority: Medium**
**Estimated Complexity: High**

### 7.1 Travel Time Reliability

```rust
pub struct ReliabilityAnalysis {
    pub scenarios: Vec<Scenario>,
    pub probabilities: Vec<f64>,
}

pub struct Scenario {
    pub weather: WeatherCondition,
    pub incident: Option<IncidentType>,
    pub demand_multiplier: f64,
}

impl ReliabilityAnalysis {
    pub fn calculate_tti(&self) -> f64;  // Travel Time Index
    pub fn calculate_pti(&self) -> f64;  // Planning Time Index
    pub fn calculate_buffer_index(&self) -> f64;
}
```

### Implementation Tasks

- [ ] Create `reliability.rs` module
- [ ] Implement scenario generation
- [ ] Implement probability distributions
- [ ] Calculate reliability metrics
- [ ] Add Monte Carlo simulation option
- [ ] Add unit tests

---

## Implementation Priority Matrix

| Phase | Module | Priority | Complexity | Dependencies |
|-------|--------|----------|------------|--------------|
| 1 | Adjustment Factors | High | Medium | None |
| 2 | Mixed-Flow Model | Medium | High | Phase 1 |
| 3 | Freeway Facilities | High | Very High | Phases 1, 4 |
| 4 | Merge/Diverge/Weave | Medium | Medium | Phase 1 |
| 5 | Bicycle LOS | Low | Low | None |
| 6 | State Defaults | Low | Low | None |
| 7 | Reliability | Medium | High | Phases 1, 3 |

---

## Recommended Implementation Order

1. **Phase 1** - Adjustment Factors (enables weather/incident analysis)
2. **Phase 4** - Merge/Diverge/Weave (needed for facility analysis)
3. **Phase 3** - Freeway Facilities (major feature, depends on 1 & 4)
4. **Phase 2** - Mixed-Flow Model (enhances accuracy for trucks/grades)
5. **Phase 7** - Reliability (builds on facility analysis)
6. **Phase 5** - Bicycle LOS (standalone, low priority)
7. **Phase 6** - State Defaults (nice to have)

---

## Testing Strategy

Each phase should include:

1. **Unit Tests**: Test individual functions/methods
2. **Integration Tests**: Test module interactions
3. **Example Problems**: Implement HCM example problems from Chapter 26
4. **Validation**: Compare results with known HCM solutions

### Example Problems to Implement

From Chapter 26, Section 7:
- [ ] Example 1: Four-lane freeway LOS
- [ ] Example 2: Number of lanes for target LOS
- [ ] Example 3: Six-lane freeway with capacity analysis
- [ ] Example 4: Five-lane multilane highway with TWLTL
- [ ] Example 5: Mixed-flow with high truck percentage
- [ ] Example 6: Severe weather effects
- [ ] Example 7: Managed lane with friction effects

---

## File Structure After Implementation

```
src/hcm/
├── mod.rs
├── common.rs                    # Shared types, LOS definitions
├── basicfreeways.rs            # Chapter 12 core (Done)
├── managed_lanes.rs            # Chapter 12 Section 4 (Done)
├── adjustment_factors.rs       # Phase 1: CAF/SAF from Chapter 11
├── mixed_flow.rs               # Phase 2: Chapters 25/26
├── freeway_facility.rs         # Phase 3: Chapter 10
├── merge_diverge.rs            # Phase 4: Chapter 14
├── weaving.rs                  # Phase 4: Chapter 13
├── bicycle_los.rs              # Phase 5: Chapter 15
├── state_defaults.rs           # Phase 6: Chapter 26
├── reliability.rs              # Phase 7: Chapter 11
└── utils/
    ├── mod.rs
    ├── pce_table.rs            # PCE lookup tables (Existing)
    └── interpolation.rs        # Interpolation utilities
```

---

## References

- HCM 7th Edition, Chapter 10: Freeway Facilities Core Methodology
- HCM 7th Edition, Chapter 11: Freeway Reliability Analysis
- HCM 7th Edition, Chapter 12: Basic Freeway and Multilane Highway Segments
- HCM 7th Edition, Chapter 13: Freeway Weaving Segments
- HCM 7th Edition, Chapter 14: Freeway Merge and Diverge Segments
- HCM 7th Edition, Chapter 15: Two-Lane Highways (Bicycle LOS)
- HCM 7th Edition, Chapter 25: Freeway Facilities: Supplemental
- HCM 7th Edition, Chapter 26: Freeway and Highway Segments: Supplemental

---

*Last Updated: 2026-01-31*
*Document Version: 1.0*
