use attribute::ExtendedAttributeList;
use common::{Default, Identifier};
use types::Type;

/// Parses dictionary members
pub type DictionaryMembers = Vec<DictionaryMember>;

ast_types! {
    /// Parses dictionary member `[attributes]? required? type identifier ( = default )?;`
    struct DictionaryMember {
        attributes: Option<ExtendedAttributeList>,
        required: Option<term!(required)>,
        type_: Type,
        identifier: Identifier,
        default: Option<Default>,
        semi_colon: term!(;),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use Parse;

    test!(should_parse_dictionary_member { "required long num = 5;" =>
        "";
        DictionaryMember;
        attributes.is_none();
        required.is_some();
        identifier.0 == "num";
        default.is_some();
    });
}
