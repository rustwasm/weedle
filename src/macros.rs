macro_rules! named {
    ($name:ident -> $o:ty, $submac:ident!( $($args:tt)* )) => {
        fn $name(input: &str) -> $crate::nom::IResult<&str, Self> {
            $submac!(input, $($args)*)
        }
    };
}

#[macro_export]
macro_rules! weedle {
    ($i:expr, $t:ty) => {
        <$t as $crate::Parse>::parse($i)
    };
}