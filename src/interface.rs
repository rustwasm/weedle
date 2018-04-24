use literals::*;
use arguments::*;
use common::*;
use Parse;
use types::*;
use attributes::*;

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

#[derive(Debug, PartialEq)]
pub struct CallbackRestOrInterfacePart {
    pub callback: term!(callback),
    pub rest: CallbackRestOrInterface,
}

#[derive(Debug, PartialEq)]
pub struct InterfaceOrMixinPart {
    pub interface: term!(interface),
    pub rest: InterfaceOrMixin,
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

#[derive(Debug, PartialEq)]
pub struct InterfaceRestPart {
    pub interface: term!(interface),
    pub rest: InterfaceRest,
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

#[derive(Debug, PartialEq)]
pub struct InterfaceMembersItem {
    pub attributes: ExtendedAttributeList,
    pub member: InterfaceMember,
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

/// Parses rest of the operation definition
///
/// ### Grammar
/// ```other
/// OperationRest ::
///     OptionalIdentifier ( ArgumentList ) ;
/// ```
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-OperationRest)
#[derive(Debug, PartialEq)]
pub struct OperationRest {
    pub identifier: Option<Identifier>,
    pub braced: Braced<ArgumentList>
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

#[derive(Debug, PartialEq)]
pub struct ReadOnlyAttributeRest {
    pub readonly: Option<term!(readonly)>,
    pub rest: AttributeRest
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

#[derive(Debug, PartialEq)]
pub struct IterableGenericsType {
    pub type_: TypeWithExtendedAttributes,
    pub rest: Option<IterableGenericsTypeRest>
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

#[derive(Debug, PartialEq)]
pub struct MaplikeGenericsType {
    pub type_1: TypeWithExtendedAttributes,
    pub comma: term!(,),
    pub type_2: TypeWithExtendedAttributes
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

#[derive(Debug, PartialEq)]
pub struct InheritAttribute {
    pub inherit: term!(inherit),
    pub readonly: Option<term!(readonly)>,
    pub rest: AttributeRest
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

#[derive(Debug, PartialEq)]
pub struct MixinMembersItem {
    pub attributes: ExtendedAttributeList,
    pub member: MixinMember
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
