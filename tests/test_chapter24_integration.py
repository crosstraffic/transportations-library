"""Integration tests for the HCM Chapter 24 (Off-Street Pedestrian and Bicycle
Facilities) Python bindings, validated against HCM 7th Edition, Chapter 35
(Pedestrians and Bicycles: Supplemental), Example Problems 1 and 2."""

import json
from pathlib import Path

import pytest

tl = pytest.importorskip("transportations_library")

CASES_DIR = Path(__file__).parent / "ExampleCases" / "hcm" / "OffStreetPedBike"


def load_case(name: str) -> dict:
    with open(CASES_DIR / name) as f:
        return json.load(f)


class TestExampleProblem1:
    """HCM Chapter 35, Example Problem 1: pedestrian LOS on shared-use and
    exclusive paths."""

    def test_shared_use_path_pedestrian_los(self):
        case = load_case("case1.json")
        inputs = case["shared_use_path"]
        expected = case["expected"]

        path = tl.SharedUsePathPedestrian(
            bicycle_demand_same_direction=inputs["bicycle_demand_same_direction"],
            bicycle_demand_opposing=inputs["bicycle_demand_opposing"],
            phf=inputs["phf"],
            pedestrian_speed=inputs["pedestrian_speed"],
            bicycle_speed=inputs["bicycle_speed"],
        )
        los = path.analyze()

        # Published: F_p = 90, F_m = 151, F = 166 events/h -> LOS E.
        assert path.passing_events == pytest.approx(
            expected["passing_events_per_hour"], abs=0.5
        )
        assert path.meeting_events == pytest.approx(
            expected["meeting_events_per_hour"], abs=0.5
        )
        assert path.total_events == pytest.approx(
            expected["total_events_per_hour"], abs=0.5
        )
        assert los == expected["shared_use_path_pedestrian_los"]

    def test_exclusive_path_pedestrian_los(self):
        case = load_case("case1.json")
        inputs = case["exclusive_path"]
        expected = case["expected"]

        facility = tl.ExclusivePedestrianFacility(
            total_walkway_width=inputs["total_walkway_width"],
            fixed_object_width=inputs["fixed_object_width"],
            peak_15min_volume=inputs["peak_15min_volume"],
            phf=inputs["phf"],
            pedestrian_speed=inputs["pedestrian_speed"],
            facility_type="walkway",
            flow_type="random",
        )
        los = facility.analyze()

        # Published: W_E = 5 ft, v_p = 1.33 p/ft/min, A_p = 180 ft2/p -> LOS A.
        assert facility.effective_width == pytest.approx(
            expected["effective_width_ft"], abs=1e-9
        )
        assert facility.unit_flow_rate == pytest.approx(
            expected["unit_flow_rate_p_ft_min"], abs=0.005
        )
        assert facility.pedestrian_space == pytest.approx(
            expected["pedestrian_space_ft2_p"], abs=0.5
        )
        assert los == expected["exclusive_path_pedestrian_los"]


class TestExampleProblem2:
    """HCM Chapter 35, Example Problem 2: bicycle LOS on a shared-use path."""

    def test_bicycle_los_on_shared_use_path(self):
        case = load_case("case2.json")
        inputs = case["bicycle_facility"]
        expected = case["expected"]

        facility = tl.OffStreetBicycleFacility(
            path_width=inputs["path_width"],
            segment_length=inputs["segment_length"],
            has_centerline=inputs["has_centerline"],
            two_way_demand=inputs["two_way_demand"],
            directional_split=inputs["directional_split"],
            phf=inputs["phf"],
            mode_splits=[g["mode_split"] for g in inputs["user_groups"]],
            mode_speeds=[g["average_speed"] for g in inputs["user_groups"]],
            mode_speed_sds=[
                g["speed_standard_deviation"] for g in inputs["user_groups"]
            ],
        )
        los = facility.analyze()

        # Step 1 (Equation 24-8), published (rounded to whole users/h):
        # 104 bicycles/h, 38 p/h, 19 runners/h, 19 skaters/h, 9 children/h.
        qs = facility.subject_flow_rates
        assert qs[0] == pytest.approx(expected["directional_bicycle_flow_rate"], abs=0.5)
        assert qs[1] == pytest.approx(
            expected["directional_pedestrian_flow_rate"], abs=0.5
        )
        assert qs[2] == pytest.approx(expected["directional_runner_flow_rate"], abs=0.5)
        assert qs[3] == pytest.approx(
            expected["directional_inline_skater_flow_rate"], abs=0.5
        )
        assert qs[4] == pytest.approx(
            expected["directional_child_bicyclist_flow_rate"], abs=0.5
        )

        # Step 2, published: A_T = 2.42 passings/min.
        assert facility.active_passings_per_minute == pytest.approx(
            expected["active_passings_per_minute"], abs=0.01
        )
        # Step 3, published: M_T = 8.33 meetings/min (published M_1 used a
        # 6.6 mi/h runner speed, a typo for the 6.5 mi/h default).
        assert facility.meetings_per_minute == pytest.approx(
            expected["meetings_per_minute"], abs=0.03
        )
        # Step 4, published: 2 effective lanes.
        assert facility.effective_lanes == expected["effective_lanes"]
        # Steps 5-6, published: P_Tds = 0.8334, DP_m = 1.82.
        assert facility.total_probability_delayed_passing == pytest.approx(
            expected["total_probability_delayed_passing"], abs=0.002
        )
        assert facility.delayed_passings_per_minute == pytest.approx(
            expected["delayed_passings_per_minute"], abs=0.01
        )
        # Step 7, published: BLOS = 2.69 -> LOS D.
        assert facility.blos_score == pytest.approx(expected["blos_score"], abs=0.01)
        assert los == expected["bicycle_los"]

    def test_low_volume_path_adjustment(self):
        """Step 8: paths with <= 5 weighted events/min are assigned LOS A."""
        facility = tl.OffStreetBicycleFacility(
            path_width=8.0, segment_length=1.0, two_way_demand=20.0
        )
        los = facility.analyze()
        assert facility.weighted_events_per_minute <= 5.0
        assert los == "A"
