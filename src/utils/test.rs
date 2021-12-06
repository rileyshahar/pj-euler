//! Utilities for testing solutions.

/// Declare the expected output of the problem.
///
/// Generates a test for the `solve` function to enable easier refactoring once an initial solution
/// is reached.
#[macro_export]
macro_rules! problem {
    ($ty:ty: $in:expr) => {
        #[must_use]
        #[allow(clippy::missing_const_for_fn)]
        pub fn solve() -> $ty {
            solve_for($in)
        }
    };

    ($ty:ty: $in:expr => $out:expr) => {
        use crate::utils::test::problem;

        problem!($ty: $in);

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
#[macro_export]
macro_rules! example {
    ( $input:expr => $output:expr ) => {
        #[cfg(test)]
        mod solve_for {
            use super::*;

            #[test]
            fn test() {
                assert_eq!(solve_for($input), $output);
            }
        }
    };
}

/// Generate multiple tests of the `compute` function.
#[macro_export]
macro_rules! examples {
    ( $($name:ident: $input:expr => $output:expr)* ) => {
        $(
            #[cfg(test)]
            mod solve_for {
                use super::*;
                use crate::utils::test::example;

                #[test]
                fn test() {
                    assert_eq!(solve_for($input), $output);
                }
            }
        )*
    }
}

pub use example;
pub use examples;
pub use problem;
