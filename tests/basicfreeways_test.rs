use transportations_library::math;
use transportations_library::basicfreeways::BasicFreeways;

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
    let ans = vec![
        60.8, 67.3
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

#[test]
fn estimate_number_of_lanes() {
    let ans = vec![
        0, 3
    ];
    let setting_files = read_test_files();

    for (index, s_file) in setting_files.iter().enumerate() {
        let bcf: BasicFreeways = settings(s_file.clone());
        let mut basicfreeways = initialize_test_case(bcf);
        let _num_lanes: u32;
        let _num_lane_med: f64;
        if basicfreeways.v_p != 0.0 {
            (_num_lanes, _num_lane_med) = basicfreeways.estimate_number_of_lanes();
        } else {
            _num_lanes = 0;
        }
        assert_eq!(
            ans[index], _num_lanes,
            "Test case {index} failed",
        );
    }
}

#[test]
fn estimate_capacity_test() {
    let ans = vec![
        2308.0, 2373.0
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
    let ans = vec![
        1142, 1694
    ];
    let setting_files = read_test_files();

    for (index, s_file) in setting_files.iter().enumerate() {
        let bcf: BasicFreeways = settings(s_file.clone());
        let mut basicfreeways = initialize_test_case(bcf);

        let _ffs = basicfreeways.determine_free_flow_speed();
        let _capacity = basicfreeways.estimate_capacity();

        // Estimate number of lanes
        if basicfreeways.v_p != 0.0 {
            (_, _) = basicfreeways.estimate_number_of_lanes();
        }

        let _demand_volume = basicfreeways.estimate_demand_volume();
        assert_eq!(
            ans[index], math::round_up_to_n_decimal(_demand_volume, 0) as i32,
            "Test case {index} failed",
        );
    }
}

// Need BreakPoint and estimated speed

#[test]
fn estimate_density_test() {
    let ans = vec![
        18.8, 25.9
    ];
    let setting_files = read_test_files();

    for (index, s_file) in setting_files.iter().enumerate() {
        let bcf: BasicFreeways = settings(s_file.clone());
        let mut basicfreeways = initialize_test_case(bcf);

        let _ffs = basicfreeways.determine_free_flow_speed();
        let _capacity = basicfreeways.estimate_capacity();

        // Estimate number of lanes
        if basicfreeways.v_p != 0.0 {
            (_, _) = basicfreeways.estimate_number_of_lanes();
        }

        let _demand_volume = basicfreeways.estimate_demand_volume();
        let _density = basicfreeways.estimate_density();
        assert_eq!(
            ans[index], math::round_up_to_n_decimal(basicfreeways.density, 1),
            "Test case {index} failed",
        );
    }
}

#[test]
fn determine_segment_los() {
    let ans = vec![
        'C', 'C'
    ];
    let setting_files = read_test_files();

    for (index, s_file) in setting_files.iter().enumerate() {
        let bcf: BasicFreeways = settings(s_file.clone());
        let mut basicfreeways = initialize_test_case(bcf);

        let _ffs = basicfreeways.determine_free_flow_speed();
        let _capacity = basicfreeways.estimate_capacity();

        // Estimate number of lanes
        if basicfreeways.v_p != 0.0 {
            (_, _) = basicfreeways.estimate_number_of_lanes();
        }

        let _demand_volume = basicfreeways.estimate_demand_volume();
        let _density = basicfreeways.estimate_density();
        let _los = basicfreeways.determine_segment_los();
        let _los_char: char = _los.into();
        assert_eq!(
            ans[index], _los_char,
            "Test case {index} failed",
        );
    }

}