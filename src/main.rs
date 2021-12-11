#![feature(test)]

use pj_euler::{aoc, euler};

fn main() {
    let mut args = std::env::args().skip(1);

    match args.next().unwrap_or_default().as_str() {
        "--euler" | "" => euler(args.next().unwrap_or_default().as_str()),
        "--aoc" => aoc(args.next().unwrap_or_default().as_str()),
        unknown => panic!(
            "please specify --aoc or --euler; {} is not a valid argument",
            unknown
        ),
    }
}

/// Generate a main function which relies on `solve` and a benchmark.
macro_rules! problems {
    ( $($mod:ident: $($num:pat => $name:ident)* );* ) => {
        $(
            fn $mod(s: &str) {
                let solution = match s {
                    $(
                        $num => $mod::$name::solve().to_string(),
                    )*
                    unknown => panic!("no matching problem: {}", unknown)
                };
                println!("{}", solution);
            }
        )*


        #[cfg(test)]
        mod problem_bench {
            $(
                mod $mod {
                    extern crate test;
                    use test::Bencher;
                    $(
                        #[bench]
                        fn $name(b: &mut Bencher) {
                            b.iter(|| pj_euler::$mod::$name::solve());
                        }
                    )*
                }
            )*
        }
    };
}

problems! {
    euler:
    "1" => p001
    "2" => p002
    "3" => p003
    "4" => p004
    "5" => p005
    "6" => p006
    "7" => p007
    "8" => p008
    "9" => p009
    "10" => p010
    "11" => p011
    "12" => p012
    "13" => p013
    "14" => p014
    "15" => p015
    ;
    aoc:
    "1" => p01
    "2" => p02
    "3" => p03
    "4" => p04
    "5" => p05
    "6" => p06
    "7" => p07
    "8" => p08
    "9" => p09
    "10" => p10
}
