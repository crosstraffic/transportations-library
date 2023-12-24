pub use crate::hcm::*;


mod hcm;

#[cfg(test)]
mod test {
    use super::TwoLaneHighways;
    use super::Segment;
    use super::SubSegment;

    #[test]
    pub fn identity_vertical_class_test(){
        let subsegment = SubSegment::new(250.0, 0.5, 3.0);
        let pt = "Passing Constrained";
        let ans_min = 0.25;
        let ans_max = 3.0;
        let segment = Segment::new(
            pt,
            3000.0,
            3.0,
            true,
            1200.0,
            0.0,
            1,
            vec![subsegment],
            0.95,
            0.05,
            1
        );
        let mut twolanehighways = TwoLaneHighways {
            segments: vec![segment],
            spl: 60.0,
            lane_width: 12.0,
            shoulder_width: 3.0,
            apd: 5.0,
            pmhvfl: 0.0,
        };

        let seg_num = 0; 
        let (_min, _max) = twolanehighways.identify_vertical_class(seg_num);
        assert_eq!([ans_min, ans_max], [_min, _max]);
    }
}