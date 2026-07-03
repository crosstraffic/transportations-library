# HCM Implementation — Items Needing User Verification

Consolidated `VERIFY-HCM` items and book discrepancies found while implementing HCM 7th Edition
chapters. Each was flagged because the manual is ambiguous, self-contradictory, or the published
example could not be reproduced from the printed procedure. Code locations carry `// VERIFY-HCM`
comments unless noted.

## Chapter 12–14 (feat/hcm-ch12-14-completion)
1. **Mountainous-terrain heavy-vehicle factor does not exist in HCM7.** `basicfreeways.rs` retains
   E_T=2.5 (freeway) and weaving/merge use E_T=5.0 as a stand-in; Exhibit 12-25 defines only
   level/rolling, HCM directs mountainous terrain to the Ch 25/26 mixed-flow model. Decide: keep
   stand-in or return an error for mountainous input.
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
1. **Ch 32 TWSC Example Problem 3 contradicts the 7th-edition exhibits** (movements 8/11 v_6 factor
   and 7/10 Stage II factor match HCM6). Code follows the 7th-ed exhibits; the published example is
   reproduced only via explicit `conflicting_flow_overrides` in the fixture. `twsc.rs:655`.
2. U-turn critical/follow-up headways on two-lane majors are "NA" in Exhibit 20-17/20-18; four-lane
   values used as fallback if coded. `twsc.rs:804,853`.

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
   implemented; override input provided (used by fixtures).
2. **Eq 23-37 (DDI clearance) printed as (W+L−D) but Exhibit 34-63 implies (W+L+D)**; printed form
   implemented, fixture supplies published values.
3. d2 per-lane vs per-lane-group basis differs between Ch 34 Examples 1 and 5; per-lane implemented.
4. Exhibit 34-70 YIELD-turn delays not reproducible from Eq 22-17 (capacities all reproduce exactly).
5. Ch 34 Ex. 5 published DDI uniform delays inconsistent with Eq 19-19 under any arrival type;
   equation-based results asserted (9/10 LOS letters still match).
6. Ex. 5 applied through-form traffic pressure to ramp lefts; left form implemented (~1% delta).
7. Exhibit 34-9 CGRD=34 counts only the phase-3 overlap; interval-intersection gives 39 (no outcome effect).
8. Shared-group f_RT convention: flow-weighted f_R via Eq 23-23 (Exhibit 34-7 convention) used.

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
   because residual-queue carryover between periods (d3) is deferred — the main known gap.

## Deferred scopes (tracked, by design — not errors)
- Ch 19 milestone 2: actuated phase-duration loop, left-turn ADP percentile queues, RTOR, Dallas
  phasing, ped/bike LOS, multi-period.
- Ch 10/25: special work-zone config tables (Exhibits 25-8..25-14), per-segment work-zone alpha, and
  the oversaturated managed-lane vertical-queue delay (Eqs 25-35/25-36). Managed-lane facilities
  (Steps A-9/A-13/A-14/A-17) and the planning-level method (Ch 25 §6) are now implemented
  (feat/hcm-ch10-managed-lanes).
- Ch 18: Ch 30 platoon dispersion + O-D/spillback + access-point delay procedure (input hooks provided).
- Ch 20: Ch 30 upstream-signal platoon inputs (p_b,x supplied by caller); pedestrian-mode method.
- Ch 23 milestone 2: RCUT/MUT/DLT alternative intersections.
- All chapters: pedestrian/bicycle/transit LOS second pass.
