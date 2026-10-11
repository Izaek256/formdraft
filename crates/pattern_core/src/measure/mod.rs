//! Measurement profile and validation (S03).
//!
//! # Types
//! - [`ProfileDraft`] — user-entered data, every field optional.
//! - [`Profile`] — complete, validated profile.
//! - [`Measurement`] — a [`Length`] with a [`SourceTag`].
//! - [`SourceTag`] — where the value came from (measured, chart-derived, estimated).
//! - [`Limits`] — per-field thresholds, either `Unset` or `Set`.
//! - [`ValidationReport`] — errors and warnings from validation.
//!
//! # Validation (AC-FR-019-1..6, INV-12, INV-15)
//! - Required fields: waist, hips, waist_to_hip, skirt_length.
//! - Optional field: bust (used only for chart lookup later).
//! - Each value carries a [`SourceTag`]. Allowed tags per SRS 5.3:
//!   - waist, hips, skirt_length, bust: `Measured` only.
//!   - waist_to_hip: `Measured`, `ChartDerived`, `Estimated`.
//! - Errors: missing field, zero/negative value, tag not allowed, below hard min,
//!   above hard max, cross-field violation (rule ID).
//! - Warnings: below warn-below, above warn-above, `LimitsNotConfigured`.
//! - With `Unset` limits: every measurement gets `LimitsNotConfigured` warning and
//!   `limits_complete = false`. No limit is ever invented.
//!
//! # Limits data (ADR-0009, D5)
//! Rule IDs: ALD4-THR-001 (waist), 002 (hips), 003 (waist_to_hip),
//! 004 (skirt_length), 005 (bust), 006 (hips_vs_waist).
//!
//! # Metadata (D6)
//! - `id`: validated string (core never generates one).
//! - `label`: non-empty, ≤100 chars.
//! - `definition_version`, `protocol_version`: ASCII letters, digits, dot, dash,
//!   underscore; ≤64 chars.
//! - `updated_at`: Unix seconds UTC, supplied by caller (core reads no clock).
//!
//! # Privacy (D7)
//! `Debug` and `Display` for `Profile` and `ProfileDraft` never show measurement
//! values or the label.

pub mod limits;
pub mod metadata;
pub mod profile;
pub mod source_tag;
pub mod validation;

pub use limits::{CrossLimit, LimitError, LimitSet, LimitWarning, Limits};
pub use metadata::{
    DefinitionVersion, MetadataError, ProfileId, ProfileLabel, ProfileMetadata, ProtocolVersion,
};
pub use profile::{Measurement, Profile, ProfileDraft, ProfileError};
pub use source_tag::{SourceTag, TagError};
pub use validation::{
    validate_draft, ValidationCode, ValidationError, ValidationReport, ValidationWarning,
};
