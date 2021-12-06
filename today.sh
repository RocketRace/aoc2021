#!/bin/zsh
day=$(date +%-d)
year=$(date +%Y)
template="#[aoc_generator(day$day)]
fn generator(input: &str) -> _ {

}

#[aoc(day$day, part1)]
fn entry(input: &_) -> usize {
    0
}
"
[ -e "./input/$year/day$day.txt" ] || cargo aoc input
[ -e "./src/y$year/d$day.rs" ] ||
    echo $template > "./src/y$year/d$day.rs" &&
    echo "mod d$day;" >> "./src/y$year/mod.rs"  &&
    echo "Created template file"
code -r "./src/y$year/d$day.rs"
open "https://adventofcode.com/$year/day/$day"
