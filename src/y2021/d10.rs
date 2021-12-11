
#[aoc_generator(day10)]
fn generator(input: &str) -> Vec<Vec<char>> {
    input.split("\n").map(|l| l.chars().collect()).collect()
}

#[aoc(day10, part1)]
fn corrupt(input: &[Vec<char>]) -> usize {
    input
        .iter()
        .map(move |chunk| {
            let mut stack = Vec::new();
            for &c in chunk {
                match c {
                    '(' => stack.push(')'),
                    '[' => stack.push(']'),
                    '{' => stack.push('}'),
                    '<' => stack.push('>'),
                    closing => {
                        match stack.pop() {
                            Some(expected) => if expected != closing {
                                return match closing {
                                    ')' => 3,
                                    ']' => 57,
                                    '}' => 1197,
                                    '>' => 25137,
                                    _ => unreachable!()
                                }
                            },
                            None => unreachable!()
                        }
                    }
                }
            }
            0
        })
        .sum()
}
#[aoc(day10, part2)]
fn incomplete(input: &[Vec<char>]) -> usize {
    let mut scores: Vec<_> = input
        .iter()
        .filter_map(move |chunk| {
            let mut stack = Vec::new();
            for &c in chunk {
                match c {
                    '(' => stack.push(')'),
                    '[' => stack.push(']'),
                    '{' => stack.push('}'),
                    '<' => stack.push('>'),
                    closing => {
                        match stack.pop() {
                            Some(expected) => if expected != closing {
                                return None
                            },
                            None => unreachable!()
                        }
                    }
                }
            }
            Some(stack
                .iter()
                .rev()
                .fold(0, |n, &c| n * 5 + match c {
                    ')' => 1,
                    ']' => 2,
                    '}' => 3,
                    '>' => 4,
                    _ => unreachable!()
                })
            )
        })
        .collect();
    scores.sort_unstable();
    dbg!(scores.len());
    scores[scores.len() / 2]
}

