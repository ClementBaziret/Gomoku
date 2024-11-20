use crate::board::Board;
use crate::model::{CellContent, Stone};
use crate::traits::GomokuAI;

pub struct Ai {
    pub board: Board,
}

impl Ai {
    const ABOUT_MAP: [(&'static str, &'static str); 4] = [
        ("name", "Patrice"),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn horizontal_win() {
        let mut ai = Ai::new();

        ai.set_board(&[
            (5, 3, Stone::Opponent),
            (5, 4, Stone::Ally),
            (5, 5, Stone::Ally),
            (5, 6, Stone::Ally),
            (5, 7, Stone::Ally),
        ]);

        let ai_move = ai.play();

        assert_eq!(ai_move, (5, 8));
    }

    #[test]
    fn vertical_win() {
        let mut ai = Ai::new();

        ai.set_board(&[
            (1, 1, Stone::Ally),
            (2, 1, Stone::Ally),
            (3, 1, Stone::Ally),
            (4, 1, Stone::Ally),
        ]);

        let ai_move = ai.play();

        assert!(ai_move == (0, 1) || ai_move == (5, 1));
    }

    #[test]
    fn up_right_diagonal_win() {
        let mut ai = Ai::new();

        ai.set_board(&[
            (5, 1, Stone::Ally),
            (4, 2, Stone::Ally),
            (3, 3, Stone::Ally),
            (2, 4, Stone::Ally),
        ]);

        let ai_move = ai.play();

        assert!(ai_move == (6, 0) || ai_move == (1, 5));
    }

    #[test]
    fn down_right_diagonal_win() {
        let mut ai = Ai::new();

        ai.set_board(&[
            (1, 1, Stone::Ally),
            (2, 2, Stone::Ally),
            (3, 3, Stone::Ally),
            (4, 4, Stone::Ally),
        ]);

        let ai_move = ai.play();

        assert!(ai_move == (0, 0) || ai_move == (5, 5));
    }

    #[test]
    fn horizontal_lose() {
        let mut ai = Ai::new();

        ai.set_board(&[
            (5, 4, Stone::Opponent),
            (5, 5, Stone::Opponent),
            (5, 6, Stone::Opponent),
            (5, 7, Stone::Opponent),
            (5, 8, Stone::Ally),
        ]);

        let ai_move = ai.play();

        assert_eq!(ai_move, (5, 3));
    }

    #[test]
    fn vertical_lose() {
        let mut ai = Ai::new();

        ai.set_board(&[
            (0, 1, Stone::Ally),
            (1, 1, Stone::Opponent),
            (2, 1, Stone::Opponent),
            (3, 1, Stone::Opponent),
            (4, 1, Stone::Opponent),
        ]);

        let ai_move = ai.play();

        assert_eq!(ai_move, (5, 1));
    }

    #[test]
    fn up_right_diagonal_lose() {
        let mut ai = Ai::new();

        ai.set_board(&[
            (6, 0, Stone::Ally),
            (5, 1, Stone::Opponent),
            (4, 2, Stone::Opponent),
            (3, 3, Stone::Opponent),
            (2, 4, Stone::Opponent),
        ]);

        let ai_move = ai.play();

        assert_eq!(ai_move, (1, 5));
    }

    #[test]
    fn down_right_diagonal_lose() {
        let mut ai = Ai::new();

        ai.set_board(&[
            (0, 0, Stone::Ally),
            (1, 1, Stone::Opponent),
            (2, 2, Stone::Opponent),
            (3, 3, Stone::Opponent),
            (4, 4, Stone::Opponent),
        ]);

        let ai_move = ai.play();

        assert_eq!(ai_move, (5, 5));
    }
}
