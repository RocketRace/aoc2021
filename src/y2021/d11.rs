#[aoc_generator(day11)]
fn generator(input: &str) -> Vec<u32> {
    input
        .split("\n")
        .flat_map(|l| l
            .chars()
            .map(|n| n
                .to_digit(10)
                .unwrap())
            .collect::<Vec<_>>())
        .collect()
}

const WIDTH: usize = 10;
const HEIGHT: usize = 10;

fn increment(grid: &mut [u32], i: usize) -> usize {
    // 0 - 9 have not flashed
    // 10 - 18 have been incremented and are about to flash
    // 256 - are marked as flashed
    grid[i] += 1;
    if grid[i] >= 10 && grid[i] < 256 {
        grid[i] = 256;
        let mut flashes = 1;
        let x = i % WIDTH;
        let y = i / WIDTH;
        if x != 0 {
            flashes += increment(grid, i - 1);
            if y != 0 {
                flashes += increment(grid, i - 1 - WIDTH);
            }
            if y != HEIGHT - 1 {
                flashes += increment(grid, i - 1 + WIDTH);
            }
        }
        if x != HEIGHT - 1 {
            flashes += increment(grid, i + 1);
            if y != 0 {
                flashes += increment(grid, i + 1 - WIDTH);
            }
            if y != HEIGHT - 1 {
                flashes += increment(grid, i + 1 + WIDTH);
            }
        }
        if y != 0 {
            flashes += increment(grid, i - WIDTH);
        }
        if y != HEIGHT - 1 {
            flashes += increment(grid, i + WIDTH);
        }
        flashes
    }
    else {
        0
    }
}

fn clean(grid: &mut [u32]) {
    for n in grid.iter_mut() {
        if *n >= 256 {
            *n = 0;
        }
    }
}

fn tick(grid: &mut [u32]) -> usize {
    (0..WIDTH * HEIGHT).fold(0, |s, i| {
        s + increment(grid, i)
    })
}


#[aoc(day11, part1)]
fn simple_loop(input: &[u32]) -> usize {
    let mut grid = input.to_owned();
    (0..100).fold(0, |s, _| {
        let n = s + tick(&mut grid);
        clean(&mut grid);
        n
    })
}

#[aoc(day11, part2)]
fn equally_simple_loop(input: &[u32]) -> usize {
    let mut grid = input.to_owned();
    let mut step = 0;
    loop {
        step += 1;
        if tick(&mut grid) == WIDTH * HEIGHT {
            return step 
        }
        clean(&mut grid);
    }
}
