use crate::support::child_names;
use crate::support::parse;
use mathml_core::mathml_xml::MathMlElementName;

// Section: 3.4.2 Underscripts and Overscripts <munder>, <mover>, <munderover>
// Spec: https://www.w3.org/TR/mathml-core/#underscripts-and-overscripts-munder-mover-munderover
//
// What this file proves:
// - Under/over script schemata are recognized as typed nodes.

#[test]
fn supports_under_and_over_script_elements() {
    let document = parse(
        r#"<math>
            <munder><mo>∑</mo><mi>i</mi></munder>
            <mover><mo>→</mo><mi>x</mi></mover>
            <munderover><mo>∑</mo><mi>i</mi><mi>n</mi></munderover>
        </math>"#,
    );

    assert_eq!(
        child_names(&document),
        vec![
            MathMlElementName::Munder,
            MathMlElementName::Mover,
            MathMlElementName::Munderover,
        ]
    );
}
