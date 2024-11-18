use crate::model::CellContent;
use crate::{
    board::Board,
    grid_iterators::{
        GridColumns, GridDownRightDiagonals, GridUpRightDiagonals,
    },
};

fn iterate_row(temp: Vec<&CellContent>) -> i32 {
    let mut result = -10000000;
    let mut temp_result = result;
    for window in temp.windows(5) {
        // Check for 5
        if window == &[&CellContent::Ally; 5] {
            return 1000000;
        }
        if window == &[&CellContent::Opponent; 5] {
            return -1000000;
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
    if result == -10000000 {
        result = 0;
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

#[test]
fn check_horizontal_winning_board() {
    let mut board = Board::new();

    board.board[1][1] = CellContent::Ally;
    board.board[1][2] = CellContent::Ally;
    board.board[1][3] = CellContent::Ally;
    board.board[1][4] = CellContent::Ally;
    board.board[1][5] = CellContent::Ally;

    let move_value = evaluate(&mut board);

    assert_eq!(move_value, 1000000);
}

#[test]
fn check_vertical_winning_board() {
    let mut board = Board::new();

    board.board[1][1] = CellContent::Ally;
    board.board[2][1] = CellContent::Ally;
    board.board[3][1] = CellContent::Ally;
    board.board[4][1] = CellContent::Ally;
    board.board[5][1] = CellContent::Ally;

    let move_value = evaluate(&mut board);

    assert_eq!(move_value, 1000000);
}

#[test]
fn check_down_right_winning_board() {
    let mut board = Board::new();

    board.board[1][1] = CellContent::Ally;
    board.board[2][2] = CellContent::Ally;
    board.board[3][3] = CellContent::Ally;
    board.board[4][4] = CellContent::Ally;
    board.board[5][5] = CellContent::Ally;

    let move_value = evaluate(&mut board);

    assert_eq!(move_value, 1000000);
}

#[test]
fn check_up_right_winning_board() {
    let mut board = Board::new();

    board.board[4][1] = CellContent::Ally;
    board.board[3][2] = CellContent::Ally;
    board.board[2][3] = CellContent::Ally;
    board.board[1][4] = CellContent::Ally;
    board.board[0][5] = CellContent::Ally;

    let move_value = evaluate(&mut board);

    assert_eq!(move_value, 1000000);
}

#[test]
fn check_horizontal_losing_board() {
    let mut board = Board::new();

    board.board[1][1] = CellContent::Opponent;
    board.board[1][2] = CellContent::Opponent;
    board.board[1][3] = CellContent::Opponent;
    board.board[1][4] = CellContent::Opponent;
    board.board[1][5] = CellContent::Opponent;

    let move_value = evaluate(&mut board);

    assert_eq!(move_value, -1000000);
}

#[test]
fn check_vertical_losing_board() {
    let mut board = Board::new();

    board.board[1][1] = CellContent::Opponent;
    board.board[2][1] = CellContent::Opponent;
    board.board[3][1] = CellContent::Opponent;
    board.board[4][1] = CellContent::Opponent;
    board.board[5][1] = CellContent::Opponent;

    let move_value = evaluate(&mut board);

    assert_eq!(move_value, -1000000);
}

#[test]
fn check_down_right_losing_board() {
    let mut board = Board::new();

    board.board[1][1] = CellContent::Opponent;
    board.board[2][2] = CellContent::Opponent;
    board.board[3][3] = CellContent::Opponent;
    board.board[4][4] = CellContent::Opponent;
    board.board[5][5] = CellContent::Opponent;

    let move_value = evaluate(&mut board);

    assert_eq!(move_value, -1000000);
}

#[test]
fn check_up_right_losing_board() {
    let mut board = Board::new();

    board.board[4][1] = CellContent::Opponent;
    board.board[3][2] = CellContent::Opponent;
    board.board[2][3] = CellContent::Opponent;
    board.board[1][4] = CellContent::Opponent;
    board.board[0][5] = CellContent::Opponent;

    let move_value = evaluate(&mut board);

    assert_eq!(move_value, -1000000);
}
