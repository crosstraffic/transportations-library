//! HCM Chapter 25: the mixed-flow model chained across a composite grade.
//!
//! A composite grade is a run of consecutive basic segments with different grades. The single
//! grade model of Chapter 26 cannot simply be applied segment by segment, because a truck
//! arrives at each segment carrying the speed the previous one left it at. Chapter 25 threads
//! that state through: each segment is entered at the previous segment's final spot rate, the
//! curve is re-entered at whatever abscissa matches that rate, and the segment length is
//! advanced from there.
//!
//! Chapter 25 states its Equations 25-53 to 25-70 are "identical to those presented in Chapter
//! 26, although they have different equation numbers". The shared primitives therefore live in
//! `mixed_flow` and are reused here; what this module adds is the chaining, plus the
//! distinction Chapter 26 does not need between a SPOT rate (at a point, carried forward) and a
//! SPACE rate (averaged over a segment, used for its travel time).
//!
//! Equations implemented:
//! - Equation 25-53 to 25-57: per-segment capacity, governed by the minimum across segments
//! - Equation 25-58/25-59: kinematic travel time rate, within and beyond 10,000 ft
//! - Equation 25-60/25-61: truck spot and space rates including traffic interaction
//! - Equation 25-62/25-63: traffic interaction term and auto-only speed
//! - Equation 25-64: automobile spot rate
//! - Equation 25-65: automobile space rate where trucks decelerate
//! - Equation 25-66: automobile space rate where trucks accelerate
//! - Equation 25-67/25-68: mixed-flow rate and speed per segment
//! - Equation 25-69/25-70: segment travel times and the overall space mean speed

use serde::{Deserialize, Serialize};

use super::mixed_flow::{
    auto_rate, branch_for, breakpoint_auto_only, caf_grade, caf_trucks, capacity_auto_only,
    speed_auto_only, traffic_interaction, AutoRateForm,
};
use crate::hcm::common::truck_curves::{self, TruckClass};

/// One grade of a composite grade.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GradeSegment {
    /// Length of this segment (mi).
    pub length: f64,
    /// Grade (percent; 5.0 means a 5% upgrade).
    pub grade: f64,
}

/// Inputs to a composite-grade mixed-flow analysis (HCM Chapter 25).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompositeGrade {
    /// Free-flow speed of the facility (mi/h). Shared by every segment, and the index into
    /// the delta exhibits regardless of which travel time curve a segment reads.
    pub ffs: f64,
    /// Mixed demand flow rate (veh/h/ln).
    pub v_mix: f64,
    /// Proportion of single-unit trucks in the stream (decimal).
    pub p_sut: f64,
    /// Proportion of tractor-trailers in the stream (decimal).
    pub p_tt: f64,
    /// Consecutive grades, in the order a vehicle meets them.
    pub segments: Vec<GradeSegment>,
    /// Auto-only capacity adjustment factor (decimal).
    #[serde(default = "unit_caf")]
    pub caf_ao: f64,
}

fn unit_caf() -> f64 {
    1.0
}

/// Per-segment results of the Chapter 25 chain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SegmentResult {
    /// Equation 25-55 (decimal).
    pub caf_g_mix: f64,
    /// Equation 25-53 (decimal).
    pub caf_mix: f64,
    /// Equation 25-57 (veh/h/ln).
    pub capacity_mix: f64,
    /// Auto-only speed, Equation 25-63 (mi/h).
    pub s_ao: f64,
    /// Traffic interaction term, Equation 25-62 (s/mi).
    pub d_tau_ti: f64,
    /// Kinematic SUT spot rate at the end of the segment (s/mi).
    pub tau_f_sut_kin: f64,
    /// Kinematic TT spot rate at the end of the segment (s/mi).
    pub tau_f_tt_kin: f64,
    /// Kinematic SUT space rate across the segment (s/mi).
    pub tau_s_sut_kin: f64,
    /// Kinematic TT space rate across the segment (s/mi).
    pub tau_s_tt_kin: f64,
    /// Automobile spot rate at the end of the segment, Equation 25-64 (s/mi).
    pub tau_f_a: f64,
    /// Automobile space rate across the segment, Equation 25-65 or 25-66 (s/mi).
    pub tau_s_a: f64,
    /// Whether both truck classes decelerate on this segment.
    pub decelerating: bool,
    /// Equation 25-67 (s/mi).
    pub tau_mix: f64,
    /// Equation 25-68 (mi/h).
    pub s_mix: f64,
    /// Equation 25-69 (s).
    pub travel_time: f64,
    /// Spot speeds at the end of the segment, autos / SUTs / TTs (mi/h). Exhibit 25-109.
    pub spot_speeds: [f64; 3],
    /// Space mean speeds across the segment, autos / SUTs / TTs (mi/h). Exhibit 25-110.
    pub space_speeds: [f64; 3],
}

/// Everything the Chapter 25 chain produces.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompositeGradeResult {
    /// Per-segment results, in input order.
    pub segments: Vec<SegmentResult>,
    /// Governing mixed-flow capacity, the minimum across segments (veh/h/ln).
    pub capacity_mix: f64,
    /// Index of the segment that governs capacity.
    pub governing_segment: usize,
    /// Spot speeds where the facility is entered, autos / SUTs / TTs (mi/h).
    pub entry_spot_speeds: [f64; 3],
    /// Total length (mi).
    pub total_length: f64,
    /// Sum of the segment travel times, Equation 25-70 (s).
    pub total_travel_time: f64,
    /// Equation 25-70 (mi/h).
    pub s_mix_overall: f64,
    /// Overall space mean speeds, autos / SUTs / TTs (mi/h). Exhibit 25-111.
    pub overall_space_speeds: [f64; 3],
    /// True when demand exceeds the governing mixed-flow capacity.
    pub oversaturated: bool,
}

impl CompositeGrade {
    /// Run the Chapter 25 composite-grade chain.
    pub fn analyze(&self) -> Result<CompositeGradeResult, String> {
        self.validate()?;
        let p_t = self.p_sut + self.p_tt;
        let p_a = 1.0 - p_t;
        let ffs_rate = 3600.0 / self.ffs;
        let capacity_ao = capacity_auto_only(self.ffs);
        let bp_ao = breakpoint_auto_only(self.ffs, self.caf_ao);
        let caf_t_mix = caf_trucks(p_t);

        // Step 2 -- capacity segment by segment, governed by the tightest.
        let mut cafs = Vec::with_capacity(self.segments.len());
        for s in &self.segments {
            let caf_g = caf_grade(p_t, s.grade, s.length);
            let caf_mix = self.caf_ao - caf_t_mix - caf_g;
            if caf_mix <= 0.0 {
                return Err(format!(
                    "mixed-flow CAF came out at {caf_mix:.4} on the {}% grade, which is not a \
                     usable capacity adjustment",
                    s.grade
                ));
            }
            cafs.push((caf_g, caf_mix, capacity_ao * caf_mix));
        }
        let (governing_segment, capacity_mix) = cafs
            .iter()
            .enumerate()
            .map(|(i, c)| (i, c.2))
            .min_by(|a, b| a.1.partial_cmp(&b.1).expect("finite capacities compare"))
            .expect("at least one segment");
        let oversaturated = self.v_mix > capacity_mix;

        // Step 3 -- a vehicle enters the facility at the free-flow rate.
        let mut entry = [ffs_rate, ffs_rate]; // SUT, TT
        let mut out: Vec<SegmentResult> = Vec::with_capacity(self.segments.len());
        let mut entry_spot_speeds = [0.0_f64; 3];

        for (j, seg) in self.segments.iter().enumerate() {
            let (caf_g_mix, caf_mix, cap_j) = cafs[j];
            let s_ao = speed_auto_only(self.ffs, self.v_mix, caf_mix, capacity_ao, bp_ao);
            let d_tau_ti = traffic_interaction(self.ffs, s_ao, caf_mix);
            if j == 0 {
                let e = ffs_rate + d_tau_ti;
                entry_spot_speeds = [3600.0 / e, 3600.0 / e, 3600.0 / e];
            }

            let mut f_kin = [0.0_f64; 2];
            let mut s_kin = [0.0_f64; 2];
            for (k, class) in [TruckClass::Sut, TruckClass::Tt].into_iter().enumerate() {
                // Step 4 -- re-enter the spot curve where its ordinate equals the rate this
                // truck arrived with, then advance by the segment length.
                let branch = branch_for(class, seg.grade, entry[k])?;
                let x0 = truck_curves::spot_distance(class, seg.grade, branch, entry[k])?;
                let end = truck_curves::spot_rate(class, seg.grade, branch, x0 + seg.length * 5280.0)?;
                // A truck cannot travel faster than the facility free-flow speed, which is what
                // the example does when a shallow grade's crawl speed exceeds the FFS.
                f_kin[k] = end.max(ffs_rate);

                // The space rate comes from the cumulative travel time family, read on the
                // graph whose starting speed is nearest the speed this truck arrived at.
                let family = truck_curves::nearest_exhibit(class, 3600.0 / entry[k])?;
                let rate = truck_curves::travel_time_rate(
                    class, family, seg.grade, seg.length, self.ffs,
                )?;
                s_kin[k] = rate.max(ffs_rate);
            }

            // Both classes decelerate, both accelerate, or they disagree.
            //
            // VERIFY-HCM: Chapter 25 defines only the first two cases, yet Equation 25-65
            // versus 25-66 is a hard branch, so the mixed case is undefined in the manual. The
            // deceleration form is used, because it is the conservative one (it yields the
            // higher automobile rate, so the lower speed) and because it is the form Chapter
            // 26 uses unconditionally for a single grade.
            let accelerating = f_kin[0] < entry[0] && f_kin[1] < entry[1];
            let decelerating = f_kin[0] > entry[0] && f_kin[1] > entry[1];
            let form = if accelerating {
                AutoRateForm::SpaceAccelerating
            } else {
                AutoRateForm::SpaceDecelerating
            };

            let mut tau_f_a = auto_rate(
                AutoRateForm::Spot, self.ffs, self.v_mix, d_tau_ti,
                self.p_sut, self.p_tt, f_kin,
            );
            let tau_s_a = auto_rate(
                form, self.ffs, self.v_mix, d_tau_ti,
                self.p_sut, self.p_tt, s_kin,
            );

            let tau_f_sut = f_kin[0] + d_tau_ti;
            let tau_f_tt = f_kin[1] + d_tau_ti;
            let tau_s_sut = s_kin[0] + d_tau_ti;
            let tau_s_tt = s_kin[1] + d_tau_ti;

            // VERIFY-HCM: Chapter 25 says "the analyst should check that the automobile spot
            // rates are always less than or equal to the truck spot rates". Example Problem 11
            // applies that to the SPOT rate only. Its Segment 2 space rates are left with the
            // automobile rate above the SUT rate, and Exhibit 25-110 then publishes autos at
            // 59.5 mi/h travelling slower than SUTs at 60.9 mi/h. Applying the clamp to the
            // space rates as well does not reproduce the published example, so it is applied
            // to spot rates only.
            tau_f_a = tau_f_a.min(tau_f_sut).min(tau_f_tt);

            let tau_mix = p_a * tau_s_a + self.p_sut * tau_s_sut + self.p_tt * tau_s_tt;
            let s_mix = 3600.0 / tau_mix;
            let travel_time = 3600.0 * seg.length / s_mix;

            out.push(SegmentResult {
                caf_g_mix,
                caf_mix,
                capacity_mix: cap_j,
                s_ao,
                d_tau_ti,
                tau_f_sut_kin: f_kin[0],
                tau_f_tt_kin: f_kin[1],
                tau_s_sut_kin: s_kin[0],
                tau_s_tt_kin: s_kin[1],
                tau_f_a,
                tau_s_a,
                decelerating,
                tau_mix,
                s_mix,
                travel_time,
                spot_speeds: [3600.0 / tau_f_a, 3600.0 / tau_f_sut, 3600.0 / tau_f_tt],
                space_speeds: [3600.0 / tau_s_a, 3600.0 / tau_s_sut, 3600.0 / tau_s_tt],
            });

            entry = f_kin;
        }

        let total_length: f64 = self.segments.iter().map(|s| s.length).sum();
        let total_travel_time: f64 = out.iter().map(|s| s.travel_time).sum();
        let s_mix_overall = 3600.0 * total_length / total_travel_time;

        // Exhibit 25-111: each class weighted by segment length, not by segment time.
        let mut overall_space_speeds = [0.0_f64; 3];
        for (k, slot) in overall_space_speeds.iter_mut().enumerate() {
            let t: f64 = self
                .segments
                .iter()
                .zip(&out)
                .map(|(seg, r)| seg.length * 3600.0 / r.space_speeds[k])
                .sum();
            *slot = 3600.0 * total_length / t;
        }

        Ok(CompositeGradeResult {
            segments: out,
            capacity_mix,
            governing_segment,
            entry_spot_speeds,
            total_length,
            total_travel_time,
            s_mix_overall,
            overall_space_speeds,
            oversaturated,
        })
    }

    fn validate(&self) -> Result<(), String> {
        if self.segments.is_empty() {
            return Err("a composite grade needs at least one segment".to_string());
        }
        for (name, v) in [
            ("FFS", self.ffs),
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
        for (i, s) in self.segments.iter().enumerate() {
            if !s.length.is_finite() || s.length <= 0.0 {
                return Err(format!("segment {i} length must be positive, got {} mi", s.length));
            }
            if !s.grade.is_finite() {
                return Err(format!("segment {i} grade must be finite, got {}", s.grade));
            }
        }
        Ok(())
    }

    /// Parse from JSON, for the Python binding.
    pub fn from_json(s: &str) -> Result<Self, String> {
        serde_json::from_str(s).map_err(|e| e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Chapter 25 Example Problem 11: three segments, six-lane freeway, FFS 65 mi/h.
    fn ep11() -> CompositeGrade {
        CompositeGrade {
            ffs: 65.0,
            v_mix: 1500.0,
            p_sut: 0.05,
            p_tt: 0.10,
            segments: vec![
                GradeSegment { length: 1.5, grade: 3.0 },
                GradeSegment { length: 2.0, grade: 2.0 },
                GradeSegment { length: 1.0, grade: 5.0 },
            ],
            caf_ao: 1.0,
        }
    }

    #[test]
    fn ep11_per_segment_capacity() {
        let r = ep11().analyze().unwrap();
        let want_g = [0.067, 0.042, 0.122];
        let want_caf = [0.798, 0.823, 0.743];
        let want_cap = [1875.0, 1934.0, 1746.0];
        for (i, s) in r.segments.iter().enumerate() {
            assert!((s.caf_g_mix - want_g[i]).abs() <= 0.001, "seg{} CAF_g {}", i + 1, s.caf_g_mix);
            assert!((s.caf_mix - want_caf[i]).abs() <= 0.001, "seg{} CAF_mix {}", i + 1, s.caf_mix);
            assert!((s.capacity_mix - want_cap[i]).abs() <= 2.0, "seg{} C_mix {}", i + 1, s.capacity_mix);
        }
        assert_eq!(r.governing_segment, 2, "the 5% segment governs capacity");
        assert!((r.capacity_mix - 1746.0).abs() <= 2.0, "C_mix {}", r.capacity_mix);
    }

    #[test]
    fn ep11_segment_speeds_and_travel_times() {
        let r = ep11().analyze().unwrap();
        let want_s = [57.7, 58.7, 47.9];
        let want_t = [93.6, 122.7, 75.2];
        for (i, s) in r.segments.iter().enumerate() {
            assert!((s.s_mix - want_s[i]).abs() <= 0.3, "seg{} S_mix {}", i + 1, s.s_mix);
            assert!((s.travel_time - want_t[i]).abs() <= 0.7, "seg{} t {}", i + 1, s.travel_time);
        }
    }

    /// VERIFY-HCM: Step 6 of Segment 2 prints `tau_mix,2 = ... = 62.6 s/mi` and then, on the
    /// very next line, `S_mix,2 = 3,600/61.3 = 58.7 mi/h`. The three inputs the 62.6 line
    /// substitutes (61.4, 62.01, 73.51) match nothing computed in Step 5, whose values give
    /// 61.33. The 62.6 is a leftover from an earlier draft; 61.3 is the self-consistent value
    /// and is what the published segment travel time and Exhibits 25-110 and 25-111 all agree
    /// with, so 61.3 is asserted here.
    #[test]
    fn ep11_segment2_rate_is_the_self_consistent_value_not_the_printed_one() {
        let r = ep11().analyze().unwrap();
        let s2 = &r.segments[1];
        assert!((s2.tau_mix - 61.3).abs() <= 0.2, "tau_mix,2 {}", s2.tau_mix);
        assert!((s2.s_mix - 58.7).abs() <= 0.2, "S_mix,2 {}", s2.s_mix);
    }

    #[test]
    fn ep11_overall() {
        let r = ep11().analyze().unwrap();
        // VERIFY-HCM: Step 7's prose says the three segment times "equal 294 s". They sum to
        // 291.5, and 291.5 is what Equation 25-70 then divides by to reach the published
        // 55.6 mi/h. The sum is asserted, not the prose.
        assert!((r.total_travel_time - 291.5).abs() <= 1.5, "sum t {}", r.total_travel_time);
        assert!((r.s_mix_overall - 55.6).abs() <= 0.3, "S_mix,oa {}", r.s_mix_overall);
        assert_eq!(r.total_length, 4.5);
    }

    /// Exhibit 25-110, space mean speeds by segment and class.
    #[test]
    fn ep11_exhibit_25_110_space_speeds() {
        let r = ep11().analyze().unwrap();
        let want = [[58.7, 57.0, 50.6], [59.5, 60.9, 51.8], [49.9, 46.6, 36.3]];
        for (i, s) in r.segments.iter().enumerate() {
            for k in 0..3 {
                assert!(
                    (s.space_speeds[k] - want[i][k]).abs() <= 0.5,
                    "seg{} class{} space speed {} vs published {}",
                    i + 1, k, s.space_speeds[k], want[i][k]
                );
            }
        }
    }

    /// Exhibit 25-111, overall space mean speeds by class.
    #[test]
    fn ep11_exhibit_25_111_overall_space_speeds() {
        let r = ep11().analyze().unwrap();
        for (k, want) in [56.8, 55.8, 47.0].into_iter().enumerate() {
            assert!(
                (r.overall_space_speeds[k] - want).abs() <= 0.4,
                "class{k} overall {} vs published {want}", r.overall_space_speeds[k]
            );
        }
    }

    /// Exhibit 25-109, spot speeds at each node.
    ///
    /// VERIFY-HCM: the exhibit's end-of-Segment-1 row reads 59.5 / 56.1 / 56.4 and is wrong.
    /// The rates Step 5 prints for that node give 56.4 mi/h for autos, 56.1 for SUTs and 46.1
    /// for TTs. So the SUT entry is right, the number labelled "TTs 56.4" is actually the
    /// automobile speed, and the "autos 59.5" is the facility entry speed duplicated from the
    /// row above. The corrected triple is asserted; the other three rows of the exhibit verify
    /// as printed.
    #[test]
    fn ep11_exhibit_25_109_spot_speeds_with_segment1_corrected() {
        let r = ep11().analyze().unwrap();
        for (k, want) in [59.5, 59.5, 59.5].into_iter().enumerate() {
            assert!(
                (r.entry_spot_speeds[k] - want).abs() <= 0.3,
                "entry class{k} {}", r.entry_spot_speeds[k]
            );
        }
        let want = [[56.4, 56.1, 46.1], [60.9, 60.9, 54.0], [45.2, 42.2, 31.8]];
        for (i, s) in r.segments.iter().enumerate() {
            for k in 0..3 {
                assert!(
                    (s.spot_speeds[k] - want[i][k]).abs() <= 1.0,
                    "seg{} class{} spot speed {} vs {}",
                    i + 1, k, s.spot_speeds[k], want[i][k]
                );
            }
        }
    }

    /// The example's own curve choices: Segment 2 is entered at about 60.9 mi/h by SUTs and
    /// 49.5 mi/h by TTs, which selects Exhibits 25-A6 and 25-A15. If the chaining ever stopped
    /// carrying speed forward, both would fall back to the 65 mi/h exhibits and every
    /// downstream number would move a little without anything failing.
    #[test]
    fn ep11_segment2_is_entered_below_free_flow() {
        let r = ep11().analyze().unwrap();
        let s1 = &r.segments[0];
        assert!((3600.0 / s1.tau_f_sut_kin - 60.9).abs() <= 1.0, "SUT entry {}", 3600.0 / s1.tau_f_sut_kin);
        assert!((3600.0 / s1.tau_f_tt_kin - 49.5).abs() <= 1.0, "TT entry {}", 3600.0 / s1.tau_f_tt_kin);
    }

    /// Segment 2 is where both classes accelerate, so it is the one segment that exercises
    /// Equation 25-66 rather than 25-65.
    #[test]
    fn ep11_segment2_uses_the_accelerating_form() {
        let r = ep11().analyze().unwrap();
        assert!(r.segments[0].decelerating, "segment 1 decelerates");
        assert!(!r.segments[1].decelerating, "segment 2 accelerates");
        assert!(r.segments[2].decelerating, "segment 3 decelerates");
    }

    /// The spot clamp fires on Segment 2 (60.1 down to 59.11) and the space rates are left
    /// alone, which is what lets Exhibit 25-110 publish autos slower than SUTs there.
    #[test]
    fn spot_clamp_applies_but_space_clamp_does_not() {
        let r = ep11().analyze().unwrap();
        let s2 = &r.segments[1];
        assert!(
            s2.tau_f_a <= s2.tau_f_sut_kin + s2.d_tau_ti + 1e-9,
            "spot rate should be clamped to the SUT rate"
        );
        assert!(
            s2.tau_s_a > s2.tau_s_sut_kin + s2.d_tau_ti,
            "space rate must be left unclamped, or Exhibit 25-110 will not reproduce"
        );
    }

    #[test]
    fn single_segment_matches_the_chapter_26_chain_shape() {
        let c = CompositeGrade {
            ffs: 65.0,
            v_mix: 1500.0,
            p_sut: 0.05,
            p_tt: 0.10,
            segments: vec![GradeSegment { length: 1.5, grade: 3.0 }],
            caf_ao: 1.0,
        };
        let r = c.analyze().unwrap();
        assert_eq!(r.segments.len(), 1);
        assert_eq!(r.governing_segment, 0);
        assert!((r.s_mix_overall - r.segments[0].s_mix).abs() < 1e-9);
    }

    #[test]
    fn bad_inputs_are_rejected() {
        let mut c = ep11();
        c.segments.clear();
        assert!(c.analyze().is_err());
        let mut c = ep11();
        c.segments[0].length = -1.0;
        assert!(c.analyze().is_err());
        let mut c = ep11();
        c.segments[1].grade = 7.0;
        assert!(c.analyze().is_err(), "7% is outside Stage 1 and must not be extrapolated");
    }
}
