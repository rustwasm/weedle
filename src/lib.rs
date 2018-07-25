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

// need a higher recusion limit for macros
#![recursion_limit = "128"]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate nom;
extern crate regex;

use argument::ArgumentList;
use attribute::ExtendedAttributeList;
use common::{Braced, Identifier, Parenthesized, PunctuatedNonEmpty};
use dictionary::DictionaryMembers;
use interface::{Inheritance, InterfaceMembers};
use literal::StringLit;
use mixin::MixinMembers;
use namespace::NamespaceMembers;
pub use nom::{types::CompleteStr, Err, IResult};
use types::{AttributedType, ReturnType};

#[macro_use]
mod whitespace;
#[macro_use]
mod macros;
#[macro_use]
pub mod term;
pub mod argument;
pub mod attribute;
pub mod common;
pub mod dictionary;
pub mod interface;
pub mod literal;
pub mod mixin;
pub mod namespace;
pub mod types;

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
pub type Definitions = Vec<Definition>;

ast_types! {
    /// Parses a definition
    enum Definition {
        /// Parses `[attributes]? callback identifier = type ( (arg1, arg2, ..., argN)? );`
        Callback(struct CallbackDefinition {
            attributes: Option<ExtendedAttributeList>,
            callback: term!(callback),
            identifier: Identifier,
            assign: term!(=),
            return_type: ReturnType,
            arguments: Braced<ArgumentList>,
            semi_colon: term!(;),
        }),
        /// Parses `[attributes]? callback interface identifier ( : inheritance )? { members };`
        CallbackInterface(struct CallbackInterfaceDefinition {
            attributes: Option<ExtendedAttributeList>,
            callback: term!(callback),
            interface: term!(interface),
            identifier: Identifier,
            inheritance: Option<Inheritance>,
            members: Parenthesized<InterfaceMembers>,
            semi_colon: term!(;),
        }),
        /// Parses `[attributes]? interface identifier ( : inheritance )? { members };`
        Interface(struct InterfaceDefinition {
            attributes: Option<ExtendedAttributeList>,
            interface: term!(interface),
            identifier: Identifier,
            inheritance: Option<Inheritance>,
            members: Parenthesized<InterfaceMembers>,
            semi_colon: term!(;),
        }),
        /// Parses `[attributes]? interface mixin identifier { members };`
        InterfaceMixin(struct InterfaceMixinDefinition {
            attributes: Option<ExtendedAttributeList>,
            interface: term!(interface),
            mixin: term!(mixin),
            identifier: Identifier,
            members: Parenthesized<MixinMembers>,
            semi_colon: term!(;),
        }),
        /// Parses `[attributes]? namespace identifier { members };`
        Namespace(struct NamespaceDefinition {
            attributes: Option<ExtendedAttributeList>,
            namespace: term!(namespace),
            identifier: Identifier,
            members: Parenthesized<NamespaceMembers>,
            semi_colon: term!(;),
        }),
        /// Parses `[attributes]? dictionary identifier ( : inheritance )? { members };`
        Dictionary(struct DictionaryDefinition {
            attributes: Option<ExtendedAttributeList>,
            dictionary: term!(dictionary),
            identifier: Identifier,
            inheritance: Option<Inheritance>,
            members: Parenthesized<DictionaryMembers>,
            semi_colon: term!(;),
        }),
        /// Parses `[attributes]? partial interface identifier { members };`
        PartialInterface(struct PartialInterfaceDefinition {
            attributes: Option<ExtendedAttributeList>,
            partial: term!(partial),
            interface: term!(interface),
            identifier: Identifier,
            members: Parenthesized<InterfaceMembers>,
            semi_colon: term!(;),
        }),
        /// Parses `[attributes]? partial interface mixin identifier { members };`
        PartialInterfaceMixin(struct PartialInterfaceMixinDefinition {
            attributes: Option<ExtendedAttributeList>,
            partial: term!(partial),
            interface: term!(interface),
            mixin: term!(mixin),
            identifier: Identifier,
            members: Parenthesized<MixinMembers>,
            semi_colon: term!(;),
        }),
        /// Parses `[attributes]? partial dictionary identifier { members };`
        PartialDictionary(struct PartialDictionaryDefinition {
            attributes: Option<ExtendedAttributeList>,
            partial: term!(partial),
            dictionary: term!(dictionary),
            identifier: Identifier,
            members: Parenthesized<DictionaryMembers>,
            semi_colon: term!(;),
        }),
        /// Parses `[attributes]? partial namespace identifier { members };`
        PartialNamespace(struct PartialNamespaceDefinition {
            attributes: Option<ExtendedAttributeList>,
            partial: term!(partial),
            namespace: term!(namespace),
            identifier: Identifier,
            members: Parenthesized<NamespaceMembers>,
            semi_colon: term!(;),
        }),
        /// Parses `[attributes]? enum identifier { values };`
        Enum(struct EnumDefinition {
            attributes: Option<ExtendedAttributeList>,
            enum_: term!(enum),
            identifier: Identifier,
            values: Parenthesized<EnumValueList>,
            semi_colon: term!(;),
        }),
        /// Parses `[attributes]? typedef attributedtype identifier;`
        Typedef(struct TypedefDefinition {
            attributes: Option<ExtendedAttributeList>,
            typedef: term!(typedef),
            type_: AttributedType,
            identifier: Identifier,
            semi_colon: term!(;),
        }),
        /// Parses `[attributes]? identifier includes identifier;`
        IncludesStatement(struct IncludesStatementDefinition {
            attributes: Option<ExtendedAttributeList>,
            lhs_identifier: Identifier,
            includes: term!(includes),
            rhs_identifier: Identifier,
            semi_colon: term!(;),
        }),
    }
}

/// Parses a non-empty enum value list
pub type EnumValueList = PunctuatedNonEmpty<StringLit, term!(,)>;

#[cfg(test)]
mod test {
    use super::*;

    test!(should_parse_includes_statement { "first includes second;" =>
        "";
        IncludesStatementDefinition;
        attributes.is_none();
        lhs_identifier.0 == "first";
        rhs_identifier.0 == "second";
    });

    test!(should_parse_typedef { "typedef short Short;" =>
        "";
        TypedefDefinition;
        attributes.is_none();
        identifier.0 == "Short";
    });

    test!(should_parse_enum { r#"enum name { "first", "second" }; "# =>
        "";
        EnumDefinition;
        attributes.is_none();
        identifier.0 == "name";
        values.body.list.len() == 2;
    });

    test!(should_parse_dictionary { "dictionary A { long c; long g; };" =>
        "";
        DictionaryDefinition;
        attributes.is_none();
        identifier.0 == "A";
        inheritance.is_none();
        members.body.len() == 2;
    });

    test!(should_parse_dictionary_inherited { "dictionary C : B { long e; long f; };" =>
        "";
        DictionaryDefinition;
        attributes.is_none();
        identifier.0 == "C";
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
        identifier.0 == "VectorUtils";
        members.body.len() == 3;
    });

    test!(should_parse_partial_dictionary { "partial dictionary C { long e; long f; };" =>
        "";
        PartialDictionaryDefinition;
        attributes.is_none();
        identifier.0 == "C";
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
        identifier.0 == "WindowSessionStorage";
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
        identifier.0 == "Window";
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
        identifier.0 == "VectorUtils";
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
        identifier.0 == "WindowSessionStorage";
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
        identifier.0 == "Window";
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
        identifier.0 == "Options";
        members.body.len() == 3;
    });

    test!(should_parse_callback { "callback AsyncOperationCallback = void (DOMString status);" =>
        "";
        CallbackDefinition;
        attributes.is_none();
        identifier.0 == "AsyncOperationCallback";
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

    test!(should_parse_with_multiple_comments { "
        // This is a comment
        // This is a comment
        // This is a comment

        // This is a comment
        callback AsyncOperationCallback = void (DOMString status);
    " =>
        "";
        CallbackDefinition;
    });
}
