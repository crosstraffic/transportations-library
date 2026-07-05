# Implementation Review Notes

Code-level findings surfaced by the documentation pass (independent agents reading each branch's code against the manual). These are issues in OUR implementation, distinct from the book discrepancies in VERIFICATION.md. Grouped by severity; branch in parentheses. None are fixed yet — each PR body lists its branch's items as a review checklist.

## Likely bugs (recommend fixing before or during branch review)

1. ~~FIXED (feat/hcm-ch15-review-fixes)~~ **PyO3 `SubSegment` docstring says length is in miles; the engine treats it as feet** (÷5280). A Python caller following the docstring gets a 5,280x error. This is the historical Ch 15 unit footgun, now printed in the bindings docs. `src/copython/chapter15.rs` (feat/hcm-restructure).
2. **`todo!()` panics reachable in `adjustment_heavy_vehicle_factor`** for grade/length PCE combos outside the transcribed grid, and its `length == 0.125` fallback clobbers previously computed `e_t`. Library code should never panic on valid-ish input. `src/hcm/chapter12/basicfreeways.rs` (feat/hcm-ch12-14-completion).
3. ~~FIXED (feat/hcm-ch15-review-fixes; also fixed: missing 0.7-0.8 mi bucket, and the downgrade branch negated LENGTH instead of GRADE so every downgrade returned class 1)~~ **`determine_vertical_alignment` missing length bucket**: upgrade branch jumps from `<= 0.5` to `> 0.6` mi, so 0.5–0.6 mi segments fall through to the catch-all thresholds. `src/hcm/chapter15/twolanehighways.rs` (feat/hcm-restructure).
4. **Divide-by-zero at g/C = 1.0** in `progression_factor`/`uniform_delay`; guard posture inconsistent with `initial_queue_delay`. `src/hcm/common/delay.rs` (feat/hcm-shared-infra).
5. **Major-merge LOS inconsistency**: `RampSegment::determine_los` returns `LevelOfService::E` while setting `self.los = None` (HCM defines no LOS there). Callers using the return value get a fabricated letter. `src/hcm/chapter14/merge_diverge.rs` (feat/hcm-ch12-14-completion).
6. **8-lane P_FM can go negative** for v_R above ~1,742 pc/h — no clamp. `merge_diverge.rs` (feat/hcm-ch12-14-completion).
7. **Oversaturated managed lane silently reports demand-based results**: `ml_dc_ratio` is computed but never routes the ML lane group through the oversaturated engine. Consistent with the documented Eq 25-35/36 deferral but should hard-error or warn instead of silently passing demand. `src/hcm/chapter10/managed_lanes.rs` (feat/hcm-ch10-managed-lanes).
8. ~~FIXED (feat/hcm-ch15-review-fixes)~~ **`tests/common/mod.rs::load_test_data_files()` reads `src/ExampleCases/...` which does not exist** — `case_study1.json` is silently excluded from all Rust tests (feat/hcm-restructure).

## Inconsistencies / dead code (fix opportunistically)

9. Mountainous-terrain PCE placeholders are mutually inconsistent: 2.5 in chapter12 vs 5.0 in chapters 13/14 (all VERIFY-HCM flagged; pending the user's keep-vs-error decision in VERIFICATION.md).
10. MSF tables silently default to 2000.0 outside transcribed rows (chapter12).
11. ~~Four fully implemented, never-called TWSC functions: `shared_major_lane_capacity`, `prob_queue_free_shared_major`, `rank1_delay`, `potential_capacity_upstream_signal`~~ — mostly addressed. `potential_capacity_upstream_signal` wired via `platoon_blockage` (feat/hcm-ch20-platoon-blockage, PR #37). `prob_queue_free_shared_major` (Step 7d p\*_0,j substitution) and `rank1_delay` (Step 11b Rank 1 delay) wired via the new `MajorLeftLaneConfig` input (feat/hcm-ch20-shared-major-left); Example Problem 4 now reproduces c_m,7 = c_m,10 = 47, d_2+3 = d_5+6 = 1.3 s, and d_I = 34.1 s exactly. **Remaining:** `shared_major_lane_capacity` (Step 10c, Equations 20-51 through 20-60, the reduced through-lane capacity c_SS) is still standalone/unwired — it does not affect the minor-street or intersection-delay outputs (in EP4 it evaluates to the s_2+3 bound), so wiring it only matters for a caller that wants the reported major-street through-lane capacity under a shared/short pocket; `f_LL` and the through/right saturation flow rates remain fixed at HCM defaults rather than field-measurable inputs (feat/hcm-ch20-22-unsignalized).
12. `queue_end_of_period` has an unreachable `else` arm in its t_A selection (feat/hcm-reliability-enhancements).
13. Stale milestone-1 comment on `step_10_queue_storage` says ADP "is a milestone-2 item" though the code now consumes the ADP result (feat/hcm-ch19-actuated).
14. `DEFAULT_BFFS_FREEWAY` (75.4) declared, never used (chapter12). Dead duplicate branch in Ch 15 `determine_demand_flow` (pt==2, 5<=phv<10 assigns 1500 in both arms). Live author-doubt comment in `calc_speed` (`// Should be ST instead of S?`).
15. ~~FIXED (doc updated to match code, feat/hcm-ch15-review-fixes)~~ `TwoLaneHighways.apd` doc comment says default 0; code uses `unwrap_or(5.0)` (feat/hcm-restructure).
16. `planning.rs` module doc cites Exhibit 25-17 for LOS but the code reuses `los_freeway_facility` (Exhibit 10-6); EP6 letters pass, so the tables likely agree — one-time check against the book (feat/hcm-ch10-managed-lanes).
17. Two overlapping validation systems: `support/constraints.rs` vs the SF-001..005 semantic firewall in `common/mod.rs` — candidates for unification (feat/hcm-restructure).
18. `TurnType::UTurn` never receives a NEMA number despite the doc comment referencing the Ch 20 1U/4U convention (feat/hcm-shared-infra).
19. Roundabout Eq 22-7 bypass capacity reuses the Eq 22-3 coefficients rather than an independent regression — confirm that is what the book prints; nonyielding bypass hardcodes zero-delay/LOS A per Ch 33 EP1 rather than a numbered equation (feat/hcm-ch20-22-unsignalized).

## Interpretations needing explicit sign-off (also in VERIFICATION.md)

20. Planning method drops Eq 25-48 and evaluates Eq 25-47 outside its printed domain — the only reading that reproduces Example Problem 6 (feat/hcm-ch10-managed-lanes).
21. Ch 17 day-boundary reset of residual-queue carryover — physically motivated, not HCM text (feat/hcm-reliability-enhancements).
22. Ch 17 oversaturation flag counts queue-draining periods (x < 1, Qb > 0) as oversaturated (feat/hcm-reliability-enhancements).
23. Adaptive-signal saturation-flow multiplier `1/(1 - pct/100)` is an invented mapping from Exhibit 37-9's published delay-reduction ranges (VERIFY-HCM flagged) (feat/hcm-reliability-enhancements).
24. Eq 31-120 available-capacity arm mixes displayed max green with effective g_p — check the intended basis (feat/hcm-ch19-actuated).
25. Ch 11 `hers_crash_rate` (Eq 25-79) implemented but not wired into `generate_scenarios`; callers pass `crash_rate_per_100mvmt` manually (feat/hcm-ch11-freeway-reliability).
26. EP4 PTI quoted inconsistently between commit/tests (1.7311→1.7462) and VERIFICATION.md ("1.73→1.75") — reconcile before citing anywhere (feat/hcm-reliability-enhancements).

## Coverage gaps (test debt, not bugs)

27. Exhibit 31-12 lag-row phasing variants (LagLead/LagLag/PermLead/PermLag) transcribed but exercised by no fixture (feat/hcm-ch19-signalized).
28. Cross-weave CAF (Eqs 13-24/25) has no published example coverage; unit-tested against the equation only (feat/hcm-ch10-managed-lanes).
29. No Python integration test for Chapter 12; managed-lane segment model has no published-example fixture (feat/hcm-ch12-14-completion).
30. Segmentation boundary conventions at exactly 3,000 ft and 1,500 ft ramp spacing match the tests but deserve a one-time check against Exhibit 10-11 (feat/hcm-ch10-freeway-facilities).
