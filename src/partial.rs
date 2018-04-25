use dictionary::*;
use interface::*;
use namespace::*;
use literal::*;
use common::*;
use Parse;

/// Parses a partial declaration
///
/// ### Grammar
/// ```other
/// Partial ::
///     partial PartialDefinition
/// ```
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-Partial)
#[derive(Debug, PartialEq)]
pub struct Partial {
    pub partial: term!(partial),
    pub definition: PartialDefinition
}

impl Parse for Partial {
    named!(parse -> Self, do_parse!(
        partial: weedle!(term!(partial)) >>
        definition: weedle!(PartialDefinition) >>
        (Partial { partial, definition })
    ));
}

/// Parses a partial definition
///
/// ### Grammar
/// ```other
/// PartialDefinition ::
///     interface PartialInterfaceOrPartialMixin
///     PartialDictionary
///     Namespace
/// ```
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-PartialDefinition)
#[derive(Debug, PartialEq)]
pub enum PartialDefinition {
    Interface(InterfacePartialDefinition),
    PartialDictionary(PartialDictionary),
    Namespace(Namespace)
}

impl Parse for PartialDefinition {
    named!(parse -> Self, alt!(
        weedle!(InterfacePartialDefinition) => {|inner| PartialDefinition::Interface(inner)} |
        weedle!(PartialDictionary) => {|inner| PartialDefinition::PartialDictionary(inner)} |
        weedle!(Namespace) => {|inner| PartialDefinition::Namespace(inner)}
    ));
}

/// Parses the `interface` variant of [`PartialDefinition`](enum.PartialDefinition.html)
///
/// `interface PartialInterfaceOrPartialMixin`
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-PartialDefinition)
#[derive(Debug, PartialEq)]
pub struct InterfacePartialDefinition {
    pub interface: term!(interface),
    pub rest: PartialInterfaceOrPartialMixin
}

impl Parse for InterfacePartialDefinition {
    named!(parse -> Self, do_parse!(
        interface: weedle!(term!(interface)) >>
        rest: weedle!(PartialInterfaceOrPartialMixin) >>
        (InterfacePartialDefinition { interface, rest })
    ));
}

/// Parses a partial interface or mixin
///
/// ### Grammar
/// ```other
/// PartialInterfaceOrPartialMixin ::
///     PartialInterfaceRest
///     MixinRest
/// ```
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-PartialInterfaceOrPartialMixin)
#[derive(Debug, PartialEq)]
pub enum PartialInterfaceOrPartialMixin {
    PartialInterfaceRest(PartialInterfaceRest),
    MixinRest(MixinRest)
}

impl Parse for PartialInterfaceOrPartialMixin {
    named!(parse -> Self, alt!(
        weedle!(PartialInterfaceRest) => {|inner| PartialInterfaceOrPartialMixin::PartialInterfaceRest(inner)} |
        weedle!(MixinRest) => {|inner| PartialInterfaceOrPartialMixin::MixinRest(inner)}
    ));
}

/// Parses remaining part of a partial interface
///
/// ### Grammar
/// ```other
/// PartialInterfaceRest ::
///     **identifier** { InterfaceMembers } ;
/// ```
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-PartialInterfaceRest)
#[derive(Debug, PartialEq)]
pub struct PartialInterfaceRest {
    pub identifier: Identifier,
    pub parenthesized: Parenthesized<InterfaceMember>,
    pub semi_colon: term!(;)
}

impl Parse for PartialInterfaceRest {
    named!(parse -> Self, do_parse!(
        identifier: weedle!(Identifier) >>
        parenthesized: weedle!(Parenthesized<InterfaceMember>) >>
        semi_colon: weedle!(term!(;)) >>
        (PartialInterfaceRest { identifier, parenthesized, semi_colon })
    ));
}

/// Parses a partial dictionary
///
/// ### Grammar
/// ```other
/// PartialDictionary ::
///     dictionary **identifier** { DictionaryMembers } ;
/// ```
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-PartialDictionary)
#[derive(Debug, PartialEq)]
pub struct PartialDictionary {
    pub dictionary: term!(dictionary),
    pub identifier: Identifier,
    pub parenthesized: Parenthesized<DictionaryMembers>,
    pub semi_colon: term!(;)
}

impl Parse for PartialDictionary {
    named!(parse -> Self, do_parse!(
        dictionary: weedle!(term!(dictionary)) >>
        identifier: weedle!(Identifier) >>
        parenthesized: weedle!(Parenthesized<DictionaryMembers>) >>
        semi_colon: weedle!(term!(;)) >>
        (PartialDictionary { dictionary, identifier, parenthesized, semi_colon })
    ));
}
