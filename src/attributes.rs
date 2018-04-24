use literals::*;
use Parse;
use common::*;
use arguments::*;
use others::*;
use types::*;

/// Takes a named argument list. Ex: `[NamedConstructor=Image(DOMString src)]`
///
/// ### Grammar
/// ```other
/// ExtendedAttributeNamedArgList ::
///     **identifier** = **identifier** ( ArgumentList )
/// ```
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-ExtendedAttributeNamedArgList)
#[derive(Debug, PartialEq)]
pub struct ExtendedAttributeNamedArgList {
    pub lhs_identifier: Identifier,
    pub assign: term!(=),
    pub rhs_identifier: Identifier,
    pub args_signature: Braced<ArgumentList>,
}

impl Parse for ExtendedAttributeNamedArgList {
    named!(parse -> Self, do_parse!(
        lhs_identifier: weedle!(Identifier) >>
        assign: weedle!(term!(=)) >>
        rhs_identifier: weedle!(Identifier) >>
        args_signature: weedle!(Braced<ArgumentList>) >>
        (ExtendedAttributeNamedArgList { lhs_identifier, assign, rhs_identifier, args_signature })
    ));
}

/// Matches attributes of basically any form
///
/// ### Grammar
/// ```other
/// ExtendedAttributeList ::
///     [ ExtendedAttribute ExtendedAttributes ]
///     ε
///
/// ExtendedAttributes ::
///     , ExtendedAttribute ExtendedAttributes
///     ε
/// ```
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-ExtendedAttributeList)
#[derive(Debug, PartialEq)]
pub struct ExtendedAttributeList {
    pub list: Option<Bracketed<Punctuated<ExtendedAttribute, term!(,)>>>
}

impl Parse for ExtendedAttributeList {
    named!(parse -> Self, do_parse!(
        list: weedle!(Option<Bracketed<Punctuated<ExtendedAttribute, term!(,)>>>) >>
        (ExtendedAttributeList { list })
    ));
}

/// Matches a section of [ExtendedAttributeList](struct.ExtendedAttributeList.html)
///
/// ### Grammar
/// ```other
/// ExtendedAttribute ::
///     ( ExtendedAttributeInner ) ExtendedAttributeRest
///     [ ExtendedAttributeInner ] ExtendedAttributeRest
///     { ExtendedAttributeInner } ExtendedAttributeRest
///     Other ExtendedAttributeRest
///
/// ExtendedAttributeRest ::
///     ExtendedAttribute
///     ε
/// ```
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-ExtendedAttribute)
#[derive(Debug, PartialEq)]
pub enum ExtendedAttribute {
    Parenthesized(ParenthesizedExtendedAttribute),
    Bracketed(BracketedExtendedAttribute),
    Braced(BracedExtendedAttribute),
    Other(OtherExtendedAttribute),
}

impl Parse for ExtendedAttribute {
    named!(parse -> Self, alt_complete!(
        weedle!(ParenthesizedExtendedAttribute) => {|inner| ExtendedAttribute::Parenthesized(inner) } |
        weedle!(BracketedExtendedAttribute) => {|inner| ExtendedAttribute::Bracketed(inner)} |
        weedle!(BracedExtendedAttribute) => {|inner| ExtendedAttribute::Braced(inner)} |
        weedle!(OtherExtendedAttribute) => {|inner| ExtendedAttribute::Other(inner)}
    ));
}

#[derive(Debug, PartialEq)]
pub struct ParenthesizedExtendedAttribute {
    pub inner: Parenthesized<ExtendedAttributeInner>,
    pub rest: Option<Box<ExtendedAttribute>>,
}

impl Parse for ParenthesizedExtendedAttribute {
    named!(parse -> Self, do_parse!(
        inner: weedle!(Parenthesized<ExtendedAttributeInner>) >>
        rest: weedle!(Option<Box<ExtendedAttribute>>) >>
        (ParenthesizedExtendedAttribute { inner, rest })
    ));
}

#[derive(Debug, PartialEq)]
pub struct BracketedExtendedAttribute {
    pub inner: Bracketed<ExtendedAttributeInner>,
    pub rest: Option<Box<ExtendedAttribute>>,
}

impl Parse for BracketedExtendedAttribute {
    named!(parse -> Self, do_parse!(
        inner: weedle!(Bracketed<ExtendedAttributeInner>) >>
        rest: weedle!(Option<Box<ExtendedAttribute>>) >>
        (BracketedExtendedAttribute { inner, rest })
    ));
}

#[derive(Debug, PartialEq)]
pub struct BracedExtendedAttribute {
    pub inner: Braced<ExtendedAttributeInner>,
    pub rest: Option<Box<ExtendedAttribute>>,
}

impl Parse for BracedExtendedAttribute {
    named!(parse -> Self, do_parse!(
        inner: weedle!(Braced<ExtendedAttributeInner>) >>
        rest: weedle!(Option<Box<ExtendedAttribute>>) >>
        (BracedExtendedAttribute { inner, rest })
    ));
}

#[derive(Debug, PartialEq)]
pub struct OtherExtendedAttribute {
    pub other: Other,
    pub rest: Option<Box<ExtendedAttribute>>,
}

impl Parse for OtherExtendedAttribute {
    named!(parse -> Self, do_parse!(
        other: weedle!(Other) >>
        rest: weedle!(Option<Box<ExtendedAttribute>>) >>
        (OtherExtendedAttribute { other, rest })
    ));
}

/// Matches a section of [ExtendedAttribute](struct.ExtendedAttribute.html)
///
/// ### Grammar
/// ```other
/// ExtendedAttributeInner ::
///     ( ExtendedAttributeInner ) ExtendedAttributeInner
///     [ ExtendedAttributeInner ] ExtendedAttributeInner
///     { ExtendedAttributeInner } ExtendedAttributeInner
///     OtherOrComma ExtendedAttributeInner
///     ε
/// ```
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-ExtendedAttributeInner)
#[derive(Debug, PartialEq)]
pub enum ExtendedAttributeInner {
    Parenthesized(ParenthesizedExtendedAttributeInner),
    Bracketed(BracketedExtendedAttributeInner),
    Braced(BracedExtendedAttributeInner),
    Other(OtherExtendedAttributeInner),
    None,
}

impl Parse for ExtendedAttributeInner {
    named!(parse -> Self, alt_complete!(
        weedle!(ParenthesizedExtendedAttributeInner) => {|inner| ExtendedAttributeInner::Parenthesized(inner)} |
        weedle!(BracketedExtendedAttributeInner) => {|inner| ExtendedAttributeInner::Bracketed(inner)} |
        weedle!(BracedExtendedAttributeInner) => {|inner| ExtendedAttributeInner::Braced(inner) }|
        weedle!(OtherExtendedAttributeInner) => {|inner| ExtendedAttributeInner::Other(inner)} |
        tag!("") => {|_| ExtendedAttributeInner::None}
    ));
}

#[derive(Debug, PartialEq)]
pub struct ParenthesizedExtendedAttributeInner {
    pub inner: Parenthesized<Box<ExtendedAttributeInner>>,
    pub rest: Box<ExtendedAttributeInner>,
}

impl Parse for ParenthesizedExtendedAttributeInner {
    named!(parse -> Self, do_parse!(
        inner: weedle!(Parenthesized<Box<ExtendedAttributeInner>>) >>
        rest: weedle!(Box<ExtendedAttributeInner>) >>
        (ParenthesizedExtendedAttributeInner { inner, rest })
    ));
}

#[derive(Debug, PartialEq)]
pub struct BracketedExtendedAttributeInner {
    pub inner: Bracketed<Box<ExtendedAttributeInner>>,
    pub rest: Box<ExtendedAttributeInner>,
}

impl Parse for BracketedExtendedAttributeInner {
    named!(parse -> Self, do_parse!(
        inner: weedle!(Bracketed<Box<ExtendedAttributeInner>>) >>
        rest: weedle!(Box<ExtendedAttributeInner>) >>
        (BracketedExtendedAttributeInner { inner, rest })
    ));
}

#[derive(Debug, PartialEq)]
pub struct BracedExtendedAttributeInner {
    pub inner: Braced<Box<ExtendedAttributeInner>>,
    pub rest: Box<ExtendedAttributeInner>,
}

impl Parse for BracedExtendedAttributeInner {
    named!(parse -> Self, do_parse!(
        inner: weedle!(Braced<Box<ExtendedAttributeInner>>) >>
        rest: weedle!(Box<ExtendedAttributeInner>) >>
        (BracedExtendedAttributeInner { inner, rest })
    ));
}

#[derive(Debug, PartialEq)]
pub struct OtherExtendedAttributeInner {
    pub inner: OtherOrComma,
    pub rest: Box<ExtendedAttributeInner>,
}

impl Parse for OtherExtendedAttributeInner {
    named!(parse -> Self, do_parse!(
        inner: weedle!(OtherOrComma) >>
        rest: weedle!(Box<ExtendedAttributeInner>) >>
        (OtherExtendedAttributeInner { inner, rest })
    ));
}

/// Takes an identifier list. Ex: `[Exposed=(Window,Worker)]`
///
/// ### Grammar
/// ```other
/// ExtendedAttributeIdentList ::
///     **identifier** = ( IdentifierList )
/// ```
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-ExtendedAttributeIdentList)
#[derive(Debug, PartialEq)]
pub struct ExtendedAttributeIdentList {
    pub identifier: Identifier,
    pub assign: term!(=),
    pub braced: Braced<IdentifierList>
}

impl Parse for ExtendedAttributeIdentList {
    named!(parse -> Self, do_parse!(
        identifier: weedle!(Identifier) >>
        assign: weedle!(term!(=)) >>
        braced: weedle!(Braced<IdentifierList>) >>
        (ExtendedAttributeIdentList { identifier, assign, braced })
    ));
}

/// Matches comma separated identifier list
///
/// ### Grammar
/// ```other
/// IdentifierList ::
///     **identifier** Identifiers
/// ```
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-IdentifierList)
#[derive(Debug, PartialEq)]
pub struct IdentifierList {
    pub punctuated: Punctuated<Identifier, term!(,)>
}

impl Parse for IdentifierList {
    named!(parse -> Self, do_parse!(
        punctuated: weedle!(Punctuated<Identifier, term!(,)>) >>
        (IdentifierList { punctuated })
    ));
}

/// Takes an identifier list. Ex: `[Exposed=(Window,Worker)]`
///
/// ### Grammar
/// ```other
/// ExtendedAttributeIdent ::
///     **identifier** = **identifier**
/// ```
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-ExtendedAttributeIdent)
#[derive(Debug, PartialEq)]
pub struct ExtendedAttributeIdent {
    pub lhs_identifier: Identifier,
    pub assign: term!(=),
    pub rhs_identifier: Identifier
}

impl Parse for ExtendedAttributeIdent {
    named!(parse -> Self, do_parse!(
        lhs_identifier: weedle!(Identifier) >>
        assign: weedle!(term!(=)) >>
        rhs_identifier: weedle!(Identifier) >>
        (ExtendedAttributeIdent { lhs_identifier, assign, rhs_identifier })
    ));
}

/// Takes an argument list. Ex: `[Constructor(double x, double y)]`
///
/// ### Grammar
/// ```other
/// ExtendedAttributeArgList ::
///     **identifier** ( ArgumentList )
/// ```
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-ExtendedAttributeArgList)
#[derive(Debug, PartialEq)]
pub struct ExtendedAttributeArgList {
    pub identifier: Identifier,
    pub braced: Braced<ArgumentList>
}

impl Parse for ExtendedAttributeArgList {
    named!(parse -> Self, do_parse!(
        identifier: weedle!(Identifier) >>
        braced: weedle!(Braced<ArgumentList>) >>
        (ExtendedAttributeArgList { identifier, braced })
    ));
}

/// Takes no arguments. Ex: `[Replaceable]`
///
/// ### Grammar
/// ```other
/// ExtendedAttributeNoArgs ::
///     **identifier**
/// ```
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-ExtendedAttributeNoArgs)
#[derive(Debug, PartialEq)]
pub struct ExtendedAttributeNoArgs {
    identifier: Identifier
}

impl Parse for ExtendedAttributeNoArgs {
    named!(parse -> Self, do_parse!(
        identifier: weedle!(Identifier) >>
        (ExtendedAttributeNoArgs { identifier })
    ));
}

/// Takes input of the form `attribute TheType theIdentifier;`
///
/// ### Grammar
/// ```other
/// AttributeRest ::
///     attribute TypeWithExtendedAttributes AttributeName ;
/// ```
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-AttributeRest)
#[derive(Debug, PartialEq)]
pub struct AttributeRest {
    pub attribute: term!(attribute),
    pub type_: TypeWithExtendedAttributes,
    pub name: AttributeName,
    pub semi_colon: term!(;)
}

impl Parse for AttributeRest {
    named!(parse -> Self, do_parse!(
        attribute: weedle!(term!(attribute)) >>
        type_: weedle!(TypeWithExtendedAttributes) >>
        name: weedle!(AttributeName) >>
        semi_colon: weedle!(term!(;)) >>
        (AttributeRest { attribute, type_, name, semi_colon })
    ));
}

/// Parses either `required` or an **identifier**
///
/// ### Grammar
/// ```other
/// AttributeName ::
///     AttributeNameKeyword
///     **identifier**
///
/// AttributeNameKeyword ::
///     required
/// ```
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-AttributeName)
#[derive(Debug, PartialEq)]
pub enum AttributeName {
    Required(term!(required)),
    Identifier(Identifier)
}

impl Parse for AttributeName {
    named!(parse -> Self, alt_complete!(
        weedle!(term!(required)) => {|inner| AttributeName::Required(inner)} |
        weedle!(Identifier) => {|inner| AttributeName::Identifier(inner)}
    ));
}

#[cfg(test)]
mod test {
    use super::*;
    use nom::types::CompleteStr;

    #[test]
    fn should_take_named_argument_list() {
        let (rem, parsed) = ExtendedAttributeNamedArgList::parse("NamedConstructor=Image(DOMString src)".into())
            .unwrap();
        assert_eq!(rem, CompleteStr(""));
        assert_eq!(parsed, ExtendedAttributeNamedArgList {
            lhs_identifier: Identifier {
                name: "NamedConstructor".to_string()
            },
            assign: term!(=),
            rhs_identifier: Identifier {
                name: "Image".to_string()
            },
            args_signature: Braced {
                open_brace: term!(OpenBrace),
                body: ArgumentList {
                    args: Punctuated {
                        list: vec![
                            Argument {
                                attributes: ExtendedAttributeList {
                                    list: None
                                },
                                rest: ArgumentRest::Normal(NormalArgumentRest {
                                    name: ArgumentName::Identifier(Identifier {
                                        name: "src".to_string()
                                    }),
                                    type_: Type::Single(
                                        Box::new(SingleType::NonAny(
                                            NonAnyType::MayBeString(
                                                MayBeNull {
                                                    type_: StringType::DOM(term!(DOMString)),
                                                    q_mark: None
                                                }
                                            )
                                        ))
                                    ),
                                    ellipsis: None
                                }),
                            },
                        ],
                        separator: term!(,),
                    }
                },
                close_brace: term!(CloseBrace),
            },
        })
    }
}
