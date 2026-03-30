use crate::support::{child_elements, first_text_child, parse};
use mathml_core::mathml_xml::MathMlElementName;

// Section: 3.2.3 Number <mn>
// Spec: https://www.w3.org/TR/mathml-core/#number-mn
//
// What this file proves:
// - <mn> is recognized as a typed number element.

#[test]
fn supports_mn_element() {
    let document = parse(r#"<math><mn>42</mn></math>"#);
    let mn = child_elements(&document)[0];

    assert_eq!(mn.name, MathMlElementName::Mn);
    assert_eq!(first_text_child(mn).raw, "42");
}
