use std::collections::BTreeMap;

use itertools::Itertools;

struct Polymers {
    template: Vec<usize>,
    rules: Vec<usize>,
    translation: Vec<u8>,
    count: usize,
}

impl Polymers {
    #[allow(dead_code)]
    fn display_pairs(&self, pairs: &[usize]) {
        for (i, &pair) in pairs.iter().enumerate() {
            let (a, b) = unkey(i, self.count);
            println!("{}{}: {}", self.translation[a] as char, self.translation[b] as char, pair);
        }
        println!();
    }
}

fn add_entry(map: &mut BTreeMap<u8, usize>, key: u8) -> usize {
    let default = map.len();
    *map.entry(key).or_insert(default)
}

fn key(a: usize, b: usize, n: usize) -> usize {
    a * n + b
}

fn unkey(i: usize, n: usize) -> (usize, usize) {
    (i / n, i % n)
}

fn double(i: usize, o: usize, n: usize) -> (usize, usize) {
    let (a, b) = unkey(i, n);
    (key(a, o, n), key(o, b, n))
}

#[aoc_generator(day14)]
fn polymers(input: &str) -> Polymers {
    let mut lines = input.lines();
    let template_indices = lines.next().unwrap().as_bytes();

    let mut translation = BTreeMap::new();
    let mut rule_indices = vec![];
    for line in lines.skip(1) {
        let bytes = line.as_bytes();
        let first = add_entry(&mut translation, bytes[0]);
        let second = add_entry(&mut translation, bytes[1]);
        let last = add_entry(&mut translation, bytes[6]);
        rule_indices.push((first, second, last));
    }

    let count = translation.len();

    let mut rules = vec![0usize; count * count];

    for (a, b, c) in rule_indices {
        rules[key(a, b, count)] = c;
    }

    let template = template_indices.iter().map(|n| translation[n]).collect();
    
    let reverse_translation = BTreeMap::from_iter(translation.iter().map(|(&a, &b)| (b, a))).values().map(|&x| x).collect();

    Polymers { 
        template,
        rules,
        translation: reverse_translation,
        count
    }
}

fn iterate(input: &Polymers, steps: usize) -> usize {
    let mut pairs = vec![0usize; input.count * input.count];
    for pair in input.template.windows(2) {
        pairs[key(pair[0], pair[1], input.count)] += 1;
    }
    for _ in 0..steps {
        let mut deltas = vec![0isize; input.count * input.count];
        for (i, &rule) in input.rules.iter().enumerate() {
            let (f, s) = double(i, rule, input.count);
            deltas[f] += pairs[i] as isize;
            deltas[s] += pairs[i] as isize;
            deltas[i] -= pairs[i] as isize;
        }
        for (n, d) in pairs.iter_mut().zip(deltas.iter()) {
            if *d > 0 {
                *n += *d as usize;
            }
            else {
                *n -= -*d as usize;
            }
        }
    }
    let mut individuals = vec![0usize; input.count];
    for (i, &count) in pairs.iter().enumerate() {
        let (a, b) = unkey(i, input.count);
        individuals[a] += count;
        individuals[b] += count;
    }
    individuals[*input.template.last().unwrap()] += 1;
    individuals[input.template[0]] += 1;
    match individuals.iter().minmax() {
        itertools::MinMaxResult::MinMax(a, b) => *b / 2 - *a / 2,
        _ => unreachable!()
    }
}

#[aoc(day14, part1)]
fn iterate_10_times(input: &Polymers) -> usize {
    iterate(input, 10)
}

#[aoc(day14, part2)]
fn iterate_40_times(input: &Polymers) -> usize {
    iterate(input, 40)
}

