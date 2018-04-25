use literal::*;
use argument::*;
use common::*;
use Parse;
use types::*;
use attribute::*;

/// Parses a `callback` or `interface` definition. Ex:
/// `callback interface identifier {
///   /* interface_members */
/// };`
///
/// ### Grammar
/// ```other
/// CallbackOrInterfaceOrMixin ::
///     callback CallbackRestOrInterface
///     interface InterfaceOrMixin
/// ```
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-CallbackOrInterfaceOrMixin)
#[derive(Debug, PartialEq)]
pub enum CallbackOrInterfaceOrMixin {
    Callback(CallbackRestOrInterfacePart),
    Interface(InterfaceOrMixinPart),
}

impl Parse for CallbackOrInterfaceOrMixin {
    named!(parse -> Self, alt_complete!(
        weedle!(CallbackRestOrInterfacePart) => {|inner| CallbackOrInterfaceOrMixin::Callback(inner)} |
        weedle!(InterfaceOrMixinPart) => {|inner| CallbackOrInterfaceOrMixin::Interface(inner)}
    ));
}

#[derive(Debug, PartialEq)]
pub struct CallbackRestOrInterfacePart {
    pub callback: term!(callback),
    pub rest: CallbackRestOrInterface,
}

impl Parse for CallbackRestOrInterfacePart {
    named!(parse -> Self, do_parse!(
        callback: weedle!(term!(callback)) >>
        rest: weedle!(CallbackRestOrInterface) >>
        (CallbackRestOrInterfacePart { callback, rest })
    ));
}

#[derive(Debug, PartialEq)]
pub struct InterfaceOrMixinPart {
    pub interface: term!(interface),
    pub rest: InterfaceOrMixin,
}

impl Parse for InterfaceOrMixinPart {
    named!(parse -> Self, do_parse!(
        interface: weedle!(term!(interface)) >>
        rest: weedle!(InterfaceOrMixin) >>
        (InterfaceOrMixinPart { interface, rest })
    ));
}

/// Parses a `callback` definition
///
/// ### Grammar
/// ```other
/// CallbackRestOrInterface ::
///     CallbackRest
///     interface InterfaceRest
/// ```
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-CallbackRestOrInterface)
#[derive(Debug, PartialEq)]
pub enum CallbackRestOrInterface {
    CallbackRest(CallbackRest),
    Interface(InterfaceRestPart),
}

impl Parse for CallbackRestOrInterface {
    named!(parse -> Self, alt_complete!(
        weedle!(CallbackRest) => {|inner| CallbackRestOrInterface::CallbackRest(inner)} |
        weedle!(InterfaceRestPart) => {|inner| CallbackRestOrInterface::Interface(inner)}
    ));
}

#[derive(Debug, PartialEq)]
pub struct InterfaceRestPart {
    pub interface: term!(interface),
    pub rest: InterfaceRest,
}

impl Parse for InterfaceRestPart {
    named!(parse -> Self, do_parse!(
        interface: weedle!(term!(interface)) >>
        rest: weedle!(InterfaceRest) >>
        (InterfaceRestPart { interface, rest })
    ));
}

/// Parses the remaining parts of a `callback` definition
///
/// ### Grammar
/// ```other
/// CallbackRest ::
///     **identifier** = ReturnType ( ArgumentList ) ;
/// ```
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-CallbackRest)
#[derive(Debug, PartialEq)]
pub struct CallbackRest {
    pub identifier: Identifier,
    pub assign: term!(=),
    pub return_type: ReturnType,
    pub braced: Braced<ArgumentList>,
}

impl Parse for CallbackRest {
    named!(parse -> Self, do_parse!(
        identifier: weedle!(Identifier) >>
        assign: weedle!(term!(=)) >>
        return_type: weedle!(ReturnType) >>
        braced: weedle!(Braced<ArgumentList>) >>
        (CallbackRest { identifier, assign, return_type, braced })
    ));
}

/// Parses the remaining parts of a interface definition
///
/// ### Grammar
/// ```other
/// InterfaceRest ::
///     **identifier** Inheritance { InterfaceMembers } ;
/// ```
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-InterfaceRest)
#[derive(Debug, PartialEq)]
pub struct InterfaceRest {
    pub identifier: Identifier,
    pub inheritance: Option<Inheritance>,
    pub parenthesized: Parenthesized<InterfaceMembers>,
    pub semi_colon: term!(;),
}

impl Parse for InterfaceRest {
    named!(parse -> Self, do_parse!(
        identifier: weedle!(Identifier) >>
        inheritance: weedle!(Option<Inheritance>) >>
        parenthesized: weedle!(Parenthesized<InterfaceMembers>) >>
        semi_colon: weedle!(term!(;)) >>
        (InterfaceRest { identifier, inheritance, parenthesized, semi_colon })
    ));
}

/// Parses inheritance definition
///
/// ### Grammar
/// ```other
/// Inheritance ::
///     : **identifier**
///     ε
/// ```
///
/// Since it is optional, Option<Inheritance> be used
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-Inheritance)
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
///
/// ### Grammar
/// ```other
/// InterfaceMembers ::
///     ExtendedAttributeList InterfaceMember InterfaceMembers
///     ε
/// ```
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-InterfaceMembers)
#[derive(Debug, PartialEq)]
pub struct InterfaceMembers {
    pub members: Vec<InterfaceMembersItem>
}

impl Parse for InterfaceMembers {
    named!(parse -> Self, do_parse!(
        members: many0!(weedle!(InterfaceMembersItem)) >>
        (InterfaceMembers { members })
    ));
}

#[derive(Debug, PartialEq)]
pub struct InterfaceMembersItem {
    pub attributes: ExtendedAttributeList,
    pub member: InterfaceMember,
}

impl Parse for InterfaceMembersItem {
    named!(parse -> Self, do_parse!(
        attributes: weedle!(ExtendedAttributeList) >>
        member: weedle!(InterfaceMember) >>
        (InterfaceMembersItem { attributes, member })
    ));
}

/// Parses one of the interface member variants
///
/// ### Grammar
/// ```other
/// InterfaceMember ::
///     Const
///     Operation
///     Stringifier
///     StaticMember
///     Iterable
///     ReadOnlyMember
///     ReadWriteAttribute
///     ReadWriteMaplike
///     ReadWriteSetlike
/// ```
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-InterfaceMember)
#[derive(Debug, PartialEq)]
pub enum InterfaceMember {
    Const(Const),
    Operation(Operation),
    Stringifier(Stringifier),
    StaticMember(StaticMember),
    Iterable(Iterable),
    ReadOnlyMember(ReadOnlyMember),
    ReadWriteAttribute(ReadWriteAttribute),
    ReadWriteMaplike(ReadWriteMaplike),
    ReadWriteSetlike(ReadWriteSetlike),
}

impl Parse for InterfaceMember {
    named!(parse -> Self, alt_complete!(
        weedle!(Const) => {|inner| InterfaceMember::Const(inner)} |
        weedle!(Operation) => {|inner| InterfaceMember::Operation(inner)} |
        weedle!(Stringifier) => {|inner| InterfaceMember::Stringifier(inner)} |
        weedle!(StaticMember) => {|inner| InterfaceMember::StaticMember(inner)} |
        weedle!(Iterable) => {|inner| InterfaceMember::Iterable(inner)} |
        weedle!(ReadOnlyMember) => {|inner| InterfaceMember::ReadOnlyMember(inner)} |
        weedle!(ReadWriteAttribute) => {|inner| InterfaceMember::ReadWriteAttribute(inner)} |
        weedle!(ReadWriteMaplike) => {|inner| InterfaceMember::ReadWriteMaplike(inner)} |
        weedle!(ReadWriteSetlike) => {|inner| InterfaceMember::ReadWriteSetlike(inner)}
    ));
}

/// Parses a `const` statement
///
/// ### Grammar
/// ```other
/// Const ::
///     const ConstType **identifier** = ConstValue ;
/// ```
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-Const)
#[derive(Debug, PartialEq)]
pub struct Const {
    pub const_: term!(const),
    pub const_type: ConstType,
    pub identifier: Identifier,
    pub assign: term!(=),
    pub const_value: ConstValue,
    pub semi_colon: term!(;)
}

impl Parse for Const {
    named!(parse -> Self, do_parse!(
        const_: weedle!(term!(const)) >>
        const_type: weedle!(ConstType) >>
        identifier: weedle!(Identifier) >>
        assign: weedle!(term!(=)) >>
        const_value: weedle!(ConstValue) >>
        semi_colon: weedle!(term!(;)) >>
        (Const { const_, const_type, identifier, assign, const_value, semi_colon })
    ));
}

/// Parses either Regular or Special operation
///
/// ### Grammar
/// ```other
/// Operation ::
///     RegularOperation
///     SpecialOperation
/// ```
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-Operation)
#[derive(Debug, PartialEq)]
pub enum Operation {
    Regular(RegularOperation),
    Special(SpecialOperation)
}

impl Parse for Operation {
    named!(parse -> Self, alt_complete!(
        weedle!(RegularOperation) => {|inner| Operation::Regular(inner)} |
        weedle!(SpecialOperation) => {|inner| Operation::Special(inner)}
    ));
}

/// Parses a regular operation with a return type
///
/// ### Grammar
/// ```other
/// RegularOperation ::
///    ReturnType OperationRest
/// ```
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-RegularOperation)
#[derive(Debug, PartialEq)]
pub struct RegularOperation {
    pub return_type: ReturnType,
    pub rest: OperationRest
}

impl Parse for RegularOperation {
    named!(parse -> Self, do_parse!(
        return_type: weedle!(ReturnType) >>
        rest: weedle!(OperationRest) >>
        (RegularOperation { return_type, rest })
    ));
}

/// Parses rest of the operation definition
///
/// ### Grammar
/// ```other
/// OperationRest ::
///     OptionalIdentifier ( ArgumentList ) ;
///
/// OptionalIdentifier ::
///     identifier
///     ε
/// ```
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-OperationRest)
#[derive(Debug, PartialEq)]
pub struct OperationRest {
    pub identifier: Option<Identifier>,
    pub braced: Braced<ArgumentList>
}

impl Parse for OperationRest {
    named!(parse -> Self, do_parse!(
        identifier: weedle!(Option<Identifier>) >>
        braced: weedle!(Braced<ArgumentList>) >>
        (OperationRest { identifier, braced })
    ));
}

/// Parses special operation
///
/// ### Grammar
/// ```other
/// SpecialOperation ::
///     Special Specials RegularOperation
///
/// Specials ::
///     Special Specials
///     ε
/// ```
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-SpecialOperation)
#[derive(Debug, PartialEq)]
pub struct SpecialOperation {
    pub specials: Vec<Special>,
    pub regular_operation: RegularOperation
}

impl Parse for SpecialOperation {
    named!(parse -> Self, do_parse!(
        specials: many0!(weedle!(Special)) >>
        regular_operation: weedle!(RegularOperation) >>
        (SpecialOperation { specials, regular_operation })
    ));
}

/// Parses one of the special keyword
///
/// ### Grammar
/// ```other
/// Special ::
///     getter
///     setter
///     deleter
/// ```
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-Special)
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

/// Parses the stringifier statement
///
/// ### Grammar
/// ```other
/// Stringifier ::
///     stringifier StringifierRest
/// ```
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-Stringifier)
#[derive(Debug, PartialEq)]
pub struct Stringifier {
    pub stringifier: term!(stringifier),
    pub rest: StringifierRest
}

impl Parse for Stringifier {
    named!(parse -> Self, do_parse!(
        stringifier: weedle!(term!(stringifier)) >>
        rest: weedle!(StringifierRest) >>
        (Stringifier { stringifier, rest })
    ));
}

/// Parses the remaining part of the stringifier statement
///
/// ### Grammar
/// ```other
/// StringifierRest ::
///     ReadOnly AttributeRest
///     RegularOperation
///     ;
///
/// ReadOnly ::
///     readonly
///     ε
/// ```
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-StringifierRest)
#[derive(Debug, PartialEq)]
pub enum StringifierRest {
    ReadOnly(ReadOnlyAttributeRest),
    RegularOperation(RegularOperation),
    SemiColon(term!(;))
}

impl Parse for StringifierRest {
    named!(parse -> Self, alt_complete!(
        weedle!(ReadOnlyAttributeRest) => {|inner| StringifierRest::ReadOnly(inner)} |
        weedle!(RegularOperation) => {|inner| StringifierRest::RegularOperation(inner)} |
        weedle!(term!(;)) => {|inner| StringifierRest::SemiColon(inner)}
    ));
}

#[derive(Debug, PartialEq)]
pub struct ReadOnlyAttributeRest {
    pub readonly: Option<term!(readonly)>,
    pub rest: AttributeRest
}

impl Parse for ReadOnlyAttributeRest {
    named!(parse -> Self, do_parse!(
        readonly: weedle!(Option<term!(readonly)>) >>
        rest: weedle!(AttributeRest) >>
        (ReadOnlyAttributeRest { readonly, rest })
    ));
}

/// Parses a `static` member definition
///
/// ### Grammar
/// ```other
/// StaticMember ::
///     static StaticMemberRest
/// ```
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-StaticMember)
#[derive(Debug, PartialEq)]
pub struct StaticMember {
    pub static_: term!(static),
    pub rest: StaticMemberRest
}

impl Parse for StaticMember {
    named!(parse -> Self, do_parse!(
        static_: weedle!(term!(static)) >>
        rest: weedle!(StaticMemberRest) >>
        (StaticMember { static_, rest })
    ));
}

/// Parses the remaining part of the static member definition
///
/// ### Grammar
/// ```other
/// StaticMemberRest ::
///     ReadOnly AttributeRest
///     RegularOperation
/// ```
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-StaticMemberRest)
#[derive(Debug, PartialEq)]
pub enum StaticMemberRest {
    ReadOnly(ReadOnlyAttributeRest),
    RegularOperation(RegularOperation)
}

impl Parse for StaticMemberRest {
    named!(parse -> Self, alt_complete!(
        weedle!(ReadOnlyAttributeRest) => {|inner| StaticMemberRest::ReadOnly(inner)} |
        weedle!(RegularOperation) => {|inner| StaticMemberRest::RegularOperation(inner)}
    ));
}

/// Parses an `iterable` declarations
///
/// ### Grammar
/// ```other
/// Iterable ::
///     iterable < TypeWithExtendedAttributes OptionalType > ;
/// ```
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-Iterable)
#[derive(Debug, PartialEq)]
pub struct Iterable {
    pub iterable: term!(iterable),
    pub generics: Generics<IterableGenericsType>
}

impl Parse for Iterable {
    named!(parse -> Self, do_parse!(
        iterable: weedle!(term!(iterable)) >>
        generics: weedle!(Generics<IterableGenericsType>) >>
        (Iterable { iterable, generics })
    ));
}

#[derive(Debug, PartialEq)]
pub struct IterableGenericsType {
    pub type_: TypeWithExtendedAttributes,
    pub rest: Option<IterableGenericsTypeRest>
}

impl Parse for IterableGenericsType {
    named!(parse -> Self, do_parse!(
        type_: weedle!(TypeWithExtendedAttributes) >>
        rest: weedle!(Option<IterableGenericsTypeRest>) >>
        (IterableGenericsType { type_, rest })
    ));
}

/// Parses the optional half of the generics type
///
/// ### Grammar
/// ```other
/// OptionalType ::
///     , TypeWithExtendedAttributes
///     ε
/// ```
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-OptionalType)
#[derive(Debug, PartialEq)]
pub struct IterableGenericsTypeRest {
    pub comma: term!(,),
    pub type_: TypeWithExtendedAttributes
}

impl Parse for IterableGenericsTypeRest {
    named!(parse -> Self, do_parse!(
        comma: weedle!(term!(,)) >>
        type_: weedle!(TypeWithExtendedAttributes) >>
        (IterableGenericsTypeRest { comma, type_ })
    ));
}

/// Parses a `readonly` declaration
///
/// ### Grammar
/// ```other
/// ReadOnlyMember ::
///     readonly ReadOnlyMemberRest
/// ```
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-ReadOnlyMember)
#[derive(Debug, PartialEq)]
pub struct ReadOnlyMember {
    pub readonly: term!(readonly),
    pub rest: ReadOnlyMemberRest
}

impl Parse for ReadOnlyMember {
    named!(parse -> Self, do_parse!(
        readonly: weedle!(term!(readonly)) >>
        rest: weedle!(ReadOnlyMemberRest) >>
        (ReadOnlyMember { readonly, rest })
    ));
}

/// Parses the remaining part of the `readonly` declaration
///
/// ### Grammar
/// ```other
/// ReadOnlyMemberRest ::
///     AttributeRest
///     ReadWriteMaplike
///     ReadWriteSetlike
/// ```
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-ReadOnlyMemberRest)
#[derive(Debug, PartialEq)]
pub enum ReadOnlyMemberRest {
    AttributeRest(AttributeRest),
    ReadWriteMaplike(ReadWriteMaplike),
    ReadWriteSetlike(ReadWriteSetlike)
}

impl Parse for ReadOnlyMemberRest {
    named!(parse -> Self, alt_complete!(
        weedle!(AttributeRest) => {|inner| ReadOnlyMemberRest::AttributeRest(inner)} |
        weedle!(ReadWriteMaplike) => {|inner| ReadOnlyMemberRest::ReadWriteMaplike(inner)} |
        weedle!(ReadWriteSetlike) => {|inner| ReadOnlyMemberRest::ReadWriteSetlike(inner)}
    ));
}

/// Parses `maplike` declaration
///
/// ### Grammar
/// ```other
/// ReadWriteMaplike ::
///     MaplikeRest
///
/// MaplikeRest ::
///     maplike < TypeWithExtendedAttributes , TypeWithExtendedAttributes > ;
/// ```
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-ReadWriteMaplike)
#[derive(Debug, PartialEq)]
pub struct ReadWriteMaplike {
    pub maplike: term!(maplike),
    pub generics: Generics<MaplikeGenericsType>,
    pub semi_colon: term!(;)
}

impl Parse for ReadWriteMaplike {
    named!(parse -> Self, do_parse!(
        maplike: weedle!(term!(maplike)) >>
        generics: weedle!(Generics<MaplikeGenericsType>) >>
        semi_colon: weedle!(term!(;)) >>
        (ReadWriteMaplike { maplike, generics, semi_colon })
    ));
}

#[derive(Debug, PartialEq)]
pub struct MaplikeGenericsType {
    pub type_1: TypeWithExtendedAttributes,
    pub comma: term!(,),
    pub type_2: TypeWithExtendedAttributes
}

impl Parse for MaplikeGenericsType {
    named!(parse -> Self, do_parse!(
        type_1: weedle!(TypeWithExtendedAttributes) >>
        comma: weedle!(term!(,)) >>
        type_2: weedle!(TypeWithExtendedAttributes) >>
        (MaplikeGenericsType { type_1, comma, type_2 })
    ));
}

/// Parses `setlike` declaration
///
/// ### Grammar
/// ```other
/// ReadWriteSetlike ::
///     SetlikeRest
///
/// SetlikeRest ::
///     setlike < TypeWithExtendedAttributes > ;
/// ```
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-ReadWriteSetlike)
#[derive(Debug, PartialEq)]
pub struct ReadWriteSetlike {
    pub setlike: term!(setlike),
    pub generics: Generics<TypeWithExtendedAttributes>,
    pub semi_colon: term!(;)
}

impl Parse for ReadWriteSetlike {
    named!(parse -> Self, do_parse!(
        setlike: weedle!(term!(setlike)) >>
        generics: weedle!(Generics<TypeWithExtendedAttributes>) >>
        semi_colon: weedle!(term!(;)) >>
        (ReadWriteSetlike { setlike, generics, semi_colon })
    ));
}

/// Parses a ReadWrite attribute declaration
///
/// ### Grammar
/// ```other
/// ReadWriteAttribute ::
///     inherit ReadOnly AttributeRest
///     AttributeRest
///
/// ReadOnly ::
///     readonly
///     ε
/// ```
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-ReadWriteAttribute)
#[derive(Debug, PartialEq)]
pub enum ReadWriteAttribute {
    Inherit(InheritAttribute),
    AttributeRest(AttributeRest)
}

impl Parse for ReadWriteAttribute {
    named!(parse -> Self, alt_complete!(
        weedle!(InheritAttribute) => {|inner| ReadWriteAttribute::Inherit(inner)} |
        weedle!(AttributeRest) => {|inner| ReadWriteAttribute::AttributeRest(inner)}
    ));
}

#[derive(Debug, PartialEq)]
pub struct InheritAttribute {
    pub inherit: term!(inherit),
    pub readonly: Option<term!(readonly)>,
    pub rest: AttributeRest
}

impl Parse for InheritAttribute {
    named!(parse -> Self, do_parse!(
        inherit: weedle!(term!(inherit)) >>
        readonly: weedle!(Option<term!(readonly)>) >>
        rest: weedle!(AttributeRest) >>
        (InheritAttribute { inherit, readonly, rest })
    ));
}

/// Parses either interface part or a mixin part
/// ### Grammar
/// ```other
/// InterfaceOrMixin ::
///     InterfaceRest
///     MixinRest
/// ```
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-InterfaceOrMixin)
#[derive(Debug, PartialEq)]
pub enum InterfaceOrMixin {
    InterfaceRest(InterfaceRest),
    MixinRest(MixinRest)
}

impl Parse for InterfaceOrMixin {
    named!(parse -> Self, alt_complete!(
        weedle!(InterfaceRest) => {|inner| InterfaceOrMixin::InterfaceRest(inner)} |
        weedle!(MixinRest) => {|inner| InterfaceOrMixin::MixinRest(inner)}
    ));
}

/// Parses the half of a mixin
///
/// ### Grammar
/// ```other
/// MixinRest ::
///     mixin **identifier** { MixinMembers } ;
/// ```
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-MixinRest)
#[derive(Debug, PartialEq)]
pub struct MixinRest {
    pub mixin: term!(mixin),
    pub identifier: Identifier,
    pub parenthesized: Parenthesized<MixinMembers>
}

impl Parse for MixinRest {
    named!(parse -> Self, do_parse!(
        mixin: weedle!(term!(mixin)) >>
        identifier: weedle!(Identifier) >>
        parenthesized: weedle!(Parenthesized<MixinMembers>) >>
        (MixinRest { mixin, identifier, parenthesized })
    ));
}

/// Parses the members declarations of a mixin
///
/// ### Grammar
/// ```other
/// MixinMembers ::
///     ExtendedAttributeList MixinMember MixinMembers
///     ε
/// ```
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-MixinMembers)
#[derive(Debug, PartialEq)]
pub struct MixinMembers {
    members: Vec<MixinMembersItem>
}

impl Parse for MixinMembers {
    named!(parse -> Self, do_parse!(
        members: many0!(weedle!(MixinMembersItem)) >>
        (MixinMembers { members })
    ));
}

#[derive(Debug, PartialEq)]
pub struct MixinMembersItem {
    pub attributes: ExtendedAttributeList,
    pub member: MixinMember
}

impl Parse for MixinMembersItem {
    named!(parse -> Self, do_parse!(
        attributes: weedle!(ExtendedAttributeList) >>
        member: weedle!(MixinMember) >>
        (MixinMembersItem { attributes, member })
    ));
}

/// Parses one of the variants of a `mixin` member
///
/// ### Grammar
/// ```other
/// MixinMember ::
///     Const
///     RegularOperation
///     Stringifier
///     ReadOnly AttributeRest
/// ```
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-MixinMember)
#[derive(Debug, PartialEq)]
pub enum MixinMember {
    Const(Const),
    RegularOperation(RegularOperation),
    Stringifier(Stringifier),
    ReadOnly(ReadOnlyAttributeRest)
}

impl Parse for MixinMember {
    named!(parse -> Self, alt_complete!(
        weedle!(Const) => {|inner| MixinMember::Const(inner)} |
        weedle!(RegularOperation) => {|inner| MixinMember::RegularOperation(inner)} |
        weedle!(Stringifier) => {|inner| MixinMember::Stringifier(inner)} |
        weedle!(ReadOnlyAttributeRest) => {|inner| MixinMember::ReadOnly(inner)}
    ));
}
