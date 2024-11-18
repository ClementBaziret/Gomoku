use crate::model::CellContent;
use crate::tree::evaluate_move_recursive;
use crate::tree::AllyMove;
use crate::tree::Move;
use crate::tree::Tree;
use crate::tree::TreeRoot;

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
        let mut board_copy = self.clone();
        let mut root: TreeRoot = TreeRoot {
            board: *self,
            tree: Tree::gen_tree(&mut board_copy, 1),
        };

        let mut best_move = AllyMove::new(0, 0);
        match root.tree.moves.first() {
            Some(m) => {
                best_move = AllyMove::new(m.x, m.y);
            }
            None => {}
        }
        let mut best_move_value: i32 = i32::MIN;

        for mut child in root.tree.moves {
            let move_value: i32 =
                evaluate_move_recursive(&mut child, &mut root.board);
            if move_value > best_move_value {
                best_move_value = move_value;
                best_move = child;
            }
        }
        // println!("{} {} : {}", best_move.x, best_move.y, best_move_value);
        (best_move.x, best_move.y)
    }

    fn is_stone_nearby(&self, x: usize, y: usize) -> bool {
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
                && nx < self.size as isize
                && ny >= 0
                && ny < self.size as isize
            {
                let nx = nx as usize;
                let ny = ny as usize;

                if self.board[ny][nx] != CellContent::Empty {
                    return true;
                }
            }
        }
        return false;
    }

    fn is_valid_adjacent_cell(&self, x: u8, y: u8) -> bool {
        let (x_us, y_us) = (x as usize, y as usize);

        if self.board[y_us][x_us] != CellContent::Empty {
            false
        } else {
            self.is_stone_nearby(x_us, y_us)
        }
    }

    pub fn get_adjacent_cells(&self) -> Vec<(u8, u8)> {
        let mut ret = Vec::new();

        for (y, row) in self.board.iter().enumerate() {
            for (x, _) in row.iter().enumerate() {
                if self.is_valid_adjacent_cell(x as u8, y as u8) {
                    ret.push((x as u8, y as u8));
                }
            }
        }

        ret
    }

    pub fn send_new_pos(&mut self) -> (u8, u8) {
        let (x, y) = self.calculate_next_move();

        self.board[y as usize][x as usize] = CellContent::Ally;
        (x, y)
    }
}
