use serde::{Serialize, Deserialize};
use crate::utils::math;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubSegment {
    /// Length of subsegment, ft.
    pub length: f64,
    /// Average speed, mi/hr.
    pub avg_speed: f64,
    /// Design radius of subsegment, ft.
    pub design_rad: f64,
    /// Horizontal Class
    pub hor_class: i32,
    /// Superelevation of subsegment, %.
    pub sup_ele: f64
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Segment {
    /// Passing Type.
    /// 0 -> Passing Constrained
    /// 1 -> Passing Zone
    /// 2 -> Passing Lane
    pub passing_type: usize,
    /// Length of segment, mi.
    pub length: f64,
    /// Segment percent grade.
    pub grade: f64,
    /// Posted speed limit, mi/hr.
    pub spl: f64,
    /// Whether the segment has horizontal class or not.
    pub is_hc: bool,
    /// Demand volume for direction i, veh/hr.
    pub volume: f64,
    /// Demand volume for opposite direction o, veh/hr. Required for PZ segments.
    /// 1500 veh/hr for PC segments and 0 for PL segments.
    pub volume_op: f64,
    /// Demand flow rate for analysis direction i, veh/hr
    pub flow_rate: f64,
    /// Demand flow rate for opposite direction i, veh/hr
    pub flow_rate_o: f64,
    /// Capacity, veh/hr
    pub capacity: i32,
    /// Free flow speed, mi/hr
    pub ffs: f64,
    /// Average speed, mi/hr
    pub avg_speed: f64,
    /// Vertical class of the segment.
    pub vertical_class: i32,
    pub subsegments: Vec<SubSegment>,
    /// Peak hour factor, unitless.
    pub phf : f64,
    /// Percentage of heavy vehicles, unitless
    pub phv: f64,
    /// Percent Followers
    pub pf: f64,
    /// Followers Density
    pub fd: f64,
    // /// Horizontal class of the segment.
    pub hor_class: i32,
}

/// Two Lane Highways on chapter 15 of HCM.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TwoLaneHighways {
    pub segments: Vec<Segment>,
    /// Lane width, ft.
    pub lane_width: f64,
    /// Shoulder width, ft.
    pub shoulder_width: f64,
    /// Access point density (access points/mi).
    /// https://highways.dot.gov/safety/other/access-management-driveways
    pub apd: f64,
    /// Percentage multiplier for heavy vehicles in the faster / passing lane
    pub pmhvfl: f64,
    /// Effective distance to passing lane
    pub l_de: f64,
}

/// Implement methods for SubSegment
impl SubSegment {
    /// Method to create a new SubSegment instance
    pub fn new(length: f64, avg_speed: f64, hor_class: i32, design_rad: f64, sup_ele: f64) -> SubSegment {
        SubSegment {
            length,
            avg_speed,
            hor_class,
            design_rad,
            sup_ele,
        }
    }

    /// Method to get the length of the SubSegment
    fn get_length(&self) -> f64 {
        self.length / 5280.0
    }

    fn get_avg_speed(&self) -> f64 {
        self.avg_speed
    }

    fn set_avg_speed(&mut self, avg_speed: f64) {
        self.avg_speed = avg_speed
    }

    fn get_hor_class(&self) -> i32 {
        self.hor_class
    }

    fn set_hor_class(&mut self, hor_class: i32) {
        self.hor_class = hor_class
    }

    fn get_design_rad(&self) -> f64 {
        self.design_rad
    }

    fn get_sup_ele(&self) -> f64 {
        self.sup_ele
    }
}


/// Implement methods for Segment
impl Segment {
    /// Method to create a new Segment instance
    pub fn new(passing_type: usize, length: f64, grade: f64, spl: f64, is_hc: bool, volume: f64, volume_op: f64, flow_rate: f64, flow_rate_o: f64, capacity: i32,
            ffs: f64, avg_speed: f64, vertical_class: i32, subsegments:Vec<SubSegment>, phf: f64, phv: f64, pf: f64, fd: f64, hor_class: i32) -> Segment {
        Segment {
            passing_type,
            length,
            grade,
            spl,
            is_hc,
            volume,
            volume_op,
            flow_rate,
            flow_rate_o,
            capacity,
            ffs,
            avg_speed,
            vertical_class,
            subsegments,
            phf,
            phv,
            pf,
            fd,
            hor_class,
        }
    }

    /// Get passing type
    // fn get_passing_type<'a>(&'a self) -> &'a str {
    //     return &self.passing_type
    // }
    pub fn get_passing_type(&self) -> usize {
        return self.passing_type
    }

    /// Get total length
    /// Need to check segment length is equal to the total length of subsegments
    pub fn get_length(&self) -> f64 {
        return self.length
        // TODO
    }

    fn get_grade(&self) -> f64 {
        return self.grade
    }

    fn get_spl(&self) -> f64 {
        return self.spl
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

    fn get_flow_rate(&self) -> f64 {
        return self.flow_rate
    }

    fn set_flow_rate(&mut self, flow_rate: f64) {
        self.flow_rate = flow_rate
    }

    fn get_flow_rate_o(&self) -> f64 {
        return self.flow_rate_o
    }

    fn set_flow_rate_o(&mut self, flow_rate_o: f64) {
        self.flow_rate_o = flow_rate_o
    }

    fn get_capacity(&self) -> i32 {
        self.capacity
    }

    fn set_capacity(&mut self, capacity: i32) {
        self.capacity = capacity
    }

    fn get_ffs(&self) -> f64 {
        return self.ffs
    }

    fn set_ffs(&mut self, ffs: f64) {
        self.ffs = ffs
    }

    fn get_avg_speed(&self) -> f64 {
        return self.avg_speed
    }

    fn set_avg_speed(&mut self, avg_speed: f64) {
        self.avg_speed = avg_speed
    }

    fn get_vertical_class(&self) -> i32 {
        return self.vertical_class
    }
    
    fn set_vertical_class(&mut self, vertical_class: i32) {
        self.vertical_class = vertical_class
    }

    pub fn get_subsegments(&self) -> &Vec<SubSegment> {
        return &self.subsegments
    }

    fn set_subsegments_avg_speed(&mut self, index: usize, avg_speed: f64) {
        if let Some(subsegment) = self.subsegments.get_mut(index) {
            subsegment.set_avg_speed(avg_speed);
        }
    }

    fn set_subsegments_hor_class(&mut self, index: usize, hor_class: i32) {
        if let Some(subsegment) = self.subsegments.get_mut(index) {
            subsegment.set_hor_class(hor_class);
        }
    }

    fn get_phf(&self) -> f64 {
        return self.phf
    }

    fn get_phv(&self) -> f64 {
        return self.phv
    }

    fn get_percent_followers(&self) -> f64 {
        self.pf
    }

    fn set_percent_followers(&mut self, pf: f64) {
       self.pf = pf
    }

    fn get_followers_density(&self) -> f64 {
        self.fd
    }

    fn set_followers_density(&mut self, fd: f64) {
        self.fd = fd
    }

    // fn get_hor_class(&self) -> i32 {
    //     return self.hor_class
    // }
}


impl TwoLaneHighways {

    /// Returns a segment LOS and LOS
    /// 
    /// # Arguments
    /// 
    /// * `segment number` - the number of segments
    /// 

    pub fn new(segments: Vec<Segment>, lane_width: f64, shoulder_width: f64, apd: f64, pmhvfl: f64, l_de: f64) -> TwoLaneHighways {
        TwoLaneHighways {
            segments: segments,
            lane_width: lane_width,
            shoulder_width: shoulder_width,
            apd: apd,
            pmhvfl: pmhvfl,
            l_de: l_de,
        }
    }

    pub fn get_segments(&self) -> &Vec<Segment> {
        return &self.segments
    }

    /// Step 1: Identify vertical class
    /// TODO: This should be carefully reviewed if a passing lane exceeds 3 mi in length...
    pub fn identify_vertical_class(&mut self, seg_num: usize) -> (f64, f64) {
        let mut _min = 0.0;
        let mut _max = 0.0;
        let vc = self.segments[seg_num].get_vertical_class();
        let pt = self.segments[seg_num].get_passing_type();
        if (vc == 1) || (vc == 2){
            if pt == 0 {
                _min = 0.25;
                _max = 3.0;
            } else if pt == 1 {
                _min = 0.25;
                _max = 2.0;
            } else if pt == 2 {
                _min = 0.5;
                _max = 3.0;
            }
        } else if vc == 3 {
            if pt == 0 {
                _min = 0.25;
                _max = 1.1;
            } else if pt == 1 {
                _min = 0.25;
                _max = 1.1;
            } else if pt == 2 {
                _min = 0.5;
                _max = 1.1;
            }
        } else if (vc == 4) || (vc == 5) {
            if pt == 0 {
                _min = 0.5;
                _max = 3.0;
            } else if pt == 1 {
                _min = 0.5;
                _max = 2.0;
            } else if pt == 2 {
                _min = 0.5;
                _max = 3.0;
            }
        };
        (_min, _max)
    }


    /// Step 2: Determine demand flow rates and capacity
    pub fn determine_demand_flow(&mut self, seg_num: usize) -> (f64, f64, i32) {

        let v_i = self.segments[seg_num].get_volume();
        let v_o = self.segments[seg_num].get_volume_op();
        let phf = self.segments[seg_num].get_phf();
        let phv = self.segments[seg_num].get_phv();
        let pt = self.segments[seg_num].get_passing_type();
        let vc = self.segments[seg_num].get_vertical_class();

        let demand_flow_i = v_i / phf;
        let mut demand_flow_o = 0.0;
        let mut capacity = 0;

        if (pt == 1) && (v_o == 0.0) {
            capacity = 1700;
        } else if pt == 1 {
            demand_flow_o = v_o / phf;
            capacity = 1700;
        } else if pt == 0 {
            demand_flow_o = 1500.0;
            capacity = 1700;
        } else if pt == 2 {
            demand_flow_o = 0.0;
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
        self.segments[seg_num].set_flow_rate(demand_flow_i);
        self.segments[seg_num].set_capacity(capacity);
        self.segments[seg_num].set_flow_rate_o(demand_flow_o);

        (demand_flow_i, demand_flow_o, capacity)


    }


    /// Step 3: Determine vertical alignment classification
    pub fn determine_vertical_alignment(&mut self, seg_num: usize) -> i32 {
        let mut seg_length = self.segments[seg_num].get_length();
        let seg_grade = self.segments[seg_num].get_grade();

        let ver_align: i32;

        if seg_grade >= 0.0 {
            if seg_length <= 0.1 {
                if seg_grade <= 7.0 { ver_align = 1 } else { ver_align = 2 };
            } else if seg_length > 0.1 && seg_length <= 0.2 {
                if seg_grade <= 4.0 { ver_align = 1 } else if seg_grade <= 7.0 { ver_align = 2 } else { ver_align = 3};
            } else if seg_length > 0.2 && seg_length <= 0.3 {
                if seg_grade <= 3.0 { ver_align = 1 }
                else if seg_grade <= 5.0 { ver_align = 2 }
                else if seg_grade <= 7.0 { ver_align = 3 }
                else if seg_grade <= 9.0 { ver_align = 4 }
                else { ver_align = 5 };
            } else if seg_length > 0.3 && seg_length <= 0.4 {
                if seg_grade <= 2.0 { ver_align = 1 }
                else if seg_grade <= 4.0 { ver_align = 2 }
                else if seg_grade <= 6.0 { ver_align = 3 }
                else if seg_grade <= 7.0 { ver_align = 4 }
                else { ver_align = 5 };
            } else if seg_length > 0.4 && seg_length <= 0.5 {
                if seg_grade <= 2.0 { ver_align = 1 }
                else if seg_grade <= 4.0 { ver_align = 2 }
                else if seg_grade <= 3.0 { ver_align = 2 }
                else if seg_grade <= 5.0 { ver_align = 3 }
                else if seg_grade <= 6.0 { ver_align = 4 }
                else { ver_align = 5 };
            } else if seg_length > 0.6 && seg_length <= 0.7 {
                if seg_grade <= 2.0 { ver_align = 1 }
                else if seg_grade <= 3.0 { ver_align = 2 }
                else if seg_grade <= 4.0 { ver_align = 3 }
                else if seg_grade <= 6.0 { ver_align = 4 }
                else { ver_align = 5 };
            } else if seg_length > 0.8 && seg_length <= 1.1 {
                if seg_grade <= 2.0 { ver_align = 1 }
                else if seg_grade <= 3.0 { ver_align = 2 }
                else if seg_grade <= 4.0 { ver_align = 3 }
                else if seg_grade <= 5.0 { ver_align = 4 }
                else { ver_align = 5 };
            } else {
                if seg_grade <= 2.0 { ver_align = 1 }
                else if seg_grade <= 3.0 { ver_align = 2 }
                else if seg_grade <= 5.0 { ver_align = 4 }
                else { ver_align = 5 };
            }
        } else {
            seg_length = -1.0 * seg_length;
            if seg_length <= 0.1 {
                if seg_grade <= 8.0 { ver_align = 1 }
                else { ver_align = 2 };
            } else if seg_length > 0.1 && seg_length <= 0.2 {
                if seg_grade <= 5.0 { ver_align = 1 }
                else if seg_grade <= 8.0 { ver_align = 2 }
                else { ver_align = 3 };
            } else if seg_length > 0.2 && seg_length <= 0.3 {
                if seg_grade <= 4.0 { ver_align = 1 }
                else if seg_grade <= 6.0 { ver_align = 2 }
                else if seg_grade <= 8.0 { ver_align = 3 }
                else if seg_grade <= 9.0 { ver_align = 4 }
                else { ver_align = 5 };
            } else if seg_length > 0.3 && seg_length <= 0.4 {
                if seg_grade <= 2.0 { ver_align = 1 }
                else if seg_grade <= 5.0 { ver_align = 2 }
                else if seg_grade <= 6.0 { ver_align = 3 }
                else if seg_grade <= 8.0 { ver_align = 4 }
                else { ver_align = 5 };
            } else if seg_length > 0.4 && seg_length <= 0.5 {
                if seg_grade <= 3.0 { ver_align = 1 }
                else if seg_grade <= 4.0 { ver_align = 2 }
                else if seg_grade <= 6.0 { ver_align = 3 }
                else if seg_grade <= 7.0 { ver_align = 4 }
                else { ver_align = 5 };
            } else if seg_length > 0.5 && seg_length <= 0.7 {
                if seg_grade <= 3.0 { ver_align = 1 }
                else if seg_grade <= 4.0 { ver_align = 2 }
                else if seg_grade <= 5.0 { ver_align = 3 }
                else if seg_grade <= 6.0 { ver_align = 4 }
                else { ver_align = 5 };
            } else if seg_length > 0.7 && seg_length <= 0.8 {
                if seg_grade <= 3.0 { ver_align = 1 }
                else if seg_grade <= 4.0 { ver_align = 3 }
                else if seg_grade <= 6.0 { ver_align = 4 }
                else { ver_align = 5 };
            } else if seg_length > 0.8 && seg_length <= 0.9 {
                if seg_grade <= 3.0 { ver_align = 1 }
                else if seg_grade <= 4.0 { ver_align = 3 }
                else if seg_grade <= 5.0 { ver_align = 4 }
                else { ver_align = 5 };
            } else if seg_length > 0.9 && seg_length <= 1.1 {
                if seg_grade <= 2.0 { ver_align = 1 }
                else if seg_grade <= 3.0 { ver_align = 2 }
                else if seg_grade <= 4.0 { ver_align = 3 }
                else if seg_grade <= 5.0 { ver_align = 4 }
                else { ver_align = 5 };
            } else {
                if seg_grade <= 2.0 { ver_align = 1 }
                else if seg_grade <= 3.0 { ver_align = 2 }
                else if seg_grade <= 5.0 { ver_align = 4 }
                else { ver_align = 5 };
            }
        }
        if ver_align != self.segments[seg_num].get_vertical_class(){
            self.segments[seg_num].set_vertical_class(ver_align);
            // Run step 1 again.
            self.identify_vertical_class(seg_num);
        }

        ver_align
    }

    /// Step 4: Determine free-flow speed
    pub fn determine_free_flow_speed(&mut self, seg_num: usize) -> f64 {

        let spl = self.segments[seg_num].get_spl();
        let vc = self.segments[seg_num].get_vertical_class();
        let vo = self.segments[seg_num].get_flow_rate_o();
        let lw = self.lane_width;
        let sw = self.shoulder_width;
        let apd = self.apd;
        let phv = self.segments[seg_num].get_phv();
        let seg_length = self.segments[seg_num].get_length();

        let ffs: f64;

        let bffs = 1.14 * spl;
        let mut a0 = 0.0;
        let mut a1 = 0.0;
        let mut a2 = 0.0;
        let mut a3 = 0.0;
        let mut a4 = 0.0;
        let mut a5 = 0.0;

        if vc == 1 {
            a0 = 0.0;
            a1 = 0.0;
            a2 = 0.0;
            a3 = 0.0;
            a4 = 0.0;
            a5 = 0.0;
        } else if vc == 2 {
            a0 = -0.45036;
            a1 = 0.00814;
            a2 = 0.01543;
            a3 = 0.01358;
            a4 = 0.0;
            a5 = 0.0;
        } else if vc == 3 {
            a0 = -0.29591;
            a1 = 0.00743;
            a2 = 0.0;
            a3 = 0.01246;
            a4 = 0.0;
            a5 = 0.0;
        } else if vc == 4 {
            a0 = -0.40902;
            a1 = 0.00975;
            a2 = 0.00767;
            a3 = -0.18363;
            a4 = 0.00423;
            a5 = 0.0;
        } else if vc == 5 {
            a0 = -0.3836;
            a1 = 0.01074;
            a2 = 0.01945;
            a3 = -0.69848;
            a4 = 0.01069;
            a5 = 0.127;
        }

        let a = f64::max(
            0.0333,
            a0 + a1 * bffs + a2 * seg_length + (f64::max(0.0, a3 + a4 * bffs + a5 * seg_length) * vo) / 1000.0,
        );

        // adjustment for lane and shoulder width, mi/hr
        let f_ls = 0.6 * (12.0 - lw) + 0.7 * (6.0 - sw);
        // adjustment for access point density, mi/hr
        let f_a = f64::min(apd / 4.0, 10.0);

        ffs = bffs - a * phv - f_ls - f_a;
        self.segments[seg_num].set_ffs(ffs);

        ffs
    }

    /// Step 5: Estimate average speed
    pub fn estimate_average_speed(&mut self, seg_num: usize) -> (f64, i32) {
        let spl = self.segments[seg_num].get_spl();
        let bffs = math::round_to_significant_digits(1.14 * spl, 3);

        // Get variables from segments
        let mut s: f64; // average speed
        let mut tot_s: f64 = 0.0; // total speed
        let res_s: f64; // Results speed
        let mut hor_class: i32;
        let seg_s:f64;
        let seg_hor_class: i32;
        let ffs = self.segments[seg_num].get_ffs();
        let pt = self.segments[seg_num].get_passing_type();
        let phf = self.segments[seg_num].get_phf();
        let phv = self.segments[seg_num].get_phv();
        let vc = self.segments[seg_num].get_vertical_class();
        let vd = self.segments[seg_num].get_flow_rate();
        let vo = self.segments[seg_num].get_flow_rate_o();
        let is_hc = self.segments[seg_num].get_is_hc();

        // Determine Segment Avg Speed
        let seg_length = self.segments[seg_num].get_length();
        // Only affected when it contains subsegments
        let rad = 0.0;
        let sup_ele = 0.0;
        (seg_s, seg_hor_class) = self.calc_speed(seg_length, bffs, ffs, pt, vc, vd, vo, phv, phf, false, rad, sup_ele);

        if is_hc {
            // Get variables from subsegments
            let subseg_num = self.segments[seg_num].get_subsegments().len();
            // let mut subseg_length: Vec<f64>; // = (0../collect();
            // let mut sup_ele: Vec<f64>; // = (0..seg_num).collect();
            let mut i = 0;
            while i < subseg_num {
                let subseg_length = self.segments[seg_num].get_subsegments()[i].get_length();
                let rad = self.segments[seg_num].get_subsegments()[i].get_design_rad();
                let sup_ele = self.segments[seg_num].get_subsegments()[i].get_sup_ele();
                if rad > 0.0 {
                    (s, hor_class) = self.calc_speed(seg_length, bffs, ffs, pt, vc, vd, vo, phv, phf, is_hc, rad, sup_ele);
                    tot_s += s * subseg_length * 5280.0;

                    self.segments[seg_num].set_subsegments_avg_speed(i, s);
                    self.segments[seg_num].set_subsegments_hor_class(i, hor_class);

                } else { // Tangent Section
                    self.segments[seg_num].set_subsegments_avg_speed(i, seg_s);
                    self.segments[seg_num].set_subsegments_hor_class(i, seg_hor_class);
                    tot_s += math::round_to_significant_digits(seg_s * subseg_length * 5280.0, 3);

                    // println!("Sub Segments: {i}, Speed: {seg_s}: Length: {subseg_length}");
                }
                i += 1;
            }
            res_s = tot_s / (seg_length * 5280.0) as f64;
        } else {
            res_s = seg_s;
        }

        self.segments[seg_num].set_avg_speed(res_s);
        // self.segments[seg_num].seg_hor_class(hor_class);

        (res_s, seg_hor_class)
    }

    fn calc_speed(&self, seg_length: f64, bffs: f64, mut ffs: f64, pt: usize, vc: i32, vd: f64, vo: f64, phv: f64, phf: f64, is_hc:bool, rad: f64, sup_ele: f64) -> (f64, i32) {
        // Parameter initialization
        let (mut b0, mut b1, mut b2, mut b3, mut b4, mut b5) = (0.0000, 0.0000, 0.0000, 0.0000, 0.0000, 0.0000);
        let (mut c0, mut c1, mut c2, mut c3) = (0.0000, 0.0000, 0.0000, 0.0000);
        let (mut d0, mut d1, mut d2, mut d3) = (0.0000, 0.0000, 0.0000, 0.0000);
        let (mut f0, mut f1, mut f2, mut f3, mut f4, mut f5, mut f6, mut f7, mut f8) = (0.0000, 0.0000, 0.0000, 0.0000, 0.0000, 0.0000, 0.0000, 0.0000, 0.0000);

        ffs = math::round_up_to_n_decimal(ffs, 1);

        let mut s: f64;
        let mut hor_class: i32 = 0;

        if pt == 0 || pt == 1 {
            if vc == 1 {
                b0 = 0.0558;
                b1 = 0.0542;
                b2 = 0.3278;
                b3 = 0.1029;
                f0 = 0.67576;
                f3 = 0.12060;
                f4 = -0.35919;
            } else if vc == 2 {
                b0 = 5.7280;
                b1 = -0.0809;
                b2 = 0.7404;
                b5 = 3.1155;
                c0 = -13.8036;
                c2 = 0.2446;
                d0 = -1.7765;
                d2 = 0.0392;
                b3 = c0 + c1 * f64::sqrt(seg_length) + c2 * ffs + c3 * ffs * f64::sqrt(seg_length);
                b4 = d0 + d1 * f64::sqrt(phv) + d2 * ffs + d3 * ffs * f64::sqrt(phf);
                f0 = 0.34524;
                f1 = 0.00591;
                f2 = 0.02031;
                f3 = 0.14911;
                f4 = -0.43784;
                f5 = -0.00296;
                f6 = 0.02956;
                f8 = 0.41622;
            } else if vc == 3 {
                b0 = 9.3079;
                b1 = -0.1706;
                b2 = 1.1292;
                b5 = 3.1155;
                c0 = -11.9703;
                c2 = 0.2542;
                d0 = -3.5550;
                d2 = 0.0826;
                b3 = c0 + c1 * f64::sqrt(seg_length) + c2 * ffs + c3 * ffs * f64::sqrt(seg_length);
                b4 = d0 + d1 * f64::sqrt(phv) + d2 * ffs + d3 * ffs * f64::sqrt(phv);
                f0 = 0.17291;
                f1 = 0.00917;
                f2 = 0.05698;
                f3 = 0.27734;
                f4 = -0.61893;
                f5 = -0.00918;
                f6 = 0.09184;
                f8 = 0.41622;
            } else if vc == 4 {
                b0 = 9.0115;
                b1 = -0.1994;
                b2 = 1.8252;
                b5 = 3.2685;
                c0 = -12.5113;
                c2 = 0.2656;
                d0 = -5.7775;
                d2 = 0.1373;
                b3 = math::round_up_to_n_decimal(c0 + c1 * f64::sqrt(seg_length) + c2 * ffs + c3 * ffs * f64::sqrt(seg_length), 4);
                b4 = math::round_up_to_n_decimal(d0 + d1 * f64::sqrt(phv) + d2 * ffs + d3 * ffs * f64::sqrt(phv), 4);
                f0 = 0.67689;
                f1 = 0.00534;
                f2 = -0.13037;
                f3 = 0.25699;
                f4 = -0.68465;
                f5 = -0.00709;
                f6 = 0.07087;
                f8 = 0.33950;
            } else if vc == 5 {
                b0 = 23.9144;
                b1 = -0.6925;
                b2 = 1.9473;
                b5 = 3.5115;
                c0 = -14.8961;
                c2 = 0.4370;
                d0 = -18.2910;
                d1 = 2.3875;
                d2 = 0.4494;
                d3 = -0.0520;
                b3 = c0 + c1 * f64::sqrt(seg_length) + c2 * ffs + c3 * ffs * f64::sqrt(seg_length);
                b4 = d0 + d1 * f64::sqrt(phv) + d2 * ffs + d3 * ffs * f64::sqrt(phv);
                f0 = 1.13262;
                f2 = -0.26367;
                f3 = 0.18811;
                f4 = -0.64304;
                f5 = -0.00867;
                f6 = 0.08675;
                f8 = 0.30590;
            }
        } else if pt == 2 {
            if vc == 1 {
                b0 = -1.1379;
                b1 = 0.0941;
                c1 = 0.2667;
                d1 = 0.1252;
                b3 = c0 + c1 * f64::sqrt(seg_length) + c2 * ffs + c3 * ffs * f64::sqrt(seg_length);
                b4 = d0 + d1 * f64::sqrt(phv) + d2 * ffs + d3 * ffs * f64::sqrt(phv);
                f0 = 0.91793;
                f1 = -0.00557;
                f2 = 0.36862;
                f5 = 0.00611;
                f7 = -0.00419;
            } else if vc == 2 {
                b0 = -2.0668;
                b1 = 0.1053;
                c1 = 0.4479;
                d1 = 0.1631;
                b3 = c0 + c1 * f64::sqrt(seg_length) + c2 * ffs + c3 * ffs * f64::sqrt(seg_length);
                b4 = d0 + d1 * f64::sqrt(phv) + d2 * ffs + d3 * ffs * f64::sqrt(phv);
                f0 = 0.65105;
                f2 = 0.34931;
                f5 = 0.00722;
                f7 = -0.00391;
            } else if vc == 3 {
                b0 = -0.5074;
                b1 = 0.0935;
                d1 = -0.2201;
                d3 = 0.0072;
                b4 = d0 + d1 * f64::sqrt(phv) + d2 * ffs + d3 * ffs * f64::sqrt(phv);
                f0 = 0.40117;
                f2 = 0.68633;
                f5 = 0.02350;
                f7 = -0.02088;
            } else if vc == 4 {
                b0 = 8.0354;
                b1 = -0.0860;
                b5 = 4.1900;
                c0 = -27.1244;
                c1 = 11.5196;
                c2 = 0.4681;
                c3 = -0.1873;
                d1 = -0.7506;
                d3 = 0.0193;
                b3 = c0 + c1 * f64::sqrt(seg_length) + c2 * ffs + c3 * ffs * f64::sqrt(seg_length);
                b4 = d0 + d1 * f64::sqrt(phv) + d2 * ffs + d3 * ffs * f64::sqrt(phv);
                f0 = 1.13282;
                f1 = -0.00798;
                f2 = 0.35425;
                f5 = 0.01521;
                f7 = -0.00987;
            } else if vc == 5 {
                b0 = 7.2991;
                b1 = -0.3535;
                b5 = 4.8700;
                c0 = -45.3391;
                c1 = 17.3749;
                c2 = 1.0587;
                c3 = -0.3729;
                d0 = 3.8457;
                d1 = -0.9112;
                d3 = 0.0170;
                b3 = c0 + c1 * f64::sqrt(seg_length) + c2 * ffs + c3 * ffs * f64::sqrt(seg_length);
                b4 = d0 + d1 * f64::sqrt(phv) + d2 * ffs + d3 * ffs * f64::sqrt(phv);
                f0 = 1.12077;
                f1 = -0.00550;
                f2 = 0.25431;
                f5 = 0.01269;
                f7 = -0.01053;
            }
        }
        // slope coefficient for average speed calculation
        let mut ms = f64::max(
        b5,
        b0 +
            b1 * ffs +
            b2 * f64::sqrt(vo / 1000.0) +
            f64::max(0.0, b3) * f64::sqrt(seg_length) +
            f64::max(0.0, b4) * f64::sqrt(phv),
        );
    
        // power coefficient for average speed calculation
        let mut ps = f64::max(
        f8,
        f0 +
            f1 * ffs +
            f2 * seg_length +
            (f3 * vo) / 1000.0 +
            f4 * f64::sqrt(vo / 1000.0) +
            f5 * phv +
            f6 * f64::sqrt(phv) +
            f7 * seg_length * phv,
        );

        ms = math::round_up_to_n_decimal(ms, 3);
        ps = math::round_up_to_n_decimal(ps, 3);

        // Length of horizontal curves = radius x central angle x pi/180
        // determine horizontal class
        if rad == 0.0 { 
            hor_class = 0;
        } else if rad > 0.0 && rad < 300.0 {
            hor_class = 5;
        } else if rad >= 300.0 && rad < 450.0 {
            hor_class = 4;
        } else if rad >= 450.0 && rad < 600.0 {
            if sup_ele < 1.0 { hor_class = 4 } else { hor_class = 3 };
        } else if rad >= 600.0 && rad < 750.0 {
            if sup_ele < 6.0 { hor_class = 3 } else { hor_class = 2 };
        } else if rad >= 750.0 && rad < 900.0 {
            hor_class = 2;
        } else if rad >= 900.0 && rad < 1050.0 {
            if sup_ele < 8.0 { hor_class = 2 } else { hor_class = 1 };
        } else if rad >= 1050.0 && rad < 1200.0 {
            if sup_ele < 4.0 { hor_class = 2 } else { hor_class = 1 };
        } else if rad >= 1200.0 && rad < 1350.0 {
            if sup_ele < 2.0 { hor_class = 2 } else { hor_class = 1 };
        } else if rad >= 1350.0 && rad < 1500.0 {
            hor_class = 1;
        } else if rad >= 1500.0 && rad < 1750.0 {
            if sup_ele < 8.0 { hor_class = 1 } else { hor_class = 0 };
        } else if rad >= 1750.0 && rad < 1800.0 {
            if sup_ele < 6.0 { hor_class = 1 } else { hor_class = 0 };
        } else if rad >= 1800.0 && rad < 1950.0 {
            if sup_ele < 5.0 { hor_class = 1 } else { hor_class = 0 };
        } else if rad >= 1950.0 && rad < 2100.0 {
            if sup_ele < 4.0 { hor_class = 1 } else { hor_class = 0 };
        } else if rad >= 2100.0 && rad < 2250.0 {
            if sup_ele < 3.0 { hor_class = 1 } else { hor_class = 0 };
        } else if rad >= 2250.0 && rad < 2400.0 {
            if sup_ele < 2.0 { hor_class = 1 } else { hor_class = 0 };
        } else if rad >= 2400.0 && rad < 2550.0 {
            if sup_ele < 1.0 { hor_class = 1 } else { hor_class = 0 };
        } else if rad >= 2550.0 {
            hor_class = 0;
        }

        if vd <= 100.0 {
            let st = ffs;
            s = st;
        } else {
            let st = ffs - ms * f64::powf(vd / 1000.0 - 0.1, ps);
            s = st;
        }

        if is_hc {
            // calculate horizontal class
            let bffshc = f64::min(bffs, 44.32 + 0.3728 * bffs - 6.868 * hor_class as f64);
            let ffshc = bffshc - 0.0255 * phv;
            let mhc = math::round_to_significant_digits(f64::max(0.277, -25.8993 - 0.7756 * ffshc + 10.6294 * f64::sqrt(ffshc) + 2.4766 * hor_class as f64 - 9.8238 * f64::sqrt(hor_class as f64)), 5);
            // println!("s: {s}");
            let shc = math::round_to_significant_digits(f64::min(s, ffshc - mhc * f64::sqrt(vd / 1000.0 - 0.1)), 3); // Should be ST instead of S?
            // println!("BFFS: {bffshc}, FFSHC: {ffshc}, MHC: {mhc}, SHC: {shc}");
            s = shc;
        }
        (s, hor_class)
    }

    fn calc_percent_followers(&self, seg_length: f64, mut ffs: f64, cap: i32, pt: usize, vc: i32, vd: f64, vo: f64, phv: f64) -> f64{

        let (mut b0, mut b1, mut b2, mut b3, mut b4, mut b5, mut b6, mut b7) = (0.000000, 0.000000, 0.000000, 0.000000, 0.000000, 0.000000, 0.000000, 0.000000);
        let (mut c0, mut c1, mut c2, mut c3, mut c4, mut c5, mut c6, mut c7) = (0.000000, 0.000000, 0.000000, 0.000000, 0.000000, 0.000000, 0.000000, 0.000000);
        let (mut d1, mut d2) = (0.000000, 0.000000);
        let (mut e0, mut e1, mut e2, mut e3, mut e4) = (0.000000, 0.000000, 0.000000, 0.000000, 0.000000);

        ffs = math::round_up_to_n_decimal(ffs, 2);

        // Percent followers at capacity
        let mut pf_cap = 0.0;
        let mut pf_25_cap = 0.0;

        if pt == 0 || pt == 1 {
            if vc == 1 {
                b0 = 37.68080;
                b1 = 3.05089;
                b2 = -7.90866;
                b3 = -0.94321;
                b4 = 13.64266;
                b5 = -0.00050;
                b6 = -0.05500;
                b7 = 7.13760;
                c0 = 18.01780;
                c1 = 10.00000;
                c2 = -21.60000;
                c3 = -0.97853;
                c4 = 12.05214;
                c5 = -0.00750;
                c6 = -0.06700;
                c7 = 11.60410;
            } else if vc == 2 {
                b0 = 58.21104;
                b1 = 5.73387;
                b2 = -13.66293;
                b3 = -0.66126;
                b4 = 9.08575;
                b5 = -0.00950;
                b6 = -0.03602;
                b7 = 7.14619;
                c0 = 47.83887;
                c1 = 12.80000;
                c2 = -28.20000;
                c3 = -0.61758;
                c4 = 5.80000;
                c5 = -0.04550;
                c6 = -0.03344;
                c7 = 11.35573;
            } else if vc == 3 {
                b0 = 113.20439;
                b1 = 10.01778;
                b2 = -18.90000;
                b3 = 0.46542;
                b4 = -6.75338;
                b5 = -0.03000;
                b6 = -0.05800;
                b7 = 10.03239;
                c0 = 125.40000;
                c1 = 19.50000;
                c2 = -34.90000;
                c3 = 0.90672;
                c4 = -16.10000;
                c5 = -0.11000;
                c6 = -0.06200;
                c7 = 14.71136;
            } else if vc == 4 {
                b0 = 58.29978;
                b1 = -0.53611;
                b2 = 7.35076;
                b3 = -0.27046;
                b4 = 4.49850;
                b5 = -0.01100;
                b6 = -0.02968;
                b7 = 8.89680;
                c0 = 103.13534;
                c1 = 14.68459;
                c2 = -23.72704;
                c3 = 0.66444;
                c4 = -11.95763;
                c5 = -0.10000;
                c6 = 0.00172;
                c7 = 14.70067;
            } else if vc == 5 {
                b0 = 3.32968;
                b1 = -0.84377;
                b2 = 7.08952;
                b3 = -1.32089;
                b4 = 19.98477;
                b5 = -0.01250;
                b6 = -0.02960;
                b7 = 9.99453;
                c0 = 89.00000;
                c1 = 19.02642;
                c2 = -34.54240;
                c3 = 0.29792;
                c4 = -6.62528;
                c5 = -0.16000;
                c6 = 0.00480;
                c7 = 17.56611;
            }
            d1 = -0.29764;
            d2 = -0.71917;
            e0 = 0.81165;
            e1 = 0.37920;
            e2 = -0.49524;
            e3 = -2.11289;
            e4 = 2.41146;

            pf_cap = b0 + b1 * seg_length + b2 * f64::sqrt(seg_length) + b3 * ffs + b4 * f64::sqrt(ffs) + b5 * phv + b6 * ffs * vo / 1000.0 + b7 * f64::sqrt(vo/1000.0);
            pf_25_cap = c0 + c1 * seg_length + c2 * f64::sqrt(seg_length) + c3 * ffs + c4 * f64::sqrt(ffs) + c5 * phv + c6 * ffs * vo / 1000.0 + c7 * f64::sqrt(vo/1000.0);
        } else if pt == 2 {
            if vc == 1 {
                b0 = 61.73075;
                b1 = 6.73922;
                b2 = -23.68853;
                b3 = -0.84126;
                b4 = 11.44533;
                b5 = -1.05124;
                b6 = 1.50390;
                b7 = 0.00491;
                c0 = 80.37105;
                c1 = 14.44997;
                c2 = -46.41831;
                c3 = -0.23367;
                c4 = 0.84914;
                c5 = -0.56747;
                c6 = 0.89427;
                c7 = 0.00119;
            } else if vc == 2 {
                b0 = 12.30096;
                b1 = 9.57465;
                b2 = -30.79427;
                b3 = -1.79448;
                b4 = 25.76436;
                b5 = -0.66350;
                b6 = 1.26039;
                b7 = -0.00323;
                c0 = 18.37886;
                c1 = 14.71856;
                c2 = -47.78892;
                c3 = -1.43373;
                c4 = 18.32040;
                c5 = -0.13226;
                c6 = 0.77127;
                c7 = -0.00778;
            } else if vc == 3 {
                b0 = 206.07369;
                b1 = -4.29885;
                b2 = 0.00000;
                b3 = 1.96483;
                b4 = -30.32556;
                b5 = -0.75812;
                b6 = 1.06453;
                b7 = -0.00839;
                c0 = 239.98930;
                c1 = 15.90683;
                c2 = -46.87525;
                c3 = 2.73582;
                c4 = -42.88130;
                c5 = -0.53746;
                c6 = -0.76271;
                c7 = -0.00428;
            } else if vc == 4 {
                b0 = 263.13428;
                b1 = 5.38749;
                b2 = -19.04859;
                b3 = 2.73018;
                b4 = -42.76919;
                b5 = -1.31277;
                b6 = -0.32242;
                b7 = 0.01412;
                c0 = 223.68435;
                c1 = 10.26908;
                c2 = -35.60830;
                c3 = 2.31877;
                c4 = -38.30034;
                c5 = -0.60275;
                c6 = -0.67758;
                c7 = 0.00117;
            } else if vc == 5 {
                b0 = 126.95629;
                b1 = 5.95754;
                b2 = -19.22229;
                b3 = 0.43238;
                b4 = -7.35636;
                b5 = -1.03017;
                b6 = -2.66026;
                b7 = 0.01389;
                c0 = 137.37633;
                c1 = 11.00106;
                c2 = -38.89043;
                c3 = 0.78501;
                c4 = -14.88672;
                c5 = -0.72576;
                c6 = -2.49546;
                c7 = 0.00872;
            }
            d1 = -0.15808;
            d2 = -0.83732;
            e0 = -1.63246;
            e1 = 1.64960;
            e2 = -4.45823;
            e3 = -4.89119;
            e4 = 10.33057;

            pf_cap = b0 + b1 * seg_length + b2 * f64::sqrt(seg_length) + b3 * ffs + b4 * f64::sqrt(ffs) + b5 * phv + b6 * f64::sqrt(phv) + b7 * ffs * phv;
            pf_25_cap = c0 + c1 * seg_length + c2 * f64::sqrt(seg_length) + c3 * ffs + c4 * f64::sqrt(ffs) + c5 * phv + c6 * f64::sqrt(phv) + c7 * ffs * phv;
        }

        let z_cap = (0.0 - f64::ln(1.0 - pf_cap / 100.0)) / (cap as f64 / 1000.0);
        let z_25_cap = (0.0 - f64::ln(1.0 - pf_25_cap / 100.0)) / ((0.25 * cap as f64) / 1000.0);

        // Slope Coefficient
        let m_pf = d1 * z_25_cap + d2 * z_cap;
        // Power Coefficient
        let p_pf = e0 + e1 * z_25_cap + e2 * z_cap + e3 * f64::sqrt(z_25_cap) + e4 * f64::sqrt(z_cap);

        let pf = 100.0 * (1.0 - f64::exp(m_pf * f64::powf(vd / 1000.0, p_pf)));

        pf
    }

    /// Step 6: Estimate percent followers
    pub fn estimate_percent_followers(&mut self, seg_num: usize) -> f64 {
        
        let seg_length = self.segments[seg_num].get_length();
        let ffs = self.segments[seg_num].get_ffs();
        let cap = self.segments[seg_num].get_capacity();
        let pt = self.segments[seg_num].get_passing_type();
        let vc = self.segments[seg_num].get_vertical_class();
        let vd = self.segments[seg_num].get_flow_rate();
        let vo = self.segments[seg_num].get_flow_rate_o();
        let phv = self.segments[seg_num].get_phv();

        let pf = self.calc_percent_followers(seg_length, ffs, cap, pt, vc, vd, vo, phv);
        
        self.segments[seg_num].set_percent_followers(pf);

        pf
    }

    pub fn estimate_average_speed_sf(&mut self, seg_num: usize, length: f64, vd: f64, phv: f64, rad: f64, sup_ele: f64) -> (f64, i32) {
        let spl = self.segments[seg_num].get_spl();
        let bffs = 1.14 * spl;

        // Get variables from segments
        let s: f64; // average speed
        let hor_class: i32;
        let ffs = self.segments[seg_num].get_ffs();
        let pt = self.segments[seg_num].get_passing_type();
        let phf = self.segments[seg_num].get_phf();
        let vc = self.segments[seg_num].get_vertical_class();
        let vo = self.segments[seg_num].get_flow_rate_o();
        let is_hc = self.segments[seg_num].get_is_hc();

        (s, hor_class) = self.calc_speed(length, bffs, ffs, pt, vc, vd, vo, phv, phf, is_hc, rad, sup_ele);
        
        (s, hor_class)
    }

    pub fn estimate_percent_followers_sf(&self, seg_num: usize, vd: f64, phv: f64) -> f64 {
        let seg_length = self.segments[seg_num].get_length();
        let ffs = self.segments[seg_num].get_ffs();
        let cap = self.segments[seg_num].get_capacity();
        let pt = self.segments[seg_num].get_passing_type();
        let vc = self.segments[seg_num].get_vertical_class();
        let vo = self.segments[seg_num].get_flow_rate_o();

        let pf = self.calc_percent_followers(seg_length, ffs, cap, pt, vc, vd, vo, phv);


        pf
    }

    // Step 7: Calculate passing lane parameters

    // Step 8: Determine follower density
    pub fn determine_follower_density_pl(&mut self, seg_num: usize) -> f64 {
        let mut s_init_fl: f64 = 0.0;
        let mut s_init_sl: f64 = 0.0;
        let pf_fl: f64;
        let pf_sl: f64;

        let seg_length = self.segments[seg_num].get_length();
        let subseg_num = self.segments[seg_num].get_subsegments().len();
        let vd = self.segments[seg_num].get_flow_rate();
        let phv = self.segments[seg_num].get_phv();
        let pm_hv_fl = self.pmhvfl;
        
        // Calculate passing lane parameters
        let nhv = f64::round(vd * phv / 100.0);
        let p_v_fl = 0.92183 - 0.05022 * f64::ln(vd) - 0.00030 * nhv;
        let vd_fl = f64::round(vd * p_v_fl);
        let vd_sl = f64::round(vd * (1.0 - p_v_fl));
        let phv_fl = phv * pm_hv_fl;
        let nhv_sl = f64::ceil(nhv - (vd_fl * phv_fl / 100.0));
        let phv_sl = nhv_sl / vd_sl * 100.0;
        let mut fl_tot: f64 = 0.0;
        let mut sl_tot: f64 = 0.0;


        // Subsection
        let mut j = 0;
        if subseg_num > 0 {
            while j < subseg_num {
                let sub_seg_len = self.segments[seg_num].get_subsegments()[j].get_length();
                let rad = self.segments[seg_num].get_subsegments()[j].get_design_rad();
                let sup_ele = self.segments[seg_num].get_subsegments()[j].get_sup_ele();
                (s_init_fl, _) = self.estimate_average_speed_sf(seg_num, sub_seg_len, vd_fl, phv_fl, rad, sup_ele);
                (s_init_sl, _) = self.estimate_average_speed_sf(seg_num, sub_seg_len, vd_sl, phv_sl, rad, sup_ele);

                fl_tot += s_init_fl * sub_seg_len;
                sl_tot += s_init_sl * sub_seg_len;
                j += 1;
            }
            s_init_fl = fl_tot / seg_length;
            s_init_sl = sl_tot / seg_length;
        } else {
            let rad = 0.0;
            let sup_ele = 0.0;
            (s_init_fl, _) = self.estimate_average_speed_sf(seg_num, seg_length, vd_fl, phv_fl, rad, sup_ele);
            (s_init_sl, _) = self.estimate_average_speed_sf(seg_num, seg_length, vd_sl, phv_sl, rad, sup_ele);
        }


        pf_fl= self.estimate_percent_followers_sf(seg_num, vd_fl, phv_fl);
        pf_sl = self.estimate_percent_followers_sf(seg_num, vd_sl, phv_sl);

        let sda = 2.750 + 0.00056 * vd + 3.8521 * phv / 100.0;
        let s_mid_fl = s_init_fl + sda / 2.0;
        let s_mid_sl = s_init_sl - sda / 2.0;

        // it's acutually fd at the midpoint of the PL segment but used for LOS calculation
        let fd_mid = (pf_fl * vd_fl / s_mid_fl + pf_sl * vd_sl / s_mid_sl) / 200.0 ;

        self.segments[seg_num].set_followers_density(fd_mid);

        fd_mid
    }


    pub fn determine_follower_density_pc_pz(&mut self, seg_num: usize) -> f64 {
        let s = self.segments[seg_num].get_avg_speed();
        let pf = self.segments[seg_num].get_percent_followers();
        let vd = self.segments[seg_num].get_flow_rate();
        let fd = (pf * vd) / (100.0 * s);

        self.segments[seg_num].set_followers_density(fd);
        fd
    }

    pub fn determine_adjustment_to_follower_density(&mut self, seg_num: usize) -> f64 {
        let seg_len = self.segments.len();
        let mut is_pl_list: Vec<bool> = Vec::new();
        let s = self.segments[seg_num].get_avg_speed();
        let mut pl_loc = 100;
        let pass_type = self.segments[seg_num].get_passing_type();

        for s_num in 0..seg_len {
            let p_type = self.segments[s_num].get_passing_type();
            if p_type == 2 {
                is_pl_list.push(true);
                pl_loc = s_num; // TODO: if there are more than three PL section
            } else {
                is_pl_list.push(false);
            }
        }

        // Accumulate segments length from PL on upstream
        let mut l_d: f64 = 0.0;
        if pl_loc <= seg_num {
            for s_num in pl_loc..seg_num+1 {
                l_d += self.segments[s_num].get_length();
            }
        }

        // Calculate downstream distance from start of passing lane
        let mut fd_adj: f64 = 0.0;
        let pf = self.segments[seg_num].get_percent_followers();
        let pl = is_pl_list.len();
        if seg_num > 0 && is_pl_list.len() > 0{
            // let pf_u = self.segments[seg_num-1].get_percent_followers();
            let pf_u = self.segments[pl_loc-1].get_percent_followers();
            let vd = self.segments[seg_num].get_flow_rate();
            let vd_u = self.segments[seg_num-1].get_flow_rate();
            let fd_u = self.segments[seg_num-1].get_followers_density();
            let mut l_de: f64 = 0.0; // effective distance

            let x_2 = 0.1 * f64::max(0.0, pf_u - 30.0);
            let x_3a = 3.5 * f64::ln(f64::max(0.3, self.segments[pl_loc].get_length()));
            let x_3b = 0.75 * self.segments[pl_loc].get_length();

            // Determine effective distance of PL
            if pass_type == 2 {
                let x_4a = 0.01 * vd_u;
                let x_4b = 0.005 * vd_u;
                let y_1a = 27.0 + x_2 + x_3a - x_4a;
                let y_2a = 3.0 + x_2 + x_3b - x_4b;
                let y_3 = (95.0 * self.segments[seg_num-1].get_followers_density() * s) / (pf_u * vd_u);

                // Solve for downstream effective length of passing lane from start of PL (LDE)
                // The percentage improvement to the percent followers becomes zero
                let l_de_1 = f64::exp(y_1a / 8.75);

                // Follower density is at least 95% of the level entering the passing lane
                let l_de_2 = f64::max(0.1, f64::exp(-1.0 * (f64::max(0.0, -1.0 * y_1a + 32.0) - 27.0) / 8.75));

                l_de = math::round_up_to_n_decimal(f64::min(l_de_1, l_de_2), 1);
                self.l_de = l_de;

                let pf_improve = f64::max(0.0, y_1a - 8.75 * f64::ln(f64::max(0.1, l_de)));
                let s_improve = f64::max(0.0, y_2a - 0.8 * l_de);
                let y_3 = (100.0 - pf_improve) / (100.0 + s_improve);
                
                fd_adj = (pf_u / 100.0) * (1.0 - pf_improve / 100.0) * vd_u / (s * (1.0 + s_improve / 100.0));
                // fd_adj = (pf_u / 100.0) * (1.0 - pf_improve / 100.0) * vd_u / (58.8 * (1.0 + s_improve / 100.0));

            } else {
                // Determine adjustment to follower density
                // if segment is within effective distance of neaest upstream passing lane
                // Passing Lane itself can also be placed within the effective length

                if l_d < self.l_de {

                    let x_1a = 8.75 * f64::ln(f64::max(0.1, l_d));
                    let x_1b = 0.8 * l_d;
                    let x_4c = 0.01 * self.segments[seg_num].get_flow_rate();
                    let x_4d = 0.005 * self.segments[seg_num].get_flow_rate();
                    let y_1b = 27.0 - x_1a + x_2 + x_3a - x_4c;
                    let y_2b = 3.0 - x_1b + x_2 + x_3b - x_4d;
                    let pf_improve = math::round_up_to_n_decimal(f64::max(0.0, y_1b), 1);
                    let s_improve = math::round_up_to_n_decimal(f64::max(0.0, y_2b), 1);

                    fd_adj = math::round_up_to_n_decimal(pf, 1) / 100.0 * (1.0 - pf_improve / 100.0) * math::round_to_significant_digits(vd, 3) / (math::round_up_to_n_decimal(s, 1) * (1.0 + s_improve / 100.0));
                }
            }
        }
        fd_adj
    }


    pub fn determine_segment_los(&self, seg_num: usize, s_pl: f64, cap: i32) -> char {
        let mut los: char = 'F';
        
        let vd = self.segments[seg_num].get_flow_rate();
        let fd = self.segments[seg_num].get_followers_density();

        if s_pl >= 50.0 {
            if fd <= 2.0 { los = 'A' }
            else if fd > 2.0 && fd <= 4.0 { los = 'B' }
            else if fd > 4.0 && fd <= 8.0 { los = 'C' }
            else if fd > 8.0 && fd <= 12.0 { los = 'D' }
            else if fd > 12.0 { los = 'E' };
            if vd > cap as f64 { los = 'F' };
        } else {
            if fd <= 2.5 { los = 'A' }
            else if fd > 2.5 && fd <= 5.0 { los = 'B' }
            else if fd > 5.0 && fd <= 10.0 { los = 'C' }
            else if fd > 10.0 && fd <= 15.0 { los = 'D' }
            else if fd > 15.0 { los = 'E' }
            if vd > cap as f64 { los = 'F' };
        }

        los
    }


    pub fn determine_facility_los(&self, fd: f64, s_pl: f64) -> char {
        let mut los: char = 'F';
        
        if s_pl >= 50.0 {
            if fd <= 2.0 { los = 'A' }
            else if fd > 2.0 && fd <= 4.0 { los = 'B' }
            else if fd > 4.0 && fd <= 8.0 { los = 'C' }
            else if fd > 8.0 && fd <= 12.0 { los = 'D' }
            else if fd > 12.0 { los = 'E' };
        } else {
            if fd <= 2.5 { los = 'A' }
            else if fd > 2.5 && fd <= 5.0 { los = 'B' }
            else if fd > 5.0 && fd <= 10.0 { los = 'C' }
            else if fd > 10.0 && fd <= 15.0 { los = 'D' }
            else if fd > 15.0 { los = 'E' }
        }

        los
    }

}

