use Parse;
use common::*;
use term;
use attribute::*;

/// Parses either single type or a union type
#[derive(Debug, PartialEq)]
pub enum Type {
    Single(SingleType),
    Union(MayBeNull<UnionType>),
}

impl Parse for Type {
    named!(parse -> Self, alt_complete!(
        weedle!(SingleType) => {|inner| Type::Single(inner)} |
        weedle!(MayBeNull<UnionType>) => {|inner| Type::Union(inner)}
    ));
}

/// Parses one of the single types
#[derive(Debug, PartialEq)]
pub enum SingleType {
    Any(term!(any)),
    Promise(PromiseType),
    Integer(MayBeNull<IntegerType>),
    FloatingPoint(MayBeNull<FloatingPointType>),
    Boolean(MayBeNull<term!(boolean)>),
    Byte(MayBeNull<term!(byte)>),
    Octet(MayBeNull<term!(octet)>),
    ByteString(MayBeNull<term!(ByteString)>),
    DOMString(MayBeNull<term!(DOMString)>),
    USVString(MayBeNull<term!(USVString)>),
    Sequence(MayBeNull<SequenceType>),
    Object(MayBeNull<term!(object)>),
    Symbol(MayBeNull<term!(symbol)>),
    Error(MayBeNull<term!(Error)>),
    ArrayBuffer(MayBeNull<term!(ArrayBuffer)>),
    DataView(MayBeNull<term!(DataView)>),
    Int8Array(MayBeNull<term!(Int8Array)>),
    Int16Array(MayBeNull<term!(Int16Array)>),
    Int32Array(MayBeNull<term!(Int32Array)>),
    Uint8Array(MayBeNull<term!(Uint8Array)>),
    Uint16Array(MayBeNull<term!(Uint16Array)>),
    Uint32Array(MayBeNull<term!(Uint32Array)>),
    Uint8ClampedArray(MayBeNull<term!(Uint8ClampedArray)>),
    Float32Array(MayBeNull<term!(Float32Array)>),
    Float64Array(MayBeNull<term!(Float64Array)>),
    FrozenArrayType(MayBeNull<FrozenArrayType>),
    RecordType(MayBeNull<RecordType>),
    Identifier(MayBeNull<Identifier>),
}

impl Parse for SingleType {
    named!(parse -> Self, alt!(
        weedle!(term!(any)) => {|inner| SingleType::Any(inner)} |
        weedle!(PromiseType) => {|inner| SingleType::Promise(inner)} |
        weedle!(MayBeNull<IntegerType>) => {|inner| SingleType::Integer(inner)} |
        weedle!(MayBeNull<FloatingPointType>) => {|inner| SingleType::FloatingPoint(inner)} |
        weedle!(MayBeNull<term!(boolean)>) => {|inner| SingleType::Boolean(inner)} |
        weedle!(MayBeNull<term!(byte)>) => {|inner| SingleType::Byte(inner)} |
        weedle!(MayBeNull<term!(octet)>) => {|inner| SingleType::Octet(inner)} |
        weedle!(MayBeNull<term!(ByteString)>) => {|inner| SingleType::ByteString(inner)} |
        weedle!(MayBeNull<term!(DOMString)>) => {|inner| SingleType::DOMString(inner)} |
        weedle!(MayBeNull<term!(USVString)>) => {|inner| SingleType::USVString(inner)} |
        weedle!(MayBeNull<SequenceType>) => {|inner| SingleType::Sequence(inner)} |
        weedle!(MayBeNull<term!(object)>) => {|inner| SingleType::Object(inner)} |
        weedle!(MayBeNull<term!(symbol)>) => {|inner| SingleType::Symbol(inner)} |
        weedle!(MayBeNull<term!(Error)>) => {|inner| SingleType::Error(inner)} |
        weedle!(MayBeNull<term!(ArrayBuffer)>) => {|inner| SingleType::ArrayBuffer(inner)} |
        weedle!(MayBeNull<term!(DataView)>) => {|inner| SingleType::DataView(inner)} |
        weedle!(MayBeNull<term!(Int8Array)>) => {|inner| SingleType::Int8Array(inner)} |
        weedle!(MayBeNull<term!(Int16Array)>) => {|inner| SingleType::Int16Array(inner)} |
        weedle!(MayBeNull<term!(Int32Array)>) => {|inner| SingleType::Int32Array(inner)} |
        weedle!(MayBeNull<term!(Uint8Array)>) => {|inner| SingleType::Uint8Array(inner)} |
        weedle!(MayBeNull<term!(Uint16Array)>) => {|inner| SingleType::Uint16Array(inner)} |
        weedle!(MayBeNull<term!(Uint32Array)>) => {|inner| SingleType::Uint32Array(inner)} |
        weedle!(MayBeNull<term!(Uint8ClampedArray)>) => {|inner| SingleType::Uint8ClampedArray(inner)} |
        weedle!(MayBeNull<term!(Float32Array)>) => {|inner| SingleType::Float32Array(inner)} |
        weedle!(MayBeNull<term!(Float64Array)>) => {|inner| SingleType::Float64Array(inner)} |
        weedle!(MayBeNull<FrozenArrayType>) => {|inner| SingleType::FrozenArrayType(inner)} |
        weedle!(MayBeNull<RecordType>) => {|inner| SingleType::RecordType(inner)} |
        weedle!(MayBeNull<Identifier>) => {|inner| SingleType::Identifier(inner)}
    ));
}

/// Parses `sequence<Type>`
#[derive(Debug, PartialEq)]
pub struct SequenceType {
    pub sequence: term!(sequence),
    pub generics: Generics<Box<Type>>,
}

impl Parse for SequenceType {
    named!(parse -> Self, do_parse!(
        sequence: weedle!(term!(sequence)) >>
        generics: weedle!(Generics<Box<Type>>) >>
        (SequenceType { sequence, generics })
    ));
}

/// Parses `FrozenArray<Type>`
#[derive(Debug, PartialEq)]
pub struct FrozenArrayType {
    pub frozen_array: term!(FrozenArray),
    pub generics: Generics<Box<Type>>,
}

impl Parse for FrozenArrayType {
    named!(parse -> Self, do_parse!(
        frozen_array: weedle!(term!(FrozenArray)) >>
        generics: weedle!(Generics<Box<Type>>) >>
        (FrozenArrayType { frozen_array, generics })
    ));
}

/// Parses a nullable type. Ex: `object | object??`
///
/// `??` means an actual ? not an optional requirement
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

/// Parses a `Promise<Type|void>` type
#[derive(Debug, PartialEq)]
pub struct PromiseType {
    pub promise: term!(Promise),
    pub generics: Generics<Box<ReturnType>>,
}

impl Parse for PromiseType {
    named!(parse -> Self, do_parse!(
        promise: weedle!(term!(Promise)) >>
        generics: weedle!(Generics<Box<ReturnType>>) >>
        (PromiseType { promise, generics })
    ));
}

/// Parses `unsigned? short|long|(long long)`
#[derive(Debug, PartialEq)]
pub enum IntegerType {
    Short(ShortType),
    LongLong(LongLongType),
    Long(LongType),
}

impl Parse for IntegerType {
    named!(parse -> Self, alt!(
        weedle!(ShortType) => {|inner| IntegerType::Short(inner)} |
        weedle!(LongLongType) => {|inner| IntegerType::LongLong(inner)} |
        weedle!(LongType) => {|inner| IntegerType::Long(inner)}
    ));
}

/// Parses `unsigned? short`
#[derive(Debug, PartialEq)]
pub struct ShortType {
    pub unsigned: Option<term!(unsigned)>,
    pub short: term!(short)
}

impl Parse for ShortType {
    named!(parse -> Self, do_parse!(
        unsigned: weedle!(Option<term!(unsigned)>) >>
        short: weedle!(term!(short)) >>
        (ShortType { unsigned, short })
    ));
}

/// Parses `unsigned? long`
#[derive(Debug, PartialEq)]
pub struct LongType {
    pub unsigned: Option<term!(unsigned)>,
    pub long: term!(long)
}

impl Parse for LongType {
    named!(parse -> Self, do_parse!(
        unsigned: weedle!(Option<term!(unsigned)>) >>
        long: weedle!(term!(long)) >>
        (LongType { unsigned, long })
    ));
}

/// Parses `unsigned? long long`
#[derive(Debug, PartialEq)]
pub struct LongLongType {
    pub unsigned: Option<term!(unsigned)>,
    pub long_long: (term!(long), term!(long))
}

impl Parse for LongLongType {
    named!(parse -> Self, do_parse!(
        unsigned: weedle!(Option<term!(unsigned)>) >>
        long_long: weedle!((term!(long), term!(long))) >>
        (LongLongType { unsigned, long_long })
    ));
}

/// Parses `unrestricted? float|double`
#[derive(Debug, PartialEq)]
pub enum FloatingPointType {
    Float(FloatType),
    Double(DoubleType),
}

impl Parse for FloatingPointType {
    named!(parse -> Self, alt!(
        weedle!(FloatType) => {|inner| FloatingPointType::Float(inner)} |
        weedle!(DoubleType) => {|inner| FloatingPointType::Double(inner)}
    ));
}

/// Parses `unrestricted? float`
#[derive(Debug, PartialEq)]
pub struct FloatType {
    pub unrestricted: Option<term!(unrestricted)>,
    pub float: term!(float)
}

impl Parse for FloatType {
    named!(parse -> Self, do_parse!(
        unrestricted: weedle!(Option<term!(unrestricted)>) >>
        float: weedle!(term!(float)) >>
        (FloatType { unrestricted, float })
    ));
}

/// Parses `unrestricted? double`
#[derive(Debug, PartialEq)]
pub struct DoubleType {
    pub unrestricted: Option<term!(unrestricted)>,
    pub double: term!(double)
}

impl Parse for DoubleType {
    named!(parse -> Self, do_parse!(
        unrestricted: weedle!(Option<term!(unrestricted)>) >>
        double: weedle!(term!(double)) >>
        (DoubleType { unrestricted, double })
    ));
}

/// Parses `record<StringType, Type>`
#[derive(Debug, PartialEq)]
pub struct RecordType {
    pub record: term!(record),
    pub generics: Generics<(StringType, term!(,), Box<Type>)>,
}

impl Parse for RecordType {
    named!(parse -> Self, do_parse!(
        record: weedle!(term!(record)) >>
        generics: weedle!(Generics<(StringType, term!(,), Box<Type>)>) >>
        (RecordType { record, generics })
    ));
}

/// Parses one of the string types `ByteString|DOMString|USVString`
#[derive(Debug, PartialEq)]
pub enum StringType {
    Byte(term!(ByteString)),
    DOM(term!(DOMString)),
    USV(term!(USVString)),
}

impl Parse for StringType {
    named!(parse -> Self, alt!(
        weedle!(term!(ByteString)) => {|inner| StringType::Byte(inner)} |
        weedle!(term!(DOMString)) => {|inner| StringType::DOM(inner)} |
        weedle!(term!(USVString)) => {|inner| StringType::USV(inner)}
    ));
}

/// Parses a union of types
pub type UnionType = Braced<Punctuated<UnionMemberType, term!(or)>>;

/// Parses one of the member of a union type
#[derive(Debug, PartialEq)]
pub enum UnionMemberType {
    Single(UnionSingleType),
    Union(MayBeNull<UnionType>),
}

impl Parse for UnionMemberType {
    named!(parse -> Self, alt!(
        weedle!(UnionSingleType) => {|inner| UnionMemberType::Single(inner)} |
        weedle!(MayBeNull<UnionType>) => {|inner| UnionMemberType::Union(inner)}
    ));
}

/// Parses one of the types
#[derive(Debug, PartialEq)]
pub enum UnionSingleType {
    Promise(PromiseType),
    Integer(MayBeNull<IntegerType>),
    FloatingPoint(MayBeNull<FloatingPointType>),
    Boolean(MayBeNull<term!(boolean)>),
    Byte(MayBeNull<term!(byte)>),
    Octet(MayBeNull<term!(octet)>),
    ByteString(MayBeNull<term!(ByteString)>),
    DOMString(MayBeNull<term!(DOMString)>),
    USVString(MayBeNull<term!(USVString)>),
    Sequence(MayBeNull<SequenceType>),
    Object(MayBeNull<term!(object)>),
    Symbol(MayBeNull<term!(symbol)>),
    Error(MayBeNull<term!(Error)>),
    ArrayBuffer(MayBeNull<term!(ArrayBuffer)>),
    DataView(MayBeNull<term!(DataView)>),
    Int8Array(MayBeNull<term!(Int8Array)>),
    Int16Array(MayBeNull<term!(Int16Array)>),
    Int32Array(MayBeNull<term!(Int32Array)>),
    Uint8Array(MayBeNull<term!(Uint8Array)>),
    Uint16Array(MayBeNull<term!(Uint16Array)>),
    Uint32Array(MayBeNull<term!(Uint32Array)>),
    Uint8ClampedArray(MayBeNull<term!(Uint8ClampedArray)>),
    Float32Array(MayBeNull<term!(Float32Array)>),
    Float64Array(MayBeNull<term!(Float64Array)>),
    FrozenArrayType(MayBeNull<FrozenArrayType>),
    RecordType(MayBeNull<RecordType>),
    Identifier(MayBeNull<Identifier>),
}

impl Parse for UnionSingleType {
    named!(parse -> Self, alt!(
        weedle!(PromiseType) => {|inner| UnionSingleType::Promise(inner)} |
        weedle!(MayBeNull<IntegerType>) => {|inner| UnionSingleType::Integer(inner)} |
        weedle!(MayBeNull<FloatingPointType>) => {|inner| UnionSingleType::FloatingPoint(inner)} |
        weedle!(MayBeNull<term!(boolean)>) => {|inner| UnionSingleType::Boolean(inner)} |
        weedle!(MayBeNull<term!(byte)>) => {|inner| UnionSingleType::Byte(inner)} |
        weedle!(MayBeNull<term!(octet)>) => {|inner| UnionSingleType::Octet(inner)} |
        weedle!(MayBeNull<term!(ByteString)>) => {|inner| UnionSingleType::ByteString(inner)} |
        weedle!(MayBeNull<term!(DOMString)>) => {|inner| UnionSingleType::DOMString(inner)} |
        weedle!(MayBeNull<term!(USVString)>) => {|inner| UnionSingleType::USVString(inner)} |
        weedle!(MayBeNull<SequenceType>) => {|inner| UnionSingleType::Sequence(inner)} |
        weedle!(MayBeNull<term!(object)>) => {|inner| UnionSingleType::Object(inner)} |
        weedle!(MayBeNull<term!(symbol)>) => {|inner| UnionSingleType::Symbol(inner)} |
        weedle!(MayBeNull<term!(Error)>) => {|inner| UnionSingleType::Error(inner)} |
        weedle!(MayBeNull<term!(ArrayBuffer)>) => {|inner| UnionSingleType::ArrayBuffer(inner)} |
        weedle!(MayBeNull<term!(DataView)>) => {|inner| UnionSingleType::DataView(inner)} |
        weedle!(MayBeNull<term!(Int8Array)>) => {|inner| UnionSingleType::Int8Array(inner)} |
        weedle!(MayBeNull<term!(Int16Array)>) => {|inner| UnionSingleType::Int16Array(inner)} |
        weedle!(MayBeNull<term!(Int32Array)>) => {|inner| UnionSingleType::Int32Array(inner)} |
        weedle!(MayBeNull<term!(Uint8Array)>) => {|inner| UnionSingleType::Uint8Array(inner)} |
        weedle!(MayBeNull<term!(Uint16Array)>) => {|inner| UnionSingleType::Uint16Array(inner)} |
        weedle!(MayBeNull<term!(Uint32Array)>) => {|inner| UnionSingleType::Uint32Array(inner)} |
        weedle!(MayBeNull<term!(Uint8ClampedArray)>) => {|inner| UnionSingleType::Uint8ClampedArray(inner)} |
        weedle!(MayBeNull<term!(Float32Array)>) => {|inner| UnionSingleType::Float32Array(inner)} |
        weedle!(MayBeNull<term!(Float64Array)>) => {|inner| UnionSingleType::Float64Array(inner)} |
        weedle!(MayBeNull<FrozenArrayType>) => {|inner| UnionSingleType::FrozenArrayType(inner)} |
        weedle!(MayBeNull<RecordType>) => {|inner| UnionSingleType::RecordType(inner)} |
        weedle!(MayBeNull<Identifier>) => {|inner| UnionSingleType::Identifier(inner)}
    ));
}

/// Parses a const type
#[derive(Debug, PartialEq)]
pub enum ConstType {
    Integer(MayBeNull<IntegerType>),
    FloatingPoint(MayBeNull<FloatingPointType>),
    Boolean(MayBeNull<term!(boolean)>),
    Byte(MayBeNull<term!(byte)>),
    Octet(MayBeNull<term!(octet)>),
    Identifier(MayBeNull<Identifier>)
}

impl Parse for ConstType {
    named!(parse -> Self, alt!(
        weedle!(MayBeNull<IntegerType>) => {|inner| ConstType::Integer(inner)} |
        weedle!(MayBeNull<FloatingPointType>) => {|inner| ConstType::FloatingPoint(inner)} |
        weedle!(MayBeNull<term!(boolean)>) => {|inner| ConstType::Boolean(inner)} |
        weedle!(MayBeNull<term!(byte)>) => {|inner| ConstType::Byte(inner)} |
        weedle!(MayBeNull<term!(octet)>) => {|inner| ConstType::Octet(inner)} |
        weedle!(MayBeNull<Identifier>) => {|inner| ConstType::Identifier(inner)}
    ));
}

/// Parses the return type which may be `void` or any given Type
#[derive(Debug, PartialEq)]
pub enum ReturnType {
    Void(term!(void)),
    Type(Type),
}

impl Parse for ReturnType {
    named!(parse -> Self, alt_complete!(
        weedle!(term!(void)) => {|inner| ReturnType::Void(inner)} |
        weedle!(Type) => {|inner| ReturnType::Type(inner)}
    ));
}

/// Parses `[attributes]? type`
#[derive(Debug, PartialEq)]
pub struct AttributedType {
    pub attributes: Option<ExtendedAttributeList>,
    pub type_: Type
}

impl Parse for AttributedType {
    named!(parse -> Self, do_parse!(
        attributes: weedle!(Option<ExtendedAttributeList>) >>
        type_: weedle!(Type) >>
        (AttributedType { attributes, type_ })
    ));
}

#[cfg(test)]
mod test {
    use super::*;

    test!(should_parse_may_be_null { "short" =>
        "";
        MayBeNull<::types::IntegerType>;
        q_mark.is_none();
    });

    test!(should_parse_nullable { "short?" =>
        "";
        MayBeNull<::types::IntegerType>;
        q_mark.is_some();
    });

    test_variants!(
        ReturnType {
            Void == "void",
            Type == "any",
        }
    );

    test_variants!(
        ConstType {
            Integer == "short",
            FloatingPoint == "float",
            Boolean == "boolean",
            Byte == "byte",
            Octet == "octet",
            Identifier == "name",
        }
    );

    test_variants!(
        UnionSingleType {
            Promise == "Promise<long>",
            Integer == "long",
            FloatingPoint == "float",
            Boolean == "boolean",
            Byte == "byte",
            Octet == "octet",
            ByteString == "ByteString",
            DOMString == "DOMString",
            USVString == "USVString",
            Sequence == "sequence<short>",
            Object == "object",
            Symbol == "symbol",
            Error == "Error",
            ArrayBuffer == "ArrayBuffer",
            DataView == "DataView",
            Int8Array == "Int8Array",
            Int16Array == "Int16Array",
            Int32Array == "Int32Array",
            Uint8Array == "Uint8Array",
            Uint16Array == "Uint16Array",
            Uint32Array == "Uint32Array",
            Uint8ClampedArray == "Uint8ClampedArray",
            Float32Array == "Float32Array",
            Float64Array == "Float64Array",
            FrozenArrayType == "FrozenArray<short>",
            RecordType == "record<DOMString, short>",
            Identifier == "mango"
        }
    );

    test_variants!(
        UnionMemberType {
            Single == "byte",
            Union == "(byte or byte)"
        }
    );

    test_variants!(
        StringType {
            DOM == "DOMString",
            USV == "USVString",
            Byte == "ByteString"
        }
    );

    test!(should_parse_record_type { "record<DOMString, short>" =>
        "";
        RecordType;
    });

    test!(should_parse_double_type { "double" =>
        "";
        DoubleType;
    });

    test!(should_parse_float_type { "float" =>
        "";
        FloatType;
    });

    test_variants!(
        FloatingPointType {
            Float == "float",
            Double == "double"
        }
    );

    test!(should_parse_long_long_type { "long long" =>
        "";
        LongLongType;
    });


    test!(should_parse_long_type { "long" =>
        "";
        LongType;
    });

    test!(should_parse_short_type { "short" =>
        "";
        ShortType;
    });

    test_variants!(
        IntegerType {
            Short == "short",
            Long == "long",
            LongLong == "long long"
        }
    );

    test!(should_parse_promise_type { "Promise<short>" =>
        "";
        PromiseType;
    });

    test!(should_parse_frozen_array_type { "FrozenArray<short>" =>
        "";
        FrozenArrayType;
    });

    test!(should_parse_sequence_type { "sequence<short>" =>
        "";
        SequenceType;
    });

    test_variants!(
        SingleType {
            Any == "any",
            Promise == "Promise<short>",
            Integer == "long",
            FloatingPoint == "float",
            Boolean == "boolean",
            Byte == "byte",
            Octet == "octet",
            ByteString == "ByteString",
            DOMString == "DOMString",
            USVString == "USVString",
            Sequence == "sequence<short>",
            Object == "object",
            Symbol == "symbol",
            Error == "Error",
            ArrayBuffer == "ArrayBuffer",
            DataView == "DataView",
            Int8Array == "Int8Array",
            Int16Array == "Int16Array",
            Int32Array == "Int32Array",
            Uint8Array == "Uint8Array",
            Uint16Array == "Uint16Array",
            Uint32Array == "Uint32Array",
            Uint8ClampedArray == "Uint8ClampedArray",
            Float32Array == "Float32Array",
            Float64Array == "Float64Array",
            FrozenArrayType == "FrozenArray<short>",
            RecordType == "record<DOMString, short>",
            Identifier == "someName"
        }
    );

    test_variants!(
        Type {
            Single == "short",
            Union == "(short or float)"
        }
    );

    test!(should_parse_attributed_type { "[Named] short" =>
        "";
        AttributedType;
        attributes.is_some();
    });
}
