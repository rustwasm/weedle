use literal::*;
use Parse;

/// Parses a include statement
///
/// ### Grammar
/// ```
/// IncludesStatement ::
///     **identifier** includes **identifier** ;
/// ```
///
/// [Link to WebIDL](https://heycam.github.io/webidl/#prod-IncludesStatement)
#[derive(Debug, PartialEq)]
pub struct IncludesStatement {
    pub lhs_identifier: Identifier,
    pub includes: term!(includes),
    pub rhs_identifier: Identifier,
    pub semi_colon: term!(;)
}

impl Parse for IncludesStatement {
    named!(parse -> Self, do_parse!(
        lhs_identifier: weedle!(Identifier) >>
        includes: weedle!(term!(includes)) >>
        rhs_identifier: weedle!(Identifier) >>
        semi_colon: weedle!(term!(;)) >>
        (IncludesStatement { lhs_identifier, includes, rhs_identifier, semi_colon })
    ));
}
