# HCM Chapter 24 — Off-Street Pedestrian and Bicycle Facilities

This document walks through the Rust translation of HCM 7th Edition Chapter 24, which covers three distinct methodologies for off-street facilities that serve only nonmotorized traffic: exclusive pedestrian facilities (walkways, cross-flow areas, stairways), pedestrians on shared-use paths, and bicyclists on shared-use or exclusive off-street bicycle facilities. All three live in the single file `src/hcm/offstreet_pedbike/offstreet_pedbike.rs`; there is no separate `exhibits.rs` for this chapter, and the exhibit tables (Exhibits 24-1 through 24-6, 24-14 through 24-16) are transcribed inline as LOS-lookup functions and constant arrays alongside the equation code they support. Cross-references are to HCM Chapter 35 ("Pedestrians and Bicycles: Supplemental") for the worked example problems, and to Hummer et al.'s FHWA shared-use-path research (HCM Chapter 24 Ref. 5) for the numerical-integration parameters of the bicycle methodology.

## Step-by-step walkthrough

### 1. Exclusive off-street pedestrian facilities (`ExclusivePedestrianFacility`, Exhibit 24-7)

| Step | Equation/Exhibit | Rust function | Inputs (units) | Outputs (units) |
|---|---|---|---|---|
| 1. Effective walkway width | Eq. 24-1, `W_E = W_T - W_O` | `determine_effective_walkway_width` | `total_walkway_width` (W_T, ft), `fixed_object_width` (W_O, ft; Exhibit 24-9 fixed-object allowances, and for stairways a 2.5-ft reverse-flow lane folded into this term) | `effective_width` (W_E, ft) |
| 2. Pedestrian flow rate | Eq. 24-2 (`v_15 = v_h/(4 PHF)`, hourly-to-peak-15-min), Eq. 24-3 (`v_p = v_15/(15 W_E)`) | `calculate_pedestrian_flow_rate` | `pedestrian_demand` (v_h, p/h, optional) or `peak_15min_volume` (v_15, p, bypasses PHF when supplied directly), `phf`, `effective_width` | `flow_rate_15min` (v_15, p), `unit_flow_rate` (v_p, p/ft/min) |
| 3. Average pedestrian space | Eq. 24-4, `A_p = S_p / v_p` | `calculate_average_pedestrian_space` | `pedestrian_speed` (S_p, ft/min, default 300 per Exhibit 24-6), `unit_flow_rate` | `pedestrian_space` (A_p, ft²/p) |
| 4. LOS | Exhibit 24-1 (random flow), 24-2 (platoon flow), or 24-3 (stairways) | `determine_los` (dispatches on `facility_type`/`flow_type` to `walkway_random_flow_los`, `walkway_platoon_flow_los`, or `stairway_los`) | `pedestrian_space`, `PedestrianFacilityType`, `PedestrianFlowType` | `los: LevelOfService` |
| 5. v/c ratio | (capacity constants, not a numbered equation) | `calculate_volume_to_capacity_ratio` | `unit_flow_rate`, facility-type capacity (`CAPACITY_WALKWAY_RANDOM` = 23, `CAPACITY_WALKWAY_PLATOON` = 18, `CAPACITY_CROSS_FLOW` = 17, `CAPACITY_STAIRWAY` = 15, all p/min/ft) | `vc_ratio` (decimal) |

`ExclusivePedestrianFacility::analyze()` runs all five steps in order. Cross-flow areas use the Exhibit 24-1/24-2 tables but with a distinct LOS E-F space threshold (`CROSS_FLOW_LOS_F_SPACE_THRESHOLD` = 13 ft²/p, note c of both exhibits) and a capacity that is the sum of both crossing flows (17 p/min/ft).

#### Equations (exclusive pedestrian facilities)

```
Equation 24-1:  W_E = W_T − W_O     [ft]
  W_T = total walkway width                                          (ft)
  W_O = fixed-object width allowance (Exhibit 24-9; for stairways includes a 2.5-ft reverse-flow lane) (ft)
Implemented in: offstreet_pedbike/offstreet_pedbike.rs::determine_effective_walkway_width
```

```
Equation 24-2:  v_15 = v_h / (4 · PHF)     [p]
Equation 24-3:  v_p = v_15 / (15 · W_E)     [p/ft/min]
  v_15 = pedestrian volume during the peak 15 min                                (p)
  v_h  = pedestrian demand during the analysis hour                              (p/h)
  PHF  = peak hour factor, default 0.85 (DEFAULT_PHF)                            (decimal)
  v_p  = pedestrian flow per unit width                                          (p/ft/min)
  W_E  = effective walkway width (Equation 24-1)                                 (ft)
  If a field-measured peak 15-min volume is supplied directly, it is used in place of v_15 and Equation 24-2 is bypassed.
Implemented in: offstreet_pedbike/offstreet_pedbike.rs::calculate_pedestrian_flow_rate
```

```
Equation 24-4:  A_p = S_p / v_p     [ft²/p]
  A_p = average pedestrian space                                                 (ft²/p)
  S_p = pedestrian speed, default 300 ft/min (DEFAULT_PEDESTRIAN_SPEED_FT_MIN)    (ft/min)
  v_p = pedestrian flow per unit width (Equation 24-3)                           (p/ft/min)
Implemented in: offstreet_pedbike/offstreet_pedbike.rs::calculate_average_pedestrian_space
```

```
Exhibit 24-1 (Random-Flow LOS Criteria for Walkways): LOS from average pedestrian space A_p (ft²/p)
  A_p > 60          -> LOS A   (v_p ≤ 5 p/min/ft,   v/c ≤ 0.21)
  60 ≥ A_p > 40     -> LOS B   (v_p >5-7,            v/c >0.21-0.31)
  40 ≥ A_p > 24     -> LOS C   (v_p >7-10,           v/c >0.31-0.44)
  24 ≥ A_p > 15     -> LOS D   (v_p >10-15,          v/c >0.44-0.65)
  15 ≥ A_p > 8      -> LOS E   (v_p >15-23,          v/c >0.65-1.00)
  A_p ≤ 8           -> LOS F   (variable)
  Note c (cross-flow): the LOS E-F space threshold becomes CROSS_FLOW_LOS_F_SPACE_THRESHOLD = 13 ft²/p instead of 8 ft²/p.
Implemented in: offstreet_pedbike/offstreet_pedbike.rs::walkway_random_flow_los
```

```
Exhibit 24-2 (Platoon-Adjusted LOS Criteria for Walkways): LOS from average pedestrian space A_p (ft²/p), flow rate averaged over 5 min
  A_p > 530         -> LOS A   (flow ≤0.5 p/min/ft)
  530 ≥ A_p > 90    -> LOS B   (>0.5-3)
  90 ≥ A_p > 40     -> LOS C   (>3-6)
  40 ≥ A_p > 23     -> LOS D   (>6-11)
  23 ≥ A_p > 11     -> LOS E   (>11-18)
  A_p ≤ 11          -> LOS F   (>18)
  Note c (cross-flow): the LOS E-F space threshold becomes 13 ft²/p (CROSS_FLOW_LOS_F_SPACE_THRESHOLD).
Implemented in: offstreet_pedbike/offstreet_pedbike.rs::walkway_platoon_flow_los
```

```
Exhibit 24-3 (LOS Criteria for Stairways): LOS from average pedestrian space A_p (ft²/p)
  A_p > 20          -> LOS A   (v_p ≤ 5 p/min/ft,   v/c ≤ 0.33)
  20 ≥ A_p > 17     -> LOS B   (v_p >5-6,            v/c >0.33-0.41)
  17 ≥ A_p > 12     -> LOS C   (v_p >6-8,            v/c >0.41-0.53)
  12 ≥ A_p > 8      -> LOS D   (v_p >8-11,           v/c >0.53-0.73)
  8 ≥ A_p > 5       -> LOS E   (v_p >11-15,          v/c >0.73-1.00)
  A_p ≤ 5           -> LOS F   (variable)
Implemented in: offstreet_pedbike/offstreet_pedbike.rs::stairway_los
```

```
Volume-to-capacity ratio (Chapter 24, Step 5; not a numbered HCM equation):  v/c = v_p / c
  v_p = pedestrian flow per unit width (Equation 24-3)                           (p/ft/min)
  c   = facility capacity (p/min/ft): CAPACITY_WALKWAY_RANDOM = 23, CAPACITY_WALKWAY_PLATOON = 18, CAPACITY_CROSS_FLOW = 17 (sum of both crossing flows), CAPACITY_STAIRWAY = 15 (ascending direction)
Implemented in: offstreet_pedbike/offstreet_pedbike.rs::calculate_volume_to_capacity_ratio
```

### 2. Pedestrians on shared-use paths (`SharedUsePathPedestrian`, Exhibit 24-4)

| Step | Equation | Rust function | Inputs (units) | Outputs (units) |
|---|---|---|---|---|
| Passing/meeting events | Eq. 24-5 (`F_p = (Q_sb/PHF)(1 - S_p/S_b)`), Eq. 24-6 (`F_m = (Q_ob/PHF)(1 + S_p/S_b)`), Eq. 24-7 (`F = F_p + 0.5 F_m`) | `calculate_bicycle_passing_and_meeting_events` | subject/opposing bicycle volumes `Q_sb`/`Q_ob` (bicycles/h), pedestrian speed `S_p` and bicycle speed `S_b` (ft/min or mi/h, consistent units), `phf` | `(passing_events_per_hour, meeting_events_per_hour, total_events_per_hour)` = `(F_p, F_m, F)` (events/h) |
| LOS | Exhibit 24-4 | `determine_los` (delegates to `shared_use_path_pedestrian_los`) | `total_events_per_hour` (F, events/h) | `los: LevelOfService` |

For a one-way path, meeting events are zero (`F_m = 0`, since there is no opposing bicycle flow); `test_one_way_path_has_no_meeting_events` in `tests.rs` confirms this. `analyze()` runs both steps in order.

#### Equations (pedestrians on shared-use paths)

```
Equation 24-5:  F_p = (Q_sb / PHF) · (1 − S_p/S_b)     [events/h]
Equation 24-6:  F_m = (Q_ob / PHF) · (1 + S_p/S_b)     [events/h]
Equation 24-7:  F = F_p + 0.5 · F_m                     [events/h]
  F_p  = number of passing events                                               (events/h)
  F_m  = number of meeting events                                               (events/h)
  F    = total (weighted) number of events                                      (events/h)
  Q_sb = bicycle demand in the same direction as the pedestrian                  (bicycles/h)
  Q_ob = bicycle demand in the opposing direction                                (bicycles/h)
  PHF  = peak hour factor, default 0.85 (DEFAULT_PHF)                           (decimal)
  S_p  = mean pedestrian speed, default 3.4 mi/h                                (mi/h; any unit consistent with S_b)
  S_b  = mean bicycle speed, default 12.8 mi/h                                  (mi/h; any unit consistent with S_p)
  If field-measured peak 15-min directional bicycle flow rates are known, they substitute directly for the Q_sb/PHF and Q_ob/PHF terms. For one-way paths, F_m = 0 (no opposing bicycle flow).
Implemented in: offstreet_pedbike/offstreet_pedbike.rs::calculate_bicycle_passing_and_meeting_events
```

```
Exhibit 24-4 (Pedestrian LOS Criteria for Shared-Use Paths): LOS from the total weighted event rate F (events/h)
  F ≤ 38            -> LOS A
  38 < F ≤ 60       -> LOS B
  60 < F ≤ 103      -> LOS C
  103 < F ≤ 144     -> LOS D
  144 < F ≤ 180     -> LOS E
  F > 180           -> LOS F
Implemented in: offstreet_pedbike/offstreet_pedbike.rs::shared_use_path_pedestrian_los
```

### 3. Bicyclists on shared-use/exclusive off-street bicycle facilities (`OffStreetBicycleFacility`, Exhibit 24-11, BLOS)

| Step | Equation(s) | Rust function | Inputs (units) | Outputs (units) |
|---|---|---|---|---|
| 1. Directional flow rates | Eq. 24-8, `q_i = (Q_T p_i)/PHF` | `calculate_directional_flow_rates` | `two_way_demand`/`directional_split` or `subject_demand`/`opposing_demand` (path users/h), per-mode `mode_split` (Exhibit 24-6 defaults in `DEFAULT_PATH_USER_GROUPS`), `phf`, `is_one_way` | `subject_flow_rates`/`opposing_flow_rates: [f64; 5]` (modal users/h, one entry per `PathUserMode`) |
| 2. Active passings per minute | Eqs. 24-9 through 24-12 (numerical integration over the segment, `dx` steps, using the normal-CDF speed-distribution model) | `calculate_active_passings_per_minute` (uses `normal_cdf`) | per-mode flow rate/speed/speed-sd, `segment_length` (L, mi), `PATH_INTEGRATION_STEP_MI` (dx = 0.01 mi) | `active_passings_by_mode: [f64; 5]` (A_i, passings/min), `active_passings_per_minute` (A_T) |
| 3. Meetings per minute | Eqs. 24-13 through 24-16 | `calculate_meetings_per_minute` | same per-mode speed/flow inputs, opposing flow rates | `meetings_on_segment` (M_1), `meetings_beyond_segment_by_mode: [f64; 5]` (M_2,i), `meetings_per_minute` (M_T) |
| 4. Effective lanes | Exhibit 24-14 | `determine_number_of_effective_lanes` | `path_width` (ft) | `effective_lanes` (2 for <11 ft, 3 for 11-<15 ft, 4 for >=15 ft) |
| 5. Probability of delayed passing | Eqs. 24-17 through 24-32 (25-modal-pair two-lane model, Eqs. 24-18/24-19/24-20/24-33 for two lanes; Eqs. 24-21 through 24-32 aggregated single-/double-lane blockage for three lanes; Eqs. 24-25/24-27 for four lanes) | `calculate_probability_of_delayed_passing` (+ `probability_blocked`, `delayed_passing_probability_two_lane`) | per-mode densities `k_i = q_i/mu_i` (users/mi), `REQUIRED_PASSING_DISTANCE_FT` (Exhibit 24-15), `TWO_LANE_BLOCKING_FREQUENCY` (Exhibit 24-16), `effective_lanes` | `total_probability_delayed_passing` (P_Tds, decimal) |
| 6. Delayed passings per minute | Eq. 24-34, `DP_m = A_T P_Tds PHF` | `calculate_delayed_passings_per_minute` | `active_passings_per_minute`, `total_probability_delayed_passing`, `phf` | `delayed_passings_per_minute` (DP_m) |
| 7. BLOS | Eq. 24-35, `BLOS = 5.446 - 0.00809E - 15.86 RW - 0.287 CL - DP` | `determine_blos` | `meetings_per_minute`, `active_passings_per_minute` (-> E = M_T + 10 A_T), `path_width` (-> RW = 1/width), `has_centerline` (CL), `delayed_passings_per_minute` (-> DP = min(0.5 DP_m, 1.5)) | `weighted_events_per_minute` (E), `blos_score`, `los` (Exhibit 24-5 via `bicycle_los_from_score`) |
| 8. Low-volume adjustment | (Chapter 24 text: Eq. 24-35 cannot yield LOS A/B for narrow paths) | `adjust_los_for_low_volume_paths` | `weighted_events_per_minute` (E), the Step 7 `los` | adjusted `los` (E <= 5 -> A; 5 < E <= 10 and base != A -> B; else unchanged) |

`OffStreetBicycleFacility::analyze()` runs Steps 1-8 in order. For an exclusive bicycle facility, set the bicycle mode split to 1.0 and all others to zero (`test_exclusive_bicycle_facility_zero_nonbike_modes` in `tests.rs` exercises this). `PathUserMode` (`Bicycle`, `Pedestrian`, `Runner`, `InlineSkater`, `ChildBicyclist`) indexes every per-mode array (`REQUIRED_PASSING_DISTANCE_FT`, `TWO_LANE_BLOCKING_FREQUENCY`, `DEFAULT_PATH_USER_GROUPS`) consistently.

#### Equations (Steps 1-4: flow rates, active passings, meetings, effective lanes)

```
Equation 24-8:  q_i = (Q_T · p_i) / PHF     [modal users/h]
  q_i  = hourly directional path flow rate for user group i                     (modal users/h)
  Q_T  = total hourly directional path demand                                   (path users/h)
  p_i  = path mode split for user group i (Exhibit 24-6 defaults, DEFAULT_PATH_USER_GROUPS) (decimal)
  PHF  = peak hour factor, default 0.85 (DEFAULT_PHF)                          (decimal)
  Computed separately for the subject direction (Q_T from subject_demand, or two_way_demand · directional_split) and the opposing direction (zero when is_one_way).
Implemented in: offstreet_pedbike/offstreet_pedbike.rs::calculate_directional_flow_rates
```

```
Equation 24-9:   P(v_i) = P[v_i < U·(1 − x/L)]
Equation 24-10:  P(v_i) = 0.5·[F(x − dx) + F(x)]
Equation 24-11:  A_i = Σ_j P(v_i) · (q_i/μ_i) · (1/t) · dx_j     [passings/min]
Equation 24-12:  A_T = Σ_i A_i                                    [passings/min]
  P(v_i) = probability that the average bicyclist passes a mode-i user present at the start of the segment (decimal)
  v_i    = speed of a path user of mode i, normally distributed with mean μ_i and std dev σ_i (mi/h)
  U      = speed of the average bicyclist (average_speed of the Bicycle mode group)              (mi/h)
  x      = distance from the average bicyclist to the user                     (mi)
  L      = length of the path segment, segment_length                          (mi)
  F(x)   = normal_cdf(U·(1 − x/L), μ_i, σ_i), the standard-normal CDF of the mode-i speed distribution
  dx_j   = length of discrete integration piece j; L is divided into n = round(L / PATH_INTEGRATION_STEP_MI) pieces of size dx = L/n, PATH_INTEGRATION_STEP_MI = 0.01 mi (mi)
  q_i    = directional hourly flow rate of mode i (Equation 24-8)              (modal users/h)
  μ_i    = average speed of mode i                                              (mi/h)
  t      = path segment travel time for the average bicyclist = (L/U) · 60      (min)
  A_i    = expected active passings per minute of mode i by the average bicyclist (passings/min)
  A_T    = total expected active passings per minute during the peak 15 min     (passings/min)
Implemented in: offstreet_pedbike/offstreet_pedbike.rs::calculate_active_passings_per_minute
```

```
Equation 24-13:  M_1 = (U/60) · Σ_i (q_i/μ_i)                      [meetings/min]
Equation 24-14:  P(v_O,i) = P(v_i > X·U/L)
Equation 24-15:  M_2,i = Σ_j P(v_O,i) · (q_i/μ_i) · (1/t) · dx_j    [meetings/min]
Equation 24-16:  M_T = M_1 + Σ_i M_2,i                              [meetings/min]
  M_1      = meetings per minute of opposing-direction users already on the segment when the average bicyclist enters (meetings/min)
  U        = speed of the average bicyclist                                    (mi/h)
  q_i      = opposing-direction hourly flow rate of mode i (Equation 24-8)      (modal users/h)
  μ_i      = average speed of mode i                                            (mi/h)
  P(v_O,i) = probability of meeting an opposing user of mode i located beyond the segment; computed as 1 − normal_cdf(X·U/L, μ_i, σ_i) (decimal)
  X        = distance of the opposing user beyond the end of the segment; the supply length x* is set equal to L, which captures ≥99% of meetings (mi)
  L        = segment_length                                                     (mi)
  t        = path segment travel time for the average bicyclist                 (min)
  dx_j     = discrete integration piece length, same discretization as Equation 24-11 (mi)
  M_2,i    = expected meetings per minute with mode-i users beyond the segment at entry (meetings/min)
  M_T      = total expected meetings per minute during the peak 15 min; M_T = 0 for one-way paths (meetings/min)
Implemented in: offstreet_pedbike/offstreet_pedbike.rs::calculate_meetings_per_minute
```

```
Exhibit 24-14 (Effective Lanes by Path Width): lanes from path width W (ft)
  8.0 ft ≤ W ≤ 10.5 ft    -> 2 lanes
  11.0 ft ≤ W ≤ 14.5 ft   -> 3 lanes
  15.0 ft ≤ W ≤ 20.0 ft   -> 4 lanes
  The exhibit is undefined at W < 8, 10.5 < W < 11, 14.5 < W < 15, and W > 20 ft; the implemented banding is W < 11 -> 2, W < 15 -> 3, else -> 4 (see Deviations item 3 below).
Implemented in: offstreet_pedbike/offstreet_pedbike.rs::determine_number_of_effective_lanes
```

### The 25-modal-pair delayed-passing model (Step 5, two-lane paths)

For a two-lane path, `calculate_probability_of_delayed_passing` computes the blocked-lane probability for each of the 5 path-user modes in the subject direction (`probability_blocked(passing_distance_ft, density)`, Equation 24-17, a Poisson model `P_n,i = 1 - e^{-p_i k_i}`) and each of the 5 modes in the opposing direction, then evaluates `delayed_passing_probability_two_lane(p_ns, p_no)` (Equation 24-20, the closed-form solution of Equations 24-18/24-19) for every one of the resulting 5x5 = 25 (subject-mode, opposing-mode) pairs, combining them via Equation 24-33 (`P_Tds = 1 - Prod_m(1 - P_m,ds)`) as a nested nested loop over `i in 0..NUM_PATH_MODES` and `j in 0..NUM_PATH_MODES`. A documented `VERIFY-HCM` note on this function states that for each pair, the required passing distance of the *subject* (passed) mode `p_i` is applied to *both* the subject and opposing blocked-lane probabilities (rather than each direction using its own mode's passing distance) — this reproduces HCM Chapter 35 Example Problem 2 exactly (`P_n,ped` = 0.1908 computed with a 100-ft passing distance, and `P_Tds` = 0.8334), but the printed text of Equation 24-17 is ambiguous on which mode's distance governs each side of a cross-mode pair.

```
Equation 24-17 (Poisson blocked-lane probability, general form — applies to both the subject and opposing directions):
  P_n,i = 1 − e^(−p_i · k_i)
  P_n,i = probability that the passing section is blocked by mode i                        (decimal)
  p_i   = distance required to pass mode i (Exhibit 24-15, REQUIRED_PASSING_DISTANCE_FT[i]; converted ft -> mi by /5280 in code) (mi)
  k_i   = density of users of mode i = q_i / μ_i                                            (users/mi)
Implemented in: offstreet_pedbike/offstreet_pedbike.rs::probability_blocked
```

```
Two-lane paths — Equations 24-18/24-19 (simultaneous) and 24-20 (closed-form solution, the form actually used by the code):
Equation 24-18:  P_ds = P_no·P_ns + P_no·(1 − P_ns)·(1 − P_do)
Equation 24-19:  P_do = P_no·P_ns + P_ns·(1 − P_no)·(1 − P_ds)
Equation 24-20:  P_ds = [P_no·P_ns + P_no·(1 − P_ns)²] / [1 − P_no·P_ns·(1 − P_no)·(1 − P_ns)]
  P_ds = probability of delayed passing in the subject direction                            (decimal)
  P_do = probability of delayed passing in the opposing direction                           (decimal)
  P_ns = probability of a blocked lane in the subject direction (Equation 24-17)             (decimal)
  P_no = probability of a blocked lane in the opposing direction (Equation 24-17)            (decimal)
  Equation 24-20 solves the simultaneous Equations 24-18/24-19 directly for P_ds without needing P_do.
Implemented in: offstreet_pedbike/offstreet_pedbike.rs::delayed_passing_probability_two_lane
```

```
Modal-pair table structure (two-lane path): the methodology evaluates every one of NUM_PATH_MODES × NUM_PATH_MODES = 5 × 5 = 25 (subject-mode i, opposing-mode j) pairs over the 5 PathUserMode values (Bicycle, Pedestrian, Runner, InlineSkater, ChildBicyclist) in each direction, then combines them with Equation 24-33:
  product = 1
  for i in 0..NUM_PATH_MODES:                     # subject-direction mode being passed
    p_i  = REQUIRED_PASSING_DISTANCE_FT[i]
    P_ns = probability_blocked(p_i, k_s[i])        # Equation 24-17, subject direction
    for j in 0..NUM_PATH_MODES:                    # opposing-direction mode
      P_no   = probability_blocked(p_i, k_o[j])    # Equation 24-17, opposing direction — uses the subject mode's p_i for both sides (see Deviations item 1 below)
      P_m,ds = delayed_passing_probability_two_lane(P_ns, P_no)   # Equation 24-20
      product *= (1 − P_m,ds)
  P_Tds = 1 − product                              # Equation 24-33

Equation 24-33 (combination across the 25 modal pairs — applies only to the two-lane branch; the three- and four-lane branches instead aggregate over modes directly, per mode, before Equations 24-23/24-24 or the four-lane form, and so do not perform a further pairwise product):
  P_Tds = 1 − Π_m (1 − P_m,ds)
  P_Tds  = total probability of delayed passing across all 25 modal pairs (two-lane paths only) (decimal)
  P_m,ds = probability of delayed passing for modal pair m (Equation 24-20)                   (decimal)
  Π_m    = product taken over all 25 modal pairs m
Implemented in: offstreet_pedbike/offstreet_pedbike.rs::calculate_probability_of_delayed_passing
```

For three-lane paths, Equations 24-21 through 24-32 are implemented as mode-aggregated single-lane-blockage (`p_ns`, `p_no`) and double-lane-blockage (`p_bs`, `p_bo`) sums (using `TWO_LANE_BLOCKING_FREQUENCY`, Exhibit 24-16, as the "frequency of blocking of two lanes" weight per mode) substituted into Equation 24-23 (a correlation term `d`) and Equation 24-24 (the combined `P_ds`, clamped to `[0,1]`). A second documented `VERIFY-HCM` note states that Equations 24-29/24-30 as printed read `1 - e^{p_i k} - P_b` (a positive exponent), which is implemented instead as `1 - e^{-p_i k} - P_b` (at-least-one-lane blockage probability minus the two-lane blockage probability) — the positive-exponent form as printed would not correspond to any sensible probability decomposition, and no published worked example exists for three-lane paths to confirm the correct sign independently. Four-lane paths are treated as operating like a divided four-lane highway, where `P_ds` simply equals the probability that both subject-direction lanes are blocked, `P_bs` (Equations 24-25/24-27, no opposing-direction interaction).

```
Three-lane paths — Equations 24-21/24-22 (simultaneous, mode-aggregated) and 24-23/24-24 (closed-form solution used by the code):
Equation 24-21:  P_ds = P_ns·[P_bo + P_no·(1 − P_do)] + P_bs
Equation 24-22:  P_do = P_no·[P_bs + P_ns·(1 − P_ds)] + P_bo
Equation 24-23:  D = [(P_bs − P_bo) + (P_ns·P_bo − P_no·P_bs)] / (1 − P_ns·P_no)
Equation 24-24:  P_ds = [P_ns·(P_bo + P_no·(1 + D)) + P_bs] / (1 + P_ns·P_no)
  P_ds, P_do = probability of delayed passing in the subject/opposing direction (a single aggregate value, not per modal pair) (decimal)
  P_ns, P_no = mode-aggregated probability that a single lane is blocked in the subject/opposing direction (Equations 24-31/24-32) (decimal)
  P_bs, P_bo = mode-aggregated probability that both lanes are blocked in the subject/opposing direction (Equations 24-27/24-28) (decimal)
  D          = P_ds − P_do, the correlation term obtained by solving the simultaneous Equations 24-21/24-22               (decimal)
  Equation 24-24 substitutes D (Equation 24-23) back into Equation 24-21 for a closed form; the code clamps the result to [0,1].
Implemented in: offstreet_pedbike/offstreet_pedbike.rs::calculate_probability_of_delayed_passing (three-lane branch)
```

```
Equations 24-25/24-26 (per-mode two-lane blockage) and 24-27/24-28 (summed over the 5 modes):
Equation 24-25:  P_bs,i = F_i · P_ns,i
Equation 24-26:  P_bo,i = F_i · P_no,i
Equation 24-27:  P_bs = Σ_i P_bs,i
Equation 24-28:  P_bo = Σ_i P_bo,i
  P_bs,i, P_bo,i = probability that a mode-i user blocks two lanes in the subject/opposing direction              (decimal)
  F_i            = frequency with which mode i blocks two lanes (Exhibit 24-16, TWO_LANE_BLOCKING_FREQUENCY[i])   (decimal)
  P_ns,i, P_no,i = probability of at least one blocked lane for mode i, per mode (Equation 24-17, before summing) (decimal)
  P_bs, P_bo     = total two-lane blockage probability, subject/opposing direction, summed over the 5 modes       (decimal)
Implemented in: offstreet_pedbike/offstreet_pedbike.rs::calculate_probability_of_delayed_passing (three-lane and four-lane branches)
```

```
Equations 24-29/24-30 (per-mode single-lane-only blockage) and 24-31/24-32 (summed over the 5 modes):
Equation 24-29 (as printed in HCM):     P_ns,i = 1 − e^(p_i·k_s,i) − P_bs,i
Equation 24-30 (as printed in HCM):     P_no,i = 1 − e^(p_i·k_o,i) − P_bo,i
Equation 24-29 (as implemented, code): P_ns,i = 1 − e^(−p_i·k_s,i) − P_bs,i
Equation 24-30 (as implemented, code): P_no,i = 1 − e^(−p_i·k_o,i) − P_bo,i
Equation 24-31:  P_ns = Σ_i P_ns,i
Equation 24-32:  P_no = Σ_i P_no,i
  P_ns,i, P_no,i = probability that mode i blocks exactly a single lane in the subject/opposing direction   (decimal)
  p_i            = required passing distance for mode i (Exhibit 24-15, REQUIRED_PASSING_DISTANCE_FT[i])    (mi)
  k_s,i, k_o,i   = density of mode-i users in the subject/opposing direction                                 (users/mi)
  P_bs,i, P_bo,i = two-lane blockage probability for mode i (Equations 24-25/24-26)                          (decimal)
  (see Deviations item 2 below for the sign convention: the code uses the negative-exponent implemented form, since the printed positive exponent does not reduce to a sensible at-least-one-lane-blocked-minus-two-lane-blocked decomposition of Equation 24-17's e^(−p_i·k) form)
Implemented in: offstreet_pedbike/offstreet_pedbike.rs::calculate_probability_of_delayed_passing (three-lane branch)
```

```
Four-lane paths — Equations 24-25 and 24-27 (no opposing-direction interaction; the path operates like a divided four-lane highway):
  P_ds = P_bs = Σ_i F_i · P_ns,i
  P_ds   = probability of delayed passing, equal to the probability that both subject-direction lanes are blocked (decimal)
  F_i    = frequency with which mode i blocks two lanes (Exhibit 24-16, TWO_LANE_BLOCKING_FREQUENCY[i])          (decimal)
  P_ns,i = probability of at least one blocked lane for mode i in the subject direction (Equation 24-17)          (decimal)
  No passing occurs in the leftmost (opposing-direction) lanes, so P_ds is independent of opposing-direction users.
Implemented in: offstreet_pedbike/offstreet_pedbike.rs::calculate_probability_of_delayed_passing (four-lane branch)
```

```
Equation 24-34:  DP_m = A_T · P_Tds · PHF     [delayed passings/min]
  DP_m  = delayed passings per minute                                                       (delayed passings/min)
  A_T   = total active passings per minute (Equation 24-12)                                 (passings/min)
  P_Tds = total probability of delayed passing (Equation 24-33 for two lanes; the aggregate P_ds for three/four lanes) (decimal)
  PHF   = peak hour factor, default 0.85; converts A_T from peak-15-min to hourly conditions, since the delayed-passing factor was calibrated on hourly volumes (decimal)
Implemented in: offstreet_pedbike/offstreet_pedbike.rs::calculate_delayed_passings_per_minute
```

### The events-based BLOS chain and shared-use-path ped LOS

Both the pedestrian and bicycle methodologies in this chapter are ultimately events-based: shared-use-path pedestrian LOS (Exhibit 24-4) is driven by a single combined passing-plus-half-meeting event rate `F` (Equation 24-7), while bicycle BLOS (Equation 24-35) is driven by a weighted event rate `E = M_T + 10 A_T` (meetings per minute plus ten times active passings per minute, reflecting that active passing maneuvers are a much stronger disamenity than simple meetings) combined with path-width, centerline, and delayed-passing terms. The `normal_cdf` helper (built on an `erf` approximation per Abramowitz & Stegun 7.1.26, |error| < 1.5e-7) underlies the Equations 24-9/24-10 speed-distribution integration used by both the active-passings and meetings-per-minute calculations, discretized at `PATH_INTEGRATION_STEP_MI` = 0.01 mi per the Chapter 24 research-finding note that this step size is appropriate for Equation 24-11 and subsequent equations.

#### Equations (Steps 7-8: BLOS score and low-volume adjustment)

```
Equation 24-35:  BLOS = 5.446 − 0.00809·E − 15.86·RW − 0.287·CL − DP
  BLOS = bicycle level-of-service score                                                     (decimal)
  E    = weighted events per minute = M_T + 10·A_T                                           (events/min)
  RW   = reciprocal of path width = 1 / path_width                                           (1/ft)
  CL   = 1 if the path has a centerline stripe, else 0                                        (decimal)
  DP   = min(DP_m · 0.5, 1.5)                                                                 (decimal)
  M_T  = meetings per minute (Equation 24-16)                                                (meetings/min)
  A_T  = active passings per minute (Equation 24-12)                                         (passings/min)
  DP_m = delayed passings per minute (Equation 24-34)                                        (delayed passings/min)
Implemented in: offstreet_pedbike/offstreet_pedbike.rs::determine_blos
```

```
Exhibit 24-5 (LOS Criteria for Bicycles on Shared-Use and Exclusive Paths): LOS from the BLOS score
  BLOS > 4.0          -> LOS A
  4.0 ≥ BLOS > 3.5    -> LOS B
  3.5 ≥ BLOS > 3.0    -> LOS C
  3.0 ≥ BLOS > 2.5    -> LOS D
  2.5 ≥ BLOS > 2.0    -> LOS E
  BLOS ≤ 2.0          -> LOS F
Implemented in: offstreet_pedbike/offstreet_pedbike.rs::bicycle_los_from_score
```

```
Step 8 low-volume LOS adjustment (Chapter 24 text; not a numbered HCM equation): overrides the Exhibit 24-5 result using the weighted events per minute E from Equation 24-35
  E ≤ 5                                          -> LOS A
  5 < E ≤ 10 and Exhibit 24-5 result ≠ LOS A     -> LOS B
  otherwise                                       -> unchanged (Exhibit 24-5 result)
Implemented in: offstreet_pedbike/offstreet_pedbike.rs::adjust_los_for_low_volume_paths
```

## Deviations (cross-referenced to `docs/hcm/VERIFICATION.md`)

`docs/hcm/VERIFICATION.md` exists at this branch's tip; its "Chapter 24 (feat/hcm-ch24-offstreet-pedbike)" section, together with the inline `// VERIFY-HCM` comments in `offstreet_pedbike.rs`, records five items: (1) the Equation 24-17 modal-pair passing-distance ambiguity documented above — the implementation reproduces the worked example (`P_Tds` = 0.8334) using the subject-mode distance for both sides of each pair; an alternative reading (each side using its own mode's required passing distance) gives 0.8241 instead; (2) the likely sign typo in Equations 24-29/24-30 (`1 - e^{p_i k} - P_b` as printed vs. the implemented `1 - e^{-p_i k} - P_b`), unconfirmed by any published three-lane worked example; (3) Exhibit 24-14 (effective lanes by path width) is undefined for widths <8 ft, 10.5-11 ft, 14.5-15 ft, and >20 ft — the documented interpolation rule used is the simple `<11 -> 2, <15 -> 3, else -> 4` banding implemented in `determine_number_of_effective_lanes` (note the code comment there also observes the methodology's own stated inapplicability above 20 ft); (4) Chapter 35 Example Problem 2's book value M1 = 5.36 is computed with a runner speed of 6.6 mi/h, which appears to be a typo for the Exhibit 24-6 default of 6.5 mi/h (the exact value at 6.5 mi/h is 5.38, the value this implementation reproduces); (5) the published child-bicycle flow rate "9/h" in the same example is a truncation of the exact computed value 9.44 (not a rounding difference the code needs to correct for, simply a note that the published figure is not the full-precision intermediate).

## Validation

Fixtures live under `tests/ExampleCases/hcm/OffStreetPedBike/` as `case1.json` and `case2.json`, exercised by `tests/chapter24_integration.rs` and mirrored by `tests/test_chapter24_integration.py`. `case1.json` reproduces HCM Chapter 35, Example Problem 1 (pedestrian LOS on shared-use and exclusive paths): `test_example_problem_1_pedestrian_los` asserts the shared-use-path passing/meeting/total event rates, the shared-use-path pedestrian LOS letter exactly, and the exclusive-path effective width, unit flow rate, pedestrian space, and LOS letter exactly, with numeric assertions at roughly half a unit of the last published significant figure (the module doc comment states published results are "rounded to two or three significant figures and were computed with rounded intermediate values," so tolerances are widened per-assertion where documented). `case2.json` reproduces Chapter 35, Example Problem 2 (bicycle LOS): `test_example_problem_2_bicycle_los` asserts active passings per minute, meetings per minute, effective lanes (exactly), the probability of delayed passing, delayed passings per minute, the BLOS score (±0.01), and the LOS letter exactly, reproducing the documented `P_Tds` = 0.8334 and M1 = 5.38 (not the book's printed 5.36, per Deviation item 4).

Unit tests in `src/hcm/offstreet_pedbike/tests.rs` (28 tests) cover every step and exhibit lookup individually: `test_step1_determine_effective_walkway_width` through `test_step5_volume_to_capacity_ratio` for the exclusive-pedestrian-facility pipeline, plus `test_exhibit_24_1_walkway_random_flow_los_thresholds`, `test_exhibit_24_2_walkway_platoon_flow_los_thresholds`, `test_exhibit_24_3_stairway_los_thresholds`, and `test_cross_flow_los_ef_threshold` for the three LOS tables and the cross-flow capacity/threshold override; `test_step2_passing_and_meeting_events`, `test_step3_shared_path_pedestrian_los`, `test_one_way_path_has_no_meeting_events`, `test_peak_15min_flow_rates_bypass_phf`, and `test_exhibit_24_4_shared_path_pedestrian_los_thresholds` for the shared-use-path pedestrian methodology; and for the bicycle BLOS methodology, `test_step1_directional_flow_rates`, `test_step2_active_passings_per_minute`, `test_step3_meetings_per_minute`, `test_step4_effective_lanes_exhibit_24_14` (all three width bands), `test_step5_blocked_lane_and_pair_probabilities` and `test_step5_total_probability_of_delayed_passing` (spot-checking the 25-modal-pair combination directly), `test_step6_delayed_passings_per_minute`, `test_step7_blos_score_and_los`, `test_step8_low_volume_adjustment`, `test_exhibit_24_5_bicycle_los_thresholds`, `test_exclusive_bicycle_facility_zero_nonbike_modes`, `test_one_way_bicycle_path_has_no_meetings`, and `test_normal_cdf_reference_values` (the shared speed-distribution CDF helper, checked against known standard-normal reference values).

`tests/test_chapter24_integration.py` exercises the PyO3 bindings against the same two fixtures: `test_shared_use_path_pedestrian_los`, `test_exclusive_path_pedestrian_los`, `test_bicycle_los_on_shared_use_path`, and `test_low_volume_path_adjustment`.

## Deferred

Chapter 24's pedestrian and bicycle methodologies for facilities other than the three modeled here (e.g., any at-grade-crossing or motorized-interaction treatments, which belong to Chapters 18-23) are out of scope, consistent with the chapter's own "off-street" framing. Within the three implemented methodologies, no sub-procedure is explicitly deferred — the module doc comment's three "Methodologies" and their step lists are each implemented in full, including Step 8's low-volume LOS override for the bicycle methodology. The three passing-distance/sign/exhibit-boundary ambiguities documented under Deviations remain open interpretation questions (not implementation gaps) since no further published guidance or worked example exists to resolve them within this repository.
