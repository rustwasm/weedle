use Parse;
use nom::types::CompleteStr;

pub fn select_first(input: Vec<CompleteStr>) -> CompleteStr {
    input[0]
}

fn parse_dec(input: CompleteStr) -> i64 {
    i64::from_str_radix(&input, 10).unwrap()
}

fn parse_hex(input: CompleteStr) -> i64 {
    i64::from_str_radix(&input[2..], 16).unwrap()
}

fn parse_oct(input: CompleteStr) -> i64 {
    i64::from_str_radix(&input, 8).unwrap()
}

/// Represents other literal symbols
///
/// Follows `/[^\t\n\r 0-9A-Za-z]/`
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct OtherLit(String);

impl Parse for OtherLit {
    named!(parse -> Self, do_parse!(
        value: ws!(re_capture_static!(r"^([^\t\n\r 0-9A-Za-z])")) >>
        (OtherLit(value[0].to_string()))
    ));
}

// Parses `-?[1-9][0-9]*`
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct DecI64(String);

impl Parse for DecI64 {
    named!(parse -> Self, do_parse!(
        num: ws!(re_capture_static!(r"^(-?[1-9][0-9]*)")) >>
        (DecI64(num[0].to_string()))
    ));
}

// Parses `-?0[Xx][0-9A-Fa-f]+)`
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct HexI64(String);

impl Parse for HexI64 {
    named!(parse -> Self, do_parse!(
        num: ws!(re_capture_static!(r"^(-?0[Xx][0-9A-Fa-f]+)")) >>
        (HexI64(num[0].to_string()))
    ));
}

// Parses `-?0[0-7]*`
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct OctI64(String);

impl Parse for OctI64 {
    named!(parse -> Self, do_parse!(
        num: ws!(re_capture_static!(r"^(-?0[0-7]*)")) >>
        (OctI64(num[0].to_string()))
    ));
}

/// Represents an integer value
///
/// Follows `/-?([1-9][0-9]*|0[Xx][0-9A-Fa-f]+|0[0-7]*)/`
#[derive(Debug, Eq, PartialEq, Clone)]
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

/// Represents a string value
///
/// Follow `/"[^"]*"/`
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct StringLit(String);

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

/// Represents a default literal value. Ex: `34|34.23|"value"|[ ]|true|false|null`
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum DefaultValue {
    Const(ConstValue),
    String(StringLit),
    EmptyArray(EmptyArray),
}

impl Parse for DefaultValue {
    named!(parse -> Self, alt_complete!(
        weedle!(ConstValue) => {|inner| DefaultValue::Const(inner)} |
        weedle!(StringLit) => {|inner| DefaultValue::String(inner)} |
        weedle!(EmptyArray) => {|inner| DefaultValue::EmptyArray(inner)}
    ));
}

/// Represents `[ ]`
pub type EmptyArray = [(); 0];

impl Parse for EmptyArray {
    named!(parse -> Self, do_parse!(
        weedle!(term!(OpenBracket)) >>
        weedle!(term!(CloseBracket)) >>
        ([])
    ));
}

/// Represents `true`, `false`, `34.23`, `null`, `56`, ...
#[derive(Debug, Eq, PartialEq, Clone)]
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
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct BooleanLit(bool);

impl Parse for BooleanLit {
    named!(parse -> Self, alt_complete!(
        weedle!(term!(true)) => {|_| BooleanLit(true)} |
        weedle!(term!(false)) => {|_| BooleanLit(false)}
    ));
}

/// Represents a floating point value, `NaN`, `Infinity`, '+Infinity`
///
/// Follows `/-?(([0-9]+\.[0-9]*|[0-9]*\.[0-9]+)([Ee][+-]?[0-9]+)?|[0-9]+[Ee][+-]?[0-9]+)/`
#[derive(Debug, Eq, PartialEq, Clone)]
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

#[cfg(test)]
mod test {
    use Parse;
    use super::*;
    use term::*;

    test!(should_parse_other_lit { "&" =>
        "";
        OtherLit;
        value == "&";
    });

    test!(should_parse_integer { "45" =>
        "";
        i64 => 45
    });

    test!(should_parse_integer_surrounding_with_spaces { "  123123  " =>
        "";
        i64 => 123123
    });

    test!(should_parse_integer_preceeding_others { "3453 string" =>
        "string";
        i64 => 3453
    });

    test!(should_parse_neg_integer { "-435" =>
        "";
        i64 => -435
    });

    test!(should_parse_hex_number { "0X08" =>
        "";
        i64 => 8
    });

    test!(should_parse_hex_large_number { "0xA" =>
        "";
        i64 => 10
    });

    test!(should_parse_float { "45.434" =>
        "";
        f64 => 45.434
    });

    test!(should_parse_float_surrounding_with_spaces { "  2345.2345  " =>
        "";
        f64 => 2345.2345
    });

    test!(should_parse_float_preceeding_others { "3453.32334 string" =>
        "string";
        f64 => 3453.32334
    });

    test!(should_parse_neg_float { "-435.3435" =>
        "";
        f64 => -435.3435
    });

    test!(should_parse_float_exp { "5.3434e23" =>
        "";
        f64 => 5.3434e23
    });

    test!(should_parse_float_exp_with_decimal { "3e23" =>
        "";
        f64 => 3e23
    });

    test!(should_parse_neg_infinity { "-Infinity" =>
        "";
        f64 => ::std::f64::NEG_INFINITY
    });

    test!(should_parse_infinity { "Infinity" =>
        "";
        f64 => ::std::f64::INFINITY
    });

    test!(should_parse_string { r#""this is a string""# =>
        "";
        String => "this is a string"
    });

    test!(should_parse_string_surround_with_spaces { r#"  "this is a string"  "# =>
        "";
        String => "this is a string"
    });

    test!(should_parse_string_followed_by_string { r#" "this is first"  "this is second" "# =>
        r#""this is second" "#;
        String => "this is first"
    });

    test!(should_parse_null { "null" =>
        "";
        Null => Null
    });

    test!(should_parse_empty_array { "[]" =>
        "";
        EmptyArray => []
    });

    test!(should_parse_bool { "true" =>
        "";
        bool => true
    });
}
