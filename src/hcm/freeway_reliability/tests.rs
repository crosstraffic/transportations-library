//! Unit tests for HCM Chapter 11 scenario generation and the reliability
//! wrapper (exhibit lookups are tested in `exhibits.rs`; distribution
//! metrics in `common::reliability`).

use super::exhibits::*;
use super::reliability::ReliabilityAnalysis;
use super::scenario_generation::*;

use crate::hcm::freeway_facilities::freeway_facilities::{FacilitySegment, FreewayFacility, SegmentType};

// ═════════════════════════════════════════════════════════════════════════
// Helpers
// ═════════════════════════════════════════════════════════════════════════

/// Small three-segment facility with four analysis periods.
fn small_facility() -> FreewayFacility {
    FreewayFacility {
        segments: vec![
            FacilitySegment {
                seg_type: SegmentType::Basic,
                length_ft: 2640.0,
                lanes: 3,
                ..Default::default()
            },
            FacilitySegment {
                seg_type: SegmentType::Merge,
                length_ft: 1500.0,
                lanes: 3,
                on_ramp_demand: vec![300.0, 400.0, 400.0, 300.0],
                ..Default::default()
            },
            FacilitySegment {
                seg_type: SegmentType::Basic,
                length_ft: 5280.0,
                lanes: 3,
                ..Default::default()
            },
        ],
        mainline_demand: vec![4000.0, 4400.0, 4600.0, 4000.0],
        ffs: 60.0,
        heavy_vehicle_pct: 0.05,
        ..Default::default()
    }
}

fn seed_stats(fac: &FreewayFacility) -> SeedStatistics {
    ReliabilityAnalysis::new(fac.clone(), ScenarioGenerationConfig::default())
        .seed_statistics()
}

// ═════════════════════════════════════════════════════════════════════════
// PRNG
// ═════════════════════════════════════════════════════════════════════════

#[test]
fn test_prng_reproducible_and_in_range() {
    let mut a = Prng::new(42);
    let mut b = Prng::new(42);
    for _ in 0..100 {
        assert_eq!(a.next_u64(), b.next_u64());
    }
    let mut r = Prng::new(7);
    for _ in 0..1000 {
        let x = r.next_f64();
        assert!((0.0..1.0).contains(&x));
        let k = r.gen_range(5);
        assert!(k < 5);
    }
    // Weighted pick: index 1 has all the weight.
    let mut r = Prng::new(3);
    for _ in 0..50 {
        assert_eq!(r.pick_weighted(&[0.0, 1.0, 0.0]), 1);
    }
}

#[test]
fn test_prng_rough_uniformity() {
    let mut r = Prng::new(123);
    let mut counts = [0usize; 4];
    for _ in 0..4000 {
        counts[r.gen_range(4)] += 1;
    }
    for c in counts {
        assert!((800..1200).contains(&c), "counts {counts:?}");
    }
}

// ═════════════════════════════════════════════════════════════════════════
// Deterministic count generation
// ═════════════════════════════════════════════════════════════════════════

#[test]
fn test_counts_matching_distribution_sums() {
    // Equation 25-83/25-84 with the default G(i) over 100 incidents.
    let counts = counts_matching_distribution(&DEFAULT_INCIDENT_SEVERITY_DISTRIBUTION, 100);
    assert_eq!(counts.iter().sum::<usize>(), 100);
    // Marginals track the distribution: 75/76 shoulders, ~20 one-lane.
    assert!((74..=77).contains(&counts[0]), "{counts:?}");
    assert!((18..=21).contains(&counts[1]), "{counts:?}");
    assert_eq!(counts[4], 0, "4+ lane closures have zero probability");

    // Small-n edge cases.
    let counts = counts_matching_distribution(&[0.5, 0.5], 1);
    assert_eq!(counts.iter().sum::<usize>(), 1);
    let counts = counts_matching_distribution(&[1.0], 5);
    assert_eq!(counts, vec![5]);
    let counts = counts_matching_distribution(&[0.2; 5], 0);
    assert_eq!(counts.iter().sum::<usize>(), 0);
}

#[test]
fn test_poisson_counts_match_example() {
    // Equations 25-80/25-81: 20 scenarios, mean 0.65 incidents/study
    // period. Poisson pmf: P(0)=0.522, P(1)=0.339, P(2)=0.110, P(3)=0.024.
    let pmf = poisson_pmf(0.65, 8);
    assert!((pmf[0] - 0.5220).abs() < 0.001);
    assert!((pmf[1] - 0.3393).abs() < 0.001);
    let counts = counts_matching_distribution(&pmf, 20);
    assert_eq!(counts.iter().sum::<usize>(), 20);
    // The bulk must sit at k=0 and k=1.
    assert!(counts[0] >= 10, "{counts:?}");
    assert!(counts[1] >= 6, "{counts:?}");
}

#[test]
fn test_equation_25_76_expected_weather_frequency() {
    // Chapter 25 Step 11 worked example: 5-h study period, probability
    // 0.10, 20 scenarios, mean duration 1 h => 10 events.
    assert_eq!(expected_weather_frequency(0.10, 5.0, 20, 60.0), 10);
    // Zero probability or duration => no events.
    assert_eq!(expected_weather_frequency(0.0, 5.0, 20, 60.0), 0);
    assert_eq!(expected_weather_frequency(0.10, 5.0, 20, 0.0), 0);
    // Example Problem 7, winter medium rain: P=0.0080, D_SP=3 h, 20
    // scenarios, mean duration 40.2 min -> E15 = 0.75 h => round(0.64) = 1.
    assert_eq!(expected_weather_frequency(0.0080, 3.0, 20, 40.2), 1);
    // EP7 winter light snow: P=0.0091, 93.1 min -> 1.5 h => round(0.364)=0.
    assert_eq!(expected_weather_frequency(0.0091, 3.0, 20, 93.1), 0);
    // EP7 summer heavy rain: P=0.0133, 33.7 min -> 0.5 h => round(1.596)=2.
    assert_eq!(expected_weather_frequency(0.0133, 3.0, 20, 33.7), 2);
}

#[test]
fn test_incident_duration_bins() {
    // Shoulder incidents: mean 34.0 min, sd 15.1, range 8.7-58 min =>
    // bins at 15/30/45/60 min (1..=4 analysis periods).
    let (bins, probs) = incident_duration_bins(&DEFAULT_INCIDENT_DURATION_PARAMS[0]);
    assert_eq!(bins.first(), Some(&1));
    assert_eq!(bins.last(), Some(&4));
    assert!((probs.iter().sum::<f64>() - 1.0).abs() < 1e-9);
    // The 30-min bin should carry the largest share for a 34-min mean.
    let max_idx = probs
        .iter()
        .enumerate()
        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
        .unwrap()
        .0;
    assert_eq!(bins[max_idx], 2, "probs {probs:?}");
}

// ═════════════════════════════════════════════════════════════════════════
// Scenario generation
// ═════════════════════════════════════════════════════════════════════════

#[test]
fn test_default_scenario_count_and_probabilities() {
    // 12 months x 5 weekdays x 4 replications = 240 scenarios (Equation
    // 25-71), each with probability ~1/240 (Equation 25-73).
    let fac = small_facility();
    let cfg = ScenarioGenerationConfig::default();
    let set = generate_scenarios(&cfg, &seed_stats(&fac)).unwrap();
    assert_eq!(set.scenarios.len(), 240);
    let total: f64 = set.scenarios.iter().map(|s| s.probability).sum();
    assert!((total - 1.0).abs() < 1e-9, "probabilities sum to {total}");
    for s in &set.scenarios {
        assert!((s.probability - 1.0 / 240.0).abs() < 1e-12);
    }
}

#[test]
fn test_scenario_probabilities_with_day_counts() {
    // Uneven day counts (Equation 25-73): a DC with 5 days is 25% more
    // likely than one with 4.
    let fac = small_facility();
    let cfg = ScenarioGenerationConfig {
        months: vec![1],
        weekdays: vec![Weekday::Monday, Weekday::Tuesday],
        replications: 2,
        day_counts: Some(vec![5.0, 4.0]),
        ..Default::default()
    };
    let set = generate_scenarios(&cfg, &seed_stats(&fac)).unwrap();
    assert_eq!(set.scenarios.len(), 4);
    let total: f64 = set.scenarios.iter().map(|s| s.probability).sum();
    assert!((total - 1.0).abs() < 1e-12);
    let p_mon = set.scenarios[0].probability;
    let p_tue = set.scenarios[2].probability;
    assert!((p_mon / p_tue - 1.25).abs() < 1e-9);
}

#[test]
fn test_demand_multipliers_and_daf() {
    // Seed = Monday January (urban default DM = 1.00): July Friday
    // scenario gets DAF = 1.62 (Exhibit 11-18) per Equation 25-72.
    let fac = small_facility();
    let cfg = ScenarioGenerationConfig::default();
    let set = generate_scenarios(&cfg, &seed_stats(&fac)).unwrap();
    let jul_fri = set
        .scenarios
        .iter()
        .find(|s| s.month == 7 && s.weekday == Weekday::Friday)
        .unwrap();
    assert!((jul_fri.demand_multiplier - 1.62).abs() < 1e-12);
    assert!((jul_fri.daf - 1.62).abs() < 1e-12);
    let jan_mon = set
        .scenarios
        .iter()
        .find(|s| s.month == 1 && s.weekday == Weekday::Monday)
        .unwrap();
    assert!((jan_mon.daf - 1.0).abs() < 1e-12);
}

#[test]
fn test_weather_generation_counts_and_no_overlap() {
    let fac = small_facility();
    // Heavy rain with a 30% timewise probability year-round, 30-min mean
    // duration: E[n] = round(0.3 x 1 h x 20 / 0.5) = 12 events per month.
    let mut weather = WeatherInputs::default();
    for m in 0..12 {
        weather.probabilities_by_month[m][1] = 0.30; // heavy rain
    }
    weather.durations_min[1] = 30.0;
    let cfg = ScenarioGenerationConfig {
        weather: Some(weather),
        rng_seed: 11,
        ..Default::default()
    };
    let set = generate_scenarios(&cfg, &seed_stats(&fac)).unwrap();
    for m in 0..12 {
        assert_eq!(set.expected_weather_events[m][1], 12, "month {}", m + 1);
    }
    assert_eq!(set.total_weather_events, 12 * 12);
    // Assigned durations are 2 analysis periods; no temporal overlap
    // within any scenario.
    for sc in &set.scenarios {
        for (a, ev_a) in sc.weather_events.iter().enumerate() {
            assert_eq!(ev_a.duration_periods, 2);
            assert!((ev_a.caf - weather_caf(WeatherType::HeavyRain, 60.0)).abs() < 1e-12);
            assert!((ev_a.saf - weather_saf(WeatherType::HeavyRain, 60.0)).abs() < 1e-12);
            for ev_b in sc.weather_events.iter().skip(a + 1) {
                let end_a = ev_a.start_period + ev_a.duration_periods;
                let end_b = ev_b.start_period + ev_b.duration_periods;
                assert!(
                    ev_a.start_period >= end_b.min(4) || ev_b.start_period >= end_a.min(4),
                    "overlapping weather events in scenario {}",
                    sc.id
                );
            }
        }
    }
}

#[test]
fn test_incident_generation_counts_severities_and_feasibility() {
    let fac = small_facility();
    let cfg = ScenarioGenerationConfig {
        incidents: Some(IncidentInputs {
            monthly_frequencies: Some(vec![1.0; 12]),
            ..Default::default()
        }),
        rng_seed: 5,
        ..Default::default()
    };
    let set = generate_scenarios(&cfg, &seed_stats(&fac)).unwrap();
    // Poisson mean 1.0 over 20 scenarios/month: counts distribution sums
    // to 20 per month; expected total incidents near 240.
    assert!(set.total_incidents > 150 && set.total_incidents < 300,
        "total incidents {}", set.total_incidents);
    let assigned: usize = set.scenarios.iter().map(|s| s.incidents.len()).sum();
    assert_eq!(assigned, set.total_incidents);
    // All incidents feasible (facility has 3-lane segments: at most a
    // 2-lane closure) and within the study period.
    for sc in &set.scenarios {
        for inc in &sc.incidents {
            assert!(inc.segment < 3);
            assert!(inc.start_period < 4);
            assert!(inc.duration_periods >= 1);
            assert!(inc.severity.lanes_closed() < 3);
            assert!(incident_caf_total(3, inc.severity).is_some());
        }
    }
    // Severity marginals follow G(i): mostly shoulder closures.
    let shoulders = set
        .scenarios
        .iter()
        .flat_map(|s| &s.incidents)
        .filter(|i| i.severity == IncidentSeverity::Shoulder)
        .count();
    let frac = shoulders as f64 / set.total_incidents as f64;
    assert!((0.70..=0.80).contains(&frac), "shoulder fraction {frac}");
}

#[test]
fn test_incident_frequency_from_crash_rate() {
    // Equations 25-77/25-78: n_j = CR x ICR x VMT_j / 1e8.
    let fac = small_facility();
    let stats = seed_stats(&fac);
    let cfg = ScenarioGenerationConfig {
        months: vec![1],
        seed_month: 1,
        seed_weekday: Weekday::Monday,
        incidents: Some(IncidentInputs {
            crash_rate_per_100mvmt: Some(150.0),
            incident_to_crash_ratio: 7.0,
            ..Default::default()
        }),
        ..Default::default()
    };
    let set = generate_scenarios(&cfg, &stats).unwrap();
    // January weekday urban DMs: 1.00/1.00/1.02/1.05/1.17, mean 1.048;
    // seed DM = 1.00 (Jan Monday).
    let mean_daf = (1.00 + 1.00 + 1.02 + 1.05 + 1.17) / 5.0;
    let expected = 150.0 * 7.0 * stats.total_vmt() * mean_daf / 1e8;
    assert!(
        (set.monthly_incident_frequency[0] - expected).abs() < 1e-9,
        "n_1 = {} vs {}",
        set.monthly_incident_frequency[0],
        expected
    );
}

#[test]
fn test_work_zone_and_special_event_assignment() {
    let fac = small_facility();
    let cfg = ScenarioGenerationConfig {
        work_zones: vec![WorkZoneEvent {
            months: vec![6, 7],
            weekdays: vec![Weekday::Monday],
            active_day_ratio: 0.5, // round(0.5 x 4) = 2 replications
            segments: vec![2],
            caf: 0.8,
            ..Default::default()
        }],
        special_events: vec![SpecialEvent {
            month: 9,
            weekday: Weekday::Friday,
            replication: 1,
            daf: 1.3,
            ..Default::default()
        }],
        ..Default::default()
    };
    let set = generate_scenarios(&cfg, &seed_stats(&fac)).unwrap();
    let wz_scenarios: Vec<_> = set
        .scenarios
        .iter()
        .filter(|s| !s.work_zones.is_empty())
        .collect();
    // 2 months x 1 weekday x 2 replications = 4 scenarios.
    assert_eq!(wz_scenarios.len(), 4);
    for s in &wz_scenarios {
        assert!(s.month == 6 || s.month == 7);
        assert_eq!(s.weekday, Weekday::Monday);
        assert!(s.replication < 2);
    }
    let se_scenarios: Vec<_> = set
        .scenarios
        .iter()
        .filter(|s| !s.special_events.is_empty())
        .collect();
    assert_eq!(se_scenarios.len(), 1);
    assert_eq!(se_scenarios[0].month, 9);
    assert_eq!(se_scenarios[0].replication, 1);
}

#[test]
fn test_generation_reproducible_for_seed() {
    let fac = small_facility();
    let mk = |seed| {
        let cfg = ScenarioGenerationConfig {
            weather: Some({
                let mut w = WeatherInputs::default();
                for m in 0..12 {
                    w.probabilities_by_month[m][0] = 0.05;
                }
                w.durations_min[0] = 40.0;
                w
            }),
            incidents: Some(IncidentInputs {
                monthly_frequencies: Some(vec![0.7; 12]),
                ..Default::default()
            }),
            rng_seed: seed,
            ..Default::default()
        };
        generate_scenarios(&cfg, &seed_stats(&fac)).unwrap()
    };
    let a = mk(99);
    let b = mk(99);
    let c = mk(100);
    assert_eq!(
        serde_json::to_string(&a).unwrap(),
        serde_json::to_string(&b).unwrap(),
        "same seed must reproduce identical scenario sets"
    );
    assert_ne!(
        serde_json::to_string(&a).unwrap(),
        serde_json::to_string(&c).unwrap(),
        "different seeds should differ in event assignment"
    );
}

// ═════════════════════════════════════════════════════════════════════════
// Reliability wrapper
// ═════════════════════════════════════════════════════════════════════════

#[test]
fn test_base_scenario_matches_chapter10() {
    // With a single demand combination, no events, and the seed date equal
    // to the scenario date, the reliability engine must reproduce the
    // Chapter 10 base results exactly (verification requirement (b)).
    let mut base = small_facility();
    base.run_analysis().unwrap();

    let cfg = ScenarioGenerationConfig {
        months: vec![1],
        weekdays: vec![Weekday::Monday],
        replications: 1,
        seed_month: 1,
        seed_weekday: Weekday::Monday,
        ..Default::default()
    };
    let mut rel = ReliabilityAnalysis::new(small_facility(), cfg);
    rel.run().unwrap();
    assert_eq!(rel.scenario_results.len(), 1);
    let sc = &rel.scenario_results[0];
    assert!((sc.probability - 1.0).abs() < 1e-12);
    for p in 0..base.num_periods() {
        let tt_base: f64 = base
            .segments
            .iter()
            .enumerate()
            .map(|(i, s)| s.length_mi() / base.speed[i][p] * 60.0)
            .sum();
        assert!(
            (sc.travel_time_min[p] - tt_base).abs() < 1e-9,
            "period {p}: {} vs {}",
            sc.travel_time_min[p],
            tt_base
        );
        assert!(
            (sc.vmt[p] - base.facility_performance[p].vmt_served).abs() < 1e-9,
            "VMT period {p}"
        );
    }
    // TTI >= 1 and finite metrics.
    let m = rel.metrics.as_ref().unwrap();
    assert!(m.tti_mean >= 1.0);
    assert!(m.tti_95 >= m.tti_50);
}

#[test]
fn test_scenario_facility_daf_and_caf_folding() {
    let fac = small_facility();
    let cfg = ScenarioGenerationConfig::default();
    let rel = ReliabilityAnalysis::new(fac, cfg);
    let sc = FreewayScenario {
        id: 0,
        month: 7,
        weekday: Weekday::Friday,
        replication: 0,
        probability: 1.0,
        demand_multiplier: 1.62,
        daf: 1.62,
        weather_events: vec![WeatherEventAssignment {
            weather: WeatherType::HeavyRain,
            start_period: 1,
            duration_periods: 2,
            caf: 0.88,
            saf: 0.93,
        }],
        incidents: vec![IncidentAssignment {
            severity: IncidentSeverity::OneLane,
            segment: 2,
            start_period: 2,
            duration_periods: 1,
        }],
        work_zones: vec![],
        special_events: vec![],
    };
    let sf = rel.build_scenario_facility(&sc);
    // Demand scaled facility-wide by the DAF.
    assert!((sf.mainline_demand[0] - 4000.0 * 1.62).abs() < 1e-9);
    assert!((sf.segments[1].on_ramp_demand[2] - 400.0 * 1.62).abs() < 1e-9);
    // Weather CAF/SAF fold into periods 2-3 for all segments.
    let caf0 = sf.segments[0].caf_schedule.as_ref().unwrap();
    let saf0 = sf.segments[0].saf_schedule.as_ref().unwrap();
    assert!((caf0[0] - 1.0).abs() < 1e-12);
    assert!((caf0[1] - 0.88).abs() < 1e-12);
    assert!((caf0[2] - 0.88).abs() < 1e-12);
    assert!((caf0[3] - 1.0).abs() < 1e-12);
    assert!((saf0[1] - 0.93).abs() < 1e-12);
    // Incident on segment 3, period 3 combines multiplicatively with the
    // weather CAF: one-lane closure on 3 lanes = 0.74 x 2/3 (Exhibit
    // 11-23 note).
    let caf2 = sf.segments[2].caf_schedule.as_ref().unwrap();
    let expected = 0.88 * 0.74 * 2.0 / 3.0;
    assert!(
        (caf2[2] - expected).abs() < 1e-9,
        "combined CAF {} vs {}",
        caf2[2],
        expected
    );
    assert!((caf2[1] - 0.88).abs() < 1e-12);
}

#[test]
fn test_reliability_run_small_rrp() {
    // Two months, weather + incidents: distribution is populated, weights
    // are consistent, and demand/weather variability produce TTI spread.
    let mut weather = WeatherInputs::default();
    for m in 0..12 {
        weather.probabilities_by_month[m][1] = 0.10; // heavy rain
    }
    weather.durations_min[1] = 45.0;
    let cfg = ScenarioGenerationConfig {
        months: vec![1, 7],
        replications: 2,
        weather: Some(weather),
        incidents: Some(IncidentInputs {
            monthly_frequencies: Some(vec![0.8; 12]),
            ..Default::default()
        }),
        rng_seed: 2,
        ..Default::default()
    };
    let mut rel = ReliabilityAnalysis::new(small_facility(), cfg);
    rel.run().unwrap();
    // 2 months x 5 weekdays x 2 replications = 20 scenarios; 4 analysis
    // periods each = 80 observations.
    assert_eq!(rel.scenario_results.len(), 20);
    assert_eq!(rel.distribution.len(), 80);
    let m = rel.metrics.as_ref().unwrap();
    assert!(m.tti_mean >= 1.0, "TTI_mean {}", m.tti_mean);
    assert!(m.tti_95 >= m.tti_80 && m.tti_80 >= m.tti_50);
    assert!(m.tti_max >= m.tti_95);
    assert!(m.misery_index >= m.tti_mean);
    assert!((0.0..=100.0).contains(&m.reliability_rating));
    // Failure percentages are monotone in the target speed.
    assert!(rel.failure_pct_below_speed(50.0) >= rel.failure_pct_below_speed(35.0));
    assert!(
        (rel.on_time_pct_at_speed(45.0) + rel.failure_pct_below_speed(45.0) - 100.0).abs()
            < 1e-9
    );
    // Scenario summaries are exported for all scenarios.
    assert_eq!(rel.scenario_summaries().len(), 20);
    // Free-flow travel time: 9,420 ft at 60 mi/h = 1.784 min.
    let expected_fftt = (9420.0 / 5280.0) / 60.0 * 60.0;
    assert!((rel.free_flow_travel_time_min - expected_fftt).abs() < 1e-9);
}

#[test]
fn test_fixture_json_roundtrip() {
    // The ReliabilityAnalysis struct deserializes from the fixture schema.
    let json = r#"{
        "facility": {
            "segments": [
                { "seg_type": "Basic", "length_ft": 2640.0, "lanes": 3 },
                { "seg_type": "Basic", "length_ft": 2640.0, "lanes": 3 }
            ],
            "mainline_demand": [4000.0, 4400.0],
            "ffs": 60.0
        },
        "scenario_generation": {
            "months": [1, 2],
            "replications": 2,
            "seed_month": 1,
            "seed_weekday": "Tuesday",
            "rng_seed": 1
        }
    }"#;
    let mut rel: ReliabilityAnalysis = serde_json::from_str(json).unwrap();
    assert_eq!(rel.scenario_generation.months, vec![1, 2]);
    assert_eq!(rel.scenario_generation.seed_weekday, Weekday::Tuesday);
    rel.run().unwrap();
    assert_eq!(rel.scenario_results.len(), 2 * 5 * 2);
}
