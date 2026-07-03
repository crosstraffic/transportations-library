//! HCM Chapter 25, Section 6: Planning-Level Methodology for Freeway
//! Facilities.
//!
//! A simplified, section-based screening method compatible with the Chapter 10
//! core methodology. It takes directional AADT, a K-factor, terrain, and a
//! coarse section geometry, and returns per-section demand-to-capacity ratios,
//! speeds, densities, queue lengths, and a facility LOS estimate over four
//! 15-min analysis periods.
//!
//! Equations implemented (all from `197_Ch25_06.xhtml`):
//! - Equation 25-40: per-period demand flow rates from AADT and the K-factor
//!   (four analysis periods with multipliers `[1, 1/PHF, 1, 2 − 1/PHF]`);
//! - Equation 25-42: heavy-vehicle adjustment factor;
//! - Equations 25-43/25-44: section demand accumulation with vertical-queue
//!   carryover;
//! - Equation 25-45: basic section capacity;
//! - Equation 25-46: weaving-section CAF;
//! - Equation 25-47 (+ Exhibit 25-16): undersaturated delay rate;
//! - Equation 25-48: oversaturated delay rate;
//! - Equations 25-49 through 25-52: travel rate, time, speed, and density;
//! - Exhibit 25-17: urban/rural LOS thresholds.
//!
//! Reproduces HCM Chapter 25 Example Problem 6 (`202_Ch25_11.xhtml`, Exhibits
//! 25-88 through 25-96).

use serde::{Deserialize, Serialize};

use crate::hcm::common::{CityType, LevelOfService};

use super::freeway_facilities::Terrain;

/// Number of 15-min analysis periods the planning method uses (Equation
/// 25-40 fixes the analysis to a single peak hour split into four periods).
pub const NUM_PLANNING_PERIODS: usize = 4;

/// Ramp-section capacity adjustment factor (Chapter 25, *Adjustments for Ramp
/// Sections*): "an average CAF of 0.9 can be used for ramp sections".
pub const DEFAULT_RAMP_CAF: f64 = 0.9;

/// Planning-method section type (Chapter 25, Exhibit 25-15).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlanningSectionType {
    /// Basic freeway section (no capacity adjustment).
    Basic,
    /// Weaving section (auxiliary lane; CAF from Equation 25-46).
    Weave,
    /// Ramp section (on- or off-ramp; CAF of 0.9 by default).
    Ramp,
}

impl Default for PlanningSectionType {
    fn default() -> Self {
        PlanningSectionType::Basic
    }
}

/// One planning-method freeway section (Chapter 25, Exhibit 25-15).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct PlanningSection {
    /// Section type (basic, weave, ramp).
    pub sec_type: PlanningSectionType,
    /// Section length L, mi.
    pub length_mi: f64,
    /// Number of lanes N in the section.
    pub lanes: u32,
    /// AADT (veh/day) entering at this section's upstream boundary — the
    /// facility entering volume on the first section and the on-ramp AADT on
    /// ramp sections that add demand. Zero if the boundary only sheds demand.
    pub inflow_aadt: f64,
    /// AADT (veh/day) leaving at this section's upstream boundary (off-ramp).
    pub outflow_aadt: f64,
    /// Weaving volume ratio V_r (weave sections; Equation 25-46). Ignored for
    /// other section types.
    pub weave_vr: f64,
    /// Explicit capacity adjustment factor override (decimal). When `None`,
    /// ramp sections use [`DEFAULT_RAMP_CAF`], weave sections use Equation
    /// 25-46, and basic sections use 1.0.
    pub caf_override: Option<f64>,
}

impl Default for PlanningSection {
    fn default() -> Self {
        Self {
            sec_type: PlanningSectionType::Basic,
            length_mi: 1.0,
            lanes: 3,
            inflow_aadt: 0.0,
            outflow_aadt: 0.0,
            weave_vr: 0.0,
            caf_override: None,
        }
    }
}

/// Per-section, per-period planning results.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlanningSectionResult {
    /// Section demand d, pc/h (includes vertical-queue carryover).
    pub demand_pc: f64,
    /// Demand-to-capacity ratio d/c.
    pub dc_ratio: f64,
    /// Delay rate ΔR, s/mi (undersaturated + oversaturated components).
    pub delay_rate: f64,
    /// Travel rate TR, s/mi.
    pub travel_rate: f64,
    /// Travel time T, s.
    pub travel_time_s: f64,
    /// Space mean speed S, mi/h.
    pub speed: f64,
    /// Density D, pc/mi/ln.
    pub density: f64,
    /// Vertical-queue length, mi.
    pub queue_length_mi: f64,
}

/// Facility-level planning summary for one analysis period (Exhibit 25-96).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanningFacilityResult {
    /// True if any section has d/c > 1.0 (Exhibit 25-96 "Oversaturated").
    pub oversaturated: bool,
    /// Facility travel time, min.
    pub travel_time_min: f64,
    /// Facility space mean speed, mi/h.
    pub space_mean_speed: f64,
    /// Facility average density (length-weighted), pc/mi/ln.
    pub avg_density: f64,
    /// Total vertical-queue length across sections, mi.
    pub total_queue_mi: f64,
    /// Facility LOS (Exhibit 25-17; F if any section d/c > 1.0).
    pub los: LevelOfService,
}

/// Undersaturated delay-rate polynomial parameters (Exhibit 25-16), keyed by
/// free-flow speed (mi/h): `(A, B, C, D, E)`.
fn undersaturated_params(ffs: f64) -> (f64, f64, f64, f64, f64) {
    // Snap to the nearest 5-mi/h column defined by Exhibit 25-16 (55–75).
    let key = (ffs / 5.0).round() * 5.0;
    match key as i32 {
        75 => (68.99, -77.97, 34.04, -5.82, 0.44),
        70 => (71.24, -85.48, 35.58, -5.44, 0.52),
        65 => (92.45, -127.33, 56.34, -8.00, 0.62),
        60 => (121.35, -184.84, 83.21, -9.33, 0.72),
        _ => (156.43, -248.99, 99.20, -0.12, 0.82), // 55 (and below)
    }
}

/// Equation 25-45: basic freeway section capacity, pc/h/ln:
/// `c = 2,200 + 10 × [min(70, FFS) − 50]`.
pub fn basic_section_capacity_pc(ffs: f64) -> f64 {
    2200.0 + 10.0 * (ffs.min(70.0) - 50.0)
}

/// Equation 25-46: weaving-section CAF:
/// `CAF_weave = min(0.884 − 0.0752·V_r + 0.0000243·L_s, 1)` with L_s in ft.
pub fn weave_caf(volume_ratio: f64, length_mi: f64) -> f64 {
    let ls_ft = length_mi * 5280.0;
    (0.884 - 0.0752 * volume_ratio + 0.0000243 * ls_ft).min(1.0)
}

/// Equation 25-47 + Exhibit 25-16: undersaturated delay rate ΔRU, s/mi.
/// The cubic is evaluated at the demand-to-capacity ratio (0 below the
/// threshold E). To reproduce the published Example Problem 6 travel rates
/// (Exhibit 25-93), the polynomial is evaluated at the **actual** d/c even
/// when it exceeds 1.0 (see the module-level VERIFY-HCM note).
pub fn undersaturated_delay_rate(dc: f64, ffs: f64) -> f64 {
    let (a, b, c, d, e) = undersaturated_params(ffs);
    if dc < e {
        0.0
    } else {
        a * dc.powi(3) + b * dc.powi(2) + c * dc + d
    }
}

/// Equation 25-48: additional oversaturation delay rate ΔRO, s/mi:
/// `ΔRO = (450 / L) × max(0, d/c − 1)` with L in mi.
pub fn oversaturated_delay_rate(dc: f64, length_mi: f64) -> f64 {
    if length_mi <= 0.0 {
        return 0.0;
    }
    450.0 / length_mi * (dc - 1.0).max(0.0)
}

/// A freeway facility analyzed with the Chapter 25 planning-level method.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct PlanningFacility {
    // ── Global inputs (Chapter 25, Section 6) ────────────────────────────
    /// Ordered sections, upstream to downstream (Exhibit 25-15).
    pub sections: Vec<PlanningSection>,
    /// Facility free-flow speed, mi/h.
    pub ffs: f64,
    /// K-factor (directional AADT → peak-hour flow).
    pub k_factor: f64,
    /// Traffic growth factor f_tg.
    pub growth_factor: f64,
    /// Peak hour factor.
    pub phf: f64,
    /// Single-unit-truck + bus percentage, decimal.
    pub pct_sut: f64,
    /// Tractor-trailer percentage, decimal.
    pub pct_tt: f64,
    /// Terrain (Exhibit 12-25 PCEs).
    pub terrain: Terrain,
    /// Urban or rural facility (Exhibit 25-17 thresholds).
    pub city_type: CityType,

    // ── Computed ([section][period]) ─────────────────────────────────────
    /// Per-section, per-period results.
    pub section_results: Vec<Vec<PlanningSectionResult>>,
    /// Facility summary by period.
    pub facility_results: Vec<PlanningFacilityResult>,
}

impl Default for PlanningFacility {
    fn default() -> Self {
        Self {
            sections: Vec::new(),
            ffs: 60.0,
            k_factor: 0.09,
            growth_factor: 1.0,
            phf: 0.9,
            pct_sut: 0.0,
            pct_tt: 0.0,
            terrain: Terrain::Level,
            city_type: CityType::Urban,
            section_results: Vec::new(),
            facility_results: Vec::new(),
        }
    }
}

impl PlanningFacility {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn num_sections(&self) -> usize {
        self.sections.len()
    }

    /// Total facility length, mi.
    pub fn total_length_mi(&self) -> f64 {
        self.sections.iter().map(|s| s.length_mi).sum()
    }

    /// Equation 25-42: heavy-vehicle adjustment factor `f_HV = 1 / (1 + P_T
    /// (E_T − 1))` with the combined truck percentage `P_T = P_SUT + P_TT`.
    pub fn f_hv(&self) -> f64 {
        let pt = self.pct_sut + self.pct_tt;
        1.0 / (1.0 + pt * (self.terrain.pce() - 1.0))
    }

    /// Equation 25-40 period demand multipliers `[1, 1/PHF, 1, 2 − 1/PHF]`.
    fn period_multipliers(&self) -> [f64; NUM_PLANNING_PERIODS] {
        let inv = 1.0 / self.phf;
        [1.0, inv, 1.0, 2.0 - inv]
    }

    /// Per-lane section capacity, pc/h/ln (Equation 25-45 with the section
    /// CAF from Equation 25-46 / the 0.9 ramp default / an override).
    fn section_capacity_pc_per_lane(&self, sec: &PlanningSection) -> f64 {
        let base = basic_section_capacity_pc(self.ffs);
        let caf = match sec.caf_override {
            Some(c) => c,
            None => match sec.sec_type {
                PlanningSectionType::Basic => 1.0,
                PlanningSectionType::Ramp => DEFAULT_RAMP_CAF,
                PlanningSectionType::Weave => weave_caf(sec.weave_vr, sec.length_mi),
            },
        };
        base * caf
    }

    /// Run the planning-level analysis (Steps 1–5).
    pub fn run_analysis(&mut self) -> Result<(), String> {
        if self.sections.is_empty() {
            return Err("planning facility has no sections".into());
        }
        let n = self.num_sections();
        let f_hv = self.f_hv();
        let base_factor = self.k_factor * self.growth_factor * f_hv;
        let mult = self.period_multipliers();

        // Net boundary flow (pc/h) added at each section's upstream boundary,
        // before the period multiplier (Equation 25-40 for each entry/ramp).
        let boundary: Vec<f64> = self
            .sections
            .iter()
            .map(|s| (s.inflow_aadt - s.outflow_aadt) * base_factor)
            .collect();

        // Section total capacities (pc/h).
        let cap_total: Vec<f64> = self
            .sections
            .iter()
            .map(|s| self.section_capacity_pc_per_lane(s) * f64::from(s.lanes))
            .collect();

        self.section_results = vec![vec![PlanningSectionResult::default(); NUM_PLANNING_PERIODS]; n];
        let mut carryover = vec![0.0f64; n];
        let ffs = self.ffs;
        let tr_ffs = 3600.0 / ffs; // free-flow travel rate, s/mi

        for p in 0..NUM_PLANNING_PERIODS {
            let mut next_carryover = vec![0.0f64; n];
            // Equation 25-43 accumulates the upstream section's demand
            // d_{i-1,p} (which already carries any released vertical queue)
            // plus this section's own carryover from the previous period.
            let mut upstream = 0.0f64;
            for i in 0..n {
                let sec = &self.sections[i];
                let demand = (upstream + boundary[i] * mult[p]).max(0.0) + carryover[i];
                upstream = demand;
                let cap = cap_total[i];
                let dc = if cap > 0.0 { demand / cap } else { 0.0 };

                // Equation 25-44: vertical-queue carryover to next period.
                next_carryover[i] = (demand - cap).max(0.0);

                // Equation 25-47: undersaturated delay rate.
                //
                // VERIFY-HCM: the published Example Problem 6 delay rates
                // (Exhibit 25-92) and travel rates (Exhibit 25-93) use ΔRU
                // only — the oversaturated ΔRO term (Equation 25-48) is NOT
                // added to either the reported delay rate or the travel rate,
                // and ΔRU is evaluated at the actual d/c even when d/c > 1.0
                // (contradicting the E ≤ d/c ≤ 1 domain printed with Equation
                // 25-47 and the ΔRU + ΔRO travel rate of Equation 25-49).
                // Oversaturation is instead expressed through the vertical
                // queue carryover (Equations 25-43/25-44). We reproduce the
                // worked example: delay/travel/speed use ΔRU; [`oversaturated_delay_rate`]
                // (Equation 25-48) is retained as a public helper.
                let dru = undersaturated_delay_rate(dc, ffs);
                let delay_rate = dru;

                // Equations 25-49..25-52: travel rate/time/speed/density.
                let travel_rate = dru + tr_ffs;
                let travel_time_s = travel_rate * sec.length_mi;
                let speed = if travel_rate > 0.0 { 3600.0 / travel_rate } else { ffs };
                let lanes = f64::from(sec.lanes.max(1));
                let density = if speed > 0.0 {
                    demand / (lanes * speed)
                } else {
                    0.0
                };
                // Queue length: vertical-queue carryover spread over the
                // section lanes at the prevailing density (Chapter 25 Step 1).
                let queue_length_mi = if density > 0.0 {
                    next_carryover[i] / (lanes * density)
                } else {
                    0.0
                };

                self.section_results[i][p] = PlanningSectionResult {
                    demand_pc: demand,
                    dc_ratio: dc,
                    delay_rate,
                    travel_rate,
                    travel_time_s,
                    speed,
                    density,
                    queue_length_mi,
                };
            }
            carryover = next_carryover;
        }

        self.aggregate_facility();
        Ok(())
    }

    /// Step 4/5: facility-level aggregation and LOS (Exhibit 25-96).
    fn aggregate_facility(&mut self) {
        let n = self.num_sections();
        let total_len = self.total_length_mi();
        self.facility_results = Vec::with_capacity(NUM_PLANNING_PERIODS);
        for p in 0..NUM_PLANNING_PERIODS {
            let mut total_time_s = 0.0;
            let mut dens_len = 0.0;
            let mut queue = 0.0;
            let mut oversat = false;
            for i in 0..n {
                let r = &self.section_results[i][p];
                total_time_s += r.travel_time_s;
                dens_len += r.density * self.sections[i].length_mi;
                queue += r.queue_length_mi;
                if r.dc_ratio > 1.0 {
                    oversat = true;
                }
            }
            let travel_time_min = total_time_s / 60.0;
            let sms = if total_time_s > 0.0 {
                total_len / (total_time_s / 3600.0)
            } else {
                self.ffs
            };
            // Facility density is a length-weighted average of section
            // densities (Exhibit 25-96 note).
            let avg_density = if total_len > 0.0 { dens_len / total_len } else { 0.0 };
            let los = super::exhibits::los_freeway_facility(avg_density, oversat, self.city_type);
            self.facility_results.push(PlanningFacilityResult {
                oversaturated: oversat,
                travel_time_min,
                space_mean_speed: sms,
                avg_density,
                total_queue_mi: queue,
                los,
            });
        }
    }

    // ── Result accessors ─────────────────────────────────────────────────

    pub fn dc_ratio(&self, section: usize, period: usize) -> f64 {
        self.section_results[section][period].dc_ratio
    }

    pub fn section_speed(&self, section: usize, period: usize) -> f64 {
        self.section_results[section][period].speed
    }

    pub fn section_density(&self, section: usize, period: usize) -> f64 {
        self.section_results[section][period].density
    }

    pub fn facility_los(&self, period: usize) -> LevelOfService {
        self.facility_results[period].los
    }

    pub fn facility_speed(&self, period: usize) -> f64 {
        self.facility_results[period].space_mean_speed
    }

    pub fn facility_density(&self, period: usize) -> f64 {
        self.facility_results[period].avg_density
    }
}
