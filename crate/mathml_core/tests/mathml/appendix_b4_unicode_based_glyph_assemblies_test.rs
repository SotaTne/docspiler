use mathml_core::mathml_xml::{GlyphAssemblyDirection, MathMlSymbol};

// Appendix: B.4 Unicode-based Glyph Assemblies
// Spec: https://www.w3.org/TR/mathml-core/#unicode-based-glyph-assemblies
//
// What this file proves:
// - Native symbols can expose fallback glyph assembly metadata for stretchable
//   operators without relying on string matching after parse.

#[test]
fn supports_unicode_based_glyph_assemblies() {
    let left_paren = MathMlSymbol::LeftParenthesis
        .glyph_assembly()
        .expect("left parenthesis should have a glyph assembly");
    assert_eq!(left_paren.direction, GlyphAssemblyDirection::Vertical);
    assert_eq!(left_paren.extender, MathMlSymbol::VerticalBar);
    assert_eq!(left_paren.bottom_or_left, MathMlSymbol::LeftFloor);
    assert_eq!(left_paren.top_or_right, Some(MathMlSymbol::LeftCeiling));

    let right_arrow = MathMlSymbol::RightArrow
        .glyph_assembly()
        .expect("right arrow should have a glyph assembly");
    assert_eq!(right_arrow.direction, GlyphAssemblyDirection::Horizontal);
    assert_eq!(right_arrow.extender, MathMlSymbol::Overline);
    assert_eq!(right_arrow.bottom_or_left, MathMlSymbol::Overline);
    assert_eq!(right_arrow.top_or_right, Some(MathMlSymbol::RightArrow));
}
