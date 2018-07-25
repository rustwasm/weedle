use attribute::ExtendedAttributeList;
use common::{Default, Identifier, Punctuated};
use types::{AttributedType, Type};

/// Parses a list of argument. Ex: `double v1, double v2, double v3, optional double alpha`
pub type ArgumentList = Punctuated<Argument, term!(,)>;

ast_types! {
    /// Parses an argument. Ex: `double v1|double... v1s`
    enum Argument {
        /// Parses `[attributes]? optional? attributedtype identifier ( = default )?`
        ///
        /// Note: `= default` is only allowed if `optional` is present
        Single(struct SingleArgument {
            attributes: Option<ExtendedAttributeList>,
            optional: Option<term!(optional)>,
            type_: AttributedType,
            identifier: Identifier,
            default: Option<Default> = map!(cond!(optional.is_some(), weedle!(Option<Default>)), |default| default.unwrap_or(None)),
        }),
        /// Parses `[attributes]? type... identifier`
        Variadic(struct VariadicArgument {
            attributes: Option<ExtendedAttributeList>,
            type_: Type,
            ellipsis: term!(...),
            identifier: Identifier,
        }),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use literal::{DecLit, DefaultValue, IntegerLit};
    use Parse;

    test!(should_parse_single_argument { "short a" =>
        "";
        SingleArgument;
        attributes.is_none();
        optional.is_none();
        identifier.0 == "a";
        default.is_none();
    });

    test!(should_parse_variadic_argument { "short... a" =>
        "";
        VariadicArgument;
        attributes.is_none();
        identifier.0 == "a";
    });

    test!(should_parse_optional_single_argument { "optional short a" =>
        "";
        SingleArgument;
        attributes.is_none();
        optional.is_some();
        identifier.0 == "a";
        default.is_none();
    });

    test!(should_parse_optional_single_argument_with_default { "optional short a = 5" =>
        "";
        SingleArgument;
        attributes.is_none();
        optional.is_some();
        identifier.0 == "a";
        default == Some(Default {
            assign: term!(=),
            value: DefaultValue::Integer(IntegerLit::Dec(DecLit("5".to_string()))),
        });
    });
}
