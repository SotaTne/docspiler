use crate::support::parse;
use mathml_core::mathml_model::{MathMlAttributeName, MathMlBoolean, ScriptLevel};

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

    let displaystyle = style
        .attribute(&MathMlAttributeName::DisplayStyle)
        .expect("displaystyle attribute");
    let scriptlevel = style
        .attribute(&MathMlAttributeName::ScriptLevel)
        .expect("scriptlevel attribute");

    assert_eq!(displaystyle.as_boolean(), Some(MathMlBoolean::True));
    assert_eq!(scriptlevel.as_scriptlevel(), Some(ScriptLevel::Absolute(1)));
}
