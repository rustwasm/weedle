use literal::*;
use Parse;
use common::*;
use argument::*;
use types::*;

/// Parses a list of attributes
pub type ExtendedAttributeList = Bracketed<Punctuated<ExtendedAttribute, term!(,)>>;

/// Parses on of the forms of attribute
#[derive(Debug, PartialEq)]
pub enum ExtendedAttribute {
    NoArgs(ExtendedAttributeNoArgs),
    ArgList(ExtendedAttributeArgList),
    NamedArgList(ExtendedAttributeNamedArgList),
    Ident(ExtendedAttributeIdent),
    IdentList(ExtendedAttributeIdentList),
}

impl Parse for ExtendedAttribute {
    named!(parse -> Self, alt_complete!(
        weedle!(ExtendedAttributeNoArgs) => {|inner| ExtendedAttribute::NoArgs(inner)} |
        weedle!(ExtendedAttributeArgList) => {|inner| ExtendedAttribute::ArgList(inner)} |
        weedle!(ExtendedAttributeNamedArgList) => {|inner| ExtendedAttribute::NamedArgList(inner)} |
        weedle!(ExtendedAttributeIdent) => {|inner| ExtendedAttribute::Ident(inner)} |
        weedle!(ExtendedAttributeIdentList) => {|inner| ExtendedAttribute::IdentList(inner)}
    ));
}

/// Parses a named argument list. Ex: `[NamedConstructor=Image(DOMString src)]`
#[derive(Debug, PartialEq)]
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

/// Parses an identifier list. Ex: `Exposed=(Window,Worker)`
#[derive(Debug, PartialEq)]
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
#[derive(Debug, PartialEq)]
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

/// Parses an argument list. Ex: `Constructor(double x, double y)`
#[derive(Debug, PartialEq)]
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
#[derive(Debug, PartialEq)]
pub struct ExtendedAttributeNoArgs {
    pub identifier: Identifier
}

impl Parse for ExtendedAttributeNoArgs {
    named!(parse -> Self, do_parse!(
        identifier: weedle!(Identifier) >>
        (ExtendedAttributeNoArgs { identifier })
    ));
}

/// Takes input of the form `attribute TheType theIdentifier;`
#[derive(Debug, PartialEq)]
pub struct AttributeRest {
    pub attribute: term!(attribute),
    pub type_: TypeWithExtendedAttributes,
    pub name: AttributeName,
    pub semi_colon: term!(;)
}

impl Parse for AttributeRest {
    named!(parse -> Self, do_parse!(
        attribute: weedle!(term!(attribute)) >>
        type_: weedle!(TypeWithExtendedAttributes) >>
        name: weedle!(AttributeName) >>
        semi_colon: weedle!(term!(;)) >>
        (AttributeRest { attribute, type_, name, semi_colon })
    ));
}

/// Parses either `required` or an **identifier**
///
/// ### Grammar
/// ```other
/// AttributeName ::
///     AttributeNameKeyword
///     **identifier**
///
/// AttributeNameKeyword ::
///     required
/// ```
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-AttributeName)
#[derive(Debug, PartialEq)]
pub enum AttributeName {
    Required(term!(required)),
    Identifier(Identifier)
}

impl Parse for AttributeName {
    named!(parse -> Self, alt_complete!(
        weedle!(term!(required)) => {|inner| AttributeName::Required(inner)} |
        weedle!(Identifier) => {|inner| AttributeName::Identifier(inner)}
    ));
}

#[cfg(test)]
mod test {
    use super::*;
    use nom::types::CompleteStr;

    #[test]
    fn should_take_named_argument_list() {
        let (rem, parsed) = ExtendedAttributeNamedArgList::parse("NamedConstructor=Image(DOMString src)".into())
            .unwrap();
        assert_eq!(rem, CompleteStr(""));
        assert_eq!(parsed, ExtendedAttributeNamedArgList {
            lhs_identifier: Identifier {
                name: "NamedConstructor".to_string()
            },
            assign: term!(=),
            rhs_identifier: Identifier {
                name: "Image".to_string()
            },
            args_signature: Braced {
                open_brace: term!(OpenBrace),
                body: ArgumentList {
                    args: Punctuated {
                        list: vec![
                            Argument {
                                attributes: ExtendedAttributeList {
                                    bracketed: None
                                },
                                rest: ArgumentRest::Normal(NormalArgumentRest {
                                    name: ArgumentName::Identifier(Identifier {
                                        name: "src".to_string()
                                    }),
                                    type_: Type::Single(
                                        Box::new(SingleType::NonAny(
                                            NonAnyType::MayBeString(
                                                MayBeNull {
                                                    type_: StringType::DOM(term!(DOMString)),
                                                    q_mark: None
                                                }
                                            )
                                        ))
                                    ),
                                    ellipsis: None
                                }),
                            },
                        ],
                        separator: term!(,),
                    }
                },
                close_brace: term!(CloseBrace),
            },
        })
    }
}
