# Library Architecture (Post-Restructure)

This document describes the module layout introduced by the `feat/hcm-restructure` branch (commit `8493fd3`, "refactor: reorganize HCM modules into per-chapter layout"), which moved the library from a flat `src/*.rs` layout to one module per HCM chapter plus shared `common`/`support` trees. It is written for the library author to check that the restructuring preserved behavior and that the compatibility shims are load-bearing for downstream consumers, in particular the "semantic firewall" / validator tooling that imports the pre-restructure paths.

A later branch (`feat/hcm-topic-folder-names`) renamed every `src/hcm/chapterNN/` directory to a topic name (`chapter12` -> `basicfreeways`, `chapter15` -> `twolanehighways`, etc. — see the mapping table in that branch's PR description for the full chapter10-24 list). `src/hcm/mod.rs` now declares the topic modules directly and re-exports each one under its old chapter-number name (`pub use basicfreeways as chapter12;`, `pub use twolanehighways as chapter15;`, ...), so every `hcm::chapterNN::...` path documented below still compiles unchanged; only new code should prefer the topic names. The directory table, register pattern, and shim descriptions below use the topic names now in effect; where a chapter-number path is shown it resolves through the alias just described.

## Module layout

`src/hcm/mod.rs` is the root of the HCM tree. Its module declarations are:

```
pub mod common;
pub mod utils;
pub mod basicfreeways; // HCM Chapter 12
pub mod weaving; // HCM Chapter 13
pub mod merge_diverge; // HCM Chapter 14
pub mod twolanehighways; // HCM Chapter 15

// Chapter-number aliases (external path stability):
pub use basicfreeways as chapter12;
pub use weaving as chapter13;
pub use merge_diverge as chapter14;
pub use twolanehighways as chapter15;
```

Each topic directory contains one file per HCM methodology in that chapter, plus a `mod.rs` that re-exports them:

| Directory | Files | HCM content |
|---|---|---|
| `src/hcm/basicfreeways/` | `basicfreeways.rs`, `managed_lanes.rs`, `mod.rs` | Ch. 12 basic freeway/multilane segments and managed-lane segments |
| `src/hcm/weaving/` | `weaving.rs`, `mod.rs` | Ch. 13 weaving segments |
| `src/hcm/merge_diverge/` | `merge_diverge.rs`, `mod.rs` | Ch. 14 merge/diverge segments |
| `src/hcm/twolanehighways/` | `twolanehighways.rs` (2,495 lines), `mod.rs` | Ch. 15 two-lane highways, motorized and bicycle methodologies |

`src/hcm/twolanehighways/mod.rs` is an 8-line re-export shim (`pub mod twolanehighways; pub use twolanehighways::*;`) — the pattern is the same for every topic `mod.rs`: a thin `pub mod` + `pub use` wrapper, no logic lives in the `mod.rs` files themselves. Note the deliberate self-collision: the folder and its single inner file share the same name (`twolanehighways/twolanehighways.rs`), so both `hcm::twolanehighways::TwoLaneHighways` (via the folder's `pub use twolanehighways::*;`) and `hcm::twolanehighways::twolanehighways::TwoLaneHighways` (the inner file directly) resolve to the same type.

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

The register pattern: each `copython::<topic>.rs` file (e.g. `copython::twolanehighways.rs` for Chapter 15) defines its `#[pyclass]` wrapper structs (thin newtypes wrapping the corresponding `hcm::<topic>` Rust struct in an `inner` field, e.g. `copython::twolanehighways::SubSegment { inner: LibSubSegment }`) and ends with a `pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()>` that calls `m.add_class::<...>()` for each wrapper. `src/copython/py_transportationslibrary.rs` is the single `#[pymodule] fn transportations_library(...)` entry point; its body is just three `register()` calls (`basicfreeways::register(m)?`, `twolanehighways::register(m)?`, `support::register(m)?`) plus the module docstring/version. Adding a new chapter's Python bindings means: write `copython::<topic>.rs` with its own `register()`, add `#[cfg(feature = "with-python")] pub mod <topic>;` to `copython/mod.rs`, and add one `super::<topic>::register(m)?;` line to `py_transportationslibrary.rs`. Chapters 13 and 14 (weaving, merge/diverge) do not yet have `copython` wrappers on this branch — only chapter 12 (`basicfreeways`) and chapter 15 (`twolanehighways`) are exposed to Python.

`src/copython/support.rs` is not a chapter wrapper; it exposes the `support::constraints` validation module as two free functions, `get_constraints()` and `validate_input(...)`, registered the same way (`m.add_function(wrap_pyfunction!(...))` instead of `m.add_class`).

## Fixture and test conventions

Test fixtures live under `tests/ExampleCases/hcm/<Chapter>/caseN.json`, one JSON-serialized `TwoLaneHighways`/equivalent struct per HCM manual worked example. On this branch: `tests/ExampleCases/hcm/BasicFreeways/{case1,case2}.json` and `tests/ExampleCases/hcm/TwoLaneHighways/{case1,case2,case3,case4,case_study1}.json`. `tests/twolanehighways_test.rs`'s `read_test_files()` walks the `TwoLaneHighways` directory and explicitly filters to filenames containing `"case"` but not `"case_study"`, so `case_study1.json` is not exercised by any Rust test on this branch (grep of `tests/*.rs` finds no reference to `case_study`) — it appears to be reserved for a downstream script/notebook rather than `cargo test`.

Two complementary test styles are used:
- **Exact-value regression tests** (`tests/twolanehighways_test.rs`): load each `caseN.json` into a `TwoLaneHighways`, round-trip it through `Segment::new`/`SubSegment::new` via `initialize_test_case()`, run individual step methods, and `assert_eq!` against hand-transcribed expected arrays (e.g. `identity_vertical_class_test`'s `ans_min`/`ans_max`, `determine_demand_flow_test`'s `ans_demand_flow_i/_o`/`ans_capacity`). Comparisons are exact (`assert_eq!` on `.round()`ed or `math::round_to_significant_digits(_, 3)`-ed values), not epsilon-based — the `assert_approx_eq` crate is declared as a dependency in `Cargo.toml` but is not actually invoked anywhere in `tests/*.rs` on this branch (`grep -rn assert_approx_eq tests/` is empty).
- **Range/sanity tests** (`tests/twolanehighways_integration.rs`, `tests/semantic_firewall_test.rs`): assert plausibility bounds (e.g. `capacity >= 1100 && capacity <= 1700`, `ffs >= 20.0 && ffs <= 80.0`) rather than exact manual values, plus the `SF-00N` constraint-boundary tests described above. `tests/basicfreeways_test.rs` uses `assert_eq!` against literal expected numbers for Chapter 12.

`tests/common/mod.rs` is a shared test-utility module (`create_sample_highway()`, `create_complex_highway()`, `create_rural_highway()`, `create_suburban_highway()`, `run_complete_analysis()`, `analyze_facility()`) used by multiple test binaries; it is not itself a `#[test]` file. It also defines `load_test_data_files()`, which used to read from `src/ExampleCases/hcm/TwoLaneHighways/` — a path that does not exist, so `case_study1.json` was silently excluded from every Rust test and the helper returned an empty vec with a printed warning. The path is now `tests/ExampleCases/hcm/TwoLaneHighways/`. The `else` branch and its warning remain as a guard rather than as the normal case.

Python-side tests are `tests/test_twolanehighways_integration.py` (pytest; skips the whole module via `pytest.skip(..., allow_module_level=True)` if `transportations_library` isn't importable, i.e. if the `with-python` wheel hasn't been built) and `tests/__init__.py` (empty).

## Backward-compatible re-export shims

`src/hcm/mod.rs` now carries two back-compat layers stacked on top of the topic modules. First, the chapter-number aliases added by `feat/hcm-topic-folder-names` (see the top of this document); then, underneath those, the older pre-chapter-layout shim that predates the chapter split entirely:

```rust
// Backward-compatible module paths from the pre-chapter layout.
// Deprecated: import via the topic modules above or `hcm::utils::*` instead.
pub use basicfreeways::managed_lanes;
pub use utils::{constraints, geometric, topology, traffic_flow};

pub mod adjustment_factors {
    pub use crate::hcm::common::adjustment_factors::*;
}
```

(The old `pub use basicfreeways::basicfreeways;` / `pub use twolanehighways::twolanehighways;` lines that used to sit alongside `managed_lanes` here were removed as part of the topic-folder rename: once the folder itself is named `basicfreeways`/`twolanehighways`, re-exporting the identically-named inner file under the same top-level name is a redundant, ambiguous rebinding rather than a useful alias.)

`src/lib.rs` re-exports through *that* flat path again: `pub use crate::hcm::basicfreeways::basicfreeways::*; pub use crate::hcm::twolanehighways::twolanehighways::*; pub use crate::hcm::topology::*;` etc., i.e. `transportations_library::twolanehighways::Segment` and `transportations_library::hcm::twolanehighways::Segment` both resolve to the same type post-restructure, identically to how the pre-restructure flat-module crate exposed them. (These two `lib.rs` lines target the inner file module directly — `basicfreeways::basicfreeways::*` / `twolanehighways::twolanehighways::*` — rather than the folder-level glob, precisely to avoid re-importing the folder's own `CHAPTER`/`TITLE` consts and self-named submodule a second time at crate root, which is what caused the ambiguous-glob-reexport warnings this rename had to resolve.) `src/copython/twolanehighways.rs` itself imports via the old flat alias (`use crate::hcm::twolanehighways::{Segment as LibSegment, ...}`), showing the shim is exercised by in-tree code, not just a courtesy for external callers.

This matters because any downstream validator/consumer code written against the pre-restructure flat paths (`transportations_library::twolanehighways::*`, `transportations_library::constraints::*`, etc.) would fail to compile if these shims were dropped, even though no chapter's actual logic moved or changed during the restructure. That this is a real, exercised path rather than a hypothetical one is confirmed in-tree: `tests/twolanehighways_test.rs` itself imports via the flat alias (`use transportations_library::twolanehighways::{BicycleLOS, Segment, SubSegment, TwoLaneHighways};`), and `src/copython/twolanehighways.rs` imports via `use crate::hcm::twolanehighways::{Segment as LibSegment, ...}` rather than the `hcm::twolanehighways::twolanehighways` path directly. Removing a shim is therefore a breaking change to any code — in-repo test files included — written against the pre-restructure paths, independent of whether the underlying HCM methodology itself changed. The same now also holds for the chapter-number aliases: any code written against `hcm::chapter15::twolanehighways` (in-tree or external) depends on `pub use twolanehighways as chapter15;` in `src/hcm/mod.rs`.

## Validation

No `docs/hcm/VERIFICATION.md` exists on this branch (`git ls-tree -r feat/hcm-restructure` has no such path), so there is no existing deviation ledger to cross-reference from this document; deviations noted here and in `chapter15.md` are sourced directly from code comments and doc-comment/implementation mismatches found while reading this branch's tree.

## Deferred

- Unifying `support::constraints` and `common`'s semantic-firewall validators into one source of truth.
- Deciding the fate of `src/hcm/utils/core/` (`HcmCore`): salvage into Chapter 10 freeway-facilities work or delete, per the comment in `support/mod.rs`.
- Deciding whether `case_study1.json` fixtures are meant to be exercised by `cargo test` at all, since the current file filter explicitly excludes them.
