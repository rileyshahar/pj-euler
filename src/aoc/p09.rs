use std::collections::HashSet;

use itertools::Itertools;

type Grid = Vec<Vec<u32>>;

/// Get values of adjacent elements. Diagonals are not adjacent.
fn neighbors(g: &Grid, x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut out = vec![];
    if x > 0 {
        out.push((x - 1, y));
    }
    if x < g.len() - 1 {
        out.push((x + 1, y));
    }
    if y > 0 {
        out.push((x, y - 1));
    }
    if y < g[x].len() - 1 {
        out.push((x, y + 1));
    }
    out
}

/// Simple flood fill with a vector as a stack.
fn flood_fill(g: &Grid, x: usize, y: usize) -> HashSet<(usize, usize)> {
    let mut stack = vec![(x, y)];
    let mut out = HashSet::default();
    while !stack.is_empty() {
        let curr = stack.pop().unwrap();
        if g[curr.0][curr.1] == 9 {
            continue;
        }
        out.insert(curr);
        for node in neighbors(g, curr.0, curr.1) {
            if !out.contains(&node) {
                stack.push(node);
            }
        }
    }
    out
}

fn is_low_point(g: &Grid, x: usize, y: usize) -> bool {
    neighbors(g, x, y).iter().all(|&(p, q)| g[p][q] > g[x][y])
}

/// Solve the problem.
fn solve_for(input: &'static str) -> usize {
    let grid: Grid = input
        .trim_end() // remove trailing `\n`
        .split('\n')
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10))
                .map(|r| r.expect("input is valid usizes"))
                .collect()
        })
        .collect();

    (0..grid.len())
        .cartesian_product(0..grid[0].len())
        .filter(|&(x, y)| is_low_point(&grid, x, y))
        .map(|(x, y)| flood_fill(&grid, x, y).len())
        .sorted()
        .rev()
        .take(3)
        .product()
}

super::example!(1134, "09");
super::problem!(usize, "09");
