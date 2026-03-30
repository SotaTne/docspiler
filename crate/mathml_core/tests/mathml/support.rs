use mathml_core::mathml_xml::{
    MathMlElementName, XmlDocument, XmlElement, XmlNode, XmlText, parse_mathml_xml,
};

pub fn parse(xml: &str) -> XmlDocument {
    parse_mathml_xml(xml).expect("MathML should parse")
}

pub fn child_elements(document: &XmlDocument) -> Vec<&mathml_core::mathml_xml::XmlElement> {
    document
        .root
        .children
        .iter()
        .filter_map(|node| match node {
            XmlNode::Element(element) => Some(element),
            _ => None,
        })
        .collect()
}

pub fn child_names(document: &XmlDocument) -> Vec<MathMlElementName> {
    child_elements(document)
        .into_iter()
        .map(|element| element.name.clone())
        .collect()
}

pub fn element_children(element: &XmlElement) -> Vec<&XmlElement> {
    element
        .children
        .iter()
        .filter_map(|node| match node {
            XmlNode::Element(element) => Some(element),
            _ => None,
        })
        .collect()
}

pub fn first_text_child(element: &XmlElement) -> &XmlText {
    element
        .children
        .iter()
        .find_map(|node| match node {
            XmlNode::Text(text) => Some(text),
            _ => None,
        })
        .expect("expected text child")
}
