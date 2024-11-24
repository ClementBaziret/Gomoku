use crate::model::{CellContent, Stone};
use crate::{
    board::Board,
    grid_iterators::{
        GridColumns, GridDownRightDiagonals, GridUpRightDiagonals,
    },
};

pub const FIVE_IN_A_ROW: i32 = 1000000;
pub const FOUR_IN_A_ROW_OPEN: i32 = 5000;
pub const FOUR_IN_A_ROW_CLOSED: i32 = 2000;
pub const THREE_IN_A_ROW_OPEN: i32 = 1000;
pub const THREE_IN_A_ROW_CLOSED: i32 = 800;
pub const TWO_IN_A_ROW: i32 = 500;
pub const MISC: i32 = 200;

fn check_five_aligned(
    line: &Vec<&CellContent>,
    stone: Stone,
) -> bool {
    line.windows(5)
        .any(|w| w == &[&CellContent::from(stone); 5])
}

fn check_for_5(temp: &Vec<&CellContent>) -> i32 {
    if check_five_aligned(temp, Stone::Ally) {
        return FIVE_IN_A_ROW;
    }
    if check_five_aligned(temp, Stone::Opponent) {
        return -FIVE_IN_A_ROW;
    }
    0
}

fn check_for_4_open(temp: &Vec<&CellContent>) -> i32 {
    for window in temp.windows(6) {
        if window
            == &[
                &CellContent::Empty,
                &CellContent::Ally,
                &CellContent::Ally,
                &CellContent::Ally,
                &CellContent::Ally,
                &CellContent::Empty,
            ]
        {
            return FOUR_IN_A_ROW_OPEN;
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
            return -FOUR_IN_A_ROW_OPEN;
        }
    }
    for window in temp.windows(8) {
        if window
            == &[
                &CellContent::Ally,
                &CellContent::Ally,
                &CellContent::Empty,
                &CellContent::Ally,
                &CellContent::Ally,
                &CellContent::Empty,
                &CellContent::Ally,
                &CellContent::Ally,
            ]
        {
            return FOUR_IN_A_ROW_OPEN;
        }
        if window
            == &[
                &CellContent::Opponent,
                &CellContent::Opponent,
                &CellContent::Empty,
                &CellContent::Opponent,
                &CellContent::Opponent,
                &CellContent::Empty,
                &CellContent::Opponent,
                &CellContent::Opponent,
            ]
        {
            return -FOUR_IN_A_ROW_OPEN;
        }
    }
    for window in temp.windows(9) {
        if window
            == &[
                &CellContent::Ally,
                &CellContent::Ally,
                &CellContent::Ally,
                &CellContent::Empty,
                &CellContent::Ally,
                &CellContent::Empty,
                &CellContent::Ally,
                &CellContent::Ally,
                &CellContent::Ally,
            ]
        {
            return FOUR_IN_A_ROW_OPEN;
        }
        if window
            == &[
                &CellContent::Opponent,
                &CellContent::Opponent,
                &CellContent::Opponent,
                &CellContent::Empty,
                &CellContent::Opponent,
                &CellContent::Empty,
                &CellContent::Opponent,
                &CellContent::Opponent,
                &CellContent::Opponent,
            ]
        {
            return -FOUR_IN_A_ROW_OPEN;
        }
    }
    0
}

fn check_for_4_closed(temp: &Vec<&CellContent>) -> i32 {
    for window in temp.windows(5) {
        if window
            == &[
                &CellContent::Ally,
                &CellContent::Ally,
                &CellContent::Empty,
                &CellContent::Ally,
                &CellContent::Ally,
            ]
        {
            return FOUR_IN_A_ROW_CLOSED;
        }
        if window
            == &[
                &CellContent::Opponent,
                &CellContent::Opponent,
                &CellContent::Empty,
                &CellContent::Opponent,
                &CellContent::Opponent,
            ]
        {
            return -FOUR_IN_A_ROW_CLOSED;
        }
    }
    for window in temp.windows(6) {
        if window
            == &[
                &CellContent::Opponent,
                &CellContent::Ally,
                &CellContent::Ally,
                &CellContent::Ally,
                &CellContent::Ally,
                &CellContent::Empty,
            ]
            || window
                == &[
                    &CellContent::Empty,
                    &CellContent::Ally,
                    &CellContent::Ally,
                    &CellContent::Ally,
                    &CellContent::Ally,
                    &CellContent::Opponent,
                ]
        {
            return FOUR_IN_A_ROW_CLOSED;
        }
        if window
            == &[
                &CellContent::Ally,
                &CellContent::Opponent,
                &CellContent::Opponent,
                &CellContent::Opponent,
                &CellContent::Opponent,
                &CellContent::Empty,
            ]
            || window
                == &[
                    &CellContent::Empty,
                    &CellContent::Opponent,
                    &CellContent::Opponent,
                    &CellContent::Opponent,
                    &CellContent::Opponent,
                    &CellContent::Ally,
                ]
        {
            return -FOUR_IN_A_ROW_CLOSED;
        }
    }
    0
}

fn check_for_3_open(temp: &Vec<&CellContent>) -> i32 {
    for window in temp.windows(7) {
        if window
            == &[
                &CellContent::Empty,
                &CellContent::Empty,
                &CellContent::Ally,
                &CellContent::Ally,
                &CellContent::Ally,
                &CellContent::Empty,
                &CellContent::Empty,
            ]
        {
            return THREE_IN_A_ROW_OPEN;
        }
        if window
            == &[
                &CellContent::Empty,
                &CellContent::Empty,
                &CellContent::Opponent,
                &CellContent::Opponent,
                &CellContent::Opponent,
                &CellContent::Empty,
                &CellContent::Empty,
            ]
        {
            return -THREE_IN_A_ROW_OPEN;
        }
    }
    for window in temp.windows(8) {
        if window
            == &[
                &CellContent::Empty,
                &CellContent::Ally,
                &CellContent::Empty,
                &CellContent::Ally,
                &CellContent::Ally,
                &CellContent::Empty,
                &CellContent::Ally,
                &CellContent::Empty,
            ]
        {
            return THREE_IN_A_ROW_OPEN;
        }
        if window
            == &[
                &CellContent::Empty,
                &CellContent::Opponent,
                &CellContent::Empty,
                &CellContent::Opponent,
                &CellContent::Opponent,
                &CellContent::Empty,
                &CellContent::Opponent,
                &CellContent::Empty,
            ]
        {
            return -THREE_IN_A_ROW_OPEN;
        }
    }
    for window in temp.windows(9) {
        if window
            == &[
                &CellContent::Empty,
                &CellContent::Ally,
                &CellContent::Empty,
                &CellContent::Ally,
                &CellContent::Empty,
                &CellContent::Ally,
                &CellContent::Empty,
                &CellContent::Ally,
                &CellContent::Empty,
            ]
        {
            return THREE_IN_A_ROW_OPEN;
        }
        if window
            == &[
                &CellContent::Empty,
                &CellContent::Opponent,
                &CellContent::Empty,
                &CellContent::Opponent,
                &CellContent::Empty,
                &CellContent::Opponent,
                &CellContent::Empty,
                &CellContent::Opponent,
                &CellContent::Empty,
            ]
        {
            return -THREE_IN_A_ROW_OPEN;
        }
    }
    0
}

fn check_for_3_closed(temp: &Vec<&CellContent>) -> i32 {
    for window in temp.windows(5) {
        if window
            == &[
                &CellContent::Empty,
                &CellContent::Ally,
                &CellContent::Ally,
                &CellContent::Ally,
                &CellContent::Empty,
            ]
        {
            return THREE_IN_A_ROW_CLOSED;
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
            return -THREE_IN_A_ROW_CLOSED;
        }
    }
    for window in temp.windows(6) {
        if window
            == &[
                &CellContent::Empty,
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
                    &CellContent::Empty,
                ]
        {
            return THREE_IN_A_ROW_CLOSED;
        }
        if window
            == &[
                &CellContent::Empty,
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
                    &CellContent::Empty,
                ]
        {
            return -THREE_IN_A_ROW_CLOSED;
        }
    }
    0
}

fn check_for_other(temp: &Vec<&CellContent>) -> i32 {
    for window in temp.windows(4) {
        if window
            == &[
                &CellContent::Opponent,
                &CellContent::Ally,
                &CellContent::Ally,
                &CellContent::Empty,
            ]
            || window
                == &[
                    &CellContent::Empty,
                    &CellContent::Ally,
                    &CellContent::Ally,
                    &CellContent::Opponent,
                ]
        {
            return MISC;
        }
        if window
            == &[
                &CellContent::Ally,
                &CellContent::Opponent,
                &CellContent::Opponent,
                &CellContent::Empty,
            ]
            || window
                == &[
                    &CellContent::Empty,
                    &CellContent::Opponent,
                    &CellContent::Opponent,
                    &CellContent::Ally,
                ]
        {
            return -MISC;
        }
    }
    for window in temp.windows(6) {
        if window
            == &[
                &CellContent::Opponent,
                &CellContent::Empty,
                &CellContent::Ally,
                &CellContent::Empty,
                &CellContent::Empty,
                &CellContent::Opponent,
            ]
            || window
                == &[
                    &CellContent::Opponent,
                    &CellContent::Empty,
                    &CellContent::Empty,
                    &CellContent::Ally,
                    &CellContent::Empty,
                    &CellContent::Opponent,
                ]
        {
            return MISC;
        }
        if window
            == &[
                &CellContent::Ally,
                &CellContent::Empty,
                &CellContent::Opponent,
                &CellContent::Empty,
                &CellContent::Empty,
                &CellContent::Ally,
            ]
            || window
                == &[
                    &CellContent::Ally,
                    &CellContent::Empty,
                    &CellContent::Empty,
                    &CellContent::Opponent,
                    &CellContent::Empty,
                    &CellContent::Ally,
                ]
        {
            return -MISC;
        }
    }
    for window in temp.windows(9) {
        if window
            == &[
                &CellContent::Opponent,
                &CellContent::Empty,
                &CellContent::Empty,
                &CellContent::Ally,
                &CellContent::Empty,
                &CellContent::Ally,
                &CellContent::Empty,
                &CellContent::Empty,
                &CellContent::Opponent,
            ]
        {
            return MISC;
        }
        if window
            == &[
                &CellContent::Ally,
                &CellContent::Empty,
                &CellContent::Empty,
                &CellContent::Opponent,
                &CellContent::Empty,
                &CellContent::Opponent,
                &CellContent::Empty,
                &CellContent::Empty,
                &CellContent::Ally,
            ]
        {
            return -MISC;
        }
    }
    0
}

fn check_for_2(temp: &Vec<&CellContent>) -> i32 {
    for window in temp.windows(4) {
        if window
            == &[
                &CellContent::Empty,
                &CellContent::Ally,
                &CellContent::Ally,
                &CellContent::Empty,
            ]
        {
            return TWO_IN_A_ROW;
        }
        if window
            == &[
                &CellContent::Empty,
                &CellContent::Opponent,
                &CellContent::Opponent,
                &CellContent::Empty,
            ]
        {
            return -TWO_IN_A_ROW;
        }
    }
    0
}

fn iterate_row(temp: Vec<&CellContent>) -> i32 {
    let mut result;

    result = check_for_5(&temp);
    if result != 0 {
        return result;
    }
    result = check_for_4_open(&temp);
    if result != 0 {
        return result;
    }
    result = check_for_4_closed(&temp);
    if result != 0 {
        return result;
    }
    result = check_for_3_open(&temp);
    if result != 0 {
        return result;
    }
    result = check_for_3_closed(&temp);
    if result != 0 {
        return result;
    }
    result = check_for_other(&temp);
    if result != 0 {
        return result;
    }
    result = check_for_2(&temp);
    if result != 0 {
        return result;
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

pub fn evaluate(board: &Board, just_played: Stone) -> i32 {
    let i = check_rows_in_board(board);
    i
}
