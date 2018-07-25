use argument::ArgumentList;
use attribute::ExtendedAttributeList;
use common::{Braced, Generics, Identifier};
use literal::ConstValue;
use types::{AttributedType, ConstType, ReturnType};

/// Parses interface members
pub type InterfaceMembers = Vec<InterfaceMember>;

ast_types! {
    /// Parses inheritance clause `: identifier`
    struct Inheritance {
        colon: term!(:),
        identifier: Identifier,
    }

    /// Parses one of the interface member variants
    enum InterfaceMember {
        /// Parses a const interface member `[attributes]? const type identifier = value;`
        Const(struct ConstMember {
            attributes: Option<ExtendedAttributeList>,
            const_: term!(const),
            const_type: ConstType,
            identifier: Identifier,
            assign: term!(=),
            const_value: ConstValue,
            semi_colon: term!(;),
        }),
        /// Parses `[attributes]? (stringifier|inherit|static)? readonly? attribute attributedtype identifier;`
        Attribute(struct AttributeInterfaceMember {
            attributes: Option<ExtendedAttributeList>,
            modifier: Option<StringifierOrInheritOrStatic>,
            readonly: Option<term!(readonly)>,
            attribute: term!(attribute),
            type_: AttributedType,
            identifier: Identifier,
            semi_colon: term!(;),
        }),
        /// Parses `[attributes]? (stringifier|static)? specials? returntype identifier? (( args ));`
        ///
        /// (( )) means ( ) chars
        Operation(struct OperationInterfaceMember {
            attributes: Option<ExtendedAttributeList>,
            modifier: Option<StringifierOrStatic>,
            specials: Vec<Special>,
            return_type: ReturnType,
            identifier: Option<Identifier>,
            args: Braced<ArgumentList>,
            semi_colon: term!(;),
        }),
        /// Parses an iterable declaration `[attributes]? (iterable<attributedtype> | iterable<attributedtype, attributedtype>) ;`
        Iterable(enum IterableInterfaceMember {
            /// Parses an iterable declaration `[attributes]? iterable<attributedtype>;`
            Single(struct SingleTypedIterable {
                attributes: Option<ExtendedAttributeList>,
                iterable: term!(iterable),
                generics: Generics<AttributedType>,
                semi_colon: term!(;),
            }),
            /// Parses an iterable declaration `[attributes]? iterable<attributedtype, attributedtype>;`
            Double(struct DoubleTypedIterable {
                attributes: Option<ExtendedAttributeList>,
                iterable: term!(iterable),
                generics: Generics<(AttributedType, term!(,), AttributedType)>,
                semi_colon: term!(;),
            }),
        }),
        /// Parses an maplike declaration `[attributes]? readonly? maplike<attributedtype, attributedtype>;`
        Maplike(struct MaplikeInterfaceMember {
            attributes: Option<ExtendedAttributeList>,
            readonly: Option<term!(readonly)>,
            maplike: term!(maplike),
            generics: Generics<(AttributedType, term!(,), AttributedType)>,
            semi_colon: term!(;),
        }),
        Setlike(struct SetlikeInterfaceMember {
            attributes: Option<ExtendedAttributeList>,
            readonly: Option<term!(readonly)>,
            setlike: term!(setlike),
            generics: Generics<AttributedType>,
            semi_colon: term!(;),
        }),
        /// Parses `stringifier;`
        Stringifier(struct StringifierMember {
            stringifier: term!(stringifier),
            semi_colon: term!(;),
        }),
    }

    /// Parses one of the special keyword `getter|setter|deleter`
    enum Special {
        Getter(term!(getter)),
        Setter(term!(setter)),
        Deleter(term!(deleter)),
    }

    /// Parses `stringifier|inherit|static`
    enum StringifierOrInheritOrStatic {
        Stringifier(term!(stringifier)),
        Inherit(term!(inherit)),
        Static(term!(static)),
    }

    /// Parses `stringifier|static`
    enum StringifierOrStatic {
        Stringifier(term!(stringifier)),
        Static(term!(static)),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use Parse;

    test!(should_parse_stringifier_member { "stringifier;" =>
        "";
        StringifierMember;
    });

    test!(should_parse_stringifier_or_static { "static" =>
        "";
        StringifierOrStatic;
    });

    test!(should_parse_stringifier_or_inherit_or_static { "inherit" =>
        "";
        StringifierOrInheritOrStatic;
    });

    test!(should_parse_setlike_interface_member { "readonly setlike<long>;" =>
        "";
        SetlikeInterfaceMember;
        attributes.is_none();
        readonly == Some(term!(readonly));
    });

    test!(should_parse_maplike_interface_member { "readonly maplike<long, short>;" =>
        "";
        MaplikeInterfaceMember;
        attributes.is_none();
        readonly == Some(term!(readonly));
    });

    test!(should_parse_attribute_interface_member { "readonly attribute unsigned long width;" =>
        "";
        AttributeInterfaceMember;
        attributes.is_none();
        readonly == Some(term!(readonly));
        identifier.0 == "width";
    });

    test!(should_parse_double_typed_iterable { "iterable<long, long>;" =>
        "";
        DoubleTypedIterable;
        attributes.is_none();
    });

    test!(should_parse_single_typed_iterable { "iterable<long>;" =>
        "";
        SingleTypedIterable;
        attributes.is_none();
    });

    test!(should_parse_operation_interface_member { "void readString(long a, long b);" =>
        "";
        OperationInterfaceMember;
        attributes.is_none();
        modifier.is_none();
        specials.is_empty();
        identifier.is_some();
    });

    test!(should_parse_const_member { "const long name = 5;" =>
        "";
        ConstMember;
        attributes.is_none();
        identifier.0 == "name";
    });
}
