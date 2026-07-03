"""Python-binding integration test for HCM Chapter 14 (merge/diverge).

Mirrors tests/ExampleCases/hcm/MergeDiverge/case2.json (HCM Chapter 28,
Example Problem 2, first off-ramp: two adjacent single-lane right-hand
off-ramps on a six-lane freeway) through the PyO3 RampSegment class.
Tolerances match the Rust integration test: flows within a few pc/h,
speeds +-0.5 mi/h, densities +-0.5 pc/mi/ln, exact LOS letters.
"""

import json
from pathlib import Path

import pytest

tl = pytest.importorskip("transportations_library")

CASE2 = Path(__file__).parent / "ExampleCases" / "hcm" / "MergeDiverge" / "case2.json"

RAMP_TYPE_MAP = {
    "OnRamp": "on_ramp",
    "OffRamp": "off_ramp",
    "MajorMerge": "major_merge",
    "MajorDiverge": "major_diverge",
}
RAMP_LANES_MAP = {"OneLane": 1, "TwoLane": 2}
ADJACENT_MAP = {"None": "none", "OnRamp": "on_ramp", "OffRamp": "off_ramp"}


def load_segment(path):
    data = json.loads(path.read_text())
    return tl.RampSegment(
        ramp_type=RAMP_TYPE_MAP[data["ramp_type"]],
        ramp_side=data["ramp_side"].lower(),
        ramp_lanes=RAMP_LANES_MAP[data["ramp_lanes"]],
        freeway_lanes=data["freeway_lanes"],
        freeway_ffs=data["freeway_ffs"],
        ramp_ffs=data["ramp_ffs"],
        decel_lane_length=data.get("decel_lane_length"),
        accel_lane_length=data.get("accel_lane_length"),
        freeway_demand=data["freeway_demand"],
        ramp_demand=data["ramp_demand"],
        phf=data["phf"],
        heavy_vehicle_pct=data["heavy_vehicle_pct"],
        ramp_heavy_vehicle_pct=data.get("ramp_heavy_vehicle_pct"),
        terrain=data["terrain"].lower(),
        adjacent_upstream=ADJACENT_MAP[data["adjacent_upstream"]],
        adjacent_downstream=ADJACENT_MAP[data["adjacent_downstream"]],
        upstream_distance=data.get("upstream_distance"),
        upstream_ramp_flow=data.get("upstream_ramp_flow"),
        downstream_distance=data.get("downstream_distance"),
        downstream_ramp_flow=data.get("downstream_ramp_flow"),
        caf=data["caf"],
        saf=data["saf"],
    )


class TestMergeDivergeExampleProblem2:
    """HCM Chapter 28, Example Problem 2, first off-ramp (six-lane freeway)."""

    def test_step_methods(self):
        seg = load_segment(CASE2)

        v_f, v_r = seg.determine_demand_flow()
        assert v_f == pytest.approx(5093.0, abs=5.0)
        assert v_r == pytest.approx(340.0, abs=2.0)

        v12 = seg.estimate_v12()
        # Downstream off-ramp is beyond L_EQ = 657 ft -> Equation 14-9,
        # P_FD = 0.617, v_12 = 3,273 pc/h
        assert seg.p_f == pytest.approx(0.617, abs=0.002)
        assert v12 == pytest.approx(3273.0, abs=6.0)

        capacity = seg.determine_capacity()
        assert capacity == pytest.approx(6900.0)
        assert seg.capacity_ramp == pytest.approx(2000.0)
        assert seg.demand_exceeds_capacity is False
        assert seg.exceeds_max_desirable is False

        density = seg.determine_density()
        assert density == pytest.approx(27.9, abs=0.5)
        assert seg.determine_los() == "C"

        s_r, s_o, s = seg.estimate_speed()
        assert s_r == pytest.approx(52.9, abs=0.5)
        assert s_o == pytest.approx(62.6, abs=0.5)
        assert s == pytest.approx(56.0, abs=0.5)

    def test_run_analysis(self):
        seg = load_segment(CASE2)
        assert seg.run_analysis() == "C"
        assert seg.density == pytest.approx(27.9, abs=0.5)
        assert seg.speed_avg == pytest.approx(56.0, abs=0.5)
        assert seg.los == "C"

    def test_repr(self):
        seg = load_segment(CASE2)
        assert "RampSegment" in repr(seg)
