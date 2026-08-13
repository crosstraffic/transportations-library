//! Integration tests for HCM Chapter 12 (Basic Freeway and Highway Segments),
//! including the Chapter 26 supplemental example problems that extend it.
//!
//! Chapter 26 coverage: Example Problems 1-3 (operational and design, case1-3 fixtures),
//! 4 (five-lane highway with a TWLTL), 6 (severe weather), and 7 (basic managed lane) are
//! pinned at published values. Example Problem 5 is covered only in its PCE-comparison half;
//! its mixed-flow half is blocked for the reason below.
//!
//! NOT COVERED: HCM Chapter 25, Example Problem 11 (Estimating Freeway
//! Composite Grade Operations with the Mixed-Flow Model), and the mixed-flow half of
//! Chapter 26, Example Problem 5 (Steps 2 through 8, Equations 26-1 through 26-16;
//! published targets are mixed-flow capacity 1,725 veh/h/ln and mixed-flow density
//! 32.6 veh/mi/ln). The mixed-flow model
//! is not implemented anywhere in this library, so there is nothing to assert
//! against. Every module that could reach it instead refuses the input or
//! substitutes an approximation and says so:
//!   - src/hcm/basicfreeways/basicfreeways.rs:780 returns 2.5 as a non-HCM PCE
//!     stand-in for mountainous terrain, and :786 rejects any other terrain
//!     name, both pointing at the Chapter 25/26 mixed-flow model;
//!   - src/hcm/common/pce_table.rs:327 rejects grades steeper than the
//!     Exhibit 12-26/27/28 maximum for the same reason;
//!   - src/hcm/merge_diverge/merge_diverge.rs:458,
//!     src/hcm/weaving/weaving.rs:370, and
//!     src/hcm/freeway_facilities/freeway_facilities.rs:108 carry the same
//!     mountainous-terrain approximation.
//!
//! Closing the gap needs Equations 25-53 through 25-70 (mixed-flow CAF, truck
//! and auto spot and space-based travel time rates, the traffic interaction
//! term, and the mixed-flow aggregation) plus the Exhibit 25-20/25-21 truck
//! spot-rate curves and the Exhibit 25-A7/25-A16 space-based travel time
//! curves. The last two are the harder half: the published solution reads
//! truck kinematic rates off nomographs by eye ("its spot rate can be read at
//! 6,780 ft and is approximately 75 s/mi"), so a faithful reproduction needs
//! those curve families digitized, not just the equations transcribed.
//!
//! Published target values for Example Problem 11, so a future implementation
//! has something to hit. Facts: three basic segments (1.5 mi at 3%, 2.0 mi at
//! 2%, 1.0 mi at 5%), six-lane freeway, 5% SUTs and 10% TTs, FFS 65 mi/h,
//! 1,500 veh/h/ln at PHF 1.0.
//!   - Governing mixed-flow capacity across the three segments: 1,746 veh/h/ln.
//!   - Mixed-flow space mean speed by segment (Equation 25-68): 57.7, 58.7,
//!     and 47.9 mi/h.
//!   - Mixed-flow travel time by segment (Equation 25-69): 93.6, 122.7, and
//!     75.2 s.
//!   - Overall mixed-flow speed (Equation 25-70): 55.6 mi/h.
//!   - Spot speeds at the end of each segment (Exhibit 25-109), autos / SUTs /
//!     TTs: 59.5 / 56.1 / 56.4, then 60.9 / 60.9 / 54.0, then 45.2 / 42.2 /
//!     31.8 mi/h. The facility entry values are 59.5 for all three modes.
//!   - Space mean speeds by segment (Exhibit 25-110): 58.7 / 57.0 / 50.6, then
//!     59.5 / 60.9 / 51.8, then 49.9 / 46.6 / 36.3 mi/h.
//!   - Overall space mean speeds (Exhibit 25-111): autos 56.8, SUTs 55.8, TTs
//!     47.0 mi/h.
//!
//! One inconsistency in the source, so a future implementation does not chase
//! it: the Step 7 prose states the overall mixed-flow travel time "equals
//! 294 s", but the three published segment travel times sum to 291.5 s, and it
//! is 291.5 that the book substitutes into Equation 25-70 to get 55.6 mi/h
//! (3,600 x 4.5 / 291.5 = 55.6; 3,600 x 4.5 / 294 would give 55.1). Target
//! 291.5 s, not 294 s.

use transportations_library::math;
use transportations_library::basicfreeways::BasicFreeways;
use transportations_library::common::LevelOfService;

use std::fs::{self, File};
use std::io::BufReader;
use std::path::PathBuf;


fn read_test_files() -> Vec<String> {
    let mut examples_root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    examples_root_dir.push("./tests/ExampleCases/hcm/BasicFreeways/");
    let paths = fs::read_dir(examples_root_dir).expect("Unbale to read directory");
    let mut setting_files: Vec<String> = Vec::new();

    for path in paths {
        setting_files.push(path.unwrap().path().display().to_string());
    }

    setting_files.sort();

    setting_files
}

fn settings(setting_file_loc: String) -> BasicFreeways {
    let f = File::open(setting_file_loc).expect("Unable to open file");
    let reader = BufReader::new(f);

    let basic_freeways: BasicFreeways =
        serde_json::from_reader(reader).expect("Failed to parse JSON");

    basic_freeways
}

fn initialize_test_case(bcf: BasicFreeways) -> BasicFreeways {

    let basicfreeways = BasicFreeways {
        length: bcf.length,
        lane_count: bcf.lane_count as u32,
        lw: bcf.lw,
        grade: bcf.grade,
        density: bcf.density,
        speed_limit: bcf.speed_limit,
        demand_flow_i: bcf.demand_flow_i,
        v_p: bcf.v_p,
        capacity: bcf.capacity,
        capacity_adj: bcf.capacity_adj,
        bffs: bcf.bffs,
        ffs: bcf.ffs,
        ffs_adj: bcf.ffs_adj,
        phf: bcf.phf,
        phv: bcf.phv,
        lc_r: bcf.lc_r,
        lc_l: bcf.lc_l,
        p_t: bcf.p_t,
        e_t: bcf.e_t,
        trd: bcf.trd,
        apd: bcf.apd,
        los: bcf.los.clone(),
        terrain_type: bcf.terrain_type,
        sut_percentage: bcf.sut_percentage,
        city_type: bcf.city_type,
        median_type: bcf.median_type,
        highway_type: bcf.highway_type,
        saf: bcf.saf,
        caf: bcf.caf,
        breakpoint: bcf.breakpoint,
        speed: bcf.speed,
        vc_ratio: bcf.vc_ratio,
        aadt: bcf.aadt,
        k_factor: bcf.k_factor,
        d_factor: bcf.d_factor,
    };

    basicfreeways
}

#[test]
fn determine_free_flow_speed_test() {
    // HCM Ch. 26 Example Problems 1-3 (published FFS values, mi/h)
    let ans = vec![
        60.8, 67.3, 70.0
    ];

    let setting_files = read_test_files();

    for (index, s_file) in setting_files.iter().enumerate() {
        let bcf: BasicFreeways = settings(s_file.clone());
        let mut basicfreeways = initialize_test_case(bcf);

        let _ffs = basicfreeways.determine_free_flow_speed();
        assert_eq!(
            ans[index], math::round_up_to_n_decimal(_ffs, 1),
            "Test case {index} failed",
        );
    }
}

/// Design analysis (Eqs 12-21 through 12-23) for all three fixtures.
///
/// Only case 2 is a published design problem (HCM Ch. 26 EP2, 3 lanes at LOS D). Cases 1 and 3
/// are operational problems, so a target LOS is imposed here and the expected lane count is
/// derived from the Exhibit 12-37 row for each case's own FFS — these are exhibit-derived, not
/// published, values. Every case runs the calculation; the previous version of this test skipped
/// cases 1 and 3 because their fixtures carry v_p = 0 and asserted 0 == 0.
#[test]
fn estimate_number_of_lanes() {
    // (target LOS, expected lanes): case 1 FFS 60.8 -> Exhibit 12-37 row 60, LOS D MSF 2,000;
    // case 2 FFS 67.3 -> row 65, LOS D MSF 2,060; case 3 FFS 70.0 -> row 70, LOS D MSF 2,110.
    let cases = vec![
        (LevelOfService::D, 2u32),
        (LevelOfService::D, 3u32),
        (LevelOfService::D, 3u32),
    ];
    let setting_files = read_test_files();

    for (index, s_file) in setting_files.iter().enumerate() {
        let bcf: BasicFreeways = settings(s_file.clone());
        let mut basicfreeways = initialize_test_case(bcf);

        let (target_los, expected) = cases[index];
        basicfreeways.determine_free_flow_speed();
        basicfreeways.los = Some(target_los);

        let (num_lanes, _unrounded) = basicfreeways
            .estimate_number_of_lanes()
            .unwrap_or_else(|e| panic!("Test case {index} errored: {e}"));
        assert_eq!(expected, num_lanes, "Test case {index} failed");
    }
}

/// Exhibit 12-37/12-38 are read at the nearest 5 mi/h with no interpolation permitted, and an
/// (FFS, LOS) pair outside the exhibit is an error rather than a silent 2,000 pc/h/ln default.
#[test]
fn max_service_flow_rate_domain() {
    let mut bf = BasicFreeways::new();
    bf.los = Some(LevelOfService::C);

    // 61 and 62.4 both round down to the 60 row; 63 rounds up to 65.
    for (ffs_adj, expected) in [(60.0, 1560.0), (61.0, 1560.0), (62.4, 1560.0), (63.0, 1660.0)] {
        bf.ffs_adj = ffs_adj;
        assert_eq!(
            expected,
            bf.determine_basic_max_service_flow_rate().unwrap(),
            "FFS {ffs_adj} read the wrong Exhibit 12-37 row",
        );
    }

    // Below the 55 mi/h floor of Exhibit 12-37, at LOS F, and with no target LOS at all.
    bf.ffs_adj = 45.0;
    assert!(bf.determine_basic_max_service_flow_rate().is_err());
    bf.ffs_adj = 65.0;
    bf.los = Some(LevelOfService::F);
    assert!(bf.determine_basic_max_service_flow_rate().is_err());
    bf.los = None;
    assert!(bf.determine_basic_max_service_flow_rate().is_err());
}

#[test]
fn estimate_capacity_test() {
    // HCM Ch. 26 Example Problems 1-3 (Equation 12-6 capacities, pc/h/ln)
    let ans = vec![
        2308.0, 2373.0, 2400.0
    ];
    let setting_files = read_test_files();

    for (index, s_file) in setting_files.iter().enumerate() {
        let bcf: BasicFreeways = settings(s_file.clone());
        let mut basicfreeways = initialize_test_case(bcf);

        let _ffs = basicfreeways.determine_free_flow_speed();
        let _capacity = basicfreeways.estimate_capacity();
        assert_eq!(
            ans[index], math::round_up_to_n_decimal(basicfreeways.capacity, 0),
            "Test case {index} failed",
        );
    }
}

#[test]
fn estimate_demand_volume_test() {
    // HCM Ch. 26 Example Problems 1-3 (Equation 12-9 demand flow, pc/h/ln)
    let ans = vec![
        1142, 1694, 1875
    ];
    let setting_files = read_test_files();

    for (index, s_file) in setting_files.iter().enumerate() {
        let bcf: BasicFreeways = settings(s_file.clone());
        let mut basicfreeways = initialize_test_case(bcf);

        let _ffs = basicfreeways.determine_free_flow_speed();
        let _capacity = basicfreeways.estimate_capacity();

        // Estimate number of lanes
        if basicfreeways.v_p != 0.0 {
            (_, _) = basicfreeways.estimate_number_of_lanes().unwrap();
        }

        let _demand_volume = basicfreeways.estimate_demand_volume().unwrap();
        assert_eq!(
            ans[index], math::round_up_to_n_decimal(_demand_volume, 0) as i32,
            "Test case {index} failed",
        );
    }
}

// Need BreakPoint and estimated speed

#[test]
fn estimate_density_test() {
    // HCM Ch. 26 Example Problems 1-3 (Equation 12-11 densities, pc/mi/ln)
    let ans = vec![
        18.8, 25.9, 29.0
    ];
    let setting_files = read_test_files();

    for (index, s_file) in setting_files.iter().enumerate() {
        let bcf: BasicFreeways = settings(s_file.clone());
        let mut basicfreeways = initialize_test_case(bcf);

        let _ffs = basicfreeways.determine_free_flow_speed();
        let _capacity = basicfreeways.estimate_capacity();

        // Estimate number of lanes
        if basicfreeways.v_p != 0.0 {
            (_, _) = basicfreeways.estimate_number_of_lanes().unwrap();
        }

        let _demand_volume = basicfreeways.estimate_demand_volume().unwrap();
        let _density = basicfreeways.estimate_density();
        assert_eq!(
            ans[index], math::round_up_to_n_decimal(basicfreeways.density, 1),
            "Test case {index} failed",
        );
    }
}

#[test]
fn determine_segment_los() {
    // HCM Ch. 26 Example Problems 1-3 (Exhibit 12-15 LOS letters)
    let ans = vec![
        'C', 'C', 'D'
    ];
    let setting_files = read_test_files();

    for (index, s_file) in setting_files.iter().enumerate() {
        let bcf: BasicFreeways = settings(s_file.clone());
        let mut basicfreeways = initialize_test_case(bcf);

        let _ffs = basicfreeways.determine_free_flow_speed();
        let _capacity = basicfreeways.estimate_capacity();

        // Estimate number of lanes
        if basicfreeways.v_p != 0.0 {
            (_, _) = basicfreeways.estimate_number_of_lanes().unwrap();
        }

        let _demand_volume = basicfreeways.estimate_demand_volume().unwrap();
        let _density = basicfreeways.estimate_density();
        let _los = basicfreeways.determine_segment_los();
        let _los_char: char = _los.into();
        assert_eq!(
            ans[index], _los_char,
            "Test case {index} failed",
        );
    }

}

#[test]
fn estimate_speed_test() {
    // HCM Ch. 26 Example Problems 1-3 (Equation 12-1 speeds, mi/h):
    // EP1: below breakpoint -> S = FFS = 60.8
    // EP2 (3-lane operational continuation): 65.4
    // EP3 (present demand): 64.7
    let ans = vec![60.8, 65.4, 64.7];
    let tol = 0.1;
    let setting_files = read_test_files();

    for (index, s_file) in setting_files.iter().enumerate() {
        let bcf: BasicFreeways = settings(s_file.clone());
        let mut basicfreeways = initialize_test_case(bcf);

        let _ffs = basicfreeways.determine_free_flow_speed();
        let _capacity = basicfreeways.estimate_capacity();
        if basicfreeways.v_p != 0.0 {
            (_, _) = basicfreeways.estimate_number_of_lanes().unwrap();
        }
        let _demand_volume = basicfreeways.estimate_demand_volume().unwrap();
        let speed = basicfreeways.calculate_speed();
        assert!(
            (speed - ans[index]).abs() < tol,
            "Test case {index} failed: speed {speed} != {}",
            ans[index]
        );
    }
}

/// Step 2 default base FFS: 75.4 mi/h for freeways (the book's stated default), and for multilane
/// highways the speed-limit rule, since Chapter 12 gives them no single default.
#[test]
fn default_base_ffs_reaches_the_constructors() {
    use transportations_library::basicfreeways::{bffs_from_speed_limit, DEFAULT_BFFS_FREEWAY};

    assert_eq!(DEFAULT_BFFS_FREEWAY, BasicFreeways::with_urban_freeway_defaults().bffs);
    assert_eq!(DEFAULT_BFFS_FREEWAY, BasicFreeways::with_rural_freeway_defaults().bffs);

    // "speed limit plus 5 mi/h for speed limits 50 mi/h and higher and ... plus 7 mi/h for speed
    // limits less than 50 mi/h"
    assert_eq!(70.0, bffs_from_speed_limit(65.0));
    assert_eq!(55.0, bffs_from_speed_limit(50.0));
    assert_eq!(52.0, bffs_from_speed_limit(45.0));

    let multilane = BasicFreeways::with_urban_multilane_defaults();
    assert_eq!(bffs_from_speed_limit(multilane.speed_limit as f64), multilane.bffs);
}

/// Exhibit 12-38 rows, and the fact that multilane design analysis has no row above FFS 60.
#[test]
fn multilane_max_service_flow_rate() {
    let mut bf = BasicFreeways::new();
    bf.highway_type = "multilane".to_string();

    for (ffs_adj, los, expected) in [
        (45.0, LevelOfService::A, 490.0),
        (50.0, LevelOfService::C, 1300.0),
        (55.0, LevelOfService::D, 1790.0),
        (60.0, LevelOfService::E, 2200.0),
        (58.0, LevelOfService::E, 2200.0), // rounds to the 60 row, not up past the exhibit
    ] {
        bf.ffs_adj = ffs_adj;
        bf.los = Some(los);
        assert_eq!(expected, bf.determine_multilane_max_service_flow_rate().unwrap());
    }

    // Exhibit 12-38 stops at 60 mi/h even though the methodology covers multilane FFS up to 70.
    bf.ffs_adj = 65.0;
    bf.los = Some(LevelOfService::D);
    assert!(bf.determine_multilane_max_service_flow_rate().is_err());
}

/// Exhibits 12-37/12-38 round FFS to the nearest 5 mi/h, which is not rounding up.
#[test]
fn round_to_nearest_5_is_not_round_up() {
    use transportations_library::math;

    assert_eq!(60, math::round_to_nearest_5(60.0));
    assert_eq!(60, math::round_to_nearest_5(61.0));
    assert_eq!(60, math::round_to_nearest_5(62.4));
    assert_eq!(65, math::round_to_nearest_5(62.5));
    assert_eq!(65, math::round_to_nearest_5(67.3));
    // The old helper rounded every non-multiple up, which read the wrong exhibit row.
    assert_eq!(65, math::round_up_to_nearest_5(61.0));
}

/// Non-finite PCE inputs error rather than clamping into a table row.
#[test]
fn pce_lookup_rejects_non_finite_inputs() {
    use transportations_library::common::pce_table::PceTable;

    let table = PceTable::for_sut_percentage(50).unwrap();
    assert!(table.lookup(f64::NAN, 0.5, 0.06).is_err());
    assert!(table.lookup(2.5, f64::NAN, 0.06).is_err());
    assert!(table.lookup(2.5, 0.5, f64::NAN).is_err());
    assert!(table.lookup(2.5, f64::INFINITY, 0.06).is_err());
    // A well-formed lookup still works: Exhibit 12-27, 2.5% grade, 0.625 mi, 6% trucks.
    assert_eq!(3.03, table.lookup(2.5, 0.625, 0.06).unwrap());
}

// ═══════════════════════════════════════════════════════════════════════════════
// Chapter 26, Example Problem 7: basic managed lane segment
// ═══════════════════════════════════════════════════════════════════════════════

/// HCM 7th Edition Chapter 26, Example Problem 7: six-lane freeway, two
/// general purpose lanes plus one continuous-access managed lane per
/// direction, FFS 60 mi/h both, PHF 0.92, 7.5% trucks on level terrain
/// (E_T = 2.0, f_HV = 0.93). The example's Step 4 prose says "5%" but its
/// own Equation 12-10 substitution uses 0.075; the printed flow rates
/// (1,169 / 2,221 / 1,519 pc/h/ln) confirm 7.5%.
///
/// Published chain: GP capacity 2,300 pc/h/ln, GP breakpoint 1,600; managed
/// lane capacity 1,650 (C_75 = 1,800, lambda_c = 10), breakpoint 500
/// (BP_75 = 500, lambda_BP = 0), S_1 = 60, S_2 = 3.7, S_3 = 14.4 mi/h.
#[test]
fn test_ch26_ep7_managed_lane_case1_low_gp_density() {
    use transportations_library::basicfreeways::managed_lanes::{ManagedLaneSegment, ManagedLaneType};

    // General purpose side, Case 1: v_p = 2,000/(0.92 x 2 x 0.93) = 1,169
    // pc/h/ln, below the 1,600 breakpoint, so S = FFS = 60 mi/h and
    // D = 1,169/60 = 19.5 pc/mi/ln (LOS C). Below the 35 pc/mi/ln friction
    // threshold, so I_c = 0 for the managed lane.
    let f_hv: f64 = 1.0 / (1.0 + 0.075 * (2.0 - 1.0));
    let vp_gp = 2000.0 / (0.92 * 2.0 * f_hv);
    assert!((vp_gp - 1169.0).abs() < 1.0, "GP Case 1 flow {vp_gp}");
    let d_gp = vp_gp / 60.0;
    assert!((d_gp - 19.5).abs() < 0.05, "GP Case 1 density {d_gp}");

    // Managed lane: v_p = 1,300/(0.92 x 1 x 0.93) = 1,519 pc/h/ln.
    let vp_ml = 1300.0 / (0.92 * 1.0 * f_hv);
    assert!((vp_ml - 1519.0).abs() < 1.0, "ML flow {vp_ml}");

    let mut ml = ManagedLaneSegment::new(ManagedLaneType::ContinuousAccess, 60.0);
    ml.set_demand(vp_ml);
    ml.set_gp_density(d_gp);
    let los = ml.run_analysis();

    assert!((ml.capacity_adj - 1650.0).abs() < 1.0, "ML capacity {}", ml.capacity_adj);
    assert!((ml.breakpoint - 500.0).abs() < 1.0, "ML breakpoint {}", ml.breakpoint);
    // I_c = 0: S_ML = S_1 - S_2 = 60 - 3.7 = 56.3 mi/h, D = 27.0 pc/mi/ln,
    // published LOS D (just past the 26 pc/mi/ln LOS C boundary).
    assert!((ml.speed - 56.3).abs() < 0.1, "ML Case 1 speed {}", ml.speed);
    assert!((ml.density - 27.0).abs() < 0.1, "ML Case 1 density {}", ml.density);
    assert_eq!(los, LevelOfService::D, "ML Case 1 LOS");
}

/// Case 2 of the same example: GP demand rises to 3,800 veh/h, putting the
/// adjacent general purpose lanes past the 35 pc/mi/ln friction threshold,
/// which activates I_c and drops the managed lane a full LOS letter.
#[test]
fn test_ch26_ep7_managed_lane_case2_gp_friction() {
    use transportations_library::basicfreeways::managed_lanes::{ManagedLaneSegment, ManagedLaneType};

    // GP Case 2: v_p = 3,800/(0.92 x 2 x 0.93) = 2,221 pc/h/ln, above the
    // breakpoint; Equation 12-1 with a = 2 gives S = 53.0 mi/h and
    // D = 41.9 pc/mi/ln (LOS E).
    let f_hv: f64 = 1.0 / (1.0 + 0.075 * (2.0 - 1.0));
    let vp_gp = 3800.0 / (0.92 * 2.0 * f_hv);
    assert!((vp_gp - 2221.0).abs() < 1.0, "GP Case 2 flow {vp_gp}");
    let c_gp: f64 = 2300.0;
    let bp_gp: f64 = 1600.0;
    let s_gp = 60.0 - (60.0 - c_gp / 45.0) * (vp_gp - bp_gp).powi(2) / (c_gp - bp_gp).powi(2);
    assert!((s_gp - 53.0).abs() < 0.1, "GP Case 2 speed {s_gp}");
    let d_gp = vp_gp / s_gp;
    assert!((d_gp - 41.9).abs() < 0.1, "GP Case 2 density {d_gp}");

    let vp_ml = 1300.0 / (0.92 * 1.0 * f_hv);
    let mut ml = ManagedLaneSegment::new(ManagedLaneType::ContinuousAccess, 60.0);
    ml.set_demand(vp_ml);
    ml.set_gp_density(d_gp);
    let los = ml.run_analysis();

    // I_c = 1: S_ML = 60 - 3.7 - 14.4 = 41.9 mi/h, D = 36.3 pc/mi/ln, LOS E.
    assert!((ml.speed - 41.9).abs() < 0.1, "ML Case 2 speed {}", ml.speed);
    assert!((ml.density - 36.3).abs() < 0.1, "ML Case 2 density {}", ml.density);
    assert_eq!(los, LevelOfService::E, "ML Case 2 LOS");
}

// ═══════════════════════════════════════════════════════════════════════════════
// Chapter 26, Example Problems 4-6
// ═══════════════════════════════════════════════════════════════════════════════
//
// These live in tests/ExampleCases/hcm/Chapter26/ rather than alongside case1-3 in
// BasicFreeways/ because `read_test_files` above walks that directory and the tests that
// consume it index a three-element expectation vector by sort position. Dropping a fourth
// file in there silently re-pairs every case with the wrong expected value.

/// Load a Chapter 26 fixture by name. Unlike `read_test_files`, this is positional-free.
fn load_ch26(name: &str) -> BasicFreeways {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/ExampleCases/hcm/Chapter26");
    path.push(name);
    let f = File::open(&path).unwrap_or_else(|_| panic!("Unable to open {path:?}"));
    serde_json::from_reader(BufReader::new(f)).expect("Failed to parse fixture JSON")
}

fn assert_approx(actual: f64, expected: f64, tol: f64, label: &str) {
    assert!(
        (actual - expected).abs() <= tol,
        "{label}: got {actual}, expected {expected} (+-{tol})"
    );
}

/// The BasicFreeways fixture directory is a positional contract: `read_test_files` sorts it
/// and the expectation vectors above are indexed by that order. A file added, removed, or
/// renamed there shifts every pairing, and for a rename or removal it shifts them silently
/// rather than panicking on a short vector. Pin the set so that failure is loud.
#[test]
fn basic_freeways_fixture_set_is_exactly_the_three_positional_cases() {
    let names: Vec<String> = read_test_files()
        .iter()
        .map(|p| {
            PathBuf::from(p)
                .file_name()
                .unwrap()
                .to_string_lossy()
                .into_owned()
        })
        .collect();
    assert_eq!(
        vec!["case1.json", "case2.json", "case3.json"],
        names,
        "tests/ExampleCases/hcm/BasicFreeways/ is indexed by sort position; add new fixtures \
         under tests/ExampleCases/hcm/Chapter26/ instead",
    );
}

/// HCM Chapter 26, Example Problem 4: LOS on a five-lane highway with a two-way left-turn
/// lane. A 6,600-ft (1.25 mi) segment of a four-lane multilane highway plus a TWLTL, on a
/// 3.5% grade, analyzed separately in each direction. No base FFS is given, so the example
/// takes BFFS = speed limit + 7 = 52 mi/h. Lane width (12 ft), total lateral clearance
/// (6 + 6 = 12 ft, the TWLTL counting as a 6-ft median clearance) and median type are all
/// base conditions, so only access-point density moves FFS.
///
/// Eastbound is the 3.5% downgrade with 10 access points/mi; westbound is the 3.5% upgrade
/// with none. Published: FFS 49.5 / 52.0 mi/h, c 1,990 / 2,040 pc/h/ln, E_T 2.24 / 3.97,
/// f_HV 0.93 / 0.85, v_p 896 / 980 pc/h/ln, both below the 1,400 pc/h/ln multilane
/// breakpoint so S = FFS, D 18.1 / 18.8 pc/mi/ln, LOS C both directions.
///
/// The example gives no area type. Both fixtures carry Urban, which is the constructor default
/// and no longer decides anything here: Exhibit 12-15 has a single density-to-LOS table for
/// basic freeway and multilane segments with no urban/rural split, and segment LOS now reads
/// it directly (see `segment_los_does_not_depend_on_area_type` below).
#[test]
fn ch26_ep4_five_lane_highway_with_twltl() {
    // (fixture, FFS, capacity, E_T, f_HV, v_p, density)
    let cases = [
        ("ep4_eastbound_downgrade.json", 49.5, 1990.0, 2.24, 0.93, 896.0, 18.1),
        ("ep4_westbound_upgrade.json", 52.0, 2040.0, 3.97, 0.85, 980.0, 18.8),
    ];

    for (name, ffs, capacity, e_t, f_hv, v_p, density) in cases {
        let mut seg = load_ch26(name);
        let los = seg.run_operational_analysis().expect(name);

        assert_approx(seg.ffs, ffs, 0.05, &format!("{name} FFS (mi/h)"));
        assert_approx(seg.capacity, capacity, 0.5, &format!("{name} capacity (pc/h/ln)"));
        assert_approx(seg.e_t.unwrap(), e_t, 1e-9, &format!("{name} E_T"));
        // The example rounds f_HV to two decimals before dividing; the engine rounds to
        // three, which is why v_p lands within about 1.5 pc/h/ln of the printed value
        // rather than on it (WB: 1,500/(0.9 x 2 x 0.849) = 981.5 against the book's
        // 1,500/(0.9 x 2 x 0.85) = 980.4).
        assert_approx(seg.phv, f_hv, 0.005, &format!("{name} f_HV"));
        assert_approx(seg.v_p, v_p, 2.0, &format!("{name} v_p (pc/h/ln)"));
        // Below the multilane breakpoint, so Equation 12-1 returns FFS unchanged.
        assert_approx(seg.breakpoint, 1400.0, 1e-9, &format!("{name} breakpoint"));
        assert_approx(seg.speed, ffs, 0.05, &format!("{name} S (mi/h)"));
        // Same rounding chain as v_p, so the density residual is about 0.08 pc/mi/ln.
        assert_approx(seg.density, density, 0.1, &format!("{name} D (pc/mi/ln)"));
        assert_eq!(LevelOfService::C, los, "{name} LOS");
    }
}

/// HCM Chapter 26, Example Problem 5, the "Comparison with the PCE-Based Approach" half.
///
/// The mixed-flow half of this example (Steps 2 through 8, Equations 26-1 through 26-16) is
/// NOT covered, for the reason given in the module header: the mixed-flow model is not
/// implemented and its published solution reads truck travel-time rates off nomographs by
/// eye. Published mixed-flow targets, so a future implementation has something to hit:
/// mixed-flow capacity 1,725 veh/h/ln and mixed-flow density 32.6 veh/mi/ln.
///
/// The PCE comparison is expressible today and is worth pinning on its own, because it is
/// the only published example that exercises grade interpolation between two tabulated rows
/// (5% falls between the Exhibit 12-26 4.5% and 5.5% rows) together with the rule that the
/// longest tabulated grade length applies to anything longer (the exhibit stops at 1 mi for
/// these grades; the segment is 2 mi).
///
/// Published: E_T 3.31 by interpolation between 3.11 and 3.51, f_HV 0.743, v_p 2,019
/// pc/h/ln, S 59.6 mi/h, D 33.9 pc/mi/ln, D_mix = D x f_HV = 25.2 veh/mi/ln.
///
/// The LOS letter asserted below is DERIVED, not published. Chapter 26 prints no letter for
/// this comparison: it stops at the densities and says only that D_mix "is the mixed-flow
/// density, not an auto-only flow density. As such, it cannot be used to derive LOS." That
/// caution is about D_mix. D itself is an auto-only PCE-based density and is exactly what
/// Exhibit 12-15 takes, and 33.9 pc/mi/ln falls in the >26-35 band, so D. The assertion is
/// here because this is the suite's only rural basic freeway segment and because 33.9 sits in
/// the one band where the old routing gave E instead (REVIEW_NOTES item 8b).
#[test]
fn ch26_ep5_pce_comparison_branch() {
    let mut seg = load_ch26("ep5_pce_comparison.json");
    let los = seg.run_operational_analysis().expect("EP5 PCE comparison");

    // Lane width, lateral clearance and ramp density are all set to base conditions because
    // the example explicitly neglects those three adjustments, leaving FFS = BFFS = 65.
    assert_approx(seg.ffs, 65.0, 1e-9, "FFS (mi/h)");
    assert_approx(seg.capacity, 2350.0, 1e-9, "capacity (pc/h/ln)");
    assert_approx(seg.e_t.unwrap(), 3.31, 1e-9, "E_T (5% grade, 2 mi, 15% trucks)");
    assert_approx(seg.phv, 0.743, 0.001, "f_HV");
    assert_approx(seg.v_p, 2019.0, 1.0, "v_p (pc/h/ln)");
    assert_approx(seg.speed, 59.6, 0.05, "S (mi/h)");
    assert_approx(seg.density, 33.9, 0.05, "D (pc/mi/ln)");
    assert_approx(seg.density * seg.phv, 25.2, 0.05, "D_mix (veh/mi/ln)");
    // Derived from the published D via Exhibit 12-15, not printed by the example. See above.
    assert_eq!(LevelOfService::D, los, "Exhibit 12-15 band for D = 33.9 pc/mi/ln");
}

/// HCM Chapter 26, Example Problem 6: severe weather effects on a basic freeway segment.
/// Same four-lane freeway as Example Problem 1 (BFFS 75.4, 11-ft lanes, 2-ft right-side
/// clearance, 4 ramps/mi, FFS 60.8 mi/h) in rolling terrain under heavy snow, with the
/// Exhibit 11-5 SAF of 0.86 and CAF of 0.78.
///
/// Published: FFS_adj = 60.8 x 0.86 = 52.3 mi/h, E_T 3.0 (Exhibit 12-25, rolling),
/// f_HV 0.909, v_p 1,195 pc/h/ln, BP_adj 1,161 pc/h/ln, S 52.3 mi/h, D 22.8 pc/mi/ln,
/// LOS C. The discussion also prints the unadjusted capacity, 2,308 pc/h/ln.
///
/// The adjusted capacity is the one value where this test does NOT assert what the seventh
/// edition prints. Chapter 26 as printed applies Equation 12-6 to FFS_adj and gets
/// c = 0.78 x (2,200 + 10 x [52.3 - 50]) = 1,734 pc/h/ln (quoted again as 1,743 two
/// paragraphs later, one of the two being a typo). The December 2022 errata reworks exactly
/// this computation to read the unadjusted FFS, giving
/// c = 0.78 x (2,200 + 10 x [60.8 - 50]) = 1,800 pc/h/ln, which is what the library
/// implements and what is asserted here. See VERIFICATION.md, Chapter 26 row.
///
/// The correction barely moves the outputs: it raises c_adj by 66 pc/h/ln, and because
/// v_p sits just past the breakpoint where the speed-flow curve is nearly flat, S changes by
/// about 0.01 mi/h and D by about 0.005 pc/mi/ln. The residual against the printed 22.8 is
/// the book's own rounding (it divides 1,195 by 52.3), not the errata.
#[test]
fn ch26_ep6_heavy_snow_basic_freeway() {
    let mut seg = load_ch26("ep6_heavy_snow.json");
    let los = seg.run_operational_analysis().expect("EP6");

    assert_approx(seg.ffs, 60.8, 0.05, "FFS (mi/h)");
    assert_approx(seg.ffs_adj, 52.3, 0.05, "FFS_adj (mi/h)");
    assert_approx(seg.e_t.unwrap(), 3.0, 1e-9, "E_T (rolling terrain)");
    assert_approx(seg.phv, 0.909, 0.001, "f_HV");
    assert_approx(seg.v_p, 1195.0, 1.0, "v_p (pc/h/ln)");
    assert_approx(seg.breakpoint, 1161.0, 1.0, "BP_adj (pc/h/ln)");
    assert_approx(seg.capacity, 2308.0, 0.5, "unadjusted capacity (pc/h/ln)");
    assert_approx(seg.capacity_adj, 1800.0, 0.5, "c_adj, December 2022 errata (pc/h/ln)");
    assert_approx(seg.speed, 52.3, 0.1, "S (mi/h)");
    assert_approx(seg.density, 22.8, 0.1, "D (pc/mi/ln)");
    assert_eq!(LevelOfService::C, los, "EP6 LOS");
}

/// Regression guard for REVIEW_NOTES item 8b. Segment LOS is Exhibit 12-15, which prints one
/// density-to-LOS table with no urban/rural split, so area type must not reach it. It used to:
/// `determine_segment_los` went through `FacilityCalculation::los_from_density`, which applies
/// the Exhibit 10-6 FACILITY bands and branches on `city_type`.
///
/// The two exhibits agree on the urban row value for value, so an all-Urban suite cannot see
/// this. The rural row breaks at 6/14/22/29/39 against 11/18/26/35/45, which is why the sweep
/// below forces both area types on the same fixture rather than trusting the one it ships with.
///
/// 33.9 pc/mi/ln (Example Problem 5) and 22.9 pc/mi/ln (Example Problem 6) are the two
/// densities in the suite that fall in a band where the exhibits disagree, so they are the
/// cases with the power to catch a regression here. The remaining fixtures are included
/// because a cheap sweep is worth more than a curated pair if a fixture's density later moves.
#[test]
fn segment_los_does_not_depend_on_area_type() {
    for name in [
        "ep4_eastbound_downgrade.json",
        "ep4_westbound_upgrade.json",
        "ep5_pce_comparison.json",
        "ep6_heavy_snow.json",
    ] {
        let mut urban = load_ch26(name);
        urban.city_type = transportations_library::common::CityType::Urban;
        let urban_los = urban.run_operational_analysis().expect(name);

        let mut rural = load_ch26(name);
        rural.city_type = transportations_library::common::CityType::Rural;
        let rural_los = rural.run_operational_analysis().expect(name);

        assert_approx(rural.density, urban.density, 1e-9, &format!("{name} density"));
        assert_eq!(urban_los, rural_los, "{name} LOS must not depend on area type");
    }

    // Control: the comparison above is only meaningful if it could have failed. These are the
    // Exhibit 10-6 bands the old path applied, and they do split on area type at both of the
    // densities the sweep covers.
    use transportations_library::hcm::freeway_facilities::exhibits::los_freeway_facility;
    use transportations_library::common::CityType;
    for density in [33.9, 22.9] {
        assert_ne!(
            los_freeway_facility(density, false, CityType::Urban),
            los_freeway_facility(density, false, CityType::Rural),
            "Exhibit 10-6 must disagree at {density} pc/mi/ln for this test to have teeth",
        );
    }
}
