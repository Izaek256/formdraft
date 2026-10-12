//! Role styles for the drawing (SRS vocabulary: stitch path, cut path, net
//! block, balance mark). Roles differ by dash pattern and weight, not by
//! colour alone (T-SK01-16). Colours and pixel sizes come from the theme.

use egui::Color32;

use crate::theme::Tokens;

/// A drawing role. These are the SRS curve and mark roles.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    /// The net block outline (construction).
    NetBlock,
    /// The stitch path, sewn at.
    Stitch,
    /// The cut path, which includes the seam allowance.
    Cut,
    /// A construction mark, such as a dart line.
    Mark,
    /// A text label.
    Label,
    /// The placeholder shape (throwaway).
    Placeholder,
    /// The straight-grain line.
    Grain,
    /// A balance mark on an edge.
    BalanceMark,
}

/// A stroke weight name, resolved to pixels by the theme.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Weight {
    /// Thin line.
    Thin,
    /// Medium line.
    Medium,
    /// Thick line.
    Thick,
}

/// A dash pattern name, resolved to pixels by the theme.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dash {
    /// A continuous line.
    Solid,
    /// A dashed line.
    Dashed,
}

/// The visual style of a role: weight and dash pattern only. Colour comes from
/// the theme.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RoleStyle {
    /// The stroke weight.
    pub weight: Weight,
    /// The dash pattern.
    pub dash: Dash,
}

/// The style for a role.
pub fn style(role: Role) -> RoleStyle {
    let (weight, dash) = match role {
        Role::NetBlock => (Weight::Thick, Dash::Solid),
        Role::Stitch => (Weight::Thin, Dash::Dashed),
        Role::Cut => (Weight::Thin, Dash::Solid),
        Role::Mark => (Weight::Medium, Dash::Dashed),
        Role::Label => (Weight::Thin, Dash::Solid),
        Role::Placeholder => (Weight::Medium, Dash::Dashed),
        Role::Grain => (Weight::Thin, Dash::Dashed),
        Role::BalanceMark => (Weight::Medium, Dash::Solid),
    };
    RoleStyle { weight, dash }
}

/// The stroke width in points for a weight.
pub fn weight_px(weight: Weight, tokens: &Tokens) -> f32 {
    match weight {
        Weight::Thin => tokens.sizes.stroke_thin,
        Weight::Medium => tokens.sizes.stroke_medium,
        Weight::Thick => tokens.sizes.stroke_thick,
    }
}

/// The colour for a role.
pub fn color(role: Role, tokens: &Tokens) -> Color32 {
    let colors = &tokens.colors;
    match role {
        Role::NetBlock => colors.text,
        Role::Stitch => colors.stitch,
        Role::Cut => colors.cut,
        Role::Mark => colors.mark,
        Role::Label => colors.label,
        Role::Placeholder => colors.placeholder,
        Role::Grain => colors.mark,
        Role::BalanceMark => colors.accent,
    }
}

/// The dash and gap lengths, in points, for a dash pattern.
pub fn dash_px(dash: Dash, tokens: &Tokens) -> (f32, f32) {
    match dash {
        Dash::Solid => (0.0, 0.0),
        Dash::Dashed => (tokens.sizes.dash_length, tokens.sizes.dash_gap),
    }
}
