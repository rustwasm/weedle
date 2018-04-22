//! Weedle - A WebIDL Parser
//!
//! Follows the grammar as defined on [WebIDL](https://heycam.github.io/webidl)

#[macro_use]
extern crate nom;

use terminals::{CloseBracket, CloseParen, Comma, OpenBracket, OpenParen, OpenBrace, CloseBrace};

#[macro_use]
mod terminals;

pub struct Parenthesized<T> {
    pub open_paren: OpenParen,
    pub body: T,
    pub close_paren: CloseParen,
}

pub struct Bracketed<T> {
    pub open_bracket: OpenBracket,
    pub body: T,
    pub close_bracket: CloseBracket,
}

pub struct Braced<T> {
    pub open_brace: OpenBrace,
    pub body: T,
    pub close_brace: CloseBrace,
}

pub struct Generics<T> {
    pub open_angle: terminals::LessThan,
    pub body: T,
    pub close_angle: terminals::GreaterThan
}

pub struct Punctuated<T, S> {
    pub list: Vec<T>,
    pub separator: S,
}

pub struct Identifier {
    pub name: String
}

/// ExtendedAttributeNamedArgList ::
///     **identifier** = **identifier** ( ArgumentList )
pub struct ExtendedAttributeNamedArgList {
    pub lhs_identifier: Identifier,
    pub assign: terminals::Assign,
    pub rhs_identifier: Identifier,
    pub args_signature: Parenthesized<ArgumentList>,
}

/// ArgumentList ::
///     Argument Arguments
///     ε
///
/// Arguments ::
///     , Argument Arguments
///     ε
pub struct ArgumentList {
    pub args: Punctuated<Argument, Comma>
}

/// Argument ::
///     ExtendedAttributeList ArgumentRest
pub struct Argument {
    pub attributes: ExtendedAttributeList,
    pub rest: ArgumentRest,
}

/// ExtendedAttributeList ::
///     [ ExtendedAttribute ExtendedAttributes ]
///     ε
///
/// ExtendedAttributes ::
///     , ExtendedAttribute ExtendedAttributes
///     ε
pub struct ExtendedAttributeList {
    pub list: Bracketed<Punctuated<ExtendedAttribute, Comma>>
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
pub enum ExtendedAttribute {
    Parenthesized(ParenthesizedExtendedAttribute),
    Bracketed(BracketedExtendedAttribute),
    Braced(BracedExtendedAttribute),
    Other(OtherExtendedAttribute),
}

pub struct ParenthesizedExtendedAttribute {
    pub inner: Parenthesized<ExtendedAttributeInner>,
    pub rest: Option<Box<ExtendedAttribute>>,
}

pub struct BracketedExtendedAttribute {
    pub inner: Bracketed<ExtendedAttributeInner>,
    pub rest: Option<Box<ExtendedAttribute>>,
}

pub struct BracedExtendedAttribute {
    pub inner: Braced<ExtendedAttributeInner>,
    pub rest: Option<Box<ExtendedAttribute>>,
}

pub struct OtherExtendedAttribute {
    pub other: Other,
    pub rest: Option<Box<ExtendedAttribute>>,
}

/// ExtendedAttributeInner ::
///     ( ExtendedAttributeInner ) ExtendedAttributeInner
///     [ ExtendedAttributeInner ] ExtendedAttributeInner
///     { ExtendedAttributeInner } ExtendedAttributeInner
///     OtherOrComma ExtendedAttributeInner
///     ε
pub enum ExtendedAttributeInner {
    Parenthesized(ParenthesizedExtendedAttributeInner),
    Bracketed(BracketedExtendedAttributeInner),
    Braced(BracedExtendedAttributeInner),
    Other(OtherExtendedAttributeInner),
    None,
}

pub struct ParenthesizedExtendedAttributeInner {
    inner: Parenthesized<Box<ExtendedAttributeInner>>,
    rest: Box<ExtendedAttributeInner>,
}

pub struct BracketedExtendedAttributeInner {
    inner: Bracketed<Box<ExtendedAttributeInner>>,
    rest: Box<ExtendedAttributeInner>,
}

pub struct BracedExtendedAttributeInner {
    inner: Braced<Box<ExtendedAttributeInner>>,
    rest: Box<ExtendedAttributeInner>,
}

pub struct OtherExtendedAttributeInner {
    inner: OtherOrComma,
    rest: Box<ExtendedAttributeInner>,
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
pub enum Other {
    IntegerLit(i64),
    FloatLit(f64),
    Identifier(Identifier),
    StringLit(String),
    Other(String),
    Minus(terminals::Minus),
    NegInfinity(terminals::NegInfinity),
    Dot(terminals::Dot),
    Ellipsis(terminals::Ellipsis),
    Colon(terminals::Colon),
    SemiColon(terminals::SemiColon),
    LessThan(terminals::LessThan),
    Assign(terminals::Assign),
    GreaterThan(terminals::GreaterThan),
    QMark(terminals::QMark),
    ByteString(terminals::ByteString),
    DOMString(terminals::DOMString),
    FrozenString(terminals::FrozenArray),
    Infinity(terminals::Infinity),
    NaN(terminals::NaN),
    USVString(terminals::USVString),
    Any(terminals::Any),
    Boolean(terminals::Boolean),
    Byte(terminals::Byte),
    Double(terminals::Double),
    False(terminals::False),
    Float(terminals::Float),
    Long(terminals::Long),
    Null(terminals::Null),
    Object(terminals::Object),
    Octect(terminals::Octet),
    Or(terminals::Or),
    Optional(terminals::Optional),
    Sequence(terminals::Sequence),
    Short(terminals::Short),
    True(terminals::True),
    Unsigned(terminals::Unsigned),
    Void(terminals::Void),
    ArgumentNameKeyword(ArgumentNameKeyword),
    BufferRelatedType(BufferRelatedType),
}

/// ArgumentRest ::
///     optional TypeWithExtendedAttributes ArgumentName Default
///     Type Ellipsis ArgumentName
pub enum ArgumentRest {
    Optional(OptionalArgumentRest),
    Normal(NormalArgumentRest),
}

pub struct OptionalArgumentRest {
    pub optional: terminals::Optional,
    pub type_: TypeWithExtendedAttributes,
    pub name: ArgumentName,
    pub default: Option<Default>,
}

pub struct NormalArgumentRest {
    pub type_: Type,
    pub ellipsis: Option<terminals::Ellipsis>,
    pub name: ArgumentName,
}

/// TypeWithExtendedAttributes ::
///     ExtendedAttributeList Type
pub struct TypeWithExtendedAttributes {
    pub attributes: ExtendedAttributeList,
    pub type_: Type,
}

/// ArgumentName ::
///     ArgumentNameKeyword
///     identifier
pub enum ArgumentName {
    Keyword(ArgumentNameKeyword),
    Identifier(Identifier),
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
pub enum ArgumentNameKeyword {
    Attribute(terminals::Attribute),
    Callback(terminals::Callback),
    Const(terminals::Const),
    Deleter(terminals::Deleter),
    Dictionary(terminals::Dictionary),
    Enum(terminals::Enum),
    Getter(terminals::Getter),
    Includes(terminals::Includes),
    Inherit(terminals::Inherit),
    Interface(terminals::Interface),
    Iterable(terminals::Iterable),
    Maplike(terminals::Maplike),
    Namespace(terminals::Namespace),
    Partial(terminals::Partial),
    Required(terminals::Required),
    Setlike(terminals::Setlike),
    Setter(terminals::Setter),
    Static(terminals::Static),
    Stringifier(terminals::Stringifier),
    Typedef(terminals::Typedef),
    Unrestricted(terminals::Unrestricted),
}

/// Default ::
///     = DefaultValue
///     ε
///
/// Default dictates an optional value. Uses Option<Default> instead.
pub struct Default {
    pub assign: terminals::Assign,
    pub value: DefaultValue,
}

/// DefaultValue ::
///     ConstValue
///     **string**
///     **[ ]**
pub enum DefaultValue {
    Const(ConstValue),
    String(String),
    EmptyArray,
}

/// ConstValue ::
///     BooleanLiteral
///     FloatLiteral
///     **integer**
///     null
pub enum ConstValue {
    BooleanLiteral(BooleanLiteral),
    FloatLiteral(FloatLiteral),
    Integer(i64),
    Null,
}

/// BooleanLiteral ::
///     true
///     false
pub enum BooleanLiteral {
    True(terminals::True),
    False(terminals::False),
}

/// FloatLiteral ::
///     **float**
///     -Infinity
///     Infinity
///     NaN
pub enum FloatLiteral {
    Float(f64),
    NegInfinity,
    Infinity,
    NaN,
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
pub enum BufferRelatedType {
    ArrayBuffer(terminals::ArrayBuffer),
    DataView(terminals::DataView),
    Int8Array(terminals::Int8Array),
    Int16Array(terminals::Int16Array),
    Int32Array(terminals::Int32Array),
    Uint8Array(terminals::Uint8Array),
    Uint16Array(terminals::Uint16Array),
    Uint32Array(terminals::Uint32Array),
    Uint8ClampedArray(terminals::Uint8ClampedArray),
    Float32Array(terminals::Float32Array),
    Float64Array(terminals::Float64Array),
}

/// OtherOrComma ::
///     Other
///     ,
pub enum OtherOrComma {
    Other(Other),
    Comma(Comma)
}

/// Type ::
///     SingleType
///     UnionType Null
pub enum Type {
    Single(Box<SingleType>),
    UnionNull(Box<UnionNullType>)
}

pub struct UnionNullType {
    pub type_: MayBeNull<UnionType>
}

/// SingleType ::
///     NonAnyType
///     any
pub enum SingleType {
    NonAny(NonAnyType),
    Any(terminals::Any)
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
pub enum NonAnyType {
    Promise(PromiseType),
    MayBePrimitive(MayBeNull<PrimitiveType>),
    MayBeString(MayBeNull<StringType>),
    MayBeIdentifier(MayBeNull<Identifier>),
    MayBeSequence(MayBeNull<SequenceType>),
    MayBeObject(MayBeNull<terminals::Object>),
    MayBeSymbol(MayBeNull<terminals::Symbol>),
    MayBeError(MayBeNull<terminals::Error>),
    MayBeBufferedRelated(MayBeNull<BufferRelatedType>),
    MayBeFrozenArray(MayBeNull<FrozenArrayType>),
    MayBeRecord(MayBeNull<RecordType>)
}

pub struct SequenceType {
    sequence: terminals::Sequence,
    generics: Generics<TypeWithExtendedAttributes>
}

pub struct FrozenArrayType {
    frozen_array: terminals::FrozenArray,
    generics: Generics<TypeWithExtendedAttributes>
}

/// Null ::
///     ?
///     ε
pub struct MayBeNull<T> {
    type_: T,
    q_mark: Option<terminals::QMark>
}

/// PromiseType ::
///    Promise < ReturnType >
pub struct PromiseType {
    promise: terminals::Promise,
    generics: Generics<ReturnType>
}

/// ReturnType ::
///     Type
///     void
pub enum ReturnType {
    Type(Type),
    Void(terminals::Void)
}


/// PrimitiveType ::
///     UnsignedIntegerType
///     UnrestrictedFloatType
///     boolean
///     byte
///     octet
pub enum PrimitiveType {
    UnsignedInteger(UnsignedIntegerType),
    UnrestrictedFloat(UnrestrictedFloatType),
    Boolean(terminals::Boolean),
    Byte(terminals::Byte),
    Octet(terminals::Octet)
}

/// UnsignedIntegerType ::
///     unsigned IntegerType
///     IntegerType
pub struct UnsignedIntegerType {
    unsigned: Option<terminals::Unsigned>,
    type_: IntegerType
}

/// IntegerType ::
///     short
///     long OptionalLong
pub enum IntegerType {
    Short(terminals::Short),
    Long(LongType)
}

/// OptionalLong ::
///     long
///     ε
pub struct LongType {
    long: terminals::Long,
    optional: Option<terminals::Long>
}

/// UnrestrictedFloatType ::
///     unrestricted FloatType
///     FloatType
pub struct UnrestrictedFloatType {
    unrestricted: Option<terminals::Unrestricted>,
    type_: FloatType
}

/// FloatType ::
///     float
///     double
pub enum FloatType {
    Float(terminals::Float),
    Double(terminals::Double)
}

/// StringType ::
///     ByteString
///     DOMString
///     USVString
pub enum StringType {
    Byte(terminals::ByteString),
    DOM(terminals::DOMString),
    USV(terminals::USVString)
}

/// RecordType ::
///     record < StringType , TypeWithExtendedAttributes >
pub struct RecordType {
    record: terminals::Record,
    generics: Generics<RecordTypeGenerics>
}

pub struct RecordTypeGenerics {
    string_type: StringType,
    comma: terminals::Comma,
    type_: TypeWithExtendedAttributes
}

/// UnionType ::
///     ( UnionMemberType or UnionMemberType UnionMemberTypes )
/// UnionMemberTypes ::
///     or UnionMemberType UnionMemberTypes
///     ε
pub struct UnionType {
    punctuated: Punctuated<UnionMemberType, terminals::Or>
}

/// UnionMemberType ::
///     ExtendedAttributeList NonAnyType
///     UnionType Null
pub enum UnionMemberType {
    Attributed(AttributedUnionMemberType),
    Simple(SimpleUnionMemberType)
}

pub struct AttributedUnionMemberType {
    attributes: ExtendedAttributeList,
    type_: NonAnyType
}

pub struct SimpleUnionMemberType {
    type_: MayBeNull<UnionType>
}
