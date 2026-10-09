/// Unit service: the only place that parses, converts, or formats a length
/// (ADR-0007, NFR-028, FR-038).
///
/// # Coordinate system
/// Stored form: exact integer nanometres in an `i64`.  One millimetre =
/// 1,000,000 nm.  Computation form: `f64` millimetres, created at the boundary
/// from the stored form.  Results return to stored form only at save/display
/// time (ADR-0002).
///
/// # Modules
/// - `unit`       — [`Unit`] enum and [`QuantityClass`] / [`Scope`]
/// - `length`     — [`Length`] type (exact nm store)
/// - `parse`      — [`parse_length`] and [`ParseError`]
/// - `format`     — [`format_length`], [`FormatOptions`], [`InchMode`]
/// - `preference` — [`PreferenceStore`] and [`UnitPreference`]
mod format;
mod length;
mod parse;
mod preference;
mod unit;

pub use format::{format_length, FormatOptions, InchMode};
pub use length::{Length, LengthError};
pub use parse::{parse_length, ParseError};
pub use preference::{PreferenceStore, UnitPreference};
pub use unit::{QuantityClass, Scope, Unit};

/// The unit service.
///
/// A single `UnitService` instance owns the preference store and exposes
/// parse/format operations.  It is intentionally free of I/O, randomness and
/// global state.
#[derive(Debug, Default)]
pub struct UnitService {
    prefs: PreferenceStore,
}

impl UnitService {
    /// Create a new service with an empty preference store.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set (or replace) a unit preference.
    pub fn set_preference(&mut self, pref: UnitPreference) {
        self.prefs.set(pref);
    }

    /// Resolve the display unit for `class` in `scope`.
    pub fn resolve(&self, class: QuantityClass, scope: Scope) -> Unit {
        self.prefs.resolve(class, scope)
    }

    /// Parse `input` as a [`Length`].  `default_unit` is used when `input` has
    /// no suffix.
    ///
    /// # Errors
    /// Returns a [`ParseError`] on invalid input.  Never panics.
    pub fn parse(&self, input: &str, default_unit: Unit) -> Result<Length, ParseError> {
        parse_length(input, default_unit)
    }

    /// Format `length` according to `opts`.
    ///
    /// Returns a string such as `"25.5 mm"` or `"1 8/16 in"`.
    pub fn format(&self, length: Length, opts: &FormatOptions) -> String {
        format_length(length, opts)
    }
}
