type Vector = [isize; 2];

fn int<'a, T: Iterator<Item=&'a str>>(iter: &mut T) -> isize {
    iter.next().unwrap().parse().unwrap()
}

#[aoc_generator(day2, part1)]
pub fn vectors(input: &str) -> Vec<Vector> {
    input
        .split("\n")
        .map(|s| {
            let mut iter = s.split(" ");
            match iter.next().unwrap() {
                "up"      => [0, -int(&mut iter)],
                "down"    => [0,  int(&mut iter)],
                "forward" => [int(&mut iter),  0],
                _ => unreachable!()
            }
        }).collect()
}

#[aoc(day2, part1)]
pub fn vector_movement(input: &[Vector]) -> isize {
    input.iter()
        .fold(
            [0, 0], 
            |acc, mov| [acc[0] + mov[0], acc[1] + mov[1]]
        ).iter()
        .product()
}

// sigh... I guess I'll have to make an instruction set this time
pub enum Instruction {
    Aim(isize),
    Move(isize),
}

#[aoc_generator(day2, part2)]
pub fn instructions(input: &str) -> Vec<Instruction> {
    input
        .split("\n")
        .map(|s| {
            let mut iter = s.split(" ");
            match iter.next().unwrap() {
                "up"      => Instruction::Aim(-int(&mut iter)),
                "down"    => Instruction::Aim(int(&mut iter)),
                "forward" => Instruction::Move(int(&mut iter)),
                _ => unreachable!()
            }
        }).collect()
}

#[aoc(day2, part2)]
pub fn instruction_movement(input: &[Instruction]) -> isize {
    input
        .iter()
        .fold(([0, 0], 0), |([x, y], a), i| match i {
            Instruction::Aim(n) => ([x, y], a + n),
            Instruction::Move(n) => ([x + n, y + a * n], a),
        }).0
        .iter()
        .product()
}
