#[derive(Debug, Default, Clone)]
pub struct Bingo {
    numbers: Vec<usize>,
    boards: Vec<Option<Vec<Option<usize>>>>
}

#[aoc_generator(day4)]
pub fn bingo_boards(input: &str) -> Bingo {
    let mut bingo = Bingo::default();
    
    let mut iter = input.split("\n");
    let nums = iter.next().unwrap();
    let board_lines: Vec<_> = iter.collect();
    
    bingo.numbers.extend(
        nums.split(",")
            .map(|n| n.parse::<usize>().unwrap())
    );

    for slice in board_lines.chunks(6) {
        bingo.boards.push(Some(slice[1..]
            .iter()
            .flat_map(|&line| 
                line.split_ascii_whitespace()
                    .map(|n| Some(n.parse().unwrap()))
            )
            .collect()
        ))
    }
    bingo
}

fn check_for_bingo_around(board: &[Option<usize>], index: usize) -> bool {
    let x = index % 5;
    let y = index / 5 * 5;
    (
        board[x].is_none() && board[x+5].is_none() && board[x+10].is_none() && board[x+15].is_none() && board[x+20].is_none()
    ) || (
        board[y].is_none() && board[y+1].is_none() && board[y+2].is_none() && board[y+3].is_none() && board[y+4].is_none()
    )
}

#[aoc(day4, part1)]
pub fn bingo_winner(input: &Bingo) -> usize {
    let mut bingo = input.clone();
    for &num in &bingo.numbers {
        for board in &mut bingo.boards { // Option::contains is not stable for some reason
            if let Some(board) = board {
                if let Some(i) = &board.iter().position(|&n| n.filter(|&inner| inner == num).is_some()) {
                    board[*i] = None;
                    if check_for_bingo_around(&board, *i) {
                        return board.iter().flatten().sum::<usize>() * num
                    }
                }
            }
        }
    }
    0
}

#[aoc(day4, part2)]
pub fn bingo_loser(input: &Bingo) -> usize {
    let mut bingo = input.clone();
    let mut remaining = bingo.boards.len();
    for &num in &bingo.numbers {
        let mut indices = vec![];
        for (b, board) in bingo.boards.iter_mut().enumerate() { // Option::contains is not stable for some reason
            if let Some(board) = board {
                if let Some(i) = &board.iter().position(|&n| n.filter(|&inner| inner == num).is_some()) {
                    board[*i] = None;
                    if check_for_bingo_around(&board, *i) {
                        indices.push(b);
                        remaining -= 1;
                        if remaining == 0 {
                            return board.iter().flatten().sum::<usize>() * num
                        }
                    }
                }
            }
        }
        for &i in indices.iter().rev() {
            bingo.boards[i] = None;
        }
    }
    0
}
