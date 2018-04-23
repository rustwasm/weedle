use Parse;

/// **identifier** = /_?[A-Za-z][0-9A-Z_a-z-]*/
#[derive(Debug)]
pub struct Identifier {
    pub name: String
}

impl Parse for Identifier {
    named!(parse -> Self, do_parse!(
        name: ws!(re_match!(r"_?[A-Za-z][0-9A-Z_a-z-]*")) >>
        (Identifier { name: name.to_owned() })
    ));
}

/// **other** = /[^\t\n\r 0-9A-Za-z]/
#[derive(Debug)]
pub struct OtherLit {
    pub value: String
}

impl Parse for OtherLit {
    named!(parse -> Self, do_parse!(
        value: ws!(re_match!(r"[^\t\n\r 0-9A-Za-z]")) >>
        (OtherLit { value: value.to_owned() })
    ));
}

/// **integer** = /-?([1-9][0-9]*|0[Xx][0-9A-Fa-f]+|0[0-7]*)/
impl Parse for i64 {
    named!(parse -> Self, do_parse!(
        value: ws!(parse_to!(re_match!(r"-?([1-9][0-9]*|0[Xx][0-9A-Fa-f]+|0[0-7]*)"))) >>
        (value)
    ));
}

/// **float** = /-?(([0-9]+\.[0-9]*|[0-9]*\.[0-9]+)([Ee][+-]?[0-9]+)?|[0-9]+[Ee][+-]?[0-9]+)/
impl Parse for f64 {
    named!(parse -> Self, do_parse!(
        value: ws!(parse_to!(re_match!(r"-?(([0-9]+\.[0-9]*|[0-9]*\.[0-9]+)([Ee][+-]?[0-9]+)?|[0-9]+[Ee][+-]?[0-9]+)"))) >>
        (value)
    ));
}

/// **string** = /"[^"]*"/
impl Parse for String {
    named!(parse -> Self, do_parse!(
        value: ws!(re_match!(r#""[^"]*""#)) >>
        (value.to_owned())
    ));
}