use std::{ops::RangeInclusive, collections::HashSet};

#[aoc_generator(day17)]
fn generator(input: &str) -> (RangeInclusive<isize>, RangeInclusive<isize>) {
    let (x, y) = input[15..].split_once(", y=").unwrap();
    let (xa, xb) = x.split_once("..").unwrap();
    let (ya, yb) = y.split_once("..").unwrap();
    (
        xa.parse().unwrap()..=xb.parse().unwrap(), 
        ya.parse().unwrap()..=yb.parse().unwrap(), 
    )
}

#[aoc(day17, part1)]
fn maximize_height(input: &(RangeInclusive<isize>, RangeInclusive<isize>)) -> isize {
    let maximum_y_drop = input.1.start();

    // maths
    maximum_y_drop * (maximum_y_drop + 1) / 2
}

#[aoc(day17, part2)]
fn part_2(input: &(RangeInclusive<isize>, RangeInclusive<isize>)) -> usize {
    // search x and y components individually for every possible "number of steps"

    let min_x = *input.0.start(); 
    let max_x = *input.0.end();
    let min_y = *input.1.start(); 
    let max_y = *input.1.end();

    let mut velocities = HashSet::new();
    
    // compute upper bound of steps from maximum y drop
    // the largest step count gives the tallest arc
    for steps in 1..=(-min_y + 1) * 2 {
        // solve bounds for yv
        // n / 2 * (yv - n + 1) is in (min_y, max_y)
        // (slightly easier since yv can be negative)

        // 2 / n * m_y + n - 1 ><= yv
        let yv_min = ((2 * min_y + steps * steps - steps) as f64 / 2.0 / steps as f64).ceil() as isize;
        let yv_max = ((2 * max_y + steps * steps - steps) as f64 / 2.0 / steps as f64).floor() as isize;

        // solve bounds for xv
        // simple case: n / 2 * (xv - n + 1) is in (min_x, max_x)
        
        // 2 / n * m_x + n - 1 ><= xv
        let xv_min = ((2 * min_x + steps * steps - steps) as f64 / 2.0 / steps as f64).ceil() as isize;
        let maybe_xv_max = ((2 * max_x + steps * steps - steps) as f64 / 2.0 / steps as f64).floor() as isize;
        
        // Special case when xv falls to zero and stays there (since it can't fall to negatives)
        // (xv * xv + xv) / 2 in minmax
        // Solve quadratic
        let special_xv_min = (-1 + (1.0 + 8.0 * min_x as f64).sqrt().ceil() as isize) / 2;
        let special_xv_max = (-1 + (1.0 + 8.0 * max_x as f64).sqrt().floor() as isize) / 2;
        
        // exclude velocities that overshoot and "curve back left"
        let xv_max = maybe_xv_max.min(2 * max_x / steps);

        let x_range = if xv_min <= xv_max || special_xv_max > steps {
            xv_min..=xv_max
        }
        else {
            special_xv_min..=special_xv_max
        };
        // only unique velocities count
        for xv in x_range {
            for yv in yv_min..=yv_max {
                velocities.insert((xv, yv));
            }
        }
    }
    
    velocities.len()
}

