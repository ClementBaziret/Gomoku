use std::str::FromStr;

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

impl CellContent {
    pub fn to_string(&self) -> &str {
        match self {
            CellContent::Empty => "Empty",
            CellContent::Opponent => "Opponent",
            CellContent::Ally => "Ally",
        }
    }
    pub fn to_char(&self) -> char {
        match self {
            CellContent::Empty => '.',
            CellContent::Opponent => 'X',
            CellContent::Ally => 'O',
        }
    }
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

impl FromStr for GameType {
    type Err = &'static str;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "0" => Ok(Self::Human),
            "1" => Ok(Self::Brain),
            "2" => Ok(Self::Tournament),
            "3" => Ok(Self::NetworkTournament),
            _ => Err("Invalid value, only accepting 0-3"),
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

#[test]
fn build_game_type_from_string() {
    let game_type: Result<GameType, _> = "2".parse();

    assert_eq!(game_type, Ok(GameType::Tournament));
}
