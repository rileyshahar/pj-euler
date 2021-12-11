/// Solve the problem.
fn solve_for(input: &'static str) -> u32 {
    let initial_positions = input
        .trim_end() // remove trailing `\n`
        .split(',')
        .map(str::parse::<u32>)
        .map(|r| r.expect("input is valid u32s"))
        .collect::<Vec<_>>();

    let &min = initial_positions.iter().min().expect("numbers in input");
    let &max = initial_positions.iter().max().expect("numbers in input");

    (min..=max)
        .map(|n| {
            initial_positions
                .iter()
                .map(|&m| {
                    // using fact that sum of first n naturals is n(n+1)/2
                    if n > m {
                        (n - m) * (n - m + 1) / 2
                    } else {
                        (m - n) * (m - n + 1) / 2
                    }
                })
                .sum()
        })
        .min()
        .expect("numbers in input")
}

super::example!(168, "07");
super::problem!(u32, "07");
