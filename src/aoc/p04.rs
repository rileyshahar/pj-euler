use itertools::Itertools;

type Row = [u32; 5];

#[derive(Debug)]
struct Board([Row; 5]);

fn take_five<T>(iter: &mut impl Iterator<Item = T>) -> Option<[T; 5]> {
    Some([
        iter.next()?,
        iter.next()?,
        iter.next()?,
        iter.next()?,
        iter.next()?,
    ])
}

impl Board {
    fn from_iterator(mut iter: impl Iterator<Item = u32>) -> Option<Self> {
        Some(Self([
            take_five(&mut iter)?,
            take_five(&mut iter)?,
            take_five(&mut iter)?,
            take_five(&mut iter)?,
            take_five(&mut iter)?,
        ]))
    }

    fn sum_unmarked(&self, marked: &[u32]) -> u32 {
        self.0
            .iter()
            .flatten()
            .filter(|x| !marked.contains(x))
            .sum()
    }

    /// Get the score.
    ///
    /// # Panics
    /// Panics if `marked` is empty.
    fn score(&self, marked: &[u32]) -> u32 {
        self.sum_unmarked(marked) * marked.last().expect("at least one marked square for bingo")
    }

    /// Check if the board has bingo, returning the score.
    fn bingod(&self, marked: &[u32]) -> Option<u32> {
        for i in 0..5 {
            // check columns
            if (0..5).all(|j| marked.contains(&self.0[i][j])) {
                return Some(self.score(marked));
            }

            // check rows
            if (0..5).all(|j| marked.contains(&self.0[j][i])) {
                return Some(self.score(marked));
            }
        }

        None
    }
}

/// Solve the problem.
fn solve_for(input: &'static str) -> u32 {
    let mut inputs = input
        .trim_end() // remove trailing `\n`
        .split('\n');

    let lottery_numbers = inputs
        .next()
        .unwrap()
        .split(',')
        .map(str::parse::<u32>)
        .map(|r| r.expect("lottery numbers are valid u32s"));

    let chunks = inputs.chunks(6);

    let mut boards: Vec<_> = chunks
        .into_iter()
        .map(|chunk| {
            Board::from_iterator(
                chunk
                    .skip(1) // skip separating line
                    .flat_map(str::split_whitespace)
                    .map(str::parse::<u32>)
                    .map(|r| r.expect("input is valid u32s")),
            )
            .expect("input is well-formed")
        })
        .collect();

    let mut called = vec![];

    for number in lottery_numbers {
        called.push(number);

        if boards.len() > 1 {
            boards.retain(|b| b.bingod(&called).is_none());
        } else if let Some(score) = boards
            .iter()
            .map(|b| b.bingod(&called))
            .next()
            .expect("there is a unique last winner")
        {
            return score;
        }
    }

    unreachable!("the problem guarantees some board eventually wins")
}

super::example!(1924, "04");
super::problem!(u32, "04");
