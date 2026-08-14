"""Python-binding integration test for the HCM Chapter 25/26 mixed-flow model.

Mirrors the two Rust integration tests through the PyO3 JSON entry points:
tests/ExampleCases/hcm/Chapter26/ep5_mixed_flow.json (Chapter 26 Example Problem 5, mixed-flow
half) and tests/ExampleCases/hcm/Chapter25/ep11_composite_grade.json (Chapter 25 Example
Problem 11). Tolerances match the Rust side.

Three published values in these examples contradict other published values in the same
examples. The self-consistent one is asserted in each case, and the reasoning is recorded at
the Rust tests that pin them (tests/chapter25_composite_grade_integration.rs and the header of
tests/chapter12_integration.rs). In short: Example Problem 5's density is 31.7 veh/mi/ln from
Step 8, not the 32.6 its comparison paragraph quotes; Example Problem 11's Segment 2 rate is
61.3 s/mi, not the 62.6 printed one line above the division that uses 61.3; and Exhibit
25-109's end-of-Segment-1 row should read 56.4 / 56.1 / 46.1.
"""

import json
from pathlib import Path

import pytest

tl = pytest.importorskip("transportations_library")

CASES = Path(__file__).parent / "ExampleCases" / "hcm"


def load(chapter, name):
    return json.loads((CASES / chapter / name).read_text())


@pytest.fixture(scope="module")
def ep5():
    return json.loads(tl.analyze_mixed_flow(json.dumps(load("Chapter26", "ep5_mixed_flow.json"))))


@pytest.fixture(scope="module")
def ep11():
    return json.loads(
        tl.analyze_composite_grade(json.dumps(load("Chapter25", "ep11_composite_grade.json")))
    )


# ── Chapter 26 Example Problem 5, mixed-flow half ────────────────────────────


def test_ep5_capacity(ep5):
    assert ep5["caf_t_mix"] == pytest.approx(0.135, abs=0.001)
    assert ep5["rho_g_mix"] == pytest.approx(0.1215, abs=1e-9)
    assert ep5["caf_g_mix"] == pytest.approx(0.131, abs=0.001)
    assert ep5["caf_mix"] == pytest.approx(0.734, abs=0.001)
    assert ep5["capacity_ao"] == pytest.approx(2350.0)
    # The example carries CAF_mix rounded to three decimals into Equation 26-5.
    assert ep5["capacity_mix"] == pytest.approx(1725.0, abs=2.0)
    assert ep5["oversaturated"] is False


def test_ep5_free_flow_speed(ep5):
    assert ep5["tau_sut_kin"] == pytest.approx(71.1, abs=0.5)
    assert ep5["tau_tt_kin"] == pytest.approx(92.2, abs=0.5)
    assert ep5["tau_a_ffs"] == pytest.approx(55.4, abs=0.1)
    assert ep5["ffs_mix"] == pytest.approx(60.1, abs=0.1)
    assert ep5["saf_mix"] == pytest.approx(0.92, abs=0.01)


def test_ep5_breakpoint_is_zero_as_printed(ep5):
    """Equation 26-16 is implemented with the printed ``+ 1``, which zeroes the breakpoint."""
    assert ep5["bp_ao"] == pytest.approx(1400.0)
    assert ep5["bp_mix"] == pytest.approx(0.0)


def test_ep5_speed_and_density(ep5):
    assert ep5["s_calib_cap"] == pytest.approx(37.5, abs=0.3)
    assert ep5["s_calib_90cap"] == pytest.approx(44.3, abs=0.3)
    assert ep5["phi_mix"] == pytest.approx(4.07, abs=0.1)
    assert ep5["s_mix"] == pytest.approx(47.3, abs=0.3)
    assert ep5["d_mix"] == pytest.approx(31.7, abs=0.3)


def test_ep5_oversaturation_returns_null_speed():
    cfg = load("Chapter26", "ep5_mixed_flow.json")
    cfg["v_mix"] = 2000.0
    r = json.loads(tl.analyze_mixed_flow(json.dumps(cfg)))
    assert r["oversaturated"] is True
    assert r["s_mix"] is None and r["d_mix"] is None


# ── Chapter 25 Example Problem 11, composite grade ───────────────────────────


def test_ep11_capacity(ep11):
    want = [1875.0, 1934.0, 1746.0]
    for i, seg in enumerate(ep11["segments"]):
        assert seg["capacity_mix"] == pytest.approx(want[i], abs=2.0)
    assert ep11["governing_segment"] == 2
    assert ep11["capacity_mix"] == pytest.approx(1746.0, abs=2.0)


def test_ep11_segment_speeds_and_travel_times(ep11):
    want_speed = [57.7, 58.7, 47.9]
    want_time = [93.6, 122.7, 75.2]
    for i, seg in enumerate(ep11["segments"]):
        assert seg["s_mix"] == pytest.approx(want_speed[i], abs=0.3)
        assert seg["travel_time"] == pytest.approx(want_time[i], abs=0.7)


def test_ep11_segment2_rate_is_self_consistent(ep11):
    """61.3 s/mi, not the 62.6 printed one line above the division that uses 61.3."""
    assert ep11["segments"][1]["tau_mix"] == pytest.approx(61.3, abs=0.2)
    assert ep11["segments"][1]["s_mix"] == pytest.approx(58.7, abs=0.2)


def test_ep11_overall(ep11):
    """291.5 s, not the 294 s of the Step 7 prose."""
    assert ep11["total_length"] == pytest.approx(4.5)
    assert ep11["total_travel_time"] == pytest.approx(291.5, abs=1.5)
    assert ep11["s_mix_overall"] == pytest.approx(55.6, abs=0.3)


def test_ep11_exhibit_25_110_space_speeds(ep11):
    want = [[58.7, 57.0, 50.6], [59.5, 60.9, 51.8], [49.9, 46.6, 36.3]]
    for i, seg in enumerate(ep11["segments"]):
        for k in range(3):
            assert seg["space_speeds"][k] == pytest.approx(want[i][k], abs=0.5)


def test_ep11_exhibit_25_111_overall_space_speeds(ep11):
    for k, want in enumerate([56.8, 55.8, 47.0]):
        assert ep11["overall_space_speeds"][k] == pytest.approx(want, abs=0.4)


def test_ep11_exhibit_25_109_spot_speeds(ep11):
    """The end-of-Segment-1 row is the corrected 56.4 / 56.1 / 46.1."""
    for k in range(3):
        assert ep11["entry_spot_speeds"][k] == pytest.approx(59.5, abs=0.3)
    want = [[56.4, 56.1, 46.1], [60.9, 60.9, 54.0], [45.2, 42.2, 31.8]]
    for i, seg in enumerate(ep11["segments"]):
        for k in range(3):
            assert seg["spot_speeds"][k] == pytest.approx(want[i][k], abs=1.0)


def test_ep11_speed_is_carried_between_segments(ep11):
    """Segment 2 must be entered below free-flow speed, or the chaining is not happening."""
    s1 = ep11["segments"][0]
    assert 3600.0 / s1["tau_f_sut_kin"] == pytest.approx(60.9, abs=1.0)
    assert 3600.0 / s1["tau_f_tt_kin"] == pytest.approx(49.5, abs=1.0)
    assert ep11["segments"][1]["decelerating"] is False


# ── Error surface ────────────────────────────────────────────────────────────


def test_undigitised_grade_raises_naming_what_is_missing():
    cfg = load("Chapter26", "ep5_mixed_flow.json")
    cfg["grade"] = 7.0
    with pytest.raises(ValueError, match="digitised"):
        tl.analyze_mixed_flow(json.dumps(cfg))


def test_composite_grade_outside_stage1_raises():
    cfg = load("Chapter25", "ep11_composite_grade.json")
    cfg["segments"].reverse()
    with pytest.raises(ValueError, match="2.5 mi/h"):
        tl.analyze_composite_grade(json.dumps(cfg))


def test_malformed_config_raises():
    with pytest.raises(ValueError, match="invalid mixed-flow config"):
        tl.analyze_mixed_flow("{not json")
    with pytest.raises(ValueError, match="invalid composite-grade config"):
        tl.analyze_composite_grade("{}")


def test_out_of_range_inputs_raise():
    cfg = load("Chapter26", "ep5_mixed_flow.json")
    cfg["p_tt"] = 0.99
    with pytest.raises(ValueError, match="truck proportions"):
        tl.analyze_mixed_flow(json.dumps(cfg))
