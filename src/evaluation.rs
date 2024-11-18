use crate::model::CellContent;
use crate::{
    board::Board,
    grid_iterators::{
        GridColumns, GridDownRightDiagonals, GridUpRightDiagonals,
    },
};

fn iterate_row(temp: Vec<&CellContent>) -> i32 {
    for window in temp.windows(5) {
        if window == &[&CellContent::Ally; 5] {
            return 1000000;
        }
        if window == &[&CellContent::Opponent; 5] {
            return -1000000;
        }
        if window
            == &[
                // &CellContent::Empty,
                &CellContent::Ally,
                &CellContent::Ally,
                &CellContent::Ally,
                &CellContent::Ally,
                // &CellContent::Empty,
            ]
        {
            return 40;
        }
        if window
            == &[
                &CellContent::Empty,
                &CellContent::Opponent,
                &CellContent::Opponent,
                &CellContent::Opponent,
                &CellContent::Opponent,
                &CellContent::Empty,
            ]
        {
            return -40;
        }
        if window
            == &[
                &CellContent::Empty,
                &CellContent::Ally,
                &CellContent::Ally,
                &CellContent::Ally,
                &CellContent::Empty,
            ]
        {
            println!("found it");
            return 20;
        }
        if window
            == &[
                &CellContent::Empty,
                &CellContent::Opponent,
                &CellContent::Opponent,
                &CellContent::Opponent,
                &CellContent::Empty,
            ]
        {
            println!("found bad");
            return -20;
        }
    }
    0
}

fn check_for_5_in_board(board: &Board) -> i32 {
    let mut score = 0;

    // board.print_board();
    println!("1: {}", &score);
    // Check horizontally
    for row in board.board.iter() {
        let temp: Vec<_> = row.iter().collect();
        score += iterate_row(temp);
    }

    println!("2: {}", &score);
    // Check vertically
    for col in GridColumns::new(&board.board) {
        let temp: Vec<_> = col.collect();
        score += iterate_row(temp);
    }

    println!("3: {}", &score);
    // Check diagonally (up right)
    for diag in GridUpRightDiagonals::new(&board.board) {
        let temp: Vec<_> = diag.collect();
        score += iterate_row(temp);
    }

    println!("4: {}", &score);
    // Check diagonally (up left)
    for diag in GridDownRightDiagonals::new(&board.board) {
        let temp: Vec<_> = diag.collect();
        score += iterate_row(temp);
    }
    println!("5: {}", &score);
    // board.print_board();
    score
}

pub fn evaluate(board: &Board) -> i32 {
    let i = check_for_5_in_board(board);
    i
}
