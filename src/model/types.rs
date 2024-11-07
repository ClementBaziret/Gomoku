
/// Enumerates the two possible stone types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stone {
    /// Stone placed by the current player.
    /// 
    /// Defined as numerical value 1 in the piskwork protocol
    Ally = 1,

    /// Stone placed by the opponent player.
    /// 
    /// Defined as numerical value 2 in the piskwork protocol
    Opponent = 2,
}

/// Enumerates all possible states of a Gomoku cell
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellContent {
    /// No stone placed in this cell.
    /// 
    /// Defined as numerical value 0 in the piskwork protocol
    Empty = 0,

    /// Cell contains a stone placed by the current player.
    /// 
    /// See [`Stone::Ally`]
    Ally = Stone::Ally as isize,

    /// Cell contains a stone placed by the opponent player.
    /// 
    /// See [`Stone::Opponent`]
    Opponent = Stone::Opponent as isize,
}

/// Describes all the possible game types defined
/// by the `INFO game_type` command
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameType {
    /// AI plays against a human
    Human,

    /// AI plays against another AI
    Brain,

    /// AI plays in a tournament
    Tournament,

    /// AI plays in a network tournament
    NetworkTournament,
}

impl GameType {
    fn from_string(option: &str) -> Option<Self> {
        match option {
            "0" => Some(Self::Human),
            "1" => Some(Self::Brain),
            "2" => Some(Self::Tournament),
            "3" => Some(Self::NetworkTournament),
            _ => None
        }
    }
}

#[test]
fn stone_is_one_byte() {
    assert_eq!(std::mem::size_of::<Stone>(), 1);
}

#[test]
fn cell_is_one_byte() {
    assert_eq!(std::mem::size_of::<CellContent>(), 1);
}
