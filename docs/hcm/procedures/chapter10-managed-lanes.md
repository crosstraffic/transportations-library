# HCM Chapter 10: Managed-Lane Facilities and the Chapter 25 Planning-Level Method

This document walks a reviewer through two extensions to the Chapter 10 core freeway facility methodology implemented on this branch. The first is the managed-lane (ML) facility extension of HCM 7th Edition Chapter 10 Section 4 / Chapter 25 Section 2 (Steps A-9, A-13, A-14, A-17): the lane-group concept pairing each general-purpose (GP) analysis segment with an optional parallel ML segment, the Step A-9 cross-weave friction effect on GP capacity (Chapter 13 Equations 13-24/13-25), the Step A-13 adjacent-friction speed effect on the ML (Chapter 12 Equations 12-18/12-19, via the Chapter 12 ML segment engine), and lane-group plus combined facility aggregation. The second is the Chapter 25 Section 6 planning-level methodology, a section-based screening method that runs from directional AADT and a K-factor over four fixed 15-min analysis periods (Equations 25-40 through 25-52, Exhibits 25-15 through 25-17). The code lives in `src/hcm/freeway_facilities/managed_lanes.rs` and `src/hcm/freeway_facilities/planning.rs`; both build on the core engine in `src/hcm/freeway_facilities/freeway_facilities.rs` (documented in `docs/hcm/procedures/chapter10.md`). Book discrepancies found during implementation are catalogued in `docs/hcm/VERIFICATION.md` under "Chapter 10/25 managed lanes + planning (feat/hcm-ch10-managed-lanes)" (items 1-7); this walkthrough cites those items by number rather than restating them.

## Part 1: Managed-lane facilities (`managed_lanes.rs`)

### Data model and orchestration

`ManagedLaneFacility` holds the GP lane group as a full `FreewayFacility` (`gp`), a parallel vector `ml: Vec<Option<MlSegmentInput>>` where `ml[i]` pairs with GP segment `i` (`None` where no adjacent managed lane exists), an ML entry demand vector by period (`ml_entry_demand`, veh/h), a facility ML free-flow speed (`ml_ffs`, mi/h, per-segment override via `MlSegmentInput::ffs`), and per-GP-segment cross-weave data (`cross_weave: Vec<Option<CrossWeave>>`). `MlSegmentInput` carries the ML separation type (`ManagedLaneType` from the Chapter 12 module, per Exhibit 12-9), lane count, calibration CAF/SAF, and ML on-/off-ramp demand vectors (veh/h by period). `ManagedLaneFacility::run_analysis` executes, in order: input-length validation, `apply_cross_weave` (Step A-9), the full GP `FreewayFacility::run_analysis` (Steps A-1 through A-17 for the GP group, including the oversaturated engine if triggered), `compute_ml_demands` and `compute_ml_capacities` plus d/c ratios, `evaluate_ml_segments` (Steps A-11/A-13 for the ML group), and `aggregate_performance` (Steps A-14/A-17).

### Step A-9: cross-weave friction on GP capacity

| Item | Equations | Rust location |
|---|---|---|
| Cross-weave capacity reduction factor CRF | Eq 13-24 | `freeway_facilities/managed_lanes.rs::cross_weave_crf` |
| Cross-weave CAF = 1 - CRF | Eq 13-25 | `freeway_facilities/managed_lanes.rs::cross_weave_caf` |
| Application to GP segment capacity | Step A-9 | `freeway_facilities/managed_lanes.rs::ManagedLaneFacility::apply_cross_weave`, `CrossWeave::caf` |

`cross_weave_crf(cw_demand_pc, l_cw_min_ft, n_gp_lanes)` implements Equation 13-24 exactly as printed (`CRF = -0.0897 + 0.0252 ln(CW) - 0.00001453 L_cw-min + 0.002967 N_GP`), with inputs cross-weave demand CW in pc/h, minimum cross-weave length in ft, and GP lane count; the result is clamped to `[0, 1]` and zero demand short-circuits to zero reduction. `apply_cross_weave` folds `base_caf * cross_weave_caf(...)` into the affected GP segment's `caf_schedule` per period (the cross-weave demand is a per-period vector, so a schedule rather than a scalar CAF is installed) before the GP analysis runs, so the reduction flows through `FreewayFacility::compute_capacities` unmodified. Note that no published HCM example exercises the cross-weave CAF (Example Problem 5 has none); coverage is equation-level unit testing plus a capacity-reduction integration check only — VERIFICATION.md item 7. Note also that the cross-weave equations live entirely in `freeway_facilities/managed_lanes.rs`; `weaving/weaving.rs` (the Chapter 13 basic weaving-segment engine) does not implement or reference Eq 13-24/13-25 at all — confirmed by inspection, no cross-weave symbols appear there.

```
Equation 13-24:  CRF = -0.0897 + 0.0252*ln(CW) - 0.00001453*L_cw-min + 0.002967*N_GP     [decimal, clamped to 0..1]
  CRF        = capacity reduction factor                                                 [decimal]
  CW         = cross-weave demand flow rate (GP ramp flow crossing to/from the ML access) [pc/h]
  L_cw-min   = minimum cross-weave length (gore to start of the ML access opening)        [ft]
  N_GP       = number of general-purpose lanes crossed                                    [ln]
  Non-positive CW short-circuits to CRF = 0 (no reduction); result otherwise clamped to [0, 1].
Implemented in: freeway_facilities/managed_lanes.rs::cross_weave_crf

Equation 13-25:  CAF = 1 - CRF                        [decimal]
                 c_GPA = c_GP x CAF                    [veh/h]
  CAF   = capacity adjustment factor applied to the GP segment                            [decimal]
  c_GPA = adjusted capacity of the general-purpose lanes                                  [veh/h]
  c_GP  = unadjusted GP capacity (Chapter 12 basic freeway procedures)                     [veh/h]
Implemented in: freeway_facilities/managed_lanes.rs::cross_weave_caf (CAF); ManagedLaneFacility::apply_cross_weave folds base_caf * CAF into the GP segment's per-period caf_schedule, so c_GPA is realized inside FreewayFacility::compute_capacities using the existing GP capacity engine as c_GP.
```

### ML demand and capacity

`compute_ml_demands` mirrors the GP Step A-4 accumulation: it walks segments upstream to downstream carrying `ml_entry_demand[p]`, adding `on_ramp_demand[p]` and subtracting `off_ramp_demand[p]` at each segment that has an `MlSegmentInput`. Units are veh/h. `compute_ml_capacities` instantiates the Chapter 12 `ManagedLaneSegment` engine (from `src/hcm/basicfreeways/managed_lanes.rs`) per ML segment, applies the calibration CAF/SAF, calls `calculate_ffs_adj()` and `calculate_capacity()` to get the adjusted per-lane capacity in pc/h/ln, and converts to veh/h as `cap_pc * lanes * f_HV` using the GP facility's heavy-vehicle factor (the ML shares the facility f_HV; there is no separate ML heavy-vehicle percentage input). ML capacity is constant across periods for a given segment.

```
ML demand accumulation (no HCM equation number; mirrors the Ch10 Step A-4 GP demand-accumulation logic for the ML lane group):
  ml_demand[i][p] = ml_demand[i-1][p] + on_ramp_demand[i][p] - off_ramp_demand[i][p]     [veh/h]
  with ml_demand[-1][p] := ml_entry_demand[p]                                             [veh/h]
  ml_entry_demand[p]    = ML volume entering the facility upstream of segment 0, period p  [veh/h]
  on_ramp_demand[i][p]  = ML on-ramp (merge/access) demand added at segment i, period p    [veh/h]
  off_ramp_demand[i][p] = ML off-ramp (diverge/access) demand removed at segment i, period p [veh/h]
Implemented in: freeway_facilities/managed_lanes.rs::ManagedLaneFacility::compute_ml_demands

ML segment capacity (Chapter 12 Eq 12-14 c_adj, converted to veh/h; no separate Ch10/25 equation number):
  cap_pc  = CAF * (c_75 - lambda_c * (75 - FFS_adj))          [pc/h/ln]   (Chapter 12 Equation 12-14, via ManagedLaneSegment::calculate_capacity)
  cap_veh = cap_pc * lanes * f_HV                              [veh/h]
  CAF        = ML calibration capacity adjustment factor                  [decimal]
  c_75       = base capacity at FFS = 75 mi/h (Exhibit 12-30, by lane type) [pc/h/ln]
  lambda_c   = rate of change in capacity per unit change in FFS (Exhibit 12-30) [pc/h/ln per mi/h]
  FFS_adj    = FFS * SAF (ML free-flow speed adjusted by the calibration speed factor) [mi/h]
  lanes      = number of managed lanes in the segment                     [ln]
  f_HV       = GP facility heavy-vehicle adjustment factor (shared with the ML; no separate ML truck %) [decimal]
Implemented in: freeway_facilities/managed_lanes.rs::ManagedLaneFacility::compute_ml_capacities (drives basicfreeways/managed_lanes.rs::ManagedLaneSegment::calculate_capacity for cap_pc)

ML demand-to-capacity ratio (no HCM equation number, analogous to the GP d/c):
  ml_dc_ratio[i][p] = ml_demand[i][p] / ml_capacity[i][p]        [decimal] (0 if capacity is 0)
Implemented in: freeway_facilities/managed_lanes.rs::ManagedLaneFacility::run_analysis (inline)
```

### Steps A-11/A-13: ML segment evaluation with adjacent friction

`evaluate_ml_segments` evaluates each ML segment per period through `ml_engine`, which converts the served ML volume (assumed equal to demand — see the deferral note on oversaturated ML operation below) to per-lane passenger-car flow `v_p = volume / (lanes * f_HV * PHF)` (pc/h/ln) and hands the Chapter 12 engine the adjacent GP segment's density in pc/mi/ln (`self.gp.density_pc[i][p]`) via `set_gp_density`. The Chapter 12 engine internally switches on the Equation 12-18/12-19 friction speed drop when that density exceeds 35 pc/mi/ln (`ADJACENT_FRICTION_THRESHOLD_PC`, declared in `managed_lanes.rs`); the boolean reported in `ml_friction_active[i][p]` additionally requires the lane type to be friction-capable (`ContinuousAccess` or `Buffer1` only, per Exhibit 12-9 — barrier-separated types never experience adjacent friction). Outputs per cell are `ml_speed` (mi/h), `ml_density_pc` (pc/mi/ln, direct from the engine), `ml_density_veh` (= density_pc x f_HV, veh/mi/ln), and `ml_los` (density-based, Exhibit 12-15 thresholds, from the Chapter 12 engine; `None` maps to F). One published cell is not reproducible — Example Problem 5 Segment 10 / Period 2 prints 58.1 mi/h while the adjacent GP density (34.2 pc/mi/ln) is below the 35 threshold, so the implementation computes the friction-free 58.9 mi/h; VERIFICATION.md item 2 documents this, and the `VERIFY-HCM` comment sits directly in `evaluate_ml_segments`.

```
Equation 12-18:  I_c = 0  if K_GP <= 35 pc/mi/ln, or segment type is Buffer2/Barrier1/Barrier2
                 I_c = 1  otherwise                                                       [indicator, 0 or 1]
  I_c   = friction indicator (whether the adjacent-friction speed drop applies)            [0 or 1]
  K_GP  = density of the adjacent general-purpose lane                                     [pc/mi/ln]
  Threshold constant declared as ADJACENT_FRICTION_THRESHOLD_PC = 35.0 pc/mi/ln.
Implemented in: freeway_facilities/managed_lanes.rs::ADJACENT_FRICTION_THRESHOLD_PC (the 35 pc/mi/ln threshold, cited in Step A-13); basicfreeways/managed_lanes.rs::ManagedLaneSegment::calculate_friction_indicator (the I_c switch, restricted to ContinuousAccess/Buffer1); the reported flag is freeway_facilities/managed_lanes.rs::ManagedLaneFacility::evaluate_ml_segments (ml_friction_active[i][p])

Equation 12-19:  S3 = [(c_adj / K_cnf) - (c_adj / K_cf)] * ((v_p - BP) / (c_adj - BP))^2   [mi/h, additional speed drop]
  S3     = additional speed drop in the curvilinear portion due to GP-lane friction         [mi/h]
  c_adj  = adjusted basic ML segment capacity (Chapter 12 Equation 12-14)                   [pc/h/ln]
  K_cnf  = density at capacity without the GP-friction effect (Exhibit 12-30, by lane type)  [pc/mi/ln]
  K_cf   = density at capacity with the GP-friction effect (Exhibit 12-30; ContinuousAccess/Buffer1 only) [pc/mi/ln]
  v_p    = 15-min average ML flow rate                                                      [pc/h/ln]
  BP     = breakpoint in the ML speed-flow curve (Chapter 12 Equation 12-13)                 [pc/h/ln]
  S3 = 0 when v_p <= BP or I_c = 0 (no friction, or demand still in the linear portion).
Implemented in: basicfreeways/managed_lanes.rs::ManagedLaneSegment::calculate_s3; applied as S_ML = S1 - S2 - I_c*S3 (Equation 12-12) in ::calculate_speed, called from freeway_facilities/managed_lanes.rs::ManagedLaneFacility::evaluate_ml_segments via ml_engine()/ManagedLaneSegment::run_analysis. v_p is computed there as v_p = volume_veh / (lanes * f_HV * PHF), and K_GP is set via set_gp_density(self.gp.density_pc[i][p]) — the paired GP segment's Step A-13 density input.
```

### Steps A-14/A-17: lane-group and combined aggregation

`aggregate_performance` computes three parallel result sets per period. The GP lane group (`gp_group_performance`) and ML lane group (`ml_group_performance`) each get `LaneGroupPerformance` records — space mean speed (Equation 25-2, `exhibits::facility_space_mean_speed`), average density in veh/mi/ln and pc/mi/ln (Equation 10-1, `exhibits::facility_density`, length-and-lane weighted), and LOS (Exhibit 10-6 via `los_freeway_facility`, forced to F if any of that group's segments has vd/c > 1.0). The ML group aggregates only over segments carrying a managed lane. The combined facility (`facility_performance`, a `PeriodPerformance` shared with the core module) concatenates both groups' flow/length/speed/density/lane vectors and applies the same Equation 10-1/25-2 formulas, plus VMT/VHT/VHD summed across both groups at 0.25-h period duration (ML volume served is taken as ML demand, so ML `vmt_served == vmt_demand` per cell). The combined density carries the VERIFY-HCM flag in `aggregate_performance`: the lane-mile-weighted Equation 10-1 combination of the book's own Exhibit 25-86 group densities gives 28.3 veh/mi/ln for Example Problem 5 Period 3, but Exhibit 25-87 prints 29.1, which is not reproducible from the book's own inputs — VERIFICATION.md item 1 (LOS unaffected).

```
Equation 10-1:  D_F = Sum_i(D_i * L_i * N_i) / Sum_i(L_i * N_i)      [pc/mi/ln or veh/mi/ln, matching the D_i unit]
  D_F  = average density for the facility (or lane group) in one 15-min analysis period      [pc/mi/ln]
  D_i  = density of segment i                                                                 [pc/mi/ln]
  L_i  = length of segment i                                                                  [mi]
  N_i  = number of lanes in segment i                                                         [ln]
  n    = number of segments in the group being aggregated
Implemented in: freeway_facilities/exhibits.rs::facility_density (length- and lane-weighted average, called once per lane group and once for the combined facility in aggregate_performance)

Equation 25-2:  SMS(NS,p) = Sum_i(SF(i,p)*L(i)) / Sum_i(SF(i,p)*L(i)/U(i,p))     [mi/h]
  SMS(NS,p) = facility (or lane group) space mean speed in analysis period p                  [mi/h]
  SF(i,p)   = flow rate on segment i in period p                                              [veh/h]
  L(i)      = length of segment i                                                              [mi]
  U(i,p)    = space mean speed of segment i in period p                                        [mi/h]
  NS        = number of segments in the group being aggregated
  Segments with U(i,p) <= 0 are excluded from the denominator sum (facility_space_mean_speed filters s > 0.0).
Implemented in: freeway_facilities/exhibits.rs::facility_space_mean_speed (called for the GP group, ML group, and combined facility in aggregate_performance)
```

## Part 2: Planning-level method (`planning.rs`)

### Structure and step order

`PlanningFacility` takes ordered `PlanningSection`s (type Basic/Weave/Ramp per Exhibit 25-15; length in mi; lanes; `inflow_aadt`/`outflow_aadt` in veh/day at the section's upstream boundary; weaving volume ratio `weave_vr` for weave sections; optional `caf_override`), a facility FFS (mi/h), K-factor, growth factor, PHF, SUT/TT percentages (decimal), terrain, and city type. `run_analysis` executes the whole method; results are `section_results[section][period]` (`PlanningSectionResult`) and `facility_results[period]` (`PlanningFacilityResult`). The analysis is fixed at `NUM_PLANNING_PERIODS = 4` 15-min periods with demand multipliers `[1, 1/PHF, 1, 2 - 1/PHF]` (Equation 25-40, `period_multipliers`).

| HCM item | Equations / Exhibits | Rust location | Units |
|---|---|---|---|
| Period demand flow rates from AADT | Eq 25-40 | `freeway_facilities/planning.rs::PlanningFacility::run_analysis` (`boundary`, `mult`) | AADT veh/day -> pc/h via `k_factor * growth_factor * f_HV` |
| Heavy-vehicle factor | Eq 25-42 | `freeway_facilities/planning.rs::PlanningFacility::f_hv` | decimal; uses `Terrain::pce()` (Exhibit 12-25) |
| Demand accumulation with vertical-queue carryover | Eq 25-43/25-44 | `freeway_facilities/planning.rs::PlanningFacility::run_analysis` (`demand`, `carryover`, `next_carryover`) | pc/h; carryover in pc |
| Basic section capacity | Eq 25-45 | `freeway_facilities/planning.rs::basic_section_capacity_pc` | pc/h/ln; `2200 + 10 (min(70, FFS) - 50)` |
| Weaving-section CAF | Eq 25-46 | `freeway_facilities/planning.rs::weave_caf` | decimal; `min(0.884 - 0.0752 V_r + 0.0000243 L_s(ft), 1)` |
| Ramp-section CAF | Ch 25 text default | `freeway_facilities/planning.rs::DEFAULT_RAMP_CAF` (0.9) | decimal |
| Undersaturated delay rate | Eq 25-47 + Exhibit 25-16 | `freeway_facilities/planning.rs::undersaturated_delay_rate`, `undersaturated_params` | s/mi (see VERIFICATION.md item 4 on the printed "min/mi" label) |
| Oversaturated delay rate | Eq 25-48 | `freeway_facilities/planning.rs::oversaturated_delay_rate` | s/mi; public helper, unused in reported results (VERIFICATION.md item 3) |
| Travel rate / time / speed / density | Eq 25-49 through 25-52 | `freeway_facilities/planning.rs::PlanningFacility::run_analysis` (inline) | s/mi, s, mi/h, pc/mi/ln |
| Facility aggregation and LOS | Exhibits 25-96 / 25-17 | `freeway_facilities/planning.rs::PlanningFacility::aggregate_facility` | length-weighted density (VERIFICATION.md item 5) |

```
Equation 25-40:  V_i,t = AADT_i * k * f_tg                       for t = 1, 3     [veh/h]
                 V_i,t = AADT_i * k * (1/PHF) * f_tg              for t = 2        [veh/h]
                 V_i,t = AADT_i * k * (2 - 1/PHF) * f_tg          for t = 4        [veh/h]
  V_i,t  = demand inflow/outflow volume for section i, analysis period t          [veh/h]
  AADT_i = directional average annual daily traffic entering/leaving at section i's boundary [veh/day]
  k      = K-factor (peak-hour proportion of AADT)                                [decimal]
  f_tg   = traffic growth factor                                                  [decimal]
  PHF    = peak hour factor                                                       [decimal]
Implemented in: freeway_facilities/planning.rs::PlanningFacility::period_multipliers (the [1, 1/PHF, 1, 2-1/PHF] vector `mult`) and ::run_analysis (`boundary[i] * mult[p]`)

Equation 25-41:  q_i,t = V_i,t / f_HV                                              [pc/h]
  q_i,t = demand flow rate converted to passenger-car-equivalent units            [pc/h]
  f_HV  = heavy-vehicle adjustment factor (Equation 25-42)                         [decimal]
Implemented in: freeway_facilities/planning.rs::PlanningFacility::run_analysis

**CORRECTED:** `run_analysis` now computes `base_factor = k_factor * growth_factor / f_hv`, dividing the AADT-derived boundary flow by `f_hv` as Equation 25-41 requires (`q = V / f_HV`, verified against `197_Ch25_06.xhtml`). Since `f_HV = 1/(1+P_T(E_T-1)) <= 1`, dividing correctly inflates the veh/h count to the larger pc-equivalent when heavy vehicles are present. The previous code multiplied by `f_hv`, biasing d/c downward for any nonzero truck percentage; this was numerically invisible in the only planning fixture (`tests/ExampleCases/hcm/FreewayFacilities/planning_case1.json`, Example Problem 6), which sets `pct_sut = pct_tt = 0.0` so `f_HV = 1.0` (EP6 output unchanged). A regression test (`test_planning_equation_25_41_heavy_vehicle_conversion`) asserts d/c rises with a nonzero truck percentage. Fixed in commit for `fix/hcm-equation-sweep` (Eq 25-41 direction).

Equation 25-42:  f_HV = 1 / (1 + P_T*(E_T - 1))                                   [decimal]
  f_HV = heavy-vehicle adjustment factor                                          [decimal]
  P_T  = combined SUT + TT proportion (pct_sut + pct_tt)                          [decimal]
  E_T  = passenger-car equivalent of one heavy vehicle (Exhibit 12-25: 2.0 level, 3.0 rolling/mountainous) [PCE]
Implemented in: freeway_facilities/planning.rs::PlanningFacility::f_hv

Equation 25-43:  d_i,p = d_(i-1),p + (q_i,p)_in - (q_i,p)_out + d'_i,(p-1)         [pc/h]
  d_i,p       = demand level on section i in analysis period p                    [pc/h]
  d_(i-1),p   = demand level on the upstream section in the same period           [pc/h]
  (q_i,p)_in  = inflow demand entering section i in period p (first section / on-ramps) [pc/h]
  (q_i,p)_out = outflow demand leaving section i in period p (off-ramps)          [pc/h]
  d'_i,(p-1)  = carryover (vertical-queue) demand released from section i, previous period p-1 [pc/h]
Implemented in: freeway_facilities/planning.rs::PlanningFacility::run_analysis (`demand = (upstream + boundary[i] * mult[p]).max(0.0) + carryover[i]`, where `upstream` carries d_(i-1),p and `boundary[i] * mult[p]` is the net q_in - q_out for period p)

Equation 25-44:  d'_i,p = max(d_i,p - c_i, 0)                                      [pc/h]
  d'_i,p = vertical-queue carryover demand on section i released to period p+1     [pc/h]
  c_i    = capacity of section i (Equation 25-45, adjusted)                        [pc/h]
Implemented in: freeway_facilities/planning.rs::PlanningFacility::run_analysis (`next_carryover[i] = (demand - cap).max(0.0)`)

Equation 25-45:  c_i = 2200 + 10 * [min(70, FFS) - 50]                             [pc/h/ln]
  c_i = capacity of freeway section i                                             [pc/h/ln]
  FFS = facility free-flow speed                                                  [mi/h]
Implemented in: freeway_facilities/planning.rs::basic_section_capacity_pc

Equation 25-46:  CAF_weave = min(0.884 - 0.0752*V_r + 0.0000243*L_s, 1.0)          [decimal, capped at 1.0]
  CAF_weave = capacity adjustment factor used for a weaving segment                [decimal]
  V_r       = ratio of weaving demand flow rate to total demand flow rate in the section [decimal]
  L_s       = weaving segment length                                              [ft]
Implemented in: freeway_facilities/planning.rs::weave_caf (the `length_mi` argument is converted internally via `length_mi * 5280.0` to get L_s in ft)

Ramp-section CAF (Chapter 25 §6 text, no equation number): "an average CAF of 0.9 can be used for ramp sections" absent a section-specific measured value.
Implemented in: freeway_facilities/planning.rs::DEFAULT_RAMP_CAF (0.9), applied in ::section_capacity_pc_per_lane when a Ramp section has no `caf_override`.

Equation 25-47 (Exhibit 25-16 coefficients; as implemented — see VERIFICATION.md items 3/4 for the two departures from the printed form, noted below rather than re-flagged):
  ΔRU_i,p = 0                                                                  if d_i,p/c_i < E
  ΔRU_i,p = A*(d_i,p/c_i)^3 + B*(d_i,p/c_i)^2 + C*(d_i,p/c_i) + D               otherwise
  ΔRU_i,p   = undersaturated delay rate for section i, period p                 [s/mi] (printed beside the equation as min/mi — VERIFICATION.md item 4)
  A,B,C,D,E = Exhibit 25-16 coefficients, keyed by free-flow speed (nearest 5-mi/h column, 55-75 mi/h) [dimensionless]
  Departure from print (VERIFICATION.md item 3): the code evaluates the cubic at the actual d_i,p/c_i even when it exceeds 1.0, rather than restricting to the printed `E <= d/c <= 1` domain.
Implemented in: freeway_facilities/planning.rs::undersaturated_delay_rate, ::undersaturated_params (the Exhibit 25-16 table, keyed by FFS)

Equation 25-48:  ΔRO_i,p = (450 / L) * max(0, d_i,p/c_i - 1.0)                     [s/mi] (printed as min/mi)
  ΔRO_i,p = additional oversaturation delay rate for section i, period p          [s/mi]
  L       = section length                                                        [mi]
Implemented in: freeway_facilities/planning.rs::oversaturated_delay_rate — public helper, retained per the printed formulation but NOT summed into the reported delay/travel rate (VERIFICATION.md item 3: oversaturation is instead expressed only through the Equation 25-43/25-44 vertical-queue carryover)

Equation 25-49 (as implemented; VERIFICATION.md item 3 — the printed ΔRO term is omitted):
  TR_i,p = ΔRU_i,p + TR_FFS                                                       [s/mi] (printed as ΔRU + ΔRO + TR_FFS, min/mi)
  TR_FFS = 3600 / FFS                                                             [s/mi] (free-flow travel rate)
Implemented in: freeway_facilities/planning.rs::PlanningFacility::run_analysis (`travel_rate = dru + tr_ffs`, `tr_ffs = 3600.0 / ffs`)

Equation 25-50:  T_i,p = TR_i,p * L_i                                             [s] (printed as min)
  T_i,p = travel time on section i, period p                                     [s]
  L_i   = length of section i                                                    [mi]
Implemented in: freeway_facilities/planning.rs::PlanningFacility::run_analysis (`travel_time_s = travel_rate * sec.length_mi`)

Equation 25-51:  S_i,p = 3600 / TR_i,p                                            [mi/h]
  S_i,p = space mean speed of section i, period p                                 [mi/h]
  (the printed form S = L/T assumes T in hours; the s/mi treatment of TR_i,p here — VERIFICATION.md item 4 — makes 3600/TR the equivalent conversion)
Implemented in: freeway_facilities/planning.rs::PlanningFacility::run_analysis (`speed = 3600.0 / travel_rate`, falling back to FFS when travel_rate <= 0)

Equation 25-52:  D_i,p = d_i,p / (N_i * S_i,p)                                    [pc/mi/ln]
  D_i,p = density of section i, period p                                         [pc/mi/ln]
  N_i   = number of lanes in section i                                           [ln]
Implemented in: freeway_facilities/planning.rs::PlanningFacility::run_analysis (`density = demand / (lanes * speed)`)

Facility aggregation (Exhibit 25-96, no single equation number):
  Facility travel time      = Sum_i(T_i,p) / 60                                              [min]
  Facility space mean speed = total_length / (Sum_i(T_i,p) / 3600)                            [mi/h] (falls back to FFS if total time is 0)
  Facility density           = Sum_i(D_i,p * L_i) / Sum_i(L_i)                                [pc/mi/ln] — length-weighted, NOT the Equation 10-1 lane-mile weighting used elsewhere in Chapter 10 (VERIFICATION.md item 5)
  Facility vertical queue    = Sum_i(queue_length_mi_i)                                        [mi]
  Facility oversaturated     = true if any section d_i,p/c_i > 1.0
Implemented in: freeway_facilities/planning.rs::PlanningFacility::aggregate_facility

Facility LOS (Exhibit 25-17): urban thresholds <=11/18/26/35/45 pc/mi/ln, rural <=6/14/22/29/39 pc/mi/ln (A through E), F above the E threshold or if any section v_d/c > 1.00 — confirmed identical to the Exhibit 10-6 breakpoints reused by the code (verified against `197_Ch25_06.xhtml`, no discrepancy).
Implemented in: freeway_facilities/exhibits.rs::los_freeway_facility, invoked from planning.rs::aggregate_facility with the `oversaturated` flag forcing F
```

### Details a reviewer should check against the manual

Demand chaining (Equations 25-43/25-44): per period, each section's demand is `upstream + boundary[i] * mult[p] + carryover[i]`, where `boundary[i] = (inflow_aadt - outflow_aadt) * K * f_tg * f_HV` (net pc/h added at the upstream boundary before the period multiplier) and `carryover[i] = max(demand_prev - capacity, 0)` is the vertical queue released from the previous period; the downstream `upstream` value carries this period's full demand (including any released queue), so an upstream vertical queue raises downstream demand in the same period it is released.

The delay-rate treatment departs from the printed Equations 25-47 through 25-49 in three coupled ways that all follow the worked Example Problem 6 instead of the printed equation text — this is VERIFICATION.md item 3 and the largest interpretive decision in this module, flagged with a long `VERIFY-HCM` comment inside `run_analysis`: the reported delay rate and travel rate use the Equation 25-47 undersaturated term only (the Equation 25-48 oversaturated term is never added, contradicting Equation 25-49 as printed); Equation 25-47's cubic is evaluated at the actual d/c even when d/c > 1.0 (contradicting the `E <= d/c <= 1` domain printed with the equation); and oversaturation manifests only through the Equation 25-43/25-44 vertical-queue carryover. `oversaturated_delay_rate` (Equation 25-48, `450/L * max(0, d/c - 1)`) remains available as a public helper for callers who want the printed formulation. Relatedly, the Equation 25-47 output is treated as s/mi rather than the "min/mi" printed beside the equation, because the worked example adds it directly to the free-flow travel rate `TR_FFS = 3600/FFS` s/mi (VERIFICATION.md item 4, which also notes the Exhibit 25-16 FFS = 55 row's anomalous `D = -0.12` coefficient, transcribed as printed in `undersaturated_params`).

Downstream quantities: travel rate `TR = delta_RU + 3600/FFS` (s/mi), travel time `T = TR * L` (s), speed `S = 3600 / TR` (mi/h), density `D = demand / (lanes * S)` (pc/mi/ln), and vertical queue length `= next_carryover / (lanes * density)` (mi). Facility aggregation (`aggregate_facility`) sums section travel times, computes the space mean speed as total length over total time, and takes a length-weighted (not lane-weighted) average of section densities per the Exhibit 25-96 note — deliberately different from the Equation 10-1 lane-mile weighting used everywhere else in Chapter 10 (VERIFICATION.md item 5). Facility LOS uses `exhibits::los_freeway_facility` with the oversaturated flag forcing F. Note that `los_freeway_facility` implements the Exhibit 10-6 urban/rural thresholds; the module doc cites Exhibit 25-17 for the planning method's LOS thresholds, so the reviewer should confirm the two exhibits carry the same breakpoints (the code assumes they do by reusing the Chapter 10 function).

## Validation

Integration tests live in `tests/chapter10_integration.rs` (the Example Problem 5 and 6 sections at the bottom of the file, after the EP1/EP2 core-method tests), reading `tests/ExampleCases/hcm/FreewayFacilities/ml_case1.json` and `planning_case1.json`. There is no Python-binding integration test for either extension on this branch (`tests/test_chapter11_integration.py` and `tests/test_twolanehighways_integration.py` are the only Python tests touching adjacent code).

**Example Problem 5** (managed-lane facility, Exhibits 25-78 through 25-87; the EP2 GP geometry with 20% of mainline entry demand allocated to one marking-separated Continuous Access ML, which keeps the GP lanes undersaturated per the fixture comment):
- `ep5_ml_capacity_matches_exhibit_25_81`: ML capacity 1,614 veh/h (1,650 pc/h/ln x f_HV) at +-3 veh/h across all 55 cells.
- `ep5_ml_dc_ratios_match_exhibit_25_82`: uniform ML d/c per period [0.62, 0.68, 0.72, 0.64, 0.52] at +-0.005 (no ML ramps, so uniform along the facility).
- `ep5_gp_density_matrix_matches_exhibit_25_84`: full 55-cell GP density matrix at +-0.6 veh/mi/ln (these densities drive the friction check).
- `ep5_ml_speeds_and_friction_match_exhibit_25_83`: friction-free speeds 59.3/58.9/58.6/59.7 mi/h (+-0.3) and friction-affected cells Segments 8-9 Period 2 = 53.5 and Segments 8-10 Period 3 = 52.1 mi/h (+-0.4), plus boolean friction-flag checks. The Segment 10 / Period 2 published 58.1 mi/h is not asserted (the non-reproducible cell of VERIFICATION.md item 2).
- `ep5_lane_group_performance_matches_exhibit_25_86`: per-period GP and ML lane-group speed/density pairs at +-0.6 / +-0.5.
- `ep5_facility_performance_matches_exhibit_25_87`: combined speed at +-0.6, LOS letters exact (C/D/D/C/C), and combined density at a widened +-1.0 tolerance to cover the Period 3 cell asserted at the computed 28.3 veh/mi/ln against the book's non-reproducible 29.1 (VERIFICATION.md item 1; the in-test comment carries both numbers).

**Example Problem 6** (planning method, Exhibits 25-88 through 25-96; seven sections derived from the EP1 geometry, directional AADT inputs, four periods):
- `ep6_dc_ratios_match_exhibit_25_91`: full 7-section x 4-period d/c matrix at +-0.01, including the Section 6 Period 2 value of 1.02 (the oversaturated cell).
- `ep6_delay_rates_match_exhibit_25_92`: full delay-rate matrix at +-0.4 s/mi, including Section 6 Period 2 = 11.7 s/mi — which is `delta_RU(1.016)`, only reproducible under the ΔRU-at-actual-d/c reading (VERIFICATION.md item 3).
- `ep6_facility_performance_matches_exhibit_25_96`: per-period oversaturated flag (period 2 only), travel time (+-0.15 min), space mean speed (+-0.6 mi/h), length-weighted density (+-0.8 pc/mi/ln, the widened band covering the Section 6 Period 2 book rounding of VERIFICATION.md item 5), total vertical queue (0.8 mi in period 2, +-0.15 — reproducing the published queue via Equations 25-43/25-44), and LOS letters exact (D/F/D/C).

Unit tests in `src/hcm/freeway_facilities/tests.rs` (managed-lanes and planning sections): `test_cross_weave_caf_equation_13_24` checks the CRF formula against a hand computation (CW = 1,000 pc/h, L = 1,000 ft, N = 3 gives CRF ~ 0.0788) and monotonicity in length; `test_cross_weave_reduces_gp_capacity_step_a9` verifies the Step A-9 capacity reduction end-to-end on the EP1 facility (capacity-reduction effect only, per VERIFICATION.md item 7); `test_ml_adjacent_friction_activates_above_threshold` verifies on the EP2 (+11%) facility that friction flags fire only where GP density exceeds 35 pc/mi/ln; `test_planning_equation_25_45_basic_capacity`, `test_planning_equation_25_46_weave_caf` (including the EP6 weave section value 0.9358 and the 1.0 cap), `test_planning_equation_25_47_delay_rate` (threshold behavior below E = 0.72, the d/c = 0.86 value 2.8 s/mi, and the Equation 25-48 helper), and `test_planning_carryover_propagates_downstream` (a synthetic two-section facility where the released vertical queue raises downstream demand in the next period).

## Deferred

- **Oversaturated ML vertical-queue delay (Equations 25-35/25-36)** — VERIFICATION.md item 6 and the `managed_lanes.rs` module doc. Chapter 25 Section 4 runs the oversaturated engine separately per lane group and models access-segment spillback as a non-propagating vertical queue; this implementation analyzes each lane group with the existing under-/oversaturated engines, which is exact when the lane groups do not exchange flow through access segments (the case in the undersaturated Example Problem 5). The vertical-queue delay accounting between lane groups is not implemented; the hook would be in `ManagedLaneFacility::run_analysis` between the GP run and `evaluate_ml_segments`. Note also that the ML group is never routed through the oversaturated engine at all — `evaluate_ml_segments` always evaluates the Chapter 12 engine at demand, and `ml_dc_ratio` is reported but does not trigger a queued evaluation path.
- **ML volume served == ML demand.** There is no ML volume-served matrix distinct from demand (consequence of the deferral above); combined-facility VMT treats them as equal.
- The Chapter 25 Section 5 special work zone configuration tables (Exhibits 25-8 through 25-14) and per-segment work-zone alpha remain deferred from the core-methodology pass (see the "Deferred scopes" section of VERIFICATION.md).
- No planning-method PyO3 bindings or Python tests exist on this branch.
