use argument::ArgumentList;
use attribute::ExtendedAttributeList;
use common::{Braced, Identifier};
use types::{AttributedType, ReturnType};

/// Parses namespace members declaration
pub type NamespaceMembers = Vec<NamespaceMember>;

ast_types! {
    /// Parses namespace member declaration
    enum NamespaceMember {
        /// Parses `[attributes]? returntype identifier? (( args ));`
        ///
        /// (( )) means ( ) chars
        Operation(struct OperationNamespaceMember {
            attributes: Option<ExtendedAttributeList>,
            return_type: ReturnType,
            identifier: Option<Identifier>,
            args: Braced<ArgumentList>,
            semi_colon: term!(;),
        }),
        /// Parses `[attribute]? readonly attributetype type identifier;`
        Attribute(struct AttributeNamespaceMember {
            attributes: Option<ExtendedAttributeList>,
            readonly: term!(readonly),
            attribute: term!(attribute),
            type_: AttributedType,
            identifier: Identifier,
            semi_colon: term!(;),
        }),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use Parse;

    test!(should_parse_attribute_namespace_member { "readonly attribute short name;" =>
        "";
        AttributeNamespaceMember;
        attributes.is_none();
        identifier.0 == "name";
    });

    test!(should_parse_operation_namespace_member { "short (long a, long b);" =>
        "";
        OperationNamespaceMember;
        attributes.is_none();
        identifier.is_none();
    });
}
