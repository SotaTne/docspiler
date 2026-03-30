use crate::support::{child_elements, first_text_child, parse};
use mathml_core::mathml_xml::MathMlElementName;

// Section: 3.2.6 String Literal <ms>
// Spec: https://www.w3.org/TR/mathml-core/#string-literal-ms
//
// What this file proves:
// - <ms> is recognized as a typed string-literal element.

#[test]
fn supports_ms_element() {
    let document = parse(r#"<math><ms>"hello"</ms></math>"#);
    let ms = child_elements(&document)[0];

    assert_eq!(ms.name, MathMlElementName::Ms);
    assert_eq!(first_text_child(ms).raw, "\"hello\"");
}
