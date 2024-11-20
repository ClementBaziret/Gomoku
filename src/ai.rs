use crate::board::Board;
use crate::model::{CellContent, Stone};
use crate::traits::GomokuAI;

pub struct Ai {
    pub board: Board,
}

impl Ai {
    const ABOUT_MAP: [(&'static str, &'static str); 4] = [
        ("name", "MyAi"),
        ("version", "0.1"),
        ("author", "L'eau ça rouille"),
        ("country", "France"),
    ];

    pub fn new() -> Self {
        Ai {
            board: Board::new(),
        }
    }
}

impl GomokuAI<u8> for Ai {
    fn start(board_size: u8) -> Option<Self> {
        let ai = Ai::new();

        if ai.board.size != board_size {
            return None;
        }
        Some(ai)
    }

    fn receive_opponent_turn(&mut self, (x, y): &(u8, u8)) {
        self.board.board[*y as usize][*x as usize] =
            CellContent::Opponent;
    }

    fn play(&mut self) -> (u8, u8) {
        self.board.send_new_pos()
    }

    fn about(&self) -> &[(&str, &str)] {
        &Ai::ABOUT_MAP
    }

    fn set_board(&mut self, stones: &[(u8, u8, Stone)]) {
        for &(x, y, stone) in stones {
            // Ensure x and y are within bounds of the board
            if x >= self.board.size || y >= self.board.size {
                continue;
            }
            match stone {
                Stone::Ally => {
                    self.board.board[y as usize][x as usize] =
                        CellContent::Ally
                }
                Stone::Opponent => {
                    self.board.board[y as usize][x as usize] =
                        CellContent::Opponent
                }
            }
        }
    }
}
