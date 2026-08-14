//! Integration test for HCM Chapter 25, Example Problem 11 (Estimating Freeway Composite
//! Grade Operations with the Mixed-Flow Model).
//!
//! Three basic segments (1.5 mi at 3%, 2.0 mi at 2%, 1.0 mi at 5%), six-lane freeway, 5% SUTs
//! and 10% TTs, FFS 65 mi/h, 1,500 veh/h/ln at PHF 1.0.
//!
//! Three published values in this example are internally inconsistent, and this file asserts
//! the self-consistent one in each case rather than the printed one. Each is argued at the
//! test that pins it:
//!   - Step 6 of Segment 2 prints `tau_mix,2 = 62.6 s/mi` and then divides 3,600 by 61.3 on
//!     the next line. Target 61.3.
//!   - Step 7's prose says the segment travel times "equal 294 s"; they sum to 291.5, which is
//!     what Equation 25-70 divides by to reach the published 55.6 mi/h.
//!   - Exhibit 25-109's end-of-Segment-1 row is 59.5 / 56.1 / 56.4; the rates printed for that
//!     node give 56.4 / 56.1 / 46.1.
//!
//! The travel time and spot rate curves this example reads by eye are digitised in
//! `src/hcm/common/truck_curves.rs`, whose own tests reproduce all twenty of the reads the
//! two worked examples state.

use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use transportations_library::basicfreeways::composite_grade::CompositeGrade;

fn load(name: &str) -> CompositeGrade {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/ExampleCases/hcm/Chapter25");
    path.push(name);
    let f = File::open(&path).unwrap_or_else(|_| panic!("Unable to open {path:?}"));
    serde_json::from_reader(BufReader::new(f)).expect("Failed to parse fixture JSON")
}

fn assert_approx(actual: f64, expected: f64, tol: f64, label: &str) {
    assert!(
        (actual - expected).abs() <= tol,
        "{label}: got {actual}, expected {expected} (+-{tol})"
    );
}

fn ep11() -> CompositeGrade {
    load("ep11_composite_grade.json")
}

/// Step 2. Capacity is computed per segment and the facility takes the tightest.
#[test]
fn ch25_ep11_capacity_is_governed_by_the_steepest_segment() {
    let r = ep11().analyze().expect("EP11");
    let want_caf_g = [0.067, 0.042, 0.122];
    let want_caf = [0.798, 0.823, 0.743];
    let want_cap = [1875.0, 1934.0, 1746.0];
    for (i, s) in r.segments.iter().enumerate() {
        assert_approx(s.caf_g_mix, want_caf_g[i], 0.001, &format!("segment {} CAF_g,mix", i + 1));
        assert_approx(s.caf_mix, want_caf[i], 0.001, &format!("segment {} CAF_mix", i + 1));
        assert_approx(s.capacity_mix, want_cap[i], 2.0, &format!("segment {} C_mix (veh/h/ln)", i + 1));
    }
    assert_eq!(r.governing_segment, 2, "the 1 mi 5% segment governs");
    assert_approx(r.capacity_mix, 1746.0, 2.0, "governing C_mix (veh/h/ln)");
    assert!(!r.oversaturated);
}

/// Steps 3 through 6, per segment.
#[test]
fn ch25_ep11_segment_speeds_and_travel_times() {
    let r = ep11().analyze().expect("EP11");
    let want_speed = [57.7, 58.7, 47.9];
    let want_time = [93.6, 122.7, 75.2];
    for (i, s) in r.segments.iter().enumerate() {
        assert_approx(s.s_mix, want_speed[i], 0.3, &format!("segment {} S_mix (mi/h)", i + 1));
        assert_approx(s.travel_time, want_time[i], 0.7, &format!("segment {} t_mix (s)", i + 1));
    }
}

/// VERIFY-HCM. Segment 2, Step 6 prints
/// `tau_mix,2 = 0.85 x 61.4 + 0.05 x 62.01 + 0.10 x 73.51 = 62.6 s/mi`
/// and immediately below it `S_mix,2 = 3,600/61.3 = 58.7 mi/h`. The three rates that line
/// substitutes match nothing computed in Step 5, which produced 60.5, 59.11 and 69.51; those
/// give 0.85(60.5) + 0.05(59.11) + 0.10(69.51) = 61.33, the value the next line uses. So the
/// 62.6 line is a leftover from a different draft. 61.3 is also what the published segment
/// travel time of 122.7 s and Exhibits 25-110 and 25-111 all agree with.
#[test]
fn ch25_ep11_segment2_rate_is_self_consistent_not_the_printed_62_6() {
    let s2 = &ep11().analyze().expect("EP11").segments[1];
    assert_approx(s2.tau_mix, 61.3, 0.2, "tau_mix,2 (s/mi)");
    assert_approx(s2.s_mix, 58.7, 0.2, "S_mix,2 (mi/h)");
    assert!(
        (s2.tau_mix - 62.6).abs() > 1.0,
        "the printed 62.6 s/mi is not reproducible from the example's own Step 5 rates"
    );
}

/// Step 7. VERIFY-HCM: the prose says the three segment travel times "equal 294 s". They sum
/// to 291.5, and it is 291.5 that Equation 25-70 divides into: 3,600 x 4.5 / 291.5 = 55.6,
/// against 55.1 for 294.
#[test]
fn ch25_ep11_overall_speed_uses_the_summed_travel_times() {
    let r = ep11().analyze().expect("EP11");
    assert_approx(r.total_length, 4.5, 1e-9, "total length (mi)");
    assert_approx(r.total_travel_time, 291.5, 1.5, "sum of segment travel times (s)");
    assert_approx(r.s_mix_overall, 55.6, 0.3, "S_mix,oa (mi/h)");
    let from_prose = 3600.0 * 4.5 / 294.0;
    assert!(
        (r.s_mix_overall - from_prose).abs() > 0.3,
        "294 s would give {from_prose:.1} mi/h, which is not the published 55.6"
    );
}

/// Exhibit 25-110, space mean speeds by segment for autos, SUTs and TTs.
#[test]
fn ch25_ep11_exhibit_25_110_space_mean_speeds() {
    let r = ep11().analyze().expect("EP11");
    let want = [[58.7, 57.0, 50.6], [59.5, 60.9, 51.8], [49.9, 46.6, 36.3]];
    let names = ["autos", "SUTs", "TTs"];
    for (i, s) in r.segments.iter().enumerate() {
        for (k, name) in names.iter().enumerate() {
            assert_approx(
                s.space_speeds[k],
                want[i][k],
                0.5,
                &format!("segment {} {name} space mean speed (mi/h)", i + 1),
            );
        }
    }
}

/// Exhibit 25-111, overall space mean speeds by class.
#[test]
fn ch25_ep11_exhibit_25_111_overall_space_mean_speeds() {
    let r = ep11().analyze().expect("EP11");
    let names = ["autos", "SUTs", "TTs"];
    for (k, want) in [56.8, 55.8, 47.0].into_iter().enumerate() {
        assert_approx(
            r.overall_space_speeds[k],
            want,
            0.4,
            &format!("overall {} space mean speed (mi/h)", names[k]),
        );
    }
}

/// Exhibit 25-109, spot speeds at the facility entry and at the end of each segment.
///
/// VERIFY-HCM: the end-of-Segment-1 row is published as autos 59.5 / SUTs 56.1 / TTs 56.4 and
/// is wrong. The rates Step 5 prints for that node are tau_f,a = 63.8, tau_f,SUT = 64.15 and
/// tau_f,TT = 78.15 s/mi, which are 56.4, 56.1 and 46.1 mi/h. So the SUT value is right, the
/// number labelled "TTs 56.4" is the automobile speed, and "autos 59.5" is the facility entry
/// speed duplicated from the row above. The other three rows verify exactly as printed, which
/// is why the defect is read as a transcription slip in one row rather than a modelling
/// difference.
#[test]
fn ch25_ep11_exhibit_25_109_spot_speeds_segment1_row_corrected() {
    let r = ep11().analyze().expect("EP11");
    let names = ["autos", "SUTs", "TTs"];
    for (k, want) in [59.5, 59.5, 59.5].into_iter().enumerate() {
        assert_approx(
            r.entry_spot_speeds[k],
            want,
            0.3,
            &format!("facility entry {} spot speed (mi/h)", names[k]),
        );
    }
    let want = [
        [56.4, 56.1, 46.1], // corrected; the exhibit prints 59.5 / 56.1 / 56.4
        [60.9, 60.9, 54.0],
        [45.2, 42.2, 31.8],
    ];
    for (i, s) in r.segments.iter().enumerate() {
        for (k, name) in names.iter().enumerate() {
            assert_approx(
                s.spot_speeds[k],
                want[i][k],
                1.0,
                &format!("end of segment {} {name} spot speed (mi/h)", i + 1),
            );
        }
    }
}

/// The whole point of Chapter 25 over Chapter 26 is that speed is carried across segment
/// boundaries. Segment 2 is entered at about 60.9 mi/h by SUTs and 49.5 mi/h by TTs, which is
/// what makes the example read its Segment 2 curves off Exhibits 25-A6 and 25-A15 rather than
/// the 65 mi/h graphs. Without the chaining every segment would restart at free-flow speed and
/// the whole facility would come out optimistic, with nothing failing.
#[test]
fn ch25_ep11_speed_is_carried_between_segments() {
    let r = ep11().analyze().expect("EP11");
    let s1 = &r.segments[0];
    assert_approx(3600.0 / s1.tau_f_sut_kin, 60.9, 1.0, "SUT speed entering segment 2 (mi/h)");
    assert_approx(3600.0 / s1.tau_f_tt_kin, 49.5, 1.0, "TT speed entering segment 2 (mi/h)");
    assert!(!r.segments[1].decelerating, "segment 2 is where the trucks recover");
    assert!(r.segments[2].decelerating, "segment 3 slows them again");
}

/// Chaining must actually change the answer. The middle segment of Example Problem 11 is
/// entered at about 60.9 and 49.5 mi/h; analysed on its own it would be entered at free-flow
/// speed instead. If the chaining were ever dropped the two would agree, and nothing else in
/// this file would fail, because every published value would simply drift a little.
#[test]
fn ch25_ep11_chaining_changes_the_middle_segment() {
    let chained = ep11().analyze().expect("EP11");
    let mut alone = ep11();
    alone.segments = vec![alone.segments[1].clone()];
    let standalone = alone.analyze().expect("2% segment on its own");

    let a = chained.segments[1].s_mix;
    let b = standalone.segments[0].s_mix;
    assert!(
        (a - b).abs() > 0.5,
        "the 2% segment should be slower when entered with trucks already slowed by the 3% \
         grade above it: chained {a:.2} mi/h, standalone {b:.2} mi/h"
    );
    assert!(a < b, "entering slower cannot make the segment faster");
}

/// Reversing the grades puts the 5% segment first, which slows the trucks to speeds Stage 1
/// has no digitised curve for. The right behaviour is to say so, naming what is missing,
/// rather than to extrapolate a curve whose crawl speed is grade-specific.
#[test]
fn ch25_reordering_outside_stage1_is_refused_by_name() {
    let mut reversed = ep11();
    reversed.segments.reverse();
    let e = reversed.analyze().expect_err("reversed order leaves Stage 1");
    assert!(e.contains("2.5 mi/h"), "error should name the snapping rule, got: {e}");
    assert!(e.contains("digitised"), "error should say what is missing, got: {e}");
}

/// Stage 1 digitised only the curves the published examples need. Anything else must say what
/// is missing rather than extrapolate, because each grade settles at its own crawl speed and
/// an extrapolated curve would be quietly wrong.
#[test]
fn ch25_ep11_undigitised_grade_is_refused() {
    let mut c = ep11();
    c.segments[1].grade = 7.0;
    let e = c.analyze().expect_err("7% is outside Stage 1");
    assert!(e.contains("digitised"), "error should name what is missing, got: {e}");
}
