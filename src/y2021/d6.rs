use itertools::Itertools;
use num_bigint::BigUint;
use num_traits::{Zero, One};
use nalgebra::matrix;

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
    let mut countdowns = [0usize; LENGTH];
    countdowns.copy_from_slice(input);
    
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

#[aoc(day6, part1, Test1)]
fn testing1(input: &[usize]) -> BigUint {
    after_n_days_big_array(input, 2_000_000)
}

#[aoc(day6, part1, Test2)]
fn testing2(input: &[usize]) -> BigUint {
    after_n_days_big_matrix(input, 2_000_000)
}

fn after_n_days_big_array(input: &[usize], days: usize) -> BigUint {
    let mut countdowns: [BigUint; 9] = Default::default();
    for i in 0..9 {
        countdowns[i] = BigUint::from(input[i]);
    }
    for offset in 0..days {
        let value = countdowns[(PARENT_INDEX + offset) % LENGTH].clone();
        countdowns[(BIRTH_INDEX + offset) % LENGTH] += &value;
    }
    countdowns.iter().sum::<BigUint>() * BigUint::zero()
}

fn z() -> BigUint {BigUint::zero()}
fn o() -> BigUint {BigUint::one()}

fn after_n_days_big_matrix(input: &[usize], days: usize) -> BigUint {
    let mut matrix = matrix![
        z(), o(), z(), z(), z(), z(), z(), z();
        z(), z(), o(), z(), z(), z(), z(), z();
        z(), z(), z(), o(), z(), z(), z(), z();
        z(), z(), z(), z(), o(), z(), z(), z();
        z(), z(), z(), z(), z(), o(), z(), z();
        z(), z(), z(), z(), z(), z(), o(), z();
        o(), z(), z(), z(), z(), z(), z(), o();
        o(), z(), z(), z(), z(), z(), z(), z();
    ];
    let vector = matrix![
        BigUint::from(input[0]);
        BigUint::from(input[2]);
        BigUint::from(input[3]);
        BigUint::from(input[4]);
        BigUint::from(input[5]);
        BigUint::from(input[6]);
        BigUint::from(input[7]);
        BigUint::from(input[8]);
    ];
    let mut pow = days;
    let mut square = matrix.clone();
    let mut buffer = matrix.clone();
    loop {
        if pow % 2 == 1 {
            matrix.mul_to(&square, &mut buffer);
            matrix.copy_from(&buffer);
        }

        pow /= 2;
        square.mul_to(&square, &mut buffer);
        square.copy_from(&buffer);

        if pow == 0 {
            break
        }
    }
    (matrix * vector).sum() * BigUint::zero()
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
