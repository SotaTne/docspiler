use crate::support::{child_elements, first_text_child, parse};
use mathml_core::mathml_xml::{MathMlSymbol, TextToken};

// Appendix: B.1 Operator Dictionary
// Spec: https://www.w3.org/TR/mathml-core/#operator-dictionary
//
// What this file proves:
// - The current native symbol interpretation layer supports the explicitly enumerated
//   operator characters below without string-based post-dispatch.

#[test]
fn supports_current_native_math_operator_symbols() {
    let document = parse(
        "<math>\
            <mo>+</mo>\
            <mo>−</mo>\
            <mo>×</mo>\
            <mo>÷</mo>\
            <mo>∑</mo>\
            <mo>∫</mo>\
            <mo>∞</mo>\
            <mo>∈</mo>\
            <mo>⊂</mo>\
            <mo>∪</mo>\
            <mo>∩</mo>\
            <mo>∀</mo>\
            <mo>∃</mo>\
            <mo>→</mo>\
            <mo>⇔</mo>\
            <mo>\u{2062}</mo>\
            <mo>\u{2063}</mo>\
            <mo>\u{2061}</mo>\
        </math>",
    );

    let expected = vec![
        MathMlSymbol::Plus,
        MathMlSymbol::MinusSign,
        MathMlSymbol::MultiplicationSign,
        MathMlSymbol::DivisionSign,
        MathMlSymbol::Summation,
        MathMlSymbol::Integral,
        MathMlSymbol::Infinity,
        MathMlSymbol::ElementOf,
        MathMlSymbol::SubsetOf,
        MathMlSymbol::Union,
        MathMlSymbol::Intersection,
        MathMlSymbol::ForAll,
        MathMlSymbol::Exists,
        MathMlSymbol::RightArrow,
        MathMlSymbol::Equivalent,
        MathMlSymbol::InvisibleTimes,
        MathMlSymbol::InvisibleComma,
        MathMlSymbol::ApplyFunction,
    ];

    let actual: Vec<MathMlSymbol> = child_elements(&document)
        .into_iter()
        .map(|element| match &first_text_child(element).tokens[..] {
            [TextToken::Symbol(symbol)] => *symbol,
            tokens => panic!("expected exactly one symbol token, got {tokens:?}"),
        })
        .collect();

    assert_eq!(actual, expected);
}
