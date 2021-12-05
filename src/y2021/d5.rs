#[derive(Clone, Copy, Debug)]
enum LineKind {
    Horizontal,
    Vertical,
    Diagonal,
    Antidiagonal,
}
#[derive(Debug, Clone)]
struct Line {
    kind: LineKind,
    x: usize,
    y: usize,
    count: usize,
}

fn abs_diff(a: usize, b: usize) -> usize {
    (a as isize - b as isize).abs() as usize
}

#[aoc_generator(day5)]
fn mapping_generator(input: &str) -> Vec<Line> {
    input.split("\n")
        .map(|line| {
            let mut getter = line
                .split(" -> ")
                .flat_map(|half| half.split(","))
                .map(|n| n.parse().unwrap());
            let x0: usize = getter.next().unwrap();
            let y0 = getter.next().unwrap();
            let x1 = getter.next().unwrap();
            let y1 = getter.next().unwrap();
            if x0 == x1 {
                Line {
                    kind: LineKind::Vertical,
                    x: x0.min(x1),
                    y: y0.min(y1),
                    count: abs_diff(y0, y1)
                }
            }
            else if y0 == y1 {
                Line {
                    kind: LineKind::Horizontal,
                    x: x0.min(x1),
                    y: y0.min(y1),
                    count: abs_diff(x0, x1)
                }
            }
            else if abs_diff(x0, x1) == abs_diff(y0, y1) {
                Line {
                    kind: LineKind::Diagonal,
                    x: x0.min(x1),
                    y: y0.min(y1),
                    count: abs_diff(x0, x1)
                }
            }
            else {
                Line {
                    kind: LineKind::Antidiagonal,
                    x: x0.min(x1),
                    y: y0.max(y1), // <- notice
                    count: abs_diff(x0, x1)
                }
            }
        })
        .collect()
}

const WIDTH_BITS: usize = 10;
const WIDTH: usize = 1 << WIDTH_BITS;

#[aoc(day5, part1)]
fn mapped_overlaps(input: &[Line]) -> usize {
    let mut counter = vec![0u8; WIDTH * WIDTH];
    for line in input.iter().filter(|&l| matches!(l.kind, LineKind::Horizontal | LineKind::Vertical)) {
        for dxy in 0..line.count {
            let xy = match line.kind {
                LineKind::Horizontal => {
                    (line.x + dxy, line.y)
                },
                LineKind::Vertical => {
                    (line.x, line.y + dxy)
                },
                _ => unreachable!()
            };
            counter[xy.0 << WIDTH_BITS | xy.1] += 1;
        }
    }
    counter.iter()
        .filter(|&&n| n >= 2)
        .count()
}
#[aoc(day5, part2)]
fn mapped_overlaps_plus_diagonals(input: &[Line]) -> usize {
    let mut counter = vec![0u8; WIDTH * WIDTH];
    for line in input {
        for dxy in 0..line.count {
            let xy = match line.kind {
                LineKind::Horizontal => {
                    (line.x + dxy, line.y)
                },
                LineKind::Vertical => {
                    (line.x, line.y + dxy)
                },
                LineKind::Diagonal => {
                    (line.x + dxy, line.y + dxy)
                },
                LineKind::Antidiagonal => {
                    (line.x + dxy, line.y - dxy)
                }
            };
            counter[xy.0 << WIDTH_BITS | xy.1] += 1;
        }
    }
    counter.iter()
        .filter(|&&n| n >= 2)
        .count()
}