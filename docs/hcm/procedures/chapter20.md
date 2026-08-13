# HCM Chapter 20 — Two-Way STOP-Controlled Intersections

This document walks the HCM 7th Edition Chapter 20 motorized-vehicle methodology for two-way STOP-controlled (TWSC) intersections as implemented on branch `feat/hcm-ch20-22-unsignalized`. The code follows Chapter 20, Section 3 ("Motorized Vehicle Core Methodology," the thirteen-step procedure of the module's own header comment, itself a transcription of HCM Exhibit 20-6) together with Section 4's pedestrian-impedance extension (Equations 20-67 through 20-75) and, since 0.3.2, Section 5's pedestrian mode (Equations 20-76 through 20-99, documented under "Section 5" below). The source files are `src/hcm/twsc/twsc.rs` (movement model, the `Twsc` facility struct, and the full Steps 1-13 pipeline), `src/hcm/twsc/pedestrian.rs` (the Section 5 pedestrian mode), and `src/hcm/twsc/tests.rs` (per-step unit tests against HCM Chapter 32 TWSC Example Problems 1 and 3). Gap-acceptance building blocks shared with Chapters 21-23 (potential capacity, queue-free probability, pedestrian blockage/impedance, movement capacity) live in `src/hcm/common/gap_acceptance.rs`; unsignalized control-delay and LOS-threshold primitives live in `src/hcm/common/delay.rs` and `src/hcm/common/los_tables.rs`. These three `common` modules are documented in depth separately in `common-infrastructure.md` and are only referenced here by function name. `docs/hcm/VERIFICATION.md` did not exist when this document was first written; it does now, and carries the consolidated book-discrepancy ledger. The deviations below remain cross-referenced to the `// VERIFY-HCM` code comments in `twsc.rs` and `pedestrian.rs`, which are the finer-grained record. This pass adds a complete equation-by-equation reference, cross-checked against both the Rust code and the HCM 7th Edition EPUB (Chapter 20 body, Chapter 30 Section 3, and Chapter 32 worked examples), for every equation the crate implements.

The public entry point is `Twsc::analyze(&mut self)`, which runs Steps 1-2, 3, 4, 5, 6-9, 10, 11, and 12 in sequence (Step 13, 95th-percentile queue, is folded into Step 11's per-movement and per-lane computation rather than broken out as a separate pass). Movement numbering follows HCM Exhibit 20-1 (module header table): major-street EB movements 1/2/3 (+ U-turn 1U), major-street WB movements 4/5/6 (+ 4U), minor-street NB (south leg) movements 7/8/9, minor-street SB (north leg) movements 10/11/12, and pedestrian movements 13-16 crossing the west, east, south, and north legs respectively. A three-leg (T) intersection is modeled by `TwscGeometry::is_three_leg`, which restricts the minor stem to movements 7 and 9 (the NB approach) per the T-intersection panel of Exhibit 20-1.

## Step-by-step walkthrough

| HCM step | Equations / Exhibits | Rust function(s) | Inputs (units) | Outputs (units) |
|---|---|---|---|---|
| Steps 1-2 — Movement priorities and demand flow rates | Equation 20-1 | `Twsc::step1_2_demand_flow_rates` | `TwscDemand.v1`..`v16` (veh/h or p/h), `phf` (unitless, `None` => 1.0) | `TwscMovementResult.flow_rate` (veh/h) per movement, and the total intersection flow rate |
| Step 3 — Conflicting flow rates | Equations 20-2 through 20-15; Exhibits 20-8, 20-10, 20-12, 20-14, 20-16 | `Twsc::step3_conflicting_flows` and its private helpers `f_minor_rt_vs_major_through`, `f_shared_half`, `f_channelized_zero`, `f_minor_lt_vs_major_through`, `f_uturn_vs_major_through`, `f_uturn_vs_major_right` | Movement flow rates (veh/h); `TwscGeometry.major_lanes_per_direction`, `major_right_turn_eb`/`wb` (`MajorRightTurnConfig`), `uturn_median_width` | `TwscMovementResult.conflicting_flow` (veh/h), plus `conflicting_flow_stage1`/`stage2` for the four two-stage-eligible movements (7, 8, 10, 11) |
| Step 4 — Critical and follow-up headways | Equations 20-16, 20-17; Exhibits 20-17, 20-18 | `Twsc::step4_headways`, `Twsc::critical_headway`, `Twsc::followup_headway`, private `base_critical_headway`/`base_followup_headway`/`tc_grade_factor`/`grade_for` | `heavy_vehicle_pct` (%), `major_lanes_per_direction`, per-leg grade (%), `is_three_leg` | `TwscMovementResult.critical_headway`/`followup_headway` (s), plus stage 1/2 critical headways for two-stage movements |
| Step 5 — Potential capacities | Equation 20-18 (base case); Equations 20-19 through 20-21 and Exhibit 20-19 (upstream-signal case) | `Twsc::step5_potential_capacities` calling `common::gap_acceptance::potential_capacity` (Step 5a) or `Twsc::potential_capacity_upstream_signal` (Step 5b, now wired via `platoon_blockage`); `PlatoonBlockage::total`/`stages` supply the Exhibit 20-19 mapping | Conflicting flow (veh/h), critical/follow-up headway (s); upstream-signal case additionally needs the analyst-supplied `platoon_blockage` proportions p_b,x and `major_lanes_per_direction` (N, for v_c,min = 1,000 N) | `TwscMovementResult.potential_capacity` (veh/h), plus stage 1/2 potential capacities |
| Steps 6-9 — Movement capacities (vehicular + pedestrian impedance) | Equations 20-22 through 20-48, 20-67 through 20-75 | `Twsc::step6_9_movement_capacities`, `Twsc::p0_major_left`, `Twsc::two_stage_adjustment_a`, `Twsc::two_stage_total_capacity`, `common::gap_acceptance::{movement_capacity, prob_queue_free, pedestrian_blockage_factor, pedestrian_impedance_factor}` | Potential capacities (veh/h), pedestrian flow rates v13-v16 (p/h), `lane_width_ft` (ft), median storage `median_storage_nb`/`sb` (veh) | `TwscMovementResult.movement_capacity` (veh/h) per Rank 2-4 movement, plus stage 1/2 movement capacities for two-stage movements |
| Step 7d — Shared/short major-left queue-free probability | Equations 20-29 through 20-34 (p\*_0,j substitution for shared/short major-street left lanes); Equation 20-30/20-32 (x_2+3, x_5+6) | `Twsc::p0_star_major_left`, `Twsc::prob_queue_free_shared_major`, `Twsc::major_through_saturation`, `Twsc::f_ll_major` (wired into `step6_9_movement_capacities`) | `MajorLeftLaneConfig` (`major_left_eb`/`major_left_wb` = `Exclusive`/`Shared`/`SharedShortPocket{storage_veh}`); major through/right flows (veh/h); default s = 1,800/1,500 veh/h | p\*_0,1+1U / p\*_0,4+4U substituted for p_0,j in every Rank 3/4 impedance product |
| Step 10 — Shared and flared minor-lane capacity | Equations 20-49, 20-50 (shared/flared minor lane); Equations 20-51 through 20-60 (shared major-street lane capacity c_SS, exposed as standalone helper) | `Twsc::step10_lane_capacities`, `Twsc::minor_lanes`, `Twsc::shared_lane_capacity`, `Twsc::flared_lane_capacity`; standalone `Twsc::shared_major_lane_capacity` (Step 10c, not wired into `analyze`) | Movement flow/capacity pairs (veh/h); `MinorLaneConfig`, `flare_storage_nb`/`sb` (veh) | `Twsc.lanes_nb`/`lanes_sb: Vec<TwscLaneResult>` with per-lane `capacity` (veh/h) |
| Step 11 — Movement and lane control delay, LOS, queue | Equation 20-61 (delay); Equations 20-62, 20-63 (Step 11b Rank 1 delay from shared/short major-left lanes, wired via `Twsc::rank1_delay` into `Twsc.rank1_major_delay`); Equation 20-66 (queue); Exhibit 20-2 (LOS) | `Twsc::step11_movement_delay`, `Twsc::compute_rank1_major_delay`, `Twsc::queue_95`, `common::delay::control_delay_unsignalized`, `common::los_tables::los_unsignalized` | Flow rate and capacity (veh/h) per movement/lane, `analysis_period_h` (h), `major_left_eb`/`wb` | `TwscMovementResult.control_delay`/`los`/`queue_95`; `TwscLaneResult.control_delay`/`los`/`queue_95`; `Twsc.rank1_major_delay: Option<[f64; 2]>` (d_2+3, d_5+6) |
| Step 12 — Approach and intersection control delay | Equations 20-64, 20-65 | `Twsc::step12_approach_intersection_delay`, `common::delay::aggregate_control_delay` | Per-movement/lane control delay and flow rate | `Twsc.approach_delays: [f64; 4]` (EB/WB/NB/SB), `Twsc.intersection_delay` |

Note that Exhibit 20-2 defines no overall LOS letter for a TWSC intersection (only for individual movements/lanes), which the code respects: `intersection_delay` is a plain seconds-per-vehicle number with no accompanying `los` field.

### Steps 1-2: demand flow rates

```
Equation 20-1:  v_i = V_i ÷ PHF                                              [veh/h]
  V_i = demand volume for movement i                                        [veh/h]
  PHF = peak hour factor for the intersection (defaults to 1.0 when `phf` is None or <= 0, treating volumes as already-adjusted flow rates)  [unitless]
Implemented in: twsc/twsc.rs::Twsc::step1_2_demand_flow_rates
```

The same PHF divides every one of the fourteen vehicular movements plus the four pedestrian movements 13-16 (the pedestrian counts are also demand-volume inputs on `TwscDemand`, divided by the same intersection-wide PHF as the vehicular movements per the HCM's single-PHF convention for Step 2).

### Step 3: conflicting flow rates and the `VERIFY-HCM` deviation

`step3_conflicting_flows` computes ten conflicting-flow expressions (Equations 20-2 through 20-15) directly from movement flow rates and the major-street right-turn/lane-count configuration, using small private "conflicting-flow factor" helpers rather than inlined literals, e.g. `f_minor_rt_vs_major_through` returns 1.0 for a one-lane major street and 0.5 for two or three lanes (Exhibit 20-10), and `f_uturn_vs_major_through` returns 0.73 for a three-lane major (Exhibit 20-12). A documented `VERIFY-HCM` block precedes the function: the HCM Chapter 32 TWSC Example Problem 3 worked values for movements 7, 8, 10, and 11's Stage II/Stage I conflicting flows use HCM 6th Edition equation forms (a factor of 1.0 rather than the 7th Edition's 0.5 on the relevant major-street right-turn term for movement 8's Stage II and movement 11's Stage I, and a different opposing-minor-through treatment for movements 7/10's Stage II). The implementation follows the 7th Edition exhibits as its default behavior; a `ConflictingFlowOverride` mechanism (movement label + stage + literal value) lets a caller substitute the published 6th-Edition-style numbers to reproduce Example Problem 3 exactly. `twsc/tests.rs::test_ep3_step3_default_exhibit_factors` explicitly checks the 7th-Edition default values against the overridden ones so both code paths are exercised.

All ten expressions share the general shape `v_c,x = Σ (f · v_conflicting) + v_ped`, a weighted sum of the vehicular movements that conflict with x plus the pedestrian movement(s) crossing x's path, where each weight f is a conflicting-flow factor of 0, 0.5, 0.73, 1, or 2 read off Exhibits 20-8/20-10/20-12/20-14/20-16 depending on lane configuration. One-stage movements (1, 4, 1U, 4U, 9, 12) use a single expression; the two-stage-eligible minor movements (7, 8, 10, 11) get separate Stage I and Stage II expressions whose sum is the one-stage total. The one-stage EB left turn is representative of the simplest family member:

```
Equation 20-2:  v_c,1 = v_5 + f(1,6)·v_6 + v_16                              [veh/h]
  v_5    = major-street WB through flow rate                                [veh/h]
  v_6    = major-street WB right-turn flow rate                             [veh/h]
  f(1,6) = 0 if movement 6 uses a STOP/YIELD-controlled channelized right-turn lane (Exhibit 20-8), else 1  [unitless]
  v_16   = pedestrian flow rate crossing the north minor leg                [p/h]
Implemented in: twsc/twsc.rs::Twsc::step3_conflicting_flows (f(1,6) via Twsc::f_channelized_zero); Equation 20-3 (v_c,4) mirrors this with the EB quantities and v_15
```

The minor-street right turns (9, 12) and major-street U-turns (1U, 4U) follow the same pattern with different factor sets (Equations 20-4/20-5, Exhibit 20-10; Equations 20-6/20-7, Exhibit 20-12), implemented by the same function via `f_minor_rt_vs_major_through`, `f_shared_half`, `f_uturn_vs_major_through`, and `f_uturn_vs_major_right`. The two-stage minor-through movement 8 is representative of the Stage I/II family (Exhibit 20-14):

```
Equation 20-8 (Stage I):   v_c,I,8  = 2·v_1 + 2·v_1U + v_2 + f(8,3)·v_3 + v_15   [veh/h]
Equation 20-10 (Stage II): v_c,II,8 = 2·v_4 + 2·v_4U + v_5 + f(8,6)·v_6 + v_16   [veh/h]
  v_1, v_1U = major-street EB left-turn / U-turn flow rate                       [veh/h]
  v_2       = major-street EB through flow rate                                 [veh/h]
  f(8,3)    = 0.5 if movement 3 shares a lane with the EB through movement (Exhibit 20-14), 0 if exclusive/channelized  [unitless]
  v_15, v_16 = pedestrian flow rate crossing the south / north minor leg         [p/h]
  v_c,8 (one-stage total) = v_c,I,8 + v_c,II,8
Implemented in: twsc/twsc.rs::Twsc::step3_conflicting_flows (f(8,3)/f(8,6) via Twsc::f_shared_half). The remaining Stage I/II pairs — Equations 20-9/20-11 (movement 11), 20-12/20-14 (movement 7), 20-13/20-15 (movement 10) — are the same weighted-sum pattern against Exhibit 20-16's factors and are implemented in the same function.
```

### Step 4: critical and follow-up headways

`base_critical_headway` and `base_followup_headway` transcribe Exhibits 20-17/20-18 as per-movement, per-lane-count match arms (e.g. major-street left turn 4.1 s for one/two major lanes, 5.3 s for three; minor-street through 6.5 s one-stage vs. 5.5 s per stage). `critical_headway` then applies Equation 20-16's `t_c = t_c,base + t_c,HV * P_HV + t_c,G * G - t_3,LT` with `t_c,HV` = 1.0 s for a one-lane major and 2.0 s for two/three lanes, `t_c,G` = 0.1 s for movements 9/12 and 0.2 s for movements 7/8/10/11, and the three-leg minor-left correction `t_3,LT` = 0.7 s. Two `VERIFY-HCM` comments flag places where Exhibit 20-17/20-18 lists "NA": U-turn critical/follow-up headway on a two-lane major street (the four-lane wide-median value 6.4 s / 2.5 s is used as a fallback if a caller codes a U-turn there anyway).

```
Equation 20-16:  t_c,x = t_c,base + t_c,HV·P_HV + t_c,G·G − t_3,LT           [s]
  t_c,base = base critical headway (Exhibit 20-17 match arm on movement × major-street through-lane count)  [s]
  t_c,HV   = heavy-vehicle adjustment: 1.0 for a one-lane major street, 2.0 for two/three lanes             [s]
  P_HV     = proportion heavy vehicles = heavy_vehicle_pct ÷ 100                                            [unitless]
  t_c,G    = grade adjustment: 0.1 for movements 9/12, 0.2 for movements 7/8/10/11, 0 otherwise             [s]
  G        = signed approach grade for the minor leg the movement is on (negative = downhill)               [%]
  t_3,LT   = three-leg minor-left correction: 0.7 for movements 7/10 at a T intersection, 0 otherwise        [s]
Implemented in: twsc/twsc.rs::Twsc::critical_headway (base value via Twsc::base_critical_headway, grade via Twsc::grade_for)
```

```
Equation 20-17:  t_f,x = t_f,base + t_f,HV·P_HV                              [s]
  t_f,base = base follow-up headway (Exhibit 20-18 match arm)                [s]
  t_f,HV   = heavy-vehicle adjustment: 0.9 for a one-lane major street, 1.0 for two/three lanes  [s]
  P_HV     = proportion heavy vehicles                                       [unitless]
Implemented in: twsc/twsc.rs::Twsc::followup_headway (base value via Twsc::base_followup_headway)
```

### Step 5a: potential capacity (gap-acceptance base case)

```
Equation 20-18:  c_p,x = v_c,x · exp(−v_c,x·t_c,x ÷ 3,600) ÷ (1 − exp(−v_c,x·t_f,x ÷ 3,600))   [veh/h]
  v_c,x = conflicting flow rate for movement x (Step 3)                      [veh/h]
  t_c,x = critical headway for movement x (Equation 20-16)                   [s]
  t_f,x = follow-up headway for movement x (Equation 20-17)                  [s]
  Limit v_c,x -> 0: c_p,x -> 3,600 ÷ t_f,x, returned explicitly for v_c,x <= 0 rather than evaluated as a 0/0 limit  [veh/h]
Implemented in: common/gap_acceptance.rs::potential_capacity, called from twsc/twsc.rs::Twsc::step5_potential_capacities for every Rank 2-4 movement (and, for two-stage movements, separately for each stage's own conflicting flow and critical headway)
```

### Step 5b: upstream-signal platoon blockage

When the TWSC intersection sits between coordinated upstream signals, arrivals on the major street are platooned and the conflicting stream alternates between blocked and unblocked periods (HCM Step 5b). `step5_potential_capacities` selects this path per movement whenever the optional `platoon_blockage: Option<PlatoonBlockage>` input is present with a nonzero proportion for that movement, otherwise it uses the plain Equation 20-18 gap-acceptance capacity. The switch is per movement and per stage, so a partially-populated `PlatoonBlockage` mixes platooned and non-platooned movements, and an all-zero (or `None`) input reduces the whole step to Equation 20-18 bit-for-bit — `test_platoon_off_equivalence` asserts that `None` and `Some(PlatoonBlockage::default())` yield identical pipeline output for Example Problem 3.

`PlatoonBlockage` carries eight analyst-supplied proportions (p_b,1, p_b,4, p_b,7, p_b,8, p_b,9, p_b,10, p_b,11, p_b,12). `PlatoonBlockage::total` and `PlatoonBlockage::stages` encode the Exhibit 20-19 mapping (confirmed against the EPUB table, whose column headers are "One-Stage Movements" / "Two-Stage Movements: Stage I" / "Two-Stage Movements: Stage II" and whose rows are "1, 1U" / "4, 4U" / "7" / "8" / "9" / "10" / "11" / "12"; only the mapping structure is reproduced here, not the exhibit's data cells): movements 1U/4U reuse p_b,1/p_b,4; the one-stage total for each movement is its own p_b; and the two-stage movements draw their Stage I/Stage II proportions from the opposing major-street left-turn direction (movements 7 and 8 use Stage I = p_b,4, Stage II = p_b,1; movements 10 and 11 mirror to Stage I = p_b,1, Stage II = p_b,4).

```
Equation 20-19:  v_c,u,x = (v_c,x − 1.5·v_c,min·p_b,x) ÷ (1 − p_b,x)   if v_c,x > 1.5·v_c,min·p_b,x, else v_c,u,x = 0   [veh/h]
  v_c,x   = total conflicting flow rate for movement x (Step 3)              [veh/h]
  v_c,min = minimum platooned conflicting flow rate ≈ 1,000·N                [veh/h]
  N       = major-street through lanes per direction (major_lanes_per_direction)  [lanes]
  p_b,x   = proportion of the analysis period movement x is blocked by the platoon (Exhibit 20-19)  [unitless]
Implemented in: twsc/twsc.rs::Twsc::potential_capacity_upstream_signal
```

```
Equation 20-21:  c_r,x = v_c,u,x · exp(−v_c,u,x·t_c,x ÷ 3,600) ÷ (1 − exp(−v_c,u,x·t_f,x ÷ 3,600))   if v_c,u,x > 0, else c_r,x = 3,600 ÷ t_f,x   [veh/h]
  v_c,u,x = unblocked-period conflicting flow (Equation 20-19)               [veh/h]
  t_c,x, t_f,x = critical / follow-up headway for movement x                 [s]
Implemented in: twsc/twsc.rs::Twsc::potential_capacity_upstream_signal (the v_c,u,x > 0 branch reuses common/gap_acceptance.rs::potential_capacity)
```

```
Equation 20-20:  c_p,x = (1 − p_b,x) · c_r,x                                  [veh/h]
  p_b,x = proportion of time blocked (as above)                              [unitless]
  c_r,x = random-flow capacity of the unblocked period (Equation 20-21)      [veh/h]
Implemented in: twsc/twsc.rs::Twsc::potential_capacity_upstream_signal
```

The proportions can be supplied directly (e.g. from the Chapter 30 movement-based access-point output, Exhibit 32-12, which reports p_b and delay-to-through-vehicles for a specific worked example rather than serving as a generic definitional table — Exhibit 20-19 is the generic p_b mapping) or computed from upstream-signal descriptors as described next.

### Step 5b input: computed proportion of time blocked (HCM Chapter 30, Section 3)

When the analyst supplies coordinated upstream-signal descriptors instead of the p_b values themselves, `Twsc::analyze` derives them via the HCM Chapter 30, Section 3 "Proportion of Time Blocked" procedure (EPUB `235_Ch30_03.xhtml` — Equation 30-13 and neighboring Equations 30-9 through 30-12 appear in this same file), implemented in `src/hcm/twsc/computed_pb.rs`. The new input is `upstream_signals: Option<UpstreamSignals>`, holding the system cycle length C and up to two `UpstreamSignal` descriptors (`eastbound`, feeding the movement-2 through-lane group; `westbound`, feeding movement 5). Each descriptor gives the segment length (`distance_ft`), progression speed (`progression_speed_mph`, which fixes the running time t_R that drives dispersion), an optional uniform midblock volume, and the upstream `discharges` (a list of `MovementDischarge` profiles reused from `chapter18::platoon_dispersion`). `analyze` calls `UpstreamSignals::compute_platoon_blockage` at the top of the pipeline (critical headways depend only on geometry and heavy-vehicle percentage, so this precedes Step 4), which:

1. Builds each direction's combined arrival flow profile at the TWSC intersection by dispersing the upstream discharge profiles over the segment (`combined_arrival_profile`, Equations 30-9 through 30-12 — the Robertson-style platoon-dispersion recursion `q'_a|u,j = F·q'_u,i + (1-F)·q'_a|u,j-1` with smoothing factor `F = 1/(1 + 0.138 t'_R + 0.315/d_t)` and platoon arrival time `t' = t'_R - 1/F + 1.25`, where `t'_R` is the segment running time in time steps; this dispersion machinery lives in `urban_segments::platoon_dispersion` and is only referenced here by function name, per this document's read-only scope on that module).
2. For each Rank 2-4 movement, forms the critical platoon flow rate q_c = 3,600 / t_c from the Chapter 20 critical headway (`Twsc::critical_headway`) and counts the cycle steps whose arrival flow rate exceeds q_c — the blocked period duration t'_p (`blocked_period_steps`).
3. Applies Equation 30-13, p_b = t'_p d_t / C (`proportion_time_blocked_from_profile`).

```
Equation 30-13:  p_b = t'_p · d_t ÷ C                                        [unitless, clamped to [0,1]]
  t'_p = blocked-period duration: count of cycle time steps whose arrival flow rate exceeds the critical platoon flow rate q_c = 3,600 ÷ t_c  [steps]
  d_t  = flow-profile time step (Chapter 30 recommends 1.0)                  [s/step]
  C    = system cycle length shared by the coordinated signals               [s]
  q_c  = critical platoon flow rate = 3,600 ÷ t_c (t_c = the movement's Chapter 20 critical headway); above q_c, platoon headways are too short to enter or cross  [veh/h]
Implemented in: twsc/computed_pb.rs::proportion_time_blocked_from_profile (single through-lane group), twsc/computed_pb.rs::union_proportion_time_blocked (minor-street left/through movements, union of both directions' blocked steps); q_c via twsc/computed_pb.rs::q_c
```

The through-lane group evaluated per movement follows the Chapter 20 conflict equations: movement 1 (EB left) and movement 12 (SB right) read the westbound profile (their v_c involves v_5); movement 4 (WB left) and movement 9 (NB right) read the eastbound profile (v_2); the minor-street left and through movements 7, 8, 10, 11 are blocked when a platoon is present from either direction, so their p_b is the union of the two directions' blocked steps. Only the one-stage totals (`pb1`, `pb4`, `pb7`, `pb8`, `pb9`, `pb10`, `pb11`, `pb12`) are populated; the two-stage Stage I/II proportions are still derived from `pb1`/`pb4` by `PlatoonBlockage::stages` per Exhibit 20-19. Section 3 does not use v_c,min in the blocked-period computation (that threshold belongs to the downstream Step 5b Equation 20-19); the blocked period is governed solely by q_c. An explicit `platoon_blockage` always takes precedence over `upstream_signals`, and absent/empty descriptors yield all-zero p_b, so the analyst-input and no-platooning pipelines are bit-identical.

### Steps 6-9: the impedance chain

This is the heart of the Rank 2/3/4 dependency structure. Rank 2 movements (major-street left turns 1/4, U-turns 1U/4U, minor-street right turns 9/12) either have unimpeded potential capacity (major left turns, `assign(self, Mv::M1, pp16)` applies only the pedestrian factor) or are impeded by the queue-free probability of another Rank 2 movement (U-turn 1U is impeded by `p0_12 = prob_queue_free(flow(M12), capacity(M12))`, i.e. Equations 20-24/20-26).

```
Equation 20-22:  c_m,j = c_p,j                                                [veh/h]  (Rank 2 major-street left turn, unimpeded by vehicular movements)
  c_p,j = potential capacity from Step 5 (Equation 20-18)                    [veh/h]
  In code the vehicular factor of 1.0 is combined with the pedestrian factor of Equation 20-69: c_m,1 = c_p,1 × p_p,16, c_m,4 = c_p,4 × p_p,15 (Exhibit 20-22 mapping)
Implemented in: twsc/twsc.rs::Twsc::step6_9_movement_capacities (assign(M1, pp16), assign(M4, pp15))
```

```
Equation 20-23 / 20-70 / 20-71 / 20-72:  c_m,j = c_p,j × f_j,  f_9 = p_p,15 × p_p,14,  f_12 = p_p,16 × p_p,13   [veh/h]
  p_p,x = pedestrian impedance factor of the conflicting pedestrian movement x (Equation 20-68)  [unitless]
Implemented in: twsc/twsc.rs::Twsc::step6_9_movement_capacities (assign(M9, pp15*pp14), assign(M12, pp16*pp13))
```

```
Equation 20-24 / 20-25:  f_1U = p_0,12 = 1 − v_12/c_m,12,   f_4U = p_0,9 = 1 − v_9/c_m,9   [unitless]
  v_12, v_9     = flow rate of the companion minor-street right turn                [veh/h]
  c_m,12, c_m,9 = movement capacity of the companion minor-street right turn        [veh/h]
Equation 20-26:  c_m,jU = c_p,jU × f_jU                                              [veh/h]
Implemented in: twsc/twsc.rs::Twsc::step6_9_movement_capacities (p0_12, p0_9, assign(M1U, p0_12), assign(M4U, p0_9)); common/gap_acceptance.rs::prob_queue_free
```

`p0_major_left` computes the Equation 20-28 queue-free probability `p_0 = 1 - v/c_m` for the combined major left + U-turn flow using the shared-lane capacity form (Equation 20-27) when the U-turn shares a lane with the left turn:

```
Equation 20-27:  c_SH = Σ v_y ÷ Σ (v_y ÷ c_m,y)                                    [veh/h]
  v_y, c_m,y = flow rate / movement capacity of each movement y sharing the lane  [veh/h]
  (algebraically the same harmonic-mean form as the general shared-lane Equation 20-49; the HCM restates it here specifically for the major-street left-turn + U-turn pair sharing one lane, immediately ahead of Equation 20-28)
Equation 20-28:  p_0,j = 1 − v_j ÷ c_m,j                                           [unitless, clamped to [0,1]]
  j = 1+1U (EB) or 4+4U (WB), using the shared-lane v_j/c_m,j of Equation 20-27 when the U-turn shares the left-turn lane, else the plain left-turn v/c
Implemented in: twsc/twsc.rs::Twsc::p0_major_left (Eq 20-27 via Twsc::shared_lane_capacity); common/gap_acceptance.rs::prob_queue_free (Eq 20-28)
```

Rank 3 movements (minor-street through, 8/11, at a four-leg intersection; minor-street left, 7, at a T) multiply their potential capacity by the product of the impeding Rank 2 queue-free probabilities and pedestrian impedance factors:

```
Equation 20-35:  f_k = Π p_0,j                                                     [unitless]
  p_0,j = queue-free probability of each impeding Rank 2 movement j (Equation 20-28, or p*_0,j of Equations 20-29..20-34 when the major left shares/short-pockets)
Equation 20-36 (extended with pedestrian terms to Equation 20-73):  c_m,k = c_p,k × f_k × p_p,x   [veh/h]
Implemented in: common/gap_acceptance.rs::vehicular_impedance_factor, movement_capacity (the literal Π p_0,j form); inlined directly at the Rank 3/4 call sites in twsc/twsc.rs::Twsc::step6_9_movement_capacities as f_r3 = p0_1*p0_4*pp15*pp16 (movements 8/11, four-leg) and f7 = p0_4*pp15*pp13 (movement 7 at a T)
```

Two-stage movements (7, 8, 10, 11 when `median_storage_nb`/`sb` is `Some(n) > 0` and the intersection is not three-leg) get Stage I and Stage II movement capacities from the same impedance chain applied to each stage's own potential capacity, then combine them via `two_stage_total_capacity`, which transcribes Equations 20-38/20-39/20-40 (Rank 3) and 20-46/20-47/20-48 (Rank 4):

```
Equation 20-37 / 20-45:  a = 1 − 0.32 · exp(−1.3·√n_m)                              [unitless]
  n_m = median storage, vehicles (median_storage_nb / median_storage_sb)          [veh]
Implemented in: twsc/twsc.rs::Twsc::two_stage_adjustment_a — verified in twsc/tests.rs::test_two_stage_total_capacity_equation against the Chapter 32 Example Problem 3 value a = 0.949 at n_m = 2
```

```
Equation 20-38 / 20-46:  y = (c_I − c_m,x) ÷ (c_II − v_L − c_m,x)                   [unitless]
  c_I    = Stage I movement capacity                                              [veh/h]
  c_II   = Stage II movement capacity                                             [veh/h]
  c_m,x  = one-stage movement capacity of the subject movement (Equation 20-36 for Rank 3, Equation 20-43/20-44 for Rank 4)  [veh/h]
  v_L    = major-street left-turn + U-turn conflicting flow (v_1+v_1U for movements 8/7, v_4+v_4U for movements 11/10)      [veh/h]
Implemented in: twsc/twsc.rs::Twsc::two_stage_total_capacity
```

```
Equation 20-39 / 20-47 (y ≠ 1):  c_T = a ÷ (y^(n_m+1) − 1) · [y·(y^n_m − 1)·(c_II − v_L) + (y − 1)·c_m,x]   [veh/h]
Equation 20-40 / 20-48 (y = 1):  c_T = a ÷ (n_m + 1) · [n_m·(c_II − v_L) + c_m,x]                            [veh/h]
Implemented in: twsc/twsc.rs::Twsc::two_stage_total_capacity, which selects the y = 1 branch when `(y - 1.0).abs() < 1e-9` as a numerically distinct case from the general y != 1 closed form
```

Rank 4 movements (minor-street left, 7/10, at a four-leg intersection) use the more elaborate "dependent" combinatorial form of Equations 20-41/20-42 (extended with the Exhibit 20-23/20-24 pedestrian terms to 20-74/20-75):

```
Equation 20-41 / 20-74:  f_p,7  = [1 ÷ (1÷(p_0,1+1U·p_0,4+4U) + 1÷p_0,11 − 1)] · p_0,12 · p_p,15 · p_p,13   [unitless]
Equation 20-42 / 20-75:  f_p,10 = [1 ÷ (1÷(p_0,1+1U·p_0,4+4U) + 1÷p_0,8  − 1)] · p_0,9  · p_p,16 · p_p,14   [unitless]
  p_0,1+1U, p_0,4+4U = queue-free probability of the major-street left+U-turn movements (p*_0,j when the lane is shared/short-pocket)  [unitless]
  p_0,11 / p_0,8     = queue-free probability of the opposing crossing minor-street through movement       [unitless]
  p_0,12 / p_0,9     = queue-free probability of the conflicting minor-street right turn                    [unitless]
  p_p,x              = pedestrian impedance factor of the relevant crossing pedestrian movements (Equation 20-68)  [unitless]
Equation 20-43 / 20-44:  c_m,7 = c_p,7 × f_p,7,   c_m,10 = c_p,10 × f_p,10                                    [veh/h]
Implemented in: twsc/twsc.rs::Twsc::step6_9_movement_capacities (closure `dependent(p_majors, p_cross) = 1/(1/p_majors + 1/p_cross - 1)`, then fp7, fp10)
```

`dependent(p_majors, p_cross) = 1 / (1/p_majors + 1/p_cross - 1)`, then multiplied by the queue-free probability of the opposing minor-street right turn and both relevant pedestrian factors, as above. For the two-stage Rank 4 movements the code extends this with a Stage II impedance term `p_ii` that additionally incorporates the opposing crossing movement's own Stage I queue-free probability (`p0_i_cross`), mirroring the worked procedure of HCM Chapter 32 Example Problem 3 rather than a single boxed equation number; this is implemented in the same function (`step6_9_movement_capacities`, the "Step 9b" block) and is exercised by the Example Problem 3 fixture.

### Step 7d: shared/short major-street left-turn queue-free probability

When a major-street left turn shares the adjacent through lane or uses a short storage pocket, a left-turning vehicle waiting for a gap can block the Rank 1 through/right traffic behind it, so the queue-free probability the impedance chain sees is p\*_0,j (Equations 20-29 through 20-34), not the exclusive-lane p_0,j (Equation 20-28). `MajorLeftLaneConfig` (`major_left_eb`/`major_left_wb`, default `Exclusive`) marks each major approach as `Exclusive`, `Shared` (n_L = 0), or `SharedShortPocket{storage_veh}` (n_L > 0). `step6_9_movement_capacities` computes the exclusive-lane p_0,1+1U and p_0,4+4U as before, then `p0_star_major_left` substitutes p\*_0,j = `prob_queue_free_shared_major(p_0, x, n_L)` whenever the config is not `Exclusive`.

```
Equation 20-30 / 20-32:  x_2+3 = f_LL,2+3 · (v_2/s_2 + v_3/s_3),   x_5+6 = f_LL,5+6 · (v_5/s_5 + v_6/s_6)   [unitless]
  f_LL,2+3, f_LL,5+6 = portion of through/right traffic using the left lane: 1.0 (N=1), 0.5 default (N=2), 0.33 default (N=3)  [unitless]
  v_2, v_5 = major-street through flow rate                                        [veh/h]
  v_3, v_6 = major-street right-turn flow rate (0 with an exclusive right-turn lane)  [veh/h]
  s_2, s_5 = major-street through saturation flow rate, default 1,800               [veh/h]
  s_3, s_6 = major-street right-turn saturation flow rate, default 1,500            [veh/h]
Implemented in: twsc/twsc.rs::Twsc::major_through_saturation (f_LL via Twsc::f_ll_major; constants MAJOR_THROUGH_SAT_FLOW = 1,800.0, MAJOR_RIGHT_SAT_FLOW = 1,500.0)
```

```
Equation 20-29 / 20-31 (n_L > 0, short pocket, general root form):
  p*_0,1+1U = 1 − (1 − p_0,1+1U) · [1 + x_2+3^(n_L+1) ÷ (1 − x_2+3)]^(1/(n_L+1))
  p*_0,4+4U = 1 − (1 − p_0,4+4U) · [1 + x_5+6^(n_L+1) ÷ (1 − x_5+6)]^(1/(n_L+1))
Equation 20-33 / 20-34 (n_L = 0, shared lane, reduced form):
  p*_0,1+1U = 1 − (1 − p_0,1+1U) ÷ (1 − x_2+3)
  p*_0,4+4U = 1 − (1 − p_0,4+4U) ÷ (1 − x_5+6)
  p_0,j = exclusive-lane queue-free probability (Equation 20-28)                    [unitless]
  n_L   = vehicles storable in the left-turn pocket (0 = shared lane, Exhibit 20-20)  [veh]
  x_2+3 / x_5+6 = combined degree of saturation (Equation 20-30 / 20-32)             [unitless]
  Result clamped to [0,1].
Implemented in: twsc/twsc.rs::Twsc::prob_queue_free_shared_major, a single root-form expression that reduces exactly to the n_L = 0 case (the bracket becomes 1/(1-x)); called from Twsc::p0_star_major_left
```

The combined degree of saturation x_2+3 / x_5+6 (Equations 20-30/20-32) uses `f_ll_major` (1.0/0.5/0.33 for one/two/three through lanes) and the default saturation flow rates s = 1,800 (through) / 1,500 (right) veh/h. The substituted p\*_0,j then propagates into every Rank 3 and Rank 4 impedance product (movements 7/8/10/11) exactly where p_0,1/p_0,4 appear. HCM Chapter 32 Example Problem 4 (case3.json, shared major lefts) exercises this path: x_2+3 = 0.304, p\*_0 = 0.856, giving c_m,7 = c_m,10 = 47 veh/h (`test_p0_star_shared_major_ep4_value`, `test_major_left_shared_wiring`). For `Exclusive` (the default) no substitution occurs and behavior is bit-identical to before.

### Step 10: shared and flared minor-lane capacity

`minor_lanes` dispatches on `MinorLaneConfig` (single shared lane, shared-left-through + exclusive-right, exclusive-left + shared-through-right, or fully separate lanes) and, for a single shared lane with `flare_storage > 0`, calls `flared_lane_capacity` (Equation 20-50), whose `n_R = 0` case is verified in `test_flared_lane_capacity_equation` to reduce exactly to `shared_lane_capacity` (Equation 20-49, the harmonic-mean-like form `c_SH = Σv_y / Σ(v_y/c_m,y)`).

```
Equation 20-49:  c_SH = Σ v_y ÷ Σ (v_y ÷ c_m,y)                                       [veh/h]
  v_y, c_m,y = flow rate / movement capacity of each movement y sharing the lane      [veh/h]
  (empty-shared-lane fallback in code: if total demand is 0, returns the minimum of the individual movement capacities rather than dividing 0/0)
Implemented in: twsc/twsc.rs::Twsc::shared_lane_capacity
```

```
Equation 20-50:  c_F = (v_R + v_L+TH) ÷ [(v_R/c_R)^(n_R+1) + (v_L+TH/c_L+TH)^(n_R+1)]^(1/(n_R+1))   [veh/h]
  v_R, c_R       = flow rate / capacity of the right-turn movement                     [veh/h]
  v_L+TH, c_L+TH = combined flow rate / shared-lane capacity of the left+through movements  [veh/h]
  n_R            = storage spaces in the flared portion of the approach (Exhibit 20-21)  [veh]
  For n_R = 0 this reduces exactly to Equation 20-49
Implemented in: twsc/twsc.rs::Twsc::flared_lane_capacity
```

The shared/short major-street left-turn queue-free probability `prob_queue_free_shared_major` (Equations 20-29 through 20-34) is now wired into the impedance chain through Step 7d (see above). The remaining shared-major-lane helper `shared_major_lane_capacity` (Step 10c, Equations 20-51 through 20-60, the reduced through-lane capacity c_SS when left turns share or short-pocket into the through lane) is implemented and unit-tested (`test_shared_major_lane_capacity`) but is not called from `step10_lane_capacities` or `analyze`: it models the capacity of the shared major-street through lane itself, which neither reported fixture output depends on (in Example Problem 4 it evaluates to the s_2+3 saturation-flow bound and does not feed the minor-street results), so it exists as a standalone, independently-verified building block a caller can invoke directly.

```
Equation 20-53 / 20-58:  x_1+1U = v_1+1U ÷ c_m,1+1U,   x_4+4U = v_4+4U ÷ c_m,4+4U     [unitless]
Equation 20-54 / 20-59:  x_2+3 = f_LL,2+3·(v_2/s_2+v_3/s_3),  x_5+6 = f_LL,5+6·(v_5/s_5+v_6/s_6)   [unitless]  (identical form to Equation 20-30/20-32)
Equation 20-52 / 20-57:  x_1+1U+2+3 = x_1+1U · [1 + x_2+3^(n_L+1)/(1−x_2+3)]^(1/(n_L+1))            [unitless]
Equation 20-55 / 20-60:  s_2+3 = (v_2+v_3) ÷ (v_2/(N·s_2) + v_3/s_3)                                [veh/h]
Equation 20-51 / 20-56:  c_SS = min[(v_1+1U+v_2+v_3) ÷ x_1+1U+2+3, s_2+3]                            [veh/h]
  N = major-street through lanes per direction                                                      [lanes]
  n_L = vehicles storable in the left-turn pocket                                                   [veh]
  The WB side (Equations 20-56 through 20-60) mirrors this chain with 4+4U/5+6 in place of 1+1U/2+3.
Implemented in: twsc/twsc.rs::Twsc::shared_major_lane_capacity (standalone; not called from step10_lane_capacities or analyze — see Deferred)
```

### Step 11a: control delay, LOS, and queue

Movement/lane control delay uses `common::delay::control_delay_unsignalized` (Equation 20-61, the standard `d = 3,600/c + 900T[...] + 5` form) uniformly for major-street left/U-turn movements and for the shared/flared minor lanes.

```
Equation 20-61:  d_x = 3,600÷c_m,x + 900·T·[(v_x/c_m,x − 1) + √((v_x/c_m,x − 1)² + (3,600/c_m,x)(v_x/c_m,x)/(450·T))] + 5   [s/veh]
  c_m,x = movement (or lane) capacity                                              [veh/h]
  v_x   = movement (or lane) demand flow rate                                      [veh/h]
  T     = analysis period (analysis_period_h, default 0.25)                        [h]
  +5    = deceleration-to and acceleration-from-the-stop term                      [s]
Implemented in: common/delay.rs::control_delay_unsignalized, called from twsc/twsc.rs::Twsc::step11_movement_delay for both exclusive-lane movement delay and per-lane (shared/flared) delay; the same algebraic form serves AWSC (Equation 21-30) and roundabouts (Equation 22-17) via the same shared function
```

LOS follows Exhibit 20-2 ("LOS Criteria: Motorized Vehicle Mode"), applied per movement/lane on the minor street and to the major-street left/U-turn movements, with v/c > 1.0 forcing LOS F regardless of the delay thresholds:

```
common/los_tables.rs::los_unsignalized(control_delay_s, vc_gt_1): thresholds A <= 10, B <= 15, C <= 25, D <= 35, E <= 50, F > 50 s/veh (Exhibit 20-2), overridden to F whenever vc_gt_1 is true
```

```
Equation 20-66:  Q_95 ≈ 900·T·[(v_x/c_m,x − 1) + √((v_x/c_m,x − 1)² + (3,600/c_m,x)(v_x/c_m,x)/(150·T))] · (c_m,x ÷ 3,600)   [veh]
  Same v_x, c_m,x, T as Equation 20-61; the radicand denominator is 150·T here versus 450·T in Equation 20-61 — confirmed against the EPUB as a genuine difference between the two formulas, not a transcription artifact
Implemented in: twsc/twsc.rs::Twsc::queue_95
```

### Step 11b: Rank 1 delay from shared/short major-left lanes

When a major approach has a shared or short left-turn lane, `step11_movement_delay` also computes the Step 11b Rank 1 delay via `compute_rank1_major_delay`, which calls `Twsc::rank1_delay` (Equations 20-62/20-63) for each of the EB (d_2+3) and WB (d_5+6) approaches and stores `[d_2+3, d_5+6]` in `Twsc.rank1_major_delay` (`None` when both major lefts are exclusive).

```
Equation 20-62 (N > 1):  d_2+3 = [(1 − p*_0,1+1U)·f_LL,2+3·(v_2+v_3)] ÷ [v_1+1U + f_LL,2+3·(v_2+v_3)] · d_1+1U   [s/veh]
Equation 20-62 (N = 1):  d_2+3 = (1 − p*_0,1+1U) · d_1+1U                                                        [s/veh]
Equation 20-63:  d_5+6 mirrors d_2+3 with the WB quantities (4+4U/5+6 in place of 1+1U/2+3)                       [s/veh]
  p*_0,1+1U / p*_0,4+4U = shared/short-pocket queue-free probability (Equations 20-29..20-34)  [unitless]
  v_1+1U / v_4+4U       = major-street left-turn + U-turn flow rate in the shared lane          [veh/h]
  v_2,v_3 / v_5,v_6     = major-street through / right-turn flow rate                            [veh/h]
  d_1+1U / d_4+4U       = control delay to the major-street left+U-turn movement (Equation 20-61)  [s/veh]
  f_LL,2+3 / f_LL,5+6   = portion of through/right traffic in the left lane                       [unitless]
  N                     = major-street through lanes per direction                                [lanes]
Implemented in: twsc/twsc.rs::Twsc::rank1_delay, orchestrated per approach by Twsc::compute_rank1_major_delay
```

`step12_approach_intersection_delay` then charges the Rank 1 major-street through/right movements (2/3, 5/6) this shared-lane delay in Equation 20-64 instead of zero. This matches the HCM Step 12 rule that Rank 1 delay is zero *with an exclusive left-turn lane* while a shared/short pocket produces the nonzero d_2+3/d_5+6 the worked Example Problem 4 uses (published d_2+3 = d_5+6 = 1.3 s; `test_rank1_delay_ep4_value`, and the integration test's d_A,EB = d_A,WB = 1.9 s and d_I = 34.1 s). For exclusive major lefts (the default) `rank1_major_delay` is `None` and the EB/WB Rank 1 movements keep zero delay, bit-identical to before.

### Step 12: approach and intersection control delay

```
Equation 20-64:  d_A,x = Σ (d_i,x · v_i,x) ÷ Σ v_i,x                                [s/veh]
  d_i,x = control delay of movement/lane i on approach x                            [s/veh]
  v_i,x = flow rate of movement/lane i on approach x                                [veh/h]
  Rank 1 major-street through/right movements carry 0 s/veh with an exclusive left-turn lane, or the Step 11b d_2+3/d_5+6 (Equations 20-62/20-63) with a shared/short-pocket left-turn lane
Equation 20-65:  d_I = Σ (d_A,x · v_A,x) ÷ Σ v_A,x, over the four approaches (EB, WB, NB, SB)   [s/veh]
  d_A,x, v_A,x = approach delay / approach flow rate from Equation 20-64             [s/veh, veh/h]
Implemented in: common/delay.rs::aggregate_control_delay (flow-rate-weighted average, `Σ(d·v)/Σv`, zero-volume guard returns 0.0), called from twsc/twsc.rs::Twsc::step12_approach_intersection_delay for both the per-approach (Eq 20-64) and intersection-level (Eq 20-65) aggregation
```

Exhibit 20-2 defines no overall LOS letter for the intersection as a whole, so `intersection_delay` is stored without an accompanying LOS field (see the note after the step table above).

### Section 4: pedestrian impedance

```
Equation 20-67:  f_pb = (v_x · w ÷ S_p) ÷ 3,600                                     [unitless]
  v_x = pedestrian flow rate for the conflicting pedestrian movement (13-16)         [p/h]
  w   = width of the lane the minor movement negotiates into (lane_width_ft, default 12)  [ft]
  S_p = pedestrian walking speed, assumed 3.5 (PEDESTRIAN_WALKING_SPEED_FT_S)        [ft/s]
Equation 20-68:  p_p,x = 1 − f_pb                                                    [unitless, clamped to [0,1]]
Implemented in: common/gap_acceptance.rs::pedestrian_blockage_factor (Eq 20-67), pedestrian_impedance_factor (Eq 20-68); called from twsc/twsc.rs::Twsc::ped_impedance for each of pp13, pp14, pp15, pp16
```

Equations 20-69 through 20-75 apply this pedestrian impedance factor p_p,x on top of the corresponding vehicle-only capacity equation, mapping each vehicular movement to its conflicting pedestrian movement(s) per Exhibits 20-22 (Rank 2 major left, movements 1/4), 20-23 (Rank 2 minor right, movements 9/12, a product of two pedestrian factors), and 20-24 (Rank 3/4 minor left/through, movements 7/8/10/11, likewise a product of two). Concretely: Equation 20-69 (`c_m,j = c_p,j × p_p,i`) is Equation 20-22 with the pedestrian factor attached, implemented at the same `assign(M1, pp16)` / `assign(M4, pp15)` call sites documented above; Equations 20-70/20-71 (`f_9 = p_p,15 × p_p,14`, `f_12 = p_p,16 × p_p,13`) and 20-72 (`c_m,j = c_p,j × f_j`) are the minor-right-turn factors documented above under Equation 20-23; Equation 20-73 (`f_k = Π p_0,j × p_p,x`) is Equation 20-35/20-36 with the pedestrian factor folded in, implemented as `f_r3 = p0_1 * p0_4 * pp15 * pp16` (movements 8/11) and `f7 = p0_4 * pp15 * pp13` (movement 7 at a T), documented above under Steps 6-9; and Equations 20-74/20-75 are Equations 20-41/20-42 with the pedestrian factor folded in, documented above under Steps 6-9's Rank 4 discussion. No separate implementation exists for the pedestrian-only forms of these particular equations — the crate always evaluates the combined vehicular-times-pedestrian form directly, since `pp13`..`pp16` default to 1.0 (no impedance) whenever the corresponding pedestrian demand is zero. That is a statement about Section 4 only. Section 5, below, is a different procedure with its own module.

### Section 5: pedestrian mode

Section 4 and Section 5 are easy to conflate and are not the same thing. Section 4 is an extension of the vehicular method in which pedestrians REDUCE vehicular capacity, and it lives in `twsc.rs` alongside the movement model. Section 5 is a procedure in its own right, in which the pedestrian is the subject and the service measure is the proportion of pedestrians dissatisfied with the crossing. It lives in `src/hcm/twsc/pedestrian.rs`, is reached through the free function `PedestrianCrossing::analyze` rather than through `Twsc::analyze`, and is exposed to Python as `analyze_twsc_pedestrian`, a JSON-in/JSON-out function rather than a class (`PedestrianCrossing::from_json` / `PedestrianCrossingAnalysis`).

It covers the two-stage crossing decomposition, group critical headway with the optional platoon adjustment, blocked-lane and delayed-crossing probabilities, gap delay, the motorist-yield reduction, and the satisfaction model, plus `pedestrian_los` for the letter and `delay_interpretation` for the qualitative delay bands. It reproduces every published value of Chapter 32 TWSC Example Problem 2 across all three of its scenarios.

Two things the source text does not settle carry VERIFY-HCM notes in the module. The Equation 20-95 coefficients I_MR and I_NY are clipped by the PDF's scrollable equation box and were solved out of the six published O(S/D) values, which over-determine them; and the book does not say whose P_d and P(Y_1) feed Equations 20-98 and 20-99 on a two-stage crossing, where the first stage's are used. Equations 20-91 and 20-92 are clipped by the same box, and the module says how they were recovered.

## Deviations

No `docs/hcm/VERIFICATION.md` exists on this branch; all known deviations are documented as `// VERIFY-HCM` comments directly in `twsc.rs`:
1. The Step 3 conflicting-flow default (7th Edition exhibit factors vs. the HCM 6th Edition forms used by the published Chapter 32 Example Problem 3 worked answers for movements 7/8/10/11) — see the Step 3 subsection above; reproducible via `conflicting_flow_overrides`.
2. U-turn base critical headway (6.4 s) and follow-up headway (2.5 s) fall back to the four-lane "wide median" row when a U-turn is coded on a two-lane major street, since Exhibit 20-17/20-18 lists "NA" for that cell.
3. Chapter 32 TWSC Example Problem 4 (case3.json) drops the major-street right-turn term in the minor-street left-turn Stage II conflicting flow (0.5 v_6 for movement 7, 0.5 v_3 for movement 10): its published v_c,7 = 1,827 and v_c,10 = 1,832 veh/h, whereas the 7th Edition Exhibit 20-16 factors used here give 1,874 and 1,879. The fixture uses `conflicting_flow_overrides` to reproduce the published totals, mirroring the Example Problem 3 pattern above. This is recorded as a `// VERIFY-HCM` note alongside the existing Step 3 comment.

No new discrepancies between the code and the HCM 7th Edition EPUB were found while cross-checking every equation above (Equations 20-1 through 20-75, plus Equation 30-13 and the surrounding Chapter 30 Section 3 procedure); every formula, branch condition, and constant in `twsc.rs`, `computed_pb.rs`, and `gap_acceptance.rs` was verified to match the EPUB text exactly, including subtle points that could plausibly have been transcription errors but were confirmed correct: the distinct 450T (Equation 20-61) vs 150T (Equation 20-66) radicand denominators, the dual role of Equation 20-27 as both the specific major-left-plus-U-turn shared-lane form and an exact restatement of the general Equation 20-49 form, and the N=1 vs N>1 branch structure of Equations 20-62/20-63.

## Validation

Fixtures live at `tests/ExampleCases/hcm/Twsc/case1.json` (HCM Chapter 32 TWSC Example Problem 1: three-leg intersection, one major lane per direction, 10% heavy vehicles), `case2.json` (TWSC Example Problem 3: four-leg, two major lanes per direction, two-stage gap acceptance with `n_m = 2`, flared single-lane minor approaches with `n_R = 1`, 10% heavy vehicles, using the `conflicting_flow_overrides` mechanism to match the published 6th-Edition-style Step 3 values), and `case3.json` (TWSC Example Problem 4: four-leg TWSC between two coordinated upstream signals, four-lane major street with shared major-street left turns, no minor-street through movements, one stage, exercising the Step 5b platoon-blockage path with p_b = 0.170 for movements 1/4/9/12 and 0.260 for movements 7/10). `test_twsc_example_problem_4_upstream_signals` reproduces the published conflicting flows (1,086 / 1,076 / 538 / 543 / 1,827 / 1,832 veh/h), the platooned potential capacities c_p,1 = 750, c_p,4 = 758, c_p,9 = 859, c_p,12 = 852, c_p,7 = 73, c_p,10 = 72 veh/h, LOS B/B/A/A/F/F, and — now that Step 7d and Step 11b are wired — the shared-major-left results exactly: c_m,7 = c_m,10 = 47 veh/h (via p\*_0 = 0.856, Equations 20-33/34), the Step 11b Rank 1 delay d_2+3 = d_5+6 = 1.3 s (Equations 20-62/63), d_A,EB = d_A,WB = 1.9 s, and d_I = 34.1 s (asserted at +-0.5 s). The two oversaturated minor-street left delays d_7 = d_10 = 529 s and the minor-approach delays d_A,NB = d_A,SB = 241 s use a wider documented tolerance (+-12 s and +-5 s): Equation 20-61 has slope ~18.6 s per veh/h near v/c = 1.7, and the book rounds c_m to the integer 47 while this library carries 46.6-47.1, so the two per-movement delays split symmetrically around 529 s (their over/under-shoots cancel in d_I). The Rust integration test is `tests/chapter20_integration.rs` (`test_twsc_example_problem_1_full_pipeline`, `test_twsc_example_problem_3_full_pipeline`, `test_twsc_fixture_roundtrip`); a PyO3-bound Python equivalent is `tests/test_chapter20_integration.py`. Declared tolerances (module doc comment of `chapter20_integration.rs`): LOS exact; control delay within +-0.5 s/veh; capacity within +-5 veh/h (the integration test file additionally widens queue tolerance to +-0.2 veh at individual assertion sites). Example Problem 1 reproduces c_m,4 = 1,238, c_m,9 = 760, c_m,7 = 268, c_SH,NB = 521 veh/h; d_4 = 8.3 s/veh (LOS A), d_SH,NB = 14.9 s/veh (LOS B); approach delays d_EB = 0.0, d_WB = 2.9, d_NB = 14.9 s/veh; intersection delay 4.1 s/veh; Q95,4 = 0.4, Q95,NB = 1.3 veh. Example Problem 3 reproduces the two-stage total capacities c_T,8 = 390, c_T,11 = 405, c_T,7 = 365, c_T,10 = 342 veh/h; flared-lane capacities c_F,NB = 498, c_F,SB = 487 veh/h; d_1 = 8.4, d_4 = 8.2 s/veh (both LOS A); d_NB = 18.3, d_SB = 15.6 s/veh (both LOS C); intersection delay 6.3 s/veh; queues 0.1/0.2/2.4/1.3 veh. Additional per-step unit tests in `src/hcm/twsc/tests.rs` (37 tests total, including the Exhibit 20-19 mapping, both Equation 20-19 branches, the Equation 20-21 zero-flow branch, platoon-off equivalence, the shared-major-left p\*_0 = 0.856 value and short-pocket variant, the EP4 Rank 1 delay value, and the end-to-end `Exclusive`-vs-`Shared` wiring check) spot-check every step against the example problems' intermediate values (conflicting flows, headways, potential capacities, stage capacities, lane capacities) at tighter tolerances (typically 1.0-2.0 veh/h or 1e-9 s for closed-form arithmetic). The computed proportion-of-time-blocked path (`computed_pb.rs`) adds eight mechanism tests: `test_proportion_time_blocked_square_wave` (Equation 30-13 on a hand-constructed 20-of-100-step platoon, p_b = 0.20 exactly), `test_computed_pb_direction_mapping` (only-eastbound signal blocks movements 4/9 not 1/12), `test_computed_pb_decreases_with_distance` (dispersion-flattening monotonicity of p_b vs segment length), `test_computed_pb_union_of_both_directions` (minor-street p_b as the union of both directions' blocked steps), `test_analyst_pb_takes_precedence_over_upstream`, `test_computed_pb_matches_manual_platoon_blockage` (computed path is bit-identical to manually setting the derived `PlatoonBlockage`), `test_computed_pb_empty_signals_equivalent_to_off`, and `test_upstream_signals_serde_roundtrip`. There is no published-target regression for the end-to-end p_b values (Chapter 30 Example Problem 1 reports them as engine output — 0.170/0.260 in Exhibit 32-12 — requiring the full Chapter 19 coordinated engine and Section 2 O-D distribution, the same dependency that blocks reproducing EP1's P = 0.493). The standalone `shared_major_lane_capacity` (Step 10c) helper remains covered by `test_shared_major_lane_capacity` only, since neither example problem's reported output depends on the shared through-lane capacity c_SS.

## Deferred

The upstream-signal potential-capacity adjustment (Equations 20-19 through 20-21, Exhibit 20-19) is now wired into `analyze` through the `platoon_blockage` input and validated end-to-end by Example Problem 4 (case3.json). What remains deferred:

- **Chapter 30 derivation of p_b,x.** *Wired (this branch).* The Chapter 30, Section 3 "Proportion of Time Blocked" computation (Equation 30-13) now derives p_b,x from `upstream_signals` (system cycle, per-direction segment length / progression speed / discharge profiles) via `src/hcm/twsc/computed_pb.rs`, using the `chapter18::platoon_dispersion` machinery to build the arrival flow profiles and the blocked-period-vs-q_c logic to count blocked steps (see "Step 5b input: computed proportion of time blocked" above). The mechanism is unit-tested (square-wave hand check, dispersion-flattening monotonicity, directional mapping, both-direction union, analyst precedence, and equivalence to manually supplying the computed `PlatoonBlockage`). What remains genuinely deferred: the HCM does not publish a hand-computable derivation chain from raw upstream timing to the Chapter 30 Example Problem 1 / Exhibit 32-12 p_b values (0.170 / 0.260) — those are engine outputs that require the full Chapter 19 coordinated-actuated discharge profiles and the Section 2 origin–destination distribution (the same dependency that blocks reproducing Example Problem 1's P = 0.493, per `chapter18-computed.md`), so there is no published-target regression for the end-to-end p_b, only the mechanism tests.
- **Shared / short major-street left-turn pocket.** *Impedance and delay wired (this branch).* The `MajorLeftLaneConfig` input (`major_left_eb`/`major_left_wb` = `Exclusive`/`Shared`/`SharedShortPocket{storage_veh}`) now drives the Step 7d p\*_0,j substitution (Equations 20-29 through 20-34, `prob_queue_free_shared_major`) into the Rank 3/4 impedance chain and the Step 11b Rank 1 delay (Equations 20-62/63, `rank1_delay`) into Step 12, so Example Problem 4 reproduces c_m,7 = c_m,10 = 47 veh/h, d_2+3 = d_5+6 = 1.3 s, and d_I = 34.1 s exactly. What remains: the Step 10c shared through-lane capacity `shared_major_lane_capacity` (Equations 20-51 through 20-60, the reduced c_SS of the major through lane) is still standalone and unwired — it does not affect the minor-street or intersection-delay outputs in Example Problem 4 (it evaluates to the s_2+3 bound), but a caller wanting the reported major-street through-lane capacity under a short pocket must invoke it directly. `f_LL` and the s = 1,800/1,500 saturation flow rates are fixed at the HCM defaults rather than exposed as per-approach field-measured inputs.

None of these are stubbed with `todo!()`.
