"""Python-binding integration test for HCM Chapter 18 (Urban Street
Segments).

Runs the Chapter 30, Section 8, Example Problem 1 (eastbound) fixture
through the PyO3 bindings and checks the published Exhibit 30-36 answers
(LOS exact; speeds within 0.5 mi/h; running time and delay within
0.5 s/veh; stop rates within 0.01).
"""

import json
import os

import pytest

tl = pytest.importorskip("transportations_library")

FIXTURE = os.path.join(
    os.path.dirname(__file__), "ExampleCases", "hcm", "UrbanSegments", "case1.json"
)


@pytest.fixture
def segment():
    if not hasattr(tl, "UrbanSegment"):
        pytest.skip("transportations_library built without UrbanSegment bindings")
    with open(FIXTURE) as f:
        config = f.read()
    analysis = tl.UrbanSegment(config)
    analysis.analyze()
    return analysis


class TestUrbanSegmentExampleProblem1:
    """HCM Chapter 30, Example Problem 1: Motorized Vehicle LOS (EB)."""

    def test_free_flow_speeds(self, segment):
        # Exhibit 30-36: base FFS 40.78 mi/h.
        assert segment.base_free_flow_speed_mph == pytest.approx(40.78, abs=0.01)
        assert segment.free_flow_speed_mph == pytest.approx(39.33, abs=0.01)

    def test_running_time_and_speed(self, segment):
        # Exhibit 30-36: running time 33.54 s, running speed 36.59 mi/h.
        assert segment.running_time_s == pytest.approx(33.54, abs=0.5)
        assert segment.running_speed_mph == pytest.approx(36.59, abs=0.5)

    def test_travel_speed_and_delay(self, segment):
        # Exhibit 30-36: through delay 18.310 s/veh, travel speed 23.67 mi/h.
        assert segment.through_delay_s == pytest.approx(18.310, abs=0.5)
        assert segment.travel_speed_mph == pytest.approx(23.67, abs=0.5)

    def test_stop_rates(self, segment):
        # Exhibit 30-36: stop rate 0.547 stops/veh, spatial 1.61 stops/mi.
        assert segment.full_stop_rate == pytest.approx(0.547, abs=0.01)
        assert segment.spatial_stop_rate == pytest.approx(1.61, abs=0.01)

    def test_los_and_score(self, segment):
        # Exhibit 30-36: through v/c 0.52, LOS C, perception score 2.53.
        assert segment.vc_ratio == pytest.approx(0.52, abs=0.005)
        assert segment.los == "C"
        assert segment.perception_score == pytest.approx(2.53, abs=0.01)

    def test_json_round_trip(self, segment):
        result = json.loads(segment.to_json())
        assert result["los"] == "C"
        assert result["travel_speed_mph"] == pytest.approx(23.67, abs=0.5)
