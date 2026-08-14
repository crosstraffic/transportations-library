//! Passenger-car-equivalent (PCE) tables for HCM Chapter 12 specific upgrades.
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

/// Exhibit 12-26 — PCEs for a mix of 30% SUTs and 70% TTs.
/// Transcribed verbatim from the HCM 7th Edition EPUB by scripts/gen_pce_table.py; do not hand-edit.
pub static ET_TABLE_30SUT: PceTable = PceTable {
    sut_percentage: 30,
    truck_pcts: &[2.0, 4.0, 5.0, 6.0, 8.0, 10.0, 15.0, 20.0, 25.0],
    grades: &[-2.0, 0.0, 2.0, 2.5, 3.5, 4.5, 5.5, 6.0],
    lengths: &[
        &[0.125, 0.375, 0.625, 0.875, 1.25, 1.5],  // -2.0% grade
        &[0.125, 0.375, 0.625, 0.875, 1.25, 1.5],  // 0.0% grade
        &[0.125, 0.375, 0.625, 0.875, 1.25, 1.5],  // 2.0% grade
        &[0.125, 0.375, 0.625, 0.875, 1.25, 1.5],  // 2.5% grade
        &[0.125, 0.375, 0.625, 0.875, 1.25, 1.5],  // 3.5% grade
        &[0.125, 0.375, 0.625, 0.875, 1.0],  // 4.5% grade
        &[0.125, 0.375, 0.625, 0.875, 1.0],  // 5.5% grade
        &[0.125, 0.375, 0.625, 0.875, 1.0],  // 6.0% grade
    ],
    values: &[
        // -2.0% grade
        &[
            &[2.62, 2.37, 2.30, 2.24, 2.17, 2.12, 2.04, 1.99, 1.97],  // 0.125 mi
            &[2.62, 2.37, 2.30, 2.24, 2.17, 2.12, 2.04, 1.99, 1.97],  // 0.375 mi
            &[2.62, 2.37, 2.30, 2.24, 2.17, 2.12, 2.04, 1.99, 1.97],  // 0.625 mi
            &[2.62, 2.37, 2.30, 2.24, 2.17, 2.12, 2.04, 1.99, 1.97],  // 0.875 mi
            &[2.62, 2.37, 2.30, 2.24, 2.17, 2.12, 2.04, 1.99, 1.97],  // 1.25 mi
            &[2.62, 2.37, 2.30, 2.24, 2.17, 2.12, 2.04, 1.99, 1.97],  // 1.5 mi
        ],
        // 0.0% grade
        &[
            &[2.62, 2.37, 2.30, 2.24, 2.17, 2.12, 2.04, 1.99, 1.97],  // 0.125 mi
            &[2.62, 2.37, 2.30, 2.24, 2.17, 2.12, 2.04, 1.99, 1.97],  // 0.375 mi
            &[2.62, 2.37, 2.30, 2.24, 2.17, 2.12, 2.04, 1.99, 1.97],  // 0.625 mi
            &[2.62, 2.37, 2.30, 2.24, 2.17, 2.12, 2.04, 1.99, 1.97],  // 0.875 mi
            &[2.62, 2.37, 2.30, 2.24, 2.17, 2.12, 2.04, 1.99, 1.97],  // 1.25 mi
            &[2.62, 2.37, 2.30, 2.24, 2.17, 2.12, 2.04, 1.99, 1.97],  // 1.5 mi
        ],
        // 2.0% grade
        &[
            &[2.62, 2.37, 2.30, 2.24, 2.17, 2.12, 2.04, 1.99, 1.97],  // 0.125 mi
            &[3.76, 2.96, 2.78, 2.65, 2.48, 2.38, 2.22, 2.14, 2.09],  // 0.375 mi
            &[4.47, 3.33, 3.08, 2.91, 2.68, 2.54, 2.34, 2.23, 2.17],  // 0.625 mi
            &[4.80, 3.50, 3.22, 3.03, 2.77, 2.61, 2.39, 2.28, 2.21],  // 0.875 mi
            &[5.00, 3.60, 3.30, 3.09, 2.83, 2.66, 2.42, 2.30, 2.23],  // 1.25 mi
            &[5.04, 3.62, 3.32, 3.11, 2.84, 2.67, 2.43, 2.31, 2.23],  // 1.5 mi
        ],
        // 2.5% grade
        &[
            &[2.62, 2.37, 2.30, 2.24, 2.17, 2.12, 2.04, 1.99, 1.97],  // 0.125 mi
            &[4.11, 3.14, 2.93, 2.78, 2.58, 2.46, 2.28, 2.19, 2.13],  // 0.375 mi
            &[5.04, 3.62, 3.32, 3.11, 2.84, 2.67, 2.43, 2.31, 2.23],  // 0.625 mi
            &[5.48, 3.85, 3.51, 3.27, 2.96, 2.77, 2.50, 2.36, 2.28],  // 0.875 mi
            &[5.73, 3.98, 3.61, 3.36, 3.03, 2.83, 2.54, 2.40, 2.31],  // 1.25 mi
            &[5.80, 4.02, 3.64, 3.38, 3.05, 2.84, 2.55, 2.41, 2.32],  // 1.5 mi
        ],
        // 3.5% grade
        &[
            &[2.62, 2.37, 2.30, 2.24, 2.17, 2.12, 2.04, 1.99, 1.97],  // 0.125 mi
            &[4.88, 3.54, 3.25, 3.05, 2.80, 2.63, 2.41, 2.29, 2.22],  // 0.375 mi
            &[6.34, 4.30, 3.87, 3.58, 3.20, 2.97, 2.64, 2.48, 2.38],  // 0.625 mi
            &[7.03, 4.66, 4.16, 3.83, 3.39, 3.12, 2.76, 2.57, 2.46],  // 0.875 mi
            &[7.44, 4.87, 4.33, 3.97, 3.50, 3.22, 2.82, 2.62, 2.50],  // 1.25 mi
            &[7.53, 4.92, 4.38, 4.01, 3.53, 3.24, 2.84, 2.63, 2.51],  // 1.5 mi
        ],
        // 4.5% grade
        &[
            &[2.62, 2.37, 2.30, 2.24, 2.17, 2.12, 2.04, 1.99, 1.97],  // 0.125 mi
            &[5.80, 4.02, 3.64, 3.38, 3.05, 2.84, 2.55, 2.41, 2.32],  // 0.375 mi
            &[7.90, 5.11, 4.53, 4.14, 3.63, 3.32, 2.90, 2.68, 2.55],  // 0.625 mi
            &[8.91, 5.64, 4.96, 4.50, 3.92, 3.56, 3.07, 2.82, 2.67],  // 0.875 mi
            &[9.19, 5.78, 5.08, 4.60, 3.99, 3.62, 3.11, 2.85, 2.70],  // 1.0 mi
        ],
        // 5.5% grade
        &[
            &[2.62, 2.37, 2.30, 2.24, 2.17, 2.12, 2.04, 1.99, 1.97],  // 0.125 mi
            &[6.87, 4.58, 4.10, 3.77, 3.35, 3.09, 2.73, 2.55, 2.44],  // 0.375 mi
            &[9.78, 6.09, 5.33, 4.82, 4.16, 3.76, 3.21, 2.93, 2.77],  // 0.625 mi
            &[11.20, 6.83, 5.94, 5.33, 4.56, 4.09, 3.45, 3.12, 2.93],  // 0.875 mi
            &[11.60, 7.04, 6.11, 5.47, 4.67, 4.18, 3.51, 3.17, 2.97],  // 1.0 mi
        ],
        // 6.0% grade
        &[
            &[2.62, 2.37, 2.30, 2.24, 2.17, 2.12, 2.04, 1.99, 1.97],  // 0.125 mi
            &[7.48, 4.90, 4.36, 3.99, 3.52, 3.23, 2.83, 2.63, 2.51],  // 0.375 mi
            &[10.87, 6.66, 5.79, 5.21, 4.46, 4.01, 3.39, 3.08, 2.89],  // 0.625 mi
            &[12.54, 7.54, 6.51, 5.81, 4.94, 4.40, 3.67, 3.30, 3.08],  // 0.875 mi
            &[13.02, 7.78, 6.71, 5.99, 5.07, 4.51, 3.75, 3.37, 3.14],  // 1.0 mi
        ],
    ],
};

/// Exhibit 12-27 — PCEs for a mix of 50% SUTs and 50% TTs.
/// Transcribed verbatim from the HCM 7th Edition EPUB by scripts/gen_pce_table.py; do not hand-edit.
pub static ET_TABLE_50SUT: PceTable = PceTable {
    sut_percentage: 50,
    truck_pcts: &[2.0, 4.0, 5.0, 6.0, 8.0, 10.0, 15.0, 20.0, 25.0],
    grades: &[-2.0, 0.0, 2.0, 2.5, 3.5, 4.5, 5.5, 6.0],
    lengths: &[
        &[0.125, 0.375, 0.625, 0.875, 1.25, 1.5],  // -2.0% grade
        &[0.125, 0.375, 0.625, 0.875, 1.25, 1.5],  // 0.0% grade
        &[0.125, 0.375, 0.625, 0.875, 1.25, 1.5],  // 2.0% grade
        &[0.125, 0.375, 0.625, 0.875, 1.25, 1.5],  // 2.5% grade
        &[0.125, 0.375, 0.625, 0.875, 1.25, 1.5],  // 3.5% grade
        &[0.125, 0.375, 0.625, 0.875, 1.0],  // 4.5% grade
        &[0.125, 0.375, 0.625, 0.875, 1.0],  // 5.5% grade
        &[0.125, 0.375, 0.625, 0.875, 1.0],  // 6.0% grade
    ],
    values: &[
        // -2.0% grade
        &[
            &[2.67, 2.38, 2.31, 2.25, 2.16, 2.11, 2.02, 1.97, 1.93],  // 0.125 mi
            &[2.67, 2.38, 2.31, 2.25, 2.16, 2.11, 2.02, 1.97, 1.93],  // 0.375 mi
            &[2.67, 2.38, 2.31, 2.25, 2.16, 2.11, 2.02, 1.97, 1.93],  // 0.625 mi
            &[2.67, 2.38, 2.31, 2.25, 2.16, 2.11, 2.02, 1.97, 1.93],  // 0.875 mi
            &[2.67, 2.38, 2.31, 2.25, 2.16, 2.11, 2.02, 1.97, 1.93],  // 1.25 mi
            &[2.67, 2.38, 2.31, 2.25, 2.16, 2.11, 2.02, 1.97, 1.93],  // 1.5 mi
        ],
        // 0.0% grade
        &[
            &[2.67, 2.38, 2.31, 2.25, 2.16, 2.11, 2.02, 1.97, 1.93],  // 0.125 mi
            &[2.67, 2.38, 2.31, 2.25, 2.16, 2.11, 2.02, 1.97, 1.93],  // 0.375 mi
            &[2.67, 2.38, 2.31, 2.25, 2.16, 2.11, 2.02, 1.97, 1.93],  // 0.625 mi
            &[2.67, 2.38, 2.31, 2.25, 2.16, 2.11, 2.02, 1.97, 1.93],  // 0.875 mi
            &[2.67, 2.38, 2.31, 2.25, 2.16, 2.11, 2.02, 1.97, 1.93],  // 1.25 mi
            &[2.67, 2.38, 2.31, 2.25, 2.16, 2.11, 2.02, 1.97, 1.93],  // 1.5 mi
        ],
        // 2.0% grade
        &[
            &[2.67, 2.38, 2.31, 2.25, 2.16, 2.11, 2.02, 1.97, 1.93],  // 0.125 mi
            &[3.76, 2.95, 2.77, 2.64, 2.47, 2.36, 2.20, 2.11, 2.06],  // 0.375 mi
            &[4.32, 3.24, 3.01, 2.84, 2.63, 2.49, 2.29, 2.19, 2.12],  // 0.625 mi
            &[4.57, 3.37, 3.11, 2.93, 2.70, 2.55, 2.33, 2.22, 2.15],  // 0.875 mi
            &[4.71, 3.45, 3.17, 2.99, 2.74, 2.58, 2.36, 2.24, 2.17],  // 1.25 mi
            &[4.74, 3.47, 3.19, 3.00, 2.75, 2.59, 2.36, 2.24, 2.17],  // 1.5 mi
        ],
        // 2.5% grade
        &[
            &[2.67, 2.38, 2.31, 2.25, 2.16, 2.11, 2.02, 1.97, 1.93],  // 0.125 mi
            &[4.10, 3.13, 2.92, 2.77, 2.57, 2.44, 2.26, 2.16, 2.10],  // 0.375 mi
            &[4.84, 3.52, 3.23, 3.03, 2.77, 2.61, 2.38, 2.26, 2.18],  // 0.625 mi
            &[5.17, 3.69, 3.37, 3.15, 2.87, 2.69, 2.43, 2.30, 2.22],  // 0.875 mi
            &[5.36, 3.79, 3.45, 3.22, 2.92, 2.73, 2.47, 2.33, 2.24],  // 1.25 mi
            &[5.40, 3.81, 3.47, 3.24, 2.93, 2.74, 2.47, 2.33, 2.25],  // 1.5 mi
        ],
        // 3.5% grade
        &[
            &[2.67, 2.38, 2.31, 2.25, 2.16, 2.11, 2.02, 1.97, 1.93],  // 0.125 mi
            &[4.89, 3.54, 3.25, 3.05, 2.79, 2.62, 2.39, 2.26, 2.19],  // 0.375 mi
            &[6.05, 4.15, 3.75, 3.47, 3.11, 2.89, 2.58, 2.42, 2.32],  // 0.625 mi
            &[6.58, 4.43, 3.97, 3.66, 3.26, 3.01, 2.67, 2.49, 2.39],  // 0.875 mi
            &[6.88, 4.58, 4.10, 3.77, 3.35, 3.09, 2.72, 2.53, 2.42],  // 1.25 mi
            &[6.95, 4.62, 4.13, 3.80, 3.37, 3.10, 2.73, 2.54, 2.43],  // 1.5 mi
        ],
        // 4.5% grade
        &[
            &[2.67, 2.38, 2.31, 2.25, 2.16, 2.11, 2.02, 1.97, 1.93],  // 0.125 mi
            &[5.83, 4.03, 3.65, 3.39, 3.05, 2.84, 2.55, 2.39, 2.30],  // 0.375 mi
            &[7.53, 4.92, 4.38, 4.01, 3.53, 3.24, 2.83, 2.62, 2.50],  // 0.625 mi
            &[8.32, 5.34, 4.72, 4.29, 3.75, 3.42, 2.97, 2.73, 2.59],  // 0.875 mi
            &[8.53, 5.45, 4.81, 4.37, 3.81, 3.47, 3.00, 2.76, 2.62],  // 1.0 mi
        ],
        // 5.5% grade
        &[
            &[2.67, 2.38, 2.31, 2.25, 2.16, 2.11, 2.02, 1.97, 1.93],  // 0.125 mi
            &[6.97, 4.63, 4.14, 3.81, 3.38, 3.11, 2.74, 2.55, 2.43],  // 0.375 mi
            &[9.37, 5.89, 5.16, 4.68, 4.05, 3.67, 3.14, 2.88, 2.72],  // 0.625 mi
            &[10.49, 6.48, 5.65, 5.09, 4.37, 3.93, 3.34, 3.03, 2.85],  // 0.875 mi
            &[10.80, 6.64, 5.78, 5.20, 4.46, 4.01, 3.39, 3.08, 2.89],  // 1.0 mi
        ],
        // 6.0% grade
        &[
            &[2.67, 2.38, 2.31, 2.25, 2.16, 2.11, 2.02, 1.97, 1.93],  // 0.125 mi
            &[7.64, 4.98, 4.43, 4.05, 3.56, 3.26, 2.85, 2.64, 2.51],  // 0.375 mi
            &[10.45, 6.45, 5.63, 5.07, 4.36, 3.92, 3.33, 3.03, 2.85],  // 0.625 mi
            &[11.78, 7.16, 6.20, 5.56, 4.74, 4.24, 3.56, 3.22, 3.01],  // 0.875 mi
            &[12.15, 7.35, 6.36, 5.69, 4.85, 4.33, 3.62, 3.27, 3.05],  // 1.0 mi
        ],
    ],
};

/// Exhibit 12-28 — PCEs for a mix of 70% SUTs and 30% TTs.
/// Transcribed verbatim from the HCM 7th Edition EPUB by scripts/gen_pce_table.py; do not hand-edit.
pub static ET_TABLE_70SUT: PceTable = PceTable {
    sut_percentage: 70,
    truck_pcts: &[2.0, 4.0, 5.0, 6.0, 8.0, 10.0, 15.0, 20.0, 25.0],
    grades: &[-2.0, 0.0, 2.0, 2.5, 3.5, 4.5, 5.5, 6.0],
    lengths: &[
        &[0.125, 0.375, 0.625, 0.875, 1.25, 1.5],  // -2.0% grade
        &[0.125, 0.375, 0.625, 0.875, 1.25, 1.5],  // 0.0% grade
        &[0.125, 0.375, 0.625, 0.875, 1.25, 1.5],  // 2.0% grade
        &[0.125, 0.375, 0.625, 0.875, 1.25, 1.5],  // 2.5% grade
        &[0.125, 0.375, 0.625, 0.875, 1.25, 1.5],  // 3.5% grade
        &[0.125, 0.375, 0.625, 0.875, 1.0],  // 4.5% grade
        &[0.125, 0.375, 0.625, 0.875, 1.0],  // 5.5% grade
        &[0.125, 0.375, 0.625, 0.875, 1.0],  // 6.0% grade
    ],
    values: &[
        // -2.0% grade
        &[
            &[2.39, 2.18, 2.12, 2.07, 2.01, 1.96, 1.89, 1.85, 1.83],  // 0.125 mi
            &[2.39, 2.18, 2.12, 2.07, 2.01, 1.96, 1.89, 1.85, 1.83],  // 0.375 mi
            &[2.39, 2.18, 2.12, 2.07, 2.01, 1.96, 1.89, 1.85, 1.83],  // 0.625 mi
            &[2.39, 2.18, 2.12, 2.07, 2.01, 1.96, 1.89, 1.85, 1.83],  // 0.875 mi
            &[2.39, 2.18, 2.12, 2.07, 2.01, 1.96, 1.89, 1.85, 1.83],  // 1.25 mi
            &[2.39, 2.18, 2.12, 2.07, 2.01, 1.96, 1.89, 1.85, 1.83],  // 1.5 mi
        ],
        // 0.0% grade
        &[
            &[2.39, 2.18, 2.12, 2.07, 2.01, 1.96, 1.89, 1.85, 1.83],  // 0.125 mi
            &[2.39, 2.18, 2.12, 2.07, 2.01, 1.96, 1.89, 1.85, 1.83],  // 0.375 mi
            &[2.39, 2.18, 2.12, 2.07, 2.01, 1.96, 1.89, 1.85, 1.83],  // 0.625 mi
            &[2.39, 2.18, 2.12, 2.07, 2.01, 1.96, 1.89, 1.85, 1.83],  // 0.875 mi
            &[2.39, 2.18, 2.12, 2.07, 2.01, 1.96, 1.89, 1.85, 1.83],  // 1.25 mi
            &[2.39, 2.18, 2.12, 2.07, 2.01, 1.96, 1.89, 1.85, 1.83],  // 1.5 mi
        ],
        // 2.0% grade
        &[
            &[2.67, 2.32, 2.23, 2.17, 2.08, 2.03, 1.94, 1.89, 1.86],  // 0.125 mi
            &[3.63, 2.82, 2.64, 2.52, 2.35, 2.25, 2.10, 2.02, 1.97],  // 0.375 mi
            &[4.12, 3.08, 2.85, 2.69, 2.49, 2.36, 2.18, 2.08, 2.02],  // 0.625 mi
            &[4.37, 3.21, 2.96, 2.78, 2.56, 2.42, 2.22, 2.11, 2.05],  // 0.875 mi
            &[4.53, 3.29, 3.02, 2.84, 2.60, 2.45, 2.24, 2.13, 2.07],  // 1.25 mi
            &[4.58, 3.31, 3.04, 2.86, 2.61, 2.46, 2.25, 2.14, 2.07],  // 1.5 mi
        ],
        // 2.5% grade
        &[
            &[2.75, 2.36, 2.27, 2.20, 2.11, 2.04, 1.95, 1.90, 1.87],  // 0.125 mi
            &[4.01, 3.02, 2.80, 2.65, 2.46, 2.33, 2.16, 2.06, 2.01],  // 0.375 mi
            &[4.66, 3.35, 3.08, 2.88, 2.64, 2.48, 2.26, 2.15, 2.08],  // 0.625 mi
            &[4.99, 3.52, 3.21, 3.00, 2.73, 2.56, 2.32, 2.19, 2.12],  // 0.875 mi
            &[5.20, 3.64, 3.30, 3.08, 2.79, 2.60, 2.35, 2.22, 2.14],  // 1.25 mi
            &[5.26, 3.67, 3.33, 3.10, 2.80, 2.62, 2.36, 2.23, 2.15],  // 1.5 mi
        ],
        // 3.5% grade
        &[
            &[2.93, 2.45, 2.34, 2.26, 2.16, 2.09, 1.98, 1.92, 1.89],  // 0.125 mi
            &[4.86, 3.46, 3.16, 2.96, 2.69, 2.53, 2.30, 2.18, 2.10],  // 0.375 mi
            &[5.88, 3.99, 3.59, 3.32, 2.98, 2.76, 2.46, 2.31, 2.22],  // 0.625 mi
            &[6.40, 4.26, 3.81, 3.51, 3.12, 2.88, 2.55, 2.38, 2.28],  // 0.875 mi
            &[6.74, 4.43, 3.96, 3.63, 3.21, 2.96, 2.60, 2.42, 2.32],  // 1.25 mi
            &[6.83, 4.48, 3.99, 3.66, 3.24, 2.98, 2.62, 2.44, 2.33],  // 1.5 mi
        ],
        // 4.5% grade
        &[
            &[3.13, 2.56, 2.43, 2.34, 2.21, 2.13, 2.01, 1.95, 1.91],  // 0.125 mi
            &[5.88, 3.99, 3.59, 3.32, 2.98, 2.76, 2.46, 2.31, 2.22],  // 0.375 mi
            &[7.35, 4.75, 4.22, 3.85, 3.39, 3.10, 2.71, 2.51, 2.39],  // 0.625 mi
            &[8.11, 5.15, 4.54, 4.13, 3.60, 3.27, 2.83, 2.61, 2.47],  // 0.875 mi
            &[8.33, 5.27, 4.63, 4.21, 3.66, 3.33, 2.87, 2.64, 2.50],  // 1.0 mi
        ],
        // 5.5% grade
        &[
            &[3.37, 2.69, 2.53, 2.42, 2.28, 2.19, 2.05, 1.98, 1.94],  // 0.125 mi
            &[7.09, 4.62, 4.11, 3.76, 3.31, 3.04, 2.66, 2.47, 2.36],  // 0.375 mi
            &[9.13, 5.68, 4.97, 4.49, 3.88, 3.51, 3.00, 2.74, 2.59],  // 0.625 mi
            &[10.21, 6.24, 5.43, 4.88, 4.18, 3.76, 3.18, 2.89, 2.71],  // 0.875 mi
            &[10.52, 6.41, 5.57, 5.00, 4.27, 3.83, 3.24, 2.93, 2.75],  // 1.0 mi
        ],
        // 6.0% grade
        &[
            &[3.51, 2.76, 2.59, 2.47, 2.32, 2.22, 2.08, 2.00, 1.95],  // 0.125 mi
            &[7.78, 4.98, 4.40, 4.01, 3.51, 3.20, 2.78, 2.56, 2.44],  // 0.375 mi
            &[10.17, 6.23, 5.42, 4.87, 4.17, 3.75, 3.18, 2.88, 2.71],  // 0.625 mi
            &[11.43, 6.88, 5.95, 5.32, 4.53, 4.04, 3.39, 3.06, 2.86],  // 0.875 mi
            &[11.81, 7.08, 6.11, 5.46, 4.64, 4.13, 3.45, 3.11, 2.90],  // 1.0 mi
        ],
    ],
};


impl PceTable {
    /// The exhibit for a given SUT share of the heavy-vehicle mix (30, 50, or 70).
    pub fn for_sut_percentage(sut_percentage: u32) -> Result<&'static PceTable, String> {
        match sut_percentage {
            30 => Ok(&ET_TABLE_30SUT),
            50 => Ok(&ET_TABLE_50SUT),
            70 => Ok(&ET_TABLE_70SUT),
            other => Err(format!(
                "HCM Chapter 12 tabulates specific-upgrade PCEs only for 30%, 50%, and 70% SUT mixes \
                 (Exhibits 12-26 through 12-28); got {other}%. Use general terrain (sut_percentage = 0) \
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
                "grade, length, and truck proportion must all be finite, got                  grade {grade}, length {length}, p_t {p_t}"
            ));
        }
        let max_grade = *self.grades.last().unwrap();
        if grade > max_grade {
            return Err(format!(
                "grade {grade}% exceeds the {max_grade}% maximum tabulated in HCM Exhibit 12-26/27/28; \
                 steep single grades require the Chapter 25/26 mixed-flow model \
                 (basicfreeways::mixed_flow for a single grade, \
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
