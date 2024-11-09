mod ai;
mod board;
mod model;
mod traits;

use ai::Ai;

fn main() -> std::io::Result<()> {
    let mut ai = Ai::new();

    ai.start_loop()
}
