use crate::evaluation::evaluate;
use crate::model::CellContent;

pub struct Move {
    pub x: u8,
    pub y: u8,
    pub next_moves: Vec<Move>,
}

#[derive(Copy, Debug, Clone)]
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
                // let symbol = match x {
                //     CellContent::Empty => '.',
                //     CellContent::Opponent => 'X',
                //     CellContent::Ally => 'O',
                // };
                print!("{} ", x.to_char());
            }
            println!();
        }
    }

    pub fn clear_board(&mut self) {
        self.board = [[CellContent::Empty; 20]; 20];
    }

    pub fn calculate_next_move(&self) -> (u8, u8) {
        let root = self.generate_tree(self);
        let mut best_move = &Move {
            x: u8::MAX,
            y: u8::MAX,
            next_moves: vec![],
        };
        let mut best_move_value: i32 = i32::MIN;

        let mut board_copy = *self;
        for child in &root.next_moves {
            let mut move_value: i32 = -5;
            // The condition `self.too_far` seems to be broken so
            // it is turned off by the `|| true for the AI to find its move`
            if (self.too_far(child)) == false || true {
                board_copy.board[child.y as usize]
                [child.x as usize] = CellContent::Ally;
                move_value = board_copy.evaluate_board();
                board_copy.board[child.y as usize]
                [child.x as usize] = CellContent::Empty;
                println!("{} {}: {}", child.x, child.y, move_value);
            }
            if move_value > best_move_value {
                best_move_value = move_value;
                best_move = child;
            }
        }
        // println!("{} {} : {}", best_move.x, best_move.y, best_move_value);
        (best_move.x, best_move.y)
    }

    fn is_stone_nearby(
        &self,
        board: &Board,
        x: usize,
        y: usize,
    ) -> bool {
        #[rustfmt::skip]
        const DIRECTIONS: [(isize, isize); 8] = [
            (-1, -1), (-1, 0), (-1, 1),
            (0, -1),          (0, 1),
            (1, -1), (1, 0), (1, 1),
        ];

        // Iterate over each surrounding cell
        for (dy, dx) in DIRECTIONS.iter() {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            // Check if the position is within bounds
            if nx >= 0
                && nx < board.size as isize
                && ny >= 0
                && ny < board.size as isize
            {
                let nx = nx as usize;
                let ny = ny as usize;

                if board.board[ny][nx] != CellContent::Empty {
                    return true;
                }
            }
        }
        return false;
    }

    fn generate_tree(&self, board: &Board) -> Move {
        let mut root = Move {
            // I don't really know what values to put there, could you help me ?
            x: u8::MAX,
            y: u8::MAX,
            next_moves: Vec::new(),
        };

        for y in 0..self.size {
            for x in 0..self.size {
                if self.is_stone_nearby(board, x as usize, y as usize)
                    && board.board[y as usize][x as usize]
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
        println!("{}", ret);
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
