use pattern_core::measure::{CrossLimit, LimitSet, Limits};
/// Thresholds loader for pattern_templates (S03, D2).
///
/// Reads the thresholds TOML data file compiled into the binary and converts
/// it into the pattern_core `Limits` type.
use pattern_core::units::{Length, Unit};

/// Load the thresholds from the embedded TOML file.
///
/// The file is compiled into the binary via `include_str!` — no filesystem
/// access at runtime.  Unknown keys or malformed TOML are typed errors.
pub fn load_thresholds() -> Result<Limits, ThresholdsError> {
    const TOML_STR: &str = include_str!("../../data/aldrich-mpc-4e/thresholds.toml");
    let toml: ThresholdsToml = toml::from_str(TOML_STR).map_err(ThresholdsError::TomlParse)?;

    // Helper to parse a LimitSet from the TOML
    fn parse_limit_set(toml: &LimitToml) -> Result<LimitSet, ThresholdsError> {
        let rule_id = &toml.rule_id;
        if toml.status == "UNSET" || toml.status == "Unset" {
            return Ok(LimitSet::Unset);
        }
        if toml.status == "SET" || toml.status == "Set" {
            let hard_min = parse_length(&toml.hard_min, &toml.unit, rule_id)?;
            let hard_max = parse_length(&toml.hard_max, &toml.unit, rule_id)?;
            let warn_below = parse_length(&toml.warn_below, &toml.unit, rule_id)?;
            let warn_above = parse_length(&toml.warn_above, &toml.unit, rule_id)?;
            // Validate ordering
            if hard_min.nanometres() > hard_max.nanometres() {
                return Err(ThresholdsError::InvalidOrder {
                    rule_id: rule_id.to_string(),
                    message: "hard_min > hard_max".to_string(),
                });
            }
            if warn_below.nanometres() > warn_above.nanometres() {
                return Err(ThresholdsError::InvalidOrder {
                    rule_id: rule_id.to_string(),
                    message: "warn_below > warn_above".to_string(),
                });
            }
            return Ok(LimitSet::Set {
                hard_min,
                hard_max,
                warn_below,
                warn_above,
            });
        }
        Err(ThresholdsError::InvalidStatus {
            rule_id: rule_id.to_string(),
            status: toml.status.clone(),
        })
    }

    // Helper for cross limit
    fn parse_cross_limit(toml: &CrossLimitToml) -> Result<CrossLimit, ThresholdsError> {
        let rule_id = &toml.rule_id;
        if toml.status == "UNSET" || toml.status == "Unset" {
            return Ok(CrossLimit::Unset);
        }
        if toml.status == "SET" || toml.status == "Set" {
            let min_diff = parse_length(&toml.min_diff, &toml.unit, rule_id)?;
            let max_diff = parse_length(&toml.max_diff, &toml.unit, rule_id)?;
            if min_diff.nanometres() > max_diff.nanometres() {
                return Err(ThresholdsError::InvalidOrder {
                    rule_id: rule_id.to_string(),
                    message: "min_diff > max_diff".to_string(),
                });
            }
            return Ok(CrossLimit::Set { min_diff, max_diff });
        }
        Err(ThresholdsError::InvalidStatus {
            rule_id: rule_id.to_string(),
            status: toml.status.clone(),
        })
    }

    // Parse a length string using the unit service
    fn parse_length(s: &str, unit_str: &str, rule_id: &str) -> Result<Length, ThresholdsError> {
        if s.trim().is_empty() {
            return Err(ThresholdsError::EmptyValue {
                rule_id: rule_id.to_string(),
            });
        }
        let unit =
            Unit::from_suffix(unit_str.trim()).ok_or_else(|| ThresholdsError::UnknownUnit {
                rule_id: rule_id.to_string(),
                unit: unit_str.to_string(),
            })?;
        let value = s
            .trim()
            .parse::<f64>()
            .map_err(|_| ThresholdsError::InvalidNumber {
                rule_id: rule_id.to_string(),
                value: s.to_string(),
            })?;
        Length::from_unit(value, unit).map_err(|e| ThresholdsError::LengthError {
            rule_id: rule_id.to_string(),
            message: e.to_string(),
        })
    }

    let limits = Limits {
        waist: parse_limit_set(&toml.waist)?,
        hips: parse_limit_set(&toml.hips)?,
        waist_to_hip: parse_limit_set(&toml.waist_to_hip)?,
        skirt_length: parse_limit_set(&toml.skirt_length)?,
        bust: parse_limit_set(&toml.bust)?,
        hips_vs_waist: parse_cross_limit(&toml.hips_vs_waist)?,
    };

    Ok(limits)
}

/// Errors from loading thresholds.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ThresholdsError {
    TomlParse(toml::de::Error),
    EmptyValue { rule_id: String },
    UnknownUnit { rule_id: String, unit: String },
    InvalidNumber { rule_id: String, value: String },
    LengthError { rule_id: String, message: String },
    InvalidStatus { rule_id: String, status: String },
    InvalidOrder { rule_id: String, message: String },
}

impl core::fmt::Display for ThresholdsError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ThresholdsError::TomlParse(e) => write!(f, "TOML parse error: {}", e),
            ThresholdsError::EmptyValue { rule_id } => {
                write!(f, "empty value for rule {}", rule_id)
            }
            ThresholdsError::UnknownUnit { rule_id, unit } => {
                write!(f, "unknown unit '{}' in rule {}", unit, rule_id)
            }
            ThresholdsError::InvalidNumber { rule_id, value } => {
                write!(f, "invalid number '{}' in rule {}", value, rule_id)
            }
            ThresholdsError::LengthError { rule_id, message } => {
                write!(f, "length error in rule {}: {}", rule_id, message)
            }
            ThresholdsError::InvalidStatus { rule_id, status } => {
                write!(f, "invalid status '{}' in rule {}", status, rule_id)
            }
            ThresholdsError::InvalidOrder { rule_id, message } => {
                write!(f, "invalid limit order in rule {}: {}", rule_id, message)
            }
        }
    }
}

impl std::error::Error for ThresholdsError {}

// Internal TOML structures matching the thresholds.toml file
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct ThresholdsToml {
    waist: LimitToml,
    hips: LimitToml,
    waist_to_hip: LimitToml,
    skirt_length: LimitToml,
    bust: LimitToml,
    hips_vs_waist: CrossLimitToml,
}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct LimitToml {
    status: String,
    hard_min: String,
    hard_max: String,
    warn_below: String,
    warn_above: String,
    unit: String,
    rule_id: String,
}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct CrossLimitToml {
    status: String,
    min_diff: String,
    max_diff: String,
    unit: String,
    rule_id: String,
}
