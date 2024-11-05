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

    pub fn clear_board(&mut self) {
        for status in self.board.iter_mut() {
            *status = Status::Empty;
        }
    }

    pub fn send_new_pos(&mut self) {
        let mut x: usize = 0;
        let mut y: usize = 0;
        while self.fetch_cell(x, y) != Status::Empty {
            x = x + 1;
            if x > self.size - 1 {
                x = 0;
                y = y + 1;
            }
        }
        self.set_cell(x, y, Status::Ally);
        println!("{}, {}", x, y);
    }
}
