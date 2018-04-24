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
    FrozenArray => "FrozenArray",
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
    Octet => "octet",
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
    Error => "Error",

    ReadOnly => "readonly",
    Mixin => "mixin"
}

#[macro_export]
macro_rules! term {
    (OpenParen) => { $crate::term::OpenParen };
    (CloseParen) => { $crate::term::CloseParen };
    (OpenBracket) => { $crate::term::OpenBracket };
    (CloseBracket) => { $crate::term::CloseBracket };
    (OpenBrace) => { $crate::term::OpenBrace };
    (CloseBrace) => { $crate::term::CloseBrace };
    (,) => { $crate::term::Comma };
    (-) => { $crate::term::Minus };
    (.) => { $crate::term::Dot };
    (...) => { $crate::term::Ellipsis };
    (:) => { $crate::term::Colon };
    (;) => { $crate::term::SemiColon };
    (<) => { $crate::term::LessThan };
    (=) => { $crate::term::Assign };
    (>) => { $crate::term::GreaterThan };
    (?) => { $crate::term::QMark };
    (or) => { $crate::term::Or };
    (optional) => { $crate::term::Optional };
    (attribute) => { $crate::term::Attribute };
    (callback) => { $crate::term::Callback };
    (const) => { $crate::term::Const };
    (deleter) => { $crate::term::Deleter };
    (dictionary) => { $crate::term::Dictionary };
    (enum) => { $crate::term::Enum };
    (getter) => { $crate::term::Getter };
    (includes) => { $crate::term::Includes };
    (inherit) => { $crate::term::Inherit };
    (interface) => { $crate::term::Interface };
    (iterable) => { $crate::term::Iterable };
    (maplike) => { $crate::term::Maplike };
    (namespace) => { $crate::term::Namespace };
    (partial) => { $crate::term::Partial };
    (required) => { $crate::term::Required };
    (setlike) => { $crate::term::Setlike };
    (setter) => { $crate::term::Setter };
    (static) => { $crate::term::Static };
    (stringifier) => { $crate::term::Stringifier };
    (typedef) => { $crate::term::Typedef };
    (unrestricted) => { $crate::term::Unrestricted };
    (symbol) => { $crate::term::Symbol };
    (-Infinity) => { $crate::term::NegInfinity };
    (ByteString) => { $crate::term::ByteString };
    (DOMString) => { $crate::term::DOMString };
    (FrozenArray) => { $crate::term::FrozenArray };
    (Infinity) => { $crate::term::Infinity };
    (NaN) => { $crate::term::NaN };
    (USVString) => { $crate::term::USVString };
    (any) => { $crate::term::Any };
    (boolean) => { $crate::term::Boolean };
    (byte) => { $crate::term::Byte };
    (double) => { $crate::term::Double };
    (false) => { $crate::term::False };
    (float) => { $crate::term::Float };
    (long) => { $crate::term::Long };
    (null) => { $crate::term::Null };
    (object) => { $crate::term::Object };
    (octet) => { $crate::term::Octet };
    (sequence) => { $crate::term::Sequence };
    (short) => { $crate::term::Short };
    (true) => { $crate::term::True };
    (unsigned) => { $crate::term::Unsigned };
    (void) => { $crate::term::Void };
    (record) => { $crate::term::Record };
    (ArrayBuffer) => { $crate::term::ArrayBuffer };
    (DataView) => { $crate::term::DataView };
    (Int8Array) => { $crate::term::Int8Array };
    (Int16Array) => { $crate::term::Int16Array };
    (Int32Array) => { $crate::term::Int32Array };
    (Uint8Array) => { $crate::term::Uint8Array };
    (Uint16Array) => { $crate::term::Uint16Array };
    (Uint32Array) => { $crate::term::Uint32Array };
    (Uint8ClampedArray) => { $crate::term::Uint8ClampedArray };
    (Float32Array) => { $crate::term::Float32Array };
    (Float64Array) => { $crate::term::Float64Array };
    (Promise) => { $crate::term::Promise };
    (Error) => { $crate::term::Error };
    (readonly) => { $crate::term::ReadOnly };
    (mixin) => { $crate::term::Mixin };
}

#[cfg(test)]
mod test {
    macro_rules! generate_tests {
        ($($m:ident, $typ:ident, $string:expr);*) => {
            $(
                mod $m {
                    use super::super::$typ;
                    use Parse;
                    use nom::types::CompleteStr;

                    #[test]
                    fn should_parse() {
                        let (rem, parsed) = $typ::parse(CompleteStr(concat!($string))).unwrap();
                        assert_eq!(rem, CompleteStr(""));
                        assert_eq!(parsed, $typ);
                    }

                    #[test]
                    fn should_parse_with_preceding_spaces() {
                        let (rem, parsed) = $typ::parse(CompleteStr(concat!("  ", $string))).unwrap();
                        assert_eq!(rem, CompleteStr(""));
                        assert_eq!(parsed, $typ);
                    }

                    #[test]
                    fn should_parse_with_succeeding_spaces() {
                        let (rem, parsed) = $typ::parse(CompleteStr(concat!($string, "  "))).unwrap();
                        assert_eq!(rem, CompleteStr(""));
                        assert_eq!(parsed, $typ);
                    }

                    #[test]
                    fn should_parse_with_surrounding_spaces() {
                        let (rem, parsed) = $typ::parse(CompleteStr(concat!("  ", $string, "  "))).unwrap();
                        assert_eq!(rem, CompleteStr(""));
                        assert_eq!(parsed, $typ);
                    }

                    #[test]
                    fn should_parse_if_anything_next() {
                        let (rem, parsed) = $typ::parse(CompleteStr(concat!($string, "  anything"))).unwrap();
                        assert_eq!(rem, CompleteStr("anything"));
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
