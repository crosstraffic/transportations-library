"""Python-binding integration test for the HCM Chapter 15 Section 4 bicycle mode.

Mirrors the Rust integration test `bicycle_los_widening_example_test`
(tests/twolanehighways_test.rs) through both PyO3 entry points, the
`analyze_bicycle_los` JSON function and the `BicycleLOS` class, against the same fixture
tests/ExampleCases/hcm/TwoLaneHighways/bicycle_widening.json. Every expected value and
tolerance below is the Rust test's, not a new one: the published worked example is a segment
evaluated for widening, realigning and repaving, with BLOS 5.90 (LOS F) before and 3.58
(LOS D) after.

The fixture is a two-design comparison, so its top level is `current` / `proposed` / `expected`
rather than a single config. The `expected` block is what this file asserts against, which the
Rust test does not do -- it hardcodes the same numbers -- so a drift between the fixture and the
assertions cannot hide here.
"""

import json
from pathlib import Path

import pytest

tl = pytest.importorskip("transportations_library")

FIXTURE = (
    Path(__file__).parent / "ExampleCases" / "hcm" / "TwoLaneHighways" / "bicycle_widening.json"
)


@pytest.fixture(scope="module")
def fixture():
    return json.loads(FIXTURE.read_text())


@pytest.fixture(scope="module")
def current(fixture):
    return json.loads(tl.analyze_bicycle_los(json.dumps(fixture["current"])))


@pytest.fixture(scope="module")
def proposed(fixture):
    return json.loads(tl.analyze_bicycle_los(json.dumps(fixture["proposed"])))


# ── The widening example through the JSON entry point ────────────────────────


def test_step_2_flow_rate_is_the_same_for_both_designs(current, proposed, fixture):
    """Only the cross section and the speed limit change, so vOL = 500 / (0.90 * 1) either way."""
    expected = fixture["expected"]["flow_rate_outside_lane"]
    assert current["flow_rate_outside_lane"] == pytest.approx(expected, abs=0.1)
    assert proposed["flow_rate_outside_lane"] == pytest.approx(expected, abs=0.1)


def test_step_3_effective_width(current, proposed, fixture):
    """The 2 ft shoulder takes Equation 15-43 and the 6 ft shoulder Equation 15-42, which is why widening moves We by 10 ft rather than by the 4 ft of shoulder added."""
    assert current["effective_width"] == pytest.approx(
        fixture["expected"]["current_effective_width"], abs=0.01
    )
    assert proposed["effective_width"] == pytest.approx(
        fixture["expected"]["proposed_effective_width"], abs=0.01
    )


def test_step_4_effective_speed_factor(current, proposed, fixture):
    """Equation 15-46 on the posted limit, 50 mi/h current and 55 mi/h proposed."""
    assert current["effective_speed_factor"] == pytest.approx(
        fixture["expected"]["current_effective_speed_factor"], abs=0.01
    )
    assert proposed["effective_speed_factor"] == pytest.approx(
        fixture["expected"]["proposed_effective_speed_factor"], abs=0.01
    )


def test_step_5_blos_score_and_los(current, proposed, fixture):
    """The published answer. Tolerance is +-0.01 because the book rounds its intermediates to two decimals; the LOS letters are exact."""
    expected = fixture["expected"]
    assert current["blos_score"] == pytest.approx(expected["current_blos_score"], abs=0.01)
    assert proposed["blos_score"] == pytest.approx(expected["proposed_blos_score"], abs=0.01)
    assert current["los"] == expected["current_los"]
    assert proposed["los"] == expected["proposed_los"]


def test_the_project_moves_two_los_letters(current, proposed):
    """The point of the example: a lower score is a better LOS, and this project buys F to D."""
    assert proposed["blos_score"] < current["blos_score"]
    assert (current["los"], proposed["los"]) == ("F", "D")


# ── The same case through the class ──────────────────────────────────────────


def test_the_class_reproduces_the_json_function(fixture, current):
    """The class constructor takes the nine fields positionally in the engine's own order."""
    d = fixture["current"]
    blos = tl.BicycleLOS(
        d["lane_width"],
        d["shoulder_width"],
        d["speed_limit"],
        d["num_lanes"],
        d["pavement_condition"],
        d["hourly_volume"],
        d["phf"],
        d["heavy_vehicle_pct"],
        d["pct_on_highway_parking"],
    )
    assert json.loads(blos.analyze()) == current


def test_the_class_exposes_each_step_separately(fixture, current):
    d = fixture["proposed"]
    blos = tl.BicycleLOS(
        d["lane_width"],
        d["shoulder_width"],
        d["speed_limit"],
        d["num_lanes"],
        d["pavement_condition"],
        d["hourly_volume"],
        d["phf"],
        d["heavy_vehicle_pct"],
        d["pct_on_highway_parking"],
    )
    assert blos.calculate_flow_rate_outside_lane() == pytest.approx(555.6, abs=0.1)
    assert blos.calculate_effective_width() == pytest.approx(24.0, abs=0.01)
    assert blos.calculate_effective_speed_factor() == pytest.approx(4.79, abs=0.01)
    assert blos.calculate_blos_score() == pytest.approx(3.58, abs=0.01)
    assert blos.determine_bicycle_los() == "D"


def test_the_class_round_trips_its_inputs(fixture):
    d = fixture["current"]
    blos = tl.BicycleLOS(
        d["lane_width"],
        d["shoulder_width"],
        d["speed_limit"],
        d["num_lanes"],
        d["pavement_condition"],
        d["hourly_volume"],
        d["phf"],
        d["heavy_vehicle_pct"],
        d["pct_on_highway_parking"],
    )
    assert blos.lane_width == d["lane_width"]
    assert blos.shoulder_width == d["shoulder_width"]
    assert blos.speed_limit == d["speed_limit"]
    assert blos.num_lanes == d["num_lanes"]
    assert blos.pavement_condition == d["pavement_condition"]
    assert blos.hourly_volume == d["hourly_volume"]
    assert blos.phf == d["phf"]
    assert blos.heavy_vehicle_pct == d["heavy_vehicle_pct"]
    assert blos.pct_on_highway_parking == d["pct_on_highway_parking"]


# ── The inputs that fail silently ────────────────────────────────────────────
# heavy_vehicle_pct and pct_on_highway_parking are decimals, not percents, and
# an argument order copied wrongly from the motorized method puts a speed limit
# where a lane width belongs. None of those raise, so the guard is that the
# score moves the way the equations say it must.


def test_heavy_vehicle_share_is_a_decimal_not_a_percent(fixture):
    """5.0 for "5 percent" is the classic unit slip. It does not raise, so pin that it lands somewhere the LOS letters cannot reach and stays distinguishable from 0.05."""
    as_decimal = json.loads(tl.analyze_bicycle_los(json.dumps(fixture["current"])))
    as_percent = json.loads(
        tl.analyze_bicycle_los(json.dumps(dict(fixture["current"], heavy_vehicle_pct=5.0)))
    )
    assert as_decimal["blos_score"] == pytest.approx(5.90, abs=0.01)
    assert as_percent["blos_score"] > 100.0


def test_a_speed_limit_at_or_below_20_returns_a_null_score_with_an_los_letter(fixture):
    """Known defect, pinned rather than fixed here.

    Equation 15-46 takes ln(Spl - 20), so a posted limit of exactly 20 mi/h gives -inf and
    anything below gives NaN. Neither is guarded. serde_json writes a non-finite float as
    `null`, so the score disappears, but `determine_bicycle_los` compares the raw f64 and still
    returns a letter: -inf sorts under the 1.5 threshold and reports LOS A, the best possible
    grade, while NaN fails every comparison and falls through to LOS F. A caller reading only
    the letter sees no sign that anything went wrong.

    Guarding this is a change to the Chapter 15 engine, not to these bindings, and it would move
    what the web calculator and the MCP server return, so it is Rei's call. The behaviour is
    pinned here so that a fix has to come past this test rather than silently.
    """
    at_20 = json.loads(tl.analyze_bicycle_los(json.dumps(dict(fixture["current"], speed_limit=20.0))))
    assert at_20["effective_speed_factor"] is None
    assert at_20["blos_score"] is None
    assert at_20["los"] == "A"

    below_20 = json.loads(tl.analyze_bicycle_los(json.dumps(dict(fixture["current"], speed_limit=15.0))))
    assert below_20["blos_score"] is None
    assert below_20["los"] == "F"

    # Just above the singularity the chain is finite again, which is what makes the two rows
    # above a domain edge rather than a broken equation.
    above = json.loads(tl.analyze_bicycle_los(json.dumps(dict(fixture["current"], speed_limit=20.5))))
    assert above["blos_score"] == pytest.approx(3.785, abs=0.001)


# ── Error surface ────────────────────────────────────────────────────────────


def test_malformed_config_raises():
    with pytest.raises(ValueError, match="invalid bicycle LOS config"):
        tl.analyze_bicycle_los("{not json")


def test_a_missing_field_raises_rather_than_defaulting(fixture):
    """BicycleLOS carries no serde defaults, so every one of the nine fields is required. A pavement rating that quietly defaulted would move the score by more than a whole LOS letter."""
    incomplete = dict(fixture["current"])
    del incomplete["pavement_condition"]
    with pytest.raises(ValueError, match="invalid bicycle LOS config"):
        tl.analyze_bicycle_los(json.dumps(incomplete))


def test_the_expected_block_is_not_an_input(fixture):
    """The fixture's own `expected` and `description` keys ride along harmlessly: BicycleLOS ignores unknown fields, so a caller can hand back a whole example case."""
    whole = dict(fixture["current"], description=fixture["description"], expected=fixture["expected"])
    assert json.loads(tl.analyze_bicycle_los(json.dumps(whole)))["blos_score"] == pytest.approx(
        5.90, abs=0.01
    )
