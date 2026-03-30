pub use crate::mathml_model::{
    ColumnSpan, LengthPercentage, LineThickness, MathDisplay, MathLength, MathLengthUnit,
    MathMlAttributeName, MathMlBoolean, MathVariantValue, ParsedMathMlAttributeValue, RowSpan,
    ScriptLevel, UnsignedInteger, XmlAttribute,
};

pub fn parse_boolean(raw: &str) -> Option<MathMlBoolean> {
    if raw.eq_ignore_ascii_case("true") {
        Some(MathMlBoolean::True)
    } else if raw.eq_ignore_ascii_case("false") {
        Some(MathMlBoolean::False)
    } else {
        None
    }
}

pub fn parse_unsigned_integer(raw: &str) -> Option<UnsignedInteger> {
    let trimmed = raw.trim();
    if trimmed.is_empty() || !trimmed.bytes().all(|byte| byte.is_ascii_digit()) {
        return None;
    }
    trimmed.parse().ok().map(UnsignedInteger)
}

pub fn parse_scriptlevel(raw: &str) -> Option<ScriptLevel> {
    let trimmed = raw.trim();
    if let Some(rest) = trimmed.strip_prefix('+') {
        let value = parse_unsigned_integer(rest)?.0;
        return i32::try_from(value).ok().map(ScriptLevel::Add);
    }
    if let Some(rest) = trimmed.strip_prefix('-') {
        let value = parse_unsigned_integer(rest)?.0;
        return i32::try_from(value)
            .ok()
            .map(|value| ScriptLevel::Add(-value));
    }
    parse_unsigned_integer(trimmed).map(|value| ScriptLevel::Absolute(value.0))
}

pub fn parse_math_display(raw: &str) -> Option<MathDisplay> {
    if raw.eq_ignore_ascii_case("inline") {
        Some(MathDisplay::Inline)
    } else if raw.eq_ignore_ascii_case("block") {
        Some(MathDisplay::Block)
    } else {
        None
    }
}

pub fn interpret_mathvariant(raw: &str) -> MathVariantValue {
    if raw.eq_ignore_ascii_case("normal") {
        MathVariantValue::Normal
    } else {
        MathVariantValue::Other(raw.to_string())
    }
}

pub fn parse_rowspan(raw: &str) -> Option<RowSpan> {
    let value = parse_unsigned_integer(raw)?.0;
    (value > 0).then_some(RowSpan(value))
}

pub fn parse_columnspan(raw: &str) -> Option<ColumnSpan> {
    let value = parse_unsigned_integer(raw)?.0;
    (value > 0).then_some(ColumnSpan(value))
}

pub fn parse_length_percentage(raw: &str) -> Option<LengthPercentage> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return None;
    }

    if let Some(number) = trimmed.strip_suffix('%') {
        return number.trim().parse().ok().map(LengthPercentage::Percentage);
    }

    let (number, unit) = parse_length_with_unit(trimmed)?;
    Some(LengthPercentage::Length(MathLength {
        value: number,
        unit,
    }))
}

pub fn parse_linethickness(raw: &str) -> Option<LineThickness> {
    parse_length_percentage(raw).map(|value| match value {
        LengthPercentage::Length(length) => LineThickness(LengthPercentage::Length(MathLength {
            value: length.value.max(0.0),
            unit: length.unit,
        })),
        LengthPercentage::Percentage(percent) => {
            LineThickness(LengthPercentage::Percentage(percent.max(0.0)))
        }
    })
}

impl XmlAttribute {
    pub fn interpreted_value(&self) -> Option<ParsedMathMlAttributeValue> {
        match self.name {
            MathMlAttributeName::DisplayStyle
            | MathMlAttributeName::Fence
            | MathMlAttributeName::Separator
            | MathMlAttributeName::Stretchy
            | MathMlAttributeName::Symmetric
            | MathMlAttributeName::LargeOp
            | MathMlAttributeName::MovableLimits
            | MathMlAttributeName::Accent
            | MathMlAttributeName::AccentUnder
            | MathMlAttributeName::Bevelled
            | MathMlAttributeName::EqualRows
            | MathMlAttributeName::EqualColumns => {
                self.as_boolean().map(ParsedMathMlAttributeValue::Boolean)
            }
            MathMlAttributeName::ScriptLevel => self
                .as_scriptlevel()
                .map(ParsedMathMlAttributeValue::ScriptLevel),
            MathMlAttributeName::Display => {
                self.as_display().map(ParsedMathMlAttributeValue::Display)
            }
            MathMlAttributeName::MathVariant => self
                .as_mathvariant()
                .map(ParsedMathMlAttributeValue::MathVariant),
            MathMlAttributeName::LSpace
            | MathMlAttributeName::RSpace
            | MathMlAttributeName::MinSize
            | MathMlAttributeName::MaxSize
            | MathMlAttributeName::Width
            | MathMlAttributeName::Height
            | MathMlAttributeName::Depth
            | MathMlAttributeName::MathSize
            | MathMlAttributeName::MinLabelSpacing => self
                .as_length_percentage()
                .map(ParsedMathMlAttributeValue::LengthPercentage),
            MathMlAttributeName::LineThickness => self
                .as_linethickness()
                .map(ParsedMathMlAttributeValue::LineThickness),
            MathMlAttributeName::RowSpan => {
                self.as_rowspan().map(ParsedMathMlAttributeValue::RowSpan)
            }
            MathMlAttributeName::ColumnSpan => self
                .as_columnspan()
                .map(ParsedMathMlAttributeValue::ColumnSpan),
            _ => None,
        }
    }

    pub fn as_boolean(&self) -> Option<MathMlBoolean> {
        parse_boolean(&self.value)
    }

    pub fn as_unsigned_integer(&self) -> Option<UnsignedInteger> {
        parse_unsigned_integer(&self.value)
    }

    pub fn as_scriptlevel(&self) -> Option<ScriptLevel> {
        parse_scriptlevel(&self.value)
    }

    pub fn as_display(&self) -> Option<MathDisplay> {
        parse_math_display(&self.value)
    }

    pub fn as_mathvariant(&self) -> Option<MathVariantValue> {
        Some(interpret_mathvariant(&self.value))
    }

    pub fn as_length_percentage(&self) -> Option<LengthPercentage> {
        parse_length_percentage(&self.value)
    }

    pub fn as_linethickness(&self) -> Option<LineThickness> {
        parse_linethickness(&self.value)
    }

    pub fn as_rowspan(&self) -> Option<RowSpan> {
        parse_rowspan(&self.value)
    }

    pub fn as_columnspan(&self) -> Option<ColumnSpan> {
        parse_columnspan(&self.value)
    }
}

fn parse_length_with_unit(raw: &str) -> Option<(f32, MathLengthUnit)> {
    const UNITS: [(&str, MathLengthUnit); 18] = [
        ("vmin", MathLengthUnit::Vmin),
        ("vmax", MathLengthUnit::Vmax),
        ("rem", MathLengthUnit::Rem),
        ("rlh", MathLengthUnit::Rlh),
        ("em", MathLengthUnit::Em),
        ("ex", MathLengthUnit::Ex),
        ("px", MathLengthUnit::Px),
        ("in", MathLengthUnit::In),
        ("cm", MathLengthUnit::Cm),
        ("mm", MathLengthUnit::Mm),
        ("pt", MathLengthUnit::Pt),
        ("pc", MathLengthUnit::Pc),
        ("ch", MathLengthUnit::Ch),
        ("lh", MathLengthUnit::Lh),
        ("vw", MathLengthUnit::Vw),
        ("vh", MathLengthUnit::Vh),
        ("q", MathLengthUnit::Q),
        ("Q", MathLengthUnit::Q),
    ];

    for (suffix, unit) in UNITS {
        if let Some(number) = raw.strip_suffix(suffix) {
            return number.trim().parse().ok().map(|value| (value, unit));
        }
    }

    let value: f32 = raw.parse().ok()?;
    (value == 0.0).then_some((value, MathLengthUnit::Unitless))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_boolean_values_case_insensitively() {
        assert_eq!(parse_boolean("true"), Some(MathMlBoolean::True));
        assert_eq!(parse_boolean("FALSE"), Some(MathMlBoolean::False));
        assert_eq!(parse_boolean("yes"), None);
        assert_eq!(parse_boolean("1"), None);
    }

    #[test]
    fn parses_unsigned_integer_without_signs() {
        assert_eq!(parse_unsigned_integer("0"), Some(UnsignedInteger(0)));
        assert_eq!(parse_unsigned_integer("42"), Some(UnsignedInteger(42)));
        assert_eq!(parse_unsigned_integer("+1"), None);
        assert_eq!(parse_unsigned_integer("-1"), None);
        assert_eq!(parse_unsigned_integer("1.5"), None);
    }

    #[test]
    fn parses_scriptlevel_forms() {
        assert_eq!(parse_scriptlevel("0"), Some(ScriptLevel::Absolute(0)));
        assert_eq!(parse_scriptlevel("+2"), Some(ScriptLevel::Add(2)));
        assert_eq!(parse_scriptlevel("-3"), Some(ScriptLevel::Add(-3)));
        assert_eq!(parse_scriptlevel("++1"), None);
        assert_eq!(parse_scriptlevel("+-1"), None);
    }

    #[test]
    fn parses_display_attribute_values() {
        assert_eq!(parse_math_display("inline"), Some(MathDisplay::Inline));
        assert_eq!(parse_math_display("BLOCK"), Some(MathDisplay::Block));
        assert_eq!(parse_math_display("grid"), None);
    }

    #[test]
    fn interprets_mathvariant_normal_separately() {
        assert_eq!(interpret_mathvariant("normal"), MathVariantValue::Normal);
        assert_eq!(interpret_mathvariant("NORMAL"), MathVariantValue::Normal);
        assert_eq!(
            interpret_mathvariant("italic"),
            MathVariantValue::Other("italic".to_string())
        );
    }

    #[test]
    fn parses_positive_table_spans() {
        assert_eq!(parse_rowspan("3"), Some(RowSpan(3)));
        assert_eq!(parse_columnspan("2"), Some(ColumnSpan(2)));
        assert_eq!(parse_rowspan("0"), None);
        assert_eq!(parse_columnspan("0"), None);
        assert_eq!(parse_rowspan("+1"), None);
    }

    #[test]
    fn parses_length_percentage_values() {
        assert_eq!(
            parse_length_percentage("1em"),
            Some(LengthPercentage::Length(MathLength {
                value: 1.0,
                unit: MathLengthUnit::Em
            }))
        );
        assert_eq!(
            parse_length_percentage("0.5ex"),
            Some(LengthPercentage::Length(MathLength {
                value: 0.5,
                unit: MathLengthUnit::Ex
            }))
        );
        assert_eq!(
            parse_length_percentage("200%"),
            Some(LengthPercentage::Percentage(200.0))
        );
        assert_eq!(
            parse_length_percentage("0"),
            Some(LengthPercentage::Length(MathLength {
                value: 0.0,
                unit: MathLengthUnit::Unitless
            }))
        );
        assert_eq!(parse_length_percentage("12"), None);
        assert_eq!(parse_length_percentage("calc(1em + 2px)"), None);
    }

    #[test]
    fn parses_linethickness_and_clamps_negative_values() {
        assert_eq!(
            parse_linethickness("2px"),
            Some(LineThickness(LengthPercentage::Length(MathLength {
                value: 2.0,
                unit: MathLengthUnit::Px
            })))
        );
        assert_eq!(
            parse_linethickness("150%"),
            Some(LineThickness(LengthPercentage::Percentage(150.0)))
        );
        assert_eq!(
            parse_linethickness("-1px"),
            Some(LineThickness(LengthPercentage::Length(MathLength {
                value: 0.0,
                unit: MathLengthUnit::Px
            })))
        );
        assert_eq!(
            parse_linethickness("-25%"),
            Some(LineThickness(LengthPercentage::Percentage(0.0)))
        );
        assert_eq!(parse_linethickness("thin"), None);
    }
}
