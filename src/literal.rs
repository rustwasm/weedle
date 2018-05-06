use Parse;
use nom::types::CompleteStr;
use std::str::FromStr;

/// Represents other literal symbols
///
/// Follows `/[^\t\n\r 0-9A-Za-z]/`
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone)]
pub struct OtherLit(pub String);

impl Parse for OtherLit {
    named!(parse -> Self, do_parse!(
        value: ws!(re_capture_static!(r"^([^\t\n\r 0-9A-Za-z])")) >>
        (OtherLit(value[0].to_string()))
    ));
}

impl OtherLit {
    pub fn value(&self) -> &str {
        &self.0
    }
}

/// Parses `-?[1-9][0-9]*`
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone)]
pub struct DecI64(pub String);

impl Parse for DecI64 {
    named!(parse -> Self, do_parse!(
        num: ws!(re_capture_static!(r"^(-?[1-9][0-9]*)")) >>
        (DecI64(num[0].to_string()))
    ));
}

impl DecI64 {
    pub fn value(&self) -> i64 {
        i64::from_str_radix(&self.0, 10).unwrap()
    }
}

/// Parses `-?0[Xx][0-9A-Fa-f]+)`
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone)]
pub struct HexI64(pub String);

impl Parse for HexI64 {
    named!(parse -> Self, do_parse!(
        num: ws!(re_capture_static!(r"^(-?0[Xx][0-9A-Fa-f]+)")) >>
        (HexI64(num[0].to_string()))
    ));
}

impl HexI64 {
    pub fn value(&self) -> i64 {
        i64::from_str_radix(&self.0[2..], 16).unwrap()
    }
}

/// Parses `-?0[0-7]*`
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone)]
pub struct OctI64(pub String);

impl Parse for OctI64 {
    named!(parse -> Self, do_parse!(
        num: ws!(re_capture_static!(r"^(-?0[0-7]*)")) >>
        (OctI64(num[0].to_string()))
    ));
}

impl OctI64 {
    pub fn value(&self) -> i64 {
        i64::from_str_radix(&self.0, 8).unwrap()
    }
}

/// Represents an integer value
///
/// Follows `/-?([1-9][0-9]*|0[Xx][0-9A-Fa-f]+|0[0-7]*)/`
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone)]
pub enum IntegerLit {
    Dec(DecI64),
    Hex(HexI64),
    Oct(OctI64)
}

impl Parse for IntegerLit {
    named!(parse -> Self, alt!(
        weedle!(DecI64) => {|num| IntegerLit::Dec(num)} |
        weedle!(HexI64) => {|num| IntegerLit::Hex(num)} |
        weedle!(OctI64) => {|num| IntegerLit::Oct(num)}
    ));
}

impl IntegerLit {
    pub fn value(&self) -> i64 {
        match *self {
            IntegerLit::Dec(ref dec) => dec.value(),
            IntegerLit::Hex(ref hex) => hex.value(),
            IntegerLit::Oct(ref oct) => oct.value()
        }
    }
}

/// Represents a string value
///
/// Follow `/"[^"]*"/`
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone)]
pub struct StringLit(pub String);

impl Parse for StringLit {
    named!(parse -> Self, do_parse!(
        value: ws!(re_capture_static!(r#"^("[^"]*")"#)) >>
        ({
            let quoted = value[0];
            let unquoted = &quoted[1..quoted.len() - 1];
            StringLit(unquoted.to_string())
        })
    ));
}

impl StringLit {
    pub fn value(&self) -> &str {
        &self.0
    }
}

/// Represents a default literal value. Ex: `34|34.23|"value"|[ ]|true|false|null`
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone)]
pub enum DefaultValue {
    Const(ConstValue),
    String(StringLit),
    EmptyArray(EmptyArrayLit),
}

impl Parse for DefaultValue {
    named!(parse -> Self, alt_complete!(
        weedle!(ConstValue) => {|inner| DefaultValue::Const(inner)} |
        weedle!(StringLit) => {|inner| DefaultValue::String(inner)} |
        weedle!(EmptyArrayLit) => {|inner| DefaultValue::EmptyArray(inner)}
    ));
}

/// Represents `[ ]`
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone)]
pub struct EmptyArrayLit(pub [(); 0]);

impl Parse for EmptyArrayLit {
    named!(parse -> Self, do_parse!(
        weedle!(term!(OpenBracket)) >>
        weedle!(term!(CloseBracket)) >>
        (EmptyArrayLit([]))
    ));
}

/// Represents `true`, `false`, `34.23`, `null`, `56`, ...
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone)]
pub enum ConstValue {
    Boolean(BooleanLit),
    Float(FloatLit),
    Integer(IntegerLit),
    Null(term!(null)),
}

impl Parse for ConstValue {
    named!(parse -> Self, alt_complete!(
        weedle!(BooleanLit) => {|inner| ConstValue::Boolean(inner)} |
        weedle!(FloatLit) => {|inner| ConstValue::Float(inner)} |
        weedle!(IntegerLit) => {|inner| ConstValue::Integer(inner)} |
        weedle!(term!(null)) => {|inner| ConstValue::Null(inner)}
    ));
}

/// Represents either `true` or `false`
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone)]
pub struct BooleanLit(pub bool);

impl Parse for BooleanLit {
    named!(parse -> Self, alt_complete!(
        weedle!(term!(true)) => {|_| BooleanLit(true)} |
        weedle!(term!(false)) => {|_| BooleanLit(false)}
    ));
}

impl BooleanLit {
    pub fn value(&self) -> bool {
        self.0
    }
}

/// Represents a floating point value, `NaN`, `Infinity`, '+Infinity`
///
/// Follows `/-?(([0-9]+\.[0-9]*|[0-9]*\.[0-9]+)([Ee][+-]?[0-9]+)?|[0-9]+[Ee][+-]?[0-9]+)/`
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone)]
pub enum FloatLit {
    Value(String),
    NegInfinity(term!(-Infinity)),
    Infinity(term!(Infinity)),
    NaN(term!(NaN))
}

impl Parse for FloatLit {
    named!(parse -> Self, alt_complete!(
        ws!(re_capture_static!(r"^(-?(([0-9]+\.[0-9]*|[0-9]*\.[0-9]+)([Ee][+-]?[0-9]+)?|[0-9]+[Ee][+-]?[0-9]+))"))
            => {|inner: Vec<CompleteStr>| FloatLit::Value(inner[0].to_string())} |
        weedle!(term!(-Infinity)) => {|inner| FloatLit::NegInfinity(inner)} |
        weedle!(term!(Infinity)) => {|inner| FloatLit::Infinity(inner)} |
        weedle!(term!(NaN)) => {|inner| FloatLit::NaN(inner)}
    ));
}

impl FloatLit {
    pub fn value(&self) -> f64 {
        match *self {
            FloatLit::Value(ref value) => f64::from_str(&value).unwrap(),
            FloatLit::NegInfinity(_) => ::std::f64::NEG_INFINITY,
            FloatLit::Infinity(_) => ::std::f64::INFINITY,
            FloatLit::NaN(_) => ::std::f64::NAN
        }
    }
}

#[cfg(test)]
mod test {
    use Parse;
    use super::*;
    use term::*;

    test!(should_parse_other_lit { "&" =>
        "";
        OtherLit;
        0 == "&";
    });

    test!(should_parse_integer { "45" =>
        "";
        IntegerLit => IntegerLit::Dec(DecI64("45".to_string()))
    });

    test!(should_parse_integer_surrounding_with_spaces { "  123123  " =>
        "";
        IntegerLit => IntegerLit::Dec(DecI64("123123".to_string()))
    });

    test!(should_parse_integer_preceeding_others { "3453 string" =>
        "string";
        IntegerLit => IntegerLit::Dec(DecI64("3453".to_string()))
    });

    test!(should_parse_neg_integer { "-435" =>
        "";
        IntegerLit => IntegerLit::Dec(DecI64("-435".to_string()))
    });

    test!(should_parse_hex_number { "0X08" =>
        "";
        IntegerLit => IntegerLit::Hex(HexI64("0X08".to_string()))
    });

    test!(should_parse_hex_large_number { "0xA" =>
        "";
        IntegerLit => IntegerLit::Hex(HexI64("0xA".to_string()))
    });

    test!(should_parse_float { "45.434" =>
        "";
        FloatLit => FloatLit::Value("45.434".to_string())
    });

    test!(should_parse_float_surrounding_with_spaces { "  2345.2345  " =>
        "";
        FloatLit => FloatLit::Value("2345.2345".to_string())
    });

    test!(should_parse_float_preceeding_others { "3453.32334 string" =>
        "string";
        FloatLit => FloatLit::Value("3453.32334".to_string())
    });

    test!(should_parse_neg_float { "-435.3435" =>
        "";
        FloatLit => FloatLit::Value("-435.3435".to_string())
    });

    test!(should_parse_float_exp { "5.3434e23" =>
        "";
        FloatLit => FloatLit::Value("5.3434e23".to_string())
    });

    test!(should_parse_float_exp_with_decimal { "3e23" =>
        "";
        FloatLit => FloatLit::Value("3e23".to_string())
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
        EmptyArrayLit => EmptyArrayLit([])
    });

    test!(should_parse_bool { "true" =>
        "";
        BooleanLit => BooleanLit(true)
    });
}
