macro_rules! generate_terms {
    ($( $(#[$attr:meta])* $typ:ident => $tok:expr ),*) => {
        $(
            $(#[$attr])*
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

macro_rules! generate_terms_for_names {
    ($( $(#[$attr:meta])* $typ:ident => $tok:expr ),*) => {
        $(
            $(#[$attr])*
            #[derive(Debug, Default, PartialEq, Eq)]
            pub struct $typ;

            impl $crate::Parse for $typ {
                named!(parse -> Self, do_parse!(
                    string: map!(
                        ws!(re_capture_static!(r"^(-?[A-Za-z][0-9A-Za-z]*)")),
                        ::literal::select_first
                    ) >>
                    err_if_not!(string.0 == $tok) >>
                    ($typ)
                ));
            }
        )*
    };
}

generate_terms! {
    #[doc="Represents the terminal symbol `{`"]
    OpenParen => "{",

    #[doc="Represents the terminal symbol `}`"]
    CloseParen => "}",

    #[doc="Represents the terminal symbol `[`"]
    OpenBracket => "[",

    #[doc="Represents the terminal symbol `]`"]
    CloseBracket => "]",

    #[doc="Represents the terminal symbol `(`"]
    OpenBrace => "(",

    #[doc="Represents the terminal symbol `)`"]
    CloseBrace => ")",

    #[doc="Represents the terminal symbol `,`"]
    Comma => ",",

    #[doc="Represents the terminal symbol `-`"]
    Minus => "-",

    #[doc="Represents the terminal symbol `.`"]
    Dot => ".",

    #[doc="Represents the terminal symbol `...`"]
    Ellipsis => "...",

    #[doc="Represents the terminal symbol `:`"]
    Colon => ":",

    #[doc="Represents the terminal symbol `;`"]
    SemiColon => ";",

    #[doc="Represents the terminal symbol `<`"]
    LessThan => "<",

    #[doc="Represents the terminal symbol `=`"]
    Assign => "=",

    #[doc="Represents the terminal symbol `>`"]
    GreaterThan => ">",

    #[doc="Represents the terminal symbol `?`"]
    QMark => "?"
}

generate_terms_for_names! {
    #[doc="Represents the terminal symbol `or`"]
    Or => "or",

    #[doc="Represents the terminal symbol `optional`"]
    Optional => "optional",

    #[doc="Represents the terminal symbol `attribute`"]
    Attribute => "attribute",

    #[doc="Represents the terminal symbol `callback`"]
    Callback => "callback",

    #[doc="Represents the terminal symbol `const`"]
    Const => "const",

    #[doc="Represents the terminal symbol `deleter`"]
    Deleter => "deleter",

    #[doc="Represents the terminal symbol `dictionary`"]
    Dictionary => "dictionary",

    #[doc="Represents the terminal symbol `enum`"]
    Enum => "enum",

    #[doc="Represents the terminal symbol `getter`"]
    Getter => "getter",

    #[doc="Represents the terminal symbol `includes`"]
    Includes => "includes",

    #[doc="Represents the terminal symbol `inherit`"]
    Inherit => "inherit",

    #[doc="Represents the terminal symbol `interface`"]
    Interface => "interface",

    #[doc="Represents the terminal symbol `iterable`"]
    Iterable => "iterable",

    #[doc="Represents the terminal symbol `maplike`"]
    Maplike => "maplike",

    #[doc="Represents the terminal symbol `namespace`"]
    Namespace => "namespace",

    #[doc="Represents the terminal symbol `partial`"]
    Partial => "partial",

    #[doc="Represents the terminal symbol `required`"]
    Required => "required",

    #[doc="Represents the terminal symbol `setlike`"]
    Setlike => "setlike",

    #[doc="Represents the terminal symbol `setter`"]
    Setter => "setter",

    #[doc="Represents the terminal symbol `static`"]
    Static => "static",

    #[doc="Represents the terminal symbol `stringifier`"]
    Stringifier => "stringifier",

    #[doc="Represents the terminal symbol `typedef`"]
    Typedef => "typedef",

    #[doc="Represents the terminal symbol `unrestricted`"]
    Unrestricted => "unrestricted",

    #[doc="Represents the terminal symbol `symbol`"]
    Symbol => "symbol",

    #[doc="Represents the terminal symbol `Infinity`"]
    NegInfinity => "-Infinity",

    #[doc="Represents the terminal symbol `ByteString`"]
    ByteString => "ByteString",

    #[doc="Represents the terminal symbol `DOMString`"]
    DOMString => "DOMString",

    #[doc="Represents the terminal symbol `FrozenArray`"]
    FrozenArray => "FrozenArray",

    #[doc="Represents the terminal symbol `Infinity`"]
    Infinity => "Infinity",

    #[doc="Represents the terminal symbol `NaN`"]
    NaN => "NaN",

    #[doc="Represents the terminal symbol `USVString`"]
    USVString => "USVString",

    #[doc="Represents the terminal symbol `any`"]
    Any => "any",

    #[doc="Represents the terminal symbol `boolean`"]
    Boolean => "boolean",

    #[doc="Represents the terminal symbol `byte`"]
    Byte => "byte",

    #[doc="Represents the terminal symbol `double`"]
    Double => "double",

    #[doc="Represents the terminal symbol `false`"]
    False => "false",

    #[doc="Represents the terminal symbol `float`"]
    Float => "float",

    #[doc="Represents the terminal symbol `long`"]
    Long => "long",

    #[doc="Represents the terminal symbol `null`"]
    Null => "null",

    #[doc="Represents the terminal symbol `object`"]
    Object => "object",

    #[doc="Represents the terminal symbol `octet`"]
    Octet => "octet",

    #[doc="Represents the terminal symbol `sequence`"]
    Sequence => "sequence",

    #[doc="Represents the terminal symbol `short`"]
    Short => "short",

    #[doc="Represents the terminal symbol `true`"]
    True => "true",

    #[doc="Represents the terminal symbol `unsigned`"]
    Unsigned => "unsigned",

    #[doc="Represents the terminal symbol `void`"]
    Void => "void",

    #[doc="Represents the terminal symbol `record`"]
    Record => "record",

    #[doc="Represents the terminal symbol `ArrayBuffer`"]
    ArrayBuffer => "ArrayBuffer",

    #[doc="Represents the terminal symbol `DataView`"]
    DataView => "DataView",

    #[doc="Represents the terminal symbol `Int8Array`"]
    Int8Array => "Int8Array",

    #[doc="Represents the terminal symbol `Int16Array`"]
    Int16Array => "Int16Array",

    #[doc="Represents the terminal symbol `Int32Array`"]
    Int32Array => "Int32Array",

    #[doc="Represents the terminal symbol `Uint8Array`"]
    Uint8Array => "Uint8Array",

    #[doc="Represents the terminal symbol `Uint16Array`"]
    Uint16Array => "Uint16Array",

    #[doc="Represents the terminal symbol `Uint32Array`"]
    Uint32Array => "Uint32Array",

    #[doc="Represents the terminal symbol `Uint8ClampedArray`"]
    Uint8ClampedArray => "Uint8ClampedArray",

    #[doc="Represents the terminal symbol `Float32Array`"]
    Float32Array => "Float32Array",

    #[doc="Represents the terminal symbol `Float64Array`"]
    Float64Array => "Float64Array",

    #[doc="Represents the terminal symbol `Promise`"]
    Promise => "Promise",

    #[doc="Represents the terminal symbol `Error`"]
    Error => "Error",

    #[doc="Represents the terminal symbol `readonly`"]
    ReadOnly => "readonly",

    #[doc="Represents the terminal symbol `mixin`"]
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
        frozenarray, FrozenArray, "FrozenArray";
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
        octet, Octet, "octet";
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
