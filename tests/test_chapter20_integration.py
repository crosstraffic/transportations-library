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


FIXTURE_EP2 = os.path.join(
    os.path.dirname(__file__), "ExampleCases", "hcm", "Twsc", "case4_pedestrian.json"
)


def _pedestrian(scenario):
    """Run one Example Problem 2 scenario through the PyO3 binding."""
    if not hasattr(tl, "analyze_twsc_pedestrian"):
        pytest.skip("transportations_library built without the pedestrian binding")
    with open(FIXTURE_EP2) as f:
        root = json.load(f)
    return json.loads(tl.analyze_twsc_pedestrian(json.dumps(root[scenario])))


class TestChapter20PedestrianMode:
    """HCM Chapter 32, TWSC Example Problem 2: the Chapter 20 Section 5
    pedestrian mode, mirroring tests/chapter20_integration.rs. This is the
    pedestrian mode proper, not the Section 4 pedestrian-impedance adjustment
    that the Twsc class applies."""

    def test_scenario_a_unmarked_single_stage(self):
        # 46-ft crossing of four lanes, 0% yield rate: t_c = 12.5 s,
        # P_b = 0.771, P_d = 0.997, d_g = 761 s, d_p = 761 s, LOS F.
        r = _pedestrian("scenario_a")
        assert len(r["stages"]) == 1
        s = r["stages"][0]
        assert s["critical_headway"] == pytest.approx(12.5, abs=0.05)
        assert s["prob_blocked_lane"] == pytest.approx(0.771, abs=0.001)
        assert s["prob_delayed_crossing"] == pytest.approx(0.997, abs=0.001)
        assert s["gap_delay"] == pytest.approx(761.0, rel=0.005)
        assert r["delay"] == pytest.approx(761.0, rel=0.005)
        assert r["odds_satisfied_no_delay"] == pytest.approx(1.066, abs=0.005)
        assert r["odds_satisfied_delay"] == pytest.approx(0.159, abs=0.001)
        assert r["prob_non_delayed"] == pytest.approx(0.003, abs=0.001)
        assert r["los"] == "F"

    def test_scenario_b_marked_crosswalk_and_median_refuge(self):
        # Two 20-ft stages, 50% yield rate. Exhibit 32-7: P_d = 0.758,
        # P(Y_1) = 0.314, P_nd = 0.481, P(D) = 0.207, LOS C; d_p = 6.0 s.
        r = _pedestrian("scenario_b")
        assert len(r["stages"]) == 2
        s = r["stages"][0]
        assert s["critical_headway"] == pytest.approx(6.0, abs=0.05)
        assert s["prob_blocked_lane"] == pytest.approx(0.508, abs=0.001)
        assert s["prob_delayed_crossing"] == pytest.approx(0.758, abs=0.001)
        assert s["gap_delay"] == pytest.approx(7.2, abs=0.05)
        assert s["gap_delay_when_delayed"] == pytest.approx(9.5, abs=0.05)
        assert s["average_short_headway"] == pytest.approx(2.3, abs=0.05)
        assert s["yield_events"] == 4
        assert r["delay"] == pytest.approx(6.0, abs=0.5)
        assert r["odds_satisfied_no_delay"] == pytest.approx(13.44, abs=0.05)
        assert r["odds_satisfied_delay"] == pytest.approx(2.00, abs=0.01)
        assert r["prob_yield_first_event"] == pytest.approx(0.314, abs=0.001)
        assert r["prob_non_delayed"] == pytest.approx(0.481, abs=0.001)
        assert r["proportion_dissatisfied"] == pytest.approx(0.207, abs=0.001)
        assert r["los"] == "C"

    def test_scenario_c_adds_rrfb(self):
        # Same as Scenario B plus RRFBs and an 80% yield rate. Exhibit 32-7:
        # P(Y_1) = 0.565, P_nd = 0.670, P(D) = 0.029, LOS A; d_p = 3.0 s.
        r = _pedestrian("scenario_c")
        assert r["delay"] == pytest.approx(3.0, abs=0.5)
        assert r["odds_satisfied_no_delay"] == pytest.approx(95.15, abs=0.15)
        assert r["odds_satisfied_delay"] == pytest.approx(14.15, abs=0.05)
        assert r["prob_yield_first_event"] == pytest.approx(0.565, abs=0.001)
        assert r["prob_non_delayed"] == pytest.approx(0.670, abs=0.001)
        assert r["proportion_dissatisfied"] == pytest.approx(0.029, abs=0.001)
        assert r["los"] == "A"

    def test_countermeasures_improve_los_f_to_c_to_a(self):
        # The Example Problem 2 discussion: a marked crosswalk plus median
        # refuge moves LOS F -> C, and adding RRFBs moves it to A.
        los = [_pedestrian(s)["los"] for s in ("scenario_a", "scenario_b", "scenario_c")]
        assert los == ["F", "C", "A"]

    def test_invalid_config_raises(self):
        if not hasattr(tl, "analyze_twsc_pedestrian"):
            pytest.skip("transportations_library built without the pedestrian binding")
        with pytest.raises(ValueError):
            tl.analyze_twsc_pedestrian("{not json")
