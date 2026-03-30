use crate::support::{child_elements, first_text_child, parse};
use mathml_core::mathml_xml::{MathMlElementName, MathMlSymbol, TextToken};

// Section: 3.2.4 Operator, Fence, Separator or Accent <mo>
// Spec: https://www.w3.org/TR/mathml-core/#operator-fence-separator-or-accent-mo
//
// What this file proves:
// - <mo> is recognized as a typed operator element.
// - Basic operator text is interpreted into symbol tokens.

#[test]
fn supports_mo_element() {
    let document = parse(r#"<math><mo>+</mo></math>"#);
    let mo = child_elements(&document)[0];

    assert_eq!(mo.name, MathMlElementName::Mo);
    assert_eq!(
        first_text_child(mo).tokens,
        vec![TextToken::Symbol(MathMlSymbol::Plus)]
    );
}
