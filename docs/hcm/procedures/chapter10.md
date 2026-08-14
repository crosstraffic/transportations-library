# HCM Chapter 10: Freeway Facilities Core Methodology

This document walks a reviewer through the Rust implementation of the HCM 7th Edition Chapter 10 core freeway facility methodology (motorized vehicle, general-purpose lanes), which orchestrates the Chapter 12 basic-segment, Chapter 13 weaving, and Chapter 14 merge/diverge segment engines over an ordered set of segments and consecutive 15-minute analysis periods (the time-space domain of Exhibit 10-10), following Exhibit 10-8's Steps A-1 through A-17. When any segment's demand-to-capacity ratio exceeds 1.00, the facility switches to the Chapter 25 Section 4 node-segment time-step engine (Equations 25-6 through 25-34) rather than the Chapter 25 Section 3 undersaturated closed-form evaluation. The code lives in `src/hcm/freeway_facilities/freeway_facilities.rs` (orchestration and undersaturated evaluation), `src/hcm/freeway_facilities/oversaturated.rs` (the 15-second time-step engine), and `src/hcm/freeway_facilities/exhibits.rs` (equation transcriptions, LOS tables, and work zone models). Managed-lane facilities (Steps A-9/A-13/A-14) and the Chapter 25 planning-level method are explicitly out of scope in this pass and are covered separately (see `docs/hcm/procedures/chapter10-managed-lanes.md`, implemented at `src/hcm/freeway_facilities/managed_lanes.rs` and `src/hcm/freeway_facilities/planning.rs`). Deviations from the printed manual that are already catalogued are cross-referenced below as `VERIFICATION.md ch10/25 item N` (see `docs/hcm/VERIFICATION.md`, "Chapter 10 / 25 oversaturated engine" section); any deviation found in this pass that is not already catalogued there is marked `**DISCREPANCY:**` inline.

## Step-by-step walkthrough

| HCM Step | Equations / Exhibits | Rust location | Notes |
|---|---|---|---|
| A-2 segmentation | Exhibits 10-1, 10-2, 10-11, 10-12 | `freeway_facilities.rs::segment_ramp_section` | Converts a gore-to-gore section into `(SegmentType, length_ft)` pieces. |
| A-3/A-4 demand balancing and accumulation | Eq 10-2, 10-3 (balancing); demand accumulation | `exhibits.rs::time_interval_scale_factor`, `exhibits.rs::balance_exit_demands`, `freeway_facilities.rs::FreewayFacility::compute_demands` | Balancing is a caller-invoked preprocessing step, separate from `compute_demands`. |
| A-6 global parameters | Exhibit 10-7 defaults | `exhibits.rs::DEFAULT_JAM_DENSITY_PC`, `DEFAULT_QUEUE_DISCHARGE_DROP`, `DEFAULT_TIME_STEP_S` | 190 pc/mi/ln, 7%, 15 s. |
| A-7/A-8 capacities | Eq 10-4, 10-5, 10-6 (CAF/SAF/DAF), Eq 12-6 (base capacity), Exhibit 14-10 (ramp capacity) | `freeway_facilities.rs::FreewayFacility::compute_capacities`, `::effective_caf`, `::effective_saf`, `::base_capacity_pc` | Weaving segment capacities recompute per period via the Chapter 13 engine; work zone CAF/SAF (Eq 10-11/10-12) multiply the calibration factors. |
| A-10 demand/capacity ratios and oversaturation detection | vd/c screening | `freeway_facilities.rs::FreewayFacility::compute_dc_ratios` | Sets `first_oversat_period` at the first cell with vd/c > 1.0. |
| A-11 undersaturated evaluation | Chapter 25 Section 3; Eq 25-1 (max achievable speed) | `freeway_facilities.rs::FreewayFacility::analyze_undersaturated_period`, `::evaluate_period_chain`, `::engine_eval`, `exhibits.rs::max_achievable_speed` | Runs per period until `first_oversat_period`. |
| A-12 oversaturated evaluation | Chapter 25 Section 4; Eq 25-6 through 25-34 | `freeway_facilities.rs::FreewayFacility::analyze_oversaturated`, `oversaturated.rs::OversaturatedEngine::run_period` | The 15-s time-step engine; see the dedicated subsection below. |
| A-15/A-17 facility aggregation and LOS | Eq 10-1, 25-2, 25-4, 25-5; Exhibit 10-6 | `freeway_facilities.rs::FreewayFacility::compute_facility_performance`, `::overall_space_mean_speed`, `::overall_density_veh`, `exhibits.rs::facility_density`, `exhibits.rs::facility_space_mean_speed`, `exhibits.rs::los_freeway_facility` | Facility LOS is F whenever any component segment has vd/c > 1.00, regardless of density (Exhibit 10-6). |
| Work zones | Eq 10-7 through 10-12; Exhibit 10-15 | `exhibits.rs::WorkZone`, `::lcsi`, `::queue_discharge_rate`, `::capacity_pc`, `::ffs`, `::caf`, `::saf` | NCHRP 03-107 lane-closure-severity-index model; `lane_closure_severity_index` reproduces the Exhibit 10-15 LCSI table for 3-to-3, 2-to-2, 4-to-3, 3-to-2, 4-to-2, and 2-to-1 closures. |

### Segmentation (Step A-2)

`segment_ramp_section(gore_to_gore_ft, has_auxiliary_lane)` implements the Exhibit 10-11/10-12 decision tree: an auxiliary lane between gores makes the whole section one weaving segment; spacing beyond 3,000 ft (2 x the 1,500-ft `RAMP_INFLUENCE_AREA_FT`) yields merge + basic + diverge; spacing between 1,500 and 3,000 ft yields merge + `OverlappingRamp` + diverge, where the overlap piece is `2 x 1500 - spacing`; and spacing at or below 1,500 ft collapses to a single `OverlappingRamp` segment spanning the whole distance (the code comment calls this "highly unusual" since it implies no auxiliary lane over a sub-influence-area gap). `FacilitySegment` itself is not auto-segmented from a raw gore list in this module — callers construct the `Vec<FacilitySegment>` directly (as the example fixtures do), and `segment_ramp_section` is a helper for producing the length breakdown, not a full facility builder.

No HCM equation governs segmentation itself — Exhibits 10-1, 10-2, 10-11, and 10-12 define it entirely through the geometric decision rules already described in the paragraph above (the 1,500-ft ramp influence area, the 3,000-ft and 1,500-ft gore-to-gore breakpoints, and the auxiliary-lane weaving rule), which is why `segment_ramp_section` returns `(SegmentType, length_ft)` tuples rather than evaluating a numbered formula.
Implemented in: freeway_facilities/freeway_facilities.rs::segment_ramp_section

### Demand accumulation (Steps A-3/A-4)

`compute_demands` accumulates segment demand SD(i, p) by walking segments upstream to downstream, adding on-ramp demand and subtracting off-ramp demand at each node (`onrd_by_node`/`offrd_by_node`), scaled by each segment's own DAF (`FacilitySegment::on_demand`/`off_demand`/`rr_demand` multiply by `self.daf`, i.e. Equation 10-6 is applied to a segment's own ramp demands, not globally). Demand balancing (Eq 10-2/10-3, reconciling inconsistent entering/exiting ramp counts) is implemented as standalone functions (`time_interval_scale_factor`, `balance_exit_demands`) that a caller would run before populating `on_ramp_demand`/`off_ramp_demand`; `compute_demands` itself assumes demands are already balanced, per its doc comment.

Equation 10-2:  f_TIS,i = ( Σ_j VON15(i,j) ) / ( Σ_j VOFF15(i,j) )                                          [dimensionless]
  f_TIS,i = time interval scale factor for analysis period i                                                [dimensionless]
  VON15(i,j) = 15-min entering count for analysis period i, entering location j                             [veh]
  VOFF15(i,j) = 15-min exit count for analysis period i, exiting location j                                 [veh]
A ratio greater than 1.00 indicates congestion inside the facility is suppressing exit counts below true exit demand.
Implemented in: freeway_facilities/exhibits.rs::time_interval_scale_factor

Equation 10-3:  VdOFF15(i,j) = VOFF15(i,j) * f_TIS,i                                                        [veh]
  VdOFF15(i,j) = adjusted (balanced) 15-min exit demand for analysis period i, exiting location j           [veh]
  VOFF15(i,j), f_TIS,i = as in Equation 10-2
Implemented in: freeway_facilities/exhibits.rs::balance_exit_demands (standalone preprocessing step; `compute_demands` does not call it, per its own doc comment)

Equation 10-6:  v_adj = v * DAF_cal                                                                          [veh/h]
  v_adj = adjusted demand input volume                                                                       [veh/h]
  v = base demand volume                                                                                     [veh/h]
  DAF_cal = calibration demand adjustment factor                                                             [decimal]  (default 1.0; primarily used in a Chapter 11 reliability analysis, per the Step A-8 text)
Implemented in: freeway_facilities/exhibits.rs::adjusted_demand (the standalone, generic form); in this module DAF is applied per-segment to that segment's own ramp demands by `FacilitySegment::on_demand`/`off_demand`/`rr_demand` (each multiplies by `self.daf`) rather than as a single facility-wide multiplier on `v`

### Capacities (Steps A-7/A-8)

`compute_capacities` computes per-segment, per-period capacity in veh/h. Basic and `OverlappingRamp` segments use `base_capacity_pc` (Equation 12-6, `2200 + 10*(FFS - 50)` capped at 2,400 pc/h/ln, or `c_ifl_override` if supplied) times CAF times lanes times f_HV. Merge/Diverge segments use `get_freeway_capacity_per_lane` from the Chapter 14 module (Exhibit 14-10). Every one of these reads the UNADJUSTED segment FFS, per the December 2022 correction to Equations 12-6/12-7; a SAF reaches capacity only through the CAF, never twice, and the weaving engine applies its own SAF internally. Weaving segments build a full `WeavingSegment` (Chapter 13 engine) per period and call `determine_capacity()` if the segment still operates as a weave after `determine_max_weaving_length()`; otherwise it falls back to the basic-segment capacity formula, consistent with Exhibit 10-12(b) ("L_S >= L_MAX: operates as a basic segment"). `effective_caf`/`effective_saf` layer a work zone's `caf()`/`saf()` (Equations 10-11/10-12) on top of any per-period `caf_schedule`/`saf_schedule` or scalar `caf`/`saf` calibration factor.

Equation 10-4:  FFS_adj = FFS * SAF_cal                                                                     [mi/h]
  FFS_adj = adjusted free-flow speed                                                                        [mi/h]
  FFS = base segment (or facility) free-flow speed                                                          [mi/h]
  SAF_cal = calibration speed adjustment factor                                                              [decimal]  (default 1.0; should be <= 1.0 per the Step A-8 text)
Implemented in: freeway_facilities/exhibits.rs::adjusted_ffs (standalone generic form); applied per segment/period as `self.seg_ffs(i) * self.effective_saf(i, p)` by freeway_facilities/freeway_facilities.rs::FreewayFacility::compute_capacities and ::engine_eval

Equation 10-5:  c_adj = c * CAF_cal                                                                          [veh/h or pc/h/ln]
  c_adj = adjusted capacity                                                                                  [veh/h or pc/h/ln]
  c = base capacity                                                                                          [veh/h or pc/h/ln]
  CAF_cal = calibration capacity adjustment factor                                                            [decimal]  (default 1.0; should be <= 1.0 per the Step A-8 text)
Implemented in: freeway_facilities/exhibits.rs::adjusted_capacity (standalone generic form); applied per segment/period via freeway_facilities/freeway_facilities.rs::FreewayFacility::effective_caf, which multiplies the calibration CAF (scalar `caf` or `caf_schedule`) by the segment's own base-capacity call

Work zone models (Chapter 10, Section 4; NCHRP 03-107), used by `effective_caf`/`effective_saf` when a segment carries a `WorkZone`:

Equation 10-7:  LCSI = 1 / (OR * N_o)                                                                        [decimal]  (capped at 2.0 for severe closures such as 3-to-1 or 4-to-1)
  LCSI = lane closure severity index                                                                          [decimal]
  OR = open ratio = N_o / N_total                                                                             [decimal]
  N_o = number of open lanes through the work zone                                                            [ln]
  N_total = normal (total) number of lanes upstream of the work zone                                          [ln]
Exhibit 10-15 values reproduced by this equation (the values transcribed as Rust test fixtures, not a lookup table): 3-to-3 = 0.33; 2-to-2 = 0.50; 4-to-3 = 0.44; 3-to-2 = 0.75; 4-to-2 = 1.00; 2-to-1 = 2.00.
Implemented in: freeway_facilities/exhibits.rs::lane_closure_severity_index, freeway_facilities/exhibits.rs::WorkZone::lcsi

Equation 10-8:  QDR_wz = 2093 - 154*LCSI - 194*f_Br - 179*f_AT + 9*f_LAT - 59*f_DN                           [pc/h/ln]
  QDR_wz = average 15-min work zone queue discharge rate                                                      [pc/h/ln]
  LCSI = lane closure severity index (Equation 10-7)                                                           [decimal]
  f_Br = barrier type indicator: 0 = concrete/hard barrier, 1 = cone/plastic drum/soft barrier
  f_AT = area type indicator: 0 = urban, 1 = rural
  f_LAT = lateral distance from the travel-lane edge to the barrier/barricades/cones                          [ft]  (range 0-12)
  f_DN = time-of-day indicator: 0 = daylight, 1 = night
Implemented in: freeway_facilities/exhibits.rs::WorkZone::queue_discharge_rate

Equation 10-9:  c_wz = QDR_wz / (100 - alpha_wz) * 100                                                        [pc/h/ln]  (capped at the non-work-zone capacity c)
  c_wz = work zone capacity (prebreakdown flow rate)                                                           [pc/h/ln]
  QDR_wz = as in Equation 10-8                                                                                 [pc/h/ln]
  alpha_wz = percentage drop in prebreakdown capacity at the work zone due to queuing                          [%]  (default 13.4%, NCHRP 03-107; the non-work-zone default queue discharge drop is 7%, Equation 25-29/Step A-6)
Implemented in: freeway_facilities/exhibits.rs::WorkZone::capacity_pc (the code stores `queue_discharge_drop` as a decimal fraction, e.g. 0.134, and computes `QDR_wz / (1.0 - queue_discharge_drop)`, which is algebraically identical to the printed `/(100-alpha_wz)*100` form when alpha_wz is expressed as a fraction rather than a percentage)

Equation 10-10:  FFS_wz = 9.95 + 33.49*f_Sr + 0.53*SL_wz - 5.60*LCSI - 3.84*f_Br - 1.71*f_DN - 8.7*TRD        [mi/h]  (1 <= f_Sr <= 1.2; result capped at the non-work-zone FFS)
  FFS_wz = work zone free-flow speed                                                                          [mi/h]
  f_Sr = speed ratio = non-work-zone speed limit / work zone speed limit                                      [decimal]  (clamped to 1.0-1.2)
  SL_wz = work zone regulatory speed limit                                                                     [mi/h]
  LCSI = as in Equation 10-7
  f_Br, f_DN = as in Equation 10-8
  TRD = total ramp density along the facility                                                                  [ramps/mi]
Implemented in: freeway_facilities/exhibits.rs::WorkZone::ffs

Equation 10-11:  CAF_wz = c_wz / c                                                                             [decimal]  (capped at 1.0)
  CAF_wz = work zone capacity adjustment factor                                                                [decimal]
  c_wz = as in Equation 10-9                                                                                    [pc/h/ln]
  c = non-work-zone basic freeway segment capacity                                                             [pc/h/ln]
Implemented in: freeway_facilities/exhibits.rs::WorkZone::caf, layered onto the segment's calibration CAF by freeway_facilities/freeway_facilities.rs::FreewayFacility::effective_caf

Equation 10-12:  SAF_wz = FFS_wz / FFS                                                                          [decimal]  (capped at 1.0)
  SAF_wz = work zone free-flow speed adjustment factor                                                          [decimal]
  FFS_wz = as in Equation 10-10                                                                                  [mi/h]
  FFS = non-work-zone free-flow speed                                                                            [mi/h]
Implemented in: freeway_facilities/exhibits.rs::WorkZone::saf, layered onto the segment's calibration SAF by freeway_facilities/freeway_facilities.rs::FreewayFacility::effective_saf

### Undersaturated evaluation (Step A-11)

`analyze_undersaturated_period` sets served volume equal to demand for every segment and calls `evaluate_period_chain`, which walks segments in order, evaluates each with the appropriate Chapter 12/13/14 engine via `engine_eval`, and applies the Equation 25-1 maximum-achievable-speed cap (`exhibits::max_achievable_speed`) using the distance between the previous and current segment midpoints. For `OverlappingRamp` segments, the code takes the worse (slower) of the adjacent merge (already computed, upstream) and diverge (computed ad hoc for the next segment) speeds, per the Exhibit 10-11(c) worst-case rule — see `evaluate_period_chain`'s special-case block for `SegmentType::OverlappingRamp`. Segment LOS is computed by `density_los`, which rounds pc density to the nearest integer before the Exhibit 25-59/12-15/14-3 threshold lookups (documented `VERIFY-HCM` rationale: the published Example Problem 1 LOS matrix is only reproducible with integer-rounded densities, e.g. Segment 8 Period 4's D_R = 28.2 pc/mi/ln rounds to LOS C at the <= 28 boundary).

Two deliberate deviations from the literal manual text are called out with `VERIFY-HCM` comments in `engine_eval`: merge/diverge segment speeds from the Chapter 14 ramp engine are additionally capped at the Chapter 12 basic speed-flow value at the same volume (`ramp.get_speed_avg().min(basic(volume))`), justified by reproducing Exhibit 25-49 Segment 10 Period 3 (published 51.8 mi/h = the Equation 12-1 value; see VERIFICATION.md ch10/25 item 5); and `density_los` falls back to the Exhibit 12-15 basic-segment thresholds (rather than a merge/diverge-specific LOS F rule, which Exhibit 14-3 does not define) for queued ramp segments (see VERIFICATION.md ch10/25 item 7). The density-rounding behavior of `density_los` itself is VERIFICATION.md ch10/25 item 6.

Equation 25-1:  V_max = FFS - (FFS - V_prev) * e^(-0.00162 * L)                                                 [mi/h]
  V_max = maximum achievable segment speed                                                                     [mi/h]
  FFS = subject segment free-flow speed (already SAF-adjusted, i.e. FFS_adj of Equation 10-4)                   [mi/h]
  V_prev = average speed on the immediately upstream segment                                                    [mi/h]
  L = distance between the midpoints of the upstream segment and the subject segment                            [ft]
Implemented in: freeway_facilities/exhibits.rs::max_achievable_speed, applied per segment/period (whenever a previous segment's speed is available) by freeway_facilities/freeway_facilities.rs::FreewayFacility::evaluate_period_chain

### Oversaturated evaluation (Step A-12; the Chapter 25 Section 4 engine)

`analyze_oversaturated(first)` constructs an `OversaturatedEngine` (lengths in mi, lanes, f_HV, jam density, queue discharge drop, and the time step in seconds — default 15 s, giving `steps_per_period = 900/15 = 60` and `steps_per_hour = 3600/15 = 240`) and calls `run_period` once per remaining analysis period starting at the first oversaturated period. Per period, `FreewayFacility` precomputes: expected demand ED (Equation 25-6, `OversaturatedEngine::expected_demand`, a static recursive min-with-capacity walk from the facility entrance); background density KB (Equation 25-7, evaluated by re-running `engine_eval` at the expected demand); diverge percentages for the current and previous period (Equations 25-23 through 25-25, `diverge_percentages`); and the front-clearing-queue flag (Equation 25-12, `OversaturatedEngine::front_clearing_active`). These feed an `OversatPeriodInput` into `run_period`, which iterates `steps_per_period` 15-s steps and, at each node from 0 to n, computes: off-ramp flow (Equations 25-22 through 25-25, handling the "deficit of vehicles destined into the upstream segment that were metered" case); mainline input (Equation 25-8); on-ramp flow with ramp queueing (Equations 25-17 through 25-21, including ramp-metering-rate and physical ramp-capacity caps); MO1, the on-ramp-flow storage constraint (Equation 25-9); MO3, the front-clearing wave-lookback constraint (Equations 25-13 through 25-15, via `lookback` — a weighted average over a bounded history ring buffer when the wave travel time is not an integer number of steps, matching the manual's explicit instruction for that case); MO2, the downstream-storage constraint (Equations 25-10/25-11, via `queue_density`); and finally mainline flow as the minimum of all constraints (Equation 25-16). Vehicle counts are then conserved per segment (Equations 25-26 through 25-28) and rolled into next-step state. At period end, segment flow, average vehicles, density, and queue length are aggregated (Equations 25-30 through 25-34).

Two `VERIFY-HCM` deviations are documented directly in `oversaturated.rs`: (1) in the MO2 constraint, the code deliberately uses the *prebreakdown* capacity (`base_sc_step`, before the Equation 25-29 queue-discharge-drop reduction) inside the Equation 25-10 queue-density ratio, while the reduced (post-drop) capacity governs node throughput (MO1/MF, Equation 25-16) — the comment states that applying Equation 25-29's reduced capacity to the KQ ratio as well would lower queue storage density and spread queues much farther upstream than the published Chapter 25 Example Problem 2 results (VERIFICATION.md ch10/25 item 1); and (2) Equation 25-34 (queue length) as printed omits the lane count from the density-difference term, but the code multiplies by `self.lanes[i]` because `uv` (unserved vehicles) is a total count while the density terms are per-lane, so lane count is needed to convert `veh / (veh/mi/ln)` into miles correctly (VERIFICATION.md ch10/25 item 3).

Below is every equation in the Section 4 engine, in the order the engine evaluates them, each with its full where-clause and the implementing Rust location. Node/segment indexing follows Exhibit 25-1/25-4: node `i` is the upstream end of segment `i` (0-based in the code; 1-based, with segment `i-1` upstream of node `i`, in the manual's own convention), and all flow variables (MI, MO1/2/3, MF, ONRF, OFRF, SF) are per-time-step totals across all lanes of the node/segment, in veh/step unless noted.

Equation 25-6 (expected demand, Segment Initialization Steps 1-4):  ED(i,p) = min[ SC(i,p), ED(i-1,p) + ONRD(i,p) - OFRD(i-1,p) ]     [veh/h]
  ED(i,p) = expected demand for segment i, period p — the flow that would arrive if all upstream queues were stacked vertically with no spillback   [veh/h]
  SC(i,p) = segment i prebreakdown capacity, period p                                                          [veh/h]
  ONRD(i,p) = on-ramp demand entering at the upstream node of segment i, period p                               [veh/h]
  OFRD(i-1,p) = off-ramp demand exiting at the downstream node of segment i-1, period p                          [veh/h]  (VERIFICATION.md ch10/25 item 4: printed under segment-based ramp indexing as OFRD(i-1,p); this module's node-based indexing addresses the same physical ramp as `offrd[i]`)
Implemented in: freeway_facilities/oversaturated.rs::OversaturatedEngine::expected_demand

Equation 25-7 (segment initialization, Step 1-4):  NV(i,0,p) = KB(i,p) * N(i,p) * L(i) + UV(i,S,p-1)             [veh]
  NV(i,0,p) = number of vehicles on segment i at the start of the first time step of period p                    [veh]
  KB(i,p) = background density for segment i, period p, from the Chapter 12/13/14 procedures evaluated at the expected demand ED(i,p)   [veh/mi/ln]
  N(i,p) = number of lanes on segment i, period p                                                                 [ln]
  L(i) = length of segment i                                                                                       [mi]
  UV(i,S,p-1) = unserved vehicles on segment i at the end of the last time step S of the preceding period p-1      [veh]  (0 for the first oversaturated period)
Implemented in: freeway_facilities/oversaturated.rs::OversaturatedEngine::init_period

Equation 25-8 (mainline input, Step 9):  MI(i,t,p) = MF(i-1,t,p) + ONRF(i-1,t,p) - OFRF(i,t,p) + UV(i-1,t-1,p)    [veh/step]
  MI(i,t,p) = mainline input at node i, time step t, period p — vehicles wishing to travel through the node this step   [veh/step]
  MF(i-1,t,p) = mainline flow across the upstream node i-1, this time step                                          [veh/step]
  ONRF(i-1,t,p) = on-ramp flow entering at node i-1, this time step                                                  [veh/step]
  OFRF(i,t,p) = off-ramp flow exiting at node i, this time step                                                     [veh/step]
  UV(i-1,t-1,p) = unserved vehicles on segment i-1 at the end of the previous time step                             [veh]
Implemented in: freeway_facilities/oversaturated.rs::OversaturatedEngine::run_period (inline, the `mi` binding inside the per-node loop)

Equation 25-9 (MO1, Step 16):  MO1(i,t,p) = min[ SC(i,t,p) - ONRF(i,t,p), MO2(i,t-1,p), MO3(i,t-1,p) ]           [veh/step]
  MO1(i,t,p) = mainline output constraint at node i from on-ramp flow sharing (competing on-ramp/mainline flow through the node)   [veh/step]
  SC(i,t,p) = segment i capacity this time step (post-Equation-25-29 reduction, if an upstream bottleneck is active)   [veh/step]
  ONRF(i,t,p) = on-ramp flow at node i, this time step                                                              [veh/step]
  MO2(i,t-1,p), MO3(i,t-1,p) = the preceding time step's MO2/MO3 at node i (Equations 25-11, 25-15)                   [veh/step]
Implemented in: freeway_facilities/oversaturated.rs::OversaturatedEngine::run_period (inline, `mo1[node]`; only evaluated when ONRF(i,t,p) > 0 — when there is no on-ramp flow the term would collapse to SC(i,t,p), which is already enforced separately as one of the Equation 25-16 mainline-flow terms, so leaving `mo1[node]` at its BIG/unconstrained default in that case does not change the final `mf[node]`)

Equation 25-10 (queue density, Steps 20-21):  KQ(i,t,p) = [KJ * f_HV(i,p)] - [(KJ - KC) * f_HV(i,p) * SF(i,t-1,p)] / SC(i,t,p)     [veh/mi/ln]
  KQ(i,t,p) = queue (congested-branch) density on segment i                                                         [veh/mi/ln]
  KJ = jam density                                                                                                   [pc/mi/ln]  (default 190, Step A-6/Exhibit 10-7)
  f_HV(i,p) = heavy-vehicle adjustment factor (pc-to-veh conversion)                                                 [decimal]
  KC = density at capacity                                                                                          [pc/mi/ln]  (45, Exhibit 12-6; `DENSITY_AT_CAPACITY_PC`)
  SF(i,t-1,p) = segment i flow in the previous time step                                                            [veh/step]
  SC(i,t,p) = segment i capacity this time step                                                                     [veh/step]  (VERIFICATION.md ch10/25 item 1: the code deliberately uses the *prebreakdown* capacity `base_sc_step` here, not the Equation 25-29-reduced capacity that governs throughput)
Implemented in: freeway_facilities/oversaturated.rs::OversaturatedEngine::queue_density

Equation 25-11 (MO2, Steps 20-21):  MO2(i,t,p) = SF(i,t-1,p) - ONRF(i,t,p) + [KQ(i,t,p) * N(i,p) * L(i)] - NV(i,t-1,p)     [veh/step]
  MO2(i,t,p) = mainline output constraint at node i from downstream segment storage (queue growth on segment i)      [veh/step]
  SF(i,t-1,p), ONRF(i,t,p) = as in Equations 25-10 and 25-9                                                          [veh/step]
  KQ(i,t,p) = queue density from Equation 25-10                                                                       [veh/mi/ln]
  N(i,p), L(i) = number of lanes and length of segment i                                                              [ln], [mi]
  NV(i,t-1,p) = number of vehicles on segment i at the end of the previous time step                                  [veh]
Implemented in: freeway_facilities/oversaturated.rs::OversaturatedEngine::run_period (inline, `mo2[node]`, with `max_veh = kq * self.lanes[node] * self.length_mi[node]`)

Equation 25-12 (front-clearing test, Steps 17-19):  front-clears if  [SC(i,p) - ONRD(i,p)] > [SC(i,p-1) - ONRD(i,p-1)]  AND  [SC(i,p) - ONRD(i,p)] > SD(i,p)
  SC(i,p), SC(i,p-1) = segment i capacity this period and the preceding period                                       [veh/h]
  ONRD(i,p), ONRD(i,p-1) = on-ramp demand at node i this period and the preceding period                              [veh/h]
  SD(i,p) = segment i demand this period                                                                              [veh/h]
Implemented in: freeway_facilities/oversaturated.rs::OversaturatedEngine::front_clearing_queue (the printed inequality) wrapped by ::front_clearing_active, evaluated once per period per segment (not per time step) by freeway_facilities/freeway_facilities.rs::FreewayFacility::analyze_oversaturated before `run_period` is called
VERIFY-HCM: read as a bare inequality this fires whenever on-ramp demand falls, on any segment, queued or not, because it tests SC - ONRD alone. The MO3 text scopes it to queues standing on a segment whose own capacity was temporarily reduced and then restored, so `front_clearing_active` additionally requires vd/c(i,p-1) > 1 and SC(i,p) > SC(i,p-1). The reasoning and the measured effect on Example Problems 2 and 4 are in the doc comment on that function.

Equation 25-13 (shock wave speed, front-clearing):  WS(i,p) = SC(i,p) / [ N(i,p) * (KJ - KC) * f_HV ]                [mi/h]
  WS(i,p) = front-clearing shock wave speed for segment i, period p                                                   [mi/h]
  SC(i,p) = segment i capacity, period p                                                                              [veh/h]
  N(i,p) = number of lanes on segment i                                                                               [ln]
  KJ, KC, f_HV = as in Equation 25-10
Implemented in: freeway_facilities/oversaturated.rs::OversaturatedEngine::run_period (inline, the `ws` binding inside the `wtt` vector construction, evaluated only for segments flagged `front_clearing`)

Equation 25-14 (wave travel time, front-clearing):  WTT = T * L(i) / WS(i,p)                                        [time steps]
  WTT = wave travel time, expressed as a (possibly fractional) number of time steps                                   [time steps]
  T = time steps per hour                                                                                             [steps/h]  (240 at the default 15-s step; `steps_per_hour`)
  L(i) = length of segment i                                                                                          [mi]
  WS(i,p) = as in Equation 25-13                                                                                      [mi/h]
Implemented in: freeway_facilities/oversaturated.rs::OversaturatedEngine::run_period (inline, `t * self.length_mi[i] / ws`)

Equation 25-15 (MO3, Steps 17-19):  MO3(i,t,p) = min[ MO1(i+1,t-WTT,p), MO2(i+1,t-WTT,p)+OFRF(i+1,t-WTT,p), MO3(i+1,t-WTT,p)+OFRF(i+1,t-WTT,p), SC(i,t-WTT,p), SC(i+1,t-WTT,p)+OFRF(i+1,t-WTT,p) ]  -  OFRF(i,t,p)     [veh/step]
  MO3(i,t,p) = mainline output constraint at node i from a front-clearing downstream queue                            [veh/step]
  MO1/MO2/MO3(i+1,t-WTT,p) = the downstream node's outputs, looked back WTT time steps (Equation 25-14); if WTT is not an integer number of steps, a weighted average of the two nearest historical steps is used, per the manual's explicit instruction for that case   [veh/step]
  OFRF(i+1,t-WTT,p) = off-ramp flow at the downstream node, looked back WTT steps                                     [veh/step]
  SC(i,t-WTT,p), SC(i+1,t-WTT,p) = segment i and i+1 capacities, looked back WTT steps                                 [veh/step]
  OFRF(i,t,p) = off-ramp flow at node i, this time step (not looked back)                                              [veh/step]
Implemented in: freeway_facilities/oversaturated.rs::OversaturatedEngine::run_period (inline, `mo3[node]`), via the `::lookback` helper against bounded history ring buffers (`hist_mo1`/`hist_mo2`/`hist_mo3`/`hist_ofrf`/`hist_sc`) that perform the weighted-average non-integer-step lookback
The constraint is held until the recovery wave has actually traveled the segment. The Equation 25-13 text says "the clearing does not affect the segment throughput until the recovery wave has reached the upstream end", and the history buffers span earlier analysis periods, so an unguarded lookback would impose the fully congested period's throughput from step 0 of the recovery. `fc_steps` counts time steps since clearing began on each segment and resets when it ends; MO3 stays unset until that count reaches WTT.

Equation 25-16 (mainline flow, Steps 22-23):  MF(i,t,p) = min[ MI(i,t,p), MO1(i,t,p), MO2(i,t,p), MO3(i,t,p), SC(i,t,p), SC(i-1,t,p) ]     [veh/step]
  MF(i,t,p) = mainline flow across node i, this time step                                                              [veh/step]
  MI, MO1, MO2, MO3 = as in Equations 25-8, 25-9, 25-11, 25-15                                                         [veh/step]
  SC(i,t,p) = downstream segment i capacity this time step                                                            [veh/step]
  SC(i-1,t,p) = upstream segment i-1 capacity this time step                                                          [veh/step]  (unconstrained/BIG at the facility entrance, node 0)
Implemented in: freeway_facilities/oversaturated.rs::OversaturatedEngine::run_period (inline, `mf[node]`)

Equation 25-17 (on-ramp input, Steps 10-11):  ONRI(i,t,p) = ONRD(i,t,p) + ONRQ(i,t-1,p)                             [veh/step]
  ONRI(i,t,p) = on-ramp input at node i, this time step                                                                [veh/step]
  ONRD(i,t,p) = on-ramp demand at node i, this time step                                                               [veh/step]
  ONRQ(i,t-1,p) = vehicles queued on the ramp at the end of the previous time step                                     [veh]
Implemented in: freeway_facilities/oversaturated.rs::OversaturatedEngine::run_period (inline, `onri`)

Equation 25-18 (on-ramp output, Step 12):  ONRO(i,t,p) = min{ RM(i,t,p), ONRC(i,t,p), max[ Lambda - MI(i,t,p),  Lambda / (2*N(i,p)) ] }     where  Lambda = min[ SC(i,t,p), MF(i+1,t-1,p)+ONRF(i,t-1,p), MO3(i,t-1,p)+ONRF(i,t-1,p) ]     [veh/step]
  ONRO(i,t,p) = maximum on-ramp output at node i, this time step                                                       [veh/step]
  RM(i,t,p) = ramp-metering rate at node i, this time step, if specified                                               [veh/step]
  ONRC(i,t,p) = physical ramp roadway capacity at node i (Exhibit 14-12/`get_ramp_capacity`)                          [veh/step]
  Lambda = total mainline+ramp throughput available at the merge point, estimated from the previous time step          [veh/step]
  MI(i,t,p) = mainline input, Equation 25-8                                                                            [veh/step]
  N(i,p) = number of lanes on segment i                                                                                [ln]  (the Lambda/2N term models forced one-to-one Lane-1 merging between ramp and freeway traffic at high demand)
Implemented in: freeway_facilities/oversaturated.rs::OversaturatedEngine::run_period (inline: `lambda` is the Lambda term above; `onro` is the `max(...)` structure; `ramp_capacity`/`ramp_metering` are then `.min()`-ed on to complete the outer `min{RM, ONRC, max{...}}`)

Equation 25-19 (on-ramp flow, unmetered, Step 13):  ONRF(i,t,p) = ONRI(i,t,p)     [when ONRI(i,t,p) <= ONRO(i,t,p)]     [veh/step]
Implemented in: freeway_facilities/oversaturated.rs::OversaturatedEngine::run_period (inline, the `if onri <= onro` branch, which also resets `self.onrq[node] = 0.0`)

Equation 25-20 (on-ramp flow, metered, Step 13):  ONRF(i,t,p) = ONRO(i,t,p)     [otherwise]     [veh/step]
Implemented in: freeway_facilities/oversaturated.rs::OversaturatedEngine::run_period (inline, the `else` branch)

Equation 25-21 (on-ramp queue, Steps 14-15):  ONRQ(i,t,p) = ONRI(i,t,p) - ONRO(i,t,p)                                [veh]
  ONRQ(i,t,p) = vehicles queued on the on-ramp at the end of this time step                                           [veh]
Implemented in: freeway_facilities/oversaturated.rs::OversaturatedEngine::run_period (inline, `self.onrq[node] = onri - onrf[node]`, evaluated only on the metered branch)

Equation 25-22 (off-ramp deficit, Steps 5-8):  DEF(i,t,p) = max{ 0,  [ Sum_{X=1}^{p-1} SD(i-1,X) - Sum_{X=1}^{p-1} Sum_{t=1}^{T} (MF(i-1,t,X)+ONRF(i-1,t,X)) ]  +  Sum_{t=1}^{t-1} (MF(i-1,t,p)+ONRF(i-1,t,p)) }     [veh]
  DEF(i,t,p) = deficit of vehicles destined into upstream segment i-1 that were metered upstream and have not yet arrived   [veh]
  SD(i-1,X) = segment i-1 demand in period X                                                                           [veh, period total]
  MF(i-1,t,X), ONRF(i-1,t,X) = mainline and on-ramp flow into segment i-1 at time step t of period X                    [veh/step]
Implemented in: freeway_facilities/oversaturated.rs::OversaturatedEngine::run_period (inline, `let deficit = (self.cum_demand[node - 1] - self.cum_arrivals[node - 1]).max(0.0);`); the code tracks the two running sums incrementally via the `cum_demand`/`cum_arrivals` fields (the former updated once per period end, the latter every time step) rather than re-summing the full history each step, which is arithmetically equivalent to the printed double summation

Equation 25-23 (off-ramp flow, deficit >= inflow, Step 6):  OFRF(i,t,p) = [MF(i-1,t,p) + ONRF(i-1,t,p)] * [OFRD(i,p-1) / SD(i-1,p-1)]     [veh/step]   (when inflow <= DEF(i,t,p))
  OFRD(i,p-1) / SD(i-1,p-1) = the preceding period's diverge percentage at node i
Implemented in: freeway_facilities/oversaturated.rs::OversaturatedEngine::run_period (inline, `inflow_now * pct_prev`)

Equation 25-24 (off-ramp flow, partial deficit, Step 7):  OFRF(i,t,p) = DEF(i,t,p) * [OFRD(i,p-1)/SD(i-1,p-1)]  +  [MF(i-1,t,p)+ONRF(i-1,t,p) - DEF(i,t,p)] * [OFRD(i,p)/SD(i-1,p)]     [veh/step]   (when 0 < DEF(i,t,p) < inflow)
Implemented in: freeway_facilities/oversaturated.rs::OversaturatedEngine::run_period (inline, `deficit * pct_prev + (inflow_now - deficit) * pct_now`)

Equation 25-25 (off-ramp flow, no deficit, Step 8):  OFRF(i,t,p) = [MF(i-1,t,p)+ONRF(i-1,t,p)] * [OFRD(i,p)/SD(i-1,p)]     [veh/step]   (when DEF(i,t,p) = 0)
  OFRD(i,p)/SD(i-1,p) = this period's diverge percentage at node i
Implemented in: freeway_facilities/oversaturated.rs::OversaturatedEngine::run_period (inline, `inflow_now * pct_now`); the diverge percentages themselves (Equations 25-23 through 25-25's OFRD/SD ratios, for both the current and preceding period) are precomputed once per period by freeway_facilities/freeway_facilities.rs::FreewayFacility::diverge_percentages

Equation 25-26 (segment flow, Steps 24-25):  SF(i-1,t,p) = MF(i,t,p) + OFRF(i,t,p)                                    [veh/step]
  SF(i-1,t,p) = segment i-1 flow (total output) this time step                                                        [veh/step]
Implemented in: freeway_facilities/oversaturated.rs::OversaturatedEngine::run_period (inline, `sf[i] = mf[i + 1] + ofrf[i + 1];` — the code's segment index `i` stands for the manual's segment `i-1`)

Equation 25-27 (vehicle conservation):  NV(i-1,t,p) = NV(i-1,t-1,p) + MF(i-1,t,p) + ONRF(i-1,t,p) - MF(i,t,p) - OFRF(i,t,p)     [veh]
  NV(i-1,t,p) = number of vehicles on segment i-1 at the end of this time step                                          [veh]
Implemented in: freeway_facilities/oversaturated.rs::OversaturatedEngine::run_period (inline, `let inflow = mf[i] + onrf[i]; self.nv[i] += inflow - sf[i];`)

Equation 25-28 (unserved vehicles):  UV(i-1,t,p) = NV(i-1,t,p) - [KB(i-1,p) * L(i-1)]                                 [veh]  (VERIFICATION.md ch10/25 item 2: printed without the lane count N; dimensional consistency with Equation 25-7's NV formula requires it — likely an erratum. The code multiplies KB by N as well.)
  UV(i-1,t,p) = unserved (queued) vehicles stored on segment i-1                                                       [veh]
  KB(i-1,p) = background density (Equation 25-7)                                                                       [veh/mi/ln]
  L(i-1) = length of segment i-1                                                                                        [mi]
Implemented in: freeway_facilities/oversaturated.rs::OversaturatedEngine::run_period (inline, `self.uv[i] = (self.nv[i] - background).max(0.0);` where `background = input.background_density[i] * self.lanes[i] * self.length_mi[i]`)

Equation 25-29 (queue discharge drop):  SC(i,t,p) = (1 - alpha) * SC(i,t,p)     [applied when UV(i-1,t,p) > 0.001]     [veh/step]
  alpha = queue discharge capacity drop                                                                                 [decimal]  (default 0.07, Step A-6/Exhibit 10-7; 0.134 in work zones, Equation 10-9)
Implemented in: freeway_facilities/oversaturated.rs::OversaturatedEngine::run_period (inline, `sc_step[i] *= 1.0 - self.capacity_drop;` when the upstream segment — or the facility entry queue, for node 0 — has UV > 0.001); this reduced capacity governs mainline throughput (Equations 25-16/25-9) but deliberately NOT the Equation 25-10 KQ ratio (VERIFICATION.md ch10/25 item 1)

Equation 25-30 (average segment flow):  SF(i,p) = (T/S) * Sum_{t=1}^{S} SF(i,t,p)                                     [veh/h]
  SF(i,p) = average segment flow rate over the analysis period                                                          [veh/h]
  T = time steps per hour                                                                                               [steps/h]  (240 at the default 15-s step)
  S = time steps per analysis period                                                                                    [steps/period]  (60 at the default 15-s step, so T/S = 4)
Implemented in: freeway_facilities/oversaturated.rs::OversaturatedEngine::run_period (inline, `segment_flow[i] = (t / s) * sum_sf[i];`)

Equation 25-31 (average vehicles):  NV(i,p) = (1/S) * Sum_{t=1}^{S} NV(i,t,p)                                          [veh]
  NV(i,p) = average number of vehicles on segment i over the analysis period                                            [veh]
Implemented in: freeway_facilities/oversaturated.rs::OversaturatedEngine::run_period (inline, `avg_vehicles[i] = sum_nv[i] / s;`)

Equation 25-32 (average density):  K(i,p) = NV(i,p) / [L(i) * N(i,p)]                                                  [veh/mi/ln]
  K(i,p) = average per-lane density of segment i over the analysis period                                               [veh/mi/ln]
  L(i) = segment length                                                                                                  [mi]
  N(i,p) = number of lanes                                                                                                [ln]
Implemented in: freeway_facilities/oversaturated.rs::OversaturatedEngine::run_period (inline, `density[i] = avg_vehicles[i] / (self.length_mi[i] * self.lanes[i]);`)

Equation 25-33 (average speed):  U(i,p) = SF(i,p) / K(i,p)                                                             [mi/h]
  U(i,p) = average space mean speed of segment i over the analysis period                                               [mi/h]
  SF(i,p) = Equation 25-30 (a total, all-lane flow rate)                                                                 [veh/h]
  K(i,p) = Equation 25-32 (a per-lane density)                                                                           [veh/mi/ln]
**DISCREPANCY:** as printed, U = SF/K divides a total (all-lane) flow by a per-lane density, which dimensionally yields a value N times too large (i.e., mi/h scaled by lane count) rather than a true space mean speed. This is the same class of lane-count omission already catalogued as VERIFICATION.md ch10/25 items 2 and 3 (Equations 25-28 and 25-34), but Equation 25-33 itself is not listed there. The code correctly divides the segment flow by the lane count before dividing by density: U = (SF(i,p)/N(i,p)) / K(i,p).
Implemented in: freeway_facilities/freeway_facilities.rs::FreewayFacility::analyze_oversaturated (inline, the `queued_speed` binding: `(res.segment_flow[i] / lanes[i]) / res.density[i]`)

Equation 25-34 (queue length):  Q(i,p) = UV(i,S,p) / max[ (KQ(i,S,p) - KB(i,p)), 1 ] * 5280                            [ft]  (VERIFICATION.md ch10/25 item 3: printed without the lane count N on the density-difference term; the code multiplies the difference by N(i,p) because UV is a total vehicle count while KQ/KB are per-lane densities, so N is needed to convert veh / (veh/mi/ln) into miles correctly)
  Q(i,p) = queue length at the end of the analysis period                                                               [ft]
  UV(i,S,p) = unserved vehicles at the end of the last time step S                                                       [veh]
  KQ(i,S,p) = queue density at the end of the period (Equation 25-10)                                                   [veh/mi/ln]
  KB(i,p) = background density (Equation 25-7)                                                                          [veh/mi/ln]
Implemented in: freeway_facilities/oversaturated.rs::OversaturatedEngine::run_period (inline, `queue_length_ft[i] = (self.uv[i] / dk * 5280.0).min(self.length_mi[i] * 5280.0);` where `dk = (kq_last[i] - input.background_density[i]).max(1.0) * self.lanes[i]`; the code additionally clamps the result to the physical segment length, a reasonable implementation bound not stated in the printed equation)

### Facility aggregation and LOS (Steps A-15/A-17)

`compute_facility_performance` computes, per period: facility space mean speed (Equation 25-2, `exhibits::facility_space_mean_speed`, flow-times-length over flow-times-length-over-speed); facility density in both veh/mi/ln and pc/mi/ln (Equation 10-1, `exhibits::facility_density`, length-and-lane weighted); facility LOS (`exhibits::los_freeway_facility`, Exhibit 10-6, forcing LOS F whenever any segment's vd/c exceeds 1.00 in that period, independent of density); and VMT/VHT/VHD accumulated segment-by-segment using served and demand volumes at 0.25-hour period duration. `overall_space_mean_speed` and `overall_density_veh` implement the facility-wide, all-period aggregates (Equations 25-4 and 25-5) as flow-times-length-weighted averages across every `[segment][period]` cell.

Equation 10-1 (facility density; also printed, identically, as Equation 25-3 "Average facility density in time interval p" — the two equation numbers share one formula):  D_F = Sum_{i=1}^{n} (D_i * L_i * N_i) / Sum_{i=1}^{n} (L_i * N_i)     [pc/mi/ln]
  D_F = average density for the facility in a given 15-min analysis period                                             [pc/mi/ln]
  D_i = density for segment i                                                                                           [pc/mi/ln]
  L_i = length of segment i                                                                                             [mi]
  N_i = number of lanes in segment i                                                                                     [ln]
  n = number of segments in the defined facility
Implemented in: freeway_facilities/exhibits.rs::facility_density, called twice per period by freeway_facilities/freeway_facilities.rs::FreewayFacility::compute_facility_performance — once with `density_veh` (reported as `avg_density_veh`, matching Exhibit 25-52's veh/mi/ln convention) and once with `density_veh / f_HV` as a pc-basis input (reported as `avg_density_pc`, the basis for the Exhibit 10-6 LOS lookup)

Equation 25-2 (facility space mean speed):  SMS(NS,p) = Sum_{i=1}^{NS} [SF(i,p) * L(i)]  /  Sum_{i=1}^{NS} [SF(i,p) * L(i)/U(i,p)]     [mi/h]
  SMS(NS,p) = facility space mean speed over NS segments in period p                                                     [mi/h]
  SF(i,p) = segment i flow                                                                                               [veh/h]
  L(i) = segment i length                                                                                                [any consistent unit; the code uses ft]
  U(i,p) = segment i speed                                                                                              [mi/h]
Implemented in: freeway_facilities/exhibits.rs::facility_space_mean_speed

Equation 25-4 (overall space mean speed, all periods):  SMS(NS,P) = Sum_{p=1}^{P} Sum_{i=1}^{NS} [SF(i,p)*L(i)]  /  Sum_{p=1}^{P} Sum_{i=1}^{NS} [SF(i,p)*L(i)/U(i,p)]     [mi/h]
  SMS(NS,P) = overall space mean speed across all P analysis periods and NS segments                                     [mi/h]
Implemented in: freeway_facilities/freeway_facilities.rs::FreewayFacility::overall_space_mean_speed

Equation 25-5 (overall average density, all periods):  K(NS,P) = Sum_{p=1}^{P} Sum_{i=1}^{NS} [K(i,p)*L(i)]  /  Sum_{p=1}^{P} Sum_{i=1}^{NS} [L(i)*N(i,p)]     [veh/mi/ln]
  K(NS,P) = overall average density across all P analysis periods and NS segments                                       [veh/mi/ln]
Implemented in: freeway_facilities/freeway_facilities.rs::FreewayFacility::overall_density_veh

## Validation

The fixture-driven integration tests live in `tests/chapter10_integration.rs` (Rust) and `tests/test_chapter10_integration.py` (PyO3 bindings, case1 only), reading `tests/ExampleCases/hcm/FreewayFacilities/case1.json` and `case2.json`. These reproduce HCM Chapter 25 Example Problem 1 (undersaturated, Exhibits 25-43 through 25-52; 6-mi, 11-segment urban facility, five 15-min periods) and Example Problem 2 (oversaturated, same geometry with +11% demand, Exhibits 25-53 through 25-60).

Declared tolerances (stated in the `tests/chapter10_integration.rs` module doc comment): speeds +-0.5 mi/h, densities +-0.5 veh/mi/ln, volumes served +-40 veh/h ("the book carries rounded intermediates and reports whole vehicles"), LOS letters exact. Example Problem 1 passes at these tolerances across the full volume-served (Exhibit 25-48), speed (Exhibit 25-49), density (Exhibit 25-50), LOS (Exhibit 25-51), and facility performance (Exhibit 25-52, including the 56.9 mi/h / 28.4 veh/mi/ln overall totals) matrices — all 55 cells (11 segments x 5 periods) per matrix.

Example Problem 2 has documented, asserted-at-computed-value reproduction gaps, all in period 4 and the period-3 Segment 5 cell, stemming from how the engine distributes a clearing queue back into upstream segments versus the published HCM engine:
- `ep2_speed_matrix_reproduced_cells_match_exhibit_25_57`: period 3 Segment 5 asserts 44.0 mi/h at +-1.5 tolerance against a published 45.3 mi/h; period 4 Segments 1-6 are not asserted against the published 47.2/47.5/51.5/48.3/56.5/24.7 mi/h at all (the computed 59.5/53.0/58.3/53.9/48.2/21.5 mi/h diverge because "the published engine spills the residual queue back into Segments 1-4 during period 4 while this implementation holds it in Segments 5-6").
- `ep2_density_matrix_reproduced_cells_match_exhibit_25_58`: period 3 Segments 5-7 (indices 4-6) use +-1.5 tolerance (computed 44.0/65.0/65.3 vs published 42.9/64.8/66.4 veh/mi/ln) versus +-0.5 elsewhere.
- `ep2_los_matrix_reproduced_cells_match_exhibit_25_59`: period 4 Segments 1-5 are asserted at their computed D/D/D/D/E rather than the published E/E/E/E/D.
- `ep2_facility_performance_matches_exhibit_25_60`: per-period speed/density/LOS all match at the standard tolerance including LOS F in period 3, but the overall (all-period) totals are asserted at the computed 49.3 mi/h / 36.5 veh/mi/ln with +-1.5 tolerance against a published 50.5 mi/h / 35.6 veh/mi/ln, attributed to the same period-4 queue-distribution gap.
- `ep2_queue_lifecycle`: a qualitative check (not matrix comparison) that the Segment 8 bottleneck never exceeds va/c = 1.0, that queues form upstream in period 3, and that all queues clear by period 5.

Unit tests in `src/hcm/freeway_facilities/tests.rs` and `src/hcm/freeway_facilities/exhibits.rs` (inline `#[cfg(test)]` modules) spot-check individual equations against hand-computed or Exhibit-sourced values, e.g. `test_equation_25_1_max_achievable_speed` against Chapter 25 Example Problem 1 Segment 3 Period 1 (V_max ~= 59.71 mi/h at +-0.05), `test_exhibit_10_15_lcsi_values` against all six Exhibit 10-15 LCSI entries, and `test_equations_10_9_through_10_12` against a hand-worked 2-to-2 work zone case. `oversaturated.rs`'s own test module exercises the engine in isolation (bottleneck metering, queue-discharge-drop persistence across periods, queue recovery, ramp forced-merge sharing, ramp metering, off-ramp diverge percentage, front-clearing detection and its scoping to a restored bottleneck, and the recovery-wave hold on MO3) with synthetic single- and multi-segment facilities rather than published HCM numbers.

## Deferred

Per the `freeway_facilities/mod.rs` and `freeway_facilities.rs` module doc comments, explicitly out of scope in this pass:
- Managed-lane facilities (Steps A-9/A-13/A-14) — implemented separately on `feat/hcm-ch10-managed-lanes` (`src/hcm/freeway_facilities/managed_lanes.rs`, `planning.rs`).
- The Chapter 25 planning-level method.
- The Chapter 25 Section 5 special work zone configuration tables (Exhibits 25-8 through 25-14) — only the general NCHRP 03-107 work zone CAF/SAF model (Equations 10-7 through 10-12) is implemented, not the specific configuration-table lookups.
- Exhibit 12-25 provides no PCE for mountainous terrain, and Chapter 10's own required-input exhibit offers only level, rolling, and specific grade; `Terrain::pce()` reuses the rolling-terrain 3.0 as a stand-in rather than the Chapter 25/26 mixed-flow model the manual directs to. Not a conservative choice: only `basicfreeways`' 2.5 sits lower among the library's four stand-ins, and because `to_weave`/`to_ramp` pass Mountainous through unchanged, one mountainous facility charges 3.0 on its basic segments and 5.0 on its weaving and ramp segments. Pending the keep-vs-error decision in `VERIFICATION.md` (Chapter 12-14 item 1).

No hooks (stub types, `todo!()`, or feature flags) for these deferrals are present in this module; they are simply unimplemented.
