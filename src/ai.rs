use crate::board::Board;
use crate::model::{CellContent, GameType, Stone};
use crate::traits::GomokuAI;
use std::path::Path;

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

    fn receive_opponent_turn(&mut self, pos: &(u8, u8)) {
        self.board.board[pos.0 as usize][pos.1 as usize] =
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
                    self.board.board[x as usize][y as usize] =
                        CellContent::Ally
                }
                Stone::Opponent => {
                    self.board.board[x as usize][y as usize] =
                        CellContent::Opponent
                }
            }
        }
    }

    fn begin(&mut self) {}
    fn end(&mut self) {}
    fn set_turn_timeout(&mut self, _time: i32) {}
    fn set_match_timeout(&mut self, _time: i32) {}
    fn set_max_memory(&mut self, _bytes: u64) {}
    fn set_time_left(&mut self, _time: i32) {}
    fn set_game_type(&mut self, _rule: GameType) {}
    fn set_persistent_folder(&mut self, _path: &Path) {}
}
