pub use crate::hcm::*;


mod hcm;

#[cfg(test)]
mod twolanehighways_test {
    use super::TwoLaneHighways;
    use super::Segment;
    use super::SubSegment;

    fn case1_settings<'a>() -> (TwoLaneHighways<'a>, usize) {
        // let subsegment = SubSegment::new(250.0, 0.5, 3.0);
        let pt = "Passing Constrained";

        let segment = Segment::new(
            pt,
            3960.0,
            0.0,
            // true,
            false,
            752.0,
            0.0,
            1,
            // vec![subsegment],
            Vec::new(),
            0.94,
            0.05,
            1
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
        assert_eq!([ans1_min, ans1_max], [_min, _max]);
    }

    #[test]
    pub fn determine_demand_flow_test() {
        let ans1_demand_flow_i = 800.0;
        let ans1_demand_flow_o = 1500.0;
        let ans1_capacity = 1700.0;

        let (twolanehighways, seg_num) = case1_settings();

        let (demand_flow_i, demand_flow_o, capacity) = twolanehighways.determine_demand_flow(seg_num);
        assert_eq!([ans1_demand_flow_i, ans1_demand_flow_o, ans1_capacity], [demand_flow_i, demand_flow_o, capacity.into()]);

    }
}