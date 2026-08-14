"""Regenerate src/hcm/common/pce_table.rs from the HCM 7th Edition EPUB (Exhibits 12-26/27/28)."""
import re, html

EPUB = 'resources/epub/OEBPS/83_Ch12_03.xhtml'
OUT = 'src/hcm/common/pce_table.rs'
PCT_COLS = [2.0, 4.0, 5.0, 6.0, 8.0, 10.0, 15.0, 20.0, 25.0]


def parse_exhibit(src, caption):
    i = src.find(caption + ':')
    j = src.find('<table', i)
    k = src.find('</table>', j)
    rows = re.findall(r'<tr.*?</tr>', src[j:k + 8], re.S)
    blocks, grade = {}, None
    for r in rows[2:]:
        cells = [html.unescape(re.sub(r'<[^>]+>', '', c)).strip()
                 for c in re.findall(r'<t[dh].*?</t[dh]>', r, re.S)]
        if len(cells) == 11:
            grade, cells = float(cells[0]), cells[1:]
        assert len(cells) == 10, cells
        blocks.setdefault(grade, []).append((float(cells[0]), [float(v) for v in cells[1:]]))
    return blocks


def fmt(x):
    """Rust f64 literal: always carries a decimal point so slices infer as &[f64]."""
    s = f'{x:g}'
    return s if '.' in s else s + '.0'


def emit(name, sut, exhibit, blocks):
    grades = sorted(blocks)
    out = [f'/// Exhibit {exhibit} — PCEs for a mix of {sut}% SUTs '
           f'and {100 - sut}% TTs.\n'
           f'/// Transcribed verbatim from the HCM 7th Edition EPUB by '
           f'scripts/gen_pce_table.py; do not hand-edit.\n'
           f'pub static {name}: PceTable = PceTable {{\n'
           f'    sut_percentage: {sut},\n'
           f'    truck_pcts: &[{", ".join(fmt(p) for p in PCT_COLS)}],\n'
           f'    grades: &[{", ".join(fmt(g) for g in grades)}],\n'
           f'    lengths: &[\n']
    for g in grades:
        out.append(f'        &[{", ".join(fmt(l) for l, _ in blocks[g])}],'
                   f'  // {fmt(g)}% grade\n')
    out.append('    ],\n    values: &[\n')
    for g in grades:
        out.append(f'        // {fmt(g)}% grade\n        &[\n')
        for l, vals in blocks[g]:
            out.append(f'            &[{", ".join(f"{v:.2f}" for v in vals)}],'
                       f'  // {fmt(l)} mi\n')
        out.append('        ],\n')
    out.append('    ],\n};\n')
    return ''.join(out)


def render():
    """The full pce_table.rs source implied by the EPUB, as a string."""
    src = open(EPUB, encoding='utf-8').read()
    tables = [('ET_TABLE_30SUT', 30, '12-26', parse_exhibit(src, 'Exhibit 12-26')),
              ('ET_TABLE_50SUT', 50, '12-27', parse_exhibit(src, 'Exhibit 12-27')),
              ('ET_TABLE_70SUT', 70, '12-28', parse_exhibit(src, 'Exhibit 12-28'))]
    return HEADER + '\n'.join(emit(*t) for t in tables) + LOOKUP, tables

HEADER = '''//! Passenger-car-equivalent (PCE) tables for HCM Chapter 12 specific upgrades.
//!
//! Exhibits 12-26, 12-27, and 12-28 give E_T for a 30/70, 50/50, and 70/30 SUT/TT mix,
//! keyed on grade (%), grade length (mi), and truck percentage. The exhibits carry the note
//! "Interpolation in the exhibit is permitted", so [`PceTable::lookup`] interpolates linearly
//! on all three axes rather than requiring an exact grid hit.
//!
//! The tables below are generated from the HCM 7th Edition EPUB by `scripts/gen_pce_table.py`.
//! Regenerate rather than hand-editing, and keep `pce_table_epub_test.rs` passing.

/// One PCE exhibit: a ragged grid of E_T indexed by [grade][length][truck percentage].
///
/// `lengths` is per-grade because the exhibits stop at 1 mi for grades above 3.5%
/// ("segment lengths for grades above 3.5% are limited to 1 mi, because steeper grades
/// are rarely longer than this in practice").
pub struct PceTable {
    /// SUT share of the heavy-vehicle mix this exhibit describes, percent.
    pub sut_percentage: u32,
    /// Truck percentages heading the exhibit columns; the last entry is the ">25%" column.
    pub truck_pcts: &'static [f64],
    /// Grades heading the exhibit row blocks, ascending.
    pub grades: &'static [f64],
    /// Grade lengths (mi) within each grade block, ascending, parallel to `grades`.
    pub lengths: &'static [&'static [f64]],
    /// E_T values indexed [grade][length][truck percentage].
    pub values: &'static [&'static [&'static [f64]]],
}

'''



LOOKUP = '''

impl PceTable {
    /// The exhibit for a given SUT share of the heavy-vehicle mix (30, 50, or 70).
    pub fn for_sut_percentage(sut_percentage: u32) -> Result<&'static PceTable, String> {
        match sut_percentage {
            30 => Ok(&ET_TABLE_30SUT),
            50 => Ok(&ET_TABLE_50SUT),
            70 => Ok(&ET_TABLE_70SUT),
            other => Err(format!(
                "HCM Chapter 12 tabulates specific-upgrade PCEs only for 30%, 50%, and 70% SUT mixes \\
                 (Exhibits 12-26 through 12-28); got {other}%. Use general terrain (sut_percentage = 0) \\
                 or one of the tabulated mixes."
            )),
        }
    }

    /// E_T for a grade, grade length, and truck percentage, interpolating within the exhibit.
    ///
    /// `grade` is percent (negative for downgrades), `length` is miles, `p_t` is the decimal
    /// proportion of heavy vehicles. Returns an error rather than a plausible-looking default
    /// when the inputs fall outside the exhibit's domain.
    pub fn lookup(&self, grade: f64, length: f64, p_t: f64) -> Result<f64, String> {
        if !grade.is_finite() || !length.is_finite() || !p_t.is_finite() {
            return Err(format!(
                "grade, length, and truck proportion must all be finite, got \
                 grade {grade}, length {length}, p_t {p_t}"
            ));
        }
        let max_grade = *self.grades.last().unwrap();
        if grade > max_grade {
            return Err(format!(
                "grade {grade}% exceeds the {max_grade}% maximum tabulated in HCM Exhibit 12-26/27/28; \\
                 steep single grades require the Chapter 25/26 mixed-flow model \\
                 (basicfreeways::mixed_flow for a single grade, \\
                 basicfreeways::composite_grade for consecutive grades)"
            ));
        }
        if length <= 0.0 {
            return Err(format!("grade length must be positive, got {length} mi"));
        }
        if p_t < 0.0 || p_t > 1.0 {
            return Err(format!("truck proportion must be in [0, 1], got {p_t}"));
        }

        // Downgrades below -2% are not tabulated. The -2% and 0% rows are identical in all three
        // exhibits (PCE shows no downgrade sensitivity), so clamping to the -2% row is the reading
        // that keeps the method usable; VERIFY-HCM.
        let grade = grade.max(self.grades[0]);
        // Below the 2% column the exhibits say nothing, and the lower clamp is symmetric with the
        // upper one: a 1% truck stream reads the 2% column. The ">25%" column is a bucket, not a
        // point, so any mix at or above 25% trucks reads it directly.
        let pct = (p_t * 100.0).min(*self.truck_pcts.last().unwrap());

        let (gi, gf) = bracket(self.grades, grade);
        let low = self.at_grade(gi, length, pct)?;
        if gf == 0.0 {
            return Ok(low);
        }
        let high = self.at_grade(gi + 1, length, pct)?;
        Ok(low + (high - low) * gf)
    }

    /// E_T within a single grade block, interpolating on length then truck percentage.
    fn at_grade(&self, gi: usize, length: f64, pct: f64) -> Result<f64, String> {
        let lengths = self.lengths[gi];
        // Beyond the longest tabulated length the PCE has effectively converged (the 1.25 and
        // 1.5 mi rows differ by at most 0.01), so the last row is carried forward; VERIFY-HCM.
        // Below the shortest, the 0.125 mi row is carried back the same way.
        let length = length.min(*lengths.last().unwrap()).max(lengths[0]);
        let (li, lf) = bracket(lengths, length);

        let row = |i: usize| -> f64 {
            let vals = self.values[gi][i];
            let (pi, pf) = bracket(self.truck_pcts, pct);
            if pf == 0.0 { vals[pi] } else { vals[pi] + (vals[pi + 1] - vals[pi]) * pf }
        };
        let lo = row(li);
        Ok(if lf == 0.0 { lo } else { lo + (row(li + 1) - lo) * lf })
    }
}

/// Index of the tabulated value at or below `x`, plus the fraction toward the next one.
fn bracket(axis: &[f64], x: f64) -> (usize, f64) {
    for i in 0..axis.len() - 1 {
        if x < axis[i + 1] {
            let span = axis[i + 1] - axis[i];
            return (i, if x <= axis[i] { 0.0 } else { (x - axis[i]) / span });
        }
    }
    (axis.len() - 1, 0.0)
}
'''

if __name__ == '__main__':
    rendered, tables = render()
    open(OUT, 'w').write(rendered)
    print(f'wrote {OUT}')
    for name, sut, exhibit, blocks in tables:
        n = sum(len(v) for v in blocks.values())
        print(f'  {name} (Exhibit {exhibit}): {len(blocks)} grades, {n} length rows, {n * 9} values')
