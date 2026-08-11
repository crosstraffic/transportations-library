"""Regression gate: WIS 35/WIS 65 (River Falls Bypass), EB — HCM Ch.15 facility.

This pins the project's canonical validation number. The directional operational
analysis of the River Falls facility must yield a facility follower density of
5.223 pc/mi/ln (LOS C). That value was validated against the analysis request and
is the reference the rest of the codebase is checked against, but until now it was
only reproducible by running scripts/analyze_river_falls.py by hand — a drift in
the Ch.15 chain would not have failed any test. This test closes that gap.

OPEN QUESTION, do not read 5.223 as settled. ``PHV`` and ``sup_ele`` below are
passed as fractions (0.08, 0.02) where the engine reads percent: Exhibit 15-9
brackets the heavy-vehicle FFS coefficient at 5/10/15/20/25 and Exhibit 15-22
brackets superelevation at 1 through 8, and every published fixture in
``tests/ExampleCases/hcm/TwoLaneHighways/`` passes 5.0/8.0 and 2.0/3.0/4.0/5.0
accordingly. Correcting only those two, holding every other input here fixed,
gives 5.309, and ``case_study1.json`` (the same corridor encoded as a fixture,
in percent) gives 5.310. So 5.223 is low by roughly 0.09 for a unit reason
rather than an engine reason. It is left in place because it is the number
currently reported in the paper; changing it is Rei's call, not a maintenance
edit. See the header of ``run_case_study.py`` for the full attribution.

Inputs are verbatim from the analysis request; nothing is estimated. Two engine
conventions that are easy to get wrong (both verified against the Rust source):
  * ``spl`` is the POSTED SPEED LIMIT; the engine derives BFFS = 1.14 * spl.
    PSL = 55 -> BFFS = 62.7.
  * Complex-alignment segments must be built with ``is_hc=True``, and SubSegment
    length is in FEET (the engine divides by 5280 internally) while Segment length
    is in MILES. Passing subsegment length in miles, or leaving is_hc False,
    silently drops the horizontal-curve speed penalty.
"""

import pytest

tl = pytest.importorskip(
    "transportations_library",
    reason="Rust library wheel not installed; River Falls gate skipped",
)
from transportations_library import (  # noqa: E402
    Segment,
    SubSegment,
    TwoLaneHighways,
)

LW, SW, APD, PHV, PHF, PSL, V, VO = 12.0, 6.0, 2.0, 0.08, 0.94, 55.0, 512.0, 512.0
FT = 5280.0


def _sub(length_ft, rad=None):
    return (
        SubSegment(length=length_ft, hor_class=1)
        if rad is None
        else SubSegment(length=length_ft, design_rad=rad, sup_ele=0.02)
    )


def _seg(pt, length_mi, grade, ss=None, hc=None):
    return Segment(
        passing_type=pt, length=length_mi, grade=grade, spl=PSL, is_hc=hc,
        volume=V, volume_op=VO, phv=PHV, phf=PHF, subsegments=ss,
    )


def _river_falls_facility():
    ss3 = [_sub(3200), _sub(2850, 840.0), _sub(2200), _sub(1800, 845.0), _sub(2300)]
    ss5 = [_sub(1848), _sub(6600, 2520.0)]
    len3 = sum(s.length for s in ss3) / FT  # feet -> miles for the segment length
    len5 = sum(s.length for s in ss5) / FT
    segments = [
        _seg(0, 0.16, 1.0),                 # S1 passing-constrained
        _seg(1, 0.64, 1.0),                 # S2 passing zone
        _seg(0, len3, 0.0, ss3, hc=True),   # S3 passing-constrained, complex alignment
        _seg(1, 0.625, 0.0),                # S4 passing zone
        _seg(0, len5, 1.0, ss5, hc=True),   # S5 passing-constrained, complex alignment
    ]
    return TwoLaneHighways(
        segments=segments, lane_width=LW, shoulder_width=SW, apd=APD, pmhvfl=PHV,
    )


def _facility_metrics(hwy):
    tot_len = w_spd = 0.0
    for i in range(hwy.num_segments):
        hwy.identify_vertical_class(i)
        _, _, cap = hwy.determine_demand_flow(i)
        hwy.determine_vertical_alignment(i)
        hwy.determine_free_flow_speed(i)
        ats = hwy.estimate_average_speed(i)[0]
        hwy.estimate_percent_followers(i)
        hwy.determine_follower_density_pc_pz(i)
        hwy.determine_adjustment_to_follower_density(i)
        s = hwy.segments[i]
        tot_len += s.length
        w_spd += ats * s.length
    fac_fd, fac_spd = hwy.determine_facility_follower_density(), w_spd / tot_len
    # Exhibit 15-6 splits on POSTED SPEED LIMIT, not on the computed average
    # speed. Every River Falls segment is posted 55, so the facility limit is
    # PSL. Both fall above 50 here, so the LOS assertion below is unaffected.
    return tot_len, fac_fd, fac_spd, hwy.determine_facility_los(fac_fd, PSL)


def test_river_falls_facility_follower_density_gate():
    """The canonical gate: facility follower density 5.223 pc/mi/ln, LOS C."""
    tot_len, fac_fd, fac_spd, fac_los = _facility_metrics(_river_falls_facility())
    assert round(fac_fd, 3) == 5.223, f"facility follower density drifted: {fac_fd}"
    assert fac_los == "C"
    # Secondary pins so a compensating error in length/speed can't hide a drift.
    assert round(tot_len, 3) == 5.364
    assert round(fac_spd, 2) == 58.36


def test_river_falls_unaffected_by_the_equation_15_39_correction():
    """River Falls has no passing lane, so Step 11 must aggregate plain densities.

    ``determine_facility_follower_density`` implements Equation 15-39 over the
    ADJUSTED segment densities, which corrects the Chapter 26 Example Problem 3
    facility from LOS D to the published LOS C. That whole adjustment chain is
    gated on the presence of a passing lane: River Falls is passing_type 0 and 1
    only, so ``determine_adjustment_to_follower_density`` returns 0.0 on every
    segment and the aggregate is unchanged at 5.223. This test states that
    dependency explicitly, so a future change to the Step 9 gating that leaked
    into non-passing-lane facilities would fail here rather than silently
    move the project's canonical number.
    """
    hwy = _river_falls_facility()
    assert {s.passing_type for s in hwy.segments} == {0, 1}, "no passing lane expected"

    _, fac_fd, _, _ = _facility_metrics(hwy)
    assert round(fac_fd, 3) == 5.223

    for i in range(hwy.num_segments):
        assert hwy.determine_adjustment_to_follower_density(i) == 0.0, (
            f"segment {i} picked up a passing-lane adjustment"
        )
