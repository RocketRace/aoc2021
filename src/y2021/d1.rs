#[aoc_generator(day1)]
pub fn ints(input: &str) -> Vec<usize> {
    input.split("\n").map(|s| s.parse().unwrap()).collect()
}

fn count_increases(input: &[usize], window: usize) -> usize {
    input.windows(window).filter(|&slice| slice[0] < slice[window - 1]).count()
}

#[aoc(day1, part1)]
pub fn single_depths(input: &[usize]) -> usize {
    count_increases(input, 2)
}

#[aoc(day1, part2)]
pub fn sliding_windows(input: &[usize]) -> usize {
    count_increases(input, 4)
}

