#[derive(Debug, Clone, Copy)]
enum Cell {
    // last step that this cell was modified
    No(usize),
    East(usize),
    South(usize),
}

#[aoc_generator(day25)]
fn generator(input: &str) -> Vec<Vec<Cell>> {
    input.lines().map(|line| line
        .as_bytes()
        .into_iter()
        .copied()
        .map(|c| match c {
            b'.' => Cell::No(0),
            b'>' => Cell::East(0),
            b'v' => Cell::South(0),
            _ => unreachable!(),
        })
        .collect::<Vec<_>>()
    )
    .collect()
}

#[aoc(day25, part1)]
fn entry(input: &[Vec<Cell>]) -> usize {
    let mut grid = input.to_vec();
    let mut half_steps = 0;
    let w = grid[0].len();
    let h = grid.len();
    loop {
        half_steps += 1;
        let mut done = true;
        // double pass because it's christmas
        for y in 0..h {
            for x in 0..w {
                match grid[y][x] {
                    Cell::East(s) if s < half_steps => {
                        match grid[y][(x+1)%w] {
                            Cell::No(t) if t < half_steps => {
                                done = false;
                                grid[y][(x+1)%w] = Cell::East(half_steps);
                                grid[y][x] = Cell::No(half_steps);
                            }
                            _ => () // do not move
                        }
                    }
                    _ => () // do not move
                }
            }
        }
        half_steps += 1;
        // double pass because it's christmas
        for x in 0..w {
            for y in 0..h {
                match grid[y][x] {
                    Cell::South(s) if s < half_steps => {
                        match grid[(y+1)%h][x] {
                            Cell::No(t) if t < half_steps => {
                                done = false;
                                grid[(y+1)%h][x] = Cell::South(half_steps);
                                grid[y][x] = Cell::No(half_steps);
                            }
                            _ => {} // do not move
                        }
                    }
                    _ => {} // do not move
                }
            }
        }
        if done {
            break half_steps / 2;
        }
    }
}

