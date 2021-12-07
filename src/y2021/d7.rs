use itertools::{Itertools, MinMaxResult};

#[aoc_generator(day7)]
fn positions(input: &str) -> Vec<usize> {
    input.split(",").map(|n| n.parse().unwrap()).collect()
}

fn minimize_fuel_consumption(input: &[usize], consumption: fn(usize) -> usize) -> usize {
    let mut widths = input.to_owned();
    if let MinMaxResult::MinMax(&min, &max) = input.iter().minmax() {
        widths.sort_unstable();
        let counts: Vec<(usize, usize)> = widths
            .iter()
            .group_by(|&x| x)
            .into_iter()
            .map(|(&n, group)| (n, group.count()))
            .filter(|&(_, count)| count != 0)
            .collect();
        (min..=max).map(|x| {
            counts.iter().fold(0, |sum, &(n, count)| sum + count * consumption((x as isize - n as isize).abs() as usize))
        }).min().unwrap()
    }
    else {unreachable!()}
}

#[aoc(day7, part1, Lazy)]
fn constant_consumption(input: &[usize]) -> usize {
    minimize_fuel_consumption(input, |n| n)
}

#[aoc(day7, part2, Lazy)]
fn linear_consumption(input: &[usize]) -> usize {
    minimize_fuel_consumption(input, |n| n * (n + 1) / 2)
}

