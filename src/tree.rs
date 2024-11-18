use std::cmp::{max, min};

use crate::{board::Board, evaluation::evaluate, model::CellContent};

#[derive(Debug)]
pub struct AllyMove {
    pub x: u8,
    pub y: u8,
    pub value: Option<i32>,
    pub opp_moves: Vec<OpponentMove>,
}

#[derive(Debug)]
pub struct OpponentMove {
    pub x: u8,
    pub y: u8,
    pub value: Option<i32>,
    pub ally_moves: Tree,
}

#[derive(Debug, Default)]
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
        max(num1, num2)
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
        min(num1, num2)
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

    let ret = {
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
