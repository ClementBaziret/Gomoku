use crate::model::CellContent;
use crate::{
    board::Board,
    grid_iterators::{
        GridColumns, GridDownRightDiagonals, GridUpRightDiagonals,
    },
};

fn iterate_row(temp: Vec<&CellContent>) -> i32 {
    let mut result = 0;
    let mut temp_result = 0;
    for window in temp.windows(5) {
        // Check for 5
        if window == &[&CellContent::Ally; 5] {
            temp_result = 1000000;
        }
        if window == &[&CellContent::Opponent; 5] {
            temp_result = -1000000;
        }

        // Check for different 4
        if window
            == &[
                &CellContent::Empty,
                &CellContent::Ally,
                &CellContent::Ally,
                &CellContent::Ally,
                &CellContent::Ally,
            ]
            || window
                == &[
                    &CellContent::Ally,
                    &CellContent::Ally,
                    &CellContent::Ally,
                    &CellContent::Ally,
                    &CellContent::Empty,
                ]
        {
            temp_result = 5000;
        }
        if window
            == &[
                &CellContent::Empty,
                &CellContent::Opponent,
                &CellContent::Opponent,
                &CellContent::Opponent,
                &CellContent::Opponent,
            ]
            || window
                == &[
                    &CellContent::Opponent,
                    &CellContent::Opponent,
                    &CellContent::Opponent,
                    &CellContent::Opponent,
                    &CellContent::Empty,
                ]
        {
            temp_result = -5000;
        }
        if window
            == &[
                &CellContent::Opponent,
                &CellContent::Ally,
                &CellContent::Ally,
                &CellContent::Ally,
                &CellContent::Ally,
            ]
            || window
                == &[
                    &CellContent::Ally,
                    &CellContent::Ally,
                    &CellContent::Ally,
                    &CellContent::Ally,
                    &CellContent::Opponent,
                ]
        {
            temp_result = 2000;
        }
        if window
            == &[
                &CellContent::Ally,
                &CellContent::Opponent,
                &CellContent::Opponent,
                &CellContent::Opponent,
                &CellContent::Opponent,
            ]
            || window
                == &[
                    &CellContent::Opponent,
                    &CellContent::Opponent,
                    &CellContent::Opponent,
                    &CellContent::Opponent,
                    &CellContent::Ally,
                ]
        {
            temp_result = -2000;
        }

        // Check for different 3
        if window
            == &[
                &CellContent::Ally,
                &CellContent::Ally,
                &CellContent::Ally,
                &CellContent::Empty,
                &CellContent::Empty,
            ]
            || window
                == &[
                    &CellContent::Empty,
                    &CellContent::Ally,
                    &CellContent::Ally,
                    &CellContent::Ally,
                    &CellContent::Empty,
                ]
            || window
                == &[
                    &CellContent::Empty,
                    &CellContent::Empty,
                    &CellContent::Ally,
                    &CellContent::Ally,
                    &CellContent::Ally,
                ]
        {
            temp_result = 1000;
        }
        if window
            == &[
                &CellContent::Opponent,
                &CellContent::Opponent,
                &CellContent::Opponent,
                &CellContent::Empty,
                &CellContent::Empty,
            ]
            || window
                == &[
                    &CellContent::Empty,
                    &CellContent::Opponent,
                    &CellContent::Opponent,
                    &CellContent::Opponent,
                    &CellContent::Empty,
                ]
            || window
                == &[
                    &CellContent::Empty,
                    &CellContent::Empty,
                    &CellContent::Opponent,
                    &CellContent::Opponent,
                    &CellContent::Opponent,
                ]
        {
            temp_result = -1000;
        }

        // Misc
        if window
            == &[
                &CellContent::Ally,
                &CellContent::Empty,
                &CellContent::Ally,
                &CellContent::Ally,
                &CellContent::Empty,
            ]
            || window
                == &[
                    &CellContent::Empty,
                    &CellContent::Ally,
                    &CellContent::Ally,
                    &CellContent::Empty,
                    &CellContent::Ally,
                ]
        {
            temp_result = 800;
        }
        if window
            == &[
                &CellContent::Opponent,
                &CellContent::Empty,
                &CellContent::Opponent,
                &CellContent::Opponent,
                &CellContent::Empty,
            ]
            || window
                == &[
                    &CellContent::Empty,
                    &CellContent::Opponent,
                    &CellContent::Opponent,
                    &CellContent::Empty,
                    &CellContent::Opponent,
                ]
        {
            temp_result = -800;
        }

        // 2 in a row
        if window
            == &[
                &CellContent::Empty,
                &CellContent::Ally,
                &CellContent::Ally,
                &CellContent::Empty,
                &CellContent::Empty,
            ]
            || window
                == &[
                    &CellContent::Empty,
                    &CellContent::Empty,
                    &CellContent::Ally,
                    &CellContent::Ally,
                    &CellContent::Empty,
                ]
        {
            temp_result = 100
        }
        if window
            == &[
                &CellContent::Empty,
                &CellContent::Opponent,
                &CellContent::Opponent,
                &CellContent::Empty,
                &CellContent::Empty,
            ]
            || window
                == &[
                    &CellContent::Empty,
                    &CellContent::Empty,
                    &CellContent::Opponent,
                    &CellContent::Opponent,
                    &CellContent::Empty,
                ]
        {
            temp_result = -100
        }
        if temp_result > result {
            result = temp_result;
        }
    }
    result
}

fn check_rows_in_board(board: &Board) -> i32 {
    let mut score = 0;

    // Check horizontally
    for row in board.board.iter() {
        let temp: Vec<_> = row.iter().collect();
        score += iterate_row(temp);
    }

    // Check vertically
    for col in GridColumns::new(&board.board) {
        let temp: Vec<_> = col.collect();
        score += iterate_row(temp);
    }

    // Check diagonally (up right)
    for diag in GridUpRightDiagonals::new(&board.board) {
        let temp: Vec<_> = diag.collect();
        score += iterate_row(temp);
    }

    // Check diagonally (up left)
    for diag in GridDownRightDiagonals::new(&board.board) {
        let temp: Vec<_> = diag.collect();
        score += iterate_row(temp);
    }
    score
}

pub fn evaluate(board: &Board) -> i32 {
    let i = check_rows_in_board(board);
    i
}
