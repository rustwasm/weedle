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
mod terminals;
mod literals;
mod attributes;
mod arguments;
mod types;
mod common;
mod others;
mod callback;

trait Parse: Sized {
    fn parse(input: CompleteStr) -> IResult<CompleteStr, Self>;
}
