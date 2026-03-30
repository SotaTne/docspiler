use crate::support::child_names;
use crate::support::parse;
use mathml_core::mathml_xml::MathMlElementName;

// Section: 3.3.3 Radicals <msqrt>, <mroot>
// Spec: https://www.w3.org/TR/mathml-core/#radicals-msqrt-mroot
//
// What this file proves:
// - Radical elements are recognized as typed MathML elements.

#[test]
fn supports_radical_elements() {
    let document =
        parse(r#"<math><msqrt><mi>x</mi></msqrt><mroot><mi>x</mi><mn>3</mn></mroot></math>"#);

    assert_eq!(
        child_names(&document),
        vec![MathMlElementName::Msqrt, MathMlElementName::Mroot]
    );
}
