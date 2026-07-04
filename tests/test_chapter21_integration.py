"""Python-binding integration test for HCM Chapter 21 (AWSC intersections).

Runs the Chapter 32 AWSC Example Problem 1 fixture through the PyO3
bindings and checks the published answers.
"""

import json
import os

import pytest

tl = pytest.importorskip("transportations_library")

FIXTURE = os.path.join(
    os.path.dirname(__file__), "ExampleCases", "hcm", "Awsc", "case1.json"
)


@pytest.fixture
def awsc():
    if not hasattr(tl, "Awsc"):
        pytest.skip("transportations_library built without Awsc bindings")
    with open(FIXTURE) as f:
        config = f.read()
    analysis = tl.Awsc(config)
    analysis.analyze()
    return analysis


class TestAwscExampleProblem1:
    """HCM Chapter 32, AWSC Example Problem 1 (single-lane, three-leg)."""

    def test_departure_headways(self, awsc):
        assert awsc.get_departure_headway("EB", 0) == pytest.approx(4.97, abs=0.1)
        assert awsc.get_departure_headway("WB", 0) == pytest.approx(4.74, abs=0.1)
        assert awsc.get_departure_headway("SB", 0) == pytest.approx(5.70, abs=0.1)
        assert awsc.get_degree_of_utilization("EB", 0) == pytest.approx(0.508, abs=0.01)

    def test_service_time(self, awsc):
        assert awsc.get_service_time("EB", 0) == pytest.approx(2.97, abs=0.1)

    def test_lane_delay_and_los(self, awsc):
        assert awsc.get_lane_delay("EB", 0) == pytest.approx(13.0, abs=0.5)
        assert awsc.get_lane_los("EB", 0) == "B"
        assert awsc.get_lane_delay("WB", 0) == pytest.approx(13.5, abs=0.5)
        assert awsc.get_lane_delay("SB", 0) == pytest.approx(10.6, abs=0.5)
        assert awsc.get_lane_queue_95("EB", 0) == pytest.approx(2.9, abs=0.2)

    def test_intersection_delay_and_los(self, awsc):
        assert awsc.intersection_delay == pytest.approx(12.8, abs=0.5)
        assert awsc.intersection_los == "B"

    def test_lane_capacity(self, awsc):
        # "Approximately 720 veh/h" per HCM Chapter 32 (exact bisection on
        # unrounded flows converges near 704 veh/h).
        capacity = awsc.compute_lane_capacity("EB", 0)
        assert capacity == pytest.approx(720.0, abs=20.0)
        assert capacity < 748.0

    def test_json_roundtrip(self, awsc):
        results = json.loads(awsc.to_json())
        assert results["intersection_delay"] == pytest.approx(12.8, abs=0.5)
        assert results["eb"]["geometry_group"] == "G1"

    def test_invalid_approach(self, awsc):
        with pytest.raises(ValueError):
            awsc.get_lane_delay("XX", 0)
