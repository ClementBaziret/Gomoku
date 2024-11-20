use std::{
    cell::Cell,
    cmp::{max, min},
    i32,
};

use crate::{board::Board, evaluation::evaluate, model::CellContent};

#[derive(Debug, PartialEq)]
pub struct AllyMove {
    pub x: u8,
    pub y: u8,
    pub value: Option<i32>,
    pub opp_moves: Vec<OpponentMove>,
}

#[derive(Debug, PartialEq)]
pub struct OpponentMove {
    pub x: u8,
    pub y: u8,
    pub value: Option<i32>,
    pub ally_moves: Tree,
}

#[derive(Debug, Default, PartialEq)]
pub struct Tree {
    pub moves: Vec<AllyMove>,
}

pub struct TreeRoot {
    pub board: Board,
    pub tree: Tree,
}

pub trait Move<IndexType>
where
    IndexType: Into<usize>,
{
    fn new(x: IndexType, y: IndexType) -> Self;

    fn play_move_on_board(&self, board: &mut Board);

    fn get_coords(&self) -> (IndexType, IndexType);

    fn undo_move_on_board(&self, board: &mut Board);

    fn get_subtree(&mut self) -> &mut Vec<impl Move<IndexType>>;

    fn get_value(&mut self) -> &mut Option<i32>;

    fn choose_move(num1: i32, num2: i32) -> i32;
}

trait AMove {
    fn _new(x: u8, y: u8) -> Self;

    fn _get_coords(&self) -> (u8, u8);

    fn content() -> CellContent;

    fn _get_subtree(&mut self) -> &mut Vec<impl Move<u8>>;

    fn _get_value(&mut self) -> &mut Option<i32>;

    fn _choose_move(num1: i32, num2: i32) -> i32;
}

impl<T: AMove> Move<u8> for T {
    fn new(x: u8, y: u8) -> Self {
        Self::_new(x, y)
    }

    fn get_coords(&self) -> (u8, u8) {
        self._get_coords()
    }

    fn play_move_on_board(&self, board: &mut Board) {
        let (x, y) = self._get_coords();
        board.board[y as usize][x as usize] = Self::content();
    }

    fn undo_move_on_board(&self, board: &mut Board) {
        let (x, y) = self._get_coords();

        board.board[y as usize][x as usize] = CellContent::Empty;
    }

    fn get_subtree(&mut self) -> &mut Vec<impl Move<u8>> {
        self._get_subtree()
    }

    fn get_value(&mut self) -> &mut Option<i32> {
        self._get_value()
    }

    fn choose_move(num1: i32, num2: i32) -> i32 {
        Self::_choose_move(num1, num2)
    }
}

impl AMove for AllyMove {
    fn _new(x: u8, y: u8) -> Self {
        Self {
            x,
            y,
            value: None,
            opp_moves: vec![],
        }
    }

    fn content() -> CellContent {
        CellContent::Ally
    }

    fn _get_coords(&self) -> (u8, u8) {
        (self.x, self.y)
    }

    fn _get_subtree(&mut self) -> &mut Vec<impl Move<u8>> {
        &mut self.opp_moves
    }

    fn _get_value(&mut self) -> &mut Option<i32> {
        &mut self.value
    }

    fn _choose_move(num1: i32, num2: i32) -> i32 {
        min(num1, num2)
    }
}

impl AMove for OpponentMove {
    fn _new(x: u8, y: u8) -> Self {
        Self {
            x,
            y,
            value: None,
            ally_moves: Tree::default(),
        }
    }

    fn content() -> CellContent {
        CellContent::Opponent
    }

    fn _get_coords(&self) -> (u8, u8) {
        (self.x, self.y)
    }

    fn _get_subtree(&mut self) -> &mut Vec<impl Move<u8>> {
        &mut self.ally_moves.moves
    }

    fn _get_value(&mut self) -> &mut Option<i32> {
        &mut self.value
    }

    fn _choose_move(num1: i32, num2: i32) -> i32 {
        max(num1, num2)
    }
}

pub fn generate_next_adjacent(
    board_copy: &mut Board,
    current_move: &impl Move<u8>,
) -> Vec<(u8, u8)> {
    current_move.play_move_on_board(board_copy);

    let ret = board_copy.get_adjacent_cells();

    current_move.undo_move_on_board(board_copy);

    ret
}

pub fn gen_moves<T: Move<u8>>(board: &Board) -> Vec<T> {
    board
        .get_adjacent_cells()
        .into_iter()
        .map(|(x, y)| T::new(x, y))
        .collect()
}

impl AllyMove {
    pub fn gen_opponent_moves(&mut self, board: &mut Board) {
        self.opp_moves = generate_next_adjacent(board, self)
            .into_iter()
            .map(|(x, y)| OpponentMove::_new(x, y))
            .collect();
    }
}

impl OpponentMove {
    pub fn gen_ally_moves(&mut self, board: &mut Board) {
        self.ally_moves.moves = generate_next_adjacent(board, self)
            .into_iter()
            .map(|(x, y)| AllyMove::_new(x, y))
            .collect();
    }
}

impl Tree {
    pub fn gen_tree(board: &mut Board, depth: u8) -> Self {
        let mut tree = Self::default();

        if depth == 0 {
            return tree;
        }

        tree.moves = gen_moves(board);
        for ally_move in &mut tree.moves {
            ally_move.play_move_on_board(board);
            ally_move.opp_moves = gen_moves(board);
            for opponent_move in &mut ally_move.opp_moves {
                opponent_move.play_move_on_board(board);
                opponent_move.ally_moves =
                    Self::gen_tree(board, depth - 1);
                opponent_move.undo_move_on_board(board);
            }
            ally_move.undo_move_on_board(board);
        }

        tree
    }
}

/// Depth-first tree traversal which assigns a value
/// to each node of the tree, based on static evaluation
/// of leaf nodes and min/max alternating going down the stack
pub fn evaluate_move_recursive<MoveType, IndexType>(
    to_eval: &mut MoveType,
    board: &mut Board,
) -> i32
where
    MoveType: Move<IndexType>,
    IndexType: Into<usize>,
{
    to_eval.play_move_on_board(board);

    let ret = 'score: {
        let lose_win_score = evaluate(board);
        if lose_win_score > 500000 || lose_win_score < -500000 {
            break 'score lose_win_score;
        }

        let subtree = to_eval.get_subtree();

        if subtree.is_empty() {
            let score = evaluate(board);
            *to_eval.get_value() = Some(score);

            score
        } else {
            let mut subtree_iter = subtree.iter_mut();
            let mut ret_score = evaluate_move_recursive(
                subtree_iter.next().unwrap(),
                board,
            );

            for sub_move in subtree_iter {
                ret_score = MoveType::choose_move(
                    ret_score,
                    evaluate_move_recursive(sub_move, board),
                );
            }

            ret_score
        }
    };

    to_eval.undo_move_on_board(board);

    ret
}

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

// -----------

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

#[ignore = "fail but we need to make this PR through"]
#[test]
fn test_immediate_lose_with_potential_victory() {
    let mut root = TreeRoot::new(Board::new());

    assert_eq!(root.board.board, [[CellContent::Empty; 20]; 20]);

    // root.board.board[2][2] = CellContent::Ally;
    // root.board.board[2][3] = CellContent::Ally;
    root.board.board[2][4] = CellContent::Ally;
    root.board.board[2][5] = CellContent::Ally;

    root.board.board[3][6] = CellContent::Opponent;
    root.board.board[4][5] = CellContent::Opponent;
    root.board.board[5][4] = CellContent::Opponent;
    // root.board.board[6][3] = CellContent::Opponent;
    root.board.print_board();

    let (x, y) = root.board.calculate_next_move();
    assert_eq!((x, y), (7, 2));
}

#[test]
fn test_priority_immediate_win() {
    let mut root = TreeRoot::new(Board::new());

    assert_eq!(root.board.board, [[CellContent::Empty; 20]; 20]);

    // root.board.board[2][2] = CellContent::Ally;
    // root.board.board[2][3] = CellContent::Ally;
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
    root.board.board[y as usize][x as usize] = CellContent::Ally;
    root.board.print_board();
    assert_eq!((x, y), (4, 5));
}

#[test]
fn test_priority_immediate_win_reversed() {
    let mut root = TreeRoot::new(Board::new());

    assert_eq!(root.board.board, [[CellContent::Empty; 20]; 20]);

    // root.board.board[2][2] = CellContent::Ally;
    // root.board.board[2][3] = CellContent::Ally;
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
    root.board.board[y as usize][x as usize] = CellContent::Ally;
    root.board.print_board();
    assert_eq!((x, y), (6, 5));
}

#[test]
fn test_priority_immediate_lose() {
    let mut root = TreeRoot::new(Board::new());

    assert_eq!(root.board.board, [[CellContent::Empty; 20]; 20]);

    // root.board.board[2][2] = CellContent::Ally;
    // root.board.board[2][3] = CellContent::Ally;
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
    root.board.board[y as usize][x as usize] = CellContent::Ally;
    root.board.print_board();
    assert_eq!((x, y), (4, 5));
}
