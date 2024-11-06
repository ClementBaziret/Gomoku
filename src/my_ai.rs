use crate::my_board::{MyBoard, Status};
use std::io;

#[derive(Debug, Clone)]
pub struct MyAI {
    pub my_board: MyBoard,
    pub begin: bool,
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
    pub fn new() -> Self {
        MyAI {
            my_board: MyBoard::new(),
            begin: true,
        }
    }

    fn handle_about(&mut self, _cmd: &str) -> bool {
        let bot_name = "my AI";

        println!("name=\"{}\", version=\"1.0\", author=\"Nymand\", country=\"USA\"", bot_name);
        false
    }

    fn handle_start(&mut self, cmd: &str) -> bool {
        let parse: Vec<&str> = cmd.split_whitespace().collect();
        if let Some(&number_str) = parse.get(1) {
            if let Ok(size) = number_str.parse::<usize>() {
                if size != 20 {
                    println!("ERROR invalid size.");
                    return false;
                }
                self.my_board.resize(20);
                self.my_board.size = 20;
                println!("OK");
                return false;
            }
        }
        false
    }

    fn handle_end(&mut self, _cmd: &str) -> bool {
        true
    }

    fn handle_info(&mut self, _cmd: &str) -> bool {
        false
    }

    fn handle_begin(&mut self, _cmd: &str) -> bool {
        self.begin = true;
        self.my_board.send_new_pos();
        false
    }

    fn handle_print(&mut self, _cmd: &str) -> bool {
        self.my_board.print_board();
        false
    }

    fn handle_turn(&mut self, cmd: &str) -> bool {
        let parse: Vec<&str> = cmd.split_whitespace().collect();
        let mut x = 0;
        let mut y = 0;

        if let Some(&number_str) = parse.get(1) {
            let parts: Vec<&str> = number_str.split(',').collect();
            if let Some(x_str) = parts.get(0) {
                if let Ok(size) = x_str.parse::<usize>() {
                    x = size;
                }
            }
            if let Some(y_str) = parts.get(1) {
                if let Ok(size) = y_str.parse::<usize>() {
                    y = size;
                }
            }
        }
        self.my_board.set_cell(x, y, Status::Enemy);
        self.my_board.send_new_pos();
        false
    }

    fn handle_board(&mut self, _cmd: &str) -> bool {
        let mut input = String::new();
        self.my_board.clear_board();

        loop {
            input.clear();
            let n = io::stdin().read_line(&mut input);
            match n {
                Ok(_) => {}
                Err(_e) => {
                    return false;
                }
            }
            if input == "DONE\n" {
                break;
            }
            let parts: Vec<&str> = input.split(',').collect();
            if parts.len() != 3 {
                return false;
            }
            let x = match parts[0].parse::<usize>() {
                Ok(num) => num,
                Err(_) => {
                    return false;
                }
            };
            let y = match parts[1].parse::<usize>() {
                Ok(num) => num,
                Err(_) => {
                    return false;
                }
            };
            let player = match parts[2].trim_end().parse::<usize>() {
                Ok(num) => num,
                Err(_) => {
                    return false;
                }
            };
            if player == 1 {
                self.my_board.set_cell(x, y, Status::Ally);
            } else {
                self.my_board.set_cell(x, y, Status::Enemy);
            }
        }
        self.my_board.send_new_pos();
        false
    }

    fn send_log(&self, log_type: LogType, msg: &str) {
        let log_str = log_type.to_str();
        println!("{log_str}: {msg}");
    }

    pub fn handle_command(&mut self, cmd: &str) -> bool {
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
            "PRINT" => self.handle_print(&cmd),
            _ => {
                self.send_log(
                    LogType::Unknown,
                    "command not implemented",
                );
                false
            }
        }
    }
    pub fn start_loop(&mut self) -> std::io::Result<()> {
        let mut input = String::new();

        loop {
            input.clear();
            let n = io::stdin().read_line(&mut input)?;

            if n == 0 {
                break;
            }

            if self.handle_command(&input) {
                break;
            }
        }
        Ok(())
    }
}
