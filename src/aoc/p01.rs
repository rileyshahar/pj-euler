use itertools::Itertools;

/// Solve the problem.
fn solve_for(input: &'static str) -> usize {
    input
        .trim_end() // remove trailing `\n`
        .split('\n')
        .map(str::parse::<usize>)
        .map(|r| r.expect("input is valid usizes"))
        .tuple_windows()
        .map(|(p, c, n)| p + c + n)
        .tuple_windows()
        .fold(0, |t, (p, c)| if c > p { t + 1 } else { t })
}

super::example!(5, "01");
super::problem!(usize, "01");
