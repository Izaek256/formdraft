use crate::units::Length;

/// A measurement value with its source tag.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Measurement {
    pub(crate) length: Length,
    pub(crate) tag: crate::measure::SourceTag,
}

impl Measurement {
    /// Create a new measurement with the given length and source tag.
    pub fn new(length: Length, tag: crate::measure::SourceTag) -> Self {
        Self { length, tag }
    }

    /// The length value.
    pub fn length(&self) -> Length {
        self.length
    }

    /// The source tag.
    pub fn tag(&self) -> crate::measure::SourceTag {
        self.tag
    }
}

/// A profile with all fields optional (what the user is typing).
///
/// Fields are public so the UI can update them directly.
///
/// `Debug` shows field presence only, never values or the label (D7).
#[derive(Default, Clone)]
pub struct ProfileDraft {
    pub waist: Option<Measurement>,
    pub hips: Option<Measurement>,
    pub waist_to_hip: Option<Measurement>,
    pub skirt_length: Option<Measurement>,
    pub bust: Option<Measurement>,
}

impl core::fmt::Debug for ProfileDraft {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ProfileDraft")
            .field("waist", &self.waist.is_some())
            .field("hips", &self.hips.is_some())
            .field("waist_to_hip", &self.waist_to_hip.is_some())
            .field("skirt_length", &self.skirt_length.is_some())
            .field("bust", &self.bust.is_some())
            .finish()
    }
}

impl ProfileDraft {
    /// Create an empty draft.
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if all required fields are present.
    pub fn is_complete(&self) -> bool {
        self.waist.is_some()
            && self.hips.is_some()
            && self.waist_to_hip.is_some()
            && self.skirt_length.is_some()
    }

    /// Get the missing required field names.
    pub fn missing_fields(&self) -> Vec<&'static str> {
        let mut missing = Vec::new();
        if self.waist.is_none() {
            missing.push("waist");
        }
        if self.hips.is_none() {
            missing.push("hips");
        }
        if self.waist_to_hip.is_none() {
            missing.push("waist_to_hip");
        }
        if self.skirt_length.is_none() {
            missing.push("skirt_length");
        }
        missing
    }
}

/// A complete, validated profile.
///
/// Construction is private; use `ProfileDraft::validate()` to get a `Profile`.
///
/// `Debug` never shows measurement values or the label (D7).
#[derive(Clone)]
pub struct Profile {
    pub(crate) waist: Measurement,
    pub(crate) hips: Measurement,
    pub(crate) waist_to_hip: Measurement,
    pub(crate) skirt_length: Measurement,
    pub(crate) bust: Option<Measurement>,
    pub(crate) metadata: crate::measure::ProfileMetadata,
}

impl core::fmt::Debug for Profile {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Profile")
            .field("bust", &self.bust.is_some())
            .finish_non_exhaustive()
    }
}

impl Profile {
    /// Get the waist measurement.
    pub fn waist(&self) -> &Measurement {
        &self.waist
    }

    /// Get the hips measurement.
    pub fn hips(&self) -> &Measurement {
        &self.hips
    }

    /// Get the waist-to-hip measurement.
    pub fn waist_to_hip(&self) -> &Measurement {
        &self.waist_to_hip
    }

    /// Get the skirt length measurement.
    pub fn skirt_length(&self) -> &Measurement {
        &self.skirt_length
    }

    /// Get the bust measurement (optional).
    pub fn bust(&self) -> Option<&Measurement> {
        self.bust.as_ref()
    }

    /// Get the metadata.
    pub fn metadata(&self) -> &crate::measure::ProfileMetadata {
        &self.metadata
    }
}

/// Errors that can occur when building a Profile from a ProfileDraft.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProfileError {
    MissingField(&'static str),
    InvalidSourceTag { field: &'static str, tag: String },
    ZeroOrNegative(&'static str),
}

impl core::fmt::Display for ProfileError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ProfileError::MissingField(field) => write!(f, "missing required field: {}", field),
            ProfileError::InvalidSourceTag { field, tag } => {
                write!(f, "invalid source tag '{}' for field '{}'", tag, field)
            }
            ProfileError::ZeroOrNegative(field) => write!(f, "field '{}' must be positive", field),
        }
    }
}

impl std::error::Error for ProfileError {}
