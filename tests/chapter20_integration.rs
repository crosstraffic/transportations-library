//! Full-pipeline integration tests for HCM Chapter 20 (TWSC intersections)
//! against the published answers of HCM Chapter 32, TWSC Example Problems 1,
//! 2, 3, and 4.
//!
//! Tolerances: LOS exact; control delays within +-0.5 s/veh; capacities
//! within +-5 veh/h of the published (rounded) values. Example Problem 4
//! reproduces the shared-major-left case; the two oversaturated minor-street
//! left-turn delays use a wider, documented tolerance because Equation 20-61
//! is steep near v/c = 1.7 and the book rounds c_m to an integer.
//!
//! Example Problem 2 covers the Chapter 20 Section 5 pedestrian mode
//! (`src/hcm/twsc/pedestrian.rs`), which is a distinct procedure from the
//! Section 4 pedestrian-*impedance* extension in `src/hcm/twsc/twsc.rs` where
//! pedestrian volumes v13-v16 reduce vehicular movement capacity. All three of
//! its scenarios are asserted against the Step 2-7 prose and Exhibit 32-7. The
//! Scenario A gap delay carries a proportional tolerance rather than the
//! +-0.5 s used elsewhere, because d_g = 761 s is published to three
//! significant figures and Equation 20-82 is exponential in v * t_c,G.

use std::fs;

use transportations_library::hcm::common::LevelOfService;
use transportations_library::hcm::twsc::pedestrian::PedestrianCrossing;
use transportations_library::hcm::twsc::twsc::{Mv, Twsc};

const DELAY_TOL: f64 = 0.5;
const CAPACITY_TOL: f64 = 5.0;

fn load(case: &str) -> Twsc {
    let path = format!("tests/ExampleCases/hcm/Twsc/{case}.json");
    let json = fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {path}: {e}"));
    Twsc::from_json(&json).unwrap_or_else(|e| panic!("parse {path}: {e}"))
}

/// Load one scenario out of the multi-scenario Example Problem 2 fixture.
fn load_pedestrian(scenario: &str) -> PedestrianCrossing {
    let path = "tests/ExampleCases/hcm/Twsc/case4_pedestrian.json";
    let json = fs::read_to_string(path).unwrap_or_else(|e| panic!("read {path}: {e}"));
    let root: serde_json::Value =
        serde_json::from_str(&json).unwrap_or_else(|e| panic!("parse {path}: {e}"));
    let node = root
        .get(scenario)
        .unwrap_or_else(|| panic!("{path} has no {scenario}"));
    serde_json::from_value(node.clone())
        .unwrap_or_else(|e| panic!("parse {scenario} of {path}: {e}"))
}

fn assert_close(value: f64, expected: f64, tol: f64, what: &str) {
    assert!(
        (value - expected).abs() <= tol,
        "{what}: got {value:.2}, expected {expected:.2} (tol {tol})"
    );
}

/// HCM Chapter 32, TWSC Example Problem 1 (three-leg intersection):
/// published answers c_m,4 = 1,238; c_m,9 = 760; c_m,7 = 268;
/// c_SH,NB = 521 veh/h; d_4 = 8.3 s (LOS A); d_NB = 14.9 s (LOS B);
/// d_A,WB = 2.9 s; d_I = 4.1 s; Q95,4 = 0.4; Q95,NB = 1.3 veh.
#[test]
fn test_twsc_example_problem_1_full_pipeline() {
    let mut twsc = load("case1");
    twsc.analyze();

    let m4 = &twsc.movements[Mv::M4.idx()];
    assert_close(m4.movement_capacity.unwrap(), 1238.0, CAPACITY_TOL, "c_m,4");
    let m9 = &twsc.movements[Mv::M9.idx()];
    assert_close(m9.movement_capacity.unwrap(), 760.0, CAPACITY_TOL, "c_m,9");
    let m7 = &twsc.movements[Mv::M7.idx()];
    assert_close(m7.movement_capacity.unwrap(), 268.0, CAPACITY_TOL, "c_m,7");

    // Shared northbound minor lane
    assert_eq!(twsc.lanes_nb.len(), 1);
    let nb = &twsc.lanes_nb[0];
    assert_close(nb.capacity, 521.0, CAPACITY_TOL, "c_SH,NB");
    assert_close(nb.control_delay, 14.9, DELAY_TOL, "d_SH,NB");
    assert_eq!(nb.los, 'B', "NB approach LOS");
    assert_close(nb.queue_95, 1.3, 0.2, "Q95,NB");

    // Major-street left turn
    assert_close(m4.control_delay.unwrap(), 8.3, DELAY_TOL, "d_4");
    assert_eq!(m4.los.unwrap(), 'A', "movement 4 LOS");
    assert_close(m4.queue_95.unwrap(), 0.4, 0.2, "Q95,4");

    // Approach and intersection delays (Equations 20-64 and 20-65)
    let [d_eb, d_wb, d_nb, _] = twsc.approach_delays.unwrap();
    assert_close(d_eb, 0.0, DELAY_TOL, "d_A,EB");
    assert_close(d_wb, 2.9, DELAY_TOL, "d_A,WB");
    assert_close(d_nb, 14.9, DELAY_TOL, "d_A,NB");
    assert_close(twsc.intersection_delay.unwrap(), 4.1, DELAY_TOL, "d_I");
}

/// HCM Chapter 32, TWSC Example Problem 3 (two-stage gap acceptance and
/// flared minor approaches): published answers c_T,8 = 390, c_T,11 = 405,
/// c_T,7 = 365, c_T,10 = 342, c_F,NB = 498, c_F,SB = 487 veh/h;
/// d_1 = 8.4 s (A), d_4 = 8.2 s (A), d_NB = 18.3 s (C), d_SB = 15.6 s (C);
/// d_I = 6.3 s; Q95: 0.1, 0.2, 2.4, 1.3 veh.
#[test]
fn test_twsc_example_problem_3_full_pipeline() {
    let mut twsc = load("case2");
    twsc.analyze();

    // Two-stage movement capacities
    assert_close(
        twsc.movements[Mv::M8.idx()].movement_capacity.unwrap(),
        390.0,
        CAPACITY_TOL,
        "c_T,8",
    );
    assert_close(
        twsc.movements[Mv::M11.idx()].movement_capacity.unwrap(),
        405.0,
        CAPACITY_TOL,
        "c_T,11",
    );
    assert_close(
        twsc.movements[Mv::M7.idx()].movement_capacity.unwrap(),
        365.0,
        CAPACITY_TOL,
        "c_T,7",
    );
    assert_close(
        twsc.movements[Mv::M10.idx()].movement_capacity.unwrap(),
        342.0,
        CAPACITY_TOL,
        "c_T,10",
    );

    // Flared-lane approach capacities (Equation 20-50)
    assert_close(twsc.lanes_nb[0].capacity, 498.0, CAPACITY_TOL, "c_F,NB");
    assert_close(twsc.lanes_sb[0].capacity, 487.0, CAPACITY_TOL, "c_F,SB");

    // Delay and LOS
    let m1 = &twsc.movements[Mv::M1.idx()];
    let m4 = &twsc.movements[Mv::M4.idx()];
    assert_close(m1.control_delay.unwrap(), 8.4, DELAY_TOL, "d_1");
    assert_close(m4.control_delay.unwrap(), 8.2, DELAY_TOL, "d_4");
    assert_eq!(m1.los.unwrap(), 'A');
    assert_eq!(m4.los.unwrap(), 'A');
    assert_close(twsc.lanes_nb[0].control_delay, 18.3, DELAY_TOL, "d_NB");
    assert_close(twsc.lanes_sb[0].control_delay, 15.6, DELAY_TOL, "d_SB");
    assert_eq!(twsc.lanes_nb[0].los, 'C', "NB LOS");
    assert_eq!(twsc.lanes_sb[0].los, 'C', "SB LOS");

    let [d_eb, d_wb, _, _] = twsc.approach_delays.unwrap();
    assert_close(d_eb, 0.8, DELAY_TOL, "d_A,EB");
    assert_close(d_wb, 1.2, DELAY_TOL, "d_A,WB");
    assert_close(twsc.intersection_delay.unwrap(), 6.3, DELAY_TOL, "d_I");

    // Queues
    assert_close(m1.queue_95.unwrap(), 0.1, 0.2, "Q95,1");
    assert_close(m4.queue_95.unwrap(), 0.2, 0.2, "Q95,4");
    assert_close(twsc.lanes_nb[0].queue_95, 2.4, 0.2, "Q95,NB");
    assert_close(twsc.lanes_sb[0].queue_95, 1.3, 0.2, "Q95,SB");
}

/// HCM Chapter 32, TWSC Example Problem 4 (TWSC between two coordinated
/// upstream signals; Step 5b platoon blockage, Equations 20-19 through
/// 20-21). Four-lane major street (N = 2), major-street left turns share the
/// through lane, no minor-street through movements, one stage. Published
/// p_b = 0.170 for movements 1/4/9/12 and 0.260 for movements 7/10
/// (Exhibit 32-12).
///
/// Published answers (Exhibit 32-13 problem text):
/// * conflicting flows v_c,1 = 1,086; v_c,4 = 1,076; v_c,9 = 538;
///   v_c,12 = 543; v_c,7 = 1,827; v_c,10 = 1,832 veh/h;
/// * unblocked conflicting flows v_c,u = 694, 682, 34, 40, 1,415, 1,422;
/// * potential (= movement, for 1/4/9/12) capacities c_p,1 = 750;
///   c_p,4 = 758; c_p,9 = 859; c_p,12 = 852; c_p,7 = 73; c_p,10 = 72 veh/h;
/// * movement capacities c_m,7 = 47; c_m,10 = 47 veh/h (f_p,7 = 0.647,
///   f_p,10 = 0.648);
/// * delays d_1 = 10.3 (B); d_4 = 10.3 (B); d_9 = 9.7 (A); d_12 = 9.8 (A);
///   d_7 = 529 (F); d_10 = 529 (F);
/// * approach delays d_A,NB = d_A,SB = 241 s; d_A,EB = d_A,WB = 1.9 s;
///   d_I = 34.1 s;
/// * queues Q95: 0.3, 0.3, 0.4, 0.4, 7.9, 7.9 veh.
///
/// The major-street left turns share the through lane (`major_left_eb` =
/// `major_left_wb` = `Shared`, n_L = 0), so Step 7d substitutes the
/// shared-major-lane queue-free probability p*_0,1+1U = p*_0,4+4U = 0.856
/// (Equations 20-33/20-34; x_2+3 = 0.304, x_5+6 = 0.307) for the exclusive-lane
/// p_0 = 0.900 in the Rank 4 impedance products, yielding c_m,7 = c_m,10 = 47
/// veh/h. Step 11b then charges the Rank 1 through/right movements a shared-lane
/// delay d_2+3 = d_5+6 = 1.3 s (Equations 20-62/20-63), which enters the EB/WB
/// approach delay in Step 12.
///
/// The two oversaturated minor-street left-turn delays (d_7, d_10, published
/// 529 s) and the minor-approach delays (d_A,NB, d_A,SB, published 241 s) use a
/// wider, documented tolerance: Equation 20-61 has slope |dd/dc| ~ 18.6 s per
/// veh/h near v/c = 1.7, and the book rounds c_m,7 = c_m,10 to the integer 47
/// while this library carries the full-precision 47.1 (NB) / 46.6 (SB), so the
/// per-movement delays split around 529 s. Every other published value is
/// asserted at +-1 s / +-1 veh/h, and d_I lands at 34.1 s because the NB
/// under-shoot and SB over-shoot cancel.
#[test]
fn test_twsc_example_problem_4_upstream_signals() {
    let mut twsc = load("case3");
    twsc.analyze();
    let m = |mv: Mv| twsc.movements[mv.idx()].clone();

    // Step 3 conflicting flows (no overrides; movements 7 and 10 reproduce
    // natively under the corrected Equations 20-14/20-15).
    assert_close(m(Mv::M1).conflicting_flow.unwrap(), 1086.0, 1.0, "v_c,1");
    assert_close(m(Mv::M4).conflicting_flow.unwrap(), 1076.0, 1.0, "v_c,4");
    assert_close(m(Mv::M9).conflicting_flow.unwrap(), 538.0, 1.0, "v_c,9");
    assert_close(m(Mv::M12).conflicting_flow.unwrap(), 543.0, 1.0, "v_c,12");
    assert_close(m(Mv::M7).conflicting_flow.unwrap(), 1827.0, 1.0, "v_c,7");
    assert_close(m(Mv::M10).conflicting_flow.unwrap(), 1832.0, 1.0, "v_c,10");

    // Step 5b potential capacities (Equations 20-19 through 20-21).
    assert_close(m(Mv::M1).potential_capacity.unwrap(), 750.0, CAPACITY_TOL, "c_p,1");
    assert_close(m(Mv::M4).potential_capacity.unwrap(), 758.0, CAPACITY_TOL, "c_p,4");
    assert_close(m(Mv::M9).potential_capacity.unwrap(), 859.0, CAPACITY_TOL, "c_p,9");
    assert_close(m(Mv::M12).potential_capacity.unwrap(), 852.0, CAPACITY_TOL, "c_p,12");
    assert_close(m(Mv::M7).potential_capacity.unwrap(), 73.0, CAPACITY_TOL, "c_p,7");
    assert_close(m(Mv::M10).potential_capacity.unwrap(), 72.0, CAPACITY_TOL, "c_p,10");

    // Movement capacities. 1/4/9/12 match published; 7/10 reproduce the
    // published 47 veh/h via the shared-major-lane p*_0 = 0.856 substitution
    // (Step 7d, Equations 20-33/20-34).
    assert_close(m(Mv::M1).movement_capacity.unwrap(), 750.0, CAPACITY_TOL, "c_m,1");
    assert_close(m(Mv::M4).movement_capacity.unwrap(), 758.0, CAPACITY_TOL, "c_m,4");
    assert_close(m(Mv::M9).movement_capacity.unwrap(), 859.0, CAPACITY_TOL, "c_m,9");
    assert_close(m(Mv::M12).movement_capacity.unwrap(), 852.0, CAPACITY_TOL, "c_m,12");
    assert_close(m(Mv::M7).movement_capacity.unwrap(), 47.0, 1.0, "c_m,7");
    assert_close(m(Mv::M10).movement_capacity.unwrap(), 47.0, 1.0, "c_m,10");

    // Step 11b Rank 1 delay to the shared-lane major-street through movements
    // (Equations 20-62/20-63): published d_2+3 = d_5+6 = 1.3 s.
    let [d23, d56] = twsc.rank1_major_delay.unwrap();
    assert_close(d23, 1.3, 0.1, "d_2+3 (Rank 1 EB)");
    assert_close(d56, 1.3, 0.1, "d_5+6 (Rank 1 WB)");

    // Delay and LOS (Equation 20-61, Exhibit 20-2). Major-street left turns
    // report at the movement level; minor-street movements report at the
    // lane level (each in an exclusive lane under the Separate config).
    assert_close(m(Mv::M1).control_delay.unwrap(), 10.3, DELAY_TOL, "d_1");
    assert_close(m(Mv::M4).control_delay.unwrap(), 10.3, DELAY_TOL, "d_4");
    assert_eq!(m(Mv::M1).los.unwrap(), 'B', "movement 1 LOS");
    assert_eq!(m(Mv::M4).los.unwrap(), 'B', "movement 4 LOS");

    let nb_lane = |mv: Mv| {
        twsc.lanes_nb
            .iter()
            .find(|l| l.movements.contains(&mv))
            .unwrap_or_else(|| panic!("NB lane for {mv:?}"))
    };
    let sb_lane = |mv: Mv| {
        twsc.lanes_sb
            .iter()
            .find(|l| l.movements.contains(&mv))
            .unwrap_or_else(|| panic!("SB lane for {mv:?}"))
    };
    let (nb_left, nb_right) = (nb_lane(Mv::M7), nb_lane(Mv::M9));
    let (sb_left, sb_right) = (sb_lane(Mv::M10), sb_lane(Mv::M12));
    assert_close(nb_right.control_delay, 9.7, DELAY_TOL, "d_9");
    assert_close(sb_right.control_delay, 9.8, DELAY_TOL, "d_12");
    assert_eq!(nb_right.los, 'A', "movement 9 LOS");
    assert_eq!(sb_right.los, 'A', "movement 12 LOS");
    // Oversaturated left turns: published d_7 = d_10 = 529 s (LOS F). Wide
    // tolerance per the doc comment (Equation 20-61 steep near v/c = 1.7; book
    // rounds c_m to 47).
    assert_close(nb_left.control_delay, 529.0, 12.0, "d_7");
    assert_close(sb_left.control_delay, 529.0, 12.0, "d_10");
    assert_eq!(nb_left.los, 'F', "NB left-turn lane LOS");
    assert_eq!(sb_left.los, 'F', "SB left-turn lane LOS");

    // Approach and intersection delay (Equations 20-64/20-65). The EB/WB
    // approaches carry the Step 11b Rank 1 delay on movements 2+3 / 5+6.
    let [d_eb, d_wb, d_nb, d_sb] = twsc.approach_delays.unwrap();
    assert_close(d_eb, 1.9, DELAY_TOL, "d_A,EB");
    assert_close(d_wb, 1.9, DELAY_TOL, "d_A,WB");
    // Published d_A,NB = d_A,SB = 241 s; wider tolerance per the doc comment.
    assert_close(d_nb, 241.0, 5.0, "d_A,NB");
    assert_close(d_sb, 241.0, 5.0, "d_A,SB");
    assert_close(twsc.intersection_delay.unwrap(), 34.1, DELAY_TOL, "d_I");

    // Step 13 queues (Equation 20-66).
    assert_close(m(Mv::M1).queue_95.unwrap(), 0.3, 0.2, "Q95,1");
    assert_close(nb_right.queue_95, 0.4, 0.2, "Q95,9");
    assert_close(nb_left.queue_95, 7.9, 0.5, "Q95,7");
}

/// Serde round-trip of a fully analyzed fixture (binding-layer contract).
#[test]
fn test_twsc_fixture_roundtrip() {
    let mut twsc = load("case1");
    twsc.analyze();
    let json = twsc.to_json().unwrap();
    let back = Twsc::from_json(&json).unwrap();
    assert_eq!(
        back.movements[Mv::M7.idx()].movement_capacity,
        twsc.movements[Mv::M7.idx()].movement_capacity
    );
}

// ═══════════════════════════════════════════════════════════════════════════════
// Example Problem 2: pedestrian mode (Chapter 20, Section 5)
// ═══════════════════════════════════════════════════════════════════════════════

/// HCM Chapter 32, TWSC Example Problem 2, Scenario A: 46-ft unmarked
/// single-stage crossing of four through lanes, 0% motorist yield rate.
/// Published answers: t_c = 12.5 s; P_b = 0.771; P_d = 0.997; d_g = 761 s;
/// d_gd = 763 s; d_p = 761 s; O(S/D) = 1.066 and 0.159 for the no-delay and
/// delay cases; P(D, no delay) = 48.4%; P(D, delay) = 86.3%; P(Y_1) = 0;
/// P_nd = 0.003; LOS F.
#[test]
fn test_twsc_example_problem_2_scenario_a() {
    let r = load_pedestrian("scenario_a").analyze();
    assert_eq!(r.stages.len(), 1, "Scenario A is a single-stage crossing");
    let s = &r.stages[0];

    // Steps 2 and 3 (Equations 20-76, 20-80, 20-81).
    assert_close(s.critical_headway, 12.5, 0.05, "t_c");
    // No platooning, so N_p = 1 row and t_c,G collapses to t_c (Equation 20-79).
    assert_close(s.spatial_distribution, 1.0, 1e-9, "N_p");
    assert_close(s.group_critical_headway, 12.5, 0.05, "t_c,G");
    assert_close(s.prob_blocked_lane, 0.771, 0.001, "P_b");
    assert_close(s.prob_delayed_crossing, 0.997, 0.001, "P_d");

    // Step 4 (Equations 20-82 and 20-83). Published to three significant
    // figures; Equation 20-82 is exponential in v * t_c,G.
    assert_close(s.gap_delay, 761.0, 761.0 * 0.005, "d_g");
    assert_close(s.gap_delay_when_delayed, 763.0, 763.0 * 0.005, "d_gd");

    // Step 5: with M_y = 0 every P(Y_i) is zero, so Equation 20-84 reduces to
    // P_d * d_gd = d_g. The book labels this "d_p,1 = d_gd = 761 s", which is a
    // mislabel of d_gd = 763 s; the value 761 s is what Equation 20-84 yields.
    assert!(
        s.prob_yield.iter().all(|p| *p == 0.0),
        "Scenario A has a 0% yield rate, so every P(Y_i) must be 0"
    );
    assert_close(r.delay, 761.0, 761.0 * 0.005, "d_p");

    // Step 7 (Equations 20-95 through 20-99, Exhibit 20-3).
    assert_close(r.odds_satisfied_no_delay, 1.066, 0.005, "O(S/D, no delay)");
    assert_close(r.prob_satisfied_no_delay, 0.516, 0.001, "P(S, no delay)");
    assert_close(r.prob_dissatisfied_no_delay, 0.484, 0.001, "P(D, no delay)");
    assert_close(r.odds_satisfied_delay, 0.159, 0.001, "O(S/D, delay)");
    assert_close(r.prob_satisfied_delay, 0.137, 0.001, "P(S, delay)");
    assert_close(r.prob_dissatisfied_delay, 0.863, 0.001, "P(D, delay)");
    assert_close(r.prob_yield_first_event, 0.0, 1e-9, "P(Y_1)");
    assert_close(r.prob_non_delayed, 0.003, 0.001, "P_nd");
    // Equation 20-99 with the published components: 0.003(0.484) + 0.997(0.863).
    assert_close(r.proportion_dissatisfied, 0.862, 0.005, "P_D");
    assert_eq!(r.los, LevelOfService::F, "Scenario A LOS");
}

/// HCM Chapter 32, TWSC Example Problem 2, Scenario B: two-stage crossing,
/// 20 ft and two through lanes per stage, marked crosswalk and median refuge,
/// 50% motorist yield rate. Published answers: t_c = 6.0 s; P_b = 0.508;
/// P_d = 0.758; d_g = 7.2 s; d_gd = 9.5 s; h = 2.3 s; n = 4; P(Y_1) = 0.314;
/// d_p,1 = d_p,2 = 3.0 s; d_p = 6.0 s. Exhibit 32-7: O(S/D) = 13.44 / 2.00,
/// P(D) = 6.9% / 33.4%, P_nd = 0.481, P(D) = 0.207, LOS C.
#[test]
fn test_twsc_example_problem_2_scenario_b() {
    let r = load_pedestrian("scenario_b").analyze();
    assert_eq!(r.stages.len(), 2, "Scenario B is a two-stage crossing");
    let s = &r.stages[0];

    assert_close(s.critical_headway, 6.0, 0.05, "t_c");
    assert_close(s.prob_blocked_lane, 0.508, 0.001, "P_b");
    assert_close(s.prob_delayed_crossing, 0.758, 0.001, "P_d");
    assert_close(s.gap_delay, 7.2, 0.05, "d_g");
    assert_close(s.gap_delay_when_delayed, 9.5, 0.05, "d_gd");

    // Step 5 (Equations 20-85, 20-84, and the two-lane Equation 20-89).
    assert_close(s.average_short_headway, 2.3, 0.05, "h");
    assert_eq!(s.yield_events, 4, "n = int(d_gd / h)");
    // The book prints the running cumulative sums inside the P(Y_i) brackets:
    // 0, 0.314, 0.498, 0.606.
    assert_close(s.prob_yield[1], 0.314, 0.001, "P(Y_1)");
    let cum2: f64 = s.prob_yield[1] + s.prob_yield[2];
    assert_close(cum2, 0.498, 0.001, "P(Y_1) + P(Y_2)");
    let cum3: f64 = cum2 + s.prob_yield[3];
    assert_close(cum3, 0.606, 0.001, "P(Y_1..3) cumulative");

    // Both stages are identical by construction, so d_p,2 = d_p,1 = 3.0 s.
    assert_close(s.delay, 3.0, DELAY_TOL, "d_p,1");
    assert_close(r.stages[1].delay, 3.0, DELAY_TOL, "d_p,2");
    // Step 6 (Equation 20-94).
    assert_close(r.delay, 6.0, DELAY_TOL, "d_p");

    // Step 7, Exhibit 32-7.
    assert_close(r.odds_satisfied_no_delay, 13.44, 0.05, "O(S/D, no delay)");
    assert_close(r.prob_satisfied_no_delay, 0.931, 0.001, "P(S, no delay)");
    assert_close(r.prob_dissatisfied_no_delay, 0.069, 0.001, "P(D, no delay)");
    assert_close(r.odds_satisfied_delay, 2.00, 0.01, "O(S/D, delay)");
    assert_close(r.prob_satisfied_delay, 0.666, 0.001, "P(S, delay)");
    assert_close(r.prob_dissatisfied_delay, 0.334, 0.001, "P(D, delay)");
    assert_close(r.prob_yield_first_event, 0.314, 0.001, "P(Y_1)");
    assert_close(r.prob_non_delayed, 0.481, 0.001, "P_nd");
    assert_close(r.proportion_dissatisfied, 0.207, 0.001, "P_D");
    assert_eq!(r.los, LevelOfService::C, "Scenario B LOS");
}

/// HCM Chapter 32, TWSC Example Problem 2, Scenario C: Scenario B plus RRFBs
/// and an 80% motorist yield rate. Published answers: P(Y_1) = 0.565;
/// d_p,1 = d_p,2 = 1.5 s; d_p = 3.0 s. Exhibit 32-7: O(S/D) = 95.15 / 14.15,
/// P(D) = 1.0% / 6.6%, P_nd = 0.670, P(D) = 0.029, LOS A.
#[test]
fn test_twsc_example_problem_2_scenario_c() {
    let r = load_pedestrian("scenario_c").analyze();
    assert_eq!(r.stages.len(), 2, "Scenario C is a two-stage crossing");
    let s = &r.stages[0];

    // Only the yield rate changes from Scenario B, so Steps 2-4 are unchanged.
    assert_close(s.prob_delayed_crossing, 0.758, 0.001, "P_d");
    assert_close(s.average_short_headway, 2.3, 0.05, "h");
    assert_eq!(s.yield_events, 4, "n = int(d_gd / h)");

    // The book's cumulative sums for Scenario C: 0, 0.565, 0.709, 0.746.
    assert_close(s.prob_yield[1], 0.565, 0.001, "P(Y_1)");
    let cum2: f64 = s.prob_yield[1] + s.prob_yield[2];
    assert_close(cum2, 0.709, 0.001, "P(Y_1) + P(Y_2)");
    let cum3: f64 = cum2 + s.prob_yield[3];
    assert_close(cum3, 0.746, 0.001, "P(Y_1..3) cumulative");

    assert_close(s.delay, 1.5, DELAY_TOL, "d_p,1");
    assert_close(r.delay, 3.0, DELAY_TOL, "d_p");

    assert_close(r.odds_satisfied_no_delay, 95.15, 0.15, "O(S/D, no delay)");
    assert_close(r.prob_satisfied_no_delay, 0.990, 0.001, "P(S, no delay)");
    assert_close(r.prob_dissatisfied_no_delay, 0.010, 0.001, "P(D, no delay)");
    assert_close(r.odds_satisfied_delay, 14.15, 0.05, "O(S/D, delay)");
    assert_close(r.prob_satisfied_delay, 0.934, 0.001, "P(S, delay)");
    assert_close(r.prob_dissatisfied_delay, 0.066, 0.001, "P(D, delay)");
    assert_close(r.prob_yield_first_event, 0.565, 0.001, "P(Y_1)");
    assert_close(r.prob_non_delayed, 0.670, 0.001, "P_nd");
    assert_close(r.proportion_dissatisfied, 0.029, 0.001, "P_D");
    assert_eq!(r.los, LevelOfService::A, "Scenario C LOS");
}

/// The three scenarios reproduce the Example Problem 2 discussion: adding a
/// marked crosswalk and median refuge moves the crossing from LOS F to C, and
/// adding RRFBs moves it to A.
#[test]
fn test_twsc_example_problem_2_countermeasure_progression() {
    let a = load_pedestrian("scenario_a").analyze();
    let b = load_pedestrian("scenario_b").analyze();
    let c = load_pedestrian("scenario_c").analyze();
    assert!(
        a.proportion_dissatisfied > b.proportion_dissatisfied
            && b.proportion_dissatisfied > c.proportion_dissatisfied,
        "P_D must fall monotonically as countermeasures are added: {} -> {} -> {}",
        a.proportion_dissatisfied,
        b.proportion_dissatisfied,
        c.proportion_dissatisfied
    );
    assert!(a.delay > b.delay && b.delay > c.delay, "d_p must fall too");
}

/// Serde round-trip of the pedestrian crossing config (binding-layer contract).
#[test]
fn test_twsc_pedestrian_roundtrip() {
    let crossing = load_pedestrian("scenario_b");
    let json = crossing.to_json().unwrap();
    let back = PedestrianCrossing::from_json(&json).unwrap();
    assert_close(
        back.analyze().proportion_dissatisfied,
        crossing.analyze().proportion_dissatisfied,
        1e-12,
        "P_D after round-trip",
    );
}
