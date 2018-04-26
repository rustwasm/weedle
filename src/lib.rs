//! Weedle - A WebIDL Parser
//!
//! Follows the grammar as defined on [WebIDL](https://heycam.github.io/webidl)

#[macro_use]
extern crate nom;
extern crate regex;
#[macro_use]
extern crate lazy_static;

use nom::{IResult, types::CompleteStr};
use attribute::*;
use interface::*;
use namespace::*;
use dictionary::*;
use enums::*;
use types::*;
use partial::*;
use includes::*;

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
///
/// ### Grammar
/// ```other
/// Definitions ::
///     ExtendedAttributeList Definition Definitions
///     ε
/// ```
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-Definitions)
#[derive(Debug, PartialEq)]
pub struct Definitions {
    pub definitions: Vec<DefinitionsItem>
}

impl Parse for Definitions {
    named!(parse -> Self, do_parse!(
        definitions: many0!(weedle!(DefinitionsItem)) >>
        (Definitions { definitions })
    ));
}

/// Parses an item of a [`Definitions`](struct.Definitions.html)
///
/// `ExtendedAttributeList Definition`
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-Definitions)
#[derive(Debug, PartialEq)]
pub struct DefinitionsItem {
    pub attributes: ExtendedAttributeList,
    pub definition: Definition
}

impl Parse for DefinitionsItem {
    named!(parse -> Self, do_parse!(
        attributes: weedle!(ExtendedAttributeList) >>
        definition: weedle!(Definition) >>
        (DefinitionsItem { attributes, definition })
    ));
}

/// Parses a definition
///
/// ### Grammar
/// ```other
/// Definition ::
///     CallbackOrInterfaceOrMixin
///     Namespace
///     Partial
///     Dictionary
///     Enum
///     Typedef
///     IncludesStatement
/// ```
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-Definition)
#[derive(Debug, PartialEq)]
pub enum Definition {
    CallbackOrInterfaceOrMixin(CallbackOrInterfaceOrMixin),
    Namespace(Namespace),
    Partial(Partial),
    Dictionary(Dictionary),
    Enum(Enum),
    Typedef(Typedef),
    IncludesStatement(IncludesStatement)
}

impl Parse for Definition {
    named!(parse -> Self, alt!(
        weedle!(CallbackOrInterfaceOrMixin) => {|inner| Definition::CallbackOrInterfaceOrMixin(inner)} |
        weedle!(Namespace) => {|inner| Definition::Namespace(inner)} |
        weedle!(Partial) => {|inner| Definition::Partial(inner)} |
        weedle!(Dictionary) => {|inner| Definition::Dictionary(inner)} |
        weedle!(Enum) => {|inner| Definition::Enum(inner)} |
        weedle!(Typedef) => {|inner| Definition::Typedef(inner)} |
        weedle!(IncludesStatement) => {|inner| Definition::IncludesStatement(inner)}
    ));
}
