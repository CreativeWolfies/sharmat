// The ~client~ (the thing that displays pieces & stuff)

extern crate iced;
extern crate iced_native;
extern crate iced_wgpu;

use std::num::NonZeroUsize;
use iced::{Application, Settings};
use sharmat::board::Board;
use sharmat::piece::{Piece, PieceBuilder};

pub mod gui;
pub mod style;
pub mod pieces;

fn main() {
    // TODO: do this in another thread or idk
    let mut board = Board::new(NonZeroUsize::new(8).unwrap(), NonZeroUsize::new(8).unwrap());
    let pawn = PieceBuilder::new().id("standard.w_pawn").build();
    let pieces = vec![pawn];
    let piece_assets = pieces::load_assets(format!("{}/assets/", env!("CARGO_MANIFEST_DIR")));
    gui::Sharmat::run(Settings::with_flags((piece_assets, board, pieces)))
}
