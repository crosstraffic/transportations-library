"""Python-binding integration test for HCM Chapter 13 (freeway weaving).

Mirrors tests/ExampleCases/hcm/Weaving/case1.json (HCM Chapter 27,
Example Problem 1: LOS of a major weaving segment) through the PyO3
WeavingSegment class. Tolerances match the Rust integration test:
flows/capacities within a few units (published values are rounded),
speeds +-0.5 mi/h, densities +-0.5 pc/mi/ln, exact LOS letters.
"""

import json
from pathlib import Path

import pytest

tl = pytest.importorskip("transportations_library")

CASES = Path(__file__).parent / "ExampleCases" / "hcm" / "Weaving"
CASE1 = CASES / "case1.json"

WEAVING_TYPE_MAP = {"OneSided": "one_sided", "TwoSided": "two_sided"}
FACILITY_TYPE_MAP = {"Freeway": "freeway", "MultilaneOrCD": "multilane"}


def load_segment(path):
    data = json.loads(path.read_text())
    return tl.WeavingSegment(
        weaving_type=WEAVING_TYPE_MAP[data["weaving_type"]],
        facility_type=FACILITY_TYPE_MAP[data["facility_type"]],
        length_short=data["length_short"],
        num_lanes=data["num_lanes"],
        num_weaving_lanes=data["num_weaving_lanes"],
        ffs=data["ffs"],
        v_ff=data["v_ff"],
        v_fr=data["v_fr"],
        v_rf=data["v_rf"],
        v_rr=data["v_rr"],
        phf=data["phf"],
        heavy_vehicle_pct=data["heavy_vehicle_pct"],
        terrain=data["terrain"].lower(),
        lc_rf=data["lc_rf"],
        lc_fr=data["lc_fr"],
        lc_rr=data["lc_rr"],
        interchange_density=data["interchange_density"],
        basic_freeway_capacity=data["basic_freeway_capacity"],
        caf=data["caf"],
        saf=data["saf"],
    )


class TestWeavingExampleProblem1:
    """HCM Chapter 27, Example Problem 1 (major weaving segment)."""

    def test_step_methods(self):
        seg = load_segment(CASE1)

        v_w, v_nw, v = seg.determine_demand_flow()
        assert v_w == pytest.approx(1995.0, abs=5.0)
        assert v_nw == pytest.approx(3591.0, abs=5.0)
        assert v == pytest.approx(5586.0, abs=5.0)
        assert seg.volume_ratio == pytest.approx(0.357, abs=0.002)

        lc_min = seg.determine_configuration_characteristics()
        assert lc_min == pytest.approx(798.0, abs=5.0)

        l_max = seg.determine_max_weaving_length()
        assert l_max == pytest.approx(4639.0, abs=5.0)
        assert seg.is_weaving

        capacity = seg.determine_capacity()
        assert capacity == pytest.approx(8038.0, abs=10.0)

        lc_all = seg.determine_lane_changing_rates()
        assert lc_all == pytest.approx(1926.0, abs=8.0)

        s_w, s_nw, s = seg.estimate_speed()
        assert s_w == pytest.approx(54.2, abs=0.5)
        assert s_nw == pytest.approx(52.5, abs=0.5)
        assert s == pytest.approx(53.1, abs=0.5)

        density = seg.determine_density()
        assert density == pytest.approx(26.3, abs=0.5)

        assert seg.determine_los() == "C"
        assert seg.los == "C"

    def test_run_analysis(self):
        seg = load_segment(CASE1)
        assert seg.run_analysis() == "C"
        assert seg.density == pytest.approx(26.3, abs=0.5)
        assert seg.speed_avg == pytest.approx(53.1, abs=0.5)
        assert seg.capacity == pytest.approx(8038.0, abs=10.0)

    def test_repr(self):
        seg = load_segment(CASE1)
        assert "WeavingSegment" in repr(seg)


class TestWeavingExampleProblems4to7:
    """HCM Chapter 27, Example Problems 4-7 through the PyO3 bindings."""

    def test_ep4_trial2_design_los_c(self):
        # Trial 2 adds an exit lane (N_WL = 3) to reach the target LOS C.
        seg = load_segment(CASES / "case4b.json")
        assert seg.run_analysis() == "C"
        assert seg.capacity == pytest.approx(8255.0, abs=15.0)
        assert seg.density == pytest.approx(24.2, abs=0.5)

    def test_ep4_trial1_design_los_f(self):
        # Trial 1 (N_WL = 2) fails: weaving-flow capacity below demand.
        seg = load_segment(CASES / "case4a.json")
        assert seg.run_analysis() == "F"
        assert seg.vc_ratio > 1.0

    def test_ep6_ml_access_los_c(self):
        seg = load_segment(CASES / "case6.json")
        assert seg.run_analysis() == "C"
        assert seg.density == pytest.approx(21.7, abs=0.5)

    def test_ep7_ml_access_los_b(self):
        # Multilane/C-D LOS thresholds (B <= 24) per the published solution.
        seg = load_segment(CASES / "case7.json")
        assert seg.run_analysis() == "B"
        assert seg.density == pytest.approx(23.6, abs=0.5)


class TestCrossWeaveAndServiceVolumes:
    """Managed-lane cross-weave (Eq. 13-24/25) and EP 5 service volumes."""

    def test_cross_weave_ep6(self):
        crf, caf, c_gpa = tl.cross_weave_gp_capacity(400.0, 1000.0, 3, 7050.0)
        assert crf == pytest.approx(0.056, abs=0.001)
        assert caf == pytest.approx(1.0 - crf, abs=1e-12)

    def test_cross_weave_ep7(self):
        crf, caf, c_gpa = tl.cross_weave_gp_capacity(100.0, 1500.0, 2, 4800.0)
        assert crf == pytest.approx(0.0105, abs=0.0005)
        assert c_gpa == pytest.approx(4750.0, abs=5.0)

    def test_cross_weave_requires_positive_demand(self):
        with pytest.raises(ValueError):
            tl.cross_weave_gp_capacity(0.0, 1000.0, 2, 4800.0)

    def test_service_flow_rate_and_volumes(self):
        # EP 5 cell: N = 3, N_WL = 2, L_S = 1,500, LOS C (D = 28) -> ~4,300 pc/h.
        seg = tl.WeavingSegment(
            weaving_type="one_sided",
            facility_type="freeway",
            length_short=1500.0,
            num_lanes=3,
            num_weaving_lanes=2,
            ffs=65.0,
            interchange_density=1.0,
            lc_rf=0,
            lc_fr=2,
            lc_rr=0,
            basic_freeway_capacity=2350.0,
        )
        sfi = tl.service_flow_rate_ideal(seg, (0.65, 0.15, 0.12, 0.08), 28.0)
        assert (sfi // 100) * 100 == pytest.approx(4300.0, abs=100.0)

        sfi_out, sf, sv, dsv = tl.service_volumes(4300.0, 0.952, 0.93, 0.08, 0.55)
        assert sf == pytest.approx(4300.0 * 0.952, abs=0.01)
        assert sv == pytest.approx(sf * 0.93, abs=0.01)
        assert dsv == pytest.approx(sv / (0.08 * 0.55), abs=0.01)
