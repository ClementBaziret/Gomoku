use crate::{board::Board, model::CellContent};

#[derive(Debug)]
pub struct AllyMove {
    pub x: u8,
    pub y: u8,
    pub opp_moves: Vec<OpponentMove>,
}

#[derive(Debug)]
pub struct OpponentMove {
    pub x: u8,
    pub y: u8,
    pub ally_moves: Tree,
}

#[derive(Debug, Default)]
pub struct Tree {
    moves: Vec<AllyMove>,
}

pub struct TreeRoot {
    board: Board,
    tree: Tree,
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
}

trait AMove {
    fn _new(x: u8, y: u8) -> Self;

    fn _get_coords(&self) -> (u8, u8);

    fn content() -> CellContent;

    fn _get_subtree(&mut self) -> &mut Vec<impl Move<u8>>;
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
}

impl AMove for AllyMove {
    fn _new(x: u8, y: u8) -> Self {
        Self {
            x,
            y,
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
}

impl AMove for OpponentMove {
    fn _new(x: u8, y: u8) -> Self {
        Self {
            x,
            y,
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

impl TreeRoot {
    pub fn new(board: Board) -> Self {
        Self {
            board,
            tree: Tree::default(),
        }
    }
}
