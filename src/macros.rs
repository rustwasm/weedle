macro_rules! named {
    ($name:ident -> $o:ty, $submac:ident!( $($args:tt)* )) => {
        fn $name(input: $crate::nom::types::CompleteStr) -> $crate::nom::IResult<$crate::nom::types::CompleteStr, $o> {
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

// Return valid option as it is & convert `Error` to `None`
#[macro_export]
macro_rules! opt_flat(
  ($i:expr, $submac:ident!( $($args:tt)* )) => (
    {
      use $crate::nom::Err;

      let i_ = $i.clone();
      match $submac!(i_, $($args)*) {
        Ok((i,o))          => Ok((i, o)),
        Err(Err::Error(_)) => Ok(($i, None)),
        Err(e)             => Err(e),
      }
    }
  );
);

// Pass if condition is true else error out
#[macro_export]
macro_rules! err_if_not(
    ($i:expr, $cond:expr) => (
        {
            use $crate::nom::{Convert,Err,ErrorKind};
            let default_err = Err(Err::convert(Err::Error(error_position!($i, ErrorKind::CondReduce::<u32>))));

            if $cond {
                Ok(($i, ""))
            } else {
                default_err
            }
        }
    );
);

#[cfg(test)]
macro_rules! test {
    (@arg $parsed:ident) => {};
    (@arg $parsed:ident $($lhs:tt).+ == $rhs:expr; $($rest:tt)*) => {
        assert_eq!($parsed.$($lhs).+, $rhs);
        test!(@arg $parsed $($rest)*);
    };
    (@arg $parsed:ident $($lhs:tt).+(); $($rest:tt)*) => {
        assert!($parsed.$($lhs).+());
        test!(@arg $parsed $($rest)*);
    };
    (@arg $parsed:ident $($lhs:tt).+() == $rhs:expr; $($rest:tt)*) => {
        assert_eq!($parsed.$($lhs).+(), $rhs);
        test!(@arg $parsed $($rest)*);
    };
    (err $name:ident { $raw:expr => $typ:ty }) => {
        #[test]
        fn $name() {
            <$typ>::parse($crate::nom::types::CompleteStr($raw)).unwrap_err();
        }
    };
    ($name:ident { $raw:expr => $rem:expr; $typ:ty => $val:expr }) => {
        #[test]
        fn $name() {
            let (rem, parsed) = <$typ>::parse($crate::nom::types::CompleteStr($raw)).unwrap();
            assert_eq!(rem, $crate::nom::types::CompleteStr($rem));
            assert_eq!(parsed, $val);
        }
    };
    ($name:ident { $raw:expr => $rem:expr; $typ:ty; $($body:tt)* }) => {
        #[test]
        fn $name() {
            let (_rem, _parsed) = <$typ>::parse($crate::nom::types::CompleteStr($raw)).unwrap();
            assert_eq!(_rem, $crate::nom::types::CompleteStr($rem));
            test!(@arg _parsed $($body)*);
        }
    };
}

#[macro_export]
macro_rules! test_variants {
    ($struct_:ident { $( $variant:ident == $value:expr ),* $(,)* }) => {
        #[allow(non_snake_case)]
        mod $struct_ {
            $(
                mod $variant {
                    use $crate::types::*;
                    use $crate::nom::types::CompleteStr;
                    #[test]
                    fn should_parse() {
                        let (rem, parsed) = $struct_::parse(CompleteStr($value)).unwrap();
                        assert_eq!(rem, CompleteStr(""));
                        match parsed {
                            $struct_::$variant(_) => {},
                            _ => { panic!("Failed to parse"); }
                        }
                    }
                }
            )*
        }
    };
}
