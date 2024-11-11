use std::path::Path;

use crate::model::{GameType, Stone};

/// Trait which defines possible interactions with a Gomoku AI
pub trait GomokuAI<IndexType = usize> {
    /// Start the AI with a given square board size. The AI
    /// is allowed to refuse playing on with this board
    /// size, in which case it should return `None`.
    fn start(board_size: IndexType) -> Option<Self>
    where
        Self: Sized;

    /// The manager informs the AI of a move made by the opponent.
    ///
    /// The AI should update its internal board to remember the opponent move.
    fn receive_opponent_turn(&mut self, pos: &(IndexType, IndexType));

    /// This asks the AI to play its next turn:
    /// - Return the move it decided to play
    /// - Update its internal board to remember it played the move
    fn play(&mut self) -> (IndexType, IndexType);

    /// The manager asks the AI for information about itself.
    ///
    /// Commonly defined fields are name, version, author, country, www, email
    fn about(&self) -> &[(&str, &str)];

    /// Reset the board to any content.
    ///
    /// Requires the AI to reset its internal board and then
    /// place the stones passed in parameter on it, effectively
    /// changing the entire state of the board to any possible
    /// combination in a single call.
    fn set_board(&mut self, stones: &[(IndexType, IndexType, Stone)]);

    /// The manager informs the AI that it will play first in the game
    fn begin(&mut self) {}

    /// The manager informs the AI that the game is over
    fn end(&mut self) {}

    /// The managers asks for the AI to adapts its
    /// response time, zero means play as fast as possible
    ///
    /// Time is represented in milliseconds.
    fn set_turn_timeout(&mut self, _time: i32) {}

    /// The managers informs the AI of the total match duration.
    /// Zero means unlimited time.
    ///
    /// Time is represented in milliseconds.
    fn set_match_timeout(&mut self, _time: i32) {}

    /// The manager informs the AI of a memory limit
    /// in bytes that the AI is allowed to use.
    fn set_max_memory(&mut self, _bytes: u64) {}

    /// The manager informs the AI of the remaining time of the game.
    /// A value of 2147483647 (i32::MAX) means unlimited time
    ///
    /// Time is represented in milliseconds
    fn set_time_left(&mut self, _time: i32) {}

    /// The manager informs the AI of the game type
    ///
    /// See documentation of `GameType` for more info
    fn set_game_type(&mut self, _rule: GameType) {}

    /// The manager informs the AI of a folder path in which
    /// the AI can store persistent files.
    ///
    /// It is to be noted that this folder will be shared
    /// by all AIs, thus each AI should create a subfolder
    /// with the same name as its executable name, and place
    /// its files in this subfolder.
    fn set_persistent_folder(&mut self, _path: &Path) {}
}
