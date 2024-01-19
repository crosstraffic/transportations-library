mod utils;
mod hcm;

pub use crate::hcm::*;



#[cfg(test)]
mod twolanehighways_test {
    use serde_json::{Result, Value};
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;
    use std::path::{Path, PathBuf};
    use std::fs;
    use std::result::Result::Ok;

    use super::TwoLaneHighways;
    use super::Segment;
    use super::SubSegment;
    use super::utils::math;

    fn read_files() -> Vec<String> {

        let examples_root_dir: &str = "src/ExampleCases/hcm/TwoLaneHighways/";
        let paths = fs::read_dir(examples_root_dir).expect("Unable to read directory");
        let mut setting_files: Vec<String> = Vec::new();

        for path in paths {
            setting_files.push(path.unwrap().path().display().to_string());
        }

        setting_files
    }


    fn settings(setting_file_loc: String) -> TwoLaneHighways {
        let f = File::open(setting_file_loc).expect("Unable to open file");
        let reader = BufReader::new(f);

        let twolanehighways: TwoLaneHighways = serde_json::from_reader(reader).expect("Failed to parse JSON");

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
        let ans_min = vec![0.25, 0.25];
        let ans_max = vec![3.0, 3.0];
        let setting_files = read_files();

        for (index, s_file) in setting_files.iter().enumerate() {
            let tlh = settings(s_file.clone());

            let (mut twolanehighways, seg_len) = case_initialize(tlh);

            for seg_num in 0..seg_len {
                let (_min, _max) = twolanehighways.identify_vertical_class(seg_num);
                assert_eq!((ans_min[index], ans_max[index]), (_min, _max));
            }
        }
    }

    #[test]
    pub fn determine_demand_flow_test() {
        let ans_demand_flow_i = vec![800.0, 800.0];
        let ans_demand_flow_o = vec![1500.0, 1500.0];
        let ans_capacity = vec![1700.0, 1700.0];

        let setting_files = read_files();
        for (index, s_file) in setting_files.iter().enumerate() {
            let tlh = settings(s_file.clone());

            let (mut twolanehighways, seg_len) = case_initialize(tlh);

            for seg_num in 0..seg_len {
                let (demand_flow_i, demand_flow_o, capacity) = twolanehighways.determine_demand_flow(seg_num);
                assert_eq!((ans_demand_flow_i[index], ans_demand_flow_o[index], ans_capacity[index]), (demand_flow_i, demand_flow_o, capacity.into()));
            }
        }

    }

    #[test]
    pub fn determine_vertical_alignment_test() {
        let ans_ver_align = vec![1, 1];

        let setting_files = read_files();
        for (index, s_file) in setting_files.iter().enumerate() {
            let tlh = settings(s_file.clone());
            let (mut twolanehighways, seg_len) = case_initialize(tlh);

            for seg_num in 0..seg_len {
                let ver_align = twolanehighways.determine_vertical_alignment(seg_num);
                assert_eq!(ans_ver_align[index], ver_align);
            }
        }

    }

    #[test]
    pub fn determine_free_flow_speed_test() {
        let ans_ffs = vec![56.83, 56.83];
        let setting_files = read_files();

        for (index, s_file) in setting_files.iter().enumerate() {
            let tlh = settings(s_file.clone());

            let (mut twolanehighways, seg_len) = case_initialize(tlh);

            for seg_num in 0..seg_len {
                let ffs = twolanehighways.determine_free_flow_speed(seg_num);
                assert_eq!(ans_ffs[index], (ffs * 100.0).round() / 100.0);
            }
        }
    }

    #[test]
    pub fn estimate_average_speed_test() {
        let ans_s = vec![53.7, 49.5];
        let setting_files = read_files();
        for (index, s_file) in setting_files.iter().enumerate() {
            let tlh = settings(s_file.clone());

            let (mut twolanehighways, seg_len) = case_initialize(tlh);


            // Set free flow speed
            for seg_num in 0..seg_len {
                let (_, _, _) = twolanehighways.determine_demand_flow(seg_num);
                let _ = twolanehighways.determine_free_flow_speed(seg_num);
                let (s, _) = twolanehighways.estimate_average_speed(seg_num);

                // let subseg_num = twolanehighways.get_segments()[seg_num].get_subsegments().len();
                // while j < subseg_num {
                //     tot_s += s;
                // }
                assert_eq!(ans_s[index], math::round_to_significant_digits(s, 3));
            }
        }
    }

    #[test]
    pub fn estimate_percent_followers_test() {
        let ans_pf = vec![67.7, 67.7];
        let setting_files = read_files();
        for (index, s_file) in setting_files.iter().enumerate() {
            let tlh = settings(s_file.clone());
            let (mut twolanehighways, seg_len) = case_initialize(tlh);

            for seg_num in 0..seg_len {
                let (_, _, _) = twolanehighways.determine_demand_flow(seg_num);
                let _ = twolanehighways.determine_free_flow_speed(seg_num);
                let pf = twolanehighways.estimate_percent_followers(seg_num);
                assert_eq!(ans_pf[index], math::round_to_significant_digits(pf, 3));
            }
        }
    }

    #[test]
    pub fn determine_follower_density_pl_test() {
        let setting_files = read_files();
        for (index, s_file) in setting_files.iter().enumerate() {
            let tlh = settings(s_file.clone());

            let (mut twolanehighways, seg_num) = case_initialize(tlh);
        }
    }

    #[test]
    pub fn determine_follower_density_pc_pz_test() {
        let ans1_fd = vec![10.1, 10.9];
        let setting_files = read_files();
        for (index, s_file) in setting_files.iter().enumerate() {
            let tlh = settings(s_file.clone());

            let (mut twolanehighways, seg_len) = case_initialize(tlh);

            for seg_num in 0..seg_len {
                let (_, _, _) = twolanehighways.determine_demand_flow(seg_num);
                let _ = twolanehighways.determine_free_flow_speed(seg_num);
                let (_, _) = twolanehighways.estimate_average_speed(seg_num);
                let _ = twolanehighways.estimate_percent_followers(seg_num);
                let fd = twolanehighways.determine_follower_density_pc_pz(seg_num);

                assert_eq!(ans1_fd[index], math::round_to_significant_digits(fd, 3));
            }
        }
    }

    #[test]
    pub fn determine_segment_los_test() {
        let ans1_los = vec!['D', 'D'];
        let setting_files = read_files();
        for (index, s_file) in setting_files.iter().enumerate() {
            let tlh = settings(s_file.clone());

            let (mut twolanehighways, seg_len) = case_initialize(tlh);

            for seg_num in 0..seg_len {
                let (demand_flow_i, demand_flow_o, capacity) = twolanehighways.determine_demand_flow(seg_num);
                let ffs = twolanehighways.determine_free_flow_speed(seg_num);
                let (s, hor_class) = twolanehighways.estimate_average_speed(seg_num);
                let pf = twolanehighways.estimate_percent_followers(seg_num);
                let fd = twolanehighways.determine_follower_density_pc_pz(seg_num);
                let los = twolanehighways.determine_segment_los(seg_num, fd, s, capacity);

                assert_eq!(ans1_los[index], los);
            }
        }
    }
}