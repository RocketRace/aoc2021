#[derive(Debug, Clone)]
struct Cuboid {
    on: bool,
    x: (isize, isize),
    y: (isize, isize),
    z: (isize, isize),
}

impl Cuboid {
    fn from_corners(x: (isize, isize), y: (isize, isize), z: (isize, isize), on: bool) -> Self {
        Self { x, y, z, on }
    }
    fn contains(&self, (x, y, z): (isize, isize, isize)) -> bool {
        self.x.0 <= x && x <= self.x.1 &&
        self.y.0 <= y && y <= self.y.1 &&
        self.z.0 <= z && z <= self.z.1
    }

    fn area(&self) -> isize {
        (self.x.1 - self.x.0 + 1) *
        (self.y.1 - self.y.0 + 1) *
        (self.z.1 - self.z.0 + 1)
    }

    fn chomp(&mut self, other: &Cuboid) -> Vec<Cuboid> {
        let mut children = vec![];
        if (self.x.0 <= other.x.1 && self.x.1 >= other.x.0) && (self.y.0 <= other.y.1 && self.y.1 >= other.y.0)  && (self.z.0 <= other.z.1 && self.z.1 >= other.z.0) {
            if self.x.0 < other.x.0 {
                children.push(Cuboid::from_corners((self.x.0, other.x.0 - 1), (self.y.0, self.y.1), (self.z.0, self.z.1), self.on));
                self.x.0 = other.x.0;
            }
            if self.x.1 > other.x.1 {
                children.push(Cuboid::from_corners((other.x.1 + 1, self.x.1), (self.y.0, self.y.1), (self.z.0, self.z.1), self.on));
                self.x.1 = other.x.1;
            }
            if self.y.0 < other.y.0 {
                children.push(Cuboid::from_corners((self.x.0, self.x.1), (self.y.0, other.y.0 - 1), (self.z.0, self.z.1), self.on));
                self.y.0 = other.y.0;
            }
            if self.y.1 > other.y.1 {
                children.push(Cuboid::from_corners((self.x.0, self.x.1), (other.y.1 + 1, self.y.1), (self.z.0, self.z.1), self.on));
                self.y.1 = other.y.1;
            }
            if self.z.0 < other.z.0 {
                children.push(Cuboid::from_corners((self.x.0, self.x.1), (self.y.0, self.y.1), (self.z.0, other.z.0 - 1), self.on));
                self.z.0 = other.z.0;
            }
            if self.z.1 > other.z.1 {
                children.push(Cuboid::from_corners((self.x.0, self.x.1), (self.y.0, self.y.1), (other.z.1 + 1, self.z.1), self.on));
                self.z.1 = other.z.1;
            }
        } else { 
            children.push(self.to_owned());
        }
        children
    }
}

#[aoc_generator(day22)]
fn generator(input: &str) -> Vec<Cuboid> {
    input
        .lines()
        .map(|line| {
            let (on, parts) = if line.starts_with("on ") {
                (true, line.trim_start_matches("on "))
            }
            else {
                (false, line.trim_start_matches("off "))
            };
            let axes: Vec<_> = parts.split(",").collect();
            let (x_raw_s, x_raw_e) = axes[0][2..].split_once("..").unwrap();
            let (y_raw_s, y_raw_e) = axes[1][2..].split_once("..").unwrap();
            let (z_raw_s, z_raw_e) = axes[2][2..].split_once("..").unwrap();
            Cuboid {
                on,
                x: (x_raw_s.parse().unwrap(), x_raw_e.parse().unwrap()),
                y: (y_raw_s.parse().unwrap(), y_raw_e.parse().unwrap()),
                z: (z_raw_s.parse().unwrap(), z_raw_e.parse().unwrap()),
            }
        })
        .collect()
}

#[aoc(day22, part1)]
fn brute_force_small_range(input: &[Cuboid]) -> usize {
    let mut count = 0;
    for x in -50..=50 {
        for y in -50..=50 {
            for z in -50..=50 {
                if match input.iter().rev().find(|cube| cube.contains((x, y, z))) {
                    Some(cube) => cube.on,
                    None => false,
                } {
                    count += 1;
                }
            }
        }
    }
    count
}

#[aoc(day22, part2)]
fn volume_big_range(input: &[Cuboid]) -> isize {
    let mut unique_cuboids: Vec<Cuboid> = vec![];
    for cuboid in input {
        let mut updated = unique_cuboids.iter_mut().flat_map(|unique| unique.chomp(cuboid)).collect();
        std::mem::swap(&mut updated, &mut unique_cuboids);
        unique_cuboids.push(cuboid.clone());
    }
    unique_cuboids.iter().map(|cuboid| if cuboid.on {cuboid.area()} else {0}).sum()
}
