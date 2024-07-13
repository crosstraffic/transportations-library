use crate::hcm::twolanehighways::{SubSegment as LibSubSegment, Segment as LibSegment, TwoLaneHighways as LibTwoLaneHighways};

#[cfg(feature = "pybindings")]
use pyo3::prelude::*;

#[cfg(feature = "pybindings")]
use pyo3::types::PyList;

#[cfg(feature = "pybindings")]
#[pyclass]
#[derive(Debug, Clone)]
pub struct SubSegment {
    inner: LibSubSegment,
}

#[cfg(feature = "pybindings")]
#[pymethods]
impl SubSegment {

    #[new]
    pub fn new(length: f64, avg_speed: f64, hor_class: i32, design_rad: f64, central_angle: f64, sup_ele: f64) -> Self {
        SubSegment {

            inner: LibSubSegment::new(
                length,
                avg_speed,
                hor_class,
                design_rad,
                central_angle,
                sup_ele,
            ),
        }
    }
    
    pub fn get_length(&self) -> f64 {
        self.inner.get_length()
    }

    pub fn get_avg_speed(&self) -> f64 {
        self.inner.get_avg_speed()
    }

    pub fn get_hor_class(&self) -> i32 {
        self.inner.get_hor_class()
    }

    pub fn get_design_rad(&self) -> f64 {
        self.inner.get_design_rad()
    }

    pub fn get_central_angle(&self) -> f64 {
        self.inner.get_central_angle()
    }

    pub fn get_sup_ele(&self) -> f64 {
        self.inner.get_sup_ele()
    }


}



#[cfg(feature = "pybindings")]
#[pyclass]
#[derive(Debug, Clone)]
pub struct Segment {
    inner: LibSegment,
}

#[cfg(feature = "pybindings")]
#[pymethods]
impl Segment {

    #[new]
    pub fn new(passing_type: usize, length: f64, grade: f64, spl: f64, is_hc: bool, volume: f64, volume_op: f64, flow_rate: f64, flow_rate_o: f64, capacity: i32,
        ffs: f64, avg_speed: f64, vertical_class: i32, py_subsegments: Vec<SubSegment>, phf: f64, phv: f64, pf: f64, fd: f64, fd_mid: f64, hor_class: i32) -> Self {

        let subsegments: Vec<LibSubSegment> = py_subsegments.into_iter().map(|py_subseg| py_subseg.inner).collect();

        Segment {
            inner: LibSegment::new(
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
                fd_mid,
                hor_class,
            ),
        }
    }


    pub fn get_passing_type(&self) -> usize {
        self.inner.get_passing_type()
    }

    pub fn get_length(&self) -> f64 {
        self.inner.get_length()
    }

    pub fn get_grade(&self) -> f64 {
        self.inner.get_grade()
    }

    pub fn get_spl(&self) -> f64 {
        self.inner.get_spl()
    }

    pub fn get_is_hc(&self) -> bool {
        self.inner.get_is_hc()
    }

    pub fn get_volume(&self) -> f64 {
        self.inner.get_volume()
    }

    pub fn get_volume_op(&self) -> f64 {
        self.inner.get_volume_op()
    }

    pub fn get_flow_rate(&self) -> f64 {
        self.inner.get_flow_rate()
    }

    // // pub fn set_flow_rate(&mut self, flow_rate: f64) {
        
    // // }

    pub fn get_flow_rate_o(&self) -> f64 {
        self.inner.get_flow_rate_o()
    }

    // // pub fn set_flow_rate_o(&mut self, flow_rate_o: f64) {
        
    // // }

    pub fn get_capacity(&self) -> i32 {
        self.inner.get_capacity()
    }

    // // pub fn set_capacity(&mut self, capacity: i32) {
    // //     self.capacity = capacity
    // // }

    pub fn get_ffs(&self) -> f64 {
        self.inner.get_ffs()
    }

    // // pub fn set_ffs(&mut self, ffs: f64) {
    // //     self.ffs = ffs
    // // }

    pub fn get_avg_speed(&self) -> f64 {
        self.inner.get_avg_speed()
    }

    // // pub fn set_avg_speed(&mut self, avg_speed: f64) {
    // //     self.avg_speed = avg_speed
    // // }

    // pub fn get_subsegments(&self) -> JsValue {
    //     self.subsegs_to_js_value()
    // }

    // pub fn get_subsegments(&self) -> Vec<LibSubSegment> {
    //     &self.inner.subsegments
    // }
    pub fn get_subsegments<'py>(&self, py: Python<'py>) -> &'py PyList {
        let subsegments: Vec<Py<SubSegment>> = self.inner
            .subsegments
            .iter()
            .map(|subseg| {
                Py::new(
                    py,
                    SubSegment {
                        inner: subseg.clone(),
                    },
                )
                .unwrap()
            })
            .collect();
        PyList::new(py, subsegments)
    }

    pub fn get_vertical_class(&self) -> i32 {
        self.inner.get_vertical_class()
    }
    
    // // pub fn set_vertical_class(&mut self, vertical_class: i32) {
    // //     self.vertical_class = vertical_class
    // // }

    pub fn get_phf(&self) -> f64 {
        self.inner.get_phf()
    }

    pub fn get_phv(&self) -> f64 {
        self.inner.get_phv()
    }

    pub fn get_percent_followers(&self) -> f64 {
        self.inner.get_percent_followers()
    }

    // // pub fn set_percent_followers(&mut self, pf: f64) {
    // //    self.pf = pf
    // // }

    pub fn get_followers_density(&self) -> f64 {
        self.inner.get_followers_density()
    }

    // // pub fn set_followers_density(&mut self, fd: f64) {
    // //     self.fd = fd
    // // }

    pub fn get_followers_density_mid(&self) -> f64 {
        self.inner.get_followers_density_mid()
    }
    pub fn get_hor_class(&self) -> i32 {
        self.inner.get_hor_class()
    }
}



#[cfg(feature = "pybindings")]
#[pyclass]
#[derive(Debug, Clone)]
pub struct TwoLaneHighways{
    inner: LibTwoLaneHighways,
}

#[cfg(feature = "pybindings")]
#[pymethods]
impl TwoLaneHighways {

    #[new]
    pub fn new(py_segments: Vec<Segment>, lane_width: f64, shoulder_width: f64, apd: f64, pmhvfl: f64, l_de: f64) -> Self {

        let segments: Vec<LibSegment> = py_segments.into_iter().map(|py_seg| py_seg.inner).collect();

        TwoLaneHighways {
            inner: LibTwoLaneHighways::new(
                segments, 
                lane_width, 
                shoulder_width, 
                apd, 
                pmhvfl, 
                l_de
            ),
        }
    }

    // fn get_py_segments(&self) -> Vec<Segment> {
    //     // self.inner.segments.iter().map(|seg| Segment { inner: seg.clone() }).collect();
    //     self.inner.get_segments().into_iter().map(|py_seg| py_seg.inner).collect();
    // }

    // pub fn get_segments(&self) -> Vec<LibSegment> {
    //     &self.inner.segments
    // }
    pub fn get_segments<'py>(&self, py: Python<'py>) -> &'py PyList {
        let segments: Vec<Py<Segment>> = self.inner
            .segments
            .iter()
            .map(|seg| {
                Py::new(
                    py,
                    Segment {
                        inner: seg.clone(),
                    },
                )
                .unwrap()
            })
            .collect();
        PyList::new(py, segments)
    }
    
    pub fn identify_vertical_class(&mut self, seg_num: usize) -> Vec<f64> {
        let mut _min = 0.0;
        let mut _max = 0.0;
        (_min, _max) = self.inner.identify_vertical_class(seg_num);
        vec![_min, _max]
    }

    pub fn determine_demand_flow(&mut self, seg_num: usize) -> Vec<f64> {
        let (demand_flow_i , demand_flow_o, capacity) = self.inner.determine_demand_flow(seg_num);

        vec![demand_flow_i, demand_flow_o, capacity as f64]
    }

    pub fn determine_vertical_alignment(&mut self, seg_num: usize) -> i32 {
        self.inner.determine_vertical_alignment(seg_num)
    }

    pub fn determine_free_flow_speed(&mut self, seg_num: usize) -> f64 {
        self.inner.determine_free_flow_speed(seg_num)
    }

    pub fn estimate_average_speed(&mut self, seg_num: usize) -> Vec<f64> {
        let (res_s, seg_hor_class) = self.inner.estimate_average_speed(seg_num);
        vec![res_s, seg_hor_class as f64]
    }

    pub fn estimate_percent_followers(&mut self, seg_num: usize) -> f64 {
        self.inner.estimate_percent_followers(seg_num)
    }

    pub fn estimate_average_speed_sf(&mut self, seg_num: usize, length: f64, vd: f64, phv: f64, rad: f64, sup_ele: f64) -> Vec<f64> {
        let (s, hor_class) = self.inner.estimate_average_speed_sf(seg_num, length, vd, phv, rad, sup_ele);
        vec![s, hor_class as f64]
    }

    pub fn estimate_percent_followers_sf(&self, seg_num: usize, vd: f64, phv: f64) -> f64 {
        self.inner.estimate_percent_followers_sf(seg_num, vd, phv)
    }

    pub fn determine_follower_density_pl(&mut self, seg_num: usize) -> Vec<f64> {
        let (fd, fd_mid) = self.inner.determine_follower_density_pl(seg_num);
        vec![fd, fd_mid]
    }

    pub fn determine_follower_density_pc_pz(&mut self, seg_num: usize) -> f64 {
        self.inner.determine_follower_density_pc_pz(seg_num)
    }

    pub fn determine_adjustment_to_follower_density(&mut self, seg_num: usize) -> f64 {
        self.inner.determine_adjustment_to_follower_density(seg_num)
    }

    pub fn determine_segment_los(&self, seg_num: usize, s_pl: f64, cap: i32) -> char {
        self.inner.determine_segment_los(seg_num, s_pl, cap)
    }

    pub fn determine_facility_los(&self, fd: f64, s_pl: f64) -> char {
        self.inner.determine_facility_los(fd, s_pl)
    }
}


#[cfg(feature = "pybindings")]
#[pymodule]
fn transportations_library(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<SubSegment>()?;
    m.add_class::<Segment>()?;
    m.add_class::<TwoLaneHighways>()?;

    
    Ok(())
}