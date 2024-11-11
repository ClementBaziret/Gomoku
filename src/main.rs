mod ai;
mod board;
mod model;
mod parser;
mod traits;
mod grid_iterators;

use parser::Parser;

fn main() -> std::io::Result<()> {
    let mut parser = Parser::new();

    parser.start_loop()
}
