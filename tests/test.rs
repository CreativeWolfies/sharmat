// The ~tests~ (the thing that tests functions & stuff)

use sharmat::*;

// Engine basic tests

#[test]
fn board_create() {
    let board = Board::new(9, 8);
}

#[test]
fn board_set_piece() {
    let mut board = Board::new(9, 8);
    board.set(Piece::new(), 0, 0);
}

#[test]
fn board_get_piece() {
    let mut board = Board::new(9, 8);
    let piece = Piece::new(/* UNKNOWN */);
    board.set(piece.clone(), 0, 0);
    assert_eq!(board.get(0, 0), piece);
}

#[test]
fn piece_create() {
    let piece = Piece::new(/* UNKNOWN */);
}

#[test]
fn rule_create() {
    let rule = Rule::new(/* UNKNOWN */);
}

#[test]
fn game_create() {
    let game = Game::new(/* UNKNOWN */);
}
