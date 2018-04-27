use Parse;
use literal::*;
use common::*;
use argument::*;
use interface::*;
use types::*;

/// Parses the members declarations of a mixin
pub type MixinMembers = Vec<MixinMember>;

/// Parses one of the variants of a mixin member
#[derive(Debug, PartialEq)]
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

/// Parses `/* stringifier */ returntype /* identifier */ ( args );`
#[derive(Debug, PartialEq)]
pub struct OperationMixinMember {
    pub stringifier: Option<term!(stringifier)>,
    pub return_type: ReturnType,
    pub identifier: Option<Identifier>,
    pub args: Braced<ArgumentList>,
    pub semi_colon: term!(;)
}

impl Parse for OperationMixinMember {
    named!(parse -> Self, do_parse!(
        stringifier: weedle!(Option<term!(stringifier)>) >>
        return_type: weedle!(ReturnType) >>
        identifier: weedle!(Option<Identifier>) >>
        args: weedle!(Braced<ArgumentList>) >>
        semi_colon: weedle!(term!(;)) >>
        (OperationMixinMember { stringifier, return_type, identifier, args, semi_colon })
    ));
}

/// Parses `/* stringifier */ /* readonly */ attribute type identifier;`
#[derive(Debug, PartialEq)]
pub struct AttributeMixinMember {
    pub stringifier: Option<term!(stringifier)>,
    pub readonly: Option<term!(readonly)>,
    pub attribute: term!(attribute),
    pub type_: Type,
    pub identifier: Identifier,
    pub semi_colon: term!(;)
}

impl Parse for AttributeMixinMember {
    named!(parse -> Self, do_parse!(
        stringifier: weedle!(Option<term!(stringifier)>) >>
        readonly: weedle!(Option<term!(readonly)>) >>
        attribute: weedle!(term!(attribute)) >>
        type_: weedle!(Type) >>
        identifier: weedle!(Identifier) >>
        semi_colon: weedle!(term!(;)) >>
        (AttributeMixinMember { stringifier, readonly, attribute, type_, identifier, semi_colon })
    ));
}
