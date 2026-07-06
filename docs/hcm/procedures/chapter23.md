# HCM Chapter 23, Part B — Interchange Ramp Terminals

This document walks through the Rust translation of HCM 7th Edition Chapter 23, Part B ("Interchange Ramp Terminals," final design and operational analysis for signalized interchanges including DDIs), which follows the nine computational steps of Exhibit 23-22 and is documented at the top of `src/hcm/ramp_terminals/ramp_terminals.rs`. Sources are EPUB `175_Ch23_pt2_03.xhtml` (core methodology), `176_Ch23_pt2_04.xhtml` (extensions), and `271_Ch34_04.xhtml` (the Chapter 34 O-D/turning-movement worksheets); numeric conventions are cross-checked against Chapter 34 Example Problems 1, 3, 4, 5, and 6 (`269_Ch34_02*.xhtml`). The code lives in `src/hcm/ramp_terminals/ramp_terminals.rs` (the `Interchange` facility struct and the nine-step pipeline) and `src/hcm/ramp_terminals/exhibits.rs` (LOS tables and the interchange-specific adjustment-factor equations); several saturation-flow-adjustment and delay building blocks are re-used directly from `crate::hcm::signalized` (lane-width/heavy-vehicle/parking/bus-blockage/area-type factors, default lane utilization, platoon ratio, back-of-queue terms) and `crate::hcm::common::delay` (uniform/incremental/initial-queue delay, progression factor, upstream filtering, roundabout control delay).

## Step-by-step walkthrough

| HCM Step | Equations/Exhibits | Rust function(s) | File | Inputs (units) | Outputs (units) |
|---|---|---|---|---|---|
| Step 1 — O-D and movement demands | Exhibit 23-20; Exhibits 34-163 through 34-177 (Chapter 34 worksheets) | `od_from_turning_movements`, `turning_movements_from_od`, `Interchange::step_1_od_and_movement_demands` | `ramp_terminals.rs` | `TurningMovements` or `OdDemands` (veh/h), `InterchangeForm` | `OdDemands`/`TurningMovements` (veh/h); `od_adjusted: Option<OdDemands>` (PHF-adjusted) |
| Step 2 — Lane groups | Chapter 19 rules (supplied as input) | `LaneGroupInput::new`, `Interchange::new` | `ramp_terminals.rs` | per-group `movement: InterchangeMovement`, `lanes`, `greens: Vec<GreenInterval>`, `LaneGroupControl` | `Vec<LaneGroupInput>` on `self.lane_groups` |
| Step 3 — Adjusted saturation flow rates | Eq. 23-14 with adjustments 23-15 through 23-23 | `Interchange::step_3_saturation_flows` (+ `lane_utilization`, `turn_proportions`, `arterial_lane_utilization_model`) | `ramp_terminals.rs`; factor functions in `exhibits.rs` | base sat flow (pc/h/ln), lane geometry, O-D turning proportions, area type, radii | `LaneGroupResult.sat_flow` (veh/h), `.lane_utilization`, `.traffic_pressure` |
| Step 4 — Effective green adjustments | Eqs. 23-24 through 23-39 | `Interchange::step_4_effective_green_adjustments` (+ `downstream_queue_length_ft`, `downstream_queue_lost_time`, `demand_starvation_initial_queue`, `demand_starvation_lost_time`, `ddi_overlap_lost_time`, `adjusted_lost_time`) | `ramp_terminals.rs` | feeding flows/lanes/greens (veh/h, ln, s), common green (s), queue spacing (ft) | `LaneGroupResult.downstream_queue_lost_time_s`, `.demand_starvation_lost_time_s`, `.adjusted_lost_time_s`, `.effective_green_s` (all s) |
| Step 5 — Closely spaced adjacent intersections | Eq. 23-40 | `adjacent_intersection_lost_time` (free function; not wired into the pipeline) | `ramp_terminals.rs` | green (s), distance to queue (ft), common green (s), cycle (s) | additional lost time L_D-Ui (s) |
| Step 6 — YIELD-controlled and free-flow turns | Eqs. 23-41 through 23-47, 23-53 through 23-56 | `yield_gap_acceptance_capacity`, `yield_no_conflict_capacity`, `yield_time_to_clear_queue_random`/`_coordinated`, `yield_clearance_time`, `yield_turn_capacity`, called from `Interchange::step_6_and_7_capacity_vc_queue` | `ramp_terminals.rs` | critical/follow-up headways (s), conflicting flow (veh/h), red/green (s), clearance distance/speed (ft, mi/h) | `LaneGroupResult.capacity` (veh/h), `.vc_ratio` |
| Step 7 — v/c ratio and queue storage ratio | Eq. 23-48; Chapter 31 §4 back-of-queue | `Interchange::step_6_and_7_capacity_vc_queue`, `assign_upstream_filtering`; finalized with d2 in Step 8 | `ramp_terminals.rs` | sat flow (veh/h), effective green (s), cycle (s) | `.capacity`, `.vc_ratio`, `.upstream_filtering` |
| Step 8 — Control delay and ETT | Eqs. 23-49/23-50; Chapter 19 d1/d2/d3 | `Interchange::step_8_control_delay`, `extra_distance_travel_time` | `ramp_terminals.rs` | v/c, effective green, cycle, arrival type, initial queue, EDTT distance/speed (ft, mi/h) | `.uniform_delay_s`/`.incremental_delay_s`/`.initial_queue_delay_s`/`.control_delay_s` (s/veh), `.back_of_queue_veh`, `.queue_storage_ratio` |
| Step 9 — LOS | Exhibit 23-10; Eqs. 23-51/23-52 | `Interchange::step_9_od_ett_and_los`, `od_path`, `los_signalized_interchange_od` | `ramp_terminals.rs`, `exhibits.rs` | per-O-D summed control delay + EDTT (s/veh), v/c>1 and R_Q>1 flags | `OdResult.ett_s`, `.los`; `self.interchange_ett_s`, `.interchange_los` |

### Step 1 — O-D <-> turning-movement conversion for all 8 forms

`InterchangeForm` enumerates the nine computational forms covered (`Diamond` also covers compressed/tight urban diamonds, which differ only in intersection spacing; `Ddi`; the four partial-cloverleaf variants `ParcloA2Q`/`ParcloA4Q`/`ParcloAB2Q`/`ParcloAB4Q`/`ParcloB2Q`/`ParcloB4Q` — six variants total; and `Spui`). `od_from_turning_movements(form, tm: &TurningMovements) -> OdDemands` implements the forward direction (Chapter 34 Exhibits 34-163 through 34-170) and `turning_movements_from_od(form, od: &OdDemands) -> TurningMovements` implements the algebraic inverse (Exhibits 34-171 through 34-177), both as an exhaustive `match` over `InterchangeForm` with one arm per form (Diamond/DDI share an arm since both use the Exhibit 34-169/34-176 diamond mapping; the six parclo variants and SPUI each have their own arm, so all eight distinct O-D <-> turning-movement mappings enumerated in the task are present). `TurningMovements` carries `_ii`-suffixed fields (e.g., `nb_left_ii`) for the AB/B-4Q parclo forms where a movement exists at both intersections. `OdDemands` (fields `a` through `n`, the fourteen Exhibit 23-20/34-162 O-D letters) provides `get(OdMovement)`, `phf_adjusted(phf)` (`v = V/PHF` on all fourteen letters), and `total()`.

```
PHF adjustment (Chapter 34 worksheet arithmetic; Exhibit 34-5 convention, `OdDemands::phf_adjusted`):
  v_x = V_x / PHF     for each O-D letter x in {A..N}     [veh/h]
  V_x  = unadjusted (peak 15-min-equivalent) O-D demand for letter x    (veh/h)
  PHF  = peak hour factor                                              (unitless, 0 < PHF <= 1; PHF <= 0 treated as 1.0)
  total() = Sigma_x v_x over all fourteen letters A..N                 (veh/h)
Implemented in: ramp_terminals/ramp_terminals.rs::OdDemands::phf_adjusted, ::total
```

The O-D <-> turning-movement conversion itself (`od_from_turning_movements` / `turning_movements_from_od`) is pure lookup-table data reproduced from Chapter 34 Exhibits 34-163 through 34-177 (and its inverse), not an equation: each interchange form gets one `match` arm that composes the fourteen O-D letters as fixed linear combinations (sums and differences) of the turning-movement fields, and the per-form arms are cross-checked against the exhibits rather than re-derived here.

### Step 3 — Saturation-flow adjustments

`step_3_saturation_flows` computes Equation 23-14 (`s = s0 x N x f_w x f_HVg x f_p x f_bb x f_a x f_LT x f_RT x f_v x f_LU x f_DDI`) per lane group, reusing the Chapter 19 factors (`f_w`, `f_HVg`, `f_p`, `f_bb`, `f_a`) directly and adding four interchange-specific adjustments: (1) traffic pressure `f_v` (Equation 23-15, `traffic_pressure_factor` in `exhibits.rs`, flow-weighted between the left-turn and through/right tabulated forms for shared lane groups, capped at `TRAFFIC_PRESSURE_DEMAND_CAP` = 30 veh/cycle/ln); (2) lane utilization `f_LU` (Equations 23-16/23-17 with Exhibit 23-24 for non-DDI external arterial approaches, dispatched by `arterial_lane_utilization_model` to one of five coefficient-row groupings; Equation 23-18 with Exhibit 23-26 for DDI external crossovers via `ddi_pct_v_lmax`; and the plain Chapter 19 Exhibit 19-15 default via `default_lane_utilization_factor` for ramp/internal/SPUI approaches); (3) turn-radius adjustments `f_LT`/`f_RT` (Equations 23-19 through 23-23, `turn_radius_factor` + `left_turn_radius_adjustment`/`right_turn_radius_adjustment`, flow-weighted across shared lane groups per the Chapter 23 text: "the adjustment factor for turn radii is estimated as the average (weighted on the basis of flows) of the respective movements"); and (4) the DDI crossover factor `f_DDI` (constant `F_DDI` = 0.913, applied only to the four DDI through-movement lane groups).

```
Equation 23-14:  s = s0 · N · f_w · f_HVg · f_p · f_bb · f_a · f_LT · f_RT · f_Lpb · f_Rpb · f_v · f_LU · f_DDI     [veh/h]
  s0     = base saturation flow rate per lane                        (pc/h/ln, default BASE_SATURATION_FLOW_METRO = 1,900 for metro pop. >= 250,000)
  N      = number of lanes in the lane group                          (ln)
  f_w    = lane-width adjustment (crate::hcm::signalized::exhibits::lane_width_factor)        (unitless)
  f_HVg  = heavy-vehicle/grade adjustment (::heavy_vehicle_grade_factor)                       (unitless)
  f_p    = parking adjustment (::parking_factor)                                               (unitless)
  f_bb   = bus-blockage adjustment (::bus_blockage_factor)                                     (unitless)
  f_a    = area-type adjustment (::area_type_factor; 0.90 CBD else 1.0)                        (unitless)
  f_LT   = left-turn adjustment, radius-modified (Eq. 23-20/23-21)                              (unitless)
  f_RT   = right-turn adjustment, radius-modified (Eq. 23-22/23-23)                             (unitless)
  f_Lpb  = pedestrian-bicycle adjustment for left turns (Chapter 19; not modeled, = 1.0)        (unitless)
  f_Rpb  = pedestrian-bicycle adjustment for right turns (Chapter 19; not modeled, = 1.0)       (unitless)
  f_v    = traffic-pressure adjustment (Eq. 23-15)                                              (unitless)
  f_LU   = lane-utilization adjustment (Eq. 23-16/23-17 or 23-18)                                (unitless)
  f_DDI  = DDI crossover adjustment (constant F_DDI = 0.913; DDI through lane groups only)      (unitless)
Implemented in: ramp_terminals/ramp_terminals.rs::Interchange::step_3_saturation_flows
```

```
Equation 23-15 (Interchange Saturation Flow Adjustment No. 1 — traffic pressure):
  f_v = 1 / (1.07 - 0.00672 · min(v_i', 30))     (left turn)
  f_v = 1 / (1.07 - 0.00486 · min(v_i', 30))     (through or right turn)
  v_i' = demand flow rate per cycle per lane                              (veh/cycle/ln)
  TRAFFIC_PRESSURE_DEMAND_CAP = 30                                        (veh/cycle/ln; v_i' above this uses 30)
  For a lane group shared by several movements, f_v is the flow-weighted average of the left-turn and through/right forms (Chapter 23 text; code: p_lt · f_v_left + (1 - p_lt) · f_v_thru).
Implemented in: ramp_terminals/exhibits.rs::traffic_pressure_factor; ramp_terminals/ramp_terminals.rs::Interchange::step_3_saturation_flows
```

```
Equations 23-16 / 23-17 (Interchange Saturation Flow Adjustment No. 2 — lane utilization, external arterial approaches of diamond/parclo interchanges):
Equation 23-16:  f_LU = 1 / (%V_Lmax · N)     [unitless]
  %V_Lmax = percent of total approach flow in the highest-volume lane, as a decimal   (unitless)
  N       = number of lanes in the lane group                                        (ln)

Equation 23-17:  %V_Li = 1/n + a1·(v_R/(v_L+v_R+v_T)) + a2·(v_L/(v_L+v_R+v_T)) + a3·(D · v_L / 10^6)     [unitless]
  %V_Li  = percent of traffic in lane Li (L1 = leftmost)                              (unitless)
  n      = number of lanes in the lane group                                          (ln)
  a1,a2,a3 = coefficients for the interchange-type/lane-position model (Exhibit 23-24) (unitless, ft^-1 for a3 scaling)
  D      = distance between the two intersections, capped at LANE_UTILIZATION_MAX_SPACING_FT = 800  (ft; valid below 800 ft)
  v_R    = O-D demand through the first intersection then turning right at the second (0 if an exclusive right-turn lane exists)  (veh/h)
  v_L    = O-D demand through the first intersection then turning left at the second   (veh/h)
  v_T    = O-D demand through both intersections                                       (veh/h)
  %V_Lmax = max(%V_Li) over the modeled lanes (leftmost, rightmost, and by-subtraction middle lane(s))
  Known deviation (already tracked): see Deviations item 1 (Eq. 23-17/Exhibit 23-24 does not reproduce the book's own %V_Lmax worked values).
Implemented in: ramp_terminals/exhibits.rs::lane_utilization_factor_from_max, ::pct_volume_in_lane, ::pct_v_lmax_arterial; ramp_terminals/ramp_terminals.rs::Interchange::lane_utilization
```

```
Equation 23-18 (DDI external-crossover lane utilization, with Exhibit 23-26 coefficients):
  %V_Li,DDI = a1 · LTDR + a2     [unitless]
  a1, a2 = coefficients for the DDI lane configuration and LTDR regime (Exhibit 23-26)   (unitless)
  LTDR   = left-turn demand ratio = left-turn demand at the external crossover / total approach volume   (decimal)
  The highest %V_Li,DDI over the configuration's regimes is used as %V_Lmax in Equation 23-16.
Implemented in: ramp_terminals/exhibits.rs::ddi_pct_v_lmax; ramp_terminals/ramp_terminals.rs::Interchange::lane_utilization
```

```
Equations 23-19 through 23-23 (Interchange Saturation Flow Adjustment No. 4 — turn radius):
Equation 23-19:  f_R = 1 / (1 + 5.61/R)     [unitless]
  R = radius of curvature of the left- or right-turning path at the center of the path   (ft)

Equation 23-20 (protected, exclusive left-turn lane):  f_LT = f_R
Equation 23-21 (protected, shared left-turn lane):      f_LT = 1 / (1 + P_LT · (1/f_R - 1))
  P_LT = proportion of left turns in the lane group (1.0 for an exclusive lane)   (decimal)

Equation 23-22 (protected, exclusive right-turn lane):  f_RT = f_R
Equation 23-23 (protected, shared right-turn lane):     f_RT = 1 / (1 + P_RT · (1/f_R - 1))
  P_RT = proportion of right turns in the lane group (1.0 for an exclusive lane)  (decimal)
  For a lane group shared by several movements, f_R is first flow-weighted across the group's movements (p · f_R_movement + (1 - p)), then entered into Eq. 23-21/23-23 (see Deviations item 8, which documents the flow-weighting convention reproducing Exhibit 34-7's published EB EXT-TH&R values).
Implemented in: ramp_terminals/exhibits.rs::turn_radius_factor, ::left_turn_radius_adjustment, ::right_turn_radius_adjustment; ramp_terminals/ramp_terminals.rs::Interchange::step_3_saturation_flows (shared_chain closure), ::turn_proportions
```

### Step 4 — The four lost-time / queue-interaction mechanisms

Four distinct mechanisms feed into the adjusted lost time `t_L'`/`t_L''` (Equations 23-24 through 23-26) and hence the effective green:

1. **Downstream internal queue** (Equations 23-29/23-30 with the queue-length Equations 23-33/23-34): `downstream_queue_length_ft` computes the average per-lane queue Q on the internal link at the start of the subject upstream phase from the *other* feeding movement's flow/lanes/green and the common green with the downstream through phase; `downstream_queue_lost_time` then converts the remaining distance-to-queue `DQ = D - Q` into lost time `L_D`, forced to zero once `DQ` exceeds `DOWNSTREAM_QUEUE_LOST_TIME_MAX_DISTANCE_FT` (200 ft). Wired into `step_4_effective_green_adjustments`'s "Pass 1" for the external arterial and ramp-left lane groups of both directions, skipped when a direct `downstream_queue_lost_time_s` input is supplied or the interchange is a DDI (which instead expects a shock-wave-estimate input per the DDI lost-time procedure).
2. **DDI overlap phasing** (Equation 23-37): `ddi_overlap_lost_time` computes `L_OL-DDI = (W + L - D)/(1.467 S_f)` for a signalized DDI off-ramp movement; this is documented as a `VERIFY-HCM` item (see Deviations) and in practice is supplied through the `overlap_lost_time_s` field on `LaneGroupInput` rather than always being computed inline.
3. **Demand starvation** (Equations 23-38/23-39): `demand_starvation_initial_queue` computes the initial internal queue `Q_initial` (veh) from the ramp-left and arterial feed flows/lanes, the common green windows `CG_RD`/`CG_UD` (each floored at the per-phase lost time `t_L`), and the internal saturation headway `h_I`; `demand_starvation_lost_time` then converts it to `L_DS = max(CG_DS - Q_initial h_I, 0)`. Wired into "Pass 2" for the internal through-movement lane groups of both directions, explicitly skipped for DDIs ("zero for DDIs per the Chapter 23 text").
4. **Closely spaced adjacent intersections** (Equation 23-40): `adjacent_intersection_lost_time` is a standalone free function (algebraically identical to `downstream_queue_lost_time`) exposed for an analyst to apply outside the pipeline, since — per its doc comment — "the facility pipeline models the interchange itself" and the full adjacent-intersection interaction (a further lane-utilization reduction plus a separate Chapter 19 evaluation of the adjacent intersection) is left to the analyst; the chapter itself directs the use of alternative tools when a downstream queue and demand starvation act on the same approach simultaneously.

```
Equations 23-24 / 23-25 / 23-26: adjusted lost time
Equation 23-24 (arterial):  t_L' = l1 + L_D-A + Y - e     [s]
Equation 23-25 (ramp):      t_L' = l1 + L_D-R + L_OL-DDI + Y - e     [s]
Equation 23-26 (internal):  t_L'' = l1 + L_DS + Y - e     [s]
  l1       = start-up lost time                                          (s, default START_UP_LOST_TIME = 2.0)
  L_D-A    = lost time on the external arterial approach due to a downstream queue (Eq. 23-29)   (s)
  L_D-R    = lost time on the external ramp approach due to a downstream queue (Eq. 23-30)        (s)
  L_OL-DDI = lost time on a signalized DDI off-ramp movement due to overlap phasing (Eq. 23-37)   (s)
  L_DS     = lost time due to demand starvation on an internal approach (Eq. 23-38)               (s)
  Y        = yellow-plus-all-red change-and-clearance interval                                   (s, `yellow_all_red_s`)
  e        = extension of effective green time into the clearance interval                        (s, default EXTENSION_OF_EFFECTIVE_GREEN = 2.0)
  Code combines L_D + overlap_lost_time_s into one "additional lost time" argument for external/ramp groups, and L_DS alone for internal groups (both floored at 0 by `adjusted_lost_time`).
Implemented in: ramp_terminals/ramp_terminals.rs::adjusted_lost_time, ::Interchange::step_4_effective_green_adjustments

Equations 23-27 / 23-28: effective green
Equation 23-27 (external, downstream-queue-adjusted):  g' = G + Y - t_L'     [s]
Equation 23-28 (internal, demand-starvation-adjusted):  g'' = G + Y - t_L''  [s]
  G = displayed green time (total_green_s)                                 (s)
Implemented in: ramp_terminals/ramp_terminals.rs::Interchange::step_4_effective_green_adjustments (`effective_green_s` field)
```

```
Equations 23-29 / 23-30 (Lost Time Adjustment No. 1 — downstream internal link queue on arterial/ramp):
Equation 23-29 (arterial):  L_D-A = G_A - 0.106·DQ_A - 5.39·CG_UD/C     [s, floored at 0]
Equation 23-30 (ramp):      L_D-R = G_R - 0.106·DQ_R - 5.39·CG_RD/C     [s, floored at 0]
  G_A, G_R = green interval for the external arterial / ramp approach                  (s)
  DQ_A, DQ_R = distance to the downstream queue at the start of the respective green (Eq. 23-31/23-32)  (ft)
  CG_UD  = common green time between the upstream arterial through and downstream through green   (s)
  CG_RD  = common green time between the upstream ramp and downstream through green               (s)
  C      = cycle length                                                                            (s)
  Both L_D-A and L_D-R are forced to 0 if negative, and forced to 0 if DQ_A/DQ_R > DOWNSTREAM_QUEUE_LOST_TIME_MAX_DISTANCE_FT = 200 ft.

Equations 23-31 / 23-32: DQ_A = D - Q_A;  DQ_R = D - Q_R     [ft]
  D = distance corresponding to storage space between the two intersections     (ft, `distance_between_intersections_ft`)
  Q_A, Q_R = average per-lane queue length on the internal link at the start of the arterial/ramp green (Eq. 23-33/23-34)   (ft)

Equations 23-33 / 23-34: average per-lane downstream queue length
  Q = (0.0107 · v_feed/N_feed - 7.96 · G_D/C - 0.082 · CG + 7.96 · G_feed/C) · L_h     [ft, floored at 0]
  v_feed, N_feed = flow (veh/h) and lanes of the *other* upstream movement feeding the internal link (ramp feed for Eq. 23-33/arterial phase; arterial feed for Eq. 23-34/ramp phase)
  G_feed = green interval of that feeding movement                              (s)
  G_D    = green interval of the downstream internal through movement           (s)
  CG     = common green (CG_UD for Eq. 23-33, CG_RD for Eq. 23-34) between the subject upstream movement and the downstream through green   (s)
  C      = cycle length                                                         (s)
  L_h    = average queue spacing in a stationary queue                          (ft/veh, default DEFAULT_QUEUE_SPACING_FT = 25)
Implemented in: ramp_terminals/ramp_terminals.rs::downstream_queue_length_ft, ::downstream_queue_lost_time, ::Interchange::step_4_effective_green_adjustments (Pass 1)
```

```
Equation 23-37 (Lost Time Adjustment No. 2 — DDI overlap phasing on a signalized off-ramp movement):
  L_OL-DDI = (W + L - D) / (1.467 · S_f)     [s, floored at 0]
  W   = width of the clear zone for the longest conflicting vehicle path, along the centerline of the outside lane   (ft)
  L   = design vehicle length, typically 20                                    (ft)
  D   = distance from the ramp movement stop bar to the conflict point         (ft)
  S_f = free-flow (design) speed of the vehicle                                (mi/h)
  Known deviation (already tracked): see Deviations item 2 (Chapter 34 Exhibit 34-63 implies (W + L + D); the printed -D form is implemented as-is).
Implemented in: ramp_terminals/ramp_terminals.rs::ddi_overlap_lost_time
```

```
Equations 23-38 / 23-39 (Lost Time Adjustment No. 3 — demand starvation on internal approaches; zero for DDIs):
Equation 23-38:  L_DS = CG_DS - Q_Initial · h_I     [s, floored at 0]
  CG_DS    = common green time with demand starvation potential                (s)
  Q_Initial = queue stored at the internal approach at the start of the demand-starvation interval (Eq. 23-39)   (veh)
  h_I      = saturation headway for the internal through approach = 3,600 / (internal saturation flow per lane)  (s)

Equation 23-39:
  Q_Initial = [ v_Ramp-L·C/(N_Ramp-L·3,600) - (CG_RD - t_L)/h_I ] + [ v_Arterial·C/(N_Arterial·3,600) - (CG_UD - t_L)/h_I ]     [veh]
  v_Ramp-L, v_Arterial = upstream ramp-left / arterial-through flow                 (veh/h)
  N_Ramp-L, N_Arterial = lanes for the upstream ramp-left / arterial-through movement (ln)
  C        = cycle length                                                          (s)
  CG_RD    = common green time between upstream ramp and downstream through green  (s, floored at t_L)
  CG_UD    = common green time between upstream arterial through and downstream through green   (s, floored at t_L)
  t_L      = lost time per phase (Eq. 23-24 / 23-25) of the feeding approaches      (s)
  h_I      = saturation headway for the internal through approach                  (s)
  Valid for CG_RD, CG_UD >= t_L (values below t_L are replaced by t_L); assumes no approach is oversaturated.
Implemented in: ramp_terminals/ramp_terminals.rs::demand_starvation_initial_queue, ::demand_starvation_lost_time, ::Interchange::step_4_effective_green_adjustments (Pass 2)
```

```
Equation 23-40 (Step 5 — closely spaced adjacent intersections; standalone free function, not wired into the pipeline):
  L_D-Ui = G_Ui - 0.106·DQ_i - 5.39·CG_UiD/C     [s, floored at 0]
  G_Ui   = green interval for upstream approach i                              (s)
  DQ_i   = distance to the downstream queue at the start of upstream green i, from Eq. 23-31/23-34's DQ/Q construction  (ft)
  CG_UiD = common green time between upstream approach i and the downstream through green   (s)
  C      = cycle length                                                        (s)
  Algebraically identical in form to Equations 23-29/23-30; the full adjacent-intersection interaction additionally subtracts 0.05 from the Chapter 19 lane-utilization factor and requires a separate Chapter 19 evaluation of the adjacent intersection (left to the analyst; see Deferred).
Implemented in: ramp_terminals/ramp_terminals.rs::adjacent_intersection_lost_time
```

### Step 6 — YIELD-controlled turn capacity (three regimes)

The YIELD-turn capacity model combines three distinct capacity regimes into one weighted-average capacity over the cycle (Equation 23-47): the **gap-acceptance regime** `c_GA` (Equation 23-42, the Siegloch form, `yield_gap_acceptance_capacity`, a function of critical/follow-up headway and conflicting flow); the **no-conflicting-flow regime** `c_NCF` (Equation 23-44, `yield_no_conflict_capacity = 3,600/t_f`, active whenever the conflicting movement is red); and the **queue-clearance/blocked regime**, where gap acceptance cannot begin until the conflicting queue clears (`t_CQ`, via `yield_time_to_clear_queue_random` for isolated/random-arrival interchanges — Equation 23-54 — or `yield_time_to_clear_queue_coordinated` for coordinated interchanges — Equation 23-55, which the doc comment notes "reduces to Equation 23-54 when P = g/C") plus the geometric clearance time for the last queued vehicle (`yield_clearance_time`, Equation 23-56). `yield_turn_capacity` assembles all three into `c_YCT = [c_GA (g - t_CQ - t_clear) + c_NCF (C - g)] / C`, flooring the gap-acceptance interval at zero per the Chapter 34 Example Problem 6 (Exhibit 34-68) convention.

```
Regime 1 — blocked by conflicting platoon (capacity c_b = 0; proportion of time blocked):
Equation 23-41 (iterative, time-step-based urban street procedure form):  p_b,x = t_p'/dt / C     [decimal]
  t_p'/dt = blocked period duration expressed in time steps                    (steps)
  dt      = time-step duration                                                 (s/step)
  C       = cycle length                                                       (s)
  Not computed by this stand-alone facility (no urban-street time-step engine); approximated instead by Equation 23-53 below.

Equation 23-53 (stand-alone-interchange approximation of p_b,x):  p_b,x' = (t_CQ + t_clear) / C     [decimal]
  t_CQ    = time to clear the conflicting queue (Eq. 23-54 or 23-55)            (s)
  t_clear = time for the last queued vehicle to clear the stop-bar-to-conflict-point distance (Eq. 23-56)   (s)
  C       = cycle length of the DDI crossover signal                           (s)
Implemented in: ramp_terminals/ramp_terminals.rs::yield_turn_capacity (the (g - t_CQ - t_clear) term folds p_b,x' and p_GA,x into one interval rather than computing them as separate proportions)
```

```
Regime 2 — gap acceptance in conflicting traffic:
Equation 23-42 (Siegloch):  c_GA = (3,600/t_f) · exp( -(t_c - t_f/2) · q_c / 3,600 )     [veh/h]
  t_c = critical headway            (s; Exhibit 23-36 defaults DDI_LEFT_CRITICAL_HEADWAY_S = 3.9 left, DDI_RIGHT_CRITICAL_HEADWAY_S = 1.8 right)
  t_f = follow-up headway           (s; defaults DDI_LEFT_FOLLOW_UP_HEADWAY_S = 2.6 left, DDI_RIGHT_FOLLOW_UP_HEADWAY_S = 2.4 right)
  q_c = conflicting flow rate       (veh/h)
Implemented in: ramp_terminals/ramp_terminals.rs::yield_gap_acceptance_capacity

Equation 23-43 (proportion of time in the gap-acceptance regime):  p_GA,x = (g - (t_CQ + t_clear)) / C     [decimal]
  g   = effective green time of the DDI crossover movement          (s)
  t_CQ, t_clear as above; C = cycle length of the crossover signal  (s)
Implemented in: ramp_terminals/ramp_terminals.rs::yield_turn_capacity (folded into the ga_time term, floored at 0)
```

```
Regime 3 — no conflicting flow:
Equation 23-44:  c_NCF = 3,600 / t_f     [veh/h]
Implemented in: ramp_terminals/ramp_terminals.rs::yield_no_conflict_capacity

Equation 23-45 (proportion of time with no conflicting flow):  p_NCF,x = r/C = (C - g)/C = 1 - g/C     [decimal]
  r = effective red time of the DDI crossover movement   (s)
  g, C as above
Implemented in: ramp_terminals/ramp_terminals.rs::yield_turn_capacity (folded into the (C - g) term)
```

```
Combined YIELD-turn capacity:
Equation 23-46 (weighted-sum form):  c_YCT = c_b·p_b,x + c_GA·p_GA,x + c_NCF·p_NCF,x     [veh/h]
  c_b = capacity during the blocked regime = 0

Equation 23-47 (simplified, implemented form):  c_YCT = (1/C) · [ c_GA·(g - t_CQ - t_clear) + c_NCF·(C - g) ]     [veh/h]
  All symbols as defined above; the gap-acceptance interval (g - t_CQ - t_clear) is floored at 0 (Chapter 34 Example Problem 6, Exhibit 34-68 note).
Implemented in: ramp_terminals/ramp_terminals.rs::yield_turn_capacity
```

```
Time to clear the conflicting queue (feeds t_CQ above):
Equation 23-54 (isolated interchange, random arrivals):  t_CQ,free = r · v_app / (s_DDI - v_app)     [s]
  r      = duration of the effective red interval for the conflicting movement     (s)
  v_app  = conflicting approach flow rate                                          (veh/h)
  s_DDI  = saturation flow rate for the DDI approach                               (veh/h)
  If s_DDI <= v_app the queue never clears; code returns r (the full red).
Implemented in: ramp_terminals/ramp_terminals.rs::yield_time_to_clear_queue_random

Equation 23-55 (coordinated interchange):  t_CQ,coord = C·(1 - P) / [ s_DDI/v_app - P·(g/C)^-1 ]     [s]
  P = proportion of conflicting arrivals during green                              (decimal)
  g = duration of the effective green interval for the conflicting movement        (s)
  C = cycle length of the crossover signal                                         (s)
  Reduces algebraically to Equation 23-54 when P = g/C. Valid only when: P·v_app·(C/g) < S_DDI (equivalently P < S_DDI·g/(v_app·C)); v_app·C <= S_DDI·g (undersaturated); and t_CQ,coord <= g.
Implemented in: ramp_terminals/ramp_terminals.rs::yield_time_to_clear_queue_coordinated
```

```
Equation 23-56 (clearance time for the last queued vehicle):
  t_clear = x_clear / (1.47 · S_f,DDI)     [s]
  x_clear   = distance between the DDI crossover stop bar and the yield conflict point   (ft)
  S_f,DDI   = free-flow speed between the stop bar and the conflict point                (mi/h)
  Validated against Chapter 34 Exhibit 34-67 (200 ft at 25 mi/h -> 5.5 s).
Implemented in: ramp_terminals/ramp_terminals.rs::yield_clearance_time
```

### Step 8 — Control delay and O-D aggregation

`step_8_control_delay` dispatches per lane group on `LaneGroupControl`: `FreeFlow` groups carry zero control delay (Chapter 23 Step 6 text); `YieldControlled` groups use the Chapter 22 roundabout control-delay procedure (`common::delay::control_delay_roundabout`, Equation 22-17) per the Chapter 23 Step 8 text and Chapter 34 Example Problem 6's explicit instruction to do so, though this is flagged `VERIFY-HCM` since Exhibit 34-70's published delays are not reproducible from Equation 22-17 (see Deviations); `Signalized` groups use the standard Chapter 19 uniform delay (`uniform_delay`, Equation 19-19, with `progression_factor` for P = R_p g/C), incremental delay evaluated **on a per-lane basis** (`incremental_delay_signalized` called with `capacity/lanes`, matching the Chapter 34 interchange worksheets' per-lane saturation-flow/d2 convention rather than the lane-group capacity — a documented `VERIFY-HCM` convention choice, see Deviations), and initial-queue delay (`initial_queue_delay`). Back-of-queue and queue storage ratio reuse the Chapter 19/31 building blocks directly (`first_term_back_of_queue`, `second_term_back_of_queue`, `average_vehicle_spacing`, `queue_storage_ratio_eq`). Step 9's `od_path(OdMovement) -> Vec<InterchangeMovement>` maps each of the fourteen O-D letters to the ordered sequence of lane groups it traverses (form- and SPUI-dependent; DDI left-onto-freeway movements traverse only the external crossover since the internal crossover is free-flowing for that path), and `step_9_od_ett_and_los` sums each path's control delays, adds the Equation 23-50 extra-distance travel time (`extra_distance_travel_time`, EDTT — negative for right turns per the Exhibit 23-8 sign convention), looks up LOS via `los_signalized_interchange_od` (Exhibit 23-10, forcing a stricter LOS whenever any traversed lane group has v/c or queue-storage-ratio exceeding 1.0), and aggregates the demand-weighted interchange-level ETT and LOS (Equations 23-51/23-52).

```
Equation 23-48 (v/c ratio for lane group i, Step 7):
  X_i = (v/c)_i = v_i / (s_i · (g_i/C)) = v_i·C / (s_i·g_i)     [unitless]
  v_i = actual or projected demand flow rate for lane group i     (veh/h)
  s_i = saturation flow rate for lane group i (Eq. 23-14)         (veh/h)
  g_i = effective green time for lane group i; use g' (Eq. 23-27) if a downstream-queue lost time applies, g'' (Eq. 23-28) if a demand-starvation lost time applies   (s)
  C   = cycle length                                              (s)
  For YIELD-controlled turns, capacity is c_YCT (Eq. 23-47) directly rather than s_i·g_i/C.
Implemented in: ramp_terminals/ramp_terminals.rs::Interchange::step_6_and_7_capacity_vc_queue
```

```
Equation 23-13 / 23-49 (experienced travel time per O-D movement, identical form introduced early in the chapter and restated at Step 8):
  ETT = Sigma d_i + Sigma EDTT     [s/veh]
  d_i  = control delay at junction i encountered on the O-D's path through the interchange (Chapter 19 d1+d2+d3 for signalized groups, Chapter 22 Eq. 22-17 control_delay_roundabout for YIELD-controlled groups, 0 for free-flow groups)   (s/veh)
  EDTT = extra distance travel time for any diverted path segment (Eq. 23-50)   (s/veh)
Implemented in: ramp_terminals/ramp_terminals.rs::Interchange::step_9_od_ett_and_los
```

```
Equation 23-50 (extra distance travel time):
  EDTT = D_t / (1.47 · v_D) + a     [s]
  D_t = signed distance traveled along the diverted movement (loop ramp, or extra distance from the interchange centerline for diamonds/DDIs), positive for left turns / small positive for DDI arterial crossings, negative for right turns (Exhibit 23-8 sign convention)   (ft)
  v_D = design speed of the loop ramp or diverted movement                     (mi/h)
  a   = deceleration/acceleration delay into and out of the turn, EDTT_LOOP_RAMP_ACCEL_DECEL_S = 5 for a loop ramp movement, 0 in the Chapter 34 diamond examples   (s)
Implemented in: ramp_terminals/ramp_terminals.rs::extra_distance_travel_time
```

```
Equations 23-51 / 23-52 (demand-weighted ETT aggregation; Equation 23-51 is per-approach and structurally identical to Equation 23-52, which the code implements at the interchange level):
Equation 23-51 (approach):     ETT_A = Sigma_j(ETT_j · v_j) / Sigma_j(v_j)     [s/veh],  j in {movements on the approach}
Equation 23-52 (interchange):  ETT_I = Sigma_k(ETT_k · v_k) / Sigma_k(v_k)     [s/veh],  k in {all movements at the interchange}
  ETT_j / ETT_k = experienced travel time for movement j / k     (s/veh)
  v_j / v_k     = demand flow rate for movement j / k            (veh/h)
Implemented in: ramp_terminals/ramp_terminals.rs::Interchange::step_9_od_ett_and_los (interchange-level Eq. 23-52 only; Eq. 23-51's per-approach aggregation is not separately exposed)
```

## Deviations (cross-referenced to `docs/hcm/VERIFICATION.md`)

`docs/hcm/VERIFICATION.md` exists at this branch's tip; its "Chapter 23 (feat/hcm-ch23-ramp-terminals)" section, together with the inline `// VERIFY-HCM` comments in `ramp_terminals.rs`, records: (1) the Equation 23-17/Exhibit 23-24 lane-utilization model does not reproduce the book's own worked values (Chapter 34 Example 6: 0.497 computed vs. 0.5056 published; Example 3: 0.625 vs. 0.5551) — the printed equation is implemented as-is, with an `lane_utilization_override` input available for fixtures/callers that need the published value directly; (2) Equation 23-37 (DDI overlap lost time) is printed as `(W + L - D)/(1.467 S_f)` but Chapter 34 Exhibit 34-63 implies `(W + L + D)` (the published 6.5/4.9 s values match the `+D` form) — the printed `-D` form is implemented, and fixtures supply the published overlap lost times directly via `overlap_lost_time_s`; (3) incremental delay d2 is evaluated on a per-lane basis (`capacity/lanes`) rather than the lane-group basis Chapter 34 Example 5 (DDI) appears to use — the two conventions differ for multilane groups near saturation, and the per-lane convention is what reproduces the Chapter 34 Exhibits 34-12 through 34-15 per-lane saturation-flow/d2 tables; (4) Exhibit 34-70's published YIELD-turn delays are not reproducible from Equation 22-17 even though the capacities (Equation 23-47/`c_YCT`) all reproduce exactly — e.g., M7 computes 9.1 s vs. published 34.7 s at `c_YCT` = 795 veh/h; (5) Chapter 34 Example 5's published DDI uniform delays are internally inconsistent with Equation 19-19 under any tabulated arrival type — equation-based results are asserted instead (9 of 10 O-D LOS letters still match the published table; O-D E computes LOS C at 33.9 s/veh vs. published LOS B at 24.7 s/veh, attributed in the integration test comment to the per-lane incremental delay on the 3-lane external crossover at X = 0.84); (6) Example 5 applied the through-movement Equation 23-15 traffic-pressure form to the ramp-left movements where the left-turn form seems intended — the left-turn form is implemented as printed (approximately a 1% delta on the affected saturation flows); (7) Exhibit 34-9's common-green value (CGRD = 34) counts only the phase-3 overlap, while a literal interval-intersection of the two green windows gives 39 s (`common_green_time` implements the literal interval intersection; no effect on any published outcome since the discrepancy occurs off the critical path); (8) the shared-lane-group right-turn saturation-flow-adjustment convention flow-weights `f_R` via Equation 23-23 (matching the Exhibit 34-7 published EB EXT-TH&R values of f_R = 0.991/f_RT = 0.999), which is the convention `shared_chain`/`turn_proportions` implement.

## Validation

Fixtures live under `tests/ExampleCases/hcm/RampTerminals/` as `case1.json` and `case2.json`, exercised by `tests/chapter23_integration.rs` and mirrored by `tests/test_chapter23_integration.py`. `case1.json` reproduces Chapter 34, Example Problem 1 (conventional diamond interchange, published O-D results in Exhibit 34-16): `test_case1_diamond_lane_groups` and `test_case1_diamond_od_results` assert, for all ten nonzero O-D movements (A through J), demand at ±1.0 veh/h, control delay at ±1.0 s/veh, EDTT at ±0.1 s/veh, ETT at ±1.0 s/veh, and LOS exactly, plus that no O-D has v/c or queue-storage-ratio exceeding 1.0; the interchange-level ETT is asserted at 52.4 s/veh ±1.0 against the Exhibit 34-16 totals row, with LOS C exactly. `case2.json` reproduces Chapter 34, Example Problem 5 (DDI with signal control, published results in Exhibits 34-62/34-63/34-65): `test_case2_ddi_results` asserts adjusted saturation flows against the Exhibit 34-62 lane-group totals (tolerances 5-55 veh/h/ln depending on lane group, the external crossovers carrying the widest tolerance due to the documented f_LU-rounding and traffic-pressure-form deviations above), effective green times against Exhibit 34-63 (±0.1 s, with several lane groups computing slightly above the book's rounded-down published values), zero demand-starvation lost time for the DDI internal through movement, and O-D ETT/LOS against the equation-based expectations (±0.5 s/veh) with the published Exhibit 34-65 values recorded inline in code comments for every O-D — nine of ten O-D LOS letters match the published table exactly, with O-D E being the one documented exception (see Deviations). The demand-weighted interchange ETT is asserted at 34.9 s/veh ±0.5 with LOS C. `test_serde_round_trip` confirms a fully analyzed `Interchange` round-trips through JSON with results intact.

Unit tests in `src/hcm/ramp_terminals/tests.rs` (26 tests) spot-check individual equations and exhibits directly: the three LOS tables (`test_exhibit_23_10_signalized_interchange_los`, `test_exhibit_23_13_alternative_intersection_los`, `test_exhibit_23_14_roundabout_interchange_los`); the traffic-pressure and turn-radius factor tables (`test_exhibit_23_23_traffic_pressure`, `test_exhibit_23_27_turn_radius`, `test_turn_radius_adjustments`); the lane-utilization equations (`test_equation_23_16_lane_utilization_factor`, `test_equation_23_17_diamond_lane_utilization`, `test_exhibit_23_24_three_lane_model`, `test_exhibit_23_26_ddi_lane_utilization`); the common-green helper (`test_exhibit_34_9_common_green`, exercising the Exhibit 34-9 discrepancy noted above); the four Step 4 lost-time mechanisms individually (`test_equation_23_34_downstream_queue_length`, `test_equation_23_30_downstream_queue_lost_time`, `test_equations_23_38_39_demand_starvation`, `test_adjusted_lost_time_and_effective_green`, `test_equation_23_37_ddi_overlap_lost_time`); the YIELD three-regime capacity chain (`test_exhibit_34_67_blocked_regime`, `test_equation_23_55_coordinated_reduces_to_random`, `test_exhibit_34_68_gap_acceptance_capacity`, `test_equation_23_47_yield_capacity`); the EDTT equation (`test_equation_23_50_edtt`); the O-D <-> turning-movement conversion (`test_exhibit_34_176_diamond_turning_movements`, `test_od_turning_movement_round_trip` — round-tripping `od_from_turning_movements`/`turning_movements_from_od` for multiple forms); the PHF adjustment and roundabout-interchange O-D movement table (`test_exhibit_34_5_phf_adjustment`, `test_exhibit_34_161_roundabout_movements`); and pipeline-level smoke/regression tests (`test_step_1_lane_group_demands`, `test_full_pipeline_smoke`, `test_step_4_demand_starvation_engaged`).

`tests/test_chapter23_integration.py` exercises the PyO3 bindings against both fixtures: `test_od_results`, `test_interchange_ett_and_los`, `test_lane_group_results`, `test_json_round_trip` for the diamond (`case1.json`), and `test_od_los`, `test_free_flow_right_turns`, `test_interchange_ett_and_los`, `test_ddi_saturation_flows` for the DDI (`case2.json`).

## Deferred

Step 5 (closely spaced adjacent intersections, Equation 23-40) is implemented only as the standalone `adjacent_intersection_lost_time` free function; the full adjacent-intersection interaction — an additional lane-utilization reduction of 0.05 plus a separate Chapter 19 evaluation of the adjacent intersection itself, and the chapter's own direction to use alternative tools when downstream-queue and demand-starvation effects coincide on one approach — is left to the analyst rather than wired into the `Interchange` pipeline. Roundabout-controlled interchange ramp terminals (Exhibit 23-14 LOS criteria are implemented via `los_roundabout_interchange_od`, and `roundabout_movement_ods` maps O-D letters to roundabout entry/exit movements, but no `Interchange`-equivalent roundabout-interchange facility struct or pipeline exists on this branch) are exposed only at the exhibit/lookup level. Part C (alternative intersections: RCUT/MUT/DLT) is out of scope for this branch and documented separately in `chapter23-alternative.md` on `feat/hcm-ch23-alternative-intersections`.
