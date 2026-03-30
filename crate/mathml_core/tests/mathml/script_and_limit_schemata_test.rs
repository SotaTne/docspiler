use crate::support::child_names;
use crate::support::parse;
use mathml_core::mathml_xml::MathMlElementName;

// Section: 3.4 Script and Limit Schemata
// Spec: https://www.w3.org/TR/mathml-core/#script-and-limit-schemata
//
// What this file proves:
// - Script and limit elements are recognized as typed MathML script nodes.

#[test]
fn script_and_limit_elements_are_recognized_as_typed_nodes() {
    let document = parse(
        r#"<math>
            <msub><mi>x</mi><mn>1</mn></msub>
            <msup><mi>x</mi><mn>2</mn></msup>
            <msubsup><mi>x</mi><mn>1</mn><mn>2</mn></msubsup>
            <munder><mo>∑</mo><mi>i</mi></munder>
            <mover><mo>→</mo><mi>x</mi></mover>
            <munderover><mo>∑</mo><mi>i</mi><mi>n</mi></munderover>
            <mmultiscripts><mi>T</mi><mi>a</mi><mi>b</mi><mprescripts/><mi>c</mi><mi>d</mi></mmultiscripts>
        </math>"#,
    );

    assert_eq!(
        child_names(&document),
        vec![
            MathMlElementName::Msub,
            MathMlElementName::Msup,
            MathMlElementName::Msubsup,
            MathMlElementName::Munder,
            MathMlElementName::Mover,
            MathMlElementName::Munderover,
            MathMlElementName::Mmultiscripts,
        ]
    );
}
