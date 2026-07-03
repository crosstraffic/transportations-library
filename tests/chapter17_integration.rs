//! Integration tests for the HCM Chapter 17 (Urban Street Reliability and
//! ATDM) core methodology, against HCM 7th Edition, Chapter 29, Section 5,
//! Example Problems 4 and 5 (Exhibits 29-62 through 29-80).
//!
//! Fixture `case1.json` reproduces the Example Problem 4 configuration
//! (3-mi Lincoln, Nebraska principal arterial; weekdays for one year;
//! 7-10 a.m. study period). Deterministic quantities (scenario count,
//! demand ratios, base free-flow travel time band) are asserted exactly
//! or tightly; the Monte Carlo reliability measures are asserted at the
//! distribution-band level around the published Exhibit 29-73 values
//! (mean TTI 1.69/1.64, TTI-80 1.57/1.56, PTI 2.98/2.61, reliability
//! rating 93.2/94.1) because — per the HCM itself — "evaluating the same
//! dataset and seed number in different software or on a different
//! platform may produce results different from those shown here. Each
//! result, though different, will be equally valid."

use transportations_library::hcm::chapter17::{AtdmStrategy, UrbanReliability};

fn load_case(name: &str) -> UrbanReliability {
    let path = format!(
        "{}/tests/ExampleCases/hcm/UrbanReliability/{}",
        env!("CARGO_MANIFEST_DIR"),
        name
    );
    let json = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {path}: {e}"));
    UrbanReliability::from_json(&json).unwrap_or_else(|e| panic!("parse {path}: {e}"))
}

/// Example Problem 4: existing urban street reliability.
#[test]
fn test_case1_example_problem_4() {
    let mut analysis = load_case("case1.json");
    let results = analysis.run().unwrap().clone();

    // Published, deterministic: 3,120 scenarios (= 12 analysis periods ×
    // 260 weekdays).
    assert_eq!(results.num_scenarios, 3_120, "published scenario count");

    // Published: base free-flow travel time 262.9 s (the fixture's
    // reconstructed geometry gives the same magnitude; ±10 s band).
    assert!(
        (results.base_free_flow_travel_time_s - 262.9).abs() < 10.0,
        "base free-flow travel time {} vs published 262.9 s",
        results.base_free_flow_travel_time_s
    );

    // Distribution-level bands around Exhibit 29-73 (EB/WB averages).
    let m = &results.metrics;
    assert!(m.tti_mean >= 1.0, "mean TTI >= 1: {}", m.tti_mean);
    assert!(
        (1.1..=2.6).contains(&m.tti_mean),
        "mean TTI {} vs published 1.69/1.64",
        m.tti_mean
    );
    assert!(
        (1.1..=2.6).contains(&m.tti_80),
        "TTI-80 {} vs published 1.57/1.56",
        m.tti_80
    );
    assert!(
        (1.3..=5.0).contains(&m.tti_95),
        "PTI {} vs published 2.98/2.61",
        m.tti_95
    );
    assert!(m.tti_50 <= m.tti_80 && m.tti_80 <= m.tti_95, "percentile ordering");
    assert!(
        (70.0..=100.0).contains(&results.reliability_rating_urban),
        "reliability rating {} vs published 93.2/94.1",
        results.reliability_rating_urban
    );
    assert!(results.total_vhd > 0.0, "positive annual through delay");
    assert!(results.num_weather_events > 50, "weather events generated");
    assert!(results.num_incidents > 50, "incidents generated");

    // Deterministic reproducibility with the published seed pattern
    // (82/11/63).
    let mut again = load_case("case1.json");
    let r2 = again.run().unwrap();
    assert_eq!(r2.num_incidents, results.num_incidents, "seeded incident stream");
    assert!(
        (r2.metrics.tti_mean - m.tti_mean).abs() < 1e-12,
        "seeded mean TTI reproducible"
    );
    assert!(
        (r2.metrics.tti_95 - m.tti_95).abs() < 1e-12,
        "seeded PTI reproducible"
    );
}

/// Different seeds (Example Problem 4's replication concept, Exhibit
/// 29-75) give a different but valid stream: measures move, bands hold.
#[test]
fn test_case1_replication_with_different_seeds() {
    let mut rep1 = load_case("case1.json");
    let r1 = rep1.run().unwrap().clone();

    let mut rep2 = load_case("case1.json");
    rep2.config.weather_seed = 83;
    rep2.config.demand_seed = 12;
    rep2.config.incident_seed = 64;
    let r2 = rep2.run().unwrap().clone();

    assert_eq!(r1.num_scenarios, r2.num_scenarios, "same RRP");
    // Published Exhibit 29-75: average travel time varied by ~±1.4%
    // across replications; allow a generous band here.
    let rel_diff = (r1.mean_travel_time_s - r2.mean_travel_time_s).abs()
        / r1.mean_travel_time_s;
    assert!(
        rel_diff < 0.10,
        "replications should agree within 10% (got {:.1}%)",
        100.0 * rel_diff
    );
    assert!(
        (1.1..=2.6).contains(&r2.metrics.tti_mean),
        "replication mean TTI {}",
        r2.metrics.tti_mean
    );
}

/// Example Problem 5: strategy evaluation. Strategy 1 (shift 5 s of
/// split to the coordinated through phase) must improve mean travel time
/// and the reliability rating, mirroring the published direction of
/// effect (Exhibit 29-78: travel time 438.2 → 400.7 s, rating 93.2 →
/// 96.8).
#[test]
fn test_case1_example_problem_5_strategy_1() {
    let mut existing = load_case("case1.json");
    let base = existing.run().unwrap().clone();

    let mut with_strategy = load_case("case1.json");
    with_strategy.atdm_strategies.push(AtdmStrategy {
        name: "EP5 Strategy 1: +5 s to the coordinated phase".into(),
        effective_green_adjustment_s: 5.0,
        ..AtdmStrategy::default()
    });
    let strat = with_strategy.run().unwrap().clone();

    assert!(
        strat.mean_travel_time_s < base.mean_travel_time_s,
        "strategy must reduce mean travel time ({:.1} vs {:.1} s)",
        strat.mean_travel_time_s,
        base.mean_travel_time_s
    );
    assert!(
        strat.metrics.tti_95 <= base.metrics.tti_95,
        "strategy must not degrade the PTI"
    );
    assert!(
        strat.reliability_rating_urban >= base.reliability_rating_urban,
        "strategy must not degrade the reliability rating ({:.1} vs {:.1})",
        strat.reliability_rating_urban,
        base.reliability_rating_urban
    );
}
