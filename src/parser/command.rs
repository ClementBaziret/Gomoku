use std::path::Path;

use crate::model::{CellContent, GameType};

/// Command sent by the game manager to the gomoku brain
///
/// Refer to the complete piskwork protocol specification
/// for complete documentation of all the commands.
pub enum Command<IndexType> {
    /// Request to start a game
    Start(IndexType),

    /// Request for the AI to play its next move
    /// according to the opponent move passed as parameter
    Turn(IndexType, IndexType),

    /// Request for the AI to play the first move of the game
    Begin,

    /// Request for the AI to set its board to
    /// the state passed as parameter
    ///
    /// The result board will only contain stones in slots
    /// passed in parameters, all other slots are empty.
    Board(Vec<(IndexType, IndexType, CellContent)>),

    /// Request of some information by the manager to the AI
    ///
    /// See [`InfoRequest`]
    Info(InfoRequest<IndexType>),

    /// Indication to the AI that the game is over
    End,

    /// Request of info about the AI from the manager
    About,
}

/// Specific information requested to the AI by the manager
///
/// See [`Command::Info`]
pub enum InfoRequest<IndexType> {
    /// Indicates the maximum thinking time of an AI per turn
    /// 
    /// A timeout of zero means play as fast as possible.
    /// 
    /// Time is in milliseconds.
    TimeoutTurn(i32),

    /// Indicates the total duration of a game
    /// 
    /// A timeout of zero means unlimited time.
    /// 
    /// Time is represented in milliseconds.
    TimeoutMatch(i32),

    /// Indicates the maximum number of bytes the AI program is allowed to use
    MaxMemory(u64),

    /// Indicates to the AI the remaining game time
    /// 
    /// A time of 2147483647 (i32::MAX) means unlimited time.
    /// 
    /// Time is represented in milliseconds.
    TimeLeft(i32),

    /// Indicates to the AI the game type (type of opponent)
    /// 
    /// See [`GameType`]
    GameType(GameType),

    /// Indicates to the AI the game rule to apply
    /// 
    /// Quoting from the piskwork protocol:
    /// > bitmask or sum of 1=exactly five in a row win,
    /// > 2=continuous game, 4=renju, 8=caro
    Rule(i32),

    /// Indication of the mouse cursor position to the AI
    /// 
    /// The protocol states that only debug versions should respond
    /// to this command, and that release versions should ignore it.
    Evaluate(IndexType, IndexType),

    /// Indication of a folder in which the AI can place persistent files
    /// 
    /// The AI should place its files in a subfolder within this folder,
    /// to avoid collision with other AIs playing at the same time.
    /// This subfolder should have the name of the AI executable program.
    Folder(Box<Path>),
}

