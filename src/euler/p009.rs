//! Find the product of the unique Pythagorean triplet with sum 1000.
fn solve_for(sum: u32) -> u32 {
    let (a, b, c) = (1..=sum)
        .flat_map(|a| ((a + 1)..=((sum - a) / 2)).map(move |b| (a, b)))
        .map(|(a, b)| (a, b, sum - a - b))
        .find(|(a, b, c)| a * a + b * b == c * c)
        .expect("puzzle has a solution");

    a * b * c
}

super::example!(12 => 60);
super::problem!(u32: 1000 => 31_875_000);
