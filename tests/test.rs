// The ~tests~ (the thing that tests functions & stuff)

use std::num::NonZeroUsize;
use sharmat::board::*;
use sharmat::piece::*;
use sharmat::rule::*;
use sharmat::game::*;

// Engine basic tests

#[test]
fn board_create() {
    let _board = Board::new(NonZeroUsize::new(9).unwrap(), NonZeroUsize::new(8).unwrap());
}

#[test]
#[allow(unused_must_use)]
fn board_set_piece() {
    let mut board = Board::new(NonZeroUsize::new(9).unwrap(), NonZeroUsize::new(8).unwrap());
    board.set(0, 0, Piece::new());
}

#[test]
#[allow(unused_must_use)]
fn board_get_piece() {
    let mut board = Board::new(NonZeroUsize::new(9).unwrap(), NonZeroUsize::new(8).unwrap());
    let piece = Piece::new(/* UNKNOWN */);
    board.set(0, 0, piece.clone());
    assert_eq!(board.get(0, 0).unwrap().clone(), Some(piece));
}

#[test]
fn piece_create() {
    let _piece = Piece::new(/* UNKNOWN */);
}

#[test]
fn rule_create() {
    let _rule = Rule::new(/* UNKNOWN */);
}

#[test]
fn game_create() {
    let _game = Game::new(/* UNKNOWN */);
}
