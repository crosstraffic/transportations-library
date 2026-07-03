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
   because residual-queue carryover between periods (d3) is deferred — the main known gap.

## Chapter 19 milestone 2 (feat/hcm-ch19-actuated)
1. **Actuated phase-duration convergence vs published EP1 durations (Exhibit 31-79).** The Section 2
   procedure (`actuated.rs`, Eqs 31-1..31-45) is driven from the EP1 controller settings holding the
   Steps 1–5 lane-flow / permitted-green operating point fixed at the published values. It reproduces
   the equivalent maximum allowable headway (3.4 EB/WB, 3.1 minor street) and the barrier balance
   exactly, and the minor-street through phases within ~4 s (Ph8 NB-T ≈ 51–54 vs 54.0; Ph4 SB-T ≈ 54
   vs 57.6; SB-T g_e ≈ 9.5 vs 7.8). Two residuals remain, both documented in the module and test:
   (a) the major-street phases 2/6 under-extend (~23 vs 34 s) because the HCM computational engine's
   combined-flow max-out model holds them at max green while the transcribed green-extension model
   (Eqs 31-29/31-30) gaps them out; (b) the leading protected left phases 3/7 are charged the full
   left-turn demand for queue service rather than only the demand not served in the following
   permitted period. The estimated cycle is ~13 s short of 101.8 s. Closing these requires embedding
   the full Steps 1–5 recomputation and the engine's combined-flow extension calibration inside every
   actuated iteration (Section 7 computational-engine detail). **VERIFY-HCM.**
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
- Ch 10/25: managed-lane facilities, planning method, special work-zone config tables, per-segment
  work-zone alpha.
- Ch 18: Ch 30 §4 access-point delay procedure and §3 platoon-dispersion primitives now implemented
  (see "Chapter 18/30 computed procedures"); still deferred: the §2 O-D/volume-balance/spillback
  adjustment and the coordinated-actuated convergence loop that would drive §3 discharge profiles
  from Ch 19 timing (so EP1 computed P = 0.493 from the raw signal remains deferred).
- Ch 20: Ch 30 upstream-signal platoon inputs (p_b,x supplied by caller); pedestrian-mode method.
- Ch 23 milestone 2: RCUT/MUT/DLT alternative intersections.
- All chapters: pedestrian/bicycle/transit LOS second pass.
