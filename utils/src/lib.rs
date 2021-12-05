//! Contains utilities for Project Euler solutions.

#[macro_export]
/// Declare the expected output of the problem.
///
/// Generates a test for the `solve` method to enable easier refactoring once an initial solution
/// is reached.
macro_rules! euler_expect {
    ($out: expr) => {
        #[cfg(test)]
        mod generated_tests {
            use super::*;

            #[test]
            fn test_example() {
                assert_eq!(solve(), $out);
            }
        }
    };
}
