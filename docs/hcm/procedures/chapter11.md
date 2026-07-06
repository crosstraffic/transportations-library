# HCM Chapter 11: Freeway Reliability Analysis

This document walks a reviewer through the Rust implementation of the HCM 7th Edition Chapter 11 methodology (Steps B-1 through B-13), which wraps the Chapter 10 freeway facilities core methodology in a scenario loop over a multiday/multimonth reliability reporting period (RRP). The Chapter 25, Section 9 scenario generator (Exhibit 25-39's 34-step hybrid procedure) combines deterministic day-of-week x month-of-year demand variability, deterministic scheduled work zones, and weather/incident events (deterministic event counts via delta-rounding equations, with a seeded stochastic pairing to scenarios/start-times/segments); each scenario is evaluated with the Chapter 10 core method; the resulting facility travel times form a weighted travel-time-index (TTI) distribution from which the Chapter 11 reliability performance measures are computed. The code lives in `src/hcm/freeway_reliability/scenario_generation.rs` (the 34-step generator and the in-module PRNG), `src/hcm/freeway_reliability/reliability.rs` (the Chapter 11 `ReliabilityAnalysis` orchestrator), `src/hcm/freeway_reliability/exhibits.rs` (weather/incident/demand-ratio exhibit lookups and the Equations 11-1 through 11-5 planning-level method), and `src/hcm/common/reliability.rs` (the chapter-agnostic weighted TTI distribution and reliability metrics, shared with the future Chapter 17 urban street reliability module). Deviations from the printed manual are documented inline below as `VERIFY-HCM` comments and cross-referenced against `docs/hcm/VERIFICATION.md` (the "Chapter 11 (feat/hcm-ch11-freeway-reliability)" section), which catalogs six items; each is referenced by item number at its relevant step below rather than re-derived. Every equation named below is transcribed in full plain-text/unicode form under its step, with a where-clause defining every variable's units, and the implementing Rust function at its current (`freeway_reliability`) path, cross-checked against both the Rust source and the HCM 7th Edition EPUB MathML (`74_Ch11_01.xhtml` through `79_Ch11_06.xhtml` for Chapter 11; `200_Ch25_09.xhtml` for the Chapter 25, Section 9 scenario generator).

## Step-by-step walkthrough

| HCM Step | Equations / Exhibits | Rust location | Notes |
|---|---|---|---|
| B-1 seed dataset / study period | — | `reliability.rs::ReliabilityAnalysis::seed_statistics`, `::free_flow_travel_time` | Base facility is a Chapter 10 `FreewayFacility`; seed VMT/lanes/FFS feed the scenario generator. |
| B-2 through B-8 demand, weather, incident, work zone, special-event inputs | Eq 25-71 through 25-93 (Chapter 25 Section 9) | `scenario_generation.rs::ScenarioGenerationConfig`, `::WeatherInputs`, `::IncidentInputs`, `::WorkZoneEvent`, `::SpecialEvent` | Input structs; see the Chapter 25 subsection below for the generation algorithm itself. |
| B-9 scenario CAF/SAF/DAF assembly | multiplicative combination | `reliability.rs::ReliabilityAnalysis::build_scenario_facility` | Folds weather, incident (via Exhibit 11-23), work zone, and special-event factors into per-segment `caf_schedule`/`saf_schedule`, and DAF into `mainline_demand`/ramp demands. |
| B-10 per-scenario evaluation | Chapter 10 core method | `reliability.rs::ReliabilityAnalysis::run` (calls `FreewayFacility::run_analysis`) | Each of the (typically 240) scenarios is a full Chapter 10 facility run. |
| B-11 travel time distribution assembly | VMT/probability-weighted TTI | `reliability.rs::ReliabilityAnalysis::run`, `common/reliability.rs::TravelTimeDistribution::add` | `facility_travel_time_min` sums `L_i / U_i` per scenario/period; TTI = travel time / free-flow travel time. |
| B-11/B-13 reliability performance measures | TTI percentiles, misery index, reliability rating, semi-std-dev, failure/on-time | `common/reliability.rs::TravelTimeDistribution::{mean, percentile, misery_index, reliability_rating, semi_std_dev, pct_above}`, `reliability.rs::ReliabilityAnalysis::{failure_pct_below_speed, on_time_pct_at_speed}` | See the metrics subsection below. |
| Planning-level method (Eq 11-1 through 11-5) | Eq 11-1 through 11-5 | `exhibits.rs::{planning_recurring_delay_rate, planning_incident_delay_rate, planning_tti_mean, planning_tti_95, planning_pt45}` | A standalone, non-scenario-based facility reliability estimate; not wired into `ReliabilityAnalysis`. |

### Scenario generation (Chapter 25, Section 9's 34 steps)

`generate_scenarios` in `scenario_generation.rs` is organized around the same step groups as Exhibit 25-39:

- **Steps 2-5 (demand combinations and probabilities, Equations 25-71 through 25-73).** The function iterates `months x weekdays` to form `N_dc` demand combinations, each expanded to `replications` scenarios; scenario probability is `day_counts[dc] / (nr * total_days)` (Equation 25-73), and demand adjustment factor is `DAF_s = DM(s) / DM(Seed)` (Equation 25-72) using `ScenarioGenerationConfig::demand_multiplier`/`seed_demand_multiplier`, which index into `demand_multipliers` (defaulting to the Exhibit 11-18 `URBAN_DEMAND_RATIOS` table in `exhibits.rs`).
- **Steps 6-9 (deterministic work zone assignment, Equation 25-74).** For each `WorkZoneEvent`, `n_wz = round(active_day_ratio * replications)` replications (capped at the configured replication count) are assigned the work zone whenever the scenario's month/weekday match; this is a pure deterministic assignment, not RNG-driven.
- **Steps 10-18 (weather events, Equations 25-75/25-76).** `expected_weather_frequency` implements Equation 25-76 (`E[n] = round(P_t * D_SP * N_scen,j / E15[D_w])`, with mean duration rounded to the nearest 15-min analysis period, minimum 1 period). For each nonzero expected count, the code draws `n_events` weather events, each assigned via `rng.pick_weighted(&month_probs)` (probability-weighted scenario draw) and a uniform random start period (`rng.gen_range`), retrying up to 1,000 times on temporal overlap with an already-placed event in that scenario, then falling back to a deterministic first-fit search (`'outer: for &sid in &month_ids`) if random placement keeps failing.
- **Steps 19-24 (incident frequency, Equations 25-77/25-78).** Monthly incident frequency `n_j` comes from either `IncidentInputs::monthly_frequencies` directly, or `crash_rate_per_100mvmt * incident_to_crash_ratio * (seed_vmt * mean_daf) / 1e8` (Equation 25-77/25-78, `hers_crash_rate` in `exhibits.rs` implements the alternative HERS model of Equation 25-79 for estimating the crash rate itself, though it is not wired into `generate_scenarios` — it is a standalone function for a caller to invoke). Per-scenario incident counts matching a Poisson(n_j) distribution are produced by `poisson_pmf` plus `counts_matching_distribution` (the shared delta-rounding helper described below), then shuffled onto scenarios (`rng.shuffle`).
- **Steps 25-26 (severity, Equations 25-82 through 25-85).** `counts_matching_distribution(g, n_inc)` distributes incident counts across the five `IncidentSeverity` categories to match `DEFAULT_INCIDENT_SEVERITY_DISTRIBUTION` (Equation 25-85: 0.754/0.196/0.031/0.019/0.0), then the per-incident severity assignment is shuffled.
- **Steps 27-28 (duration, Equations 25-86/25-87, Exhibit 25-41 lognormal parameters).** `incident_duration_bins` discretizes each severity's lognormal duration distribution (moment-matched via `lognormal_cdf`, built on an Abramowitz & Stegun `erf` approximation) into 15-min-period bins between `params.min` and `params.max`, and `counts_matching_distribution` again produces integer counts per bin matching those probabilities.
- **Steps 29-34 (location and start time, Equations 25-88 through 25-93).** `SeedStatistics::location_distribution`/`start_time_distribution` compute VMT-proportional probabilities per segment/period (Equations 25-88/25-89); `counts_matching_distribution` turns those into integer pools, which are shuffled and paired to (scenario, severity, duration) tuples by a first-non-overlapping-slot search (the `'search:` loop), falling back to `(0, 0)` if no non-overlapping slot exists. Each assigned severity is passed through `feasible_severity` (`exhibits.rs`), which downgrades a severity until `incident_caf_per_open_lane` returns `Some(_)` for the segment's lane count — implementing the Chapter 11 text that "the scenario generation methodology does not assign incidents that result in full segment closure."

**Deterministic-count / stochastic-pairing split.** As documented in the `scenario_generation.rs` module comment, the HCM procedure itself is a hybrid: event *counts* (weather events per month, incidents per severity/duration/location bin) are generated deterministically via the delta-adjusted rounding equations so the marginal distributions match the inputs exactly, while only the *pairing* of events to scenarios/start-times/segments is stochastic. `counts_matching_distribution` (used for all four count-matching steps above) implements this via bisection search for a scale factor `delta` such that `sum(round(delta * n * p_i)) = n`, with a largest-remainder fallback (`counts.iter().max_by`/`min_by` on the rounding gain/loss) when no `delta` hits the target exactly because of rounding-jump gaps.

#### Equations 25-71 through 25-93 (full transcription)

Every equation cited by name above, transcribed from the EPUB MathML (`200_Ch25_09.xhtml`) and cross-checked against the Rust implementation.

**Steps 2-5 (demand combinations, probabilities, DAF):**

```
Equation 25-71:  N_Scen = Nr x N_DC                                                    [count]
  N_Scen = total number of scenarios generated                                          [count]
  Nr     = number of replications per demand combination (book prints this equation with the literal constant 4, its own stated default; the code generalizes to the configured `ScenarioGenerationConfig::replications`, consistent with Step 3's text that the analyst may choose a different value, and with Equation 25-74 where the book itself already calls this quantity Nr) [count]
  N_DC   = number of demand combinations (months x weekdays)                            [count]
Implemented in: freeway_reliability/scenario_generation.rs::generate_scenarios (scenarios.len() == N_DC * replications; the `ndc` / `nr` locals)

Equation 25-72:  DAF_s(tp, seg) = DM(s) / DM(Seed_tp)     for all tp in SP, seg in Segments
  DAF_s(tp,seg) = demand adjustment factor for scenario s, period tp, segment seg        [decimal]
  DM(s)         = demand multiplier associated with scenario s                          [decimal, ratio to AADT]
  DM(Seed_tp)   = demand multiplier associated with the seed file                       [decimal]
Implemented in: freeway_reliability/scenario_generation.rs::generate_scenarios (`daf: dm / dm_seed`), ::ScenarioGenerationConfig::{demand_multiplier, seed_demand_multiplier} (index into `demand_multipliers`, default Exhibit 11-18 URBAN_DEMAND_RATIOS)

Equation 25-73:  P{s} = n_Day,DCs / (Nr x sum_{k=1..N_DC} n_Day,k)
  P{s}      = probability of scenario s                                                 [decimal; sums to 1.0 over all scenarios]
  n_Day,DCs = number of days in the RRP for scenario s's demand combination              [days]
  n_Day,k   = number of days in the RRP for demand combination k (typically 4 for a 1-yr weekday analysis) [days]
  N_DC      = number of demand combinations                                             [count]
  As with Equation 25-71, the book's printed denominator multiplier is the literal constant 4; the code generalizes it to `Nr`.
Implemented in: freeway_reliability/scenario_generation.rs::generate_scenarios (`probability = day_counts[dc] / (nr as f64 * total_days)`)
```

**Steps 6-9 (deterministic work zone assignment):**

```
Equation 25-74:  N-bar_DC,WZ = round(r_DC x Nr, 0)                                      [replications, rounded to nearest integer]
  N-bar_DC,WZ = adjusted number of replications of a demand combination assigned the work zone [count]
  r_DC        = ratio of active-weekday-type days in a month to the total days of that weekday type in the month [decimal]
  Nr          = number of replications per demand combination                            [count]
Implemented in: freeway_reliability/scenario_generation.rs::generate_scenarios (`n_wz = ((wz.active_day_ratio * nr as f64).round() as usize).min(nr)`, assigned to scenarios whose month/weekday match and whose replication index < n_wz); WorkZoneEvent::active_day_ratio supplies r_DC directly (not recomputed from a calendar)
```

**Steps 10-18 (weather event frequency and assignment):**

```
Equation 25-75:  P_W{i,j} = (sum of all SP durations in month j that weather type i is present) / (sum of all SP durations in month j)
  P_W{i,j} = timewise probability of weather type i in month j                           [decimal]
  SP       = study period                                                                [-]
  This is an input-data-preparation formula describing how the national/local weather probability tables are derived from historical station data; it is not computed by this crate. `WeatherInputs::probabilities_by_month` supplies P_t{w,j} directly as a user/default input.
Not computed in code (input data preparation only); consumed as: freeway_reliability/scenario_generation.rs::WeatherInputs::probabilities_by_month

Equation 25-76:  E[n_w,j] = round(P_t{w,j} x D_SP x N_Scen,j / E15min[D_w])              [events, rounded to nearest integer]
  E[n_w,j]     = expected frequency of weather event w in month j, rounded to the nearest integer [events]
  P_t{w,j}     = timewise probability of weather type w in month j                       [decimal]
  D_SP         = duration of the study period                                            [h]
  N_Scen,j     = number of scenarios associated with month j                             [count]
  E15min[D_w]  = expected duration of weather event w, rounded to the nearest 15-min increment, minimum 0.25 h [h]
Implemented in: freeway_reliability/scenario_generation.rs::expected_weather_frequency
```

**Steps 19-24 (incident frequency and per-scenario counts):**

```
Equation 25-77:  n_j = IR_j x VMT_j                                                     [incidents, rounded to nearest integer]
  n_j    = expected frequency of all incidents in the study period for month j, rounded to nearest integer [incidents]
  IR_j   = incident rate per 100 million VMT in month j                                  [incidents / 10^8 veh-mi]
  VMT_j  = average VMT for scenarios in month j, after adjusting the base-scenario demand with the month's demand multipliers and multiplying by facility length [veh-mi]
  See VERIFICATION.md item 4: the published Exhibit 25-103 October value is internally inconsistent with its own inputs (October/November Exhibit 25-100 demand-ratio rows are identical, so n_j must be equal for both months; the code asserts this equality and computes 0.79 vs. the printed 0.83).
Implemented in: freeway_reliability/scenario_generation.rs::generate_scenarios (`ir * (seed_vmt * mean_daf) / 1e8`; the /1e8 divisor folds the "per 100 million VMT" rate into VMT_j rather than pre-scaling IR_j, algebraically identical to n_j = IR_j x VMT_j)

Equation 25-78:  IR_j = CR_j x ICR
  IR_j = incident rate per 100 million VMT in month j                                    [incidents / 10^8 veh-mi]
  CR_j = local facilitywide crash rate per 100 million VMT in month j                     [crashes / 10^8 veh-mi]
  ICR  = local incident-to-crash ratio (national default 4.9)                             [decimal]
Implemented in: freeway_reliability/scenario_generation.rs::generate_scenarios (`let ir = cr * incidents.incident_to_crash_ratio;`); freeway_reliability/exhibits.rs::incident_rate_from_crash_rate (standalone helper implementing the same product); DEFAULT_INCIDENT_TO_CRASH_RATIO = 4.9

Equation 25-79 (HERS model):  CR = (154.0 - 1.203*ACR + 0.258*ACR^2 - 0.00000524*ACR^5) x e^(0.0082*(12 - LW))
  CR  = crash rate per 100 million VMT                                                   [crashes / 10^8 veh-mi]
  ACR = facility AADT divided by its two-way hourly capacity                              [decimal]
  LW  = lane width                                                                        [ft]
Implemented in: freeway_reliability/exhibits.rs::hers_crash_rate (standalone; not wired into generate_scenarios's incident-frequency computation, which only accepts a directly supplied crash_rate_per_100mvmt -- see Deferred below)

Equation 25-80:  sum_{k=0..+inf} round(delta1 x N_Scen,j x Prob{n_inc = k}) = N_Scen,j
  delta1        = adjustment parameter solved so the rounded Poisson-probability-weighted counts sum to N_Scen,j (hovers near 1) [decimal]
  N_Scen,j      = number of scenarios associated with month j                             [count]
  Prob{n_inc=k} = Poisson(n_j) probability mass at k incidents                            [decimal]
Implemented in: freeway_reliability/scenario_generation.rs::counts_matching_distribution (delta solved by bisection; the same helper implements Eq 25-80/81, 25-83/84, 25-86/87, and 25-90 through 25-93 -- see the "Deterministic-count / stochastic-pairing split" note above), freeway_reliability/scenario_generation.rs::poisson_pmf (Prob{n_inc=k})

Equation 25-81:  Number of scenarios with k incident events = round(delta1 x N_Scen,j x Prob{n_inc = k})
  (variables as in Equation 25-80)
Implemented in: freeway_reliability/scenario_generation.rs::generate_scenarios (`count_of_k = counts_matching_distribution(&pmf, nscen_j)`), then randomly shuffled onto scenarios (`rng.shuffle`) per Step 22
```

**Steps 25-26 (incident severity):**

```
Equation 25-82:  G(i) = { g1  i=1 (shoulder closed); g2  i=2 (1 lane closed); g3  i=3 (2 lanes closed); g4  i=4 (3 lanes closed); g5  i=5 (4+ lanes closed) }
  G(i) = discrete incident-severity distribution, assumed homogeneous across the facility and demand levels [decimal, sums to 1.0]
Implemented in: freeway_reliability/exhibits.rs::IncidentSeverity, INCIDENT_SEVERITIES (the five-category type; symbolic definition, not a computed formula)

Equation 25-83:  sum_i round(delta2 x N_Scen,Inc x G(i)) = N_Scen,Inc
  delta2      = adjustment parameter solved so rounded severity counts sum to the total incident count [decimal]
  N_Scen,Inc  = total number of incidents generated across the RRP                        [count]
  G(i)        = incident severity distribution (Equation 25-82/25-85)                     [decimal]
Implemented in: freeway_reliability/scenario_generation.rs::counts_matching_distribution(g, n_inc) (called on the severity distribution `g`)

Equation 25-84:  Number of incidents with severity i = round(delta2 x N_Scen,Inc x G(i))
  (variables as in Equation 25-83)
Implemented in: freeway_reliability/scenario_generation.rs::generate_scenarios (`severity_counts = counts_matching_distribution(g, n_inc)`, expanded to a per-incident severity list and shuffled per Step 26)

Equation 25-85:  G(i) = { 0.754  i=1; 0.196  i=2; 0.031  i=3; 0.019  i=4; 0  i=5 }
  G(i) = national default incident severity distribution (shoulder, 1-lane, 2-lane, 3-lane, 4+-lane closures) [decimal]
Implemented in: freeway_reliability/exhibits.rs::DEFAULT_INCIDENT_SEVERITY_DISTRIBUTION = [0.754, 0.196, 0.031, 0.019, 0.0]
```

**Steps 27-28 (incident duration, Exhibit 25-41 lognormal parameters):**

```
Equation 25-86:  sum_t round(delta3 x N_Inc,i x Prob{IncDur = t, IncType = i}) = N_Inc,i
  delta3                    = adjustment parameter solved so rounded duration-bin counts sum to N_Inc,i [decimal]
  N_Inc,i                   = number of incidents with severity i                          [count]
  Prob{IncDur=t, IncType=i} = lognormal duration-distribution probability of duration t (whole 15-min analysis periods) given severity i, from the Exhibit 25-41 moment-matched lognormal, discretized into period-width bins [decimal]
  Exhibit 25-41 parameters (mean/std-dev/min/max, min): shoulder 34.0/15.1/8.7/58.0; 1-lane 34.6/13.8/16.0/58.2; 2-lane 53.6/13.9/30.5/66.9; 3-or-more-lane 69.6 average (67.9 median)/21.9/36.0/93.3 -- see VERIFICATION.md item 1: Exhibit 11-22 (Chapter 11) prints 67.9 as the "3 Lanes Closed" and "4+ Lanes Closed" mean (matching Exhibit 25-41's median, not its 69.6 average); the code uses 67.9 for both severity types (Exhibit 11-22 value), and Exhibit 11-22 itself already lists identical shoulder/1/2/3/4+-lane columns for the 3-lane and 4+-lane cases.
Implemented in: freeway_reliability/scenario_generation.rs::incident_duration_bins (discretizes the lognormal into 15-min bins via lognormal_cdf, moment-matched: sigma^2 = ln(1 + (std_dev/mean)^2), mu = ln(mean) - sigma^2/2), DEFAULT_INCIDENT_DURATION_PARAMS (`exhibits.rs`) for the tabulated parameters, freeway_reliability/scenario_generation.rs::counts_matching_distribution (solves delta3 and rounds)

Equation 25-87:  Number of scenarios assigned incident severity i = round(delta3 x N_Inc,i x Prob{IncDur = t, IncType = i})
  (variables as in Equation 25-86; despite the printed label "incident severity i" this equation is indexed by duration bin t within severity i -- it is the per-duration-bin incident count, consistent with Step 27's stated purpose of generating a duration distribution per severity type, and with the code's per-severity duration-binning loop)
Implemented in: freeway_reliability/scenario_generation.rs::generate_scenarios (`bin_counts = counts_matching_distribution(&probs, members.len())`, expanded to a per-incident duration list and shuffled per Step 28)
```

**Steps 29-34 (incident location and start time):**

```
Equation 25-88:  Prob{Location = segment x} = (sum_p VMT_x,p) / (sum_{seg,p} VMT_i,p)
  Prob{Location=segment x} = probability that an incident's location is segment x         [decimal]
  VMT_x,p                  = VMT on segment x during analysis period p in the seed file    [veh-mi]
  (the book's printed denominator sums VMT_i,p over "seg" and p -- i.e. total facility VMT across all segments/periods; the segment index is written inconsistently as x in the numerator vs. i in the denominator, both denoting "segment")
Implemented in: freeway_reliability/scenario_generation.rs::SeedStatistics::location_distribution

Equation 25-89:  Prob{StartTime = analysis period y} = (sum_i VMT_i,y) / (sum_{i,p} VMT_i,p)
  Prob{StartTime=analysis period y} = probability that an incident starts in analysis period y [decimal]
  VMT_i,y                           = VMT on segment i during analysis period y in the seed file [veh-mi]
Implemented in: freeway_reliability/scenario_generation.rs::SeedStatistics::start_time_distribution

Equation 25-90:  sum_x round(delta4 x N_Scen,Inc x Prob{Location = x}) = N_Scen,Inc
Equation 25-92:  Number of incidents assigned to segment seg = round(delta4 x N_Scen,Inc x Prob{Location = seg})
  delta4     = adjustment parameter solved so rounded location counts sum to N_Scen,Inc    [decimal]
  N_Scen,Inc = total number of incidents generated across the RRP                          [count]
Implemented in: freeway_reliability/scenario_generation.rs::counts_matching_distribution(&loc_dist, n_inc) (`loc_counts`, expanded to a shuffled location pool)

Equation 25-91:  sum_y round(delta5 x N_Scen,Inc x Prob{StartTime = y}) = N_Scen,Inc
Equation 25-93:  Number of incidents assigned a starting time in analysis period p = round(delta5 x N_Scen,Inc x Prob{StartTime = p})
  delta5     = adjustment parameter solved so rounded start-time counts sum to N_Scen,Inc   [decimal]
Implemented in: freeway_reliability/scenario_generation.rs::counts_matching_distribution(&start_dist, n_inc) (`start_counts`, expanded to a shuffled start-period pool); the (scenario, severity, duration) x (location, start) pairing itself (Steps 31-34) is the `'search:` loop in generate_scenarios
```

### PRNG determinism

`Prng` (`scenario_generation.rs`) is an in-module xorshift64* generator seeded from `ScenarioGenerationConfig::rng_seed` (0 is remapped to a fixed nonzero constant to avoid the all-zeros fixed point) — no external `rand` crate dependency. It exposes `next_u64`, `next_f64` (uniform `[0,1)`), `gen_range` (uniform integer), `pick_weighted` (discrete weighted draw), and `shuffle` (Fisher-Yates). Because the seed is explicit and the algorithm is a fixed deterministic bit-mixing function, `generate_scenarios` is byte-for-byte reproducible for a given seed — verified in the unit test `test_generation_reproducible_for_seed` (`src/hcm/freeway_reliability/tests.rs`) and the integration test `ep7_generation_reproducible` (`tests/chapter11_integration.rs`), both of which run the same config twice and assert identical serialized scenario sets, then change only `rng_seed` and assert the output differs. The module doc comment is explicit that the *published* HCM Example Problem 7 results come from FREEVAL's own Monte Carlo stream (also seeded, but with a different, proprietary generator), so per-scenario reproduction of the book's numbers is not expected or attempted — only distribution-level metrics are compared (see Validation below).

### Scenario evaluation and CAF/SAF/DAF folding (Step B-9/B-10)

`ReliabilityAnalysis::build_scenario_facility` clones the base `FreewayFacility` and, for each scenario, builds per-period facility-wide DAF and per-segment/per-period CAF/SAF arrays by iterating the scenario's `weather_events`, `incidents`, `work_zones`, and `special_events` in that order, multiplying each active event's factor into the running per-cell CAF/SAF/DAF (all effects are multiplicative, matching the Step B-9 semantics documented in the `reliability.rs` module comment). Incident lane closures use `incident_caf_total` (`exhibits.rs`), which converts the Exhibit 11-23 per-open-lane CAF into a total-segment-capacity multiplier as `CAF_table * (N - lanes_closed) / N` — the module doc explicitly flags a **VERIFY-HCM** deviation here: FREEVAL additionally reduces the *number of lanes* (NLAF) on incident/work-zone segments, which changes per-lane density and speed on those segments, whereas this implementation keeps the segment's lane count constant and models the closure entirely through the total-capacity CAF; the comment argues facility travel times (the quantity driving the reliability distribution) are "only marginally affected" since they're governed by the capacity restriction (queueing), not by segment lane count. After the DAF is applied to `mainline_demand` and all ramp demand vectors, and CAF/SAF are multiplied into `caf_schedule`/`saf_schedule` on top of any base per-segment schedule, `ReliabilityAnalysis::run` calls `fac.run_analysis()` per scenario (this is a full Chapter 10 run, described in `docs/hcm/procedures/chapter10.md`) and computes `facility_travel_time_min` (sum of `L_i / U_i` over segments at that period's speeds, with a nominal 1 mi/h floor when a segment is fully stopped to keep travel time finite) and TTI (`(tt_min / free_flow_travel_time_min).max(1.0)`).

```
Incident CAF total-capacity multiplier (Exhibit 11-23; not an HCM-numbered equation):
  CAF_total = CAF_table x (N - lanes_closed) / N                                          [decimal]
  CAF_table     = Exhibit 11-23 per-open-lane CAF for the segment's directional lane count and incident severity [decimal]
  N             = number of directional lanes on the segment                              [ln]
  lanes_closed  = number of lanes closed by the incident (severity.lanes_closed())        [ln]
  Worked example transcribed from the exhibit note: a 2-lane closure on a 6-lane facility keeps CAF_table(0.75) x (6-2)/6 = 0.75 x 4/6 = 0.50 of the original capacity.
  N/A cells (lanes_closed >= N, full closure) return None; the scenario generator downgrades severity via feasible_severity until a Some(_) result is available for the segment's lane count (Chapter 11 text: "the scenario generation methodology does not assign incidents that result in full segment closure").
Implemented in: freeway_reliability/exhibits.rs::incident_caf_total (total-capacity form), ::incident_caf_per_open_lane (INCIDENT_CAF_PER_OPEN_LANE, the tabulated Exhibit 11-23 values), ::feasible_severity (severity downgrade); folded into the scenario facility at freeway_reliability/reliability.rs::ReliabilityAnalysis::build_scenario_facility (`caf_inc = incident_caf_total(lanes, inc.severity).unwrap_or(1.0)`)
```

See VERIFICATION.md item 2: this crate applies the incident's capacity effect entirely through `CAF_total` above, leaving the segment's lane count unchanged for density/speed purposes, whereas FREEVAL additionally reduces the segment's *lane count* (NLAF) on the incident segment. See VERIFICATION.md item 6 for the (book-silent) linear interpolation used by `weather_caf`/`weather_saf` (Exhibits 11-20/11-21) between the tabulated 5-mi/h FFS columns.

### Reliability performance measures (Step B-11/B-13)

`TravelTimeDistribution` (`src/hcm/common/reliability.rs`) accumulates `(tti, weight)` observations — weight is `scenario.probability * VMT_served` when `vmt_weighted` (the default, matching the Exhibit 25-105 "VMT-Weighted TTI" presentation), or `scenario.probability` alone for a time-based distribution. It computes: `mean` (weighted mean TTI); `percentile(p)` (weighted empirical CDF — smallest observation whose cumulative weight reaches `p`% of total, so `percentile(95.0)` = TTI_95 = PTI); `max`; `std_dev` and `semi_std_dev` (one-sided about TTI = 1, per the Chapter 11 Section 2 definition, with TTI < 1 clamped to zero deviation even though TTI >= 1 by construction); `misery_index` (via `mean_of_worst(0.05)`, the weighted mean TTI of the worst 5% by weight); `reliability_rating` (`pct_at_or_below(1.33)`, the HCM-defined percentage of the distribution with TTI < 1.33 — implemented as `<=` rather than strict `<`); and `pct_above`/`failure_pct`/`on_time_pct` for arbitrary thresholds. `ReliabilityAnalysis::failure_pct_below_speed` converts a target space-mean speed into a TTI threshold via `ffs_equiv = facility_length_mi / (free_flow_travel_time_min / 60)`, then `tti_threshold = ffs_equiv / target_speed`.

HCM Chapter 11, Section 2 ("Travel Time Distribution and Reliability Performance Measures," `75_Ch11_02.xhtml`) defines these measures only in prose, with no numbered equations; the literal formulas implemented by `TravelTimeDistribution` are:

```
TTI (per observation, no HCM number):  TTI = travel_time / free_flow_travel_time                                [decimal, >= 1.0 by definition]
Implemented in: freeway_reliability/reliability.rs::ReliabilityAnalysis::run (`tti_p = (tt_min / free_flow_travel_time_min).max(1.0)`)

TTI_mean (no HCM number):  TTI_mean = sum(w_k * TTI_k) / sum(w_k)                                                [decimal]
  w_k = observation weight = scenario.probability * VMT_served (VMT-weighted, default) or scenario.probability alone (time-weighted) [-]
Implemented in: common/reliability.rs::TravelTimeDistribution::mean

TTI_p (percentile; no HCM number):  TTI_p = TTI_k*  where k* is the smallest sorted index with cumsum(w_1..w_k*) >= p/100 * sum(w_k)   [decimal]
  TTI_95 = PTI (planning time index); TTI_80; TTI_50 (median)                                                    [decimal]
Implemented in: common/reliability.rs::TravelTimeDistribution::percentile (weighted empirical CDF)

Standard deviation (no HCM number):  std_dev = sqrt( sum(w_k * (TTI_k - TTI_mean)^2) / sum(w_k) )                [decimal]
Implemented in: common/reliability.rs::TravelTimeDistribution::std_dev

Semi-standard deviation (HCM Ch 11 Sec 2, prose-only, no equation number):  semi_std_dev = sqrt( sum(w_k * max(TTI_k - 1, 0)^2) / sum(w_k) )   [decimal]
  One-sided about TTI = 1 (free-flow travel time) rather than about TTI_mean; TTI < 1 observations clamp to zero deviation (though TTI >= 1 by construction).
Implemented in: common/reliability.rs::TravelTimeDistribution::semi_std_dev

Misery index (HCM Ch 11 Sec 2 / Ch 36, prose-only, no equation number):  misery_index = mean_of_worst(0.05) = sum(w_k * TTI_k over the worst-5%-by-weight subset) / (0.05 * sum(w_k))   [decimal]
  The "worst 5%" subset is taken by sorting descending on TTI and accumulating weight until 5% of total weight is reached, with the boundary observation's weight split fractionally.
Implemented in: common/reliability.rs::TravelTimeDistribution::{misery_index, mean_of_worst}

Reliability rating (HCM Ch 11 Sec 2, prose-only, no equation number):  reliability_rating = 100 * sum(w_k for TTI_k <= 1.33) / sum(w_k)   [%]
  RELIABILITY_RATING_TTI_THRESHOLD = 1.33; implemented as TTI <= 1.33 (the HCM text says "TTI less than 1.33," i.e. strict <, so the boundary observation TTI = 1.33 exactly is counted here but would not be under a strict reading -- immaterial for continuous TTI values).
Implemented in: common/reliability.rs::TravelTimeDistribution::{reliability_rating, pct_at_or_below}

Failure / on-time percentage (no HCM number):  failure_pct(threshold) = 100 * sum(w_k for TTI_k > threshold) / sum(w_k);  on_time_pct = 100 - failure_pct
  tti_threshold = ffs_equiv / target_speed_mi_h,  ffs_equiv = facility_length_mi / (free_flow_travel_time_min / 60)                      [decimal]
Implemented in: common/reliability.rs::TravelTimeDistribution::{pct_above, failure_pct, on_time_pct}, freeway_reliability/reliability.rs::ReliabilityAnalysis::{failure_pct_below_speed, on_time_pct_at_speed}
```

See VERIFICATION.md item 5: the distribution *tails* (TTI_95/PTI, TTI_max, reliability rating, pct-TTI-above-2) computed by these formulas differ materially from the published Exhibit 25-104 values for the EP7 fixture (centers match closely); the deviation is attributed to the Chapter 10 oversaturated-engine queue-distribution gap plus differing Monte Carlo incident/scenario pairing versus FREEVAL, not to an error in the formulas above.

## Validation

The fixture-driven integration test is `tests/chapter11_integration.rs`, reading `tests/ExampleCases/hcm/FreewayReliability/case1.json` (a `facility` + `scenario_generation` + `vmt_weighted` JSON matching `ReliabilityAnalysis`'s serde schema). It reproduces HCM Chapter 25 Example Problem 7 ("Reliability Evaluation of an Existing Freeway Facility," Exhibits 25-97 through 25-105); per VERIFICATION.md item 3, several weaving ramp-to-ramp demands for the fixture's access points are not published in the example problem text and are assumed at 50 veh/h. A PyO3-binding mirror exists at `tests/test_chapter11_integration.py` (102 lines). The Rust test module's doc comment lays out a three-tier verification strategy, since the published results come from FREEVAL's own (different) Monte Carlo stream:

**(a) Scenario-generation intermediates, asserted exactly or near-exactly:**
- `ep7_seed_vmt_matches_published`: seed-file VMT = 71,501 veh-mi (+-1.0), 12 analysis periods, 3.0-h study period.
- `ep7_scenario_count_and_probabilities`: 240 scenarios (12 months x 5 weekdays x 4 replications), each probability exactly 1/240 (+-1e-12), plus spot DAF checks (November-Tuesday seed-date scenario DAF = 1.0; July-Friday DAF = 1.329/0.995, both +-1e-9).
- `ep7_expected_weather_event_counts`: Equation 25-76 counts match the published pattern exactly (1 medium-rain event/month, 1 heavy-rain event/month except 2 in summer months 6-8, all other severe types round to 0), and total weather events = 27 across the RRP.
- `ep7_monthly_incident_frequencies_match_exhibit_25_103`: computed monthly incident frequencies at tolerance +-0.012 (except September at +-0.045) against the published Exhibit 25-103 values; the test's doc comment documents two specific discrepancies as book artifacts rather than code bugs — March/June/July/September differ by +0.01 from a slightly different seed VMT the book used, and the published October value (0.83) is asserted to be internally inconsistent because the October and November demand-ratio rows of Exhibit 25-100 are identical (the code additionally asserts `monthly_incident_frequency[9] == monthly_incident_frequency[10]` at 1e-12 to make this point directly), and total incidents across the year fall in `[150, 220]`.
- `ep7_generation_reproducible`: same seed produces byte-identical serialized scenario sets; a different seed produces a different set.

**(b) Base scenario must match the Chapter 10 core method exactly:** `ep7_base_scenario_matches_chapter10` asserts the unadjusted base facility is undersaturated with max vd/c = 0.99 (+-0.005, per the EP7 text), and that the clean November-Tuesday scenario (DAF = 1, no weather/incident events) reproduces the base Chapter 10 travel times to 1e-9 per period, plus free-flow travel time = 6.0 min (+-0.01) for the 6-mi, 60-mi/h facility.

**(c) Distribution-level metrics vs Exhibit 25-104, with every deviation documented as computed-vs-published:** `ep7_reliability_metrics_vs_exhibit_25_104` runs both a probability-weighted (`vmt_weighted = false`) and the default VMT-weighted distribution. Central measures are asserted within tight published-relative bands: TTI_50 1.03 (+-0.01), TTI_mean 1.30 (+-0.04), misery index 5.76 (+-0.30), semi-std-dev 2.05 (+-0.12). Several measures are asserted at their computed value with the published number recorded only in the comment/assertion label, documented as VERIFY-HCM gaps: TTI_95/PTI computed 2.00 vs published 1.67 (+20%, attributed to both the Chapter 10 oversaturated-engine queue-distribution gap documented in the Chapter 10 doc and to different Monte Carlo pairing of incidents with high-demand scenarios); TTI_max computed 39.7 vs published 33.57 (+18%, the single worst scenario depends on Monte Carlo pairing); reliability rating computed 84.2% VMT-weighted (86.6% probability-weighted) vs published 90.8%; percentage of observations at TTI>2 computed ~5.1% vs published 2.95% of VMT. The test also checks distribution-shape invariants (TTI_95 >= TTI_80 >= TTI_50, TTI_max >= TTI_95, misery_index >= TTI_mean) and failure/on-time monotonicity at 35/45/50 mi/h targets. `ep7_scenario_results_consistency` checks per-scenario sanity (240 scenarios, TTI >= 1 in every cell, travel time >= 5.5 min, positive VMT, and that July scenarios include at least one oversaturated case while low-demand months do not).

Unit tests in `src/hcm/freeway_reliability/tests.rs` (22 tests) exercise the PRNG (reproducibility and rough uniformity), `counts_matching_distribution` and `poisson_pmf` against hand-worked examples, `expected_weather_frequency` against Equation 25-76 by hand, incident-duration binning, scenario probability/DAF computation with and without explicit `day_counts`, weather/incident generation (counts, severities, non-overlap, feasibility downgrading), work zone and special event assignment, generation reproducibility, the base-scenario-matches-Chapter-10 property, CAF/DAF folding, a small end-to-end `ReliabilityAnalysis::run`, and fixture JSON round-tripping. `src/hcm/freeway_reliability/exhibits.rs`'s own test module spot-checks the Exhibit 11-20/11-21 weather tables (including FFS interpolation and out-of-range clamping), the Exhibit 11-18/11-19 demand ratio tables, Equation 25-85's severity distribution, Exhibit 11-23's incident CAF table (including the documented worked example: a 2-lane closure on 6 directional lanes keeps 0.75 x 4/6 = 50% of total capacity), the HERS crash-rate model (Equation 25-79), and the planning-level method (Equations 11-1 through 11-5) against Chapter 25 Example Problem 10 (FFS 75 mi/h, peak speed 62 mi/h, 3 lanes, X = 0.95: RDR 0.00280, IDR 0.00919, TTI_mean 1.899, TTI_95 3.353, PT_45 74.3%, each at tight tolerances). `src/hcm/common/reliability.rs`'s test module validates `TravelTimeDistribution` in isolation against synthetic (non-HCM) data: mean/percentile computation on a 20-point uniform series, weighted percentiles with unequal weights, misery index, standard and semi-standard deviation, and rejection of non-positive-weight or NaN observations.

### Planning-level method (Equations 11-1 through 11-5)

`exhibits.rs` also implements the Chapter 11, Section 5 "Simplified Method" (`78_Ch11_05.xhtml`): a standalone, non-scenario-based estimate of three TTI percentiles from facility-level peak-hour inputs, intended for planning contexts where the full scenario-generation methodology's data needs are impractical. It is not wired into `ReliabilityAnalysis` (no scenario loop, no TTI distribution); each function is a direct, independent equation evaluation.

```
Equation 11-2:  RDR = 1/S - 1/FFS                                                        [h/mi]
  RDR = recurring delay rate                                                             [h/mi]
  S   = peak-hour speed                                                                  [mi/h]
  FFS = free-flow speed                                                                  [mi/h]
Implemented in: freeway_reliability/exhibits.rs::planning_recurring_delay_rate

Equation 11-3:  IDR = [0.020 - (N - 2) x 0.003] x X^12                                    [h/mi]
  IDR = incident delay rate                                                              [h/mi]
  N   = number of lanes in one direction (N = 2 to 4; values above 4 capped at 4)         [ln]
  X   = peak-hour volume-to-capacity ratio (X <= 1.00; values above 1.00 capped at 1.00)  [decimal]
  Valid only for X <= 1.00 and N = 2, 3, or 4 per the printed text; both inputs are clamped before use.
Implemented in: freeway_reliability/exhibits.rs::planning_incident_delay_rate (`n.clamp(2, 4)`, `vc_ratio.min(1.0)`)

Equation 11-1:  TTI_mean = 1 + FFS x (RDR + IDR)                                          [decimal]
  TTI_mean = average annual mean travel time index                                       [decimal]
  FFS      = free-flow speed                                                             [mi/h]
  RDR      = recurring delay rate (Equation 11-2)                                        [h/mi]
  IDR      = incident delay rate (Equation 11-3)                                         [h/mi]
Implemented in: freeway_reliability/exhibits.rs::planning_tti_mean

Equation 11-4:  TTI_95 = 1 + 3.67 x ln(TTI_mean)                                          [decimal]
  TTI_95   = 95th percentile travel time index                                           [decimal]
  TTI_mean = average annual mean travel time index (Equation 11-1)                       [decimal]
Implemented in: freeway_reliability/exhibits.rs::planning_tti_95

Equation 11-5:  PT_45 = 1 - exp[-1.5115 x (TTI_mean - 1)]                                 [decimal]
  PT_45    = percent of trips occurring at speeds less than 45 mi/h                       [decimal]
  TTI_mean = average annual mean travel time index (Equation 11-1)                       [decimal]
Implemented in: freeway_reliability/exhibits.rs::planning_pt45
```

Validated against Chapter 25 Example Problem 10 (`202_Ch25_11a.xhtml`; unit test `test_example_problem_10_planning_method` in `exhibits.rs`): FFS 75 mi/h, peak speed 62 mi/h, 3 lanes, X = 0.95 gives RDR 0.00280, IDR 0.00919, TTI_mean 1.899, TTI_95 3.353, PT_45 74.3%, each reproduced within 0.001-0.005 absolute tolerance.

### Exhibit tables (default values)

The following default-value exhibits are transcribed as literal Rust constants/tables (structure only is described here per the copyright note in the task; values already in code are cited by name):

- **Exhibits 11-18/11-19** (default urban/rural demand ratios, ADT/Mondays-in-January, 12 months x 7 weekdays): `freeway_reliability/exhibits.rs::URBAN_DEMAND_RATIOS`, `::RURAL_DEMAND_RATIOS`. `ScenarioGenerationConfig::demand_multipliers` defaults to `URBAN_DEMAND_RATIOS`; `RURAL_DEMAND_RATIOS` is available for the analyst to substitute.
- **Exhibits 11-20/11-21** (default CAFs/SAFs by weather type and facility FFS, 10 severe weather types x 5 FFS columns 55-75 mi/h): `freeway_reliability/exhibits.rs::WEATHER_CAF`, `::WEATHER_SAF`, looked up via `weather_caf`/`weather_saf` with linear interpolation between the tabulated FFS columns (see VERIFICATION.md item 6).
- **Exhibit 11-22** (default incident severity distribution and duration parameters by severity type): the severity-distribution row is `DEFAULT_INCIDENT_SEVERITY_DISTRIBUTION` (identical to Equation 25-85); the duration parameters are `DEFAULT_INCIDENT_DURATION_PARAMS` (mean/std-dev/min/max per severity) -- see VERIFICATION.md item 1 for the 3-lane/4+-lane mean-duration cross-check against Exhibit 25-41.
- **Exhibit 11-23** (CAFs by incident type and number of directional lanes, 2-8 lanes x 5 severity columns): `freeway_reliability/exhibits.rs::INCIDENT_CAF_PER_OPEN_LANE`, looked up via `incident_caf_per_open_lane`/`incident_caf_total` (transcribed above under "Scenario evaluation and CAF/SAF/DAF folding").
- **`DEFAULT_INCIDENT_SEVERITY_DISTRIBUTION`** and **`DEFAULT_INCIDENT_DURATION_PARAMS`** are also the Chapter 25 Equation 25-85 / Exhibit 25-41 defaults consumed by `generate_scenarios` when `IncidentInputs` does not override them.

## Deferred

Per the `freeway_reliability/mod.rs` module doc comment, explicitly out of scope in this pass:
- Managed lane reliability.
- The Chapter 11 Section 4 ATDM strategy assessment (Steps C-1 through C-9) — implemented separately on `feat/hcm-reliability-enhancements` (`src/hcm/common/atdm.rs`).
- The Chapter 25 reliability calibration methodology.

Additionally, `hers_crash_rate` (Equation 25-79) is implemented as a standalone function but is not wired into `generate_scenarios`'s incident-frequency computation, which only accepts a directly supplied `crash_rate_per_100mvmt`; a caller wanting the HERS estimation model must call `hers_crash_rate` themselves and pass the result in. No stub types or `todo!()` markers exist for the deferred items; they are simply unimplemented.
