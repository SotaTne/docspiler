use crate::support::{child_elements, parse};
use mathml_core::mathml_model::{LengthPercentage, MathLength, MathLengthUnit};
use mathml_core::mathml_xml::{MathMlAttributeName, MathMlElementName};

// Section: 3.3.6 Adjust Space Around Content <mpadded>
// Spec: https://www.w3.org/TR/mathml-core/#adjust-space-around-content-mpadded
//
// What this file proves:
// - <mpadded> is recognized as a typed layout adjustment container.
// - Core size attributes are preserved.

#[test]
fn supports_mpadded_element() {
    let document = parse(
        r#"<math><mpadded width="+1em" height="+1ex" depth="+1ex"><mi>x</mi></mpadded></math>"#,
    );
    let mpadded = child_elements(&document)[0];

    assert_eq!(mpadded.name, MathMlElementName::Mpadded);
    assert_eq!(
        mpadded
            .attribute(&MathMlAttributeName::Width)
            .and_then(|attr| attr.as_length_percentage()),
        Some(LengthPercentage::Length(MathLength {
            value: 1.0,
            unit: MathLengthUnit::Em,
        }))
    );
    assert_eq!(
        mpadded
            .attribute(&MathMlAttributeName::Height)
            .and_then(|attr| attr.as_length_percentage()),
        Some(LengthPercentage::Length(MathLength {
            value: 1.0,
            unit: MathLengthUnit::Ex,
        }))
    );
    assert_eq!(
        mpadded
            .attribute(&MathMlAttributeName::Depth)
            .and_then(|attr| attr.as_length_percentage()),
        Some(LengthPercentage::Length(MathLength {
            value: 1.0,
            unit: MathLengthUnit::Ex,
        }))
    );
}
