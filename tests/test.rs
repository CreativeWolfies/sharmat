// The ~tests~ (the thing that tests functions & stuff)

use sharmat::Board;

// Engine basic tests

#[test]
fn board_create() {
    let board = Board::new();
}

#[test]
fn board_set_piece() {
    let mut board = Board::new();
    board.set(Piece::new(), 0, 0);
}

#[test]
fn board_get_piece() {
    let mut board = Board::new();
    let piece = Piece::new();
    board.set(piece.clone(), 0, 0);
    assert_eq!(board.get(0, 0), piece);
}

#[test]
fn piece_create() {
    let piece = Piece::new();
}

#[test]
fn rule_create() {
    let rule = Rule::new();
}

#[test]
fn game_create() {
    let game = Game::new();
}
