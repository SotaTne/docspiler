use crate::support::{child_elements, element_children, parse};
use mathml_core::mathml_xml::MathMlElementName;

// Section: 3.5.3 Entry in Table or Matrix <mtd>
// Spec: https://www.w3.org/TR/mathml-core/#entry-in-table-or-matrix-mtd
//
// What this file proves:
// - <mtd> is recognized as a typed table cell.

#[test]
fn supports_mtd_element() {
    let document = parse(r#"<math><mtable><mtr><mtd><mi>a</mi></mtd></mtr></mtable></math>"#);
    let mtable = child_elements(&document)[0];
    let mtr = element_children(mtable)[0];
    let mtd = element_children(mtr)[0];

    assert_eq!(mtd.name, MathMlElementName::Mtd);
}
