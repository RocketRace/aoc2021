use std::collections::{HashMap, BTreeMap, BTreeSet};

#[aoc_generator(day12)]
fn generator(input: &str) -> Vec<(String, String)> {
    input
        .split("\n")
        .map(|line| {
            let mut iter = line.split("-").map(|s| s.to_owned());
            (iter.next().unwrap(), iter.next().unwrap())
        })
        .collect()
}

fn recursively_count(paths: &BTreeMap<usize, Vec<usize>>, big: &BTreeSet<usize>, visited: &Vec<usize>, current: usize, goal: usize) -> usize {
    let options = &paths[&current];
    let mut sum = 0;
    let mut new = visited.clone();
    if !big.contains(&current) {
        new.push(current);
    }
    for &option in options {
        if option == goal {
            sum += 1;
        }
        else if !visited.contains(&option) {
            sum += recursively_count(paths, big, &new, option, goal);
        }
    }
    sum
}

fn recursively_count2(paths: &BTreeMap<usize, Vec<usize>>, big: &BTreeSet<usize>, visited: &Vec<usize>, current: usize, start: usize, goal: usize, extra_used: bool) -> usize {
    let options = &paths[&current];
    let mut sum = 0;
    let mut new = visited.clone();
    if !big.contains(&current) {
        new.push(current);
    }
    for &option in options {
        if option != start {
            if option == goal {
                sum += 1;
            }
            else if !visited.contains(&option) {
                sum += recursively_count2(paths, big, &new, option, start, goal, extra_used);
            }
            else if !extra_used  {
                sum += recursively_count2(paths, big, &new, option, start, goal, true);
            }
        }
    }
    sum
}

#[aoc(day12, part1)]
fn count_paths(input: &[(String, String)]) -> usize {
    // I had a lot of code for graph substitutions but I deleted it all, it was too buggy :(
    let mut names = HashMap::new();
    let mut paths = BTreeMap::new();
    let mut big = BTreeSet::new();
    for (f, t) in input {
        let l = names.len();
        names.entry(f.clone()).or_insert(l);
        let l = names.len();
        names.entry(t.clone()).or_insert(l);
        paths.entry(names[f]).or_insert(vec![]).push(names[t]);
        paths.entry(names[t]).or_insert(vec![]).push(names[f]);
        if &f.to_ascii_uppercase() == f {
            big.insert(names[f]);
        }
        if &t.to_ascii_uppercase() == t {
            big.insert(names[t]);
        }
    }
    let start_node = names["start"];
    let end_node = names["end"];
    recursively_count(&paths, &big, &vec![start_node], start_node, end_node)
}

#[aoc(day12, part2)]
fn count_paths_repetition(input: &[(String, String)]) -> usize {
    // I had a lot of code for graph substitutions but I deleted it all, it was too buggy :(
    let mut names = HashMap::new();
    let mut paths = BTreeMap::new();
    let mut big = BTreeSet::new();
    for (f, t) in input {
        let l = names.len();
        names.entry(f.clone()).or_insert(l);
        let l = names.len();
        names.entry(t.clone()).or_insert(l);
        paths.entry(names[f]).or_insert(vec![]).push(names[t]);
        paths.entry(names[t]).or_insert(vec![]).push(names[f]);
        if &f.to_ascii_uppercase() == f {
            big.insert(names[f]);
        }
        if &t.to_ascii_uppercase() == t {
            big.insert(names[t]);
        }
    }
    let start_node = names["start"];
    let end_node = names["end"];
    recursively_count2(&paths, &big, &vec![], start_node, start_node, end_node, false)
}

