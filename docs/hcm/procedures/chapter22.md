# HCM Chapter 22 — Roundabouts

This document walks the HCM 7th Edition Chapter 22 motorized-vehicle methodology for roundabouts as implemented on branch `feat/hcm-ch20-22-unsignalized`. The code follows Chapter 22, Section 3 (core methodology) plus Section 4's capacity-model calibration extension (Equations 22-21 through 22-23), transcribed as the twelve-step procedure in the module header of `src/hcm/roundabouts/roundabouts.rs` (the sole source file; `src/hcm/roundabouts/tests.rs` holds the per-step unit tests against HCM Chapter 33 Example Problems 1 and 2). Delay/LOS primitives shared with Chapters 19-21 (`control_delay_roundabout`, `aggregate_control_delay` in `src/hcm/common/delay.rs`; `los_unsignalized` in `src/hcm/common/los_tables.rs`) are documented separately in `common-infrastructure.md`. `docs/hcm/VERIFICATION.md` did not exist when this document was first written; it does now, and carries the consolidated book-discrepancy ledger. The deviations noted below come directly from code comments and test doc comments. Every equation transcribed below was cross-checked against both the Rust function body and the HCM 7th Edition Chapter 22 EPUB text (Sections 3 and 4); no mathematical discrepancies were found — every case is either an exact match or a documented, already-flagged simplification (see Deviations).

The geometry convention (stated in the module header) is a standard four-leg roundabout with the NB entry on the south leg, SB on the north leg, EB on the west leg, and WB on the east leg — i.e., `Leg` values name the direction of *travel through* the entry, not the compass position of the leg itself. The public entry point is `Roundabouts::analyze(&mut self)`, which runs Steps 1-2 (`step1_2_flow_rates_pce`), Step 3 (`step3_conflicting_flows`), Steps 4 through 10 and 12 combined per-approach (`step4_12_lane_performance`), and Step 11 (`step11_aggregate_delay`).

## Step-by-step walkthrough

| HCM step | Equations / Exhibits | Rust function(s) | Inputs (units) | Outputs (units) |
|---|---|---|---|---|
| Steps 1-2 — Flow rates and heavy-vehicle adjustment | Equations 22-8 through 22-10; Exhibit 22-11 | `Roundabouts::step1_2_flow_rates_pce`, `Roundabouts::heavy_vehicle_factor` | Movement demand volumes `v_u`/`v_l`/`v_t`/`v_r` (veh/h), `phf`, `heavy_vehicle_pct` (%) | `RoundaboutApproach.flows_pce: [f64; 4]` (U, L, T, R in pc/h) |
| Step 3 — Circulating and exiting flow | Equations 22-11, 22-12 | `Roundabouts::circulating_flow_pce`, `Roundabouts::bypass_conflicting_exit_flow_pce`, `Roundabouts::step3_conflicting_flows` | Adjacent-leg movement flows (pc/h), `Leg::opposite`/`left_of`/`right_of` topology | `RoundaboutApproach.circulating_flow_pce`, `bypass_conflicting_flow_pce` (pc/h) |
| Step 4 — Entry lane flows | Exhibits 22-14 (de facto lane checks), 22-15 (volume assignment), 22-9 (lane-utilization defaults) | `Roundabouts::entry_lane_flows_pce` | Approach U/L/T/R flows (pc/h), `LaneAssignment`, `pct_left_lane` (decimal, `None` => 0.47 or 0.53 default) | `(left_lane_pce, right_lane_pce)` tuple |
| Step 5 — Entry/bypass capacity | Equations 22-1 through 22-7 (national models); Equation 22-21 (calibrated form) | `Roundabouts::entry_capacity_pce`, `capacity_single_lane`, `capacity_two_lane_entry_one_circ`, `capacity_one_lane_entry_two_circ`, `capacity_two_lane_entry_two_circ_left`/`_right`, `capacity_bypass_one_exit_lane`/`_two_exit_lanes`, `capacity_exponential` | Circulating (or, for bypass, conflicting exit) flow (pc/h); `Roundabouts.calibration: Option<(f64, f64)>` overrides all national models if set | Lane capacity (pc/h) |
| Step 6 — Pedestrian impedance | Exhibits 22-18 (one-lane), 22-20 (two-lane) | `ped_factor_one_lane`, `ped_factor_two_lane` | Conflicting pedestrian flow `n_ped` (p/h), circulating flow (pc/h) | Pedestrian adjustment factor f_ped (decimal) |
| Step 7 — Convert to veh/h | Equations 22-13, 22-14 | Inline in `Roundabouts::step4_12_lane_performance` (`v_veh = v_pce * f_hv`; `c_veh = c_pce * f_hv * f_ped`) | pc/h flow and capacity, f_HV, f_ped | `RoundaboutLaneResult.flow_veh`/`capacity_veh` (veh/h) |
| Step 8 — v/c ratio | Equation 22-16 | Inline in `step4_12_lane_performance` | Flow and capacity (veh/h) | `RoundaboutLaneResult.v_c_ratio` |
| Step 9 — Control delay | Equation 22-17 | `common::delay::control_delay_roundabout` | Flow, capacity (veh/h), analysis period T (h) | `RoundaboutLaneResult.control_delay` (s/veh) |
| Step 10 — LOS | Exhibit 22-8 | `common::los_tables::los_unsignalized` | Control delay, v/c > 1.0 flag | `RoundaboutLaneResult.los` |
| Step 11 — Approach/intersection aggregation | Equations 22-18, 22-19 | `Roundabouts::step11_aggregate_delay`, `common::delay::aggregate_control_delay` | Per-lane (and bypass-lane) control delay and flow rate | `RoundaboutApproach.control_delay`/`los`; `Roundabouts.intersection_delay`/`intersection_los` |
| Step 12 — 95th percentile queue | Equation 22-20 | `Roundabouts::queue_95` | v/c ratio, capacity (veh/h), analysis period T (h) | `RoundaboutLaneResult.queue_95` (veh) |

### Steps 1-2: flow rates, PCE, and heavy-vehicle adjustment

```
Equation 22-8:   v_i = V_i / PHF                                              [veh/h]
  v_i  = demand flow rate for movement i                                      [veh/h]
  V_i  = demand volume for movement i                                         [veh/h]
  PHF  = peak hour factor
Implemented in: roundabouts/roundabouts.rs::Roundabouts::step1_2_flow_rates_pce (`conv` closure, divide-by-phf leg)

Equation 22-9:   v_i,pce = v_i / f_HV                                         [pc/h]
  v_i,pce = demand flow rate for movement i, passenger-car equivalents        [pc/h]
  v_i     = demand flow rate for movement i (Equation 22-8)                   [veh/h]
  f_HV    = heavy-vehicle adjustment factor (Equation 22-10)
Implemented in: roundabouts/roundabouts.rs::Roundabouts::step1_2_flow_rates_pce (`conv(v) = v / phf / f_hv`, Equations 22-8 and 22-9 fused into one closure)

Equation 22-10:  f_HV = 1 / [1 + P_T·(E_T − 1)]
  f_HV = heavy-vehicle adjustment factor
  P_T  = proportion of demand volume that is heavy vehicles                   [decimal] (code: `heavy_vehicle_pct / 100.0`)
  E_T  = passenger car equivalent for heavy vehicles = 2.0 (Exhibit 22-11; code: `E_T_HEAVY_VEHICLE`)
Implemented in: roundabouts/roundabouts.rs::Roundabouts::heavy_vehicle_factor
```

`E_T_HEAVY_VEHICLE = 2.0` (Exhibit 22-11's single passenger-car-equivalent for heavy vehicles; roundabouts do not distinguish truck sub-classes or grade the way Chapter 12 does). `heavy_vehicle_factor` (Equation 22-10) is applied twice in the pipeline: once in Step 1-2 to convert a veh/h demand volume into a pc/h flow rate (dividing by `f_hv` inflates veh/h into pc/h, Equations 22-8/22-9 combined), and again in Step 7 to convert a pc/h capacity back into veh/h (Equation 22-14, multiplying by `f_hv`) — the two uses are algebraically inverse operations on the same factor, correctly applied in opposite directions for flow (down-scale demand to pc/h by dividing) versus capacity (up-scale a pc/h capacity back to veh/h by multiplying), each additionally combined with the pedestrian factor only on the capacity side. The HCM's Equation 22-15 (a per-movement, flow-weighted average of `f_HV` across a shared lane's U/L/T/R movements, for cases where different movements have different heavy-vehicle percentages) is not implemented: the input model carries a single `heavy_vehicle_pct` per approach rather than per movement, so `f_HV` is computed once per approach and reused for every lane and movement on it. This is a simplification of input granularity, not a wrong equation — it was not in this task's enrichment scope and no fixture exercises per-movement heavy-vehicle percentages.

### Step 3: circulating and exiting flow (Equations 22-11, 22-12)

```
Equation 22-11 (example, northbound entry):
  v_c,NB,pce = v_WB,U + v_SB,L + v_SB,U + v_EB,T + v_EB,L + v_EB,U             [pc/h]
  v_c,NB,pce = conflicting circulating flow rate for the NB entry              [pc/h]
  v_X,M,pce  = flow rate of movement M (U-turn/left/through) on leg X, pc/h
Implemented in: roundabouts/roundabouts.rs::Roundabouts::circulating_flow_pce

Equation 22-12 (example, southbound exit):
  v_ex,SB,pce = v_NB,U + v_WB,L + v_SB,T + v_EB,R − v_EB,R,bypass,pce          [pc/h]
  v_ex,SB,pce      = conflicting exiting flow for a bypass lane merging into the SB exit   [pc/h]
  v_EB,R,bypass,pce = right-turn flow from the immediate upstream (EB) entry already diverted to its own bypass lane, excluded if that entry has no bypass   [pc/h]
Implemented in: roundabouts/roundabouts.rs::Roundabouts::bypass_conflicting_exit_flow_pce
```

Both equations generalize across all four legs using the `Leg::opposite`/`left_of`/`right_of` topology rather than four separately hard-coded formulas. `circulating_flow_pce(leg)` computes, for the general entry `leg`: the U-turn flow of the entry to its right, plus the U-turn and left-turn flows of the opposite entry, plus the U-turn, left-turn, and through flows of the entry to its left — substituting `leg = NB` reproduces Equation 22-11 exactly (`right_of(NB) = WB`, `opposite(NB) = SB`, `left_of(NB) = EB`). `bypass_conflicting_exit_flow_pce(entry)` computes the exiting flow on the leg that `entry`'s right-turning traffic merges into (`exit_dir = entry.left_of()`) as the U-turn flow of the opposite leg, plus the left-turn flow of the leg to the exit's left, plus the exit leg's own through flow, plus the right-turn flow of the leg to the exit's right, minus that same right-turn flow if the entry generating it also has its own bypass lane (to avoid double-counting bypass traffic that never enters the circulatory roadway) — substituting `exit_dir = SB` reproduces Equation 22-12 exactly (`opposite(SB) = NB`, `left_of(SB) = WB`, `right_of(SB) = EB`).

### Step 4: entry-lane flows (Exhibits 22-14, 22-15, 22-9)

`entry_lane_flows_pce` first computes the full entry volume `v_e = v_U + v_L + v_T + v_R,e` (nonbypass right-turn flow `v_R,e` excluded when a bypass lane exists, since bypass traffic does not use the entry lanes at all — HCM Step 4, item 1), then, for a two-lane entry, applies Exhibit 22-14's de facto lane reclassification before assigning volumes per Exhibit 22-15. This reclassification is a precise conditional rule even though it is not a numbered equation, so it is written out here rather than only described:

```
Exhibit 22-14, designated Left-Through | Through-Right entry:
  if v_U + v_L > v_T + v_R,e:  reclassify as Left | Through-Right          (de facto left-turn lane)
  else if v_R,e > v_U + v_L + v_T:  reclassify as Left-Through | Right     (de facto right-turn lane)
  else:  keep Left-Through | Through-Right
Implemented in: roundabouts/roundabouts.rs::Roundabouts::entry_lane_flows_pce (LaneAssignment::LeftThroughAndThroughRight match arm)

Exhibit 22-14, designated Left | Left-Through-Right entry:
  if v_T + v_R,e > v_U + v_L:  reclassify as Left | Through-Right          (de facto through-right lane)
  else:  keep Left | Left-Through-Right
Implemented in: roundabouts/roundabouts.rs::Roundabouts::entry_lane_flows_pce (LaneAssignment::LeftAndAllMovements match arm)

Exhibit 22-14, designated Left-Through-Right | Right entry:
  if v_U + v_L + v_T > v_R,e:  reclassify as Left-Through | Right         (de facto left-through lane)
  else:  keep Left-Through-Right | Right
Implemented in: roundabouts/roundabouts.rs::Roundabouts::entry_lane_flows_pce (LaneAssignment::AllMovementsAndRight match arm)
```

Once the (possibly reclassified) lane assignment is known, Exhibit 22-15 assigns volumes to the left and right lanes:

```
Exhibit 22-15, case Left | Through-Right:       left = v_U + v_L,            right = v_T + v_R,e
Exhibit 22-15, case Left-Through | Right:       left = v_U + v_L + v_T,      right = v_R,e
Exhibit 22-15, cases Left-Through | Through-Right, Left | Left-Through-Right, Left-Through-Right | Right (no de facto split applies):
                                                 left = %LL·v_e,             right = %RL·v_e = (1 − %LL)·v_e
  %LL = percentage of entry traffic using the left lane                     [decimal] (Exhibit 22-9 default: 0.53 for Left | Left-Through-Right, 0.47 for the other two; overridable per approach via `pct_left_lane`)
Implemented in: roundabouts/roundabouts.rs::Roundabouts::entry_lane_flows_pce (final `match assignment` block)
```

`test_de_facto_right_turn_lane` exercises the de facto right-turn-lane branch directly by giving an `LT|TR` approach overwhelming right-turn demand and confirming the left lane absorbs all left+through volume while the right lane gets only the right-turn flow.

### Step 5: entry-capacity regression equations — exact coefficients

All six national entry/bypass capacity models reduce to the same generalized Siegloch exponential form (HCM Equation 22-21's calibrated form, reused as the literal implementation of every "national" equation):

```
Equation 22-21 (general Siegloch form):  c_pce = A·e^(−B·v_c)                 [pc/h]
  c_pce = lane capacity, adjusted for heavy vehicles                          [pc/h]
  v_c   = conflicting flow (circulating, or opposing-exit for a bypass lane)  [pc/h]
  A, B  = calibration constants (intercept, decay rate); national values per Equations 22-1 through 22-7 below, or a caller-supplied override (Section 4)
Implemented in: roundabouts/roundabouts.rs::capacity_exponential
```

The exact literal constants transcribed in `roundabouts.rs`, quoted verbatim from the source (no rounding applied here), each written out in the full Siegloch exponential form:

```
Equation 22-1:  c_e,pce = 1,380·e^(−1.02×10⁻³·v_c,pce)                        [pc/h]   one-lane entry, one circulating lane
Equation 22-2:  c_e,pce = 1,420·e^(−0.91×10⁻³·v_c,pce)                        [pc/h]   each lane of a two-lane entry, one circulating lane
Equation 22-3:  c_e,pce = 1,420·e^(−0.85×10⁻³·v_c,pce)                        [pc/h]   one-lane entry, two circulating lanes (v_c,pce is the two-lane total)
Equation 22-4:  c_e,R,pce = 1,420·e^(−0.85×10⁻³·v_c,pce)                      [pc/h]   right lane of a two-lane entry, two circulating lanes
Equation 22-5:  c_e,L,pce = 1,350·e^(−0.92×10⁻³·v_c,pce)                      [pc/h]   left lane of a two-lane entry, two circulating lanes
Equation 22-6:  c_bypass,pce = 1,380·e^(−1.02×10⁻³·v_ex,pce)                  [pc/h]   yielding bypass, one opposing exit lane
Equation 22-7:  c_bypass,pce = 1,420·e^(−0.85×10⁻³·v_ex,pce)                  [pc/h]   yielding bypass, two opposing exit lanes
  v_c,pce  = conflicting circulating flow rate                                [pc/h]
  v_ex,pce = conflicting exiting flow rate (bypass lanes only, Equation 22-12) [pc/h]
Implemented in: roundabouts/roundabouts.rs::capacity_single_lane, capacity_two_lane_entry_one_circ, capacity_one_lane_entry_two_circ, capacity_two_lane_entry_two_circ_right, capacity_two_lane_entry_two_circ_left, capacity_bypass_one_exit_lane, capacity_bypass_two_exit_lanes (all via capacity_exponential)
```

Two of these coefficient pairs are numerically identical to each other by construction rather than coincidence, and the code makes this explicit rather than duplicating the arithmetic: Equation 22-4 (two-lane entry, right lane, two circulating lanes) and Equation 22-3 (one-lane entry, two circulating lanes) share `(1_420.0, 0.85e-3)`, and `capacity_bypass_two_exit_lanes` (Equation 22-7) is implemented as a direct call to `capacity_two_lane_entry_two_circ_right` rather than a second literal — `test_bypass_capacity_equations` asserts this equivalence directly (`capacity_bypass_two_exit_lanes(500.0) == capacity_two_lane_entry_two_circ_right(500.0)` to `1e-9`). None of the seven coefficient pairs look physically implausible: intercepts A cluster around 1,350-1,420 pc/h (the theoretical maximum entry rate at zero conflicting flow, consistent with a follow-up headway of roughly 2.5-2.7 s), and slopes B are all small negative-exponent decay rates on the order of 0.85e-3 to 1.02e-3 per pc/h.

### Section 4: calibration (Equations 22-21 through 22-23)

```
Equation 22-22:  A = 3,600 / t_f                                              [pc/h]
  A   = calibrated intercept (Equation 22-21)                                 [pc/h]
  t_f = field-measured follow-up headway                                      [s]
Implemented in: roundabouts/roundabouts.rs::calibrated_intercept_a

Equation 22-23:  B = (t_c − t_f/2) / 3,600                                     [pc/h⁻¹]
  B   = calibrated slope (Equation 22-21)                                     [pc/h⁻¹]
  t_c = field-measured critical headway                                       [s]
  t_f = field-measured follow-up headway                                      [s]
Implemented in: roundabouts/roundabouts.rs::calibrated_slope_b
```

`Roundabouts.calibration: Option<(f64, f64)>` is an intersection-wide override: when set, `entry_capacity_pce` bypasses the geometry-based dispatch on `(entry_lanes, circulating_lanes)` entirely and calls `capacity_exponential(a, b, v_c)` with the caller-supplied (A, B) pair for every entry lane at the intersection (bypass lanes are unaffected — they always use the national `capacity_bypass_*` models). Nothing in `analyze()` calls `calibrated_intercept_a`/`calibrated_slope_b` automatically — a caller measuring local headways would compute (A, B) externally and set `Roundabouts.calibration` directly. `test_calibration_equations` verifies the round-trip: solving Equations 22-22/22-23 backward for `t_f = 3,600/1,380 ~= 2.609 s` and `t_c ~= 4.976 s` reproduces Equation 22-1's exact (1,380, 1.02e-3) pair.

### Step 6: pedestrian impedance (Exhibits 22-18, 22-20)

```
Exhibit 22-18 (one-lane entry), piecewise in v_c,pce and n_ped:
  if v_c,pce > 881:      f_ped = 1
  else if n_ped <= 101:   f_ped = 1 − 0.000137·n_ped
  else:                   f_ped = [1,119.5 − 0.715·v_c,pce − 0.644·n_ped + 0.00073·v_c,pce·n_ped] / [1,068.6 − 0.654·v_c,pce]
  f_ped  = entry capacity adjustment factor for pedestrians                   [decimal]
  n_ped  = number of conflicting pedestrians                                  [p/h]
  v_c,pce = conflicting circulating flow rate                                 [pc/h]
Implemented in: roundabouts/roundabouts.rs::ped_factor_one_lane

Exhibit 22-20 (two-lane entry), piecewise in n_ped:
  f_100 = [1,260.6 − 0.329·v_c,pce − 0.381×100] / [1,380 − 0.5·v_c,pce]
  if n_ped < 100:  f_ped = min[1 − (n_ped/100)·(1 − f_100), 1]
  else:            f_ped = min[(1,260.6 − 0.329·v_c,pce − 0.381·n_ped) / (1,380 − 0.5·v_c,pce), 1]
Implemented in: roundabouts/roundabouts.rs::ped_factor_two_lane
```

`ped_factor_one_lane` (Exhibit 22-18) is a three-branch function: full credit (factor 1.0) whenever conflicting circulating flow exceeds 881 pc/h (pedestrians are assumed to find adequate gaps regardless of their own volume at that point), a simple linear reduction for light pedestrian volumes (n_ped <= 101 p/h), and the bilinear regression above for heavier pedestrian flows, clamped to [0, 1] and verified continuous and monotonically decreasing at the boundary and beyond by `test_ped_factor_one_lane_exhibit_22_18`. `ped_factor_two_lane` (Exhibit 22-20) interpolates linearly between full credit and the n_ped = 100 regression value for light pedestrian flows, continuity across the n_ped = 100 breakpoint confirmed by `test_ped_factor_two_lane_exhibit_22_20`. Every numeric coefficient above (881, 101, 0.000137, 1,119.5, 0.715, 0.644, 0.00073, 1,068.6, 0.654, 1,260.6, 0.329, 0.381, 1,380, 0.5) was checked against both the code and the HCM Chapter 22 EPUB text and matches exactly.

### Step 7: pc/h to veh/h conversion (Equations 22-13, 22-14)

```
Equation 22-13:  v_i = v_i,PCE · f_HV,e                                       [veh/h]
  v_i     = flow rate for lane i                                              [veh/h]
  v_i,PCE = flow rate for lane i                                              [pc/h]
  f_HV,e  = heavy-vehicle adjustment factor for the lane
Implemented in: roundabouts/roundabouts.rs::Roundabouts::step4_12_lane_performance (`v_veh = v_pce * f_hv`)

Equation 22-14:  c_i = c_i,PCE · f_HV,e · f_ped                                [veh/h]
  c_i     = capacity for lane i                                               [veh/h]
  c_i,PCE = capacity for lane i                                               [pc/h]
  f_HV,e  = heavy-vehicle adjustment factor for the lane
  f_ped   = pedestrian impedance factor (Step 6)
Implemented in: roundabouts/roundabouts.rs::Roundabouts::step4_12_lane_performance (`c_veh = c_pce * f_hv * f_ped`)
```

### Step 8: volume-to-capacity ratio (Equation 22-16)

```
Equation 22-16:  x_i = v_i / c_i                                              [unitless]
  x_i = volume-to-capacity ratio of subject lane i
  v_i = demand flow rate of subject lane i                                    [veh/h]
  c_i = capacity of subject lane i                                            [veh/h]
Implemented in: roundabouts/roundabouts.rs::Roundabouts::step4_12_lane_performance (`x = if c_veh > 0.0 { v_veh / c_veh } else { 0.0 }`)
```

### Step 9: control delay (Equation 22-17)

```
Equation 22-17:  d = 3,600/c + 900·T·[x − 1 + √((x−1)² + (3,600/c)·x/(450·T))] + 5·min(x, 1)   [s/veh]
  d = average control delay                                                   [s/veh]
  x = volume-to-capacity ratio of the subject lane
  c = capacity of the subject lane                                            [veh/h]
  T = analysis period, h (0.25 h for a 15-min analysis)                       [h]
Implemented in: roundabouts/roundabouts.rs (called via common::delay::control_delay_roundabout)
```

Equation 22-17 is verified to be the same shape as the Chapter 20/21 delay equation: it is identical to Equation 20-61/21-30 except the final "+5" term is scaled by `min(x, 1)` rather than applied unconditionally, reflecting YIELD control (an entering driver need not stop at all when there is no conflicting traffic; at higher v/c the likelihood of a full stop rises toward 1, matching STOP-control behavior). `control_delay_roundabout` implements exactly this: `3_600.0/capacity + 900.0*t_h*(...) + 5.0*x.min(1.0)`.

### Steps 10-11: LOS and aggregation (Equations 22-18, 22-19)

Step 10 (LOS) reuses `los_unsignalized` per Exhibit 22-8 with no chapter-specific logic; documented in `common-infrastructure.md`.

```
Equation 22-18:  d_approach = [d_LL·v_LL + d_RL·v_RL + d_bypass·v_bypass] / [v_LL + v_RL + v_bypass]   [s/veh]
  d_approach = approach control delay
  d_LL, d_RL, d_bypass = control delay of the left lane, right lane, and bypass lane (as present)   [s/veh]
  v_LL, v_RL, v_bypass = flow rate of the left lane, right lane, and bypass lane (as present)         [veh/h]
Implemented in: roundabouts/roundabouts.rs::Roundabouts::step11_aggregate_delay (via common::delay::aggregate_control_delay, over the approach's `lanes` plus its optional `bypass_lane`)

Equation 22-19:  d_intersection = Σ(d_i·v_i) / Σ(v_i)                         [s/veh]
  d_i = control delay for approach i                                         [s/veh]
  v_i = flow rate for approach i                                             [veh/h]
Implemented in: roundabouts/roundabouts.rs::Roundabouts::step11_aggregate_delay (via common::delay::aggregate_control_delay, invoked a second time over approach-level pairs)
```

`aggregate_control_delay` is the single flow-weighted-average implementation shared by both equations (and by the analogous Chapter 19/20/21 aggregation equations); the module-level function is generic over any list of `(delay, flow)` pairs rather than being called once per named lane, so Equation 22-18's three named terms (LL, RL, bypass) are simply whichever of `a.lanes` and `a.bypass_lane` are present for that approach, collected into one `pairs` vector before the weighted average is taken.

### Step 12: 95th percentile queue (Equation 22-20)

```
Equation 22-20:  Q_95 = 900·T·[x − 1 + √((1−x)² + (3,600/c)·x/(150·T))]·(c/3,600)   [veh]
  Q_95 = 95th percentile queue                                                [veh]
  x    = volume-to-capacity ratio of the subject lane
  c    = capacity of the subject lane                                        [veh/h]
  T    = analysis period                                                     [h]
Implemented in: roundabouts/roundabouts.rs::Roundabouts::queue_95
```

`queue_95` matches Equation 22-20 exactly, including the `(1−x)²` (equal to `(x−1)²`) form and the `150·T` divisor inside the radicand and the final `(c/3,600)` scaling factor outside the brackets. It is algebraically the same family as the AWSC/TWSC queue equations once the substitution `h_d = 3,600/c` is made: `900·T/h_d` (the AWSC prefactor) equals `900·T·c/3,600` (this equation's prefactor after distributing the trailing `c/3,600` term), confirmed by direct algebraic expansion during this review; no discrepancy.

## Deviations

No `docs/hcm/VERIFICATION.md` exists in this branch's working tree. No `VERIFY-HCM` code comments appear in `roundabouts.rs`; the file's only documented modeling choice worth flagging as an interpretation (not a stated deviation) is the nonyielding (Type 2) bypass lane treatment: `step4_12_lane_performance`'s bypass-lane match arm assigns a nonyielding bypass zero capacity, zero v/c, zero delay, and LOS A unconditionally, citing "the HCM Chapter 33 Example Problem 1 treatment" in an inline comment rather than a numbered equation — the HCM textual guidance is that nonyielding (merge-style) bypass lanes experience negligible delay because they do not yield to circulating traffic, but the code's zero-capacity/zero-v-c representation is a simplification (a real nonyielding bypass has a finite, generally very high, capacity) rather than a literal transcription of an HCM equation. This is verified against Chapter 33 Example Problem 1's SB bypass (nonyielding), where the published answer is indeed 0 s/veh delay and LOS A, so the simplification reproduces the fixture correctly but would not distinguish a hypothetical highly-congested nonyielding bypass from an empty one. A second, newly-noted (but not a numerical error) simplification found during this pass: Equation 22-15's per-movement, flow-weighted heavy-vehicle adjustment factor is not implemented, because the input model has one `heavy_vehicle_pct` per approach rather than per movement (see Steps 1-2 above) — this is an input-granularity limitation, not a discrepancy against any equation the code does implement, and no fixture requires per-movement heavy-vehicle percentages. No other new discrepancies were found while cross-checking the code against the HCM 7th Edition Chapter 22 EPUB text (Sections 3 and 4) for this pass: Equations 22-1 through 22-23 (including the entry/bypass capacity regressions, the circulating/exiting flow topology, the de facto lane reclassification and volume-assignment rules, both pedestrian-impedance exhibits, the calibration equations, and the Equation 22-17/22-20 delay and queue forms) all match the published equations exactly.

## Validation

Fixtures live at `tests/ExampleCases/hcm/Roundabouts/case1.json` (HCM Chapter 33 Example Problem 1: four-leg single-lane roundabout with a yielding WB bypass and a nonyielding SB bypass, 2% heavy vehicles, PHF = 0.94, 50 p/h crossing the NB entry) and `case2.json` (Example Problem 2: multilane roundabout — NB single-lane entry against two circulating lanes, SB two-lane `LT|R` entry against two circulating lanes, EB/WB two-lane `LT|TR` entries against one circulating lane, 5% heavy vehicles EB/WB and 2% NB/SB, PHF = 0.95). The Rust integration test is `tests/chapter22_integration.rs` (`test_roundabout_example_problem_1_full_pipeline`, `test_roundabout_example_problem_2_full_pipeline`, `test_roundabout_fixture_roundtrip`); the Python-bound equivalent is `tests/test_chapter22_integration.py`. Declared tolerances (module doc comment of `chapter22_integration.rs`): LOS exact, control delays within +-0.5 s/veh, capacities within +-5 veh/h. Example Problem 1 reproduces entry capacities c_NB = 597, c_SB = 618, c_EB = 824, c_WB = 694 veh/h and bypass capacity c_bypass,WB = 851 veh/h; v/c x_NB = 0.70; per-lane delays 22.6/14.0/0/22.0/26.8/20.2 s/veh with LOS C/B/A/C/D/C (Exhibit 33-8); approach delays d_WB = 23.3 s/veh (LOS C), d_SB = 4.7 s/veh (LOS A); intersection delay 17.5 s/veh (LOS C); Q95,NB = 5.7 veh. Example Problem 2 reproduces c_NB = 607, c_SB,L = 651, c_SB,R = 723, c_EB = 675 (both lanes), c_WB = 964 veh/h (both lanes); per-lane delays including d_NB = 11.8 s/veh (LOS B) and d_EB,R = 16.1 s/veh (LOS C, the only LOS-C lane in this fixture); approach delays d_SB = 13.9, d_EB = 15.1, d_WB = 8.3 s/veh; intersection delay 12.3 s/veh (LOS B); Q95,NB = 1.9 veh. Additional per-step unit tests in `src/hcm/roundabouts/tests.rs` spot-check the seven capacity equations directly at the fixtures' own conflicting-flow values (e.g. `capacity_single_lane(796.0) ~= 613` pc/h, within 1.5 pc/h), the calibration-equation round-trip, both pedestrian-impedance exhibits, the Step 1-3 conflicting/exiting flow computations (tolerance 3 pc/h, attributed to the published values' own rounding of intermediate flow rates), the Step 4 lane-flow assignment including the de facto lane checks, and a serde round-trip of a fully analyzed fixture.

## Deferred

No equations from Chapter 22 Sections 3 or 4 appear unimplemented or stubbed. The one explicitly noted simplification is the nonyielding-bypass zero-delay/zero-capacity treatment described above, which reproduces the one fixture case that exercises it (Example Problem 1's SB bypass) but is not a literal HCM equation. Equation 22-15 (per-movement weighted heavy-vehicle factor) is not implemented, per the input-model limitation noted above. No `todo!()` markers or `VERIFY-HCM` comments were found in `roundabouts.rs` or `tests.rs`.
