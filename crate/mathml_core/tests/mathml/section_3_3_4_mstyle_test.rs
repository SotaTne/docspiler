use crate::support::{child_elements, parse};
use mathml_core::mathml_xml::{MathMlAttributeName, MathMlElementName};

// Section: 3.3.4 Style Change <mstyle>
// Spec: https://www.w3.org/TR/mathml-core/#style-change-mstyle
//
// What this file proves:
// - <mstyle> is recognized as a typed style container.
// - Style-related attributes are preserved.

#[test]
fn supports_mstyle_element() {
    let document = parse(r#"<math><mstyle mathcolor="red"><mi>x</mi></mstyle></math>"#);
    let style = child_elements(&document)[0];

    assert_eq!(style.name, MathMlElementName::Mstyle);
    assert_eq!(style.attributes[0].name, MathMlAttributeName::MathColor);
}
