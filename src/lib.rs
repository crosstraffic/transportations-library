pub use crate::hcm::*;
extern crate round;


mod hcm;

#[cfg(test)]
mod twolanehighways_test {
    use super::TwoLaneHighways;
    use super::Segment;
    use super::SubSegment;

    fn case1_settings() -> (TwoLaneHighways, usize) {
        // let subsegment = SubSegment::new(250.0, 0.5, 3.0);
        let pt = "Passing Constrained";
        let mut passing_type = 0;

        if pt == "Passing Constrained" {
            passing_type = 0;
        }

        let segment = Segment::new(
            passing_type,
            3960.0,
            0.0,
            // true,
            false,
            752.0,
            0.0,
            0.0,
            0.0,
            1,
            // vec![subsegment],
            Vec::new(),
            0.94,
            5.0,
            0,
        );
        let twolanehighways = TwoLaneHighways {
            segments: vec![segment],
            spl: 50.0,
            lane_width: 12.0,
            shoulder_width: 6.0,
            apd: 0.0,
            pmhvfl: 0.0,
        };

        let seg_num = 0; 

        (twolanehighways, seg_num)
    }

    #[test]
    pub fn identity_vertical_class_test() {
        let ans1_min = 0.25;
        let ans1_max = 3.0;

        let (mut twolanehighways, seg_num) = case1_settings();

        let (_min, _max) = twolanehighways.identify_vertical_class(seg_num);
        assert_eq!((ans1_min, ans1_max), (_min, _max));
    }

    #[test]
    pub fn determine_demand_flow_test() {
        let ans1_demand_flow_i = 800.0;
        let ans1_demand_flow_o = 1500.0;
        let ans1_capacity = 1700.0;

        let (twolanehighways, seg_num) = case1_settings();

        let (demand_flow_i, demand_flow_o, capacity) = twolanehighways.determine_demand_flow(seg_num);
        assert_eq!((ans1_demand_flow_i, ans1_demand_flow_o, ans1_capacity), (demand_flow_i, demand_flow_o, capacity.into()));

    }

    #[test]
    pub fn determine_vertical_alignment_test() {
        let ans1_ver_align = 1;

        let (twolanehighways, seg_num) = case1_settings();

        let ver_align = twolanehighways.determine_vertical_alignment(seg_num);
        assert_eq!(ans1_ver_align, ver_align);

    }

    #[test]
    pub fn determine_free_flow_speed_test() {
        let ans1_ffs = 56.83;

        let (mut twolanehighways, seg_num) = case1_settings();

        let ffs = twolanehighways.determine_free_flow_speed(seg_num);
        assert_eq!(ans1_ffs, (ffs * 100.0).round() / 100.0);
    }

    #[test]
    pub fn estimate_average_speed_teset() {
        let ans1_s = 53.7;
        let ans1_hor_class = 0;

        let (mut twolanehighways, seg_num) = case1_settings();

        // Set free flow speed
        let ffs = twolanehighways.determine_free_flow_speed(seg_num);
        let (s, hor_class) = twolanehighways.estimate_average_speed(seg_num);
        assert_eq!((ans1_s, ans1_hor_class), (s, hor_class));

    }
}