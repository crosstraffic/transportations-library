# HCM Chapter 18/30 — Computed Platoon Dispersion and Access-Point Delay (Milestone 2)

This document walks through the "milestone 2" computed procedures for HCM 7th Edition Chapter 18 urban street segments, implemented on branch `feat/hcm-ch18-platoon-dispersion`: the Chapter 30, Section 3 platoon-dispersion primitives (EPUB `235_Ch30_03.xhtml`) that let Step 3 (proportion arriving during green) be computed from an upstream signal's discharge flow profile instead of assumed uniform or taken from a supplied platoon ratio, and the Chapter 30, Section 4 delay-due-to-turns-at-access-points procedure (EPUB `236_Ch30_04.xhtml`) that lets Step 2's `Σ d_ap,i` term be computed from access-point geometry and turning volumes instead of supplied per-point delays or the Exhibit 18-13 planning estimate. The code lives in `src/hcm/urban_segments/platoon_dispersion.rs` (Section 3) and `src/hcm/urban_segments/access_point_delay.rs` (Section 4); both are wired into `UrbanSegment` in `src/hcm/urban_segments/urban_segments.rs` (milestone 1, documented in `chapter18.md`) as optional, computed-vs-input mode switches. `docs/hcm/VERIFICATION.md` exists at this branch's tip and carries a "Chapter 18/30 computed procedures (feat/hcm-ch18-platoon-dispersion)" section that is the authoritative source for the deviations below.

## Section 3 — Platoon dispersion and the arrival flow profile

The model represents one average signal cycle as `C' = cycle_s / time_step_s` discrete one-second time steps (the Chapter 30 TRANSYT-7F convention), and disperses a per-step discharge-flow-rate vector into a per-step arrival-flow-rate vector at the downstream boundary.

| Step | Equation(s) | Rust function | File | Inputs (units) | Outputs (units) |
|---|---|---|---|---|---|
| Smoothing factor | Eq. 30-11, `F = 1/(1 + 0.138 t'_R + 0.315/d_t)` | `smoothing_factor` | `platoon_dispersion.rs` | `running_time_s` (t_R, s), `time_step_s` (d_t, s/step) | `F` (unitless) |
| Platoon arrival time | Eq. 30-12, `t' = t'_R − 1/F + 1.25` (steps) | `platoon_arrival_time_steps` | `platoon_dispersion.rs` | same | `t'` (steps) |
| Dispersion recursion | Eq. 30-9/30-10, `q'_a\|j = F q'_u,i + (1−F) q'_a\|j−1`, `j = i + t'` | `disperse_profile` | `platoon_dispersion.rs` | `discharge_profile: &[f64]` (veh/step, len C'), `running_time_s`, `time_step_s` | `Vec<f64>` arrival profile (veh/step, len C'), flow-conserving |
| Discharge profile construction | (not separately numbered; queue-service-time-aware discretization) | `MovementDischarge::to_profile` | `platoon_dispersion.rs` | `discharge_volume_veh_h`, `saturation_flow_veh_h`, `green_start_s`, `green_duration_s`, `queue_service_time_s` | `Vec<f64>` discharge profile (veh/step) |
| Combined arrival profile | Eq. 30-9 applied per movement, summed, plus uniform midblock term | `combined_arrival_profile` | `platoon_dispersion.rs` | `movements: &[MovementDischarge]`, `uniform_volume_veh_h` (veh/h), `cycle_steps`, `time_step_s`, `running_time_s` | `Vec<f64>` combined arrival profile (veh/step) |
| Proportion arriving on green | (Step 3 definition; not separately numbered) | `proportion_arriving_green` | `platoon_dispersion.rs` | `arrival_profile: &[f64]`, `green_start_step`, `green_steps` | `P` (decimal, clamped to `[0, 1]`) |
| Proportion of time blocked | Eq. 30-13, `p_b = t'_p d_t / C` | `proportion_time_blocked` | `platoon_dispersion.rs` | `blocked_period_steps`, `time_step_s`, `cycle_s` | `p_b` (decimal, clamped) |
| Critical platoon flow rate | (Section 3 "Proportion of Time Blocked" discussion) `q_c = 3,600/t_c` | `critical_platoon_flow_rate` | `platoon_dispersion.rs` | `critical_headway_s` (t_c, s, Chapter 20) | `q_c` (veh/h) |

### Section 3 equations in full

```
Equation 30-11:  F = 1 / (1 + 0.138·t'_R + 0.315/d_t)     [unitless]
  t'_R = segment running time expressed in time steps    (steps; = t_R / d_t)
  t_R  = segment running time                            (s)
  d_t  = time step duration                              (s/step; Chapter 30 recommends 1.0)
Implemented in: urban_segments/platoon_dispersion.rs::smoothing_factor
```

```
Equation 30-12:  t' = t'_R − 1/F + 1.25     [steps]
  t'_R = segment running time expressed in time steps    (steps; = t_R / d_t)
  F    = smoothing factor                                (unitless, Equation 30-11)
Implemented in: urban_segments/platoon_dispersion.rs::platoon_arrival_time_steps
```

```
Equation 30-9:   q'_a|u,j = F·q'_u,i + (1 − F)·q'_a|u,j−1     [veh/step]
Equation 30-10:  j = i + t'
  q'_a|u,j = arrival flow rate at the downstream intersection, time step j, from upstream source u   (veh/step)
  q'_u,i   = departure (discharge) flow rate at upstream source u, time step i                        (veh/step)
  F        = smoothing factor                                                                         (unitless, Eq 30-11)
  t'       = platoon arrival time                                                                      (steps, Eq 30-12)
  i, j     = time-step indices, 0-based, cyclic modulo C' = cycle length in steps                      (steps)
Code note: the recursion is iterated to its periodic steady state (up to 100 passes, L1 convergence < 1e-12 on the wrap-around seam) rather than solved in one closed pass, and t' is rounded to the nearest integer step for the index lag j = i + t' — the fractional remainder is absorbed by the (1 − F) smoothing memory rather than by interpolation. The module doc comment flags this as a discretization choice, not a literal transcription of Equation 30-10.
Implemented in: urban_segments/platoon_dispersion.rs::disperse_profile
```

```
Discharge flow profile construction (Chapter 30 §3 "Discharge Flow Profile" narrative; not separately numbered):
  during the queue service time g_s:        rate = s·d_t/3,600                                              [veh/step]
  during the remainder of green (g − g_s):  rate = back-solved so Σ(profile over the cycle) = V·(C'·d_t)/3,600  [veh/step]
  outside of green:                         rate = 0
  g   = effective green duration                                    (s)
  g_s = queue service time, clamped to [0, g]                       (s)
  s   = saturation flow rate of the movement's lane group           (veh/h)
  V   = adjusted discharge volume for the movement                  (veh/h)
  C'  = cycle length in time steps, d_t = time step duration        (steps, s/step)
Code note: both the green and queue-service intervals are discretized to whole steps first, then the post-queue-service rate is back-solved from the remaining vehicle count, so the profile integrates exactly to V with no continuous-vs-discrete rounding drift (per the function's doc comment).
Implemented in: urban_segments/platoon_dispersion.rs::MovementDischarge::to_profile
```

```
Combined arrival flow profile (Equation 30-9 applied per upstream movement and summed, plus a uniform midblock term per Chapter 30 §3 "Arrival Flow Profile": midsegment arrivals "are assumed to have a uniform arrival flow profile"; not separately numbered):
  q'_a,j(combined) = Σ_u disperse(q'_u,•)_j  +  v_unif·d_t/3,600     [veh/step]
  disperse(q'_u,•)_j = the Equation 30-9/30-10 dispersed profile for upstream movement u, at step j   (veh/step)
  v_unif = uniform (midblock/access-point) arrival volume entering the cycle uniformly                 (veh/h)
Implemented in: urban_segments/platoon_dispersion.rs::combined_arrival_profile
```

```
Proportion of vehicles arriving during green (Step 3 definition; not separately numbered):
  P = ( Σ_{j∈green} q'_a,j ) / ( Σ_{j=1}^{C'} q'_a,j )     [decimal, clamped to [0, 1]]
  green = the green_steps time steps starting at green_start_step, indices wrapped modulo C'
Implemented in: urban_segments/platoon_dispersion.rs::proportion_arriving_green
```

```
Equation 30-13:  p_b = t'_p·d_t / C     [decimal, clamped to [0, 1]]
  t'_p = blocked period duration                    (steps)
  d_t  = time step duration                         (s/step)
  C    = cycle length                               (s)
Implemented in: urban_segments/platoon_dispersion.rs::proportion_time_blocked
```

```
Critical platoon flow rate (Chapter 30 §3 "Proportion of Time Blocked" discussion; not separately numbered):
  q_c = 3,600 / t_c     [veh/h]
  t_c = critical headway of the minor movement (Chapter 20, TWSC)     (s)
Implemented in: urban_segments/platoon_dispersion.rs::critical_platoon_flow_rate
```

`disperse_profile` implements the cyclic steady-state recursion by iterating the periodic `(1−F)`-memory update up to 100 passes with an L1 convergence check at `1e-12`; the platoon arrival time `t'` is rounded to the nearest integer step for the index lag (the fractional remainder is absorbed by the smoothing recursion rather than by interpolation), which the module doc comment flags as a discretization choice rather than a literal transcription of Equation 30-10.

`MovementDischarge::to_profile` builds the per-step discharge vector for one upstream movement: during the queue-service time `g_s` (clamped to `[0, g]`) the rate is the saturation flow rate; for the remainder of green the rate is set so the profile integrates exactly to `discharge_volume_veh_h` over the cycle (both intervals are discretized to whole steps first, then the post-queue-service rate is back-solved from the remaining vehicle count, avoiding continuous-vs-discrete rounding drift per the function's doc comment).

`UrbanSegment::step_3_proportion_arriving_green` (in `urban_segments.rs`) implements the mode switch: when `upstream_discharge_profiles` (`Option<Vec<MovementDischarge>>`) is supplied together with the segment's own running time (`running_time_s`, computed by Step 2) and the downstream green window (`downstream_green_start_s`, `effective_green_s`, `cycle_length_s`), the private helper `computed_proportion_arriving_green` builds the combined arrival profile via `combined_arrival_profile` and reads off `P` via `proportion_arriving_green`; otherwise Step 3 falls back to the milestone-1 behavior documented in `chapter18.md` (`P = R_p g/C` from a supplied platoon ratio or Exhibit 19-13 arrival type, or `P = g/C` for uniform arrivals). `arrival_uniform_volume_veh_h` (veh/h) and `flow_profile_time_step_s` (defaults to 1.0 s/step) are the two additional inputs specific to the computed path.

**Deferred / not reproducible.** Per the module doc comment on `platoon_dispersion.rs` and the VERIFICATION.md entry, Example Problem 1's published computed `P = 0.493` for the eastbound internal WB-through movement (Exhibit 30-32, only +0.007 above the uniform `g/C = 0.486`) requires the full Chapter 19 coordinated-actuated engine's discharge-flow profiles and the Chapter 30, Section 2 origin-destination distribution — neither is reproducible from the published intermediates alone (the through queue-service times print as 0.000 in Exhibit 30-33). The dispersion primitives themselves are unit-tested directly against the equations (see Validation), not against this published end-to-end `P` value.

## Section 4 — Delay due to turns at access points

The procedure computes the delay imposed on major-street through vehicles by same-direction left- and right-turning traffic at one unsignalized access-point approach, assuming random (unplatooned) segment flow, which the Chapter 30 text calls "conservative in that it will yield slightly larger estimates of delay."

| Component | Equation(s) | Rust function | File |
|---|---|---|---|
| Average vehicle spacing in queue | Eq. 30-15, `L_h = L_pc(1−0.01 P_HV) + 0.01 L_HV P_HV` | `stationary_queue_spacing` | `access_point_delay.rs` |
| Permitted left-turn capacity | Eq. 30-35, `c_l = v_o e^(−v_o t_cg/3600)/(1 − e^(−v_o t_fh/3600))` | `permitted_left_capacity` | `access_point_delay.rs` |
| Lane-change probability | Eq. 30-32/30-33, `P_lc = 1 − [(2v_app/s_lc) − 1]^2` | `probability_lane_change` | `access_point_delay.rs` |
| Left-turn bay overflow probability | Eq. 30-53/30-54, `p_ov = (v_lt/c_l)^(N_qx,lt+1)` | `probability_left_bay_overflow` | `access_point_delay.rs` |
| Lane split (P_L, P_R, v_1, v_n, v_2) | Eq. 30-36 through 30-46 | `lane_split` (private) | `access_point_delay.rs` |
| Incremental (randomized) queue delay | shared term of Eq. 30-48/30-51 | `incremental_delay` (private) | `access_point_delay.rs` |
| Delay due to left turns d_ap,l | Eq. 30-31 through 30-54 (merge capacity Eq. 30-47, merge/non-merge delay Eq. 30-48/30-51/30-52) | `access_point_through_delay` (left-turn branch, `I_t = 1.0`) | `access_point_delay.rs` |
| Through-vehicle delay per right-turn maneuver d_t\|r | Eq. 30-56 through 30-68 | `through_delay_per_right_turn` (private) | `access_point_delay.rs` |
| Delay due to right turns d_ap,r | Eq. 30-55 | `access_point_through_delay` (right-turn branch, `I_t = 0.00001`) | `access_point_delay.rs` |
| Total delay | `d_ap = d_ap,l + d_ap,r` | `access_point_through_delay` (public entry point) | `access_point_delay.rs` |

### Section 4 equations in full

```
Equation 30-15:  L_h = L_pc·(1 − 0.01·P_HV) + 0.01·L_HV·P_HV     [ft/veh]
  L_pc = stored passenger-car lane length      (ft, default 25.0)
  L_HV = stored heavy-vehicle lane length      (ft, default 45.0)
  P_HV = percent heavy vehicles in the movement group      (%, 0-100)
Implemented in: urban_segments/access_point_delay.rs::stationary_queue_spacing
```

```
Equation 30-32:  P_lc = 1 − [(2·v_app/s_lc) − 1]²  ≥ 0.0     [unitless]
Equation 30-33:  v_app = (v_lt + v_th + v_rt) / (N_sl + N_t + N_sr)     [veh/h/ln]
  s_lc = maximum flow rate at which a lane change can occur = 3,600/t_lc     (veh/h/ln)
  t_lc = critical merge headway = 3.7                                       (s)
  v_lt, v_th, v_rt = left-turn / through / right-turn demand flow rates     (veh/h)
  N_sl, N_t, N_sr  = lane counts: shared left-turn/through, exclusive through, shared right-turn/through   (ln)
Note: per the Chapter 30 text, if v_app/s_lc exceeds 1.0 it is capped at 1.0 before squaring.
Implemented in: urban_segments/access_point_delay.rs::probability_lane_change (v_app itself is computed inline in access_point_through_delay)
```

```
Equation 30-34:  E_L1 = 1,800 / c_l     [through-car-equivalents/veh] (used as 1.0 directly when a left-turn bay is present)
Equation 30-35:  c_l = v_o·e^(−v_o·t_cg/3,600) / (1 − e^(−v_o·t_fh/3,600))     [veh/h]
  v_o  = opposing demand flow rate (opposing through plus opposing right turn)     (veh/h)
  t_fh = follow-up headway for a permitted left turn = 2.2                        (s)
  t_cg = critical headway for a permitted left turn = 4.1                         (s)
Implemented in: urban_segments/access_point_delay.rs::permitted_left_capacity (Equation 30-35); the Equation 30-34 division (e_l1 = 1,800.0 / c_l) is inlined at the call site in access_point_through_delay
```

```
Equations 30-36 through 30-46: the lane split (grouped per the code's `lane_split` function; I_t = indicator, 1.0 for the left-turn branch, 0.00001 for the right-turn branch; I_lt, I_rt = 1.0 if no left-/right-turn bay else 0.0; E_R,ap = 2.20 if no right-turn bay else 1.0):

  Eq 30-36:  E_L1,m = (E_L1 − 1)·P_lc + 1                                    [through-car-equivalents]  modified permitted-left equivalent
  Eq 30-37:  E_R,m  = (E_R,ap − 1)·P_lc + 1                                  [through-car-equivalents]  modified protected-right equivalent
  Eq 30-41:  R = 1 + I_rt·P_rt·(E_R,m − 1)                                   [unitless]                 intermediate
  Eq 30-39:  b = R − I_lt·P_lt·{ I_t + (N_sl+N_t+N_sr−1)·[(1+I_t)·E_L1,m − 1] }   [unitless]             intermediate
  Eq 30-40:  c = −I_lt·P_lt·(N_sl+N_t+N_sr)                                  [unitless]                 intermediate
  Eq 30-38:  P_L = [ −b + √(b² − 4·I_t·R·c) ] / (2·I_t·R)  ≤ 1.0             [decimal]                  proportion of left turns in the inside lane
             (if N_sl+N_t+N_sr = 1, P_L = P_lt instead)
  Eq 30-43:  s_1 = 1,800·(1 + P_L·I_t) / (1 + P_L·(E_L1,m − 1) + P_L·E_L1,m·I_t)   [veh/h/ln]           inside-lane saturation flow rate
  Eq 30-42:  P_R = I_rt·P_rt·(s_1/1,800 + N_through − 1) / (1 − I_rt·P_rt·(s_1/1,800 + N_through − 2)·(E_R,m − 1))  ≤ 1.0   [decimal]   proportion of right turns in the outside lane
             (if N_through = 1, P_R = P_rt instead; N_through = N_sl+N_t+N_sr)
  Eq 30-44:  v_1 = v_lt / P_L                                                [veh/h/ln]                 inside-lane flow rate
  Eq 30-45:  v_n = v_rt/P_R  if P_R > 0.0;  else (v_lt+v_th+v_rt−v_1)/(N_through−1)   [veh/h/ln]         outside-lane flow rate
  Eq 30-46:  v_2 (v_i for intermediate lanes) = (v_lt+v_th+v_rt−v_1−v_n)/(N_through−2)  if N_through > 2;  else v_2 = v_n   [veh/h/ln]

  P_lt, P_rt = proportion of left-/right-turning vehicles on the approach (decimal) = v_lt/(v_lt+v_th+v_rt), v_rt/(v_lt+v_th+v_rt)
Implemented in: urban_segments/access_point_delay.rs::lane_split (private); returns the LaneSplit{p_l, p_r, v_1, v_n, v_2} struct
```

```
Equations 30-47 through 30-52: merge and non-merge capacity/delay for the inside lane, and the min-of-two-forms rule (T = analysis period duration, h):

  Eq 30-47:  c_mg = v_2·e^(−v_2·t_lc/3,600) / (1 − e^(−v_2·t_lc/3,600))     [veh/h]     merge capacity (t_lc = 3.7 s)
  Eq 30-49:  v_mg = max(v_1 − v_lt, 0.0)                                    [veh/h/ln]  merge flow rate
  Eq 30-48:  d_mg = 3,600·(1/c_mg − 1/1,800) + 900·T·[ v_mg/c_mg − 1 + √((v_mg/c_mg − 1)² + 8·v_mg/(c_mg²·T)) ]     [s/veh]   merge delay
  Eq 30-50:  c_nm = 1,800·(1 + P_L) / (1 + P_L·(E_L1 − 1) + P_L·E_L1)       [veh/h]     non-merge capacity (uses the unadjusted E_L1, not E_L1,m)
  Eq 30-51:  d_nm = 3,600·(1/c_nm − 1/1,800) + 900·T·[ v_1/c_nm − 1 + √((v_1/c_nm − 1)² + 8·v_1/(c_nm²·T)) ]        [s/veh]   non-merge delay
  Eq 30-52:  d_t,1 = min(d_nm, d_mg)                                        [s/veh]     delay to through vehicles in the inside lane

Code note: the 3,600(1/c − 1/1,800) + 900T[...] shape of Eq 30-48/30-51 is shared and factored into one helper; for a single-lane approach (no adjacent lane to merge into) the code uses only the non-merge form (d_t,1 = d_nm, computed with c_nm as above), since through vehicles there cannot merge by definition.
Implemented in: urban_segments/access_point_delay.rs::incremental_delay (private, the shared Eq 30-48/30-51 term) and the left-turn branch of access_point_through_delay (Eq 30-47, 30-49, 30-50, 30-52)
```

```
Equation 30-53:  p_ov = (v_lt / c_l)^(N_qx,lt + 1)     [decimal]
Equation 30-54:  N_qx,lt = N_lt·L_a,lt / L_h     [veh]  (= 0.0 for an undivided cross section / no left-turn bay)
  v_lt    = left-turn demand flow rate                                   (veh/h)
  c_l     = permitted left-turn capacity                                 (veh/h, Equation 30-35)
  N_lt    = number of lanes in the left-turn bay                         (ln)
  L_a,lt  = available left-turn bay storage distance                     (ft/ln)
  L_h     = average vehicle spacing in stationary queue                  (ft/veh, Equation 30-15)
Implemented in: urban_segments/access_point_delay.rs::probability_left_bay_overflow (Eq 30-53); N_qx,lt (Eq 30-54) is computed inline in access_point_through_delay
```

```
Equation 30-31:  d_ap,l = p_ov·d_t,1·(1/P_L − 1)·P_lt / (1 − P_lt − P_rt)     [s/veh]
  p_ov  = probability of left-turn bay overflow            (decimal, Equation 30-53)
  d_t,1 = delay to through vehicles in the inside lane      (s/veh, Equation 30-52)
  P_L   = proportion of left turns in the inside lane       (decimal, Equation 30-38)
  P_lt, P_rt = proportion of left-/right-turning vehicles on the approach   (decimal)
Implemented in: urban_segments/access_point_delay.rs::access_point_through_delay (left-turn branch, evaluated at I_t = 1.0)
```

```
Equation 30-55:  d_ap,r = 0.67·d_t|r·P_rt / (1 − P_lt − P_rt)     [s/veh]
  d_t|r = through-vehicle delay per right-turn maneuver     (s/veh, Equations 30-56 through 30-68)
  0.67  = field-data calibration constant (Chapter 30 §4 text)
  P_lt, P_rt = proportion of left-/right-turning vehicles on the approach   (decimal)
Code note: a right-turn bay short-circuits d_ap,r to 0.0 s/veh (the Exhibit 18-13 "both bays adequate ⇒ 0.0" convention), since the printed §4 equations have no explicit bay term for this case.
Implemented in: urban_segments/access_point_delay.rs::access_point_through_delay (right-turn branch, evaluated at I_t = 0.00001, forced P_lc = 1.0)
```

```
Equations 30-56 through 30-60: minimum speed and conditional delay to the first delayed through vehicle (λ from Eq 30-59; the h̄ function is shared by Eq 30-57/30-62/30-65):

  Eq 30-59:  λ = 1 / (1/q_n − Δ)                                                          [veh/s]   flow-rate parameter
  Eq 30-57:  h̄|Δ<h<H_1 = 1/λ + (Δ − H_1·e^(−λ(H_1−Δ))) / (1 − e^(−λ(H_1−Δ)))              [s/veh]   mean headway between Δ and H_1
  Eq 30-58:  H_1 = (1.47·S_f − u_rt)/r_d + t_cl + L_h/(1.47·S_f)  ≥ Δ                      [s/veh]   max headway for the first vehicle to still be delayed
  Eq 30-56:  u_m = max( 1.47·S_f − r_d·(H_1 − h̄|Δ<h<H_1),  u_rt )                         [ft/s]    minimum speed of the delayed first through vehicle
  Eq 30-60:  d_1 = [ (1.47·S_f − u_m)² / (2·1.47·S_f) ] · (1/r_d + 1/r_a)                  [s/veh]   conditional delay to the first through vehicle

  q_n   = outside-lane flow rate = v_n/3,600                              (veh/s)
  S_f   = approaching through-vehicle speed ("free-flow speed" per the printed text — see "The posted-speed-vs-FFS finding" above)   (mi/h)
  u_rt  = right-turn speed = 20.0                                         (ft/s)
  r_d   = deceleration rate = 6.7                                         (ft/s²)
  r_a   = acceleration rate = 3.5                                         (ft/s²)
  t_cl  = clearance time of the right-turn vehicle = 0.6                  (s)
  L_h   = average vehicle spacing in stationary queue (Equation 30-15)    (ft/veh)
  Δ     = headway of the bunched vehicle stream = 1.5                     (s/veh)
  1.47  = mi/h → ft/s conversion factor

Equation 30-60 grouping: see "The Eq 30-60 grouping finding" above — the printed (1/r_d + 1/r_a) term is a multiplier on the squared-speed-deficit fraction, not its denominator, confirmed from the raw MathML `<mfrac>` structure.
Implemented in: urban_segments/access_point_delay.rs::through_delay_per_right_turn (private)
```

```
Equations 30-61 through 30-68: geometric-decay sum of delay to the second and subsequent through vehicles, iterated (in code) up to 50 terms or until an individual d_i falls below 0.1 s:

  Eq 30-63 (i=2) / Eq 30-66 (i≥3):  H_i = d_(i−1) + Δ                                                          [s/veh]
  Eq 30-62 (i=2) / Eq 30-65 (i≥3):  h̄|Δ<h<H_i = 1/λ + (Δ − H_i·e^(−λ(H_i−Δ))) / (1 − e^(−λ(H_i−Δ)))            [s/veh]
  Eq 30-61 (i=2) / Eq 30-64 (i≥3):  d_i = d_(i−1) − (h̄|Δ<h<H_i − Δ)                                            [s/veh]
  Eq 30-67 (closed form, first two vehicles) / Eq 30-68 (general form, any number of vehicles):
    d_t|r = Σ_{i=1}^{∞} [ d_i × Π_{j=1}^{i} (1 − e^(−λ(H_j−Δ))) × (1 − P_R)^i ]                                 [s/veh]

  d_i    = conditional delay to through vehicle i (i = 1 is Equation 30-60's d_1)     (s/veh)
  λ, Δ   = as in the Eq 30-56 through 30-60 block above
  P_R    = proportion of right turns in the outside lane                              (decimal, Equation 30-42)
Implemented in: urban_segments/access_point_delay.rs::through_delay_per_right_turn (private)
```

`access_point_through_delay(ap: &AccessPointApproach, speed_mph: f64, analysis_period_h: f64) -> AccessPointDelay` is the single public entry point; `AccessPointApproach` carries the per-approach geometry and turning volumes (`v_lt`/`v_th`/`v_rt` veh/h, `n_sl`/`n_t`/`n_sr` lane counts, `opposing_flow_veh_h` veh/h, `left_turn_bay`/`right_turn_bay` bools, `n_lt_lanes`, `left_bay_storage_ft` ft, `pct_heavy_veh` %); `AccessPointDelay` returns `delay_left_s`, `delay_right_s`, `delay_total_s` (all s/veh) and `prob_inside_lane_blocked` (p_ov, unitless). The left-turn branch computes the modified through-car equivalent `E_L1` from the permitted-left capacity (1.0 when a left-turn bay is present), evaluates the lane split at `I_t = 1.0`, computes the merge capacity/delay (Eq. 30-47 through 30-49) and non-merge capacity/delay (Eq. 30-50/30-51), takes the lesser (Eq. 30-52), and scales by the bay-overflow probability and turning proportions (Eq. 30-31). The right-turn branch re-solves the lane split at a near-zero `I_t` (`0.00001`, matching the printed indicator convention for that branch) and forced `P_lc = 1.0`, then calls `through_delay_per_right_turn`, which implements the full first-vehicle delay (Eq. 30-56 through 30-60) and the geometric-decay sum over subsequent delayed vehicles (Eq. 30-61 through 30-68, iterated up to 50 terms or until an individual delay term drops below 0.1 s) — a right-turn bay short-circuits this branch to 0.0 s/veh per the Exhibit 18-13 "both bays adequate ⇒ 0.0" convention, since the code's docstring notes the printed §4 equations have no explicit bay term for this case.

`UrbanSegment` wires this into Step 2 (`step_2_running_time` in `urban_segments.rs`) as the highest-priority of a three-way mode switch on the `Σ d_ap,i` term: (1) computed, when `access_point_approaches: Option<Vec<AccessPointApproach>>` is supplied — each approach is evaluated via `access_point_through_delay` at `access_point_turn_delay_speed_mph.unwrap_or(speed_limit_mph)` and `analysis_period_h`, and results are stored on `access_point_delays_computed: Option<Vec<AccessPointDelay>>`; (2) input, when `access_point_delays_s: Option<Vec<f64>>` is supplied instead (summed directly); (3) the Exhibit 18-13 planning estimate (milestone 1, `chapter18.md`) otherwise.

### The posted-speed-vs-FFS finding

Equations 30-56 and 30-58 define the approaching through-vehicle speed used in the right-turn delay chain as "S_f = free-flow speed" in the printed text. Reproducing Example Problem 1's published per-access-point delay (0.193/0.194 s/veh, AP1 eastbound/westbound, Exhibit 30-35) and the published inside-lane blockage probability (0.115) requires evaluating the right-turn branch at the **posted speed limit** (35 mi/h) rather than the segment's computed free-flow speed (39.33 mi/h); using the free-flow speed instead yields 0.217 s/veh, about 12% high. `p_ov` and `d_ap,l` are independent of this speed and reproduce the published values regardless of which speed is used. The code defaults `access_point_turn_delay_speed_mph` to the posted speed limit and documents this as a `VERIFY-HCM` item (both in the `access_point_delay.rs` module doc comment and inline on `through_delay_per_right_turn`'s call site in `urban_segments.rs`), cross-referenced in `docs/hcm/VERIFICATION.md` under "Chapter 18/30 computed procedures."

### The Eq 30-60 grouping finding

Equation 30-60 (conditional delay to the first through vehicle, `d1`) is printed with a fraction `(S_f − u_m)^2/(2·1.47 S_f)` immediately followed by a parenthesized `(1/r_d + 1/r_a)` term. A plausible OCR flattening of the printed layout would read the parenthesized term as a **denominator** of the fraction; the code instead reads it as a **multiplier** — `d1 = [(v − u_m)^2/(2v)] × (1/r_d + 1/r_a)`, implemented literally as such in `through_delay_per_right_turn`. Reading it as a denominator instead inflates `d1` roughly 5x and the resulting `d_ap,r` roughly 12x, which does not reproduce the published Exhibit 30-35 values. The module doc comment states the MathML source (`<mfrac>...</mfrac><mrow>(...)</mrow>`) confirms the multiplier reading directly from the EPUB markup structure, i.e., this was resolved from the source markup rather than purely from numerical reproduction. Cross-referenced in `docs/hcm/VERIFICATION.md`.

### Computed-vs-input mode switching

Both Section 3 and Section 4 follow the same pattern used elsewhere in Chapter 18: a milestone-1 analyst-supplied-input or planning-estimate path remains the default, and the milestone-2 computed path is opt-in by supplying the richer input (`upstream_discharge_profiles` for Section 3; `access_point_approaches` for Section 4). Neither computed path is silently preferred if its required inputs are partially missing — `computed_proportion_arriving_green` returns `None` (falling through to the milestone-1 path) if `running_time_s` (Step 2 must have already run) or the green-window inputs are absent, and the Section 4 switch is a plain `if let Some(...)` / `else if let Some(...)` / `else` chain in `step_2_running_time` with no partial-credit blending between the three levels.

## Validation

The primary fixture remains HCM Chapter 30, Example Problem 1 (Exhibits 30-26 through 30-36), reused from `chapter18.md`'s milestone-1 validation but extended with a third fixture, `tests/ExampleCases/hcm/UrbanSegments/case3.json`, which is identical to `case1.json` except that `access_point_delays_s` (the milestone-1 input hook) is replaced by `access_point_approaches` (two active access points, AP1 and AP2, with their turning volumes, opposing flow, and undivided/no-bay geometry taken from Exhibit 30-35). `tests/chapter18_integration.rs::test_case3_example_problem_1_computed_access_point_delay` asserts: the computed per-access-point `delay_total_s` for AP1 and AP2 match the published 0.193 and 0.194 s/veh at ±0.001; and every downstream Step 2-10 performance measure (base FFS, running time, running speed, travel speed, v/c, LOS) reproduces the same published Exhibit 30-36 values as `case1.json`, at the same tolerances documented in `chapter18.md` (e.g., ±0.01 s running time, ±0.01 mi/h travel speed, exact LOS).

Unit tests in `src/hcm/urban_segments/tests.rs` cover the equations directly: `test_equation_30_15_queue_spacing`, `test_equation_30_35_permitted_left_capacity`, `test_equation_30_32_probability_lane_change`, and `test_equation_30_53_overflow_probability` at tight (≤1e-9 to 1e-12) tolerances on closed-form arithmetic; `test_access_point_delay_example_problem_1` reproduces the published AP1 eastbound/westbound delay and blockage probability (±0.001) and additionally asserts the free-flow-speed alternative (0.217 s/veh, ±0.002) as a documented sensitivity check, not a pass/fail regression; `test_access_point_delay_single_lane` and `test_access_point_delay_turn_bays` are synthetic sanity checks (finite/non-negative delay for a single-lane approach; near-zero delay and near-zero overflow probability when both turn bays are adequate). For Section 3: `test_equation_30_11_smoothing_factor` and `test_equation_30_12_platoon_arrival_time` check the two closed-form equations against Example Problem 1's own running time (33.54 s) at ±0.0001-0.005; `test_equation_30_9_dispersion_conserves_and_smooths`, `test_proportion_arriving_green_uniform`, `test_equation_30_13_proportion_time_blocked`, and `test_critical_platoon_flow_rate` are synthetic checks of the dispersion recursion's conservation property and the two remaining closed-form equations; `test_movement_discharge_profile_integrates_to_volume` and `test_combined_arrival_profile_and_P` are synthetic checks that the discharge-profile constructor integrates exactly to the input volume and that the combined-profile-to-`P` pipeline runs end to end. `test_serde_round_trip` covers `AccessPointApproach`/`MovementDischarge` serde symmetry. There is no unit or integration test that reproduces the published `P = 0.493` end-to-end value, consistent with the documented "not reproducible from the published intermediates alone" finding above.

`tests/test_chapter18_integration.py` was not extended with a `case3` Python-binding test as of this branch (it still exercises only `case1.json`, per `chapter18.md`).

## Deferred

Per the `platoon_dispersion.rs` module doc comment: driving Section 3 for a full coordinated system requires the upstream signal's phase durations, saturation flows, and queue service times from the Chapter 19 coordinated-actuated engine, together with the Chapter 30, Section 2 origin-destination distribution; this full wiring (which would let Step 3 reproduce Example Problem 1's published `P = 0.493` from raw signal inputs rather than from an analyst-supplied discharge profile) is deferred. The Chapter 30, Section 2 demand-adjustment procedure itself (origin-destination estimation, volume balancing, spillback checks) remains deferred per `chapter18.md`. No new pedestrian/bicycle/transit scope is added by this branch.

The companion Section 3 output — the *proportion of time blocked* p_b consumed by Chapter 20 TWSC Step 5b — is now wired on the `feat/hcm-ch20-computed-pb` branch: `src/hcm/twsc/computed_pb.rs` reuses these same dispersion primitives (`combined_arrival_profile`, plus the `blocked_period_steps` / q_c = 3,600/t_c blocked-period logic) to build the TWSC `PlatoonBlockage` from upstream-signal descriptors (Equation 30-13). See `docs/hcm/procedures/chapter20.md`, "Step 5b input: computed proportion of time blocked". The same "not reproducible from published intermediates alone" caveat applies to its end-to-end p_b values (0.170 / 0.260 in Chapter 32 Exhibit 32-12, from Chapter 30 Example Problem 1); that module is validated by mechanism tests rather than a published-target regression.
