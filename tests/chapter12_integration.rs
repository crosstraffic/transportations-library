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
