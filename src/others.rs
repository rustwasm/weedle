use literals::*;
use types::*;
use Parse;
use arguments::*;

/// Parses any one of the `Other` variants
///
/// ### Grammar
/// ```other
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
/// ```
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-Other)
#[derive(Debug, PartialEq)]
pub enum Other {
    IntegerLit(i64),
    FloatLit(f64),
    Identifier(Identifier),
    StringLit(String),
    OtherLit(OtherLit),
    Minus(term!(-)),
    NegInfinity(term!(-Infinity)),
    Dot(term!(.)),
    Ellipsis(term!(...)),
    Colon(term!(:)),
    SemiColon(term!(;)),
    LessThan(term!(<)),
    Assign(term!(=)),
    GreaterThan(term!(>)),
    QMark(term!(?)),
    ByteString(term!(ByteString)),
    DOMString(term!(DOMString)),
    FrozenArray(term!(FrozenArray)),
    Infinity(term!(Infinity)),
    NaN(term!(NaN)),
    USVString(term!(USVString)),
    Any(term!(any)),
    Boolean(term!(boolean)),
    Byte(term!(byte)),
    Double(term!(double)),
    False(term!(false)),
    Float(term!(float)),
    Long(term!(long)),
    Null(term!(null)),
    Object(term!(object)),
    Octet(term!(octet)),
    Or(term!(or)),
    Optional(term!(optional)),
    Sequence(term!(sequence)),
    Short(term!(short)),
    True(term!(true)),
    Unsigned(term!(unsigned)),
    Void(term!(void)),
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
        weedle!(term!(-)) => {|inner| Other::Minus(inner)} |
        weedle!(term!(-Infinity)) => {|inner| Other::NegInfinity(inner)} |
        weedle!(term!(.)) => {|inner| Other::Dot(inner)} |
        weedle!(term!(...)) => {|inner| Other::Ellipsis(inner)} |
        weedle!(term!(:)) => {|inner| Other::Colon(inner)} |
        weedle!(term!(;)) => {|inner| Other::SemiColon(inner)} |
        weedle!(term!(<)) => {|inner| Other::LessThan(inner)} |
        weedle!(term!(=)) => {|inner| Other::Assign(inner)} |
        weedle!(term!(>)) => {|inner| Other::GreaterThan(inner)} |
        weedle!(term!(?)) => {|inner| Other::QMark(inner)} |
        weedle!(term!(ByteString)) => {|inner| Other::ByteString(inner)} |
        weedle!(term!(DOMString)) => {|inner| Other::DOMString(inner)} |
        weedle!(term!(FrozenArray)) => {|inner| Other::FrozenArray(inner)} |
        weedle!(term!(Infinity)) => {|inner| Other::Infinity(inner)} |
        weedle!(term!(NaN)) => {|inner| Other::NaN(inner)} |
        weedle!(term!(USVString)) => {|inner| Other::USVString(inner)} |
        weedle!(term!(any)) => {|inner| Other::Any(inner)} |
        weedle!(term!(boolean)) => {|inner| Other::Boolean(inner)} |
        weedle!(term!(byte)) => {|inner| Other::Byte(inner)} |
        weedle!(term!(double)) => {|inner| Other::Double(inner)} |
        weedle!(term!(false)) => {|inner| Other::False(inner)} |
        weedle!(term!(float)) => {|inner| Other::Float(inner)} |
        weedle!(term!(long)) => {|inner| Other::Long(inner)} |
        weedle!(term!(null)) => {|inner| Other::Null(inner)} |
        weedle!(term!(object)) => {|inner| Other::Object(inner)} |
        weedle!(term!(octet)) => {|inner| Other::Octet(inner)} |
        weedle!(term!(or)) => {|inner| Other::Or(inner)} |
        weedle!(term!(optional)) => {|inner| Other::Optional(inner)} |
        weedle!(term!(sequence)) => {|inner| Other::Sequence(inner)} |
        weedle!(term!(short)) => {|inner| Other::Short(inner)} |
        weedle!(term!(true)) => {|inner| Other::True(inner)} |
        weedle!(term!(unsigned)) => {|inner| Other::Unsigned(inner)} |
        weedle!(term!(void)) => {|inner| Other::Void(inner)} |
        weedle!(ArgumentNameKeyword) => {|inner| Other::ArgumentNameKeyword(inner)} |
        weedle!(BufferRelatedType) => {|inner| Other::BufferRelatedType(inner)}
    ));
}

/// Parses either `Other` or `,`
///
/// ### Grammar
/// ```other
/// OtherOrComma ::
///     Other
///     ,
/// ```
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-OtherOrComma)
#[derive(Debug, PartialEq)]
pub enum OtherOrComma {
    Other(Other),
    Comma(term!(,)),
}

impl Parse for OtherOrComma {
    named!(parse -> Self, alt_complete!(
        weedle!(Other) => {|inner| OtherOrComma::Other(inner)} |
        weedle!(term!(,)) => {|inner| OtherOrComma::Comma(inner)}
    ));
}

/// Parses rhs of an assignment expression. Ex: `= 45`
/// ### Grammar
/// ```other
/// Default ::
///     = DefaultValue
///     ε
/// ```
///
/// Default dictates an optional value. Uses Option<Default> instead.
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-Default)
#[derive(Debug, PartialEq)]
pub struct Default {
    pub assign: term!(=),
    pub value: DefaultValue,
}

impl Parse for Default {
    named!(parse -> Self, do_parse!(
        assign: weedle!(term!(=)) >>
        value: weedle!(DefaultValue) >>
        (Default { assign, value })
    ));
}
