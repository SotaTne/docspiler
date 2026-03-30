use crate::support::{child_elements, parse};
use mathml_core::mathml_xml::MathMlElementName;

// Section: 3.4.3 Prescripts and Tensor Indices <mmultiscripts>
// Spec: https://www.w3.org/TR/mathml-core/#prescripts-and-tensor-indices-mmultiscripts
//
// What this file proves:
// - <mmultiscripts> and <mprescripts/> are recognized as typed script nodes.

#[test]
fn supports_mmultiscripts_element() {
    let document = parse(
        r#"<math><mmultiscripts><mi>T</mi><mi>a</mi><mi>b</mi><mprescripts/><mi>c</mi><mi>d</mi></mmultiscripts></math>"#,
    );
    let mmultiscripts = child_elements(&document)[0];
    let names: Vec<MathMlElementName> = crate::support::element_children(mmultiscripts)
        .into_iter()
        .map(|child| child.name.clone())
        .collect();

    assert_eq!(mmultiscripts.name, MathMlElementName::Mmultiscripts);
    assert_eq!(names[3], MathMlElementName::Mprescripts);
}
