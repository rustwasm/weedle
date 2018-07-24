use {CompleteStr, IResult};

pub fn sp(input: CompleteStr) -> IResult<CompleteStr, CompleteStr> {
    re_find_static!(
        input,
        r"^(?x:
            [\t\n\r\x20]     # normal whitespace
            |
            //.*             # line comment
            |
            /\*(?:.|\n)*?\*/ # block comment
        )*"
    )
}

/// ws! also ignores line & block comments
macro_rules! ws (
    ($i:expr, $($args:tt)*) => ({
        use $crate::whitespace::sp;
        use $crate::nom::Convert;
        use $crate::nom::Err;
        use $crate::nom::lib::std::result::Result::*;

        match sep!($i, sp, $($args)*) {
            Err(e) => Err(e),
            Ok((i1, o)) => {
                match (sp)(i1) {
                    Err(e) => Err(Err::convert(e)),
                    Ok((i2, _)) => Ok((i2, o))
                }
            }
        }
    });
);
