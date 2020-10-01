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
    let piece = PieceBuilder::new().id("a").build();
    board.set(0, 0, &piece);
}

#[test]
#[allow(unused_must_use)]
fn board_get_piece() {
    let mut board = Board::new(NonZeroUsize::new(9).unwrap(), NonZeroUsize::new(8).unwrap());
    let piece = PieceBuilder::new()
        .id("b")
        .build();
    board.set(0, 0, &piece);
    assert_eq!(board.get(0, 0).unwrap().map(|x| x.clone()), Some(piece));
}

#[test]
fn board_move_piece() {
    let mut board = Board::new(NonZeroUsize::new(9).unwrap(), NonZeroUsize::new(8).unwrap());
    let piece = PieceBuilder::new().id("c").build();
    board.set(0, 0, &piece);
    board.move_piece(0, 0, 3, 3);
    assert_eq!(board.get(3, 3).unwrap().map(|x| x.clone()), Some(piece));
}

#[test]
fn board_clear_piece() {
    let mut board = Board::new(NonZeroUsize::new(9).unwrap(), NonZeroUsize::new(8).unwrap());
    let piece = PieceBuilder::new().id("d").build();
    board.set(0, 0, &piece);
    board.clear_pos(0, 0);
    assert_eq!(board.get(0, 0).unwrap(), None);
}

#[test]
fn board_clear_board() {
    let empty_board = Board::new(NonZeroUsize::new(5).unwrap(), NonZeroUsize::new(5).unwrap());
    let mut board = Board::new(NonZeroUsize::new(5).unwrap(), NonZeroUsize::new(5).unwrap());
    let piece = PieceBuilder::new().id("e").build();
    board.set(0, 0, &piece);
    board.set(0, 3, &piece);
    board.set(3, 0, &piece);
    board.clear();
    assert_eq!(board, empty_board);
}

#[test]
fn piece_create_builder() {
    let _piece = PieceBuilder::new();
}

#[test]
fn piece_create_with_id() {
    let _piece = PieceBuilder::new()
        .id("piece_name")
        .build();
}

#[test]
fn piece_create_with_alias() {
    let _piece = PieceBuilder::new()
        .id("piece_name")
        .alias("Piece name")
        .build();
}

#[test]
fn piece_create_with_description() {
    let _piece = PieceBuilder::new()
        .id("piece_name")
        .desc("It's just a test piece")
        .build();
}

#[test]
fn piece_get_id() {
    let piece = PieceBuilder::new()
        .id("piece_name")
        .build();
    assert_eq!(piece.id(), "piece_name");
}

#[test]
fn piece_get_alias() {
    let piece = PieceBuilder::new()
        .id("piece_name")
        .alias("Piece name")
        .build();
    assert_eq!(piece.alias(), "Piece name");
}

#[test]
fn piece_get_description() {
    let piece = PieceBuilder::new()
        .id("piece_name")
        .desc("It's just a test piece")
        .build();
    assert_eq!(piece.desc(), "It's just a test piece");
}

#[test]
fn piece_id_override() {
    let piece = PieceBuilder::new()
        .id("piece_name")
        .id("other_piece_name")
        .build();
    assert_eq!(piece.id(), "other_piece_name");
}

#[test]
fn piece_alias_append() {
    let piece = PieceBuilder::new()
        .id("piece_name")
        .alias("Insert text")
        .alias("So-called test piece")
        .build();
    assert_eq!(piece.alias(), "Insert text; So-called test piece");
}

#[test]
fn piece_description_append() {
    let piece = PieceBuilder::new()
        .id("piece_name")
        .desc("First line")
        .desc("Second line")
        .build();
    assert_eq!(piece.desc(), "First line\nSecond line");
}

#[test]
fn rule_create() {
    let _rule = Rule::new(/* UNKNOWN */);
}

#[test]
fn game_create() {
    let _game = Game::new(/* UNKNOWN */);
}
