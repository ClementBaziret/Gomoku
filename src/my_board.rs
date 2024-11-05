use rand::Rng;
use std::io::{Write};
use std::fs::File;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Status {
    Empty,
    Enemy,
    Ally,
}

#[derive(Debug, Clone)]
pub struct MyBoard {
    pub board: Vec<Status>,
    pub size: usize,
}

impl MyBoard {
    pub fn new() -> Self {
        MyBoard {
            board: Vec::new(), size: 0
        }
    }

    pub fn resize(&mut self, size: usize) {
        let new_size = size * size;
        self.board = vec![Status::Empty; new_size];
    }

    pub fn fetch_cell(&mut self, x: usize, y:usize) -> Status {
        let place = (y * self.size) + x;
        return self.board[place];
    }

    pub fn set_cell(&mut self, x: usize, y:usize, state: Status) {
        let place = (y * self.size) + x;
        self.board[place] = state;
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
    pub fn send_new_pos(&mut self, mut file: &File) {
        // send a random cell
        let mut rng = rand::thread_rng();

        let mut x: usize = 0;
        let mut y: usize = 0;

        while self.board[(20 * y) + x] != Status::Empty {
            // file.write_all(format!("failed value: {}, {}\n", x, y).as_bytes());

            x = rng.gen_range(0..=19);
            y = rng.gen_range(0..=19);
            
        }
        self.set_cell(x, y, Status::Ally);
        println!("{}, {}", x, y);
        // file.write_all(format!("new pos: {}, {}\n", x, y).as_bytes());
    }
}