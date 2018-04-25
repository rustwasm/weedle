use literal::*;
use interface::*;
use common::*;
use attribute::*;
use types::*;
use others::*;
use Parse;

/// Parses a dictionary declaration
///
/// ### Grammar
/// ```other
/// Dictionary ::
///     dictionary **identifier** Inheritance { DictionaryMembers } ;
/// ```
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-Dictionary)
#[derive(Debug, PartialEq)]
pub struct Dictionary {
    pub dictionary: term!(dictionary),
    pub identifier: Identifier,
    pub inheritance: Inheritance,
    pub parenthesized: Parenthesized<DictionaryMembers>,
    pub semi_colon: term!(;)
}

impl Parse for Dictionary {
    named!(parse -> Self, do_parse!(
        dictionary: weedle!(term!(dictionary)) >>
        identifier: weedle!(Identifier) >>
        inheritance: weedle!(Inheritance) >>
        parenthesized: weedle!(Parenthesized<DictionaryMembers>) >>
        semi_colon: weedle!(term!(;)) >>
        (Dictionary { dictionary, identifier, inheritance, parenthesized, semi_colon })
    ));
}

/// Parses dictionary members
///
/// ### Grammar
/// ```other
/// DictionaryMembers ::
///     DictionaryMember DictionaryMembers
///     ε
/// ```
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-DictionaryMembers)
#[derive(Debug, PartialEq)]
pub struct DictionaryMembers {
    members: Vec<DictionaryMember>
}

impl Parse for DictionaryMembers {
    named!(parse -> Self, do_parse!(
        members: many0!(weedle!(DictionaryMember)) >>
        (DictionaryMembers { members })
    ));
}

/// Parses a dictionary member
///
/// ### Grammar
/// ```other
/// DictionaryMember ::
///     ExtendedAttributeList DictionaryMemberRest
/// ```
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-DictionaryMember)
#[derive(Debug, PartialEq)]
pub struct DictionaryMember {
    pub attributes: ExtendedAttributeList,
    pub rest: DictionaryMemberRest
}

impl Parse for DictionaryMember {
    named!(parse -> Self, do_parse!(
        attributes: weedle!(ExtendedAttributeList) >>
        rest: weedle!(DictionaryMemberRest) >>
        (DictionaryMember { attributes, rest })
    ));
}

/// Parses a dictionary member without its attributes
///
/// ### Grammar
/// ```other
/// DictionaryMemberRest ::
///     required TypeWithExtendedAttributes **identifier** Default ;
///     Type **identifier** Default ;
/// ```
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-DictionaryMemberRest)
#[derive(Debug, PartialEq)]
pub enum DictionaryMemberRest {
    Required(RequiredDictionaryMemberRest),
    Plain(PlainDictionaryMemberRest)
}

impl Parse for DictionaryMemberRest {
    named!(parse -> Self, alt!(
        weedle!(RequiredDictionaryMemberRest) => {|inner| DictionaryMemberRest::Required(inner)} |
        weedle!(PlainDictionaryMemberRest) => {|inner| DictionaryMemberRest::Plain(inner)}
    ));
}

/// Parses a variant of [`DictionaryMemberRest`](enum.DictionaryMemberRest.html)
///
/// `required TypeWithExtendedAttributes **identifier** Default ;`
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-DictionaryMemberRest)
#[derive(Debug, PartialEq)]
pub struct RequiredDictionaryMemberRest {
    pub required: term!(required),
    pub type_: TypeWithExtendedAttributes,
    pub identifier: Identifier,
    pub default: Default,
    pub semi_colon: term!(;)
}

impl Parse for RequiredDictionaryMemberRest {
    named!(parse -> Self, do_parse!(
        required: weedle!(term!(required)) >>
        type_: weedle!(TypeWithExtendedAttributes) >>
        identifier: weedle!(Identifier) >>
        default: weedle!(Default) >>
        semi_colon: weedle!(term!(;)) >>
        (RequiredDictionaryMemberRest { required, type_, identifier, default, semi_colon })
    ));
}

/// Parses a variant of [`DictionaryMemberRest`](enum.DictionaryMemberRest.html)
///
/// `Type **identifier** Default ;`
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-DictionaryMemberRest)
#[derive(Debug, PartialEq)]
pub struct PlainDictionaryMemberRest {
    pub type_: Type,
    pub identifier: Identifier,
    pub default: Default,
    pub semi_colon: term!(;)
}

impl Parse for PlainDictionaryMemberRest {
    named!(parse -> Self, do_parse!(
        type_: weedle!(Type) >>
        identifier: weedle!(Identifier) >>
        default: weedle!(Default) >>
        semi_colon: weedle!(term!(;)) >>
        (PlainDictionaryMemberRest { type_, identifier, default, semi_colon })
    ));
}
