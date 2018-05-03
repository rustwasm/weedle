use Parse;
use common::*;
use argument::*;

/// Parses a list of attributes. Ex: `[ attribute1, attribute2 ]`
pub type ExtendedAttributeList = Bracketed<Punctuated<ExtendedAttribute, term!(,)>>;

/// Parses on of the forms of attribute
#[derive(Debug, PartialEq, Clone)]
pub enum ExtendedAttribute {
    ArgList(ExtendedAttributeArgList),
    NamedArgList(ExtendedAttributeNamedArgList),
    IdentList(ExtendedAttributeIdentList),
    Ident(ExtendedAttributeIdent),
    NoArgs(ExtendedAttributeNoArgs),
}

impl Parse for ExtendedAttribute {
    named!(parse -> Self, alt!(
        weedle!(ExtendedAttributeArgList) => {|inner| ExtendedAttribute::ArgList(inner)} |
        weedle!(ExtendedAttributeNamedArgList) => {|inner| ExtendedAttribute::NamedArgList(inner)} |
        weedle!(ExtendedAttributeIdentList) => {|inner| ExtendedAttribute::IdentList(inner)} |
        weedle!(ExtendedAttributeIdent) => {|inner| ExtendedAttribute::Ident(inner)} |
        weedle!(ExtendedAttributeNoArgs) => {|inner| ExtendedAttribute::NoArgs(inner)}
    ));
}

/// Parses a named argument list. Ex: `NamedConstructor=Image((DOMString src))`
///
/// (( )) means ( ) chars
#[derive(Debug, PartialEq, Clone)]
pub struct ExtendedAttributeNamedArgList {
    pub lhs_identifier: Identifier,
    pub assign: term!(=),
    pub rhs_identifier: Identifier,
    pub args: Braced<ArgumentList>,
}

impl Parse for ExtendedAttributeNamedArgList {
    named!(parse -> Self, do_parse!(
        lhs_identifier: weedle!(Identifier) >>
        assign: weedle!(term!(=)) >>
        rhs_identifier: weedle!(Identifier) >>
        args: weedle!(Braced<ArgumentList>) >>
        (ExtendedAttributeNamedArgList { lhs_identifier, assign, rhs_identifier, args })
    ));
}

/// Parses an identifier list. Ex: `Exposed=((Window,Worker))`
///
/// (( )) means ( ) chars
#[derive(Debug, PartialEq, Clone)]
pub struct ExtendedAttributeIdentList {
    pub identifier: Identifier,
    pub assign: term!(=),
    pub list: Braced<IdentifierList>
}

impl Parse for ExtendedAttributeIdentList {
    named!(parse -> Self, do_parse!(
        identifier: weedle!(Identifier) >>
        assign: weedle!(term!(=)) >>
        list: weedle!(Braced<IdentifierList>) >>
        (ExtendedAttributeIdentList { identifier, assign, list })
    ));
}

/// Matches comma separated identifier list
pub type IdentifierList = Punctuated<Identifier, term!(,)>;

/// Parses an attribute with an identifier. Ex: `PutForwards=name`
#[derive(Debug, PartialEq, Clone)]
pub struct ExtendedAttributeIdent {
    pub lhs_identifier: Identifier,
    pub assign: term!(=),
    pub rhs_identifier: Identifier
}

impl Parse for ExtendedAttributeIdent {
    named!(parse -> Self, do_parse!(
        lhs_identifier: weedle!(Identifier) >>
        assign: weedle!(term!(=)) >>
        rhs_identifier: weedle!(Identifier) >>
        (ExtendedAttributeIdent { lhs_identifier, assign, rhs_identifier })
    ));
}

/// Parses an argument list. Ex: `Constructor((double x, double y))`
///
/// (( )) means ( ) chars
#[derive(Debug, PartialEq, Clone)]
pub struct ExtendedAttributeArgList {
    pub identifier: Identifier,
    pub args: Braced<ArgumentList>
}

impl Parse for ExtendedAttributeArgList {
    named!(parse -> Self, do_parse!(
        identifier: weedle!(Identifier) >>
        args: weedle!(Braced<ArgumentList>) >>
        (ExtendedAttributeArgList { identifier, args })
    ));
}

/// Parses a plain attribute. Ex: `Replaceable`
#[derive(Debug, PartialEq, Clone)]
pub struct ExtendedAttributeNoArgs {
    pub identifier: Identifier
}

impl Parse for ExtendedAttributeNoArgs {
    named!(parse -> Self, do_parse!(
        identifier: weedle!(Identifier) >>
        (ExtendedAttributeNoArgs { identifier })
    ));
}

#[cfg(test)]
mod test {
    use super::*;

    test!(should_parse_attribute_no_args { "Replaceable" =>
        "";
        ExtendedAttributeNoArgs;
        identifier.name == "Replaceable";
    });

    test!(should_parse_attribute_arg_list { "Constructor(double x, double y)" =>
        "";
        ExtendedAttributeArgList;
        identifier.name == "Constructor";
        args.body.list.len() == 2;
    });

    test!(should_parse_attribute_ident { "PutForwards=name" =>
        "";
        ExtendedAttributeIdent;
        lhs_identifier.name == "PutForwards";
        rhs_identifier.name == "name";
    });

    test!(should_parse_ident_list { "Exposed=(Window,Worker)" =>
        "";
        ExtendedAttributeIdentList;
        identifier.name == "Exposed";
        list.body.list.len() == 2;
    });

    test!(should_parse_named_arg_list { "NamedConstructor=Image(DOMString src)" =>
        "";
        ExtendedAttributeNamedArgList;
        lhs_identifier.name == "NamedConstructor";
        rhs_identifier.name == "Image";
        args.body.list.len() == 1;
    });
}
