use crate::support::{child_elements, parse};
use mathml_core::mathml_xml::MathMlElementName;

// Section: 3.3.7 Making Sub-Expressions Invisible <mphantom>
// Spec: https://www.w3.org/TR/mathml-core/#making-sub-expressions-invisible-mphantom
//
// What this file proves:
// - <mphantom> is recognized as a typed invisibility container.

#[test]
fn supports_mphantom_element() {
    let document = parse(r#"<math><mphantom><mi>x</mi></mphantom></math>"#);
    let mphantom = child_elements(&document)[0];

    assert_eq!(mphantom.name, MathMlElementName::Mphantom);
}
