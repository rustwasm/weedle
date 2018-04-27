use literal::*;
use argument::*;
use common::*;
use Parse;
use types::*;
use attribute::*;

/// Parses inheritance clause `: identifier`
#[derive(Debug, PartialEq)]
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
#[derive(Debug, PartialEq)]
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

/// Parses a const interface member `/* [attributes] */ const ConstType identifier = ConstValue;`
#[derive(Debug, PartialEq)]
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

/// Parses `/* [attributes] */ /* stringifier|inherit|static */ /* readonly */ attribute type identifier;`
#[derive(Debug, PartialEq)]
pub struct AttributeInterfaceMember {
    pub attributes: Option<ExtendedAttributeList>,
    pub modifier: Option<StringifierOrInheritOrStatic>,
    pub readonly: Option<term!(readonly)>,
    pub attribute: term!(attribute),
    pub type_: Type,
    pub identifier: Identifier,
    pub semi_colon: term!(;)
}

impl Parse for AttributeInterfaceMember {
    named!(parse -> Self, do_parse!(
        attributes: weedle!(Option<ExtendedAttributeList>) >>
        modifier: weedle!(Option<StringifierOrInheritOrStatic>) >>
        readonly: weedle!(Option<term!(readonly)>) >>
        attribute: weedle!(term!(attribute)) >>
        type_: weedle!(Type) >>
        identifier: weedle!(Identifier) >>
        semi_colon: weedle!(term!(;)) >>
        (AttributeInterfaceMember { attributes, modifier, readonly, attribute, type_, identifier, semi_colon })
    ));
}

/// Parses `/* [attributes] */ /* stringifier|static */ /* specials */ returntype /* identifier */ ( args );`
#[derive(Debug, PartialEq)]
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
#[derive(Debug, PartialEq)]
pub enum Special {
    Getter(term!(getter)),
    Setter(term!(setter)),
    Deleter(term!(deleter))
}

impl Parse for Special {
    named!(parse -> Self, alt_complete!(
        weedle!(term!(getter)) => {|inner| Special::Getter(inner)} |
        weedle!(term!(setter)) => {|inner| Special::Setter(inner)} |
        weedle!(term!(deleter)) => {|inner| Special::Deleter(inner)}
    ));
}

/// Parses an iterable declaration `iterable<Type> | iterable<Type, Type> ;`
#[derive(Debug, PartialEq)]
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

/// Parses an iterable declaration `iterable<Type>;`
#[derive(Debug, PartialEq)]
pub struct SingleTypedIterable {
    pub attributes: Option<ExtendedAttributeList>,
    pub iterable: term!(iterable),
    pub generics: Generics<Type>,
    pub semi_colon: term!(;)
}

impl Parse for SingleTypedIterable {
    named!(parse -> Self, do_parse!(
        attributes: weedle!(Option<ExtendedAttributeList>) >>
        iterable: weedle!(term!(iterable)) >>
        generics: weedle!(Generics<Type>) >>
        semi_colon: weedle!(term!(;)) >>
        (SingleTypedIterable { attributes, iterable, generics, semi_colon })
    ));
}

/// Parses an iterable declaration `iterable<Type, Type>;`
#[derive(Debug, PartialEq)]
pub struct DoubleTypedIterable {
    pub attributes: Option<ExtendedAttributeList>,
    pub iterable: term!(iterable),
    pub generics: Generics<(Type, Type)>,
    pub semi_colon: term!(;)
}

impl Parse for DoubleTypedIterable {
    named!(parse -> Self, do_parse!(
        attributes: weedle!(Option<ExtendedAttributeList>) >>
        iterable: weedle!(term!(iterable)) >>
        generics: weedle!(Generics<(Type, Type)>) >>
        semi_colon: weedle!(term!(;)) >>
        (DoubleTypedIterable { attributes, iterable, generics, semi_colon })
    ));
}

/// Parses an maplike declaration `/* readonly */ maplike<Type, Type>;`
#[derive(Debug, PartialEq)]
pub struct MaplikeInterfaceMember {
    pub attributes: Option<ExtendedAttributeList>,
    pub readonly: Option<term!(readonly)>,
    pub maplike: term!(maplike),
    pub generics: Generics<(Type, Type)>,
    pub semi_colon: term!(;)
}

impl Parse for MaplikeInterfaceMember {
    named!(parse -> Self, do_parse!(
        attributes: weedle!(Option<ExtendedAttributeList>) >>
        readonly: weedle!(Option<term!(readonly)>) >>
        maplike: weedle!(term!(maplike)) >>
        generics: weedle!(Generics<(Type, Type)>) >>
        semi_colon: weedle!(term!(;)) >>
        (MaplikeInterfaceMember { attributes, readonly, maplike, generics, semi_colon })
    ));
}

/// Parses an setlike declaration `/* readonly */ setlike<Type>;`
#[derive(Debug, PartialEq)]
pub struct SetlikeInterfaceMember {
    pub attributes: Option<ExtendedAttributeList>,
    pub readonly: Option<term!(readonly)>,
    pub setlike: term!(setlike),
    pub generics: Generics<Type>,
    pub semi_colon: term!(;)
}

impl Parse for SetlikeInterfaceMember {
    named!(parse -> Self, do_parse!(
        attributes: weedle!(Option<ExtendedAttributeList>) >>
        readonly: weedle!(Option<term!(readonly)>) >>
        setlike: weedle!(term!(setlike)) >>
        generics: weedle!(Generics<Type>) >>
        semi_colon: weedle!(term!(;)) >>
        (SetlikeInterfaceMember { attributes, readonly, setlike, generics, semi_colon })
    ));
}

/// Parses `stringifier|inherit|static`
#[derive(Debug, PartialEq)]
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
#[derive(Debug, PartialEq)]
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
#[derive(Debug, PartialEq)]
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
