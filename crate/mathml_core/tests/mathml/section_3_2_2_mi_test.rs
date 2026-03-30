use crate::support::{child_elements, first_text_child, parse};
use mathml_core::mathml_xml::MathMlElementName;

// Section: 3.2.2 Identifier <mi>
// Spec: https://www.w3.org/TR/mathml-core/#the-mi-element
//
// What this file proves:
// - <mi> is recognized as a typed identifier element.

#[test]
fn supports_mi_element() {
    let document = parse(r#"<math><mi>x</mi></math>"#);
    let mi = child_elements(&document)[0];

    assert_eq!(mi.name, MathMlElementName::Mi);
    assert_eq!(first_text_child(mi).raw, "x");
}
