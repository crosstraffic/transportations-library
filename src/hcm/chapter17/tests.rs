//! Unit tests for HCM Chapter 17 (Urban Street Reliability and ATDM).
//!
//! Exact assertions come from the deterministic sub-computations
//! published in Chapter 29, Example Problem 4 (`230_Ch29_05.xhtml`,
//! Exhibits 29-66 through 29-72) and the exhibit tables of Chapter 17
//! (`121_Ch17_03.xhtml`). Monte Carlo results are asserted at the
//! distribution level (the HCM notes that seeded random streams are
//! software-specific, so per-scenario reproduction of the printed
//! example is not expected).

use super::exhibits::*;
use super::urban_reliability::*;
use crate::hcm::chapter16::urban_facilities::UrbanFacility;
use crate::hcm::chapter18::urban_segments::{BoundaryControlType, UrbanSegment};

macro_rules! assert_near {
    ($actual:expr, $expected:expr, $tol:expr, $what:expr) => {
        let (a, e) = ($actual, $expected);
        assert!(
            (a - e).abs() <= $tol,
            "{}: got {a}, expected {e} (tol {})",
            $what,
            $tol
        );
    };
}

// ═══════════════════════════════════════════════════════════════════════════
// Exhibits 17-5 through 17-7 (demand ratios)
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_exhibit_17_5_hour_of_day() {
    use FunctionalClass::*;
    // Example Problem 4 lookups: principal arterial weekday 7 a.m. =
    // 0.071 and 9 a.m. = 0.047.
    assert_near!(exhibit_17_5_hour_of_day_ratio(UrbanPrincipalArterial, 7, false), 0.071, 1e-9, "PA 7am");
    assert_near!(exhibit_17_5_hour_of_day_ratio(UrbanPrincipalArterial, 9, false), 0.047, 1e-9, "PA 9am");
    // Spot checks across the table corners.
    assert_near!(exhibit_17_5_hour_of_day_ratio(Expressway, 0, false), 0.010, 1e-9, "Expwy 0h wd");
    assert_near!(exhibit_17_5_hour_of_day_ratio(Expressway, 17, false), 0.090, 1e-9, "Expwy 5pm wd");
    assert_near!(exhibit_17_5_hour_of_day_ratio(Expressway, 23, true), 0.023, 1e-9, "Expwy 11pm we");
    assert_near!(exhibit_17_5_hour_of_day_ratio(UrbanMinorArterial, 7, false), 0.067, 1e-9, "MA 7am wd");
    assert_near!(exhibit_17_5_hour_of_day_ratio(UrbanMinorArterial, 0, true), 0.028, 1e-9, "MA 0h we");
    assert_near!(exhibit_17_5_hour_of_day_ratio(UrbanPrincipalArterial, 12, true), 0.071, 1e-9, "PA noon we");
    // Each column sums to ~1.0 (ratios of daily traffic).
    for class in [Expressway, UrbanPrincipalArterial, UrbanMinorArterial] {
        for weekend in [false, true] {
            let sum: f64 = (0..24)
                .map(|h| exhibit_17_5_hour_of_day_ratio(class, h, weekend))
                .sum();
            assert_near!(sum, 1.0, 0.01, "hour-of-day column sum");
        }
    }
}

#[test]
fn test_exhibit_17_6_day_of_week() {
    let expected = [0.87, 0.98, 0.98, 1.00, 1.03, 1.15, 0.99]; // Sun..Sat
    for (d, e) in expected.iter().enumerate() {
        assert_near!(exhibit_17_6_day_of_week_ratio(d as u32), *e, 1e-9, "dow ratio");
    }
}

#[test]
fn test_exhibit_17_7_month_of_year() {
    use FunctionalClass::*;
    // Example Problem 4 lookups: PA January 0.831, April 0.987.
    assert_near!(exhibit_17_7_month_of_year_ratio(UrbanPrincipalArterial, 1), 0.831, 1e-9, "PA Jan");
    assert_near!(exhibit_17_7_month_of_year_ratio(UrbanPrincipalArterial, 4), 0.987, 1e-9, "PA Apr");
    assert_near!(exhibit_17_7_month_of_year_ratio(Expressway, 8), 1.142, 1e-9, "Expwy Aug");
    assert_near!(exhibit_17_7_month_of_year_ratio(UrbanMinorArterial, 7), 1.150, 1e-9, "MA Jul");
    assert_near!(exhibit_17_7_month_of_year_ratio(UrbanMinorArterial, 12), 0.903, 1e-9, "MA Dec");
}

/// Example Problem 4 demand profile: the Tuesday-January-7 a.m. count has
/// a base ratio of 0.0578; a snowy Monday-January-7 a.m. analysis period
/// has ratio 0.0463 and total/base 0.800 (Exhibit 29-67).
#[test]
fn test_example_problem_4_demand_ratios() {
    let cfg = UrbanReliabilityConfig {
        functional_class: FunctionalClass::UrbanPrincipalArterial,
        count_month: 1,
        count_day_of_week: 2, // Tuesday
        count_hour: 7,
        ..UrbanReliabilityConfig::default()
    };
    let base = cfg.base_demand_ratio();
    assert_near!(base, 0.0578, 0.0002, "base demand ratio");
    // Monday, January, 7 a.m. with snow (DCF 0.80).
    let ratio = cfg.demand_ratio(1, 1, 7) * DEFAULT_DEMAND_CHANGE_SNOW;
    assert_near!(ratio, 0.0463, 0.0002, "snowy Monday ratio");
    assert_near!(ratio / base, 0.800, 0.002, "total/base ratio");
    // Wednesday, April, 9 a.m., dry: 0.047 × 1.00 × 0.987 = 0.0464
    // (Exhibit 29-67 last row), total/base 0.802.
    let apr = cfg.demand_ratio(4, 3, 9);
    assert_near!(apr, 0.0464, 0.0002, "April Wednesday 9am ratio");
    assert_near!(apr / base, 0.802, 0.005, "April total/base");
}

// ═══════════════════════════════════════════════════════════════════════════
// Exhibits 17-8 through 17-12 (weather/incident defaults)
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_exhibit_17_9_defaults() {
    assert_near!(exhibit_17_9_cfaf(WeatherCondition::Rainfall), 2.0, 1e-9, "CFAF rf");
    assert_near!(exhibit_17_9_cfaf(WeatherCondition::WetPavement), 3.0, 1e-9, "CFAF wp");
    assert_near!(exhibit_17_9_cfaf(WeatherCondition::Snowfall), 1.5, 1e-9, "CFAF sf");
    assert_near!(exhibit_17_9_cfaf(WeatherCondition::SnowOrIceOnPavement), 2.75, 1e-9, "CFAF sp");
    assert_near!(DEFAULT_INCIDENT_DETECTION_MIN, 2.0, 1e-9, "detection");
    assert_near!(exhibit_17_9_response_time_min(WeatherCondition::Dry), 15.0, 1e-9, "resp dry");
    assert_near!(exhibit_17_9_response_time_min(WeatherCondition::Snowfall), 20.4, 1e-9, "resp sf");
}

/// Exhibit 29-72 duration inputs: a dry-weather noncrash one-lane
/// breakdown has clearance 10.8 min and average duration 2.0 + 15.0 +
/// 10.8 = 27.8 min.
#[test]
fn test_exhibit_17_10_clearance_and_duration() {
    use IncidentSeverity::*;
    use WeatherCondition::*;
    assert_near!(exhibit_17_10_clearance_time_min(NoncrashBreakdown, Dry), 10.8, 1e-9, "bkd dry");
    assert_near!(default_incident_duration_min(NoncrashBreakdown, Dry), 27.8, 1e-9, "bkd dry total");
    assert_near!(exhibit_17_10_clearance_time_min(CrashFatalInjury, Dry), 56.4, 1e-9, "FI dry");
    assert_near!(exhibit_17_10_clearance_time_min(CrashFatalInjury, Snowfall), 76.7, 1e-9, "FI snow");
    assert_near!(exhibit_17_10_clearance_time_min(CrashPropertyDamage, Rainfall), 28.6, 1e-9, "PDO rf");
    assert_near!(exhibit_17_10_clearance_time_min(NoncrashOther, WetPavement), 2.8, 1e-9, "other wp");
    // Snow/ice column applies to both snow conditions (exhibit note b).
    assert_near!(
        exhibit_17_10_clearance_time_min(NoncrashBreakdown, SnowOrIceOnPavement),
        14.7, 1e-9, "bkd sp"
    );
}

#[test]
fn test_incident_distribution_proportions() {
    // Crash proportions (Exhibits 17-11/17-12 "Type" column).
    assert_near!(crash_proportion(StreetLocation::Segment), 0.358, 1e-9, "seg pc");
    assert_near!(crash_proportion(StreetLocation::Intersection), 0.310, 1e-9, "int pc");
    // Joint proportions total 1.000 per street location (exhibit note;
    // printed values round to 0.999-1.003).
    for street in [StreetLocation::Segment, StreetLocation::Intersection] {
        for shoulder in [true, false] {
            let joint = incident_joint_proportions(street, shoulder);
            let sum: f64 = joint.iter().sum();
            assert_near!(sum, 1.0, 0.005, "joint proportion sum");
            if !shoulder {
                // Exhibit 17-12: shoulder rows are zero.
                assert_eq!(joint[4], 0.0);
                assert_eq!(joint[5], 0.0);
                assert_eq!(joint[10], 0.0);
                assert_eq!(joint[11], 0.0);
            }
        }
    }
    // Exhibit 17-11 spot values used by Exhibit 29-70: segment noncrash
    // one-lane breakdown 0.456; crash shoulder PDO 0.160.
    let seg = incident_joint_proportions(StreetLocation::Segment, true);
    assert_near!(seg[6], 0.456, 1e-9, "seg nc-1L-bkd");
    assert_near!(seg[5], 0.160, 1e-9, "seg cr-sh-PDO");
    let int = incident_joint_proportions(StreetLocation::Intersection, true);
    assert_near!(int[6], 0.486, 1e-9, "int nc-1L-bkd");
}

// ═══════════════════════════════════════════════════════════════════════════
// Equations 29-13 through 29-18 (Example Problem 4 exact values)
// ═══════════════════════════════════════════════════════════════════════════

/// Exhibit 29-69: with the Lincoln 2-year weather hours (dry 17,026.98;
/// rainfall 278.22; wet 104.33; snowfall 64.61; snow/ice 45.86) and the
/// default CFAFs, Segment 1-2's observed 15 crashes/year become 14.50
/// (dry), 29.01 (rainfall), 43.51 (wet), 21.76 (snowfall), and 39.89
/// (snow/ice) crashes/year; Segment 2-3's 16 become 15.47 dry.
#[test]
fn test_equation_29_13_crash_frequency_by_weather() {
    let hours = [17_026.98, 278.22, 104.33, 64.61, 45.86];
    let cfaf = [2.0, 3.0, 1.5, 2.75];
    let dry = equivalent_crash_frequency_dry(15.0, hours, 2.0, cfaf);
    assert_near!(dry, 14.50, 0.01, "Fc dry (seg 1-2)");
    assert_near!(dry * 2.0, 29.01, 0.02, "Fc rainfall");
    assert_near!(dry * 3.0, 43.51, 0.02, "Fc wet pavement");
    assert_near!(dry * 1.5, 21.76, 0.02, "Fc snowfall");
    assert_near!(dry * 2.75, 39.89, 0.02, "Fc snow/ice");
    assert_near!(
        equivalent_crash_frequency_dry(16.0, hours, 2.0, cfaf),
        15.47, 0.01, "Fc dry (seg 2-3)"
    );
}

/// Example Problem 4, Step 5 text: Fi = 14.50/0.358 = 40.5 incidents/year
/// (dry) and 21.76/0.358 = 60.8 (snowfall); hourly frequencies 0.00515
/// (Wednesday, April, 9 a.m.) and 0.00963 (Monday, January, 7 a.m.), and
/// the Exhibit 29-70/29-71 no-incident probabilities.
#[test]
fn test_equations_29_15_through_29_17() {
    let fi_dry = 14.50 / crash_proportion(StreetLocation::Segment);
    assert_near!(fi_dry, 40.5, 0.05, "Fi dry");
    let fi_snow = 21.76 / crash_proportion(StreetLocation::Segment);
    assert_near!(fi_snow, 60.8, 0.05, "Fi snowfall");
    // Equation 29-16.
    let f_apr = fi_dry / 8_760.0 * (24.0 * 0.047) * 1.00 * 0.987;
    assert_near!(f_apr, 0.00515, 0.00002, "hourly fi April");
    let f_jan = fi_snow / 8_760.0 * (24.0 * 0.071) * 0.98 * 0.831;
    assert_near!(f_jan, 0.00963, 0.00002, "hourly fi January");
    // Equation 29-17 (Exhibit 29-70 rows; the printed exhibit's
    // "0.021/0.016" shoulder-crash proportions are typos for the Exhibit
    // 17-11 values 0.020/0.160 — its p0 column back-computes to the
    // latter).
    assert_near!((-f_apr * 0.456f64).exp(), 0.99766, 0.00002, "p0 nc-1L-bkd");
    assert_near!((-f_apr * 0.036f64).exp(), 0.99981, 0.00002, "p0 cr-1L-FI");
    assert_near!((-f_apr * 0.160f64).exp(), 0.99918, 0.00002, "p0 cr-sh-PDO");
    assert_near!((-f_jan * 0.456f64).exp(), 0.99562, 0.00002, "p0 nc-1L-bkd Jan");
}

// ═══════════════════════════════════════════════════════════════════════════
// Statistical inverses (Exhibits 29-66 and 29-72)
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_normal_inverse() {
    // Standard checks.
    assert_near!(normal_inverse(0.5, 0.0, 1.0), 0.0, 1e-6, "z(0.5)");
    assert_near!(normal_inverse(0.975, 0.0, 1.0), 1.959964, 1e-4, "z(0.975)");
    // Exhibit 29-66, Jan 10: RN 0.94, mean 22.4°F, sd 5 → 30°F.
    assert_near!(normal_inverse(0.94, 22.4, 5.0), 30.17, 0.05, "Jan 10 temperature");
    // Exhibit 29-66, Apr 5: RN 0.11, mean 51.2 → 45°F.
    assert_near!(normal_inverse(0.11, 51.2, 5.0), 45.07, 0.05, "Apr 5 temperature");
}

#[test]
fn test_gamma_inverse() {
    // Exponential special case (mean = sd): gamma⁻¹(p, μ, μ) = −μ ln(1−p).
    assert_near!(gamma_inverse(0.83, 0.30, 0.30), -0.30 * (0.17f64).ln(), 1e-6, "exp case");
    // Exhibit 29-72: gamma⁻¹(0.57455, 0.463 h, 0.371 h) = 0.433 h
    // (α = 1.5625, β = 0.2965).
    assert_near!(gamma_inverse(0.57455, 0.463, 0.371), 0.433, 0.005, "EP4 incident duration");
    // Median of a gamma is below the mean for these shapes.
    assert!(gamma_inverse(0.5, 1.0, 0.8) < 1.0);
    // CDF round-trip monotonicity.
    let lo = gamma_inverse(0.25, 2.0, 1.0);
    let hi = gamma_inverse(0.75, 2.0, 1.0);
    assert!(lo < hi);
}

// ═══════════════════════════════════════════════════════════════════════════
// Equations 29-25 through 29-28 and 29-34 through 29-36
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_weather_adjustment_factors() {
    use WeatherCondition::*;
    assert_near!(weather_sat_flow_factor(Dry, 0.0), 1.0, 1e-9, "frs dry");
    // Equation 29-25: rain at 0.5 in/h → 1/(1 + 0.48×0.5) = 0.806.
    assert_near!(weather_sat_flow_factor(Rainfall, 0.5), 1.0 / 1.24, 1e-9, "frs rain");
    // Snow uses the 0.39 coefficient on the water-equivalent rate.
    assert_near!(weather_sat_flow_factor(Snowfall, 0.1), 1.0 / 1.039, 1e-9, "frs snow");
    assert_near!(weather_sat_flow_factor(WetPavement, 0.0), 0.95, 1e-9, "frs wet");
    assert_near!(weather_sat_flow_factor(SnowOrIceOnPavement, 0.0), 0.90, 1e-9, "frs ice");
    // Equation 29-26: snow coefficient is 1.4.
    assert_near!(weather_ffs_factor(Snowfall, 0.1), 1.0 / 1.14, 1e-9, "fs snow");
    assert_near!(weather_ffs_factor(Rainfall, 0.5), 1.0 / 1.24, 1e-9, "fs rain");
    assert_near!(weather_ffs_factor(WetPavement, 0.0), 0.95, 1e-9, "fs wet");
}

#[test]
fn test_incident_sat_flow_factor() {
    use IncidentSeverity::*;
    // Equation 29-27: one of two through lanes blocked by an FI crash on
    // a 4-lane approach: (1 − 1/2)(1 − 0.58/4) = 0.4275.
    assert_near!(incident_sat_flow_factor(1, 2, 4, CrashFatalInjury), 0.4275, 1e-9, "1of2 FI");
    // Shoulder incident (0 lanes blocked): first term 1.0.
    assert_near!(incident_sat_flow_factor(0, 2, 4, NoncrashBreakdown), 1.0 - 0.17 / 4.0, 1e-9, "shoulder");
    // All lanes blocked floors at 0.10.
    assert_near!(incident_sat_flow_factor(2, 2, 2, CrashFatalInjury), 0.10, 1e-9, "all blocked");
    // Severity coefficients (Equation 29-28).
    assert_near!(incident_severity_coefficient(CrashFatalInjury), 0.58, 1e-9, "b FI");
    assert_near!(incident_severity_coefficient(CrashPropertyDamage), 0.42, 1e-9, "b PDO");
    assert_near!(incident_severity_coefficient(NoncrashOther), 0.17, 1e-9, "b other");
}

#[test]
fn test_additional_delay_equations() {
    // Equations 29-34/29-35: 2,640-ft segment, base FFS 40 mi/h, snow/ice
    // (factor 0.90), PDO crash in a 2-lane direction:
    // S* = 40 × 0.90 × (1 − 0.42/2) = 28.44 mi/h.
    let s_star = adjusted_base_ffs(40.0, 0.90, Some(IncidentSeverity::CrashPropertyDamage), 2);
    assert_near!(s_star, 28.44, 1e-9, "adjusted FFS");
    let d = additional_delay_s(2_640.0, 40.0, s_star);
    let expected =
        2_640.0 * (1.0 / (28.44 * 5_280.0 / 3_600.0) - 1.0 / (40.0 * 5_280.0 / 3_600.0));
    assert_near!(d, expected, 1e-9, "d_other");
    assert!(d > 0.0);
    // No weather, no incident → no additional delay.
    assert_near!(
        additional_delay_s(2_640.0, 40.0, adjusted_base_ffs(40.0, 1.0, None, 2)),
        0.0, 1e-9, "dry"
    );
}

#[test]
fn test_exhibit_29_5_lt_headway() {
    use WeatherCondition::*;
    assert_near!(exhibit_29_5_extra_lt_headway_s(Dry), 0.0, 1e-9, "dry");
    assert_near!(exhibit_29_5_extra_lt_headway_s(Rainfall), 0.7, 1e-9, "raining");
    assert_near!(exhibit_29_5_extra_lt_headway_s(Snowfall), 1.2, 1e-9, "snowing");
    assert_near!(exhibit_29_5_extra_lt_headway_s(WetPavement), 0.7, 1e-9, "water");
    assert_near!(exhibit_29_5_extra_lt_headway_s(SnowOrIceOnPavement), 0.9, 1e-9, "snow/ice");
}

// ═══════════════════════════════════════════════════════════════════════════
// Weather event generation
// ═══════════════════════════════════════════════════════════════════════════

fn lincoln_like_weather() -> Vec<MonthlyWeather> {
    // January and April from Exhibit 29-65; the other months use mild
    // interpolations (test data — the published exhibit lists only two
    // months).
    vec![
        MonthlyWeather { total_precip_in: 0.67, total_snowfall_in: 6.6, days_with_precip: 5.0, mean_temp_f: 22.4, precip_rate_in_h: 0.030 },
        MonthlyWeather { total_precip_in: 0.80, total_snowfall_in: 5.0, days_with_precip: 6.0, mean_temp_f: 27.0, precip_rate_in_h: 0.035 },
        MonthlyWeather { total_precip_in: 1.80, total_snowfall_in: 3.0, days_with_precip: 7.0, mean_temp_f: 39.0, precip_rate_in_h: 0.045 },
        MonthlyWeather { total_precip_in: 2.90, total_snowfall_in: 1.5, days_with_precip: 9.0, mean_temp_f: 51.2, precip_rate_in_h: 0.062 },
        MonthlyWeather { total_precip_in: 4.20, total_snowfall_in: 0.0, days_with_precip: 11.0, mean_temp_f: 62.0, precip_rate_in_h: 0.070 },
        MonthlyWeather { total_precip_in: 3.50, total_snowfall_in: 0.0, days_with_precip: 9.0, mean_temp_f: 72.0, precip_rate_in_h: 0.080 },
        MonthlyWeather { total_precip_in: 3.00, total_snowfall_in: 0.0, days_with_precip: 8.0, mean_temp_f: 78.0, precip_rate_in_h: 0.085 },
        MonthlyWeather { total_precip_in: 3.20, total_snowfall_in: 0.0, days_with_precip: 8.0, mean_temp_f: 75.0, precip_rate_in_h: 0.080 },
        MonthlyWeather { total_precip_in: 2.90, total_snowfall_in: 0.0, days_with_precip: 7.0, mean_temp_f: 66.0, precip_rate_in_h: 0.070 },
        MonthlyWeather { total_precip_in: 1.90, total_snowfall_in: 0.5, days_with_precip: 6.0, mean_temp_f: 54.0, precip_rate_in_h: 0.055 },
        MonthlyWeather { total_precip_in: 1.20, total_snowfall_in: 2.5, days_with_precip: 5.0, mean_temp_f: 38.0, precip_rate_in_h: 0.040 },
        MonthlyWeather { total_precip_in: 0.80, total_snowfall_in: 6.0, days_with_precip: 5.0, mean_temp_f: 26.0, precip_rate_in_h: 0.032 },
    ]
}

#[test]
fn test_weather_generation_deterministic_and_plausible() {
    let mut cfg = UrbanReliabilityConfig {
        weather: lincoln_like_weather(),
        weather_seed: 82,
        ..UrbanReliabilityConfig::default()
    };
    let a = generate_weather_events(&cfg);
    let b = generate_weather_events(&cfg);
    assert_eq!(a.len(), b.len(), "same seed → same events");
    assert!(!a.is_empty(), "wet climate must generate events");
    // Expected event count ≈ Σ days_with_precip over 2 years ≈ 172;
    // allow a broad Monte Carlo band.
    assert!(
        (100..=250).contains(&a.len()),
        "event count {} outside plausible band",
        a.len()
    );
    for e in &a {
        assert!(e.start_h >= 0.0 && e.start_h < 24.0);
        assert!(e.precip_duration_h >= 0.0);
        assert!(e.pavement_duration_h >= e.precip_duration_h);
        assert!(e.start_h + e.pavement_duration_h <= 24.0 + 1e-9, "truncated at midnight");
        // Snow only when cold (Equation 29-4).
        if e.is_snow {
            assert!(e.temperature_f < 32.0);
        } else {
            assert!(e.temperature_f >= 32.0);
        }
    }
    // Both event types occur in this climate.
    assert!(a.iter().any(|e| e.is_snow), "some snow events");
    assert!(a.iter().any(|e| !e.is_snow), "some rain events");
    // Different seed → different stream.
    cfg.weather_seed = 99;
    let c = generate_weather_events(&cfg);
    assert_ne!(
        a.iter().map(|e| e.day).collect::<Vec<_>>(),
        c.iter().map(|e| e.day).collect::<Vec<_>>(),
        "different seed should give a different event pattern"
    );
    // Condition-hours bookkeeping sums to the 2-year total.
    let hours = weather_condition_hours(&a);
    assert_near!(hours.iter().sum::<f64>(), 17_520.0, 1e-6, "total hours");
    assert!(hours[1] > 0.0, "some rainfall hours");
}

#[test]
fn test_weather_at_timeline() {
    let events = vec![WeatherEvent {
        day: 10,
        is_snow: false,
        temperature_f: 47.0,
        precip_rate_in_h: 0.10,
        total_precip_in: 0.2,
        start_h: 7.0,
        precip_duration_h: 2.0,
        pavement_duration_h: 3.0,
    }];
    // During rainfall.
    let (c, r) = weather_at(&events, 10, 7.5, 0.25);
    assert_eq!(c, WeatherCondition::Rainfall);
    assert_near!(r, 0.10, 1e-9, "rate");
    // Wet pavement window (9:00-10:00).
    let (c, r) = weather_at(&events, 10, 9.25, 0.25);
    assert_eq!(c, WeatherCondition::WetPavement);
    assert_near!(r, 0.0, 1e-9, "no rate");
    // After drying / other day.
    assert_eq!(weather_at(&events, 10, 10.5, 0.25).0, WeatherCondition::Dry);
    assert_eq!(weather_at(&events, 11, 7.5, 0.25).0, WeatherCondition::Dry);
    // Snow event → snowfall then snow/ice, with water-equivalent rate.
    let snow = vec![WeatherEvent {
        day: 3,
        is_snow: true,
        temperature_f: 25.0,
        precip_rate_in_h: 0.54, // in. of snow per hour
        total_precip_in: 2.08,
        start_h: 4.5,
        precip_duration_h: 4.0,
        pavement_duration_h: 5.25,
    }];
    let (c, r) = weather_at(&snow, 3, 7.0, 0.25);
    assert_eq!(c, WeatherCondition::Snowfall);
    assert_near!(r, 0.054, 1e-9, "water-equivalent snow rate");
    assert_eq!(weather_at(&snow, 3, 9.0, 0.25).0, WeatherCondition::SnowOrIceOnPavement);
}

// ═══════════════════════════════════════════════════════════════════════════
// End-to-end reliability run (distribution level)
// ═══════════════════════════════════════════════════════════════════════════

/// An Example Problem 4-like facility: 3-mi principal arterial, six
/// 0.5-mi segments, seven signals, 2 through lanes, 35 mi/h, coordinated
/// at C = 100 s with good progression.
fn ep4_like() -> UrbanReliability {
    let mut segments = Vec::new();
    for _ in 0..6 {
        let mut s =
            UrbanSegment::new(2_640.0, 2, 35.0, 1_000.0, BoundaryControlType::Signalized);
        s.proportion_with_curb = 1.0;
        s.n_access_points_subject = 2.0;
        s.n_access_points_opposing = 2.0;
        s.midsegment_flow_veh_h = Some(1_000.0);
        s.cycle_length_s = Some(100.0);
        s.effective_green_s = Some(45.0);
        s.platoon_ratio = Some(1.333);
        s.sat_flow_veh_h_ln = Some(1_800.0);
        s.full_stop_rate_override = Some(0.5);
        segments.push(s);
    }
    let facility = UrbanFacility::new(segments);
    let cfg = UrbanReliabilityConfig {
        functional_class: FunctionalClass::UrbanPrincipalArterial,
        months: (1..=12).collect(),
        days_of_week: vec![1, 2, 3, 4, 5],
        jan1_day_of_week: 6, // Saturday → Jan 4 is a Tuesday, as in EP4
        study_period_start_hour: 7,
        analysis_periods_per_day: 12, // 7-10 a.m.
        weather: lincoln_like_weather(),
        count_month: 1,
        count_day_of_week: 2,
        count_hour: 7,
        incidents: IncidentConfig {
            segment_crash_frequencies: vec![15.0, 16.0, 17.0, 18.0, 19.0, 20.0],
            intersection_crash_frequencies: vec![32.0, 33.0, 34.0, 35.0, 36.0, 37.0, 38.0],
            shoulder_present: true,
            cfaf_override: None,
            minor_leg_volume_veh_h: 1_300.0,
            opposing_demand_veh_h: None,
        },
        boundary_signals: vec![
            BoundarySignal {
                cycle_length_s: 100.0,
                effective_green_s: 45.0,
                sat_flow_veh_h_ln: 1_800.0,
                platoon_ratio: 1.333,
                k_factor: 0.5,
                i_factor: 1.0,
                approach_lanes: 4,
            };
            6
        ],
        weather_seed: 82,
        demand_seed: 11,
        incident_seed: 63,
        ..UrbanReliabilityConfig::default()
    };
    UrbanReliability::new(facility, cfg)
}

#[test]
fn test_reliability_run_ep4_like_distribution() {
    let mut analysis = ep4_like();
    let results = analysis.run().unwrap().clone();

    // Scenario count: weekdays in the modeled year × 12 analysis periods.
    // With Jan 1 on Saturday the 365-day year has 260 weekdays, giving
    // the published Example Problem 4 count of 3,120 scenarios.
    assert_eq!(results.num_scenarios, 3_120, "scenario count");
    assert_eq!(analysis.scenarios.len(), results.num_scenarios);

    // Base free-flow travel time: published EP4 value is 262.9 s for the
    // 3-mi facility (base FFS ≈ 41 mi/h); the assembled inputs are not
    // the complete published dataset, so assert a band around it.
    assert!(
        (250.0..280.0).contains(&results.base_free_flow_travel_time_s),
        "base FF travel time {}",
        results.base_free_flow_travel_time_s
    );

    // Distribution-level checks against the published EP4 magnitudes
    // (mean TTI 1.69/1.64, TTI80 1.57/1.56, PTI 2.98/2.61, reliability
    // rating 93.2/94.1). Monte Carlo streams differ by implementation,
    // so wide bands are asserted.
    let m = &results.metrics;
    assert!(m.tti_mean >= 1.0, "TTI >= 1 by definition, got {}", m.tti_mean);
    assert!((1.1..2.6).contains(&m.tti_mean), "mean TTI {}", m.tti_mean);
    assert!(m.tti_80 >= m.tti_50, "percentile ordering");
    assert!(m.tti_95 >= m.tti_80, "percentile ordering");
    assert!((1.2..5.0).contains(&m.tti_95), "PTI {}", m.tti_95);
    assert!(
        (70.0..=100.0).contains(&results.reliability_rating_urban),
        "urban reliability rating {}",
        results.reliability_rating_urban
    );
    assert!(
        results.reliability_rating_urban >= m.reliability_rating,
        "TTI<2.5 share cannot be below TTI<1.33 share"
    );
    assert!(results.num_incidents > 50, "incidents {}", results.num_incidents);
    assert!(results.num_weather_events > 50, "weather events {}", results.num_weather_events);
    assert!(results.pct_nondry_scenarios > 0.5, "some nondry scenarios");
    assert!(results.total_vhd > 0.0);

    // Determinism: same seeds → identical metrics.
    let mut again = ep4_like();
    let r2 = again.run().unwrap();
    assert_near!(r2.metrics.tti_mean, m.tti_mean, 1e-12, "deterministic mean TTI");
    assert_near!(r2.metrics.tti_95, m.tti_95, 1e-12, "deterministic PTI");
    assert_eq!(r2.num_incidents, results.num_incidents);
}

/// ATDM strategy hooks: added through-phase green (Example Problem 5,
/// Strategy 1 direction of effect) must improve mean travel time and not
/// degrade the PTI or reliability rating; a crash-frequency hook (EP5
/// Strategies 2/3 HSM effects) must increase generated incidents.
#[test]
fn test_atdm_strategy_hooks() {
    let mut base = ep4_like();
    let base_results = base.run().unwrap().clone();

    let mut improved = ep4_like();
    improved.atdm_strategies.push(AtdmStrategy {
        name: "Shift 5 s to the coordinated phase".into(),
        effective_green_adjustment_s: 5.0,
        ..AtdmStrategy::default()
    });
    let strat_results = improved.run().unwrap().clone();
    assert!(
        strat_results.mean_travel_time_s < base_results.mean_travel_time_s,
        "green reallocation should reduce mean travel time ({} vs {})",
        strat_results.mean_travel_time_s,
        base_results.mean_travel_time_s
    );
    assert!(
        strat_results.metrics.tti_95 <= base_results.metrics.tti_95,
        "PTI should not degrade"
    );
    assert!(
        strat_results.reliability_rating_urban >= base_results.reliability_rating_urban,
        "reliability rating should not degrade"
    );

    let mut riskier = ep4_like();
    riskier.atdm_strategies.push(AtdmStrategy {
        name: "Higher crash risk configuration".into(),
        crash_frequency_adjustment: 1.5,
        ..AtdmStrategy::default()
    });
    riskier.run().unwrap();
    assert!(
        riskier.results.as_ref().unwrap().num_incidents > base_results.num_incidents,
        "higher CFAF must generate more incidents"
    );
}

/// Residual-queue carryover (HCM Chapter 17, Section 3, Facility
/// Evaluation: "the initial queue input value for the next analysis
/// period is set equal to the residual queue output for the current
/// analysis period"). A single-lane, deliberately over-capacity segment
/// with no weather/incidents (so demand ratio and capacity are identical
/// across matching periods) isolates the carryover effect: the second of
/// two same-hour analysis periods must show a higher TTI than the first
/// because it inherits a nonzero initial queue Qb, and the pattern must
/// reset (not persist) across the ~21-h gap to the next day's first
/// period.
#[test]
fn test_residual_queue_carryover_and_day_reset() {
    let mut s = UrbanSegment::new(2_640.0, 1, 35.0, 3_000.0, BoundaryControlType::Signalized);
    s.proportion_with_curb = 1.0;
    s.n_access_points_subject = 2.0;
    s.n_access_points_opposing = 2.0;
    s.midsegment_flow_veh_h = Some(3_000.0);
    s.cycle_length_s = Some(100.0);
    s.effective_green_s = Some(45.0);
    s.platoon_ratio = Some(1.0);
    s.sat_flow_veh_h_ln = Some(1_800.0);
    s.full_stop_rate_override = Some(0.5);
    let facility = UrbanFacility::new(vec![s]);

    let cfg = UrbanReliabilityConfig {
        functional_class: FunctionalClass::UrbanPrincipalArterial,
        months: vec![1],
        days_of_week: vec![2], // Tuesdays only
        jan1_day_of_week: 6,   // Saturday -> Jan 4 is a Tuesday
        study_period_start_hour: 7,
        analysis_periods_per_day: 2, // two 15-min periods, same clock hour
        weather: vec![MonthlyWeather::default(); 12], // all-zero: no events
        count_month: 1,
        count_day_of_week: 2,
        count_hour: 7,
        incidents: IncidentConfig {
            segment_crash_frequencies: vec![0.0],
            intersection_crash_frequencies: vec![0.0, 0.0],
            shoulder_present: true,
            ..IncidentConfig::default()
        },
        boundary_signals: vec![BoundarySignal {
            cycle_length_s: 100.0,
            effective_green_s: 45.0,
            sat_flow_veh_h_ln: 1_800.0,
            platoon_ratio: 1.0,
            k_factor: 0.5,
            i_factor: 1.0,
            approach_lanes: 1,
        }],
        weather_seed: 1,
        demand_seed: 1,
        incident_seed: 1,
        ..UrbanReliabilityConfig::default()
    };

    let mut analysis = UrbanReliability::new(facility, cfg);
    let results = analysis.run().unwrap().clone();
    assert!(
        results.num_scenarios >= 4,
        "need at least two Tuesdays of two periods each, got {}",
        results.num_scenarios
    );

    let r = &analysis.scenario_results;
    // Demand (3,000 veh/h) vastly exceeds the one-lane capacity
    // (1 * 1,800 * 45/100 = 810 veh/h), so both periods of every day are
    // oversaturated even at ratio 1.0.
    assert!(r[0].oversaturated, "day A period 1 must be oversaturated");
    assert!(r[1].oversaturated, "day A period 2 must be oversaturated");
    assert!(
        r[1].tti > r[0].tti,
        "carried-in queue must raise period 2 TTI above period 1 ({} vs {})",
        r[1].tti,
        r[0].tti
    );
    assert!(
        r[1].vhd > r[0].vhd,
        "carried-in queue must raise period 2 vehicle-hours of delay ({} vs {})",
        r[1].vhd,
        r[0].vhd
    );

    // Day-boundary reset: day B's first period starts fresh (Qb = 0) with
    // the same demand ratio, weather, and capacity as day A's first
    // period, so it must reproduce day A's period-1 TTI exactly rather
    // than inheriting day A's end-of-study-period queue.
    assert!(
        (r[2].tti - r[0].tti).abs() < 1e-9,
        "day reset: day B period 1 TTI {} should equal day A period 1 TTI {}",
        r[2].tti,
        r[0].tti
    );
}

#[test]
fn test_validation_errors() {
    let mut a = ep4_like();
    a.config.boundary_signals.pop();
    assert!(a.run().is_err(), "mismatched boundary signals");

    let mut b = ep4_like();
    b.config.incidents.intersection_crash_frequencies.pop();
    assert!(b.run().is_err(), "needs segments+1 intersections");

    let mut c = ep4_like();
    c.config.months.clear();
    assert!(c.run().is_err(), "empty RRP");
}
