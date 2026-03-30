use quick_xml::Reader;
use quick_xml::events::{BytesStart, Event};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct XmlDocument {
    pub root: XmlElement,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct XmlElement {
    pub name: MathMlElementName,
    pub attributes: Vec<XmlAttribute>,
    pub children: Vec<XmlNode>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct XmlAttribute {
    pub name: MathMlAttributeName,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum XmlNode {
    Element(XmlElement),
    Text(XmlText),
    Cdata(String),
    Comment(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct XmlText {
    pub raw: String,
    pub tokens: Vec<TextToken>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextToken {
    Symbol(MathMlSymbol),
    Char(char),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MathMlElementName {
    Math,
    Mi,
    Mn,
    Mo,
    Mtext,
    Mspace,
    Ms,
    Mrow,
    Mfrac,
    Msqrt,
    Mroot,
    Mstyle,
    Merror,
    Mpadded,
    Mphantom,
    Msub,
    Msup,
    Msubsup,
    Munder,
    Mover,
    Munderover,
    Mmultiscripts,
    Mprescripts,
    Mtable,
    Mtr,
    Mtd,
    Semantics,
    Annotation,
    AnnotationXml,
    Unknown(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MathMlAttributeName {
    Display,
    DisplayStyle,
    ScriptLevel,
    ScriptSizeMultiplier,
    ScriptMinSize,
    MathVariant,
    MathSize,
    MathColor,
    MathBackground,
    Dir,
    Form,
    Fence,
    Separator,
    Stretchy,
    Symmetric,
    LargeOp,
    MovableLimits,
    Accent,
    AccentUnder,
    LineBreak,
    LineBreakStyle,
    LSpace,
    RSpace,
    MinSize,
    MaxSize,
    Width,
    Height,
    Depth,
    LineThickness,
    NumAlign,
    DenomAlign,
    Bevelled,
    Open,
    Close,
    Separators,
    RowAlign,
    ColumnAlign,
    GroupAlign,
    Align,
    ColumnSpacing,
    RowSpacing,
    ColumnLines,
    RowLines,
    Frame,
    FrameSpacing,
    EqualRows,
    EqualColumns,
    Side,
    MinLabelSpacing,
    RowSpan,
    ColumnSpan,
    Encoding,
    DefinitionUrl,
    Src,
    AltText,
    AltImg,
    AltImgWidth,
    AltImgHeight,
    Unknown(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MathMlSymbol {
    Plus,
    HyphenMinus,
    MinusSign,
    PlusMinus,
    MinusPlus,
    MultiplicationSign,
    DivisionSign,
    Asterisk,
    Equals,
    NotEquals,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    MuchLessThan,
    MuchGreaterThan,
    LeftParenthesis,
    RightParenthesis,
    LeftSquareBracket,
    RightSquareBracket,
    LeftCurlyBrace,
    RightCurlyBrace,
    VerticalBar,
    DoubleVerticalBar,
    LeftAngleBracket,
    RightAngleBracket,
    Comma,
    Dot,
    MiddleDot,
    Colon,
    Semicolon,
    Prime,
    DoublePrime,
    TriplePrime,
    Summation,
    Product,
    Coproduct,
    Integral,
    DoubleIntegral,
    TripleIntegral,
    PartialDifferential,
    Nabla,
    Infinity,
    ProportionalTo,
    ElementOf,
    NotElementOf,
    ContainsAsMember,
    SubsetOf,
    SupersetOf,
    SubsetOfOrEqualTo,
    SupersetOfOrEqualTo,
    Union,
    Intersection,
    LogicalAnd,
    LogicalOr,
    ForAll,
    Exists,
    Therefore,
    Because,
    LeftArrow,
    RightArrow,
    LeftRightArrow,
    UpArrow,
    DownArrow,
    Implies,
    Equivalent,
    InvisibleTimes,
    InvisibleComma,
    ApplyFunction,
}

#[derive(Debug)]
pub enum ParseXmlError {
    Xml(quick_xml::Error),
    Attr(quick_xml::events::attributes::AttrError),
    Encoding(quick_xml::encoding::EncodingError),
    Escape(quick_xml::escape::EscapeError),
    Utf8(std::str::Utf8Error),
    NoRootElement,
    MultipleRootElements,
    MismatchedEndTag {
        expected: MathMlElementName,
        found: MathMlElementName,
    },
}

impl fmt::Display for ParseXmlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Xml(err) => write!(f, "xml parse error: {err}"),
            Self::Attr(err) => write!(f, "xml attribute error: {err}"),
            Self::Encoding(err) => write!(f, "xml encoding error: {err}"),
            Self::Escape(err) => write!(f, "xml escape error: {err}"),
            Self::Utf8(err) => write!(f, "xml utf-8 error: {err}"),
            Self::NoRootElement => write!(f, "no root element found"),
            Self::MultipleRootElements => write!(f, "multiple root elements found"),
            Self::MismatchedEndTag { expected, found } => {
                write!(
                    f,
                    "mismatched end tag: expected {expected:?}, found {found:?}"
                )
            }
        }
    }
}

impl std::error::Error for ParseXmlError {}

impl From<quick_xml::Error> for ParseXmlError {
    fn from(value: quick_xml::Error) -> Self {
        Self::Xml(value)
    }
}

impl From<quick_xml::events::attributes::AttrError> for ParseXmlError {
    fn from(value: quick_xml::events::attributes::AttrError) -> Self {
        Self::Attr(value)
    }
}

impl From<quick_xml::encoding::EncodingError> for ParseXmlError {
    fn from(value: quick_xml::encoding::EncodingError) -> Self {
        Self::Encoding(value)
    }
}

impl From<quick_xml::escape::EscapeError> for ParseXmlError {
    fn from(value: quick_xml::escape::EscapeError) -> Self {
        Self::Escape(value)
    }
}

impl From<std::str::Utf8Error> for ParseXmlError {
    fn from(value: std::str::Utf8Error) -> Self {
        Self::Utf8(value)
    }
}

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
                let value = text.xml_content()?.into_owned();
                if !value.is_empty() {
                    push_node(&mut stack, &mut root, XmlNode::Text(parse_xml_text(value)))?;
                }
            }
            Event::CData(text) => {
                let value = text.decode()?.into_owned();
                push_node(&mut stack, &mut root, XmlNode::Cdata(value))?;
            }
            Event::Comment(text) => {
                let value = text.decode()?.into_owned();
                push_node(&mut stack, &mut root, XmlNode::Comment(value))?;
            }
            Event::Decl(_) | Event::PI(_) | Event::DocType(_) | Event::GeneralRef(_) => {}
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

fn parse_xml_text(raw: String) -> XmlText {
    let tokens = raw
        .chars()
        .map(|ch| match MathMlSymbol::from_char(ch) {
            Some(symbol) => TextToken::Symbol(symbol),
            None => TextToken::Char(ch),
        })
        .collect();

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
    fn from_char(ch: char) -> Option<Self> {
        match ch {
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
            '\u{2062}' => Some(Self::InvisibleTimes),
            '\u{2063}' => Some(Self::InvisibleComma),
            '\u{2061}' => Some(Self::ApplyFunction),
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
