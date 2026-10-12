//! The throwaway placeholder shape (D10, Stage C).
//!
//! One rectangle: hips wide, skirt length high, in millimetres, y down, with
//! its top-left corner at the world origin. Straight edges only. When any of
//! the four fields is missing or invalid, there is no rectangle and the caller
//! names the field.

pub mod demo_profile;

use pattern_core::measure::ProfileDraft;
use pattern_core::units::Length;

use crate::canvas::Bounds;

/// The label shown for the placeholder piece.
pub const PIECE_LABEL: &str = "Placeholder rectangle";

/// The always-visible notice (BR-01, BR-17).
pub const NOTICE: &str = "PLACEHOLDER SHAPE, NOT A PATTERN. Dimensions are your inputs only.";

/// The rectangle for a given width (hips) and height (skirt length).
pub fn rectangle_bounds(hips: Length, skirt_length: Length) -> Bounds {
    Bounds::new([0.0, 0.0], [hips.to_mm(), skirt_length.to_mm()])
}

/// The rectangle for a draft, or `None` when hips or skirt length is absent.
pub fn from_draft(draft: &ProfileDraft) -> Option<Bounds> {
    let hips = draft.hips.as_ref()?;
    let skirt_length = draft.skirt_length.as_ref()?;
    Some(rectangle_bounds(hips.length(), skirt_length.length()))
}
