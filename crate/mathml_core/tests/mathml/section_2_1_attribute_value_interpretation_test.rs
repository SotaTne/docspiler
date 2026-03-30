use crate::support::{child_elements, element_children, parse};
use mathml_core::mathml_attr_value::{
    ColumnSpan, LengthPercentage, LineThickness, MathDisplay, MathLength, MathLengthUnit,
    MathMlBoolean, MathVariantValue, RowSpan, ScriptLevel, UnsignedInteger, interpret_mathvariant,
    parse_boolean, parse_columnspan, parse_length_percentage, parse_linethickness,
    parse_math_display, parse_rowspan, parse_scriptlevel, parse_unsigned_integer,
};
use mathml_core::mathml_xml::MathMlAttributeName;

// Section: 2.1 / Rendering-relevant attribute value interpretation
// Specs:
// - https://www.w3.org/TR/mathml-core/#the-top-level-math-element
// - https://www.w3.org/TR/mathml-core/#the-displaystyle-and-scriptlevel-attributes
// - https://www.w3.org/TR/mathml-core/#the-mathvariant-attribute
// - https://www.w3.org/TR/mathml-core/#entry-in-table-or-matrix-mtd
//
// What this file proves:
// - Parsed MathML attributes can be interpreted through typed helper parsers.
// - The helpers accept valid rendering-relevant forms and reject invalid ones.

#[test]
fn supports_typed_display_attribute_interpretation() {
    let document = parse(r#"<math display="block"><mi>x</mi></math>"#);
    let display = &document.root.attributes[0];

    assert_eq!(display.name, MathMlAttributeName::Display);
    assert_eq!(parse_math_display(&display.value), Some(MathDisplay::Block));
    assert_eq!(parse_math_display("INLINE"), Some(MathDisplay::Inline));
    assert_eq!(parse_math_display("grid"), None);
}

#[test]
fn supports_typed_boolean_and_scriptlevel_interpretation() {
    let document =
        parse(r#"<math><mstyle displaystyle="true" scriptlevel="-2"><mi>x</mi></mstyle></math>"#);
    let style = child_elements(&document)[0];

    let displaystyle = &style.attributes[0];
    let scriptlevel = &style.attributes[1];

    assert_eq!(displaystyle.name, MathMlAttributeName::DisplayStyle);
    assert_eq!(scriptlevel.name, MathMlAttributeName::ScriptLevel);
    assert_eq!(
        parse_boolean(&displaystyle.value),
        Some(MathMlBoolean::True)
    );
    assert_eq!(parse_boolean("FALSE"), Some(MathMlBoolean::False));
    assert_eq!(parse_boolean("1"), None);
    assert_eq!(
        parse_scriptlevel(&scriptlevel.value),
        Some(ScriptLevel::Add(-2))
    );
    assert_eq!(parse_scriptlevel("+3"), Some(ScriptLevel::Add(3)));
    assert_eq!(parse_scriptlevel("4"), Some(ScriptLevel::Absolute(4)));
    assert_eq!(parse_scriptlevel("+-1"), None);
}

#[test]
fn supports_typed_mathvariant_interpretation() {
    let document =
        parse(r#"<math><mi mathvariant="normal">x</mi><mi mathvariant="bold">y</mi></math>"#);
    let children = child_elements(&document);

    let normal = &children[0].attributes[0];
    let other = &children[1].attributes[0];

    assert_eq!(normal.name, MathMlAttributeName::MathVariant);
    assert_eq!(other.name, MathMlAttributeName::MathVariant);
    assert_eq!(
        interpret_mathvariant(&normal.value),
        MathVariantValue::Normal
    );
    assert_eq!(
        interpret_mathvariant(&other.value),
        MathVariantValue::Other("bold".to_string())
    );
}

#[test]
fn supports_typed_unsigned_integer_and_table_span_interpretation() {
    let document = parse(
        r#"<math><mtable><mtr><mtd rowspan="2" columnspan="3"><mi>a</mi></mtd></mtr></mtable></math>"#,
    );
    let mtable = child_elements(&document)[0];
    let mtr = element_children(mtable)[0];
    let mtd = element_children(mtr)[0];

    let rowspan = &mtd.attributes[0];
    let columnspan = &mtd.attributes[1];

    assert_eq!(rowspan.name, MathMlAttributeName::RowSpan);
    assert_eq!(columnspan.name, MathMlAttributeName::ColumnSpan);
    assert_eq!(
        parse_unsigned_integer(&rowspan.value),
        Some(UnsignedInteger(2))
    );
    assert_eq!(parse_unsigned_integer("+1"), None);
    assert_eq!(parse_rowspan(&rowspan.value), Some(RowSpan(2)));
    assert_eq!(parse_columnspan(&columnspan.value), Some(ColumnSpan(3)));
    assert_eq!(parse_rowspan("0"), None);
    assert_eq!(parse_columnspan("0"), None);
}

#[test]
fn supports_typed_length_percentage_interpretation() {
    let document = parse(
        r#"<math><mo lspace="0.2em" rspace="150%" minsize="2em" maxsize="300%">+</mo><mspace width="1em" height="2ex" depth="0.5ex"/></math>"#,
    );
    let children = child_elements(&document);
    let mo = children[0];
    let mspace = children[1];

    assert_eq!(
        parse_length_percentage(&mo.attributes[0].value),
        Some(LengthPercentage::Length(MathLength {
            value: 0.2,
            unit: MathLengthUnit::Em,
        }))
    );
    assert_eq!(
        parse_length_percentage(&mo.attributes[1].value),
        Some(LengthPercentage::Percentage(150.0))
    );
    assert_eq!(
        parse_length_percentage(&mo.attributes[2].value),
        Some(LengthPercentage::Length(MathLength {
            value: 2.0,
            unit: MathLengthUnit::Em,
        }))
    );
    assert_eq!(
        parse_length_percentage(&mo.attributes[3].value),
        Some(LengthPercentage::Percentage(300.0))
    );
    assert_eq!(
        parse_length_percentage(&mspace.attributes[0].value),
        Some(LengthPercentage::Length(MathLength {
            value: 1.0,
            unit: MathLengthUnit::Em,
        }))
    );
    assert_eq!(
        parse_length_percentage(&mspace.attributes[1].value),
        Some(LengthPercentage::Length(MathLength {
            value: 2.0,
            unit: MathLengthUnit::Ex,
        }))
    );
    assert_eq!(
        parse_length_percentage(&mspace.attributes[2].value),
        Some(LengthPercentage::Length(MathLength {
            value: 0.5,
            unit: MathLengthUnit::Ex,
        }))
    );
}

#[test]
fn supports_typed_linethickness_interpretation() {
    let document =
        parse(r#"<math><mfrac linethickness="-1px"><mi>x</mi><mn>2</mn></mfrac></math>"#);
    let mfrac = child_elements(&document)[0];
    let linethickness = &mfrac.attributes[0];

    assert_eq!(linethickness.name, MathMlAttributeName::LineThickness);
    assert_eq!(
        parse_linethickness(&linethickness.value),
        Some(LineThickness(LengthPercentage::Length(MathLength {
            value: 0.0,
            unit: MathLengthUnit::Px,
        })))
    );
    assert_eq!(
        parse_linethickness("200%"),
        Some(LineThickness(LengthPercentage::Percentage(200.0)))
    );
    assert_eq!(parse_linethickness("thin"), None);
}
