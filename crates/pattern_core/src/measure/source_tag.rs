/// Source tag for a measurement value (SRS 5.3, D8).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SourceTag {
    /// Value was directly measured from the person.
    Measured,
    /// Value was derived from a standard size chart.
    ChartDerived,
    /// Value was estimated (e.g., default waist-to-hip).
    Estimated,
}

impl SourceTag {
    /// All tags allowed for waist_to_hip.
    pub const WAIST_TO_HIP_ALLOWED: &[SourceTag] = &[
        SourceTag::Measured,
        SourceTag::ChartDerived,
        SourceTag::Estimated,
    ];

    /// Tags allowed for waist, hips, skirt_length, bust.
    pub const BODY_MEASUREMENT_ALLOWED: &[SourceTag] = &[SourceTag::Measured];

    /// Check if this tag is allowed for the given field.
    pub fn is_allowed_for(self, field: &str) -> bool {
        match field {
            "waist_to_hip" => Self::WAIST_TO_HIP_ALLOWED.contains(&self),
            "waist" | "hips" | "skirt_length" | "bust" => {
                Self::BODY_MEASUREMENT_ALLOWED.contains(&self)
            }
            _ => false,
        }
    }
}

impl core::fmt::Display for SourceTag {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            SourceTag::Measured => write!(f, "measured"),
            SourceTag::ChartDerived => write!(f, "chart-derived"),
            SourceTag::Estimated => write!(f, "estimated"),
        }
    }
}

impl core::str::FromStr for SourceTag {
    type Err = TagError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "measured" => Ok(SourceTag::Measured),
            "chart-derived" | "chart_derived" => Ok(SourceTag::ChartDerived),
            "estimated" => Ok(SourceTag::Estimated),
            _ => Err(TagError::UnknownTag(s.to_string())),
        }
    }
}

/// Errors from source tag parsing/validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TagError {
    UnknownTag(String),
    NotAllowed { field: String, tag: String },
}

impl core::fmt::Display for TagError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            TagError::UnknownTag(s) => write!(f, "unknown source tag: {}", s),
            TagError::NotAllowed { field, tag } => {
                write!(f, "source tag '{}' not allowed for field '{}'", tag, field)
            }
        }
    }
}

impl std::error::Error for TagError {}
