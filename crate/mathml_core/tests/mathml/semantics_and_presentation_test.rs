use crate::support::child_elements;
use crate::support::parse;
use mathml_core::mathml_xml::{MathMlAttributeName, MathMlElementName, XmlNode};

// Section: 3.7 Semantics and Presentation
// Spec: https://www.w3.org/TR/mathml-core/#semantics-and-presentation
//
// What this file proves:
// - Semantics wrapper elements are recognized in typed form.
// - Annotation variants keep their basic structure and attributes.

#[test]
fn semantics_elements_are_recognized_with_annotation_structure() {
    let document = parse(
        r#"<math><semantics><mi>x</mi><annotation encoding="text/plain">x</annotation><annotation-xml encoding="application/xml"><mrow><mi>x</mi></mrow></annotation-xml></semantics></math>"#,
    );

    let top = child_elements(&document);
    assert_eq!(top.len(), 1);
    assert_eq!(top[0].name, MathMlElementName::Semantics);

    let XmlNode::Element(annotation) = &top[0].children[1] else {
        panic!("expected <annotation>");
    };
    assert_eq!(annotation.name, MathMlElementName::Annotation);
    assert_eq!(annotation.attributes[0].name, MathMlAttributeName::Encoding);

    let XmlNode::Element(annotation_xml) = &top[0].children[2] else {
        panic!("expected <annotation-xml>");
    };
    assert_eq!(annotation_xml.name, MathMlElementName::AnnotationXml);
    assert_eq!(
        annotation_xml.attributes[0].name,
        MathMlAttributeName::Encoding
    );
}
