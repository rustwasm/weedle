//! Weedle - A WebIDL Parser
//!
//! Follows the grammar as defined on [WebIDL](https://heycam.github.io/webidl)

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate nom;
extern crate regex;

use argument::*;
use attribute::*;
use common::*;
use dictionary::*;
use enums::*;
use interface::*;
use literal::*;
use namespace::*;
use nom::{IResult, types::CompleteStr};
use types::*;

#[macro_use]
mod macros;
#[macro_use]
pub mod term;
pub mod literal;
pub mod attribute;
pub mod argument;
pub mod types;
pub mod common;
pub mod others;
pub mod interface;
pub mod enums;
pub mod dictionary;
pub mod namespace;
pub mod partial;
pub mod includes;

pub trait Parse: Sized {
    fn parse(input: CompleteStr) -> IResult<CompleteStr, Self>;
}

/// Parses WebIDL definitions
#[derive(Debug, PartialEq)]
pub struct Definitions {
    pub definitions: Vec<Definition>
}

impl Parse for Definitions {
    named!(parse -> Self, do_parse!(
        definitions: many0!(weedle!(Definition)) >>
        (Definitions { definitions })
    ));
}

/// Parses a definition
#[derive(Debug, PartialEq)]
pub enum Definition {
    Callback(CallbackDefinition),
    CallbackInterface(CallbackInterfaceDefinition),
    Interface(InterfaceDefinition),
    InterfaceMixin(InterfaceMixinDefinition),
    Namespace(NamespaceDefinition),
    Dictionary(DictionaryDefinition),
    PartialInterface(PartialInterfaceDefinition),
    PartialInterfaceMixin(PartialInterfaceMixinDefinition),
    PartialDictionary(PartialDictionaryDefinition),
    PartialNamespace(PartialNamespaceDefinition),
    Enum(EnumDefinition),
    Typedef(TypedefDefinition),
    IncludesStatement(IncludesStatementDefinition),
}

impl Parse for Definition {
    named!(parse -> Self, alt!(
        weedle!(CallbackDefinition) => {|inner| Definition::Callback(inner)} |
        weedle!(CallbackInterfaceDefinition) => {|inner| Definition::CallbackInterface(inner)} |
        weedle!(InterfaceDefinition) => {|inner| Definition::Interface(inner)} |
        weedle!(InterfaceMixinDefinition) => {|inner| Definition::InterfaceMixin(inner)} |
        weedle!(NamespaceDefinition) => {|inner| Definition::Namespace(inner)} |
        weedle!(DictionaryDefinition) => {|inner| Definition::Dictionary(inner)} |
        weedle!(PartialInterfaceDefinition) => {|inner| Definition::PartialInterface(inner)} |
        weedle!(PartialInterfaceMixinDefinition) => {|inner| Definition::PartialInterfaceMixin(inner)} |
        weedle!(PartialDictionaryDefinition) => {|inner| Definition::PartialDictionary(inner)} |
        weedle!(PartialNamespaceDefinition) => {|inner| Definition::PartialNamespace(inner)} |
        weedle!(EnumDefinition) => {|inner| Definition::Enum(inner)} |
        weedle!(TypedefDefinition) => {|inner| Definition::Typedef(inner)} |
        weedle!(IncludesStatementDefinition) => {|inner| Definition::IncludesStatement(inner)}
    ));
}

/// Parses `/* [attributes] */ callback identifier = type ( /* arg1, arg2, ..., argN */ );`
#[derive(Debug, PartialEq)]
pub struct CallbackDefinition {
    pub attributes: Option<ExtendedAttributeList>,
    pub callback: term!(callback),
    pub identifier: Identifier,
    pub assign: term!(=),
    pub return_type: ReturnType,
    pub arguments: Braced<ArgumentList>,
    pub semi_colon: term!(;),
}

impl Parse for CallbackDefinition {
    named!(parse -> Self, do_parse!(
        attributes: weedle!(Option<ExtendedAttributeList>) >>
        callback: weedle!(term!(callback)) >>
        identifier: weedle!(Identifier) >>
        assign: weedle!(term!(=)) >>
        return_type: weedle!(ReturnType) >>
        arguments: weedle!(Braced<ArgumentList>) >>
        semi_colon: weedle!(term!(;)) >>
        (CallbackDefinition { attributes, callback, identifier, assign, return_type, arguments, semi_colon })
    ));
}

/// Parses `/* [attributes] */ callback interface identifier /* : inheritance */ { members };`
#[derive(Debug, PartialEq)]
pub struct CallbackInterfaceDefinition {
    pub attributes: Option<ExtendedAttributeList>,
    pub callback: term!(callback),
    pub interface: term!(interface),
    pub identifier: Identifier,
    pub inheritance: Option<Inheritance>,
    pub members: Parenthesized<InterfaceMembers>,
    pub semi_colon: term!(;),
}

impl Parse for CallbackInterfaceDefinition {
    named!(parse -> Self, do_parse!(
        attributes: weedle!(Option<ExtendedAttributeList>) >>
        callback: weedle!(term!(callback)) >>
        interface: weedle!(term!(interface)) >>
        identifier: weedle!(Identifier) >>
        inheritance: weedle!(Option<Inheritance>) >>
        members: weedle!(Parenthesized<InterfaceMembers>) >>
        semi_colon: weedle!(term!(;)) >>
        (CallbackInterfaceDefinition { attributes, callback, interface, identifier, inheritance, members, semi_colon })
    ));
}

/// Parses `/* [attributes] */ interface identifier /* : inheritance */ { members };`
#[derive(Debug, PartialEq)]
pub struct InterfaceDefinition {
    pub attributes: Option<ExtendedAttributeList>,
    pub interface: term!(interface),
    pub identifier: Identifier,
    pub inheritance: Option<Inheritance>,
    pub members: Parenthesized<InterfaceMembers>,
    pub semi_colon: term!(;)
}

impl Parse for InterfaceDefinition {
    named!(parse -> Self, do_parse!(
        attributes: weedle!(Option<ExtendedAttributeList>) >>
        interface: weedle!(term!(interface)) >>
        identifier: weedle!(Identifier) >>
        inheritance: weedle!(Option<Inheritance>) >>
        members: weedle!(Parenthesized<InterfaceMembers>) >>
        semi_colon: weedle!(term!(;)) >>
        (InterfaceDefinition { attributes, interface, identifier, inheritance, members, semi_colon })
    ));
}

/// Parses `/* [attributes] */ interface mixin identifier { members };`
#[derive(Debug, PartialEq)]
pub struct InterfaceMixinDefinition {
    pub attributes: Option<ExtendedAttributeList>,
    pub interface: term!(interface),
    pub mixin: term!(mixin),
    pub identifier: Identifier,
    pub members: Parenthesized<MixinMembers>,
    pub semi_colon: term!(;)
}

impl Parse for InterfaceMixinDefinition {
    named!(parse -> Self, do_parse!(
        attributes: weedle!(Option<ExtendedAttributeList>) >>
        interface: weedle!(term!(interface)) >>
        mixin: weedle!(term!(mixin)) >>
        identifier: weedle!(Identifier) >>
        members: weedle!(Parenthesized<MixinMembers>) >>
        semi_colon: weedle!(term!(;)) >>
        (InterfaceMixinDefinition { attributes, interface, mixin, identifier, members, semi_colon })
    ));
}

/// Parses `/* [attributes] */ namespace identifier { members };`
#[derive(Debug, PartialEq)]
pub struct NamespaceDefinition {
    pub attributes: Option<ExtendedAttributeList>,
    pub namespace: term!(namespace),
    pub identifier: Identifier,
    pub members: Parenthesized<NamespaceMembers>,
    pub semi_colon: term!(;)
}

impl Parse for NamespaceDefinition {
    named!(parse -> Self, do_parse!(
        attributes: weedle!(Option<ExtendedAttributeList>) >>
        namespace: weedle!(term!(namespace)) >>
        identifier: weedle!(Identifier) >>
        members: weedle!(Parenthesized<NamespaceMembers>) >>
        semi_colon: weedle!(term!(;)) >>
        (NamespaceDefinition { attributes, namespace, identifier, members, semi_colon })
    ));
}

/// Parses `/* [attributes] */ partial interface identifier { members };`
#[derive(Debug, PartialEq)]
pub struct PartialInterfaceDefinition {
    pub attributes: Option<ExtendedAttributeList>,
    pub partial: term!(partial),
    pub interface: term!(interface),
    pub identifier: Identifier,
    pub members: Parenthesized<InterfaceMembers>,
    pub semi_colon: term!(;)
}

impl Parse for PartialInterfaceDefinition {
    named!(parse -> Self, do_parse!(
        attributes: weedle!(Option<ExtendedAttributeList>) >>
        partial: weedle!(term!(partial)) >>
        interface: weedle!(term!(interface)) >>
        identifier: weedle!(Identifier) >>
        members: weedle!(Parenthesized<InterfaceMembers>) >>
        semi_colon: weedle!(term!(;)) >>
        (PartialInterfaceDefinition { attributes, partial, interface, identifier, members, semi_colon })
    ));
}

/// Parses `/* [attributes] */ partial interface mixin identifier { members };`
#[derive(Debug, PartialEq)]
pub struct PartialInterfaceMixinDefinition {
    pub attributes: Option<ExtendedAttributeList>,
    pub partial: term!(partial),
    pub interface: term!(interface),
    pub mixin: term!(mixin),
    pub identifier: Identifier,
    pub members: Parenthesized<MixinMembers>,
    pub semi_colon: term!(;)
}

impl Parse for PartialInterfaceMixinDefinition {
    named!(parse -> Self, do_parse!(
        attributes: weedle!(Option<ExtendedAttributeList>) >>
        partial: weedle!(term!(partial)) >>
        interface: weedle!(term!(interface)) >>
        mixin: weedle!(term!(mixin)) >>
        identifier: weedle!(Identifier) >>
        members: weedle!(Parenthesized<MixinMembers>) >>
        semi_colon: weedle!(term!(;)) >>
        (PartialInterfaceMixinDefinition { attributes, partial, interface, mixin, identifier, members, semi_colon })
    ));
}

/// Parses `/* [attributes] */ partial dictionary identifier { members };`
#[derive(Debug, PartialEq)]
pub struct PartialDictionaryDefinition {
    pub attributes: Option<ExtendedAttributeList>,
    pub partial: term!(partial),
    pub dictionary: term!(dictionary),
    pub identifier: Identifier,
    pub members: Parenthesized<DictionaryMembers>,
    pub semi_colon: term!(;)
}

impl Parse for PartialDictionaryDefinition {
    named!(parse -> Self, do_parse!(
        attributes: weedle!(Option<ExtendedAttributeList>) >>
        partial: weedle!(term!(partial)) >>
        dictionary: weedle!(term!(dictionary)) >>
        identifier: weedle!(Identifier) >>
        members: weedle!(Parenthesized<DictionaryMembers>) >>
        semi_colon: weedle!(term!(;)) >>
        (PartialDictionaryDefinition { attributes, partial, dictionary, identifier, members, semi_colon })
    ));
}

/// Parses `/* [attributes] */ partial namespace identifier { members };`
#[derive(Debug, PartialEq)]
pub struct PartialNamespaceDefinition {
    pub attributes: Option<ExtendedAttributeList>,
    pub partial: term!(partial),
    pub namespace: term!(namespace),
    pub identifier: Identifier,
    pub members: Parenthesized<NamespaceMembers>,
    pub semi_colon: term!(;)
}

impl Parse for PartialNamespaceDefinition {
    named!(parse -> Self, do_parse!(
        attributes: weedle!(Option<ExtendedAttributeList>) >>
        partial: weedle!(term!(partial)) >>
        namespace: weedle!(term!(namespace)) >>
        identifier: weedle!(Identifier) >>
        members: weedle!(Parenthesized<NamespaceMembers>) >>
        semi_colon: weedle!(term!(;)) >>
        (PartialNamespaceDefinition { attributes, partial, namespace, identifier, members, semi_colon })
    ));
}

/// Parses `/* [attributes] */ dictionary identifier /* : inheritance */ { members };`
#[derive(Debug, PartialEq)]
pub struct DictionaryDefinition {
    pub attributes: Option<ExtendedAttributeList>,
    pub dictionary: term!(dictionary),
    pub identifier: Identifier,
    pub inheritance: Option<Inheritance>,
    pub members: Parenthesized<DictionaryMembers>,
    pub semi_colon: term!(;)
}

impl Parse for DictionaryDefinition {
    named!(parse -> Self, do_parse!(
        attributes: weedle!(Option<ExtendedAttributeList>) >>
        dictionary: weedle!(term!(dictionary)) >>
        identifier: weedle!(Identifier) >>
        inheritance: weedle!(Option<Inheritance>) >>
        members: weedle!(Parenthesized<DictionaryMembers>) >>
        semi_colon: weedle!(term!(;)) >>
        (DictionaryDefinition { attributes, dictionary, identifier, inheritance, members, semi_colon })
    ));
}

/// Parses `/* [attributes] */ enum identifier { values };`
#[derive(Debug, PartialEq)]
pub struct EnumDefinition {
    pub attributes: Option<ExtendedAttributeList>,
    pub enum_: term!(enum),
    pub identifier: Identifier,
    pub values: Parenthesized<EnumValueList>,
    pub semi_colon: term!(;)
}

impl Parse for EnumDefinition {
    named!(parse -> Self, do_parse!(
        attributes: weedle!(Option<ExtendedAttributeList>) >>
        enum_: weedle!(term!(enum)) >>
        identifier: weedle!(Identifier) >>
        values: weedle!(Parenthesized<EnumValueList>) >>
        semi_colon: weedle!(term!(;)) >>
        (EnumDefinition { attributes, enum_, identifier, values, semi_colon })
    ));
}

/// Parses `/* [attributes] */ typedef type identifier;`
#[derive(Debug, PartialEq)]
pub struct TypedefDefinition {
    pub attributes: Option<ExtendedAttributeList>,
    pub typedef: term!(typedef),
    pub type_: Type,
    pub identifier: Identifier,
    pub semi_colon: term!(;)
}

impl Parse for TypedefDefinition {
    named!(parse -> Self, do_parse!(
        attributes: weedle!(Option<ExtendedAttributeList>) >>
        typedef: weedle!(term!(typedef)) >>
        type_: weedle!(Type) >>
        identifier: weedle!(Identifier) >>
        semi_colon: weedle!(term!(;)) >>
        (TypedefDefinition { attributes, typedef, type_, identifier, semi_colon })
    ));
}

/// Parses `/* [attributes] */ identifier includes identifier;`
#[derive(Debug, PartialEq)]
pub struct IncludesStatementDefinition {
    pub attributes: Option<ExtendedAttributeList>,
    pub lhs_identifier: Identifier,
    pub includes: term!(includes),
    pub rhs_identifier: Identifier,
    pub semi_colon: term!(;)
}

impl Parse for IncludesStatementDefinition {
    named!(parse -> Self, do_parse!(
        attributes: weedle!(Option<ExtendedAttributeList>) >>
        lhs_identifier: weedle!(Identifier) >>
        includes: weedle!(term!(includes)) >>
        rhs_identifier: weedle!(Identifier) >>
        semi_colon: weedle!(term!(;)) >>
        (IncludesStatementDefinition { attributes, lhs_identifier, includes, rhs_identifier, semi_colon })
    ));
}
