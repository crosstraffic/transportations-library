"""Python-binding integration test for HCM Chapter 22 (roundabouts).

Runs the Chapter 33 Roundabout Example Problem 1 fixture through the PyO3
bindings and checks the published answers.
"""

import json
import os

import pytest

tl = pytest.importorskip("transportations_library")

FIXTURE = os.path.join(
    os.path.dirname(__file__), "ExampleCases", "hcm", "Roundabouts", "case1.json"
)


@pytest.fixture
def roundabout():
    if not hasattr(tl, "Roundabouts"):
        pytest.skip("transportations_library built without Roundabouts bindings")
    with open(FIXTURE) as f:
        config = f.read()
    analysis = tl.Roundabouts(config)
    analysis.analyze()
    return analysis


class TestRoundaboutExampleProblem1:
    """HCM Chapter 33, Example Problem 1 (single-lane with bypass lanes)."""

    def test_circulating_flows(self, roundabout):
        assert roundabout.get_circulating_flow_pce("NB") == pytest.approx(796.0, abs=3.0)
        assert roundabout.get_circulating_flow_pce("EB") == pytest.approx(487.0, abs=3.0)

    def test_entry_lane_results(self, roundabout):
        flow, capacity, x, delay, los, q95 = roundabout.get_lane_result("NB", 0)
        assert flow == pytest.approx(420.0, abs=3.0)
        assert capacity == pytest.approx(597.0, abs=5.0)
        assert x == pytest.approx(0.70, abs=0.01)
        assert delay == pytest.approx(22.6, abs=0.5)
        assert los == "C"
        assert q95 == pytest.approx(5.7, abs=0.3)

        _, capacity_wb, _, delay_wb, los_wb, _ = roundabout.get_lane_result("WB", 0)
        assert capacity_wb == pytest.approx(694.0, abs=5.0)
        assert delay_wb == pytest.approx(26.8, abs=0.5)
        assert los_wb == "D"

    def test_bypass_lanes(self, roundabout):
        # Yielding WB bypass (Equation 22-6)
        _, capacity, _, delay, los, _ = roundabout.get_bypass_result("WB")
        assert capacity == pytest.approx(851.0, abs=5.0)
        assert delay == pytest.approx(20.2, abs=0.5)
        assert los == "C"
        # Nonyielding SB bypass: delay assumed 0
        _, _, _, delay_sb, los_sb, _ = roundabout.get_bypass_result("SB")
        assert delay_sb == pytest.approx(0.0, abs=1e-9)
        assert los_sb == "A"
        # No bypass on the NB entry
        assert roundabout.get_bypass_result("NB") is None

    def test_approach_and_intersection(self, roundabout):
        assert roundabout.get_approach_delay("WB") == pytest.approx(23.3, abs=0.5)
        assert roundabout.get_approach_los("WB") == "C"
        assert roundabout.get_approach_delay("SB") == pytest.approx(4.7, abs=0.5)
        assert roundabout.get_approach_los("SB") == "A"
        assert roundabout.intersection_delay == pytest.approx(17.5, abs=0.5)
        assert roundabout.intersection_los == "C"

    def test_json_roundtrip(self, roundabout):
        results = json.loads(roundabout.to_json())
        assert results["intersection_delay"] == pytest.approx(17.5, abs=0.5)
        assert results["wb"]["bypass"] == "Yielding"

    def test_invalid_entry(self, roundabout):
        with pytest.raises(ValueError):
            roundabout.get_lane_result("XX", 0)
