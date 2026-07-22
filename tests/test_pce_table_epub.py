"""Guard that src/hcm/common/pce_table.rs still equals what the HCM EPUB says.

The Chapter 12 PCE tables were hand-transcribed once and drifted badly: all three exhibits ended
up holding one copy of Exhibit 12-28, with a grade-2 block from 12-27 and one value matching no
exhibit at all. Nothing caught it, because a wrong E_T only shows up as a slightly-off density.

This test re-derives the Rust module from the EPUB and compares it to the committed file, so any
future hand-edit or partial regeneration fails loudly. It skips when the EPUB is absent, since
resources/ is gitignored (copyrighted source material) and CI has no copy.
"""

import importlib.util
from pathlib import Path

import pytest

ROOT = Path(__file__).resolve().parents[1]
EPUB = ROOT / "resources" / "epub" / "OEBPS" / "83_Ch12_03.xhtml"
GENERATED = ROOT / "src" / "hcm" / "common" / "pce_table.rs"
GENERATOR = ROOT / "scripts" / "gen_pce_table.py"


def load_generator():
    spec = importlib.util.spec_from_file_location("gen_pce_table", GENERATOR)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


pytestmark = pytest.mark.skipif(
    not EPUB.exists(),
    reason=f"HCM EPUB not present at {EPUB} (gitignored source material)",
)


def test_committed_table_matches_epub():
    """The checked-in Rust module is byte-for-byte what the EPUB generates."""
    gen = load_generator()
    rendered, _ = gen.render()
    assert rendered == GENERATED.read_text(), (
        "src/hcm/common/pce_table.rs differs from the EPUB-derived output; "
        "re-run scripts/gen_pce_table.py rather than hand-editing it"
    )


def test_exhibits_are_distinct():
    """The three exhibits carry genuinely different values, the check the original transcription
    would have failed: 30/50/70% SUT mixes are not interchangeable."""
    gen = load_generator()
    src = EPUB.read_text(encoding="utf-8")
    e26, e27, e28 = (gen.parse_exhibit(src, f"Exhibit 12-{n}") for n in (26, 27, 28))

    assert e26 != e27 and e27 != e28 and e26 != e28

    # parse_exhibit returns {grade: [(length_mi, [E_T per truck percentage]), ...]}.
    # The 2.5% grade, 0.625 mi row differs across all three mixes at every truck percentage.
    def row(exhibit):
        return next(vals for length, vals in exhibit[2.5] if length == 0.625)

    r26, r27, r28 = (row(e) for e in (e26, e27, e28))
    for i, pct in enumerate(gen.PCT_COLS):
        assert len({r26[i], r27[i], r28[i]}) == 3, f"{pct}% trucks"


def test_full_grade_range_is_transcribed():
    """All eight printed grades are present, not just the four the old code handled."""
    gen = load_generator()
    src = EPUB.read_text(encoding="utf-8")
    for n in (26, 27, 28):
        grades = sorted(gen.parse_exhibit(src, f"Exhibit 12-{n}"))
        assert grades == [-2.0, 0.0, 2.0, 2.5, 3.5, 4.5, 5.5, 6.0], f"Exhibit 12-{n}"
