//! Python bindings for HCM Chapter 20 (Two-Way STOP-Controlled
//! Intersections).

use crate::hcm::twsc::twsc::{MajorLeftLaneConfig, Mv, PlatoonBlockage, Twsc as LibTwsc};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

fn parse_major_left(config: &str, storage_veh: Option<u32>) -> PyResult<MajorLeftLaneConfig> {
    match config.to_ascii_lowercase().as_str() {
        "exclusive" => Ok(MajorLeftLaneConfig::Exclusive),
        "shared" => Ok(MajorLeftLaneConfig::Shared),
        "short_pocket" | "sharedshortpocket" => Ok(MajorLeftLaneConfig::SharedShortPocket {
            storage_veh: storage_veh.ok_or_else(|| {
                PyValueError::new_err("short_pocket major-left config requires storage_veh")
            })?,
        }),
        other => Err(PyValueError::new_err(format!(
            "unknown major-left lane config: {other} (expected exclusive|shared|short_pocket)"
        ))),
    }
}

fn major_left_to_py(cfg: MajorLeftLaneConfig) -> (String, Option<u32>) {
    match cfg {
        MajorLeftLaneConfig::Exclusive => ("exclusive".to_string(), None),
        MajorLeftLaneConfig::Shared => ("shared".to_string(), None),
        MajorLeftLaneConfig::SharedShortPocket { storage_veh } => {
            ("short_pocket".to_string(), Some(storage_veh))
        }
    }
}

/// HCM Chapter 20 TWSC intersection analysis.
///
/// Construct from a JSON configuration matching the
/// `tests/ExampleCases/hcm/Twsc/*.json` fixture format (demand by HCM
/// Exhibit 20-1 movement number, geometry, PHF, heavy-vehicle percentage),
/// call `analyze()`, then read per-movement and per-lane results.
#[pyclass]
#[derive(Debug, Clone)]
pub struct Twsc {
    pub inner: LibTwsc,
}

fn parse_movement(label: &str) -> PyResult<Mv> {
    Mv::from_label(label)
        .ok_or_else(|| PyValueError::new_err(format!("unknown TWSC movement label: {label}")))
}

#[pymethods]
impl Twsc {
    /// Create a TWSC analysis from a JSON configuration string.
    ///
    /// Args:
    ///     config_json: JSON with `demand`, `geometry`, optional `phf`,
    ///         `analysis_period_h` (default 0.25), `heavy_vehicle_pct`,
    ///         `conflicting_flow_overrides`, `platoon_blockage` (the Step 5b
    ///         proportion-of-time-blocked inputs p_b,x for coordinated upstream
    ///         signals, HCM Equations 20-19 through 20-21), and
    ///         `upstream_signals` (coordinated upstream-signal descriptors from
    ///         which p_b,x is computed by the HCM Chapter 30, Section 3
    ///         procedure when no explicit `platoon_blockage` is given).
    #[new]
    pub fn new(config_json: &str) -> PyResult<Self> {
        let inner = LibTwsc::from_json(config_json)
            .map_err(|e| PyValueError::new_err(format!("invalid TWSC JSON: {e}")))?;
        Ok(Twsc { inner })
    }

    /// Run the complete HCM Chapter 20 procedure (Steps 1-13).
    pub fn analyze(&mut self) {
        self.inner.analyze();
    }

    /// Peak hour factor (None if demand values are flow rates).
    #[getter]
    pub fn get_phf(&self) -> Option<f64> {
        self.inner.phf
    }

    #[setter]
    pub fn set_phf(&mut self, phf: Option<f64>) {
        self.inner.phf = phf;
    }

    /// Heavy-vehicle percentage applied to all movements (%).
    #[getter]
    pub fn get_heavy_vehicle_pct(&self) -> f64 {
        self.inner.heavy_vehicle_pct
    }

    #[setter]
    pub fn set_heavy_vehicle_pct(&mut self, pct: f64) {
        self.inner.heavy_vehicle_pct = pct;
    }

    /// Analysis period T, h.
    #[getter]
    pub fn get_analysis_period_h(&self) -> f64 {
        self.inner.analysis_period_h
    }

    #[setter]
    pub fn set_analysis_period_h(&mut self, t: f64) {
        self.inner.analysis_period_h = t;
    }

    /// Set the upstream-signal proportion-of-time-blocked inputs p_b,x for
    /// Step 5b (HCM Equations 20-19 through 20-21, Exhibit 20-19). Movements
    /// 1U and 4U reuse `pb1`/`pb4`; the two-stage movements 7, 8, 10, and 11
    /// draw their Stage I/II values from `pb1`/`pb4` per Exhibit 20-19. Pass
    /// all zeros (or never call this) for a no-platooning analysis.
    #[allow(clippy::too_many_arguments)]
    pub fn set_platoon_blockage(
        &mut self,
        pb1: f64,
        pb4: f64,
        pb7: f64,
        pb8: f64,
        pb9: f64,
        pb10: f64,
        pb11: f64,
        pb12: f64,
    ) {
        self.inner.platoon_blockage = Some(PlatoonBlockage {
            pb1,
            pb4,
            pb7,
            pb8,
            pb9,
            pb10,
            pb11,
            pb12,
        });
    }

    /// The upstream-signal proportion-of-time-blocked inputs as
    /// `(pb1, pb4, pb7, pb8, pb9, pb10, pb11, pb12)`, or `None` if unset.
    /// After `analyze()` with `upstream_signals` supplied, this returns the
    /// values computed by the HCM Chapter 30, Section 3 procedure.
    #[getter]
    pub fn get_platoon_blockage(&self) -> Option<(f64, f64, f64, f64, f64, f64, f64, f64)> {
        self.inner
            .platoon_blockage
            .as_ref()
            .map(|p| (p.pb1, p.pb4, p.pb7, p.pb8, p.pb9, p.pb10, p.pb11, p.pb12))
    }

    /// Set the coordinated upstream-signal descriptors from a JSON object, so
    /// the Step 5b proportions p_b,x are derived by the HCM Chapter 30,
    /// Section 3 procedure (Equation 30-13) during `analyze()` instead of
    /// being supplied directly. The JSON matches the `UpstreamSignals` schema:
    /// `cycle_s`, optional `eastbound`/`westbound` signal objects (each with
    /// `distance_ft`, `progression_speed_mph`, `discharges` [a list of
    /// movement discharge profiles], and optional `uniform_volume_veh_h`), and
    /// optional `time_step_s` (default 1.0). An explicit `platoon_blockage`
    /// always takes precedence over the computed values.
    pub fn set_upstream_signals(&mut self, config_json: &str) -> PyResult<()> {
        let signals = serde_json::from_str(config_json)
            .map_err(|e| PyValueError::new_err(format!("invalid upstream_signals JSON: {e}")))?;
        self.inner.upstream_signals = Some(signals);
        Ok(())
    }

    /// Clear any coordinated upstream-signal descriptors.
    pub fn clear_upstream_signals(&mut self) {
        self.inner.upstream_signals = None;
    }

    /// Whether coordinated upstream-signal descriptors are set (the computed
    /// Chapter 30, Section 3 p_b path is active when no explicit
    /// `platoon_blockage` is supplied).
    #[getter]
    pub fn get_has_upstream_signals(&self) -> bool {
        self.inner.upstream_signals.is_some()
    }

    /// Set the major-street left-turn lane configuration on an approach
    /// ("EB" for movements 1+1U, "WB" for 4+4U), HCM Step 7d. `config` is one
    /// of "exclusive" (default), "shared" (shares the through lane, n_L = 0),
    /// or "short_pocket" (requires `storage_veh` = n_L). A shared or short
    /// pocket triggers the p*_0,j substitution of Equations 20-29 through
    /// 20-34 and the Step 11b Rank 1 delay.
    #[pyo3(signature = (approach, config, storage_veh=None))]
    pub fn set_major_left_config(
        &mut self,
        approach: &str,
        config: &str,
        storage_veh: Option<u32>,
    ) -> PyResult<()> {
        let cfg = parse_major_left(config, storage_veh)?;
        match approach.to_ascii_uppercase().as_str() {
            "EB" => self.inner.geometry.major_left_eb = cfg,
            "WB" => self.inner.geometry.major_left_wb = cfg,
            other => {
                return Err(PyValueError::new_err(format!(
                    "unknown major approach: {other} (expected EB or WB)"
                )))
            }
        }
        Ok(())
    }

    /// The major-street left-turn lane configuration of an approach ("EB" or
    /// "WB") as `(config, storage_veh)`, where `config` is "exclusive",
    /// "shared", or "short_pocket".
    pub fn get_major_left_config(&self, approach: &str) -> PyResult<(String, Option<u32>)> {
        let cfg = match approach.to_ascii_uppercase().as_str() {
            "EB" => self.inner.geometry.major_left_eb,
            "WB" => self.inner.geometry.major_left_wb,
            other => {
                return Err(PyValueError::new_err(format!(
                    "unknown major approach: {other} (expected EB or WB)"
                )))
            }
        };
        Ok(major_left_to_py(cfg))
    }

    /// HCM Step 11b Rank 1 delay `(d_2+3, d_5+6)` to major-street through
    /// vehicles sharing a lane with a blocked left turn, s/veh (Equations
    /// 20-62/20-63), or `None` when both major lefts have exclusive lanes.
    #[getter]
    pub fn get_rank1_major_delay(&self) -> Option<(f64, f64)> {
        self.inner.rank1_major_delay.map(|a| (a[0], a[1]))
    }

    /// Demand flow rate of a movement ("1", "1U", ..., "12"), veh/h.
    pub fn get_flow_rate(&self, movement: &str) -> PyResult<f64> {
        Ok(self.inner.get_flow_rate(parse_movement(movement)?))
    }

    /// Conflicting flow rate v_c,x of a movement, veh/h (Step 3).
    pub fn get_conflicting_flow(&self, movement: &str) -> PyResult<Option<f64>> {
        let mv = parse_movement(movement)?;
        Ok(self.inner.movements[mv.idx()].conflicting_flow)
    }

    /// Potential capacity c_p,x of a movement, veh/h (Equation 20-18).
    pub fn get_potential_capacity(&self, movement: &str) -> PyResult<Option<f64>> {
        let mv = parse_movement(movement)?;
        Ok(self.inner.movements[mv.idx()].potential_capacity)
    }

    /// Movement capacity c_m,x, veh/h (Steps 6-9).
    pub fn get_movement_capacity(&self, movement: &str) -> PyResult<Option<f64>> {
        let mv = parse_movement(movement)?;
        Ok(self.inner.movements[mv.idx()].movement_capacity)
    }

    /// Control delay of an exclusive-lane movement, s/veh (Equation 20-61).
    pub fn get_movement_delay(&self, movement: &str) -> PyResult<Option<f64>> {
        let mv = parse_movement(movement)?;
        Ok(self.inner.movements[mv.idx()].control_delay)
    }

    /// LOS letter of an exclusive-lane movement (Exhibit 20-2).
    pub fn get_movement_los(&self, movement: &str) -> PyResult<Option<String>> {
        let mv = parse_movement(movement)?;
        Ok(self.inner.movements[mv.idx()].los.map(|c| c.to_string()))
    }

    /// 95th percentile queue of a movement, veh (Equation 20-66).
    pub fn get_movement_queue_95(&self, movement: &str) -> PyResult<Option<f64>> {
        let mv = parse_movement(movement)?;
        Ok(self.inner.movements[mv.idx()].queue_95)
    }

    /// Number of minor-street approach lanes ("NB" or "SB").
    pub fn get_lane_count(&self, approach: &str) -> PyResult<usize> {
        Ok(self.lanes(approach)?.len())
    }

    /// (capacity veh/h, delay s/veh, LOS, Q95 veh) of a minor approach lane.
    pub fn get_lane_result(
        &self,
        approach: &str,
        lane: usize,
    ) -> PyResult<(f64, f64, String, f64)> {
        let lanes = self.lanes(approach)?;
        let l = lanes
            .get(lane)
            .ok_or_else(|| PyValueError::new_err(format!("no lane {lane} on {approach}")))?;
        Ok((l.capacity, l.control_delay, l.los.to_string(), l.queue_95))
    }

    /// Approach control delays [EB, WB, NB, SB], s/veh (Equation 20-64).
    #[getter]
    pub fn get_approach_delays(&self) -> Option<[f64; 4]> {
        self.inner.approach_delays
    }

    /// Intersection control delay, s/veh (Equation 20-65). Note LOS is not
    /// defined for a TWSC intersection as a whole.
    #[getter]
    pub fn get_intersection_delay(&self) -> Option<f64> {
        self.inner.intersection_delay
    }

    /// Full analysis (inputs + results) as JSON.
    pub fn to_json(&self) -> PyResult<String> {
        self.inner
            .to_json()
            .map_err(|e| PyValueError::new_err(format!("serialize TWSC: {e}")))
    }

    pub fn __repr__(&self) -> String {
        format!(
            "Twsc(three_leg={}, major_lanes={}, intersection_delay={:?})",
            self.inner.geometry.is_three_leg,
            self.inner.geometry.major_lanes_per_direction,
            self.inner.intersection_delay
        )
    }
}

impl Twsc {
    fn lanes(&self, approach: &str) -> PyResult<&Vec<crate::hcm::twsc::twsc::TwscLaneResult>> {
        match approach.to_ascii_uppercase().as_str() {
            "NB" => Ok(&self.inner.lanes_nb),
            "SB" => Ok(&self.inner.lanes_sb),
            other => Err(PyValueError::new_err(format!(
                "TWSC minor approach must be NB or SB, got {other}"
            ))),
        }
    }
}

pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Twsc>()?;
    Ok(())
}
