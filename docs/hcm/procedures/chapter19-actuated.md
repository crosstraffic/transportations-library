# HCM Chapter 19 — Milestone 2: Actuated Phase Duration, Left-Turn ADP Back of Queue, and RTOR Estimation

This document walks the milestone-2 extensions to the Chapter 19 signalized-intersection implementation on branch `feat/hcm-ch19-actuated`: the HCM 7th Edition Chapter 31, Section 2 "Actuated Phase Duration" procedure (Equations 31-1 through 31-45), the Chapter 31, Section 4 left-turn arrival–departure-polygon (ADP) first-term back of queue (Exhibits 31-26 through 31-31 with Equation 31-141), and the Chapter 31, Section 8 right-turn-on-red (RTOR) volume estimate. The new code lives in `src/hcm/signalized/actuated.rs` (equation-level pure functions plus the `estimate_fully_actuated` driver for the eight-phase dual-ring NEMA structure of Exhibit 19-2) and in additions to `src/hcm/signalized/signalized.rs` (`adp_first_term_left`, `SignalizedIntersection::estimate_actuated_timings`, `estimate_rtor_volume`, `apply_rtor_estimates`, and new optional `PhaseTiming` fields `min_green_s`, `detector_length_ft`, `recall_max`). The fixed-timing milestone-1 pipeline documented in `chapter19.md` remains the default analysis path; the actuated estimator is a separate, non-mutating entry point that consumes the analyzed operating point (per the `mod.rs` module doc on this branch). Milestone-1 code already present in `signalized.rs` and `src/hcm/common/delay.rs` supplies the k-factor and available-capacity plumbing (Eqs. 19-22 through 19-25) that lets converged actuated durations feed the incremental delay d2; that hand-off is documented in Part 4 below. Deviations carry `// VERIFY-HCM` comments in code and are consolidated in `docs/hcm/VERIFICATION.md`, section "Chapter 19 milestone 2 (feat/hcm-ch19-actuated)". Every equation written out below has been cross-checked against both the Rust function body and the HCM 7th Edition EPUB MathML source (`resources/epub/OEBPS/245_Ch31_02.xhtml` for the Section 2 procedure, `247_Ch31_04.xhtml` for the Section 4 ADP material, `251_Ch31_08.xhtml` for the Section 8 RTOR guidance); newly found code-vs-book disagreements are flagged inline as **DISCREPANCY** blocks.

## Part 1 — Average phase duration convergence loop (Eqs. 31-1 through 31-45)

The procedure estimates each phase's average duration from controller settings (max/min green, passage time, walk + pedestrian clear, recall), detector design, and lane-group demand. It is iterative because queue service time, green extension, and the cycle length all depend on the green durations being estimated. The driver is `estimate_fully_actuated(phases, base_sat_flow, simultaneous_gap_out, platoon_ratio)` in `actuated.rs`, which loops up to 80 passes with 50% relaxation on both greens and cycle, converging when the maximum green-interval change falls below 0.05 s (the HCM's Step R specifies 0.1 s; the code converges tighter). The initial green estimate is the maximum green (Step B, fully actuated), and the initial cycle is the sum of phases 2–4 max greens plus change periods.

| Sub-step (HCM Ch. 31 §2) | Equations / Exhibits | Rust function / file | Inputs (units) | Outputs (units) |
|---|---|---|---|---|
| Maximum allowable headway (MAH) | Eq. 31-12 (approach speed), Eq. 31-11 (detected vehicle length), Eqs. 31-13..31-19 (per-lane-group MAH family), Eq. 31-10 context | `average_approach_speed`, `detected_vehicle_length`, `max_allowable_headway` with the `MahLaneGroup` enum (Through 31-13, LeftProtectedExclusive 31-14, LeftProtectedShared 31-15, LeftPermittedExclusive 31-16, RightProtectedExclusive 31-17, LeftPermittedShared 31-18, RightPermittedShared 31-19) in `actuated.rs` | Posted speed limit S_pl (mi/h), passage time PT (s), detection zone length L_ds (ft), heavy-vehicle % (with `STORED_PASSENGER_CAR_LENGTH_FT` 25 ft, `STORED_HEAVY_VEHICLE_LENGTH_FT` 45 ft, `DISTANCE_BETWEEN_STORED_VEHICLES_FT` 8 ft), base sat flow s_o (pc/h/ln), permitted-left sat flow s_l (veh/h/ln), f_Rpb (unitless) | MAH per lane group (s/veh); the phase MAH is the λ-weighted mean over its lane groups; `equivalent_mah_simultaneous` (Eq. 31-26) combines barrier-partner MAHs when simultaneous gap-out is enabled |
| Flow-rate parameters | Eqs. 31-3..31-8 (bunched-arrival λ*, φ*, Δ*, q*) | `bunching_params` (Δ = 1.5 s single-lane else 0.5 s; b = 0.6/0.5/0.8 for 1/2/3+ lanes), private `lane_group_flow_terms` (Eqs. 31-4/31-5), `phase_flow_parameters` (Eqs. 31-3, 31-6, 31-7, 31-8) in `actuated.rs` | Lane-group demand v (veh/h) and lane count (ln) | λ* flow-rate parameter (veh/s), φ* proportion free (unitless), Δ* headway offset (s), q* total call rate (veh/s) |
| Queue service time | Eq. 31-9 (with the Eq. 19-3/31-2 effective green g = G − l₁ + e) | private `queue_service` in `actuated.rs`; permitted-left lane groups are served over their unblocked green g_u at s_l instead of the phase green at s | v (veh/h), s or s_l (veh/h/ln), lanes, effective green or g_u (s), cycle (s), platoon ratio R_p (unitless, through P = R_p g/C) | g_s per phase = max over its lane groups (s); capped at the effective green (oversaturated lane groups return the full green) |
| Green extension and max-out | Eq. 31-28 (number of extensions n), Eq. 31-29 (extension probability p), Eq. 31-30 (average extension g_e), Eqs. 31-43..31-45 (max-out probability p_x) | `number_of_extensions`, `prob_green_extension`, `green_extension_time`, `prob_max_out` in `actuated.rs` | q* (veh/s), G_max (s), g_s (s), MAH* and Δ* (s), λ* (veh/s), φ* (unitless) | n (unitless), p (probability, clamped [0,1]), g_e (s), p_x (probability) |
| Probability of a phase call | Eqs. 31-31..31-33 (vehicle call p_v, pedestrian call p_p with P_p = 0.51 via `PROB_PEDESTRIAN_PUSH`) | `prob_phase_call` in `actuated.rs`; recall-to-max phases get p_c = 1.0 in the driver | Activating vehicle call rate q_v* (veh/s), pedestrian rate q_p* (p/s), cycle C (s) | p_c (probability) |
| Unbalanced green and phase duration | Eqs. 31-34..31-36 (green given vehicle/pedestrian calls, G_min floor, G_max cap) | `unbalanced_green` in `actuated.rs`; unbalanced phase duration D_up = G + Y + R_c inline in the driver (Steps M/N); protected-left phases suppress the pedestrian term | g_s, g_e, G_min, G_max, Walk + PC (s), p_v, p_p | Unbalanced green (s) and unbalanced phase duration (s) |
| Barrier balancing and cycle | Eqs. 31-38/31-39 (major/minor-street duration as the max over the two rings), Eq. 31-42 (equilibrium cycle = sum of Ring 1 phase durations) | Steps O–R inline in `estimate_fully_actuated` (the `set_dp` closure; Ring 1 = phases 1,2,3,4; Ring 2 = 5,6,7,8; major-street barrier {1,2,5,6}, minor-street barrier {3,4,7,8}; the lagging phase in each ring absorbs the barrier slack) | Per-phase unbalanced durations (s) | Balanced per-phase `ActuatedPhaseResult` (duration_s, green_s, queue_service_s, green_extension_s, mah_star_s, prob_max_out, prob_call; s or probabilities) and the cycle length (s) |

The facility-level wrapper `SignalizedIntersection::estimate_actuated_timings(simultaneous_gap_out)` in `signalized.rs` builds the `ActuatedPhaseInput` list from the *analyzed* intersection: it groups the milestone-1 lane groups by controlling phase number, maps each `LaneGroupKind`/`LeftTurnMode` pair to a `MahLaneGroup` category, reads v and s from the converged Step 3/4 results, g_u from the Step 4 permitted-green state, and detection/pedestrian/speed inputs from the owning `PhaseTiming` (defaults when absent: detector length 40 ft, min green 5 s, passage time 2 s) and `SignalApproach`. It requires `analyze()` to have been run first and does not mutate `self` — the fixed-timing pipeline stays the default path. Two wrapper simplifications worth noting when reviewing: it passes `f_rpb: 1.0` for every lane group (the Eq. 31-19 shared-right MAH form is reachable with a non-unit f_Rpb only through the lower-level `ActuatedLaneGroupInput` API, even though the analyzed state carries a computed f_Rpb), and it passes platoon ratio 1.0 (random arrivals) to the driver regardless of the approaches' input platoon ratios.

### Equation reference — queue service (Steps C/D)

The effective green used throughout the loop follows the Equation 19-3 / 31-2 relationship. With the HCM defaults l₁ = e = 2.0 s the two corrections cancel, so the driver's effective green equals the displayed green exactly:

```
Equation 31-2:  g = D_p − l₁ − l₂ = g_s + g_e + e     [s]
  g   = effective green time                                          [s]
  D_p = phase duration = G + Y + R_c                                  [s]
  l₁  = start-up lost time: 2.0 s (START_UP_LOST_TIME)
  l₂  = clearance lost time = Y + R_c − e                             [s]
  e   = extension of effective green: 2.0 s (EXTENSION_OF_EFFECTIVE_GREEN)
  g_s = queue service time (Eq. 31-9)                                 [s]
  g_e = green extension time (Eq. 31-30)                              [s]
Implemented in: src/hcm/signalized/actuated.rs::estimate_fully_actuated (g_eff = G − l₁ + e, floored at 0.1 s)
```

```
Equation 31-9:  g_s = q·C·(1 − P) ÷ (s/3,600 − q·C·(P/g))     [s]
  g_s = queue service time, capped at the effective green g           [s]
  q   = arrival flow rate = v ÷ (N × 3,600)                           [veh/s/ln]
  C   = cycle length                                                  [s]
  P   = proportion of vehicles arriving during green = R_p·g/C, capped at 1.0   [decimal]
  R_p = platoon ratio: 1.0 (random arrivals) from the facility wrapper
  s   = adjusted saturation flow rate                                 [veh/h/ln]
  g   = effective green time (g_u at s_l for a permitted left-turn lane group)   [s]
Implemented in: src/hcm/signalized/actuated.rs::queue_service
```

The code short-circuits to g_s = g when the lane group is oversaturated (arrivals per cycle q·C reach the served capacity s·g/3,600) or when the denominator is non-positive, and takes the phase g_s as the maximum over its lane groups; permitted-left lane groups (`LeftPermittedExclusive`, `LeftPermittedShared`) are served over their unblocked green g_u at the permitted saturation flow s_l instead of the phase green at s.

**CORRECTED:** the denominator of Eq. 31-9 now carries the cycle-length factor on its second term. The book's denominator is `s/3,600 − q·C·(P/g)` (verified against `245_Ch31_02.xhtml`); `queue_service` previously computed `denom = s/3,600 − q·P/g`, missing the `C`, which made the denominator too large and g_s too small. The code now reads `denom = s/3,600 − q·cycle_s·p/g_eff`, matching the correct milestone-1 form in `signalized.rs::queue_service_time`. Re-running the EP1 actuated convergence test (`test_actuated_phase_duration_ep1`) moved the minor-street through phases and cycle onto the published values: Ph8 51.26 → 54.00 s (pub 54.0), Ph4 53.87 → 57.79 s (pub 57.6), cycle 89.02 → 100.01 s (pub 101.8). As predicted, the correction slightly enlarges the documented protected-left over-service residual (Ph3 12.39 → 14.30 s, Ph7 14.99 → 18.09 s vs pub 10.2 / 13.8); the major-street phases 2/6 rose 22.76 → 27.92 s (pub 34.0), still under-extended by the combined-flow max-out engine gap. Fixed in commit for `fix/hcm-equation-sweep` (Eq 31-9 denominator).

### Equation reference — maximum allowable headway (Steps A/E/G)

```
Equation 31-12:  S_a = 0.90 × (25.6 + 0.47 × S_pl)     [mi/h]
  S_a  = average speed on the intersection approach                   [mi/h]
  S_pl = posted speed limit                                           [mi/h]
Implemented in: src/hcm/signalized/actuated.rs::average_approach_speed
```

```
Equation 31-11:  L_v = L_pc × (1 − 0.01·P_HV) + 0.01 × L_HV × P_HV − D_sv     [ft]
  L_v  = detected length of the vehicle                               [ft]
  L_pc = stored passenger-car lane length: 25 ft (STORED_PASSENGER_CAR_LENGTH_FT)
  P_HV = percentage heavy vehicles in the movement group              [%]
  L_HV = stored heavy-vehicle lane length: 45 ft (STORED_HEAVY_VEHICLE_LENGTH_FT)
  D_sv = distance between stored vehicles: 8 ft (DISTANCE_BETWEEN_STORED_VEHICLES_FT)
Implemented in: src/hcm/signalized/actuated.rs::detected_vehicle_length
```

The MAH family (Eqs. 31-13 through 31-19) shares one presence-mode base form (Eq. 31-10) — passage time plus the time for a detected vehicle length to traverse the detection zone — to which each lane-group category adds a turn-penalty term:

```
Equation 31-10 (general form):  MAH = PT + (L_ds + L_v) ÷ (1.47 × S_a) + ⟨turn-penalty term⟩     [s/veh]
  MAH  = maximum allowable headway (presence-mode stop-line detection)   [s/veh]
  PT   = passage time setting                                            [s]
  L_ds = length of the stop-line detection zone: 40 ft wrapper default   [ft]
  L_v  = detected vehicle length (Eq. 31-11)                             [ft]
  S_a  = average approach speed (Eq. 31-12)                              [mi/h]
  (in pulse mode MAH = PT directly; pulse substitution is left to the call site)
Implemented in: src/hcm/signalized/actuated.rs::max_allowable_headway
```

The fully worked representative case is the through lane group, whose turn-penalty term is zero:

```
Equation 31-13 (Through):  MAH_th = PT_th + (L_ds,th + L_v) ÷ (1.47 × S_a)     [s/veh]
  MAH_th  = maximum allowable headway for through vehicles               [s/veh]
  PT_th   = passage time setting for the phase serving through vehicles  [s]
  L_ds,th = stop-line detection zone length in the through lanes         [ft]
Implemented in: src/hcm/signalized/actuated.rs::max_allowable_headway (MahLaneGroup::Through)
```

The remaining six rows follow the same base-plus-penalty pattern; see the `MahLaneGroup` match in `max_allowable_headway` in `src/hcm/signalized/actuated.rs` for the exact per-branch arithmetic. Their penalty terms as transcribed in code are: Eq. 31-14 (LeftProtectedExclusive) adds `(E_L − 1)/(s_o/3,600)` with E_L = 1.05 (`E_L_PROTECTED_LEFT`); Eq. 31-15 (LeftProtectedShared) adds the same E_L term to MAH_th; Eq. 31-16 (LeftPermittedExclusive) adds `3,600/s_l − t_fh` with t_fh = 2.5 s (`FOLLOW_UP_HEADWAY_PERMITTED_LEFT`); Eq. 31-17 (RightProtectedExclusive) adds `(E_R − 1)/(s_o/3,600)` with E_R = 1.18 (`E_R_PROTECTED_RIGHT`); Eq. 31-18 (LeftPermittedShared) adds `3,600/s_l − t_fh` to MAH_th; Eq. 31-19 (RightPermittedShared) adds `((E_R/f_Rpb) − 1)/(s_o/3,600)` to MAH_th.

The per-phase MAH is then formed as the flow-weighted mean over the phase's lane groups, `MAH_phase = Σ(MAH_i·λ_i)/Σλ_i`, which reproduces the book's all-exclusive-lane combination (Eq. 31-23) exactly.

**DISCREPANCY:** the phase-level MAH* aggregation does not implement the shared-lane splits of Equations 31-20 through 31-25. For a phase serving a shared-lane lane group, the book splits that lane group's flow-rate parameter λ between its turning proportion (P_L or P_R, weighted by MAH_lt,s or MAH_rt,s) and its through proportion (1 − P_L or 1 − P_R, weighted by MAH_th) — e.g. Eq. 31-20's numerator is `P_L·λ_sl·MAH_lt,s + [(1−P_L)·λ_sl + λ_t + (1−P_R)·λ_sr]·MAH_th + P_R·λ_sr·MAH_rt,s`. The driver instead weights each lane group's entire λ by the single MAH of its `mah_kind` category and carries no P_L/P_R input at all (`ActuatedLaneGroupInput` has no turning-proportion field). This coincides with the book only when every lane group is an exclusive lane (Eqs. 31-23/31-24); whenever a shared lane group's through proportion is non-trivial the code overstates the turning movement's influence on MAH* and understates the through movement's. The code is left unchanged; recorded here as a new finding from this documentation pass.

```
Equation 31-26:  MAH* = (MAH_i × Σλ_i + MAH_c × Σλ_c,i) ÷ (Σλ_i + Σλ_c,i)     [s/veh]
  MAH*   = equivalent maximum allowable headway for the phase (simultaneous gap-out)   [s/veh]
  MAH_i  = phase MAH computed above for the subject phase              [s/veh]
  MAH_c  = phase MAH computed above for the concurrent barrier partner (2↔6, 4↔8)   [s/veh]
  λ_i    = flow rate parameter, lane group i of the subject phase      [veh/s]
  λ_c,i  = flow rate parameter, lane group i of the concurrent phase   [veh/s]
Implemented in: src/hcm/signalized/actuated.rs::equivalent_mah_simultaneous
```

### Equation reference — bunched-arrival flow parameters (Step F)

Lane-group terms first (each lane group's arrival stream is modeled as a bunched Poisson process):

```
Equation 31-5:  φ_i = e^(−b_i × Δ_i × q_i)     [decimal]
Equation 31-4:  λ_i = φ_i × q_i ÷ (1 − Δ_i × q_i)     [veh/s]
  φ_i = proportion of free (unbunched) vehicles in lane group i
  λ_i = flow rate parameter for lane group i                           [veh/s]
  q_i = arrival flow rate for lane group i = v_i ÷ 3,600               [veh/s]
  v_i = demand flow rate for lane group i                              [veh/h]
  Δ_i = bunched-stream headway: 1.5 s (single-lane group), 0.5 s otherwise
  b_i = bunching factor: 0.6 (1 lane), 0.5 (2 lanes), 0.8 (3+ lanes)
Implemented in: src/hcm/signalized/actuated.rs::lane_group_flow_terms (Δ, b from bunching_params)
```

Then the phase-level aggregation over the m lane groups the phase serves:

```
Equation 31-3:  λ* = Σᵢ λ_i     [veh/s]
Equation 31-6:  φ* = e^(−Σᵢ b_i·Δ_i·q_i)     [decimal]
Equation 31-7:  Δ* = Σᵢ(λ_i·Δ_i) ÷ λ*     [s/veh]
Equation 31-8:  q* = Σᵢ q_i     [veh/s]
  λ* = flow rate parameter for the phase                               [veh/s]
  φ* = combined proportion of free (unbunched) vehicles for the phase  [decimal]
  Δ* = equivalent headway of the bunched stream served by the phase    [s/veh]
  q* = arrival (call) flow rate for the phase                          [veh/s]
Implemented in: src/hcm/signalized/actuated.rs::phase_flow_parameters
```

### Equation reference — green extension and max-out (Steps H/I)

```
Equation 31-28:  n = max(q* × [G_max − (g_s + l₁)], 0)     [extensions]
  n     = average number of extensions before max-out
  q*    = phase call rate (Eq. 31-8)                                   [veh/s]
  G_max = maximum green setting                                        [s]
  g_s   = queue service time (Eq. 31-9)                                [s]
  l₁    = start-up lost time: 2.0 s
Implemented in: src/hcm/signalized/actuated.rs::number_of_extensions
```

```
Equation 31-29:  p = 1 − φ* × e^(−λ*·(MAH* − Δ*))     [decimal]
  p = probability that a call headway is less than MAH* (green is extended), clamped to [0, 1]
Implemented in: src/hcm/signalized/actuated.rs::prob_green_extension
```

```
Equation 31-30:  g_e = p² × (1 − pⁿ) ÷ (q* × (1 − p))     [s]
  g_e = average green extension time                                   [s]
  n   = number of extensions (Eq. 31-28)
Implemented in: src/hcm/signalized/actuated.rs::green_extension_time
```

```
Equation 31-45:  h = [Δ* + φ*/λ* − (MAH* + 1/λ*) × φ* × e^(−λ*·(MAH*−Δ*))] ÷ [1 − φ* × e^(−λ*·(MAH*−Δ*))]     [s]
Equation 31-44:  n_x = max((G_max − MAH* − (g_s + l₁)) ÷ h, 0)     [calls]
Equation 31-43:  p_x = p^(n_x)     [decimal]
  h   = average call headway for calls shorter than MAH*               [s]
  n_x = number of calls necessary to extend the green to max-out
  p_x = probability of phase termination by max-out
Implemented in: src/hcm/signalized/actuated.rs::prob_max_out
```

### Equation reference — phase call probability and unbalanced green (Steps J through N)

```
Equation 31-32:  p_v = 1 − e^(−q_v*·C)     [decimal]
Equation 31-33:  p_p = 1 − e^(−q_p*·P_p·C)     [decimal]
Equation 31-31:  p_c = p_v·(1 − p_p) + p_p·(1 − p_v) + p_v·p_p     [decimal]
  p_c  = probability that the subject phase is called
  p_v  = probability of a call by vehicle detection
  p_p  = probability of a call by pedestrian detection
  q_v* = activating vehicular call rate for the phase                  [veh/s]
  q_p* = activating pedestrian call rate for the phase                 [p/s]
  P_p  = probability a pedestrian presses the detector button: 0.51 (PROB_PEDESTRIAN_PUSH)
  C    = cycle length                                                  [s]
Implemented in: src/hcm/signalized/actuated.rs::prob_phase_call (recall-to-max phases get p_c = 1.0 in the driver)
```

```
Equation 31-35:  G|veh = max(l₁ + g_s + g_e, G_min)     [s]
Equation 31-36:  G|ped = Walk + PC     [s]
Equation 31-34:  G_u = G|veh·p_v·(1 − p_p) + G|ped·p_p·(1 − p_v) + max(G|veh, G|ped)·p_v·p_p, capped at G_max     [s]
  G_u   = unbalanced green interval duration for the phase             [s]
  G|veh = green duration given a vehicle call                          [s]
  G|ped = green duration given a pedestrian call                       [s]
  G_min = minimum green setting: 5 s wrapper default                   [s]
  Walk, PC = pedestrian walk and pedestrian-clear settings             [s]
Implemented in: src/hcm/signalized/actuated.rs::unbalanced_green
```

When the phase has no pedestrian service (Walk + PC absent or zero, or a protected-left phase, which suppresses the pedestrian term in the driver) the code takes G_u = G|veh directly. The unbalanced phase duration is then D_up = G_u + Y + R_c inline in the driver (Steps M/N).

### Equation reference — barrier balancing and equilibrium cycle (Steps O through R)

```
Equation 31-38 (major street):  D_p,b = max(D_up,1 + D_up,2, D_up,5 + D_up,6) − D_p,a     [s]
Equation 31-39 (minor street):  D_p,b = max(D_up,3 + D_up,4, D_up,7 + D_up,8) − D_p,a     [s]
  D_p,a  = duration of the phase that occurs first in the barrier's phase pair (its unbalanced duration D_up,a)   [s]
  D_p,b  = duration of the phase that follows phase a before the barrier (absorbs the barrier slack)              [s]
  D_up,i = unbalanced phase duration for phase i (Steps M/N)           [s]
Implemented in: src/hcm/signalized/actuated.rs::estimate_fully_actuated (the set_dp closure; Ring 1 = phases 1,2,3,4; Ring 2 = 5,6,7,8; major-street barrier {1,2,5,6}, minor-street barrier {3,4,7,8})
```

In the driver, `major`/`minor` are the two max(...) terms; phases 2 and 6 receive `major − D_p,1` and `major − D_p,5` (Eq. 31-38 applied to each ring), phase 4 receives `minor − D_p,3` (leading-left arrangement, 3 leads 4) and phase 7 receives `minor − D_p,8` (lagging-left arrangement, 8 leads 7), matching the book's worked examples for Eq. 31-39.

```
Equation 31-42:  C_e = Σᵢ₌₁⁴ D_p,i     [s]
  C_e   = equilibrium cycle length, summed over the Ring 1 phases (1, 2, 3, 4)
  D_p,i = balanced duration of Ring 1 phase i                          [s]
Implemented in: src/hcm/signalized/actuated.rs::estimate_fully_actuated (the in-loop new_cycle and the returned cycle, relaxed 50% per pass)
```

## Part 2 — Left-turn ADP first-term back of queue (Exhibits 31-26..31-31, Eq. 31-141)

Milestone 1 approximated Q1 for permitted and protected-permitted left-turn lane groups by the maximum instantaneous queue of the Step 8 queue accumulation polygon. Milestone 2 replaces that with `adp_first_term_left(intervals, cycle_s, d_a)` in `signalized.rs`, called from `step_8_delay`'s QAP branch (the same `QapInterval` list built by `build_left_turn_qap` feeds both d1 and Q1). The function counts *full stops* N_f rather than the peak queue: it first iterates the polygon to its steady-state starting queue (fixed point of the cycle map, up to 80 passes), then walks two concatenated cycles at a 0.02 s step so a busy period spanning the cycle boundary is captured in full, takes the longest contiguous busy period (queue > 0, with sneaker removals able to end a busy period), and returns `q × max_busy − q × d_a/2` (veh/ln) — the arrival count over the longest busy period minus the partial-stop window of Equations 31-139/31-141 (the fully-stopped departure "dashed line" of Section 4, Step 3 leads the solid departure by d_a/2, with d_a from `accel_decel_delay`, Eq. 31-131, in s). This follows the Section 4, Step 6 rule for complex left-turn ADPs whose queue dissipates at two or more points per cycle: N_f,i is computed per period between dissipation points and the largest governs. Inputs are the polygon intervals (durations s, discharge veh/h/ln, arrivals veh/s/ln, sneakers veh), cycle length (s), and d_a (s); the output Q1 (veh/ln) is stored on `lg.q1_veh` and consumed unchanged by `step_10_queue_storage` for permitted/protected-permitted left-turn lane groups. For a lane group served in two batches per cycle (protected phase plus permitted/sneaker service) N_f exceeds the peak queue because it counts every vehicle that stops — exactly the published SB-left case (4.9 veh/ln, where the milestone-1 QAP peak gave 3.2).

The acceleration–deceleration delay that positions the partial-stop window comes from the Section 4 speed model:

```
Equation 31-132:  S_a = 0.90 × (25.6 + 0.47 × S_pl)     [mi/h]
Equation 31-131:  d_a = [1.47 × (S_a − S_s)]² ÷ (2 × 1.47 × S_a) × (1/r_a + 1/r_d)     [s]
  d_a  = acceleration–deceleration delay per full stop                 [s]
  S_a  = average (unimpeded) speed on the intersection approach        [mi/h]
  S_pl = posted speed limit                                            [mi/h]
  S_s  = threshold speed defining a stopped vehicle: 5.0 mi/h (STOP_THRESHOLD_SPEED_MPH)
  r_a  = acceleration rate: 3.5 ft/s² (QUEUE_ACCELERATION_RATE)
  r_d  = deceleration rate: 4.0 ft/s² (QUEUE_DECELERATION_RATE)
Implemented in: src/hcm/signalized/signalized.rs::accel_decel_delay
```

For the basic single-dissipation-point polygon (Exhibit 31-25 — through movements, protected turns, shared through+right), the book's closed-form full-stop accounting is transcribed exactly in `first_term_back_of_queue` and is documented in `chapter19.md`'s Step 10 section (Eqs. 31-137 through 31-140, with Q1 = N_f per Eqs. 31-136/31-141). The left-turn ADP family extends that polygon to six left-turn-specific geometries: Exhibit 31-26 (permitted, exclusive lane), 31-27 (permitted, shared lane), 31-28 (leading protected-permitted, exclusive), 31-29 (lagging protected-permitted, exclusive), 31-30 (leading protected-permitted, shared), and 31-31 (lagging protected-permitted, shared). Each introduces the permitted-period variables g_p, g_u, g_f, the protected green g_l, the permitted saturation flow s_p, the protected left-turn saturation flow s_lt = s_th/E_L, and the shared-lane left proportion P_L. The book gives no closed-form N_f for these shapes — only the Step 6 rule quoted above (compute N_f,i per period between queue dissipation points; the largest governs):

```
Equation 31-141:  Q1 = N_f     [veh/ln]
  Q1  = first-term back-of-queue size                                  [veh/ln]
  N_f = number of fully stopped vehicles; for complex left-turn polygons, the largest N_f,i over the periods between queue dissipation points   [veh/ln]
Implemented in: src/hcm/signalized/signalized.rs::adp_first_term_left (left-turn path; returns q × max_busy − q × d_a/2, floored at 0); src/hcm/signalized/signalized.rs::first_term_back_of_queue (basic Exhibit 31-25 path)
```

`build_left_turn_qap` reproduces the Exhibit 31-26/31-27 (permitted) and 31-28..31-31 (protected-permitted, leading/lagging, exclusive/shared) geometries as ordered `QapInterval` lists (durations, discharge rates s_left_perm/s_left_prot and the shared-lane rates derived from s_th_curb, arrival rate q, sneaker counts), and `adp_first_term_left` evaluates the max-over-dissipation-points rule numerically on that list. The single-formula collapse `q × max_busy − q × d_a/2` stands in for the book's per-interval d_a-shifted counting (Eqs. 31-137..31-140 applied per dissipation interval); this approximation and its ~0.1–0.3 veh/ln residual are the already-documented Deviations item 2 below.

## Part 3 — RTOR estimation (Ch. 31 §8)

`SignalizedIntersection::estimate_rtor_volume(direction)` in `signalized.rs` implements the Chapter 31, Section 8 suggestion that an exclusive right-turn lane's RTOR volume can be estimated as the demand of the complementary cross-street left-turn movement whenever that movement is provided a left-turn phase. The complementary movement is identified by the private `cross_street_left_shadow` as the approach 90° counterclockwise of the subject (EB→NB, NB→WB, WB→SB, SB→EB), i.e. the cross-street left that discharges into the subject right turn's receiving lanes and whose protected phase clears the conflicting through movement. The estimate returns 0.0 when the subject has no exclusive right-turn lane, when the shadow approach is absent, or when the shadow's left-turn mode is not `Protected`/`ProtectedPermitted`; otherwise it returns the shadow approach's PHF-adjusted left-turn flow rate (veh/h) capped at the subject's PHF-adjusted right-turn flow rate (veh/h). `apply_rtor_estimates()` populates `volume_rtor` on every approach that has none supplied (a nonzero field-measured `volume_rtor` is left untouched) and is meant to be called before `analyze()`, which subtracts RTOR from the right-turn demand in Step 2 (see `chapter19.md`). Shared-lane RTOR is left at 0.0 — the HCM offers no estimate for that case and recommends field data or an alternative tool. Both RTOR methods and `estimate_actuated_timings` are exposed to Python in `src/copython/signalized.rs` (the actuated results are returned as a JSON array of per-phase records).

The book states this rule in prose only — Section 8's "Effect of Right-Turn-on-Red Operation" subsection (inside the "Use of Alternative Tools" section, EPUB `251_Ch31_08.xhtml`) carries no numbered equation: "the methodology suggests RTOR volume can be estimated as equal to the left-turn demand of the complementary cross street left-turn movement, whenever this movement is provided a left-turn phase." The book gives no formal movement map for "complementary" (the 90°-counterclockwise reading is the implemented interpretation, Deviations item 3), no explicit cap, and no shared-lane estimate; the code's cap at the subject's own right-turn flow rate is a physical bound filled in by the implementation (RTOR volume cannot exceed the right-turn demand it is drawn from) rather than a book statement. Implemented in: `src/hcm/signalized/signalized.rs::estimate_rtor_volume`, `src/hcm/signalized/signalized.rs::cross_street_left_shadow`, `src/hcm/signalized/signalized.rs::apply_rtor_estimates`.

## Part 4 — How k and c_a feed d2

The incremental-delay hand-off from actuated timing to Step 8 lives in milestone-1 code paths that milestone 2 makes fully usable. When `SignalizedIntersection.control` is `ActuatedSignal`/`SemiActuatedSignal` and the governing phase carries both `passage_time_s` and `max_green_s`, `step_8_delay` in `signalized.rs` computes the incremental delay factor as k = `incremental_delay_factor_actuated(v/c_a, k_min)` (Eq. 19-22, clamped to [k_min, 0.50]) with k_min = `incremental_delay_factor_min(PT)` (Eq. 19-23, floored at 0.04 via `K_MIN_LOWER_BOUND`), both in `src/hcm/common/delay.rs`; otherwise k falls back to `K_PRETIMED` = 0.50 (pretimed, coordinated, and recall-to-max phases per the Chapter 19, Step 8C text).

```
Equation 19-23:  k_min = −0.375 + 0.354·PT − 0.0910·PT² + 0.00889·PT³, floored at 0.04     [unitless]
  k_min = minimum incremental delay factor
  PT    = passage time setting                                         [s]
Implemented in: src/hcm/common/delay.rs::incremental_delay_factor_min (K_MIN_LOWER_BOUND = 0.04)
```

```
Equation 19-22:  k = (1 − 2·k_min) × (v/c_a − 0.5) + k_min, clamped to [k_min, 0.50]     [unitless]
  k     = incremental delay factor for actuated operation
  v/c_a = ratio of demand flow rate to available capacity (Eq. 19-24)
Implemented in: src/hcm/common/delay.rs::incremental_delay_factor_actuated (K_PRETIMED = 0.50 for the non-actuated fallback)
```

The available capacity c_a comes from Step 7 (`step_7_capacity_and_vc`): the generic lane-group form is c_a = N·s·g_a/C with g_a = `PhaseTiming::available_effective_green_s()`:

```
Equation 19-25:  g_a = G_max + Y + R_c − l₁ − l₂  (= G_max − l₁ + e)     [s]
  g_a   = available effective green time for an actuated lane group     [s]
  G_max = maximum green setting                                         [s]
  Y     = yellow change interval                                        [s]
  R_c   = red clearance interval                                        [s]
  l₁    = start-up lost time: 2.0 s
  l₂    = clearance lost time = Y + R_c − e; e = 2.0 s
Implemented in: src/hcm/signalized/signalized.rs::PhaseTiming::available_effective_green_s
```

and the permitted / protected-permitted left-turn arms use the Eq. 31-120 / 31-125 available-capacity forms (written out in `chapter19.md`'s Step 7 section). The resulting k and the Step 7 capacity then enter d2:

```
Equation 19-26:  d2 = 900 × T × [(X_A − 1) + √((X_A − 1)² + 8·k·I·X_A ÷ (c_A·T))]     [s/veh]
  d2  = incremental delay                                              [s/veh]
  T   = analysis period duration: 0.25 default                         [h]
  X_A = average volume-to-capacity ratio v/c_A                         [unitless]
  c_A = average lane group capacity                                    [veh/h]
  k   = incremental delay factor (Eq. 19-22, or K_PRETIMED = 0.50)     [0.04–0.50]
  I   = upstream filtering adjustment factor: 1.0 isolated (I_ISOLATED), floored at 0.090 (I_MIN)
Implemented in: src/hcm/common/delay.rs::incremental_delay_signalized
```

The intended workflow for an actuated intersection is therefore: run `analyze()` at an initial timing, run `estimate_actuated_timings` to converge the average phase durations, write those durations back into the `PhaseTiming.duration_s` inputs (the caller's responsibility — the estimator does not mutate the facility), and re-run `analyze()` so capacities, k, and d2 reflect the converged timings.

## Deviations (cross-referenced to `docs/hcm/VERIFICATION.md`, "Chapter 19 milestone 2")

1. **Actuated convergence vs the published EP1 durations (VERIFICATION.md item 1; `VERIFY-HCM` comment at the green-extension step inside `estimate_fully_actuated`).** Driven from the Example Problem 1 controller settings with the Steps 1–5 operating point held at the published values, the procedure reproduces the equivalent MAH exactly (3.4 s EB/WB, 3.1 s minor street) and the barrier balance exactly. Following the Eq. 31-9 denominator correction (Part 1 above), the minor-street through phases now land on the published durations (Ph8 = 54.00 vs 54.0 s; Ph4 = 57.79 vs 57.6 s; SB-T g_e = 9.02 vs 7.8 s) and the estimated cycle is 100.0 s, within ~2 s of the published 101.8 s (was ~89 s). Two residuals remain: (a) the major-street phases 2/6 under-extend (~28 vs 34 s) because the HCM computational engine's combined-flow max-out model holds them at max green while the transcribed Eq. 31-29/31-30 green-extension model gaps them out; (b) the leading protected left phases 3/7 are charged the full left-turn demand for queue service rather than only the demand not served in the following permitted period, so they over-serve (Ph3 = 14.30 vs 10.2 s; Ph7 = 18.09 vs 13.8 s) — a residual the Eq. 31-9 correction slightly enlarges. Closing both requires embedding the full Steps 1–5 recomputation and the engine's combined-flow extension calibration inside every actuated iteration (a Section 7 computational-engine detail). Of the two **DISCREPANCY** blocks in Part 1 above, the missing cycle-length factor in Eq. 31-9's denominator is now **fixed** (as summarized here); the missing Eq. 31-20..31-25 shared-lane MAH* split remains a reported, unfixed finding.
2. **Left-turn ADP first-term partial-stop offset (VERIFICATION.md item 2; `VERIFY-HCM` comment in `adp_first_term_left`).** Q1 is computed as the largest per-busy-period arrival count less q·d_a/2, standing in for the engine's exact multi-segment N_f accounting (Eqs. 31-137..31-140 applied per dissipation interval). This reproduces EP1 EB-left 1.8 exactly and SB-left 4.9 → 5.0 (previously 3.2 under the QAP peak) and keeps NB-left within the published queue-storage tolerance; the exact accounting would remove a ~0.1–0.3 veh/ln residual on the more complex polygons.
3. **RTOR complementary-movement identification (VERIFICATION.md item 3; `VERIFY-HCM` comment in `cross_street_left_shadow`).** The HCM text says "the left-turn demand of the complementary cross street left-turn movement" without a formal movement map; the 90°-counterclockwise rotation is the implemented reading (the receiving-lane match). Shared-lane RTOR is 0.0 by design (no HCM estimate exists).
4. **Deferred controller-emulation details (VERIFICATION.md item 4).** Permissive-period modeling; the coordinated-actuated force-off/yield-point emulation beyond the equivalent-maximum-green abstraction (Eqs. 31-27, 31-40); Dallas left-turn phasing; dual-entry activation edge cases; and pulse-mode detection (in pulse mode MAH equals PT; the presence-mode form is implemented and pulse substitution is left to the call site, per the `max_allowable_headway` doc comment).

## Validation

Fixture: `tests/ExampleCases/hcm/Signalized/case1.json` (HCM Chapter 31, Section 10, Example Problem 1; controller settings of Exhibit 31-72, published lane-group demand of Exhibit 31-76, adjusted saturation flows of Exhibit 31-77, converged durations and permitted g_u of Exhibit 31-79, back-of-queue values of Exhibit 31-82). Three test layers cover milestone 2. (1) Unit tests in `actuated.rs`'s `#[cfg(test)]` module: `test_average_approach_speed` (Eq. 31-12, S_a = 37.845 mi/h at 35 mi/h, tolerance 1e-3), `test_detected_vehicle_length` (Eq. 31-11, 18.0 / 17.4 ft at 5% / 2% HV, exact), `test_through_mah_ep1` (Eqs. 31-13/31-10, through MAH ≈ 3.03 ±0.05 s for the EP1 40-ft zone at PT = 2 s), `test_flow_rate_parameter_monotonic`, `test_green_extension`, and `test_actuated_phase_duration_ep1` — the milestone acceptance test that hand-builds the six EP1 phases at the published operating point and asserts the barrier balance to 1e-6, MAH within 0.15–0.2 s of the published values, Ph8 duration 54.0 ±3.5 s, Ph4 duration 57.6 ±4.5 s, Ph4 green extension 7.8 ±2.5 s, Ph3/Ph7 durations ±3.0 s, Ph8 max-out probability > 0.5 (published 1.00), and cycle within 14 s of the published 101.8 s (the documented engine gap). (2) Unit tests added to `src/hcm/signalized/tests.rs`: `test_adp_first_term_left_full_stops` (a single protected batch reduces N_f to the peak queue; a permitted lane group whose queue is held most of the cycle and released by sneakers counts nearly every arrival), `test_rtor_estimate_exclusive_right_lane` (EP1 modified so an eastbound exclusive right lane is shadowed by the northbound protected-permitted left), `test_rtor_no_left_phase_no_estimate` (rotation consistency — no fixed points — and the zero estimate without a protected shadow phase), `test_estimate_actuated_timings_from_facility` (the facility wrapper reproduces the EP1 minor-street through phases within the documented tolerance), plus the updated `test_step_10_back_of_queue` asserting the Exhibit 31-82 left-turn values via the ADP path (±0.5 veh/ln; oversaturated NB through groups ±0.8). (3) Integration tests appended to `tests/chapter19_integration.rs`: `test_m2_sb_left_back_of_queue` (SB-left 4.9 ±0.5, EB-left 1.8 ±0.4, NB-left 1.4 ±0.5 veh/ln vs Exhibit 31-82), `test_m2_actuated_phase_durations` (barrier balance to 1e-6, MAH Ph2 3.4 ±0.1 and Ph8 3.1 ±0.2 s, Ph8 54.0 ±4.0 s, Ph4 57.6 ±5.0 s), and `test_m2_rtor_estimate` (zero-estimate guards for permitted-only shadow lefts and shared right lanes).

## Deferred

Tracked in `docs/hcm/VERIFICATION.md` under "Deferred scopes — Ch 19 later": full computational-engine actuated convergence to 0.1 s (combined-flow max-out and in-loop Steps 1–5 recomputation), Dallas phasing, pedestrian/bicycle LOS, and multi-period analysis. Within this branch's code specifically: `estimate_actuated_timings` holds demand and permitted green fixed at the analyzed operating point (no in-loop recomputation); coordinated-actuated operation is represented only through the equivalent-maximum-green abstraction; converged durations are not automatically written back into the analysis pipeline (the caller re-runs `analyze()` with them); the facility wrapper fixes f_Rpb = 1.0 and R_p = 1.0 as noted in Part 1; and shared-lane RTOR estimation is intentionally absent.
