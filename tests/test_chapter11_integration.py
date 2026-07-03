"""Python-binding integration test for HCM Chapter 11 (freeway reliability).

Mirrors tests/ExampleCases/hcm/FreewayReliability/case1.json (HCM Chapter 25,
Example Problem 7: reliability evaluation of an existing freeway facility;
Exhibits 25-97 through 25-105) through the PyO3 FreewayReliability class.
The published Exhibit 25-104 values come from FREEVAL's Monte Carlo stream,
so central measures are checked within the documented tolerance bands and
the scenario-generation intermediates are checked exactly (see
tests/chapter11_integration.rs for the full computed-vs-published notes).
"""

import json
from pathlib import Path

import pytest

tl = pytest.importorskip("transportations_library")

CASE1 = Path(__file__).parent / "ExampleCases" / "hcm" / "FreewayReliability" / "case1.json"


@pytest.fixture(scope="module")
def analysis():
    rel = tl.FreewayReliability(CASE1.read_text())
    rel.run()
    return rel


class TestFreewayReliabilityExampleProblem7:
    """HCM Chapter 25, Example Problem 7 (Exhibits 25-97 through 25-105)."""

    def test_structure(self, analysis):
        # 12 months x 5 weekdays x 4 replications = 240 scenarios; 12
        # analysis periods each.
        assert analysis.num_scenarios == 240
        assert analysis.num_observations == 240 * 12
        assert analysis.free_flow_travel_time_min == pytest.approx(6.0, abs=0.01)
        assert analysis.expected_vhd > 0.0

    def test_scenario_probabilities(self, analysis):
        probs = analysis.scenario_probabilities()
        assert len(probs) == 240
        assert sum(probs) == pytest.approx(1.0, abs=1e-9)
        assert all(p == pytest.approx(1 / 240, abs=1e-12) for p in probs)

    def test_scenario_set_intermediates(self, analysis):
        st = json.loads(analysis.scenario_set_json())
        # Equation 25-76 weather event counts: 1 medium rain per month,
        # 2 heavy rain in summer months (6-8), 1 otherwise, nothing else.
        for month in range(1, 13):
            row = st["expected_weather_events"][month - 1]
            assert row[0] == 1
            assert row[1] == (2 if month in (6, 7, 8) else 1)
            assert all(c == 0 for c in row[2:])
        assert st["total_weather_events"] == 27
        # Exhibit 25-103 monthly incident frequencies (see the Rust
        # integration test for the documented October book inconsistency).
        published = [0.65, 0.67, 0.72, 0.77, 0.77, 0.80,
                     0.89, 0.82, 0.83, 0.83, 0.79, 0.77]
        for m, p in enumerate(published):
            tol = 0.045 if m == 9 else 0.012
            assert st["monthly_incident_frequency"][m] == pytest.approx(p, abs=tol)

    def test_reliability_metrics(self, analysis):
        m = json.loads(analysis.metrics())
        # VMT-weighted central measures (published Exhibit 25-104 values in
        # comments; FREEVAL Monte Carlo, distribution-level comparison).
        assert m["tti_50"] == pytest.approx(1.04, abs=0.01)  # published 1.03
        assert m["tti_mean"] == pytest.approx(1.24, abs=0.05)  # published 1.30
        assert m["reliability_rating"] == pytest.approx(84.2, abs=1.5)  # 90.8
        # Shape invariants.
        assert m["tti_95"] >= m["tti_80"] >= m["tti_50"] >= 1.0
        assert m["tti_max"] >= m["tti_95"]
        assert m["misery_index"] >= m["tti_mean"]

    def test_distribution_accessors(self, analysis):
        assert analysis.tti_mean() >= 1.0
        assert analysis.tti_percentile(95.0) >= analysis.tti_percentile(50.0)
        assert 0.0 <= analysis.reliability_rating() <= 100.0
        assert analysis.misery_index() >= analysis.tti_mean()
        assert analysis.semi_std_dev() > 0.0
        f45 = analysis.failure_pct_below_speed(45.0)
        f50 = analysis.failure_pct_below_speed(50.0)
        assert 0.0 <= f45 <= f50 <= 100.0
        tti = analysis.scenario_tti()
        assert len(tti) == 240
        assert all(len(row) == 12 for row in tti)
        assert all(t >= 1.0 for row in tti for t in row)

    def test_repr(self, analysis):
        assert "FreewayReliability" in repr(analysis)


class TestPlanningLevelReliability:
    """Chapter 25, Example Problem 10 (planning-level; Equations 11-1
    through 11-5): FFS 75 mi/h, peak speed 62 mi/h, 3 lanes, X = 0.95."""

    def test_example_problem_10(self):
        tti_mean, tti_95, pt45 = tl.planning_reliability(75.0, 62.0, 3, 0.95)
        assert tti_mean == pytest.approx(1.899, abs=0.001)
        assert tti_95 == pytest.approx(3.353, abs=0.005)
        assert pt45 == pytest.approx(0.743, abs=0.001)
