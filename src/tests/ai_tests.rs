#[cfg(test)]
use crate::ai::Ai;
#[cfg(test)]
use crate::model::Stone;
#[cfg(test)]
use crate::traits::GomokuAI;

#[test]
fn horizontal_win() {
    let mut ai = Ai::new();

    ai.set_board(&[
        (5, 3, Stone::Opponent),
        (5, 4, Stone::Ally),
        (5, 5, Stone::Ally),
        (5, 6, Stone::Ally),
        (5, 7, Stone::Ally),
    ]);

    let ai_move = ai.play();

    assert_eq!(ai_move, (5, 8));
}

#[test]
fn vertical_win() {
    let mut ai = Ai::new();

    ai.set_board(&[
        (1, 1, Stone::Ally),
        (2, 1, Stone::Ally),
        (3, 1, Stone::Ally),
        (4, 1, Stone::Ally),
    ]);

    let ai_move = ai.play();

    assert!(ai_move == (0, 1) || ai_move == (5, 1));
}

#[test]
fn up_right_diagonal_win() {
    let mut ai = Ai::new();

    ai.set_board(&[
        (5, 1, Stone::Ally),
        (4, 2, Stone::Ally),
        (3, 3, Stone::Ally),
        (2, 4, Stone::Ally),
    ]);

    let ai_move = ai.play();

    assert!(ai_move == (6, 0) || ai_move == (1, 5));
}

#[test]
fn down_right_diagonal_win() {
    let mut ai = Ai::new();

    ai.set_board(&[
        (1, 1, Stone::Ally),
        (2, 2, Stone::Ally),
        (3, 3, Stone::Ally),
        (4, 4, Stone::Ally),
    ]);

    let ai_move = ai.play();

    assert!(ai_move == (0, 0) || ai_move == (5, 5));
}

#[test]
fn horizontal_lose() {
    let mut ai = Ai::new();

    ai.set_board(&[
        (5, 4, Stone::Opponent),
        (5, 5, Stone::Opponent),
        (5, 6, Stone::Opponent),
        (5, 7, Stone::Opponent),
        (5, 8, Stone::Ally),
    ]);

    let ai_move = ai.play();

    assert_eq!(ai_move, (5, 3));
}

#[test]
fn vertical_lose() {
    let mut ai = Ai::new();

    ai.set_board(&[
        (0, 1, Stone::Ally),
        (1, 1, Stone::Opponent),
        (2, 1, Stone::Opponent),
        (3, 1, Stone::Opponent),
        (4, 1, Stone::Opponent),
    ]);

    let ai_move = ai.play();

    assert_eq!(ai_move, (5, 1));
}

#[test]
fn up_right_diagonal_lose() {
    let mut ai = Ai::new();

    ai.set_board(&[
        (6, 0, Stone::Ally),
        (5, 1, Stone::Opponent),
        (4, 2, Stone::Opponent),
        (3, 3, Stone::Opponent),
        (2, 4, Stone::Opponent),
    ]);

    let ai_move = ai.play();

    assert_eq!(ai_move, (1, 5));
}

#[test]
fn down_right_diagonal_lose() {
    let mut ai = Ai::new();

    ai.set_board(&[
        (0, 0, Stone::Ally),
        (1, 1, Stone::Opponent),
        (2, 2, Stone::Opponent),
        (3, 3, Stone::Opponent),
        (4, 4, Stone::Opponent),
    ]);

    let ai_move = ai.play();

    assert_eq!(ai_move, (5, 5));
}
