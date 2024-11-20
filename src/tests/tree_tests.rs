use crate::tree::{
    evaluate_move_recursive, AllyMove, Move, OpponentMove, Tree,
    TreeRoot,
};
use crate::{board::Board, model::CellContent};
#[cfg(test)]
impl TreeRoot {
    pub fn new(board: Board) -> Self {
        Self {
            board,
            tree: Tree::default(),
        }
    }
}

// Units tests

#[test]
fn check_tree_generation() {
    let mut board = Board::new();

    board.board[1][1] = CellContent::Ally;

    let mut root: TreeRoot = TreeRoot {
        board,
        tree: Tree::gen_tree(&mut board, 1),
    };
    let expected: Vec<AllyMove> = vec![
        AllyMove::new(0, 0),
        AllyMove::new(1, 0),
        AllyMove::new(2, 0),
        AllyMove::new(0, 1),
        AllyMove::new(2, 1),
        AllyMove::new(0, 2),
        AllyMove::new(1, 2),
        AllyMove::new(2, 2),
    ];
    for (i, m) in root.tree.moves.iter().enumerate() {
        assert!(m.x == expected[i].x && m.y == expected[i].y);
    }
}

#[test]
fn check_win_move_evaluation() {
    let mut board = Board::new();

    board.board[1][1] = CellContent::Ally;
    board.board[1][2] = CellContent::Ally;
    board.board[1][3] = CellContent::Ally;
    board.board[1][4] = CellContent::Ally;

    let mut ally_play = AllyMove::new(5, 1);
    let move_value =
        evaluate_move_recursive(&mut ally_play, &mut board);

    assert_eq!(move_value, 1000000);
}

#[test]
fn check_lose_move_evaluation() {
    let mut board = Board::new();

    board.board[1][1] = CellContent::Opponent;
    board.board[1][2] = CellContent::Opponent;
    board.board[1][3] = CellContent::Opponent;
    board.board[1][4] = CellContent::Opponent;

    let mut opponent_play = OpponentMove::new(5, 1);
    let move_value =
        evaluate_move_recursive(&mut opponent_play, &mut board);

    assert_eq!(move_value, -1000000);
}

// ----------- Units tests -----------

#[cfg(test)]
fn assert_expected_move(root: &mut TreeRoot, x: u8, y: u8) {
    root.tree = Tree::gen_tree(&mut root.board, 1);

    let mut chosen_move: Option<&AllyMove> = None;
    let mut best_value = i32::MIN;

    for tree_move in &mut root.tree.moves {
        let val = evaluate_move_recursive(tree_move, &mut root.board);
        if val > best_value {
            best_value = val;
            chosen_move = Some(tree_move);
        }
    }

    assert!(chosen_move.is_some());
    let chosen_move = chosen_move.unwrap();

    assert_eq!((chosen_move.x, chosen_move.y), (x, y));
}

#[test]
fn test_detect_horizontal_up_immediate_win() {
    let mut root = TreeRoot::new(Board::new());

    assert_eq!(root.board.board, [[CellContent::Empty; 20]; 20]);

    root.board.board[10][10] = CellContent::Ally;
    root.board.board[10][11] = CellContent::Ally;
    root.board.board[10][12] = CellContent::Ally;
    root.board.board[10][13] = CellContent::Ally;
    root.board.board[10][14] = CellContent::Opponent;

    assert_expected_move(&mut root, 9, 10);
}

#[test]
fn test_detect_horizontal_down_immediate_win() {
    let mut root = TreeRoot::new(Board::new());

    assert_eq!(root.board.board, [[CellContent::Empty; 20]; 20]);

    root.board.board[10][9] = CellContent::Opponent;
    root.board.board[10][10] = CellContent::Ally;
    root.board.board[10][11] = CellContent::Ally;
    root.board.board[10][12] = CellContent::Ally;
    root.board.board[10][13] = CellContent::Ally;

    assert_expected_move(&mut root, 14, 10);
}

#[test]
fn test_detect_vertical_up_immediate_win() {
    let mut root = TreeRoot::new(Board::new());

    assert_eq!(root.board.board, [[CellContent::Empty; 20]; 20]);

    root.board.board[10][10] = CellContent::Ally;
    root.board.board[11][10] = CellContent::Ally;
    root.board.board[12][10] = CellContent::Ally;
    root.board.board[13][10] = CellContent::Ally;
    root.board.board[14][10] = CellContent::Opponent;

    assert_expected_move(&mut root, 10, 9);
}

#[test]
fn test_detect_vertical_down_immediate_win() {
    let mut root = TreeRoot::new(Board::new());

    assert_eq!(root.board.board, [[CellContent::Empty; 20]; 20]);

    root.board.board[9][10] = CellContent::Opponent;
    root.board.board[10][10] = CellContent::Ally;
    root.board.board[11][10] = CellContent::Ally;
    root.board.board[12][10] = CellContent::Ally;
    root.board.board[13][10] = CellContent::Ally;

    assert_expected_move(&mut root, 10, 14);
}

#[test]
fn test_detect_up_right_diagonal_up_immediate_win() {
    let mut root = TreeRoot::new(Board::new());

    assert_eq!(root.board.board, [[CellContent::Empty; 20]; 20]);

    root.board.board[8][1] = CellContent::Ally;
    root.board.board[7][2] = CellContent::Ally;
    root.board.board[6][3] = CellContent::Ally;
    root.board.board[5][4] = CellContent::Ally;
    root.board.board[4][5] = CellContent::Opponent;

    assert_expected_move(&mut root, 0, 9);
}

#[test]
fn test_detect_up_right_diagonal_down_immediate_win() {
    let mut root = TreeRoot::new(Board::new());

    assert_eq!(root.board.board, [[CellContent::Empty; 20]; 20]);

    root.board.board[9][0] = CellContent::Opponent;
    root.board.board[8][1] = CellContent::Ally;
    root.board.board[7][2] = CellContent::Ally;
    root.board.board[6][3] = CellContent::Ally;
    root.board.board[5][4] = CellContent::Ally;

    assert_expected_move(&mut root, 5, 4);
}

#[test]
fn test_detect_down_right_diagonal_up_immediate_win() {
    let mut root = TreeRoot::new(Board::new());

    assert_eq!(root.board.board, [[CellContent::Empty; 20]; 20]);

    root.board.board[1][1] = CellContent::Ally;
    root.board.board[2][2] = CellContent::Ally;
    root.board.board[3][3] = CellContent::Ally;
    root.board.board[4][4] = CellContent::Ally;
    root.board.board[5][5] = CellContent::Opponent;

    assert_expected_move(&mut root, 0, 0);
}

#[test]
fn test_detect_down_right_diagonal_down_immediate_win() {
    let mut root = TreeRoot::new(Board::new());

    assert_eq!(root.board.board, [[CellContent::Empty; 20]; 20]);

    root.board.board[0][0] = CellContent::Opponent;
    root.board.board[1][1] = CellContent::Ally;
    root.board.board[2][2] = CellContent::Ally;
    root.board.board[3][3] = CellContent::Ally;
    root.board.board[4][4] = CellContent::Ally;

    assert_expected_move(&mut root, 5, 5);
}

#[test]
fn test_detect_horizontal_up_immediate_lose() {
    let mut root = TreeRoot::new(Board::new());

    assert_eq!(root.board.board, [[CellContent::Empty; 20]; 20]);

    root.board.board[10][10] = CellContent::Opponent;
    root.board.board[10][11] = CellContent::Opponent;
    root.board.board[10][12] = CellContent::Opponent;
    root.board.board[10][13] = CellContent::Opponent;
    root.board.board[10][14] = CellContent::Ally;

    assert_expected_move(&mut root, 9, 10);
}

#[test]
fn test_detect_horizontal_down_immediate_lose() {
    let mut root = TreeRoot::new(Board::new());

    assert_eq!(root.board.board, [[CellContent::Empty; 20]; 20]);

    root.board.board[10][9] = CellContent::Ally;
    root.board.board[10][10] = CellContent::Opponent;
    root.board.board[10][11] = CellContent::Opponent;
    root.board.board[10][12] = CellContent::Opponent;
    root.board.board[10][13] = CellContent::Opponent;

    assert_expected_move(&mut root, 14, 10);
}

#[test]
fn test_detect_vertical_up_immediate_lose() {
    let mut root = TreeRoot::new(Board::new());

    assert_eq!(root.board.board, [[CellContent::Empty; 20]; 20]);

    root.board.board[10][10] = CellContent::Opponent;
    root.board.board[11][10] = CellContent::Opponent;
    root.board.board[12][10] = CellContent::Opponent;
    root.board.board[13][10] = CellContent::Opponent;
    root.board.board[14][10] = CellContent::Ally;

    assert_expected_move(&mut root, 10, 9);
}

#[test]
fn test_detect_vertical_down_immediate_lose() {
    let mut root = TreeRoot::new(Board::new());

    assert_eq!(root.board.board, [[CellContent::Empty; 20]; 20]);

    root.board.board[9][10] = CellContent::Ally;
    root.board.board[10][10] = CellContent::Opponent;
    root.board.board[11][10] = CellContent::Opponent;
    root.board.board[12][10] = CellContent::Opponent;
    root.board.board[13][10] = CellContent::Opponent;

    assert_expected_move(&mut root, 10, 14);
}

#[test]
fn test_detect_up_right_diagonal_up_immediate_lose() {
    let mut root = TreeRoot::new(Board::new());

    assert_eq!(root.board.board, [[CellContent::Empty; 20]; 20]);

    root.board.board[8][1] = CellContent::Opponent;
    root.board.board[7][2] = CellContent::Opponent;
    root.board.board[6][3] = CellContent::Opponent;
    root.board.board[5][4] = CellContent::Opponent;
    root.board.board[4][5] = CellContent::Ally;

    assert_expected_move(&mut root, 0, 9);
}

#[test]
fn test_detect_up_right_diagonal_down_immediate_lose() {
    let mut root = TreeRoot::new(Board::new());

    assert_eq!(root.board.board, [[CellContent::Empty; 20]; 20]);

    root.board.board[9][0] = CellContent::Ally;
    root.board.board[8][1] = CellContent::Opponent;
    root.board.board[7][2] = CellContent::Opponent;
    root.board.board[6][3] = CellContent::Opponent;
    root.board.board[5][4] = CellContent::Opponent;

    assert_expected_move(&mut root, 5, 4);
}

#[test]
fn test_detect_down_right_diagonal_up_immediate_lose() {
    let mut root = TreeRoot::new(Board::new());

    assert_eq!(root.board.board, [[CellContent::Empty; 20]; 20]);

    root.board.board[1][1] = CellContent::Opponent;
    root.board.board[2][2] = CellContent::Opponent;
    root.board.board[3][3] = CellContent::Opponent;
    root.board.board[4][4] = CellContent::Opponent;
    root.board.board[5][5] = CellContent::Ally;

    assert_expected_move(&mut root, 0, 0);
}

#[test]
fn test_detect_down_right_diagonal_down_immediate_lose() {
    let mut root = TreeRoot::new(Board::new());

    assert_eq!(root.board.board, [[CellContent::Empty; 20]; 20]);

    root.board.board[0][0] = CellContent::Ally;
    root.board.board[1][1] = CellContent::Opponent;
    root.board.board[2][2] = CellContent::Opponent;
    root.board.board[3][3] = CellContent::Opponent;
    root.board.board[4][4] = CellContent::Opponent;

    assert_expected_move(&mut root, 5, 5);
}

#[test]
fn test_immediate_lose_with_potential_victory() {
    let mut root = TreeRoot::new(Board::new());

    assert_eq!(root.board.board, [[CellContent::Empty; 20]; 20]);

    root.board.board[2][4] = CellContent::Ally;
    root.board.board[2][5] = CellContent::Ally;

    root.board.board[3][6] = CellContent::Opponent;
    root.board.board[4][5] = CellContent::Opponent;
    root.board.board[5][4] = CellContent::Opponent;

    let (x, y) = root.board.calculate_next_move();
    assert_eq!((x, y), (7, 2));
}

#[test]
fn test_priority_immediate_win() {
    let mut root = TreeRoot::new(Board::new());

    assert_eq!(root.board.board, [[CellContent::Empty; 20]; 20]);

    root.board.board[0][4] = CellContent::Opponent;
    root.board.board[1][4] = CellContent::Ally;
    root.board.board[2][4] = CellContent::Ally;
    root.board.board[3][4] = CellContent::Ally;
    root.board.board[4][4] = CellContent::Ally;

    root.board.board[0][5] = CellContent::Ally;
    root.board.board[1][5] = CellContent::Opponent;
    root.board.board[2][5] = CellContent::Opponent;
    root.board.board[3][5] = CellContent::Opponent;
    root.board.board[4][5] = CellContent::Opponent;

    let (x, y) = root.board.calculate_next_move();
    assert_eq!((x, y), (4, 5));
}

#[test]
fn test_priority_immediate_win_reversed() {
    let mut root = TreeRoot::new(Board::new());

    assert_eq!(root.board.board, [[CellContent::Empty; 20]; 20]);

    root.board.board[0][6] = CellContent::Opponent;
    root.board.board[1][6] = CellContent::Ally;
    root.board.board[2][6] = CellContent::Ally;
    root.board.board[3][6] = CellContent::Ally;
    root.board.board[4][6] = CellContent::Ally;

    root.board.board[0][5] = CellContent::Ally;
    root.board.board[1][5] = CellContent::Opponent;
    root.board.board[2][5] = CellContent::Opponent;
    root.board.board[3][5] = CellContent::Opponent;
    root.board.board[4][5] = CellContent::Opponent;

    let (x, y) = root.board.calculate_next_move();
    assert_eq!((x, y), (6, 5));
}

#[test]
fn test_priority_immediate_lose() {
    let mut root = TreeRoot::new(Board::new());

    assert_eq!(root.board.board, [[CellContent::Empty; 20]; 20]);

    root.board.board[0][4] = CellContent::Ally;
    root.board.board[1][4] = CellContent::Opponent;
    root.board.board[2][4] = CellContent::Opponent;
    root.board.board[3][4] = CellContent::Opponent;
    root.board.board[4][4] = CellContent::Opponent;

    root.board.board[0][5] = CellContent::Opponent;
    root.board.board[1][5] = CellContent::Ally;
    root.board.board[2][5] = CellContent::Ally;
    root.board.board[3][5] = CellContent::Ally;

    let (x, y) = root.board.calculate_next_move();
    assert_eq!((x, y), (4, 5));
}
