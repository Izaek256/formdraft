//! Engineering tolerances for the geometry module (S02 decision D3).
//!
//! These are *engineering* tolerances used in tests and validity checks.
//! They are NOT drafting tolerances from the hand-draft workbook (OQ-17).
//!
//! # Values
//! All values are in millimetres unless stated otherwise.
//!
//! | Constant                 | Value    | Purpose                                          |
//! |--------------------------|----------|--------------------------------------------------|
//! | `CLOSURE_GAP_MM`         | 1e-6 mm  | Maximum gap for a path to count as closed        |
//! | `LENGTH_ACCURACY_MM`     | 0.01 mm  | Maximum error for arc-length measurements        |
//! | `EXPORT_TESSELLATION_MM` | 0.01 mm  | Default deviation budget for export flattening   |
//! | `FLATTEN_MM`             | 0.001 mm | Flattening tolerance used when computing length  |
//! | `ORACLE_SAMPLES`         | 100_000  | Minimum sample count for test oracles            |

/// Maximum gap between the last point and the first point of a closed path.
///
/// 1 nm = 1e-6 mm.
pub const CLOSURE_GAP_MM: f64 = 1e-6;

/// Maximum arc-length error when measuring a flattened curve against a dense
/// oracle.  Tests check that `|computed − oracle| ≤ LENGTH_ACCURACY_MM`.
pub const LENGTH_ACCURACY_MM: f64 = 0.01;

/// Default tessellation tolerance for export (decision D2).
/// Maximum deviation of the flattened polyline from the true curve.
/// This is ten times tighter than the 0.1 mm export budget in NFR-015.
pub const EXPORT_TESSELLATION_MM: f64 = 0.01;

/// Tessellation tolerance used internally when computing arc length.
/// Tighter than the export tolerance so that length errors stay under
/// `LENGTH_ACCURACY_MM`.
pub const FLATTEN_MM: f64 = 0.001;

/// Minimum number of sample points per curve used in test oracles.
pub const ORACLE_SAMPLES: usize = 100_000;
