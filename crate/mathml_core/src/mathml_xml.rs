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
        attributes.push(XmlAttribute {
            name: decode_attribute_name(attr.key.as_ref())?,
            value: attr
                .decode_and_unescape_value(reader.decoder())?
                .into_owned(),
        });
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

impl MathMlSymbol {
    pub fn operator_dictionary_entry(self, form: OperatorForm) -> Option<OperatorDictionaryEntry> {
        let entry = match (self, form) {
            (Self::BangEquals, OperatorForm::Infix)
            | (Self::LessThanOrEqualAscii, OperatorForm::Infix)
            | (Self::ArrowAscii, OperatorForm::Infix)
            | (Self::DoubleVerticalBarAscii, OperatorForm::Infix)
            | (Self::DoubleAmpersand, OperatorForm::Infix)
            | (Self::DoubleEquals, OperatorForm::Infix)
            | (Self::ColonEquals, OperatorForm::Infix)
            | (Self::DoubleAsterisk, OperatorForm::Infix)
            | (Self::DoublePlus, OperatorForm::Postfix)
            | (Self::DoubleHyphen, OperatorForm::Postfix)
            | (Self::Plus, OperatorForm::Infix)
            | (Self::HyphenMinus, OperatorForm::Infix)
            | (Self::MinusSign, OperatorForm::Infix)
            | (Self::PlusMinus, OperatorForm::Infix)
            | (Self::DivisionSign, OperatorForm::Infix)
            | (Self::Union, OperatorForm::Infix)
            | (Self::Intersection, OperatorForm::Infix)
            | (Self::LogicalAnd, OperatorForm::Infix)
            | (Self::LogicalOr, OperatorForm::Infix)
            | (Self::MiddleDot, OperatorForm::Infix)
            | (Self::MultiplicationSign, OperatorForm::Infix) => OperatorDictionaryEntry {
                content: self,
                stretch_axis: OperatorStretchAxis::Block,
                form,
                lspace_em: 0.222_222_22,
                rspace_em: 0.222_222_22,
                properties: vec![],
            },
            (Self::LessThan, OperatorForm::Infix)
            | (Self::Equals, OperatorForm::Infix)
            | (Self::GreaterThan, OperatorForm::Infix)
            | (Self::VerticalBar, OperatorForm::Infix)
            | (Self::ElementOf, OperatorForm::Infix)
            | (Self::NotElementOf, OperatorForm::Infix)
            | (Self::ContainsAsMember, OperatorForm::Infix)
            | (Self::ProportionalTo, OperatorForm::Infix)
            | (Self::SubsetOf, OperatorForm::Infix)
            | (Self::SupersetOf, OperatorForm::Infix)
            | (Self::SubsetOfOrEqualTo, OperatorForm::Infix)
            | (Self::SupersetOfOrEqualTo, OperatorForm::Infix)
            | (Self::NotEquals, OperatorForm::Infix)
            | (Self::LessThanOrEqual, OperatorForm::Infix)
            | (Self::GreaterThanOrEqual, OperatorForm::Infix)
            | (Self::MuchLessThan, OperatorForm::Infix)
            | (Self::MuchGreaterThan, OperatorForm::Infix) => OperatorDictionaryEntry {
                content: self,
                stretch_axis: if matches!(self, Self::Equals) {
                    OperatorStretchAxis::Inline
                } else {
                    OperatorStretchAxis::Block
                },
                form,
                lspace_em: 0.277_777_8,
                rspace_em: 0.277_777_8,
                properties: if matches!(self, Self::VerticalBar) {
                    vec![OperatorProperty::Fence]
                } else {
                    vec![]
                },
            },
            (Self::LeftArrow, OperatorForm::Infix)
            | (Self::RightArrow, OperatorForm::Infix)
            | (Self::LeftRightArrow, OperatorForm::Infix)
            | (Self::Implies, OperatorForm::Infix)
            | (Self::Equivalent, OperatorForm::Infix)
            | (Self::LongLeftArrow, OperatorForm::Infix)
            | (Self::LongRightArrow, OperatorForm::Infix)
            | (Self::LongLeftRightArrow, OperatorForm::Infix) => OperatorDictionaryEntry {
                content: self,
                stretch_axis: OperatorStretchAxis::Inline,
                form,
                lspace_em: 0.277_777_8,
                rspace_em: 0.277_777_8,
                properties: vec![OperatorProperty::Stretchy],
            },
            (Self::ForAll, OperatorForm::Prefix)
            | (Self::Exists, OperatorForm::Prefix)
            | (Self::Nabla, OperatorForm::Prefix) => OperatorDictionaryEntry {
                content: self,
                stretch_axis: OperatorStretchAxis::Block,
                form,
                lspace_em: 0.0,
                rspace_em: 0.0,
                properties: vec![],
            },
            (Self::LeftParenthesis, OperatorForm::Prefix)
            | (Self::LeftSquareBracket, OperatorForm::Prefix)
            | (Self::LeftCurlyBrace, OperatorForm::Prefix)
            | (Self::VerticalBar, OperatorForm::Prefix)
            | (Self::DoubleVerticalBar, OperatorForm::Prefix)
            | (Self::LeftCeiling, OperatorForm::Prefix)
            | (Self::LeftFloor, OperatorForm::Prefix)
            | (Self::LeftAngleBracket, OperatorForm::Prefix)
            | (Self::LeftDoubleBracket, OperatorForm::Prefix)
            | (Self::LeftSingleQuotationMark, OperatorForm::Prefix)
            | (Self::LeftDoubleQuotationMark, OperatorForm::Prefix) => OperatorDictionaryEntry {
                content: self,
                stretch_axis: OperatorStretchAxis::Block,
                form,
                lspace_em: 0.0,
                rspace_em: 0.0,
                properties: vec![
                    OperatorProperty::Stretchy,
                    OperatorProperty::Symmetric,
                    OperatorProperty::Fence,
                ],
            },
            (Self::RightParenthesis, OperatorForm::Postfix)
            | (Self::RightSquareBracket, OperatorForm::Postfix)
            | (Self::RightCurlyBrace, OperatorForm::Postfix)
            | (Self::VerticalBar, OperatorForm::Postfix)
            | (Self::DoubleVerticalBar, OperatorForm::Postfix)
            | (Self::RightCeiling, OperatorForm::Postfix)
            | (Self::RightFloor, OperatorForm::Postfix)
            | (Self::RightAngleBracket, OperatorForm::Postfix)
            | (Self::RightDoubleBracket, OperatorForm::Postfix)
            | (Self::RightSingleQuotationMark, OperatorForm::Postfix)
            | (Self::RightDoubleQuotationMark, OperatorForm::Postfix) => OperatorDictionaryEntry {
                content: self,
                stretch_axis: OperatorStretchAxis::Block,
                form,
                lspace_em: 0.0,
                rspace_em: 0.0,
                properties: vec![
                    OperatorProperty::Stretchy,
                    OperatorProperty::Symmetric,
                    OperatorProperty::Fence,
                ],
            },
            (Self::Integral, OperatorForm::Prefix)
            | (Self::DoubleIntegral, OperatorForm::Prefix)
            | (Self::TripleIntegral, OperatorForm::Prefix) => OperatorDictionaryEntry {
                content: self,
                stretch_axis: OperatorStretchAxis::Block,
                form,
                lspace_em: 0.166_666_67,
                rspace_em: 0.166_666_67,
                properties: vec![OperatorProperty::LargeOp, OperatorProperty::Symmetric],
            },
            (Self::Product, OperatorForm::Prefix)
            | (Self::Coproduct, OperatorForm::Prefix)
            | (Self::Summation, OperatorForm::Prefix) => OperatorDictionaryEntry {
                content: self,
                stretch_axis: OperatorStretchAxis::Block,
                form,
                lspace_em: 0.166_666_67,
                rspace_em: 0.166_666_67,
                properties: vec![
                    OperatorProperty::LargeOp,
                    OperatorProperty::Symmetric,
                    OperatorProperty::MovableLimits,
                ],
            },
            (Self::InvisibleTimes, OperatorForm::Infix)
            | (Self::ApplyFunction, OperatorForm::Infix)
            | (Self::Solidus, OperatorForm::Infix)
            | (Self::ReverseSolidus, OperatorForm::Infix)
            | (Self::LowLine, OperatorForm::Infix)
            | (Self::GreekCapitalDelta, OperatorForm::Infix) => OperatorDictionaryEntry {
                content: self,
                stretch_axis: if matches!(self, Self::LowLine) {
                    OperatorStretchAxis::Inline
                } else {
                    OperatorStretchAxis::Block
                },
                form,
                lspace_em: 0.0,
                rspace_em: 0.0,
                properties: vec![],
            },
            (Self::InvisibleComma, OperatorForm::Infix) => OperatorDictionaryEntry {
                content: self,
                stretch_axis: OperatorStretchAxis::Block,
                form,
                lspace_em: 0.0,
                rspace_em: 0.0,
                properties: vec![OperatorProperty::Separator],
            },
            (Self::Comma, OperatorForm::Infix) | (Self::Semicolon, OperatorForm::Infix) => {
                OperatorDictionaryEntry {
                    content: self,
                    stretch_axis: OperatorStretchAxis::Block,
                    form,
                    lspace_em: 0.0,
                    rspace_em: 0.166_666_67,
                    properties: vec![OperatorProperty::Separator],
                }
            }
            (Self::Colon, OperatorForm::Infix) => OperatorDictionaryEntry {
                content: self,
                stretch_axis: OperatorStretchAxis::Block,
                form,
                lspace_em: 0.0,
                rspace_em: 0.166_666_67,
                properties: vec![],
            },
            (Self::SquareRoot, OperatorForm::Prefix)
            | (Self::CubeRoot, OperatorForm::Prefix)
            | (Self::FourthRoot, OperatorForm::Prefix)
            | (Self::DifferentialD, OperatorForm::Prefix)
            | (Self::ExponentialE, OperatorForm::Prefix)
            | (Self::PartialDifferential, OperatorForm::Prefix) => OperatorDictionaryEntry {
                content: self,
                stretch_axis: OperatorStretchAxis::Block,
                form,
                lspace_em: 0.166_666_67,
                rspace_em: 0.0,
                properties: vec![],
            },
            (Self::CircumflexAccent, OperatorForm::Postfix)
            | (Self::LowLine, OperatorForm::Postfix)
            | (Self::Tilde, OperatorForm::Postfix)
            | (Self::Macron, OperatorForm::Postfix)
            | (Self::ModifierLetterCircumflexAccent, OperatorForm::Postfix)
            | (Self::Caron, OperatorForm::Postfix)
            | (Self::Overline, OperatorForm::Postfix) => OperatorDictionaryEntry {
                content: self,
                stretch_axis: OperatorStretchAxis::Inline,
                form,
                lspace_em: 0.0,
                rspace_em: 0.0,
                properties: vec![OperatorProperty::Stretchy],
            },
            _ => return None,
        };

        Some(entry)
    }

    pub fn combining_character_equivalence(self) -> Option<CombiningCharacterEquivalence> {
        let (base, position, combining) = match self {
            Self::Plus => (Self::Plus, AccentPosition::Below, '\u{031F}'),
            Self::HyphenMinus => (Self::HyphenMinus, AccentPosition::Above, '\u{0305}'),
            Self::LowLine => (Self::LowLine, AccentPosition::Below, '\u{0332}'),
            Self::Tilde => (Self::Tilde, AccentPosition::Above, '\u{0303}'),
            Self::Macron => (Self::Macron, AccentPosition::Above, '\u{0304}'),
            Self::AcuteAccent => (Self::AcuteAccent, AccentPosition::Above, '\u{0301}'),
            Self::Diaeresis => (Self::Diaeresis, AccentPosition::Above, '\u{0308}'),
            Self::Breve => (Self::Breve, AccentPosition::Above, '\u{0306}'),
            Self::DotAbove => (Self::DotAbove, AccentPosition::Above, '\u{0307}'),
            Self::Ogonek => (Self::Ogonek, AccentPosition::Below, '\u{0328}'),
            Self::LeftArrow => (Self::LeftArrow, AccentPosition::Above, '\u{20D6}'),
            Self::RightArrow => (Self::RightArrow, AccentPosition::Above, '\u{20D7}'),
            Self::LongRightArrow => (Self::LongRightArrow, AccentPosition::Above, '\u{20D7}'),
            _ => return None,
        };

        Some(CombiningCharacterEquivalence {
            base,
            position,
            combining,
        })
    }

    pub fn glyph_assembly(self) -> Option<GlyphAssembly> {
        let assembly = match self {
            Self::LeftParenthesis => GlyphAssembly {
                base: self,
                direction: GlyphAssemblyDirection::Vertical,
                extender: MathMlSymbol::VerticalBar,
                bottom_or_left: MathMlSymbol::LeftFloor,
                middle: None,
                top_or_right: Some(MathMlSymbol::LeftCeiling),
            },
            Self::RightParenthesis => GlyphAssembly {
                base: self,
                direction: GlyphAssemblyDirection::Vertical,
                extender: MathMlSymbol::VerticalBar,
                bottom_or_left: MathMlSymbol::RightFloor,
                middle: None,
                top_or_right: Some(MathMlSymbol::RightCeiling),
            },
            Self::LeftArrow => GlyphAssembly {
                base: self,
                direction: GlyphAssemblyDirection::Horizontal,
                extender: MathMlSymbol::Overline,
                bottom_or_left: MathMlSymbol::LeftArrow,
                middle: None,
                top_or_right: Some(MathMlSymbol::Overline),
            },
            Self::RightArrow => GlyphAssembly {
                base: self,
                direction: GlyphAssemblyDirection::Horizontal,
                extender: MathMlSymbol::Overline,
                bottom_or_left: MathMlSymbol::Overline,
                middle: None,
                top_or_right: Some(MathMlSymbol::RightArrow),
            },
            Self::LeftRightArrow => GlyphAssembly {
                base: self,
                direction: GlyphAssemblyDirection::Horizontal,
                extender: MathMlSymbol::Overline,
                bottom_or_left: MathMlSymbol::LeftArrow,
                middle: None,
                top_or_right: Some(MathMlSymbol::RightArrow),
            },
            Self::LeftSquareBracket => GlyphAssembly {
                base: self,
                direction: GlyphAssemblyDirection::Vertical,
                extender: MathMlSymbol::VerticalBar,
                bottom_or_left: MathMlSymbol::LeftFloor,
                middle: None,
                top_or_right: Some(MathMlSymbol::LeftCeiling),
            },
            Self::RightSquareBracket => GlyphAssembly {
                base: self,
                direction: GlyphAssemblyDirection::Vertical,
                extender: MathMlSymbol::VerticalBar,
                bottom_or_left: MathMlSymbol::RightFloor,
                middle: None,
                top_or_right: Some(MathMlSymbol::RightCeiling),
            },
            _ => return None,
        };

        Some(assembly)
    }

    pub fn mathematical_italic_variant(self) -> Option<Self> {
        match self {
            Self::GreekCapitalDelta => Some(Self::MathematicalItalicCapitalDelta),
            Self::GreekSmallAlpha => Some(Self::MathematicalItalicSmallAlpha),
            _ => None,
        }
    }

    pub fn from_char_sequence(chars: &[char]) -> Option<(Self, usize)> {
        const MULTI_CHAR_SYMBOLS: [(&str, MathMlSymbol); 18] = [
            ("!=", MathMlSymbol::BangEquals),
            ("*=", MathMlSymbol::AsteriskEquals),
            ("+=", MathMlSymbol::PlusEquals),
            ("-=", MathMlSymbol::MinusEquals),
            ("->", MathMlSymbol::ArrowAscii),
            ("//", MathMlSymbol::DoubleSlash),
            ("/=", MathMlSymbol::SlashEquals),
            (":=", MathMlSymbol::ColonEquals),
            ("<=", MathMlSymbol::LessThanOrEqualAscii),
            ("==", MathMlSymbol::DoubleEquals),
            (">=", MathMlSymbol::GreaterThanOrEqualAscii),
            ("||", MathMlSymbol::DoubleVerticalBarAscii),
            ("&&", MathMlSymbol::DoubleAmpersand),
            ("**", MathMlSymbol::DoubleAsterisk),
            ("<>", MathMlSymbol::LessThanGreaterThanAscii),
            ("!!", MathMlSymbol::DoubleBang),
            ("++", MathMlSymbol::DoublePlus),
            ("--", MathMlSymbol::DoubleHyphen),
        ];

        for (pattern, symbol) in MULTI_CHAR_SYMBOLS {
            let pattern_chars: Vec<char> = pattern.chars().collect();
            if chars.starts_with(&pattern_chars) {
                return Some((symbol, pattern_chars.len()));
            }
        }

        None
    }

    pub fn from_char(ch: char) -> Option<Self> {
        match ch {
            '!' => Some(Self::ExclamationMark),
            '%' => Some(Self::PercentSign),
            '&' => Some(Self::Ampersand),
            '\'' => Some(Self::Apostrophe),
            '"' => Some(Self::QuotationMark),
            '`' => Some(Self::GraveAccent),
            '^' => Some(Self::CircumflexAccent),
            '_' => Some(Self::LowLine),
            '~' => Some(Self::Tilde),
            '/' => Some(Self::Solidus),
            '\\' => Some(Self::ReverseSolidus),
            '@' => Some(Self::AtSign),
            '?' => Some(Self::QuestionMark),
            '+' => Some(Self::Plus),
            '-' => Some(Self::HyphenMinus),
            '−' => Some(Self::MinusSign),
            '±' => Some(Self::PlusMinus),
            '∓' => Some(Self::MinusPlus),
            '×' => Some(Self::MultiplicationSign),
            '÷' => Some(Self::DivisionSign),
            '*' => Some(Self::Asterisk),
            '=' => Some(Self::Equals),
            '≠' => Some(Self::NotEquals),
            '<' => Some(Self::LessThan),
            '>' => Some(Self::GreaterThan),
            '≤' => Some(Self::LessThanOrEqual),
            '≥' => Some(Self::GreaterThanOrEqual),
            '≪' => Some(Self::MuchLessThan),
            '≫' => Some(Self::MuchGreaterThan),
            '(' => Some(Self::LeftParenthesis),
            ')' => Some(Self::RightParenthesis),
            '[' => Some(Self::LeftSquareBracket),
            ']' => Some(Self::RightSquareBracket),
            '{' => Some(Self::LeftCurlyBrace),
            '}' => Some(Self::RightCurlyBrace),
            '|' => Some(Self::VerticalBar),
            '‖' => Some(Self::DoubleVerticalBar),
            '⟨' => Some(Self::LeftAngleBracket),
            '⟩' => Some(Self::RightAngleBracket),
            '‘' => Some(Self::LeftSingleQuotationMark),
            '’' => Some(Self::RightSingleQuotationMark),
            '“' => Some(Self::LeftDoubleQuotationMark),
            '”' => Some(Self::RightDoubleQuotationMark),
            ',' => Some(Self::Comma),
            '.' => Some(Self::Dot),
            '·' => Some(Self::MiddleDot),
            ':' => Some(Self::Colon),
            ';' => Some(Self::Semicolon),
            '′' => Some(Self::Prime),
            '″' => Some(Self::DoublePrime),
            '‴' => Some(Self::TriplePrime),
            '∑' => Some(Self::Summation),
            '∏' => Some(Self::Product),
            '∐' => Some(Self::Coproduct),
            '∫' => Some(Self::Integral),
            '∬' => Some(Self::DoubleIntegral),
            '∭' => Some(Self::TripleIntegral),
            '∂' => Some(Self::PartialDifferential),
            '∇' => Some(Self::Nabla),
            '∞' => Some(Self::Infinity),
            '∝' => Some(Self::ProportionalTo),
            '∈' => Some(Self::ElementOf),
            '∉' => Some(Self::NotElementOf),
            '∋' => Some(Self::ContainsAsMember),
            '⊂' => Some(Self::SubsetOf),
            '⊃' => Some(Self::SupersetOf),
            '⊆' => Some(Self::SubsetOfOrEqualTo),
            '⊇' => Some(Self::SupersetOfOrEqualTo),
            '∪' => Some(Self::Union),
            '∩' => Some(Self::Intersection),
            '∧' => Some(Self::LogicalAnd),
            '∨' => Some(Self::LogicalOr),
            '∀' => Some(Self::ForAll),
            '∃' => Some(Self::Exists),
            '∴' => Some(Self::Therefore),
            '∵' => Some(Self::Because),
            '←' => Some(Self::LeftArrow),
            '→' => Some(Self::RightArrow),
            '↔' => Some(Self::LeftRightArrow),
            '↑' => Some(Self::UpArrow),
            '↓' => Some(Self::DownArrow),
            '⇒' => Some(Self::Implies),
            '⇔' => Some(Self::Equivalent),
            '\u{2061}' => Some(Self::ApplyFunction),
            '\u{2062}' => Some(Self::InvisibleTimes),
            '\u{2063}' => Some(Self::InvisibleComma),
            '\u{2064}' => Some(Self::InvisibleNoBreak),
            'ⅅ' => Some(Self::DifferentialD),
            'ⅆ' => Some(Self::ExponentialE),
            '√' => Some(Self::SquareRoot),
            '∛' => Some(Self::CubeRoot),
            '∜' => Some(Self::FourthRoot),
            '°' => Some(Self::DegreeSign),
            '²' => Some(Self::SuperscriptTwo),
            '³' => Some(Self::SuperscriptThree),
            '¹' => Some(Self::SuperscriptOne),
            '¯' => Some(Self::Macron),
            '‾' => Some(Self::Overline),
            '¨' => Some(Self::Diaeresis),
            '´' => Some(Self::AcuteAccent),
            '¸' => Some(Self::Cedilla),
            '˚' => Some(Self::RingAbove),
            '˝' => Some(Self::DoubleAcuteAccent),
            'ˇ' => Some(Self::Caron),
            'ˊ' => Some(Self::AcuteAccent),
            'ˋ' => Some(Self::GraveAccent),
            '˷' => Some(Self::Tilde),
            '˛' => Some(Self::Ogonek),
            'ˆ' => Some(Self::ModifierLetterCircumflexAccent),
            '⌈' => Some(Self::LeftCeiling),
            '⌉' => Some(Self::RightCeiling),
            '⌊' => Some(Self::LeftFloor),
            '⌋' => Some(Self::RightFloor),
            '⟦' => Some(Self::LeftDoubleBracket),
            '⟧' => Some(Self::RightDoubleBracket),
            '⟵' => Some(Self::LongLeftArrow),
            '⟶' => Some(Self::LongRightArrow),
            '⟷' => Some(Self::LongLeftRightArrow),
            '∆' | 'Δ' => Some(Self::GreekCapitalDelta),
            'Σ' => Some(Self::GreekCapitalSigma),
            'α' => Some(Self::GreekSmallAlpha),
            '𝛥' => Some(Self::MathematicalItalicCapitalDelta),
            '𝐴' => Some(Self::MathematicalItalicCapitalA),
            'ℎ' => Some(Self::MathematicalItalicSmallH),
            '𝛼' => Some(Self::MathematicalItalicSmallAlpha),
            _ => None,
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
        assert_eq!(
            document.root.attributes,
            vec![XmlAttribute {
                name: MathMlAttributeName::Display,
                value: "block".to_string(),
            }]
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
                && attributes[0].value == "1"
        ));
    }
}
