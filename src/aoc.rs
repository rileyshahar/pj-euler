//! Advent of code problems.
pub mod p01;
pub mod p02;
pub mod p03;
pub mod p04;
pub mod p05;
pub mod p06;
pub mod p07;
pub mod p08;
pub mod p09;
pub mod p10;

/// Read the example input from `resources/example.txt`.
#[allow(unused_macros)]
macro_rules! example_input {
    ($file:literal) => {
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/aoc/resources/",
            $file,
            "/example.txt"
        ))
    };
}

/// Read the example input from `resources/input.txt`.
macro_rules! problem_input {
    ($file:literal) => {
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/aoc/resources/",
            $file,
            "/input.txt"
        ))
    };
}

/// Declare the expected output of the problem.
///
/// Generates a test for the `solve` function to enable easier refactoring once an initial solution
/// is reached.
macro_rules! problem {
    ($ty:ty, $file:literal) => {
        use super::problem_input;

        #[must_use]
        #[allow(clippy::missing_const_for_fn)]
        pub fn solve() -> $ty {
            solve_for(problem_input!($file))
        }
    };

    ($ty:ty => $out:expr) => {
        use super::problem;

        problem!($ty);

        #[cfg(test)]
        mod solve {
            use super::*;

            #[test]
            fn test() {
                assert_eq!(solve(), $out);
            }
        }
    };
}

/// Generate a test of the `compute` function.
macro_rules! example {
    ($output:expr, $file:literal) => {
        #[cfg(test)]
        mod example_test {
            use super::super::example_input;
            use super::solve_for;

            #[test]
            fn test() {
                assert_eq!(solve_for(example_input!($file)), $output);
            }
        }
    };
}

use example;
use problem;
use problem_input;

#[cfg(test)]
use example_input;
