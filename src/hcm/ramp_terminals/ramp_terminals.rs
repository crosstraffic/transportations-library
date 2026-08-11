//! HCM Chapter 23, Part B: Interchange Ramp Terminal Evaluation
//! (final design and operational analysis for signalized interchanges,
//! including DDIs).
//!
//! The computational steps follow HCM 7th Edition Exhibit 23-22:
//!
//! 1. Determine O-D demands and movement demands (Exhibit 23-20 with the
//!    Chapter 34 worksheets, Exhibits 34-163 through 34-177)
//! 2. Determine lane groups (Chapter 19 rules; supplied as inputs here)
//! 3. Determine adjusted saturation flow rates (Equation 23-14 with the
//!    interchange adjustments f_v, f_LU, f_DDI, and f_R of Equations
//!    23-15 through 23-23)
//! 4. Determine effective green adjustments due to interchange operation
//!    (Equations 23-24 through 23-39: downstream internal queue, DDI
//!    overlap phasing, and demand starvation lost times)
//! 5. Determine effective green adjustment due to closely spaced adjacent
//!    intersections (Equation 23-40; provided as a free function — the
//!    facility pipeline models the interchange itself)
//! 6. Determine performance of YIELD-controlled and free-flow turns
//!    (Equations 23-41 through 23-47 and 23-53 through 23-56)
//! 7. Determine v/c ratio and queue storage ratio (Equation 23-48 with
//!    the Chapter 31 Section 4 back-of-queue procedure)
//! 8. Determine control delay and experienced travel time for each O-D
//!    (Equations 23-49 / 23-50 with the Chapter 19 delay equations)
//! 9. Determine LOS (Exhibit 23-10; Equations 23-51 / 23-52 aggregation)
//!
//! Sources: EPUB files 175_Ch23_pt2_03.xhtml (core methodology),
//! 176_Ch23_pt2_04.xhtml (extensions), and 271_Ch34_04.xhtml (O-D /
//! turning movement worksheets). Numeric conventions cross-checked
//! against Chapter 34 Example Problems 1, 3, 4, 5, and 6
//! (269_Ch34_02*.xhtml).
//!
//! Implementation notes (documented deviations / conventions):
//! * Incremental delay d2 (Equation 19-26) is evaluated with the lane group
//!   capacity, which is what the equation's own variable list requires: c_A is
//!   "the average capacity (veh/h) ... equal to the capacity c computed in Step
//!   7", and the Step 7 c is a lane group capacity, not a per-lane one.
//!   Example Problems 3 and 5 agree with that reading (EP3 publishes d2 = 110.5
//!   s/veh for the eastbound external through, which reproduces at 110.36 with
//!   the lane group capacity of 1,672 veh/h against 119.9 with c/N = 557).
//!   // VERIFY-HCM: Example Problem 1 is the outlier. Its published d2 of 4.6
//!   // s/veh reproduces only on a per-lane basis (4.65), so the EP1 worked
//!   // values are inconsistent with the definition Equation 19-26 gives for
//!   // its own variable. Treated as a book defect alongside the Chapter 12 /
//!   // Chapter 20 errata, since two example problems and the equation text
//!   // outvote one worksheet. The affected EP1 expectations in
//!   // tests/chapter23_integration.rs are pinned to engine values.
//! * Interchange LOS (Step 9) is graded from the demand-weighted ETT alone.
//!   The Exhibit 23-10 "automatically LOS F" rule for v/c > 1 or R_Q > 1 is a
//!   per-O-D rule, and each O-D traveling through a flagged lane group still
//!   receives F. Step 9 explicitly anticipates a failing O-D being masked at
//!   the interchange level and directs the analyst to report the poorest O-D
//!   as context rather than to force the aggregate letter down. Exhibits 34-43
//!   and 34-57 confirm it, grading their interchanges E and D from ETT while
//!   carrying flagged O-Ds.
//! * Uniform delay d1 uses Equation 19-19 with the Equation 19-20
//!   progression factor (P = R_p g/C). This reproduces the Example 1
//!   movement delays; the Example 5 (DDI) published uniform delays are
//!   not reproducible from the printed equations (// VERIFY-HCM).

use serde::{Deserialize, Serialize};

use super::exhibits::*;
use crate::hcm::signalized::exhibits::{
    area_type_factor, bus_blockage_factor, default_lane_utilization_factor,
    heavy_vehicle_grade_factor, lane_width_factor, parking_factor,
    platoon_ratio_for_arrival_type, LaneUtilizationGroup, BASE_SATURATION_FLOW_METRO,
    EXTENSION_OF_EFFECTIVE_GREEN, START_UP_LOST_TIME,
};
use crate::hcm::signalized::signalized::{
    accel_decel_delay, average_vehicle_spacing, first_term_back_of_queue,
    queue_storage_ratio_eq, second_term_back_of_queue,
};
use crate::hcm::common::delay::{
    control_delay_roundabout, incremental_delay_signalized, initial_queue_delay,
    progression_factor, uniform_delay, upstream_filtering_factor, K_PRETIMED,
};
use crate::hcm::common::LevelOfService;

// ═══════════════════════════════════════════════════════════════════════════════
// Interchange forms and O-D demand structures (Step 1)
// ═══════════════════════════════════════════════════════════════════════════════

/// Service interchange forms covered by the Chapter 23 Part B methodology
/// (Exhibits 23-15 through 23-18). Conventional, compressed, and tight
/// urban diamonds share the `Diamond` computational form (they differ
/// only in the intersection spacing input D).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InterchangeForm {
    /// Conventional / compressed / tight urban diamond (Exhibit 23-15).
    Diamond,
    /// Diverging diamond interchange (Exhibit 23-16). O-D / turning
    /// movement conversion follows the diamond worksheet (Chapter 34,
    /// Example Problem 5 text).
    Ddi,
    /// Partial cloverleaf variants (Exhibit 23-17).
    ParcloA2Q,
    ParcloA4Q,
    ParcloAB2Q,
    ParcloAB4Q,
    ParcloB2Q,
    ParcloB4Q,
    /// Single-point urban interchange (Exhibit 23-18): one intersection.
    Spui,
}

/// O-D demands A–N of Exhibit 23-20 / Exhibit 34-162, veh/h.
///
/// For a diamond interchange (arterial east–west, Intersection I on the
/// west with the southbound ramps, Intersection II on the east with the
/// northbound ramps) the letters are:
/// * A — NB off-ramp to WB arterial (left from freeway)
/// * B — NB off-ramp to EB arterial (right from freeway)
/// * C — SB off-ramp to WB arterial (right from freeway)
/// * D — SB off-ramp to EB arterial (left from freeway)
/// * E — EB arterial to NB on-ramp (left onto freeway)
/// * F — EB arterial to SB on-ramp (right onto freeway)
/// * G — WB arterial to NB on-ramp (right onto freeway)
/// * H — WB arterial to SB on-ramp (left onto freeway)
/// * I — EB arterial through;  J — WB arterial through
/// * K, L — ramp through movements (frontage roads; usually 0)
/// * M, N — freeway U-turns (NB and SB; user-specified)
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct OdDemands {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
    pub e: f64,
    pub f: f64,
    pub g: f64,
    pub h: f64,
    pub i: f64,
    pub j: f64,
    pub k: f64,
    pub l: f64,
    pub m: f64,
    pub n: f64,
}

impl OdDemands {
    /// Demand for one O-D letter, veh/h.
    pub fn get(&self, m: OdMovement) -> f64 {
        match m {
            OdMovement::A => self.a,
            OdMovement::B => self.b,
            OdMovement::C => self.c,
            OdMovement::D => self.d,
            OdMovement::E => self.e,
            OdMovement::F => self.f,
            OdMovement::G => self.g,
            OdMovement::H => self.h,
            OdMovement::I => self.i,
            OdMovement::J => self.j,
            OdMovement::K => self.k,
            OdMovement::L => self.l,
            OdMovement::M => self.m,
            OdMovement::N => self.n,
        }
    }

    /// All demands divided by the peak hour factor (v = V / PHF).
    pub fn phf_adjusted(&self, phf: f64) -> OdDemands {
        let p = if phf > 0.0 { phf } else { 1.0 };
        OdDemands {
            a: self.a / p,
            b: self.b / p,
            c: self.c / p,
            d: self.d / p,
            e: self.e / p,
            f: self.f / p,
            g: self.g / p,
            h: self.h / p,
            i: self.i / p,
            j: self.j / p,
            k: self.k / p,
            l: self.l / p,
            m: self.m / p,
            n: self.n / p,
        }
    }

    /// Sum of all fourteen O-D demands, veh/h.
    pub fn total(&self) -> f64 {
        self.a + self.b
            + self.c
            + self.d
            + self.e
            + self.f
            + self.g
            + self.h
            + self.i
            + self.j
            + self.k
            + self.l
            + self.m
            + self.n
    }
}

/// Intersection turning movements of the Chapter 34 O-D worksheets
/// (Exhibits 34-163 through 34-170), veh/h. Fields not applicable to a
/// given interchange form are left at 0.0 (the worksheets shade them).
///
/// `*_ii` fields carry the Intersection II counterpart where a movement
/// exists at both intersections of AB / B-4Q parclos (e.g., `nb_right`
/// = NB RT(I), `nb_right_ii` = NB RT(II)).
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct TurningMovements {
    pub eb_ext_left: f64,
    pub eb_ext_right: f64,
    pub eb_ext_through: f64,
    pub eb_int_left: f64,
    pub eb_int_right: f64,
    pub eb_int_through: f64,
    pub wb_ext_left: f64,
    pub wb_ext_right: f64,
    pub wb_ext_through: f64,
    pub wb_int_left: f64,
    pub wb_int_right: f64,
    pub wb_int_through: f64,
    pub nb_left: f64,
    pub nb_left_ii: f64,
    pub nb_right: f64,
    pub nb_right_ii: f64,
    pub nb_through: f64,
    pub nb_uturn: f64,
    pub nb_uturn_ii: f64,
    pub sb_left: f64,
    pub sb_right: f64,
    pub sb_right_ii: f64,
    pub sb_through: f64,
    pub sb_uturn: f64,
}

/// HCM Exhibits 34-163 through 34-170: O-D movements from turning
/// movements for each interchange form. The freeway U-turn flows (M, N)
/// are taken from the user-specified U-turn fields.
pub fn od_from_turning_movements(form: InterchangeForm, tm: &TurningMovements) -> OdDemands {
    use InterchangeForm::*;
    match form {
        // Exhibit 34-169 (diamond; also applied to DDIs per Chapter 34
        // Example Problem 5).
        Diamond | Ddi => OdDemands {
            a: tm.nb_left - tm.nb_uturn,
            b: tm.nb_right,
            c: tm.sb_right,
            d: tm.sb_left - tm.sb_uturn,
            e: tm.eb_int_left - tm.sb_uturn,
            f: tm.eb_ext_right,
            g: tm.wb_ext_right,
            h: tm.wb_int_left - tm.nb_uturn,
            i: tm.eb_int_through - tm.sb_left + tm.sb_uturn,
            j: tm.wb_int_through - tm.nb_left + tm.nb_uturn,
            k: tm.nb_through,
            l: tm.sb_through,
            m: tm.nb_uturn,
            n: tm.sb_uturn,
        },
        // Exhibit 34-163 (Parclo A-2Q).
        ParcloA2Q => OdDemands {
            a: tm.nb_left - tm.nb_uturn,
            b: tm.nb_right,
            c: tm.sb_right,
            d: tm.sb_left - tm.sb_uturn,
            e: tm.eb_int_right - tm.sb_uturn,
            f: tm.eb_ext_left,
            g: tm.wb_ext_left,
            h: tm.wb_int_right - tm.nb_uturn,
            i: tm.eb_int_through - tm.sb_left + tm.sb_uturn,
            j: tm.wb_int_through - tm.nb_left + tm.nb_uturn,
            k: tm.nb_through,
            l: tm.sb_through,
            m: tm.nb_uturn,
            n: tm.sb_uturn,
        },
        // Exhibit 34-164 (Parclo A-4Q).
        ParcloA4Q => OdDemands {
            a: tm.nb_left - tm.nb_uturn,
            b: tm.nb_right,
            c: tm.sb_right,
            d: tm.sb_left - tm.sb_uturn,
            e: tm.eb_int_right - tm.sb_uturn,
            f: tm.eb_ext_right,
            g: tm.wb_ext_right,
            h: tm.wb_int_right - tm.nb_uturn,
            i: tm.eb_int_through - tm.sb_left + tm.sb_uturn,
            j: tm.wb_int_through - tm.nb_left + tm.nb_uturn,
            k: tm.nb_through,
            l: tm.sb_through,
            m: tm.nb_uturn,
            n: tm.sb_uturn,
        },
        // Exhibit 34-165 (Parclo AB-2Q). Both freeway U-turns are made by
        // the NB ramps: M = NB UT(II), N = NB UT(I).
        ParcloAB2Q => OdDemands {
            a: tm.nb_left_ii - tm.nb_uturn_ii,
            b: tm.nb_right_ii,
            c: tm.nb_left,
            d: tm.nb_right - tm.nb_uturn,
            e: tm.eb_int_right - tm.nb_uturn,
            f: tm.eb_ext_right,
            g: tm.wb_ext_left,
            h: tm.wb_int_left - tm.nb_uturn_ii,
            i: tm.eb_int_through - tm.nb_right + tm.nb_uturn,
            j: tm.wb_int_through - tm.nb_left_ii + tm.nb_uturn_ii,
            k: tm.nb_through,
            l: tm.sb_through,
            m: tm.nb_uturn_ii,
            n: tm.nb_uturn,
        },
        // Exhibit 34-166 (Parclo AB-4Q).
        ParcloAB4Q => OdDemands {
            a: tm.nb_left_ii - tm.nb_uturn_ii,
            b: tm.nb_right_ii,
            c: tm.sb_right,
            d: tm.nb_right - tm.nb_uturn,
            e: tm.eb_int_right - tm.nb_uturn,
            f: tm.eb_ext_right,
            g: tm.wb_ext_left,
            h: tm.wb_int_left - tm.nb_uturn_ii,
            i: tm.eb_int_through - tm.nb_right + tm.nb_uturn,
            j: tm.wb_int_through - tm.nb_left_ii + tm.nb_uturn_ii,
            k: tm.nb_through,
            l: tm.sb_through,
            m: tm.nb_uturn_ii,
            n: tm.nb_uturn,
        },
        // Exhibit 34-167 (Parclo B-2Q): M = SB UT, N = NB UT.
        ParcloB2Q => OdDemands {
            a: tm.sb_right - tm.sb_uturn,
            b: tm.sb_left,
            c: tm.nb_left,
            d: tm.nb_right - tm.nb_uturn,
            e: tm.eb_int_left - tm.nb_uturn,
            f: tm.eb_ext_right,
            g: tm.wb_ext_right,
            h: tm.wb_int_left - tm.sb_uturn,
            i: tm.eb_int_through - tm.nb_right + tm.nb_uturn,
            j: tm.wb_int_through - tm.sb_right + tm.sb_uturn,
            k: tm.nb_through,
            l: tm.sb_through,
            m: tm.sb_uturn,
            n: tm.nb_uturn,
        },
        // Exhibit 34-168 (Parclo B-4Q): M = SB UT, N = NB UT.
        ParcloB4Q => OdDemands {
            a: tm.sb_right_ii - tm.sb_uturn,
            b: tm.nb_right_ii,
            c: tm.sb_right,
            d: tm.nb_right - tm.nb_uturn,
            e: tm.eb_int_left - tm.nb_uturn,
            f: tm.eb_ext_right,
            g: tm.wb_ext_right,
            h: tm.wb_int_left - tm.sb_uturn,
            i: tm.eb_int_through - tm.nb_right + tm.nb_uturn,
            j: tm.wb_int_through - tm.sb_right_ii + tm.sb_uturn,
            k: tm.nb_through,
            l: tm.sb_through,
            m: tm.sb_uturn,
            n: tm.nb_uturn,
        },
        // Exhibit 34-170 (SPUI): single intersection; the external fields
        // carry the approach turning movements.
        Spui => OdDemands {
            a: tm.nb_left,
            b: tm.nb_right,
            c: tm.sb_right,
            d: tm.sb_left,
            e: tm.eb_ext_left,
            f: tm.eb_ext_right,
            g: tm.wb_ext_right,
            h: tm.wb_ext_left,
            i: tm.eb_ext_through,
            j: tm.wb_ext_through,
            k: tm.nb_through,
            l: tm.sb_through,
            m: tm.nb_uturn,
            n: tm.sb_uturn,
        },
    }
}

/// HCM Exhibits 34-171 through 34-177: turning movements from O-D
/// movements (the algebraic inverse of `od_from_turning_movements`; the
/// diamond mapping matches Exhibit 34-176 as printed).
pub fn turning_movements_from_od(form: InterchangeForm, od: &OdDemands) -> TurningMovements {
    use InterchangeForm::*;
    let mut tm = TurningMovements::default();
    match form {
        // Exhibit 34-176 (diamond / DDI).
        Diamond | Ddi => {
            tm.eb_ext_right = od.f;
            tm.eb_ext_through = od.i + od.e;
            tm.eb_int_left = od.e + od.n;
            tm.eb_int_through = od.i + od.d;
            tm.wb_int_left = od.h + od.m;
            tm.wb_int_through = od.j + od.a;
            tm.wb_ext_right = od.g;
            tm.wb_ext_through = od.j + od.h;
            tm.nb_left = od.a + od.m;
            tm.nb_right = od.b;
            tm.nb_through = od.k;
            tm.nb_uturn = od.m;
            tm.sb_left = od.d + od.n;
            tm.sb_right = od.c;
            tm.sb_through = od.l;
            tm.sb_uturn = od.n;
        }
        // Exhibit 34-171 (Parclo A-2Q).
        ParcloA2Q => {
            tm.eb_ext_left = od.f;
            tm.eb_ext_through = od.i + od.e;
            tm.eb_int_right = od.e + od.n;
            tm.eb_int_through = od.i + od.d;
            tm.wb_ext_left = od.g;
            tm.wb_ext_through = od.j + od.h;
            tm.wb_int_right = od.h + od.m;
            tm.wb_int_through = od.j + od.a;
            tm.nb_left = od.a + od.m;
            tm.nb_right = od.b;
            tm.nb_through = od.k;
            tm.nb_uturn = od.m;
            tm.sb_left = od.d + od.n;
            tm.sb_right = od.c;
            tm.sb_through = od.l;
            tm.sb_uturn = od.n;
        }
        // Exhibit 34-171 (Parclo A-4Q; outer-ramp rights instead of lefts).
        ParcloA4Q => {
            tm.eb_ext_right = od.f;
            tm.eb_ext_through = od.i + od.e;
            tm.eb_int_right = od.e + od.n;
            tm.eb_int_through = od.i + od.d;
            tm.wb_ext_right = od.g;
            tm.wb_ext_through = od.j + od.h;
            tm.wb_int_right = od.h + od.m;
            tm.wb_int_through = od.j + od.a;
            tm.nb_left = od.a + od.m;
            tm.nb_right = od.b;
            tm.nb_through = od.k;
            tm.nb_uturn = od.m;
            tm.sb_left = od.d + od.n;
            tm.sb_right = od.c;
            tm.sb_through = od.l;
            tm.sb_uturn = od.n;
        }
        // Exhibit 34-172 (Parclo AB-2Q).
        ParcloAB2Q => {
            tm.eb_ext_right = od.f;
            tm.eb_ext_through = od.i + od.e;
            tm.eb_int_right = od.e + od.n;
            tm.eb_int_through = od.i + od.d;
            tm.wb_ext_left = od.g;
            tm.wb_ext_through = od.j + od.h;
            tm.wb_int_left = od.h + od.m;
            tm.wb_int_through = od.j + od.a;
            tm.nb_left = od.c;
            tm.nb_right = od.d + od.n;
            tm.nb_left_ii = od.a + od.m;
            tm.nb_right_ii = od.b;
            tm.nb_uturn = od.n;
            tm.nb_uturn_ii = od.m;
            tm.nb_through = od.k;
            tm.sb_through = od.l;
        }
        // Exhibit 34-173 (Parclo AB-4Q).
        ParcloAB4Q => {
            tm.eb_ext_right = od.f;
            tm.eb_ext_through = od.i + od.e;
            tm.eb_int_right = od.e + od.n;
            tm.eb_int_through = od.i + od.d;
            tm.wb_ext_left = od.g;
            tm.wb_ext_through = od.j + od.h;
            tm.wb_int_left = od.h + od.m;
            tm.wb_int_through = od.j + od.a;
            tm.sb_right = od.c;
            tm.nb_right = od.d + od.n;
            tm.nb_left_ii = od.a + od.m;
            tm.nb_right_ii = od.b;
            tm.nb_uturn = od.n;
            tm.nb_uturn_ii = od.m;
            tm.nb_through = od.k;
            tm.sb_through = od.l;
        }
        // Exhibit 34-174 (Parclo B-2Q).
        ParcloB2Q => {
            tm.eb_ext_right = od.f;
            tm.eb_ext_through = od.i + od.e;
            tm.eb_int_left = od.e + od.n;
            tm.eb_int_through = od.i + od.d;
            tm.wb_ext_right = od.g;
            tm.wb_ext_through = od.j + od.h;
            tm.wb_int_left = od.h + od.m;
            tm.wb_int_through = od.j + od.a;
            tm.nb_left = od.c;
            tm.nb_right = od.d + od.n;
            tm.sb_left = od.b;
            tm.sb_right = od.a + od.m;
            tm.sb_uturn = od.m;
            tm.nb_uturn = od.n;
            tm.nb_through = od.k;
            tm.sb_through = od.l;
        }
        // Exhibit 34-175 (Parclo B-4Q).
        ParcloB4Q => {
            tm.eb_ext_right = od.f;
            tm.eb_ext_through = od.i + od.e;
            tm.eb_int_left = od.e + od.n;
            tm.eb_int_through = od.i + od.d;
            tm.wb_ext_right = od.g;
            tm.wb_ext_through = od.j + od.h;
            tm.wb_int_left = od.h + od.m;
            tm.wb_int_through = od.j + od.a;
            tm.nb_right = od.d + od.n;
            tm.nb_right_ii = od.b;
            tm.sb_right = od.c;
            tm.sb_right_ii = od.a + od.m;
            tm.sb_uturn = od.m;
            tm.nb_uturn = od.n;
            tm.nb_through = od.k;
            tm.sb_through = od.l;
        }
        // Exhibit 34-177 (SPUI).
        Spui => {
            tm.eb_ext_left = od.e;
            tm.eb_ext_right = od.f;
            tm.eb_ext_through = od.i;
            tm.wb_ext_left = od.h;
            tm.wb_ext_right = od.g;
            tm.wb_ext_through = od.j;
            tm.nb_left = od.a;
            tm.nb_right = od.b;
            tm.nb_through = od.k;
            tm.nb_uturn = od.m;
            tm.sb_left = od.d;
            tm.sb_right = od.c;
            tm.sb_through = od.l;
            tm.sb_uturn = od.n;
        }
    }
    tm
}

// ═══════════════════════════════════════════════════════════════════════════════
// Green intervals and common green time (Exhibit 23-28)
// ═══════════════════════════════════════════════════════════════════════════════

/// One displayed green interval within the cycle. `begin_s` is measured
/// from the system reference time (Chapter 34 examples reference the
/// beginning of green of Phase 1 at Intersection I); the interval may
/// wrap past the end of the cycle.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct GreenInterval {
    /// Beginning of green, s (within [0, C)).
    pub begin_s: f64,
    /// Displayed green duration, s.
    pub duration_s: f64,
}

/// Common green time between two sets of green intervals on a common
/// cycle (HCM Chapter 23, Step 4: "the amount of time during which both
/// phases have a green indication"; Exhibit 23-28).
pub fn common_green_time(a: &[GreenInterval], b: &[GreenInterval], cycle_s: f64) -> f64 {
    if cycle_s <= 0.0 {
        return 0.0;
    }
    // Unroll wrapped intervals onto [0, 2C) and intersect on [0, C) by
    // splitting at the cycle boundary.
    let split = |ivs: &[GreenInterval]| -> Vec<(f64, f64)> {
        let mut out = Vec::new();
        for iv in ivs {
            if iv.duration_s <= 0.0 {
                continue;
            }
            let b0 = iv.begin_s.rem_euclid(cycle_s);
            let e0 = b0 + iv.duration_s.min(cycle_s);
            if e0 <= cycle_s {
                out.push((b0, e0));
            } else {
                out.push((b0, cycle_s));
                out.push((0.0, e0 - cycle_s));
            }
        }
        out
    };
    let sa = split(a);
    let sb = split(b);
    let mut total = 0.0;
    for &(ab, ae) in &sa {
        for &(bb, be) in &sb {
            total += (ae.min(be) - ab.max(bb)).max(0.0);
        }
    }
    total
}

// ═══════════════════════════════════════════════════════════════════════════════
// Step 4 lost-time building blocks (Equations 23-24 through 23-40)
// ═══════════════════════════════════════════════════════════════════════════════

/// HCM Equation 23-33 / 23-34: average per-lane queue length (ft) for the
/// internal through movement at the beginning of an upstream phase
///
/// `Q = [0.0107 v_f/N_f − 7.96 G_D/C − 0.082 CG + 7.96 G_f/C] L_h >= 0`
///
/// * `feeding_flow` / `feeding_lanes` — flow (veh/h) and lanes of the
///   *other* movement feeding the internal link (the ramp feed when the
///   subject phase is the arterial phase, Equation 23-33, and vice versa,
///   Equation 23-34)
/// * `feeding_green_s` — green interval of that feeding movement G, s
/// * `downstream_green_s` — green interval of the downstream internal
///   through movement G_D, s
/// * `common_green_s` — common green CG_UD or CG_RD between the *subject*
///   upstream movement and the downstream through green, s
/// * `queue_spacing_ft` — average queue spacing L_h, ft/veh
///
/// Validated against Chapter 34 Exhibits 34-10, 34-37 (QR = 108.6 ft),
/// and 34-51.
#[allow(clippy::too_many_arguments)] // mirrors the HCM equation parameter list
pub fn downstream_queue_length_ft(
    feeding_flow: f64,
    feeding_lanes: u32,
    feeding_green_s: f64,
    downstream_green_s: f64,
    common_green_s: f64,
    cycle_s: f64,
    queue_spacing_ft: f64,
) -> f64 {
    if cycle_s <= 0.0 {
        return 0.0;
    }
    let q = 0.0107 * feeding_flow / feeding_lanes.max(1) as f64
        - 7.96 * downstream_green_s / cycle_s
        - 0.082 * common_green_s
        + 7.96 * feeding_green_s / cycle_s;
    (q * queue_spacing_ft).max(0.0)
}

/// HCM Equations 23-29 / 23-30 / 23-40: additional lost time on an
/// upstream approach due to the presence of a downstream queue
///
/// `L_D = G − 0.106 DQ − 5.39 CG/C >= 0`, and 0 when DQ > 200 ft.
///
/// * `green_s` — green interval of the subject upstream approach G, s
/// * `distance_to_queue_ft` — DQ = D − Q at the beginning of the upstream
///   green, ft
/// * `common_green_s` — common green between the subject upstream green
///   and the downstream through green, s
///
/// Validated against Chapter 34 Exhibit 34-37 (SB-L: 5.5 s).
pub fn downstream_queue_lost_time(
    green_s: f64,
    distance_to_queue_ft: f64,
    common_green_s: f64,
    cycle_s: f64,
) -> f64 {
    if distance_to_queue_ft > DOWNSTREAM_QUEUE_LOST_TIME_MAX_DISTANCE_FT || cycle_s <= 0.0 {
        return 0.0;
    }
    (green_s - 0.106 * distance_to_queue_ft - 5.39 * common_green_s / cycle_s).max(0.0)
}

/// HCM Equation 23-39: initial internal queue (veh) at the beginning of
/// the interval with demand starvation potential
///
/// `Q_initial = [v_RL C / (N_RL 3,600) − (CG_RD − t_L)/h_I]
///            + [v_A C / (N_A 3,600) − (CG_UD − t_L)/h_I]`
///
/// * `ramp_left_flow` / `arterial_flow` — upstream feed flows, veh/h
/// * `lost_time_per_phase_s` — t_L from Equation 23-24 / 23-25, s
/// * `sat_headway_internal_s` — h_I = 3,600 / (internal saturation flow
///   per lane), s
///
/// CG values below t_L are replaced by t_L (Chapter 23 text). Validated
/// against Chapter 34 Exhibit 34-52 (6.8 and 2.8 veh).
#[allow(clippy::too_many_arguments)] // mirrors the HCM equation parameter list
pub fn demand_starvation_initial_queue(
    ramp_left_flow: f64,
    ramp_left_lanes: u32,
    arterial_flow: f64,
    arterial_lanes: u32,
    cycle_s: f64,
    cg_rd_s: f64,
    cg_ud_s: f64,
    lost_time_per_phase_s: f64,
    sat_headway_internal_s: f64,
) -> f64 {
    let h = sat_headway_internal_s.max(1e-9);
    let t_l = lost_time_per_phase_s;
    let cg_rd = cg_rd_s.max(t_l);
    let cg_ud = cg_ud_s.max(t_l);
    (ramp_left_flow * cycle_s / (ramp_left_lanes.max(1) as f64 * 3_600.0) - (cg_rd - t_l) / h)
        + (arterial_flow * cycle_s / (arterial_lanes.max(1) as f64 * 3_600.0)
            - (cg_ud - t_l) / h)
}

/// HCM Equation 23-38: additional lost time due to demand starvation
/// `L_DS = CG_DS − Q_initial h_I >= 0` (0 when the queue discharge time
/// reaches the starvation window).
pub fn demand_starvation_lost_time(
    cg_ds_s: f64,
    q_initial_veh: f64,
    sat_headway_internal_s: f64,
) -> f64 {
    (cg_ds_s - q_initial_veh.max(0.0) * sat_headway_internal_s).max(0.0)
}

/// HCM Equation 23-37: lost time on a signalized DDI off-ramp movement
/// due to overlap phasing
///
/// `L_OL-DDI = (W + L − D) / (1.467 S_f)`
///
/// * `clear_zone_width_ft` — width of the clear zone W for the longest
///   vehicle path, ft
/// * `vehicle_length_ft` — design vehicle length L (typically 20 ft)
/// * `stopbar_to_conflict_ft` — distance D from the ramp stop bar to the
///   conflict point, ft
/// * `free_flow_speed_mph` — free-flow (design) speed S_f, mi/h
///
/// // VERIFY-HCM: Chapter 34 Example Problem 5 (Exhibit 34-63) publishes
/// // 6.5 s / 4.9 s, which correspond to (W + L + D)/(1.467 × 25 mi/h) —
/// // the printed Equation 23-37 subtracts D. The equation is implemented
/// // as printed; supply the published values directly through the
/// // `overlap_lost_time_s` input to reproduce Example Problem 5.
pub fn ddi_overlap_lost_time(
    clear_zone_width_ft: f64,
    vehicle_length_ft: f64,
    stopbar_to_conflict_ft: f64,
    free_flow_speed_mph: f64,
) -> f64 {
    if free_flow_speed_mph <= 0.0 {
        return 0.0;
    }
    ((clear_zone_width_ft + vehicle_length_ft - stopbar_to_conflict_ft)
        / (1.467 * free_flow_speed_mph))
        .max(0.0)
}

/// HCM Equation 23-24 / 23-25 / 23-26: adjusted lost time
/// `t_L' = l_1 + L_D + L_OL-DDI + Y − e` (external / ramp) and
/// `t_L'' = l_1 + L_DS + Y − e` (internal).
pub fn adjusted_lost_time(
    start_up_lost_time_s: f64,
    additional_lost_time_s: f64,
    yellow_all_red_s: f64,
    extension_of_green_s: f64,
) -> f64 {
    (start_up_lost_time_s + additional_lost_time_s + yellow_all_red_s - extension_of_green_s)
        .max(0.0)
}

// ═══════════════════════════════════════════════════════════════════════════════
// Step 6 building blocks: YIELD-controlled DDI turns
// (Equations 23-41 through 23-47 and 23-53 through 23-56)
// ═══════════════════════════════════════════════════════════════════════════════

/// HCM Equation 23-42 (Siegloch): capacity of the gap acceptance regime
/// `c_GA = 3,600/t_f exp(−(t_c − t_f/2) q_c / 3,600)`.
///
/// * `conflicting_flow` — conflicting flow rate q_c, veh/h
///
/// Validated against Chapter 34 Exhibit 34-68 (M7: 541 veh/h).
pub fn yield_gap_acceptance_capacity(
    critical_headway_s: f64,
    follow_up_headway_s: f64,
    conflicting_flow: f64,
) -> f64 {
    3_600.0 / follow_up_headway_s
        * (-(critical_headway_s - follow_up_headway_s / 2.0) * conflicting_flow / 3_600.0).exp()
}

/// HCM Equation 23-44: capacity of the no-conflicting-flow regime
/// `c_NCF = 3,600 / t_f`.
pub fn yield_no_conflict_capacity(follow_up_headway_s: f64) -> f64 {
    3_600.0 / follow_up_headway_s
}

/// HCM Equation 23-54: time to clear the conflicting queue for an
/// isolated interchange with random arrivals
/// `t_CQ,free = r v_app / (s_DDI − v_app)`.
///
/// * `red_s` — effective red of the conflicting movement r, s
/// * `approach_flow` — conflicting approach flow rate per lane, veh/h
/// * `sat_flow` — conflicting approach saturation flow rate per lane, veh/h
///
/// Validated against Chapter 34 Exhibit 34-67 (M7: 22.4 s).
pub fn yield_time_to_clear_queue_random(red_s: f64, approach_flow: f64, sat_flow: f64) -> f64 {
    if sat_flow <= approach_flow {
        return red_s.max(0.0); // saturated: the queue does not clear
    }
    (red_s * approach_flow / (sat_flow - approach_flow)).max(0.0)
}

/// HCM Equation 23-55: time to clear the conflicting queue for a
/// coordinated interchange
/// `t_CQ,coord = C (1 − P) / (s_DDI/v_app − [P (g/C)^-1])`.
///
/// Reduces to Equation 23-54 when P = g/C.
pub fn yield_time_to_clear_queue_coordinated(
    cycle_s: f64,
    proportion_on_green: f64,
    approach_flow: f64,
    sat_flow: f64,
    green_s: f64,
) -> f64 {
    if approach_flow <= 0.0 || green_s <= 0.0 {
        return 0.0;
    }
    let denom = sat_flow / approach_flow - proportion_on_green * cycle_s / green_s;
    if denom <= 0.0 {
        return green_s;
    }
    (cycle_s * (1.0 - proportion_on_green) / denom).max(0.0)
}

/// HCM Equation 23-56: time for the last queued vehicle to clear the
/// distance between the crossover stop bar and the yield conflict point
/// `t_clear = x_clear / (1.47 S_f)`.
///
/// Validated against Chapter 34 Exhibit 34-67 (200 ft at 25 mi/h: 5.5 s).
pub fn yield_clearance_time(distance_ft: f64, speed_mph: f64) -> f64 {
    if speed_mph <= 0.0 {
        return 0.0;
    }
    distance_ft / (1.47 * speed_mph)
}

/// HCM Equation 23-47 (with Equations 23-43, 23-45, and 23-53): combined
/// capacity of a YIELD-controlled DDI turn
///
/// `c_YCT = 1/C [ c_GA (g − t_CQ − t_clear) + c_NCF (C − g) ]`
///
/// where g is the effective green of the conflicting crossover movement.
/// The gap-acceptance interval is floored at zero (Chapter 34 Example
/// Problem 6, Exhibit 34-68 note).
pub fn yield_turn_capacity(
    cycle_s: f64,
    conflicting_green_s: f64,
    t_cq_s: f64,
    t_clear_s: f64,
    c_ga: f64,
    c_ncf: f64,
) -> f64 {
    if cycle_s <= 0.0 {
        return 0.0;
    }
    let ga_time = (conflicting_green_s - t_cq_s - t_clear_s).max(0.0);
    (c_ga * ga_time + c_ncf * (cycle_s - conflicting_green_s).max(0.0)) / cycle_s
}

// ═══════════════════════════════════════════════════════════════════════════════
// Step 8 building block: extra distance travel time (Equation 23-50)
// ═══════════════════════════════════════════════════════════════════════════════

/// HCM Equation 23-50: extra distance travel time
/// `EDTT = D_t / (1.47 v_D) + a`
///
/// * `distance_ft` — signed distance traveled along the diverted movement
///   D_t, ft (negative for right turns per the Exhibit 23-8 convention,
///   which produces negative EDTT for right turns)
/// * `design_speed_mph` — design speed of the loop ramp or diverted
///   movement v_D, mi/h
/// * `accel_decel_s` — delay due to deceleration into and acceleration
///   out of the turns a, s (5 s for a loop ramp movement; 0 in the
///   Chapter 34 diamond examples)
pub fn extra_distance_travel_time(distance_ft: f64, design_speed_mph: f64, accel_decel_s: f64) -> f64 {
    if design_speed_mph <= 0.0 {
        return 0.0;
    }
    let tt = distance_ft / (1.47 * design_speed_mph);
    if distance_ft >= 0.0 {
        tt + accel_decel_s
    } else {
        tt - accel_decel_s
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Lane groups
// ═══════════════════════════════════════════════════════════════════════════════

/// Lane groups of a two-intersection interchange (diamond / parclo / DDI
/// naming; arterial east–west). For a DDI the mapping to the Chapter 34
/// Example Problem 5 movement numbers is: M6 = `EbExtThrough`,
/// M1 = `EbIntThrough`, M2 = `WbExtThrough`, M5 = `WbIntThrough`,
/// M3 = `NbRampLeft`, M4 = `NbRampRight`, M7 = `SbRampLeft`,
/// M8 = `SbRampRight` (the DDI has no internal left-turn lane groups —
/// left turns onto the freeway are free-flowing at the internal
/// crossover). For a SPUI the `Ext` groups are the single-intersection
/// arterial groups and `EbIntLeft` / `WbIntLeft` are the arterial
/// left-turn groups.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InterchangeMovement {
    /// External arterial through (+ right when shared) at Intersection I.
    EbExtThrough,
    /// Internal arterial through at Intersection II.
    EbIntThrough,
    /// Internal left onto the freeway at Intersection II.
    EbIntLeft,
    /// External arterial through (+ right when shared) at Intersection II.
    WbExtThrough,
    /// Internal arterial through at Intersection I.
    WbIntThrough,
    /// Internal left onto the freeway at Intersection I.
    WbIntLeft,
    /// NB off-ramp left turn (Intersection II).
    NbRampLeft,
    /// NB off-ramp right turn (Intersection II).
    NbRampRight,
    /// SB off-ramp left turn (Intersection I).
    SbRampLeft,
    /// SB off-ramp right turn (Intersection I).
    SbRampRight,
}

/// Traffic control of a lane group.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LaneGroupControl {
    /// Signalized movement (the default).
    Signalized,
    /// YIELD-controlled DDI turn evaluated with the Step 6 three-regime
    /// capacity model.
    YieldControlled(YieldTurnInput),
    /// Free-flowing movement (zero control delay; e.g., DDI free-flow
    /// right-turn bypass or the free-flowing internal left at a DDI).
    FreeFlow,
}

/// Inputs for a YIELD-controlled DDI turn (Step 6, Equations 23-41
/// through 23-47 and 23-53 through 23-56).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YieldTurnInput {
    /// Critical headway t_c, s (Exhibit 23-36 defaults: 3.9 left / 1.8
    /// right).
    pub critical_headway_s: f64,
    /// Follow-up headway t_f, s (Exhibit 23-36 defaults: 2.6 left / 2.4
    /// right).
    pub follow_up_headway_s: f64,
    /// Conflicting (crossover) flow rate q_c, veh/h.
    pub conflicting_flow_veh_h: f64,
    /// Conflicting crossover approach flow per lane, veh/h/ln (queue
    /// clearance, Equation 23-54).
    pub conflicting_flow_per_lane: f64,
    /// Conflicting crossover saturation flow per lane, veh/h/ln.
    pub conflicting_sat_flow_per_lane: f64,
    /// Effective green of the conflicting crossover movement g, s.
    pub conflicting_green_s: f64,
    /// Distance from the crossover stop bar to the yield conflict point
    /// x_clear, ft.
    pub clearance_distance_ft: f64,
    /// Free-flow speed between the stop bar and the conflict point, mi/h.
    pub clearance_speed_mph: f64,
    /// Proportion of conflicting arrivals on green P for a coordinated
    /// interchange (Equation 23-55); `None` for random arrivals
    /// (Equation 23-54).
    pub proportion_on_green: Option<f64>,
}

/// Input description of one interchange lane group.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaneGroupInput {
    /// Which interchange movement the lane group serves.
    pub movement: InterchangeMovement,
    /// Number of lanes N.
    pub lanes: u32,
    /// Displayed green interval(s) within the cycle (a movement may
    /// receive green twice per cycle, e.g., the internal through of a
    /// three-phase diamond).
    pub greens: Vec<GreenInterval>,
    /// Yellow-plus-all-red change-and-clearance interval Y, s.
    pub yellow_all_red_s: f64,
    /// Traffic control (signalized / YIELD / free-flow).
    pub control: LaneGroupControl,
    /// Turn radius of the turning path, ft (`None` for through
    /// movements; Equation 23-19).
    pub turn_radius_ft: Option<f64>,
    /// Turn radius of the right-turn path sharing the lane group, ft
    /// (external through-plus-right groups).
    pub shared_right_turn_radius_ft: Option<f64>,
    /// Percent heavy vehicles, %.
    pub pct_heavy_vehicles: f64,
    /// Approach grade, % (positive upgrade).
    pub grade_pct: f64,
    /// Average lane width, ft (Exhibit 23-21 default 12 ft).
    pub lane_width_ft: f64,
    /// Parking maneuver rate adjacent to the lane group, maneuvers/h
    /// (`None` = no parking lane).
    pub parking_maneuvers_h: Option<f64>,
    /// Local bus stopping rate, buses/h.
    pub bus_stops_h: f64,
    /// Arrival type (HCM Exhibit 19-13; Chapter 34 interchange examples
    /// use 4 for arterial movements and 3 elsewhere).
    pub arrival_type: u8,
    /// Available queue storage per lane L_a, ft (`None` = unbounded; the
    /// queue storage ratio is then not computed).
    pub storage_ft: Option<f64>,
    /// Field-measured lane utilization factor f_LU override (preferred
    /// by the chapter when available). `None` = model estimate
    /// (Equations 23-16 through 23-18) for external arterial approaches
    /// and 1.0 elsewhere.
    pub lane_utilization_override: Option<f64>,
    /// Direct additional lost time due to a downstream queue L_D, s
    /// (`None` = computed from Equations 23-29 through 23-34 for
    /// diamond / parclo external and ramp-left groups; DDIs supply the
    /// shock-wave estimate directly per the Chapter 23 DDI procedure).
    pub downstream_queue_lost_time_s: Option<f64>,
    /// Lost time due to DDI overlap phasing L_OL-DDI, s (Equation 23-37
    /// or `ddi_overlap_lost_time`); applies to signalized DDI off-ramp
    /// movements.
    pub overlap_lost_time_s: f64,
    /// Start-up lost time l_1, s (default 2.0).
    pub start_up_lost_time_s: f64,
    /// Extension of effective green into the clearance interval e, s
    /// (default 2.0).
    pub extension_of_green_s: f64,
    /// Upstream filtering adjustment factor I override (`None` =
    /// computed from the upstream external v/c for internal lane groups
    /// via Equation 19-6, 1.0 for external groups).
    pub upstream_filtering_override: Option<f64>,
    /// Speed limit used for the acceleration–deceleration term of the
    /// back-of-queue procedure, mi/h (Equation 31-132).
    pub speed_limit_mph: f64,
    /// Initial queue at the start of the analysis period Q_b, veh.
    pub initial_queue_veh: f64,
    /// Demand flow rate override, veh/h (`None` = composed from the O-D
    /// demands per the Exhibit 34-176 worksheet composition).
    pub demand_override_veh_h: Option<f64>,
}

impl LaneGroupInput {
    /// A signalized lane group with the Chapter 23 defaults.
    pub fn new(movement: InterchangeMovement, lanes: u32, green: GreenInterval, y: f64) -> Self {
        LaneGroupInput {
            movement,
            lanes,
            greens: vec![green],
            yellow_all_red_s: y,
            control: LaneGroupControl::Signalized,
            turn_radius_ft: None,
            shared_right_turn_radius_ft: None,
            pct_heavy_vehicles: 0.0,
            grade_pct: 0.0,
            lane_width_ft: 12.0,
            parking_maneuvers_h: None,
            bus_stops_h: 0.0,
            arrival_type: 3,
            storage_ft: None,
            lane_utilization_override: None,
            downstream_queue_lost_time_s: None,
            overlap_lost_time_s: 0.0,
            start_up_lost_time_s: START_UP_LOST_TIME,
            extension_of_green_s: EXTENSION_OF_EFFECTIVE_GREEN,
            upstream_filtering_override: None,
            speed_limit_mph: 40.0,
            initial_queue_veh: 0.0,
            demand_override_veh_h: None,
        }
    }

    /// Total displayed green G, s.
    pub fn total_green_s(&self) -> f64 {
        self.greens.iter().map(|g| g.duration_s).sum()
    }
}

/// Computed performance of one lane group. `Option` fields are filled by
/// the corresponding methodology step.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaneGroupResult {
    pub movement: InterchangeMovement,
    /// Demand flow rate v (PHF-adjusted), veh/h (Step 1).
    pub flow_rate: f64,
    /// Lane utilization factor f_LU (Step 3).
    pub lane_utilization: Option<f64>,
    /// Traffic pressure factor f_v (Step 3, Equation 23-15).
    pub traffic_pressure: Option<f64>,
    /// Adjusted saturation flow rate s, veh/h (lane group total; Step 3,
    /// Equation 23-14).
    pub sat_flow: Option<f64>,
    /// Additional lost time due to a downstream queue L_D, s (Step 4).
    pub downstream_queue_lost_time_s: Option<f64>,
    /// Additional lost time due to demand starvation L_DS, s (Step 4).
    pub demand_starvation_lost_time_s: Option<f64>,
    /// Adjusted lost time t_L' / t_L'', s (Step 4).
    pub adjusted_lost_time_s: Option<f64>,
    /// Effective green g' / g'', s (Step 4, Equations 23-27 / 23-28).
    pub effective_green_s: Option<f64>,
    /// Capacity c, veh/h (Step 7; YIELD capacity c_YCT for
    /// YIELD-controlled turns).
    pub capacity: Option<f64>,
    /// Volume-to-capacity ratio X (Step 7, Equation 23-48).
    pub vc_ratio: Option<f64>,
    /// Upstream filtering factor I used for d2.
    pub upstream_filtering: Option<f64>,
    /// 50th-percentile back of queue Q, veh/ln (Step 7).
    pub back_of_queue_veh: Option<f64>,
    /// Queue storage ratio R_Q (Step 7).
    pub queue_storage_ratio: Option<f64>,
    /// Uniform delay d1, s/veh (Step 8).
    pub uniform_delay_s: Option<f64>,
    /// Incremental delay d2, s/veh (Step 8).
    pub incremental_delay_s: Option<f64>,
    /// Initial queue delay d3, s/veh (Step 8).
    pub initial_queue_delay_s: Option<f64>,
    /// Control delay d, s/veh (Step 8).
    pub control_delay_s: Option<f64>,
}

// ═══════════════════════════════════════════════════════════════════════════════
// O-D results
// ═══════════════════════════════════════════════════════════════════════════════

/// Computed performance of one O-D movement (Steps 8 and 9).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OdResult {
    pub movement: OdMovement,
    /// PHF-adjusted demand, veh/h.
    pub demand: f64,
    /// Sum of control delays along the O-D path, s/veh.
    pub control_delay_s: f64,
    /// Extra distance travel time EDTT, s/veh (Equation 23-50).
    pub edtt_s: f64,
    /// Experienced travel time ETT = Σd + ΣEDTT, s/veh (Equation 23-49).
    pub ett_s: f64,
    /// v/c > 1 for any lane group on the path.
    pub vc_exceeds_one: bool,
    /// R_Q > 1 for any lane group on the path.
    pub rq_exceeds_one: bool,
    /// O-D level of service (Exhibit 23-10).
    pub los: LevelOfService,
}

/// Per-O-D extra travel distance input for the EDTT computation
/// (Equation 23-50). Distances follow the Exhibit 23-8 sign convention:
/// positive for left turns, negative for right turns, and (small)
/// positive values for DDI arterial through movements crossing over.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct ExtraDistance {
    /// Signed extra travel distance D_t, ft.
    pub distance_ft: f64,
    /// Deceleration/acceleration delay a, s (5 s for loop ramps).
    pub accel_decel_s: f64,
}

// ═══════════════════════════════════════════════════════════════════════════════
// Interchange facility
// ═══════════════════════════════════════════════════════════════════════════════

/// A signalized interchange ramp terminal facility (two ramp terminal
/// intersections — or one for a SPUI — plus the interchange
/// configuration), analyzed with the HCM Chapter 23 Part B final design
/// and operational analysis methodology (Exhibit 23-22).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Interchange {
    /// Interchange configuration.
    pub form: InterchangeForm,
    /// Cycle length C, s (both intersections share the cycle).
    pub cycle_length_s: f64,
    /// Analysis period T, h (0.25 for a 15-min period).
    pub analysis_period_h: f64,
    /// Base saturation flow rate s_0, pc/h/ln (Exhibit 23-21 default
    /// 1,900 for metro population >= 250,000).
    pub base_saturation_flow: f64,
    /// CBD-like area type (f_a = 0.90).
    pub area_type_cbd: bool,
    /// Peak hour factor applied to the O-D demands.
    pub peak_hour_factor: f64,
    /// Distance between the two intersections D, ft (storage space of
    /// the internal link).
    pub distance_between_intersections_ft: f64,
    /// Average queue spacing in a stationary queue L_h, ft/veh
    /// (Exhibit 23-29 default 25).
    pub queue_spacing_ft: f64,
    /// O-D demand volumes (unadjusted), veh/h.
    pub od: OdDemands,
    /// Whether the external arterial approaches include the right-turn
    /// O-D (v_F / v_G) in a shared lane. `false` when an exclusive
    /// right-turn lane (or a DDI free-flow bypass) removes the right
    /// turns from the external through group; the right-turn O-D is then
    /// excluded from the group demand and from Equation 23-17
    /// (Exhibit 23-24 note).
    pub eb_external_right_shared: bool,
    pub wb_external_right_shared: bool,
    /// DDI external-crossover lane configurations (Exhibit 23-25) for
    /// the Equation 23-18 lane utilization model. Ignored for other
    /// forms.
    pub ddi_eb_lane_config: Option<DdiLaneConfiguration>,
    pub ddi_wb_lane_config: Option<DdiLaneConfiguration>,
    /// Extra travel distances per O-D movement A..N (Equation 23-50).
    pub extra_distances: [ExtraDistance; 14],
    /// Design speed of the diverted movements v_D, mi/h.
    pub extra_distance_speed_mph: f64,
    /// The interchange lane groups.
    pub lane_groups: Vec<LaneGroupInput>,

    // ── Computed results ───────────────────────────────────────────────────
    /// PHF-adjusted O-D demands (Step 1).
    #[serde(default)]
    pub od_adjusted: Option<OdDemands>,
    /// Lane group results (Steps 1–8).
    #[serde(default)]
    pub results: Vec<LaneGroupResult>,
    /// O-D results (Steps 8–9).
    #[serde(default)]
    pub od_results: Vec<OdResult>,
    /// Demand-weighted interchange ETT, s/veh (Equation 23-52).
    #[serde(default)]
    pub interchange_ett_s: Option<f64>,
    /// Interchange LOS (Exhibit 23-10 applied to the interchange ETT).
    #[serde(default)]
    pub interchange_los: Option<LevelOfService>,
}

impl Interchange {
    /// Create a new interchange with the Chapter 23 defaults
    /// (T = 0.25 h, s_0 = 1,900 pc/h/ln, non-CBD, L_h = 25 ft/veh,
    /// shared external right turns, no extra distances).
    pub fn new(form: InterchangeForm, cycle_length_s: f64, od: OdDemands) -> Self {
        Interchange {
            form,
            cycle_length_s,
            analysis_period_h: 0.25,
            base_saturation_flow: BASE_SATURATION_FLOW_METRO,
            area_type_cbd: false,
            peak_hour_factor: 1.0,
            distance_between_intersections_ft: 500.0,
            queue_spacing_ft: DEFAULT_QUEUE_SPACING_FT,
            od,
            eb_external_right_shared: true,
            wb_external_right_shared: true,
            ddi_eb_lane_config: None,
            ddi_wb_lane_config: None,
            extra_distances: [ExtraDistance::default(); 14],
            extra_distance_speed_mph: 35.0,
            lane_groups: Vec::new(),
            od_adjusted: None,
            results: Vec::new(),
            od_results: Vec::new(),
            interchange_ett_s: None,
            interchange_los: None,
        }
    }

    // ── Accessors ──────────────────────────────────────────────────────────

    pub fn get_form(&self) -> InterchangeForm {
        self.form
    }
    pub fn get_cycle_length(&self) -> f64 {
        self.cycle_length_s
    }
    pub fn set_cycle_length(&mut self, c: f64) {
        self.cycle_length_s = c;
    }
    pub fn get_peak_hour_factor(&self) -> f64 {
        self.peak_hour_factor
    }
    pub fn set_peak_hour_factor(&mut self, phf: f64) {
        self.peak_hour_factor = phf;
    }
    pub fn get_interchange_ett(&self) -> Option<f64> {
        self.interchange_ett_s
    }
    pub fn get_interchange_los(&self) -> Option<LevelOfService> {
        self.interchange_los
    }
    pub fn get_results(&self) -> &[LaneGroupResult] {
        &self.results
    }
    pub fn get_od_results(&self) -> &[OdResult] {
        &self.od_results
    }

    fn result_index(&self, m: InterchangeMovement) -> Option<usize> {
        self.results.iter().position(|r| r.movement == m)
    }

    // ═══════════════════════════════════════════════════════════════════════
    // Full pipeline
    // ═══════════════════════════════════════════════════════════════════════

    /// Run the full Exhibit 23-22 operational analysis (Steps 1–9) and
    /// store the results in `self`.
    pub fn analyze(&mut self) {
        self.step_1_od_and_movement_demands();
        // Step 2 (lane groups) is an input: `lane_groups`.
        self.step_3_saturation_flows();
        self.step_4_effective_green_adjustments();
        // Step 5 (closely spaced adjacent intersections) is provided as
        // free functions (Equation 23-40); adjacent intersections are
        // outside the facility boundary of this struct.
        self.step_6_and_7_capacity_vc_queue();
        self.step_8_control_delay();
        self.step_9_od_ett_and_los();
    }

    // ═══════════════════════════════════════════════════════════════════════
    // Step 1: O-D demands and lane group demand composition
    // ═══════════════════════════════════════════════════════════════════════

    /// Lane group demand composition (Exhibit 34-176 worksheet): which
    /// PHF-adjusted O-D demands travel in each lane group.
    fn lane_group_demand(&self, m: InterchangeMovement, od: &OdDemands) -> f64 {
        use InterchangeForm::*;
        use InterchangeMovement::*;
        let ddi = matches!(self.form, Ddi);
        let spui = matches!(self.form, Spui);
        match m {
            EbExtThrough => {
                if spui {
                    // SPUI arterial through (+ shared right).
                    od.i + if self.eb_external_right_shared { od.f } else { 0.0 }
                } else if ddi {
                    // DDI external crossover: through + left onto freeway
                    // (rights depart upstream of the crossover).
                    od.i + od.e + if self.eb_external_right_shared { od.f } else { 0.0 }
                } else {
                    od.i + od.e + if self.eb_external_right_shared { od.f } else { 0.0 }
                }
            }
            WbExtThrough => {
                if spui {
                    od.j + if self.wb_external_right_shared { od.g } else { 0.0 }
                } else {
                    od.j + od.h + if self.wb_external_right_shared { od.g } else { 0.0 }
                }
            }
            EbIntThrough => od.i + od.d, // internal link: arterial through + SB ramp left
            WbIntThrough => od.j + od.a,
            EbIntLeft => od.e + od.n,
            WbIntLeft => od.h + od.m,
            NbRampLeft => od.a + od.m,
            NbRampRight => od.b,
            SbRampLeft => od.d + od.n,
            SbRampRight => od.c,
        }
    }

    /// Step 1: adjust the O-D demands by the PHF and compose the lane
    /// group demand flow rates.
    pub fn step_1_od_and_movement_demands(&mut self) {
        let od = self.od.phf_adjusted(self.peak_hour_factor);
        self.od_adjusted = Some(od);
        self.results = self
            .lane_groups
            .iter()
            .map(|g| LaneGroupResult {
                movement: g.movement,
                flow_rate: g
                    .demand_override_veh_h
                    .unwrap_or_else(|| self.lane_group_demand(g.movement, &od)),
                lane_utilization: None,
                traffic_pressure: None,
                sat_flow: None,
                downstream_queue_lost_time_s: None,
                demand_starvation_lost_time_s: None,
                adjusted_lost_time_s: None,
                effective_green_s: None,
                capacity: None,
                vc_ratio: None,
                upstream_filtering: None,
                back_of_queue_veh: None,
                queue_storage_ratio: None,
                uniform_delay_s: None,
                incremental_delay_s: None,
                initial_queue_delay_s: None,
                control_delay_s: None,
            })
            .collect();
    }

    // ═══════════════════════════════════════════════════════════════════════
    // Step 3: adjusted saturation flow rates (Equation 23-14)
    // ═══════════════════════════════════════════════════════════════════════

    /// Lane utilization factor f_LU for a lane group (Equations 23-16
    /// through 23-18; 1.0 for non-external groups and single lanes).
    fn lane_utilization(&self, g: &LaneGroupInput, od: &OdDemands) -> f64 {
        use InterchangeMovement::*;
        if let Some(f) = g.lane_utilization_override {
            return f;
        }
        if g.lanes <= 1 {
            return 1.0;
        }
        let (is_ext, eastbound) = match g.movement {
            EbExtThrough => (true, true),
            WbExtThrough => (true, false),
            _ => (false, false),
        };
        if !is_ext || matches!(self.form, InterchangeForm::Spui) {
            // Non-external (ramp, internal, SPUI) approaches use the
            // Chapter 19 Exhibit 19-15 defaults (Chapter 23 Step 3 text:
            // "The lane utilization factors for all other interchange
            // approaches ... are estimated by using the procedures of
            // Chapter 19").
            let group = match g.movement {
                EbIntLeft | WbIntLeft | NbRampLeft | SbRampLeft => {
                    LaneUtilizationGroup::ExclusiveLeft
                }
                NbRampRight | SbRampRight => LaneUtilizationGroup::ExclusiveRight,
                _ => LaneUtilizationGroup::ExclusiveThrough,
            };
            return default_lane_utilization_factor(group, g.lanes);
        }
        if matches!(self.form, InterchangeForm::Ddi) {
            // Equation 23-18 with Exhibit 23-26. LTDR = left-turn demand
            // at the external crossover / total approach volume.
            let (v_l, v_t, cfg) = if eastbound {
                (od.e, od.i, self.ddi_eb_lane_config)
            } else {
                (od.h, od.j, self.ddi_wb_lane_config)
            };
            let total = v_l + v_t;
            let Some(cfg) = cfg else { return 1.0 };
            if total <= 0.0 {
                return 1.0;
            }
            let ltdr = v_l / total;
            return lane_utilization_factor_from_max(ddi_pct_v_lmax(cfg, ltdr), g.lanes);
        }
        // Equations 23-16 / 23-17 with Exhibit 23-24.
        let model = self.arterial_lane_utilization_model(eastbound);
        let (v_l, v_r, v_t) = if eastbound {
            let shared = self.eb_external_right_shared;
            (od.e, if shared { od.f } else { 0.0 }, od.i)
        } else {
            let shared = self.wb_external_right_shared;
            (od.h, if shared { od.g } else { 0.0 }, od.j)
        };
        let pct = pct_v_lmax_arterial(
            model,
            g.lanes,
            v_l,
            v_r,
            v_t,
            self.distance_between_intersections_ft
                .min(LANE_UTILIZATION_MAX_SPACING_FT),
        );
        lane_utilization_factor_from_max(pct, g.lanes)
    }

    /// Exhibit 23-24 model grouping for the subject external approach.
    fn arterial_lane_utilization_model(&self, eastbound: bool) -> LaneUtilizationModel {
        use InterchangeForm::*;
        match (self.form, eastbound) {
            (Diamond | Ddi, _) => LaneUtilizationModel::Diamond,
            (ParcloA2Q, _) => LaneUtilizationModel::ParcloA2Q,
            (ParcloB2Q | ParcloB4Q, _) => LaneUtilizationModel::ParcloB2QB4QAb4QWestbound,
            (ParcloAB4Q, false) => LaneUtilizationModel::ParcloB2QB4QAb4QWestbound,
            (ParcloA4Q, _) | (ParcloAB2Q, true) | (ParcloAB4Q, true) => {
                LaneUtilizationModel::ParcloA4QAb2QEbAb4QEastbound
            }
            (ParcloAB2Q, false) => LaneUtilizationModel::ParcloAB2QWestbound,
            (Spui, _) => LaneUtilizationModel::Diamond, // not used (f_LU = 1)
        }
    }

    /// Turning proportions in the lane group (P_LT, P_RT) from the O-D
    /// composition, for the shared-lane radius adjustments.
    fn turn_proportions(&self, m: InterchangeMovement, od: &OdDemands) -> (f64, f64) {
        use InterchangeMovement::*;
        let v = self.lane_group_demand(m, od).max(1e-9);
        match m {
            EbExtThrough => {
                let p_rt = if self.eb_external_right_shared { od.f / v } else { 0.0 };
                // DDI external crossover carries the (free at the internal
                // crossover) left-turn demand but turns within the
                // crossover geometry are treated as through (f_DDI covers
                // the crossover effect).
                (0.0, p_rt)
            }
            WbExtThrough => {
                let p_rt = if self.wb_external_right_shared { od.g / v } else { 0.0 };
                (0.0, p_rt)
            }
            EbIntLeft | WbIntLeft | NbRampLeft | SbRampLeft => (1.0, 0.0),
            NbRampRight | SbRampRight => (0.0, 1.0),
            EbIntThrough | WbIntThrough => (0.0, 0.0),
        }
    }

    /// Step 3: adjusted saturation flow rate for each lane group
    /// (Equation 23-14).
    pub fn step_3_saturation_flows(&mut self) {
        let od = self.od_adjusted.unwrap_or(self.od);
        let cbd = self.area_type_cbd;
        let s0 = self.base_saturation_flow;
        let c = self.cycle_length_s;
        let mut computed: Vec<(f64, f64, f64)> = Vec::with_capacity(self.lane_groups.len());
        for (g, r) in self.lane_groups.iter().zip(self.results.iter()) {
            let n = g.lanes.max(1);
            let f_w = lane_width_factor(g.lane_width_ft);
            let f_hvg = heavy_vehicle_grade_factor(g.pct_heavy_vehicles, g.grade_pct);
            let f_p = match g.parking_maneuvers_h {
                Some(nm) => parking_factor(n, nm),
                None => 1.0,
            };
            let f_bb = bus_blockage_factor(n, g.bus_stops_h);
            let f_a = area_type_factor(cbd);
            let f_lu = self.lane_utilization(g, &od);

            // Interchange adjustment No. 4 (Equations 23-19 through
            // 23-23): radius-based f_LT / f_RT. For shared lane groups
            // the movement radius factor is first flow-weighted across
            // the group ("the adjustment factor for turn radii is
            // estimated as the average (weighted on the basis of flows)
            // of the respective movements"), then entered into the
            // Equation 23-21 / 23-23 form — the convention that
            // reproduces the Chapter 34 Exhibit 34-7 published values
            // (e.g., EB EXT-TH&R: f_R = 0.991, f_RT = 0.999).
            let (p_lt, p_rt) = self.turn_proportions(g.movement, &od);
            let shared_chain = |p: f64, radius: Option<f64>| -> f64 {
                match radius {
                    Some(rad) if p > 0.0 => {
                        let f_r_movement = turn_radius_factor(rad);
                        let f_r_group = p * f_r_movement + (1.0 - p); // flow-weighted
                        left_turn_radius_adjustment(p, f_r_group)
                    }
                    _ => 1.0,
                }
            };
            let f_lt = shared_chain(p_lt, g.turn_radius_ft);
            let f_rt = shared_chain(
                p_rt,
                g.shared_right_turn_radius_ft.or(g.turn_radius_ft),
            );

            // Interchange adjustment No. 1 (Equation 23-15): traffic
            // pressure from the demand per cycle per lane, flow-weighted
            // between the left-turn and through/right forms for shared
            // groups.
            let v_cycle_lane = r.flow_rate * c / 3_600.0 / n as f64;
            let f_v_left = traffic_pressure_factor(v_cycle_lane, true);
            let f_v_thru = traffic_pressure_factor(v_cycle_lane, false);
            let f_v = p_lt * f_v_left + (1.0 - p_lt) * f_v_thru;

            // Interchange adjustment No. 3: f_DDI for the DDI crossover
            // through movements.
            let f_ddi = if matches!(self.form, InterchangeForm::Ddi)
                && matches!(
                    g.movement,
                    InterchangeMovement::EbExtThrough
                        | InterchangeMovement::WbExtThrough
                        | InterchangeMovement::EbIntThrough
                        | InterchangeMovement::WbIntThrough
                ) {
                F_DDI
            } else {
                1.0
            };

            // Equation 23-14 (f_Lpb / f_Rpb = 1.0: pedestrian-bicycle
            // adjustments from Chapter 19 apply when those volumes are
            // supplied; the Chapter 34 interchange examples carry none).
            let s = s0 * n as f64 * f_w * f_hvg * f_p * f_bb * f_a * f_lt * f_rt * f_v * f_lu
                * f_ddi;
            computed.push((f_lu, f_v, s));
        }
        for (r, (f_lu, f_v, s)) in self.results.iter_mut().zip(computed) {
            r.lane_utilization = Some(f_lu);
            r.traffic_pressure = Some(f_v);
            r.sat_flow = Some(s);
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    // Step 4: effective green adjustments (Equations 23-24 through 23-39)
    // ═══════════════════════════════════════════════════════════════════════

    /// Arterial feed flow entering the internal link (external through
    /// demand minus the external right turns), veh/h.
    fn internal_link_arterial_feed(&self, eastbound: bool, od: &OdDemands) -> f64 {
        if eastbound {
            od.i + od.e
        } else {
            od.j + od.h
        }
    }

    /// Step 4: additional lost times (downstream internal queue, DDI
    /// overlap phasing, demand starvation) and adjusted effective greens.
    pub fn step_4_effective_green_adjustments(&mut self) {
        use InterchangeMovement::*;
        let od = self.od_adjusted.unwrap_or(self.od);
        let c = self.cycle_length_s;
        let d_ft = self.distance_between_intersections_ft;
        let lh = self.queue_spacing_ft;
        let two_intersections = !matches!(self.form, InterchangeForm::Spui);
        let is_ddi = matches!(self.form, InterchangeForm::Ddi);

        // Snapshot of green intervals and lanes for the movements
        // referenced by the lost-time equations (avoids borrowing `self`
        // while the results are updated).
        let snapshot: Vec<(InterchangeMovement, Vec<GreenInterval>, f64, u32)> = self
            .lane_groups
            .iter()
            .map(|g| (g.movement, g.greens.clone(), g.total_green_s(), g.lanes))
            .collect();
        let greens = |m: InterchangeMovement| -> Vec<GreenInterval> {
            snapshot
                .iter()
                .find(|(mv, ..)| *mv == m)
                .map(|(_, g, ..)| g.clone())
                .unwrap_or_default()
        };
        let total_green = |m: InterchangeMovement| -> f64 {
            snapshot
                .iter()
                .find(|(mv, ..)| *mv == m)
                .map(|(_, _, tg, _)| *tg)
                .unwrap_or(0.0)
        };
        let lanes = |m: InterchangeMovement| -> u32 {
            snapshot
                .iter()
                .find(|(mv, ..)| *mv == m)
                .map(|(_, _, _, n)| *n)
                .unwrap_or(1)
        };

        // Per-direction relationships (diamond / parclo / DDI):
        // * EB internal link (downstream = EbIntThrough at Int II) is fed
        //   by EbExtThrough (Int I) and SbRampLeft (Int I); starvation
        //   window is the WbIntLeft green (Int I).
        // * WB internal link mirrors with NbRampLeft and EbIntLeft.
        struct LinkSpec {
            downstream: InterchangeMovement,
            arterial: InterchangeMovement,
            ramp: InterchangeMovement,
            blocking_left: InterchangeMovement,
            eastbound: bool,
        }
        let links = [
            LinkSpec {
                downstream: EbIntThrough,
                arterial: EbExtThrough,
                ramp: SbRampLeft,
                blocking_left: WbIntLeft,
                eastbound: true,
            },
            LinkSpec {
                downstream: WbIntThrough,
                arterial: WbExtThrough,
                ramp: NbRampLeft,
                blocking_left: EbIntLeft,
                eastbound: false,
            },
        ];

        // Pass 1: downstream-queue lost times for the external arterial
        // and ramp-left groups (Equations 23-29 through 23-34). Directly
        // supplied values (e.g., the DDI shock-wave estimate of the
        // Chapter 23 DDI lost-time procedure) take precedence.
        let mut lost_d: Vec<f64> = self
            .lane_groups
            .iter()
            .map(|g| g.downstream_queue_lost_time_s.unwrap_or(0.0))
            .collect();
        if two_intersections {
            for link in &links {
                let gd = total_green(link.downstream);
                let down_greens = greens(link.downstream);
                let art_feed = self.internal_link_arterial_feed(link.eastbound, &od);
                let ramp_feed = self
                    .results
                    .iter()
                    .find(|r| r.movement == link.ramp)
                    .map(|r| r.flow_rate)
                    .unwrap_or(0.0);

                for (subject, feeding_flow, feeding_lanes, feeding_green) in [
                    // Equation 23-33: queue at the beginning of the
                    // arterial phase is built by the ramp feed.
                    (
                        link.arterial,
                        ramp_feed,
                        lanes(link.ramp),
                        total_green(link.ramp),
                    ),
                    // Equation 23-34: queue at the beginning of the ramp
                    // phase is built by the arterial feed.
                    (
                        link.ramp,
                        art_feed,
                        lanes(link.arterial),
                        total_green(link.arterial),
                    ),
                ] {
                    let Some(idx) = self.lane_groups.iter().position(|g| g.movement == subject)
                    else {
                        continue;
                    };
                    let ginp = &self.lane_groups[idx];
                    if ginp.downstream_queue_lost_time_s.is_some() || is_ddi {
                        // Direct input (already applied) — or a DDI,
                        // which uses the shock-wave estimate supplied as
                        // an input (Chapter 23 DDI lost-time procedure).
                        continue;
                    }
                    let cg = common_green_time(&ginp.greens, &down_greens, c);
                    let mut q = downstream_queue_length_ft(
                        feeding_flow,
                        feeding_lanes,
                        feeding_green,
                        gd,
                        cg,
                        c,
                        lh,
                    );
                    let mut dq = d_ft - q;
                    if q > d_ft {
                        q = d_ft;
                        dq = 0.0;
                    }
                    let _ = q;
                    lost_d[idx] =
                        downstream_queue_lost_time(ginp.total_green_s(), dq.max(0.0), cg, c);
                }
            }
        }

        // Adjusted lost time and effective green for the external / ramp
        // groups (Equations 23-24 / 23-25 / 23-27); internal groups are
        // handled in pass 2.
        for (idx, g) in self.lane_groups.iter().enumerate() {
            let internal = matches!(g.movement, EbIntThrough | WbIntThrough);
            if internal {
                continue;
            }
            let tl = adjusted_lost_time(
                g.start_up_lost_time_s,
                lost_d[idx] + g.overlap_lost_time_s,
                g.yellow_all_red_s,
                g.extension_of_green_s,
            );
            let r = &mut self.results[idx];
            r.downstream_queue_lost_time_s = Some(lost_d[idx]);
            r.adjusted_lost_time_s = Some(tl);
            r.effective_green_s = Some((g.total_green_s() + g.yellow_all_red_s - tl).max(0.0));
        }

        // Pass 2: demand starvation for the internal through groups
        // (Equations 23-38 / 23-39; zero for DDIs per the Chapter 23
        // text).
        for link in &links {
            let Some(idx) = self
                .lane_groups
                .iter()
                .position(|g| g.movement == link.downstream)
            else {
                continue;
            };
            let g = &self.lane_groups[idx];
            let mut lds = 0.0;
            if two_intersections && !is_ddi {
                let cg_ds = common_green_time(&g.greens, &greens(link.blocking_left), c);
                if cg_ds > 0.0 {
                    let s_int = self.results[idx].sat_flow.unwrap_or(0.0)
                        / g.lanes.max(1) as f64;
                    if s_int > 0.0 {
                        let h_i = 3_600.0 / s_int;
                        let art_idx = self.lane_groups.iter().position(|x| x.movement == link.arterial);
                        let ramp_idx = self.lane_groups.iter().position(|x| x.movement == link.ramp);
                        let art_feed = self.internal_link_arterial_feed(link.eastbound, &od);
                        let ramp_feed = ramp_idx
                            .map(|i| self.results[i].flow_rate)
                            .unwrap_or(0.0);
                        let cg_ud = art_idx
                            .map(|i| common_green_time(&self.lane_groups[i].greens, &g.greens, c))
                            .unwrap_or(0.0);
                        let cg_rd = ramp_idx
                            .map(|i| common_green_time(&self.lane_groups[i].greens, &g.greens, c))
                            .unwrap_or(0.0);
                        // t_L per phase (Equations 23-24 / 23-25) of the
                        // feeding approaches; both use the base phase
                        // lost time in the Chapter 34 examples.
                        let t_l = adjusted_lost_time(
                            g.start_up_lost_time_s,
                            0.0,
                            g.yellow_all_red_s,
                            g.extension_of_green_s,
                        );
                        // Equation 23-39.
                        let q_init = demand_starvation_initial_queue(
                            ramp_feed,
                            ramp_idx.map(|i| self.lane_groups[i].lanes).unwrap_or(1),
                            art_feed,
                            art_idx.map(|i| self.lane_groups[i].lanes).unwrap_or(1),
                            c,
                            cg_rd,
                            cg_ud,
                            t_l,
                            h_i,
                        )
                        .max(0.0);
                        lds = demand_starvation_lost_time(cg_ds, q_init, h_i);
                    }
                }
            }
            let tl2 = adjusted_lost_time(
                g.start_up_lost_time_s,
                lds,
                g.yellow_all_red_s,
                g.extension_of_green_s,
            );
            let r = &mut self.results[idx];
            r.demand_starvation_lost_time_s = Some(lds);
            r.adjusted_lost_time_s = Some(tl2);
            r.effective_green_s = Some((g.total_green_s() + g.yellow_all_red_s - tl2).max(0.0));
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    // Steps 6 and 7: YIELD-turn performance, capacity, v/c, queue storage
    // ═══════════════════════════════════════════════════════════════════════

    /// Steps 6 and 7: capacity and v/c per lane group (Equation 23-48;
    /// Equations 23-42 through 23-47 for YIELD-controlled turns) and the
    /// Chapter 31 Section 4 queue storage ratio.
    pub fn step_6_and_7_capacity_vc_queue(&mut self) {
        let c = self.cycle_length_s;
        let t_h = self.analysis_period_h;
        for (g, r) in self.lane_groups.iter().zip(self.results.iter_mut()) {
            match &g.control {
                LaneGroupControl::FreeFlow => {
                    r.capacity = None;
                    r.vc_ratio = Some(0.0);
                }
                LaneGroupControl::YieldControlled(y) => {
                    // Step 6 (Equations 23-53 through 23-56, 23-42,
                    // 23-44, 23-47).
                    let red = (c - y.conflicting_green_s).max(0.0);
                    let t_cq = match y.proportion_on_green {
                        Some(p) => yield_time_to_clear_queue_coordinated(
                            c,
                            p,
                            y.conflicting_flow_per_lane,
                            y.conflicting_sat_flow_per_lane,
                            y.conflicting_green_s,
                        ),
                        None => yield_time_to_clear_queue_random(
                            red,
                            y.conflicting_flow_per_lane,
                            y.conflicting_sat_flow_per_lane,
                        ),
                    };
                    let t_clear =
                        yield_clearance_time(y.clearance_distance_ft, y.clearance_speed_mph);
                    let c_ga = yield_gap_acceptance_capacity(
                        y.critical_headway_s,
                        y.follow_up_headway_s,
                        y.conflicting_flow_veh_h,
                    );
                    let c_ncf = yield_no_conflict_capacity(y.follow_up_headway_s);
                    let cap = yield_turn_capacity(
                        c,
                        y.conflicting_green_s,
                        t_cq,
                        t_clear,
                        c_ga,
                        c_ncf,
                    );
                    r.capacity = Some(cap);
                    r.vc_ratio = Some(if cap > 0.0 { r.flow_rate / cap } else { f64::INFINITY });
                }
                LaneGroupControl::Signalized => {
                    // Equation 23-48 with g replaced by g' / g''.
                    let s = r.sat_flow.unwrap_or(0.0);
                    let g_eff = r.effective_green_s.unwrap_or(0.0);
                    let cap = s * g_eff / c;
                    r.capacity = Some(cap);
                    r.vc_ratio = Some(if cap > 0.0 { r.flow_rate / cap } else { f64::INFINITY });
                }
            }
        }
        // Upstream filtering I for the internal groups (Equation 19-6
        // from the upstream external arterial v/c; Chapter 34 Exhibit
        // 34-12 convention), then d2-dependent queue terms in Step 8.
        self.assign_upstream_filtering();
        // Queue storage ratio (Chapter 31 Section 4) is finalized in
        // Step 8 once d2 is available (Q2 depends on d2).
        let _ = t_h;
    }

    fn assign_upstream_filtering(&mut self) {
        use InterchangeMovement::*;
        let x_eb = self
            .result_index(EbExtThrough)
            .and_then(|i| self.results[i].vc_ratio);
        let x_wb = self
            .result_index(WbExtThrough)
            .and_then(|i| self.results[i].vc_ratio);
        for (g, r) in self.lane_groups.iter().zip(self.results.iter_mut()) {
            let i_factor = if let Some(over) = g.upstream_filtering_override {
                over
            } else {
                match g.movement {
                    EbIntThrough | EbIntLeft => {
                        x_eb.map(upstream_filtering_factor).unwrap_or(1.0)
                    }
                    WbIntThrough | WbIntLeft => {
                        x_wb.map(upstream_filtering_factor).unwrap_or(1.0)
                    }
                    _ => 1.0,
                }
            };
            r.upstream_filtering = Some(i_factor);
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    // Step 8: control delay per lane group (Chapter 19 equations)
    // ═══════════════════════════════════════════════════════════════════════

    /// Step 8: uniform, incremental, and initial-queue delay per lane
    /// group, plus the back-of-queue / queue storage ratio.
    pub fn step_8_control_delay(&mut self) {
        let c = self.cycle_length_s;
        let t_h = self.analysis_period_h;
        for (g, r) in self.lane_groups.iter().zip(self.results.iter_mut()) {
            match &g.control {
                LaneGroupControl::FreeFlow => {
                    // Chapter 23 Step 6: free-flowing movements carry
                    // zero control delay.
                    r.control_delay_s = Some(0.0);
                }
                LaneGroupControl::YieldControlled(_) => {
                    // Chapter 23 Step 8 / Chapter 34 Example Problem 6:
                    // "control delay for the movement is then estimated
                    // by using the control delay procedure for
                    // roundabouts given in Equation 22-17".
                    // // VERIFY-HCM: Exhibit 34-70 publishes larger
                    // // delays (e.g., M7: 34.7 s vs. 9.1 s from
                    // // Equation 22-17 with c_YCT = 795 veh/h); the
                    // // published values are not reproducible from the
                    // // printed equations.
                    let cap = r.capacity.unwrap_or(0.0);
                    r.control_delay_s = Some(if cap > 0.0 {
                        control_delay_roundabout(r.flow_rate, cap, t_h)
                    } else {
                        f64::INFINITY
                    });
                }
                LaneGroupControl::Signalized => {
                    let x = r.vc_ratio.unwrap_or(0.0);
                    let g_eff = r.effective_green_s.unwrap_or(0.0);
                    let g_over_c = (g_eff / c).min(1.0);
                    let rp = platoon_ratio_for_arrival_type(g.arrival_type).unwrap_or(1.0);
                    let p = (rp * g_over_c).min(1.0);
                    let pf = if g_over_c < 1.0 {
                        progression_factor(p, g_over_c, x)
                    } else {
                        1.0
                    };
                    let d1 = uniform_delay(c, g_eff, x, pf);
                    // d2 uses the lane group capacity: Equation 19-26 defines
                    // c_A as "the average capacity (veh/h) ... equal to the
                    // capacity c computed in Step 7", and the Step 7 c is the
                    // lane group capacity. cap_ln below stays per-lane because
                    // the Chapter 31 back-of-queue Q1 is a per-lane queue.
                    let cap = r.capacity.unwrap_or(0.0);
                    let cap_ln = cap / g.lanes.max(1) as f64;
                    let i_f = r.upstream_filtering.unwrap_or(1.0);
                    let d2 = if cap > 0.0 {
                        incremental_delay_signalized(t_h, x, cap, K_PRETIMED, i_f)
                    } else {
                        0.0
                    };
                    let d3 = initial_queue_delay(
                        g.initial_queue_veh,
                        r.flow_rate,
                        r.capacity.unwrap_or(1e-9),
                        t_h,
                    );
                    r.uniform_delay_s = Some(d1);
                    r.incremental_delay_s = Some(d2);
                    r.initial_queue_delay_s = Some(d3);
                    r.control_delay_s = Some(d1 + d2 + d3);

                    // Back of queue and queue storage ratio (Chapter 31
                    // Section 4 via the Chapter 19 building blocks).
                    let n = g.lanes.max(1);
                    let v_ln = r.flow_rate / n as f64;
                    let s_ln = r.sat_flow.unwrap_or(0.0) / n as f64;
                    let d_a = accel_decel_delay(g.speed_limit_mph);
                    let q1 =
                        first_term_back_of_queue(v_ln, cap_ln, s_ln, p, g_eff, c, d_a);
                    let q2 = second_term_back_of_queue(r.capacity.unwrap_or(0.0), n, d2);
                    let q = q1 + q2;
                    r.back_of_queue_veh = Some(q);
                    if let Some(la) = g.storage_ft {
                        let lh = average_vehicle_spacing(g.pct_heavy_vehicles);
                        r.queue_storage_ratio = Some(queue_storage_ratio_eq(lh, q, la));
                    }
                }
            }
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    // Step 9: O-D control delay, ETT, and LOS (Equations 23-49 to 23-52)
    // ═══════════════════════════════════════════════════════════════════════

    /// Lane groups traversed by each O-D movement (diamond / parclo O-D
    /// letters; validated against Chapter 34 Exhibits 34-16 and 34-65).
    /// O-D movements with no represented lane groups (e.g., free-flow
    /// DDI right turns, K / L ramp throughs) return an empty path.
    fn od_path(&self, m: OdMovement) -> Vec<InterchangeMovement> {
        use InterchangeMovement::*;
        use OdMovement::*;
        let spui = matches!(self.form, InterchangeForm::Spui);
        let raw: Vec<InterchangeMovement> = match m {
            A => {
                if spui {
                    vec![NbRampLeft]
                } else {
                    vec![NbRampLeft, WbIntThrough]
                }
            }
            B => vec![NbRampRight],
            C => vec![SbRampRight],
            D => {
                if spui {
                    vec![SbRampLeft]
                } else {
                    vec![SbRampLeft, EbIntThrough]
                }
            }
            E => {
                if spui {
                    vec![EbIntLeft]
                } else if matches!(self.form, InterchangeForm::Ddi) {
                    // The DDI left onto the freeway departs at the
                    // internal crossover after traversing the external
                    // crossover (free-flowing at the internal crossover).
                    vec![EbExtThrough]
                } else {
                    vec![EbExtThrough, EbIntLeft]
                }
            }
            F => {
                if self.eb_external_right_shared {
                    vec![EbExtThrough]
                } else {
                    vec![] // exclusive / free-flow bypass
                }
            }
            G => {
                if self.wb_external_right_shared {
                    vec![WbExtThrough]
                } else {
                    vec![]
                }
            }
            H => {
                if spui {
                    vec![WbIntLeft]
                } else if matches!(self.form, InterchangeForm::Ddi) {
                    vec![WbExtThrough]
                } else {
                    vec![WbExtThrough, WbIntLeft]
                }
            }
            I => {
                if spui {
                    vec![EbExtThrough]
                } else {
                    vec![EbExtThrough, EbIntThrough]
                }
            }
            J => {
                if spui {
                    vec![WbExtThrough]
                } else {
                    vec![WbExtThrough, WbIntThrough]
                }
            }
            // Freeway U-turns traverse the off-ramp left and the
            // opposite internal left (diamond geometry).
            M if !spui => vec![NbRampLeft, WbIntLeft],
            N if !spui => vec![SbRampLeft, EbIntLeft],
            _ => vec![],
        };
        raw.into_iter()
            .filter(|mv| self.result_index(*mv).is_some())
            .collect()
    }

    /// Step 9 (with the Step 8 O-D aggregation): O-D delays, ETT, LOS
    /// (Exhibit 23-10), and the interchange ETT / LOS (Equation 23-52).
    pub fn step_9_od_ett_and_los(&mut self) {
        let od = self.od_adjusted.unwrap_or(self.od);
        let mut results = Vec::new();
        let mut num = 0.0;
        let mut den = 0.0;
        for (k, m) in OdMovement::ALL.iter().enumerate() {
            let demand = od.get(*m);
            if demand <= 0.0 {
                continue;
            }
            let path = self.od_path(*m);
            // Free-flow right-turn O-Ds (F / G with an exclusive bypass)
            // carry zero control delay; other O-Ds without represented
            // lane groups (e.g., K / L ramp throughs) are skipped.
            let free_flow_right =
                matches!(m, OdMovement::F | OdMovement::G) && path.is_empty();
            if path.is_empty() && !free_flow_right {
                continue;
            }
            let mut delay = 0.0;
            let mut vc_gt_1 = false;
            let mut rq_gt_1 = false;
            for mv in &path {
                let r = &self.results[self.result_index(*mv).unwrap()];
                delay += r.control_delay_s.unwrap_or(0.0);
                if r.vc_ratio.unwrap_or(0.0) > 1.0 {
                    vc_gt_1 = true;
                }
                if r.queue_storage_ratio.unwrap_or(0.0) > 1.0 {
                    rq_gt_1 = true;
                }
            }
            let xd = self.extra_distances[k];
            let edtt = extra_distance_travel_time(
                xd.distance_ft,
                self.extra_distance_speed_mph,
                xd.accel_decel_s,
            );
            let ett = delay + edtt;
            let los = los_signalized_interchange_od(ett, vc_gt_1, rq_gt_1);
            num += ett * demand;
            den += demand;
            results.push(OdResult {
                movement: *m,
                demand,
                control_delay_s: delay,
                edtt_s: edtt,
                ett_s: ett,
                vc_exceeds_one: vc_gt_1,
                rq_exceeds_one: rq_gt_1,
                los,
            });
        }
        self.od_results = results;
        if den > 0.0 {
            let ett_i = num / den;
            self.interchange_ett_s = Some(ett_i);
            // Equation 23-52 grades the interchange from the demand-weighted
            // ETT only. The per-O-D v/c and R_Q flags are deliberately not
            // propagated here; see the Step 9 note in the module header.
            self.interchange_los = Some(los_signalized_interchange_od(ett_i, false, false));
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Step 5 free function (closely spaced adjacent intersections)
// ═══════════════════════════════════════════════════════════════════════════════

/// HCM Equation 23-40: additional lost time on upstream approach i of a
/// closely spaced adjacent intersection
/// `L_D-Ui = G_Ui − 0.106 DQ_i − 5.39 CG_UiD / C >= 0`.
///
/// The full adjacent-intersection interaction (lane utilization minus
/// 0.05 per the Chapter 23 text, plus the Chapter 19 evaluation of the
/// adjacent intersection itself) is applied by the analyst; when both a
/// downstream queue and demand starvation act on the same approach the
/// chapter directs the use of alternative tools.
pub fn adjacent_intersection_lost_time(
    green_s: f64,
    distance_to_queue_ft: f64,
    common_green_s: f64,
    cycle_s: f64,
) -> f64 {
    downstream_queue_lost_time(green_s, distance_to_queue_ft, common_green_s, cycle_s)
}
