//! Profile metadata (D6).

/// Validated profile ID string.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProfileId(String);

impl ProfileId {
    /// Create a new ProfileId after validation.
    ///
    /// Rules: non-empty, at most 64 characters, ASCII letters, digits, dash,
    /// underscore only.
    pub fn new(s: &str) -> Result<Self, MetadataError> {
        if s.is_empty() {
            return Err(MetadataError::EmptyId);
        }
        if s.len() > 64 {
            return Err(MetadataError::IdTooLong);
        }
        if !s
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
        {
            return Err(MetadataError::InvalidIdChars);
        }
        Ok(Self(s.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl core::fmt::Display for ProfileId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Validated profile label string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfileLabel(String);

impl ProfileLabel {
    /// Create a new ProfileLabel after validation.
    ///
    /// Rules: non-empty, at most 100 characters.
    pub fn new(s: &str) -> Result<Self, MetadataError> {
        if s.is_empty() {
            return Err(MetadataError::EmptyLabel);
        }
        if s.len() > 100 {
            return Err(MetadataError::LabelTooLong);
        }
        Ok(Self(s.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl core::fmt::Display for ProfileLabel {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Validated definition version string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DefinitionVersion(String);

impl DefinitionVersion {
    /// Create a new DefinitionVersion after validation.
    ///
    /// Rules: ASCII letters, digits, dot, dash, underscore; at most 64 chars.
    pub fn new(s: &str) -> Result<Self, MetadataError> {
        if s.is_empty() {
            return Err(MetadataError::EmptyVersion);
        }
        if s.len() > 64 {
            return Err(MetadataError::VersionTooLong);
        }
        if !s
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '-' || c == '_')
        {
            return Err(MetadataError::InvalidVersionChars);
        }
        Ok(Self(s.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl core::fmt::Display for DefinitionVersion {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Validated protocol version string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProtocolVersion(String);

impl ProtocolVersion {
    /// Create a new ProtocolVersion after validation.
    ///
    /// Rules: ASCII letters, digits, dot, dash, underscore; at most 64 chars.
    pub fn new(s: &str) -> Result<Self, MetadataError> {
        if s.is_empty() {
            return Err(MetadataError::EmptyVersion);
        }
        if s.len() > 64 {
            return Err(MetadataError::VersionTooLong);
        }
        if !s
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '-' || c == '_')
        {
            return Err(MetadataError::InvalidVersionChars);
        }
        Ok(Self(s.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl core::fmt::Display for ProtocolVersion {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Complete profile metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfileMetadata {
    pub id: ProfileId,
    pub label: ProfileLabel,
    pub definition_version: DefinitionVersion,
    pub protocol_version: ProtocolVersion,
    /// Unix seconds since epoch, UTC. Supplied by caller; core reads no clock.
    pub updated_at: i64,
}

impl ProfileMetadata {
    pub fn new(
        id: ProfileId,
        label: ProfileLabel,
        definition_version: DefinitionVersion,
        protocol_version: ProtocolVersion,
        updated_at: i64,
    ) -> Self {
        Self {
            id,
            label,
            definition_version,
            protocol_version,
            updated_at,
        }
    }
}

/// Errors from metadata validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MetadataError {
    EmptyId,
    IdTooLong,
    InvalidIdChars,
    EmptyLabel,
    LabelTooLong,
    EmptyVersion,
    VersionTooLong,
    InvalidVersionChars,
}

impl core::fmt::Display for MetadataError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            MetadataError::EmptyId => write!(f, "profile id cannot be empty"),
            MetadataError::IdTooLong => write!(f, "profile id exceeds 64 characters"),
            MetadataError::InvalidIdChars => {
                write!(f, "profile id contains invalid characters")
            }
            MetadataError::EmptyLabel => write!(f, "profile label cannot be empty"),
            MetadataError::LabelTooLong => write!(f, "profile label exceeds 100 characters"),
            MetadataError::EmptyVersion => write!(f, "version string cannot be empty"),
            MetadataError::VersionTooLong => write!(f, "version string exceeds 64 characters"),
            MetadataError::InvalidVersionChars => {
                write!(f, "version string contains invalid characters")
            }
        }
    }
}

impl std::error::Error for MetadataError {}
