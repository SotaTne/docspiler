use crate::support::{child_elements, parse};
use mathml_core::mathml_xml::MathMlElementName;

// Section: 3.3.1 Group Sub-Expressions <mrow>
// Spec: https://www.w3.org/TR/mathml-core/#horizontally-group-sub-expressions-mrow
//
// What this file proves:
// - <mrow> is recognized as a typed grouping element.

#[test]
fn supports_mrow_element() {
    let document = parse(r#"<math><mrow><mi>a</mi><mo>+</mo><mi>b</mi></mrow></math>"#);
    let mrow = child_elements(&document)[0];

    assert_eq!(mrow.name, MathMlElementName::Mrow);
}
