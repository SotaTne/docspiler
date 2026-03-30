use crate::support::child_elements;
use crate::support::parse;
use mathml_core::mathml_xml::MathMlElementName;

// Section: 3.5 Tabular Math
// Spec: https://www.w3.org/TR/mathml-core/#tabular-math
//
// What this file proves:
// - Table-related MathML elements are recognized with their structural nesting.

#[test]
fn tabular_math_elements_are_recognized_with_basic_structure() {
    let document = parse(
        r#"<math><mtable><mtr><mtd><mi>a</mi></mtd><mtd><mi>b</mi></mtd></mtr></mtable></math>"#,
    );

    let top = child_elements(&document);
    assert_eq!(top.len(), 1);
    assert_eq!(top[0].name, MathMlElementName::Mtable);

    let XmlNode::Element(row) = &top[0].children[0] else {
        panic!("expected <mtr>");
    };
    assert_eq!(row.name, MathMlElementName::Mtr);

    let cell_names: Vec<MathMlElementName> = row
        .children
        .iter()
        .filter_map(|node| match node {
            mathml_core::mathml_xml::XmlNode::Element(element) => Some(element.name.clone()),
            _ => None,
        })
        .collect();
    assert_eq!(
        cell_names,
        vec![MathMlElementName::Mtd, MathMlElementName::Mtd]
    );
}

use mathml_core::mathml_xml::XmlNode;
