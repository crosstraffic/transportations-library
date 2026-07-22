# Changelog

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
