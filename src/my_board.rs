use rand::Rng;
use std::fs::File;
use std::io::Write;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Status {
    Empty,
    Enemy,
    Ally,
}

impl Status {
    pub fn to_str(&self) -> &str {
        match self {
            Self::Empty => "Empty",
            Self::Enemy => "Enemy",
            Self::Ally => "Ally",
        }
    }
}

#[derive(Debug, Clone)]
pub struct MyBoard {
    pub board: Vec<Status>,
    pub size: usize,
}

struct Move {
    move_position: usize,
    next_moves: Vec<Move>,
}

impl MyBoard {
    pub fn new() -> Self {
        MyBoard {
            board: Vec::new(),
            size: 0,
        }
    }

    pub fn resize(&mut self, size: usize) {
        let new_size = size * size;
        self.board = vec![Status::Empty; new_size];
    }

    pub fn fetch_cell(&mut self, x: usize, y: usize) -> Status {
        let cell = (y * self.size) + x;
        self.board[cell]
    }

    pub fn set_cell(&mut self, x: usize, y: usize, state: Status) {
        let cell = (y * self.size) + x;
        self.board[cell] = state;
    }

    pub fn print(&self) {
        for (i, status) in self.board.iter().enumerate() {
            let symbol = match status {
                Status::Empty => '.',
                Status::Enemy => 'X',
                Status::Ally => 'O',
            };
            print!("{} ", symbol);
            if (i + 1) % self.size == 0 {
                println!();
            }
        }
    }

    fn calculate_next_move(&self) -> usize {
        let root = self.generate_tree();
        let mut best_move = &Move {
            move_position: usize::MAX,
            next_moves: vec![],
        };
        let mut best_move_value: u32 = 0;

        for child in &root.next_moves {
            println!("Move position: {}", child.move_position);
            let move_value = self.evaluate_board(child);
            if move_value > best_move_value {
                best_move_value = move_value;
                best_move = child;
            }
        }
        return best_move.move_position;
    }

    fn generate_tree(&self) -> Move {
        let mut root = Move {
            move_position: usize::MAX,
            next_moves: Vec::new(),
        };

        for (index, status) in self.board.iter().enumerate() {
            if matches!(status, Status::Empty) {
                let child_move = Move {
                    move_position: index,
                    next_moves: Vec::new(),
                };
                root.next_moves.push(child_move);
            }
        }
        return root;
    }

    fn evaluate_board(&self, _move: &Move) -> u32 {
        return rand::thread_rng().gen_range(0..=100);
    }

    pub fn send_new_pos(&mut self, mut file: &File) {
        // send a random cell
        let mut rng = rand::thread_rng();

        let mut x: usize = rng.gen_range(0..=19);
        let mut y: usize = rng.gen_range(0..=19);

        let _ = file.write_all(
            format!(
                "checked value before: {}, {}, {}\n",
                x,
                y,
                self.fetch_cell(x, y).to_str()
            )
            .as_bytes(),
        );
        while self.fetch_cell(x, y) != Status::Empty {
            let _ = file.write_all(
                format!("failed value: {}, {}\n", x, y).as_bytes(),
            );

            x = rng.gen_range(0..=19);
            y = rng.gen_range(0..=19);
        }
        self.set_cell(x, y, Status::Ally);
        let _ = file.write_all(
            format!(
                "checked value after : {}, {}, {}\n",
                x,
                y,
                self.fetch_cell(x, y).to_str()
            )
            .as_bytes(),
        );
        println!("{}, {}", x, y);
    }
}
