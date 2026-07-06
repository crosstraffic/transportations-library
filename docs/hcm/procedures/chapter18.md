# Chapter 18 — Urban Street Segments

This document walks through the Rust translation of HCM 7th Edition Chapter 18, Section 3 ("Motorized Vehicle Mode"), which is the computational core evaluated per direction of travel for one urban street segment, together with the Exhibit 18-1 LOS criteria of Section 2 and the Equations 18-17 through 18-22 automobile traveler perception score. The implementation lives in `src/hcm/urban_segments/urban_segments.rs` (the `UrbanSegment` struct and its ten-step pipeline) and `src/hcm/urban_segments/exhibits.rs` (Exhibit 18-1, 18-7, 18-11, and 18-13 lookups), with `src/hcm/urban_segments/mod.rs` re-exporting both modules. The module doc comment states this covers "milestone 1": per-direction segment evaluation with analyst-supplied boundary-intersection performance inputs, while the Chapter 30 supplemental procedures for demand balancing (Section 2), platoon dispersion (Section 3), and access-point delay (Section 4) are deferred to a milestone 2, and the pedestrian/bicycle/transit methodologies of Chapter 18 are out of scope entirely for this branch. Boundary-intersection outputs (through control delay, through capacity, and full stop rate) are treated as "HCM method output" inputs per Exhibit 18-5, meaning this module does not itself run the Chapter 19 (signalized), 21 (AWSC), 20 (TWSC), or 22 (roundabout) engines — it expects their results to be supplied through the `UrbanSegment` fields described below, though `shared_lane_through_delay` (Equation 18-10) is provided to help an analyst combine per-lane-group delay outputs from those engines before handing the segment its `through_control_delay_s` input.

## Step-by-step walkthrough

Step 1, "Determine Traffic Demand Adjustments," is implemented by `UrbanSegment::step_1_demand_adjustment` in `urban_segments.rs`. It is a simplified version of the manual step: flow rates are supplied by the analyst rather than balanced by the module, and the function's only computation is a capacity-constraint check that flags but does not meter demand in excess of capacity. It reads `through_capacity_veh_h` (`Option<f64>`, veh/h) and `through_demand_veh_h` (`f64`, veh/h) and writes `demand_exceeds_capacity` (`Option<bool>`); it also returns the midsegment flow rate `v_m` (veh/h) computed by `midsegment_flow_rate`, which is `midsegment_flow_veh_h` when supplied or else defaults to `through_demand_veh_h` per the Exhibit 18-5 default. The full Chapter 30, Section 2 origin–destination, volume-balance, and spillback-check procedure is out of scope here; the module docs are explicit that analysts must supply already-balanced demand flow rates.

```
Step 1 has no dedicated HCM equation number — the manual's Section 2 demand-adjustment procedure (origin-destination estimation, volume balancing, spillback checks) is deferred, and this step is only the capacity-constraint flag plus the Exhibit 18-5 midsegment-flow default:
  demand_exceeds_capacity = c_th.is_some() AND c_th > 0 AND v_th > c_th     [bool, None if c_th absent]
  v_m = midsegment_flow_veh_h if supplied, else v_th                       [veh/h]  (Exhibit 18-5 default)
    v_th = through_demand_veh_h, through-demand flow rate at the downstream boundary intersection   (veh/h)
    c_th = through_capacity_veh_h, through-movement capacity at the downstream boundary intersection (veh/h)
    v_m  = midsegment demand flow rate, all movements traveling along the segment                    (veh/h)
  Note: the flag is descriptive only — demand in excess of capacity is not metered (Chapter 30 §2 volume-balance procedure deferred).
Implemented in: urban_segments/urban_segments.rs::UrbanSegment::step_1_demand_adjustment
```

Step 2, "Determine Running Time," is the largest step and is implemented by `UrbanSegment::step_2_running_time` in `urban_segments.rs`, which orchestrates several helper functions from `exhibits.rs` and `urban_segments.rs`. The base free-flow speed (Equation 18-3, `S_fo = S_calib + S_0 + f_CS + f_A + f_pk`) is assembled from `speed_constant_s0` (Exhibit 18-11 note a, `S_0 = 25.6 + 0.47 S_pl` from `speed_limit_mph`, mi/h), `cross_section_adjustment` (Exhibit 18-11 note b, `f_CS`, mi/h, from `proportion_restrictive_median()` and `proportion_with_curb`), `access_point_adjustment` (Exhibit 18-11 note c, `f_A`, mi/h, driven by `access_point_density` — points/mi — computed from `n_access_points_subject`, `n_access_points_opposing`, `segment_length_ft`, and `upstream_intersection_width_ft`), and `parking_adjustment` (Exhibit 18-11 note d, `f_pk`, mi/h, from `proportion_on_street_parking`); all four helpers live in `exhibits.rs`. The signal-spacing adjustment (Equation 18-4, `f_L = 1.02 − 4.7(S_fo − 19.5)/max(L_s, 400) ≤ 1.0`) is `signal_spacing_adjustment` in `urban_segments.rs`, taking `base_ffs_mph` (mi/h) and `signal_spacing_ft` (ft, defaulting to `segment_length_ft`) and returning the dimensionless factor `f_L`. Free-flow speed (Equation 18-5, `S_f = max(S_fo f_L, S_pl)`) is computed inline in `step_2_running_time`, mi/h, and is overridden entirely by `free_flow_speed_override_mph` (mi/h) when the analyst supplies a field-measured value. Vehicle proximity (Equation 18-6, `f_v = 2/(1 + (1 − v_m/(52.8 N_th S_f))^0.21)`) is `proximity_adjustment` in `urban_segments.rs`, taking midsegment demand `v_m` (veh/h), `n_through_lanes` (ln), and `S_f` (mi/h) and returning the dimensionless factor `f_v`; the ratio inside the exponent is clamped to `[0, 0.999999]` to keep the exponentiation real for out-of-range demand, a defensive clamp not explicit in the manual text. Delay due to turning vehicles at access points is either the sum of an analyst-supplied `access_point_delays_s` (`Vec<f64>`, s/veh per access point — the Chapter 30, Section 4 procedure output) or, when absent, the Exhibit 18-13 planning estimate via `exhibit_18_13_turn_delay_adjusted` in `exhibits.rs`, which looks up `exhibit_18_13_turn_delay` (s/veh/pt, by midsegment volume per lane and lane count) and applies the turn-percentage and turn-bay adjustments described in the Chapter 18 text (percentage scaling by `(pct_left + pct_right)/20`, and a 0.5/0.0 multiplier for one/two adequate turn bays). Segment running time itself (Equations 18-7 and 18-8, `t_R = (6 − l1)/(0.0025 L) f_X + (3600 L)/(5280 S_f) f_v + Σd_ap,i + d_other`) is computed inline: start-up lost time `l1` and the first-term multiplier `f_X` are chosen by `control` (2.0 s / 1.0 for `Signalized`, 2.5 s / 1.0 for `AllWayStop`, 2.5 s / v/c-capped-at-1.0 for `YieldControlled`/`Roundabout`, 2.5 s / 0.0 for `Uncontrolled`), and `midsegment_other_delay_s` (s/veh) is added as `d_other`. The step writes `speed_constant_mph`, `f_cs_mph`, `f_a_mph`, `f_pk_mph`, `base_ffs_mph`, `f_l`, `free_flow_speed_mph`, `f_v`, `access_point_delay_total_s`, `running_time_s` (t_R, s), and `running_speed_mph` (mi/h, `= 3,600 L/(5,280 t_R)`, per the Chapter 18 Step 2 discussion of Exhibit 18-12). Two spec gaps are cross-referenced to the VERIFICATION.md Chapter 18 entry: `exhibit_18_13_turn_delay` clamps its midsegment-volume input to 200–700 veh/h/ln because the exhibit is undefined outside that range, and `access_point_density`/related lookups have no analogous manual guidance for degenerate (zero or negative) link lengths beyond the `link <= 0.0 → 0.0` guard in the code.

```
Equation 18-3:  S_fo = S_calib + S_0 + f_CS + f_A + f_pk     [mi/h]
  S_fo    = base free-flow speed                                                  (mi/h)
  S_calib = base free-flow speed calibration factor (default 0.0; field-calibrated per Chapter 30 §6) (mi/h)
  S_0     = speed constant (Exhibit 18-11, note a)                                (mi/h)
  f_CS    = adjustment for cross section (Exhibit 18-11, note b)                  (mi/h)
  f_A     = adjustment for access points (Exhibit 18-11, note c)                  (mi/h)
  f_pk    = adjustment for on-street parking (Exhibit 18-11, note d)              (mi/h)

  Exhibit 18-11, note a:  S_0 = 25.6 + 0.47·S_pl     [mi/h]
    S_pl = posted speed limit                                                    (mi/h)
  Implemented in: urban_segments/exhibits.rs::speed_constant_s0

  Exhibit 18-11, note b:  f_CS = 1.5·p_rm − 0.47·p_curb − 3.7·p_curb·p_rm     [mi/h]
    p_rm   = proportion of link length with a restrictive median                 (decimal)
    p_curb = proportion of segment with curb on the right-hand side, within 4 ft of the traveled way (decimal; Exhibit 18-5 default 1.0)
  Implemented in: urban_segments/exhibits.rs::cross_section_adjustment (p_rm via urban_segments.rs::UrbanSegment::proportion_restrictive_median)

  Exhibit 18-11, note c:  D_a = 5,280·(N_ap,s + N_ap,o) / (L − W_i)     [points/mi]
                          f_A = −0.078·D_a / N_th                       [mi/h]
    N_ap,s = access point approaches on the right side, subject direction of travel (points)
    N_ap,o = access point approaches on the right side, opposing direction of travel (points)
    L      = segment length                                                      (ft)
    W_i    = width of the upstream signalized intersection                       (ft)
    N_th   = number of through lanes on the segment in the subject direction     (ln)
  Implemented in: urban_segments/exhibits.rs::access_point_density, urban_segments/exhibits.rs::access_point_adjustment

  Exhibit 18-11, note d:  f_pk = −3.0 · p_pk     [mi/h]
    p_pk = proportion of link length with on-street parking available on the right-hand side (decimal)
  Implemented in: urban_segments/exhibits.rs::parking_adjustment
Implemented in: urban_segments/urban_segments.rs::UrbanSegment::step_2_running_time

Equation 18-4:  f_L = 1.02 − 4.7·(S_fo − 19.5) / max(L_s, 400)  ≤ 1.0     [unitless]
  S_fo = base free-flow speed (Eq. 18-3)                                          (mi/h)
  L_s  = distance between the two intersections that bracket the subject segment and can legally require the through movement to stop or yield; defaults to segment_length_ft (ft)
Implemented in: urban_segments/urban_segments.rs::signal_spacing_adjustment

Equation 18-5:  S_f = max(S_fo · f_L, S_pl)     [mi/h]
  S_fo = base free-flow speed (Eq. 18-3)                                          (mi/h)
  f_L  = signal spacing adjustment factor (Eq. 18-4)                              (unitless)
  S_pl = posted speed limit                                                       (mi/h)
  Note: overridden entirely by free_flow_speed_override_mph when the analyst supplies a field-measured value (Chapter 30 §6 field procedure).
Implemented in: urban_segments/urban_segments.rs::UrbanSegment::step_2_running_time (inline)

Equation 18-6:  f_v = 2 / (1 + (1 − v_m/(52.8·N_th·S_f))^0.21)     [unitless]
  v_m  = midsegment demand flow rate                                              (veh/h)
  N_th = number of through lanes in the subject direction of travel               (ln)
  S_f  = free-flow speed (Eq. 18-5)                                               (mi/h)
  Note: the ratio inside the exponent is clamped to [0, 0.999999] in code, a defensive clamp not explicit in the manual text (VERIFICATION.md Chapter 18 entry).
Implemented in: urban_segments/urban_segments.rs::proximity_adjustment

Equation 18-7:  t_R = (6.0 − l1)/(0.0025·L)·f_X + (3,600·L)/(5,280·S_f)·f_v + Σ_{i=1}^{N_ap} d_ap,i + d_other     [s]
  l1      = start-up lost time: 2.0 if signalized; 2.5 if STOP or YIELD controlled                     (s)
  L       = segment length                                                                              (ft)
  f_X     = control-type adjustment factor (Eq. 18-8)                                                   (unitless)
  S_f     = free-flow speed (Eq. 18-5)                                                                  (mi/h)
  f_v     = proximity adjustment factor (Eq. 18-6)                                                      (unitless)
  d_ap,i  = delay due to left and right turns from the street into access point intersection i (analyst-supplied, computed via the Chapter 30 §4 procedure, or the Exhibit 18-13 planning estimate — see the discussion above; the Exhibit 18-13 lookup clamps its midsegment-volume input to 200-700 veh/h/ln, a documented spec gap) (s/veh)
  N_ap    = number of influential access point approaches = N_ap,s + p_ap,lt·N_ap,o                     (points)
  d_other = delay due to other midsegment sources (e.g., curb parking, pedestrians)                      (s/veh)
Implemented in: urban_segments/urban_segments.rs::UrbanSegment::step_2_running_time (inline)

Equation 18-8:  f_X = 1.00                     if signalized or STOP-controlled through movement
                    = 0.00                     if uncontrolled through movement
                    = min(v_th/c_th, 1.00)      if YIELD-controlled through movement     [unitless]
  v_th = through-demand flow rate                                                        (veh/h)
  c_th = through-movement capacity                                                       (veh/h)
  Code detail: (l1, f_X) pairs by `control` are Signalized (2.0, 1.0), AllWayStop (2.5, 1.0) — both fall under the manual's "signalized or STOP-controlled" f_X = 1.00 case, differing only in l1 per the Eq. 18-7 where-clause — YieldControlled/Roundabout (2.5, min(v_th/c_th, 1.0), defaulting to 1.0 when capacity is unknown), Uncontrolled (2.5, 0.0).
Implemented in: urban_segments/urban_segments.rs::UrbanSegment::step_2_running_time (inline)
```

Step 3, "Determine the Proportion of Vehicles Arriving During Green," is `UrbanSegment::step_3_proportion_arriving_green` in `urban_segments.rs`. It applies HCM Equation 19-15, `P = R_p g/C`, using `effective_green_s` (g, s) and `cycle_length_s` (C, s), with the platoon ratio `R_p` taken from `platoon_ratio` (`Option<f64>`) if supplied, else mapped from `arrival_type` (`Option<u8>`, 1–6) via `platoon_ratio_for_arrival_type` in `crate::hcm::signalized::exhibits` (the Exhibit 19-13 arrival-type-to-platoon-ratio mapping), else defaulting to `R_p = 1.0` (uniform arrivals, `P = g/C`) per the Chapter 18 Step 3 text for a noncoordinated upstream intersection. It returns `None` — and leaves `proportion_arriving_green` as `None` — when `control` is not `Signalized` or when `effective_green_s`/`cycle_length_s` are absent. The result `P` (decimal, 0–1, capped at 1.0) is stored in `proportion_arriving_green`; the Chapter 30, Section 3 platoon-dispersion arrival-profile procedure for coordinated systems, which would otherwise supply `P` directly, is deferred to milestone 2.

```
Equation 19-15:  P = R_p · g / C     [decimal, 0-1]
  R_p = platoon ratio: from `platoon_ratio` if supplied, else the Exhibit 19-13 arrival-type mapping (`arrival_type`, 1-6), else 1.0 (uniform arrivals) per the Chapter 18 Step 3 text for a noncoordinated upstream intersection (unitless)
  g   = effective green time for the phase serving the through movement at the downstream signal   (s)
  C   = cycle length at the downstream signal                                                       (s)
  Note: P is capped at 1.0 in code; the step returns None entirely when `control` is not Signalized or when g/C are absent.
Implemented in: urban_segments/urban_segments.rs::UrbanSegment::step_3_proportion_arriving_green
```

Step 4, "Determine Signal Phase Duration," is not implemented as a computation. `cycle_length_s` and `effective_green_s` are plain analyst-supplied input fields on `UrbanSegment`; the module docs direct the analyst to the Chapter 19 pretimed/coordinated timing engine to obtain them, and note that the actuated average-phase-duration loop is deferred along with it.

```
Step 4 has no HCM equation implemented here — the manual directs pretimed signals to use an input phase duration directly, and actuated signals to the Section 3 (Chapter 19) average-phase-duration procedure, iterating Steps 1-4 to convergence when the boundary intersections are coordinated. This module takes `cycle_length_s` (C, s) and `effective_green_s` (g, s) as plain analyst-supplied fields rather than computing or iterating them; the Chapter 19 actuated timing engine and the Steps 1-4 convergence loop are deferred to milestone 2.
Implemented in: (input fields only) urban_segments/urban_segments.rs::UrbanSegment::cycle_length_s, ::effective_green_s
```

Step 5, "Determine Through Delay," is `UrbanSegment::step_5_through_delay` in `urban_segments.rs`. For any controlled boundary it passes through `through_control_delay_s` (`Option<f64>`, s/veh, defaulting to 0.0 if absent) as the through delay `d_t`; for `BoundaryControlType::Uncontrolled` it hard-codes `d_t = 0.0` s/veh per the Chapter 18 text (the major-street through movement at a TWSC boundary intersection is uncontrolled). The result is written to `through_delay_s`. As noted above, this is where the Chapter 19/20/21/22 signalized/unsignalized delay engines are expected to feed in — `through_control_delay_s` is documented as "HCM method output" — and `shared_lane_through_delay` (Equation 18-10, `d_t = (d_th v_t N_t + d_sl v_sl(1 − P_L) + d_sr v_sr(1 − P_R))/v_th`) is available as a standalone helper to combine exclusive-through and shared-lane delay outputs from those engines into the single `through_control_delay_s` scalar before it is assigned. Delay imposed on the major street by turning traffic at a TWSC boundary is explicitly not modeled in this step (per the module docs it "appears midsegment via the access-point delay terms" from Step 2 instead).

```
Step 5 pass-through (no dedicated HCM equation number for the step itself — control delay is the "HCM method output" of Chapters 19/21/22, or 0.0 s/veh for an uncontrolled through movement per the Chapter 18 text):
  d_t = 0.0                                          if control == Uncontrolled
      = through_control_delay_s, default 0.0         otherwise     [s/veh]
Implemented in: urban_segments/urban_segments.rs::UrbanSegment::step_5_through_delay

Equation 18-10:  d_t = (d_th·v_t·N_t + d_sl·v_sl·(1 − P_L) + d_sr·v_sr·(1 − P_R)) / v_th     [s/veh]
  d_th, v_t, N_t = delay (s/veh), demand flow rate per lane (veh/h/ln), and lane count of the exclusive-through lane group
  d_sl, v_sl, P_L = delay (s/veh), demand flow rate (veh/h), and proportion of left-turning vehicles in the shared left-turn/through lane group
  d_sr, v_sr, P_R = same for the shared right-turn/through lane group
  v_th = through-demand flow rate                                                                        (veh/h)
  Note: a standalone helper for an analyst to combine per-lane-group delay outputs from the Chapter 19/21/22 engines into the single through_control_delay_s scalar before Step 5 reads it; not called by step_5_through_delay itself.
Implemented in: urban_segments/urban_segments.rs::shared_lane_through_delay
```

Step 6, "Determine Through Stop Rate," is `UrbanSegment::step_6_stop_rate` in `urban_segments.rs`. If `full_stop_rate_override` (`Option<f64>`, stops/veh) is set, it is returned directly. Otherwise, for `Signalized` control the step evaluates Equation 18-11 via `full_stop_rate_signalized` in `urban_segments.rs`, `h = 3,600[N_f/(min(1, v_th C/(N_th s g)) g s) + N_th Q_2+3/(v_th C)]`, using `stopped_vehicles_veh_ln` (N_f, veh/ln), `queue2_veh_ln` + `queue3_veh_ln` (Q_2, Q_3, veh/ln, summed to Q_2+3), `effective_green_s` (g, s), `sat_flow_veh_h_ln` (s, veh/h/ln), `through_demand_veh_h` (v_th, veh/h), `cycle_length_s` (C, s), and `n_through_lanes` (N_th, ln); it returns `None` if any required Chapter 31 input is missing. For `AllWayStop` it returns 1.0 stops/veh; for `Uncontrolled` it returns 0.0 stops/veh; for `YieldControlled` and `Roundabout` it returns the through-movement v/c ratio (`through_demand_veh_h / through_capacity_veh_h`, or `None` if capacity is unavailable) — all per the Chapter 18 text defaults documented in the code. When the through movement is served across multiple lane groups, `weighted_through_lane_value` (Equations 18-12, 18-13, and 18-14, a shared per-lane weighting formula for N_f, adjusted saturation flow rate, and back-of-queue size) is provided as a standalone helper to compute the weighted N_f/s/Q_2+3 inputs before calling `full_stop_rate_signalized`. The result is written to `full_stop_rate` (h, stops/veh).

```
Equation 18-11:  h = 3,600·[ N_f / (min(1, v_th·C/(N_th·s·g))·g·s) + N_th·Q_2+3/(v_th·C) ]     [stops/veh]
  N_f    = number of fully stopped vehicles (Chapter 31 §4 output; Eq. 18-12 weighting for multiple lane groups)  (veh/ln)
  Q_2+3  = Q_2 + Q_3, second- plus third-term back-of-queue size (Chapter 31 output; Eq. 18-14 weighting)          (veh/ln)
  g      = effective green time                                                                                   (s)
  s      = adjusted saturation flow rate of the through lane group (Eq. 18-13 weighting)                          (veh/h/ln)
  v_th   = through-demand flow rate                                                                               (veh/h)
  C      = cycle length                                                                                           (s)
  N_th   = number of through lanes, shared or exclusive                                                           (ln)
Implemented in: urban_segments/urban_segments.rs::full_stop_rate_signalized

Equations 18-12 / 18-13 / 18-14 (shared per-lane weighting formula):
  x = (x_t·N_t + x_sl·(1 − P_L) + x_sr·(1 − P_R)) / N_th
    Eq. 18-12:  x = N_f (number of fully stopped vehicles)                (veh/ln)
    Eq. 18-13:  x = s (adjusted saturation flow rate)                     (veh/h/ln)
    Eq. 18-14:  x = Q_2+3 (back-of-queue size)                            (veh/ln)
  x_t, N_t = per-lane value and lane count of the exclusive-through lane group
  x_sl, P_L = per-lane value of the shared left/through lane group and proportion of left-turning vehicles in it
  x_sr, P_R = same for the shared right/through lane group
  N_th = number of through lanes, shared or exclusive                     (ln)
Implemented in: urban_segments/urban_segments.rs::weighted_through_lane_value

Unsignalized/YIELD defaults (Chapter 18 text, no equation number):
  h = 1.0            AllWayStop
    = 0.0            Uncontrolled
    = v_th / c_th    YieldControlled | Roundabout (returns None if c_th is unavailable)     [stops/veh]
Implemented in: urban_segments/urban_segments.rs::UrbanSegment::step_6_stop_rate
```

Step 7, "Determine Travel Speed," is `UrbanSegment::step_7_travel_speed` in `urban_segments.rs`, implementing Equation 18-15, `S_T,seg = 3,600 L/(5,280(t_R + d_t))`. It requires `running_time_s` (t_R, s, from Step 2) and `through_delay_s` (d_t, s/veh, from Step 5), combines them with `segment_length_ft` (L, ft), and writes `travel_speed_mph` (S_T,seg, mi/h); it returns `None` if either prerequisite step has not run.

```
Equation 18-15:  S_T,seg = 3,600·L / (5,280·(t_R + d_t))     [mi/h]
  L   = segment length                                            (ft)
  t_R = segment running time (Step 2, Eq. 18-7)                   (s)
  d_t = through delay (Step 5)                                    (s/veh)
Implemented in: urban_segments/urban_segments.rs::UrbanSegment::step_7_travel_speed
```

Step 8, "Determine Spatial Stop Rate," is `UrbanSegment::step_8_spatial_stop_rate` in `urban_segments.rs`, implementing Equation 18-16, `H_seg = 5,280(h + h_other)/L`. It requires `full_stop_rate` (h, stops/veh, from Step 6), adds `stop_rate_other` (h_other, stops/veh), divides by `segment_length_ft` (L, ft), and writes `spatial_stop_rate_stops_mi` (H_seg, stops/mi).

```
Equation 18-16:  H_seg = 5,280·(h + h_other) / L     [stops/mi]
  h       = full stop rate (Step 6)                                  (stops/veh)
  h_other = stop rate due to other midsegment sources                (stops/veh)
  L       = segment length                                           (ft)
Implemented in: urban_segments/urban_segments.rs::UrbanSegment::step_8_spatial_stop_rate
```

Step 9, "Determine LOS," is `UrbanSegment::step_9_los` in `urban_segments.rs`, delegating the table lookup to `exhibit_18_1_los` in `exhibits.rs`. It requires `travel_speed_mph` (S_T,seg, mi/h, Step 7) and `base_ffs_mph` (S_fo, mi/h, Step 2), computes the through-movement v/c ratio from `through_capacity_veh_h` and `through_demand_veh_h` (writing it to `vc_ratio`), and passes `vc_ratio > 1.0` as a flag that forces LOS F regardless of speed. `exhibit_18_1_los` interpolates the Exhibit 18-1 travel-speed thresholds for LOS A–E via `exhibit_18_1_speed_thresholds`, which linearly interpolates between the tabulated base-free-flow-speed column headings `{55, 50, 45, 40, 35, 30, 25}` mi/h as directed by the Chapter 18 text, and clamps `base_ffs_mph` to `[25, 55]` mi/h when it falls outside the tabulated range — a spec gap cross-referenced to the VERIFICATION.md Chapter 18 entry, since Exhibit 18-1 does not define thresholds beyond its column headings. When no capacity input is available, `vc_ratio` remains `None` and the v/c > 1.0 rule is simply not evaluated (treated as v/c ≤ 1.0), which is a code-level default rather than an HCM-specified behavior. The result is written to `los` (`LevelOfService`, from `crate::hcm::common`).

```
Exhibit 18-1 interpolation mechanism (no HCM equation number; the Chapter 18 text: "The threshold value is interpolated when the base free-flow speed is between the values shown in the column headings"):
  frac = (S_fo − BFFS_lo) / (BFFS_hi − BFFS_lo)
  threshold[LOS] = row[BFFS_lo] + frac · (row[BFFS_hi] − row[BFFS_lo])     [mi/h]
    S_fo          = base free-flow speed (Eq. 18-3)                                          (mi/h)
    BFFS_hi, BFFS_lo = the two tabulated base-free-flow-speed column headings from
                       {55, 50, 45, 40, 35, 30, 25} mi/h that bracket S_fo
    row[BFFS_hi], row[BFFS_lo] = the Exhibit 18-1 tabulated travel-speed threshold cell values
                       for a given LOS letter (A-E) at those two column headings — copyrighted
                       exhibit data, cited by function name only, not transcribed here
  Note: S_fo is clamped to [25, 55] mi/h before interpolating when it falls outside the tabulated range — a documented spec gap (VERIFICATION.md Chapter 18 entry), since Exhibit 18-1 does not define thresholds beyond its column headings. The Chapter 18 text's own worked example (S_fo = 42 mi/h) rounds its interpolated LOS A threshold to 34 mi/h in prose; the code's unrounded arithmetic gives 33.6 mi/h, matched by `test_exhibit_18_1_interpolation_chapter_text_example`.
Implemented in: urban_segments/exhibits.rs::exhibit_18_1_speed_thresholds

LOS assignment:
  LOS = F                                    if vc_ratio > 1.0   (through-movement v/c at the downstream boundary, forces LOS F regardless of speed)
      = A                                    if S_T,seg > threshold[A]
      = B                                    if S_T,seg > threshold[B]
      = C                                    if S_T,seg > threshold[C]
      = D                                    if S_T,seg > threshold[D]
      = E                                    if S_T,seg > threshold[E]
      = F                                    otherwise
    vc_ratio = through_demand_veh_h / through_capacity_veh_h                                 (unitless)
    S_T,seg  = travel speed (Step 7, Eq. 18-15)                                              (mi/h)
Implemented in: urban_segments/exhibits.rs::exhibit_18_1_los; urban_segments/urban_segments.rs::UrbanSegment::step_9_los
```

Step 10, "Determine Automobile Traveler Perception Score," is `UrbanSegment::step_10_perception_score` in `urban_segments.rs`, implementing Equations 18-17 through 18-22 via the free function `traveler_perception_score` in the same file: `I_a,seg = 1 + P_BCDEF + P_CDEF + P_DEF + P_EF + P_F`, with each `P_x = (1 + e^(a_x − 0.253 H_seg + 0.3434 P_LTL,seg))^-1` and intercepts `a = {−1.1614, 0.6234, 1.7389, 2.7047, 3.8044}`. It requires `spatial_stop_rate_stops_mi` (H_seg, stops/mi, from Step 8) and the input `prop_left_turn_lanes` (P_LTL,seg, decimal proportion of segment intersections with a left-turn lane or bay), and writes the dimensionless score to `perception_score`.

```
Equation 18-17:  I_a,seg = 1 + P_BCDEF + P_CDEF + P_DEF + P_EF + P_F     [unitless score]

Equation 18-18:  P_BCDEF = (1 + e^(−1.1614 − 0.253·H_seg + 0.3434·P_LTL,seg))^-1
Equation 18-19:  P_CDEF  = (1 + e^( 0.6234 − 0.253·H_seg + 0.3434·P_LTL,seg))^-1
Equation 18-20:  P_DEF   = (1 + e^( 1.7389 − 0.253·H_seg + 0.3434·P_LTL,seg))^-1
Equation 18-21:  P_EF    = (1 + e^( 2.7047 − 0.253·H_seg + 0.3434·P_LTL,seg))^-1
Equation 18-22:  P_F     = (1 + e^( 3.8044 − 0.253·H_seg + 0.3434·P_LTL,seg))^-1

  H_seg      = spatial stop rate (Step 8, Eq. 18-16)                                        (stops/mi)
  P_LTL,seg  = proportion of segment intersections with a left-turn lane or bay (`prop_left_turn_lanes`) (decimal)
  Intercepts a = {−1.1614, 0.6234, 1.7389, 2.7047, 3.8044} for P_BCDEF, P_CDEF, P_DEF, P_EF, P_F respectively.
Implemented in: urban_segments/urban_segments.rs::traveler_perception_score
```

`UrbanSegment::analyze`, also in `urban_segments.rs`, runs Steps 1, 2, 3, 5, 6, 7, 8, 9, and 10 in that order (Step 4 is skipped because it is an input, not a computation), mutating the struct's `Option` result fields in place.

The following table summarizes the step-to-implementation mapping:

| HCM Step | Equations / Exhibits | Rust function | File |
|---|---|---|---|
| 1. Traffic demand adjustments | (simplified; Ch. 30 §2 deferred) | `UrbanSegment::step_1_demand_adjustment` | `src/hcm/urban_segments/urban_segments.rs` |
| 2. Running time | Eq. 18-3–18-8, Exhibits 18-11, 18-13 | `UrbanSegment::step_2_running_time` (+ `speed_constant_s0`, `cross_section_adjustment`, `access_point_density`, `access_point_adjustment`, `parking_adjustment` in `exhibits.rs`; `signal_spacing_adjustment`, `proximity_adjustment` in `urban_segments.rs`; `exhibit_18_13_turn_delay(_adjusted)` in `exhibits.rs`) | `src/hcm/urban_segments/urban_segments.rs`, `src/hcm/urban_segments/exhibits.rs` |
| 3. Proportion arriving on green | Eq. 19-15, Exhibit 19-13 | `UrbanSegment::step_3_proportion_arriving_green` (+ `platoon_ratio_for_arrival_type` in `signalized::exhibits`) | `src/hcm/urban_segments/urban_segments.rs` |
| 4. Signal phase duration | (input; Ch. 19 engine) | not implemented — `cycle_length_s`, `effective_green_s` fields | `src/hcm/urban_segments/urban_segments.rs` |
| 5. Through delay | Eq. 18-10 (weighting helper) | `UrbanSegment::step_5_through_delay` (+ `shared_lane_through_delay`) | `src/hcm/urban_segments/urban_segments.rs` |
| 6. Through stop rate | Eq. 18-11–18-14 | `UrbanSegment::step_6_stop_rate` (+ `full_stop_rate_signalized`, `weighted_through_lane_value`) | `src/hcm/urban_segments/urban_segments.rs` |
| 7. Travel speed | Eq. 18-15 | `UrbanSegment::step_7_travel_speed` | `src/hcm/urban_segments/urban_segments.rs` |
| 8. Spatial stop rate | Eq. 18-16 | `UrbanSegment::step_8_spatial_stop_rate` | `src/hcm/urban_segments/urban_segments.rs` |
| 9. LOS | Exhibit 18-1 | `UrbanSegment::step_9_los` (+ `exhibit_18_1_los`, `exhibit_18_1_speed_thresholds`) | `src/hcm/urban_segments/urban_segments.rs`, `src/hcm/urban_segments/exhibits.rs` |
| 10. Traveler perception score | Eq. 18-17–18-22 | `UrbanSegment::step_10_perception_score` (+ `traveler_perception_score`) | `src/hcm/urban_segments/urban_segments.rs` |

Two additional free functions exist outside the numbered pipeline but are part of the Chapter 18 equation set: `default_access_point_count` (Equation 18-1, `N_ap,s = 0.5 D_a L/5,280`) and `through_capacity_uncontrolled` (Equation 18-2, `c_th = 1,800(N_th − 1 + p*_0,j)`, for the uncontrolled through movement's capacity at a TWSC boundary). The doc comment on `through_capacity_uncontrolled` notes, and the VERIFICATION.md Chapter 18 entry confirms, that the Chapter 18 text cites "Equation 20-43" for `p*_0,j`, which is an HCM 6th Edition equation number; the HCM 7th Edition equivalents are Equations 20-29 through 20-34 (computed elsewhere by `Twsc::prob_queue_free_shared_major` in `src/hcm/twsc/twsc.rs`), and 7th-edition Equation 20-43 is instead the Rank 4 movement capacity equation — a citation mismatch in the manual text rather than in this code.

```
Equation 18-1:  N_ap,s = 0.5 · D_a · L / 5,280     [points]
  D_a = access point density on the segment (e.g., the Exhibit 18-7 default)   (points/mi)
  L   = segment length                                                         (ft)
  Note: a default estimate used only when the actual access point count is not known.
Implemented in: urban_segments/urban_segments.rs::default_access_point_count

Equation 18-2:  c_th = 1,800 · (N_th − 1 + p*_0,j)     [veh/h]
  N_th    = number of through lanes, shared or exclusive                       (ln)
  p*_0,j  = probability that there will be no queue in the inside through lane; equal to 1.0
            if a left-turn bay is provided for left turns from the major street, otherwise
            computed with `Twsc::prob_queue_free_shared_major` (HCM 7th Edition Equations
            20-29 through 20-34 — the Chapter 18 text's "Equation 20-43" citation is the
            HCM 6th Edition number for this same quantity; see the discrepancy note above)
Implemented in: urban_segments/urban_segments.rs::through_capacity_uncontrolled; p*_0,j via twsc/twsc.rs::Twsc::prob_queue_free_shared_major
```

## Validation

The primary validation fixture is HCM 7th Edition Chapter 30 ("Urban Street Segments: Supplemental"), Section 8, Example Problem 1 ("Motorized Vehicle LOS"), Exhibits 30-26 through 30-36. Two directions of that example are reproduced as JSON fixtures under `tests/ExampleCases/hcm/UrbanSegments/`: `case1.json` (eastbound, using the published Chapter 30 Section 4 per-access-point turning delays from Exhibit 30-35 as direct inputs) and `case2.json` (westbound, using the Exhibit 18-13 planning-level turning-delay estimate instead of the Section 4 procedure, to exercise that code path). Both are exercised by the Rust integration tests in `tests/chapter18_integration.rs`: `test_case1_example_problem_1_eastbound` and `test_case2_example_problem_1_westbound_planning_estimate`, plus a serde-round-trip check `test_fixture_round_trip`. For `case1.json`, tolerances are ±0.01 mi/h for base FFS/free-flow speed, ±0.01 s for running time, ±0.01 mi/h for running speed and travel speed, ±0.001 s/veh for through delay, ±0.001 stops/veh for stop rate, ±0.01 stops/mi for spatial stop rate, ±0.005 for v/c, exact match for LOS, and ±0.01 for perception score, with intermediate Step 2 chain values (S_0, f_CS, f_A at ±0.001, f_L at ±0.0005, f_v at ±0.0005) also checked; the Step 3 proportion arriving on green is checked at ±0.001 against the milestone-1 uniform-arrival value of 0.486, with the test comment noting the published (platoon-dispersion) engine value is 0.493 — a documented, expected deviation since Chapter 30 Section 3 dispersion is deferred. For `case2.json`, the access-point delay total is checked against the Exhibit 18-13 estimate (0.540 s, ±0.005) rather than the published Section 4 value (0.387 s), and running time and travel speed are each asserted twice: once loosely against the published Exhibit 30-36 values (±0.5 s / ±0.5 mi/h, acknowledging the estimate-vs-procedure deviation) and once tightly against the module's own computed values (33.70 s and 23.60 mi/h, ±0.01) — all other Exhibit 30-36 measures (through delay, spatial stop rate, v/c, LOS, perception score) reproduce exactly within the same tolerances as case1.

Unit-level coverage lives in `src/hcm/urban_segments/tests.rs`, which builds the same Example Problem 1 eastbound segment programmatically (`example_problem_1_segment`) and checks each step's intermediates individually — e.g., `test_step_2_base_free_flow_speed_example_problem_1` (S_0, f_CS, f_A, f_pk, S_fo, f_L, S_f, each within 0.001–0.005 depending on the quantity), `test_step_2_running_time_example_problem_1` (f_v ±0.0005, running time ±0.01 s, running speed ±0.01 mi/h), `test_step_3_proportion_arriving_green` (platoon-ratio, arrival-type, and uniform-arrival paths, each ±0.001 or ±0.0001), `test_equation_18_11_stop_rate` (a hand-computed check of `full_stop_rate_signalized` against a magnitude consistent with the published 0.547 stops/veh, ±0.0005), `test_step_6_stop_rate_defaults` (STOP/uncontrolled/YIELD defaults), and `test_steps_7_to_10_example_problem_1` (travel speed, spatial stop rate, v/c, LOS, and perception score, at the same tolerances as the integration test). Additional unit tests hand-verify the free functions in isolation, e.g. `test_equation_18_2_uncontrolled_capacity`, `test_equation_18_10_shared_lane_delay`, `test_equations_18_12_to_18_14_weighting`, and `test_equation_18_1_default_access_points`, all at tight (≤1e-9) tolerances since they check closed-form arithmetic rather than published example values. `src/hcm/urban_segments/exhibits.rs` additionally carries its own `#[cfg(test)]` module validating the Exhibit 18-1, 18-7, 18-11, and 18-13 lookup tables directly against tabulated HCM values and two textual worked examples (the Chapter 18 text's 42 mi/h BFFS interpolation example, and Chapter 30 Example Problem 1's 40.78 mi/h BFFS thresholds), at tolerances ranging from 0.05 mi/h down to exact equality for table-boundary values.

`tests/test_chapter18_integration.py` additionally exercises the PyO3 Python bindings against `case1.json` (skipped if the bindings are not built with `UrbanSegment` support), with looser tolerances than the Rust tests — ±0.01 mi/h for free-flow speeds, ±0.5 s/mi/h for running time/speed and through delay/travel speed, ±0.01 for stop rates, ±0.005 for v/c, exact LOS ("C"), and ±0.01 for perception score — confirming the same Exhibit 30-36 answers are reachable through the language binding.

## Deferred

Per the module docs in `src/hcm/urban_segments/mod.rs` and `urban_segments.rs`, the following are explicitly out of scope for this branch (milestone 1) and deferred to a milestone 2: the Chapter 30, Section 2 demand-adjustment procedure (origin–destination estimation, volume balancing, and spillback checks — Step 1 here only performs a capacity-constraint flag on analyst-supplied, already-balanced flow rates); the Chapter 30, Section 3 platoon-dispersion arrival-profile and coordinated-system convergence loop (Steps 3–4 — Step 3 falls back to a supplied platoon ratio/arrival type or assumes uniform arrivals, and Step 4 signal phase duration is a plain input rather than a computed loop); and the Chapter 30, Section 4 access-point delay procedure (probability of inside-lane blockage and per-movement platoon interaction — an analyst can supply its output per access point via `access_point_delays_s`, or fall back to the Exhibit 18-13 planning-level estimate, which the code implements in full). The pedestrian, bicycle, and transit mode methodologies of Chapter 18 (LOS scores and supporting equations for those modes) are out of scope entirely and are not present in this module. Per the VERIFICATION.md Chapter 18 entry, there are no VERIFY-HCM items on this branch — the deviations are the two documented spec gaps (Exhibit 18-13 clamping outside 200–700 veh/h/ln, and Exhibit 18-1 clamping outside 25–55 mi/h base free-flow speed, both because the respective exhibits do not define values beyond their tabulated ranges) and the Chapter 18 text's "Equation 20-43" citation, which is an HCM 6th Edition equation number rather than the 7th Edition's Equations 20-29 through 20-34. A grep of `urban_segments.rs` and `exhibits.rs` for `// VERIFY-HCM` found no such markers in either file.
