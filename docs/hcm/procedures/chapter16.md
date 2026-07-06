# HCM Chapter 16 — Urban Street Facilities, Motorized Vehicle Methodology

This document walks through the Rust translation of HCM 7th Edition Chapter 16, Section 3 ("Motorized Vehicle Mode," EPUB `112_Ch16_03.xhtml`), which aggregates the per-segment Chapter 18 measures of an ordered sequence of segments in one direction of travel into facility-level base free-flow speed, travel speed, spatial stop rate, LOS, and a facility-wide automobile traveler perception score. LOS criteria (Exhibit 16-3) are transcribed from `111_Ch16_02.xhtml`. The code lives entirely in `src/hcm/urban_facilities/urban_facilities.rs`, which re-uses the Chapter 18 `UrbanSegment` engine and LOS/perception-score helpers directly (`crate::hcm::urban_segments::exhibits::exhibit_18_1_los`, `crate::hcm::urban_segments::urban_segments::{traveler_perception_score, through_capacity_uncontrolled, UrbanSegment}`) rather than re-implementing them, since Exhibit 16-3's travel-speed thresholds are, per the module doc comment, "value-for-value identical" to Chapter 18's Exhibit 18-1, and Chapter 16 Equation 16-1 is identical to Chapter 18 Equation 18-2.

## Step-by-step walkthrough

| HCM Step | Equation/Exhibit | Rust function | File | Inputs (units) | Outputs (units) |
|---|---|---|---|---|---|
| Step 1 — Facility base free-flow speed | Eq. 16-2, `S_fo,F = ΣL_i / Σ(L_i/S_fo,i)` | `facility_base_ffs` (thin wrapper over `length_weighted_harmonic_mean`) | `urban_facilities.rs` | `&[(f64, f64)]` per-segment `(L_i ft, S_fo,i mi/h)` pairs | `S_fo,F` (mi/h) |
| Step 2 — Facility travel speed | Eq. 16-3, `S_T,F = ΣL_i / Σ(L_i/S_T,seg,i)` | `facility_travel_speed` (same `length_weighted_harmonic_mean` helper) | `urban_facilities.rs` | `&[(f64, f64)]` per-segment `(L_i ft, S_T,seg,i mi/h)` pairs | `S_T,F` (mi/h) |
| Step 3 — Facility spatial stop rate | Eq. 16-4, `H_F = Σ(H_seg,i L_i) / ΣL_i` | `facility_spatial_stop_rate` | `urban_facilities.rs` | `&[(f64, f64)]` per-segment `(L_i ft, H_seg,i stops/mi)` pairs | `H_F` (stops/mi); `None` unless every segment reports a stop rate |
| (Step 3, continued) — Facility perception score | Ch. 18 Eqs. 18-17..18-22, with `H_F`/facility-wide P_LTL substituted | `traveler_perception_score` (re-used from `urban_segments::urban_segments`) | `urban_segments/urban_segments.rs` (called from `aggregate_segment_summaries`) | `H_F` (stops/mi), `prop_left_turn_lanes` (P_LTL, decimal) | `perception_score` (dimensionless), `None` unless both inputs present |
| Step 4 — LOS | Exhibit 16-3 (= Exhibit 18-1) with the critical v/c footnote | `exhibit_16_3_los` (delegates to `exhibit_18_1_los`) | `urban_facilities.rs` | `travel_speed_mph` (S_T,F), `base_ffs_mph` (S_fo,F), `critical_vc_gt_1: bool` | `LevelOfService` |
| Step 4, continued — Critical v/c and poorest-segment report | Exhibit 16-3 footnote; Chapter 16 Step 4 discussion | `aggregate_segment_summaries` (critical-v/c fold and `los_rank`-ordered poorest-segment fold) | `urban_facilities.rs` | per-segment `vc_ratio: Option<f64>`, `los: Option<LevelOfService>` | `critical_vc_ratio: Option<f64>` (max across segments), `poorest_segment_los: Option<LevelOfService>` |

`aggregate_segment_summaries(segments: &[SegmentSummary], prop_left_turn_lanes: Option<f64>) -> Result<FacilityResults, String>` is the single function implementing Steps 1-4 together; it validates that every segment has positive length and positive base/travel speeds, then runs Equations 16-2 through 16-4 and the LOS/perception-score logic described above. `SegmentSummary` is the Exhibit 16-7 "HCM method output" input record (`length_ft`, `base_ffs_mph`, `travel_speed_mph`, `spatial_stop_rate_stops_mi: Option<f64>`, `vc_ratio: Option<f64>`, `los: Option<LevelOfService>`) — it can be supplied directly (e.g., published example-problem values) or produced by `UrbanFacility::segment_summaries()` from already-analyzed Chapter 18 `UrbanSegment`s. `FacilityResults` additionally carries `length_ft` (ΣL_i, ft), `travel_time_s` and `base_free_flow_travel_time_s` (both `3,600 L/(5,280 S)`, s, computed directly in `aggregate_segment_summaries` rather than being separately-numbered HCM equations).

`UrbanFacility` (the top-level struct, `segments: Vec<UrbanSegment>`, `prop_left_turn_lanes: Option<f64>`, `spillback_inputs: Option<Vec<SpillbackCheckInput>>`) exposes two entry points: `analyze()`, which runs every segment's Chapter 18 `analyze()` first and then aggregates, and `aggregate()`, which aggregates already-computed segment measures without re-running Chapter 18 (for when per-segment measures were supplied directly, e.g. `case1.json`/`case2.json` below). `spatial_stop_rate` is only computed when every segment in the facility reports a stop rate, per the code comment "a partial aggregation would misstate the facility value" — this is a code-level completeness guard, not a separate HCM rule.

### Step equations in full

#### Step 1 — Facility base free-flow speed

```
Equation 16-2:  S_fo,F = Σ(i=1→m) L_i / Σ(i=1→m) (L_i / S_fo,i)     [mi/h]
  L_i    = length of segment i                                      (ft)
  m      = number of segments on the facility
  S_fo,i = base free-flow speed for segment i                       (mi/h)
Implemented in: urban_facilities/urban_facilities.rs::facility_base_ffs
```

This is the length-weighted harmonic mean of the segment base free-flow speeds (equivalently, total facility length divided by the total travel time at the base free-flow speed); the shared helper `length_weighted_harmonic_mean` computes both Equation 16-2 and Equation 16-3.

#### Step 2 — Facility travel speed

```
Equation 16-3:  S_T,F = Σ(i=1→m) L_i / Σ(i=1→m) (L_i / S_T,seg,i)   [mi/h]
  S_T,F     = travel speed for the facility                         (mi/h)
  L_i       = length of segment i                                   (ft)
  m         = number of segments on the facility
  S_T,seg,i = travel speed of through vehicles for segment i         (mi/h)
Implemented in: urban_facilities/urban_facilities.rs::facility_travel_speed
```

#### Step 3 — Facility spatial stop rate and perception score

```
Equation 16-4:  H_F = Σ(i=1→m) (H_seg,i · L_i) / Σ(i=1→m) L_i        [stops/mi]
  H_F     = spatial stop rate for the facility                      (stops/mi)
  H_seg,i = spatial stop rate for segment i                         (stops/mi)
  L_i     = length of segment i                                     (ft)
  m       = number of segments on the facility
Implemented in: urban_facilities/urban_facilities.rs::facility_spatial_stop_rate
```

`H_F` is computed only when every segment reports a stop rate (a code-level completeness guard, not a separate HCM rule, per the note above). When it is available, Chapter 16's text directs that "the equations in Step 10, Section 3, of Chapter 18" (Equations 18-17 through 18-22) be reused with `H_F` substituted for `H_seg` and a facility-wide `P_LTL` substituted for `P_LTL,seg`:

```
Equations 18-17..18-22 (facility-substituted):
  I_a,F = 1 + P_BCDEF + P_CDEF + P_DEF + P_EF + P_F                  [dimensionless]
  P_x   = (1 + e^(a_x − 0.253·H_F + 0.3434·P_LTL))⁻¹
  a_x   = −1.1614, 0.6234, 1.7389, 2.7047, 3.8044 for x = BCDEF, CDEF, DEF, EF, F respectively
  H_F   = facility spatial stop rate (Equation 16-4)                 (stops/mi)
  P_LTL = facility-wide proportion of intersections with a left-turn lane (or bay)  (decimal)
Implemented in: urban_facilities/urban_facilities.rs::aggregate_segment_summaries (calls urban_segments/urban_segments.rs::traveler_perception_score with H_F and the facility-wide P_LTL substituted for H_seg and P_LTL,seg)
```

#### Step 4 — Through-movement capacity, critical v/c, and LOS

Equation 16-1 supplies the through-movement capacity input (for the v/c ratio in `SegmentSummary::vc_ratio`) when the downstream boundary intersection is a two-way STOP-controlled intersection and the through movement is uncontrolled — Chapter 20 does not otherwise provide a capacity procedure for that movement:

```
Equation 16-1:  c_th = 1,800 · (N_th − 1 + p*_0,j)                   [veh/h]
  c_th   = through-movement capacity                                 (veh/h)
  N_th   = number of through lanes, shared or exclusive               (ln)
  p*_0,j = probability that there will be no queue in the inside through lane (dimensionless);
           equal to 1.0 if a left-turn bay is provided for left turns from the major street,
           otherwise computed per Chapter 20's queue-free-probability procedure
  1,800  = HCM default saturation flow rate per lane assumption        (veh/h/ln)
Implemented in: urban_facilities/urban_facilities.rs::through_capacity_uncontrolled (re-exported from urban_segments/urban_segments.rs::through_capacity_uncontrolled, Equation 18-2 — identical form)
```

The `p*_0,j` computation itself (Chapter 20's queue-free-probability procedure) is Chapter 18/20 territory and is not re-documented here; see `docs/hcm/VERIFICATION.md`'s Chapter 18 section for the HCM6/HCM7 equation-numbering note on that citation ("Eq 20-43" vs. Eqs 20-29..20-34).

LOS itself (Exhibit 16-3) is a table lookup, not an equation — its travel-speed thresholds by base free-flow speed are value-for-value identical to Chapter 18's Exhibit 18-1 (already transcribed as `exhibit_18_1_los`), so only the critical v/c footnote logic is new at the facility level:

```
Exhibit 16-3 footnote (critical v/c rule):
  vc_crit = max(i=1→m) { vc_ratio_i }                                 [dimensionless]
  LOS = F  if vc_crit > 1.0, else per Exhibit 16-3 travel-speed thresholds
  vc_ratio_i = through-movement v/c ratio at segment i's downstream boundary intersection (dimensionless)
Implemented in: urban_facilities/urban_facilities.rs::aggregate_segment_summaries (critical_vc fold) and urban_facilities/urban_facilities.rs::exhibit_16_3_los (delegates to urban_segments/exhibits.rs::exhibit_18_1_los)
```

Chapter 16 Step 4 also directs reporting the poorest-performing segment's own LOS for context; that is a simple max-over-`los_rank` fold with no HCM equation number, implemented in `urban_facilities/urban_facilities.rs::aggregate_segment_summaries` alongside the critical-v/c fold.

### Spillback check (queue-storage-ratio hook)

Chapter 16 requires the methodology not be applied to segments experiencing sustained spillback; the full Chapter 29, Section 3 iterative evaluation (re-running the Chapter 18/19 engines under a capacity constraint) is out of scope for this branch. `SpillbackCheckInput` (`available_storage_ft`, `back_of_queue_veh_ln`, `avg_vehicle_spacing_ft` — defaulting to 25.0 ft/veh, the HCM Equation 31-155 value at 0% heavy vehicles) implements only the queue-storage-ratio flag: `storage_ratio()` computes `R_Q = L_h Q / L_a` (the Equation 19-36 form) and `spillback_expected()` returns true when `R_Q > 1.0`. `UrbanFacility::aggregate()` populates `spillback_flags: Option<Vec<bool>>` from `spillback_inputs` when supplied, one flag per segment, and errors if the lengths mismatch. This is a screening hook for the analyst to invoke the full Chapter 29 procedure or an alternative tool — it is not itself the Chapter 29 procedure.

## Validation

Fixtures live under `tests/ExampleCases/hcm/UrbanFacilities/` as `case1.json`, `case2.json`, and `case3.json`, exercised by `tests/chapter16_integration.rs` and mirrored (case1 only) by `tests/test_chapter16_integration.py`.

`case1.json` and `case2.json` reproduce HCM Chapter 29, Section 5, Example Problem 1 (Exhibits 29-39 through 29-49), eastbound and westbound respectively, using published per-segment Chapter 18 outputs directly (via `aggregate()`, not `analyze()`). Because Segments 2-4 of the published five-segment facility are not individually reported in the extracted exhibit text (the fixture reuses Segments 1 and 5's values for them), the facility-level travel speed and stop rate only approximately reproduce the published aggregate: `test_case1_example_problem_1_eastbound` asserts facility length exactly (5,280 ft, tolerance 1e-9), base FFS at ±0.05 mi/h against the published 40.1 (exact, since every segment base FFS is individually published), travel speed at ±0.6 mi/h against 22.6 (approximate), spatial stop rate at ±0.15 stops/mi against 1.83 (approximate), and LOS/poorest-segment-LOS exactly (C / D). `test_case2_example_problem_1_westbound` follows the same pattern with published values 22.2 mi/h (±0.8) and 1.93 stops/mi (±0.25).

`case3.json` instead drives the full Chapter 18 pipeline (`analyze()`) over three identical copies of the Chapter 30, Example Problem 1 eastbound segment (documented in `chapter18.md`), so the facility-level aggregation of three identical segments must reproduce that segment's own published Exhibit 30-36 values exactly: `test_case3_chapter18_driven_facility` asserts base FFS 40.78 mi/h (±0.02), travel speed 23.67 mi/h (±0.02), spatial stop rate 1.61 stops/mi (±0.02), critical v/c 0.52 (±0.005, `= 968/1848`), LOS C, and poorest-segment LOS C. This test also contains the length-weighted-speed cross-check the task calls out: it independently recomputes `Σ L_i / Σ (L_i/S_T,seg,i)` from the analyzed segments' own `segment_length_ft`/`travel_speed_mph` fields and asserts the facility's `travel_speed_mph` matches that identity to 1e-12 — i.e., that `aggregate_segment_summaries`'s Equation 16-3 implementation is exactly the harmonic-mean identity applied to the Chapter 18 engine's own outputs, not merely close to it. `test_round_trip` additionally confirms a JSON round-trip of a fully analyzed facility preserves the computed travel speed to 1e-12.

Unit tests in `src/hcm/urban_facilities/tests.rs` spot-check the aggregation equations directly: `test_eq_16_2_example_problem_1_base_ffs` and `test_harmonic_aggregation` against hand-computed harmonic means, `test_eq_16_4_spatial_stop_rate` against a hand-computed length-weighted arithmetic mean, `test_exhibit_16_3_los` against the shared Exhibit 18-1 table, `test_aggregate_example_problem_1_eastbound` (a unit-level version of the case1 integration check), `test_critical_vc_rule_forces_f` and (mirrored in `tests/chapter16_integration.rs::test_case1_vc_footnote_forces_los_f`) confirming a single segment's v/c > 1.0 forces facility LOS F regardless of speed, `test_aggregate_rejects_empty_and_invalid` for the input-validation error paths, `test_facility_speed_equals_length_weighted_ch18_computation` (the unit-level counterpart of the case3 cross-check, run directly against Chapter-18-analyzed segments rather than through the JSON fixture), `test_facility_json_round_trip`, `test_spillback_check_hook`, and `test_eq_16_1_through_capacity` (confirming the re-exported Chapter 18 Equation 18-2 implementation).

`tests/test_chapter16_integration.py` exercises the PyO3 bindings against `case1.json`, including its own `test_length_weighted_identity` (the Python-side counterpart of the Rust cross-check) and `test_published_values`, `test_los`, `test_json_round_trip`.

## Deferred

The Chapter 29, Section 3 sustained-spillback evaluation procedure (the full iterative re-analysis of a spillback-affected segment under a capacity constraint) is deferred; only the queue-storage-ratio screening flag (`SpillbackCheckInput`) is implemented. The Chapter 16 pedestrian, bicycle, and transit LOS methodologies are out of scope entirely and are not present in this module. Facility-level automobile traveler perception score requires both a facility spatial stop rate (all segments reporting) and an analyst-supplied facility-wide `prop_left_turn_lanes`; when either is absent it is simply `None` rather than partially estimated.
