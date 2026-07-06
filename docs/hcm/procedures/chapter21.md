# HCM Chapter 21 — All-Way STOP-Controlled Intersections

This document walks the HCM 7th Edition Chapter 21 motorized-vehicle methodology for all-way STOP-controlled (AWSC) intersections as implemented on branch `feat/hcm-ch20-22-unsignalized`. The code follows Chapter 21, Section 3 (the core two-lane-approach methodology) and Section 4 (the three-lane-approach extension), transcribed as the sixteen-step procedure in the module header of `src/hcm/awsc/awsc.rs`, which is also the sole source file (`src/hcm/awsc/tests.rs` holds the per-step unit tests against HCM Chapter 32 AWSC Example Problems 1 and 2). As with Chapter 20, the shared delay/LOS primitives (`control_delay_awsc`, `aggregate_control_delay` in `src/hcm/common/delay.rs`; `los_unsignalized` in `src/hcm/common/los_tables.rs`) are documented separately in `common-infrastructure.md`. `docs/hcm/VERIFICATION.md` does not exist in this branch's working tree; deviations are called out inline below with reference to the relevant test or code comment. Every equation transcribed below was cross-checked against both the Rust function body and the HCM 7th Edition Chapter 21 EPUB text (Sections 3 and 4); no mathematical discrepancies were found between the code, this document, and the published equations — every case is either an exact match or a documented, already-flagged simplification (see Deviations).

The public entry point is `Awsc::analyze(&mut self)`, which runs Steps 1-2, 3, 4, 5-11 (the departure-headway iteration), 13-14 and 16 (lane delay/LOS/queue), and 15 (approach/intersection aggregation) — deliberately *excluding* Step 12 (capacity), which the module doc comment flags as "expensive" since it requires a fresh bisection search (each trial itself re-running the full departure-headway iteration) per lane; callers who need lane capacity call `Awsc::step12_capacities()` separately.

## Step-by-step walkthrough

| HCM step | Equations / Exhibits | Rust function(s) | Inputs (units) | Outputs (units) |
|---|---|---|---|---|
| Steps 1-2 — Demand flow rates | Equation 21-12 | `Awsc::step1_2_flow_rates` | `AwscLane.volume_left`/`volume_through`/`volume_right` (veh/h), `phf` (`None` => 1.0) | `AwscLane.flow_rate` (veh/h) |
| Step 3 — Geometry group | Exhibit 21-11 | `geometry_group` (free function), `Awsc::step3_geometry_groups`, `Awsc::is_four_leg` | Lane counts on the subject, opposing, and (max of) the two conflicting approaches | `AwscApproach.geometry_group: GeometryGroup` (one of G1, G2, G3a, G3b, G4a, G4b, G5, G6) |
| Step 4 — Headway adjustment | Equation 21-13; Exhibit 21-12 | `headway_adjustment` (free function), `Awsc::step4_headway_adjustments` | Turning proportions p_LT/p_RT (decimal, from lane volumes), heavy-vehicle proportion p_HV (decimal), geometry group | `AwscLane.headway_adjustment` (s) |
| Steps 5-11 — Departure headway iteration | Equations 21-14 through 21-28 (two-lane framework); 21-34 through 21-45 (three-lane framework); Exhibits 21-13 through 21-16 | `Awsc::iterate_departure_headways`, `Awsc::departure_headway_for`, `base_saturation_headway` | Lane flow rates (veh/h), headway adjustment (s), geometry group; initial h_d = `INITIAL_DEPARTURE_HEADWAY_S` = 3.2 s; convergence tolerance `CONVERGENCE_TOLERANCE_S` = 0.1 s | `AwscLane.departure_headway` (s), `degree_of_utilization` (x, unitless); `Awsc.iterations` (count) |
| Step 12 — Capacity | Bisection search on subject-lane flow until x = 1.0 | `Awsc::capacity_of_lane`, `Awsc::step12_capacities` (not called by `analyze`) | Approach/lane reference; searches flow 0-3,600 veh/h | `AwscLane.capacity` (veh/h) |
| Step 13 — Service time | Equation 21-29 | Inline in `Awsc::step13_14_16_lane_delay` (`ts = h_d - m`) | Converged departure headway h_d (s), move-up time m (s, via `GeometryGroup::move_up_time`) | `AwscLane.service_time` (s) |
| Step 14 — Control delay and LOS | Equation 21-30; Exhibit 21-8 | `Awsc::step13_14_16_lane_delay`, `common::delay::control_delay_awsc`, `common::los_tables::los_unsignalized` | Service time, departure headway, degree of utilization x, analysis period T (h) | `AwscLane.control_delay` (s/veh), `los` |
| Step 15 — Approach/intersection delay and LOS | Equations 21-31, 21-32 | `Awsc::step15_aggregate_delay`, `common::delay::aggregate_control_delay` | Per-lane control delay and flow rate | `AwscApproach.control_delay`/`los`; `Awsc.intersection_delay`/`intersection_los` |
| Step 16 — 95th percentile queue | Equation 21-33 | `Awsc::queue_95` | Degree of utilization x, departure headway h_d (s), analysis period T (h) | `AwscLane.queue_95` (veh) |

### Steps 1-2: demand flow rates

```
Equation 21-12:  v_i = V_i / PHF                                              [veh/h]
  v_i = demand flow rate for movement i                                       [veh/h]
  V_i = demand volume for movement i                                          [veh/h]
  PHF = peak hour factor
Implemented in: awsc/awsc.rs::Awsc::step1_2_flow_rates
```

The code applies Equation 21-12 to each lane's total assigned demand (`AwscLane::total_volume`, the sum of the left/through/right volumes already assigned to that lane by the caller — Step 2's lane assignment is left to the caller rather than automated) rather than to a single intersection-wide movement, since AWSC lane flow rates are already the per-lane unit of analysis. `Awsc.phf` is `Option<f64>`; a `None` or non-positive value is treated as PHF = 1.0, i.e. the input volumes are already 15-min flow rates (matching Example Problem 2's fixture, which supplies flow rates directly with no PHF).

### Step 3: geometry groups (Exhibit 21-11)

`geometry_group(four_leg, subject_lanes, opposing_lanes, conflicting_lanes)` implements the full Exhibit 21-11 decision tree as a nested `match` on subject-approach lane count (1, 2, or 3+), then on `(opposing, conflicting)`. A single-lane subject approach with a single-lane opposing and conflicting approach is Group 1; two-lane opposing/single-lane conflicting is Group 3a at a T or Group 4a at a four-leg (and symmetrically 3b/4b for two-lane conflicting); anything with a three-lane opposing or conflicting approach, or a multilane subject approach paired with a three-lane cross street, falls to Group 5 (moderate multilane complexity) or Group 6 (full three-lane-everywhere complexity per Section 4). `awsc/tests.rs::test_geometry_group_exhibit_21_11` exercises every named branch.

Exhibit 21-11 note (a) is a genuine equation-like rule rather than a table entry, and the code implements it as an explicit combinatorial step before the decision tree is even consulted:

```
Exhibit 21-11, note a:  n_conflicting = max(n_conflicting_left, n_conflicting_right)     [lanes]
  n_conflicting_left  = lane count of the approach conflicting from the subject driver's left
  n_conflicting_right = lane count of the approach conflicting from the subject driver's right
  (if the two conflicting approaches have a different number of lanes, the higher of the two is used)
Implemented in: awsc/awsc.rs::Awsc::step3_geometry_groups
```

Concretely, `step3_geometry_groups` computes `conflicting = conflicting_left.lanes.len().max(conflicting_right.lanes.len())` — this `.max()` call is note (a) itself. A second, unrelated `.max(1)` is then applied to that result before it is passed into `geometry_group(...)`; this second call is only a floor that substitutes 1 for the 0-lane conflicting approach at a T intersection's missing fourth leg (a T intersection still has real subject/opposing/conflicting-from-one-side geometry, so the decision tree needs a nonzero placeholder rather than a true "no conflicting approach" case), and is not itself part of note (a).

### Step 4: headway adjustment coefficients

```
Equation 21-13:  h_adj = h_LT,adj·P_LT + h_RT,adj·P_RT + h_HV,adj·P_HV        [s]
  h_LT,adj = left-turn headway adjustment for the lane's geometry group        [s]
  h_RT,adj = right-turn headway adjustment for the lane's geometry group       [s]
  h_HV,adj = heavy-vehicle headway adjustment for the lane's geometry group    [s]
  P_LT = proportion of left-turning vehicles in the lane                      [decimal]
  P_RT = proportion of right-turning vehicles in the lane                     [decimal]
  P_HV = proportion of heavy vehicles in the lane                             [decimal]
Implemented in: awsc/awsc.rs::headway_adjustment (via Awsc::step4_headway_adjustments)
```

`H_LT_ADJ`, `H_RT_ADJ`, `H_HV_ADJ` are `[f64; 8]` arrays transcribing Exhibit 21-12's three adjustment rows across all eight geometry-group columns: left-turn adjustment is a uniform 0.2 s for Groups 1-4b and 0.5 s for Groups 5/6; right-turn adjustment is -0.6 s for Groups 1-4b and -0.7 s for Groups 5/6; heavy-vehicle adjustment is a uniform 1.7 s across every group. `headway_adjustment` then applies Equation 21-13 as a simple weighted sum, verified against the Chapter 32 AWSC Example Problem 1 (`h_adj,EB = 0.063`, `h_adj,WB = -0.116`, `h_adj,SB = -0.034` s — the last within a documented 0.002 s tolerance because the published value rounds intermediate flow rates to whole vehicles) and Example Problem 2 values.

### Steps 5-11: the departure-headway iteration and the 64-/512-state frameworks

This is the procedure the task brief specifically calls out, and the code's actual state-space size is confirmed by direct inspection of `departure_headway_for`: `frame_lanes` is chosen once per intersection by `Awsc::iterate_departure_headways` as 2 if every approach has at most two lanes, or 3 if `max_lanes() >= 3` anywhere in the intersection (Section 4's three-lane-approach extension). Inside `departure_headway_for`, `n_states = 1u32 << (3 * frame_lanes)`, i.e. `2^(3*2) = 64` states for the two-lane framework and `2^(3*3) = 512` states for the three-lane framework — **both sizes appear in the code**, selected dynamically per-intersection rather than being two separate hard-coded tables. Each "state" is one specific occupied/unoccupied combination across three approach groups relative to the subject lane — the opposing approach O, the conflicting-from-the-left approach CL, and the conflicting-from-the-right approach CR — with up to `frame_lanes` lanes tracked per group; the state is encoded as an integer whose bits, read `frame_lanes` at a time, give the occupancy pattern of each group's lanes (`(state >> (g * frame_lanes + lane)) & 1`).

#### Probability of a state (Equations 21-14, 21-15, 21-34)

```
Equation 21-14:  x = v·h_d / 3,600                                            [unitless]
  x   = degree of utilization for a framework lane
  v   = lane flow rate                                                        [veh/h]
  h_d = departure headway from the previous iteration (3.2 s at the first iteration)   [s]
Implemented in: awsc/awsc.rs::Awsc::iterate_departure_headways (x recomputed once per iteration and capped at 1.0 mid-iteration per Step 6's guidance; `(v * h / 3_600.0).min(1.0)`)

Equation 21-15 (two-lane framework):  P(i) = Π_j P(a_j)                       [unitless]
  i   = one specific occupancy combination across the six framework lanes O1, O2, CL1, CL2, CR1, CR2 (64 combinations total, Exhibit 21-14)
  a_j = 1 if a vehicle occupies framework lane j, else 0
  P(a_j) = x_j if a_j=1 and V_j>0; 1−x_j if a_j=0 and V_j>0; 0 if a_j=1 and V_j=0; 1 if a_j=0 and V_j=0    (Exhibit 21-13)
Equation 21-34 (three-lane framework): identical form over the nine framework lanes O1-3, CL1-3, CR1-3 (512 combinations, Exhibit 21-16 / Exhibit 32-15)
Implemented in: awsc/awsc.rs::Awsc::departure_headway_for (the `probs` closure builds each P(a_j); the `state` loop enumerates all `n_states = 2^(3·frame_lanes)` combinations as bit patterns and multiplies each lane's occupied/unoccupied probability into `prob`, which is P(i))
```

The code's enumeration is a direct bit-pattern walk rather than a literal transcription of Exhibit 21-14/21-16's printed rows, but it is mathematically the identical partition: iterating `state` from 0 to `n_states - 1` and reading each group's occupancy off `state`'s bits produces exactly the same 64 (or 512) combinations the exhibits list by row number, in a different but equivalent order.

#### Degree-of-conflict case probabilities (Equations 21-16 through 21-20, 21-35 through 21-39)

```
Equation 21-16 / 21-35:  P(C1) = P(1)                                          [unitless]
Equation 21-17 / 21-36:  P(C2) = Σ P(i), for i = 2..4 (two-lane) or i = 2..8 (three-lane)
Equation 21-18 / 21-37:  P(C3) = Σ P(i), for i = 5..10 (two-lane) or i = 9..22 (three-lane)
Equation 21-19 / 21-38:  P(C4) = Σ P(i), for i = 11..37 (two-lane) or i = 23..169 (three-lane)
Equation 21-20 / 21-39:  P(C5) = Σ P(i), for i = 38..64 (two-lane) or i = 170..512 (three-lane)
Implemented in: awsc/awsc.rs::Awsc::departure_headway_for (`p_case[0..5]`)
```

Rather than summing a fixed printed index range, the code classifies each enumerated state directly by which of the three approach groups (O, CL, CR) have *any* occupied lane and accumulates its probability into the matching `p_case` bucket — mathematically the same partition of the 64/512 states into five degree-of-conflict cases, just computed from occupancy rather than looked up by row number:

* **Case 1** — no conflicting vehicles present (none of O, CL, CR occupied).
* **Case 2** — only the opposing approach O is occupied.
* **Case 3** — only one conflicting approach (CL or CR, not both) is occupied.
* **Case 4** — two of the three groups (O, CL, CR) are occupied.
* **Case 5** — all three groups are occupied simultaneously.

This matches Exhibits 21-5/21-6/21-7's degree-of-conflict definitions exactly.

#### Probability adjustment (serial correlation) (Equations 21-21 through 21-26, 21-40 through 21-45)

```
Equation 21-21 / 21-40:  AdjP(1) = α·[P(C2) + 2·P(C3) + 3·P(C4) + 4·P(C5)] / 1              [unitless]
Equation 21-22 / 21-41:  AdjP(case 2 states) = α·[P(C3) + 2·P(C4) + 3·P(C5) − P(C2)] / n_c2 [unitless]
Equation 21-23 / 21-42:  AdjP(case 3 states) = α·[P(C4) + 2·P(C5) − 3·P(C3)] / n_c3         [unitless]
Equation 21-24 / 21-43:  AdjP(case 4 states) = α·[P(C5) − 6·P(C4)] / n_c4                    [unitless]
Equation 21-25 / 21-44:  AdjP(case 5 states) = −α·10·P(C5) / n_c5                            [unitless]
  α = serial-correlation coefficient = 0.01 (0.00 disables the correlation adjustment)         (code: `ALPHA`)
  n_c2, n_c3, n_c4, n_c5 = combinatorial divisors, the count of state indices spanned by each case:
    two-lane framework:   n_c2=3, n_c3=6,  n_c4=27,  n_c5=27   (index ranges 2-4, 5-10, 11-37, 38-64)
    three-lane framework: n_c2=7, n_c3=14, n_c4=147, n_c5=343  (index ranges 2-8, 9-22, 23-169, 170-512)
Equation 21-26 / 21-45:  P'(i) = P(i) + AdjP(i)                                [unitless]
Implemented in: awsc/awsc.rs::Awsc::departure_headway_for (`adj[0..5]`, and `p_adj = prob + adj[case-1]` applied per enumerated state)
```

The code derives the divisors from `m = 2^frame_lanes − 1` (the number of nonzero occupancy patterns per approach group) as `n_c2 = m`, `n_c3 = 2m`, `n_c4 = 3m²`, `n_c5 = m³`: for `frame_lanes = 2`, `m = 3` gives `(3, 6, 27, 27)`, and for `frame_lanes = 3`, `m = 7` gives `(7, 14, 147, 343)` — both derivations were checked against the HCM's literal divisors (3/6/27/27 in Equations 21-22 through 21-25; 7/14/147/343 in Equations 21-41 through 21-44) and match exactly. Because Equations 21-21 through 21-25 (and their three-lane counterparts) assign the *same* AdjP value to every state index within a case's range, the code's `p_adj = prob + adj[case-1]` — applying one adjustment value per case to every state in that case — is algebraically identical to looking up a distinct `AdjP(i)` for each `i` and adding it via Equation 21-26/21-45.

#### Saturation headway and departure headway (Equations 21-27, 21-28)

```
Equation 21-27:  h_si = h_base + h_adj                                        [s]
  h_base = base saturation headway for state i's degree-of-conflict case, geometry group, and (Groups 5/6 only) vehicle count (Exhibit 21-15)   [s]
  h_adj  = headway adjustment from Equation 21-13 (Step 4)                    [s]
Implemented in: awsc/awsc.rs::base_saturation_headway (h_base lookup) and Awsc::departure_headway_for (`h_si = base_saturation_headway(...) + h_adj`)

Equation 21-28:  h_d = Σ P'(i)·h_si, for i = 1..64 (two-lane) or i = 1..512 (three-lane)    [s]
Implemented in: awsc/awsc.rs::Awsc::departure_headway_for (`h_d += p_adj * h_si`, accumulated over every enumerated combination with nonzero probability)
```

`base_saturation_headway(case, vehicles, group)` looks up Exhibit 21-15's base saturation headway by case, geometry group, and (for Groups 5/6 only) the total vehicle count across the occupied lanes; Groups 1-4b use a single value per case regardless of vehicle count since those geometries cap conflicting lanes at one or two per group.

#### Fully worked example: degree-of-conflict Case 1 (two-lane framework)

Case 1 is the simplest of the five and corresponds to exactly one enumerated state: `state = 0`, i.e. every bit in the framework's six lanes (O1, O2, CL1, CL2, CR1, CR2) is unoccupied.

1. **State probability (Equation 21-15).** With every `a_j = 0`, `P(a_j) = 1 − x_j` for any lane carrying demand (`V_j > 0`) and `P(a_j) = 1` for any lane with no demand (`V_j = 0`). So `P(1) = Π_j (1 − x_j)` over the six framework lanes — the probability that none of the opposing or conflicting lanes has a vehicle present.
2. **Case probability (Equation 21-16).** Because state 1 is the only combination with zero occupied groups, `P(C1) = P(1)` directly (no summation needed).
3. **Adjustment (Equation 21-21).** `AdjP(1) = α·[P(C2) + 2·P(C3) + 3·P(C4) + 4·P(C5)] / 1`; unlike the other four adjustments, the divisor is exactly 1 because Case 1 spans only the single state `i = 1`.
4. **Adjusted probability (Equation 21-26).** `P'(1) = P(1) + AdjP(1)`.
5. **Saturation headway (Equation 21-27).** `h_s1 = h_base(case=1, group) + h_adj`; Exhibit 21-15's Case 1 row is a single value per geometry group regardless of vehicle count (there are, by definition, no vehicles present in Case 1) — e.g. 3.9 s for Group 1, 4.5 s for Groups 5 and 6 (`BY_CASE[0]` / the `(1, _) => 4.5` arm in `base_saturation_headway`).
6. **Contribution to Equation 21-28.** This state contributes the term `P'(1)·h_s1` to the sum that produces `h_d`.

In code, this is `state = 0` on the first pass through the loop in `departure_headway_for`: `occupied = [0, 0, 0]` for all three groups, which the `match` on `(occupied[0] > 0, occupied[1] > 0, occupied[2] > 0)` classifies as `(false, false, false) => 1` (Case 1), `prob` accumulates as the product of `1.0 - p_occ` over every framework lane, and the same `prob` is pushed into both `p_case[0]` (`P(C1)`) and the `combos` vector for the later `h_d` accumulation. The remaining 63 (or 511) states for the two-lane (three-lane) framework follow the identical mechanics for Cases 2 through 5; see `awsc/awsc.rs::Awsc::departure_headway_for` for the full enumeration rather than retyping all 64/512 rows here.

Finally, the whole procedure iterates (`Awsc::iterate_departure_headways`) by feeding each lane's new `x = v·h_d/3,600` back into the next round's occupancy probabilities, capped at 100 iterations and declared converged at `CONVERGENCE_TOLERANCE_S` = 0.1 s change in any lane's headway — matching Exhibit 21-10 Step 11's literal convergence text quoted in the module header ("If the values change by more than 0.1 s ... repeat").

`awsc/tests.rs::test_ep2_departure_headway_iteration` specifically exercises the 512-state (three-lane) framework against Example Problem 2's published `h_d,EB,1 ~= 8.19 s`, `x_EB,1 ~= 0.1274`, confirming the framework selection logic and the larger state space both function correctly; `test_ep1_departure_headway_iteration` exercises the 64-state framework against Example Problem 1.

### Step 12: capacity via bisection

Step 12 is not itself a numbered HCM equation — the HCM text (Steps 12a-12e) describes it as a search procedure: pick a trial subject-lane flow, rerun Steps 5-11 to get the resulting degree of utilization, and adjust the trial flow up or down until `x = 1.0`. The code implements this search structure as follows.

`capacity_of_lane` clones the entire `Awsc` per trial, sets the subject lane's flow rate to a trial value, re-runs `iterate_departure_headways(0.01)` (a tighter, cheaper convergence tolerance than the main analysis's 0.1 s), and reads back the resulting degree of utilization. The search has two phases:

1. **Upper-bound expansion.** Starting from `hi = 400` veh/h, `lo = 0`, the trial flow `hi` is increased in 200 veh/h steps (each step re-running the full departure-headway iteration at that trial flow) until `x_at(hi) >= 1.0` or `hi` reaches the 3,600 veh/h cap; each expansion also advances `lo` to the previous `hi`, so `[lo, hi]` always brackets the point where `x` crosses 1.0.
2. **Bisection.** For up to 30 iterations, the midpoint `mid = 0.5·(lo + hi)` is evaluated; if `x_at(mid) < 1.0` the bracket's lower bound moves up to `mid` (capacity is higher), otherwise the upper bound moves down to `mid`. The loop stops early once the bracket narrows to less than 1 veh/h. The final capacity estimate is the bracket's midpoint.

`test_ep1_step12_capacity` documents a known, explained gap versus the literal published number: the HCM Chapter 32 text reports "approximately 720 veh/h" for the EB lane, but a naive `v/x` estimate gives 748 veh/h; a bisection on exact (unrounded) flow rates converges to about 704 veh/h, and the published 720 reflects the HCM's own coarser spreadsheet search rather than an exact closed form — the test therefore uses a +-20 veh/h tolerance and additionally asserts the qualitative fact that the computed capacity is below the naive 748 veh/h estimate (confirming the approach-interaction effect is present, even if its exact magnitude differs slightly from the published rounding). This is the same discrepancy already documented in Deviations below; it is not a new finding.

### Steps 13-16: service time, delay, LOS, queue, and aggregation

```
Equation 21-29:  t_s = h_d − m                                                [s]
  h_d = converged departure headway (Equation 21-28)                          [s]
  m   = move-up time: 2.0 s (Geometry Groups 1-4b), 2.3 s (Groups 5-6)         [s]
Implemented in: awsc/awsc.rs::Awsc::step13_14_16_lane_delay (`ts = h_d - m`) and GeometryGroup::move_up_time

Equation 21-30:  d = t_s + 900·T·[(x−1) + √((x−1)² + h_d·x/(450·T))] + 5       [s/veh]
  t_s = service time (Equation 21-29)                                         [s]
  x   = degree of utilization = v·h_d/3,600                                   [unitless]
  h_d = departure headway                                                     [s]
  T   = analysis period, h (0.25 h for 15 min; code: `Awsc.analysis_period_h`) [h]
  5   = deceleration/acceleration-to/from-stop constant                       [s]
Implemented in: awsc/awsc.rs::Awsc::step13_14_16_lane_delay (via common::delay::control_delay_awsc)

Equation 21-31:  d_a = Σ(d_i·v_i) / Σ(v_i)                                    [s/veh]
  d_a = approach control delay
  d_i = control delay for lane i                                              [s/veh]
  v_i = flow rate for lane i                                                  [veh/h]
Implemented in: awsc/awsc.rs::Awsc::step15_aggregate_delay (via common::delay::aggregate_control_delay)

Equation 21-32:  d_intersection = Σ(d_a·v_a) / Σ(v_a)                         [s/veh]
  d_a = approach control delay (Equation 21-31)                               [s/veh]
  v_a = approach flow rate, the sum of the approach's lane flow rates         [veh/h]
Implemented in: awsc/awsc.rs::Awsc::step15_aggregate_delay (via common::delay::aggregate_control_delay, invoked a second time over the approach-level (delay, flow) pairs)

Equation 21-33:  Q_95 ≈ (900·T/h_d)·[(x−1) + √((x−1)² + h_d·x/(150·T))]        [veh]
  x   = degree of utilization = v·h_d/3,600                                   [unitless]
  h_d = departure headway                                                     [s]
  T   = analysis period                                                       [h]
Implemented in: awsc/awsc.rs::Awsc::queue_95
```

`control_delay_awsc` (Equation 21-30) takes `t_s`, `h_d`, `x`, and `T` directly rather than recomputing them, and its formula is algebraically the same overloaded-queueing shape used by `control_delay_unsignalized` (Chapter 20) and `control_delay_roundabout` (Chapter 22) — see `common-infrastructure.md`. LOS uses `los_unsignalized` per Exhibit 21-8, whose note (a) restricts LOS to a pure function of control delay for approaches and the intersection as a whole (the `false` literal passed as the `vc_gt_1` argument in `step15_aggregate_delay` reflects this — approach/intersection LOS never gets forced to F purely from an individual lane's v/c). Equation 21-33's 95th-percentile queue (`queue_95`) is algebraically identical in form to the Chapter 20 and Chapter 22 queue equations modulo the substitution of `h_d` for `3,600/c` (and the divisor 150·T rather than 450·T used in the delay equation — both were verified against the HCM 7th Edition Chapter 21 EPUB text and match exactly, including the divisor difference between Equations 21-30 and 21-33).

## Deviations

No `docs/hcm/VERIFICATION.md` file exists at this branch's tip. The one documented, test-visible deviation is the Step 12 capacity discrepancy described above (`test_ep1_step12_capacity`'s doc comment): the published "approximately 720 veh/h" is a coarse HCM spreadsheet estimate, and an exact bisection on unrounded flow rates converges closer to 704 veh/h; the test asserts against the published value at a wide (+-20 veh/h) tolerance while separately asserting the qualitative approach-interaction effect (computed capacity below the naive v/x estimate) holds regardless of the exact number. No other `VERIFY-HCM`-style comments appear in `awsc.rs`. No new discrepancies were found while cross-checking the code against the HCM 7th Edition Chapter 21 EPUB text (Sections 3 and 4) for this pass: Equations 21-12 through 21-33 (including the full probability/adjustment/case-classification machinery of Equations 21-14 through 21-28 and 21-34 through 21-45, the exact combinatorial divisors 3/6/27/27 and 7/14/147/343, and the Equation 21-30/21-33 delay and queue forms) all match the published equations exactly.

## Validation

Fixtures live at `tests/ExampleCases/hcm/Awsc/case1.json` (HCM Chapter 32 AWSC Example Problem 1: single-lane, three-leg T intersection, 2% heavy vehicles, PHF = 0.95) and `case2.json` (AWSC Example Problem 2: four-leg multilane intersection with two-lane EB/WB approaches and three-lane NB/SB approaches, exercising the 512-state framework, 2% heavy vehicles, 15-min flow rates with no PHF). The Rust integration test is `tests/chapter21_integration.rs` (`test_awsc_example_problem_1_full_pipeline`, `test_awsc_example_problem_1_capacity`, `test_awsc_example_problem_2_full_pipeline`, `test_awsc_fixture_roundtrip`); the Python-bound equivalent is `tests/test_chapter21_integration.py`. Declared tolerances (module doc comment of `chapter21_integration.rs`): LOS exact, control delays within +-0.5 s/veh, departure headways within +-0.1 s (widened to +-0.15 s for the 512-state Example Problem 2 case at individual assertion sites, and capacity to +-20 veh/h per the Step 12 note above). Example Problem 1 reproduces h_d,EB = 4.97 s, h_d,WB = 4.74 s, h_d,SB = 5.70 s (Exhibit 32-21); t_s,EB = 2.97 s; d_EB = 13.0 s/veh (LOS B), d_WB = 13.5 s/veh, d_SB = 10.6 s/veh; intersection delay 12.8 s/veh (LOS B); Q95,EB = 2.9 veh; EB lane capacity ~720 veh/h. Example Problem 2 reproduces h_d,EB,1 = 8.19 s, x_EB,1 = 0.1274, t_s,EB,1 = 5.89 s, d_EB,1 = 12.1 s/veh (LOS B), d_EB,2 = 16.1 s/veh (LOS C); approach delays d_EB = 15.3 (LOS C), d_WB = 14.3, d_NB = 13.1, d_SB = 12.6 s/veh; intersection delay 14.0 s/veh (LOS B); Q95,EB,1 = 0.4 veh. Additional per-step unit tests in `src/hcm/awsc/tests.rs` spot-check the geometry-group decision tree (all eight groups and their T-vs-four-leg variants), the Exhibit 21-12 headway-adjustment coefficients for both example problems, the Exhibit 21-15 base saturation headway table across representative case/group/vehicle-count combinations, the first-iteration degree-of-utilization values before any headway feedback, and the Equation 21-33 queue formula in isolation.

## Deferred

`Awsc::step12_capacities` (the full-intersection lane-capacity sweep) is implemented and tested but intentionally excluded from `analyze()` because of its cost (each of the up to 12 lanes in a three-lane, four-leg intersection requires its own ~30-step bisection, each bisection step itself running a full up-to-100-iteration departure-headway convergence); callers who need capacity call it explicitly. No pedestrian-impedance extension is implemented for AWSC (unlike Chapters 20 and 22, Chapter 21's HCM text does not define one). No `VERIFY-HCM` items or `todo!()` stubs were found in `awsc.rs`; every documented Chapter 21 equation from Section 3 and Section 4 appears to be wired into the `analyze()` pipeline except for the capacity step noted above.
