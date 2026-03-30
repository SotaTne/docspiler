use crate::support::parse;
use mathml_core::mathml_xml::MathMlAttributeName;

// Section: 2.1.6 The `displaystyle` and `scriptlevel` attributes
// Spec: https://www.w3.org/TR/mathml-core/#the-displaystyle-and-scriptlevel-attributes
//
// What this file proves:
// - MathML style-depth attributes are interpreted as typed attributes.

#[test]
fn supports_displaystyle_and_scriptlevel_attributes() {
    let document =
        parse(r#"<math><mstyle displaystyle="true" scriptlevel="1"><mi>x</mi></mstyle></math>"#);
    let style = crate::support::child_elements(&document)[0];

    assert_eq!(style.attributes.len(), 2);
    assert_eq!(style.attributes[0].name, MathMlAttributeName::DisplayStyle);
    assert_eq!(style.attributes[0].value, "true");
    assert_eq!(style.attributes[1].name, MathMlAttributeName::ScriptLevel);
    assert_eq!(style.attributes[1].value, "1");
}
