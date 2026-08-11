//! HCM Chapter 15 (two-lane highways) tests.
//!
//! # Fixture-to-example-problem mapping
//!
//! `read_test_files` sorts the `caseN.json` fixtures by name, so the index into
//! every `ans_*` table below is the fixture number minus one. Each fixture is a
//! faithful transcription of one published example problem in HCM Chapter 26,
//! Section 8, and the correspondence is one-to-one:
//!
//! | Index | Fixture | Chapter 26 example problem | Input source |
//! |-------|---------|----------------------------|--------------|
//! | 0 | `case1.json` | EP1, level straight Passing Constrained segment | The Facts (0.75 mi, 752 veh/h, PHF 0.94, 50 mi/h, 5% HV, 0% grade) |
//! | 1 | `case2.json` | EP2, Passing Constrained segment with horizontal curves | EP1 inputs plus the 11 subsegments of Exhibit 26-23 |
//! | 2 | `case3.json` | EP3, facility analysis in level terrain | Exhibit 26-26 |
//! | 3 | `case4.json` | EP4, facility analysis on a mountain road | Exhibits 26-29 (volumes, segment types) and 26-30 (grades, curves) |
//!
//! No fixture in this directory is synthetic. The two non-`caseN` fixtures are
//! also published: `bicycle_widening.json` is EP5 (two-lane highway bicycle
//! LOS), exercised separately by `bicycle_los_widening_example_test`, and
//! `case_study1.json` is the River Falls corridor rather than an HCM example.
//! Both are excluded from `read_test_files` by shape.
//!
//! Input transcription was checked against the exhibits and conserves: the
//! case2 subsegment lengths sum to the 3,960-ft segment (Exhibit 26-23), the
//! case3 segment lengths sum to the 5.5-mi facility (Exhibit 26-26), and each
//! case4 segment's subsegment lengths sum to its own length, totalling the
//! 5.1-mi facility (Exhibit 26-30).
//!
//! # Which expected values are published, and which are not
//!
//! Not every number in the `ans_*` tables has a published counterpart, because
//! the example problems report different measures. Read them as follows.
//!
//! * Average speed (`ans_s`): case1 seg 1 = 53.7 and case2 seg 1 = 49.5 are the
//!   published EP1 and EP2 results, the latter being the length-weighted
//!   average over Exhibit 26-25. The case4 row reproduces the "Adjusted S" row
//!   of Exhibit 26-34 exactly (47.9 / 43.9 / 50.8 / 49.2 / 56.0 / 58.3). EP3
//!   publishes no per-segment speed table, so the case3 row is engine output.
//! * Follower density (`ans_fd`): this test computes the UNADJUSTED per-segment
//!   density, before the Step 9 passing-lane adjustment. For case4 that is the
//!   FD row of Exhibit 26-35, which the fixture matches on five of six
//!   segments; segment 6 expects 16.4 against a published 16.5. For case1,
//!   10.1 is the published EP1 result. EP2 stops at average speed and EP3
//!   publishes only adjusted densities, so case2's 10.9 and case3's segments
//!   2-5 have no published counterpart.
//! * Adjusted follower density (`ans_fd_adj`): case3 segments 4 and 5 (8.2 and
//!   8.8) and case4 segment 6 (13.2) match Exhibits 26-27 and 26-36 exactly.
//!   case3 segment 3 expects 8.3 against a published 8.2. The passing-lane
//!   entries (case3 index 1, case4 index 4) are not comparable to the exhibits:
//!   the book reports the passing-lane midpoint density there (2.9 and 6.2),
//!   whereas this test calls `determine_adjustment_to_follower_density` on
//!   every segment uniformly.
//! * Segment LOS (`ans_los`): every entry matches the published LOS column of
//!   Exhibit 26-27 (case3: D, B, D, D, D) and Exhibit 26-36 (case4: E, E, E, E,
//!   C, E). Note that the case4 row follows Exhibit 26-36, not the EP4 Step 10
//!   prose, which claims "all segments operate at LOS E" while its own exhibit
//!   puts the passing-lane segment 5 at 6.2 followers/mi and LOS C. The exhibit
//!   is the consistent reading, since 6.2 cannot be LOS E under Exhibit 15-6.
//!
//! # Facility aggregation
//!
//! `determine_facility_los_test` now calls
//! `TwoLaneHighways::determine_facility_follower_density`, which aggregates
//! what Equation 15-39 asks for: the adjusted density where the Step 9
//! passing-lane benefit applies, FD_PLmid on a passing lane segment, and the
//! plain Step 8 density elsewhere. Weighting the published per-segment column
//! of Exhibit 26-27 by segment length gives (10.7)(0.75) + (2.9)(1.5) +
//! (8.2)(1.0) + (8.2)(0.5) + (8.8)(1.75) = 40.075 over 5.5 mi = 7.3
//! followers/mi and LOS C, which the engine now reproduces at 7.271. The
//! earlier harness weighted the unadjusted densities and reached 8.041, hence
//! LOS D. case4 carries the same correction (20.219 to 19.897) but was masked,
//! because both aggregates fall inside the LOS E band.

use assert_approx_eq::assert_approx_eq;
use transportations_library::math;
use transportations_library::twolanehighways::{BicycleLOS, Segment, SubSegment, TwoLaneHighways};

use std::fs::{self, File};
use std::io::BufReader;
use std::path::PathBuf;

fn read_test_files() -> Vec<String> {
    let mut examples_root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    examples_root_dir.push("./tests/ExampleCases/hcm/TwoLaneHighways/");
    let paths = fs::read_dir(examples_root_dir).expect("Unable to read directory");
    let mut setting_files: Vec<String> = Vec::new();

    for path in paths {
        let path = path.unwrap().path();
        // Only include the motorized-methodology fixtures caseN.json; other
        // fixtures in this directory (case_study*, bicycle_*) have different shapes.
        let name = path.file_name().unwrap().to_string_lossy().to_string();
        if name.starts_with("case")
            && !name.contains("case_study")
            && name[4..name.len() - 5].chars().all(|c| c.is_ascii_digit())
        {
            setting_files.push(path.display().to_string());
        }
    }

    setting_files.sort();

    setting_files
}

// fn settings<T: SegmentOperations>(setting_file_loc: String) -> TwoLaneHighways<T> {
fn settings(setting_file_loc: String) -> TwoLaneHighways {
    let f = File::open(setting_file_loc).expect("Unable to open file");
    let reader = BufReader::new(f);

    // let twolanehighways: TwoLaneHighways<T> = serde_json::from_reader(reader).expect("Failed to parse JSON");
    let twolanehighways: TwoLaneHighways =
        serde_json::from_reader(reader).expect("Failed to parse JSON");

    twolanehighways
}

// fn case_initialize<T: SegmentOperations>(tlh: TwoLaneHighways<T>) -> (TwoLaneHighways<Segment>, usize) {
fn initialize_test_case(tlh: TwoLaneHighways) -> (TwoLaneHighways, usize) {
    let seg_len = tlh.segments.len();
    let mut segments_vec = Vec::new();

    for seg_num in 0..seg_len {
        // let subseg_len = tlh.segments[seg_num].subsegments.len();
        let subseg_len = tlh.segments[seg_num].get_subsegments().len();
        let mut subsegments_vec = Vec::new();
        for subseg_num in 0..subseg_len {
            let subsegment = SubSegment::new(
                tlh.segments[seg_num].get_subsegments()[subseg_num].length,
                tlh.segments[seg_num].get_subsegments()[subseg_num].avg_speed,
                tlh.segments[seg_num].get_subsegments()[subseg_num].hor_class,
                tlh.segments[seg_num].get_subsegments()[subseg_num].design_rad,
                tlh.segments[seg_num].get_subsegments()[subseg_num].central_angle,
                tlh.segments[seg_num].get_subsegments()[subseg_num].sup_ele,
            );
            subsegments_vec.push(subsegment);
        }

        let segment = Segment::new(
            tlh.segments[seg_num].get_passing_type(),
            tlh.segments[seg_num].get_length(),
            tlh.segments[seg_num].get_grade(),
            tlh.segments[seg_num].get_spl(),
            Some(tlh.segments[seg_num].get_is_hc()),
            Some(tlh.segments[seg_num].get_volume()),
            Some(tlh.segments[seg_num].get_volume_op()),
            Some(tlh.segments[seg_num].get_flow_rate()),
            Some(tlh.segments[seg_num].get_flow_rate_o()),
            Some(tlh.segments[seg_num].get_capacity()),
            Some(tlh.segments[seg_num].get_ffs()),
            Some(tlh.segments[seg_num].get_avg_speed()),
            Some(tlh.segments[seg_num].get_vertical_class()),
            Some(subsegments_vec),
            Some(tlh.segments[seg_num].get_phf()),
            Some(tlh.segments[seg_num].get_phv()),
            Some(tlh.segments[seg_num].get_percent_followers()),
            Some(tlh.segments[seg_num].get_followers_density()),
            Some(tlh.segments[seg_num].get_followers_density_mid()),
            Some(tlh.segments[seg_num].get_hor_class()),
        );
        segments_vec.push(segment);
    }

    let twolanehighways = TwoLaneHighways {
        segments: segments_vec,
        lane_width: tlh.lane_width,
        shoulder_width: tlh.shoulder_width,
        apd: tlh.apd,
        pmhvfl: tlh.pmhvfl,
        l_de: tlh.l_de,
    };

    (twolanehighways, seg_len)
}

#[test]
fn identity_vertical_class_test() {
    let ans_min = vec![
        [0.25, 0.0, 0.0, 0.0, 0.0, 0.0],
        [0.25, 0.0, 0.0, 0.0, 0.0, 0.0],
        [0.25, 0.5, 0.25, 0.25, 0.25, 0.0],
        [0.5, 0.5, 0.5, 0.5, 0.5, 0.25],
    ];
    let ans_max = vec![
        [3.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        [3.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        [3.0, 3.0, 3.0, 2.0, 3.0, 0.0],
        [3.0, 3.0, 3.0, 3.0, 3.0, 3.0],
    ];
    let setting_files = read_test_files();

    for (index, s_file) in setting_files.iter().enumerate() {
        // let tlh: TwoLaneHighways<Segment> = settings(s_file.clone());
        let tlh: TwoLaneHighways = settings(s_file.clone());
        let (mut twolanehighways, seg_len) = initialize_test_case(tlh);

        for seg_num in 0..seg_len {
            let (_min, _max) = twolanehighways.identify_vertical_class(seg_num);
            assert_eq!(
                (ans_min[index][seg_num], ans_max[index][seg_num]),
                (_min, _max)
            );
        }
    }
}

#[test]
fn determine_demand_flow_test() {
    let ans_demand_flow_i = vec![
        [800.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        [800.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        [904.0, 868.0, 863.0, 851.0, 850.0, 0.0],
        [1222.0, 1222.0, 1222.0, 1222.0, 1222.0, 1222.0],
    ];
    let ans_demand_flow_o = vec![
        [1500.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        [1500.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        [1500.0, 0.0, 1500.0, 532.0, 1500.0, 0.0],
        [1500.0, 1500.0, 1500.0, 1500.0, 0.0, 1500.0],
    ];
    let ans_capacity = vec![
        [1700.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        [1700.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        [1700.0, 1500.0, 1700.0, 1700.0, 1700.0, 0.0],
        [1700.0, 1700.0, 1700.0, 1700.0, 1500.0, 1700.0],
    ];

    let setting_files = read_test_files();
    for (index, s_file) in setting_files.iter().enumerate() {
        // let tlh: TwoLaneHighways<Segment> = settings(s_file.clone());
        let tlh: TwoLaneHighways = settings(s_file.clone());

        let (mut twolanehighways, seg_len) = initialize_test_case(tlh);

        for seg_num in 0..seg_len {
            let (demand_flow_i, demand_flow_o, capacity) =
                twolanehighways.determine_demand_flow(seg_num);
            assert_eq!(
                (
                    ans_demand_flow_i[index][seg_num],
                    ans_demand_flow_o[index][seg_num],
                    ans_capacity[index][seg_num]
                ),
                // (demand_flow_i, math::round_to_significant_digits(demand_flow_o, 3), capacity.into()));
                (
                    demand_flow_i.round(),
                    demand_flow_o.round(),
                    capacity.into()
                )
            );
        }
    }
}

#[test]
fn determine_vertical_alignment_test() {
    let ans_ver_align = vec![
        [1, 0, 0, 0, 0, 0],
        [1, 0, 0, 0, 0, 0],
        [1, 1, 1, 1, 1, 1],
        [4, 5, 4, 4, 1, 1],
    ];

    let setting_files = read_test_files();
    for (index, s_file) in setting_files.iter().enumerate() {
        // let tlh : TwoLaneHighways<Segment>= settings(s_file.clone());
        let tlh: TwoLaneHighways = settings(s_file.clone());
        let (mut twolanehighways, seg_len) = initialize_test_case(tlh);

        for seg_num in 0..seg_len {
            let ver_align = twolanehighways.determine_vertical_alignment(seg_num);
            assert_eq!(ans_ver_align[index][seg_num], ver_align);
        }
    }
}

#[test]
fn determine_free_flow_speed_test() {
    let ans_ffs = vec![
        [56.83, 0.0, 0.0, 0.0, 0.0, 0.0],
        [56.83, 0.0, 0.0, 0.0, 0.0, 0.0],
        [62.43, 62.43, 62.43, 62.45, 62.43, 0.0],
        [60.02, 59.04, 60.07, 60.02, 62.43, 62.43],
    ];
    let setting_files = read_test_files();

    for (index, s_file) in setting_files.iter().enumerate() {
        // let tlh : TwoLaneHighways<Segment>= settings(s_file.clone());
        let tlh: TwoLaneHighways = settings(s_file.clone());

        let (mut twolanehighways, seg_len) = initialize_test_case(tlh);

        for seg_num in 0..seg_len {
            let (_, _, _) = twolanehighways.determine_demand_flow(seg_num);
            let ffs = twolanehighways.determine_free_flow_speed(seg_num);
            assert_eq!(ans_ffs[index][seg_num], math::round_up_to_n_decimal(ffs, 2));
        }
    }
}

#[test]
fn estimate_average_speed_test() {
    let ans_s = vec![
        [53.7, 0.0, 0.0, 0.0, 0.0, 0.0],
        [49.5, 0.0, 0.0, 0.0, 0.0, 0.0],
        [58.8, 57.8, 58.9, 59.2, 58.9, 0.0],
        [47.9, 43.9, 50.8, 49.2, 56.0, 58.3],
    ];
    let setting_files = read_test_files();
    for (index, s_file) in setting_files.iter().enumerate() {
        // let tlh : TwoLaneHighways<Segment>= settings(s_file.clone());
        let tlh: TwoLaneHighways = settings(s_file.clone());

        let (mut twolanehighways, seg_len) = initialize_test_case(tlh);

        // Set free flow speed
        for seg_num in 0..seg_len {
            let (_, _, _) = twolanehighways.determine_demand_flow(seg_num);
            let _ = twolanehighways.determine_free_flow_speed(seg_num);
            let (s, _) = twolanehighways.estimate_average_speed(seg_num);

            // let subseg_num = twolanehighways.get_segments()[seg_num].get_subsegments().len();
            // while j < subseg_num {
            //     tot_s += s;
            // }
            assert_eq!(ans_s[index][seg_num], math::round_up_to_n_decimal(s, 1));
        }
    }
}

#[test]
fn estimate_percent_followers_test() {
    let ans_pf = vec![
        [67.7, 0.0, 0.0, 0.0, 0.0, 0.0],
        [67.7, 0.0, 0.0, 0.0, 0.0, 0.0],
        [69.7, 60.7, 68.0, 67.8, 67.7, 0.0],
        [86.9, 89.3, 83.9, 86.9, 78.2, 78.4],
    ];
    let setting_files = read_test_files();
    for (index, s_file) in setting_files.iter().enumerate() {
        // let tlh : TwoLaneHighways<Segment>= settings(s_file.clone());
        let tlh: TwoLaneHighways = settings(s_file.clone());
        let (mut twolanehighways, seg_len) = initialize_test_case(tlh);

        for seg_num in 0..seg_len {
            let (_, _, _) = twolanehighways.determine_demand_flow(seg_num);
            let _ = twolanehighways.determine_free_flow_speed(seg_num);
            let pf = twolanehighways.estimate_percent_followers(seg_num);
            assert_eq!(
                ans_pf[index][seg_num],
                math::round_to_significant_digits(pf, 3)
            );
        }
    }
}

#[test]
fn determine_follower_density_test() {
    // let ans_fd = vec![[10.1, 0.0, 0.0, 0.0, 0.0, 0.0], [10.9, 0.0, 0.0, 0.0, 0.0, 0.0], [10.7, 9.1, 10.0, 9.8, 9.8, 0.0], [22.2, 24.9, 20.2, 21.6, 17.2, 16.4]];
    let ans_fd = vec![
        [10.1, 0.0, 0.0, 0.0, 0.0, 0.0],
        [10.9, 0.0, 0.0, 0.0, 0.0, 0.0],
        [10.7, 9.1, 10.0, 9.7, 9.8, 0.0],
        [22.2, 24.9, 20.2, 21.6, 17.1, 16.4],
    ];
    let setting_files = read_test_files();
    for (index, s_file) in setting_files.iter().enumerate() {
        // let tlh : TwoLaneHighways<Segment>= settings(s_file.clone());
        let tlh: TwoLaneHighways = settings(s_file.clone());

        let (mut twolanehighways, seg_len) = initialize_test_case(tlh);

        let mut fd: f64;

        for seg_num in 0..seg_len {
            let (_, _, _) = twolanehighways.determine_demand_flow(seg_num);
            let _ = twolanehighways.determine_free_flow_speed(seg_num);
            let (_, _) = twolanehighways.estimate_average_speed(seg_num);
            let _ = twolanehighways.estimate_percent_followers(seg_num);
            if twolanehighways.get_segments()[seg_num].passing_type == 2 {
                (fd, _) = twolanehighways.determine_follower_density_pl(seg_num);
            } else {
                fd = twolanehighways.determine_follower_density_pc_pz(seg_num);
            }

            assert_eq!(ans_fd[index][seg_num], math::round_up_to_n_decimal(fd, 1));
        }
    }
}

#[test]
fn determine_adjustment_to_follower_density_test() {
    let ans_fd_adj = vec![
        [0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        [0.0, 10.3, 8.3, 8.2, 8.8, 0.0],
        [0.0, 0.0, 0.0, 0.0, 18.0, 13.2],
    ];
    let setting_files = read_test_files();
    for (index, s_file) in setting_files.iter().enumerate() {
        // let tlh : TwoLaneHighways<Segment>= settings(s_file.clone());
        let tlh: TwoLaneHighways = settings(s_file.clone());

        let (mut twolanehighways, seg_len) = initialize_test_case(tlh);

        for seg_num in 0..seg_len {
            println!("Case {}", index);
            println!("Segment {}", seg_num);
            let (_, _, _) = twolanehighways.determine_demand_flow(seg_num);
            let _ = twolanehighways.determine_free_flow_speed(seg_num);
            let (_, _) = twolanehighways.estimate_average_speed(seg_num);
            let _ = twolanehighways.estimate_percent_followers(seg_num);
            let _ = twolanehighways.determine_follower_density_pc_pz(seg_num);

            let fd_adj = twolanehighways.determine_adjustment_to_follower_density(seg_num);

            // assert_eq!(ans_fd_adj[index][seg_num], math::round_to_significant_digits(fd_adj, 3));
            assert_eq!(
                ans_fd_adj[index][seg_num],
                math::round_up_to_n_decimal(fd_adj, 1)
            );
        }
    }
}

#[test]
fn determine_segment_los_test() {
    let ans_los = vec![
        ['D', '\0', '\0', '\0', '\0', '\0'],
        ['D', '\0', '\0', '\0', '\0', '\0'],
        ['D', 'B', 'D', 'D', 'D', '\0'],
        ['E', 'E', 'E', 'E', 'C', 'E'],
    ];
    let setting_files = read_test_files();
    for (index, s_file) in setting_files.iter().enumerate() {
        // let tlh : TwoLaneHighways<Segment>= settings(s_file.clone());
        let tlh: TwoLaneHighways = settings(s_file.clone());

        let (mut twolanehighways, seg_len) = initialize_test_case(tlh);

        for seg_num in 0..seg_len {
            let (_, _, capacity) = twolanehighways.determine_demand_flow(seg_num);
            let _ = twolanehighways.determine_free_flow_speed(seg_num);
            let (s, _) = twolanehighways.estimate_average_speed(seg_num);
            let _ = twolanehighways.estimate_percent_followers(seg_num);
            if twolanehighways.get_segments()[seg_num].get_passing_type() == 2 {
                let (_, _) = twolanehighways.determine_follower_density_pl(seg_num);
            } else {
                let _ = twolanehighways.determine_follower_density_pc_pz(seg_num);
            }
            let los = twolanehighways.determine_segment_los(seg_num, s, capacity);

            assert_eq!(ans_los[index][seg_num], los);
        }
    }
}

#[test]
fn determine_facility_los_test() {
    // case3 is Chapter 26 Example Problem 3: facility follower density 7.3
    // followers/mi and LOS C in Exhibit 26-27. case4 is Example Problem 4,
    // LOS E in Exhibit 26-36. case1 and case2 are single-segment fixtures
    // with no published facility row.
    let ans_los = ['D', 'D', 'C', 'E'];
    let ans_fd_f = [10.092, 10.933, 7.271, 19.897];

    let setting_files = read_test_files();
    for (index, s_file) in setting_files.iter().enumerate() {
        // let tlh : TwoLaneHighways<Segment>= settings(s_file.clone());
        let tlh: TwoLaneHighways = settings(s_file.clone());

        let (mut twolanehighways, seg_len) = initialize_test_case(tlh);
        let mut tot_len: f64 = 0.0;
        let mut s_tot: f64 = 0.0;

        for seg_num in 0..seg_len {
            let (_, _, _) = twolanehighways.determine_demand_flow(seg_num);
            let _ = twolanehighways.determine_free_flow_speed(seg_num);
            let (s, _) = twolanehighways.estimate_average_speed(seg_num);
            let _ = twolanehighways.estimate_percent_followers(seg_num);
            if twolanehighways.get_segments()[seg_num].get_passing_type() == 2 {
                let (_, _) = twolanehighways.determine_follower_density_pl(seg_num);
            } else {
                let _ = twolanehighways.determine_follower_density_pc_pz(seg_num);
            }
            tot_len += twolanehighways.get_segments()[seg_num].get_length();
            s_tot += s * twolanehighways.get_segments()[seg_num].get_length();
        }

        let fd_f = twolanehighways.determine_facility_follower_density();
        let average_speed = s_tot / tot_len;
        let fac_los = twolanehighways.determine_facility_los(fd_f, average_speed);

        assert_approx_eq!(ans_fd_f[index], fd_f, 0.001);
        assert_eq!(ans_los[index], fac_los);
    }
}

/// Test Bicycle LOS calculation based on HCM Chapter 15 Section 4
/// Example based on typical two-lane highway conditions
#[test]
fn bicycle_los_test() {
    // Test case 1: Good conditions (wide lane, wide shoulder, good pavement)
    let bike_los1 = BicycleLOS::new(
        12.0,  // lane width
        6.0,   // shoulder width
        50.0,  // speed limit
        1,     // num lanes
        4.0,   // pavement condition (good)
        500.0, // hourly volume
        0.88,  // PHF
        0.06,  // heavy vehicle %
        0.0,   // on-highway parking %
    );

    let result1 = bike_los1.analyze();
    assert!(result1.blos_score > 0.0, "BLOS score should be positive");
    assert!(result1.effective_width > 0.0, "Effective width should be positive");
    assert!(['A', 'B', 'C', 'D', 'E', 'F'].contains(&result1.los), "LOS should be A-F");

    // Test case 2: Poor conditions (narrow lane, no shoulder, poor pavement)
    let bike_los2 = BicycleLOS::new(
        10.0,   // lane width
        0.0,    // shoulder width
        55.0,   // speed limit
        1,      // num lanes
        2.0,    // pavement condition (poor)
        800.0,  // hourly volume
        0.88,   // PHF
        0.10,   // heavy vehicle %
        0.0,    // on-highway parking %
    );

    let result2 = bike_los2.analyze();
    // Poor conditions should result in worse (higher) BLOS score
    assert!(
        result2.blos_score > result1.blos_score,
        "Worse conditions should have higher BLOS score"
    );

    // Test case 3: Very low volume (should benefit from Equation 15-45)
    let bike_los3 = BicycleLOS::new(
        12.0,   // lane width
        4.0,    // shoulder width
        45.0,   // speed limit
        1,      // num lanes
        4.0,    // pavement condition (good)
        100.0,  // hourly volume (< 160)
        0.88,   // PHF
        0.06,   // heavy vehicle %
        0.0,    // on-highway parking %
    );

    let result3 = bike_los3.analyze();
    // Low volume should have better (lower) BLOS score
    assert!(
        result3.blos_score < result1.blos_score,
        "Lower volume should have lower BLOS score"
    );

    // Test default constructor
    let bike_los_default = BicycleLOS::default();
    let result_default = bike_los_default.analyze();
    assert!(result_default.blos_score > 0.0, "Default BLOS should work");
}

/// HCM worked example (Chapter 26 two-lane highway example problems): a segment is evaluated for
/// widening, realigning, and repaving; the BLOS in the peak direction is compared between the
/// current roadway and the proposed design. Fixture: bicycle_widening.json. Published results:
/// current BLOS 5.90 (LOS F), proposed BLOS 3.58 (LOS D). Tolerances ±0.01 on scores (book rounds
/// intermediates to two decimals), LOS letters exact.
#[test]
fn bicycle_los_widening_example_test() {
    let mut fixture = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    fixture.push("tests/ExampleCases/hcm/TwoLaneHighways/bicycle_widening.json");
    let f = File::open(fixture).expect("Unable to open bicycle_widening.json");
    let json: serde_json::Value =
        serde_json::from_reader(BufReader::new(f)).expect("Failed to parse JSON");

    let load = |key: &str| -> BicycleLOS {
        serde_json::from_value(json[key].clone()).expect("Failed to parse BicycleLOS inputs")
    };

    let current = load("current").analyze();
    let proposed = load("proposed").analyze();

    // Step 2: vOL = 500 / (0.90 * 1) = 556 veh/h (both designs)
    assert_approx_eq!(current.flow_rate_outside_lane, 555.6, 0.1);

    // Step 3: We = 14 ft current (Eqs 15-43/15-44), 24 ft proposed (Eqs 15-42/15-44)
    assert_approx_eq!(current.effective_width, 14.0, 0.01);
    assert_approx_eq!(proposed.effective_width, 24.0, 0.01);

    // Step 4: St = 4.62 current (Spl 50), 4.79 proposed (Spl 55) per Eq 15-46
    assert_approx_eq!(current.effective_speed_factor, 4.62, 0.01);
    assert_approx_eq!(proposed.effective_speed_factor, 4.79, 0.01);

    // Step 5: BLOS 5.90 -> F current, 3.58 -> D proposed (Eq 15-47, Exhibit 15-7)
    assert_approx_eq!(current.blos_score, 5.90, 0.01);
    assert_approx_eq!(proposed.blos_score, 3.58, 0.01);
    assert_eq!(current.los, 'F');
    assert_eq!(proposed.los, 'D');
}
