use crate::board::Board;
use crate::evaluation::evaluate;
use crate::evaluation::{FIVE_IN_A_ROW, FOUR_IN_A_ROW_OPEN};
use crate::model::{CellContent, Stone};

// 5 in-a-row
#[test]
fn evaluate_horizontal_5_in_a_row_1_ally() {
    let result;
    let target_result = FIVE_IN_A_ROW;
    let mut board: Board = Board::new();

    board.board[0][0] = CellContent::Ally;
    board.board[0][1] = CellContent::Ally;
    board.board[0][2] = CellContent::Ally;
    board.board[0][3] = CellContent::Ally;
    board.board[0][4] = CellContent::Ally;

    result = evaluate(&board, Stone::Ally);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_horizontal_5_in_a_row_2_ally() {
    let result;
    let target_result = FIVE_IN_A_ROW;
    let mut board: Board = Board::new();

    board.board[10][0] = CellContent::Ally;
    board.board[10][1] = CellContent::Ally;
    board.board[10][2] = CellContent::Ally;
    board.board[10][3] = CellContent::Ally;
    board.board[10][4] = CellContent::Ally;

    result = evaluate(&board, Stone::Ally);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_horizontal_5_in_a_row_3_ally() {
    let result;
    let target_result = FIVE_IN_A_ROW;
    let mut board: Board = Board::new();

    board.board[19][0] = CellContent::Ally;
    board.board[19][1] = CellContent::Ally;
    board.board[19][2] = CellContent::Ally;
    board.board[19][3] = CellContent::Ally;
    board.board[19][4] = CellContent::Ally;

    result = evaluate(&board, Stone::Ally);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_horizontal_5_in_a_row_4_ally() {
    let result;
    let target_result = FIVE_IN_A_ROW;
    let mut board: Board = Board::new();

    board.board[0][10] = CellContent::Ally;
    board.board[0][11] = CellContent::Ally;
    board.board[0][12] = CellContent::Ally;
    board.board[0][13] = CellContent::Ally;
    board.board[0][14] = CellContent::Ally;

    result = evaluate(&board, Stone::Ally);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_horizontal_5_in_a_row_5_ally() {
    let result;
    let target_result = FIVE_IN_A_ROW;
    let mut board: Board = Board::new();

    board.board[10][10] = CellContent::Ally;
    board.board[10][11] = CellContent::Ally;
    board.board[10][12] = CellContent::Ally;
    board.board[10][13] = CellContent::Ally;
    board.board[10][14] = CellContent::Ally;

    result = evaluate(&board, Stone::Ally);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_vertical_5_in_a_row_1_ally() {
    let result;
    let target_result = FIVE_IN_A_ROW;
    let mut board: Board = Board::new();

    board.board[0][0] = CellContent::Ally;
    board.board[1][0] = CellContent::Ally;
    board.board[2][0] = CellContent::Ally;
    board.board[3][0] = CellContent::Ally;
    board.board[4][0] = CellContent::Ally;

    result = evaluate(&board, Stone::Ally);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_vertical_5_in_a_row_2_ally() {
    let result;
    let target_result = FIVE_IN_A_ROW;
    let mut board: Board = Board::new();

    board.board[0][10] = CellContent::Ally;
    board.board[1][10] = CellContent::Ally;
    board.board[2][10] = CellContent::Ally;
    board.board[3][10] = CellContent::Ally;
    board.board[4][10] = CellContent::Ally;

    result = evaluate(&board, Stone::Ally);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_vertical_5_in_a_row_3_ally() {
    let result;
    let target_result = FIVE_IN_A_ROW;
    let mut board: Board = Board::new();

    board.board[10][0] = CellContent::Ally;
    board.board[11][0] = CellContent::Ally;
    board.board[12][0] = CellContent::Ally;
    board.board[13][0] = CellContent::Ally;
    board.board[14][0] = CellContent::Ally;

    result = evaluate(&board, Stone::Ally);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_vertical_5_in_a_row_4_ally() {
    let result;
    let target_result = FIVE_IN_A_ROW;
    let mut board: Board = Board::new();

    board.board[0][0] = CellContent::Ally;
    board.board[1][0] = CellContent::Ally;
    board.board[2][0] = CellContent::Ally;
    board.board[3][0] = CellContent::Ally;
    board.board[4][0] = CellContent::Ally;

    result = evaluate(&board, Stone::Ally);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_vertical_5_in_a_row_5_ally() {
    let result;
    let target_result = FIVE_IN_A_ROW;
    let mut board: Board = Board::new();

    board.board[10][10] = CellContent::Ally;
    board.board[11][10] = CellContent::Ally;
    board.board[12][10] = CellContent::Ally;
    board.board[13][10] = CellContent::Ally;
    board.board[14][10] = CellContent::Ally;

    result = evaluate(&board, Stone::Ally);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_diag_up_right_5_in_a_row_1_ally() {
    let result;
    let target_result = FIVE_IN_A_ROW;
    let mut board: Board = Board::new();

    board.board[4][0] = CellContent::Ally;
    board.board[3][1] = CellContent::Ally;
    board.board[2][2] = CellContent::Ally;
    board.board[1][3] = CellContent::Ally;
    board.board[0][4] = CellContent::Ally;

    result = evaluate(&board, Stone::Ally);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_diag_up_right_5_in_a_row_2_ally() {
    let result;
    let target_result = FIVE_IN_A_ROW;
    let mut board: Board = Board::new();

    board.board[14][0] = CellContent::Ally;
    board.board[13][1] = CellContent::Ally;
    board.board[12][2] = CellContent::Ally;
    board.board[11][3] = CellContent::Ally;
    board.board[10][4] = CellContent::Ally;

    result = evaluate(&board, Stone::Ally);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_diag_up_right_5_in_a_row_3_ally() {
    let result;
    let target_result = FIVE_IN_A_ROW;
    let mut board: Board = Board::new();

    board.board[4][10] = CellContent::Ally;
    board.board[3][11] = CellContent::Ally;
    board.board[2][12] = CellContent::Ally;
    board.board[1][13] = CellContent::Ally;
    board.board[0][14] = CellContent::Ally;

    result = evaluate(&board, Stone::Ally);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_diag_up_right_5_in_a_row_4_ally() {
    let result;
    let target_result = FIVE_IN_A_ROW;
    let mut board: Board = Board::new();

    board.board[14][10] = CellContent::Ally;
    board.board[13][11] = CellContent::Ally;
    board.board[12][12] = CellContent::Ally;
    board.board[11][13] = CellContent::Ally;
    board.board[10][14] = CellContent::Ally;

    result = evaluate(&board, Stone::Ally);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_diag_up_right_5_in_a_row_5_ally() {
    let result;
    let target_result = FIVE_IN_A_ROW;
    let mut board: Board = Board::new();

    board.board[19][0] = CellContent::Ally;
    board.board[18][1] = CellContent::Ally;
    board.board[17][2] = CellContent::Ally;
    board.board[16][3] = CellContent::Ally;
    board.board[15][4] = CellContent::Ally;

    result = evaluate(&board, Stone::Ally);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_diag_up_left_5_in_a_row_1_ally() {
    let result;
    let target_result = FIVE_IN_A_ROW;
    let mut board: Board = Board::new();

    board.board[0][0] = CellContent::Ally;
    board.board[1][1] = CellContent::Ally;
    board.board[2][2] = CellContent::Ally;
    board.board[3][3] = CellContent::Ally;
    board.board[4][4] = CellContent::Ally;

    result = evaluate(&board, Stone::Ally);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_diag_up_left_5_in_a_row_2_ally() {
    let result;
    let target_result = FIVE_IN_A_ROW;
    let mut board: Board = Board::new();

    board.board[10][0] = CellContent::Ally;
    board.board[11][1] = CellContent::Ally;
    board.board[12][2] = CellContent::Ally;
    board.board[13][3] = CellContent::Ally;
    board.board[14][4] = CellContent::Ally;

    result = evaluate(&board, Stone::Ally);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_diag_up_left_5_in_a_row_3_ally() {
    let result;
    let target_result = FIVE_IN_A_ROW;
    let mut board: Board = Board::new();

    board.board[0][10] = CellContent::Ally;
    board.board[1][11] = CellContent::Ally;
    board.board[2][12] = CellContent::Ally;
    board.board[3][13] = CellContent::Ally;
    board.board[4][14] = CellContent::Ally;

    result = evaluate(&board, Stone::Ally);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_diag_up_left_5_in_a_row_4_ally() {
    let result;
    let target_result = FIVE_IN_A_ROW;
    let mut board: Board = Board::new();

    board.board[10][10] = CellContent::Ally;
    board.board[11][11] = CellContent::Ally;
    board.board[12][12] = CellContent::Ally;
    board.board[13][13] = CellContent::Ally;
    board.board[14][14] = CellContent::Ally;

    result = evaluate(&board, Stone::Ally);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_diag_up_left_5_in_a_row_5_ally() {
    let result;
    let target_result = FIVE_IN_A_ROW;
    let mut board: Board = Board::new();

    board.board[15][10] = CellContent::Ally;
    board.board[16][11] = CellContent::Ally;
    board.board[17][12] = CellContent::Ally;
    board.board[18][13] = CellContent::Ally;
    board.board[19][14] = CellContent::Ally;

    result = evaluate(&board, Stone::Ally);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_horizontal_5_in_a_row_1_opp() {
    let result;
    let target_result = -FIVE_IN_A_ROW;
    let mut board: Board = Board::new();

    board.board[0][0] = CellContent::Opponent;
    board.board[0][1] = CellContent::Opponent;
    board.board[0][2] = CellContent::Opponent;
    board.board[0][3] = CellContent::Opponent;
    board.board[0][4] = CellContent::Opponent;

    result = evaluate(&board, Stone::Opponent);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_horizontal_5_in_a_row_2_opp() {
    let result;
    let target_result = -FIVE_IN_A_ROW;
    let mut board: Board = Board::new();

    board.board[10][0] = CellContent::Opponent;
    board.board[10][1] = CellContent::Opponent;
    board.board[10][2] = CellContent::Opponent;
    board.board[10][3] = CellContent::Opponent;
    board.board[10][4] = CellContent::Opponent;

    result = evaluate(&board, Stone::Opponent);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_horizontal_5_in_a_row_3_opp() {
    let result;
    let target_result = -FIVE_IN_A_ROW;
    let mut board: Board = Board::new();

    board.board[19][0] = CellContent::Opponent;
    board.board[19][1] = CellContent::Opponent;
    board.board[19][2] = CellContent::Opponent;
    board.board[19][3] = CellContent::Opponent;
    board.board[19][4] = CellContent::Opponent;

    result = evaluate(&board, Stone::Opponent);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_horizontal_5_in_a_row_4_opp() {
    let result;
    let target_result = -FIVE_IN_A_ROW;
    let mut board: Board = Board::new();

    board.board[0][10] = CellContent::Opponent;
    board.board[0][11] = CellContent::Opponent;
    board.board[0][12] = CellContent::Opponent;
    board.board[0][13] = CellContent::Opponent;
    board.board[0][14] = CellContent::Opponent;

    result = evaluate(&board, Stone::Opponent);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_horizontal_5_in_a_row_5_opp() {
    let result;
    let target_result = -FIVE_IN_A_ROW;
    let mut board: Board = Board::new();

    board.board[10][10] = CellContent::Opponent;
    board.board[10][11] = CellContent::Opponent;
    board.board[10][12] = CellContent::Opponent;
    board.board[10][13] = CellContent::Opponent;
    board.board[10][14] = CellContent::Opponent;

    result = evaluate(&board, Stone::Opponent);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_vertical_5_in_a_row_1_opp() {
    let result;
    let target_result = -FIVE_IN_A_ROW;
    let mut board: Board = Board::new();

    board.board[0][0] = CellContent::Opponent;
    board.board[1][0] = CellContent::Opponent;
    board.board[2][0] = CellContent::Opponent;
    board.board[3][0] = CellContent::Opponent;
    board.board[4][0] = CellContent::Opponent;

    result = evaluate(&board, Stone::Opponent);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_vertical_5_in_a_row_2_opp() {
    let result;
    let target_result = -FIVE_IN_A_ROW;
    let mut board: Board = Board::new();

    board.board[0][10] = CellContent::Opponent;
    board.board[1][10] = CellContent::Opponent;
    board.board[2][10] = CellContent::Opponent;
    board.board[3][10] = CellContent::Opponent;
    board.board[4][10] = CellContent::Opponent;

    result = evaluate(&board, Stone::Opponent);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_vertical_5_in_a_row_3_opp() {
    let result;
    let target_result = -FIVE_IN_A_ROW;
    let mut board: Board = Board::new();

    board.board[10][0] = CellContent::Opponent;
    board.board[11][0] = CellContent::Opponent;
    board.board[12][0] = CellContent::Opponent;
    board.board[13][0] = CellContent::Opponent;
    board.board[14][0] = CellContent::Opponent;

    result = evaluate(&board, Stone::Opponent);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_vertical_5_in_a_row_4_opp() {
    let result;
    let target_result = -FIVE_IN_A_ROW;
    let mut board: Board = Board::new();

    board.board[0][0] = CellContent::Opponent;
    board.board[1][0] = CellContent::Opponent;
    board.board[2][0] = CellContent::Opponent;
    board.board[3][0] = CellContent::Opponent;
    board.board[4][0] = CellContent::Opponent;

    result = evaluate(&board, Stone::Opponent);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_vertical_5_in_a_row_5_opp() {
    let result;
    let target_result = -FIVE_IN_A_ROW;
    let mut board: Board = Board::new();

    board.board[10][10] = CellContent::Opponent;
    board.board[11][10] = CellContent::Opponent;
    board.board[12][10] = CellContent::Opponent;
    board.board[13][10] = CellContent::Opponent;
    board.board[14][10] = CellContent::Opponent;

    result = evaluate(&board, Stone::Opponent);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_diag_up_right_5_in_a_row_1_opp() {
    let result;
    let target_result = -FIVE_IN_A_ROW;
    let mut board: Board = Board::new();

    board.board[4][0] = CellContent::Opponent;
    board.board[3][1] = CellContent::Opponent;
    board.board[2][2] = CellContent::Opponent;
    board.board[1][3] = CellContent::Opponent;
    board.board[0][4] = CellContent::Opponent;

    result = evaluate(&board, Stone::Opponent);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_diag_up_right_5_in_a_row_2_opp() {
    let result;
    let target_result = -FIVE_IN_A_ROW;
    let mut board: Board = Board::new();

    board.board[14][0] = CellContent::Opponent;
    board.board[13][1] = CellContent::Opponent;
    board.board[12][2] = CellContent::Opponent;
    board.board[11][3] = CellContent::Opponent;
    board.board[10][4] = CellContent::Opponent;

    result = evaluate(&board, Stone::Opponent);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_diag_up_right_5_in_a_row_3_opp() {
    let result;
    let target_result = -FIVE_IN_A_ROW;
    let mut board: Board = Board::new();

    board.board[4][10] = CellContent::Opponent;
    board.board[3][11] = CellContent::Opponent;
    board.board[2][12] = CellContent::Opponent;
    board.board[1][13] = CellContent::Opponent;
    board.board[0][14] = CellContent::Opponent;

    result = evaluate(&board, Stone::Opponent);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_diag_up_right_5_in_a_row_4_opp() {
    let result;
    let target_result = -FIVE_IN_A_ROW;
    let mut board: Board = Board::new();

    board.board[14][10] = CellContent::Opponent;
    board.board[13][11] = CellContent::Opponent;
    board.board[12][12] = CellContent::Opponent;
    board.board[11][13] = CellContent::Opponent;
    board.board[10][14] = CellContent::Opponent;

    result = evaluate(&board, Stone::Opponent);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_diag_up_right_5_in_a_row_5_opp() {
    let result;
    let target_result = -FIVE_IN_A_ROW;
    let mut board: Board = Board::new();

    board.board[19][0] = CellContent::Opponent;
    board.board[18][1] = CellContent::Opponent;
    board.board[17][2] = CellContent::Opponent;
    board.board[16][3] = CellContent::Opponent;
    board.board[15][4] = CellContent::Opponent;

    result = evaluate(&board, Stone::Opponent);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_diag_up_left_5_in_a_row_1_opp() {
    let result;
    let target_result = -FIVE_IN_A_ROW;
    let mut board: Board = Board::new();

    board.board[0][0] = CellContent::Opponent;
    board.board[1][1] = CellContent::Opponent;
    board.board[2][2] = CellContent::Opponent;
    board.board[3][3] = CellContent::Opponent;
    board.board[4][4] = CellContent::Opponent;

    result = evaluate(&board, Stone::Opponent);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_diag_up_left_5_in_a_row_2_opp() {
    let result;
    let target_result = -FIVE_IN_A_ROW;
    let mut board: Board = Board::new();

    board.board[10][0] = CellContent::Opponent;
    board.board[11][1] = CellContent::Opponent;
    board.board[12][2] = CellContent::Opponent;
    board.board[13][3] = CellContent::Opponent;
    board.board[14][4] = CellContent::Opponent;

    result = evaluate(&board, Stone::Opponent);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_diag_up_left_5_in_a_row_3_opp() {
    let result;
    let target_result = -FIVE_IN_A_ROW;
    let mut board: Board = Board::new();

    board.board[0][10] = CellContent::Opponent;
    board.board[1][11] = CellContent::Opponent;
    board.board[2][12] = CellContent::Opponent;
    board.board[3][13] = CellContent::Opponent;
    board.board[4][14] = CellContent::Opponent;

    result = evaluate(&board, Stone::Opponent);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_diag_up_left_5_in_a_row_4_opp() {
    let result;
    let target_result = -FIVE_IN_A_ROW;
    let mut board: Board = Board::new();

    board.board[10][10] = CellContent::Opponent;
    board.board[11][11] = CellContent::Opponent;
    board.board[12][12] = CellContent::Opponent;
    board.board[13][13] = CellContent::Opponent;
    board.board[14][14] = CellContent::Opponent;

    result = evaluate(&board, Stone::Opponent);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_diag_up_left_5_in_a_row_5_opp() {
    let result;
    let target_result = -FIVE_IN_A_ROW;
    let mut board: Board = Board::new();

    board.board[15][10] = CellContent::Opponent;
    board.board[16][11] = CellContent::Opponent;
    board.board[17][12] = CellContent::Opponent;
    board.board[18][13] = CellContent::Opponent;
    board.board[19][14] = CellContent::Opponent;

    result = evaluate(&board, Stone::Opponent);

    assert_eq!(result, target_result);
}

// 4 in-a-row
#[test]
fn evaluate_horizontal_4_in_a_row_1_ally() {
    let result;
    let target_result = FOUR_IN_A_ROW_OPEN;
    let mut board: Board = Board::new();

    board.board[0][1] = CellContent::Ally;
    board.board[0][2] = CellContent::Ally;
    board.board[0][3] = CellContent::Ally;
    board.board[0][4] = CellContent::Ally;

    result = evaluate(&board, Stone::Ally);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_horizontal_4_in_a_row_2_ally() {
    let result;
    let target_result = FOUR_IN_A_ROW_OPEN;
    let mut board: Board = Board::new();

    board.board[10][1] = CellContent::Ally;
    board.board[10][2] = CellContent::Ally;
    board.board[10][3] = CellContent::Ally;
    board.board[10][4] = CellContent::Ally;

    result = evaluate(&board, Stone::Ally);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_horizontal_4_in_a_row_3_ally() {
    let result;
    let target_result = FOUR_IN_A_ROW_OPEN;
    let mut board: Board = Board::new();

    board.board[19][1] = CellContent::Ally;
    board.board[19][2] = CellContent::Ally;
    board.board[19][3] = CellContent::Ally;
    board.board[19][4] = CellContent::Ally;

    result = evaluate(&board, Stone::Ally);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_horizontal_4_in_a_row_4_ally() {
    let result;
    let target_result = FOUR_IN_A_ROW_OPEN;
    let mut board: Board = Board::new();

    board.board[0][11] = CellContent::Ally;
    board.board[0][12] = CellContent::Ally;
    board.board[0][13] = CellContent::Ally;
    board.board[0][14] = CellContent::Ally;

    result = evaluate(&board, Stone::Ally);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_horizontal_4_in_a_row_5_ally() {
    let result;
    let target_result = FOUR_IN_A_ROW_OPEN;
    let mut board: Board = Board::new();

    board.board[10][11] = CellContent::Ally;
    board.board[10][12] = CellContent::Ally;
    board.board[10][13] = CellContent::Ally;
    board.board[10][14] = CellContent::Ally;

    result = evaluate(&board, Stone::Ally);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_vertical_4_in_a_row_1_ally() {
    let result;
    let target_result = FOUR_IN_A_ROW_OPEN;
    let mut board: Board = Board::new();

    board.board[1][0] = CellContent::Ally;
    board.board[2][0] = CellContent::Ally;
    board.board[3][0] = CellContent::Ally;
    board.board[4][0] = CellContent::Ally;

    result = evaluate(&board, Stone::Ally);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_vertical_4_in_a_row_2_ally() {
    let result;
    let target_result = FOUR_IN_A_ROW_OPEN;
    let mut board: Board = Board::new();

    board.board[1][10] = CellContent::Ally;
    board.board[2][10] = CellContent::Ally;
    board.board[3][10] = CellContent::Ally;
    board.board[4][10] = CellContent::Ally;

    result = evaluate(&board, Stone::Ally);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_vertical_4_in_a_row_3_ally() {
    let result;
    let target_result = FOUR_IN_A_ROW_OPEN;
    let mut board: Board = Board::new();

    board.board[11][0] = CellContent::Ally;
    board.board[12][0] = CellContent::Ally;
    board.board[13][0] = CellContent::Ally;
    board.board[14][0] = CellContent::Ally;

    result = evaluate(&board, Stone::Ally);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_vertical_4_in_a_row_4_ally() {
    let result;
    let target_result = FOUR_IN_A_ROW_OPEN;
    let mut board: Board = Board::new();

    board.board[1][0] = CellContent::Ally;
    board.board[2][0] = CellContent::Ally;
    board.board[3][0] = CellContent::Ally;
    board.board[4][0] = CellContent::Ally;

    result = evaluate(&board, Stone::Ally);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_vertical_4_in_a_row_5_ally() {
    let result;
    let target_result = FOUR_IN_A_ROW_OPEN;
    let mut board: Board = Board::new();

    board.board[11][10] = CellContent::Ally;
    board.board[12][10] = CellContent::Ally;
    board.board[13][10] = CellContent::Ally;
    board.board[14][10] = CellContent::Ally;

    result = evaluate(&board, Stone::Ally);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_diag_up_right_4_in_a_row_1_ally() {
    let result;
    let target_result = FOUR_IN_A_ROW_OPEN;
    let mut board: Board = Board::new();

    board.board[4][1] = CellContent::Ally;
    board.board[3][2] = CellContent::Ally;
    board.board[2][3] = CellContent::Ally;
    board.board[1][4] = CellContent::Ally;

    result = evaluate(&board, Stone::Ally);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_diag_up_right_4_in_a_row_2_ally() {
    let result;
    let target_result = FOUR_IN_A_ROW_OPEN;
    let mut board: Board = Board::new();

    board.board[13][1] = CellContent::Ally;
    board.board[12][2] = CellContent::Ally;
    board.board[11][3] = CellContent::Ally;
    board.board[10][4] = CellContent::Ally;

    result = evaluate(&board, Stone::Ally);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_diag_up_right_4_in_a_row_3_ally() {
    let result;
    let target_result = FOUR_IN_A_ROW_OPEN;
    let mut board: Board = Board::new();

    board.board[4][10] = CellContent::Ally;
    board.board[3][11] = CellContent::Ally;
    board.board[2][12] = CellContent::Ally;
    board.board[1][13] = CellContent::Ally;

    result = evaluate(&board, Stone::Ally);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_diag_up_right_4_in_a_row_4_ally() {
    let result;
    let target_result = FOUR_IN_A_ROW_OPEN;
    let mut board: Board = Board::new();

    board.board[13][11] = CellContent::Ally;
    board.board[12][12] = CellContent::Ally;
    board.board[11][13] = CellContent::Ally;
    board.board[10][14] = CellContent::Ally;

    result = evaluate(&board, Stone::Ally);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_diag_up_right_4_in_a_row_5_ally() {
    let result;
    let target_result = FOUR_IN_A_ROW_OPEN;
    let mut board: Board = Board::new();

    board.board[18][1] = CellContent::Ally;
    board.board[17][2] = CellContent::Ally;
    board.board[16][3] = CellContent::Ally;
    board.board[15][4] = CellContent::Ally;

    result = evaluate(&board, Stone::Ally);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_diag_up_left_4_in_a_row_1_ally() {
    let result;
    let target_result = FOUR_IN_A_ROW_OPEN;
    let mut board: Board = Board::new();

    board.board[1][1] = CellContent::Ally;
    board.board[2][2] = CellContent::Ally;
    board.board[3][3] = CellContent::Ally;
    board.board[4][4] = CellContent::Ally;

    result = evaluate(&board, Stone::Ally);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_diag_up_left_4_in_a_row_2_ally() {
    let result;
    let target_result = FOUR_IN_A_ROW_OPEN;
    let mut board: Board = Board::new();

    board.board[11][1] = CellContent::Ally;
    board.board[12][2] = CellContent::Ally;
    board.board[13][3] = CellContent::Ally;
    board.board[14][4] = CellContent::Ally;

    result = evaluate(&board, Stone::Ally);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_diag_up_left_4_in_a_row_3_ally() {
    let result;
    let target_result = FOUR_IN_A_ROW_OPEN;
    let mut board: Board = Board::new();

    board.board[1][11] = CellContent::Ally;
    board.board[2][12] = CellContent::Ally;
    board.board[3][13] = CellContent::Ally;
    board.board[4][14] = CellContent::Ally;

    result = evaluate(&board, Stone::Ally);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_diag_up_left_4_in_a_row_4_ally() {
    let result;
    let target_result = FOUR_IN_A_ROW_OPEN;
    let mut board: Board = Board::new();

    board.board[11][11] = CellContent::Ally;
    board.board[12][12] = CellContent::Ally;
    board.board[13][13] = CellContent::Ally;
    board.board[14][14] = CellContent::Ally;

    result = evaluate(&board, Stone::Ally);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_diag_up_left_4_in_a_row_5_ally() {
    let result;
    let target_result = FOUR_IN_A_ROW_OPEN;
    let mut board: Board = Board::new();

    board.board[15][10] = CellContent::Ally;
    board.board[16][11] = CellContent::Ally;
    board.board[17][12] = CellContent::Ally;
    board.board[18][13] = CellContent::Ally;

    result = evaluate(&board, Stone::Ally);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_horizontal_4_in_a_row_1_opp() {
    let result;
    let target_result = -FOUR_IN_A_ROW_OPEN;
    let mut board: Board = Board::new();

    board.board[0][1] = CellContent::Opponent;
    board.board[0][2] = CellContent::Opponent;
    board.board[0][3] = CellContent::Opponent;
    board.board[0][4] = CellContent::Opponent;

    result = evaluate(&board, Stone::Opponent);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_horizontal_4_in_a_row_2_opp() {
    let result;
    let target_result = -FOUR_IN_A_ROW_OPEN;
    let mut board: Board = Board::new();

    board.board[10][1] = CellContent::Opponent;
    board.board[10][2] = CellContent::Opponent;
    board.board[10][3] = CellContent::Opponent;
    board.board[10][4] = CellContent::Opponent;

    result = evaluate(&board, Stone::Opponent);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_horizontal_4_in_a_row_3_opp() {
    let result;
    let target_result = -FOUR_IN_A_ROW_OPEN;
    let mut board: Board = Board::new();

    board.board[19][1] = CellContent::Opponent;
    board.board[19][2] = CellContent::Opponent;
    board.board[19][3] = CellContent::Opponent;
    board.board[19][4] = CellContent::Opponent;

    result = evaluate(&board, Stone::Opponent);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_horizontal_4_in_a_row_4_opp() {
    let result;
    let target_result = -FOUR_IN_A_ROW_OPEN;
    let mut board: Board = Board::new();

    board.board[0][11] = CellContent::Opponent;
    board.board[0][12] = CellContent::Opponent;
    board.board[0][13] = CellContent::Opponent;
    board.board[0][14] = CellContent::Opponent;

    result = evaluate(&board, Stone::Opponent);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_horizontal_4_in_a_row_5_opp() {
    let result;
    let target_result = -FOUR_IN_A_ROW_OPEN;
    let mut board: Board = Board::new();

    board.board[10][11] = CellContent::Opponent;
    board.board[10][12] = CellContent::Opponent;
    board.board[10][13] = CellContent::Opponent;
    board.board[10][14] = CellContent::Opponent;

    result = evaluate(&board, Stone::Opponent);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_vertical_4_in_a_row_1_opp() {
    let result;
    let target_result = -FOUR_IN_A_ROW_OPEN;
    let mut board: Board = Board::new();

    board.board[1][0] = CellContent::Opponent;
    board.board[2][0] = CellContent::Opponent;
    board.board[3][0] = CellContent::Opponent;
    board.board[4][0] = CellContent::Opponent;

    result = evaluate(&board, Stone::Opponent);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_vertical_4_in_a_row_2_opp() {
    let result;
    let target_result = -FOUR_IN_A_ROW_OPEN;
    let mut board: Board = Board::new();

    board.board[1][10] = CellContent::Opponent;
    board.board[2][10] = CellContent::Opponent;
    board.board[3][10] = CellContent::Opponent;
    board.board[4][10] = CellContent::Opponent;

    result = evaluate(&board, Stone::Opponent);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_vertical_4_in_a_row_3_opp() {
    let result;
    let target_result = -FOUR_IN_A_ROW_OPEN;
    let mut board: Board = Board::new();

    board.board[11][0] = CellContent::Opponent;
    board.board[12][0] = CellContent::Opponent;
    board.board[13][0] = CellContent::Opponent;
    board.board[14][0] = CellContent::Opponent;

    result = evaluate(&board, Stone::Opponent);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_vertical_4_in_a_row_4_opp() {
    let result;
    let target_result = -FOUR_IN_A_ROW_OPEN;
    let mut board: Board = Board::new();

    board.board[1][0] = CellContent::Opponent;
    board.board[2][0] = CellContent::Opponent;
    board.board[3][0] = CellContent::Opponent;
    board.board[4][0] = CellContent::Opponent;

    result = evaluate(&board, Stone::Opponent);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_vertical_4_in_a_row_5_opp() {
    let result;
    let target_result = -FOUR_IN_A_ROW_OPEN;
    let mut board: Board = Board::new();

    board.board[11][10] = CellContent::Opponent;
    board.board[12][10] = CellContent::Opponent;
    board.board[13][10] = CellContent::Opponent;
    board.board[14][10] = CellContent::Opponent;

    result = evaluate(&board, Stone::Opponent);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_diag_up_right_4_in_a_row_1_opp() {
    let result;
    let target_result = -FOUR_IN_A_ROW_OPEN;
    let mut board: Board = Board::new();

    board.board[4][1] = CellContent::Opponent;
    board.board[3][2] = CellContent::Opponent;
    board.board[2][3] = CellContent::Opponent;
    board.board[1][4] = CellContent::Opponent;

    result = evaluate(&board, Stone::Opponent);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_diag_up_right_4_in_a_row_2_opp() {
    let result;
    let target_result = -FOUR_IN_A_ROW_OPEN;
    let mut board: Board = Board::new();

    board.board[13][1] = CellContent::Opponent;
    board.board[12][2] = CellContent::Opponent;
    board.board[11][3] = CellContent::Opponent;
    board.board[10][4] = CellContent::Opponent;

    result = evaluate(&board, Stone::Opponent);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_diag_up_right_4_in_a_row_3_opp() {
    let result;
    let target_result = -FOUR_IN_A_ROW_OPEN;
    let mut board: Board = Board::new();

    board.board[4][10] = CellContent::Opponent;
    board.board[3][11] = CellContent::Opponent;
    board.board[2][12] = CellContent::Opponent;
    board.board[1][13] = CellContent::Opponent;

    result = evaluate(&board, Stone::Opponent);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_diag_up_right_4_in_a_row_4_opp() {
    let result;
    let target_result = -FOUR_IN_A_ROW_OPEN;
    let mut board: Board = Board::new();

    board.board[13][11] = CellContent::Opponent;
    board.board[12][12] = CellContent::Opponent;
    board.board[11][13] = CellContent::Opponent;
    board.board[10][14] = CellContent::Opponent;

    result = evaluate(&board, Stone::Opponent);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_diag_up_right_4_in_a_row_5_opp() {
    let result;
    let target_result = -FOUR_IN_A_ROW_OPEN;
    let mut board: Board = Board::new();

    board.board[18][1] = CellContent::Opponent;
    board.board[17][2] = CellContent::Opponent;
    board.board[16][3] = CellContent::Opponent;
    board.board[15][4] = CellContent::Opponent;

    result = evaluate(&board, Stone::Opponent);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_diag_up_left_4_in_a_row_1_opp() {
    let result;
    let target_result = -FOUR_IN_A_ROW_OPEN;
    let mut board: Board = Board::new();

    board.board[1][1] = CellContent::Opponent;
    board.board[2][2] = CellContent::Opponent;
    board.board[3][3] = CellContent::Opponent;
    board.board[4][4] = CellContent::Opponent;

    result = evaluate(&board, Stone::Opponent);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_diag_up_left_4_in_a_row_2_opp() {
    let result;
    let target_result = -FOUR_IN_A_ROW_OPEN;
    let mut board: Board = Board::new();

    board.board[11][1] = CellContent::Opponent;
    board.board[12][2] = CellContent::Opponent;
    board.board[13][3] = CellContent::Opponent;
    board.board[14][4] = CellContent::Opponent;

    result = evaluate(&board, Stone::Opponent);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_diag_up_left_4_in_a_row_3_opp() {
    let result;
    let target_result = -FOUR_IN_A_ROW_OPEN;
    let mut board: Board = Board::new();

    board.board[1][11] = CellContent::Opponent;
    board.board[2][12] = CellContent::Opponent;
    board.board[3][13] = CellContent::Opponent;
    board.board[4][14] = CellContent::Opponent;

    result = evaluate(&board, Stone::Opponent);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_diag_up_left_4_in_a_row_4_opp() {
    let result;
    let target_result = -FOUR_IN_A_ROW_OPEN;
    let mut board: Board = Board::new();

    board.board[11][11] = CellContent::Opponent;
    board.board[12][12] = CellContent::Opponent;
    board.board[13][13] = CellContent::Opponent;
    board.board[14][14] = CellContent::Opponent;

    result = evaluate(&board, Stone::Opponent);

    assert_eq!(result, target_result);
}

#[test]
fn evaluate_diag_up_left_4_in_a_row_5_opp() {
    let result;
    let target_result = -FOUR_IN_A_ROW_OPEN;
    let mut board: Board = Board::new();

    board.board[15][10] = CellContent::Opponent;
    board.board[16][11] = CellContent::Opponent;
    board.board[17][12] = CellContent::Opponent;
    board.board[18][13] = CellContent::Opponent;

    result = evaluate(&board, Stone::Opponent);

    assert_eq!(result, target_result);
}
