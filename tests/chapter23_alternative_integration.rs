//! Integration tests for the HCM Chapter 23, Part C (Alternative
//! Intersection Evaluation) methodology: full O-D journey / experienced
//! travel time pipeline runs against the published answers of HCM 7th
//! Edition, Chapter 34 (Interchange Ramp Terminals: Supplemental),
//! Example Problems 12–17.
//!
//! Fixtures (tests/ExampleCases/hcm/AlternativeIntersections/):
//! * `case1.json` — Example Problem 13: three-legged RCUT with STOP signs.
//!   Every junction control delay is COMPUTED here with the Chapter 20
//!   gap-acceptance procedure (Exhibit 34-128 reproduced exactly).
//! * `case2.json` — Example Problem 12: four-legged RCUT with merges
//!   (EDTT-driven; major-street left delays from Chapter 20 supplied).
//! * `case3.json` — Example Problem 15: four-legged MUT with STOP signs at
//!   the U-turn crossovers (Part C ETT assembly over Chapter 19 / 20
//!   junction delays).
//! * `case4.json` — Example Problem 16: partial DLT (offset computation
//!   Equations 23-63…23-68 and weighted-average control delay
//!   Equation 23-69).
//! * `case5.json` — Example Problem 17: full DLT of the same intersection,
//!   carried out as two partial DLT analyses (offsets in both directions
//!   plus the five-junction Equation 23-69 aggregation of Exhibit 34-150).
//!
//! Documented tolerances: O-D experienced travel time ±1.0 s/veh of the
//! published Exhibit 34-129 / 34-133 / 34-138 / 34-145 values; O-D LOS
//! exact (Exhibit 23-13). The one sub-0.1 s delta (Example 13 EB L, computed
//! 55.1 vs. published 55.2 from intermediate rounding) is within tolerance
//! and does not change LOS.

use transportations_library::hcm::ramp_terminals::alternative_intersections::{
    dlt_offset, AlternativeIntersection, DisplacedLeftTurn,
};
use transportations_library::hcm::common::LevelOfService;

fn fixture_path(name: &str) -> String {
    format!(
        "{}/tests/ExampleCases/hcm/AlternativeIntersections/{}",
        env!("CARGO_MANIFEST_DIR"),
        name
    )
}

fn load_rcut_mut(name: &str) -> AlternativeIntersection {
    let path = fixture_path(name);
    let json = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {path}: {e}"));
    AlternativeIntersection::from_json(&json).unwrap_or_else(|e| panic!("parse {path}: {e}"))
}

/// Assert every movement's ETT (±1 s/veh) and LOS (exact) against a table of
/// `(label, ett, los)` expectations.
fn assert_movements(ix: &AlternativeIntersection, expected: &[(&str, f64, LevelOfService)]) {
    let results = ix.evaluate();
    for (label, ett, los) in expected {
        let r = results
            .iter()
            .find(|r| r.label == *label)
            .unwrap_or_else(|| panic!("movement {label} not found"));
        assert!(
            (r.ett_s - ett).abs() <= 1.0,
            "{label} ETT: got {:.2}, expected {ett} (±1.0)",
            r.ett_s
        );
        assert_eq!(r.los, *los, "{label} LOS: got {:?}, expected {los:?}", r.los);
    }
}

#[test]
fn example_13_three_legged_rcut_stop_signs() {
    use LevelOfService::*;
    let ix = load_rcut_mut("case1.json");
    assert_movements(
        &ix,
        &[
            ("EB L", 55.2, E),
            ("EB R", 22.9, C),
            ("NB L", 13.0, B),
            ("NB T", 0.0, A),
            ("SB T", 0.0, A),
            ("SB R", 0.0, A),
        ],
    );
    // The EB L journey delays are fully computed from Chapter 20 primitives
    // (Exhibit 34-128): main-junction right 22.9 s + U-turn crossover 16.3 s.
    let ebl = ix
        .evaluate()
        .into_iter()
        .find(|r| r.label == "EB L")
        .unwrap();
    assert!((ebl.junction_delays_s[0] - 22.9).abs() < 0.1);
    assert!((ebl.junction_delays_s[1] - 16.3).abs() < 0.1);
    assert!((ebl.edtt_s - 15.9).abs() < 0.1);
}

#[test]
fn example_12_four_legged_rcut_merges() {
    use LevelOfService::*;
    let ix = load_rcut_mut("case2.json");
    assert_movements(
        &ix,
        &[
            ("EB L", 11.2, B),
            ("WB L", 15.0, B),
            ("EB T", 0.0, A),
            ("WB R", 0.0, A),
            ("NB L", 55.4, E),
            ("NB T", 60.4, E),
            ("NB R", 0.0, A),
            ("SB L", 55.4, E),
            ("SB T", 60.4, E),
        ],
    );
}

#[test]
fn example_15_four_legged_mut_stop_signs() {
    use LevelOfService::*;
    let ix = load_rcut_mut("case3.json");
    assert_movements(
        &ix,
        &[
            ("NB L", 78.0, E),
            ("SB L", 56.1, E),
            ("NB T", 9.3, A),
            ("SB T", 12.3, B),
            ("NB R", 9.4, A),
            ("SB R", 13.7, B),
            ("EB L", 67.4, E),
            ("WB L", 87.5, F),
            ("EB T", 25.1, C),
            ("WB T", 22.2, C),
            ("EB R", 23.7, C),
            ("WB R", 20.2, C),
        ],
    );
}

#[test]
fn example_16_partial_dlt_offset_and_weighted_delay() {
    // Step 5 offset computation (Equations 23-63…23-68).
    let off = dlt_offset(350.0, 35.0, 0.0, 52.0, 0.0, 0.0, 65.0);
    assert!((off.tt_dlt_s - 6.8).abs() < 0.05, "TT_DLT {}", off.tt_dlt_s);
    assert_eq!(off.st_th_s, 52.0);
    // Published O_SUPP = 45 s (rounds TT_DLT to 7); computed 45.2 s.
    assert!((off.offset_supp_s - 45.2).abs() < 0.1, "O_SUPP {}", off.offset_supp_s);

    // Step 9/10 weighted-average control delay (Equation 23-69).
    let path = fixture_path("case4.json");
    let json = std::fs::read_to_string(&path).unwrap();
    let root: serde_json::Value = serde_json::from_str(&json).unwrap();
    let dlt: DisplacedLeftTurn =
        serde_json::from_value(root["dlt"].clone()).expect("parse dlt block");
    let ett = dlt.intersection_ett();
    assert!((ett - 28.5).abs() <= 0.1, "ETT_DLT: got {ett:.2}, expected 28.5");
    assert_eq!(dlt.los(), LevelOfService::C, "DLT LOS");
}

/// Chapter 34, Example Problem 17 (full DLT): the Step 5 offsets in both
/// directions (Exhibits 34-148 and 34-149) and the five-junction Step 9/10
/// aggregation of Exhibit 34-150.
///
/// The full DLT is analyzed as two partial DLT analyses on a shared 45-s
/// cycle, so the same `dlt_offset` and `DisplacedLeftTurn` surfaces cover it;
/// only the junction count and the LAG_TH values change.
#[test]
fn example_17_full_dlt_offsets_and_weighted_delay() {
    let path = fixture_path("case5.json");
    let json = std::fs::read_to_string(&path).unwrap();
    let root: serde_json::Value = serde_json::from_str(&json).unwrap();

    // Step 5 offsets. LAG_TH is the time from the reference point (end of the
    // major-street through phase) to the start of the major-street through
    // phase at the main intersection: 18 + 4 + 1 = 23 s east–west and
    // 17 + 4 + 1 = 22 s north–south, both read off the Exhibit 34-148 /
    // 34-149 timing plans. The example rounds TT_DLT to 7 s and reports 16
    // and 15 s; the unrounded 6.8 s gives 16.2 and 15.2 s.
    for (key, o_supp_expected, o_supp_published) in
        [("dlt_offset_ew", 16.2, 16.0), ("dlt_offset_ns", 15.2, 15.0)]
    {
        let b = &root[key];
        let off = dlt_offset(
            b["td_dlt_ft"].as_f64().unwrap(),
            b["sf_dlt_mph"].as_f64().unwrap(),
            b["lag_dlt_s"].as_f64().unwrap(),
            b["lag_th_s"].as_f64().unwrap(),
            b["offset_supp_s"].as_f64().unwrap(),
            b["offset_main_s"].as_f64().unwrap(),
            b["cycle_s"].as_f64().unwrap(),
        );
        assert!((off.tt_dlt_s - 6.8).abs() < 0.05, "{key} TT_DLT {}", off.tt_dlt_s);
        assert_eq!(off.st_dlt_s, 0.0, "{key} ST_DLT");
        assert_eq!(off.st_th_s, b["lag_th_s"].as_f64().unwrap(), "{key} ST_TH");
        assert!(
            (off.offset_supp_s - o_supp_expected).abs() < 0.1,
            "{key} O_SUPP: got {}, expected {o_supp_expected} (published {o_supp_published})",
            off.offset_supp_s
        );
        // Both offsets land inside [0, C) without needing the Equation
        // 23-67 / 23-68 wrap, which is the check Steps 8 and 9 of the
        // example perform explicitly against the 45-s cycle.
        assert!(off.offset_supp_s >= 0.0 && off.offset_supp_s < 45.0, "{key} offset range");
    }

    // Step 9/10 weighted-average control delay over the five junctions.
    let dlt: DisplacedLeftTurn =
        serde_json::from_value(root["dlt"].clone()).expect("parse dlt block");
    assert_eq!(dlt.cells.len(), 20, "Exhibit 34-150 cell count");

    // Every cell reproduces its printed Products entry. The exhibit prints
    // the products to whole veh-s/h, so ±0.5 is exact agreement.
    for (cell, product_published) in dlt.cells.iter().zip([
        12_024.0, 515.0, 6_337.0, 14_061.0, 6_161.0, 8_505.0, 25_006.0, 4_352.0, 334.0, 4_231.0,
        11_233.0, 5_751.0, 518.0, 22_976.0, 5_610.0, 8_700.0, 4_441.0, 16_919.0, 359.0, 4_342.0,
    ]) {
        let product = cell.flow_veh_h * cell.control_delay_s;
        assert!(
            (product - product_published).abs() <= 0.5,
            "product {product} vs published {product_published} (v = {}, d = {})",
            cell.flow_veh_h,
            cell.control_delay_s
        );
    }

    // The published Products total is 162,373; the unrounded cells sum to
    // 162,375, so the aggregate lands 0.0004 s/veh above the published 29.0.
    let ett = dlt.intersection_ett();
    assert!((ett - 29.0).abs() <= 0.1, "ETT_DLT: got {ett:.2}, expected 29.0");
    assert_eq!(dlt.los(), LevelOfService::C, "DLT LOS");

    // The full DLT does not beat the partial DLT of Example Problem 16 at
    // this 350-ft spacing, which is the finding the example closes on
    // (29.0 against 28.5 s/veh, both LOS C, against 64.1 s/veh for the
    // conventional intersection).
    let partial_path = fixture_path("case4.json");
    let partial_json = std::fs::read_to_string(&partial_path).unwrap();
    let partial_root: serde_json::Value = serde_json::from_str(&partial_json).unwrap();
    let partial: DisplacedLeftTurn =
        serde_json::from_value(partial_root["dlt"].clone()).unwrap();
    assert!(
        ett > partial.intersection_ett(),
        "full DLT {ett:.2} should exceed partial DLT {:.2}",
        partial.intersection_ett()
    );
    assert_eq!(partial.total_od_demand_veh_h, dlt.total_od_demand_veh_h, "shared O-D total");
}
