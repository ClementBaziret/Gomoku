mod model;
mod my_ai;
mod my_board;
mod traits;
mod evaluation;

use my_ai::MyAI;

fn main() -> std::io::Result<()> {
    let mut ai = MyAI::new();

    ai.start_loop()
}
