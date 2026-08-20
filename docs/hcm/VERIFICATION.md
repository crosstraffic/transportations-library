# HCM Implementation — Items Needing User Verification

Consolidated `VERIFY-HCM` items and book discrepancies found while implementing HCM 7th Edition
chapters. Each was flagged because the manual is ambiguous, self-contradictory, or the published
example could not be reproduced from the printed procedure. Code locations carry `// VERIFY-HCM`
comments unless noted.

## Chapter 12–14 (feat/hcm-ch12-14-completion)
1. **Mountainous-terrain heavy-vehicle factor does not exist in HCM7.** The book half of this item is settled and needs no further verification: Chapter 12's "Equivalents for General Terrain Segments" says outright that "No PCE is provided for mountainous terrain" and that the Chapter 25/26 mixed-flow model "must be used to estimate speeds and densities". Exhibit 12-25 tabulates level and rolling only. Chapter 13 Step 1 takes f_HV "from Chapter 12" and the Equation 14-1 note says the adjustment factors "are the same as those used in Chapter 12", so neither chapter supplies its own value; Chapter 10's required-input exhibit does not even offer the category, listing terrain as level, rolling, or specific grade. What remains is a product decision, not a reading of the manual.

   Four sites carry a stand-in across three distinct values: `basicfreeways.rs` `adjustment_heavy_vehicle_factor` (2.5), `freeway_facilities.rs` `Terrain::pce` (3.0, also reached by `planning.rs`), `weaving.rs` `calculate_fhv` (5.0), `merge_diverge.rs` `fhv_for` (5.0). The freeway-facilities value is the sharpest case, because `Terrain::to_weave` and `Terrain::to_ramp` pass Mountainous straight through: a single mountainous facility charges E_T = 3.0 on its basic segments and E_T = 5.0 on its weaving and ramp segments.

   **Decide: keep the stand-ins or return an error for mountainous input.** Notes for whichever way it goes. Routing into the real model is not an option at any of the four, since `MixedFlowSegment` requires a grade percent, a grade length, and an SUT/TT split that a terrain class does not supply, and it returns speeds and densities rather than a PCE; mountainous analysis has to enter through `basicfreeways::mixed_flow` or `basicfreeways::composite_grade` directly. Harmonizing to one number is not available either, because no published example problem covers mountainous general terrain, so any single value would be invented rather than derived. Erroring is the HCM-faithful choice, but only `adjustment_heavy_vehicle_factor` already returns `Result`; the other three would need a fallible path threaded out through public infallible surface (`determine_demand_flow` and friends), which is a breaking change for the middleware and calculator that track this crate.
2. `merge_diverge.rs` caps *effective total* accel/decel length at 1,500 ft; HCM caps individual
   lane lengths for two-lane ramps (stricter than book).
3. Eq 14-24 aggregate density: exhibit doesn't fix the lane basis for v; implemented as
   per-mainline-lane flow; no published example exercises it.
4. Ch 28 EP3: book's all-lane speed S=58.8 is not reproducible from its own S_R/S_O via
   Exhibit 14-15 (gives 58.2). Components asserted instead.
5. Ch 27 EP3: book's v_NW internally inconsistent (5,015 vs 4,995); lane-change tolerances widened.

## Chapter 10 / 25 oversaturated engine (feat/hcm-ch10-freeway-facilities)
1. Eq 25-10 (KQ): must use pre-breakdown segment capacity; using the Eq 25-29 queue-discharge-
   reduced capacity spreads queues far beyond published results. Book redefines SC in place.
2. **Eq 25-28 omits lane count N** (KB×L); dimensional consistency (and Eq 25-7) require it. Likely erratum.
3. **Eq 25-34 (queue length) omits N** on the density difference. Likely erratum.
4. Eq 25-6 off-ramp indexing printed as segment-based OFRD(i−1); node-based indexing required.
5. Ramp-segment speeds must additionally be capped by the Ch 12 basic speed-flow curve at the same
   volume to reproduce Exhibit 25-49 (seg 10, 51.8 mi/h) — not stated in the procedure.
6. Densities must be rounded to integer before LOS lookup to match published boundary cells (e.g. 28.2 → C).
7. Queued ramp segments: Exhibit 14-3 defines no density-based F; Exhibit 12-15 thresholds used.
8. EP2 fixture: interchange density not stated in the book's facts; ID=0.8 int/mi reproduces published weave speeds.
9. EP2 period 4: published engine spills residual queue into segments 1–4; ours holds it in 5–6.
   Facility-level aggregates match (±0.2 mi/h); segment speeds in that period differ (documented in tests).

## Chapter 10/25 managed lanes + planning (feat/hcm-ch10-managed-lanes)
1. **Eq 25-87 (Example Problem 5) combined facility density is not reproducible from its own Exhibit 25-86 lane-group densities.** Our combined density is the exact Equation 10-1 lane-mile-weighted average of the GP (31.0) and ML (20.0) group densities the book itself reports; in the peak period that gives 28.3 veh/mi/ln, but Exhibit 25-87 prints 29.1. The facility LOS (D) is unaffected. Asserted at the computed value with a wider tolerance (`tests/chapter10_integration.rs`).
2. **Example Problem 5, ML speed Segment 10 / Period 2 (58.1 mi/h) not reproducible.** The adjacent GP density there is 33.4 veh/mi/ln ⇒ 34.2 pc/mi/ln, below the 35 pc/mi/ln friction threshold (Eq 12-18), so our friction-free speed is 58.9 mi/h. Every other ML cell — including the friction-active cells (Segments 8–9 P2 = 53.5; Segments 8–10 P3 = 52.1) and the friction-free 59.3/58.9/58.6/59.2/59.7 — reproduces exactly. Single 0.8-mi/h cell; the lane-group aggregate (Exhibit 25-86) still matches.
3. **Planning method (Eqs 25-47..25-49) — the worked Example Problem 6 contradicts the printed equations.** Exhibit 25-92 delay rates and Exhibit 25-93 travel rates use ΔRU **only**: the oversaturated ΔRO term (Eq 25-48) is not added to the travel rate as Eq 25-49 states, and ΔRU is evaluated at the **actual** d/c even when d/c > 1.0 (e.g. Section 6 P2: ΔRU(1.016)=11.7, matching the exhibit — not ΔRU capped at 1.0 plus ΔRO). Oversaturation is expressed only through the vertical-queue carryover (Eqs 25-43/25-44, which reproduce the published 0.8-mi queue). Implemented per the worked example; `oversaturated_delay_rate` (Eq 25-48) retained as a public helper but unused in the reported results. `planning.rs`.
4. Eq 25-47 delay-rate polynomial output is treated as **s/mi** (not the "min/mi" printed with the equation): the worked example adds it directly to TR_FFS=3600/FFS s/mi and reproduces Exhibit 25-93. Exhibit 25-16 parameters transcribed verbatim (the FFS=55 row's `D=−0.12` looks anomalous next to the −5.44…−9.33 of the other rows, but is transcribed as printed).
5. Planning facility density is a **length-weighted** (not lane-weighted) average of section densities, per the Exhibit 25-96 note — different from Equation 10-1. Minor book rounding: Section 6 P2 density printed as 41.2 vs 41.9 implied by its own travel rate; facility aggregates asserted within ~0.8 pc/mi/ln.
6. **Oversaturated ML vertical-queue delay (Eqs 25-35/25-36) deferred.** Chapter 25 §4 runs the oversaturated engine separately per lane group and models access-segment spillback only as a non-propagating vertical queue. The GP and ML groups are each analyzed with the existing engines (exact when lane groups do not exchange flow through access segments — the case in Example Problem 5, which is undersaturated). The vertical-queue delay accounting is not implemented.
7. Cross-weave CAF (Eq 13-24/13-25) is provided as `cross_weave_caf` and applied to GP capacity in Step A-9, but no published HCM example exercises it (Example Problem 5 has no cross-weave); unit-tested against the equation directly, integration-tested for the capacity-reduction effect only.

## Chapter 19 (feat/hcm-ch19-signalized)
No VERIFY-HCM items. Interpretation notes: Exhibit 31-12 lag-row sub-variants transcribed but only
PermPerm/LeadLead validated against published g_u; Gq for opposing shared T+R uses g_s+l1
(validated numerically). Known engine deviations (documented in tests): prot-perm left d1 ±1.2 s vs
published QAP interval detail; SB-left back-of-queue needs milestone-2 ADP procedure.

## Chapter 20–22 (feat/hcm-ch20-22-unsignalized)
1. **Ch 32 TWSC Example Problem 3 contradicted the pre-correction 7th-edition exhibits** (movements
   8/11 v_6 factor and 7/10 Stage II factor). **Both halves RESOLVED** by the December 2022
   corrections, and the worked examples were right all along.
   - **The 7/10 half.** The corrections change Equation 20-14 from `f_c,7,6 v_6` to `f_c,7,11 v_11`,
     Equation 20-15 from `f_c,10,3 v_3` to `f_c,10,8 v_8`, and the matching Exhibit 20-16 rows: the
     Stage II conflicting movement is the opposing minor-street through, not the major-street right
     turn. With the correction applied, Example Problem 3 reproduces v_c,II,7 = 337 and
     v_c,II,10 = 257 and Example Problem 4 reproduces v_c,7 = 1,827 and v_c,10 = 1,832, all without
     overrides.
   - **The 8/11 half.** The corrections (page 20-18) swap the Exhibit 20-14 conflicting-movement-6
     entries between movement 8 Stage II and movement 11 Stage I, so f(8,6) becomes the
     "channelized 0 / all others 1" form and f(11,6) the shared-lane 0.5 form. With the swap
     applied, Example Problem 3 reproduces v_c,II,8 = 532 and v_c,I,11 = 482 natively. This entry
     was missed in the first pass over the corrections document (the review was recorded as
     complete on 2026-07-29 with this item still marked open); caught on the 2026-08-01 re-review.
   - No `conflicting_flow_overrides` remain in any TWSC fixture: `case2.json` and `case3.json`
     both carry an empty list.
   - **VERIFY-HCM:** the corrected Exhibit 20-16 gives `f_c,7,11 = f_c,10,8 = 0` when the
     conflicting minor-street movement runs in a STOP- or YIELD-controlled channelized lane and 0.5
     otherwise. `MinorLaneConfig` has no channelized variant, so only the 0.5 case is reachable and
     no published example exercises the other.
2. U-turn critical/follow-up headways on two-lane majors are "NA" in Exhibit 20-17/20-18; four-lane
   values used as fallback if coded. `twsc.rs:804,853`.
3. **Step 5b upstream-signal platoon blockage (Eqs 20-19..20-21, Exhibit 20-19) wired** via the
   `platoon_blockage` input (analyst-supplied p_b,x). The Chapter 30 §3 derivation of p_b,x is now
   also wired (`feat/hcm-ch20-computed-pb`): `src/hcm/chapter20/computed_pb.rs` builds p_b from
   `upstream_signals` descriptors using the Ch 18/30 dispersion primitives and the Eq 30-13
   blocked-period-vs-q_c logic; an explicit `platoon_blockage` takes precedence. No published
   end-to-end p_b regression exists (the 0.170/0.260 of Exhibit 32-12 are Ch 30 EP1 engine output,
   requiring the full Ch 19 coordinated engine + §2 O-D), so it is validated by mechanism tests
   (square-wave hand check, dispersion monotonicity, directional mapping, both-direction union,
   analyst precedence, computed-vs-manual `PlatoonBlockage` equivalence).
   Validated against Ch 32 TWSC Example Problem 4 (`case3.json`, `test_twsc_example_problem_4_upstream_signals`):
   conflicting flows, v_c,u,x, and platooned c_p (750/758/859/852/73/72) reproduce exactly. Two
   findings:
   - **EP4 Stage II conflicting flow drops the major-street right-turn term** (0.5 v_6 for movement 7,
     0.5 v_3 for movement 10), same class as finding 1 and resolved the same way: the December 2022
     corrections to Equations 20-14/20-15 make the published v_c,7 = 1,827 and v_c,10 = 1,832
     reproduce natively (the pre-correction Exhibit 20-16 reading gave 1,874/1,879). The fixture's
     `conflicting_flow_overrides` are gone.
   - **EP4 shared major-street left turn now modeled** (feat/hcm-ch20-shared-major-left). The
     `MajorLeftLaneConfig` input (`major_left_eb`/`major_left_wb`; case3.json sets both to `Shared`)
     drives the Step 7d p\*_0,j substitution (Eqs 20-29..20-34, `prob_queue_free_shared_major`) into
     the Rank 3/4 impedance chain and the Step 11b Rank 1 delay (Eqs 20-62/63, `rank1_delay`) into
     Step 12. With x_2+3 = 0.304, p\*_0 = 0.856, the test now asserts the published c_m,7 = c_m,10 = 47
     veh/h, d_2+3 = d_5+6 = 1.3 s, d_A,EB/WB = 1.9 s, and d_I = 34.1 s (all +-0.5 s / +-1 veh/h). The
     two oversaturated minor-left delays d_7 = d_10 = 529 s and d_A,NB/SB = 241 s use +-12 s / +-5 s
     tolerances because Eq 20-61 slopes ~18.6 s per veh/h near v/c = 1.7 and the book rounds c_m to 47
     while this library carries 46.6-47.1 (the over/under-shoots cancel in d_I). **Remaining:** the
     Step 10c through-lane capacity helper `shared_major_lane_capacity` (Eqs 20-51..20-60, c_SS) is
     still standalone/unwired — it does not affect the minor-street or intersection-delay outputs.

## Chapter 24 (feat/hcm-ch24-offstreet-pedbike)
1. Eq 24-17 modal-pair passing distance ambiguity: implementation reproduces the worked example
   (PTds=0.8334); alternative reading gives 0.8241.
2. **Eqs 24-29/24-30 likely sign typo in print** (`1−e^(pi·k)−Pb` vs implemented `1−e^(−pi·k)−Pb`);
   no worked example exists for three-lane paths to confirm.
3. Exhibit 24-14 undefined for widths <8, 10.5–11, 14.5–15, >20 ft; documented interpolation rule used.
4. Ch 35 EP2 book value M1=5.36 computed with runner speed 6.6 mi/h — apparent typo for the 6.5
   default (exact: 5.38).
5. Published child-bike flow "9/h" truncated from 9.44.

## Chapter 18 (feat/hcm-ch18-urban-segments)
No VERIFY-HCM items. Spec gaps documented: Exhibit 18-13 clamping outside 200–700 veh/h/ln and
Exhibit 18-1 outside 25–55 mi/h BFFS are undefined by the exhibits. Ch 18 text cites "Eq 20-43"
for p*_0,j — an HCM6 number; HCM7 equivalents are Eqs 20-29..20-34.

## Chapter 18/30 computed procedures (feat/hcm-ch18-platoon-dispersion)
Ch 30 §3 platoon dispersion (Eqs 30-9..30-13) and §4 delay due to turns (Eqs 30-31..30-68)
implemented (EPUB `235_Ch30_03.xhtml`, `236_Ch30_04.xhtml`; EP1 intermediates in
`240_Ch30_08.xhtml`). All §4 equations transcribed verbatim from the MathML.
1. **§4 right-turn delay approach speed.** Eqs 30-56/30-58 print `S_f = free-flow speed`, but
   EP1's published per-access-point delay (0.193/0.194 s/veh, Exhibit 30-35) reproduces exactly
   only when the right-turn branch uses the **posted speed limit** (35 mi/h): AP1 EB = 0.1934,
   AP2 EB = 0.1947; using the segment free-flow speed (39.33 mi/h) gives 0.217. The reference
   engine evidently evaluates the maneuver at the posted speed. `p_ov = v_lt/c_l = 0.115`
   (Exhibit 30-35) and `d_ap,l` are speed-independent and reproduce regardless. Implementation
   defaults the turn-delay speed to the posted limit (overridable via
   `access_point_turn_delay_speed_mph`).
2. **Eq 30-60 grouping.** The `(1/r_d + 1/r_a)` factor multiplies the fraction
   `(1.47 S_f − u_m)²/(2·1.47 S_f)`; reading it as a denominator (a plausible OCR flattening)
   inflates d1 ≈ 5× and d_ap,r ≈ 12×. MathML `<mfrac>…</mfrac><mrow>(…)</mrow>` confirms the
   multiplier reading.
3. **§4 turn-bay right-turn delay.** A right-turn bay zeroes `d_ap,r` (the right-turner
   decelerates in the bay, not the through lane), consistent with Exhibit 18-13's "both bays ⇒
   0.0" rule; the printed §4 right-turn equations have no explicit bay term.
4. **§3 EP1 P = 0.493 (deferred).** The published proportion arriving on green for the internal
   WB-through at Intersection 1 (0.493, Exhibit 30-32) is only +0.007 above the uniform
   `g/C = 0.486`. Reproducing it requires the full Chapter 19 coordinated-actuated discharge-flow
   profiles (the through queue-service times print as 0.000 in Exhibit 30-33) plus the Section 2
   O-D distribution and offset alignment — not reproducible from the published intermediates
   alone. The dispersion primitives (Eqs 30-9..30-13), the discharge/arrival profile builders,
   and the computed-P path are implemented and unit-tested against the equations; `step_3` uses
   them when `upstream_discharge_profiles` is supplied and falls back to the uniform /
   platoon-ratio assumption otherwise (P = 0.486 for EP1).

## Chapter 11 (feat/hcm-ch11-freeway-reliability)
1. Exhibit 11-22 vs Exhibit 25-41 disagree on 3-lane incident mean duration (67.9 vs 69.6); 11-22 used.
2. Incident lane closures modeled via total-capacity CAF (CAF×open/N) rather than FREEVAL's
   explicit lane-count reduction; densities/speeds use full lane count.
3. EP7 fixture: weaving ramp-to-ramp demands for several APs not published; 50 veh/h assumed.
4. **Published Exhibit 25-103 October incident frequency (0.83) is internally inconsistent** with
   its own inputs (Oct/Nov demand rows of Exhibit 25-100 identical ⇒ frequencies must be equal; 0.79 computed).
5. Distribution tails (TTI_95 2.00 vs 1.67, TTI_max, reliability rating 84.2% vs 90.8%) differ from
   FREEVAL's Monte Carlo results — centers match; traced to the Ch 25 queue-distribution divergence
   + different MC pairing. Asserted at computed values.
6. Weather CAF/SAF interpolation between the 5-mi/h FFS columns is unspecified; linear used.

## Chapter 23 (feat/hcm-ch23-ramp-terminals)
1. **Eq 23-17/Exhibit 23-24 lane-utilization model does not reproduce the book's own worked values**
   (Ex. 34-6: 0.497 computed vs 0.5056 published; Ex. 3: 0.625 vs 0.5551). Printed equation
   implemented; override input provided (used by fixtures). Example Problem 2 narrows where the
   Parclo A-2Q row goes wrong: with v_L = v_E / v_H and v_R = 0, the printed a2 of −0.363 reproduces
   Exhibit 34-20's leftmost-lane shares exactly in both directions (0.2659 against 0.2660 eastbound,
   0.2264 against 0.2263 westbound), while the rightmost-lane shares need a2 = 0.655 where the
   exhibit prints 0.605 (0.4549 implies 0.6548, 0.5265 implies 0.6558 — the same unprinted value
   from two independent approaches). The printed 0.605 is kept.
2. **Eq 23-37 (DDI clearance) printed as (W+L−D) but Exhibit 34-63 implies (W+L+D)**; printed form
   implemented, fixture supplies published values.
3. d2 per-lane vs per-lane-group basis differs between Ch 34 Examples 1 and 5. Settled in 0.3.1 on
   the lane group capacity, which is what Eq 19-26 defines c_A as; Examples 3 and 5 agree and
   Example 1's worksheet is treated as a book defect.
4. Exhibit 34-70 YIELD-turn delays not reproducible from Eq 22-17 (capacities all reproduce exactly).
5. Ch 34 Ex. 5 published DDI uniform delays inconsistent with Eq 19-19 under any arrival type;
   equation-based results asserted (9/10 LOS letters still match).
6. Ex. 5 applied through-form traffic pressure to ramp lefts; left form implemented (~1% delta).
7. Common green is scoped to a phase pair, per Exhibit 23-28. A movement green twice per cycle
   contributes one candidate overlap per window and the largest governs; the book does not say which
   pair governs when several qualify. Exhibits 34-9 and 34-89 both print CG_RD = 34 (a union of the
   two windows gives 39, and Exhibit 34-10's 4.1-ft queue only follows from 34).
8. Shared-group f_RT convention: flow-weighted f_R via Eq 23-23 (Exhibit 34-7 convention) used.
9. **Exhibit 34-22 gives the Example Problem 2 internal shared through-and-right groups f_LU = 1.000**
   where Chapter 19 Exhibit 19-15's default for a three-lane through group is 0.908, which is what
   Examples 1, 3, and 4 print in the same column (Exhibit 34-34 prints 0.908 for exactly this group)
   and what Chapter 23 Step 3 directs for every non-external approach. The default is implemented.
   Forcing 1.000 reproduces the published saturation flows to 4 veh/h but raises mean absolute error
   against the ten Exhibit 34-29 O-D ETTs from 0.26 to 0.63 s/veh, so Exhibit 34-22 is inconsistent
   with Exhibits 34-27 through 34-29 of its own example.
10. **Exhibit 34-25 prints v = 1,282 veh/h for the Example Problem 2 eastbound internal
    through-and-right group** where the Exhibit 34-163 worksheet composition gives v_I + v_D + v_E =
    1,356. Two cells inside the book corroborate 1,356 against that one: Exhibit 34-27 prints
    X = 0.56 for the movement, which is 1,356/2,401 and not 1,282/2,401 (0.53), and the arrival rate
    row of Exhibit 34-25 itself prints q = 0.38 veh/s, which is 1,368 veh/h. The worksheet
    composition is implemented.
11. **Example Problem 2 evaluates EDTT at two design speeds.** The chapter text computes the two
    loop-ramp O-Ds as 1,200/(1.47 × 25) + 5 = 37.7 s/veh while the remaining diverted O-Ds resolve at
    35 mi/h over the 800-ft interchange spacing. `ExtraDistance::design_speed_mph` carries the
    per-movement override, which is what Eq 23-50 defines v_D as. The Facts section of the example
    states the loop-ramp extra distance as 1,600 ft where the worked EDTT uses 1,200 ft.
12. **The parclo family beyond A-2Q is structurally supported and unvalidated.** Routing and lane
    group composition for all six Exhibit 23-17 parclos and the SPUI come from the same Exhibits
    34-171 through 34-177 worksheets that the validated forms use, and every form is exercised end to
    end by `test_every_form_runs_the_pipeline` and `test_every_form_routes_every_od`, but only the
    diamond (Examples 1, 3, 4), the DDI (Examples 5, 6), the Parclo A-2Q (Example 2), and the SPUI
    (Example 7, since the 0.3.4 protected-plus-permitted left-turn support) are pinned to published
    numbers. Chapter 34 publishes no worked example for the other five parclos, which remain
    structurally supported and unvalidated. Example Problem 7's worksheets carry seven documented
    defects (computed with HCM 2000 factors among them), so eight of its ten O-D letters reproduce
    and the rest are pinned at engine values with the published figures named.

## Chapter 23 Part C (feat/hcm-ch23-alternative-intersections)
Alternative intersections (RCUT / MUT / DLT), EPUB 178–182_Ch23_pt3_*.xhtml, Ch 34 Example
Problems 12–17 (269_Ch34_02b/02c.xhtml). The Part C module (`alternative_intersections.rs`)
implements the genuinely Part-C-specific steps — the O-D → junction traversal (Exhibits 23-48/49/50),
EDTT (Eq 23-58/23-59), ETT assembly (Eq 23-60), approach/intersection aggregation (Eq 23-61/23-62),
LOS (Exhibit 23-13), and the DLT offset (Eq 23-63…23-68) and weighted-average delay (Eq 23-69).
STOP-controlled junction delays are computed from Chapter 20 primitives (Eq 20-18 + Eq 20-61) and
reproduce Exhibit 34-128 exactly; signalized junction delays are the Chapter 19 IQA outputs supplied
as `Provided` junction steps.

1. **DLT LOS basis.** Part C Step 10 for RCUT/MUT reads LOS from Exhibit 23-13, but the DLT worked
   examples (Ch 34 Ex. 16/17 discussion of Exhibit 34-145) read LOS from the **Chapter 19**
   control-delay thresholds (ETT ≈ control delay for a DLT). `DisplacedLeftTurn::los` follows the
   worked examples (Ch 19 thresholds); `los_alternative_intersection_od` (Exhibit 23-13) remains
   available. Flagged `// VERIFY-HCM` in code.
2. **Ex. 15 EDTT is misprinted in the manual.** The inline equation prints
   EDTT = (800+800)/(1.47·50) = 21.8 s — a copy-paste from Ex. 14 — while the facts state 600 ft /
   40 mi/h and the Exhibit 34-138 ETT column uses the correct (600+600)/(1.47·40) = 20.4 s. The
   fixture (case3) and tests use 20.4 s (reproduces the published ETTs).
3. **Ex. 13 EB L ETT.** Fully computed value 55.1 s (22.9 + 16.3 + 15.9) vs. published 55.2 s — a
   0.1 s intermediate-rounding delta; LOS E either way. Asserted computed within ±1 s.
4. **Eq 23-66 / 23-68 print errors.** Eq 23-66 prints the last term as `TT_LTD` (a typo for
   `TT_DLT`); Eq 23-68 prints the guard as `if O_SUPP < C` while the accompanying prose says
   "lower than zero". Both implemented per the prose / derivation (Ex. 16 reproduces O_SUPP = 45.2 s
   vs. the published 45 s, which rounds TT_DLT 6.8 → 7 s).
5. **Ex. 12 inputs not fully tabulated.** The Exhibit 34-123 turning-movement demands are not in the
   extracted text, and the two major-street-left control delays (11.2 / 15.0 s) come from a Chapter 20
   run whose conflicting flows the example does not print. Both are supplied as fixture inputs; the
   asserted EDTT/ETT/LOS are independent of the unlisted demands.
6. **Scope: signalized RCUT/MUT junction delays are inputs, not recomputed.** Wiring the full
   Chapter 19 incremental-queue-accumulation + Chapter 18 flow-profile pipeline per signalized
   sub-junction (Ex. 14/15 main junctions and signal crossovers) is deferred; those junction control
   delays enter as `Provided` steps (Exhibit 34-132/34-137 values). The STOP junctions (Ex. 13, and
   the RCUT/MUT U-turn crossovers) are computed from first principles.

## Chapter 16/17 (feat/hcm-ch16-17-urban-facilities)
1. Exhibit 29-66's snow rows omit the +0.19 night drying term of Eq 29-12 while its rain rows
   include it; implementation follows the exhibit.
2. **Exhibit 29-70 printed shoulder-crash proportions (0.021/0.016) are typos** for Exhibit 17-11's
   0.020/0.160 — its own p₀ column back-computes to the latter.
3. Eq 29-8 σ cap for snow: the 0.65-in rain cap scaled by the 10:1 snow/rain depth ratio (matches
   Exhibit 29-66 magnitudes; HCM text silent).
4. Ch 29 EP4 fixture: published coordinated-actuated average phase duration not printed; 45 s
   effective green chosen to reproduce the published base condition.
5. Ch 29 EP1 facility fixtures: segments 2–4 are not individually published, so facility speed/stop
   rate differ slightly from published (22.1 vs 22.6 mi/h); the fully published Ch 30 EP1 segment
   case reproduces exactly.
6. Ch 29 EP4 reliability: TTI-80 within 0.03 of published; PTI tail lighter (1.73 vs ~2.6-3.0)
   because residual-queue carryover between periods (d3) was deferred. **Update (feat/hcm-
   reliability-enhancements): carryover is now implemented** (see the new section below); the PTI
   gap narrowed only modestly (1.73 → 1.75) and is now attributed to other still-deferred elements,
   not to the missing carryover mechanism itself.

## Reliability enhancements (Ch 17 carryover, Ch 37 ATDM) (feat/hcm-reliability-enhancements)
1. **Residual-queue carryover day-boundary reset is an interpretation, not a literal reading.**
   Chapter 17, Section 3 ("Facility Evaluation") states "the initial queue input value for the next
   analysis period is set equal to the residual queue output for the current analysis period" without
   an explicit exception at the boundary between one day's study period and the next day's. A
   strictly literal reading would carry a queue across the ~21-h gap between (e.g.) 9:45-10:00 a.m.
   Monday and 7:00-7:15 a.m. Tuesday, which is not physically defensible and is inconsistent with (a)
   the Chapter 11 freeway reliability engine, where each scenario/day is evaluated from a fresh
   facility clone with no cross-scenario state, and (b) the Chapter 29, Section 3 multiple-time-period/
   spillback technique, whose queue hand-off is described as scoped to "subperiods" of one multi-period
   analysis. Implemented: carryover resets to Qb = 0 at the first analysis period of each day.
   `src/hcm/chapter17/urban_reliability.rs` (module docs + `run()`).
2. **Ch 19, Section 4's saturated/baseline capacity blend (Eqs 19-38 through 19-43) is not
   implemented.** The full HCM initial-queue extension computes a blended average capacity `cA`
   from a separate "saturated capacity" `cs` (serving the backlog) and the ordinary capacity `c`,
   weighted by the unmet-demand duration within the period, and similarly blends d1. This
   implementation uses the scenario's ordinary lane-group capacity directly as `cA` in
   `common::delay::initial_queue_delay`/`queue_end_of_period` — exact when there is no initial
   queue, an approximation otherwise. `src/hcm/common/delay.rs`,
   `src/hcm/chapter17/urban_reliability.rs`.
3. **Shoulder/median lane "user-specified capacity" default is unstated.** Chapter 37, Section 3
   says the buses-only/HOV-only shoulder lane capacity is "the number of buses [or HOVs] per hour
   ... or the user-specified capacity, whichever is less (the user can override the default
   capacity)" but never states what that default numerically is. Implemented: defaults to a normal
   mixed-flow lane's capacity (so the observed vehicle count is normally the binding term).
   `src/hcm/common/atdm.rs` (`ShoulderLaneUse::BusesOnly`/`HovOnly`).
4. **Adaptive signal control has no HCM-endorsed delay-reduction formula.** Chapter 37, Section 5
   explicitly states "it has not been possible to develop a generalized method adaptive signal
   control method for the HCM" and reports only an illustrative three-corridor simulation study
   (Exhibit 37-9: delay reductions 3%-24%, TTI reductions 3%-13%) with inconsistent magnitudes
   across corridors/directions. `adaptive_signal_sat_flow_adjustment` converts a target delay-
   reduction percentage (default: the range midpoint, 13.5%) into a Chapter 17
   `AtdmStrategy::sat_flow_adjustment` via `1 / (1 - pct/100)`, a documented modeling
   simplification (not an HCM-derived equation) chosen so a fixed demand held at capacity yields
   the same fractional delay reduction as the target. Analysts should prefer a directly calibrated
   value from their own study. `src/hcm/common/atdm.rs`
   (`adaptive_signal_sat_flow_adjustment`).
5. **Ch 37, Sections 6-7 (Dynamic Lane Grouping, Reversible Center Lanes) are not modeled.** Both
   sections list Chapter 18/19 inputs an analyst may need to reconsider (lane assignments, turn bay
   lengths, left/right-turn operational mode, median type) but publish no exhibit, equation, or
   default adjustment factor — nothing to transcribe without fabricating a number. Not implemented;
   flagged here rather than with an in-code `// VERIFY-HCM` marker since there is no code to attach
   it to.
6. **Multi-segment CAF interactions can shift a freeway facility's bottleneck downstream.**
   Verified while testing the Ch 37 shoulder-lane/ramp-metering CAFs against the Chapter 25 EP7
   fixture: applying a capacity-increasing CAF to a single segment (or a single merge segment) can
   *raise* the facility's aggregate TTI/VHD, because relieving an upstream bottleneck sends more
   vehicles into a downstream segment that was already the binding constraint. This is legitimate
   Chapter 10 facility-engine behavior (not a bug), so the Chapter 11 integration tests apply these
   strategies uniformly across all affected segments (all segments for the shoulder-lane strategy,
   all merge segments for the ramp-metering strategy) rather than asserting a naive "capacity up ⇒
   TTI down" monotonicity for a single segment in isolation. `tests/chapter11_integration.rs`.

## Chapter 19 milestone 2 (feat/hcm-ch19-actuated)
1. **Actuated phase-duration convergence vs published EP1 durations (Exhibit 31-79).** The Section 2
   procedure (`actuated.rs`, Eqs 31-1..31-45) is driven from the EP1 controller settings holding the
   Steps 1–5 lane-flow / permitted-green operating point fixed at the published values. It reproduces
   the equivalent maximum allowable headway (3.4 EB/WB, 3.1 minor street) and the barrier balance
   exactly. Following the Eq 31-9 denominator correction (the missing cycle-length factor `C` in the
   queue-service-time second term, fixed on `fix/hcm-equation-sweep`), the minor-street through phases
   now match the published durations (Ph8 NB-T 54.00 vs 54.0; Ph4 SB-T 57.79 vs 57.6; SB-T g_e 9.02 vs
   7.8) and the estimated cycle is 100.0 s, within ~2 s of 101.8 (before the fix these were 51.3 / 53.9
   / g_e 9.5 and cycle ~89). Two residuals remain, both documented in the module and test:
   (a) the major-street phases 2/6 under-extend (~28 vs 34 s) because the HCM computational engine's
   combined-flow max-out model holds them at max green while the transcribed green-extension model
   (Eqs 31-29/31-30) gaps them out; (b) the leading protected left phases 3/7 are charged the full
   left-turn demand for queue service rather than only the demand not served in the following
   permitted period, so they over-serve (Ph3 14.30 vs 10.2; Ph7 18.09 vs 13.8) — a residual the Eq 31-9
   correction slightly enlarges. Closing these requires embedding the full Steps 1–5 recomputation and
   the engine's combined-flow extension calibration inside every actuated iteration (Section 7
   computational-engine detail). **VERIFY-HCM.**
2. **Left-turn ADP first-term partial-stop offset.** The first-term back of queue for permitted /
   protected-permitted left-turn lane groups (Eq 31-141) is computed as the largest per-busy-period
   arrival count less `q·d_a/2` (the fully-stopped departure dashed line of Section 4, Step 3 leads
   the solid departure by d_a/2). This reproduces EP1 EB-left 1.8 (exact), SB-left 4.9 → 5.0 (was 3.2
   under the QAP peak), and keeps NB-left within the published queue-storage tolerance. The engine's
   exact multi-segment N_f accounting (Eqs 31-137..31-140 per dissipation interval) would remove the
   ~0.1–0.3 residual on the more complex polygons. **VERIFY-HCM.**
3. **RTOR complementary-movement identification.** HCM Ch 31 §8 estimates exclusive-right-lane RTOR as
   "the left-turn demand of the complementary cross street left-turn movement" but gives no formal
   movement map. Implemented as the approach 90° counterclockwise (the cross-street left that
   discharges into the subject right turn's receiving lanes and whose protected phase clears the
   conflicting through movement). Shared-lane RTOR is left at 0.0 (HCM offers no estimate).
   **VERIFY-HCM.**
4. **Deferred controller-emulation details (HCM defers these to the Section 7 engine):** permissive-
   period modeling; the coordinated-actuated force-off / yield-point emulation beyond the
   equivalent-maximum-green abstraction (Eqs 31-27, 31-40); Dallas left-turn phasing; dual-entry
   activation edge cases; and pulse-mode detection.

## Deferred scopes (tracked, by design — not errors)
- Ch 19 later: full computational-engine actuated convergence to 0.1 s (combined-flow max-out and
  in-loop Steps 1–5 recomputation), Dallas phasing, ped/bike LOS, multi-period.
- Ch 10/25: special work-zone config tables (Exhibits 25-8..25-14), per-segment work-zone alpha, and
  the oversaturated managed-lane vertical-queue delay (Eqs 25-35/25-36). Managed-lane facilities
  (Steps A-9/A-13/A-14/A-17) and the planning-level method (Ch 25 §6) are now implemented
  (feat/hcm-ch10-managed-lanes).
- Ch 18: Ch 30 §4 access-point delay procedure and §3 platoon-dispersion primitives now implemented
  (see "Chapter 18/30 computed procedures"); still deferred: the §2 O-D/volume-balance/spillback
  adjustment and the coordinated-actuated convergence loop that would drive §3 discharge profiles
  from Ch 19 timing (so EP1 computed P = 0.493 from the raw signal remains deferred).
- Ch 20: Ch 30 §3 upstream-signal platoon inputs p_b,x now computed from `upstream_signals`
  descriptors (or supplied directly by the caller); the end-to-end p_b regression against Ch 30 EP1
  / Exhibit 32-12 stays deferred behind the Ch 19 coordinated engine + §2 O-D. Pedestrian-mode
  method still deferred.
- Ch 23: signalized RCUT/MUT sub-junction delays enter as provided inputs (Ch 34 worksheet
  convention); full Ch 19/18 recomputation per sub-junction deferred. Part C itself is implemented
  (feat/hcm-ch23-alternative-intersections).
- Ch 17: random 15-min demand variation (Eqs 29-30..33) and incident-duration calibration — the
  remaining known contributors to the light EP4 PTI tail after residual-queue carryover landed.
- All chapters: pedestrian/bicycle/transit LOS second pass.

## HCM Edition 7.1 replacement Chapters 13/14 (feat/hcm-7-1-versioned-weaving-merge)

1. **Exhibit 27-5 labels the ramp entry and ramp exit demands the wrong way round.** Example Problem 2 is a simple weave with v_RF = 300 and v_FR = 600 pc/h, so the ramp entry carries v_RF + v_RR = 400 pc/h and the ramp exit v_FR + v_RR = 700 pc/h. The exhibit prints "Ramp entry 600 + 100 = 700" and "Ramp exit 300 + 100 = 400". The check still passes either way (both are far below the 2,000 pc/h ramp capacity), so nothing downstream changes. Compare Exhibit 27-7 in Example Problem 3, which orders the same two rows correctly.
2. **Chapter 28 Example Problem 1 uses two different ramp flows within one equation.** The Equation 14-10 line substitutes `0.143 (625/740)` while the Equation 14-11 line immediately below substitutes `71.4 (624/740)`, from a v_R the same problem computed as 624 pc/h. The printed b = -0.911 is consistent with either to three decimals. Implemented with 624 throughout.
3. **"The merge model will in most cases yield a lower capacity than the diverge model" has a computable crossover.** Setting the two impedance terms equal at a common ramp-lane length L gives `0.00408/L = 0.00014/L^0.536`, whose solution `L = (0.00408/0.00014)^(1/0.464)` is about 1,435 ft and is independent of the ramp volume. Below that length the merge governs, as the manual describes; above it the ordering reverses and the diverge capacity is marginally lower. The manual's "in most cases" is doing real work, and the crossover sits above the acceleration-lane lengths of the manual's own defaults. Pinned by `merge_capacity_is_the_lower_of_the_two_below_the_crossover` in `src/hcm/merge_diverge/v7_1.rs`.
4. **Exhibit 13-14's two-sided coefficients are identical to Exhibit 13-13's simple row** (0.016, 0.021, 0.181, 3.217). Verified against the source PDF rather than the text extraction, since an identical row is exactly what a copy-paste error looks like. It is what the manual prints; the models differ in the flow term they weight, not in their coefficients. Do not "fix" this.
5. **Chapter 27 and 28 page footers disagree about the edition's status.** Chapter 28's footers alternate between "Version 7.1" and "Version 7.1 (DRAFT January 2024)" within the same example problem, while Chapter 13/14/27 footers read "Version 7.1" throughout. The document's own cover page dates it November 2025. Worth re-checking Chapter 28's worked values against any later erratum.
   - **No worked value is affected, as far as the published examples reach.** Every printed intermediate in Chapter 28's Example Problems 3 and 4, the two whose pages carry the DRAFT footer, reproduces from the library within the same tolerances the non-DRAFT problems need (2026-08-19). Pinned by `example_problem_3_merge`, `example_problem_3_diverge` and `example_problem_4_left_hand_on_ramp`. The footer inconsistency remains a source-document defect, not a numeric one.
6. **Chapter 27 Example Problem 4 Trial 1 states the wrong free-flow speed in prose.** Page 27-18 reads "the basic segment speed Sb equals the FFS of 65 mi/h" for a problem whose stated FFS is 60 mi/h on every leg and in the weaving segment (p. 27-16). The arithmetic on the next page uses 60 (`So = Sb - SIW = 60 - 6.75 = 53.25`), so the 65 is a stray from Example Problem 1 and nothing downstream is wrong. Implemented with 60; pinned by `example_problem_4_trial_1_complex_0_2`.
7. **Chapter 27 Example Problem 4 Trial 2 carries three wrong cross-references in one step.** Page 27-20 heads the step "Step 5: Determine Density and LOS—Trial 1" inside Trial 2, cites Equation 13-22 for the density Trial 1 took from Equation 13-21 one page earlier, and reads the LOS letter off "Exhibit 13-6" when the one-sided weaving LOS thresholds are Exhibit 13-7 (Exhibit 13-6 is the two-sided NW_RR table). The printed values are Trial 2's own and are correct under the right citations. Pinned by `example_problem_4_trial_2_complex_0_1`.
8. **Chapter 28 cites two different exhibits for the same merge/diverge LOS table.** Example Problem 3 reads its letters "From Exhibit 14-3" (p. 28-16, twice) and Example Problem 4 reads its letter "From Exhibit 14-2" (p. 28-21). Example Problem 4's citation is the right one. Chapter 14 titles Exhibit 14-2 "LOS Criteria for Freeway Merge and Diverge Segments" (p. 14-7) and cites it for the LOS determination in the method steps; Exhibit 14-3 is "Range of Observed Conditions Used to Develop Merge and Diverge Models" and carries no thresholds at all. Both problems' printed letters follow from Exhibit 14-2, which is what the library implements.

## HCM 7th Edition December 2022 corrections (feat/hcm-7-1-versioned-weaving-merge)

Source: `resource/HCM7-corrections-clarifications-updates-12-2022.pdf`, the TRB Highway Capacity and Quality of Service Committee's corrections through December 2022. It applies to the 7th Edition, not to the Edition 7.1 replacement chapters.

1. **Equation 12-6/12-7 base capacity reads the UNADJUSTED free-flow speed.** The corrections change FFS_adj to FFS in both equations and add: "It is important to note that FFS used in the adjusted capacity computation is the original and unadjusted free-flow speed (FFS)." The library previously computed capacity from `ffs_adj = ffs x saf` in `basicfreeways`, in the Chapter 10 facilities engine, and in the new Edition 7.1 modules. Now fixed: capacity comes from the unadjusted FFS and SAF reaches capacity only through CAF. The breakpoint continues to use FFS_adj, and that asymmetry is the trap. Invisible in every published example problem, all of which set SAF = 1.0; pinned by `speed_adjustment_factor_does_not_move_base_capacity`.
   - **Cost to one reproduction.** Chapter 25 Example Problem 4 (work zone, SAF_wz = 0.982) shifts, by a little. Period 5, the queue-recovery period, moved from (14.20 mi/h, 90.4 veh/mi/ln) to (14.82, 88.3) against a published (13.7, 93.4), and other periods moved mixed rather than systematically worse. This was originally recorded as the reason Example Problem 4's period-5 tolerances are wide. That attribution is superseded: scoping the Equation 25-12 front-clearing test to a restored bottleneck (2026-08-11) moved period 5 to (13.03, 98.3), an order of magnitude more than the 0.5% of one segment's capacity this correction is worth. The capacity correction is not what those tolerances are carrying; the residual oversaturated-regime gap is, and it is documented on the Example Problem 4 tests themselves.
   - **Inference, not stated in the corrections.** Exhibits 14-8 and 14-10 tabulate Equation 12-6 capacities by FFS, so they are now read at the unadjusted FFS on the same reasoning. The corrections address Equations 12-6 and 12-7 explicitly and these exhibits only by implication.
2. **APPLIED.** The corrections resolve the whole open Chapter 20 discrepancy recorded above, in two pieces. Equations 20-14/20-15 and the matching Exhibit 20-16 rows replace the major-street right-turn term in the minor-street left turns' Stage II conflicting flow with the opposing minor-street through movement, and the page 20-18 Exhibit 20-14 correction swaps the conflicting-movement-6 entries between movement 8 Stage II and movement 11 Stage I. Example Problems 3 and 4 now reproduce with no `conflicting_flow_overrides` at all. The Exhibit 20-14 entry was missed when this review was first recorded as complete (2026-07-29) and applied on re-review (2026-08-01).
3. **Chapter 26 independently confirms the Chapter 12 capacity correction, including in a worked example.** The same `c = 2,200 + 10 x (FFS - 50)` change appears at pages 26-31, 26-33, 26-36, and 26-50, and 26-39 carries the multilane form. Page 26-48 goes further and reworks a worked example: `c = 0.78 x (2,200 + 10 x [52.3 - 50]) = 1,743` becomes `c = 0.78 x (2,200 + 10 x [60.8 - 50]) = 1,800 pc/h/ln`, replacing the SAF-reduced 52.3 mi/h with the unadjusted 60.8 mi/h, while the Step 5 speed computation on the same page keeps 52.3 in the speed-flow curve and takes 1,800 as the capacity. That is exactly the capacity-from-FFS, speed-from-FFS_adj asymmetry implemented above, demonstrated on the manual's own numbers.

4. **Chapter 32 confirms the Chapter 20 correction.** Page 32-17 restates the `f_c,7,6 v_6` to `f_c,7,11 v_11` change and additionally corrects the paragraph's citation from Equation 20-26 to Equation 20-14. Consistent with what was applied.

## Review status of the December 2022 corrections

The document is fully reviewed against the code. Only two of its items ever required a code change, and both are applied.

| Chapter | Items | Outcome |
|---|---|---|
| 3, 9, 11, 14 | Exhibit titles and captions, a glossary entry for BP_adj, a default-value pointer | Editorial. No code impact. |
| 12 | Equations 12-6/12-7 read the unadjusted FFS | **Fixed.** See item 1 above. |
| 15 | Passing-lane effective length measured from the start rather than the end; N in Equation 15-40 is directional lanes (1 for a two-lane highway); We wording in Equations 15-45/15-47 | **No change needed.** The code already implements every corrected reading: `l_de` is documented and computed as the distance from the passing lane's start, and `BicycleLOS::num_lanes` is already documented as "number of directional through lanes (1 for two-lane highways, 2+ for multilane)". |
| 20 | Equations 20-14/20-15 and Exhibit 20-16 Stage II conflicting movements; Exhibit 20-14 movement 8 Stage II / movement 11 Stage I swap | **Fixed.** See the Chapter 20-22 section above. |
| 26 | Four repeats of the Chapter 12 capacity correction plus a reworked example; Exhibit 26-15 caption; a Step 10 LOS sentence; Section 5 significantly revised | **No change needed.** The library has no separate Chapter 26 capacity path; the formula lives only in the sites fixed under item 1. The Section 5 revision is guidance, not computation. |
| 31 | Two cross-reference corrections pointing the Exhibit 31-65 left-turn saturation adjustment at Equations 31-110/31-112 rather than 19-8 | **No code impact.** Exhibit 31-65 is not implemented. The `Eq. 31-65` reference in `signalized.rs` is to Equation 31-65 (revised lane-group flow rates), a different object. Recorded as a coverage gap in REVIEW_NOTES.md. |
| 32 | Restates the Chapter 20 correction | Covered by the Chapter 20 fix. |
| 38 | Editorial cross-references; BP renamed BP_adj in Equation 38-14 | **No code impact.** Chapter 38 is not implemented. |

