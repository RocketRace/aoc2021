use std::collections::BTreeSet;

#[aoc_generator(day9)]
fn generator(input: &str) -> Vec<u32> {
    input
        .split("\n")
        .flat_map(|line| line
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<_>>())
        .collect()
}

const WIDTH: usize = 100;
const X: usize = WIDTH - 1;
const HEIGHT: usize = 100;
const Y: usize = HEIGHT - 1;

fn is_local_minimum(data: &[u32], position: usize, value: u32) -> bool {
    let (x, y) = (position % WIDTH, position / WIDTH );
    match (x, y) {
        _p @ (0, 0) => data[position + 1] > value && data[position + WIDTH] > value,
        _p @ (0, Y) => data[position + 1] > value && data[position + WIDTH] > value,
        _p @ (X, 0) => data[position - 1] > value && data[position - WIDTH] > value,
        _p @ (X, Y) => data[position - 1] > value && data[position - WIDTH] > value,
        _p @ (_, 0) => data[position + 1] > value && data[position - 1] > value && data[position + WIDTH] > value,
        _p @ (_, Y) => data[position + 1] > value && data[position - 1] > value && data[position - WIDTH] > value,
        _p @ (0, _) => data[position + 1] > value && data[position + WIDTH] > value && data[position - WIDTH] > value,
        _p @ (X, _) => data[position - 1] > value && data[position + WIDTH] > value && data[position - WIDTH] > value,
        _p @ (_, _) => data[position + 1] > value && data[position - 1] > value && data[position + WIDTH] > value && data[position - WIDTH] > value,
    }
}

#[aoc(day9, part1)]
fn sum_mimima(input: &[u32]) -> u32 {
    input
        .iter()
        .enumerate()
        .filter(|&(pos, &value)| is_local_minimum(input, pos, value))
        .map(|(_, &value)| value + 1)
        .sum()
}

fn basin_size_flood(orig: usize, free: &mut BTreeSet<usize>, position: usize) -> usize {
    if !free.contains(&position) {
        return 0;
    }
    free.remove(&position);
    let (x, y) = (position % WIDTH, position / WIDTH);
    let mut neighbors = vec![];
    if x != 0 {
        neighbors.push(position - 1);
    }
    if x != WIDTH - 1 {
        neighbors.push(position + 1);
    }
    if y != 0 {
        neighbors.push(position - WIDTH);
    }
    if y != HEIGHT - 1 {
        neighbors.push(position + WIDTH);
    }
    let mut sum = 1;
    for pos in neighbors {
        sum += basin_size_flood(orig, free, pos);
    }
    sum
}

#[aoc(day9, part2)]
fn basins(input: &[u32]) -> usize {
    let mut free = BTreeSet::from_iter(
        input.iter().enumerate().filter_map(|(pos, &n)| (n != 9).then_some(pos))
    );
    let mut basin_sizes = vec![];
    while let Some(&pos) = free.last() {
        basin_sizes.push(basin_size_flood(pos, &mut free, pos));
    }
    basin_sizes.sort_unstable();
    let len = basin_sizes.len();
    basin_sizes[len - 1] * basin_sizes[len - 2] * basin_sizes[len - 3] 
}