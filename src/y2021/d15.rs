use std::collections::BinaryHeap;

struct Grid {
    ravel: Vec<usize>,
    width: usize,
    height: usize,
}

impl Grid {
    fn area(&self) -> usize {
        self.width * self.height
    }

    fn x_width(&self) -> usize {
        self.width * 5
    }
    
    fn x_height(&self) -> usize {
        self.height * 5
    }

    fn x_area(&self) -> usize {
        self.x_width() * self.x_height()
    }

    fn get(&self, index: usize) -> usize {
        self.ravel[index]
    }

    fn x_get(&self, index: usize) -> usize {
        let (x, y) = unkey(index, self.x_width());
        let extra_weight = x / self.width + y / self.width;
        (self.ravel[key((x % self.width, y % self.width), self.width)] + extra_weight - 1) % 9 + 1
    }

    fn adjacent_indices(&self, index: usize) -> Vec<usize> {
        self.adjacent_indices_with(index, self.width, self.height)
    }

    fn adjacent_indices_with(&self, index: usize, width: usize, height: usize) -> Vec<usize> {
        let mut adj = vec![];
        if index % width != 0 {
            adj.push(index - 1);
        }
        if index % self.width != width - 1 {
            adj.push(index + 1);
        }
        if index / width != 0 {
            adj.push(index - width);
        }
        if index / width != height - 1 {
            adj.push(index + width);
        }
        adj
    }

    fn extended_indices(&self, index: usize) -> Vec<usize> {
        self.adjacent_indices_with(index, self.x_width(), self.x_height())
    }
}

#[aoc_generator(day15)]
fn generator(input: &str) -> Grid {
    let mut width = 0;
    let ravel: Vec<_> = input
        .lines()
        .flat_map(|line| {
            width = line.as_bytes().len();
            line.bytes().map(|n| (n - b'0') as usize)
            .collect::<Vec<_>>()
        })
        .collect();
    let height = ravel.len() / width;
    Grid { ravel, width, height }
}

fn key((x, y): (usize, usize), w: usize) -> usize {
    x * w + y
}
fn unkey(k: usize, w: usize) -> (usize, usize) {
    (k / w, k % w)
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Cost {
    value: usize,
    index: usize
}

impl Ord for Cost {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.value.cmp(&self.value).then_with(|| self.index.cmp(&other.index))
    }
}
impl PartialOrd for Cost {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[aoc(day15, part1)]
fn djikstra_pathfinding(input: &Grid) -> usize {
    let start = (0, 0);
    let end = (input.width - 1, input.height - 1);
    let mut values = vec![usize::MAX; input.area()];
    let mut min_heap = BinaryHeap::new();
    values[key(start, input.width)] = 0;
    min_heap.push(Cost { value: 0, index: key(start, input.width) });

    while let Some(Cost { value, index }) = min_heap.pop() {
        if unkey(index, input.width) == end {
            return value;
        }
        else if value > values[index] {
            continue;
        }
        else {
            for adj_index in input.adjacent_indices(index) {
                let possible_path = Cost { value: value + input.get(adj_index), index: adj_index };
                if possible_path.value < values[adj_index] {
                    min_heap.push(possible_path);
                    values[adj_index] = possible_path.value;
                }
            }
        }
    }
    usize::MAX
}

#[aoc(day15, part2)]
fn extended_board(input: &Grid) -> usize {
    let start = (0, 0);
    let end = (input.x_width() - 1, input.x_height() - 1);
    let mut values = vec![usize::MAX; input.x_area()];
    let mut min_heap = BinaryHeap::new();
    values[key(start, input.x_width())] = 0;
    min_heap.push(Cost { value: 0, index: key(start, input.x_width()) });

    while let Some(Cost { value, index }) = min_heap.pop() {
        if unkey(index, input.x_width()) == end {
            return value;
        }
        else if value > values[index] {
            continue;
        }
        else {
            for adj_index in input.extended_indices(index) {
                let possible_path = Cost { value: value + input.x_get(adj_index), index: adj_index };
                if possible_path.value < values[adj_index] {
                    min_heap.push(possible_path);
                    values[adj_index] = possible_path.value;
                }
            }
        }
    }
    usize::MAX
}

