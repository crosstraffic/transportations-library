# HCM Chapter 17 — Urban Street Reliability and ATDM, Core Methodology

This document walks through the Rust translation of HCM 7th Edition Chapter 17, Section 3 core methodology (EPUB `121_Ch17_03.xhtml`), together with the Chapter 29 ("Urban Street Facilities: Supplemental"), Section 2 Scenario Generation Procedure (`227_Ch29_02.xhtml`) that supplies its computational detail: weather event generation (Equations 29-1 through 29-12), traffic demand variation (Exhibits 17-5 through 17-8), traffic incident generation (Equations 29-13 through 29-24), and scenario dataset generation (Equations 29-25 through 29-36). The code lives in `src/hcm/urban_reliability/urban_reliability.rs` (the Monte Carlo generation stages, the inverse-distribution primitives, and the `UrbanReliability` driver) and `src/hcm/urban_reliability/exhibits.rs` (Exhibits 17-5 through 17-12 lookups and Equations 29-25 through 29-36). Each generated scenario (one analysis period of one day) is evaluated with the Chapter 16/18 facility engine documented in `chapter16.md`; the per-scenario facility travel times feed `crate::hcm::common::reliability::TravelTimeDistribution`, from which the Chapter 17 performance measures are computed.

Per the module doc comment, the HCM procedure is itself Monte Carlo ("A random number seed is used ... so that the sequence of random events can be reproduced"), and the module follows the Chapter 11 freeway-reliability implementation's convention of an in-crate seeded xorshift64* PRNG (`crate::hcm::freeway_reliability::scenario_generation::Prng`) with three independent seeds (`weather_seed`, `demand_seed` — unused directly since demand is deterministic given the systematic ratios, `incident_seed`). Because the HCM text itself states that "evaluating the same dataset and seed number in different software ... may produce results different from those shown [in the printed examples]. Each result, though different, will be equally valid," the published Example Problem 4 outputs are verified at the distribution-band level rather than as exact values, while the deterministic sub-computations (crash frequencies, demand ratios, adjustment factors) are verified exactly.

## Step-by-step walkthrough

| Stage | Equations/Exhibits | Rust function(s) | File | Inputs (units) | Outputs (units) |
|---|---|---|---|---|---|
| Inverse-distribution primitives | (statistical machinery underlying Eqs. 29-3, 29-5/29-6, 29-19) | `normal_inverse` (Acklam's rational approximation, rel. error < 1.15e-9), `ln_gamma`/`gamma_p` (Lanczos + Numerical Recipes `gammp`), `gamma_inverse` (bisection on `gamma_p`) | `urban_reliability.rs` | probability `p` (0-1), `mean`, `sd` | inverse-CDF value, same units as `mean`/`sd` |
| Stage 1a — Weather event generation | Eqs. 29-1 through 29-12 | `generate_weather_events` | `urban_reliability.rs` | `UrbanReliabilityConfig` (12 `MonthlyWeather` entries: `total_precip_in`, `total_snowfall_in`, `days_with_precip`, `mean_temp_f`, `precip_rate_in_h`), `weather_seed` | `Vec<WeatherEvent>` over a 2-year record (`day`, `is_snow`, `temperature_f` °F, `precip_rate_in_h`, `total_precip_in` in, `start_h`/`precip_duration_h`/`pavement_duration_h` h) |
| Weather lookup helpers | Step 8 discussion | `weather_at`, `weather_condition_hours` | `urban_reliability.rs` | `&[WeatherEvent]`, `day`, `hour_start`, `duration_h` | `(WeatherCondition, precip_rate_in_h)`; `[f64; 5]` total hours per condition (Eq. 29-13's N_h inputs) |
| Equivalent crash frequency | Eq. 29-13, `F_c,dry = F_c × 8,760 N_y / (N_h,dry + ΣCFAF_wea N_h,wea)` | `equivalent_crash_frequency_dry` | `urban_reliability.rs` | `expected_crash_frequency` (crashes/yr), `hours: [f64;5]`, `n_years`, `cfaf: [f64;4]` | `F_c,dry` (crashes/yr) |
| Stage 1c — Traffic incident generation | Eqs. 29-14 through 29-24 | `UrbanReliability::generate_incidents` | `urban_reliability.rs` | per-location crash frequencies (`IncidentConfig`), weather record, functional-class demand ratios | `Vec<UrbanIncident>` (`day_of_year`, `street`, `location_index`, `affects_subject_direction`, `incident_type`, `start_hour`, `duration_h`, `weather`) |
| Stage 1b/1d — Demand variation + scenario generation | Eq. 29-29 (demand ratio); Exhibits 17-5/17-6/17-7/17-8 | `UrbanReliability::generate_scenarios`, `UrbanReliabilityConfig::demand_ratio`/`base_demand_ratio` | `urban_reliability.rs` | functional class, month/day-of-week/hour, weather demand-change factors | `Vec<UrbanScenario>` (one per analysis period in the reliability reporting period) |
| Stage 2 — Scenario (facility) evaluation | Eqs. 29-25 through 29-28, 29-34 through 29-36; Chapter 19 d1/d2 delay via `common::delay` | `UrbanReliability::evaluate_scenario` | `urban_reliability.rs` | scenario demand ratio, active incidents, `BoundarySignal` per segment, ATDM `AtdmStrategy` schedule | `UrbanScenarioResult` (`travel_time_s`, `tti`, `vmt` veh-mi, `vhd` veh-h, `oversaturated: bool`) |
| Performance summary | Chapter 17, Section 3 performance measures | `UrbanReliability::run` (drives all stages then calls `TravelTimeDistribution::metrics`/`pct_at_or_below`) | `urban_reliability.rs` | all of the above | `UrbanReliabilityResults` (`mean_travel_time_s`, `metrics: ReliabilityMetrics`, `reliability_rating_urban`, `total_vhd`, `num_weather_events`, `num_incidents`, `pct_nondry_scenarios`) |

### Weather event generation (Stage 1a)

`generate_weather_events` walks a 2-year (730-day) record and, per day, follows Steps 1-8 of the Chapter 29 procedure: Step 1 (Eqs. 29-1/29-2) draws whether precipitation occurs from the monthly `days_with_precip / n_days` probability; Step 2 (Eqs. 29-3/29-4) draws a daily mean temperature via `normal_inverse` (`s_T = 5.0°F` fixed, `TEMPERATURE_SD_F`) and classifies rain vs. snow at the 32°F threshold; Steps 3/7 (Eqs. 29-5 through 29-8) draw a correlated precipitation rate and total depth via two calls to `gamma_inverse` sharing the same uniform draw `r_r` (so `R_td = R_rd`, "perfectly correlated" per the code comment), with snow statistics obtained by scaling the rain statistics by `SNOW_TO_RAIN_DEPTH_RATIO`; Step 4 (Eq. 29-9) derives duration as `total/rate` capped at 24 h; Step 5 (Eq. 29-10) places a start time rounded to the 15-minute analysis-period increment; Step 6 (Eqs. 29-11/29-12) computes the wet/snow-covered pavement duration as precipitation duration plus a runoff time (`RAIN_PAVEMENT_RUNOFF_H` for rain, the analyst-configurable `snow_runoff_h` for snow) plus a drying time `0.888 e^(−0.0070 T) + 0.19·I_night` (night bonus applied for rain only). This last point is a documented deviation: the code comment states Exhibit 29-66 "reproduces the night term for rain events only (its snow rows omit it)," and the implementation follows the exhibit rather than applying the +0.19 term uniformly, cross-referenced to `docs/hcm/VERIFICATION.md`.

```
Equation 29-1:  P(precip)_m = Ndp_m / Nd_m                                                [probability, unitless]
Equation 29-2:  No precipitation if Rp_d,m ≥ P(precip)_m;  Precipitation if Rp_d,m < P(precip)_m
  P(precip)_m = probability of precipitation on any given day of month m                    (unitless)
  Ndp_m       = number of days with precipitation ≥ 0.01 in. in month m, analyst input `days_with_precip` (d)
  Nd_m        = total number of days in month m                                             (d)
  Rp_d,m      = uniform random number (0,1) for precipitation on day d of month m            (unitless)
Implemented in: urban_reliability/urban_reliability.rs::generate_weather_events (Step 1: p_precip, r_pd)
```

```
Equation 29-3:  T_d,m = normal⁻¹(p = Rg_d, μ = T̄_m, σ = s_T)                               [°F]
Equation 29-4:  Rain if T_d,m ≥ 32°F;  Snow if T_d,m < 32°F
  T_d,m = average temperature for day d of month m                                          (°F)
  Rg_d  = uniform random number (0,1) for temperature on day d                               (unitless)
  T̄_m   = normal daily mean temperature in month m, analyst input `mean_temp_f`               (°F)
  s_T   = standard deviation of daily mean temperature in a month, TEMPERATURE_SD_F = 5.0     (°F)
  normal⁻¹(p, μ, σ) = inverse standard-normal CDF scaled to μ, σ (Acklam's rational approximation, relative error < 1.15e-9)
Implemented in: urban_reliability/urban_reliability.rs::normal_inverse (primitive); urban_reliability/urban_reliability.rs::generate_weather_events (Step 2: temp, is_snow)
```

```
Equation 29-5:  rr_d,m = gamma⁻¹(p = Rr_d, μ = r̄r_m, σ = s_rr,m)                            [in./h]
Equation 29-6:  tr_d,m = gamma⁻¹(p = Rt_d, μ = t̄r_m, σ = s_tr,m)     with Rt_d = Rr_d         [in./event]
Equation 29-7:  t̄r_m = tp_m / Ndp_m                                                          [in./event]
  rr_d,m = rainfall rate for the rain event on day d of month m                              (in./h)
  Rr_d   = uniform random number (0,1) for rainfall rate on day d, shared with Rt_d           (unitless — "R_td = R_rd, perfectly correlated" per code comment)
  r̄r_m   = average precipitation rate in month m, analyst input `precip_rate_in_h`            (in./h)
  s_rr,m = standard deviation of precipitation rate in month m (= 1.0 × r̄r_m)                 (in./h)
  tr_d,m = total rainfall for the rain event on day d of month m                              (in./event)
  t̄r_m   = average total rainfall per event in month m                                       (in./event)
  tp_m   = total normal precipitation in month m, analyst input `total_precip_in`             (in.)
  Ndp_m  = number of days with precipitation in month m, analyst input `days_with_precip`      (d)
  s_tr,m = standard deviation of total rainfall per event in month m (Equation 29-8)           (in.)
  gamma⁻¹(p, μ, σ) = inverse gamma CDF parameterized by mean/sd, shape α = μ²/σ², scale β = σ²/μ (bisection on the regularized lower incomplete gamma function P(α, x/β); Lanczos ln-Γ plus Numerical Recipes series/continued-fraction `gammp`)
  Step 7 (snow): r̄r_m and t̄r_m are the rain statistics scaled by SNOW_TO_RAIN_DEPTH_RATIO = 10.0 in./in. before applying Equations 29-5/29-6/29-7 unchanged.
Implemented in: urban_reliability/urban_reliability.rs::gamma_inverse (primitive, via ln_gamma/gamma_p); urban_reliability/urban_reliability.rs::generate_weather_events (Steps 3/7: rate_mean, rate, total_mean, total)
```

```
Equation 29-8:  s_tr,m = min(2.5 · t̄r_m, 0.65)                                              [in.]
  s_tr,m = standard deviation of total rainfall per event in month m                          (in.)
  t̄r_m   = average total rainfall per event in month m (Equation 29-7)                       (in.)
  0.65   = published cap on the rain-event standard deviation                                 (in.)
  For snow events the code scales both terms of the min() by SNOW_TO_RAIN_DEPTH_RATIO = 10.0, i.e. `total_sd = (2.5 * total_mean).min(0.65 * depth_ratio)`, since the HCM text states only that "the equations are the same" for snow without giving a snow-specific cap (see Deviations, item 3).
Implemented in: urban_reliability/urban_reliability.rs::generate_weather_events (total_sd)
```

```
Equation 29-9:  dr_d,m = tr_d,m / rr_d,m                                                    [h/event]
  dr_d,m = rainfall duration for the rain event on day d of month m, capped at 24 h           (h/event)
  tr_d,m = total rainfall for the event (Equation 29-6)                                       (in./event)
  rr_d,m = rainfall rate for the event (Equation 29-5)                                        (in./h)
Implemented in: urban_reliability/urban_reliability.rs::generate_weather_events (dur)
```

```
Equation 29-10:  ts_d,m = (24 − dr_d,m) · R_s,d                                             [h after midnight]
  ts_d,m = start time of the weather event on day d of month m, rounded to the analysis-period increment (h)
  dr_d,m = event duration (Equation 29-9)                                                    (h)
  R_s,d  = uniform random number (0,1) for start time on day d                                (unitless)
Implemented in: urban_reliability/urban_reliability.rs::generate_weather_events (start)
```

```
Equation 29-11:  dw_d,m = dr_d,m + do_d,m + dd_d,m                                          [h/event]
Equation 29-12:  dd_d,m = 0.888 · e^(−0.0070·T_d,m) + 0.19·I_night
  dw_d,m  = duration of wet (or snow-covered) pavement for the event on day d of month m       (h/event)
  dr_d,m  = precipitation duration (Equation 29-9)                                             (h/event)
  do_d,m  = pavement runoff duration: RAIN_PAVEMENT_RUNOFF_H = 0.083 for rain; analyst-configured `snow_runoff_h` (default DEFAULT_SNOW_PAVEMENT_RUNOFF_H = 0.5) for snow  (h/event)
  dd_d,m  = drying-time duration                                                               (h/event)
  T_d,m   = average temperature for the day (Equation 29-3)                                    (°F)
  I_night = 1.0 if the event starts outside 6:00 a.m.-6:00 p.m., else 0.0                       (indicator, unitless)
  dw_d,m is truncated so the event never extends past midnight (capped against the hours remaining from the start time to 24:00).
  (See Deviations: Exhibit 29-66's snow rows omit the +0.19 night term that its rain rows include; the implementation follows the exhibit and applies the night bonus to rain events only.)
Implemented in: urban_reliability/urban_reliability.rs::generate_weather_events (runoff, night, drying, wet_total)
```

### Equivalent crash frequency and incident generation (Stage 1c)

`equivalent_crash_frequency_dry` implements Equation 29-13 exactly. `UrbanReliability::generate_incidents` then, per reliability-reporting-period day and hour, computes the weather-adjusted crash frequency (`fc_wea = fc_dry × CFAF_wea`, Eq. 29-15's `Fi = CFAF_str × Fc,wea / p_c`, with `crash_proportion` supplying `p_c` from Exhibit 17-11/17-12 depending on shoulder presence), the hourly Poisson rate (Eq. 29-16, `fi_hour = fi_year/8,760 × 24 f_hod × f_dow × f_moy`), and for each of the 12 `INCIDENT_TYPES` (Exhibit 17-11 row order) draws occurrence via the no-incident probability `p0 = e^(−fi_hour·p_i)` (Eqs. 29-17/29-18) and, if it occurs, a gamma-distributed duration (Eq. 29-19, mean from `default_incident_duration_min` by severity/weather, sd = `INCIDENT_DURATION_CV × mean`) truncated at midnight and rounded to the analysis-period increment. Location assignment (Eqs. 29-20 through 29-24) draws whether the incident affects the subject direction from a volume-proportional probability: for segment incidents, `v_subject/(v_subject + v_opposing)`; for intersection incidents, the major-leg share of total intersection volume (`major/(2·major + 2·minor)`), where `minor` is the analyst-configured `minor_leg_volume_veh_h` applied uniformly to both minor legs of every intersection (a simplification the code does not separately number against a specific per-intersection HCM variable).

```
Equation 29-13:  Fc_dry = Fc · 8,760 · Ny / (Nh_dry + CFAF_rf·Nh_rf + CFAF_wp·Nh_wp + CFAF_sf·Nh_sf + CFAF_sp·Nh_sp)     [crashes/yr]
  Fc      = expected (base) crash frequency for the street location                            (crashes/yr, analyst input)
  Fc_dry  = equivalent crash frequency if every day were dry                                    (crashes/yr)
  8,760   = hours per year                                                                      (h/yr)
  Ny      = number of years spanned by the weather record (= 2.0, the 2-yr record)              (yr)
  Nh_dry  = hours of dry weather in the record                                                  (h)
  CFAF_rf/wp/sf/sp = crash frequency adjustment factors for rainfall / wet pavement / snowfall / snow-or-ice, Exhibit 17-9 defaults (`exhibit_17_9_cfaf`: 2.0 / 3.0 / 1.5 / 2.75)  (unitless)
  Nh_rf/wp/sf/sp   = hours of each weather condition in the record (`weather_condition_hours`)   (h)
Implemented in: urban_reliability/urban_reliability.rs::equivalent_crash_frequency_dry
```

```
Equation 29-14:  Fc_wea = Fc_dry · CFAF_wea                                                    [crashes/yr]
  Fc_wea   = equivalent crash frequency for weather condition wea (CFAF_dry = 1.0)               (crashes/yr)
  Fc_dry   = equivalent dry-weather crash frequency (Equation 29-13)                             (crashes/yr)
  CFAF_wea = crash frequency adjustment factor for condition wea, Exhibit 17-9                   (unitless)
Implemented in: urban_reliability/urban_reliability.rs::UrbanReliability::generate_incidents (fc_wea, via the cfaf_of closure)
```

```
Equation 29-15:  Fi_wea = CFAF_str · Fc_wea / pc_wea                                            [incidents/yr]
  Fi_wea   = average incident frequency for the street location and weather condition            (incidents/yr)
  CFAF_str = crash frequency adjustment factor for an active ATDM strategy/work zone/special event (default 1.0; the product of every active `AtdmStrategy::crash_frequency_adjustment`)  (unitless)
  Fc_wea   = equivalent crash frequency for weather condition wea (Equation 29-14)               (crashes/yr)
  pc_wea   = proportion of incidents that are crashes at the street location, Exhibit 17-11/17-12 (`crash_proportion`: 0.358 segment / 0.310 intersection)  (unitless)
Implemented in: urban_reliability/urban_reliability.rs::UrbanReliability::generate_incidents (fi_year, strategy_cfaf)
```

```
Equation 29-16:  fi_h,d = (Fi_wea / 8,760) · (24 · f_hod,h,d) · f_dow,d · f_moy,d                [incidents/h]
  fi_h,d    = expected hourly incident frequency for hour h of day d                             (incidents/h)
  Fi_wea    = average incident frequency (Equation 29-15)                                        (incidents/yr)
  8,760     = hours per year                                                                     (h/yr)
  f_hod,h,d = hour-of-day demand ratio, Exhibit 17-5 (`exhibit_17_5_hour_of_day_ratio`)           (unitless)
  f_dow,d   = day-of-week demand ratio, Exhibit 17-6 (`exhibit_17_6_day_of_week_ratio`)           (unitless)
  f_moy,d   = month-of-year demand ratio, Exhibit 17-7 (`exhibit_17_7_month_of_year_ratio`)       (unitless)
Implemented in: urban_reliability/urban_reliability.rs::UrbanReliability::generate_incidents (fi_hour)
```

```
Equation 29-17:  p0_h,d = e^(−fi_h,d · pi)                                                      [probability, unitless]
Equation 29-18:  No incident if Ri_h,d ≤ p0_h,d;  Incident if Ri_h,d > p0_h,d
  p0_h,d = probability of no incident of the given event type / lane location / severity         (unitless)
  fi_h,d = expected hourly incident frequency (Equation 29-16)                                   (incidents/h)
  pi     = joint proportion of incidents of type con/lan/sev at this street location, Exhibit 17-11/17-12 (`incident_joint_proportions`), one of the 12 `INCIDENT_TYPES` rows  (unitless)
  Ri_h,d = uniform random number (0,1) for incident occurrence                                   (unitless)
Implemented in: urban_reliability/urban_reliability.rs::UrbanReliability::generate_incidents (p0, `r <= p0` test)
```

```
Equation 29-19:  di = gamma⁻¹(p = Rd, μ = d̄i, σ = s)                                            [h]
  di   = incident duration                                                                      (h)
  Rd   = uniform random number (0,1) for incident duration                                      (unitless)
  d̄i   = mean incident duration = detection + response + clearance time (`default_incident_duration_min`), Exhibit 17-9/17-10  (min, converted to h)
  s    = standard deviation of incident duration = INCIDENT_DURATION_CV · d̄i, INCIDENT_DURATION_CV = 0.8  (h)
  The result is truncated at midnight and rounded to the nearest analysis-period increment (0.25 h).
Implemented in: urban_reliability/urban_reliability.rs::UrbanReliability::generate_incidents (mean_h, sd_h, dur)
```

```
Equation 29-20:  pv_2 = lv_2/(2·tv);  pv_4 = pv_2 + lv_4/(2·tv);  pv_6 = pv_4 + lv_6/(2·tv);  pv_8 = 1.0
Equation 29-21:  tv = Σ_(j=1..12) v_input,j                                                      [veh/h]
  pv_n = cumulative volume proportion for the leg served by NEMA phase n (n = 2, 4, 6, 8)         (unitless)
  lv_n = two-way leg volume for NEMA phase n at the intersection                                 (veh/h)
  tv   = total intersection volume, summed over the 12 input movement volumes                    (veh/h)
  Code simplification: only the major-street through legs (phases 2/6, carrying the segment's two-way through demand `v_subj + v_opp`) and a single analyst-configured two-way minor-leg volume (`minor_leg_volume_veh_h`, applied identically to both minor legs, phases 4/8) are modeled, rather than 12 distinct per-movement input volumes.
Implemented in: urban_reliability/urban_reliability.rs::UrbanReliability::generate_incidents (major, minor, tv2, p2 for `StreetLocation::Intersection`)
```

```
Equation 29-22:  Incident on Phase 2 if Rv ≤ pv_2;  Phase 4 if pv_2 < Rv ≤ pv_4;  Phase 6 if pv_4 < Rv ≤ pv_6;  Phase 8 if pv_6 < Rv ≤ pv_8
  Rv         = uniform random number (0,1) for incident leg assignment                           (unitless)
  pv_2/4/6/8 = cumulative volume proportions (Equation 29-20)                                    (unitless)
  Code simplification: only a binary subject-through vs. not draw is made (`rng.next_f64() <= p2` against the major-leg share), since the facility evaluation models only the subject through movement at each boundary intersection.
Implemented in: urban_reliability/urban_reliability.rs::UrbanReliability::generate_incidents (`StreetLocation::Intersection` branch, affects_subject)
```

```
Equation 29-23:  pv_2 = dv_2 / (dv_2 + dv_6);  pv_6 = 1.0                                       [proportion, unitless]
  pv_2/pv_6 = cumulative volume proportion for the Phase 2 / Phase 6 direction of travel on the segment  (unitless)
  dv_2, dv_6 = demand flow rate in the Phase 2 and Phase 6 directions of the segment              (veh/h)
Implemented in: urban_reliability/urban_reliability.rs::UrbanReliability::generate_incidents (v_subj, v_opp, p2 for `StreetLocation::Segment`)
```

```
Equation 29-24:  Incident in Phase 2 direction if Rv ≤ pv_2;  Phase 6 direction if pv_2 < Rv ≤ pv_6
  Rv         = uniform random number (0,1) for direction assignment                              (unitless)
  pv_2/pv_6  = cumulative volume proportions (Equation 29-23)                                    (unitless)
  v_subj     = `through_demand_veh_h` of the subject direction on the segment                     (veh/h)
  v_opp      = opposing-direction demand, analyst-configured `opposing_demand_veh_h` (defaults to v_subj)  (veh/h)
Implemented in: urban_reliability/urban_reliability.rs::UrbanReliability::generate_incidents (`StreetLocation::Segment` branch, affects_subject)
```

### Demand variation and scenario dataset generation (Stage 1b/1d, Stage 2)

`UrbanReliabilityConfig::demand_ratio` composes the Exhibit 17-5 (hour-of-day), 17-6 (day-of-week), and 17-7 (month-of-year) default ratios by functional class; `generate_scenarios` computes each scenario's demand ratio as `demand_ratio(month, dow, hour) × weather_demand_change_factor / base_demand_ratio()` (Equation 29-29's structure, with the weather demand-change factor being Exhibit 17-8's rain/snow multiplier, `demand_change_rain`/`demand_change_snow`, applied only during rain/snow — not during wet-pavement or snow-on-pavement conditions). `evaluate_scenario` then applies, per segment: the demand ratio and any active ATDM strategy demand adjustment to `through_demand_veh_h`; the more-severe of any active segment/intersection incidents (`is_more_severe`/`worse`, ranked by `severity_rank`) with lane closures computed by `lanes_blocked` (always leaving at least one lane open, per the HCM text quoted in the doc comment); the Equation 29-35 adjusted base free-flow speed (`adjusted_base_ffs`) and Equation 29-34 additional running delay (`additional_delay_s`), added into the Chapter 18 `midsegment_other_delay_s` input rather than modifying the segment's own free-flow speed field; the Equation 29-27 incident saturation-flow-rate factor (`incident_sat_flow_factor`) combined multiplicatively with the Equation 29-25 weather saturation-flow factor (`weather_sat_flow_factor`) and any ATDM `sat_flow_adjustment`; and finally the Chapter 19 uniform + incremental delay equations (`common::delay::{progression_factor, uniform_delay, incremental_delay}`) recomputed on the adjusted demand/capacity/green to produce `through_control_delay_s`, before calling the segment's own Chapter 18 `analyze()`. Facility travel time, VMT, and oversaturation flag are accumulated across segments; `UrbanReliability::run` aggregates every scenario's travel-time-index observation (VMT-weighted by default, `vmt_weighted`) into a `TravelTimeDistribution` and reads off the Chapter 17 performance measures, including `reliability_rating_urban` — the percentage of the (VMT-weighted) distribution with TTI below the urban-street threshold `URBAN_RELIABILITY_RATING_TTI_THRESHOLD` (2.5), which the doc comment distinguishes from `ReliabilityMetrics::reliability_rating`'s freeway 1.33 threshold in the shared module.

```
Equation 29-29:  v_h,d = (v_input / (f_hod,input · f_dow,input · f_moy,input)) · f_hod,h,d · f_dow,d · f_moy,d     [veh/h]
  v_h,d   = adjusted hourly flow rate for hour h of day d                                        (veh/h)
  v_input = base-dataset traffic count volume                                                    (veh/h)
  f_hod,input / f_dow,input / f_moy,input = systematic ratios at the count's month/day-of-week/hour, the denominator (`base_demand_ratio()`)  (unitless)
  f_hod,h,d / f_dow,d / f_moy,d = systematic ratios at the scenario's month/day-of-week/hour (`demand_ratio()`)  (unitless)
  Code restates this as a pure ratio (scenario ratio / base ratio) applied multiplicatively to `through_demand_veh_h`, with the Exhibit 17-8 weather demand-change factor (`demand_change_rain`/`demand_change_snow`) folded in as an additional multiplier on rain/snow analysis periods only (not wet-pavement/snow-on-pavement periods).
Implemented in: urban_reliability/urban_reliability.rs::UrbanReliabilityConfig::demand_ratio, ::base_demand_ratio; ::UrbanReliability::generate_scenarios (ratio)
```

```
Equation 29-25:  f_rs = 1 / (1 + 0.48·R_r + 0.39·R_s)                                            [unitless]
  f_rs = saturation flow rate adjustment factor for rainfall/snowfall (0.95 for wet pavement not raining; 0.90 for snow/ice on pavement not snowing, applied as fixed constants rather than this formula)  (unitless)
  R_r  = rainfall rate, water-equivalent                                                          (in./h)
  R_s  = snowfall rate, water-equivalent                                                          (in./h)
Implemented in: urban_reliability/exhibits.rs::weather_sat_flow_factor
```

```
Equation 29-26:  f_s,rs = 1 / (1 + 0.48·R_r + 1.4·R_s)                                           [unitless]
  f_s,rs = free-flow speed adjustment factor for rainfall/snowfall (0.95 for wet pavement not raining; 0.90 for snow/ice on pavement not snowing, fixed constants)  (unitless)
  R_r / R_s = rainfall/snowfall rate, water-equivalent                                            (in./h)
Implemented in: urban_reliability/exhibits.rs::weather_ffs_factor
```

```
Equation 29-27:  f_ic = (1 − N_ic/N_n) · (1 − b_ic/ΣN_n) ≥ 0.10                                  [unitless]
Equation 29-28:  b_ic = 0.58·I_fi + 0.42·I_pdo + 0.17·I_other
  f_ic  = saturation flow rate adjustment factor for an incident on an intersection movement       (unitless, floored at 0.10)
  N_ic  = number of lanes serving the movement blocked by the incident                             (ln)
  N_n   = number of lanes serving the movement under normal conditions                             (ln)
  ΣN_n  = total lanes on the approach across all movements (`approach_lanes`)                      (ln)
  b_ic  = incident-severity calibration coefficient                                                (unitless)
  I_fi/I_pdo/I_other = indicator (1/0) for fatal-injury / property-damage-only / other (noncrash) severity  (unitless)
Implemented in: urban_reliability/exhibits.rs::incident_sat_flow_factor, ::incident_severity_coefficient
```

```
Equation 29-34:  d_other = L · (1/S* − 1/S_fo)                                                  [s/veh]
Equation 29-35:  S* = S_fo · f_s,rs · (1 − b_ic/N_o)                                              [mi/h]
Equation 29-36:  b_ic = 0.58·I_fi + 0.42·I_pdo + 0.17·I_other
  d_other = additional running delay from weather/incidents, added to the Chapter 18 `midsegment_other_delay_s` input  (s/veh)
  L       = segment length                                                                        (ft; converted with 5,280/3,600 to consistent ft/s speeds)
  S*      = adjusted base free-flow speed during the analysis period                              (mi/h)
  S_fo    = base free-flow speed under base conditions                                            (mi/h)
  f_s,rs  = weather free-flow-speed adjustment factor (Equation 29-26)                             (unitless)
  b_ic    = incident-severity coefficient (Equation 29-28/29-36), 0 if no incident in the subject direction  (unitless)
  N_o     = number of lanes serving the subject direction (`direction_lanes`)                      (ln)
Implemented in: urban_reliability/exhibits.rs::adjusted_base_ffs, ::additional_delay_s, ::incident_severity_coefficient
```

Random 15-min demand variation (Equations 29-30 through 29-33, the randomized flow-rate element) is a documented deferral — see the "Deferred" section below — and is not implemented, so it is not written out here.

### ATDM strategy hook

`AtdmStrategy` (name, month/day-of-week/period activation schedule, multiplicative `demand_adjustment`/`sat_flow_adjustment`/`ffs_adjustment`, additive `effective_green_adjustment_s`, multiplicative `crash_frequency_adjustment`) is applied per-scenario in `evaluate_scenario` when the scenario's month/day-of-week/period matches the strategy's (empty schedule fields = always active), and its `crash_frequency_adjustment` additionally multiplies the incident-generation crash frequency (Eq. 29-15's `CFAF_str`) in `generate_incidents`. This is documented as an input-hook level implementation of Chapter 17's ATDM strategy assessment concept ("geometric-configuration and signal-control ATDM strategies are evaluated ... by using scenarios ... for each desired lane configuration") rather than the Chapter 37 strategy-specific behavioral models, which remain deferred.

```
ATDM adjustment composition (input-hook level; not itself an HCM-numbered equation):
  demand_ratio_scenario = demand_ratio(month,dow,hour) · Π(active AtdmStrategy.demand_adjustment)                   [unitless multiplier on through_demand_veh_h]
  sat_flow_adjusted     = sat_flow_veh_h_ln · f_rs · f_ic · Π(active AtdmStrategy.sat_flow_adjustment)               [veh/h/ln]
  green_adjusted        = clamp(effective_green_s + Σ(active AtdmStrategy.effective_green_adjustment_s), 1, cycle_length_s − 1)   [s]
  ffs_adjusted          = base_ffs_mph · Π(active AtdmStrategy.ffs_adjustment)                                       [mi/h]
  CFAF_str (Eq 29-15)   = Π(active AtdmStrategy.crash_frequency_adjustment)                                         [unitless]
  where "active" = strategies whose months/days_of_week/periods schedule matches the scenario (empty list = always active); f_rs, f_ic are Equations 29-25/29-27.
Implemented in: urban_reliability/urban_reliability.rs::UrbanReliability::evaluate_scenario (strat_demand, strat_sat, strat_green, strat_ffs); ::generate_incidents (strategy_cfaf)
```

## Deviations (cross-referenced to `docs/hcm/VERIFICATION.md`)

`docs/hcm/VERIFICATION.md` exists at this branch's tip; its "Chapter 16/17 (feat/hcm-ch16-17-urban-facilities)" section records: (1) Exhibit 29-66's snow rows omit the +0.19 night-drying term of Equation 29-12 that its rain rows include; the implementation follows the exhibit (documented above); (2) Exhibit 29-70's printed shoulder-crash proportions (0.021/0.016) are typos for Exhibit 17-11's 0.020/0.160 — the exhibit's own p0 column back-computes to the latter, and `crash_proportion`/`incident_joint_proportions` in `exhibits.rs` use the corrected 0.020/0.160 values; (3) the Equation 29-8 standard-deviation cap for snow (`total_sd = (2.5 × total_mean).min(0.65 × depth_ratio)` in `generate_weather_events`) scales the printed 0.65-in rain cap by the 10:1 snow/rain depth ratio, matching Exhibit 29-66's magnitudes, since the HCM text is silent on a snow-specific cap; (4) the Chapter 29 Example Problem 4 fixture's published coordinated-actuated average phase duration is not printed in the extracted text, so the fixture's `BoundarySignal.effective_green_s` (45 s) was chosen to reproduce the published base condition rather than transcribed from an exhibit; (5) the Chapter 29 Example Problem 1 facility fixtures (`chapter16.md`'s `case1.json`/`case2.json`) have unpublished Segments 2-4, so facility-level speed/stop-rate differ slightly from the published aggregate (22.1 vs. 22.6 mi/h) — the fully-published Chapter 30 Example Problem 1 segment case (`case3.json`) reproduces exactly; (6) the Chapter 29 Example Problem 4 reliability distribution's TTI-80 is within 0.03 of the published value but the PTI (TTI-95) tail is lighter (1.73 computed vs. roughly 2.6-3.0 published), originally attributed to the then-deferred residual-queue carryover between analysis periods (the d3 initial-queue delay term); per the VERIFICATION.md item's own update, carryover has since been implemented on `feat/hcm-reliability-enhancements` (see `reliability-enhancements.md` and VERIFICATION.md's "Reliability enhancements" section) — the PTI gap narrowed only modestly (1.73 → 1.75) and is now attributed to other still-deferred elements (random 15-minute demand variation, incident-duration defaults) rather than the carryover mechanism. All six items are interpretation/reproduction notes rather than `VERIFY-HCM`-flagged code defects; a grep of `urban_reliability.rs` and `exhibits.rs` for `VERIFY-HCM` found no inline markers in either file (the deviations are documented in module/function doc comments instead).

## Validation

The fixture lives at `tests/ExampleCases/hcm/UrbanReliability/case1.json`, reproducing the HCM Chapter 29, Section 5, Example Problem 4 configuration (a 3-mi Lincoln, Nebraska principal arterial; weekdays for one year; 7-10 a.m. study period), exercised by `tests/chapter17_integration.rs` and mirrored by `tests/test_chapter17_integration.py`. `test_case1_example_problem_4` asserts the deterministic scenario count exactly (3,120 = 12 analysis periods x 260 weekdays), the base free-flow travel time within a ±10 s band of the published 262.9 s, and the Monte Carlo distribution measures (mean TTI, TTI-80, TTI-95/PTI, reliability rating) within bands around the published Exhibit 29-73 eastbound/westbound values (mean TTI 1.69/1.64, TTI-80 1.57/1.56, PTI 2.98/2.61, reliability rating 93.2/94.1), plus percentile-ordering (`tti_50 <= tti_80 <= tti_95`), positive total VHD, and more than 50 generated weather events and incidents. `test_case1_replication_with_different_seeds` reproduces the Exhibit 29-75 "replication" concept — different seeds should agree within a generous 10% band on mean travel time while remaining within the same TTI-mean band, mirroring the HCM's own published ~1.4% variation across replications. `test_case1_example_problem_5_strategy_1` reproduces the Example Problem 5 strategy-evaluation direction of effect (a +5 s split shift to the coordinated phase must reduce mean travel time, not increase PTI, and not degrade the reliability rating, mirroring the published 438.2 -> 400.7 s / 93.2 -> 96.8 rating shift) as a directional (not magnitude) check.

Unit tests in `src/hcm/urban_reliability/tests.rs` spot-check individual equations and lookup tables against tabulated/derived values: `test_exhibit_17_5_hour_of_day`, `test_exhibit_17_6_day_of_week`, `test_exhibit_17_7_month_of_year`, `test_example_problem_4_demand_ratios` (composed ratio against Example Problem 4's own worked demand factors), `test_exhibit_17_9_defaults`, `test_exhibit_17_10_clearance_and_duration`, `test_incident_distribution_proportions` (Exhibit 17-11/17-12 tables, including the corrected shoulder-crash proportions), `test_equation_29_13_crash_frequency_by_weather`, `test_equations_29_15_through_29_17`, `test_normal_inverse` and `test_gamma_inverse` (the inverse-CDF primitives against known percentiles), `test_weather_adjustment_factors`, `test_incident_sat_flow_factor`, `test_additional_delay_equations`, `test_exhibit_29_5_lt_headway`, `test_weather_generation_deterministic_and_plausible` and `test_weather_at_timeline` (generation-pipeline sanity and determinism checks), `test_reliability_run_ep4_like_distribution` (a lighter-weight synthetic version of the integration test), `test_atdm_strategy_hooks`, and `test_validation_errors`.

`tests/test_chapter17_integration.py` exercises the PyO3 bindings against the same `case1.json`: `test_scenario_count`, `test_base_free_flow_travel_time`, `test_distribution_measures`, `test_results_json`, and `test_seeded_reproducibility`.

## Deferred

Per the `urban_reliability.rs` module doc comment's "Documented deferrals" list: random 15-minute demand variation (Equations 29-30 through 29-33, the optional randomized flow-rate element — scenarios use only the systematic hour/day/month/weather demand factors); full alternative HCM datasets for work zones and special events (only the `AtdmStrategy` input-hook-level adjustment is implemented, not distinct work-zone/special-event dataset generation); the Chapter 37 ATDM strategy-specific behavioral models (only the input-hook demand/saturation-flow/green-time/crash-frequency adjustment schedule is implemented); and the Exhibit 29-5 critical left-turn-headway weather adjustment's direct wiring into a left-turn engine (the factor is exposed via `exhibit_29_5_extra_lt_headway_s` for a caller to apply to the Chapter 19/20 engines, since the facility evaluation here models only the through movement, which the adjustment does not affect directly).
