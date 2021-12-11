fn bit_of(i: u32, bit: usize) -> u8 {
    ((i >> bit) & 1).try_into().expect("a bit is 0 or 1")
}

fn most_common_bit(inputs: &[u32], bit: usize) -> u8 {
    let mut count = 0;
    for x in inputs {
        if bit_of(*x, bit) == 1 {
            count += 1;
        } else {
            count -= 1;
        }
    }
    if count >= 0 {
        1
    } else {
        0
    }
}

fn solve_for(input: &'static str) -> u32 {
    let inputs: Vec<_> = input
        .trim_end() // remove trailing `\n`
        .split('\n')
        .map(|s| u32::from_str_radix(s, 2).expect("input is valid binary u32s"))
        .collect();

    let max_length = input
        .split('\n')
        .next()
        .expect("at least one value in input")
        .len();

    let mut oxygen = inputs.clone();
    for bit in (0..max_length).rev() {
        let mcb = most_common_bit(&oxygen, bit);
        oxygen.retain(|n| bit_of(*n, bit) == mcb);
        if oxygen.len() == 1 {
            break;
        }
    }

    let mut co2 = inputs;
    for bit in (0..max_length).rev() {
        let lcb = 1 - most_common_bit(&co2, bit);
        co2.retain(|n| bit_of(*n, bit) == lcb);
        if co2.len() == 1 {
            break;
        }
    }

    // safety: at least one item in each vec by problem guarantees
    oxygen[0] * co2[0]
}

super::example!(230, "03");
super::problem!(u32, "03");
