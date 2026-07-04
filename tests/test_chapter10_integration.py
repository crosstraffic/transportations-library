"""Python-binding integration test for HCM Chapter 10 (freeway facilities).

Mirrors tests/ExampleCases/hcm/FreewayFacilities/case1.json (HCM Chapter 25,
Example Problem 1: undersaturated urban freeway facility, 11 segments, five
15-min analysis periods) through the PyO3 FreewayFacility class. Tolerances
match the Rust integration test: speeds +-0.5 mi/h, densities
+-0.5 veh/mi/ln, exact LOS letters.
"""

from pathlib import Path

import pytest

tl = pytest.importorskip("transportations_library")

CASE1 = Path(__file__).parent / "ExampleCases" / "hcm" / "FreewayFacilities" / "case1.json"


@pytest.fixture(scope="module")
def facility():
    fac = tl.FreewayFacility(CASE1.read_text())
    fac.run_analysis()
    return fac


class TestFreewayFacilitiesExampleProblem1:
    """HCM Chapter 25, Example Problem 1 (Exhibits 25-43 through 25-52)."""

    def test_structure(self, facility):
        assert facility.num_segments == 11
        assert facility.num_periods == 5
        assert facility.total_length_mi == pytest.approx(6.0, abs=0.01)
        assert not facility.oversaturated

    def test_demands_and_capacities(self, facility):
        demand = facility.demand()
        # Exhibit 25-48, Analysis Period 1 (undersaturated: served = demand)
        expected_p1 = [4505, 4955, 4955, 4955, 4685, 5225, 4865, 5315, 5315, 5315, 5045]
        for i, e in enumerate(expected_p1):
            assert demand[i][0] == pytest.approx(e, abs=0.5)
        # Exhibit 25-46: basic/ramp segment capacities of 6,748 veh/h
        capacity = facility.capacity()
        assert capacity[0][0] == pytest.approx(6748, abs=5)
        # Weaving segment capacity varies by period
        assert capacity[5][0] == pytest.approx(8273, abs=25)

    def test_speed_matrix_period1(self, facility):
        # Exhibit 25-49, Analysis Period 1
        expected = [60.0, 53.9, 59.7, 56.1, 60.0, 48.0, 59.9, 53.4, 53.4, 56.0, 59.7]
        speed = facility.speed()
        for i, e in enumerate(expected):
            assert speed[i][0] == pytest.approx(e, abs=0.5)

    def test_density_matrix_period1(self, facility):
        # Exhibit 25-50, Analysis Period 1 (veh/mi/ln)
        expected = [25.0, 30.6, 27.6, 29.4, 26.0, 27.2, 27.1, 33.2, 33.2, 31.6, 28.1]
        density = facility.density_veh()
        for i, e in enumerate(expected):
            assert density[i][0] == pytest.approx(e, abs=0.5)

    def test_los_matrix_period1(self, facility):
        # Exhibit 25-51, Analysis Period 1
        expected = ["C", "C", "D", "C", "D", "C", "D", "D", "D", "D", "D"]
        los = facility.los()
        for i, e in enumerate(expected):
            assert los[i][0] == e

    def test_facility_performance(self, facility):
        # Exhibit 25-52: facility performance measure summary
        expected = [
            (57.6, 27.5, "D"),
            (56.6, 31.3, "D"),
            (55.0, 34.8, "E"),
            (57.9, 27.5, "D"),
            (58.4, 21.4, "C"),
        ]
        for p, (speed, density, los) in enumerate(expected):
            assert facility.facility_speed(p) == pytest.approx(speed, abs=0.5)
            assert facility.facility_density_veh(p) == pytest.approx(density, abs=0.5)
            assert facility.facility_los(p) == los
        # Totals: 56.9 mi/h and 28.4 veh/mi/ln
        assert facility.overall_speed() == pytest.approx(56.9, abs=0.5)
        assert facility.overall_density_veh() == pytest.approx(28.4, abs=0.5)

    def test_roundtrip_json(self, facility):
        text = facility.to_json()
        clone = tl.FreewayFacility(text)
        assert clone.num_segments == 11
        assert repr(facility).startswith("FreewayFacility(")
