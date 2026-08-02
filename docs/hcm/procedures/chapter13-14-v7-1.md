# HCM Edition 7.1: Chapters 13 and 14 — Procedure Walkthrough

HCM Edition 7.1 (November 2025) is not a new manual. It is a set of four replacement chapters — 13 and 14, plus their supplements 27 and 28 — issued against the 7th Edition and based on NCHRP Research Report 1038, *Update of Highway Capacity Manual: Merge, Diverge, and Weaving Methodologies*. Every other chapter of the 7th Edition stands unchanged, which is why this library treats the edition as a per-segment input rather than a global upgrade.

## Selecting an edition

`HcmVersion` (`src/hcm/common/version.rs`) has two variants, `V7` and `V7_1`, and defaults to `V7`. `WeavingSegment` and `RampSegment` each carry a `version` field, and `run_analysis()` dispatches on it. From Python the same selection is a constructor argument or a settable property:

```python
seg = tl.WeavingSegment(version="7.1", ..., nw_rf=2, nw_fr=1)
seg.run_analysis()                  # "C"
result = json.loads(seg.analysis_v7_1())
```

`hcm_versions()`, `hcm_latest_version()`, and `hcm_version_changes_chapter(version, chapter)` support a version picker. The last of these encodes that only Chapters 13, 14, 27, and 28 differ, so selecting 7.1 for a Chapter 19 analysis is a visible no-op rather than a silent one.

The default stays at the 7th Edition deliberately. The two editions are different models, not successive refinements of one, so flipping the default would change the numbers of every existing caller without any of them editing a line. Choosing 7.1 is a deliberate act.

## What changed conceptually

Both replacement chapters abandon their 7th Edition structure in favor of a common shape:

1. Compute the speed of an **equivalent basic segment** — the same lanes, demand, and free-flow speed, without the turbulence — from Chapter 12's Equation 12-1, which Edition 7.1 leaves untouched.
2. Subtract a **speed impedance** term calibrated to the configuration and the ramp or weaving flow.
3. Derive **capacity analytically** by asking at what per-lane flow the segment reaches a breakdown density of 35 pc/mi/ln. Because Equation 12-1 is quadratic in flow, this is a quadratic in capacity with a closed-form root.
4. Convert speed to density and read LOS from bands keyed to that same 35 pc/mi/ln threshold.

The 7th Edition, by contrast, estimated separate weaving and nonweaving speeds from lane-changing rates (Chapter 13) and modeled the distribution of flow into Lanes 1 and 2 (Chapter 14). Edition 7.1 has no lane-changing rate model and no lane-distribution model at all.

## Chapter 13, Freeway Weaving Segments

Implementation: `src/hcm/weaving/v7_1.rs`. Validated against Chapter 27 Example Problems 1, 2, and 3 in `tests/chapter13_v7_1_integration.rs`.

| Manual step | HCM Eq./Exhibit | Rust item | Output (units) |
|---|---|---|---|
| Step 2: adjust volumes | Eq 13-1; Eqs 13-2–13-6 for the simple estimation method | `analyze_v7_1`, `estimate_movement_flows` | four movement flows (pc/h) |
| Step 3: overall speed | Eq 13-7 (`S_o = S_b − SIW`), Eq 13-8/13-9 (W), Eq 13-10 (SIW), Eq 13-13 (two-sided) | `weaving_intensity`, `configured_weaving_flow`, `speed_impedance` | S_o (mi/h) |
| Step 4: capacity | Eqs 13-15–13-19, Eq 13-20 (d/c) | `weaving_capacity_per_lane` | C_W (pc/h/ln) |
| Step 5: density and LOS | Eq 13-21, Exhibit 13-7 | `determine_weaving_los` | D (pc/mi/ln), LOS |

The configuration enters through a weight of `(LC + 1)/(NW + 1)` applied to each weaving movement: more required lane changes raise a movement's influence, more lanes from which the maneuver can be made lower it. This is why the new `nw_rf`, `nw_fr`, and `nw_rr` input fields exist; the 7th Edition methodology never reads them.

`WeavingClass` (Simple, Complex, TwoSided) selects the Exhibit 13-13/13-14 regression coefficients and is **derived** from the four configuration parameters rather than supplied, so it cannot contradict them. A weave is simple only when `LC_RF = LC_FR = NW_RF = NW_FR = 1`.

Two coefficient rows deserve a note. Exhibit 13-14's two-sided row is identical to Exhibit 13-13's simple row, value for value (0.016, 0.021, 0.181, 3.217). That is what the manual prints, verified against the source PDF, not a transcription slip: the two models differ in which flow they weight (Equation 13-13 uses the ramp-to-ramp flow alone), not in their coefficients.

## Chapter 14, Freeway Merge and Diverge Segments

Implementation: `src/hcm/merge_diverge/v7_1.rs`. Validated against Chapter 28 Example Problems 1 and 2 in `tests/chapter14_v7_1_integration.rs`.

| Manual step | HCM Eq./Exhibit | Rust item | Output (units) |
|---|---|---|---|
| Step 1: adjust volumes | Eq 14-1 | `demand_flows_v7_1` | v_F, v_R (pc/h) |
| Step 2: speed | Eqs 14-2/14-3, Eq 14-4 (SIM), Eq 14-5 (SID) | `merge_speed_impedance`, `diverge_speed_impedance` | S_M or S_D (mi/h) |
| Step 3: capacity | Eq 14-8 with Eqs 14-9–14-11 (merge) or 14-12–14-14 (diverge); Exhibits 14-8, 14-9, 14-10 | `ramp_capacity_per_lane`, `neighboring_freeway_capacity`, `ramp_roadway_capacity` | C_M or C_D (pc/h/ln) |
| Step 4: density and LOS | Eqs 14-15/14-16, Exhibit 14-2 | `analyze_v7_1` | D (pc/mi/ln), LOS |

The merge and diverge models are deliberately asymmetric. A merge works on the flow downstream of the on-ramp, `(v_F + v_R)/N`, and scales turbulence by `v_R/L_A`. A diverge works on the mainline flow approaching the off-ramp, `v_F/N`, which already contains the exiting vehicles, and scales turbulence by `v_R/L_D^0.536`.

The printed capacity coefficients confirm the speed equations: 0.143 and 71.4 in Equations 14-10/14-11 are `0.00408 × 35` and `0.00408 × 35 × 500`; 0.0049 and 2.45 in Equations 14-13/14-14 are the same products of 0.00014. Deriving the quadratic by hand reproduces the printed forms exactly.

Edition 7.1 applies no Lane 5 deduction on 10-lane freeways. That deduction (7th Edition Equation 14-27, Exhibit 14-19) existed to feed the lane-distribution model, and there is no such model here.

## LOS thresholds

Exhibit 13-7 and Exhibit 14-2 print identical bands, because both chapters now key LOS to the same breakdown density:

| LOS | Density (pc/mi/ln) |
|---|---|
| A | 0–11 |
| B | >11–18 |
| C | >18–25 |
| D | >25–30 |
| E | >30–35 |
| F | >35, or demand exceeds capacity |

They share one implementation in `src/hcm/common/los_tables.rs` with two chapter-named entry points, so a future change to one exhibit cannot silently drift from the other.

These are not the 7th Edition's bands, and the differences are not cosmetic. Under the 7th Edition weaving LOS F began at 43 pc/mi/ln; it now begins at 35. Under the 7th Edition Exhibit 14-3, a merge or diverge density above 35 pc/mi/ln was LOS E and only insufficient capacity produced LOS F; density alone now produces it. A segment that read LOS D under the 7th Edition can read LOS F under Edition 7.1 on identical inputs.

## Reproduction status

| Example problem | Configuration | Reproduced |
|---|---|---|
| Ch 27 EP1 | Complex 0–1 weave, four-lane freeway | every published intermediate value |
| Ch 27 EP2 | Simple weave, demands as pc/h | every published intermediate value |
| Ch 27 EP3 | Two-sided weave, three lanes | every published intermediate value |
| Ch 28 EP1 | Isolated one-lane on-ramp, four-lane freeway | every published intermediate value |
| Ch 28 EP2 | Two adjacent one-lane off-ramps, six-lane freeway | both ramps, every published intermediate value |

Not implemented: Chapter 27 EP4–EP7 (design, service volume tables, ML access with cross-weave), Chapter 28 EP3–EP5 (adjacent ramp interaction, left-hand ramps, service flow rates), and the Section 4 extensions of both chapters (two-lane ramps without a lane add, major merges and diverges, lane additions and drops). The Chapter 13 `estimate_movement_flows` helper implements the simple weaving volume estimation method but no published example exercises it end to end; it is covered by a self-consistency test only.
