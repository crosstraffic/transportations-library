//! Python bindings for HCM Chapter 24: Off-Street Pedestrian and Bicycle
//! Facilities.

use crate::hcm::offstreet_pedbike::offstreet_pedbike::{
    ExclusivePedestrianFacility as LibExclusivePedestrianFacility,
    OffStreetBicycleFacility as LibOffStreetBicycleFacility, PathUserGroup,
    PedestrianFacilityType, PedestrianFlowType,
    SharedUsePathPedestrian as LibSharedUsePathPedestrian, NUM_PATH_MODES,
};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

fn parse_facility_type(s: &str) -> PyResult<PedestrianFacilityType> {
    match s.to_ascii_lowercase().replace(['-', '_', ' '], "").as_str() {
        "walkway" => Ok(PedestrianFacilityType::Walkway),
        "crossflow" => Ok(PedestrianFacilityType::CrossFlow),
        "stairway" => Ok(PedestrianFacilityType::Stairway),
        other => Err(PyValueError::new_err(format!(
            "unknown facility_type '{other}'; expected 'walkway', 'cross_flow', or 'stairway'"
        ))),
    }
}

fn parse_flow_type(s: &str) -> PyResult<PedestrianFlowType> {
    match s.to_ascii_lowercase().as_str() {
        "random" => Ok(PedestrianFlowType::Random),
        "platooned" | "platoon" => Ok(PedestrianFlowType::Platooned),
        other => Err(PyValueError::new_err(format!(
            "unknown flow_type '{other}'; expected 'random' or 'platooned'"
        ))),
    }
}

/// Exclusive off-street pedestrian facility analysis (HCM Chapter 24).
#[pyclass]
#[derive(Debug, Clone)]
pub struct ExclusivePedestrianFacility {
    pub inner: LibExclusivePedestrianFacility,
}

#[pymethods]
impl ExclusivePedestrianFacility {
    /// Create a new exclusive pedestrian facility analysis.
    ///
    /// Args:
    ///     total_walkway_width: Total walkway width W_T (ft)
    ///     fixed_object_width: Sum of fixed-object effective widths and shy distances W_O (ft, default 0)
    ///     pedestrian_demand: Hourly pedestrian demand v_h (p/h), if known
    ///     peak_15min_volume: Field-measured peak 15-min pedestrian volume (p), if known
    ///     phf: Peak hour factor (default 0.85)
    ///     pedestrian_speed: Average pedestrian speed S_p (ft/min, default 300)
    ///     facility_type: "walkway", "cross_flow", or "stairway" (default "walkway")
    ///     flow_type: "random" or "platooned" (default "random")
    #[new]
    #[pyo3(signature = (total_walkway_width, fixed_object_width=0.0, pedestrian_demand=None, peak_15min_volume=None, phf=None, pedestrian_speed=None, facility_type="walkway", flow_type="random"))]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        total_walkway_width: f64,
        fixed_object_width: f64,
        pedestrian_demand: Option<f64>,
        peak_15min_volume: Option<f64>,
        phf: Option<f64>,
        pedestrian_speed: Option<f64>,
        facility_type: &str,
        flow_type: &str,
    ) -> PyResult<Self> {
        Ok(ExclusivePedestrianFacility {
            inner: LibExclusivePedestrianFacility::new(
                total_walkway_width,
                fixed_object_width,
                pedestrian_demand,
                peak_15min_volume,
                phf,
                pedestrian_speed,
                parse_facility_type(facility_type)?,
                parse_flow_type(flow_type)?,
            ),
        })
    }

    /// Step 1: Determine effective walkway width W_E (ft). HCM Equation 24-1.
    pub fn determine_effective_walkway_width(&mut self) -> f64 {
        self.inner.determine_effective_walkway_width()
    }

    /// Step 2: Calculate the pedestrian unit flow rate v_p (p/ft/min).
    /// HCM Equations 24-2 and 24-3.
    pub fn calculate_pedestrian_flow_rate(&mut self) -> f64 {
        self.inner.calculate_pedestrian_flow_rate()
    }

    /// Step 3: Calculate average pedestrian space A_p (ft²/p). HCM Equation 24-4.
    pub fn calculate_average_pedestrian_space(&mut self) -> f64 {
        self.inner.calculate_average_pedestrian_space()
    }

    /// Step 4: Determine LOS (Exhibits 24-1, 24-2, or 24-3). Returns "A"-"F".
    pub fn determine_los(&mut self) -> char {
        self.inner.determine_los().into()
    }

    /// Step 5: Calculate the volume-to-capacity ratio.
    pub fn calculate_volume_to_capacity_ratio(&mut self) -> f64 {
        self.inner.calculate_volume_to_capacity_ratio()
    }

    /// Run the complete methodology (Steps 1-5) and return the LOS letter.
    pub fn analyze(&mut self) -> char {
        self.inner.analyze().into()
    }

    /// Effective walkway width W_E (ft), if computed.
    #[getter]
    pub fn effective_width(&self) -> Option<f64> {
        self.inner.effective_width
    }

    /// Pedestrian volume during the peak 15 min (p), if computed.
    #[getter]
    pub fn flow_rate_15min(&self) -> Option<f64> {
        self.inner.flow_rate_15min
    }

    /// Pedestrian flow per unit width v_p (p/ft/min), if computed.
    #[getter]
    pub fn unit_flow_rate(&self) -> Option<f64> {
        self.inner.unit_flow_rate
    }

    /// Average pedestrian space A_p (ft²/p), if computed.
    #[getter]
    pub fn pedestrian_space(&self) -> Option<f64> {
        self.inner.pedestrian_space
    }

    /// Volume-to-capacity ratio, if computed.
    #[getter]
    pub fn vc_ratio(&self) -> Option<f64> {
        self.inner.vc_ratio
    }

    /// Level of service letter ("A"-"F"), if computed.
    #[getter]
    pub fn los(&self) -> Option<char> {
        self.inner.los.map(|los| los.into())
    }

    pub fn __repr__(&self) -> String {
        format!(
            "ExclusivePedestrianFacility(width={:.1} ft, space={:?} ft2/p, los={:?})",
            self.inner.total_walkway_width, self.inner.pedestrian_space, self.los()
        )
    }
}

/// Pedestrian LOS analysis on a shared-use path (HCM Chapter 24).
#[pyclass]
#[derive(Debug, Clone)]
pub struct SharedUsePathPedestrian {
    pub inner: LibSharedUsePathPedestrian,
}

#[pymethods]
impl SharedUsePathPedestrian {
    /// Create a new shared-use path pedestrian analysis.
    ///
    /// Args:
    ///     bicycle_demand_same_direction: Q_sb (bicycles/h)
    ///     bicycle_demand_opposing: Q_ob (bicycles/h)
    ///     phf: Peak hour factor (default 0.85)
    ///     pedestrian_speed: Mean pedestrian speed S_p (default 3.4 mi/h; any
    ///         unit consistent with bicycle_speed)
    ///     bicycle_speed: Mean bicycle speed S_b (default 12.8 mi/h)
    ///     bicycle_flow_rate_same_direction: Peak 15-min same-direction flow
    ///         rate (bicycles/h), substitutes for Q_sb/PHF when provided
    ///     bicycle_flow_rate_opposing: Peak 15-min opposing flow rate
    ///         (bicycles/h), substitutes for Q_ob/PHF when provided
    ///     is_one_way: One-way path flag (no meeting events; default False)
    #[new]
    #[pyo3(signature = (bicycle_demand_same_direction=None, bicycle_demand_opposing=None, phf=None, pedestrian_speed=None, bicycle_speed=None, bicycle_flow_rate_same_direction=None, bicycle_flow_rate_opposing=None, is_one_way=false))]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        bicycle_demand_same_direction: Option<f64>,
        bicycle_demand_opposing: Option<f64>,
        phf: Option<f64>,
        pedestrian_speed: Option<f64>,
        bicycle_speed: Option<f64>,
        bicycle_flow_rate_same_direction: Option<f64>,
        bicycle_flow_rate_opposing: Option<f64>,
        is_one_way: bool,
    ) -> Self {
        let mut inner = LibSharedUsePathPedestrian::new(
            bicycle_demand_same_direction,
            bicycle_demand_opposing,
            phf,
            pedestrian_speed,
            bicycle_speed,
        );
        inner.bicycle_flow_rate_same_direction = bicycle_flow_rate_same_direction;
        inner.bicycle_flow_rate_opposing = bicycle_flow_rate_opposing;
        inner.is_one_way = is_one_way;
        SharedUsePathPedestrian { inner }
    }

    /// Step 2: Calculate bicycle passing and meeting events.
    /// HCM Equations 24-5 to 24-7. Returns (F_p, F_m, F) in events/h.
    pub fn calculate_bicycle_passing_and_meeting_events(&mut self) -> (f64, f64, f64) {
        self.inner.calculate_bicycle_passing_and_meeting_events()
    }

    /// Step 3: Determine LOS (Exhibit 24-4). Returns "A"-"F".
    pub fn determine_los(&mut self) -> char {
        self.inner.determine_los().into()
    }

    /// Run the complete methodology (Steps 1-3) and return the LOS letter.
    pub fn analyze(&mut self) -> char {
        self.inner.analyze().into()
    }

    /// Number of passing events F_p (events/h), if computed.
    #[getter]
    pub fn passing_events(&self) -> Option<f64> {
        self.inner.passing_events
    }

    /// Number of meeting events F_m (events/h), if computed.
    #[getter]
    pub fn meeting_events(&self) -> Option<f64> {
        self.inner.meeting_events
    }

    /// Total weighted events F (events/h), if computed.
    #[getter]
    pub fn total_events(&self) -> Option<f64> {
        self.inner.total_events
    }

    /// Level of service letter ("A"-"F"), if computed.
    #[getter]
    pub fn los(&self) -> Option<char> {
        self.inner.los.map(|los| los.into())
    }

    pub fn __repr__(&self) -> String {
        format!(
            "SharedUsePathPedestrian(total_events={:?} events/h, los={:?})",
            self.inner.total_events,
            self.los()
        )
    }
}

/// Bicycle LOS (BLOS) analysis on a shared-use or exclusive off-street bicycle
/// facility (HCM Chapter 24).
#[pyclass]
#[derive(Debug, Clone)]
pub struct OffStreetBicycleFacility {
    pub inner: LibOffStreetBicycleFacility,
}

#[pymethods]
impl OffStreetBicycleFacility {
    /// Create a new off-street bicycle facility analysis.
    ///
    /// Args:
    ///     path_width: Path width (ft; methodology applies up to 20 ft)
    ///     segment_length: Path segment length L (mi)
    ///     has_centerline: Whether the path has a centerline stripe (default False)
    ///     two_way_demand: Total two-directional path demand (users/h)
    ///     directional_split: Subject-direction share of demand (default 0.50)
    ///     phf: Peak hour factor (default 0.85)
    ///     subject_demand: Directional demand Q_T (users/h), overrides two_way_demand
    ///     opposing_demand: Opposing directional demand (users/h), overrides two_way_demand
    ///     is_one_way: One-way path flag (default False)
    ///     mode_splits: Optional mode splits for [bicycle, pedestrian, runner,
    ///         inline skater, child bicyclist] (defaults from Exhibit 24-6:
    ///         [0.55, 0.20, 0.10, 0.10, 0.05])
    ///     mode_speeds: Optional average mode speeds (mi/h; defaults
    ///         [12.8, 3.4, 6.5, 10.1, 7.9])
    ///     mode_speed_sds: Optional mode speed standard deviations (mi/h;
    ///         defaults [3.4, 0.6, 1.2, 2.7, 1.9])
    #[new]
    #[pyo3(signature = (path_width, segment_length, has_centerline=false, two_way_demand=None, directional_split=None, phf=None, subject_demand=None, opposing_demand=None, is_one_way=false, mode_splits=None, mode_speeds=None, mode_speed_sds=None))]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        path_width: f64,
        segment_length: f64,
        has_centerline: bool,
        two_way_demand: Option<f64>,
        directional_split: Option<f64>,
        phf: Option<f64>,
        subject_demand: Option<f64>,
        opposing_demand: Option<f64>,
        is_one_way: bool,
        mode_splits: Option<Vec<f64>>,
        mode_speeds: Option<Vec<f64>>,
        mode_speed_sds: Option<Vec<f64>>,
    ) -> PyResult<Self> {
        let mut inner = LibOffStreetBicycleFacility::new(
            path_width,
            segment_length,
            has_centerline,
            two_way_demand,
            directional_split,
            phf,
        );
        inner.subject_demand = subject_demand;
        inner.opposing_demand = opposing_demand;
        inner.is_one_way = is_one_way;

        fn apply(
            groups: &mut [PathUserGroup; NUM_PATH_MODES],
            values: Option<Vec<f64>>,
            name: &str,
            set: fn(&mut PathUserGroup, f64),
        ) -> PyResult<()> {
            if let Some(values) = values {
                if values.len() != NUM_PATH_MODES {
                    return Err(PyValueError::new_err(format!(
                        "{name} must contain exactly {NUM_PATH_MODES} values \
                         (bicycle, pedestrian, runner, inline skater, child bicyclist)"
                    )));
                }
                for (group, value) in groups.iter_mut().zip(values) {
                    set(group, value);
                }
            }
            Ok(())
        }
        apply(&mut inner.user_groups, mode_splits, "mode_splits", |g, v| {
            g.mode_split = v
        })?;
        apply(&mut inner.user_groups, mode_speeds, "mode_speeds", |g, v| {
            g.average_speed = v
        })?;
        apply(
            &mut inner.user_groups,
            mode_speed_sds,
            "mode_speed_sds",
            |g, v| g.speed_standard_deviation = v,
        )?;
        Ok(OffStreetBicycleFacility { inner })
    }

    /// Step 1: Calculate hourly directional flow rates by mode (HCM Equation
    /// 24-8). Returns (subject_flow_rates, opposing_flow_rates).
    pub fn calculate_directional_flow_rates(&mut self) -> (Vec<f64>, Vec<f64>) {
        let (qs, qo) = self.inner.calculate_directional_flow_rates();
        (qs.to_vec(), qo.to_vec())
    }

    /// Step 2: Calculate active passings per minute A_T (HCM Equations 24-9 to 24-12).
    pub fn calculate_active_passings_per_minute(&mut self) -> f64 {
        self.inner.calculate_active_passings_per_minute()
    }

    /// Step 3: Calculate meetings per minute M_T (HCM Equations 24-13 to 24-16).
    pub fn calculate_meetings_per_minute(&mut self) -> f64 {
        self.inner.calculate_meetings_per_minute()
    }

    /// Step 4: Determine the number of effective lanes (Exhibit 24-14).
    pub fn determine_number_of_effective_lanes(&mut self) -> u8 {
        self.inner.determine_number_of_effective_lanes()
    }

    /// Step 5: Calculate the total probability of delayed passing P_Tds
    /// (HCM Equations 24-17 to 24-33).
    pub fn calculate_probability_of_delayed_passing(&mut self) -> f64 {
        self.inner.calculate_probability_of_delayed_passing()
    }

    /// Step 6: Calculate delayed passings per minute DP_m (HCM Equation 24-34).
    pub fn calculate_delayed_passings_per_minute(&mut self) -> f64 {
        self.inner.calculate_delayed_passings_per_minute()
    }

    /// Step 7: Determine the BLOS score (HCM Equation 24-35).
    pub fn determine_blos(&mut self) -> f64 {
        self.inner.determine_blos()
    }

    /// Step 8: Adjust LOS for low-volume paths. Returns "A"-"F".
    pub fn adjust_los_for_low_volume_paths(&mut self) -> char {
        self.inner.adjust_los_for_low_volume_paths().into()
    }

    /// Run the complete methodology (Steps 1-8) and return the LOS letter.
    pub fn analyze(&mut self) -> char {
        self.inner.analyze().into()
    }

    /// Subject-direction hourly flow rates by mode (modal users/h), if computed.
    #[getter]
    pub fn subject_flow_rates(&self) -> Option<Vec<f64>> {
        self.inner.subject_flow_rates.map(|q| q.to_vec())
    }

    /// Opposing-direction hourly flow rates by mode (modal users/h), if computed.
    #[getter]
    pub fn opposing_flow_rates(&self) -> Option<Vec<f64>> {
        self.inner.opposing_flow_rates.map(|q| q.to_vec())
    }

    /// Active passings per minute by mode, if computed.
    #[getter]
    pub fn active_passings_by_mode(&self) -> Option<Vec<f64>> {
        self.inner.active_passings_by_mode.map(|a| a.to_vec())
    }

    /// Total active passings per minute A_T, if computed.
    #[getter]
    pub fn active_passings_per_minute(&self) -> Option<f64> {
        self.inner.active_passings_per_minute
    }

    /// Total meetings per minute M_T, if computed.
    #[getter]
    pub fn meetings_per_minute(&self) -> Option<f64> {
        self.inner.meetings_per_minute
    }

    /// Number of effective lanes, if computed.
    #[getter]
    pub fn effective_lanes(&self) -> Option<u8> {
        self.inner.effective_lanes
    }

    /// Total probability of delayed passing P_Tds, if computed.
    #[getter]
    pub fn total_probability_delayed_passing(&self) -> Option<f64> {
        self.inner.total_probability_delayed_passing
    }

    /// Delayed passings per minute DP_m, if computed.
    #[getter]
    pub fn delayed_passings_per_minute(&self) -> Option<f64> {
        self.inner.delayed_passings_per_minute
    }

    /// Weighted events per minute E = M_T + 10 × A_T, if computed.
    #[getter]
    pub fn weighted_events_per_minute(&self) -> Option<f64> {
        self.inner.weighted_events_per_minute
    }

    /// BLOS score (HCM Equation 24-35), if computed.
    #[getter]
    pub fn blos_score(&self) -> Option<f64> {
        self.inner.blos_score
    }

    /// Bicycle level of service letter ("A"-"F"), if computed.
    #[getter]
    pub fn los(&self) -> Option<char> {
        self.inner.los.map(|los| los.into())
    }

    pub fn __repr__(&self) -> String {
        format!(
            "OffStreetBicycleFacility(width={:.1} ft, length={:.2} mi, blos={:?}, los={:?})",
            self.inner.path_width,
            self.inner.segment_length,
            self.inner.blos_score,
            self.los()
        )
    }
}

pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<ExclusivePedestrianFacility>()?;
    m.add_class::<SharedUsePathPedestrian>()?;
    m.add_class::<OffStreetBicycleFacility>()?;
    Ok(())
}
