//! Weedle - A WebIDL Parser
//!
//! Follows the grammar as defined on [WebIDL](https://heycam.github.io/webidl)

#[macro_use]
extern crate nom;
extern crate regex;
#[macro_use]
extern crate lazy_static;

use nom::{IResult, types::CompleteStr};

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

pub trait Parse: Sized {
    fn parse(input: CompleteStr) -> IResult<CompleteStr, Self>;
}
