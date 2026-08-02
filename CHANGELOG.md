# Changelog

## Unreleased

### Added

- **HCM Edition 7.1 support, selectable per segment.** Edition 7.1 (November 2025) replaces Chapters 13, 14, 27, and 28 with new weaving, merge, and diverge methodologies from NCHRP Research Report 1038; the rest of the 7th Edition is unchanged. `HcmVersion` (`V7`, `V7_1`) selects the edition on `WeavingSegment` and `RampSegment`, and `run_analysis()` dispatches on it. From Python: `WeavingSegment(version="7.1", ...)`, a settable `.version` property, `.analysis_v7_1()` for the full typed result, and `hcm_versions()` / `hcm_latest_version()` / `hcm_version_changes_chapter()` for building a version picker.
- **Edition 7.1 Chapter 13** (`src/hcm/weaving/v7_1.rs`): overall speed as an equivalent basic segment less a speed impedance (Eqs 13-7 through 13-14), capacity solved analytically from the 35 pc/mi/ln breakdown density (Eqs 13-15 through 13-19), the simple weaving volume estimation method (Eqs 13-2 through 13-6), and Exhibit 13-7 LOS. Reproduces Chapter 27 Example Problems 1, 2, and 3 value for value.
- **Edition 7.1 Chapter 14** (`src/hcm/merge_diverge/v7_1.rs`): merge and diverge speed impedance (Eqs 14-4/14-5), the capacity quadratic (Eqs 14-8 through 14-14), all three capacity checks (Exhibits 14-8, 14-9, 14-10), and Exhibit 14-2 LOS. Reproduces Chapter 28 Example Problems 1 and 2 value for value.
- `WeavingSegment` gains `nw_rf`, `nw_fr`, and `nw_rr`, the weaving-lane counts the Edition 7.1 configuration weighting reads. The 7th Edition methodology ignores them.

### Breaking

- **`RampSegment::determine_los` and `RampSegment::run_analysis` now return `Option<LevelOfService>`**, and the PyO3 `RampSegment.run_analysis()` returns `None` instead of a letter. A major merge operating under capacity has no 7th Edition level of service; the previous code set `self.los = None` but returned `LevelOfService::E`, so a caller reading the return value got a letter the HCM does not sanction. Under Edition 7.1 this case always yields a letter, because Exhibit 14-2 extends its criteria to major merges and diverges.
- **`BasicFreeways::lc_r` and `lc_l` are `f64`, not `u32`.** The Exhibit 12-21 note says "Interpolate for noninteger values of right-side lateral clearance", which an integer field cannot express, and the previously dead interpolating lookup is now the implementation. Integer clearances read exactly the exhibit values, so existing results do not move; Python callers can pass ints or floats.

### Fixed

- **Base capacity was computed from the speed-adjusted free-flow speed.** The December 2022 corrections to the 7th Edition change Equations 12-6 and 12-7 from FFS_adj to FFS and state that "FFS used in the adjusted capacity computation is the original and unadjusted free-flow speed". `basicfreeways`, the Chapter 10 facilities engine, and the new Edition 7.1 modules all passed `ffs x saf`, so a speed adjustment factor suppressed capacity a second time on top of CAF. The breakpoint still uses FFS_adj; that asymmetry is deliberate. No published example problem could catch this because they all set SAF = 1.0. Results change wherever SAF varies: weather and incident scenarios, work zones, ATDM. Chapter 25 Example Problem 4 shifts as a result, documented in `docs/hcm/VERIFICATION.md`.
- **Minor-street left turns used the wrong Stage II conflicting movement.** The December 2022 corrections change Equations 20-14 and 20-15 and the matching Exhibit 20-16 rows so that a minor-street left turn's Stage II conflicting flow counts the opposing minor-street through movement, not the major-street right turn. The worked examples had always used the corrected form, which is why Chapter 32 Example Problems 3 and 4 previously reproduced only through explicit `conflicting_flow_overrides`. With the correction applied they reproduce natively, and those overrides are gone from the fixtures.
- **Minor-street crossing movements used pre-correction Exhibit 20-14 factors.** The same December 2022 corrections (page 20-18) swap the conflicting-movement-6 entries between movement 8 Stage II and movement 11 Stage I, so f(8,6) is the "channelized 0 / all others 1" form and f(11,6) the shared-lane 0.5 form. Example Problem 3's v_c,II,8 = 532 and v_c,I,11 = 482 now reproduce natively, and no TWSC fixture carries `conflicting_flow_overrides` any more. This entry was missed in the first pass over the corrections document and caught on re-review.
- **Eight-lane P_FM could go negative.** The Exhibit 14-8 regression `0.2178 - 0.000125 v_R` falls below zero past roughly 1,742 pc/h of ramp demand, putting a negative flow in Lanes 1 and 2 and a negative density downstream. Both eight-lane forms and the Exhibit 14-9 base form are now clamped to a proportion, with a VERIFY-HCM note that a clamp signals an input outside the regression's fitted range.
- Verified that Exhibit 25-17 and Exhibit 10-6 are identical value for value, closing an open question about whether `planning.rs` reusing `los_freeway_facility` was a mismatch. It is not; the module doc now records the check.
- **The Edition 7.1 capacity quadratics refuse off-domain inputs instead of returning the wrong root.** Below FFS_adj = C_b,adj/45 (roughly SAF under 0.71 at 75 mi/h) the leading coefficient goes non-positive and `(-b + sqrt)/(2a)` is no longer the larger root; both solvers now return `None` there, consistent with their None-not-NaN design.

### Changed

- The Exhibit 12-6 basic-segment primitives (breakpoint, capacity, and the Equation 12-1 speed-flow curve) are now free functions in `basicfreeways`, which `BasicFreeways` delegates to and Chapters 13 and 14 share. Behavior is unchanged; there is now one definition of each parameter rather than one per calling chapter.
- The Chapter 10 facilities engine's `basic_speed` and `base_capacity_pc` delegate to those same primitives instead of re-inlining the formulas (values identical), and the capacity-loop variable that held the unadjusted FFS is no longer named `ffs_adj_input`.

### Notes

- The default edition remains the 7th, so existing callers keep their numbers. The two editions are different models rather than successive refinements, and on identical inputs a segment can land several pc/mi/ln and a full LOS letter apart between them.
- Edition 7.1 LOS bands are tighter than the 7th Edition's at every letter. Weaving LOS F now begins at 35 rather than 43 pc/mi/ln, and a merge or diverge density above 35 pc/mi/ln is now LOS F on its own where the 7th Edition read it as LOS E.
- Book discrepancies found while implementing these chapters are recorded in `docs/hcm/VERIFICATION.md`; the walkthrough is `docs/hcm/procedures/chapter13-14-v7-1.md`.

## 0.2.0 — 2026-07

### Corrected (affects results computed with 0.1.10–0.1.12)

- **Chapter 12 passenger-car-equivalent (PCE) specific-upgrade tables were the wrong HCM exhibit.** In 0.1.10 through 0.1.12 the 30%, 50%, and 70% single-unit-truck tables were byte-identical, all holding Exhibit 12-28 (70% SUT) equivalents. Any 30% or 50% SUT analysis in those versions used incorrect passenger-car equivalents and produced incorrect heavy-vehicle factors and densities. The tables are now generated directly from the HCM source and verified byte-for-byte against it. **Versions 0.1.10, 0.1.11, and 0.1.12 are yanked; do not use them for SUT-mix freeway analyses.**
- **Service-flow-rate rounding** now rounds free-flow speed to the nearest 5 mi/h with no interpolation, per Exhibits 12-37 and 12-38 (previously rounded up).
- Several Chapter 12 paths that silently returned a default value off the tabulated domain now return an error instead.

### Breaking

- **`sut_percentage` default changed 50 → 0.** A segment that never sets it now reads the general-terrain exhibit (12-25) instead of a specific-upgrade table, which shifts computed LOS for such callers — e.g. a stated freeway-design segment moves LOS E → D as its density corrects from 35.55 to 34.28. Set `sut_percentage` explicitly (30/50/70) to reach the specific-upgrade tables. This changes numbers for correct-looking code, so it is called out here deliberately.
- `adjustment_heavy_vehicle_factor`, `estimate_demand_volume`, `estimate_number_of_lanes`, `estimate_lanes_from_aadt`, `run_operational_analysis`, and both `determine_*_max_service_flow_rate` methods now return `Result`; previously silent-default paths now surface errors.
- The Python constructor gained a `sut_percentage` argument, plus `set_target_los`, `e_t`, `f_hv`, and `estimate_number_of_lanes` methods.
- `src/hcm` chapter modules were renamed to topic-named subfolders.

### Notes

- Reproducibility gate unchanged: River Falls facility follower density 5.223 (LOS C).
