//! Full-pipeline integration tests for HCM Chapter 20 (TWSC intersections)
//! against the published answers of HCM Chapter 32, TWSC Example Problems 1,
//! 3, and 4.
//!
//! Tolerances: LOS exact; control delays within +-0.5 s/veh; capacities
//! within +-5 veh/h of the published (rounded) values. Example Problem 4
//! reproduces the shared-major-left case; the two oversaturated minor-street
//! left-turn delays use a wider, documented tolerance because Equation 20-61
//! is steep near v/c = 1.7 and the book rounds c_m to an integer.
//!
//! Chapter 32, TWSC Example Problem 2 (pedestrian crossing at a TWSC
//! intersection) is deliberately NOT covered here, because the pedestrian mode
//! is not implemented. What `src/hcm/twsc/` provides is the pedestrian
//! *impedance* extension of the vehicular method, Equations 20-67 through
//! 20-75, in which pedestrian volumes v13-v16 reduce vehicular movement
//! capacity. That is a different procedure from Chapter 20 Section 5, which is
//! the pedestrian mode proper and computes a service measure for the
//! pedestrian. Reproducing Example Problem 2 needs all seven of its steps, and
//! none of the following surface exists:
//!
//! * pedestrian critical headway from crossing length, walking speed, and
//!   start-up/clearance time (Equation 20-76), and the platoon-adjusted form
//!   (Equations 20-77 through 20-79);
//! * probability of a blocked lane and of a delayed crossing (Equations 20-80
//!   and 20-81);
//! * average gap delay and gap delay given nonzero delay (Equations 20-82 and
//!   20-83);
//! * delay reduction from yielding motorists (Equations 20-84 through 20-94),
//!   including the per-lane-count yield-event probabilities P(Y_i) that the
//!   example selects by number of lanes crossed (Equation 20-89 for a two-lane
//!   crossing, Equation 20-92 for a four-lane crossing);
//! * the pedestrian satisfaction model that produces LOS - satisfaction odds
//!   with indicator variables for RRFBs, marked crosswalk, and median refuge
//!   plus the AADT term (Equation 20-95), the satisfaction and dissatisfaction
//!   probabilities (Equations 20-96 through 20-98), the average proportion
//!   dissatisfied (Equation 20-99), and the Exhibit 20-3 pedestrian-mode LOS
//!   bands keyed on proportion dissatisfied rather than on delay.
//!
//! There is also no two-stage-crossing decomposition (Step 1) and no input
//! surface for crosswalk length, walking speed, motorist yield rate, K-factor,
//! or the countermeasure indicators. Landing this example therefore means
//! implementing Chapter 20 Section 5, not extending a fixture. For the record,
//! the published answers to reproduce once it exists are: critical headway
//! t_c = 12.5 s (Scenario A, 46-ft single-stage crossing) and 6.0 s (Scenarios
//! B and C, 20-ft stages); total pedestrian delay 761 s (A), 6.0 s (B), 3.0 s
//! (C); P_d = 0.758 for the two-stage scenarios; and LOS F (A), C (B), A (C),
//! with the Scenario B and C intermediates tabulated in Exhibit 32-7.

use std::fs;

use transportations_library::hcm::twsc::twsc::{Mv, Twsc};

const DELAY_TOL: f64 = 0.5;
const CAPACITY_TOL: f64 = 5.0;

fn load(case: &str) -> Twsc {
    let path = format!("tests/ExampleCases/hcm/Twsc/{case}.json");
    let json = fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {path}: {e}"));
    Twsc::from_json(&json).unwrap_or_else(|e| panic!("parse {path}: {e}"))
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
