use std::collections::HashMap;

use itertools::Itertools;

#[aoc_generator(day8, part1)]
fn what_is_this_even(input: &str) -> Vec<Vec<usize>> {
    input
        .split("\n")
        .map(|line| line
            .split("|")
            .nth(1)
            .unwrap()
            .split(" ")
            .map(|s| s.len())
            .collect()
        ).collect()
}

#[aoc(day8, part1)]
fn part_1_feels_like_a_throwaway(input: &[Vec<usize>]) -> usize {
    input
        .iter()
        .flatten()
        .filter(|&&s| s == 2 || s == 3 || s == 4 || s == 7)
        .count()
}

struct Row {
    hints: [[bool; 7]; 10],
    finals: [[bool; 7]; 4]
}

fn to_bits<T: Iterator<Item=char>>(input: T) -> [bool; 7] {
    let mut field = [false; 7];
    for c in input {
        field[match c {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            _ => unreachable!()
        }] = true;
    }
    field
}

#[aoc_generator(day8, part2)]
fn actual_problem(input: &str) -> Vec<Row> {
    input
        .split("\n")
        .map(|line| {
            let mut iter = line.split(" | ");
            let mut hints = [[false; 7]; 10];
            let first = iter.next().unwrap().split(" ");
            for (i, segments) in first.enumerate() {
                hints[i] = to_bits(segments.chars())
            }
            let mut finals = [[false; 7]; 4];
            let second = iter.next().unwrap().split(" ");
            for (i, segments) in second.enumerate() {
                finals[i] = to_bits(segments.chars())
            }
            Row { hints, finals }
        })
        .collect()
}

fn check_digit(digit: &[bool; 7], permutation: &[u8]) -> Option<u8> {
    let mut permuted = [false; 7];
    for i in 0..7 {
        permuted[i] = digit[permutation[i] as usize];
    }
    match permuted {
        [true, true, true, false, true, true, true] => Some(0),
        [false, false, true, false, false, true, false] => Some(1),
        [true, false, true, true, true, false, true] => Some(2),
        [true, false, true, true, false, true, true] => Some(3),
        [false, true, true, true, false, true, false] => Some(4),
        [true, true, false, true, false, true, true] => Some(5),
        [true, true, false, true, true, true, true] => Some(6),
        [true, false, true, false, false, true, false] => Some(7),
        [true, true, true, true, true, true, true] => Some(8),
        [true, true, true, true, false, true, true] => Some(9),
        _ => None
    }
}

#[aoc(day8, part2, Bad)]
fn bad_brute_force(input: &[Row]) -> usize {
    input
        .iter()
        .map(|row| {
            for perm in (0..7u8).permutations(7) {
                if row.hints.iter().filter_map(|d| check_digit(d, &perm)).unique().count() == 10 {
                    let digits: Vec<_> = row.finals.iter().flat_map(|d| check_digit(d, &perm)).collect();
                    return digits[0] as usize * 1000 + digits[1] as usize * 100 + digits[2] as usize * 10 + digits[3] as usize
                }
            }
            0
        })
        .sum()
}

struct Row2 {
    hints: [u8; 10],
    finals: [u8; 4]
}

impl Row2 {
    fn from_row(row: &Row) -> Self {
        let mut hints = [0; 10];
        for (i, hint) in row.hints.iter().enumerate() {
            for (j, bit) in hint.iter().enumerate() {
                if *bit {
                    hints[i] |= 1 << j
                }
            }
        }
        let mut finals = [0; 4];
        for (i, hint) in row.finals.iter().enumerate() {
            for (j, bit) in hint.iter().enumerate() {
                if *bit {
                    finals[i] |= 1 << j
                }
            }
        }
        Row2 { hints, finals }
    }
}

#[aoc(day8, part2, Good)]
fn cool_boolean_statement(input: &[Row]) -> usize {
    input
        .into_iter()
        .map(|row| Row2::from_row(row))
        .map(|row| {
            // One pass to find knowns-by-count
            let mut permutation: HashMap<u8, u8> = HashMap::new();
            let (mut one, mut four): (u8, u8) = (0, 0);
            for &hint in &row.hints {
                match hint.count_ones() {
                    2 => {
                        permutation.insert(hint, 1);
                        one = hint;
                    },
                    3 => {permutation.insert(hint, 7);},
                    4 => {
                        permutation.insert(hint, 4);
                        four = hint;
                    },
                    7 => {permutation.insert(hint, 8);},
                    _ => ()
                }
            }
            // One pass to find by overlap
            for hint in row.hints {
                match (
                    hint.count_ones(), 
                    (hint & one as u8).count_ones(), 
                    (hint & four as u8).count_ones()
                ) {
                    (2, _, _) => (),
                    (3, _, _) => (),
                    (4, _, _) => (),
                    (7, _, _) => (),
                    (5, 2, _) => {permutation.insert(hint, 3);},
                    (6, 1, _) => {permutation.insert(hint, 6);},
                    (5, 1, 2) => {permutation.insert(hint, 2);},
                    (5, 1, 3) => {permutation.insert(hint, 5);},
                    (6, 2, 3) => {permutation.insert(hint, 0);},
                    (6, 2, 4) => {permutation.insert(hint, 9);},
                    x => println!("{:?}", x)
                }
            }
            permutation[&row.finals[0]] as usize * 1000 +
            permutation[&row.finals[1]] as usize * 100 +
            permutation[&row.finals[2]] as usize * 10 +
            permutation[&row.finals[3]] as usize
        })
        .sum() 
}

