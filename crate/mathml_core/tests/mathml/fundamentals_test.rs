use crate::support::parse;
use mathml_core::mathml_model::MathDisplay;
use mathml_core::mathml_xml::{MathMlAttributeName, MathMlElementName};

// Section: 2.1.1 The Top-Level <math> Element
// Spec: https://www.w3.org/TR/mathml-core/#the-top-level-math-element
//
// What this file proves:
// - <math> root is recognized as a typed MathML element.
// - Top-level MathML attributes are preserved in typed form.

#[test]
fn top_level_math_element_is_parsed_as_typed_root() {
    let document = parse(r#"<math display="block"></math>"#);

    assert_eq!(document.root.name, MathMlElementName::Math);
    let display = document
        .root
        .attribute(&MathMlAttributeName::Display)
        .expect("display attribute");
    assert_eq!(display.as_display(), Some(MathDisplay::Block));
}
