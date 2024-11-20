mod ai;
mod board;
mod evaluation;
mod grid_iterators;
mod model;
mod parser;
#[cfg(test)]
mod tests;
mod traits;
mod tree;

use parser::Parser;

fn main() -> std::io::Result<()> {
    let mut parser = Parser::new();

    parser.start_loop()
}
