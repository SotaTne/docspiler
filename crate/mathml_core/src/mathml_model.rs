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
    pub(crate) value: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ParsedMathMlAttributeValue {
    Boolean(MathMlBoolean),
    UnsignedInteger(UnsignedInteger),
    ScriptLevel(ScriptLevel),
    Display(MathDisplay),
    MathVariant(MathVariantValue),
    LengthPercentage(LengthPercentage),
    LineThickness(LineThickness),
    RowSpan(RowSpan),
    ColumnSpan(ColumnSpan),
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OperatorStretchAxis {
    Inline,
    Block,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OperatorForm {
    Prefix,
    Infix,
    Postfix,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OperatorProperty {
    Fence,
    Separator,
    Stretchy,
    Symmetric,
    LargeOp,
    MovableLimits,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OperatorDictionaryEntry {
    pub content: MathMlSymbol,
    pub stretch_axis: OperatorStretchAxis,
    pub form: OperatorForm,
    pub lspace_em: f32,
    pub rspace_em: f32,
    pub properties: Vec<OperatorProperty>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccentPosition {
    Above,
    Below,
    Over,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CombiningCharacterEquivalence {
    pub base: MathMlSymbol,
    pub position: AccentPosition,
    pub combining: char,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GlyphAssemblyDirection {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct GlyphAssembly {
    pub base: MathMlSymbol,
    pub direction: GlyphAssemblyDirection,
    pub extender: MathMlSymbol,
    pub bottom_or_left: MathMlSymbol,
    pub middle: Option<MathMlSymbol>,
    pub top_or_right: Option<MathMlSymbol>,
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

impl MathMlElementName {
    pub(crate) fn from_raw_name(name: &str) -> Self {
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
    pub(crate) fn from_raw_name(name: &str) -> Self {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MathMlSymbol {
    BangEquals,
    AsteriskEquals,
    PlusEquals,
    MinusEquals,
    ArrowAscii,
    DoubleSlash,
    SlashEquals,
    ColonEquals,
    LessThanOrEqualAscii,
    DoubleEquals,
    GreaterThanOrEqualAscii,
    DoubleVerticalBarAscii,
    DoubleAmpersand,
    DoubleAsterisk,
    DoubleBang,
    DoublePlus,
    DoubleHyphen,
    LessThanGreaterThanAscii,
    Plus,
    ExclamationMark,
    PercentSign,
    Ampersand,
    Apostrophe,
    QuotationMark,
    GraveAccent,
    CircumflexAccent,
    LowLine,
    Tilde,
    Solidus,
    ReverseSolidus,
    AtSign,
    QuestionMark,
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
    LeftSingleQuotationMark,
    RightSingleQuotationMark,
    LeftDoubleQuotationMark,
    RightDoubleQuotationMark,
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
    InvisiblePlus,
    InvisibleNoBreak,
    DifferentialD,
    ExponentialE,
    SquareRoot,
    CubeRoot,
    FourthRoot,
    DegreeSign,
    SuperscriptTwo,
    SuperscriptThree,
    SuperscriptOne,
    Overline,
    Macron,
    Diaeresis,
    AcuteAccent,
    Cedilla,
    RingAbove,
    DoubleAcuteAccent,
    Breve,
    DotAbove,
    Ogonek,
    ModifierLetterCircumflexAccent,
    Caron,
    PrimeReversed,
    DotOperator,
    Bullet,
    RingOperator,
    CircledTimes,
    CircledDot,
    CircledRingOperator,
    CircledAsteriskOperator,
    BoxTimes,
    BoxDot,
    DiamondOperator,
    StarOperator,
    WhiteDiamondContainingBlackSmallDiamond,
    UnionWithDot,
    IntersectionWithDot,
    LogicalAndDouble,
    LogicalOrDouble,
    LeftFloor,
    RightFloor,
    LeftCeiling,
    RightCeiling,
    LeftDoubleBracket,
    RightDoubleBracket,
    LongRightArrow,
    LongLeftArrow,
    LongLeftRightArrow,
    GreekCapitalDelta,
    GreekCapitalSigma,
    GreekSmallAlpha,
    MathematicalItalicCapitalDelta,
    MathematicalItalicCapitalA,
    MathematicalItalicSmallH,
    MathematicalItalicSmallAlpha,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MathMlBoolean {
    True,
    False,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct UnsignedInteger(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptLevel {
    Absolute(u32),
    Add(i32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MathDisplay {
    Inline,
    Block,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MathVariantValue {
    Normal,
    Other(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RowSpan(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ColumnSpan(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MathLengthUnit {
    Unitless,
    Em,
    Ex,
    Px,
    In,
    Cm,
    Mm,
    Pt,
    Pc,
    Q,
    Ch,
    Rem,
    Lh,
    Rlh,
    Vw,
    Vh,
    Vmin,
    Vmax,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct MathLength {
    pub value: f32,
    pub unit: MathLengthUnit,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum LengthPercentage {
    Length(MathLength),
    Percentage(f32),
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct LineThickness(pub LengthPercentage);

impl XmlElement {
    pub fn attribute(&self, name: &MathMlAttributeName) -> Option<&XmlAttribute> {
        self.attributes
            .iter()
            .find(|attribute| &attribute.name == name)
    }
}

impl XmlAttribute {
    pub(crate) fn new(name: MathMlAttributeName, value: String) -> Self {
        Self { name, value }
    }
}

#[derive(Debug)]
pub enum ParseXmlError {
    Xml(quick_xml::Error),
    Attr(quick_xml::events::attributes::AttrError),
    Encoding(quick_xml::encoding::EncodingError),
    Escape(quick_xml::escape::EscapeError),
    Utf8(std::str::Utf8Error),
    InvalidReference(String),
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
            Self::InvalidReference(reference) => {
                write!(f, "invalid xml general reference: {reference}")
            }
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
