use crate::support::{child_elements, first_text_child, parse};
use mathml_core::mathml_xml::MathMlElementName;

// Section: 3.2.1 Text <mtext>
// Spec: https://www.w3.org/TR/mathml-core/#text-mtext
//
// What this file proves:
// - <mtext> is recognized as a typed token element.
// - Plain text content is preserved.

#[test]
fn supports_mtext_element() {
    let document = parse(r#"<math><mtext>Hello world</mtext></math>"#);
    let mtext = child_elements(&document)[0];

    assert_eq!(mtext.name, MathMlElementName::Mtext);
    assert_eq!(first_text_child(mtext).raw, "Hello world");
}
