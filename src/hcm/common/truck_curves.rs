//! Digitised truck-performance curves for the HCM Chapter 25/26 mixed-flow model.
//!
//! HCM 7 publishes these as figures only. Neither chapter gives a closed form: Chapter 25
//! Step 4 states only that "the travel time rates presented here are based on a model that
//! assumes constant peak-engine power", and points at NCFRP Report 31, which is not part of
//! the manual. So the curves have to be digitised from the printed exhibits, and every
//! number below was measured off the exhibit raster rather than transcribed from text.
//!
//! Method: the exhibit raster is extracted from the chapter PDF, the axes are calibrated
//! from the printed gridlines, and each series is followed across the plot by matching its
//! colour and requiring continuity in y (dash gaps are bridged by extrapolating the local
//! slope). Stations are every 250 ft from 0 to 10,000 ft. See `scripts/digitize_truck_curves.py`.
//!
//! Accuracy: the manual reads these same graphs by eye, and its worked examples state 20
//! such reads. All 20 reproduce from the tables below within the manual's own stated
//! fidelity of about 1 s/mi -- see the calibration tests in this module. Three of the 20 are
//! stated as abscissae rather than ordinates, and on the flat part of a curve a 1 s/mi
//! reading difference moves the abscissa by several hundred feet, so those three sit outside
//! +-100 ft while remaining inside +-1 s/mi. That is a property of the source figures, not of
//! the digitisation.
//!
//! Stage 1 scope: only the grades and initial speeds the two published worked examples
//! consume, plus the level baseline. Anything else returns an error naming the exhibit that
//! would have to be digitised, on the `pce_table` precedent -- these curves must never be
//! extrapolated, because the crawl speed a truck settles at is entirely grade-specific.

/// Stations at which every curve below is tabulated (ft).
pub const STATIONS_FT: [f64; 41] = [
        0.00,   250.00,   500.00,   750.00,  1000.00,  1250.00,
     1500.00,  1750.00,  2000.00,  2250.00,  2500.00,  2750.00,
     3000.00,  3250.00,  3500.00,  3750.00,  4000.00,  4250.00,
     4500.00,  4750.00,  5000.00,  5250.00,  5500.00,  5750.00,
     6000.00,  6250.00,  6500.00,  6750.00,  7000.00,  7250.00,
     7500.00,  7750.00,  8000.00,  8250.00,  8500.00,  8750.00,
     9000.00,  9250.00,  9500.00,  9750.00, 10000.00,
];

/// Grades tabulated in Stage 1 (percent). -5% is drawn coincident with 0% in the source
/// exhibits ("0%, -5%" share one plotted line), so it maps onto the 0% column.
pub const STAGE1_GRADES: [i32; 4] = [0, 2, 3, 5];

// ============================================================================
// Family A -- spot travel time rate versus distance
// Exhibit 25-20 (SUTs) and Exhibit 25-21 (TTs), chap25.pdf pages 77 and 78.
// DECEL is the solid branch (truck enters at 75 mi/h = 48 s/mi and slows).
// ACCEL is the dashed branch (SUT enters at 30 mi/h = 120 s/mi, TT at 20 mi/h = 180 s/mi).
// Both branches of a grade approach the same crawl rate; for TTs on shallow grades they
// have not yet met at 10,000 ft, which is visible in the source figure and preserved here.
// ============================================================================

/// Exhibit 25-20, SUT 0% grade, decelerating branch (s/mi).
/// Level: the truck holds its 75 mi/h entry speed, drawn as a flat line at 48 s/mi.
static SPOT_SUT_DECEL_G0: [f64; 41] = [
       48.00,    48.00,    48.00,    48.00,    48.00,    48.00,
       48.00,    48.00,    48.00,    48.00,    48.00,    48.00,
       48.00,    48.00,    48.00,    48.00,    48.00,    48.00,
       48.00,    48.00,    48.00,    48.00,    48.00,    48.00,
       48.00,    48.00,    48.00,    48.00,    48.00,    48.00,
       48.00,    48.00,    48.00,    48.00,    48.00,    48.00,
       48.00,    48.00,    48.00,    48.00,    48.00,
];

/// Exhibit 25-20, SUT 2% grade, decelerating branch (s/mi).
static SPOT_SUT_DECEL_G2: [f64; 41] = [
       48.00,    48.10,    48.39,    48.69,    48.90,    49.13,
       49.31,    49.50,    49.71,    49.87,    50.04,    50.20,
       50.35,    50.51,    50.63,    50.75,    50.83,    50.94,
       51.10,    51.21,    51.29,    51.36,    51.44,    51.60,
       51.62,    51.68,    51.75,    51.82,    51.91,    51.91,
       51.98,    52.06,    52.06,    52.07,    52.18,    52.22,
       52.25,    52.30,    52.30,    52.30,    52.37,
];

/// Exhibit 25-20, SUT 3% grade, decelerating branch (s/mi).
static SPOT_SUT_DECEL_G3: [f64; 41] = [
       48.00,    48.42,    49.20,    49.75,    50.31,    50.87,
       51.35,    51.89,    52.35,    52.84,    53.25,    53.67,
       53.96,    54.44,    54.78,    55.16,    55.46,    55.74,
       56.00,    56.28,    56.58,    56.86,    57.03,    57.23,
       57.42,    57.61,    57.72,    57.90,    58.04,    58.13,
       58.29,    58.39,    58.46,    58.61,    58.70,    58.82,
       58.87,    58.92,    59.06,    59.07,    59.10,
];

/// Exhibit 25-20, SUT 5% grade, decelerating branch (s/mi).
static SPOT_SUT_DECEL_G5: [f64; 41] = [
       48.00,    49.15,    50.47,    51.75,    53.05,    54.31,
       55.65,    56.93,    58.29,    59.85,    60.74,    61.97,
       63.14,    64.29,    65.36,    66.42,    67.39,    68.30,
       69.19,    69.99,    70.74,    71.40,    72.02,    72.60,
       73.11,    73.56,    73.96,    74.35,    74.66,    74.98,
       75.22,    75.46,    75.68,    75.83,    76.01,    76.11,
       76.19,    76.34,    76.42,    76.57,    76.58,
];

/// Exhibit 25-20, SUT 0% grade, accelerating branch (s/mi).
static SPOT_SUT_ACCEL_G0: [f64; 41] = [
       87.15,    87.15,    81.90,    74.89,    69.29,    66.16,
       63.56,    61.16,    59.01,    57.37,    55.87,    54.78,
       53.91,    52.87,    52.12,    51.17,    50.51,    50.12,
       49.51,    49.03,    48.38,    48.08,    48.02,    48.02,
       48.02,    48.02,    48.02,    48.02,    48.02,    48.02,
       48.02,    48.02,    48.02,    48.02,    48.02,    48.02,
       48.02,    48.02,    48.02,    48.02,    48.02,
];

/// Exhibit 25-20, SUT 2% grade, accelerating branch (s/mi).
static SPOT_SUT_ACCEL_G2: [f64; 41] = [
       94.09,    94.09,    87.52,    81.18,    76.35,    73.33,
       70.79,    68.45,    66.71,    65.03,    63.69,    62.74,
       61.85,    61.05,    60.30,    59.62,    59.10,    58.68,
       58.16,    57.67,    57.24,    57.02,    56.59,    56.36,
       56.04,    55.83,    55.61,    55.43,    55.22,    55.04,
       54.93,    54.75,    54.60,    54.45,    54.40,    54.24,
       54.19,    54.09,    53.98,    53.87,    53.85,
];

/// Exhibit 25-20, SUT 3% grade, accelerating branch (s/mi).
static SPOT_SUT_ACCEL_G3: [f64; 41] = [
      100.60,    99.72,    91.01,    84.94,    80.22,    77.36,
       75.00,    72.90,    71.19,    69.87,    68.82,    68.07,
       67.13,    66.27,    65.47,    65.04,    64.55,    64.19,
       63.68,    63.34,    62.86,    62.91,    62.46,    62.22,
       62.02,    61.86,    61.67,    61.50,    61.36,    61.25,
       61.14,    60.98,    60.88,    60.83,    60.74,    60.63,
       60.57,    60.56,    60.49,    60.44,    60.34,
];

/// Exhibit 25-20, SUT 5% grade, accelerating branch (s/mi).
static SPOT_SUT_ACCEL_G5: [f64; 41] = [
      115.49,   107.18,    98.81,    93.83,    90.77,    88.28,
       86.32,    84.60,    83.23,    82.39,    81.52,    81.01,
       80.38,    79.83,    79.48,    79.14,    78.90,    78.60,
       78.40,    78.21,    78.09,    77.96,    77.84,    77.74,
       77.65,    77.59,    77.50,    77.43,    77.43,    77.31,
       77.28,    77.28,    77.28,    77.28,    77.24,    77.16,
       77.12,    77.12,    77.12,    77.12,    77.12,
];

/// Exhibit 25-21, TT 0% grade, decelerating branch (s/mi).
/// Level: the truck holds its 75 mi/h entry speed, drawn as a flat line at 48 s/mi.
static SPOT_TT_DECEL_G0: [f64; 41] = [
       48.00,    48.00,    48.00,    48.00,    48.00,    48.00,
       48.00,    48.00,    48.00,    48.00,    48.00,    48.00,
       48.00,    48.00,    48.00,    48.00,    48.00,    48.00,
       48.00,    48.00,    48.00,    48.00,    48.00,    48.00,
       48.00,    48.00,    48.00,    48.00,    48.00,    48.00,
       48.00,    48.00,    48.00,    48.00,    48.00,    48.00,
       48.00,    48.00,    48.00,    48.00,    48.00,
];

/// Exhibit 25-21, TT 2% grade, decelerating branch (s/mi).
static SPOT_TT_DECEL_G2: [f64; 41] = [
       48.00,    48.01,    48.42,    49.05,    49.57,    49.97,
       50.46,    50.92,    51.37,    51.77,    52.14,    52.56,
       53.00,    53.37,    53.78,    54.10,    54.50,    54.87,
       55.20,    55.42,    55.90,    56.22,    56.46,    56.78,
       57.01,    57.33,    57.53,    57.79,    58.13,    58.28,
       58.46,    58.69,    58.93,    59.07,    59.34,    59.47,
       59.65,    59.79,    60.06,    60.17,    60.31,
];

/// Exhibit 25-21, TT 3% grade, decelerating branch (s/mi).
static SPOT_TT_DECEL_G3: [f64; 41] = [
       48.00,    48.20,    48.98,    50.08,    50.89,    51.77,
       52.55,    53.44,    54.26,    55.12,    55.96,    56.78,
       57.60,    58.37,    59.25,    60.08,    60.79,    61.52,
       62.31,    63.06,    63.73,    64.48,    65.09,    65.73,
       66.31,    66.74,    67.52,    68.07,    68.57,    69.04,
       69.52,    69.88,    70.32,    70.73,    71.14,    71.45,
       71.76,    72.01,    72.32,    72.51,    72.77,
];

/// Exhibit 25-21, TT 5% grade, decelerating branch (s/mi).
static SPOT_TT_DECEL_G5: [f64; 41] = [
       48.00,    50.51,    50.66,    52.30,    54.06,    55.88,
       57.80,    59.84,    61.90,    64.15,    66.40,    68.80,
       71.25,    73.68,    76.28,    78.83,    81.13,    83.91,
       86.36,    88.70,    90.90,    92.98,    94.84,    96.52,
       98.10,    99.39,   100.58,   101.61,   102.45,   103.13,
      103.79,   104.30,   104.71,   105.04,   105.36,   105.53,
      105.77,   105.94,   106.35,   106.45,   106.45,
];

/// Exhibit 25-21, TT 0% grade, accelerating branch (s/mi).
static SPOT_TT_ACCEL_G0: [f64; 41] = [
      114.52,   114.52,   100.16,    89.10,    82.54,    77.36,
       73.55,    70.56,    68.31,    65.88,    63.91,    62.56,
       61.19,    60.23,    59.06,    57.84,    57.05,    56.29,
       55.60,    54.89,    54.32,    53.76,    53.25,    52.64,
       52.23,    51.87,    51.53,    51.11,    50.67,    50.40,
       50.13,    49.82,    49.51,    49.36,    49.02,    48.84,
       48.34,    48.03,    48.05,    47.96,    47.91,
];

/// Exhibit 25-21, TT 2% grade, accelerating branch (s/mi).
static SPOT_TT_ACCEL_G2: [f64; 41] = [
      121.35,   121.35,   109.73,   100.40,    93.64,    89.02,
       85.46,    82.66,    80.61,    78.78,    76.64,    75.44,
       74.31,    73.32,    72.40,    71.40,    70.82,    70.40,
       69.66,    69.01,    68.60,    68.11,    67.80,    67.30,
       67.13,    66.80,    66.57,    66.13,    65.88,    65.68,
       65.53,    65.31,    65.11,    64.97,    64.87,    64.72,
       64.56,    64.43,    64.42,    64.21,    64.15,
];

/// Exhibit 25-21, TT 3% grade, accelerating branch (s/mi).
static SPOT_TT_ACCEL_G3: [f64; 41] = [
      127.37,   127.37,   116.02,   107.43,   100.90,    96.56,
       93.28,    90.93,    88.84,    86.88,    85.41,    84.41,
       83.44,    82.48,    81.62,    81.07,    80.69,    80.05,
       79.45,    79.12,    78.76,    78.46,    78.08,    78.22,
       77.69,    77.57,    77.32,    77.14,    77.04,    76.84,
       76.77,    76.57,    76.47,    76.40,    76.30,    76.28,
       76.27,    76.16,    76.11,    76.08,    76.02,
];

/// Exhibit 25-21, TT 5% grade, accelerating branch (s/mi).
static SPOT_TT_ACCEL_G5: [f64; 41] = [
      146.18,   145.34,   132.11,   124.54,   119.79,   116.55,
      114.25,   112.78,   111.19,   110.32,   109.52,   109.05,
      108.49,   108.06,   107.82,   107.56,   107.38,   107.25,
      107.12,   107.00,   107.00,   106.94,   106.73,   106.73,
      106.72,   106.73,   106.73,   106.73,   106.73,   106.73,
      106.73,   106.73,   106.73,   106.73,   106.73,   106.73,
      106.73,   106.60,   106.59,   106.58,   106.57,
];

// ============================================================================
// Family B -- cumulative travel time versus distance
// Chapter 25 exhibits are indexed by the truck's INITIAL SPEED; Chapter 26 exhibits are
// indexed by the segment FFS. They are different families and are not interchangeable,
// so both are carried. Where they overlap (65 mi/h) they agree to about 0.2 s, which
// `ch25_and_ch26_65mih_exhibits_agree` asserts.
// ============================================================================

/// Exhibit 25-A6 (chap25.pdf p.230), SUT, 60 mi/h initial speed, 0% grade (s).
static FAMB_SUT_INIT60_G0: [f64; 41] = [
        0.00,     2.72,     5.43,     8.15,    10.87,    13.18,
       15.77,    18.45,    20.69,    23.11,    25.51,    27.94,
       30.27,    32.48,    34.68,    36.98,    39.26,    41.47,
       43.79,    46.09,    48.23,    50.54,    52.73,    55.02,
       57.43,    59.63,    61.92,    64.09,    66.47,    68.82,
       71.00,    73.34,    75.50,    77.88,    80.17,    82.43,
       84.69,    86.97,    89.34,    91.57,    93.60,
];

/// Exhibit 25-A6 (chap25.pdf p.230), SUT, 60 mi/h initial speed, 2% grade (s).
static FAMB_SUT_INIT60_G2: [f64; 41] = [
        0.00,     2.93,     5.86,     8.79,    11.72,    14.29,
       15.51,    18.76,    21.71,    24.48,    27.34,    29.98,
       32.61,    35.13,    37.73,    40.41,    42.96,    45.58,
       48.23,    50.79,    53.15,    55.72,    58.43,    60.94,
       63.49,    65.96,    68.69,    71.16,    73.71,    76.21,
       78.84,    81.38,    83.89,    86.39,    88.99,    91.55,
       94.03,    96.45,    99.21,   101.67,   103.97,
];

/// Exhibit 25-A6 (chap25.pdf p.230), SUT, 60 mi/h initial speed, 3% grade (s).
static FAMB_SUT_INIT60_G3: [f64; 41] = [
        0.00,     2.83,     5.66,     8.49,    11.32,    14.15,
       16.60,    19.75,    22.64,    25.52,    28.54,    31.34,
       34.08,    36.86,    39.49,    42.48,    45.28,    48.26,
       51.06,    53.83,    56.56,    59.48,    62.23,    65.05,
       68.04,    70.81,    73.58,    76.37,    79.40,    82.15,
       84.96,    87.89,    90.70,    93.50,    96.28,    99.30,
      102.07,   104.79,   107.67,   110.64,   113.21,
];

/// Exhibit 25-A6 (chap25.pdf p.230), SUT, 60 mi/h initial speed, 5% grade (s).
static FAMB_SUT_INIT60_G5: [f64; 41] = [
        0.00,     3.03,     6.06,     9.10,    11.60,    14.56,
       18.16,    23.75,    24.46,    26.39,    31.17,    34.40,
       37.79,    41.14,    44.57,    48.13,    51.55,    55.05,
       58.67,    62.16,    65.44,    69.16,    72.66,    76.20,
       79.93,    83.48,    87.02,    90.72,    94.32,    97.99,
      101.60,   105.25,   108.90,   112.52,   116.10,   119.89,
      123.47,   126.98,   130.85,   134.44,   137.74,
];

/// Exhibit 25-A7 (chap25.pdf p.231), SUT, 65 mi/h initial speed, 0% grade (s).
static FAMB_SUT_INIT65_G0: [f64; 41] = [
        0.00,     2.31,     4.62,     6.93,     9.24,    11.55,
       13.85,    16.16,    18.47,    20.78,    23.09,    25.40,
       27.71,    30.02,    32.33,    34.64,    36.95,    39.25,
       41.56,    43.87,    46.18,    48.49,    50.80,    53.11,
       55.42,    57.73,    60.04,    62.35,    64.66,    66.96,
       69.27,    71.58,    73.89,    76.20,    78.51,    80.82,
       83.13,    85.44,    87.75,    90.06,    92.20,
];

/// Exhibit 25-A7 (chap25.pdf p.231), SUT, 65 mi/h initial speed, 2% grade (s).
static FAMB_SUT_INIT65_G2: [f64; 41] = [
        0.00,     2.64,     5.28,     7.92,    10.56,    13.19,
       15.83,    16.86,    19.97,    22.97,    25.52,    28.28,
       30.84,    33.36,    35.99,    38.52,    40.98,    43.60,
       46.11,    48.59,    51.16,    53.72,    56.21,    58.73,
       61.25,    63.77,    66.27,    68.81,    71.32,    73.78,
       76.39,    78.89,    81.36,    83.87,    86.43,    88.88,
       91.40,    93.81,    96.41,    98.87,   101.44,
];

/// Exhibit 25-A7 (chap25.pdf p.231), SUT, 65 mi/h initial speed, 3% grade (s).
static FAMB_SUT_INIT65_G3: [f64; 41] = [
        0.00,     2.68,     5.35,     8.03,    10.71,    13.38,
       16.06,    18.73,    21.22,    23.90,    26.66,    29.52,
       32.19,    34.94,    37.71,    40.37,    43.25,    45.95,
       48.68,    51.58,    54.33,    57.09,    59.94,    62.68,
       65.43,    68.27,    71.07,    73.80,    76.69,    79.45,
       82.27,    85.15,    87.91,    90.71,    93.58,    96.32,
       99.18,   101.81,   104.75,   107.60,   110.38,
];

/// Exhibit 25-A7 (chap25.pdf p.231), SUT, 65 mi/h initial speed, 5% grade (s).
static FAMB_SUT_INIT65_G5: [f64; 41] = [
        0.00,     2.80,     5.60,     8.41,    11.21,    13.73,
       16.61,    19.93,    24.21,    24.67,    29.10,    32.21,
       35.53,    38.77,    42.11,    45.47,    48.79,    52.27,
       55.62,    59.14,    62.66,    66.15,    69.69,    73.17,
       76.80,    80.28,    83.87,    87.42,    91.04,    94.62,
       98.23,   101.90,   105.46,   109.14,   112.69,   116.40,
      119.96,   123.33,   127.20,   130.78,   134.41,
];

/// Exhibit 25-22 (chap25.pdf p.80), SUT, 70 mi/h initial speed, 0% grade (s).
static FAMB_SUT_INIT70_G0: [f64; 41] = [
        0.00,     2.30,     4.61,     6.91,     9.22,    11.52,
       13.82,    16.13,    18.43,    20.74,    23.04,    25.34,
       27.65,    29.95,    32.26,    34.56,    36.87,    39.17,
       41.47,    43.78,    46.08,    48.39,    50.69,    52.99,
       55.30,    57.60,    59.91,    62.21,    64.51,    66.82,
       69.12,    71.43,    73.73,    76.03,    78.34,    80.64,
       82.95,    85.25,    87.56,    89.86,    92.00,
];

/// Exhibit 25-22 (chap25.pdf p.80), SUT, 70 mi/h initial speed, 2% grade (s).
static FAMB_SUT_INIT70_G2: [f64; 41] = [
        0.00,     2.47,     4.93,     7.40,     9.87,    12.34,
       14.80,    17.27,    19.74,    22.21,    24.67,    27.14,
       29.61,    31.47,    34.11,    36.74,    39.19,    41.60,
       44.23,    46.67,    49.10,    51.69,    54.13,    56.58,
       59.08,    61.60,    64.06,    66.52,    69.09,    71.58,
       74.05,    76.61,    79.07,    81.53,    84.02,    86.58,
       89.01,    91.53,    94.08,    96.50,    98.91,
];

/// Exhibit 25-22 (chap25.pdf p.80), SUT, 70 mi/h initial speed, 3% grade (s).
static FAMB_SUT_INIT70_G3: [f64; 41] = [
        0.00,     2.52,     5.04,     7.57,    10.09,    12.61,
       15.13,    17.00,    19.74,    22.49,    25.14,    27.89,
       30.46,    33.10,    35.86,    38.49,    41.13,    43.86,
       46.58,    49.28,    52.11,    54.82,    57.50,    60.36,
       63.06,    65.78,    68.67,    71.37,    74.10,    76.99,
       79.72,    82.48,    85.36,    88.12,    90.88,    93.75,
       96.52,    99.32,   102.17,   104.93,   107.60,
];

/// Exhibit 25-22 (chap25.pdf p.80), SUT, 70 mi/h initial speed, 5% grade (s).
static FAMB_SUT_INIT70_G5: [f64; 41] = [
        0.00,     2.78,     5.57,     8.35,    11.13,    13.91,
       16.70,    19.48,    22.26,    25.05,    27.72,    30.31,
       33.34,    36.59,    39.72,    42.95,    46.30,    49.56,
       53.03,    56.40,    59.84,    63.27,    66.68,    70.25,
       73.70,    77.31,    80.78,    84.38,    87.90,    91.43,
       95.17,    98.68,   102.39,   105.92,   109.60,   113.15,
      116.79,   120.43,   124.00,   127.69,   131.18,
];

/// Exhibit 25-A15 (chap25.pdf p.239), TT, 50 mi/h initial speed, 0% grade (s).
static FAMB_TT_INIT50_G0: [f64; 41] = [
        0.00,     2.77,     5.53,     8.30,    11.06,    13.83,
       16.60,    19.36,    22.13,    24.89,    27.66,    30.43,
       33.19,    35.96,    38.73,    41.49,    44.26,    47.02,
       49.79,    52.56,    55.53,    57.32,    59.74,    62.47,
       64.78,    67.31,    69.86,    71.92,    73.96,    76.84,
       79.21,    81.36,    83.39,    85.80,    87.88,    90.74,
       92.93,    94.98,    96.99,    99.60,   101.31,
];

/// Exhibit 25-A15 (chap25.pdf p.239), TT, 50 mi/h initial speed, 2% grade (s).
static FAMB_TT_INIT50_G2: [f64; 41] = [
        0.00,     3.28,     6.55,     9.83,    13.10,    16.38,
       19.65,    22.93,    26.23,    29.33,    32.64,    35.78,
       38.74,    42.24,    44.92,    47.97,    51.77,    54.83,
       57.54,    60.48,    63.92,    67.21,    70.26,    73.25,
       76.24,    79.23,    82.31,    85.20,    88.41,    91.33,
       94.06,    97.33,   100.00,   103.29,   106.30,   109.41,
      112.90,   115.41,   118.74,   121.72,   124.13,
];

/// Exhibit 25-A15 (chap25.pdf p.239), TT, 50 mi/h initial speed, 3% grade (s).
static FAMB_TT_INIT50_G3: [f64; 41] = [
        0.00,     3.54,     7.08,    10.61,    14.15,    17.69,
       21.23,    24.77,    27.89,    31.36,    34.64,    38.21,
       41.74,    44.86,    48.51,    51.95,    55.93,    59.00,
       62.82,    66.58,    70.15,    73.81,    77.48,    80.53,
       83.68,    87.67,    91.45,    95.14,    98.55,   102.29,
      106.03,   109.48,   112.78,   116.21,   120.16,   123.60,
      127.18,   130.91,   134.61,   138.09,   141.01,
];

/// Exhibit 25-A15 (chap25.pdf p.239), TT, 50 mi/h initial speed, 5% grade (s).
static FAMB_TT_INIT50_G5: [f64; 41] = [
        0.00,     4.19,     8.38,    12.56,    16.75,    20.94,
       24.63,    26.68,    29.46,    35.38,    40.22,    44.64,
       49.13,    53.80,    58.42,    63.44,    68.69,    73.16,
       78.00,    82.83,    88.02,    92.50,    97.81,   102.83,
      108.01,   112.82,   118.11,   122.92,   128.13,   133.22,
      138.19,   143.18,   148.25,   153.60,   158.40,   163.21,
      168.42,   173.41,   178.65,   183.33,   187.87,
];

/// Exhibit 25-A16 (chap25.pdf p.240), TT, 55 mi/h initial speed, 0% grade (s).
static FAMB_TT_INIT55_G0: [f64; 41] = [
        0.00,     3.00,     5.99,     8.99,    11.99,    14.98,
       17.98,    20.14,    22.73,    25.11,    28.03,    30.46,
       33.05,    35.59,    38.10,    40.66,    43.03,    45.45,
       47.92,    50.48,    52.75,    55.13,    57.54,    59.92,
       62.28,    64.62,    66.95,    69.29,    71.62,    73.94,
       76.23,    78.39,    80.78,    82.97,    85.30,    87.61,
       89.79,    92.06,    94.41,    96.67,    98.84,
];

/// Exhibit 25-A16 (chap25.pdf p.240), TT, 55 mi/h initial speed, 2% grade (s).
static FAMB_TT_INIT55_G2: [f64; 41] = [
        0.00,     2.79,     5.57,     8.36,    11.15,    13.94,
       16.80,    20.63,    24.20,    27.12,    30.35,    33.44,
       36.57,    39.63,    42.64,    45.76,    48.74,    51.73,
       54.70,    57.74,    60.72,    63.79,    66.80,    69.81,
       72.90,    75.78,    78.74,    81.81,    84.81,    87.82,
       90.88,    93.85,    96.85,    99.80,   102.75,   105.73,
      108.79,   111.81,   114.81,   117.88,   120.59,
];

/// Exhibit 25-A16 (chap25.pdf p.240), TT, 55 mi/h initial speed, 3% grade (s).
static FAMB_TT_INIT55_G3: [f64; 41] = [
        0.00,     3.17,     6.35,     9.52,    12.70,    15.87,
       19.05,    21.82,    25.31,    28.91,    32.17,    35.57,
       38.85,    42.27,    45.77,    49.19,    52.56,    55.90,
       59.43,    62.94,    66.32,    69.85,    73.39,    76.80,
       80.34,    83.79,    87.38,    90.97,    94.46,    97.96,
      101.46,   105.08,   108.64,   112.20,   115.66,   119.34,
      122.85,   126.43,   130.03,   133.54,   137.05,
];

/// Exhibit 25-A16 (chap25.pdf p.240), TT, 55 mi/h initial speed, 5% grade (s).
static FAMB_TT_INIT55_G5: [f64; 41] = [
        0.00,     3.44,     6.89,    10.33,    13.77,    16.68,
       20.90,    26.23,    27.21,    32.63,    36.63,    41.01,
       45.46,    49.93,    54.39,    59.10,    63.81,    68.53,
       73.42,    78.17,    83.08,    88.00,    92.94,    97.97,
      102.93,   107.97,   113.06,   118.04,   122.95,   127.98,
      132.98,   138.07,   143.12,   148.19,   153.31,   158.36,
      163.50,   168.51,   173.40,   178.52,   183.46,
];

/// Exhibit 25-A18 (chap25.pdf p.242), TT, 65 mi/h initial speed, 0% grade (s).
static FAMB_TT_INIT65_G0: [f64; 41] = [
        0.00,     2.62,     5.24,     7.85,    10.47,    13.09,
       15.71,    17.75,    20.32,    22.62,    25.00,    27.28,
       29.67,    32.06,    34.48,    36.80,    39.13,    41.54,
       43.75,    46.13,    48.48,    50.54,    52.89,    55.18,
       57.37,    59.66,    61.85,    64.08,    66.30,    68.60,
       70.94,    73.12,    75.47,    77.79,    79.96,    82.25,
       84.59,    86.83,    89.10,    91.46,    93.61,
];

/// Exhibit 25-A18 (chap25.pdf p.242), TT, 65 mi/h initial speed, 2% grade (s).
static FAMB_TT_INIT65_G2: [f64; 41] = [
        0.00,     2.82,     5.64,     8.46,    11.27,    14.09,
       15.66,    16.15,    20.14,    23.19,    26.61,    29.49,
       32.29,    35.16,    38.06,    40.77,    43.63,    46.47,
       49.33,    52.18,    55.07,    57.71,    60.58,    63.18,
       66.13,    68.97,    71.93,    74.80,    77.78,    80.69,
       83.50,    86.42,    89.39,    92.33,    95.29,    98.21,
      101.18,   104.11,   106.97,   109.93,   112.86,
];

/// Exhibit 25-A18 (chap25.pdf p.242), TT, 65 mi/h initial speed, 3% grade (s).
static FAMB_TT_INIT65_G3: [f64; 41] = [
        0.00,     2.79,     5.58,     8.37,    11.15,    13.94,
       16.73,    19.52,    22.31,    24.94,    28.07,    31.13,
       34.10,    37.21,    40.39,    43.54,    46.68,    49.89,
       53.10,    56.32,    59.60,    62.61,    65.87,    69.22,
       72.61,    75.95,    79.38,    82.74,    86.20,    89.64,
       93.10,    96.50,    99.97,   103.43,   106.95,   110.47,
      114.04,   117.61,   121.11,   124.68,   128.15,
];

/// Exhibit 25-A18 (chap25.pdf p.242), TT, 65 mi/h initial speed, 5% grade (s).
static FAMB_TT_INIT65_G5: [f64; 41] = [
        0.00,     2.88,     5.76,     8.64,    11.51,    14.39,
       16.87,    20.88,    25.83,    26.35,    31.30,    34.93,
       38.83,    42.97,    46.88,    51.26,    55.54,    60.05,
       64.30,    68.77,    73.57,    78.03,    82.79,    87.54,
       92.57,    97.40,   102.41,   107.39,   112.38,   117.38,
      122.41,   127.45,   132.42,   137.70,   142.56,   147.57,
      152.61,   157.71,   162.77,   167.83,   172.86,
];

/// Exhibit 26-A4 (chap26.pdf p.172), SUT, 65 mi/h FFS, 0% grade (s).
static FAMB_SUT_FFS65_G0: [f64; 41] = [
        0.00,     2.47,     4.95,     7.42,     9.90,    12.37,
       14.84,    17.32,    19.87,    21.93,    24.22,    26.50,
       28.76,    31.02,    33.27,    35.63,    37.86,    40.12,
       42.41,    44.69,    46.97,    49.14,    51.54,    53.76,
       55.98,    58.22,    60.62,    62.84,    65.04,    67.43,
       69.66,    71.90,    74.13,    76.51,    78.79,    80.96,
       83.31,    85.53,    87.86,    90.06,    92.39,
];

/// Exhibit 26-A4 (chap26.pdf p.172), SUT, 65 mi/h FFS, 2% grade (s).
static FAMB_SUT_FFS65_G2: [f64; 41] = [
        0.00,     2.62,     5.23,     7.85,    10.47,    13.08,
       15.70,    17.71,    19.98,    22.95,    25.55,    28.23,
       30.86,    33.36,    36.00,    38.53,    41.04,    43.64,
       46.14,    48.63,    51.20,    53.72,    56.23,    58.74,
       61.28,    63.78,    66.32,    68.86,    71.35,    73.83,
       76.41,    78.90,    81.39,    83.90,    86.47,    88.94,
       91.48,    93.68,    96.51,    98.98,   101.49,
];

/// Exhibit 26-A4 (chap26.pdf p.172), SUT, 65 mi/h FFS, 3% grade (s).
static FAMB_SUT_FFS65_G3: [f64; 41] = [
        0.00,     2.64,     5.28,     7.92,    10.56,    13.20,
       15.84,    18.55,    21.05,    23.86,    26.72,    29.45,
       32.20,    34.89,    37.73,    40.39,    43.32,    46.02,
       48.74,    51.62,    54.35,    57.09,    59.96,    62.69,
       65.45,    68.33,    71.11,    73.87,    76.74,    79.50,
       82.26,    85.12,    87.93,    90.76,    93.64,    96.38,
       99.25,   101.85,   104.85,   107.81,   110.49,
];

/// Exhibit 26-A4 (chap26.pdf p.172), SUT, 65 mi/h FFS, 5% grade (s).
static FAMB_SUT_FFS65_G5: [f64; 41] = [
        0.00,     2.95,     5.90,     8.84,    11.79,    14.74,
       17.69,    20.64,    23.58,    26.53,    29.15,    32.21,
       35.55,    38.76,    42.14,    45.52,    48.81,    52.34,
       55.68,    59.19,    62.68,    66.14,    69.72,    73.21,
       76.84,    80.34,    83.94,    87.51,    91.10,    94.73,
       98.26,   101.95,   105.48,   109.20,   112.78,   116.49,
      120.09,   123.52,   127.36,   130.93,   134.57,
];

/// Exhibit 26-A9 (chap26.pdf p.177), TT, 65 mi/h FFS, 0% grade (s).
static FAMB_TT_FFS65_G0: [f64; 41] = [
        0.00,     2.57,     5.14,     7.71,    10.28,    12.85,
       15.42,    17.77,    20.33,    22.57,    24.94,    27.25,
       29.65,    32.03,    34.40,    36.76,    39.08,    41.49,
       43.69,    46.08,    48.37,    50.61,    52.89,    55.25,
       57.43,    59.75,    62.05,    64.24,    66.61,    68.85,
       71.12,    73.49,    75.67,    77.95,    80.26,    82.42,
       84.79,    87.06,    89.29,    91.60,    93.86,
];

/// Exhibit 26-A9 (chap26.pdf p.177), TT, 65 mi/h FFS, 2% grade (s).
static FAMB_TT_FFS65_G2: [f64; 41] = [
        0.00,     2.63,     5.26,     7.89,    10.52,    13.15,
       15.77,    18.40,    21.03,    23.05,    26.58,    29.39,
       32.23,    35.12,    38.00,    40.70,    43.58,    46.40,
       49.24,    52.09,    55.00,    57.81,    60.61,    63.56,
       66.46,    69.30,    72.19,    75.14,    77.95,    80.86,
       83.81,    86.68,    89.50,    92.49,    95.47,    98.37,
      101.32,   104.24,   107.26,   110.21,   112.89,
];

/// Exhibit 26-A9 (chap26.pdf p.177), TT, 65 mi/h FFS, 3% grade (s).
static FAMB_TT_FFS65_G3: [f64; 41] = [
        0.00,     2.72,     5.44,     8.16,    10.88,    13.60,
       16.32,    19.04,    21.76,    24.94,    28.06,    31.06,
       34.07,    37.16,    40.32,    43.45,    46.60,    49.72,
       52.99,    56.28,    59.54,    62.86,    66.19,    69.50,
       72.91,    76.30,    79.53,    82.94,    86.37,    89.84,
       93.29,    96.76,   100.27,   103.79,   107.23,   110.73,
      114.21,   117.73,   121.23,   124.80,   128.30,
];

/// Exhibit 26-A9 (chap26.pdf p.177), TT, 65 mi/h FFS, 5% grade (s).
static FAMB_TT_FFS65_G5: [f64; 41] = [
        0.00,     2.94,     5.87,     8.81,    11.74,    14.16,
       17.39,    20.81,    25.18,    27.01,    31.19,    34.95,
       38.76,    42.75,    46.90,    51.11,    55.43,    59.86,
       64.33,    69.03,    73.65,    78.22,    83.02,    87.94,
       92.78,    97.71,   102.59,   107.56,   112.58,   117.58,
      122.58,   127.59,   132.59,   137.58,   142.68,   147.67,
      152.76,   157.77,   162.86,   167.89,   172.93,
];


// ============================================================================
// Exhibits 25-24 / 26-7 and 25-25 / 26-8 -- delta, the slope of the travel time
// versus distance curve beyond 10,000 ft (s/ft).
//
// The two chapters print these tables identically. They are indexed by the
// segment FREE-FLOW SPEED, not by the truck's initial speed: Chapter 25
// Example Problem 11 reads its Segment 2 curves off the 60 mi/h (SUT) and
// 50 mi/h (TT) exhibits but takes delta from the FFS-65 column. Getting that
// wrong is silent, because every column is the same order of magnitude.
// ============================================================================

/// FFS columns of Exhibits 25-24/25-25 (mi/h).
const DELTA_FFS_COLUMNS: [u32; 6] = [50, 55, 60, 65, 70, 75];

/// Grade rows of Exhibits 25-24/25-25 (percent).
const DELTA_GRADE_ROWS: [i32; 9] = [-5, 0, 2, 3, 4, 5, 6, 7, 8];

/// Exhibit 25-24 / 26-7 -- delta for SUTs (s/ft), rows by grade, columns by FFS.
static DELTA_SUT: [[f64; 6]; 9] = [
    [0.0136, 0.0124, 0.0114, 0.0105, 0.0097, 0.0091], // -5%
    [0.0136, 0.0124, 0.0114, 0.0105, 0.0097, 0.0091], // 0%
    [0.0136, 0.0124, 0.0114, 0.0105, 0.0100, 0.0099], // 2%
    [0.0136, 0.0124, 0.0114, 0.0113, 0.0112, 0.0112], // 3%
    [0.0136, 0.0129, 0.0128, 0.0128, 0.0128, 0.0127], // 4%
    [0.0146, 0.0146, 0.0146, 0.0146, 0.0145, 0.0145], // 5%
    [0.0165, 0.0165, 0.0165, 0.0165, 0.0165, 0.0165], // 6%
    [0.0186, 0.0186, 0.0186, 0.0186, 0.0186, 0.0186], // 7%
    [0.0208, 0.0208, 0.0208, 0.0208, 0.0208, 0.0208], // 8%
];

/// Exhibit 25-25 / 26-8 -- delta for TTs (s/ft), rows by grade, columns by FFS.
static DELTA_TT: [[f64; 6]; 9] = [
    [0.0136, 0.0124, 0.0114, 0.0105, 0.0097, 0.0091], // -5%
    [0.0136, 0.0124, 0.0114, 0.0105, 0.0097, 0.0091], // 0%
    [0.0136, 0.0124, 0.0119, 0.0118, 0.0116, 0.0115], // 2%
    [0.0143, 0.0143, 0.0142, 0.0141, 0.0140, 0.0138], // 3%
    [0.0171, 0.0171, 0.0171, 0.0170, 0.0169, 0.0168], // 4%
    [0.0202, 0.0202, 0.0202, 0.0202, 0.0202, 0.0202], // 5%
    [0.0236, 0.0236, 0.0236, 0.0236, 0.0236, 0.0236], // 6%
    [0.0272, 0.0272, 0.0272, 0.0272, 0.0272, 0.0272], // 7%
    [0.0310, 0.0310, 0.0310, 0.0310, 0.0310, 0.0310], // 8%
];

// ============================================================================
// Public API
// ============================================================================

use serde::{Deserialize, Serialize};

/// The two truck classes the mixed-flow model tracks separately.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TruckClass {
    /// Single-unit truck.
    Sut,
    /// Tractor-trailer.
    Tt,
}

/// Which branch of a spot-rate curve applies. A truck entering a grade faster than the
/// grade's crawl speed decelerates towards it; one entering slower accelerates towards it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpotBranch {
    /// Solid curves of Exhibit 25-20/25-21 (entry at 75 mi/h).
    Decelerating,
    /// Dashed curves of Exhibit 25-20/25-21 (SUT entry at 30 mi/h, TT at 20 mi/h).
    Accelerating,
}

/// Which family of cumulative travel time curves to read.
///
/// Chapter 25's Appendix A is indexed by the truck's initial speed; Chapter 26's Appendix A
/// is indexed by the segment free-flow speed. They cover overlapping ground but are not
/// interchangeable, and the manual is explicit that a composite-grade analysis reads the
/// Chapter 25 family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CurveFamily {
    /// Chapter 25 Appendix A / Exhibit 25-22, keyed by initial speed (mi/h).
    Ch25InitialSpeed(u32),
    /// Chapter 26 Appendix A, keyed by free-flow speed (mi/h).
    Ch26Ffs(u32),
}

/// Initial speeds digitised in Stage 1 for Chapter 25 SUT curves (mi/h).
pub const STAGE1_CH25_SUT_SPEEDS: [u32; 3] = [60, 65, 70];
/// Initial speeds digitised in Stage 1 for Chapter 25 TT curves (mi/h).
pub const STAGE1_CH25_TT_SPEEDS: [u32; 3] = [50, 55, 65];
/// FFS values digitised in Stage 1 for Chapter 26 curves, both classes (mi/h).
pub const STAGE1_CH26_FFS: [u32; 1] = [65];

/// Map a grade in percent onto the Stage-1 grade column, or explain what is missing.
///
/// -5% shares a plotted line with 0% in every source exhibit, so it resolves to the same
/// column. Grades are matched exactly rather than interpolated: the manual's curves are one
/// per tabulated grade and the crawl speed between them is not linear in grade.
fn grade_column(grade_pct: f64, exhibit: &str) -> Result<i32, String> {
    if !grade_pct.is_finite() {
        return Err(format!("grade must be finite, got {grade_pct}"));
    }
    let g = if (grade_pct + 5.0).abs() < 1e-9 { 0.0 } else { grade_pct };
    for &col in STAGE1_GRADES.iter() {
        if (g - f64::from(col)).abs() < 1e-9 {
            return Ok(col);
        }
    }
    Err(format!(
        "grade {grade_pct}% is not digitised in Stage 1 of the mixed-flow truck curves \
         (available: -5, 0, 2, 3, 5); {exhibit} would have to be digitised for it. The curves \
         are not extrapolated because each grade settles at its own crawl speed."
    ))
}

fn spot_table(class: TruckClass, grade: i32, branch: SpotBranch) -> &'static [f64; 41] {
    match (class, branch, grade) {
        (TruckClass::Sut, SpotBranch::Decelerating, 0) => &SPOT_SUT_DECEL_G0,
        (TruckClass::Sut, SpotBranch::Decelerating, 2) => &SPOT_SUT_DECEL_G2,
        (TruckClass::Sut, SpotBranch::Decelerating, 3) => &SPOT_SUT_DECEL_G3,
        (TruckClass::Sut, SpotBranch::Decelerating, 5) => &SPOT_SUT_DECEL_G5,
        (TruckClass::Sut, SpotBranch::Accelerating, 0) => &SPOT_SUT_ACCEL_G0,
        (TruckClass::Sut, SpotBranch::Accelerating, 2) => &SPOT_SUT_ACCEL_G2,
        (TruckClass::Sut, SpotBranch::Accelerating, 3) => &SPOT_SUT_ACCEL_G3,
        (TruckClass::Sut, SpotBranch::Accelerating, 5) => &SPOT_SUT_ACCEL_G5,
        (TruckClass::Tt, SpotBranch::Decelerating, 0) => &SPOT_TT_DECEL_G0,
        (TruckClass::Tt, SpotBranch::Decelerating, 2) => &SPOT_TT_DECEL_G2,
        (TruckClass::Tt, SpotBranch::Decelerating, 3) => &SPOT_TT_DECEL_G3,
        (TruckClass::Tt, SpotBranch::Decelerating, 5) => &SPOT_TT_DECEL_G5,
        (TruckClass::Tt, SpotBranch::Accelerating, 0) => &SPOT_TT_ACCEL_G0,
        (TruckClass::Tt, SpotBranch::Accelerating, 2) => &SPOT_TT_ACCEL_G2,
        (TruckClass::Tt, SpotBranch::Accelerating, 3) => &SPOT_TT_ACCEL_G3,
        (TruckClass::Tt, SpotBranch::Accelerating, 5) => &SPOT_TT_ACCEL_G5,
        _ => unreachable!("grade_column admits only the Stage-1 grades"),
    }
}

fn famb_table(class: TruckClass, family: CurveFamily, grade: i32) -> Result<&'static [f64; 41], String> {
    let t = match (class, family, grade) {
        (TruckClass::Sut, CurveFamily::Ch25InitialSpeed(60), 0) => &FAMB_SUT_INIT60_G0,
        (TruckClass::Sut, CurveFamily::Ch25InitialSpeed(60), 2) => &FAMB_SUT_INIT60_G2,
        (TruckClass::Sut, CurveFamily::Ch25InitialSpeed(60), 3) => &FAMB_SUT_INIT60_G3,
        (TruckClass::Sut, CurveFamily::Ch25InitialSpeed(60), 5) => &FAMB_SUT_INIT60_G5,
        (TruckClass::Sut, CurveFamily::Ch25InitialSpeed(65), 0) => &FAMB_SUT_INIT65_G0,
        (TruckClass::Sut, CurveFamily::Ch25InitialSpeed(65), 2) => &FAMB_SUT_INIT65_G2,
        (TruckClass::Sut, CurveFamily::Ch25InitialSpeed(65), 3) => &FAMB_SUT_INIT65_G3,
        (TruckClass::Sut, CurveFamily::Ch25InitialSpeed(65), 5) => &FAMB_SUT_INIT65_G5,
        (TruckClass::Sut, CurveFamily::Ch25InitialSpeed(70), 0) => &FAMB_SUT_INIT70_G0,
        (TruckClass::Sut, CurveFamily::Ch25InitialSpeed(70), 2) => &FAMB_SUT_INIT70_G2,
        (TruckClass::Sut, CurveFamily::Ch25InitialSpeed(70), 3) => &FAMB_SUT_INIT70_G3,
        (TruckClass::Sut, CurveFamily::Ch25InitialSpeed(70), 5) => &FAMB_SUT_INIT70_G5,
        (TruckClass::Tt, CurveFamily::Ch25InitialSpeed(50), 0) => &FAMB_TT_INIT50_G0,
        (TruckClass::Tt, CurveFamily::Ch25InitialSpeed(50), 2) => &FAMB_TT_INIT50_G2,
        (TruckClass::Tt, CurveFamily::Ch25InitialSpeed(50), 3) => &FAMB_TT_INIT50_G3,
        (TruckClass::Tt, CurveFamily::Ch25InitialSpeed(50), 5) => &FAMB_TT_INIT50_G5,
        (TruckClass::Tt, CurveFamily::Ch25InitialSpeed(55), 0) => &FAMB_TT_INIT55_G0,
        (TruckClass::Tt, CurveFamily::Ch25InitialSpeed(55), 2) => &FAMB_TT_INIT55_G2,
        (TruckClass::Tt, CurveFamily::Ch25InitialSpeed(55), 3) => &FAMB_TT_INIT55_G3,
        (TruckClass::Tt, CurveFamily::Ch25InitialSpeed(55), 5) => &FAMB_TT_INIT55_G5,
        (TruckClass::Tt, CurveFamily::Ch25InitialSpeed(65), 0) => &FAMB_TT_INIT65_G0,
        (TruckClass::Tt, CurveFamily::Ch25InitialSpeed(65), 2) => &FAMB_TT_INIT65_G2,
        (TruckClass::Tt, CurveFamily::Ch25InitialSpeed(65), 3) => &FAMB_TT_INIT65_G3,
        (TruckClass::Tt, CurveFamily::Ch25InitialSpeed(65), 5) => &FAMB_TT_INIT65_G5,
        (TruckClass::Sut, CurveFamily::Ch26Ffs(65), 0) => &FAMB_SUT_FFS65_G0,
        (TruckClass::Sut, CurveFamily::Ch26Ffs(65), 2) => &FAMB_SUT_FFS65_G2,
        (TruckClass::Sut, CurveFamily::Ch26Ffs(65), 3) => &FAMB_SUT_FFS65_G3,
        (TruckClass::Sut, CurveFamily::Ch26Ffs(65), 5) => &FAMB_SUT_FFS65_G5,
        (TruckClass::Tt, CurveFamily::Ch26Ffs(65), 0) => &FAMB_TT_FFS65_G0,
        (TruckClass::Tt, CurveFamily::Ch26Ffs(65), 2) => &FAMB_TT_FFS65_G2,
        (TruckClass::Tt, CurveFamily::Ch26Ffs(65), 3) => &FAMB_TT_FFS65_G3,
        (TruckClass::Tt, CurveFamily::Ch26Ffs(65), 5) => &FAMB_TT_FFS65_G5,
        (c, f, g) => {
            let (chapter, key) = match f {
                CurveFamily::Ch25InitialSpeed(s) => ("Chapter 25 Appendix A", format!("{s} mi/h initial speed")),
                CurveFamily::Ch26Ffs(s) => ("Chapter 26 Appendix A", format!("{s} mi/h FFS")),
            };
            return Err(format!(
                "no Stage-1 travel time curve for {c:?} at {key}, {g}% grade; the {chapter} \
                 exhibit for that combination has not been digitised. Stage 1 covers the \
                 combinations the published worked examples use: Chapter 25 SUT at 60/65/70 \
                 mi/h, Chapter 25 TT at 50/55/65 mi/h, Chapter 26 at 65 mi/h FFS, each at \
                 -5/0/2/3/5% grade."
            ));
        }
    };
    Ok(t)
}

/// Linear interpolation on the 250 ft station grid. Distances are clamped to the tabulated
/// range: past 10,000 ft the caller must use Equation 25-59 / 26-12 with `delta` instead,
/// which is what `travel_time_rate` does.
fn interp(table: &[f64; 41], ft: f64) -> f64 {
    let last = STATIONS_FT[STATIONS_FT.len() - 1];
    if ft <= 0.0 {
        return table[0];
    }
    if ft >= last {
        return table[table.len() - 1];
    }
    let i = (ft / 250.0).floor() as usize;
    let t = (ft - STATIONS_FT[i]) / 250.0;
    table[i] + t * (table[i + 1] - table[i])
}

/// Spot travel time rate at a distance into a grade (s/mi).
///
/// This is the ordinate of Exhibit 25-20 (SUTs) or 25-21 (TTs).
pub fn spot_rate(
    class: TruckClass,
    grade_pct: f64,
    branch: SpotBranch,
    distance_ft: f64,
) -> Result<f64, String> {
    if !distance_ft.is_finite() || distance_ft < 0.0 {
        return Err(format!("distance must be finite and non-negative, got {distance_ft} ft"));
    }
    let g = grade_column(grade_pct, "Exhibit 25-20/25-21")?;
    Ok(interp(spot_table(class, g, branch), distance_ft))
}

/// Distance into a grade at which a truck reaches a given spot rate (ft).
///
/// This is the inverse read the manual's Step 4 calls for: to carry a truck's speed from one
/// segment to the next you find the abscissa whose ordinate equals the entry rate, treat that
/// as the origin, and advance by the segment length. Returns the clamped end of the curve
/// when the rate lies beyond it, which is the manual's own behaviour ("crawl speed is reached
/// at around 10,000 ft").
pub fn spot_distance(
    class: TruckClass,
    grade_pct: f64,
    branch: SpotBranch,
    rate: f64,
) -> Result<f64, String> {
    if !rate.is_finite() {
        return Err(format!("spot rate must be finite, got {rate}"));
    }
    let g = grade_column(grade_pct, "Exhibit 25-20/25-21")?;
    let t = spot_table(class, g, branch);
    let rising = t[t.len() - 1] > t[0];
    // Both branches are monotone, one rising and one falling, so a single scan suffices.
    for i in 0..t.len() - 1 {
        let (a, b) = (t[i], t[i + 1]);
        let hit = if rising { rate >= a && rate <= b } else { rate <= a && rate >= b };
        if hit {
            if (b - a).abs() < 1e-12 {
                return Ok(STATIONS_FT[i]);
            }
            return Ok(STATIONS_FT[i] + 250.0 * (rate - a) / (b - a));
        }
    }
    Ok(if rising == (rate > t[0]) { STATIONS_FT[t.len() - 1] } else { 0.0 })
}

/// The crawl rate a grade drives a truck towards (s/mi), read at 10,000 ft.
pub fn crawl_rate(class: TruckClass, grade_pct: f64, branch: SpotBranch) -> Result<f64, String> {
    spot_rate(class, grade_pct, branch, STATIONS_FT[STATIONS_FT.len() - 1])
}

/// Cumulative travel time over a length of grade (s), Exhibit 25-22/25-A* or 26-A*.
pub fn travel_time(
    class: TruckClass,
    family: CurveFamily,
    grade_pct: f64,
    distance_ft: f64,
) -> Result<f64, String> {
    if !distance_ft.is_finite() || distance_ft < 0.0 {
        return Err(format!("distance must be finite and non-negative, got {distance_ft} ft"));
    }
    let g = grade_column(grade_pct, "the travel time exhibit")?;
    Ok(interp(famb_table(class, family, g)?, distance_ft))
}

/// delta from Exhibit 25-24/25-25 (s/ft), indexed by grade and segment FFS.
pub fn delta(class: TruckClass, grade_pct: f64, ffs: f64) -> Result<f64, String> {
    if !grade_pct.is_finite() || !ffs.is_finite() {
        return Err(format!("grade and FFS must be finite, got grade {grade_pct}, FFS {ffs}"));
    }
    let row = DELTA_GRADE_ROWS
        .iter()
        .position(|&r| (grade_pct - f64::from(r)).abs() < 1e-9)
        .ok_or_else(|| {
            format!(
                "grade {grade_pct}% is not a row of HCM Exhibit 25-24/25-25 \
                 (rows: -5, 0, 2, 3, 4, 5, 6, 7, 8)"
            )
        })?;
    let col = DELTA_FFS_COLUMNS
        .iter()
        .position(|&c| (ffs - f64::from(c)).abs() < 1e-9)
        .ok_or_else(|| {
            format!(
                "FFS {ffs} mi/h is not a column of HCM Exhibit 25-24/25-25 \
                 (columns: 50, 55, 60, 65, 70, 75)"
            )
        })?;
    Ok(match class {
        TruckClass::Sut => DELTA_SUT[row][col],
        TruckClass::Tt => DELTA_TT[row][col],
    })
}

/// Travel time rate over a length of grade (s/mi): Equation 26-11/25-58 within the plotted
/// range, Equation 26-12/25-59 beyond 10,000 ft.
///
/// `ffs` selects the delta column and, per Chapter 25 Example Problem 11, is the SEGMENT free-flow
/// speed even when the curve itself was chosen for a different initial speed.
pub fn travel_time_rate(
    class: TruckClass,
    family: CurveFamily,
    grade_pct: f64,
    length_mi: f64,
    ffs: f64,
) -> Result<f64, String> {
    if !length_mi.is_finite() || length_mi <= 0.0 {
        return Err(format!("segment length must be positive, got {length_mi} mi"));
    }
    let ft = length_mi * 5280.0;
    if ft <= 10_000.0 {
        return Ok(travel_time(class, family, grade_pct, ft)? / length_mi);
    }
    let t10 = travel_time(class, family, grade_pct, 10_000.0)?;
    let d = delta(class, grade_pct, ffs)?;
    Ok(t10 / length_mi + d * (1.0 - 10_000.0 / (5280.0 * length_mi)) * 5280.0)
}

/// Snap a speed onto the nearest digitised exhibit for a class and family.
///
/// Chapter 25 Step 4 says to "use the Appendix A graph that has a starting spot speed closest
/// to the value computed in the first substep", within 2.5 mi/h. The exhibits step by 5 mi/h,
/// so the model snaps rather than interpolating between graphs.
pub fn nearest_exhibit(class: TruckClass, speed_mih: f64) -> Result<CurveFamily, String> {
    if !speed_mih.is_finite() || speed_mih <= 0.0 {
        return Err(format!("speed must be positive and finite, got {speed_mih} mi/h"));
    }
    let avail: &[u32] = match class {
        TruckClass::Sut => &STAGE1_CH25_SUT_SPEEDS,
        TruckClass::Tt => &STAGE1_CH25_TT_SPEEDS,
    };
    let best = avail
        .iter()
        .copied()
        .min_by(|a, b| {
            (speed_mih - f64::from(*a))
                .abs()
                .partial_cmp(&(speed_mih - f64::from(*b)).abs())
                .expect("finite speeds compare")
        })
        .expect("Stage-1 speed lists are non-empty");
    if (speed_mih - f64::from(best)).abs() > 2.5 {
        return Err(format!(
            "no Stage-1 Chapter 25 Appendix A curve within 2.5 mi/h of {speed_mih:.1} mi/h for \
             {class:?} (digitised: {avail:?}); that exhibit would have to be digitised"
        ));
    }
    Ok(CurveFamily::Ch25InitialSpeed(best))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every spot-rate read the two worked examples state as an ORDINATE.
    ///
    /// Tolerance is the manual's own reading fidelity of about 1 s/mi. The 2% crawl reads are
    /// the loosest: both examples quote a single crawl rate per grade, but the accelerating and
    /// decelerating branches have not actually met by 10,000 ft on a 2% grade, so the quoted
    /// value sits between the two branches rather than on either.
    #[test]
    fn spot_rate_reproduces_published_ordinate_reads() {
        let cases = [
            // class, grade, branch, station ft, published s/mi, tol, source
            (TruckClass::Sut, 3.0, SpotBranch::Decelerating, 10_000.0, 59.0, 1.0, "chap25 p.208 SUT 3% crawl"),
            (TruckClass::Tt, 3.0, SpotBranch::Decelerating, 10_000.0, 73.0, 1.0, "chap25 p.208 TT 3% crawl"),
            (TruckClass::Sut, 2.0, SpotBranch::Accelerating, 10_000.0, 53.0, 1.0, "chap25 p.212 SUT 2% crawl"),
            (TruckClass::Tt, 2.0, SpotBranch::Accelerating, 10_000.0, 63.0, 1.3, "chap25 p.212 TT 2% crawl"),
            (TruckClass::Sut, 5.0, SpotBranch::Decelerating, 6_780.0, 75.0, 1.0, "chap25 p.216 SUT 5% @6,780 ft"),
            (TruckClass::Tt, 5.0, SpotBranch::Decelerating, 7_330.0, 103.0, 1.0, "chap25 p.216 TT 5% @7,330 ft"),
        ];
        for (c, g, b, ft, want, tol, src) in cases {
            let got = spot_rate(c, g, b, ft).expect("Stage-1 curve");
            assert!(
                (got - want).abs() <= tol,
                "{src}: got {got:.2} s/mi, published {want} (+-{tol})"
            );
        }
    }

    /// Every spot-rate read the examples state as an ABSCISSA.
    ///
    /// Two of these need a wider band than the +-100 ft the manual implies, and the reason is a
    /// property of the figures rather than of the digitisation. Reading a distance off a curve
    /// inverts it, so the abscissa tolerance is the ordinate tolerance divided by the local
    /// slope. The SUT 3% curve rises about 1.2 s/mi per 1,000 ft near this read, so the
    /// manual's own +-1 s/mi is worth +-820 ft there. At the stated stations the ordinates
    /// agree to 0.17 s/mi (SUT 3%) and 0.80 s/mi (TT 3%), which is the meaningful check.
    #[test]
    fn spot_distance_reproduces_published_abscissa_reads() {
        let cases = [
            (TruckClass::Sut, 3.0, SpotBranch::Decelerating, 55.4, 4_100.0, 250.0, "chap25 p.208"),
            (TruckClass::Tt, 3.0, SpotBranch::Decelerating, 55.4, 2_100.0, 250.0, "chap25 p.208"),
            (TruckClass::Sut, 2.0, SpotBranch::Accelerating, 59.0, 4_000.0, 100.0, "chap25 p.212"),
            (TruckClass::Tt, 2.0, SpotBranch::Accelerating, 73.0, 3_360.0, 100.0, "chap25 p.212"),
            (TruckClass::Sut, 5.0, SpotBranch::Decelerating, 55.4, 1_500.0, 100.0, "chap25 p.216"),
            (TruckClass::Tt, 5.0, SpotBranch::Decelerating, 63.0, 2_050.0, 100.0, "chap25 p.216"),
        ];
        for (c, g, b, rate, want, tol, src) in cases {
            let got = spot_distance(c, g, b, rate).expect("Stage-1 curve");
            assert!(
                (got - want).abs() <= tol,
                "{src}: {rate} s/mi found at {got:.0} ft, published about {want} ft (+-{tol})"
            );
        }
        // The two wide ones, checked in the direction the tolerance is actually stated in.
        for (c, g, ft, want) in [
            (TruckClass::Sut, 3.0, 4_100.0, 55.4),
            (TruckClass::Tt, 3.0, 2_100.0, 55.4),
        ] {
            let got = spot_rate(c, g, SpotBranch::Decelerating, ft).expect("Stage-1 curve");
            assert!(
                (got - want).abs() <= 1.0,
                "at the published station {ft} ft the rate is {got:.2}, published {want}"
            );
        }
    }

    /// Every cumulative travel time read the two worked examples state.
    #[test]
    fn travel_time_reproduces_published_reads() {
        let cases = [
            (TruckClass::Sut, CurveFamily::Ch25InitialSpeed(65), 3.0, 7_920.0, 87.0, 1.0, "Exh 25-A7, chap25 p.209"),
            (TruckClass::Tt, CurveFamily::Ch25InitialSpeed(65), 3.0, 7_920.0, 99.0, 1.0, "Exh 25-A18, chap25 p.209"),
            (TruckClass::Sut, CurveFamily::Ch25InitialSpeed(60), 2.0, 10_000.0, 105.0, 1.1, "Exh 25-A6, chap25 p.213"),
            (TruckClass::Tt, CurveFamily::Ch25InitialSpeed(50), 2.0, 10_000.0, 125.0, 1.0, "Exh 25-A15, chap25 p.213"),
            (TruckClass::Sut, CurveFamily::Ch25InitialSpeed(65), 5.0, 5_280.0, 67.0, 1.0, "Exh 25-A7, chap25 p.217"),
            (TruckClass::Tt, CurveFamily::Ch25InitialSpeed(55), 5.0, 5_280.0, 89.0, 1.0, "Exh 25-A16, chap25 p.217"),
            (TruckClass::Sut, CurveFamily::Ch26Ffs(65), 5.0, 10_000.0, 134.0, 1.0, "Exh 26-A4, chap26 p.72"),
            (TruckClass::Tt, CurveFamily::Ch26Ffs(65), 5.0, 10_000.0, 173.0, 1.0, "Exh 26-A9, chap26 p.72"),
        ];
        for (c, f, g, ft, want, tol, src) in cases {
            let got = travel_time(c, f, g, ft).expect("Stage-1 curve");
            assert!(
                (got - want).abs() <= tol,
                "{src}: got {got:.2} s, published {want} (+-{tol})"
            );
        }
    }

    /// The 65 mi/h exhibits of the two chapters were digitised independently from different
    /// pages of different PDFs. They describe the same truck, so they must agree; if a future
    /// edit corrupts one raster or one calibration, this catches it where a single-table test
    /// could not.
    #[test]
    fn ch25_and_ch26_65mih_exhibits_agree() {
        for g in [0.0, 2.0, 3.0, 5.0] {
            for (c, a, b) in [
                (TruckClass::Sut, CurveFamily::Ch25InitialSpeed(65), CurveFamily::Ch26Ffs(65)),
                (TruckClass::Tt, CurveFamily::Ch25InitialSpeed(65), CurveFamily::Ch26Ffs(65)),
            ] {
                let x = travel_time(c, a, g, 10_000.0).unwrap();
                let y = travel_time(c, b, g, 10_000.0).unwrap();
                assert!(
                    (x - y).abs() <= 0.5,
                    "{c:?} {g}%: Chapter 25 gives {x:.2} s, Chapter 26 gives {y:.2} s"
                );
            }
        }
    }

    /// A faster entry speed must never cost more time over the same grade. This is the check
    /// that catches a mis-assigned curve, which is the failure mode of digitising a
    /// nine-series chart: the tables stay smooth and plausible but belong to the wrong grade.
    #[test]
    fn travel_time_falls_as_initial_speed_rises() {
        for g in [0.0, 2.0, 3.0, 5.0] {
            let sut: Vec<f64> = STAGE1_CH25_SUT_SPEEDS
                .iter()
                .map(|&s| travel_time(TruckClass::Sut, CurveFamily::Ch25InitialSpeed(s), g, 10_000.0).unwrap())
                .collect();
            let tt: Vec<f64> = STAGE1_CH25_TT_SPEEDS
                .iter()
                .map(|&s| travel_time(TruckClass::Tt, CurveFamily::Ch25InitialSpeed(s), g, 10_000.0).unwrap())
                .collect();
            for w in sut.windows(2) {
                assert!(w[0] > w[1], "SUT {g}%: travel time rose with initial speed: {sut:?}");
            }
            for w in tt.windows(2) {
                assert!(w[0] > w[1], "TT {g}%: travel time rose with initial speed: {tt:?}");
            }
        }
    }

    /// A steeper grade must cost more time, and cumulative time must never decrease with
    /// distance.
    #[test]
    fn curves_are_monotone_in_grade_and_distance() {
        for &s in STAGE1_CH25_TT_SPEEDS.iter() {
            let f = CurveFamily::Ch25InitialSpeed(s);
            let mut prev = 0.0;
            for g in [0.0, 2.0, 3.0, 5.0] {
                let t = travel_time(TruckClass::Tt, f, g, 10_000.0).unwrap();
                assert!(t > prev, "TT {s} mi/h: {g}% gave {t:.2} s, not above {prev:.2}");
                prev = t;
                let mut last = -1.0;
                for ft in (0..=10_000).step_by(250) {
                    let v = travel_time(TruckClass::Tt, f, g, f64::from(ft)).unwrap();
                    assert!(v >= last - 1e-9, "TT {s} {g}%: time fell at {ft} ft");
                    last = v;
                }
            }
        }
    }

    /// Beyond 10,000 ft the rate comes from Equation 25-59/26-12 rather than the curve, and
    /// delta is read at the SEGMENT FFS. Example Problem 5 substitutes 134 s and 173 s over a
    /// 2 mi grade and prints 71.1 and 92.2 s/mi.
    #[test]
    fn travel_time_rate_beyond_10000_ft_matches_ep5() {
        let sut = travel_time_rate(TruckClass::Sut, CurveFamily::Ch26Ffs(65), 5.0, 2.0, 65.0).unwrap();
        let tt = travel_time_rate(TruckClass::Tt, CurveFamily::Ch26Ffs(65), 5.0, 2.0, 65.0).unwrap();
        assert!((sut - 71.1).abs() <= 0.5, "SUT rate {sut:.2}, published 71.1");
        assert!((tt - 92.2).abs() <= 0.5, "TT rate {tt:.2}, published 92.2");
    }

    /// Example Problem 11 reads its Segment 2 curves off the 60 mi/h (SUT) and 50 mi/h (TT)
    /// exhibits but takes delta from the FFS-65 column, not the 60 or 50 column. Pin that,
    /// because indexing delta by the curve's speed instead is silent and plausible.
    #[test]
    fn delta_is_indexed_by_segment_ffs_not_truck_speed() {
        assert_eq!(delta(TruckClass::Sut, 2.0, 65.0).unwrap(), 0.0105);
        assert_eq!(delta(TruckClass::Tt, 2.0, 65.0).unwrap(), 0.0118);
        // The columns that would be used if the truck's own curve speed were the index.
        assert_eq!(delta(TruckClass::Sut, 2.0, 60.0).unwrap(), 0.0114);
        assert_eq!(delta(TruckClass::Tt, 2.0, 50.0).unwrap(), 0.0136);
    }

    #[test]
    fn undigitised_combinations_name_the_missing_exhibit() {
        let e = travel_time(TruckClass::Sut, CurveFamily::Ch25InitialSpeed(35), 3.0, 5_000.0)
            .expect_err("35 mi/h is not in Stage 1");
        assert!(e.contains("Chapter 25 Appendix A"), "{e}");
        assert!(e.contains("not been digitised"), "{e}");

        let e = spot_rate(TruckClass::Tt, 7.0, SpotBranch::Decelerating, 5_000.0)
            .expect_err("7% is not in Stage 1");
        assert!(e.contains("Exhibit 25-20/25-21"), "{e}");

        let e = nearest_exhibit(TruckClass::Sut, 40.0).expect_err("40 mi/h is far from Stage 1");
        assert!(e.contains("2.5 mi/h"), "{e}");
    }

    /// -5% shares a plotted line with 0% in the source exhibits, so it must resolve to the
    /// same column rather than erroring.
    #[test]
    fn minus_five_percent_maps_onto_the_level_curve() {
        let a = travel_time(TruckClass::Sut, CurveFamily::Ch25InitialSpeed(65), -5.0, 8_000.0).unwrap();
        let b = travel_time(TruckClass::Sut, CurveFamily::Ch25InitialSpeed(65), 0.0, 8_000.0).unwrap();
        assert_eq!(a, b);
    }

    /// Exhibit selection snaps to the nearest 5 mi/h graph, which is what reproduces the
    /// example's own choices: Segment 2 entry speeds of 60.9 and 49.5 mi/h pick the 60 and 50
    /// exhibits.
    #[test]
    fn nearest_exhibit_reproduces_ep11_choices() {
        assert_eq!(nearest_exhibit(TruckClass::Sut, 60.9).unwrap(), CurveFamily::Ch25InitialSpeed(60));
        assert_eq!(nearest_exhibit(TruckClass::Tt, 49.5).unwrap(), CurveFamily::Ch25InitialSpeed(50));
        assert_eq!(nearest_exhibit(TruckClass::Sut, 65.0).unwrap(), CurveFamily::Ch25InitialSpeed(65));
        assert_eq!(nearest_exhibit(TruckClass::Tt, 56.1).unwrap(), CurveFamily::Ch25InitialSpeed(55));
    }
}
