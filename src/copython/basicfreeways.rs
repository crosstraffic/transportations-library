use crate::hcm::basicfreeways::BasicFreeways as LibBasicFreeways;
use crate::hcm::common::CityType;
use crate::hcm::managed_lanes::{ManagedLaneSegment as LibManagedLaneSegment, ManagedLaneType};
use pyo3::exceptions::PyValueError;
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
        highway_type=None, city_type=None, sut_percentage=None
    ))]
    pub fn new(
        bffs: Option<f64>,
        lane_width: Option<f64>,
        lane_count: Option<u32>,
        lc_r: Option<f64>,
        lc_l: Option<f64>,
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
        sut_percentage: Option<u32>,
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
        if let Some(v) = sut_percentage {
            inner.sut_percentage = v;
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
    pub fn run_operational_analysis(&mut self) -> PyResult<String> {
        let los = self.inner
            .run_operational_analysis()
            .map_err(pyo3::exceptions::PyValueError::new_err)?;
        let los: char = los.into();
        Ok(los.to_string())
    }

    /// Set the target LOS used by design analysis (Exhibit 12-37/12-38 lookup).
    pub fn set_target_los(&mut self, los: &str) -> PyResult<()> {
        let letter = los.trim().to_ascii_uppercase();
        let letter = letter.chars().next().ok_or_else(|| {
            pyo3::exceptions::PyValueError::new_err("target LOS must be one of A-F")
        })?;
        if !('A'..='F').contains(&letter) {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "target LOS must be one of A-F, got {los:?}"
            )));
        }
        self.inner.los = Some(letter.into());
        Ok(())
    }

    /// The passenger-car equivalent E_T selected for the current inputs, once demand has been
    /// adjusted (Exhibit 12-25 for general terrain, 12-26/27/28 for a specific upgrade).
    pub fn e_t(&self) -> Option<f64> {
        self.inner.e_t
    }

    /// The heavy-vehicle adjustment factor f_HV (Equation 12-10).
    pub fn f_hv(&self) -> f64 {
        self.inner.phv
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
    /// Errors when the heavy-vehicle inputs fall outside the Exhibit 12-25/12-26/12-27/12-28 domain.
    pub fn estimate_demand_volume(&mut self) -> PyResult<f64> {
        self.inner
            .estimate_demand_volume()
            .map_err(pyo3::exceptions::PyValueError::new_err)
    }

    /// Design analysis: lanes required for the target LOS (Equations 12-21 through 12-23).
    /// Returns the rounded-up lane count and the unrounded value it came from.
    pub fn estimate_number_of_lanes(&mut self) -> PyResult<(u32, f64)> {
        self.inner
            .estimate_number_of_lanes()
            .map_err(pyo3::exceptions::PyValueError::new_err)
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

#[pyclass]
#[derive(Debug, Clone)]
pub struct ManagedLanes {
    pub inner: LibManagedLaneSegment,
}

#[pymethods]
impl ManagedLanes {
    /// Create a basic managed lane segment (HCM Chapter 12, Section 4).
    ///
    /// Args:
    ///     lane_type: "continuous_access" (default), "buffer1", "buffer2",
    ///         "barrier1", or "barrier2" (Exhibit 12-30 segment types)
    ///     ffs: free-flow speed, mi/h
    ///     demand: 15-min average flow rate v_p, pc/h/ln
    ///     gp_density: adjacent general purpose lane density K_GP, pc/mi/ln
    ///     caf, saf: capacity/speed adjustment factors
    #[new]
    #[pyo3(signature = (lane_type=None, ffs=None, demand=None, gp_density=None, caf=None, saf=None))]
    pub fn new(
        lane_type: Option<String>,
        ffs: Option<f64>,
        demand: Option<f64>,
        gp_density: Option<f64>,
        caf: Option<f64>,
        saf: Option<f64>,
    ) -> PyResult<Self> {
        let lt = match lane_type.as_deref().map(str::to_lowercase).as_deref() {
            None | Some("continuous_access") | Some("continuous") => {
                ManagedLaneType::ContinuousAccess
            }
            Some("buffer1") => ManagedLaneType::Buffer1,
            Some("buffer2") => ManagedLaneType::Buffer2,
            Some("barrier1") => ManagedLaneType::Barrier1,
            Some("barrier2") => ManagedLaneType::Barrier2,
            Some(other) => {
                return Err(PyValueError::new_err(format!("unknown lane_type: {other}")))
            }
        };
        let mut inner = LibManagedLaneSegment::new(lt, ffs.unwrap_or(65.0));
        if let Some(v) = demand {
            inner.set_demand(v);
        }
        if let Some(v) = gp_density {
            inner.set_gp_density(v);
        }
        if let Some(v) = caf {
            inner.set_caf(v);
        }
        if let Some(v) = saf {
            inner.set_saf(v);
        }
        Ok(ManagedLanes { inner })
    }

    // ── HCM Ch.12 Section 4 step methods ───────────────────────────────

    /// Breakpoint BP (pc/h/ln) - Equation 12-13.
    pub fn calculate_breakpoint(&mut self) -> f64 {
        self.inner.calculate_ffs_adj();
        self.inner.calculate_breakpoint()
    }

    /// Adjusted capacity c_adj (pc/h/ln) - Equation 12-14.
    pub fn calculate_capacity(&mut self) -> f64 {
        self.inner.calculate_ffs_adj();
        self.inner.calculate_capacity()
    }

    /// Space mean speed S_ML (mi/h) - Equation 12-12.
    pub fn calculate_speed(&mut self) -> f64 {
        self.inner.calculate_speed()
    }

    /// Density (pc/mi/ln).
    pub fn calculate_density(&mut self) -> f64 {
        self.inner.calculate_density()
    }

    /// Level of service letter (Exhibit 12-15 criteria).
    pub fn determine_los(&mut self) -> String {
        let los: char = self.inner.determine_los().into();
        los.to_string()
    }

    /// Run the full managed lane analysis; returns the LOS letter.
    pub fn run_analysis(&mut self) -> String {
        let los: char = self.inner.run_analysis().into();
        los.to_string()
    }

    pub fn set_demand(&mut self, v_p: f64) {
        self.inner.set_demand(v_p);
    }

    pub fn set_gp_density(&mut self, k_gp: f64) {
        self.inner.set_gp_density(k_gp);
    }

    // ── Getters ─────────────────────────────────────────────────────────

    #[getter]
    pub fn breakpoint(&self) -> f64 {
        self.inner.breakpoint
    }

    #[getter]
    pub fn capacity(&self) -> f64 {
        self.inner.capacity_adj
    }

    #[getter]
    pub fn speed(&self) -> f64 {
        self.inner.speed
    }

    #[getter]
    pub fn density(&self) -> f64 {
        self.inner.density
    }

    #[getter]
    pub fn los(&self) -> Option<String> {
        self.inner.los.map(|l| {
            let c: char = l.into();
            c.to_string()
        })
    }

    /// Whether the segment type is subject to GP-lane friction
    /// (continuous access and Buffer 1 types).
    pub fn has_friction_effect(&self) -> bool {
        self.inner.has_friction_effect()
    }

    /// Whether friction is active (K_GP > 35 pc/mi/ln on a friction type).
    pub fn is_friction_active(&self) -> bool {
        self.inner.is_friction_active()
    }

    pub fn __repr__(&self) -> String {
        format!(
            "ManagedLanes(type={:?}, ffs={:.0}, demand={:.0}, gp_density={:.1})",
            self.inner.lane_type, self.inner.ffs, self.inner.v_p, self.inner.k_gp
        )
    }
}

pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<BasicFreeways>()?;
    m.add_class::<ManagedLanes>()?;
    Ok(())
}
