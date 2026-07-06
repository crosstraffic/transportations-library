//! HCM Chapter 37 (ATDM: Supplemental) strategy-impact models.
//!
//! Chapter 37 is explicitly "Supplemental" content: unlike a numbered
//! HCM analysis chapter, it does not define its own facility methodology
//! or produce results of its own. Its entire purpose (Chapter 11, Section
//! 4, "Extensions to the Methodology, Active Traffic and Demand
//! Management": "the strategy or plan is ultimately translated into a
//! series of HCM inputs and adjustment factors to demand, capacity, and
//! speed") is to supply the CAF/SAF/DAF-shaped adjustments that feed the
//! freeway reliability engine ([`crate::hcm::freeway_reliability`]) and the urban
//! street reliability engine ([`crate::hcm::urban_reliability`]), both of which
//! already expose scenario-level adjustment hooks
//! ([`crate::hcm::freeway_reliability::scenario_generation::WorkZoneEvent`] /
//! `SpecialEvent`, and [`crate::hcm::urban_reliability::AtdmStrategy`]). This
//! module is therefore placed under `common/` alongside
//! [`crate::hcm::common::delay`] and [`crate::hcm::common::reliability`]
//! — chapter-agnostic shared primitives — rather than as a `chapter37/`
//! module that would contain no engine of its own. Per the existing
//! `common/*` dependency direction (chapters depend on `common`, never
//! the reverse), this module exposes pure equation/constant
//! transcriptions only; the convenience constructors that build a
//! `WorkZoneEvent` or `AtdmStrategy` from these values live in
//! `freeway_reliability::scenario_generation` and `urban_reliability::urban_reliability`
//! respectively, so they can depend on this module without creating a
//! cycle.
//!
//! Sources (HCM 7th Edition EPUB, Chapter 37, "ATDM: Supplemental"):
//! - Section 3, "Effects of Shoulder and Median Lane Strategies"
//!   (`286_Ch37_03.xhtml`) — Equation 37-1 (average per-lane capacity with
//!   an open shoulder/median lane) and the shoulder-lane capacity/FFS
//!   assumptions for the six use variants (auxiliary lane, buses-only,
//!   HOV-only, all-traffic, on right or median shoulders).
//! - Section 4, "Effects of Ramp-Metering Strategies" (`287_Ch37_04.xhtml`)
//!   — the merge-segment CAF for metered ramps and Equation 37-2 (the
//!   ALINEA-derived locally dynamic metering rate).
//! - Section 5, "Effects of Adaptive Signals" (`288_Ch37_05.xhtml`) —
//!   Exhibit 37-9's illustrative simulation-study ranges (no HCM-endorsed
//!   formula; see [`ADAPTIVE_SIGNAL_DELAY_REDUCTION_PCT_RANGE`]).
//!
//! ## Deferred (no computable HCM values published)
//! Section 6 ("Effects of Dynamic Lane Grouping") and Section 7 ("Effects
//! of Reversible Center Lanes") each list Chapter 18/19 *inputs an
//! analyst may need to reconsider* (lane assignments, turn bay lengths,
//! left/right-turn operational mode, median type, and so on) but publish
//! no exhibit, equation, or default adjustment factor — the HCM directs
//! the analyst to re-run the base methodology with hand-edited inputs.
//! There is nothing in these two sections to transcribe into a typed
//! strategy without fabricating a number the HCM does not provide, so
//! they are not modeled here.

use serde::{Deserialize, Serialize};

/// HCM chapter implemented by this module.
pub const CHAPTER: u32 = 37;

// ═══════════════════════════════════════════════════════════════════════════════
// Section 3: Effects of Shoulder and Median Lane Strategies
// ═══════════════════════════════════════════════════════════════════════════════

/// Default capacity of an auxiliary shoulder/median lane open to all
/// traffic, as a fraction of a normal mixed-flow lane's capacity (HCM
/// Chapter 37, Section 3: "this procedure assumes that the capacity of an
/// auxiliary shoulder lane is one-half that of a normal freeway through
/// lane"). Applies to the "open shoulders as auxiliary lanes between
/// adjacent on- and off-ramps" strategy; the "open right/median shoulder
/// to all traffic" variants instead use an analyst-specified capacity
/// (see [`ShoulderLaneUse::AllTraffic`]).
pub const AUX_SHOULDER_CAPACITY_RATIO: f64 = 0.5;

/// How a shoulder or median lane is opened (HCM Chapter 37, Section 3),
/// which determines the shoulder/median lane's capacity per
/// [`shoulder_lane_capacity_veh_h_ln`].
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ShoulderLaneUse {
    /// Auxiliary lane between adjacent on/off-ramps, open right shoulder,
    /// or open median shoulder, to all traffic. Capacity defaults to
    /// [`AUX_SHOULDER_CAPACITY_RATIO`] of a normal mixed-flow lane (the
    /// only variant with a published default ratio); pass
    /// `capacity_override_veh_h_ln` for the "open right/median shoulder
    /// to all traffic" variants, whose capacity the HCM leaves fully to
    /// the analyst ("the capacity of the shoulder lane is as specified by
    /// the user").
    AllTraffic { capacity_override_veh_h_ln: Option<f64> },
    /// Open to buses only. Capacity is the lesser of the observed/forecast
    /// bus volume and a capacity value ("the number of buses per hour
    /// using the shoulder lane or the user-specified capacity, whichever
    /// is less"). VERIFY-HCM: Section 3 does not state a numeric default
    /// for that capacity value when the user does not override it; this
    /// implementation defaults it to a normal mixed-flow lane's capacity
    /// (i.e., the buses-per-hour term is normally the binding constraint).
    BusesOnly { buses_per_h: f64, capacity_override_veh_h_ln: Option<f64> },
    /// Open to HOVs (buses, vanpools, carpools) only. Same rule as
    /// [`Self::BusesOnly`].
    HovOnly { hov_per_h: f64, capacity_override_veh_h_ln: Option<f64> },
}

/// Capacity of the shoulder/median lane itself, veh/h, for the given use
/// (HCM Chapter 37, Section 3).
///
/// * `use_` — how the shoulder/median lane is opened
/// * `mixed_flow_capacity_veh_h_ln` — capacity of a normal mixed-flow lane
///   in the section, veh/h/ln (CapMFlanes(s) of Equation 37-1)
pub fn shoulder_lane_capacity_veh_h_ln(
    use_: ShoulderLaneUse,
    mixed_flow_capacity_veh_h_ln: f64,
) -> f64 {
    match use_ {
        ShoulderLaneUse::AllTraffic { capacity_override_veh_h_ln } => {
            capacity_override_veh_h_ln
                .unwrap_or(AUX_SHOULDER_CAPACITY_RATIO * mixed_flow_capacity_veh_h_ln)
        }
        ShoulderLaneUse::BusesOnly { buses_per_h, capacity_override_veh_h_ln } => {
            buses_per_h.min(capacity_override_veh_h_ln.unwrap_or(mixed_flow_capacity_veh_h_ln))
        }
        ShoulderLaneUse::HovOnly { hov_per_h, capacity_override_veh_h_ln } => {
            hov_per_h.min(capacity_override_veh_h_ln.unwrap_or(mixed_flow_capacity_veh_h_ln))
        }
    }
}

/// HCM Equation 37-1: average capacity per lane for a freeway section with
/// an open shoulder/median lane,
/// `AveCap(s) = [CapShldr(s) + CapMFlanes(s) x MFlanes(s)] / [1 + MFlanes(s)]`.
///
/// * `shoulder_capacity_veh_h_ln` — capacity per shoulder lane, veh/h/ln
///   (CapShldr(s); see [`shoulder_lane_capacity_veh_h_ln`])
/// * `mixed_flow_capacity_veh_h_ln` — capacity per mixed-flow lane, veh/h/ln
///   (CapMFlanes(s))
/// * `mixed_flow_lanes` — number of mixed-flow lanes in the section
///   (MFlanes(s))
///
/// Returns the average capacity per lane, veh/h/ln, across the section's
/// `mixed_flow_lanes + 1` lanes (the number of lanes is increased by one
/// for the shoulder lane, per Section 3).
pub fn shoulder_lane_average_capacity_veh_h_ln(
    shoulder_capacity_veh_h_ln: f64,
    mixed_flow_capacity_veh_h_ln: f64,
    mixed_flow_lanes: u32,
) -> f64 {
    let mf = mixed_flow_lanes as f64;
    (shoulder_capacity_veh_h_ln + mixed_flow_capacity_veh_h_ln * mf) / (1.0 + mf)
}

/// Total-section capacity adjustment factor (CAF) equivalent to opening a
/// shoulder/median lane (HCM Chapter 37, Section 3, Equation 37-1),
/// derived for engines — such as the Chapter 11 freeway reliability
/// engine's per-segment CAF schedule — that model a capacity change as a
/// single multiplicative factor on the segment's *total* capacity while
/// keeping the segment's lane count fixed for density purposes (the same
/// simplification the Chapter 11 module already documents for incident
/// lane closures).
///
/// `CAF = AveCap(s) x (MFlanes + 1) / (CapMFlanes x MFlanes)`
///      `= 1 + CapShldr(s) / (CapMFlanes(s) x MFlanes(s))`
///
/// Returns 1.0 (no effect) if `mixed_flow_lanes` is 0 or
/// `mixed_flow_capacity_veh_h_ln` is non-positive.
pub fn shoulder_lane_caf(
    shoulder_capacity_veh_h_ln: f64,
    mixed_flow_capacity_veh_h_ln: f64,
    mixed_flow_lanes: u32,
) -> f64 {
    let mf = mixed_flow_lanes as f64;
    if mixed_flow_capacity_veh_h_ln <= 0.0 || mf <= 0.0 {
        return 1.0;
    }
    1.0 + shoulder_capacity_veh_h_ln / (mixed_flow_capacity_veh_h_ln * mf)
}

// ═══════════════════════════════════════════════════════════════════════════════
// Section 4: Effects of Ramp-Metering Strategies
// ═══════════════════════════════════════════════════════════════════════════════

/// Capacity adjustment factor recommended for freeway merge segments in
/// the Chapter 10 freeway facilities method for the times when ramp
/// metering is in operation (HCM Chapter 37, Section 4, "Capacity of
/// Ramp-Metered Merge Sections": "A capacity adjustment factor of 1.03 is
/// recommended to be applied to freeway merge segments ... for those
/// times when ramp metering is in operation").
pub const RAMP_METERED_MERGE_CAF: f64 = 1.03;

/// Default minimum ramp-metering rate, veh/h/ln (HCM Chapter 37, Section
/// 4, Equation 37-2: MinRate default value).
pub const ALINEA_DEFAULT_MIN_RATE_VEH_H_LN: f64 = 240.0;

/// Default maximum ramp-metering rate, veh/h/ln (HCM Chapter 37, Section
/// 4, Equation 37-2: MaxRate default value).
pub const ALINEA_DEFAULT_MAX_RATE_VEH_H_LN: f64 = 900.0;

/// HCM Equation 37-2: locally dynamic ramp-metering rate for one analysis
/// period, an adaptation of the ALINEA algorithm.
///
/// `R(t) = (CM - VM(t)) / NR`, subject to `MinRate < R(t) < MaxRate` and
/// `R(t) > [VR(t) + QR(t-1) - QRS] / NR` (the metering rate must not be so
/// restrictive that the on-ramp queue would exceed its storage capacity).
///
/// * `downstream_capacity_veh_h` — capacity of the downstream section,
///   veh/h (CM)
/// * `upstream_volume_veh_h` — volume on the upstream section for the
///   analysis period, veh/h (VM(t))
/// * `ramp_volume_veh_h` — volume on the ramp during the analysis period,
///   veh/h (VR(t))
/// * `ramp_queue_prev_veh` — queue on the ramp at the end of the previous
///   analysis period, veh (QR(t-1))
/// * `ramp_queue_storage_veh` — queue storage capacity of the ramp, veh
///   (QRS)
/// * `metered_lanes` — number of metered lanes on the ramp (NR)
/// * `min_rate_veh_h_ln` / `max_rate_veh_h_ln` — metering rate bounds,
///   veh/h/ln ([`ALINEA_DEFAULT_MIN_RATE_VEH_H_LN`] /
///   [`ALINEA_DEFAULT_MAX_RATE_VEH_H_LN`] by default)
///
/// Returns the metering rate R(t), veh/h/ln. `metered_lanes` of 0 returns
/// `max_rate_veh_h_ln` (metering has no effect without a metered lane).
pub fn alinea_metering_rate(
    downstream_capacity_veh_h: f64,
    upstream_volume_veh_h: f64,
    ramp_volume_veh_h: f64,
    ramp_queue_prev_veh: f64,
    ramp_queue_storage_veh: f64,
    metered_lanes: u32,
    min_rate_veh_h_ln: f64,
    max_rate_veh_h_ln: f64,
) -> f64 {
    let nr = metered_lanes as f64;
    if nr <= 0.0 {
        return max_rate_veh_h_ln;
    }
    let unclamped = (downstream_capacity_veh_h - upstream_volume_veh_h) / nr;
    let queue_floor =
        (ramp_volume_veh_h + ramp_queue_prev_veh - ramp_queue_storage_veh) / nr;
    unclamped
        .max(queue_floor)
        .clamp(min_rate_veh_h_ln, max_rate_veh_h_ln)
}

// ═══════════════════════════════════════════════════════════════════════════════
// Section 5: Effects of Adaptive Signals
// ═══════════════════════════════════════════════════════════════════════════════

/// Illustrative delay-reduction range from adaptive signal control,
/// percent (HCM Chapter 37, Section 5, Exhibit 37-9: a three-corridor
/// simulation study found "delay reductions between 3% and 24%"). The HCM
/// does not endorse a generalized adaptive-signal-control method or a
/// single default value — "it has not been possible to develop a
/// generalized method adaptive signal control method for the HCM" — so
/// this range is illustrative case-study data, not a design value.
pub const ADAPTIVE_SIGNAL_DELAY_REDUCTION_PCT_RANGE: (f64, f64) = (3.0, 24.0);

/// Illustrative travel time index (TTI) reduction range from adaptive
/// signal control, percent (Exhibit 37-9: "TTI reductions between 3% and
/// 13%").
pub const ADAPTIVE_SIGNAL_TTI_REDUCTION_PCT_RANGE: (f64, f64) = (3.0, 13.0);

/// Approximate boundary-intersection saturation flow adjustment
/// equivalent to a target delay reduction from adaptive signal control.
///
/// VERIFY-HCM: Chapter 37, Section 5 gives no formula translating a
/// delay-reduction percentage into an HCM input adjustment — the
/// recommended analysis approach is a proprietary-API simulation tool,
/// not an HCM closed-form method. This function is a documented modeling
/// simplification (not an HCM-derived equation) for use with the Chapter
/// 17 [`crate::hcm::urban_reliability::AtdmStrategy::sat_flow_adjustment`] hook:
/// it treats the delay reduction as achieved through better green-time
/// utilization at capacity, i.e., `sat_flow_adjustment = 1 / (1 -
/// target_delay_reduction_pct / 100)`, so demand held at capacity yields
/// the same fractional reduction in the Chapter 19 incremental delay
/// term's implied excess demand. Analysts should prefer directly
/// calibrated `sat_flow_adjustment`/`effective_green_adjustment_s` values
/// from their own simulation study over this default when one is
/// available.
///
/// * `target_delay_reduction_pct` — desired delay reduction, percent;
///   `None` defaults to the midpoint of
///   [`ADAPTIVE_SIGNAL_DELAY_REDUCTION_PCT_RANGE`] (13.5%). Clamped to the
///   published range regardless.
///
/// Returns a saturation flow adjustment factor >= 1.0.
pub fn adaptive_signal_sat_flow_adjustment(target_delay_reduction_pct: Option<f64>) -> f64 {
    let (lo, hi) = ADAPTIVE_SIGNAL_DELAY_REDUCTION_PCT_RANGE;
    let pct = target_delay_reduction_pct
        .unwrap_or((lo + hi) / 2.0)
        .clamp(lo, hi);
    1.0 / (1.0 - pct / 100.0)
}

// ═══════════════════════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    // ── Section 3: shoulder/median lanes ────────────────────────────────

    #[test]
    fn test_aux_shoulder_capacity_default_ratio() {
        assert_eq!(AUX_SHOULDER_CAPACITY_RATIO, 0.5);
        let cap = shoulder_lane_capacity_veh_h_ln(
            ShoulderLaneUse::AllTraffic { capacity_override_veh_h_ln: None },
            2_400.0,
        );
        assert!((cap - 1_200.0).abs() < 1e-9);
    }

    #[test]
    fn test_all_traffic_capacity_override() {
        let cap = shoulder_lane_capacity_veh_h_ln(
            ShoulderLaneUse::AllTraffic { capacity_override_veh_h_ln: Some(1_800.0) },
            2_400.0,
        );
        assert_eq!(cap, 1_800.0);
    }

    #[test]
    fn test_buses_only_capacity_whichever_is_less() {
        // Observed bus volume below the (defaulted) capacity: binding.
        let low = shoulder_lane_capacity_veh_h_ln(
            ShoulderLaneUse::BusesOnly { buses_per_h: 40.0, capacity_override_veh_h_ln: None },
            2_400.0,
        );
        assert_eq!(low, 40.0);
        // Observed bus volume above an explicit override cap: cap binds.
        let high = shoulder_lane_capacity_veh_h_ln(
            ShoulderLaneUse::BusesOnly {
                buses_per_h: 5_000.0,
                capacity_override_veh_h_ln: Some(1_000.0),
            },
            2_400.0,
        );
        assert_eq!(high, 1_000.0);
    }

    #[test]
    fn test_hov_only_capacity_whichever_is_less() {
        let cap = shoulder_lane_capacity_veh_h_ln(
            ShoulderLaneUse::HovOnly { hov_per_h: 300.0, capacity_override_veh_h_ln: None },
            2_400.0,
        );
        assert_eq!(cap, 300.0);
    }

    #[test]
    fn test_equation_37_1_average_capacity() {
        // CapShldr = 1,200 (half of 2,400), CapMFlanes = 2,400, MFlanes = 3:
        // AveCap = (1,200 + 2,400*3) / 4 = 8,400/4 = 2,100.
        let ave = shoulder_lane_average_capacity_veh_h_ln(1_200.0, 2_400.0, 3);
        assert!((ave - 2_100.0).abs() < 1e-9);
    }

    #[test]
    fn test_equation_37_1_no_shoulder_reduces_to_mixed_flow_capacity() {
        // With CapShldr = CapMFlanes (a "shoulder" lane identical to a
        // normal lane), AveCap must equal CapMFlanes exactly.
        let ave = shoulder_lane_average_capacity_veh_h_ln(2_400.0, 2_400.0, 4);
        assert!((ave - 2_400.0).abs() < 1e-9);
    }

    #[test]
    fn test_shoulder_lane_caf_matches_total_capacity_ratio() {
        let shldr = 1_200.0;
        let mf_cap = 2_400.0;
        let mf_lanes = 3u32;
        let caf = shoulder_lane_caf(shldr, mf_cap, mf_lanes);
        // CAF * (total capacity without shoulder) must equal the total
        // capacity with the shoulder lane (AveCap * (MFlanes + 1)).
        let total_without = mf_cap * mf_lanes as f64;
        let total_with = shoulder_lane_average_capacity_veh_h_ln(shldr, mf_cap, mf_lanes)
            * (mf_lanes as f64 + 1.0);
        assert!((caf * total_without - total_with).abs() < 1e-6);
        assert!(caf > 1.0, "opening a shoulder lane must increase capacity");
    }

    #[test]
    fn test_shoulder_lane_caf_degenerate_inputs() {
        assert_eq!(shoulder_lane_caf(1_200.0, 2_400.0, 0), 1.0);
        assert_eq!(shoulder_lane_caf(1_200.0, 0.0, 3), 1.0);
    }

    // ── Section 4: ramp metering ─────────────────────────────────────────

    #[test]
    fn test_ramp_metered_merge_caf_constant() {
        assert_eq!(RAMP_METERED_MERGE_CAF, 1.03);
    }

    #[test]
    fn test_alinea_unconstrained_matches_equation_37_2() {
        // CM = 2,000, VM = 1,500, NR = 1 -> R = 500, within [240, 900]
        // bounds and not queue-storage-limited.
        let r = alinea_metering_rate(2_000.0, 1_500.0, 100.0, 0.0, 50.0, 1, 240.0, 900.0);
        assert!((r - 500.0).abs() < 1e-9);
    }

    #[test]
    fn test_alinea_clamped_to_max_rate() {
        // CM - VM is very large: rate must clamp at MaxRate.
        let r = alinea_metering_rate(5_000.0, 100.0, 100.0, 0.0, 50.0, 1, 240.0, 900.0);
        assert_eq!(r, 900.0);
    }

    #[test]
    fn test_alinea_clamped_to_min_rate() {
        // CM - VM is negative (oversaturated downstream): rate must clamp
        // at MinRate.
        let r = alinea_metering_rate(1_000.0, 1_800.0, 100.0, 0.0, 50.0, 1, 240.0, 900.0);
        assert_eq!(r, 240.0);
    }

    #[test]
    fn test_alinea_queue_storage_floor_binds() {
        // Unconstrained rate would be low (near MinRate), but a large
        // ramp queue relative to storage forces a higher rate so the
        // queue does not exceed storage: the queue floor must dominate.
        let unconstrained =
            alinea_metering_rate(1_200.0, 1_000.0, 400.0, 0.0, 50.0, 1, 0.0, 900.0);
        let queue_bound =
            alinea_metering_rate(1_200.0, 1_000.0, 400.0, 300.0, 50.0, 1, 0.0, 900.0);
        assert!(
            queue_bound > unconstrained,
            "a larger carried-in ramp queue must raise the metering rate \
             floor ({queue_bound} vs {unconstrained})"
        );
    }

    #[test]
    fn test_alinea_zero_metered_lanes_returns_max_rate() {
        assert_eq!(
            alinea_metering_rate(2_000.0, 1_000.0, 100.0, 0.0, 50.0, 0, 240.0, 900.0),
            900.0
        );
    }

    #[test]
    fn test_alinea_default_bounds() {
        assert_eq!(ALINEA_DEFAULT_MIN_RATE_VEH_H_LN, 240.0);
        assert_eq!(ALINEA_DEFAULT_MAX_RATE_VEH_H_LN, 900.0);
    }

    // ── Section 5: adaptive signals ──────────────────────────────────────

    #[test]
    fn test_adaptive_signal_ranges() {
        assert_eq!(ADAPTIVE_SIGNAL_DELAY_REDUCTION_PCT_RANGE, (3.0, 24.0));
        assert_eq!(ADAPTIVE_SIGNAL_TTI_REDUCTION_PCT_RANGE, (3.0, 13.0));
    }

    #[test]
    fn test_adaptive_signal_default_is_range_midpoint() {
        let default_factor = adaptive_signal_sat_flow_adjustment(None);
        let midpoint_factor = adaptive_signal_sat_flow_adjustment(Some(13.5));
        assert!((default_factor - midpoint_factor).abs() < 1e-12);
        assert!(default_factor > 1.0);
    }

    #[test]
    fn test_adaptive_signal_clamped_to_published_range() {
        let below = adaptive_signal_sat_flow_adjustment(Some(0.0));
        let at_min = adaptive_signal_sat_flow_adjustment(Some(3.0));
        assert!((below - at_min).abs() < 1e-12, "must clamp below the range");

        let above = adaptive_signal_sat_flow_adjustment(Some(50.0));
        let at_max = adaptive_signal_sat_flow_adjustment(Some(24.0));
        assert!((above - at_max).abs() < 1e-12, "must clamp above the range");
    }

    #[test]
    fn test_adaptive_signal_monotonic_in_target_pct() {
        let low = adaptive_signal_sat_flow_adjustment(Some(3.0));
        let high = adaptive_signal_sat_flow_adjustment(Some(24.0));
        assert!(high > low, "a larger target delay reduction must imply a larger sat-flow bump");
    }
}
