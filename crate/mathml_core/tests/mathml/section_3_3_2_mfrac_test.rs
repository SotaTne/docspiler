use crate::support::{child_elements, parse};
use mathml_core::mathml_xml::MathMlElementName;

// Section: 3.3.2 Fractions <mfrac>
// Spec: https://www.w3.org/TR/mathml-core/#fractions-mfrac
//
// What this file proves:
// - <mfrac> is recognized as a typed fraction element.

#[test]
fn supports_mfrac_element() {
    let document = parse(r#"<math><mfrac><mi>x</mi><mn>2</mn></mfrac></math>"#);
    let mfrac = child_elements(&document)[0];

    assert_eq!(mfrac.name, MathMlElementName::Mfrac);
}
