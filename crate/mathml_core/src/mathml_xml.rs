use quick_xml::Reader;
use quick_xml::events::{BytesStart, Event};

pub use crate::mathml_model::{
    AccentPosition, CombiningCharacterEquivalence, GlyphAssembly, GlyphAssemblyDirection,
    MathMlAttributeName, MathMlElementName, MathMlSymbol, OperatorDictionaryEntry, OperatorForm,
    OperatorProperty, OperatorStretchAxis, ParseXmlError, TextToken, XmlAttribute, XmlDocument,
    XmlElement, XmlNode, XmlText,
};

pub fn parse_mathml_xml(xml: &str) -> Result<XmlDocument, ParseXmlError> {
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(false);

    let mut stack: Vec<XmlElement> = Vec::new();
    let mut root: Option<XmlElement> = None;

    loop {
        match reader.read_event()? {
            Event::Start(start) => stack.push(start_to_element(&reader, &start)?),
            Event::Empty(start) => {
                let element = start_to_element(&reader, &start)?;
                push_node(&mut stack, &mut root, XmlNode::Element(element))?;
            }
            Event::End(end) => {
                let found = decode_element_name(end.name().as_ref())?;
                let element = stack.pop().ok_or(ParseXmlError::NoRootElement)?;

                if element.name != found {
                    return Err(ParseXmlError::MismatchedEndTag {
                        expected: element.name,
                        found,
                    });
                }

                if let Some(parent) = stack.last_mut() {
                    parent.children.push(XmlNode::Element(element));
                } else if root.replace(element).is_some() {
                    return Err(ParseXmlError::MultipleRootElements);
                }
            }
            Event::Text(text) => {
                push_text_segment(&mut stack, &mut root, text.xml_content()?.into_owned())?
            }
            Event::CData(text) => {
                let value = text.decode()?.into_owned();
                push_node(&mut stack, &mut root, XmlNode::Cdata(value))?;
            }
            Event::Comment(text) => {
                let value = text.decode()?.into_owned();
                push_node(&mut stack, &mut root, XmlNode::Comment(value))?;
            }
            Event::GeneralRef(reference) => {
                let value = resolve_general_reference(&reference)?;
                push_text_segment(&mut stack, &mut root, value)?;
            }
            Event::Decl(_) | Event::PI(_) | Event::DocType(_) => {}
            Event::Eof => break,
        }
    }

    if !stack.is_empty() {
        return Err(ParseXmlError::NoRootElement);
    }

    let root = root.ok_or(ParseXmlError::NoRootElement)?;
    Ok(XmlDocument { root })
}

fn start_to_element(
    reader: &Reader<&[u8]>,
    start: &BytesStart<'_>,
) -> Result<XmlElement, ParseXmlError> {
    let name = decode_element_name(start.name().as_ref())?;
    let mut attributes = Vec::new();

    for attr in start.attributes() {
        let attr = attr?;
        attributes.push(XmlAttribute::new(
            decode_attribute_name(attr.key.as_ref())?,
            attr.decode_and_unescape_value(reader.decoder())?
                .into_owned(),
        ));
    }

    Ok(XmlElement {
        name,
        attributes,
        children: Vec::new(),
    })
}

fn decode_name(bytes: &[u8]) -> Result<String, ParseXmlError> {
    Ok(std::str::from_utf8(bytes)?.to_owned())
}

fn decode_element_name(bytes: &[u8]) -> Result<MathMlElementName, ParseXmlError> {
    Ok(MathMlElementName::from_raw_name(&decode_name(bytes)?))
}

fn decode_attribute_name(bytes: &[u8]) -> Result<MathMlAttributeName, ParseXmlError> {
    Ok(MathMlAttributeName::from_raw_name(&decode_name(bytes)?))
}

fn push_node(
    stack: &mut [XmlElement],
    root: &mut Option<XmlElement>,
    node: XmlNode,
) -> Result<(), ParseXmlError> {
    if let Some(parent) = stack.last_mut() {
        parent.children.push(node);
        Ok(())
    } else {
        match node {
            XmlNode::Element(element) => {
                if root.replace(element).is_some() {
                    Err(ParseXmlError::MultipleRootElements)
                } else {
                    Ok(())
                }
            }
            XmlNode::Text(text) if text.raw.trim().is_empty() => Ok(()),
            XmlNode::Comment(_) => Ok(()),
            XmlNode::Cdata(_) | XmlNode::Text(_) => Err(ParseXmlError::NoRootElement),
        }
    }
}

fn push_text_segment(
    stack: &mut [XmlElement],
    root: &mut Option<XmlElement>,
    segment: String,
) -> Result<(), ParseXmlError> {
    if segment.is_empty() {
        return Ok(());
    }

    if let Some(parent) = stack.last_mut() {
        if let Some(XmlNode::Text(existing)) = parent.children.last_mut() {
            existing.raw.push_str(&segment);
            *existing = parse_xml_text(existing.raw.clone());
        } else {
            parent.children.push(XmlNode::Text(parse_xml_text(segment)));
        }
        return Ok(());
    }

    if segment.trim().is_empty() {
        Ok(())
    } else {
        match root {
            Some(_) => Err(ParseXmlError::MultipleRootElements),
            None => Err(ParseXmlError::NoRootElement),
        }
    }
}

fn resolve_general_reference(
    reference: &quick_xml::events::BytesRef<'_>,
) -> Result<String, ParseXmlError> {
    let decoded = reference.decode()?.into_owned();

    if let Some(hex) = decoded
        .strip_prefix("#x")
        .or_else(|| decoded.strip_prefix("#X"))
    {
        let codepoint = u32::from_str_radix(hex, 16)
            .ok()
            .and_then(char::from_u32)
            .ok_or_else(|| ParseXmlError::InvalidReference(decoded.clone()))?;
        return Ok(codepoint.to_string());
    }

    if let Some(decimal) = decoded.strip_prefix('#') {
        let codepoint = decimal
            .parse::<u32>()
            .ok()
            .and_then(char::from_u32)
            .ok_or_else(|| ParseXmlError::InvalidReference(decoded.clone()))?;
        return Ok(codepoint.to_string());
    }

    match decoded.as_str() {
        "lt" => Ok("<".to_string()),
        "gt" => Ok(">".to_string()),
        "amp" => Ok("&".to_string()),
        "apos" => Ok("'".to_string()),
        "quot" => Ok("\"".to_string()),
        _ => Err(ParseXmlError::InvalidReference(decoded)),
    }
}

fn parse_xml_text(raw: String) -> XmlText {
    let chars: Vec<char> = raw.chars().collect();
    let mut tokens = Vec::new();
    let mut i = 0;

    while i < chars.len() {
        if let Some((symbol, consumed)) = MathMlSymbol::from_char_sequence(&chars[i..]) {
            tokens.push(TextToken::Symbol(symbol));
            i += consumed;
            continue;
        }

        tokens.push(match MathMlSymbol::from_char(chars[i]) {
            Some(symbol) => TextToken::Symbol(symbol),
            None => TextToken::Char(chars[i]),
        });
        i += 1;
    }

    XmlText { raw, tokens }
}

impl MathMlElementName {
    fn from_raw_name(name: &str) -> Self {
        match name {
            "math" => Self::Math,
            "mi" => Self::Mi,
            "mn" => Self::Mn,
            "mo" => Self::Mo,
            "mtext" => Self::Mtext,
            "mspace" => Self::Mspace,
            "ms" => Self::Ms,
            "mrow" => Self::Mrow,
            "mfrac" => Self::Mfrac,
            "msqrt" => Self::Msqrt,
            "mroot" => Self::Mroot,
            "mstyle" => Self::Mstyle,
            "merror" => Self::Merror,
            "mpadded" => Self::Mpadded,
            "mphantom" => Self::Mphantom,
            "msub" => Self::Msub,
            "msup" => Self::Msup,
            "msubsup" => Self::Msubsup,
            "munder" => Self::Munder,
            "mover" => Self::Mover,
            "munderover" => Self::Munderover,
            "mmultiscripts" => Self::Mmultiscripts,
            "mprescripts" => Self::Mprescripts,
            "mtable" => Self::Mtable,
            "mtr" => Self::Mtr,
            "mtd" => Self::Mtd,
            "semantics" => Self::Semantics,
            "annotation" => Self::Annotation,
            "annotation-xml" => Self::AnnotationXml,
            _ => Self::Unknown(name.to_owned()),
        }
    }
}

impl MathMlAttributeName {
    fn from_raw_name(name: &str) -> Self {
        match name {
            "display" => Self::Display,
            "displaystyle" => Self::DisplayStyle,
            "scriptlevel" => Self::ScriptLevel,
            "scriptsizemultiplier" => Self::ScriptSizeMultiplier,
            "scriptminsize" => Self::ScriptMinSize,
            "mathvariant" => Self::MathVariant,
            "mathsize" => Self::MathSize,
            "mathcolor" => Self::MathColor,
            "mathbackground" => Self::MathBackground,
            "dir" => Self::Dir,
            "form" => Self::Form,
            "fence" => Self::Fence,
            "separator" => Self::Separator,
            "stretchy" => Self::Stretchy,
            "symmetric" => Self::Symmetric,
            "largeop" => Self::LargeOp,
            "movablelimits" => Self::MovableLimits,
            "accent" => Self::Accent,
            "accentunder" => Self::AccentUnder,
            "linebreak" => Self::LineBreak,
            "linebreakstyle" => Self::LineBreakStyle,
            "lspace" => Self::LSpace,
            "rspace" => Self::RSpace,
            "minsize" => Self::MinSize,
            "maxsize" => Self::MaxSize,
            "width" => Self::Width,
            "height" => Self::Height,
            "depth" => Self::Depth,
            "linethickness" => Self::LineThickness,
            "numalign" => Self::NumAlign,
            "denomalign" => Self::DenomAlign,
            "bevelled" => Self::Bevelled,
            "open" => Self::Open,
            "close" => Self::Close,
            "separators" => Self::Separators,
            "rowalign" => Self::RowAlign,
            "columnalign" => Self::ColumnAlign,
            "groupalign" => Self::GroupAlign,
            "align" => Self::Align,
            "columnspacing" => Self::ColumnSpacing,
            "rowspacing" => Self::RowSpacing,
            "columnlines" => Self::ColumnLines,
            "rowlines" => Self::RowLines,
            "frame" => Self::Frame,
            "framespacing" => Self::FrameSpacing,
            "equalrows" => Self::EqualRows,
            "equalcolumns" => Self::EqualColumns,
            "side" => Self::Side,
            "minlabelspacing" => Self::MinLabelSpacing,
            "rowspan" => Self::RowSpan,
            "columnspan" => Self::ColumnSpan,
            "encoding" => Self::Encoding,
            "definitionURL" => Self::DefinitionUrl,
            "src" => Self::Src,
            "alttext" => Self::AltText,
            "altimg" => Self::AltImg,
            "altimg-width" => Self::AltImgWidth,
            "altimg-height" => Self::AltImgHeight,
            _ => Self::Unknown(name.to_owned()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_preserves_mathml_elements_with_typed_names() {
        let xml = r#"<math display="block"><mfrac><mi>x</mi><mn>2</mn></mfrac></math>"#;

        let document = parse_mathml_xml(xml).expect("mathml xml should parse");

        assert_eq!(document.root.name, MathMlElementName::Math);
        let display = document
            .root
            .attribute(&MathMlAttributeName::Display)
            .expect("display attribute");
        assert_eq!(
            display.as_display(),
            Some(crate::mathml_model::MathDisplay::Block)
        );

        let XmlNode::Element(mfrac) = &document.root.children[0] else {
            panic!("expected mfrac element");
        };

        assert_eq!(mfrac.name, MathMlElementName::Mfrac);
    }

    #[test]
    fn parse_tokenizes_known_math_symbols_in_text() {
        let xml = "<math><mo>∑</mo><mo>\u{2062}</mo><mi>x</mi></math>";

        let document = parse_mathml_xml(xml).expect("mathml xml should parse");

        let XmlNode::Element(sum) = &document.root.children[0] else {
            panic!("expected mo element");
        };
        let XmlNode::Text(sum_text) = &sum.children[0] else {
            panic!("expected text");
        };
        assert_eq!(
            sum_text.tokens,
            vec![TextToken::Symbol(MathMlSymbol::Summation)]
        );

        let XmlNode::Element(invisible_times) = &document.root.children[1] else {
            panic!("expected mo element");
        };
        let XmlNode::Text(invisible_times_text) = &invisible_times.children[0] else {
            panic!("expected text");
        };
        assert_eq!(
            invisible_times_text.tokens,
            vec![TextToken::Symbol(MathMlSymbol::InvisibleTimes)]
        );
    }

    #[test]
    fn parse_preserves_comments_cdata_and_unknown_names() {
        let xml =
            r#"<math><mtext>Hello</mtext><!--note--><![CDATA[raw]]><x-foo custom="1"/></math>"#;

        let document = parse_mathml_xml(xml).expect("mathml xml should parse");

        assert!(matches!(
            &document.root.children[0],
            XmlNode::Element(XmlElement {
                name: MathMlElementName::Mtext,
                ..
            })
        ));
        assert!(matches!(&document.root.children[1], XmlNode::Comment(text) if text == "note"));
        assert!(matches!(&document.root.children[2], XmlNode::Cdata(text) if text == "raw"));
        assert!(matches!(
            &document.root.children[3],
            XmlNode::Element(XmlElement {
                name: MathMlElementName::Unknown(name),
                attributes,
                ..
            }) if name == "x-foo"
                && attributes[0].name == MathMlAttributeName::Unknown("custom".to_string())
        ));
    }
}
