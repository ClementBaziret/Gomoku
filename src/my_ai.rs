use crate::my_board::{MyBoard, Status};
use std::io;

use std::io::{Write};
use std::fs::File;
use rand::Rng;

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
                self.my_board.resize(size);
                self.my_board.size = size;
                println!("OK");
                return false;
            }
        }
        return false;
    }

    fn handle_end(&mut self, _cmd: &str) -> bool {
        true
    }

    fn handle_info(&mut self, _cmd: &str) -> bool {
        false
    }

    fn handle_begin(&mut self, _cmd: &str, file: &File) -> bool {
        self.begin = true;
        self.my_board.send_new_pos(&file);
        return false;
    }

    fn handle_turn(&mut self, cmd: &str, file: &File) -> bool {
        let parse: Vec<&str> = cmd.split_whitespace().collect();
        let mut x = 0;
        let mut y = 0;

        if let Some(&number_str) = parse.get(1) {
            if let Ok(size) = number_str.parse::<usize>() {
                x = size;
            }
        }
        if let Some(&number_str) = parse.get(1) {
            if let Ok(size) = number_str.parse::<usize>() {
                y = size;
            }
        }

        self.my_board.set_cell(x, y, Status::Enemy);
        self.my_board.send_new_pos(&file);
        return false;
    }

    fn handle_board(&mut self, _cmd: &str) -> bool {
        todo!()
    }

    fn send_log(&self, log_type: LogType, msg: &str) {
        let log_str = log_type.to_str();
        println!("{log_str}: {msg}");
    }

    pub fn handle_command(&mut self, cmd: &str, file: &File) -> bool {
        let uppercase = cmd.split_whitespace().next().unwrap().to_uppercase();
        let token = uppercase.as_str();

        match token {
            "ABOUT" => self.handle_about(&cmd),
            "START" => self.handle_start(&cmd),
            "END" => self.handle_end(&cmd),
            "INFO" => self.handle_info(&cmd),
            "BEGIN" => self.handle_begin(&cmd, &file),
            "TURN" => self.handle_turn(&cmd, &file),
            "BOARD" => self.handle_board(&cmd),
            _ => {
                self.send_log(LogType::Unknown, "command not implemented");
                false
            }
        }
    }
    pub fn start_loop(&mut self) -> std::io::Result<()> {
        let mut input = String::new();
        let mut rng = rand::thread_rng();
        let y: u8 = rng.gen(); // generates a float between 0 and 1
        let mut file = File::create(format!("input{}.txt", y))?;
        let mut file2 = File::create(format!("output{}.txt", y))?;

        loop {
                input.clear();
                let n = io::stdin().read_line(&mut input)?;
                // file.write_all(input.as_bytes())?;
        
                if n == 0 {
                    break;
                }
                
                if self.handle_command(&input, &file2) {
                    break;
                }
                // ai.my_board.print();
            }
            file2.flush()?;
            file.flush()?;
            return Ok(());
    }
}
