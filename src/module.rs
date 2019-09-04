use crate::argument::ArgumentList;
use crate::attribute::ExtendedAttributeList;
use crate::common::{Identifier, Parenthesized};
use crate::types::{AttributedType, ReturnType};

/// Parses module members declaration
pub type ModuleMembers<'a> = Vec<ModuleMember<'a>>;

ast_types! {
    /// Parses module member declaration
    enum ModuleMember<'a> {
        /// Parses a definition
        Definition(struct DefinitionModuleMember<'a> {
            definition: crate::Definition<'a>,
        }),
        /// Parses `[attributes]? returntype identifier? (( args ));`
        ///
        /// (( )) means ( ) chars
        Operation(struct OperationModuleMember<'a> {
            attributes: Option<ExtendedAttributeList<'a>>,
            return_type: ReturnType<'a>,
            identifier: Option<Identifier<'a>>,
            args: Parenthesized<ArgumentList<'a>>,
            semi_colon: term!(;),
        }),
        /// Parses `[attribute]? readonly attributetype type identifier;`
        Attribute(struct AttributeModuleMember<'a> {
            attributes: Option<ExtendedAttributeList<'a>>,
            readonly: term!(readonly),
            attribute: term!(attribute),
            type_: AttributedType<'a>,
            identifier: Identifier<'a>,
            semi_colon: term!(;),
        }),
    }
}
