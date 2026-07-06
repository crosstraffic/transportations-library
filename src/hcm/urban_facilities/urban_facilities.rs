//! # Urban Street Facilities (HCM Chapter 16), motorized vehicle methodology
//!
//! Implements the HCM 7th Edition Chapter 16, Section 3 computational steps
//! (EPUB `112_Ch16_03.xhtml`; LOS criteria and Exhibit 16-3 from
//! `111_Ch16_02.xhtml`). The facility is an ordered set of Chapter 18
//! [`UrbanSegment`]s for one direction of travel; "Each travel direction
//! along the facility is separately evaluated."
//!
//! The methodology "describes a process for aggregating key performance
//! measures associated with the segments that make up the facility"
//! (Chapter 16, Overview of the Methodology). Segment-level inputs
//! (travel speed, base free-flow speed, spatial stop rate, through
//! volume-to-capacity ratio at the downstream boundary intersection) are
//! "HCM method output" per Exhibit 16-7 — computed here with the Chapter
//! 18 engine, or supplied directly as published values via
//! [`SegmentSummary`].

use serde::{Deserialize, Serialize};

use crate::hcm::urban_segments::exhibits::exhibit_18_1_los;
use crate::hcm::urban_segments::urban_segments::{traveler_perception_score, UrbanSegment};
use crate::hcm::common::LevelOfService;

// Equation 16-1 is identical to Chapter 18's Equation 18-2 (both compute
// `c_th = 1,800 (N_th − 1 + p*_0,j)` for the uncontrolled through movement
// at a TWSC boundary intersection); re-export the shared implementation.
pub use crate::hcm::urban_segments::urban_segments::through_capacity_uncontrolled;

// ═══════════════════════════════════════════════════════════════════════════════
// Aggregation equations (Equations 16-2 through 16-4)
// ═══════════════════════════════════════════════════════════════════════════════

/// HCM Equation 16-2: base free-flow speed for the facility,
/// `S_fo,F = Σ L_i / Σ (L_i / S_fo,i)` (mi/h) — the length-weighted
/// harmonic mean (equivalently, total length divided by the total travel
/// time at the base free-flow speed).
///
/// * `segments` — `(L_i, S_fo,i)` pairs: segment length (ft) and segment
///   base free-flow speed (mi/h)
pub fn facility_base_ffs(segments: &[(f64, f64)]) -> f64 {
    length_weighted_harmonic_mean(segments)
}

/// HCM Equation 16-3: travel speed for the facility,
/// `S_T,F = Σ L_i / Σ (L_i / S_T,seg,i)` (mi/h).
///
/// * `segments` — `(L_i, S_T,seg,i)` pairs: segment length (ft) and
///   segment through-vehicle travel speed (mi/h)
pub fn facility_travel_speed(segments: &[(f64, f64)]) -> f64 {
    length_weighted_harmonic_mean(segments)
}

/// HCM Equation 16-4: spatial stop rate for the facility,
/// `H_F = Σ (H_seg,i L_i) / Σ L_i` (stops/mi) — the length-weighted
/// arithmetic mean of the segment spatial stop rates.
///
/// * `segments` — `(L_i, H_seg,i)` pairs: segment length (ft) and segment
///   spatial stop rate (stops/mi)
pub fn facility_spatial_stop_rate(segments: &[(f64, f64)]) -> f64 {
    let total_length: f64 = segments.iter().map(|(l, _)| l).sum();
    if total_length <= 0.0 {
        return 0.0;
    }
    segments.iter().map(|(l, h)| l * h).sum::<f64>() / total_length
}

/// Ordering rank of a LOS letter (A = best = 0 … F = worst = 5), used for
/// the poorest-performing-segment report.
fn los_rank(los: LevelOfService) -> u8 {
    match los {
        LevelOfService::A => 0,
        LevelOfService::B => 1,
        LevelOfService::C => 2,
        LevelOfService::D => 3,
        LevelOfService::E => 4,
        LevelOfService::F => 5,
    }
}

fn length_weighted_harmonic_mean(segments: &[(f64, f64)]) -> f64 {
    let total_length: f64 = segments.iter().map(|(l, _)| l).sum();
    let total_time: f64 = segments
        .iter()
        .map(|(l, s)| if *s > 0.0 { l / s } else { 0.0 })
        .sum();
    if total_length <= 0.0 || total_time <= 0.0 {
        return 0.0;
    }
    total_length / total_time
}

/// HCM Exhibit 16-3: LOS Criteria: Motorized Vehicle Mode.
///
/// The Exhibit 16-3 travel-speed thresholds by base free-flow speed are
/// value-for-value identical to Chapter 18's Exhibit 18-1 (both
/// transcribed from the EPUB: `111_Ch16_02.xhtml` and `127_Ch18_02.xhtml`),
/// including the interpolation rule for base free-flow speeds between the
/// column headings; the shared implementation is used.
///
/// * `travel_speed_mph` — facility travel speed S_T,F (Equation 16-3)
/// * `base_ffs_mph` — facility base free-flow speed S_fo,F (Equation 16-2)
/// * `critical_vc_gt_1` — true if the critical volume-to-capacity ratio
///   exceeds 1.0. Per the Exhibit 16-3 footnote, the critical ratio "is
///   based on consideration of the through movement volume-to-capacity
///   ratio at each boundary intersection in the subject direction of
///   travel" and "is the largest ratio of those considered"; LOS F is
///   assigned "if a volume-to-capacity ratio greater than 1.0 exists for
///   the through movement at one or more boundary intersections."
pub fn exhibit_16_3_los(
    travel_speed_mph: f64,
    base_ffs_mph: f64,
    critical_vc_gt_1: bool,
) -> LevelOfService {
    exhibit_18_1_los(travel_speed_mph, base_ffs_mph, critical_vc_gt_1)
}

// ═══════════════════════════════════════════════════════════════════════════════
// Segment summary (published or engine-computed per-segment measures)
// ═══════════════════════════════════════════════════════════════════════════════

/// Per-segment performance measures consumed by the Chapter 16
/// aggregation (the Exhibit 16-7 input data elements). Produced by
/// [`UrbanFacility::analyze`] from the Chapter 18 engine, or supplied
/// directly (e.g., published example-problem values) to
/// [`aggregate_segment_summaries`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SegmentSummary {
    /// Segment length L_i, ft.
    pub length_ft: f64,
    /// Segment base free-flow speed S_fo,i, mi/h (Chapter 18 output).
    pub base_ffs_mph: f64,
    /// Segment through-vehicle travel speed S_T,seg,i, mi/h (Chapter 18
    /// output).
    pub travel_speed_mph: f64,
    /// Segment spatial stop rate H_seg,i, stops/mi (Chapter 18 output).
    #[serde(default)]
    pub spatial_stop_rate_stops_mi: Option<f64>,
    /// Volume-to-capacity ratio of the through movement at the segment's
    /// downstream boundary intersection (Chapters 19-23 output, or
    /// Equation 16-1 for a TWSC uncontrolled through movement).
    #[serde(default)]
    pub vc_ratio: Option<f64>,
    /// Segment LOS (Exhibit 18-1), for the poorest-performing-segment
    /// report.
    #[serde(default)]
    pub los: Option<LevelOfService>,
}

/// Facility-level results of the Chapter 16 motorized vehicle methodology.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FacilityResults {
    /// Facility length Σ L_i, ft.
    pub length_ft: f64,
    /// Facility base free-flow speed S_fo,F, mi/h (Equation 16-2).
    pub base_ffs_mph: f64,
    /// Facility travel speed S_T,F, mi/h (Equation 16-3).
    pub travel_speed_mph: f64,
    /// Facility travel time at the travel speed, s.
    pub travel_time_s: f64,
    /// Facility travel time at the base free-flow speed, s.
    pub base_free_flow_travel_time_s: f64,
    /// Facility spatial stop rate H_F, stops/mi (Equation 16-4). None when
    /// no segment supplied a stop rate.
    pub spatial_stop_rate_stops_mi: Option<f64>,
    /// Critical volume-to-capacity ratio: the largest through-movement
    /// v/c ratio among the boundary intersections (Exhibit 16-3
    /// footnote). None when no segment supplied a ratio.
    pub critical_vc_ratio: Option<f64>,
    /// Facility motorized vehicle LOS (Exhibit 16-3).
    pub los: LevelOfService,
    /// LOS of the poorest-performing segment — Chapter 16 Step 4 directs
    /// the analyst to "consider reporting the LOS for the
    /// poorest-performing segment as a means of providing context for the
    /// interpretation of facility LOS."
    pub poorest_segment_los: Option<LevelOfService>,
    /// Facility automobile traveler perception score I_a (Chapter 16 Step
    /// 3: the Chapter 18 Step 10 equations with H_F substituted for H_seg
    /// and the facility-wide P_LTL). Requires `prop_left_turn_lanes`.
    pub perception_score: Option<f64>,
}

/// Run the Chapter 16 aggregation (Steps 1-4) over per-segment summaries.
///
/// * `segments` — ordered per-segment measures (at least one)
/// * `prop_left_turn_lanes` — facility-wide proportion of intersections
///   with a left-turn lane, P_LTL (decimal), for the optional perception
///   score
pub fn aggregate_segment_summaries(
    segments: &[SegmentSummary],
    prop_left_turn_lanes: Option<f64>,
) -> Result<FacilityResults, String> {
    if segments.is_empty() {
        return Err("facility must contain at least one segment".into());
    }
    for (i, s) in segments.iter().enumerate() {
        if s.length_ft <= 0.0 {
            return Err(format!("segment {i}: length must be positive"));
        }
        if s.base_ffs_mph <= 0.0 || s.travel_speed_mph <= 0.0 {
            return Err(format!("segment {i}: speeds must be positive"));
        }
    }

    let length_ft: f64 = segments.iter().map(|s| s.length_ft).sum();

    // Step 1: Equation 16-2.
    let base_pairs: Vec<(f64, f64)> =
        segments.iter().map(|s| (s.length_ft, s.base_ffs_mph)).collect();
    let base_ffs = facility_base_ffs(&base_pairs);

    // Step 2: Equation 16-3.
    let speed_pairs: Vec<(f64, f64)> =
        segments.iter().map(|s| (s.length_ft, s.travel_speed_mph)).collect();
    let travel_speed = facility_travel_speed(&speed_pairs);

    // Step 3: Equation 16-4 (only when every segment reports a stop rate;
    // a partial aggregation would misstate the facility value).
    let spatial_stop_rate = if segments.iter().all(|s| s.spatial_stop_rate_stops_mi.is_some()) {
        let pairs: Vec<(f64, f64)> = segments
            .iter()
            .map(|s| (s.length_ft, s.spatial_stop_rate_stops_mi.unwrap()))
            .collect();
        Some(facility_spatial_stop_rate(&pairs))
    } else {
        None
    };
    let perception_score = match (spatial_stop_rate, prop_left_turn_lanes) {
        (Some(h_f), Some(p_ltl)) => Some(traveler_perception_score(h_f, p_ltl)),
        _ => None,
    };

    // Step 4: Exhibit 16-3 with the critical v/c footnote.
    let critical_vc = segments
        .iter()
        .filter_map(|s| s.vc_ratio)
        .fold(None::<f64>, |acc, v| Some(acc.map_or(v, |a| a.max(v))));
    let los = exhibit_16_3_los(travel_speed, base_ffs, critical_vc.is_some_and(|v| v > 1.0));
    let poorest_segment_los = segments
        .iter()
        .filter_map(|s| s.los)
        .fold(None::<LevelOfService>, |acc, l| {
            Some(acc.map_or(l, |a| if los_rank(l) > los_rank(a) { l } else { a }))
        });

    Ok(FacilityResults {
        length_ft,
        base_ffs_mph: base_ffs,
        travel_speed_mph: travel_speed,
        travel_time_s: 3_600.0 * length_ft / (5_280.0 * travel_speed),
        base_free_flow_travel_time_s: 3_600.0 * length_ft / (5_280.0 * base_ffs),
        spatial_stop_rate_stops_mi: spatial_stop_rate,
        critical_vc_ratio: critical_vc,
        los,
        poorest_segment_los,
        perception_score,
    })
}

// ═══════════════════════════════════════════════════════════════════════════════
// Spillback check hook
// ═══════════════════════════════════════════════════════════════════════════════

/// Inputs for the per-segment queue-storage (spillback) check.
///
/// Chapter 16 requires the methodology not be applied to segments
/// experiencing sustained spillback; the full Chapter 29, Section 3
/// evaluation procedure (iterative capacity constraint over the Chapter
/// 18/19 engines) is deferred. This hook flags segments at risk so the
/// analyst can invoke that procedure or an alternative tool.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpillbackCheckInput {
    /// Available queue storage on the segment, ft (typically the distance
    /// from the downstream stop line to the nearest upstream conflict
    /// point, per lane).
    pub available_storage_ft: f64,
    /// Predicted back-of-queue size, veh/ln (Chapter 19/31 output, e.g.,
    /// the 95th percentile or average back of queue).
    pub back_of_queue_veh_ln: f64,
    /// Average queued-vehicle spacing L_h, ft/veh (HCM Equation 31-155:
    /// `L_h = 25.0 + 2 L_pc P_HV/100` with L_pc = 8 ft; ≈25 ft for 0%
    /// heavy vehicles).
    #[serde(default = "default_spacing")]
    pub avg_vehicle_spacing_ft: f64,
}

fn default_spacing() -> f64 {
    25.0
}

impl SpillbackCheckInput {
    /// Queue storage ratio `R_q = L_h Q / L_a` (HCM Equation 19-36 form).
    pub fn storage_ratio(&self) -> f64 {
        if self.available_storage_ft <= 0.0 {
            return f64::INFINITY;
        }
        self.avg_vehicle_spacing_ft * self.back_of_queue_veh_ln / self.available_storage_ft
    }

    /// True when the back of queue is predicted to exceed the available
    /// storage (storage ratio > 1.0) — spillback risk; the Chapter 29,
    /// Section 3 sustained spillback procedure should be applied.
    pub fn spillback_expected(&self) -> bool {
        self.storage_ratio() > 1.0
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Urban street facility (one direction of travel)
// ═══════════════════════════════════════════════════════════════════════════════

/// One direction of travel on an urban street facility (HCM Chapter 16,
/// motorized vehicle methodology): an ordered sequence of Chapter 18
/// urban street segments plus facility-level inputs.
///
/// Populate the segments (directly or via [`UrbanFacility::from_json`]),
/// call [`UrbanFacility::analyze`], then read `results`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UrbanFacility {
    /// Ordered Chapter 18 segments in the subject direction of travel
    /// (upstream to downstream).
    pub segments: Vec<UrbanSegment>,
    /// Facility-wide proportion of intersections with a left-turn lane,
    /// P_LTL (decimal), for the facility perception score (Chapter 16
    /// Step 3).
    #[serde(default)]
    pub prop_left_turn_lanes: Option<f64>,
    /// Optional per-segment spillback check inputs (same order/length as
    /// `segments`).
    #[serde(default)]
    pub spillback_inputs: Option<Vec<SpillbackCheckInput>>,

    // ───────────────────── Computed results ─────────────────────
    /// Facility aggregation results (populated by [`Self::analyze`]).
    #[serde(default)]
    pub results: Option<FacilityResults>,
    /// Per-segment spillback flags (populated by [`Self::analyze`] when
    /// `spillback_inputs` is provided).
    #[serde(default)]
    pub spillback_flags: Option<Vec<bool>>,
}

impl UrbanFacility {
    /// Create a facility from ordered Chapter 18 segments.
    pub fn new(segments: Vec<UrbanSegment>) -> Self {
        UrbanFacility {
            segments,
            prop_left_turn_lanes: None,
            spillback_inputs: None,
            results: None,
            spillback_flags: None,
        }
    }

    /// Deserialize from the `tests/ExampleCases/hcm/UrbanFacilities`
    /// fixture JSON format (field names match the struct fields; each
    /// entry of `segments` uses the Chapter 18 `UrbanSegment` schema).
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Serialize the full analysis (inputs and results) to JSON.
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Facility length Σ L_i, ft.
    pub fn length_ft(&self) -> f64 {
        self.segments.iter().map(|s| s.segment_length_ft).sum()
    }

    /// Number of segments m.
    pub fn num_segments(&self) -> usize {
        self.segments.len()
    }

    /// Per-segment summaries from the segments' computed fields (call
    /// after the segments have been analyzed).
    pub fn segment_summaries(&self) -> Result<Vec<SegmentSummary>, String> {
        self.segments
            .iter()
            .enumerate()
            .map(|(i, s)| {
                Ok(SegmentSummary {
                    length_ft: s.segment_length_ft,
                    base_ffs_mph: s
                        .base_ffs_mph
                        .ok_or(format!("segment {i}: base free-flow speed not computed"))?,
                    travel_speed_mph: s
                        .travel_speed_mph
                        .ok_or(format!("segment {i}: travel speed not computed"))?,
                    spatial_stop_rate_stops_mi: s.spatial_stop_rate_stops_mi,
                    vc_ratio: s.vc_ratio,
                    los: s.los,
                })
            })
            .collect()
    }

    /// Run the full Chapter 16 motorized vehicle pipeline: evaluate every
    /// segment with the Chapter 18 engine, aggregate per Equations 16-2
    /// through 16-4, determine LOS per Exhibit 16-3, and evaluate the
    /// spillback check hook.
    pub fn analyze(&mut self) -> Result<&FacilityResults, String> {
        for segment in &mut self.segments {
            segment.analyze();
        }
        self.aggregate()
    }

    /// Aggregate the already-computed segment measures without re-running
    /// the Chapter 18 engine (Steps 1-4 only). Use when the per-segment
    /// measures were supplied directly (e.g., published values).
    pub fn aggregate(&mut self) -> Result<&FacilityResults, String> {
        let summaries = self.segment_summaries()?;
        let results = aggregate_segment_summaries(&summaries, self.prop_left_turn_lanes)?;
        if let Some(inputs) = &self.spillback_inputs {
            if inputs.len() != self.segments.len() {
                return Err(format!(
                    "spillback_inputs has {} entries for {} segments",
                    inputs.len(),
                    self.segments.len()
                ));
            }
            self.spillback_flags =
                Some(inputs.iter().map(|s| s.spillback_expected()).collect());
        }
        self.results = Some(results);
        Ok(self.results.as_ref().unwrap())
    }

    // ─────────────────────── Accessors ───────────────────────

    pub fn get_base_ffs_mph(&self) -> Option<f64> {
        self.results.as_ref().map(|r| r.base_ffs_mph)
    }
    pub fn get_travel_speed_mph(&self) -> Option<f64> {
        self.results.as_ref().map(|r| r.travel_speed_mph)
    }
    pub fn get_spatial_stop_rate(&self) -> Option<f64> {
        self.results.as_ref().and_then(|r| r.spatial_stop_rate_stops_mi)
    }
    pub fn get_critical_vc_ratio(&self) -> Option<f64> {
        self.results.as_ref().and_then(|r| r.critical_vc_ratio)
    }
    pub fn get_los(&self) -> Option<LevelOfService> {
        self.results.as_ref().map(|r| r.los)
    }
    pub fn get_poorest_segment_los(&self) -> Option<LevelOfService> {
        self.results.as_ref().and_then(|r| r.poorest_segment_los)
    }
    pub fn get_perception_score(&self) -> Option<f64> {
        self.results.as_ref().and_then(|r| r.perception_score)
    }
}
