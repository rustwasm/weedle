macro_rules! generate_tokens {
    ($( $typ:ident => $tok:expr ),*) => {
        $(
            #[derive(Debug, Default, PartialEq, Eq)]
            pub struct $typ;

            impl $crate::Parse for $typ {
                named!(parse -> Self, do_parse!(
                    ws!(tag!($tok)) >>
                    ($typ)
                ));
            }
        )*
    };
}

generate_tokens! {
    OpenParen => "{",
    CloseParen => "}",
    OpenBracket => "[",
    CloseBracket => "]",
    OpenBrace => "(",
    CloseBrace => ")",

    Comma => ",",
    Minus => "-",
    Dot => ".",
    Ellipsis => "...",
    Colon => ":",
    SemiColon => ";",
    LessThan => "<",
    Assign => "=",
    GreaterThan => ">",
    QMark => "?",

    Or => "or",
    Optional => "optional",
    Attribute => "attribute",
    Callback => "callback",
    Const => "const",
    Deleter => "deleter",
    Dictionary => "dictionary",
    Enum => "enum",
    Getter => "getter",
    Includes => "includes",
    Inherit => "inherit",
    Interface => "interface",
    Iterable => "iterable",
    Maplike => "maplike",
    Namespace => "namespace",
    Partial => "partial",
    Required => "required",
    Setlike => "setlike",
    Setter => "setter",
    Static => "static",
    Stringifier => "stringifier",
    Typedef => "typedef",
    Unrestricted => "unrestricted",
    Symbol => "symbol",

    NegInfinity => "-Infinity",
    ByteString => "ByteString",
    DOMString => "DOMString",
    FrozenArray => "FrozenString",
    Infinity => "Infinity",
    NaN => "NaN",
    USVString => "USVString",
    Any => "any",
    Boolean => "boolean",
    Byte => "byte",
    Double => "double",
    False => "false",
    Float => "float",
    Long => "long",
    Null => "null",
    Object => "object",
    Octet => "octect",
    Sequence => "sequence",
    Short => "short",
    True => "true",
    Unsigned => "unsigned",
    Void => "void",
    Record => "record",

    ArrayBuffer => "ArrayBuffer",
    DataView => "DataView",
    Int8Array => "Int8Array",
    Int16Array => "Int16Array",
    Int32Array => "Int32Array",
    Uint8Array => "Uint8Array",
    Uint16Array => "Uint16Array",
    Uint32Array => "Uint32Array",
    Uint8ClampedArray => "Uint8ClampedArray",
    Float32Array => "Float32Array",
    Float64Array => "Float64Array",

    Promise => "Promise",
    Error => "Error"
}

#[cfg(test)]
mod test {
    macro_rules! generate_tests {
        ($($m:ident, $typ:ident, $string:expr);*) => {
            $(
                mod $m {
                    use super::super::$typ;
                    use Parse;

                    #[test]
                    fn should_parse() {
                        let (rem, parsed) = $typ::parse($string).unwrap();
                        assert_eq!(rem, "");
                        assert_eq!(parsed, $typ);
                    }

                    #[test]
                    fn should_parse_with_preceding_spaces() {
                        let (rem, parsed) = $typ::parse(concat!("  ", $string)).unwrap();
                        assert_eq!(rem, "");
                        assert_eq!(parsed, $typ);
                    }

                    #[test]
                    fn should_parse_with_succeeding_spaces() {
                        let (rem, parsed) = $typ::parse(concat!($string, "  ")).unwrap();
                        assert_eq!(rem, "");
                        assert_eq!(parsed, $typ);
                    }

                    #[test]
                    fn should_parse_with_surrounding_spaces() {
                        let (rem, parsed) = $typ::parse(concat!("  ", $string, "  ")).unwrap();
                        assert_eq!(rem, "");
                        assert_eq!(parsed, $typ);
                    }

                    #[test]
                    fn should_parse_if_anything_next() {
                        let (rem, parsed) = $typ::parse(concat!($string, "  anything")).unwrap();
                        assert_eq!(rem, "anything");
                        assert_eq!(parsed, $typ);
                    }
                }
            )*
        };
    }

    generate_tests![
        openparen, OpenParen, "{";
        closeparen, CloseParen, "}";
        openbracket, OpenBracket, "[";
        closebracket, CloseBracket, "]";
        openbrace, OpenBrace, "(";
        closebrace, CloseBrace, ")";
        comma, Comma, ",";
        minus, Minus, "-";
        dot, Dot, ".";
        ellipsis, Ellipsis, "...";
        colon, Colon, ":";
        semicolon, SemiColon, ";";
        lessthan, LessThan, "<";
        assign, Assign, "=";
        greaterthan, GreaterThan, ">";
        qmark, QMark, "?";
        or, Or, "or";
        optional, Optional, "optional";
        attribute, Attribute, "attribute";
        callback, Callback, "callback";
        const_, Const, "const";
        deleter, Deleter, "deleter";
        dictionary, Dictionary, "dictionary";
        enum_, Enum, "enum";
        getter, Getter, "getter";
        includes, Includes, "includes";
        inherit, Inherit, "inherit";
        interface, Interface, "interface";
        iterable, Iterable, "iterable";
        maplike, Maplike, "maplike";
        namespace, Namespace, "namespace";
        partial, Partial, "partial";
        required, Required, "required";
        setlike, Setlike, "setlike";
        setter, Setter, "setter";
        static_, Static, "static";
        stringifier, Stringifier, "stringifier";
        typedef, Typedef, "typedef";
        unrestricted, Unrestricted, "unrestricted";
        symbol, Symbol, "symbol";
        neginfinity, NegInfinity, "-Infinity";
        bytestring, ByteString, "ByteString";
        domstring, DOMString, "DOMString";
        frozenarray, FrozenArray, "FrozenString";
        infinity, Infinity, "Infinity";
        nan, NaN, "NaN";
        usvstring, USVString, "USVString";
        any, Any, "any";
        boolean, Boolean, "boolean";
        byte, Byte, "byte";
        double, Double, "double";
        false_, False, "false";
        float, Float, "float";
        long, Long, "long";
        null, Null, "null";
        object, Object, "object";
        octet, Octet, "octect";
        sequence, Sequence, "sequence";
        short, Short, "short";
        true_, True, "true";
        unsigned, Unsigned, "unsigned";
        void, Void, "void";
        record, Record, "record";
        arraybuffer, ArrayBuffer, "ArrayBuffer";
        dataview, DataView, "DataView";
        int8array, Int8Array, "Int8Array";
        int16array, Int16Array, "Int16Array";
        int32array, Int32Array, "Int32Array";
        uint8array, Uint8Array, "Uint8Array";
        uint16array, Uint16Array, "Uint16Array";
        uint32array, Uint32Array, "Uint32Array";
        uint8clampedarray, Uint8ClampedArray, "Uint8ClampedArray";
        float32array, Float32Array, "Float32Array";
        float64array, Float64Array, "Float64Array";
        promise, Promise, "Promise";
        error, Error, "Error"
    ];
}
