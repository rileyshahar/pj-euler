#![feature(test)]

/// Generate a main function which relies on `solve` and a benchmark.
macro_rules! euler_problems {
    ( $($num:pat => $name:ident)* ) => {
        fn main() {
            // really simple CLI arg parser to get the first arg and check it against problems
            let solution = match std::env::args().skip(1).next().unwrap_or_default().as_str() {
                $(
                    $num | stringify!($name) => $name::solve(),
                )*
                unknown => panic!("no matching problem: {}", unknown)
            };
            println!("{}", solution);
        }

        #[cfg(test)]
        mod benches {
            extern crate test;
            use test::Bencher;

            $(
                #[bench]
                fn $name(b: &mut Bencher) {
                    b.iter(|| $name::solve());
                }
            )*
        }
    };
}

euler_problems! {
    "1" => divide_three_five
    "2" => fib_even
}
