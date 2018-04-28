use Parse;
use common::*;
use argument::*;

/// Parses a list of attributes. Ex: `[ attribute1, attribute2 ]`
pub type ExtendedAttributeList = Bracketed<Punctuated<ExtendedAttribute, term!(,)>>;

/// Parses on of the forms of attribute
#[derive(Debug, PartialEq)]
pub enum ExtendedAttribute {
    NoArgs(ExtendedAttributeNoArgs),
    ArgList(ExtendedAttributeArgList),
    NamedArgList(ExtendedAttributeNamedArgList),
    Ident(ExtendedAttributeIdent),
    IdentList(ExtendedAttributeIdentList),
}

impl Parse for ExtendedAttribute {
    named!(parse -> Self, alt_complete!(
        weedle!(ExtendedAttributeNoArgs) => {|inner| ExtendedAttribute::NoArgs(inner)} |
        weedle!(ExtendedAttributeArgList) => {|inner| ExtendedAttribute::ArgList(inner)} |
        weedle!(ExtendedAttributeNamedArgList) => {|inner| ExtendedAttribute::NamedArgList(inner)} |
        weedle!(ExtendedAttributeIdent) => {|inner| ExtendedAttribute::Ident(inner)} |
        weedle!(ExtendedAttributeIdentList) => {|inner| ExtendedAttribute::IdentList(inner)}
    ));
}

/// Parses a named argument list. Ex: `NamedConstructor=Image(DOMString src)`
#[derive(Debug, PartialEq)]
pub struct ExtendedAttributeNamedArgList {
    pub lhs_identifier: Identifier,
    pub assign: term!(=),
    pub rhs_identifier: Identifier,
    pub args: Braced<ArgumentList>,
}

impl Parse for ExtendedAttributeNamedArgList {
    named!(parse -> Self, do_parse!(
        lhs_identifier: weedle!(Identifier) >>
        assign: weedle!(term!(=)) >>
        rhs_identifier: weedle!(Identifier) >>
        args: weedle!(Braced<ArgumentList>) >>
        (ExtendedAttributeNamedArgList { lhs_identifier, assign, rhs_identifier, args })
    ));
}

/// Parses an identifier list. Ex: `Exposed=(Window,Worker)`
#[derive(Debug, PartialEq)]
pub struct ExtendedAttributeIdentList {
    pub identifier: Identifier,
    pub assign: term!(=),
    pub list: Braced<IdentifierList>
}

impl Parse for ExtendedAttributeIdentList {
    named!(parse -> Self, do_parse!(
        identifier: weedle!(Identifier) >>
        assign: weedle!(term!(=)) >>
        list: weedle!(Braced<IdentifierList>) >>
        (ExtendedAttributeIdentList { identifier, assign, list })
    ));
}

/// Matches comma separated identifier list
pub type IdentifierList = Punctuated<Identifier, term!(,)>;

/// Parses an attribute with an identifier. Ex: `PutForwards=name`
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

/// Parses an argument list. Ex: `Constructor(double x, double y)`
#[derive(Debug, PartialEq)]
pub struct ExtendedAttributeArgList {
    pub identifier: Identifier,
    pub args: Braced<ArgumentList>
}

impl Parse for ExtendedAttributeArgList {
    named!(parse -> Self, do_parse!(
        identifier: weedle!(Identifier) >>
        args: weedle!(Braced<ArgumentList>) >>
        (ExtendedAttributeArgList { identifier, args })
    ));
}

/// Parses a plain attribute. Ex: `Replaceable`
#[derive(Debug, PartialEq)]
pub struct ExtendedAttributeNoArgs {
    pub identifier: Identifier
}

impl Parse for ExtendedAttributeNoArgs {
    named!(parse -> Self, do_parse!(
        identifier: weedle!(Identifier) >>
        (ExtendedAttributeNoArgs { identifier })
    ));
}

#[cfg(test)]
mod test {
    use super::*;
    use types::*;

    test!(should_parse_attribute_no_args { "Replaceable" =>
        "";
        ExtendedAttributeNoArgs {
            identifier => Identifier {
                name: "Replaceable".to_string()
            }
        }
    });

    test!(should_parse_attribute_arg_list { "Constructor(double x, double y)" =>
        "";
        ExtendedAttributeArgList {
            identifier => Identifier {
                name: "Constructor".to_string()
            },
            args => Braced {
                open_brace: term!(OpenBrace),
                body: Punctuated {
                    list: vec![
                        Argument::Single(SingleArgument {
                            attributes: None,
                            optional: None,
                            type_: Type::Single(SingleType::FloatingPoint(MayBeNull {
                                type_: FloatingPointType::Double(DoubleType {
                                    unrestricted: None,
                                    double: term!(double)
                                }),
                                q_mark: None
                            })),
                            identifier: Identifier {
                                name: "x".to_string()
                            },
                            default: None
                        }),
                        Argument::Single(SingleArgument {
                            attributes: None,
                            optional: None,
                            type_: Type::Single(SingleType::FloatingPoint(MayBeNull {
                                type_: FloatingPointType::Double(DoubleType {
                                    unrestricted: None,
                                    double: term!(double)
                                }),
                                q_mark: None
                            })),
                            identifier: Identifier {
                                name: "y".to_string()
                            },
                            default: None
                        }),
                    ],
                    separator: term!(,)
                },
                close_brace: term!(CloseBrace)
            }
        }
    });

    test!(should_parse_attribute_ident { "PutForwards=name" =>
        "";
        ExtendedAttributeIdent {
            lhs_identifier => Identifier {
                name: "PutForwards".to_string()
            },
            assign => term!(=),
            rhs_identifier => Identifier {
                name: "name".to_string()
            }
        }
    });

    test!(should_parse_ident_list { "Exposed=(Window,Worker)" =>
        "";
        ExtendedAttributeIdentList {
            identifier => Identifier {
                name: "Exposed".to_string()
            },
            assign => term!(=),
            list => Braced {
                open_brace: term!(OpenBrace),
                body: Punctuated {
                    list: vec![
                        Identifier {
                            name: "Window".to_string()
                        },
                        Identifier {
                            name: "Worker".to_string()
                        }
                    ],
                    separator: term!(,)
                },
                close_brace: term!(CloseBrace)
            }
        }
    });

    test!(should_parse_named_arg_list { "NamedConstructor=Image(DOMString src)" =>
        "";
        ExtendedAttributeNamedArgList {
            lhs_identifier => Identifier {
                name: "NamedConstructor".to_string()
            },
            assign => term!(=),
            rhs_identifier => Identifier {
                name: "Image".to_string()
            },
            args => Braced {
                open_brace: term!(OpenBrace),
                body: Punctuated {
                    list: vec![
                        Argument::Single(SingleArgument {
                            attributes: None,
                            optional: None,
                            type_: Type::Single(SingleType::DOMString(MayBeNull {
                                type_: term!(DOMString),
                                q_mark: None
                            })),
                            identifier: Identifier {
                                name: "src".to_string()
                            },
                            default: None
                        })
                    ],
                    separator: term!(,)
                },
                close_brace: term!(CloseBrace)
            }
        }
    });
}
