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
