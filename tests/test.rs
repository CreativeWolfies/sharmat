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
    let piece = PieceBuilder::new().build();
    board.set(0, 0, &piece);
}

#[test]
fn board_oob_set_piece() {
    let mut board = Board::new(NonZeroUsize::new(9).unwrap(), NonZeroUsize::new(8).unwrap());
    let piece = PieceBuilder::new().build();
    assert_eq!(board.set(20, 20, &piece).unwrap_err(), BoardError::OutOfBounds);
}

#[test]
#[allow(unused_must_use)]
fn board_get_piece() {
    let mut board = Board::new(NonZeroUsize::new(9).unwrap(), NonZeroUsize::new(8).unwrap());
    let piece = PieceBuilder::new().build();
    board.set(0, 0, &piece);
    assert_eq!(board.get(0, 0).unwrap().map(|x| x.clone()), Some(piece));
}

#[test]
fn board_oob_get_piece() {
    let board = Board::new(NonZeroUsize::new(3).unwrap(), NonZeroUsize::new(3).unwrap());
    assert_eq!(board.get(5, 5).unwrap_err(), BoardError::OutOfBounds);
}

#[test]
#[allow(unused_must_use)]
fn board_move_piece() {
    let mut board = Board::new(NonZeroUsize::new(9).unwrap(), NonZeroUsize::new(8).unwrap());
    let piece = PieceBuilder::new().build();
    board.set(0, 0, &piece);
    board.move_piece(0, 0, 3, 3);
    assert_eq!(board.get(3, 3).unwrap().map(|x| x.clone()), Some(piece));
}

#[test]
fn board_oob_move_piece_first_pos() {
    let mut board = Board::new(NonZeroUsize::new(5).unwrap(), NonZeroUsize::new(5).unwrap());
    assert_eq!(board.move_piece(6, 6, 0, 0).unwrap_err(), BoardError::OutOfBounds);
}

#[test]
fn board_oob_move_piece_scnd_pos() {
    let mut board = Board::new(NonZeroUsize::new(5).unwrap(), NonZeroUsize::new(5).unwrap());
    assert_eq!(board.move_piece(0, 0, 6, 6).unwrap_err(), BoardError::OutOfBounds);
}

#[test]
#[allow(unused_must_use)]
fn board_clear_piece() {
    let mut board = Board::new(NonZeroUsize::new(9).unwrap(), NonZeroUsize::new(8).unwrap());
    let piece = PieceBuilder::new().id("d").build();
    board.set(0, 0, &piece);
    board.clear_pos(0, 0);
    assert_eq!(board.get(0, 0).unwrap(), None);
}

#[test]
fn board_oob_clear_piece() {
    let mut board = Board::new(NonZeroUsize::new(5).unwrap(), NonZeroUsize::new(5).unwrap());
    assert_eq!(board.clear_pos(6, 6).unwrap_err(), BoardError::OutOfBounds);
}

#[test]
#[allow(unused_must_use)]
fn board_clear_board() {
    let empty_board = Board::new(NonZeroUsize::new(5).unwrap(), NonZeroUsize::new(5).unwrap());
    let mut board = Board::new(NonZeroUsize::new(5).unwrap(), NonZeroUsize::new(5).unwrap());
    let piece = PieceBuilder::new().build();
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
    let _game = GameBuilder::new();
}

#[test]
fn game_create_with_board() {
    let board = Board::new(NonZeroUsize::new(5).unwrap(), NonZeroUsize::new(5).unwrap());
    let _game = GameBuilder::new()
        .board(&board)
        .build();
}

#[test]
fn game_create_with_board_push() {
    let board = Board::new(NonZeroUsize::new(5).unwrap(), NonZeroUsize::new(5).unwrap());
    let board2 = Board::new(NonZeroUsize::new(5).unwrap(), NonZeroUsize::new(5).unwrap());
    let _game = GameBuilder::new()
        .board(&board)
        .board(&board2)
        .build();
}

#[test]
fn game_create_with_boards_push() {
    let board = Board::new(NonZeroUsize::new(5).unwrap(), NonZeroUsize::new(5).unwrap());
    let board2 = Board::new(NonZeroUsize::new(5).unwrap(), NonZeroUsize::new(5).unwrap());
    let _game = GameBuilder::new()
        .boards(vec![&board, &board2])
        .build();
}

#[test]
fn game_create_with_board_and_boards() {
    let board = Board::new(NonZeroUsize::new(5).unwrap(), NonZeroUsize::new(5).unwrap());
    let board2 = Board::new(NonZeroUsize::new(5).unwrap(), NonZeroUsize::new(5).unwrap());
    let board3 = Board::new(NonZeroUsize::new(5).unwrap(), NonZeroUsize::new(5).unwrap());
    let _game = GameBuilder::new()
        .board(&board)
        .boards(vec![&board2, &board3])
        .build();
}

#[test]
fn game_get_boards_with_board() {
    let board = Board::new(NonZeroUsize::new(5).unwrap(), NonZeroUsize::new(5).unwrap());
    let game = GameBuilder::new()
        .board(&board)
        .build();
    assert_eq!(game.boards(), &vec![&board]);
}

#[test]
fn game_get_boards_with_board_push() {
    let board = Board::new(NonZeroUsize::new(5).unwrap(), NonZeroUsize::new(5).unwrap());
    let board2 = Board::new(NonZeroUsize::new(5).unwrap(), NonZeroUsize::new(5).unwrap());
    let game = GameBuilder::new()
        .board(&board)
        .board(&board2)
        .build();
    assert_eq!(*game.boards(), vec![&board, &board2]);
}

#[test]
fn game_get_boards_with_boards_push() {
    let board = Board::new(NonZeroUsize::new(5).unwrap(), NonZeroUsize::new(5).unwrap());
    let board2 = Board::new(NonZeroUsize::new(5).unwrap(), NonZeroUsize::new(5).unwrap());
    let game = GameBuilder::new()
        .boards(vec![&board, &board2])
        .build();
    assert_eq!(*game.boards(), vec![&board, &board2]);
}

#[test]
fn game_get_boards_with_board_and_boards_push() {
    let board = Board::new(NonZeroUsize::new(5).unwrap(), NonZeroUsize::new(5).unwrap());
    let board2 = Board::new(NonZeroUsize::new(5).unwrap(), NonZeroUsize::new(5).unwrap());
    let board3 = Board::new(NonZeroUsize::new(5).unwrap(), NonZeroUsize::new(5).unwrap());
    let game = GameBuilder::new()
        .board(&board)
        .boards(vec![&board2, &board3])
        .build();
    assert_eq!(*game.boards(), vec![&board, &board2, &board3]);
}

#[test]
fn game_create_with_piece() {
    let piece = PieceBuilder::new().build();
    let _game = GameBuilder::new()
        .piece(&piece)
        .build();
}

#[test]
fn game_create_with_piece_push() {
    let piece = PieceBuilder::new().build();
    let piece2 = PieceBuilder::new().build();
    let _game = GameBuilder::new()
        .piece(&piece)
        .piece(&piece2)
        .build();
}

#[test]
fn game_create_with_pieces_push() {
    let piece = PieceBuilder::new().build();
    let piece2 = PieceBuilder::new().build();
    let _game = GameBuilder::new()
        .pieces(vec![&piece, &piece2])
        .build();
}

#[test]
fn game_create_with_piece_and_pieces_push() {
    let piece = PieceBuilder::new().build();
    let piece2 = PieceBuilder::new().build();
    let piece3 = PieceBuilder::new().build();
    let _game = GameBuilder::new()
        .piece(&piece)
        .pieces(vec![&piece2, &piece3])
        .build();
}

#[test]
fn game_get_pieces_with_piece() {
    let piece = PieceBuilder::new().build();
    let game = GameBuilder::new()
        .piece(&piece)
        .build();
    assert_eq!(*game.pieces(), vec![&piece]);
}

#[test]
fn game_get_pieces_with_piece_push() {
    let piece = PieceBuilder::new().build();
    let piece2 = PieceBuilder::new().build();
    let game = GameBuilder::new()
        .piece(&piece)
        .piece(&piece2)
        .build();
    assert_eq!(*game.pieces(), vec![&piece, &piece2]);
}

#[test]
fn game_get_pieces_with_pieces_push() {
    let piece = PieceBuilder::new().build();
    let piece2 = PieceBuilder::new().build();
    let game = GameBuilder::new()
        .pieces(vec![&piece, &piece2])
        .build();
    assert_eq!(*game.pieces(), vec![&piece, &piece2]);
}

#[test]
fn game_get_pieces_with_piece_and_pieces_push() {
    let piece = PieceBuilder::new().build();
    let piece2 = PieceBuilder::new().build();
    let piece3 = PieceBuilder::new().build();
    let game = GameBuilder::new()
        .piece(&piece)
        .pieces(vec![&piece2, &piece3])
        .build();
    assert_eq!(*game.pieces(), vec![&piece, &piece2, &piece3]);
}

#[test]
fn game_search_piece_by_id_success() {
    let piece = PieceBuilder::new().build();
    let piece2 = PieceBuilder::new().build();
    let piece3 = PieceBuilder::new().build();
    let game = GameBuilder::new()
        .pieces(vec![&piece, &piece2, &piece3])
        .build();
    assert_eq!(game.search_piece(piece.id()), Some(&&piece));
}

#[test]
fn game_search_piece_by_id_fail() {
    let piece = PieceBuilder::new().build();
    let piece2 = PieceBuilder::new().build();
    let piece3 = PieceBuilder::new().build();
    let game = GameBuilder::new()
        .pieces(vec![&piece, &piece2, &piece3])
        .build();
    assert_eq!(game.search_piece("random"), None);
}

#[test]
fn game_search_piece_by_alias_success() {
    let piece = PieceBuilder::new().alias("test").build();
    let piece2 = PieceBuilder::new().alias("foo").alias("test").build();
    let piece3 = PieceBuilder::new().build();
    let game = GameBuilder::new()
        .pieces(vec![&piece, &piece2, &piece3])
        .build();
    assert_eq!(game.search_piece_alias("test"), vec![&&piece, &&piece2]);
}

#[test]
fn game_search_piece_by_alias_fail() {
    let piece = PieceBuilder::new().alias("test").build();
    let piece2 = PieceBuilder::new().alias("foo").alias("test").build();
    let piece3 = PieceBuilder::new().build();
    let game = GameBuilder::new()
        .pieces(vec![&piece, &piece2, &piece3])
        .build();
    assert_eq!(game.search_piece_alias("hmm"), Vec::<&&Piece>::new());
}
