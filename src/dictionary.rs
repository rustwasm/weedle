use attribute::*;
use types::*;
use Parse;
use common::*;

/// Parses dictionary members
pub type DictionaryMembers = Vec<DictionaryMember>;

/// Parses dictionary member `/* [attributes] */ /* required */ type identifier /* = default */`
#[derive(Debug, PartialEq)]
pub struct DictionaryMember {
    pub attributes: Option<ExtendedAttributeList>,
    pub required: Option<term!(required)>,
    pub type_: Type,
    pub identifier: Identifier,
    pub default: Option<Default>
}

impl Parse for DictionaryMember {
    named!(parse -> Self, do_parse!(
        attributes: weedle!(Option<ExtendedAttributeList>) >>
        required: weedle!(Option<term!(required)>) >>
        type_: weedle!(Type) >>
        identifier: weedle!(Identifier) >>
        default: weedle!(Option<Default>) >>
        (DictionaryMember { attributes, required, type_, identifier, default })
    ));
}
