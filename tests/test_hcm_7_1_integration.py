"""Python-binding integration tests for HCM Edition 7.1.

Edition 7.1 (November 2025) replaces Chapters 13, 14, 27, and 28 with new weaving, merge, and
diverge methodologies from NCHRP Research Report 1038. It is selected per segment via the
``version`` argument, the way a documentation site lets a reader pick a language version.

These mirror the Rust integration tests: Chapter 27 Example Problems 1 and 2 for weaving, and
Chapter 28 Example Problem 1 for a merge, through the PyO3 classes.
"""

import json

import pytest

tl = pytest.importorskip("transportations_library")


def test_version_picker_lists_both_editions():
    versions = tl.hcm_versions()
    assert versions == ["7", "7.1"]
    assert tl.hcm_latest_version() == "7.1"


def test_only_the_four_replaced_chapters_change():
    for chapter in (13, 14, 27, 28):
        assert tl.hcm_version_changes_chapter("7.1", chapter) is True
    for chapter in (10, 11, 12, 15, 19, 22, 26):
        assert tl.hcm_version_changes_chapter("7.1", chapter) is False
    # Nothing changes under the 7th Edition, by definition.
    for chapter in (13, 14, 27, 28):
        assert tl.hcm_version_changes_chapter("7", chapter) is False


def test_unknown_version_is_rejected():
    with pytest.raises(ValueError):
        tl.hcm_version_changes_chapter("6", 13)
    with pytest.raises(ValueError):
        tl.WeavingSegment(version="8.0")


def test_default_version_is_the_seventh_edition():
    assert tl.WeavingSegment().version == "7"
    assert tl.RampSegment().version == "7"
    # A segment left on the default edition has no 7.1 result.
    seg = tl.WeavingSegment()
    seg.run_analysis()
    assert seg.analysis_v7_1() is None


def test_chapter_27_example_problem_1_complex_weave():
    """Chapter 27 EP1: a "Complex 0-1" weave on a four-lane urban freeway."""
    seg = tl.WeavingSegment(
        version="7.1",
        weaving_type="one_sided",
        length_short=1500.0,
        num_lanes=4,
        ffs=65.0,
        v_ff=1815.0,
        v_fr=692.0,
        v_rf=1037.0,
        v_rr=1297.0,
        phf=0.91,
        heavy_vehicle_pct=0.05,
        terrain="level",
        lc_rf=0,
        lc_fr=1,
        nw_rf=2,
        nw_fr=1,
    )
    assert seg.version == "7.1"
    assert seg.run_analysis() == "C"

    a = json.loads(seg.analysis_v7_1())
    assert a["class"] == "Complex"
    assert a["speed_basic"] == pytest.approx(65.0, abs=1e-9)
    assert a["weaving_intensity"] == pytest.approx(0.006336, abs=5e-6)
    assert a["speed_impedance"] == pytest.approx(5.68, abs=0.02)
    assert a["speed_avg"] == pytest.approx(59.32, abs=0.02)
    assert a["capacity_per_lane"] == pytest.approx(1866.0, abs=2.0)
    assert a["dc_ratio"] == pytest.approx(0.75, abs=0.005)
    assert a["density"] == pytest.approx(23.6, abs=0.1)
    assert a["los"] == "C"


def test_chapter_27_example_problem_2_simple_weave():
    """Chapter 27 EP2: a simple weave, demands already stated as pc/h flow rates."""
    seg = tl.WeavingSegment(
        version="7.1",
        weaving_type="one_sided",
        length_short=1000.0,
        num_lanes=4,
        ffs=75.0,
        v_ff=4000.0,
        v_fr=600.0,
        v_rf=300.0,
        v_rr=100.0,
        phf=1.0,
        heavy_vehicle_pct=0.0,
        lc_rf=1,
        lc_fr=1,
        nw_rf=1,
        nw_fr=1,
    )
    assert seg.run_analysis() == "B"

    a = json.loads(seg.analysis_v7_1())
    assert a["class"] == "Simple"
    # 2,200 + 10(75 - 50) = 2,450, capped at the Exhibit 12-4 maximum.
    assert a["capacity_basic_adj"] == pytest.approx(2400.0, abs=1e-9)
    assert a["speed_basic"] == pytest.approx(74.31, abs=0.01)
    assert a["speed_avg"] == pytest.approx(70.70, abs=0.01)
    assert a["capacity_per_lane"] == pytest.approx(1992.0, abs=2.0)
    assert a["density"] == pytest.approx(17.7, abs=0.05)


def test_chapter_28_example_problem_1_on_ramp():
    """Chapter 28 EP1: an isolated one-lane, right-hand on-ramp to a four-lane freeway."""
    seg = tl.RampSegment(
        version="7.1",
        ramp_type="on_ramp",
        ramp_side="right",
        ramp_lanes=1,
        freeway_lanes=2,
        freeway_ffs=60.0,
        ramp_ffs=45.0,
        accel_lane_length=740.0,
        freeway_demand=2500.0,
        ramp_demand=535.0,
        phf=0.90,
        heavy_vehicle_pct=0.05,
        terrain="level",
    )
    assert seg.run_analysis() == "E"

    a = json.loads(seg.analysis_v7_1())
    assert a["flow_freeway"] == pytest.approx(2918.0, abs=2.0)
    assert a["flow_ramp"] == pytest.approx(624.0, abs=1.0)
    assert a["speed_basic"] == pytest.approx(59.47, abs=0.02)
    assert a["speed_impedance"] == pytest.approx(4.37, abs=0.02)
    assert a["speed_avg"] == pytest.approx(55.10, abs=0.03)
    assert a["capacity_per_lane"] == pytest.approx(1882.0, abs=3.0)
    assert a["capacity_neighboring_freeway"] == pytest.approx(4600.0, abs=1e-9)
    assert a["capacity_ramp_roadway"] == pytest.approx(2100.0, abs=1e-9)
    assert a["density"] == pytest.approx(32.1, abs=0.1)
    assert a["los"] == "E"


def test_the_two_editions_disagree_on_the_same_segment():
    """The editions are different models, so the same inputs give different answers."""

    def build(version):
        return tl.WeavingSegment(
            version=version,
            weaving_type="one_sided",
            length_short=1500.0,
            num_lanes=4,
            num_weaving_lanes=2,
            ffs=65.0,
            v_ff=1815.0,
            v_fr=692.0,
            v_rf=1037.0,
            v_rr=1297.0,
            phf=0.91,
            heavy_vehicle_pct=0.05,
            lc_rf=0,
            lc_fr=1,
            nw_rf=2,
            nw_fr=1,
        )

    v7, v71 = build("7"), build("7.1")
    v7.run_analysis()
    v71.run_analysis()
    assert abs(v7.density - v71.density) > 1.0


def test_version_is_settable_after_construction():
    seg = tl.WeavingSegment()
    assert seg.version == "7"
    seg.version = "7.1"
    assert seg.version == "7.1"
    with pytest.raises(ValueError):
        seg.version = "nonsense"
