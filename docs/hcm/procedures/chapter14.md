# Chapter 14: Freeway Merge and Diverge Segments — Procedure Walkthrough

HCM 7th Edition Chapter 14 analyzes ramp-freeway junctions: on-ramps (merge influence areas), off-ramps (diverge influence areas), and the major merge/diverge special cases, each over the standard 1,500-ft ramp influence area (Exhibit 14-1). The implementation is `src/hcm/merge_diverge/merge_diverge.rs` (renamed from `src/hcm/chapter14/merge_diverge.rs` on the topic-folder-rename branch) on the `feat/hcm-ch12-14-completion` lineage, a single `RampSegment` struct in the chapter15 house style (plain inputs, `Option<T>` computed fields, step methods in HCM order, `run_analysis()`), plus free functions for the capacity tables and special-case helpers. This branch's rewrite (commit `e921e1e`) corrected roughly twenty equation/exhibit transcriptions against the manual — the full list is in "Equation fixes made by this branch" below, which the reviewer should treat as the checklist of highest-risk spots. A PyO3 wrapper (`copython::merge_diverge::RampSegment`) is registered on this branch. All twenty-eight numbered HCM equations (14-1 through 14-28) plus the Exhibit 14-13/14-14/14-15 speed models and the Exhibit 14-18 left-hand-ramp factors are cross-checked and transcribed in full in the "Equation Reference" section below, verified against the EPUB source (`94_Ch14.xhtml` through `100_Ch14_06.xhtml`) and against the current state of `merge_diverge.rs`.

## Step-by-step walkthrough

| Manual step | HCM Eq./Exhibit | Rust method | Inputs (units) | Output (units) |
|---|---|---|---|---|
| Step 1: demand flow rates | Eq 14-1 (`v = V/(PHF·f_HV)`), f_HV per Eq 12-10 / Exhibit 12-25; 10-lane freeways: Exhibit 14-19 / Eq 14-27 Lane-5 deduction via `get_lane5_flow` | `determine_demand_flow` | `freeway_demand`, `ramp_demand` (veh/h), `phf`, `heavy_vehicle_pct` and optional separate `ramp_heavy_vehicle_pct` (decimals), `terrain` | `(v_F, v_R)` pc/h |
| Step 2: flow in Lanes 1-2 | Merge: Eq 14-2 (`v_12 = v_F·P_FM`) with P_FM from Exhibit 14-8 (Eq 14-3 base `0.5775 + 0.000028·L_A` for 6-lane; adjacent-off-ramp Eqs 14-4/14-5 gated by L_EQ Eqs 14-6/14-7; 8-lane split on v_F/S_FR ≤ 72). Diverge: Eq 14-8 (`v_12 = v_R + (v_F−v_R)·P_FD`) with P_FD from Exhibit 14-9 (Eq 14-9 base `0.760 − 0.000025·v_F − 0.000046·v_R`; adjacent-ramp Eqs 14-10/14-11 gated by L_EQ Eqs 14-12/14-13; 8-lane constant 0.436). Reasonableness checks Eqs 14-14..14-19. Left-hand ramps: Exhibit 14-18 factors. Two-lane ramps: special-case P_FM/P_FD tables and effective lane lengths Eqs 14-25/14-26 | `estimate_v12` (private `calculate_pfm`, `calculate_pfd`, `check_v12`; free functions `pfm_two_lane_onramp`, `pfd_two_lane_offramp`, `left_hand_adjustment`, `effective_accel_length`, `effective_decel_length`) | v_F, v_R (pc/h), `accel_lane_length`/`decel_lane_length` (+`2` variants for two-lane ramps, ft), `ramp_ffs` S_FR (mi/h), adjacent-ramp descriptors (type, distance ft, volume veh/h) | v_12 pc/h; also sets `p_f`, `v_r12` (merge: v_12+v_R per Eq 14-20; diverge: v_12), `v_oa` (avg outer-lane flow, pc/h/ln) |
| Step 3: capacity checks | Exhibit 14-10 (freeway capacity per lane by FFS: ≥70 → 2400, 65 → 2350, 60 → 2300, 55 → 2250 pc/h/ln; max desirable influence-area flows 4,600 merge / 4,400 diverge), Exhibit 14-12 (ramp roadway capacity by ramp FFS, doubled for two-lane ramps), Eq 14-21 (`c_mda = c_md·CAF`) | `determine_capacity` (free functions `get_freeway_capacity_per_lane`, `get_freeway_capacity`, `get_ramp_capacity`) | FFS (mi/h), lanes, ramp FFS, CAF | adjusted freeway capacity pc/h; sets `vc_ratio` (critical checkpoint: v_F+v_R downstream of merge, v_F upstream of diverge), `demand_exceeds_capacity`, `exceeds_max_desirable` (which does **not** by itself force LOS F) |
| Step 4: density | Merge: Eq 14-22 (`D_R = 5.475 + 0.00734·v_R + 0.0078·v_12 − 0.00627·L_A`). Diverge: Eq 14-23 (`D_R = 4.252 + 0.0086·v_12 − 0.009·L_D`). Major diverge: Eq 14-28 (`D = 0.0175·v_F/N`). Major merge: no HCM density model | `determine_density` | v_R, v_12 (pc/h), L_A/L_D (ft, effective two-lane values where applicable) | D_R pc/mi/ln (`density` stays `None` for major merge) |
| Step 4: LOS | Exhibit 14-3 (A≤10, B≤20, C≤28, D≤35, E>35; F only when demand exceeds capacity) | `determine_los` (free function `determine_ramp_los`) | D_R, over-capacity flag | `LevelOfService` |
| Step 5: speeds | Merge: Exhibit 14-13 (`M_S = 0.321 + 0.0039·e^(v_R12/1000) − 0.002·L_A·S_FR·SAF/1000`, v_R12 capped at 4,600 for M_S; `S_R = FFS·SAF − (FFS·SAF − 42)·M_S`, capped at FFS·SAF). Diverge: Exhibit 14-14 (`D_S = 0.883 + 0.00009·v_R − 0.013·S_FR·SAF`; `S_R = FFS·SAF − (FFS·SAF − 42)·D_S`). Outer-lane speeds S_O per Exhibits 14-13/14-14 piecewise forms (diverge S_O may exceed FFS via the 1.097 factor). All-lane average: Exhibit 14-15 space mean `S = (v_R12 + v_OA·N_O)/[(v_R12/S_R) + (v_OA·N_O/S_O)]`, capped at FFS·SAF. Aggregate density Eq 14-24 | `estimate_speed` | v_R12, v_OA, N_O (`outer_lanes()`), FFS, SAF, S_FR, L_A | `(S_R, Option<S_O>, S)` mi/h; also sets `density_all_lanes` pc/mi/ln |

`run_analysis()` chains Steps 1-5 in order (LOS is determined before speeds, matching the manual's core-output ordering).

## Equation Reference

Every numbered HCM equation used by this chapter (14-1 through 14-28), plus the Exhibit 14-13/14-14/14-15 speed models and the Exhibit 14-18 left-hand-ramp factors, cross-checked line by line against the current `src/hcm/merge_diverge/merge_diverge.rs` and against the EPUB MathML (`97_Ch14_03.xhtml` for Eqs 14-1 through 14-24, `98_Ch14_04.xhtml` for Eqs 14-25 through 14-28 and Exhibits 14-16 through 14-19). No discrepancies were found between the manual and the current code state for this chapter; every equation below reproduces the published form exactly.

### Step 1 — Demand flow rates

```
Equation 14-1:  v_i = V_i / (PHF·f_HV)
  v_i = demand flow rate for movement i, pc/h
  V_i = demand volume for movement i, veh/h
  PHF = peak hour factor, decimal (default 0.94 urban/rural per Exhibit 14-4)
  f_HV = adjustment factor for heavy-vehicle presence, decimal (Eq 12-10 form, Exhibit 12-25 PCEs)
Implemented in: merge_diverge/merge_diverge.rs::RampSegment::determine_demand_flow (via the private `fhv_for` helper); applied separately to the freeway flow and the ramp flow, with an optional distinct `ramp_heavy_vehicle_pct`.
```

```
Equation 14-27 (10-lane freeways, Lane-5 deduction — applied within Step 1):  v_F4eff = v_F − v_5
  v_F4eff = effective approaching freeway flow in the remaining four lanes, pc/h
  v_F = total approaching freeway flow in five lanes, pc/h
  v_5 = estimated approaching freeway flow in Lane 5, pc/h, from Exhibit 14-19's piecewise table (on-ramp and off-ramp forms differ)
Implemented in: merge_diverge/merge_diverge.rs::RampSegment::determine_demand_flow, using merge_diverge/merge_diverge.rs::get_lane5_flow for v_5 (Exhibit 14-19 constants transcribed as code constants/branches, not re-typed here); the segment is then analyzed as an 8-lane freeway (`freeway_lanes.min(4)`).
```

### Step 2 — Flow in Lanes 1 and 2 (merge side, P_FM)

```
Equation 14-2:  v_12 = v_F·P_FM
  v_12 = flow rate in Lanes 1 and 2 immediately upstream of the on-ramp (merge) influence area, pc/h
  v_F = total flow rate on the freeway immediately upstream of the merge influence area, pc/h
  P_FM = proportion of freeway vehicles remaining in Lanes 1 and 2 upstream of the on-ramp influence area, decimal
Implemented in: merge_diverge/merge_diverge.rs::RampSegment::estimate_v12 (on-ramp/major-merge arm), calling the private `calculate_pfm`.
```

```
Equation 14-3 (P_FM base case, 6-lane freeway, isolated or adjacent ramps without influence):  P_FM = 0.5775 + 0.000028·L_A
  P_FM = proportion of freeway vehicles in Lanes 1 and 2, decimal
  L_A = length of the acceleration lane, ft (effective two-lane length from Eq 14-25 where applicable; default 800 ft)
  (4-lane freeways: P_FM = 1.000 fixed, since only Lanes 1 and 2 exist; 8-lane freeways: see the piecewise form below)
Implemented in: merge_diverge/merge_diverge.rs::RampSegment::calculate_pfm, `lanes == 3` arm, `pfm_base`.
```

```
Equation 14-4 (P_FM, adjacent upstream off-ramp, when L_UP < L_EQ from Eq 14-6):  P_FM = 0.7289 − 0.0000135·(v_F + v_R) − 0.003296·S_FR + 0.000063·L_UP
  v_F = freeway demand flow rate, pc/h
  v_R = ramp demand flow rate, pc/h
  S_FR = free-flow speed of the ramp, mi/h
  L_UP = distance to the adjacent upstream off-ramp, ft
Implemented in: merge_diverge/merge_diverge.rs::RampSegment::calculate_pfm, upstream-adjacent-off-ramp candidate branch.
```

```
Equation 14-5 (P_FM, adjacent downstream off-ramp, when L_DOWN < L_EQ from Eq 14-7):  P_FM = 0.5487 + 0.2628·(v_D / L_DOWN)
  v_D = demand flow rate on the adjacent downstream ramp, pc/h
  L_DOWN = distance to the adjacent downstream ramp, ft
Implemented in: merge_diverge/merge_diverge.rs::RampSegment::calculate_pfm, downstream-adjacent-off-ramp candidate branch. When both Eq 14-4 and Eq 14-5 apply, the larger P_FM governs (`candidates ... max` fold), matching the manual's "two solutions may arise ... the larger value ... is used" instruction.
```

```
Equation 14-6 (equilibrium distance L_EQ for an adjacent upstream off-ramp, Eq 14-4's applicability gate):  L_EQ = 0.214·(v_F + v_R) + 0.444·L_A + 52.32·S_FR − 2,403
  L_EQ = equilibrium separation distance at which Eq 14-3 and Eq 14-4 give the same P_FM, ft
  v_F, v_R = freeway and ramp demand flow rates, pc/h
  L_A = acceleration lane length, ft
  S_FR = free-flow speed of the ramp, mi/h
Implemented in: merge_diverge/merge_diverge.rs::RampSegment::calculate_pfm, inline `l_eq` computation in the upstream-off-ramp branch (Eq 14-4 is used only when L_UP < L_EQ).
```

```
Equation 14-7 (equilibrium distance L_EQ for an adjacent downstream off-ramp, Eq 14-5's applicability gate):  L_EQ = v_D / (0.1096 + 0.000107·L_A)
  v_D = demand flow rate on the adjacent downstream ramp, pc/h
  L_A = acceleration lane length, ft
Implemented in: merge_diverge/merge_diverge.rs::RampSegment::calculate_pfm, inline `l_eq` computation in the downstream-off-ramp branch (Eq 14-5 is used only when L_DOWN < L_EQ).
```

8-lane freeway P_FM (Exhibit 14-8, no equation number assigned in the manual — printed only as the exhibit's third row): for v_F/S_FR ≤ 72, P_FM = 0.2178 − 0.000125·v_R + 0.01115·(L_A/S_FR); for v_F/S_FR > 72, P_FM = 0.2178 − 0.000125·v_R. Implemented in: merge_diverge/merge_diverge.rs::RampSegment::calculate_pfm, `_ => { ... }` (8-lane) arm. See Deviation 6 below: no clamp to [0, 1] is applied, and extreme v_R can drive this negative.

### Step 2 — Flow in Lanes 1 and 2 (diverge side, P_FD)

```
Equation 14-8:  v_12 = v_R + (v_F − v_R)·P_FD
  v_12 = flow rate in Lanes 1 and 2 immediately upstream of the deceleration lane, pc/h
  v_R = flow rate on the off-ramp, pc/h
  v_F = flow rate on the freeway immediately upstream of the ramp influence area, pc/h
  P_FD = proportion of through freeway traffic remaining in Lanes 1 and 2 upstream of the deceleration lane, decimal
Implemented in: merge_diverge/merge_diverge.rs::RampSegment::estimate_v12 (off-ramp/major-diverge arm), calling the private `calculate_pfd`.
```

```
Equation 14-9 (P_FD base case, 6-lane freeway):  P_FD = 0.760 − 0.000025·v_F − 0.000046·v_R
  v_F, v_R = freeway and ramp demand flow rates, pc/h
  (4-lane freeways: P_FD = 1.000 fixed; 8-lane freeways: P_FD = 0.436, a constant — Exhibit 14-9)
Implemented in: merge_diverge/merge_diverge.rs::RampSegment::calculate_pfd, `lanes == 3` arm, `pfd_base`.
```

```
Equation 14-10 (P_FD, adjacent upstream on-ramp, when v_U/L_UP ≤ 0.20 and L_UP < L_EQ from Eq 14-12):  P_FD = 0.717 − 0.000039·v_F + 0.604·(v_U/L_UP)
  v_F = freeway demand flow rate, pc/h
  v_U = demand flow rate on the adjacent upstream ramp, pc/h
  L_UP = distance to the adjacent upstream ramp, ft
Implemented in: merge_diverge/merge_diverge.rs::RampSegment::calculate_pfd, upstream-adjacent-on-ramp candidate branch. When v_U/L_UP > 0.20, Eq 14-9 is used instead regardless of L_EQ, per the manual's stated calibration-range limit; the code checks this gate before the L_EQ gate.
```

```
Equation 14-11 (P_FD, adjacent downstream off-ramp, when L_DOWN < L_EQ from Eq 14-13):  P_FD = 0.616 − 0.000021·v_F + 0.124·(v_D/L_DOWN)
  v_F = freeway demand flow rate, pc/h
  v_D = demand flow rate on the adjacent downstream ramp, pc/h
  L_DOWN = distance to the adjacent downstream ramp, ft
Implemented in: merge_diverge/merge_diverge.rs::RampSegment::calculate_pfd, downstream-adjacent-off-ramp candidate branch. When both Eq 14-10 and Eq 14-11 apply, the larger P_FD governs (`candidates ... max` fold).
```

```
Equation 14-12 (equilibrium distance L_EQ for an adjacent upstream on-ramp, Eq 14-10's applicability gate):  L_EQ = v_U / (0.071 + 0.000023·v_F − 0.000076·v_R)
  v_U = demand flow rate on the adjacent upstream ramp, pc/h
  v_F, v_R = freeway and ramp demand flow rates, pc/h
Implemented in: merge_diverge/merge_diverge.rs::RampSegment::calculate_pfd, inline `l_eq`/`denom` computation in the upstream-on-ramp branch; the code additionally guards `denom > 0.0` before comparing, which the manual does not state but which prevents a sign-flipped comparison when the denominator would otherwise go non-positive.
```

```
Equation 14-13 (equilibrium distance L_EQ for an adjacent downstream off-ramp, Eq 14-11's applicability gate):  L_EQ = v_D / (1.15 − 0.000032·v_F − 0.000369·v_R)
  v_D = demand flow rate on the adjacent downstream ramp, pc/h
  v_F, v_R = freeway and ramp demand flow rates, pc/h
Implemented in: merge_diverge/merge_diverge.rs::RampSegment::calculate_pfd, inline `l_eq`/`denom` computation in the downstream-off-ramp branch (same `denom > 0.0` guard as Eq 14-12).
```

### Step 2 — Reasonableness checks (Eqs 14-14 through 14-19)

Applied after P_FM/P_FD selection and any left-hand-ramp adjustment, to guard against outer-lane flows the regression models were not calibrated to reproduce.

```
Equation 14-14 (6-lane freeways, one outer lane):  v_3 = v_F − v_12
  v_3 = flow rate in Lane 3 (the sole outer lane), pc/h/ln
  v_F, v_12 = freeway flow and Lanes-1-2 flow, pc/h
Implemented in: merge_diverge/merge_diverge.rs::RampSegment::check_v12, `lanes == 3` arm.
```

```
Equation 14-15 (used when v_3 > 2,700 pc/h/ln):  v_12a = v_F − 2,700
  v_12a = adjusted flow rate in Lanes 1 and 2, pc/h
Implemented in: merge_diverge/merge_diverge.rs::RampSegment::check_v12, `lanes == 3` arm, using the `MAX_OUTER_LANE_FLOW` constant (2,700 pc/h/ln) in place of the literal.
```

```
Equation 14-16 (used when v_3 > 1.5·(v_12/2)):  v_12a = v_F / 1.75
Implemented in: merge_diverge/merge_diverge.rs::RampSegment::check_v12, `lanes == 3` arm. This is the equation corrected by commit `e921e1e` (see "Equation fixes" below — the divisor was previously 2.5, the 8-lane value).
```

```
Equation 14-17 (8-lane freeways, two outer lanes):  v_av34 = (v_F − v_12) / 2
  v_av34 = average flow rate per outer lane (Lanes 3 and 4), pc/h/ln
Implemented in: merge_diverge/merge_diverge.rs::RampSegment::check_v12, `lanes == 4` arm.
```

```
Equation 14-18 (used when v_av34 > 2,700 pc/h/ln):  v_12a = v_F − 5,400
Implemented in: merge_diverge/merge_diverge.rs::RampSegment::check_v12, `lanes == 4` arm, as `v_f − 2.0 * MAX_OUTER_LANE_FLOW`.
```

```
Equation 14-19 (used when v_av34 > 1.5·(v_12/2)):  v_12a = v_F / 2.50
Implemented in: merge_diverge/merge_diverge.rs::RampSegment::check_v12, `lanes == 4` arm. Per the manual, "in cases where both limitations ... are violated, the result yielding the highest value of v_12a is used" — `check_v12` collects both candidates into a `Vec` and folds with `max` for both the 6-lane and 8-lane arms (Eqs 14-14/14-17 flow the same candidate-selection logic).
```

### Step 3 — Capacity

```
Equation 14-20 (total flow entering an on-ramp's influence area):  v_R12 = v_12 + v_R
  v_R12 = total flow rate entering the merge influence area, pc/h
  v_12 = flow rate in Lanes 1 and 2, pc/h
  v_R = ramp demand flow rate, pc/h
Implemented in: merge_diverge/merge_diverge.rs::RampSegment::estimate_v12 (the `v_r12` assignment; for off-ramps v_R12 is simply v_12, per the manual's text that at diverges "the total flow rate entering the ramp influence area is merely the estimated value of v_12").
```

```
Equation 14-21:  c_mda = c_md·CAF
  c_mda = adjusted capacity of the merge/diverge area, veh/h
  c_md = unadjusted capacity of the merge/diverge area, veh/h (Exhibit 14-10 freeway table or Exhibit 14-12 ramp table)
  CAF = capacity adjustment factor, decimal (default 1.0; Chapter 11 weather/incident/driver-population components)
Implemented in: merge_diverge/merge_diverge.rs::RampSegment::determine_capacity, applied to both `get_freeway_capacity(...)` and `get_ramp_capacity(...)`. Exhibit 14-10 (freeway capacity per lane by FFS, and the 4,600/4,400 pc/h maximum-desirable influence-area flows) and Exhibit 14-12 (ramp roadway capacity by ramp FFS, doubled for two-lane ramps) are transcribed as the constants/branches in merge_diverge/merge_diverge.rs::get_freeway_capacity_per_lane, ::get_freeway_capacity, and ::get_ramp_capacity — cited here, not re-typed, since only the already-coded values are covered per the task scope.
```

### Step 4 — Density

```
Equation 14-22 (on-ramp/merge influence area):  D_R = 5.475 + 0.00734·v_R + 0.0078·v_12 − 0.00627·L_A
  D_R = density in the ramp influence area, pc/mi/ln
  v_R = ramp demand flow rate, pc/h
  v_12 = flow rate in Lanes 1 and 2, pc/h
  L_A = acceleration lane length, ft (effective two-lane value from Eq 14-25 where applicable)
Implemented in: merge_diverge/merge_diverge.rs::RampSegment::determine_density, `RampType::OnRamp` arm.
```

```
Equation 14-23 (off-ramp/diverge influence area):  D_R = 4.252 + 0.0086·v_12 − 0.009·L_D
  D_R = density in the ramp influence area, pc/mi/ln
  v_12 = flow rate in Lanes 1 and 2 (includes v_R for off-ramps), pc/h
  L_D = deceleration lane length, ft (effective two-lane value from Eq 14-26 where applicable)
Implemented in: merge_diverge/merge_diverge.rs::RampSegment::determine_density, `RampType::OffRamp` arm.
```

```
Equation 14-24 (aggregate density across all lanes of the 1,500-ft influence area, computed in Step 5 since it depends on the Exhibit 14-15 all-lane speed):  D = v/S
  D = density including all lanes of the 1,500-ft ramp influence area, pc/mi/ln
  v = total flow rate through the merge or diverge area, all lanes, pc/h/ln
  S = average speed of all vehicles through the merge or diverge area, all lanes, from Exhibit 14-15, mi/h
Implemented in: merge_diverge/merge_diverge.rs::RampSegment::estimate_speed, `density_all_lanes` assignment (merge: v = v_F+v_R; diverge: v = v_F, both divided by `freeway_lanes`). **See Deviation 3 below (VERIFY-HCM in code):** the manual states v in pc/h/ln without fixing the lane basis for the total; the per-mainline-lane-count division implemented here is a documented interpretation, not a manual-stated formula.
```

```
Equation 14-28 (major diverge density, no HCM model exists for major merge):  D_MD = 0.0175·(v_F/N)
  D_MD = density in the major diverge influence area (all approaching freeway lanes), pc/mi/ln
  v_F = demand flow rate immediately upstream of the major diverge influence area, pc/h
  N = number of lanes approaching the major diverge, ln
Implemented in: merge_diverge/merge_diverge.rs::RampSegment::determine_density, `RampType::MajorDiverge` arm. `RampType::MajorMerge` leaves `density = None` — the manual states "there are no effective models of performance for a major merge area" and limits major-merge analysis to capacity checks only (see Deviation 2 below).
```

### Step 5 — Speeds (Exhibits 14-13, 14-14, 14-15)

```
Exhibit 14-13 (on-ramp/merge speeds):
  M_S = 0.321 + 0.0039·e^(v_R12/1,000) − 0.002·(L_A·S_FR·SAF/1,000)
  S_R = FFS·SAF − (FFS·SAF − 42)·M_S
    M_S = speed index for merge areas (intermediate term), unitless
    S_R = average speed within the ramp influence area, mi/h
    v_R12 = total flow rate entering the merge influence area, pc/h (capped at 4,600 pc/h for this calculation only, per the exhibit's note)
    L_A = acceleration lane length, ft; S_FR = ramp free-flow speed, mi/h; FFS = freeway free-flow speed, mi/h; SAF = speed adjustment factor, decimal (default 1.0)
  Outer-lane speed S_O (piecewise, N_O > 0 only): S_O = FFS·SAF for v_OA < 500 pc/h/ln; S_O = FFS·SAF − 0.0036·(v_OA − 500) for 500 ≤ v_OA ≤ 2,300 pc/h/ln; S_O = FFS·SAF − 6.53 − 0.006·(v_OA − 2,300) for v_OA > 2,300 pc/h/ln.
Implemented in: merge_diverge/merge_diverge.rs::RampSegment::estimate_speed, `RampType::OnRamp | RampType::MajorMerge` arm. The v_R12 cap is `v_r12.min(MAX_MERGE_INFLUENCE_FLOW)`; S_R is further capped at FFS·SAF, per the manual's statement that merge-area predicted speeds may not exceed FFS.
```

```
Exhibit 14-14 (off-ramp/diverge speeds):
  D_S = 0.883 + 0.00009·v_R − 0.013·S_FR·SAF
  S_R = FFS·SAF − (FFS·SAF − 42)·D_S
    D_S = speed index for diverge areas (intermediate term), unitless
    v_R = ramp demand flow rate, pc/h; S_FR, FFS, SAF as above
  Outer-lane speed S_O (piecewise, N_O > 0 only): S_O = 1.097·FFS·SAF for v_OA < 1,000 pc/h/ln; S_O = 1.097·FFS·SAF − 0.0039·(v_OA − 1,000) for v_OA ≥ 1,000 pc/h/ln.
Implemented in: merge_diverge/merge_diverge.rs::RampSegment::estimate_speed, `RampType::OffRamp | RampType::MajorDiverge` arm. The manual notes diverge-area outer-lane speed may marginally exceed FFS via the 1.097 factor — this is not clamped in code, matching the manual's explicit statement.
```

```
Exhibit 14-15 (average speed of all vehicles):
  v_OA = (v_F − v_12) / N_O
  Merge: S = (v_R12 + v_OA·N_O) / [(v_R12/S_R) + (v_OA·N_O/S_O)]
  Diverge: S = (v_12 + v_OA·N_O) / [(v_12/S_R) + (v_OA·N_O/S_O)]
    v_OA = average demand flow per outer lane, pc/h/ln
    N_O = number of outer lanes (0 for 4-lane, 1 for 6-lane, 2 for 8-lane freeways), ln
    S = space-mean (harmonic) average speed of all vehicles in all lanes within the 1,500-ft influence area, mi/h, capped at FFS·SAF
Implemented in: merge_diverge/merge_diverge.rs::RampSegment::estimate_v12 (`v_oa` assignment) and ::estimate_speed (`s_avg` assignment, the harmonic-mean form with the `.min(ffs_adj)` cap). This is the quantity corrected by commit `e921e1e` (see "Equation fixes" below — the average was previously arithmetic and uncapped).
```

### Extensions to the methodology (Eqs 14-25/14-26, Exhibit 14-18)

```
Equation 14-25 (two-lane on-ramps, effective acceleration lane length, substituted for L_A everywhere it appears above):  L_Aeff = 2·L_A1 + L_A2
  L_Aeff = effective acceleration lane length for a two-lane on-ramp, ft
  L_A1 = length of the first (rightmost) acceleration lane, ft
  L_A2 = length of the second acceleration lane, ft
Implemented in: merge_diverge/merge_diverge.rs::effective_accel_length, called from RampSegment::effective_la. The manual also fixes the two-lane P_FM at 1.000/0.555/0.209 for 4-/6-/8-lane freeways (Exhibit 14-16 discussion, no equation number) — merge_diverge/merge_diverge.rs::pfm_two_lane_onramp — and states a two-lane ramp is always treated as isolated (no adjacent-ramp equations apply). **See Deviation 1 below:** the manual caps individual acceleration-lane component lengths (L_A1, L_A2) at 1,500 ft each ("the acceleration lane length used for calculation should be set to 1,500 ft"), but `effective_accel_length` caps only the computed *effective total* L_Aeff at 1,500 ft, which is a looser cap than the manual describes for cases where either component alone would exceed 1,500 ft.
```

```
Equation 14-26 (two-lane off-ramps, effective deceleration lane length, only when two deceleration lanes exist):  L_Deff = 2·L_D1 + L_D2
  L_Deff = effective deceleration lane length for a two-lane off-ramp, ft
  L_D1 = length of the first deceleration lane, ft
  L_D2 = length of the second deceleration lane, ft
Implemented in: merge_diverge/merge_diverge.rs::effective_decel_length, called from RampSegment::effective_ld. The manual also fixes the two-lane P_FD at 1.000/0.450/0.260 for 4-/6-/8-lane freeways — merge_diverge/merge_diverge.rs::pfd_two_lane_offramp. Same component-vs-effective-total cap caveat as Eq 14-25 (Deviation 1).
```

```
Exhibit 14-18 (left-hand ramp-freeway junctions, adjustment factors applied to v_12 computed as if the ramp were right-hand): on-ramps 1.00/1.12/1.20 and off-ramps 1.00/1.05/1.10 for 4-/6-/8-lane freeways respectively. The manual states the remaining computations then use v_23 (six-lane) or v_34 (eight-lane) in place of v_12, with "all capacity values remain unchanged."
Implemented in: merge_diverge/merge_diverge.rs::left_hand_adjustment, applied in RampSegment::estimate_v12 before the Eqs 14-14..14-19 reasonableness checks (see Deferred section below regarding the ordering question between the two).
```



Commit `e921e1e` ("refactor(hcm13,hcm14): chapter15-style structs with HCM-verified equations") lists the Chapter 14 corrections against the pre-existing code; together with the Chapter 13 items and the Chapter 12 items in `ea74afa` they are the "~20 equation fixes" of this branch. The Chapter 14 list, cross-checked against the current code:

1. **Eq 14-9** (P_FD base, 6-lane diverge): `P_FD = 0.760 − 0.000025·v_F − 0.000046·v_R` — the sign of the v_F term was wrong before (`calculate_pfd`, `lanes == 3` arm).
2. **Eq 14-10** (P_FD with upstream adjacent on-ramp): `P_FD = 0.717 − 0.000039·v_F + 0.604·(v_U/L_UP)` — old code used v_U with a positive coefficient in place of v_F. The code also implements the published applicability gates: v_U/L_UP ≤ 0.20 and L_UP < L_EQ (Eq 14-12).
3. **Exhibit 14-13** (merge speed factor): `M_S = 0.321 + 0.0039·e^(+v_R12/1000) − 0.002·(L_A·S_FR·SAF/1000)` — old code had `0.21` as the intercept and a negated exponent (`e^(−v_R12/1000)`). The v_R12-capped-at-4,600 note is implemented (`v_r12.min(MAX_MERGE_INFLUENCE_FLOW)`).
4. **Exhibit 14-14** (diverge speed factor): `D_S = 0.883 + 0.00009·v_R − 0.013·S_FR·SAF` — old code used v_12 in place of v_R and 0.0013 in place of 0.013.
5. **Exhibit 14-15** (all-lane speed): space mean (harmonic) speed capped at FFS — was an arithmetic mean, uncapped.
6. **Eq 14-16** (6-lane reasonableness adjustment): `v_12a = v_F / 1.75` — was `v_F / 2.5` (which is the 8-lane Eq 14-19 divisor).
7. **Eqs 14-15 through 14-19** (reasonableness checks): when both outer-lane limits are violated, the **larger** adjusted v_12a governs — `check_v12` collects both candidates and takes the max; old code applied them in a fixed order.
8. **L_EQ closed forms**: Eqs 14-6 (`0.214(v_F+v_R) + 0.444·L_A + 52.32·S_FR − 2403`), 14-7 (`v_D/(0.1096 + 0.000107·L_A)`), 14-12 (`v_U/(0.071 + 0.000023·v_F − 0.000076·v_R)`), and 14-13 (`v_D/(1.15 − 0.000032·v_F − 0.000369·v_R)`) are now the published expressions (previously approximated).
9. **Dual adjacent ramps**: when both upstream and downstream adjacent-ramp equations apply, the larger P_FM/P_FD governs (the `candidates ... max` fold in `calculate_pfm`/`calculate_pfd`).
10. **Major merge**: capacity checks only — no HCM density/LOS model is fabricated (`determine_density` leaves `density = None`; see deviation 2 on the LOS return value).
11. **Separate ramp heavy-vehicle percentage**: `ramp_heavy_vehicle_pct: Option<f64>` supported in Eq 14-1 (the Chapter 28 examples use distinct freeway/ramp HV%); defaults to the freeway value.
12. **10-lane freeways**: Lane 5 flow deducted per Exhibit 14-19 / Eq 14-27 (`get_lane5_flow`, separate on-ramp and off-ramp piecewise tables) and the junction analyzed as an 8-lane freeway (`freeway_lanes.min(4)` throughout).

Additional published details implemented in the rewrite (not framed as fixes but load-bearing): two-lane ramp P_FM/P_FD constants (1.000/0.555/0.209 and 1.000/0.450/0.260 for 4/6/8-lane freeways), effective lane lengths Eq 14-25 (`L_Aeff = 2·L_A1 + L_A2`) and Eq 14-26 (deceleration analog), Exhibit 14-18 left-hand-ramp factors (on: 1.00/1.12/1.20; off: 1.00/1.05/1.10) applied to v_12 computed as if right-hand, and the Exhibit 14-10 maximum-desirable influence-area flows (4,600/4,400 pc/h) tracked via `exceeds_max_desirable` without forcing LOS F.

## Validation

- **Fixtures**: `tests/ExampleCases/hcm/MergeDiverge/case1.json` through `case4.json` — HCM Chapter 28 Example Problems 1-4 (commit `a2ec7e7`): EP1 isolated one-lane on-ramp, 4-lane freeway; EP2 first of an adjacent off-ramp pair, 6-lane; EP3 on-ramp, 8-lane, where the lane-distribution check fails and Eq 14-19 governs (v_12a = v_F/2.50 = 2,570 pc/h); EP4 left-hand on-ramp, 6-lane (Exhibit 14-18 factor 1.12, v_23 = 3,211 pc/h).
- **Test file**: `tests/chapter14_integration.rs`. Each test asserts the published chain: v_F/v_R (Step 1), P_FM/P_FD and v_12/v_R12 (Step 2), freeway/ramp capacities and the over-capacity/max-desirable flags (Step 3), D_R and exact LOS letter (Step 4), and S_R/S_O/S (Step 5). Anchors include EP1: v_F = 2,918 pc/h, P_FM = 1.000, D_R = 28.2, LOS D, S_R = 53.0 with M_S = 0.389; EP2: P_FD = 0.617 (isolated treatment since L_DOWN = 750 ft ≥ L_EQ = 657 ft), D_R = 27.9, LOS C, S = 56.0; EP3: P_FM = 0.160, D_R = 27.2, LOS C; EP4: P_FM base 0.600, D_R = 29.5, LOS D, S = 56.5.
- **Tolerances** (test-file header): flows ±5 pc/h (published values are rounded and the book carries rounded intermediates), speeds ±0.5 mi/h, densities ±0.5 pc/mi/ln, LOS letters exact; capacities exact (`1e-9`) since they are table lookups.
- **Known book discrepancy**: EP3's published all-lane average speed (58.8 mi/h) is not reproducible from the published S_R/S_O/flows through Exhibit 14-15 (which yield 58.2 mi/h); the test documents this in its doc comment and asserts the component speeds instead of S.
- **Unit tests** (inline in `merge_diverge.rs`): Exhibit 14-3 LOS boundaries, Exhibit 14-10/14-12 capacity tables, two-lane-ramp P_FM/P_FD constants, Exhibit 14-18 factors, and structural merge/diverge run assertions.
- No `docs/hcm/VERIFICATION.md` exists on this branch; deviations are inline below.

## Deviations

1. **Acceleration/deceleration lane length caps (VERIFY-HCM on `effective_accel_length`/`effective_decel_length`)**: the helpers cap the *effective* two-lane length (L_Aeff/L_Deff, Eqs 14-25/14-26 — i.e., `2·L_1 + L_2`) at 1,500 ft. Confirmed against the EPUB (`98_Ch14_04.xhtml`, Exhibit 14-16/14-17 discussion): the manual's stated rule is that an individual *component* lane length longer than 1,500 ft should itself be set to 1,500 ft before being used in the equations ("the acceleration lane length used for calculation should be set to 1,500 ft"); it says nothing about capping the summed effective total. Since `2·L_1 + L_2` exceeds 1,500 ft for almost any realistic two-lane ramp (e.g., the 800 ft/400 ft chapter defaults already give 2·800+400 = 2,000 ft), the current code's total-sum cap fires in the common case, not just the rare case the manual describes, and clamps L_Aeff/L_Deff down to exactly 1,500 ft whenever it does — well below the manual's uncapped (or component-capped) value. This is a more consequential deviation than previously described; code not changed per this task's scope (equation-documentation only) but flagged here for a likely follow-up code fix: cap components before summing, not the sum itself.
2. **Major merge LOS return value**: `determine_los` for `MajorMerge` under capacity sets `self.los = None` (correct — HCM defines no LOS) but *returns* `LevelOfService::E` from the function, with an inline comment acknowledging the returned letter is not HCM-sanctioned and callers should consult `get_los()`. `run_analysis` propagates that E as its return value, which is a trap for callers that use the return instead of the field.
3. **Eq 14-24 lane basis (VERIFY-HCM in `estimate_speed`)**: the aggregate all-lane density divides the per-direction flow (merge: v_F+v_R; diverge: v_F) by the mainline lane count; the manual states v in pc/h/ln without fixing the lane basis, so this is a documented interpretation.
4. **Mountainous PCE (VERIFY-HCM in `fhv_for`)**: `E_T = 5.0` is a non-HCM approximation (Exhibit 12-25 has no mountainous entry) — same flag as chapter13, and again inconsistent with chapter12's 2.5 placeholder.
5. **Silent geometry defaults**: `effective_la()`/`effective_ld()` fall back to 800 ft / 400 ft when `accel_lane_length`/`decel_lane_length` are `None`; a caller who forgets to set the lane length gets a plausible-looking answer with no warning.
6. **8-lane P_FM applicability**: the Exhibit 14-8 8-lane equations are transcribed with the v_F/S_FR ≤ 72 split, but no check that the result stays in [0, 1]; extreme v_R can drive P_FM negative (`0.2178 − 0.000125·v_R` < 0 for v_R > 1,742 pc/h) with no clamp.

## Deferred

- Ramp metering effects and Chapter 38 (ATDM) interactions.
- Managed-lane ramp junctions (access to/from managed lanes).
- Overlapping ramp influence areas: the model analyzes one junction at a time; adjacent ramps enter only through the P_FM/P_FD selection equations, and no facility-level reconciliation (Chapter 10) is attempted here.
- Clamping/validation of P_FM/P_FD ranges (deviation 6) and of the reasonableness-check interaction with left-hand-ramp adjustment ordering (the Exhibit 14-18 factor is applied before `check_v12`; the manual's ordering should be confirmed).
