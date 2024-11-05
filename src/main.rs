mod my_board;
mod my_ai;

use my_ai::MyAI;

fn main() -> std::io::Result<()> {
    let mut ai = MyAI::new();

    ai.start_loop()
}