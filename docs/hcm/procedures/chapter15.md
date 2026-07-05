# Chapter 15: Two-Lane Highways — Procedure Walkthrough

HCM 7th Edition Chapter 15 covers the motorized-vehicle methodology (Section 3, "Motorized Vehicle Methodology") for Passing Constrained (PC), Passing Zone (PZ), and Passing Lane (PL) segments, plus the bicycle LOS methodology (Section 4). Both are implemented in a single file, `src/hcm/chapter15/twolanehighways.rs` (2,495 lines) on the `feat/hcm-restructure` branch, re-exported through `src/hcm/chapter15/mod.rs`. The motorized methodology walks each `Segment` through an 11-step sequence (free-flow speed → average speed → percent followers → follower density → LOS), driven by the `TwoLaneHighways` struct holding a `Vec<Segment>` plus facility-wide geometry (`lane_width`, `shoulder_width`, `apd`, `pmhvfl`, `l_de`); the bicycle methodology is a self-contained `BicycleLOS` struct with its own 5-step chain. This document covers the motorized steps in manual order, then the bicycle method, then documents the two internal unit conventions that most easily trip up a caller.

## Step-by-step walkthrough (motorized methodology)

| Manual step | HCM Eq./Exhibit | Rust method | File | Inputs | Output (units) |
|---|---|---|---|---|---|
| Step 1: segment length applicability | Exhibit 15-10 | `identify_vertical_class` | `twolanehighways.rs` | `seg_num` (uses stored `vertical_class`, `passing_type`) | `(min_length, max_length)` in mi |
| Step 2: demand flow rates & capacity | Eq 15-1; capacity Exhibit 15-5 | `determine_demand_flow` | `twolanehighways.rs` | `seg_num` (uses `volume`, `volume_op`, `phf`, `phv`, `passing_type`, `vertical_class`) | `(demand_flow_i, demand_flow_o veh/h, capacity veh/h)`; also mutates `segments[seg_num]` |
| Step 3: vertical alignment classification | Exhibit 15-11 | `determine_vertical_alignment` | `twolanehighways.rs` | `seg_num` (`length` mi, `grade` %) | vertical class `i32` 1-5; re-invokes `identify_vertical_class` if the class changed |
| Step 4: free-flow speed | Eq 15-2 to 15-6, coefficients Exhibit 15-12 | `determine_free_flow_speed` | `twolanehighways.rs` | `seg_num`, facility `lane_width`/`shoulder_width`/`apd` (ft, ft, pts/mi) | FFS, mi/h |
| Step 5: average speed | Eq 15-7 to 15-16, coefficients Exhibits 15-13/15-14/15-19/15-20 | `estimate_average_speed` (delegates per-subsegment/tangent work to private `calc_speed`) | `twolanehighways.rs` | `seg_num` | `(average_speed mi/h, horizontal_class 0-5)` |
| Step 6: percent followers | Eq 15-17 to 15-23, coefficients Exhibits 15-24 to 15-29 | `estimate_percent_followers` (delegates to private `calc_percent_followers`) | `twolanehighways.rs` | `seg_num` | percent followers, 0-100 |
| Steps 7-8: passing-lane flow split, lane-specific speed/PF, midpoint follower density | Eq 15-24 to 15-27 (flow split), Eq 15-34 (FD at PL midpoint) | `determine_follower_density_pl` (uses helper `estimate_average_speed_sf` / `estimate_percent_followers_sf` for the faster-lane/slower-lane sub-calculations) | `twolanehighways.rs` | `seg_num`, facility `pmhvfl` | `(fd, fd_mid)` followers/mi/ln |
| Step 8 (PC/PZ path): follower density | Eq 15-35 | `determine_follower_density_pc_pz` | `twolanehighways.rs` | `seg_num` (`avg_speed`, `pf`, `flow_rate`) | followers/mi/ln |
| Step 9: adjustment for upstream passing lane | Eq 15-30 to 15-33 (unlabeled in code comments; see below) | `determine_adjustment_to_follower_density` | `twolanehighways.rs` | `seg_num` | follower-density adjustment, followers/mi/ln |
| Step 10: segment LOS | Exhibit 15-6 | `determine_segment_los` | `twolanehighways.rs` | `seg_num`, `s_pl` (posted speed limit, mi/h — **not** average speed, see Unit footguns), `cap` (veh/h) | LOS char `'A'..'F'` |
| Step 11: facility LOS | Eq 15-39 | `determine_facility_los` | `twolanehighways.rs` | length-weighted `fd` (followers/mi/ln), `s_pl` (mi/h) | LOS char `'A'..'F'` |

The recommended per-segment call order is documented directly on `TwoLaneHighways` (module-level `# Analysis Workflow` doc comment) and matches `tests/common/mod.rs::run_complete_analysis()`: `identify_vertical_class` → `determine_demand_flow` → `determine_vertical_alignment` → `determine_free_flow_speed` → `estimate_average_speed` → `estimate_percent_followers` → (`determine_follower_density_pl` if `passing_type == 2` else `determine_follower_density_pc_pz`) → `determine_adjustment_to_follower_density`.

### Step 3 detail: vertical class thresholds

`determine_vertical_alignment` is a large, hand-transcribed decision tree over `(seg_length, seg_grade)` bins directly encoding Exhibit 15-11's upgrade/downgrade tables (separate branches for `seg_grade >= 0.0` vs. negative grade, the latter negating `seg_length` before bucketing). There is a length-bin gap worth flagging to the reviewer: the upgrade branch has an `else if seg_length > 0.4 && seg_length <= 0.5` bucket followed directly by `else if seg_length > 0.6 && seg_length <= 0.7` — **the `0.5 < seg_length <= 0.6` interval is never matched** by any explicit branch and falls through to the final `else` (the `> 1.1`-equivalent catch-all bucket), which uses coarser thresholds than the intended 0.5-0.7 mi bucket in Exhibit 15-11. This looks like a transcription gap rather than an intentional simplification.

### Step 4 detail: the `a` (HV speed-reduction) coefficient

Eq 15-4's five coefficient sets (`a0`..`a5` per vertical class 1-5) are transcribed as literal `f64` constants inline in `determine_free_flow_speed`, keyed by `if vc == 1 { ... } else if vc == 2 { ... }` etc., rather than pulled from a shared exhibit table — this duplicates the same literal-constant style used for the `b`/`c`/`d`/`f` coefficients in `calc_speed` and the `b`/`c`/`d`/`e` coefficients in `calc_percent_followers` (all three coefficient sets are re-declared per function with no shared table). A reviewer checking these against Exhibit 15-12 needs to check the literals directly in each function since there is no single canonical constants module.

### Step 5 detail: horizontal-curve adjustment and `is_hc`

`estimate_average_speed` first computes the whole-segment tangent speed via `calc_speed(..., is_hc=false, rad=0.0, sup_ele=0.0)`. If `Segment.is_hc` is `true`, it then iterates `Segment.subsegments`, converts each `SubSegment.length` from feet to miles (`/ 5280.0`, line ~1132), and for curved subsegments (`design_rad > 0.0`) recomputes speed with `calc_speed(..., is_hc=true, rad, sup_ele)`, applying the horizontal-class speed cap inside `calc_speed`: `bffshc = min(bffs, 44.32 + 0.3728*bffs - 6.868*hor_class)`, `ffshc = bffshc - 0.0255*phv`, then `shc = min(s, ffshc - mhc*sqrt(vd/1000 - 0.1))` (Eq 15-12 to 15-15 region). The final segment speed is the subsegment-length-weighted average, `tot_s / seg_length`. **If `is_hc` is left `false` (or unset — `Segment::get_is_hc()` defaults to `false`), subsegments and their curve data are silently ignored even if present**, since the `if is_hc { ... }` branch (line ~1124) is the only path that reads `subsegments`. The horizontal-class table itself (radius/superelevation → class 0-5, Exhibit 15-22) is transcribed as a 20-branch `if`/`else if` cascade on `rad`/`sup_ele` inside `calc_speed`.

One inline comment is a live author TODO worth the reviewer's direct attention: at the point where `shc` is computed (`calc_speed`, immediately before `s = shc;`), the code comment reads `// Should be ST instead of S?` — i.e., the original author was unsure whether the `min(s, ...)` term should use the pre-curve tangent speed `s` (as coded) or a separate "ST" quantity from the manual. This is exactly the kind of translation-fidelity question this documentation exercise is meant to surface.

### Step 7-8 detail: passing-lane split

`determine_follower_density_pl` computes `NumHV = round(v_d * phv/100)`, the faster-lane flow proportion `PropFlowFL = 0.92183 - 0.05022*ln(v_d) - 0.00030*NumHV` (Eq 15-25), splits `vd_fl`/`vd_sl` and heavy-vehicle percentages accordingly, computes a speed differential adjustment `sda = 2.750 + 0.00056*v_d + 3.8521*phv/100`, and combines lane speeds/percent-followers into `fd_mid = (pf_fl*vd_fl/s_mid_fl + pf_sl*vd_sl/s_mid_sl) / 200`. It also calls `determine_follower_density_pc_pz` internally to populate the ordinary endpoint `fd` alongside `fd_mid`, so both are available for Step 10/11 (Step 10 picks `fd_mid` when `passing_type == 2`, `fd` otherwise — see `determine_segment_los`). When the segment has more than one subsegment, the faster/slower-lane speeds are themselves subsegment-length-weighted using the same `/5280.0` feet→miles conversion as Step 5.

### Step 9 detail: unlabeled equations

`determine_adjustment_to_follower_density` implements the "effective distance downstream of a passing lane" logic (locating the nearest upstream PL segment, accumulating downstream length `l_d`, and computing `pf_improve`/`s_improve` via the `y_1`/`y_2`/`x_2`/`x_3`/`x_4` intermediate terms) but **carries no HCM equation-number comments at all**, unlike every other step method in this file. The manual's Eq 15-30 to 15-33 region (percent-followers and speed improvement downstream of a passing lane) is the likely source, but this should be confirmed against the manual directly since the code gives no citation to check against. The method also contains a `// TODO: if there are more than three PL section` comment on the `pl_loc` upstream-passing-lane lookup, which only tracks a single PL location (last one wins) rather than multiple.

## Bicycle LOS methodology (Section 4)

| Manual step | HCM Eq. | Rust method | File | Inputs | Output |
|---|---|---|---|---|---|
| Step 2: outside-lane flow rate | Eq 15-40 | `calculate_flow_rate_outside_lane` | `twolanehighways.rs` | `hourly_volume` (veh/h), `phf`, `num_lanes` | veh/h |
| Step 3: effective width | Eq 15-41 to 15-45 (branch selection via private `calculate_wv`) | `calculate_effective_width` | `twolanehighways.rs` | `shoulder_width`, `lane_width` (ft), `pct_on_highway_parking` (decimal), `hourly_volume` (veh/h, per lane for the 160-veh/h branch and Eq 15-45) | ft |
| Step 4: effective speed factor | Eq 15-46 | `calculate_effective_speed_factor` | `twolanehighways.rs` | `speed_limit` (mi/h) | unitless factor, `1.1199*ln(Spl-20)+0.8103` |
| Step 5: BLOS score | Eq 15-47 | `calculate_blos_score` | `twolanehighways.rs` | outputs of steps 2-4 plus `pavement_condition` (1-5 FHWA scale), `heavy_vehicle_pct` | BLOS score (typically 0.5-6.5) |
| LOS lookup | Exhibit 15-7 | `determine_bicycle_los` | `twolanehighways.rs` | BLOS score | LOS char `'A'..'F'`, thresholds ≤1.5/2.5/3.5/4.5/5.5 |
| Convenience wrapper | — | `analyze` | `twolanehighways.rs` | `&self` | `BicycleLOSResult { flow_rate_outside_lane, effective_width, effective_speed_factor, blos_score, los }` |

`calculate_effective_width` implements the three shoulder-width branches exactly as printed: Eq 15-41 (`Ws >= 8`): `We = Wv + Ws - %OHP*10`; Eq 15-42 (`4 <= Ws < 8`): `We = Wv + Ws - 2*[%OHP*(2 + Ws)]`; Eq 15-43 (`Ws < 4`): `We = Wv - %OHP*(2 + Ws)`; with `calculate_wv()` returning `W_OL + Ws` when per-lane volume exceeds 160 veh/h (Eq 15-44) and `(W_OL + Ws)*(2 - 0.005V)` otherwise (Eq 15-45). This was corrected against the manual and verified against the HCM Chapter 26 widening worked example (current We = 14 ft, proposed We = 24 ft; see Validation). `calculate_blos_score` clamps `heavy_vehicle_pct` to a maximum of 0.5 when `hourly_volume < 200.0` per the Eq 15-47 note in the manual, and guards `ln(v_ol)` against a non-positive argument by substituting `0.0` (defensive addition, not manual text).

## Unit footguns

Two unit conventions are easy to get backwards and are not enforced by the type system (both fields are plain `f64`/`Option<f64>`):

- **`Segment.spl` is the *posted* speed limit** (mi/h), and Step 4's base free-flow speed is `BFFS = 1.14 * spl` (`determine_free_flow_speed`, and duplicated inline in `estimate_average_speed` as `bffs = round_to_significant_digits(1.14 * spl, 3)` and again in `estimate_average_speed_sf` as `bffs = 1.14 * spl`) — i.e., **BFFS is derived, not itself a stored/settable field**; passing an already-adjusted FFS-like value as `spl` will silently double-inflate BFFS by 14%.
- **`SubSegment.length` is in FEET**, while **`Segment.length` is in MILES** — both are documented correctly in the field doc comments (`/// Length of subsegment, ft.` vs. `/// Length of segment, mi.`), and the conversion is applied consistently at each subsegment read site (`get_length() / 5280.0` in `estimate_average_speed` and `determine_follower_density_pl`). However, the Python binding's constructor docstring in `src/copython/chapter15.rs` (`SubSegment::new`) says *"length: Length of the sub-segment in miles (default: 0.0)"* — **that docstring is wrong**; the Rust-side field comment and every use site treat it as feet. A Python caller following the PyO3 docstring would pass miles and get a value 5,280× too small once divided by 5280.0 downstream. This is a genuine deviation between the Python-facing documentation and the Rust implementation, worth the reviewer's attention independent of anything else in this document.
- **`Segment.is_hc` gates whether horizontal-class/curve data is used at all.** Supplying `subsegments` with real `design_rad`/`sup_ele` data but leaving `is_hc` at its default (`false`, via `get_is_hc()`'s `unwrap_or(false)`) means Step 5 silently falls back to the tangent-only `calc_speed` path and never reads the subsegment curve data — there is no warning or error, the curve data is simply inert.

## Deviations

No `docs/hcm/VERIFICATION.md` exists on this branch to cross-reference (checked via `git ls-tree -r feat/hcm-restructure`), so the deviations below are called out inline rather than cross-referenced to an existing ledger entry:

1. Step 3 (`determine_vertical_alignment`) has a missing length bucket (`0.5 < length <= 0.6` mi, upgrade branch) — see "Step 3 detail" above.
2. Step 9 (`determine_adjustment_to_follower_density`) has no HCM equation-number comments, unlike every other step, and only tracks one upstream passing-lane location (`// TODO: if there are more than three PL section`).
3. `calc_speed`'s horizontal-curve speed cap carries a live author doubt comment, `// Should be ST instead of S?`.
4. The `copython::chapter15::SubSegment::new` PyO3 docstring states subsegment length is in miles; the Rust implementation and field doc comment say feet.
6. `TwoLaneHighways.apd` field doc comment says "default: 0" but `determine_free_flow_speed` reads `self.apd.unwrap_or(5.0)` — a 5.0-points/mi default, not 0, is used whenever `apd` is `None`. (Noted in `architecture.md` as well since it's a doc/code mismatch rather than an HCM-fidelity issue.)

## Validation

- **Fixtures**: `tests/ExampleCases/hcm/TwoLaneHighways/case1.json` through `case4.json` (one `TwoLaneHighways` JSON per case; `case1.json` inspected directly: single PC segment, 0.75 mi, 50 mph posted, 752 veh/h volume, PHF 0.94, PHV 5%, 12 ft lanes, 6 ft shoulders, `apd: 0.0`). `case_study1.json` also exists in the same directory but is excluded by the test file filter (see `architecture.md`) and not exercised by `cargo test`.
- **Test files / tolerance**: `tests/twolanehighways_test.rs` runs `identity_vertical_class_test`, `determine_demand_flow_test`, `determine_vertical_alignment_test` (and further step-by-step tests later in the 485-line file not fully enumerated here) against `case1-4.json`, asserting exact equality (`assert_eq!`) between hand-transcribed expected arrays and computed values rounded via `.round()` or `math::round_to_significant_digits(_, 3)` — i.e., tolerance is "exact match after rounding to the same precision as the expected value," not an epsilon comparison. `bicycle_los_test` (same file) only asserts range/ordering properties (`blos_score > 0.0`, worse conditions produce a higher score, `los` is one of `A`-`F`), not exact manual example values. `tests/twolanehighways_integration.rs` (336 lines) runs the same `case*.json` fixtures through the full step sequence and asserts HCM-plausible ranges (e.g., `capacity` in `[1100, 1700]`, `ffs` in `[20.0, 80.0]`) rather than exact figures. `tests/semantic_firewall_test.rs` (324 lines) tests only the `common::mod.rs` `validate_*` boundary functions (`SF-001`..`SF-005`), not the step methods themselves.
- **Python integration test**: `tests/test_twolanehighways_integration.py` exercises the compiled `transportations_library` Python extension if importable (skips otherwise), but its assertions are generic (`hasattr`, type checks) rather than HCM-example-value checks.

## Deferred

- No test in this branch reproduces a full HCM 7th Edition worked example end-to-end with published intermediate values cited by page/exhibit number in the test itself — the `case*.json` fixtures' provenance (whether they are transcribed from a specific manual example) is not documented in the test files or fixture directory.
- Step 9's uncited equations and the `// Should be ST instead of S?` comment are flagged above but not resolved here — resolving them requires the manual, which is outside this documentation task's scope.
- The `apd` default mismatch (item 6 above) is reported, not fixed.
