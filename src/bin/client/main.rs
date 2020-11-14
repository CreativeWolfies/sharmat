// The ~client~ (the thing that displays pieces & stuff)

extern crate iced;
extern crate iced_native;
extern crate iced_wgpu;

use std::num::NonZeroUsize;
use iced::{Application, Settings};
use sharmat::board::Board;
use sharmat::piece::*;
use sharmat::game::*;

pub mod gui;
pub mod style;
pub mod pieces;

fn main() {
    // TODO: do this in another thread or idk
    let mut board = Board::new(NonZeroUsize::new(8).unwrap(), NonZeroUsize::new(8).unwrap());
    board.set(0, 7, Some(5)).unwrap();
    board.set(1, 7, Some(2)).unwrap();
    board.set(2, 7, Some(0)).unwrap();
    board.set(3, 7, Some(4)).unwrap();
    board.set(4, 7, Some(1)).unwrap();
    board.set(5, 7, Some(0)).unwrap();
    board.set(6, 7, Some(2)).unwrap();
    board.set(7, 7, Some(5)).unwrap();

    for x in 0..8 {
        board.set(x, 6, Some(3)).unwrap();
        board.set(x, 1, Some(6 + 3)).unwrap();
    }

    board.set(0, 0, Some(6 + 5)).unwrap();
    board.set(1, 0, Some(6 + 2)).unwrap();
    board.set(2, 0, Some(6 + 0)).unwrap();
    board.set(3, 0, Some(6 + 4)).unwrap();
    board.set(4, 0, Some(6 + 1)).unwrap();
    board.set(5, 0, Some(6 + 0)).unwrap();
    board.set(6, 0, Some(6 + 2)).unwrap();
    board.set(7, 0, Some(6 + 5)).unwrap();
    let game = GameBuilder::new()
        .board(board)
        .pieces(
            vec![
                "standard.w_bishop",
                "standard.w_king",
                "standard.w_knight",
                "standard.w_pawn",
                "standard.w_queen",
                "standard.w_rook",
                "standard.b_bishop",
                "standard.b_king",
                "standard.b_knight",
                "standard.b_pawn",
                "standard.b_queen",
                "standard.b_rook",
            ].into_iter()
            .map(|name| PieceBuilder::new().id(name).build())
            .collect()
        )
        .build();
    let piece_assets = pieces::load_assets(format!("{}/assets/", env!("CARGO_MANIFEST_DIR")));
    gui::Sharmat::run(Settings::with_flags((piece_assets, game)))
}
