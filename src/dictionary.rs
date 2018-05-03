use attribute::*;
use types::*;
use Parse;
use common::*;

/// Parses dictionary members
pub type DictionaryMembers = Vec<DictionaryMember>;

/// Parses dictionary member `[attributes]? required? type identifier ( = default )?;`
#[derive(Debug, PartialEq, Clone)]
pub struct DictionaryMember {
    pub attributes: Option<ExtendedAttributeList>,
    pub required: Option<term!(required)>,
    pub type_: Type,
    pub identifier: Identifier,
    pub default: Option<Default>,
    pub semi_colon: term!(;)
}

impl Parse for DictionaryMember {
    named!(parse -> Self, do_parse!(
        attributes: weedle!(Option<ExtendedAttributeList>) >>
        required: weedle!(Option<term!(required)>) >>
        type_: weedle!(Type) >>
        identifier: weedle!(Identifier) >>
        default: weedle!(Option<Default>) >>
        semi_colon: weedle!(term!(;)) >>
        (DictionaryMember { attributes, required, type_, identifier, default, semi_colon })
    ));
}

#[cfg(test)]
mod test {
    use super::*;

    test!(should_parse_dictionary_member { "required long num = 5;" =>
        "";
        DictionaryMember;
        attributes.is_none();
        required.is_some();
        identifier.name == "num";
        default.is_some();
    });
}
