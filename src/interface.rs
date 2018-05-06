use literal::*;
use argument::*;
use common::*;
use Parse;
use types::*;
use attribute::*;

/// Parses inheritance clause `: identifier`
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone)]
pub struct Inheritance {
    pub colon: term!(:),
    pub identifier: Identifier,
}

impl Parse for Inheritance {
    named!(parse -> Self, do_parse!(
        colon: weedle!(term!(:)) >>
        identifier: weedle!(Identifier) >>
        (Inheritance { colon, identifier })
    ));
}

/// Parses interface members
pub type InterfaceMembers = Vec<InterfaceMember>;

/// Parses one of the interface member variants
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone)]
pub enum InterfaceMember {
    Const(ConstMember),
    Attribute(AttributeInterfaceMember),
    Operation(OperationInterfaceMember),
    Iterable(IterableInterfaceMember),
    Maplike(MaplikeInterfaceMember),
    Setlike(SetlikeInterfaceMember),
    Stringifier(StringifierMember),
}

impl Parse for InterfaceMember {
    named!(parse -> Self, alt!(
        weedle!(ConstMember) => {|inner| InterfaceMember::Const(inner)} |
        weedle!(AttributeInterfaceMember) => {|inner| InterfaceMember::Attribute(inner)} |
        weedle!(OperationInterfaceMember) => {|inner| InterfaceMember::Operation(inner)} |
        weedle!(IterableInterfaceMember) => {|inner| InterfaceMember::Iterable(inner)} |
        weedle!(MaplikeInterfaceMember) => {|inner| InterfaceMember::Maplike(inner)} |
        weedle!(SetlikeInterfaceMember) => {|inner| InterfaceMember::Setlike(inner)} |
        weedle!(StringifierMember) => {|inner| InterfaceMember::Stringifier(inner)}
    ));
}

/// Parses a const interface member `[attributes]? const type identifier = value;`
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone)]
pub struct ConstMember {
    pub attributes: Option<ExtendedAttributeList>,
    pub const_: term!(const),
    pub const_type: ConstType,
    pub identifier: Identifier,
    pub assign: term!(=),
    pub const_value: ConstValue,
    pub semi_colon: term!(;)
}

impl Parse for ConstMember {
    named!(parse -> Self, do_parse!(
        attributes: weedle!(Option<ExtendedAttributeList>) >>
        const_: weedle!(term!(const)) >>
        const_type: weedle!(ConstType) >>
        identifier: weedle!(Identifier) >>
        assign: weedle!(term!(=)) >>
        const_value: weedle!(ConstValue) >>
        semi_colon: weedle!(term!(;)) >>
        (ConstMember { attributes, const_, const_type, identifier, assign, const_value, semi_colon })
    ));
}

/// Parses `[attributes]? (stringifier|inherit|static)? readonly? attribute attributedtype identifier;`
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone)]
pub struct AttributeInterfaceMember {
    pub attributes: Option<ExtendedAttributeList>,
    pub modifier: Option<StringifierOrInheritOrStatic>,
    pub readonly: Option<term!(readonly)>,
    pub attribute: term!(attribute),
    pub type_: AttributedType,
    pub identifier: Identifier,
    pub semi_colon: term!(;)
}

impl Parse for AttributeInterfaceMember {
    named!(parse -> Self, do_parse!(
        attributes: weedle!(Option<ExtendedAttributeList>) >>
        modifier: weedle!(Option<StringifierOrInheritOrStatic>) >>
        readonly: weedle!(Option<term!(readonly)>) >>
        attribute: weedle!(term!(attribute)) >>
        type_: weedle!(AttributedType) >>
        identifier: weedle!(Identifier) >>
        semi_colon: weedle!(term!(;)) >>
        (AttributeInterfaceMember { attributes, modifier, readonly, attribute, type_, identifier, semi_colon })
    ));
}

/// Parses `[attributes]? (stringifier|static)? specials? returntype identifier? (( args ));`
///
/// (( )) means ( ) chars
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone)]
pub struct OperationInterfaceMember {
    pub attributes: Option<ExtendedAttributeList>,
    pub modifier: Option<StringifierOrStatic>,
    pub specials: Vec<Special>,
    pub return_type: ReturnType,
    pub identifier: Option<Identifier>,
    pub args: Braced<ArgumentList>,
    pub semi_colon: term!(;)
}

impl Parse for OperationInterfaceMember {
    named!(parse -> Self, do_parse!(
        attributes: weedle!(Option<ExtendedAttributeList>) >>
        modifier: weedle!(Option<StringifierOrStatic>) >>
        specials: weedle!(Vec<Special>) >>
        return_type: weedle!(ReturnType) >>
        identifier: weedle!(Option<Identifier>) >>
        args: weedle!(Braced<ArgumentList>) >>
        semi_colon: weedle!(term!(;)) >>
        (OperationInterfaceMember { attributes, modifier, specials, return_type, identifier, args, semi_colon })
    ));
}

/// Parses one of the special keyword `getter|setter|deleter`
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone)]
pub enum Special {
    Getter(term!(getter)),
    Setter(term!(setter)),
    Deleter(term!(deleter))
}

impl Parse for Special {
    named!(parse -> Self, alt!(
        weedle!(term!(getter)) => {|inner| Special::Getter(inner)} |
        weedle!(term!(setter)) => {|inner| Special::Setter(inner)} |
        weedle!(term!(deleter)) => {|inner| Special::Deleter(inner)}
    ));
}

/// Parses an iterable declaration `[attributes]? (iterable<attributedtype> | iterable<attributedtype, attributedtype>) ;`
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone)]
pub enum IterableInterfaceMember {
    Single(SingleTypedIterable),
    Double(DoubleTypedIterable)
}

impl Parse for IterableInterfaceMember {
    named!(parse -> Self, alt!(
        weedle!(SingleTypedIterable) => {|inner| IterableInterfaceMember::Single(inner)} |
        weedle!(DoubleTypedIterable) => {|inner| IterableInterfaceMember::Double(inner)}
    ));
}

/// Parses an iterable declaration `[attributes]? iterable<attributedtype>;`
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone)]
pub struct SingleTypedIterable {
    pub attributes: Option<ExtendedAttributeList>,
    pub iterable: term!(iterable),
    pub generics: Generics<AttributedType>,
    pub semi_colon: term!(;)
}

impl Parse for SingleTypedIterable {
    named!(parse -> Self, do_parse!(
        attributes: weedle!(Option<ExtendedAttributeList>) >>
        iterable: weedle!(term!(iterable)) >>
        generics: weedle!(Generics<AttributedType>) >>
        semi_colon: weedle!(term!(;)) >>
        (SingleTypedIterable { attributes, iterable, generics, semi_colon })
    ));
}

/// Parses an iterable declaration `[attributes]? iterable<attributedtype, attributedtype>;`
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone)]
pub struct DoubleTypedIterable {
    pub attributes: Option<ExtendedAttributeList>,
    pub iterable: term!(iterable),
    pub generics: Generics<(AttributedType, term!(,), AttributedType)>,
    pub semi_colon: term!(;)
}

impl Parse for DoubleTypedIterable {
    named!(parse -> Self, do_parse!(
        attributes: weedle!(Option<ExtendedAttributeList>) >>
        iterable: weedle!(term!(iterable)) >>
        generics: weedle!(Generics<(AttributedType, term!(,), AttributedType)>) >>
        semi_colon: weedle!(term!(;)) >>
        (DoubleTypedIterable { attributes, iterable, generics, semi_colon })
    ));
}

/// Parses an maplike declaration `[attributes]? readonly? maplike<attributedtype, attributedtype>;`
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone)]
pub struct MaplikeInterfaceMember {
    pub attributes: Option<ExtendedAttributeList>,
    pub readonly: Option<term!(readonly)>,
    pub maplike: term!(maplike),
    pub generics: Generics<(AttributedType, term!(,), AttributedType)>,
    pub semi_colon: term!(;)
}

impl Parse for MaplikeInterfaceMember {
    named!(parse -> Self, do_parse!(
        attributes: weedle!(Option<ExtendedAttributeList>) >>
        readonly: weedle!(Option<term!(readonly)>) >>
        maplike: weedle!(term!(maplike)) >>
        generics: weedle!(Generics<(AttributedType, term!(,), AttributedType)>) >>
        semi_colon: weedle!(term!(;)) >>
        (MaplikeInterfaceMember { attributes, readonly, maplike, generics, semi_colon })
    ));
}

/// Parses an setlike declaration `[attributes]? readonly? setlike<attributedtype>;`
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone)]
pub struct SetlikeInterfaceMember {
    pub attributes: Option<ExtendedAttributeList>,
    pub readonly: Option<term!(readonly)>,
    pub setlike: term!(setlike),
    pub generics: Generics<AttributedType>,
    pub semi_colon: term!(;)
}

impl Parse for SetlikeInterfaceMember {
    named!(parse -> Self, do_parse!(
        attributes: weedle!(Option<ExtendedAttributeList>) >>
        readonly: weedle!(Option<term!(readonly)>) >>
        setlike: weedle!(term!(setlike)) >>
        generics: weedle!(Generics<AttributedType>) >>
        semi_colon: weedle!(term!(;)) >>
        (SetlikeInterfaceMember { attributes, readonly, setlike, generics, semi_colon })
    ));
}

/// Parses `stringifier|inherit|static`
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone)]
pub enum StringifierOrInheritOrStatic {
    Stringifier(term!(stringifier)),
    Inherit(term!(inherit)),
    Static(term!(static))
}

impl Parse for StringifierOrInheritOrStatic {
    named!(parse -> Self, alt!(
        weedle!(term!(stringifier)) => {|inner| StringifierOrInheritOrStatic::Stringifier(inner)} |
        weedle!(term!(inherit)) => {|inner| StringifierOrInheritOrStatic::Inherit(inner)} |
        weedle!(term!(static)) => {|inner| StringifierOrInheritOrStatic::Static(inner)}
    ));
}

/// Parses `stringifier|static`
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone)]
pub enum StringifierOrStatic {
    Stringifier(term!(stringifier)),
    Static(term!(static))
}

impl Parse for StringifierOrStatic {
    named!(parse -> Self, alt!(
        weedle!(term!(stringifier)) => {|inner| StringifierOrStatic::Stringifier(inner)} |
        weedle!(term!(static)) => {|inner| StringifierOrStatic::Static(inner)}
    ));
}

/// Parses `stringifier;`
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone)]
pub struct StringifierMember {
    pub stringifier: term!(stringifier),
    pub semi_colon: term!(;)
}

impl Parse for StringifierMember {
    named!(parse -> Self, do_parse!(
        stringifier: weedle!(term!(stringifier)) >>
        semi_colon: weedle!(term!(;)) >>
        (StringifierMember { stringifier, semi_colon })
    ));
}

#[cfg(test)]
mod test {
    use super::*;

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
        identifier.name == "width";
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
        identifier.name == "name";
    });
}
