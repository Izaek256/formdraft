//! Fixed demo metadata for the walking skeleton (D10). Throwaway.

use pattern_core::measure::{
    DefinitionVersion, MetadataError, ProfileId, ProfileLabel, ProfileMetadata, ProtocolVersion,
};

/// The demo profile id (D10).
pub const DEMO_ID: &str = "sk01-demo";
/// The demo profile label (D10).
pub const DEMO_LABEL: &str = "Demo profile";
/// The demo definition and protocol version (D10).
pub const DEMO_VERSION: &str = "sk01";
/// The demo updated-at stamp (D10).
pub const DEMO_UPDATED_AT: i64 = 0;

/// Build the demo metadata. Returns an error rather than panicking if the
/// hard-coded values were ever invalid.
pub fn metadata() -> Result<ProfileMetadata, MetadataError> {
    Ok(ProfileMetadata::new(
        ProfileId::new(DEMO_ID)?,
        ProfileLabel::new(DEMO_LABEL)?,
        DefinitionVersion::new(DEMO_VERSION)?,
        ProtocolVersion::new(DEMO_VERSION)?,
        DEMO_UPDATED_AT,
    ))
}
