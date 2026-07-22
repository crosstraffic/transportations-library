//! Smoke test for the three module-path layers left after the
//! `chapterNN` -> topic-name folder rename (`feat/hcm-topic-folder-names`):
//!
//! 1. the chapter-number alias added by that rename
//!    (`hcm::chapter15::twolanehighways`, via `pub use twolanehighways as
//!    chapter15;` in `src/hcm/mod.rs`);
//! 2. the real topic path the rename introduced
//!    (`hcm::twolanehighways::twolanehighways`, the folder's self-named
//!    inner file);
//! 3. the older, pre-restructure flat shim that predates the chapter split
//!    entirely (`transportations_library::twolanehighways`, via the
//!    `src/lib.rs` re-export).
//!
//! All three must keep compiling and constructing a working
//! `TwoLaneHighways` so downstream consumers (in-tree tests, the Python
//! bindings, and external validators) written against any of these paths
//! are not broken by the rename.
//!
//! The `as _` imports below exist only to prove the path resolves at
//! compile time (the constructing tests below use the fully-qualified
//! paths directly); they are intentionally unused under that name.
#![allow(unused_imports)]

use transportations_library::hcm::chapter15::twolanehighways::TwoLaneHighways;
use transportations_library::hcm::twolanehighways::twolanehighways::TwoLaneHighways as _;
use transportations_library::twolanehighways::TwoLaneHighways as _;

#[test]
fn chapter_number_alias_path_constructs() {
    let highway = transportations_library::hcm::chapter15::twolanehighways::TwoLaneHighways::new(
        vec![],
        None,
        None,
        None,
        None,
        None,
    );
    assert_eq!(highway.get_segments().len(), 0);
}

#[test]
fn topic_path_constructs() {
    let highway =
        transportations_library::hcm::twolanehighways::twolanehighways::TwoLaneHighways::new(
            vec![],
            None,
            None,
            None,
            None,
            None,
        );
    assert_eq!(highway.get_segments().len(), 0);
}

#[test]
fn pre_restructure_flat_shim_constructs() {
    let highway =
        transportations_library::twolanehighways::TwoLaneHighways::new(vec![], None, None, None, None, None);
    assert_eq!(highway.get_segments().len(), 0);
}

#[test]
fn all_three_paths_are_the_same_type() {
    fn assert_same<T>(_: T) {}
    let a = transportations_library::hcm::chapter15::twolanehighways::TwoLaneHighways::new(
        vec![],
        None,
        None,
        None,
        None,
        None,
    );
    let b: TwoLaneHighways = a;
    assert_same::<transportations_library::hcm::twolanehighways::twolanehighways::TwoLaneHighways>(b);
}
