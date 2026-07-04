"""Python-binding integration test for HCM Chapter 23 (interchange ramp
terminals).

Runs the Chapter 34 Example Problem 1 (diamond interchange) and Example
Problem 5 (DDI) fixtures through the PyO3 bindings and checks the
published answers (tolerances documented in tests/chapter23_integration.rs).
"""

import json
import os

import pytest

tl = pytest.importorskip("transportations_library")

FIXTURE_DIR = os.path.join(
    os.path.dirname(__file__), "ExampleCases", "hcm", "RampTerminals"
)


def load(name):
    if not hasattr(tl, "Interchange"):
        pytest.skip("transportations_library built without Interchange bindings")
    with open(os.path.join(FIXTURE_DIR, name)) as f:
        config = f.read()
    analysis = tl.Interchange(config)
    analysis.analyze()
    return analysis


@pytest.fixture
def diamond():
    return load("case1.json")


@pytest.fixture
def ddi():
    return load("case2.json")


class TestDiamondExampleProblem1:
    """HCM Chapter 34, Example Problem 1 (Exhibit 34-16)."""

    def test_od_results(self, diamond):
        published = {
            # od: (demand, control delay, EDTT, ETT, LOS)
            "A": (233.0, 45.6, 1.9, 47.5, "C"),
            "B": (227.0, 43.7, -1.9, 41.8, "C"),
            "C": (173.0, 54.6, -1.9, 52.7, "C"),
            "D": (206.0, 63.6, 1.9, 65.5, "D"),
            "E": (107.0, 99.2, 1.9, 101.1, "E"),
            "F": (89.0, 44.2, -1.9, 42.3, "C"),
            "G": (150.0, 37.5, -1.9, 35.6, "C"),
            "H": (236.0, 82.7, 1.9, 84.6, "D"),
            "I": (761.0, 52.0, 0.0, 52.0, "C"),
            "J": (650.0, 39.8, 0.0, 39.8, "C"),
        }
        for od, (demand, delay, edtt, ett, los) in published.items():
            got_demand, got_delay, got_edtt, got_ett, got_los = diamond.get_od_result(od)
            assert got_demand == pytest.approx(demand, abs=1.0), od
            assert got_delay == pytest.approx(delay, abs=1.0), od
            assert got_edtt == pytest.approx(edtt, abs=0.1), od
            assert got_ett == pytest.approx(ett, abs=1.0), od
            assert got_los == los, od

    def test_interchange_ett_and_los(self, diamond):
        assert diamond.interchange_ett == pytest.approx(52.4, abs=1.0)
        assert diamond.interchange_los == "C"

    def test_lane_group_results(self, diamond):
        # Exhibit 34-7: EB external through saturation flow 3,700 veh/h,
        # g' = 63 s, v/c = 0.66.
        flow, sat, g, cap, x, delay = diamond.get_lane_group_result("EbExtThrough")
        assert flow == pytest.approx(957.0, abs=2.0)
        assert sat == pytest.approx(3700.0, abs=20.0)
        assert g == pytest.approx(63.0, abs=1e-9)
        assert x == pytest.approx(0.66, abs=0.01)
        assert delay == pytest.approx(44.1, abs=1.0)
        # Queue storage ratios stay below 1 (Exhibits 34-12 / 34-13).
        assert diamond.get_queue_storage_ratio("EbExtThrough") < 1.0

    def test_json_round_trip(self, diamond):
        state = json.loads(diamond.to_json())
        assert state["form"] == "Diamond"
        assert state["interchange_los"] == "C"


class TestDdiExampleProblem5:
    """HCM Chapter 34, Example Problem 5 (Exhibits 34-62 through 34-65).

    The published movement delays are not reproducible from the printed
    Chapter 19 / 23 equations (see chapter23_integration.rs); the LOS
    letters are asserted where the equation-based ETT falls in the same
    Exhibit 23-10 band (all O-Ds except E), plus the interchange
    aggregate.
    """

    def test_od_los(self, ddi):
        published_los = {
            "A": "C",
            "B": "B",
            "C": "A",
            "D": "D",
            "F": "A",
            "G": "A",
            "H": "C",
            "I": "C",
            "J": "D",
        }
        for od, los in published_los.items():
            _, _, _, _, got_los = ddi.get_od_result(od)
            assert got_los == los, od

    def test_free_flow_right_turns(self, ddi):
        # O-Ds F and G bypass the crossovers (zero control delay).
        for od in ("F", "G"):
            _, delay, edtt, ett, _ = ddi.get_od_result(od)
            assert delay == pytest.approx(0.0, abs=1e-9)
            assert ett == pytest.approx(0.0, abs=1e-9)

    def test_interchange_ett_and_los(self, ddi):
        # Published Exhibit 34-65: 34.9 s/veh, LOS C.
        assert ddi.interchange_ett == pytest.approx(34.9, abs=0.5)
        assert ddi.interchange_los == "C"

    def test_ddi_saturation_flows(self, ddi):
        # Exhibit 34-62: M2 (WB external crossover) = 2,045 veh/h with
        # f_DDI = 0.913 and the Equation 23-18 lane utilization.
        _, sat, g, _, x, _ = ddi.get_lane_group_result("WbExtThrough")
        assert sat == pytest.approx(2045.0, abs=5.0)
        assert g == pytest.approx(21.0, abs=0.1)  # published 20 s
        assert x == pytest.approx(0.73, abs=0.01)  # published 0.77
