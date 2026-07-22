# HCM Chapter 19 — Signalized Intersections, Motorized Vehicle Method (Steps 1–10)

This document walks the HCM 7th Edition Chapter 19 motorized-vehicle methodology for signalized intersections as implemented on branch `feat/hcm-ch19-signalized` (milestone 1: pretimed and coordinated/fixed-timing operation; the actuated phase-duration convergence loop is milestone 2, documented separately in `chapter19-actuated.md`). The methodology follows Chapter 19, Section 3 ("Motorized Vehicle Core Methodology," Exhibit 19-18's ten steps) together with the supplemental procedures of Chapter 31 ("Signalized Intersections: Supplemental") that Chapter 19 incorporates by reference for lane-group flow distribution (Chapter 31, Section 2), the permitted/protected-permitted left-turn saturation-flow and opposing-queue procedures (Chapter 31, Section 3), and back-of-queue/queue-storage-ratio (Chapter 31, Section 4). The two source files are `src/hcm/signalized/signalized.rs` (input model, the `SignalizedIntersection` facility struct, and the full ten-step pipeline) and `src/hcm/signalized/exhibits.rs` (coefficient tables and saturation-flow adjustment factors transcribed with per-exhibit citations). Shared delay/LOS primitives used by this chapter (and re-used by Chapters 20–23) live in `src/hcm/common/delay.rs` and `src/hcm/common/los_tables.rs`. `docs/hcm/VERIFICATION.md` does not exist yet as a file at this branch's tip — it is added later by `feat/hcm-ch19-actuated` — so the interpretation notes below are cited from that file's "Chapter 19 (feat/hcm-ch19-signalized)" section as read from the later commit's history, not from a file present in this branch's working tree. Every equation written out in the Equation reference below has been cross-checked against both the Rust function body and the HCM 7th Edition EPUB MathML source (`resources/epub/OEBPS/136_Ch19_02.xhtml`–`138_Ch19_04.xhtml` for the Chapter 19 body, `245_Ch31_02.xhtml`–`247_Ch31_04.xhtml` for the Chapter 31 supplemental procedures); newly found code-vs-book disagreements are flagged inline as **DISCREPANCY** blocks.

The public entry point is `SignalizedIntersection::analyze(&mut self)` in `signalized.rs`, which runs Steps 1–4 (jointly iterated), 5, 7, 8, 9, and 10 in sequence and stores results back onto `self.lane_groups`, `self.approach_results`, `self.intersection_delay_s`, `self.intersection_los`, and `self.critical_vc_ratio`. Step 6 (signal phase duration) is treated as an *input* for pretimed/coordinated control per the HCM's own Step 6 text — callers supply `PhaseTiming` (duration, yellow, red clearance, and optionally max green / passage time / walk / pedestrian clear) on each `SignalApproach`; the Chapter 31, Section 2 pretimed-design free functions (`cycle_length_for_target_xc`, `pretimed_effective_green`) are provided separately for callers who want to *derive* a timing plan rather than supply one.

## Step-by-step walkthrough

| HCM step | Equations / Exhibits | Rust function(s) / file | Inputs (units) | Outputs (units) |
|---|---|---|---|---|
| Step 1 — Movement and lane groups | Exhibit 19-19 (lane-group assignment rules) | `SignalizedIntersection::build_lane_groups` in `signalized.rs`; `LaneGroupKind` enum (`ExclusiveLeft`, `SharedLeftThrough`, `ExclusiveThrough`, `SharedRightThrough`, `ExclusiveRight`) | Per-approach lane counts (`exclusive_left_lanes`, `through_lanes`, `exclusive_right_lanes`: ln) and shared-lane flags (`shared_left_through_lane`, `shared_right_through_lane`: bool) from `SignalApproach` | `Vec<LaneGroup>` on `self.lane_groups`, one entry per materialized group with `direction`, `kind`, `phase_no`, `lanes` (ln) |
| Step 2 — Movement group flow rate | PHF adjustment (v = V/PHF); RTOR subtraction | `SignalApproach::flow_rates` and the Step-2 block inside `SignalizedIntersection::compute_states` in `signalized.rs` | `volume_left`/`volume_through`/`volume_right` (veh/h), `peak_hour_factor` (unitless, `None` ⇒ 1.0), `volume_rtor` (veh/h) | `ApproachState.v_lt`/`v_th`/`v_rt` (veh/h); `v_rt = max(V_r/PHF − RTOR, 0)` |
| Step 3 — Lane group flow rate on multi-lane approaches | Eqs. 31-46 through 31-66 (lane-flow equalization) | `SignalizedIntersection::distribute_lane_flows` and `assign_flows_simple` in `signalized.rs`; iterated jointly with Step 4 inside `compute_states` (up to 25 outer passes, convergence tolerance 0.05 veh/h or g_u, s/veh·h/ln, on the max of `{v_thru_group, v_sl_group, v_sr_group, g_u, s_shared_rt, s_shared_lt}`) | Movement-group flows (veh/h) from Step 2; lane geometry; `s_th_curb`/`s_th_excl` (veh/h/ln) and permitted-left parameters from Step 4 | `ApproachState.v_left_group`/`v_thru_group`/`v_sl_group`/`v_sr_group`/`v_right_group` (veh/h); shared-lane turn sub-flows `v_sl_lt`/`v_sr_rt` (veh/h) |
| Step 4 — Adjusted saturation flow rate | Eq. 19-8 (base × factors) plus Ch. 31 §3 permitted-left procedure and §2 pedestrian–bicycle adjustment | `SignalizedIntersection::update_lane_group_sat_flows`, `update_permitted_state`, `opposing_queue_data` in `signalized.rs`; factor functions in `exhibits.rs` (`lane_width_factor`, `heavy_vehicle_grade_factor`, `parking_factor`, `bus_blockage_factor`, `area_type_factor`, `default_lane_utilization_factor`, `protected_left_turn_factor`, `protected_right_turn_factor`); permitted-left building blocks in `signalized.rs` (`permitted_left_saturation_flow` Eq. 31-100, `el1_permitted_left` Eq. 31-101, `el2_permitted_left` Eqs. 31-102/31-103, `permitted_green_times` Exhibit 31-12 with Eqs. 31-94/31-95, `time_before_first_left_blocks` Eqs. 31-96..31-99, `shared_left_lane_saturation_flow` Eq. 31-122, `shared_left_lane_saturation_flow_modified` Eqs. 31-59/31-60); pedestrian–bicycle factors `ped_bike_factor_right` (Eqs. 31-74..31-83) and `ped_factor_left_two_way` (Eqs. 31-85..31-88) | Base saturation flow `base_saturation_flow` (pc/h/ln; default 1,900 via `BASE_SATURATION_FLOW_METRO`), lane width (ft), heavy-vehicle % and grade (%), parking/bus rates (maneuvers or buses/h), pedestrian/bicycle flow (p/h, bicycles/h), opposing approach's through/right flow and phase timing | Per-lane-group `sat_flow`/`sat_flow_permitted` (veh/h/ln); working state `s_left_prot`/`s_left_perm`/`s_thru`/`s_shared_lt`/`s_shared_rt`/`s_right` (veh/h/ln); `g_p`/`g_u`/`g_f` (s) |
| Step 5 — Proportion arriving during green | Eq. 19-15: P = R_p (g/C) | `SignalizedIntersection::step_5_proportion_arriving_on_green` in `signalized.rs` | Platoon ratio `platoon_ratio_left`/`platoon_ratio_through` (unitless, from Exhibit 19-13 arrival type via `exhibits::platoon_ratio_for_arrival_type`), lane-group effective green (s), cycle length (s) | `LaneGroup.proportion_on_green` (decimal, capped at 1.0) |
| Step 6 — Signal phase duration | Input for pretimed/coordinated (HCM Step 6 text); Ch. 31 §2 pretimed design Eqs. 31-67..31-69 provided as free functions | `PhaseTiming` struct methods `effective_green_s` (Eq. 19-3), `lost_time_s` (Eq. 19-1), `change_period_s`, `available_effective_green_s` (Eq. 19-25) in `signalized.rs`; design helpers `critical_vc_ratio_eq` (Eq. 19-30 / 31-67), `cycle_length_for_target_xc` (Eq. 31-68), `pretimed_effective_green` (Eq. 31-69) | Phase `duration_s`/`yellow_s`/`red_clearance_s`/`max_green_s`/`passage_time_s` (s); `START_UP_LOST_TIME` = 2.0 s and `EXTENSION_OF_EFFECTIVE_GREEN` = 2.0 s constants (Eq. 19-1 variable list, `exhibits.rs`) | `effective_green_s` (s) per phase used by all downstream steps |
| Step 7 — Capacity and v/c ratio | Eq. 19-16 (c = Nsg/C), Eq. 19-17 (X = v/c), Eq. 19-30 (X_c); protected-permitted/permitted left-turn capacity forms Eqs. 31-117 through 31-127 | `SignalizedIntersection::step_7_capacity_and_vc` in `signalized.rs` (per-`LaneGroupKind`/`LeftTurnMode` match arm selects the Eq. 31-119/31-120 permitted-exclusive form, the Eq. 31-124/31-125 protected-permitted-exclusive form, the Eq. 31-121/31-126 shared-lane forms, or the generic Eq. 19-16 form) | Lane group `sat_flow`/`sat_flow_permitted` (veh/h/ln), `effective_green_s` (s), `g_p`/`g_u` (s), sneakers-per-cycle `sneakers_per_cycle` (veh, default 2.0 via `SNEAKERS_PER_CYCLE`) | `LaneGroup.capacity` (veh/h), `available_capacity` (veh/h, Eq. 19-24/19-25 family), `vc_ratio` (unitless) |
| Step 8 — Delay | Eqs. 19-18 through 19-27 (uniform/incremental/control delay) via `hcm::common::delay`; Eqs. 19-44..19-49 (initial queue delay d3); Ch. 19 §4 incremental queue accumulation polygon (Eqs. 19-32..19-36, equivalently 31-92/31-115/31-116) for permitted and protected-permitted left-turn lane groups | `SignalizedIntersection::step_8_delay` in `signalized.rs`, dispatching to either the closed-form path (`common::delay::progression_factor` Eq. 19-20, `uniform_delay` Eq. 19-19) or the QAP path (`build_left_turn_qap` building `QapInterval`s per Exhibits 31-13..31-17, `qap_evaluate` Eqs. 19-34..19-36); `common::delay::incremental_delay_factor_min`/`incremental_delay_factor_actuated` (Eqs. 19-22/19-23), `incremental_delay_signalized` (Eq. 19-26), `initial_queue_delay` (Eqs. 19-44..19-49), `control_delay_signalized` (Eq. 19-18) | v/c ratio X, capacity c (veh/h), proportion on green P, cycle C and green g (s), passage time/max green for actuated k (s), initial queue Q_b (veh, via `initial_queue_for_group`), upstream filtering I (`upstream_filtering_i`, unitless, 1.0 isolated per `I_ISOLATED`), analysis period T (h, `analysis_period_h`) | `LaneGroup.uniform_delay_s`/`incremental_delay_s`/`initial_queue_delay_s`/`control_delay_s` (s/veh); `k_factor` (unitless, 0.50 pretimed via `K_PRETIMED` or Eq. 19-22/19-23 actuated form) |
| Step 9 — LOS | Exhibit 19-8 thresholds; Eq. 19-28 (approach delay), Eq. 19-29 (intersection delay) | `SignalizedIntersection::step_9_los` in `signalized.rs`; `common::los_tables::los_signalized_intersection` (A ≤10, B ≤20, C ≤35, D ≤55, E ≤80, else F s/veh; forced F when X > 1); `common::delay::aggregate_control_delay` (flow-weighted mean) | Lane-group `control_delay_s` (s/veh) and `vc_ratio`; per-lane-group `(delay, flow_rate)` pairs | `LaneGroup.los`; `ApproachResult.control_delay_s`/`los`; `self.intersection_delay_s`/`intersection_los` |
| Step 10 — Back of queue / queue storage ratio | Ch. 31 §4, Eqs. 31-130 through 31-156 | `SignalizedIntersection::step_10_queue_storage` in `signalized.rs`; `accel_decel_delay` (Eqs. 31-131/31-132), `first_term_back_of_queue` (Eqs. 31-133..31-141, basic arrival–departure polygon), `second_term_back_of_queue` (Eq. 31-142), `third_term_back_of_queue` (Eqs. 31-143..31-148), `percentile_back_of_queue` (Eqs. 31-150..31-153), `average_vehicle_spacing` (Eq. 31-155), `queue_storage_ratio_eq` (Eqs. 31-154/31-156) | Lane demand v_ln (veh/h/ln), capacity c_ln (veh/h/ln), sat flow s_ln (veh/h/ln), proportion on green P, effective green/cycle (s), speed limit (mi/h, Eq. 31-132), heavy-vehicle % and turn-bay storage length `storage_left_ft`/`storage_through_ft` (ft/ln) | `LaneGroup.q1_veh`/`q2_veh`/`q3_veh`/`back_of_queue_veh` (Q, veh/ln), `back_of_queue_95_veh` (Q_95%, veh/ln), `queue_storage_ratio`/`queue_storage_ratio_95` (R_Q, unitless) |
| (Not separately numbered) — Critical v/c ratio | Eq. 19-30 (X_c), Eq. 19-31 (lost time L), Ch. 19 §4 critical-path rules | `SignalizedIntersection::compute_critical_vc` in `signalized.rs`, called from `analyze` after Step 10; `critical_vc_ratio_eq` | Per-ring (NB/SB, EB/WB) critical flow ratios y_c,i and phase lost times (s) | `self.critical_vc_ratio` (X_c, unitless) |

### Notes on Step 1–4 iteration and permitted-left mechanics

Steps 1–4 are not run as four independent passes: `compute_states` seeds an even lane-flow assignment (`assign_flows_simple`), then iterates the Step 4 static factors (`f_w`, `f_hvg`, `f_a`, `f_bb`, `f_p`, `f_lu`), the permitted-left/pedestrian-bicycle state (`update_permitted_state`), the lane-group saturation flows (`update_lane_group_sat_flows`), and the Step 3 lane-flow distribution (`distribute_lane_flows`) up to 25 times, because `g_u` (unblocked permitted green) depends on the opposing approach's queue-service time `g_s`, which in turn depends on that approach's own converged lane flows and saturation flows (`opposing_queue_data`, using `queue_service_time`). The opposing-queue Case 1/Case 2 gap-acceptance selection (`opposing_right_turn_influences_gaps` on `SignalApproach`) implements Chapter 31, Section 3, Step 3. `permitted_green_times` transcribes all seven rows of Exhibit 31-12 (`LeadLead`, `LeadLag`, `LagLead`, `LagLag`, `PermLead`, `PermLag`, `PermPerm`) including the note-b/note-c start-up-lost-time and extension corrections for `LeadLead` and `LagLag`.

## Equation reference

The blocks below write out, per pipeline step, every HCM equation the code implements, with all variables, units, and code defaults, and the implementing function. Where a family of near-identical equations is large (the Exhibit 31-12 g_u rows, the Eq. 31-46..31-66 lane-flow sub-cases), the general form plus one fully worked representative case is given and the remaining rows are cited to the code.

### Step 2 — Movement group flow rate

Step 2 carries no standalone numbered equation beyond the peak-hour-factor conversion: each movement volume V (veh/h) is divided by the peak hour factor to yield the analysis flow rate v = V/PHF (veh/h; PHF defaults to 1.0 when `peak_hour_factor` is `None`), and the right-turn movement additionally subtracts the right-turn-on-red volume, `v_rt = max(V_r/PHF − v_RTOR, 0)`, per the Chapter 19 Step 2 text (RTOR vehicles are removed from the right-turn demand because they do not consume green). Implemented in: `src/hcm/signalized/signalized.rs::SignalApproach::flow_rates` and the Step-2 block of `src/hcm/signalized/signalized.rs::compute_states`. The RTOR estimate itself (when no field measurement is supplied) is milestone 2, `chapter19-actuated.md` Part 3.

### Step 3 — Lane group flow rate (Ch. 31 §2 lane-flow equalization, Eqs. 31-46..31-66)

The governing principle is that drivers distribute themselves across the through-serving lanes of an approach so that every lane carries the same flow ratio:

```
Equation 31-46 (equalization principle):  v_i ÷ s_i = (Σᵢ v_i) ÷ (Σᵢ s_i), for each of the N_th through-serving lanes i     [unitless]
  v_i  = demand flow rate in lane i                                    [veh/h/ln]
  s_i  = saturation flow rate in lane i                                [veh/h/ln]
  N_th = number of through-serving lanes (shared or exclusive)         [ln]
Implemented in: src/hcm/signalized/signalized.rs::distribute_lane_flows (iterative equalization); src/hcm/signalized/signalized.rs::through_serving_lanes (N_th)
```

The procedure's fully worked representative sub-equations are the Step F/G/H closing trio — the approach flow ratio and the revised turn-lane flows it implies:

```
Equation 31-64:  y* = (v_l·N_l + v_sl·N_sl + v_t·N_t + v_sr·N_sr + v_r·N_r + v_lr·N_lr) ÷ (s_l·N_l + s_sl·N_sl + s_t·N_t + s_sr·N_sr + s_r·N_r + s_lr·N_lr)     [unitless]
Equation 31-65:  v_l = s_l × y*     [veh/h/ln]
Equation 31-66:  v_sl,lt = max(v_lt − v_l, 0)     [veh/h/ln]
  y*      = approach flow ratio
  v_x, s_x, N_x = demand flow rate (veh/h/ln), saturation flow rate (veh/h/ln), and lane count (ln) for lane group x ∈ {l exclusive left, sl shared left+through, t exclusive through, sr shared right+through, r exclusive right, lr shared left+right}
  v_l     = revised exclusive-left per-lane flow rate                  [veh/h/ln]
  v_sl,lt = left-turn flow remaining in the shared lane                [veh/h/ln]
Implemented in: src/hcm/signalized/signalized.rs::distribute_lane_flows
```

The upstream sub-equations of the same procedure are transcribed in the same function and verified term-for-term against the book: the modified through-car equivalents Eq. 31-47 (E_L,m = (E_L − 1)·P_lc + 1), Eqs. 31-48/31-49 (E_L1,m/E_L2,m = (E_L1 or E_L2 / f_Lpb − 1)·P_lc + 1), the lane-change probability Eq. 31-50 (P_lc = max(1 − (2·v_app/s_lc − 1)², 0)) with Eq. 31-51 (v_app = (v_lt + v_th + v_rt)/(N_sl + N_t + N_sr)), Eq. 31-53 (E_R,m = (E_R/f_Rpb − 1)·P_lc + 1), the per-lane exclusive flows Eqs. 31-55/31-56, the shared-lane turn proportions Eqs. 31-57/31-58 (P_L, P_R), the shared-lane saturation flows Eqs. 31-59/31-60 (written out under Step 4 below), Eq. 31-61 (s_sr = s_th/(1 + P_R·(E_R,m − 1))), and Eq. 31-62 (the 0.91 through-lane factor when a permitted shared left is present). The remaining sub-cases (the shared left+right lane group Eq. 31-63 and the per-geometry variants) follow the same pattern; see `distribute_lane_flows` in `src/hcm/signalized/signalized.rs`. Convergence is iterated jointly with Step 4 as described in the Notes above.

### Step 4 — Adjusted saturation flow rate (Eq. 19-8 and the Ch. 31 factor procedures)

```
Equation 19-8:  s = s_o × f_w × f_HVg × f_p × f_bb × f_a × f_LU × f_LT × f_RT × f_Lpb × f_Rpb × f_wz × f_ms × f_sp     [veh/h/ln]
  s     = adjusted saturation flow rate                                 [veh/h/ln]
  s_o   = base saturation flow rate: 1,900 (metro, BASE_SATURATION_FLOW_METRO) or 1,750 (non-metro, BASE_SATURATION_FLOW_NON_METRO)   [pc/h/ln]
  f_w   = lane width adjustment factor
  f_HVg = heavy-vehicle and grade adjustment factor
  f_p   = parking adjustment factor
  f_bb  = bus blockage adjustment factor
  f_a   = area type adjustment factor
  f_LU  = lane utilization adjustment factor
  f_LT, f_RT   = left-/right-turn adjustment factors (protected)
  f_Lpb, f_Rpb = pedestrian–bicycle adjustment factors for left/right turns
  f_wz, f_ms, f_sp = work zone / downstream lane blockage / sustained spillback factors: 1.0 (not wired into the product; see note)
Implemented in: src/hcm/signalized/signalized.rs::update_lane_group_sat_flows and ::compute_states (the Step-4 static-factor block)
```

The code builds two through-lane saturation flows per approach, each a subset of the thirteen-factor product: the exclusive through lane `s_th_excl = s_o·f_w·f_HVg·f_a·f_bb·f_LU` and the curb (parking-adjacent) lane `s_th_curb = s_o·f_w·f_HVg·f_a·f_bb·f_p`. The turn factors f_LT/f_RT and the pedestrian–bicycle factors f_Lpb/f_Rpb are applied only to the turn-lane-group saturation flows. Scope notes (deliberate omissions, not book disagreements): f_wz exists as a standalone, currently unused free function (`exhibits.rs::work_zone_factor`, Eqs. 31-89..31-91, verified against the book), and f_ms/f_sp are not implemented anywhere — every product that the book writes with `f_ms·f_sp` is implemented with those two factors at their 1.0 defaults. Likewise Eq. 31-84 (the one-way-street pedestrian factor for left turns) is not separately implemented; only the two-way-street procedure (Eqs. 31-85..31-88) exists.

The static factors, as transcribed into `src/hcm/signalized/exhibits.rs` (structure and code constants; the formulas below are the code's own arithmetic, verified against the book):

```
Exhibit 19-20 (lane width):  f_w = 0.96 (width < 10.0 ft), 1.00 (10.0–12.9 ft), 1.04 (> 12.9 ft)
Implemented in: src/hcm/signalized/exhibits.rs::lane_width_factor
```

```
Equations 19-9/19-10 (heavy vehicles and grade):
  downhill (P_g < 0):        f_HVg = (100 − 0.79·P_HV − 2.07·P_g) ÷ 100
  level or uphill (P_g ≥ 0): f_HVg = (100 − 0.78·P_HV − 0.31·P_g²) ÷ 100
  P_HV = percentage heavy vehicles                                      [%]
  P_g  = approach grade                                                 [%]
Implemented in: src/hcm/signalized/exhibits.rs::heavy_vehicle_grade_factor
```

```
Equation 19-11 (parking):  f_p = (N − 0.1 − 18·N_m/3,600) ÷ N, floored at 0.050     [unitless]
  N   = lanes in the lane group                                         [ln]
  N_m = parking maneuver rate adjacent to the group, capped at 180      [maneuvers/h]
Implemented in: src/hcm/signalized/exhibits.rs::parking_factor
```

```
Equation 19-12 (bus blockage):  f_bb = (N − 14.4·N_b/3,600) ÷ N, floored at 0.050     [unitless]
  N_b = bus stopping rate on the approach, capped at 250                [buses/h]
Implemented in: src/hcm/signalized/exhibits.rs::bus_blockage_factor
```

```
Area type:  f_a = 0.90 (CBD), 1.00 (otherwise)
Implemented in: src/hcm/signalized/exhibits.rs::area_type_factor
```

```
Equation 19-7 (lane utilization, measured):  f_LU = v_g ÷ (N_e × v_g1)     [unitless]
  v_g  = demand flow rate for the movement group                        [veh/h]
  N_e  = number of exclusive lanes in the movement group                [ln]
  v_g1 = demand flow rate in the highest-flow exclusive lane            [veh/h/ln]
Implemented in: src/hcm/signalized/exhibits.rs::lane_utilization_factor
(defaults per Exhibit 19-15 via exhibits.rs::default_lane_utilization_factor — through 1/2/3+ lanes: 1.000/0.952/0.908; left 1/2+: 1.000/0.971; right 1/2+: 1.000/0.885; lane counts beyond the table take the smallest tabulated factor per Exhibit 19-15 note a)
```

```
Equations 19-13/19-14 (protected turn factors):  f_RT = 1/E_R,  f_LT = 1/E_L     [unitless]
  E_R = equivalent through-car count, protected right turn: 1.18 (E_R_PROTECTED_RIGHT)
  E_L = equivalent through-car count, protected left turn: 1.05 (E_L_PROTECTED_LEFT)
Implemented in: src/hcm/signalized/exhibits.rs::protected_right_turn_factor, ::protected_left_turn_factor
```

The permitted-left procedure (Ch. 31 §3) builds the permitted saturation flow from gap acceptance against the opposing stream:

```
Equation 31-100:  s_p = v_o·e^(−v_o·t_cg/3,600) ÷ (1 − e^(−v_o·t_fh/3,600))     [veh/h/ln]
  s_p  = saturation flow rate of a permitted left-turn movement
  v_o  = opposing demand flow rate (0.1 veh/h substituted when 0)       [veh/h]
  t_cg = critical headway: 4.5 s (CRITICAL_HEADWAY_PERMITTED_LEFT)
  t_fh = follow-up headway: 2.5 s (FOLLOW_UP_HEADWAY_PERMITTED_LEFT)
Implemented in: src/hcm/signalized/signalized.rs::permitted_left_saturation_flow
```

```
Equation 31-101:  E_L1 = s_o ÷ s_p     [unitless]
  E_L1 = equivalent through-car count for a permitted left-turning vehicle (filtering case)
Implemented in: src/hcm/signalized/signalized.rs::el1_permitted_left
```

```
Equation 31-102:  E_L2 = max((1 − (1 − P_lto)^n_q) ÷ P_lto, E_L)     [unitless]
Equation 31-103:  n_q = max(0.278 × (g_p − g_u − g_f), 0)             [veh]
  E_L2  = equivalent through-car count, permitted left opposed by a single-lane queue
  P_lto = proportion of left-turning vehicles in the opposing stream   [decimal]
  n_q   = maximum opposing vehicles that could arrive between g_f and g_u
  E_L   = 1.05 (the floor)
Implemented in: src/hcm/signalized/signalized.rs::el2_permitted_left
```

The permitted green windows come from Exhibit 31-12, whose rows specialize Eqs. 31-94/31-95 by phase sequence:

```
Equation 31-94:  g_p = max(G_p − l₁,p + e_p, 0)     [s]
Equation 31-95:  g_u = min(G_u + e_p, g_p)          [s]
  g_p  = effective green time for permitted left-turn operation         [s]
  G_p  = displayed green interval corresponding to g_p                  [s]
  l₁,p = permitted start-up lost time (per Exhibit 31-12 row; base 2.0 s)
  e_p  = permitted extension of effective green (per Exhibit 31-12 row; base 2.0 s)
  g_u  = permitted green not blocked by the opposing queue; G_u = its displayed interval   [s]
Implemented in: src/hcm/signalized/signalized.rs::permitted_green_times
```

The fully worked representative row is the permitted-permitted sequence (`PermPerm`, the validated Example Problem 1 case), whose displayed unblocked interval is `G_U = D_p,opp − Y_own − R_c,own − G_q`, i.e. the opposing approach's through-phase duration less the subject's own change period less the worst-case opposing-queue clearance G_q (computed as g_s + l₁, or the shared-lane equivalent per Exhibit 31-12 note a). A subscript caution when reading the exhibit against the code: Exhibit 31-12's "D_p2/Y_2/R_c2" notation refers to the *opposing* approach's through phase and "D_p6/Y_6/R_c6" to the *subject's own* through phase (confirmed by the HCM's own footnote text), which is counter-intuitive from the phase numbering alone. All seven rows (`LeadLead`, `LeadLag`, `LagLead`, `LagLag`, `PermLead`, `PermLag`, `PermPerm`), including the note-b/note-c l₁,p and e_p corrections for `LeadLead` and `LagLag`, were verified term-by-term against the book and match; the remaining six rows follow the same pattern — see `permitted_green_times` in `src/hcm/signalized/signalized.rs`.

```
Equation 31-99:  LTC = v_lt × C ÷ 3,600     [veh/cycle]
Equation 31-97 (one-lane approach):   g_f = min(max(G_p·e^(−0.860·LTC^0.629) − l₁,p, 0), g_f,max)     [s]
Equation 31-98 (multilane approach):  g_f = min(max(G_p·e^(−0.882·LTC^0.717) − l₁,p, 0), g_f,max)     [s]
Equation 31-96:  g_f,max = max([(1−P_L)/(0.5·P_L)] × (1 − (1−P_L)^(0.5·g_p)) − l₁,p, 0)     [s]
  g_f  = time before the first left-turning vehicle arrives and blocks the shared lane     [s]
  LTC  = left-turn flow rate per cycle                                  [veh/cycle]
  v_lt = left-turn demand flow rate                                     [veh/h]
  P_L  = proportion of left-turning vehicles in the shared lane         [decimal]
Implemented in: src/hcm/signalized/signalized.rs::time_before_first_left_blocks
```

```
Equation 31-122:  s_sl = (s_th/g_p) × [ g_f + g_diff/(1 + P_L·(E_L2/f_Lpb − 1)) + min(g_p − g_f, g_u)/(1 + P_L·(E_L1/f_Lpb − 1)) ]     [veh/h/ln]
  with Equation 31-107:  g_diff = max(g_p − g_u − g_f, 0)     [s]
  s_sl  = saturation flow rate of the shared left-turn/through lane     [veh/h/ln]
  s_th  = through saturation flow rate (curb lane)                      [veh/h/ln]
  f_Lpb = pedestrian adjustment factor for left turns (Eqs. 31-85..31-88)
Implemented in: src/hcm/signalized/signalized.rs::shared_left_lane_saturation_flow
```

```
Equation 31-59:  s_sl = (s_th/g_p) × [ g_f + g_diff/(1 + P_L·(E_L2,m − 1)) + min(g_p − g_f, g_u)/(1 + P_L·(E_L1,m − 1)) + 3,600·n_s*·f_ms·f_sp/s_th ]     [veh/h/ln]
Equation 31-60:  n_s* = P_L/(1 − P_L) × (1 − P_L^n_s)  if P_L < 0.999;  n_s* = n_s·P_L  if P_L ≥ 0.999     [veh]
  n_s* = expected sneakers per cycle in the shared left-turn lane
  n_s  = sneakers per cycle: 2.0 (SNEAKERS_PER_CYCLE)                   [veh]
  E_L1,m, E_L2,m = modified through-car equivalents (Eqs. 31-48/31-49)
  f_ms, f_sp = 1.0 (not implemented; see the Eq. 19-8 note)
Implemented in: src/hcm/signalized/signalized.rs::shared_left_lane_saturation_flow_modified
```

The pedestrian–bicycle adjustment factors (Ch. 31 §2 Steps E–G):

```
Equation 31-74:  v_pedg = min(v_ped × C ÷ g_ped, 5,000)     [p/h]
Equation 31-75 (v_pedg ≤ 1,000):  OCC_pedg = v_pedg ÷ 2,000
Equation 31-76 (v_pedg > 1,000):  OCC_pedg = min(0.4 + v_pedg ÷ 10,000, 0.90)
Equation 31-77:  v_bicg = min(v_bic × C ÷ g, 1,900)          [bicycles/h]
Equation 31-78:  OCC_bicg = 0.02 + v_bicg ÷ 2,700
Equation 31-79 (no bicycles):    OCC_r = (g_ped/g) × OCC_pedg
Equation 31-80 (with bicycles):  OCC_r = (g_ped/g)·OCC_pedg + OCC_bicg − (g_ped/g)·OCC_pedg·OCC_bicg
Equation 31-81 (receiving lanes = turn lanes):  A_pbT = 1 − OCC_r
Equation 31-82 (receiving lanes > turn lanes):  A_pbT = 1 − 0.6 × OCC_r
Equation 31-83:  f_Rpb = A_pbT
  v_ped, v_bic = pedestrian / bicycle flow rates                        [p/h, bicycles/h]
  g_ped = pedestrian service time                                       [s]
  OCC_pedg, OCC_bicg, OCC_r = pedestrian / bicycle / relevant-conflict-zone occupancies   [decimal]
Implemented in: src/hcm/signalized/signalized.rs::ped_bike_factor_right
```

```
Equation 31-85 (g_q < g_ped):  OCC_pedu = OCC_pedg × (1 − 0.5·g_q/g_ped)
Equation 31-86 (g_q ≥ g_ped):  OCC_pedu = 0.0
Equation 31-87:  OCC_r = [(g_ped − g_q) ÷ (g_p − g_q)] × OCC_pedu × e^(−5.00·v_o/3,600)
Equation 31-88:  f_Lpb = A_pbT   (via Eqs. 31-81/31-82 with g_p in place of g)
  g_q = opposing-queue service time = g_p − g_u                         [s]
  v_o = opposing demand flow rate                                       [veh/h]
Implemented in: src/hcm/signalized/signalized.rs::ped_factor_left_two_way
```

The opposing-queue clearance used to derive g_u is computed by `queue_service_time` (`g_s = q_r·r/(s/3,600 − q_g)`, with red/green arrival rates q_r = (1−P)·q·C/r and q_g = P·q·C/g); this is the same equation as Chapter 31's Eq. 31-9 — see `chapter19-actuated.md` Part 1 for the full block (and note that the milestone-1 `queue_service_time` here carries the correct book form).

### Step 5 — Proportion arriving during green

```
Equation 19-15:  P = min(R_p × g ÷ C, 1.0)     [decimal]
  P   = proportion of vehicles arriving during the green indication
  R_p = platoon ratio (Exhibit 19-13, by arrival type): 0.33 (AT1) / 0.67 (AT2) / 1.00 (AT3, random) / 1.33 (AT4) / 1.67 (AT5) / 2.00 (AT6)
  g   = lane-group effective green time                                 [s]
  C   = cycle length                                                    [s]
Implemented in: src/hcm/signalized/signalized.rs::step_5_proportion_arriving_on_green, using src/hcm/signalized/exhibits.rs::platoon_ratio_for_arrival_type
```

The cap at 1.0 is a physical clamp added by the code (the book states the bare product).

### Step 6 — Signal phase duration (input) and the timing primitives

```
Equation 19-1:  l_t = l₁ + l₂ = l₁ + Y + R_c − e     [s]
  l_t = phase lost time                                                 [s]
  l₁  = start-up lost time: 2.0 s (START_UP_LOST_TIME)
  l₂  = clearance lost time = Y + R_c − e                               [s]
  Y   = yellow change interval                                          [s]
  R_c = red clearance interval                                          [s]
  e   = extension of effective green: 2.0 s (EXTENSION_OF_EFFECTIVE_GREEN)
Implemented in: src/hcm/signalized/signalized.rs::PhaseTiming::lost_time_s (constants in src/hcm/signalized/exhibits.rs)
```

```
Equation 19-3:  g = D_p − l₁ − l₂     [s]
  g   = effective green time (floored at 0 in code)                     [s]
  D_p = phase duration = G + Y + R_c                                    [s]
Implemented in: src/hcm/signalized/signalized.rs::PhaseTiming::effective_green_s
```

The change period CP = Y + R_c is not a separately numbered equation (it is the Exhibit 19-6 term definition) and is exposed as `PhaseTiming::change_period_s`.

```
Equation 19-25:  g_a = G_max + Y + R_c − l₁ − l₂  (= G_max − l₁ + e)     [s]
  g_a   = available effective green time for an actuated lane group     [s]
  G_max = maximum green setting                                         [s]
Implemented in: src/hcm/signalized/signalized.rs::PhaseTiming::available_effective_green_s (falls back to effective_green_s when no max_green_s is set)
```

The pretimed-design free functions implement the Chapter 31, Section 2 timing-plan equations (Eq. 31-67 is the identical formula to Eq. 19-30, quoted under the critical-v/c heading below):

```
Equation 31-68:  C = L × X_c ÷ (X_c − Σᵢ y_c,i)     [s]
  C     = cycle length for a target critical v/c ratio                  [s]
  L     = cycle lost time (Eq. 19-31)                                   [s]
  X_c   = target critical intersection volume-to-capacity ratio
  y_c,i = critical flow ratio for critical phase i = v_i/(N_i·s_i)
Implemented in: src/hcm/signalized/signalized.rs::cycle_length_for_target_xc (returns None when X_c ≤ Σy_c,i, the undefined-denominator domain)
```

```
Equation 31-69:  g_i = y_c,i × (C ÷ X_i)     [s]
  g_i = effective green allocated to critical phase i                   [s]
  X_i = target volume-to-capacity ratio for lane group i
Implemented in: src/hcm/signalized/signalized.rs::pretimed_effective_green
```

### Step 7 — Capacity and v/c ratio

```
Equation 19-16:  c = N × s × g ÷ C     [veh/h]
  c = lane-group capacity                                               [veh/h]
  N = number of lanes in the lane group                                 [ln]
  s = adjusted saturation flow rate                                     [veh/h/ln]
  g = effective green time                                              [s]
  C = cycle length                                                      [s]
Implemented in: src/hcm/signalized/signalized.rs::step_7_capacity_and_vc (generic match arm; available capacity c_a = N·s·g_a/C with g_a from Eq. 19-25, the Eq. 19-24 family)
```

```
Equation 19-17:  X = v ÷ c     [unitless]
  X = volume-to-capacity ratio (∞ when c = 0 in code)
  v = demand flow rate                                                  [veh/h]
Implemented in: src/hcm/signalized/signalized.rs::step_7_capacity_and_vc
```

The book bars Eq. 19-16 for shared-lane and permitted-operation lane groups; the code routes those to the Chapter 31 forms (f_ms = f_sp = 1.0 throughout, per the Step 4 scope note):

```
Equation 31-119:  c_l,e = (g_u·s_l + 3,600·n_s·f_ms·f_sp) ÷ C × N_l     [veh/h]
Equation 31-120:  c_a,l,e = c_l,e + max(G_max − g_p, 0) × s_l ÷ C × N_l     [veh/h]
  c_l,e   = capacity, exclusive-lane lane group with permitted left-turn operation
  c_a,l,e = available capacity of that lane group
  g_u   = unblocked permitted green                                     [s]
  s_l   = permitted left-turn saturation flow (s_left_perm)             [veh/h/ln]
  n_s   = sneakers per cycle: 2.0 (SNEAKERS_PER_CYCLE)                  [veh]
  N_l   = lanes in the exclusive left-turn group                        [ln]
  G_max = maximum green of the phase serving the permitted period       [s]
  g_p   = effective permitted green (the code's resolution of the book's un-subscripted g)   [s]
Implemented in: src/hcm/signalized/signalized.rs::step_7_capacity_and_vc ((ExclusiveLeft, Permitted) arm)
```

```
Equation 31-124:  c_l,e,pp = (g_l·s_lt + g_u·s_l + 3,600·n_s·f_ms·f_sp) ÷ C × N_l     [veh/h]
Equation 31-125:  c_a,l,e,pp = (G_max·s_lt + g_u·s_l + 3,600·n_s·f_ms·f_sp) ÷ C × N_l     [veh/h]
  c_l,e,pp = capacity, exclusive-lane lane group with protected-permitted left-turn operation
  g_l   = effective green of the protected left-turn phase              [s]
  s_lt  = protected left-turn saturation flow (s_left_prot)             [veh/h/ln]
  G_max = maximum green of the left-turn phase (falls back to g_l when unset)   [s]
Implemented in: src/hcm/signalized/signalized.rs::step_7_capacity_and_vc ((ExclusiveLeft, ProtectedPermitted) arm; c_a floored at c)
```

```
Equation 31-121:  c_sl = (g_p·s_sl + 3,600·(1 + P_L)·f_ms·f_sp) ÷ C     [veh/h]
Equation 31-126:  c_sl,pp = g_l·s_sl4 ÷ C + (g_p·s_sl + 3,600·(1 + P_L)·f_ms·f_sp) ÷ C     [veh/h]
  c_sl    = capacity, shared-lane lane group with permitted left-turn operation
  c_sl,pp = capacity, shared-lane lane group with protected-permitted operation
  s_sl    = shared-lane saturation flow (Eq. 31-122)                    [veh/h/ln]
  s_sl4   = shared-lane saturation flow during the protected period (Eq. 31-113) = s_th ÷ (1 + P_L·(E_L − 1))   [veh/h/ln]
  P_L     = proportion of left turns in the shared lane                 [decimal]
Implemented in: src/hcm/signalized/signalized.rs::step_7_capacity_and_vc ((SharedLeftThrough, Permitted|ProtectedPermitted) arm)
```

**CORRECTED:** the shared-lane available capacity of Equations 31-123 and 31-127 is now implemented. The book defines, for a shared-lane lane group with permitted operation, `c_a,sl = c_sl + (G_max − g_p)·s_sl3/C` (Eq. 31-123, verified against `246_Ch31_03.xhtml`), and for protected-permitted operation `c_a,sl,pp = c_sl,pp + (G_max − g_p)·s_sl3/C` (Eq. 31-127). The `SharedLeftThrough` arm of `step_7_capacity_and_vc` computed the capacity correctly per Eqs. 31-121/31-126 but returned available capacity equal to capacity, omitting the `(G_max − g_p)·s_sl3/C` extension. The arm now adds that extension, computing `s_sl3` from Eq. 31-109 (`s_th_curb / (1 + P_L(E_L1/f_Lpb − 1))`, using the `el1`/`f_lpb`/`s_th_curb` already carried in `ApproachState`) and `G_max` from the through phase's max green, mirroring the exclusive-lane arms above it. This affects the actuated incremental-delay factor k (which consumes `available_capacity` in `step_8_delay`) for shared left-turn lane groups under actuated control; pretimed analysis is bit-identical (k = K_PRETIMED there regardless of available capacity), confirmed by the full suite passing unchanged. Fixed in commit for `fix/hcm-equation-sweep` (Eqs 31-123/31-127 shared-left available capacity).

### Step 8 — Delay

```
Equation 19-18:  d = d₁ + d₂ + d₃     [s/veh]
  d  = control delay
  d₁ = uniform delay (Eq. 19-19, PF included)                           [s/veh]
  d₂ = incremental delay (Eq. 19-26)                                    [s/veh]
  d₃ = initial queue delay (Eqs. 19-44..19-49)                          [s/veh]
Implemented in: src/hcm/common/delay.rs::control_delay_signalized
```

```
Equation 19-19:  d₁ = PF × [0.5·C·(1 − g/C)²] ÷ [1 − min(1, X)·g/C]     [s/veh]
  PF = progression adjustment factor (Eq. 19-20); 1.0 for random arrivals
  C  = cycle length                                                     [s]
  g  = effective green time                                             [s]
  X  = lane-group volume-to-capacity ratio
Implemented in: src/hcm/common/delay.rs::uniform_delay (returns 0 when g/C ≥ 1, the guarded analytic limit)
```

```
Equation 19-21:  y = min(1, X) × g/C     [unitless]
Equation 19-20:  PF = [(1 − P)/(1 − g/C)] × [(1 − y)/(1 − min(1,X)·P)] × [1 + y·(1 − P·C/g)/(1 − g/C)]     [unitless]
  P = proportion of vehicles arriving during green (Eq. 19-15)          [decimal]
  y = flow ratio (Eq. 19-21)
Implemented in: src/hcm/common/delay.rs::progression_factor (Eq. 19-21 as delay.rs::flow_ratio; returns 1.0 when g/C ≥ 1, a guard against the 0/0 singularity)
```

```
Equation 19-23:  k_min = −0.375 + 0.354·PT − 0.0910·PT² + 0.00889·PT³, floored at 0.04     [unitless]
Equation 19-22:  k = (1 − 2·k_min) × (v/c_a − 0.5) + k_min, clamped to [k_min, 0.50]     [unitless]
  k     = incremental delay factor; K_PRETIMED = 0.50 for pretimed/coordinated/recall-to-max phases
  PT    = passage time setting                                          [s]
  v/c_a = demand over available capacity (Eq. 19-24 family)
Implemented in: src/hcm/common/delay.rs::incremental_delay_factor_min, ::incremental_delay_factor_actuated
```

```
Equation 19-26:  d₂ = 900 × T × [(X_A − 1) + √((X_A − 1)² + 8·k·I·X_A ÷ (c_A·T))]     [s/veh]
  T   = analysis period duration: 0.25 default (analysis_period_h)      [h]
  X_A = average volume-to-capacity ratio v/c_A (Eq. 19-27; milestone 1 uses c_A = c)
  c_A = average lane-group capacity                                     [veh/h]
  I   = upstream filtering factor: 1.0 isolated (I_ISOLATED), floor 0.090 (I_MIN)
Implemented in: src/hcm/common/delay.rs::incremental_delay_signalized
```

```
Equations 19-44..19-49 (initial queue delay d₃):
  Equation 19-45:  Q_e = Q_b + t_A × (v − c_A)     [veh]
  if v ≥ c_A:  Equation 19-46: Q_eo = T × (v − c_A);  Equation 19-47: t_A = T
  if v < c_A:  Equation 19-48: Q_eo = 0.0;            Equation 19-49: t_A = min(Q_b/(c_A − v), T)
  Equation 19-44:  d₃ = 3,600/(v·T) × [ t_A·(Q_b + Q_e − Q_eo)/2 + (Q_e² − Q_eo²)/(2·c_A) − Q_b²/(2·c_A) ]     [s/veh]
  Q_b  = initial queue at the start of the period (via initial_queue_for_group)   [veh]
  Q_e  = queue at the end of the period                                 [veh]
  Q_eo = queue at the end of the period, zero-initial-queue baseline    [veh]
  t_A  = duration of unmet demand within the period                     [h]
  (d₃ = 0 whenever Q_b ≤ 0, v ≤ 0, or T ≤ 0, matching the book's no-initial-queue case)
Implemented in: src/hcm/common/delay.rs::initial_queue_delay (Eq. 19-45's Q_e also exposed as delay.rs::queue_end_of_period for multi-period hand-off)
```

For permitted and protected-permitted left-turn lane groups, d₁ comes from the incremental queue accumulation polygon instead of the closed form. `build_left_turn_qap` constructs the interval sequence per the Exhibit 31-13..31-17 shapes (red, blocked permitted, unblocked permitted at s_l, protected period at s_lt or s_sl4, sneaker removals at period ends) and `qap_evaluate` integrates it:

```
Equations 19-32..19-36 (QAP evaluation, general form):
  per interval i with net slope w = arrival − discharge (veh/s):
  Equation 19-36:  t_t,i = min(t_d,i, Q_(i−1) ÷ |w|)  when the queue is draining (w < 0), else t_d,i     [s]
  Equation 19-35:  d₁ = [0.5 × Σᵢ (Q_(i−1) + Q_i) × t_t,i] ÷ (q_avg × C)     [s/veh]
  Q_i   = queue at the end of interval i                                [veh]
  t_d,i = interval duration                                             [s]
  q_avg = average arrival rate over the cycle                           [veh/s]
Implemented in: src/hcm/signalized/signalized.rs::qap_evaluate (steady-state polygon iterated to a fixed starting queue, trapezoid areas truncated at queue-zero crossings); intervals from src/hcm/signalized/signalized.rs::build_left_turn_qap
```

### Step 9 — LOS

```
Equation 19-28:  d_A = Σᵢ(dᵢ × vᵢ) ÷ Σᵢ vᵢ, over the lane groups i of the approach     [s/veh]
Equation 19-29:  d_I = Σᵢ(dᵢ × vᵢ) ÷ Σᵢ vᵢ, over all lane groups of the intersection    [s/veh]
  dᵢ, vᵢ = lane-group control delay (s/veh) and demand flow rate (veh/h)
Implemented in: src/hcm/common/delay.rs::aggregate_control_delay, called from src/hcm/signalized/signalized.rs::step_9_los at both levels
```

```
Exhibit 19-8 (LOS thresholds, s/veh control delay):
  A ≤ 10;  B ≤ 20;  C ≤ 35;  D ≤ 55;  E ≤ 80;  F > 80
  Forced F when v/c > 1.0 — applied at the lane-group level only; approach and intersection LOS are defined solely by control delay per the exhibit's own footnote
Implemented in: src/hcm/common/los_tables.rs::los_signalized_intersection (step_9_los passes vc_gt_1 = false for approach/intersection calls)
```

### Step 10 — Back of queue and queue storage ratio (Ch. 31 §4)

```
Equation 31-132:  S_a = 0.90 × (25.6 + 0.47 × S_pl)     [mi/h]
Equation 31-131:  d_a = [1.47 × (S_a − S_s)]² ÷ (2 × 1.47 × S_a) × (1/r_a + 1/r_d)     [s]
  d_a  = acceleration–deceleration delay per full stop                  [s]
  S_a  = average approach speed                                         [mi/h]
  S_pl = posted speed limit                                             [mi/h]
  S_s  = stop-threshold speed: 5.0 mi/h (STOP_THRESHOLD_SPEED_MPH)
  r_a  = acceleration rate: 3.5 ft/s² (QUEUE_ACCELERATION_RATE)
  r_d  = deceleration rate: 4.0 ft/s² (QUEUE_DECELERATION_RATE)
Implemented in: src/hcm/signalized/signalized.rs::accel_decel_delay
```

```
Equations 31-133..31-141 (first-term back of queue Q1, basic Exhibit 31-25 polygon):
  q_r = (1 − P) × q × C ÷ r;  q_g = P × q × C ÷ g;  q = v_ln/3,600;  r = C − g
  if d_a ≤ (1 − P)·g·X:
    Equation 31-137:  t_f = q·C·(1 − P − P·d_a/g) ÷ (s·(1 − min(1, X)·P))     [s]
    Equation 31-139:  N_f = q_r·r + q_g·(t_f − d_a)     [veh/ln]
  else:
    Equation 31-138:  t_f = q·C·(1 − P)·(r − d_a) ÷ (s·(r − min(1, X)·(1 − P)·g))     [s]
    Equation 31-140:  N_f = q_r·(r − d_a + t_f)     [veh/ln]
  Equation 31-141:  Q1 = N_f, floored at 0     [veh/ln]
  t_f = service time for fully stopped vehicles                         [s]
  q_r, q_g = arrival rates during effective red / green                 [veh/s]
  s   = adjusted saturation flow rate (per lane, veh/s in the formula)
Implemented in: src/hcm/signalized/signalized.rs::first_term_back_of_queue (through movements, protected turns, shared through+right); permitted/protected-permitted left-turn lane groups instead reuse the Step 8 QAP maximum queue in milestone 1 (see Deviations) and the ADP full-stop count in milestone 2 (chapter19-actuated.md Part 2)
```

```
Equation 31-142:  Q2 = c_A ÷ (3,600 × N) × d₂     [veh/ln]
  Q2  = second-term (incremental) back of queue
  c_A = average lane-group capacity                                     [veh/h]
  N   = lanes in the group                                              [ln]
  d₂  = incremental delay (Eq. 19-26)                                   [s/veh]
Implemented in: src/hcm/signalized/signalized.rs::second_term_back_of_queue
```

```
Equations 31-143..31-148 (third-term back of queue Q3, initial-queue contribution):
  Equation 31-144:  Q_e = Q_b + t_A × (v − c_A)     [veh]
  if v ≥ c_A:  Equation 31-145: Q_eo = T × (v − c_A);  Equation 31-146: t_A = T
  if v < c_A:  Equation 31-147: Q_eo = 0.0;            Equation 31-148: t_A = min(Q_b/(c_A − v), T)
  Equation 31-143:  Q3 = 1/(N·T) × t_A × (Q_b + Q_e − Q_eo)/2     [veh/ln]
  (same branch structure as d₃ but without the /(2·c_A) quadratic terms — Q3 is the linear queue-count analog, matching the book)
Implemented in: src/hcm/signalized/signalized.rs::third_term_back_of_queue
```

```
Equations 31-150..31-153 (percentile back of queue):
  Equation 31-150:  Q% = (Q1 + Q2) × f_B% + Q3     [veh/ln]
  if v ≥ c_A (Eqs. 31-151/31-152):  f_B% = min(1.8, 1 + z·√(I/(Q1+Q2)) + 0.60·z^0.24·(g/C)^0.33·(1 − e^(2 − 2·X_A)))
  if v < c_A (Eq. 31-153):          f_B% = min(1.8, 1 + z·√(I/(Q1+Q2)))
  z = percentile parameter: 1.04 (85th), 1.28 (90th), 1.64 (95th)
  I = upstream filtering adjustment factor
Implemented in: src/hcm/signalized/signalized.rs::percentile_back_of_queue
```

```
Equation 31-155:  L_h = L_pc × (1 − 0.01·P_HV) + 0.01 × L_HV × P_HV     [ft/veh]
  L_h  = average queue spacing
  L_pc = stored passenger-car length: 25.0 ft (STORED_PASSENGER_CAR_LENGTH_FT)
  L_HV = stored heavy-vehicle length: 45.0 ft (STORED_HEAVY_VEHICLE_LENGTH_FT)
  P_HV = percent heavy vehicles                                         [%]
Implemented in: src/hcm/signalized/signalized.rs::average_vehicle_spacing
```

```
Equations 31-154/31-156:  R_Q = L_h × Q ÷ L_a     [unitless]
  R_Q = queue storage ratio (Eq. 31-154 with the 50th-percentile Q; Eq. 31-156 with Q%)
  Q   = back-of-queue estimate                                          [veh/ln]
  L_a = available queue storage distance (storage_left_ft/storage_through_ft)   [ft/ln]
Implemented in: src/hcm/signalized/signalized.rs::queue_storage_ratio_eq
```

### Critical v/c ratio

```
Equation 19-31:  L = Σᵢ l_t,i, over the critical phases i     [s]
Equation 19-30 (= Equation 31-67):  X_c = C ÷ (C − L) × Σᵢ y_c,i     [unitless]
  X_c   = critical intersection volume-to-capacity ratio (∞ when C ≤ L in code)
  L     = cycle lost time                                               [s]
  y_c,i = critical flow ratio for phase i = v_i ÷ (N_i·s_i)
Implemented in: src/hcm/signalized/signalized.rs::critical_vc_ratio_eq (the shared formula); src/hcm/signalized/signalized.rs::compute_critical_vc (critical-path selection and the L / Σy_c,i accumulation)
```

The critical-path selection in `compute_critical_vc` treats the two direction pairs [NB, SB] and [EB, WB] as opposing barriers. For each pair it computes per approach a through flow ratio (the maximum over that approach's non-exclusive-left lane groups) and, for `Protected`/`ProtectedPermitted` lefts, splits the left-turn volume into a protected part (up to the protected phase's capacity N_l·s_lt·g_l/C) and a permitted remainder, each with its own flow ratio; a `Permitted` left contributes only a permitted flow ratio. With both approaches present it evaluates three candidate paths — Ring 1 (this approach's protected-left ratio plus the opposing through ratio), Ring 2 (the mirror), and each approach's own protected-plus-permitted left combination (one lost-time increment, the max of the two phases') — keeping the largest flow-ratio sum with its associated lost time; with one approach the path is simply that approach's through plus protected-left ratios. The winning sums and lost times from both barriers accumulate into Σy_c,i and L for Eq. 19-30.

## Deviations (cross-referenced to `docs/hcm/VERIFICATION.md`)

`docs/hcm/VERIFICATION.md` is introduced by a later branch (`feat/hcm-ch19-actuated`, commit that adds the file consolidates the project's `VERIFY-HCM` items); it is not present in the working tree at this branch's tip. Its "Chapter 19 (feat/hcm-ch19-signalized)" section, read from that later commit's history, records no `VERIFY-HCM` items for this branch's code, but does flag two interpretation notes worth the reviewer's attention: (1) the Exhibit 31-12 lag-row sub-variants (`LagLead`, `LagLag`, `PermLead`, `PermLag`) were transcribed into `permitted_green_times` but only the `PermPerm` and `LeadLead` rows were validated against a published g_u value (the `case1.json`/Example Problem 1 fixture uses `PermPerm`; `test_permitted_green_times_lead_lead` in `tests.rs` covers `LeadLead`) — the other four rows are un-exercised by any fixture in this repository as of this branch. (2) The opposing-queue clearance Gq for an opposing shared through+right lane group is computed as `g_s + l_1` inside `opposing_queue_data`, matching Exhibit 31-12 note (a); this was validated numerically against Example Problem 1 rather than against an independently published Gq value. The same VERIFICATION.md entry also records two *known engine deviations* documented directly in `tests.rs`/`chapter19_integration.rs` rather than as `VERIFY-HCM` code comments: the protected-permitted left-turn d1 (uniform delay, via the QAP path) carries roughly ±1.2 s/veh of residual error against the published Exhibit 31-81 values in the tighter cases (the `chapter19_integration.rs` `d_tol` column widens to 1.5 s/veh for every permitted/protected-permitted left-turn lane group and for the oversaturated NB lane groups, versus 0.5 s/veh for the closed-form cases); and the SB-left back-of-queue estimate uses the QAP polygon's maximum queue (`lg.q1_veh` set inside `step_8_delay`'s QAP branch, reused directly by `step_10_queue_storage` for `is_perm_left` groups) as a stand-in for the full left-turn arrival–departure-polygon (ADP) family of Exhibits 31-26 through 31-31, which is explicitly deferred to milestone 2 (`chapter19-actuated.md`). This Q1 substitution is documented inline in `step_10_queue_storage`'s doc comment as "a milestone-2 item." Additional to these previously recorded items, the Step 7 shared-lane available-capacity omission (Eqs. 31-123/31-127) found in this documentation pass has since been **fixed** (see the corrected block under Step 7 above), and the Step 4 scope notes on f_wz/f_ms/f_sp and Eq. 31-84 (deliberate omissions defaulting to 1.0 or absent) remain called out there for completeness.

## Validation

The fixtures live under `tests/ExampleCases/hcm/Signalized/` as `case1.json` and `case2.json`, both loaded via `serde_json` into `SignalizedIntersection`. `case1.json` encodes HCM Chapter 31, Section 10, Example Problem 1 (Exhibits 31-69 through 31-82) with the Exhibit 31-79 converged phase durations supplied as fixed pretimed/coordinated timing (this fixture therefore validates Steps 1–5 and 7–10 without exercising the actuated convergence loop). `case2.json` encodes the Chapter 31, Section 2 pretimed phase-duration example (Exhibit 31-7) as a full pretimed timing plan, with delay/LOS expectations hand-computed from Eq. 19-19 (PF = 1 for random arrivals) and Eq. 19-26 (pretimed k = 0.50) rather than taken directly from a published delay exhibit. Three test layers exercise these fixtures and the underlying functions: unit tests in `src/hcm/signalized/tests.rs` (per-step spot checks, e.g. `test_permitted_left_saturation_flow_eq_31_100`, `test_unblocked_permitted_green_exhibit_31_79`, `test_ped_bike_factors_exhibit_31_77`, `test_pretimed_phase_duration_exhibit_31_7`, `test_permitted_green_times_perm_perm`/`_lead_lead`, `test_qap_matches_closed_form_uniform_delay`); coefficient-table unit tests in `src/hcm/signalized/exhibits.rs` (e.g. `test_lane_width_factor_exhibit_19_20`, `test_heavy_vehicle_grade_factor`, `test_default_lane_utilization_exhibit_19_15`, `test_default_system_cycle_length_exhibit_19_17`); and the full-pipeline integration tests in `tests/chapter19_integration.rs` (`test_case1_example_problem_1_full_pipeline`, `test_case2_pretimed_exhibit_31_7_timing_plan`, `test_fixture_serde_roundtrip`), mirrored for the Python bindings in `tests/test_chapter19_integration.py`. The integration test's documented tolerances, stated verbatim in its module doc comment, are: LOS exact; intersection/approach control delay ±0.5 s/veh; lane-group control delay ±0.5 s/veh for lane groups on the Eq. 19-19 closed-form path, widened to ±1.5 s/veh for permitted/protected-permitted left-turn lane groups (QAP path) and for the oversaturated NB lane groups in `case1.json` (sensitivity of d2 to small lane-flow differences); adjusted saturation flow ±10 veh/h/ln; capacity ±6 veh/h; v/c ratio ±0.02. `case2.json`'s critical v/c ratio X_c is checked to ±0.001 against the published 0.923, and its per-approach capacity/v-c/d1/d2/delay values to ±1.0 veh/h / ±0.005 / ±0.5 s/veh (each).

## Deferred

Per the `signalized.rs` module doc comment and `mod.rs`: the actuated phase-duration estimation loop (Chapter 31, Section 2's Eqs. 31-1 through 31-45) is deferred to milestone 2 (`feat/hcm-ch19-actuated`, documented in `chapter19-actuated.md`) — on this branch, `ActuatedSignal`/`SemiActuatedSignal` control types are accepted by `step_8_delay`'s k-factor branch only insofar as the caller supplies already-converged average phase durations plus `max_green_s`/`passage_time_s`, using Eqs. 19-22 through 19-25 directly rather than running the convergence procedure. The full left-turn arrival–departure-polygon (ADP) family for percentile back-of-queue (Exhibits 31-26 through 31-31) is deferred; Step 10 currently reuses the Step 8 QAP's maximum queue as the Q1 estimate for permitted/protected-permitted left-turn lane groups. Pedestrian/bicycle LOS methodologies (separate from the saturation-flow ped/bike adjustment factors, which are implemented) are not covered by this branch.
