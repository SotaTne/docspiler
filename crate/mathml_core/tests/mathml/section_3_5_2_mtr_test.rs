use crate::support::{child_elements, element_children, parse};
use mathml_core::mathml_xml::MathMlElementName;

// Section: 3.5.2 Row in Table or Matrix <mtr>
// Spec: https://www.w3.org/TR/mathml-core/#row-in-table-or-matrix-mtr
//
// What this file proves:
// - <mtr> is recognized as a typed row inside table structure.

#[test]
fn supports_mtr_element() {
    let document = parse(r#"<math><mtable><mtr></mtr></mtable></math>"#);
    let mtable = child_elements(&document)[0];
    let mtr = element_children(mtable)[0];

    assert_eq!(mtr.name, MathMlElementName::Mtr);
}
