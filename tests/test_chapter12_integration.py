"""Python-binding integration test for HCM Chapter 12 (basic freeway and multilane segments).

Mirrors tests/ExampleCases/hcm/BasicFreeways/case{1,2,3}.json (HCM Chapter 26 Example
Problems 1-3) through the PyO3 BasicFreeways class, plus the Exhibit 12-26/12-27/12-28
specific-upgrade PCE path, which no fixture exercises. Tolerances match the Rust integration
test: FFS and density to 0.1, capacity and flow rates to the published integer, exact LOS letters.
"""

import json
from pathlib import Path

import pytest

tl = pytest.importorskip("transportations_library")

CASES = Path(__file__).parent / "ExampleCases" / "hcm" / "BasicFreeways"

# HCM Chapter 26 Example Problems 1-3, published values. EP2 is a design problem: its published
# operational results describe the 3-lane solution, so the Exhibit 12-37 lane-count step has to
# run before demand is converted to a per-lane flow rate. `design_los` marks that case.
EXPECTED = {
    "case1.json": dict(ffs=60.8, capacity=2308, v_p=1142, density=18.8, speed=60.8, los="C"),
    "case2.json": dict(ffs=67.3, capacity=2373, v_p=1694, density=25.9, speed=65.4, los="C",
                       design_los="D", lanes=3),
    "case3.json": dict(ffs=70.0, capacity=2400, v_p=1875, density=29.0, speed=64.7, los="D"),
}


def analyze(name):
    """FFS -> capacity -> (design lane count) -> demand -> speed -> density, per the fixture."""
    want = EXPECTED[name]
    seg = build(name)
    seg.determine_free_flow_speed()
    seg.estimate_capacity()
    if "design_los" in want:
        seg.set_target_los(want["design_los"])
        lanes, _ = seg.estimate_number_of_lanes()
        assert lanes == want["lanes"]
    seg.estimate_demand_volume()
    seg.calculate_speed()
    seg.estimate_density()
    return seg


def build(name):
    data = json.loads((CASES / name).read_text())
    return tl.BasicFreeways(
        bffs=data["bffs"],
        lane_width=data["lw"],
        lane_count=data["lane_count"],
        lc_r=data["lc_r"],
        lc_l=data["lc_l"],
        trd=data["trd"],
        apd=data["apd"],
        grade=data["grade"],
        terrain_type=data["terrain_type"],
        speed_limit=data["speed_limit"],
        phf=data["phf"],
        p_t=data["p_t"],
        demand_flow_i=data["demand_flow_i"],
        length=data["length"],
        highway_type=data["highway_type"],
        sut_percentage=data["sut_percentage"],
    )


@pytest.mark.parametrize("name", sorted(EXPECTED))
def test_operational_analysis(name):
    """Steps 2-6 against the published example-problem values."""
    want = EXPECTED[name]
    seg = analyze(name)

    assert seg.ffs() == pytest.approx(want["ffs"], abs=0.05)
    assert round(seg.capacity()) == want["capacity"]
    assert round(seg.density() * seg.speed()) == pytest.approx(want["v_p"], abs=2)
    assert seg.density() == pytest.approx(want["density"], abs=0.05)
    assert seg.speed() == pytest.approx(want["speed"], abs=0.1)
    assert seg.determine_segment_los() == want["los"]


@pytest.mark.parametrize("name", sorted(n for n in EXPECTED if "design_los" not in EXPECTED[n]))
def test_step_methods_match_orchestrator(name):
    """For the operational cases, run_operational_analysis() equals calling the steps by hand."""
    chained = build(name)
    los = chained.run_operational_analysis()
    stepwise = analyze(name)

    assert stepwise.density() == pytest.approx(chained.density())
    assert stepwise.speed() == pytest.approx(chained.speed())
    assert stepwise.determine_segment_los() == los


def test_general_terrain_pce_is_case_insensitive():
    """Exhibit 12-25: level 2.0, rolling 3.0, whatever case the caller writes them in."""
    for terrain, want in [("level", 2.0), ("Level", 2.0), ("ROLLING", 3.0), ("rolling", 3.0)]:
        seg = tl.BasicFreeways(
            bffs=75.4, terrain_type=terrain, p_t=0.10, phf=0.95,
            demand_flow_i=3000.0, lane_count=3, length=1.0,
        )
        seg.determine_free_flow_speed()
        seg.estimate_demand_volume()
        assert seg.e_t() == want, f"terrain {terrain!r}"
        assert seg.f_hv() == pytest.approx(1.0 / (1.0 + 0.10 * (want - 1.0)))


def test_specific_upgrade_pce_table():
    """Exhibit 12-27 (50% SUT), 2.5% grade, 0.625 mi, 6% trucks: E_T = 3.03."""
    seg = tl.BasicFreeways(
        bffs=75.4, grade=2.5, length=0.625, p_t=0.06, sut_percentage=50,
        phf=0.95, demand_flow_i=3000.0, lane_count=3,
    )
    seg.determine_free_flow_speed()
    seg.estimate_demand_volume()
    assert seg.e_t() == pytest.approx(3.03)


def test_specific_upgrade_pce_interpolates():
    """The exhibits permit interpolation, so an untabulated length lands between its neighbours."""
    def e_t(length):
        seg = tl.BasicFreeways(
            bffs=75.4, grade=2.5, length=length, p_t=0.06, sut_percentage=50,
            phf=0.95, demand_flow_i=3000.0, lane_count=3,
        )
        seg.determine_free_flow_speed()
        seg.estimate_demand_volume()
        return seg.e_t()

    # Exhibit 12-27, 2.5% grade, 6% trucks: 0.375 mi -> 2.77, 0.625 mi -> 3.03.
    assert e_t(0.5) == pytest.approx((2.77 + 3.03) / 2)
    assert e_t(0.375) < e_t(0.5) < e_t(0.625)


def test_off_domain_inputs_error_rather_than_defaulting():
    """A grade past the exhibit, an untabulated SUT mix, and an unknown terrain all raise."""
    steep = tl.BasicFreeways(
        bffs=75.4, grade=8.0, length=0.5, p_t=0.06, sut_percentage=50,
        phf=0.95, demand_flow_i=3000.0, lane_count=3,
    )
    with pytest.raises(ValueError, match="mixed-flow model"):
        steep.estimate_demand_volume()

    odd_mix = tl.BasicFreeways(
        bffs=75.4, grade=2.5, length=0.5, p_t=0.06, sut_percentage=40,
        phf=0.95, demand_flow_i=3000.0, lane_count=3,
    )
    with pytest.raises(ValueError, match="30%, 50%, and 70%"):
        odd_mix.estimate_demand_volume()

    unknown_terrain = tl.BasicFreeways(
        bffs=75.4, terrain_type="hilly", p_t=0.06, phf=0.95,
        demand_flow_i=3000.0, lane_count=3, length=1.0,
    )
    with pytest.raises(ValueError, match="unknown terrain type"):
        unknown_terrain.estimate_demand_volume()


def test_design_analysis_lane_count():
    """Case 2 (HCM Ch. 26 EP2): 4,000 veh/h at LOS D needs 3 lanes.

    FFS 67.3 mi/h rounds to the nearest tabulated row at 65, not up to 70, per Exhibit 12-37.
    """
    seg = build("case2.json")
    seg.determine_free_flow_speed()
    seg.set_target_los("D")

    lanes, unrounded = seg.estimate_number_of_lanes()
    assert lanes == 3
    assert 2 < unrounded <= 3


def test_design_analysis_rejects_untabulated_los():
    """LOS F has no maximum service flow rate; Exhibit 12-37 stops at E."""
    seg = build("case2.json")
    seg.determine_free_flow_speed()
    seg.set_target_los("F")

    with pytest.raises(ValueError, match="LOS F"):
        seg.estimate_number_of_lanes()

    with pytest.raises(ValueError, match="A-F"):
        seg.set_target_los("Z")
