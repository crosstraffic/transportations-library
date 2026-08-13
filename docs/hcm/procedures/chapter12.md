# Chapter 12: Basic Freeway and Multilane Highway Segments — Procedure Walkthrough

HCM 7th Edition Chapter 12 covers basic freeway and multilane highway segments (Sections 2-3, operational core in Exhibit 12-19's six steps) plus the basic managed lane segment methodology (Section 4, Equations 12-12 through 12-19). Both are implemented under `src/hcm/basicfreeways/` (renamed from `src/hcm/chapter12/`; the old name is retained as a `pub use basicfreeways as chapter12` re-export in `src/hcm/mod.rs` for compatibility): `basicfreeways.rs` (the `BasicFreeways` struct handling both basic freeway and multilane highway variants via the `highway_type` string field, `"basic"` vs `"multilane"`) and `managed_lanes.rs` (the `ManagedLaneSegment` struct with per-type parameters from Exhibit 12-30). This branch's commit `ea74afa` corrected three managed-lane equations and one Exhibit 12-37 table entry against the manual (detailed below; all three equation fixes were cross-checked against the HCM7 EPUB MathML for this pass and reproduce the printed forms exactly). Python bindings for both structs are registered in `src/copython/basicfreeways.rs` (`BasicFreeways`, `ManagedLanes` pyclasses).

## Operational analysis walkthrough (Exhibit 12-19 steps)

The full sequence is orchestrated by `BasicFreeways::run_operational_analysis()`, which chains the steps below and returns the LOS.

| Manual step | HCM Eq./Exhibit | Rust method | File | Inputs (units) | Output (units) |
|---|---|---|---|---|---|
| Step 1: input data | Exhibit 12-18 defaults | `DefaultValues::{urban_freeway, rural_freeway, urban_multilane, rural_multilane, suburban_multilane_high_density}` + `BasicFreeways::with_*_defaults()` / `apply_defaults()` | `basicfreeways.rs` | — | populated struct |
| Step 2: FFS estimation (freeway) | Eq 12-2 | private `estimate_basic_lane_ffs` via `determine_free_flow_speed` | `basicfreeways.rs` | `bffs` (mi/h), lane width `lw` (ft), right lateral clearance `lc_r` (ft), total ramp density `trd` (ramps/mi) | `ffs` mi/h; `FFS = BFFS − f_LW − f_RLC − 3.22·TRD^0.84` |
| Step 2: FFS estimation (multilane) | Eq 12-3, Eq 12-4 (TLC) | private `estimate_multi_lane_ffs`; `calculate_total_lateral_clearance` | `basicfreeways.rs` | `bffs`, `lw`, `lc_r`+`lc_l` (ft, each capped 6 ft), median type, access point density `apd` (pts/mi) | `ffs` mi/h; `FFS = BFFS − f_LW − f_TLC − f_M − f_A` |
| Step 2 adjustments | Exhibit 12-20 (lane width), Exhibit 12-21 (right clearance), Exhibit 12-22 (TLC, with linear interpolation), Exhibit 12-24 (access points) | `adjustment_average_lane_width`, `adjustment_right_side_lateral_clearance` (+ `_interpolated` variant), `adjustment_total_lateral_clearance`, `adjustment_median_type`, `adjustment_access_point_density` (formula `min(0.25·APD, 10)`; a table-lookup variant `adjustment_access_point_density_table` exists but is `#[allow(dead_code)]`) | `basicfreeways.rs` | as above | mi/h reductions |
| Step 2: adjusted FFS | Eq 12-5 | `determine_free_flow_speed` (tail) | `basicfreeways.rs` | `saf` (unitless) | `ffs_adj = ffs × SAF`, mi/h |
| Step 3: capacity | Eq 12-6 (basic, `2200 + 10(FFS−50)`, max 2,400), Eq 12-7 (multilane, `1900 + 20(FFS−45)`, max 2,300), Eq 12-8 (`c_adj = c × CAF`). Both read the UNADJUSTED FFS, per the December 2022 errata | `estimate_capacity`, `estimate_adjusted_capacity` | `basicfreeways.rs` | `ffs` (mi/h), `caf` | pc/h/ln |
| Step 4: demand adjustment | Eq 12-9 (`v_p = V/(PHF·N·f_HV)`), Eq 12-10 (`f_HV = 1/(1+P_T(E_T−1))`), Exhibit 12-25/12-26 PCE | `estimate_demand_volume`, private `adjustment_heavy_vehicle_factor` (uses `ET_TABLE_30SUT/50SUT/70SUT` from `src/hcm/common/pce_table.rs` keyed by SUT mix) | `basicfreeways.rs` | `demand_flow_i` (veh/h), `phf`, `lane_count`, `p_t` (decimal), terrain, grade (%), length (mi) | `v_p` pc/h/ln |
| Step 5: speed-flow curve | Eq 12-1 with Exhibit 12-6 parameters (breakpoint `BP = [1000 + 40(75 − FFS_adj)]·CAF²` basic; BP = 1,400 constant multilane; exponent a = 2.0 basic / 1.31 multilane; density at capacity 45 pc/mi/ln) | `calculate_breakpoint`, `calculate_speed` | `basicfreeways.rs` | `v_p`, `ffs_adj`, `capacity_adj` | space mean speed S, mi/h (0.0 sentinel when demand > capacity) |
| Step 5: density | Eq 12-11 (`D = v_p/S`) | `estimate_density` | `basicfreeways.rs` | `v_p` (pc/h/ln), S (mi/h) | pc/mi/ln (sentinel 46.0 when oversaturated) |
| Step 6: LOS | Exhibit 12-15 | `determine_segment_los` (v/c check then `common::los_tables::los_basic_freeway`; no area-type split, unlike the Exhibit 10-6 facility table) | `basicfreeways.rs` | density, v/c | `LevelOfService` A-F |

### Equations (Steps 2-6)

All equation forms below were cross-checked against the HCM 7th Edition EPUB MathML (`resources/epub/OEBPS/82_Ch12_02.xhtml` for Eq 12-1, `83_Ch12_03.xhtml` for Eq 12-2 through 12-11) and reproduce the printed forms exactly; no discrepancies were found for this chapter's core methodology.

```
Equation 12-2 (Step 2, basic freeway FFS):  FFS = BFFS − f_LW − f_RLC − 3.22 × TRD^0.84
  FFS = free-flow speed of the basic freeway segment, mi/h
  BFFS = base free-flow speed, mi/h (struct field `bffs`, default 65.0 in BasicFreeways::new(); the HCM default base FFS of 75.4 mi/h is declared as DEFAULT_BFFS_FREEWAY but is not read by the constructor, see Deviation 6)
  f_LW = adjustment for lane width, mi/h (Exhibit 12-20: >=12 ft -> 0.0, >=11-12 ft -> 1.9, >=10-11 ft -> 6.6)
  f_RLC = adjustment for right-side lateral clearance, mi/h (Exhibit 12-21, keyed on lane count and clearance in ft, 0-6 ft; interpolated variant available for non-integer clearance)
  TRD = total ramp density, ramps/mi (struct field `trd`; ramps within 3 mi upstream/downstream of the segment midpoint, divided by 6 mi)
Implemented in: basicfreeways/basicfreeways.rs::estimate_basic_lane_ffs (via determine_free_flow_speed); adjustments in adjustment_average_lane_width, adjustment_right_side_lateral_clearance[_interpolated]
```

```
Equation 12-3 (Step 2, multilane highway FFS):  FFS = BFFS − f_LW − f_TLC − f_M − f_A
  FFS = free-flow speed of the multilane highway segment, mi/h
  BFFS = base free-flow speed, mi/h (struct field `bffs`)
  f_LW = adjustment for lane width, mi/h (Exhibit 12-20, shared with Eq 12-2)
  f_TLC = adjustment for total lateral clearance, mi/h (Exhibit 12-22, four-lane vs six-lane tables, linearly interpolated between tabulated TLC values)
  f_M = adjustment for median type, mi/h (Exhibit 12-23: undivided 1.6, TWLTL 0.0, divided 0.0)
  f_A = adjustment for access point density, mi/h (Exhibit 12-24: min(0.25 × APD, 10.0), interpolation to the nearest 0.1 recommended by the exhibit note; a table-lookup variant matching the exhibit's tabulated points (0/10/20/30/>=40 -> 0.0/2.5/5.0/7.5/10.0) also exists but is `#[allow(dead_code)]`)
Implemented in: basicfreeways/basicfreeways.rs::estimate_multi_lane_ffs (via determine_free_flow_speed); adjustments in adjustment_average_lane_width, adjustment_total_lateral_clearance, adjustment_median_type, adjustment_access_point_density[_table]
```

```
Equation 12-4 (TLC, feeds Eq 12-3):  TLC = LC_R + LC_L
  TLC = total lateral clearance, ft (maximum value 12 ft)
  LC_R = right-side lateral clearance, ft (maximum value 6 ft; struct field `lc_r`, default 6)
  LC_L = left-side lateral clearance, ft (maximum value 6 ft; struct field `lc_l`, default 6)
Implemented in: basicfreeways/basicfreeways.rs::calculate_total_lateral_clearance
```

```
Equation 12-5 (Step 2, SAF adjustment, basic freeway only):  FFS_adj = FFS × SAF
  FFS_adj = adjusted free-flow speed, mi/h
  FFS = free-flow speed from Eq 12-2/12-3 (or field-measured), mi/h
  SAF = speed adjustment factor, decimal (struct field `saf`, default 1.0 = base conditions; weather/work-zone/driver-population SAF defaults live in Chapter 11, src/hcm/common/adjustment_factors.rs — see Deferred)
Note: the book states "no adjustment of the speed-flow equation using these SAFs is possible for multilane highway segments" (no empirical research); the struct computes FFS_adj = FFS × SAF for both highway_type values, so a caller analyzing a multilane segment should leave `saf` at 1.0 to stay within the documented method.
Implemented in: basicfreeways/basicfreeways.rs::determine_free_flow_speed
```

```
Equation 12-1 (Step 5, speed-flow curve):  S = FFS_adj                                                          if v_p <= BP
                                            S = FFS_adj − [(FFS_adj − c_adj/D_c) / (c_adj − BP)^a] × (v_p − BP)^a   if BP < v_p <= c_adj
  S = space mean speed of the traffic stream, mi/h
  FFS_adj = adjusted free-flow speed, mi/h (Eq 12-5)
  v_p = demand flow rate under equivalent base conditions, pc/h/ln (Eq 12-9)
  BP = breakpoint in the speed-flow curve, pc/h/ln — basic freeway: BP = [1,000 + 40 × (75 − FFS_adj)] × CAF², multilane: BP = 1,400 (constant), both from Exhibit 12-6
  c_adj = adjusted segment capacity, pc/h/ln (Eq 12-8)
  D_c = density at capacity = DENSITY_AT_CAPACITY = 45.0 pc/mi/ln (Exhibit 12-6)
  a = exponent calibration parameter (Exhibit 12-6): EXPONENT_BASIC_FREEWAY = 2.00 (basic freeway), EXPONENT_MULTILANE = 1.31 (multilane)
  Returns 0.0 (breakdown sentinel) when v_p > c_adj — see Deviation 5
Implemented in: basicfreeways/basicfreeways.rs::calculate_speed; breakpoint in calculate_breakpoint
```

```
Equation 12-6 (Step 3, basic freeway base capacity):     c = 2,200 + 10 × (FFS − 50)   [capped at 2,400 pc/h/ln, valid 55 <= FFS <= 75]
Equation 12-7 (Step 3, multilane highway base capacity): c = 1,900 + 20 × (FFS − 45)   [capped at 2,300 pc/h/ln, valid 45 <= FFS <= 70]
  c = base segment capacity, pc/h/ln
  FFS = free-flow speed BEFORE the Equation 12-5 SAF adjustment, mi/h
Implemented in: basicfreeways/basicfreeways.rs::estimate_capacity
```

```
Equation 12-8 (Step 3, adjusted capacity, basic freeway only):  c_adj = c × CAF
  c_adj = adjusted capacity of the segment, pc/h/ln
  c = base capacity, pc/h/ln (Eq 12-6/12-7)
  CAF = capacity adjustment factor, decimal (struct field `caf`, default 1.0 = base conditions; per the book, no CAF adjustment is defined for multilane highways either)
Implemented in: basicfreeways/basicfreeways.rs::estimate_adjusted_capacity
```

Equations 12-6 and 12-7 as originally printed read FFS_adj. The December 2022 corrections replace that with the unadjusted FFS, so a SAF reaches capacity only through the separate CAF of Equation 12-8, never twice. Chapter 26 reworks one of its own worked examples to demonstrate it: Example Problem 6's heavy-snow capacity becomes `c = 0.78 x (2,200 + 10 x [60.8 - 50]) = 1,800 pc/h/ln` where the chapter as printed computes `0.78 x (2,200 + 10 x [52.3 - 50]) = 1,734`. `estimate_capacity` therefore reads `self.ffs`, not `self.ffs_adj`, while `calculate_speed` and `calculate_breakpoint` keep reading `ffs_adj`. That asymmetry is deliberate and is pinned by `ch26_ep6_heavy_snow_basic_freeway` in `tests/chapter12_integration.rs`. See the Chapter 26 row of `docs/hcm/VERIFICATION.md`.

```
Equation 12-9 (Step 4, demand flow rate):  v_p = V / (PHF × N × f_HV)
  v_p = demand flow rate under equivalent base conditions, pc/h/ln
  V = demand volume under prevailing conditions, veh/h (struct field `demand_flow_i`)
  PHF = peak hour factor, decimal (struct field `phf`)
  N = number of lanes in the analysis direction, ln (struct field `lane_count`, default 2)
  f_HV = heavy vehicle adjustment factor, decimal (Eq 12-10; struct field `phv`)
Implemented in: basicfreeways/basicfreeways.rs::estimate_demand_volume
```

```
Equation 12-10 (Step 4, heavy vehicle adjustment):  f_HV = 1 / (1 + P_T × (E_T − 1))
  f_HV = heavy vehicle adjustment factor, decimal
  P_T = proportion of SUTs and TTs in the traffic stream, decimal (struct field `p_t`)
  E_T = passenger car equivalent of one heavy vehicle, PCEs — Exhibit 12-25 general terrain when `sut_percentage` is 0 (level 2.0, rolling 3.0, matched case-insensitively; mountainous 2.5 is a non-HCM approximation, see Deviation 1), or the Exhibit 12-26/12-27/12-28 specific-upgrade tables for a 30/50/70% SUT mix, keyed on (grade %, length mi, truck %). The three tables live in `src/hcm/common/pce_table.rs` as `PceTable` statics generated from the EPUB by `scripts/gen_pce_table.py`; `PceTable::lookup` interpolates linearly on grade, length, and truck percentage, which the exhibits explicitly permit ("Interpolation in the exhibit is permitted"), and the ">25%" column is read as a bucket for any P_T at or above 25%. Inputs outside the exhibit domain return an error rather than a default (see Deviation 2)
Implemented in: basicfreeways/basicfreeways.rs::adjustment_heavy_vehicle_factor
```

```
Equation 12-11 (Step 5, density):  D = v_p / S
  D = density, pc/mi/ln (sentinel DENSITY_AT_CAPACITY + 1.0 = 46.0 when v_p/c_adj > 1.0 or S <= 0.0 — the book states density is undefined once v_p/c exceeds 1.00 and directs analysts to Chapter 10; see Deviation 5)
  v_p = demand flow rate, pc/h/ln (Eq 12-9)
  S = mean speed of the traffic stream, mi/h (Eq 12-1)
Implemented in: basicfreeways/basicfreeways.rs::estimate_density
```

### Planning and design analysis

Planning-level entry points: `estimate_ddhv` (Eq 12-20, `DDHV = AADT × K × D`), `estimate_number_of_lanes` (Eqs 12-21/12-22/12-23, `N = ceil(v/MSF_i)`), `estimate_lanes_from_aadt` (chains 12-20 into 12-23), `determine_basic_max_service_flow_rate` / `determine_multilane_max_service_flow_rate` (Exhibit 12-37/12-38 MSF tables keyed on FFS rounded to the nearest 5 mi/h and target LOS), `calculate_service_flow_rate` (Eq 12-24), `calculate_service_volume` (Eq 12-25), and `calculate_daily_service_volume` (Eq 12-26). Commit `ea74afa` corrected the Exhibit 12-37 FFS-60/LOS-A entry to 660 pc/h/ln (was 600); the FFS-60 row now reads A=660, B=1080, C=1560, D=2000, E=2300. This corrected row was verified against the EPUB (`85_Ch12_05.xhtml`) for this pass and matches the printed Exhibit 12-37 exactly.

#### Equations (planning and design, Eqs 12-20 through 12-26)

Cross-checked against `resources/epub/OEBPS/85_Ch12_05.xhtml`; all forms below match the printed equations exactly.

```
Equation 12-20 (planning, DDHV):  V = DDHV = AADT × K × D
  V = DDHV = directional design hour volume, veh/h
  AADT = annual average daily traffic, veh/day (struct field `aadt`, Option<f64>)
  K = proportion of AADT occurring during the peak hour, decimal (struct field `k_factor`, default 0.09; Exhibit 12-18 typical ranges 0.08-0.10 urban, 0.09-0.13 rural)
  D = proportion of peak-hour volume traveling in the peak direction, decimal (struct field `d_factor`, default 0.55; typical value 0.55 for both urban and rural freeways)
Implemented in: basicfreeways/basicfreeways.rs::estimate_ddhv
```

```
Equation 12-21 (design, demand flow rate not per lane):  v = V / (PHF × f_HV)
  v = demand flow rate, pc/h (not per lane — used when the number of lanes is itself unknown)
  V = demand volume under prevailing conditions, veh/h
  PHF = peak hour factor, decimal
  f_HV = heavy vehicle adjustment factor, decimal (Eq 12-10)
Implemented in: basicfreeways/basicfreeways.rs::estimate_number_of_lanes (inlined as the local `demand_flow_rate`, rather than a separate method)
```

```
Equation 12-22 (design, required lanes):  N = v / MSF_i
  N = number of lanes required, ln (always rounded up to the next-higher integer)
  v = demand flow rate, pc/h (Eq 12-21)
  MSF_i = maximum service flow rate for target LOS i, pc/h/ln (Exhibit 12-37 basic freeway / Exhibit 12-38 multilane, keyed on FFS rounded to the nearest 5 mi/h and target LOS, no interpolation permitted per the exhibit note; (FFS, LOS) pairs outside the transcribed rows — including LOS F — return an error naming the reason, see Deviation 4)
Implemented in: basicfreeways/basicfreeways.rs::estimate_number_of_lanes; MSF lookups in determine_basic_max_service_flow_rate / determine_multilane_max_service_flow_rate
```

```
Equation 12-23 (design, Eq 12-21 + Eq 12-22 combined):  N = V / (MSF_i × PHF × f_HV)
  All variables as defined in Eq 12-21 and Eq 12-22
Implemented in: basicfreeways/basicfreeways.rs::estimate_number_of_lanes (the combined computation is inlined rather than exposed as a separate method)
```

```
Equation 12-24 (service flow rate):  SF_i = MSF_i × N × f_HV
  SF_i = service flow rate for LOS i, veh/h (maximum rate that can exist while LOS i is maintained during the 15-min analysis period)
  MSF_i = maximum service flow rate, pc/h/ln (Exhibit 12-37/12-38)
  N = number of lanes, ln (struct field `lane_count`)
  f_HV = heavy vehicle adjustment factor, decimal (struct field `phv`)
Implemented in: basicfreeways/basicfreeways.rs::calculate_service_flow_rate
```

```
Equation 12-25 (service volume):  SV_i = SF_i × PHF
  SV_i = service volume for LOS i, veh/h (maximum hourly volume during the worst 15-min period of the analysis hour)
  SF_i = service flow rate, veh/h (Eq 12-24)
  PHF = peak hour factor, decimal
Implemented in: basicfreeways/basicfreeways.rs::calculate_service_volume
```

```
Equation 12-26 (daily service volume):  DSV_i = SV_i / (K × D) = (MSF_i × N × f_HV × PHF) / (K × D)
  DSV_i = daily service volume for LOS i, veh/day (stated as a total in both directions, unlike SF/SV which are single-direction)
  SV_i = service volume, veh/h (Eq 12-25)
  K = K-factor, decimal (struct field `k_factor`)
  D = D-factor, decimal (struct field `d_factor`)
Implemented in: basicfreeways/basicfreeways.rs::calculate_daily_service_volume
```

## Managed lane segment model (Section 4, Eqs 12-12 to 12-19)

`ManagedLaneSegment` in `src/hcm/basicfreeways/managed_lanes.rs` (renamed from `src/hcm/chapter12/managed_lanes.rs`; also re-exported at `src/hcm/managed_lanes` via `pub use basicfreeways::managed_lanes` in `src/hcm/mod.rs`) implements the five managed-lane types of Exhibit 12-9 (`ContinuousAccess`, `Buffer1`, `Buffer2`, `Barrier1`, `Barrier2`) with per-type calibration parameters from Exhibit 12-30 (`ManagedLaneParams::for_type`: BP_75, λ_BP, c_75, λ_c, A2_55, λ_A2, A1, K_cnf, and optional K_cf, where `k_cf` is `Some` only for ContinuousAccess and Buffer1, the two types with a general-purpose-lane friction effect).

| HCM Eq. | Quantity | Rust method | Notes |
|---|---|---|---|
| 12-12 | `S_ML = S1 − S2 − Ic·S3` | `calculate_speed` | linear portion (v_p ≤ BP) returns S1 only; demand > capacity returns 0.0 sentinel |
| 12-13 | `BP = [BP_75 + λ_BP(75 − FFS_adj)]·CAF²` | `calculate_breakpoint` | **fix (ea74afa)**: CAF is squared, matching the basic-segment breakpoint form; old code scaled by CAF |
| 12-14 | `c_adj = CAF·(c_75 − λ_c(75 − FFS_adj))` | `calculate_capacity` | pc/h/ln |
| 12-15 | `S1 = FFS_adj − A1·min(v_p, BP)` | `calculate_s1` | **fix (ea74afa)**: linear speed drop stops accruing past the breakpoint via `min(v_p, BP)`; old code used raw v_p |
| 12-16 | `A2 = A2_55 + λ_A2(FFS_adj − 55)` | `calculate_a2` | |
| 12-17 | `S2 = (S1,BP − c_adj/K_cnf)·((v_p−BP)/(c_adj−BP))^A2` | `calculate_s2` | returns 0.0 for v_p ≤ BP |
| 12-18 | `Ic = 1 if K_GP > 35 pc/mi/ln else 0` | `calculate_friction_indicator` | forced 0 for Buffer2/Barrier1/Barrier2 regardless of K_GP |
| 12-19 | `S3 = (c_adj/K_cnf − c_adj/K_cf)·((v_p−BP)/(c_adj−BP))²` | `calculate_s3` | **fix (ea74afa)**: leading term is the difference of speeds at capacity without/with friction and the exponent is fixed at 2; old code used `(S1,BP − c_adj/K_cf)·ratio^A2 − S2` |

Density is `calculate_density` (D = v_p/S, sentinel 50.0 when oversaturated), LOS reuses the Exhibit 12-15 thresholds inline in `determine_los` (LOS F when v_p > c_adj), and `run_analysis` chains FFS-adjust → breakpoint → capacity → speed → density → LOS. Exhibit 12-11 estimated lane capacities are separately available via the free function `get_estimated_capacity(lane_type, ffs)` for FFS ∈ {55, 60, 65, 70, 75} mi/h.

#### Equations (Eqs 12-12 through 12-19)

Cross-checked against `resources/epub/OEBPS/84_Ch12_04.xhtml`; all forms below (including the three ea74afa-fixed equations) match the printed forms exactly — no new discrepancies found for this pass. Exhibit 12-30 parameter values below are transcribed from the same source and match `ManagedLaneParams::for_type`.

```
Equation 12-12 (managed lane speed-flow curve):  S_ML = S1                        if v_p <= BP
                                                  S_ML = S1 − S2 − Ic × S3         if BP < v_p <= c_adj
  S_ML = space mean speed of the basic managed lane segment, mi/h
  S1 = speed within the linear portion, mi/h (Eq 12-15)
  S2 = speed drop within the curvilinear portion, mi/h (Eq 12-17)
  S3 = additional speed drop from adjacent GP-lane friction, mi/h (Eq 12-19)
  Ic = friction indicator, 0 or 1 (Eq 12-18)
  BP = breakpoint, pc/h/ln (Eq 12-13)
  v_p = 15-min average flow rate, pc/h/ln (struct field `v_p`)
  c_adj = adjusted capacity, pc/h/ln (Eq 12-14); returns 0.0 (breakdown sentinel) when v_p > c_adj
Implemented in: basicfreeways/managed_lanes.rs::calculate_speed
```

```
Equation 12-13 (breakpoint):  BP = [BP_75 + λ_BP × (75 − FFS_adj)] × CAF²
  BP = breakpoint separating the linear and curvilinear sections, pc/h/ln
  BP_75 = breakpoint at FFS = 75 mi/h, pc/h/ln (Exhibit 12-30: ContinuousAccess 500, Buffer1 600, Buffer2 500, Barrier1 800, Barrier2 700)
  λ_BP = rate of increase in BP per unit decrease in FFS, pc/h/ln (Exhibit 12-30: 0 for ContinuousAccess/Buffer1/Barrier1, 10 for Buffer2, 20 for Barrier2)
  FFS_adj = adjusted free-flow speed, mi/h
  CAF = capacity adjustment factor, decimal (struct field `caf`, default 1.0)
Implemented in: basicfreeways/managed_lanes.rs::calculate_breakpoint (the squared CAF term matches the book exactly; see the ea74afa fix noted in the table above)
```

```
Equation 12-14 (managed lane capacity):  c_adj = CAF × [c_75 − λ_c × (75 − FFS_adj)]
  c_adj = adjusted basic managed lane segment capacity, pc/h/ln
  CAF = capacity adjustment factor, decimal
  c_75 = managed lane capacity at FFS = 75 mi/h, pc/h/ln (Exhibit 12-30: ContinuousAccess 1,800, Buffer1 1,700, Buffer2 1,850, Barrier1 1,750, Barrier2 2,100)
  λ_c = rate of change in capacity per unit change in FFS, pc/h/ln (Exhibit 12-30: 10 for all five types)
  FFS_adj = adjusted free-flow speed, mi/h
Implemented in: basicfreeways/managed_lanes.rs::calculate_capacity
```

```
Equation 12-15 (linear portion speed):  S1 = FFS_adj − A1 × min(v_p, BP)
  S1 = speed within the linear portion of the speed-flow curve, mi/h
  FFS_adj = adjusted free-flow speed, mi/h
  A1 = speed reduction per unit flow in the linear section, mi/h per pc/h/ln (Exhibit 12-30: 0 for ContinuousAccess/Buffer2/Barrier2, 0.0033 for Buffer1, 0.004 for Barrier1)
  v_p = 15-min average flow rate, pc/h/ln
  BP = breakpoint, pc/h/ln (Eq 12-13)
Implemented in: basicfreeways/managed_lanes.rs::calculate_s1 (the min(v_p, BP) clamp is printed in the book's own Eq 12-15 — this is the ea74afa fix noted in the table above, not a new discrepancy)
```

```
Equation 12-16 (curvilinear calibration factor):  A2 = A2_55 + λ_A2 × (FFS_adj − 55)
  A2 = speed reduction per unit flow in the curvilinear section, mi/h
  A2_55 = calibration factor at FFS = 55 mi/h, mi/h (Exhibit 12-30: ContinuousAccess 2.5, Buffer1 1.4, Buffer2 1.5, Barrier1 1.4, Barrier2 1.3)
  λ_A2 = rate of change in A2 per unit increase in FFS (Exhibit 12-30: 0 for ContinuousAccess/Buffer1/Barrier1, 0.02 for Buffer2/Barrier2)
  FFS_adj = adjusted free-flow speed, mi/h
Implemented in: basicfreeways/managed_lanes.rs::calculate_a2
```

```
Equation 12-17 (curvilinear speed drop, K_GP <= 35):  S2 = [(S1,BP − c_adj/K_cnf) / (c_adj − BP)^A2] × (v_p − BP)^A2
  S2 = speed drop within the curvilinear portion, mi/h (0 when v_p <= BP)
  S1,BP = S1 evaluated at v_p = BP, mi/h (Eq 12-15)
  c_adj = adjusted capacity, pc/h/ln (Eq 12-14)
  K_cnf = density at capacity without GP-lane friction, pc/mi/ln (Exhibit 12-30: ContinuousAccess 30, Buffer1 30, Buffer2 45 [average value, footnote a], Barrier1 35, Barrier2 45)
  A2 = curvilinear calibration factor (Eq 12-16)
  BP, v_p as above
Implemented in: basicfreeways/managed_lanes.rs::calculate_s2 (S1,BP via calculate_s1_bp)
```

```
Equation 12-18 (friction indicator):  Ic = 0   if K_GP <= 35 pc/mi/ln, or segment type is Buffer2/Barrier1/Barrier2
                                       Ic = 1   otherwise (ContinuousAccess or Buffer1 with K_GP > 35 pc/mi/ln)
  Ic = friction indicator, 0 or 1
  K_GP = density of the adjacent general purpose lane, pc/mi/ln (struct field `k_gp`)
Implemented in: basicfreeways/managed_lanes.rs::calculate_friction_indicator
```

```
Equation 12-19 (additional speed drop from GP friction):  S3 = [(c_adj/K_cnf) − (c_adj/K_cf)] × [(v_p − BP)/(c_adj − BP)]²
  S3 = additional speed drop within the curvilinear portion, mi/h (0 when v_p <= BP or Ic = 0)
  c_adj = adjusted capacity, pc/h/ln
  K_cnf = density at capacity without friction, pc/mi/ln (Exhibit 12-30, see Eq 12-17)
  K_cf = density at capacity with friction, pc/mi/ln (Exhibit 12-30: ContinuousAccess 45, Buffer1 42 [average value, footnote a]; NA/None for Buffer2, Barrier1, Barrier2 — the only two types with has_friction_effect() == true)
  v_p, BP as above
Implemented in: basicfreeways/managed_lanes.rs::calculate_s3 (fixed exponent of 2 and the leading term as the difference of speeds-at-capacity without/with friction match the book's printed form exactly — the ea74afa fix noted in the table above, not a new discrepancy)
```

## Validation

- **Fixtures**: `tests/ExampleCases/hcm/BasicFreeways/case1.json`, `case2.json`, `case3.json` — per this branch's commit `a2ec7e7`, these correspond to HCM Chapter 26 Example Problems 1-3 (case3 is EP3: six-lane freeway, measured FFS, rolling terrain).
- **Test file**: `tests/chapter12_integration.rs` (renamed from `basicfreeways_test.rs` on this branch). Reproduced published values: FFS {60.8, 67.3, 70.0} mi/h (`determine_free_flow_speed_test`), Eq 12-6 capacities {2308, 2373, 2400} pc/h/ln (`estimate_capacity_test`), Eq 12-9 demand flows {1142, 1694, 1875} pc/h/ln (`estimate_demand_volume_test`), Eq 12-11 densities {18.8, 25.9, 29.0} pc/mi/ln (`estimate_density_test`), LOS {C, C, D} (`determine_segment_los`), and Eq 12-1 speeds {60.8, 65.4, 64.7} mi/h (`estimate_speed_test`). `estimate_number_of_lanes` asserts {2, 3, 3} at a LOS D target — only case 2's is a published figure (EP2), the other two are derived from each case's own Exhibit 12-37 row; the previous {0, 3, 0} expectation skipped cases 1 and 3 entirely because their fixtures carry v_p = 0. `max_service_flow_rate_domain` covers the nearest-5 rounding and the three error paths.
- **Tolerances**: exact `assert_eq!` after rounding to the published precision (`math::round_up_to_n_decimal(_, 1)` for FFS/density, `_, 0` for capacity/flows) for all tests except `estimate_speed_test`, which uses an absolute tolerance of 0.1 mi/h.
- **Python**: `tests/test_chapter12_integration.py` covers Example Problems 1-3 through the PyO3 `BasicFreeways` class (operational path, stepwise-vs-orchestrator equivalence, general-terrain and specific-upgrade PCEs, interpolation, off-domain errors, and the EP2 design analysis). Note that EP2 is a design problem: its published operational results describe the 3-lane solution, so the Exhibit 12-37 lane-count step must run before demand is converted to a per-lane rate.
- **PCE tables**: `tests/test_pce_table_epub.py` regenerates `pce_table.rs` from the EPUB and asserts the committed file matches byte for byte, and separately asserts the three exhibits are distinct at every truck percentage and that all eight printed grades survive. It skips when `resources/` is absent, which is the CI case (gitignored, copyrighted), so it guards local edits rather than acting as an enforced gate.
- **Managed lanes**: only three inline unit tests exist (`managed_lanes.rs` `#[cfg(test)]`): Exhibit 12-30 parameter spot-check for ContinuousAccess, friction-effect eligibility for Barrier2, and friction activation at K_GP = 40. No HCM worked example reproduces the managed-lane speed model on this branch.
- No `docs/hcm/VERIFICATION.md` exists on this branch to cross-reference; deviations are listed inline below.

## Deviations

1. **Mountainous-terrain PCE (VERIFY-HCM, `basicfreeways.rs` in `adjustment_heavy_vehicle_factor`)**: Exhibit 12-25 provides no PCE for mountainous terrain (HCM directs analysts to the Chapter 25/26 mixed-flow model); the `E_T = 2.5` used here is a flagged non-HCM approximation retained for API stability.
2. **PCE lookup domain** (rewritten on `fix/hcm-ch12-pce-tables`): the `todo!()` panics and the silent `e_t = None` miss are gone. `PceTable::lookup` interpolates within the exhibit and returns `Err` outside it — grades above 6% (the mixed-flow model's territory), SUT mixes other than 30/50/70, and unknown terrain strings. Two clamps are flagged VERIFY-HCM in the code rather than erroring: downgrades steeper than −2% read the −2% row (identical to the 0% row in all three exhibits, so the tables show no downgrade sensitivity), and lengths past the longest tabulated row carry that row forward (the 1.25 and 1.5 mi values differ by at most 0.01).

   The bug this replaced was much larger than the panic. All three `ET_TABLE_*` maps were byte-identical and held (mostly) Exhibit 12-28, so a 30% or 50% SUT analysis silently used 70%-SUT equivalents; 1 of 192 entries in the nominal 30% table matched Exhibit 12-26, one value (grade 2.0, 1.25 mi, 4% trucks = 3.42) matched no exhibit at all, and grades 3.5-6% were absent from every path despite being printed rows. `tests/test_pce_table_epub.py` now re-derives the module from the EPUB and diffs it byte for byte.
3. **`length == 0.125` fallback** (removed on `fix/hcm-ch12-pce-tables`): the block unconditionally overwrote `e_t` from an Exhibit 12-26 row whenever segment length was exactly 0.125 mi, clobbering the terrain- or mix-derived value. It was also only ever right for the 30% mix — Exhibit 12-27's 0.125 mi row differs, and Exhibit 12-28's varies by grade from 2.39 to 3.51.
4. **MSF table domain and rounding** (fixed on `fix/hcm-ch12-pce-tables`): both `determine_*_max_service_flow_rate` functions used to return a silent 2,000 pc/h/ln for any (FFS, LOS) pair outside the exhibit, including LOS F and unset LOS; they now return an error naming the reason. They also rounded FFS **up** to the nearest 5 mi/h, while Exhibit 12-37's instructions say "the FFS should be rounded to the nearest 5 mi/h, and no interpolation is permitted" — `math::round_to_nearest_5` now implements the printed rule, so FFS 61 reads the 60 row rather than the 65 row. The transcribed table values themselves were verified correct against the EPUB for both exhibits.
5. **Oversaturation sentinels**: `calculate_speed` returns 0.0 and `estimate_density` returns `DENSITY_AT_CAPACITY + 1.0` (46.0) when demand exceeds capacity; `ManagedLaneSegment::calculate_density` uses 50.0. These sentinel conventions are internally consistent but not HCM values, and Chapter 12 itself says density is undefined there (Chapter 10 methodology required).
6. **Default BFFS** (fixed on `fix/hcm-ch12-pce-tables`): `DEFAULT_BFFS_FREEWAY = 75.4` was declared but never read. `DefaultValues` now carries `base_ffs`, which `apply_defaults` writes into `bffs`: the two freeway constructors use 75.4 ("a default base FFS of 75.4 mi/h, which resulted in the most accurate predictions in the underlying research"), and the three multilane constructors leave it `None`, because Chapter 12 gives multilane highways no single default and instead prescribes speed limit + 5 mi/h at limits of 50 mi/h and above, or + 7 mi/h below that — implemented as `bffs_from_speed_limit`. The four `with_*_defaults` constructors delegate to `apply_defaults` instead of duplicating it, which is what carries `base_ffs` through. A bare `BasicFreeways::new()` still starts at 65.0.

7. **`sut_percentage` default** (fixed on `fix/hcm-ch12-pce-tables`): `new()` defaulted it to 50, so every default-constructed segment took the specific-upgrade path and, with `p_t = None`, panicked on an `unwrap`. It now defaults to 0, which selects the Exhibit 12-25 general-terrain PCE and matches what all three fixtures already set.

## Deferred

- Driver population adjustments and work-zone/weather CAF/SAF derivation are inputs (`saf`/`caf` fields) rather than computed here; the Chapter 11 factor tables live in `src/hcm/common/adjustment_factors.rs`.
- The Chapter 25/26 mixed-flow model for high truck percentages / mountainous terrain (the correct treatment behind deviation 1).
- Managed-lane worked-example validation (no Chapter 26 example reproduces Eqs 12-12..12-19 in the test suite).
- Bicycle LOS for multilane highways (Chapter 15 Section 4 covers it; not wired to `BasicFreeways`).
