"""Python-binding integration test for HCM Chapter 13 (freeway weaving).

Mirrors tests/ExampleCases/hcm/Weaving/case1.json (HCM Chapter 27,
Example Problem 1: LOS of a major weaving segment) through the PyO3
WeavingSegment class. Tolerances match the Rust integration test:
flows/capacities within a few units (published values are rounded),
speeds +-0.5 mi/h, densities +-0.5 pc/mi/ln, exact LOS letters.
"""

import json
from pathlib import Path

import pytest

tl = pytest.importorskip("transportations_library")

CASE1 = Path(__file__).parent / "ExampleCases" / "hcm" / "Weaving" / "case1.json"

WEAVING_TYPE_MAP = {"OneSided": "one_sided", "TwoSided": "two_sided"}
FACILITY_TYPE_MAP = {"Freeway": "freeway", "MultilaneOrCD": "multilane"}


def load_segment(path):
    data = json.loads(path.read_text())
    return tl.WeavingSegment(
        weaving_type=WEAVING_TYPE_MAP[data["weaving_type"]],
        facility_type=FACILITY_TYPE_MAP[data["facility_type"]],
        length_short=data["length_short"],
        num_lanes=data["num_lanes"],
        num_weaving_lanes=data["num_weaving_lanes"],
        ffs=data["ffs"],
        v_ff=data["v_ff"],
        v_fr=data["v_fr"],
        v_rf=data["v_rf"],
        v_rr=data["v_rr"],
        phf=data["phf"],
        heavy_vehicle_pct=data["heavy_vehicle_pct"],
        terrain=data["terrain"].lower(),
        lc_rf=data["lc_rf"],
        lc_fr=data["lc_fr"],
        lc_rr=data["lc_rr"],
        interchange_density=data["interchange_density"],
        basic_freeway_capacity=data["basic_freeway_capacity"],
        caf=data["caf"],
        saf=data["saf"],
    )


class TestWeavingExampleProblem1:
    """HCM Chapter 27, Example Problem 1 (major weaving segment)."""

    def test_step_methods(self):
        seg = load_segment(CASE1)

        v_w, v_nw, v = seg.determine_demand_flow()
        assert v_w == pytest.approx(1995.0, abs=5.0)
        assert v_nw == pytest.approx(3591.0, abs=5.0)
        assert v == pytest.approx(5586.0, abs=5.0)
        assert seg.volume_ratio == pytest.approx(0.357, abs=0.002)

        lc_min = seg.determine_configuration_characteristics()
        assert lc_min == pytest.approx(798.0, abs=5.0)

        l_max = seg.determine_max_weaving_length()
        assert l_max == pytest.approx(4639.0, abs=5.0)
        assert seg.is_weaving

        capacity = seg.determine_capacity()
        assert capacity == pytest.approx(8038.0, abs=10.0)

        lc_all = seg.determine_lane_changing_rates()
        assert lc_all == pytest.approx(1926.0, abs=8.0)

        s_w, s_nw, s = seg.estimate_speed()
        assert s_w == pytest.approx(54.2, abs=0.5)
        assert s_nw == pytest.approx(52.5, abs=0.5)
        assert s == pytest.approx(53.1, abs=0.5)

        density = seg.determine_density()
        assert density == pytest.approx(26.3, abs=0.5)

        assert seg.determine_los() == "C"
        assert seg.los == "C"

    def test_run_analysis(self):
        seg = load_segment(CASE1)
        assert seg.run_analysis() == "C"
        assert seg.density == pytest.approx(26.3, abs=0.5)
        assert seg.speed_avg == pytest.approx(53.1, abs=0.5)
        assert seg.capacity == pytest.approx(8038.0, abs=10.0)

    def test_repr(self):
        seg = load_segment(CASE1)
        assert "WeavingSegment" in repr(seg)
