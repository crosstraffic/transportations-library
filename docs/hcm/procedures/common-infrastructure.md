# Common Infrastructure for Interrupted-Flow Chapters (HCM Chapters 19-23)

The `feat/hcm-shared-infra` branch (commit `ec40d0e`) adds five modules under `src/hcm/common/` that factor out the machinery shared by the interrupted-flow chapters of HCM 7th Edition: the intersection/movement data model of Chapter 19 Exhibit 19-1 (reused by Chapters 20-23), the control-delay equation family (Chapter 19 signalized d1/d2/d3 terms plus the structurally identical unsignalized forms of Chapters 20-22), the Chapter 20 gap-acceptance capacity core (reused by Chapters 22 and 23), LOS threshold tables from seven exhibits across Chapters 12-14 and 19-22, and analysis-period/demand-profile primitives for the future multiperiod Chapter 10/11 work. All five modules are declared in `src/hcm/common/mod.rs` (`pub mod delay; pub mod gap_acceptance; pub mod intersection; pub mod los_tables; pub mod time_period;`) alongside the pre-existing `adjustment_factors` and `pce_table`. Every public function carries its HCM equation or exhibit number in its doc comment, and every module has an inline `#[cfg(test)]` suite; this document walks each module in turn.

## `src/hcm/common/intersection.rs` — the NEMA movement model

Implements the intersection data model shared by Chapters 19 (signalized), 20 (TWSC), 21 (AWSC), 22 (roundabouts), and 23 (ramp terminals), with movement numbering per HCM Exhibit 19-1.

| Item | HCM source | Rust item | Inputs | Output |
|---|---|---|---|---|
| Movement numbering | Exhibit 19-1 | `nema_movement_number(direction, turn_type)` | `Direction` (NB/SB/EB/WB), `TurnType` (Left/Through/Right/UTurn) | `Option<u8>` NEMA number |
| Demand flow rate | Ch. 19 Step 2 / Ch. 20 Step 3 (v = V/PHF) | `Movement::demand_flow_rate()` | `volume` (veh/h), `phf` (unitless 0-1, optional) | veh/h |
| Approach totals | — (aggregation helpers) | `Approach::total_volume()`, `Approach::total_flow_rate()` | movement list | veh/h |
| Intersection totals / lookup | — | `Intersection::total_volume()`, `total_flow_rate()`, `approach(direction)` | approach list | veh/h / `Option<&Approach>` |

The numbering table transcribed into the `match` in `nema_movement_number`: left turns EB=5, WB=1, NB=3, SB=7; throughs EB=2, WB=6, NB=8, SB=4; right turns are through-plus-ten (12/16/18/14), which is asserted structurally by the `test_right_turn_is_through_plus_ten` unit test. U-turns return `None` with a doc-comment rationale (Exhibit 19-1 assigns no distinct number; Chapter 20 denotes major-street U-turns 1U/4U). Pedestrian phases (2P/4P/6P/8P) are documented in the doc comment but not represented in the type system — there is no pedestrian movement variant, so chapters needing pedestrian phases will have to extend `TurnType` or model them separately.

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

Constants: `K_PRETIMED = 0.50` (Ch. 19 Step 8 Part C recommendation for pretimed/coordinated/recall-to-max phases), `K_MIN_LOWER_BOUND = 0.04` (Eq 19-23 floor), `I_ISOLATED = 1.0` (Eq 19-6 discussion, intersections ≥0.6 mi from the nearest upstream signal), `I_MIN = 0.090` (Eq 19-6 floor). `initial_queue_delay` implements the Eq 19-45/19-46/19-47/19-48/19-49 case split (`v >= c_A`: Q_eo = T(v-c_A), t_A = T; `v < c_A`: Q_eo = 0, t_A = min(Q_b/(c_A - v), T)) and returns 0.0 for Q_b ≤ 0 per Ch. 19 Step 8 Part B. `uniform_delay` caps X at 1.0 inside the denominator (`x.min(1.0)`), matching the min(1, X) term in Eq 19-19.

### Unsignalized family (Chapters 20, 21, 22)

| Delay form | HCM Eq. | Rust function | Inputs (units) | Chapter |
|---|---|---|---|---|
| TWSC movement control delay | 20-61 | `control_delay_unsignalized(volume, capacity, t_h)` | v_x (veh/h), c_m,x (veh/h), T (h) | 20 |
| Roundabout lane control delay | 22-17 | `control_delay_roundabout(volume, capacity, t_h)` | lane v (veh/h), c (veh/h), T (h) | 22 |
| AWSC lane control delay | 21-30 | `control_delay_awsc(service_time_s, departure_headway_s, x, t_h)` | t_s (s), h_d (s), x = v·h_d/3600 (unitless), T (h) | 21 |

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
- **What the tests pin down**: `intersection.rs` pins the full Exhibit 19-1 numbering table and the right-equals-through-plus-ten structure; `delay.rs` pins limit behavior (d2 → 0 as x → 0, d3 = 0 at Q_b = 0), continuity of Eq 19-26 through x = 1, monotonicity in x, the Eq 19-23/19-22 clamps, the Eq 19-6 floor at 0.090 with X_u capped at 1.0, the oversaturated d3 case against a hand-expanded expected value, and the k·I = 1 algebraic identity linking the unsignalized family to Eq 19-26; `gap_acceptance.rs` pins the v_c → 0 limit (3600/t_f), monotonic decrease of c_p in v_c, and the [0,1] clamps; `los_tables.rs` probes every threshold boundary from both sides for all seven exhibits; `time_period.rs` pins the count→flow conversion and peak lookup including the empty-profile edge.
- **Run**: `cargo test` (no features needed; none of these modules are behind `with-python`).
- No `docs/hcm/VERIFICATION.md` exists on this branch (`git ls-tree -r` confirms), so there are no ledger entries to cross-reference; the deviations below are documented inline.

## Deviations

1. `progression_factor` (Eq 19-20) has no guard against `g_over_c == 1.0` or `x.min(1.0) * p == 1.0`, both of which divide by zero; the HCM presumes g/C < 1 so this is a domain assumption rather than a transcription error, but callers get `inf`/`NaN` rather than an error on bad input. The same applies to `uniform_delay` at g/C = 1 with X = 1.
2. `control_delay_unsignalized`, `control_delay_roundabout`, and `potential_capacity` divide by `capacity`/`t_f` without a zero guard (`potential_capacity` guards `v_c <= 0` but not `t_f <= 0`). Consistent with the rest of the library's convention of trusting HCM-plausible inputs, but noted since `delay.rs`'s `initial_queue_delay` *does* defensively return 0.0 on degenerate inputs — the defensive posture is inconsistent across the module.
3. The doc comment on `prob_queue_free` extends Eq 20-28 to the Rank 2 U-turn adjustment factors f_1U/f_4U (Eq 20-24/20-25) by structural identity; the manual presents those as distinct equations, so a reviewer should confirm the identification (the algebraic form 1 − v/c is the same, but the manual's variable bindings differ).
4. Pedestrian movements (2P/4P/6P/8P per Exhibit 19-1) are documented but not representable in the `Movement`/`TurnType` model — deferred to the chapters that need them.

## Deferred

- Rank 2/3/4 movement-hierarchy wiring for TWSC (which impedance terms apply to which movement) — intentionally left to the Chapter 20 implementation; this module only supplies the factors.
- Conflicting-flow computation (HCM Equations 20-3 through 20-17) is not implemented here; `potential_capacity` takes v_c as an input.
- Critical/follow-up headway base values and adjustments (Exhibits 20-14/20-15, Eq 20-19 to 20-21) are not tabulated; t_c and t_f are caller inputs.
- Multiperiod chaining logic for Chapters 10/11 (`time_period.rs` explicitly marks itself as a growth point).
- Pedestrian and bicycle LOS tables for Chapters 19-22 (only motorized-vehicle exhibits are transcribed in `los_tables.rs`).
