use literals::*;
use terminals::*;
use Parse;
use common::*;
use arguments::*;
use others::*;
use types::*;

/// ExtendedAttributeNamedArgList ::
///     **identifier** = **identifier** ( ArgumentList )
#[derive(Debug, PartialEq)]
pub struct ExtendedAttributeNamedArgList {
    pub lhs_identifier: Identifier,
    pub assign: Assign,
    pub rhs_identifier: Identifier,
    pub args_signature: Braced<ArgumentList>,
}

impl Parse for ExtendedAttributeNamedArgList {
    named!(parse -> Self, do_parse!(
        lhs_identifier: weedle!(Identifier) >>
        assign: weedle!(Assign) >>
        rhs_identifier: weedle!(Identifier) >>
        args_signature: weedle!(Braced<ArgumentList>) >>
        (ExtendedAttributeNamedArgList { lhs_identifier, assign, rhs_identifier, args_signature })
    ));
}

/// ExtendedAttributeList ::
///     [ ExtendedAttribute ExtendedAttributes ]
///     ε
///
/// ExtendedAttributes ::
///     , ExtendedAttribute ExtendedAttributes
///     ε
#[derive(Debug, PartialEq)]
pub struct ExtendedAttributeList {
    pub list: Option<Bracketed<Punctuated<ExtendedAttribute, Comma>>>
}

impl Parse for ExtendedAttributeList {
    named!(parse -> Self, do_parse!(
        list: weedle!(Option<Bracketed<Punctuated<ExtendedAttribute, Comma>>>) >>
        (ExtendedAttributeList { list })
    ));
}

/// ExtendedAttribute ::
///     ( ExtendedAttributeInner ) ExtendedAttributeRest
///     [ ExtendedAttributeInner ] ExtendedAttributeRest
///     { ExtendedAttributeInner } ExtendedAttributeRest
///     Other ExtendedAttributeRest
///
/// ExtendedAttributeRest ::
///     ExtendedAttribute
///     ε
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

/// ExtendedAttributeInner ::
///     ( ExtendedAttributeInner ) ExtendedAttributeInner
///     [ ExtendedAttributeInner ] ExtendedAttributeInner
///     { ExtendedAttributeInner } ExtendedAttributeInner
///     OtherOrComma ExtendedAttributeInner
///     ε
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

/// ExtendedAttributeIdentList ::
///     identifier = ( IdentifierList )
#[derive(Debug, PartialEq)]
pub struct ExtendedAttributeIdentList {
    pub identifier: Identifier,
    pub assign: Assign,
    pub braced: Braced<IdentifierList>
}

impl Parse for ExtendedAttributeIdentList {
    named!(parse -> Self, do_parse!(
        identifier: weedle!(Identifier) >>
        assign: weedle!(Assign) >>
        braced: weedle!(Braced<IdentifierList>) >>
        (ExtendedAttributeIdentList { identifier, assign, braced })
    ));
}

/// IdentifierList ::
///     identifier Identifiers
#[derive(Debug, PartialEq)]
pub struct IdentifierList {
    pub punctuated: Punctuated<Identifier, Comma>
}

impl Parse for IdentifierList {
    named!(parse -> Self, do_parse!(
        punctuated: weedle!(Punctuated<Identifier, Comma>) >>
        (IdentifierList { punctuated })
    ));
}

/// ExtendedAttributeIdent ::
///     identifier = identifier
#[derive(Debug, PartialEq)]
pub struct ExtendedAttributeIdent {
    pub lhs_identifier: Identifier,
    pub assign: Assign,
    pub rhs_identifier: Identifier
}

impl Parse for ExtendedAttributeIdent {
    named!(parse -> Self, do_parse!(
        lhs_identifier: weedle!(Identifier) >>
        assign: weedle!(Assign) >>
        rhs_identifier: weedle!(Identifier) >>
        (ExtendedAttributeIdent { lhs_identifier, assign, rhs_identifier })
    ));
}

/// ExtendedAttributeArgList ::
///     identifier ( ArgumentList )
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

/// ExtendedAttributeNoArgs ::
///     identifier
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

/// AttributeRest ::
///     attribute TypeWithExtendedAttributes AttributeName ;
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-AttributeRest)
#[derive(Debug, PartialEq)]
pub struct AttributeRest {
    pub attribute: Attribute,
    pub type_: TypeWithExtendedAttributes,
    pub name: AttributeName,
    pub semi_colon: SemiColon
}

/// AttributeName ::
///     AttributeNameKeyword
///     identifier
///
/// AttributeNameKeyword ::
///     required
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-AttributeName)
#[derive(Debug, PartialEq)]
pub enum AttributeName {
    Required(Required),
    Identifier(Identifier)
}

#[cfg(test)]
mod test {
    use super::*;
    use nom::types::CompleteStr;
    use types::*;

    #[test]
    fn should_take_named_argument_list() {
        let (rem, parsed) = ExtendedAttributeNamedArgList::parse("NamedConstructor=Image(DOMString src)".into())
            .unwrap();
        assert_eq!(rem, CompleteStr(""));
        assert_eq!(parsed, ExtendedAttributeNamedArgList {
            lhs_identifier: Identifier {
                name: "NamedConstructor".to_string()
            },
            assign: Assign,
            rhs_identifier: Identifier {
                name: "Image".to_string()
            },
            args_signature: Braced {
                open_brace: OpenBrace,
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
                                                    type_: StringType::DOM(DOMString),
                                                    q_mark: None
                                                }
                                            )
                                        ))
                                    ),
                                    ellipsis: None
                                }),
                            },
                        ],
                        separator: Comma,
                    }
                },
                close_brace: CloseBrace,
            },
        })
    }
}
