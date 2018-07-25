use argument::ArgumentList;
use common::{Braced, Bracketed, Identifier, Punctuated};

/// Parses a list of attributes. Ex: `[ attribute1, attribute2 ]`
pub type ExtendedAttributeList = Bracketed<Punctuated<ExtendedAttribute, term!(,)>>;

/// Matches comma separated identifier list
pub type IdentifierList = Punctuated<Identifier, term!(,)>;

ast_types! {
    /// Parses on of the forms of attribute
    enum ExtendedAttribute {
        /// Parses an argument list. Ex: `Constructor((double x, double y))`
        ///
        /// (( )) means ( ) chars
        ArgList(struct ExtendedAttributeArgList {
            identifier: Identifier,
            args: Braced<ArgumentList>,
        }),
        /// Parses a named argument list. Ex: `NamedConstructor=Image((DOMString src))`
        ///
        /// (( )) means ( ) chars
        NamedArgList(struct ExtendedAttributeNamedArgList {
            lhs_identifier: Identifier,
            assign: term!(=),
            rhs_identifier: Identifier,
            args: Braced<ArgumentList>,

        }),
        /// Parses an identifier list. Ex: `Exposed=((Window,Worker))`
        ///
        /// (( )) means ( ) chars
        IdentList(struct ExtendedAttributeIdentList {
            identifier: Identifier,
            assign: term!(=),
            list: Braced<IdentifierList>,
        }),
        /// Parses an attribute with an identifier. Ex: `PutForwards=name`
        Ident(struct ExtendedAttributeIdent {
            lhs_identifier: Identifier,
            assign: term!(=),
            rhs_identifier: Identifier,
        }),
        /// Parses a plain attribute. Ex: `Replaceable`
        NoArgs(struct ExtendedAttributeNoArgs(
            Identifier,
        )),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use Parse;

    test!(should_parse_attribute_no_args { "Replaceable" =>
        "";
        ExtendedAttributeNoArgs => ExtendedAttributeNoArgs(Identifier("Replaceable".to_string()))
    });

    test!(should_parse_attribute_arg_list { "Constructor(double x, double y)" =>
        "";
        ExtendedAttributeArgList;
        identifier.0 == "Constructor";
        args.body.list.len() == 2;
    });

    test!(should_parse_attribute_ident { "PutForwards=name" =>
        "";
        ExtendedAttributeIdent;
        lhs_identifier.0 == "PutForwards";
        rhs_identifier.0 == "name";
    });

    test!(should_parse_ident_list { "Exposed=(Window,Worker)" =>
        "";
        ExtendedAttributeIdentList;
        identifier.0 == "Exposed";
        list.body.list.len() == 2;
    });

    test!(should_parse_named_arg_list { "NamedConstructor=Image(DOMString src)" =>
        "";
        ExtendedAttributeNamedArgList;
        lhs_identifier.0 == "NamedConstructor";
        rhs_identifier.0 == "Image";
        args.body.list.len() == 1;
    });
}
