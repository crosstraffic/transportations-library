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
