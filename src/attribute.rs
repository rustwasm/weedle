use crate::argument::ArgumentList;
use crate::common::{Bracketed, Identifier, Parenthesized, Punctuated};
use crate::literal::StringLit;

/// Parses a list of attributes. Ex: `[ attribute1, attribute2 ]`
pub type ExtendedAttributeList<'a> = Bracketed<Punctuated<ExtendedAttribute<'a>, term!(,)>>;

/// Matches comma separated identifier list
pub type IdentifierList<'a> = Punctuated<Identifier<'a>, term!(,)>;

ast_types! {
    /// Parses on of the forms of attribute
    enum ExtendedAttribute<'a> {
        /// Parses an argument list. Ex: `Constructor((double x, double y))`
        ///
        /// (( )) means ( ) chars
        ArgList(struct ExtendedAttributeArgList<'a> {
            identifier: Identifier<'a>,
            args: Parenthesized<ArgumentList<'a>>,
        }),
        /// Parses a named argument list. Ex: `NamedConstructor=Image((DOMString src))`
        ///
        /// (( )) means ( ) chars
        NamedArgList(struct ExtendedAttributeNamedArgList<'a> {
            lhs_identifier: Identifier<'a>,
            assign: term!(=),
            rhs_identifier: Identifier<'a>,
            args: Parenthesized<ArgumentList<'a>>,

        }),
        /// Parses an identifier list. Ex: `Exposed=((Window,Worker))`
        ///
        /// (( )) means ( ) chars
        IdentList(struct ExtendedAttributeIdentList<'a> {
            identifier: Identifier<'a>,
            assign: term!(=),
            list: Parenthesized<IdentifierList<'a>>,
        }),
        /// Parses an attribute with an identifier. Ex: `PutForwards=name`
        Ident(struct ExtendedAttributeIdent<'a> {
            lhs_identifier: Identifier<'a>,
            assign: term!(=),
            rhs: IdentifierOrString<'a>,
        }),
        /// Parses a plain attribute. Ex: `Replaceable`
        NoArgs(struct ExtendedAttributeNoArgs<'a>(
            Identifier<'a>,
        )),
    }

    /// Parses `stringifier|static`
    enum IdentifierOrString<'a> {
        Identifier(Identifier<'a>),
        String(StringLit<'a>),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Parse;

    test!(should_parse_attribute_no_args { "Replaceable" =>
        "";
        ExtendedAttributeNoArgs => ExtendedAttributeNoArgs(Identifier { specifier: vec![], identifier: "Replaceable" })
    });

    test!(should_parse_attribute_arg_list { "Constructor(double x, double y)" =>
        "";
        ExtendedAttributeArgList;
        identifier.identifier == "Constructor";
        args.body.list.len() == 2;
    });

    test!(should_parse_attribute_ident { "PutForwards=name" =>
        "";
        ExtendedAttributeIdent;
        lhs_identifier.identifier == "PutForwards";
        rhs == IdentifierOrString::Identifier(Identifier { specifier: vec![], identifier: "name" });
    });

    test!(should_parse_ident_list { "Exposed=(Window,Worker)" =>
        "";
        ExtendedAttributeIdentList;
        identifier.identifier == "Exposed";
        list.body.list.len() == 2;
    });

    test!(should_parse_named_arg_list { "NamedConstructor=Image(DOMString src)" =>
        "";
        ExtendedAttributeNamedArgList;
        lhs_identifier.identifier == "NamedConstructor";
        rhs_identifier.identifier == "Image";
        args.body.list.len() == 1;
    });
}
