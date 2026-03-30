use crate::support::{child_elements, parse};
use mathml_core::mathml_xml::MathMlElementName;

// Section: 3.5.1 Table or Matrix <mtable>
// Spec: https://www.w3.org/TR/mathml-core/#table-or-matrix-mtable
//
// What this file proves:
// - <mtable> is recognized as a typed table container.

#[test]
fn supports_mtable_element() {
    let document = parse(r#"<math><mtable></mtable></math>"#);
    let mtable = child_elements(&document)[0];

    assert_eq!(mtable.name, MathMlElementName::Mtable);
}
