#![allow(dead_code)]

#[aoc_generator(day13)]
fn generator(input: &str) -> (Vec<(usize, usize)>, Vec<Line>) {
    let mut iter = input.split("\n\n");
    let points = iter.next().unwrap();
    let folds = iter.next().unwrap();
    (
        points.split("\n").map(|line| {
            let mut iter = line.split(",");
            (iter.next().unwrap().parse().unwrap(), iter.next().unwrap().parse().unwrap())
        }).collect(),
        folds.split("\n").map(|line| {
            let n = line[13..].parse().unwrap();
            match line.chars().nth(11).unwrap() {
                'x' => Line::Vertical(n),
                'y' => Line::Horizontal(n),
                _ => unreachable!()
            }
        }).collect()
    )
}


#[derive(Debug, Clone, Copy)]
enum Line {
    Horizontal(usize),
    Vertical(usize),
}


#[derive(Debug, Clone)]
struct Grid { 
    bits: Vec<bool>,
    width: usize,
    height: usize,
    original_orientation: bool,
}

impl Grid {
    fn from_slice(slice: &[(usize, usize)]) -> Self {
        // unfortunately requires extra passes to figure out the grid size
        let width = 1 + slice.iter().max_by(|x, y| x.0.cmp(&y.0)).unwrap().0;
        let height = 1 + slice.iter().max_by(|x, y| x.1.cmp(&y.1)).unwrap().1;
        let mut inner = vec![false; width * height];
        for &(x, y) in slice {
            inner[y * width + x] = true;
        }
        Grid { bits: inner, width, height, original_orientation: true }
    }

    fn fold_vertical(&mut self, x: usize) {
        self.set_orientation(false);
        self.fold_horizontal_inner(x);
    }

    fn fold_horizontal(&mut self, y: usize) {
        self.set_orientation(true);
        self.fold_horizontal_inner(y);
    }

    fn fold_horizontal_inner(&mut self, y: usize) {
        let bottom: Vec<_> = self.bits.drain(y * self.width..).collect();
        let (top_len, bot_len) = (self.bits.len(), bottom.len());
        for (a, b) in self.bits
            .iter_mut()
            .skip(top_len - bot_len + self.width) // +width for the fold row
            .zip(bottom.chunks(self.width).rev().flatten()) {
            *a |= *b;
        }
        self.height /= 2;
    }

    fn transpose(&mut self) {
        let buf = self.bits.clone();
        // Swap axes
        std::mem::swap(&mut self.height, &mut self.width);
        for (i, bit) in buf.iter().enumerate() {
            let y = i % self.height;
            let x = i / self.height;
            self.bits[y * self.width + x] = *bit;
        }
    }

    fn count_ones(&self) -> usize {
        self.bits.iter().filter(|&&b| b).count()
    }

    fn set_orientation(&mut self, original: bool) {
        if self.original_orientation != original {
            self.transpose();
            self.original_orientation = original;
        }
    }

    fn show(&mut self) {
        self.set_orientation(true);
        for chunk in self.bits.chunks(self.width) {
            for &b in chunk {
                if b {
                    print!("#");
                }
                else {
                    print!(" ");
                }
            }
            println!("");
        }
        println!("");
    }
}

#[aoc(day13, part1)]
fn one_fold((points, folds): &(Vec<(usize, usize)>, Vec<Line>)) -> usize {
    let mut grid = Grid::from_slice(&points);
    match folds[0] {
        Line::Horizontal(y) => {
            grid.fold_horizontal(y);
        },
        Line::Vertical(x) => {
            grid.fold_vertical(x);
        },
    }
    grid.count_ones()
}

struct No;
impl std::fmt::Display for No {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { Ok(()) }
}

#[aoc(day13, part2)]
fn activation_code((points, folds): &(Vec<(usize, usize)>, Vec<Line>)) -> No {
    let mut grid = Grid::from_slice(&points);
    for fold in folds {
        match fold {
            Line::Horizontal(y) => {
                grid.fold_horizontal(*y);
            },
            Line::Vertical(x) => {
                grid.fold_vertical(*x);
            },
        }
    }
    grid.show();
    No
}

