use crate::support::{child_elements, first_text_child, parse};
use mathml_core::mathml_xml::{MathMlSymbol, TextToken};

// Appendix: B.1 Operator Dictionary
// Spec: https://www.w3.org/TR/mathml-core/#operator-dictionary
//
// What this file proves:
// - Multi-character ASCII operator groups from the operator dictionary are interpreted
//   as single native symbols at the XML layer.

#[test]
fn supports_multi_character_operator_groups() {
    let cases = vec![
        ("<mo>!=</mo>", MathMlSymbol::BangEquals),
        ("<mo>&lt;=</mo>", MathMlSymbol::LessThanOrEqualAscii),
        ("<mo>-></mo>", MathMlSymbol::ArrowAscii),
        ("<mo>||</mo>", MathMlSymbol::DoubleVerticalBarAscii),
        ("<mo>&amp;&amp;</mo>", MathMlSymbol::DoubleAmpersand),
        ("<mo>==</mo>", MathMlSymbol::DoubleEquals),
        ("<mo>:=</mo>", MathMlSymbol::ColonEquals),
        ("<mo>**</mo>", MathMlSymbol::DoubleAsterisk),
        ("<mo>++</mo>", MathMlSymbol::DoublePlus),
        ("<mo>--</mo>", MathMlSymbol::DoubleHyphen),
    ];

    for (fragment, expected) in cases {
        let document = parse(&format!("<math>{fragment}</math>"));
        let element = child_elements(&document)
            .into_iter()
            .next()
            .expect("expected one mo element");
        let text = first_text_child(element);

        match &text.tokens[..] {
            [TextToken::Symbol(symbol)] => assert_eq!(*symbol, expected),
            tokens => panic!(
                "expected one grouped symbol token, raw={:?}, got {tokens:?}",
                text.raw
            ),
        }
    }
}
