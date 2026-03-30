use mathml_core::mathml_xml::MathMlSymbol;

// Appendix: C.1 italic mappings
// Spec: https://www.w3.org/TR/mathml-core/#italic-mappings
//
// What this file proves:
// - Native symbols can expose representative Unicode math-italic mappings.

#[test]
fn supports_representative_italic_mappings() {
    assert_eq!(
        MathMlSymbol::GreekCapitalDelta.mathematical_italic_variant(),
        Some(MathMlSymbol::MathematicalItalicCapitalDelta)
    );
    assert_eq!(
        MathMlSymbol::GreekSmallAlpha.mathematical_italic_variant(),
        Some(MathMlSymbol::MathematicalItalicSmallAlpha)
    );
}
