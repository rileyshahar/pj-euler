/// Read the example input from `resources/example.txt`.
#[macro_export]
macro_rules! example_input {
    () => {
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/aoc/resources/example",
            module_path!(),
            ".txt"
        ))
    };
}

/// Read the example input from `resources/input.txt`.
#[macro_export]
macro_rules! problem_input {
    () => {
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/aoc/resources/input",
            module_path!(),
            ".txt"
        ))
    };
}

/// Declare the expected output of the problem.
///
/// Generates a test for the `solve` function to enable easier refactoring once an initial solution
/// is reached.
macro_rules! problem {
    ($ty:ty) => {
        #[must_use]
        #[allow(clippy::missing_const_for_fn)]
        pub fn solve() -> $ty {
            solve_for(problem_input!())
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
    ($output:expr) => {
        #[cfg(test)]
        mod solve_for {
            use super::*;

            #[test]
            fn test() {
                assert_eq!(solve_for(example_input!()), $output);
            }
        }
    };
}

use example;
use problem;
