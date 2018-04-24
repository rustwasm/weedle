use literals::*;
use Parse;
use common::*;
use attributes::*;
use term;

/// ### Grammar
/// ```
/// TypeWithExtendedAttributes ::
///     ExtendedAttributeList Type
/// ```
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-TypeWithExtendedAttributes)
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

/// ### Grammar
/// ```
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
/// ```
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-BufferRelatedType)
#[derive(Debug, PartialEq)]
pub enum BufferRelatedType {
    ArrayBuffer(term!(ArrayBuffer)),
    DataView(term!(DataView)),
    Int8Array(term!(Int8Array)),
    Int16Array(term!(Int16Array)),
    Int32Array(term!(Int32Array)),
    Uint8Array(term!(Uint8Array)),
    Uint16Array(term!(Uint16Array)),
    Uint32Array(term!(Uint32Array)),
    Uint8ClampedArray(term!(Uint8ClampedArray)),
    Float32Array(term!(Float32Array)),
    Float64Array(term!(Float64Array)),
}

impl Parse for BufferRelatedType {
    named!(parse -> Self, alt_complete!(
        weedle!(term!(ArrayBuffer)) => {|inner| BufferRelatedType::ArrayBuffer(inner)} |
        weedle!(term!(DataView)) => {|inner| BufferRelatedType::DataView(inner)} |
        weedle!(term!(Int8Array)) => {|inner| BufferRelatedType::Int8Array(inner)} |
        weedle!(term!(Int16Array)) => {|inner| BufferRelatedType::Int16Array(inner)} |
        weedle!(term!(Int32Array)) => {|inner| BufferRelatedType::Int32Array(inner)} |
        weedle!(term!(Uint8Array)) => {|inner| BufferRelatedType::Uint8Array(inner)} |
        weedle!(term!(Uint16Array)) => {|inner| BufferRelatedType::Uint16Array(inner)} |
        weedle!(term!(Uint32Array)) => {|inner| BufferRelatedType::Uint32Array(inner)} |
        weedle!(term!(Uint8ClampedArray)) => {|inner| BufferRelatedType::Uint8ClampedArray(inner)} |
        weedle!(term!(Float32Array)) => {|inner| BufferRelatedType::Float32Array(inner)} |
        weedle!(term!(Float64Array)) => {|inner| BufferRelatedType::Float64Array(inner)}
    ));
}

/// ### Grammar
/// ```
/// Type ::
///     SingleType
///     UnionType Null
/// ```
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-Type)
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

/// ### Grammar
/// ```
/// SingleType ::
///     NonAnyType
///     any
/// ```
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-SingleType)
#[derive(Debug, PartialEq)]
pub enum SingleType {
    NonAny(NonAnyType),
    Any(term!(any)),
}

impl Parse for SingleType {
    named!(parse -> Self, alt_complete!(
        weedle!(NonAnyType) => {|inner| SingleType::NonAny(inner)} |
        weedle!(term!(any)) => {|inner| SingleType::Any(inner)}
    ));
}

/// ### Grammar
/// ```
/// NonAnyType ::
///     PromiseType ε
///     PrimitiveType Null
///     StringType Null
///     **identifier** Null
///     sequence < TypeWithExtendedAttributes > Null
///     object Null
///     symbol Null
///     Error Null
///     BufferRelatedType Null
///     FrozenArray < TypeWithExtendedAttributes > Null
///     RecordType Null
/// ```
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-NonAnyType)
#[derive(Debug, PartialEq)]
pub enum NonAnyType {
    Promise(PromiseType),
    MayBePrimitive(MayBeNull<PrimitiveType>),
    MayBeString(MayBeNull<StringType>),
    MayBeIdentifier(MayBeNull<Identifier>),
    MayBeSequence(MayBeNull<SequenceType>),
    MayBeObject(MayBeNull<term!(object)>),
    MayBeSymbol(MayBeNull<term!(symbol)>),
    MayBeError(MayBeNull<term!(Error)>),
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
        weedle!(MayBeNull<term!(object)>) => {|inner| NonAnyType::MayBeObject(inner)} |
        weedle!(MayBeNull<term!(symbol)>) => {|inner| NonAnyType::MayBeSymbol(inner)} |
        weedle!(MayBeNull<term!(Error)>) => {|inner| NonAnyType::MayBeError(inner)} |
        weedle!(MayBeNull<BufferRelatedType>) => {|inner| NonAnyType::MayBeBufferedRelated(inner)} |
        weedle!(MayBeNull<FrozenArrayType>) => {|inner| NonAnyType::MayBeFrozenArray(inner)} |
        weedle!(MayBeNull<RecordType>) => {|inner| NonAnyType::MayBeRecord(inner)}
    ));
}

#[derive(Debug, PartialEq)]
pub struct SequenceType {
    pub sequence: term!(sequence),
    pub generics: Generics<TypeWithExtendedAttributes>,
}

impl Parse for SequenceType {
    named!(parse -> Self, do_parse!(
        sequence: weedle!(term!(sequence)) >>
        generics: weedle!(Generics<TypeWithExtendedAttributes>) >>
        (SequenceType { sequence, generics })
    ));
}

#[derive(Debug, PartialEq)]
pub struct FrozenArrayType {
    pub frozen_array: term!(FrozenArray),
    pub generics: Generics<TypeWithExtendedAttributes>,
}

impl Parse for FrozenArrayType {
    named!(parse -> Self, do_parse!(
        frozen_array: weedle!(term!(FrozenArray)) >>
        generics: weedle!(Generics<TypeWithExtendedAttributes>) >>
        (FrozenArrayType { frozen_array, generics })
    ));
}

/// ### Grammar
/// ```
/// Null ::
///     ?
///     ε
/// ```
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-Null)
#[derive(Debug, PartialEq)]
pub struct MayBeNull<T> {
    pub type_: T,
    pub q_mark: Option<term::QMark>,
}

impl<T: Parse> Parse for MayBeNull<T> {
    named!(parse -> Self, do_parse!(
        type_: weedle!(T) >>
        q_mark: weedle!(Option<term!(?)>) >>
        (MayBeNull { type_, q_mark })
    ));
}

/// ### Grammar
/// ```
/// PromiseType ::
///    Promise < ReturnType >
/// ```
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-PromiseType)
#[derive(Debug, PartialEq)]
pub struct PromiseType {
    pub promise: term!(Promise),
    pub generics: Generics<ReturnType>,
}

impl Parse for PromiseType {
    named!(parse -> Self, do_parse!(
        promise: weedle!(term!(Promise)) >>
        generics: weedle!(Generics<ReturnType>) >>
        (PromiseType { promise, generics })
    ));
}

/// ### Grammar
/// ```
/// ReturnType ::
///     Type
///     void
/// ```
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-ReturnType)
#[derive(Debug, PartialEq)]
pub enum ReturnType {
    Type(Type),
    Void(term!(void)),
}

impl Parse for ReturnType {
    named!(parse -> Self, alt_complete!(
        weedle!(Type) => {|inner| ReturnType::Type(inner)} |
        weedle!(term!(void)) => {|inner| ReturnType::Void(inner)}
    ));
}

/// ### Grammar
/// ```
/// PrimitiveType ::
///     UnsignedIntegerType
///     UnrestrictedFloatType
///     boolean
///     byte
///     octet
/// ```
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-PrimitiveType)
#[derive(Debug, PartialEq)]
pub enum PrimitiveType {
    UnsignedIntegerType(UnsignedIntegerType),
    UnrestrictedFloatType(UnrestrictedFloatType),
    Boolean(term!(boolean)),
    Byte(term!(byte)),
    Octet(term!(octet)),
}

impl Parse for PrimitiveType {
    named!(parse -> Self, alt_complete!(
        weedle!(UnsignedIntegerType) => {|inner| PrimitiveType::UnsignedIntegerType(inner)} |
        weedle!(UnrestrictedFloatType) => {|inner| PrimitiveType::UnrestrictedFloatType(inner)} |
        weedle!(term!(boolean)) => {|inner| PrimitiveType::Boolean(inner)} |
        weedle!(term!(byte)) => {|inner| PrimitiveType::Byte(inner)} |
        weedle!(term!(octet)) => {|inner| PrimitiveType::Octet(inner)}
    ));
}

/// ### Grammar
/// ```
/// UnsignedIntegerType ::
///     unsigned IntegerType
///     IntegerType
/// ```
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-UnsignedIntegerType)
#[derive(Debug, PartialEq)]
pub struct UnsignedIntegerType {
    pub unsigned: Option<term!(unsigned)>,
    pub type_: IntegerType,
}

impl Parse for UnsignedIntegerType {
    named!(parse -> Self, do_parse!(
        unsigned: weedle!(Option<term!(unsigned)>) >>
        type_: weedle!(IntegerType) >>
        (UnsignedIntegerType { unsigned, type_ })
    ));
}

/// ### Grammar
/// ```
/// IntegerType ::
///     short
///     long OptionalLong
///
/// OptionalLong ::
///     long
///     ε
/// ```
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-IntegerType)
#[derive(Debug, PartialEq)]
pub enum IntegerType {
    Short(term!(short)),
    Long(LongType),
}

impl Parse for IntegerType {
    named!(parse -> Self, alt_complete!(
        weedle!(term!(short)) => {|inner| IntegerType::Short(inner)} |
        weedle!(LongType) => {|inner| IntegerType::Long(inner)}
    ));
}

#[derive(Debug, PartialEq)]
pub struct LongType {
    pub long: term!(long),
    pub optional: Option<term!(long)>,
}

impl Parse for LongType {
    named!(parse -> Self, do_parse!(
        long: weedle!(term!(long)) >>
        optional: weedle!(Option<term!(long)>) >>
        (LongType { long, optional })
    ));
}

/// ### Grammar
/// ```
/// UnrestrictedFloatType ::
///     unrestricted FloatType
///     FloatType
/// ```
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-UnrestrictedFloatType)
#[derive(Debug, PartialEq)]
pub struct UnrestrictedFloatType {
    pub unrestricted: Option<term!(unrestricted)>,
    pub type_: FloatType,
}

impl Parse for UnrestrictedFloatType {
    named!(parse -> Self, do_parse!(
        unrestricted: weedle!(Option<term!(unrestricted)>) >>
        type_: weedle!(FloatType) >>
        (UnrestrictedFloatType { unrestricted, type_ })
    ));
}

/// ### Grammar
/// ```
/// FloatType ::
///     float
///     double
/// ```
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-FloatType)
#[derive(Debug, PartialEq)]
pub enum FloatType {
    Float(term!(float)),
    Double(term!(double)),
}

impl Parse for FloatType {
    named!(parse -> Self, alt_complete!(
        weedle!(term!(float)) => {|inner| FloatType::Float(inner)} |
        weedle!(term!(double)) => {|inner| FloatType::Double(inner)}
    ));
}

/// ### Grammar
/// ```
/// StringType ::
///     ByteString
///     DOMString
///     USVString
/// ```
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-StringType)
#[derive(Debug, PartialEq)]
pub enum StringType {
    Byte(term!(ByteString)),
    DOM(term!(DOMString)),
    USV(term!(USVString)),
}

impl Parse for StringType {
    named!(parse -> Self, alt_complete!(
        weedle!(term!(ByteString)) => {|inner| StringType::Byte(inner)} |
        weedle!(term!(DOMString)) => {|inner| StringType::DOM(inner)} |
        weedle!(term!(USVString)) => {|inner| StringType::USV(inner)}
    ));
}

/// ### Grammar
/// ```
/// RecordType ::
///     record < StringType , TypeWithExtendedAttributes >
/// ```
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-RecordType)
#[derive(Debug, PartialEq)]
pub struct RecordType {
    pub record: term!(record),
    pub generics: Generics<RecordTypeGenerics>,
}

impl Parse for RecordType {
    named!(parse -> Self, do_parse!(
        record: weedle!(term!(record)) >>
        generics: weedle!(Generics<RecordTypeGenerics>) >>
        (RecordType { record, generics })
    ));
}

#[derive(Debug, PartialEq)]
pub struct RecordTypeGenerics {
    pub string_type: StringType,
    pub comma: term!(,),
    pub type_: TypeWithExtendedAttributes,
}

impl Parse for RecordTypeGenerics {
    named!(parse -> Self, do_parse!(
        string_type: weedle!(StringType) >>
        comma: weedle!(term!(,)) >>
        type_: weedle!(TypeWithExtendedAttributes) >>
        (RecordTypeGenerics { string_type, comma, type_ })
    ));
}

/// ### Grammar
/// ```
/// UnionType ::
///     ( UnionMemberType or UnionMemberType UnionMemberTypes )
/// UnionMemberTypes ::
///     or UnionMemberType UnionMemberTypes
///     ε
/// ```
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-UnionType)
#[derive(Debug, PartialEq)]
pub struct UnionType {
    pub punctuated: Punctuated<UnionMemberType, term!(or)>
}

impl Parse for UnionType {
    named!(parse -> Self, do_parse!(
        punctuated: weedle!(Punctuated<UnionMemberType, term!(or)>) >>
        (UnionType { punctuated })
    ));
}

/// ### Grammar
/// ```
/// UnionMemberType ::
///     ExtendedAttributeList NonAnyType
///     UnionType Null
/// ```
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-UnionMemberType)
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

/// ### Grammar
/// ```
/// Typedef ::
///     typedef TypeWithExtendedAttributes **identifier** ;
/// ```
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-Typedef)
#[derive(Debug, PartialEq)]
pub struct TypeDefinition {
    pub typedef: term!(typedef),
    pub type_: TypeWithExtendedAttributes,
    pub identifier: Identifier,
    pub semi_colon: term!(;)
}

impl Parse for TypeDefinition {
    named!(parse -> Self, do_parse!(
        typedef: weedle!(term!(typedef)) >>
        type_: weedle!(TypeWithExtendedAttributes) >>
        identifier: weedle!(Identifier) >>
        semi_colon: weedle!(term!(;)) >>
        (TypeDefinition { typedef, type_, identifier, semi_colon })
    ));
}

/// ### Grammar
/// ```
/// ConstType ::
///    PrimitiveType Null
///    **identifier** Null
/// ```
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#index-prod-ConstType)
#[derive(Debug, PartialEq)]
pub enum ConstType {
    Primitive(MayBeNull<PrimitiveType>),
    Identifier(MayBeNull<Identifier>)
}