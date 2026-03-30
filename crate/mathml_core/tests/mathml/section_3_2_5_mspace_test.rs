use crate::support::{child_elements, parse};
use mathml_core::mathml_model::{LengthPercentage, MathLength, MathLengthUnit};
use mathml_core::mathml_xml::{MathMlAttributeName, MathMlElementName};

// Section: 3.2.5 Space <mspace>
// Spec: https://www.w3.org/TR/mathml-core/#space-mspace
//
// What this file proves:
// - <mspace> is recognized as a typed spacing element.
// - Basic sizing attributes are preserved.

#[test]
fn supports_mspace_element() {
    let document = parse(r#"<math><mspace width="1em" height="2ex" depth="0.5ex"/></math>"#);
    let mspace = child_elements(&document)[0];

    assert_eq!(mspace.name, MathMlElementName::Mspace);
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
