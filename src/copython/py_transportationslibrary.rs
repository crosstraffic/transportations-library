use crate::hcm::twolanehighways::{
    Segment as LibSegment,
    SubSegment as LibSubSegment,
    TwoLaneHighways as LibTwoLaneHighways,
};

#[cfg(feature = "with-python")]
use pyo3::prelude::*;
#[cfg(feature = "with-python")]
use pyo3::types::PyList;

#[cfg(feature = "with-python")]
#[pyclass]
#[derive(Debug, Clone)]
pub struct SubSegment {
    pub inner: LibSubSegment,
}

#[cfg(feature = "with-python")]
#[pymethods]
impl SubSegment {
    /// Create a new SubSegment.
    ///
    /// Args:
    ///     length: Length of the sub-segment in miles (default: 0.0)
    ///     avg_speed: Average travel speed in mph (default: 0.0)
    ///     hor_class: Horizontal alignment class (1-5, default: 1)
    ///     design_rad: Design radius in feet (default: 0.0)
    ///     central_angle: Central angle in degrees (default: 0.0)
    ///     sup_ele: Superelevation rate as decimal (default: 0.0)
    ///
    /// Returns:
    ///     SubSegment: A new sub-segment instance
    #[new]
    #[pyo3(signature = (length=None, avg_speed=None, hor_class=None, design_rad=None, central_angle=None, sup_ele=None))]
    pub fn new(
        length: Option<f64>,
        avg_speed: Option<f64>,
        hor_class: Option<i32>,
        design_rad: Option<f64>,
        central_angle: Option<f64>,
        sup_ele: Option<f64>,
    ) -> Self {
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

    /// Get the length of the sub-segment in miles.
    #[getter]
    pub fn get_length(&self) -> f64 {
        self.inner.get_length()
    }

    /// Get the average speed in mph.
    #[getter]
    pub fn get_avg_speed(&self) -> f64 {
        self.inner.get_avg_speed()
    }

    /// Get the horizontal alignment class (1-5).
    #[getter]
    pub fn get_hor_class(&self) -> i32 {
        self.inner.get_hor_class()
    }

    /// Get the design radius in feet.
    #[getter]
    pub fn get_design_rad(&self) -> f64 {
        self.inner.get_design_rad()
    }

    /// Get the central angle in degrees.
    #[getter]
    pub fn get_central_angle(&self) -> f64 {
        self.inner.get_central_angle()
    }

    /// Get the superelevation rate as decimal.
    #[getter]
    pub fn get_sup_ele(&self) -> f64 {
        self.inner.get_sup_ele()
    }

    /// String representation for debugging and display.
    pub fn __repr__(&self) -> String {
        format!(
            "SubSegment(length={:.2}, avg_speed={:.1}, hor_class={}, design_rad={:.0}, central_angle={:.1}, sup_ele={:.3})",
            self.get_length(),
            self.get_avg_speed(),
            self.get_hor_class(),
            self.get_design_rad(),
            self.get_central_angle(),
            self.get_sup_ele()
        )
    }

    /// Detailed string representation.
    pub fn __str__(&self) -> String {
        format!(
            "SubSegment: {:.2} miles, {:.1} mph average speed, horizontal class {}",
            self.get_length(),
            self.get_avg_speed(),
            self.get_hor_class()
        )
    }

    /// Check equality with another SubSegment.
    pub fn __eq__(&self, other: &Self) -> bool {
        (self.get_length() - other.get_length()).abs() < 1e-6
            && (self.get_avg_speed() - other.get_avg_speed()).abs() < 1e-6
            && self.get_hor_class() == other.get_hor_class()
    }
}


#[cfg(feature = "with-python")]
#[pyclass]
#[derive(Debug, Clone)]
pub struct Segment {
    pub inner: LibSegment,
}

#[cfg(feature = "with-python")]
#[pymethods]
impl Segment {
    #[new]
    #[pyo3(signature = (
        passing_type,
        length,
        grade,
        spl,
        is_hc=None,
        volume=None,
        volume_op=None,
        flow_rate=None,
        flow_rate_o=None,
        capacity=None,
        ffs=None,
        avg_speed=None,
        vertical_class=None,
        subsegments=None,
        phf=None,
        phv=None,
        pf=None,
        fd=None,
        fd_mid=None,
        hor_class=None
    ))]
    pub fn new(
        passing_type: usize,
        length: f64,
        grade: f64,
        spl: f64,
        is_hc: Option<bool>,
        volume: Option<f64>,
        volume_op: Option<f64>,
        flow_rate: Option<f64>,
        flow_rate_o: Option<f64>,
        capacity: Option<i32>,
        ffs: Option<f64>,
        avg_speed: Option<f64>,
        vertical_class: Option<i32>,
        subsegments: Option<Vec<SubSegment>>,
        phf: Option<f64>,
        phv: Option<f64>,
        pf: Option<f64>,
        fd: Option<f64>,
        fd_mid: Option<f64>,
        hor_class: Option<i32>,
    ) -> Self {
        // let lib_subsegments: Vec<LibSubSegment> = py_subsegments
        //     .into_iter()
        //     .map(|py_subseg| py_subseg.inner)
        //     .collect();
        let lib_subsegments: Option<Vec<LibSubSegment>> = if let Some(subsegments) = subsegments {
            Some(
                subsegments
                    .into_iter()
                    .map(|py_subseg| py_subseg.inner)
                    .collect()
            )
        } else {
            Some(Vec::new())
        };

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
                lib_subsegments,
                phf,
                phv,
                pf,
                fd,
                fd_mid,
                hor_class,
            ),
        }
    }

    #[getter]
    pub fn get_passing_type(&self) -> usize {
        self.inner.get_passing_type()
    }

    #[getter]
    pub fn get_length(&self) -> f64 {
        self.inner.get_length()
    }

    #[getter]
    pub fn get_grade(&self) -> f64 {
        self.inner.get_grade()
    }

    #[getter]
    pub fn get_spl(&self) -> f64 {
        self.inner.get_spl()
    }

    #[getter]
    pub fn get_is_hc(&self) -> bool {
        self.inner.get_is_hc()
    }

    #[getter]
    pub fn get_volume(&self) -> f64 {
        self.inner.get_volume()
    }

    #[getter]
    pub fn get_volume_op(&self) -> f64 {
        self.inner.get_volume_op()
    }

    #[getter]
    pub fn get_flow_rate(&self) -> f64 {
        self.inner.get_flow_rate()
    }

    // // pub fn set_flow_rate(&mut self, flow_rate: f64) {

    // // }

    #[getter]
    pub fn get_flow_rate_o(&self) -> f64 {
        self.inner.get_flow_rate_o()
    }

    // // pub fn set_flow_rate_o(&mut self, flow_rate_o: f64) {

    // // }

    #[getter]
    pub fn get_capacity(&self) -> i32 {
        self.inner.get_capacity()
    }

    // // pub fn set_capacity(&mut self, capacity: i32) {
    // //     self.capacity = capacity
    // // }

    #[getter]
    pub fn get_ffs(&self) -> f64 {
        self.inner.get_ffs()
    }

    // // pub fn set_ffs(&mut self, ffs: f64) {
    // //     self.ffs = ffs
    // // }

    #[getter]
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

    /// Get all subsegments as a Python list
    #[getter]
    pub fn get_subsegments<'py>(&self, py: Python<'py>) -> Bound<'py, PyList> {
        let subsegments: Vec<Py<SubSegment>> = self
            .inner
            .subsegments
            .as_ref()
            .unwrap_or(&Vec::new())
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
        PyList::new_bound(py, subsegments)
    }

    #[getter]
    pub fn get_vertical_class(&self) -> i32 {
        self.inner.get_vertical_class()
    }

    // // pub fn set_vertical_class(&mut self, vertical_class: i32) {
    // //     self.vertical_class = vertical_class
    // // }

    #[getter]
    pub fn get_phf(&self) -> f64 {
        self.inner.get_phf()
    }

    #[getter]
    pub fn get_phv(&self) -> f64 {
        self.inner.get_phv()
    }

    #[getter]
    pub fn get_percent_followers(&self) -> f64 {
        self.inner.get_percent_followers()
    }

    // // pub fn set_percent_followers(&mut self, pf: f64) {
    // //    self.pf = pf
    // // }

    #[getter]
    pub fn get_followers_density(&self) -> f64 {
        self.inner.get_followers_density()
    }

    // // pub fn set_followers_density(&mut self, fd: f64) {
    // //     self.fd = fd
    // // }

    #[getter]
    pub fn get_followers_density_mid(&self) -> f64 {
        self.inner.get_followers_density_mid()
    }

    #[getter]
    pub fn get_hor_class(&self) -> i32 {
        self.inner.get_hor_class()
    }

    pub fn __repr__(&self) -> String {
        format!(
            "Segment(passing_type={}, length={:.2}, grade={:.1}, volume={:.0}, capacity={})",
            self.get_passing_type(),
            self.get_length(),
            self.get_grade(),
            self.get_volume(),
            self.get_capacity()
        )
    }

    pub fn __str__(&self) -> String {
        format!(
            "{:.2}-mile segment, {:.1}% grade, {:.0} mph speed limit",
            self.get_length(),
            self.get_grade(),
            self.get_spl()
        )
    }
}

#[cfg(feature = "with-python")]
#[pyclass]
#[derive(Debug, Clone)]
pub struct TwoLaneHighways {
    pub inner: LibTwoLaneHighways,
}

#[cfg(feature = "with-python")]
#[pymethods]
impl TwoLaneHighways {
    #[new]
    #[pyo3(signature = (segments, lane_width=None, shoulder_width=None, apd=None, pmhvfl=None, l_de=None))]
    pub fn new(
        segments: Vec<Segment>,
        lane_width: Option<f64>,
        shoulder_width: Option<f64>,
        apd: Option<f64>,
        pmhvfl: Option<f64>,
        l_de: Option<f64>,
    ) -> Self {
        let segments: Vec<LibSegment> =
            segments.into_iter().map(|py_seg| py_seg.inner).collect();

        TwoLaneHighways {
            inner: LibTwoLaneHighways::new(segments, lane_width, shoulder_width, apd, pmhvfl, l_de),
        }
    }

    // fn get_py_segments(&self) -> Vec<Segment> {
    //     // self.inner.segments.iter().map(|seg| Segment { inner: seg.clone() }).collect();
    //     self.inner.get_segments().into_iter().map(|py_seg| py_seg.inner).collect();
    // }

    // pub fn get_segments(&self) -> Vec<LibSegment> {
    //     &self.inner.segments
    // }
    /// Get all segments as a Python list.
    #[getter]
    pub fn get_segments<'py>(&self, py: Python<'py>) -> Bound<'py, PyList> {
        let segments: Vec<Py<Segment>> = self
            .inner
            .segments
            .iter()
            .map(|seg| Py::new(py, Segment { inner: seg.clone() }).unwrap())
            .collect();
        PyList::new_bound(py, segments)
    }

    /// Get the number of segments in the facility.
    #[getter]
    pub fn num_segments(&self) -> usize {
        self.inner.segments.len()
    }

    /// Get the total length of the facility.
    #[getter]
    pub fn total_length(&self) -> f64 {
        self.inner.segments.iter().map(|s| s.get_length()).sum()
    }

    pub fn identify_vertical_class(&mut self, seg_num: usize) -> Vec<f64> {
        let mut _min = 0.0;
        let mut _max = 0.0;
        (_min, _max) = self.inner.identify_vertical_class(seg_num);
        vec![_min, _max]
    }

    pub fn determine_demand_flow(&mut self, seg_num: usize) -> Vec<f64> {
        let (demand_flow_i, demand_flow_o, capacity) = self.inner.determine_demand_flow(seg_num);

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

    pub fn estimate_average_speed_sf(
        &mut self,
        seg_num: usize,
        length: f64,
        vd: f64,
        phv: f64,
        rad: f64,
        sup_ele: f64,
    ) -> Vec<f64> {
        let (s, hor_class) = self
            .inner
            .estimate_average_speed_sf(seg_num, length, vd, phv, rad, sup_ele);
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


    pub fn __repr__(&self) -> String {
        format!(
            "TwoLaneHighways(segments={}, total_length={:.2} miles)",
            self.num_segments(),
            self.total_length()
        )
    }

    pub fn __str__(&self) -> String {
        format!(
            "Two-lane highway facility with {} segments ({:.2} miles total)",
            self.num_segments(),
            self.total_length()
        )
    }

    /// Get a summary of the facility characteristics.
    pub fn summary(&self) -> String {
        let total_len = self.total_length();
        let num_segs = self.num_segments();
        let avg_seg_len = if num_segs > 0 { total_len / num_segs as f64 } else { 0.0 };
        
        format!(
            "Highway Facility Summary:\n  Total Length: {:.2} miles\n  Number of Segments: {}\n  Average Segment Length: {:.2} miles",
            total_len, num_segs, avg_seg_len
        )
    }
}

#[cfg(feature = "with-python")]
#[pymodule]
fn transportations_library(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<SubSegment>()?;
    m.add_class::<Segment>()?;
    m.add_class::<TwoLaneHighways>()?;

    // Add module-level documentation
    m.add("__doc__", 
        "Transportation analysis library for two-lane highways using HCM methodology.\n\n\
        This library provides tools for analyzing two-lane highway capacity and level of service\n\
        according to the Highway Capacity Manual (HCM) procedures.\n\n\
        Main Classes:\n\
        - SubSegment: Represents a sub-section of highway with uniform characteristics\n\
        - Segment: Represents a highway segment for analysis\n\
        - TwoLaneHighways: Main analysis class for highway facilities\n\
        Example Usage:\n\
        >>> from transportations_library import Segment, TwoLaneHighways\n\
        >>> segment = Segment(passing_type=1, length=2.5, grade=3.0, spl=55.0)\n\
        >>> highway = TwoLaneHighways([segment])\n"
    )?;
    m.add("__version__", "0.1.3")?;

    Ok(())
}
