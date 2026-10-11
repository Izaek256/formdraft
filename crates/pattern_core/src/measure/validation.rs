use crate::measure::{Limits, Profile, ProfileDraft, ProfileError, ProfileMetadata, SourceTag};
use crate::units::Length;

/// Complete validation result.
#[derive(Debug, Clone)]
pub struct ValidationReport {
    pub errors: Vec<ValidationError>,
    pub warnings: Vec<ValidationWarning>,
    pub limits_complete: bool,
}

impl ValidationReport {
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
            warnings: Vec::new(),
            limits_complete: false,
        }
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn add_error(&mut self, error: ValidationError) {
        self.errors.push(error);
    }

    pub fn add_warning(&mut self, warning: ValidationWarning) {
        self.warnings.push(warning);
    }
}

impl Default for ValidationReport {
    fn default() -> Self {
        Self::new()
    }
}

/// Validation error with stable code and message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationError {
    pub code: ValidationCode,
    pub message: String,
}

impl ValidationError {
    pub fn new(code: ValidationCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

/// Validation warning with stable code and message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationWarning {
    pub code: ValidationCode,
    pub message: String,
}

impl ValidationWarning {
    pub fn new(code: ValidationCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

/// Stable validation codes (FR-036 style).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ValidationCode {
    // Required field missing
    MissingField,
    // Source tag not allowed for this field
    InvalidSourceTag,
    // Value is zero or negative
    ZeroOrNegative,
    // Below hard minimum
    BelowHardMin,
    // Above hard maximum
    AboveHardMax,
    // Hips-waist difference below minimum
    HipsWaistDiffBelowMin,
    // Hips-waist difference above maximum
    HipsWaistDiffAboveMax,
    // Below warn-below
    BelowWarnBelow,
    // Above warn-above
    AboveWarnAbove,
    // Limits not configured
    LimitsNotConfigured,
    // Metadata validation errors
    EmptyId,
    IdTooLong,
    InvalidIdChars,
    EmptyLabel,
    LabelTooLong,
    EmptyVersion,
    VersionTooLong,
    InvalidVersionChars,
}

impl ValidationCode {
    pub fn as_str(&self) -> &'static str {
        match self {
            ValidationCode::MissingField => "MISSING_FIELD",
            ValidationCode::InvalidSourceTag => "INVALID_SOURCE_TAG",
            ValidationCode::ZeroOrNegative => "ZERO_OR_NEGATIVE",
            ValidationCode::BelowHardMin => "BELOW_HARD_MIN",
            ValidationCode::AboveHardMax => "ABOVE_HARD_MAX",
            ValidationCode::HipsWaistDiffBelowMin => "HIPS_WAIST_DIFF_BELOW_MIN",
            ValidationCode::HipsWaistDiffAboveMax => "HIPS_WAIST_DIFF_ABOVE_MAX",
            ValidationCode::BelowWarnBelow => "BELOW_WARN_BELOW",
            ValidationCode::AboveWarnAbove => "ABOVE_WARN_ABOVE",
            ValidationCode::LimitsNotConfigured => "LIMITS_NOT_CONFIGURED",
            ValidationCode::EmptyId => "EMPTY_ID",
            ValidationCode::IdTooLong => "ID_TOO_LONG",
            ValidationCode::InvalidIdChars => "INVALID_ID_CHARS",
            ValidationCode::EmptyLabel => "EMPTY_LABEL",
            ValidationCode::LabelTooLong => "LABEL_TOO_LONG",
            ValidationCode::EmptyVersion => "EMPTY_VERSION",
            ValidationCode::VersionTooLong => "VERSION_TOO_LONG",
            ValidationCode::InvalidVersionChars => "INVALID_VERSION_CHARS",
        }
    }
}

impl core::fmt::Display for ValidationCode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Validate a `ProfileDraft` against limits and metadata.
///
/// On success returns the completed [`Profile`] together with the
/// [`ValidationReport`], which may still carry warnings (for example
/// `LimitsNotConfigured` for unset limits).  On failure returns the report
/// (with a description of every blocking problem: missing field, invalid
/// source tag, zero or negative value) together with the first blocking
/// [`ProfileError`].  A profile is only returned when all four mandatory
/// fields are present, tagged correctly and positive.
pub fn validate_draft(
    draft: ProfileDraft,
    limits: &Limits,
    metadata: ProfileMetadata,
) -> Result<(Profile, ValidationReport), (ValidationReport, ProfileError)> {
    let mut report = ValidationReport::new();

    // Field-level checks.  Any blocking error ends validation here because a
    // Profile cannot be built until the inputs are sound.
    let profile = match validate_profile_draft(draft, metadata) {
        Ok(p) => p,
        Err(e) => {
            report.add_error(error_for(&e));
            return Err((report, e));
        }
    };

    // Limits completeness.
    report.limits_complete = limits.all_set();
    if !report.limits_complete {
        add_unset_warnings(limits, &mut report);
    }

    // Limit checks against the completed profile.
    check_limits(&profile, limits, &mut report);

    Ok((profile, report))
}

/// Build a `Profile` from a draft, checking required fields, tags and sign.
fn validate_profile_draft(
    draft: ProfileDraft,
    metadata: ProfileMetadata,
) -> Result<Profile, ProfileError> {
    let waist = draft.waist.ok_or(ProfileError::MissingField("waist"))?;
    let hips = draft.hips.ok_or(ProfileError::MissingField("hips"))?;
    let waist_to_hip = draft
        .waist_to_hip
        .ok_or(ProfileError::MissingField("waist_to_hip"))?;
    let skirt_length = draft
        .skirt_length
        .ok_or(ProfileError::MissingField("skirt_length"))?;

    validate_source_tag("waist", waist.tag())?;
    validate_source_tag("hips", hips.tag())?;
    validate_source_tag("waist_to_hip", waist_to_hip.tag())?;
    validate_source_tag("skirt_length", skirt_length.tag())?;

    check_positive("waist", waist.length())?;
    check_positive("hips", hips.length())?;
    check_positive("waist_to_hip", waist_to_hip.length())?;
    check_positive("skirt_length", skirt_length.length())?;

    let bust = match draft.bust {
        Some(b) => {
            validate_source_tag("bust", b.tag())?;
            check_positive("bust", b.length())?;
            Some(b)
        }
        None => None,
    };

    Ok(Profile {
        waist,
        hips,
        waist_to_hip,
        skirt_length,
        bust,
        metadata,
    })
}

/// Convert a blocking `ProfileError` into a report error.
fn error_for(e: &ProfileError) -> ValidationError {
    match e {
        ProfileError::MissingField(field) => ValidationError::new(
            ValidationCode::MissingField,
            format!("missing required field: {}", field),
        ),
        ProfileError::InvalidSourceTag { field, tag } => ValidationError::new(
            ValidationCode::InvalidSourceTag,
            format!("invalid source tag '{}' for field '{}'", tag, field),
        ),
        ProfileError::ZeroOrNegative(field) => ValidationError::new(
            ValidationCode::ZeroOrNegative,
            format!("field '{}' must be positive", field),
        ),
    }
}

/// Add a `LimitsNotConfigured` warning for every unset limit.
fn add_unset_warnings(limits: &Limits, report: &mut ValidationReport) {
    let entries = [
        (limits.waist.is_set(), "waist", "ALD4-THR-001"),
        (limits.hips.is_set(), "hips", "ALD4-THR-002"),
        (limits.waist_to_hip.is_set(), "waist_to_hip", "ALD4-THR-003"),
        (limits.skirt_length.is_set(), "skirt_length", "ALD4-THR-004"),
        (limits.bust.is_set(), "bust", "ALD4-THR-005"),
        (
            limits.hips_vs_waist.is_set(),
            "hips_vs_waist",
            "ALD4-THR-006",
        ),
    ];
    for (is_set, field, rule_id) in entries {
        if !is_set {
            report.add_warning(ValidationWarning::new(
                ValidationCode::LimitsNotConfigured,
                format!("limits not configured for {} (rule {})", field, rule_id),
            ));
        }
    }
}

/// Validate a source tag for a field.
fn validate_source_tag(field: &'static str, tag: SourceTag) -> Result<(), ProfileError> {
    if !tag.is_allowed_for(field) {
        return Err(ProfileError::InvalidSourceTag {
            field,
            tag: tag.to_string(),
        });
    }
    Ok(())
}

/// Check that a length is positive (non-zero).
fn check_positive(field: &'static str, length: Length) -> Result<(), ProfileError> {
    if length.nanometres() <= 0 {
        return Err(ProfileError::ZeroOrNegative(field));
    }
    Ok(())
}

/// Check limits against a complete profile.
fn check_limits(profile: &Profile, limits: &Limits, report: &mut ValidationReport) {
    // Rule IDs per D5
    const RULE_WAIST: &str = "ALD4-THR-001";
    const RULE_HIPS: &str = "ALD4-THR-002";
    const RULE_WAIST_TO_HIP: &str = "ALD4-THR-003";
    const RULE_SKIRT_LENGTH: &str = "ALD4-THR-004";
    const RULE_BUST: &str = "ALD4-THR-005";
    const RULE_HIPS_VS_WAIST: &str = "ALD4-THR-006";

    // Helper to check a single limit set
    fn check_limit(
        field: &str,
        value: Length,
        limit_set: &crate::measure::LimitSet,
        rule_id: &str,
        report: &mut ValidationReport,
    ) {
        match limit_set {
            crate::measure::LimitSet::Unset => {
                // LimitsNotConfigured warning already added in validate_draft
            }
            crate::measure::LimitSet::Set {
                hard_min,
                hard_max,
                warn_below,
                warn_above,
            } => {
                if value < *hard_min {
                    report.add_error(ValidationError::new(
                        ValidationCode::BelowHardMin,
                        format!("{} below hard minimum (rule {})", field, rule_id),
                    ));
                } else if value < *warn_below {
                    report.add_warning(ValidationWarning::new(
                        ValidationCode::BelowWarnBelow,
                        format!("{} below warning threshold (rule {})", field, rule_id),
                    ));
                } else if value > *hard_max {
                    report.add_error(ValidationError::new(
                        ValidationCode::AboveHardMax,
                        format!("{} above hard maximum (rule {})", field, rule_id),
                    ));
                } else if value > *warn_above {
                    report.add_warning(ValidationWarning::new(
                        ValidationCode::AboveWarnAbove,
                        format!("{} above warning threshold (rule {})", field, rule_id),
                    ));
                }
            }
        }
    }

    check_limit(
        "waist",
        profile.waist.length(),
        &limits.waist,
        RULE_WAIST,
        report,
    );
    check_limit(
        "hips",
        profile.hips.length(),
        &limits.hips,
        RULE_HIPS,
        report,
    );
    check_limit(
        "waist_to_hip",
        profile.waist_to_hip.length(),
        &limits.waist_to_hip,
        RULE_WAIST_TO_HIP,
        report,
    );
    check_limit(
        "skirt_length",
        profile.skirt_length.length(),
        &limits.skirt_length,
        RULE_SKIRT_LENGTH,
        report,
    );
    if let Some(bust) = profile.bust.as_ref() {
        check_limit("bust", bust.length(), &limits.bust, RULE_BUST, report);
    }

    // Cross-field: hips - waist
    if let crate::measure::CrossLimit::Set { min_diff, max_diff } = limits.hips_vs_waist {
        let diff_nm = profile.hips.length().nanometres() - profile.waist.length().nanometres();
        let diff = Length::from_nanometres(diff_nm);
        if diff < min_diff {
            report.add_error(ValidationError::new(
                ValidationCode::HipsWaistDiffBelowMin,
                format!(
                    "hips-waist difference below minimum (rule {})",
                    RULE_HIPS_VS_WAIST
                ),
            ));
        } else if diff > max_diff {
            report.add_error(ValidationError::new(
                ValidationCode::HipsWaistDiffAboveMax,
                format!(
                    "hips-waist difference above maximum (rule {})",
                    RULE_HIPS_VS_WAIST
                ),
            ));
        }
    }
    // If Unset, LimitsNotConfigured warning already added
}
