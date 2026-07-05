"""Python-binding integration test for HCM Chapter 17 (Urban Street
Reliability and ATDM).

Runs the Chapter 29, Section 5, Example Problem 4 fixture (3-mi Lincoln,
Nebraska principal arterial; weekdays for one year; 7-10 a.m.) through
the PyO3 bindings. Deterministic quantities (the published 3,120
scenario count, seeded reproducibility) are asserted exactly; the Monte
Carlo reliability measures at the distribution-band level around the
published Exhibit 29-73 values (mean TTI 1.69/1.64, PTI 2.98/2.61,
reliability rating 93.2/94.1) - the HCM notes seeded streams are
software-specific.
"""

import json
import os

import pytest

tl = pytest.importorskip("transportations_library")

FIXTURE = os.path.join(
    os.path.dirname(__file__), "ExampleCases", "hcm", "UrbanReliability", "case1.json"
)


def load():
    if not hasattr(tl, "UrbanReliability"):
        pytest.skip("transportations_library built without UrbanReliability bindings")
    with open(FIXTURE) as f:
        return tl.UrbanReliability(f.read())


@pytest.fixture(scope="module")
def analysis():
    a = load()
    a.run()
    return a


class TestUrbanReliabilityExampleProblem4:
    """HCM Chapter 29, Example Problem 4 (Exhibit 29-73)."""

    def test_scenario_count(self, analysis):
        # Published: 3,120 scenarios (4/h x 3 h x 5 days x 52 weeks).
        assert analysis.num_scenarios == 3120

    def test_base_free_flow_travel_time(self, analysis):
        # Published: 262.9 s for the 3-mi facility.
        assert analysis.base_free_flow_travel_time_s == pytest.approx(262.9, abs=10.0)

    def test_distribution_measures(self, analysis):
        # Bands around the published EB/WB values (Monte Carlo streams
        # are implementation-specific).
        tti_mean = analysis.tti_mean()
        assert 1.1 <= tti_mean <= 2.6  # published 1.69/1.64
        pti = analysis.tti_percentile(95.0)
        assert 1.3 <= pti <= 5.0  # published 2.98/2.61
        assert analysis.tti_percentile(80.0) <= pti
        assert 70.0 <= analysis.reliability_rating() <= 100.0  # published 93.2/94.1
        assert analysis.total_vhd > 0.0
        assert analysis.num_incidents > 50
        assert analysis.num_weather_events > 50

    def test_results_json(self, analysis):
        results = json.loads(analysis.results())
        assert results["num_scenarios"] == 3120
        assert results["reliability_rating_urban"] == pytest.approx(
            analysis.reliability_rating(), abs=1e-9
        )
        assert len(analysis.scenario_tti()) == 3120
        assert len(analysis.scenario_travel_times()) == 3120

    def test_seeded_reproducibility(self, analysis):
        again = load()
        again.run()
        assert again.num_incidents == analysis.num_incidents
        assert again.tti_mean() == pytest.approx(analysis.tti_mean(), abs=1e-12)
        assert again.tti_percentile(95.0) == pytest.approx(
            analysis.tti_percentile(95.0), abs=1e-12
        )
