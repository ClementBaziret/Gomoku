use crate::model::CellContent;
use crate::{
    board::Board,
    grid_iterators::{
        GridColumns, GridDownRightDiagonals, GridUpRightDiagonals,
    },
};

fn iterate_row(temp: Vec<&CellContent>) -> i32 {
    let mut result = -1000000;
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
    if result == -1000000 {
        return 0;
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



/***************************************************************************
*                       Unit Tests for evaluation function                 *
****************************************************************************

The following functions are unit tests for the evaluation function. Each test
case is based on a newly created board, with several cases inside.        */


// 5 in-a-row
#[test]
fn evaluate_horizontal_5_in_a_row_1_ally() {
    let result;
    let target_result = 1000000;
    let mut board: Board = Board::new();
    
    board.board[0][0] = CellContent::Ally;
    board.board[0][1] = CellContent::Ally;
    board.board[0][2] = CellContent::Ally;
    board.board[0][3] = CellContent::Ally;
    board.board[0][4] = CellContent::Ally;

    result = evaluate(&board);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_horizontal_5_in_a_row_2_ally() {
    let result;
    let target_result = 1000000;
    let mut board: Board = Board::new();

    board.board[10][0] = CellContent::Ally;
    board.board[10][1] = CellContent::Ally;
    board.board[10][2] = CellContent::Ally;
    board.board[10][3] = CellContent::Ally;
    board.board[10][4] = CellContent::Ally;

    result = evaluate(&board);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_horizontal_5_in_a_row_3_ally() {
    let result;
    let target_result = 1000000;
    let mut board: Board = Board::new();

    board.board[19][0] = CellContent::Ally;
    board.board[19][1] = CellContent::Ally;
    board.board[19][2] = CellContent::Ally;
    board.board[19][3] = CellContent::Ally;
    board.board[19][4] = CellContent::Ally;

    result = evaluate(&board);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_horizontal_5_in_a_row_4_ally() {
    let result;
    let target_result = 1000000;
    let mut board: Board = Board::new();

    board.board[0][10] = CellContent::Ally;
    board.board[0][11] = CellContent::Ally;
    board.board[0][12] = CellContent::Ally;
    board.board[0][13] = CellContent::Ally;
    board.board[0][14] = CellContent::Ally;

    result = evaluate(&board);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_horizontal_5_in_a_row_5_ally() {
    let result;
    let target_result = 1000000;
    let mut board: Board = Board::new();

    board.board[10][10] = CellContent::Ally;
    board.board[10][11] = CellContent::Ally;
    board.board[10][12] = CellContent::Ally;
    board.board[10][13] = CellContent::Ally;
    board.board[10][14] = CellContent::Ally;

    result = evaluate(&board);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_vertical_5_in_a_row_1_ally() {
    let result;
    let target_result = 1000000;
    let mut board: Board = Board::new();

    board.board[0][0] = CellContent::Ally;
    board.board[1][0] = CellContent::Ally;
    board.board[2][0] = CellContent::Ally;
    board.board[3][0] = CellContent::Ally;
    board.board[4][0] = CellContent::Ally;

    result = evaluate(&board);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_vertical_5_in_a_row_2_ally() {
    let result;
    let target_result = 1000000;
    let mut board: Board = Board::new();

    board.board[0][10] = CellContent::Ally;
    board.board[1][10] = CellContent::Ally;
    board.board[2][10] = CellContent::Ally;
    board.board[3][10] = CellContent::Ally;
    board.board[4][10] = CellContent::Ally;

    result = evaluate(&board);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_vertical_5_in_a_row_3_ally() {
    let result;
    let target_result = 1000000;
    let mut board: Board = Board::new();

    board.board[10][0] = CellContent::Ally;
    board.board[11][0] = CellContent::Ally;
    board.board[12][0] = CellContent::Ally;
    board.board[13][0] = CellContent::Ally;
    board.board[14][0] = CellContent::Ally;

    result = evaluate(&board);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_vertical_5_in_a_row_4_ally() {
    let result;
    let target_result = 1000000;
    let mut board: Board = Board::new();

    board.board[0][0] = CellContent::Ally;
    board.board[1][0] = CellContent::Ally;
    board.board[2][0] = CellContent::Ally;
    board.board[3][0] = CellContent::Ally;
    board.board[4][0] = CellContent::Ally;

    result = evaluate(&board);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_vertical_5_in_a_row_5_ally() {
    let result;
    let target_result = 1000000;
    let mut board: Board = Board::new();

    board.board[10][10] = CellContent::Ally;
    board.board[11][10] = CellContent::Ally;
    board.board[12][10] = CellContent::Ally;
    board.board[13][10] = CellContent::Ally;
    board.board[14][10] = CellContent::Ally;

    result = evaluate(&board);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_diag_up_right_5_in_a_row_1_ally() {
    let result;
    let target_result = 1000000;
    let mut board: Board = Board::new();

    board.board[4][0] = CellContent::Ally;
    board.board[3][1] = CellContent::Ally;
    board.board[2][2] = CellContent::Ally;
    board.board[1][3] = CellContent::Ally;
    board.board[0][4] = CellContent::Ally;

    result = evaluate(&board);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_diag_up_right_5_in_a_row_2_ally() {
    let result;
    let target_result = 1000000;
    let mut board: Board = Board::new();

    board.board[14][0] = CellContent::Ally;
    board.board[13][1] = CellContent::Ally;
    board.board[12][2] = CellContent::Ally;
    board.board[11][3] = CellContent::Ally;
    board.board[10][4] = CellContent::Ally;

    result = evaluate(&board);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_diag_up_right_5_in_a_row_3_ally() {
    let result;
    let target_result = 1000000;
    let mut board: Board = Board::new();

    board.board[4][10] = CellContent::Ally;
    board.board[3][11] = CellContent::Ally;
    board.board[2][12] = CellContent::Ally;
    board.board[1][13] = CellContent::Ally;
    board.board[0][14] = CellContent::Ally;

    result = evaluate(&board);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_diag_up_right_5_in_a_row_4_ally() {
    let result;
    let target_result = 1000000;
    let mut board: Board = Board::new();

    board.board[14][10] = CellContent::Ally;
    board.board[13][11] = CellContent::Ally;
    board.board[12][12] = CellContent::Ally;
    board.board[11][13] = CellContent::Ally;
    board.board[10][14] = CellContent::Ally;

    result = evaluate(&board);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_diag_up_right_5_in_a_row_5_ally() {
    let result;
    let target_result = 1000000;
    let mut board: Board = Board::new();

    board.board[19][0] = CellContent::Ally;
    board.board[18][1] = CellContent::Ally;
    board.board[17][2] = CellContent::Ally;
    board.board[16][3] = CellContent::Ally;
    board.board[15][4] = CellContent::Ally;

    result = evaluate(&board);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_diag_up_left_5_in_a_row_1_ally() {
    let result;
    let target_result = 1000000;
    let mut board: Board = Board::new();

    board.board[0][0] = CellContent::Ally;
    board.board[1][1] = CellContent::Ally;
    board.board[2][2] = CellContent::Ally;
    board.board[3][3] = CellContent::Ally;
    board.board[4][4] = CellContent::Ally;
    
    result = evaluate(&board);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_diag_up_left_5_in_a_row_2_ally() {
    let result;
    let target_result = 1000000;
    let mut board: Board = Board::new();

    board.board[10][0] = CellContent::Ally;
    board.board[11][1] = CellContent::Ally;
    board.board[12][2] = CellContent::Ally;
    board.board[13][3] = CellContent::Ally;
    board.board[14][4] = CellContent::Ally;
    
    result = evaluate(&board);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_diag_up_left_5_in_a_row_3_ally() {
    let result;
    let target_result = 1000000;
    let mut board: Board = Board::new();

    board.board[0][10] = CellContent::Ally;
    board.board[1][11] = CellContent::Ally;
    board.board[2][12] = CellContent::Ally;
    board.board[3][13] = CellContent::Ally;
    board.board[4][14] = CellContent::Ally;
    
    result = evaluate(&board);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_diag_up_left_5_in_a_row_4_ally() {
    let result;
    let target_result = 1000000;
    let mut board: Board = Board::new();

    board.board[10][10] = CellContent::Ally;
    board.board[11][11] = CellContent::Ally;
    board.board[12][12] = CellContent::Ally;
    board.board[13][13] = CellContent::Ally;
    board.board[14][14] = CellContent::Ally;
    
    result = evaluate(&board);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_diag_up_left_5_in_a_row_5_ally() {
    let result;
    let target_result = 1000000;
    let mut board: Board = Board::new();

    board.board[15][10] = CellContent::Ally;
    board.board[16][11] = CellContent::Ally;
    board.board[17][12] = CellContent::Ally;
    board.board[18][13] = CellContent::Ally;
    board.board[19][14] = CellContent::Ally;
    
    result = evaluate(&board);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_horizontal_5_in_a_row_1_opp() {
    let result;
    let target_result = -1000000;
    let mut board: Board = Board::new();
    
    board.board[0][0] = CellContent::Opponent;
    board.board[0][1] = CellContent::Opponent;
    board.board[0][2] = CellContent::Opponent;
    board.board[0][3] = CellContent::Opponent;
    board.board[0][4] = CellContent::Opponent;

    result = evaluate(&board);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_horizontal_5_in_a_row_2_opp() {
    let result;
    let target_result = -1000000;
    let mut board: Board = Board::new();

    board.board[10][0] = CellContent::Opponent;
    board.board[10][1] = CellContent::Opponent;
    board.board[10][2] = CellContent::Opponent;
    board.board[10][3] = CellContent::Opponent;
    board.board[10][4] = CellContent::Opponent;

    result = evaluate(&board);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_horizontal_5_in_a_row_3_opp() {
    let result;
    let target_result = -1000000;
    let mut board: Board = Board::new();

    board.board[19][0] = CellContent::Opponent;
    board.board[19][1] = CellContent::Opponent;
    board.board[19][2] = CellContent::Opponent;
    board.board[19][3] = CellContent::Opponent;
    board.board[19][4] = CellContent::Opponent;

    result = evaluate(&board);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_horizontal_5_in_a_row_4_opp() {
    let result;
    let target_result = -1000000;
    let mut board: Board = Board::new();

    board.board[0][10] = CellContent::Opponent;
    board.board[0][11] = CellContent::Opponent;
    board.board[0][12] = CellContent::Opponent;
    board.board[0][13] = CellContent::Opponent;
    board.board[0][14] = CellContent::Opponent;

    result = evaluate(&board);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_horizontal_5_in_a_row_5_opp() {
    let result;
    let target_result = -1000000;
    let mut board: Board = Board::new();

    board.board[10][10] = CellContent::Opponent;
    board.board[10][11] = CellContent::Opponent;
    board.board[10][12] = CellContent::Opponent;
    board.board[10][13] = CellContent::Opponent;
    board.board[10][14] = CellContent::Opponent;

    result = evaluate(&board);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_vertical_5_in_a_row_1_opp() {
    let result;
    let target_result = -1000000;
    let mut board: Board = Board::new();

    board.board[0][0] = CellContent::Opponent;
    board.board[1][0] = CellContent::Opponent;
    board.board[2][0] = CellContent::Opponent;
    board.board[3][0] = CellContent::Opponent;
    board.board[4][0] = CellContent::Opponent;

    result = evaluate(&board);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_vertical_5_in_a_row_2_opp() {
    let result;
    let target_result = -1000000;
    let mut board: Board = Board::new();

    board.board[0][10] = CellContent::Opponent;
    board.board[1][10] = CellContent::Opponent;
    board.board[2][10] = CellContent::Opponent;
    board.board[3][10] = CellContent::Opponent;
    board.board[4][10] = CellContent::Opponent;

    result = evaluate(&board);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_vertical_5_in_a_row_3_opp() {
    let result;
    let target_result = -1000000;
    let mut board: Board = Board::new();

    board.board[10][0] = CellContent::Opponent;
    board.board[11][0] = CellContent::Opponent;
    board.board[12][0] = CellContent::Opponent;
    board.board[13][0] = CellContent::Opponent;
    board.board[14][0] = CellContent::Opponent;

    result = evaluate(&board);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_vertical_5_in_a_row_4_opp() {
    let result;
    let target_result = -1000000;
    let mut board: Board = Board::new();

    board.board[0][0] = CellContent::Opponent;
    board.board[1][0] = CellContent::Opponent;
    board.board[2][0] = CellContent::Opponent;
    board.board[3][0] = CellContent::Opponent;
    board.board[4][0] = CellContent::Opponent;

    result = evaluate(&board);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_vertical_5_in_a_row_5_opp() {
    let result;
    let target_result = -1000000;
    let mut board: Board = Board::new();

    board.board[10][10] = CellContent::Opponent;
    board.board[11][10] = CellContent::Opponent;
    board.board[12][10] = CellContent::Opponent;
    board.board[13][10] = CellContent::Opponent;
    board.board[14][10] = CellContent::Opponent;

    result = evaluate(&board);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_diag_up_right_5_in_a_row_1_opp() {
    let result;
    let target_result = -1000000;
    let mut board: Board = Board::new();

    board.board[4][0] = CellContent::Opponent;
    board.board[3][1] = CellContent::Opponent;
    board.board[2][2] = CellContent::Opponent;
    board.board[1][3] = CellContent::Opponent;
    board.board[0][4] = CellContent::Opponent;

    result = evaluate(&board);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_diag_up_right_5_in_a_row_2_opp() {
    let result;
    let target_result = -1000000;
    let mut board: Board = Board::new();

    board.board[14][0] = CellContent::Opponent;
    board.board[13][1] = CellContent::Opponent;
    board.board[12][2] = CellContent::Opponent;
    board.board[11][3] = CellContent::Opponent;
    board.board[10][4] = CellContent::Opponent;

    result = evaluate(&board);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_diag_up_right_5_in_a_row_3_opp() {
    let result;
    let target_result = -1000000;
    let mut board: Board = Board::new();

    board.board[4][10] = CellContent::Opponent;
    board.board[3][11] = CellContent::Opponent;
    board.board[2][12] = CellContent::Opponent;
    board.board[1][13] = CellContent::Opponent;
    board.board[0][14] = CellContent::Opponent;

    result = evaluate(&board);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_diag_up_right_5_in_a_row_4_opp() {
    let result;
    let target_result = -1000000;
    let mut board: Board = Board::new();

    board.board[14][10] = CellContent::Opponent;
    board.board[13][11] = CellContent::Opponent;
    board.board[12][12] = CellContent::Opponent;
    board.board[11][13] = CellContent::Opponent;
    board.board[10][14] = CellContent::Opponent;

    result = evaluate(&board);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_diag_up_right_5_in_a_row_5_opp() {
    let result;
    let target_result = -1000000;
    let mut board: Board = Board::new();

    board.board[19][0] = CellContent::Opponent;
    board.board[18][1] = CellContent::Opponent;
    board.board[17][2] = CellContent::Opponent;
    board.board[16][3] = CellContent::Opponent;
    board.board[15][4] = CellContent::Opponent;

    result = evaluate(&board);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_diag_up_left_5_in_a_row_1_opp() {
    let result;
    let target_result = -1000000;
    let mut board: Board = Board::new();

    board.board[0][0] = CellContent::Opponent;
    board.board[1][1] = CellContent::Opponent;
    board.board[2][2] = CellContent::Opponent;
    board.board[3][3] = CellContent::Opponent;
    board.board[4][4] = CellContent::Opponent;
    
    result = evaluate(&board);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_diag_up_left_5_in_a_row_2_opp() {
    let result;
    let target_result = -1000000;
    let mut board: Board = Board::new();

    board.board[10][0] = CellContent::Opponent;
    board.board[11][1] = CellContent::Opponent;
    board.board[12][2] = CellContent::Opponent;
    board.board[13][3] = CellContent::Opponent;
    board.board[14][4] = CellContent::Opponent;
    
    result = evaluate(&board);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_diag_up_left_5_in_a_row_3_opp() {
    let result;
    let target_result = -1000000;
    let mut board: Board = Board::new();

    board.board[0][10] = CellContent::Opponent;
    board.board[1][11] = CellContent::Opponent;
    board.board[2][12] = CellContent::Opponent;
    board.board[3][13] = CellContent::Opponent;
    board.board[4][14] = CellContent::Opponent;
    
    result = evaluate(&board);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_diag_up_left_5_in_a_row_4_opp() {
    let result;
    let target_result = -1000000;
    let mut board: Board = Board::new();

    board.board[10][10] = CellContent::Opponent;
    board.board[11][11] = CellContent::Opponent;
    board.board[12][12] = CellContent::Opponent;
    board.board[13][13] = CellContent::Opponent;
    board.board[14][14] = CellContent::Opponent;
    
    result = evaluate(&board);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_diag_up_left_5_in_a_row_5_opp() {
    let result;
    let target_result = -1000000;
    let mut board: Board = Board::new();

    board.board[15][10] = CellContent::Opponent;
    board.board[16][11] = CellContent::Opponent;
    board.board[17][12] = CellContent::Opponent;
    board.board[18][13] = CellContent::Opponent;
    board.board[19][14] = CellContent::Opponent;
    
    result = evaluate(&board);

    assert_eq!(result, target_result);
}

// 4 in-a-row
#[test]
fn evaluate_horizontal_4_in_a_row_1_ally() {
    let result;
    let target_result = 5000;
    let mut board: Board = Board::new();
    
    board.board[0][1] = CellContent::Ally;
    board.board[0][2] = CellContent::Ally;
    board.board[0][3] = CellContent::Ally;
    board.board[0][4] = CellContent::Ally;

    result = evaluate(&board);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_horizontal_4_in_a_row_2_ally() {
    let result;
    let target_result = 5000;
    let mut board: Board = Board::new();

    board.board[10][1] = CellContent::Ally;
    board.board[10][2] = CellContent::Ally;
    board.board[10][3] = CellContent::Ally;
    board.board[10][4] = CellContent::Ally;

    result = evaluate(&board);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_horizontal_4_in_a_row_3_ally() {
    let result;
    let target_result = 5000;
    let mut board: Board = Board::new();

    board.board[19][1] = CellContent::Ally;
    board.board[19][2] = CellContent::Ally;
    board.board[19][3] = CellContent::Ally;
    board.board[19][4] = CellContent::Ally;

    result = evaluate(&board);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_horizontal_4_in_a_row_4_ally() {
    let result;
    let target_result = 5000;
    let mut board: Board = Board::new();

    board.board[0][11] = CellContent::Ally;
    board.board[0][12] = CellContent::Ally;
    board.board[0][13] = CellContent::Ally;
    board.board[0][14] = CellContent::Ally;

    result = evaluate(&board);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_horizontal_4_in_a_row_5_ally() {
    let result;
    let target_result = 5000;
    let mut board: Board = Board::new();

    board.board[10][11] = CellContent::Ally;
    board.board[10][12] = CellContent::Ally;
    board.board[10][13] = CellContent::Ally;
    board.board[10][14] = CellContent::Ally;

    result = evaluate(&board);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_vertical_4_in_a_row_1_ally() {
    let result;
    let target_result = 5000;
    let mut board: Board = Board::new();

    board.board[1][0] = CellContent::Ally;
    board.board[2][0] = CellContent::Ally;
    board.board[3][0] = CellContent::Ally;
    board.board[4][0] = CellContent::Ally;

    result = evaluate(&board);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_vertical_4_in_a_row_2_ally() {
    let result;
    let target_result = 5000;
    let mut board: Board = Board::new();

    board.board[1][10] = CellContent::Ally;
    board.board[2][10] = CellContent::Ally;
    board.board[3][10] = CellContent::Ally;
    board.board[4][10] = CellContent::Ally;

    result = evaluate(&board);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_vertical_4_in_a_row_3_ally() {
    let result;
    let target_result = 5000;
    let mut board: Board = Board::new();

    board.board[11][0] = CellContent::Ally;
    board.board[12][0] = CellContent::Ally;
    board.board[13][0] = CellContent::Ally;
    board.board[14][0] = CellContent::Ally;

    result = evaluate(&board);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_vertical_4_in_a_row_4_ally() {
    let result;
    let target_result = 5000;
    let mut board: Board = Board::new();

    board.board[1][0] = CellContent::Ally;
    board.board[2][0] = CellContent::Ally;
    board.board[3][0] = CellContent::Ally;
    board.board[4][0] = CellContent::Ally;

    result = evaluate(&board);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_vertical_4_in_a_row_5_ally() {
    let result;
    let target_result = 5000;
    let mut board: Board = Board::new();

    board.board[11][10] = CellContent::Ally;
    board.board[12][10] = CellContent::Ally;
    board.board[13][10] = CellContent::Ally;
    board.board[14][10] = CellContent::Ally;

    result = evaluate(&board);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_diag_up_right_4_in_a_row_1_ally() {
    let result;
    let target_result = 5000;
    let mut board: Board = Board::new();

    board.board[4][1] = CellContent::Ally;
    board.board[3][2] = CellContent::Ally;
    board.board[2][3] = CellContent::Ally;
    board.board[1][4] = CellContent::Ally;

    result = evaluate(&board);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_diag_up_right_4_in_a_row_2_ally() {
    let result;
    let target_result = 5000;
    let mut board: Board = Board::new();

    board.board[13][1] = CellContent::Ally;
    board.board[12][2] = CellContent::Ally;
    board.board[11][3] = CellContent::Ally;
    board.board[10][4] = CellContent::Ally;

    result = evaluate(&board);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_diag_up_right_4_in_a_row_3_ally() {
    let result;
    let target_result = 5000;
    let mut board: Board = Board::new();

    board.board[4][10] = CellContent::Ally;
    board.board[3][11] = CellContent::Ally;
    board.board[2][12] = CellContent::Ally;
    board.board[1][13] = CellContent::Ally;

    result = evaluate(&board);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_diag_up_right_4_in_a_row_4_ally() {
    let result;
    let target_result = 5000;
    let mut board: Board = Board::new();

    board.board[13][11] = CellContent::Ally;
    board.board[12][12] = CellContent::Ally;
    board.board[11][13] = CellContent::Ally;
    board.board[10][14] = CellContent::Ally;

    result = evaluate(&board);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_diag_up_right_4_in_a_row_5_ally() {
    let result;
    let target_result = 5000;
    let mut board: Board = Board::new();

    board.board[18][1] = CellContent::Ally;
    board.board[17][2] = CellContent::Ally;
    board.board[16][3] = CellContent::Ally;
    board.board[15][4] = CellContent::Ally;

    result = evaluate(&board);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_diag_up_left_4_in_a_row_1_ally() {
    let result;
    let target_result = 5000;
    let mut board: Board = Board::new();

    board.board[1][1] = CellContent::Ally;
    board.board[2][2] = CellContent::Ally;
    board.board[3][3] = CellContent::Ally;
    board.board[4][4] = CellContent::Ally;
    
    result = evaluate(&board);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_diag_up_left_4_in_a_row_2_ally() {
    let result;
    let target_result = 5000;
    let mut board: Board = Board::new();

    board.board[11][1] = CellContent::Ally;
    board.board[12][2] = CellContent::Ally;
    board.board[13][3] = CellContent::Ally;
    board.board[14][4] = CellContent::Ally;
    
    result = evaluate(&board);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_diag_up_left_4_in_a_row_3_ally() {
    let result;
    let target_result = 5000;
    let mut board: Board = Board::new();

    board.board[1][11] = CellContent::Ally;
    board.board[2][12] = CellContent::Ally;
    board.board[3][13] = CellContent::Ally;
    board.board[4][14] = CellContent::Ally;
    
    result = evaluate(&board);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_diag_up_left_4_in_a_row_4_ally() {
    let result;
    let target_result = 5000;
    let mut board: Board = Board::new();

    board.board[11][11] = CellContent::Ally;
    board.board[12][12] = CellContent::Ally;
    board.board[13][13] = CellContent::Ally;
    board.board[14][14] = CellContent::Ally;
    
    result = evaluate(&board);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_diag_up_left_4_in_a_row_5_ally() {
    let result;
    let target_result = 5000;
    let mut board: Board = Board::new();

    board.board[15][10] = CellContent::Ally;
    board.board[16][11] = CellContent::Ally;
    board.board[17][12] = CellContent::Ally;
    board.board[18][13] = CellContent::Ally;
    
    result = evaluate(&board);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_horizontal_4_in_a_row_1_opp() {
    let result;
    let target_result = /* -5000 To check */ -1000;
    let mut board: Board = Board::new();
    
    board.board[0][1] = CellContent::Opponent;
    board.board[0][2] = CellContent::Opponent;
    board.board[0][3] = CellContent::Opponent;
    board.board[0][4] = CellContent::Opponent;

    result = evaluate(&board);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_horizontal_4_in_a_row_2_opp() {
    let result;
    let target_result = /* -5000 To check */ -1000;
    let mut board: Board = Board::new();

    board.board[10][1] = CellContent::Opponent;
    board.board[10][2] = CellContent::Opponent;
    board.board[10][3] = CellContent::Opponent;
    board.board[10][4] = CellContent::Opponent;

    result = evaluate(&board);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_horizontal_4_in_a_row_3_opp() {
    let result;
    let target_result = /* -5000 To check */ -1000;
    let mut board: Board = Board::new();

    board.board[19][1] = CellContent::Opponent;
    board.board[19][2] = CellContent::Opponent;
    board.board[19][3] = CellContent::Opponent;
    board.board[19][4] = CellContent::Opponent;

    result = evaluate(&board);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_horizontal_4_in_a_row_4_opp() {
    let result;
    let target_result = /* -5000 To check */ -1000;
    let mut board: Board = Board::new();

    board.board[0][11] = CellContent::Opponent;
    board.board[0][12] = CellContent::Opponent;
    board.board[0][13] = CellContent::Opponent;
    board.board[0][14] = CellContent::Opponent;

    result = evaluate(&board);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_horizontal_4_in_a_row_5_opp() {
    let result;
    let target_result = /* -5000 To check */ -1000;
    let mut board: Board = Board::new();

    board.board[10][11] = CellContent::Opponent;
    board.board[10][12] = CellContent::Opponent;
    board.board[10][13] = CellContent::Opponent;
    board.board[10][14] = CellContent::Opponent;

    result = evaluate(&board);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_vertical_4_in_a_row_1_opp() {
    let result;
    let target_result = /* -5000 To check */ -1000;
    let mut board: Board = Board::new();

    board.board[1][0] = CellContent::Opponent;
    board.board[2][0] = CellContent::Opponent;
    board.board[3][0] = CellContent::Opponent;
    board.board[4][0] = CellContent::Opponent;

    result = evaluate(&board);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_vertical_4_in_a_row_2_opp() {
    let result;
    let target_result = /* -5000 To check */ -1000;
    let mut board: Board = Board::new();

    board.board[1][10] = CellContent::Opponent;
    board.board[2][10] = CellContent::Opponent;
    board.board[3][10] = CellContent::Opponent;
    board.board[4][10] = CellContent::Opponent;

    result = evaluate(&board);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_vertical_4_in_a_row_3_opp() {
    let result;
    let target_result = /* -5000 To check */ -1000;
    let mut board: Board = Board::new();

    board.board[11][0] = CellContent::Opponent;
    board.board[12][0] = CellContent::Opponent;
    board.board[13][0] = CellContent::Opponent;
    board.board[14][0] = CellContent::Opponent;

    result = evaluate(&board);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_vertical_4_in_a_row_4_opp() {
    let result;
    let target_result = /* -5000 To check */ -1000;
    let mut board: Board = Board::new();

    board.board[1][0] = CellContent::Opponent;
    board.board[2][0] = CellContent::Opponent;
    board.board[3][0] = CellContent::Opponent;
    board.board[4][0] = CellContent::Opponent;

    result = evaluate(&board);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_vertical_4_in_a_row_5_opp() {
    let result;
    let target_result = /* -5000 To check */ -1000;
    let mut board: Board = Board::new();

    board.board[11][10] = CellContent::Opponent;
    board.board[12][10] = CellContent::Opponent;
    board.board[13][10] = CellContent::Opponent;
    board.board[14][10] = CellContent::Opponent;

    result = evaluate(&board);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_diag_up_right_4_in_a_row_1_opp() {
    let result;
    let target_result = -5000;
    let mut board: Board = Board::new();

    board.board[4][1] = CellContent::Opponent;
    board.board[3][2] = CellContent::Opponent;
    board.board[2][3] = CellContent::Opponent;
    board.board[1][4] = CellContent::Opponent;

    result = evaluate(&board);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_diag_up_right_4_in_a_row_2_opp() {
    let result;
    let target_result = /* -5000 To check */ -1000;
    let mut board: Board = Board::new();

    board.board[13][1] = CellContent::Opponent;
    board.board[12][2] = CellContent::Opponent;
    board.board[11][3] = CellContent::Opponent;
    board.board[10][4] = CellContent::Opponent;

    result = evaluate(&board);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_diag_up_right_4_in_a_row_3_opp() {
    let result;
    let target_result = /* -5000 To check */ -1000;
    let mut board: Board = Board::new();

    board.board[4][10] = CellContent::Opponent;
    board.board[3][11] = CellContent::Opponent;
    board.board[2][12] = CellContent::Opponent;
    board.board[1][13] = CellContent::Opponent;

    result = evaluate(&board);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_diag_up_right_4_in_a_row_4_opp() {
    let result;
    let target_result = /* -5000 To check */ -1000;
    let mut board: Board = Board::new();

    board.board[13][11] = CellContent::Opponent;
    board.board[12][12] = CellContent::Opponent;
    board.board[11][13] = CellContent::Opponent;
    board.board[10][14] = CellContent::Opponent;

    result = evaluate(&board);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_diag_up_right_4_in_a_row_5_opp() {
    let result;
    let target_result = /* -5000 To check */ -1000;
    let mut board: Board = Board::new();

    board.board[18][1] = CellContent::Opponent;
    board.board[17][2] = CellContent::Opponent;
    board.board[16][3] = CellContent::Opponent;
    board.board[15][4] = CellContent::Opponent;

    result = evaluate(&board);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_diag_up_left_4_in_a_row_1_opp() {
    let result;
    let target_result = /* -5000 To check */ -1000;
    let mut board: Board = Board::new();

    board.board[1][1] = CellContent::Opponent;
    board.board[2][2] = CellContent::Opponent;
    board.board[3][3] = CellContent::Opponent;
    board.board[4][4] = CellContent::Opponent;
    
    result = evaluate(&board);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_diag_up_left_4_in_a_row_2_opp() {
    let result;
    let target_result = /* -5000 To check */ -1000;
    let mut board: Board = Board::new();

    board.board[11][1] = CellContent::Opponent;
    board.board[12][2] = CellContent::Opponent;
    board.board[13][3] = CellContent::Opponent;
    board.board[14][4] = CellContent::Opponent;
    
    result = evaluate(&board);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_diag_up_left_4_in_a_row_3_opp() {
    let result;
    let target_result = /* -5000 To check */ -1000;
    let mut board: Board = Board::new();

    board.board[1][11] = CellContent::Opponent;
    board.board[2][12] = CellContent::Opponent;
    board.board[3][13] = CellContent::Opponent;
    board.board[4][14] = CellContent::Opponent;
    
    result = evaluate(&board);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_diag_up_left_4_in_a_row_4_opp() {
    let result;
    let target_result = /* -5000 To check */ -1000;
    let mut board: Board = Board::new();

    board.board[11][11] = CellContent::Opponent;
    board.board[12][12] = CellContent::Opponent;
    board.board[13][13] = CellContent::Opponent;
    board.board[14][14] = CellContent::Opponent;
    
    result = evaluate(&board);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_diag_up_left_4_in_a_row_5_opp() {
    let result;
    let target_result = /* -5000 To check */ -1000;
    let mut board: Board = Board::new();

    board.board[15][10] = CellContent::Opponent;
    board.board[16][11] = CellContent::Opponent;
    board.board[17][12] = CellContent::Opponent;
    board.board[18][13] = CellContent::Opponent;
    
    result = evaluate(&board);

    assert_eq!(result, target_result);
}