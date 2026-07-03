//! Python bindings for HCM Chapter 14 (Freeway Merge and Diverge Segments).

use crate::hcm::chapter14::merge_diverge::{
    AdjacentRampType, RampLanes, RampSegment as LibRampSegment, RampSide, RampType, TerrainType,
};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

fn parse_adjacent(s: &str) -> PyResult<AdjacentRampType> {
    match s.to_lowercase().as_str() {
        "none" => Ok(AdjacentRampType::None),
        "on_ramp" | "onramp" | "on-ramp" | "on" => Ok(AdjacentRampType::OnRamp),
        "off_ramp" | "offramp" | "off-ramp" | "off" => Ok(AdjacentRampType::OffRamp),
        other => Err(PyValueError::new_err(format!(
            "unknown adjacent ramp type: {other}"
        ))),
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct RampSegment {
    pub inner: LibRampSegment,
}

#[pymethods]
impl RampSegment {
    /// Create an HCM Chapter 14 ramp-freeway junction.
    ///
    /// Args:
    ///     ramp_type: "on_ramp" (default), "off_ramp", "major_merge", or "major_diverge"
    ///     ramp_side: "right" (default) or "left"
    ///     ramp_lanes: 1 (default) or 2
    ///     freeway_lanes: directional freeway lanes (2-5)
    ///     freeway_ffs, ramp_ffs: free-flow speeds, mi/h
    ///     accel_lane_length / decel_lane_length: L_A / L_D, ft
    ///     accel_lane_length2 / decel_lane_length2: second lane lengths for
    ///         two-lane ramps (Eqs. 14-25/14-26), ft
    ///     freeway_demand, ramp_demand: demand volumes, veh/h
    ///     phf: peak hour factor
    ///     heavy_vehicle_pct: freeway heavy-vehicle proportion (decimal)
    ///     ramp_heavy_vehicle_pct: ramp heavy-vehicle proportion (decimal)
    ///     terrain: "level" (default), "rolling", or "mountainous"
    ///     adjacent_upstream / adjacent_downstream: "none", "on_ramp", "off_ramp"
    ///     upstream_distance / downstream_distance: ramp spacing, ft
    ///     upstream_ramp_flow / downstream_ramp_flow: adjacent demand, veh/h
    ///     caf, saf: capacity/speed adjustment factors
    #[new]
    #[pyo3(signature = (
        ramp_type=None, ramp_side=None, ramp_lanes=None, freeway_lanes=None,
        freeway_ffs=None, ramp_ffs=None, accel_lane_length=None,
        accel_lane_length2=None, decel_lane_length=None, decel_lane_length2=None,
        freeway_demand=None, ramp_demand=None, phf=None, heavy_vehicle_pct=None,
        ramp_heavy_vehicle_pct=None, terrain=None, adjacent_upstream=None,
        upstream_distance=None, upstream_ramp_flow=None, adjacent_downstream=None,
        downstream_distance=None, downstream_ramp_flow=None, caf=None, saf=None
    ))]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        ramp_type: Option<String>,
        ramp_side: Option<String>,
        ramp_lanes: Option<u32>,
        freeway_lanes: Option<u32>,
        freeway_ffs: Option<f64>,
        ramp_ffs: Option<f64>,
        accel_lane_length: Option<f64>,
        accel_lane_length2: Option<f64>,
        decel_lane_length: Option<f64>,
        decel_lane_length2: Option<f64>,
        freeway_demand: Option<f64>,
        ramp_demand: Option<f64>,
        phf: Option<f64>,
        heavy_vehicle_pct: Option<f64>,
        ramp_heavy_vehicle_pct: Option<f64>,
        terrain: Option<String>,
        adjacent_upstream: Option<String>,
        upstream_distance: Option<f64>,
        upstream_ramp_flow: Option<f64>,
        adjacent_downstream: Option<String>,
        downstream_distance: Option<f64>,
        downstream_ramp_flow: Option<f64>,
        caf: Option<f64>,
        saf: Option<f64>,
    ) -> PyResult<Self> {
        let mut inner = LibRampSegment::new();

        if let Some(rt) = ramp_type {
            inner.ramp_type = match rt.to_lowercase().as_str() {
                "on_ramp" | "onramp" | "on-ramp" | "merge" => RampType::OnRamp,
                "off_ramp" | "offramp" | "off-ramp" | "diverge" => RampType::OffRamp,
                "major_merge" => RampType::MajorMerge,
                "major_diverge" => RampType::MajorDiverge,
                other => {
                    return Err(PyValueError::new_err(format!("unknown ramp_type: {other}")))
                }
            };
        }
        if let Some(rs) = ramp_side {
            inner.ramp_side = match rs.to_lowercase().as_str() {
                "right" => RampSide::Right,
                "left" => RampSide::Left,
                other => {
                    return Err(PyValueError::new_err(format!("unknown ramp_side: {other}")))
                }
            };
        }
        if let Some(rl) = ramp_lanes {
            inner.ramp_lanes = match rl {
                1 => RampLanes::OneLane,
                2 => RampLanes::TwoLane,
                other => {
                    return Err(PyValueError::new_err(format!(
                        "ramp_lanes must be 1 or 2, got {other}"
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
        if let Some(a) = adjacent_upstream {
            inner.adjacent_upstream = parse_adjacent(&a)?;
        }
        if let Some(a) = adjacent_downstream {
            inner.adjacent_downstream = parse_adjacent(&a)?;
        }
        if let Some(v) = freeway_lanes {
            inner.freeway_lanes = v;
        }
        if let Some(v) = freeway_ffs {
            inner.freeway_ffs = v;
        }
        if let Some(v) = ramp_ffs {
            inner.ramp_ffs = v;
        }
        if accel_lane_length.is_some() {
            inner.accel_lane_length = accel_lane_length;
        }
        if accel_lane_length2.is_some() {
            inner.accel_lane_length2 = accel_lane_length2;
        }
        if decel_lane_length.is_some() {
            inner.decel_lane_length = decel_lane_length;
        }
        if decel_lane_length2.is_some() {
            inner.decel_lane_length2 = decel_lane_length2;
        }
        if let Some(v) = freeway_demand {
            inner.freeway_demand = v;
        }
        if let Some(v) = ramp_demand {
            inner.ramp_demand = v;
        }
        if let Some(v) = phf {
            inner.phf = v;
        }
        if let Some(v) = heavy_vehicle_pct {
            inner.heavy_vehicle_pct = v;
        }
        if ramp_heavy_vehicle_pct.is_some() {
            inner.ramp_heavy_vehicle_pct = ramp_heavy_vehicle_pct;
        }
        if upstream_distance.is_some() {
            inner.upstream_distance = upstream_distance;
        }
        if upstream_ramp_flow.is_some() {
            inner.upstream_ramp_flow = upstream_ramp_flow;
        }
        if downstream_distance.is_some() {
            inner.downstream_distance = downstream_distance;
        }
        if downstream_ramp_flow.is_some() {
            inner.downstream_ramp_flow = downstream_ramp_flow;
        }
        if let Some(v) = caf {
            inner.caf = v;
        }
        if let Some(v) = saf {
            inner.saf = v;
        }

        Ok(RampSegment { inner })
    }

    // ── HCM Ch.14 step methods (stateful; call in analysis order) ──────────

    /// Step 1: demand flows (v_F, v_R) in pc/h - Eq. 14-1.
    pub fn determine_demand_flow(&mut self) -> (f64, f64) {
        self.inner.determine_demand_flow()
    }

    /// Step 2: flow in Lanes 1 and 2, v_12 (pc/h) - Eqs. 14-2..14-19.
    pub fn estimate_v12(&mut self) -> f64 {
        self.inner.estimate_v12()
    }

    /// Step 3: adjusted freeway capacity (pc/h) and capacity checks
    /// (Exhibits 14-10/14-12, Eq. 14-21).
    pub fn determine_capacity(&mut self) -> f64 {
        self.inner.determine_capacity()
    }

    /// Step 4: density in the ramp influence area (pc/mi/ln)
    /// - Eqs. 14-22/14-23/14-28.
    pub fn determine_density(&mut self) -> f64 {
        self.inner.determine_density()
    }

    /// Level of service letter - Exhibit 14-3.
    pub fn determine_los(&mut self) -> String {
        let los: char = self.inner.determine_los().into();
        los.to_string()
    }

    /// Step 5: speeds (S_R, S_O or None, S) in mi/h
    /// - Exhibits 14-13/14-14/14-15.
    pub fn estimate_speed(&mut self) -> (f64, Option<f64>, f64) {
        self.inner.estimate_speed()
    }

    /// Run the full Chapter 14 analysis (Steps 1-5); returns the LOS letter.
    pub fn run_analysis(&mut self) -> String {
        let los: char = self.inner.run_analysis().into();
        los.to_string()
    }

    // ── Getters ─────────────────────────────────────────────────────────

    #[getter]
    pub fn flow_freeway(&self) -> f64 {
        self.inner.get_flow_freeway()
    }

    #[getter]
    pub fn flow_ramp(&self) -> f64 {
        self.inner.get_flow_ramp()
    }

    #[getter]
    pub fn p_f(&self) -> Option<f64> {
        self.inner.p_f
    }

    #[getter]
    pub fn v12(&self) -> f64 {
        self.inner.get_v12()
    }

    #[getter]
    pub fn vr12(&self) -> f64 {
        self.inner.get_vr12()
    }

    #[getter]
    pub fn capacity_freeway(&self) -> f64 {
        self.inner.get_capacity_freeway()
    }

    #[getter]
    pub fn capacity_ramp(&self) -> f64 {
        self.inner.get_capacity_ramp()
    }

    #[getter]
    pub fn vc_ratio(&self) -> f64 {
        self.inner.get_vc_ratio()
    }

    #[getter]
    pub fn demand_exceeds_capacity(&self) -> Option<bool> {
        self.inner.demand_exceeds_capacity
    }

    #[getter]
    pub fn exceeds_max_desirable(&self) -> Option<bool> {
        self.inner.exceeds_max_desirable
    }

    #[getter]
    pub fn density(&self) -> f64 {
        self.inner.get_density()
    }

    #[getter]
    pub fn speed_ramp(&self) -> f64 {
        self.inner.get_speed_ramp()
    }

    #[getter]
    pub fn speed_outer(&self) -> Option<f64> {
        self.inner.get_speed_outer()
    }

    #[getter]
    pub fn speed_avg(&self) -> f64 {
        self.inner.get_speed_avg()
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
            "RampSegment(type={:?}, side={:?}, lanes={}, ffs={:.0}, ramp_ffs={:.0})",
            self.inner.ramp_type,
            self.inner.ramp_side,
            self.inner.freeway_lanes,
            self.inner.freeway_ffs,
            self.inner.ramp_ffs
        )
    }
}

pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<RampSegment>()?;
    Ok(())
}
