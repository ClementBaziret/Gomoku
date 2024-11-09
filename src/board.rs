use crate::model::CellContent;

#[derive(Debug, Clone)]
pub struct Board {
    pub board: [[CellContent; 20]; 20],
    pub size: u8,
}

struct Move {
    x: u8,
    y: u8,
    next_moves: Vec<Move>,
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
        let mut best_move_value: u32 = 0;

        for child in &root.next_moves {
            let move_value = self.evaluate_board(child);
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

    fn evaluate_board(&self, _move: &Move) -> u32 {
        return 10; // Fixed evaluation value for now
    }

    pub fn send_new_pos(&mut self) -> (u8, u8) {
        let (x, y) = self.calculate_next_move();

        self.board[y as usize][x as usize] = CellContent::Ally;
        println!("{},{}", x, y);
        (x, y)
    }
}
