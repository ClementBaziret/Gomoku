use std::path::Path;

use crate::model::{CellContent, GameType};

pub enum Command<'a, IndexType> {
    Start(IndexType),

    Turn(IndexType, IndexType),

    Begin,

    Board(&'a [(IndexType, IndexType, CellContent)]),

    Info(InfoRequest<'a, IndexType>),

    End,

    About,
}

pub enum InfoRequest<'a, IndexType> {
    TimeoutTurn(i32),
    TimeoutMatch(i32),
    MaxMemory(u64),
    TimeLeft(i32),
    GameType(GameType),
    Rule(GomokuRule),
    Evaluate(IndexType, IndexType),
    Folder(&'a Path),
}

pub enum GomokuRule {
    Gomoku
}
