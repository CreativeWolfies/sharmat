use std::num::NonZeroUsize;
use sharmat::board::*;

#[test]
fn board_create() {
    let _board = Board::new(NonZeroUsize::new(9).unwrap(), NonZeroUsize::new(8).unwrap());
}

#[test]
fn board_set_name() {
    let mut board = Board::new(NonZeroUsize::new(1).unwrap(), NonZeroUsize::new(1).unwrap());
    board.set_name("Hello");
}

#[test]
fn board_get_name() {
    let mut board = Board::new(NonZeroUsize::new(1).unwrap(), NonZeroUsize::new(1).unwrap());
    board.set_name("Hello");
    assert_eq!(board.name(), "Hello");
}

#[test]
#[allow(unused_must_use)]
fn board_set_piece() {
    let mut board = Board::new(NonZeroUsize::new(9).unwrap(), NonZeroUsize::new(8).unwrap());
    board.set(0, 0, Some(2));
}

#[test]
fn board_oob_set_piece() {
    let mut board = Board::new(NonZeroUsize::new(9).unwrap(), NonZeroUsize::new(8).unwrap());
    assert_eq!(board.set(20, 20, Some(2)).unwrap_err(), BoardError::OutOfBounds(20, 20));
}

#[test]
#[allow(unused_must_use)]
fn board_get_piece() {
    for x in 0..9 {
        for y in 0..8 {
            let mut board = Board::new(NonZeroUsize::new(9).unwrap(), NonZeroUsize::new(8).unwrap());
            board.set(x, y, Some(1));
            assert_eq!(board.get(x, y).unwrap(), Some(1));
        }
    }
}

#[test]
fn board_oob_get_piece() {
    let board = Board::new(NonZeroUsize::new(3).unwrap(), NonZeroUsize::new(3).unwrap());
    assert_eq!(board.get(5, 5).unwrap_err(), BoardError::OutOfBounds(5, 5));
}

#[test]
#[allow(unused_must_use)]
fn board_move_piece() {
    let mut board = Board::new(NonZeroUsize::new(9).unwrap(), NonZeroUsize::new(8).unwrap());
    board.set(0, 0, Some(1));
    board.move_piece(0, 0, 3, 3);
    assert_eq!(board.get(3, 3).unwrap(), Some(1));
}

#[test]
fn board_oob_move_piece_first_pos() {
    let mut board = Board::new(NonZeroUsize::new(5).unwrap(), NonZeroUsize::new(5).unwrap());
    assert_eq!(board.move_piece(6, 6, 0, 0).unwrap_err(), BoardError::OutOfBounds(6, 6));
}

#[test]
fn board_oob_move_piece_scnd_pos() {
    let mut board = Board::new(NonZeroUsize::new(5).unwrap(), NonZeroUsize::new(5).unwrap());
    assert_eq!(board.move_piece(0, 0, 6, 6).unwrap_err(), BoardError::OutOfBounds(6, 6));
}

#[test]
#[allow(unused_must_use)]
fn board_clear_piece() {
    let mut board = Board::new(NonZeroUsize::new(9).unwrap(), NonZeroUsize::new(8).unwrap());
    board.set(0, 0, Some(1));
    board.clear_pos(0, 0);
    assert_eq!(board.get(0, 0).unwrap(), None);
}

#[test]
fn board_oob_clear_piece() {
    let mut board = Board::new(NonZeroUsize::new(5).unwrap(), NonZeroUsize::new(5).unwrap());
    assert_eq!(board.clear_pos(6, 6).unwrap_err(), BoardError::OutOfBounds(6, 6));
}

#[test]
#[allow(unused_must_use)]
fn board_clear_board() {
    let empty_board = Board::new(NonZeroUsize::new(5).unwrap(), NonZeroUsize::new(5).unwrap());
    let mut board = Board::new(NonZeroUsize::new(5).unwrap(), NonZeroUsize::new(5).unwrap());
    board.set(0, 0, Some(1));
    board.set(0, 3, Some(2));
    board.set(3, 0, Some(3));
    board.clear();
    assert_eq!(board, empty_board);
}
