use literal::*;
use types::*;
use Parse;
use common::*;
use attribute::*;
use others::*;

/// Parses a list of argument. Ex: `double v1, double v2, double v3, optional double alpha`
pub type ArgumentList = Punctuated<Argument, term!(,)>;

/// Parses an argument. Ex: `double v1`
///
/// ### Grammar
/// ```other
/// Argument ::
///     ExtendedAttributeList ArgumentRest
/// ```
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-Argument)
#[derive(Debug, PartialEq)]
pub struct Argument {
    pub attributes: ExtendedAttributeList,
    pub rest: ArgumentRest,
}

impl Parse for Argument {
    named!(parse -> Self, do_parse!(
        attributes: weedle!(ExtendedAttributeList) >>
        rest: weedle!(ArgumentRest) >>
        (Argument { attributes, rest })
    ));
}

/// Parses either optional or non-optional argument. Ex: `optional double alpha` or `double alpha`.
///
/// ### Grammar
/// ```other
/// ArgumentRest ::
///     optional TypeWithExtendedAttributes ArgumentName Default
///     Type Ellipsis ArgumentName
/// ```
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-ArgumentRest)
#[derive(Debug, PartialEq)]
pub enum ArgumentRest {
    Optional(OptionalArgumentRest),
    Normal(NormalArgumentRest),
}

impl Parse for ArgumentRest {
    named!(parse -> Self, alt_complete!(
        weedle!(OptionalArgumentRest) => {|inner| ArgumentRest::Optional(inner)} |
        weedle!(NormalArgumentRest) => {|inner| ArgumentRest::Normal(inner)}
    ));
}

#[derive(Debug, PartialEq)]
pub struct OptionalArgumentRest {
    pub optional: term!(optional),
    pub type_: TypeWithExtendedAttributes,
    pub name: ArgumentName,
    pub default: Option<Default>,
}

impl Parse for OptionalArgumentRest {
    named!(parse -> Self, do_parse!(
        optional: weedle!(term!(optional)) >>
        type_: weedle!(TypeWithExtendedAttributes) >>
        name: weedle!(ArgumentName) >>
        default: weedle!(Option<Default>) >>
        (OptionalArgumentRest { optional, type_, name, default })
    ));
}

#[derive(Debug, PartialEq)]
pub struct NormalArgumentRest {
    pub type_: Type,
    pub ellipsis: Option<term!(...)>,
    pub name: ArgumentName,
}

impl Parse for NormalArgumentRest {
    named!(parse -> Self, do_parse!(
        type_: weedle!(Type) >>
        ellipsis: weedle!(Option<term!(...)>) >>
        name: weedle!(ArgumentName) >>
        (NormalArgumentRest { type_, ellipsis, name })
    ));
}

/// Parses the argument name
///
/// ### Grammar
/// ```other
/// ArgumentName ::
///     ArgumentNameKeyword
///     **identifier**
/// ```
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-ArgumentName)
#[derive(Debug, PartialEq)]
pub enum ArgumentName {
    Keyword(ArgumentNameKeyword),
    Identifier(Identifier),
}

impl Parse for ArgumentName {
    named!(parse -> Self, alt_complete!(
        weedle!(ArgumentNameKeyword) => {|inner| ArgumentName::Keyword(inner)} |
        weedle!(Identifier) => {|inner| ArgumentName::Identifier(inner)}
    ));
}

/// Parses any one of the keyword
///
/// ### Grammar
/// ```other
/// ArgumentNameKeyword ::
///     attribute
///     callback
///     const
///     deleter
///     dictionary
///     enum
///     getter
///     includes
///     inherit
///     interface
///     iterable
///     maplike
///     namespace
///     partial
///     required
///     setlike
///     setter
///     static
///     stringifier
///     typedef
///     unrestricted
/// ```
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-ArgumentNameKeyword)
#[derive(Debug, PartialEq)]
pub enum ArgumentNameKeyword {
    Attribute(term!(attribute)),
    Callback(term!(callback)),
    Const(term!(const)),
    Deleter(term!(deleter)),
    Dictionary(term!(dictionary)),
    Enum(term!(enum)),
    Getter(term!(getter)),
    Includes(term!(includes)),
    Inherit(term!(inherit)),
    Interface(term!(interface)),
    Iterable(term!(iterable)),
    Maplike(term!(maplike)),
    Namespace(term!(namespace)),
    Partial(term!(partial)),
    Required(term!(required)),
    Setlike(term!(setlike)),
    Setter(term!(setter)),
    Static(term!(static)),
    Stringifier(term!(stringifier)),
    Typedef(term!(typedef)),
    Unrestricted(term!(unrestricted)),
}

impl Parse for ArgumentNameKeyword {
    named!(parse -> Self, alt_complete!(
        weedle!(term!(attribute)) => {|inner| ArgumentNameKeyword::Attribute(inner)} |
        weedle!(term!(callback)) => {|inner| ArgumentNameKeyword::Callback(inner)} |
        weedle!(term!(const)) => {|inner| ArgumentNameKeyword::Const(inner)} |
        weedle!(term!(deleter)) => {|inner| ArgumentNameKeyword::Deleter(inner)} |
        weedle!(term!(dictionary)) => {|inner| ArgumentNameKeyword::Dictionary(inner)} |
        weedle!(term!(enum)) => {|inner| ArgumentNameKeyword::Enum(inner)} |
        weedle!(term!(getter)) => {|inner| ArgumentNameKeyword::Getter(inner)} |
        weedle!(term!(includes)) => {|inner| ArgumentNameKeyword::Includes(inner)} |
        weedle!(term!(inherit)) => {|inner| ArgumentNameKeyword::Inherit(inner)} |
        weedle!(term!(interface)) => {|inner| ArgumentNameKeyword::Interface(inner)} |
        weedle!(term!(iterable)) => {|inner| ArgumentNameKeyword::Iterable(inner)} |
        weedle!(term!(maplike)) => {|inner| ArgumentNameKeyword::Maplike(inner)} |
        weedle!(term!(namespace)) => {|inner| ArgumentNameKeyword::Namespace(inner)} |
        weedle!(term!(partial)) => {|inner| ArgumentNameKeyword::Partial(inner)} |
        weedle!(term!(required)) => {|inner| ArgumentNameKeyword::Required(inner)} |
        weedle!(term!(setlike)) => {|inner| ArgumentNameKeyword::Setlike(inner)} |
        weedle!(term!(setter)) => {|inner| ArgumentNameKeyword::Setter(inner)} |
        weedle!(term!(static)) => {|inner| ArgumentNameKeyword::Static(inner)} |
        weedle!(term!(stringifier)) => {|inner| ArgumentNameKeyword::Stringifier(inner)} |
        weedle!(term!(typedef)) => {|inner| ArgumentNameKeyword::Typedef(inner)} |
        weedle!(term!(unrestricted)) => {|inner| ArgumentNameKeyword::Unrestricted(inner)}
    ));
}

#[cfg(test)]
mod test {
    use super::*;
    use Parse;
    use nom::types::CompleteStr;

    macro_rules! test_argument_name_keyword {
        ($(fn $name:ident { $var:ident => $val:expr }),*) => {
            $(
                #[test]
                fn $name() {
                    let (rem, parsed) = ArgumentNameKeyword::parse(CompleteStr($val)).unwrap();
                    assert_eq!(rem, CompleteStr(""));
                    assert_eq!(parsed, ArgumentNameKeyword::$var(::term::$var));
                }
            )*
        };
    }

    test_argument_name_keyword! {
        fn should_parse_argument_name_keyword_variant_attribute { Attribute => "attribute" },
        fn should_parse_argument_name_keyword_variant_callback { Callback => "callback" },
        fn should_parse_argument_name_keyword_variant_const { Const => "const" },
        fn should_parse_argument_name_keyword_variant_deleter { Deleter => "deleter" },
        fn should_parse_argument_name_keyword_variant_dictionary { Dictionary => "dictionary" },
        fn should_parse_argument_name_keyword_variant_enum { Enum => "enum" },
        fn should_parse_argument_name_keyword_variant_getter { Getter => "getter" },
        fn should_parse_argument_name_keyword_variant_includes { Includes => "includes" },
        fn should_parse_argument_name_keyword_variant_inherit { Inherit => "inherit" },
        fn should_parse_argument_name_keyword_variant_interface { Interface => "interface" },
        fn should_parse_argument_name_keyword_variant_iterable { Iterable => "iterable" },
        fn should_parse_argument_name_keyword_variant_maplike { Maplike => "maplike" },
        fn should_parse_argument_name_keyword_variant_namespace { Namespace => "namespace" },
        fn should_parse_argument_name_keyword_variant_partial { Partial => "partial" },
        fn should_parse_argument_name_keyword_variant_required { Required => "required" },
        fn should_parse_argument_name_keyword_variant_setlike { Setlike => "setlike" },
        fn should_parse_argument_name_keyword_variant_setter { Setter => "setter" },
        fn should_parse_argument_name_keyword_variant_static { Static => "static" },
        fn should_parse_argument_name_keyword_variant_stringifier { Stringifier => "stringifier" },
        fn should_parse_argument_name_keyword_variant_typedef { Typedef => "typedef" },
        fn should_parse_argument_name_keyword_variant_unrestricted { Unrestricted => "unrestricted" }
    }

    #[test]
    fn should_parse_argument_name_variant_keyword() {
        let (rem, parsed) = ArgumentName::parse(CompleteStr("getter")).unwrap();
        assert_eq!(rem, CompleteStr(""));
        assert_eq!(parsed, ArgumentName::Keyword(ArgumentNameKeyword::Getter(term!(getter))));
    }

    #[test]
    fn should_parse_argument_name_variant_identifier() {
        let (rem, parsed) = ArgumentName::parse(CompleteStr("document")).unwrap();
        assert_eq!(rem, CompleteStr(""));
        assert_eq!(parsed, ArgumentName::Identifier(Identifier { name: "document".to_string() }));
    }
}
