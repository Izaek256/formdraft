//! The properties panel model and view (Stage C).
//!
//! The model is pure and unit tested (T-SK01-01..07, 09). Parsing and display
//! go only through the S01 unit service (ADR-0007).

use pattern_core::measure::{Measurement, ProfileDraft, SourceTag};
use pattern_core::units::{parse_length, Unit};

use crate::theme::Tokens;

/// The six units offered by both selectors (FR-038).
pub const UNITS: [Unit; 6] = [Unit::Mm, Unit::Cm, Unit::M, Unit::In, Unit::Ft, Unit::Yd];

/// The inch fraction denominators offered (OQ-33).
pub const INCH_DENOMINATORS: [u8; 6] = [2, 4, 8, 16, 32, 64];

/// One of the four body measurement fields.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldId {
    /// Waist circumference.
    Waist,
    /// Hip circumference.
    Hips,
    /// Waist-to-hip depth.
    WaistToHip,
    /// Skirt length.
    SkirtLength,
}

impl FieldId {
    /// All fields, in reading order.
    pub const ALL: [FieldId; 4] = [
        FieldId::Waist,
        FieldId::Hips,
        FieldId::WaistToHip,
        FieldId::SkirtLength,
    ];

    /// The stable key used by `pattern_core` validation.
    pub fn key(self) -> &'static str {
        match self {
            FieldId::Waist => "waist",
            FieldId::Hips => "hips",
            FieldId::WaistToHip => "waist_to_hip",
            FieldId::SkirtLength => "skirt_length",
        }
    }

    /// The human label shown in the form.
    pub fn label(self) -> &'static str {
        match self {
            FieldId::Waist => "Waist",
            FieldId::Hips => "Hips",
            FieldId::WaistToHip => "Waist to hip",
            FieldId::SkirtLength => "Skirt length",
        }
    }
}

/// A per-field problem: empty, unparseable, or not positive.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldError {
    /// Which field the message belongs to.
    pub field: FieldId,
    /// The message, taken from the unit service or written plainly.
    pub message: String,
}

/// The editable form state.
#[derive(Debug, Clone)]
pub struct PropertiesForm {
    /// Waist text.
    pub waist: String,
    /// Hips text.
    pub hips: String,
    /// Waist-to-hip text.
    pub waist_to_hip: String,
    /// Skirt length text.
    pub skirt_length: String,
    /// The source tag for waist-to-hip only.
    pub waist_to_hip_tag: SourceTag,
    /// The class default unit for body measurements.
    pub body_unit: Unit,
    /// The display unit for pattern dimensions.
    pub pattern_unit: Unit,
    /// Whether inches display as fractions.
    pub inch_fraction: bool,
    /// The inch fraction denominator.
    pub inch_denominator: u8,
}

impl Default for PropertiesForm {
    fn default() -> Self {
        Self {
            waist: String::new(),
            hips: String::new(),
            waist_to_hip: String::new(),
            skirt_length: String::new(),
            waist_to_hip_tag: SourceTag::Measured,
            body_unit: Unit::Cm,
            pattern_unit: Unit::Mm,
            inch_fraction: true,
            inch_denominator: 16,
        }
    }
}

impl PropertiesForm {
    /// The current text for a field.
    pub fn text(&self, field: FieldId) -> &str {
        match field {
            FieldId::Waist => &self.waist,
            FieldId::Hips => &self.hips,
            FieldId::WaistToHip => &self.waist_to_hip,
            FieldId::SkirtLength => &self.skirt_length,
        }
    }

    /// Replace the text for a field.
    pub fn set_text(&mut self, field: FieldId, text: String) {
        match field {
            FieldId::Waist => self.waist = text,
            FieldId::Hips => self.hips = text,
            FieldId::WaistToHip => self.waist_to_hip = text,
            FieldId::SkirtLength => self.skirt_length = text,
        }
    }

    /// The tags offered for a field. Only waist-to-hip accepts more than
    /// `Measured` (SRS 5.3).
    pub fn allowed_tags(&self, field: FieldId) -> &'static [SourceTag] {
        if field == FieldId::WaistToHip {
            SourceTag::WAIST_TO_HIP_ALLOWED
        } else {
            SourceTag::BODY_MEASUREMENT_ALLOWED
        }
    }

    /// Parse every field into a draft, collecting per-field errors.
    pub fn parse(&self) -> ParsedForm {
        let mut draft = ProfileDraft::new();
        let mut errors = Vec::new();

        for field in FieldId::ALL {
            let raw = self.text(field).trim();
            if raw.is_empty() {
                errors.push(FieldError {
                    field,
                    message: format!("{}: value required", field.label()),
                });
                continue;
            }
            match parse_length(raw, self.body_unit) {
                Ok(length) => {
                    if length.nanometres() <= 0 {
                        errors.push(FieldError {
                            field,
                            message: format!("{}: must be greater than zero", field.label()),
                        });
                        continue;
                    }
                    let tag = if field == FieldId::WaistToHip {
                        self.waist_to_hip_tag
                    } else {
                        SourceTag::Measured
                    };
                    let measurement = Measurement::new(length, tag);
                    match field {
                        FieldId::Waist => draft.waist = Some(measurement),
                        FieldId::Hips => draft.hips = Some(measurement),
                        FieldId::WaistToHip => draft.waist_to_hip = Some(measurement),
                        FieldId::SkirtLength => draft.skirt_length = Some(measurement),
                    }
                }
                Err(error) => errors.push(FieldError {
                    field,
                    message: format!("{}: {}", field.label(), error),
                }),
            }
        }

        ParsedForm { draft, errors }
    }
}

/// The result of parsing the form.
#[derive(Debug, Clone)]
pub struct ParsedForm {
    /// The draft holding every field that parsed.
    pub draft: ProfileDraft,
    /// One entry per field that did not parse, in field order.
    pub errors: Vec<FieldError>,
}

impl ParsedForm {
    /// Whether every field parsed well enough to draw the placeholder.
    pub fn is_drawable(&self) -> bool {
        self.errors.is_empty()
    }

    /// The error message for a field, if any.
    pub fn error_for(&self, field: FieldId) -> Option<&str> {
        self.errors
            .iter()
            .find(|e| e.field == field)
            .map(|e| e.message.as_str())
    }
}

/// Draw the properties form and report whether any field changed.
pub fn draw(
    ui: &mut egui::Ui,
    form: &mut PropertiesForm,
    parsed: &ParsedForm,
    tokens: &Tokens,
) -> bool {
    let mut changed = false;

    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("Body unit").size(tokens.sizes.font_small));
        changed |= unit_combo(ui, "body_unit", &mut form.body_unit);
        ui.label(egui::RichText::new("Pattern unit").size(tokens.sizes.font_small));
        changed |= unit_combo(ui, "pattern_unit", &mut form.pattern_unit);
    });

    ui.horizontal(|ui| {
        changed |= ui
            .checkbox(&mut form.inch_fraction, "Inch fractions")
            .changed();
        egui::ComboBox::from_id_salt("inch_denominator")
            .selected_text(format!("1/{}", form.inch_denominator))
            .show_ui(ui, |ui| {
                for denominator in INCH_DENOMINATORS {
                    changed |= ui
                        .selectable_value(
                            &mut form.inch_denominator,
                            denominator,
                            format!("1/{denominator}"),
                        )
                        .changed();
                }
            });
    });

    ui.separator();

    for field in FieldId::ALL {
        let mut text = form.text(field).to_owned();
        ui.horizontal(|ui| {
            ui.label(field.label());
            if ui.text_edit_singleline(&mut text).changed() {
                form.set_text(field, text.clone());
                changed = true;
            }
        });
        if let Some(message) = parsed.error_for(field) {
            ui.label(
                egui::RichText::new(message)
                    .size(tokens.sizes.font_small)
                    .color(tokens.colors.error),
            );
        }
        if field == FieldId::WaistToHip {
            let mut tag = form.waist_to_hip_tag;
            egui::ComboBox::from_id_salt("waist_to_hip_tag")
                .selected_text(tag.to_string())
                .show_ui(ui, |ui| {
                    for option in form.allowed_tags(field) {
                        changed |= ui
                            .selectable_value(&mut tag, *option, option.to_string())
                            .changed();
                    }
                });
            form.waist_to_hip_tag = tag;
        }
    }

    changed
}

fn unit_combo(ui: &mut egui::Ui, id: &str, unit: &mut Unit) -> bool {
    let mut changed = false;
    egui::ComboBox::from_id_salt(id)
        .selected_text(unit.suffix())
        .show_ui(ui, |ui| {
            for option in UNITS {
                if ui.selectable_value(unit, option, option.suffix()).changed() {
                    changed = true;
                }
            }
        });
    changed
}
