use crate::support::{child_elements, parse};
use mathml_core::mathml_xml::{MathMlAttributeName, MathMlElementName};

// Section: 3.2.5 Space <mspace>
// Spec: https://www.w3.org/TR/mathml-core/#space-mspace
//
// What this file proves:
// - <mspace> is recognized as a typed spacing element.
// - Basic sizing attributes are preserved.

#[test]
fn supports_mspace_element() {
    let document = parse(r#"<math><mspace width="1em" height="2ex" depth="0.5ex"/></math>"#);
    let mspace = child_elements(&document)[0];

    assert_eq!(mspace.name, MathMlElementName::Mspace);
    assert_eq!(mspace.attributes[0].name, MathMlAttributeName::Width);
    assert_eq!(mspace.attributes[1].name, MathMlAttributeName::Height);
    assert_eq!(mspace.attributes[2].name, MathMlAttributeName::Depth);
}
