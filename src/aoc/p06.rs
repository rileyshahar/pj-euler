const CYCLE_LEN: usize = 7;

/// Each index stores the number of lanterfish with that time remaining.
struct LanternfishSchool([u64; CYCLE_LEN + 2]);

impl LanternfishSchool {
    fn update(&mut self) {
        self.0.rotate_left(1);

        // fish with state 0 create ones with state 8, which is handled by the rotation, and go to state 6
        self.0[CYCLE_LEN - 1] += self.0[CYCLE_LEN + 1];
    }

    fn sum(&self) -> u64 {
        self.0.iter().sum()
    }
}

/// Solve the problem.
fn solve_for(input: &'static str) -> u64 {
    let mut lanternfish = LanternfishSchool(
        input
            .split(',')
            .map(str::parse::<usize>)
            .map(|r| r.expect("input is valid usizes"))
            .fold([0; CYCLE_LEN + 2], |mut accum, n| {
                // safety: the input data only includes arguments leq CYCLE_LEN + 1
                accum[n] += 1;
                accum
            }),
    );

    for _ in 0..256 {
        lanternfish.update();
    }

    lanternfish.sum()
}

super::example!(26_984_457_539, "06");
super::problem!(u64, "06");
