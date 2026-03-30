use crate::support::{child_elements, element_children, parse};
use mathml_core::mathml_model::{
    ColumnSpan, LengthPercentage, LineThickness, MathDisplay, MathLength, MathLengthUnit,
    MathMlAttributeName, MathMlBoolean, MathVariantValue, ParsedMathMlAttributeValue, RowSpan,
    ScriptLevel, UnsignedInteger,
};

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
    let display = document
        .root
        .attribute(&MathMlAttributeName::Display)
        .expect("math display attribute");

    assert_eq!(display.as_display(), Some(MathDisplay::Block));
    assert_eq!(
        display.interpreted_value(),
        Some(ParsedMathMlAttributeValue::Display(MathDisplay::Block))
    );
}

#[test]
fn supports_typed_boolean_and_scriptlevel_interpretation() {
    let document =
        parse(r#"<math><mstyle displaystyle="true" scriptlevel="-2"><mi>x</mi></mstyle></math>"#);
    let style = child_elements(&document)[0];

    let displaystyle = style
        .attribute(&MathMlAttributeName::DisplayStyle)
        .expect("displaystyle attribute");
    let scriptlevel = style
        .attribute(&MathMlAttributeName::ScriptLevel)
        .expect("scriptlevel attribute");

    assert_eq!(displaystyle.as_boolean(), Some(MathMlBoolean::True));
    assert_eq!(scriptlevel.as_scriptlevel(), Some(ScriptLevel::Add(-2)));
    assert_eq!(
        scriptlevel.interpreted_value(),
        Some(ParsedMathMlAttributeValue::ScriptLevel(ScriptLevel::Add(
            -2
        )))
    );
}

#[test]
fn supports_typed_mathvariant_interpretation() {
    let document =
        parse(r#"<math><mi mathvariant="normal">x</mi><mi mathvariant="bold">y</mi></math>"#);
    let children = child_elements(&document);

    let normal = children[0]
        .attribute(&MathMlAttributeName::MathVariant)
        .expect("normal mathvariant");
    let other = children[1]
        .attribute(&MathMlAttributeName::MathVariant)
        .expect("other mathvariant");

    assert_eq!(normal.as_mathvariant(), Some(MathVariantValue::Normal));
    assert_eq!(
        other.as_mathvariant(),
        Some(MathVariantValue::Other("bold".to_string()))
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

    let rowspan = mtd
        .attribute(&MathMlAttributeName::RowSpan)
        .expect("rowspan attribute");
    let columnspan = mtd
        .attribute(&MathMlAttributeName::ColumnSpan)
        .expect("columnspan attribute");

    assert_eq!(rowspan.as_unsigned_integer(), Some(UnsignedInteger(2)));
    assert_eq!(rowspan.as_rowspan(), Some(RowSpan(2)));
    assert_eq!(columnspan.as_columnspan(), Some(ColumnSpan(3)));
    assert_eq!(
        rowspan.interpreted_value(),
        Some(ParsedMathMlAttributeValue::RowSpan(RowSpan(2)))
    );
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
        mo.attribute(&MathMlAttributeName::LSpace)
            .and_then(|attr| attr.as_length_percentage()),
        Some(LengthPercentage::Length(MathLength {
            value: 0.2,
            unit: MathLengthUnit::Em,
        }))
    );
    assert_eq!(
        mo.attribute(&MathMlAttributeName::RSpace)
            .and_then(|attr| attr.as_length_percentage()),
        Some(LengthPercentage::Percentage(150.0))
    );
    assert_eq!(
        mo.attribute(&MathMlAttributeName::MinSize)
            .and_then(|attr| attr.as_length_percentage()),
        Some(LengthPercentage::Length(MathLength {
            value: 2.0,
            unit: MathLengthUnit::Em,
        }))
    );
    assert_eq!(
        mo.attribute(&MathMlAttributeName::MaxSize)
            .and_then(|attr| attr.as_length_percentage()),
        Some(LengthPercentage::Percentage(300.0))
    );
    assert_eq!(
        mspace
            .attribute(&MathMlAttributeName::Width)
            .and_then(|attr| attr.as_length_percentage()),
        Some(LengthPercentage::Length(MathLength {
            value: 1.0,
            unit: MathLengthUnit::Em,
        }))
    );
    assert_eq!(
        mspace
            .attribute(&MathMlAttributeName::Height)
            .and_then(|attr| attr.as_length_percentage()),
        Some(LengthPercentage::Length(MathLength {
            value: 2.0,
            unit: MathLengthUnit::Ex,
        }))
    );
    assert_eq!(
        mspace
            .attribute(&MathMlAttributeName::Depth)
            .and_then(|attr| attr.as_length_percentage()),
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
    let linethickness = mfrac
        .attribute(&MathMlAttributeName::LineThickness)
        .expect("linethickness attribute");

    assert_eq!(
        linethickness.as_linethickness(),
        Some(LineThickness(LengthPercentage::Length(MathLength {
            value: 0.0,
            unit: MathLengthUnit::Px,
        })))
    );
    assert_eq!(
        linethickness.interpreted_value(),
        Some(ParsedMathMlAttributeValue::LineThickness(LineThickness(
            LengthPercentage::Length(MathLength {
                value: 0.0,
                unit: MathLengthUnit::Px,
            })
        )))
    );
}
