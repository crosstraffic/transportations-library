# Common Infrastructure for Interrupted-Flow Chapters (HCM Chapters 19-23)

The `feat/hcm-shared-infra` branch (commit `ec40d0e`) adds five modules under `src/hcm/common/` that factor out the machinery shared by the interrupted-flow chapters of HCM 7th Edition: the intersection/movement data model of Chapter 19 Exhibit 19-1 (reused by Chapters 20-23), the control-delay equation family (Chapter 19 signalized d1/d2/d3 terms plus the structurally identical unsignalized forms of Chapters 20-22), the Chapter 20 gap-acceptance capacity core (reused by Chapters 22 and 23), LOS threshold tables from seven exhibits across Chapters 12-14 and 19-22, and analysis-period/demand-profile primitives for the future multiperiod Chapter 10/11 work. All five modules are declared in `src/hcm/common/mod.rs` (`pub mod delay; pub mod gap_acceptance; pub mod intersection; pub mod los_tables; pub mod time_period;`) alongside the pre-existing `adjustment_factors` and `pce_table`. Every public function carries its HCM equation or exhibit number in its doc comment, and every module has an inline `#[cfg(test)]` suite; this document walks each module in turn.

## `src/hcm/common/intersection.rs` — the NEMA movement model

Implements the intersection data model shared by Chapters 19 (signalized), 20 (TWSC), 21 (AWSC), 22 (roundabouts), and 23 (ramp terminals), with movement numbering per HCM Exhibit 19-1.

| Item | HCM source | Rust item | Inputs | Output |
|---|---|---|---|---|
| Movement numbering | Exhibit 19-1 | `nema_movement_number(direction, turn_type)` | `Direction` (NB/SB/EB/WB), `TurnType` (Left/Through/Right/UTurn) | `Option<u8>` NEMA number |
| Ch. 20 U-turn adjunct label | Exhibit 20-1 | `ch20_uturn_label(direction)` | `Direction` (NB/SB/EB/WB) | `Option<&'static str>` ("1U"/"4U"/`None`) |
| Demand flow rate | Ch. 19 Step 2 / Ch. 20 Step 3 (v = V/PHF) | `Movement::demand_flow_rate()` | `volume` (veh/h), `phf` (unitless 0-1, optional) | veh/h |
| Approach totals | — (aggregation helpers) | `Approach::total_volume()`, `Approach::total_flow_rate()` | movement list | veh/h |
| Intersection totals / lookup | — | `Intersection::total_volume()`, `total_flow_rate()`, `approach(direction)` | approach list | veh/h / `Option<&Approach>` |

The numbering table transcribed into the `match` in `nema_movement_number`: left turns EB=5, WB=1, NB=3, SB=7; throughs EB=2, WB=6, NB=8, SB=4; right turns are through-plus-ten (12/16/18/14), which is asserted structurally by the `test_right_turn_is_through_plus_ten` unit test. U-turns return `None` for every direction — Exhibit 19-1 has no NEMA phase slot for U-turns at all, so this is not a direction-specific gap. That is a separate fact from Chapter 20's own Exhibit 20-1 movement numbering, which *does* give the two major-street U-turns adjunct labels 1U (EB) and 4U (WB) — minor-street (NB/SB) U-turns are unlabeled there too. `ch20_uturn_label(direction)` implements that Chapter-20-specific lookup (verified by `test_ch20_uturn_label` and `test_nema_movement_number_uturn_always_none`); it is intentionally a separate function rather than a change to `nema_movement_number`, since the two exhibits assign different numeric slots to the same cardinal directions (movement 1 is a WB left under Exhibit 19-1 but an EB left under Exhibit 20-1) and Chapter 19 code depends on `nema_movement_number`'s `None` for `UTurn` staying as-is. Pedestrian phases (2P/4P/6P/8P) are documented in the doc comment but not represented in the type system — there is no pedestrian movement variant, so chapters needing pedestrian phases will have to extend `TurnType` or model them separately.

`Movement` carries `movement_no: Option<u8>`, `turn_type`, `volume` (veh/h), `phf: Option<f64>`, `heavy_vehicle_pct: Option<f64>` (%), `lanes: u32`, `shared_lane: Option<bool>`. `Movement::demand_flow_rate()` returns `volume / phf` when `phf` is `Some` and positive, otherwise returns `volume` unchanged under the documented convention that a missing PHF means the volume is already a flow rate. `ControlType` enumerates PretimedSignal / ActuatedSignal / SemiActuatedSignal (Ch. 19), TwoWayStop (Ch. 20), AllWayStop (Ch. 21), Roundabout (Ch. 22), and YieldControl. All types are serde-serializable and round-trip tested (`test_serde_roundtrip`).

## `src/hcm/common/delay.rs` — control-delay building blocks

Units throughout: delays s/veh, capacities veh/h, analysis period `t_h` in hours (0.25 h for the standard 15-min period).

### Signalized (Chapter 19) d1/d2/d3 terms

| Delay form | HCM Eq. | Rust function | Inputs (units) | Consumed by |
|---|---|---|---|---|
| Flow ratio y | 19-21 | `flow_ratio(x, g_over_c)` | X (unitless, min'd with 1), g/C | internal to PF |
| Progression adjustment PF | 19-20 | `progression_factor(p, g_over_c, x)` | P (decimal arrivals on green), g/C, X | Ch. 19 (coordinated/platooned arrivals) |
| Uniform delay d1 | 19-19 | `uniform_delay(cycle_s, green_s, x, pf)` | C (s), g (s), X, PF | Ch. 19 |
| Upstream filtering I | 19-6 | `upstream_filtering_factor(x_u)` | weighted upstream X_u (capped to [0,1]) | Ch. 19 d2 at nonisolated intersections |
| k_min (actuated) | 19-23 | `incremental_delay_factor_min(passage_time_s)` | passage time PT (s) | Ch. 19 actuated phases |
| k (actuated) | 19-22 | `incremental_delay_factor_actuated(v_over_ca, k_min)` | v/c_a (Eq 19-24), k_min | Ch. 19 actuated phases |
| Incremental delay d2 | 19-26 | `incremental_delay_signalized(t_h, x, capacity, k, i_factor)` | T (h), X_A (Eq 19-27), c_A (veh/h), k, I | Ch. 19 |
| Generic d2 alias | 19-26 | `incremental_delay(...)` | same | any chapter reusing the 900T[(x-1)+sqrt(...)] form |
| Initial queue delay d3 | 19-44 to 19-49 | `initial_queue_delay(queue_initial_veh, v, capacity, t_h)` | Q_b (veh), v (veh/h), c_A (veh/h), T (h) | Ch. 19 |
| Control delay d = d1+d2+d3 | 19-18 | `control_delay_signalized(d1, d2, d3)` | s/veh each | Ch. 19 |
| Aggregation | 19-28 / 20-64 (also 19-29 / 20-65 intersectionwide) | `aggregate_control_delay(&[(d, v)])` | (s/veh, veh/h) pairs | Ch. 19 and Ch. 20 approach/intersection rollups |

#### Full equation set, Eqs 19-18 through 19-28 and 19-44 through 19-49

Every equation and constant below was cross-checked against the HCM 7th Edition MathML source (`resources/epub/OEBPS/137_Ch19_03.xhtml`, `138_Ch19_04.xhtml`) in the user's main checkout; all match the current Rust implementation exactly, with no discrepancies found.

```
Equation 19-21:  y = min(1, X)·(g/C)
  y = flow ratio (unitless)
  X = lane group volume-to-capacity ratio (unitless)
  g/C = effective green ratio (unitless)
Implemented in: common/delay.rs::flow_ratio
```

```
Equation 19-20:  PF = [(1 − P)/(1 − g/C)] · [(1 − y)/(1 − min(1, X)·P)] · [1 + y·(1 − P·C/g)/(1 − g/C)]
  PF = progression adjustment factor (unitless; 1.0 for random arrivals when P = g/C at low X)
  P = proportion of vehicles arriving during the green indication (decimal, 0–1)
  g/C = effective green ratio (unitless)
  X = lane group volume-to-capacity ratio (unitless)
  y = flow ratio, min(1, X)·g/C (Eq 19-21)
  Guard: returns 1.0 (no adjustment) when g/C ≥ 1.0 — the analytic limit, since term1 and term3 would otherwise divide by 1 − g/C = 0 (see the Deviations section for the residual, narrower unguarded edge case)
Implemented in: common/delay.rs::progression_factor
```

```
Equation 19-19:  d1 = PF·[0.5·C·(1 − g/C)²] / [1 − min(1, X)·(g/C)]
  d1 = uniform delay, s/veh
  C = cycle length, s
  g = effective green time, s
  X = lane group volume-to-capacity ratio (unitless)
  PF = progression adjustment factor (Eq 19-20; 1.0 for random arrivals)
  Guard: returns 0.0 when g/C ≥ 1.0 (no red interval), the analytic limit of the formula as g/C → 1 for both X ≥ 1 and X < 1
Implemented in: common/delay.rs::uniform_delay
```

```
Equation 19-6:  I = 1.0 − 0.91·X_u^2.68, floored at 0.090
  I = upstream filtering adjustment factor (unitless, range [0.090, 1.0])
  X_u = weighted volume-to-capacity ratio of upstream movements feeding the subject movement group, capped at 1.0
  Constants: I_ISOLATED = 1.0 (intersections ≥0.6 mi from the nearest upstream signal, HCM Chapter 19 Eq 19-6 discussion); I_MIN = 0.090 (the equation's own floor)
Implemented in: common/delay.rs::upstream_filtering_factor
```

```
Equation 19-23:  k_min = −0.375 + 0.354·PT − 0.0910·PT² + 0.00889·PT³, floored at 0.04
  k_min = minimum incremental delay factor for an actuated phase (unitless)
  PT = passage time (unit extension) controller setting, s
  Constant: K_MIN_LOWER_BOUND = 0.04 (the equation's own floor)
Implemented in: common/delay.rs::incremental_delay_factor_min
```

```
Equation 19-22:  k = (1 − 2·k_min)·(v/c_a − 0.5) + k_min, clamped to [k_min, 0.50]
  k = incremental delay factor for an actuated phase (unitless)
  k_min = minimum incremental delay factor (Eq 19-23)
  v/c_a = ratio of demand flow rate to available capacity for the phase (Eq 19-24)
  Constant: K_PRETIMED = 0.50 is used directly in place of this equation for pretimed, coordinated, and recall-to-maximum phases (HCM Chapter 19, Step 8, Part C)
Implemented in: common/delay.rs::incremental_delay_factor_actuated
```

```
Equation 19-26:  d2 = 900·T·[(X_A − 1) + sqrt((X_A − 1)² + (8·k·I·X_A)/(c_A·T))]
  d2 = incremental delay, s/veh
  T = analysis period duration, h (default 0.25 h = 15 min, DEFAULT_ANALYSIS_PERIOD_H in common/time_period.rs)
  X_A = ratio of flow to capacity for the incremental delay calculation (Eq 19-27, X_A = v/c_A)
  k = incremental delay factor dependent on controller type (0.50 for pretimed, K_PRETIMED; Eqs 19-22/19-23 for actuated)
  I = upstream filtering/metering adjustment factor (1.0 isolated intersection, I_ISOLATED; Eq 19-6 for the general case, floored at 0.090, I_MIN)
  c_A = adjusted lane group capacity, veh/h
Implemented in: common/delay.rs::incremental_delay_signalized (generic alias: common/delay.rs::incremental_delay, reused verbatim by the unsignalized family below with k·I fixed at 1.0 — see the algebraic-identity note in the source and the Deviations section)
```

```
Equation 19-44:  d3 = [3,600/(v·T)] · { t_A·(Q_b + Q_e − Q_eo)/2 + (Q_e² − Q_eo²)/(2·c_A) − Q_b²/(2·c_A) }
  d3 = initial queue delay, s/veh
  v = demand flow rate, veh/h
  T = analysis period duration, h (default 0.25 h)
  t_A = duration of unmet demand in the analysis period, h (Eqs 19-47/19-49)
  Q_b = initial queue at the start of the analysis period, veh
  Q_e = queue at the end of the analysis period (residual queue), veh (Eq 19-45)
  Q_eo = queue at the end of the analysis period if Q_b were 0.0 veh, veh (Eqs 19-46/19-48)
  c_A = adjusted lane group capacity, veh/h
  Special case: d3 = 0.0 s/veh whenever Q_b ≤ 0 (HCM Chapter 19, Step 8, Part B)
Implemented in: common/delay.rs::initial_queue_delay
```

```
Equation 19-45:  Q_e = Q_b + t_A·(v − c_A)
  Q_e = residual queue at the end of the analysis period, veh
  Q_b = initial queue at the start of the analysis period, veh
  t_A = duration of unmet demand, h (Eqs 19-47/19-49)
  v = demand flow rate, veh/h
  c_A = adjusted lane group capacity, veh/h
Implemented in: common/delay.rs::initial_queue_delay (inline); common/delay.rs::queue_end_of_period computes the same Q_e standalone for multiperiod residual-queue hand-off (HCM Chapter 17, Section 3 / Chapter 29, Section 3)
```

```
Equation 19-46:  Q_eo = T·(v − c_A)                    [case v ≥ c_A]
Equation 19-47:  t_A = T                                [case v ≥ c_A]
  Q_eo = queue at the end of the analysis period if Q_b were 0.0 veh, veh
  T = analysis period duration, h
  v = demand flow rate, veh/h
  c_A = adjusted lane group capacity, veh/h
Implemented in: common/delay.rs::initial_queue_delay, common/delay.rs::queue_end_of_period (the `v >= capacity` branch)
```

```
Equation 19-48:  Q_eo = 0.0 veh                                    [case v < c_A]
Equation 19-49:  t_A = min(Q_b/(c_A − v), T)                       [case v < c_A]
  Q_eo = queue at the end of the analysis period if Q_b were 0.0 veh, veh
  Q_b = initial queue, veh
  c_A = adjusted lane group capacity, veh/h
  v = demand flow rate, veh/h
  T = analysis period duration, h
Implemented in: common/delay.rs::initial_queue_delay, common/delay.rs::queue_end_of_period (the `v < capacity` branch, with the `.min(t_h)` cap matching the HCM's own "≤ T" qualifier on Eq 19-49)
```

```
Equation 19-18:  d = d1 + d2 + d3
  d = control delay per vehicle, s/veh
  d1 = uniform delay, s/veh (Eq 19-19)
  d2 = incremental delay, s/veh (Eq 19-26)
  d3 = initial queue delay, s/veh (Eqs 19-44 through 19-49)
Implemented in: common/delay.rs::control_delay_signalized
```

```
Equation 19-28:  d_A,j = Σ(d_i·v_i) / Σ(v_i)     for movements/lane groups i = 1..m_j in approach/group j
  d_A,j = aggregated control delay for approach/lane group j, s/veh
  d_i = control delay of movement/lane group i, s/veh
  v_i = flow rate of movement/lane group i, veh/h
  The same weighted-average form aggregates to intersectionwide delay (Eq 19-29) and is reused for TWSC approach/intersection delay (Eqs 20-64/20-65)
Implemented in: common/delay.rs::aggregate_control_delay
```

Constants: `K_PRETIMED = 0.50` (Ch. 19 Step 8 Part C recommendation for pretimed/coordinated/recall-to-max phases), `K_MIN_LOWER_BOUND = 0.04` (Eq 19-23 floor), `I_ISOLATED = 1.0` (Eq 19-6 discussion, intersections ≥0.6 mi from the nearest upstream signal), `I_MIN = 0.090` (Eq 19-6 floor). `initial_queue_delay` implements the Eq 19-45/19-46/19-47/19-48/19-49 case split (`v >= c_A`: Q_eo = T(v-c_A), t_A = T; `v < c_A`: Q_eo = 0, t_A = min(Q_b/(c_A - v), T)) and returns 0.0 for Q_b ≤ 0 per Ch. 19 Step 8 Part B. `uniform_delay` caps X at 1.0 inside the denominator (`x.min(1.0)`), matching the min(1, X) term in Eq 19-19.

Both `uniform_delay` and `progression_factor` now guard `g_over_c >= 1.0` (no red interval) as an early return rather than letting their respective denominators (`1 - min(1,X)·g/C` for `uniform_delay`; `1 - g/C` in both the term1 and term3 factors of `progression_factor`) hit zero. `uniform_delay` returns `0.0`: taking `g/C -> 1^-` with `X >= 1` fixed, `0.5 C (1-g/C)^2 / (1-g/C) = 0.5 C (1-g/C) -> 0`, and for `X < 1` the unguarded formula already gives the same `0` (its denominator tends to `1 - X > 0`), so the guard changes no observable value, it only replaces the `X >= 1` `0/0` with the correct limit. `progression_factor` returns `1.0` (no adjustment) at `g/C >= 1.0`, matching the guard already used at its chapter23 `ramp_terminals.rs` call site (`if g_over_c < 1.0 { progression_factor(..) } else { 1.0 }`) — PF's value is moot there since `uniform_delay` (its only consumer) tends to 0 regardless of PF in that limit. `test_uniform_delay_g_over_c_one_exact`, `test_uniform_delay_g_over_c_near_one_continuity`, `test_uniform_delay_x_above_one_at_g_over_c_one`, and `test_progression_factor_g_over_c_one_returns_one` cover the exact boundary, the `g/C = 0.9999` continuity approach, and `X > 1` at `g/C = 1.0`.

### Unsignalized family (Chapters 20, 21, 22)

| Delay form | HCM Eq. | Rust function | Inputs (units) | Chapter |
|---|---|---|---|---|
| TWSC movement control delay | 20-61 | `control_delay_unsignalized(volume, capacity, t_h)` | v_x (veh/h), c_m,x (veh/h), T (h) | 20 |
| Roundabout lane control delay | 22-17 | `control_delay_roundabout(volume, capacity, t_h)` | lane v (veh/h), c (veh/h), T (h) | 22 |
| AWSC lane control delay | 21-30 | `control_delay_awsc(service_time_s, departure_headway_s, x, t_h)` | t_s (s), h_d (s), x = v·h_d/3600 (unitless), T (h) | 21 |

#### Full equation set, Eqs 20-61, 21-30, 22-17

Cross-checked against `resources/epub/OEBPS/146_Ch20_03.xhtml` (Eq 20-61), `155_Ch21_03.xhtml` (Eq 21-30), and `164_Ch22_03.xhtml` (Eq 22-17); all three EPUB sources were available and all match the Rust implementation exactly.

```
Equation 20-61:  d = 3,600/c_m,x + 900·T·[(v_x/c_m,x − 1) + sqrt((v_x/c_m,x − 1)² + (3,600/c_m,x)·(v_x/c_m,x)/(450·T))] + 5
  d = control delay, s/veh
  v_x = movement demand flow rate, veh/h
  c_m,x = movement capacity, veh/h
  T = analysis period duration, h (default 0.25 h)
  +5 s/veh = deceleration/acceleration stop penalty (full stop assumed for the TWSC minor-street/major-street-left movement)
Implemented in: common/delay.rs::control_delay_unsignalized
```

```
Equation 21-30:  d = t_s + 900·T·[(x − 1) + sqrt((x − 1)² + h_d·x/(450·T))] + 5
  d = control delay, s/veh
  t_s = service time, s
  h_d = departure headway, s
  x = degree of utilization, x = v·h_d/3,600 (unitless)
  T = analysis period duration, h
  +5 s/veh = deceleration/acceleration stop penalty (all-way stop control, every vehicle stops)
Implemented in: common/delay.rs::control_delay_awsc
```

```
Equation 22-17:  d = 3,600/c + 900·T·[(x − 1) + sqrt((x − 1)² + (3,600/c)·x/(450·T))] + 5·min(x, 1)
  d = control delay, s/veh
  c = lane capacity, veh/h
  x = degree of saturation v/c (unitless)
  T = analysis period duration, h
  5·min(x, 1) = YIELD-control stop penalty, scaling toward 0 s/veh as conflict vanishes (x → 0) instead of the full +5 s/veh STOP penalty used by Eqs 20-61/21-30
Implemented in: common/delay.rs::control_delay_roundabout
```

All three share the `900T[(x-1) + sqrt((x-1)^2 + radicand)]` structure. The doc comment on `incremental_delay` records the algebraic identity that ties the family to the Chapter 19 form — the unsignalized radicand `(3600/c)·x/(450T)` equals `8x/(cT)`, i.e., Eq 19-26 with k·I = 1 — and the unit test `test_unsignalized_radicand_equals_generic_with_ki_one` verifies it numerically. The three forms differ only in their leading term and their stop penalty: TWSC is `3600/c + ... + 5` (full +5 s/veh stop penalty), roundabout is `3600/c + ... + 5·min(x, 1)` (yield control, penalty scales away at low conflict), AWSC is `t_s + ... + 5` (service time replaces 3600/c). These distinctions are each verified by a dedicated test (`test_control_delay_roundabout_yield_term`, `test_control_delay_awsc_zero_utilization`).

## `src/hcm/common/gap_acceptance.rs` — gap-acceptance capacity

The Chapter 20 TWSC capacity core, reused by Chapters 22 (roundabouts) and 23 (ramp terminals) per the module doc comment. Units: flows/capacities veh/h, headways s.

| Item | HCM Eq. | Rust function | Inputs (units) | Output |
|---|---|---|---|---|
| Potential capacity | 20-18 | `potential_capacity(v_c, t_c, t_f)` | conflicting flow v_c (veh/h), critical headway t_c (s), follow-up headway t_f (s) | c_p (veh/h); returns 3600/t_f explicitly when v_c ≤ 0 (the analytic limit) |
| Queue-free probability | 20-28 (also the f_1U/f_4U U-turn factors of Eq 20-24/20-25) | `prob_queue_free(v, c_m)` | v (veh/h), c_m (veh/h) | p_0 clamped to [0, 1]; 0.0 if c_m ≤ 0 |
| Vehicular impedance | 20-35 | `vehicular_impedance_factor(&[p0])` | queue-free probabilities of impeding higher-rank movements | product (1.0 for empty slice) |
| Pedestrian blockage | 20-67 | `pedestrian_blockage_factor(v_ped, lane_width_ft, walking_speed_ft_s)` | p/h, ft, ft/s | proportion of time blocked |
| Pedestrian impedance | 20-68 | `pedestrian_impedance_factor(f_pb)` | blockage factor | p_p clamped to [0, 1] |
| Movement capacity | 20-22 / 20-26 / 20-36 | `movement_capacity(c_p, impedance)` | c_p (veh/h), combined impedance factor | c_m (veh/h) |

#### Full equation set, Eq 20-18 and the impedance chain (Eqs 20-28, 20-35, 20-67, 20-68, 20-22/20-26/20-36)

Cross-checked against `resources/epub/OEBPS/146_Ch20_03.xhtml` (Eqs 20-18, 20-28, 20-35, 20-22, 20-26) and `147_Ch20_04.xhtml` (Eqs 20-67, 20-68, 20-36); all match the current Rust implementation exactly.

```
Equation 20-18:  c_p,x = v_c,x·exp(−v_c,x·t_c,x/3,600) / [1 − exp(−v_c,x·t_f,x/3,600)]
  c_p,x = potential capacity of minor movement x, veh/h
  v_c,x = conflicting flow rate, veh/h
  t_c,x = critical headway, s
  t_f,x = follow-up headway, s
  Limit: c_p,x → 3,600/t_f,x as v_c,x → 0 (returned explicitly for v_c,x ≤ 0)
Implemented in: common/gap_acceptance.rs::potential_capacity
```

```
Equation 20-28:  p0,j = 1 − v_j/c_m,j
  p0,j = probability that movement j operates in a queue-free state (unitless, clamped to [0, 1])
  v_j = demand flow rate of movement j, veh/h
  c_m,j = movement capacity of movement j, veh/h
  Extended by structural identity to the Rank 2 U-turn adjustment factors f_1U/f_4U (Eqs 20-24/20-25); flagged as a reviewer-confirm item in Deviations item 3 below, since the HCM presents those as formally distinct equations sharing the same 1 − v/c form
Implemented in: common/gap_acceptance.rs::prob_queue_free
```

```
Equation 20-35:  f_k = Π_j p0,j
  f_k = vehicular impedance (capacity adjustment) factor for Rank 3 (and, combined with additional terms, Rank 4) movement k (unitless)
  p0,j = queue-free probability of each impeding higher-rank movement j (Eq 20-28)
  Product over an empty movement set returns 1.0 (no impedance)
Implemented in: common/gap_acceptance.rs::vehicular_impedance_factor
```

```
Equation 20-67:  f_pb = (v_x·w/S_p) / 3,600
  f_pb = pedestrian blockage factor, proportion of time the lane is blocked by pedestrians (unitless)
  v_x = pedestrian flow rate of the conflicting pedestrian movement, p/h
  w = width of the lane the minor movement is negotiating into, ft
  S_p = pedestrian walking speed, ft/s (default PEDESTRIAN_WALKING_SPEED_FT_S = 3.5 ft/s)
Implemented in: common/gap_acceptance.rs::pedestrian_blockage_factor
```

```
Equation 20-68:  p_p,x = 1 − f_pb
  p_p,x = pedestrian impedance factor for movement x (unitless, clamped to [0, 1])
  f_pb = pedestrian blockage factor (Eq 20-67)
Implemented in: common/gap_acceptance.rs::pedestrian_impedance_factor
```

```
Equations 20-22, 20-26, 20-36:  c_m = c_p · f
  c_m = movement capacity, veh/h
  c_p = potential capacity (Eq 20-18), veh/h
  f = combined capacity adjustment factor: 1.0 for unimpeded Rank 2 major-street left turns (Eq 20-22); vehicular impedance × pedestrian impedance for Rank 3/4 movements and U-turns (Eqs 20-26, 20-36)
Implemented in: common/gap_acceptance.rs::movement_capacity
```

The constant `PEDESTRIAN_WALKING_SPEED_FT_S = 3.5` transcribes the Chapter 20 Eq 20-67 variable-definition assumption. Note the composition responsibility is the caller's: `movement_capacity` takes a single pre-combined impedance argument, and the rank-2/3/4 wiring (which p_0 terms multiply into which movement, per the Chapter 20 rank hierarchy) is deliberately not encoded here — a chapter-20 implementation must assemble `vehicular_impedance_factor(...) * pedestrian_impedance_factor(...)` itself per movement rank. `test_movement_capacity_composition` demonstrates the intended composition.

## `src/hcm/common/los_tables.rs` — LOS threshold tables

All functions take the service-measure value plus a `demand_exceeds_capacity`/`vc_gt_1` boolean that forces LOS F, and return the `LevelOfService` enum from `common::mod`. Delay thresholds s/veh; density thresholds pc/mi/ln.

| Function | HCM exhibit(s) | Thresholds (A/B/C/D/E upper bounds) | Consumed by |
|---|---|---|---|
| `los_signalized_intersection(control_delay_s, vc_gt_1)` | Exhibit 19-8 | 10 / 20 / 35 / 55 / 80 s/veh | Ch. 19 |
| `los_unsignalized(control_delay_s, vc_gt_1)` | Exhibits 20-2, 21-8, 22-8 (identical thresholds, one function) | 10 / 15 / 25 / 35 / 50 s/veh | Ch. 20, 21, 22 |
| `los_basic_freeway(density, demand_exceeds_capacity)` | Exhibit 12-15 | 11 / 18 / 26 / 35 / 45 pc/mi/ln (>45 is also F) | Ch. 12 |
| `los_multilane(density, demand_exceeds_capacity)` | Exhibit 12-15 (same boundaries per Ch. 12 text; delegates to `los_basic_freeway`) | same | Ch. 12 |
| `los_weaving(density, demand_exceeds_capacity, facility)` | Exhibit 13-6 | Freeway: 10 / 20 / 28 / 35 / 43; Multilane/C-D: 12 / 24 / 32 / 36 / 40 | Ch. 13 |
| `los_merge_diverge(density, demand_exceeds_capacity)` | Exhibit 14-3 | 10 / 20 / 28 / 35 / (density > 35 is E, not F) | Ch. 14 |

Two table-semantics subtleties are encoded and doc-commented rather than left implicit. First, Exhibit 14-3 differs from Exhibits 12-15 and 13-6 in that LOS F is assigned *only* on demand > capacity — density above 35 pc/mi/ln alone remains LOS E — and `los_merge_diverge`'s match arm structure (`_ => LevelOfService::E`) plus `test_los_merge_diverge_boundaries_exhibit_14_3` (density 50.0 → E) enforce that. Second, the exhibit notes about which assessments use the v/c override are carried in doc comments: Exhibit 19-8 approach/intersectionwide LOS is delay-only (callers pass `vc_gt_1 = false`), Exhibit 20-2 applies per lane/movement with no whole-intersection TWSC LOS defined, and Exhibits 21-8/22-8 approach/intersection LOS are delay-only. `WeavingFacilityType` (Freeway vs. MultilaneOrCDRoad) selects the Exhibit 13-6 column.

Boundary behavior: all thresholds are inclusive upper bounds (`d <= 10.0` is A, `10.01` is B), matched exactly by the boundary tests (`test_los_signalized_boundaries_exhibit_19_8` etc., which probe every breakpoint from both sides).

## `src/hcm/common/time_period.rs` — analysis period primitives

Minimal multiperiod scaffolding for the future Chapter 10/11 freeway-facility work (module doc comment states it "will grow as those chapters are implemented"). `DEFAULT_ANALYSIS_PERIOD_H = 0.25` encodes the standard HCM 15-min period. `AnalysisPeriod { duration_h, num_periods }` provides `total_duration_h()` and `count_to_flow_rate(count_veh)` (a 15-min count × 4 → veh/h). `DemandProfile { period_volumes }` (counts per period, veh) provides `peak_period_index()`, `peak_period_volume()`, `total_volume()`, and `flow_rates(&AnalysisPeriod)`. No HCM equation numbers apply; these are unit-conversion and bookkeeping types.

## Validation

- **Test style**: all validation on this branch is *inline unit tests* within each module (`#[cfg(test)] mod tests`), not JSON fixtures — no files were added under `tests/ExampleCases/` for this infrastructure, which is appropriate since these are building blocks rather than complete chapter procedures with published worked examples. Test tolerances are explicit epsilon comparisons, typically `abs() < 1e-9` for closed-form arithmetic checks and `1e-12` for exact algebraic identities; there are no manual-example reproductions at this layer.
- **What the tests pin down**: `intersection.rs` pins the full Exhibit 19-1 numbering table and the right-equals-through-plus-ten structure, that `nema_movement_number` returns `None` for `UTurn` regardless of direction, and the Chapter 20 Exhibit 20-1 1U/4U labels from `ch20_uturn_label`; `delay.rs` pins limit behavior (d2 → 0 as x → 0, d3 = 0 at Q_b = 0), the g/C → 1 guard on `uniform_delay`/`progression_factor` (exact g/C = 1.0 at X ≥ 1 and X < 1, continuity approaching g/C = 0.9999, and X > 1 at g/C = 1.0), continuity of Eq 19-26 through x = 1, monotonicity in x, the Eq 19-23/19-22 clamps, the Eq 19-6 floor at 0.090 with X_u capped at 1.0, the oversaturated d3 case against a hand-expanded expected value, and the k·I = 1 algebraic identity linking the unsignalized family to Eq 19-26; `gap_acceptance.rs` pins the v_c → 0 limit (3600/t_f), monotonic decrease of c_p in v_c, and the [0,1] clamps; `los_tables.rs` probes every threshold boundary from both sides for all seven exhibits; `time_period.rs` pins the count→flow conversion and peak lookup including the empty-profile edge.
- **Run**: `cargo test` (no features needed; none of these modules are behind `with-python`).
- No `docs/hcm/VERIFICATION.md` exists on this branch (`git ls-tree -r` confirms), so there are no ledger entries to cross-reference; the deviations below are documented inline.

## Deviations

1. ~~FIXED (feat/hcm-common-review-fixes): `progression_factor` and `uniform_delay` now guard `g_over_c >= 1.0` internally (see the `delay.rs` section above), returning the analytic `g/C -> 1` limit (PF = 1.0, d1 = 0.0) instead of propagating `NaN`/`inf`.~~ `progression_factor` (Eq 19-20) has no guard against `g_over_c == 1.0` or `x.min(1.0) * p == 1.0`, both of which divide by zero; the HCM presumes g/C < 1 so this is a domain assumption rather than a transcription error, but callers get `inf`/`NaN` rather than an error on bad input. The same applies to `uniform_delay` at g/C = 1 with X = 1. `progression_factor`'s `x.min(1.0) * p == 1.0` case in `term2`'s denominator (distinct from the `g_over_c` guard) remains unaddressed — that requires both X ≥ 1 and P = 1 (arrivals entirely on green with X at or above capacity), a narrower and separately-domained edge case not covered by this fix.
2. `control_delay_unsignalized`, `control_delay_roundabout`, and `potential_capacity` divide by `capacity`/`t_f` without a zero guard (`potential_capacity` guards `v_c <= 0` but not `t_f <= 0`). Consistent with the rest of the library's convention of trusting HCM-plausible inputs, but noted since `delay.rs`'s `initial_queue_delay` *does* defensively return 0.0 on degenerate inputs — the defensive posture is inconsistent across the module.
3. The doc comment on `prob_queue_free` extends Eq 20-28 to the Rank 2 U-turn adjustment factors f_1U/f_4U (Eq 20-24/20-25) by structural identity; the manual presents those as distinct equations, so a reviewer should confirm the identification (the algebraic form 1 − v/c is the same, but the manual's variable bindings differ).
4. Pedestrian movements (2P/4P/6P/8P per Exhibit 19-1) are documented but not representable in the `Movement`/`TurnType` model — deferred to the chapters that need them.

## Deferred

- Rank 2/3/4 movement-hierarchy wiring for TWSC (which impedance terms apply to which movement) — intentionally left to the Chapter 20 implementation; this module only supplies the factors.
- Conflicting-flow computation (HCM Equations 20-3 through 20-17) is not implemented here; `potential_capacity` takes v_c as an input.
- Critical/follow-up headway base values and adjustments (Exhibits 20-14/20-15, Eq 20-19 to 20-21) are not tabulated; t_c and t_f are caller inputs.
- Multiperiod chaining logic for Chapters 10/11 (`time_period.rs` explicitly marks itself as a growth point).
- Pedestrian and bicycle LOS tables for Chapters 19-22 (only motorized-vehicle exhibits are transcribed in `los_tables.rs`).
