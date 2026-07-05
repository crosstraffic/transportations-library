# Chapter 13: Freeway Weaving Segments — Procedure Walkthrough

HCM 7th Edition Chapter 13 analyzes freeway weaving segments (and weaving segments on multilane highways and collector-distributor roads) through the core methodology of Steps 2-8: demand adjustment, configuration characteristics, maximum weaving length, capacity, lane-changing rates, speeds, and density/LOS. The implementation is `src/hcm/chapter13/weaving.rs` on the `feat/hcm-ch12-14-completion` branch, a single `WeavingSegment` struct in the chapter15 house style (plain input fields, `Option<T>` computed fields populated by step methods in HCM order, `get_`/`set_` accessors, and a `run_analysis()` that chains Steps 2-8). This branch's commit `e921e1e` rewrote the module with HCM-verified equations; the headline correction is the Equation 13-22 space-mean (harmonic) speed, which replaced an arithmetic weighted mean. A PyO3 wrapper (`copython::chapter13::WeavingSegment`) is registered on this branch.

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

Two-sided weaving segments are handled through the `WeavingType` enum: `nwl()` forces N_WL = 0, Step 2 assigns only the ramp-to-ramp flow as weaving (`v_W = v_RR`, `v_NW = v_FF + v_FR + v_RF`), Step 3 uses Eq 13-3, and Step 5's weaving-flow criterion is skipped (`capacity_weaving = None`). The Exhibit 13-6 LOS table distinguishes freeway thresholds (A≤10 / B≤20 / C≤28 / D≤35 / E≤43 pc/mi/ln) from multilane/C-D thresholds (12/24/32/36/40) via the `FacilityType` enum; F is assigned above the E bound or whenever demand exceeds capacity.

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
