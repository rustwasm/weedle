use literals::*;
use terminals::*;
use Parse;
use common::*;
use attributes::*;

/// TypeWithExtendedAttributes ::
///     ExtendedAttributeList Type
#[derive(Debug, PartialEq)]
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
#[derive(Debug, PartialEq)]
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


/// Type ::
///     SingleType
///     UnionType Null
#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
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
#[derive(Debug, PartialEq)]
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
#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
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
#[derive(Debug, PartialEq)]
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
#[derive(Debug, PartialEq)]
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
#[derive(Debug, PartialEq)]
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
#[derive(Debug, PartialEq)]
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
#[derive(Debug, PartialEq)]
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
#[derive(Debug, PartialEq)]
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
#[derive(Debug, PartialEq)]
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
#[derive(Debug, PartialEq)]
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
#[derive(Debug, PartialEq)]
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
#[derive(Debug, PartialEq)]
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
#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
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
#[derive(Debug, PartialEq)]
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
#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub struct SimpleUnionMemberType {
    pub type_: MayBeNull<UnionType>
}

impl Parse for SimpleUnionMemberType {
    named!(parse -> Self, do_parse!(
        type_: weedle!(MayBeNull<UnionType>) >>
        (SimpleUnionMemberType { type_ })
    ));
}

/// Typedef ::
///     typedef TypeWithExtendedAttributes identifier ;
#[derive(Debug, PartialEq)]
pub struct TypeDefinition {
    typedef: Typedef,
    type_: TypeWithExtendedAttributes,
    identifier: Identifier,
    semi_colon: SemiColon
}

impl Parse for TypeDefinition {
    named!(parse -> Self, do_parse!(
        typedef: weedle!(Typedef) >>
        type_: weedle!(TypeWithExtendedAttributes) >>
        identifier: weedle!(Identifier) >>
        semi_colon: weedle!(SemiColon) >>
        (TypeDefinition { typedef, type_, identifier, semi_colon })
    ));
}
