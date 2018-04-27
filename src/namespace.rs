use literal::Identifier;
use common::*;
use argument::*;
use types::*;
use Parse;

/// Parses namespace members declaration
pub type NamespaceMembers = Vec<NamespaceMember>;

/// Parses namespace member declaration
#[derive(Debug, PartialEq)]
pub enum NamespaceMember {
    Operation(OperationNamespaceMember),
    Attribute(AttributeNamespaceMember)
}

impl Parse for NamespaceMember {
    named!(parse -> Self, alt!(
        weedle!(OperationNamespaceMember) => {|inner| NamespaceMember::Operation(inner)} |
        weedle!(AttributeNamespaceMember) => {|inner| NamespaceMember::Attribute(inner)}
    ));
}

/// Parses `returntype /* identifier */( args );`
#[derive(Debug, PartialEq)]
pub struct OperationNamespaceMember {
    pub return_type: ReturnType,
    pub identifier: Option<Identifier>,
    pub args: Braced<ArgumentList>,
    pub semi_colon: term!(;)
}

impl Parse for OperationNamespaceMember {
    named!(parse -> Self, do_parse!(
        return_type: weedle!(ReturnType) >>
        identifier: weedle!(Option<Identifier>) >>
        args: weedle!(Braced<ArgumentList>) >>
        semi_colon: weedle!(term!(;)) >>
        (OperationNamespaceMember { return_type, identifier, args, semi_colon })
    ));
}

#[derive(Debug, PartialEq)]
pub struct AttributeNamespaceMember {
    pub readonly: term!(readonly),
    pub attribute: term!(attribute),
    pub type_: Type,
    pub identifier: Identifier,
    pub semi_colon: term!(;)
}

impl Parse for AttributeNamespaceMember {
    named!(parse -> Self, do_parse!(
        readonly: weedle!(term!(readonly)) >>
        attribute: weedle!(term!(attribute)) >>
        type_: weedle!(Type) >>
        identifier: weedle!(Identifier) >>
        semi_colon: weedle!(term!(;)) >>
        (AttributeNamespaceMember { readonly, attribute, type_, identifier, semi_colon })
    ));
}
