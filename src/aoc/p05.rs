use itertools::Itertools;

use std::str::FromStr;

#[derive(Debug, thiserror::Error, PartialEq)]
enum Error {
    #[error("invalid str: {0}")]
    MissingDivisor(&'static str),

    #[error("{0}")]
    NotAnInt(#[from] std::num::ParseIntError),
}

#[derive(Debug, PartialEq)]
struct Line {
    start: (u32, u32),
    end: (u32, u32),
}

impl Line {
    fn normalize(&mut self) {
        if self.end.0 < self.start.0 {
            std::mem::swap(&mut self.end, &mut self.start);
        }
    }

    const fn is_diagonal(&self) -> bool {
        if self.end.1 > self.start.1 {
            self.end.0 - self.start.0 == self.end.1 - self.start.1
        } else {
            self.end.0 - self.start.0 == self.start.1 - self.end.1
        }
    }

    fn covers(&self) -> Vec<(u32, u32)> {
        // start.0 always less than end.0
        let x_range = self.start.0..=self.end.0;

        let y_range = if self.start.1 < self.end.1 {
            self.start.1..=self.end.1
        } else {
            self.end.1..=self.start.1
        };

        if self.start.0 == self.end.0 {
            y_range.map(|n| (self.start.0, n)).collect()
        } else if self.start.1 == self.end.1 {
            x_range.map(|n| (n, self.start.1)).collect()
        } else if self.is_diagonal() {
            if self.start.1 > self.end.1 {
                x_range.zip(y_range.rev()).collect()
            } else {
                x_range.zip(y_range).collect()
            }
        } else {
            vec![]
        }
    }
}

impl FromStr for Line {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        let (start_str, end_str) = s.split_once(" -> ").ok_or(Error::MissingDivisor("->"))?;

        let (sx, sy) = start_str
            .split_once(',')
            .ok_or(Error::MissingDivisor(","))?;
        let start: (u32, u32) = (sx.parse()?, sy.parse()?);

        let (ex, ey) = end_str.split_once(',').ok_or(Error::MissingDivisor(","))?;
        let end: (u32, u32) = (ex.parse()?, ey.parse()?);

        let mut out = Self { start, end };
        out.normalize();
        Ok(out)
    }
}

/// Solve the problem.
fn solve_for(input: &'static str) -> usize {
    input
        .trim_end() // remove trailing `\n`
        .split('\n')
        .map(str::parse::<Line>)
        .map(|r| r.expect("input is valid lines"))
        .flat_map(|l| l.covers().into_iter())
        .duplicates()
        .count()
}

super::example!(12, "05");
super::problem!(usize, "05");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_diagonal() {
        assert!(Line {
            start: (1, 3),
            end: (4, 6)
        }
        .is_diagonal());
        assert!(!Line {
            start: (4, 3),
            end: (4, 6)
        }
        .is_diagonal());
    }

    #[test]
    fn covers() {
        assert_eq!(
            Line {
                start: (0, 8),
                end: (8, 0)
            }
            .covers(),
            vec![
                (0, 8),
                (1, 7),
                (2, 6),
                (3, 5),
                (4, 4),
                (5, 3),
                (6, 2),
                (7, 1),
                (8, 0)
            ]
        );
    }

    #[test]
    fn parse_line() -> Result<(), Error> {
        assert_eq!(
            "3,4 -> 5,6".parse::<Line>()?,
            Line {
                start: (3, 4),
                end: (5, 6)
            }
        );
        Ok(())
    }

    #[test]
    fn line_covers() {
        assert_eq!(
            Line {
                start: (1, 2),
                end: (4, 2)
            }
            .covers(),
            vec![(1, 2), (2, 2), (3, 2), (4, 2)]
        );
    }
}
