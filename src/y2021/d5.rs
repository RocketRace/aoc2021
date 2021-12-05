use std::ops::RangeInclusive;

struct Line {
    start: (usize, usize),
    end: (usize, usize)
}

fn abs_range(a: usize, b: usize) -> RangeInclusive<usize> {
    if a <= b { a..=b } else {b..=a }
}

fn range(a: usize, b: usize) -> RangeInclusive<isize> {
    if a <= b { a as isize..=b as isize } else { -(a as isize)..=-(b as isize) }
}

impl Line {
    fn horizontal_range(&self) -> Option<RangeInclusive<usize>> {
        if self.start.1 == self.end.1 {
            Some(abs_range(self.start.0, self.end.0))
        }
        else {None}
    }
    fn vertical_range(&self) -> Option<RangeInclusive<usize>> {
        if self.start.0 == self.end.0 {
            Some(abs_range(self.start.1, self.end.1))
        }
        else {None}
    }
    fn diagonal_range(&self) -> Option<(RangeInclusive<isize>, RangeInclusive<isize>)> {
        if self.start.0 != self.end.0 && self.start.1 != self.end.1 {
            Some((range(self.start.0, self.end.0), range(self.start.1, self.end.1)))
        }
        else {None}
    }
    fn is_nondiagonal(&self) -> bool {
        self.horizontal_range().is_some() || self.vertical_range().is_some()
    }

    fn add_to_map(&self, map: &mut Vec<Vec<usize>>, diagonals: bool) {
        if let Some(r) = self.horizontal_range() {
            for x in r {
                map[self.start.1][x] += 1;
            }
        }
        else if let Some(r) = self.vertical_range() {
            for y in r {
                map[y][self.start.0] += 1;
            }
        }
        else if diagonals {
            if let Some((r0, r1)) = self.diagonal_range() {
                for (x, y) in r0.zip(r1) {
                    map[y.abs() as usize][x.abs() as usize] += 1;
                }
            }
        }
        else {
            unreachable!()
        }
    }
}

#[aoc_generator(day5)]
fn lines(input: &str) -> Vec<Line> {
    input.split("\n")
        .map(|line| line
            .split(" -> ")
            .flat_map(|half| half.split(","))
            .map(|n| n.parse().unwrap())
            .take(4)
            .collect::<Vec<usize>>()
        )
        .map( |ints|
            Line { start: (ints[0], ints[1]), end: (ints[2], ints[3]) }
        )
        .collect()
}

#[aoc(day5, part1)]
fn overlap_points(input: &[Line]) -> usize {
    input.iter()
        .filter(|&l| l.is_nondiagonal())
        .fold(vec![vec![0usize; 1000]; 1000], |mut map, line| {
            line.add_to_map(&mut map, false);
            map
        })
        .iter()
        .flatten()
        .map(|&n| if n >= 2 {1} else {0})
        .sum()
}

#[aoc(day5, part2)]
fn now_with_diagonals(input: &[Line]) -> usize {
    input.iter()
        .fold(vec![vec![0usize; 1000]; 1000], |mut map, line| {
            line.add_to_map(&mut map, true);
            map
        })
        .iter()
        .flatten()
        .map(|&n| if n >= 2 {1} else {0})
        .sum()
}