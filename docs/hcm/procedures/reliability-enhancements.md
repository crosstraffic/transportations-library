# Reliability Enhancements: Chapter 17 Residual-Queue Carryover and Chapter 37 ATDM Strategy Models

This document walks a reviewer through the two enhancements delivered on this branch, both closing documented deferrals in the reliability engines. The first is the HCM 7th Edition Chapter 17, Section 3 ("Facility Evaluation") residual-queue carryover between chronological analysis periods of the urban street reliability method: the initial-queue delay term d3 (Chapter 19 Equations 19-44 through 19-49) is now computed per boundary intersection with the queue Qb inherited from the previous analysis period, and the residual queue Qe (Equation 19-45) is carried forward, with a day-boundary reset rule. The second is a new HCM Chapter 37 ("ATDM: Supplemental") module transcribing the strategy-impact models that feed the Chapter 11 freeway and Chapter 17 urban reliability engines: Section 3 shoulder/median lane capacity models (Equation 37-1), Section 4 ramp metering (the 1.03 merge CAF and the Equation 37-2 ALINEA-derived metering rate), and Section 5 adaptive signal control (Exhibit 37-9's illustrative ranges). The code lives in `src/hcm/common/delay.rs` (`queue_end_of_period`, new), `src/hcm/urban_reliability/urban_reliability.rs` (carryover threading through the scenario loop), `src/hcm/common/atdm.rs` (new module), and convenience constructors in `src/hcm/freeway_reliability/scenario_generation.rs` and `src/hcm/urban_reliability/urban_reliability.rs`. Interpretations and book gaps are catalogued in `docs/hcm/VERIFICATION.md` under "Reliability enhancements (Ch 17 carryover, Ch 37 ATDM) (feat/hcm-reliability-enhancements)" (items 1-6); this walkthrough cites those items rather than restating them. Every equation named below is reproduced in full below its subsection, with a where-clause giving units and defaults, so this document is verifiable standalone against a Rust translation without opening the HCM manual; equation text is cross-checked against both the Rust source and the HCM 7th Edition EPUB MathML (Chapter 19: `135_Ch19_01.xhtml`-`142_Ch19_08.xhtml`; Chapter 37: `284_Ch37_01.xhtml`-`292_Ch37_09.xhtml`).

## Part 1: Chapter 17 residual-queue carryover

### Step-by-step

| HCM item | Equations | Rust location | Inputs / outputs (units) |
|---|---|---|---|
| Initial queue delay d3 | Eq 19-44 through 19-49 | `common/delay.rs::initial_queue_delay` (pre-existing on this branch's base) | Qb (veh), v (veh/h), c_A (veh/h), T (h) -> d3 (s/veh) |
| Residual queue at period end | Eq 19-45 (with t_A/Q_eo from Eq 19-46 through 19-49) | `common/delay.rs::queue_end_of_period` (new) | Qb (veh), v (veh/h), c_A (veh/h), T (h) -> Qe (veh, >= 0) |
| Chronological queue hand-off | Ch 17 Sec. 3, "Facility Evaluation" | `urban_reliability/urban_reliability.rs::UrbanReliability::run` (the `queue_state`/`last_day` loop) and `::evaluate_scenario` (new `queue_in` parameter, `queue_out` return) | per-boundary-intersection queue vector (veh), one entry per segment |
| Day-boundary reset | interpretation — VERIFICATION.md item 1 | `run` — `if last_day != Some(scenario.day_of_year) { queue_state = vec![0.0; n_seg]; }` | resets Qb to 0 at each day's first analysis period |

#### Equations 19-44 through 19-49 in full

These are the six equations that together define d3 (initial queue delay) and Qe (residual queue). Transcribed directly from the Chapter 19 MathML (`138_Ch19_04.xhtml`) and matching `common/delay.rs::initial_queue_delay` and `common/delay.rs::queue_end_of_period` term for term.

```
Equation 19-44:  d3 = (3,600 / (v T)) * [ t_A (Qb + Qe - Qeo)/2  +  (Qe^2 - Qeo^2)/(2 c_A)  -  Qb^2/(2 c_A) ]     [s/veh]
  v     = demand flow rate for the analysis period                                          [veh/h]
  T     = analysis period duration (ANALYSIS_PERIOD_H = 0.25 in this codebase)               [h]
  t_A   = adjusted duration of unmet demand within the analysis period (Eq 19-47 or 19-49)    [h]
  Qb    = initial queue at the start of the analysis period (0 if none)                      [veh]
  Qe    = queue at the end of the analysis period, i.e. the residual queue (Eq 19-45)         [veh]
  Qeo   = queue at the end of the analysis period if v >= c_A and Qb = 0 (Eq 19-46 or 19-48)  [veh]
  c_A   = average lane-group capacity for the analysis period (Eq 19-42/19-43 in the book;
          this implementation passes the ordinary lane-group capacity directly -- see the
          "Documented simplification" note below and VERIFICATION.md item 2)                 [veh/h]
By definition (HCM Chapter 19, Step 8, Part B): d3 = 0.0 s/veh whenever Qb = 0 (no initial queue).

Equation 19-45:  Qe = Qb + t_A (v - c_A)                                                      [veh, clamped >= 0]
  (same variables as above; this is the "residual queue" carried forward as the next period's Qb)

If v >= c_A (the analysis period is oversaturated or exactly at capacity):
  Equation 19-46:  Qeo = T (v - c_A)                                                          [veh]
  Equation 19-47:  t_A = T                                                                    [h]

If v < c_A (the analysis period is undersaturated):
  Equation 19-48:  Qeo = 0.0 veh                                                               [veh]
  Equation 19-49:  t_A = min( Qb / (c_A - v), T )                                             [h]
    -- Qb / (c_A - v) is the time for the initial queue to dissipate at the excess-capacity
       rate (c_A - v); if that time exceeds T, the queue does not fully clear within the
       period and t_A is capped at T.
```

Implemented in: `common/delay.rs::initial_queue_delay(queue_initial_veh, v, capacity, t_h) -> d3` and `common/delay.rs::queue_end_of_period(queue_initial_veh, v, capacity, t_h) -> Qe`. Both functions independently re-derive `t_A`/`Qeo` from the same `v`/`capacity` comparison rather than sharing a helper, so the branch logic in each must be read side by side to confirm they agree (they do — verified by `test_queue_end_of_period_oversaturated_matches_eq_19_45` and the analogous `initial_queue_delay` tests in `common/delay.rs`). `queue_end_of_period` carries a defensive third branch (`else { t_h }`) that is unreachable in `f64` arithmetic once the `v >= capacity` and `capacity > v` arms are exhausted (it would only fire on a NaN comparison); documented as harmless dead code in the walkthrough prose above.

`queue_end_of_period` implements Equation 19-45, `Qe = Qb + t_A (v - c_A)`, selecting `t_A = T` when v >= c_A (Equation 19-47) and `t_A = min(Qb / (c_A - v), T)` when v < c_A (Equation 19-49), clamped to Qe >= 0. Its doc comment quotes the governing Chapter 17 sentence ("the initial queue input value for the next analysis period is set equal to the residual queue output for the current analysis period") and notes the same hand-off appears in Chapter 29, Section 3 for the multiple-time-period/spillback technique. One code-reading note: the function's `t_A` selection has an unreachable third branch (`if v >= capacity ... else if capacity > v ... else t_h` — the `else` can only fire on NaN inputs); harmless but dead code.

In `evaluate_scenario`, the queue state is threaded per boundary-intersection through movement (one lane group per segment's downstream signal, in `self.facility.segments` order). For each segment, the carried-in `qb` feeds `initial_queue_delay(qb, through_demand_veh_h, c_veh_h, ANALYSIS_PERIOD_H)` (with `ANALYSIS_PERIOD_H = 0.25` h) and the through control delay becomes `d1 + d2 + d3` (previously `d1 + d2` with d3 = 0 as a documented deferral); `queue_out[i] = queue_end_of_period(...)` with the same arguments becomes the next chronological period's Qb. A scenario is flagged oversaturated when `x > 1.0 || qb > 0.0` — a carried-in queue marks the period oversaturated even at x <= 1, which is an implementation choice a reviewer may want to note (a period draining a residual queue at x < 1 is arguably "recovering" rather than oversaturated, but it does carry initial-queue delay). The carryover implicitly assumes `self.scenarios` is ordered chronologically (grouped by `day_of_year` with periods in order within each day), which is how the Chapter 17 scenario generator produces them.

**The day-reset rule** (VERIFICATION.md item 1) is an interpretation, not a literal reading: the HCM text states the hand-off without an explicit exception at the boundary between one day's study period and the next day's, but a literal reading would carry a queue across the roughly 21-hour gap between, say, 9:45-10:00 a.m. Monday and 7:00-7:15 a.m. Tuesday. The module doc (the "Residual-queue carryover between analysis periods" section of `urban_reliability.rs`) defends the reset on three grounds: physical implausibility of a queue surviving the off-study-period gap, consistency with the Chapter 11 freeway reliability engine (each scenario/day evaluated from a fresh facility clone with no cross-scenario state), and the Chapter 29 Section 3 technique's queue hand-off being explicitly scoped to "subperiods" of one multi-period analysis. Implemented as: `queue_state` zeroed whenever the scenario's `day_of_year` differs from the previous scenario's.

#### Day-boundary reset rule (interpretation, not an HCM equation)

Stated as pseudocode since this is a modeling decision layered on top of the literal Chapter 17 hand-off sentence, not a numbered HCM equation:

```
For each scenario, taken in the chronological order produced by the Chapter 17 scenario generator
(grouped by day_of_year, periods in order within each day):

  if scenario.day_of_year != previous_scenario.day_of_year (or this is the first scenario):
      queue_state[seg] = 0.0 for every boundary intersection seg      # Qb reset to 0
  else:
      queue_state[seg] = queue_out[seg] from the immediately preceding scenario   # Eq 19-45 hand-off

  (result, queue_out) = evaluate_scenario(scenario, queue_state)
  queue_state = queue_out
```

`queue_state: Vec<f64>` has one entry per boundary intersection (`self.facility.segments` order); `queue_out[i]` for segment `i` is exactly `queue_end_of_period(queue_state[i], through_demand_veh_h, c_veh_h, ANALYSIS_PERIOD_H)` (Eq 19-45). Implemented in: `urban_reliability/urban_reliability.rs::UrbanReliability::run` (the `queue_state`/`last_day` loop shown above) calling `::evaluate_scenario` per scenario. This rule assumes `self.scenarios` is already chronologically ordered — true for the Chapter 17 scenario generator's output, but not re-verified defensively inside `run`.

**Documented simplification — the Equations 19-38 through 19-43 capacity blend is not implemented** (VERIFICATION.md item 2). The full Chapter 19, Section 4 initial-queue extension computes a blended average capacity `cA` from a separate saturated capacity `cs` (serving the backlog) and the ordinary capacity `c`, weighted by the unmet-demand duration, and re-derives d1 with a saturated/baseline uniform-delay blend. This implementation passes the scenario's ordinary lane-group capacity directly as `cA` into both `initial_queue_delay` and `queue_end_of_period` — exact when there is no initial queue, an approximation otherwise. The module doc argues the approximation is reasonable because d2 and d3 are additive and `cA` differs from `c` only during the typically short unmet-demand interval within a 15-min period.

## Part 2: Chapter 37 ATDM strategy models (`common/atdm.rs`)

The module doc explains the placement decision: Chapter 37 is supplemental content with no facility methodology of its own — its purpose (per Chapter 11, Section 4) is to translate strategies into CAF/SAF/DAF-shaped inputs for the reliability engines — so it lives under `common/` as pure equation/constant transcriptions, with the convenience constructors that build engine-facing objects placed in `freeway_reliability`/`urban_reliability` to preserve the `common/` dependency direction (chapters depend on common, never the reverse).

### Section 3: shoulder and median lane strategies

| Item | Source | Rust location | Units |
|---|---|---|---|
| Auxiliary-lane capacity default (half a through lane) | Ch 37 Sec. 3 text | `atdm.rs::AUX_SHOULDER_CAPACITY_RATIO` (0.5) | ratio |
| Shoulder-lane capacity by use variant | Ch 37 Sec. 3 text | `atdm.rs::ShoulderLaneUse` (enum: `AllTraffic`, `BusesOnly`, `HovOnly`), `atdm.rs::shoulder_lane_capacity_veh_h_ln` | veh/h/ln |
| Average per-lane capacity with an open shoulder | Eq 37-1 | `atdm.rs::shoulder_lane_average_capacity_veh_h_ln` | veh/h/ln; `(CapShldr + CapMF x MFlanes) / (1 + MFlanes)` |
| Total-capacity CAF equivalent | derived from Eq 37-1 | `atdm.rs::shoulder_lane_caf` | decimal, >= 1; `1 + CapShldr / (CapMF x MFlanes)` |
| Chapter 11 engine hook | — | `freeway_reliability/scenario_generation.rs::WorkZoneEvent::shoulder_lane_strategy` | builds a scheduled `WorkZoneEvent` with `active_day_ratio = 1.0` and the CAF above |

`BusesOnly`/`HovOnly` capacity is the lesser of the observed bus/HOV volume and a capacity value; the HCM never states the numeric default for that capacity when the user does not override it, so the implementation defaults it to a normal mixed-flow lane's capacity, making the observed vehicle count normally binding — VERIFICATION.md item 3 and a `VERIFY-HCM` comment on the enum variant. `shoulder_lane_caf` deliberately keeps the segment's lane count fixed for density purposes and expresses the whole effect as a total-capacity multiplier, the same simplification the Chapter 11 module already documents for incident lane closures (its doc comment says so explicitly).

#### Equation 37-1 and the derived total-capacity CAF, in full

Transcribed directly from the Chapter 37 MathML (`286_Ch37_03.xhtml`) and matching `atdm.rs::shoulder_lane_average_capacity_veh_h_ln` and `atdm.rs::shoulder_lane_caf`.

```
Equation 37-1:  AveCap(s) = [ CapShldr(s) + CapMFlanes(s) x MFlanes(s) ] / [ 1 + MFlanes(s) ]     [veh/h/ln]
  AveCap(s)     = average capacity per lane for section s, across all MFlanes(s)+1 lanes         [veh/h/ln]
  CapShldr(s)   = capacity per shoulder lane for section s (see shoulder_lane_capacity_veh_h_ln
                  below; AUX_SHOULDER_CAPACITY_RATIO x CapMFlanes(s) = 0.5 x CapMFlanes(s) by
                  default for the auxiliary-lane / all-traffic-no-override variant)             [veh/h/ln]
  CapMFlanes(s) = capacity per mixed-flow (normal through) lane in section s                     [veh/h/ln]
  MFlanes(s)    = number of mixed-flow lanes in section s (integer)                              [lanes]
Per the HCM text: "The number of lanes on the freeway segments between adjacent on- and off-ramps
is increased by one for the shoulder lane" -- i.e. the section now has MFlanes(s)+1 lanes total.

Derived total-capacity CAF (not an HCM equation number; a re-expression of Eq 37-1 as a single
multiplicative factor on the section's pre-existing total capacity, for engines such as the
Chapter 11 freeway reliability engine's per-segment CAF schedule that model a capacity change as
one CAF while holding the segment's lane count fixed for density purposes):

  CAF = [ AveCap(s) x (MFlanes(s) + 1) ] / [ CapMFlanes(s) x MFlanes(s) ]
      = 1 + CapShldr(s) / ( CapMFlanes(s) x MFlanes(s) )                                        [dimensionless, >= 1]
  (the two forms are algebraically identical; substituting Eq 37-1's AveCap(s) into the first
  form and canceling the common (MFlanes(s)+1) factor gives the second, closed form actually
  implemented). Returns 1.0 (no effect) if MFlanes(s) = 0 or CapMFlanes(s) <= 0.
```

Implemented in: `common/atdm.rs::shoulder_lane_average_capacity_veh_h_ln(shoulder_capacity_veh_h_ln, mixed_flow_capacity_veh_h_ln, mixed_flow_lanes) -> AveCap(s)` and `common/atdm.rs::shoulder_lane_caf(shoulder_capacity_veh_h_ln, mixed_flow_capacity_veh_h_ln, mixed_flow_lanes) -> CAF`; hooked into the freeway engine via `freeway_reliability/scenario_generation.rs::WorkZoneEvent::shoulder_lane_strategy`.

### Section 4: ramp metering

`RAMP_METERED_MERGE_CAF = 1.03` transcribes the Section 4 recommendation for freeway merge segments while metering operates; `WorkZoneEvent::ramp_metering_merge_strategy` (in `freeway_reliability/scenario_generation.rs`) applies it to the listed merge segments on the metering schedule. `alinea_metering_rate` implements Equation 37-2 — `R(t) = (CM - VM(t)) / NR`, clamped to `[MinRate, MaxRate]` (defaults `ALINEA_DEFAULT_MIN_RATE_VEH_H_LN = 240`, `ALINEA_DEFAULT_MAX_RATE_VEH_H_LN = 900` veh/h/ln) and floored by the queue-storage constraint `R(t) > (VR(t) + QR(t-1) - QRS) / NR` — inputs are downstream capacity CM (veh/h), upstream volume VM (veh/h), ramp volume VR (veh/h), prior ramp queue QR (veh), ramp storage QRS (veh), and metered lane count NR; output is veh/h/ln, with `NR = 0` returning MaxRate. The constructor's doc is explicit that the ALINEA rate is not folded into the `WorkZoneEvent` (it caps a specific ramp's volume rather than producing a CAF/SAF/DAF), so an analyst applies it directly to a facility's on-ramp demand or via the Chapter 10 `ramp_metering` schedule.

#### Equation 37-2 in full, with clamps

Transcribed directly from the Chapter 37 MathML (`287_Ch37_04.xhtml`) and matching `atdm.rs::alinea_metering_rate`.

```
Equation 37-2:  R(t) = ( CM - VM(t) ) / NR                                                       [veh/h/ln]

subject to:     MinRate < R(t) < MaxRate
                R(t) > [ VR(t) + QR(t-1) - QRS ] / NR

  R(t)      = ramp-metering rate for analysis period t                                          [veh/h/ln]
  CM        = capacity of the downstream section                                                [veh/h]
  VM(t)     = volume on the upstream section for analysis period t                               [veh/h]
  NR        = number of metered lanes on the ramp (integer)                                      [lanes]
  VR(t)     = volume on the ramp during analysis period t                                        [veh/h]
  QR(t-1)   = queue on the ramp at the end of the previous analysis period                        [veh]
  QRS       = queue storage capacity of the ramp                                                  [veh]
  MinRate   = user-defined minimum ramp-metering rate; default ALINEA_DEFAULT_MIN_RATE_VEH_H_LN
              = 240                                                                              [veh/h/ln]
  MaxRate   = user-defined maximum ramp-metering rate; default ALINEA_DEFAULT_MAX_RATE_VEH_H_LN
              = 900                                                                              [veh/h/ln]

Implementation detail (a defensive extension beyond the book's literal "<"/">" text): the two
subject-to conditions are implemented as an inclusive clamp -- take the unconstrained rate,
floor it at the queue-storage constraint, then clamp the result to [MinRate, MaxRate] -- rather
than as strict open-interval inequalities, since a hard floor/ceiling is the only way to return a
single well-defined R(t) in code. NR = 0 (no metered lanes) returns MaxRate (metering has no
effect without a metered lane).
```

Implemented in: `common/atdm.rs::alinea_metering_rate(downstream_capacity_veh_h, upstream_volume_veh_h, ramp_volume_veh_h, ramp_queue_prev_veh, ramp_queue_storage_veh, metered_lanes, min_rate_veh_h_ln, max_rate_veh_h_ln) -> R(t)`. Not wired into any engine automatically (see the Deferred section).

### Section 5: adaptive signal control

The HCM publishes no closed-form adaptive-signal method — Section 5 states "it has not been possible to develop a generalized method" and reports only a three-corridor simulation study (Exhibit 37-9: delay reductions 3%-24%, TTI reductions 3%-13%), transcribed as `ADAPTIVE_SIGNAL_DELAY_REDUCTION_PCT_RANGE` and `ADAPTIVE_SIGNAL_TTI_REDUCTION_PCT_RANGE`. `adaptive_signal_sat_flow_adjustment(target_pct)` converts a target delay-reduction percentage (default: the range midpoint, 13.5%; always clamped to the published 3-24 range) into a Chapter 17 saturation-flow adjustment via `1 / (1 - pct/100)` — a documented modeling simplification, not an HCM-derived equation, flagged `VERIFY-HCM` in the code and as VERIFICATION.md item 4; the rationale is that demand held at capacity then yields the same fractional reduction in the Chapter 19 incremental-delay term's implied excess demand, and the doc directs analysts to prefer directly calibrated values. `AtdmStrategy::adaptive_signal_control` (in `urban_reliability/urban_reliability.rs`) wraps this into a scheduled Chapter 17 strategy.

#### Exhibit 37-9 (illustrative ranges, transcribed constants) and the adaptive-signal sat-flow adjustment formula

Exhibit 37-9 reports only the two illustrative percentage ranges below from a three-corridor simulation study; the HCM prints no formula, so only these two constants are transcribed into code (the rest of the exhibit's structure — per-corridor, per-direction breakdowns — is not carried into code, since no downstream computation consumes it):

```
Exhibit 37-9 (illustrative simulation-study ranges, not a design equation):
  ADAPTIVE_SIGNAL_DELAY_REDUCTION_PCT_RANGE = (3.0, 24.0)   # delay reduction, percent
  ADAPTIVE_SIGNAL_TTI_REDUCTION_PCT_RANGE   = (3.0, 13.0)   # travel time index (TTI) reduction, percent
```

The sat-flow adjustment formula below is a documented modeling simplification, not an HCM-derived equation (flagged `VERIFY-HCM` in `atdm.rs` and as VERIFICATION.md item 4):

```
adaptive_signal_sat_flow_adjustment:

  pct = clamp( target_delay_reduction_pct (default: 13.5, the range midpoint), 3.0, 24.0 )      [%]
  sat_flow_adjustment = 1 / (1 - pct/100)                                                        [dimensionless, >= 1]

  target_delay_reduction_pct = analyst's desired delay reduction from adaptive signal control;
                                None defaults to the midpoint of ADAPTIVE_SIGNAL_DELAY_REDUCTION_
                                PCT_RANGE (13.5%); always clamped to the published 3-24 range     [%]
  sat_flow_adjustment        = multiplicative adjustment to the boundary-intersection saturation
                                flow rate, consumed by
                                urban_reliability::AtdmStrategy::sat_flow_adjustment              [dimensionless]

Rationale (not derived from an HCM formula): treating the target delay reduction as achieved
through better green-time utilization at capacity, demand held at capacity yields the same
fractional reduction in the Chapter 19 incremental-delay term's implied excess demand under this
transform. Analysts should prefer a directly calibrated sat_flow_adjustment /
effective_green_adjustment_s from their own simulation study over this default when available.
```

Implemented in: `common/atdm.rs::adaptive_signal_sat_flow_adjustment(target_delay_reduction_pct: Option<f64>) -> f64`; wrapped into a scheduled strategy by `urban_reliability/urban_reliability.rs::AtdmStrategy::adaptive_signal_control`.

PyO3 bindings added: `shoulder_lane_caf`, `shoulder_lane_default_capacity_veh_h_ln`, `ramp_metered_merge_caf`, `alinea_metering_rate` (`src/copython/freeway_reliability.rs`) and `adaptive_signal_sat_flow_adjustment` (`src/copython/urban_reliability.rs`).

## Validation

No published HCM example problem exercises either enhancement end-to-end with asserted numbers, so validation is a mix of exact equation-level unit tests, a synthetic mechanism test, and direction-of-effect integration tests, with the fixture-level metric movement documented as computed-vs-computed (before/after carryover) against the still-unreached published band:

- **`common/delay.rs` unit tests** (in-module): `test_queue_end_of_period_no_initial_queue_undersaturated` (Qb = 0, v < cA gives Qe = 0), `test_queue_end_of_period_clears_within_period` (small Qb dissipates, Qe = 0), `test_queue_end_of_period_large_initial_queue_undersaturated` (t_A = T binds, Qe = Qb + T(v - cA) exactly), and an oversaturated Equation 19-45 check — all hand-computed values, exact or 1e-9 tolerance.
- **`src/hcm/urban_reliability/tests.rs::test_residual_queue_carryover_and_day_reset`** — the synthetic mechanism test. A single-lane, deliberately over-capacity segment (demand 3,000 veh/h against a one-lane capacity of about 810 veh/h) with all-zero weather and Tuesdays-only RRP isolates the carryover: the second of two same-clock-hour analysis periods must show a strictly higher TTI than the first (it inherits a nonzero Qb), and the next day's first period must reproduce the previous day's period-1 TTI exactly (day reset, not persistence). Per the commit message, queues in this fixture build past 400 veh and drive TTI to about 8.3.
- **`tests/chapter17_integration.rs::test_case1_example_problem_4`** (fixture `tests/ExampleCases/hcm/UrbanReliability/case1.json`, the Chapter 29 Example Problem 4 urban reliability case): the reliability metrics are asserted in wide bands with the exact computed values recorded in the assertion messages and the test's doc comment. The carryover moved every metric in the published direction without closing the gap: mean TTI 1.5249 -> 1.5449 (published 1.69/1.64), TTI-80 1.5883 -> 1.5927 (published 1.57/1.56, already within band), PTI 1.7311 -> 1.7462 (published 2.98/2.61 — the main remaining gap), reliability rating 99.54% -> 98.83% (published 93.2/94.1), annual through VHD 30,902 -> 32,083 veh-h, oversaturated scenarios 37 -> 70 of 3,120. The residual PTI gap is now attributed to other still-deferred elements (random 15-min demand variation, incident-duration defaults), not to the carryover mechanism — the corresponding update is written into VERIFICATION.md's Chapter 16/17 section, item 6.
- **`tests/chapter17_integration.rs::test_case1_atdm_adaptive_signal_control`**: direction-of-effect only (Chapter 37 publishes no assertable effect size) — the strategy must not raise mean travel time, must not degrade PTI, and must not degrade the reliability rating relative to the base run.
- **`tests/chapter11_integration.rs::ep7_atdm_shoulder_lane_strategy_improves_or_holds_reliability`** and **`::ep7_atdm_ramp_metering_strategy_improves_or_holds_reliability`** (fixture `tests/ExampleCases/hcm/FreewayReliability/case1.json`, the Chapter 25 EP7 freeway reliability case): direction-of-effect assertions (mean TTI, reliability rating, expected VHD must not degrade). Both deliberately apply the strategy facility-wide (all segments for the shoulder lane, all merge segments for metering) rather than to a single segment, because a partial capacity boost can shift the binding bottleneck downstream and legitimately worsen aggregate TTI/VHD — VERIFICATION.md item 6 documents this multi-segment interaction as engine behavior, not a bug.
- **`common/atdm.rs` unit tests** (18 in-module tests): the 0.5 auxiliary-capacity ratio, capacity-override and whichever-is-less logic for all three `ShoulderLaneUse` variants, Equation 37-1 against a hand computation (CapShldr 1,200 / CapMF 2,400 / 3 lanes gives 2,100 veh/h/ln) and its degenerate identity (shoulder capacity equal to a normal lane reproduces CapMF exactly), the CAF-vs-total-capacity consistency identity, the 1.03 merge CAF constant, ALINEA unconstrained/min-clamped/max-clamped/queue-floor/zero-lane cases against hand values, the Exhibit 37-9 ranges, and the adaptive-signal default-midpoint/clamping/monotonicity behavior.

The commit message records the overall count: 487 pre-existing tests remain green, 28 new tests added (515 total), with both `cargo build` and `cargo build --features with-python` clean.

## Deferred

- **Equations 19-38 through 19-43** (saturated/baseline capacity blend and the d1 uniform-delay blend for periods with an initial queue) — VERIFICATION.md item 2; the hook is the `capacity` argument of `initial_queue_delay`/`queue_end_of_period` in `evaluate_scenario`, where a blended `cA` would replace the ordinary lane-group capacity.
- **Chapter 37, Sections 6-7** (Dynamic Lane Grouping, Reversible Center Lanes) — VERIFICATION.md item 5 and the `atdm.rs` module doc's "Deferred" section: both sections list Chapter 18/19 inputs an analyst may need to reconsider but publish no exhibit, equation, or default adjustment factor, so there is nothing to transcribe without fabricating a number; not modeled, and flagged in VERIFICATION.md rather than with an in-code marker since there is no code to attach one to.
- **Random 15-min demand variation** (Equations 29-30 through 29-33) in the Chapter 17 scenario generator remains deferred (module doc "Documented deferrals"), and is now the leading suspect for the remaining EP4 PTI gap.
- The ALINEA metering rate (Equation 37-2) is computed but not automatically wired into any engine; the analyst applies it to on-ramp demand or the Chapter 10 `ramp_metering` schedule manually (per the `ramp_metering_merge_strategy` doc comment).
