use std::cmp::min;

use crate::evaluation::evaluate;
use crate::model::CellContent;

pub struct Move {
    pub x: u8,
    pub y: u8,
    pub next_moves: Vec<Move>,
}

#[derive(Debug, Clone)]
pub struct Board {
    pub board: [[CellContent; 20]; 20],
    pub size: u8,
}

impl Board {
    pub fn new() -> Self {
        Board {
            board: [[CellContent::Empty; 20]; 20],
            size: 20,
        }
    }

    pub fn print_board(&self) {
        for y in self.board.iter() {
            for x in y.iter() {
                let symbol = match x {
                    CellContent::Empty => '.',
                    CellContent::Opponent => 'X',
                    CellContent::Ally => 'O',
                };
                print!("{} ", symbol);
            }
            println!();
        }
    }

    pub fn clear_board(&mut self) {
        self.board = [[CellContent::Empty; 20]; 20];
    }

    pub fn calculate_next_move(&self) -> (u8, u8) {
        let root = self.generate_tree();
        let mut best_move = &Move {
            // I don't really know what values to put there, could you help me ?
            x: u8::MAX,
            y: u8::MAX,
            next_moves: vec![],
        };
        let mut best_move_value: i32 = i32::MIN;

        for child in &root.next_moves {
            let mut move_value: i32 = -5;
            if (self.too_far(child)) == false {
                move_value = self.evaluate_board();
            }
            if move_value > best_move_value {
                best_move_value = move_value;
                best_move = child;
            }
        }
        (best_move.x, best_move.y)
    }

    fn generate_tree(&self) -> Move {
        let mut root = Move {
            // I don't really know what values to put there, could you help me ?
            x: u8::MAX,
            y: u8::MAX,
            next_moves: Vec::new(),
        };

        for y in 0..self.size {
            for x in 0..self.size {
                if self.board[y as usize][x as usize]
                    == CellContent::Empty
                {
                    let child_move = Move {
                        x,
                        y,
                        next_moves: Vec::new(),
                    };
                    root.next_moves.push(child_move);
                }
            }
        }
        root
    }

    fn evaluate_board(&self) -> i32 {
        let ret = evaluate(self);
        ret
    }

    fn too_far(&self, pos: &Move) -> bool {
        let x = pos.x as i32;
        let y = pos.y as i32;

        let x_min = (x - 2).max(0);
        let x_max = (x + 2).min(self.size as i32 - 1);
        let y_min = (y - 2).max(0);
        let y_max = (y + 2).min(self.size as i32 - 1);

        for i in x_min..=x_max {
            for j in y_min..=y_max {
                if (i != x || j != y)
                    && self.board[i as usize][j as usize]
                        != CellContent::Empty
                {
                    return false;
                }
            }
        }
        true
    }

    pub fn send_new_pos(&mut self) -> (u8, u8) {
        let (x, y) = self.calculate_next_move();

        self.board[y as usize][x as usize] = CellContent::Ally;
        (x, y)
    }
}
