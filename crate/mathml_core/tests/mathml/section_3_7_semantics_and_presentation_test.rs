use crate::support::{child_elements, parse};
use mathml_core::mathml_xml::{MathMlAttributeName, MathMlElementName, XmlNode};

// Section: 3.7 Semantics and Presentation
// Spec: https://www.w3.org/TR/mathml-core/#semantics-and-presentation
//
// What this file proves:
// - <semantics>, <annotation>, and <annotation-xml> are recognized in typed form.

#[test]
fn supports_semantics_and_annotation_elements() {
    let document = parse(
        r#"<math><semantics><mi>x</mi><annotation encoding="text/plain">x</annotation><annotation-xml encoding="application/xml"><mrow><mi>x</mi></mrow></annotation-xml></semantics></math>"#,
    );

    let semantics = child_elements(&document)[0];
    assert_eq!(semantics.name, MathMlElementName::Semantics);

    let XmlNode::Element(annotation) = &semantics.children[1] else {
        panic!("expected annotation element");
    };
    assert_eq!(annotation.name, MathMlElementName::Annotation);
    assert_eq!(annotation.attributes[0].name, MathMlAttributeName::Encoding);

    let XmlNode::Element(annotation_xml) = &semantics.children[2] else {
        panic!("expected annotation-xml element");
    };
    assert_eq!(annotation_xml.name, MathMlElementName::AnnotationXml);
    assert_eq!(
        annotation_xml.attributes[0].name,
        MathMlAttributeName::Encoding
    );
}
