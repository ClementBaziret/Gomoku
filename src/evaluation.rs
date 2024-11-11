use crate::grid_iterators::{
    GridColumns, GridDownRightDiagonals, GridUpRightDiagonals,
};
use crate::my_board::{CellType, Move, MyBoard}; // Import MyBoard and Move

fn check_for_5_in_board(board: &MyBoard) -> i32 {
    // Check horizontally
    for row in board.board.iter() {
        for window in row.windows(5) {
            if window == &[CellType::Ally; 5] {
                return 1000000;
            }
            if window == &[CellType::Enemy; 5] {
                return -1000000;
            }
        }
    }

    // Check vertically
    for col in GridColumns::new(&board.board) {
        let temp: Vec<_> = col.collect();
        for window in temp.windows(5) {
            if window == &[&CellType::Ally; 5] {
                return 1000000;
            }
            if window == &[&CellType::Enemy; 5] {
                return -1000000;
            }
        }
    }

    // Check diagonally (up right)
    for diag in GridUpRightDiagonals::new(&board.board) {
        let temp: Vec<_> = diag.collect();
        for window in temp.windows(5) {
            if window == &[&CellType::Ally; 5] {
                return 1000000;
            }
            if window == &[&CellType::Enemy; 5] {
                return -1000000;
            }
        }
    }

    // Check diagonally (up left)
    for diag in GridDownRightDiagonals::new(&board.board) {
        let temp: Vec<_> = diag.collect();
        for window in temp.windows(5) {
            if window == &[&CellType::Ally; 5] {
                return 1000000;
            }
            if window == &[&CellType::Enemy; 5] {
                return -1000000;
            }
        }
    }

    0
}

pub fn evaluate(board: &MyBoard) -> i32 {
    let i = check_for_5_in_board(board);
    i
}
