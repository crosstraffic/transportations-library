# Chapter 15: Two-Lane Highways — Procedure Walkthrough

HCM 7th Edition Chapter 15 covers the motorized-vehicle methodology (Section 3, "Motorized Vehicle Methodology") for Passing Constrained (PC), Passing Zone (PZ), and Passing Lane (PL) segments, plus the bicycle LOS methodology (Section 4). Both are implemented in a single file, `src/hcm/twolanehighways/twolanehighways.rs` (2,510 lines), re-exported through `src/hcm/twolanehighways/mod.rs` (this is the current topic-folder path; the module was renamed from `src/hcm/chapter15/`). The motorized methodology walks each `Segment` through an 11-step sequence (free-flow speed → average speed → percent followers → follower density → LOS), driven by the `TwoLaneHighways` struct holding a `Vec<Segment>` plus facility-wide geometry (`lane_width`, `shoulder_width`, `apd`, `pmhvfl`, `l_de`); the bicycle methodology is a self-contained `BicycleLOS` struct with its own 5-step chain. This document covers the motorized steps in manual order, then the bicycle method, then documents the two internal unit conventions that most easily trip up a caller.

This pass re-verified every equation below directly against the HCM 7th Edition EPUB (`resources/epub/OEBPS/101_Ch15.xhtml` through `108_Ch15_appa.xhtml` in the user's main checkout, read-only) and against the *current* `twolanehighways.rs`, not against the previous revision of this document. Two things called out as open questions in the previous revision are resolved below: the Step 3 vertical-class bucket gap (Exhibit 15-11) has been fixed in code since this document was last written and is transcribed correctly row-for-row against the manual; and Step 9's previously uncited equations are confirmed to be Eq 15-36 through 15-38, not Eq 15-30 through 15-33 as previously guessed. Several small numeric coefficient transcription errors were found in the process and are marked `**DISCREPANCY:**` inline.

## Step-by-step walkthrough (motorized methodology)

| Manual step | HCM Eq./Exhibit | Rust method | File | Inputs | Output (units) |
|---|---|---|---|---|---|
| Step 1: segment length applicability | Exhibit 15-10 | `identify_vertical_class` | `twolanehighways.rs` | `seg_num` (uses stored `vertical_class`, `passing_type`) | `(min_length, max_length)` in mi |
| Step 2: demand flow rates & capacity | Eq 15-1; capacity Exhibit 15-5 | `determine_demand_flow` | `twolanehighways.rs` | `seg_num` (uses `volume`, `volume_op`, `phf`, `phv`, `passing_type`, `vertical_class`) | `(demand_flow_i, demand_flow_o veh/h, capacity veh/h)`; also mutates `segments[seg_num]` |
| Step 3: vertical alignment classification | Exhibit 15-11 | `determine_vertical_alignment` | `twolanehighways.rs` | `seg_num` (`length` mi, `grade` %) | vertical class `i32` 1-5; re-invokes `identify_vertical_class` if the class changed |
| Step 4: free-flow speed | Eq 15-2 to 15-6, coefficients Exhibit 15-12 | `determine_free_flow_speed` | `twolanehighways.rs` | `seg_num`, facility `lane_width`/`shoulder_width`/`apd` (ft, ft, pts/mi) | FFS, mi/h |
| Step 5: average speed | Eq 15-7 to 15-16, coefficients Exhibits 15-13/15-14/15-19/15-20/15-22 | `estimate_average_speed` (delegates per-subsegment/tangent work to private `calc_speed`) | `twolanehighways.rs` | `seg_num` | `(average_speed mi/h, horizontal_class 0-5)` |
| Step 6: percent followers | Eq 15-17 to 15-23, coefficients Exhibits 15-24 to 15-29 | `estimate_percent_followers` (delegates to private `calc_percent_followers`) | `twolanehighways.rs` | `seg_num` | percent followers, 0-100 |
| Steps 7-8: passing-lane flow split, lane-specific speed/PF, midpoint follower density | Eq 15-24 to 15-34 (flow/HV split, speed differential, midpoint speeds, midpoint FD) | `determine_follower_density_pl` (uses helper `estimate_average_speed_sf` / `estimate_percent_followers_sf` for the faster-lane/slower-lane sub-calculations) | `twolanehighways.rs` | `seg_num`, facility `pmhvfl` | `(fd, fd_mid)` followers/mi/ln |
| Step 8 (PC/PZ path): follower density | Eq 15-35 | `determine_follower_density_pc_pz` | `twolanehighways.rs` | `seg_num` (`avg_speed`, `pf`, `flow_rate`) | followers/mi/ln |
| Step 9: adjustment for upstream passing lane | Eq 15-36 to 15-38 (confirmed against the EPUB this pass — see "Step 9 detail" below) | `determine_adjustment_to_follower_density` | `twolanehighways.rs` | `seg_num` | follower-density adjustment, followers/mi/ln |
| Step 10: segment LOS | Exhibit 15-6 | `determine_segment_los` | `twolanehighways.rs` | `seg_num`, `s_pl` (posted speed limit, mi/h — **not** average speed, see Unit footguns), `cap` (veh/h) | LOS char `'A'..'F'` |
| Step 11: facility follower density | Eq 15-39 | `determine_facility_follower_density` | `twolanehighways.rs` | (none; walks `self.segments` in order) | FD_F, followers/mi/ln |
| Step 11: facility LOS | Exhibit 15-6 | `determine_facility_los` | `twolanehighways.rs` | `fd` (followers/mi/ln, from `determine_facility_follower_density`), `s_pl` (posted speed limit, mi/h — **not** average speed, see Unit footguns) | LOS char `'A'..'F'` |

The recommended per-segment call order is documented directly on `TwoLaneHighways` (module-level `# Analysis Workflow` doc comment) and matches `tests/common/mod.rs::run_complete_analysis()`: `identify_vertical_class` → `determine_demand_flow` → `determine_vertical_alignment` → `determine_free_flow_speed` → `estimate_average_speed` → `estimate_percent_followers` → (`determine_follower_density_pl` if `passing_type == 2` else `determine_follower_density_pc_pz`) → `determine_adjustment_to_follower_density`.

### Step 2 detail: demand flow and capacity (Eq 15-1, Exhibit 15-5)

```
Equation 15-1:  v_i = V_i / PHF
  v_i = demand flow rate in direction i, veh/h (i = "d" analysis direction, or "o" opposing direction)
  V_i = demand volume for direction i, veh/h
  PHF = peak hour factor (decimal)
Implemented in: twolanehighways/twolanehighways.rs::determine_demand_flow
```

`determine_demand_flow` applies Eq 15-1 as `demand_flow_i = v_i / phf` for the analysis direction; the opposing-direction flow rate is set per the passing-type rule the manual specifies (PC: `v_o = 1500` veh/h fixed high-opposing-flow assumption; PZ: `v_o = V_o/PHF` if `volume_op` is supplied, else `0.0`; PL: `v_o = 0.0`, since passing on a PL segment doesn't use the opposing lane). Capacity is 1,700 veh/h flat for PC and PZ segments; for PL segments it is read from Exhibit 15-5 (Maximum Flow Rates for Passing Lane Segments), a 5×5 table of heavy-vehicle-percentage bracket × vertical class, transcribed as nested `if`/`else if` branches on `phv` and `vc` in `determine_demand_flow`.

**CORRECTED:** Exhibit 15-5's `≥5% <10%` heavy-vehicle bracket gives 1,500 veh/h for vertical classes 1-4 but **1,400 veh/h for vertical class 5** (verified against `103_Ch15_02.xhtml`). The branch now reads `if vc == 1 || vc == 2 || vc == 3 || vc == 4 { capacity = 1500 } else { capacity = 1400 }`, so vertical class 5 receives its 1,400 veh/h capacity. No fixture exercises a Passing Lane (pt == 2) vertical-class-5 segment in this bracket, so no `caseN` assertion moved. All other heavy-vehicle brackets (`<5`, `10-15`, `15-20`, `20-25`, `≥25`) were checked row-by-row against Exhibit 15-5 and match. Fixed in commit for `fix/hcm-equation-sweep` (Exhibit 15-5 vc-5 capacity).

**Note on PHF default:** `TwoLaneHighways`'s module-level doc comment lists the Exhibit 15-8 base-condition PHF as `0.94`, but `Segment::get_phf()` returns `self.phf.unwrap_or(0.95)` — i.e., the actual default used when `phf` is `None` is 0.95, not the 0.94 documented as the base condition. Same class of doc/code default mismatch as the `apd` item already noted below.

### Step 3 detail: vertical class thresholds (Exhibit 15-11) — gap now fixed

The previous revision of this document flagged a missing `0.5 < seg_length <= 0.6` mi bucket in the upgrade branch of `determine_vertical_alignment`, and a bug in the downgrade branch. **Both are fixed in the current code.** The upgrade branch now has an explicit `else if seg_length > 0.5 && seg_length <= 0.6` bucket (tagged in code with the comment `// HCM Exhibit 15-11 row >0.5-0.6 mi (upgrades)`), and the downgrade branch negates the *grade* (`let seg_grade = -1.0 * seg_grade;`) rather than the length, with a code comment explicitly noting the old bug ("Previously the LENGTH was negated instead, which sent every downgrade into the first length bucket with an always-true grade test, so all downgrades returned class 1"). This pass re-verified `determine_vertical_alignment` against Exhibit 15-11 row by row (all 13 length bins × 10 grade columns, upgrade and downgrade/parenthesized values both) and found every threshold now matches the manual exactly, including the two places where the manual's own bucketing skips a vertical class outright: the `>1.1` mi upgrade row jumps from class 2 directly to class 4 (no class-3 bucket exists at that length, i.e. `else if seg_grade <= 5.0 { ver_align = 4 }` following the `<= 3.0 => 2` branch), and the `>0.7-0.8` mi downgrade row similarly skips class 2. Both of these apparent "gaps" are correctly transcribed and are not bugs — they are exactly how Exhibit 15-11 is printed in the manual. The vertical-class bucket gap noted previously in this document's Deviations list no longer applies; see the Deviations section below for the updated status.

### Step 4 detail: free-flow speed (Eq 15-2 to 15-6, Exhibit 15-12)

```
Equation 15-2:  BFFS = 1.14 * Spl
  BFFS = base free-flow speed, mi/h
  Spl = posted speed limit, mi/h
Implemented in: twolanehighways/twolanehighways.rs::determine_free_flow_speed (also inlined in estimate_average_speed and estimate_average_speed_sf — see Unit footguns)

Equation 15-3:  FFS = BFFS - a*(HV%) - f_LS - f_A
  FFS = free-flow speed in the analysis direction, mi/h
  a = heavy-vehicle adjustment factor from Eq 15-4 (decimal)
  HV% = percent heavy vehicles in the analysis direction (e.g. 5% expressed as 5, not 0.05)
  f_LS = lane/shoulder-width adjustment, mi/h, from Eq 15-5
  f_A = access-point-density adjustment, mi/h, from Eq 15-6
Implemented in: twolanehighways/twolanehighways.rs::determine_free_flow_speed

Equation 15-4:  a = max(0.0333, a0 + a1*BFFS + a2*L + max(0, a3 + a4*BFFS + a5*L) * (v_o/1000))
  a0..a5 = coefficients from Exhibit 15-12, keyed by vertical class (transcribed as literal f64 per-vc branches)
  L = segment length, mi (subject to Step 1 min/max)
  v_o = opposing-direction demand flow rate, veh/h (1,500 for PC segments, 0 for PL segments)
Implemented in: twolanehighways/twolanehighways.rs::determine_free_flow_speed

Equation 15-5:  f_LS = 0.6*(12 - LW) + 0.7*(6 - SW)
  LW = lane width, ft (manual: constrained to [9, 12] ft)
  SW = shoulder width, ft (manual: constrained to [0, 6] ft)
Implemented in: twolanehighways/twolanehighways.rs::determine_free_flow_speed

Equation 15-6:  f_A = min(APD/4, 10)
  APD = access-point density, access points/mi
Implemented in: twolanehighways/twolanehighways.rs::determine_free_flow_speed
```

Exhibit 15-12's five coefficient sets (`a0..a5` per vertical class 1-5) were checked value-for-value against the EPUB and match exactly: class 1 is all zeros; class 2 is `-0.45036, 0.00814, 0.01543, 0.01358, 0, 0`; class 3 is `-0.29591, 0.00743, 0, 0.01246, 0, 0`; class 4 is `-0.40902, 0.00975, 0.00767, -0.18363, 0.00423, 0`; class 5 is `-0.38360, 0.01074, 0.01945, -0.69848, 0.01069, 0.12700`. They are transcribed as literal `f64` constants inline in `determine_free_flow_speed` (`if vc == 1 { ... } else if vc == 2 { ... }` etc.) rather than pulled from a shared exhibit table — this duplicates the same literal-constant style used for the `b`/`c`/`d`/`f` coefficients in `calc_speed` and the `b`/`c`/`d`/`e` coefficients in `calc_percent_followers` (all three coefficient sets are re-declared per function with no shared constants module). A reviewer checking these against Exhibit 15-12 needs to check the literals directly in each function since there is no single canonical constants module.

**DISCREPANCY:** Eq 15-5's `LW` and `SW` inputs are explicitly constrained by the manual to `[9, 12]` ft and `[0, 6]` ft respectively before being plugged into the formula. `determine_free_flow_speed` reads `self.lane_width.unwrap_or(12.0)` and `self.shoulder_width.unwrap_or(6.0)` directly into `f_ls = 0.6*(12.0-lw) + 0.7*(6.0-sw)` with no clamping — a caller supplying, say, `lane_width = 14.0` (wider than the manual's 12 ft ceiling) or a negative/oversized shoulder width would silently get a `f_LS` outside the range the manual's model was fit for, rather than being clamped to the boundary value the manual specifies.

### Step 5 detail: average speed (Eq 15-7 to 15-16, Exhibits 15-13/15-14/15-19/15-20/15-22)

```
Equation 15-7:  S = FFS                              if v_d <= 100 veh/h
                S = FFS - m*(v_d/1000 - 0.1)^p        if v_d > 100 veh/h
  S = average speed in the analysis direction, mi/h
  m = slope coefficient from Eq 15-8
  p = power coefficient from Eq 15-11
Implemented in: twolanehighways/twolanehighways.rs::calc_speed

Equation 15-8:  m = max(b5, b0 + b1*FFS + b2*sqrt(v_o/1000) + max(0,b3)*sqrt(L) + max(0,b4)*sqrt(HV%))
  b0..b5 = coefficients from Exhibit 15-13 (PC/PZ) or Exhibit 15-14 (PL), keyed by vertical class
Implemented in: twolanehighways/twolanehighways.rs::calc_speed

Equation 15-9:  b3 = c0 + c1*sqrt(L) + c2*FFS + c3*FFS*sqrt(L)
  c0..c3 = coefficients from Exhibit 15-15 (PC/PZ) or Exhibit 15-16 (PL)
Implemented in: twolanehighways/twolanehighways.rs::calc_speed

Equation 15-10:  b4 = d0 + d1*sqrt(HV%) + d2*FFS + d3*FFS*sqrt(HV%)
  d0..d3 = coefficients from Exhibit 15-17 (PC/PZ) or Exhibit 15-18 (PL)
Implemented in: twolanehighways/twolanehighways.rs::calc_speed

Equation 15-11:  p = max(f8, f0 + f1*FFS + f2*L + f3*(v_o/1000) + f4*sqrt(v_o/1000) + f5*HV% + f6*sqrt(HV%) + f7*L*HV%)
  f0..f8 = coefficients from Exhibit 15-19 (PC/PZ) or Exhibit 15-20 (PL)
Implemented in: twolanehighways/twolanehighways.rs::calc_speed
```

The `b0..b5`, `c0..c3`, `d0..d3`, `f0..f8` literal constants in `calc_speed` were checked against Exhibits 15-13 through 15-20 across all 5 vertical classes and both segment-type branches (PC/PZ and PL); all matched exactly with two exceptions:

**CORRECTED:** Exhibit 15-14 (Eq 15-8 coefficients, Passing Lane segments), vertical class 2, `b0`: the manual gives **-2.0688** (verified against `104_Ch15_03.xhtml`); the code now has `b0 = -2.0688;`. No `caseN` fixture uses a PL vc-2 segment, so no assertion moved. Fixed in commit for `fix/hcm-equation-sweep`.

Horizontal-curve adjustment (Step 5d, Eq 15-12 to 15-16):

```
Equation 15-12:  S_HCi = min(S, FFS_HCi - m*sqrt(v_d/1000 - 0.1))
  S_HCi = average speed on horizontal-curve subsegment i, mi/h
  S = average speed from Eq 15-7 for the whole segment (the tangent speed) — see note below
Implemented in: twolanehighways/twolanehighways.rs::calc_speed (the `shc` computation)

Equation 15-13:  FFS_HCi = BFFS_HCi - 0.0255*HV%
Implemented in: twolanehighways/twolanehighways.rs::calc_speed (`ffshc`)

Equation 15-14:  BFFS_HCi = min(BFFS_T, 44.32 + 0.3728*BFFS_T - 6.868*HorizClass_i)
  BFFS_T = base free-flow speed on the preceding tangent subsegment, mi/h
  HorizClass_i = horizontal classification for subsegment i, 0-5 (Exhibit 15-22)
Implemented in: twolanehighways/twolanehighways.rs::calc_speed (`bffshc`)

Equation 15-15:  m = max(0.277, -25.8993 - 0.7756*FFS_HCi + 10.6294*sqrt(FFS_HCi) + 2.4766*HorizClass_i - 9.8238*sqrt(HorizClass_i))
Implemented in: twolanehighways/twolanehighways.rs::calc_speed (`mhc`)

Equation 15-16:  S = Sum_i(SubsegSpeed_i * SubsegLength_i) / L
  L = actual segment length, mi
Implemented in: twolanehighways/twolanehighways.rs::estimate_average_speed (`res_s = tot_s / seg_length`)
```

`estimate_average_speed` first computes the whole-segment tangent speed via `calc_speed(..., is_hc=false, rad=0.0, sup_ele=0.0)`. If `Segment.is_hc` is `true`, it then iterates `Segment.subsegments`, converts each `SubSegment.length` from feet to miles (`/ 5280.0`, line ~1149), and for curved subsegments (`design_rad > 0.0`) recomputes speed with `calc_speed(..., is_hc=true, rad, sup_ele)`. The final segment speed is the subsegment-length-weighted average, `tot_s / seg_length`, matching Eq 15-16. **If `is_hc` is left `false` (or unset — `Segment::get_is_hc()` defaults to `false`), subsegments and their curve data are silently ignored even if present**, since the `if is_hc { ... }` branch is the only path that reads `subsegments`. The horizontal-class table itself (radius/superelevation → class 0-5, Exhibit 15-22) is transcribed as a 20-branch `if`/`else if` cascade on `rad`/`sup_ele` inside `calc_speed`.

**Resolved this pass — the `// Should be ST instead of S?` comment appears to be unfounded author doubt, not a real bug.** The comment sits immediately before `s = shc;` in `calc_speed`, at the `min(s, ffshc - mhc*sqrt(vd/1000.0 - 0.1))` line implementing Eq 15-12. Checking the manual's own variable list for Eq 15-12: there is no separately defined "S_T" ("speed on tangent") term anywhere in Chapter 15 — the only "T" subscript that appears nearby is `BFFS_T` in Eq 15-14 (a distinct base-free-flow-speed term already handled by the `bffs` argument). Eq 15-12's cap is literally bare "S", which the manual's prose defines as "the average speed in the analysis direction ... with consideration of horizontal curvature" and clarifies further: "All tangent subsegments use the average speed calculated by Equation 15-7. The tangent average speed also serves as a limiting speed for any calculated horizontal subsegment speeds." In the code, when `calc_speed` is called with `is_hc=true` for a curve subsegment, `seg_length`, `ffs`, `vc`, `pt`, `vd`, `vo`, and `phv` are passed identically to the whole-segment tangent-only call (only `rad`/`sup_ele` differ), and `rad`/`sup_ele` do not enter the `ms`/`ps`/`s` computation at all — they only affect `hor_class` and the subsequent `is_hc` block. So the `s` value at the `// Should be ST instead of S?` line is already numerically identical to the whole-segment tangent speed (`seg_s`) computed in the outer call, which is exactly what Eq 15-12's "S" means. The code's use of `s` (not some hypothetical distinct "ST" term) matches the manual. This finding doesn't change the code; it downgrades the open question in the Deviations list below from "unresolved" to "checked, appears correct."

### Step 6 detail: percent followers (Eq 15-17 to 15-23, Exhibits 15-24 to 15-29)

```
Equation 15-17:  PF = 100 * (1 - e^(m*(v_d/1000)^p))
  PF = percent followers in the analysis direction, %
  m, p = slope/power coefficients from Eq 15-22/15-23
Implemented in: twolanehighways/twolanehighways.rs::calc_percent_followers

Equation 15-18 (PC/PZ):  PF_cap = b0 + b1*L + b2*sqrt(L) + b3*FFS + b4*sqrt(FFS) + b5*HV% + b6*FFS*(v_o/1000) + b7*sqrt(v_o/1000)
  b0..b7 = Exhibit 15-24 coefficients, by vertical class
Implemented in: twolanehighways/twolanehighways.rs::calc_percent_followers (`pf_cap`, `pt == 0 || pt == 1` branch)

Equation 15-19 (PL):  PF_cap = b0 + b1*L + b2*sqrt(L) + b3*FFS + b4*sqrt(FFS) + b5*HV% + b6*sqrt(HV%) + b7*FFS*HV%
  b0..b7 = Exhibit 15-25 coefficients, by vertical class
Implemented in: twolanehighways/twolanehighways.rs::calc_percent_followers (`pf_cap`, `pt == 2` branch)

Equation 15-20 (PC/PZ):  PF_25cap = c0 + c1*L + c2*sqrt(L) + c3*FFS + c4*sqrt(FFS) + c5*HV% + c6*FFS*(v_o/1000) + c7*sqrt(v_o/1000)
  c0..c7 = Exhibit 15-26 coefficients, by vertical class
Implemented in: twolanehighways/twolanehighways.rs::calc_percent_followers (`pf_25_cap`, `pt == 0 || pt == 1` branch)

Equation 15-21 (PL):  PF_25cap = c0 + c1*L + c2*sqrt(L) + c3*FFS + c4*sqrt(FFS) + c5*HV% + c6*sqrt(HV%) + c7*FFS*HV%
  c0..c7 = Exhibit 15-27 coefficients, by vertical class
Implemented in: twolanehighways/twolanehighways.rs::calc_percent_followers (`pf_25_cap`, `pt == 2` branch)

Equation 15-22:  m = d1*z_25cap + d2*z_cap,  where z_x = -ln(1 - PF_x/100) / (cap_x/1000)
  d1, d2 = Exhibit 15-28 coefficients, by segment type (PC/PZ: -0.29764, -0.71917; PL: -0.15808, -0.83732)
  cap = directional capacity, veh/h (Exhibit 15-5 for PL, 1,700 for PC/PZ)
Implemented in: twolanehighways/twolanehighways.rs::calc_percent_followers (`m_pf`, `z_cap`, `z_25_cap`)

Equation 15-23:  p = e0 + e1*z_25cap + e2*z_cap + e3*sqrt(z_25cap) + e4*sqrt(z_cap)
  e0..e4 = Exhibit 15-29 coefficients, by segment type (PC/PZ: 0.81165, 0.37920, -0.49524, -2.11289, 2.41146; PL: -1.63246, 1.64960, -4.45823, -4.89119, 10.33057)
Implemented in: twolanehighways/twolanehighways.rs::calc_percent_followers (`p_pf`)
```

Exhibit 15-28 and 15-29 (the `d1`/`d2` and `e0..e4` constants, which don't vary by vertical class, only by segment type) match the manual exactly. Exhibits 15-24 through 15-27 (the `b0..b7`/`c0..c7` per-vertical-class tables) were checked value-for-value; two transcription errors were found:

**CORRECTED:** Exhibit 15-24 (Eq 15-18 coefficients, PC/PZ), vertical class 1, `b7`: the manual gives **7.13758** (verified against `104_Ch15_03.xhtml`); the code now has `b7 = 7.13758;`. This cell is exercised by the PC vc-1 fixtures (case1/2/3) but the ~2e-5 shift is below their assertion tolerance, so no value moved. Fixed in commit for `fix/hcm-equation-sweep`.

**CORRECTED:** Exhibit 15-26 (Eq 15-20 coefficients, PC/PZ), vertical class 1, `c7`: the manual gives **11.60405** (verified against `104_Ch15_03.xhtml`); the code now has `c7 = 11.60405;`. Same PC vc-1 fixtures; sub-tolerance shift, no assertion moved. Fixed in commit for `fix/hcm-equation-sweep`.

**CORRECTED:** Exhibit 15-27 (Eq 15-21 coefficients, PL), vertical class 2, `c6`: the manual gives **0.77217** (verified against `104_Ch15_03.xhtml`); the code now has `c6 = 0.77217;`. No `caseN` fixture uses a PL vc-2 segment, so no assertion moved. Fixed in commit for `fix/hcm-equation-sweep`.

All other entries across the four exhibits (Exhibit 15-24/15-25/15-26/15-27, 5 vertical classes each) matched the manual exactly.

### Step 7-8 detail: passing-lane flow/HV split and midpoint measures (Eq 15-24 to 15-34)

```
Equation 15-24:  NumHV = v_d * HV%/100
Equation 15-25:  PropFlowRate_FL = 0.92183 - 0.05022*ln(v_d) - 0.00030*NumHV
Equation 15-26:  FlowRate_FL = v_d * PropFlowRate_FL
Equation 15-27:  FlowRate_SL = v_d * (1 - PropFlowRate_FL)
  NumHV = number of heavy vehicles entering the passing lane segment, veh
  v_d = demand flow rate entering the passing lane segment, veh/h
  HV% = percentage of heavy vehicles entering the passing lane segment, %
  PropFlowRate_FL = proportion of demand flow in the faster (passing) lane, decimal
  FlowRate_FL, FlowRate_SL = demand flow rate in the faster/slower lane, veh/h
Implemented in: twolanehighways/twolanehighways.rs::determine_follower_density_pl (`nhv`, `p_v_fl`, `vd_fl`, `vd_sl`)

Equation 15-28:  HV%_FL = HV% * HVPropMultiplier_FL
  HVPropMultiplier_FL = 0.4 (manual: a fixed constant, not a caller-supplied parameter — see DISCREPANCY below)
Equation 15-29:  NumHV_SL = NumHV - (FlowRate_FL * HV%_FL/100)
Equation 15-30:  HV%_SL = NumHV_SL / FlowRate_SL * 100
Implemented in: twolanehighways/twolanehighways.rs::determine_follower_density_pl (`phv_fl`, `nhv_sl`, `phv_sl`)

Equation 15-31:  AvgSpeedDiffAdj = 2.750 + 0.00056*v_d + 3.8521*(HV%/100)
Implemented in: twolanehighways/twolanehighways.rs::determine_follower_density_pl (`sda`)

Equation 15-32:  S_PLmid_FL = S_init_FL + AvgSpeedDiffAdj/2
Equation 15-33:  S_PLmid_SL = S_init_SL - AvgSpeedDiffAdj/2
  S_init_FL, S_init_SL = initial average speed in the faster/slower lane (from Step 5's equations/coefficients applied per-lane), mi/h
Implemented in: twolanehighways/twolanehighways.rs::determine_follower_density_pl (`s_mid_fl`, `s_mid_sl`)

Equation 15-34:  FD_PLmid = (PF_PLmid_FL/100 * FlowRate_FL/S_PLmid_FL + PF_PLmid_SL/100 * FlowRate_SL/S_PLmid_SL) / 2
  FD_PLmid = follower density at the passing-lane segment midpoint, followers/mi/ln
  PF_PLmid_FL, PF_PLmid_SL = percent followers in each lane at the segment midpoint, % (from Step 6's equations applied per-lane)
Implemented in: twolanehighways/twolanehighways.rs::determine_follower_density_pl (`fd_mid = (pf_fl*vd_fl/s_mid_fl + pf_sl*vd_sl/s_mid_sl) / 200.0` — dividing by 200 instead of 100-then-2 is algebraically identical since PF is stored as a 0-100 percent, not a 0-1 decimal)
```

`determine_follower_density_pl` implements Eq 15-24 through 15-34 in full: it computes `NumHV`, splits flow via `PropFlowFL`, computes each lane's HV% via Eq 15-28 to 15-30, calls `estimate_average_speed_sf`/`estimate_percent_followers_sf` per lane (applying Step 5/6's coefficient tables with the PL-specific flow and HV% for each lane, subsegment-length-weighted if the segment has curve subsegments), computes the speed-differential adjustment (`sda`, Eq 15-31), the midpoint lane speeds (Eq 15-32/15-33), and finally `fd_mid` (Eq 15-34). It also calls `determine_follower_density_pc_pz` internally to populate the ordinary endpoint `fd` alongside `fd_mid`, so both are available for Step 10/11 (Step 10 picks `fd_mid` when `passing_type == 2`, `fd` otherwise — see `determine_segment_los`).

**DISCREPANCY:** Eq 15-28 defines `HVPropMultiplier_FL` as a **fixed constant, 0.4**, not a tunable input. The code instead reads it from `self.pmhvfl.unwrap_or(0.0)` — a facility-level `Option<f64>` field the caller must supply, defaulting to **0.0** (not the manual's 0.4) when omitted. A caller who doesn't set `pmhvfl` gets `phv_fl = 0.0` for every PL segment (i.e., the faster lane is modeled as having zero heavy vehicles regardless of the facility's actual heavy-vehicle mix), rather than the manual's fixed 40% allocation. The field's own doc comment ("Proportion of heavy vehicles using the faster/passing lane (for PL segments). Used in Equation 15-28 for passing lane analysis.") documents it as a real facility input rather than flagging that the manual treats this as a constant, so this is a modeling deviation, not just a doc gap.

### Step 8 detail (PC/PZ path): follower density (Eq 15-35)

```
Equation 15-35:  FD = (PF/100) * v_d / S
  FD = follower density, followers/mi/ln
  PF = percent followers, %
  v_d = demand flow rate, veh/h
  S = average speed, mi/h
Implemented in: twolanehighways/twolanehighways.rs::determine_follower_density_pc_pz
```

Code computes `fd = (pf * vd) / (100.0 * s)`, algebraically identical to `(PF/100)*v_d/S` since `pf` is stored as a 0-100 percent.

### Step 9 detail: adjustment for downstream-of-passing-lane segments (Eq 15-36 to 15-38) — citation resolved this pass

The previous revision of this document flagged `determine_adjustment_to_follower_density` as carrying no HCM equation-number comments and guessed the source was Eq 15-30 to 15-33. **That guess was wrong.** Cross-checking the EPUB directly (`104_Ch15_03.xhtml`), Eq 15-30 is actually the last of the *Step 7b heavy-vehicle-split* equations (`HV%_SL`, now cited above as part of the Eq 15-24 to 15-34 block), and Eq 15-31 to 15-33 are the *speed-differential/midpoint-speed* equations (also cited above). The manual's own "Step 9: Determine Potential Adjustment to Follower Density" section is headed explicitly by "Equation 15-36 through Equation 15-38 are used to both determine the passing lane's effective length and the improvement in performance measures in downstream segments within the effective length":

```
Equation 15-36:  %ImprovePF = max(0, 27 - 8.75*ln(max(0.1, DownstreamDistance)) + 0.1*max(0, PF-30) + 3.5*ln(max(0.3, PassLaneLength)) - 0.01*FlowRate)
Equation 15-37:  %ImproveS  = max(0, 3 - 0.8*DownstreamDistance + 0.1*max(0, PF-30) + 0.75*PassLaneLength - 0.005*FlowRate)
Equation 15-38:  FD_adj = (PF/100) * (1 - %ImprovePF/100) * FlowRate / (S * (1 + %ImproveS/100))
  DownstreamDistance = distance downstream from the start of the passing lane segment, mi
  PassLaneLength = length of the passing-lane segment, mi
  PF, FlowRate = per the manual's rule, use the values entering the passing lane segment when solving for effective length, and the analysis segment's own PF/FlowRate when computing FD_adj downstream
  S = average speed for the analysis segment, mi/h
  FD_adj = adjusted follower density for a segment downstream of a passing lane, followers/mi/ln
Implemented in: twolanehighways/twolanehighways.rs::determine_adjustment_to_follower_density
```

`determine_adjustment_to_follower_density` implements this in two branches. When the segment being analyzed (`seg_num`) is itself the Passing Lane segment (`pass_type == 2`), it solves Eq 15-36/15-37 for the effective length `l_de` — the distance downstream at which `%ImprovePF` reaches zero (`l_de_1 = exp(y_1a/8.75)`) or follower density recovers to 95% of the level entering the passing lane (`l_de_2`), taking whichever is shorter, matching the manual's "whichever of these two distances is shorter is taken as the effective length" rule; `x_2`, `x_3a`/`x_3b`, `x_4a`/`x_4b` are the `0.1*max(0,PF-30)`, `3.5*ln(max(0.3,PassLaneLength))`/`0.75*PassLaneLength`, and `0.01*FlowRate`/`0.005*FlowRate` terms of Eq 15-36/15-37 respectively. For downstream non-PL segments within that effective length (`l_d < self.l_de`), the code evaluates Eq 15-36 through 15-38 directly with `DownstreamDistance = l_d` (the accumulated length from the passing lane's start): `x_1a = 8.75*ln(max(0.1,l_d))`, `x_1b = 0.8*l_d`, and the resulting `y_1b`/`y_2b`/`fd_adj` expressions line up term-for-term with Eq 15-36/15-37/15-38 using the *analysis segment's* own `pf`/`vd`, matching the manual's rule for `FD_adj` exactly.

One nuance worth a reviewer's attention, extending the existing TODO in the code: `pl_loc` is found by scanning **all** segments in the facility (`for s_num in 0..seg_len`) and taking the last one with `passing_type == 2`, not the nearest passing lane strictly upstream of `seg_num` — the existing `// TODO: if there are more than three PL section` comment already flags that only one PL location is tracked at all. Additionally, `vd_u` (the flow rate "entering the passing lane segment," per the manual's definition) is read from `self.segments[seg_num - 1]` rather than `self.segments[pl_loc - 1]`; these coincide only when `seg_num == pl_loc` (i.e., in the branch where the current segment is itself the PL segment, which is exactly the situation where this line executes), so it happens to be correct in the one branch that reads it, but is worth flagging as fragile if the method's branching is ever restructured.

## Step 10 and Step 11

Step 10 is `determine_segment_los` against Exhibit 15-6. Step 11 is two methods, `determine_facility_follower_density` for Equation 15-39 and then `determine_facility_los` for the facility letter.

```
Equation 15-39:  FD_F = Sum_i(FD_i * L_i) / Sum_i(L_i)
  FD_F = average follower density for the facility in the analysis direction, followers/mi/ln
  FD_i = follower density (or adjusted follower density) for segment i, followers/mi/ln
  L_i = actual segment length for segment i, mi (Step 1 min/max constraints do not apply to this sum)
Implemented in: twolanehighways/twolanehighways.rs::determine_facility_follower_density
```

Until 0.3.1 no aggregation existed in the library at all, and every caller reweighted the unadjusted per-segment column itself, which discarded the entire Step 9 downstream passing-lane benefit that most of Chapter 26 Example Problem 3 is spent computing. `determine_facility_follower_density` now implements Equation 15-39 in one place, and it is the "or adjusted follower density" clause that makes it more than a weighted mean: the term each segment contributes is `FD_PLmid` on a passing lane segment, the Step 9 adjusted density on any segment inside the effective downstream length of an upstream passing lane, and the plain Step 8 density everywhere else.

That ordering matters to callers. The method walks segments in index order and calls `determine_adjustment_to_follower_density` on every one of them, including the passing lanes themselves, because that method records the effective downstream length `l_de` when it reaches a passing lane and every later segment is measured against it. Segments must already have been carried through Steps 1 to 8.

Example Problem 3's facility moved from 8.041 followers/mi and LOS D to 7.271 and LOS C against the published 7.3 and LOS C in Exhibit 26-27; Example Problem 4 moved from 20.219 to 19.897 against a published 20.0, staying inside the LOS E band that had been masking the same omission. The River Falls case study is unaffected, because the Step 9 chain cannot activate on a facility with no passing lane segment, and a regression test states that dependency rather than leaving it implicit.

## Bicycle LOS methodology (Section 4)

| Manual step | HCM Eq. | Rust method | File | Inputs | Output |
|---|---|---|---|---|---|
| Step 2: outside-lane flow rate | Eq 15-40 | `calculate_flow_rate_outside_lane` | `twolanehighways.rs` | `hourly_volume` (veh/h), `phf`, `num_lanes` | veh/h |
| Step 3: effective width | Eq 15-41 to 15-45 (branch selection via private `calculate_wv`) | `calculate_effective_width` | `twolanehighways.rs` | `shoulder_width`, `lane_width` (ft), `pct_on_highway_parking` (decimal), `hourly_volume` (veh/h, per lane for the 160-veh/h branch and Eq 15-45) | ft |
| Step 4: effective speed factor | Eq 15-46 | `calculate_effective_speed_factor` | `twolanehighways.rs` | `speed_limit` (mi/h) | unitless factor, `1.1199*ln(Spl-20)+0.8103` |
| Step 5: BLOS score | Eq 15-47 | `calculate_blos_score` | `twolanehighways.rs` | outputs of steps 2-4 plus `pavement_condition` (1-5 FHWA scale), `heavy_vehicle_pct` | BLOS score (typically 0.5-6.5) |
| LOS lookup | Exhibit 15-7 | `determine_bicycle_los` | `twolanehighways.rs` | BLOS score | LOS char `'A'..'F'`, thresholds ≤1.5/2.5/3.5/4.5/5.5 |
| Convenience wrapper | — | `analyze` | `twolanehighways.rs` | `&self` | `BicycleLOSResult { flow_rate_outside_lane, effective_width, effective_speed_factor, blos_score, los }` |

This section's equations and Exhibit 15-11 (above) were the two areas flagged as recently corrected in code and were re-verified fresh against the current `twolanehighways.rs` and the EPUB (`105_Ch15_04.xhtml`), not assumed from the prior revision of this document. All four bicycle equations match the manual exactly, with no discrepancies found.

```
Equation 15-40:  v_OL = V / (PHF * N)
  v_OL = directional demand flow rate in the outside lane, veh/h
  V = hourly directional volume, veh/h
  PHF = peak hour factor, decimal
  N = number of directional lanes (= 1 for two-lane highways)
Implemented in: twolanehighways/twolanehighways.rs::calculate_flow_rate_outside_lane

Equation 15-41 (Ws >= 8 ft):  We = Wv + Ws - (%OHP * 10 ft)
Equation 15-42 (4 ft <= Ws < 8 ft):  We = Wv + Ws - 2*[%OHP*(2 ft + Ws)]
Equation 15-43 (Ws < 4 ft):  We = Wv - [%OHP*(2 ft + Ws)]
  We = average effective width of the outside through lane, ft
  Wv = effective width as a function of traffic volume, ft, from Eq 15-44/15-45
  Ws = paved shoulder width, ft
  %OHP = percentage of segment with occupied on-highway parking, decimal
Implemented in: twolanehighways/twolanehighways.rs::calculate_effective_width

Equation 15-44 (V > 160 veh/h per lane):  Wv = W_OL + Ws
Equation 15-45 (V <= 160 veh/h per lane):  Wv = (W_OL + Ws) * (2 - 0.005*V)
  W_OL = outside lane width, ft
  V = hourly directional volume per lane, veh/h
Implemented in: twolanehighways/twolanehighways.rs::calculate_effective_width (private helper calculate_wv)

Equation 15-46:  St = 1.1199*ln(Spl - 20) + 0.8103
  St = effective speed factor, unitless
  Spl = posted speed limit, mi/h
Implemented in: twolanehighways/twolanehighways.rs::calculate_effective_speed_factor

Equation 15-47:  BLOS = 0.507*ln(v_OL) + 0.1999*St*(1 + 10.38*HV)^2 + 7.066*(1/P)^2 - 0.005*(We)^2 + 0.760
  BLOS = bicycle level-of-service score
  v_OL = directional demand flow rate in the outside lane, veh/h
  St = effective speed factor (Eq 15-46)
  HV = proportion of heavy vehicles, decimal; if V < 200 veh/h, HV is limited to a maximum of 0.5
  P = FHWA's 5-point pavement surface condition rating (1-5)
  We = average effective width of the outside through lane, ft (Eq 15-41 to 15-45)
Implemented in: twolanehighways/twolanehighways.rs::calculate_blos_score
```

`calculate_effective_width` implements the three shoulder-width branches exactly as printed in Eq 15-41 to 15-43, with `calculate_wv()` returning `W_OL + Ws` when per-lane volume exceeds 160 veh/h (Eq 15-44) and `(W_OL + Ws)*(2 - 0.005V)` otherwise (Eq 15-45) — all coefficients and branch thresholds confirmed against the EPUB verbatim. This was previously corrected against the manual and verified against the HCM Chapter 26 widening worked example (current We = 14 ft, proposed We = 24 ft; see Validation). `calculate_blos_score`'s formula was checked term-for-term against Eq 15-47 in the EPUB (`0.507*ln(v_OL) + 0.1999*St*(1+10.38*HV)^2 + 7.066*(1/P)^2 - 0.005*(We)^2 + 0.760`) and matches exactly, including the exact coefficients `0.1999`, `10.38`, `7.066`, `0.005`, and `0.760`. It clamps `heavy_vehicle_pct` to a maximum of 0.5 when `hourly_volume < 200.0` per the Eq 15-47 note in the manual (`HV should be limited to a maximum of 0.5` when `V < 200 veh/h`), and guards `ln(v_ol)` against a non-positive argument by substituting `0.0` (defensive addition, not manual text).

Exhibit 15-11's vertical-alignment table (used by the motorized methodology, Step 3 above) was likewise re-verified fresh this pass rather than trusted from the prior revision — see "Step 3 detail" above for the row-by-row confirmation.

## Unit footguns

Three conventions are easy to get backwards and are not enforced by the type system (the fields are all plain `f64`/`Option<f64>`):

- **`Segment.spl` is the *posted* speed limit** (mi/h), and Step 4's base free-flow speed is `BFFS = 1.14 * spl` (`determine_free_flow_speed`, and duplicated inline in `estimate_average_speed` as `bffs = round_to_significant_digits(1.14 * spl, 3)` and again in `estimate_average_speed_sf` as `bffs = 1.14 * spl`) — i.e., **BFFS is derived, not itself a stored/settable field**; passing an already-adjusted FFS-like value as `spl` will silently double-inflate BFFS by 14%.
- **`SubSegment.length` is in FEET**, while **`Segment.length` is in MILES** — both are documented correctly in the field doc comments (`/// Length of subsegment, ft.` vs. `/// Length of segment, mi.`), and the conversion is applied consistently at each subsegment read site (`get_length() / 5280.0` in `estimate_average_speed` and `determine_follower_density_pl`). The Python binding's constructor docstring in `src/copython/twolanehighways.rs` (`SubSegment::new`) previously stated the length was in miles, which contradicted the Rust field and every use site; **this has been fixed** — the docstring now reads "Length of the sub-segment in FEET (default: 0.0). Note: unlike Segment.length (miles), sub-segment lengths are in feet; the engine divides by 5,280 internally." (checked directly in the current `src/copython/twolanehighways.rs`, which itself is the renamed file — it was previously `src/copython/chapter15.rs`).
- **The `s_pl` argument to `determine_segment_los` and `determine_facility_los` is the POSTED speed limit, not the computed average travel speed.** Exhibit 15-6 prints its two columns as "Posted Speed Limit >= 50 mi/h" and "Posted Speed Limit < 50 mi/h", and the chapter text states the split as "On higher-speed two-lane highways (>= 50 mi/h)". Both methods always documented the parameter that way, but until 0.3.2 every caller in the repository passed the computed average speed instead: both analysis scripts, the River Falls gate, the Rust integration and example-problem harnesses, `tests/common/mod.rs`, the README, and the crate's own doc examples. No published value moved when they were corrected, because on all four Chapter 26 fixtures and on River Falls the posted limit and the average speed fall on the same side of 50 mi/h, which is exactly what makes this a latent trap rather than a visible one. HCM Step 11 defines no facility-level posted limit for a facility with mixed limits, so the facility callers length-weight it, which reduces to the common value when every segment shares one.
- **`Segment.is_hc` gates whether horizontal-class/curve data is used at all.** Supplying `subsegments` with real `design_rad`/`sup_ele` data but leaving `is_hc` at its default (`false`, via `get_is_hc()`'s `unwrap_or(false)`) means Step 5 silently falls back to the tangent-only `calc_speed` path and never reads the subsegment curve data — there is no warning or error, the curve data is simply inert.

## Deviations

This section predates `docs/hcm/VERIFICATION.md`, which now exists and carries the consolidated book-discrepancy ledger; the deviations below are code-level and are called out inline:

1. ~~Step 3 (`determine_vertical_alignment`) has a missing length bucket~~ **Fixed in code and re-verified against Exhibit 15-11 this pass — no longer a live deviation.** See "Step 3 detail" above; the current code matches the manual's vertical-class table exactly for every length bin and grade column, including the two rows where the manual itself skips a class.
2. Step 9 (`determine_adjustment_to_follower_density`) still has no HCM equation-number comments in the code itself, unlike every other step, but **this document now confirms the source is Eq 15-36 to 15-38** (not Eq 15-30 to 15-33 as previously guessed) — see "Step 9 detail" above. It also still only tracks one upstream passing-lane location (`// TODO: if there are more than three PL section`), and this pass additionally found that `vd_u` is read from `segments[seg_num-1]` rather than `segments[pl_loc-1]`, which only coincide in the branch where they're currently used.
3. `calc_speed`'s horizontal-curve speed cap carries a live author doubt comment, `// Should be ST instead of S?` — **checked against the manual this pass and the code's use of `s` appears correct** (there is no separately defined "S_T" term in Eq 15-12; the manual's own prose confirms the tangent-computed "S" from Eq 15-7 is meant to cap the curve speed, and that is exactly what `s` already equals at that point in the code). See "Step 5 detail" above. The comment itself was left in place since this task's scope excludes code changes.
4. The `copython::twolanehighways::SubSegment::new` PyO3 docstring previously stated subsegment length is in miles, contradicting the Rust implementation and field doc comment (feet). **This has been fixed** in the current code (file also renamed from `chapter15.rs` to `twolanehighways.rs`).
5. **Fixed** in `fix/hcm-equation-sweep`: four small coefficient transcription errors, each corrected to the manual value: Exhibit 15-14 (`Eq 15-8`, PL, vc 2) `b0` -2.0668 → -2.0688; Exhibit 15-24 (`Eq 15-18`, PC/PZ, vc 1) `b7` 7.13760 → 7.13758; Exhibit 15-26 (`Eq 15-20`, PC/PZ, vc 1) `c7` 11.60410 → 11.60405; Exhibit 15-27 (`Eq 15-21`, PL, vc 2) `c6` 0.77127 → 0.77217. See the `**CORRECTED:**` markers in "Step 4/5/6 detail" above. All shifts are sub-tolerance for the exercised fixtures; no `caseN` assertion moved.
6. **Fixed** in `fix/hcm-equation-sweep`: Exhibit 15-5's `≥5% <10%` heavy-vehicle capacity bracket now returns 1,400 veh/h for vertical class 5 (1,500 for classes 1-4), replacing the no-op `if`/`else` that always returned 1,500. See "Step 2 detail" above.
7. `HVPropMultiplier_FL` (Eq 15-28) is a fixed 0.4 constant in the manual but is exposed as a caller-supplied `pmhvfl` field defaulting to 0.0 in code — a real modeling deviation, not just a doc-comment mismatch. See "Step 7-8 detail" above.
8. `TwoLaneHighways.apd` field doc comment says "default: 0" but `determine_free_flow_speed` reads `self.apd.unwrap_or(5.0)` — a 5.0-points/mi default, not 0, is used whenever `apd` is `None`. (Noted in `architecture.md` as well since it's a doc/code mismatch rather than an HCM-fidelity issue.)
9. `Segment::get_phf()` defaults to `0.95` when `phf` is `None`, but the module's own `# Base Conditions (Exhibit 15-8)` doc comment lists the base-condition PHF as `0.94`. Same class of doc/code default mismatch as item 8.

## Validation

- **Fixtures**: `tests/ExampleCases/hcm/TwoLaneHighways/case1.json` through `case4.json` (one `TwoLaneHighways` JSON per case; `case1.json` inspected directly: single PC segment, 0.75 mi, 50 mph posted, 752 veh/h volume, PHF 0.94, PHV 5%, 12 ft lanes, 6 ft shoulders, `apd: 0.0`). `case_study1.json` also exists in the same directory but is excluded by the test file filter (see `architecture.md`) and not exercised by `cargo test`.
- **Test files / tolerance**: `tests/twolanehighways_test.rs` runs `identity_vertical_class_test`, `determine_demand_flow_test`, `determine_vertical_alignment_test` (and further step-by-step tests later in the 485-line file not fully enumerated here) against `case1-4.json`, asserting exact equality (`assert_eq!`) between hand-transcribed expected arrays and computed values rounded via `.round()` or `math::round_to_significant_digits(_, 3)` — i.e., tolerance is "exact match after rounding to the same precision as the expected value," not an epsilon comparison. `bicycle_los_test` (same file) only asserts range/ordering properties (`blos_score > 0.0`, worse conditions produce a higher score, `los` is one of `A`-`F`), not exact manual example values. `tests/twolanehighways_integration.rs` (336 lines) runs the same `case*.json` fixtures through the full step sequence and asserts HCM-plausible ranges (e.g., `capacity` in `[1100, 1700]`, `ffs` in `[20.0, 80.0]`) rather than exact figures. `tests/semantic_firewall_test.rs` (324 lines) tests only the `common::mod.rs` `validate_*` boundary functions (`SF-001`..`SF-005`), not the step methods themselves.
- **Python integration test**: `tests/test_twolanehighways_integration.py` exercises the compiled `transportations_library` Python extension if importable (skips otherwise), but its assertions are generic (`hasattr`, type checks) rather than HCM-example-value checks.

## Deferred

- No test in this branch reproduces a full HCM 7th Edition worked example end-to-end with published intermediate values cited by page/exhibit number in the test itself — the `case*.json` fixtures' provenance (whether they are transcribed from a specific manual example) is not documented in the test files or fixture directory.
- Step 9's equation citation is now resolved (Eq 15-36 to 15-38, see above); the `// Should be ST instead of S?` comment is now checked and appears to be unfounded doubt rather than a bug (see Deviations item 3). Neither required a code change, so both remain as-is in the source.
- The four coefficient-table items (Deviations 5) and the capacity-bracket no-op (Deviations 6) are **fixed** in `fix/hcm-equation-sweep`. The `pmhvfl`/`HVPropMultiplier_FL` deviation (Deviations 7) remains **reported, not fixed** — it is a pending user decision (whether to change the field default to the manual's fixed 0.4) and is out of scope for the equation-sweep pass.
- The `apd` and `phf` default mismatches (Deviations 8-9) are reported, not fixed.
