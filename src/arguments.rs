use literals::*;
use terminals::*;
use types::*;
use Parse;
use common::*;
use attributes::*;
use others::*;

/// ArgumentList ::
///     Argument Arguments
///     ε
///
/// Arguments ::
///     , Argument Arguments
///     ε
#[derive(Debug, PartialEq)]
pub struct ArgumentList {
    pub args: Punctuated<Argument, Comma>
}

impl Parse for ArgumentList {
    named!(parse -> Self, do_parse!(
        args: weedle!(Punctuated<Argument, Comma>) >>
        (ArgumentList { args })
    ));
}

/// Argument ::
///     ExtendedAttributeList ArgumentRest
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


/// ArgumentRest ::
///     optional TypeWithExtendedAttributes ArgumentName Default
///     Type Ellipsis ArgumentName
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
    pub optional: Optional,
    pub type_: TypeWithExtendedAttributes,
    pub name: ArgumentName,
    pub default: Option<Default>,
}

impl Parse for OptionalArgumentRest {
    named!(parse -> Self, do_parse!(
        optional: weedle!(Optional) >>
        type_: weedle!(TypeWithExtendedAttributes) >>
        name: weedle!(ArgumentName) >>
        default: weedle!(Option<Default>) >>
        (OptionalArgumentRest { optional, type_, name, default })
    ));
}

#[derive(Debug, PartialEq)]
pub struct NormalArgumentRest {
    pub type_: Type,
    pub ellipsis: Option<Ellipsis>,
    pub name: ArgumentName,
}

impl Parse for NormalArgumentRest {
    named!(parse -> Self, do_parse!(
        type_: weedle!(Type) >>
        ellipsis: weedle!(Option<Ellipsis>) >>
        name: weedle!(ArgumentName) >>
        (NormalArgumentRest { type_, ellipsis, name })
    ));
}

/// ArgumentName ::
///     ArgumentNameKeyword
///     identifier
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
#[derive(Debug, PartialEq)]
pub enum ArgumentNameKeyword {
    Attribute(Attribute),
    Callback(Callback),
    Const(Const),
    Deleter(Deleter),
    Dictionary(Dictionary),
    Enum(Enum),
    Getter(Getter),
    Includes(Includes),
    Inherit(Inherit),
    Interface(Interface),
    Iterable(Iterable),
    Maplike(Maplike),
    Namespace(Namespace),
    Partial(Partial),
    Required(Required),
    Setlike(Setlike),
    Setter(Setter),
    Static(Static),
    Stringifier(Stringifier),
    Typedef(Typedef),
    Unrestricted(Unrestricted),
}

impl Parse for ArgumentNameKeyword {
    named!(parse -> Self, alt_complete!(
        weedle!(Attribute) => {|inner| ArgumentNameKeyword::Attribute(inner)} |
        weedle!(Callback) => {|inner| ArgumentNameKeyword::Callback(inner)} |
        weedle!(Const) => {|inner| ArgumentNameKeyword::Const(inner)} |
        weedle!(Deleter) => {|inner| ArgumentNameKeyword::Deleter(inner)} |
        weedle!(Dictionary) => {|inner| ArgumentNameKeyword::Dictionary(inner)} |
        weedle!(Enum) => {|inner| ArgumentNameKeyword::Enum(inner)} |
        weedle!(Getter) => {|inner| ArgumentNameKeyword::Getter(inner)} |
        weedle!(Includes) => {|inner| ArgumentNameKeyword::Includes(inner)} |
        weedle!(Inherit) => {|inner| ArgumentNameKeyword::Inherit(inner)} |
        weedle!(Interface) => {|inner| ArgumentNameKeyword::Interface(inner)} |
        weedle!(Iterable) => {|inner| ArgumentNameKeyword::Iterable(inner)} |
        weedle!(Maplike) => {|inner| ArgumentNameKeyword::Maplike(inner)} |
        weedle!(Namespace) => {|inner| ArgumentNameKeyword::Namespace(inner)} |
        weedle!(Partial) => {|inner| ArgumentNameKeyword::Partial(inner)} |
        weedle!(Required) => {|inner| ArgumentNameKeyword::Required(inner)} |
        weedle!(Setlike) => {|inner| ArgumentNameKeyword::Setlike(inner)} |
        weedle!(Setter) => {|inner| ArgumentNameKeyword::Setter(inner)} |
        weedle!(Static) => {|inner| ArgumentNameKeyword::Static(inner)} |
        weedle!(Stringifier) => {|inner| ArgumentNameKeyword::Stringifier(inner)} |
        weedle!(Typedef) => {|inner| ArgumentNameKeyword::Typedef(inner)} |
        weedle!(Unrestricted) => {|inner| ArgumentNameKeyword::Unrestricted(inner)}
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
                    assert_eq!(parsed, ArgumentNameKeyword::$var($var));
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
        assert_eq!(parsed, ArgumentName::Keyword(ArgumentNameKeyword::Getter(Getter)));
    }

    #[test]
    fn should_parse_argument_name_variant_identifier() {
        let (rem, parsed) = ArgumentName::parse(CompleteStr("document")).unwrap();
        assert_eq!(rem, CompleteStr(""));
        assert_eq!(parsed, ArgumentName::Identifier(Identifier { name: "document".to_string() }));
    }

    #[test]
    fn should_parse_argument_rest() {

    }
}
