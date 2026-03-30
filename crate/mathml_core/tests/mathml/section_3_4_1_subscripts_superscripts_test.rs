use crate::support::child_names;
use crate::support::parse;
use mathml_core::mathml_xml::MathMlElementName;

// Section: 3.4.1 Subscripts and Superscripts <msub>, <msup>, <msubsup>
// Spec: https://www.w3.org/TR/mathml-core/#subscripts-and-superscripts-msub-msup-msubsup
//
// What this file proves:
// - Basic sub/sup script schemata are recognized as typed nodes.

#[test]
fn supports_subscript_and_superscript_elements() {
    let document = parse(
        r#"<math>
            <msub><mi>x</mi><mn>1</mn></msub>
            <msup><mi>x</mi><mn>2</mn></msup>
            <msubsup><mi>x</mi><mn>1</mn><mn>2</mn></msubsup>
        </math>"#,
    );

    assert_eq!(
        child_names(&document),
        vec![
            MathMlElementName::Msub,
            MathMlElementName::Msup,
            MathMlElementName::Msubsup,
        ]
    );
}
