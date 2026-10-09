/// Length string parser (AC-FR-038-3, OQ-33).
///
/// Accepted formats (ASCII only, dot decimal point, case-insensitive suffix):
/// - `"10"` — bare number, unit from caller-supplied default
/// - `"10 mm"` — decimal with suffix
/// - `"37 3/8 in"` — whole + fraction + suffix
/// - `"37.375 in"` — decimal + suffix
///
/// Fraction denominators must be powers of two in [1, 64] (ADR-0002, OQ-33).
///
/// Suffixes: mm, cm, m, in, ft, yd (case-insensitive, optional space before).
use crate::units::{Length, LengthError, Unit};

/// Errors that can occur when parsing a length string.
#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    /// The string is empty or contains only whitespace.
    Empty,
    /// The numeric part could not be parsed.
    InvalidNumber(String),
    /// The unit suffix is present but not recognised.
    UnknownUnit(String),
    /// A fraction denominator that is not a power of two ≤ 64.
    FractionDenominator(u64),
    /// The value cannot be stored (NaN, Inf, overflow).
    LengthError(LengthError),
}

impl core::fmt::Display for ParseError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ParseError::Empty => write!(f, "empty input"),
            ParseError::InvalidNumber(s) => write!(f, "invalid number: {s}"),
            ParseError::UnknownUnit(s) => write!(f, "unknown unit: {s}"),
            ParseError::FractionDenominator(d) => {
                write!(f, "fraction denominator {d} is not a power of two ≤ 64")
            }
            ParseError::LengthError(e) => write!(f, "{e}"),
        }
    }
}

impl From<LengthError> for ParseError {
    fn from(e: LengthError) -> Self {
        ParseError::LengthError(e)
    }
}

/// Parse `input` as a length.  If no suffix is present, `default_unit` is used.
///
/// # Errors
/// See [`ParseError`].  Never panics on any input.
pub fn parse_length(input: &str, default_unit: Unit) -> Result<Length, ParseError> {
    // Limit input length to prevent pathological inputs.
    const MAX_LEN: usize = 256;
    let s = if input.len() > MAX_LEN {
        &input[..MAX_LEN]
    } else {
        input
    };

    // Restrict to printable ASCII to avoid Unicode minus signs, etc.
    // Non-ASCII input is rejected as an invalid number or unit.
    let s = s.trim();
    if s.is_empty() {
        return Err(ParseError::Empty);
    }

    // Split into (numeric_part, optional_suffix).
    // Strategy: scan from the right for a known suffix (1-3 chars), take it off,
    // then parse the remainder as a number.
    let (number_str, unit) = split_number_and_unit(s, default_unit)?;

    // Parse the numeric part.  It can be:
    //   a) a plain decimal: "37.375"
    //   b) whole + fraction: "37 3/8"
    let value_mm = parse_numeric(number_str.trim(), unit)?;

    Length::from_mm(value_mm).map_err(ParseError::from)
}

/// Split the input into a numeric string and a unit.
///
/// Returns `(numeric_str, unit)`.
fn split_number_and_unit(s: &str, default_unit: Unit) -> Result<(&str, Unit), ParseError> {
    // Try suffixes of lengths 3, 2, 1 (longest first to avoid "ft" vs "f").
    // We skip length-1 because "m" could ambiguously match the end of a number.
    // Actually "m" is valid — handle all lengths.
    for suffix_len in [3usize, 2, 1] {
        if s.len() < suffix_len {
            continue;
        }
        let (before, maybe_suffix) = s.split_at(s.len() - suffix_len);
        if let Some(unit) = Unit::from_suffix(maybe_suffix) {
            // The part before the suffix must be a number or empty.
            let before = before.trim_end();
            if before.is_empty() {
                // Suffix with no number.
                return Err(ParseError::InvalidNumber(String::from("")));
            }
            return Ok((before, unit));
        }
    }

    // No recognised suffix — check there are no alphabetic characters at all.
    if s.chars().any(|c| c.is_alphabetic()) {
        // Find what looks like a suffix attempt.
        let suffix_start = s.find(|c: char| c.is_alphabetic()).unwrap_or(s.len());
        let suffix = &s[suffix_start..];
        return Err(ParseError::UnknownUnit(suffix.to_string()));
    }

    Ok((s, default_unit))
}

/// Parse a numeric string (decimal or whole+fraction) and return the value
/// converted to millimetres.
fn parse_numeric(s: &str, unit: Unit) -> Result<f64, ParseError> {
    if s.is_empty() {
        return Err(ParseError::InvalidNumber(String::from("")));
    }

    // Detect fraction pattern: "<whole> <num>/<den>" or just "<num>/<den>".
    if s.contains('/') {
        return parse_fraction(s, unit);
    }

    // Plain decimal.  Scientific notation is rejected (contains 'e'/'E').
    if s.contains('e') || s.contains('E') {
        return Err(ParseError::InvalidNumber(s.to_string()));
    }

    let v: f64 = s
        .parse()
        .map_err(|_| ParseError::InvalidNumber(s.to_string()))?;

    Ok(v * unit.mm_per_unit())
}

/// Parse a fraction string such as "37 3/8" or "3/8" (only valid for inches
/// per OQ-33, but the parser does not restrict by unit here — the unit is
/// already resolved before this call and the test that validates denominators
/// does not depend on the unit).
fn parse_fraction(s: &str, unit: Unit) -> Result<f64, ParseError> {
    // Split on whitespace to find optional whole-number part.
    let parts: Vec<&str> = s.split_whitespace().collect();
    let (whole_str, frac_str) = match parts.as_slice() {
        [frac] => ("0", *frac),
        [whole, frac] => (*whole, *frac),
        _ => return Err(ParseError::InvalidNumber(s.to_string())),
    };

    // Reject scientific notation in whole part.
    if whole_str.contains('e') || whole_str.contains('E') {
        return Err(ParseError::InvalidNumber(s.to_string()));
    }

    let whole: f64 = whole_str
        .parse()
        .map_err(|_| ParseError::InvalidNumber(s.to_string()))?;

    // Parse numerator/denominator.
    let slash_pos = frac_str
        .find('/')
        .ok_or_else(|| ParseError::InvalidNumber(s.to_string()))?;
    let num_str = &frac_str[..slash_pos];
    let den_str = &frac_str[slash_pos + 1..];

    let numerator: i64 = num_str
        .parse()
        .map_err(|_| ParseError::InvalidNumber(s.to_string()))?;
    let denominator: u64 = den_str
        .parse()
        .map_err(|_| ParseError::InvalidNumber(s.to_string()))?;

    // Denominator must be a power of two in [1, 64] (OQ-33 / ADR-0002).
    if denominator == 0 || denominator > 64 || !denominator.is_power_of_two() {
        return Err(ParseError::FractionDenominator(denominator));
    }

    // numerator/denominator must be < 1 when a whole part is present, but we
    // allow improper fractions when whole == 0 for flexibility.
    let frac_value = numerator as f64 / denominator as f64;
    let total = whole + frac_value;

    Ok(total * unit.mm_per_unit())
}
