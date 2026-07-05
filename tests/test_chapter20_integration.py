"""Python-binding integration test for HCM Chapter 20 (TWSC intersections).

Runs the Chapter 32 TWSC Example Problem 1 fixture through the PyO3
bindings and checks the published answers (LOS exact; delays within
0.5 s/veh; capacities within 5 veh/h).
"""

import json
import os

import pytest

tl = pytest.importorskip("transportations_library")

FIXTURE = os.path.join(
    os.path.dirname(__file__), "ExampleCases", "hcm", "Twsc", "case1.json"
)


@pytest.fixture
def twsc():
    if not hasattr(tl, "Twsc"):
        pytest.skip("transportations_library built without Twsc bindings")
    with open(FIXTURE) as f:
        config = f.read()
    analysis = tl.Twsc(config)
    analysis.analyze()
    return analysis


class TestTwscExampleProblem1:
    """HCM Chapter 32, TWSC Example Problem 1 (three-leg intersection)."""

    def test_movement_capacities(self, twsc):
        assert twsc.get_movement_capacity("4") == pytest.approx(1238.0, abs=5.0)
        assert twsc.get_movement_capacity("9") == pytest.approx(760.0, abs=5.0)
        assert twsc.get_movement_capacity("7") == pytest.approx(268.0, abs=5.0)

    def test_major_left_turn_delay_and_los(self, twsc):
        assert twsc.get_movement_delay("4") == pytest.approx(8.3, abs=0.5)
        assert twsc.get_movement_los("4") == "A"
        assert twsc.get_movement_queue_95("4") == pytest.approx(0.4, abs=0.2)

    def test_minor_shared_lane(self, twsc):
        assert twsc.get_lane_count("NB") == 1
        capacity, delay, los, q95 = twsc.get_lane_result("NB", 0)
        assert capacity == pytest.approx(521.0, abs=5.0)
        assert delay == pytest.approx(14.9, abs=0.5)
        assert los == "B"
        assert q95 == pytest.approx(1.3, abs=0.2)

    def test_approach_and_intersection_delay(self, twsc):
        d_eb, d_wb, d_nb, _ = twsc.approach_delays
        assert d_eb == pytest.approx(0.0, abs=0.5)
        assert d_wb == pytest.approx(2.9, abs=0.5)
        assert d_nb == pytest.approx(14.9, abs=0.5)
        assert twsc.intersection_delay == pytest.approx(4.1, abs=0.5)

    def test_json_roundtrip(self, twsc):
        results = json.loads(twsc.to_json())
        assert results["geometry"]["is_three_leg"] is True
        assert results["intersection_delay"] == pytest.approx(4.1, abs=0.5)

    def test_invalid_movement_label(self, twsc):
        with pytest.raises(ValueError):
            twsc.get_movement_capacity("13")
