use literal::*;
use common::*;
use Parse;

/// Parse an enum
///
/// ### Grammar
/// ```other
/// Enum ::
///     enum **identifier** { EnumValueList } ;
/// ```
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-Enum)
#[derive(Debug, PartialEq)]
pub struct Enum {
    pub enum_: term!(enum),
    pub identifier: Identifier,
    pub parenthesized: Parenthesized<EnumValueList>,
    pub semi_colon: term!(;)
}

impl Parse for Enum {
    named!(parse -> Self, do_parse!(
        enum_: weedle!(term!(enum)) >>
        identifier: weedle!(Identifier) >>
        parenthesized: weedle!(Parenthesized<EnumValueList>) >>
        semi_colon: weedle!(term!(;)) >>
        (Enum { enum_, identifier, parenthesized, semi_colon })
    ));
}

/// Parses a non-empty enum value list
///
/// ### Grammar
/// ```other
/// EnumValueList ::
///     **string** EnumValueListComma
/// EnumValueListComma ::
///     , EnumValueListString
///     ε
///
/// EnumValueListString ::
///     string EnumValueListComma
///     ε
///
/// ```
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-EnumValueList)
#[derive(Debug, PartialEq)]
pub struct EnumValueList {
    pub punctuated: PunctuatedNonEmpty<String, term!(,)>
}

impl Parse for EnumValueList {
    named!(parse -> Self, do_parse!(
        punctuated: weedle!(PunctuatedNonEmpty<String, term!(,)>) >>
        (EnumValueList { punctuated })
    ));
}
