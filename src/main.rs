use rand::Rng;
use std::io; // Add this to use random number generation

enum Status {
    Empty,
    Ennemy,
    Ally,
}

struct MyAI {
    pub board: Vec<Status>,
}

// #[derive(Clone, Copy)]
struct Node {
    move_position: usize, // Index of the move on the board
    children: Vec<Node>,  // Possible next moves
}

impl MyAI {
    fn calculate_next_move(&self) -> usize {
        let root = self.generate_tree();
        let mut best_move = &Node {
            move_position: usize::MAX,
            children: vec![],
        };
        let mut best_move_value: u32 = 0;

        for child in &root.children {
            println!("Move position: {}", child.move_position);
            let move_value = self.evaluate_board(child);
            if move_value > best_move_value {
                best_move_value = move_value;
                best_move = child;
            }
        }
        return best_move.move_position;
    }

    fn generate_tree(&self) -> Node {
        let mut root = Node {
            move_position: usize::MAX,
            children: Vec::new(),
        };

        for (index, status) in self.board.iter().enumerate() {
            if matches!(status, Status::Empty) {
                let child_node = Node {
                    move_position: index,
                    children: Vec::new(),
                };
                root.children.push(child_node);
            }
        }
        return root;
    }

    fn evaluate_board(&self, _move: &Node) -> u32 {
        let mut rng = rand::thread_rng();
        return rng.gen_range(0..=100);
    }
}

enum LogType {
    Unknown,
    Error,
    Message,
    Debug,
}

impl LogType {
    fn to_str(&self) -> &str {
        match self {
            Self::Unknown => "UNKNOWN",
            Self::Error => "ERROR",
            Self::Message => "MESSAGE",
            Self::Debug => "DEBUG",
        }
    }
}

impl MyAI {
    fn new() -> Self {
        Self { board: vec![] }
    }

    fn handle_about(&mut self, _cmd: &str) -> bool {
        let bot_name = "rust_template";
        println!("name=\"{bot_name}\", version\"0.42\"");
        false
    }

    fn handle_start(&mut self, _cmd: &str) -> bool {
        todo!()
    }

    fn handle_end(&mut self, _cmd: &str) -> bool {
        true
    }

    fn handle_info(&mut self, _cmd: &str) -> bool {
        todo!()
    }

    fn handle_begin(&mut self, _cmd: &str) -> bool {
        todo!()
    }

    fn handle_turn(&mut self, _cmd: &str) -> bool {
        todo!()
    }

    fn handle_board(&mut self, _cmd: &str) -> bool {
        todo!()
    }

    fn send_log(&self, log_type: LogType, msg: &str) {
        let log_str = log_type.to_str();
        println!("{log_str}: {msg}");
    }

    fn handle_command(&mut self, cmd: &str) -> bool {
        let uppercase =
            cmd.split_whitespace().next().unwrap().to_uppercase();
        let token = uppercase.as_str();

        match token {
            "ABOUT" => self.handle_about(&cmd),
            "START" => self.handle_start(&cmd),
            "END" => self.handle_end(&cmd),
            "INFO" => self.handle_info(&cmd),
            "BEGIN" => self.handle_begin(&cmd),
            "TURN" => self.handle_turn(&cmd),
            "BOARD" => self.handle_board(&cmd),
            _ => {
                self.send_log(
                    LogType::Unknown,
                    "command not implemented",
                );
                false
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut ai = MyAI::new();
    let mut input = String::new();

    loop {
        input.clear();
        let n = io::stdin().read_line(&mut input)?;

        if n == 0 {
            return Ok(());
        }

        if ai.handle_command(&input) {
            return Ok(());
        }
    }
}
