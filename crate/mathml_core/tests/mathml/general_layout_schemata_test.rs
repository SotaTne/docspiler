use crate::support::child_names;
use crate::support::parse;
use mathml_core::mathml_xml::MathMlElementName;

// Section: 3.3 General Layout Schemata
// Spec: https://www.w3.org/TR/mathml-core/#general-layout-schemata
//
// What this file proves:
// - General layout container elements are recognized as typed layout nodes.

#[test]
fn general_layout_elements_are_recognized_as_typed_nodes() {
    let document = parse(
        r#"<math>
            <mrow><mi>a</mi><mo>+</mo><mi>b</mi></mrow>
            <mfrac><mi>x</mi><mi>y</mi></mfrac>
            <msqrt><mi>x</mi></msqrt>
            <mroot><mi>x</mi><mn>3</mn></mroot>
            <mstyle><mi>z</mi></mstyle>
            <merror><mtext>bad</mtext></merror>
            <mpadded><mi>p</mi></mpadded>
            <mphantom><mi>q</mi></mphantom>
        </math>"#,
    );

    assert_eq!(
        child_names(&document),
        vec![
            MathMlElementName::Mrow,
            MathMlElementName::Mfrac,
            MathMlElementName::Msqrt,
            MathMlElementName::Mroot,
            MathMlElementName::Mstyle,
            MathMlElementName::Merror,
            MathMlElementName::Mpadded,
            MathMlElementName::Mphantom,
        ]
    );
}
