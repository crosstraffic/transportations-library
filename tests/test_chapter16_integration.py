"""Python-binding integration test for HCM Chapter 16 (Urban Street
Facilities).

Runs the Chapter 29, Section 5, Example Problem 1 fixtures through the
PyO3 bindings: eastbound published segment measures aggregated per
Equations 16-2 through 16-4 (base FFS exact at 40.1 mi/h, LOS C exact;
travel speed / stop rate within the documented bands because segments 2-4
are not individually published), plus the full Chapter 18-driven pipeline
(case3, three copies of the Chapter 30 EP1 segment; exact reproduction).
"""

import os

import pytest

tl = pytest.importorskip("transportations_library")

FIXTURE_DIR = os.path.join(
    os.path.dirname(__file__), "ExampleCases", "hcm", "UrbanFacilities"
)


def load(name):
    if not hasattr(tl, "UrbanFacility"):
        pytest.skip("transportations_library built without UrbanFacility bindings")
    with open(os.path.join(FIXTURE_DIR, name)) as f:
        return tl.UrbanFacility(f.read())


class TestUrbanFacilityExampleProblem1:
    """HCM Chapter 29, Example Problem 1 (Exhibit 29-49), eastbound."""

    @pytest.fixture
    def facility(self):
        facility = load("case1.json")
        facility.aggregate()
        return facility

    def test_geometry(self, facility):
        assert facility.num_segments == 5
        assert facility.length_ft == pytest.approx(5280.0)

    def test_base_free_flow_speed(self, facility):
        # Exhibit 29-49: 40.1 mi/h (exact; all segment values published).
        assert facility.base_free_flow_speed_mph == pytest.approx(40.1, abs=0.05)

    def test_travel_speed_and_stop_rate(self, facility):
        # Exhibit 29-49: 22.6 mi/h and 1.83 stops/mi (approximate here;
        # segments 2-4 copy segments 1/5 - see the fixture _source note).
        assert facility.travel_speed_mph == pytest.approx(22.6, abs=0.6)
        assert facility.spatial_stop_rate == pytest.approx(1.83, abs=0.15)

    def test_los(self, facility):
        # Exhibit 29-49: facility LOS C, poorest segment LOS D (exact).
        assert facility.los == "C"
        assert facility.poorest_segment_los == "D"
        assert facility.critical_vc_ratio <= 1.0


class TestUrbanFacilityChapter18Pipeline:
    """Case 3: full Chapter 18-driven analyze() (Chapter 30 EP1 values)."""

    @pytest.fixture
    def facility(self):
        facility = load("case3.json")
        facility.analyze()
        return facility

    def test_published_values(self, facility):
        assert facility.base_free_flow_speed_mph == pytest.approx(40.78, abs=0.02)
        assert facility.travel_speed_mph == pytest.approx(23.67, abs=0.02)
        assert facility.spatial_stop_rate == pytest.approx(1.61, abs=0.02)
        assert facility.critical_vc_ratio == pytest.approx(0.52, abs=0.005)
        assert facility.los == "C"

    def test_length_weighted_identity(self, facility):
        # Facility speed equals the length-weighted harmonic mean of the
        # Chapter 18 segment speeds (Equation 16-3).
        speeds = facility.segment_travel_speeds()
        assert all(s is not None for s in speeds)
        length = facility.length_ft / len(speeds)
        harmonic = facility.length_ft / sum(length / s for s in speeds)
        assert facility.travel_speed_mph == pytest.approx(harmonic, abs=1e-9)

    def test_json_round_trip(self, facility):
        restored = tl.UrbanFacility(facility.to_json())
        assert restored.num_segments == facility.num_segments
