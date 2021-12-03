#[aoc_generator(day3, part1)]
pub fn bits(input: &str) -> Vec<Vec<u32>> {
    input
        .split("\n")
        .map(|s| 
            s
                .chars()    
                .map(|c| c.to_digit(2).unwrap())
                .collect()
        )
        .collect()
}

fn vector_sum(left: &[u32], right: &[u32]) -> Vec<u32> {
    left
        .iter()
        .zip(right.iter())
        .map(|(&l, &r)| l + r)
        .collect()
}

#[aoc(day3, part1)]
pub fn gamma_epsilon(input: &[Vec<u32>]) -> u32 {
    let half = input.len() as u32 / 2;
    let width = input[0].len() as u32;
    let mask = (1 << width) - 1;
    let bits: Vec<u32> = input
        .iter()
        .fold(
            vec![0u32; 16], 
            |acc, bits| vector_sum(&acc, &bits)
        )
        .iter()
        .map(|&n| (n / half) as u32)
        .collect();
    let gamma = bits
        .iter()
        .enumerate()
        .rfold(0, |n, (i, &b)| n ^ (b << (width - 1 - i  as u32)));
    
    gamma * (!gamma & mask)
}

#[derive(Debug, Clone, Default)]
struct Node {
    count: usize,
    zeroes: Option<Box<Node>>,
    ones: Option<Box<Node>>
}
impl Node {
    fn insert(self, n: u32, depth: u32, max_depth: u32) -> Node {
        if depth >= max_depth {
            return self;
        }
        let (child, other) = if n >> depth & 1 == 1 {
            (self.ones, self.zeroes)
        }
        else {
            (self.zeroes, self.ones)
        };
        let new_node = Some(Box::new(match child {
            Some(node) => {
                node.insert(n, depth + 1, max_depth)
            },
            None => {
                Node::default().insert(n, depth + 1, max_depth)
            }
        }));
        if n >> depth & 1 == 1 {
            Node { count: self.count + 1, ones: new_node, zeroes: other }
        }
        else {
            Node { count: self.count + 1, zeroes: new_node, ones: other }
        }
    }

    fn most_common(&self, depth: u32, max_depth: u32) -> u32 {
        if depth == max_depth {
            return 0
        }
        match (&self.zeroes, &self.ones) {
            (None, None) => unreachable!(),
            (None, Some(o)) => o.most_common(depth + 1, max_depth) ^ 1 << depth,
            (Some(z), None) => z.most_common(depth + 1, max_depth),
            (Some(z), Some(o)) => {
                if o.count >= z.count {
                    o.most_common(depth + 1, max_depth) ^ 1 << depth
                }
                else {
                    z.most_common(depth + 1, max_depth)
                }
            },
        }
    }

    fn least_common(&self, depth: u32, max_depth: u32) -> u32 {
        if depth == max_depth {
            return 0
        }
        match (&self.zeroes, &self.ones) {
            (None, None) => unreachable!(),
            (None, Some(o)) => o.most_common(depth + 1, max_depth) ^ 1 << depth,
            (Some(z), None) => z.most_common(depth + 1, max_depth),
            (Some(z), Some(o)) => {
                if o.count >= z.count {
                    z.least_common(depth + 1, max_depth)
                }
                else {
                    o.least_common(depth + 1, max_depth) ^ 1 << depth
                }
            },
        }
    }
}

const BIT_COUNT: u32 = 12;

#[aoc_generator(day3, part2)]
pub fn trees(input: &str) -> Vec<u32> {
    input
        .split("\n")
        .map(|s| u32::from_str_radix(s, 2).unwrap().reverse_bits() >> (32 - BIT_COUNT)
        )
        .collect()
}

#[aoc(day3, part2)]
pub fn air_rating(input: &[u32]) -> u32 {
    let tree = input
        .iter()
        .fold(Node::default(), |tree, &n| tree.insert(n, 0, BIT_COUNT));

    let most = &tree.most_common(0, BIT_COUNT).reverse_bits() >> (32 - BIT_COUNT);
    let least = &tree.least_common(0, BIT_COUNT).reverse_bits() >> (32 - BIT_COUNT);

    most * least
}