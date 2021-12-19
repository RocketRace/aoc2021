use std::collections::{HashSet, HashMap};

use itertools::Itertools;

type Vector = (isize, isize, isize);
type Scanner = HashSet<Vector>;

#[aoc_generator(day19)]
fn generator(input: &str) -> Vec<Scanner> {
    input.split("--- scanner ").map(|chunk| chunk
        .lines()
        .skip(1)
        .flat_map(|n| {
            let maybe = n.split_once(",");
            maybe.map(|(a, rest)| {
                let (b, c) = rest.split_once(",").unwrap();
                (a.parse().unwrap(), b.parse().unwrap(), c.parse().unwrap())
            })
        }).collect::<Scanner>()
    ).collect()
}


fn rotations() -> Vec<fn(Vector) -> Vector> {
    // oh no
    vec![
        |(x, y, z): Vector| ( x,  y,  z),
        |(x, y, z): Vector| ( x,  z,  y), //
        |(x, y, z): Vector| ( y,  x,  z), //
        |(x, y, z): Vector| ( y,  z,  x),
        |(x, y, z): Vector| ( z,  x,  y),
        |(x, y, z): Vector| ( z,  y,  x), //
        |(x, y, z): Vector| (-x, -y,  z),
        |(x, y, z): Vector| (-x, -z,  y), //
        |(x, y, z): Vector| (-y, -x,  z), //
        |(x, y, z): Vector| (-y, -z,  x),
        |(x, y, z): Vector| (-z, -x,  y), //
        |(x, y, z): Vector| (-z, -y,  x),
        |(x, y, z): Vector| ( x, -y, -z),
        |(x, y, z): Vector| ( x, -z, -y), //
        |(x, y, z): Vector| ( y, -x, -z), //
        |(x, y, z): Vector| ( y, -z, -x),
        |(x, y, z): Vector| ( z, -x, -y), //
        |(x, y, z): Vector| ( z, -y, -x),
        |(x, y, z): Vector| (-x,  y, -z),
        |(x, y, z): Vector| (-x,  z, -y), //
        |(x, y, z): Vector| (-y,  x, -z), //
        |(x, y, z): Vector| (-y,  z, -x),
        |(x, y, z): Vector| (-z,  x, -y), //
        |(x, y, z): Vector| (-z,  y, -x),
        |(x, y, z): Vector| (-x, -y, -z), //
        |(x, y, z): Vector| (-x, -z, -y),
        |(x, y, z): Vector| (-y, -x, -z),
        |(x, y, z): Vector| (-y, -z, -x), //
        |(x, y, z): Vector| (-z, -x, -y), //
        |(x, y, z): Vector| (-z, -y, -x),
        |(x, y, z): Vector| (-x,  y,  z), //
        |(x, y, z): Vector| (-x,  z,  y),
        |(x, y, z): Vector| (-y,  x,  z),
        |(x, y, z): Vector| (-y,  z,  x), //
        |(x, y, z): Vector| (-z,  x,  y), //
        |(x, y, z): Vector| (-z,  y,  x),
        |(x, y, z): Vector| ( x, -y,  z), //
        |(x, y, z): Vector| ( x, -z,  y),
        |(x, y, z): Vector| ( y, -x,  z),
        |(x, y, z): Vector| ( y, -z,  x), //
        |(x, y, z): Vector| ( z, -x,  y), //
        |(x, y, z): Vector| ( z, -y,  x),
        |(x, y, z): Vector| ( x,  y, -z), //
        |(x, y, z): Vector| ( x,  z, -y),
        |(x, y, z): Vector| ( y,  x, -z),
        |(x, y, z): Vector| ( y,  z, -x), //
        |(x, y, z): Vector| ( z,  x, -y), //
        |(x, y, z): Vector| ( z,  y, -x),
    ]
}

fn orient(scanner: &Scanner, orientation: fn(Vector) -> Vector) -> Scanner {
    scanner.to_owned().into_iter().map(orientation).collect()
}

fn find_overlapping_pair(scanners: &[Scanner]) -> (usize, usize, Vector, Scanner) {
    for (i, base) in scanners.iter().enumerate() {
        for (j, other) in
            scanners.iter().enumerate().skip(i + 1)
        {
            for rot in rotations() {
                let oriented = orient(&other, rot);
                match check_overlap(base, &oriented) {
                    None => continue,
                    Some(offset) => {
                        let mut union = shift(&oriented, offset);
                        union.extend(base);
                        return (i, j, offset, union);
                    },
                }
            }
        }
    }
    panic!("uh oh")
}

fn shift(scanner: &Scanner, (ox, oy, oz): Vector) -> Scanner {
    scanner
        .iter()
        .map(|(x, y, z)| (x + ox, y + oy, z + oz))
        .collect()
}

fn check_overlap(left: &Scanner, right: &Scanner) -> Option<Vector> {
    let mut ds: HashMap<Vector, HashSet<Vector>> = HashMap::new();
    for (ax, ay, az) in right.iter().copied() {
        for (bx, by, bz) in left.iter().copied() {
            ds.entry((bx - ax, by - ay, bz - az,)).or_default().insert((ax, ay, az));
        }
    }
    let shared: Vec<_> = ds
        .iter()
        .filter(|(_, points)| points.len() >= 12)
        .map(|(distance, _)| *distance)
        .collect();

    shared.first().and_then(|&d| Some(d))
}

#[aoc(day19, part1)]
/// oh no
/// 
/// # Examples
/// 
/// oh no
fn oh_no(input: &[Scanner]) -> usize {
    let mut scanners = input[1..].to_vec();
    while scanners.len() > 1 {
        let (i, j, _, union) = find_overlapping_pair(&scanners);
        scanners.remove(j);
        *scanners.get_mut(i).unwrap() = union;
    }
    scanners[0].len()
}

fn dist((ax, ay, az): Vector, (bx, by, bz): Vector) -> usize {
    (ax-bx).abs() as usize + (ay-by).abs() as usize + (az-bz).abs() as usize
}

#[aoc(day19, part2)]
fn oh_no_episode_2(input: &[Scanner]) -> usize {
    let mut scanners = input[1..].to_vec();
    let mut offsets = vec![];
    while scanners.len() > 1 {
        let (i, j, offset, union) = find_overlapping_pair(&scanners);
        scanners.remove(j);
        offsets.push(offset);
        *scanners.get_mut(i).unwrap() = union;
    }
    offsets
        .iter()
        .combinations(2)
        .map(|pair| dist(*pair[0], *pair[1]))
        .max()
        .unwrap_or_default()
}
