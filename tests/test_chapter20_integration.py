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
FIXTURE_EP4 = os.path.join(
    os.path.dirname(__file__), "ExampleCases", "hcm", "Twsc", "case3.json"
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


@pytest.fixture
def twsc_ep4():
    if not hasattr(tl, "Twsc"):
        pytest.skip("transportations_library built without Twsc bindings")
    with open(FIXTURE_EP4) as f:
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


class TestTwscExampleProblem4:
    """HCM Chapter 32, TWSC Example Problem 4 (upstream-signal platoon
    blockage; Step 5b, Equations 20-19 through 20-21)."""

    def test_platoon_blockage_inputs_round_trip(self, twsc_ep4):
        # The p_b,x inputs parsed from case3.json are exposed by the binding.
        pb = twsc_ep4.platoon_blockage
        assert pb is not None
        pb1, pb4, pb7, pb8, pb9, pb10, pb11, pb12 = pb
        assert pb1 == pytest.approx(0.170)
        assert pb4 == pytest.approx(0.170)
        assert pb7 == pytest.approx(0.260)
        assert pb10 == pytest.approx(0.260)

    def test_platooned_potential_capacities(self, twsc_ep4):
        # Equations 20-19 through 20-21 platooned potential capacities.
        assert twsc_ep4.get_potential_capacity("1") == pytest.approx(750.0, abs=5.0)
        assert twsc_ep4.get_potential_capacity("9") == pytest.approx(859.0, abs=5.0)
        assert twsc_ep4.get_potential_capacity("7") == pytest.approx(73.0, abs=5.0)

    def test_minor_left_turn_los_f(self, twsc_ep4):
        # Movement 7 (NB minor left) is oversaturated: LOS F.
        capacity, _delay, los, _q95 = twsc_ep4.get_lane_result("NB", 0)
        assert los == "F"

    def test_set_platoon_blockage_setter(self):
        # Setting all-zero p_b reduces Step 5 to Equation 20-18; the platooned
        # potential capacity then differs from the case3 platooned value.
        with open(FIXTURE_EP4) as f:
            config = f.read()
        analysis = tl.Twsc(config)
        analysis.set_platoon_blockage(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0)
        analysis.analyze()
        # All-zero p_b falls back to Equation 20-18 (v_c,1 = 1,086 continuous),
        # which is lower than the platooned 750 veh/h because platooning
        # concentrates the conflict into the blocked period and frees the rest.
        assert analysis.get_potential_capacity("1") == pytest.approx(644.0, abs=8.0)
