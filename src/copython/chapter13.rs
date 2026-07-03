//! Python bindings for HCM Chapter 13 (Freeway Weaving Segments).

use crate::hcm::chapter13::weaving::{
    FacilityType, TerrainType, WeavingSegment as LibWeavingSegment, WeavingType,
};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

#[pyclass]
#[derive(Debug, Clone)]
pub struct WeavingSegment {
    pub inner: LibWeavingSegment,
}

#[pymethods]
impl WeavingSegment {
    /// Create an HCM Chapter 13 weaving segment.
    ///
    /// Args:
    ///     weaving_type: "one_sided" (default) or "two_sided"
    ///     facility_type: "freeway" (default) or "multilane" (multilane/C-D)
    ///     length_short: short length L_S, ft
    ///     num_lanes: lanes within the weaving segment N
    ///     num_weaving_lanes: N_WL (2 or 3 one-sided; 0 two-sided)
    ///     ffs: free-flow speed, mi/h
    ///     v_ff, v_fr, v_rf, v_rr: component demand volumes, veh/h
    ///     phf: peak hour factor
    ///     heavy_vehicle_pct: heavy vehicle proportion (decimal)
    ///     terrain: "level" (default), "rolling", or "mountainous"
    ///     lc_rf, lc_fr, lc_rr: minimum lane changes per weaving vehicle
    ///     interchange_density: ID, int/mi
    ///     basic_freeway_capacity: c_IFL, pc/h/ln
    ///     caf, saf: capacity/speed adjustment factors
    #[new]
    #[pyo3(signature = (
        weaving_type=None, facility_type=None, length_short=None, num_lanes=None,
        num_weaving_lanes=None, ffs=None, v_ff=None, v_fr=None, v_rf=None,
        v_rr=None, phf=None, heavy_vehicle_pct=None, terrain=None, lc_rf=None,
        lc_fr=None, lc_rr=None, interchange_density=None,
        basic_freeway_capacity=None, caf=None, saf=None
    ))]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        weaving_type: Option<String>,
        facility_type: Option<String>,
        length_short: Option<f64>,
        num_lanes: Option<u32>,
        num_weaving_lanes: Option<u32>,
        ffs: Option<f64>,
        v_ff: Option<f64>,
        v_fr: Option<f64>,
        v_rf: Option<f64>,
        v_rr: Option<f64>,
        phf: Option<f64>,
        heavy_vehicle_pct: Option<f64>,
        terrain: Option<String>,
        lc_rf: Option<u32>,
        lc_fr: Option<u32>,
        lc_rr: Option<u32>,
        interchange_density: Option<f64>,
        basic_freeway_capacity: Option<f64>,
        caf: Option<f64>,
        saf: Option<f64>,
    ) -> PyResult<Self> {
        let mut inner = LibWeavingSegment::new();

        if let Some(wt) = weaving_type {
            inner.weaving_type = match wt.to_lowercase().as_str() {
                "one_sided" | "onesided" | "one-sided" => WeavingType::OneSided,
                "two_sided" | "twosided" | "two-sided" => WeavingType::TwoSided,
                other => {
                    return Err(PyValueError::new_err(format!(
                        "unknown weaving_type: {other}"
                    )))
                }
            };
        }
        if let Some(ft) = facility_type {
            inner.facility_type = match ft.to_lowercase().as_str() {
                "freeway" => FacilityType::Freeway,
                "multilane" | "cd" | "multilane_or_cd" => FacilityType::MultilaneOrCD,
                other => {
                    return Err(PyValueError::new_err(format!(
                        "unknown facility_type: {other}"
                    )))
                }
            };
        }
        if let Some(t) = terrain {
            inner.terrain = match t.to_lowercase().as_str() {
                "level" => TerrainType::Level,
                "rolling" => TerrainType::Rolling,
                "mountainous" => TerrainType::Mountainous,
                other => return Err(PyValueError::new_err(format!("unknown terrain: {other}"))),
            };
        }
        if let Some(v) = length_short {
            inner.length_short = v;
        }
        if let Some(v) = num_lanes {
            inner.num_lanes = v;
        }
        if let Some(v) = num_weaving_lanes {
            inner.num_weaving_lanes = v;
        }
        if let Some(v) = ffs {
            inner.ffs = v;
        }
        if let Some(v) = v_ff {
            inner.v_ff = v;
        }
        if let Some(v) = v_fr {
            inner.v_fr = v;
        }
        if let Some(v) = v_rf {
            inner.v_rf = v;
        }
        if let Some(v) = v_rr {
            inner.v_rr = v;
        }
        if let Some(v) = phf {
            inner.phf = v;
        }
        if let Some(v) = heavy_vehicle_pct {
            inner.heavy_vehicle_pct = v;
        }
        if let Some(v) = lc_rf {
            inner.lc_rf = v;
        }
        if let Some(v) = lc_fr {
            inner.lc_fr = v;
        }
        if let Some(v) = lc_rr {
            inner.lc_rr = v;
        }
        if let Some(v) = interchange_density {
            inner.interchange_density = v;
        }
        if let Some(v) = basic_freeway_capacity {
            inner.basic_freeway_capacity = v;
        }
        if let Some(v) = caf {
            inner.caf = v;
        }
        if let Some(v) = saf {
            inner.saf = v;
        }

        Ok(WeavingSegment { inner })
    }

    // ── HCM Ch.13 step methods (stateful; call in analysis order) ──────────

    /// Step 2: demand flows under equivalent ideal conditions (Eq. 13-1).
    /// Returns (v_W, v_NW, v) in pc/h.
    pub fn determine_demand_flow(&mut self) -> (f64, f64, f64) {
        self.inner.determine_demand_flow()
    }

    /// Step 3: minimum lane-changing rate LC_MIN (lc/h) - Eqs. 13-2/13-3.
    pub fn determine_configuration_characteristics(&mut self) -> f64 {
        self.inner.determine_configuration_characteristics()
    }

    /// Step 4: maximum weaving length L_MAX (ft) - Eq. 13-4.
    pub fn determine_max_weaving_length(&mut self) -> f64 {
        self.inner.determine_max_weaving_length()
    }

    /// Step 5: weaving segment capacity (veh/h) - Eqs. 13-5..13-10.
    pub fn determine_capacity(&mut self) -> f64 {
        self.inner.determine_capacity()
    }

    /// Step 6: total lane-changing rate LC_ALL (lc/h) - Eqs. 13-11..13-17.
    pub fn determine_lane_changing_rates(&mut self) -> f64 {
        self.inner.determine_lane_changing_rates()
    }

    /// Step 7: speeds (S_W, S_NW, S) in mi/h - Eqs. 13-18..13-22.
    pub fn estimate_speed(&mut self) -> (f64, f64, f64) {
        self.inner.estimate_speed()
    }

    /// Step 8a: density (pc/mi/ln) - Eq. 13-23.
    pub fn determine_density(&mut self) -> f64 {
        self.inner.determine_density()
    }

    /// Step 8b: level of service letter - Exhibit 13-6.
    pub fn determine_los(&mut self) -> String {
        let los: char = self.inner.determine_los().into();
        los.to_string()
    }

    /// Run the full Chapter 13 analysis (Steps 2-8); returns the LOS letter.
    pub fn run_analysis(&mut self) -> String {
        let los: char = self.inner.run_analysis().into();
        los.to_string()
    }

    // ── Getters ─────────────────────────────────────────────────────────

    #[getter]
    pub fn flow_weaving(&self) -> f64 {
        self.inner.get_flow_weaving()
    }

    #[getter]
    pub fn flow_nonweaving(&self) -> f64 {
        self.inner.get_flow_nonweaving()
    }

    #[getter]
    pub fn flow_total(&self) -> f64 {
        self.inner.get_flow_total()
    }

    #[getter]
    pub fn volume_ratio(&self) -> f64 {
        self.inner.get_volume_ratio()
    }

    #[getter]
    pub fn lc_min(&self) -> f64 {
        self.inner.get_lc_min()
    }

    #[getter]
    pub fn l_max(&self) -> f64 {
        self.inner.get_l_max()
    }

    #[getter]
    pub fn is_weaving(&self) -> bool {
        self.inner.is_weaving_segment()
    }

    #[getter]
    pub fn capacity(&self) -> f64 {
        self.inner.get_capacity()
    }

    #[getter]
    pub fn vc_ratio(&self) -> f64 {
        self.inner.get_vc_ratio()
    }

    #[getter]
    pub fn lc_all(&self) -> f64 {
        self.inner.get_lc_all()
    }

    #[getter]
    pub fn speed_weaving(&self) -> f64 {
        self.inner.get_speed_weaving()
    }

    #[getter]
    pub fn speed_nonweaving(&self) -> f64 {
        self.inner.get_speed_nonweaving()
    }

    #[getter]
    pub fn speed_avg(&self) -> f64 {
        self.inner.get_speed_avg()
    }

    #[getter]
    pub fn density(&self) -> f64 {
        self.inner.get_density()
    }

    #[getter]
    pub fn los(&self) -> Option<String> {
        self.inner.get_los().map(|l| {
            let c: char = l.into();
            c.to_string()
        })
    }

    pub fn __repr__(&self) -> String {
        format!(
            "WeavingSegment(type={:?}, L_S={:.0} ft, N={}, N_WL={}, ffs={:.0})",
            self.inner.weaving_type,
            self.inner.length_short,
            self.inner.num_lanes,
            self.inner.num_weaving_lanes,
            self.inner.ffs
        )
    }
}

pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<WeavingSegment>()?;
    Ok(())
}
