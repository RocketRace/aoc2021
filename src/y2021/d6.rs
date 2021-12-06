use itertools::Itertools;

#[aoc_generator(day6)]
fn generator(input: &str) -> [usize; 9] {
    let mut timers: Vec<usize> = input.split(",").map(|n| n.parse().unwrap()).collect();
    timers.sort_unstable();
    let counts: Vec<_> = timers
        .iter()
        .group_by(|&&a| a)
        .into_iter()
        .map(|(n, group)| (n, group.count()))
        .collect();
    let mut countdowns = [0usize; LENGTH];
    for (n, count) in counts {
        countdowns[n] = count;
    }
    countdowns
}

const LENGTH: usize = 9;
const BIRTH_INDEX: usize = 6;
const PARENT_INDEX: usize = 8;

fn after_n_days(input: &[usize], days: usize) -> usize {
    let mut countdowns = input.to_owned();
    
    // Basic idea: store a list containing the number of fish with each timer state
    // But instead of shifting the values on each step, simply update an offset
    // variable and update a single index with the birthed fish.
    // 
    // The list is stored in ascending order by the "countdown" state.

    for offset in 0..days + 1 {
        countdowns[(BIRTH_INDEX + offset) % LENGTH] += countdowns[(PARENT_INDEX + offset) % LENGTH];
    }
    countdowns.iter().sum()
}

#[aoc(day6, part1, Arrays)]
fn after_80_days(input: &[usize]) -> usize {
    after_n_days(input, 80)
}

#[aoc(day6, part2, Arrays)]
fn after_256_days(input: &[usize]) -> usize {
    after_n_days(input, 256)
}

fn do_cheese(input: &[usize], magic: [usize; 9]) -> usize {
    (0..9).fold(0, |n, i| n + input[i] * magic[i])
}

#[aoc(day6, part1, Cheese)]
fn cheesy_part1(input: &[usize]) -> usize {
    do_cheese(input, [1421, 1401, 1191, 1154, 1034, 950, 905, 779, 768])
    
}
#[aoc(day6, part2, Cheese)]
fn cheesy_part2(input: &[usize]) -> usize {
    do_cheese(input, [6703087164, 6206821033, 5617089148, 5217223242, 4726100874, 4368232009, 3989468462, 3649885552, 3369186778])
}
