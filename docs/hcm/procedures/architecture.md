# Library Architecture (Post-Restructure)

This document describes the module layout introduced by the `feat/hcm-restructure` branch (commit `8493fd3`, "refactor: reorganize HCM modules into per-chapter layout"), which moved the library from a flat `src/*.rs` layout to one module per HCM chapter plus shared `common`/`support` trees. It is written for the library author to check that the restructuring preserved behavior and that the compatibility shims are load-bearing for downstream consumers, in particular the "semantic firewall" / validator tooling that imports the pre-restructure paths.

## Module layout

`src/hcm/mod.rs` is the root of the HCM tree. Its module declarations are:

```
pub mod common;
pub mod utils;
pub mod chapter12;
pub mod chapter13;
pub mod chapter14;
pub mod chapter15;
```

Each `chapterNN` directory contains one file per HCM methodology in that chapter, plus a `mod.rs` that re-exports them:

| Directory | Files | HCM content |
|---|---|---|
| `src/hcm/chapter12/` | `basicfreeways.rs`, `managed_lanes.rs`, `mod.rs` | Ch. 12 basic freeway/multilane segments and managed-lane segments |
| `src/hcm/chapter13/` | `weaving.rs`, `mod.rs` | Ch. 13 weaving segments |
| `src/hcm/chapter14/` | `merge_diverge.rs`, `mod.rs` | Ch. 14 merge/diverge segments |
| `src/hcm/chapter15/` | `twolanehighways.rs` (2,495 lines), `mod.rs` | Ch. 15 two-lane highways, motorized and bicycle methodologies |

`src/hcm/chapter15/mod.rs` is an 8-line re-export shim (`pub mod twolanehighways; pub use twolanehighways::*;`) — the pattern is the same for every `chapterNN/mod.rs`: a thin `pub mod` + `pub use` wrapper, no logic lives in the `mod.rs` files themselves.

### `common/` vs `support/`

The split is functional, not per-chapter:

- **`src/hcm/common/`** (declared `pub mod common;` in `src/hcm/mod.rs`) holds types and tables that are genuinely shared *inputs* to multiple chapters' HCM procedures: `mod.rs` (1,053 lines — lane marking types, `LevelOfService`, `FacilityType`, the NEMA movement/delay/gap-acceptance types consumed by later chapters, and the "semantic firewall" validators `validate_lane_width`/`validate_shoulder_width`/`validate_horizontal_class`/`validate_passing_type`/`validate_speed_curvature`, constraint IDs `SF-001`..`SF-005`), `adjustment_factors.rs` (1,169 lines — Chapter 11 Capacity/Speed Adjustment Factor tables for weather, incidents, and work zones per Exhibits 11-20/11-21, reused wherever CAF/SAF applies), and `pce_table.rs` (697 lines — passenger-car-equivalent lookup tables for heavy vehicles).
- **`src/hcm/utils/`** (declared `pub mod utils;`) holds cross-cutting engineering/topology helpers that are not themselves an HCM chapter procedure: `geometric.rs` (AASHTO Green Book curve/sight-distance safety checks), `topology.rs` (OpenDRIVE-mapped network types: `Node`, `NetworkSegment`, `Intersection`, `Direction`), `traffic_flow.rs` (HCM Chapter 2 fundamental D=V/S and v/c relationships), and `constraints.rs` (a second, independent parameter-validation system — see "Two validation systems" below).

Note there are **two separate, non-unified parameter-validation frameworks** in the tree: `support/constraints.rs` (`get_constraints_json()`, `validate_two_lane_highway(...)`, JSON-exportable ranges, used by the `copython::support` Python bindings) and `common/mod.rs`'s "semantic firewall" (`SF-001`..`SF-005`, `ValidationResult`/`ValidationError` with `constraint_id`, used only by `tests/semantic_firewall_test.rs`). Both encode overlapping HCM Exhibit 15-8/15-10/15-11/15-22 ranges independently; they can drift out of sync since neither delegates to the other. This is worth the author's attention even though it is not strictly a restructure regression.

`src/hcm/utils/mod.rs` also has commented-out `// pub mod core;` with an explanatory comment: the `src/hcm/utils/core/` directory (`core.rs`, 329 lines defining `HcmCore`; `mod.rs`; `tests.rs`, 175 lines) exists on disk but is **not part of the compiled module tree** — the comment states it is "unfinished WIP inherited from main (was never declared in the module tree there either; does not compile)" and is deferred to the Chapter 10 freeway-facilities work. A reviewer diffing file lists against `mod.rs` declarations should not expect `HcmCore` to be reachable from `transportations_library::hcm::utils::core` on this branch.

## `with-python` feature gating and the copython register pattern

`Cargo.toml` declares:

```toml
[dependencies]
pyo3 = { version = "0.23.3", features = ["extension-module"], optional = true }

[features]
default = []
with-python = ["pyo3"]
pybindings = ["with-python"]   # deprecated alias for with-python; will be removed in a future release
```

PyO3 bindings live under `src/copython/`, gated per-module in `src/lib.rs` (`mod copython;` unconditionally, but `pub use copython::py_transportationslibrary::*;` only `#[cfg(feature = "with-python")]`) and per-file in `src/copython/mod.rs`, where every `pub mod chapterNN;` / `pub mod support;` / `pub mod py_transportationslibrary;` declaration carries `#[cfg(feature = "with-python")]`. `pyproject.toml`'s `[tool.maturin] features = ["with-python"]` is what turns the flag on for the Python wheel build; a plain `cargo build`/`cargo test` (no `--features with-python`) compiles none of `src/copython/*`.

The register pattern: each `copython::chapterNN.rs` file defines its `#[pyclass]` wrapper structs (thin newtypes wrapping the corresponding `hcm::chapterNN` Rust struct in an `inner` field, e.g. `copython::chapter15::SubSegment { inner: LibSubSegment }`) and ends with a `pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()>` that calls `m.add_class::<...>()` for each wrapper. `src/copython/py_transportationslibrary.rs` is the single `#[pymodule] fn transportations_library(...)` entry point; its body is just three `register()` calls (`chapter12::register(m)?`, `chapter15::register(m)?`, `support::register(m)?`) plus the module docstring/version. Adding a new chapter's Python bindings means: write `copython::chapterNN.rs` with its own `register()`, add `#[cfg(feature = "with-python")] pub mod chapterNN;` to `copython/mod.rs`, and add one `super::chapterNN::register(m)?;` line to `py_transportationslibrary.rs`. Chapters 13 and 14 (weaving, merge/diverge) do not yet have `copython` wrappers on this branch — only chapter12 and chapter15 are exposed to Python.

`src/copython/support.rs` is not a chapter wrapper; it exposes the `support::constraints` validation module as two free functions, `get_constraints()` and `validate_input(...)`, registered the same way (`m.add_function(wrap_pyfunction!(...))` instead of `m.add_class`).

## Fixture and test conventions

Test fixtures live under `tests/ExampleCases/hcm/<Chapter>/caseN.json`, one JSON-serialized `TwoLaneHighways`/equivalent struct per HCM manual worked example. On this branch: `tests/ExampleCases/hcm/BasicFreeways/{case1,case2}.json` and `tests/ExampleCases/hcm/TwoLaneHighways/{case1,case2,case3,case4,case_study1}.json`. `tests/twolanehighways_test.rs`'s `read_test_files()` walks the `TwoLaneHighways` directory and explicitly filters to filenames containing `"case"` but not `"case_study"`, so `case_study1.json` is not exercised by any Rust test on this branch (grep of `tests/*.rs` finds no reference to `case_study`) — it appears to be reserved for a downstream script/notebook rather than `cargo test`.

Two complementary test styles are used:
- **Exact-value regression tests** (`tests/twolanehighways_test.rs`): load each `caseN.json` into a `TwoLaneHighways`, round-trip it through `Segment::new`/`SubSegment::new` via `initialize_test_case()`, run individual step methods, and `assert_eq!` against hand-transcribed expected arrays (e.g. `identity_vertical_class_test`'s `ans_min`/`ans_max`, `determine_demand_flow_test`'s `ans_demand_flow_i/_o`/`ans_capacity`). Comparisons are exact (`assert_eq!` on `.round()`ed or `math::round_to_significant_digits(_, 3)`-ed values), not epsilon-based — the `assert_approx_eq` crate is declared as a dependency in `Cargo.toml` but is not actually invoked anywhere in `tests/*.rs` on this branch (`grep -rn assert_approx_eq tests/` is empty).
- **Range/sanity tests** (`tests/twolanehighways_integration.rs`, `tests/semantic_firewall_test.rs`): assert plausibility bounds (e.g. `capacity >= 1100 && capacity <= 1700`, `ffs >= 20.0 && ffs <= 80.0`) rather than exact manual values, plus the `SF-00N` constraint-boundary tests described above. `tests/basicfreeways_test.rs` uses `assert_eq!` against literal expected numbers for Chapter 12.

`tests/common/mod.rs` is a shared test-utility module (`create_sample_highway()`, `create_complex_highway()`, `create_rural_highway()`, `create_suburban_highway()`, `run_complete_analysis()`, `analyze_facility()`) used by multiple test binaries; it is not itself a `#[test]` file. It also defines `load_test_data_files()`, which reads from `src/ExampleCases/hcm/TwoLaneHighways/` (note: `src/ExampleCases`, not `tests/ExampleCases`) — that path does not exist in the repository, so this helper silently returns an empty vec with a printed warning if called; nothing in the current test suite calls it, but it is a latent path bug if someone wires it up later.

Python-side tests are `tests/test_twolanehighways_integration.py` (pytest; skips the whole module via `pytest.skip(..., allow_module_level=True)` if `transportations_library` isn't importable, i.e. if the `with-python` wheel hasn't been built) and `tests/__init__.py` (empty).

## Backward-compatible re-export shims

`src/hcm/mod.rs` ends with:

```rust
// Backward-compatible module paths from the pre-chapter layout.
// Deprecated: import via `hcm::chapterNN::*` or `hcm::utils::*` instead.
pub use chapter12::basicfreeways;
pub use chapter12::managed_lanes;
pub use chapter13::weaving;
pub use chapter14::merge_diverge;
pub use chapter15::twolanehighways;
pub use support::{constraints, geometric, topology, traffic_flow};

pub mod adjustment_factors {
    pub use crate::hcm::common::adjustment_factors::*;
}
```

and `src/lib.rs` re-exports through *that* flat path again: `pub use crate::hcm::basicfreeways::*; pub use crate::hcm::twolanehighways::*; pub use crate::hcm::topology::*;` etc., i.e. `transportations_library::twolanehighways::Segment` and `transportations_library::hcm::twolanehighways::Segment` both resolve to the same type post-restructure, identically to how the pre-restructure flat-module crate exposed them. `src/copython/chapter15.rs` itself imports via the old flat alias (`use crate::hcm::twolanehighways::{Segment as LibSegment, ...}`), showing the shim is exercised by in-tree code, not just a courtesy for external callers.

This matters because any downstream validator/consumer code written against the pre-restructure flat paths (`transportations_library::twolanehighways::*`, `transportations_library::constraints::*`, etc.) would fail to compile if these shims were dropped, even though no chapter's actual logic moved or changed during the restructure. That this is a real, exercised path rather than a hypothetical one is confirmed in-tree: `tests/twolanehighways_test.rs` itself imports via the flat alias (`use transportations_library::twolanehighways::{BicycleLOS, Segment, SubSegment, TwoLaneHighways};`), and `src/copython/chapter15.rs` imports via `use crate::hcm::twolanehighways::{Segment as LibSegment, ...}` rather than the new `hcm::chapter15::twolanehighways` path. Removing a shim is therefore a breaking change to any code — in-repo test files included — written against the pre-restructure paths, independent of whether the underlying HCM methodology itself changed.

## Validation

No `docs/hcm/VERIFICATION.md` exists on this branch (`git ls-tree -r feat/hcm-restructure` has no such path), so there is no existing deviation ledger to cross-reference from this document; deviations noted here and in `chapter15.md` are sourced directly from code comments and doc-comment/implementation mismatches found while reading this branch's tree.

## Deferred

- Unifying `support::constraints` and `common`'s semantic-firewall validators into one source of truth.
- Deciding the fate of `src/hcm/utils/core/` (`HcmCore`): salvage into Chapter 10 freeway-facilities work or delete, per the comment in `support/mod.rs`.
- Wiring `copython` bindings for Chapters 13 and 14 (weaving, merge/diverge), which currently have no Python-exposed classes.
- Fixing or removing `tests/common/mod.rs::load_test_data_files()`, whose hardcoded path (`src/ExampleCases/hcm/TwoLaneHighways/`) does not exist in the repository.
- Deciding whether `case_study1.json` fixtures are meant to be exercised by `cargo test` at all, since the current file filter explicitly excludes them.
