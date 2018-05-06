use Parse;
use common::*;
use argument::*;
use interface::*;
use types::*;
use attribute::*;

/// Parses the members declarations of a mixin
pub type MixinMembers = Vec<MixinMember>;

/// Parses one of the variants of a mixin member
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone)]
pub enum MixinMember {
    Const(ConstMember),
    Operation(OperationMixinMember),
    Attribute(AttributeMixinMember),
    Stringifier(StringifierMember)
}

impl Parse for MixinMember {
    named!(parse -> Self, alt!(
        weedle!(ConstMember) => {|inner| MixinMember::Const(inner)} |
        weedle!(OperationMixinMember) => {|inner| MixinMember::Operation(inner)} |
        weedle!(AttributeMixinMember) => {|inner| MixinMember::Attribute(inner)} |
        weedle!(StringifierMember) => {|inner| MixinMember::Stringifier(inner)}
    ));
}

/// Parses `[attributes]? stringifier? returntype identifier? (( args ));`
///
/// (( )) means ( ) chars
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone)]
pub struct OperationMixinMember {
    pub attributes: Option<ExtendedAttributeList>,
    pub stringifier: Option<term!(stringifier)>,
    pub return_type: ReturnType,
    pub identifier: Option<Identifier>,
    pub args: Braced<ArgumentList>,
    pub semi_colon: term!(;)
}

impl Parse for OperationMixinMember {
    named!(parse -> Self, do_parse!(
        attributes: weedle!(Option<ExtendedAttributeList>) >>
        stringifier: weedle!(Option<term!(stringifier)>) >>
        return_type: weedle!(ReturnType) >>
        identifier: weedle!(Option<Identifier>) >>
        args: weedle!(Braced<ArgumentList>) >>
        semi_colon: weedle!(term!(;)) >>
        (OperationMixinMember { attributes, stringifier, return_type, identifier, args, semi_colon })
    ));
}

/// Parses `[attributes]? stringifier? readonly? attribute attributedtype identifier;`
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone)]
pub struct AttributeMixinMember {
    pub attributes: Option<ExtendedAttributeList>,
    pub stringifier: Option<term!(stringifier)>,
    pub readonly: Option<term!(readonly)>,
    pub attribute: term!(attribute),
    pub type_: AttributedType,
    pub identifier: Identifier,
    pub semi_colon: term!(;)
}

impl Parse for AttributeMixinMember {
    named!(parse -> Self, do_parse!(
        attributes: weedle!(Option<ExtendedAttributeList>) >>
        stringifier: weedle!(Option<term!(stringifier)>) >>
        readonly: weedle!(Option<term!(readonly)>) >>
        attribute: weedle!(term!(attribute)) >>
        type_: weedle!(AttributedType) >>
        identifier: weedle!(Identifier) >>
        semi_colon: weedle!(term!(;)) >>
        (AttributeMixinMember { attributes, stringifier, readonly, attribute, type_, identifier, semi_colon })
    ));
}

#[cfg(test)]
mod test {
    use super::*;

    test!(should_parse_attribute_mixin_member { "stringifier readonly attribute short name;" =>
        "";
        AttributeMixinMember;
        attributes.is_none();
        stringifier.is_some();
        readonly.is_some();
        identifier.name == "name";
    });

    test!(should_parse_operation_mixin_member { "short fnName(long a);" =>
        "";
        OperationMixinMember;
        attributes.is_none();
        stringifier.is_none();
        identifier.is_some();
    });
}
