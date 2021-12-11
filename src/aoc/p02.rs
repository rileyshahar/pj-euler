use std::str::FromStr;

#[derive(Debug, thiserror::Error, PartialEq)]
enum Error {
    #[error("invalid str: {0}")]
    StepParse(String),

    #[error("invalid direction: {0}")]
    InvalidDirection(String),

    #[error("{0}")]
    NotAnInt(#[from] std::num::ParseIntError),
}

#[derive(Default)]
struct Position {
    depth: u32,
    horizontal: u32,
    aim: u32,
}

impl Position {
    const fn apply(mut self, step: Step) -> Self {
        match step {
            Step::Up(x) => self.aim -= x,
            Step::Down(x) => self.aim += x,
            Step::Forward(x) => {
                self.horizontal += x;
                self.depth += self.aim * x;
            }
        }
        self
    }

    const fn product(&self) -> u32 {
        self.depth * self.horizontal
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Step {
    Up(u32),
    Down(u32),
    Forward(u32),
}

impl FromStr for Step {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, size) = s
            .split_once(' ')
            .ok_or_else(|| Error::StepParse(s.into()))?;
        let n = size.parse()?;
        Ok(match direction {
            "up" => Self::Up(n),
            "down" => Self::Down(n),
            "forward" => Self::Forward(n),
            _ => return Err(Error::InvalidDirection(direction.into())),
        })
    }
}

fn solve_for(input: &'static str) -> u32 {
    input
        .trim_end() // remove trailing `\n`
        .split('\n')
        .map(str::parse::<Step>)
        .map(|r| r.expect("input is valid steps"))
        .fold(Position::default(), Position::apply)
        .product()
}

super::example!(900, "02");
super::problem!(u32, "02");

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn step_parser_works() {
        assert_eq!("up 2".parse(), Ok(Step::Up(2)));
    }
}
