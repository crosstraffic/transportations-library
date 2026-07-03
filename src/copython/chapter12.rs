use crate::hcm::basicfreeways::BasicFreeways as LibBasicFreeways;
use crate::hcm::common::CityType;
use pyo3::prelude::*;

#[pyclass]
#[derive(Debug, Clone)]
pub struct BasicFreeways {
    pub inner: LibBasicFreeways,
}

#[cfg(feature = "with-python")]
#[pymethods]
impl BasicFreeways {
    /// Create a basic-freeway (HCM Chapter 12) segment.
    ///
    /// Only the inputs provided are set; the rest keep HCM defaults. The
    /// analysis chain (FFS → capacity → demand → speed → density → LOS) reads
    /// these fields, so the executor mirrors the curated AFFECTS graph.
    #[new]
    #[pyo3(signature = (
        bffs=None, lane_width=None, lane_count=None, lc_r=None, lc_l=None,
        trd=None, apd=None, grade=None, terrain_type=None, speed_limit=None,
        phf=None, p_t=None, demand_flow_i=None, length=None,
        highway_type=None, city_type=None
    ))]
    pub fn new(
        bffs: Option<f64>,
        lane_width: Option<f64>,
        lane_count: Option<u32>,
        lc_r: Option<u32>,
        lc_l: Option<u32>,
        trd: Option<u32>,
        apd: Option<u32>,
        grade: Option<f64>,
        terrain_type: Option<String>,
        speed_limit: Option<u32>,
        phf: Option<f64>,
        p_t: Option<f64>,
        demand_flow_i: Option<f64>,
        length: Option<f64>,
        highway_type: Option<String>,
        city_type: Option<String>,
    ) -> Self {
        let mut inner = LibBasicFreeways::new();
        if let Some(v) = bffs {
            inner.bffs = v;
        }
        if lane_width.is_some() {
            inner.lw = lane_width;
        }
        if let Some(v) = lane_count {
            inner.lane_count = v;
        }
        if let Some(v) = lc_r {
            inner.lc_r = v;
        }
        if let Some(v) = lc_l {
            inner.lc_l = v;
        }
        if let Some(v) = trd {
            inner.trd = v;
        }
        if let Some(v) = apd {
            inner.apd = v;
        }
        if let Some(v) = grade {
            inner.grade = v;
        }
        if terrain_type.is_some() {
            inner.terrain_type = terrain_type;
        }
        if let Some(v) = speed_limit {
            inner.speed_limit = v;
        }
        if let Some(v) = phf {
            inner.phf = v;
        }
        if p_t.is_some() {
            inner.p_t = p_t;
        }
        if let Some(v) = demand_flow_i {
            inner.demand_flow_i = v;
        }
        if let Some(v) = length {
            inner.length = v;
        }
        if let Some(v) = highway_type {
            inner.highway_type = v;
        }
        if let Some(ct) = city_type {
            inner.city_type = match ct.to_lowercase().as_str() {
                "rural" => CityType::Rural,
                _ => CityType::Urban,
            };
        }
        BasicFreeways { inner }
    }

    /// Run the full HCM Ch.12 operational analysis; returns the LOS letter.
    /// Populates ffs, capacity, speed, density, and v/c ratio.
    pub fn run_operational_analysis(&mut self) -> String {
        let los: char = self.inner.run_operational_analysis().into();
        los.to_string()
    }

    pub fn determine_free_flow_speed(&mut self) -> f64 {
        self.inner.determine_free_flow_speed()
    }

    pub fn ffs(&self) -> f64 {
        self.inner.get_ffs()
    }

    pub fn capacity(&self) -> f64 {
        self.inner.get_capacity()
    }

    pub fn adjusted_capacity(&self) -> f64 {
        self.inner.get_adjusted_capacity()
    }

    pub fn speed(&self) -> f64 {
        self.inner.get_speed()
    }

    pub fn density(&self) -> f64 {
        self.inner.get_density()
    }

    pub fn vc_ratio(&self) -> f64 {
        self.inner.get_vc_ratio()
    }

    pub fn lane_count(&self) -> u32 {
        self.inner.get_lane_count()
    }

    // ─── HCM Ch.12 step methods (stateful; call in analysis order) ──────────

    /// Step 3: base + adjusted capacity (pc/h/ln). Errors if lane width is infeasible.
    pub fn estimate_capacity(&mut self) -> PyResult<f64> {
        self.inner
            .estimate_capacity()
            .map(|c| c as f64)
            .map_err(pyo3::exceptions::PyValueError::new_err)
    }

    /// Step 4: convert demand to per-lane flow rate v_p (pc/h/ln).
    pub fn estimate_demand_volume(&mut self) -> f64 {
        self.inner.estimate_demand_volume()
    }

    /// Step 5a: space mean speed via the speed-flow curve (mi/h).
    pub fn calculate_speed(&mut self) -> f64 {
        self.inner.calculate_speed()
    }

    /// Step 5b: density D = v_p / S (pc/mi/ln).
    pub fn estimate_density(&mut self) -> f64 {
        self.inner.estimate_density()
    }

    /// Volume-to-capacity ratio.
    pub fn calculate_vc_ratio(&mut self) -> f64 {
        self.inner.calculate_vc_ratio()
    }

    /// Step 6: segment level of service (A-F) from density / v-c ratio.
    pub fn determine_segment_los(&mut self) -> String {
        let los: char = self.inner.determine_segment_los().into();
        los.to_string()
    }

    pub fn __repr__(&self) -> String {
        format!(
            "BasicFreeways(lanes={}, lw={:?}, bffs={:.0}, demand={:.0})",
            self.inner.lane_count, self.inner.lw, self.inner.bffs, self.inner.demand_flow_i
        )
    }
}

pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<BasicFreeways>()?;
    Ok(())
}
