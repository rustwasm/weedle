use nom::types::CompleteStr;

named!(block_comment<CompleteStr, CompleteStr>, delimited!(
    tag!("/*"),
    take_until!("*/"),
    tag!("*/")
));

named!(line_comment<CompleteStr, CompleteStr>, delimited!(
    tag!("//"),
    take_until!("\n"),
    tag!("\n")
));

named!(comment<CompleteStr, CompleteStr>, alt!(
    block_comment |
    line_comment
));

named!(pub whitespace<CompleteStr, Option<CompleteStr>>, ws!(opt!(comment)));

/// ws! also ignores line & block comments
#[macro_export]
macro_rules! ws {
    ($i:expr, $($args:tt)*) => (
        {
            use $crate::whitespace::whitespace;
            use $crate::nom::Convert;
            use $crate::nom::Err;

            match sep!($i, whitespace, $($args)*) {
                Err(e) => Err(e),
                Ok((i1,o)) => {
                    match (whitespace)(i1) {
                        Err(e) => Err(Err::convert(e)),
                        Ok((i2,_))    => Ok((i2, o))
                    }
                }
            }
        }
      )
}