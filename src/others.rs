use literals::*;
use terminals::*;
use types::*;
use Parse;
use arguments::*;

/// Other ::
///     **integer**
///     **float**
///     **identifier**
///     **string**
///     **other**
///     -
///     -Infinity
///     .
///     ...
///     :
///     ;
///     <
///     =
///     >
///     ?
///     ByteString
///     DOMString
///     FrozenArray
///     Infinity
///     NaN
///     USVString
///     any
///     boolean
///     byte
///     double
///     false
///     float
///     long
///     null
///     object
///     octet
///     or
///     optional
///     sequence
///     short
///     true
///     unsigned
///     void
///     ArgumentNameKeyword
///     BufferRelatedType
#[derive(Debug, PartialEq)]
pub enum Other {
    IntegerLit(i64),
    FloatLit(f64),
    Identifier(Identifier),
    StringLit(String),
    OtherLit(OtherLit),
    Minus(Minus),
    NegInfinity(NegInfinity),
    Dot(Dot),
    Ellipsis(Ellipsis),
    Colon(Colon),
    SemiColon(SemiColon),
    LessThan(LessThan),
    Assign(Assign),
    GreaterThan(GreaterThan),
    QMark(QMark),
    ByteString(ByteString),
    DOMString(DOMString),
    FrozenArray(FrozenArray),
    Infinity(Infinity),
    NaN(NaN),
    USVString(USVString),
    Any(Any),
    Boolean(Boolean),
    Byte(Byte),
    Double(Double),
    False(False),
    Float(Float),
    Long(Long),
    Null(Null),
    Object(Object),
    Octet(Octet),
    Or(Or),
    Optional(Optional),
    Sequence(Sequence),
    Short(Short),
    True(True),
    Unsigned(Unsigned),
    Void(Void),
    ArgumentNameKeyword(ArgumentNameKeyword),
    BufferRelatedType(BufferRelatedType),
}

impl Parse for Other {
    named!(parse -> Self, alt_complete!(
        weedle!(i64) => {|inner| Other::IntegerLit(inner)} |
        weedle!(f64) => {|inner| Other::FloatLit(inner)} |
        weedle!(Identifier) => {|inner| Other::Identifier(inner)} |
        weedle!(String) => {|inner| Other::StringLit(inner)} |
        weedle!(OtherLit) => {|inner| Other::OtherLit(inner)} |
        weedle!(Minus) => {|inner| Other::Minus(inner)} |
        weedle!(NegInfinity) => {|inner| Other::NegInfinity(inner)} |
        weedle!(Dot) => {|inner| Other::Dot(inner)} |
        weedle!(Ellipsis) => {|inner| Other::Ellipsis(inner)} |
        weedle!(Colon) => {|inner| Other::Colon(inner)} |
        weedle!(SemiColon) => {|inner| Other::SemiColon(inner)} |
        weedle!(LessThan) => {|inner| Other::LessThan(inner)} |
        weedle!(Assign) => {|inner| Other::Assign(inner)} |
        weedle!(GreaterThan) => {|inner| Other::GreaterThan(inner)} |
        weedle!(QMark) => {|inner| Other::QMark(inner)} |
        weedle!(ByteString) => {|inner| Other::ByteString(inner)} |
        weedle!(DOMString) => {|inner| Other::DOMString(inner)} |
        weedle!(FrozenArray) => {|inner| Other::FrozenArray(inner)} |
        weedle!(Infinity) => {|inner| Other::Infinity(inner)} |
        weedle!(NaN) => {|inner| Other::NaN(inner)} |
        weedle!(USVString) => {|inner| Other::USVString(inner)} |
        weedle!(Any) => {|inner| Other::Any(inner)} |
        weedle!(Boolean) => {|inner| Other::Boolean(inner)} |
        weedle!(Byte) => {|inner| Other::Byte(inner)} |
        weedle!(Double) => {|inner| Other::Double(inner)} |
        weedle!(False) => {|inner| Other::False(inner)} |
        weedle!(Float) => {|inner| Other::Float(inner)} |
        weedle!(Long) => {|inner| Other::Long(inner)} |
        weedle!(Null) => {|inner| Other::Null(inner)} |
        weedle!(Object) => {|inner| Other::Object(inner)} |
        weedle!(Octet) => {|inner| Other::Octet(inner)} |
        weedle!(Or) => {|inner| Other::Or(inner)} |
        weedle!(Optional) => {|inner| Other::Optional(inner)} |
        weedle!(Sequence) => {|inner| Other::Sequence(inner)} |
        weedle!(Short) => {|inner| Other::Short(inner)} |
        weedle!(True) => {|inner| Other::True(inner)} |
        weedle!(Unsigned) => {|inner| Other::Unsigned(inner)} |
        weedle!(Void) => {|inner| Other::Void(inner)} |
        weedle!(ArgumentNameKeyword) => {|inner| Other::ArgumentNameKeyword(inner)} |
        weedle!(BufferRelatedType) => {|inner| Other::BufferRelatedType(inner)}
    ));
}

/// OtherOrComma ::
///     Other
///     ,
#[derive(Debug, PartialEq)]
pub enum OtherOrComma {
    Other(Other),
    Comma(Comma),
}

impl Parse for OtherOrComma {
    named!(parse -> Self, alt_complete!(
        weedle!(Other) => {|inner| OtherOrComma::Other(inner)} |
        weedle!(Comma) => {|inner| OtherOrComma::Comma(inner)}
    ));
}

/// Default ::
///     = DefaultValue
///     ε
///
/// Default dictates an optional value. Uses Option<Default> instead.
#[derive(Debug, PartialEq)]
pub struct Default {
    pub assign: Assign,
    pub value: DefaultValue,
}

impl Parse for Default {
    named!(parse -> Self, do_parse!(
        assign: weedle!(Assign) >>
        value: weedle!(DefaultValue) >>
        (Default { assign, value })
    ));
}
