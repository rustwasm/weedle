use attribute::ExtendedAttributeList;
use common::{Braced, Generics, Identifier, Punctuated};
use term;
use Parse;

/// Parses a union of types
pub type UnionType = Braced<Punctuated<UnionMemberType, term!(or)>>;

ast_types! {
    /// Parses either single type or a union type
    enum Type {
        /// Parses one of the single types
        Single(enum SingleType {
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
        }),
        Union(MayBeNull<UnionType>),
    }

    /// Parses `sequence<Type>`
    struct SequenceType {
        sequence: term!(sequence),
        generics: Generics<Box<Type>>,
    }

    /// Parses `FrozenArray<Type>`
    struct FrozenArrayType {
        frozen_array: term!(FrozenArray),
        generics: Generics<Box<Type>>,
    }

    /// Parses a nullable type. Ex: `object | object??`
    ///
    /// `??` means an actual ? not an optional requirement
    #[derive(Copy)]
    struct MayBeNull(T) where [T: Parse] {
        type_: T,
        q_mark: Option<term::QMark>,
    }

    /// Parses a `Promise<Type|void>` type
    struct PromiseType {
        promise: term!(Promise),
        generics: Generics<Box<ReturnType>>,
    }

    /// Parses `unsigned? short|long|(long long)`
    #[derive(Copy)]
    enum IntegerType {
        /// Parses `unsigned? long long`
        #[derive(Copy)]
        LongLong(struct LongLongType {
            unsigned: Option<term!(unsigned)>,
            long_long: (term!(long), term!(long)),
        }),
        /// Parses `unsigned? long`
        #[derive(Copy)]
        Long(struct LongType {
            unsigned: Option<term!(unsigned)>,
            long: term!(long),
        }),
        /// Parses `unsigned? short`
        #[derive(Copy)]
        Short(struct ShortType {
            unsigned: Option<term!(unsigned)>,
            short: term!(short),
        }),
    }

    /// Parses `unrestricted? float|double`
    #[derive(Copy)]
    enum FloatingPointType {
        /// Parses `unrestricted? float`
        #[derive(Copy)]
        Float(struct FloatType {
            unrestricted: Option<term!(unrestricted)>,
            float: term!(float),
        }),
        /// Parses `unrestricted? double`
        #[derive(Copy)]
        Double(struct DoubleType {
            unrestricted: Option<term!(unrestricted)>,
            double: term!(double),
        }),
    }

    /// Parses `record<StringType, Type>`
    struct RecordType {
        record: term!(record),
        generics: Generics<(StringType, term!(,), Box<Type>)>,
    }

    /// Parses one of the string types `ByteString|DOMString|USVString`
    enum StringType {
        Byte(term!(ByteString)),
        DOM(term!(DOMString)),
        USV(term!(USVString)),
    }

    /// Parses one of the member of a union type
    enum UnionMemberType {
        /// Parses one of the types
        Single(enum UnionSingleType {
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
        }),
        Union(MayBeNull<UnionType>),
    }

    /// Parses a const type
    enum ConstType {
        Integer(MayBeNull<IntegerType>),
        FloatingPoint(MayBeNull<FloatingPointType>),
        Boolean(MayBeNull<term!(boolean)>),
        Byte(MayBeNull<term!(byte)>),
        Octet(MayBeNull<term!(octet)>),
        Identifier(MayBeNull<Identifier>),
    }

    /// Parses the return type which may be `void` or any given Type
    enum ReturnType {
        Void(term!(void)),
        Type(Type),
    }

    /// Parses `[attributes]? type`
    struct AttributedType {
        attributes: Option<ExtendedAttributeList>,
        type_: Type,
    }
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

    test!(should_parse_type_as_identifier { "DOMStringMap" =>
        // if type is not parsed as identifier, it is parsed as `DOMString` and 'Map' is left
        "";
        ::types::Type;
    });
}
