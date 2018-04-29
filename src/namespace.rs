use common::*;
use argument::*;
use types::*;
use Parse;
use attribute::*;

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

/// Parses `[attributes]? returntype identifier? (( args ));`
///
/// (( )) means ( ) chars
#[derive(Debug, PartialEq)]
pub struct OperationNamespaceMember {
    pub attributes: Option<ExtendedAttributeList>,
    pub return_type: ReturnType,
    pub identifier: Option<Identifier>,
    pub args: Braced<ArgumentList>,
    pub semi_colon: term!(;)
}

impl Parse for OperationNamespaceMember {
    named!(parse -> Self, do_parse!(
        attributes: weedle!(Option<ExtendedAttributeList>) >>
        return_type: weedle!(ReturnType) >>
        identifier: weedle!(Option<Identifier>) >>
        args: weedle!(Braced<ArgumentList>) >>
        semi_colon: weedle!(term!(;)) >>
        (OperationNamespaceMember { attributes, return_type, identifier, args, semi_colon })
    ));
}

/// Parses `[attribute]? readonly attribute type identifier;`
#[derive(Debug, PartialEq)]
pub struct AttributeNamespaceMember {
    pub attributes: Option<ExtendedAttributeList>,
    pub readonly: term!(readonly),
    pub attribute: term!(attribute),
    pub type_: Type,
    pub identifier: Identifier,
    pub semi_colon: term!(;)
}

impl Parse for AttributeNamespaceMember {
    named!(parse -> Self, do_parse!(
        attributes: weedle!(Option<ExtendedAttributeList>) >>
        readonly: weedle!(term!(readonly)) >>
        attribute: weedle!(term!(attribute)) >>
        type_: weedle!(Type) >>
        identifier: weedle!(Identifier) >>
        semi_colon: weedle!(term!(;)) >>
        (AttributeNamespaceMember { attributes, readonly, attribute, type_, identifier, semi_colon })
    ));
}

#[cfg(test)]
mod test {
    use super::*;

    test!(should_parse_attribute_namespace_member { "readonly attribute short name;" =>
        "";
        AttributeNamespaceMember;
        attributes.is_none();
        identifier.name == "name";
    });

    test!(should_parse_operation_namespace_member { "short (long a, long b);" =>
        "";
        OperationNamespaceMember;
        attributes.is_none();
        identifier.is_none();
    });
}
