use core::num;
use std::{error::Error, str::FromStr};

use super::command::Command;

impl FromStr for Command<u8> {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split_whitespace();

        match words.next() {
            Some("Start") => {
                let num_str = words.next().ok_or("Expects an arg")?;

                match num_str.parse() {
                    Ok(num) => Ok(Self::Start(num)),
                    Err(parse_error) => Err(parse_error.into()),
                }
            }
            Some("Turn") => {
                let mut args =
                    words.next().ok_or("Expected arg")?.split(',');

                let (x, y): (u8, u8) = (
                    args.next().ok_or("Expecting number")?.parse()?,
                    args.next().ok_or("Expecting number")?.parse()?,
                );

                Ok(Self::Turn(x, y))
            }
            Some("Begin") => Ok(Self::Begin),
            Some("End") => Ok(Self::End),
            Some(_) => Err("Unknown command".into()),
            None => Err("Empty command".into()),
        }
    }
}
