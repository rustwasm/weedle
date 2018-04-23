use terminals::*;
use Parse;
use nom::types::CompleteStr;

fn select_first(input: Vec<CompleteStr>) -> CompleteStr {
    input[0]
}

// Workaround to use `CompleteStr`
macro_rules! re_capture_static (
  ($i:expr, $re:expr) => (
    {
      use $crate::nom::{Err,ErrorKind,IResult};
      use $crate::nom::Slice;

      regex!(RE, $re);
      if let Some(c) = RE.captures(&$i) {
        let v:Vec<_> = c.iter().filter(|el| el.is_some()).map(|el| el.unwrap()).map(|m| $i.slice(m.start()..m.end())).collect();
        let offset = {
          let end = v.last().unwrap();
          end.as_ptr() as usize + end.len() - $i.as_ptr() as usize
        };
        Ok(($i.slice(offset..), v))
      } else {
        let res: IResult<_,_> = Err(Err::Error(error_position!($i, ErrorKind::RegexpCapture::<u32>)));
        res
      }
    }
  )
);

/// **identifier** = /_?[A-Za-z][0-9A-Z_a-z-]*/
#[derive(Debug, Eq, PartialEq)]
pub struct Identifier {
    pub name: String
}

impl Parse for Identifier {
    named!(parse -> Self, do_parse!(
        name: ws!(re_capture_static!(r"^(_?[A-Za-z][0-9A-Z_a-z-]*)")) >>
        (Identifier { name: name[0].to_string() })
    ));
}

/// **other** = /[^\t\n\r 0-9A-Za-z]/
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

/// **integer** = /-?([1-9][0-9]*|0[Xx][0-9A-Fa-f]+|0[0-7]*)/
impl Parse for i64 {
    named!(parse -> Self, do_parse!(
        value: flat_map!(
            map!(
                ws!(re_capture_static!(r"^(-?([1-9][0-9]*|0[Xx][0-9A-Fa-f]+|0[0-7]*))")),
                select_first
            ),
            parse_to!(i64)
        ) >>
        (value)
    ));
}

/// **float** = /-?(([0-9]+\.[0-9]*|[0-9]*\.[0-9]+)([Ee][+-]?[0-9]+)?|[0-9]+[Ee][+-]?[0-9]+)/
impl Parse for f64 {
    named!(parse -> Self, do_parse!(
        value: flat_map!(
            map!(
                ws!(re_capture_static!(r"^(-?(([0-9]+\.[0-9]*|[0-9]*\.[0-9]+)([Ee][+-]?[0-9]+)?|[0-9]+[Ee][+-]?[0-9]+))")),
                select_first
            ),
            parse_to!(f64)
        ) >>
        (value)
    ));
}

/// **string** = /"[^"]*"/
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

/// DefaultValue ::
///     ConstValue
///     **string**
///     **[ ]**
#[derive(Debug, PartialEq)]
pub enum DefaultValue {
    Const(ConstValue),
    String(String),
    EmptyArray(EmptyArrayLit),
}

impl Parse for DefaultValue {
    named!(parse -> Self, alt_complete!(
        weedle!(ConstValue) => {|inner| DefaultValue::Const(inner)} |
        weedle!(String) => {|inner| DefaultValue::String(inner)} |
        weedle!(EmptyArrayLit) => {|inner| DefaultValue::EmptyArray(inner)}
    ));
}

#[derive(Debug, PartialEq)]
pub struct EmptyArrayLit {
    pub open_bracket: OpenBracket,
    pub close_bracket: CloseBracket,
}

impl Parse for EmptyArrayLit {
    named!(parse -> Self, do_parse!(
        open_bracket: weedle!(OpenBracket) >>
        close_bracket: weedle!(CloseBracket) >>
        (EmptyArrayLit { open_bracket, close_bracket })
    ));
}

/// ConstValue ::
///     BooleanLiteral
///     FloatLiteral
///     **integer**
///     null
#[derive(Debug, PartialEq)]
pub enum ConstValue {
    BooleanLiteral(BooleanLiteral),
    FloatLiteral(FloatLiteral),
    Integer(i64),
    Null(Null),
}

impl Parse for ConstValue {
    named!(parse -> Self, alt_complete!(
        weedle!(BooleanLiteral) => {|inner| ConstValue::BooleanLiteral(inner)} |
        weedle!(FloatLiteral) => {|inner| ConstValue::FloatLiteral(inner)} |
        weedle!(i64) => {|inner| ConstValue::Integer(inner)} |
        weedle!(Null) => {|inner| ConstValue::Null(inner)}
    ));
}

/// BooleanLiteral ::
///     true
///     false
#[derive(Debug, PartialEq)]
pub enum BooleanLiteral {
    True(True),
    False(False),
}

impl Parse for BooleanLiteral {
    named!(parse -> Self, alt_complete!(
        weedle!(True) => {|inner| BooleanLiteral::True(inner)} |
        weedle!(False) => {|inner| BooleanLiteral::False(inner)}
    ));
}

/// FloatLiteral ::
///     **float**
///     -Infinity
///     Infinity
///     NaN
#[derive(Debug, PartialEq)]
pub enum FloatLiteral {
    Float(f64),
    NegInfinity(NegInfinity),
    Infinity(Infinity),
    NaN(NaN),
}

impl Parse for FloatLiteral {
    named!(parse -> Self, alt_complete!(
        weedle!(f64) => {|inner| FloatLiteral::Float(inner)} |
        weedle!(NegInfinity) => {|inner| FloatLiteral::NegInfinity(inner)} |
        weedle!(Infinity) => {|inner| FloatLiteral::Infinity(inner)} |
        weedle!(NaN) => {|inner| FloatLiteral::NaN(inner)}
    ));
}

#[cfg(test)]
mod test {
    use Parse;
    use super::{Identifier, OtherLit};
    use nom::types::CompleteStr;

    macro_rules! test_literal {
        ($name:ident { $raw:expr => $rem:expr; $typ:ident => $val:expr }) => {
            #[test]
            fn $name() {
                let (rem, parsed) = $typ::parse(CompleteStr($raw)).unwrap();
                assert_eq!(rem, CompleteStr($rem));
                assert_eq!(parsed, $val);
            }
        };
        ($name:ident { $raw:expr => $rem:expr; $typ:ident { $($field:ident => $val:expr),* } }) => {
            #[test]
            fn $name() {
                let (rem, parsed) = $typ::parse(CompleteStr($raw)).unwrap();
                assert_eq!(rem, CompleteStr($rem));
                assert_eq!(parsed, $typ { $($field: $val),* });
            }
        };
    }

    test_literal!(should_parse_identifier { "hello" =>
        "";
        Identifier {
            name => "hello".to_string()
        }
    });

    test_literal!(should_parse_numbered_identifier { "hello5" =>
        "";
        Identifier {
            name => "hello5".to_string()
        }
    });

    test_literal!(should_parse_underscored_identifier { "_hello_" =>
        "";
        Identifier {
            name => "_hello_".to_string()
        }
    });

    test_literal!(should_parse_identifier_surrounding_with_spaces { "  hello  " =>
        "";
        Identifier {
            name => "hello".to_string()
        }
    });

    test_literal!(should_parse_identifier_preceeding_others { "hello  note" =>
        "note";
        Identifier {
            name => "hello".to_string()
        }
    });

    test_literal!(should_parse_identifier_attached_to_symbol { "hello=" =>
        "=";
        Identifier {
            name => "hello".to_string()
        }
    });

    test_literal!(should_parse_other_lit { "&" =>
        "";
        OtherLit {
            value => "&".to_string()
        }
    });

    test_literal!(should_parse_integer { "45" =>
        "";
        i64 => 45
    });

    test_literal!(should_parse_integer_surrounding_with_spaces { "  123123  " =>
        "";
        i64 => 123123
    });

    test_literal!(should_parse_integer_preceeding_others { "3453 string" =>
        "string";
        i64 => 3453
    });

    test_literal!(should_parse_neg_integer { "-435" =>
        "";
        i64 => -435
    });

    test_literal!(should_parse_float { "45.434" =>
        "";
        f64 => 45.434
    });

    test_literal!(should_parse_float_surrounding_with_spaces { "  2345.2345  " =>
        "";
        f64 => 2345.2345
    });

    test_literal!(should_parse_float_preceeding_others { "3453.32334 string" =>
        "string";
        f64 => 3453.32334
    });

    test_literal!(should_parse_neg_float { "-435.3435" =>
        "";
        f64 => -435.3435
    });

    test_literal!(should_parse_float_exp { "5.3434e23" =>
        "";
        f64 => 5.3434e23
    });

    test_literal!(should_parse_float_exp_with_decimal { "3e23" =>
        "";
        f64 => 3e23
    });

    test_literal!(should_parse_string { r#""this is a string""# =>
        "";
        String => "this is a string"
    });

    test_literal!(should_parse_string_surround_with_spaces { r#"  "this is a string"  "# =>
        "";
        String => "this is a string"
    });

    test_literal!(should_parse_string_followed_by_string { r#" "this is first"  "this is second" "# =>
        r#""this is second" "#;
        String => "this is first"
    });
}
