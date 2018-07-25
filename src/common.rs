use literal::DefaultValue;
use term;
use Parse;

impl<T: Parse> Parse for Option<T> {
    named!(parse -> Self, opt!(weedle!(T)));
}

impl<T: Parse> Parse for Box<T> {
    named!(parse -> Self, do_parse!(
        inner: weedle!(T) >>
        (Box::new(inner))
    ));
}

/// Parses `item1 item2 item3...`
impl<T: Parse> Parse for Vec<T> {
    named!(parse -> Self, many0!(weedle!(T)));
}

impl<T: Parse, U: Parse> Parse for (T, U) {
    named!(parse-> Self, do_parse!(
        f: weedle!(T) >>
        s: weedle!(U) >>
        ((f, s))
    ));
}

impl<T: Parse, U: Parse, V: Parse> Parse for (T, U, V) {
    named!(parse-> Self, do_parse!(
        f: weedle!(T) >>
        s: weedle!(U) >>
        t: weedle!(V) >>
        ((f, s, t))
    ));
}

ast_types! {
    /// Parses `{ body }`
    struct Parenthesized(T) where [T: Parse] {
        open_paren: term::OpenParen,
        body: T,
        close_paren: term::CloseParen,
    }

    /// Parses `[ body ]`
    struct Bracketed(T) where [T: Parse] {
        open_bracket: term::OpenBracket,
        body: T,
        close_bracket: term::CloseBracket,
    }

    /// Parses `( body )`
    struct Braced(T) where [T: Parse] {
        open_brace: term::OpenBrace,
        body: T,
        close_brace: term::CloseBrace,
    }

    /// Parses `< body >`
    struct Generics(T) where [T: Parse] {
        open_angle: term::LessThan,
        body: T,
        close_angle: term::GreaterThan,
    }

    /// Parses `(item1, item2, item3,...)?`
    struct Punctuated(T, S) where [T: Parse, S: Parse + ::std::default::Default] {
        list: Vec<T> = separated_list!(weedle!(S), weedle!(T)),
        separator: S = marker,
    }

    /// Parses `item1, item2, item3, ...`
    struct PunctuatedNonEmpty(T, S) where [T: Parse, S: Parse + ::std::default::Default] {
        list: Vec<T> = separated_nonempty_list!(weedle!(S), weedle!(T)),
        separator: S = marker,
    }

    /// Represents an identifier
    ///
    /// Follows `/_?[A-Za-z][0-9A-Z_a-z-]*/`
    struct Identifier(
        String = map!(
            ws!(re_find_static!(r"^_?[A-Za-z][0-9A-Z_a-z-]*")),
            |inner| inner.to_string()
        ),
    )

    /// Parses rhs of an assignment expression. Ex: `= 45`
    struct Default {
        assign: term!(=),
        value: DefaultValue,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    test!(should_parse_optional_present { "one" =>
        "";
        Option<Identifier>;
        is_some();
    });

    test!(should_parse_optional_not_present { "" =>
        "";
        Option<Identifier>;
        is_none();
    });

    test!(should_parse_boxed { "one" =>
        "";
        Box<Identifier>;
    });

    test!(should_parse_vec { "one two three" =>
        "";
        Vec<Identifier>;
        len() == 3;
    });

    test!(should_parse_parenthesized { "{ one }" =>
        "";
        Parenthesized<Identifier>;
        body.0 == "one";
    });

    test!(should_parse_bracketed { "[ one ]" =>
        "";
        Bracketed<Identifier>;
        body.0 == "one";
    });

    test!(should_parse_braced { "( one )" =>
        "";
        Braced<Identifier>;
        body.0 == "one";
    });

    test!(should_parse_generics { "<one>" =>
        "";
        Generics<Identifier>;
        body.0 == "one";
    });

    test!(should_parse_generics_two { "<one, two>" =>
        "";
        Generics<(Identifier, term!(,), Identifier)> =>
            Generics {
                open_angle: term!(<),
                body: (Identifier("one".to_string()), term!(,), Identifier("two".to_string())),
                close_angle: term!(>),
            }
    });

    test!(should_parse_comma_separated_values { "one, two, three" =>
        "";
        Punctuated<Identifier, term!(,)>;
        list.len() == 3;
    });

    test!(err should_not_parse_comma_separated_values_empty { "" =>
        PunctuatedNonEmpty<Identifier, term!(,)>
    });

    test!(should_parse_identifier { "hello" =>
        "";
        Identifier;
        0 == "hello";
    });

    test!(should_parse_numbered_identifier { "hello5" =>
        "";
        Identifier;
        0 == "hello5";
    });

    test!(should_parse_underscored_identifier { "_hello_" =>
        "";
        Identifier;
        0 == "_hello_";
    });

    test!(should_parse_identifier_surrounding_with_spaces { "  hello  " =>
        "";
        Identifier;
        0 == "hello";
    });

    test!(should_parse_identifier_preceeding_others { "hello  note" =>
        "note";
        Identifier;
        0 == "hello";
    });

    test!(should_parse_identifier_attached_to_symbol { "hello=" =>
        "=";
        Identifier;
        0 == "hello";
    });
}
