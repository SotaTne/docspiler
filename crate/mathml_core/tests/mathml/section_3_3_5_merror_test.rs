use crate::support::{child_elements, parse};
use mathml_core::mathml_xml::MathMlElementName;

// Section: 3.3.5 Error Message <merror>
// Spec: https://www.w3.org/TR/mathml-core/#error-message-merror
//
// What this file proves:
// - <merror> is recognized as a typed error container.

#[test]
fn supports_merror_element() {
    let document = parse(r#"<math><merror><mtext>bad</mtext></merror></math>"#);
    let merror = child_elements(&document)[0];

    assert_eq!(merror.name, MathMlElementName::Merror);
}
