#![allow(unused_assignments)] // nightly bugginess
use itertools::Itertools;

#[derive(Debug, Clone)]
enum Snail {
    Number(u8),
    Pair(Box<(Snail, Snail)>)
}

#[derive(Debug, Clone, Copy)]
enum Shrapnel {
    None,
    Left(u8),
    Right(u8),
    Both(u8, u8),
}


impl Snail {
    fn add(self, other: Snail) -> Snail {
        Snail::Pair(Box::new((self, other))).reduce()
    }

    fn is_topmost(&self) -> bool {
        matches!(self, &Snail::Pair(box (Snail::Number(_), Snail::Number(_))))
    }

    fn reduce(self) -> Snail {
        let mut reduced = self;
        loop {
            let mut update = false;
            // trailing shrapnel is ignored
            // (it flies off the board)
            (reduced, update, _) = reduced.explode(0);
            if !update {
                (reduced, update) = reduced.split();
            }
            if !update {
                break reduced
            }
        }
    }

    fn add_rightmost(self, value: u8) -> Self {
        match self {
            Snail::Number(n) => Snail::Number(n + value),
            Snail::Pair(box (l, r)) => Snail::Pair(Box::new((l, r.add_rightmost(value)))),
        }
    }

    fn add_leftmost(self, value: u8) -> Self {
        match self {
            Snail::Number(n) => Snail::Number(n + value),
            Snail::Pair(box (l, r)) => Snail::Pair(Box::new((l.add_leftmost(value), r))),
        }
    }

    fn explode(self, depth: usize) -> (Self, bool, Shrapnel) {
        if self.is_topmost() && depth >= 4 {
            // this one is going to boom
            if let Snail::Pair(box (Snail::Number(lv), Snail::Number(rv))) = self {
                (Snail::Number(0), true, Shrapnel::Both(lv, rv))
            }
            else {
                unreachable!("refactor incoming")
            }
        }
        else {
            match self {
                Snail::Number(_) => (self, false, Shrapnel::None),
                Snail::Pair(box (l, r)) => {
                    let (new_l, exploded, shrapnel) = l.explode(depth + 1);
                    if exploded {
                        match shrapnel {
                            Shrapnel::None => (Snail::Pair(Box::new((new_l, r))), true, Shrapnel::None),
                            Shrapnel::Left(lv) => (Snail::Pair(Box::new((new_l, r))), true, Shrapnel::Left(lv)),
                            Shrapnel::Right(rv) => (Snail::Pair(Box::new((new_l, r.add_leftmost(rv)))), true, Shrapnel::None),
                            Shrapnel::Both(lv, rv) => (Snail::Pair(Box::new((new_l, r.add_leftmost(rv)))), true, Shrapnel::Left(lv)),
                        }
                    }
                    else {
                        let (new_r, exploded, shrapnel) = r.explode(depth + 1);
                        if exploded {
                            match shrapnel {
                                Shrapnel::None => (Snail::Pair(Box::new((new_l, new_r))), true, Shrapnel::None),
                                Shrapnel::Left(lv) => (Snail::Pair(Box::new((new_l.add_rightmost(lv), new_r))), true, Shrapnel::None),
                                Shrapnel::Right(rv) => (Snail::Pair(Box::new((new_l, new_r))), true, Shrapnel::Right(rv)),
                                Shrapnel::Both(lv, rv) => (Snail::Pair(Box::new((new_l.add_rightmost(lv), new_r))), true, Shrapnel::Right(rv)),
                            }
                        }
                        else {
                            (Snail::Pair(Box::new((new_l, new_r))), false, Shrapnel::None)
                        }
                    }
                },
            }
        }
    }

    fn split(self) -> (Self, bool) {
        match self {
            Snail::Number(n) => {
                if n >= 10 {
                    (Snail::Pair(Box::new((Snail::Number(n / 2), Snail::Number(n - n / 2)))), true)
                }
                else {
                    (self, false)
                }
            }
            Snail::Pair(box (l, r)) => {
                let (new_l, updated) = l.split();
                if updated {
                    (Snail::Pair(Box::new((new_l, r))), true)
                }
                else {
                    let (new_r, update) = r.split();
                    (Snail::Pair(Box::new((new_l, new_r))), update)
                }
            }
        }
    }

    fn magnitude(&self) -> usize {
        match self {
            Snail::Number(n) => *n as usize,
            Snail::Pair(box (l, r)) => 3 * l.magnitude() + 2 * r.magnitude(),
        }
    }
}

// sorry, we don't have any parser generators in stock at the moment
fn recursive_descent_value<T: Iterator<Item=u8>>(input: &mut T) -> Snail {
    let c = input.next().unwrap();
    if c == b'[' {
        let l = recursive_descent_value(input);
        assert_eq!(input.next().unwrap(), b',');
        let r = recursive_descent_value(input);
        assert_eq!(input.next().unwrap(), b']');
        Snail::Pair(Box::new((l, r)))
    }
    else {
        Snail::Number(c - b'0')
    }
}

#[aoc_generator(day18)]
fn snail(input: &str) -> Vec<Snail> {
    // snail :3
    input.lines().map(|line| {
        // parser time :>
        // grammar:
        // - value := "[" value "," value "]" | "0"..="9"
        recursive_descent_value(&mut line.bytes())
    }).collect()
}

#[aoc(day18, part1)]
fn sum(input: &[Snail]) -> usize {
    input.to_owned().into_iter().reduce(Snail::add).map(|x| x.magnitude()).unwrap_or(0)
}

#[aoc(day18, part2)]
fn greatest_sum(input: &[Snail]) -> usize {
    // quite frankly I don't have the energy to work through the mathematical
    // properties of snail addition so I will just
    input.to_owned().into_iter().permutations(2).flat_map(|pair| [
        pair[0].clone().add(pair[1].clone()).magnitude(), // look at me in fear
        pair[1].clone().add(pair[0].clone()).magnitude(), // look at me in fear
    ]).max().unwrap_or(0)
}

