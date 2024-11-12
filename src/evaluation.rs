use crate::model::CellContent;
use crate::{
    board::Board,
    grid_iterators::{
        GridColumns, GridDownRightDiagonals, GridUpRightDiagonals,
    },
};

fn check_for_5_in_board(board: &Board) -> i32 {
    // Check horizontally
    for row in board.board.iter() {
        for window in row.windows(5) {
            if window == &[CellContent::Ally; 5] {
                return 1000000;
            }
            if window == &[CellContent::Opponent; 5] {
                return -1000000;
            }
        }
    }

    // Check vertically
    for col in GridColumns::new(&board.board) {
        let temp: Vec<_> = col.collect();
        for window in temp.windows(5) {
            if window == &[&CellContent::Ally; 5] {
                return 1000000;
            }
            if window == &[&CellContent::Opponent; 5] {
                return -1000000;
            }
        }
    }

    // Check diagonally (up right)
    for diag in GridUpRightDiagonals::new(&board.board) {
        let temp: Vec<_> = diag.collect();
        for window in temp.windows(5) {
            if window == &[&CellContent::Ally; 5] {
                return 1000000;
            }
            if window == &[&CellContent::Opponent; 5] {
                return -1000000;
            }
        }
    }

    // Check diagonally (up left)
    for diag in GridDownRightDiagonals::new(&board.board) {
        let temp: Vec<_> = diag.collect();
        for window in temp.windows(5) {
            if window == &[&CellContent::Ally; 5] {
                return 1000000;
            }
            if window == &[&CellContent::Opponent; 5] {
                return -1000000;
            }
        }
    }

    0
}

pub fn evaluate(board: &Board) -> i32 {
    let i = check_for_5_in_board(board);
    i
}
