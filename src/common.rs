use Parse;
use term;

impl<T: Parse> Parse for Option<T> {
    named!(parse -> Self, do_parse!(
        parsed: opt!(weedle!(T)) >>
        (parsed)
    ));
}

impl<T: Parse> Parse for Box<T> {
    named!(parse -> Self, do_parse!(
        inner: weedle!(T) >>
        (Box::new(inner))
    ));
}

#[derive(Debug, PartialEq)]
pub struct Parenthesized<T> {
    pub open_paren: term::OpenParen,
    pub body: T,
    pub close_paren: term::CloseParen,
}

impl<T: Parse> Parse for Parenthesized<T> {
    named!(parse -> Self, do_parse!(
        open_paren: weedle!(term::OpenParen) >>
        body: weedle!(T) >>
        close_paren: weedle!(term::CloseParen) >>
        (Parenthesized {  open_paren, body, close_paren })
    ));
}

#[derive(Debug, PartialEq)]
pub struct Bracketed<T> {
    pub open_bracket: term::OpenBracket,
    pub body: T,
    pub close_bracket: term::CloseBracket,
}

impl<T: Parse> Parse for Bracketed<T> {
    named!(parse -> Self, do_parse!(
        open_bracket: weedle!(term::OpenBracket) >>
        body: weedle!(T) >>
        close_bracket: weedle!(term::CloseBracket) >>
        (Bracketed { open_bracket, body, close_bracket })
    ));
}

#[derive(Debug, PartialEq)]
pub struct Braced<T> {
    pub open_brace: term::OpenBrace,
    pub body: T,
    pub close_brace: term::CloseBrace,
}

impl<T: Parse> Parse for Braced<T> {
    named!(parse -> Self, do_parse!(
        open_brace: weedle!(term::OpenBrace) >>
        body: weedle!(T) >>
        close_brace: weedle!(term::CloseBrace) >>
        (Braced { open_brace, body, close_brace })
    ));
}

#[derive(Debug, PartialEq)]
pub struct Generics<T> {
    pub open_angle: term::LessThan,
    pub body: T,
    pub close_angle: term::GreaterThan
}

impl<T: Parse> Parse for Generics<T> {
    named!(parse -> Self, do_parse!(
        open_angle: weedle!(term::LessThan) >>
        body: weedle!(T) >>
        close_angle: weedle!(term::GreaterThan) >>
        (Generics { open_angle, body, close_angle })
    ));
}

#[derive(Debug, PartialEq)]
pub struct Punctuated<T, S> {
    pub list: Vec<T>,
    pub separator: S,
}

impl<T: Parse, S: Parse + ::std::default::Default> Parse for Punctuated<T, S> {
    named!(parse -> Self, do_parse!(
        list: separated_list!(weedle!(S), weedle!(T)) >>
        (Punctuated { list, separator: S::default() })
    ));
}