use std::collections::HashMap;

use itertools::Itertools;

type Point = (isize, isize);

#[derive(Debug, Clone)]
struct Image {
    rules: [bool; 512],
    cells: HashMap<Point, bool>,
    // determines whether other cells are interpreted as dead or alive
    universe: bool
}

impl Image {
    fn cell_count(&self) -> usize {
        self.cells.iter().filter(|&(_, &b)| b).count()
    }

    fn delta(&self, (x, y): Point, (dx, dy): Point) -> Point {
        (x + dx, y + dy)
    }

    fn value(&self, pos: Point) -> usize {
        self.cells.get(&pos).map(|&b| b).unwrap_or(self.universe) as usize
    }

    fn neighbors(&self, pos: Point) -> [Point; 9] {
        [
            // observe the S Q U A R E
            self.delta(pos, (-1, -1)),
            self.delta(pos, ( 0, -1)),
            self.delta(pos, ( 1, -1)),
            self.delta(pos, (-1,  0)),
            self.delta(pos, ( 0,  0)),
            self.delta(pos, ( 1,  0)),
            self.delta(pos, (-1,  1)),
            self.delta(pos, ( 0,  1)),
            self.delta(pos, ( 1,  1)),
        ]
    }

    fn fingerprint(&self, pos: Point) -> usize {
        let n = self.neighbors(pos);
        // lesser square
        self.value(n[0]) << 8 | 
        self.value(n[1]) << 7 |
        self.value(n[2]) << 6 |
        self.value(n[3]) << 5 |
        self.value(n[4]) << 4 |
        self.value(n[5]) << 3 |
        self.value(n[6]) << 2 |
        self.value(n[7]) << 1 |
        self.value(n[8]) << 0
    }

    fn tick(&mut self) {
        let new_cells: HashMap<_, _> = self.cells
            .keys()
            .copied()
            .flat_map(|pos | self.neighbors(pos))
            .unique()
            .map(|pos| (pos, self.rules[self.fingerprint(pos)]))
            .collect();

        self.cells.clear();
        self.cells.extend(new_cells);

        self.universe ^= self.rules[0];
    }    
}

#[aoc_generator(day20)]
fn generator(input: &str) -> Image {
    let (algo, img) = input.split_once("\n\n").unwrap();
    let mut rules = [false; 512];
    algo
        .bytes()
        .enumerate()
        .filter_map(|(i, b)| (b == b'#').then(|| i))
        .for_each(|i| rules[i] = true);
    let cells: HashMap<_, _> = img
        .lines()
        .enumerate()
        .flat_map(|(y, line)| line
            .bytes()
            .enumerate()
            .map(|(x, b)| ((x as isize, y as isize), (b == b'#')))
            .collect::<Vec<_>>()
        )
        .collect();
    Image { rules, cells, universe: false }
}

fn emulate_steps(image: &mut Image, steps: usize) -> usize {
    for _ in 0..steps {
        image.tick();
    }
    image.cell_count()
}

// This day is literally a generalized GoL.
// I could parse the 512-bit rule string and pass it directly
// to a program that runs Game-like cellular automata.
#[aoc(day20, part1)]
fn game_of_life(input: &Image) -> usize {
    let mut image = input.to_owned();
    emulate_steps(&mut image, 2)
}

#[aoc(day20, part2)]
fn game_of_life_deluxe(input: &Image) -> usize {
    let mut image = input.to_owned();
    emulate_steps(&mut image, 50)
}
