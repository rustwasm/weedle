//! Weedle - A WebIDL Parser
//!
//! Parses valid WebIDL definitions & produces a data structure starting from
//! [`Definitions`](struct.Definitions.html).
//!
//! ### Example
//!
//! ```
//! extern crate weedle;
//!
//! let parsed = weedle::parse("
//!     interface Window {
//!         readonly attribute Storage sessionStorage;
//!     };
//! ").unwrap();
//! println!("{:?}", parsed);
//! ```
//!
//! Note:
//! This parser follows the grammar given at [WebIDL](https://heycam.github.io/webidl).
//!
//! If any flaws found when parsing string with a valid grammar, create an issue.

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate nom;
extern crate regex;

use argument::*;
use attribute::*;
use common::*;
use dictionary::*;
use interface::*;
use namespace::*;
pub use nom::{IResult, types::CompleteStr};
use types::*;
use mixin::*;
use nom::Err;

#[macro_use]
mod whitespace;
#[macro_use]
mod macros;
#[macro_use]
pub mod term;
pub mod literal;
pub mod attribute;
pub mod argument;
pub mod types;
pub mod common;
pub mod interface;
pub mod mixin;
pub mod dictionary;
pub mod namespace;

/// A convenient parse function
///
/// ### Example
///
/// ```
/// extern crate weedle;
///
/// let parsed = weedle::parse("
///     interface Window {
///         readonly attribute Storage sessionStorage;
///     };
/// ").unwrap();
///
/// println!("{:?}", parsed);
/// ```
pub fn parse(raw: &str) -> Result<Definitions, Err<CompleteStr, u32>> {
    let (_, parsed) = Definitions::parse(CompleteStr(raw))?;
    Ok(parsed)
}

pub trait Parse: Sized {
    fn parse(input: CompleteStr) -> IResult<CompleteStr, Self>;
}

/// Parses WebIDL definitions. It is the root struct for a complete WebIDL definition.
///
/// ### Example
/// ```
/// use weedle::{Definitions, CompleteStr, Parse};
///
/// let (_, parsed) = Definitions::parse(CompleteStr("
///     interface Window {
///         readonly attribute Storage sessionStorage;
///     };
/// ")).unwrap();
///
/// println!("{:?}", parsed);
/// ```
///
/// It is recommended to use [`parse`](fn.parse.html) instead.
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

/// Parses `[attributes]? callback identifier = type ( (arg1, arg2, ..., argN)? );`
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

/// Parses `[attributes]? callback interface identifier ( : inheritance )? { members };`
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

/// Parses `[attributes]? interface identifier ( : inheritance )? { members };`
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

/// Parses `[attributes]? interface mixin identifier { members };`
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

/// Parses `[attributes]? namespace identifier { members };`
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

/// Parses `[attributes]? partial interface identifier { members };`
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

/// Parses `[attributes]? partial interface mixin identifier { members };`
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

/// Parses `[attributes]? partial dictionary identifier { members };`
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

/// Parses `[attributes]? partial namespace identifier { members };`
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

/// Parses `[attributes]? dictionary identifier ( : inheritance )? { members };`
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

/// Parses `[attributes]? enum identifier { values };`
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

/// Parses `[attributes]? typedef attributedtype identifier;`
#[derive(Debug, PartialEq)]
pub struct TypedefDefinition {
    pub attributes: Option<ExtendedAttributeList>,
    pub typedef: term!(typedef),
    pub type_: AttributedType,
    pub identifier: Identifier,
    pub semi_colon: term!(;)
}

impl Parse for TypedefDefinition {
    named!(parse -> Self, do_parse!(
        attributes: weedle!(Option<ExtendedAttributeList>) >>
        typedef: weedle!(term!(typedef)) >>
        type_: weedle!(AttributedType) >>
        identifier: weedle!(Identifier) >>
        semi_colon: weedle!(term!(;)) >>
        (TypedefDefinition { attributes, typedef, type_, identifier, semi_colon })
    ));
}

/// Parses `[attributes]? identifier includes identifier;`
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

/// Parses a non-empty enum value list
pub type EnumValueList = PunctuatedNonEmpty<String, term!(,)>;

#[cfg(test)]
mod test {
    use super::*;

    test!(should_parse_includes_statement { "first includes second;" =>
        "";
        IncludesStatementDefinition;
        attributes.is_none();
        lhs_identifier.name == "first";
        rhs_identifier.name == "second";
    });

    test!(should_parse_typedef { "typedef short Short;" =>
        "";
        TypedefDefinition;
        attributes.is_none();
        identifier.name == "Short";
    });

    test!(should_parse_enum { r#"enum name { "first", "second" }; "# =>
        "";
        EnumDefinition;
        attributes.is_none();
        identifier.name == "name";
        values.body.list.len() == 2;
    });

    test!(should_parse_dictionary { "dictionary A { long c; long g; };" =>
        "";
        DictionaryDefinition;
        attributes.is_none();
        identifier.name == "A";
        inheritance.is_none();
        members.body.len() == 2;
    });

    test!(should_parse_dictionary_inherited { "dictionary C : B { long e; long f; };" =>
        "";
        DictionaryDefinition;
        attributes.is_none();
        identifier.name == "C";
        inheritance.is_some();
        members.body.len() == 2;
    });

    test!(should_parse_partial_namespace { "
        partial namespace VectorUtils {
            readonly attribute Vector unit;
            double dotProduct(Vector x, Vector y);
            Vector crossProduct(Vector x, Vector y);
        };
    " =>
        "";
        PartialNamespaceDefinition;
        attributes.is_none();
        identifier.name == "VectorUtils";
        members.body.len() == 3;
    });

    test!(should_parse_partial_dictionary { "partial dictionary C { long e; long f; };" =>
        "";
        PartialDictionaryDefinition;
        attributes.is_none();
        identifier.name == "C";
        members.body.len() == 2;
    });

    test!(should_parse_partial_interface_mixin { "
        partial interface mixin WindowSessionStorage {
          readonly attribute Storage sessionStorage;
        };
    " =>
        "";
        PartialInterfaceMixinDefinition;
        attributes.is_none();
        identifier.name == "WindowSessionStorage";
        members.body.len() == 1;
    });

    test!(should_parse_partial_interface { "
        partial interface Window {
          readonly attribute Storage sessionStorage;
        };
    " =>
        "";
        PartialInterfaceDefinition;
        attributes.is_none();
        identifier.name == "Window";
        members.body.len() == 1;
    });

    test!(should_parse_namespace { "
        namespace VectorUtils {
          readonly attribute Vector unit;
          double dotProduct(Vector x, Vector y);
          Vector crossProduct(Vector x, Vector y);
        };
    " =>
        "";
        NamespaceDefinition;
        attributes.is_none();
        identifier.name == "VectorUtils";
        members.body.len() == 3;
    });

    test!(should_parse_interface_mixin { "
        interface mixin WindowSessionStorage {
          readonly attribute Storage sessionStorage;
        };
    " =>
        "";
        InterfaceMixinDefinition;
        attributes.is_none();
        identifier.name == "WindowSessionStorage";
        members.body.len() == 1;
    });

    test!(should_parse_interface { "
        interface Window {
          readonly attribute Storage sessionStorage;
        };
    " =>
        "";
        InterfaceDefinition;
        attributes.is_none();
        identifier.name == "Window";
        members.body.len() == 1;
    });

    test!(should_parse_callback_interface {"
        callback interface Options {
          attribute DOMString? option1;
          attribute DOMString? option2;
          attribute long? option3;
        };
    " =>
        "";
        CallbackInterfaceDefinition;
        attributes.is_none();
        identifier.name == "Options";
        members.body.len() == 3;
    });

    test!(should_parse_callback { "callback AsyncOperationCallback = void (DOMString status);" =>
        "";
        CallbackDefinition;
        attributes.is_none();
        identifier.name == "AsyncOperationCallback";
        arguments.body.list.len() == 1;
    });

    test!(should_parse_with_line_comments { "
        // This is a comment
        callback AsyncOperationCallback = void (DOMString status);
    " =>
        "";
        CallbackDefinition;
    });

    test!(should_parse_with_block_comments { "
        /* This is a comment */
        callback AsyncOperationCallback = void (DOMString status);
    " =>
        "";
        CallbackDefinition;
    });
}
