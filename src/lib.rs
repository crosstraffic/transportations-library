mod utils;
mod hcm;

pub use crate::hcm::*;



#[cfg(test)]
mod twolanehighways_test {
    use serde_json::{Result, Value};
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;
    use std::path::Path;

    use super::TwoLaneHighways;
    use super::Segment;
    use super::SubSegment;
    use super::utils::math;
    // const examples_root_dir: &str = "src/ExampleCases/hcm/TwoLaneHighways";
    // const case1_path: &Path = &Path::new(examples_root_dir).join("case1.json");
    const case1_path: &str = "src/ExampleCases/hcm/TwoLaneHighways/case1.json";
    const case2_path: &str = "src/ExampleCases/hcm/TwoLaneHighways/case2.json";
    // const case3_path: &str = "src/ExampleCases/hcm/TwoLaneHighways/case3.json";
    // const case4_path: &str = "src/ExampleCases/hcm/TwoLaneHighways/case4.json";
    // const case5_path: &str = "src/ExampleCases/hcm/TwoLaneHighways/case5.json";

    // TODO: Automatically gets all cases
    // const SETTING_FILES: [&str; 2]= [case1_path, case2_path];
    const SETTING_FILES: [&str; 1]= [case1_path];


    fn settings(setting_file_loc: &str) -> TwoLaneHighways {
        let f = File::open(setting_file_loc).expect("Unable to open file");
        let reader = BufReader::new(f);

        let twolanehighways: TwoLaneHighways = serde_json::from_reader(reader).expect("Failed to parse JSON");
        // let segment: Segment = value["1"];

        twolanehighways
    }


    fn case_initialize(tlh: TwoLaneHighways) -> (TwoLaneHighways, usize) {

        let seg_len = tlh.segments.len();
        let mut segments_vec =  Vec::new();

        for seg_num in 0..seg_len {
            let subseg_len = tlh.segments[seg_num].subsegments.len();
            let mut subsegments_vec = Vec::new();
            for subseg_num in 0..subseg_len {
                let subsegment = SubSegment::new(
                    tlh.segments[seg_num].subsegments[subseg_num].length,
                    tlh.segments[seg_num].subsegments[subseg_num].avg_speed,
                    tlh.segments[seg_num].subsegments[subseg_num].hor_class,
                    tlh.segments[seg_num].subsegments[subseg_num].design_rad,
                    tlh.segments[seg_num].subsegments[subseg_num].sup_ele,
                );
                subsegments_vec.push(subsegment);
            }

            let segment = Segment::new(
                tlh.segments[seg_num].passing_type,
                tlh.segments[seg_num].length,
                tlh.segments[seg_num].grade,
                tlh.segments[seg_num].is_hc,
                tlh.segments[seg_num].volume,
                tlh.segments[seg_num].volume_op,
                tlh.segments[seg_num].flow_rate,
                tlh.segments[seg_num].flow_rate_o,
                tlh.segments[seg_num].capacity,
                tlh.segments[seg_num].ffs,
                tlh.segments[seg_num].avg_speed,
                tlh.segments[seg_num].vertical_class,
                subsegments_vec,
                tlh.segments[seg_num].phf,
                tlh.segments[seg_num].phv,
                tlh.segments[seg_num].pf,
                tlh.segments[seg_num].hor_class,
            );
            segments_vec.push(segment);
        }

        let twolanehighways = TwoLaneHighways {
            segments: segments_vec,
            spl: 50.0,
            lane_width: 12.0,
            shoulder_width: 6.0,
            apd: 0.0,
            pmhvfl: 0.0,
        };


        (twolanehighways, seg_len)
    }

    
    #[test]
    pub fn identity_vertical_class_test() {
        let ans1_min = 0.25;
        let ans1_max = 3.0;

        for s_file in SETTING_FILES {
            let tlh = settings(s_file);

            let (mut twolanehighways, seg_len) = case_initialize(tlh);

            for seg_num in 0..seg_len {
                let (_min, _max) = twolanehighways.identify_vertical_class(seg_num);
                assert_eq!((ans1_min, ans1_max), (_min, _max));
            }
        }
    }

    #[test]
    pub fn determine_demand_flow_test() {
        let ans1_demand_flow_i = 800.0;
        let ans1_demand_flow_o = 1500.0;
        let ans1_capacity = 1700.0;

        for s_file in SETTING_FILES {
            let tlh = settings(s_file);

            let (mut twolanehighways, seg_len) = case_initialize(tlh);

            for seg_num in 0..seg_len {
                let (demand_flow_i, demand_flow_o, capacity) = twolanehighways.determine_demand_flow(seg_num);
                assert_eq!((ans1_demand_flow_i, ans1_demand_flow_o, ans1_capacity), (demand_flow_i, demand_flow_o, capacity.into()));
            }
        }

    }

    #[test]
    pub fn determine_vertical_alignment_test() {
        let ans1_ver_align = 1;

        for s_file in SETTING_FILES {
            let tlh = settings(s_file);
            let (mut twolanehighways, seg_len) = case_initialize(tlh);

            for seg_num in 0..seg_len {
                let ver_align = twolanehighways.determine_vertical_alignment(seg_num);
                assert_eq!(ans1_ver_align, ver_align);
            }
        }

    }

    #[test]
    pub fn determine_free_flow_speed_test() {
        let ans1_ffs = 56.83;

        for s_file in SETTING_FILES {
            let tlh = settings(s_file);

            let (mut twolanehighways, seg_len) = case_initialize(tlh);

            for seg_num in 0..seg_len {
                let ffs = twolanehighways.determine_free_flow_speed(seg_num);
                assert_eq!(ans1_ffs, (ffs * 100.0).round() / 100.0);
            }
        }
    }

    #[test]
    pub fn estimate_average_speed_test() {
        let ans1_s = 53.7;
        let ans1_hor_class = 0;
        for s_file in SETTING_FILES {
            let tlh = settings(s_file);

            let (mut twolanehighways, seg_len) = case_initialize(tlh);


            // Set free flow speed
            for seg_num in 0..seg_len {
                let (demand_flow_i, demand_flow_o, capacity) = twolanehighways.determine_demand_flow(seg_num);
                let ffs = twolanehighways.determine_free_flow_speed(seg_num);
                let (s, hor_class) = twolanehighways.estimate_average_speed(seg_num);

                // let subseg_num = twolanehighways.get_segments()[seg_num].get_subsegments().len();
                // while j < subseg_num {
                //     tot_s += s;
                // }
                assert_eq!((ans1_s, ans1_hor_class), (math::round_to_significant_digits(s, 3), hor_class));
            }
        }
    }

    #[test]
    pub fn estimate_percent_followers_test() {
        let ans1_pf = 67.7;
        for s_file in SETTING_FILES {
            let tlh = settings(s_file);
            let (mut twolanehighways, seg_len) = case_initialize(tlh);

            for seg_num in 0..seg_len {
                let (demand_flow_i, demand_flow_o, capacity) = twolanehighways.determine_demand_flow(seg_num);
                let ffs = twolanehighways.determine_free_flow_speed(seg_num);
                let pf = twolanehighways.estimate_percent_followers(seg_num);
                assert_eq!(ans1_pf, math::round_to_significant_digits(pf, 3));
            }
        }
    }

    #[test]
    pub fn determine_follower_density_pl_test() {
        // let ans1_fd_mid = 
        for s_file in SETTING_FILES {
            let tlh = settings(s_file);

            let (mut twolanehighways, seg_num) = case_initialize(tlh);
        }
    }

    #[test]
    pub fn determine_follower_density_pc_pz_test() {
        let ans1_fd = 10.1;
        for s_file in SETTING_FILES {
            let tlh = settings(s_file);

            let (mut twolanehighways, seg_len) = case_initialize(tlh);

            for seg_num in 0..seg_len {
                let (demand_flow_i, demand_flow_o, capacity) = twolanehighways.determine_demand_flow(seg_num);
                let ffs = twolanehighways.determine_free_flow_speed(seg_num);
                let (s, hor_class) = twolanehighways.estimate_average_speed(seg_num);
                let pf = twolanehighways.estimate_percent_followers(seg_num);
                let fd = twolanehighways.determine_follower_density_pc_pz(seg_num);

                assert_eq!(ans1_fd, math::round_to_significant_digits(fd, 3));
            }
        }
    }

    #[test]
    pub fn determine_segment_los_test() {
        let ans1_los = 'D';
        for s_file in SETTING_FILES {
            let tlh = settings(s_file);

            let (mut twolanehighways, seg_len) = case_initialize(tlh);

            for seg_num in 0..seg_len {
                let (demand_flow_i, demand_flow_o, capacity) = twolanehighways.determine_demand_flow(seg_num);
                let ffs = twolanehighways.determine_free_flow_speed(seg_num);
                let (s, hor_class) = twolanehighways.estimate_average_speed(seg_num);
                let pf = twolanehighways.estimate_percent_followers(seg_num);
                let fd = twolanehighways.determine_follower_density_pc_pz(seg_num);

                assert_eq!(ans1_los, ans1_los);
            }
        }
    }
}