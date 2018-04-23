macro_rules! generate_tokens {
    ($( $typ:ident => $tok:expr ),*) => {
        $(
            #[derive(Debug, Default)]
            pub struct $typ;

            impl $crate::Parse for $typ {
                named!(parse -> Self, do_parse!(
                    tag!($tok) >>
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