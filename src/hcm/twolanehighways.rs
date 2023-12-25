#[derive(Debug, Clone)]
pub struct SubSegment {
    /// Length of subsegment, ft.
    pub length: f64,
    /// Design radius of subsegment, ft.
    pub design_rad: f64,
    /// Superelevation of subsegment, %.
    pub sup_ele: f64
}

#[derive(Debug, Clone)]
pub struct Segment <'a>{
    /// Passing Type.
    pub passing_type: &'a str,
    /// Length of segment, mi.
    pub length: f64,
    /// Segment percent grade.
    pub grade: f64,
    /// Whether the segment has horizontal class or not.
    pub is_hc: bool,
    /// Demand volume for direction i, veh/hr.
    pub volume: f64,
    /// Demand volume for opposite direction o, veh/hr. Required for PZ segments.
    /// 1500 veh/hr for PC segments and 0 for PL segments.
    pub volume_op: f64,
    /// Vertical class of the segment.
    pub vertical_class: i32,
    pub subsegments: Vec<SubSegment>,
    /// Peak hour factor, unitless.
    pub phf : f64,
    /// Percentage of heavy vehicles, unitless
    pub phv: f64,
    // /// Horizontal class of the segment.
    pub hor_class: i32,
}

/// Two Lane Highways on chapter 15 of HCM.
#[derive(Debug, Clone)]
pub struct TwoLaneHighways<'a> {
    pub segments: Vec<Segment<'a>>,
    /// Posted speed limit, mi/hr.
    pub spl: f64,
    /// Lane width, ft.
    pub lane_width: f64,
    /// Shoulder width, ft.
    pub shoulder_width: f64,
    /// Access point density (access points/mi).
    /// https://highways.dot.gov/safety/other/access-management-driveways
    pub apd: f64,
    // /// Percentage multiplier for heavy vehicles in the faster / passing lane
    pub pmhvfl: f64,
}

/// Implement methods for SubSegment
impl SubSegment {
    /// Method to create a new SubSegment instance
    pub fn new(length: f64, design_rad: f64, sup_ele: f64) -> SubSegment {
        SubSegment {
            length,
            design_rad,
            sup_ele,
        }
    }

    /// Method to get the length of the SubSegment
    fn get_length(&self) -> f64 {
        self.length
    }

    fn get_design_rad(&self) -> f64 {
        self.design_rad
    }

    fn get_sup_ele(&self) -> f64 {
        self.sup_ele
    }
}


/// Implement methods for Segment
impl Segment<'_> {
    /// Method to create a new Segment instance
    pub fn new(passing_type: &str, length: f64, grade: f64, is_hc: bool, volume: f64,
            volume_op: f64, vertical_class: i32, subsegments: Vec<SubSegment>, phf: f64, phv: f64, hor_class: i32) -> Segment {
        Segment {
            passing_type,
            length,
            grade,
            is_hc,
            volume,
            volume_op,
            vertical_class,
            subsegments,
            phf,
            phv,
            hor_class,
        }
    }

    /// Get passing type
    fn get_passing_type<'a>(&'a self) -> &'a str {
        return &self.passing_type
    }

    /// Get total length
    /// Need to check segment length is equal to the total length of subsegments
    fn get_length(&self) -> f64 {
        return self.length
        // TODO
    }

    fn get_grade(&self) -> f64 {
        return self.grade
    }

    fn get_is_hc(&self) -> bool {
        return self.is_hc
    }

    fn get_volume(&self) -> f64 {
        return self.volume
    }

    fn get_volume_op(&self) -> f64 {
        return self.volume_op
    }

    fn get_vertical_class(&self) -> i32 {
        return self.vertical_class
    }

    fn get_subsegments(&self) -> &Vec<SubSegment> {
        return &self.subsegments
    }

    fn get_phf(&self) -> f64 {
        return self.phf
    }

    fn get_phv(&self) -> f64 {
        return self.phv
    }

    fn get_hor_class(&self) -> i32 {
        return self.hor_class
    }
}


impl TwoLaneHighways<'_> {

    /// Returns a segment LOS and LOS
    /// 
    /// # Arguments
    /// 
    /// * `segment number` - the number of segments
    /// 

    // pub fn new(vi: f64, vo: f64, phf: f64, phv: f64, spl: f64, seg_length: f64, seg_grade: i32, 
    //     lw: f64, sw: f64, apd: f64, is_hc: bool, pmhvfl: f64, hor_class: i32, vc: i32, pt: &str, sub_params: Vec<(f64, f64, f64)>) -> Self {
    // pub fn new(seg: &Segment, subseg: &SubSegment) -> Self {
    fn new(segments: Vec<Segment<'_>>, spl: f64, lane_width: f64, shoulder_width: f64, apd: f64, pmhvfl: f64) -> TwoLaneHighways {

        // let subseg: Vec<SubSegment> = sub_params
        //     .into_iter()
        //     .map(|(subseg_length, design_rad, super_ele)| SubSegment {
        //         subseg_length: subseg_length, design_rad: design_rad, sup_ele: super_ele
        //     })
        //     .collect();

        TwoLaneHighways {
            segments: segments,
            spl: spl,
            lane_width: lane_width,
            shoulder_width: shoulder_width,
            apd: apd,
            pmhvfl: pmhvfl,
        }
    }

    fn horizontal_class(&self) -> i32 {1}

    /// Step 1: Identify vertical class
    pub fn identify_vertical_class(&mut self, seg_num: usize) -> (f64, f64) {
        let mut _min = 0.0;
        let mut _max = 0.0;
        let vc = self.segments[seg_num].get_vertical_class();
        let pt = self.segments[seg_num].get_passing_type();
        if (vc == 1) || (vc == 2){
            if pt == "Passing Constrained" {
                _min = 0.25;
                _max = 3.0;
            } else if pt == "Passing Zone" {
                _min = 0.25;
                _max = 2.0;
            } else if pt == "Passing Lane" {
                _min = 0.5;
                _max = 3.0;
            }
        } else if vc == 3 {
            if pt == "Passing Constrained" {
                _min = 0.25;
                _max = 1.1;
            } else if pt == "Passing Zone" {
                _min = 0.25;
                _max = 1.1;
            } else if pt == "Passing Lane" {
                _min = 0.5;
                _max = 1.1;
            }
        } else if (vc == 4) && (vc == 5) {
            if pt == "Passing Constrained" {
                _min = 0.5;
                _max = 3.0;
            } else if pt == "Passing Zone" {
                _min = 0.5;
                _max = 2.0;
            } else if pt == "Passing Lane" {
                _min = 0.5;
                _max = 3.0;
            }
        };
        (_min, _max)
    }


    /// Step 2: Determine demand flow rates and capacity
    pub fn determine_demand_flow(&self, seg_num: usize) -> (f64, f64, i32) {

        let v_i = self.segments[seg_num].get_volume();
        let v_o = self.segments[seg_num].get_volume_op();
        let phf = self.segments[seg_num].get_phf();
        let phv = self.segments[seg_num].get_phv();
        let pt = self.segments[seg_num].get_passing_type();
        let vc = self.segments[seg_num].get_vertical_class();

        let demand_flow_i = v_i / phf;
        let mut demand_flow_o = 0.0;
        let mut capacity = 0;

        if (pt == "Passing Zone") && (v_o == 0.0) {
            capacity = 1700;
        } else if pt == "Passing Zone" {
            demand_flow_o = v_o / phf;
            capacity = 1700;
        } else if pt == "Passing Constrained" {
            demand_flow_o = 1500.0;
            capacity = 1700;
        } else if pt == "Passing Lane" {
            if phv < 5.0 {
                capacity = 1500;
            } else if phv >= 5.0 && phv < 10.0 {
                if vc == 1 || vc == 2 || vc == 3 {
                    capacity = 1500;
                } else {
                    capacity = 1500;
                }
            } else if phv >= 10.0 && phv < 15.0 {
                if vc == 1 || vc == 2 || vc == 3 {
                    capacity = 1400;
                } else {
                    capacity = 1300;
                }
            } else if phv >= 15.0 && phv < 20.0 {
                if vc == 1 || vc == 2 || vc == 3 || vc == 4 {
                capacity = 1300;
                } else {
                capacity = 1200;
                }
            } else if phv >= 20.0 && phv < 25.0 {
                if vc == 1 || vc == 2 || vc == 3 {
                    capacity = 1300;
                } else if vc == 4 {
                    capacity = 1200;
                } else {
                    capacity = 1100;
                }
            } else if phv >= 25.0 {
                capacity = 1100;
            }
        }

        (demand_flow_i, demand_flow_o, capacity)


    }


    // /// Step 3: Determine vertical alignment classification
    // pub fn determine_vertical_alignment(&self) -> i32 {
    //     let mut seg_length = self.seg_length;
    //     let mut seg_grade = self.seg_grade;

    //     let mut ver_align = 0;

    //     if seg_grade >= 0 {
    //         if seg_length <= 0.1 {
    //             if seg_grade <= 7 { ver_align = 1 } else { ver_align = 2 };
    //         } else if seg_length > 0.1 && seg_length <= 0.2 {
    //             if seg_grade <= 4 { ver_align = 1 } else if seg_grade <= 7 { ver_align = 2 } else { ver_align = 3};
    //         } else if seg_length > 0.2 && seg_length <= 0.3 {
    //             if seg_grade <= 3 { ver_align = 1 }
    //             else if seg_grade <= 5 { ver_align = 2 }
    //             else if seg_grade <= 7 { ver_align = 3 }
    //             else if seg_grade <= 9 { ver_align = 4 }
    //             else { ver_align = 5 };
    //         } else if seg_length > 0.3 && seg_length <= 0.4 {
    //             if seg_grade <= 2 { ver_align = 1 }
    //             else if seg_grade <= 4 { ver_align = 2 }
    //             else if seg_grade <= 6 { ver_align = 3 }
    //             else if seg_grade <= 7 { ver_align = 4 }
    //             else { ver_align = 5 };
    //         } else if seg_length > 0.4 && seg_length <= 0.5 {
    //             if seg_grade <= 2 { ver_align = 1 }
    //             else if seg_grade <= 4 { ver_align = 2 }
    //             else if seg_grade <= 3 { ver_align = 2 }
    //             else if seg_grade <= 5 { ver_align = 3 }
    //             else if seg_grade <= 6 { ver_align = 4 }
    //             else { ver_align = 5 };
    //         } else if seg_length > 0.6 && seg_length <= 0.7 {
    //             if seg_grade <= 2 { ver_align = 1 }
    //             else if seg_grade <= 3 { ver_align = 2 }
    //             else if seg_grade <= 4 { ver_align = 3 }
    //             else if seg_grade <= 6 { ver_align = 4 }
    //             else { ver_align = 5 };
    //         } else if seg_length > 0.8 && seg_length <= 1.1 {
    //             if seg_grade <= 2 { ver_align = 1 }
    //             else if seg_grade <= 3 { ver_align = 2 }
    //             else if seg_grade <= 4 { ver_align = 3 }
    //             else if seg_grade <= 5 { ver_align = 4 }
    //             else { ver_align = 5 };
    //         } else {
    //             if seg_grade <= 2 { ver_align = 1 }
    //             else if seg_grade <= 3 { ver_align = 2 }
    //             else if seg_grade <= 5 { ver_align = 4 }
    //             else { ver_align = 5 };
    //         }
    //     } else {
    //         seg_length = -1.0 * seg_length;
    //         if seg_length <= 0.1 {
    //             if seg_grade <= 8 { ver_align = 1 }
    //             else { ver_align = 2 };
    //         } else if seg_length > 0.1 && seg_length <= 0.2 {
    //             if seg_grade <= 5 { ver_align = 1 }
    //             else if seg_grade <= 8 { ver_align = 2 }
    //             else { ver_align = 3 };
    //         } else if seg_length > 0.2 && seg_length <= 0.3 {
    //             if seg_grade <= 4 { ver_align = 1 }
    //             else if seg_grade <= 6 { ver_align = 2 }
    //             else if seg_grade <= 8 { ver_align = 3 }
    //             else if seg_grade <= 9 { ver_align = 4 }
    //             else { ver_align = 5 };
    //         } else if seg_length > 0.3 && seg_length <= 0.4 {
    //             if seg_grade <= 2 { ver_align = 1 }
    //             else if seg_grade <= 5 { ver_align = 2 }
    //             else if seg_grade <= 6 { ver_align = 3 }
    //             else if seg_grade <= 8 { ver_align = 4 }
    //             else { ver_align = 5 };
    //         } else if seg_length > 0.4 && seg_length <= 0.5 {
    //             if seg_grade <= 3 { ver_align = 1 }
    //             else if seg_grade <= 4 { ver_align = 2 }
    //             else if seg_grade <= 6 { ver_align = 3 }
    //             else if seg_grade <= 7 { ver_align = 4 }
    //             else { ver_align = 5 };
    //         } else if seg_length > 0.5 && seg_length <= 0.7 {
    //             if seg_grade <= 3 { ver_align = 1 }
    //             else if seg_grade <= 4 { ver_align = 2 }
    //             else if seg_grade <= 5 { ver_align = 3 }
    //             else if seg_grade <= 6 { ver_align = 4 }
    //             else { ver_align = 5 };
    //         } else if seg_length > 0.7 && seg_length <= 0.8 {
    //             if seg_grade <= 3 { ver_align = 1 }
    //             else if seg_grade <= 4 { ver_align = 3 }
    //             else if seg_grade <= 6 { ver_align = 4 }
    //             else { ver_align = 5 };
    //         } else if seg_length > 0.8 && seg_length <= 0.9 {
    //             if seg_grade <= 3 { ver_align = 1 }
    //             else if seg_grade <= 4 { ver_align = 3 }
    //             else if seg_grade <= 5 { ver_align = 4 }
    //             else { ver_align = 5 };
    //         } else if seg_length > 0.9 && seg_length <= 1.1 {
    //             if seg_grade <= 2 { ver_align = 1 }
    //             else if seg_grade <= 3 { ver_align = 2 }
    //             else if seg_grade <= 4 { ver_align = 3 }
    //             else if seg_grade <= 5 { ver_align = 4 }
    //             else { ver_align = 5 };
    //         } else {
    //             if seg_grade <= 2 { ver_align = 1 }
    //             else if seg_grade <= 3 { ver_align = 2 }
    //             else if seg_grade <= 5 { ver_align = 4 }
    //             else { ver_align = 5 };
    //         }
    //     }

    //     ver_align
    // }

    // /// Step 4: Determine free-flow speed
    // pub fn determine_free_flow_speed(&self, seg_length: f64) -> f64 {

    //     let mut spl = self.spl;
    //     let mut vc = self.vertical_class;
    //     let mut vo = self.volume_op;
    //     let mut lw = self.lane_width;
    //     let mut sw = self.shoulder_width;
    //     let apd = self.apd;
    //     let phv = self.phv;

    //     let mut ffs: f64;

    //     let bffs = 1.14 * spl;
    //     let a0: f64;
    //     let a1: f64;
    //     let a2: f64;
    //     let a3: f64;
    //     let a4: f64;
    //     let a5: f64;

    //     if vc == 1 {
    //         a0 = 0.0;
    //         a1 = 0.0;
    //         a2 = 0.0;
    //         a3 = 0.0;
    //         a4 = 0.0;
    //         a5 = 0.0;
    //     } else if vc == 2 {
    //         a0 = -0.45036;
    //         a1 = 0.00814;
    //         a2 = 0.01543;
    //         a3 = 0.01358;
    //         a4 = 0.0;
    //         a5 = 0.0;
    //     } else if vc == 3 {
    //         a0 = -0.29591;
    //         a1 = 0.00743;
    //         a2 = 0.0;
    //         a3 = 0.01246;
    //         a4 = 0.0;
    //         a5 = 0.0;
    //     } else if vc == 4 {
    //         a0 = -0.40902;
    //         a1 = 0.00975;
    //         a2 = 0.00767;
    //         a3 = -0.18363;
    //         a4 = 0.00423;
    //         a5 = 0.0;
    //     } else if vc == 5 {
    //         a0 = -0.3836;
    //         a1 = 0.01074;
    //         a2 = 0.01945;
    //         a3 = -0.69848;
    //         a4 = 0.01069;
    //         a5 = 0.127;
    //     }

    //     let a = f64::max(
    //         0.0333,
    //         a0 + a1 * bffs + a2 * seg_length + (f64::max(0.0, a3 + a4 * bffs + a5 * seg_length) * vo) / 1000.0,
    //     );

    //     // adjustment for lane and shoulder width, mi/hr
    //     let f_ls = 0.6 * (12.0 - lw) + 0.7 * (6.0 - sw);
    //     // adjustment for access point density, mi/hr
    //     let f_a = f64::min(apd / 4.0, 10.0);

    //     ffs = bffs - a * phv - f_ls - f_a;


    //     ffs

    // }

    // /// Step 5: Estimate average speed
    // pub fn estimate_average_speed(&self, vd: f64, vc: i32, ffs: f64, seg_length: f64, rad: f64, sup_ele: f64) -> (f64, i32) {
    //     let mut bffs = 1.14 * self.spl;
    //     let (mut b0, mut b1, mut b2, mut b3, mut b4, mut b5) = (0.0000, 0.0000, 0.0000, 0.0000, 0.0000, 0.0000);
    //     let (mut c0, mut c1, mut c2, mut c3) = (0.0000, 0.0000, 0.0000, 0.0000);
    //     let (mut d0, mut d1, mut d2, mut d3) = (0.0000, 0.0000, 0.0000, 0.0000);
    //     let (mut f0, mut f1, mut f2, mut f3, mut f4, mut f5, mut f6, mut f7, mut f8) = (0.0000, 0.0000, 0.0000, 0.0000, 0.0000, 0.0000, 0.0000, 0.0000, 0.0000);
    //     let s: f64; // average speed
    //     let mut hor_class: i32;
    //     let mut pt = self.passing_type;
    //     let phv = self.phv;
    //     let mut vo = self.volume_op;

    //     if pt == "Passing Constrained" || pt == "Passing Zone" {
    //         if vc == 1 {
    //             b0 = 0.0558;
    //             b1 = 0.0542;
    //             b2 = 0.3278;
    //             b3 = 0.1029;
    //             f0 = 0.67576;
    //             f3 = 0.12060;
    //             f4 = -0.35919;
    //         } else if vc == 2 {
    //             b0 = 5.7280;
    //             b1 = -0.0809;
    //             b2 = 0.7404;
    //             b5 = 3.1155;
    //             c0 = -13.8036;
    //             c2 = 0.2446;
    //             d0 = -1.7765;
    //             d2 = 0.0392;
    //             b3 = c0 + c1 * f64::sqrt(seg_length) + c2 * ffs + c3 * ffs * f64::sqrt(seg_length);
    //             b4 = d0 + d1 * f64::sqrt(phv) + d2 * ffs + d3 * ffs * f64::sqrt(phf);
    //             f0 = 0.34524;
    //             f1 = 0.00591;
    //             f2 = 0.02031;
    //             f3 = 0.14911;
    //             f4 = -0.43784;
    //             f5 = -0.00296;
    //             f6 = 0.02956;
    //             f8 = 0.41622;
    //         } else if vc == 3 {
    //             b0 = 9.3079;
    //             b1 = -0.1706;
    //             b2 = 1.1292;
    //             b5 = 3.1155;
    //             c0 = -11.9703;
    //             c2 = 0.2542;
    //             d0 = -3.5550;
    //             d2 = 0.0826;
    //             b3 = c0 + c1 * f64::sqrt(seg_length) + c2 * ffs + c3 * ffs * f64::sqrt(seg_length);
    //             b4 = d0 + d1 * f64::sqrt(phv) + d2 * ffs + d3 * ffs * f64::sqrt(phv);
    //             f0 = 0.17291;
    //             f1 = 0.00917;
    //             f2 = 0.05698;
    //             f3 = 0.27734;
    //             f4 = -0.61893;
    //             f5 = -0.00918;
    //             f6 = 0.09184;
    //             f8 = 0.41622;
    //         } else if vc == 4 {
    //             b0 = 9.0115;
    //             b1 = -0.1994;
    //             b2 = 1.8252;
    //             b5 = 3.2685;
    //             c0 = -12.5113;
    //             c2 = 0.2656;
    //             d0 = -5.7775;
    //             d2 = 0.1373;
    //             b3 = c0 + c1 * f64::sqrt(seg_length) + c2 * ffs + c3 * ffs * f64::sqrt(seg_length);
    //             b4 = d0 + d1 * f64::sqrt(phv) + d2 * ffs + d3 * ffs * f64::sqrt(phv);
    //             f0 = 0.67689;
    //             f1 = 0.00534;
    //             f2 = -0.13037;
    //             f3 = 0.25699;
    //             f4 = -0.68465;
    //             f5 = -0.00709;
    //             f6 = 0.07087;
    //             f8 = 0.33950;
    //         } else if vc == 5 {
    //             b0 = 23.9144;
    //             b1 = -0.6925;
    //             b2 = 1.9473;
    //             b5 = 3.5115;
    //             c0 = -14.8961;
    //             c2 = 0.4370;
    //             d0 = -18.2910;
    //             d1 = 2.3875;
    //             d2 = 0.4494;
    //             d3 = -0.0520;
    //             b3 = c0 + c1 * f64::sqrt(seg_length) + c2 * ffs + c3 * ffs * f64::sqrt(seg_length);
    //             b4 = d0 + d1 * f64::sqrt(phv) + d2 * ffs + d3 * ffs * f64::sqrt(phv);
    //             f0 = 1.13262;
    //             f2 = -0.26367;
    //             f3 = 0.18811;
    //             f4 = -0.64304;
    //             f5 = -0.00867;
    //             f6 = 0.08675;
    //             f8 = 0.30590;
    //         }
    //     } else if pt == "Passing Lane" {
    //         if vc == 1 {
    //             b0 = -1.1379;
    //             b1 = 0.0941;
    //             c1 = 0.2667;
    //             d1 = 0.1252;
    //             b3 = c0 + c1 * f64::sqrt(seg_length) + c2 * ffs + c3 * ffs * f64::sqrt(seg_length);
    //             b4 = d0 + d1 * f64::sqrt(phv) + d2 * ffs + d3 * ffs * f64::sqrt(phv);
    //             f0 = 0.91793;
    //             f1 = -0.00557;
    //             f2 = 0.36862;
    //             f5 = 0.00611;
    //             f7 = -0.00419;
    //         } else if vc == 2 {
    //             b0 = -2.0668;
    //             b1 = 0.1053;
    //             c1 = 0.4479;
    //             d1 = 0.1631;
    //             b3 = c0 + c1 * f64::sqrt(seg_length) + c2 * ffs + c3 * ffs * f64::sqrt(seg_length);
    //             b4 = d0 + d1 * f64::sqrt(phv) + d2 * ffs + d3 * ffs * f64::sqrt(phv);
    //             f0 = 0.65105;
    //             f2 = 0.34931;
    //             f5 = 0.00722;
    //             f7 = -0.00391;
    //         } else if vc == 3 {
    //             b0 = -0.5074;
    //             b1 = 0.0935;
    //             d1 = -0.2201;
    //             d3 = 0.0072;
    //             b4 = d0 + d1 * f64::sqrt(phv) + d2 * ffs + d3 * ffs * f64::sqrt(phv);
    //             f0 = 0.40117;
    //             f2 = 0.68633;
    //             f5 = 0.02350;
    //             f7 = -0.02088;
    //         } else if vc == 4 {
    //             b0 = 8.0354;
    //             b1 = -0.0860;
    //             b5 = 4.1900;
    //             c0 = -27.1244;
    //             c1 = 11.5196;
    //             c2 = 0.4681;
    //             c3 = -0.1873;
    //             d1 = -0.7506;
    //             d3 = 0.0193;
    //             b3 = c0 + c1 * f64::sqrt(seg_length) + c2 * ffs + c3 * ffs * f64::sqrt(seg_length);
    //             b4 = d0 + d1 * f64::sqrt(phv) + d2 * ffs + d3 * ffs * f64::sqrt(phv);
    //             f0 = 1.13282;
    //             f1 = -0.00798;
    //             f2 = 0.35425;
    //             f5 = 0.01521;
    //             f7 = -0.00987;
    //         } else if vc == 5 {
    //             b0 = 7.2991;
    //             b1 = -0.3535;
    //             b5 = 4.8700;
    //             c0 = -45.3391;
    //             c1 = 17.3749;
    //             c2 = 1.0587;
    //             c3 = -0.3729;
    //             d0 = 3.8457;
    //             d1 = -0.9112;
    //             d3 = 0.0170;
    //             b3 = c0 + c1 * f64::sqrt(seg_length) + c2 * ffs + c3 * ffs * f64::sqrt(seg_length);
    //             b4 = d0 + d1 * f64::sqrt(phv) + d2 * ffs + d3 * ffs * f64::sqrt(phv);
    //             f0 = 1.12077;
    //             f1 = -0.00550;
    //             f2 = 0.25431;
    //             f5 = 0.01269;
    //             f7 = -0.01053;
    //         }
    //     }
    //     // slope coefficient for average speed calculation
    //     let mut ms = f64::max(
    //     b5,
    //     b0 +
    //         b1 * ffs +
    //         b2 * vo / 1000.0 +
    //         f64::max(0.0, b3) * f64::sqrt(seg_length) +
    //         f64::max(0.0, b4) * f64::sqrt(phv),
    //     );
    
    //     // power coefficient for average speed calculation
    //     let mut ps = f64::max(
    //     f8,
    //     f0 +
    //         f1 * ffs +
    //         f2 * seg_length +
    //         (f3 * vo) / 1000.0 +
    //         f4 * f64::sqrt(vo / 1000.0) +
    //         f5 * phv +
    //         f6 * f64::sqrt(phv) +
    //         f7 * seg_length * phv,
    //     );
    //     // Length of horizontal curves = radius x central angle x pi/180
    //     // determine horizontal class
    //     if rad == 0.0 { 
    //         hor_class = 0;
    //     } else if rad > 0.0 && rad < 300.0 {
    //         hor_class = 5;
    //     } else if rad >= 300.0 && rad < 450.0 {
    //         hor_class = 4;
    //     } else if rad >= 450.0 && rad < 600.0 {
    //         if sup_ele < 1.0 { hor_class = 4 } else { hor_class = 3 };
    //     } else if rad >= 600.0 && rad < 750.0 {
    //         if sup_ele < 6.0 { hor_class = 3 } else { hor_class = 2 };
    //     } else if rad >= 750.0 && rad < 900.0 {
    //         hor_class = 2;
    //     } else if rad >= 900.0 && rad < 1050.0 {
    //         if sup_ele < 8.0 { hor_class = 2 } else { hor_class = 1 };
    //     } else if rad >= 1050.0 && rad < 1200.0 {
    //         if sup_ele < 4.0 { hor_class = 2 } else { hor_class = 1 };
    //     } else if rad >= 1200.0 && rad < 1350.0 {
    //         if sup_ele < 2.0 { hor_class = 2 } else { hor_class = 1 };
    //     } else if rad >= 1350.0 && rad < 1500.0 {
    //         hor_class = 1;
    //     } else if rad >= 1500.0 && rad < 1750.0 {
    //         if sup_ele < 8.0 { hor_class = 1 } else { hor_class = 0 };
    //     } else if rad >= 1750.0 && rad < 1800.0 {
    //         if sup_ele < 6.0 { hor_class = 1 } else { hor_class = 0 };
    //     } else if rad >= 1800.0 && rad < 1950.0 {
    //         if sup_ele < 5.0 { hor_class = 1 } else { hor_class = 0 };
    //     } else if rad >= 1950.0 && rad < 2100.0 {
    //         if sup_ele < 4.0 { hor_class = 1 } else { hor_class = 0 };
    //     } else if rad >= 2100.0 && rad < 2250.0 {
    //         if sup_ele < 3.0 { hor_class = 1 } else { hor_class = 0 };
    //     } else if rad >= 2250.0 && rad < 2400.0 {
    //         if sup_ele < 2.0 { hor_class = 1 } else { hor_class = 0 };
    //     } else if rad >= 2400.0 && rad < 2550.0 {
    //         if sup_ele < 1.0 { hor_class = 1 } else { hor_class = 0 };
    //     } else if rad >= 2550.0 {
    //         hor_class = 0;
    //     }

    //     if vd <= 100.0 {
    //         let mut st = ffs;
    //         s = st;
    //     } else {
    //         let mut st = ffs - ms * f64::powf(vd / 1000.0 - 0.1, ps);
    //         s = st;
    //     }

    //     if hor_class == 0 { self.is_hc = false }; // treat curve as tanget section

    //     if self.is_hc == true {
    //         // calculate horizontal class
    //         let mut bffshc = f64::min(bffs, 44.32 + 0.3728 * bffs - 6.868 * hor_class as f64);
    //         let mut ffshc = bffshc - 0.0255 * phv;
    //         let mut mhc = f64::max(0.277, -25.8993 - 0.7756 * ffshc + 10.6294 * f64::sqrt(ffshc) + 2.4766 * hor_class as f64 - 9.8238 * f64::sqrt(hor_class as f64));
    //         let mut shc = f64::min(s, ffshc - mhc * f64::sqrt(vd / 1000.0 - 0.1)); // Should be ST instead of S?
    //         s = shc;
    //     }

    //     (s, hor_class)
    // }

    // /// Step 6: Estimate percent followers
    // pub fn estimate_percent_followers(&self, vc: i32, seg_length: f64, ffs: f64, cap: i32) -> f64 {
    //     let (mut b0, mut b1, mut b2, mut b3, mut b4, mut b5, mut b6, mut b7) = (0.000000, 0.000000, 0.000000, 0.000000, 0.000000, 0.000000, 0.000000, 0.000000);
    //     let (mut c0, mut c1, mut c2, mut c3, mut c4, mut c5, mut c6, mut c7) = (0.000000, 0.000000, 0.000000, 0.000000, 0.000000, 0.000000, 0.000000, 0.000000);
    //     let (mut d0, mut d1, mut d2) = (0.000000, 0.000000, 0.000000);
    //     let (mut e0, mut e1, mut e2, mut e3, mut e4) = (0.000000, 0.000000, 0.000000, 0.000000, 0.000000);

    //     // Percent followers at capacity
    //     let mut pf_cap = 0.0;
    //     let mut pf_25_cap = 0.0;
        
    //     let pt = self.passing_type;
    //     let vd = self.volume;
    //     let vo = self.volume_op;
    //     let phv = self.phv;

    //     if pt == "Passing Constrained" || pt == "Passing Zone" {
    //         if vc == 1 {
    //             b0 = 37.68080;
    //             b1 = 3.05089;
    //             b2 = -7.90866;
    //             b3 = -0.94321;
    //             b4 = 13.64266;
    //             b5 = -0.00050;
    //             b6 = -0.05500;
    //             b7 = 7.13758;
    //             c0 = 18.01780;
    //             c1 = 10.00000;
    //             c2 = -21.60000;
    //             c3 = -0.97853;
    //             c4 = 12.05214;
    //             c5 = -0.00750;
    //             c6 = -0.06700;
    //             c7 = 11.60405;
    //         } else if vc == 2 {
    //             b0 = 58.21104;
    //             b1 = 5.73387;
    //             b2 = -13.66293;
    //             b3 = -0.66126;
    //             b4 = 9.08575;
    //             b5 = -0.00950;
    //             b6 = -0.03602;
    //             b7 = 7.14619;
    //             c0 = 47.83887;
    //             c1 = 12.80000;
    //             c2 = -28.20000;
    //             c3 = -0.61758;
    //             c4 = 5.80000;
    //             c5 = -0.04550;
    //             c6 = -0.03344;
    //             c7 = 11.35573;
    //         } else if vc == 3 {
    //             b0 = 113.20439;
    //             b1 = 10.01778;
    //             b2 = -18.90000;
    //             b3 = 0.46542;
    //             b4 = -6.75338;
    //             b5 = -0.03000;
    //             b6 = -0.05800;
    //             b7 = 10.03239;
    //             c0 = 125.40000;
    //             c1 = 19.50000;
    //             c2 = -34.90000;
    //             c3 = 0.90672;
    //             c4 = -16.10000;
    //             c5 = -0.11000;
    //             c6 = -0.06200;
    //             c7 = 14.71136;
    //         } else if vc == 4 {
    //             b0 = 58.29978;
    //             b1 = -0.53611;
    //             b2 = 7.35076;
    //             b3 = -0.27046;
    //             b4 = 4.49850;
    //             b5 = -0.01100;
    //             b6 = -0.02968;
    //             b7 = 8.89680;
    //             c0 = 103.13534;
    //             c1 = 14.68459;
    //             c2 = -23.72704;
    //             c3 = 0.66444;
    //             c4 = -11.95763;
    //             c5 = -0.10000;
    //             c6 = 0.00172;
    //             c7 = 14.56611;
    //         } else if vc == 5 {
    //             b0 = 3.32968;
    //             b1 = -0.84377;
    //             b2 = 7.08952;
    //             b3 = -1.32089;
    //             b4 = 19.98477;
    //             b5 = -0.01250;
    //             b6 = -0.02960;
    //             b7 = 9.99453;
    //             c0 = 89.00000;
    //             c1 = 19.02642;
    //             c2 = -34.54240;
    //             c3 = 0.29792;
    //             c4 = -6.62528;
    //             c5 = -0.16000;
    //             c6 = 0.00480;
    //             c7 = 17.56611;
    //         }
    //         d1 = -0.29764;
    //         d2 = -0.71917;
    //         e0 = 0.81165;
    //         e1 = 0.37920;
    //         e2 = -0.49524;
    //         e3 = -2.11289;
    //         e4 = 2.41146;

    //         pf_cap = b0 + b1 * seg_length + b2 * f64::sqrt(seg_length) + b3 * ffs + b4 * f64::sqrt(ffs) + b5 * phv + b6 * ffs * vo / 1000.0 + b7 * f64::sqrt(vo/1000.0);
    //         pf_25_cap = c0 + c1 * seg_length + c2 * f64::sqrt(seg_length) + c3 * ffs + c4 * f64::sqrt(ffs) + c5 * phv + c6 * ffs * vo / 1000.0 + c7 * f64::sqrt(vo/1000.0);
    //     } else if pt == "Passing Lane" {
    //         if vc == 1 {
    //             b0 = 61.73075;
    //             b1 = 6.73922;
    //             b2 = -23.68853;
    //             b3 = -0.84126;
    //             b4 = 11.44533;
    //             b5 = -1.05124;
    //             b6 = 1.50390;
    //             b7 = 0.00491;
    //             c0 = 80.37105;
    //             c1 = 14.44997;
    //             c2 = -46.41831;
    //             c3 = -0.23367;
    //             c4 = 0.84914;
    //             c5 = -0.56747;
    //             c6 = 0.89427;
    //             c7 = 0.00119;
    //         } else if vc == 2 {
    //             b0 = 12.30096;
    //             b1 = 9.57465;
    //             b2 = -30.79427;
    //             b3 = -1.79448;
    //             b4 = 25.76436;
    //             b5 = -0.66350;
    //             b6 = 1.26039;
    //             b7 = -0.00323;
    //             c0 = 18.37886;
    //             c1 = 14.71856;
    //             c2 = -47.78892;
    //             c3 = -1.43373;
    //             c4 = 18.32040;
    //             c5 = -0.13226;
    //             c6 = 0.77127;
    //             c7 = -0.00778;
    //         } else if vc == 3 {
    //             b0 = 206.07369;
    //             b1 = -4.29885;
    //             b2 = 0.00000;
    //             b3 = 1.96483;
    //             b4 = -30.32556;
    //             b5 = -0.75812;
    //             b6 = 1.06453;
    //             b7 = -0.00839;
    //             c0 = 239.98930;
    //             c1 = 15.90683;
    //             c2 = -46.87525;
    //             c3 = 2.73582;
    //             c4 = -42.88130;
    //             c5 = -0.53746;
    //             c6 = -0.76271;
    //             c7 = -0.00428;
    //         } else if vc == 4 {
    //             b0 = 263.13428;
    //             b1 = 5.38749;
    //             b2 = -19.04859;
    //             b3 = 2.73018;
    //             b4 = -42.76919;
    //             b5 = -1.31277;
    //             b6 = -0.32242;
    //             b7 = 0.01412;
    //             c0 = 223.68435;
    //             c1 = 10.26908;
    //             c2 = -35.60830;
    //             c3 = 2.31877;
    //             c4 = -38.30034;
    //             c5 = -0.60275;
    //             c6 = -0.67758;
    //             c7 = 0.00117;
    //         } else if vc == 5 {
    //             b0 = 126.95629;
    //             b1 = 5.95754;
    //             b2 = -19.22229;
    //             b3 = 0.43238;
    //             b4 = -7.35636;
    //             b5 = -1.03017;
    //             b6 = -2.66026;
    //             b7 = 0.01389;
    //             c0 = 137.37633;
    //             c1 = 11.00106;
    //             c2 = -38.89043;
    //             c3 = 0.78501;
    //             c4 = -14.88672;
    //             c5 = -0.72576;
    //             c6 = -2.49546;
    //             c7 = 0.00872;
    //         }
    //         d1 = -0.15808;
    //         d2 = -0.83732;
    //         e0 = -1.63246;
    //         e1 = 1.64960;
    //         e2 = -4.45823;
    //         e3 = -4.89119;
    //         e4 = 10.33057;

    //         pf_cap = b0 + b1 * seg_length + b2 * f64::sqrt(seg_length) + b3 * ffs + b4 * f64::sqrt(ffs) + b5 * phv + b6 * f64::sqrt(phv) + b7 * ffs * phv;
    //         pf_25_cap = c0 + c1 * seg_length + c2 * f64::sqrt(seg_length) + c3 * ffs + c4 * f64::sqrt(ffs) + c5 * phv + c6 * f64::sqrt(phv) + c7 * ffs * phv;
    //     }

    //     let mut z_cap = (0.0 - f64::ln(1.0 - pf_cap / 100.0)) / (cap as f64 / 1000.0);
    //     let mut z_25_cap = (0.0 - f64::ln(1.0 - pf_25_cap / 100.0)) / ((0.25 * cap as f64) / 1000.0);

    //     // Slope Coefficient
    //     let mut m_pf = d1 * z_25_cap + d2 * z_cap;
    //     // Power Coefficient
    //     let mut p_pf = e0 + e1 * z_25_cap + e2 * z_cap + e3 * f64::sqrt(z_25_cap) + e4 * f64::sqrt(z_cap);

    //     let mut pf = 100.0 * (1.0 - f64::exp(m_pf * f64::powf(vd / 1000.0, p_pf)));

    //     pf
    // }

    // // Step 7: Calculate passing lane parameters

    // // Step 8: Determine follower density
    // pub fn determine_follower_density_pl(&self) -> f64 {
    //     let mut s_init_fl: f64;
    //     let mut s_init_sl: f64;
    //     let mut pf_fl: f64;
    //     let mut pf_sl: f64;
    //     let mut hor_class = 0;

    //     let vd = self.volume;
    //     let phv = self.phv;
    //     let pm_hv_fl = self.pm_hv_fl;
    //     let subrows_len = self.subrows_len;
        
    //     // Calculate passing lane parameters
    //     let nhv = f64::ceil(vd * phv / 100.0);
    //     let p_v_fl = 0.92183 - 0.05022 * f64::ln(vd) - 0.00030 * nhv;
    //     let vd_fl = f64::ceil(vd * p_v_fl);
    //     let vd_sl = f64::ceil(vd * (1.0 - p_v_fl));
    //     let phv_fl = phv * pm_hv_fl;
    //     let nhv_sl = f64::ceil(nhv - (vd_fl * phv_fl / 100.0));
    //     let phv_sl = nhv_sl / vd_sl * 100.0;
    //     let mut fl_tot = 0.0;
    //     let mut sl_tot = 0.0;

    //     // Subsection
    //     let mut j = 0;
    //     if subrows_len > 0 {
    //         while j < subrows_len {
    //             [s_init_fl, hor_class] = estimateAverageSpeed(Spl, pass_type, ver_cls, subSeg_len[j], ffs, vdFL, vo, PHVFL, is_hc, rad[j], sup_ele[j]);
    //             [s_init_sl, hor_class] = estimateAverageSpeed(Spl, pass_type, ver_cls, subSeg_len[j], ffs, vdSL, vo, PHVSL, is_hc, rad[j], sup_ele[j]);

    //             fl_tot += s_init_fl * subSeg_len[j];
    //             sl_tot += s_init_sl * subSeg_len[j];
    //             j += 1;
    //         }
    //         s_init_fl = s_init_fl / seg_length;
    //         s_init_sl = s_init_sl / seg_length;
    //     } else {
    //         [s_init_fl, hor_class] = estimateAverageSpeed(Spl, pass_type, ver_cls, seg_length, ffs, vdFL, vo, PHVFL, is_hc, rad, sup_ele);
    //         [s_init_sl, hor_class] = estimateAverageSpeed(Spl, pass_type, ver_cls, seg_length, ffs, vdSL, vo, PHVSL, is_hc, rad, sup_ele);
    //     }


    //     pf_fl= estimatePercentFollowers(pass_type, ver_cls, seg_length, ffs, phv_fl, vdFL, vo, capacity);
    //     pf_sl = estimatePercentFollowers(pass_type, ver_cls, seg_length, ffs, phv_sl, vdSL, vo, capacity);

    //     let sda = 2.750 + 0.00056 * vd + 3.8521 * phv / 100.0;
    //     let s_mid_fl = s_init_fl + sda / 2.0;
    //     let s_mid_sl = s_init_sl - sda / 2.0;

    //     // it's acutually fd at the midpoint of the PL segment but used for LOS calculation
    //     let fd_mid = (pf_fl * vd_fl / s_mid_fl + pf_sl * vd_sl / s_mid_sl) / 200.0 ;

    //     fd_mid
    // }


    // pub fn determine_follower_density_pc_pz(&self, pf: f64, s: f64) -> f64 {
    //     let vd = self.volume;
    //     let mut fd = (pf * vd) / (100.0 * s);
    //     fd
    // }


    // pub fn determine_segment_los(&self, fd: f64, s_pl: f64, cap: f64) -> char {
    //     let mut los: char;
        
    //     let vd = self.volume;

    //     if s_pl >= 50.0 {
    //         if fd <= 2.0 { los = 'A' }
    //         else if fd > 2.0 && fd <= 4.0 { los = 'B' }
    //         else if fd > 4.0 && fd <= 8.0 { los = 'C' }
    //         else if fd > 8.0 && fd <= 12.0 { los = 'D' }
    //         else if fd > 12.0 { los = 'E' };
    //         if vd > cap as f64 { los = 'F' };
    //     } else {
    //         if fd <= 2.5 { los = 'A' }
    //         else if fd > 2.5 && fd <= 5.0 { los = 'B' }
    //         else if fd > 5.0 && fd <= 10.0 { los = 'C' }
    //         else if fd > 10.0 && fd <= 15.0 { los = 'D' }
    //         else if fd > 15.0 { los = 'E' }
    //         if vd > cap as f64 { los = 'F' };
    //     }

    //     los
    // }

    // Calculate segment LOS
    // fn calc_seg_LOS(&self) -> String {}
    
    // Calculate LOS
    // fn calc_LOS(&self) -> String {}
}

