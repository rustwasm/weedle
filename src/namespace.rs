use literal::Identifier;
use common::*;
use attribute::*;
use interface::*;
use Parse;

/// Parses a `namespace` declaration
///
/// ### Grammar
/// ```other
/// Namespace ::
///     namespace identifier { NamespaceMembers } ;
/// ```
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-Namespace)
#[derive(Debug, PartialEq)]
pub struct Namespace {
    pub namespace: term!(namespace),
    pub identifier: Identifier,
    pub parenthesized: Parenthesized<NamespaceMembers>,
    pub semi_colon: term!(;)
}

impl Parse for Namespace {
    named!(parse -> Self, do_parse!(
        namespace: weedle!(term!(namespace)) >>
        identifier: weedle!(Identifier) >>
        parenthesized: weedle!(Parenthesized<NamespaceMembers>) >>
        semi_colon: weedle!(term!(;)) >>
        (Namespace { namespace, identifier, parenthesized, semi_colon })
    ));
}

/// Parses a `namespace` members declaration
///
/// ### Grammar
/// ```other
/// NamespaceMembers ::
///     ExtendedAttributeList NamespaceMember NamespaceMembers
///     ε
/// ```
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-NamespaceMembers)
#[derive(Debug, PartialEq)]
pub struct NamespaceMembers {
    pub members: Vec<NamespaceMembersItem>
}

impl Parse for NamespaceMembers {
    named!(parse -> Self, do_parse!(
        members: many0!(weedle!(NamespaceMembersItem)) >>
        (NamespaceMembers { members })
    ));
}

/// Parses a single unit of [`NamespaceMembers`](struct.NamespaceMembers.html)
///
/// `ExtendedAttributeList NamespaceMember`
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-NamespaceMembers)
#[derive(Debug, PartialEq)]
pub struct NamespaceMembersItem {
    pub attributes: ExtendedAttributeList,
    pub member: NamespaceMember
}

impl Parse for NamespaceMembersItem {
    named!(parse -> Self, do_parse!(
        attributes: weedle!(ExtendedAttributeList) >>
        member: weedle!(NamespaceMember) >>
        (NamespaceMembersItem { attributes, member })
    ));
}

/// Parses a `namespace` member declaration
///
/// ### Grammar
/// ```other
/// NamespaceMember ::
///     RegularOperation
///     readonly AttributeRest
/// ```
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-NamespaceMember)
#[derive(Debug, PartialEq)]
pub enum NamespaceMember {
    RegularOperation(RegularOperation),
    ReadOnly(ReadOnlyAttributeRest)
}

impl Parse for NamespaceMember {
    named!(parse -> Self, alt!(
        weedle!(RegularOperation) => {|inner| NamespaceMember::RegularOperation(inner)} |
        weedle!(ReadOnlyAttributeRest) => {|inner| NamespaceMember::ReadOnly(inner)}
    ));
}
