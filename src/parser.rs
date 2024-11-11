use crate::model::CellContent;
use std::io::Write;
use std::process;
use std::{fs::File, io};

use crate::ai::Ai;
use crate::traits::GomokuAI;

pub struct Parser {
    ai: Ai,

    #[cfg(debug_assertions)]
    pub input_file: File,
    #[cfg(debug_assertions)]
    pub output_file: File,
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

impl Parser {
    pub fn new() -> Self {
        #[cfg(debug_assertions)]
        let pid = process::id();
        #[cfg(debug_assertions)]
        let input_file = File::create(format!("input_{}.txt", pid))
            .expect("Failed to create input file in debug mode");
        #[cfg(debug_assertions)]
        let output_file = File::create(format!("output_{}.txt", pid))
            .expect("Failed to create output file in debug mode");

        Parser {
            ai: Ai::new(),
            #[cfg(debug_assertions)]
            input_file,
            #[cfg(debug_assertions)]
            output_file,
        }
    }

    pub fn write_to_input_file(&mut self, mes: &str) {
        #[cfg(debug_assertions)]
        let _res = write!(self.input_file, "{}", mes);
    }

    pub fn write_to_output_file(&mut self, mes: &str) {
        #[cfg(debug_assertions)]
        let _res = write!(self.output_file, "{}", mes);
    }

    fn send_play(&mut self) {
        let play = self.ai.play();
        println!("{},{}", play.0, play.1);
    }

    fn handle_about(&mut self, _cmd: &str) -> bool {
        for info in self.ai.about() {
            print!("{}=\"{}\", ", info.0, info.1);
        }
        print!("\n");
        false
    }

    fn handle_start(&mut self, cmd: &str) -> bool {
        let parse: Vec<&str> = cmd.split_whitespace().collect();
        if let Some(&number_str) = parse.get(1) {
            if let Ok(size) = number_str.parse::<usize>() {
                if size != self.ai.board.size as usize {
                    self.write_to_output_file("ERROR invalid size.");
                    println!("ERROR invalid size.");
                    return false;
                }
                self.write_to_output_file("OK");
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
        self.send_play();

        false
    }

    fn handle_print(&mut self, _cmd: &str) -> bool {
        self.ai.board.print_board();

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
        self.ai.receive_opponent_turn(&(x as u8, y as u8));
        self.send_play();

        false
    }

    fn handle_board(&mut self, _cmd: &str) -> bool {
        let mut input = String::new();
        self.ai.board.clear_board();

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
                self.ai.board.board[y as usize][x as usize] =
                    CellContent::Ally;
            } else {
                self.ai.board.board[y as usize][x as usize] =
                    CellContent::Opponent;
            }
        }
        self.send_play();

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

            self.write_to_input_file(&input);

            if n == 0 {
                break;
            }
            if input == "\n" {
                continue;
            }

            if self.handle_command(&input) {
                break;
            }
        }
        Ok(())
    }
}
