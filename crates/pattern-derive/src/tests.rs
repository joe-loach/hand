extern crate proc_macro2;
extern crate syn;

#[cfg(test)]
macro_rules! test_impl {
    ($name:path { $($i:tt)* } expands to { $($o:tt)* }) => {
        {
            #[allow(dead_code)]
            fn ensure_compiles() {
                #[allow(unused_imports)]
                use cir::structured::*;

                {$($i)*
                $($o)*}
            }

            test_impl!($name { $($i)* } expands to { $($o)* } no_build);
        }
    };
    ($name:path { $($i:tt)* } expands to { $($o:tt)* } no_build) => {
        {
            let expected = stringify!( $($o)* )
                .parse::<$crate::tests::proc_macro2::TokenStream>()
                .expect("output should be a valid TokenStream");

            let i = stringify!( $($i)* );
            let parsed = $crate::tests::syn::parse_str(i).expect(
                concat!("Failed to parse input to ", stringify!($name))
            );
            let res = $name(parsed).unwrap();
            assert_eq!(
                prettyplease::unparse(&syn::parse_file(&res.to_string()).unwrap()),
                prettyplease::unparse(&syn::parse_file(&expected.to_string()).unwrap())
            );
        }
    };
}

macro_rules! test {
    (no_build $test_name:ident { $($i:tt)* } expands to { $($o:tt)* }) => {
        #[test]
        fn $test_name() {
            test_impl!(
                $crate::pattern::derive { $($i)* }
                expands to { $($o)* } no_build
            );
        }
    };
    ($test_name:ident { $($i:tt)* } expands to { $($o:tt)* }) => {
        #[test]
        fn $test_name() {
            test_impl!(
                $crate::pattern::derive { $($i)* }
                expands to { $($o)* }
            );
        }
    };
}

test! {
    struct_unit_unit {
        #[derive(Debug)]
        struct Unit;
    } expands to {
        impl ::matcher::ConstPattern for Unit {
            const PATTERN: &[::matcher::Pattern] = &[];
        }
    }
}

test! {
    struct_vis {
        #[derive(Debug)]
        pub struct Vis;
    } expands to {
        impl ::matcher::ConstPattern for Vis {
            const PATTERN: &[::matcher::Pattern] = &[];
        }
    }
}

test! {
    struct_tuple {
        pub struct Tuple(Condition, Label);
    } expands to {
        impl ::matcher::ConstPattern for Tuple {
            const PATTERN: &[::matcher::Pattern] = &[
                <Condition as ::matcher::PatternToken>::TOKEN,
                <Label as ::matcher::PatternToken>::TOKEN,
            ];
        }
    }
}

test! {
    struct_named {
        pub struct Named {
            cond: Condition,
            rd: Register<D>,
            rn: Register<N>,
            imm12: Number<12>,
        }
    } expands to {
        impl ::matcher::ConstPattern for Named {
            const PATTERN: &[::matcher::Pattern] = &[
                <Condition as ::matcher::PatternToken>::TOKEN,
                <Register<D> as ::matcher::PatternToken>::TOKEN,
                <Register<N> as ::matcher::PatternToken>::TOKEN,
                <Number<12> as ::matcher::PatternToken>::TOKEN,
            ];
        }
    }
}
