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


class TestUrbanSegmentMultimodalLOS:
    """HCM Chapter 30 Example Problems 2-4: pedestrian, bicycle, transit LOS
    through the PyO3 JSON analysis functions."""

    def test_ep2_pedestrian_los(self):
        cfg = {
            "length_ft": 1320.0, "num_through_lanes": 2.0, "midseg_flow_rate": 940.0,
            "ped_flow_rate": 2000.0, "prop_parking_occupied": 0.20,
            "width_sidewalk_ft": 10.0, "width_buffer_ft": 5.0, "prop_fence": 0.50,
            "width_outside_lane_ft": 12.0, "width_bike_lane_ft": 5.0,
            "width_parking_lane_ft": 9.5, "curb_present": True,
            "motor_running_speed": 33.0, "free_flow_walk_speed": 4.4,
            "ped_delay_parallel": 40.0, "ped_delay_crossing_signal": 80.0,
            "ped_delay_crossing_uncontrolled": 740.0, "ped_los_score_intersection": 3.60,
            "prop_midblock_crossing": 0.35,
        }
        r = json.loads(tl.analyze_pedestrian_segment(json.dumps(cfg)))
        assert r["link_score"] == pytest.approx(2.35, abs=0.02)
        assert r["pedestrian_space"] == pytest.approx(32.0, abs=0.3)
        assert r["segment_score"] == pytest.approx(3.62, abs=0.03)
        assert r["segment_los"] == "D"

    def test_ep3_bicycle_los(self):
        cfg = {
            "length_ft": 1320.0, "num_through_lanes": 2.0, "midseg_flow_rate": 940.0,
            "pct_heavy_vehicles": 8.0, "prop_parking_occupied": 0.20,
            "width_outside_lane_ft": 12.0, "width_bike_lane_ft": 5.0,
            "width_parking_lane_ft": 9.5, "curb_present": True,
            "num_access_points_right": 3.0, "pavement_condition": 2.0,
            "motor_running_speed": 33.0, "bicycle_running_speed": 15.0,
            "bicycle_control_delay": 40.0, "bicycle_los_score_intersection": 0.08,
        }
        r = json.loads(tl.analyze_bicycle_segment(json.dumps(cfg)))
        assert r["link_score"] == pytest.approx(3.62, abs=0.02)
        assert r["link_los"] == "D"
        assert r["segment_score"] == pytest.approx(2.88, abs=0.02)
        assert r["segment_los"] == "C"

    def test_ep4_transit_los(self):
        cfg = {
            "length_ft": 1320.0, "num_transit_stops": 1.0, "motor_running_speed": 33.0,
            "dwell_time_s": 20.0, "transit_frequency": 4.0, "g_c_ratio": 0.4729,
            "near_side_signalized_stop": True, "reentry_delay_s": 16.17,
            "through_delay_s": 19.4, "passenger_load_factor": 0.83,
            "prop_stops_bench": 1.0, "passenger_trip_length": 3.7,
            "on_time_performance": 0.92, "base_travel_time_rate": 4.0,
            "ped_los_score_link": 3.53,
        }
        r = json.loads(tl.analyze_transit_segment(json.dumps(cfg)))
        assert r["travel_speed"] == pytest.approx(11.3, abs=0.1)
        assert r["wait_ride_score"] == pytest.approx(2.47, abs=0.02)
        assert r["segment_score"] == pytest.approx(2.83, abs=0.03)
        assert r["segment_los"] == "C"
