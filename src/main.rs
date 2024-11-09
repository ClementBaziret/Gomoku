mod ai;
mod board;
mod model;
mod parser;
mod traits;

use parser::Parser;

fn main() -> std::io::Result<()> {
    let mut parser = Parser::new();

    parser.start_loop()
}
