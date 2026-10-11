use crate::units::Length;

/// Limit set for a single measurement (D4).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LimitSet {
    /// Limits are not configured. No validation is performed against these.
    Unset,
    /// Limits are configured.
    Set {
        hard_min: Length,
        hard_max: Length,
        warn_below: Length,
        warn_above: Length,
    },
}

impl LimitSet {
    /// Create a new `Set` limit set.
    pub fn set(hard_min: Length, hard_max: Length, warn_below: Length, warn_above: Length) -> Self {
        Self::Set {
            hard_min,
            hard_max,
            warn_below,
            warn_above,
        }
    }

    /// Check if limits are set.
    pub fn is_set(&self) -> bool {
        matches!(self, Self::Set { .. })
    }

    /// Get the hard minimum if set.
    pub fn hard_min(&self) -> Option<Length> {
        match self {
            Self::Set { hard_min, .. } => Some(*hard_min),
            Self::Unset => None,
        }
    }

    /// Get the hard maximum if set.
    pub fn hard_max(&self) -> Option<Length> {
        match self {
            Self::Set { hard_max, .. } => Some(*hard_max),
            Self::Unset => None,
        }
    }

    /// Get the warn-below if set.
    pub fn warn_below(&self) -> Option<Length> {
        match self {
            Self::Set { warn_below, .. } => Some(*warn_below),
            Self::Unset => None,
        }
    }

    /// Get the warn-above if set.
    pub fn warn_above(&self) -> Option<Length> {
        match self {
            Self::Set { warn_above, .. } => Some(*warn_above),
            Self::Unset => None,
        }
    }
}

/// Cross-field limit for hips vs waist (D5, rule ALD4-THR-006).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CrossLimit {
    /// Cross-field limit not configured.
    Unset,
    /// Cross-field limit configured.
    Set { min_diff: Length, max_diff: Length },
}

impl CrossLimit {
    /// Create a new `Set` cross limit.
    pub fn set(min_diff: Length, max_diff: Length) -> Self {
        Self::Set { min_diff, max_diff }
    }

    /// Check if the cross limit is set.
    pub fn is_set(&self) -> bool {
        matches!(self, Self::Set { .. })
    }

    /// Get the minimum difference if set.
    pub fn min_diff(&self) -> Option<Length> {
        match self {
            Self::Set { min_diff, .. } => Some(*min_diff),
            Self::Unset => None,
        }
    }

    /// Get the maximum difference if set.
    pub fn max_diff(&self) -> Option<Length> {
        match self {
            Self::Set { max_diff, .. } => Some(*max_diff),
            Self::Unset => None,
        }
    }
}

/// Complete limits for a profile (D4).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Limits {
    pub waist: LimitSet,
    pub hips: LimitSet,
    pub waist_to_hip: LimitSet,
    pub skirt_length: LimitSet,
    pub bust: LimitSet,
    pub hips_vs_waist: CrossLimit,
}

impl Default for Limits {
    fn default() -> Self {
        Self {
            waist: LimitSet::Unset,
            hips: LimitSet::Unset,
            waist_to_hip: LimitSet::Unset,
            skirt_length: LimitSet::Unset,
            bust: LimitSet::Unset,
            hips_vs_waist: CrossLimit::Unset,
        }
    }
}

impl Limits {
    /// Create new limits with all entries unset.
    pub fn new_unset() -> Self {
        Self::default()
    }

    /// Check if all limits are configured.
    pub fn all_set(&self) -> bool {
        self.waist.is_set()
            && self.hips.is_set()
            && self.waist_to_hip.is_set()
            && self.skirt_length.is_set()
            && self.bust.is_set()
            && self.hips_vs_waist.is_set()
    }
}

/// Errors from limit validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LimitError {
    BelowHardMin {
        field: &'static str,
        rule_id: &'static str,
    },
    AboveHardMax {
        field: &'static str,
        rule_id: &'static str,
    },
    HipsWaistDiffBelowMin {
        rule_id: &'static str,
    },
    HipsWaistDiffAboveMax {
        rule_id: &'static str,
    },
}

impl core::fmt::Display for LimitError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            LimitError::BelowHardMin { field, rule_id } => {
                write!(f, "{} below hard minimum (rule {})", field, rule_id)
            }
            LimitError::AboveHardMax { field, rule_id } => {
                write!(f, "{} above hard maximum (rule {})", field, rule_id)
            }
            LimitError::HipsWaistDiffBelowMin { rule_id } => {
                write!(f, "hips-waist difference below minimum (rule {})", rule_id)
            }
            LimitError::HipsWaistDiffAboveMax { rule_id } => {
                write!(f, "hips-waist difference above maximum (rule {})", rule_id)
            }
        }
    }
}

impl std::error::Error for LimitError {}

/// Warnings from limit validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LimitWarning {
    BelowWarnBelow {
        field: &'static str,
        rule_id: &'static str,
    },
    AboveWarnAbove {
        field: &'static str,
        rule_id: &'static str,
    },
    LimitsNotConfigured {
        field: &'static str,
        rule_id: &'static str,
    },
}

impl core::fmt::Display for LimitWarning {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            LimitWarning::BelowWarnBelow { field, rule_id } => {
                write!(f, "{} below warning threshold (rule {})", field, rule_id)
            }
            LimitWarning::AboveWarnAbove { field, rule_id } => {
                write!(f, "{} above warning threshold (rule {})", field, rule_id)
            }
            LimitWarning::LimitsNotConfigured { field, rule_id } => {
                write!(f, "limits not configured for {} (rule {})", field, rule_id)
            }
        }
    }
}

impl std::error::Error for LimitWarning {}
