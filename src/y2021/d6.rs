use itertools::Itertools;

#[aoc_generator(day6)]
fn generator(input: &str) -> Vec<usize> {
    input.split(",").map(|n| n.parse().unwrap()).collect()
}

const LENGTH: usize = 9;
const BIRTH_INDEX: usize = 6;
const PARENT_INDEX: usize = 8;

fn after_n_days(input: &[usize], days: usize) -> usize {
    let mut timers = input.to_owned();
    timers.sort_unstable();
    let mut countdowns = [0usize; LENGTH];
    let initial_counts: Vec<_> = timers
        .iter()
        .group_by(|&&a| a)
        .into_iter()
        .map(|(n, group)| (n, group.count()))
        .collect();
    for (n, count) in initial_counts {
        countdowns[n] = count;
    }
    
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

#[aoc(day6, part1)]
fn after_80_days(input: &[usize]) -> usize {
    after_n_days(input, 80)
}

#[aoc(day6, part2)]
fn after_256_days(input: &[usize]) -> usize {
    after_n_days(input, 256)
}

