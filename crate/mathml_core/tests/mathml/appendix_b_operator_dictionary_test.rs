use crate::support::{child_elements, first_text_child, parse};
use mathml_core::mathml_xml::{MathMlSymbol, TextToken};

// Appendix: B.1 Operator Dictionary
// Spec: https://www.w3.org/TR/mathml-core/#operator-dictionary
//
// What this file proves:
// - Known operator dictionary symbols are interpreted into native enum tokens.

#[test]
fn supports_known_operator_dictionary_symbols() {
    let document = parse("<math><mo>∑</mo><mo>\u{2062}</mo></math>");
    let elements = child_elements(&document);

    assert_eq!(
        first_text_child(elements[0]).tokens,
        vec![TextToken::Symbol(MathMlSymbol::Summation)]
    );
    assert_eq!(
        first_text_child(elements[1]).tokens,
        vec![TextToken::Symbol(MathMlSymbol::InvisibleTimes)]
    );
}
