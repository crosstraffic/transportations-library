# Chapter 13: Freeway Weaving Segments — Procedure Walkthrough

HCM 7th Edition Chapter 13 analyzes freeway weaving segments (and weaving segments on multilane highways and collector-distributor roads) through the core methodology of Steps 2-8: demand adjustment, configuration characteristics, maximum weaving length, capacity, lane-changing rates, speeds, and density/LOS. The implementation is `src/hcm/weaving/weaving.rs` on the `feat/hcm-ch12-14-completion` branch, a single `WeavingSegment` struct in the chapter15 house style (plain input fields, `Option<T>` computed fields populated by step methods in HCM order, `get_`/`set_` accessors, and a `run_analysis()` that chains Steps 2-8). This branch's commit `e921e1e` rewrote the module with HCM-verified equations; the headline correction is the Equation 13-22 space-mean (harmonic) speed, which replaced an arithmetic weighted mean. A PyO3 wrapper (`copython::weaving::WeavingSegment`, in `src/copython/weaving.rs` — renamed from `src/copython/chapter13.rs` with the topic-folder rename) is registered on this branch.

## Step-by-step walkthrough

| Manual step | HCM Eq./Exhibit | Rust method | Inputs (units) | Output (units) |
|---|---|---|---|---|
| Step 1: inputs | — | struct fields | `length_short` L_S (ft), `num_lanes` N, `num_weaving_lanes` N_WL, `ffs` (mi/h), component volumes `v_ff`/`v_fr`/`v_rf`/`v_rr` (veh/h), `phf`, `heavy_vehicle_pct` (decimal), `terrain`, `lc_rf`/`lc_fr`/`lc_rr` (lc), `interchange_density` (int/mi), `basic_freeway_capacity` c_IFL (pc/h/ln), `caf`, `saf` | — |
| Step 2: adjust volumes | Eq 13-1 (`v_i = V_i/(PHF·f_HV)`), f_HV per Eq 12-10 with Exhibit 12-25 PCEs | `determine_demand_flow` (private `calculate_fhv`) | volumes veh/h | `(v_W, v_NW, v)` pc/h, plus `volume_ratio` VR = v_W/v |
| Step 3: configuration | Eq 13-2 (one-sided, `LC_MIN = LC_RF·v_RF + LC_FR·v_FR`), Eq 13-3 (two-sided, `LC_MIN = LC_RR·v_RR`) | `determine_configuration_characteristics` | flows pc/h | LC_MIN, lc/h |
| Step 4: max weaving length | Eq 13-4 (`L_MAX = 5728(1+VR)^1.6 − 1566·N_WL`) | `determine_max_weaving_length` | VR, N_WL | L_MAX ft; sets `is_weaving = (L_S < L_MAX)` |
| Step 5: capacity | Eq 13-5 (density criterion `c_IWL = c_IFL − 438.2(1+VR)^1.6 + 0.0765·L_S + 119.8·N_WL`), Eq 13-6 (`c_W = c_IWL·N·f_HV`), Eq 13-7 (weaving-flow criterion, 2400/VR for N_WL=2, 3500/VR for N_WL=3, no limit for two-sided N_WL=0), Eq 13-8 (`c_W = c_IW·f_HV`), Eq 13-9 (`c_wa = min(...)·CAF`), Eq 13-10 (`v/c = v·f_HV/c_wa`) | `determine_capacity` | VR, L_S, N_WL, f_HV, CAF | c_wa veh/h; sets `vc_ratio` and `demand_exceeds_capacity` |
| Step 6: lane-changing rates | Eq 13-11 (`LC_W = LC_MIN + 0.39·(L_S−300)^0.5·N²·(1+ID)^0.8`, with L_S−300 floored at 0 per the ≤300 ft rule), Eq 13-12 (`I_NW = L_S·ID·v_NW/10000`), Eq 13-13 (LC_NW1, floored at 0), Eq 13-14 (LC_NW2), Eq 13-15 (interpolation), Eq 13-16 (four-branch selection), Eq 13-17 (`LC_ALL = LC_W + LC_NW`) | `determine_lane_changing_rates` | LC_MIN, v_NW, N, ID, L_S | LC_ALL lc/h |
| Step 7: speeds | Eq 13-20 (`W = 0.226(LC_ALL/L_S)^0.789`), Eq 13-19 (`S_W = 15 + (FFS·SAF − 15)/(1+W)`, 15 mi/h = S_MIN of Eq 13-18), Eq 13-21 (`S_NW = FFS·SAF − 0.0072·LC_MIN − 0.0048·v/N`), Eq 13-22 (space mean `S = v/(v_W/S_W + v_NW/S_NW)`) | `estimate_speed` | LC_ALL, LC_MIN, flows, FFS, SAF, N | `(S_W, S_NW, S)` mi/h |
| Step 8a: density | Eq 13-23 (`D = (v/N)/S`) | `determine_density` | v pc/h, N, S mi/h | D pc/mi/ln |
| Step 8b: LOS | Exhibit 13-6 | `determine_los` (free function `determine_weaving_los`) | D, over-capacity flag, `facility_type` | `LevelOfService` |

Two-sided weaving segments are handled through the `WeavingType` enum: `nwl()` forces N_WL = 0, Step 2 assigns only the ramp-to-ramp flow as weaving (`v_W = v_RR`, `v_NW = v_FF + v_FR + v_RF`), Step 3 uses Eq 13-3, and Step 5's weaving-flow criterion is skipped (`capacity_weaving = None`). The Exhibit 13-6 LOS table distinguishes freeway thresholds (A≤10 / B≤20 / C≤28 / D≤35 / E≤43 pc/mi/ln) from multilane/C-D thresholds (12/24/32/36/40) via the `FacilityType` enum; F is assigned above the E bound or whenever demand exceeds capacity. The same Exhibit 13-6 thresholds are also transcribed independently as `los_weaving` / `WeavingFacilityType` in `src/hcm/common/los_tables.rs` (with its own boundary unit tests); `WeavingSegment::determine_los` does not call through that common helper, it uses the free function `determine_weaving_los` defined alongside the struct in `weaving/weaving.rs`, so the two are duplicate-but-consistent transcriptions of the same exhibit rather than a caller/callee pair.

## Equations by step

Every equation below is cross-checked against the HCM 7th Edition MathML source (`resources/epub/OEBPS/90_Ch13_03.xhtml`, Equations 13-1 through 13-23) and against the constants and formulas actually implemented in `src/hcm/weaving/weaving.rs`. All 23 equations agree between the manual and the code exactly as shown; no new discrepancies were found beyond the four already recorded in the fix-history section below and in `docs/hcm/VERIFICATION.md`'s "Chapter 12–14" entries.

### Step 2: Adjust volume

```
Equation 13-1:  v_i = V_i / (PHF × f_HV)
  v_i = flow rate for movement i under equivalent ideal conditions, pc/h
  V_i = hourly demand volume for movement i under prevailing conditions, veh/h
  PHF = peak hour factor, decimal (Exhibit 13-7 default = 0.94 urban and rural; code struct default phf = 0.94)
  f_HV = heavy vehicle adjustment factor, decimal, per Chapter 12/Equation 12-10 form 1 / (1 + P_HV·(E_T − 1))
  i ∈ {FF, FR, RF, RR} (component movements), combined into W (weaving) and NW (nonweaving) per Exhibits 13-9/13-10
Implemented in: weaving/weaving.rs::determine_demand_flow (f_HV itself via the private calculate_fhv, with E_T = 2.0 level / 3.0 rolling from Exhibit 12-25, and the non-HCM E_T = 5.0 mountainous stand-in — see Deviations item 1)
```

One-sided segments aggregate `v_W = v_RF + v_FR` and `v_NW = v_FF + v_RR` (Exhibit 13-9); two-sided segments use `v_W = v_RR` and `v_NW = v_FF + v_FR + v_RF` (Exhibit 13-10), both per Equation 13-1 applied component-by-component before aggregation.

### Step 3: Determine configuration characteristics

```
Equation 13-2 (one-sided):  LC_MIN = (LC_RF × v_RF) + (LC_FR × v_FR)
  LC_MIN = minimum rate at which weaving vehicles must change lanes to complete all weaving maneuvers, lc/h
  LC_RF = minimum lane changes for one ramp-to-freeway vehicle, lc (Exhibit 13-7 default = 1; code field lc_rf)
  v_RF = ramp-to-freeway flow rate, pc/h
  LC_FR = minimum lane changes for one freeway-to-ramp vehicle, lc (Exhibit 13-7 default = 1; code field lc_fr)
  v_FR = freeway-to-ramp flow rate, pc/h
Implemented in: weaving/weaving.rs::determine_configuration_characteristics
```

```
Equation 13-3 (two-sided):  LC_MIN = LC_RR × v_RR
  LC_RR = minimum lane changes for one ramp-to-ramp vehicle, lc (Exhibit 13-7 default = 0; code field lc_rr)
  v_RR = ramp-to-ramp flow rate, pc/h
Implemented in: weaving/weaving.rs::determine_configuration_characteristics
```

N_WL (number of lanes from which a weaving maneuver can be made with one or no lane changes) is 2 or 3 for one-sided segments per Exhibit 13-5 (the code field `num_weaving_lanes`, an analyst-supplied input, not computed), and is always 0 for two-sided segments (the `nwl()` helper forces this via `WeavingType::TwoSided`).

### Step 4: Determine maximum weaving length

```
Equation 13-4:  L_MAX = 5,728·(1 + VR)^1.6 − 1,566·N_WL
  L_MAX = maximum weaving segment length, ft
  VR = volume ratio = v_W / v (unitless)
  N_WL = number of lanes from which a weaving maneuver can be made with one or no lane change, integer
Implemented in: weaving/weaving.rs::determine_max_weaving_length
```

If L_S < L_MAX the segment is analyzed as a weaving segment (continue to Step 5); if L_S ≥ L_MAX the manual directs the analyst to Chapter 14 merge/diverge methodology instead. `determine_max_weaving_length` sets `is_weaving = (L_S < L_MAX)` but `run_analysis` does not branch on it — see Deviations item 4. Exhibit 13-11's VR/N_WL spot values are transcribed as an inline unit test (`test_max_weaving_length`, VR = 0.3 → 5,584 ft at N_WL = 2 and 4,018 ft at N_WL = 3, both ±5 ft).

### Step 5: Determine weaving segment capacity

```
Equation 13-5:  c_IWL = c_IFL − 438.2·(1 + VR)^1.6 + 0.0765·L_S + 119.8·N_WL
  c_IWL = capacity per lane of the weaving segment under equivalent ideal conditions, pc/h/ln
  c_IFL = capacity per lane of a basic freeway segment with the same FFS under ideal conditions, pc/h/ln (code field basic_freeway_capacity; struct default = 2,400 pc/h/ln)
  VR = volume ratio (unitless)
  L_S = short length of the weaving segment, ft (code field length_short)
  N_WL = number of weaving lanes, integer (0, 2, or 3)
Implemented in: weaving/weaving.rs::determine_capacity (c_iwl)
```

```
Equation 13-6:  c_W = c_IWL × N × f_HV
  c_W = capacity of the weaving segment under prevailing conditions, density criterion, veh/h
  N = number of lanes within the weaving segment, ln (code field num_lanes)
  f_HV = heavy vehicle adjustment factor, decimal
Implemented in: weaving/weaving.rs::determine_capacity (capacity_density)
```

```
Equation 13-7:  c_IW = 2,400 / VR  for N_WL = 2 lanes
                c_IW = 3,500 / VR  for N_WL = 3 lanes
  c_IW = capacity of all lanes in the weaving segment under ideal conditions, weaving-flow criterion, pc/h
  VR = volume ratio (unitless)
  No limiting value is defined for two-sided segments (N_WL = 0)
  Constants: MAX_WEAVING_FLOW_NWL2 = 2,400.0 pc/h, MAX_WEAVING_FLOW_NWL3 = 3,500.0 pc/h
Implemented in: weaving/weaving.rs::determine_capacity (capacity_weaving; combined directly with Eq 13-8 into `MAX_WEAVING_FLOW_NWLx / vr * f_hv`; returns None for two-sided N_WL = 0, matching "no limiting value")
```

```
Equation 13-8:  c_W = c_IW × f_HV
  c_W = capacity of the weaving segment under prevailing conditions, weaving-flow criterion, veh/h
Implemented in: weaving/weaving.rs::determine_capacity (folded into the Eq 13-7 computation, per the harmonic-speed-fix commit e921e1e — see fix history below)
```

```
Equation 13-9:  c_wa = c_W × CAF
  c_wa = adjusted capacity of the weaving segment, veh/h
  c_W = min(Equation 13-6, Equation 13-8), unadjusted capacity under prevailing conditions, veh/h
  CAF = capacity adjustment factor, decimal (struct default caf = 1.0; Chapter 11 weather/incident/driver-population adjustments)
Implemented in: weaving/weaving.rs::determine_capacity (capacity)
```

```
Equation 13-10:  v/c = (v × f_HV) / c_wa
  v/c = volume-to-capacity ratio, decimal
  v = total demand flow rate = v_W + v_NW, pc/h
  f_HV = heavy vehicle adjustment factor, decimal
  c_wa = adjusted capacity, veh/h
Implemented in: weaving/weaving.rs::determine_capacity (vc_ratio; demand_exceeds_capacity = vc_ratio > 1.0, which forces LOS F in Step 8)
```

### Step 6: Determine lane-changing rates

```
Equation 13-11:  LC_W = LC_MIN + 0.39·[(L_S − 300)^0.5 · N² · (1 + ID)^0.8]
  LC_W = total rate of lane changing by weaving vehicles, lc/h
  LC_MIN = minimum lane-changing rate from Step 3, lc/h
  L_S = length of the weaving segment (short length definition), ft; 300 ft is used for all L_S ≤ 300 ft (constant MIN_WEAVING_LENGTH = 300.0)
  N = number of lanes within the weaving segment, ln
  ID = interchange density, int/mi (Exhibit 13-7 default: urban 0.8/mi, rural 0.4/mi; code field interchange_density, default 0.8)
Implemented in: weaving/weaving.rs::determine_lane_changing_rates (lc_w; the (L_S − 300) floor is applied via `.max(0.0)` on `ls_adj`)
```

```
Equation 13-12:  I_NW = (L_S × ID × v_NW) / 10,000
  I_NW = nonweaving vehicle index, unitless
  v_NW = nonweaving demand flow rate, pc/h
Implemented in: weaving/weaving.rs::determine_lane_changing_rates (i_nw, local to the method)
```

```
Equation 13-13:  LC_NW1 = 0.206·v_NW + 0.542·L_S − 192.6·N
  LC_NW1 = nonweaving lane-changing rate, lc/h, valid for I_NW ≤ 1,300 (the majority of cases)
  Can be arithmetically negative; the manual specifies the minimum must be externally set at 0
Implemented in: weaving/weaving.rs::determine_lane_changing_rates (lc_nw1, floored at 0 via `.max(0.0)` — see Deviations item 2)
```

```
Equation 13-14:  LC_NW2 = 2,135 + 0.223·(v_NW − 2,000)
  LC_NW2 = nonweaving lane-changing rate, lc/h, valid for I_NW ≥ 1,950
Implemented in: weaving/weaving.rs::determine_lane_changing_rates (lc_nw2)
```

```
Equation 13-15:  LC_NW3 = LC_NW1 + (LC_NW2 − LC_NW1) · [(I_NW − 1,300) / 650]
  LC_NW3 = interpolated nonweaving lane-changing rate, lc/h, used for 1,300 < I_NW < 1,950
  Only valid when LC_NW1 < LC_NW2; if LC_NW1 ≥ LC_NW2, LC_NW2 is used instead (see Equation 13-16)
Implemented in: weaving/weaving.rs::determine_lane_changing_rates (inline interpolation branch inside the lc_nw match, not a separate named binding)
```

```
Equation 13-16 (selection logic):
  If LC_NW1 ≥ LC_NW2:            LC_NW = LC_NW2
  Else if I_NW ≤ 1,300:          LC_NW = LC_NW1
  Else if I_NW ≥ 1,950:          LC_NW = LC_NW2
  Else (1,300 < I_NW < 1,950):   LC_NW = LC_NW3   (Equation 13-15)
  LC_NW = total rate of lane changing by nonweaving vehicles, lc/h
Implemented in: weaving/weaving.rs::determine_lane_changing_rates (the four-armed `if lc_nw1 >= lc_nw2 { .. } else if i_nw <= 1300.0 { .. } else if i_nw >= 1950.0 { .. } else { .. }` match, checking the LC_NW1≥LC_NW2 override first exactly as the manual's Equation 13-16 orders it)
```

```
Equation 13-17:  LC_ALL = LC_W + LC_NW
  LC_ALL = total rate of lane changing of all vehicles within the weaving segment, lc/h
Implemented in: weaving/weaving.rs::determine_lane_changing_rates (lc_all, the method's return value)
```

### Step 7: Determine average speeds

```
Equation 13-18 (general form):  S_W = S_MIN + (S_MAX − S_MIN) / (1 + W)
  S_W = average speed of weaving vehicles within the weaving segment, mi/h
  S_MIN = minimum average speed of weaving vehicles expected in a weaving segment, mi/h (constant MIN_WEAVING_SPEED = 15.0 mi/h)
  S_MAX = maximum average speed of weaving vehicles expected, mi/h — taken as FFS (adjusted by SAF), per the manual's guidance below Equation 13-18
  W = weaving intensity factor, unitless (Equation 13-20)
Implemented in: weaving/weaving.rs::estimate_speed (the general form is not coded separately; it is realized directly as Equation 13-19 with S_MIN and S_MAX substituted)
```

```
Equation 13-19:  S_W = 15 + (FFS × SAF − 15) / (1 + W)
  FFS = free-flow speed of the weaving segment, mi/h (code field ffs)
  SAF = speed adjustment factor, decimal (struct default saf = 1.0; Chapter 11 weather/work-zone adjustments)
  W = weaving intensity factor, unitless
  15 mi/h = S_MIN of Equation 13-18 (constant MIN_WEAVING_SPEED)
Implemented in: weaving/weaving.rs::estimate_speed (speed_weaving)
```

```
Equation 13-20:  W = 0.226 · (LC_ALL / L_S)^0.789
  W = weaving intensity factor, unitless
  LC_ALL = total lane-changing rate of all vehicles, lc/h
  L_S = length of the weaving segment, ft
Implemented in: weaving/weaving.rs::estimate_speed (weaving_intensity)
```

```
Equation 13-21:  S_NW = FFS × SAF − 0.0072·LC_MIN − 0.0048·(v/N)
  S_NW = average speed of nonweaving vehicles within the weaving segment, mi/h
  LC_MIN = minimum lane-changing rate from Step 3, lc/h
  v = total demand flow rate = v_W + v_NW, pc/h
  N = number of lanes within the weaving segment, ln
Implemented in: weaving/weaving.rs::estimate_speed (speed_nonweaving; no floor is applied, matching the manual — a previously present non-HCM 15 mi/h floor was removed, see fix history below)
```

```
Equation 13-22:  S = (v_W + v_NW) / [(v_W / S_W) + (v_NW / S_NW)]
  S = space mean speed of all vehicles within the weaving segment, mi/h
Implemented in: weaving/weaving.rs::estimate_speed (speed_avg; the flow-weighted harmonic mean, guarded for v > 0 and both speeds > 0, falling back to S_W otherwise — this is the headline e921e1e fix, see below)
```

### Step 8: Determine density and LOS

```
Equation 13-23:  D = (v/N) / S
  D = average density of all vehicles within the weaving segment, pc/mi/ln
  v = total demand flow rate, pc/h
  N = number of lanes within the weaving segment, ln
  S = space mean speed of all vehicles, mi/h (Equation 13-22)
Implemented in: weaving/weaving.rs::determine_density
```

LOS is then assigned from D (and the `demand_exceeds_capacity` flag) via Exhibit 13-6, already transcribed into `determine_weaving_los` in `weaving/weaving.rs` (and independently into `los_weaving` in `src/hcm/common/los_tables.rs`); see the facility-type thresholds noted above.

## The harmonic space-mean speed fix (Eq 13-22)

Before commit `e921e1e`, Step 7's all-vehicle speed was an arithmetic flow-weighted mean of S_W and S_NW. The HCM's Equation 13-22 is a space mean speed, `S = (v_W + v_NW) / [(v_W/S_W) + (v_NW/S_NW)]`, i.e., the flow-weighted *harmonic* mean, which is always at or below the arithmetic mean and is the quantity consistent with the density definition D = (v/N)/S in Eq 13-23. The current `estimate_speed` implements the harmonic form directly (guarding v > 0 and both speeds > 0, else falling back to S_W). The same commit made three sibling corrections in Step 5: Eq 13-8 is implemented directly as `c_W = c_IW × f_HV` (per lane-flow criterion under prevailing conditions), Eq 13-10 computes `v/c = v × f_HV / c_wa` exactly as published, and a non-HCM 15 mi/h floor previously applied to S_NW was removed (Eq 13-21 has no floor; only Eq 13-19's weaving speed has the 15 mi/h S_MIN anchor). The Eq 13-16 selection order for LC_NW now follows the published four-branch form: LC_NW2 governs whenever LC_NW1 ≥ LC_NW2, else LC_NW1 for I_NW ≤ 1,300, LC_NW2 for I_NW ≥ 1,950, and the Eq 13-15 interpolation `LC_NW1 + (LC_NW2 − LC_NW1)(I_NW − 1300)/650` between.

## Validation

- **Fixtures**: `tests/ExampleCases/hcm/Weaving/case1.json` through `case3.json` — HCM Chapter 27 Example Problems 1-3 (major weave; ramp weave; two-sided weave), added in commit `a2ec7e7`.
- **Test file**: `tests/chapter13_integration.rs`. Each test asserts the full published chain: component flows and VR (Step 2), LC_MIN (Step 3), L_MAX (Step 4), c_IWL and c_W (Step 5), LC_W/LC_NW/LC_ALL (Step 6), S_W/S_NW/S (Step 7), density and LOS letter (Step 8). Published anchor values include EP1: v = 5,586 pc/h, L_MAX = 4,639 ft, c_W = 8,038 veh/h, S = 53.1 mi/h, D = 26.3 pc/mi/ln, LOS C; EP2: c_W weaving criterion 13,333 pc/h, LOS C; EP3 (two-sided): c_W = 4,593 veh/h, `capacity_weaving` is `None`, D = 39.2, LOS E.
- **Tolerances** (documented in the test-file header): flows/capacities ±5 pc/h or veh/h and lane-change rates ±5 lc/h (published values are rounded and the book carries rounded intermediates); speeds ±0.5 mi/h; densities ±0.5 pc/mi/ln; LOS letters exact. EP3's lane-change tolerances are widened to ±10-15 lc/h because the published solution carries an internally inconsistent nonweaving flow (5,015 vs 4,995 pc/h) into Eqs 13-12/13-13 — this discrepancy is documented in the test's doc comment.
- **Unit tests** (inline in `weaving.rs`): Exhibit 13-6 boundary checks for both facility types, f_HV spot checks (level 5% → 0.952, rolling 10% → 0.833), LC_MIN for one-sided (900 lc/h) and two-sided (400 lc/h) hand cases, an Eq 13-4 spot check against Exhibit 13-11 (VR = 0.3: N_WL 2 → 5,584 ft, N_WL 3 → 4,018 ft, ±5 ft), and structural assertions on full one-sided/two-sided runs.
- No `docs/hcm/VERIFICATION.md` exists on this branch; deviations are inline below.

## Deviations

1. **Mountainous PCE (VERIFY-HCM in `calculate_fhv`)**: Exhibit 12-25 provides no PCE for mountainous terrain; the code's `E_T = 5.0` is a flagged non-HCM approximation (HCM directs to the Chapter 25/26 mixed-flow model). Note this module uses 5.0 while `chapter12/basicfreeways.rs` uses 2.5 for the same undefined case — the two non-HCM placeholders disagree with each other.
2. **Eq 13-13 floor**: `lc_nw1` is floored at 0 with the comment "(minimum externally set at 0)" — the flooring is an implementation guard, since the published Eq 13-13 can go negative for short/low-volume segments; confirm the manual's handling.
3. **EP3 published-solution inconsistency**: reproduced only to widened tolerance for LC rates (see Validation); this is a book erratum carried into the fixture expectations, not a code deviation.
4. **`is_weaving == false` is not acted on**: Step 4 sets the flag when L_S ≥ L_MAX (the segment should then be analyzed as separate merge/diverge segments per the manual), but `run_analysis` continues through Steps 5-8 regardless; the caller must check `is_weaving_segment()` themselves.

## Deferred

- Multiple weaving segments and weaving on C-D roads beyond the LOS-threshold distinction (the `MultilaneOrCD` facility type affects only Exhibit 13-6 thresholds, not the speed/capacity models).
- ML-access weaving (managed-lane cross-weave friction, Chapter 13 extension for managed-lane access segments).
- Automatic segmentation fallback when L_S ≥ L_MAX (deviation 4).
