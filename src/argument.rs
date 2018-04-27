use literal::*;
use types::*;
use Parse;
use common::*;
use attribute::*;
use others::*;

/// Parses a list of argument. Ex: `double v1, double v2, double v3, optional double alpha`
pub type ArgumentList = Punctuated<Argument, term!(,)>;

/// Parses an argument. Ex: `double v1|double... v1s`
#[derive(Debug, PartialEq)]
pub enum Argument {
    Single(SingleArgument),
    Variadic(VariadicArgument)
}

impl Parse for Argument {
    named!(parse -> Self, alt!(
        weedle!(SingleArgument) => {|inner| Argument::Single(inner)} |
        weedle!(VariadicArgument) => {|inner| Argument::Variadic(inner)}
    ));
}

/// Parses `/* [attributes] */ /* optional */ type identifier /* = default */`
#[derive(Debug, PartialEq)]
pub struct SingleArgument {
    pub attributes: Option<ExtendedAttributeList>,
    pub optional: Option<term!(optional)>,
    pub type_: Type,
    pub identifier: Identifier,
    pub default: Option<Default>
}

impl Parse for SingleArgument {
    named!(parse -> Self, do_parse!(
        attributes: weedle!(Option<ExtendedAttributeList>) >>
        optional: weedle!(Option<term!(optional)>) >>
        type_: weedle!(Type) >>
        identifier: weedle!(Identifier) >>
        default: weedle!(Option<Default>) >>
        (SingleArgument { attributes, optional, type_, identifier, default })
    ));
}

/// Parses `/* [attributes] */ type/* ... */ identifier`
#[derive(Debug, PartialEq)]
pub struct VariadicArgument {
    pub attributes: Option<ExtendedAttributeList>,
    pub type_: Type,
    pub ellipsis: Option<term!(...)>,
    pub identifier: Identifier
}

impl Parse for VariadicArgument {
    named!(parse -> Self, do_parse!(
        attributes: weedle!(Option<ExtendedAttributeList>) >>
        type_: weedle!(Type) >>
        ellipsis: weedle!(Option<term!(...)>) >>
        identifier: weedle!(Identifier) >>
        (VariadicArgument { attributes, type_, ellipsis, identifier })
    ));
}

/// Parses any one of the keyword
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
