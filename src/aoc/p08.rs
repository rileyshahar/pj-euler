use std::collections::{BTreeSet, HashMap, HashSet};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Wire {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl Wire {
    fn from_char(c: char) -> Self {
        match c {
            'a' => Self::A,
            'b' => Self::B,
            'c' => Self::C,
            'd' => Self::D,
            'e' => Self::E,
            'f' => Self::F,
            'g' => Self::G,
            _ => panic!("invalid char: {}", c),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Signal(BTreeSet<Wire>);

fn parse_signals<C: FromIterator<Signal>>(v: &str) -> C {
    v.trim()
        .split(' ')
        .map(|s| Signal(s.chars().map(Wire::from_char).collect()))
        .collect()
}

fn solve_line(mut inputs: HashSet<Signal>, outputs: &[Signal]) -> u32 {
    let one = inputs.iter().find(|s| s.0.len() == 2).unwrap().clone();
    inputs.remove(&one);

    let four = inputs.iter().find(|s| s.0.len() == 4).unwrap().clone();
    inputs.remove(&four);

    let seven = inputs.iter().find(|s| s.0.len() == 3).unwrap().clone();
    inputs.remove(&seven);

    let eight = inputs.iter().find(|s| s.0.len() == 7).unwrap().clone();
    inputs.remove(&eight);

    let nine = inputs
        .iter()
        .find(|s| s.0.len() == 6 && s.0.is_superset(&four.0))
        .unwrap()
        .clone();
    inputs.remove(&nine);

    let three = inputs
        .iter()
        .find(|s| s.0.len() == 5 && s.0.is_superset(&one.0))
        .unwrap()
        .clone();
    inputs.remove(&three);

    let zero = inputs
        .iter()
        .find(|s| s.0.len() == 6 && s.0.is_superset(&one.0))
        .unwrap()
        .clone();
    inputs.remove(&zero);

    let six = inputs.iter().find(|s| s.0.len() == 6).unwrap().clone();
    inputs.remove(&six);

    let five = inputs
        .iter()
        .find(|s| s.0.len() == 5 && s.0.is_subset(&six.0))
        .unwrap()
        .clone();
    inputs.remove(&five);

    let two = inputs.iter().find(|s| s.0.len() == 5).unwrap().clone();
    inputs.remove(&two);

    let map = HashMap::from([
        (zero, 0),
        (one, 1),
        (two, 2),
        (three, 3),
        (four, 4),
        (five, 5),
        (six, 6),
        (seven, 7),
        (eight, 8),
        (nine, 9),
    ]);

    outputs
        .iter()
        .map(|s| map.get(s).unwrap())
        .fold(0, |s, n| s * 10 + n)
}

fn solve_for(input: &'static str) -> u32 {
    input
        .trim_end() // remove trailing `\n`
        .split('\n')
        .map(|s| s.split('|'))
        .map(|mut line| -> u32 {
            solve_line(
                parse_signals(line.next().expect("input is well-formed")),
                &parse_signals::<Vec<_>>(line.next().expect("input is well-formed")),
            )
        })
        .sum::<u32>()
}

super::example!(61229, "08");
super::problem!(u32, "08");
