#![feature(test)]

use pj_euler::problems;

/// Generate a main function which relies on `solve` and a benchmark.
macro_rules! euler_problems {
    ( $($num:pat => $name:ident)* ) => {
        fn main() {
            // really simple CLI arg parser to get the first arg and check it against problems
            let solution = match std::env::args().skip(1).next().unwrap_or_default().as_str() {
                $(
                    $num => problems::$name::solve().to_string(),
                )*
                unknown => panic!("no matching problem: {}", unknown)
            };
            println!("{}", solution);
        }

        #[cfg(test)]
        mod problem_benches {
            use super::*;

            extern crate test;
            use test::Bencher;

            $(
                #[bench]
                fn $name(b: &mut Bencher) {
                    b.iter(|| problems::$name::solve());
                }
            )*
        }
    };
}

euler_problems! {
    "1" => p001
    "2" => p002
    "3" => p003
    "4" => p004
    "5" => p005
    "6" => p006
    "7" => p007
    "8" => p008
    "9" => p009
}
