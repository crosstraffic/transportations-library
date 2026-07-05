# Chapter 12: Basic Freeway and Multilane Highway Segments — Procedure Walkthrough

HCM 7th Edition Chapter 12 covers basic freeway and multilane highway segments (Sections 2-3, operational core in Exhibit 12-19's six steps) plus the basic managed lane segment methodology (Section 4, Equations 12-12 through 12-19). Both are implemented under `src/hcm/chapter12/` on the `feat/hcm-ch12-14-completion` branch: `basicfreeways.rs` (the `BasicFreeways` struct handling both basic freeway and multilane highway variants via the `highway_type` string field, `"basic"` vs `"multilane"`) and `managed_lanes.rs` (the `ManagedLaneSegment` struct with per-type parameters from Exhibit 12-30). This branch's commit `ea74afa` corrected three managed-lane equations and one Exhibit 12-37 table entry against the manual (detailed below). Python bindings for both structs are registered in `src/copython/chapter12.rs` (`BasicFreeways`, `ManagedLanes` pyclasses).

## Operational analysis walkthrough (Exhibit 12-19 steps)

The full sequence is orchestrated by `BasicFreeways::run_operational_analysis()`, which chains the steps below and returns the LOS.

| Manual step | HCM Eq./Exhibit | Rust method | File | Inputs (units) | Output (units) |
|---|---|---|---|---|---|
| Step 1: input data | Exhibit 12-18 defaults | `DefaultValues::{urban_freeway, rural_freeway, urban_multilane, rural_multilane, suburban_multilane_high_density}` + `BasicFreeways::with_*_defaults()` / `apply_defaults()` | `basicfreeways.rs` | — | populated struct |
| Step 2: FFS estimation (freeway) | Eq 12-2 | private `estimate_basic_lane_ffs` via `determine_free_flow_speed` | `basicfreeways.rs` | `bffs` (mi/h), lane width `lw` (ft), right lateral clearance `lc_r` (ft), total ramp density `trd` (ramps/mi) | `ffs` mi/h; `FFS = BFFS − f_LW − f_RLC − 3.22·TRD^0.84` |
| Step 2: FFS estimation (multilane) | Eq 12-3, Eq 12-4 (TLC) | private `estimate_multi_lane_ffs`; `calculate_total_lateral_clearance` | `basicfreeways.rs` | `bffs`, `lw`, `lc_r`+`lc_l` (ft, each capped 6 ft), median type, access point density `apd` (pts/mi) | `ffs` mi/h; `FFS = BFFS − f_LW − f_TLC − f_M − f_A` |
| Step 2 adjustments | Exhibit 12-20 (lane width), Exhibit 12-21 (right clearance), Exhibit 12-22 (TLC, with linear interpolation), Exhibit 12-24 (access points) | `adjustment_average_lane_width`, `adjustment_right_side_lateral_clearance` (+ `_interpolated` variant), `adjustment_total_lateral_clearance`, `adjustment_median_type`, `adjustment_access_point_density` (formula `min(0.25·APD, 10)`; a table-lookup variant `adjustment_access_point_density_table` exists but is `#[allow(dead_code)]`) | `basicfreeways.rs` | as above | mi/h reductions |
| Step 2: adjusted FFS | Eq 12-5 | `determine_free_flow_speed` (tail) | `basicfreeways.rs` | `saf` (unitless) | `ffs_adj = ffs × SAF`, mi/h |
| Step 3: capacity | Eq 12-6 (basic, `2200 + 10(FFS−50)`, max 2,400), Eq 12-7 (multilane, `1900 + 20(FFS−45)`, max 2,300), Eq 12-8 (`c_adj = c × CAF`) | `estimate_capacity`, `estimate_adjusted_capacity` | `basicfreeways.rs` | `ffs_adj` (mi/h), `caf` | pc/h/ln |
| Step 4: demand adjustment | Eq 12-9 (`v_p = V/(PHF·N·f_HV)`), Eq 12-10 (`f_HV = 1/(1+P_T(E_T−1))`), Exhibit 12-25/12-26 PCE | `estimate_demand_volume`, private `adjustment_heavy_vehicle_factor` (uses `ET_TABLE_30SUT/50SUT/70SUT` from `src/hcm/common/pce_table.rs` keyed by SUT mix) | `basicfreeways.rs` | `demand_flow_i` (veh/h), `phf`, `lane_count`, `p_t` (decimal), terrain, grade (%), length (mi) | `v_p` pc/h/ln |
| Step 5: speed-flow curve | Eq 12-1 with Exhibit 12-6 parameters (breakpoint `BP = [1000 + 40(75 − FFS_adj)]·CAF²` basic; BP = 1,400 constant multilane; exponent a = 2.0 basic / 1.31 multilane; density at capacity 45 pc/mi/ln) | `calculate_breakpoint`, `calculate_speed` | `basicfreeways.rs` | `v_p`, `ffs_adj`, `capacity_adj` | space mean speed S, mi/h (0.0 sentinel when demand > capacity) |
| Step 5: density | Eq 12-11 (`D = v_p/S`) | `estimate_density` | `basicfreeways.rs` | `v_p` (pc/h/ln), S (mi/h) | pc/mi/ln (sentinel 46.0 when oversaturated) |
| Step 6: LOS | Exhibit 12-15 | `determine_segment_los` (v/c check then delegates to `common::FacilityCalculation::los_from_density`) | `basicfreeways.rs` | density, v/c | `LevelOfService` A-F |

### Planning and design analysis

Planning-level entry points: `estimate_ddhv` (Eq 12-20, `DDHV = AADT × K × D`), `estimate_number_of_lanes` (Eqs 12-21/12-22/12-23, `N = ceil(v/MSF_i)`), `estimate_lanes_from_aadt` (chains 12-20 into 12-23), `determine_basic_max_service_flow_rate` / `determine_multilane_max_service_flow_rate` (Exhibit 12-37/12-38 MSF tables keyed on FFS rounded up to the nearest 5 mi/h and target LOS), `calculate_service_flow_rate` (Eq 12-24), `calculate_service_volume` (Eq 12-25), and `calculate_daily_service_volume` (Eq 12-26). Commit `ea74afa` corrected the Exhibit 12-37 FFS-60/LOS-A entry to 660 pc/h/ln (was 600); the FFS-60 row now reads A=660, B=1080, C=1560, D=2000, E=2300.

## Managed lane segment model (Section 4, Eqs 12-12 to 12-19)

`ManagedLaneSegment` in `src/hcm/chapter12/managed_lanes.rs` implements the five managed-lane types of Exhibit 12-9 (`ContinuousAccess`, `Buffer1`, `Buffer2`, `Barrier1`, `Barrier2`) with per-type calibration parameters from Exhibit 12-30 (`ManagedLaneParams::for_type`: BP_75, λ_BP, c_75, λ_c, A2_55, λ_A2, A1, K_cnf, and optional K_cf, where `k_cf` is `Some` only for ContinuousAccess and Buffer1, the two types with a general-purpose-lane friction effect).

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

## Validation

- **Fixtures**: `tests/ExampleCases/hcm/BasicFreeways/case1.json`, `case2.json`, `case3.json` — per this branch's commit `a2ec7e7`, these correspond to HCM Chapter 26 Example Problems 1-3 (case3 is EP3: six-lane freeway, measured FFS, rolling terrain).
- **Test file**: `tests/chapter12_integration.rs` (renamed from `basicfreeways_test.rs` on this branch). Reproduced published values: FFS {60.8, 67.3, 70.0} mi/h (`determine_free_flow_speed_test`), Eq 12-6 capacities {2308, 2373, 2400} pc/h/ln (`estimate_capacity_test`), Eq 12-9 demand flows {1142, 1694, 1875} pc/h/ln (`estimate_demand_volume_test`), Eq 12-11 densities {18.8, 25.9, 29.0} pc/mi/ln (`estimate_density_test`), LOS {C, C, D} (`determine_segment_los`), required lanes {0, 3, 0} (`estimate_number_of_lanes`), and Eq 12-1 speeds {60.8, 65.4, 64.7} mi/h (`estimate_speed_test`).
- **Tolerances**: exact `assert_eq!` after rounding to the published precision (`math::round_up_to_n_decimal(_, 1)` for FFS/density, `_, 0` for capacity/flows) for all tests except `estimate_speed_test`, which uses an absolute tolerance of 0.1 mi/h.
- **Managed lanes**: only three inline unit tests exist (`managed_lanes.rs` `#[cfg(test)]`): Exhibit 12-30 parameter spot-check for ContinuousAccess, friction-effect eligibility for Barrier2, and friction activation at K_GP = 40. No HCM worked example reproduces the managed-lane speed model on this branch.
- No `docs/hcm/VERIFICATION.md` exists on this branch to cross-reference; deviations are listed inline below.

## Deviations

1. **Mountainous-terrain PCE (VERIFY-HCM, `basicfreeways.rs` in `adjustment_heavy_vehicle_factor`)**: Exhibit 12-25 provides no PCE for mountainous terrain (HCM directs analysts to the Chapter 25/26 mixed-flow model); the `E_T = 2.5` used here is a flagged non-HCM approximation retained for API stability.
2. **PCE lookup panics on unhandled inputs**: the `p_t >= 0.25` branches of `adjustment_heavy_vehicle_factor` end in `todo!("Unhandled grade/length combination")` — any grade/length pair outside the hand-transcribed set {−2.0, 0.0, 2.0, 2.5} % × {0.125 … 1.5} mi panics at runtime rather than interpolating or erroring gracefully. The sub-0.25 branch's HashMap lookup (`ET_TABLE_*SUT` keyed on `(p_t×100, length×1000, grade×100)` as integers) silently yields `e_t = None` (then treated as 0.0 in the f_HV formula, which produces f_HV > 1) on a key miss. Both are reliability hazards for off-grid inputs.
3. **`length == 0.125` fallback overrides terrain/table PCE**: the final `if self.length == 0.125` block in `adjustment_heavy_vehicle_factor` unconditionally overwrites `e_t` from a small `p_t`-keyed table whenever segment length is exactly 0.125 mi, clobbering any terrain- or SUT-mix-derived value computed above it.
4. **MSF table fallback**: both `determine_*_max_service_flow_rate` functions return a silent default of 2000.0 pc/h/ln for any (FFS, LOS) combination outside the transcribed exhibit rows (including LOS F or unset LOS), with no warning.
5. **Oversaturation sentinels**: `calculate_speed` returns 0.0 and `estimate_density` returns `DENSITY_AT_CAPACITY + 1.0` (46.0) when demand exceeds capacity; `ManagedLaneSegment::calculate_density` uses 50.0. These sentinel conventions are internally consistent but not HCM values, and Chapter 12 itself says density is undefined there (Chapter 10 methodology required).
6. **Eq 12-41/12-42-style multilane FFS default BFFS**: `DEFAULT_BFFS_FREEWAY = 75.4` is declared (Step 2 default when FFS is not field-measured) but nothing in `basicfreeways.rs` reads it; `bffs` defaults to 65.0 in `BasicFreeways::new()`. A caller relying on the HCM default BFFS must set it manually.

## Deferred

- Driver population adjustments and work-zone/weather CAF/SAF derivation are inputs (`saf`/`caf` fields) rather than computed here; the Chapter 11 factor tables live in `src/hcm/common/adjustment_factors.rs`.
- The Chapter 25/26 mixed-flow model for high truck percentages / mountainous terrain (the correct treatment behind deviation 1).
- Managed-lane worked-example validation (no Chapter 26 example reproduces Eqs 12-12..12-19 in the test suite).
- Bicycle LOS for multilane highways (Chapter 15 Section 4 covers it; not wired to `BasicFreeways`).
