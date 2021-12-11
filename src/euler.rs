pub mod p001;
pub mod p002;
pub mod p003;
pub mod p004;
pub mod p005;
pub mod p006;
pub mod p007;
pub mod p008;
pub mod p009;
pub mod p010;
pub mod p011;
pub mod p012;
pub mod p013;
pub mod p014;
pub mod p015;

/// Declare the expected output of the problem.
///
/// Generates a test for the `solve` function to enable easier refactoring once an initial solution
/// is reached.
macro_rules! problem {
    ($ty:ty: $in:expr) => {
        #[must_use]
        #[allow(clippy::missing_const_for_fn)]
        pub fn solve() -> $ty {
            solve_for($in)
        }
    };

    ($ty:ty: $in:expr => $out:expr) => {
        use super::problem;

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

use example;
use problem;
