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

    def test_shared_major_left_capacities(self, twsc_ep4):
        # Step 7d p*_0 = 0.856 substitution yields c_m,7 = c_m,10 = 47 veh/h.
        assert twsc_ep4.get_movement_capacity("7") == pytest.approx(47.0, abs=1.0)
        assert twsc_ep4.get_movement_capacity("10") == pytest.approx(47.0, abs=1.0)

    def test_major_left_config_binding(self, twsc_ep4):
        # case3.json declares shared major-left lanes on both approaches.
        assert twsc_ep4.get_major_left_config("EB") == ("shared", None)
        assert twsc_ep4.get_major_left_config("WB") == ("shared", None)

    def test_rank1_major_delay(self, twsc_ep4):
        # Step 11b Rank 1 delay d_2+3 = d_5+6 = 1.3 s (Equations 20-62/20-63).
        d23, d56 = twsc_ep4.rank1_major_delay
        assert d23 == pytest.approx(1.3, abs=0.1)
        assert d56 == pytest.approx(1.3, abs=0.1)

    def test_approach_and_intersection_delay(self, twsc_ep4):
        # Published d_A,EB = d_A,WB = 1.9 s (with Step 11b Rank 1 delay);
        # d_A,NB = d_A,SB = 241 s; d_I = 34.1 s.
        d_eb, d_wb, d_nb, d_sb = twsc_ep4.approach_delays
        assert d_eb == pytest.approx(1.9, abs=0.5)
        assert d_wb == pytest.approx(1.9, abs=0.5)
        assert d_nb == pytest.approx(241.0, abs=5.0)
        assert d_sb == pytest.approx(241.0, abs=5.0)
        assert twsc_ep4.intersection_delay == pytest.approx(34.1, abs=0.5)

    def test_exclusive_major_left_no_rank1_delay(self):
        # Overriding to exclusive major-left lanes removes the p* substitution
        # and the Rank 1 delay, raising c_m,7 above the shared-lane value.
        with open(FIXTURE_EP4) as f:
            config = f.read()
        analysis = tl.Twsc(config)
        analysis.set_major_left_config("EB", "exclusive")
        analysis.set_major_left_config("WB", "exclusive")
        analysis.analyze()
        assert analysis.rank1_major_delay is None
        assert analysis.get_movement_capacity("7") > 47.0

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
