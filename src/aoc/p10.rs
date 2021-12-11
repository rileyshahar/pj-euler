use itertools::Itertools;

use std::str::Chars;

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("invalid opening delimiter: {0}")]
    InvalidOpeningDelim(char),

    #[error("invalid character: got {got}, expected {expected}")]
    UnexpectedChar { got: char, expected: char },
}

/// The Ok value is the last character, then the remaining characters, then the autocomplete score.
type IResult<'a> = Result<(Option<char>, Chars<'a>, u64), Error>;

// Recursive descent parser for a chunk.
//
// Not the cleanest parser ever but it works
fn parse_chunk(mut c: Chars) -> IResult {
    let open = match c.next() {
        None => return Ok((None, c, 0)),
        Some(o) => o,
    };
    let expected = match open {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        ')' | ']' | '}' | '>' => return Ok((Some(open), c, 0)),
        c => return Err(Error::InvalidOpeningDelim(c)),
    };
    let (got, c, score) = parse_chunk(c)?;
    match got {
        Some(e) if e == expected => parse_chunk(c),
        Some(got) => Err(Error::UnexpectedChar { got, expected }),
        None => Ok((None, c, score * 5 + autocomplete_score(expected))),
    }
}

fn autocomplete_score(c: char) -> u64 {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => unreachable!("invalid score"),
    }
}

/// Solve the problem.
fn solve_for(input: &'static str) -> u64 {
    let mut scores = input
        .trim_end() // remove trailing `\n`
        .split('\n')
        .map(|l| parse_chunk(l.chars()))
        .filter_map(|r| {
            if let Ok((_, _, score)) = r {
                Some(score)
            } else {
                None
            }
        })
        .sorted();

    scores.nth(scores.len() / 2).unwrap()
}

super::example!(288_957, "10");
super::problem!(u64, "10");
