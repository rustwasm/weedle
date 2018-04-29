use Parse;
use nom::types::CompleteStr;

fn select_first(input: Vec<CompleteStr>) -> CompleteStr {
    input[0]
}

/// Represents other literal symbols
///
/// Follows `/[^\t\n\r 0-9A-Za-z]/`
#[derive(Debug, Eq, PartialEq)]
pub struct OtherLit {
    pub value: String
}

impl Parse for OtherLit {
    named!(parse -> Self, do_parse!(
        value: ws!(re_capture_static!(r"^([^\t\n\r 0-9A-Za-z])")) >>
        (OtherLit { value: value[0].to_string() })
    ));
}

/// Represents an integer value
///
/// Follows `/-?([1-9][0-9]*|0[Xx][0-9A-Fa-f]+|0[0-7]*)/`
impl Parse for i64 {
    named!(parse -> Self, flat_map!(
        map!(
            ws!(re_capture_static!(r"^(-?([1-9][0-9]*|0[Xx][0-9A-Fa-f]+|0[0-7]*))")),
            select_first
        ),
        parse_to!(i64)
    ));
}

/// Represents a string value
///
/// Follow `/"[^"]*"/`
impl Parse for String {
    named!(parse -> Self, do_parse!(
        value: ws!(re_capture_static!(r#"^("[^"]*")"#)) >>
        ({
            let quoted = value[0];
            let unquoted = &quoted[1..quoted.len() - 1];
            unquoted.to_string()
        })
    ));
}

/// Represents a default literal value. Ex: `34|34.23|"value"|[ ]|true|false|null`
#[derive(Debug, PartialEq)]
pub enum DefaultValue {
    Const(ConstValue),
    String(String),
    EmptyArray(EmptyArray),
}

impl Parse for DefaultValue {
    named!(parse -> Self, alt_complete!(
        weedle!(ConstValue) => {|inner| DefaultValue::Const(inner)} |
        weedle!(String) => {|inner| DefaultValue::String(inner)} |
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
#[derive(Debug, PartialEq)]
pub enum ConstValue {
    BooleanLiteral(bool),
    FloatLiteral(f64),
    Integer(i64),
    Null(term!(null)),
}

impl Parse for ConstValue {
    named!(parse -> Self, alt_complete!(
        weedle!(bool) => {|inner| ConstValue::BooleanLiteral(inner)} |
        weedle!(f64) => {|inner| ConstValue::FloatLiteral(inner)} |
        weedle!(i64) => {|inner| ConstValue::Integer(inner)} |
        weedle!(term!(null)) => {|inner| ConstValue::Null(inner)}
    ));
}

/// Represents either `true` or `false`
impl Parse for bool {
    named!(parse -> Self, alt_complete!(
        weedle!(term!(true)) => {|_| true} |
        weedle!(term!(false)) => {|_| false}
    ));
}

/// Represents a floating point value, `NaN`, `Infinity`, '+Infinity`
///
/// Follows `/-?(([0-9]+\.[0-9]*|[0-9]*\.[0-9]+)([Ee][+-]?[0-9]+)?|[0-9]+[Ee][+-]?[0-9]+)/`
impl Parse for f64 {
    named!(parse -> Self, alt_complete!(
        flat_map!(map!(ws!(
            re_capture_static!(r"^(-?(([0-9]+\.[0-9]*|[0-9]*\.[0-9]+)([Ee][+-]?[0-9]+)?|[0-9]+[Ee][+-]?[0-9]+))")),
            select_first
        ),
        parse_to!(f64)) => {|inner| inner} |
        weedle!(term!(-Infinity)) => {|_| ::std::f64::NEG_INFINITY} |
        weedle!(term!(Infinity)) => {|_| ::std::f64::INFINITY} |
        weedle!(term!(NaN)) => {|_| ::std::f64::NAN}
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
