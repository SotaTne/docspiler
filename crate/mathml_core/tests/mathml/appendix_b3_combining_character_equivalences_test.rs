use mathml_core::mathml_xml::{AccentPosition, MathMlSymbol};

// Appendix: B.3 Combining Character Equivalences
// Spec: https://www.w3.org/TR/mathml-core/#combining-character-equivalences
//
// What this file proves:
// - Native symbols can expose the MathML Core combining-accent equivalence table
//   without string-based post-dispatch.

#[test]
fn supports_combining_character_equivalences() {
    let cases = vec![
        (MathMlSymbol::Plus, AccentPosition::Below, '\u{031F}'),
        (MathMlSymbol::HyphenMinus, AccentPosition::Above, '\u{0305}'),
        (MathMlSymbol::Tilde, AccentPosition::Above, '\u{0303}'),
        (MathMlSymbol::RightArrow, AccentPosition::Above, '\u{20D7}'),
        (
            MathMlSymbol::LongRightArrow,
            AccentPosition::Above,
            '\u{20D7}',
        ),
    ];

    for (symbol, position, combining) in cases {
        let entry = symbol
            .combining_character_equivalence()
            .expect("expected combining character equivalence");

        assert_eq!(entry.base, symbol);
        assert_eq!(entry.position, position);
        assert_eq!(entry.combining, combining);
    }
}
