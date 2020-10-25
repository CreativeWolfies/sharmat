use std::num::NonZeroUsize;
use sharmat::board::*;
use sharmat::piece::*;
use sharmat::game::*;

#[test]
fn game_create() {
    let _game = GameBuilder::new();
}

#[test]
fn game_create_with_board() {
    let board = Board::new(NonZeroUsize::new(5).unwrap(), NonZeroUsize::new(5).unwrap());
    let _game = GameBuilder::new()
        .board(board)
        .build();
}

#[test]
fn game_create_with_board_push() {
    let board = Board::new(NonZeroUsize::new(5).unwrap(), NonZeroUsize::new(5).unwrap());
    let board2 = Board::new(NonZeroUsize::new(5).unwrap(), NonZeroUsize::new(5).unwrap());
    let _game = GameBuilder::new()
        .board(board)
        .board(board2)
        .build();
}

#[test]
fn game_create_with_boards_push() {
    let board = Board::new(NonZeroUsize::new(5).unwrap(), NonZeroUsize::new(5).unwrap());
    let board2 = Board::new(NonZeroUsize::new(5).unwrap(), NonZeroUsize::new(5).unwrap());
    let _game = GameBuilder::new()
        .boards(vec![board, board2])
        .build();
}

#[test]
fn game_create_with_board_and_boards() {
    let board = Board::new(NonZeroUsize::new(5).unwrap(), NonZeroUsize::new(5).unwrap());
    let board2 = Board::new(NonZeroUsize::new(5).unwrap(), NonZeroUsize::new(5).unwrap());
    let board3 = Board::new(NonZeroUsize::new(5).unwrap(), NonZeroUsize::new(5).unwrap());
    let _game = GameBuilder::new()
        .board(board)
        .boards(vec![board2, board3])
        .build();
}

#[test]
fn game_get_boards_with_board() {
    let board = Board::new(NonZeroUsize::new(5).unwrap(), NonZeroUsize::new(5).unwrap());
    let game = GameBuilder::new()
        .board(board.clone())
        .build();
    assert_eq!(game.boards(), &vec![board]);
}

#[test]
fn game_get_boards_with_board_push() {
    let board = Board::new(NonZeroUsize::new(5).unwrap(), NonZeroUsize::new(5).unwrap());
    let board2 = Board::new(NonZeroUsize::new(5).unwrap(), NonZeroUsize::new(5).unwrap());
    let game = GameBuilder::new()
        .board(board.clone())
        .board(board2.clone())
        .build();
    assert_eq!(*game.boards(), vec![board, board2]);
}

#[test]
fn game_get_boards_with_boards_push() {
    let board = Board::new(NonZeroUsize::new(5).unwrap(), NonZeroUsize::new(5).unwrap());
    let board2 = Board::new(NonZeroUsize::new(5).unwrap(), NonZeroUsize::new(5).unwrap());
    let game = GameBuilder::new()
        .boards(vec![board.clone(), board2.clone()])
        .build();
    assert_eq!(*game.boards(), vec![board, board2]);
}

#[test]
fn game_get_boards_with_board_and_boards_push() {
    let board = Board::new(NonZeroUsize::new(5).unwrap(), NonZeroUsize::new(5).unwrap());
    let board2 = Board::new(NonZeroUsize::new(5).unwrap(), NonZeroUsize::new(5).unwrap());
    let board3 = Board::new(NonZeroUsize::new(5).unwrap(), NonZeroUsize::new(5).unwrap());
    let game = GameBuilder::new()
        .board(board.clone())
        .boards(vec![board2.clone(), board3.clone()])
        .build();
    assert_eq!(*game.boards(), vec![board, board2, board3]);
}

#[test]
fn game_create_with_piece() {
    let piece = PieceBuilder::new().build();
    let _game = GameBuilder::new()
        .piece(piece)
        .build();
}

#[test]
fn game_create_with_piece_push() {
    let piece = PieceBuilder::new().build();
    let piece2 = PieceBuilder::new().build();
    let _game = GameBuilder::new()
        .piece(piece)
        .piece(piece2)
        .build();
}

#[test]
fn game_create_with_pieces_push() {
    let piece = PieceBuilder::new().build();
    let piece2 = PieceBuilder::new().build();
    let _game = GameBuilder::new()
        .pieces(vec![piece, piece2])
        .build();
}

#[test]
fn game_create_with_piece_and_pieces_push() {
    let piece = PieceBuilder::new().build();
    let piece2 = PieceBuilder::new().build();
    let piece3 = PieceBuilder::new().build();
    let _game = GameBuilder::new()
        .piece(piece)
        .pieces(vec![piece2, piece3])
        .build();
}

#[test]
fn game_get_pieces_with_piece() {
    let piece = PieceBuilder::new().build();
    let game = GameBuilder::new()
        .piece(piece.clone())
        .build();
    assert_eq!(*game.pieces(), vec![piece]);
}

#[test]
fn game_get_pieces_with_piece_push() {
    let piece = PieceBuilder::new().build();
    let piece2 = PieceBuilder::new().build();
    let game = GameBuilder::new()
        .piece(piece.clone())
        .piece(piece2.clone())
        .build();
    assert_eq!(*game.pieces(), vec![piece, piece2]);
}

#[test]
fn game_get_pieces_with_pieces_push() {
    let piece = PieceBuilder::new().build();
    let piece2 = PieceBuilder::new().build();
    let game = GameBuilder::new()
        .pieces(vec![piece.clone(), piece2.clone()])
        .build();
    assert_eq!(*game.pieces(), vec![piece, piece2]);
}

#[test]
fn game_get_pieces_with_piece_and_pieces_push() {
    let piece = PieceBuilder::new().build();
    let piece2 = PieceBuilder::new().build();
    let piece3 = PieceBuilder::new().build();
    let game = GameBuilder::new()
        .piece(piece.clone())
        .pieces(vec![piece2.clone(), piece3.clone()])
        .build();
    assert_eq!(*game.pieces(), vec![piece, piece2, piece3]);
}

#[test]
fn game_search_piece_by_id_success() {
    let piece = PieceBuilder::new().build();
    let piece2 = PieceBuilder::new().build();
    let piece3 = PieceBuilder::new().build();
    let game = GameBuilder::new()
        .pieces(vec![piece.clone(), piece2, piece3])
        .build();
    assert_eq!(game.search_piece(piece.id()), Some(&piece));
}

#[test]
fn game_search_piece_by_id_fail() {
    let piece = PieceBuilder::new().build();
    let piece2 = PieceBuilder::new().build();
    let piece3 = PieceBuilder::new().build();
    let game = GameBuilder::new()
        .pieces(vec![piece, piece2, piece3])
        .build();
    assert_eq!(game.search_piece("random"), None);
}

#[test]
fn game_search_piece_by_alias_success() {
    let piece = PieceBuilder::new().alias("test").build();
    let piece2 = PieceBuilder::new().alias("foo").alias("test").build();
    let piece3 = PieceBuilder::new().build();
    let game = GameBuilder::new()
        .pieces(vec![piece.clone(), piece2.clone(), piece3])
        .build();
    assert_eq!(game.search_piece_alias("test"), vec![&piece, &piece2]);
}

#[test]
fn game_search_piece_by_alias_fail() {
    let piece = PieceBuilder::new().alias("test").build();
    let piece2 = PieceBuilder::new().alias("foo").alias("test").build();
    let piece3 = PieceBuilder::new().build();
    let game = GameBuilder::new()
        .pieces(vec![piece, piece2, piece3])
        .build();
    assert_eq!(game.search_piece_alias("hmm"), Vec::<&Piece>::new());
}
