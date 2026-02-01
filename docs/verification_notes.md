# HCM Implementation Verification Notes

This document tracks assumptions and values that need verification against the HCM 7th Edition manual.

---

## Chapter 11: Freeway Reliability Analysis (`adjustment_factors.rs`)

### Items Requiring Verification

#### 1. Incident Delay Rate Coefficients (Equation 11-3)
**Status:** NEEDS VERIFICATION
**Location:** `adjustment_factors.rs` - `calculate_incident_delay_rate()`

Currently using assumed coefficients:
```
N=2 lanes: a=0.000584, b=6.32
N=3 lanes: a=0.000382, b=5.42
N=4 lanes: a=0.000260, b=4.56
```

**Action:** Verify Equation 11-3 coefficients from HCM Chapter 11 or Chapter 25.

---

#### 2. Work Zone CAF Ranges
**Status:** NEEDS VERIFICATION
**Location:** `adjustment_factors.rs` - `WorkZoneType::get_caf_range()`

Currently using:
```
ShoulderWork: 0.90-0.95
OneLaneClosure: 0.70-0.85
TwoLaneClosure: 0.50-0.65
ThreePlusLaneClosure: 0.30-0.50 (assumed, may not exist)
```

**Action:** Verify work zone CAFs from HCM Chapter 10, Section 4. The `ThreePlusLaneClosure` variant may need to be removed if not in manual.

---

#### 3. Driver Population Adjustments
**Status:** NEEDS VERIFICATION
**Location:** `adjustment_factors.rs` - `DriverPopulation` enum

Currently using:
```
Commuter: CAF=1.00, SAF=1.00
Recreational: CAF=0.85-0.95, SAF=0.95
TouristHeavy: CAF=0.75-0.90, SAF=0.90
```

**Action:** Verify driver population CAF/SAF from HCM Chapter 26 or relevant section. These values were from the implementation plan, not directly from Chapter 11.

---

#### 4. FFS Interpolation Method
**Status:** NEEDS VERIFICATION
**Location:** `adjustment_factors.rs` - `WeatherCondition::interpolate_by_ffs()`

Currently using linear interpolation between FFS bins (55, 60, 65, 70, 75 mi/h).

**Action:** Verify if HCM specifies interpolation method or requires using exact bin values only.

---

## Verified Items (From Chapter 11 Exhibits)

### Weather CAFs - Exhibit 11-20
**Status:** VERIFIED
Values match exhibit exactly for all 11 weather types across 5 FFS bins.

### Weather SAFs - Exhibit 11-21
**Status:** VERIFIED
Values match exhibit exactly for all 11 weather types across 5 FFS bins.

### Incident Severity Distribution - Exhibit 11-22
**Status:** VERIFIED
```
Shoulder: 75.4%
1 Lane: 19.6%
2 Lanes: 3.1%
3 Lanes: 1.9%
4+ Lanes: 0%
```

### Incident Duration Parameters - Exhibit 11-22
**Status:** VERIFIED
Mean, std dev, min, max durations match exhibit.

### Incident CAFs by Lane Count - Exhibit 11-23
**Status:** VERIFIED
CAF values for 2-8 directional lanes match exhibit exactly.

### Urban Demand Ratios - Exhibit 11-18
**Status:** VERIFIED
12 months x 7 days matrix matches exhibit.

### Rural Demand Ratios - Exhibit 11-19
**Status:** VERIFIED
12 months x 7 days matrix matches exhibit.

---

## Notes

- Planning-level reliability equations (11-1, 11-4, 11-5) structure is correct, but Equation 11-3 coefficients need verification
- The manual references Chapter 25 for detailed computational procedures - may need to consult for full equation details
- Work zone analysis is primarily in Chapter 10, not Chapter 11

---

*Last Updated: 2026-01-31*
