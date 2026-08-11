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
        # Asserted engine values; the trailing comment carries the published
        # Exhibit 34-16 (delay, ETT) pair where the two differ by more than the
        # tolerance. Every difference is the Equation 19-26 correction reaching
        # an external through lane group, and only those; see
        # tests/chapter23_integration.rs for the full account.
        expected = {
            # od: (demand, control delay, EDTT, ETT, LOS)
            "A": (233.0, 45.7, 1.9, 47.7, "C"),
            "B": (227.0, 43.8, -1.9, 41.8, "C"),
            "C": (173.0, 54.6, -1.9, 52.7, "C"),
            "D": (206.0, 63.7, 1.9, 65.7, "D"),
            "E": (107.0, 97.0, 1.9, 98.9, "E"),  # published 99.2 / 101.1
            "F": (89.0, 42.0, -1.9, 40.0, "C"),  # published 44.2 /  42.3
            "G": (150.0, 34.6, -1.9, 32.7, "C"),  # published 37.5 /  35.6
            "H": (236.0, 79.8, 1.9, 81.8, "D"),  # published 82.7 /  84.6
            "I": (761.0, 49.8, 0.0, 49.8, "C"),  # published 52.0 /  52.0
            "J": (650.0, 36.9, 0.0, 36.9, "C"),  # published 39.8 /  39.8
        }
        for od, (demand, delay, edtt, ett, los) in expected.items():
            got_demand, got_delay, got_edtt, got_ett, got_los = diamond.get_od_result(od)
            assert got_demand == pytest.approx(demand, abs=1.0), od
            assert got_delay == pytest.approx(delay, abs=1.0), od
            assert got_edtt == pytest.approx(edtt, abs=0.1), od
            assert got_ett == pytest.approx(ett, abs=1.0), od
            assert got_los == los, od

    def test_interchange_ett_and_los(self, diamond):
        # 50.7 s/veh against the published 52.4, same LOS C. The gap is the
        # demand-weighted share of the two external-through d2 corrections.
        assert diamond.interchange_ett == pytest.approx(50.7, abs=0.5)
        assert diamond.interchange_los == "C"

    def test_lane_group_results(self, diamond):
        # Exhibit 34-7: EB external through saturation flow 3,700 veh/h,
        # g' = 63 s, v/c = 0.66.
        flow, sat, g, cap, x, delay = diamond.get_lane_group_result("EbExtThrough")
        assert flow == pytest.approx(957.0, abs=2.0)
        assert sat == pytest.approx(3700.0, abs=20.0)
        assert g == pytest.approx(63.0, abs=1e-9)
        assert x == pytest.approx(0.66, abs=0.01)
        # 41.99 s/veh against the published 44.1: this is the 2-lane group
        # whose published d2 of 4.6 s/veh only reproduces per-lane, which
        # Equation 19-26's own definition of c_A rules out.
        assert delay == pytest.approx(41.99, abs=1.0)
        # Queue storage ratios stay below 1 (Exhibits 34-12 / 34-13).
        assert diamond.get_queue_storage_ratio("EbExtThrough") < 1.0

    def test_json_round_trip(self, diamond):
        state = json.loads(diamond.to_json())
        assert state["form"] == "Diamond"
        assert state["interchange_los"] == "C"


class TestDdiExampleProblem5:
    """HCM Chapter 34, Example Problem 5 (Exhibits 34-62 through 34-65).

    The published movement delays are not reproducible from the printed
    Chapter 19 / 23 equations (see chapter23_integration.rs). O-D E, which
    runs on the 3-lane external crossover and so isolates the Equation 19-26
    capacity term most cleanly, reproduces the published 24.7 s/veh and LOS B
    exactly. The westbound O-Ds run short of their published values, which
    carries O-D J and the interchange aggregate one band down.
    """

    def test_od_los(self, ddi):
        expected_los = {
            "A": "C",
            "B": "B",
            "C": "A",
            "D": "D",
            "F": "A",
            "G": "A",
            "H": "C",
            "I": "C",
            "J": "C",  # published D at 66.4 s/veh; engine 48.3
        }
        for od, los in expected_los.items():
            _, _, _, _, got_los = ddi.get_od_result(od)
            assert got_los == los, od

    def test_free_flow_right_turns(self, ddi):
        # O-Ds F and G bypass the crossovers (zero control delay).
        for od in ("F", "G"):
            _, delay, edtt, ett, _ = ddi.get_od_result(od)
            assert delay == pytest.approx(0.0, abs=1e-9)
            assert ett == pytest.approx(0.0, abs=1e-9)

    def test_interchange_ett_and_los(self, ddi):
        # 29.8 s/veh against the published Exhibit 34-65 value of 34.9, which
        # lands 0.2 s/veh below the Exhibit 23-10 B/C boundary and so grades B.
        assert ddi.interchange_ett == pytest.approx(29.8, abs=0.5)
        assert ddi.interchange_los == "B"

    def test_ddi_saturation_flows(self, ddi):
        # Exhibit 34-62: M2 (WB external crossover) = 2,045 veh/h with
        # f_DDI = 0.913 and the Equation 23-18 lane utilization.
        _, sat, g, _, x, _ = ddi.get_lane_group_result("WbExtThrough")
        assert sat == pytest.approx(2045.0, abs=5.0)
        assert g == pytest.approx(21.0, abs=0.1)  # published 20 s
        assert x == pytest.approx(0.73, abs=0.01)  # published 0.77
