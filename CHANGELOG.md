# Changelog

## Unreleased

### Added

- **HCM Edition 7.1 support, selectable per segment.** Edition 7.1 (November 2025) replaces Chapters 13, 14, 27, and 28 with new weaving, merge, and diverge methodologies from NCHRP Research Report 1038; the rest of the 7th Edition is unchanged. `HcmVersion` (`V7`, `V7_1`) selects the edition on `WeavingSegment` and `RampSegment`, and `run_analysis()` dispatches on it. From Python: `WeavingSegment(version="7.1", ...)`, a settable `.version` property, `.analysis_v7_1()` for the full typed result, and `hcm_versions()` / `hcm_latest_version()` / `hcm_version_changes_chapter()` for building a version picker.
- **Edition 7.1 Chapter 13** (`src/hcm/weaving/v7_1.rs`): overall speed as an equivalent basic segment less a speed impedance (Eqs 13-7 through 13-14), capacity solved analytically from the 35 pc/mi/ln breakdown density (Eqs 13-15 through 13-19), the simple weaving volume estimation method (Eqs 13-2 through 13-6), and Exhibit 13-7 LOS. Reproduces Chapter 27 Example Problems 1, 2, and 3 value for value.
- **Edition 7.1 Chapter 14** (`src/hcm/merge_diverge/v7_1.rs`): merge and diverge speed impedance (Eqs 14-4/14-5), the capacity quadratic (Eqs 14-8 through 14-14), all three capacity checks (Exhibits 14-8, 14-9, 14-10), and Exhibit 14-2 LOS. Reproduces Chapter 28 Example Problems 1 and 2 value for value.
- `WeavingSegment` gains `nw_rf`, `nw_fr`, and `nw_rr`, the weaving-lane counts the Edition 7.1 configuration weighting reads. The 7th Edition methodology ignores them.

### Changed

- The Exhibit 12-6 basic-segment primitives (breakpoint, capacity, and the Equation 12-1 speed-flow curve) are now free functions in `basicfreeways`, which `BasicFreeways` delegates to and Chapters 13 and 14 share. Behavior is unchanged; there is now one definition of each parameter rather than one per calling chapter.

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
