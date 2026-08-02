"""Integration test for the HCM Chapter 19 (Signalized Intersections)
Python bindings.

Runs the HCM 7th Edition Chapter 31 Example Problem 1 fixture
(tests/ExampleCases/hcm/Signalized/case1.json) through the full motorized
vehicle methodology and checks the published answers of Exhibit 31-81:
intersection control delay 45.9 s/veh (tolerance +/-0.5) and LOS D
(exact).
"""

import json
import pathlib

import pytest

tl = pytest.importorskip("transportations_library")

FIXTURE = pathlib.Path(__file__).parent / "ExampleCases" / "hcm" / "Signalized" / "case1.json"


@pytest.fixture
def example_problem_1():
    ix = tl.SignalizedIntersection(FIXTURE.read_text())
    ix.analyze()
    return ix


class TestChapter19Signalized:
    def test_class_exposed(self):
        assert hasattr(tl, "SignalizedIntersection")

    def test_invalid_json_raises(self):
        with pytest.raises(ValueError):
            tl.SignalizedIntersection("{not json}")

    def test_intersection_delay_and_los(self, example_problem_1):
        """HCM Ch. 31 Exhibit 31-81: d_I = 45.9 s/veh, LOS D."""
        ix = example_problem_1
        assert ix.intersection_delay_s == pytest.approx(45.9, abs=0.5)
        assert ix.intersection_los == "D"

    def test_approach_delay_and_los(self, example_problem_1):
        """HCM Ch. 31 Exhibit 31-81 approach rows (tolerance +/-0.5 s/veh)."""
        ix = example_problem_1
        expected = {"EB": (32.4, "C"), "WB": (37.0, "D"), "NB": (70.0, "E"), "SB": (19.6, "B")}
        for direction, (delay, los) in expected.items():
            assert ix.approach_delay_s(direction) == pytest.approx(delay, abs=0.5)
            assert ix.approach_los(direction) == los

    def test_lane_groups_json(self, example_problem_1):
        """Twelve lane groups; NB through lane group is oversaturated (LOS F)."""
        ix = example_problem_1
        groups = json.loads(ix.lane_groups_json())
        assert len(groups) == 12
        assert ix.num_lane_groups == 12
        nb_through = next(
            g for g in groups if g["direction"] == "NB" and g["kind"] == "ExclusiveThrough"
        )
        assert nb_through["vc_ratio"] == pytest.approx(1.05, abs=0.02)
        assert nb_through["los"] == "F"

    def test_to_json_roundtrip(self, example_problem_1):
        ix = example_problem_1
        again = tl.SignalizedIntersection(ix.to_json())
        assert again.intersection_los == "D"
        assert again.cycle_length_s == pytest.approx(101.8)

    def test_unknown_approach_raises(self, example_problem_1):
        with pytest.raises(ValueError):
            example_problem_1.approach_delay_s("XX")


class TestSignalizedMultimodalLOS:
    """HCM Chapter 31 Example Problems 2-4: pedestrian and bicycle LOS and the
    two-stage crossing delay, through the PyO3 JSON functions."""

    def test_ep3_bicycle_los(self):
        cfg = {
            "saturation_flow": 2000.0, "effective_green_s": 48.0, "cycle_length_s": 120.0,
            "bicycle_flow": 120.0, "cross_street_width_ft": 70.0, "total_width_ft": 17.0,
            "v_left": 85.0, "v_through": 924.0, "v_right": 77.0, "num_through_lanes": 2.0,
        }
        r = json.loads(tl.analyze_signalized_bicycle(json.dumps(cfg)))
        assert r["capacity"] == pytest.approx(800.0, abs=1.0)
        assert r["delay"] == pytest.approx(23.0, abs=0.1)
        assert r["los_score"] == pytest.approx(2.45, abs=0.01)
        assert r["los"] == "B"

    def test_ep2_pedestrian_los(self):
        cfg = {
            "cycle_length_s": 80.0, "walk_setting_s": 7.0, "lanes_crossed": 2.0,
            "v_rtor": 30.0, "v_lt_perm": 42.0, "num_rtci": 0.0,
            "crossed_street_volume_sum": 986.0, "crossed_street_lanes": 2.0, "speed_85_mph": 35.0,
        }
        r = json.loads(tl.analyze_signalized_pedestrian(json.dumps(cfg)))
        assert r["delay"] == pytest.approx(29.8, abs=0.1)
        assert r["los_score"] == pytest.approx(2.37, abs=0.02)
        assert r["los"] == "B"

    def test_ep4_two_stage_crossing(self):
        cfg = {
            "cycle_length_s": 140.0, "walk_setting_x_s": 5.0, "walk_setting_y_s": 5.0,
            "first_stage_distance_ft": 56.0, "walk_speed_fps": 3.3,
            "walk_start_x_s": 78.0, "walk_start_y_s": 112.0,
        }
        assert tl.signalized_two_stage_crossing_delay(json.dumps(cfg)) == pytest.approx(78.0, abs=0.5)
