use crate::support::{child_elements, parse};
use mathml_core::mathml_xml::{MathMlElementName, MathMlSymbol, TextToken, XmlNode};

// Section: 3.2 Token Elements
// Spec: https://www.w3.org/TR/mathml-core/#token-elements
//
// What this file proves:
// - Token elements are recognized as typed MathML token nodes.
// - Basic operator symbol interpretation is preserved at the XML layer.

#[test]
fn token_elements_are_recognized_and_operator_symbols_are_tokenized() {
    let document = parse(
        r#"<math><mi>x</mi><mn>2</mn><mo>+</mo><mtext>speed</mtext><ms>"hi"</ms><mspace width="1em"/></math>"#,
    );

    let children = child_elements(&document);
    assert_eq!(children.len(), 6);
    assert_eq!(children[0].name, MathMlElementName::Mi);
    assert_eq!(children[1].name, MathMlElementName::Mn);
    assert_eq!(children[2].name, MathMlElementName::Mo);
    assert_eq!(children[3].name, MathMlElementName::Mtext);
    assert_eq!(children[4].name, MathMlElementName::Ms);
    assert_eq!(children[5].name, MathMlElementName::Mspace);

    let XmlNode::Text(text) = &children[2].children[0] else {
        panic!("expected text inside <mo>");
    };
    assert_eq!(text.tokens, vec![TextToken::Symbol(MathMlSymbol::Plus)]);
}
