use crate::my_board::{CellType, Move, MyBoard}; // Import MyBoard and Move

fn check_consecutive(
    board: &MyBoard,
    x: u8,
    y: u8,
    cell_type: CellType,
    consecutive: u8,
) -> bool {
    let mut in_a_row: i8 = 1;
    for i in 1..consecutive {
        let newx: i8 = x as i8 + i as i8;
        let newy = y as i8 + i as i8;
        println!("l1{} {} {}", newx, newy, board.size);
        if newx < 0
            || newy < 0
            || newx >= board.size as i8
            || newy >= board.size as i8
        {
            println!("Is in condition");
            break;
        }
        println!("l1{} {} {}", newx, newy, board.size);
        if board.board[newx as usize][newy as usize] == cell_type {
            in_a_row = in_a_row + 1;
        } else {
            break;
        }
    }
    for i in 1..consecutive {
        let newx: i8 = x as i8 - i as i8;
        let newy: i8 = y as i8 - i as i8;
        println!("l2{} {} {}", newx, newy, board.size);
        if newx < 0
            || newy < 0
            || newx >= board.size as i8
            || newy >= board.size as i8
        {
            break;
        }
        println!("l2{} {} {}", newx, newy, board.size);
        if board.board[newx as usize][newy as usize] == cell_type {
            in_a_row = in_a_row + 1;
        } else {
            break;
        }
    }
    if in_a_row >= consecutive as i8 {
        println!("{} - {} in a 5", x, y);
        return true;
    }

    false
}

fn check_for_5(board: &MyBoard, mv: &Move) -> i32 {
    let x = mv.x;
    let y = mv.y;

    if check_consecutive(board, x, y, CellType::Enemy, 5) {
        return 1000000;
    }
    if check_consecutive(board, x, y, CellType::Ally, 5) {
        return -1000000;
    }
    0
}

fn check_for_5_in_board(board: &MyBoard) -> i32 {
    for (i, row) in board.board.iter().enumerate() {
        for window in row.windows(5) {
            if window == &[CellType::Ally; 5] {
                return 1000000;
            }
            if window == &[CellType::Enemy; 5] {
                return -1000000;
            }
        }
    }

    0
}

pub fn evaluate(board: &MyBoard) -> i32 {
    let i = check_for_5_in_board(board);
    i + 10
}
