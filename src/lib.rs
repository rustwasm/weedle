//! Weedle - A WebIDL Parser
//!
//! Follows the grammar as defined on [WebIDL](https://heycam.github.io/webidl)

#[macro_use]
extern crate nom;
extern crate regex;

use nom::IResult;
use terminals::*;
use literals::*;

#[macro_use]
mod macros;
mod terminals;
mod literals;

trait Parse: Sized {
    fn parse(input: &str) -> IResult<&str, Self>;
}

impl<T: Parse> Parse for Option<T> {
    named!(parse -> Self, do_parse!(
        parsed: opt!(weedle!(T)) >>
        (parsed)
    ));
}

impl<T: Parse> Parse for Box<T> {
    named!(parse -> Self, do_parse!(
        inner: weedle!(T) >>
        (Box::new(inner))
    ));
}

#[derive(Debug)]
pub struct Parenthesized<T> {
    pub open_paren: OpenParen,
    pub body: T,
    pub close_paren: CloseParen,
}

impl<T: Parse> Parse for Parenthesized<T> {
    named!(parse -> Self, do_parse!(
        open_paren: weedle!(OpenParen) >>
        body: weedle!(T) >>
        close_paren: weedle!(CloseParen) >>
        (Parenthesized {  open_paren, body, close_paren })
    ));
}

#[derive(Debug)]
pub struct Bracketed<T> {
    pub open_bracket: OpenBracket,
    pub body: T,
    pub close_bracket: CloseBracket,
}

impl<T: Parse> Parse for Bracketed<T> {
    named!(parse -> Self, do_parse!(
        open_bracket: weedle!(OpenBracket) >>
        body: weedle!(T) >>
        close_bracket: weedle!(CloseBracket) >>
        (Bracketed { open_bracket, body, close_bracket })
    ));
}

#[derive(Debug)]
pub struct Braced<T> {
    pub open_brace: OpenBrace,
    pub body: T,
    pub close_brace: CloseBrace,
}

impl<T: Parse> Parse for Braced<T> {
    named!(parse -> Self, do_parse!(
        open_brace: weedle!(OpenBrace) >>
        body: weedle!(T) >>
        close_brace: weedle!(CloseBrace) >>
        (Braced { open_brace, body, close_brace })
    ));
}

#[derive(Debug)]
pub struct Generics<T> {
    pub open_angle: LessThan,
    pub body: T,
    pub close_angle: GreaterThan,
}

impl<T: Parse> Parse for Generics<T> {
    named!(parse -> Self, do_parse!(
        open_angle: weedle!(LessThan) >>
        body: weedle!(T) >>
        close_angle: weedle!(GreaterThan) >>
        (Generics { open_angle, body, close_angle })
    ));
}

#[derive(Debug)]
pub struct Punctuated<T, S> {
    pub list: Vec<T>,
    pub separator: S,
}

impl<T: Parse, S: Parse + ::std::default::Default> Parse for Punctuated<T, S> {
    named!(parse -> Self, do_parse!(
        list: separated_list!(weedle!(S), weedle!(T)) >>
        (Punctuated { list, separator: S::default() })
    ));
}

/// ExtendedAttributeNamedArgList ::
///     **identifier** = **identifier** ( ArgumentList )
#[derive(Debug)]
pub struct ExtendedAttributeNamedArgList {
    pub lhs_identifier: Identifier,
    pub assign: Assign,
    pub rhs_identifier: Identifier,
    pub args_signature: Parenthesized<ArgumentList>,
}

impl Parse for ExtendedAttributeNamedArgList {
    named!(parse -> Self, do_parse!(
        lhs_identifier: weedle!(Identifier) >>
        assign: weedle!(Assign) >>
        rhs_identifier: weedle!(Identifier) >>
        args_signature: weedle!(Parenthesized<ArgumentList>) >>
        (ExtendedAttributeNamedArgList { lhs_identifier, assign, rhs_identifier, args_signature })
    ));
}

/// ArgumentList ::
///     Argument Arguments
///     ε
///
/// Arguments ::
///     , Argument Arguments
///     ε
#[derive(Debug)]
pub struct ArgumentList {
    pub args: Punctuated<Argument, Comma>
}

impl Parse for ArgumentList {
    named!(parse -> Self, do_parse!(
        args: weedle!(Punctuated<Argument, Comma>) >>
        (ArgumentList { args })
    ));
}

/// Argument ::
///     ExtendedAttributeList ArgumentRest
#[derive(Debug)]
pub struct Argument {
    pub attributes: ExtendedAttributeList,
    pub rest: ArgumentRest,
}

impl Parse for Argument {
    named!(parse -> Self, do_parse!(
        attributes: weedle!(ExtendedAttributeList) >>
        rest: weedle!(ArgumentRest) >>
        (Argument { attributes, rest })
    ));
}

/// ExtendedAttributeList ::
///     [ ExtendedAttribute ExtendedAttributes ]
///     ε
///
/// ExtendedAttributes ::
///     , ExtendedAttribute ExtendedAttributes
///     ε
#[derive(Debug)]
pub struct ExtendedAttributeList {
    pub list: Option<Bracketed<Punctuated<ExtendedAttribute, Comma>>>
}

impl Parse for ExtendedAttributeList {
    named!(parse -> Self, do_parse!(
        list: weedle!(Option<Bracketed<Punctuated<ExtendedAttribute, Comma>>>) >>
        (ExtendedAttributeList { list })
    ));
}

/// ExtendedAttribute ::
///     ( ExtendedAttributeInner ) ExtendedAttributeRest
///     [ ExtendedAttributeInner ] ExtendedAttributeRest
///     { ExtendedAttributeInner } ExtendedAttributeRest
///     Other ExtendedAttributeRest
///
/// ExtendedAttributeRest ::
///     ExtendedAttribute
///     ε
#[derive(Debug)]
pub enum ExtendedAttribute {
    Parenthesized(ParenthesizedExtendedAttribute),
    Bracketed(BracketedExtendedAttribute),
    Braced(BracedExtendedAttribute),
    Other(OtherExtendedAttribute),
}

impl Parse for ExtendedAttribute {
    named!(parse -> Self, alt_complete!(
        weedle!(ParenthesizedExtendedAttribute) => {|inner| ExtendedAttribute::Parenthesized(inner) } |
        weedle!(BracketedExtendedAttribute) => {|inner| ExtendedAttribute::Bracketed(inner)} |
        weedle!(BracedExtendedAttribute) => {|inner| ExtendedAttribute::Braced(inner)} |
        weedle!(OtherExtendedAttribute) => {|inner| ExtendedAttribute::Other(inner)}
    ));
}

#[derive(Debug)]
pub struct ParenthesizedExtendedAttribute {
    pub inner: Parenthesized<ExtendedAttributeInner>,
    pub rest: Option<Box<ExtendedAttribute>>,
}

impl Parse for ParenthesizedExtendedAttribute {
    named!(parse -> Self, do_parse!(
        inner: weedle!(Parenthesized<ExtendedAttributeInner>) >>
        rest: weedle!(Option<Box<ExtendedAttribute>>) >>
        (ParenthesizedExtendedAttribute { inner, rest })
    ));
}

#[derive(Debug)]
pub struct BracketedExtendedAttribute {
    pub inner: Bracketed<ExtendedAttributeInner>,
    pub rest: Option<Box<ExtendedAttribute>>,
}

impl Parse for BracketedExtendedAttribute {
    named!(parse -> Self, do_parse!(
        inner: weedle!(Bracketed<ExtendedAttributeInner>) >>
        rest: weedle!(Option<Box<ExtendedAttribute>>) >>
        (BracketedExtendedAttribute { inner, rest })
    ));
}

#[derive(Debug)]
pub struct BracedExtendedAttribute {
    pub inner: Braced<ExtendedAttributeInner>,
    pub rest: Option<Box<ExtendedAttribute>>,
}

impl Parse for BracedExtendedAttribute {
    named!(parse -> Self, do_parse!(
        inner: weedle!(Braced<ExtendedAttributeInner>) >>
        rest: weedle!(Option<Box<ExtendedAttribute>>) >>
        (BracedExtendedAttribute { inner, rest })
    ));
}

#[derive(Debug)]
pub struct OtherExtendedAttribute {
    pub other: Other,
    pub rest: Option<Box<ExtendedAttribute>>,
}

impl Parse for OtherExtendedAttribute {
    named!(parse -> Self, do_parse!(
        other: weedle!(Other) >>
        rest: weedle!(Option<Box<ExtendedAttribute>>) >>
        (OtherExtendedAttribute { other, rest })
    ));
}

/// ExtendedAttributeInner ::
///     ( ExtendedAttributeInner ) ExtendedAttributeInner
///     [ ExtendedAttributeInner ] ExtendedAttributeInner
///     { ExtendedAttributeInner } ExtendedAttributeInner
///     OtherOrComma ExtendedAttributeInner
///     ε
#[derive(Debug)]
pub enum ExtendedAttributeInner {
    Parenthesized(ParenthesizedExtendedAttributeInner),
    Bracketed(BracketedExtendedAttributeInner),
    Braced(BracedExtendedAttributeInner),
    Other(OtherExtendedAttributeInner),
    None,
}

impl Parse for ExtendedAttributeInner {
    named!(parse -> Self, alt_complete!(
        weedle!(ParenthesizedExtendedAttributeInner) => {|inner| ExtendedAttributeInner::Parenthesized(inner)} |
        weedle!(BracketedExtendedAttributeInner) => {|inner| ExtendedAttributeInner::Bracketed(inner)} |
        weedle!(BracedExtendedAttributeInner) => {|inner| ExtendedAttributeInner::Braced(inner) }|
        weedle!(OtherExtendedAttributeInner) => {|inner| ExtendedAttributeInner::Other(inner)} |
        tag!("") => {|_| ExtendedAttributeInner::None}
    ));
}

#[derive(Debug)]
pub struct ParenthesizedExtendedAttributeInner {
    pub inner: Parenthesized<Box<ExtendedAttributeInner>>,
    pub rest: Box<ExtendedAttributeInner>,
}

impl Parse for ParenthesizedExtendedAttributeInner {
    named!(parse -> Self, do_parse!(
        inner: weedle!(Parenthesized<Box<ExtendedAttributeInner>>) >>
        rest: weedle!(Box<ExtendedAttributeInner>) >>
        (ParenthesizedExtendedAttributeInner { inner, rest })
    ));
}

#[derive(Debug)]
pub struct BracketedExtendedAttributeInner {
    pub inner: Bracketed<Box<ExtendedAttributeInner>>,
    pub rest: Box<ExtendedAttributeInner>,
}

impl Parse for BracketedExtendedAttributeInner {
    named!(parse -> Self, do_parse!(
        inner: weedle!(Bracketed<Box<ExtendedAttributeInner>>) >>
        rest: weedle!(Box<ExtendedAttributeInner>) >>
        (BracketedExtendedAttributeInner { inner, rest })
    ));
}

#[derive(Debug)]
pub struct BracedExtendedAttributeInner {
    pub inner: Braced<Box<ExtendedAttributeInner>>,
    pub rest: Box<ExtendedAttributeInner>,
}

impl Parse for BracedExtendedAttributeInner {
    named!(parse -> Self, do_parse!(
        inner: weedle!(Braced<Box<ExtendedAttributeInner>>) >>
        rest: weedle!(Box<ExtendedAttributeInner>) >>
        (BracedExtendedAttributeInner { inner, rest })
    ));
}

#[derive(Debug)]
pub struct OtherExtendedAttributeInner {
    pub inner: OtherOrComma,
    pub rest: Box<ExtendedAttributeInner>,
}

impl Parse for OtherExtendedAttributeInner {
    named!(parse -> Self, do_parse!(
        inner: weedle!(OtherOrComma) >>
        rest: weedle!(Box<ExtendedAttributeInner>) >>
        (OtherExtendedAttributeInner { inner, rest })
    ));
}

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
#[derive(Debug)]
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

/// ArgumentRest ::
///     optional TypeWithExtendedAttributes ArgumentName Default
///     Type Ellipsis ArgumentName
#[derive(Debug)]
pub enum ArgumentRest {
    Optional(OptionalArgumentRest),
    Normal(NormalArgumentRest),
}

impl Parse for ArgumentRest {
    named!(parse -> Self, alt_complete!(
        weedle!(OptionalArgumentRest) => {|inner| ArgumentRest::Optional(inner)} |
        weedle!(NormalArgumentRest) => {|inner| ArgumentRest::Normal(inner)}
    ));
}

#[derive(Debug)]
pub struct OptionalArgumentRest {
    pub optional: Optional,
    pub type_: TypeWithExtendedAttributes,
    pub name: ArgumentName,
    pub default: Option<Default>,
}

impl Parse for OptionalArgumentRest {
    named!(parse -> Self, do_parse!(
        optional: weedle!(Optional) >>
        type_: weedle!(TypeWithExtendedAttributes) >>
        name: weedle!(ArgumentName) >>
        default: weedle!(Option<Default>) >>
        (OptionalArgumentRest { optional, type_, name, default })
    ));
}

#[derive(Debug)]
pub struct NormalArgumentRest {
    pub type_: Type,
    pub ellipsis: Option<Ellipsis>,
    pub name: ArgumentName,
}

impl Parse for NormalArgumentRest {
    named!(parse -> Self, do_parse!(
        type_: weedle!(Type) >>
        ellipsis: weedle!(Option<Ellipsis>) >>
        name: weedle!(ArgumentName) >>
        (NormalArgumentRest { type_, ellipsis, name })
    ));
}

/// TypeWithExtendedAttributes ::
///     ExtendedAttributeList Type
#[derive(Debug)]
pub struct TypeWithExtendedAttributes {
    pub attributes: ExtendedAttributeList,
    pub type_: Type,
}

impl Parse for TypeWithExtendedAttributes {
    named!(parse -> Self, do_parse!(
        attributes: weedle!(ExtendedAttributeList) >>
        type_: weedle!(Type) >>
        (TypeWithExtendedAttributes { attributes, type_ })
    ));
}

/// ArgumentName ::
///     ArgumentNameKeyword
///     identifier
#[derive(Debug)]
pub enum ArgumentName {
    Keyword(ArgumentNameKeyword),
    Identifier(Identifier),
}

impl Parse for ArgumentName {
    named!(parse -> Self, alt_complete!(
        weedle!(ArgumentNameKeyword) => {|inner| ArgumentName::Keyword(inner)} |
        weedle!(Identifier) => {|inner| ArgumentName::Identifier(inner)}
    ));
}

/// ArgumentNameKeyword ::
///     attribute
///     callback
///     const
///     deleter
///     dictionary
///     enum
///     getter
///     includes
///     inherit
///     interface
///     iterable
///     maplike
///     namespace
///     partial
///     required
///     setlike
///     setter
///     static
///     stringifier
///     typedef
///     unrestricted
#[derive(Debug)]
pub enum ArgumentNameKeyword {
    Attribute(Attribute),
    Callback(Callback),
    Const(Const),
    Deleter(Deleter),
    Dictionary(Dictionary),
    Enum(Enum),
    Getter(Getter),
    Includes(Includes),
    Inherit(Inherit),
    Interface(Interface),
    Iterable(Iterable),
    Maplike(Maplike),
    Namespace(Namespace),
    Partial(Partial),
    Required(Required),
    Setlike(Setlike),
    Setter(Setter),
    Static(Static),
    Stringifier(Stringifier),
    Typedef(Typedef),
    Unrestricted(Unrestricted),
}

impl Parse for ArgumentNameKeyword {
    named!(parse -> Self, alt_complete!(
        weedle!(Attribute) => {|inner| ArgumentNameKeyword::Attribute(inner)} |
        weedle!(Callback) => {|inner| ArgumentNameKeyword::Callback(inner)} |
        weedle!(Const) => {|inner| ArgumentNameKeyword::Const(inner)} |
        weedle!(Deleter) => {|inner| ArgumentNameKeyword::Deleter(inner)} |
        weedle!(Dictionary) => {|inner| ArgumentNameKeyword::Dictionary(inner)} |
        weedle!(Enum) => {|inner| ArgumentNameKeyword::Enum(inner)} |
        weedle!(Getter) => {|inner| ArgumentNameKeyword::Getter(inner)} |
        weedle!(Includes) => {|inner| ArgumentNameKeyword::Includes(inner)} |
        weedle!(Inherit) => {|inner| ArgumentNameKeyword::Inherit(inner)} |
        weedle!(Interface) => {|inner| ArgumentNameKeyword::Interface(inner)} |
        weedle!(Iterable) => {|inner| ArgumentNameKeyword::Iterable(inner)} |
        weedle!(Maplike) => {|inner| ArgumentNameKeyword::Maplike(inner)} |
        weedle!(Namespace) => {|inner| ArgumentNameKeyword::Namespace(inner)} |
        weedle!(Partial) => {|inner| ArgumentNameKeyword::Partial(inner)} |
        weedle!(Required) => {|inner| ArgumentNameKeyword::Required(inner)} |
        weedle!(Setlike) => {|inner| ArgumentNameKeyword::Setlike(inner)} |
        weedle!(Setter) => {|inner| ArgumentNameKeyword::Setter(inner)} |
        weedle!(Static) => {|inner| ArgumentNameKeyword::Static(inner)} |
        weedle!(Stringifier) => {|inner| ArgumentNameKeyword::Stringifier(inner)} |
        weedle!(Typedef) => {|inner| ArgumentNameKeyword::Typedef(inner)} |
        weedle!(Unrestricted) => {|inner| ArgumentNameKeyword::Unrestricted(inner)}
    ));
}

/// Default ::
///     = DefaultValue
///     ε
///
/// Default dictates an optional value. Uses Option<Default> instead.
#[derive(Debug)]
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

/// DefaultValue ::
///     ConstValue
///     **string**
///     **[ ]**
#[derive(Debug)]
pub enum DefaultValue {
    Const(ConstValue),
    String(String),
    EmptyArray(EmptyArrayLit),
}

impl Parse for DefaultValue {
    named!(parse -> Self, alt!(
        weedle!(ConstValue) => {|inner| DefaultValue::Const(inner)} |
        weedle!(String) => {|inner| DefaultValue::String(inner)} |
        weedle!(EmptyArrayLit) => {|inner| DefaultValue::EmptyArray(inner)}
    ));
}

#[derive(Debug)]
pub struct EmptyArrayLit {
    pub open_bracket: OpenBracket,
    pub close_bracket: CloseBracket,
}

impl Parse for EmptyArrayLit {
    named!(parse -> Self, do_parse!(
        open_bracket: weedle!(OpenBracket) >>
        close_bracket: weedle!(CloseBracket) >>
        (EmptyArrayLit { open_bracket, close_bracket })
    ));
}

/// ConstValue ::
///     BooleanLiteral
///     FloatLiteral
///     **integer**
///     null
#[derive(Debug)]
pub enum ConstValue {
    BooleanLiteral(BooleanLiteral),
    FloatLiteral(FloatLiteral),
    Integer(i64),
    Null(Null),
}

impl Parse for ConstValue {
    named!(parse -> Self, alt_complete!(
        weedle!(BooleanLiteral) => {|inner| ConstValue::BooleanLiteral(inner)} |
        weedle!(FloatLiteral) => {|inner| ConstValue::FloatLiteral(inner)} |
        weedle!(i64) => {|inner| ConstValue::Integer(inner)} |
        weedle!(Null) => {|inner| ConstValue::Null(inner)}
    ));
}

/// BooleanLiteral ::
///     true
///     false
#[derive(Debug)]
pub enum BooleanLiteral {
    True(True),
    False(False),
}

impl Parse for BooleanLiteral {
    named!(parse -> Self, alt_complete!(
        weedle!(True) => {|inner| BooleanLiteral::True(inner)} |
        weedle!(False) => {|inner| BooleanLiteral::False(inner)}
    ));
}

/// FloatLiteral ::
///     **float**
///     -Infinity
///     Infinity
///     NaN
#[derive(Debug)]
pub enum FloatLiteral {
    Float(f64),
    NegInfinity(NegInfinity),
    Infinity(Infinity),
    NaN(NaN),
}

impl Parse for FloatLiteral {
    named!(parse -> Self, alt_complete!(
        weedle!(f64) => {|inner| FloatLiteral::Float(inner)} |
        weedle!(NegInfinity) => {|inner| FloatLiteral::NegInfinity(inner)} |
        weedle!(Infinity) => {|inner| FloatLiteral::Infinity(inner)} |
        weedle!(NaN) => {|inner| FloatLiteral::NaN(inner)}
    ));
}

/// BufferRelatedType ::
///     ArrayBuffer
///     DataView
///     Int8Array
///     Int16Array
///     Int32Array
///     Uint8Array
///     Uint16Array
///     Uint32Array
///     Uint8ClampedArray
///     Float32Array
///     Float64Array
#[derive(Debug)]
pub enum BufferRelatedType {
    ArrayBuffer(ArrayBuffer),
    DataView(DataView),
    Int8Array(Int8Array),
    Int16Array(Int16Array),
    Int32Array(Int32Array),
    Uint8Array(Uint8Array),
    Uint16Array(Uint16Array),
    Uint32Array(Uint32Array),
    Uint8ClampedArray(Uint8ClampedArray),
    Float32Array(Float32Array),
    Float64Array(Float64Array),
}

impl Parse for BufferRelatedType {
    named!(parse -> Self, alt_complete!(
        weedle!(ArrayBuffer) => {|inner| BufferRelatedType::ArrayBuffer(inner)} |
        weedle!(DataView) => {|inner| BufferRelatedType::DataView(inner)} |
        weedle!(Int8Array) => {|inner| BufferRelatedType::Int8Array(inner)} |
        weedle!(Int16Array) => {|inner| BufferRelatedType::Int16Array(inner)} |
        weedle!(Int32Array) => {|inner| BufferRelatedType::Int32Array(inner)} |
        weedle!(Uint8Array) => {|inner| BufferRelatedType::Uint8Array(inner)} |
        weedle!(Uint16Array) => {|inner| BufferRelatedType::Uint16Array(inner)} |
        weedle!(Uint32Array) => {|inner| BufferRelatedType::Uint32Array(inner)} |
        weedle!(Uint8ClampedArray) => {|inner| BufferRelatedType::Uint8ClampedArray(inner)} |
        weedle!(Float32Array) => {|inner| BufferRelatedType::Float32Array(inner)} |
        weedle!(Float64Array) => {|inner| BufferRelatedType::Float64Array(inner)}
    ));
}

/// OtherOrComma ::
///     Other
///     ,
#[derive(Debug)]
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

/// Type ::
///     SingleType
///     UnionType Null
#[derive(Debug)]
pub enum Type {
    Single(Box<SingleType>),
    UnionNull(Box<UnionNullType>),
}

impl Parse for Type {
    named!(parse -> Self, alt_complete!(
        weedle!(Box<SingleType>) => {|inner| Type::Single(inner)} |
        weedle!(Box<UnionNullType>) => {|inner| Type::UnionNull(inner)}
    ));
}

#[derive(Debug)]
pub struct UnionNullType {
    pub type_: MayBeNull<UnionType>
}

impl Parse for UnionNullType {
    named!(parse -> Self, do_parse!(
        type_: weedle!(MayBeNull<UnionType>) >>
        (UnionNullType { type_ })
    ));
}

/// SingleType ::
///     NonAnyType
///     any
#[derive(Debug)]
pub enum SingleType {
    NonAny(NonAnyType),
    Any(Any),
}

impl Parse for SingleType {
    named!(parse -> Self, alt_complete!(
        weedle!(NonAnyType) => {|inner| SingleType::NonAny(inner)} |
        weedle!(Any) => {|inner| SingleType::Any(inner)}
    ));
}

/// NonAnyType ::
///     PromiseType ε
///     PrimitiveType Null
///     StringType Null
///     identifier Null
///     sequence < TypeWithExtendedAttributes > Null
///     object Null
///     symbol Null
///     Error Null
///     BufferRelatedType Null
///     FrozenArray < TypeWithExtendedAttributes > Null
///     RecordType Null
#[derive(Debug)]
pub enum NonAnyType {
    Promise(PromiseType),
    MayBePrimitive(MayBeNull<PrimitiveType>),
    MayBeString(MayBeNull<StringType>),
    MayBeIdentifier(MayBeNull<Identifier>),
    MayBeSequence(MayBeNull<SequenceType>),
    MayBeObject(MayBeNull<Object>),
    MayBeSymbol(MayBeNull<Symbol>),
    MayBeError(MayBeNull<Error>),
    MayBeBufferedRelated(MayBeNull<BufferRelatedType>),
    MayBeFrozenArray(MayBeNull<FrozenArrayType>),
    MayBeRecord(MayBeNull<RecordType>),
}

impl Parse for NonAnyType {
    named!(parse -> Self, alt_complete!(
        weedle!(PromiseType) => {|inner| NonAnyType::Promise(inner)} |
        weedle!(MayBeNull<PrimitiveType>) => {|inner| NonAnyType::MayBePrimitive(inner)} |
        weedle!(MayBeNull<StringType>) => {|inner| NonAnyType::MayBeString(inner)} |
        weedle!(MayBeNull<Identifier>) => {|inner| NonAnyType::MayBeIdentifier(inner)} |
        weedle!(MayBeNull<SequenceType>) => {|inner| NonAnyType::MayBeSequence(inner)} |
        weedle!(MayBeNull<Object>) => {|inner| NonAnyType::MayBeObject(inner)} |
        weedle!(MayBeNull<Symbol>) => {|inner| NonAnyType::MayBeSymbol(inner)} |
        weedle!(MayBeNull<Error>) => {|inner| NonAnyType::MayBeError(inner)} |
        weedle!(MayBeNull<BufferRelatedType>) => {|inner| NonAnyType::MayBeBufferedRelated(inner)} |
        weedle!(MayBeNull<FrozenArrayType>) => {|inner| NonAnyType::MayBeFrozenArray(inner)} |
        weedle!(MayBeNull<RecordType>) => {|inner| NonAnyType::MayBeRecord(inner)}
    ));
}

#[derive(Debug)]
pub struct SequenceType {
    pub sequence: Sequence,
    pub generics: Generics<TypeWithExtendedAttributes>,
}

impl Parse for SequenceType {
    named!(parse -> Self, do_parse!(
        sequence: weedle!(Sequence) >>
        generics: weedle!(Generics<TypeWithExtendedAttributes>) >>
        (SequenceType { sequence, generics })
    ));
}

#[derive(Debug)]
pub struct FrozenArrayType {
    pub frozen_array: FrozenArray,
    pub generics: Generics<TypeWithExtendedAttributes>,
}

impl Parse for FrozenArrayType {
    named!(parse -> Self, do_parse!(
        frozen_array: weedle!(FrozenArray) >>
        generics: weedle!(Generics<TypeWithExtendedAttributes>) >>
        (FrozenArrayType { frozen_array, generics })
    ));
}

/// Null ::
///     ?
///     ε
#[derive(Debug)]
pub struct MayBeNull<T> {
    pub type_: T,
    pub q_mark: Option<QMark>,
}

impl<T: Parse> Parse for MayBeNull<T> {
    named!(parse -> Self, do_parse!(
        type_: weedle!(T) >>
        q_mark: weedle!(Option<QMark>) >>
        (MayBeNull { type_, q_mark })
    ));
}

/// PromiseType ::
///    Promise < ReturnType >
#[derive(Debug)]
pub struct PromiseType {
    pub promise: Promise,
    pub generics: Generics<ReturnType>,
}

impl Parse for PromiseType {
    named!(parse -> Self, do_parse!(
        promise: weedle!(Promise) >>
        generics: weedle!(Generics<ReturnType>) >>
        (PromiseType { promise, generics })
    ));
}

/// ReturnType ::
///     Type
///     void
#[derive(Debug)]
pub enum ReturnType {
    Type(Type),
    Void(Void),
}

impl Parse for ReturnType {
    named!(parse -> Self, alt_complete!(
        weedle!(Type) => {|inner| ReturnType::Type(inner)} |
        weedle!(Void) => {|inner| ReturnType::Void(inner)}
    ));
}

/// PrimitiveType ::
///     UnsignedIntegerType
///     UnrestrictedFloatType
///     boolean
///     byte
///     octet
#[derive(Debug)]
pub enum PrimitiveType {
    UnsignedIntegerType(UnsignedIntegerType),
    UnrestrictedFloatType(UnrestrictedFloatType),
    Boolean(Boolean),
    Byte(Byte),
    Octet(Octet),
}

impl Parse for PrimitiveType {
    named!(parse -> Self, alt_complete!(
        weedle!(UnsignedIntegerType) => {|inner| PrimitiveType::UnsignedIntegerType(inner)} |
        weedle!(UnrestrictedFloatType) => {|inner| PrimitiveType::UnrestrictedFloatType(inner)} |
        weedle!(Boolean) => {|inner| PrimitiveType::Boolean(inner)} |
        weedle!(Byte) => {|inner| PrimitiveType::Byte(inner)} |
        weedle!(Octet) => {|inner| PrimitiveType::Octet(inner)}
    ));
}

/// UnsignedIntegerType ::
///     unsigned IntegerType
///     IntegerType
#[derive(Debug)]
pub struct UnsignedIntegerType {
    pub unsigned: Option<Unsigned>,
    pub type_: IntegerType,
}

impl Parse for UnsignedIntegerType {
    named!(parse -> Self, do_parse!(
        unsigned: weedle!(Option<Unsigned>) >>
        type_: weedle!(IntegerType) >>
        (UnsignedIntegerType { unsigned, type_ })
    ));
}

/// IntegerType ::
///     short
///     long OptionalLong
#[derive(Debug)]
pub enum IntegerType {
    Short(Short),
    Long(LongType),
}

impl Parse for IntegerType {
    named!(parse -> Self, alt_complete!(
        weedle!(Short) => {|inner| IntegerType::Short(inner)} |
        weedle!(LongType) => {|inner| IntegerType::Long(inner)}
    ));
}

/// OptionalLong ::
///     long
///     ε
#[derive(Debug)]
pub struct LongType {
    pub long: Long,
    pub optional: Option<Long>,
}

impl Parse for LongType {
    named!(parse -> Self, do_parse!(
        long: weedle!(Long) >>
        optional: weedle!(Option<Long>) >>
        (LongType { long, optional })
    ));
}

/// UnrestrictedFloatType ::
///     unrestricted FloatType
///     FloatType
#[derive(Debug)]
pub struct UnrestrictedFloatType {
    pub unrestricted: Option<Unrestricted>,
    pub type_: FloatType,
}

impl Parse for UnrestrictedFloatType {
    named!(parse -> Self, do_parse!(
        unrestricted: weedle!(Option<Unrestricted>) >>
        type_: weedle!(FloatType) >>
        (UnrestrictedFloatType { unrestricted, type_ })
    ));
}

/// FloatType ::
///     float
///     double
#[derive(Debug)]
pub enum FloatType {
    Float(Float),
    Double(Double),
}

impl Parse for FloatType {
    named!(parse -> Self, alt_complete!(
        weedle!(Float) => {|inner| FloatType::Float(inner)} |
        weedle!(Double) => {|inner| FloatType::Double(inner)}
    ));
}

/// StringType ::
///     ByteString
///     DOMString
///     USVString
#[derive(Debug)]
pub enum StringType {
    Byte(ByteString),
    DOM(DOMString),
    USV(USVString),
}

impl Parse for StringType {
    named!(parse -> Self, alt_complete!(
        weedle!(ByteString) => {|inner| StringType::Byte(inner)} |
        weedle!(DOMString) => {|inner| StringType::DOM(inner)} |
        weedle!(USVString) => {|inner| StringType::USV(inner)}
    ));
}

/// RecordType ::
///     record < StringType , TypeWithExtendedAttributes >
#[derive(Debug)]
pub struct RecordType {
    pub record: Record,
    pub generics: Generics<RecordTypeGenerics>,
}

impl Parse for RecordType {
    named!(parse -> Self, do_parse!(
        record: weedle!(Record) >>
        generics: weedle!(Generics<RecordTypeGenerics>) >>
        (RecordType { record, generics })
    ));
}

#[derive(Debug)]
pub struct RecordTypeGenerics {
    pub string_type: StringType,
    pub comma: Comma,
    pub type_: TypeWithExtendedAttributes,
}

impl Parse for RecordTypeGenerics {
    named!(parse -> Self, do_parse!(
        string_type: weedle!(StringType) >>
        comma: weedle!(Comma) >>
        type_: weedle!(TypeWithExtendedAttributes) >>
        (RecordTypeGenerics { string_type, comma, type_ })
    ));
}

/// UnionType ::
///     ( UnionMemberType or UnionMemberType UnionMemberTypes )
/// UnionMemberTypes ::
///     or UnionMemberType UnionMemberTypes
///     ε
#[derive(Debug)]
pub struct UnionType {
    pub punctuated: Punctuated<UnionMemberType, Or>
}

impl Parse for UnionType {
    named!(parse -> Self, do_parse!(
        punctuated: weedle!(Punctuated<UnionMemberType, Or>) >>
        (UnionType { punctuated })
    ));
}

/// UnionMemberType ::
///     ExtendedAttributeList NonAnyType
///     UnionType Null
#[derive(Debug)]
pub enum UnionMemberType {
    Attributed(AttributedUnionMemberType),
    Simple(SimpleUnionMemberType),
}

impl Parse for UnionMemberType {
    named!(parse -> Self, alt_complete!(
        weedle!(AttributedUnionMemberType) => {|inner| UnionMemberType::Attributed(inner)} |
        weedle!(SimpleUnionMemberType) => {|inner| UnionMemberType::Simple(inner)}
    ));
}

#[derive(Debug)]
pub struct AttributedUnionMemberType {
    pub attributes: ExtendedAttributeList,
    pub type_: NonAnyType,
}

impl Parse for AttributedUnionMemberType {
    named!(parse -> Self, do_parse!(
        attributes: weedle!(ExtendedAttributeList) >>
        type_: weedle!(NonAnyType) >>
        (AttributedUnionMemberType { attributes, type_ })
    ));
}

#[derive(Debug)]
pub struct SimpleUnionMemberType {
    pub type_: MayBeNull<UnionType>
}

impl Parse for SimpleUnionMemberType {
    named!(parse -> Self, do_parse!(
        type_: weedle!(MayBeNull<UnionType>) >>
        (SimpleUnionMemberType { type_ })
    ));
}
