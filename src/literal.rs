ast_types! {
    /// Represents an integer value
    enum IntegerLit {
        /// Parses `-?[1-9][0-9]*`
        Dec(struct DecLit(
            String = map!(
                ws!(re_find_static!(r"^-?[1-9][0-9]*")),
                |inner| inner.to_string()
            ),
        )),
        /// Parses `-?0[Xx][0-9A-Fa-f]+)`
        Hex(struct HexLit(
            String = map!(
                ws!(re_find_static!(r"^-?0[Xx][0-9A-Fa-f]+")),
                |inner| inner.to_string()
            ),
        )),
        /// Parses `-?0[0-7]*`
        Oct(struct OctLit(
            String = map!(
                ws!(re_find_static!(r"^-?0[0-7]*")),
                |inner| inner.to_string()
            ),
        )),
    }

    /// Represents a string value
    ///
    /// Follow `/"[^"]*"/`
    struct StringLit(
        String = map!(
            ws!(re_find_static!(r#"^"[^"]*""#)),
            |quoted| quoted[1..quoted.len()-1].to_string()
        ),
    )

    /// Represents a default literal value. Ex: `34|34.23|"value"|[ ]|true|false|null`
    enum DefaultValue {
        Boolean(BooleanLit),
        /// Represents `[ ]`
        #[derive(Copy, Default)]
        EmptyArray(struct EmptyArrayLit {
            open_bracket: term!(OpenBracket),
            close_bracket: term!(CloseBracket),
        }),
        Float(FloatLit),
        Integer(IntegerLit),
        Null(term!(null)),
        String(StringLit),
    }

    /// Represents `true`, `false`, `34.23`, `null`, `56`, ...
    enum ConstValue {
        Boolean(BooleanLit),
        Float(FloatLit),
        Integer(IntegerLit),
        Null(term!(null)),
    }

    /// Represents either `true` or `false`
    #[derive(Copy)]
    struct BooleanLit(
        bool = alt!(
            weedle!(term!(true)) => {|_| true} |
            weedle!(term!(false)) => {|_| false}
        ),
    )

    /// Represents a floating point value, `NaN`, `Infinity`, '+Infinity`
    enum FloatLit {
        /// Parses `/-?(([0-9]+\.[0-9]*|[0-9]*\.[0-9]+)([Ee][+-]?[0-9]+)?|[0-9]+[Ee][+-]?[0-9]+)/`
        Value(struct FloatValueLit(
            String = map!(
                ws!(re_find_static!(r"^-?(?:(?:[0-9]+\.[0-9]*|[0-9]*\.[0-9]+)(?:[Ee][+-]?[0-9]+)?|[0-9]+[Ee][+-]?[0-9]+)")),
                |inner| inner.to_string()
            ),
        )),
        NegInfinity(term!(-Infinity)),
        Infinity(term!(Infinity)),
        NaN(term!(NaN)),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use term::*;
    use Parse;

    test!(should_parse_integer { "45" =>
        "";
        IntegerLit => IntegerLit::Dec(DecLit("45".to_string()))
    });

    test!(should_parse_integer_surrounding_with_spaces { "  123123  " =>
        "";
        IntegerLit => IntegerLit::Dec(DecLit("123123".to_string()))
    });

    test!(should_parse_integer_preceeding_others { "3453 string" =>
        "string";
        IntegerLit => IntegerLit::Dec(DecLit("3453".to_string()))
    });

    test!(should_parse_neg_integer { "-435" =>
        "";
        IntegerLit => IntegerLit::Dec(DecLit("-435".to_string()))
    });

    test!(should_parse_hex_number { "0X08" =>
        "";
        IntegerLit => IntegerLit::Hex(HexLit("0X08".to_string()))
    });

    test!(should_parse_hex_large_number { "0xA" =>
        "";
        IntegerLit => IntegerLit::Hex(HexLit("0xA".to_string()))
    });

    test!(should_parse_zero { "0" =>
        "";
        IntegerLit => IntegerLit::Oct(OctLit("0".to_string()))
    });

    test!(should_parse_oct_number { "-07561" =>
        "";
        IntegerLit => IntegerLit::Oct(OctLit("-07561".to_string()))
    });

    test!(should_parse_float { "45.434" =>
        "";
        FloatLit => FloatLit::Value(FloatValueLit("45.434".to_string()))
    });

    test!(should_parse_float_surrounding_with_spaces { "  2345.2345  " =>
        "";
        FloatLit => FloatLit::Value(FloatValueLit("2345.2345".to_string()))
    });

    test!(should_parse_float_preceeding_others { "3453.32334 string" =>
        "string";
        FloatLit => FloatLit::Value(FloatValueLit("3453.32334".to_string()))
    });

    test!(should_parse_neg_float { "-435.3435" =>
        "";
        FloatLit => FloatLit::Value(FloatValueLit("-435.3435".to_string()))
    });

    test!(should_parse_float_exp { "5.3434e23" =>
        "";
        FloatLit => FloatLit::Value(FloatValueLit("5.3434e23".to_string()))
    });

    test!(should_parse_float_exp_with_decimal { "3e23" =>
        "";
        FloatLit => FloatLit::Value(FloatValueLit("3e23".to_string()))
    });

    test!(should_parse_neg_infinity { "-Infinity" =>
        "";
        FloatLit => FloatLit::NegInfinity(term!(-Infinity))
    });

    test!(should_parse_infinity { "Infinity" =>
        "";
        FloatLit => FloatLit::Infinity(term!(Infinity))
    });

    test!(should_parse_string { r#""this is a string""# =>
        "";
        StringLit => StringLit("this is a string".to_string())
    });

    test!(should_parse_string_surround_with_spaces { r#"  "this is a string"  "# =>
        "";
        StringLit => StringLit("this is a string".to_string())
    });

    test!(should_parse_string_followed_by_string { r#" "this is first"  "this is second" "# =>
        r#""this is second" "#;
        StringLit => StringLit("this is first".to_string())
    });

    test!(should_parse_null { "null" =>
        "";
        Null => Null
    });

    test!(should_parse_empty_array { "[]" =>
        "";
        EmptyArrayLit => Default::default()
    });

    test!(should_parse_bool_true { "true" =>
        "";
        BooleanLit => BooleanLit(true)
    });

    test!(should_parse_bool_false { "false" =>
        "";
        BooleanLit => BooleanLit(false)
    });
}
