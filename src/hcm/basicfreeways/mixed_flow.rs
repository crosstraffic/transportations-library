//! HCM Chapter 26: the mixed-flow model for a single grade.
//!
//! Chapter 12's passenger-car-equivalent method converts trucks into passenger cars and then
//! analyses one homogeneous stream. That breaks down on a sustained steep grade, where the
//! trucks are not slowed versions of cars but a separate population settling towards a crawl
//! speed the cars never approach. The mixed-flow model instead tracks automobiles, single-unit
//! trucks and tractor-trailers as three streams with their own travel time rates and combines
//! them at the end.
//!
//! Equations implemented:
//! - Equation 26-1: mixed-flow capacity adjustment factor
//! - Equation 26-2: CAF for truck percentage
//! - Equation 26-3: CAF for grade effect
//! - Equation 26-4: rho for the grade CAF
//! - Equation 26-5: mixed-flow capacity
//! - Equation 26-6/26-7: truck travel time rates including traffic interaction
//! - Equation 26-8: automobile travel time rate
//! - Equation 26-9: traffic interaction term
//! - Equation 26-10: auto-only speed
//! - Equation 26-11/26-12: kinematic travel time rate, within and beyond 10,000 ft
//! - Equation 26-13/26-14: mixed-flow free-flow speed
//! - Equation 26-15: mixed-flow speed adjustment factor
//! - Equation 26-16: mixed-flow breakpoint
//! - Equation 26-19: calibration speeds at capacity and 90% of capacity
//! - Equation 26-20: the exponent phi
//! - Equation 26-21/26-22: mixed-flow speed and density
//!
//! Chapter 25 states that its Equations 25-53 to 25-70 are "identical to those presented in
//! Chapter 26, although they have different equation numbers". The primitives in this module
//! are therefore shared with `composite_grade`, which adds per-segment chaining on top.

use serde::{Deserialize, Serialize};

use crate::hcm::common::truck_curves::{
    self, CurveFamily, SpotBranch, TruckClass,
};

/// Density at capacity for a basic freeway segment (pc/mi/ln), Exhibit 12-6.
pub(crate) const D_C: f64 = 45.0;

/// Auto-only capacity from Exhibit 12-6 (pc/h/ln).
pub(crate) fn capacity_auto_only(ffs: f64) -> f64 {
    2200.0 + 10.0 * (ffs - 50.0)
}

/// Auto-only breakpoint from Exhibit 12-6 (veh/h/ln).
pub(crate) fn breakpoint_auto_only(ffs: f64, caf: f64) -> f64 {
    (1000.0 + 40.0 * (75.0 - ffs)) * caf * caf
}

/// Equation 26-4 / 25-56.
pub(crate) fn rho_g_mix(p_t: f64) -> f64 {
    if p_t < 0.01 {
        8.0 * p_t
    } else {
        0.126 - 0.03 * p_t
    }
}

/// Equation 26-3 / 25-55, the grade component of the capacity adjustment.
///
/// The print PDFs of both chapters truncate this equation at a horizontal scrollbar, mid-way
/// through the third factor. The form below is transcribed from the EPUB's MathML, which
/// carries it complete, and it reproduces all four `CAF_g,mix` values the worked examples
/// print (0.131, 0.067, 0.042, 0.122).
pub(crate) fn caf_grade(p_t: f64, grade_pct: f64, length_mi: f64) -> f64 {
    let g = grade_pct / 100.0;
    rho_g_mix(p_t)
        * (0.69 * ((12.9 * g).exp() - 1.0)).max(0.0)
        * (1.72 * (1.0 - 1.71 * (-3.16 * length_mi).exp())).max(0.0)
}

/// Equation 26-2 / 25-54.
pub(crate) fn caf_trucks(p_t: f64) -> f64 {
    0.53 * p_t.powf(0.72)
}

/// Equation 26-10 / 25-63, the auto-only speed the traffic interaction term is built from.
pub(crate) fn speed_auto_only(ffs: f64, v_mix: f64, caf_mix: f64, cap: f64, bp_ao: f64) -> f64 {
    let x = v_mix / caf_mix;
    if x <= bp_ao {
        ffs
    } else {
        ffs - (ffs - cap / D_C) * (x - bp_ao).powi(2) / (cap - bp_ao).powi(2)
    }
}

/// Equation 26-9 / 25-62.
pub(crate) fn traffic_interaction(ffs: f64, s_ao: f64, caf_mix: f64) -> f64 {
    (3600.0 / s_ao - 3600.0 / ffs) * (1.0 + 3.0 * (1.0 / caf_mix - 1.0))
}

/// Which of the three automobile travel time rate equations to evaluate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AutoRateForm {
    /// Equation 26-8 / 25-65: space-based rate where the trucks are decelerating.
    SpaceDecelerating,
    /// Equation 25-66: space-based rate where the trucks are accelerating.
    SpaceAccelerating,
    /// Equation 25-64: spot rate at the end of a segment.
    Spot,
}

/// Equations 26-8, 25-64, 25-65 and 25-66. They share a shape and differ only in eight
/// coefficients.
///
/// `tau_kin` is the pair of KINEMATIC truck rates, `[SUT, TT]` in s/mi, before the traffic
/// interaction term is added. Feeding the adjusted rates in instead still produces plausible numbers, so this
/// is worth stating: Example Problem 5 prints an automobile rate at capacity of 92.9 s/mi,
/// and the adjusted rates give 111.0.
pub(crate) fn auto_rate(
    form: AutoRateForm,
    ffs: f64,
    v_mix: f64,
    d_tau_ti: f64,
    p_sut: f64,
    p_tt: f64,
    tau_kin: [f64; 2],
) -> f64 {
    let (a1, b1, c1, e1, a2, b2, c2, e2) = match form {
        AutoRateForm::SpaceDecelerating => (100.42, 0.46, 0.68, 2.76, 110.64, 1.36, 0.62, 1.81),
        AutoRateForm::SpaceAccelerating => (54.72, 1.16, 0.28, 1.73, 69.72, 1.32, 0.61, 1.33),
        AutoRateForm::Spot => (64.50, 0.77, 0.34, 1.53, 79.50, 0.81, 0.56, 1.32),
    };
    let floor = 3600.0 / (ffs * 100.0);
    let v = v_mix / 1000.0;
    3600.0 / ffs
        + d_tau_ti
        + a1 * v.powf(b1) * p_sut.powf(c1) * (tau_kin[0] / 100.0 - floor).max(0.0).powf(e1)
        + a2 * v.powf(b2) * p_tt.powf(c2) * (tau_kin[1] / 100.0 - floor).max(0.0).powf(e2)
}

/// Equation 26-16, the mixed-flow breakpoint.
///
/// VERIFY-HCM: the manual prints `max[0, e^(30g) + 1]`, and the `+ 1` is almost certainly a
/// typo for `- 1`. With `+ 1` the inner max is vacuous, because the argument can never be
/// negative, and on level ground the term is 2 rather than 0, which reduces the breakpoint on
/// terrain that should not be affected by grade at all. The printed form is kept because it is
/// load-bearing for reproduction: Example Problem 5 propagates it to `BP_mix = 0` and the text
/// then rationalises that result ("this result implies that speeds drop immediately at zero
/// flow"), and `BP_mix` feeds both Equation 26-20 and Equation 26-21. Correcting the sign here
/// would silently move the published speed and density. This is not listed in the December
/// 2022 HCM 7 corrections.
pub(crate) fn breakpoint_mixed(bp_ao: f64, p_t: f64, grade_pct: f64, length_mi: f64) -> f64 {
    let g = grade_pct / 100.0;
    let d_ft = length_mi * 5280.0;
    (bp_ao * (1.0 - 0.4 * p_t.powf(0.1) * ((30.0 * g).exp() + 1.0).max(0.0) * d_ft.powf(0.01)))
        .max(0.0)
}

/// Inputs to a single-grade mixed-flow analysis (HCM Chapter 26).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MixedFlowSegment {
    /// Free-flow speed of the segment (mi/h).
    pub ffs: f64,
    /// Length of the grade (mi).
    pub length: f64,
    /// Grade (percent; 5.0 means a 5% upgrade).
    pub grade: f64,
    /// Mixed demand flow rate (veh/h/ln).
    pub v_mix: f64,
    /// Proportion of single-unit trucks in the stream (decimal).
    pub p_sut: f64,
    /// Proportion of tractor-trailers in the stream (decimal).
    pub p_tt: f64,
    /// Auto-only capacity adjustment factor (decimal). 1.0 unless weather, incident,
    /// work zone or driver population adjustments apply.
    #[serde(default = "unit_caf")]
    pub caf_ao: f64,
}

fn unit_caf() -> f64 {
    1.0
}

/// Everything the Chapter 26 chain produces, in the order the method computes it.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MixedFlowResult {
    /// Equation 26-2 (decimal).
    pub caf_t_mix: f64,
    /// Equation 26-4 (decimal).
    pub rho_g_mix: f64,
    /// Equation 26-3 (decimal).
    pub caf_g_mix: f64,
    /// Equation 26-1 (decimal).
    pub caf_mix: f64,
    /// Exhibit 12-6 auto-only capacity (pc/h/ln).
    pub capacity_ao: f64,
    /// Equation 26-5 (veh/h/ln).
    pub capacity_mix: f64,
    /// Kinematic SUT travel time rate (s/mi).
    pub tau_sut_kin: f64,
    /// Kinematic TT travel time rate (s/mi).
    pub tau_tt_kin: f64,
    /// Automobile rate under free-flow conditions (s/mi).
    pub tau_a_ffs: f64,
    /// Equation 26-13 evaluated at free flow (s/mi).
    pub tau_mix_ffs: f64,
    /// Equation 26-14 (mi/h).
    pub ffs_mix: f64,
    /// Equation 26-15 (decimal).
    pub saf_mix: f64,
    /// Exhibit 12-6 auto-only breakpoint (veh/h/ln).
    pub bp_ao: f64,
    /// Equation 26-16 (veh/h/ln).
    pub bp_mix: f64,
    /// Equation 26-19 at capacity (mi/h).
    pub s_calib_cap: f64,
    /// Equation 26-19 at 90% of capacity (mi/h).
    pub s_calib_90cap: f64,
    /// Equation 26-20 (decimal).
    pub phi_mix: f64,
    /// Equation 26-21 (mi/h). `None` when demand exceeds mixed-flow capacity.
    pub s_mix: Option<f64>,
    /// Equation 26-22 (veh/mi/ln). `None` when demand exceeds mixed-flow capacity.
    pub d_mix: Option<f64>,
    /// True when `v_mix` exceeds `capacity_mix`, which Chapter 26 Step 2 calls LOS F and
    /// stops on.
    pub oversaturated: bool,
}

impl MixedFlowSegment {
    /// Run the Chapter 26 single-grade mixed-flow chain, Steps 2 through 8.
    pub fn analyze(&self) -> Result<MixedFlowResult, String> {
        self.validate()?;
        let p_t = self.p_sut + self.p_tt;
        let p_a = 1.0 - p_t;

        // Step 2 -- capacity.
        let caf_t_mix = caf_trucks(p_t);
        let rho = rho_g_mix(p_t);
        let caf_g = caf_grade(p_t, self.grade, self.length);
        let caf_mix = self.caf_ao - caf_t_mix - caf_g;
        if caf_mix <= 0.0 {
            return Err(format!(
                "mixed-flow CAF came out at {caf_mix:.4}, which is not a usable capacity \
                 adjustment; check the truck proportions ({p_t:.3}) and grade ({}%)",
                self.grade
            ));
        }
        let capacity_ao = capacity_auto_only(self.ffs);
        let capacity_mix = capacity_ao * caf_mix;
        let oversaturated = self.v_mix > capacity_mix;

        // Step 3 -- mixed-flow free-flow speed. The curves are read on the Chapter 26 family,
        // which is indexed by FFS rather than by the truck's initial speed.
        let family = CurveFamily::Ch26Ffs(self.ffs.round() as u32);
        let tau_sut_kin =
            truck_curves::travel_time_rate(TruckClass::Sut, family, self.grade, self.length, self.ffs)?;
        let tau_tt_kin =
            truck_curves::travel_time_rate(TruckClass::Tt, family, self.grade, self.length, self.ffs)?;

        // Chapter 26 is explicit that the free-flow evaluation sets the traffic interaction
        // term to zero and the flow rate to 1 veh/h/ln, so that only the grade is left.
        let tau_a_ffs = auto_rate(
            AutoRateForm::SpaceDecelerating,
            self.ffs,
            1.0,
            0.0,
            self.p_sut,
            self.p_tt,
            [tau_sut_kin, tau_tt_kin],
        );
        let tau_mix_ffs = p_a * tau_a_ffs + self.p_sut * tau_sut_kin + self.p_tt * tau_tt_kin;
        let ffs_mix = 3600.0 / tau_mix_ffs;
        let saf_mix = ffs_mix / self.ffs;

        // Step 4 -- breakpoint.
        let bp_ao = breakpoint_auto_only(self.ffs, self.caf_ao);
        let bp_mix = breakpoint_mixed(bp_ao, p_t, self.grade, self.length);

        // Steps 5 and 6 -- calibration speeds at capacity and at 90% of capacity.
        let calib = |v: f64| {
            let s_ao = speed_auto_only(self.ffs, v, caf_mix, capacity_ao, bp_ao);
            let dti = traffic_interaction(self.ffs, s_ao, caf_mix);
            let tau_a = auto_rate(
                AutoRateForm::SpaceDecelerating,
                self.ffs,
                v,
                dti,
                self.p_sut,
                self.p_tt,
                [tau_sut_kin, tau_tt_kin],
            );
            3600.0
                / (p_a * tau_a
                    + self.p_sut * (tau_sut_kin + dti)
                    + self.p_tt * (tau_tt_kin + dti))
        };
        let s_calib_cap = calib(capacity_mix);
        let s_calib_90cap = calib(0.9 * capacity_mix);

        // Step 7 -- the exponent.
        let phi_mix = 1.195
            * ((ffs_mix - s_calib_90cap) / (ffs_mix - s_calib_cap)).ln()
            / ((0.9 * capacity_mix - bp_mix) / (capacity_mix - bp_mix)).ln();

        // Step 8 -- speed and density.
        let (s_mix, d_mix) = if oversaturated {
            (None, None)
        } else if self.v_mix <= bp_mix {
            (Some(ffs_mix), Some(self.v_mix / ffs_mix))
        } else {
            let s = ffs_mix
                - (ffs_mix - s_calib_cap)
                    * ((self.v_mix - bp_mix) / (capacity_mix - bp_mix)).powf(phi_mix);
            (Some(s), Some(self.v_mix / s))
        };

        Ok(MixedFlowResult {
            caf_t_mix,
            rho_g_mix: rho,
            caf_g_mix: caf_g,
            caf_mix,
            capacity_ao,
            capacity_mix,
            tau_sut_kin,
            tau_tt_kin,
            tau_a_ffs,
            tau_mix_ffs,
            ffs_mix,
            saf_mix,
            bp_ao,
            bp_mix,
            s_calib_cap,
            s_calib_90cap,
            phi_mix,
            s_mix,
            d_mix,
            oversaturated,
        })
    }

    fn validate(&self) -> Result<(), String> {
        for (name, v) in [
            ("FFS", self.ffs),
            ("length", self.length),
            ("grade", self.grade),
            ("v_mix", self.v_mix),
            ("p_sut", self.p_sut),
            ("p_tt", self.p_tt),
            ("caf_ao", self.caf_ao),
        ] {
            if !v.is_finite() {
                return Err(format!("{name} must be finite, got {v}"));
            }
        }
        if self.ffs <= 0.0 {
            return Err(format!("FFS must be positive, got {} mi/h", self.ffs));
        }
        if self.length <= 0.0 {
            return Err(format!("length must be positive, got {} mi", self.length));
        }
        if self.v_mix < 0.0 {
            return Err(format!("v_mix must be non-negative, got {}", self.v_mix));
        }
        let p_t = self.p_sut + self.p_tt;
        if self.p_sut < 0.0 || self.p_tt < 0.0 || p_t >= 1.0 {
            return Err(format!(
                "truck proportions must be non-negative and sum below 1, got \
                 p_sut {} and p_tt {}",
                self.p_sut, self.p_tt
            ));
        }
        Ok(())
    }

    /// Parse from JSON, for the Python binding.
    pub fn from_json(s: &str) -> Result<Self, String> {
        serde_json::from_str(s).map_err(|e| e.to_string())
    }
}

/// Whether a truck entering a grade at `entry_rate` will speed up or slow down on it.
///
/// A truck settles towards the grade's crawl rate from whichever side it enters on, so the
/// branch is decided by comparing the entry rate against that crawl rate.
pub(crate) fn branch_for(
    class: TruckClass,
    grade_pct: f64,
    entry_rate: f64,
) -> Result<SpotBranch, String> {
    let crawl = truck_curves::crawl_rate(class, grade_pct, SpotBranch::Decelerating)?;
    Ok(if entry_rate < crawl {
        SpotBranch::Decelerating
    } else {
        SpotBranch::Accelerating
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ep5() -> MixedFlowSegment {
        MixedFlowSegment {
            ffs: 65.0,
            length: 2.0,
            grade: 5.0,
            v_mix: 1500.0,
            p_sut: 0.05,
            p_tt: 0.10,
            caf_ao: 1.0,
        }
    }

    #[test]
    fn ep5_capacity_half() {
        let r = ep5().analyze().unwrap();
        assert!((r.caf_t_mix - 0.135).abs() <= 0.001, "CAF_T,mix {}", r.caf_t_mix);
        assert!((r.rho_g_mix - 0.1215).abs() <= 1e-9, "rho {}", r.rho_g_mix);
        assert!((r.caf_g_mix - 0.131).abs() <= 0.001, "CAF_g,mix {}", r.caf_g_mix);
        assert!((r.caf_mix - 0.734).abs() <= 0.001, "CAF_mix {}", r.caf_mix);
        assert_eq!(r.capacity_ao, 2350.0);
        assert!((r.capacity_mix - 1725.0).abs() <= 2.0, "C_mix {}", r.capacity_mix);
    }

    #[test]
    fn ep5_free_flow_speed_half() {
        let r = ep5().analyze().unwrap();
        assert!((r.tau_sut_kin - 71.1).abs() <= 0.5, "tau_SUT {}", r.tau_sut_kin);
        assert!((r.tau_tt_kin - 92.2).abs() <= 0.5, "tau_TT {}", r.tau_tt_kin);
        assert!((r.tau_a_ffs - 55.4).abs() <= 0.1, "tau_a {}", r.tau_a_ffs);
        assert!((r.tau_mix_ffs - 59.87).abs() <= 0.1, "tau_mix {}", r.tau_mix_ffs);
        assert!((r.ffs_mix - 60.1).abs() <= 0.1, "FFS_mix {}", r.ffs_mix);
        assert!((r.saf_mix - 0.92).abs() <= 0.01, "SAF_mix {}", r.saf_mix);
    }

    /// The breakpoint is where the printed `+ 1` of Equation 26-16 shows itself.
    #[test]
    fn ep5_breakpoint_is_zero_as_printed() {
        let r = ep5().analyze().unwrap();
        assert_eq!(r.bp_ao, 1400.0);
        assert_eq!(r.bp_mix, 0.0, "Equation 26-16 as printed drives BP_mix to zero");
    }

    #[test]
    fn ep5_speed_and_density() {
        let r = ep5().analyze().unwrap();
        assert!((r.s_calib_cap - 37.5).abs() <= 0.3, "S_calib,cap {}", r.s_calib_cap);
        assert!((r.s_calib_90cap - 44.3).abs() <= 0.3, "S_calib,90 {}", r.s_calib_90cap);
        assert!((r.phi_mix - 4.07).abs() <= 0.1, "phi {}", r.phi_mix);
        assert!((r.s_mix.unwrap() - 47.3).abs() <= 0.3, "S_mix {:?}", r.s_mix);
        assert!((r.d_mix.unwrap() - 31.7).abs() <= 0.3, "D_mix {:?}", r.d_mix);
    }

    /// Equation 26-8 takes the kinematic truck rates. Feeding it the traffic-interaction
    /// adjusted rates instead is the natural mistake and produces a plausible but wrong
    /// answer, so pin the size of the error.
    #[test]
    fn auto_rate_uses_kinematic_truck_rates() {
        let r = ep5().analyze().unwrap();
        let dti = {
            let s_ao = speed_auto_only(65.0, r.capacity_mix, r.caf_mix, r.capacity_ao, r.bp_ao);
            traffic_interaction(65.0, s_ao, r.caf_mix)
        };
        let right = auto_rate(
            AutoRateForm::SpaceDecelerating, 65.0, r.capacity_mix, dti, 0.05, 0.10,
            [r.tau_sut_kin, r.tau_tt_kin],
        );
        let wrong = auto_rate(
            AutoRateForm::SpaceDecelerating, 65.0, r.capacity_mix, dti, 0.05, 0.10,
            [r.tau_sut_kin + dti, r.tau_tt_kin + dti],
        );
        assert!((right - 92.9).abs() <= 0.5, "published 92.9 s/mi, got {right}");
        assert!(wrong > 110.0, "the adjusted-rate mistake should be far off, got {wrong}");
    }

    #[test]
    fn demand_above_capacity_reports_oversaturation_rather_than_a_speed() {
        let mut s = ep5();
        s.v_mix = 2000.0;
        let r = s.analyze().unwrap();
        assert!(r.oversaturated);
        assert!(r.s_mix.is_none() && r.d_mix.is_none());
    }

    #[test]
    fn undigitised_grade_is_refused_rather_than_extrapolated() {
        let mut s = ep5();
        s.grade = 7.0;
        let e = s.analyze().expect_err("7% is outside Stage 1");
        assert!(e.contains("digitised"), "{e}");
    }

    #[test]
    fn bad_inputs_are_rejected() {
        let mut s = ep5();
        s.p_tt = 0.99;
        assert!(s.analyze().is_err());
        let mut s = ep5();
        s.length = 0.0;
        assert!(s.analyze().is_err());
        let mut s = ep5();
        s.ffs = f64::NAN;
        assert!(s.analyze().is_err());
    }
}
